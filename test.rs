#[feature(globs)];

extern mod wx;

use std::libc::*;
use std::rt::start_on_main_thread;
use std::vec;

use wx::native::*;
use wx::wrapper::*;


static nullptr: *u8 = 0 as *u8;
static idAny: c_int = -1;


#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    start_on_main_thread(argc, argv, crate_map, on_main)
}

#[fixed_stack_segment]
#[inline(never)]
fn on_main() {
    let closure = wxClosure::new(wx_main as *u8, nullptr);
    let args: ~[*i32] = ~[];
    ELJApp::initializeC(closure, args.len() as i32, vec::raw::to_ptr(args) as *i32);
}

extern "C"
fn wx_main() {
    let frame = MyFrame::new();
    frame.show();
    frame.raise();
}

struct MyFrame(@wxFrame);
impl MyFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn new() -> MyFrame {
        let defaultFrameStyle = 536878656 | 4194304;
        
        let frame = wxFrame::new(wxWindow::null(), idAny, "Hello, wxRust!", -1, -1, -1, -1, defaultFrameStyle);
        let menubar = MyMenuBar::new();
        frame.setMenuBar(menubar.asMenuBar());
        
        MyButton::new(frame);

        MyFrame(frame)
    }
}

struct MyMenuBar(@wxMenuBar);
impl MyMenuBar {
    #[fixed_stack_segment]
    #[inline(never)]
    fn new() -> MyMenuBar {
        let menubar = wxMenuBar::new(0);
        
        let fileMenu = wxMenu::new("", 0);
        let fileNew = wxMenuItem::newEx(idAny, "New", "Create a new file.", 0, wxMenu::null());
        fileMenu.appendItem(fileNew);

        menubar.append(fileMenu, "File");
        MyMenuBar(menubar)
    }
    fn asMenuBar(&self) -> @wxMenuBar {
        return **self;
    }
}

struct MyButton(@wxButton);
impl MyButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn new<T: _wxWindow>(parent: &T) -> MyButton {
        let button = wxButton::new(parent, idAny, "Push me!", 10, 10, 50, 30, 0);
        let closure = wxClosure::new(MyButton::clicked as *u8, parent.handle());
        unsafe {
            button.connect(idAny, idAny, expEVT_COMMAND_BUTTON_CLICKED(), closure.handle());
        }

        MyButton(button)
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clicked(fun: *u8, data: *u8, evt: *u8) {
        println("hello!");
        let parent = wxWindow::from(data);
        let msgDlg = wxMessageDialog::new(parent, "Pushed!!", "The Button", 0);
        msgDlg.showModal();
    }
}
