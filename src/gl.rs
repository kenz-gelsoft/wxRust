use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

/// Wraps the wxWidgets' [wxGLCanvas](http://docs.wxwidgets.org/3.0/classwx_glc_anvas.html) class.
pub struct GLCanvas { ptr: *mut c_void }
impl GLCanvasMethods for GLCanvas {}
impl ScrolledWindowMethods for GLCanvas {}
impl PanelMethods for GLCanvas {}
impl WindowMethods for GLCanvas {}
impl EvtHandlerMethods for GLCanvas {}
impl ObjectMethods for GLCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GLCanvas {
    pub fn from(ptr: *mut c_void) -> GLCanvas { GLCanvas { ptr: ptr } }
    pub fn null() -> GLCanvas { GLCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: PaletteMethods>(parent: &T, windowID: c_int, attributes: *mut c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: &str, palette: &U) -> GLCanvas {
        let title = strToString(title);
        unsafe { GLCanvas::from(wxGLCanvas_Create(parent.ptr(), windowID, attributes, x, y, w, h, style, title.ptr(), palette.ptr())) }
    }
    pub fn isDisplaySupported(attributes: *mut c_int) -> c_int {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    pub fn isExtensionSupported(extension: &str) -> c_int {
        let extension = strToString(extension);
        unsafe { wxGLCanvas_IsExtensionSupported(extension.ptr()) }
    }
}

/// Methods of the wxWidgets' [wxGLCanvas](http://docs.wxwidgets.org/3.0/classwx_glc_anvas.html) class.
pub trait GLCanvasMethods : ScrolledWindowMethods {
    fn setColour<T: ColourMethods>(&self, colour: &T) -> c_int {
        unsafe { wxGLCanvas_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCurrent<T: GLContextMethods>(&self, ctxt: &T) -> c_int {
        unsafe { wxGLCanvas_SetCurrent(self.ptr(), ctxt.ptr()) }
    }
    fn swapBuffers(&self) -> c_int {
        unsafe { wxGLCanvas_SwapBuffers(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxGLContext](http://docs.wxwidgets.org/3.0/classwx_glc_ontext.html) class.
pub struct GLContext { ptr: *mut c_void }
impl GLContextMethods for GLContext {}
impl ObjectMethods for GLContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GLContext {
    pub fn from(ptr: *mut c_void) -> GLContext { GLContext { ptr: ptr } }
    pub fn null() -> GLContext { GLContext::from(0 as *mut c_void) }
    
    pub fn new<T: GLCanvasMethods, U: GLContextMethods>(win: &T, other: &U) -> GLContext {
        unsafe { GLContext::from(wxGLContext_Create(win.ptr(), other.ptr())) }
    }
    pub fn newFromNull<T: GLCanvasMethods>(win: &T) -> GLContext {
        unsafe { GLContext::from(wxGLContext_CreateFromNull(win.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxGLContext](http://docs.wxwidgets.org/3.0/classwx_glc_ontext.html) class.
pub trait GLContextMethods : ObjectMethods {
    fn setCurrent<T: GLCanvasMethods>(&self, win: &T) -> c_int {
        unsafe { wxGLContext_SetCurrent(self.ptr(), win.ptr()) }
    }
}

