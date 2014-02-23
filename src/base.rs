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

pub fn wxT(s: &str) -> WxString {
    unsafe {
        s.to_c_str().with_ref(|c_str| {
            WxString { ptr: wxString_CreateUTF8(c_str as *mut c_void) }
        })
    }
}

pub struct WxString { ptr: *mut c_void }
impl Drop for WxString {
    fn drop(&mut self) {
        unsafe { wxString_Delete(self.ptr); }
    }
}
impl WxString {
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

pub struct WxrClient { ptr: *mut c_void }
impl TWxrClient for WxrClient {}
impl TWxClient for WxrClient {}
impl TWxClientBase for WxrClient {}
impl TWxObject for WxrClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrClient {
    pub fn from(ptr: *mut c_void) -> WxrClient { WxrClient { ptr: ptr } }
    pub fn null() -> WxrClient { WxrClient::from(0 as *mut c_void) }
    
}

pub trait TWxrClient : TWxClient {
}

pub struct WxrConnection { ptr: *mut c_void }
impl TWxrConnection for WxrConnection {}
impl TWxConnection for WxrConnection {}
impl TWxConnectionBase for WxrConnection {}
impl TWxObject for WxrConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrConnection {
    pub fn from(ptr: *mut c_void) -> WxrConnection { WxrConnection { ptr: ptr } }
    pub fn null() -> WxrConnection { WxrConnection::from(0 as *mut c_void) }
    
}

pub trait TWxrConnection : TWxConnection {
}

pub struct WxrLocale { ptr: *mut c_void }
impl TWxrLocale for WxrLocale {}
impl TWxLocale for WxrLocale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrLocale {
    pub fn from(ptr: *mut c_void) -> WxrLocale { WxrLocale { ptr: ptr } }
    pub fn null() -> WxrLocale { WxrLocale::from(0 as *mut c_void) }
    
}

pub trait TWxrLocale : TWxLocale {
}

pub struct WxrServer { ptr: *mut c_void }
impl TWxrServer for WxrServer {}
impl TWxServer for WxrServer {}
impl TWxServerBase for WxrServer {}
impl TWxObject for WxrServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrServer {
    pub fn from(ptr: *mut c_void) -> WxrServer { WxrServer { ptr: ptr } }
    pub fn null() -> WxrServer { WxrServer::from(0 as *mut c_void) }
    
}

pub trait TWxrServer : TWxServer {
}

pub struct WxArray { ptr: *mut c_void }
impl TWxArray for WxArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxArray {
    pub fn from(ptr: *mut c_void) -> WxArray { WxArray { ptr: ptr } }
    pub fn null() -> WxArray { WxArray::from(0 as *mut c_void) }
    
}

pub trait TWxArray {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxArrayString { ptr: *mut c_void }
impl TWxArrayString for WxArrayString {}
impl TWxArray for WxArrayString { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxArrayString {
    pub fn from(ptr: *mut c_void) -> WxArrayString { WxArrayString { ptr: ptr } }
    pub fn null() -> WxArrayString { WxArrayString::from(0 as *mut c_void) }
    
}

pub trait TWxArrayString : TWxArray {
}

pub struct WxBufferedInputStream { ptr: *mut c_void }
impl TWxBufferedInputStream for WxBufferedInputStream {}
impl TWxFilterInputStream for WxBufferedInputStream {}
impl TWxInputStream for WxBufferedInputStream {}
impl TWxStreamBase for WxBufferedInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBufferedInputStream {
    pub fn from(ptr: *mut c_void) -> WxBufferedInputStream { WxBufferedInputStream { ptr: ptr } }
    pub fn null() -> WxBufferedInputStream { WxBufferedInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxBufferedInputStream : TWxFilterInputStream {
}

pub struct WxBufferedOutputStream { ptr: *mut c_void }
impl TWxBufferedOutputStream for WxBufferedOutputStream {}
impl TWxFilterOutputStream for WxBufferedOutputStream {}
impl TWxOutputStream for WxBufferedOutputStream {}
impl TWxStreamBase for WxBufferedOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBufferedOutputStream {
    pub fn from(ptr: *mut c_void) -> WxBufferedOutputStream { WxBufferedOutputStream { ptr: ptr } }
    pub fn null() -> WxBufferedOutputStream { WxBufferedOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxBufferedOutputStream : TWxFilterOutputStream {
}

pub struct WxCSConv { ptr: *mut c_void }
impl TWxCSConv for WxCSConv {}
impl TWxMBConv for WxCSConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCSConv {
    pub fn from(ptr: *mut c_void) -> WxCSConv { WxCSConv { ptr: ptr } }
    pub fn null() -> WxCSConv { WxCSConv::from(0 as *mut c_void) }
    
}

pub trait TWxCSConv : TWxMBConv {
}

pub struct WxClassInfo { ptr: *mut c_void }
impl TWxClassInfo for WxClassInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClassInfo {
    pub fn from(ptr: *mut c_void) -> WxClassInfo { WxClassInfo { ptr: ptr } }
    pub fn null() -> WxClassInfo { WxClassInfo::from(0 as *mut c_void) }
    
    pub fn findClass(_txt: &str) -> WxClassInfo {
        let _txt = wxT(_txt);
        unsafe { WxClassInfo { ptr: wxClassInfo_FindClass(_txt.ptr()) } }
    }
}

pub trait TWxClassInfo {
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
        unsafe { WxString { ptr: wxClassInfo_GetBaseClassName1(self.ptr()) }.to_str() }
    }
    fn getBaseClassName2(&self) -> ~str {
        unsafe { WxString { ptr: wxClassInfo_GetBaseClassName2(self.ptr()) }.to_str() }
    }
    fn getClassNameEx(&self) -> ~str {
        unsafe { WxString { ptr: wxClassInfo_GetClassNameEx(self.ptr()) }.to_str() }
    }
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.ptr()) }
    }
    fn isKindOfEx<T: TWxClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxClassInfo_IsKindOfEx(self.ptr(), classInfo.ptr()) }
    }
}

pub struct WxClient { ptr: *mut c_void }
impl TWxClient for WxClient {}
impl TWxClientBase for WxClient {}
impl TWxObject for WxClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClient {
    pub fn from(ptr: *mut c_void) -> WxClient { WxClient { ptr: ptr } }
    pub fn null() -> WxClient { WxClient::from(0 as *mut c_void) }
    
}

pub trait TWxClient : TWxClientBase {
}

pub struct WxClientBase { ptr: *mut c_void }
impl TWxClientBase for WxClientBase {}
impl TWxObject for WxClientBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClientBase {
    pub fn from(ptr: *mut c_void) -> WxClientBase { WxClientBase { ptr: ptr } }
    pub fn null() -> WxClientBase { WxClientBase::from(0 as *mut c_void) }
    
}

pub trait TWxClientBase : TWxObject {
}

pub struct WxClientData { ptr: *mut c_void }
impl TWxClientData for WxClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClientData {
    pub fn from(ptr: *mut c_void) -> WxClientData { WxClientData { ptr: ptr } }
    pub fn null() -> WxClientData { WxClientData::from(0 as *mut c_void) }
    
}

pub trait TWxClientData {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxClientDataContainer { ptr: *mut c_void }
impl TWxClientDataContainer for WxClientDataContainer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClientDataContainer {
    pub fn from(ptr: *mut c_void) -> WxClientDataContainer { WxClientDataContainer { ptr: ptr } }
    pub fn null() -> WxClientDataContainer { WxClientDataContainer::from(0 as *mut c_void) }
    
}

pub trait TWxClientDataContainer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxClosure { ptr: *mut c_void }
impl TWxClosure for WxClosure {}
impl TWxObject for WxClosure { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClosure {
    pub fn from(ptr: *mut c_void) -> WxClosure { WxClosure { ptr: ptr } }
    pub fn null() -> WxClosure { WxClosure::from(0 as *mut c_void) }
    
