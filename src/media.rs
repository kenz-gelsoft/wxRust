use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct WxMediaCtrl { ptr: *mut c_void }
impl TWxMediaCtrl for WxMediaCtrl {}
impl TWxWindow for WxMediaCtrl {}
impl TWxEvtHandler for WxMediaCtrl {}
impl TWxObject for WxMediaCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMediaCtrl {
    pub fn from(ptr: *mut c_void) -> WxMediaCtrl { WxMediaCtrl { ptr: ptr } }
    pub fn null() -> WxMediaCtrl { WxMediaCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(parent: &T, windowID: c_int, fileName: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: &str, name: &str) -> WxMediaCtrl {
        let fileName = wxT(fileName);
        let szBackend = wxT(szBackend);
        let name = wxT(name);
        unsafe { WxMediaCtrl { ptr: wxMediaCtrl_Create(parent.ptr(), windowID, fileName.ptr(), x, y, w, h, style, szBackend.ptr(), name.ptr()) } }
    }
}

pub trait TWxMediaCtrl : TWxWindow {
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

pub struct WxMediaEvent { ptr: *mut c_void }
impl TWxMediaEvent for WxMediaEvent {}
impl TWxNotifyEvent for WxMediaEvent {}
impl TWxCommandEvent for WxMediaEvent {}
impl TWxEvent for WxMediaEvent {}
impl TWxObject for WxMediaEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMediaEvent {
    pub fn from(ptr: *mut c_void) -> WxMediaEvent { WxMediaEvent { ptr: ptr } }
    pub fn null() -> WxMediaEvent { WxMediaEvent::from(0 as *mut c_void) }
    
}

pub trait TWxMediaEvent : TWxNotifyEvent {
}

