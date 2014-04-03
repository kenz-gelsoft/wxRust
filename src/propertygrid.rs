use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

/// Wraps the wxWidgets' [wxPropertyGrid](http://docs.wxwidgets.org/3.0/classwx_property_grid.html) class.
pub struct PropertyGrid { ptr: *mut c_void }
impl PropertyGridMethods for PropertyGrid {}
impl ControlMethods for PropertyGrid {}
impl WindowMethods for PropertyGrid {}
impl EvtHandlerMethods for PropertyGrid {}
impl ObjectMethods for PropertyGrid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PropertyGrid {
    pub fn from(ptr: *mut c_void) -> PropertyGrid { PropertyGrid { ptr: ptr } }
    pub fn null() -> PropertyGrid { PropertyGrid::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> PropertyGrid {
        unsafe { PropertyGrid::from(wxPropertyGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxPropertyGrid](http://docs.wxwidgets.org/3.0/classwx_property_grid.html) class.
pub trait PropertyGridMethods : ControlMethods {
    fn append<T: PGPropertyMethods>(&self, prop: &T) -> PGProperty {
        unsafe { PGProperty::from(wxPropertyGrid_Append(self.ptr(), prop.ptr())) }
    }
    fn disableProperty(&self, propName: &str) -> c_int {
        let propName = strToString(propName);
        unsafe { wxPropertyGrid_DisableProperty(self.ptr(), propName.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPropertyGridEvent](http://docs.wxwidgets.org/3.0/classwx_property_grid_event.html) class.
pub struct PropertyGridEvent { ptr: *mut c_void }
impl PropertyGridEventMethods for PropertyGridEvent {}
impl NotifyEventMethods for PropertyGridEvent {}
impl CommandEventMethods for PropertyGridEvent {}
impl EventMethods for PropertyGridEvent {}
impl ObjectMethods for PropertyGridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PropertyGridEvent {
    pub fn from(ptr: *mut c_void) -> PropertyGridEvent { PropertyGridEvent { ptr: ptr } }
    pub fn null() -> PropertyGridEvent { PropertyGridEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPropertyGridEvent](http://docs.wxwidgets.org/3.0/classwx_property_grid_event.html) class.
pub trait PropertyGridEventMethods : NotifyEventMethods {
    fn hasProperty(&self) -> c_int {
        unsafe { wxPropertyGridEvent_HasProperty(self.ptr()) }
    }
    fn getProperty(&self) -> PGProperty {
        unsafe { PGProperty::from(wxPropertyGridEvent_GetProperty(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxPGProperty](http://docs.wxwidgets.org/3.0/classwx_pgp_roperty.html) class.
pub struct PGProperty { ptr: *mut c_void }
impl PGPropertyMethods for PGProperty {}
impl ObjectMethods for PGProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PGProperty {
    pub fn from(ptr: *mut c_void) -> PGProperty { PGProperty { ptr: ptr } }
    pub fn null() -> PGProperty { PGProperty::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPGProperty](http://docs.wxwidgets.org/3.0/classwx_pgp_roperty.html) class.
pub trait PGPropertyMethods : ObjectMethods {
    fn getLabel(&self) -> ~str {
        unsafe { String::from(wxPGProperty_GetLabel(self.ptr())).to_str() }
    }
    fn getName(&self) -> ~str {
        unsafe { String::from(wxPGProperty_GetName(self.ptr())).to_str() }
    }
    fn getValueAsString(&self) -> ~str {
        unsafe { String::from(wxPGProperty_GetValueAsString(self.ptr())).to_str() }
    }
    fn getValueType(&self) -> ~str {
        unsafe { String::from(wxPGProperty_GetValueType(self.ptr())).to_str() }
    }
    fn setHelpString(&self, helpString: &str) {
        let helpString = strToString(helpString);
        unsafe { wxPGProperty_SetHelpString(self.ptr(), helpString.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxStringProperty](http://docs.wxwidgets.org/3.0/classwx_string_property.html) class.
pub struct StringProperty { ptr: *mut c_void }
impl StringPropertyMethods for StringProperty {}
impl PGPropertyMethods for StringProperty {}
impl ObjectMethods for StringProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringProperty {
    pub fn from(ptr: *mut c_void) -> StringProperty { StringProperty { ptr: ptr } }
    pub fn null() -> StringProperty { StringProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: &str) -> StringProperty {
        let label = strToString(label);
        let name = strToString(name);
        let value = strToString(value);
        unsafe { StringProperty::from(wxStringProperty_Create(label.ptr(), name.ptr(), value.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxStringProperty](http://docs.wxwidgets.org/3.0/classwx_string_property.html) class.
pub trait StringPropertyMethods : PGPropertyMethods {
}

/// Wraps the wxWidgets' [wxIntProperty](http://docs.wxwidgets.org/3.0/classwx_int_property.html) class.
pub struct IntProperty { ptr: *mut c_void }
impl IntPropertyMethods for IntProperty {}
impl PGPropertyMethods for IntProperty {}
impl ObjectMethods for IntProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IntProperty {
    pub fn from(ptr: *mut c_void) -> IntProperty { IntProperty { ptr: ptr } }
    pub fn null() -> IntProperty { IntProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_int) -> IntProperty {
        let label = strToString(label);
        let name = strToString(name);
        unsafe { IntProperty::from(wxIntProperty_Create(label.ptr(), name.ptr(), value)) }
    }
}

/// Methods of the wxWidgets' [wxIntProperty](http://docs.wxwidgets.org/3.0/classwx_int_property.html) class.
pub trait IntPropertyMethods : PGPropertyMethods {
}

/// Wraps the wxWidgets' [wxBoolProperty](http://docs.wxwidgets.org/3.0/classwx_bool_property.html) class.
pub struct BoolProperty { ptr: *mut c_void }
impl BoolPropertyMethods for BoolProperty {}
impl PGPropertyMethods for BoolProperty {}
impl ObjectMethods for BoolProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BoolProperty {
    pub fn from(ptr: *mut c_void) -> BoolProperty { BoolProperty { ptr: ptr } }
    pub fn null() -> BoolProperty { BoolProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_int) -> BoolProperty {
        let label = strToString(label);
        let name = strToString(name);
        unsafe { BoolProperty::from(wxBoolProperty_Create(label.ptr(), name.ptr(), value)) }
    }
}

/// Methods of the wxWidgets' [wxBoolProperty](http://docs.wxwidgets.org/3.0/classwx_bool_property.html) class.
pub trait BoolPropertyMethods : PGPropertyMethods {
}

/// Wraps the wxWidgets' [wxFloatProperty](http://docs.wxwidgets.org/3.0/classwx_float_property.html) class.
pub struct FloatProperty { ptr: *mut c_void }
impl FloatPropertyMethods for FloatProperty {}
impl PGPropertyMethods for FloatProperty {}
impl ObjectMethods for FloatProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FloatProperty {
    pub fn from(ptr: *mut c_void) -> FloatProperty { FloatProperty { ptr: ptr } }
    pub fn null() -> FloatProperty { FloatProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: c_float) -> FloatProperty {
        let label = strToString(label);
        let name = strToString(name);
        unsafe { FloatProperty::from(wxFloatProperty_Create(label.ptr(), name.ptr(), value)) }
    }
}

/// Methods of the wxWidgets' [wxFloatProperty](http://docs.wxwidgets.org/3.0/classwx_float_property.html) class.
pub trait FloatPropertyMethods : PGPropertyMethods {
}

/// Wraps the wxWidgets' [wxDateProperty](http://docs.wxwidgets.org/3.0/classwx_date_property.html) class.
pub struct DateProperty { ptr: *mut c_void }
impl DatePropertyMethods for DateProperty {}
impl PGPropertyMethods for DateProperty {}
impl ObjectMethods for DateProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DateProperty {
    pub fn from(ptr: *mut c_void) -> DateProperty { DateProperty { ptr: ptr } }
    pub fn null() -> DateProperty { DateProperty::from(0 as *mut c_void) }
    
    pub fn new<T: DateTimeMethods>(label: &str, name: &str, value: &T) -> DateProperty {
        let label = strToString(label);
        let name = strToString(name);
        unsafe { DateProperty::from(wxDateProperty_Create(label.ptr(), name.ptr(), value.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxDateProperty](http://docs.wxwidgets.org/3.0/classwx_date_property.html) class.
pub trait DatePropertyMethods : PGPropertyMethods {
}

/// Wraps the wxWidgets' [wxFileProperty](http://docs.wxwidgets.org/3.0/classwx_file_property.html) class.
pub struct FileProperty { ptr: *mut c_void }
impl FilePropertyMethods for FileProperty {}
impl PGPropertyMethods for FileProperty {}
impl ObjectMethods for FileProperty { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileProperty {
    pub fn from(ptr: *mut c_void) -> FileProperty { FileProperty { ptr: ptr } }
    pub fn null() -> FileProperty { FileProperty::from(0 as *mut c_void) }
    
    pub fn new(label: &str, name: &str, value: &str) -> FileProperty {
        let label = strToString(label);
        let name = strToString(name);
        let value = strToString(value);
        unsafe { FileProperty::from(wxFileProperty_Create(label.ptr(), name.ptr(), value.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxFileProperty](http://docs.wxwidgets.org/3.0/classwx_file_property.html) class.
pub trait FilePropertyMethods : PGPropertyMethods {
}

/// Wraps the wxWidgets' [wxPropertyCategory](http://docs.wxwidgets.org/3.0/classwx_property_category.html) class.
pub struct PropertyCategory { ptr: *mut c_void }
impl PropertyCategoryMethods for PropertyCategory {}
impl PGPropertyMethods for PropertyCategory {}
impl ObjectMethods for PropertyCategory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PropertyCategory {
    pub fn from(ptr: *mut c_void) -> PropertyCategory { PropertyCategory { ptr: ptr } }
    pub fn null() -> PropertyCategory { PropertyCategory::from(0 as *mut c_void) }
    
    pub fn new(label: &str) -> PropertyCategory {
        let label = strToString(label);
        unsafe { PropertyCategory::from(wxPropertyCategory_Create(label.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPropertyCategory](http://docs.wxwidgets.org/3.0/classwx_property_category.html) class.
pub trait PropertyCategoryMethods : PGPropertyMethods {
}

