use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct wxMediaCtrl { handle: *mut c_void }
impl _wxMediaCtrl for wxMediaCtrl {}
impl _wxWindow for wxMediaCtrl {}
impl _wxEvtHandler for wxMediaCtrl {}
impl _wxObject for wxMediaCtrl { fn handle(&self) -> *mut c_void { self.handle } }

impl wxMediaCtrl {
    pub fn from(handle: *mut c_void) -> wxMediaCtrl { wxMediaCtrl { handle: handle } }
    pub fn null() -> wxMediaCtrl { wxMediaCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, windowID: c_int, fileName: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: &str, name: &str) -> wxMediaCtrl {
        let fileName = wxT(fileName);
        let szBackend = wxT(szBackend);
        let name = wxT(name);
        unsafe { wxMediaCtrl { handle: wxMediaCtrl_Create(parent.handle(), windowID, fileName.handle(), x, y, w, h, style, szBackend.handle(), name.handle()) } }
    }
}

pub trait _wxMediaCtrl : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPlaybackRate(&self) -> c_double {
        unsafe { wxMediaCtrl_GetPlaybackRate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVolume(&self) -> c_double {
        unsafe { wxMediaCtrl_GetVolume(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getState(&self) -> c_int {
        unsafe { wxMediaCtrl_GetState(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn length(&self) -> i64 {
        unsafe { wxMediaCtrl_Length(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, fileName: &str) -> c_int {
        let fileName = wxT(fileName);
        unsafe { wxMediaCtrl_Load(self.handle(), fileName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadURI(&self, uri: &str) -> c_int {
        let uri = wxT(uri);
        unsafe { wxMediaCtrl_LoadURI(self.handle(), uri.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadURIWithProxy(&self, uri: &str, proxy: &str) -> c_int {
        let uri = wxT(uri);
        let proxy = wxT(proxy);
        unsafe { wxMediaCtrl_LoadURIWithProxy(self.handle(), uri.handle(), proxy.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pause(&self) -> c_int {
        unsafe { wxMediaCtrl_Pause(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play(&self) -> c_int {
        unsafe { wxMediaCtrl_Play(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn seek(&self, offsetWhere: i64, mode: c_int) -> i64 {
        unsafe { wxMediaCtrl_Seek(self.handle(), offsetWhere, mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPlaybackRate(&self, dRate: c_double) -> c_int {
        unsafe { wxMediaCtrl_SetPlaybackRate(self.handle(), dRate) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVolume(&self, dVolume: c_double) -> c_int {
        unsafe { wxMediaCtrl_SetVolume(self.handle(), dVolume) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showPlayerControls(&self, flags: c_int) -> c_int {
        unsafe { wxMediaCtrl_ShowPlayerControls(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) -> c_int {
        unsafe { wxMediaCtrl_Stop(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tell(&self) -> i64 {
        unsafe { wxMediaCtrl_Tell(self.handle()) }
    }
}

pub struct wxMediaEvent { handle: *mut c_void }
impl _wxMediaEvent for wxMediaEvent {}
impl _wxNotifyEvent for wxMediaEvent {}
impl _wxCommandEvent for wxMediaEvent {}
impl _wxEvent for wxMediaEvent {}
impl _wxObject for wxMediaEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxMediaEvent {
    pub fn from(handle: *mut c_void) -> wxMediaEvent { wxMediaEvent { handle: handle } }
    pub fn null() -> wxMediaEvent { wxMediaEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMediaEvent : _wxNotifyEvent {
}

