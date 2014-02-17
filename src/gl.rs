use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct wxGLCanvas { handle: *mut c_void }
impl _wxGLCanvas for wxGLCanvas {}
impl _wxScrolledWindow for wxGLCanvas {}
impl _wxPanel for wxGLCanvas {}
impl _wxWindow for wxGLCanvas {}
impl _wxEvtHandler for wxGLCanvas {}
impl _wxObject for wxGLCanvas { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGLCanvas {
    pub fn from(handle: *mut c_void) -> wxGLCanvas { wxGLCanvas { handle: handle } }
    pub fn null() -> wxGLCanvas { wxGLCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow, U: _wxPalette>(parent: &T, windowID: c_int, attributes: *mut c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: &str, palette: &U) -> wxGLCanvas {
        let title = wxT(title);
        unsafe { wxGLCanvas { handle: wxGLCanvas_Create(parent.handle(), windowID, attributes, x, y, w, h, style, title.handle(), palette.handle()) } }
    }
    pub fn isDisplaySupported(attributes: *mut c_int) -> c_int {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    pub fn isExtensionSupported(extension: &str) -> c_int {
        let extension = wxT(extension);
        unsafe { wxGLCanvas_IsExtensionSupported(extension.handle()) }
    }
}

pub trait _wxGLCanvas : _wxScrolledWindow {
    fn setColour<T: _wxColour>(&self, colour: &T) -> c_int {
        unsafe { wxGLCanvas_SetColour(self.handle(), colour.handle()) }
    }
    fn setCurrent<T: _wxGLContext>(&self, ctxt: &T) -> c_int {
        unsafe { wxGLCanvas_SetCurrent(self.handle(), ctxt.handle()) }
    }
    fn swapBuffers(&self) -> c_int {
        unsafe { wxGLCanvas_SwapBuffers(self.handle()) }
    }
}

pub struct wxGLContext { handle: *mut c_void }
impl _wxGLContext for wxGLContext {}
impl _wxObject for wxGLContext { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGLContext {
    pub fn from(handle: *mut c_void) -> wxGLContext { wxGLContext { handle: handle } }
    pub fn null() -> wxGLContext { wxGLContext::from(0 as *mut c_void) }
    
    pub fn new<T: _wxGLCanvas, U: _wxGLContext>(win: &T, other: &U) -> wxGLContext {
        unsafe { wxGLContext { handle: wxGLContext_Create(win.handle(), other.handle()) } }
    }
    pub fn newFromNull<T: _wxGLCanvas>(win: &T) -> wxGLContext {
        unsafe { wxGLContext { handle: wxGLContext_CreateFromNull(win.handle()) } }
    }
}

pub trait _wxGLContext : _wxObject {
    fn setCurrent<T: _wxGLCanvas>(&self, win: &T) -> c_int {
        unsafe { wxGLContext_SetCurrent(self.handle(), win.handle()) }
    }
}

