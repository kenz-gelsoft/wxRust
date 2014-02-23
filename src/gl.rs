use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct GLCanvas { ptr: *mut c_void }
impl TGLCanvas for GLCanvas {}
impl TScrolledWindow for GLCanvas {}
impl TPanel for GLCanvas {}
impl TWindow for GLCanvas {}
impl TEvtHandler for GLCanvas {}
impl TObject for GLCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GLCanvas {
    pub fn from(ptr: *mut c_void) -> GLCanvas { GLCanvas { ptr: ptr } }
    pub fn null() -> GLCanvas { GLCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TPalette>(parent: &T, windowID: c_int, attributes: *mut c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: &str, palette: &U) -> GLCanvas {
        let title = wxT(title);
        unsafe { GLCanvas { ptr: wxGLCanvas_Create(parent.ptr(), windowID, attributes, x, y, w, h, style, title.ptr(), palette.ptr()) } }
    }
    pub fn isDisplaySupported(attributes: *mut c_int) -> c_int {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    pub fn isExtensionSupported(extension: &str) -> c_int {
        let extension = wxT(extension);
        unsafe { wxGLCanvas_IsExtensionSupported(extension.ptr()) }
    }
}

pub trait TGLCanvas : TScrolledWindow {
    fn setColour<T: TColour>(&self, colour: &T) -> c_int {
        unsafe { wxGLCanvas_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCurrent<T: TGLContext>(&self, ctxt: &T) -> c_int {
        unsafe { wxGLCanvas_SetCurrent(self.ptr(), ctxt.ptr()) }
    }
    fn swapBuffers(&self) -> c_int {
        unsafe { wxGLCanvas_SwapBuffers(self.ptr()) }
    }
}

pub struct GLContext { ptr: *mut c_void }
impl TGLContext for GLContext {}
impl TObject for GLContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GLContext {
    pub fn from(ptr: *mut c_void) -> GLContext { GLContext { ptr: ptr } }
    pub fn null() -> GLContext { GLContext::from(0 as *mut c_void) }
    
    pub fn new<T: TGLCanvas, U: TGLContext>(win: &T, other: &U) -> GLContext {
        unsafe { GLContext { ptr: wxGLContext_Create(win.ptr(), other.ptr()) } }
    }
    pub fn newFromNull<T: TGLCanvas>(win: &T) -> GLContext {
        unsafe { GLContext { ptr: wxGLContext_CreateFromNull(win.ptr()) } }
    }
}

pub trait TGLContext : TObject {
    fn setCurrent<T: TGLCanvas>(&self, win: &T) -> c_int {
        unsafe { wxGLContext_SetCurrent(self.ptr(), win.ptr()) }
    }
}

