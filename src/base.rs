use std::libc::*;
use std::str;
use native::*;

#[link_args="-lwxc"]
extern {
    fn wxString_CreateUTF8(buffer: *mut c_void) -> *mut c_void;
    fn wxString_GetUtf8(wxs: *mut c_void) -> *mut c_void;
    fn wxCharBuffer_DataUtf8(wxcb: *mut c_void) -> *c_char;
    fn wxCharBuffer_Delete(wxcb: *mut c_void);
}

#[fixed_stack_segment]
#[inline(never)]
pub fn wxT(s: &str) -> wxString {
    unsafe {
        do s.to_c_str().with_ref |c_str| {
            wxString { handle: wxString_CreateUTF8(c_str as *mut c_void) }
        }
    }
}

pub struct wxString { handle: *mut c_void }
impl Drop for wxString {
    #[fixed_stack_segment]
    #[inline(never)]
    fn drop(&mut self) {
        unsafe { wxString_Delete(self.handle); }
    }
}
impl wxString {
    pub fn handle(&self) -> *mut c_void { self.handle }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn to_str(&self) -> ~str {
        unsafe {
            let charBuffer = wxString_GetUtf8(self.handle);
            let utf8 = wxCharBuffer_DataUtf8(charBuffer);
            wxCharBuffer_Delete(charBuffer);
            str::raw::from_c_str(utf8)
        }
    }
}

pub struct ELJClient(*mut c_void);
impl _ELJClient for ELJClient {}
impl _wxClient for ELJClient {}
impl _wxClientBase for ELJClient {}
impl _wxObject for ELJClient { fn handle(&self) -> *mut c_void { **self } }

impl ELJClient {
    pub fn from(handle: *mut c_void) -> @ELJClient { @ELJClient(handle) }
    pub fn null() -> @ELJClient { ELJClient::from(0 as *mut c_void) }
    
}

pub trait _ELJClient : _wxClient {
}

pub struct ELJConnection(*mut c_void);
impl _ELJConnection for ELJConnection {}
impl _wxConnection for ELJConnection {}
impl _wxConnectionBase for ELJConnection {}
impl _wxObject for ELJConnection { fn handle(&self) -> *mut c_void { **self } }

impl ELJConnection {
    pub fn from(handle: *mut c_void) -> @ELJConnection { @ELJConnection(handle) }
    pub fn null() -> @ELJConnection { ELJConnection::from(0 as *mut c_void) }
    
}

pub trait _ELJConnection : _wxConnection {
}

pub struct ELJLocale(*mut c_void);
impl _ELJLocale for ELJLocale {}
impl _wxLocale for ELJLocale { fn handle(&self) -> *mut c_void { **self } }

impl ELJLocale {
    pub fn from(handle: *mut c_void) -> @ELJLocale { @ELJLocale(handle) }
    pub fn null() -> @ELJLocale { ELJLocale::from(0 as *mut c_void) }
    
}

pub trait _ELJLocale : _wxLocale {
}

pub struct ELJServer(*mut c_void);
impl _ELJServer for ELJServer {}
impl _wxServer for ELJServer {}
impl _wxServerBase for ELJServer {}
impl _wxObject for ELJServer { fn handle(&self) -> *mut c_void { **self } }

impl ELJServer {
    pub fn from(handle: *mut c_void) -> @ELJServer { @ELJServer(handle) }
    pub fn null() -> @ELJServer { ELJServer::from(0 as *mut c_void) }
    
}

pub trait _ELJServer : _wxServer {
}

pub struct wxArray(*mut c_void);
impl _wxArray for wxArray { fn handle(&self) -> *mut c_void { **self } }

impl wxArray {
    pub fn from(handle: *mut c_void) -> @wxArray { @wxArray(handle) }
    pub fn null() -> @wxArray { wxArray::from(0 as *mut c_void) }
    
}

