#[feature(globs)];
#[feature(macro_rules)];
#[feature(managed_boxes)];

#[link_args="-lwxc"];

extern mod native;
extern mod wx;

use std::libc::c_void;

use wx::_unsafe::*;
use wx::defs::*;
use wx::base::*;
use wx::core::*;

mod macros;


wxApp!(wx_main)

extern "C"
fn wx_main() {
    let frame = MyFrame::new();
    frame.show();
    frame.raise();
}

struct MyFrame(@wxFrame);
impl MyFrame {
    fn new() -> MyFrame {
        let frame = wxFrame::new(wxWindow::null(), wxID_ANY, "Hello, wxRust!", -1, -1, -1, -1, wxDEFAULT_FRAME_STYLE);
        let menubar = MyMenuBar::new();
        frame.setMenuBar(menubar.asMenuBar());
        
        MyButton::new(frame);

        MyFrame(frame)
    }
}

struct MyMenuBar(@wxMenuBar);
impl MyMenuBar {
    fn new() -> MyMenuBar {
        let menubar = wxMenuBar::new(0);
        
        let fileMenu = wxMenu::new("", 0);
        let fileNew = wxMenuItem::newEx(wxID_ANY, "New", "Create a new file.", 0, wxMenu::null());
        fileMenu.appendItem(fileNew);

        menubar.append(fileMenu, "File");
        MyMenuBar(menubar)
    }
    fn asMenuBar(&self) -> @wxMenuBar {
        return **self;
    }
}

extern "C"
fn MyButton_clicked(fun: *mut c_void, data: *mut c_void, evt: *mut c_void) {
    println("hello!");
    let parent = wxWindow::from(data);
    let msgDlg = wxMessageDialog::new(parent, "Pushed!!", "The Button", wxOK);
    msgDlg.showModal();
}

struct MyButton(@wxButton);
impl MyButton {
    fn new<T: _wxWindow>(parent: &T) -> MyButton {
        let button = wxButton::new(parent, wxID_ANY, "Push me!", 10, 10, 50, 30, 0);
        let closure = wxClosure::new(MyButton_clicked as *mut c_void, parent.handle());
        unsafe {
            button.connect(wxID_ANY, wxID_ANY, expEVT_COMMAND_BUTTON_CLICKED(), closure.handle());
        }

        MyButton(button)
    }
}
