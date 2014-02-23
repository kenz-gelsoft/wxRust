use std::libc::*;
use base::*;
use core::*;

pub struct WxrMessageParameters { ptr: *mut c_void }
impl TWxrMessageParameters for WxrMessageParameters { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrMessageParameters {
    pub fn from(ptr: *mut c_void) -> WxrMessageParameters { WxrMessageParameters { ptr: ptr } }
    pub fn null() -> WxrMessageParameters { WxrMessageParameters::from(0 as *mut c_void) }
    
}

pub trait TWxrMessageParameters {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxrPlotCurve { ptr: *mut c_void }
impl TWxrPlotCurve for WxrPlotCurve {}
impl TWxPlotCurve for WxrPlotCurve {}
impl TWxObject for WxrPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrPlotCurve {
    pub fn from(ptr: *mut c_void) -> WxrPlotCurve { WxrPlotCurve { ptr: ptr } }
    pub fn null() -> WxrPlotCurve { WxrPlotCurve::from(0 as *mut c_void) }
    
}

pub trait TWxrPlotCurve : TWxPlotCurve {
}

pub struct WxDynToolInfo { ptr: *mut c_void }
impl TWxDynToolInfo for WxDynToolInfo {}
impl TWxToolLayoutItem for WxDynToolInfo {}
impl TWxObject for WxDynToolInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDynToolInfo {
    pub fn from(ptr: *mut c_void) -> WxDynToolInfo { WxDynToolInfo { ptr: ptr } }
    pub fn null() -> WxDynToolInfo { WxDynToolInfo::from(0 as *mut c_void) }
    
}

pub trait TWxDynToolInfo : TWxToolLayoutItem {
}

pub struct WxDynamicSashWindow { ptr: *mut c_void }
impl TWxDynamicSashWindow for WxDynamicSashWindow {}
impl TWxWindow for WxDynamicSashWindow {}
impl TWxEvtHandler for WxDynamicSashWindow {}
impl TWxObject for WxDynamicSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDynamicSashWindow {
    pub fn from(ptr: *mut c_void) -> WxDynamicSashWindow { WxDynamicSashWindow { ptr: ptr } }
    pub fn null() -> WxDynamicSashWindow { WxDynamicSashWindow::from(0 as *mut c_void) }
    
}

pub trait TWxDynamicSashWindow : TWxWindow {
}

pub struct WxDynamicToolBar { ptr: *mut c_void }
impl TWxDynamicToolBar for WxDynamicToolBar {}
impl TWxToolBarBase for WxDynamicToolBar {}
impl TWxControl for WxDynamicToolBar {}
impl TWxWindow for WxDynamicToolBar {}
impl TWxEvtHandler for WxDynamicToolBar {}
impl TWxObject for WxDynamicToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDynamicToolBar {
    pub fn from(ptr: *mut c_void) -> WxDynamicToolBar { WxDynamicToolBar { ptr: ptr } }
    pub fn null() -> WxDynamicToolBar { WxDynamicToolBar::from(0 as *mut c_void) }
    
}

pub trait TWxDynamicToolBar : TWxToolBarBase {
}

pub struct WxExpr { ptr: *mut c_void }
impl TWxExpr for WxExpr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxExpr {
    pub fn from(ptr: *mut c_void) -> WxExpr { WxExpr { ptr: ptr } }
    pub fn null() -> WxExpr { WxExpr::from(0 as *mut c_void) }
    
}

pub trait TWxExpr {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxExprDatabase { ptr: *mut c_void }
impl TWxExprDatabase for WxExprDatabase {}
impl TWxList for WxExprDatabase {}
impl TWxObject for WxExprDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxExprDatabase {
    pub fn from(ptr: *mut c_void) -> WxExprDatabase { WxExprDatabase { ptr: ptr } }
    pub fn null() -> WxExprDatabase { WxExprDatabase::from(0 as *mut c_void) }
    
}

pub trait TWxExprDatabase : TWxList {
}

pub struct WxFrameLayout { ptr: *mut c_void }
impl TWxFrameLayout for WxFrameLayout {}
impl TWxEvtHandler for WxFrameLayout {}
impl TWxObject for WxFrameLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFrameLayout {
    pub fn from(ptr: *mut c_void) -> WxFrameLayout { WxFrameLayout { ptr: ptr } }
    pub fn null() -> WxFrameLayout { WxFrameLayout::from(0 as *mut c_void) }
    
}

pub trait TWxFrameLayout : TWxEvtHandler {
}

pub struct WxHashMap { ptr: *mut c_void }
impl TWxHashMap for WxHashMap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHashMap {
    pub fn from(ptr: *mut c_void) -> WxHashMap { WxHashMap { ptr: ptr } }
    pub fn null() -> WxHashMap { WxHashMap::from(0 as *mut c_void) }
    
}

pub trait TWxHashMap {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxLEDNumberCtrl { ptr: *mut c_void }
impl TWxLEDNumberCtrl for WxLEDNumberCtrl {}
impl TWxControl for WxLEDNumberCtrl {}
impl TWxWindow for WxLEDNumberCtrl {}
impl TWxEvtHandler for WxLEDNumberCtrl {}
impl TWxObject for WxLEDNumberCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLEDNumberCtrl {
    pub fn from(ptr: *mut c_void) -> WxLEDNumberCtrl { WxLEDNumberCtrl { ptr: ptr } }
    pub fn null() -> WxLEDNumberCtrl { WxLEDNumberCtrl::from(0 as *mut c_void) }
    
}

pub trait TWxLEDNumberCtrl : TWxControl {
}

pub struct WxMBConvFile { ptr: *mut c_void }
impl TWxMBConvFile for WxMBConvFile {}
impl TWxMBConv for WxMBConvFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMBConvFile {
    pub fn from(ptr: *mut c_void) -> WxMBConvFile { WxMBConvFile { ptr: ptr } }
    pub fn null() -> WxMBConvFile { WxMBConvFile::from(0 as *mut c_void) }
    
}

pub trait TWxMBConvFile : TWxMBConv {
}

pub struct WxMultiCellCanvas { ptr: *mut c_void }
impl TWxMultiCellCanvas for WxMultiCellCanvas {}
impl TWxFlexGridSizer for WxMultiCellCanvas {}
impl TWxGridSizer for WxMultiCellCanvas {}
impl TWxSizer for WxMultiCellCanvas {}
impl TWxObject for WxMultiCellCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMultiCellCanvas {
    pub fn from(ptr: *mut c_void) -> WxMultiCellCanvas { WxMultiCellCanvas { ptr: ptr } }
    pub fn null() -> WxMultiCellCanvas { WxMultiCellCanvas::from(0 as *mut c_void) }
    
}

pub trait TWxMultiCellCanvas : TWxFlexGridSizer {
}

pub struct WxMultiCellItemHandle { ptr: *mut c_void }
impl TWxMultiCellItemHandle for WxMultiCellItemHandle {}
impl TWxObject for WxMultiCellItemHandle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMultiCellItemHandle {
    pub fn from(ptr: *mut c_void) -> WxMultiCellItemHandle { WxMultiCellItemHandle { ptr: ptr } }
    pub fn null() -> WxMultiCellItemHandle { WxMultiCellItemHandle::from(0 as *mut c_void) }
    
}

pub trait TWxMultiCellItemHandle : TWxObject {
}

pub struct WxMultiCellSizer { ptr: *mut c_void }
impl TWxMultiCellSizer for WxMultiCellSizer {}
impl TWxSizer for WxMultiCellSizer {}
impl TWxObject for WxMultiCellSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMultiCellSizer {
    pub fn from(ptr: *mut c_void) -> WxMultiCellSizer { WxMultiCellSizer { ptr: ptr } }
    pub fn null() -> WxMultiCellSizer { WxMultiCellSizer::from(0 as *mut c_void) }
    
}

pub trait TWxMultiCellSizer : TWxSizer {
}

pub struct WxNewBitmapButton { ptr: *mut c_void }
impl TWxNewBitmapButton for WxNewBitmapButton {}
impl TWxPanel for WxNewBitmapButton {}
impl TWxWindow for WxNewBitmapButton {}
impl TWxEvtHandler for WxNewBitmapButton {}
impl TWxObject for WxNewBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxNewBitmapButton {
    pub fn from(ptr: *mut c_void) -> WxNewBitmapButton { WxNewBitmapButton { ptr: ptr } }
    pub fn null() -> WxNewBitmapButton { WxNewBitmapButton::from(0 as *mut c_void) }
    
}

pub trait TWxNewBitmapButton : TWxPanel {
}

pub struct WxPlotCurve { ptr: *mut c_void }
impl TWxPlotCurve for WxPlotCurve {}
impl TWxObject for WxPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPlotCurve {
    pub fn from(ptr: *mut c_void) -> WxPlotCurve { WxPlotCurve { ptr: ptr } }
    pub fn null() -> WxPlotCurve { WxPlotCurve::from(0 as *mut c_void) }
    
}

pub trait TWxPlotCurve : TWxObject {
}

pub struct WxPlotEvent { ptr: *mut c_void }
impl TWxPlotEvent for WxPlotEvent {}
impl TWxNotifyEvent for WxPlotEvent {}
impl TWxCommandEvent for WxPlotEvent {}
impl TWxEvent for WxPlotEvent {}
impl TWxObject for WxPlotEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPlotEvent {
    pub fn from(ptr: *mut c_void) -> WxPlotEvent { WxPlotEvent { ptr: ptr } }
    pub fn null() -> WxPlotEvent { WxPlotEvent::from(0 as *mut c_void) }
    
}

pub trait TWxPlotEvent : TWxNotifyEvent {
}

pub struct WxPlotOnOffCurve { ptr: *mut c_void }
impl TWxPlotOnOffCurve for WxPlotOnOffCurve {}
impl TWxObject for WxPlotOnOffCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPlotOnOffCurve {
    pub fn from(ptr: *mut c_void) -> WxPlotOnOffCurve { WxPlotOnOffCurve { ptr: ptr } }
    pub fn null() -> WxPlotOnOffCurve { WxPlotOnOffCurve::from(0 as *mut c_void) }
    
}

pub trait TWxPlotOnOffCurve : TWxObject {
}

pub struct WxPlotWindow { ptr: *mut c_void }
impl TWxPlotWindow for WxPlotWindow {}
impl TWxScrolledWindow for WxPlotWindow {}
impl TWxPanel for WxPlotWindow {}
impl TWxWindow for WxPlotWindow {}
impl TWxEvtHandler for WxPlotWindow {}
impl TWxObject for WxPlotWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPlotWindow {
    pub fn from(ptr: *mut c_void) -> WxPlotWindow { WxPlotWindow { ptr: ptr } }
    pub fn null() -> WxPlotWindow { WxPlotWindow::from(0 as *mut c_void) }
    
}

pub trait TWxPlotWindow : TWxScrolledWindow {
}

pub struct WxRemotelyScrolledTreeCtrl { ptr: *mut c_void }
impl TWxRemotelyScrolledTreeCtrl for WxRemotelyScrolledTreeCtrl {}
impl TWxTreeCtrl for WxRemotelyScrolledTreeCtrl {}
impl TWxControl for WxRemotelyScrolledTreeCtrl {}
impl TWxWindow for WxRemotelyScrolledTreeCtrl {}
impl TWxEvtHandler for WxRemotelyScrolledTreeCtrl {}
impl TWxObject for WxRemotelyScrolledTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRemotelyScrolledTreeCtrl {
    pub fn from(ptr: *mut c_void) -> WxRemotelyScrolledTreeCtrl { WxRemotelyScrolledTreeCtrl { ptr: ptr } }
    pub fn null() -> WxRemotelyScrolledTreeCtrl { WxRemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

pub trait TWxRemotelyScrolledTreeCtrl : TWxTreeCtrl {
}

pub struct WxSplitterScrolledWindow { ptr: *mut c_void }
impl TWxSplitterScrolledWindow for WxSplitterScrolledWindow {}
impl TWxScrolledWindow for WxSplitterScrolledWindow {}
impl TWxPanel for WxSplitterScrolledWindow {}
impl TWxWindow for WxSplitterScrolledWindow {}
impl TWxEvtHandler for WxSplitterScrolledWindow {}
impl TWxObject for WxSplitterScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSplitterScrolledWindow {
    pub fn from(ptr: *mut c_void) -> WxSplitterScrolledWindow { WxSplitterScrolledWindow { ptr: ptr } }
    pub fn null() -> WxSplitterScrolledWindow { WxSplitterScrolledWindow::from(0 as *mut c_void) }
    
}

pub trait TWxSplitterScrolledWindow : TWxScrolledWindow {
}

pub struct WxStreamToTextRedirector { ptr: *mut c_void }
impl TWxStreamToTextRedirector for WxStreamToTextRedirector { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStreamToTextRedirector {
    pub fn from(ptr: *mut c_void) -> WxStreamToTextRedirector { WxStreamToTextRedirector { ptr: ptr } }
    pub fn null() -> WxStreamToTextRedirector { WxStreamToTextRedirector::from(0 as *mut c_void) }
    
}

pub trait TWxStreamToTextRedirector {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxTabCtrl { ptr: *mut c_void }
impl TWxTabCtrl for WxTabCtrl {}
impl TWxControl for WxTabCtrl {}
impl TWxWindow for WxTabCtrl {}
impl TWxEvtHandler for WxTabCtrl {}
impl TWxObject for WxTabCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTabCtrl {
    pub fn from(ptr: *mut c_void) -> WxTabCtrl { WxTabCtrl { ptr: ptr } }
    pub fn null() -> WxTabCtrl { WxTabCtrl::from(0 as *mut c_void) }
    
}

pub trait TWxTabCtrl : TWxControl {
}

pub struct WxTabEvent { ptr: *mut c_void }
impl TWxTabEvent for WxTabEvent {}
impl TWxCommandEvent for WxTabEvent {}
impl TWxEvent for WxTabEvent {}
impl TWxObject for WxTabEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTabEvent {
    pub fn from(ptr: *mut c_void) -> WxTabEvent { WxTabEvent { ptr: ptr } }
    pub fn null() -> WxTabEvent { WxTabEvent::from(0 as *mut c_void) }
    
}

pub trait TWxTabEvent : TWxCommandEvent {
}

pub struct WxThinSplitterWindow { ptr: *mut c_void }
impl TWxThinSplitterWindow for WxThinSplitterWindow {}
impl TWxSplitterWindow for WxThinSplitterWindow {}
impl TWxWindow for WxThinSplitterWindow {}
impl TWxEvtHandler for WxThinSplitterWindow {}
impl TWxObject for WxThinSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxThinSplitterWindow {
    pub fn from(ptr: *mut c_void) -> WxThinSplitterWindow { WxThinSplitterWindow { ptr: ptr } }
    pub fn null() -> WxThinSplitterWindow { WxThinSplitterWindow::from(0 as *mut c_void) }
    
}

pub trait TWxThinSplitterWindow : TWxSplitterWindow {
}

pub struct WxTimerBase { ptr: *mut c_void }
impl TWxTimerBase for WxTimerBase {}
impl TWxObject for WxTimerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTimerBase {
    pub fn from(ptr: *mut c_void) -> WxTimerBase { WxTimerBase { ptr: ptr } }
    pub fn null() -> WxTimerBase { WxTimerBase::from(0 as *mut c_void) }
    
}

pub trait TWxTimerBase : TWxObject {
}

pub struct WxToolLayoutItem { ptr: *mut c_void }
impl TWxToolLayoutItem for WxToolLayoutItem {}
impl TWxObject for WxToolLayoutItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxToolLayoutItem {
    pub fn from(ptr: *mut c_void) -> WxToolLayoutItem { WxToolLayoutItem { ptr: ptr } }
    pub fn null() -> WxToolLayoutItem { WxToolLayoutItem::from(0 as *mut c_void) }
    
}

pub trait TWxToolLayoutItem : TWxObject {
}

pub struct WxToolWindow { ptr: *mut c_void }
impl TWxToolWindow for WxToolWindow {}
impl TWxFrame for WxToolWindow {}
impl TWxTopLevelWindow for WxToolWindow {}
impl TWxWindow for WxToolWindow {}
impl TWxEvtHandler for WxToolWindow {}
impl TWxObject for WxToolWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxToolWindow {
    pub fn from(ptr: *mut c_void) -> WxToolWindow { WxToolWindow { ptr: ptr } }
    pub fn null() -> WxToolWindow { WxToolWindow::from(0 as *mut c_void) }
    
}

pub trait TWxToolWindow : TWxFrame {
}

pub struct WxTreeCompanionWindow { ptr: *mut c_void }
impl TWxTreeCompanionWindow for WxTreeCompanionWindow {}
impl TWxWindow for WxTreeCompanionWindow {}
impl TWxEvtHandler for WxTreeCompanionWindow {}
impl TWxObject for WxTreeCompanionWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeCompanionWindow {
    pub fn from(ptr: *mut c_void) -> WxTreeCompanionWindow { WxTreeCompanionWindow { ptr: ptr } }
    pub fn null() -> WxTreeCompanionWindow { WxTreeCompanionWindow::from(0 as *mut c_void) }
    
}

pub trait TWxTreeCompanionWindow : TWxWindow {
}

pub struct WxTreeLayout { ptr: *mut c_void }
impl TWxTreeLayout for WxTreeLayout {}
impl TWxObject for WxTreeLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeLayout {
    pub fn from(ptr: *mut c_void) -> WxTreeLayout { WxTreeLayout { ptr: ptr } }
    pub fn null() -> WxTreeLayout { WxTreeLayout::from(0 as *mut c_void) }
    
}

pub trait TWxTreeLayout : TWxObject {
}

pub struct WxTreeLayoutStored { ptr: *mut c_void }
impl TWxTreeLayoutStored for WxTreeLayoutStored {}
impl TWxTreeLayout for WxTreeLayoutStored {}
impl TWxObject for WxTreeLayoutStored { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeLayoutStored {
    pub fn from(ptr: *mut c_void) -> WxTreeLayoutStored { WxTreeLayoutStored { ptr: ptr } }
    pub fn null() -> WxTreeLayoutStored { WxTreeLayoutStored::from(0 as *mut c_void) }
    
}

pub trait TWxTreeLayoutStored : TWxTreeLayout {
}

pub struct WxGauge95 { ptr: *mut c_void }
impl TWxGauge95 for WxGauge95 {}
impl TWxGauge for WxGauge95 {}
impl TWxControl for WxGauge95 {}
impl TWxWindow for WxGauge95 {}
impl TWxEvtHandler for WxGauge95 {}
impl TWxObject for WxGauge95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGauge95 {
    pub fn from(ptr: *mut c_void) -> WxGauge95 { WxGauge95 { ptr: ptr } }
    pub fn null() -> WxGauge95 { WxGauge95::from(0 as *mut c_void) }
    
}

pub trait TWxGauge95 : TWxGauge {
}

pub struct WxGaugeMSW { ptr: *mut c_void }
impl TWxGaugeMSW for WxGaugeMSW {}
impl TWxGauge for WxGaugeMSW {}
impl TWxControl for WxGaugeMSW {}
impl TWxWindow for WxGaugeMSW {}
impl TWxEvtHandler for WxGaugeMSW {}
impl TWxObject for WxGaugeMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGaugeMSW {
    pub fn from(ptr: *mut c_void) -> WxGaugeMSW { WxGaugeMSW { ptr: ptr } }
    pub fn null() -> WxGaugeMSW { WxGaugeMSW::from(0 as *mut c_void) }
    
}

pub trait TWxGaugeMSW : TWxGauge {
}

pub struct WxSlider95 { ptr: *mut c_void }
impl TWxSlider95 for WxSlider95 {}
impl TWxSlider for WxSlider95 {}
impl TWxControl for WxSlider95 {}
impl TWxWindow for WxSlider95 {}
impl TWxEvtHandler for WxSlider95 {}
impl TWxObject for WxSlider95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSlider95 {
    pub fn from(ptr: *mut c_void) -> WxSlider95 { WxSlider95 { ptr: ptr } }
    pub fn null() -> WxSlider95 { WxSlider95::from(0 as *mut c_void) }
    
}

pub trait TWxSlider95 : TWxSlider {
}

pub struct WxSliderMSW { ptr: *mut c_void }
impl TWxSliderMSW for WxSliderMSW {}
impl TWxSlider for WxSliderMSW {}
impl TWxControl for WxSliderMSW {}
impl TWxWindow for WxSliderMSW {}
impl TWxEvtHandler for WxSliderMSW {}
impl TWxObject for WxSliderMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSliderMSW {
    pub fn from(ptr: *mut c_void) -> WxSliderMSW { WxSliderMSW { ptr: ptr } }
    pub fn null() -> WxSliderMSW { WxSliderMSW::from(0 as *mut c_void) }
    
}

pub trait TWxSliderMSW : TWxSlider {
}

