#[feature(globs)];
#[feature(macro_rules)];

#[link_args="-lwxc"];

extern crate native;
extern crate wx;

use std::libc::c_void;

use wx::_unsafe::*;
use wx::defs::*;
use wx::base::*;
use wx::core::*;

mod macros;


wxApp!(wx_main)

extern "C"
fn wx_main() {
    let frame = make_frame();
    frame.show();
    frame.raise();
}

fn make_frame() -> WxFrame {
    let frame = WxFrame::new(&WxWindow::null(), wxID_ANY, "Hello, wxRust!", -1, -1, -1, -1, wxDEFAULT_FRAME_STYLE);
    let menubar = make_menubar();
    frame.setMenuBar(&menubar);
    
    make_button(&frame);

    frame
}

fn make_menubar() -> WxMenuBar {
    let menubar = WxMenuBar::new(0);
    
    let fileMenu = WxMenu::new("", 0);
    let fileNew = WxMenuItem::newEx(wxID_ANY, "New", "Create a new file.", 0, &WxMenu::null());
    fileMenu.appendItem(&fileNew);

    menubar.append(&fileMenu, "File");
    menubar
}

extern "C"
fn MyButton_clicked(fun: *mut c_void, data: *mut c_void, evt: *mut c_void) {
    if evt == 0 as *mut c_void {
        // Comes here when the target widget is destroyed.
        return;
    }

    println!("hello!");
    let parent = WxWindow::from(data);
    let msgDlg = WxMessageDialog::new(&parent, "Pushed!!", "The Button", wxOK);
    msgDlg.showModal();
}

fn make_button<T: TWxWindow>(parent: &T) -> WxButton {
    let button = WxButton::new(parent, wxID_ANY, "Push me!", 10, 10, 50, 30, 0);
    let closure = WxClosure::new(MyButton_clicked as *mut c_void, parent.ptr());
    unsafe {
        button.connect(wxID_ANY, wxID_ANY, expEVT_COMMAND_BUTTON_CLICKED(), closure.ptr());
    }

    button
}
