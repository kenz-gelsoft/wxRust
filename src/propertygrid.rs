use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct WxPropertyGrid { ptr: *mut c_void }
impl TWxPropertyGrid for WxPropertyGrid {}
impl TWxControl for WxPropertyGrid {}
impl TWxWindow for WxPropertyGrid {}
impl TWxEvtHandler for WxPropertyGrid {}
impl TWxObject for WxPropertyGrid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPropertyGrid {
    pub fn from(ptr: *mut c_void) -> WxPropertyGrid { WxPropertyGrid { ptr: ptr } }
    pub fn null() -> WxPropertyGrid { WxPropertyGrid::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxPropertyGrid {
        unsafe { WxPropertyGrid { ptr: wxPropertyGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxPropertyGrid : TWxControl {
    fn append<T: TWxPGProperty>(&self, prop: &T) -> WxPGProperty {
        unsafe { WxPGProperty { ptr: wxPropertyGrid_Append(self.ptr(), prop.ptr()) } }
    }
    fn disableProperty(&self, propName: &str) -> c_int {
        let propName = wxT(propName);
        unsafe { wxPropertyGrid_DisableProperty(self.ptr(), propName.ptr()) }
    }
}

pub struct WxPropertyGridEvent { ptr: *mut c_void }
impl TWxPropertyGridEvent for WxPropertyGridEvent {}
impl TWxNotifyEvent for WxPropertyGridEvent {}
impl TWxCommandEvent for WxPropertyGridEvent {}
impl TWxEvent for WxPropertyGridEvent {}
impl TWxObject for WxPropertyGridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPropertyGridEvent {
    pub fn from(ptr: *mut c_void) -> WxPropertyGridEvent { WxPropertyGridEvent { ptr: ptr } }
    pub fn null() -> WxPropertyGridEvent { WxPropertyGridEvent::from(0 as *mut c_void) }
    
}

pub trait TWxPropertyGridEvent : TWxNotifyEvent {
    fn hasProperty(&self) -> c_int {
        unsafe { wxPropertyGridEvent_HasProperty(self.ptr()) }
    }
    fn getProperty(&self) -> WxPGProperty {
        unsafe { WxPGProperty { ptr: wxPropertyGridEvent_GetProperty(self.ptr()) } }
    }
}

pub struct WxPGProperty { ptr: *mut c_void }
impl TWxPGProperty for WxPGProperty {}
impl TWxObject for WxPGProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPGProperty {
    pub fn from(ptr: *mut c_void) -> WxPGProperty { WxPGProperty { ptr: ptr } }
    pub fn null() -> WxPGProperty { WxPGProperty::from(0 as *mut c_void) }
    
}

pub trait TWxPGProperty : TWxObject {
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxPGProperty_GetLabel(self.ptr()) }.to_str() }
    }
    fn getName(&self) -> ~str {
        unsafe { WxString { ptr: wxPGProperty_GetName(self.ptr()) }.to_str() }
    }
    fn getValueAsString(&self) -> ~str {
        unsafe { WxString { ptr: wxPGProperty_GetValueAsString(self.ptr()) }.to_str() }
    }
    fn getValueType(&self) -> ~str {
        unsafe { WxString { ptr: wxPGProperty_GetValueType(self.ptr()) }.to_str() }
    }
    fn setHelpString(&self, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxPGProperty_SetHelpString(self.ptr(), helpString.ptr()) }
    }
}

pub struct WxStringProperty { ptr: *mut c_void }
impl TWxStringProperty for WxStringProperty {}
impl TWxPGProperty for WxStringProperty {}
impl TWxObject for WxStringProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStringProperty {
    pub fn from(ptr: *mut c_void) -> WxStringProperty { WxStringProperty { ptr: ptr } }
    pub fn null() -> WxStringProperty { WxStringProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: &str) -> WxStringProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { WxStringProperty { ptr: wxStringProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait TWxStringProperty : TWxPGProperty {
}

pub struct WxIntProperty { ptr: *mut c_void }
impl TWxIntProperty for WxIntProperty {}
impl TWxPGProperty for WxIntProperty {}
impl TWxObject for WxIntProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIntProperty {
    pub fn from(ptr: *mut c_void) -> WxIntProperty { WxIntProperty { ptr: ptr } }
    pub fn null() -> WxIntProperty { WxIntProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_int) -> WxIntProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { WxIntProperty { ptr: wxIntProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait TWxIntProperty : TWxPGProperty {
}

pub struct WxBoolProperty { ptr: *mut c_void }
impl TWxBoolProperty for WxBoolProperty {}
impl TWxPGProperty for WxBoolProperty {}
impl TWxObject for WxBoolProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBoolProperty {
    pub fn from(ptr: *mut c_void) -> WxBoolProperty { WxBoolProperty { ptr: ptr } }
    pub fn null() -> WxBoolProperty { WxBoolProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_int) -> WxBoolProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { WxBoolProperty { ptr: wxBoolProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait TWxBoolProperty : TWxPGProperty {
}

pub struct WxFloatProperty { ptr: *mut c_void }
impl TWxFloatProperty for WxFloatProperty {}
impl TWxPGProperty for WxFloatProperty {}
impl TWxObject for WxFloatProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFloatProperty {
    pub fn from(ptr: *mut c_void) -> WxFloatProperty { WxFloatProperty { ptr: ptr } }
    pub fn null() -> WxFloatProperty { WxFloatProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_float) -> WxFloatProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { WxFloatProperty { ptr: wxFloatProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait TWxFloatProperty : TWxPGProperty {
}

pub struct WxDateProperty { ptr: *mut c_void }
impl TWxDateProperty for WxDateProperty {}
impl TWxPGProperty for WxDateProperty {}
impl TWxObject for WxDateProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDateProperty {
    pub fn from(ptr: *mut c_void) -> WxDateProperty { WxDateProperty { ptr: ptr } }
    pub fn null() -> WxDateProperty { WxDateProperty::from(0 as *mut c_void) }
    
    pub fn new<T: TWxDateTime>(label: &str, name: &str, value: &T) -> WxDateProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { WxDateProperty { ptr: wxDateProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait TWxDateProperty : TWxPGProperty {
}

pub struct WxFileProperty { ptr: *mut c_void }
impl TWxFileProperty for WxFileProperty {}
impl TWxPGProperty for WxFileProperty {}
impl TWxObject for WxFileProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileProperty {
    pub fn from(ptr: *mut c_void) -> WxFileProperty { WxFileProperty { ptr: ptr } }
    pub fn null() -> WxFileProperty { WxFileProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: &str) -> WxFileProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { WxFileProperty { ptr: wxFileProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait TWxFileProperty : TWxPGProperty {
}

pub struct WxPropertyCategory { ptr: *mut c_void }
impl TWxPropertyCategory for WxPropertyCategory {}
impl TWxPGProperty for WxPropertyCategory {}
impl TWxObject for WxPropertyCategory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPropertyCategory {
    pub fn from(ptr: *mut c_void) -> WxPropertyCategory { WxPropertyCategory { ptr: ptr } }
    pub fn null() -> WxPropertyCategory { WxPropertyCategory::from(0 as *mut c_void) }
    
    pub fn new(label: &str) -> WxPropertyCategory {
        let label = wxT(label);
        unsafe { WxPropertyCategory { ptr: wxPropertyCategory_Create(label.ptr()) } }
    }
}

pub trait TWxPropertyCategory : TWxPGProperty {
}

