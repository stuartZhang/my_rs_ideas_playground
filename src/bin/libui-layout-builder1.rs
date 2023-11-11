#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, feature(trace_macros, log_syntax))]
use libui::prelude::*;
fn main() {
    let ui = UI::init().unwrap();
    libui::layout! { &ui,
        let layout = VerticalBox(padded: true) {
            Stretchy: let form = Form(padded: true) {
                (Compact, "用户名"): let tb_user = Entry()
                (Compact, "密码"): let tb_passwd = PasswordEntry()
            }
            Compact: let bt_submit = Button("确定")
        }
    };    
    {
        let ui = ui.clone();
        bt_submit.on_clicked(move |_src| {
            ui.quit();
        });
    }    
    let mut window = Window::new(&ui, "布局宏的简单用例", 320, 200,WindowType::NoMenubar);
    window.set_child(layout);
    window.show();
    ui.main();
}
