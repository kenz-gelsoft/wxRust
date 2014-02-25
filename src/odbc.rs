use std::libc::*;
use base::*;

/// Wraps the wxWidgets' [wxDatabase](http://docs.wxwidgets.org/3.0/classwx_database.html) class.
pub struct Database { ptr: *mut c_void }
impl TDatabase for Database {}
impl TObject for Database { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Database {
    pub fn from(ptr: *mut c_void) -> Database { Database { ptr: ptr } }
    pub fn null() -> Database { Database::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDatabase](http://docs.wxwidgets.org/3.0/classwx_database.html) class.
pub trait TDatabase : TObject {
}

/// Wraps the wxWidgets' [wxDb](http://docs.wxwidgets.org/3.0/classwx_db.html) class.
pub struct Db { ptr: *mut c_void }
impl TDb for Db { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Db {
    pub fn from(ptr: *mut c_void) -> Db { Db { ptr: ptr } }
    pub fn null() -> Db { Db::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDb](http://docs.wxwidgets.org/3.0/classwx_db.html) class.
pub trait TDb {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbColDef](http://docs.wxwidgets.org/3.0/classwx_db_col_def.html) class.
pub struct DbColDef { ptr: *mut c_void }
impl TDbColDef for DbColDef { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbColDef {
    pub fn from(ptr: *mut c_void) -> DbColDef { DbColDef { ptr: ptr } }
    pub fn null() -> DbColDef { DbColDef::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbColDef](http://docs.wxwidgets.org/3.0/classwx_db_col_def.html) class.
pub trait TDbColDef {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbColFor](http://docs.wxwidgets.org/3.0/classwx_db_col_for.html) class.
pub struct DbColFor { ptr: *mut c_void }
impl TDbColFor for DbColFor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbColFor {
    pub fn from(ptr: *mut c_void) -> DbColFor { DbColFor { ptr: ptr } }
    pub fn null() -> DbColFor { DbColFor::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbColFor](http://docs.wxwidgets.org/3.0/classwx_db_col_for.html) class.
pub trait TDbColFor {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbColInf](http://docs.wxwidgets.org/3.0/classwx_db_col_inf.html) class.
pub struct DbColInf { ptr: *mut c_void }
impl TDbColInf for DbColInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbColInf {
    pub fn from(ptr: *mut c_void) -> DbColInf { DbColInf { ptr: ptr } }
    pub fn null() -> DbColInf { DbColInf::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbColInf](http://docs.wxwidgets.org/3.0/classwx_db_col_inf.html) class.
pub trait TDbColInf {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbConnectInf](http://docs.wxwidgets.org/3.0/classwx_db_connect_inf.html) class.
pub struct DbConnectInf { ptr: *mut c_void }
impl TDbConnectInf for DbConnectInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbConnectInf {
    pub fn from(ptr: *mut c_void) -> DbConnectInf { DbConnectInf { ptr: ptr } }
    pub fn null() -> DbConnectInf { DbConnectInf::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbConnectInf](http://docs.wxwidgets.org/3.0/classwx_db_connect_inf.html) class.
pub trait TDbConnectInf {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbInf](http://docs.wxwidgets.org/3.0/classwx_db_inf.html) class.
pub struct DbInf { ptr: *mut c_void }
impl TDbInf for DbInf { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbInf {
    pub fn from(ptr: *mut c_void) -> DbInf { DbInf { ptr: ptr } }
    pub fn null() -> DbInf { DbInf::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbInf](http://docs.wxwidgets.org/3.0/classwx_db_inf.html) class.
pub trait TDbInf {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbSqlTypeInfo](http://docs.wxwidgets.org/3.0/classwx_db_sql_type_info.html) class.
pub struct DbSqlTypeInfo { ptr: *mut c_void }
impl TDbSqlTypeInfo for DbSqlTypeInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbSqlTypeInfo {
    pub fn from(ptr: *mut c_void) -> DbSqlTypeInfo { DbSqlTypeInfo { ptr: ptr } }
    pub fn null() -> DbSqlTypeInfo { DbSqlTypeInfo::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbSqlTypeInfo](http://docs.wxwidgets.org/3.0/classwx_db_sql_type_info.html) class.
pub trait TDbSqlTypeInfo {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbTable](http://docs.wxwidgets.org/3.0/classwx_db_table.html) class.
pub struct DbTable { ptr: *mut c_void }
impl TDbTable for DbTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbTable {
    pub fn from(ptr: *mut c_void) -> DbTable { DbTable { ptr: ptr } }
    pub fn null() -> DbTable { DbTable::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbTable](http://docs.wxwidgets.org/3.0/classwx_db_table.html) class.
pub trait TDbTable {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDbTableInfo](http://docs.wxwidgets.org/3.0/classwx_db_table_info.html) class.
pub struct DbTableInfo { ptr: *mut c_void }
impl TDbTableInfo for DbTableInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DbTableInfo {
    pub fn from(ptr: *mut c_void) -> DbTableInfo { DbTableInfo { ptr: ptr } }
    pub fn null() -> DbTableInfo { DbTableInfo::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDbTableInfo](http://docs.wxwidgets.org/3.0/classwx_db_table_info.html) class.
pub trait TDbTableInfo {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxQueryCol](http://docs.wxwidgets.org/3.0/classwx_query_col.html) class.
pub struct QueryCol { ptr: *mut c_void }
impl TQueryCol for QueryCol {}
impl TObject for QueryCol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl QueryCol {
    pub fn from(ptr: *mut c_void) -> QueryCol { QueryCol { ptr: ptr } }
    pub fn null() -> QueryCol { QueryCol::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxQueryCol](http://docs.wxwidgets.org/3.0/classwx_query_col.html) class.
pub trait TQueryCol : TObject {
}

/// Wraps the wxWidgets' [wxQueryField](http://docs.wxwidgets.org/3.0/classwx_query_field.html) class.
pub struct QueryField { ptr: *mut c_void }
impl TQueryField for QueryField {}
impl TObject for QueryField { fn ptr(&self) -> *mut c_void { self.ptr } }

impl QueryField {
    pub fn from(ptr: *mut c_void) -> QueryField { QueryField { ptr: ptr } }
    pub fn null() -> QueryField { QueryField::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxQueryField](http://docs.wxwidgets.org/3.0/classwx_query_field.html) class.
pub trait TQueryField : TObject {
}

/// Wraps the wxWidgets' [wxRecordSet](http://docs.wxwidgets.org/3.0/classwx_record_set.html) class.
pub struct RecordSet { ptr: *mut c_void }
impl TRecordSet for RecordSet {}
impl TObject for RecordSet { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RecordSet {
    pub fn from(ptr: *mut c_void) -> RecordSet { RecordSet { ptr: ptr } }
    pub fn null() -> RecordSet { RecordSet::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxRecordSet](http://docs.wxwidgets.org/3.0/classwx_record_set.html) class.
pub trait TRecordSet : TObject {
}

/// Wraps the wxWidgets' [wxTablesInUse](http://docs.wxwidgets.org/3.0/classwx_tables_in_use.html) class.
pub struct TablesInUse { ptr: *mut c_void }
impl TTablesInUse for TablesInUse {}
impl TObject for TablesInUse { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TablesInUse {
    pub fn from(ptr: *mut c_void) -> TablesInUse { TablesInUse { ptr: ptr } }
    pub fn null() -> TablesInUse { TablesInUse::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTablesInUse](http://docs.wxwidgets.org/3.0/classwx_tables_in_use.html) class.
pub trait TTablesInUse : TObject {
}

