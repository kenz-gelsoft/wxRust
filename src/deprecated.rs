use std::libc::*;
use base::*;
use core::*;
use advanced::*;
use native::*;


pub struct wxDynToolInfo(*mut c_void);
impl _wxDynToolInfo for wxDynToolInfo {}
impl _wxToolLayoutItem for wxDynToolInfo {}
impl _wxObject for wxDynToolInfo { fn handle(&self) -> *mut c_void { **self } }

impl wxDynToolInfo {
    pub fn from(handle: *mut c_void) -> @wxDynToolInfo { @wxDynToolInfo(handle) }
    pub fn null() -> @wxDynToolInfo { wxDynToolInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDynToolInfo : _wxToolLayoutItem {
}

pub struct wxDynamicSashWindow(*mut c_void);
impl _wxDynamicSashWindow for wxDynamicSashWindow {}
impl _wxWindow for wxDynamicSashWindow {}
impl _wxEvtHandler for wxDynamicSashWindow {}
impl _wxObject for wxDynamicSashWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxDynamicSashWindow {
    pub fn from(handle: *mut c_void) -> @wxDynamicSashWindow { @wxDynamicSashWindow(handle) }
    pub fn null() -> @wxDynamicSashWindow { wxDynamicSashWindow::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicSashWindow : _wxWindow {
}

pub struct wxDynamicToolBar(*mut c_void);
impl _wxDynamicToolBar for wxDynamicToolBar {}
impl _wxToolBarBase for wxDynamicToolBar {}
impl _wxControl for wxDynamicToolBar {}
impl _wxWindow for wxDynamicToolBar {}
impl _wxEvtHandler for wxDynamicToolBar {}
impl _wxObject for wxDynamicToolBar { fn handle(&self) -> *mut c_void { **self } }

impl wxDynamicToolBar {
    pub fn from(handle: *mut c_void) -> @wxDynamicToolBar { @wxDynamicToolBar(handle) }
    pub fn null() -> @wxDynamicToolBar { wxDynamicToolBar::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicToolBar : _wxToolBarBase {
}

pub struct wxExpr(*mut c_void);
impl _wxExpr for wxExpr { fn handle(&self) -> *mut c_void { **self } }

impl wxExpr {
    pub fn from(handle: *mut c_void) -> @wxExpr { @wxExpr(handle) }
    pub fn null() -> @wxExpr { wxExpr::from(0 as *mut c_void) }
    
}

pub trait _wxExpr {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxExprDatabase(*mut c_void);
impl _wxExprDatabase for wxExprDatabase {}
impl _wxList for wxExprDatabase {}
impl _wxObject for wxExprDatabase { fn handle(&self) -> *mut c_void { **self } }

impl wxExprDatabase {
    pub fn from(handle: *mut c_void) -> @wxExprDatabase { @wxExprDatabase(handle) }
    pub fn null() -> @wxExprDatabase { wxExprDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxExprDatabase : _wxList {
}

pub struct wxFrameLayout(*mut c_void);
impl _wxFrameLayout for wxFrameLayout {}
impl _wxEvtHandler for wxFrameLayout {}
impl _wxObject for wxFrameLayout { fn handle(&self) -> *mut c_void { **self } }

impl wxFrameLayout {
    pub fn from(handle: *mut c_void) -> @wxFrameLayout { @wxFrameLayout(handle) }
    pub fn null() -> @wxFrameLayout { wxFrameLayout::from(0 as *mut c_void) }
    
}

pub trait _wxFrameLayout : _wxEvtHandler {
}

pub struct wxHashMap(*mut c_void);
impl _wxHashMap for wxHashMap { fn handle(&self) -> *mut c_void { **self } }

impl wxHashMap {
    pub fn from(handle: *mut c_void) -> @wxHashMap { @wxHashMap(handle) }
    pub fn null() -> @wxHashMap { wxHashMap::from(0 as *mut c_void) }
    
}

pub trait _wxHashMap {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxLEDNumberCtrl(*mut c_void);
impl _wxLEDNumberCtrl for wxLEDNumberCtrl {}
impl _wxControl for wxLEDNumberCtrl {}
impl _wxWindow for wxLEDNumberCtrl {}
impl _wxEvtHandler for wxLEDNumberCtrl {}
impl _wxObject for wxLEDNumberCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxLEDNumberCtrl {
    pub fn from(handle: *mut c_void) -> @wxLEDNumberCtrl { @wxLEDNumberCtrl(handle) }
    pub fn null() -> @wxLEDNumberCtrl { wxLEDNumberCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxLEDNumberCtrl : _wxControl {
}

pub struct wxMBConvFile(*mut c_void);
impl _wxMBConvFile for wxMBConvFile {}
impl _wxMBConv for wxMBConvFile { fn handle(&self) -> *mut c_void { **self } }

impl wxMBConvFile {
    pub fn from(handle: *mut c_void) -> @wxMBConvFile { @wxMBConvFile(handle) }
    pub fn null() -> @wxMBConvFile { wxMBConvFile::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvFile : _wxMBConv {
}

pub struct wxMultiCellCanvas(*mut c_void);
impl _wxMultiCellCanvas for wxMultiCellCanvas {}
impl _wxFlexGridSizer for wxMultiCellCanvas {}
impl _wxGridSizer for wxMultiCellCanvas {}
impl _wxSizer for wxMultiCellCanvas {}
impl _wxObject for wxMultiCellCanvas { fn handle(&self) -> *mut c_void { **self } }

impl wxMultiCellCanvas {
    pub fn from(handle: *mut c_void) -> @wxMultiCellCanvas { @wxMultiCellCanvas(handle) }
    pub fn null() -> @wxMultiCellCanvas { wxMultiCellCanvas::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellCanvas : _wxFlexGridSizer {
}

pub struct wxMultiCellItemHandle(*mut c_void);
impl _wxMultiCellItemHandle for wxMultiCellItemHandle {}
impl _wxObject for wxMultiCellItemHandle { fn handle(&self) -> *mut c_void { **self } }

impl wxMultiCellItemHandle {
    pub fn from(handle: *mut c_void) -> @wxMultiCellItemHandle { @wxMultiCellItemHandle(handle) }
    pub fn null() -> @wxMultiCellItemHandle { wxMultiCellItemHandle::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellItemHandle : _wxObject {
}

pub struct wxMultiCellSizer(*mut c_void);
impl _wxMultiCellSizer for wxMultiCellSizer {}
impl _wxSizer for wxMultiCellSizer {}
impl _wxObject for wxMultiCellSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxMultiCellSizer {
    pub fn from(handle: *mut c_void) -> @wxMultiCellSizer { @wxMultiCellSizer(handle) }
    pub fn null() -> @wxMultiCellSizer { wxMultiCellSizer::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellSizer : _wxSizer {
}

pub struct wxNewBitmapButton(*mut c_void);
impl _wxNewBitmapButton for wxNewBitmapButton {}
impl _wxPanel for wxNewBitmapButton {}
impl _wxWindow for wxNewBitmapButton {}
impl _wxEvtHandler for wxNewBitmapButton {}
impl _wxObject for wxNewBitmapButton { fn handle(&self) -> *mut c_void { **self } }

impl wxNewBitmapButton {
    pub fn from(handle: *mut c_void) -> @wxNewBitmapButton { @wxNewBitmapButton(handle) }
    pub fn null() -> @wxNewBitmapButton { wxNewBitmapButton::from(0 as *mut c_void) }
    
}

pub trait _wxNewBitmapButton : _wxPanel {
}

pub struct wxToolLayoutItem(*mut c_void);
impl _wxToolLayoutItem for wxToolLayoutItem {}
impl _wxObject for wxToolLayoutItem { fn handle(&self) -> *mut c_void { **self } }

impl wxToolLayoutItem {
    pub fn from(handle: *mut c_void) -> @wxToolLayoutItem { @wxToolLayoutItem(handle) }
    pub fn null() -> @wxToolLayoutItem { wxToolLayoutItem::from(0 as *mut c_void) }
    
}

pub trait _wxToolLayoutItem : _wxObject {
}

pub struct wxGauge95(*mut c_void);
impl _wxGauge95 for wxGauge95 {}
impl _wxGauge for wxGauge95 {}
impl _wxControl for wxGauge95 {}
impl _wxWindow for wxGauge95 {}
impl _wxEvtHandler for wxGauge95 {}
impl _wxObject for wxGauge95 { fn handle(&self) -> *mut c_void { **self } }

impl wxGauge95 {
    pub fn from(handle: *mut c_void) -> @wxGauge95 { @wxGauge95(handle) }
    pub fn null() -> @wxGauge95 { wxGauge95::from(0 as *mut c_void) }
    
}

pub trait _wxGauge95 : _wxGauge {
}

pub struct wxGaugeMSW(*mut c_void);
impl _wxGaugeMSW for wxGaugeMSW {}
impl _wxGauge for wxGaugeMSW {}
impl _wxControl for wxGaugeMSW {}
impl _wxWindow for wxGaugeMSW {}
impl _wxEvtHandler for wxGaugeMSW {}
impl _wxObject for wxGaugeMSW { fn handle(&self) -> *mut c_void { **self } }

impl wxGaugeMSW {
    pub fn from(handle: *mut c_void) -> @wxGaugeMSW { @wxGaugeMSW(handle) }
    pub fn null() -> @wxGaugeMSW { wxGaugeMSW::from(0 as *mut c_void) }
    
}

pub trait _wxGaugeMSW : _wxGauge {
}

