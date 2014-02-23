use std::libc::*;
use base::*;
use core::*;

pub struct WxFTP { ptr: *mut c_void }
impl TWxFTP for WxFTP {}
impl TWxProtocol for WxFTP {}
impl TWxSocketClient for WxFTP {}
impl TWxSocketBase for WxFTP {}
impl TWxObject for WxFTP { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFTP {
    pub fn from(ptr: *mut c_void) -> WxFTP { WxFTP { ptr: ptr } }
    pub fn null() -> WxFTP { WxFTP::from(0 as *mut c_void) }
    
}

pub trait TWxFTP : TWxProtocol {
}

pub struct WxHTTP { ptr: *mut c_void }
impl TWxHTTP for WxHTTP {}
impl TWxProtocol for WxHTTP {}
impl TWxSocketClient for WxHTTP {}
impl TWxSocketBase for WxHTTP {}
impl TWxObject for WxHTTP { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHTTP {
    pub fn from(ptr: *mut c_void) -> WxHTTP { WxHTTP { ptr: ptr } }
    pub fn null() -> WxHTTP { WxHTTP::from(0 as *mut c_void) }
    
}

pub trait TWxHTTP : TWxProtocol {
}

pub struct WxIPV4address { ptr: *mut c_void }
impl TWxIPV4address for WxIPV4address {}
impl TWxSockAddress for WxIPV4address {}
impl TWxObject for WxIPV4address { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIPV4address {
    pub fn from(ptr: *mut c_void) -> WxIPV4address { WxIPV4address { ptr: ptr } }
    pub fn null() -> WxIPV4address { WxIPV4address::from(0 as *mut c_void) }
    
}

pub trait TWxIPV4address : TWxSockAddress {
}

pub struct WxProtocol { ptr: *mut c_void }
impl TWxProtocol for WxProtocol {}
impl TWxSocketClient for WxProtocol {}
impl TWxSocketBase for WxProtocol {}
impl TWxObject for WxProtocol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxProtocol {
    pub fn from(ptr: *mut c_void) -> WxProtocol { WxProtocol { ptr: ptr } }
    pub fn null() -> WxProtocol { WxProtocol::from(0 as *mut c_void) }
    
}

pub trait TWxProtocol : TWxSocketClient {
}

pub struct WxSockAddress { ptr: *mut c_void }
impl TWxSockAddress for WxSockAddress {}
impl TWxObject for WxSockAddress { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSockAddress {
    pub fn from(ptr: *mut c_void) -> WxSockAddress { WxSockAddress { ptr: ptr } }
    pub fn null() -> WxSockAddress { WxSockAddress::from(0 as *mut c_void) }
    
}

pub trait TWxSockAddress : TWxObject {
}

pub struct WxSocketBase { ptr: *mut c_void }
impl TWxSocketBase for WxSocketBase {}
impl TWxObject for WxSocketBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSocketBase {
    pub fn from(ptr: *mut c_void) -> WxSocketBase { WxSocketBase { ptr: ptr } }
    pub fn null() -> WxSocketBase { WxSocketBase::from(0 as *mut c_void) }
    
}

pub trait TWxSocketBase : TWxObject {
}

pub struct WxSocketClient { ptr: *mut c_void }
impl TWxSocketClient for WxSocketClient {}
impl TWxSocketBase for WxSocketClient {}
impl TWxObject for WxSocketClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSocketClient {
    pub fn from(ptr: *mut c_void) -> WxSocketClient { WxSocketClient { ptr: ptr } }
    pub fn null() -> WxSocketClient { WxSocketClient::from(0 as *mut c_void) }
    
}

pub trait TWxSocketClient : TWxSocketBase {
}

pub struct WxSocketEvent { ptr: *mut c_void }
impl TWxSocketEvent for WxSocketEvent {}
impl TWxEvent for WxSocketEvent {}
impl TWxObject for WxSocketEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSocketEvent {
    pub fn from(ptr: *mut c_void) -> WxSocketEvent { WxSocketEvent { ptr: ptr } }
    pub fn null() -> WxSocketEvent { WxSocketEvent::from(0 as *mut c_void) }
    
}

pub trait TWxSocketEvent : TWxEvent {
}

pub struct WxSocketInputStream { ptr: *mut c_void }
impl TWxSocketInputStream for WxSocketInputStream {}
impl TWxInputStream for WxSocketInputStream {}
impl TWxStreamBase for WxSocketInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSocketInputStream {
    pub fn from(ptr: *mut c_void) -> WxSocketInputStream { WxSocketInputStream { ptr: ptr } }
    pub fn null() -> WxSocketInputStream { WxSocketInputStream::from(0 as *mut c_void) }
    
}

pub trait TWxSocketInputStream : TWxInputStream {
}

pub struct WxSocketOutputStream { ptr: *mut c_void }
impl TWxSocketOutputStream for WxSocketOutputStream {}
impl TWxOutputStream for WxSocketOutputStream {}
impl TWxStreamBase for WxSocketOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSocketOutputStream {
    pub fn from(ptr: *mut c_void) -> WxSocketOutputStream { WxSocketOutputStream { ptr: ptr } }
    pub fn null() -> WxSocketOutputStream { WxSocketOutputStream::from(0 as *mut c_void) }
    
}

pub trait TWxSocketOutputStream : TWxOutputStream {
}

pub struct WxSocketServer { ptr: *mut c_void }
impl TWxSocketServer for WxSocketServer {}
impl TWxSocketBase for WxSocketServer {}
impl TWxObject for WxSocketServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSocketServer {
    pub fn from(ptr: *mut c_void) -> WxSocketServer { WxSocketServer { ptr: ptr } }
    pub fn null() -> WxSocketServer { WxSocketServer::from(0 as *mut c_void) }
    
}

pub trait TWxSocketServer : TWxSocketBase {
}

pub struct WxURL { ptr: *mut c_void }
impl TWxURL for WxURL {}
impl TWxObject for WxURL { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxURL {
    pub fn from(ptr: *mut c_void) -> WxURL { WxURL { ptr: ptr } }
    pub fn null() -> WxURL { WxURL::from(0 as *mut c_void) }
    
}

pub trait TWxURL : TWxObject {
}

