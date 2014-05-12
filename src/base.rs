use std::libc::*;
use std::str;
use _unsafe::*;

#[link(name="wxc")]
extern {
    fn wxString_CreateUTF8(buffer: *mut c_void) -> *mut c_void;
    fn wxString_GetUtf8(wxs: *mut c_void) -> *mut c_void;
    fn wxCharBuffer_DataUtf8(wxcb: *mut c_void) -> *c_char;
    fn wxCharBuffer_Delete(wxcb: *mut c_void);
}

pub fn strToString(s: &str) -> String {
    unsafe {
        s.to_c_str().with_ref(|c_str| {
            String::from(wxString_CreateUTF8(c_str as *mut c_void))
        })
    }
}

pub struct String { ptr: *mut c_void }
impl Drop for String {
    fn drop(&mut self) {
        unsafe { wxString_Delete(self.ptr); }
    }
}
impl String {
    pub fn ptr(&self) -> *mut c_void { self.ptr }
    pub fn from(ptr: *mut c_void) -> String { String { ptr: ptr } }
    pub fn to_str(&self) -> ~str {
        unsafe {
            let charBuffer = wxString_GetUtf8(self.ptr);
            let utf8 = wxCharBuffer_DataUtf8(charBuffer);
            wxCharBuffer_Delete(charBuffer);
            str::raw::from_c_str(utf8)
        }
    }
}

