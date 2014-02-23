use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct WxGLCanvas { ptr: *mut c_void }
impl TWxGLCanvas for WxGLCanvas {}
impl TWxScrolledWindow for WxGLCanvas {}
impl TWxPanel for WxGLCanvas {}
impl TWxWindow for WxGLCanvas {}
impl TWxEvtHandler for WxGLCanvas {}
impl TWxObject for WxGLCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGLCanvas {
    pub fn from(ptr: *mut c_void) -> WxGLCanvas { WxGLCanvas { ptr: ptr } }
    pub fn null() -> WxGLCanvas { WxGLCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxPalette>(parent: &T, windowID: c_int, attributes: *mut c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: &str, palette: &U) -> WxGLCanvas {
        let title = wxT(title);
        unsafe { WxGLCanvas { ptr: wxGLCanvas_Create(parent.ptr(), windowID, attributes, x, y, w, h, style, title.ptr(), palette.ptr()) } }
    }
    pub fn isDisplaySupported(attributes: *mut c_int) -> c_int {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    pub fn isExtensionSupported(extension: &str) -> c_int {
        let extension = wxT(extension);
        unsafe { wxGLCanvas_IsExtensionSupported(extension.ptr()) }
    }
}

pub trait TWxGLCanvas : TWxScrolledWindow {
    fn setColour<T: TWxColour>(&self, colour: &T) -> c_int {
        unsafe { wxGLCanvas_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCurrent<T: TWxGLContext>(&self, ctxt: &T) -> c_int {
        unsafe { wxGLCanvas_SetCurrent(self.ptr(), ctxt.ptr()) }
    }
    fn swapBuffers(&self) -> c_int {
        unsafe { wxGLCanvas_SwapBuffers(self.ptr()) }
    }
}

pub struct WxGLContext { ptr: *mut c_void }
impl TWxGLContext for WxGLContext {}
impl TWxObject for WxGLContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGLContext {
    pub fn from(ptr: *mut c_void) -> WxGLContext { WxGLContext { ptr: ptr } }
    pub fn null() -> WxGLContext { WxGLContext::from(0 as *mut c_void) }
    
    pub fn new<T: TWxGLCanvas, U: TWxGLContext>(win: &T, other: &U) -> WxGLContext {
        unsafe { WxGLContext { ptr: wxGLContext_Create(win.ptr(), other.ptr()) } }
    }
    pub fn newFromNull<T: TWxGLCanvas>(win: &T) -> WxGLContext {
        unsafe { WxGLContext { ptr: wxGLContext_CreateFromNull(win.ptr()) } }
    }
}

pub trait TWxGLContext : TWxObject {
    fn setCurrent<T: TWxGLCanvas>(&self, win: &T) -> c_int {
        unsafe { wxGLContext_SetCurrent(self.ptr(), win.ptr()) }
    }
}

