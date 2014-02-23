use std::libc::*;
use base::*;
use core::*;

pub struct RustMessageParameters { ptr: *mut c_void }
impl TRustMessageParameters for RustMessageParameters { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustMessageParameters {
    pub fn from(ptr: *mut c_void) -> RustMessageParameters { RustMessageParameters { ptr: ptr } }
    pub fn null() -> RustMessageParameters { RustMessageParameters::from(0 as *mut c_void) }
    
}

pub trait TRustMessageParameters {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct RustPlotCurve { ptr: *mut c_void }
impl TRustPlotCurve for RustPlotCurve {}
impl TPlotCurve for RustPlotCurve {}
impl TObject for RustPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPlotCurve {
    pub fn from(ptr: *mut c_void) -> RustPlotCurve { RustPlotCurve { ptr: ptr } }
    pub fn null() -> RustPlotCurve { RustPlotCurve::from(0 as *mut c_void) }
    
}

pub trait TRustPlotCurve : TPlotCurve {
}

pub struct DynToolInfo { ptr: *mut c_void }
impl TDynToolInfo for DynToolInfo {}
impl TToolLayoutItem for DynToolInfo {}
impl TObject for DynToolInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynToolInfo {
    pub fn from(ptr: *mut c_void) -> DynToolInfo { DynToolInfo { ptr: ptr } }
    pub fn null() -> DynToolInfo { DynToolInfo::from(0 as *mut c_void) }
    
}

pub trait TDynToolInfo : TToolLayoutItem {
}

pub struct DynamicSashWindow { ptr: *mut c_void }
impl TDynamicSashWindow for DynamicSashWindow {}
impl TWindow for DynamicSashWindow {}
impl TEvtHandler for DynamicSashWindow {}
impl TObject for DynamicSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicSashWindow {
    pub fn from(ptr: *mut c_void) -> DynamicSashWindow { DynamicSashWindow { ptr: ptr } }
    pub fn null() -> DynamicSashWindow { DynamicSashWindow::from(0 as *mut c_void) }
    
}

pub trait TDynamicSashWindow : TWindow {
}

pub struct DynamicToolBar { ptr: *mut c_void }
impl TDynamicToolBar for DynamicToolBar {}
impl TToolBarBase for DynamicToolBar {}
impl TControl for DynamicToolBar {}
impl TWindow for DynamicToolBar {}
impl TEvtHandler for DynamicToolBar {}
impl TObject for DynamicToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicToolBar {
    pub fn from(ptr: *mut c_void) -> DynamicToolBar { DynamicToolBar { ptr: ptr } }
    pub fn null() -> DynamicToolBar { DynamicToolBar::from(0 as *mut c_void) }
    
}

pub trait TDynamicToolBar : TToolBarBase {
}

pub struct Expr { ptr: *mut c_void }
impl TExpr for Expr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Expr {
    pub fn from(ptr: *mut c_void) -> Expr { Expr { ptr: ptr } }
    pub fn null() -> Expr { Expr::from(0 as *mut c_void) }
    
}

pub trait TExpr {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct ExprDatabase { ptr: *mut c_void }
impl TExprDatabase for ExprDatabase {}
impl TList for ExprDatabase {}
impl TObject for ExprDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ExprDatabase {
    pub fn from(ptr: *mut c_void) -> ExprDatabase { ExprDatabase { ptr: ptr } }
    pub fn null() -> ExprDatabase { ExprDatabase::from(0 as *mut c_void) }
    
}

pub trait TExprDatabase : TList {
}

pub struct FrameLayout { ptr: *mut c_void }
impl TFrameLayout for FrameLayout {}
impl TEvtHandler for FrameLayout {}
impl TObject for FrameLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FrameLayout {
    pub fn from(ptr: *mut c_void) -> FrameLayout { FrameLayout { ptr: ptr } }
    pub fn null() -> FrameLayout { FrameLayout::from(0 as *mut c_void) }
    
}

pub trait TFrameLayout : TEvtHandler {
}

pub struct HashMap { ptr: *mut c_void }
impl THashMap for HashMap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HashMap {
    pub fn from(ptr: *mut c_void) -> HashMap { HashMap { ptr: ptr } }
    pub fn null() -> HashMap { HashMap::from(0 as *mut c_void) }
    
}

pub trait THashMap {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct LEDNumberCtrl { ptr: *mut c_void }
impl TLEDNumberCtrl for LEDNumberCtrl {}
impl TControl for LEDNumberCtrl {}
impl TWindow for LEDNumberCtrl {}
impl TEvtHandler for LEDNumberCtrl {}
impl TObject for LEDNumberCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LEDNumberCtrl {
    pub fn from(ptr: *mut c_void) -> LEDNumberCtrl { LEDNumberCtrl { ptr: ptr } }
    pub fn null() -> LEDNumberCtrl { LEDNumberCtrl::from(0 as *mut c_void) }
    
}

pub trait TLEDNumberCtrl : TControl {
}

pub struct MBConvFile { ptr: *mut c_void }
impl TMBConvFile for MBConvFile {}
impl TMBConv for MBConvFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvFile {
    pub fn from(ptr: *mut c_void) -> MBConvFile { MBConvFile { ptr: ptr } }
    pub fn null() -> MBConvFile { MBConvFile::from(0 as *mut c_void) }
    
}

pub trait TMBConvFile : TMBConv {
}

pub struct MultiCellCanvas { ptr: *mut c_void }
impl TMultiCellCanvas for MultiCellCanvas {}
impl TFlexGridSizer for MultiCellCanvas {}
impl TGridSizer for MultiCellCanvas {}
impl TSizer for MultiCellCanvas {}
impl TObject for MultiCellCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellCanvas {
    pub fn from(ptr: *mut c_void) -> MultiCellCanvas { MultiCellCanvas { ptr: ptr } }
    pub fn null() -> MultiCellCanvas { MultiCellCanvas::from(0 as *mut c_void) }
    
}

pub trait TMultiCellCanvas : TFlexGridSizer {
}

pub struct MultiCellItemHandle { ptr: *mut c_void }
impl TMultiCellItemHandle for MultiCellItemHandle {}
impl TObject for MultiCellItemHandle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellItemHandle {
    pub fn from(ptr: *mut c_void) -> MultiCellItemHandle { MultiCellItemHandle { ptr: ptr } }
    pub fn null() -> MultiCellItemHandle { MultiCellItemHandle::from(0 as *mut c_void) }
    
}

pub trait TMultiCellItemHandle : TObject {
}

pub struct MultiCellSizer { ptr: *mut c_void }
impl TMultiCellSizer for MultiCellSizer {}
impl TSizer for MultiCellSizer {}
impl TObject for MultiCellSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellSizer {
    pub fn from(ptr: *mut c_void) -> MultiCellSizer { MultiCellSizer { ptr: ptr } }
    pub fn null() -> MultiCellSizer { MultiCellSizer::from(0 as *mut c_void) }
    
}

pub trait TMultiCellSizer : TSizer {
}

pub struct NewBitmapButton { ptr: *mut c_void }
impl TNewBitmapButton for NewBitmapButton {}
impl TPanel for NewBitmapButton {}
impl TWindow for NewBitmapButton {}
impl TEvtHandler for NewBitmapButton {}
impl TObject for NewBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NewBitmapButton {
    pub fn from(ptr: *mut c_void) -> NewBitmapButton { NewBitmapButton { ptr: ptr } }
    pub fn null() -> NewBitmapButton { NewBitmapButton::from(0 as *mut c_void) }
    
}

pub trait TNewBitmapButton : TPanel {
}

pub struct PlotCurve { ptr: *mut c_void }
impl TPlotCurve for PlotCurve {}
impl TObject for PlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotCurve {
    pub fn from(ptr: *mut c_void) -> PlotCurve { PlotCurve { ptr: ptr } }
    pub fn null() -> PlotCurve { PlotCurve::from(0 as *mut c_void) }
    
}

pub trait TPlotCurve : TObject {
}

pub struct PlotEvent { ptr: *mut c_void }
impl TPlotEvent for PlotEvent {}
impl TNotifyEvent for PlotEvent {}
impl TCommandEvent for PlotEvent {}
impl TEvent for PlotEvent {}
impl TObject for PlotEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotEvent {
    pub fn from(ptr: *mut c_void) -> PlotEvent { PlotEvent { ptr: ptr } }
    pub fn null() -> PlotEvent { PlotEvent::from(0 as *mut c_void) }
    
}

pub trait TPlotEvent : TNotifyEvent {
}

pub struct PlotOnOffCurve { ptr: *mut c_void }
impl TPlotOnOffCurve for PlotOnOffCurve {}
impl TObject for PlotOnOffCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotOnOffCurve {
    pub fn from(ptr: *mut c_void) -> PlotOnOffCurve { PlotOnOffCurve { ptr: ptr } }
    pub fn null() -> PlotOnOffCurve { PlotOnOffCurve::from(0 as *mut c_void) }
    
}

pub trait TPlotOnOffCurve : TObject {
}

pub struct PlotWindow { ptr: *mut c_void }
impl TPlotWindow for PlotWindow {}
impl TScrolledWindow for PlotWindow {}
impl TPanel for PlotWindow {}
impl TWindow for PlotWindow {}
impl TEvtHandler for PlotWindow {}
impl TObject for PlotWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotWindow {
    pub fn from(ptr: *mut c_void) -> PlotWindow { PlotWindow { ptr: ptr } }
    pub fn null() -> PlotWindow { PlotWindow::from(0 as *mut c_void) }
    
}

pub trait TPlotWindow : TScrolledWindow {
}

pub struct RemotelyScrolledTreeCtrl { ptr: *mut c_void }
impl TRemotelyScrolledTreeCtrl for RemotelyScrolledTreeCtrl {}
impl TTreeCtrl for RemotelyScrolledTreeCtrl {}
impl TControl for RemotelyScrolledTreeCtrl {}
impl TWindow for RemotelyScrolledTreeCtrl {}
impl TEvtHandler for RemotelyScrolledTreeCtrl {}
impl TObject for RemotelyScrolledTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RemotelyScrolledTreeCtrl {
    pub fn from(ptr: *mut c_void) -> RemotelyScrolledTreeCtrl { RemotelyScrolledTreeCtrl { ptr: ptr } }
    pub fn null() -> RemotelyScrolledTreeCtrl { RemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

pub trait TRemotelyScrolledTreeCtrl : TTreeCtrl {
}

pub struct SplitterScrolledWindow { ptr: *mut c_void }
impl TSplitterScrolledWindow for SplitterScrolledWindow {}
impl TScrolledWindow for SplitterScrolledWindow {}
impl TPanel for SplitterScrolledWindow {}
impl TWindow for SplitterScrolledWindow {}
impl TEvtHandler for SplitterScrolledWindow {}
impl TObject for SplitterScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterScrolledWindow {
    pub fn from(ptr: *mut c_void) -> SplitterScrolledWindow { SplitterScrolledWindow { ptr: ptr } }
    pub fn null() -> SplitterScrolledWindow { SplitterScrolledWindow::from(0 as *mut c_void) }
    
}

pub trait TSplitterScrolledWindow : TScrolledWindow {
}

pub struct StreamToTextRedirector { ptr: *mut c_void }
impl TStreamToTextRedirector for StreamToTextRedirector { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamToTextRedirector {
    pub fn from(ptr: *mut c_void) -> StreamToTextRedirector { StreamToTextRedirector { ptr: ptr } }
    pub fn null() -> StreamToTextRedirector { StreamToTextRedirector::from(0 as *mut c_void) }
    
}

pub trait TStreamToTextRedirector {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct TabCtrl { ptr: *mut c_void }
impl TTabCtrl for TabCtrl {}
impl TControl for TabCtrl {}
impl TWindow for TabCtrl {}
impl TEvtHandler for TabCtrl {}
impl TObject for TabCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TabCtrl {
    pub fn from(ptr: *mut c_void) -> TabCtrl { TabCtrl { ptr: ptr } }
    pub fn null() -> TabCtrl { TabCtrl::from(0 as *mut c_void) }
    
}

pub trait TTabCtrl : TControl {
}

pub struct TabEvent { ptr: *mut c_void }
impl TTabEvent for TabEvent {}
impl TCommandEvent for TabEvent {}
impl TEvent for TabEvent {}
impl TObject for TabEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TabEvent {
    pub fn from(ptr: *mut c_void) -> TabEvent { TabEvent { ptr: ptr } }
    pub fn null() -> TabEvent { TabEvent::from(0 as *mut c_void) }
    
}

pub trait TTabEvent : TCommandEvent {
}

pub struct ThinSplitterWindow { ptr: *mut c_void }
impl TThinSplitterWindow for ThinSplitterWindow {}
impl TSplitterWindow for ThinSplitterWindow {}
impl TWindow for ThinSplitterWindow {}
impl TEvtHandler for ThinSplitterWindow {}
impl TObject for ThinSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ThinSplitterWindow {
    pub fn from(ptr: *mut c_void) -> ThinSplitterWindow { ThinSplitterWindow { ptr: ptr } }
    pub fn null() -> ThinSplitterWindow { ThinSplitterWindow::from(0 as *mut c_void) }
    
}

pub trait TThinSplitterWindow : TSplitterWindow {
}

pub struct TimerBase { ptr: *mut c_void }
impl TTimerBase for TimerBase {}
impl TObject for TimerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerBase {
    pub fn from(ptr: *mut c_void) -> TimerBase { TimerBase { ptr: ptr } }
    pub fn null() -> TimerBase { TimerBase::from(0 as *mut c_void) }
    
}

pub trait TTimerBase : TObject {
}

pub struct ToolLayoutItem { ptr: *mut c_void }
impl TToolLayoutItem for ToolLayoutItem {}
impl TObject for ToolLayoutItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolLayoutItem {
    pub fn from(ptr: *mut c_void) -> ToolLayoutItem { ToolLayoutItem { ptr: ptr } }
    pub fn null() -> ToolLayoutItem { ToolLayoutItem::from(0 as *mut c_void) }
    
}

pub trait TToolLayoutItem : TObject {
}

pub struct ToolWindow { ptr: *mut c_void }
impl TToolWindow for ToolWindow {}
impl TFrame for ToolWindow {}
impl TTopLevelWindow for ToolWindow {}
impl TWindow for ToolWindow {}
impl TEvtHandler for ToolWindow {}
impl TObject for ToolWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolWindow {
    pub fn from(ptr: *mut c_void) -> ToolWindow { ToolWindow { ptr: ptr } }
    pub fn null() -> ToolWindow { ToolWindow::from(0 as *mut c_void) }
    
}

pub trait TToolWindow : TFrame {
}

pub struct TreeCompanionWindow { ptr: *mut c_void }
impl TTreeCompanionWindow for TreeCompanionWindow {}
impl TWindow for TreeCompanionWindow {}
impl TEvtHandler for TreeCompanionWindow {}
impl TObject for TreeCompanionWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeCompanionWindow {
    pub fn from(ptr: *mut c_void) -> TreeCompanionWindow { TreeCompanionWindow { ptr: ptr } }
    pub fn null() -> TreeCompanionWindow { TreeCompanionWindow::from(0 as *mut c_void) }
    
}

pub trait TTreeCompanionWindow : TWindow {
}

pub struct TreeLayout { ptr: *mut c_void }
impl TTreeLayout for TreeLayout {}
impl TObject for TreeLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeLayout {
    pub fn from(ptr: *mut c_void) -> TreeLayout { TreeLayout { ptr: ptr } }
    pub fn null() -> TreeLayout { TreeLayout::from(0 as *mut c_void) }
    
}

pub trait TTreeLayout : TObject {
}

pub struct TreeLayoutStored { ptr: *mut c_void }
impl TTreeLayoutStored for TreeLayoutStored {}
impl TTreeLayout for TreeLayoutStored {}
impl TObject for TreeLayoutStored { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeLayoutStored {
    pub fn from(ptr: *mut c_void) -> TreeLayoutStored { TreeLayoutStored { ptr: ptr } }
    pub fn null() -> TreeLayoutStored { TreeLayoutStored::from(0 as *mut c_void) }
    
}

pub trait TTreeLayoutStored : TTreeLayout {
}

pub struct Gauge95 { ptr: *mut c_void }
impl TGauge95 for Gauge95 {}
impl TGauge for Gauge95 {}
impl TControl for Gauge95 {}
impl TWindow for Gauge95 {}
impl TEvtHandler for Gauge95 {}
impl TObject for Gauge95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Gauge95 {
    pub fn from(ptr: *mut c_void) -> Gauge95 { Gauge95 { ptr: ptr } }
    pub fn null() -> Gauge95 { Gauge95::from(0 as *mut c_void) }
    
}

pub trait TGauge95 : TGauge {
}

pub struct GaugeMSW { ptr: *mut c_void }
impl TGaugeMSW for GaugeMSW {}
impl TGauge for GaugeMSW {}
impl TControl for GaugeMSW {}
impl TWindow for GaugeMSW {}
impl TEvtHandler for GaugeMSW {}
impl TObject for GaugeMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GaugeMSW {
    pub fn from(ptr: *mut c_void) -> GaugeMSW { GaugeMSW { ptr: ptr } }
    pub fn null() -> GaugeMSW { GaugeMSW::from(0 as *mut c_void) }
    
}

pub trait TGaugeMSW : TGauge {
}

pub struct Slider95 { ptr: *mut c_void }
impl TSlider95 for Slider95 {}
impl TSlider for Slider95 {}
impl TControl for Slider95 {}
impl TWindow for Slider95 {}
impl TEvtHandler for Slider95 {}
impl TObject for Slider95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Slider95 {
    pub fn from(ptr: *mut c_void) -> Slider95 { Slider95 { ptr: ptr } }
    pub fn null() -> Slider95 { Slider95::from(0 as *mut c_void) }
    
}

pub trait TSlider95 : TSlider {
}

pub struct SliderMSW { ptr: *mut c_void }
impl TSliderMSW for SliderMSW {}
impl TSlider for SliderMSW {}
impl TControl for SliderMSW {}
impl TWindow for SliderMSW {}
impl TEvtHandler for SliderMSW {}
impl TObject for SliderMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SliderMSW {
    pub fn from(ptr: *mut c_void) -> SliderMSW { SliderMSW { ptr: ptr } }
    pub fn null() -> SliderMSW { SliderMSW::from(0 as *mut c_void) }
    
}

pub trait TSliderMSW : TSlider {
}

