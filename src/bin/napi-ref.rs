/**
 * 该模块是本例程对 N-API bindings 的 Stub，因为 nj_sys 并没有在
 * playground.org 的 top 100 依赖包清单里。
 * 被模拟的 N-API 外部函数接口包括：
 * (1) napi_value 代表 JS VM 堆内存里的一个对象
 * (2) napi_ref   代表指向 napi_value 值的引用计数指针
 * (3) napi_create_reference() 函数从 napi_value 构造 napi_ref
 * (4) napi_delete_reference() 函数析构掉 napi_ref
 */
#[allow(dead_code, non_camel_case_types)]
mod nj_sys {
    #[derive(Copy, Clone, Debug)]
    pub struct napi_value;
    #[derive(Copy, Clone, Debug)]
    pub struct napi_ref {
        value: napi_value,
        ref_count: i8
    }
    pub fn napi_create_reference(value: napi_value, ref_count: i8) -> napi_ref {
        napi_ref {value, ref_count}
    }
    pub fn napi_delete_reference(reference: napi_ref) {
        println!("[napi_delete_reference]{:#?}", reference);
    }
}
/**
 * “二段式”引用计数中 Rc<T> 的包装类。根据 Newtypes 设计模式，该包装
 * 类是 Rc<T> 的智能指针 — 智能指针的智能指针。
 * 该包装类同时完成了两个功能：
 * (1) 接管对【引用个数】的跟踪功能。即，克隆智能指针时，引用个数加1；
 *     析构智能指针时，引用个数减一。
 * (2) 在【引用个数】归零时，通知 N-API 清空 JS 堆内存中的引用计数，和
 *     敦促 JS VM 的 GC 回收 JS 堆对象。
 */
mod napi_rc {
    use ::std::{rc::Rc, ops::{Deref, DerefMut}};
    use crate::nj_sys::{napi_create_reference, napi_delete_reference, napi_ref, napi_value};
    #[derive(Clone, Debug)]
    pub struct NapiRc(Rc<napi_ref>);
    // 构造函数
    impl NapiRc {
        pub fn new(value: napi_value) -> Self {
            NapiRc(Rc::new(napi_create_reference(value, 1)))
        }
    }
    // 智能指针标配
    impl Deref for NapiRc {
        type Target = Rc<napi_ref>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl DerefMut for NapiRc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    // 跟踪【引用数量】。当引用计数归零时，就调用 N-API 的外部函数接口
    // 以清空 JS 堆内存中的引用计数。
    impl Drop for NapiRc {
        fn drop(&mut self) {
            let count = Rc::strong_count(self) - 1;
            if count > 0 {
                #[cfg(debug_assertions)]
                println!("[NapiRc::drop]还有{}个引用，尚不能清空 N-API 端的引用计数", count);
            } else {
                Rc::get_mut(&mut self.0).map(|napi_ref| {
                    #[cfg(debug_assertions)]
                    println!("[NapiRc::drop]引用计数归零了，级联清空 N-API 端的引用计数");
                    napi_delete_reference(*napi_ref);
                });
            }
        }
    }
}
#[cfg(debug_assertions)]
use ::std::rc::Rc;
use napi_rc::NapiRc;
use nj_sys::napi_value;
// 处理复杂业务逻辑的主体代码。
pub fn napi_export_method(napi_value: napi_value) {
    let w = NapiRc::new(napi_value);
    #[cfg(debug_assertions)]
    eprintln!("[main]初始引用计数：{}", Rc::strong_count(&w));
    // 经由复杂的业务处理的功能实现
    {
        let w = w.clone();
        {
            let w = w.clone();
            print(2, w);
        } // 还有两个引用计数
        print(1, w);
    } // 还有一个引用计数
    {
        let w = w.clone();
        print(3, w);
    } // 还有一个引用计数
    print(4, w);
    fn print(index: u8, rc: NapiRc) {
        println!("[print]复本{}：{:#?}", index, rc);
    }
} // 引用计数归零
fn main() {
    println!("===== 开始 =====");
    // 模拟从 FFI 获取到 napi_value 值。
    let napi_value = napi_value {};
    // 执行复杂业务处理逻辑
    napi_export_method(napi_value);
    // 再做些其它的工作...
    println!("===== 结束 =====");
}