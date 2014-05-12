use libc::*;
use base::*;
use core::*;

pub struct RustMessageParameters { ptr: *mut c_void }
impl RustMessageParametersMethods for RustMessageParameters { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustMessageParameters {
    pub fn from(ptr: *mut c_void) -> RustMessageParameters { RustMessageParameters { ptr: ptr } }
    pub fn null() -> RustMessageParameters { RustMessageParameters::from(0 as *mut c_void) }
    
}

pub trait RustMessageParametersMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// The wxRust-specific derived class of [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html).
pub struct RustPlotCurve { ptr: *mut c_void }
impl RustPlotCurveMethods for RustPlotCurve {}
impl PlotCurveMethods for RustPlotCurve {}
impl ObjectMethods for RustPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPlotCurve {
    pub fn from(ptr: *mut c_void) -> RustPlotCurve { RustPlotCurve { ptr: ptr } }
    pub fn null() -> RustPlotCurve { RustPlotCurve::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html).
pub trait RustPlotCurveMethods : PlotCurveMethods {
}

/// Wraps the wxWidgets' [wxDynToolInfo](http://docs.wxwidgets.org/3.0/classwx_dyn_tool_info.html) class.
pub struct DynToolInfo { ptr: *mut c_void }
impl DynToolInfoMethods for DynToolInfo {}
impl ToolLayoutItemMethods for DynToolInfo {}
impl ObjectMethods for DynToolInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynToolInfo {
    pub fn from(ptr: *mut c_void) -> DynToolInfo { DynToolInfo { ptr: ptr } }
    pub fn null() -> DynToolInfo { DynToolInfo::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynToolInfo](http://docs.wxwidgets.org/3.0/classwx_dyn_tool_info.html) class.
pub trait DynToolInfoMethods : ToolLayoutItemMethods {
}

/// Wraps the wxWidgets' [wxDynamicSashWindow](http://docs.wxwidgets.org/3.0/classwx_dynamic_sash_window.html) class.
pub struct DynamicSashWindow { ptr: *mut c_void }
impl DynamicSashWindowMethods for DynamicSashWindow {}
impl WindowMethods for DynamicSashWindow {}
impl EvtHandlerMethods for DynamicSashWindow {}
impl ObjectMethods for DynamicSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicSashWindow {
    pub fn from(ptr: *mut c_void) -> DynamicSashWindow { DynamicSashWindow { ptr: ptr } }
    pub fn null() -> DynamicSashWindow { DynamicSashWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynamicSashWindow](http://docs.wxwidgets.org/3.0/classwx_dynamic_sash_window.html) class.
pub trait DynamicSashWindowMethods : WindowMethods {
}

/// Wraps the wxWidgets' [wxDynamicToolBar](http://docs.wxwidgets.org/3.0/classwx_dynamic_tool_bar.html) class.
pub struct DynamicToolBar { ptr: *mut c_void }
impl DynamicToolBarMethods for DynamicToolBar {}
impl ToolBarBaseMethods for DynamicToolBar {}
impl ControlMethods for DynamicToolBar {}
impl WindowMethods for DynamicToolBar {}
impl EvtHandlerMethods for DynamicToolBar {}
impl ObjectMethods for DynamicToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicToolBar {
    pub fn from(ptr: *mut c_void) -> DynamicToolBar { DynamicToolBar { ptr: ptr } }
    pub fn null() -> DynamicToolBar { DynamicToolBar::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynamicToolBar](http://docs.wxwidgets.org/3.0/classwx_dynamic_tool_bar.html) class.
pub trait DynamicToolBarMethods : ToolBarBaseMethods {
}

/// Wraps the wxWidgets' [wxExpr](http://docs.wxwidgets.org/3.0/classwx_expr.html) class.
pub struct Expr { ptr: *mut c_void }
impl ExprMethods for Expr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Expr {
    pub fn from(ptr: *mut c_void) -> Expr { Expr { ptr: ptr } }
    pub fn null() -> Expr { Expr::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxExpr](http://docs.wxwidgets.org/3.0/classwx_expr.html) class.
pub trait ExprMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxExprDatabase](http://docs.wxwidgets.org/3.0/classwx_expr_database.html) class.
pub struct ExprDatabase { ptr: *mut c_void }
impl ExprDatabaseMethods for ExprDatabase {}
impl ListMethods for ExprDatabase {}
impl ObjectMethods for ExprDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ExprDatabase {
    pub fn from(ptr: *mut c_void) -> ExprDatabase { ExprDatabase { ptr: ptr } }
    pub fn null() -> ExprDatabase { ExprDatabase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxExprDatabase](http://docs.wxwidgets.org/3.0/classwx_expr_database.html) class.
pub trait ExprDatabaseMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxFrameLayout](http://docs.wxwidgets.org/3.0/classwx_frame_layout.html) class.
pub struct FrameLayout { ptr: *mut c_void }
impl FrameLayoutMethods for FrameLayout {}
impl EvtHandlerMethods for FrameLayout {}
impl ObjectMethods for FrameLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FrameLayout {
    pub fn from(ptr: *mut c_void) -> FrameLayout { FrameLayout { ptr: ptr } }
    pub fn null() -> FrameLayout { FrameLayout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFrameLayout](http://docs.wxwidgets.org/3.0/classwx_frame_layout.html) class.
pub trait FrameLayoutMethods : EvtHandlerMethods {
}

/// Wraps the wxWidgets' [wxHashMap](http://docs.wxwidgets.org/3.0/classwx_hash_map.html) class.
pub struct HashMap { ptr: *mut c_void }
impl HashMapMethods for HashMap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HashMap {
    pub fn from(ptr: *mut c_void) -> HashMap { HashMap { ptr: ptr } }
    pub fn null() -> HashMap { HashMap::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHashMap](http://docs.wxwidgets.org/3.0/classwx_hash_map.html) class.
pub trait HashMapMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxLEDNumberCtrl](http://docs.wxwidgets.org/3.0/classwx_ledn_umber_ctrl.html) class.
pub struct LEDNumberCtrl { ptr: *mut c_void }
impl LEDNumberCtrlMethods for LEDNumberCtrl {}
impl ControlMethods for LEDNumberCtrl {}
impl WindowMethods for LEDNumberCtrl {}
impl EvtHandlerMethods for LEDNumberCtrl {}
impl ObjectMethods for LEDNumberCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LEDNumberCtrl {
    pub fn from(ptr: *mut c_void) -> LEDNumberCtrl { LEDNumberCtrl { ptr: ptr } }
    pub fn null() -> LEDNumberCtrl { LEDNumberCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxLEDNumberCtrl](http://docs.wxwidgets.org/3.0/classwx_ledn_umber_ctrl.html) class.
pub trait LEDNumberCtrlMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxMBConvFile](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_file.html) class.
pub struct MBConvFile { ptr: *mut c_void }
impl MBConvFileMethods for MBConvFile {}
impl MBConvMethods for MBConvFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvFile {
    pub fn from(ptr: *mut c_void) -> MBConvFile { MBConvFile { ptr: ptr } }
    pub fn null() -> MBConvFile { MBConvFile::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMBConvFile](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_file.html) class.
pub trait MBConvFileMethods : MBConvMethods {
}

/// Wraps the wxWidgets' [wxMultiCellCanvas](http://docs.wxwidgets.org/3.0/classwx_multi_cell_canvas.html) class.
pub struct MultiCellCanvas { ptr: *mut c_void }
impl MultiCellCanvasMethods for MultiCellCanvas {}
impl FlexGridSizerMethods for MultiCellCanvas {}
impl GridSizerMethods for MultiCellCanvas {}
impl SizerMethods for MultiCellCanvas {}
impl ObjectMethods for MultiCellCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellCanvas {
    pub fn from(ptr: *mut c_void) -> MultiCellCanvas { MultiCellCanvas { ptr: ptr } }
    pub fn null() -> MultiCellCanvas { MultiCellCanvas::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMultiCellCanvas](http://docs.wxwidgets.org/3.0/classwx_multi_cell_canvas.html) class.
pub trait MultiCellCanvasMethods : FlexGridSizerMethods {
}

/// Wraps the wxWidgets' [wxMultiCellItemHandle](http://docs.wxwidgets.org/3.0/classwx_multi_cell_item_handle.html) class.
pub struct MultiCellItemHandle { ptr: *mut c_void }
impl MultiCellItemHandleMethods for MultiCellItemHandle {}
impl ObjectMethods for MultiCellItemHandle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellItemHandle {
    pub fn from(ptr: *mut c_void) -> MultiCellItemHandle { MultiCellItemHandle { ptr: ptr } }
    pub fn null() -> MultiCellItemHandle { MultiCellItemHandle::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMultiCellItemHandle](http://docs.wxwidgets.org/3.0/classwx_multi_cell_item_handle.html) class.
pub trait MultiCellItemHandleMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxMultiCellSizer](http://docs.wxwidgets.org/3.0/classwx_multi_cell_sizer.html) class.
pub struct MultiCellSizer { ptr: *mut c_void }
impl MultiCellSizerMethods for MultiCellSizer {}
impl SizerMethods for MultiCellSizer {}
impl ObjectMethods for MultiCellSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellSizer {
    pub fn from(ptr: *mut c_void) -> MultiCellSizer { MultiCellSizer { ptr: ptr } }
    pub fn null() -> MultiCellSizer { MultiCellSizer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMultiCellSizer](http://docs.wxwidgets.org/3.0/classwx_multi_cell_sizer.html) class.
pub trait MultiCellSizerMethods : SizerMethods {
}

/// Wraps the wxWidgets' [wxNewBitmapButton](http://docs.wxwidgets.org/3.0/classwx_new_bitmap_button.html) class.
pub struct NewBitmapButton { ptr: *mut c_void }
impl NewBitmapButtonMethods for NewBitmapButton {}
impl PanelMethods for NewBitmapButton {}
impl WindowMethods for NewBitmapButton {}
impl EvtHandlerMethods for NewBitmapButton {}
impl ObjectMethods for NewBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NewBitmapButton {
    pub fn from(ptr: *mut c_void) -> NewBitmapButton { NewBitmapButton { ptr: ptr } }
    pub fn null() -> NewBitmapButton { NewBitmapButton::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxNewBitmapButton](http://docs.wxwidgets.org/3.0/classwx_new_bitmap_button.html) class.
pub trait NewBitmapButtonMethods : PanelMethods {
}

/// Wraps the wxWidgets' [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html) class.
/// Rather use the wxRust-specific [RustPlotCurve](struct.RustPlotCurve.html) class.
pub struct PlotCurve { ptr: *mut c_void }
impl PlotCurveMethods for PlotCurve {}
impl ObjectMethods for PlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotCurve {
    pub fn from(ptr: *mut c_void) -> PlotCurve { PlotCurve { ptr: ptr } }
    pub fn null() -> PlotCurve { PlotCurve::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html) class.
pub trait PlotCurveMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxPlotEvent](http://docs.wxwidgets.org/3.0/classwx_plot_event.html) class.
pub struct PlotEvent { ptr: *mut c_void }
impl PlotEventMethods for PlotEvent {}
impl NotifyEventMethods for PlotEvent {}
impl CommandEventMethods for PlotEvent {}
impl EventMethods for PlotEvent {}
impl ObjectMethods for PlotEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotEvent {
    pub fn from(ptr: *mut c_void) -> PlotEvent { PlotEvent { ptr: ptr } }
    pub fn null() -> PlotEvent { PlotEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotEvent](http://docs.wxwidgets.org/3.0/classwx_plot_event.html) class.
pub trait PlotEventMethods : NotifyEventMethods {
}

/// Wraps the wxWidgets' [wxPlotOnOffCurve](http://docs.wxwidgets.org/3.0/classwx_plot_on_off_curve.html) class.
pub struct PlotOnOffCurve { ptr: *mut c_void }
impl PlotOnOffCurveMethods for PlotOnOffCurve {}
impl ObjectMethods for PlotOnOffCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotOnOffCurve {
    pub fn from(ptr: *mut c_void) -> PlotOnOffCurve { PlotOnOffCurve { ptr: ptr } }
    pub fn null() -> PlotOnOffCurve { PlotOnOffCurve::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotOnOffCurve](http://docs.wxwidgets.org/3.0/classwx_plot_on_off_curve.html) class.
pub trait PlotOnOffCurveMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxPlotWindow](http://docs.wxwidgets.org/3.0/classwx_plot_window.html) class.
pub struct PlotWindow { ptr: *mut c_void }
impl PlotWindowMethods for PlotWindow {}
impl ScrolledWindowMethods for PlotWindow {}
impl PanelMethods for PlotWindow {}
impl WindowMethods for PlotWindow {}
impl EvtHandlerMethods for PlotWindow {}
impl ObjectMethods for PlotWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotWindow {
    pub fn from(ptr: *mut c_void) -> PlotWindow { PlotWindow { ptr: ptr } }
    pub fn null() -> PlotWindow { PlotWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotWindow](http://docs.wxwidgets.org/3.0/classwx_plot_window.html) class.
pub trait PlotWindowMethods : ScrolledWindowMethods {
}

/// Wraps the wxWidgets' [wxRemotelyScrolledTreeCtrl](http://docs.wxwidgets.org/3.0/classwx_remotely_scrolled_tree_ctrl.html) class.
pub struct RemotelyScrolledTreeCtrl { ptr: *mut c_void }
impl RemotelyScrolledTreeCtrlMethods for RemotelyScrolledTreeCtrl {}
impl TreeCtrlMethods for RemotelyScrolledTreeCtrl {}
impl ControlMethods for RemotelyScrolledTreeCtrl {}
impl WindowMethods for RemotelyScrolledTreeCtrl {}
impl EvtHandlerMethods for RemotelyScrolledTreeCtrl {}
impl ObjectMethods for RemotelyScrolledTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RemotelyScrolledTreeCtrl {
    pub fn from(ptr: *mut c_void) -> RemotelyScrolledTreeCtrl { RemotelyScrolledTreeCtrl { ptr: ptr } }
    pub fn null() -> RemotelyScrolledTreeCtrl { RemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxRemotelyScrolledTreeCtrl](http://docs.wxwidgets.org/3.0/classwx_remotely_scrolled_tree_ctrl.html) class.
pub trait RemotelyScrolledTreeCtrlMethods : TreeCtrlMethods {
}

/// Wraps the wxWidgets' [wxSplitterScrolledWindow](http://docs.wxwidgets.org/3.0/classwx_splitter_scrolled_window.html) class.
pub struct SplitterScrolledWindow { ptr: *mut c_void }
impl SplitterScrolledWindowMethods for SplitterScrolledWindow {}
impl ScrolledWindowMethods for SplitterScrolledWindow {}
impl PanelMethods for SplitterScrolledWindow {}
impl WindowMethods for SplitterScrolledWindow {}
impl EvtHandlerMethods for SplitterScrolledWindow {}
impl ObjectMethods for SplitterScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterScrolledWindow {
    pub fn from(ptr: *mut c_void) -> SplitterScrolledWindow { SplitterScrolledWindow { ptr: ptr } }
    pub fn null() -> SplitterScrolledWindow { SplitterScrolledWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSplitterScrolledWindow](http://docs.wxwidgets.org/3.0/classwx_splitter_scrolled_window.html) class.
pub trait SplitterScrolledWindowMethods : ScrolledWindowMethods {
}

/// Wraps the wxWidgets' [wxStreamToTextRedirector](http://docs.wxwidgets.org/3.0/classwx_stream_to_text_redirector.html) class.
pub struct StreamToTextRedirector { ptr: *mut c_void }
impl StreamToTextRedirectorMethods for StreamToTextRedirector { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamToTextRedirector {
    pub fn from(ptr: *mut c_void) -> StreamToTextRedirector { StreamToTextRedirector { ptr: ptr } }
    pub fn null() -> StreamToTextRedirector { StreamToTextRedirector::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStreamToTextRedirector](http://docs.wxwidgets.org/3.0/classwx_stream_to_text_redirector.html) class.
pub trait StreamToTextRedirectorMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxTabCtrl](http://docs.wxwidgets.org/3.0/classwx_tab_ctrl.html) class.
pub struct TabCtrl { ptr: *mut c_void }
impl TabCtrlMethods for TabCtrl {}
impl ControlMethods for TabCtrl {}
impl WindowMethods for TabCtrl {}
impl EvtHandlerMethods for TabCtrl {}
impl ObjectMethods for TabCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TabCtrl {
    pub fn from(ptr: *mut c_void) -> TabCtrl { TabCtrl { ptr: ptr } }
    pub fn null() -> TabCtrl { TabCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTabCtrl](http://docs.wxwidgets.org/3.0/classwx_tab_ctrl.html) class.
pub trait TabCtrlMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxTabEvent](http://docs.wxwidgets.org/3.0/classwx_tab_event.html) class.
pub struct TabEvent { ptr: *mut c_void }
impl TabEventMethods for TabEvent {}
impl CommandEventMethods for TabEvent {}
impl EventMethods for TabEvent {}
impl ObjectMethods for TabEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TabEvent {
    pub fn from(ptr: *mut c_void) -> TabEvent { TabEvent { ptr: ptr } }
    pub fn null() -> TabEvent { TabEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTabEvent](http://docs.wxwidgets.org/3.0/classwx_tab_event.html) class.
pub trait TabEventMethods : CommandEventMethods {
}

/// Wraps the wxWidgets' [wxThinSplitterWindow](http://docs.wxwidgets.org/3.0/classwx_thin_splitter_window.html) class.
pub struct ThinSplitterWindow { ptr: *mut c_void }
impl ThinSplitterWindowMethods for ThinSplitterWindow {}
impl SplitterWindowMethods for ThinSplitterWindow {}
impl WindowMethods for ThinSplitterWindow {}
impl EvtHandlerMethods for ThinSplitterWindow {}
impl ObjectMethods for ThinSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ThinSplitterWindow {
    pub fn from(ptr: *mut c_void) -> ThinSplitterWindow { ThinSplitterWindow { ptr: ptr } }
    pub fn null() -> ThinSplitterWindow { ThinSplitterWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxThinSplitterWindow](http://docs.wxwidgets.org/3.0/classwx_thin_splitter_window.html) class.
pub trait ThinSplitterWindowMethods : SplitterWindowMethods {
}

/// Wraps the wxWidgets' [wxTimerBase](http://docs.wxwidgets.org/3.0/classwx_timer_base.html) class.
pub struct TimerBase { ptr: *mut c_void }
impl TimerBaseMethods for TimerBase {}
impl ObjectMethods for TimerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerBase {
    pub fn from(ptr: *mut c_void) -> TimerBase { TimerBase { ptr: ptr } }
    pub fn null() -> TimerBase { TimerBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTimerBase](http://docs.wxwidgets.org/3.0/classwx_timer_base.html) class.
pub trait TimerBaseMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxToolLayoutItem](http://docs.wxwidgets.org/3.0/classwx_tool_layout_item.html) class.
pub struct ToolLayoutItem { ptr: *mut c_void }
impl ToolLayoutItemMethods for ToolLayoutItem {}
impl ObjectMethods for ToolLayoutItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolLayoutItem {
    pub fn from(ptr: *mut c_void) -> ToolLayoutItem { ToolLayoutItem { ptr: ptr } }
    pub fn null() -> ToolLayoutItem { ToolLayoutItem::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxToolLayoutItem](http://docs.wxwidgets.org/3.0/classwx_tool_layout_item.html) class.
pub trait ToolLayoutItemMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxToolWindow](http://docs.wxwidgets.org/3.0/classwx_tool_window.html) class.
pub struct ToolWindow { ptr: *mut c_void }
impl ToolWindowMethods for ToolWindow {}
impl FrameMethods for ToolWindow {}
impl TopLevelWindowMethods for ToolWindow {}
impl WindowMethods for ToolWindow {}
impl EvtHandlerMethods for ToolWindow {}
impl ObjectMethods for ToolWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolWindow {
    pub fn from(ptr: *mut c_void) -> ToolWindow { ToolWindow { ptr: ptr } }
    pub fn null() -> ToolWindow { ToolWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxToolWindow](http://docs.wxwidgets.org/3.0/classwx_tool_window.html) class.
pub trait ToolWindowMethods : FrameMethods {
}

/// Wraps the wxWidgets' [wxTreeCompanionWindow](http://docs.wxwidgets.org/3.0/classwx_tree_companion_window.html) class.
pub struct TreeCompanionWindow { ptr: *mut c_void }
impl TreeCompanionWindowMethods for TreeCompanionWindow {}
impl WindowMethods for TreeCompanionWindow {}
impl EvtHandlerMethods for TreeCompanionWindow {}
impl ObjectMethods for TreeCompanionWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeCompanionWindow {
    pub fn from(ptr: *mut c_void) -> TreeCompanionWindow { TreeCompanionWindow { ptr: ptr } }
    pub fn null() -> TreeCompanionWindow { TreeCompanionWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeCompanionWindow](http://docs.wxwidgets.org/3.0/classwx_tree_companion_window.html) class.
pub trait TreeCompanionWindowMethods : WindowMethods {
}

/// Wraps the wxWidgets' [wxTreeLayout](http://docs.wxwidgets.org/3.0/classwx_tree_layout.html) class.
pub struct TreeLayout { ptr: *mut c_void }
impl TreeLayoutMethods for TreeLayout {}
impl ObjectMethods for TreeLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeLayout {
    pub fn from(ptr: *mut c_void) -> TreeLayout { TreeLayout { ptr: ptr } }
    pub fn null() -> TreeLayout { TreeLayout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeLayout](http://docs.wxwidgets.org/3.0/classwx_tree_layout.html) class.
pub trait TreeLayoutMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxTreeLayoutStored](http://docs.wxwidgets.org/3.0/classwx_tree_layout_stored.html) class.
pub struct TreeLayoutStored { ptr: *mut c_void }
impl TreeLayoutStoredMethods for TreeLayoutStored {}
impl TreeLayoutMethods for TreeLayoutStored {}
impl ObjectMethods for TreeLayoutStored { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeLayoutStored {
    pub fn from(ptr: *mut c_void) -> TreeLayoutStored { TreeLayoutStored { ptr: ptr } }
    pub fn null() -> TreeLayoutStored { TreeLayoutStored::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeLayoutStored](http://docs.wxwidgets.org/3.0/classwx_tree_layout_stored.html) class.
pub trait TreeLayoutStoredMethods : TreeLayoutMethods {
}

/// Wraps the wxWidgets' [wxGauge95](http://docs.wxwidgets.org/3.0/classwx_gauge_95.html) class.
pub struct Gauge95 { ptr: *mut c_void }
impl Gauge95Methods for Gauge95 {}
impl GaugeMethods for Gauge95 {}
impl ControlMethods for Gauge95 {}
impl WindowMethods for Gauge95 {}
impl EvtHandlerMethods for Gauge95 {}
impl ObjectMethods for Gauge95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Gauge95 {
    pub fn from(ptr: *mut c_void) -> Gauge95 { Gauge95 { ptr: ptr } }
    pub fn null() -> Gauge95 { Gauge95::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGauge95](http://docs.wxwidgets.org/3.0/classwx_gauge_95.html) class.
pub trait Gauge95Methods : GaugeMethods {
}

/// Wraps the wxWidgets' [wxGaugeMSW](http://docs.wxwidgets.org/3.0/classwx_gauge_msw.html) class.
pub struct GaugeMSW { ptr: *mut c_void }
impl GaugeMSWMethods for GaugeMSW {}
impl GaugeMethods for GaugeMSW {}
impl ControlMethods for GaugeMSW {}
impl WindowMethods for GaugeMSW {}
impl EvtHandlerMethods for GaugeMSW {}
impl ObjectMethods for GaugeMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GaugeMSW {
    pub fn from(ptr: *mut c_void) -> GaugeMSW { GaugeMSW { ptr: ptr } }
    pub fn null() -> GaugeMSW { GaugeMSW::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGaugeMSW](http://docs.wxwidgets.org/3.0/classwx_gauge_msw.html) class.
pub trait GaugeMSWMethods : GaugeMethods {
}

/// Wraps the wxWidgets' [wxSlider95](http://docs.wxwidgets.org/3.0/classwx_slider_95.html) class.
pub struct Slider95 { ptr: *mut c_void }
impl Slider95Methods for Slider95 {}
impl SliderMethods for Slider95 {}
impl ControlMethods for Slider95 {}
impl WindowMethods for Slider95 {}
impl EvtHandlerMethods for Slider95 {}
impl ObjectMethods for Slider95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Slider95 {
    pub fn from(ptr: *mut c_void) -> Slider95 { Slider95 { ptr: ptr } }
    pub fn null() -> Slider95 { Slider95::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSlider95](http://docs.wxwidgets.org/3.0/classwx_slider_95.html) class.
pub trait Slider95Methods : SliderMethods {
}

/// Wraps the wxWidgets' [wxSliderMSW](http://docs.wxwidgets.org/3.0/classwx_slider_msw.html) class.
pub struct SliderMSW { ptr: *mut c_void }
impl SliderMSWMethods for SliderMSW {}
impl SliderMethods for SliderMSW {}
impl ControlMethods for SliderMSW {}
impl WindowMethods for SliderMSW {}
impl EvtHandlerMethods for SliderMSW {}
impl ObjectMethods for SliderMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SliderMSW {
    pub fn from(ptr: *mut c_void) -> SliderMSW { SliderMSW { ptr: ptr } }
    pub fn null() -> SliderMSW { SliderMSW::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSliderMSW](http://docs.wxwidgets.org/3.0/classwx_slider_msw.html) class.
pub trait SliderMSWMethods : SliderMethods {
}

