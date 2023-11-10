use ::deferred_future::LocalDeferredFuture;
use ::futures::FutureExt;
use ::nwg::{self as nwg, ControlHandle, Event as NwgEvent, Frame, FrameBuilder, FrameFlags, NwgError};
use ::webview2::{Controller, Result as WvResult};
use ::std::{cell::RefCell, mem, rc::Rc};
use ::winapi::{shared::windef::{HWND, RECT}, um::winuser::{GetClientRect, SC_RESTORE, WM_SYSCOMMAND}};
use super::{NwgResult, WebviewContainer};
pub struct WebviewContainerBuilder {
    window: Option<ControlHandle>,
    frame_builder: FrameBuilder
}
impl Default for WebviewContainerBuilder {
    fn default() -> Self {
        Self {
            window: None,
            frame_builder: Frame::builder()
        }
    }
}
impl WebviewContainerBuilder {
    pub fn flags(mut self, flags: FrameFlags) -> WebviewContainerBuilder {
        self.frame_builder = self.frame_builder.flags(flags);
        self
    }
    pub fn size(mut self, size: (i32, i32)) -> WebviewContainerBuilder {
        self.frame_builder = self.frame_builder.size(size);
        self
    }
    pub fn position(mut self, pos: (i32, i32)) -> WebviewContainerBuilder {
        self.frame_builder = self.frame_builder.position(pos);
        self
    }
    pub fn enabled(mut self, e: bool) -> WebviewContainerBuilder {
        self.frame_builder = self.frame_builder.enabled(e);
        self
    }
    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> WebviewContainerBuilder {
        self.frame_builder = self.frame_builder.parent(p);
        self
    }
    pub fn window<C: Into<ControlHandle>>(mut self, window: C) -> WebviewContainerBuilder {
        self.window = Some(window.into());
        self
    }
    pub fn build(self, webview_container: &mut WebviewContainer) -> NwgResult<()> {
        let window_handle = self.window.ok_or(NwgError::initialization("window 配置项代表了主窗体。它是必填项"))?;
        self.frame_builder.build(&mut webview_container.frame.borrow_mut())?;
        let frame_hwnd = webview_container.frame.borrow().handle.hwnd().ok_or(NwgError::control_create("Frame 初始化失败"))?;
        // webview 组件构造异步锁
        webview_container.ready_fut.replace({
            let webview_ctrl = Rc::clone(&webview_container.webview_ctrl);
            let webview_ready_future = LocalDeferredFuture::default();
            let defer = webview_ready_future.defer();
            let frame = Rc::clone(&webview_container.frame);
            webview2::Environment::builder().build(move |env|
                env?.create_controller(frame_hwnd, move |webview_ctrl_core| {
                    let webview_ctrl_core = webview_ctrl_core?;
                    let webview = webview_ctrl_core.get_webview()?;
                    align_webview_2_container(&webview_ctrl_core, frame, frame_hwnd)?;
                    webview_ctrl.borrow_mut().replace(webview_ctrl_core);
                    defer.borrow_mut().complete(Some(webview));
                    Ok(())
                })
            ).map(|_| webview_ready_future.shared()).map_err(|err|NwgError::control_create(err.to_string()))
        }?);
        webview_container.event_handle.replace({ // 因为【主窗体】直接就是 webview 的父组件，所以传递主窗体的事件给 webview 组件。
            let webview_ctrl = Rc::clone(&webview_container.webview_ctrl);
            let frame = Rc::clone(&webview_container.frame);
            let is_closing = Rc::clone(&webview_container.is_closing);
            nwg::full_bind_event_handler(&window_handle, move |event, _data, _handle| {
                if *is_closing.borrow() {
                    return;
                }
                match event {
                    // 当主窗体被调整大小时，徒手传递尺寸调整事件给 webview 组件。
                    NwgEvent::OnResize | NwgEvent::OnWindowMaximize => {
                        let frame = Rc::clone(&frame);
                        webview_ctrl.borrow().as_ref().and_then(move |controller|
                            align_webview_2_container(controller, frame, frame_hwnd).map_err(|err| eprintln!("[OnResize|OnWindowMaximize]{err}")).ok()
                        )
                    },
                    // 当主窗体被最小化时，关闭 webview 组件，以减小空耗。
                    NwgEvent::OnWindowMinimize => webview_ctrl.borrow().as_ref().and_then(|controller| {
                        #[cfg(debug_assertions)]
                        println!("[WebviewContainer][OnWindowMinimize]Webview 被挂起了");
                        controller.put_is_visible(false).map_err(|err| eprintln!("[OnWindowMinimize]{err}")).ok()
                    }),
                    // 当主窗体被移动时，徒手传递位移事件给 webview 组件。
                    NwgEvent::OnMove => webview_ctrl.borrow().as_ref().and_then(|controller|
                        controller.notify_parent_window_position_changed().map_err(|err| eprintln!("[OnMove]{err}")).ok()
                    ),
                    _ => Some(())
                };
            })
        });
        webview_container.raw_event_handle.replace({ // nwg 封闭里漏掉了【主窗体】的 restore 事件，所以这里直接经由 winapi crate 的原始接口挂事件处理函数了。
            let webview_ctrl = Rc::clone(&webview_container.webview_ctrl);
            nwg::bind_raw_event_handler(&window_handle, 0xffff + 1, move |_, msg, w, _| {
                if (WM_SYSCOMMAND, SC_RESTORE) == (msg, w as usize) {
                    #[cfg(debug_assertions)]
                    println!("[WebviewContainer][OnWindowMinimize]Webview 被恢复了");
                    webview_ctrl.borrow().as_ref().and_then(|controller| // 当主窗体被还原时，打开 webview 组件。
                        controller.put_is_visible(true).map_err(|err| eprintln!("[OnWindowRestore]{err}")).ok()
                    );
                }
                None
            })?
        });
        Ok(())
    }
}
/// 调整 webview 控件的大小·至·包含该 webview 控件的容器元素的最新大小
fn align_webview_2_container(webview_ctrl: &Controller, frame: Rc<RefCell<Frame>>, frame_hwnd: HWND) -> WvResult<()> {
    let (successful, mut rect) = unsafe {
        let mut rect = mem::zeroed();
        let successful = GetClientRect(frame_hwnd, &mut rect);
        (successful, rect)
    };
    if successful == 0 {
        let position = frame.borrow().position();
        let size = frame.borrow().size();
        rect = RECT {
            top: position.1,
            left: position.0,
            right: size.0 as i32,
            bottom: size.1 as i32
        }
    }
    println!("rect={{top: {}, left: {}, width: {}, height: {} }}", rect.top, rect.left, rect.right, rect.bottom);
    return webview_ctrl.put_bounds(rect);
}