pub trait _wxArray {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxArrayString(*mut c_void);
impl _wxArrayString for wxArrayString {}
impl _wxArray for wxArrayString { fn handle(&self) -> *mut c_void { **self } }

impl wxArrayString {
    pub fn from(handle: *mut c_void) -> @wxArrayString { @wxArrayString(handle) }
    pub fn null() -> @wxArrayString { wxArrayString::from(0 as *mut c_void) }
    
}

pub trait _wxArrayString : _wxArray {
}

pub struct wxBufferedInputStream(*mut c_void);
impl _wxBufferedInputStream for wxBufferedInputStream {}
impl _wxFilterInputStream for wxBufferedInputStream {}
impl _wxInputStream for wxBufferedInputStream {}
impl _wxStreamBase for wxBufferedInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxBufferedInputStream {
    pub fn from(handle: *mut c_void) -> @wxBufferedInputStream { @wxBufferedInputStream(handle) }
    pub fn null() -> @wxBufferedInputStream { wxBufferedInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxBufferedInputStream : _wxFilterInputStream {
}

pub struct wxBufferedOutputStream(*mut c_void);
impl _wxBufferedOutputStream for wxBufferedOutputStream {}
impl _wxFilterOutputStream for wxBufferedOutputStream {}
impl _wxOutputStream for wxBufferedOutputStream {}
impl _wxStreamBase for wxBufferedOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxBufferedOutputStream {
    pub fn from(handle: *mut c_void) -> @wxBufferedOutputStream { @wxBufferedOutputStream(handle) }
    pub fn null() -> @wxBufferedOutputStream { wxBufferedOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxBufferedOutputStream : _wxFilterOutputStream {
}

pub struct wxCSConv(*mut c_void);
impl _wxCSConv for wxCSConv {}
impl _wxMBConv for wxCSConv { fn handle(&self) -> *mut c_void { **self } }

impl wxCSConv {
    pub fn from(handle: *mut c_void) -> @wxCSConv { @wxCSConv(handle) }
    pub fn null() -> @wxCSConv { wxCSConv::from(0 as *mut c_void) }
    
}

pub trait _wxCSConv : _wxMBConv {
}

pub struct wxClassInfo(*mut c_void);
impl _wxClassInfo for wxClassInfo { fn handle(&self) -> *mut c_void { **self } }

impl wxClassInfo {
    pub fn from(handle: *mut c_void) -> @wxClassInfo { @wxClassInfo(handle) }
    pub fn null() -> @wxClassInfo { wxClassInfo::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findClass(_txt: &str) -> @wxClassInfo {
        let _txt = wxT(_txt);
        unsafe { @wxClassInfo(wxClassInfo_FindClass(_txt.handle())) }
    }
}

pub trait _wxClassInfo {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn newClassByName(&self) -> *mut c_void {
        unsafe { wxClassInfo_CreateClassByName(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClassName(&self) -> *mut c_void {
        unsafe { wxClassInfo_GetClassName(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isKindOf(&self, _name: &str) -> c_int {
        let _name = wxT(_name);
        unsafe { wxClassInfo_IsKindOf(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBaseClassName1(&self) -> ~str {
        unsafe { wxString { handle: wxClassInfo_GetBaseClassName1(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBaseClassName2(&self) -> ~str {
        unsafe { wxString { handle: wxClassInfo_GetBaseClassName2(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClassNameEx(&self) -> ~str {
        unsafe { wxString { handle: wxClassInfo_GetClassNameEx(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isKindOfEx<T: _wxClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxClassInfo_IsKindOfEx(self.handle(), classInfo.handle()) }
    }
}

pub struct wxClient(*mut c_void);
impl _wxClient for wxClient {}
impl _wxClientBase for wxClient {}
impl _wxObject for wxClient { fn handle(&self) -> *mut c_void { **self } }

impl wxClient {
    pub fn from(handle: *mut c_void) -> @wxClient { @wxClient(handle) }
    pub fn null() -> @wxClient { wxClient::from(0 as *mut c_void) }
    
}

pub trait _wxClient : _wxClientBase {
}

pub struct wxClientBase(*mut c_void);
impl _wxClientBase for wxClientBase {}
impl _wxObject for wxClientBase { fn handle(&self) -> *mut c_void { **self } }

impl wxClientBase {
    pub fn from(handle: *mut c_void) -> @wxClientBase { @wxClientBase(handle) }
    pub fn null() -> @wxClientBase { wxClientBase::from(0 as *mut c_void) }
    
}

pub trait _wxClientBase : _wxObject {
}

pub struct wxClientData(*mut c_void);
impl _wxClientData for wxClientData { fn handle(&self) -> *mut c_void { **self } }

impl wxClientData {
    pub fn from(handle: *mut c_void) -> @wxClientData { @wxClientData(handle) }
    pub fn null() -> @wxClientData { wxClientData::from(0 as *mut c_void) }
    
}

pub trait _wxClientData {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxClientDataContainer(*mut c_void);
impl _wxClientDataContainer for wxClientDataContainer { fn handle(&self) -> *mut c_void { **self } }

impl wxClientDataContainer {
    pub fn from(handle: *mut c_void) -> @wxClientDataContainer { @wxClientDataContainer(handle) }
    pub fn null() -> @wxClientDataContainer { wxClientDataContainer::from(0 as *mut c_void) }
    
}

pub trait _wxClientDataContainer {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxClosure(*mut c_void);
impl _wxClosure for wxClosure {}
impl _wxObject for wxClosure { fn handle(&self) -> *mut c_void { **self } }

impl wxClosure {
    pub fn from(handle: *mut c_void) -> @wxClosure { @wxClosure(handle) }
    pub fn null() -> @wxClosure { wxClosure::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_fun_CEvent: *mut c_void, _data: *mut c_void) -> @wxClosure {
        unsafe { @wxClosure(wxClosure_Create(_fun_CEvent, _data)) }
    }
}

pub trait _wxClosure : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> *mut c_void {
        unsafe { wxClosure_GetData(self.handle()) }
    }
}

pub struct wxCommandLineParser(*mut c_void);
impl _wxCommandLineParser for wxCommandLineParser { fn handle(&self) -> *mut c_void { **self } }

impl wxCommandLineParser {
    pub fn from(handle: *mut c_void) -> @wxCommandLineParser { @wxCommandLineParser(handle) }
    pub fn null() -> @wxCommandLineParser { wxCommandLineParser::from(0 as *mut c_void) }
    
}

pub trait _wxCommandLineParser {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxCondition(*mut c_void);
impl _wxCondition for wxCondition { fn handle(&self) -> *mut c_void { **self } }

impl wxCondition {
    pub fn from(handle: *mut c_void) -> @wxCondition { @wxCondition(handle) }
    pub fn null() -> @wxCondition { wxCondition::from(0 as *mut c_void) }
    
}

pub trait _wxCondition {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxConfigBase(*mut c_void);
impl _wxConfigBase for wxConfigBase { fn handle(&self) -> *mut c_void { **self } }

impl wxConfigBase {
    pub fn from(handle: *mut c_void) -> @wxConfigBase { @wxConfigBase(handle) }
    pub fn null() -> @wxConfigBase { wxConfigBase::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> @wxConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Get()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn set<T: _wxConfigBase>(self_: &T) {
        unsafe { wxConfigBase_Set(self_.handle()) }
    }
}

pub trait _wxConfigBase {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAll(&self) -> c_int {
        unsafe { wxConfigBase_DeleteAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteEntry(&self, key: &str, bDeleteGroupIfEmpty: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_DeleteEntry(self.handle(), key.handle(), bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteGroup(&self, key: &str) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_DeleteGroup(self.handle(), key.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn exists(&self, strName: &str) -> c_int {
        let strName = wxT(strName);
        unsafe { wxConfigBase_Exists(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expandEnvVars(&self, str: &str) -> ~str {
        let str = wxT(str);
        unsafe { wxString { handle: wxConfigBase_ExpandEnvVars(self.handle(), str.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flush(&self, bCurrentOnly: c_int) -> c_int {
        unsafe { wxConfigBase_Flush(self.handle(), bCurrentOnly) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAppName(&self) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetAppName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEntryType(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxConfigBase_GetEntryType(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetFirstEntry(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetFirstGroup(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetNextEntry(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetNextGroup(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfEntries(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfEntries(self.handle(), bRecursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfGroups(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfGroups(self.handle(), bRecursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPath(&self) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetPath(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVendorName(&self) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetVendorName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasEntry(&self, strName: &str) -> c_int {
        let strName = wxT(strName);
        unsafe { wxConfigBase_HasEntry(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasGroup(&self, strName: &str) -> c_int {
        let strName = wxT(strName);
        unsafe { wxConfigBase_HasGroup(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExpandingEnvVars(&self) -> c_int {
        unsafe { wxConfigBase_IsExpandingEnvVars(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRecordingDefaults(&self) -> c_int {
        unsafe { wxConfigBase_IsRecordingDefaults(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readBool(&self, key: &str, defVal: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadBool(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readDouble(&self, key: &str, defVal: c_double) -> c_double {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadDouble(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readInteger(&self, key: &str, defVal: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadInteger(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readString(&self, key: &str, defVal: &str) -> ~str {
        let key = wxT(key);
        let defVal = wxT(defVal);
        unsafe { wxString { handle: wxConfigBase_ReadString(self.handle(), key.handle(), defVal.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renameEntry(&self, oldName: &str, newName: &str) -> c_int {
        let oldName = wxT(oldName);
        let newName = wxT(newName);
        unsafe { wxConfigBase_RenameEntry(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renameGroup(&self, oldName: &str, newName: &str) -> c_int {
        let oldName = wxT(oldName);
        let newName = wxT(newName);
        unsafe { wxConfigBase_RenameGroup(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAppName(&self, appName: &str) {
        let appName = wxT(appName);
        unsafe { wxConfigBase_SetAppName(self.handle(), appName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExpandEnvVars(&self, bDoIt: c_int) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPath(&self, strPath: &str) {
        let strPath = wxT(strPath);
        unsafe { wxConfigBase_SetPath(self.handle(), strPath.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRecordDefaults(&self, bDoIt: c_int) {
        unsafe { wxConfigBase_SetRecordDefaults(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxConfigBase_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVendorName(&self, vendorName: &str) {
        let vendorName = wxT(vendorName);
        unsafe { wxConfigBase_SetVendorName(self.handle(), vendorName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeBool(&self, key: &str, value: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteBool(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeDouble(&self, key: &str, value: c_double) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteDouble(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeInteger(&self, key: &str, value: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteInteger(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeLong(&self, key: &str, value: c_long) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteLong(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeString(&self, key: &str, value: &str) -> c_int {
        let key = wxT(key);
        let value = wxT(value);
        unsafe { wxConfigBase_WriteString(self.handle(), key.handle(), value.handle()) }
    }
}

pub struct wxConnection(*mut c_void);
impl _wxConnection for wxConnection {}
impl _wxConnectionBase for wxConnection {}
impl _wxObject for wxConnection { fn handle(&self) -> *mut c_void { **self } }

impl wxConnection {
    pub fn from(handle: *mut c_void) -> @wxConnection { @wxConnection(handle) }
    pub fn null() -> @wxConnection { wxConnection::from(0 as *mut c_void) }
    
}

pub trait _wxConnection : _wxConnectionBase {
}

pub struct wxConnectionBase(*mut c_void);
impl _wxConnectionBase for wxConnectionBase {}
impl _wxObject for wxConnectionBase { fn handle(&self) -> *mut c_void { **self } }

impl wxConnectionBase {
    pub fn from(handle: *mut c_void) -> @wxConnectionBase { @wxConnectionBase(handle) }
    pub fn null() -> @wxConnectionBase { wxConnectionBase::from(0 as *mut c_void) }
    
}

pub trait _wxConnectionBase : _wxObject {
}

pub struct wxCountingOutputStream(*mut c_void);
impl _wxCountingOutputStream for wxCountingOutputStream {}
impl _wxOutputStream for wxCountingOutputStream {}
impl _wxStreamBase for wxCountingOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxCountingOutputStream {
    pub fn from(handle: *mut c_void) -> @wxCountingOutputStream { @wxCountingOutputStream(handle) }
    pub fn null() -> @wxCountingOutputStream { wxCountingOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxCountingOutputStream : _wxOutputStream {
}

pub struct wxCriticalSection(*mut c_void);
impl _wxCriticalSection for wxCriticalSection { fn handle(&self) -> *mut c_void { **self } }

impl wxCriticalSection {
    pub fn from(handle: *mut c_void) -> @wxCriticalSection { @wxCriticalSection(handle) }
    pub fn null() -> @wxCriticalSection { wxCriticalSection::from(0 as *mut c_void) }
    
}

pub trait _wxCriticalSection {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxCriticalSectionLocker(*mut c_void);
impl _wxCriticalSectionLocker for wxCriticalSectionLocker { fn handle(&self) -> *mut c_void { **self } }

impl wxCriticalSectionLocker {
    pub fn from(handle: *mut c_void) -> @wxCriticalSectionLocker { @wxCriticalSectionLocker(handle) }
    pub fn null() -> @wxCriticalSectionLocker { wxCriticalSectionLocker::from(0 as *mut c_void) }
    
}

pub trait _wxCriticalSectionLocker {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDDEClient(*mut c_void);
impl _wxDDEClient for wxDDEClient {}
impl _wxClientBase for wxDDEClient {}
impl _wxObject for wxDDEClient { fn handle(&self) -> *mut c_void { **self } }

impl wxDDEClient {
    pub fn from(handle: *mut c_void) -> @wxDDEClient { @wxDDEClient(handle) }
    pub fn null() -> @wxDDEClient { wxDDEClient::from(0 as *mut c_void) }
    
}

pub trait _wxDDEClient : _wxClientBase {
}

pub struct wxDDEConnection(*mut c_void);
impl _wxDDEConnection for wxDDEConnection {}
impl _wxConnectionBase for wxDDEConnection {}
impl _wxObject for wxDDEConnection { fn handle(&self) -> *mut c_void { **self } }

impl wxDDEConnection {
    pub fn from(handle: *mut c_void) -> @wxDDEConnection { @wxDDEConnection(handle) }
    pub fn null() -> @wxDDEConnection { wxDDEConnection::from(0 as *mut c_void) }
    
}

pub trait _wxDDEConnection : _wxConnectionBase {
}

pub struct wxDDEServer(*mut c_void);
impl _wxDDEServer for wxDDEServer {}
impl _wxServerBase for wxDDEServer {}
impl _wxObject for wxDDEServer { fn handle(&self) -> *mut c_void { **self } }

impl wxDDEServer {
    pub fn from(handle: *mut c_void) -> @wxDDEServer { @wxDDEServer(handle) }
    pub fn null() -> @wxDDEServer { wxDDEServer::from(0 as *mut c_void) }
    
}

pub trait _wxDDEServer : _wxServerBase {
}

pub struct wxDataInputStream(*mut c_void);
impl _wxDataInputStream for wxDataInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxDataInputStream {
    pub fn from(handle: *mut c_void) -> @wxDataInputStream { @wxDataInputStream(handle) }
    pub fn null() -> @wxDataInputStream { wxDataInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxDataInputStream {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDataOutputStream(*mut c_void);
impl _wxDataOutputStream for wxDataOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxDataOutputStream {
    pub fn from(handle: *mut c_void) -> @wxDataOutputStream { @wxDataOutputStream(handle) }
    pub fn null() -> @wxDataOutputStream { wxDataOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxDataOutputStream {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDateTime(*mut c_void);
impl _wxDateTime for wxDateTime { fn handle(&self) -> *mut c_void { **self } }

impl wxDateTime {
    pub fn from(handle: *mut c_void) -> @wxDateTime { @wxDateTime(handle) }
    pub fn null() -> @wxDateTime { wxDateTime::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxDateTime {
        unsafe { @wxDateTime(wxDateTime_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getAmString() -> ~str {
        unsafe { wxString { handle: wxDateTime_GetAmString() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getBeginDST<T: _wxDateTime>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetBeginDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCentury(year: c_int) -> c_int {
        unsafe { wxDateTime_GetCentury(year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCountry() -> c_int {
        unsafe { wxDateTime_GetCountry() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCurrentMonth(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentMonth(cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCurrentYear(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentYear(cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getEndDST<T: _wxDateTime>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getMonthName(month: c_int, flags: c_int) -> ~str {
        unsafe { wxString { handle: wxDateTime_GetMonthName(month, flags) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getPmString() -> ~str {
        unsafe { wxString { handle: wxDateTime_GetPmString() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> ~str {
        unsafe { wxString { handle: wxDateTime_GetWeekDayName(weekday, flags) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isDSTApplicable(year: c_int, country: c_int) -> c_int {
        unsafe { wxDateTime_IsDSTApplicable(year, country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isLeapYear(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_IsLeapYear(year, cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isWestEuropeanCountry(country: c_int) -> c_int {
        unsafe { wxDateTime_IsWestEuropeanCountry(country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setCountry(country: c_int) {
        unsafe { wxDateTime_SetCountry(country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn wxDateTime(hi_long: c_int, lo_long: c_int) -> *mut c_void {
        unsafe { wxDateTime_wxDateTime(hi_long, lo_long) }
    }
}

pub trait _wxDateTime {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addDate<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.handle(), _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTime<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.handle(), _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn format(&self, format: *mut c_void, tz: c_int) -> ~str {
        unsafe { wxString { handle: wxDateTime_Format(self.handle(), format, tz) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatDate(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatDate(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatISODate(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatISODate(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatISOTime(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatISOTime(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatTime(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatTime(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDay(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDay(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDayOfYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDayOfYear(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHour(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetHour(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastMonthDay<T: _wxDateTime>(&self, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastMonthDay(self.handle(), month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastWeekDay<T: _wxDateTime>(&self, weekday: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastWeekDay(self.handle(), weekday, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMillisecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMillisecond(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinute(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMinute(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMonth(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMonth(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetNextWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrevWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetPrevWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetSecond(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTicks(&self) -> time_t {
        unsafe { wxDateTime_GetTicks(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self, hi_long: *mut c_void, lo_long: *mut c_void) {
        unsafe { wxDateTime_GetValue(self.handle(), hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDay<T: _wxDateTime>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDay(self.handle(), weekday, n, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDayInSameWeek<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDayInSameWeek(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDayTZ(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekDayTZ(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekOfMonth(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfMonth(self.handle(), flags, tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekOfYear(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfYear(self.handle(), flags, tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetYear(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: &T, t2: &U) -> c_int {
        unsafe { wxDateTime_IsBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isDST(&self, country: c_int) -> c_int {
        unsafe { wxDateTime_IsDST(self.handle(), country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEarlierThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEarlierThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqualTo(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualTo(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqualUpTo<T: _wxDateTime>(&self, dt: &T, ts: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualUpTo(self.handle(), dt.handle(), ts) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isLaterThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsLaterThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSameDate<T: _wxDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameDate(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSameTime<T: _wxDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameTime(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStrictlyBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: &T, t2: &U) -> c_int {
        unsafe { wxDateTime_IsStrictlyBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isValid(&self) -> c_int {
        unsafe { wxDateTime_IsValid(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWorkDay(&self, country: c_int) -> c_int {
        unsafe { wxDateTime_IsWorkDay(self.handle(), country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_MakeGMT(self.handle(), noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_MakeTimezone(self.handle(), tz, noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn now(&self) {
        unsafe { wxDateTime_Now(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseDate(&self, date: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseDateTime(&self, datetime: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseDateTime(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseFormat(&self, date: *mut c_void, format: *mut c_void, dateDef: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseFormat(self.handle(), date, format, dateDef) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseRfc822Date(&self, date: *mut c_void) -> *mut c_void {
        unsafe { wxDateTime_ParseRfc822Date(self.handle(), date) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseTime<T: _wxTime>(&self, time: &T) -> *mut c_void {
        unsafe { wxDateTime_ParseTime(self.handle(), time.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetTime(&self) {
        unsafe { wxDateTime_ResetTime(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, day: c_int, month: c_int, year: c_int, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_Set(self.handle(), day, month, year, hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDay(&self, day: c_int) {
        unsafe { wxDateTime_SetDay(self.handle(), day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHour(&self, hour: c_int) {
        unsafe { wxDateTime_SetHour(self.handle(), hour) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMillisecond(&self, millisecond: c_int) {
        unsafe { wxDateTime_SetMillisecond(self.handle(), millisecond) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinute(&self, minute: c_int) {
        unsafe { wxDateTime_SetMinute(self.handle(), minute) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMonth(&self, month: c_int) {
        unsafe { wxDateTime_SetMonth(self.handle(), month) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSecond(&self, second: c_int) {
        unsafe { wxDateTime_SetSecond(self.handle(), second) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTime(&self, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_SetTime(self.handle(), hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToCurrent(&self) {
        unsafe { wxDateTime_SetToCurrent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToLastMonthDay(&self, month: c_int, year: c_int) {
        unsafe { wxDateTime_SetToLastMonthDay(self.handle(), month, year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int) -> c_int {
        unsafe { wxDateTime_SetToLastWeekDay(self.handle(), weekday, month, year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToNextWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToNextWeekDay(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToPrevWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToPrevWeekDay(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int) -> c_int {
        unsafe { wxDateTime_SetToWeekDay(self.handle(), weekday, n, month, year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToWeekDayInSameWeek(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToWeekDayInSameWeek(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setYear(&self, year: c_int) {
        unsafe { wxDateTime_SetYear(self.handle(), year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractDate<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractTime<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_ToGMT(self.handle(), noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_ToTimezone(self.handle(), tz, noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn today(&self) {
        unsafe { wxDateTime_Today(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn uNow(&self) {
        unsafe { wxDateTime_UNow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDateTime_Delete(self.handle()) }
    }
}

pub struct wxDebugContext(*mut c_void);
impl _wxDebugContext for wxDebugContext { fn handle(&self) -> *mut c_void { **self } }

impl wxDebugContext {
    pub fn from(handle: *mut c_void) -> @wxDebugContext { @wxDebugContext(handle) }
    pub fn null() -> @wxDebugContext { wxDebugContext::from(0 as *mut c_void) }
    
}

pub trait _wxDebugContext {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDirTraverser(*mut c_void);
impl _wxDirTraverser for wxDirTraverser { fn handle(&self) -> *mut c_void { **self } }

impl wxDirTraverser {
    pub fn from(handle: *mut c_void) -> @wxDirTraverser { @wxDirTraverser(handle) }
    pub fn null() -> @wxDirTraverser { wxDirTraverser::from(0 as *mut c_void) }
    
}

pub trait _wxDirTraverser {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDllLoader(*mut c_void);
impl _wxDllLoader for wxDllLoader { fn handle(&self) -> *mut c_void { **self } }

impl wxDllLoader {
    pub fn from(handle: *mut c_void) -> @wxDllLoader { @wxDllLoader(handle) }
    pub fn null() -> @wxDllLoader { wxDllLoader::from(0 as *mut c_void) }
    
}

pub trait _wxDllLoader {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDynamicLibrary(*mut c_void);
impl _wxDynamicLibrary for wxDynamicLibrary { fn handle(&self) -> *mut c_void { **self } }

impl wxDynamicLibrary {
    pub fn from(handle: *mut c_void) -> @wxDynamicLibrary { @wxDynamicLibrary(handle) }
    pub fn null() -> @wxDynamicLibrary { wxDynamicLibrary::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicLibrary {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxEncodingConverter(*mut c_void);
impl _wxEncodingConverter for wxEncodingConverter {}
impl _wxObject for wxEncodingConverter { fn handle(&self) -> *mut c_void { **self } }

impl wxEncodingConverter {
    pub fn from(handle: *mut c_void) -> @wxEncodingConverter { @wxEncodingConverter(handle) }
    pub fn null() -> @wxEncodingConverter { wxEncodingConverter::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxEncodingConverter {
        unsafe { @wxEncodingConverter(wxEncodingConverter_Create()) }
    }
}

pub trait _wxEncodingConverter : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn convert(&self, input: *mut c_void, output: *mut c_void) {
        unsafe { wxEncodingConverter_Convert(self.handle(), input, output) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllEquivalents<T: _wxList>(&self, enc: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.handle(), enc, _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPlatformEquivalents<T: _wxList>(&self, enc: c_int, platform: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.handle(), enc, platform, _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.handle(), input_enc, output_enc, method) }
    }
}

pub struct wxFFile(*mut c_void);
impl _wxFFile for wxFFile { fn handle(&self) -> *mut c_void { **self } }

impl wxFFile {
    pub fn from(handle: *mut c_void) -> @wxFFile { @wxFFile(handle) }
    pub fn null() -> @wxFFile { wxFFile::from(0 as *mut c_void) }
    
}

pub trait _wxFFile {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxFFileInputStream(*mut c_void);
impl _wxFFileInputStream for wxFFileInputStream {}
impl _wxInputStream for wxFFileInputStream {}
impl _wxStreamBase for wxFFileInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxFFileInputStream {
    pub fn from(handle: *mut c_void) -> @wxFFileInputStream { @wxFFileInputStream(handle) }
    pub fn null() -> @wxFFileInputStream { wxFFileInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFFileInputStream : _wxInputStream {
}

pub struct wxFFileOutputStream(*mut c_void);
impl _wxFFileOutputStream for wxFFileOutputStream {}
impl _wxOutputStream for wxFFileOutputStream {}
impl _wxStreamBase for wxFFileOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxFFileOutputStream {
    pub fn from(handle: *mut c_void) -> @wxFFileOutputStream { @wxFFileOutputStream(handle) }
    pub fn null() -> @wxFFileOutputStream { wxFFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFFileOutputStream : _wxOutputStream {
}

pub struct wxFSFile(*mut c_void);
impl _wxFSFile for wxFSFile {}
impl _wxObject for wxFSFile { fn handle(&self) -> *mut c_void { **self } }

impl wxFSFile {
    pub fn from(handle: *mut c_void) -> @wxFSFile { @wxFSFile(handle) }
    pub fn null() -> @wxFSFile { wxFSFile::from(0 as *mut c_void) }
    
}

pub trait _wxFSFile : _wxObject {
}

pub struct wxFileInputStream(*mut c_void);
impl _wxFileInputStream for wxFileInputStream {}
impl _wxInputStream for wxFileInputStream {}
impl _wxStreamBase for wxFileInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxFileInputStream {
    pub fn from(handle: *mut c_void) -> @wxFileInputStream { @wxFileInputStream(handle) }
    pub fn null() -> @wxFileInputStream { wxFileInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFileInputStream : _wxInputStream {
}

pub struct wxFileName(*mut c_void);
impl _wxFileName for wxFileName { fn handle(&self) -> *mut c_void { **self } }

impl wxFileName {
    pub fn from(handle: *mut c_void) -> @wxFileName { @wxFileName(handle) }
    pub fn null() -> @wxFileName { wxFileName::from(0 as *mut c_void) }
    
}

pub trait _wxFileName {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxFileOutputStream(*mut c_void);
impl _wxFileOutputStream for wxFileOutputStream {}
impl _wxOutputStream for wxFileOutputStream {}
impl _wxStreamBase for wxFileOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxFileOutputStream {
    pub fn from(handle: *mut c_void) -> @wxFileOutputStream { @wxFileOutputStream(handle) }
    pub fn null() -> @wxFileOutputStream { wxFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFileOutputStream : _wxOutputStream {
}

pub struct wxFileSystem(*mut c_void);
impl _wxFileSystem for wxFileSystem {}
impl _wxObject for wxFileSystem { fn handle(&self) -> *mut c_void { **self } }

impl wxFileSystem {
    pub fn from(handle: *mut c_void) -> @wxFileSystem { @wxFileSystem(handle) }
    pub fn null() -> @wxFileSystem { wxFileSystem::from(0 as *mut c_void) }
    
}

pub trait _wxFileSystem : _wxObject {
}

pub struct wxFileSystemHandler(*mut c_void);
impl _wxFileSystemHandler for wxFileSystemHandler {}
impl _wxObject for wxFileSystemHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxFileSystemHandler {
    pub fn from(handle: *mut c_void) -> @wxFileSystemHandler { @wxFileSystemHandler(handle) }
    pub fn null() -> @wxFileSystemHandler { wxFileSystemHandler::from(0 as *mut c_void) }
    
}

pub trait _wxFileSystemHandler : _wxObject {
}

pub struct wxFilterInputStream(*mut c_void);
impl _wxFilterInputStream for wxFilterInputStream {}
impl _wxInputStream for wxFilterInputStream {}
impl _wxStreamBase for wxFilterInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxFilterInputStream {
    pub fn from(handle: *mut c_void) -> @wxFilterInputStream { @wxFilterInputStream(handle) }
    pub fn null() -> @wxFilterInputStream { wxFilterInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFilterInputStream : _wxInputStream {
}

pub struct wxFilterOutputStream(*mut c_void);
impl _wxFilterOutputStream for wxFilterOutputStream {}
impl _wxOutputStream for wxFilterOutputStream {}
impl _wxStreamBase for wxFilterOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxFilterOutputStream {
    pub fn from(handle: *mut c_void) -> @wxFilterOutputStream { @wxFilterOutputStream(handle) }
    pub fn null() -> @wxFilterOutputStream { wxFilterOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFilterOutputStream : _wxOutputStream {
}

pub struct wxInputStream(*mut c_void);
impl _wxInputStream for wxInputStream {}
impl _wxStreamBase for wxInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxInputStream {
    pub fn from(handle: *mut c_void) -> @wxInputStream { @wxInputStream(handle) }
    pub fn null() -> @wxInputStream { wxInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxInputStream : _wxStreamBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn eof(&self) -> c_int {
        unsafe { wxInputStream_Eof(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getC(&self) -> int8_t {
        unsafe { wxInputStream_GetC(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastRead(&self) -> c_int {
        unsafe { wxInputStream_LastRead(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn peek(&self) -> int8_t {
        unsafe { wxInputStream_Peek(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn read(&self, buffer: *mut c_void, size: c_int) {
        unsafe { wxInputStream_Read(self.handle(), buffer, size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn seekI(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxInputStream_SeekI(self.handle(), pos, mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tell(&self) -> c_int {
        unsafe { wxInputStream_Tell(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ungetBuffer(&self, buffer: *mut c_void, size: c_int) -> c_int {
        unsafe { wxInputStream_UngetBuffer(self.handle(), buffer, size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ungetch(&self, c: int8_t) -> c_int {
        unsafe { wxInputStream_Ungetch(self.handle(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canRead(&self) -> c_int {
        unsafe { wxInputStream_CanRead(self.handle()) }
    }
}

pub struct wxList(*mut c_void);
impl _wxList for wxList {}
impl _wxObject for wxList { fn handle(&self) -> *mut c_void { **self } }

impl wxList {
    pub fn from(handle: *mut c_void) -> @wxList { @wxList(handle) }
    pub fn null() -> @wxList { wxList::from(0 as *mut c_void) }
    
}

pub trait _wxList : _wxObject {
}

pub struct wxLocale(*mut c_void);
impl _wxLocale for wxLocale { fn handle(&self) -> *mut c_void { **self } }

impl wxLocale {
    pub fn from(handle: *mut c_void) -> @wxLocale { @wxLocale(handle) }
    pub fn null() -> @wxLocale { wxLocale::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_name: c_int, _flags: c_int) -> @wxLocale {
        unsafe { @wxLocale(wxLocale_Create(_name, _flags)) }
    }
}

pub trait _wxLocale {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCatalog(&self, szDomain: *mut c_void) -> c_int {
        unsafe { wxLocale_AddCatalog(self.handle(), szDomain) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCatalogLookupPathPrefix(&self, prefix: *mut c_void) {
        unsafe { wxLocale_AddCatalogLookupPathPrefix(self.handle(), prefix) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxLocale_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLocale(&self) -> @wxLocale {
        unsafe { @wxLocale(wxLocale_GetLocale(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { handle: wxLocale_GetName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, szOrigString: *mut c_void, szDomain: *mut c_void) -> *mut int8_t {
        unsafe { wxLocale_GetString(self.handle(), szOrigString, szDomain) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isLoaded(&self, szDomain: *mut c_void) -> c_int {
        unsafe { wxLocale_IsLoaded(self.handle(), szDomain) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxLocale_IsOk(self.handle()) }
    }
}

pub struct wxLongLong(*mut c_void);
impl _wxLongLong for wxLongLong { fn handle(&self) -> *mut c_void { **self } }

impl wxLongLong {
    pub fn from(handle: *mut c_void) -> @wxLongLong { @wxLongLong(handle) }
    pub fn null() -> @wxLongLong { wxLongLong::from(0 as *mut c_void) }
    
}

pub trait _wxLongLong {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxMBConv(*mut c_void);
impl _wxMBConv for wxMBConv { fn handle(&self) -> *mut c_void { **self } }

impl wxMBConv {
    pub fn from(handle: *mut c_void) -> @wxMBConv { @wxMBConv(handle) }
    pub fn null() -> @wxMBConv { wxMBConv::from(0 as *mut c_void) }
    
}

pub trait _wxMBConv {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxMBConvUTF7(*mut c_void);
impl _wxMBConvUTF7 for wxMBConvUTF7 {}
impl _wxMBConv for wxMBConvUTF7 { fn handle(&self) -> *mut c_void { **self } }

impl wxMBConvUTF7 {
    pub fn from(handle: *mut c_void) -> @wxMBConvUTF7 { @wxMBConvUTF7(handle) }
    pub fn null() -> @wxMBConvUTF7 { wxMBConvUTF7::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvUTF7 : _wxMBConv {
}

pub struct wxMBConvUTF8(*mut c_void);
impl _wxMBConvUTF8 for wxMBConvUTF8 {}
impl _wxMBConv for wxMBConvUTF8 { fn handle(&self) -> *mut c_void { **self } }

impl wxMBConvUTF8 {
    pub fn from(handle: *mut c_void) -> @wxMBConvUTF8 { @wxMBConvUTF8(handle) }
    pub fn null() -> @wxMBConvUTF8 { wxMBConvUTF8::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvUTF8 : _wxMBConv {
}

pub struct wxMemoryFSHandler(*mut c_void);
impl _wxMemoryFSHandler for wxMemoryFSHandler {}
impl _wxFileSystemHandler for wxMemoryFSHandler {}
impl _wxObject for wxMemoryFSHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxMemoryFSHandler {
    pub fn from(handle: *mut c_void) -> @wxMemoryFSHandler { @wxMemoryFSHandler(handle) }
    pub fn null() -> @wxMemoryFSHandler { wxMemoryFSHandler::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryFSHandler : _wxFileSystemHandler {
}

pub struct wxMemoryInputStream(*mut c_void);
impl _wxMemoryInputStream for wxMemoryInputStream {}
impl _wxInputStream for wxMemoryInputStream {}
impl _wxStreamBase for wxMemoryInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxMemoryInputStream {
    pub fn from(handle: *mut c_void) -> @wxMemoryInputStream { @wxMemoryInputStream(handle) }
    pub fn null() -> @wxMemoryInputStream { wxMemoryInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryInputStream : _wxInputStream {
}

pub struct wxMemoryOutputStream(*mut c_void);
impl _wxMemoryOutputStream for wxMemoryOutputStream {}
impl _wxOutputStream for wxMemoryOutputStream {}
impl _wxStreamBase for wxMemoryOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxMemoryOutputStream {
    pub fn from(handle: *mut c_void) -> @wxMemoryOutputStream { @wxMemoryOutputStream(handle) }
    pub fn null() -> @wxMemoryOutputStream { wxMemoryOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryOutputStream : _wxOutputStream {
}

pub struct wxModule(*mut c_void);
impl _wxModule for wxModule {}
impl _wxObject for wxModule { fn handle(&self) -> *mut c_void { **self } }

impl wxModule {
    pub fn from(handle: *mut c_void) -> @wxModule { @wxModule(handle) }
    pub fn null() -> @wxModule { wxModule::from(0 as *mut c_void) }
    
}

pub trait _wxModule : _wxObject {
}

pub struct wxMutex(*mut c_void);
impl _wxMutex for wxMutex { fn handle(&self) -> *mut c_void { **self } }

impl wxMutex {
    pub fn from(handle: *mut c_void) -> @wxMutex { @wxMutex(handle) }
    pub fn null() -> @wxMutex { wxMutex::from(0 as *mut c_void) }
    
}

pub trait _wxMutex {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxMutexLocker(*mut c_void);
impl _wxMutexLocker for wxMutexLocker { fn handle(&self) -> *mut c_void { **self } }

impl wxMutexLocker {
    pub fn from(handle: *mut c_void) -> @wxMutexLocker { @wxMutexLocker(handle) }
    pub fn null() -> @wxMutexLocker { wxMutexLocker::from(0 as *mut c_void) }
    
}

pub trait _wxMutexLocker {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxNodeBase(*mut c_void);
impl _wxNodeBase for wxNodeBase { fn handle(&self) -> *mut c_void { **self } }

impl wxNodeBase {
    pub fn from(handle: *mut c_void) -> @wxNodeBase { @wxNodeBase(handle) }
    pub fn null() -> @wxNodeBase { wxNodeBase::from(0 as *mut c_void) }
    
}

pub trait _wxNodeBase {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxObject(*mut c_void);
impl _wxObject for wxObject { fn handle(&self) -> *mut c_void { **self } }

impl wxObject {
    pub fn from(handle: *mut c_void) -> @wxObject { @wxObject(handle) }
    pub fn null() -> @wxObject { wxObject::from(0 as *mut c_void) }
    
}

pub trait _wxObject {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { @wxClosure(wxObject_GetClientClosure(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientClosure<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxObject_SetClientClosure(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxObject_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClassInfo(&self) -> @wxClassInfo {
        unsafe { @wxClassInfo(wxObject_GetClassInfo(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isKindOf<T: _wxClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxObject_IsKindOf(self.handle(), classInfo.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isScrolledWindow(&self) -> c_int {
        unsafe { wxObject_IsScrolledWindow(self.handle()) }
    }
}

pub struct wxObjectRefData(*mut c_void);
impl _wxObjectRefData for wxObjectRefData { fn handle(&self) -> *mut c_void { **self } }

impl wxObjectRefData {
    pub fn from(handle: *mut c_void) -> @wxObjectRefData { @wxObjectRefData(handle) }
    pub fn null() -> @wxObjectRefData { wxObjectRefData::from(0 as *mut c_void) }
    
}

pub trait _wxObjectRefData {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxOutputStream(*mut c_void);
impl _wxOutputStream for wxOutputStream {}
impl _wxStreamBase for wxOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxOutputStream {
    pub fn from(handle: *mut c_void) -> @wxOutputStream { @wxOutputStream(handle) }
    pub fn null() -> @wxOutputStream { wxOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxOutputStream : _wxStreamBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastWrite(&self) -> c_int {
        unsafe { wxOutputStream_LastWrite(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn putC(&self, c: int8_t) {
        unsafe { wxOutputStream_PutC(self.handle(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn seek(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxOutputStream_Seek(self.handle(), pos, mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sync(&self) {
        unsafe { wxOutputStream_Sync(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tell(&self) -> c_int {
        unsafe { wxOutputStream_Tell(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn write(&self, buffer: *mut c_void, size: c_int) {
        unsafe { wxOutputStream_Write(self.handle(), buffer, size) }
    }
}

pub struct wxPathList(*mut c_void);
impl _wxPathList for wxPathList {}
impl _wxList for wxPathList {}
impl _wxObject for wxPathList { fn handle(&self) -> *mut c_void { **self } }

impl wxPathList {
    pub fn from(handle: *mut c_void) -> @wxPathList { @wxPathList(handle) }
    pub fn null() -> @wxPathList { wxPathList::from(0 as *mut c_void) }
    
}

pub trait _wxPathList : _wxList {
}

pub struct wxRegEx(*mut c_void);
impl _wxRegEx for wxRegEx { fn handle(&self) -> *mut c_void { **self } }

impl wxRegEx {
    pub fn from(handle: *mut c_void) -> @wxRegEx { @wxRegEx(handle) }
    pub fn null() -> @wxRegEx { wxRegEx::from(0 as *mut c_void) }
    
}

pub trait _wxRegEx {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxScopedArray(*mut c_void);
impl _wxScopedArray for wxScopedArray { fn handle(&self) -> *mut c_void { **self } }

impl wxScopedArray {
    pub fn from(handle: *mut c_void) -> @wxScopedArray { @wxScopedArray(handle) }
    pub fn null() -> @wxScopedArray { wxScopedArray::from(0 as *mut c_void) }
    
}

pub trait _wxScopedArray {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxScopedPtr(*mut c_void);
impl _wxScopedPtr for wxScopedPtr { fn handle(&self) -> *mut c_void { **self } }

impl wxScopedPtr {
    pub fn from(handle: *mut c_void) -> @wxScopedPtr { @wxScopedPtr(handle) }
    pub fn null() -> @wxScopedPtr { wxScopedPtr::from(0 as *mut c_void) }
    
}

pub trait _wxScopedPtr {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxSemaphore(*mut c_void);
impl _wxSemaphore for wxSemaphore { fn handle(&self) -> *mut c_void { **self } }

impl wxSemaphore {
    pub fn from(handle: *mut c_void) -> @wxSemaphore { @wxSemaphore(handle) }
    pub fn null() -> @wxSemaphore { wxSemaphore::from(0 as *mut c_void) }
    
}

pub trait _wxSemaphore {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxServer(*mut c_void);
impl _wxServer for wxServer {}
impl _wxServerBase for wxServer {}
impl _wxObject for wxServer { fn handle(&self) -> *mut c_void { **self } }

impl wxServer {
    pub fn from(handle: *mut c_void) -> @wxServer { @wxServer(handle) }
    pub fn null() -> @wxServer { wxServer::from(0 as *mut c_void) }
    
}

pub trait _wxServer : _wxServerBase {
}

pub struct wxServerBase(*mut c_void);
impl _wxServerBase for wxServerBase {}
impl _wxObject for wxServerBase { fn handle(&self) -> *mut c_void { **self } }

impl wxServerBase {
    pub fn from(handle: *mut c_void) -> @wxServerBase { @wxServerBase(handle) }
    pub fn null() -> @wxServerBase { wxServerBase::from(0 as *mut c_void) }
    
}

pub trait _wxServerBase : _wxObject {
}

pub struct wxSingleInstanceChecker(*mut c_void);
impl _wxSingleInstanceChecker for wxSingleInstanceChecker { fn handle(&self) -> *mut c_void { **self } }

impl wxSingleInstanceChecker {
    pub fn from(handle: *mut c_void) -> @wxSingleInstanceChecker { @wxSingleInstanceChecker(handle) }
    pub fn null() -> @wxSingleInstanceChecker { wxSingleInstanceChecker::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, name: &str, path: &str) -> c_int {
        let name = wxT(name);
        let path = wxT(path);
        unsafe { wxSingleInstanceChecker_Create(_obj, name.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxSingleInstanceChecker {
        unsafe { @wxSingleInstanceChecker(wxSingleInstanceChecker_CreateDefault()) }
    }
}

pub trait _wxSingleInstanceChecker {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAnotherRunning(&self) -> c_int {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.handle()) }
    }
}

pub struct wxStopWatch(*mut c_void);
impl _wxStopWatch for wxStopWatch { fn handle(&self) -> *mut c_void { **self } }

impl wxStopWatch {
    pub fn from(handle: *mut c_void) -> @wxStopWatch { @wxStopWatch(handle) }
    pub fn null() -> @wxStopWatch { wxStopWatch::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxStopWatch {
        unsafe { @wxStopWatch(wxStopWatch_Create()) }
    }
}

pub trait _wxStopWatch {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxStopWatch_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self, msec: c_int) {
        unsafe { wxStopWatch_Start(self.handle(), msec) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pause(&self) {
        unsafe { wxStopWatch_Pause(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxStopWatch_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn time(&self) -> c_int {
        unsafe { wxStopWatch_Time(self.handle()) }
    }
}

pub struct wxStreamBase(*mut c_void);
impl _wxStreamBase for wxStreamBase { fn handle(&self) -> *mut c_void { **self } }

impl wxStreamBase {
    pub fn from(handle: *mut c_void) -> @wxStreamBase { @wxStreamBase(handle) }
    pub fn null() -> @wxStreamBase { wxStreamBase::from(0 as *mut c_void) }
    
}

pub trait _wxStreamBase {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastError(&self) -> c_int {
        unsafe { wxStreamBase_GetLastError(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> c_int {
        unsafe { wxStreamBase_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxStreamBase_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxStreamBase_Delete(self.handle()) }
    }
}

pub struct wxStreamBuffer(*mut c_void);
impl _wxStreamBuffer for wxStreamBuffer { fn handle(&self) -> *mut c_void { **self } }

impl wxStreamBuffer {
    pub fn from(handle: *mut c_void) -> @wxStreamBuffer { @wxStreamBuffer(handle) }
    pub fn null() -> @wxStreamBuffer { wxStreamBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxStreamBuffer {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxStringBuffer(*mut c_void);
impl _wxStringBuffer for wxStringBuffer { fn handle(&self) -> *mut c_void { **self } }

impl wxStringBuffer {
    pub fn from(handle: *mut c_void) -> @wxStringBuffer { @wxStringBuffer(handle) }
    pub fn null() -> @wxStringBuffer { wxStringBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxStringBuffer {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxStringClientData(*mut c_void);
impl _wxStringClientData for wxStringClientData {}
impl _wxClientData for wxStringClientData { fn handle(&self) -> *mut c_void { **self } }

impl wxStringClientData {
    pub fn from(handle: *mut c_void) -> @wxStringClientData { @wxStringClientData(handle) }
    pub fn null() -> @wxStringClientData { wxStringClientData::from(0 as *mut c_void) }
    
}

pub trait _wxStringClientData : _wxClientData {
}

pub struct wxStringList(*mut c_void);
impl _wxStringList for wxStringList {}
impl _wxList for wxStringList {}
impl _wxObject for wxStringList { fn handle(&self) -> *mut c_void { **self } }

impl wxStringList {
    pub fn from(handle: *mut c_void) -> @wxStringList { @wxStringList(handle) }
    pub fn null() -> @wxStringList { wxStringList::from(0 as *mut c_void) }
    
}

pub trait _wxStringList : _wxList {
}

pub struct wxStringTokenizer(*mut c_void);
impl _wxStringTokenizer for wxStringTokenizer {}
impl _wxObject for wxStringTokenizer { fn handle(&self) -> *mut c_void { **self } }

impl wxStringTokenizer {
    pub fn from(handle: *mut c_void) -> @wxStringTokenizer { @wxStringTokenizer(handle) }
    pub fn null() -> @wxStringTokenizer { wxStringTokenizer::from(0 as *mut c_void) }
    
}

pub trait _wxStringTokenizer : _wxObject {
}

pub struct wxSystemOptions(*mut c_void);
impl _wxSystemOptions for wxSystemOptions {}
impl _wxObject for wxSystemOptions { fn handle(&self) -> *mut c_void { **self } }

impl wxSystemOptions {
    pub fn from(handle: *mut c_void) -> @wxSystemOptions { @wxSystemOptions(handle) }
    pub fn null() -> @wxSystemOptions { wxSystemOptions::from(0 as *mut c_void) }
    
}

pub trait _wxSystemOptions : _wxObject {
}

pub struct wxTempFile(*mut c_void);
impl _wxTempFile for wxTempFile { fn handle(&self) -> *mut c_void { **self } }

impl wxTempFile {
    pub fn from(handle: *mut c_void) -> @wxTempFile { @wxTempFile(handle) }
    pub fn null() -> @wxTempFile { wxTempFile::from(0 as *mut c_void) }
    
}

pub trait _wxTempFile {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxTextFile(*mut c_void);
impl _wxTextFile for wxTextFile { fn handle(&self) -> *mut c_void { **self } }

impl wxTextFile {
    pub fn from(handle: *mut c_void) -> @wxTextFile { @wxTextFile(handle) }
    pub fn null() -> @wxTextFile { wxTextFile::from(0 as *mut c_void) }
    
}

pub trait _wxTextFile {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxTextInputStream(*mut c_void);
impl _wxTextInputStream for wxTextInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxTextInputStream {
    pub fn from(handle: *mut c_void) -> @wxTextInputStream { @wxTextInputStream(handle) }
    pub fn null() -> @wxTextInputStream { wxTextInputStream::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream>(inputStream: &T, sep: &str) -> @wxTextInputStream {
        let sep = wxT(sep);
        unsafe { @wxTextInputStream(wxTextInputStream_Create(inputStream.handle(), sep.handle())) }
    }
}

pub trait _wxTextInputStream {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readLine(&self) -> ~str {
        unsafe { wxString { handle: wxTextInputStream_ReadLine(self.handle()) }.to_str() }
    }
}

pub struct wxTextOutputStream(*mut c_void);
impl _wxTextOutputStream for wxTextOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxTextOutputStream {
    pub fn from(handle: *mut c_void) -> @wxTextOutputStream { @wxTextOutputStream(handle) }
    pub fn null() -> @wxTextOutputStream { wxTextOutputStream::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxOutputStream>(outputStream: &T, mode: c_int) -> @wxTextOutputStream {
        unsafe { @wxTextOutputStream(wxTextOutputStream_Create(outputStream.handle(), mode)) }
    }
}

pub trait _wxTextOutputStream {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeString(&self, txt: &str) {
        let txt = wxT(txt);
        unsafe { wxTextOutputStream_WriteString(self.handle(), txt.handle()) }
    }
}

pub struct wxThread(*mut c_void);
impl _wxThread for wxThread { fn handle(&self) -> *mut c_void { **self } }

impl wxThread {
    pub fn from(handle: *mut c_void) -> @wxThread { @wxThread(handle) }
    pub fn null() -> @wxThread { wxThread::from(0 as *mut c_void) }
    
}

pub trait _wxThread {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxTime(*mut c_void);
impl _wxTime for wxTime {}
impl _wxObject for wxTime { fn handle(&self) -> *mut c_void { **self } }

impl wxTime {
    pub fn from(handle: *mut c_void) -> @wxTime { @wxTime(handle) }
    pub fn null() -> @wxTime { wxTime::from(0 as *mut c_void) }
    
}

pub trait _wxTime : _wxObject {
}

pub struct wxTimeSpan(*mut c_void);
impl _wxTimeSpan for wxTimeSpan { fn handle(&self) -> *mut c_void { **self } }

impl wxTimeSpan {
    pub fn from(handle: *mut c_void) -> @wxTimeSpan { @wxTimeSpan(handle) }
    pub fn null() -> @wxTimeSpan { wxTimeSpan::from(0 as *mut c_void) }
    
}

pub trait _wxTimeSpan {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxVariant(*mut c_void);
impl _wxVariant for wxVariant {}
impl _wxObject for wxVariant { fn handle(&self) -> *mut c_void { **self } }

impl wxVariant {
    pub fn from(handle: *mut c_void) -> @wxVariant { @wxVariant(handle) }
    pub fn null() -> @wxVariant { wxVariant::from(0 as *mut c_void) }
    
}

pub trait _wxVariant : _wxObject {
}

pub struct wxVariantData(*mut c_void);
impl _wxVariantData for wxVariantData {}
impl _wxObject for wxVariantData { fn handle(&self) -> *mut c_void { **self } }

impl wxVariantData {
    pub fn from(handle: *mut c_void) -> @wxVariantData { @wxVariantData(handle) }
    pub fn null() -> @wxVariantData { wxVariantData::from(0 as *mut c_void) }
    
}

pub trait _wxVariantData : _wxObject {
}

pub struct wxZipInputStream(*mut c_void);
impl _wxZipInputStream for wxZipInputStream {}
impl _wxInputStream for wxZipInputStream {}
impl _wxStreamBase for wxZipInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxZipInputStream {
    pub fn from(handle: *mut c_void) -> @wxZipInputStream { @wxZipInputStream(handle) }
    pub fn null() -> @wxZipInputStream { wxZipInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxZipInputStream : _wxInputStream {
}

pub struct wxZlibInputStream(*mut c_void);
impl _wxZlibInputStream for wxZlibInputStream {}
impl _wxFilterInputStream for wxZlibInputStream {}
impl _wxInputStream for wxZlibInputStream {}
impl _wxStreamBase for wxZlibInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxZlibInputStream {
    pub fn from(handle: *mut c_void) -> @wxZlibInputStream { @wxZlibInputStream(handle) }
    pub fn null() -> @wxZlibInputStream { wxZlibInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxZlibInputStream : _wxFilterInputStream {
}

pub struct wxZlibOutputStream(*mut c_void);
impl _wxZlibOutputStream for wxZlibOutputStream {}
impl _wxFilterOutputStream for wxZlibOutputStream {}
impl _wxOutputStream for wxZlibOutputStream {}
impl _wxStreamBase for wxZlibOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxZlibOutputStream {
    pub fn from(handle: *mut c_void) -> @wxZlibOutputStream { @wxZlibOutputStream(handle) }
    pub fn null() -> @wxZlibOutputStream { wxZlibOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxZlibOutputStream : _wxFilterOutputStream {
}

pub struct wxMemoryBuffer(*mut c_void);
impl _wxMemoryBuffer for wxMemoryBuffer { fn handle(&self) -> *mut c_void { **self } }

impl wxMemoryBuffer {
    pub fn from(handle: *mut c_void) -> @wxMemoryBuffer { @wxMemoryBuffer(handle) }
    pub fn null() -> @wxMemoryBuffer { wxMemoryBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryBuffer {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxFileConfig(*mut c_void);
impl _wxFileConfig for wxFileConfig {}
impl _wxConfigBase for wxFileConfig { fn handle(&self) -> *mut c_void { **self } }

impl wxFileConfig {
    pub fn from(handle: *mut c_void) -> @wxFileConfig { @wxFileConfig(handle) }
    pub fn null() -> @wxFileConfig { wxFileConfig::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream>(inp: &T) -> @wxFileConfig {
        unsafe { @wxFileConfig(wxFileConfig_Create(inp.handle())) }
    }
}

pub trait _wxFileConfig : _wxConfigBase {
}