/// The wxRust-specific derived class of [wxClient](http://docs.wxwidgets.org/3.0/classwx_client.html).
pub struct RustClient { ptr: *mut c_void }
impl RustClientMethods for RustClient {}
impl ClientMethods for RustClient {}
impl ClientBaseMethods for RustClient {}
impl ObjectMethods for RustClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustClient {
    pub fn from(ptr: *mut c_void) -> RustClient { RustClient { ptr: ptr } }
    pub fn null() -> RustClient { RustClient::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxClient](http://docs.wxwidgets.org/3.0/classwx_client.html).
pub trait RustClientMethods : ClientMethods {
}

/// The wxRust-specific derived class of [wxConnection](http://docs.wxwidgets.org/3.0/classwx_connection.html).
pub struct RustConnection { ptr: *mut c_void }
impl RustConnectionMethods for RustConnection {}
impl ConnectionMethods for RustConnection {}
impl ConnectionBaseMethods for RustConnection {}
impl ObjectMethods for RustConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustConnection {
    pub fn from(ptr: *mut c_void) -> RustConnection { RustConnection { ptr: ptr } }
    pub fn null() -> RustConnection { RustConnection::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxConnection](http://docs.wxwidgets.org/3.0/classwx_connection.html).
pub trait RustConnectionMethods : ConnectionMethods {
}

/// The wxRust-specific derived class of [wxLocale](http://docs.wxwidgets.org/3.0/classwx_locale.html).
pub struct RustLocale { ptr: *mut c_void }
impl RustLocaleMethods for RustLocale {}
impl LocaleMethods for RustLocale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustLocale {
    pub fn from(ptr: *mut c_void) -> RustLocale { RustLocale { ptr: ptr } }
    pub fn null() -> RustLocale { RustLocale::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxLocale](http://docs.wxwidgets.org/3.0/classwx_locale.html).
pub trait RustLocaleMethods : LocaleMethods {
}

/// The wxRust-specific derived class of [wxServer](http://docs.wxwidgets.org/3.0/classwx_server.html).
pub struct RustServer { ptr: *mut c_void }
impl RustServerMethods for RustServer {}
impl ServerMethods for RustServer {}
impl ServerBaseMethods for RustServer {}
impl ObjectMethods for RustServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustServer {
    pub fn from(ptr: *mut c_void) -> RustServer { RustServer { ptr: ptr } }
    pub fn null() -> RustServer { RustServer::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxServer](http://docs.wxwidgets.org/3.0/classwx_server.html).
pub trait RustServerMethods : ServerMethods {
}

/// Wraps the wxWidgets' [wxArray](http://docs.wxwidgets.org/3.0/classwx_array.html) class.
pub struct Array { ptr: *mut c_void }
impl ArrayMethods for Array { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Array {
    pub fn from(ptr: *mut c_void) -> Array { Array { ptr: ptr } }
    pub fn null() -> Array { Array::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxArray](http://docs.wxwidgets.org/3.0/classwx_array.html) class.
pub trait ArrayMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxArrayString](http://docs.wxwidgets.org/3.0/classwx_array_string.html) class.
pub struct ArrayString { ptr: *mut c_void }
impl ArrayStringMethods for ArrayString {}
impl ArrayMethods for ArrayString { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ArrayString {
    pub fn from(ptr: *mut c_void) -> ArrayString { ArrayString { ptr: ptr } }
    pub fn null() -> ArrayString { ArrayString::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxArrayString](http://docs.wxwidgets.org/3.0/classwx_array_string.html) class.
pub trait ArrayStringMethods : ArrayMethods {
}

/// Wraps the wxWidgets' [wxBufferedInputStream](http://docs.wxwidgets.org/3.0/classwx_buffered_input_stream.html) class.
pub struct BufferedInputStream { ptr: *mut c_void }
impl BufferedInputStreamMethods for BufferedInputStream {}
impl FilterInputStreamMethods for BufferedInputStream {}
impl InputStreamMethods for BufferedInputStream {}
impl StreamBaseMethods for BufferedInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedInputStream {
    pub fn from(ptr: *mut c_void) -> BufferedInputStream { BufferedInputStream { ptr: ptr } }
    pub fn null() -> BufferedInputStream { BufferedInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxBufferedInputStream](http://docs.wxwidgets.org/3.0/classwx_buffered_input_stream.html) class.
pub trait BufferedInputStreamMethods : FilterInputStreamMethods {
}

/// Wraps the wxWidgets' [wxBufferedOutputStream](http://docs.wxwidgets.org/3.0/classwx_buffered_output_stream.html) class.
pub struct BufferedOutputStream { ptr: *mut c_void }
impl BufferedOutputStreamMethods for BufferedOutputStream {}
impl FilterOutputStreamMethods for BufferedOutputStream {}
impl OutputStreamMethods for BufferedOutputStream {}
impl StreamBaseMethods for BufferedOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedOutputStream {
    pub fn from(ptr: *mut c_void) -> BufferedOutputStream { BufferedOutputStream { ptr: ptr } }
    pub fn null() -> BufferedOutputStream { BufferedOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxBufferedOutputStream](http://docs.wxwidgets.org/3.0/classwx_buffered_output_stream.html) class.
pub trait BufferedOutputStreamMethods : FilterOutputStreamMethods {
}

/// Wraps the wxWidgets' [wxCSConv](http://docs.wxwidgets.org/3.0/classwx_csc_onv.html) class.
pub struct CSConv { ptr: *mut c_void }
impl CSConvMethods for CSConv {}
impl MBConvMethods for CSConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CSConv {
    pub fn from(ptr: *mut c_void) -> CSConv { CSConv { ptr: ptr } }
    pub fn null() -> CSConv { CSConv::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCSConv](http://docs.wxwidgets.org/3.0/classwx_csc_onv.html) class.
pub trait CSConvMethods : MBConvMethods {
}

/// Wraps the wxWidgets' [wxClassInfo](http://docs.wxwidgets.org/3.0/classwx_class_info.html) class.
pub struct ClassInfo { ptr: *mut c_void }
impl ClassInfoMethods for ClassInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClassInfo {
    pub fn from(ptr: *mut c_void) -> ClassInfo { ClassInfo { ptr: ptr } }
    pub fn null() -> ClassInfo { ClassInfo::from(0 as *mut c_void) }
    
    pub fn findClass(_txt: &str) -> ClassInfo {
        let _txt = strToString(_txt);
        unsafe { ClassInfo::from(wxClassInfo_FindClass(_txt.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxClassInfo](http://docs.wxwidgets.org/3.0/classwx_class_info.html) class.
pub trait ClassInfoMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn newClassByName(&self) -> *mut c_void {
        unsafe { wxClassInfo_CreateClassByName(self.ptr()) }
    }
    fn getClassName(&self) -> *mut c_void {
        unsafe { wxClassInfo_GetClassName(self.ptr()) }
    }
    fn isKindOf(&self, _name: &str) -> c_int {
        let _name = strToString(_name);
        unsafe { wxClassInfo_IsKindOf(self.ptr(), _name.ptr()) }
    }
    fn getBaseClassName1(&self) -> ~str {
        unsafe { String::from(wxClassInfo_GetBaseClassName1(self.ptr())).to_str() }
    }
    fn getBaseClassName2(&self) -> ~str {
        unsafe { String::from(wxClassInfo_GetBaseClassName2(self.ptr())).to_str() }
    }
    fn getClassNameEx(&self) -> ~str {
        unsafe { String::from(wxClassInfo_GetClassNameEx(self.ptr())).to_str() }
    }
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.ptr()) }
    }
    fn isKindOfEx<T: ClassInfoMethods>(&self, classInfo: &T) -> c_int {
        unsafe { wxClassInfo_IsKindOfEx(self.ptr(), classInfo.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxClient](http://docs.wxwidgets.org/3.0/classwx_client.html) class.
/// Rather use the wxRust-specific [RustClient](struct.RustClient.html) class.
pub struct Client { ptr: *mut c_void }
impl ClientMethods for Client {}
impl ClientBaseMethods for Client {}
impl ObjectMethods for Client { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Client {
    pub fn from(ptr: *mut c_void) -> Client { Client { ptr: ptr } }
    pub fn null() -> Client { Client::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxClient](http://docs.wxwidgets.org/3.0/classwx_client.html) class.
pub trait ClientMethods : ClientBaseMethods {
}

/// Wraps the wxWidgets' [wxClientBase](http://docs.wxwidgets.org/3.0/classwx_client_base.html) class.
pub struct ClientBase { ptr: *mut c_void }
impl ClientBaseMethods for ClientBase {}
impl ObjectMethods for ClientBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientBase {
    pub fn from(ptr: *mut c_void) -> ClientBase { ClientBase { ptr: ptr } }
    pub fn null() -> ClientBase { ClientBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxClientBase](http://docs.wxwidgets.org/3.0/classwx_client_base.html) class.
pub trait ClientBaseMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxClientData](http://docs.wxwidgets.org/3.0/classwx_client_data.html) class.
pub struct ClientData { ptr: *mut c_void }
impl ClientDataMethods for ClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientData {
    pub fn from(ptr: *mut c_void) -> ClientData { ClientData { ptr: ptr } }
    pub fn null() -> ClientData { ClientData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxClientData](http://docs.wxwidgets.org/3.0/classwx_client_data.html) class.
pub trait ClientDataMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxClientDataContainer](http://docs.wxwidgets.org/3.0/classwx_client_data_container.html) class.
pub struct ClientDataContainer { ptr: *mut c_void }
impl ClientDataContainerMethods for ClientDataContainer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientDataContainer {
    pub fn from(ptr: *mut c_void) -> ClientDataContainer { ClientDataContainer { ptr: ptr } }
    pub fn null() -> ClientDataContainer { ClientDataContainer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxClientDataContainer](http://docs.wxwidgets.org/3.0/classwx_client_data_container.html) class.
pub trait ClientDataContainerMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxClosure](http://docs.wxwidgets.org/3.0/classwx_closure.html) class.
pub struct Closure { ptr: *mut c_void }
impl ClosureMethods for Closure {}
impl ObjectMethods for Closure { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Closure {
    pub fn from(ptr: *mut c_void) -> Closure { Closure { ptr: ptr } }
    pub fn null() -> Closure { Closure::from(0 as *mut c_void) }
    
    pub fn new(_fun_CEvent: *mut c_void, _data: *mut c_void) -> Closure {
        unsafe { Closure::from(wxClosure_Create(_fun_CEvent, _data)) }
    }
}

/// Methods of the wxWidgets' [wxClosure](http://docs.wxwidgets.org/3.0/classwx_closure.html) class.
pub trait ClosureMethods : ObjectMethods {
    fn getData(&self) -> *mut c_void {
        unsafe { wxClosure_GetData(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxCommandLineParser](http://docs.wxwidgets.org/3.0/classwx_command_line_parser.html) class.
pub struct CommandLineParser { ptr: *mut c_void }
impl CommandLineParserMethods for CommandLineParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CommandLineParser {
    pub fn from(ptr: *mut c_void) -> CommandLineParser { CommandLineParser { ptr: ptr } }
    pub fn null() -> CommandLineParser { CommandLineParser::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCommandLineParser](http://docs.wxwidgets.org/3.0/classwx_command_line_parser.html) class.
pub trait CommandLineParserMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxCondition](http://docs.wxwidgets.org/3.0/classwx_condition.html) class.
pub struct Condition { ptr: *mut c_void }
impl ConditionMethods for Condition { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Condition {
    pub fn from(ptr: *mut c_void) -> Condition { Condition { ptr: ptr } }
    pub fn null() -> Condition { Condition::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCondition](http://docs.wxwidgets.org/3.0/classwx_condition.html) class.
pub trait ConditionMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxConfigBase](http://docs.wxwidgets.org/3.0/classwx_config_base.html) class.
pub struct ConfigBase { ptr: *mut c_void }
impl ConfigBaseMethods for ConfigBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ConfigBase {
    pub fn from(ptr: *mut c_void) -> ConfigBase { ConfigBase { ptr: ptr } }
    pub fn null() -> ConfigBase { ConfigBase::from(0 as *mut c_void) }
    
    pub fn new() -> ConfigBase {
        unsafe { ConfigBase::from(wxConfigBase_Create()) }
    }
    pub fn get() -> ConfigBase {
        unsafe { ConfigBase::from(wxConfigBase_Get()) }
    }
    pub fn set<T: ConfigBaseMethods>(self_: &T) {
        unsafe { wxConfigBase_Set(self_.ptr()) }
    }
}

/// Methods of the wxWidgets' [wxConfigBase](http://docs.wxwidgets.org/3.0/classwx_config_base.html) class.
pub trait ConfigBaseMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self.ptr()) }
    }
    fn deleteAll(&self) -> c_int {
        unsafe { wxConfigBase_DeleteAll(self.ptr()) }
    }
    fn deleteEntry(&self, key: &str, bDeleteGroupIfEmpty: c_int) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_DeleteEntry(self.ptr(), key.ptr(), bDeleteGroupIfEmpty) }
    }
    fn deleteGroup(&self, key: &str) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_DeleteGroup(self.ptr(), key.ptr()) }
    }
    fn exists(&self, strName: &str) -> c_int {
        let strName = strToString(strName);
        unsafe { wxConfigBase_Exists(self.ptr(), strName.ptr()) }
    }
    fn expandEnvVars(&self, str: &str) -> ~str {
        let str = strToString(str);
        unsafe { String::from(wxConfigBase_ExpandEnvVars(self.ptr(), str.ptr())).to_str() }
    }
    fn flush(&self, bCurrentOnly: c_int) -> c_int {
        unsafe { wxConfigBase_Flush(self.ptr(), bCurrentOnly) }
    }
    fn getAppName(&self) -> ~str {
        unsafe { String::from(wxConfigBase_GetAppName(self.ptr())).to_str() }
    }
    fn getEntryType(&self, name: &str) -> c_int {
        let name = strToString(name);
        unsafe { wxConfigBase_GetEntryType(self.ptr(), name.ptr()) }
    }
    fn getFirstEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { String::from(wxConfigBase_GetFirstEntry(self.ptr(), lIndex)).to_str() }
    }
    fn getFirstGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { String::from(wxConfigBase_GetFirstGroup(self.ptr(), lIndex)).to_str() }
    }
    fn getNextEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { String::from(wxConfigBase_GetNextEntry(self.ptr(), lIndex)).to_str() }
    }
    fn getNextGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { String::from(wxConfigBase_GetNextGroup(self.ptr(), lIndex)).to_str() }
    }
    fn getNumberOfEntries(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfEntries(self.ptr(), bRecursive) }
    }
    fn getNumberOfGroups(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfGroups(self.ptr(), bRecursive) }
    }
    fn getPath(&self) -> ~str {
        unsafe { String::from(wxConfigBase_GetPath(self.ptr())).to_str() }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.ptr()) }
    }
    fn getVendorName(&self) -> ~str {
        unsafe { String::from(wxConfigBase_GetVendorName(self.ptr())).to_str() }
    }
    fn hasEntry(&self, strName: &str) -> c_int {
        let strName = strToString(strName);
        unsafe { wxConfigBase_HasEntry(self.ptr(), strName.ptr()) }
    }
    fn hasGroup(&self, strName: &str) -> c_int {
        let strName = strToString(strName);
        unsafe { wxConfigBase_HasGroup(self.ptr(), strName.ptr()) }
    }
    fn isExpandingEnvVars(&self) -> c_int {
        unsafe { wxConfigBase_IsExpandingEnvVars(self.ptr()) }
    }
    fn isRecordingDefaults(&self) -> c_int {
        unsafe { wxConfigBase_IsRecordingDefaults(self.ptr()) }
    }
    fn readBool(&self, key: &str, defVal: c_int) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_ReadBool(self.ptr(), key.ptr(), defVal) }
    }
    fn readDouble(&self, key: &str, defVal: c_double) -> c_double {
        let key = strToString(key);
        unsafe { wxConfigBase_ReadDouble(self.ptr(), key.ptr(), defVal) }
    }
    fn readInteger(&self, key: &str, defVal: c_int) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_ReadInteger(self.ptr(), key.ptr(), defVal) }
    }
    fn readString(&self, key: &str, defVal: &str) -> ~str {
        let key = strToString(key);
        let defVal = strToString(defVal);
        unsafe { String::from(wxConfigBase_ReadString(self.ptr(), key.ptr(), defVal.ptr())).to_str() }
    }
    fn renameEntry(&self, oldName: &str, newName: &str) -> c_int {
        let oldName = strToString(oldName);
        let newName = strToString(newName);
        unsafe { wxConfigBase_RenameEntry(self.ptr(), oldName.ptr(), newName.ptr()) }
    }
    fn renameGroup(&self, oldName: &str, newName: &str) -> c_int {
        let oldName = strToString(oldName);
        let newName = strToString(newName);
        unsafe { wxConfigBase_RenameGroup(self.ptr(), oldName.ptr(), newName.ptr()) }
    }
    fn setAppName(&self, appName: &str) {
        let appName = strToString(appName);
        unsafe { wxConfigBase_SetAppName(self.ptr(), appName.ptr()) }
    }
    fn setExpandEnvVars(&self, bDoIt: c_int) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.ptr(), bDoIt) }
    }
    fn setPath(&self, strPath: &str) {
        let strPath = strToString(strPath);
        unsafe { wxConfigBase_SetPath(self.ptr(), strPath.ptr()) }
    }
    fn setRecordDefaults(&self, bDoIt: c_int) {
        unsafe { wxConfigBase_SetRecordDefaults(self.ptr(), bDoIt) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxConfigBase_SetStyle(self.ptr(), style) }
    }
    fn setVendorName(&self, vendorName: &str) {
        let vendorName = strToString(vendorName);
        unsafe { wxConfigBase_SetVendorName(self.ptr(), vendorName.ptr()) }
    }
    fn writeBool(&self, key: &str, value: c_int) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_WriteBool(self.ptr(), key.ptr(), value) }
    }
    fn writeDouble(&self, key: &str, value: c_double) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_WriteDouble(self.ptr(), key.ptr(), value) }
    }
    fn writeInteger(&self, key: &str, value: c_int) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_WriteInteger(self.ptr(), key.ptr(), value) }
    }
    fn writeLong(&self, key: &str, value: c_long) -> c_int {
        let key = strToString(key);
        unsafe { wxConfigBase_WriteLong(self.ptr(), key.ptr(), value) }
    }
    fn writeString(&self, key: &str, value: &str) -> c_int {
        let key = strToString(key);
        let value = strToString(value);
        unsafe { wxConfigBase_WriteString(self.ptr(), key.ptr(), value.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxConnection](http://docs.wxwidgets.org/3.0/classwx_connection.html) class.
/// Rather use the wxRust-specific [RustConnection](struct.RustConnection.html) class.
pub struct Connection { ptr: *mut c_void }
impl ConnectionMethods for Connection {}
impl ConnectionBaseMethods for Connection {}
impl ObjectMethods for Connection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Connection {
    pub fn from(ptr: *mut c_void) -> Connection { Connection { ptr: ptr } }
    pub fn null() -> Connection { Connection::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxConnection](http://docs.wxwidgets.org/3.0/classwx_connection.html) class.
pub trait ConnectionMethods : ConnectionBaseMethods {
}

/// Wraps the wxWidgets' [wxConnectionBase](http://docs.wxwidgets.org/3.0/classwx_connection_base.html) class.
pub struct ConnectionBase { ptr: *mut c_void }
impl ConnectionBaseMethods for ConnectionBase {}
impl ObjectMethods for ConnectionBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ConnectionBase {
    pub fn from(ptr: *mut c_void) -> ConnectionBase { ConnectionBase { ptr: ptr } }
    pub fn null() -> ConnectionBase { ConnectionBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxConnectionBase](http://docs.wxwidgets.org/3.0/classwx_connection_base.html) class.
pub trait ConnectionBaseMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxCountingOutputStream](http://docs.wxwidgets.org/3.0/classwx_counting_output_stream.html) class.
pub struct CountingOutputStream { ptr: *mut c_void }
impl CountingOutputStreamMethods for CountingOutputStream {}
impl OutputStreamMethods for CountingOutputStream {}
impl StreamBaseMethods for CountingOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CountingOutputStream {
    pub fn from(ptr: *mut c_void) -> CountingOutputStream { CountingOutputStream { ptr: ptr } }
    pub fn null() -> CountingOutputStream { CountingOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCountingOutputStream](http://docs.wxwidgets.org/3.0/classwx_counting_output_stream.html) class.
pub trait CountingOutputStreamMethods : OutputStreamMethods {
}

/// Wraps the wxWidgets' [wxCriticalSection](http://docs.wxwidgets.org/3.0/classwx_critical_section.html) class.
pub struct CriticalSection { ptr: *mut c_void }
impl CriticalSectionMethods for CriticalSection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CriticalSection {
    pub fn from(ptr: *mut c_void) -> CriticalSection { CriticalSection { ptr: ptr } }
    pub fn null() -> CriticalSection { CriticalSection::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCriticalSection](http://docs.wxwidgets.org/3.0/classwx_critical_section.html) class.
pub trait CriticalSectionMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxCriticalSectionLocker](http://docs.wxwidgets.org/3.0/classwx_critical_section_locker.html) class.
pub struct CriticalSectionLocker { ptr: *mut c_void }
impl CriticalSectionLockerMethods for CriticalSectionLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CriticalSectionLocker {
    pub fn from(ptr: *mut c_void) -> CriticalSectionLocker { CriticalSectionLocker { ptr: ptr } }
    pub fn null() -> CriticalSectionLocker { CriticalSectionLocker::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCriticalSectionLocker](http://docs.wxwidgets.org/3.0/classwx_critical_section_locker.html) class.
pub trait CriticalSectionLockerMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDDEClient](http://docs.wxwidgets.org/3.0/classwx_ddec_lient.html) class.
pub struct DDEClient { ptr: *mut c_void }
impl DDEClientMethods for DDEClient {}
impl ClientBaseMethods for DDEClient {}
impl ObjectMethods for DDEClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DDEClient {
    pub fn from(ptr: *mut c_void) -> DDEClient { DDEClient { ptr: ptr } }
    pub fn null() -> DDEClient { DDEClient::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDDEClient](http://docs.wxwidgets.org/3.0/classwx_ddec_lient.html) class.
pub trait DDEClientMethods : ClientBaseMethods {
}

/// Wraps the wxWidgets' [wxDDEConnection](http://docs.wxwidgets.org/3.0/classwx_ddec_onnection.html) class.
pub struct DDEConnection { ptr: *mut c_void }
impl DDEConnectionMethods for DDEConnection {}
impl ConnectionBaseMethods for DDEConnection {}
impl ObjectMethods for DDEConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DDEConnection {
    pub fn from(ptr: *mut c_void) -> DDEConnection { DDEConnection { ptr: ptr } }
    pub fn null() -> DDEConnection { DDEConnection::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDDEConnection](http://docs.wxwidgets.org/3.0/classwx_ddec_onnection.html) class.
pub trait DDEConnectionMethods : ConnectionBaseMethods {
}

/// Wraps the wxWidgets' [wxDDEServer](http://docs.wxwidgets.org/3.0/classwx_ddes_erver.html) class.
pub struct DDEServer { ptr: *mut c_void }
impl DDEServerMethods for DDEServer {}
impl ServerBaseMethods for DDEServer {}
impl ObjectMethods for DDEServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DDEServer {
    pub fn from(ptr: *mut c_void) -> DDEServer { DDEServer { ptr: ptr } }
    pub fn null() -> DDEServer { DDEServer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDDEServer](http://docs.wxwidgets.org/3.0/classwx_ddes_erver.html) class.
pub trait DDEServerMethods : ServerBaseMethods {
}

/// Wraps the wxWidgets' [wxDataInputStream](http://docs.wxwidgets.org/3.0/classwx_data_input_stream.html) class.
pub struct DataInputStream { ptr: *mut c_void }
impl DataInputStreamMethods for DataInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataInputStream {
    pub fn from(ptr: *mut c_void) -> DataInputStream { DataInputStream { ptr: ptr } }
    pub fn null() -> DataInputStream { DataInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDataInputStream](http://docs.wxwidgets.org/3.0/classwx_data_input_stream.html) class.
pub trait DataInputStreamMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDataOutputStream](http://docs.wxwidgets.org/3.0/classwx_data_output_stream.html) class.
pub struct DataOutputStream { ptr: *mut c_void }
impl DataOutputStreamMethods for DataOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataOutputStream {
    pub fn from(ptr: *mut c_void) -> DataOutputStream { DataOutputStream { ptr: ptr } }
    pub fn null() -> DataOutputStream { DataOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDataOutputStream](http://docs.wxwidgets.org/3.0/classwx_data_output_stream.html) class.
pub trait DataOutputStreamMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDateTime](http://docs.wxwidgets.org/3.0/classwx_date_time.html) class.
pub struct DateTime { ptr: *mut c_void }
impl DateTimeMethods for DateTime { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DateTime {
    pub fn from(ptr: *mut c_void) -> DateTime { DateTime { ptr: ptr } }
    pub fn null() -> DateTime { DateTime::from(0 as *mut c_void) }
    
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    pub fn new() -> DateTime {
        unsafe { DateTime::from(wxDateTime_Create()) }
    }
    pub fn getAmString() -> ~str {
        unsafe { String::from(wxDateTime_GetAmString()).to_str() }
    }
    pub fn getBeginDST<T: DateTimeMethods>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetBeginDST(year, country, dt.ptr()) }
    }
    pub fn getCentury(year: c_int) -> c_int {
        unsafe { wxDateTime_GetCentury(year) }
    }
    pub fn getCountry() -> c_int {
        unsafe { wxDateTime_GetCountry() }
    }
    pub fn getCurrentMonth(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentMonth(cal) }
    }
    pub fn getCurrentYear(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentYear(cal) }
    }
    pub fn getEndDST<T: DateTimeMethods>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.ptr()) }
    }
    pub fn getMonthName(month: c_int, flags: c_int) -> ~str {
        unsafe { String::from(wxDateTime_GetMonthName(month, flags)).to_str() }
    }
    pub fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    pub fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    pub fn getPmString() -> ~str {
        unsafe { String::from(wxDateTime_GetPmString()).to_str() }
    }
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> ~str {
        unsafe { String::from(wxDateTime_GetWeekDayName(weekday, flags)).to_str() }
    }
    pub fn isDSTApplicable(year: c_int, country: c_int) -> c_int {
        unsafe { wxDateTime_IsDSTApplicable(year, country) }
    }
    pub fn isLeapYear(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_IsLeapYear(year, cal) }
    }
    pub fn isWestEuropeanCountry(country: c_int) -> c_int {
        unsafe { wxDateTime_IsWestEuropeanCountry(country) }
    }
    pub fn setCountry(country: c_int) {
        unsafe { wxDateTime_SetCountry(country) }
    }
    pub fn wxDateTime(hi_long: c_int, lo_long: c_int) -> *mut c_void {
        unsafe { wxDateTime_wxDateTime(hi_long, lo_long) }
    }
}

/// Methods of the wxWidgets' [wxDateTime](http://docs.wxwidgets.org/3.0/classwx_date_time.html) class.
pub trait DateTimeMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn addDate<T: DateTimeMethods>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.ptr(), _yrs, _mnt, _wek, _day) }
    }
    fn addTime<T: DateTimeMethods>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddTime(self.ptr(), diff, _ref.ptr()) }
    }
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.ptr(), _hrs, _min, _sec, _mls) }
    }
    fn format(&self, format: *mut c_void, tz: c_int) -> ~str {
        unsafe { String::from(wxDateTime_Format(self.ptr(), format, tz)).to_str() }
    }
    fn formatDate(&self) -> ~str {
        unsafe { String::from(wxDateTime_FormatDate(self.ptr())).to_str() }
    }
    fn formatISODate(&self) -> ~str {
        unsafe { String::from(wxDateTime_FormatISODate(self.ptr())).to_str() }
    }
    fn formatISOTime(&self) -> ~str {
        unsafe { String::from(wxDateTime_FormatISOTime(self.ptr())).to_str() }
    }
    fn formatTime(&self) -> ~str {
        unsafe { String::from(wxDateTime_FormatTime(self.ptr())).to_str() }
    }
    fn getDay(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDay(self.ptr(), tz) }
    }
    fn getDayOfYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDayOfYear(self.ptr(), tz) }
    }
    fn getHour(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetHour(self.ptr(), tz) }
    }
    fn getLastMonthDay<T: DateTimeMethods>(&self, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastMonthDay(self.ptr(), month, year, _ref.ptr()) }
    }
    fn getLastWeekDay<T: DateTimeMethods>(&self, weekday: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastWeekDay(self.ptr(), weekday, month, year, _ref.ptr()) }
    }
    fn getMillisecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMillisecond(self.ptr(), tz) }
    }
    fn getMinute(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMinute(self.ptr(), tz) }
    }
    fn getMonth(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMonth(self.ptr(), tz) }
    }
    fn getNextWeekDay<T: DateTimeMethods>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetNextWeekDay(self.ptr(), weekday, _ref.ptr()) }
    }
    fn getPrevWeekDay<T: DateTimeMethods>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetPrevWeekDay(self.ptr(), weekday, _ref.ptr()) }
    }
    fn getSecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetSecond(self.ptr(), tz) }
    }
    fn getTicks(&self) -> time_t {
        unsafe { wxDateTime_GetTicks(self.ptr()) }
    }
    fn getValue(&self, hi_long: *mut c_void, lo_long: *mut c_void) {
        unsafe { wxDateTime_GetValue(self.ptr(), hi_long, lo_long) }
    }
    fn getWeekDay<T: DateTimeMethods>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDay(self.ptr(), weekday, n, month, year, _ref.ptr()) }
    }
    fn getWeekDayInSameWeek<T: DateTimeMethods>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDayInSameWeek(self.ptr(), weekday, _ref.ptr()) }
    }
    fn getWeekDayTZ(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekDayTZ(self.ptr(), tz) }
    }
    fn getWeekOfMonth(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfMonth(self.ptr(), flags, tz) }
    }
    fn getWeekOfYear(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfYear(self.ptr(), flags, tz) }
    }
    fn getYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetYear(self.ptr(), tz) }
    }
    fn isBetween<T: DateTimeMethods, U: DateTimeMethods>(&self, t1: &T, t2: &U) -> c_int {
        unsafe { wxDateTime_IsBetween(self.ptr(), t1.ptr(), t2.ptr()) }
    }
    fn isDST(&self, country: c_int) -> c_int {
        unsafe { wxDateTime_IsDST(self.ptr(), country) }
    }
    fn isEarlierThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEarlierThan(self.ptr(), datetime) }
    }
    fn isEqualTo(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualTo(self.ptr(), datetime) }
    }
    fn isEqualUpTo<T: DateTimeMethods>(&self, dt: &T, ts: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualUpTo(self.ptr(), dt.ptr(), ts) }
    }
    fn isLaterThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsLaterThan(self.ptr(), datetime) }
    }
    fn isSameDate<T: DateTimeMethods>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameDate(self.ptr(), dt.ptr()) }
    }
    fn isSameTime<T: DateTimeMethods>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameTime(self.ptr(), dt.ptr()) }
    }
    fn isStrictlyBetween<T: DateTimeMethods, U: DateTimeMethods>(&self, t1: &T, t2: &U) -> c_int {
        unsafe { wxDateTime_IsStrictlyBetween(self.ptr(), t1.ptr(), t2.ptr()) }
    }
    fn isValid(&self) -> c_int {
        unsafe { wxDateTime_IsValid(self.ptr()) }
    }
    fn isWorkDay(&self, country: c_int) -> c_int {
        unsafe { wxDateTime_IsWorkDay(self.ptr(), country) }
    }
    fn makeGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_MakeGMT(self.ptr(), noDST) }
    }
    fn makeTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_MakeTimezone(self.ptr(), tz, noDST) }
    }
    fn now(&self) {
        unsafe { wxDateTime_Now(self.ptr()) }
    }
    fn parseDate(&self, date: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseDate(self.ptr(), date) }
    }
    fn parseDateTime(&self, datetime: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseDateTime(self.ptr(), datetime) }
    }
    fn parseFormat(&self, date: *mut c_void, format: *mut c_void, dateDef: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseFormat(self.ptr(), date, format, dateDef) }
    }
    fn parseRfc822Date(&self, date: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseRfc822Date(self.ptr(), date) }
    }
    fn parseTime<T: TimeMethods>(&self, time: &T) -> *mut c_void {
        unsafe { wxDateTime_ParseTime(self.ptr(), time.ptr()) }
    }
    fn resetTime(&self) {
        unsafe { wxDateTime_ResetTime(self.ptr()) }
    }
    fn set(&self, day: c_int, month: c_int, year: c_int, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_Set(self.ptr(), day, month, year, hour, minute, second, millisec) }
    }
    fn setDay(&self, day: c_int) {
        unsafe { wxDateTime_SetDay(self.ptr(), day) }
    }
    fn setHour(&self, hour: c_int) {
        unsafe { wxDateTime_SetHour(self.ptr(), hour) }
    }
    fn setMillisecond(&self, millisecond: c_int) {
        unsafe { wxDateTime_SetMillisecond(self.ptr(), millisecond) }
    }
    fn setMinute(&self, minute: c_int) {
        unsafe { wxDateTime_SetMinute(self.ptr(), minute) }
    }
    fn setMonth(&self, month: c_int) {
        unsafe { wxDateTime_SetMonth(self.ptr(), month) }
    }
    fn setSecond(&self, second: c_int) {
        unsafe { wxDateTime_SetSecond(self.ptr(), second) }
    }
    fn setTime(&self, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_SetTime(self.ptr(), hour, minute, second, millisec) }
    }
    fn setToCurrent(&self) {
        unsafe { wxDateTime_SetToCurrent(self.ptr()) }
    }
    fn setToLastMonthDay(&self, month: c_int, year: c_int) {
        unsafe { wxDateTime_SetToLastMonthDay(self.ptr(), month, year) }
    }
    fn setToLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int) -> c_int {
        unsafe { wxDateTime_SetToLastWeekDay(self.ptr(), weekday, month, year) }
    }
    fn setToNextWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToNextWeekDay(self.ptr(), weekday) }
    }
    fn setToPrevWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToPrevWeekDay(self.ptr(), weekday) }
    }
    fn setToWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int) -> c_int {
        unsafe { wxDateTime_SetToWeekDay(self.ptr(), weekday, n, month, year) }
    }
    fn setToWeekDayInSameWeek(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToWeekDayInSameWeek(self.ptr(), weekday) }
    }
    fn setYear(&self, year: c_int) {
        unsafe { wxDateTime_SetYear(self.ptr(), year) }
    }
    fn subtractDate<T: DateTimeMethods>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn subtractTime<T: DateTimeMethods>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractTime(self.ptr(), diff, _ref.ptr()) }
    }
    fn toGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_ToGMT(self.ptr(), noDST) }
    }
    fn toTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_ToTimezone(self.ptr(), tz, noDST) }
    }
    fn today(&self) {
        unsafe { wxDateTime_Today(self.ptr()) }
    }
    fn uNow(&self) {
        unsafe { wxDateTime_UNow(self.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxDateTime_Delete(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxDebugContext](http://docs.wxwidgets.org/3.0/classwx_debug_context.html) class.
pub struct DebugContext { ptr: *mut c_void }
impl DebugContextMethods for DebugContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DebugContext {
    pub fn from(ptr: *mut c_void) -> DebugContext { DebugContext { ptr: ptr } }
    pub fn null() -> DebugContext { DebugContext::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDebugContext](http://docs.wxwidgets.org/3.0/classwx_debug_context.html) class.
pub trait DebugContextMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDirTraverser](http://docs.wxwidgets.org/3.0/classwx_dir_traverser.html) class.
pub struct DirTraverser { ptr: *mut c_void }
impl DirTraverserMethods for DirTraverser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DirTraverser {
    pub fn from(ptr: *mut c_void) -> DirTraverser { DirTraverser { ptr: ptr } }
    pub fn null() -> DirTraverser { DirTraverser::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDirTraverser](http://docs.wxwidgets.org/3.0/classwx_dir_traverser.html) class.
pub trait DirTraverserMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDllLoader](http://docs.wxwidgets.org/3.0/classwx_dll_loader.html) class.
pub struct DllLoader { ptr: *mut c_void }
impl DllLoaderMethods for DllLoader { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DllLoader {
    pub fn from(ptr: *mut c_void) -> DllLoader { DllLoader { ptr: ptr } }
    pub fn null() -> DllLoader { DllLoader::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDllLoader](http://docs.wxwidgets.org/3.0/classwx_dll_loader.html) class.
pub trait DllLoaderMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDynamicLibrary](http://docs.wxwidgets.org/3.0/classwx_dynamic_library.html) class.
pub struct DynamicLibrary { ptr: *mut c_void }
impl DynamicLibraryMethods for DynamicLibrary { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicLibrary {
    pub fn from(ptr: *mut c_void) -> DynamicLibrary { DynamicLibrary { ptr: ptr } }
    pub fn null() -> DynamicLibrary { DynamicLibrary::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynamicLibrary](http://docs.wxwidgets.org/3.0/classwx_dynamic_library.html) class.
pub trait DynamicLibraryMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxEncodingConverter](http://docs.wxwidgets.org/3.0/classwx_encoding_converter.html) class.
pub struct EncodingConverter { ptr: *mut c_void }
impl EncodingConverterMethods for EncodingConverter {}
impl ObjectMethods for EncodingConverter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EncodingConverter {
    pub fn from(ptr: *mut c_void) -> EncodingConverter { EncodingConverter { ptr: ptr } }
    pub fn null() -> EncodingConverter { EncodingConverter::from(0 as *mut c_void) }
    
    pub fn new() -> EncodingConverter {
        unsafe { EncodingConverter::from(wxEncodingConverter_Create()) }
    }
}

/// Methods of the wxWidgets' [wxEncodingConverter](http://docs.wxwidgets.org/3.0/classwx_encoding_converter.html) class.
pub trait EncodingConverterMethods : ObjectMethods {
    fn convert(&self, input: *mut c_void, output: *mut c_void) {
        unsafe { wxEncodingConverter_Convert(self.ptr(), input, output) }
    }
    fn getAllEquivalents<T: ListMethods>(&self, enc: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.ptr(), enc, _lst.ptr()) }
    }
    fn getPlatformEquivalents<T: ListMethods>(&self, enc: c_int, platform: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.ptr(), enc, platform, _lst.ptr()) }
    }
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.ptr(), input_enc, output_enc, method) }
    }
}

/// Wraps the wxWidgets' [wxFFile](http://docs.wxwidgets.org/3.0/classwx_ff_ile.html) class.
pub struct FFile { ptr: *mut c_void }
impl FFileMethods for FFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FFile {
    pub fn from(ptr: *mut c_void) -> FFile { FFile { ptr: ptr } }
    pub fn null() -> FFile { FFile::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFFile](http://docs.wxwidgets.org/3.0/classwx_ff_ile.html) class.
pub trait FFileMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxFFileInputStream](http://docs.wxwidgets.org/3.0/classwx_ff_ile_input_stream.html) class.
pub struct FFileInputStream { ptr: *mut c_void }
impl FFileInputStreamMethods for FFileInputStream {}
impl InputStreamMethods for FFileInputStream {}
impl StreamBaseMethods for FFileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FFileInputStream {
    pub fn from(ptr: *mut c_void) -> FFileInputStream { FFileInputStream { ptr: ptr } }
    pub fn null() -> FFileInputStream { FFileInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFFileInputStream](http://docs.wxwidgets.org/3.0/classwx_ff_ile_input_stream.html) class.
pub trait FFileInputStreamMethods : InputStreamMethods {
}

/// Wraps the wxWidgets' [wxFFileOutputStream](http://docs.wxwidgets.org/3.0/classwx_ff_ile_output_stream.html) class.
pub struct FFileOutputStream { ptr: *mut c_void }
impl FFileOutputStreamMethods for FFileOutputStream {}
impl OutputStreamMethods for FFileOutputStream {}
impl StreamBaseMethods for FFileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FFileOutputStream {
    pub fn from(ptr: *mut c_void) -> FFileOutputStream { FFileOutputStream { ptr: ptr } }
    pub fn null() -> FFileOutputStream { FFileOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFFileOutputStream](http://docs.wxwidgets.org/3.0/classwx_ff_ile_output_stream.html) class.
pub trait FFileOutputStreamMethods : OutputStreamMethods {
}

/// Wraps the wxWidgets' [wxFSFile](http://docs.wxwidgets.org/3.0/classwx_fsf_ile.html) class.
pub struct FSFile { ptr: *mut c_void }
impl FSFileMethods for FSFile {}
impl ObjectMethods for FSFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FSFile {
    pub fn from(ptr: *mut c_void) -> FSFile { FSFile { ptr: ptr } }
    pub fn null() -> FSFile { FSFile::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFSFile](http://docs.wxwidgets.org/3.0/classwx_fsf_ile.html) class.
pub trait FSFileMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxFileInputStream](http://docs.wxwidgets.org/3.0/classwx_file_input_stream.html) class.
pub struct FileInputStream { ptr: *mut c_void }
impl FileInputStreamMethods for FileInputStream {}
impl InputStreamMethods for FileInputStream {}
impl StreamBaseMethods for FileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileInputStream {
    pub fn from(ptr: *mut c_void) -> FileInputStream { FileInputStream { ptr: ptr } }
    pub fn null() -> FileInputStream { FileInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileInputStream](http://docs.wxwidgets.org/3.0/classwx_file_input_stream.html) class.
pub trait FileInputStreamMethods : InputStreamMethods {
}

/// Wraps the wxWidgets' [wxFileName](http://docs.wxwidgets.org/3.0/classwx_file_name.html) class.
pub struct FileName { ptr: *mut c_void }
impl FileNameMethods for FileName { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileName {
    pub fn from(ptr: *mut c_void) -> FileName { FileName { ptr: ptr } }
    pub fn null() -> FileName { FileName::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileName](http://docs.wxwidgets.org/3.0/classwx_file_name.html) class.
pub trait FileNameMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxFileOutputStream](http://docs.wxwidgets.org/3.0/classwx_file_output_stream.html) class.
pub struct FileOutputStream { ptr: *mut c_void }
impl FileOutputStreamMethods for FileOutputStream {}
impl OutputStreamMethods for FileOutputStream {}
impl StreamBaseMethods for FileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileOutputStream {
    pub fn from(ptr: *mut c_void) -> FileOutputStream { FileOutputStream { ptr: ptr } }
    pub fn null() -> FileOutputStream { FileOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileOutputStream](http://docs.wxwidgets.org/3.0/classwx_file_output_stream.html) class.
pub trait FileOutputStreamMethods : OutputStreamMethods {
}

/// Wraps the wxWidgets' [wxFileSystem](http://docs.wxwidgets.org/3.0/classwx_file_system.html) class.
pub struct FileSystem { ptr: *mut c_void }
impl FileSystemMethods for FileSystem {}
impl ObjectMethods for FileSystem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileSystem {
    pub fn from(ptr: *mut c_void) -> FileSystem { FileSystem { ptr: ptr } }
    pub fn null() -> FileSystem { FileSystem::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileSystem](http://docs.wxwidgets.org/3.0/classwx_file_system.html) class.
pub trait FileSystemMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxFileSystemHandler](http://docs.wxwidgets.org/3.0/classwx_file_system_handler.html) class.
pub struct FileSystemHandler { ptr: *mut c_void }
impl FileSystemHandlerMethods for FileSystemHandler {}
impl ObjectMethods for FileSystemHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileSystemHandler {
    pub fn from(ptr: *mut c_void) -> FileSystemHandler { FileSystemHandler { ptr: ptr } }
    pub fn null() -> FileSystemHandler { FileSystemHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileSystemHandler](http://docs.wxwidgets.org/3.0/classwx_file_system_handler.html) class.
pub trait FileSystemHandlerMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxFilterInputStream](http://docs.wxwidgets.org/3.0/classwx_filter_input_stream.html) class.
pub struct FilterInputStream { ptr: *mut c_void }
impl FilterInputStreamMethods for FilterInputStream {}
impl InputStreamMethods for FilterInputStream {}
impl StreamBaseMethods for FilterInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FilterInputStream {
    pub fn from(ptr: *mut c_void) -> FilterInputStream { FilterInputStream { ptr: ptr } }
    pub fn null() -> FilterInputStream { FilterInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFilterInputStream](http://docs.wxwidgets.org/3.0/classwx_filter_input_stream.html) class.
pub trait FilterInputStreamMethods : InputStreamMethods {
}

/// Wraps the wxWidgets' [wxFilterOutputStream](http://docs.wxwidgets.org/3.0/classwx_filter_output_stream.html) class.
pub struct FilterOutputStream { ptr: *mut c_void }
impl FilterOutputStreamMethods for FilterOutputStream {}
impl OutputStreamMethods for FilterOutputStream {}
impl StreamBaseMethods for FilterOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FilterOutputStream {
    pub fn from(ptr: *mut c_void) -> FilterOutputStream { FilterOutputStream { ptr: ptr } }
    pub fn null() -> FilterOutputStream { FilterOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFilterOutputStream](http://docs.wxwidgets.org/3.0/classwx_filter_output_stream.html) class.
pub trait FilterOutputStreamMethods : OutputStreamMethods {
}

/// Wraps the wxWidgets' [wxInputStream](http://docs.wxwidgets.org/3.0/classwx_input_stream.html) class.
pub struct InputStream { ptr: *mut c_void }
impl InputStreamMethods for InputStream {}
impl StreamBaseMethods for InputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InputStream {
    pub fn from(ptr: *mut c_void) -> InputStream { InputStream { ptr: ptr } }
    pub fn null() -> InputStream { InputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxInputStream](http://docs.wxwidgets.org/3.0/classwx_input_stream.html) class.
pub trait InputStreamMethods : StreamBaseMethods {
    fn eof(&self) -> c_int {
        unsafe { wxInputStream_Eof(self.ptr()) }
    }
    fn getC(&self) -> int8_t {
        unsafe { wxInputStream_GetC(self.ptr()) }
    }
    fn lastRead(&self) -> c_int {
        unsafe { wxInputStream_LastRead(self.ptr()) }
    }
    fn peek(&self) -> int8_t {
        unsafe { wxInputStream_Peek(self.ptr()) }
    }
    fn read(&self, buffer: *mut c_void, size: c_int) {
        unsafe { wxInputStream_Read(self.ptr(), buffer, size) }
    }
    fn seekI(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxInputStream_SeekI(self.ptr(), pos, mode) }
    }
    fn tell(&self) -> c_int {
        unsafe { wxInputStream_Tell(self.ptr()) }
    }
    fn ungetBuffer(&self, buffer: *mut c_void, size: c_int) -> c_int {
        unsafe { wxInputStream_UngetBuffer(self.ptr(), buffer, size) }
    }
    fn ungetch(&self, c: int8_t) -> c_int {
        unsafe { wxInputStream_Ungetch(self.ptr(), c) }
    }
    fn canRead(&self) -> c_int {
        unsafe { wxInputStream_CanRead(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxList](http://docs.wxwidgets.org/3.0/classwx_list.html) class.
pub struct List { ptr: *mut c_void }
impl ListMethods for List {}
impl ObjectMethods for List { fn ptr(&self) -> *mut c_void { self.ptr } }

impl List {
    pub fn from(ptr: *mut c_void) -> List { List { ptr: ptr } }
    pub fn null() -> List { List::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxList](http://docs.wxwidgets.org/3.0/classwx_list.html) class.
pub trait ListMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxLocale](http://docs.wxwidgets.org/3.0/classwx_locale.html) class.
/// Rather use the wxRust-specific [RustLocale](struct.RustLocale.html) class.
pub struct Locale { ptr: *mut c_void }
impl LocaleMethods for Locale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Locale {
    pub fn from(ptr: *mut c_void) -> Locale { Locale { ptr: ptr } }
    pub fn null() -> Locale { Locale::from(0 as *mut c_void) }
    
    pub fn new(_name: c_int, _flags: c_int) -> Locale {
        unsafe { Locale::from(wxLocale_Create(_name, _flags)) }
    }
}

/// Methods of the wxWidgets' [wxLocale](http://docs.wxwidgets.org/3.0/classwx_locale.html) class.
pub trait LocaleMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn addCatalog(&self, szDomain: *mut c_void) -> c_int {
        unsafe { wxLocale_AddCatalog(self.ptr(), szDomain) }
    }
    fn addCatalogLookupPathPrefix(&self, prefix: *mut c_void) {
        unsafe { wxLocale_AddCatalogLookupPathPrefix(self.ptr(), prefix) }
    }
    fn delete(&self) {
        unsafe { wxLocale_Delete(self.ptr()) }
    }
    fn getLocale(&self) -> Locale {
        unsafe { Locale::from(wxLocale_GetLocale(self.ptr())) }
    }
    fn getName(&self) -> ~str {
        unsafe { String::from(wxLocale_GetName(self.ptr())).to_str() }
    }
    fn getString(&self, szOrigString: *mut c_void, szDomain: *mut c_void) -> *mut int8_t {
        unsafe { wxLocale_GetString(self.ptr(), szOrigString, szDomain) }
    }
    fn isLoaded(&self, szDomain: *mut c_void) -> c_int {
        unsafe { wxLocale_IsLoaded(self.ptr(), szDomain) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxLocale_IsOk(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxLongLong](http://docs.wxwidgets.org/3.0/classwx_long_long.html) class.
pub struct LongLong { ptr: *mut c_void }
impl LongLongMethods for LongLong { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LongLong {
    pub fn from(ptr: *mut c_void) -> LongLong { LongLong { ptr: ptr } }
    pub fn null() -> LongLong { LongLong::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxLongLong](http://docs.wxwidgets.org/3.0/classwx_long_long.html) class.
pub trait LongLongMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxMBConv](http://docs.wxwidgets.org/3.0/classwx_mbc_onv.html) class.
pub struct MBConv { ptr: *mut c_void }
impl MBConvMethods for MBConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConv {
    pub fn from(ptr: *mut c_void) -> MBConv { MBConv { ptr: ptr } }
    pub fn null() -> MBConv { MBConv::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMBConv](http://docs.wxwidgets.org/3.0/classwx_mbc_onv.html) class.
pub trait MBConvMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxMBConvUTF7](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_utf7.html) class.
pub struct MBConvUTF7 { ptr: *mut c_void }
impl MBConvUTF7Methods for MBConvUTF7 {}
impl MBConvMethods for MBConvUTF7 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvUTF7 {
    pub fn from(ptr: *mut c_void) -> MBConvUTF7 { MBConvUTF7 { ptr: ptr } }
    pub fn null() -> MBConvUTF7 { MBConvUTF7::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMBConvUTF7](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_utf7.html) class.
pub trait MBConvUTF7Methods : MBConvMethods {
}

/// Wraps the wxWidgets' [wxMBConvUTF8](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_utf8.html) class.
pub struct MBConvUTF8 { ptr: *mut c_void }
impl MBConvUTF8Methods for MBConvUTF8 {}
impl MBConvMethods for MBConvUTF8 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvUTF8 {
    pub fn from(ptr: *mut c_void) -> MBConvUTF8 { MBConvUTF8 { ptr: ptr } }
    pub fn null() -> MBConvUTF8 { MBConvUTF8::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMBConvUTF8](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_utf8.html) class.
pub trait MBConvUTF8Methods : MBConvMethods {
}

/// Wraps the wxWidgets' [wxMemoryFSHandler](http://docs.wxwidgets.org/3.0/classwx_memory_fsh_andler.html) class.
pub struct MemoryFSHandler { ptr: *mut c_void }
impl MemoryFSHandlerMethods for MemoryFSHandler {}
impl FileSystemHandlerMethods for MemoryFSHandler {}
impl ObjectMethods for MemoryFSHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryFSHandler {
    pub fn from(ptr: *mut c_void) -> MemoryFSHandler { MemoryFSHandler { ptr: ptr } }
    pub fn null() -> MemoryFSHandler { MemoryFSHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMemoryFSHandler](http://docs.wxwidgets.org/3.0/classwx_memory_fsh_andler.html) class.
pub trait MemoryFSHandlerMethods : FileSystemHandlerMethods {
}

/// Wraps the wxWidgets' [wxMemoryInputStream](http://docs.wxwidgets.org/3.0/classwx_memory_input_stream.html) class.
pub struct MemoryInputStream { ptr: *mut c_void }
impl MemoryInputStreamMethods for MemoryInputStream {}
impl InputStreamMethods for MemoryInputStream {}
impl StreamBaseMethods for MemoryInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryInputStream {
    pub fn from(ptr: *mut c_void) -> MemoryInputStream { MemoryInputStream { ptr: ptr } }
    pub fn null() -> MemoryInputStream { MemoryInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMemoryInputStream](http://docs.wxwidgets.org/3.0/classwx_memory_input_stream.html) class.
pub trait MemoryInputStreamMethods : InputStreamMethods {
}

/// Wraps the wxWidgets' [wxMemoryOutputStream](http://docs.wxwidgets.org/3.0/classwx_memory_output_stream.html) class.
pub struct MemoryOutputStream { ptr: *mut c_void }
impl MemoryOutputStreamMethods for MemoryOutputStream {}
impl OutputStreamMethods for MemoryOutputStream {}
impl StreamBaseMethods for MemoryOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryOutputStream {
    pub fn from(ptr: *mut c_void) -> MemoryOutputStream { MemoryOutputStream { ptr: ptr } }
    pub fn null() -> MemoryOutputStream { MemoryOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMemoryOutputStream](http://docs.wxwidgets.org/3.0/classwx_memory_output_stream.html) class.
pub trait MemoryOutputStreamMethods : OutputStreamMethods {
}

/// Wraps the wxWidgets' [wxModule](http://docs.wxwidgets.org/3.0/classwx_module.html) class.
pub struct Module { ptr: *mut c_void }
impl ModuleMethods for Module {}
impl ObjectMethods for Module { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Module {
    pub fn from(ptr: *mut c_void) -> Module { Module { ptr: ptr } }
    pub fn null() -> Module { Module::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxModule](http://docs.wxwidgets.org/3.0/classwx_module.html) class.
pub trait ModuleMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxMutex](http://docs.wxwidgets.org/3.0/classwx_mutex.html) class.
pub struct Mutex { ptr: *mut c_void }
impl MutexMethods for Mutex { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Mutex {
    pub fn from(ptr: *mut c_void) -> Mutex { Mutex { ptr: ptr } }
    pub fn null() -> Mutex { Mutex::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMutex](http://docs.wxwidgets.org/3.0/classwx_mutex.html) class.
pub trait MutexMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxMutexLocker](http://docs.wxwidgets.org/3.0/classwx_mutex_locker.html) class.
pub struct MutexLocker { ptr: *mut c_void }
impl MutexLockerMethods for MutexLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MutexLocker {
    pub fn from(ptr: *mut c_void) -> MutexLocker { MutexLocker { ptr: ptr } }
    pub fn null() -> MutexLocker { MutexLocker::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMutexLocker](http://docs.wxwidgets.org/3.0/classwx_mutex_locker.html) class.
pub trait MutexLockerMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxNodeBase](http://docs.wxwidgets.org/3.0/classwx_node_base.html) class.
pub struct NodeBase { ptr: *mut c_void }
impl NodeBaseMethods for NodeBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NodeBase {
    pub fn from(ptr: *mut c_void) -> NodeBase { NodeBase { ptr: ptr } }
    pub fn null() -> NodeBase { NodeBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxNodeBase](http://docs.wxwidgets.org/3.0/classwx_node_base.html) class.
pub trait NodeBaseMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxObject](http://docs.wxwidgets.org/3.0/classwx_object.html) class.
pub struct Object { ptr: *mut c_void }
impl ObjectMethods for Object { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Object {
    pub fn from(ptr: *mut c_void) -> Object { Object { ptr: ptr } }
    pub fn null() -> Object { Object::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxObject](http://docs.wxwidgets.org/3.0/classwx_object.html) class.
pub trait ObjectMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.ptr()) }
    }
    fn getClientClosure(&self) -> Closure {
        unsafe { Closure::from(wxObject_GetClientClosure(self.ptr())) }
    }
    fn setClientClosure<T: ClosureMethods>(&self, closure: &T) {
        unsafe { wxObject_SetClientClosure(self.ptr(), closure.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxObject_Delete(self.ptr()) }
    }
    fn getClassInfo(&self) -> ClassInfo {
        unsafe { ClassInfo::from(wxObject_GetClassInfo(self.ptr())) }
    }
    fn isKindOf<T: ClassInfoMethods>(&self, classInfo: &T) -> c_int {
        unsafe { wxObject_IsKindOf(self.ptr(), classInfo.ptr()) }
    }
    fn isScrolledWindow(&self) -> c_int {
        unsafe { wxObject_IsScrolledWindow(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxObjectRefData](http://docs.wxwidgets.org/3.0/classwx_object_ref_data.html) class.
pub struct ObjectRefData { ptr: *mut c_void }
impl ObjectRefDataMethods for ObjectRefData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ObjectRefData {
    pub fn from(ptr: *mut c_void) -> ObjectRefData { ObjectRefData { ptr: ptr } }
    pub fn null() -> ObjectRefData { ObjectRefData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxObjectRefData](http://docs.wxwidgets.org/3.0/classwx_object_ref_data.html) class.
pub trait ObjectRefDataMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxOutputStream](http://docs.wxwidgets.org/3.0/classwx_output_stream.html) class.
pub struct OutputStream { ptr: *mut c_void }
impl OutputStreamMethods for OutputStream {}
impl StreamBaseMethods for OutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl OutputStream {
    pub fn from(ptr: *mut c_void) -> OutputStream { OutputStream { ptr: ptr } }
    pub fn null() -> OutputStream { OutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxOutputStream](http://docs.wxwidgets.org/3.0/classwx_output_stream.html) class.
pub trait OutputStreamMethods : StreamBaseMethods {
    fn lastWrite(&self) -> c_int {
        unsafe { wxOutputStream_LastWrite(self.ptr()) }
    }
    fn putC(&self, c: int8_t) {
        unsafe { wxOutputStream_PutC(self.ptr(), c) }
    }
    fn seek(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxOutputStream_Seek(self.ptr(), pos, mode) }
    }
    fn sync(&self) {
        unsafe { wxOutputStream_Sync(self.ptr()) }
    }
    fn tell(&self) -> c_int {
        unsafe { wxOutputStream_Tell(self.ptr()) }
    }
    fn write(&self, buffer: *mut c_void, size: c_int) {
        unsafe { wxOutputStream_Write(self.ptr(), buffer, size) }
    }
}

/// Wraps the wxWidgets' [wxPathList](http://docs.wxwidgets.org/3.0/classwx_path_list.html) class.
pub struct PathList { ptr: *mut c_void }
impl PathListMethods for PathList {}
impl ListMethods for PathList {}
impl ObjectMethods for PathList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PathList {
    pub fn from(ptr: *mut c_void) -> PathList { PathList { ptr: ptr } }
    pub fn null() -> PathList { PathList::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPathList](http://docs.wxwidgets.org/3.0/classwx_path_list.html) class.
pub trait PathListMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxRegEx](http://docs.wxwidgets.org/3.0/classwx_reg_ex.html) class.
pub struct RegEx { ptr: *mut c_void }
impl RegExMethods for RegEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RegEx {
    pub fn from(ptr: *mut c_void) -> RegEx { RegEx { ptr: ptr } }
    pub fn null() -> RegEx { RegEx::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxRegEx](http://docs.wxwidgets.org/3.0/classwx_reg_ex.html) class.
pub trait RegExMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxScopedArray](http://docs.wxwidgets.org/3.0/classwx_scoped_array.html) class.
pub struct ScopedArray { ptr: *mut c_void }
impl ScopedArrayMethods for ScopedArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScopedArray {
    pub fn from(ptr: *mut c_void) -> ScopedArray { ScopedArray { ptr: ptr } }
    pub fn null() -> ScopedArray { ScopedArray::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxScopedArray](http://docs.wxwidgets.org/3.0/classwx_scoped_array.html) class.
pub trait ScopedArrayMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxScopedPtr](http://docs.wxwidgets.org/3.0/classwx_scoped_ptr.html) class.
pub struct ScopedPtr { ptr: *mut c_void }
impl ScopedPtrMethods for ScopedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScopedPtr {
    pub fn from(ptr: *mut c_void) -> ScopedPtr { ScopedPtr { ptr: ptr } }
    pub fn null() -> ScopedPtr { ScopedPtr::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxScopedPtr](http://docs.wxwidgets.org/3.0/classwx_scoped_ptr.html) class.
pub trait ScopedPtrMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxSemaphore](http://docs.wxwidgets.org/3.0/classwx_semaphore.html) class.
pub struct Semaphore { ptr: *mut c_void }
impl SemaphoreMethods for Semaphore { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Semaphore {
    pub fn from(ptr: *mut c_void) -> Semaphore { Semaphore { ptr: ptr } }
    pub fn null() -> Semaphore { Semaphore::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSemaphore](http://docs.wxwidgets.org/3.0/classwx_semaphore.html) class.
pub trait SemaphoreMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxServer](http://docs.wxwidgets.org/3.0/classwx_server.html) class.
/// Rather use the wxRust-specific [RustServer](struct.RustServer.html) class.
pub struct Server { ptr: *mut c_void }
impl ServerMethods for Server {}
impl ServerBaseMethods for Server {}
impl ObjectMethods for Server { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Server {
    pub fn from(ptr: *mut c_void) -> Server { Server { ptr: ptr } }
    pub fn null() -> Server { Server::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxServer](http://docs.wxwidgets.org/3.0/classwx_server.html) class.
pub trait ServerMethods : ServerBaseMethods {
}

/// Wraps the wxWidgets' [wxServerBase](http://docs.wxwidgets.org/3.0/classwx_server_base.html) class.
pub struct ServerBase { ptr: *mut c_void }
impl ServerBaseMethods for ServerBase {}
impl ObjectMethods for ServerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ServerBase {
    pub fn from(ptr: *mut c_void) -> ServerBase { ServerBase { ptr: ptr } }
    pub fn null() -> ServerBase { ServerBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxServerBase](http://docs.wxwidgets.org/3.0/classwx_server_base.html) class.
pub trait ServerBaseMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxSingleInstanceChecker](http://docs.wxwidgets.org/3.0/classwx_single_instance_checker.html) class.
pub struct SingleInstanceChecker { ptr: *mut c_void }
impl SingleInstanceCheckerMethods for SingleInstanceChecker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SingleInstanceChecker {
    pub fn from(ptr: *mut c_void) -> SingleInstanceChecker { SingleInstanceChecker { ptr: ptr } }
    pub fn null() -> SingleInstanceChecker { SingleInstanceChecker::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, name: &str, path: &str) -> c_int {
        let name = strToString(name);
        let path = strToString(path);
        unsafe { wxSingleInstanceChecker_Create(_obj, name.ptr(), path.ptr()) }
    }
    pub fn newDefault() -> SingleInstanceChecker {
        unsafe { SingleInstanceChecker::from(wxSingleInstanceChecker_CreateDefault()) }
    }
}

/// Methods of the wxWidgets' [wxSingleInstanceChecker](http://docs.wxwidgets.org/3.0/classwx_single_instance_checker.html) class.
pub trait SingleInstanceCheckerMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.ptr()) }
    }
    fn isAnotherRunning(&self) -> c_int {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxStopWatch](http://docs.wxwidgets.org/3.0/classwx_stop_watch.html) class.
pub struct StopWatch { ptr: *mut c_void }
impl StopWatchMethods for StopWatch { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StopWatch {
    pub fn from(ptr: *mut c_void) -> StopWatch { StopWatch { ptr: ptr } }
    pub fn null() -> StopWatch { StopWatch::from(0 as *mut c_void) }
    
    pub fn new() -> StopWatch {
        unsafe { StopWatch::from(wxStopWatch_Create()) }
    }
}

/// Methods of the wxWidgets' [wxStopWatch](http://docs.wxwidgets.org/3.0/classwx_stop_watch.html) class.
pub trait StopWatchMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxStopWatch_Delete(self.ptr()) }
    }
    fn start(&self, msec: c_int) {
        unsafe { wxStopWatch_Start(self.ptr(), msec) }
    }
    fn pause(&self) {
        unsafe { wxStopWatch_Pause(self.ptr()) }
    }
    fn resume(&self) {
        unsafe { wxStopWatch_Resume(self.ptr()) }
    }
    fn time(&self) -> c_int {
        unsafe { wxStopWatch_Time(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxStreamBase](http://docs.wxwidgets.org/3.0/classwx_stream_base.html) class.
pub struct StreamBase { ptr: *mut c_void }
impl StreamBaseMethods for StreamBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamBase {
    pub fn from(ptr: *mut c_void) -> StreamBase { StreamBase { ptr: ptr } }
    pub fn null() -> StreamBase { StreamBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStreamBase](http://docs.wxwidgets.org/3.0/classwx_stream_base.html) class.
pub trait StreamBaseMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn getLastError(&self) -> c_int {
        unsafe { wxStreamBase_GetLastError(self.ptr()) }
    }
    fn getSize(&self) -> c_int {
        unsafe { wxStreamBase_GetSize(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxStreamBase_IsOk(self.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxStreamBase_Delete(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxStreamBuffer](http://docs.wxwidgets.org/3.0/classwx_stream_buffer.html) class.
pub struct StreamBuffer { ptr: *mut c_void }
impl StreamBufferMethods for StreamBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamBuffer {
    pub fn from(ptr: *mut c_void) -> StreamBuffer { StreamBuffer { ptr: ptr } }
    pub fn null() -> StreamBuffer { StreamBuffer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStreamBuffer](http://docs.wxwidgets.org/3.0/classwx_stream_buffer.html) class.
pub trait StreamBufferMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxStringBuffer](http://docs.wxwidgets.org/3.0/classwx_string_buffer.html) class.
pub struct StringBuffer { ptr: *mut c_void }
impl StringBufferMethods for StringBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringBuffer {
    pub fn from(ptr: *mut c_void) -> StringBuffer { StringBuffer { ptr: ptr } }
    pub fn null() -> StringBuffer { StringBuffer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStringBuffer](http://docs.wxwidgets.org/3.0/classwx_string_buffer.html) class.
pub trait StringBufferMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxStringClientData](http://docs.wxwidgets.org/3.0/classwx_string_client_data.html) class.
pub struct StringClientData { ptr: *mut c_void }
impl StringClientDataMethods for StringClientData {}
impl ClientDataMethods for StringClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringClientData {
    pub fn from(ptr: *mut c_void) -> StringClientData { StringClientData { ptr: ptr } }
    pub fn null() -> StringClientData { StringClientData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStringClientData](http://docs.wxwidgets.org/3.0/classwx_string_client_data.html) class.
pub trait StringClientDataMethods : ClientDataMethods {
}

/// Wraps the wxWidgets' [wxStringList](http://docs.wxwidgets.org/3.0/classwx_string_list.html) class.
pub struct StringList { ptr: *mut c_void }
impl StringListMethods for StringList {}
impl ListMethods for StringList {}
impl ObjectMethods for StringList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringList {
    pub fn from(ptr: *mut c_void) -> StringList { StringList { ptr: ptr } }
    pub fn null() -> StringList { StringList::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStringList](http://docs.wxwidgets.org/3.0/classwx_string_list.html) class.
pub trait StringListMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxStringTokenizer](http://docs.wxwidgets.org/3.0/classwx_string_tokenizer.html) class.
pub struct StringTokenizer { ptr: *mut c_void }
impl StringTokenizerMethods for StringTokenizer {}
impl ObjectMethods for StringTokenizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringTokenizer {
    pub fn from(ptr: *mut c_void) -> StringTokenizer { StringTokenizer { ptr: ptr } }
    pub fn null() -> StringTokenizer { StringTokenizer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStringTokenizer](http://docs.wxwidgets.org/3.0/classwx_string_tokenizer.html) class.
pub trait StringTokenizerMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxSystemOptions](http://docs.wxwidgets.org/3.0/classwx_system_options.html) class.
pub struct SystemOptions { ptr: *mut c_void }
impl SystemOptionsMethods for SystemOptions {}
impl ObjectMethods for SystemOptions { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SystemOptions {
    pub fn from(ptr: *mut c_void) -> SystemOptions { SystemOptions { ptr: ptr } }
    pub fn null() -> SystemOptions { SystemOptions::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSystemOptions](http://docs.wxwidgets.org/3.0/classwx_system_options.html) class.
pub trait SystemOptionsMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxTempFile](http://docs.wxwidgets.org/3.0/classwx_temp_file.html) class.
pub struct TempFile { ptr: *mut c_void }
impl TempFileMethods for TempFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TempFile {
    pub fn from(ptr: *mut c_void) -> TempFile { TempFile { ptr: ptr } }
    pub fn null() -> TempFile { TempFile::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTempFile](http://docs.wxwidgets.org/3.0/classwx_temp_file.html) class.
pub trait TempFileMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxTextFile](http://docs.wxwidgets.org/3.0/classwx_text_file.html) class.
pub struct TextFile { ptr: *mut c_void }
impl TextFileMethods for TextFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextFile {
    pub fn from(ptr: *mut c_void) -> TextFile { TextFile { ptr: ptr } }
    pub fn null() -> TextFile { TextFile::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTextFile](http://docs.wxwidgets.org/3.0/classwx_text_file.html) class.
pub trait TextFileMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxTextInputStream](http://docs.wxwidgets.org/3.0/classwx_text_input_stream.html) class.
pub struct TextInputStream { ptr: *mut c_void }
impl TextInputStreamMethods for TextInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextInputStream {
    pub fn from(ptr: *mut c_void) -> TextInputStream { TextInputStream { ptr: ptr } }
    pub fn null() -> TextInputStream { TextInputStream::from(0 as *mut c_void) }
    
    pub fn new<T: InputStreamMethods>(inputStream: &T, sep: &str) -> TextInputStream {
        let sep = strToString(sep);
        unsafe { TextInputStream::from(wxTextInputStream_Create(inputStream.ptr(), sep.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxTextInputStream](http://docs.wxwidgets.org/3.0/classwx_text_input_stream.html) class.
pub trait TextInputStreamMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.ptr()) }
    }
    fn readLine(&self) -> ~str {
        unsafe { String::from(wxTextInputStream_ReadLine(self.ptr())).to_str() }
    }
}

/// Wraps the wxWidgets' [wxTextOutputStream](http://docs.wxwidgets.org/3.0/classwx_text_output_stream.html) class.
pub struct TextOutputStream { ptr: *mut c_void }
impl TextOutputStreamMethods for TextOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextOutputStream {
    pub fn from(ptr: *mut c_void) -> TextOutputStream { TextOutputStream { ptr: ptr } }
    pub fn null() -> TextOutputStream { TextOutputStream::from(0 as *mut c_void) }
    
    pub fn new<T: OutputStreamMethods>(outputStream: &T, mode: c_int) -> TextOutputStream {
        unsafe { TextOutputStream::from(wxTextOutputStream_Create(outputStream.ptr(), mode)) }
    }
}

/// Methods of the wxWidgets' [wxTextOutputStream](http://docs.wxwidgets.org/3.0/classwx_text_output_stream.html) class.
pub trait TextOutputStreamMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.ptr()) }
    }
    fn writeString(&self, txt: &str) {
        let txt = strToString(txt);
        unsafe { wxTextOutputStream_WriteString(self.ptr(), txt.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxThread](http://docs.wxwidgets.org/3.0/classwx_thread.html) class.
pub struct Thread { ptr: *mut c_void }
impl ThreadMethods for Thread { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Thread {
    pub fn from(ptr: *mut c_void) -> Thread { Thread { ptr: ptr } }
    pub fn null() -> Thread { Thread::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxThread](http://docs.wxwidgets.org/3.0/classwx_thread.html) class.
pub trait ThreadMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxTime](http://docs.wxwidgets.org/3.0/classwx_time.html) class.
pub struct Time { ptr: *mut c_void }
impl TimeMethods for Time {}
impl ObjectMethods for Time { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Time {
    pub fn from(ptr: *mut c_void) -> Time { Time { ptr: ptr } }
    pub fn null() -> Time { Time::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTime](http://docs.wxwidgets.org/3.0/classwx_time.html) class.
pub trait TimeMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxTimeSpan](http://docs.wxwidgets.org/3.0/classwx_time_span.html) class.
pub struct TimeSpan { ptr: *mut c_void }
impl TimeSpanMethods for TimeSpan { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimeSpan {
    pub fn from(ptr: *mut c_void) -> TimeSpan { TimeSpan { ptr: ptr } }
    pub fn null() -> TimeSpan { TimeSpan::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTimeSpan](http://docs.wxwidgets.org/3.0/classwx_time_span.html) class.
pub trait TimeSpanMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxVariant](http://docs.wxwidgets.org/3.0/classwx_variant.html) class.
pub struct Variant { ptr: *mut c_void }
impl VariantMethods for Variant {}
impl ObjectMethods for Variant { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Variant {
    pub fn from(ptr: *mut c_void) -> Variant { Variant { ptr: ptr } }
    pub fn null() -> Variant { Variant::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxVariant](http://docs.wxwidgets.org/3.0/classwx_variant.html) class.
pub trait VariantMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxVariantData](http://docs.wxwidgets.org/3.0/classwx_variant_data.html) class.
pub struct VariantData { ptr: *mut c_void }
impl VariantDataMethods for VariantData {}
impl ObjectMethods for VariantData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl VariantData {
    pub fn from(ptr: *mut c_void) -> VariantData { VariantData { ptr: ptr } }
    pub fn null() -> VariantData { VariantData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxVariantData](http://docs.wxwidgets.org/3.0/classwx_variant_data.html) class.
pub trait VariantDataMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxZipInputStream](http://docs.wxwidgets.org/3.0/classwx_zip_input_stream.html) class.
pub struct ZipInputStream { ptr: *mut c_void }
impl ZipInputStreamMethods for ZipInputStream {}
impl InputStreamMethods for ZipInputStream {}
impl StreamBaseMethods for ZipInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ZipInputStream {
    pub fn from(ptr: *mut c_void) -> ZipInputStream { ZipInputStream { ptr: ptr } }
    pub fn null() -> ZipInputStream { ZipInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxZipInputStream](http://docs.wxwidgets.org/3.0/classwx_zip_input_stream.html) class.
pub trait ZipInputStreamMethods : InputStreamMethods {
}

/// Wraps the wxWidgets' [wxZlibInputStream](http://docs.wxwidgets.org/3.0/classwx_zlib_input_stream.html) class.
pub struct ZlibInputStream { ptr: *mut c_void }
impl ZlibInputStreamMethods for ZlibInputStream {}
impl FilterInputStreamMethods for ZlibInputStream {}
impl InputStreamMethods for ZlibInputStream {}
impl StreamBaseMethods for ZlibInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ZlibInputStream {
    pub fn from(ptr: *mut c_void) -> ZlibInputStream { ZlibInputStream { ptr: ptr } }
    pub fn null() -> ZlibInputStream { ZlibInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxZlibInputStream](http://docs.wxwidgets.org/3.0/classwx_zlib_input_stream.html) class.
pub trait ZlibInputStreamMethods : FilterInputStreamMethods {
}

/// Wraps the wxWidgets' [wxZlibOutputStream](http://docs.wxwidgets.org/3.0/classwx_zlib_output_stream.html) class.
pub struct ZlibOutputStream { ptr: *mut c_void }
impl ZlibOutputStreamMethods for ZlibOutputStream {}
impl FilterOutputStreamMethods for ZlibOutputStream {}
impl OutputStreamMethods for ZlibOutputStream {}
impl StreamBaseMethods for ZlibOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ZlibOutputStream {
    pub fn from(ptr: *mut c_void) -> ZlibOutputStream { ZlibOutputStream { ptr: ptr } }
    pub fn null() -> ZlibOutputStream { ZlibOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxZlibOutputStream](http://docs.wxwidgets.org/3.0/classwx_zlib_output_stream.html) class.
pub trait ZlibOutputStreamMethods : FilterOutputStreamMethods {
}

/// Wraps the wxWidgets' [wxMemoryBuffer](http://docs.wxwidgets.org/3.0/classwx_memory_buffer.html) class.
pub struct MemoryBuffer { ptr: *mut c_void }
impl MemoryBufferMethods for MemoryBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryBuffer {
    pub fn from(ptr: *mut c_void) -> MemoryBuffer { MemoryBuffer { ptr: ptr } }
    pub fn null() -> MemoryBuffer { MemoryBuffer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMemoryBuffer](http://docs.wxwidgets.org/3.0/classwx_memory_buffer.html) class.
pub trait MemoryBufferMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxFileConfig](http://docs.wxwidgets.org/3.0/classwx_file_config.html) class.
pub struct FileConfig { ptr: *mut c_void }
impl FileConfigMethods for FileConfig {}
impl ConfigBaseMethods for FileConfig { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileConfig {
    pub fn from(ptr: *mut c_void) -> FileConfig { FileConfig { ptr: ptr } }
    pub fn null() -> FileConfig { FileConfig::from(0 as *mut c_void) }
    
    pub fn new<T: InputStreamMethods>(inp: &T) -> FileConfig {
        unsafe { FileConfig::from(wxFileConfig_Create(inp.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxFileConfig](http://docs.wxwidgets.org/3.0/classwx_file_config.html) class.
pub trait FileConfigMethods : ConfigBaseMethods {
}

