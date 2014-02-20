use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct wxGLCanvas { ptr: *mut c_void }
impl _wxGLCanvas for wxGLCanvas {}
impl _wxScrolledWindow for wxGLCanvas {}
impl _wxPanel for wxGLCanvas {}
impl _wxWindow for wxGLCanvas {}
impl _wxEvtHandler for wxGLCanvas {}
impl _wxObject for wxGLCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGLCanvas {
    pub fn from(ptr: *mut c_void) -> wxGLCanvas { wxGLCanvas { ptr: ptr } }
    pub fn null() -> wxGLCanvas { wxGLCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow, U: _wxPalette>(parent: &T, windowID: c_int, attributes: *mut c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: &str, palette: &U) -> wxGLCanvas {
        let title = wxT(title);
        unsafe { wxGLCanvas { ptr: wxGLCanvas_Create(parent.ptr(), windowID, attributes, x, y, w, h, style, title.ptr(), palette.ptr()) } }
    }
    pub fn isDisplaySupported(attributes: *mut c_int) -> c_int {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    pub fn isExtensionSupported(extension: &str) -> c_int {
        let extension = wxT(extension);
        unsafe { wxGLCanvas_IsExtensionSupported(extension.ptr()) }
    }
}

pub trait _wxGLCanvas : _wxScrolledWindow {
    fn setColour<T: _wxColour>(&self, colour: &T) -> c_int {
        unsafe { wxGLCanvas_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCurrent<T: _wxGLContext>(&self, ctxt: &T) -> c_int {
        unsafe { wxGLCanvas_SetCurrent(self.ptr(), ctxt.ptr()) }
    }
    fn swapBuffers(&self) -> c_int {
        unsafe { wxGLCanvas_SwapBuffers(self.ptr()) }
    }
}

pub struct wxGLContext { ptr: *mut c_void }
impl _wxGLContext for wxGLContext {}
impl _wxObject for wxGLContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGLContext {
    pub fn from(ptr: *mut c_void) -> wxGLContext { wxGLContext { ptr: ptr } }
    pub fn null() -> wxGLContext { wxGLContext::from(0 as *mut c_void) }
    
    pub fn new<T: _wxGLCanvas, U: _wxGLContext>(win: &T, other: &U) -> wxGLContext {
        unsafe { wxGLContext { ptr: wxGLContext_Create(win.ptr(), other.ptr()) } }
    }
    pub fn newFromNull<T: _wxGLCanvas>(win: &T) -> wxGLContext {
        unsafe { wxGLContext { ptr: wxGLContext_CreateFromNull(win.ptr()) } }
    }
}

pub trait _wxGLContext : _wxObject {
    fn setCurrent<T: _wxGLCanvas>(&self, win: &T) -> c_int {
        unsafe { wxGLContext_SetCurrent(self.ptr(), win.ptr()) }
    }
}