    pub fn new(_fun_CEvent: *mut c_void, _data: *mut c_void) -> WxClosure {
        unsafe { WxClosure { ptr: wxClosure_Create(_fun_CEvent, _data) } }
    }
}

pub trait TWxClosure : TWxObject {
    fn getData(&self) -> *mut c_void {
        unsafe { wxClosure_GetData(self.ptr()) }
    }
}

pub struct WxCommandLineParser { ptr: *mut c_void }
impl TWxCommandLineParser for WxCommandLineParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCommandLineParser {
    pub fn from(ptr: *mut c_void) -> WxCommandLineParser { WxCommandLineParser { ptr: ptr } }
    pub fn null() -> WxCommandLineParser { WxCommandLineParser::from(0 as *mut c_void) }
    
}

pub trait TWxCommandLineParser {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxCondition { ptr: *mut c_void }
impl TWxCondition for WxCondition { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCondition {
    pub fn from(ptr: *mut c_void) -> WxCondition { WxCondition { ptr: ptr } }
    pub fn null() -> WxCondition { WxCondition::from(0 as *mut c_void) }
    
}

pub trait TWxCondition {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxConfigBase { ptr: *mut c_void }
impl TWxConfigBase for WxConfigBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxConfigBase {
    pub fn from(ptr: *mut c_void) -> WxConfigBase { WxConfigBase { ptr: ptr } }
    pub fn null() -> WxConfigBase { WxConfigBase::from(0 as *mut c_void) }
    
