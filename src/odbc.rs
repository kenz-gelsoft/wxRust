use std::libc::*;
use base::*;

pub struct WxDatabase { ptr: *mut c_void }
impl TWxDatabase for WxDatabase {}
impl TWxObject for WxDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDatabase {
    pub fn from(ptr: *mut c_void) -> WxDatabase { WxDatabase { ptr: ptr } }
    pub fn null() -> WxDatabase { WxDatabase::from(0 as *mut c_void) }
    
}

pub trait TWxDatabase : TWxObject {
}

pub struct WxDb { ptr: *mut c_void }
impl TWxDb for WxDb { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDb {
    pub fn from(ptr: *mut c_void) -> WxDb { WxDb { ptr: ptr } }
    pub fn null() -> WxDb { WxDb::from(0 as *mut c_void) }
    
}

pub trait TWxDb {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbColDef { ptr: *mut c_void }
impl TWxDbColDef for WxDbColDef { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbColDef {
    pub fn from(ptr: *mut c_void) -> WxDbColDef { WxDbColDef { ptr: ptr } }
    pub fn null() -> WxDbColDef { WxDbColDef::from(0 as *mut c_void) }
    
}

pub trait TWxDbColDef {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbColFor { ptr: *mut c_void }
impl TWxDbColFor for WxDbColFor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbColFor {
    pub fn from(ptr: *mut c_void) -> WxDbColFor { WxDbColFor { ptr: ptr } }
    pub fn null() -> WxDbColFor { WxDbColFor::from(0 as *mut c_void) }
    
}

pub trait TWxDbColFor {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbColInf { ptr: *mut c_void }
impl TWxDbColInf for WxDbColInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbColInf {
    pub fn from(ptr: *mut c_void) -> WxDbColInf { WxDbColInf { ptr: ptr } }
    pub fn null() -> WxDbColInf { WxDbColInf::from(0 as *mut c_void) }
    
}

pub trait TWxDbColInf {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbConnectInf { ptr: *mut c_void }
impl TWxDbConnectInf for WxDbConnectInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbConnectInf {
    pub fn from(ptr: *mut c_void) -> WxDbConnectInf { WxDbConnectInf { ptr: ptr } }
    pub fn null() -> WxDbConnectInf { WxDbConnectInf::from(0 as *mut c_void) }
    
}

pub trait TWxDbConnectInf {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbInf { ptr: *mut c_void }
impl TWxDbInf for WxDbInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbInf {
    pub fn from(ptr: *mut c_void) -> WxDbInf { WxDbInf { ptr: ptr } }
    pub fn null() -> WxDbInf { WxDbInf::from(0 as *mut c_void) }
    
}

pub trait TWxDbInf {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbSqlTypeInfo { ptr: *mut c_void }
impl TWxDbSqlTypeInfo for WxDbSqlTypeInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbSqlTypeInfo {
    pub fn from(ptr: *mut c_void) -> WxDbSqlTypeInfo { WxDbSqlTypeInfo { ptr: ptr } }
    pub fn null() -> WxDbSqlTypeInfo { WxDbSqlTypeInfo::from(0 as *mut c_void) }
    
}

pub trait TWxDbSqlTypeInfo {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbTable { ptr: *mut c_void }
impl TWxDbTable for WxDbTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbTable {
    pub fn from(ptr: *mut c_void) -> WxDbTable { WxDbTable { ptr: ptr } }
    pub fn null() -> WxDbTable { WxDbTable::from(0 as *mut c_void) }
    
}

pub trait TWxDbTable {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDbTableInfo { ptr: *mut c_void }
impl TWxDbTableInfo for WxDbTableInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDbTableInfo {
    pub fn from(ptr: *mut c_void) -> WxDbTableInfo { WxDbTableInfo { ptr: ptr } }
    pub fn null() -> WxDbTableInfo { WxDbTableInfo::from(0 as *mut c_void) }
    
}

pub trait TWxDbTableInfo {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxQueryCol { ptr: *mut c_void }
impl TWxQueryCol for WxQueryCol {}
impl TWxObject for WxQueryCol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxQueryCol {
    pub fn from(ptr: *mut c_void) -> WxQueryCol { WxQueryCol { ptr: ptr } }
    pub fn null() -> WxQueryCol { WxQueryCol::from(0 as *mut c_void) }
    
}

pub trait TWxQueryCol : TWxObject {
}

pub struct WxQueryField { ptr: *mut c_void }
impl TWxQueryField for WxQueryField {}
impl TWxObject for WxQueryField { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxQueryField {
    pub fn from(ptr: *mut c_void) -> WxQueryField { WxQueryField { ptr: ptr } }
    pub fn null() -> WxQueryField { WxQueryField::from(0 as *mut c_void) }
    
}

pub trait TWxQueryField : TWxObject {
}

pub struct WxRecordSet { ptr: *mut c_void }
impl TWxRecordSet for WxRecordSet {}
impl TWxObject for WxRecordSet { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRecordSet {
    pub fn from(ptr: *mut c_void) -> WxRecordSet { WxRecordSet { ptr: ptr } }
    pub fn null() -> WxRecordSet { WxRecordSet::from(0 as *mut c_void) }
    
}

pub trait TWxRecordSet : TWxObject {
}

pub struct WxTablesInUse { ptr: *mut c_void }
impl TWxTablesInUse for WxTablesInUse {}
impl TWxObject for WxTablesInUse { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTablesInUse {
    pub fn from(ptr: *mut c_void) -> WxTablesInUse { WxTablesInUse { ptr: ptr } }
    pub fn null() -> WxTablesInUse { WxTablesInUse::from(0 as *mut c_void) }
    
}

pub trait TWxTablesInUse : TWxObject {
}

