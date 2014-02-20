use std::libc::*;
use base::*;
use core::*;

pub struct wxRustMessageParameters { ptr: *mut c_void }
impl _wxRustMessageParameters for wxRustMessageParameters { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustMessageParameters {
    pub fn from(ptr: *mut c_void) -> wxRustMessageParameters { wxRustMessageParameters { ptr: ptr } }
    pub fn null() -> wxRustMessageParameters { wxRustMessageParameters::from(0 as *mut c_void) }
    
}

pub trait _wxRustMessageParameters {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxRustPlotCurve { ptr: *mut c_void }
impl _wxRustPlotCurve for wxRustPlotCurve {}
impl _wxPlotCurve for wxRustPlotCurve {}
impl _wxObject for wxRustPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustPlotCurve {
    pub fn from(ptr: *mut c_void) -> wxRustPlotCurve { wxRustPlotCurve { ptr: ptr } }
    pub fn null() -> wxRustPlotCurve { wxRustPlotCurve::from(0 as *mut c_void) }
    
}

pub trait _wxRustPlotCurve : _wxPlotCurve {
}

pub struct wxDynToolInfo { ptr: *mut c_void }
impl _wxDynToolInfo for wxDynToolInfo {}
impl _wxToolLayoutItem for wxDynToolInfo {}
impl _wxObject for wxDynToolInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDynToolInfo {
    pub fn from(ptr: *mut c_void) -> wxDynToolInfo { wxDynToolInfo { ptr: ptr } }
    pub fn null() -> wxDynToolInfo { wxDynToolInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDynToolInfo : _wxToolLayoutItem {
}

pub struct wxDynamicSashWindow { ptr: *mut c_void }
impl _wxDynamicSashWindow for wxDynamicSashWindow {}
impl _wxWindow for wxDynamicSashWindow {}
impl _wxEvtHandler for wxDynamicSashWindow {}
impl _wxObject for wxDynamicSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDynamicSashWindow {
    pub fn from(ptr: *mut c_void) -> wxDynamicSashWindow { wxDynamicSashWindow { ptr: ptr } }
    pub fn null() -> wxDynamicSashWindow { wxDynamicSashWindow::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicSashWindow : _wxWindow {
}

pub struct wxDynamicToolBar { ptr: *mut c_void }
impl _wxDynamicToolBar for wxDynamicToolBar {}
impl _wxToolBarBase for wxDynamicToolBar {}
impl _wxControl for wxDynamicToolBar {}
impl _wxWindow for wxDynamicToolBar {}
impl _wxEvtHandler for wxDynamicToolBar {}
impl _wxObject for wxDynamicToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDynamicToolBar {
    pub fn from(ptr: *mut c_void) -> wxDynamicToolBar { wxDynamicToolBar { ptr: ptr } }
    pub fn null() -> wxDynamicToolBar { wxDynamicToolBar::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicToolBar : _wxToolBarBase {
}

pub struct wxExpr { ptr: *mut c_void }
impl _wxExpr for wxExpr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxExpr {
    pub fn from(ptr: *mut c_void) -> wxExpr { wxExpr { ptr: ptr } }
    pub fn null() -> wxExpr { wxExpr::from(0 as *mut c_void) }
    
}

pub trait _wxExpr {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxExprDatabase { ptr: *mut c_void }
impl _wxExprDatabase for wxExprDatabase {}
impl _wxList for wxExprDatabase {}
impl _wxObject for wxExprDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxExprDatabase {
    pub fn from(ptr: *mut c_void) -> wxExprDatabase { wxExprDatabase { ptr: ptr } }
    pub fn null() -> wxExprDatabase { wxExprDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxExprDatabase : _wxList {
}

pub struct wxFrameLayout { ptr: *mut c_void }
impl _wxFrameLayout for wxFrameLayout {}
impl _wxEvtHandler for wxFrameLayout {}
impl _wxObject for wxFrameLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFrameLayout {
    pub fn from(ptr: *mut c_void) -> wxFrameLayout { wxFrameLayout { ptr: ptr } }
    pub fn null() -> wxFrameLayout { wxFrameLayout::from(0 as *mut c_void) }
    
}

pub trait _wxFrameLayout : _wxEvtHandler {
}

pub struct wxHashMap { ptr: *mut c_void }
impl _wxHashMap for wxHashMap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHashMap {
    pub fn from(ptr: *mut c_void) -> wxHashMap { wxHashMap { ptr: ptr } }
    pub fn null() -> wxHashMap { wxHashMap::from(0 as *mut c_void) }
    
}

pub trait _wxHashMap {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxLEDNumberCtrl { ptr: *mut c_void }
impl _wxLEDNumberCtrl for wxLEDNumberCtrl {}
impl _wxControl for wxLEDNumberCtrl {}
impl _wxWindow for wxLEDNumberCtrl {}
impl _wxEvtHandler for wxLEDNumberCtrl {}
impl _wxObject for wxLEDNumberCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLEDNumberCtrl {
    pub fn from(ptr: *mut c_void) -> wxLEDNumberCtrl { wxLEDNumberCtrl { ptr: ptr } }
    pub fn null() -> wxLEDNumberCtrl { wxLEDNumberCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxLEDNumberCtrl : _wxControl {
}

pub struct wxMBConvFile { ptr: *mut c_void }
impl _wxMBConvFile for wxMBConvFile {}
impl _wxMBConv for wxMBConvFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMBConvFile {
    pub fn from(ptr: *mut c_void) -> wxMBConvFile { wxMBConvFile { ptr: ptr } }
    pub fn null() -> wxMBConvFile { wxMBConvFile::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvFile : _wxMBConv {
}

pub struct wxMultiCellCanvas { ptr: *mut c_void }
impl _wxMultiCellCanvas for wxMultiCellCanvas {}
impl _wxFlexGridSizer for wxMultiCellCanvas {}
impl _wxGridSizer for wxMultiCellCanvas {}
impl _wxSizer for wxMultiCellCanvas {}
impl _wxObject for wxMultiCellCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMultiCellCanvas {
    pub fn from(ptr: *mut c_void) -> wxMultiCellCanvas { wxMultiCellCanvas { ptr: ptr } }
    pub fn null() -> wxMultiCellCanvas { wxMultiCellCanvas::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellCanvas : _wxFlexGridSizer {
}

pub struct wxMultiCellItemHandle { ptr: *mut c_void }
impl _wxMultiCellItemHandle for wxMultiCellItemHandle {}
impl _wxObject for wxMultiCellItemHandle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMultiCellItemHandle {
    pub fn from(ptr: *mut c_void) -> wxMultiCellItemHandle { wxMultiCellItemHandle { ptr: ptr } }
    pub fn null() -> wxMultiCellItemHandle { wxMultiCellItemHandle::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellItemHandle : _wxObject {
}

pub struct wxMultiCellSizer { ptr: *mut c_void }
impl _wxMultiCellSizer for wxMultiCellSizer {}
impl _wxSizer for wxMultiCellSizer {}
impl _wxObject for wxMultiCellSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMultiCellSizer {
    pub fn from(ptr: *mut c_void) -> wxMultiCellSizer { wxMultiCellSizer { ptr: ptr } }
    pub fn null() -> wxMultiCellSizer { wxMultiCellSizer::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellSizer : _wxSizer {
}

pub struct wxNewBitmapButton { ptr: *mut c_void }
impl _wxNewBitmapButton for wxNewBitmapButton {}
impl _wxPanel for wxNewBitmapButton {}
impl _wxWindow for wxNewBitmapButton {}
impl _wxEvtHandler for wxNewBitmapButton {}
impl _wxObject for wxNewBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxNewBitmapButton {
    pub fn from(ptr: *mut c_void) -> wxNewBitmapButton { wxNewBitmapButton { ptr: ptr } }
    pub fn null() -> wxNewBitmapButton { wxNewBitmapButton::from(0 as *mut c_void) }
    
}

pub trait _wxNewBitmapButton : _wxPanel {
}

pub struct wxPlotCurve { ptr: *mut c_void }
impl _wxPlotCurve for wxPlotCurve {}
impl _wxObject for wxPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPlotCurve {
    pub fn from(ptr: *mut c_void) -> wxPlotCurve { wxPlotCurve { ptr: ptr } }
    pub fn null() -> wxPlotCurve { wxPlotCurve::from(0 as *mut c_void) }
    
}

pub trait _wxPlotCurve : _wxObject {
}

pub struct wxPlotEvent { ptr: *mut c_void }
impl _wxPlotEvent for wxPlotEvent {}
impl _wxNotifyEvent for wxPlotEvent {}
impl _wxCommandEvent for wxPlotEvent {}
impl _wxEvent for wxPlotEvent {}
impl _wxObject for wxPlotEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPlotEvent {
    pub fn from(ptr: *mut c_void) -> wxPlotEvent { wxPlotEvent { ptr: ptr } }
    pub fn null() -> wxPlotEvent { wxPlotEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPlotEvent : _wxNotifyEvent {
}

pub struct wxPlotOnOffCurve { ptr: *mut c_void }
impl _wxPlotOnOffCurve for wxPlotOnOffCurve {}
impl _wxObject for wxPlotOnOffCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPlotOnOffCurve {
    pub fn from(ptr: *mut c_void) -> wxPlotOnOffCurve { wxPlotOnOffCurve { ptr: ptr } }
    pub fn null() -> wxPlotOnOffCurve { wxPlotOnOffCurve::from(0 as *mut c_void) }
    
}

pub trait _wxPlotOnOffCurve : _wxObject {
}

pub struct wxPlotWindow { ptr: *mut c_void }
impl _wxPlotWindow for wxPlotWindow {}
impl _wxScrolledWindow for wxPlotWindow {}
impl _wxPanel for wxPlotWindow {}
impl _wxWindow for wxPlotWindow {}
impl _wxEvtHandler for wxPlotWindow {}
impl _wxObject for wxPlotWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPlotWindow {
    pub fn from(ptr: *mut c_void) -> wxPlotWindow { wxPlotWindow { ptr: ptr } }
    pub fn null() -> wxPlotWindow { wxPlotWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPlotWindow : _wxScrolledWindow {
}

pub struct wxRemotelyScrolledTreeCtrl { ptr: *mut c_void }
impl _wxRemotelyScrolledTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxControl for wxRemotelyScrolledTreeCtrl {}
impl _wxWindow for wxRemotelyScrolledTreeCtrl {}
impl _wxEvtHandler for wxRemotelyScrolledTreeCtrl {}
impl _wxObject for wxRemotelyScrolledTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRemotelyScrolledTreeCtrl {
    pub fn from(ptr: *mut c_void) -> wxRemotelyScrolledTreeCtrl { wxRemotelyScrolledTreeCtrl { ptr: ptr } }
    pub fn null() -> wxRemotelyScrolledTreeCtrl { wxRemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxRemotelyScrolledTreeCtrl : _wxTreeCtrl {
}

pub struct wxSplitterScrolledWindow { ptr: *mut c_void }
impl _wxSplitterScrolledWindow for wxSplitterScrolledWindow {}
impl _wxScrolledWindow for wxSplitterScrolledWindow {}
impl _wxPanel for wxSplitterScrolledWindow {}
impl _wxWindow for wxSplitterScrolledWindow {}
impl _wxEvtHandler for wxSplitterScrolledWindow {}
impl _wxObject for wxSplitterScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSplitterScrolledWindow {
    pub fn from(ptr: *mut c_void) -> wxSplitterScrolledWindow { wxSplitterScrolledWindow { ptr: ptr } }
    pub fn null() -> wxSplitterScrolledWindow { wxSplitterScrolledWindow::from(0 as *mut c_void) }
    
}

pub trait _wxSplitterScrolledWindow : _wxScrolledWindow {
}

pub struct wxStreamToTextRedirector { ptr: *mut c_void }
impl _wxStreamToTextRedirector for wxStreamToTextRedirector { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStreamToTextRedirector {
    pub fn from(ptr: *mut c_void) -> wxStreamToTextRedirector { wxStreamToTextRedirector { ptr: ptr } }
    pub fn null() -> wxStreamToTextRedirector { wxStreamToTextRedirector::from(0 as *mut c_void) }
    
}

pub trait _wxStreamToTextRedirector {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxTabCtrl { ptr: *mut c_void }
impl _wxTabCtrl for wxTabCtrl {}
impl _wxControl for wxTabCtrl {}
impl _wxWindow for wxTabCtrl {}
impl _wxEvtHandler for wxTabCtrl {}
impl _wxObject for wxTabCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTabCtrl {
    pub fn from(ptr: *mut c_void) -> wxTabCtrl { wxTabCtrl { ptr: ptr } }
    pub fn null() -> wxTabCtrl { wxTabCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxTabCtrl : _wxControl {
}

pub struct wxTabEvent { ptr: *mut c_void }
impl _wxTabEvent for wxTabEvent {}
impl _wxCommandEvent for wxTabEvent {}
impl _wxEvent for wxTabEvent {}
impl _wxObject for wxTabEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTabEvent {
    pub fn from(ptr: *mut c_void) -> wxTabEvent { wxTabEvent { ptr: ptr } }
    pub fn null() -> wxTabEvent { wxTabEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTabEvent : _wxCommandEvent {
}

pub struct wxThinSplitterWindow { ptr: *mut c_void }
impl _wxThinSplitterWindow for wxThinSplitterWindow {}
impl _wxSplitterWindow for wxThinSplitterWindow {}
impl _wxWindow for wxThinSplitterWindow {}
impl _wxEvtHandler for wxThinSplitterWindow {}
impl _wxObject for wxThinSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxThinSplitterWindow {
    pub fn from(ptr: *mut c_void) -> wxThinSplitterWindow { wxThinSplitterWindow { ptr: ptr } }
    pub fn null() -> wxThinSplitterWindow { wxThinSplitterWindow::from(0 as *mut c_void) }
    
}

pub trait _wxThinSplitterWindow : _wxSplitterWindow {
}

pub struct wxTimerBase { ptr: *mut c_void }
impl _wxTimerBase for wxTimerBase {}
impl _wxObject for wxTimerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTimerBase {
    pub fn from(ptr: *mut c_void) -> wxTimerBase { wxTimerBase { ptr: ptr } }
    pub fn null() -> wxTimerBase { wxTimerBase::from(0 as *mut c_void) }
    
}

pub trait _wxTimerBase : _wxObject {
}

pub struct wxToolLayoutItem { ptr: *mut c_void }
impl _wxToolLayoutItem for wxToolLayoutItem {}
impl _wxObject for wxToolLayoutItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxToolLayoutItem {
    pub fn from(ptr: *mut c_void) -> wxToolLayoutItem { wxToolLayoutItem { ptr: ptr } }
    pub fn null() -> wxToolLayoutItem { wxToolLayoutItem::from(0 as *mut c_void) }
    
}

pub trait _wxToolLayoutItem : _wxObject {
}

pub struct wxToolWindow { ptr: *mut c_void }
impl _wxToolWindow for wxToolWindow {}
impl _wxFrame for wxToolWindow {}
impl _wxTopLevelWindow for wxToolWindow {}
impl _wxWindow for wxToolWindow {}
impl _wxEvtHandler for wxToolWindow {}
impl _wxObject for wxToolWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxToolWindow {
    pub fn from(ptr: *mut c_void) -> wxToolWindow { wxToolWindow { ptr: ptr } }
    pub fn null() -> wxToolWindow { wxToolWindow::from(0 as *mut c_void) }
    
}

pub trait _wxToolWindow : _wxFrame {
}

pub struct wxTreeCompanionWindow { ptr: *mut c_void }
impl _wxTreeCompanionWindow for wxTreeCompanionWindow {}
impl _wxWindow for wxTreeCompanionWindow {}
impl _wxEvtHandler for wxTreeCompanionWindow {}
impl _wxObject for wxTreeCompanionWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeCompanionWindow {
    pub fn from(ptr: *mut c_void) -> wxTreeCompanionWindow { wxTreeCompanionWindow { ptr: ptr } }
    pub fn null() -> wxTreeCompanionWindow { wxTreeCompanionWindow::from(0 as *mut c_void) }
    
}

pub trait _wxTreeCompanionWindow : _wxWindow {
}

pub struct wxTreeLayout { ptr: *mut c_void }
impl _wxTreeLayout for wxTreeLayout {}
impl _wxObject for wxTreeLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeLayout {
    pub fn from(ptr: *mut c_void) -> wxTreeLayout { wxTreeLayout { ptr: ptr } }
    pub fn null() -> wxTreeLayout { wxTreeLayout::from(0 as *mut c_void) }
    
}

pub trait _wxTreeLayout : _wxObject {
}

pub struct wxTreeLayoutStored { ptr: *mut c_void }
impl _wxTreeLayoutStored for wxTreeLayoutStored {}
impl _wxTreeLayout for wxTreeLayoutStored {}
impl _wxObject for wxTreeLayoutStored { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeLayoutStored {
    pub fn from(ptr: *mut c_void) -> wxTreeLayoutStored { wxTreeLayoutStored { ptr: ptr } }
    pub fn null() -> wxTreeLayoutStored { wxTreeLayoutStored::from(0 as *mut c_void) }
    
}

pub trait _wxTreeLayoutStored : _wxTreeLayout {
}

pub struct wxGauge95 { ptr: *mut c_void }
impl _wxGauge95 for wxGauge95 {}
impl _wxGauge for wxGauge95 {}
impl _wxControl for wxGauge95 {}
impl _wxWindow for wxGauge95 {}
impl _wxEvtHandler for wxGauge95 {}
impl _wxObject for wxGauge95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGauge95 {
    pub fn from(ptr: *mut c_void) -> wxGauge95 { wxGauge95 { ptr: ptr } }
    pub fn null() -> wxGauge95 { wxGauge95::from(0 as *mut c_void) }
    
}

pub trait _wxGauge95 : _wxGauge {
}

pub struct wxGaugeMSW { ptr: *mut c_void }
impl _wxGaugeMSW for wxGaugeMSW {}
impl _wxGauge for wxGaugeMSW {}
impl _wxControl for wxGaugeMSW {}
impl _wxWindow for wxGaugeMSW {}
impl _wxEvtHandler for wxGaugeMSW {}
impl _wxObject for wxGaugeMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGaugeMSW {
    pub fn from(ptr: *mut c_void) -> wxGaugeMSW { wxGaugeMSW { ptr: ptr } }
    pub fn null() -> wxGaugeMSW { wxGaugeMSW::from(0 as *mut c_void) }
    
}

pub trait _wxGaugeMSW : _wxGauge {
}

pub struct wxSlider95 { ptr: *mut c_void }
impl _wxSlider95 for wxSlider95 {}
impl _wxSlider for wxSlider95 {}
impl _wxControl for wxSlider95 {}
impl _wxWindow for wxSlider95 {}
impl _wxEvtHandler for wxSlider95 {}
impl _wxObject for wxSlider95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSlider95 {
    pub fn from(ptr: *mut c_void) -> wxSlider95 { wxSlider95 { ptr: ptr } }
    pub fn null() -> wxSlider95 { wxSlider95::from(0 as *mut c_void) }
    
}

pub trait _wxSlider95 : _wxSlider {
}

pub struct wxSliderMSW { ptr: *mut c_void }
impl _wxSliderMSW for wxSliderMSW {}
impl _wxSlider for wxSliderMSW {}
impl _wxControl for wxSliderMSW {}
impl _wxWindow for wxSliderMSW {}
impl _wxEvtHandler for wxSliderMSW {}
impl _wxObject for wxSliderMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSliderMSW {
    pub fn from(ptr: *mut c_void) -> wxSliderMSW { wxSliderMSW { ptr: ptr } }
    pub fn null() -> wxSliderMSW { wxSliderMSW::from(0 as *mut c_void) }
    
}

pub trait _wxSliderMSW : _wxSlider {
}

