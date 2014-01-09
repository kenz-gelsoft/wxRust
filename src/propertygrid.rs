use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct wxPropertyGrid { handle: *mut c_void }
impl _wxPropertyGrid for wxPropertyGrid {}
impl _wxControl for wxPropertyGrid {}
impl _wxWindow for wxPropertyGrid {}
impl _wxEvtHandler for wxPropertyGrid {}
impl _wxObject for wxPropertyGrid { fn handle(&self) -> *mut c_void { self.handle } }

impl wxPropertyGrid {
    pub fn from(handle: *mut c_void) -> @wxPropertyGrid { @wxPropertyGrid { handle: handle } }
    pub fn null() -> @wxPropertyGrid { wxPropertyGrid::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxPropertyGrid {
        unsafe { @wxPropertyGrid { handle: wxPropertyGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxPropertyGrid : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append<T: _wxPGProperty>(&self, prop: &T) -> @wxPGProperty {
        unsafe { @wxPGProperty { handle: wxPropertyGrid_Append(self.handle(), prop.handle()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableProperty(&self, propName: &str) -> c_int {
        let propName = wxT(propName);
        unsafe { wxPropertyGrid_DisableProperty(self.handle(), propName.handle()) }
    }
}

pub struct wxPropertyGridEvent { handle: *mut c_void }
impl _wxPropertyGridEvent for wxPropertyGridEvent {}
impl _wxNotifyEvent for wxPropertyGridEvent {}
impl _wxCommandEvent for wxPropertyGridEvent {}
impl _wxEvent for wxPropertyGridEvent {}
impl _wxObject for wxPropertyGridEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxPropertyGridEvent {
    pub fn from(handle: *mut c_void) -> @wxPropertyGridEvent { @wxPropertyGridEvent { handle: handle } }
    pub fn null() -> @wxPropertyGridEvent { wxPropertyGridEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPropertyGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasProperty(&self) -> c_int {
        unsafe { wxPropertyGridEvent_HasProperty(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getProperty(&self) -> @wxPGProperty {
        unsafe { @wxPGProperty { handle: wxPropertyGridEvent_GetProperty(self.handle()) } }
    }
}

pub struct wxPGProperty { handle: *mut c_void }
impl _wxPGProperty for wxPGProperty {}
impl _wxObject for wxPGProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxPGProperty {
    pub fn from(handle: *mut c_void) -> @wxPGProperty { @wxPGProperty { handle: handle } }
    pub fn null() -> @wxPGProperty { wxPGProperty::from(0 as *mut c_void) }
    
}

pub trait _wxPGProperty : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetLabel(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValueAsString(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetValueAsString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValueType(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetValueType(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxPGProperty_SetHelpString(self.handle(), helpString.handle()) }
    }
}

pub struct wxStringProperty { handle: *mut c_void }
impl _wxStringProperty for wxStringProperty {}
impl _wxPGProperty for wxStringProperty {}
impl _wxObject for wxStringProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxStringProperty {
    pub fn from(handle: *mut c_void) -> @wxStringProperty { @wxStringProperty { handle: handle } }
    pub fn null() -> @wxStringProperty { wxStringProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: &str) -> @wxStringProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { @wxStringProperty { handle: wxStringProperty_Create(label.handle(), name.handle(), value.handle()) } }
    }
}

pub trait _wxStringProperty : _wxPGProperty {
}

pub struct wxIntProperty { handle: *mut c_void }
impl _wxIntProperty for wxIntProperty {}
impl _wxPGProperty for wxIntProperty {}
impl _wxObject for wxIntProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxIntProperty {
    pub fn from(handle: *mut c_void) -> @wxIntProperty { @wxIntProperty { handle: handle } }
    pub fn null() -> @wxIntProperty { wxIntProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_int) -> @wxIntProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxIntProperty { handle: wxIntProperty_Create(label.handle(), name.handle(), value) } }
    }
}

pub trait _wxIntProperty : _wxPGProperty {
}

pub struct wxBoolProperty { handle: *mut c_void }
impl _wxBoolProperty for wxBoolProperty {}
impl _wxPGProperty for wxBoolProperty {}
impl _wxObject for wxBoolProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxBoolProperty {
    pub fn from(handle: *mut c_void) -> @wxBoolProperty { @wxBoolProperty { handle: handle } }
    pub fn null() -> @wxBoolProperty { wxBoolProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_int) -> @wxBoolProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxBoolProperty { handle: wxBoolProperty_Create(label.handle(), name.handle(), value) } }
    }
}

pub trait _wxBoolProperty : _wxPGProperty {
}

pub struct wxFloatProperty { handle: *mut c_void }
impl _wxFloatProperty for wxFloatProperty {}
impl _wxPGProperty for wxFloatProperty {}
impl _wxObject for wxFloatProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxFloatProperty {
    pub fn from(handle: *mut c_void) -> @wxFloatProperty { @wxFloatProperty { handle: handle } }
    pub fn null() -> @wxFloatProperty { wxFloatProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_float) -> @wxFloatProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxFloatProperty { handle: wxFloatProperty_Create(label.handle(), name.handle(), value) } }
    }
}

pub trait _wxFloatProperty : _wxPGProperty {
}

pub struct wxDateProperty { handle: *mut c_void }
impl _wxDateProperty for wxDateProperty {}
impl _wxPGProperty for wxDateProperty {}
impl _wxObject for wxDateProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDateProperty {
    pub fn from(handle: *mut c_void) -> @wxDateProperty { @wxDateProperty { handle: handle } }
    pub fn null() -> @wxDateProperty { wxDateProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxDateTime>(label: &str, name: &str, value: &T) -> @wxDateProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxDateProperty { handle: wxDateProperty_Create(label.handle(), name.handle(), value.handle()) } }
    }
}

pub trait _wxDateProperty : _wxPGProperty {
}

pub struct wxFileProperty { handle: *mut c_void }
impl _wxFileProperty for wxFileProperty {}
impl _wxPGProperty for wxFileProperty {}
impl _wxObject for wxFileProperty { fn handle(&self) -> *mut c_void { self.handle } }

impl wxFileProperty {
    pub fn from(handle: *mut c_void) -> @wxFileProperty { @wxFileProperty { handle: handle } }
    pub fn null() -> @wxFileProperty { wxFileProperty::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: &str) -> @wxFileProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { @wxFileProperty { handle: wxFileProperty_Create(label.handle(), name.handle(), value.handle()) } }
    }
}

pub trait _wxFileProperty : _wxPGProperty {
}

pub struct wxPropertyCategory { handle: *mut c_void }
impl _wxPropertyCategory for wxPropertyCategory {}
impl _wxPGProperty for wxPropertyCategory {}
impl _wxObject for wxPropertyCategory { fn handle(&self) -> *mut c_void { self.handle } }

impl wxPropertyCategory {
    pub fn from(handle: *mut c_void) -> @wxPropertyCategory { @wxPropertyCategory { handle: handle } }
    pub fn null() -> @wxPropertyCategory { wxPropertyCategory::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str) -> @wxPropertyCategory {
        let label = wxT(label);
        unsafe { @wxPropertyCategory { handle: wxPropertyCategory_Create(label.handle()) } }
    }
}

pub trait _wxPropertyCategory : _wxPGProperty {
}

