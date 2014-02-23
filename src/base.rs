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

pub struct RustClient { ptr: *mut c_void }
impl TRustClient for RustClient {}
impl TClient for RustClient {}
impl TClientBase for RustClient {}
impl TObject for RustClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustClient {
    pub fn from(ptr: *mut c_void) -> RustClient { RustClient { ptr: ptr } }
    pub fn null() -> RustClient { RustClient::from(0 as *mut c_void) }
    
}

pub trait TRustClient : TClient {
}

pub struct RustConnection { ptr: *mut c_void }
impl TRustConnection for RustConnection {}
impl TConnection for RustConnection {}
impl TConnectionBase for RustConnection {}
impl TObject for RustConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustConnection {
    pub fn from(ptr: *mut c_void) -> RustConnection { RustConnection { ptr: ptr } }
    pub fn null() -> RustConnection { RustConnection::from(0 as *mut c_void) }
    
}

pub trait TRustConnection : TConnection {
}

pub struct RustLocale { ptr: *mut c_void }
impl TRustLocale for RustLocale {}
impl TLocale for RustLocale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustLocale {
    pub fn from(ptr: *mut c_void) -> RustLocale { RustLocale { ptr: ptr } }
    pub fn null() -> RustLocale { RustLocale::from(0 as *mut c_void) }
    
}

pub trait TRustLocale : TLocale {
}

pub struct RustServer { ptr: *mut c_void }
impl TRustServer for RustServer {}
impl TServer for RustServer {}
impl TServerBase for RustServer {}
impl TObject for RustServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustServer {
    pub fn from(ptr: *mut c_void) -> RustServer { RustServer { ptr: ptr } }
    pub fn null() -> RustServer { RustServer::from(0 as *mut c_void) }
    
}

pub trait TRustServer : TServer {
}

pub struct Array { ptr: *mut c_void }
impl TArray for Array { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Array {
    pub fn from(ptr: *mut c_void) -> Array { Array { ptr: ptr } }
    pub fn null() -> Array { Array::from(0 as *mut c_void) }
    
}

pub trait TArray {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct ArrayString { ptr: *mut c_void }
impl TArrayString for ArrayString {}
impl TArray for ArrayString { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ArrayString {
    pub fn from(ptr: *mut c_void) -> ArrayString { ArrayString { ptr: ptr } }
    pub fn null() -> ArrayString { ArrayString::from(0 as *mut c_void) }
    
}

pub trait TArrayString : TArray {
}

pub struct BufferedInputStream { ptr: *mut c_void }
impl TBufferedInputStream for BufferedInputStream {}
impl TFilterInputStream for BufferedInputStream {}
impl TInputStream for BufferedInputStream {}
impl TStreamBase for BufferedInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedInputStream {
    pub fn from(ptr: *mut c_void) -> BufferedInputStream { BufferedInputStream { ptr: ptr } }
    pub fn null() -> BufferedInputStream { BufferedInputStream::from(0 as *mut c_void) }
    
}

pub trait TBufferedInputStream : TFilterInputStream {
}

pub struct BufferedOutputStream { ptr: *mut c_void }
impl TBufferedOutputStream for BufferedOutputStream {}
impl TFilterOutputStream for BufferedOutputStream {}
impl TOutputStream for BufferedOutputStream {}
impl TStreamBase for BufferedOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedOutputStream {
    pub fn from(ptr: *mut c_void) -> BufferedOutputStream { BufferedOutputStream { ptr: ptr } }
    pub fn null() -> BufferedOutputStream { BufferedOutputStream::from(0 as *mut c_void) }
    
}

pub trait TBufferedOutputStream : TFilterOutputStream {
}

pub struct CSConv { ptr: *mut c_void }
impl TCSConv for CSConv {}
impl TMBConv for CSConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CSConv {
    pub fn from(ptr: *mut c_void) -> CSConv { CSConv { ptr: ptr } }
    pub fn null() -> CSConv { CSConv::from(0 as *mut c_void) }
    
}

pub trait TCSConv : TMBConv {
}

pub struct ClassInfo { ptr: *mut c_void }
impl TClassInfo for ClassInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClassInfo {
    pub fn from(ptr: *mut c_void) -> ClassInfo { ClassInfo { ptr: ptr } }
    pub fn null() -> ClassInfo { ClassInfo::from(0 as *mut c_void) }
    
    pub fn findClass(_txt: &str) -> ClassInfo {
        let _txt = wxT(_txt);
        unsafe { ClassInfo { ptr: wxClassInfo_FindClass(_txt.ptr()) } }
    }
}

pub trait TClassInfo {
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
    fn isKindOfEx<T: TClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxClassInfo_IsKindOfEx(self.ptr(), classInfo.ptr()) }
    }
}

pub struct Client { ptr: *mut c_void }
impl TClient for Client {}
impl TClientBase for Client {}
impl TObject for Client { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Client {
    pub fn from(ptr: *mut c_void) -> Client { Client { ptr: ptr } }
    pub fn null() -> Client { Client::from(0 as *mut c_void) }
    
}

pub trait TClient : TClientBase {
}

pub struct ClientBase { ptr: *mut c_void }
impl TClientBase for ClientBase {}
impl TObject for ClientBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientBase {
    pub fn from(ptr: *mut c_void) -> ClientBase { ClientBase { ptr: ptr } }
    pub fn null() -> ClientBase { ClientBase::from(0 as *mut c_void) }
    
}

pub trait TClientBase : TObject {
}

pub struct ClientData { ptr: *mut c_void }
impl TClientData for ClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientData {
    pub fn from(ptr: *mut c_void) -> ClientData { ClientData { ptr: ptr } }
    pub fn null() -> ClientData { ClientData::from(0 as *mut c_void) }
    
}

pub trait TClientData {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct ClientDataContainer { ptr: *mut c_void }
impl TClientDataContainer for ClientDataContainer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientDataContainer {
    pub fn from(ptr: *mut c_void) -> ClientDataContainer { ClientDataContainer { ptr: ptr } }
    pub fn null() -> ClientDataContainer { ClientDataContainer::from(0 as *mut c_void) }
    
}

pub trait TClientDataContainer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Closure { ptr: *mut c_void }
impl TClosure for Closure {}
impl TObject for Closure { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Closure {
    pub fn from(ptr: *mut c_void) -> Closure { Closure { ptr: ptr } }
    pub fn null() -> Closure { Closure::from(0 as *mut c_void) }
    
