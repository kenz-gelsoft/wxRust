use std::libc::*;
use base::*;
use core::*;

/// Wraps the wxWidgets' [wxFTP](http://docs.wxwidgets.org/3.0/classwx_ftp.html) class.
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

/// Methods of the wxWidgets' [wxFTP](http://docs.wxwidgets.org/3.0/classwx_ftp.html) class.
pub trait TFTP : TProtocol {
}

/// Wraps the wxWidgets' [wxHTTP](http://docs.wxwidgets.org/3.0/classwx_http.html) class.
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

/// Methods of the wxWidgets' [wxHTTP](http://docs.wxwidgets.org/3.0/classwx_http.html) class.
pub trait THTTP : TProtocol {
}

/// Wraps the wxWidgets' [wxIPV4address](http://docs.wxwidgets.org/3.0/classwx_ipv4_address.html) class.
pub struct IPV4address { ptr: *mut c_void }
impl TIPV4address for IPV4address {}
impl TSockAddress for IPV4address {}
impl TObject for IPV4address { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IPV4address {
    pub fn from(ptr: *mut c_void) -> IPV4address { IPV4address { ptr: ptr } }
    pub fn null() -> IPV4address { IPV4address::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxIPV4address](http://docs.wxwidgets.org/3.0/classwx_ipv4_address.html) class.
pub trait TIPV4address : TSockAddress {
}

/// Wraps the wxWidgets' [wxProtocol](http://docs.wxwidgets.org/3.0/classwx_protocol.html) class.
pub struct Protocol { ptr: *mut c_void }
impl TProtocol for Protocol {}
impl TSocketClient for Protocol {}
impl TSocketBase for Protocol {}
impl TObject for Protocol { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Protocol {
    pub fn from(ptr: *mut c_void) -> Protocol { Protocol { ptr: ptr } }
    pub fn null() -> Protocol { Protocol::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxProtocol](http://docs.wxwidgets.org/3.0/classwx_protocol.html) class.
pub trait TProtocol : TSocketClient {
}

/// Wraps the wxWidgets' [wxSockAddress](http://docs.wxwidgets.org/3.0/classwx_sock_address.html) class.
pub struct SockAddress { ptr: *mut c_void }
impl TSockAddress for SockAddress {}
impl TObject for SockAddress { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SockAddress {
    pub fn from(ptr: *mut c_void) -> SockAddress { SockAddress { ptr: ptr } }
    pub fn null() -> SockAddress { SockAddress::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSockAddress](http://docs.wxwidgets.org/3.0/classwx_sock_address.html) class.
pub trait TSockAddress : TObject {
}

/// Wraps the wxWidgets' [wxSocketBase](http://docs.wxwidgets.org/3.0/classwx_socket_base.html) class.
pub struct SocketBase { ptr: *mut c_void }
impl TSocketBase for SocketBase {}
impl TObject for SocketBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketBase {
    pub fn from(ptr: *mut c_void) -> SocketBase { SocketBase { ptr: ptr } }
    pub fn null() -> SocketBase { SocketBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSocketBase](http://docs.wxwidgets.org/3.0/classwx_socket_base.html) class.
pub trait TSocketBase : TObject {
}

/// Wraps the wxWidgets' [wxSocketClient](http://docs.wxwidgets.org/3.0/classwx_socket_client.html) class.
pub struct SocketClient { ptr: *mut c_void }
impl TSocketClient for SocketClient {}
impl TSocketBase for SocketClient {}
impl TObject for SocketClient { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketClient {
    pub fn from(ptr: *mut c_void) -> SocketClient { SocketClient { ptr: ptr } }
    pub fn null() -> SocketClient { SocketClient::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSocketClient](http://docs.wxwidgets.org/3.0/classwx_socket_client.html) class.
pub trait TSocketClient : TSocketBase {
}

/// Wraps the wxWidgets' [wxSocketEvent](http://docs.wxwidgets.org/3.0/classwx_socket_event.html) class.
pub struct SocketEvent { ptr: *mut c_void }
impl TSocketEvent for SocketEvent {}
impl TEvent for SocketEvent {}
impl TObject for SocketEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketEvent {
    pub fn from(ptr: *mut c_void) -> SocketEvent { SocketEvent { ptr: ptr } }
    pub fn null() -> SocketEvent { SocketEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSocketEvent](http://docs.wxwidgets.org/3.0/classwx_socket_event.html) class.
pub trait TSocketEvent : TEvent {
}

/// Wraps the wxWidgets' [wxSocketInputStream](http://docs.wxwidgets.org/3.0/classwx_socket_input_stream.html) class.
pub struct SocketInputStream { ptr: *mut c_void }
impl TSocketInputStream for SocketInputStream {}
impl TInputStream for SocketInputStream {}
impl TStreamBase for SocketInputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketInputStream {
    pub fn from(ptr: *mut c_void) -> SocketInputStream { SocketInputStream { ptr: ptr } }
    pub fn null() -> SocketInputStream { SocketInputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSocketInputStream](http://docs.wxwidgets.org/3.0/classwx_socket_input_stream.html) class.
pub trait TSocketInputStream : TInputStream {
}

/// Wraps the wxWidgets' [wxSocketOutputStream](http://docs.wxwidgets.org/3.0/classwx_socket_output_stream.html) class.
pub struct SocketOutputStream { ptr: *mut c_void }
impl TSocketOutputStream for SocketOutputStream {}
impl TOutputStream for SocketOutputStream {}
impl TStreamBase for SocketOutputStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketOutputStream {
    pub fn from(ptr: *mut c_void) -> SocketOutputStream { SocketOutputStream { ptr: ptr } }
    pub fn null() -> SocketOutputStream { SocketOutputStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSocketOutputStream](http://docs.wxwidgets.org/3.0/classwx_socket_output_stream.html) class.
pub trait TSocketOutputStream : TOutputStream {
}

/// Wraps the wxWidgets' [wxSocketServer](http://docs.wxwidgets.org/3.0/classwx_socket_server.html) class.
pub struct SocketServer { ptr: *mut c_void }
impl TSocketServer for SocketServer {}
impl TSocketBase for SocketServer {}
impl TObject for SocketServer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SocketServer {
    pub fn from(ptr: *mut c_void) -> SocketServer { SocketServer { ptr: ptr } }
    pub fn null() -> SocketServer { SocketServer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSocketServer](http://docs.wxwidgets.org/3.0/classwx_socket_server.html) class.
pub trait TSocketServer : TSocketBase {
}

/// Wraps the wxWidgets' [wxURL](http://docs.wxwidgets.org/3.0/classwx_url.html) class.
pub struct URL { ptr: *mut c_void }
impl TURL for URL {}
impl TObject for URL { fn ptr(&self) -> *mut c_void { self.ptr } }

impl URL {
    pub fn from(ptr: *mut c_void) -> URL { URL { ptr: ptr } }
    pub fn null() -> URL { URL::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxURL](http://docs.wxwidgets.org/3.0/classwx_url.html) class.
pub trait TURL : TObject {
}

