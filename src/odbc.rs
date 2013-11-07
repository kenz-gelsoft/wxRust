use std::libc::*;
use base::*;
use core::*;
use advanced::*;
use native::*;


pub struct wxDatabase(*mut c_void);
impl _wxDatabase for wxDatabase {}
impl _wxObject for wxDatabase { fn handle(&self) -> *mut c_void { **self } }

impl wxDatabase {
    pub fn from(handle: *mut c_void) -> @wxDatabase { @wxDatabase(handle) }
    pub fn null() -> @wxDatabase { wxDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxDatabase : _wxObject {
}

pub struct wxDb(*mut c_void);
impl _wxDb for wxDb { fn handle(&self) -> *mut c_void { **self } }

impl wxDb {
    pub fn from(handle: *mut c_void) -> @wxDb { @wxDb(handle) }
    pub fn null() -> @wxDb { wxDb::from(0 as *mut c_void) }
    
}

pub trait _wxDb {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbColDef(*mut c_void);
impl _wxDbColDef for wxDbColDef { fn handle(&self) -> *mut c_void { **self } }

impl wxDbColDef {
    pub fn from(handle: *mut c_void) -> @wxDbColDef { @wxDbColDef(handle) }
    pub fn null() -> @wxDbColDef { wxDbColDef::from(0 as *mut c_void) }
    
}

pub trait _wxDbColDef {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbColFor(*mut c_void);
impl _wxDbColFor for wxDbColFor { fn handle(&self) -> *mut c_void { **self } }

impl wxDbColFor {
    pub fn from(handle: *mut c_void) -> @wxDbColFor { @wxDbColFor(handle) }
    pub fn null() -> @wxDbColFor { wxDbColFor::from(0 as *mut c_void) }
    
}

pub trait _wxDbColFor {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbColInf(*mut c_void);
impl _wxDbColInf for wxDbColInf { fn handle(&self) -> *mut c_void { **self } }

impl wxDbColInf {
    pub fn from(handle: *mut c_void) -> @wxDbColInf { @wxDbColInf(handle) }
    pub fn null() -> @wxDbColInf { wxDbColInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbColInf {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbConnectInf(*mut c_void);
impl _wxDbConnectInf for wxDbConnectInf { fn handle(&self) -> *mut c_void { **self } }

impl wxDbConnectInf {
    pub fn from(handle: *mut c_void) -> @wxDbConnectInf { @wxDbConnectInf(handle) }
    pub fn null() -> @wxDbConnectInf { wxDbConnectInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbConnectInf {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbInf(*mut c_void);
impl _wxDbInf for wxDbInf { fn handle(&self) -> *mut c_void { **self } }

impl wxDbInf {
    pub fn from(handle: *mut c_void) -> @wxDbInf { @wxDbInf(handle) }
    pub fn null() -> @wxDbInf { wxDbInf::from(0 as *mut c_void) }
    
}

pub trait _wxDbInf {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbSqlTypeInfo(*mut c_void);
impl _wxDbSqlTypeInfo for wxDbSqlTypeInfo { fn handle(&self) -> *mut c_void { **self } }

impl wxDbSqlTypeInfo {
    pub fn from(handle: *mut c_void) -> @wxDbSqlTypeInfo { @wxDbSqlTypeInfo(handle) }
    pub fn null() -> @wxDbSqlTypeInfo { wxDbSqlTypeInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDbSqlTypeInfo {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbTable(*mut c_void);
impl _wxDbTable for wxDbTable { fn handle(&self) -> *mut c_void { **self } }

impl wxDbTable {
    pub fn from(handle: *mut c_void) -> @wxDbTable { @wxDbTable(handle) }
    pub fn null() -> @wxDbTable { wxDbTable::from(0 as *mut c_void) }
    
}

pub trait _wxDbTable {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDbTableInfo(*mut c_void);
impl _wxDbTableInfo for wxDbTableInfo { fn handle(&self) -> *mut c_void { **self } }

impl wxDbTableInfo {
    pub fn from(handle: *mut c_void) -> @wxDbTableInfo { @wxDbTableInfo(handle) }
    pub fn null() -> @wxDbTableInfo { wxDbTableInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDbTableInfo {
    fn handle(&self) -> *mut c_void;
    
}

