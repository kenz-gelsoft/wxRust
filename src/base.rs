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

pub fn wxT(s: &str) -> wxString {
    unsafe {
        s.to_c_str().with_ref(|c_str| {
            wxString { ptr: wxString_CreateUTF8(c_str as *mut c_void) }
        })
    }
}

pub struct wxString { ptr: *mut c_void }
impl Drop for wxString {
    fn drop(&mut self) {
        unsafe { wxString_Delete(self.ptr); }
    }
}
impl wxString {
    pub fn ptr(&self) -> *mut c_void { self.ptr }
    pub fn to_str(&self) -> ~str {
        unsafe {
            let charBuffer = wxString_GetUtf8(self.ptr);
            let utf8 = wxCharBuffer_DataUtf8(charBuffer);
            wxCharBuffer_Delete(charBuffer);
            str::raw::from_c_str(utf8)
        }
    }
}

pub struct wxRustClient { ptr: *mut c_void }
impl _wxRustClient for wxRustClient {}
impl _wxClient for wxRustClient {}
impl _wxClientBase for wxRustClient {}
impl _wxObject for wxRustClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustClient {
    pub fn from(ptr: *mut c_void) -> wxRustClient { wxRustClient { ptr: ptr } }
    pub fn null() -> wxRustClient { wxRustClient::from(0 as *mut c_void) }
    
}

pub trait _wxRustClient : _wxClient {
}

pub struct wxRustConnection { ptr: *mut c_void }
impl _wxRustConnection for wxRustConnection {}
impl _wxConnection for wxRustConnection {}
impl _wxConnectionBase for wxRustConnection {}
impl _wxObject for wxRustConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustConnection {
    pub fn from(ptr: *mut c_void) -> wxRustConnection { wxRustConnection { ptr: ptr } }
    pub fn null() -> wxRustConnection { wxRustConnection::from(0 as *mut c_void) }
    
}

pub trait _wxRustConnection : _wxConnection {
}

pub struct wxRustLocale { ptr: *mut c_void }
impl _wxRustLocale for wxRustLocale {}
impl _wxLocale for wxRustLocale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustLocale {
    pub fn from(ptr: *mut c_void) -> wxRustLocale { wxRustLocale { ptr: ptr } }
    pub fn null() -> wxRustLocale { wxRustLocale::from(0 as *mut c_void) }
    
}

pub trait _wxRustLocale : _wxLocale {
}

pub struct wxRustServer { ptr: *mut c_void }
impl _wxRustServer for wxRustServer {}
impl _wxServer for wxRustServer {}
impl _wxServerBase for wxRustServer {}
impl _wxObject for wxRustServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustServer {
    pub fn from(ptr: *mut c_void) -> wxRustServer { wxRustServer { ptr: ptr } }
    pub fn null() -> wxRustServer { wxRustServer::from(0 as *mut c_void) }
    
}

pub trait _wxRustServer : _wxServer {
}

pub struct wxArray { ptr: *mut c_void }
impl _wxArray for wxArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxArray {
    pub fn from(ptr: *mut c_void) -> wxArray { wxArray { ptr: ptr } }
    pub fn null() -> wxArray { wxArray::from(0 as *mut c_void) }
    
}

pub trait _wxArray {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxArrayString { ptr: *mut c_void }
impl _wxArrayString for wxArrayString {}
impl _wxArray for wxArrayString { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxArrayString {
    pub fn from(ptr: *mut c_void) -> wxArrayString { wxArrayString { ptr: ptr } }
    pub fn null() -> wxArrayString { wxArrayString::from(0 as *mut c_void) }
    
}

pub trait _wxArrayString : _wxArray {
}

pub struct wxBufferedInputStream { ptr: *mut c_void }
impl _wxBufferedInputStream for wxBufferedInputStream {}
impl _wxFilterInputStream for wxBufferedInputStream {}
impl _wxInputStream for wxBufferedInputStream {}
impl _wxStreamBase for wxBufferedInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBufferedInputStream {
    pub fn from(ptr: *mut c_void) -> wxBufferedInputStream { wxBufferedInputStream { ptr: ptr } }
    pub fn null() -> wxBufferedInputStream { wxBufferedInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxBufferedInputStream : _wxFilterInputStream {
}

pub struct wxBufferedOutputStream { ptr: *mut c_void }
impl _wxBufferedOutputStream for wxBufferedOutputStream {}
impl _wxFilterOutputStream for wxBufferedOutputStream {}
impl _wxOutputStream for wxBufferedOutputStream {}
impl _wxStreamBase for wxBufferedOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBufferedOutputStream {
    pub fn from(ptr: *mut c_void) -> wxBufferedOutputStream { wxBufferedOutputStream { ptr: ptr } }
    pub fn null() -> wxBufferedOutputStream { wxBufferedOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxBufferedOutputStream : _wxFilterOutputStream {
}

pub struct wxCSConv { ptr: *mut c_void }
impl _wxCSConv for wxCSConv {}
impl _wxMBConv for wxCSConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCSConv {
    pub fn from(ptr: *mut c_void) -> wxCSConv { wxCSConv { ptr: ptr } }
    pub fn null() -> wxCSConv { wxCSConv::from(0 as *mut c_void) }
    
}

pub trait _wxCSConv : _wxMBConv {
}

pub struct wxClassInfo { ptr: *mut c_void }
impl _wxClassInfo for wxClassInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClassInfo {
    pub fn from(ptr: *mut c_void) -> wxClassInfo { wxClassInfo { ptr: ptr } }
    pub fn null() -> wxClassInfo { wxClassInfo::from(0 as *mut c_void) }
    
    pub fn findClass(_txt: &str) -> wxClassInfo {
        let _txt = wxT(_txt);
        unsafe { wxClassInfo { ptr: wxClassInfo_FindClass(_txt.ptr()) } }
    }
}

pub trait _wxClassInfo {
    fn ptr(&self) -> *mut c_void;
    
    fn newClassByName(&self) -> *mut c_void {
        unsafe { wxClassInfo_CreateClassByName(self.ptr()) }
    }
    fn getClassName(&self) -> *mut c_void {
        unsafe { wxClassInfo_GetClassName(self.ptr()) }
    }
    fn isKindOf(&self, _name: &str) -> c_int {
        let _name = wxT(_name);
        unsafe { wxClassInfo_IsKindOf(self.ptr(), _name.ptr()) }
    }
    fn getBaseClassName1(&self) -> ~str {
        unsafe { wxString { ptr: wxClassInfo_GetBaseClassName1(self.ptr()) }.to_str() }
    }
    fn getBaseClassName2(&self) -> ~str {
        unsafe { wxString { ptr: wxClassInfo_GetBaseClassName2(self.ptr()) }.to_str() }
    }
    fn getClassNameEx(&self) -> ~str {
        unsafe { wxString { ptr: wxClassInfo_GetClassNameEx(self.ptr()) }.to_str() }
    }
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.ptr()) }
    }
    fn isKindOfEx<T: _wxClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxClassInfo_IsKindOfEx(self.ptr(), classInfo.ptr()) }
    }
}

pub struct wxClient { ptr: *mut c_void }
impl _wxClient for wxClient {}
impl _wxClientBase for wxClient {}
impl _wxObject for wxClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClient {
    pub fn from(ptr: *mut c_void) -> wxClient { wxClient { ptr: ptr } }
    pub fn null() -> wxClient { wxClient::from(0 as *mut c_void) }
    
}

pub trait _wxClient : _wxClientBase {
}

pub struct wxClientBase { ptr: *mut c_void }
impl _wxClientBase for wxClientBase {}
impl _wxObject for wxClientBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClientBase {
    pub fn from(ptr: *mut c_void) -> wxClientBase { wxClientBase { ptr: ptr } }
    pub fn null() -> wxClientBase { wxClientBase::from(0 as *mut c_void) }
    
}

pub trait _wxClientBase : _wxObject {
}

pub struct wxClientData { ptr: *mut c_void }
impl _wxClientData for wxClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClientData {
    pub fn from(ptr: *mut c_void) -> wxClientData { wxClientData { ptr: ptr } }
    pub fn null() -> wxClientData { wxClientData::from(0 as *mut c_void) }
    
}

pub trait _wxClientData {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxClientDataContainer { ptr: *mut c_void }
impl _wxClientDataContainer for wxClientDataContainer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClientDataContainer {
    pub fn from(ptr: *mut c_void) -> wxClientDataContainer { wxClientDataContainer { ptr: ptr } }
    pub fn null() -> wxClientDataContainer { wxClientDataContainer::from(0 as *mut c_void) }
    
}

pub trait _wxClientDataContainer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxClosure { ptr: *mut c_void }
impl _wxClosure for wxClosure {}
impl _wxObject for wxClosure { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClosure {
    pub fn from(ptr: *mut c_void) -> wxClosure { wxClosure { ptr: ptr } }
    pub fn null() -> wxClosure { wxClosure::from(0 as *mut c_void) }
    
