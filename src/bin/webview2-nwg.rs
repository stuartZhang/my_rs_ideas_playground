#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, feature(trace_macros, log_syntax))]

#[path ="../nwg-webview/mod.rs"]
mod webview;

use ::futures::{FutureExt, executor::LocalPool, task::LocalSpawnExt};
use ::nwg::{self as nwg, Event as NwgEvent, FrameFlags, GridLayout, Window};
use ::std::error::Error;
use webview::WebviewContainer;
fn main() -> Result<(), Box<dyn Error>> {
    // 开启高分辨率模式。从 COM API 开启的方式将会被废弃，推荐从【应用程序】配置清单文件开启。
    #[allow(deprecated)]
    unsafe {
        nwg::set_dpi_awareness()
    };
    nwg::init()?;
    // 主窗体
    let mut window = Window::default();
    Window::builder().title("内嵌 WebView 例程").size((1024, 168)).build(&mut window)?;
    nwg::full_bind_event_handler(&window.handle, move |event, _data, _handle| {
        if let NwgEvent::OnWindowClose = event { // 关闭主窗体。
            nwg::stop_thread_dispatch();
        }
    });
    // WebView 容器
    let mut webview_container = WebviewContainer::default();
    WebviewContainer::builder().parent(&window).window(&window).enabled(true).flags(FrameFlags::VISIBLE).build(&mut webview_container)?;
    let mut grid = GridLayout::default();
    GridLayout::builder().margin([0; 4]).max_column(Some(1)).max_row(Some(1)).child(0, 0, &webview_container).parent(&window).build(&mut grid)?;
    // 业务处理逻辑
    let mut executor = {
        let executor = LocalPool::new();
        let webview_ready_fut = webview_container.ready_fut()?;
        executor.spawner().spawn_local(async move {
            let webview = webview_ready_fut.await.unwrap();
            webview.navigate("https://www.minxing365.com")?;
            Ok::<_, Box<dyn Error>>(())
        }.map(|result| {
            if let Err(err) = result {
                eprintln!("[app_main]{err}");
            }
        }))?;
        executor
    };
    // 阻塞主线程，等待用户手动关闭主窗体
    nwg::dispatch_thread_events_with_callback(move ||
        // 以 win32 UI 的事件循环为【反应器】，对接 futures crate 的【执行器】
        executor.run_until_stalled());
    Ok(())
}
