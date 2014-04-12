#[crate_id = "test#0.1"];

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

fn make_frame() -> Frame {
    let frame = Frame::new(&Window::null(), ID_ANY, "Hello, wxRust!", -1, -1, -1, -1, DEFAULT_FRAME_STYLE);
    let menubar = make_menubar();
    frame.setMenuBar(&menubar);
    
    make_button(&frame);

    frame
}

fn make_menubar() -> MenuBar {
    let menubar = MenuBar::new(0);
    
    let fileMenu = Menu::new("", 0);
    let fileNew = MenuItem::newEx(ID_ANY, "New", "Create a new file.", 0, &Menu::null());
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
    let parent = Window::from(data);
    let msgDlg = MessageDialog::new(&parent, "Pushed!!", "The Button", OK);
    msgDlg.showModal();
}

fn make_button<T: WindowMethods>(parent: &T) -> Button {
    let button = Button::new(parent, ID_ANY, "Push me!", 10, 10, 50, 30, 0);
    let closure = Closure::new(MyButton_clicked as *mut c_void, parent.ptr());
    unsafe {
        button.connect(ID_ANY, ID_ANY, expEVT_COMMAND_BUTTON_CLICKED(), closure.ptr());
    }

    button
}
