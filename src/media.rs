use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

/// Wraps the wxWidgets' [wxMediaCtrl](http://docs.wxwidgets.org/3.0/classwx_media_ctrl.html) class.
pub struct MediaCtrl { ptr: *mut c_void }
impl TMediaCtrl for MediaCtrl {}
impl TWindow for MediaCtrl {}
impl TEvtHandler for MediaCtrl {}
impl TObject for MediaCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MediaCtrl {
    pub fn from(ptr: *mut c_void) -> MediaCtrl { MediaCtrl { ptr: ptr } }
    pub fn null() -> MediaCtrl { MediaCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(parent: &T, windowID: c_int, fileName: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: &str, name: &str) -> MediaCtrl {
        let fileName = wxT(fileName);
        let szBackend = wxT(szBackend);
        let name = wxT(name);
        unsafe { MediaCtrl { ptr: wxMediaCtrl_Create(parent.ptr(), windowID, fileName.ptr(), x, y, w, h, style, szBackend.ptr(), name.ptr()) } }
    }
}

/// Methods of the wxWidgets' [wxMediaCtrl](http://docs.wxwidgets.org/3.0/classwx_media_ctrl.html) class.
pub trait TMediaCtrl : TWindow {
    fn getPlaybackRate(&self) -> c_double {
        unsafe { wxMediaCtrl_GetPlaybackRate(self.ptr()) }
    }
    fn getVolume(&self) -> c_double {
        unsafe { wxMediaCtrl_GetVolume(self.ptr()) }
    }
    fn getState(&self) -> c_int {
        unsafe { wxMediaCtrl_GetState(self.ptr()) }
    }
    fn length(&self) -> i64 {
        unsafe { wxMediaCtrl_Length(self.ptr()) }
    }
    fn load(&self, fileName: &str) -> c_int {
        let fileName = wxT(fileName);
        unsafe { wxMediaCtrl_Load(self.ptr(), fileName.ptr()) }
    }
    fn loadURI(&self, uri: &str) -> c_int {
        let uri = wxT(uri);
        unsafe { wxMediaCtrl_LoadURI(self.ptr(), uri.ptr()) }
    }
    fn loadURIWithProxy(&self, uri: &str, proxy: &str) -> c_int {
        let uri = wxT(uri);
        let proxy = wxT(proxy);
        unsafe { wxMediaCtrl_LoadURIWithProxy(self.ptr(), uri.ptr(), proxy.ptr()) }
    }
    fn pause(&self) -> c_int {
        unsafe { wxMediaCtrl_Pause(self.ptr()) }
    }
    fn play(&self) -> c_int {
        unsafe { wxMediaCtrl_Play(self.ptr()) }
    }
    fn seek(&self, offsetWhere: i64, mode: c_int) -> i64 {
        unsafe { wxMediaCtrl_Seek(self.ptr(), offsetWhere, mode) }
    }
    fn setPlaybackRate(&self, dRate: c_double) -> c_int {
        unsafe { wxMediaCtrl_SetPlaybackRate(self.ptr(), dRate) }
    }
    fn setVolume(&self, dVolume: c_double) -> c_int {
        unsafe { wxMediaCtrl_SetVolume(self.ptr(), dVolume) }
    }
    fn showPlayerControls(&self, flags: c_int) -> c_int {
        unsafe { wxMediaCtrl_ShowPlayerControls(self.ptr(), flags) }
    }
    fn stop(&self) -> c_int {
        unsafe { wxMediaCtrl_Stop(self.ptr()) }
    }
    fn tell(&self) -> i64 {
        unsafe { wxMediaCtrl_Tell(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMediaEvent](http://docs.wxwidgets.org/3.0/classwx_media_event.html) class.
pub struct MediaEvent { ptr: *mut c_void }
impl TMediaEvent for MediaEvent {}
impl TNotifyEvent for MediaEvent {}
impl TCommandEvent for MediaEvent {}
impl TEvent for MediaEvent {}
impl TObject for MediaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MediaEvent {
    pub fn from(ptr: *mut c_void) -> MediaEvent { MediaEvent { ptr: ptr } }
    pub fn null() -> MediaEvent { MediaEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMediaEvent](http://docs.wxwidgets.org/3.0/classwx_media_event.html) class.
pub trait TMediaEvent : TNotifyEvent {
}

