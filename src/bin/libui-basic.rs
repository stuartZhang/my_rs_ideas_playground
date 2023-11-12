#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, feature(trace_macros, log_syntax))]
use libui::{controls::{Button, Group, Label, VerticalBox}, prelude::*};
fn main() {
    // 1. 创建一个上下文
    let ui = UI::init().expect("Couldn't initialize UI library");
    // 2. 从上下文，创建一个主窗体实例。
    let mut win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);
    win.set_child({
        let mut vbox = VerticalBox::new(); // 3. 创建布局对象
        vbox.set_padded(true);
        vbox.append({ // 4. 创建与添加带换行的标签
            let mut label_text = String::new();
            label_text.push_str("There is a ton of text in this label.\n");
            label_text.push_str("Pretty much every unicode character is supported.\n");
            label_text.push_str("🎉 用户界面");
            Label::new(&label_text)
        }, LayoutStrategy::Stretchy);    
        vbox.append({
            let mut group = Group::new("Group");
            group.set_child({
                let mut group_vbox = VerticalBox::new();
                group_vbox.append({ // 4. 创建按钮控件
                    let mut button = Button::new("Button");
                    button.on_clicked(move |btn| {
                        btn.set_text("Clicked!");
                    });
                    button
                }, LayoutStrategy::Compact);
                group_vbox.append({ // 创建退出按钮控件
                    let ui = ui.clone();
                    let mut quit_button = Button::new("Quit");        
                    quit_button.on_clicked(move |_| {
                        ui.quit();
                    });  
                    quit_button  
                }, LayoutStrategy::Compact);
                group_vbox
            });
            group
        }, LayoutStrategy::Compact);
        vbox
    });
    // 弹出主窗体
    win.show();
    // 阻塞主线程，直到主窗体关闭
    ui.main();
}
