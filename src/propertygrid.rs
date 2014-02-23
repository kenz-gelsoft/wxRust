use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

pub struct PropertyGrid { ptr: *mut c_void }
impl TPropertyGrid for PropertyGrid {}
impl TControl for PropertyGrid {}
impl TWindow for PropertyGrid {}
impl TEvtHandler for PropertyGrid {}
impl TObject for PropertyGrid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PropertyGrid {
    pub fn from(ptr: *mut c_void) -> PropertyGrid { PropertyGrid { ptr: ptr } }
    pub fn null() -> PropertyGrid { PropertyGrid::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> PropertyGrid {
        unsafe { PropertyGrid { ptr: wxPropertyGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TPropertyGrid : TControl {
    fn append<T: TPGProperty>(&self, prop: &T) -> PGProperty {
        unsafe { PGProperty { ptr: wxPropertyGrid_Append(self.ptr(), prop.ptr()) } }
    }
    fn disableProperty(&self, propName: &str) -> c_int {
        let propName = wxT(propName);
        unsafe { wxPropertyGrid_DisableProperty(self.ptr(), propName.ptr()) }
    }
}

pub struct PropertyGridEvent { ptr: *mut c_void }
impl TPropertyGridEvent for PropertyGridEvent {}
impl TNotifyEvent for PropertyGridEvent {}
impl TCommandEvent for PropertyGridEvent {}
impl TEvent for PropertyGridEvent {}
impl TObject for PropertyGridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PropertyGridEvent {
    pub fn from(ptr: *mut c_void) -> PropertyGridEvent { PropertyGridEvent { ptr: ptr } }
    pub fn null() -> PropertyGridEvent { PropertyGridEvent::from(0 as *mut c_void) }
    
}

pub trait TPropertyGridEvent : TNotifyEvent {
    fn hasProperty(&self) -> c_int {
        unsafe { wxPropertyGridEvent_HasProperty(self.ptr()) }
    }
    fn getProperty(&self) -> PGProperty {
        unsafe { PGProperty { ptr: wxPropertyGridEvent_GetProperty(self.ptr()) } }
    }
}

pub struct PGProperty { ptr: *mut c_void }
impl TPGProperty for PGProperty {}
impl TObject for PGProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PGProperty {
    pub fn from(ptr: *mut c_void) -> PGProperty { PGProperty { ptr: ptr } }
    pub fn null() -> PGProperty { PGProperty::from(0 as *mut c_void) }
    
}

pub trait TPGProperty : TObject {
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

pub struct StringProperty { ptr: *mut c_void }
impl TStringProperty for StringProperty {}
impl TPGProperty for StringProperty {}
impl TObject for StringProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringProperty {
    pub fn from(ptr: *mut c_void) -> StringProperty { StringProperty { ptr: ptr } }
    pub fn null() -> StringProperty { StringProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: &str) -> StringProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { StringProperty { ptr: wxStringProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait TStringProperty : TPGProperty {
}

pub struct IntProperty { ptr: *mut c_void }
impl TIntProperty for IntProperty {}
impl TPGProperty for IntProperty {}
impl TObject for IntProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IntProperty {
    pub fn from(ptr: *mut c_void) -> IntProperty { IntProperty { ptr: ptr } }
    pub fn null() -> IntProperty { IntProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_int) -> IntProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { IntProperty { ptr: wxIntProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait TIntProperty : TPGProperty {
}

pub struct BoolProperty { ptr: *mut c_void }
impl TBoolProperty for BoolProperty {}
impl TPGProperty for BoolProperty {}
impl TObject for BoolProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BoolProperty {
    pub fn from(ptr: *mut c_void) -> BoolProperty { BoolProperty { ptr: ptr } }
    pub fn null() -> BoolProperty { BoolProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_int) -> BoolProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { BoolProperty { ptr: wxBoolProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait TBoolProperty : TPGProperty {
}

pub struct FloatProperty { ptr: *mut c_void }
impl TFloatProperty for FloatProperty {}
impl TPGProperty for FloatProperty {}
impl TObject for FloatProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FloatProperty {
    pub fn from(ptr: *mut c_void) -> FloatProperty { FloatProperty { ptr: ptr } }
    pub fn null() -> FloatProperty { FloatProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_float) -> FloatProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { FloatProperty { ptr: wxFloatProperty_Create(label.ptr(), name.ptr(), value) } }
    }
}

pub trait TFloatProperty : TPGProperty {
}

pub struct DateProperty { ptr: *mut c_void }
impl TDateProperty for DateProperty {}
impl TPGProperty for DateProperty {}
impl TObject for DateProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DateProperty {
    pub fn from(ptr: *mut c_void) -> DateProperty { DateProperty { ptr: ptr } }
    pub fn null() -> DateProperty { DateProperty::from(0 as *mut c_void) }
    
    pub fn new<T: TDateTime>(label: &str, name: &str, value: &T) -> DateProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { DateProperty { ptr: wxDateProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait TDateProperty : TPGProperty {
}

pub struct FileProperty { ptr: *mut c_void }
impl TFileProperty for FileProperty {}
impl TPGProperty for FileProperty {}
impl TObject for FileProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileProperty {
    pub fn from(ptr: *mut c_void) -> FileProperty { FileProperty { ptr: ptr } }
    pub fn null() -> FileProperty { FileProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: &str) -> FileProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { FileProperty { ptr: wxFileProperty_Create(label.ptr(), name.ptr(), value.ptr()) } }
    }
}

pub trait TFileProperty : TPGProperty {
}

pub struct PropertyCategory { ptr: *mut c_void }
impl TPropertyCategory for PropertyCategory {}
impl TPGProperty for PropertyCategory {}
impl TObject for PropertyCategory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PropertyCategory {
    pub fn from(ptr: *mut c_void) -> PropertyCategory { PropertyCategory { ptr: ptr } }
    pub fn null() -> PropertyCategory { PropertyCategory::from(0 as *mut c_void) }
    
    pub fn new(label: &str) -> PropertyCategory {
        let label = wxT(label);
        unsafe { PropertyCategory { ptr: wxPropertyCategory_Create(label.ptr()) } }
    }
}

pub trait TPropertyCategory : TPGProperty {
}