    pub fn new(_fun_CEvent: *mut c_void, _data: *mut c_void) -> Closure {
        unsafe { Closure { ptr: wxClosure_Create(_fun_CEvent, _data) } }
    }
}

pub trait TClosure : TObject {
    fn getData(&self) -> *mut c_void {
        unsafe { wxClosure_GetData(self.ptr()) }
    }
}

pub struct CommandLineParser { ptr: *mut c_void }
impl TCommandLineParser for CommandLineParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CommandLineParser {
    pub fn from(ptr: *mut c_void) -> CommandLineParser { CommandLineParser { ptr: ptr } }
    pub fn null() -> CommandLineParser { CommandLineParser::from(0 as *mut c_void) }
    
}

pub trait TCommandLineParser {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Condition { ptr: *mut c_void }
impl TCondition for Condition { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Condition {
    pub fn from(ptr: *mut c_void) -> Condition { Condition { ptr: ptr } }
    pub fn null() -> Condition { Condition::from(0 as *mut c_void) }
    
}

pub trait TCondition {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct ConfigBase { ptr: *mut c_void }
impl TConfigBase for ConfigBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ConfigBase {
    pub fn from(ptr: *mut c_void) -> ConfigBase { ConfigBase { ptr: ptr } }
    pub fn null() -> ConfigBase { ConfigBase::from(0 as *mut c_void) }
    
