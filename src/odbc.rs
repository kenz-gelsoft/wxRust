use std::libc::*;
use base::*;

pub struct wxDatabase { handle: *mut c_void }
impl _wxDatabase for wxDatabase {}
impl _wxObject for wxDatabase { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDatabase {
    pub fn from(handle: *mut c_void) -> @wxDatabase { @wxDatabase { handle: handle } }
    pub fn null() -> @wxDatabase { wxDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxDatabase : _wxObject {
}

pub struct wxDb { handle: *mut c_void }
impl _wxDb for wxDb { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDb {
    pub fn from(handle: *mut c_void) -> @wxDb { @wxDb { handle: handle } }
    pub fn null() -> @wxDb { wxDb::from(0 as *mut c_void) }
    
}

pub trait _wxDb {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbColDef { handle: *mut c_void }
impl _wxDbColDef for wxDbColDef { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbColDef {
    pub fn from(handle: *mut c_void) -> @wxDbColDef { @wxDbColDef { handle: handle } }
    pub fn null() -> @wxDbColDef { wxDbColDef::from(0 as *mut c_void) }
    
}

pub trait _wxDbColDef {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbColFor { handle: *mut c_void }
impl _wxDbColFor for wxDbColFor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbColFor {
    pub fn from(handle: *mut c_void) -> @wxDbColFor { @wxDbColFor { handle: handle } }
    pub fn null() -> @wxDbColFor { wxDbColFor::from(0 as *mut c_void) }
    
}

pub trait _wxDbColFor {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbColInf { handle: *mut c_void }
impl _wxDbColInf for wxDbColInf { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbColInf {
    pub fn from(handle: *mut c_void) -> @wxDbColInf { @wxDbColInf { handle: handle } }
    pub fn null() -> @wxDbColInf { wxDbColInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbColInf {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbConnectInf { handle: *mut c_void }
impl _wxDbConnectInf for wxDbConnectInf { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbConnectInf {
    pub fn from(handle: *mut c_void) -> @wxDbConnectInf { @wxDbConnectInf { handle: handle } }
    pub fn null() -> @wxDbConnectInf { wxDbConnectInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbConnectInf {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbInf { handle: *mut c_void }
impl _wxDbInf for wxDbInf { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbInf {
    pub fn from(handle: *mut c_void) -> @wxDbInf { @wxDbInf { handle: handle } }
    pub fn null() -> @wxDbInf { wxDbInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbInf {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbSqlTypeInfo { handle: *mut c_void }
impl _wxDbSqlTypeInfo for wxDbSqlTypeInfo { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbSqlTypeInfo {
    pub fn from(handle: *mut c_void) -> @wxDbSqlTypeInfo { @wxDbSqlTypeInfo { handle: handle } }
    pub fn null() -> @wxDbSqlTypeInfo { wxDbSqlTypeInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDbSqlTypeInfo {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbTable { handle: *mut c_void }
impl _wxDbTable for wxDbTable { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbTable {
    pub fn from(handle: *mut c_void) -> @wxDbTable { @wxDbTable { handle: handle } }
    pub fn null() -> @wxDbTable { wxDbTable::from(0 as *mut c_void) }
    
}

pub trait _wxDbTable {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbTableInfo { handle: *mut c_void }
impl _wxDbTableInfo for wxDbTableInfo { fn handle(&self) -> *mut c_void { self.handle } }

impl wxDbTableInfo {
    pub fn from(handle: *mut c_void) -> @wxDbTableInfo { @wxDbTableInfo { handle: handle } }
    pub fn null() -> @wxDbTableInfo { wxDbTableInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDbTableInfo {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxQueryCol { handle: *mut c_void }
impl _wxQueryCol for wxQueryCol {}
impl _wxObject for wxQueryCol { fn handle(&self) -> *mut c_void { self.handle } }

impl wxQueryCol {
    pub fn from(handle: *mut c_void) -> @wxQueryCol { @wxQueryCol { handle: handle } }
    pub fn null() -> @wxQueryCol { wxQueryCol::from(0 as *mut c_void) }
    
}

pub trait _wxQueryCol : _wxObject {
}

pub struct wxQueryField { handle: *mut c_void }
impl _wxQueryField for wxQueryField {}
impl _wxObject for wxQueryField { fn handle(&self) -> *mut c_void { self.handle } }

impl wxQueryField {
    pub fn from(handle: *mut c_void) -> @wxQueryField { @wxQueryField { handle: handle } }
    pub fn null() -> @wxQueryField { wxQueryField::from(0 as *mut c_void) }
    
}

pub trait _wxQueryField : _wxObject {
}

pub struct wxRecordSet { handle: *mut c_void }
impl _wxRecordSet for wxRecordSet {}
impl _wxObject for wxRecordSet { fn handle(&self) -> *mut c_void { self.handle } }

impl wxRecordSet {
    pub fn from(handle: *mut c_void) -> @wxRecordSet { @wxRecordSet { handle: handle } }
    pub fn null() -> @wxRecordSet { wxRecordSet::from(0 as *mut c_void) }
    
}

pub trait _wxRecordSet : _wxObject {
}

pub struct wxTablesInUse { handle: *mut c_void }
impl _wxTablesInUse for wxTablesInUse {}
impl _wxObject for wxTablesInUse { fn handle(&self) -> *mut c_void { self.handle } }

impl wxTablesInUse {
    pub fn from(handle: *mut c_void) -> @wxTablesInUse { @wxTablesInUse { handle: handle } }
    pub fn null() -> @wxTablesInUse { wxTablesInUse::from(0 as *mut c_void) }
    
}

pub trait _wxTablesInUse : _wxObject {
}

