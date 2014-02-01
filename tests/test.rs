#[feature(globs)];
#[feature(macro_rules)];

#[link_args="-lwxc"];

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
    frame.asFrame().show();
    frame.asFrame().raise();
}

struct MyFrame { base: wxFrame }
impl MyFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn new() -> MyFrame {
        let frame = wxFrame::new(&wxWindow::null(), wxID_ANY, "Hello, wxRust!", -1, -1, -1, -1, wxDEFAULT_FRAME_STYLE);
        let menubar = MyMenuBar::new();
        frame.setMenuBar(&menubar.asMenuBar());
        
        MyButton::new(&frame);

        MyFrame { base: frame }
    }
    fn asFrame(&self) -> wxFrame {
        return self.base;
    }
}

struct MyMenuBar { base: wxMenuBar }
impl MyMenuBar {
    #[fixed_stack_segment]
    #[inline(never)]
    fn new() -> MyMenuBar {
        let menubar = wxMenuBar::new(0);
        
        let fileMenu = wxMenu::new("", 0);
        let fileNew = wxMenuItem::newEx(wxID_ANY, "New", "Create a new file.", 0, &wxMenu::null());
        fileMenu.appendItem(&fileNew);

        menubar.append(&fileMenu, "File");
        MyMenuBar { base: menubar }
    }
    fn asMenuBar(&self) -> wxMenuBar {
        return self.base;
    }
}

extern "C"
fn MyButton_clicked(fun: *mut c_void, data: *mut c_void, evt: *mut c_void) {
    if (evt == 0 as *mut c_void) {
        // Comes here when the target widget is destroyed.
        return;
    }

    println!("hello!");
    let parent = wxWindow::from(data);
    let msgDlg = wxMessageDialog::new(&parent, "Pushed!!", "The Button", wxOK);
    msgDlg.showModal();
}

struct MyButton(wxButton);
impl MyButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn new<T: _wxWindow>(parent: &T) -> MyButton {
        let button = wxButton::new(parent, wxID_ANY, "Push me!", 10, 10, 50, 30, 0);
        let closure = wxClosure::new(MyButton_clicked as *mut c_void, parent.handle());
        unsafe {
            button.connect(wxID_ANY, wxID_ANY, expEVT_COMMAND_BUTTON_CLICKED(), closure.handle());
        }

        MyButton(button)
    }
}
