use std::libc::*;
use base::*;
use core::*;

pub struct wxFTP { ptr: *mut c_void }
impl _wxFTP for wxFTP {}
impl _wxProtocol for wxFTP {}
impl _wxSocketClient for wxFTP {}
impl _wxSocketBase for wxFTP {}
impl _wxObject for wxFTP { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFTP {
    pub fn from(ptr: *mut c_void) -> wxFTP { wxFTP { ptr: ptr } }
    pub fn null() -> wxFTP { wxFTP::from(0 as *mut c_void) }
    
}

pub trait _wxFTP : _wxProtocol {
}

pub struct wxHTTP { ptr: *mut c_void }
impl _wxHTTP for wxHTTP {}
impl _wxProtocol for wxHTTP {}
impl _wxSocketClient for wxHTTP {}
impl _wxSocketBase for wxHTTP {}
impl _wxObject for wxHTTP { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHTTP {
    pub fn from(ptr: *mut c_void) -> wxHTTP { wxHTTP { ptr: ptr } }
    pub fn null() -> wxHTTP { wxHTTP::from(0 as *mut c_void) }
    
}

pub trait _wxHTTP : _wxProtocol {
}

pub struct wxIPV4address { ptr: *mut c_void }
impl _wxIPV4address for wxIPV4address {}
impl _wxSockAddress for wxIPV4address {}
impl _wxObject for wxIPV4address { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIPV4address {
    pub fn from(ptr: *mut c_void) -> wxIPV4address { wxIPV4address { ptr: ptr } }
    pub fn null() -> wxIPV4address { wxIPV4address::from(0 as *mut c_void) }
    
}

pub trait _wxIPV4address : _wxSockAddress {
}

pub struct wxProtocol { ptr: *mut c_void }
impl _wxProtocol for wxProtocol {}
impl _wxSocketClient for wxProtocol {}
impl _wxSocketBase for wxProtocol {}
impl _wxObject for wxProtocol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxProtocol {
    pub fn from(ptr: *mut c_void) -> wxProtocol { wxProtocol { ptr: ptr } }
    pub fn null() -> wxProtocol { wxProtocol::from(0 as *mut c_void) }
    
}

pub trait _wxProtocol : _wxSocketClient {
}

pub struct wxSockAddress { ptr: *mut c_void }
impl _wxSockAddress for wxSockAddress {}
impl _wxObject for wxSockAddress { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSockAddress {
    pub fn from(ptr: *mut c_void) -> wxSockAddress { wxSockAddress { ptr: ptr } }
    pub fn null() -> wxSockAddress { wxSockAddress::from(0 as *mut c_void) }
    
}

pub trait _wxSockAddress : _wxObject {
}

pub struct wxSocketBase { ptr: *mut c_void }
impl _wxSocketBase for wxSocketBase {}
impl _wxObject for wxSocketBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSocketBase {
    pub fn from(ptr: *mut c_void) -> wxSocketBase { wxSocketBase { ptr: ptr } }
    pub fn null() -> wxSocketBase { wxSocketBase::from(0 as *mut c_void) }
    
}

pub trait _wxSocketBase : _wxObject {
}

pub struct wxSocketClient { ptr: *mut c_void }
impl _wxSocketClient for wxSocketClient {}
impl _wxSocketBase for wxSocketClient {}
impl _wxObject for wxSocketClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSocketClient {
    pub fn from(ptr: *mut c_void) -> wxSocketClient { wxSocketClient { ptr: ptr } }
    pub fn null() -> wxSocketClient { wxSocketClient::from(0 as *mut c_void) }
    
}

pub trait _wxSocketClient : _wxSocketBase {
}

pub struct wxSocketEvent { ptr: *mut c_void }
impl _wxSocketEvent for wxSocketEvent {}
impl _wxEvent for wxSocketEvent {}
impl _wxObject for wxSocketEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSocketEvent {
    pub fn from(ptr: *mut c_void) -> wxSocketEvent { wxSocketEvent { ptr: ptr } }
    pub fn null() -> wxSocketEvent { wxSocketEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSocketEvent : _wxEvent {
}

pub struct wxSocketInputStream { ptr: *mut c_void }
impl _wxSocketInputStream for wxSocketInputStream {}
impl _wxInputStream for wxSocketInputStream {}
impl _wxStreamBase for wxSocketInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSocketInputStream {
    pub fn from(ptr: *mut c_void) -> wxSocketInputStream { wxSocketInputStream { ptr: ptr } }
    pub fn null() -> wxSocketInputStream { wxSocketInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxSocketInputStream : _wxInputStream {
}

pub struct wxSocketOutputStream { ptr: *mut c_void }
impl _wxSocketOutputStream for wxSocketOutputStream {}
impl _wxOutputStream for wxSocketOutputStream {}
impl _wxStreamBase for wxSocketOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSocketOutputStream {
    pub fn from(ptr: *mut c_void) -> wxSocketOutputStream { wxSocketOutputStream { ptr: ptr } }
    pub fn null() -> wxSocketOutputStream { wxSocketOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxSocketOutputStream : _wxOutputStream {
}

pub struct wxSocketServer { ptr: *mut c_void }
impl _wxSocketServer for wxSocketServer {}
impl _wxSocketBase for wxSocketServer {}
impl _wxObject for wxSocketServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSocketServer {
    pub fn from(ptr: *mut c_void) -> wxSocketServer { wxSocketServer { ptr: ptr } }
    pub fn null() -> wxSocketServer { wxSocketServer::from(0 as *mut c_void) }
    
}

pub trait _wxSocketServer : _wxSocketBase {
}

pub struct wxURL { ptr: *mut c_void }
impl _wxURL for wxURL {}
impl _wxObject for wxURL { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxURL {
    pub fn from(ptr: *mut c_void) -> wxURL { wxURL { ptr: ptr } }
    pub fn null() -> wxURL { wxURL::from(0 as *mut c_void) }
    
}

pub trait _wxURL : _wxObject {
}

