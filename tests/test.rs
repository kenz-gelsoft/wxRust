#[feature(globs)];
#[feature(macro_rules)];

extern mod wx;

use std::libc::*;
use std::rt::start_on_main_thread;
use std::vec;

use wx::defs::*;
use wx::native::*;
use wx::wrapper::*;


static nullptr: *mut c_void = 0 as *mut c_void;


macro_rules! wxApp(
    ($f: ident) => (
        fn start(argc: int, argv: **u8) -> int {
            #[start];
            fn on_main() {
                #[fixed_stack_segment];
                #[inline(never)];
                let closure = wxClosure::new($f as *mut c_void, nullptr);
                let args: ~[*i32] = ~[];
                ELJApp::initializeC(closure, args.len() as i32, vec::raw::to_ptr(args) as *mut *mut i8);
            }
            start_on_main_thread(argc, argv, on_main)
        }
    )
)

wxApp!(wx_main)

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
        let frame = wxFrame::new(wxWindow::null(), wxID_ANY, "Hello, wxRust!", -1, -1, -1, -1, wxDEFAULT_FRAME_STYLE);
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
        let fileNew = wxMenuItem::newEx(wxID_ANY, "New", "Create a new file.", 0, wxMenu::null());
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
        let button = wxButton::new(parent, wxID_ANY, "Push me!", 10, 10, 50, 30, 0);
        let closure = wxClosure::new(MyButton::clicked as *mut c_void, parent.handle());
        unsafe {
            button.connect(wxID_ANY, wxID_ANY, expEVT_COMMAND_BUTTON_CLICKED(), closure.handle());
        }

        MyButton(button)
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clicked(fun: *mut c_void, data: *mut c_void, evt: *mut c_void) {
        println("hello!");
        let parent = wxWindow::from(data);
        let msgDlg = wxMessageDialog::new(parent, "Pushed!!", "The Button", 0);
        msgDlg.showModal();
    }
}
