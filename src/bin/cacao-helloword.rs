
fn main() {
    #[cfg(target_os = "macos")]
    {
        use ::cacao::appkit::{App, AppDelegate, window::Window};
        #[derive(Default)]
        struct BasicApp {
            window: Window
        }
        impl AppDelegate for BasicApp {
            fn did_finish_launching(&self) {
            self.window.set_minimum_content_size(400., 400.);
            self.window.set_title("Hello World!");
            self.window.show();
            }
        }
        App::new("com.hello.world", BasicApp::default()).run();
    }
}