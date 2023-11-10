mod builder;
use ::deferred_future::LocalDeferredFuture;
use ::futures::future::Shared;
use ::nwg::{self as nwg, ControlHandle, EventHandler, Frame, NwgError, RawEventHandler};
use ::std::{cell::RefCell, ops::Deref, rc::Rc};
use ::webview2::{Controller, WebView};
pub use builder::WebviewContainerBuilder;

pub type NwgResult<T> = Result<T, NwgError>;
#[derive(Default)]
pub struct WebviewContainer {
    is_closing: Rc<RefCell<bool>>,
    frame: Rc<RefCell<Frame>>,
    webview_ctrl: Rc<RefCell<Option<Controller>>>,
    ready_fut: Option<Shared<LocalDeferredFuture<Option<WebView>>>>,
    event_handle: Option<EventHandler>,
    raw_event_handle: Option<RawEventHandler>
}
impl PartialEq for WebviewContainer {
    fn eq(&self, other: &Self) -> bool {
        self.frame.borrow().eq(other.frame.borrow().deref())
    }
}
impl Eq for WebviewContainer {}
impl From<WebviewContainer> for ControlHandle {
    fn from(value: WebviewContainer) -> Self {
        value.frame.borrow().handle
    }
}
impl From<&WebviewContainer> for ControlHandle {
    fn from(value: &WebviewContainer) -> Self {
        value.frame.borrow().handle
    }
}
impl Drop for WebviewContainer {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        println!("[WebviewContainer][drop]");
        *self.is_closing.borrow_mut() = true;
        self.event_handle.as_ref().map(nwg::unbind_event_handler);
        self.raw_event_handle.as_ref().map(nwg::unbind_raw_event_handler);
        self.webview_ctrl.borrow().as_ref().and_then(|controller| {
            controller.close().map_err(|err| eprintln!("[WebviewContainer][drop]{err}")).ok()
        });
        self.frame.borrow_mut().handle.destroy();
    }
}
impl WebviewContainer {
    pub fn builder() -> WebviewContainerBuilder {
        WebviewContainerBuilder::default()
    }
    pub fn ready_fut(&self) -> NwgResult<Shared<LocalDeferredFuture<Option<WebView>>>> {
        self.ready_fut.clone().ok_or(NwgError::control_create("Webview 控件初始化失败或还未被初始化"))
    }
}