    pub fn new(_fun_CEvent: *mut c_void, _data: *mut c_void) -> wxClosure {
        unsafe { wxClosure { ptr: wxClosure_Create(_fun_CEvent, _data) } }
    }
}

pub trait _wxClosure : _wxObject {
    fn getData(&self) -> *mut c_void {
        unsafe { wxClosure_GetData(self.ptr()) }
    }
}

pub struct wxCommandLineParser { ptr: *mut c_void }
impl _wxCommandLineParser for wxCommandLineParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCommandLineParser {
    pub fn from(ptr: *mut c_void) -> wxCommandLineParser { wxCommandLineParser { ptr: ptr } }
    pub fn null() -> wxCommandLineParser { wxCommandLineParser::from(0 as *mut c_void) }
    
}

pub trait _wxCommandLineParser {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxCondition { ptr: *mut c_void }
impl _wxCondition for wxCondition { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCondition {
    pub fn from(ptr: *mut c_void) -> wxCondition { wxCondition { ptr: ptr } }
    pub fn null() -> wxCondition { wxCondition::from(0 as *mut c_void) }
    
}

pub trait _wxCondition {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxConfigBase { ptr: *mut c_void }
impl _wxConfigBase for wxConfigBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxConfigBase {
    pub fn from(ptr: *mut c_void) -> wxConfigBase { wxConfigBase { ptr: ptr } }
    pub fn null() -> wxConfigBase { wxConfigBase::from(0 as *mut c_void) }
    
    pub fn new() -> wxConfigBase {
        unsafe { wxConfigBase { ptr: wxConfigBase_Create() } }
    }
    pub fn get() -> wxConfigBase {
        unsafe { wxConfigBase { ptr: wxConfigBase_Get() } }
    }
    pub fn set<T: _wxConfigBase>(self_: &T) {
        unsafe { wxConfigBase_Set(self_.ptr()) }
    }
}

pub trait _wxConfigBase {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self.ptr()) }
    }
    fn deleteAll(&self) -> c_int {
        unsafe { wxConfigBase_DeleteAll(self.ptr()) }
    }
    fn deleteEntry(&self, key: &str, bDeleteGroupIfEmpty: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_DeleteEntry(self.ptr(), key.ptr(), bDeleteGroupIfEmpty) }
    }
    fn deleteGroup(&self, key: &str) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_DeleteGroup(self.ptr(), key.ptr()) }
    }
    fn exists(&self, strName: &str) -> c_int {
        let strName = wxT(strName);
        unsafe { wxConfigBase_Exists(self.ptr(), strName.ptr()) }
    }
    fn expandEnvVars(&self, str: &str) -> ~str {
        let str = wxT(str);
        unsafe { wxString { ptr: wxConfigBase_ExpandEnvVars(self.ptr(), str.ptr()) }.to_str() }
    }
    fn flush(&self, bCurrentOnly: c_int) -> c_int {
        unsafe { wxConfigBase_Flush(self.ptr(), bCurrentOnly) }
    }
    fn getAppName(&self) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetAppName(self.ptr()) }.to_str() }
    }
    fn getEntryType(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxConfigBase_GetEntryType(self.ptr(), name.ptr()) }
    }
    fn getFirstEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetFirstEntry(self.ptr(), lIndex) }.to_str() }
    }
    fn getFirstGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetFirstGroup(self.ptr(), lIndex) }.to_str() }
    }
    fn getNextEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetNextEntry(self.ptr(), lIndex) }.to_str() }
    }
    fn getNextGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetNextGroup(self.ptr(), lIndex) }.to_str() }
    }
    fn getNumberOfEntries(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfEntries(self.ptr(), bRecursive) }
    }
    fn getNumberOfGroups(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfGroups(self.ptr(), bRecursive) }
    }
    fn getPath(&self) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetPath(self.ptr()) }.to_str() }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.ptr()) }
    }
    fn getVendorName(&self) -> ~str {
        unsafe { wxString { ptr: wxConfigBase_GetVendorName(self.ptr()) }.to_str() }
    }
    fn hasEntry(&self, strName: &str) -> c_int {
        let strName = wxT(strName);
        unsafe { wxConfigBase_HasEntry(self.ptr(), strName.ptr()) }
    }
    fn hasGroup(&self, strName: &str) -> c_int {
        let strName = wxT(strName);
        unsafe { wxConfigBase_HasGroup(self.ptr(), strName.ptr()) }
    }
    fn isExpandingEnvVars(&self) -> c_int {
        unsafe { wxConfigBase_IsExpandingEnvVars(self.ptr()) }
    }
    fn isRecordingDefaults(&self) -> c_int {
        unsafe { wxConfigBase_IsRecordingDefaults(self.ptr()) }
    }
    fn readBool(&self, key: &str, defVal: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadBool(self.ptr(), key.ptr(), defVal) }
    }
    fn readDouble(&self, key: &str, defVal: c_double) -> c_double {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadDouble(self.ptr(), key.ptr(), defVal) }
    }
    fn readInteger(&self, key: &str, defVal: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadInteger(self.ptr(), key.ptr(), defVal) }
    }
    fn readString(&self, key: &str, defVal: &str) -> ~str {
        let key = wxT(key);
        let defVal = wxT(defVal);
        unsafe { wxString { ptr: wxConfigBase_ReadString(self.ptr(), key.ptr(), defVal.ptr()) }.to_str() }
    }
    fn renameEntry(&self, oldName: &str, newName: &str) -> c_int {
        let oldName = wxT(oldName);
        let newName = wxT(newName);
        unsafe { wxConfigBase_RenameEntry(self.ptr(), oldName.ptr(), newName.ptr()) }
    }
    fn renameGroup(&self, oldName: &str, newName: &str) -> c_int {
        let oldName = wxT(oldName);
        let newName = wxT(newName);
        unsafe { wxConfigBase_RenameGroup(self.ptr(), oldName.ptr(), newName.ptr()) }
    }
    fn setAppName(&self, appName: &str) {
        let appName = wxT(appName);
        unsafe { wxConfigBase_SetAppName(self.ptr(), appName.ptr()) }
    }
    fn setExpandEnvVars(&self, bDoIt: c_int) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.ptr(), bDoIt) }
    }
    fn setPath(&self, strPath: &str) {
        let strPath = wxT(strPath);
        unsafe { wxConfigBase_SetPath(self.ptr(), strPath.ptr()) }
    }
    fn setRecordDefaults(&self, bDoIt: c_int) {
        unsafe { wxConfigBase_SetRecordDefaults(self.ptr(), bDoIt) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxConfigBase_SetStyle(self.ptr(), style) }
    }
    fn setVendorName(&self, vendorName: &str) {
        let vendorName = wxT(vendorName);
        unsafe { wxConfigBase_SetVendorName(self.ptr(), vendorName.ptr()) }
    }
    fn writeBool(&self, key: &str, value: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteBool(self.ptr(), key.ptr(), value) }
    }
    fn writeDouble(&self, key: &str, value: c_double) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteDouble(self.ptr(), key.ptr(), value) }
    }
    fn writeInteger(&self, key: &str, value: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteInteger(self.ptr(), key.ptr(), value) }
    }
    fn writeLong(&self, key: &str, value: c_long) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteLong(self.ptr(), key.ptr(), value) }
    }
    fn writeString(&self, key: &str, value: &str) -> c_int {
        let key = wxT(key);
        let value = wxT(value);
        unsafe { wxConfigBase_WriteString(self.ptr(), key.ptr(), value.ptr()) }
    }
}

