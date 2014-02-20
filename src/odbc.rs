use std::libc::*;
use base::*;

pub struct wxDatabase { ptr: *mut c_void }
impl _wxDatabase for wxDatabase {}
impl _wxObject for wxDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDatabase {
    pub fn from(ptr: *mut c_void) -> wxDatabase { wxDatabase { ptr: ptr } }
    pub fn null() -> wxDatabase { wxDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxDatabase : _wxObject {
}

pub struct wxDb { ptr: *mut c_void }
impl _wxDb for wxDb { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDb {
    pub fn from(ptr: *mut c_void) -> wxDb { wxDb { ptr: ptr } }
    pub fn null() -> wxDb { wxDb::from(0 as *mut c_void) }
    
}

pub trait _wxDb {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbColDef { ptr: *mut c_void }
impl _wxDbColDef for wxDbColDef { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbColDef {
    pub fn from(ptr: *mut c_void) -> wxDbColDef { wxDbColDef { ptr: ptr } }
    pub fn null() -> wxDbColDef { wxDbColDef::from(0 as *mut c_void) }
    
}

pub trait _wxDbColDef {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbColFor { ptr: *mut c_void }
impl _wxDbColFor for wxDbColFor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbColFor {
    pub fn from(ptr: *mut c_void) -> wxDbColFor { wxDbColFor { ptr: ptr } }
    pub fn null() -> wxDbColFor { wxDbColFor::from(0 as *mut c_void) }
    
}

pub trait _wxDbColFor {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbColInf { ptr: *mut c_void }
impl _wxDbColInf for wxDbColInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbColInf {
    pub fn from(ptr: *mut c_void) -> wxDbColInf { wxDbColInf { ptr: ptr } }
    pub fn null() -> wxDbColInf { wxDbColInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbColInf {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbConnectInf { ptr: *mut c_void }
impl _wxDbConnectInf for wxDbConnectInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbConnectInf {
    pub fn from(ptr: *mut c_void) -> wxDbConnectInf { wxDbConnectInf { ptr: ptr } }
    pub fn null() -> wxDbConnectInf { wxDbConnectInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbConnectInf {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbInf { ptr: *mut c_void }
impl _wxDbInf for wxDbInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbInf {
    pub fn from(ptr: *mut c_void) -> wxDbInf { wxDbInf { ptr: ptr } }
    pub fn null() -> wxDbInf { wxDbInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbInf {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbSqlTypeInfo { ptr: *mut c_void }
impl _wxDbSqlTypeInfo for wxDbSqlTypeInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbSqlTypeInfo {
    pub fn from(ptr: *mut c_void) -> wxDbSqlTypeInfo { wxDbSqlTypeInfo { ptr: ptr } }
    pub fn null() -> wxDbSqlTypeInfo { wxDbSqlTypeInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDbSqlTypeInfo {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbTable { ptr: *mut c_void }
impl _wxDbTable for wxDbTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbTable {
    pub fn from(ptr: *mut c_void) -> wxDbTable { wxDbTable { ptr: ptr } }
    pub fn null() -> wxDbTable { wxDbTable::from(0 as *mut c_void) }
    
}

pub trait _wxDbTable {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDbTableInfo { ptr: *mut c_void }
impl _wxDbTableInfo for wxDbTableInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDbTableInfo {
    pub fn from(ptr: *mut c_void) -> wxDbTableInfo { wxDbTableInfo { ptr: ptr } }
    pub fn null() -> wxDbTableInfo { wxDbTableInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDbTableInfo {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxQueryCol { ptr: *mut c_void }
impl _wxQueryCol for wxQueryCol {}
impl _wxObject for wxQueryCol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxQueryCol {
    pub fn from(ptr: *mut c_void) -> wxQueryCol { wxQueryCol { ptr: ptr } }
    pub fn null() -> wxQueryCol { wxQueryCol::from(0 as *mut c_void) }
    
}

pub trait _wxQueryCol : _wxObject {
}

pub struct wxQueryField { ptr: *mut c_void }
impl _wxQueryField for wxQueryField {}
impl _wxObject for wxQueryField { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxQueryField {
    pub fn from(ptr: *mut c_void) -> wxQueryField { wxQueryField { ptr: ptr } }
    pub fn null() -> wxQueryField { wxQueryField::from(0 as *mut c_void) }
    
}

pub trait _wxQueryField : _wxObject {
}

pub struct wxRecordSet { ptr: *mut c_void }
impl _wxRecordSet for wxRecordSet {}
impl _wxObject for wxRecordSet { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRecordSet {
    pub fn from(ptr: *mut c_void) -> wxRecordSet { wxRecordSet { ptr: ptr } }
    pub fn null() -> wxRecordSet { wxRecordSet::from(0 as *mut c_void) }
    
}

pub trait _wxRecordSet : _wxObject {
}

pub struct wxTablesInUse { ptr: *mut c_void }
impl _wxTablesInUse for wxTablesInUse {}
impl _wxObject for wxTablesInUse { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTablesInUse {
    pub fn from(ptr: *mut c_void) -> wxTablesInUse { wxTablesInUse { ptr: ptr } }
    pub fn null() -> wxTablesInUse { wxTablesInUse::from(0 as *mut c_void) }
    
}

pub trait _wxTablesInUse : _wxObject {
}

