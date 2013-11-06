use std::libc::*;
use base::*;
use native::*;


pub struct wxFTP(*mut c_void);
impl _wxFTP for wxFTP {}
impl _wxProtocol for wxFTP {}
impl _wxSocketClient for wxFTP {}
impl _wxSocketBase for wxFTP {}
impl _wxObject for wxFTP { fn handle(&self) -> *mut c_void { **self } }

impl wxFTP {
    pub fn from(handle: *mut c_void) -> @wxFTP { @wxFTP(handle) }
    pub fn null() -> @wxFTP { wxFTP::from(0 as *mut c_void) }
    
}

pub trait _wxFTP : _wxProtocol {
}

pub struct wxHTTP(*mut c_void);
impl _wxHTTP for wxHTTP {}
impl _wxProtocol for wxHTTP {}
impl _wxSocketClient for wxHTTP {}
impl _wxSocketBase for wxHTTP {}
impl _wxObject for wxHTTP { fn handle(&self) -> *mut c_void { **self } }

impl wxHTTP {
    pub fn from(handle: *mut c_void) -> @wxHTTP { @wxHTTP(handle) }
    pub fn null() -> @wxHTTP { wxHTTP::from(0 as *mut c_void) }
    
}

pub trait _wxHTTP : _wxProtocol {
}

pub struct wxIPV4address(*mut c_void);
impl _wxIPV4address for wxIPV4address {}
impl _wxSockAddress for wxIPV4address {}
impl _wxObject for wxIPV4address { fn handle(&self) -> *mut c_void { **self } }

impl wxIPV4address {
    pub fn from(handle: *mut c_void) -> @wxIPV4address { @wxIPV4address(handle) }
    pub fn null() -> @wxIPV4address { wxIPV4address::from(0 as *mut c_void) }
    
}

pub trait _wxIPV4address : _wxSockAddress {
}

pub struct wxProtocol(*mut c_void);
impl _wxProtocol for wxProtocol {}
impl _wxSocketClient for wxProtocol {}
impl _wxSocketBase for wxProtocol {}
impl _wxObject for wxProtocol { fn handle(&self) -> *mut c_void { **self } }

impl wxProtocol {
    pub fn from(handle: *mut c_void) -> @wxProtocol { @wxProtocol(handle) }
    pub fn null() -> @wxProtocol { wxProtocol::from(0 as *mut c_void) }
    
}

pub trait _wxProtocol : _wxSocketClient {
}

pub struct wxSockAddress(*mut c_void);
impl _wxSockAddress for wxSockAddress {}
impl _wxObject for wxSockAddress { fn handle(&self) -> *mut c_void { **self } }

impl wxSockAddress {
    pub fn from(handle: *mut c_void) -> @wxSockAddress { @wxSockAddress(handle) }
    pub fn null() -> @wxSockAddress { wxSockAddress::from(0 as *mut c_void) }
    
}

pub trait _wxSockAddress : _wxObject {
}

pub struct wxSocketBase(*mut c_void);
impl _wxSocketBase for wxSocketBase {}
impl _wxObject for wxSocketBase { fn handle(&self) -> *mut c_void { **self } }

impl wxSocketBase {
    pub fn from(handle: *mut c_void) -> @wxSocketBase { @wxSocketBase(handle) }
    pub fn null() -> @wxSocketBase { wxSocketBase::from(0 as *mut c_void) }
    
}

pub trait _wxSocketBase : _wxObject {
}

pub struct wxSocketClient(*mut c_void);
impl _wxSocketClient for wxSocketClient {}
impl _wxSocketBase for wxSocketClient {}
impl _wxObject for wxSocketClient { fn handle(&self) -> *mut c_void { **self } }

impl wxSocketClient {
    pub fn from(handle: *mut c_void) -> @wxSocketClient { @wxSocketClient(handle) }
    pub fn null() -> @wxSocketClient { wxSocketClient::from(0 as *mut c_void) }
    
}

pub trait _wxSocketClient : _wxSocketBase {
}

pub struct wxSocketInputStream(*mut c_void);
impl _wxSocketInputStream for wxSocketInputStream {}
impl _wxInputStream for wxSocketInputStream {}
impl _wxStreamBase for wxSocketInputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxSocketInputStream {
    pub fn from(handle: *mut c_void) -> @wxSocketInputStream { @wxSocketInputStream(handle) }
    pub fn null() -> @wxSocketInputStream { wxSocketInputStream::from(0 as *mut c_void) }
    
}

pub trait _wxSocketInputStream : _wxInputStream {
}

pub struct wxSocketOutputStream(*mut c_void);
impl _wxSocketOutputStream for wxSocketOutputStream {}
impl _wxOutputStream for wxSocketOutputStream {}
impl _wxStreamBase for wxSocketOutputStream { fn handle(&self) -> *mut c_void { **self } }

impl wxSocketOutputStream {
    pub fn from(handle: *mut c_void) -> @wxSocketOutputStream { @wxSocketOutputStream(handle) }
    pub fn null() -> @wxSocketOutputStream { wxSocketOutputStream::from(0 as *mut c_void) }
    
}

pub trait _wxSocketOutputStream : _wxOutputStream {
}

pub struct wxSocketServer(*mut c_void);
impl _wxSocketServer for wxSocketServer {}
impl _wxSocketBase for wxSocketServer {}
impl _wxObject for wxSocketServer { fn handle(&self) -> *mut c_void { **self } }

impl wxSocketServer {
    pub fn from(handle: *mut c_void) -> @wxSocketServer { @wxSocketServer(handle) }
    pub fn null() -> @wxSocketServer { wxSocketServer::from(0 as *mut c_void) }
    
}

pub trait _wxSocketServer : _wxSocketBase {
}

pub struct wxURL(*mut c_void);
impl _wxURL for wxURL {}
impl _wxObject for wxURL { fn handle(&self) -> *mut c_void { **self } }

impl wxURL {
    pub fn from(handle: *mut c_void) -> @wxURL { @wxURL(handle) }
    pub fn null() -> @wxURL { wxURL::from(0 as *mut c_void) }
    
}

pub trait _wxURL : _wxObject {
}

