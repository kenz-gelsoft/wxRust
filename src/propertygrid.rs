use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct wxPropertyGrid { ptr: *mut c_void }
impl _wxPropertyGrid for wxPropertyGrid {}
impl _wxControl for wxPropertyGrid {}
impl _wxWindow for wxPropertyGrid {}
impl _wxEvtHandler for wxPropertyGrid {}
impl _wxObject for wxPropertyGrid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPropertyGrid {
    pub fn from(ptr: *mut c_void) -> wxPropertyGrid { wxPropertyGrid { ptr: ptr } }
    pub fn null() -> wxPropertyGrid { wxPropertyGrid::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxPropertyGrid {
        unsafe { wxPropertyGrid { ptr: wxPropertyGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxPropertyGrid : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append<T: _wxPGProperty>(&self, prop: &T) -> wxPGProperty {
        unsafe { wxPGProperty { ptr: wxPropertyGrid_Append(self.ptr(), prop.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableProperty(&self, propName: &str) -> c_int {
        let propName = wxT(propName);
        unsafe { wxPropertyGrid_DisableProperty(self.ptr(), propName.ptr()) }
    }
}

pub struct wxPropertyGridEvent { ptr: *mut c_void }
impl _wxPropertyGridEvent for wxPropertyGridEvent {}
impl _wxNotifyEvent for wxPropertyGridEvent {}
impl _wxCommandEvent for wxPropertyGridEvent {}
impl _wxEvent for wxPropertyGridEvent {}
impl _wxObject for wxPropertyGridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPropertyGridEvent {
    pub fn from(ptr: *mut c_void) -> wxPropertyGridEvent { wxPropertyGridEvent { ptr: ptr } }
    pub fn null() -> wxPropertyGridEvent { wxPropertyGridEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPropertyGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasProperty(&self) -> c_int {
        unsafe { wxPropertyGridEvent_HasProperty(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getProperty(&self) -> wxPGProperty {
        unsafe { wxPGProperty { ptr: wxPropertyGridEvent_GetProperty(self.ptr()) } }
    }
}

pub struct wxPGProperty { ptr: *mut c_void }
impl _wxPGProperty for wxPGProperty {}
impl _wxObject for wxPGProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPGProperty {
    pub fn from(ptr: *mut c_void) -> wxPGProperty { wxPGProperty { ptr: ptr } }
    pub fn null() -> wxPGProperty { wxPGProperty::from(0 as *mut c_void) }
    
}

pub trait _wxPGProperty : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { ptr: wxPGProperty_GetLabel(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { ptr: wxPGProperty_GetName(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValueAsString(&self) -> ~str {
        unsafe { wxString { ptr: wxPGProperty_GetValueAsString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValueType(&self) -> ~str {
        unsafe { wxString { ptr: wxPGProperty_GetValueType(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxPGProperty_SetHelpString(self.ptr(), helpString.ptr()) }
    }
}

pub struct wxStringProperty { ptr: *mut c_void }
impl _wxStringProperty for wxStringProperty {}
impl _wxPGProperty for wxStringProperty {}
impl _wxObject for wxStringProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStringProperty {
    pub fn from(ptr: *mut c_void) -> wxStringProperty { wxStringProperty { ptr: ptr } }
    pub fn null() -> wxStringProperty { wxStringProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: &str) -> wxStringProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { wxStringProperty { ptr: wxStringProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait _wxStringProperty : _wxPGProperty {
}

pub struct wxIntProperty { ptr: *mut c_void }
impl _wxIntProperty for wxIntProperty {}
impl _wxPGProperty for wxIntProperty {}
impl _wxObject for wxIntProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIntProperty {
    pub fn from(ptr: *mut c_void) -> wxIntProperty { wxIntProperty { ptr: ptr } }
    pub fn null() -> wxIntProperty { wxIntProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_int) -> wxIntProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { wxIntProperty { ptr: wxIntProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait _wxIntProperty : _wxPGProperty {
}

pub struct wxBoolProperty { ptr: *mut c_void }
impl _wxBoolProperty for wxBoolProperty {}
impl _wxPGProperty for wxBoolProperty {}
impl _wxObject for wxBoolProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBoolProperty {
    pub fn from(ptr: *mut c_void) -> wxBoolProperty { wxBoolProperty { ptr: ptr } }
    pub fn null() -> wxBoolProperty { wxBoolProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_int) -> wxBoolProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { wxBoolProperty { ptr: wxBoolProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait _wxBoolProperty : _wxPGProperty {
}

pub struct wxFloatProperty { ptr: *mut c_void }
impl _wxFloatProperty for wxFloatProperty {}
impl _wxPGProperty for wxFloatProperty {}
impl _wxObject for wxFloatProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFloatProperty {
    pub fn from(ptr: *mut c_void) -> wxFloatProperty { wxFloatProperty { ptr: ptr } }
    pub fn null() -> wxFloatProperty { wxFloatProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_float) -> wxFloatProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { wxFloatProperty { ptr: wxFloatProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait _wxFloatProperty : _wxPGProperty {
}

pub struct wxDateProperty { ptr: *mut c_void }
impl _wxDateProperty for wxDateProperty {}
impl _wxPGProperty for wxDateProperty {}
impl _wxObject for wxDateProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDateProperty {
    pub fn from(ptr: *mut c_void) -> wxDateProperty { wxDateProperty { ptr: ptr } }
    pub fn null() -> wxDateProperty { wxDateProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxDateTime>(label: &str, name: &str, value: &T) -> wxDateProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { wxDateProperty { ptr: wxDateProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait _wxDateProperty : _wxPGProperty {
}

pub struct wxFileProperty { ptr: *mut c_void }
impl _wxFileProperty for wxFileProperty {}
impl _wxPGProperty for wxFileProperty {}
impl _wxObject for wxFileProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileProperty {
    pub fn from(ptr: *mut c_void) -> wxFileProperty { wxFileProperty { ptr: ptr } }
    pub fn null() -> wxFileProperty { wxFileProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: &str) -> wxFileProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { wxFileProperty { ptr: wxFileProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait _wxFileProperty : _wxPGProperty {
}

pub struct wxPropertyCategory { ptr: *mut c_void }
impl _wxPropertyCategory for wxPropertyCategory {}
impl _wxPGProperty for wxPropertyCategory {}
impl _wxObject for wxPropertyCategory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPropertyCategory {
    pub fn from(ptr: *mut c_void) -> wxPropertyCategory { wxPropertyCategory { ptr: ptr } }
    pub fn null() -> wxPropertyCategory { wxPropertyCategory::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str) -> wxPropertyCategory {
        let label = wxT(label);
        unsafe { wxPropertyCategory { ptr: wxPropertyCategory_Create(label.ptr()) } }
    }
}

pub trait _wxPropertyCategory : _wxPGProperty {
}

