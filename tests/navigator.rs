#[feature(globs)];
#[feature(macro_rules)];

extern mod wx;

use std::libc::*;

use wx::_unsafe::*;
use wx::base::*;
use wx::core::*;
use wx::defs::*;
use wx::gl::*;

mod macros;

wxApp!(wx_main)

fn on_paint(data: *mut c_void) {
    let gl  = wxGLCanvas::from(data);
    let ctx = wxGLContext::from(gl.getClientData().handle());
    gl.setCurrent(ctx);

    gl.swapBuffers();
}

extern "C"
fn wx_main()
{
    let frame = wxFrame::new(wxWindow::null(), wxID_ANY, "Netscape Navigator", -1, -1, -1, -1, wxDEFAULT_FRAME_STYLE);
    
    let gl = wxGLCanvas::new(frame, wxID_ANY, 0 as *mut c_int, -1, -1, -1, -1, 0, "GLCanvas", wxPalette::null());
    let ctx = wxGLContext::newFromNull(gl);
    let cd  = wxClientData::from(ctx.handle());
    gl.setClientData(cd);
    let closure = wxClosure::new(on_paint as *mut c_void, gl.handle());
    unsafe {
        gl.connect(wxID_ANY, wxID_ANY, expEVT_PAINT(), closure.handle());
    }

//    let bmp = wxBitmap::newDefault();
//    let bmp2 = wxBitmap::newDefault();
//    let tb = frame.newToolBar((wxNO_BORDER | wxHORIZONTAL) as i64);
//    println("ok");
//    tb.addTool2(wxID_ANY, "label", bmp, bmp2, wxITEM_NORMAL, "a", "b");
//    println("ok");
//    frame.setToolBar(tb);
    frame.show();
}