    pub fn new() -> ConfigBase {
        unsafe { ConfigBase { ptr: wxConfigBase_Create() } }
    }
    pub fn get() -> ConfigBase {
        unsafe { ConfigBase { ptr: wxConfigBase_Get() } }
    }
    pub fn set<T: TConfigBase>(self_: &T) {
        unsafe { wxConfigBase_Set(self_.ptr()) }
    }
}

pub trait TConfigBase {
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

pub struct Connection { ptr: *mut c_void }
impl TConnection for Connection {}
impl TConnectionBase for Connection {}
impl TObject for Connection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Connection {
    pub fn from(ptr: *mut c_void) -> Connection { Connection { ptr: ptr } }
    pub fn null() -> Connection { Connection::from(0 as *mut c_void) }
    
}

pub trait TConnection : TConnectionBase {
}

pub struct ConnectionBase { ptr: *mut c_void }
impl TConnectionBase for ConnectionBase {}
impl TObject for ConnectionBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ConnectionBase {
    pub fn from(ptr: *mut c_void) -> ConnectionBase { ConnectionBase { ptr: ptr } }
    pub fn null() -> ConnectionBase { ConnectionBase::from(0 as *mut c_void) }
    
}

pub trait TConnectionBase : TObject {
}

pub struct CountingOutputStream { ptr: *mut c_void }
impl TCountingOutputStream for CountingOutputStream {}
impl TOutputStream for CountingOutputStream {}
impl TStreamBase for CountingOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CountingOutputStream {
    pub fn from(ptr: *mut c_void) -> CountingOutputStream { CountingOutputStream { ptr: ptr } }
    pub fn null() -> CountingOutputStream { CountingOutputStream::from(0 as *mut c_void) }
    
}

pub trait TCountingOutputStream : TOutputStream {
}

pub struct CriticalSection { ptr: *mut c_void }
impl TCriticalSection for CriticalSection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CriticalSection {
    pub fn from(ptr: *mut c_void) -> CriticalSection { CriticalSection { ptr: ptr } }
    pub fn null() -> CriticalSection { CriticalSection::from(0 as *mut c_void) }
    
}

pub trait TCriticalSection {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct CriticalSectionLocker { ptr: *mut c_void }
impl TCriticalSectionLocker for CriticalSectionLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CriticalSectionLocker {
    pub fn from(ptr: *mut c_void) -> CriticalSectionLocker { CriticalSectionLocker { ptr: ptr } }
    pub fn null() -> CriticalSectionLocker { CriticalSectionLocker::from(0 as *mut c_void) }
    
}

pub trait TCriticalSectionLocker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DDEClient { ptr: *mut c_void }
impl TDDEClient for DDEClient {}
impl TClientBase for DDEClient {}
impl TObject for DDEClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DDEClient {
    pub fn from(ptr: *mut c_void) -> DDEClient { DDEClient { ptr: ptr } }
    pub fn null() -> DDEClient { DDEClient::from(0 as *mut c_void) }
    
}

pub trait TDDEClient : TClientBase {
}

pub struct DDEConnection { ptr: *mut c_void }
impl TDDEConnection for DDEConnection {}
impl TConnectionBase for DDEConnection {}
impl TObject for DDEConnection { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DDEConnection {
    pub fn from(ptr: *mut c_void) -> DDEConnection { DDEConnection { ptr: ptr } }
    pub fn null() -> DDEConnection { DDEConnection::from(0 as *mut c_void) }
    
}

pub trait TDDEConnection : TConnectionBase {
}

pub struct DDEServer { ptr: *mut c_void }
impl TDDEServer for DDEServer {}
impl TServerBase for DDEServer {}
impl TObject for DDEServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DDEServer {
    pub fn from(ptr: *mut c_void) -> DDEServer { DDEServer { ptr: ptr } }
    pub fn null() -> DDEServer { DDEServer::from(0 as *mut c_void) }
    
}

pub trait TDDEServer : TServerBase {
}

pub struct DataInputStream { ptr: *mut c_void }
impl TDataInputStream for DataInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataInputStream {
    pub fn from(ptr: *mut c_void) -> DataInputStream { DataInputStream { ptr: ptr } }
    pub fn null() -> DataInputStream { DataInputStream::from(0 as *mut c_void) }
    
}

pub trait TDataInputStream {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DataOutputStream { ptr: *mut c_void }
impl TDataOutputStream for DataOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataOutputStream {
    pub fn from(ptr: *mut c_void) -> DataOutputStream { DataOutputStream { ptr: ptr } }
    pub fn null() -> DataOutputStream { DataOutputStream::from(0 as *mut c_void) }
    
}

pub trait TDataOutputStream {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DateTime { ptr: *mut c_void }
impl TDateTime for DateTime { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DateTime {
    pub fn from(ptr: *mut c_void) -> DateTime { DateTime { ptr: ptr } }
    pub fn null() -> DateTime { DateTime::from(0 as *mut c_void) }
    
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    pub fn new() -> DateTime {
        unsafe { DateTime { ptr: wxDateTime_Create() } }
    }
    pub fn getAmString() -> ~str {
        unsafe { WxString { ptr: wxDateTime_GetAmString() }.to_str() }
    }
    pub fn getBeginDST<T: TDateTime>(year: c_int, country: c_int, dt: &T) {
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
    pub fn getEndDST<T: TDateTime>(year: c_int, country: c_int, dt: &T) {
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

pub trait TDateTime {
    fn ptr(&self) -> *mut c_void;
    
    fn addDate<T: TDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_AddDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.ptr(), _yrs, _mnt, _wek, _day) }
    }
    fn addTime<T: TDateTime>(&self, diff: *mut c_void, _ref: &T) {
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
    fn getLastMonthDay<T: TDateTime>(&self, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastMonthDay(self.ptr(), month, year, _ref.ptr()) }
    }
    fn getLastWeekDay<T: TDateTime>(&self, weekday: c_int, month: c_int, year: c_int, _ref: &T) {
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
    fn getNextWeekDay<T: TDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetNextWeekDay(self.ptr(), weekday, _ref.ptr()) }
    }
    fn getPrevWeekDay<T: TDateTime>(&self, weekday: c_int, _ref: &T) {
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
    fn getWeekDay<T: TDateTime>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDay(self.ptr(), weekday, n, month, year, _ref.ptr()) }
    }
    fn getWeekDayInSameWeek<T: TDateTime>(&self, weekday: c_int, _ref: &T) {
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
    fn isBetween<T: TDateTime, U: TDateTime>(&self, t1: &T, t2: &U) -> c_int {
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
    fn isEqualUpTo<T: TDateTime>(&self, dt: &T, ts: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsEqualUpTo(self.ptr(), dt.ptr(), ts) }
    }
    fn isLaterThan(&self, datetime: *mut c_void) -> c_int {
        unsafe { wxDateTime_IsLaterThan(self.ptr(), datetime) }
    }
    fn isSameDate<T: TDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameDate(self.ptr(), dt.ptr()) }
    }
    fn isSameTime<T: TDateTime>(&self, dt: &T) -> c_int {
        unsafe { wxDateTime_IsSameTime(self.ptr(), dt.ptr()) }
    }
    fn isStrictlyBetween<T: TDateTime, U: TDateTime>(&self, t1: &T, t2: &U) -> c_int {
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
    fn parseTime<T: TTime>(&self, time: &T) -> *mut c_void {
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
    fn subtractDate<T: TDateTime>(&self, diff: *mut c_void, _ref: &T) {
        unsafe { wxDateTime_SubtractDate(self.ptr(), diff, _ref.ptr()) }
    }
    fn subtractTime<T: TDateTime>(&self, diff: *mut c_void, _ref: &T) {
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

pub struct DebugContext { ptr: *mut c_void }
impl TDebugContext for DebugContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DebugContext {
    pub fn from(ptr: *mut c_void) -> DebugContext { DebugContext { ptr: ptr } }
    pub fn null() -> DebugContext { DebugContext::from(0 as *mut c_void) }
    
}

pub trait TDebugContext {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DirTraverser { ptr: *mut c_void }
impl TDirTraverser for DirTraverser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DirTraverser {
    pub fn from(ptr: *mut c_void) -> DirTraverser { DirTraverser { ptr: ptr } }
    pub fn null() -> DirTraverser { DirTraverser::from(0 as *mut c_void) }
    
}

pub trait TDirTraverser {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DllLoader { ptr: *mut c_void }
impl TDllLoader for DllLoader { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DllLoader {
    pub fn from(ptr: *mut c_void) -> DllLoader { DllLoader { ptr: ptr } }
    pub fn null() -> DllLoader { DllLoader::from(0 as *mut c_void) }
    
}

pub trait TDllLoader {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DynamicLibrary { ptr: *mut c_void }
impl TDynamicLibrary for DynamicLibrary { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicLibrary {
    pub fn from(ptr: *mut c_void) -> DynamicLibrary { DynamicLibrary { ptr: ptr } }
    pub fn null() -> DynamicLibrary { DynamicLibrary::from(0 as *mut c_void) }
    
}

pub trait TDynamicLibrary {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct EncodingConverter { ptr: *mut c_void }
impl TEncodingConverter for EncodingConverter {}
impl TObject for EncodingConverter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EncodingConverter {
    pub fn from(ptr: *mut c_void) -> EncodingConverter { EncodingConverter { ptr: ptr } }
    pub fn null() -> EncodingConverter { EncodingConverter::from(0 as *mut c_void) }
    
    pub fn new() -> EncodingConverter {
        unsafe { EncodingConverter { ptr: wxEncodingConverter_Create() } }
    }
}

pub trait TEncodingConverter : TObject {
    fn convert(&self, input: *mut c_void, output: *mut c_void) {
        unsafe { wxEncodingConverter_Convert(self.ptr(), input, output) }
    }
    fn getAllEquivalents<T: TList>(&self, enc: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.ptr(), enc, _lst.ptr()) }
    }
    fn getPlatformEquivalents<T: TList>(&self, enc: c_int, platform: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.ptr(), enc, platform, _lst.ptr()) }
    }
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.ptr(), input_enc, output_enc, method) }
    }
}

pub struct FFile { ptr: *mut c_void }
impl TFFile for FFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FFile {
    pub fn from(ptr: *mut c_void) -> FFile { FFile { ptr: ptr } }
    pub fn null() -> FFile { FFile::from(0 as *mut c_void) }
    
}

pub trait TFFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct FFileInputStream { ptr: *mut c_void }
impl TFFileInputStream for FFileInputStream {}
impl TInputStream for FFileInputStream {}
impl TStreamBase for FFileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FFileInputStream {
    pub fn from(ptr: *mut c_void) -> FFileInputStream { FFileInputStream { ptr: ptr } }
    pub fn null() -> FFileInputStream { FFileInputStream::from(0 as *mut c_void) }
    
}

pub trait TFFileInputStream : TInputStream {
}

pub struct FFileOutputStream { ptr: *mut c_void }
impl TFFileOutputStream for FFileOutputStream {}
impl TOutputStream for FFileOutputStream {}
impl TStreamBase for FFileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FFileOutputStream {
    pub fn from(ptr: *mut c_void) -> FFileOutputStream { FFileOutputStream { ptr: ptr } }
    pub fn null() -> FFileOutputStream { FFileOutputStream::from(0 as *mut c_void) }
    
}

pub trait TFFileOutputStream : TOutputStream {
}

pub struct FSFile { ptr: *mut c_void }
impl TFSFile for FSFile {}
impl TObject for FSFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FSFile {
    pub fn from(ptr: *mut c_void) -> FSFile { FSFile { ptr: ptr } }
    pub fn null() -> FSFile { FSFile::from(0 as *mut c_void) }
    
}

pub trait TFSFile : TObject {
}

pub struct FileInputStream { ptr: *mut c_void }
impl TFileInputStream for FileInputStream {}
impl TInputStream for FileInputStream {}
impl TStreamBase for FileInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileInputStream {
    pub fn from(ptr: *mut c_void) -> FileInputStream { FileInputStream { ptr: ptr } }
    pub fn null() -> FileInputStream { FileInputStream::from(0 as *mut c_void) }
    
}

pub trait TFileInputStream : TInputStream {
}

pub struct FileName { ptr: *mut c_void }
impl TFileName for FileName { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileName {
    pub fn from(ptr: *mut c_void) -> FileName { FileName { ptr: ptr } }
    pub fn null() -> FileName { FileName::from(0 as *mut c_void) }
    
}

pub trait TFileName {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct FileOutputStream { ptr: *mut c_void }
impl TFileOutputStream for FileOutputStream {}
impl TOutputStream for FileOutputStream {}
impl TStreamBase for FileOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileOutputStream {
    pub fn from(ptr: *mut c_void) -> FileOutputStream { FileOutputStream { ptr: ptr } }
    pub fn null() -> FileOutputStream { FileOutputStream::from(0 as *mut c_void) }
    
}

pub trait TFileOutputStream : TOutputStream {
}

pub struct FileSystem { ptr: *mut c_void }
impl TFileSystem for FileSystem {}
impl TObject for FileSystem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileSystem {
    pub fn from(ptr: *mut c_void) -> FileSystem { FileSystem { ptr: ptr } }
    pub fn null() -> FileSystem { FileSystem::from(0 as *mut c_void) }
    
}

pub trait TFileSystem : TObject {
}

pub struct FileSystemHandler { ptr: *mut c_void }
impl TFileSystemHandler for FileSystemHandler {}
impl TObject for FileSystemHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileSystemHandler {
    pub fn from(ptr: *mut c_void) -> FileSystemHandler { FileSystemHandler { ptr: ptr } }
    pub fn null() -> FileSystemHandler { FileSystemHandler::from(0 as *mut c_void) }
    
}

pub trait TFileSystemHandler : TObject {
}

pub struct FilterInputStream { ptr: *mut c_void }
impl TFilterInputStream for FilterInputStream {}
impl TInputStream for FilterInputStream {}
impl TStreamBase for FilterInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FilterInputStream {
    pub fn from(ptr: *mut c_void) -> FilterInputStream { FilterInputStream { ptr: ptr } }
    pub fn null() -> FilterInputStream { FilterInputStream::from(0 as *mut c_void) }
    
}

pub trait TFilterInputStream : TInputStream {
}

pub struct FilterOutputStream { ptr: *mut c_void }
impl TFilterOutputStream for FilterOutputStream {}
impl TOutputStream for FilterOutputStream {}
impl TStreamBase for FilterOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FilterOutputStream {
    pub fn from(ptr: *mut c_void) -> FilterOutputStream { FilterOutputStream { ptr: ptr } }
    pub fn null() -> FilterOutputStream { FilterOutputStream::from(0 as *mut c_void) }
    
}

pub trait TFilterOutputStream : TOutputStream {
}

pub struct InputStream { ptr: *mut c_void }
impl TInputStream for InputStream {}
impl TStreamBase for InputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InputStream {
    pub fn from(ptr: *mut c_void) -> InputStream { InputStream { ptr: ptr } }
    pub fn null() -> InputStream { InputStream::from(0 as *mut c_void) }
    
}

pub trait TInputStream : TStreamBase {
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

pub struct List { ptr: *mut c_void }
impl TList for List {}
impl TObject for List { fn ptr(&self) -> *mut c_void { self.ptr } }

impl List {
    pub fn from(ptr: *mut c_void) -> List { List { ptr: ptr } }
    pub fn null() -> List { List::from(0 as *mut c_void) }
    
}

pub trait TList : TObject {
}

pub struct Locale { ptr: *mut c_void }
impl TLocale for Locale { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Locale {
    pub fn from(ptr: *mut c_void) -> Locale { Locale { ptr: ptr } }
    pub fn null() -> Locale { Locale::from(0 as *mut c_void) }
    
    pub fn new(_name: c_int, _flags: c_int) -> Locale {
        unsafe { Locale { ptr: wxLocale_Create(_name, _flags) } }
    }
}

pub trait TLocale {
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
        unsafe { Locale { ptr: wxLocale_GetLocale(self.ptr()) } }
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

pub struct LongLong { ptr: *mut c_void }
impl TLongLong for LongLong { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LongLong {
    pub fn from(ptr: *mut c_void) -> LongLong { LongLong { ptr: ptr } }
    pub fn null() -> LongLong { LongLong::from(0 as *mut c_void) }
    
}

pub trait TLongLong {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct MBConv { ptr: *mut c_void }
impl TMBConv for MBConv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConv {
    pub fn from(ptr: *mut c_void) -> MBConv { MBConv { ptr: ptr } }
    pub fn null() -> MBConv { MBConv::from(0 as *mut c_void) }
    
}

pub trait TMBConv {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct MBConvUTF7 { ptr: *mut c_void }
impl TMBConvUTF7 for MBConvUTF7 {}
impl TMBConv for MBConvUTF7 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvUTF7 {
    pub fn from(ptr: *mut c_void) -> MBConvUTF7 { MBConvUTF7 { ptr: ptr } }
    pub fn null() -> MBConvUTF7 { MBConvUTF7::from(0 as *mut c_void) }
    
}

pub trait TMBConvUTF7 : TMBConv {
}

pub struct MBConvUTF8 { ptr: *mut c_void }
impl TMBConvUTF8 for MBConvUTF8 {}
impl TMBConv for MBConvUTF8 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvUTF8 {
    pub fn from(ptr: *mut c_void) -> MBConvUTF8 { MBConvUTF8 { ptr: ptr } }
    pub fn null() -> MBConvUTF8 { MBConvUTF8::from(0 as *mut c_void) }
    
}

pub trait TMBConvUTF8 : TMBConv {
}

pub struct MemoryFSHandler { ptr: *mut c_void }
impl TMemoryFSHandler for MemoryFSHandler {}
impl TFileSystemHandler for MemoryFSHandler {}
impl TObject for MemoryFSHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryFSHandler {
    pub fn from(ptr: *mut c_void) -> MemoryFSHandler { MemoryFSHandler { ptr: ptr } }
    pub fn null() -> MemoryFSHandler { MemoryFSHandler::from(0 as *mut c_void) }
    
}

pub trait TMemoryFSHandler : TFileSystemHandler {
}

pub struct MemoryInputStream { ptr: *mut c_void }
impl TMemoryInputStream for MemoryInputStream {}
impl TInputStream for MemoryInputStream {}
impl TStreamBase for MemoryInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryInputStream {
    pub fn from(ptr: *mut c_void) -> MemoryInputStream { MemoryInputStream { ptr: ptr } }
    pub fn null() -> MemoryInputStream { MemoryInputStream::from(0 as *mut c_void) }
    
}

pub trait TMemoryInputStream : TInputStream {
}

pub struct MemoryOutputStream { ptr: *mut c_void }
impl TMemoryOutputStream for MemoryOutputStream {}
impl TOutputStream for MemoryOutputStream {}
impl TStreamBase for MemoryOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryOutputStream {
    pub fn from(ptr: *mut c_void) -> MemoryOutputStream { MemoryOutputStream { ptr: ptr } }
    pub fn null() -> MemoryOutputStream { MemoryOutputStream::from(0 as *mut c_void) }
    
}

pub trait TMemoryOutputStream : TOutputStream {
}

pub struct Module { ptr: *mut c_void }
impl TModule for Module {}
impl TObject for Module { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Module {
    pub fn from(ptr: *mut c_void) -> Module { Module { ptr: ptr } }
    pub fn null() -> Module { Module::from(0 as *mut c_void) }
    
}

pub trait TModule : TObject {
}

pub struct Mutex { ptr: *mut c_void }
impl TMutex for Mutex { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Mutex {
    pub fn from(ptr: *mut c_void) -> Mutex { Mutex { ptr: ptr } }
    pub fn null() -> Mutex { Mutex::from(0 as *mut c_void) }
    
}

pub trait TMutex {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct MutexLocker { ptr: *mut c_void }
impl TMutexLocker for MutexLocker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MutexLocker {
    pub fn from(ptr: *mut c_void) -> MutexLocker { MutexLocker { ptr: ptr } }
    pub fn null() -> MutexLocker { MutexLocker::from(0 as *mut c_void) }
    
}

pub trait TMutexLocker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct NodeBase { ptr: *mut c_void }
impl TNodeBase for NodeBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NodeBase {
    pub fn from(ptr: *mut c_void) -> NodeBase { NodeBase { ptr: ptr } }
    pub fn null() -> NodeBase { NodeBase::from(0 as *mut c_void) }
    
}

pub trait TNodeBase {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Object { ptr: *mut c_void }
impl TObject for Object { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Object {
    pub fn from(ptr: *mut c_void) -> Object { Object { ptr: ptr } }
    pub fn null() -> Object { Object::from(0 as *mut c_void) }
    
}

pub trait TObject {
    fn ptr(&self) -> *mut c_void;
    
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.ptr()) }
    }
    fn getClientClosure(&self) -> Closure {
        unsafe { Closure { ptr: wxObject_GetClientClosure(self.ptr()) } }
    }
    fn setClientClosure<T: TClosure>(&self, closure: &T) {
        unsafe { wxObject_SetClientClosure(self.ptr(), closure.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxObject_Delete(self.ptr()) }
    }
    fn getClassInfo(&self) -> ClassInfo {
        unsafe { ClassInfo { ptr: wxObject_GetClassInfo(self.ptr()) } }
    }
    fn isKindOf<T: TClassInfo>(&self, classInfo: &T) -> c_int {
        unsafe { wxObject_IsKindOf(self.ptr(), classInfo.ptr()) }
    }
    fn isScrolledWindow(&self) -> c_int {
        unsafe { wxObject_IsScrolledWindow(self.ptr()) }
    }
}

pub struct ObjectRefData { ptr: *mut c_void }
impl TObjectRefData for ObjectRefData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ObjectRefData {
    pub fn from(ptr: *mut c_void) -> ObjectRefData { ObjectRefData { ptr: ptr } }
    pub fn null() -> ObjectRefData { ObjectRefData::from(0 as *mut c_void) }
    
}

pub trait TObjectRefData {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct OutputStream { ptr: *mut c_void }
impl TOutputStream for OutputStream {}
impl TStreamBase for OutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl OutputStream {
    pub fn from(ptr: *mut c_void) -> OutputStream { OutputStream { ptr: ptr } }
    pub fn null() -> OutputStream { OutputStream::from(0 as *mut c_void) }
    
}

pub trait TOutputStream : TStreamBase {
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

pub struct PathList { ptr: *mut c_void }
impl TPathList for PathList {}
impl TList for PathList {}
impl TObject for PathList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PathList {
    pub fn from(ptr: *mut c_void) -> PathList { PathList { ptr: ptr } }
    pub fn null() -> PathList { PathList::from(0 as *mut c_void) }
    
}

pub trait TPathList : TList {
}

pub struct RegEx { ptr: *mut c_void }
impl TRegEx for RegEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RegEx {
    pub fn from(ptr: *mut c_void) -> RegEx { RegEx { ptr: ptr } }
    pub fn null() -> RegEx { RegEx::from(0 as *mut c_void) }
    
}

pub trait TRegEx {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct ScopedArray { ptr: *mut c_void }
impl TScopedArray for ScopedArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScopedArray {
    pub fn from(ptr: *mut c_void) -> ScopedArray { ScopedArray { ptr: ptr } }
    pub fn null() -> ScopedArray { ScopedArray::from(0 as *mut c_void) }
    
}

pub trait TScopedArray {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct ScopedPtr { ptr: *mut c_void }
impl TScopedPtr for ScopedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScopedPtr {
    pub fn from(ptr: *mut c_void) -> ScopedPtr { ScopedPtr { ptr: ptr } }
    pub fn null() -> ScopedPtr { ScopedPtr::from(0 as *mut c_void) }
    
}

pub trait TScopedPtr {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Semaphore { ptr: *mut c_void }
impl TSemaphore for Semaphore { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Semaphore {
    pub fn from(ptr: *mut c_void) -> Semaphore { Semaphore { ptr: ptr } }
    pub fn null() -> Semaphore { Semaphore::from(0 as *mut c_void) }
    
}

pub trait TSemaphore {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Server { ptr: *mut c_void }
impl TServer for Server {}
impl TServerBase for Server {}
impl TObject for Server { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Server {
    pub fn from(ptr: *mut c_void) -> Server { Server { ptr: ptr } }
    pub fn null() -> Server { Server::from(0 as *mut c_void) }
    
}

pub trait TServer : TServerBase {
}

pub struct ServerBase { ptr: *mut c_void }
impl TServerBase for ServerBase {}
impl TObject for ServerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ServerBase {
    pub fn from(ptr: *mut c_void) -> ServerBase { ServerBase { ptr: ptr } }
    pub fn null() -> ServerBase { ServerBase::from(0 as *mut c_void) }
    
}

pub trait TServerBase : TObject {
}

pub struct SingleInstanceChecker { ptr: *mut c_void }
impl TSingleInstanceChecker for SingleInstanceChecker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SingleInstanceChecker {
    pub fn from(ptr: *mut c_void) -> SingleInstanceChecker { SingleInstanceChecker { ptr: ptr } }
    pub fn null() -> SingleInstanceChecker { SingleInstanceChecker::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, name: &str, path: &str) -> c_int {
        let name = wxT(name);
        let path = wxT(path);
        unsafe { wxSingleInstanceChecker_Create(_obj, name.ptr(), path.ptr()) }
    }
    pub fn newDefault() -> SingleInstanceChecker {
        unsafe { SingleInstanceChecker { ptr: wxSingleInstanceChecker_CreateDefault() } }
    }
}

pub trait TSingleInstanceChecker {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.ptr()) }
    }
    fn isAnotherRunning(&self) -> c_int {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.ptr()) }
    }
}

pub struct StopWatch { ptr: *mut c_void }
impl TStopWatch for StopWatch { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StopWatch {
    pub fn from(ptr: *mut c_void) -> StopWatch { StopWatch { ptr: ptr } }
    pub fn null() -> StopWatch { StopWatch::from(0 as *mut c_void) }
    
    pub fn new() -> StopWatch {
        unsafe { StopWatch { ptr: wxStopWatch_Create() } }
    }
}

pub trait TStopWatch {
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

pub struct StreamBase { ptr: *mut c_void }
impl TStreamBase for StreamBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamBase {
    pub fn from(ptr: *mut c_void) -> StreamBase { StreamBase { ptr: ptr } }
    pub fn null() -> StreamBase { StreamBase::from(0 as *mut c_void) }
    
}

pub trait TStreamBase {
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

pub struct StreamBuffer { ptr: *mut c_void }
impl TStreamBuffer for StreamBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamBuffer {
    pub fn from(ptr: *mut c_void) -> StreamBuffer { StreamBuffer { ptr: ptr } }
    pub fn null() -> StreamBuffer { StreamBuffer::from(0 as *mut c_void) }
    
}

pub trait TStreamBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct StringBuffer { ptr: *mut c_void }
impl TStringBuffer for StringBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringBuffer {
    pub fn from(ptr: *mut c_void) -> StringBuffer { StringBuffer { ptr: ptr } }
    pub fn null() -> StringBuffer { StringBuffer::from(0 as *mut c_void) }
    
}

pub trait TStringBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct StringClientData { ptr: *mut c_void }
impl TStringClientData for StringClientData {}
impl TClientData for StringClientData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringClientData {
    pub fn from(ptr: *mut c_void) -> StringClientData { StringClientData { ptr: ptr } }
    pub fn null() -> StringClientData { StringClientData::from(0 as *mut c_void) }
    
}

pub trait TStringClientData : TClientData {
}

pub struct StringList { ptr: *mut c_void }
impl TStringList for StringList {}
impl TList for StringList {}
impl TObject for StringList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringList {
    pub fn from(ptr: *mut c_void) -> StringList { StringList { ptr: ptr } }
    pub fn null() -> StringList { StringList::from(0 as *mut c_void) }
    
}

pub trait TStringList : TList {
}

pub struct StringTokenizer { ptr: *mut c_void }
impl TStringTokenizer for StringTokenizer {}
impl TObject for StringTokenizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StringTokenizer {
    pub fn from(ptr: *mut c_void) -> StringTokenizer { StringTokenizer { ptr: ptr } }
    pub fn null() -> StringTokenizer { StringTokenizer::from(0 as *mut c_void) }
    
}

pub trait TStringTokenizer : TObject {
}

pub struct SystemOptions { ptr: *mut c_void }
impl TSystemOptions for SystemOptions {}
impl TObject for SystemOptions { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SystemOptions {
    pub fn from(ptr: *mut c_void) -> SystemOptions { SystemOptions { ptr: ptr } }
    pub fn null() -> SystemOptions { SystemOptions::from(0 as *mut c_void) }
    
}

pub trait TSystemOptions : TObject {
}

pub struct TempFile { ptr: *mut c_void }
impl TTempFile for TempFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TempFile {
    pub fn from(ptr: *mut c_void) -> TempFile { TempFile { ptr: ptr } }
    pub fn null() -> TempFile { TempFile::from(0 as *mut c_void) }
    
}

pub trait TTempFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct TextFile { ptr: *mut c_void }
impl TTextFile for TextFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextFile {
    pub fn from(ptr: *mut c_void) -> TextFile { TextFile { ptr: ptr } }
    pub fn null() -> TextFile { TextFile::from(0 as *mut c_void) }
    
}

pub trait TTextFile {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct TextInputStream { ptr: *mut c_void }
impl TTextInputStream for TextInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextInputStream {
    pub fn from(ptr: *mut c_void) -> TextInputStream { TextInputStream { ptr: ptr } }
    pub fn null() -> TextInputStream { TextInputStream::from(0 as *mut c_void) }
    
    pub fn new<T: TInputStream>(inputStream: &T, sep: &str) -> TextInputStream {
        let sep = wxT(sep);
        unsafe { TextInputStream { ptr: wxTextInputStream_Create(inputStream.ptr(), sep.ptr()) } }
    }
}

pub trait TTextInputStream {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.ptr()) }
    }
    fn readLine(&self) -> ~str {
        unsafe { WxString { ptr: wxTextInputStream_ReadLine(self.ptr()) }.to_str() }
    }
}

pub struct TextOutputStream { ptr: *mut c_void }
impl TTextOutputStream for TextOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextOutputStream {
    pub fn from(ptr: *mut c_void) -> TextOutputStream { TextOutputStream { ptr: ptr } }
    pub fn null() -> TextOutputStream { TextOutputStream::from(0 as *mut c_void) }
    
    pub fn new<T: TOutputStream>(outputStream: &T, mode: c_int) -> TextOutputStream {
        unsafe { TextOutputStream { ptr: wxTextOutputStream_Create(outputStream.ptr(), mode) } }
    }
}

pub trait TTextOutputStream {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.ptr()) }
    }
    fn writeString(&self, txt: &str) {
        let txt = wxT(txt);
        unsafe { wxTextOutputStream_WriteString(self.ptr(), txt.ptr()) }
    }
}

pub struct Thread { ptr: *mut c_void }
impl TThread for Thread { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Thread {
    pub fn from(ptr: *mut c_void) -> Thread { Thread { ptr: ptr } }
    pub fn null() -> Thread { Thread::from(0 as *mut c_void) }
    
}

pub trait TThread {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Time { ptr: *mut c_void }
impl TTime for Time {}
impl TObject for Time { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Time {
    pub fn from(ptr: *mut c_void) -> Time { Time { ptr: ptr } }
    pub fn null() -> Time { Time::from(0 as *mut c_void) }
    
}

pub trait TTime : TObject {
}

pub struct TimeSpan { ptr: *mut c_void }
impl TTimeSpan for TimeSpan { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimeSpan {
    pub fn from(ptr: *mut c_void) -> TimeSpan { TimeSpan { ptr: ptr } }
    pub fn null() -> TimeSpan { TimeSpan::from(0 as *mut c_void) }
    
}

pub trait TTimeSpan {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Variant { ptr: *mut c_void }
impl TVariant for Variant {}
impl TObject for Variant { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Variant {
    pub fn from(ptr: *mut c_void) -> Variant { Variant { ptr: ptr } }
    pub fn null() -> Variant { Variant::from(0 as *mut c_void) }
    
}

pub trait TVariant : TObject {
}

pub struct VariantData { ptr: *mut c_void }
impl TVariantData for VariantData {}
impl TObject for VariantData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl VariantData {
    pub fn from(ptr: *mut c_void) -> VariantData { VariantData { ptr: ptr } }
    pub fn null() -> VariantData { VariantData::from(0 as *mut c_void) }
    
}

pub trait TVariantData : TObject {
}

pub struct ZipInputStream { ptr: *mut c_void }
impl TZipInputStream for ZipInputStream {}
impl TInputStream for ZipInputStream {}
impl TStreamBase for ZipInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ZipInputStream {
    pub fn from(ptr: *mut c_void) -> ZipInputStream { ZipInputStream { ptr: ptr } }
    pub fn null() -> ZipInputStream { ZipInputStream::from(0 as *mut c_void) }
    
}

pub trait TZipInputStream : TInputStream {
}

pub struct ZlibInputStream { ptr: *mut c_void }
impl TZlibInputStream for ZlibInputStream {}
impl TFilterInputStream for ZlibInputStream {}
impl TInputStream for ZlibInputStream {}
impl TStreamBase for ZlibInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ZlibInputStream {
    pub fn from(ptr: *mut c_void) -> ZlibInputStream { ZlibInputStream { ptr: ptr } }
    pub fn null() -> ZlibInputStream { ZlibInputStream::from(0 as *mut c_void) }
    
}

pub trait TZlibInputStream : TFilterInputStream {
}

pub struct ZlibOutputStream { ptr: *mut c_void }
impl TZlibOutputStream for ZlibOutputStream {}
impl TFilterOutputStream for ZlibOutputStream {}
impl TOutputStream for ZlibOutputStream {}
impl TStreamBase for ZlibOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ZlibOutputStream {
    pub fn from(ptr: *mut c_void) -> ZlibOutputStream { ZlibOutputStream { ptr: ptr } }
    pub fn null() -> ZlibOutputStream { ZlibOutputStream::from(0 as *mut c_void) }
    
}

pub trait TZlibOutputStream : TFilterOutputStream {
}

pub struct MemoryBuffer { ptr: *mut c_void }
impl TMemoryBuffer for MemoryBuffer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryBuffer {
    pub fn from(ptr: *mut c_void) -> MemoryBuffer { MemoryBuffer { ptr: ptr } }
    pub fn null() -> MemoryBuffer { MemoryBuffer::from(0 as *mut c_void) }
    
}

pub trait TMemoryBuffer {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct FileConfig { ptr: *mut c_void }
impl TFileConfig for FileConfig {}
impl TConfigBase for FileConfig { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileConfig {
    pub fn from(ptr: *mut c_void) -> FileConfig { FileConfig { ptr: ptr } }
    pub fn null() -> FileConfig { FileConfig::from(0 as *mut c_void) }
    
    pub fn new<T: TInputStream>(inp: &T) -> FileConfig {
        unsafe { FileConfig { ptr: wxFileConfig_Create(inp.ptr()) } }
    }
}

pub trait TFileConfig : TConfigBase {
}

