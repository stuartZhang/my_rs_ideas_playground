use ::deferred_future::LocalDeferredFuture;
use ::futures::{executor::LocalPool, task::LocalSpawnExt};
use ::native_windows_gui::{self as nwg, Event as NwgEvent, Frame, FrameBuilder, GridLayout, Window};
use ::std::{cell::RefCell, error::Error as StdErr, rc::Rc};
use ::webview2::{Controller, Error as WvErr, Result as WvResult};
use ::winapi::{shared::{windef::RECT, winerror::E_FAIL}, um::winuser::{SC_RESTORE, WM_SYSCOMMAND}};
fn main() -> Result<(), Box<dyn StdErr>> {
    // 开启高分辨率模式。从 COM API 开启的方式将会被废弃，推荐从【应用程序】配置清单文件开启。
    #[allow(deprecated)]
    unsafe {
        nwg::set_dpi_awareness()
    };
    nwg::init()?;
    let mut window = Window::default();
    Window::builder().title("内嵌 WebView 例程").size((1024, 168)).build(&mut window)?;
    let (webview_container, webview_ready_future) = assemble_webview(&window, Frame::builder().enabled(true).parent(&window))?;
    let mut grid = GridLayout::default();
    GridLayout::builder().max_column(Some(1)).max_row(Some(1)).child(0, 0, webview_container.as_ref()).parent(&window).build(&mut grid)?;
    let mut executor = {
        let executor = LocalPool::new();
        executor.spawner().spawn_local(async move {
            let webview_ctrl = webview_ready_future.await;
            let webview_ctrl = webview_ctrl.borrow();
            if let Some(controller) = webview_ctrl.as_ref() {
                app_main(controller).await.map_err(|err| eprintln!("{err}")).ok();
            }
        })?;
        executor
    };
    // 阻塞主线程，等待用户手动关闭主窗体
    nwg::dispatch_thread_events_with_callback(move ||
        // 以 win32 UI 的事件循环为【反应器】，对接 futures crate 的【执行器】
        executor.run_until_stalled());
    Ok(())
}
/// 在 webview 组件被成功初始化之后，开始执行应用程序的业务处理逻辑正文。
async fn app_main(webview_ctrl: &Controller) -> Result<(), Box<dyn StdErr>> {
    let webview = webview_ctrl.get_webview()?;
    webview.navigate("https://www.minxing365.com")?;
    Ok(())
}
/// 在指定容器元素内，创建一个 webview 组件，并挂到窗体上。
fn assemble_webview(window: &Window, frame_builder: FrameBuilder) -> Result<(Rc<Frame>, LocalDeferredFuture<Rc<RefCell<Option<Controller>>>>), Box<dyn StdErr>> {
    let mut frame = Rc::new(Frame::default());
    frame_builder.build(Rc::get_mut(&mut frame).unwrap())?;
    let webview_ctrl: Rc<RefCell<Option<Controller>>> = Rc::new(RefCell::new(None));
    let webview_ready_future = { // 创建 webview 组件
        let controller = Rc::clone(&webview_ctrl);
        let webview_ready_future = LocalDeferredFuture::default();
        let defer = webview_ready_future.defer();
        let frame = Rc::clone(&frame);
        webview2::Environment::builder().build(move |env| env?.create_controller(frame.handle.hwnd().ok_or(WvErr::new(E_FAIL))?, move |c| {
            let c = c?;
            align_webview_2_container(&c, frame)?;
            controller.borrow_mut().replace(c);
            defer.borrow_mut().complete(controller);
            Ok(())
        })).map(|_| webview_ready_future).map_err(|err| -> ! {
            nwg::modal_fatal_message(
                &window.handle,
                "Webview2 初始化失败",
                &format!("{}", err),
            );
        })
    }?;
    { // 因为【主窗体】直接就是 webview 的父组件，所以传递主窗体的事件给 webview 组件。
        let webview_ctrl = Rc::clone(&webview_ctrl);
        let frame = Rc::clone(&frame);
        nwg::full_bind_event_handler(&window.handle, move |event, _data, _handle| {
            match event {
                // 关闭主窗体时，先析构 webview，再【就绪】阻塞主线程的 Task。
                NwgEvent::OnWindowClose => webview_ctrl.borrow().as_ref().and_then(|controller| controller.close().map_err(|err| eprintln!("{err}")).ok()).map(|_| nwg::stop_thread_dispatch()),
                // 当主窗体被调整大小时，徒手传递尺寸调整事件给 webview 组件。
                NwgEvent::OnResize | NwgEvent::OnWindowMaximize => {
                    let frame = Rc::clone(&frame);
                    webview_ctrl.borrow().as_ref().and_then(move |controller| align_webview_2_container(controller, frame).map_err(|err| eprintln!("{err}")).ok())
                },
                // 当主窗体被最小化时，关闭 webview 组件，以减小空耗。
                NwgEvent::OnWindowMinimize => webview_ctrl.borrow().as_ref().and_then(|controller| controller.put_is_visible(false).map_err(|err| eprintln!("{err}")).ok()),
                // 当主窗体被移动时，徒手传递位移事件给 webview 组件。
                NwgEvent::OnMove => webview_ctrl.borrow().as_ref().and_then(|controller| controller.notify_parent_window_position_changed().map_err(|err| eprintln!("{err}")).ok()),
                _ => Some(())
            };
        });
    }
    { // nwg 封闭里漏掉了【主窗体】的 restore 事件，所以这里直接经由 winapi crate 的原始接口挂事件处理函数了。
        let webview_ctrl = Rc::clone(&webview_ctrl);
        nwg::bind_raw_event_handler(&window.handle, 0xffff + 1, move |_, msg, w, _| {
            if (WM_SYSCOMMAND, SC_RESTORE) == (msg, w as usize) {
                // 当主窗体被还原时，打开 webview 组件。
                webview_ctrl.borrow().as_ref().and_then(|controller| controller.put_is_visible(true).map_err(|err| eprintln!("{err}")).ok());
            }
            None
        })?;
    }
    return Ok((frame, webview_ready_future));
    /// 调整 webview 控件的大小·至·包含该 webview 控件的容器元素的最新大小
    fn align_webview_2_container(webview_ctrl: &Controller, frame: Rc<Frame>) -> WvResult<()> {
        let position = frame.position();
        let size = frame.size();
        return webview_ctrl.put_bounds(RECT {
            top: position.1,
            left: position.0,
            right: size.0 as i32,
            bottom: size.1 as i32
        });
    }
}
