extern mod wx;

use std::rt::start_on_main_thread;

use wx::native::*;
use wx::types::*;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    do start_on_main_thread(argc, argv, crate_map) {
        unsafe {
            ELJApp_InitAllImageHandlers();
            let idAny = -1;
            let frameRect = Rect {x:0, y:0, w:200, h:300};
            let defaultFrameStyle = 536878656;
            do "hello world".to_c_str().with_ref |s| {
                let frame = wxFrame_Create(0 as *u8, idAny, s, frameRect, defaultFrameStyle);
            }
        }
    }
}
