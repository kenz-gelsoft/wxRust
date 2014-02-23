use std::libc::*;
use base::*;
use core::*;

pub struct FTP { ptr: *mut c_void }
impl TFTP for FTP {}
impl TProtocol for FTP {}
impl TSocketClient for FTP {}
impl TSocketBase for FTP {}
impl TObject for FTP { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FTP {
    pub fn from(ptr: *mut c_void) -> FTP { FTP { ptr: ptr } }
    pub fn null() -> FTP { FTP::from(0 as *mut c_void) }
    
}

pub trait TFTP : TProtocol {
}

pub struct HTTP { ptr: *mut c_void }
impl THTTP for HTTP {}
impl TProtocol for HTTP {}
impl TSocketClient for HTTP {}
impl TSocketBase for HTTP {}
impl TObject for HTTP { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HTTP {
    pub fn from(ptr: *mut c_void) -> HTTP { HTTP { ptr: ptr } }
    pub fn null() -> HTTP { HTTP::from(0 as *mut c_void) }
    
}

pub trait THTTP : TProtocol {
}

pub struct IPV4address { ptr: *mut c_void }
impl TIPV4address for IPV4address {}
impl TSockAddress for IPV4address {}
impl TObject for IPV4address { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IPV4address {
    pub fn from(ptr: *mut c_void) -> IPV4address { IPV4address { ptr: ptr } }
    pub fn null() -> IPV4address { IPV4address::from(0 as *mut c_void) }
    
}

pub trait TIPV4address : TSockAddress {
}

pub struct Protocol { ptr: *mut c_void }
impl TProtocol for Protocol {}
impl TSocketClient for Protocol {}
impl TSocketBase for Protocol {}
impl TObject for Protocol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Protocol {
    pub fn from(ptr: *mut c_void) -> Protocol { Protocol { ptr: ptr } }
    pub fn null() -> Protocol { Protocol::from(0 as *mut c_void) }
    
}

pub trait TProtocol : TSocketClient {
}

pub struct SockAddress { ptr: *mut c_void }
impl TSockAddress for SockAddress {}
impl TObject for SockAddress { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SockAddress {
    pub fn from(ptr: *mut c_void) -> SockAddress { SockAddress { ptr: ptr } }
    pub fn null() -> SockAddress { SockAddress::from(0 as *mut c_void) }
    
}

pub trait TSockAddress : TObject {
}

pub struct SocketBase { ptr: *mut c_void }
impl TSocketBase for SocketBase {}
impl TObject for SocketBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketBase {
    pub fn from(ptr: *mut c_void) -> SocketBase { SocketBase { ptr: ptr } }
    pub fn null() -> SocketBase { SocketBase::from(0 as *mut c_void) }
    
}

pub trait TSocketBase : TObject {
}

pub struct SocketClient { ptr: *mut c_void }
impl TSocketClient for SocketClient {}
impl TSocketBase for SocketClient {}
impl TObject for SocketClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketClient {
    pub fn from(ptr: *mut c_void) -> SocketClient { SocketClient { ptr: ptr } }
    pub fn null() -> SocketClient { SocketClient::from(0 as *mut c_void) }
    
}

pub trait TSocketClient : TSocketBase {
}

pub struct SocketEvent { ptr: *mut c_void }
impl TSocketEvent for SocketEvent {}
impl TEvent for SocketEvent {}
impl TObject for SocketEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketEvent {
    pub fn from(ptr: *mut c_void) -> SocketEvent { SocketEvent { ptr: ptr } }
    pub fn null() -> SocketEvent { SocketEvent::from(0 as *mut c_void) }
    
}

pub trait TSocketEvent : TEvent {
}

pub struct SocketInputStream { ptr: *mut c_void }
impl TSocketInputStream for SocketInputStream {}
impl TInputStream for SocketInputStream {}
impl TStreamBase for SocketInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketInputStream {
    pub fn from(ptr: *mut c_void) -> SocketInputStream { SocketInputStream { ptr: ptr } }
    pub fn null() -> SocketInputStream { SocketInputStream::from(0 as *mut c_void) }
    
}

pub trait TSocketInputStream : TInputStream {
}

pub struct SocketOutputStream { ptr: *mut c_void }
impl TSocketOutputStream for SocketOutputStream {}
impl TOutputStream for SocketOutputStream {}
impl TStreamBase for SocketOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketOutputStream {
    pub fn from(ptr: *mut c_void) -> SocketOutputStream { SocketOutputStream { ptr: ptr } }
    pub fn null() -> SocketOutputStream { SocketOutputStream::from(0 as *mut c_void) }
    
}

pub trait TSocketOutputStream : TOutputStream {
}

pub struct SocketServer { ptr: *mut c_void }
impl TSocketServer for SocketServer {}
impl TSocketBase for SocketServer {}
impl TObject for SocketServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketServer {
    pub fn from(ptr: *mut c_void) -> SocketServer { SocketServer { ptr: ptr } }
    pub fn null() -> SocketServer { SocketServer::from(0 as *mut c_void) }
    
}

pub trait TSocketServer : TSocketBase {
}

pub struct URL { ptr: *mut c_void }
impl TURL for URL {}
impl TObject for URL { fn ptr(&self) -> *mut c_void { self.ptr } }

impl URL {
    pub fn from(ptr: *mut c_void) -> URL { URL { ptr: ptr } }
    pub fn null() -> URL { URL::from(0 as *mut c_void) }
    
}

pub trait TURL : TObject {
}

