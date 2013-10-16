#[feature(globs)];

extern mod wx;

use std::rt::start_on_main_thread;
use std::vec;

use wx::*;
use wx::native::*;
use wx::wrapper::*;


static nullptr: *u8 = 0 as *u8;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    start_on_main_thread(argc, argv, crate_map, on_main)
}

#[fixed_stack_segment]
fn on_main() {
    let closure = wxClosure::new(wx_main as *u8, nullptr);
    let args: ~[*i32] = ~[];
    ELJApp::initializeC(closure, args.len() as i32, vec::raw::to_ptr(args) as *i32);
}

#[fixed_stack_segment]
fn wxT(s: &str) -> wxString {
    unsafe {
        do s.to_c_str().with_ref |c_str| {
            wxString(wxString_CreateUTF8(c_str as *u8))
        }
    }
}

extern "C"
fn wx_main() {
    unsafe {
        // not mandatory
        // ELJApp::initAllImageHandlers();
        let idAny = -1;
        let defaultFrameStyle = 536878656 | 4194304;
        
        let frame = wxFrame::new(wxWindow(nullptr), idAny, wxT("Hello, wxRust!"), -1, -1, -1, -1, defaultFrameStyle);
        println("OK");
        
        let menubar = wxMenuBar::new(0);
        
        let fileMenu = wxMenu::new(wxT("File2"), 1);
        let fileNew = wxMenuItem::newEx(idAny, wxT("New"), wxT("Create a new file."), 0, wxMenu(nullptr));
        fileMenu.appendItem(fileNew);

        menubar.append(fileMenu, wxT("File"));
        
        frame.setMenuBar(menubar);

        let id = 30;
        let button = wxButton::new(frame, id, wxT("Push me!"), 10, 10, 50, 30, 0);
        fn button_clicked(fun: *u8, data: *u8, evt: *u8) {
            println("hello!");
            let frame = wxFrame(data);
            let msgDlg = wxMessageDialog::new(frame, wxT("Pushed!!"), wxT("The Button"), 0);
            msgDlg.showModal();
        }
        let closure = wxClosure::new(button_clicked as *u8, frame.handle());
        button.connect(id, id, expEVT_COMMAND_BUTTON_CLICKED(), closure.handle());
        
        frame.show();
        frame.raise();
    }
}