pub struct wxConnection { ptr: *mut c_void }
impl _wxConnection for wxConnection {}
impl _wxConnectionBase for wxConnection {}
impl _wxObject for wxConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxConnection {
    pub fn from(ptr: *mut c_void) -> wxConnection { wxConnection { ptr: ptr } }
    pub fn null() -> wxConnection { wxConnection::from(0 as *mut c_void) }
    
}

pub trait _wxConnection : _wxConnectionBase {
}

pub struct wxConnectionBase { ptr: *mut c_void }
impl _wxConnectionBase for wxConnectionBase {}
impl _wxObject for wxConnectionBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxConnectionBase {
    pub fn from(ptr: *mut c_void) -> wxConnectionBase { wxConnectionBase { ptr: ptr } }
    pub fn null() -> wxConnectionBase { wxConnectionBase::from(0 as *mut c_void) }
    
}

pub trait _wxConnectionBase : _wxObject {
}

pub struct wxCountingOutputStream { ptr: *mut c_void }
impl _wxCountingOutputStream for wxCountingOutputStream {}
impl _wxOutputStream for wxCountingOutputStream {}
impl _wxStreamBase for wxCountingOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCountingOutputStream {
    pub fn from(ptr: *mut c_void) -> wxCountingOutputStream { wxCountingOutputStream { ptr: ptr } }
    pub fn null() -> wxCountingOutputStream { wxCountingOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxCountingOutputStream : _wxOutputStream {
}

pub struct wxCriticalSection { ptr: *mut c_void }
impl _wxCriticalSection for wxCriticalSection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCriticalSection {
    pub fn from(ptr: *mut c_void) -> wxCriticalSection { wxCriticalSection { ptr: ptr } }
    pub fn null() -> wxCriticalSection { wxCriticalSection::from(0 as *mut c_void) }
    
}

pub trait _wxCriticalSection {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxCriticalSectionLocker { ptr: *mut c_void }
impl _wxCriticalSectionLocker for wxCriticalSectionLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCriticalSectionLocker {
    pub fn from(ptr: *mut c_void) -> wxCriticalSectionLocker { wxCriticalSectionLocker { ptr: ptr } }
    pub fn null() -> wxCriticalSectionLocker { wxCriticalSectionLocker::from(0 as *mut c_void) }
    
}

pub trait _wxCriticalSectionLocker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDDEClient { ptr: *mut c_void }
impl _wxDDEClient for wxDDEClient {}
impl _wxClientBase for wxDDEClient {}
impl _wxObject for wxDDEClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDDEClient {
    pub fn from(ptr: *mut c_void) -> wxDDEClient { wxDDEClient { ptr: ptr } }
    pub fn null() -> wxDDEClient { wxDDEClient::from(0 as *mut c_void) }
    
}

pub trait _wxDDEClient : _wxClientBase {
}

pub struct wxDDEConnection { ptr: *mut c_void }
impl _wxDDEConnection for wxDDEConnection {}
impl _wxConnectionBase for wxDDEConnection {}
impl _wxObject for wxDDEConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDDEConnection {
    pub fn from(ptr: *mut c_void) -> wxDDEConnection { wxDDEConnection { ptr: ptr } }
    pub fn null() -> wxDDEConnection { wxDDEConnection::from(0 as *mut c_void) }
    
}

pub trait _wxDDEConnection : _wxConnectionBase {
}

pub struct wxDDEServer { ptr: *mut c_void }
impl _wxDDEServer for wxDDEServer {}
impl _wxServerBase for wxDDEServer {}
impl _wxObject for wxDDEServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDDEServer {
    pub fn from(ptr: *mut c_void) -> wxDDEServer { wxDDEServer { ptr: ptr } }
    pub fn null() -> wxDDEServer { wxDDEServer::from(0 as *mut c_void) }
    
}

pub trait _wxDDEServer : _wxServerBase {
}

pub struct wxDataInputStream { ptr: *mut c_void }
impl _wxDataInputStream for wxDataInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDataInputStream {
    pub fn from(ptr: *mut c_void) -> wxDataInputStream { wxDataInputStream { ptr: ptr } }
    pub fn null() -> wxDataInputStream { wxDataInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxDataInputStream {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDataOutputStream { ptr: *mut c_void }
impl _wxDataOutputStream for wxDataOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDataOutputStream {
    pub fn from(ptr: *mut c_void) -> wxDataOutputStream { wxDataOutputStream { ptr: ptr } }
    pub fn null() -> wxDataOutputStream { wxDataOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxDataOutputStream {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDateTime { ptr: *mut c_void }
impl _wxDateTime for wxDateTime { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDateTime {
    pub fn from(ptr: *mut c_void) -> wxDateTime { wxDateTime { ptr: ptr } }
    pub fn null() -> wxDateTime { wxDateTime::from(0 as *mut c_void) }
    
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    pub fn new() -> wxDateTime {
        unsafe { wxDateTime { ptr: wxDateTime_Create() } }
    }
    pub fn getAmString() -> ~str {
        unsafe { wxString { ptr: wxDateTime_GetAmString() }.to_str() }
    }
    pub fn getBeginDST<T: _wxDateTime>(year: c_int, country: c_int, dt: &T) {
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
    pub fn getEndDST<T: _wxDateTime>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.ptr()) }
    }
    pub fn getMonthName(month: c_int, flags: c_int) -> ~str {
        unsafe { wxString { ptr: wxDateTime_GetMonthName(month, flags) }.to_str() }
    }
    pub fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    pub fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    pub fn getPmString() -> ~str {
        unsafe { wxString { ptr: wxDateTime_GetPmString() }.to_str() }
    }
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> ~str {
        unsafe { wxString { ptr: wxDateTime_GetWeekDayName(weekday, flags) }.to_str() }
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

pub trait _wxDateTime {
    fn ptr(&self) -> *mut c_void;
    
    fn addDate<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.ptr(), _yrs, _mnt, _wek, _day) }
    }
    fn addTime<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddTime(self.ptr(), diff, _ref.ptr()) }
    }
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.ptr(), _hrs, _min, _sec, _mls) }
    }
    fn format(&self, format: *mut c_void, tz: c_int) -> ~str {
        unsafe { wxString { ptr: wxDateTime_Format(self.ptr(), format, tz) }.to_str() }
    }
    fn formatDate(&self) -> ~str {
        unsafe { wxString { ptr: wxDateTime_FormatDate(self.ptr()) }.to_str() }
    }
    fn formatISODate(&self) -> ~str {
        unsafe { wxString { ptr: wxDateTime_FormatISODate(self.ptr()) }.to_str() }
    }
    fn formatISOTime(&self) -> ~str {
        unsafe { wxString { ptr: wxDateTime_FormatISOTime(self.ptr()) }.to_str() }
    }
    fn formatTime(&self) -> ~str {
        unsafe { wxString { ptr: wxDateTime_FormatTime(self.ptr()) }.to_str() }
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
    fn getLastMonthDay<T: _wxDateTime>(&self, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastMonthDay(self.ptr(), month, year, _ref.ptr()) }
    }
    fn getLastWeekDay<T: _wxDateTime>(&self, weekday: c_int, month: c_int, year: c_int, _ref: &T) {
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
    fn getNextWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetNextWeekDay(self.ptr(), weekday, _ref.ptr()) }
    }
    fn getPrevWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
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
    fn getWeekDay<T: _wxDateTime>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDay(self.ptr(), weekday, n, month, year, _ref.ptr()) }
    }
    fn getWeekDayInSameWeek<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
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
    fn isBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: &T, t2: &U) -> c_int {
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
    fn isEqualUpTo<T: _wxDateTime>(&self, dt: &T, ts: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualUpTo(self.ptr(), dt.ptr(), ts) }
    }
    fn isLaterThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsLaterThan(self.ptr(), datetime) }
    }
    fn isSameDate<T: _wxDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameDate(self.ptr(), dt.ptr()) }
    }
    fn isSameTime<T: _wxDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameTime(self.ptr(), dt.ptr()) }
    }
    fn isStrictlyBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: &T, t2: &U) -> c_int {
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
    fn parseTime<T: _wxTime>(&self, time: &T) -> *mut c_void {
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
    fn subtractDate<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn subtractTime<T: _wxDateTime>(&self, diff: *mut c_void, _ref: &T) {
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

pub struct wxDebugContext { ptr: *mut c_void }
impl _wxDebugContext for wxDebugContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDebugContext {
    pub fn from(ptr: *mut c_void) -> wxDebugContext { wxDebugContext { ptr: ptr } }
    pub fn null() -> wxDebugContext { wxDebugContext::from(0 as *mut c_void) }
    
}

pub trait _wxDebugContext {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDirTraverser { ptr: *mut c_void }
impl _wxDirTraverser for wxDirTraverser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDirTraverser {
    pub fn from(ptr: *mut c_void) -> wxDirTraverser { wxDirTraverser { ptr: ptr } }
    pub fn null() -> wxDirTraverser { wxDirTraverser::from(0 as *mut c_void) }
    
}

pub trait _wxDirTraverser {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDllLoader { ptr: *mut c_void }
impl _wxDllLoader for wxDllLoader { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDllLoader {
    pub fn from(ptr: *mut c_void) -> wxDllLoader { wxDllLoader { ptr: ptr } }
    pub fn null() -> wxDllLoader { wxDllLoader::from(0 as *mut c_void) }
    
}

pub trait _wxDllLoader {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDynamicLibrary { ptr: *mut c_void }
impl _wxDynamicLibrary for wxDynamicLibrary { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDynamicLibrary {
    pub fn from(ptr: *mut c_void) -> wxDynamicLibrary { wxDynamicLibrary { ptr: ptr } }
    pub fn null() -> wxDynamicLibrary { wxDynamicLibrary::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicLibrary {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxEncodingConverter { ptr: *mut c_void }
impl _wxEncodingConverter for wxEncodingConverter {}
impl _wxObject for wxEncodingConverter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxEncodingConverter {
    pub fn from(ptr: *mut c_void) -> wxEncodingConverter { wxEncodingConverter { ptr: ptr } }
    pub fn null() -> wxEncodingConverter { wxEncodingConverter::from(0 as *mut c_void) }
    
    pub fn new() -> wxEncodingConverter {
        unsafe { wxEncodingConverter { ptr: wxEncodingConverter_Create() } }
    }
}

pub trait _wxEncodingConverter : _wxObject {
    fn convert(&self, input: *mut c_void, output: *mut c_void) {
        unsafe { wxEncodingConverter_Convert(self.ptr(), input, output) }
    }
    fn getAllEquivalents<T: _wxList>(&self, enc: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.ptr(), enc, _lst.ptr()) }
    }
    fn getPlatformEquivalents<T: _wxList>(&self, enc: c_int, platform: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.ptr(), enc, platform, _lst.ptr()) }
    }
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.ptr(), input_enc, output_enc, method) }
    }
}

pub struct wxFFile { ptr: *mut c_void }
impl _wxFFile for wxFFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFFile {
    pub fn from(ptr: *mut c_void) -> wxFFile { wxFFile { ptr: ptr } }
    pub fn null() -> wxFFile { wxFFile::from(0 as *mut c_void) }
    
}

pub trait _wxFFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxFFileInputStream { ptr: *mut c_void }
impl _wxFFileInputStream for wxFFileInputStream {}
impl _wxInputStream for wxFFileInputStream {}
impl _wxStreamBase for wxFFileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFFileInputStream {
    pub fn from(ptr: *mut c_void) -> wxFFileInputStream { wxFFileInputStream { ptr: ptr } }
    pub fn null() -> wxFFileInputStream { wxFFileInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFFileInputStream : _wxInputStream {
}

pub struct wxFFileOutputStream { ptr: *mut c_void }
impl _wxFFileOutputStream for wxFFileOutputStream {}
impl _wxOutputStream for wxFFileOutputStream {}
impl _wxStreamBase for wxFFileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFFileOutputStream {
    pub fn from(ptr: *mut c_void) -> wxFFileOutputStream { wxFFileOutputStream { ptr: ptr } }
    pub fn null() -> wxFFileOutputStream { wxFFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFFileOutputStream : _wxOutputStream {
}

pub struct wxFSFile { ptr: *mut c_void }
impl _wxFSFile for wxFSFile {}
impl _wxObject for wxFSFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFSFile {
    pub fn from(ptr: *mut c_void) -> wxFSFile { wxFSFile { ptr: ptr } }
    pub fn null() -> wxFSFile { wxFSFile::from(0 as *mut c_void) }
    
}

pub trait _wxFSFile : _wxObject {
}

pub struct wxFileInputStream { ptr: *mut c_void }
impl _wxFileInputStream for wxFileInputStream {}
impl _wxInputStream for wxFileInputStream {}
impl _wxStreamBase for wxFileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileInputStream {
    pub fn from(ptr: *mut c_void) -> wxFileInputStream { wxFileInputStream { ptr: ptr } }
    pub fn null() -> wxFileInputStream { wxFileInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFileInputStream : _wxInputStream {
}

pub struct wxFileName { ptr: *mut c_void }
impl _wxFileName for wxFileName { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileName {
    pub fn from(ptr: *mut c_void) -> wxFileName { wxFileName { ptr: ptr } }
    pub fn null() -> wxFileName { wxFileName::from(0 as *mut c_void) }
    
}

pub trait _wxFileName {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxFileOutputStream { ptr: *mut c_void }
impl _wxFileOutputStream for wxFileOutputStream {}
impl _wxOutputStream for wxFileOutputStream {}
impl _wxStreamBase for wxFileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileOutputStream {
    pub fn from(ptr: *mut c_void) -> wxFileOutputStream { wxFileOutputStream { ptr: ptr } }
    pub fn null() -> wxFileOutputStream { wxFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFileOutputStream : _wxOutputStream {
}

pub struct wxFileSystem { ptr: *mut c_void }
impl _wxFileSystem for wxFileSystem {}
impl _wxObject for wxFileSystem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileSystem {
    pub fn from(ptr: *mut c_void) -> wxFileSystem { wxFileSystem { ptr: ptr } }
    pub fn null() -> wxFileSystem { wxFileSystem::from(0 as *mut c_void) }
    
}

pub trait _wxFileSystem : _wxObject {
}

pub struct wxFileSystemHandler { ptr: *mut c_void }
impl _wxFileSystemHandler for wxFileSystemHandler {}
impl _wxObject for wxFileSystemHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileSystemHandler {
    pub fn from(ptr: *mut c_void) -> wxFileSystemHandler { wxFileSystemHandler { ptr: ptr } }
    pub fn null() -> wxFileSystemHandler { wxFileSystemHandler::from(0 as *mut c_void) }
    
}

pub trait _wxFileSystemHandler : _wxObject {
}

pub struct wxFilterInputStream { ptr: *mut c_void }
impl _wxFilterInputStream for wxFilterInputStream {}
impl _wxInputStream for wxFilterInputStream {}
impl _wxStreamBase for wxFilterInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFilterInputStream {
    pub fn from(ptr: *mut c_void) -> wxFilterInputStream { wxFilterInputStream { ptr: ptr } }
    pub fn null() -> wxFilterInputStream { wxFilterInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFilterInputStream : _wxInputStream {
}

pub struct wxFilterOutputStream { ptr: *mut c_void }
impl _wxFilterOutputStream for wxFilterOutputStream {}
impl _wxOutputStream for wxFilterOutputStream {}
impl _wxStreamBase for wxFilterOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFilterOutputStream {
    pub fn from(ptr: *mut c_void) -> wxFilterOutputStream { wxFilterOutputStream { ptr: ptr } }
    pub fn null() -> wxFilterOutputStream { wxFilterOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxFilterOutputStream : _wxOutputStream {
}

pub struct wxInputStream { ptr: *mut c_void }
impl _wxInputStream for wxInputStream {}
impl _wxStreamBase for wxInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxInputStream {
    pub fn from(ptr: *mut c_void) -> wxInputStream { wxInputStream { ptr: ptr } }
    pub fn null() -> wxInputStream { wxInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxInputStream : _wxStreamBase {
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

pub struct wxList { ptr: *mut c_void }
impl _wxList for wxList {}
impl _wxObject for wxList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxList {
    pub fn from(ptr: *mut c_void) -> wxList { wxList { ptr: ptr } }
    pub fn null() -> wxList { wxList::from(0 as *mut c_void) }
    
}

pub trait _wxList : _wxObject {
}

pub struct wxLocale { ptr: *mut c_void }
impl _wxLocale for wxLocale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLocale {
    pub fn from(ptr: *mut c_void) -> wxLocale { wxLocale { ptr: ptr } }
    pub fn null() -> wxLocale { wxLocale::from(0 as *mut c_void) }
    
    pub fn new(_name: c_int, _flags: c_int) -> wxLocale {
        unsafe { wxLocale { ptr: wxLocale_Create(_name, _flags) } }
    }
}

pub trait _wxLocale {
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
    fn getLocale(&self) -> wxLocale {
        unsafe { wxLocale { ptr: wxLocale_GetLocale(self.ptr()) } }
    }
    fn getName(&self) -> ~str {
        unsafe { wxString { ptr: wxLocale_GetName(self.ptr()) }.to_str() }
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

pub struct wxLongLong { ptr: *mut c_void }
impl _wxLongLong for wxLongLong { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLongLong {
    pub fn from(ptr: *mut c_void) -> wxLongLong { wxLongLong { ptr: ptr } }
    pub fn null() -> wxLongLong { wxLongLong::from(0 as *mut c_void) }
    
}

pub trait _wxLongLong {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxMBConv { ptr: *mut c_void }
impl _wxMBConv for wxMBConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMBConv {
    pub fn from(ptr: *mut c_void) -> wxMBConv { wxMBConv { ptr: ptr } }
    pub fn null() -> wxMBConv { wxMBConv::from(0 as *mut c_void) }
    
}

pub trait _wxMBConv {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxMBConvUTF7 { ptr: *mut c_void }
impl _wxMBConvUTF7 for wxMBConvUTF7 {}
impl _wxMBConv for wxMBConvUTF7 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMBConvUTF7 {
    pub fn from(ptr: *mut c_void) -> wxMBConvUTF7 { wxMBConvUTF7 { ptr: ptr } }
    pub fn null() -> wxMBConvUTF7 { wxMBConvUTF7::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvUTF7 : _wxMBConv {
}

pub struct wxMBConvUTF8 { ptr: *mut c_void }
impl _wxMBConvUTF8 for wxMBConvUTF8 {}
impl _wxMBConv for wxMBConvUTF8 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMBConvUTF8 {
    pub fn from(ptr: *mut c_void) -> wxMBConvUTF8 { wxMBConvUTF8 { ptr: ptr } }
    pub fn null() -> wxMBConvUTF8 { wxMBConvUTF8::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvUTF8 : _wxMBConv {
}

pub struct wxMemoryFSHandler { ptr: *mut c_void }
impl _wxMemoryFSHandler for wxMemoryFSHandler {}
impl _wxFileSystemHandler for wxMemoryFSHandler {}
impl _wxObject for wxMemoryFSHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMemoryFSHandler {
    pub fn from(ptr: *mut c_void) -> wxMemoryFSHandler { wxMemoryFSHandler { ptr: ptr } }
    pub fn null() -> wxMemoryFSHandler { wxMemoryFSHandler::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryFSHandler : _wxFileSystemHandler {
}

pub struct wxMemoryInputStream { ptr: *mut c_void }
impl _wxMemoryInputStream for wxMemoryInputStream {}
impl _wxInputStream for wxMemoryInputStream {}
impl _wxStreamBase for wxMemoryInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMemoryInputStream {
    pub fn from(ptr: *mut c_void) -> wxMemoryInputStream { wxMemoryInputStream { ptr: ptr } }
    pub fn null() -> wxMemoryInputStream { wxMemoryInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryInputStream : _wxInputStream {
}

pub struct wxMemoryOutputStream { ptr: *mut c_void }
impl _wxMemoryOutputStream for wxMemoryOutputStream {}
impl _wxOutputStream for wxMemoryOutputStream {}
impl _wxStreamBase for wxMemoryOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMemoryOutputStream {
    pub fn from(ptr: *mut c_void) -> wxMemoryOutputStream { wxMemoryOutputStream { ptr: ptr } }
    pub fn null() -> wxMemoryOutputStream { wxMemoryOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryOutputStream : _wxOutputStream {
}

pub struct wxModule { ptr: *mut c_void }
impl _wxModule for wxModule {}
impl _wxObject for wxModule { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxModule {
    pub fn from(ptr: *mut c_void) -> wxModule { wxModule { ptr: ptr } }
    pub fn null() -> wxModule { wxModule::from(0 as *mut c_void) }
    
}

pub trait _wxModule : _wxObject {
}

pub struct wxMutex { ptr: *mut c_void }
impl _wxMutex for wxMutex { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMutex {
    pub fn from(ptr: *mut c_void) -> wxMutex { wxMutex { ptr: ptr } }
    pub fn null() -> wxMutex { wxMutex::from(0 as *mut c_void) }
    
}

pub trait _wxMutex {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxMutexLocker { ptr: *mut c_void }
impl _wxMutexLocker for wxMutexLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMutexLocker {
    pub fn from(ptr: *mut c_void) -> wxMutexLocker { wxMutexLocker { ptr: ptr } }
    pub fn null() -> wxMutexLocker { wxMutexLocker::from(0 as *mut c_void) }
    
}

pub trait _wxMutexLocker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxNodeBase { ptr: *mut c_void }
impl _wxNodeBase for wxNodeBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxNodeBase {
    pub fn from(ptr: *mut c_void) -> wxNodeBase { wxNodeBase { ptr: ptr } }
    pub fn null() -> wxNodeBase { wxNodeBase::from(0 as *mut c_void) }
    
}

pub trait _wxNodeBase {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxObject { ptr: *mut c_void }
impl _wxObject for wxObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxObject {
    pub fn from(ptr: *mut c_void) -> wxObject { wxObject { ptr: ptr } }
    pub fn null() -> wxObject { wxObject::from(0 as *mut c_void) }
    
}

pub trait _wxObject {
    fn ptr(&self) -> *mut c_void;
    
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.ptr()) }
    }
    fn getClientClosure(&self) -> wxClosure {
        unsafe { wxClosure { ptr: wxObject_GetClientClosure(self.ptr()) } }
    }
    fn setClientClosure<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxObject_SetClientClosure(self.ptr(), closure.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxObject_Delete(self.ptr()) }
    }
    fn getClassInfo(&self) -> wxClassInfo {
        unsafe { wxClassInfo { ptr: wxObject_GetClassInfo(self.ptr()) } }
    }
    fn isKindOf<T: _wxClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxObject_IsKindOf(self.ptr(), classInfo.ptr()) }
    }
    fn isScrolledWindow(&self) -> c_int {
        unsafe { wxObject_IsScrolledWindow(self.ptr()) }
    }
}

pub struct wxObjectRefData { ptr: *mut c_void }
impl _wxObjectRefData for wxObjectRefData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxObjectRefData {
    pub fn from(ptr: *mut c_void) -> wxObjectRefData { wxObjectRefData { ptr: ptr } }
    pub fn null() -> wxObjectRefData { wxObjectRefData::from(0 as *mut c_void) }
    
}

pub trait _wxObjectRefData {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxOutputStream { ptr: *mut c_void }
impl _wxOutputStream for wxOutputStream {}
impl _wxStreamBase for wxOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxOutputStream {
    pub fn from(ptr: *mut c_void) -> wxOutputStream { wxOutputStream { ptr: ptr } }
    pub fn null() -> wxOutputStream { wxOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxOutputStream : _wxStreamBase {
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

pub struct wxPathList { ptr: *mut c_void }
impl _wxPathList for wxPathList {}
impl _wxList for wxPathList {}
impl _wxObject for wxPathList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPathList {
    pub fn from(ptr: *mut c_void) -> wxPathList { wxPathList { ptr: ptr } }
    pub fn null() -> wxPathList { wxPathList::from(0 as *mut c_void) }
    
}

pub trait _wxPathList : _wxList {
}

pub struct wxRegEx { ptr: *mut c_void }
impl _wxRegEx for wxRegEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRegEx {
    pub fn from(ptr: *mut c_void) -> wxRegEx { wxRegEx { ptr: ptr } }
    pub fn null() -> wxRegEx { wxRegEx::from(0 as *mut c_void) }
    
}

pub trait _wxRegEx {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxScopedArray { ptr: *mut c_void }
impl _wxScopedArray for wxScopedArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScopedArray {
    pub fn from(ptr: *mut c_void) -> wxScopedArray { wxScopedArray { ptr: ptr } }
    pub fn null() -> wxScopedArray { wxScopedArray::from(0 as *mut c_void) }
    
}

pub trait _wxScopedArray {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxScopedPtr { ptr: *mut c_void }
impl _wxScopedPtr for wxScopedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScopedPtr {
    pub fn from(ptr: *mut c_void) -> wxScopedPtr { wxScopedPtr { ptr: ptr } }
    pub fn null() -> wxScopedPtr { wxScopedPtr::from(0 as *mut c_void) }
    
}

pub trait _wxScopedPtr {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxSemaphore { ptr: *mut c_void }
impl _wxSemaphore for wxSemaphore { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSemaphore {
    pub fn from(ptr: *mut c_void) -> wxSemaphore { wxSemaphore { ptr: ptr } }
    pub fn null() -> wxSemaphore { wxSemaphore::from(0 as *mut c_void) }
    
}

pub trait _wxSemaphore {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxServer { ptr: *mut c_void }
impl _wxServer for wxServer {}
impl _wxServerBase for wxServer {}
impl _wxObject for wxServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxServer {
    pub fn from(ptr: *mut c_void) -> wxServer { wxServer { ptr: ptr } }
    pub fn null() -> wxServer { wxServer::from(0 as *mut c_void) }
    
}

pub trait _wxServer : _wxServerBase {
}

pub struct wxServerBase { ptr: *mut c_void }
impl _wxServerBase for wxServerBase {}
impl _wxObject for wxServerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxServerBase {
    pub fn from(ptr: *mut c_void) -> wxServerBase { wxServerBase { ptr: ptr } }
    pub fn null() -> wxServerBase { wxServerBase::from(0 as *mut c_void) }
    
}

pub trait _wxServerBase : _wxObject {
}

pub struct wxSingleInstanceChecker { ptr: *mut c_void }
impl _wxSingleInstanceChecker for wxSingleInstanceChecker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSingleInstanceChecker {
    pub fn from(ptr: *mut c_void) -> wxSingleInstanceChecker { wxSingleInstanceChecker { ptr: ptr } }
    pub fn null() -> wxSingleInstanceChecker { wxSingleInstanceChecker::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, name: &str, path: &str) -> c_int {
        let name = wxT(name);
        let path = wxT(path);
        unsafe { wxSingleInstanceChecker_Create(_obj, name.ptr(), path.ptr()) }
    }
    pub fn newDefault() -> wxSingleInstanceChecker {
        unsafe { wxSingleInstanceChecker { ptr: wxSingleInstanceChecker_CreateDefault() } }
    }
}

pub trait _wxSingleInstanceChecker {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.ptr()) }
    }
    fn isAnotherRunning(&self) -> c_int {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.ptr()) }
    }
}

pub struct wxStopWatch { ptr: *mut c_void }
impl _wxStopWatch for wxStopWatch { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStopWatch {
    pub fn from(ptr: *mut c_void) -> wxStopWatch { wxStopWatch { ptr: ptr } }
    pub fn null() -> wxStopWatch { wxStopWatch::from(0 as *mut c_void) }
    
    pub fn new() -> wxStopWatch {
        unsafe { wxStopWatch { ptr: wxStopWatch_Create() } }
    }
}

pub trait _wxStopWatch {
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

pub struct wxStreamBase { ptr: *mut c_void }
impl _wxStreamBase for wxStreamBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStreamBase {
    pub fn from(ptr: *mut c_void) -> wxStreamBase { wxStreamBase { ptr: ptr } }
    pub fn null() -> wxStreamBase { wxStreamBase::from(0 as *mut c_void) }
    
}

pub trait _wxStreamBase {
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

pub struct wxStreamBuffer { ptr: *mut c_void }
impl _wxStreamBuffer for wxStreamBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStreamBuffer {
    pub fn from(ptr: *mut c_void) -> wxStreamBuffer { wxStreamBuffer { ptr: ptr } }
    pub fn null() -> wxStreamBuffer { wxStreamBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxStreamBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxStringBuffer { ptr: *mut c_void }
impl _wxStringBuffer for wxStringBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStringBuffer {
    pub fn from(ptr: *mut c_void) -> wxStringBuffer { wxStringBuffer { ptr: ptr } }
    pub fn null() -> wxStringBuffer { wxStringBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxStringBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxStringClientData { ptr: *mut c_void }
impl _wxStringClientData for wxStringClientData {}
impl _wxClientData for wxStringClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStringClientData {
    pub fn from(ptr: *mut c_void) -> wxStringClientData { wxStringClientData { ptr: ptr } }
    pub fn null() -> wxStringClientData { wxStringClientData::from(0 as *mut c_void) }
    
}

pub trait _wxStringClientData : _wxClientData {
}

pub struct wxStringList { ptr: *mut c_void }
impl _wxStringList for wxStringList {}
impl _wxList for wxStringList {}
impl _wxObject for wxStringList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStringList {
    pub fn from(ptr: *mut c_void) -> wxStringList { wxStringList { ptr: ptr } }
    pub fn null() -> wxStringList { wxStringList::from(0 as *mut c_void) }
    
}

pub trait _wxStringList : _wxList {
}

pub struct wxStringTokenizer { ptr: *mut c_void }
impl _wxStringTokenizer for wxStringTokenizer {}
impl _wxObject for wxStringTokenizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStringTokenizer {
    pub fn from(ptr: *mut c_void) -> wxStringTokenizer { wxStringTokenizer { ptr: ptr } }
    pub fn null() -> wxStringTokenizer { wxStringTokenizer::from(0 as *mut c_void) }
    
}

pub trait _wxStringTokenizer : _wxObject {
}

pub struct wxSystemOptions { ptr: *mut c_void }
impl _wxSystemOptions for wxSystemOptions {}
impl _wxObject for wxSystemOptions { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSystemOptions {
    pub fn from(ptr: *mut c_void) -> wxSystemOptions { wxSystemOptions { ptr: ptr } }
    pub fn null() -> wxSystemOptions { wxSystemOptions::from(0 as *mut c_void) }
    
}

pub trait _wxSystemOptions : _wxObject {
}

pub struct wxTempFile { ptr: *mut c_void }
impl _wxTempFile for wxTempFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTempFile {
    pub fn from(ptr: *mut c_void) -> wxTempFile { wxTempFile { ptr: ptr } }
    pub fn null() -> wxTempFile { wxTempFile::from(0 as *mut c_void) }
    
}

pub trait _wxTempFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxTextFile { ptr: *mut c_void }
impl _wxTextFile for wxTextFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextFile {
    pub fn from(ptr: *mut c_void) -> wxTextFile { wxTextFile { ptr: ptr } }
    pub fn null() -> wxTextFile { wxTextFile::from(0 as *mut c_void) }
    
}

pub trait _wxTextFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxTextInputStream { ptr: *mut c_void }
impl _wxTextInputStream for wxTextInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextInputStream {
    pub fn from(ptr: *mut c_void) -> wxTextInputStream { wxTextInputStream { ptr: ptr } }
    pub fn null() -> wxTextInputStream { wxTextInputStream::from(0 as *mut c_void) }
    
    pub fn new<T: _wxInputStream>(inputStream: &T, sep: &str) -> wxTextInputStream {
        let sep = wxT(sep);
        unsafe { wxTextInputStream { ptr: wxTextInputStream_Create(inputStream.ptr(), sep.ptr()) } }
    }
}

pub trait _wxTextInputStream {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.ptr()) }
    }
    fn readLine(&self) -> ~str {
        unsafe { wxString { ptr: wxTextInputStream_ReadLine(self.ptr()) }.to_str() }
    }
}

pub struct wxTextOutputStream { ptr: *mut c_void }
impl _wxTextOutputStream for wxTextOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextOutputStream {
    pub fn from(ptr: *mut c_void) -> wxTextOutputStream { wxTextOutputStream { ptr: ptr } }
    pub fn null() -> wxTextOutputStream { wxTextOutputStream::from(0 as *mut c_void) }
    
    pub fn new<T: _wxOutputStream>(outputStream: &T, mode: c_int) -> wxTextOutputStream {
        unsafe { wxTextOutputStream { ptr: wxTextOutputStream_Create(outputStream.ptr(), mode) } }
    }
}

pub trait _wxTextOutputStream {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.ptr()) }
    }
    fn writeString(&self, txt: &str) {
        let txt = wxT(txt);
        unsafe { wxTextOutputStream_WriteString(self.ptr(), txt.ptr()) }
    }
}

pub struct wxThread { ptr: *mut c_void }
impl _wxThread for wxThread { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxThread {
    pub fn from(ptr: *mut c_void) -> wxThread { wxThread { ptr: ptr } }
    pub fn null() -> wxThread { wxThread::from(0 as *mut c_void) }
    
}

pub trait _wxThread {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxTime { ptr: *mut c_void }
impl _wxTime for wxTime {}
impl _wxObject for wxTime { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTime {
    pub fn from(ptr: *mut c_void) -> wxTime { wxTime { ptr: ptr } }
    pub fn null() -> wxTime { wxTime::from(0 as *mut c_void) }
    
}

pub trait _wxTime : _wxObject {
}

pub struct wxTimeSpan { ptr: *mut c_void }
impl _wxTimeSpan for wxTimeSpan { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTimeSpan {
    pub fn from(ptr: *mut c_void) -> wxTimeSpan { wxTimeSpan { ptr: ptr } }
    pub fn null() -> wxTimeSpan { wxTimeSpan::from(0 as *mut c_void) }
    
}

pub trait _wxTimeSpan {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxVariant { ptr: *mut c_void }
impl _wxVariant for wxVariant {}
impl _wxObject for wxVariant { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxVariant {
    pub fn from(ptr: *mut c_void) -> wxVariant { wxVariant { ptr: ptr } }
    pub fn null() -> wxVariant { wxVariant::from(0 as *mut c_void) }
    
}

pub trait _wxVariant : _wxObject {
}

pub struct wxVariantData { ptr: *mut c_void }
impl _wxVariantData for wxVariantData {}
impl _wxObject for wxVariantData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxVariantData {
    pub fn from(ptr: *mut c_void) -> wxVariantData { wxVariantData { ptr: ptr } }
    pub fn null() -> wxVariantData { wxVariantData::from(0 as *mut c_void) }
    
}

pub trait _wxVariantData : _wxObject {
}

pub struct wxZipInputStream { ptr: *mut c_void }
impl _wxZipInputStream for wxZipInputStream {}
impl _wxInputStream for wxZipInputStream {}
impl _wxStreamBase for wxZipInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxZipInputStream {
    pub fn from(ptr: *mut c_void) -> wxZipInputStream { wxZipInputStream { ptr: ptr } }
    pub fn null() -> wxZipInputStream { wxZipInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxZipInputStream : _wxInputStream {
}

pub struct wxZlibInputStream { ptr: *mut c_void }
impl _wxZlibInputStream for wxZlibInputStream {}
impl _wxFilterInputStream for wxZlibInputStream {}
impl _wxInputStream for wxZlibInputStream {}
impl _wxStreamBase for wxZlibInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxZlibInputStream {
    pub fn from(ptr: *mut c_void) -> wxZlibInputStream { wxZlibInputStream { ptr: ptr } }
    pub fn null() -> wxZlibInputStream { wxZlibInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxZlibInputStream : _wxFilterInputStream {
}

pub struct wxZlibOutputStream { ptr: *mut c_void }
impl _wxZlibOutputStream for wxZlibOutputStream {}
impl _wxFilterOutputStream for wxZlibOutputStream {}
impl _wxOutputStream for wxZlibOutputStream {}
impl _wxStreamBase for wxZlibOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxZlibOutputStream {
    pub fn from(ptr: *mut c_void) -> wxZlibOutputStream { wxZlibOutputStream { ptr: ptr } }
    pub fn null() -> wxZlibOutputStream { wxZlibOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxZlibOutputStream : _wxFilterOutputStream {
}

pub struct wxMemoryBuffer { ptr: *mut c_void }
impl _wxMemoryBuffer for wxMemoryBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMemoryBuffer {
    pub fn from(ptr: *mut c_void) -> wxMemoryBuffer { wxMemoryBuffer { ptr: ptr } }
    pub fn null() -> wxMemoryBuffer { wxMemoryBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxMemoryBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxFileConfig { ptr: *mut c_void }
impl _wxFileConfig for wxFileConfig {}
impl _wxConfigBase for wxFileConfig { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileConfig {
    pub fn from(ptr: *mut c_void) -> wxFileConfig { wxFileConfig { ptr: ptr } }
    pub fn null() -> wxFileConfig { wxFileConfig::from(0 as *mut c_void) }
    
    pub fn new<T: _wxInputStream>(inp: &T) -> wxFileConfig {
        unsafe { wxFileConfig { ptr: wxFileConfig_Create(inp.ptr()) } }
    }
}

pub trait _wxFileConfig : _wxConfigBase {
}

