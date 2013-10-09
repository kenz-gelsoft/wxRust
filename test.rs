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
    unsafe {
        let closure = wxClosure_Create(wx_main as *u8, nullptr);
        let args: ~[*i32] = ~[];
        ELJApp_InitializeC(closure, args.len() as i32, vec::raw::to_ptr(args) as *i32);
    }
}

extern "C"
fn wx_main() {
    unsafe {
        // not mandatory
        // ELJApp_InitAllImageHandlers();
        let idAny = -1;
        let defaultFrameStyle = 536878656 | 4194304;
        do "Hello, wxRust!".to_c_str().with_ref |s| {
            let title = wxString_CreateUTF8(s as *u8);
            let frame = wxFrame_Create(nullptr, idAny, title, -1, -1, -1, -1, defaultFrameStyle);
            println("OK");
            wxWindow_Show(frame);
//            wxWindow_Raise(frame);
        }
    }
}