    pub fn new() -> WxConfigBase {
        unsafe { WxConfigBase { ptr: wxConfigBase_Create() } }
    }
    pub fn get() -> WxConfigBase {
        unsafe { WxConfigBase { ptr: wxConfigBase_Get() } }
    }
    pub fn set<T: TWxConfigBase>(self_: &T) {
        unsafe { wxConfigBase_Set(self_.ptr()) }
    }
}

pub trait TWxConfigBase {
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
        unsafe { WxString { ptr: wxConfigBase_ExpandEnvVars(self.ptr(), str.ptr()) }.to_str() }
    }
    fn flush(&self, bCurrentOnly: c_int) -> c_int {
        unsafe { wxConfigBase_Flush(self.ptr(), bCurrentOnly) }
    }
    fn getAppName(&self) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetAppName(self.ptr()) }.to_str() }
    }
    fn getEntryType(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxConfigBase_GetEntryType(self.ptr(), name.ptr()) }
    }
    fn getFirstEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetFirstEntry(self.ptr(), lIndex) }.to_str() }
    }
    fn getFirstGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetFirstGroup(self.ptr(), lIndex) }.to_str() }
    }
    fn getNextEntry(&self, lIndex: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetNextEntry(self.ptr(), lIndex) }.to_str() }
    }
    fn getNextGroup(&self, lIndex: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetNextGroup(self.ptr(), lIndex) }.to_str() }
    }
    fn getNumberOfEntries(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfEntries(self.ptr(), bRecursive) }
    }
    fn getNumberOfGroups(&self, bRecursive: c_int) -> c_int {
        unsafe { wxConfigBase_GetNumberOfGroups(self.ptr(), bRecursive) }
    }
    fn getPath(&self) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetPath(self.ptr()) }.to_str() }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.ptr()) }
    }
    fn getVendorName(&self) -> ~str {
        unsafe { WxString { ptr: wxConfigBase_GetVendorName(self.ptr()) }.to_str() }
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
        unsafe { WxString { ptr: wxConfigBase_ReadString(self.ptr(), key.ptr(), defVal.ptr()) }.to_str() }
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

pub struct WxConnection { ptr: *mut c_void }
impl TWxConnection for WxConnection {}
impl TWxConnectionBase for WxConnection {}
impl TWxObject for WxConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxConnection {
    pub fn from(ptr: *mut c_void) -> WxConnection { WxConnection { ptr: ptr } }
    pub fn null() -> WxConnection { WxConnection::from(0 as *mut c_void) }
    
}

pub trait TWxConnection : TWxConnectionBase {
}

pub struct WxConnectionBase { ptr: *mut c_void }
impl TWxConnectionBase for WxConnectionBase {}
impl TWxObject for WxConnectionBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxConnectionBase {
    pub fn from(ptr: *mut c_void) -> WxConnectionBase { WxConnectionBase { ptr: ptr } }
    pub fn null() -> WxConnectionBase { WxConnectionBase::from(0 as *mut c_void) }
    
}

pub trait TWxConnectionBase : TWxObject {
}

pub struct WxCountingOutputStream { ptr: *mut c_void }
impl TWxCountingOutputStream for WxCountingOutputStream {}
impl TWxOutputStream for WxCountingOutputStream {}
impl TWxStreamBase for WxCountingOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCountingOutputStream {
    pub fn from(ptr: *mut c_void) -> WxCountingOutputStream { WxCountingOutputStream { ptr: ptr } }
    pub fn null() -> WxCountingOutputStream { WxCountingOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxCountingOutputStream : TWxOutputStream {
}

pub struct WxCriticalSection { ptr: *mut c_void }
impl TWxCriticalSection for WxCriticalSection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCriticalSection {
    pub fn from(ptr: *mut c_void) -> WxCriticalSection { WxCriticalSection { ptr: ptr } }
    pub fn null() -> WxCriticalSection { WxCriticalSection::from(0 as *mut c_void) }
    
}

pub trait TWxCriticalSection {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxCriticalSectionLocker { ptr: *mut c_void }
impl TWxCriticalSectionLocker for WxCriticalSectionLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCriticalSectionLocker {
    pub fn from(ptr: *mut c_void) -> WxCriticalSectionLocker { WxCriticalSectionLocker { ptr: ptr } }
    pub fn null() -> WxCriticalSectionLocker { WxCriticalSectionLocker::from(0 as *mut c_void) }
    
}

pub trait TWxCriticalSectionLocker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDDEClient { ptr: *mut c_void }
impl TWxDDEClient for WxDDEClient {}
impl TWxClientBase for WxDDEClient {}
impl TWxObject for WxDDEClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDDEClient {
    pub fn from(ptr: *mut c_void) -> WxDDEClient { WxDDEClient { ptr: ptr } }
    pub fn null() -> WxDDEClient { WxDDEClient::from(0 as *mut c_void) }
    
}

pub trait TWxDDEClient : TWxClientBase {
}

pub struct WxDDEConnection { ptr: *mut c_void }
impl TWxDDEConnection for WxDDEConnection {}
impl TWxConnectionBase for WxDDEConnection {}
impl TWxObject for WxDDEConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDDEConnection {
    pub fn from(ptr: *mut c_void) -> WxDDEConnection { WxDDEConnection { ptr: ptr } }
    pub fn null() -> WxDDEConnection { WxDDEConnection::from(0 as *mut c_void) }
    
}

pub trait TWxDDEConnection : TWxConnectionBase {
}

pub struct WxDDEServer { ptr: *mut c_void }
impl TWxDDEServer for WxDDEServer {}
impl TWxServerBase for WxDDEServer {}
impl TWxObject for WxDDEServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDDEServer {
    pub fn from(ptr: *mut c_void) -> WxDDEServer { WxDDEServer { ptr: ptr } }
    pub fn null() -> WxDDEServer { WxDDEServer::from(0 as *mut c_void) }
    
}

pub trait TWxDDEServer : TWxServerBase {
}

pub struct WxDataInputStream { ptr: *mut c_void }
impl TWxDataInputStream for WxDataInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDataInputStream {
    pub fn from(ptr: *mut c_void) -> WxDataInputStream { WxDataInputStream { ptr: ptr } }
    pub fn null() -> WxDataInputStream { WxDataInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxDataInputStream {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDataOutputStream { ptr: *mut c_void }
impl TWxDataOutputStream for WxDataOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDataOutputStream {
    pub fn from(ptr: *mut c_void) -> WxDataOutputStream { WxDataOutputStream { ptr: ptr } }
    pub fn null() -> WxDataOutputStream { WxDataOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxDataOutputStream {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDateTime { ptr: *mut c_void }
impl TWxDateTime for WxDateTime { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDateTime {
    pub fn from(ptr: *mut c_void) -> WxDateTime { WxDateTime { ptr: ptr } }
    pub fn null() -> WxDateTime { WxDateTime::from(0 as *mut c_void) }
    
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    pub fn new() -> WxDateTime {
        unsafe { WxDateTime { ptr: wxDateTime_Create() } }
    }
    pub fn getAmString() -> ~str {
        unsafe { WxString { ptr: wxDateTime_GetAmString() }.to_str() }
    }
    pub fn getBeginDST<T: TWxDateTime>(year: c_int, country: c_int, dt: &T) {
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
    pub fn getEndDST<T: TWxDateTime>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.ptr()) }
    }
    pub fn getMonthName(month: c_int, flags: c_int) -> ~str {
        unsafe { WxString { ptr: wxDateTime_GetMonthName(month, flags) }.to_str() }
    }
    pub fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    pub fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    pub fn getPmString() -> ~str {
        unsafe { WxString { ptr: wxDateTime_GetPmString() }.to_str() }
    }
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> ~str {
        unsafe { WxString { ptr: wxDateTime_GetWeekDayName(weekday, flags) }.to_str() }
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

pub trait TWxDateTime {
    fn ptr(&self) -> *mut c_void;
    
    fn addDate<T: TWxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.ptr(), _yrs, _mnt, _wek, _day) }
    }
    fn addTime<T: TWxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddTime(self.ptr(), diff, _ref.ptr()) }
    }
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.ptr(), _hrs, _min, _sec, _mls) }
    }
    fn format(&self, format: *mut c_void, tz: c_int) -> ~str {
        unsafe { WxString { ptr: wxDateTime_Format(self.ptr(), format, tz) }.to_str() }
    }
    fn formatDate(&self) -> ~str {
        unsafe { WxString { ptr: wxDateTime_FormatDate(self.ptr()) }.to_str() }
    }
    fn formatISODate(&self) -> ~str {
        unsafe { WxString { ptr: wxDateTime_FormatISODate(self.ptr()) }.to_str() }
    }
    fn formatISOTime(&self) -> ~str {
        unsafe { WxString { ptr: wxDateTime_FormatISOTime(self.ptr()) }.to_str() }
    }
    fn formatTime(&self) -> ~str {
        unsafe { WxString { ptr: wxDateTime_FormatTime(self.ptr()) }.to_str() }
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
    fn getLastMonthDay<T: TWxDateTime>(&self, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastMonthDay(self.ptr(), month, year, _ref.ptr()) }
    }
    fn getLastWeekDay<T: TWxDateTime>(&self, weekday: c_int, month: c_int, year: c_int, _ref: &T) {
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
    fn getNextWeekDay<T: TWxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetNextWeekDay(self.ptr(), weekday, _ref.ptr()) }
    }
    fn getPrevWeekDay<T: TWxDateTime>(&self, weekday: c_int, _ref: &T) {
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
    fn getWeekDay<T: TWxDateTime>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDay(self.ptr(), weekday, n, month, year, _ref.ptr()) }
    }
    fn getWeekDayInSameWeek<T: TWxDateTime>(&self, weekday: c_int, _ref: &T) {
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
    fn isBetween<T: TWxDateTime, U: TWxDateTime>(&self, t1: &T, t2: &U) -> c_int {
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
    fn isEqualUpTo<T: TWxDateTime>(&self, dt: &T, ts: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualUpTo(self.ptr(), dt.ptr(), ts) }
    }
    fn isLaterThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsLaterThan(self.ptr(), datetime) }
    }
    fn isSameDate<T: TWxDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameDate(self.ptr(), dt.ptr()) }
    }
    fn isSameTime<T: TWxDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameTime(self.ptr(), dt.ptr()) }
    }
    fn isStrictlyBetween<T: TWxDateTime, U: TWxDateTime>(&self, t1: &T, t2: &U) -> c_int {
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
    fn parseTime<T: TWxTime>(&self, time: &T) -> *mut c_void {
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
    fn subtractDate<T: TWxDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn subtractTime<T: TWxDateTime>(&self, diff: *mut c_void, _ref: &T) {
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

pub struct WxDebugContext { ptr: *mut c_void }
impl TWxDebugContext for WxDebugContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDebugContext {
    pub fn from(ptr: *mut c_void) -> WxDebugContext { WxDebugContext { ptr: ptr } }
    pub fn null() -> WxDebugContext { WxDebugContext::from(0 as *mut c_void) }
    
}

pub trait TWxDebugContext {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDirTraverser { ptr: *mut c_void }
impl TWxDirTraverser for WxDirTraverser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDirTraverser {
    pub fn from(ptr: *mut c_void) -> WxDirTraverser { WxDirTraverser { ptr: ptr } }
    pub fn null() -> WxDirTraverser { WxDirTraverser::from(0 as *mut c_void) }
    
}

pub trait TWxDirTraverser {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDllLoader { ptr: *mut c_void }
impl TWxDllLoader for WxDllLoader { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDllLoader {
    pub fn from(ptr: *mut c_void) -> WxDllLoader { WxDllLoader { ptr: ptr } }
    pub fn null() -> WxDllLoader { WxDllLoader::from(0 as *mut c_void) }
    
}

pub trait TWxDllLoader {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDynamicLibrary { ptr: *mut c_void }
impl TWxDynamicLibrary for WxDynamicLibrary { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDynamicLibrary {
    pub fn from(ptr: *mut c_void) -> WxDynamicLibrary { WxDynamicLibrary { ptr: ptr } }
    pub fn null() -> WxDynamicLibrary { WxDynamicLibrary::from(0 as *mut c_void) }
    
}

pub trait TWxDynamicLibrary {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxEncodingConverter { ptr: *mut c_void }
impl TWxEncodingConverter for WxEncodingConverter {}
impl TWxObject for WxEncodingConverter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxEncodingConverter {
    pub fn from(ptr: *mut c_void) -> WxEncodingConverter { WxEncodingConverter { ptr: ptr } }
    pub fn null() -> WxEncodingConverter { WxEncodingConverter::from(0 as *mut c_void) }
    
    pub fn new() -> WxEncodingConverter {
        unsafe { WxEncodingConverter { ptr: wxEncodingConverter_Create() } }
    }
}

pub trait TWxEncodingConverter : TWxObject {
    fn convert(&self, input: *mut c_void, output: *mut c_void) {
        unsafe { wxEncodingConverter_Convert(self.ptr(), input, output) }
    }
    fn getAllEquivalents<T: TWxList>(&self, enc: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.ptr(), enc, _lst.ptr()) }
    }
    fn getPlatformEquivalents<T: TWxList>(&self, enc: c_int, platform: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.ptr(), enc, platform, _lst.ptr()) }
    }
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.ptr(), input_enc, output_enc, method) }
    }
}

pub struct WxFFile { ptr: *mut c_void }
impl TWxFFile for WxFFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFFile {
    pub fn from(ptr: *mut c_void) -> WxFFile { WxFFile { ptr: ptr } }
    pub fn null() -> WxFFile { WxFFile::from(0 as *mut c_void) }
    
}

pub trait TWxFFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxFFileInputStream { ptr: *mut c_void }
impl TWxFFileInputStream for WxFFileInputStream {}
impl TWxInputStream for WxFFileInputStream {}
impl TWxStreamBase for WxFFileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFFileInputStream {
    pub fn from(ptr: *mut c_void) -> WxFFileInputStream { WxFFileInputStream { ptr: ptr } }
    pub fn null() -> WxFFileInputStream { WxFFileInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxFFileInputStream : TWxInputStream {
}

pub struct WxFFileOutputStream { ptr: *mut c_void }
impl TWxFFileOutputStream for WxFFileOutputStream {}
impl TWxOutputStream for WxFFileOutputStream {}
impl TWxStreamBase for WxFFileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFFileOutputStream {
    pub fn from(ptr: *mut c_void) -> WxFFileOutputStream { WxFFileOutputStream { ptr: ptr } }
    pub fn null() -> WxFFileOutputStream { WxFFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxFFileOutputStream : TWxOutputStream {
}

pub struct WxFSFile { ptr: *mut c_void }
impl TWxFSFile for WxFSFile {}
impl TWxObject for WxFSFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFSFile {
    pub fn from(ptr: *mut c_void) -> WxFSFile { WxFSFile { ptr: ptr } }
    pub fn null() -> WxFSFile { WxFSFile::from(0 as *mut c_void) }
    
}

pub trait TWxFSFile : TWxObject {
}

pub struct WxFileInputStream { ptr: *mut c_void }
impl TWxFileInputStream for WxFileInputStream {}
impl TWxInputStream for WxFileInputStream {}
impl TWxStreamBase for WxFileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileInputStream {
    pub fn from(ptr: *mut c_void) -> WxFileInputStream { WxFileInputStream { ptr: ptr } }
    pub fn null() -> WxFileInputStream { WxFileInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxFileInputStream : TWxInputStream {
}

pub struct WxFileName { ptr: *mut c_void }
impl TWxFileName for WxFileName { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileName {
    pub fn from(ptr: *mut c_void) -> WxFileName { WxFileName { ptr: ptr } }
    pub fn null() -> WxFileName { WxFileName::from(0 as *mut c_void) }
    
}

pub trait TWxFileName {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxFileOutputStream { ptr: *mut c_void }
impl TWxFileOutputStream for WxFileOutputStream {}
impl TWxOutputStream for WxFileOutputStream {}
impl TWxStreamBase for WxFileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileOutputStream {
    pub fn from(ptr: *mut c_void) -> WxFileOutputStream { WxFileOutputStream { ptr: ptr } }
    pub fn null() -> WxFileOutputStream { WxFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxFileOutputStream : TWxOutputStream {
}

pub struct WxFileSystem { ptr: *mut c_void }
impl TWxFileSystem for WxFileSystem {}
impl TWxObject for WxFileSystem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileSystem {
    pub fn from(ptr: *mut c_void) -> WxFileSystem { WxFileSystem { ptr: ptr } }
    pub fn null() -> WxFileSystem { WxFileSystem::from(0 as *mut c_void) }
    
}

pub trait TWxFileSystem : TWxObject {
}

pub struct WxFileSystemHandler { ptr: *mut c_void }
impl TWxFileSystemHandler for WxFileSystemHandler {}
impl TWxObject for WxFileSystemHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileSystemHandler {
    pub fn from(ptr: *mut c_void) -> WxFileSystemHandler { WxFileSystemHandler { ptr: ptr } }
    pub fn null() -> WxFileSystemHandler { WxFileSystemHandler::from(0 as *mut c_void) }
    
}

pub trait TWxFileSystemHandler : TWxObject {
}

pub struct WxFilterInputStream { ptr: *mut c_void }
impl TWxFilterInputStream for WxFilterInputStream {}
impl TWxInputStream for WxFilterInputStream {}
impl TWxStreamBase for WxFilterInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFilterInputStream {
    pub fn from(ptr: *mut c_void) -> WxFilterInputStream { WxFilterInputStream { ptr: ptr } }
    pub fn null() -> WxFilterInputStream { WxFilterInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxFilterInputStream : TWxInputStream {
}

pub struct WxFilterOutputStream { ptr: *mut c_void }
impl TWxFilterOutputStream for WxFilterOutputStream {}
impl TWxOutputStream for WxFilterOutputStream {}
impl TWxStreamBase for WxFilterOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFilterOutputStream {
    pub fn from(ptr: *mut c_void) -> WxFilterOutputStream { WxFilterOutputStream { ptr: ptr } }
    pub fn null() -> WxFilterOutputStream { WxFilterOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxFilterOutputStream : TWxOutputStream {
}

pub struct WxInputStream { ptr: *mut c_void }
impl TWxInputStream for WxInputStream {}
impl TWxStreamBase for WxInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxInputStream {
    pub fn from(ptr: *mut c_void) -> WxInputStream { WxInputStream { ptr: ptr } }
    pub fn null() -> WxInputStream { WxInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxInputStream : TWxStreamBase {
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

pub struct WxList { ptr: *mut c_void }
impl TWxList for WxList {}
impl TWxObject for WxList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxList {
    pub fn from(ptr: *mut c_void) -> WxList { WxList { ptr: ptr } }
    pub fn null() -> WxList { WxList::from(0 as *mut c_void) }
    
}

pub trait TWxList : TWxObject {
}

pub struct WxLocale { ptr: *mut c_void }
impl TWxLocale for WxLocale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLocale {
    pub fn from(ptr: *mut c_void) -> WxLocale { WxLocale { ptr: ptr } }
    pub fn null() -> WxLocale { WxLocale::from(0 as *mut c_void) }
    
    pub fn new(_name: c_int, _flags: c_int) -> WxLocale {
        unsafe { WxLocale { ptr: wxLocale_Create(_name, _flags) } }
    }
}

pub trait TWxLocale {
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
    fn getLocale(&self) -> WxLocale {
        unsafe { WxLocale { ptr: wxLocale_GetLocale(self.ptr()) } }
    }
    fn getName(&self) -> ~str {
        unsafe { WxString { ptr: wxLocale_GetName(self.ptr()) }.to_str() }
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

pub struct WxLongLong { ptr: *mut c_void }
impl TWxLongLong for WxLongLong { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLongLong {
    pub fn from(ptr: *mut c_void) -> WxLongLong { WxLongLong { ptr: ptr } }
    pub fn null() -> WxLongLong { WxLongLong::from(0 as *mut c_void) }
    
}

pub trait TWxLongLong {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxMBConv { ptr: *mut c_void }
impl TWxMBConv for WxMBConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMBConv {
    pub fn from(ptr: *mut c_void) -> WxMBConv { WxMBConv { ptr: ptr } }
    pub fn null() -> WxMBConv { WxMBConv::from(0 as *mut c_void) }
    
}

pub trait TWxMBConv {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxMBConvUTF7 { ptr: *mut c_void }
impl TWxMBConvUTF7 for WxMBConvUTF7 {}
impl TWxMBConv for WxMBConvUTF7 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMBConvUTF7 {
    pub fn from(ptr: *mut c_void) -> WxMBConvUTF7 { WxMBConvUTF7 { ptr: ptr } }
    pub fn null() -> WxMBConvUTF7 { WxMBConvUTF7::from(0 as *mut c_void) }
    
}

pub trait TWxMBConvUTF7 : TWxMBConv {
}

pub struct WxMBConvUTF8 { ptr: *mut c_void }
impl TWxMBConvUTF8 for WxMBConvUTF8 {}
impl TWxMBConv for WxMBConvUTF8 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMBConvUTF8 {
    pub fn from(ptr: *mut c_void) -> WxMBConvUTF8 { WxMBConvUTF8 { ptr: ptr } }
    pub fn null() -> WxMBConvUTF8 { WxMBConvUTF8::from(0 as *mut c_void) }
    
}

pub trait TWxMBConvUTF8 : TWxMBConv {
}

pub struct WxMemoryFSHandler { ptr: *mut c_void }
impl TWxMemoryFSHandler for WxMemoryFSHandler {}
impl TWxFileSystemHandler for WxMemoryFSHandler {}
impl TWxObject for WxMemoryFSHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMemoryFSHandler {
    pub fn from(ptr: *mut c_void) -> WxMemoryFSHandler { WxMemoryFSHandler { ptr: ptr } }
    pub fn null() -> WxMemoryFSHandler { WxMemoryFSHandler::from(0 as *mut c_void) }
    
}

pub trait TWxMemoryFSHandler : TWxFileSystemHandler {
}

pub struct WxMemoryInputStream { ptr: *mut c_void }
impl TWxMemoryInputStream for WxMemoryInputStream {}
impl TWxInputStream for WxMemoryInputStream {}
impl TWxStreamBase for WxMemoryInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMemoryInputStream {
    pub fn from(ptr: *mut c_void) -> WxMemoryInputStream { WxMemoryInputStream { ptr: ptr } }
    pub fn null() -> WxMemoryInputStream { WxMemoryInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxMemoryInputStream : TWxInputStream {
}

pub struct WxMemoryOutputStream { ptr: *mut c_void }
impl TWxMemoryOutputStream for WxMemoryOutputStream {}
impl TWxOutputStream for WxMemoryOutputStream {}
impl TWxStreamBase for WxMemoryOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMemoryOutputStream {
    pub fn from(ptr: *mut c_void) -> WxMemoryOutputStream { WxMemoryOutputStream { ptr: ptr } }
    pub fn null() -> WxMemoryOutputStream { WxMemoryOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxMemoryOutputStream : TWxOutputStream {
}

pub struct WxModule { ptr: *mut c_void }
impl TWxModule for WxModule {}
impl TWxObject for WxModule { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxModule {
    pub fn from(ptr: *mut c_void) -> WxModule { WxModule { ptr: ptr } }
    pub fn null() -> WxModule { WxModule::from(0 as *mut c_void) }
    
}

pub trait TWxModule : TWxObject {
}

pub struct WxMutex { ptr: *mut c_void }
impl TWxMutex for WxMutex { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMutex {
    pub fn from(ptr: *mut c_void) -> WxMutex { WxMutex { ptr: ptr } }
    pub fn null() -> WxMutex { WxMutex::from(0 as *mut c_void) }
    
}

pub trait TWxMutex {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxMutexLocker { ptr: *mut c_void }
impl TWxMutexLocker for WxMutexLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMutexLocker {
    pub fn from(ptr: *mut c_void) -> WxMutexLocker { WxMutexLocker { ptr: ptr } }
    pub fn null() -> WxMutexLocker { WxMutexLocker::from(0 as *mut c_void) }
    
}

pub trait TWxMutexLocker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxNodeBase { ptr: *mut c_void }
impl TWxNodeBase for WxNodeBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxNodeBase {
    pub fn from(ptr: *mut c_void) -> WxNodeBase { WxNodeBase { ptr: ptr } }
    pub fn null() -> WxNodeBase { WxNodeBase::from(0 as *mut c_void) }
    
}

pub trait TWxNodeBase {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxObject { ptr: *mut c_void }
impl TWxObject for WxObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxObject {
    pub fn from(ptr: *mut c_void) -> WxObject { WxObject { ptr: ptr } }
    pub fn null() -> WxObject { WxObject::from(0 as *mut c_void) }
    
}

pub trait TWxObject {
    fn ptr(&self) -> *mut c_void;
    
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.ptr()) }
    }
    fn getClientClosure(&self) -> WxClosure {
        unsafe { WxClosure { ptr: wxObject_GetClientClosure(self.ptr()) } }
    }
    fn setClientClosure<T: TWxClosure>(&self, closure: &T) {
        unsafe { wxObject_SetClientClosure(self.ptr(), closure.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxObject_Delete(self.ptr()) }
    }
    fn getClassInfo(&self) -> WxClassInfo {
        unsafe { WxClassInfo { ptr: wxObject_GetClassInfo(self.ptr()) } }
    }
    fn isKindOf<T: TWxClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxObject_IsKindOf(self.ptr(), classInfo.ptr()) }
    }
    fn isScrolledWindow(&self) -> c_int {
        unsafe { wxObject_IsScrolledWindow(self.ptr()) }
    }
}

pub struct WxObjectRefData { ptr: *mut c_void }
impl TWxObjectRefData for WxObjectRefData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxObjectRefData {
    pub fn from(ptr: *mut c_void) -> WxObjectRefData { WxObjectRefData { ptr: ptr } }
    pub fn null() -> WxObjectRefData { WxObjectRefData::from(0 as *mut c_void) }
    
}

pub trait TWxObjectRefData {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxOutputStream { ptr: *mut c_void }
impl TWxOutputStream for WxOutputStream {}
impl TWxStreamBase for WxOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxOutputStream {
    pub fn from(ptr: *mut c_void) -> WxOutputStream { WxOutputStream { ptr: ptr } }
    pub fn null() -> WxOutputStream { WxOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxOutputStream : TWxStreamBase {
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

pub struct WxPathList { ptr: *mut c_void }
impl TWxPathList for WxPathList {}
impl TWxList for WxPathList {}
impl TWxObject for WxPathList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPathList {
    pub fn from(ptr: *mut c_void) -> WxPathList { WxPathList { ptr: ptr } }
    pub fn null() -> WxPathList { WxPathList::from(0 as *mut c_void) }
    
}

pub trait TWxPathList : TWxList {
}

pub struct WxRegEx { ptr: *mut c_void }
impl TWxRegEx for WxRegEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRegEx {
    pub fn from(ptr: *mut c_void) -> WxRegEx { WxRegEx { ptr: ptr } }
    pub fn null() -> WxRegEx { WxRegEx::from(0 as *mut c_void) }
    
}

pub trait TWxRegEx {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxScopedArray { ptr: *mut c_void }
impl TWxScopedArray for WxScopedArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScopedArray {
    pub fn from(ptr: *mut c_void) -> WxScopedArray { WxScopedArray { ptr: ptr } }
    pub fn null() -> WxScopedArray { WxScopedArray::from(0 as *mut c_void) }
    
}

pub trait TWxScopedArray {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxScopedPtr { ptr: *mut c_void }
impl TWxScopedPtr for WxScopedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScopedPtr {
    pub fn from(ptr: *mut c_void) -> WxScopedPtr { WxScopedPtr { ptr: ptr } }
    pub fn null() -> WxScopedPtr { WxScopedPtr::from(0 as *mut c_void) }
    
}

pub trait TWxScopedPtr {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxSemaphore { ptr: *mut c_void }
impl TWxSemaphore for WxSemaphore { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSemaphore {
    pub fn from(ptr: *mut c_void) -> WxSemaphore { WxSemaphore { ptr: ptr } }
    pub fn null() -> WxSemaphore { WxSemaphore::from(0 as *mut c_void) }
    
}

pub trait TWxSemaphore {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxServer { ptr: *mut c_void }
impl TWxServer for WxServer {}
impl TWxServerBase for WxServer {}
impl TWxObject for WxServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxServer {
    pub fn from(ptr: *mut c_void) -> WxServer { WxServer { ptr: ptr } }
    pub fn null() -> WxServer { WxServer::from(0 as *mut c_void) }
    
}

pub trait TWxServer : TWxServerBase {
}

pub struct WxServerBase { ptr: *mut c_void }
impl TWxServerBase for WxServerBase {}
impl TWxObject for WxServerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxServerBase {
    pub fn from(ptr: *mut c_void) -> WxServerBase { WxServerBase { ptr: ptr } }
    pub fn null() -> WxServerBase { WxServerBase::from(0 as *mut c_void) }
    
}

pub trait TWxServerBase : TWxObject {
}

pub struct WxSingleInstanceChecker { ptr: *mut c_void }
impl TWxSingleInstanceChecker for WxSingleInstanceChecker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSingleInstanceChecker {
    pub fn from(ptr: *mut c_void) -> WxSingleInstanceChecker { WxSingleInstanceChecker { ptr: ptr } }
    pub fn null() -> WxSingleInstanceChecker { WxSingleInstanceChecker::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, name: &str, path: &str) -> c_int {
        let name = wxT(name);
        let path = wxT(path);
        unsafe { wxSingleInstanceChecker_Create(_obj, name.ptr(), path.ptr()) }
    }
    pub fn newDefault() -> WxSingleInstanceChecker {
        unsafe { WxSingleInstanceChecker { ptr: wxSingleInstanceChecker_CreateDefault() } }
    }
}

pub trait TWxSingleInstanceChecker {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.ptr()) }
    }
    fn isAnotherRunning(&self) -> c_int {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.ptr()) }
    }
}

pub struct WxStopWatch { ptr: *mut c_void }
impl TWxStopWatch for WxStopWatch { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStopWatch {
    pub fn from(ptr: *mut c_void) -> WxStopWatch { WxStopWatch { ptr: ptr } }
    pub fn null() -> WxStopWatch { WxStopWatch::from(0 as *mut c_void) }
    
    pub fn new() -> WxStopWatch {
        unsafe { WxStopWatch { ptr: wxStopWatch_Create() } }
    }
}

pub trait TWxStopWatch {
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

pub struct WxStreamBase { ptr: *mut c_void }
impl TWxStreamBase for WxStreamBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStreamBase {
    pub fn from(ptr: *mut c_void) -> WxStreamBase { WxStreamBase { ptr: ptr } }
    pub fn null() -> WxStreamBase { WxStreamBase::from(0 as *mut c_void) }
    
}

pub trait TWxStreamBase {
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

pub struct WxStreamBuffer { ptr: *mut c_void }
impl TWxStreamBuffer for WxStreamBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStreamBuffer {
    pub fn from(ptr: *mut c_void) -> WxStreamBuffer { WxStreamBuffer { ptr: ptr } }
    pub fn null() -> WxStreamBuffer { WxStreamBuffer::from(0 as *mut c_void) }
    
}

pub trait TWxStreamBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxStringBuffer { ptr: *mut c_void }
impl TWxStringBuffer for WxStringBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStringBuffer {
    pub fn from(ptr: *mut c_void) -> WxStringBuffer { WxStringBuffer { ptr: ptr } }
    pub fn null() -> WxStringBuffer { WxStringBuffer::from(0 as *mut c_void) }
    
}

pub trait TWxStringBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxStringClientData { ptr: *mut c_void }
impl TWxStringClientData for WxStringClientData {}
impl TWxClientData for WxStringClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStringClientData {
    pub fn from(ptr: *mut c_void) -> WxStringClientData { WxStringClientData { ptr: ptr } }
    pub fn null() -> WxStringClientData { WxStringClientData::from(0 as *mut c_void) }
    
}

pub trait TWxStringClientData : TWxClientData {
}

pub struct WxStringList { ptr: *mut c_void }
impl TWxStringList for WxStringList {}
impl TWxList for WxStringList {}
impl TWxObject for WxStringList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStringList {
    pub fn from(ptr: *mut c_void) -> WxStringList { WxStringList { ptr: ptr } }
    pub fn null() -> WxStringList { WxStringList::from(0 as *mut c_void) }
    
}

pub trait TWxStringList : TWxList {
}

pub struct WxStringTokenizer { ptr: *mut c_void }
impl TWxStringTokenizer for WxStringTokenizer {}
impl TWxObject for WxStringTokenizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStringTokenizer {
    pub fn from(ptr: *mut c_void) -> WxStringTokenizer { WxStringTokenizer { ptr: ptr } }
    pub fn null() -> WxStringTokenizer { WxStringTokenizer::from(0 as *mut c_void) }
    
}

pub trait TWxStringTokenizer : TWxObject {
}

pub struct WxSystemOptions { ptr: *mut c_void }
impl TWxSystemOptions for WxSystemOptions {}
impl TWxObject for WxSystemOptions { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSystemOptions {
    pub fn from(ptr: *mut c_void) -> WxSystemOptions { WxSystemOptions { ptr: ptr } }
    pub fn null() -> WxSystemOptions { WxSystemOptions::from(0 as *mut c_void) }
    
}

pub trait TWxSystemOptions : TWxObject {
}

pub struct WxTempFile { ptr: *mut c_void }
impl TWxTempFile for WxTempFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTempFile {
    pub fn from(ptr: *mut c_void) -> WxTempFile { WxTempFile { ptr: ptr } }
    pub fn null() -> WxTempFile { WxTempFile::from(0 as *mut c_void) }
    
}

pub trait TWxTempFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxTextFile { ptr: *mut c_void }
impl TWxTextFile for WxTextFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextFile {
    pub fn from(ptr: *mut c_void) -> WxTextFile { WxTextFile { ptr: ptr } }
    pub fn null() -> WxTextFile { WxTextFile::from(0 as *mut c_void) }
    
}

pub trait TWxTextFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxTextInputStream { ptr: *mut c_void }
impl TWxTextInputStream for WxTextInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextInputStream {
    pub fn from(ptr: *mut c_void) -> WxTextInputStream { WxTextInputStream { ptr: ptr } }
    pub fn null() -> WxTextInputStream { WxTextInputStream::from(0 as *mut c_void) }
    
    pub fn new<T: TWxInputStream>(inputStream: &T, sep: &str) -> WxTextInputStream {
        let sep = wxT(sep);
        unsafe { WxTextInputStream { ptr: wxTextInputStream_Create(inputStream.ptr(), sep.ptr()) } }
    }
}

pub trait TWxTextInputStream {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.ptr()) }
    }
    fn readLine(&self) -> ~str {
        unsafe { WxString { ptr: wxTextInputStream_ReadLine(self.ptr()) }.to_str() }
    }
}

pub struct WxTextOutputStream { ptr: *mut c_void }
impl TWxTextOutputStream for WxTextOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextOutputStream {
    pub fn from(ptr: *mut c_void) -> WxTextOutputStream { WxTextOutputStream { ptr: ptr } }
    pub fn null() -> WxTextOutputStream { WxTextOutputStream::from(0 as *mut c_void) }
    
    pub fn new<T: TWxOutputStream>(outputStream: &T, mode: c_int) -> WxTextOutputStream {
        unsafe { WxTextOutputStream { ptr: wxTextOutputStream_Create(outputStream.ptr(), mode) } }
    }
}

pub trait TWxTextOutputStream {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.ptr()) }
    }
    fn writeString(&self, txt: &str) {
        let txt = wxT(txt);
        unsafe { wxTextOutputStream_WriteString(self.ptr(), txt.ptr()) }
    }
}

pub struct WxThread { ptr: *mut c_void }
impl TWxThread for WxThread { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxThread {
    pub fn from(ptr: *mut c_void) -> WxThread { WxThread { ptr: ptr } }
    pub fn null() -> WxThread { WxThread::from(0 as *mut c_void) }
    
}

pub trait TWxThread {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxTime { ptr: *mut c_void }
impl TWxTime for WxTime {}
impl TWxObject for WxTime { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTime {
    pub fn from(ptr: *mut c_void) -> WxTime { WxTime { ptr: ptr } }
    pub fn null() -> WxTime { WxTime::from(0 as *mut c_void) }
    
}

pub trait TWxTime : TWxObject {
}

pub struct WxTimeSpan { ptr: *mut c_void }
impl TWxTimeSpan for WxTimeSpan { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTimeSpan {
    pub fn from(ptr: *mut c_void) -> WxTimeSpan { WxTimeSpan { ptr: ptr } }
    pub fn null() -> WxTimeSpan { WxTimeSpan::from(0 as *mut c_void) }
    
}

pub trait TWxTimeSpan {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxVariant { ptr: *mut c_void }
impl TWxVariant for WxVariant {}
impl TWxObject for WxVariant { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxVariant {
    pub fn from(ptr: *mut c_void) -> WxVariant { WxVariant { ptr: ptr } }
    pub fn null() -> WxVariant { WxVariant::from(0 as *mut c_void) }
    
}

pub trait TWxVariant : TWxObject {
}

pub struct WxVariantData { ptr: *mut c_void }
impl TWxVariantData for WxVariantData {}
impl TWxObject for WxVariantData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxVariantData {
    pub fn from(ptr: *mut c_void) -> WxVariantData { WxVariantData { ptr: ptr } }
    pub fn null() -> WxVariantData { WxVariantData::from(0 as *mut c_void) }
    
}

pub trait TWxVariantData : TWxObject {
}

pub struct WxZipInputStream { ptr: *mut c_void }
impl TWxZipInputStream for WxZipInputStream {}
impl TWxInputStream for WxZipInputStream {}
impl TWxStreamBase for WxZipInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxZipInputStream {
    pub fn from(ptr: *mut c_void) -> WxZipInputStream { WxZipInputStream { ptr: ptr } }
    pub fn null() -> WxZipInputStream { WxZipInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxZipInputStream : TWxInputStream {
}

pub struct WxZlibInputStream { ptr: *mut c_void }
impl TWxZlibInputStream for WxZlibInputStream {}
impl TWxFilterInputStream for WxZlibInputStream {}
impl TWxInputStream for WxZlibInputStream {}
impl TWxStreamBase for WxZlibInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxZlibInputStream {
    pub fn from(ptr: *mut c_void) -> WxZlibInputStream { WxZlibInputStream { ptr: ptr } }
    pub fn null() -> WxZlibInputStream { WxZlibInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxZlibInputStream : TWxFilterInputStream {
}

pub struct WxZlibOutputStream { ptr: *mut c_void }
impl TWxZlibOutputStream for WxZlibOutputStream {}
impl TWxFilterOutputStream for WxZlibOutputStream {}
impl TWxOutputStream for WxZlibOutputStream {}
impl TWxStreamBase for WxZlibOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxZlibOutputStream {
    pub fn from(ptr: *mut c_void) -> WxZlibOutputStream { WxZlibOutputStream { ptr: ptr } }
    pub fn null() -> WxZlibOutputStream { WxZlibOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxZlibOutputStream : TWxFilterOutputStream {
}

pub struct WxMemoryBuffer { ptr: *mut c_void }
impl TWxMemoryBuffer for WxMemoryBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMemoryBuffer {
    pub fn from(ptr: *mut c_void) -> WxMemoryBuffer { WxMemoryBuffer { ptr: ptr } }
    pub fn null() -> WxMemoryBuffer { WxMemoryBuffer::from(0 as *mut c_void) }
    
}

pub trait TWxMemoryBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxFileConfig { ptr: *mut c_void }
impl TWxFileConfig for WxFileConfig {}
impl TWxConfigBase for WxFileConfig { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileConfig {
    pub fn from(ptr: *mut c_void) -> WxFileConfig { WxFileConfig { ptr: ptr } }
    pub fn null() -> WxFileConfig { WxFileConfig::from(0 as *mut c_void) }
    
    pub fn new<T: TWxInputStream>(inp: &T) -> WxFileConfig {
        unsafe { WxFileConfig { ptr: wxFileConfig_Create(inp.ptr()) } }
    }
}

pub trait TWxFileConfig : TWxConfigBase {
}

