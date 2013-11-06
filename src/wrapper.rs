use std::libc::*;
use base::*;
use core::*;
use html::*;
use net::*;
use advanced::*;
use native::*;


pub struct ELJArtProv(*mut c_void);
impl _ELJArtProv for ELJArtProv {}
impl _wxArtProvider for ELJArtProv {}
impl _wxObject for ELJArtProv { fn handle(&self) -> *mut c_void { **self } }

impl ELJArtProv {
    pub fn from(handle: *mut c_void) -> @ELJArtProv { @ELJArtProv(handle) }
    pub fn null() -> @ELJArtProv { ELJArtProv::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _clb: *mut c_void) -> @ELJArtProv {
        unsafe { @ELJArtProv(ELJArtProv_Create(_obj, _clb)) }
    }
}

pub trait _ELJArtProv : _wxArtProvider {
    #[fixed_stack_segment]
    #[inline(never)]
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.handle()) }
    }
}

pub struct ELJClient(*mut c_void);
impl _ELJClient for ELJClient {}
impl _wxClient for ELJClient {}
impl _wxClientBase for ELJClient {}
impl _wxObject for ELJClient { fn handle(&self) -> *mut c_void { **self } }

impl ELJClient {
    pub fn from(handle: *mut c_void) -> @ELJClient { @ELJClient(handle) }
    pub fn null() -> @ELJClient { ELJClient::from(0 as *mut c_void) }
    
}

pub trait _ELJClient : _wxClient {
}

pub struct ELJCommand(*mut c_void);
impl _ELJCommand for ELJCommand {}
impl _wxCommand for ELJCommand {}
impl _wxObject for ELJCommand { fn handle(&self) -> *mut c_void { **self } }

impl ELJCommand {
    pub fn from(handle: *mut c_void) -> @ELJCommand { @ELJCommand(handle) }
    pub fn null() -> @ELJCommand { ELJCommand::from(0 as *mut c_void) }
    
}

pub trait _ELJCommand : _wxCommand {
}

pub struct ELJConnection(*mut c_void);
impl _ELJConnection for ELJConnection {}
impl _wxConnection for ELJConnection {}
impl _wxConnectionBase for ELJConnection {}
impl _wxObject for ELJConnection { fn handle(&self) -> *mut c_void { **self } }

impl ELJConnection {
    pub fn from(handle: *mut c_void) -> @ELJConnection { @ELJConnection(handle) }
    pub fn null() -> @ELJConnection { ELJConnection::from(0 as *mut c_void) }
    
}

pub trait _ELJConnection : _wxConnection {
}

pub struct ELJDragDataObject(*mut c_void);
impl _ELJDragDataObject for ELJDragDataObject { fn handle(&self) -> *mut c_void { **self } }

impl ELJDragDataObject {
    pub fn from(handle: *mut c_void) -> @ELJDragDataObject { @ELJDragDataObject(handle) }
    pub fn null() -> @ELJDragDataObject { ELJDragDataObject::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fmt: &str, _func1: *mut c_void, _func2: *mut c_void, _func3: *mut c_void) -> @ELJDragDataObject {
        let _fmt = wxT(_fmt);
        unsafe { @ELJDragDataObject(ELJDragDataObject_Create(_obj, _fmt.handle(), _func1, _func2, _func3)) }
    }
}

pub trait _ELJDragDataObject {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.handle()) }
    }
}

pub struct ELJDropTarget(*mut c_void);
impl _ELJDropTarget for ELJDropTarget {}
impl _wxDropTarget for ELJDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl ELJDropTarget {
    pub fn from(handle: *mut c_void) -> @ELJDropTarget { @ELJDropTarget(handle) }
    pub fn null() -> @ELJDropTarget { ELJDropTarget::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void) -> @ELJDropTarget {
        unsafe { @ELJDropTarget(ELJDropTarget_Create(_obj)) }
    }
}

pub trait _ELJDropTarget : _wxDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

pub struct ELJFileDropTarget(*mut c_void);
impl _ELJFileDropTarget for ELJFileDropTarget {}
impl _wxFileDropTarget for ELJFileDropTarget {}
impl _wxDropTarget for ELJFileDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl ELJFileDropTarget {
    pub fn from(handle: *mut c_void) -> @ELJFileDropTarget { @ELJFileDropTarget(handle) }
    pub fn null() -> @ELJFileDropTarget { ELJFileDropTarget::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> @ELJFileDropTarget {
        unsafe { @ELJFileDropTarget(ELJFileDropTarget_Create(_obj, _func)) }
    }
}

pub trait _ELJFileDropTarget : _wxFileDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJFileDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

pub struct ELJLocale(*mut c_void);
impl _ELJLocale for ELJLocale {}
impl _wxLocale for ELJLocale { fn handle(&self) -> *mut c_void { **self } }

impl ELJLocale {
    pub fn from(handle: *mut c_void) -> @ELJLocale { @ELJLocale(handle) }
    pub fn null() -> @ELJLocale { ELJLocale::from(0 as *mut c_void) }
    
}

pub trait _ELJLocale : _wxLocale {
}

pub struct ELJMessageParameters(*mut c_void);
impl _ELJMessageParameters for ELJMessageParameters { fn handle(&self) -> *mut c_void { **self } }

impl ELJMessageParameters {
    pub fn from(handle: *mut c_void) -> @ELJMessageParameters { @ELJMessageParameters(handle) }
    pub fn null() -> @ELJMessageParameters { ELJMessageParameters::from(0 as *mut c_void) }
    
}

pub trait _ELJMessageParameters {
    fn handle(&self) -> *mut c_void;
    
}

pub struct ELJPlotCurve(*mut c_void);
impl _ELJPlotCurve for ELJPlotCurve {}
impl _wxPlotCurve for ELJPlotCurve {}
impl _wxObject for ELJPlotCurve { fn handle(&self) -> *mut c_void { **self } }

impl ELJPlotCurve {
    pub fn from(handle: *mut c_void) -> @ELJPlotCurve { @ELJPlotCurve(handle) }
    pub fn null() -> @ELJPlotCurve { ELJPlotCurve::from(0 as *mut c_void) }
    
}

pub trait _ELJPlotCurve : _wxPlotCurve {
}

pub struct ELJPreviewControlBar(*mut c_void);
impl _ELJPreviewControlBar for ELJPreviewControlBar {}
impl _wxPreviewControlBar for ELJPreviewControlBar {}
impl _wxPanel for ELJPreviewControlBar {}
impl _wxWindow for ELJPreviewControlBar {}
impl _wxEvtHandler for ELJPreviewControlBar {}
impl _wxObject for ELJPreviewControlBar { fn handle(&self) -> *mut c_void { **self } }

impl ELJPreviewControlBar {
    pub fn from(handle: *mut c_void) -> @ELJPreviewControlBar { @ELJPreviewControlBar(handle) }
    pub fn null() -> @ELJPreviewControlBar { ELJPreviewControlBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(preview: *mut c_void, buttons: c_int, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewControlBar {
        unsafe { @ELJPreviewControlBar(ELJPreviewControlBar_Create(preview, buttons, parent.handle(), title, x, y, w, h, style)) }
    }
}

pub trait _ELJPreviewControlBar : _wxPreviewControlBar {
}

pub struct ELJPreviewFrame(*mut c_void);
impl _ELJPreviewFrame for ELJPreviewFrame {}
impl _wxPreviewFrame for ELJPreviewFrame {}
impl _wxFrame for ELJPreviewFrame {}
impl _wxTopLevelWindow for ELJPreviewFrame {}
impl _wxWindow for ELJPreviewFrame {}
impl _wxEvtHandler for ELJPreviewFrame {}
impl _wxObject for ELJPreviewFrame { fn handle(&self) -> *mut c_void { **self } }

impl ELJPreviewFrame {
    pub fn from(handle: *mut c_void) -> @ELJPreviewFrame { @ELJPreviewFrame(handle) }
    pub fn null() -> @ELJPreviewFrame { ELJPreviewFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_obj: *mut c_void, _init: *mut c_void, _create_canvas: *mut c_void, _create_toolbar: *mut c_void, preview: *mut c_void, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewFrame {
        unsafe { @ELJPreviewFrame(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.handle(), title, x, y, w, h, style)) }
    }
}

pub trait _ELJPreviewFrame : _wxPreviewFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControlBar(&self) -> *mut c_void {
        unsafe { ELJPreviewFrame_GetControlBar(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPreviewCanvas(&self) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(ELJPreviewFrame_GetPreviewCanvas(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintPreview(&self) -> @wxPrintPreview {
        unsafe { @wxPrintPreview(ELJPreviewFrame_GetPrintPreview(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setControlBar(&self, obj: *mut c_void) {
        unsafe { ELJPreviewFrame_SetControlBar(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPreviewCanvas<T: _wxPreviewCanvas>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintPreview<T: _wxPrintPreview>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.handle(), obj.handle()) }
    }
}

pub struct ELJServer(*mut c_void);
impl _ELJServer for ELJServer {}
impl _wxServer for ELJServer {}
impl _wxServerBase for ELJServer {}
impl _wxObject for ELJServer { fn handle(&self) -> *mut c_void { **self } }

impl ELJServer {
    pub fn from(handle: *mut c_void) -> @ELJServer { @ELJServer(handle) }
    pub fn null() -> @ELJServer { ELJServer::from(0 as *mut c_void) }
    
}

pub trait _ELJServer : _wxServer {
}

pub struct ELJTextDropTarget(*mut c_void);
impl _ELJTextDropTarget for ELJTextDropTarget {}
impl _wxTextDropTarget for ELJTextDropTarget {}
impl _wxDropTarget for ELJTextDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl ELJTextDropTarget {
    pub fn from(handle: *mut c_void) -> @ELJTextDropTarget { @ELJTextDropTarget(handle) }
    pub fn null() -> @ELJTextDropTarget { ELJTextDropTarget::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> @ELJTextDropTarget {
        unsafe { @ELJTextDropTarget(ELJTextDropTarget_Create(_obj, _func)) }
    }
}

pub trait _ELJTextDropTarget : _wxTextDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJTextDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

pub struct ELJTextValidator(*mut c_void);
impl _ELJTextValidator for ELJTextValidator {}
impl _wxTextValidator for ELJTextValidator {}
impl _wxValidator for ELJTextValidator {}
impl _wxEvtHandler for ELJTextValidator {}
impl _wxObject for ELJTextValidator { fn handle(&self) -> *mut c_void { **self } }

impl ELJTextValidator {
    pub fn from(handle: *mut c_void) -> @ELJTextValidator { @ELJTextValidator(handle) }
    pub fn null() -> @ELJTextValidator { ELJTextValidator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void, _txt: *mut c_void, _stl: c_int) -> @ELJTextValidator {
        unsafe { @ELJTextValidator(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) }
    }
}

pub trait _ELJTextValidator : _wxTextValidator {
}

pub struct cbAntiflickerPlugin(*mut c_void);
impl _cbAntiflickerPlugin for cbAntiflickerPlugin {}
impl _cbPluginBase for cbAntiflickerPlugin {}
impl _wxEvtHandler for cbAntiflickerPlugin {}
impl _wxObject for cbAntiflickerPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbAntiflickerPlugin {
    pub fn from(handle: *mut c_void) -> @cbAntiflickerPlugin { @cbAntiflickerPlugin(handle) }
    pub fn null() -> @cbAntiflickerPlugin { cbAntiflickerPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbAntiflickerPlugin : _cbPluginBase {
}

pub struct cbBarDragPlugin(*mut c_void);
impl _cbBarDragPlugin for cbBarDragPlugin {}
impl _cbPluginBase for cbBarDragPlugin {}
impl _wxEvtHandler for cbBarDragPlugin {}
impl _wxObject for cbBarDragPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbBarDragPlugin {
    pub fn from(handle: *mut c_void) -> @cbBarDragPlugin { @cbBarDragPlugin(handle) }
    pub fn null() -> @cbBarDragPlugin { cbBarDragPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbBarDragPlugin : _cbPluginBase {
}

pub struct cbBarHintsPlugin(*mut c_void);
impl _cbBarHintsPlugin for cbBarHintsPlugin {}
impl _cbPluginBase for cbBarHintsPlugin {}
impl _wxEvtHandler for cbBarHintsPlugin {}
impl _wxObject for cbBarHintsPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbBarHintsPlugin {
    pub fn from(handle: *mut c_void) -> @cbBarHintsPlugin { @cbBarHintsPlugin(handle) }
    pub fn null() -> @cbBarHintsPlugin { cbBarHintsPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbBarHintsPlugin : _cbPluginBase {
}

pub struct cbBarInfo(*mut c_void);
impl _cbBarInfo for cbBarInfo {}
impl _wxObject for cbBarInfo { fn handle(&self) -> *mut c_void { **self } }

impl cbBarInfo {
    pub fn from(handle: *mut c_void) -> @cbBarInfo { @cbBarInfo(handle) }
    pub fn null() -> @cbBarInfo { cbBarInfo::from(0 as *mut c_void) }
    
}

pub trait _cbBarInfo : _wxObject {
}

pub struct cbBarSpy(*mut c_void);
impl _cbBarSpy for cbBarSpy {}
impl _wxEvtHandler for cbBarSpy {}
impl _wxObject for cbBarSpy { fn handle(&self) -> *mut c_void { **self } }

impl cbBarSpy {
    pub fn from(handle: *mut c_void) -> @cbBarSpy { @cbBarSpy(handle) }
    pub fn null() -> @cbBarSpy { cbBarSpy::from(0 as *mut c_void) }
    
}

pub trait _cbBarSpy : _wxEvtHandler {
}

pub struct cbCloseBox(*mut c_void);
impl _cbCloseBox for cbCloseBox {}
impl _cbMiniButton for cbCloseBox {}
impl _wxObject for cbCloseBox { fn handle(&self) -> *mut c_void { **self } }

impl cbCloseBox {
    pub fn from(handle: *mut c_void) -> @cbCloseBox { @cbCloseBox(handle) }
    pub fn null() -> @cbCloseBox { cbCloseBox::from(0 as *mut c_void) }
    
}

pub trait _cbCloseBox : _cbMiniButton {
}

pub struct cbCollapseBox(*mut c_void);
impl _cbCollapseBox for cbCollapseBox {}
impl _cbMiniButton for cbCollapseBox {}
impl _wxObject for cbCollapseBox { fn handle(&self) -> *mut c_void { **self } }

impl cbCollapseBox {
    pub fn from(handle: *mut c_void) -> @cbCollapseBox { @cbCollapseBox(handle) }
    pub fn null() -> @cbCollapseBox { cbCollapseBox::from(0 as *mut c_void) }
    
}

pub trait _cbCollapseBox : _cbMiniButton {
}

pub struct cbCommonPaneProperties(*mut c_void);
impl _cbCommonPaneProperties for cbCommonPaneProperties {}
impl _wxObject for cbCommonPaneProperties { fn handle(&self) -> *mut c_void { **self } }

impl cbCommonPaneProperties {
    pub fn from(handle: *mut c_void) -> @cbCommonPaneProperties { @cbCommonPaneProperties(handle) }
    pub fn null() -> @cbCommonPaneProperties { cbCommonPaneProperties::from(0 as *mut c_void) }
    
}

pub trait _cbCommonPaneProperties : _wxObject {
}

pub struct cbCustomizeBarEvent(*mut c_void);
impl _cbCustomizeBarEvent for cbCustomizeBarEvent {}
impl _cbPluginEvent for cbCustomizeBarEvent {}
impl _wxEvent for cbCustomizeBarEvent {}
impl _wxObject for cbCustomizeBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbCustomizeBarEvent {
    pub fn from(handle: *mut c_void) -> @cbCustomizeBarEvent { @cbCustomizeBarEvent(handle) }
    pub fn null() -> @cbCustomizeBarEvent { cbCustomizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbCustomizeBarEvent : _cbPluginEvent {
}

pub struct cbCustomizeLayoutEvent(*mut c_void);
impl _cbCustomizeLayoutEvent for cbCustomizeLayoutEvent {}
impl _cbPluginEvent for cbCustomizeLayoutEvent {}
impl _wxEvent for cbCustomizeLayoutEvent {}
impl _wxObject for cbCustomizeLayoutEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbCustomizeLayoutEvent {
    pub fn from(handle: *mut c_void) -> @cbCustomizeLayoutEvent { @cbCustomizeLayoutEvent(handle) }
    pub fn null() -> @cbCustomizeLayoutEvent { cbCustomizeLayoutEvent::from(0 as *mut c_void) }
    
}

pub trait _cbCustomizeLayoutEvent : _cbPluginEvent {
}

pub struct cbDimHandlerBase(*mut c_void);
impl _cbDimHandlerBase for cbDimHandlerBase {}
impl _wxObject for cbDimHandlerBase { fn handle(&self) -> *mut c_void { **self } }

impl cbDimHandlerBase {
    pub fn from(handle: *mut c_void) -> @cbDimHandlerBase { @cbDimHandlerBase(handle) }
    pub fn null() -> @cbDimHandlerBase { cbDimHandlerBase::from(0 as *mut c_void) }
    
}

pub trait _cbDimHandlerBase : _wxObject {
}

pub struct cbDimInfo(*mut c_void);
impl _cbDimInfo for cbDimInfo {}
impl _wxObject for cbDimInfo { fn handle(&self) -> *mut c_void { **self } }

impl cbDimInfo {
    pub fn from(handle: *mut c_void) -> @cbDimInfo { @cbDimInfo(handle) }
    pub fn null() -> @cbDimInfo { cbDimInfo::from(0 as *mut c_void) }
    
}

pub trait _cbDimInfo : _wxObject {
}

pub struct cbDockBox(*mut c_void);
impl _cbDockBox for cbDockBox {}
impl _cbMiniButton for cbDockBox {}
impl _wxObject for cbDockBox { fn handle(&self) -> *mut c_void { **self } }

impl cbDockBox {
    pub fn from(handle: *mut c_void) -> @cbDockBox { @cbDockBox(handle) }
    pub fn null() -> @cbDockBox { cbDockBox::from(0 as *mut c_void) }
    
}

pub trait _cbDockBox : _cbMiniButton {
}

pub struct cbDockPane(*mut c_void);
impl _cbDockPane for cbDockPane {}
impl _wxObject for cbDockPane { fn handle(&self) -> *mut c_void { **self } }

impl cbDockPane {
    pub fn from(handle: *mut c_void) -> @cbDockPane { @cbDockPane(handle) }
    pub fn null() -> @cbDockPane { cbDockPane::from(0 as *mut c_void) }
    
}

pub trait _cbDockPane : _wxObject {
}

pub struct cbDrawBarDecorEvent(*mut c_void);
impl _cbDrawBarDecorEvent for cbDrawBarDecorEvent {}
impl _cbPluginEvent for cbDrawBarDecorEvent {}
impl _wxEvent for cbDrawBarDecorEvent {}
impl _wxObject for cbDrawBarDecorEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawBarDecorEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawBarDecorEvent { @cbDrawBarDecorEvent(handle) }
    pub fn null() -> @cbDrawBarDecorEvent { cbDrawBarDecorEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawBarDecorEvent : _cbPluginEvent {
}

pub struct cbDrawBarHandlesEvent(*mut c_void);
impl _cbDrawBarHandlesEvent for cbDrawBarHandlesEvent {}
impl _cbPluginEvent for cbDrawBarHandlesEvent {}
impl _wxEvent for cbDrawBarHandlesEvent {}
impl _wxObject for cbDrawBarHandlesEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawBarHandlesEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawBarHandlesEvent { @cbDrawBarHandlesEvent(handle) }
    pub fn null() -> @cbDrawBarHandlesEvent { cbDrawBarHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawBarHandlesEvent : _cbPluginEvent {
}

pub struct cbDrawHintRectEvent(*mut c_void);
impl _cbDrawHintRectEvent for cbDrawHintRectEvent {}
impl _cbPluginEvent for cbDrawHintRectEvent {}
impl _wxEvent for cbDrawHintRectEvent {}
impl _wxObject for cbDrawHintRectEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawHintRectEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawHintRectEvent { @cbDrawHintRectEvent(handle) }
    pub fn null() -> @cbDrawHintRectEvent { cbDrawHintRectEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawHintRectEvent : _cbPluginEvent {
}

pub struct cbDrawPaneBkGroundEvent(*mut c_void);
impl _cbDrawPaneBkGroundEvent for cbDrawPaneBkGroundEvent {}
impl _cbPluginEvent for cbDrawPaneBkGroundEvent {}
impl _wxEvent for cbDrawPaneBkGroundEvent {}
impl _wxObject for cbDrawPaneBkGroundEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawPaneBkGroundEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawPaneBkGroundEvent { @cbDrawPaneBkGroundEvent(handle) }
    pub fn null() -> @cbDrawPaneBkGroundEvent { cbDrawPaneBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawPaneBkGroundEvent : _cbPluginEvent {
}

pub struct cbDrawPaneDecorEvent(*mut c_void);
impl _cbDrawPaneDecorEvent for cbDrawPaneDecorEvent {}
impl _cbPluginEvent for cbDrawPaneDecorEvent {}
impl _wxEvent for cbDrawPaneDecorEvent {}
impl _wxObject for cbDrawPaneDecorEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawPaneDecorEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawPaneDecorEvent { @cbDrawPaneDecorEvent(handle) }
    pub fn null() -> @cbDrawPaneDecorEvent { cbDrawPaneDecorEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawPaneDecorEvent : _cbPluginEvent {
}

pub struct cbDrawRowBkGroundEvent(*mut c_void);
impl _cbDrawRowBkGroundEvent for cbDrawRowBkGroundEvent {}
impl _cbPluginEvent for cbDrawRowBkGroundEvent {}
impl _wxEvent for cbDrawRowBkGroundEvent {}
impl _wxObject for cbDrawRowBkGroundEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawRowBkGroundEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawRowBkGroundEvent { @cbDrawRowBkGroundEvent(handle) }
    pub fn null() -> @cbDrawRowBkGroundEvent { cbDrawRowBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawRowBkGroundEvent : _cbPluginEvent {
}

pub struct cbDrawRowDecorEvent(*mut c_void);
impl _cbDrawRowDecorEvent for cbDrawRowDecorEvent {}
impl _cbPluginEvent for cbDrawRowDecorEvent {}
impl _wxEvent for cbDrawRowDecorEvent {}
impl _wxObject for cbDrawRowDecorEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawRowDecorEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawRowDecorEvent { @cbDrawRowDecorEvent(handle) }
    pub fn null() -> @cbDrawRowDecorEvent { cbDrawRowDecorEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawRowDecorEvent : _cbPluginEvent {
}

pub struct cbDrawRowHandlesEvent(*mut c_void);
impl _cbDrawRowHandlesEvent for cbDrawRowHandlesEvent {}
impl _cbPluginEvent for cbDrawRowHandlesEvent {}
impl _wxEvent for cbDrawRowHandlesEvent {}
impl _wxObject for cbDrawRowHandlesEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawRowHandlesEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawRowHandlesEvent { @cbDrawRowHandlesEvent(handle) }
    pub fn null() -> @cbDrawRowHandlesEvent { cbDrawRowHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawRowHandlesEvent : _cbPluginEvent {
}

pub struct cbDynToolBarDimHandler(*mut c_void);
impl _cbDynToolBarDimHandler for cbDynToolBarDimHandler {}
impl _cbDimHandlerBase for cbDynToolBarDimHandler {}
impl _wxObject for cbDynToolBarDimHandler { fn handle(&self) -> *mut c_void { **self } }

impl cbDynToolBarDimHandler {
    pub fn from(handle: *mut c_void) -> @cbDynToolBarDimHandler { @cbDynToolBarDimHandler(handle) }
    pub fn null() -> @cbDynToolBarDimHandler { cbDynToolBarDimHandler::from(0 as *mut c_void) }
    
}

pub trait _cbDynToolBarDimHandler : _cbDimHandlerBase {
}

pub struct cbFinishDrawInAreaEvent(*mut c_void);
impl _cbFinishDrawInAreaEvent for cbFinishDrawInAreaEvent {}
impl _cbPluginEvent for cbFinishDrawInAreaEvent {}
impl _wxEvent for cbFinishDrawInAreaEvent {}
impl _wxObject for cbFinishDrawInAreaEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbFinishDrawInAreaEvent {
    pub fn from(handle: *mut c_void) -> @cbFinishDrawInAreaEvent { @cbFinishDrawInAreaEvent(handle) }
    pub fn null() -> @cbFinishDrawInAreaEvent { cbFinishDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait _cbFinishDrawInAreaEvent : _cbPluginEvent {
}

pub struct cbFloatedBarWindow(*mut c_void);
impl _cbFloatedBarWindow for cbFloatedBarWindow {}
impl _wxToolWindow for cbFloatedBarWindow {}
impl _wxFrame for cbFloatedBarWindow {}
impl _wxTopLevelWindow for cbFloatedBarWindow {}
impl _wxWindow for cbFloatedBarWindow {}
impl _wxEvtHandler for cbFloatedBarWindow {}
impl _wxObject for cbFloatedBarWindow { fn handle(&self) -> *mut c_void { **self } }

impl cbFloatedBarWindow {
    pub fn from(handle: *mut c_void) -> @cbFloatedBarWindow { @cbFloatedBarWindow(handle) }
    pub fn null() -> @cbFloatedBarWindow { cbFloatedBarWindow::from(0 as *mut c_void) }
    
}

pub trait _cbFloatedBarWindow : _wxToolWindow {
}

pub struct cbGCUpdatesMgr(*mut c_void);
impl _cbGCUpdatesMgr for cbGCUpdatesMgr {}
impl _cbSimpleUpdatesMgr for cbGCUpdatesMgr {}
impl _cbUpdatesManagerBase for cbGCUpdatesMgr {}
impl _wxObject for cbGCUpdatesMgr { fn handle(&self) -> *mut c_void { **self } }

impl cbGCUpdatesMgr {
    pub fn from(handle: *mut c_void) -> @cbGCUpdatesMgr { @cbGCUpdatesMgr(handle) }
    pub fn null() -> @cbGCUpdatesMgr { cbGCUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait _cbGCUpdatesMgr : _cbSimpleUpdatesMgr {
}

pub struct cbHintAnimationPlugin(*mut c_void);
impl _cbHintAnimationPlugin for cbHintAnimationPlugin {}
impl _cbPluginBase for cbHintAnimationPlugin {}
impl _wxEvtHandler for cbHintAnimationPlugin {}
impl _wxObject for cbHintAnimationPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbHintAnimationPlugin {
    pub fn from(handle: *mut c_void) -> @cbHintAnimationPlugin { @cbHintAnimationPlugin(handle) }
    pub fn null() -> @cbHintAnimationPlugin { cbHintAnimationPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbHintAnimationPlugin : _cbPluginBase {
}

pub struct cbInsertBarEvent(*mut c_void);
impl _cbInsertBarEvent for cbInsertBarEvent {}
impl _cbPluginEvent for cbInsertBarEvent {}
impl _wxEvent for cbInsertBarEvent {}
impl _wxObject for cbInsertBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbInsertBarEvent {
    pub fn from(handle: *mut c_void) -> @cbInsertBarEvent { @cbInsertBarEvent(handle) }
    pub fn null() -> @cbInsertBarEvent { cbInsertBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbInsertBarEvent : _cbPluginEvent {
}

pub struct cbLayoutRowEvent(*mut c_void);
impl _cbLayoutRowEvent for cbLayoutRowEvent {}
impl _cbPluginEvent for cbLayoutRowEvent {}
impl _wxEvent for cbLayoutRowEvent {}
impl _wxObject for cbLayoutRowEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLayoutRowEvent {
    pub fn from(handle: *mut c_void) -> @cbLayoutRowEvent { @cbLayoutRowEvent(handle) }
    pub fn null() -> @cbLayoutRowEvent { cbLayoutRowEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLayoutRowEvent : _cbPluginEvent {
}

pub struct cbLeftDClickEvent(*mut c_void);
impl _cbLeftDClickEvent for cbLeftDClickEvent {}
impl _cbPluginEvent for cbLeftDClickEvent {}
impl _wxEvent for cbLeftDClickEvent {}
impl _wxObject for cbLeftDClickEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLeftDClickEvent {
    pub fn from(handle: *mut c_void) -> @cbLeftDClickEvent { @cbLeftDClickEvent(handle) }
    pub fn null() -> @cbLeftDClickEvent { cbLeftDClickEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLeftDClickEvent : _cbPluginEvent {
}

pub struct cbLeftDownEvent(*mut c_void);
impl _cbLeftDownEvent for cbLeftDownEvent {}
impl _cbPluginEvent for cbLeftDownEvent {}
impl _wxEvent for cbLeftDownEvent {}
impl _wxObject for cbLeftDownEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLeftDownEvent {
    pub fn from(handle: *mut c_void) -> @cbLeftDownEvent { @cbLeftDownEvent(handle) }
    pub fn null() -> @cbLeftDownEvent { cbLeftDownEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLeftDownEvent : _cbPluginEvent {
}

pub struct cbLeftUpEvent(*mut c_void);
impl _cbLeftUpEvent for cbLeftUpEvent {}
impl _cbPluginEvent for cbLeftUpEvent {}
impl _wxEvent for cbLeftUpEvent {}
impl _wxObject for cbLeftUpEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLeftUpEvent {
    pub fn from(handle: *mut c_void) -> @cbLeftUpEvent { @cbLeftUpEvent(handle) }
    pub fn null() -> @cbLeftUpEvent { cbLeftUpEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLeftUpEvent : _cbPluginEvent {
}

pub struct cbMiniButton(*mut c_void);
impl _cbMiniButton for cbMiniButton {}
impl _wxObject for cbMiniButton { fn handle(&self) -> *mut c_void { **self } }

impl cbMiniButton {
    pub fn from(handle: *mut c_void) -> @cbMiniButton { @cbMiniButton(handle) }
    pub fn null() -> @cbMiniButton { cbMiniButton::from(0 as *mut c_void) }
    
}

pub trait _cbMiniButton : _wxObject {
}

pub struct cbMotionEvent(*mut c_void);
impl _cbMotionEvent for cbMotionEvent {}
impl _cbPluginEvent for cbMotionEvent {}
impl _wxEvent for cbMotionEvent {}
impl _wxObject for cbMotionEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbMotionEvent {
    pub fn from(handle: *mut c_void) -> @cbMotionEvent { @cbMotionEvent(handle) }
    pub fn null() -> @cbMotionEvent { cbMotionEvent::from(0 as *mut c_void) }
    
}

pub trait _cbMotionEvent : _cbPluginEvent {
}

pub struct cbPaneDrawPlugin(*mut c_void);
impl _cbPaneDrawPlugin for cbPaneDrawPlugin {}
impl _cbPluginBase for cbPaneDrawPlugin {}
impl _wxEvtHandler for cbPaneDrawPlugin {}
impl _wxObject for cbPaneDrawPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbPaneDrawPlugin {
    pub fn from(handle: *mut c_void) -> @cbPaneDrawPlugin { @cbPaneDrawPlugin(handle) }
    pub fn null() -> @cbPaneDrawPlugin { cbPaneDrawPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbPaneDrawPlugin : _cbPluginBase {
}

pub struct cbPluginBase(*mut c_void);
impl _cbPluginBase for cbPluginBase {}
impl _wxEvtHandler for cbPluginBase {}
impl _wxObject for cbPluginBase { fn handle(&self) -> *mut c_void { **self } }

impl cbPluginBase {
    pub fn from(handle: *mut c_void) -> @cbPluginBase { @cbPluginBase(handle) }
    pub fn null() -> @cbPluginBase { cbPluginBase::from(0 as *mut c_void) }
    
}

pub trait _cbPluginBase : _wxEvtHandler {
}

pub struct cbPluginEvent(*mut c_void);
impl _cbPluginEvent for cbPluginEvent {}
impl _wxEvent for cbPluginEvent {}
impl _wxObject for cbPluginEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbPluginEvent {
    pub fn from(handle: *mut c_void) -> @cbPluginEvent { @cbPluginEvent(handle) }
    pub fn null() -> @cbPluginEvent { cbPluginEvent::from(0 as *mut c_void) }
    
}

pub trait _cbPluginEvent : _wxEvent {
}

pub struct cbRemoveBarEvent(*mut c_void);
impl _cbRemoveBarEvent for cbRemoveBarEvent {}
impl _cbPluginEvent for cbRemoveBarEvent {}
impl _wxEvent for cbRemoveBarEvent {}
impl _wxObject for cbRemoveBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbRemoveBarEvent {
    pub fn from(handle: *mut c_void) -> @cbRemoveBarEvent { @cbRemoveBarEvent(handle) }
    pub fn null() -> @cbRemoveBarEvent { cbRemoveBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbRemoveBarEvent : _cbPluginEvent {
}

pub struct cbResizeBarEvent(*mut c_void);
impl _cbResizeBarEvent for cbResizeBarEvent {}
impl _cbPluginEvent for cbResizeBarEvent {}
impl _wxEvent for cbResizeBarEvent {}
impl _wxObject for cbResizeBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbResizeBarEvent {
    pub fn from(handle: *mut c_void) -> @cbResizeBarEvent { @cbResizeBarEvent(handle) }
    pub fn null() -> @cbResizeBarEvent { cbResizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbResizeBarEvent : _cbPluginEvent {
}

pub struct cbResizeRowEvent(*mut c_void);
impl _cbResizeRowEvent for cbResizeRowEvent {}
impl _cbPluginEvent for cbResizeRowEvent {}
impl _wxEvent for cbResizeRowEvent {}
impl _wxObject for cbResizeRowEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbResizeRowEvent {
    pub fn from(handle: *mut c_void) -> @cbResizeRowEvent { @cbResizeRowEvent(handle) }
    pub fn null() -> @cbResizeRowEvent { cbResizeRowEvent::from(0 as *mut c_void) }
    
}

pub trait _cbResizeRowEvent : _cbPluginEvent {
}

pub struct cbRightDownEvent(*mut c_void);
impl _cbRightDownEvent for cbRightDownEvent {}
impl _cbPluginEvent for cbRightDownEvent {}
impl _wxEvent for cbRightDownEvent {}
impl _wxObject for cbRightDownEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbRightDownEvent {
    pub fn from(handle: *mut c_void) -> @cbRightDownEvent { @cbRightDownEvent(handle) }
    pub fn null() -> @cbRightDownEvent { cbRightDownEvent::from(0 as *mut c_void) }
    
}

pub trait _cbRightDownEvent : _cbPluginEvent {
}

pub struct cbRightUpEvent(*mut c_void);
impl _cbRightUpEvent for cbRightUpEvent {}
impl _cbPluginEvent for cbRightUpEvent {}
impl _wxEvent for cbRightUpEvent {}
impl _wxObject for cbRightUpEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbRightUpEvent {
    pub fn from(handle: *mut c_void) -> @cbRightUpEvent { @cbRightUpEvent(handle) }
    pub fn null() -> @cbRightUpEvent { cbRightUpEvent::from(0 as *mut c_void) }
    
}

pub trait _cbRightUpEvent : _cbPluginEvent {
}

pub struct cbRowDragPlugin(*mut c_void);
impl _cbRowDragPlugin for cbRowDragPlugin {}
impl _cbPluginBase for cbRowDragPlugin {}
impl _wxEvtHandler for cbRowDragPlugin {}
impl _wxObject for cbRowDragPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbRowDragPlugin {
    pub fn from(handle: *mut c_void) -> @cbRowDragPlugin { @cbRowDragPlugin(handle) }
    pub fn null() -> @cbRowDragPlugin { cbRowDragPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbRowDragPlugin : _cbPluginBase {
}

pub struct cbRowInfo(*mut c_void);
impl _cbRowInfo for cbRowInfo {}
impl _wxObject for cbRowInfo { fn handle(&self) -> *mut c_void { **self } }

impl cbRowInfo {
    pub fn from(handle: *mut c_void) -> @cbRowInfo { @cbRowInfo(handle) }
    pub fn null() -> @cbRowInfo { cbRowInfo::from(0 as *mut c_void) }
    
}

pub trait _cbRowInfo : _wxObject {
}

pub struct cbRowLayoutPlugin(*mut c_void);
impl _cbRowLayoutPlugin for cbRowLayoutPlugin {}
impl _cbPluginBase for cbRowLayoutPlugin {}
impl _wxEvtHandler for cbRowLayoutPlugin {}
impl _wxObject for cbRowLayoutPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbRowLayoutPlugin {
    pub fn from(handle: *mut c_void) -> @cbRowLayoutPlugin { @cbRowLayoutPlugin(handle) }
    pub fn null() -> @cbRowLayoutPlugin { cbRowLayoutPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbRowLayoutPlugin : _cbPluginBase {
}

pub struct cbSimpleCustomizationPlugin(*mut c_void);
impl _cbSimpleCustomizationPlugin for cbSimpleCustomizationPlugin {}
impl _cbPluginBase for cbSimpleCustomizationPlugin {}
impl _wxEvtHandler for cbSimpleCustomizationPlugin {}
impl _wxObject for cbSimpleCustomizationPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbSimpleCustomizationPlugin {
    pub fn from(handle: *mut c_void) -> @cbSimpleCustomizationPlugin { @cbSimpleCustomizationPlugin(handle) }
    pub fn null() -> @cbSimpleCustomizationPlugin { cbSimpleCustomizationPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbSimpleCustomizationPlugin : _cbPluginBase {
}

pub struct cbSimpleUpdatesMgr(*mut c_void);
impl _cbSimpleUpdatesMgr for cbSimpleUpdatesMgr {}
impl _cbUpdatesManagerBase for cbSimpleUpdatesMgr {}
impl _wxObject for cbSimpleUpdatesMgr { fn handle(&self) -> *mut c_void { **self } }

impl cbSimpleUpdatesMgr {
    pub fn from(handle: *mut c_void) -> @cbSimpleUpdatesMgr { @cbSimpleUpdatesMgr(handle) }
    pub fn null() -> @cbSimpleUpdatesMgr { cbSimpleUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait _cbSimpleUpdatesMgr : _cbUpdatesManagerBase {
}

pub struct cbSizeBarWndEvent(*mut c_void);
impl _cbSizeBarWndEvent for cbSizeBarWndEvent {}
impl _cbPluginEvent for cbSizeBarWndEvent {}
impl _wxEvent for cbSizeBarWndEvent {}
impl _wxObject for cbSizeBarWndEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbSizeBarWndEvent {
    pub fn from(handle: *mut c_void) -> @cbSizeBarWndEvent { @cbSizeBarWndEvent(handle) }
    pub fn null() -> @cbSizeBarWndEvent { cbSizeBarWndEvent::from(0 as *mut c_void) }
    
}

pub trait _cbSizeBarWndEvent : _cbPluginEvent {
}

pub struct cbStartBarDraggingEvent(*mut c_void);
impl _cbStartBarDraggingEvent for cbStartBarDraggingEvent {}
impl _cbPluginEvent for cbStartBarDraggingEvent {}
impl _wxEvent for cbStartBarDraggingEvent {}
impl _wxObject for cbStartBarDraggingEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbStartBarDraggingEvent {
    pub fn from(handle: *mut c_void) -> @cbStartBarDraggingEvent { @cbStartBarDraggingEvent(handle) }
    pub fn null() -> @cbStartBarDraggingEvent { cbStartBarDraggingEvent::from(0 as *mut c_void) }
    
}

pub trait _cbStartBarDraggingEvent : _cbPluginEvent {
}

pub struct cbStartDrawInAreaEvent(*mut c_void);
impl _cbStartDrawInAreaEvent for cbStartDrawInAreaEvent {}
impl _cbPluginEvent for cbStartDrawInAreaEvent {}
impl _wxEvent for cbStartDrawInAreaEvent {}
impl _wxObject for cbStartDrawInAreaEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbStartDrawInAreaEvent {
    pub fn from(handle: *mut c_void) -> @cbStartDrawInAreaEvent { @cbStartDrawInAreaEvent(handle) }
    pub fn null() -> @cbStartDrawInAreaEvent { cbStartDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait _cbStartDrawInAreaEvent : _cbPluginEvent {
}

pub struct cbUpdatesManagerBase(*mut c_void);
impl _cbUpdatesManagerBase for cbUpdatesManagerBase {}
impl _wxObject for cbUpdatesManagerBase { fn handle(&self) -> *mut c_void { **self } }

impl cbUpdatesManagerBase {
    pub fn from(handle: *mut c_void) -> @cbUpdatesManagerBase { @cbUpdatesManagerBase(handle) }
    pub fn null() -> @cbUpdatesManagerBase { cbUpdatesManagerBase::from(0 as *mut c_void) }
    
}

pub trait _cbUpdatesManagerBase : _wxObject {
}

pub struct wxPaintEvent(*mut c_void);
impl _wxPaintEvent for wxPaintEvent {}
impl _wxEvent for wxPaintEvent {}
impl _wxObject for wxPaintEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxPaintEvent {
    pub fn from(handle: *mut c_void) -> @wxPaintEvent { @wxPaintEvent(handle) }
    pub fn null() -> @wxPaintEvent { wxPaintEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPaintEvent : _wxEvent {
}

pub struct wxPaletteChangedEvent(*mut c_void);
impl _wxPaletteChangedEvent for wxPaletteChangedEvent {}
impl _wxEvent for wxPaletteChangedEvent {}
impl _wxObject for wxPaletteChangedEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxPaletteChangedEvent {
    pub fn from(handle: *mut c_void) -> @wxPaletteChangedEvent { @wxPaletteChangedEvent(handle) }
    pub fn null() -> @wxPaletteChangedEvent { wxPaletteChangedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPaletteChangedEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChangedWindow(&self) -> *mut c_void {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChangedWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.handle(), win.handle()) }
    }
}

pub struct wxPlotCurve(*mut c_void);
impl _wxPlotCurve for wxPlotCurve {}
impl _wxObject for wxPlotCurve { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotCurve {
    pub fn from(handle: *mut c_void) -> @wxPlotCurve { @wxPlotCurve(handle) }
    pub fn null() -> @wxPlotCurve { wxPlotCurve::from(0 as *mut c_void) }
    
}

pub trait _wxPlotCurve : _wxObject {
}

pub struct wxPlotEvent(*mut c_void);
impl _wxPlotEvent for wxPlotEvent {}
impl _wxNotifyEvent for wxPlotEvent {}
impl _wxCommandEvent for wxPlotEvent {}
impl _wxEvent for wxPlotEvent {}
impl _wxObject for wxPlotEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotEvent {
    pub fn from(handle: *mut c_void) -> @wxPlotEvent { @wxPlotEvent(handle) }
    pub fn null() -> @wxPlotEvent { wxPlotEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPlotEvent : _wxNotifyEvent {
}

pub struct wxPlotOnOffCurve(*mut c_void);
impl _wxPlotOnOffCurve for wxPlotOnOffCurve {}
impl _wxObject for wxPlotOnOffCurve { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotOnOffCurve {
    pub fn from(handle: *mut c_void) -> @wxPlotOnOffCurve { @wxPlotOnOffCurve(handle) }
    pub fn null() -> @wxPlotOnOffCurve { wxPlotOnOffCurve::from(0 as *mut c_void) }
    
}

pub trait _wxPlotOnOffCurve : _wxObject {
}

pub struct wxPlotWindow(*mut c_void);
impl _wxPlotWindow for wxPlotWindow {}
impl _wxScrolledWindow for wxPlotWindow {}
impl _wxPanel for wxPlotWindow {}
impl _wxWindow for wxPlotWindow {}
impl _wxEvtHandler for wxPlotWindow {}
impl _wxObject for wxPlotWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotWindow {
    pub fn from(handle: *mut c_void) -> @wxPlotWindow { @wxPlotWindow(handle) }
    pub fn null() -> @wxPlotWindow { wxPlotWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPlotWindow : _wxScrolledWindow {
}

pub struct wxPostScriptPrintNativeData(*mut c_void);
impl _wxPostScriptPrintNativeData for wxPostScriptPrintNativeData {}
impl _wxObject for wxPostScriptPrintNativeData { fn handle(&self) -> *mut c_void { **self } }

impl wxPostScriptPrintNativeData {
    pub fn from(handle: *mut c_void) -> @wxPostScriptPrintNativeData { @wxPostScriptPrintNativeData(handle) }
    pub fn null() -> @wxPostScriptPrintNativeData { wxPostScriptPrintNativeData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxPostScriptPrintNativeData {
        unsafe { @wxPostScriptPrintNativeData(wxPostScriptPrintNativeData_Create()) }
    }
}

pub trait _wxPostScriptPrintNativeData : _wxObject {
}

pub struct wxPrivateDropTarget(*mut c_void);
impl _wxPrivateDropTarget for wxPrivateDropTarget {}
impl _wxDropTarget for wxPrivateDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl wxPrivateDropTarget {
    pub fn from(handle: *mut c_void) -> @wxPrivateDropTarget { @wxPrivateDropTarget(handle) }
    pub fn null() -> @wxPrivateDropTarget { wxPrivateDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxPrivateDropTarget : _wxDropTarget {
}

pub struct wxProcessEvent(*mut c_void);
impl _wxProcessEvent for wxProcessEvent {}
impl _wxEvent for wxProcessEvent {}
impl _wxObject for wxProcessEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxProcessEvent {
    pub fn from(handle: *mut c_void) -> @wxProcessEvent { @wxProcessEvent(handle) }
    pub fn null() -> @wxProcessEvent { wxProcessEvent::from(0 as *mut c_void) }
    
}

pub trait _wxProcessEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPid(&self) -> c_int {
        unsafe { wxProcessEvent_GetPid(self.handle()) }
    }
}

pub struct wxProgressDialog(*mut c_void);
impl _wxProgressDialog for wxProgressDialog {}
impl _wxFrame for wxProgressDialog {}
impl _wxTopLevelWindow for wxProgressDialog {}
impl _wxWindow for wxProgressDialog {}
impl _wxEvtHandler for wxProgressDialog {}
impl _wxObject for wxProgressDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxProgressDialog {
    pub fn from(handle: *mut c_void) -> @wxProgressDialog { @wxProgressDialog(handle) }
    pub fn null() -> @wxProgressDialog { wxProgressDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(title: &str, message: &str, max: c_int, parent: &T, style: c_int) -> @wxProgressDialog {
        let title = wxT(title);
        let message = wxT(message);
        unsafe { @wxProgressDialog(wxProgressDialog_Create(title.handle(), message.handle(), max, parent.handle(), style)) }
    }
}

pub trait _wxProgressDialog : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn update(&self, value: c_int) -> c_int {
        unsafe { wxProgressDialog_Update(self.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateWithMessage(&self, value: c_int, message: &str) -> c_int {
        let message = wxT(message);
        unsafe { wxProgressDialog_UpdateWithMessage(self.handle(), value, message.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.handle()) }
    }
}

pub struct wxQueryCol(*mut c_void);
impl _wxQueryCol for wxQueryCol {}
impl _wxObject for wxQueryCol { fn handle(&self) -> *mut c_void { **self } }

impl wxQueryCol {
    pub fn from(handle: *mut c_void) -> @wxQueryCol { @wxQueryCol(handle) }
    pub fn null() -> @wxQueryCol { wxQueryCol::from(0 as *mut c_void) }
    
}

pub trait _wxQueryCol : _wxObject {
}

pub struct wxQueryField(*mut c_void);
impl _wxQueryField for wxQueryField {}
impl _wxObject for wxQueryField { fn handle(&self) -> *mut c_void { **self } }

impl wxQueryField {
    pub fn from(handle: *mut c_void) -> @wxQueryField { @wxQueryField(handle) }
    pub fn null() -> @wxQueryField { wxQueryField::from(0 as *mut c_void) }
    
}

pub trait _wxQueryField : _wxObject {
}

pub struct wxQueryLayoutInfoEvent(*mut c_void);
impl _wxQueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl _wxEvent for wxQueryLayoutInfoEvent {}
impl _wxObject for wxQueryLayoutInfoEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxQueryLayoutInfoEvent {
    pub fn from(handle: *mut c_void) -> @wxQueryLayoutInfoEvent { @wxQueryLayoutInfoEvent(handle) }
    pub fn null() -> @wxQueryLayoutInfoEvent { wxQueryLayoutInfoEvent::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(id: c_int) -> @wxQueryLayoutInfoEvent {
        unsafe { @wxQueryLayoutInfoEvent(wxQueryLayoutInfoEvent_Create(id)) }
    }
}

pub trait _wxQueryLayoutInfoEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlignment(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRequestedLength(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetRequestedLength(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxQueryLayoutInfoEvent_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAlignment(&self, align: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetAlignment(self.handle(), align) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetOrientation(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRequestedLength(&self, length: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetRequestedLength(self.handle(), length) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSize(&self, w: c_int, h: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetSize(self.handle(), w, h) }
    }
}

pub struct wxQueryNewPaletteEvent(*mut c_void);
impl _wxQueryNewPaletteEvent for wxQueryNewPaletteEvent {}
impl _wxEvent for wxQueryNewPaletteEvent {}
impl _wxObject for wxQueryNewPaletteEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxQueryNewPaletteEvent {
    pub fn from(handle: *mut c_void) -> @wxQueryNewPaletteEvent { @wxQueryNewPaletteEvent(handle) }
    pub fn null() -> @wxQueryNewPaletteEvent { wxQueryNewPaletteEvent::from(0 as *mut c_void) }
    
}

pub trait _wxQueryNewPaletteEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaletteRealized(&self) -> c_int {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaletteRealized(&self, realized: c_int) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self.handle(), realized) }
    }
}

pub struct wxRadioBox(*mut c_void);
impl _wxRadioBox for wxRadioBox {}
impl _wxControl for wxRadioBox {}
impl _wxWindow for wxRadioBox {}
impl _wxEvtHandler for wxRadioBox {}
impl _wxObject for wxRadioBox { fn handle(&self) -> *mut c_void { **self } }

impl wxRadioBox {
    pub fn from(handle: *mut c_void) -> @wxRadioBox { @wxRadioBox(handle) }
    pub fn null() -> @wxRadioBox { wxRadioBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *mut *mut c_char, _dim: c_int, _stl: c_int) -> @wxRadioBox {
        let _txt = wxT(_txt);
        unsafe { @wxRadioBox(wxRadioBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) }
    }
}

pub trait _wxRadioBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableItem(&self, item: c_int, enable: c_int) {
        unsafe { wxRadioBox_EnableItem(self.handle(), item, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxRadioBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemLabel(&self, item: c_int) -> ~str {
        unsafe { wxString { handle: wxRadioBox_GetItemLabel(self.handle(), item) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfRowsOrCols(&self) -> c_int {
        unsafe { wxRadioBox_GetNumberOfRowsOrCols(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxRadioBox_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStringSelection(&self) -> ~str {
        unsafe { wxString { handle: wxRadioBox_GetStringSelection(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemBitmap<T: _wxBitmap>(&self, item: c_int, bitmap: &T) {
        unsafe { wxRadioBox_SetItemBitmap(self.handle(), item, bitmap.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemLabel(&self, item: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxRadioBox_SetItemLabel(self.handle(), item, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNumberOfRowsOrCols(&self, n: c_int) {
        unsafe { wxRadioBox_SetNumberOfRowsOrCols(self.handle(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, _n: c_int) {
        unsafe { wxRadioBox_SetSelection(self.handle(), _n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStringSelection(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxRadioBox_SetStringSelection(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showItem(&self, item: c_int, show: c_int) {
        unsafe { wxRadioBox_ShowItem(self.handle(), item, show) }
    }
}

pub struct wxRecordSet(*mut c_void);
impl _wxRecordSet for wxRecordSet {}
impl _wxObject for wxRecordSet { fn handle(&self) -> *mut c_void { **self } }

impl wxRecordSet {
    pub fn from(handle: *mut c_void) -> @wxRecordSet { @wxRecordSet(handle) }
    pub fn null() -> @wxRecordSet { wxRecordSet::from(0 as *mut c_void) }
    
}

pub trait _wxRecordSet : _wxObject {
}

pub struct wxRegionIterator(*mut c_void);
impl _wxRegionIterator for wxRegionIterator {}
impl _wxObject for wxRegionIterator { fn handle(&self) -> *mut c_void { **self } }

impl wxRegionIterator {
    pub fn from(handle: *mut c_void) -> @wxRegionIterator { @wxRegionIterator(handle) }
    pub fn null() -> @wxRegionIterator { wxRegionIterator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxRegionIterator {
        unsafe { @wxRegionIterator(wxRegionIterator_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromRegion<T: _wxRegion>(region: &T) -> @wxRegionIterator {
        unsafe { @wxRegionIterator(wxRegionIterator_CreateFromRegion(region.handle())) }
    }
}

pub trait _wxRegionIterator : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxRegionIterator_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxRegionIterator_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxRegionIterator_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxRegionIterator_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn haveRects(&self) -> c_int {
        unsafe { wxRegionIterator_HaveRects(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn next(&self) {
        unsafe { wxRegionIterator_Next(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reset(&self) {
        unsafe { wxRegionIterator_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetToRegion<T: _wxRegion>(&self, region: &T) {
        unsafe { wxRegionIterator_ResetToRegion(self.handle(), region.handle()) }
    }
}

pub struct wxRemotelyScrolledTreeCtrl(*mut c_void);
impl _wxRemotelyScrolledTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxControl for wxRemotelyScrolledTreeCtrl {}
impl _wxWindow for wxRemotelyScrolledTreeCtrl {}
impl _wxEvtHandler for wxRemotelyScrolledTreeCtrl {}
impl _wxObject for wxRemotelyScrolledTreeCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxRemotelyScrolledTreeCtrl {
    pub fn from(handle: *mut c_void) -> @wxRemotelyScrolledTreeCtrl { @wxRemotelyScrolledTreeCtrl(handle) }
    pub fn null() -> @wxRemotelyScrolledTreeCtrl { wxRemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxRemotelyScrolledTreeCtrl : _wxTreeCtrl {
}

pub struct wxSashEvent(*mut c_void);
impl _wxSashEvent for wxSashEvent {}
impl _wxEvent for wxSashEvent {}
impl _wxObject for wxSashEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSashEvent {
    pub fn from(handle: *mut c_void) -> @wxSashEvent { @wxSashEvent(handle) }
    pub fn null() -> @wxSashEvent { wxSashEvent::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(id: c_int, edge: c_int) -> @wxSashEvent {
        unsafe { @wxSashEvent(wxSashEvent_Create(id, edge)) }
    }
}

pub trait _wxSashEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDragRect(&self) -> @wxRect {
        unsafe { @wxRect(wxSashEvent_GetDragRect(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDragStatus(&self) -> c_int {
        unsafe { wxSashEvent_GetDragStatus(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdge(&self) -> c_int {
        unsafe { wxSashEvent_GetEdge(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDragRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxSashEvent_SetDragRect(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDragStatus(&self, status: c_int) {
        unsafe { wxSashEvent_SetDragStatus(self.handle(), status) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEdge(&self, edge: c_int) {
        unsafe { wxSashEvent_SetEdge(self.handle(), edge) }
    }
}

pub struct wxScopedArray(*mut c_void);
impl _wxScopedArray for wxScopedArray { fn handle(&self) -> *mut c_void { **self } }

impl wxScopedArray {
    pub fn from(handle: *mut c_void) -> @wxScopedArray { @wxScopedArray(handle) }
    pub fn null() -> @wxScopedArray { wxScopedArray::from(0 as *mut c_void) }
    
}

pub trait _wxScopedArray {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxScopedPtr(*mut c_void);
impl _wxScopedPtr for wxScopedPtr { fn handle(&self) -> *mut c_void { **self } }

impl wxScopedPtr {
    pub fn from(handle: *mut c_void) -> @wxScopedPtr { @wxScopedPtr(handle) }
    pub fn null() -> @wxScopedPtr { wxScopedPtr::from(0 as *mut c_void) }
    
}

pub trait _wxScopedPtr {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxScrollWinEvent(*mut c_void);
impl _wxScrollWinEvent for wxScrollWinEvent {}
impl _wxEvent for wxScrollWinEvent {}
impl _wxObject for wxScrollWinEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxScrollWinEvent {
    pub fn from(handle: *mut c_void) -> @wxScrollWinEvent { @wxScrollWinEvent(handle) }
    pub fn null() -> @wxScrollWinEvent { wxScrollWinEvent::from(0 as *mut c_void) }
    
}

pub trait _wxScrollWinEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxScrollWinEvent_SetOrientation(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxScrollWinEvent_SetPosition(self.handle(), pos) }
    }
}

pub struct wxSemaphore(*mut c_void);
impl _wxSemaphore for wxSemaphore { fn handle(&self) -> *mut c_void { **self } }

impl wxSemaphore {
    pub fn from(handle: *mut c_void) -> @wxSemaphore { @wxSemaphore(handle) }
    pub fn null() -> @wxSemaphore { wxSemaphore::from(0 as *mut c_void) }
    
}

pub trait _wxSemaphore {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxServer(*mut c_void);
impl _wxServer for wxServer {}
impl _wxServerBase for wxServer {}
impl _wxObject for wxServer { fn handle(&self) -> *mut c_void { **self } }

impl wxServer {
    pub fn from(handle: *mut c_void) -> @wxServer { @wxServer(handle) }
    pub fn null() -> @wxServer { wxServer::from(0 as *mut c_void) }
    
}

pub trait _wxServer : _wxServerBase {
}

pub struct wxSetCursorEvent(*mut c_void);
impl _wxSetCursorEvent for wxSetCursorEvent {}
impl _wxEvent for wxSetCursorEvent {}
impl _wxObject for wxSetCursorEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSetCursorEvent {
    pub fn from(handle: *mut c_void) -> @wxSetCursorEvent { @wxSetCursorEvent(handle) }
    pub fn null() -> @wxSetCursorEvent { wxSetCursorEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSetCursorEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCursor(&self) -> @wxCursor {
        unsafe { @wxCursor(wxSetCursorEvent_GetCursor(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasCursor(&self) -> c_int {
        unsafe { wxSetCursorEvent_HasCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCursor<T: _wxCursor>(&self, cursor: &T) {
        unsafe { wxSetCursorEvent_SetCursor(self.handle(), cursor.handle()) }
    }
}

pub struct wxShowEvent(*mut c_void);
impl _wxShowEvent for wxShowEvent {}
impl _wxEvent for wxShowEvent {}
impl _wxObject for wxShowEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxShowEvent {
    pub fn from(handle: *mut c_void) -> @wxShowEvent { @wxShowEvent(handle) }
    pub fn null() -> @wxShowEvent { wxShowEvent::from(0 as *mut c_void) }
    
}

pub trait _wxShowEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> c_int {
        unsafe { wxShowEvent_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShow(&self, show: c_int) {
        unsafe { wxShowEvent_SetShow(self.handle(), show) }
    }
}

pub struct wxSingleInstanceChecker(*mut c_void);
impl _wxSingleInstanceChecker for wxSingleInstanceChecker { fn handle(&self) -> *mut c_void { **self } }

impl wxSingleInstanceChecker {
    pub fn from(handle: *mut c_void) -> @wxSingleInstanceChecker { @wxSingleInstanceChecker(handle) }
    pub fn null() -> @wxSingleInstanceChecker { wxSingleInstanceChecker::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, name: &str, path: &str) -> c_int {
        let name = wxT(name);
        let path = wxT(path);
        unsafe { wxSingleInstanceChecker_Create(_obj, name.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxSingleInstanceChecker {
        unsafe { @wxSingleInstanceChecker(wxSingleInstanceChecker_CreateDefault()) }
    }
}

pub trait _wxSingleInstanceChecker {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAnotherRunning(&self) -> c_int {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.handle()) }
    }
}

pub struct wxSizeEvent(*mut c_void);
impl _wxSizeEvent for wxSizeEvent {}
impl _wxEvent for wxSizeEvent {}
impl _wxObject for wxSizeEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSizeEvent {
    pub fn from(handle: *mut c_void) -> @wxSizeEvent { @wxSizeEvent(handle) }
    pub fn null() -> @wxSizeEvent { wxSizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSizeEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxSizeEvent_GetSize(self.handle())) }
    }
}

pub struct wxSocketEvent(*mut c_void);
impl _wxSocketEvent for wxSocketEvent {}
impl _wxEvent for wxSocketEvent {}
impl _wxObject for wxSocketEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSocketEvent {
    pub fn from(handle: *mut c_void) -> @wxSocketEvent { @wxSocketEvent(handle) }
    pub fn null() -> @wxSocketEvent { wxSocketEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSocketEvent : _wxEvent {
}

pub struct wxSpinEvent(*mut c_void);
impl _wxSpinEvent for wxSpinEvent {}
impl _wxNotifyEvent for wxSpinEvent {}
impl _wxCommandEvent for wxSpinEvent {}
impl _wxEvent for wxSpinEvent {}
impl _wxObject for wxSpinEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSpinEvent {
    pub fn from(handle: *mut c_void) -> @wxSpinEvent { @wxSpinEvent(handle) }
    pub fn null() -> @wxSpinEvent { wxSpinEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSpinEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxSpinEvent_SetPosition(self.handle(), pos) }
    }
}

pub struct wxSplitterEvent(*mut c_void);
impl _wxSplitterEvent for wxSplitterEvent {}
impl _wxNotifyEvent for wxSplitterEvent {}
impl _wxCommandEvent for wxSplitterEvent {}
impl _wxEvent for wxSplitterEvent {}
impl _wxObject for wxSplitterEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSplitterEvent {
    pub fn from(handle: *mut c_void) -> @wxSplitterEvent { @wxSplitterEvent(handle) }
    pub fn null() -> @wxSplitterEvent { wxSplitterEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSplitterEvent : _wxNotifyEvent {
}

pub struct wxSplitterScrolledWindow(*mut c_void);
impl _wxSplitterScrolledWindow for wxSplitterScrolledWindow {}
impl _wxScrolledWindow for wxSplitterScrolledWindow {}
impl _wxPanel for wxSplitterScrolledWindow {}
impl _wxWindow for wxSplitterScrolledWindow {}
impl _wxEvtHandler for wxSplitterScrolledWindow {}
impl _wxObject for wxSplitterScrolledWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxSplitterScrolledWindow {
    pub fn from(handle: *mut c_void) -> @wxSplitterScrolledWindow { @wxSplitterScrolledWindow(handle) }
    pub fn null() -> @wxSplitterScrolledWindow { wxSplitterScrolledWindow::from(0 as *mut c_void) }
    
}

pub trait _wxSplitterScrolledWindow : _wxScrolledWindow {
}

pub struct wxSplitterWindow(*mut c_void);
impl _wxSplitterWindow for wxSplitterWindow {}
impl _wxWindow for wxSplitterWindow {}
impl _wxEvtHandler for wxSplitterWindow {}
impl _wxObject for wxSplitterWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxSplitterWindow {
    pub fn from(handle: *mut c_void) -> @wxSplitterWindow { @wxSplitterWindow(handle) }
    pub fn null() -> @wxSplitterWindow { wxSplitterWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxSplitterWindow {
        unsafe { @wxSplitterWindow(wxSplitterWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxSplitterWindow : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetBorderSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinimumPaneSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetMinimumPaneSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashPosition(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSplitMode(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSplitMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow1(&self) -> @wxWindow {
        unsafe { @wxWindow(wxSplitterWindow_GetWindow1(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow2(&self) -> @wxWindow {
        unsafe { @wxWindow(wxSplitterWindow_GetWindow2(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSplitterWindow_Initialize(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSplit(&self) -> c_int {
        unsafe { wxSplitterWindow_IsSplit(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, winOld: &T, winNew: &U) -> c_int {
        unsafe { wxSplitterWindow_ReplaceWindow(self.handle(), winOld.handle(), winNew.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorderSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetBorderSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinimumPaneSize(&self, min: c_int) {
        unsafe { wxSplitterWindow_SetMinimumPaneSize(self.handle(), min) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashPosition(&self, position: c_int, redraw: c_int) {
        unsafe { wxSplitterWindow_SetSashPosition(self.handle(), position, redraw) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetSashSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSplitMode(&self, mode: c_int) {
        unsafe { wxSplitterWindow_SetSplitMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn splitHorizontally<T: _wxWindow, U: _wxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitHorizontally(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn splitVertically<T: _wxWindow, U: _wxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitVertically(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unsplit<T: _wxWindow>(&self, toRemove: &T) -> c_int {
        unsafe { wxSplitterWindow_Unsplit(self.handle(), toRemove.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashGravity(&self, gravity: c_double) {
        unsafe { wxSplitterWindow_SetSashGravity(self.handle(), gravity) }
    }
}

pub struct wxStaticBitmap(*mut c_void);
impl _wxStaticBitmap for wxStaticBitmap {}
impl _wxControl for wxStaticBitmap {}
impl _wxWindow for wxStaticBitmap {}
impl _wxEvtHandler for wxStaticBitmap {}
impl _wxObject for wxStaticBitmap { fn handle(&self) -> *mut c_void { **self } }

impl wxStaticBitmap {
    pub fn from(handle: *mut c_void) -> @wxStaticBitmap { @wxStaticBitmap(handle) }
    pub fn null() -> @wxStaticBitmap { wxStaticBitmap::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, bitmap: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticBitmap {
        unsafe { @wxStaticBitmap(wxStaticBitmap_Create(_prt.handle(), _id, bitmap.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxStaticBitmap : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmap<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon<T: _wxIcon>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetIcon(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmap<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxStaticBitmap_SetBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcon<T: _wxIcon>(&self, icon: &T) {
        unsafe { wxStaticBitmap_SetIcon(self.handle(), icon.handle()) }
    }
}

pub struct wxStreamToTextRedirector(*mut c_void);
impl _wxStreamToTextRedirector for wxStreamToTextRedirector { fn handle(&self) -> *mut c_void { **self } }

impl wxStreamToTextRedirector {
    pub fn from(handle: *mut c_void) -> @wxStreamToTextRedirector { @wxStreamToTextRedirector(handle) }
    pub fn null() -> @wxStreamToTextRedirector { wxStreamToTextRedirector::from(0 as *mut c_void) }
    
}

pub trait _wxStreamToTextRedirector {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxStringBuffer(*mut c_void);
impl _wxStringBuffer for wxStringBuffer { fn handle(&self) -> *mut c_void { **self } }

impl wxStringBuffer {
    pub fn from(handle: *mut c_void) -> @wxStringBuffer { @wxStringBuffer(handle) }
    pub fn null() -> @wxStringBuffer { wxStringBuffer::from(0 as *mut c_void) }
    
}

pub trait _wxStringBuffer {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxSysColourChangedEvent(*mut c_void);
impl _wxSysColourChangedEvent for wxSysColourChangedEvent {}
impl _wxEvent for wxSysColourChangedEvent {}
impl _wxObject for wxSysColourChangedEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxSysColourChangedEvent {
    pub fn from(handle: *mut c_void) -> @wxSysColourChangedEvent { @wxSysColourChangedEvent(handle) }
    pub fn null() -> @wxSysColourChangedEvent { wxSysColourChangedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSysColourChangedEvent : _wxEvent {
}

pub struct wxTabCtrl(*mut c_void);
impl _wxTabCtrl for wxTabCtrl {}
impl _wxControl for wxTabCtrl {}
impl _wxWindow for wxTabCtrl {}
impl _wxEvtHandler for wxTabCtrl {}
impl _wxObject for wxTabCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxTabCtrl {
    pub fn from(handle: *mut c_void) -> @wxTabCtrl { @wxTabCtrl(handle) }
    pub fn null() -> @wxTabCtrl { wxTabCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxTabCtrl : _wxControl {
}

pub struct wxTabEvent(*mut c_void);
impl _wxTabEvent for wxTabEvent {}
impl _wxCommandEvent for wxTabEvent {}
impl _wxEvent for wxTabEvent {}
impl _wxObject for wxTabEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxTabEvent {
    pub fn from(handle: *mut c_void) -> @wxTabEvent { @wxTabEvent(handle) }
    pub fn null() -> @wxTabEvent { wxTabEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTabEvent : _wxCommandEvent {
}

pub struct wxTablesInUse(*mut c_void);
impl _wxTablesInUse for wxTablesInUse {}
impl _wxObject for wxTablesInUse { fn handle(&self) -> *mut c_void { **self } }

impl wxTablesInUse {
    pub fn from(handle: *mut c_void) -> @wxTablesInUse { @wxTablesInUse(handle) }
    pub fn null() -> @wxTablesInUse { wxTablesInUse::from(0 as *mut c_void) }
    
}

pub trait _wxTablesInUse : _wxObject {
}

pub struct wxTaskBarIcon(*mut c_void);
impl _wxTaskBarIcon for wxTaskBarIcon {}
impl _wxEvtHandler for wxTaskBarIcon {}
impl _wxObject for wxTaskBarIcon { fn handle(&self) -> *mut c_void { **self } }

impl wxTaskBarIcon {
    pub fn from(handle: *mut c_void) -> @wxTaskBarIcon { @wxTaskBarIcon(handle) }
    pub fn null() -> @wxTaskBarIcon { wxTaskBarIcon::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxTaskBarIcon {
        unsafe { @wxTaskBarIcon(wxTaskBarIcon_Create()) }
    }
}

pub trait _wxTaskBarIcon : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIconInstalled(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn popupMenu<T: _wxMenu>(&self, menu: &T) -> c_int {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeIcon(&self) -> c_int {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcon<T: _wxIcon>(&self, icon: &T, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

pub struct wxTextEntryDialog(*mut c_void);
impl _wxTextEntryDialog for wxTextEntryDialog {}
impl _wxDialog for wxTextEntryDialog {}
impl _wxTopLevelWindow for wxTextEntryDialog {}
impl _wxWindow for wxTextEntryDialog {}
impl _wxEvtHandler for wxTextEntryDialog {}
impl _wxObject for wxTextEntryDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxTextEntryDialog {
    pub fn from(handle: *mut c_void) -> @wxTextEntryDialog { @wxTextEntryDialog(handle) }
    pub fn null() -> @wxTextEntryDialog { wxTextEntryDialog::from(0 as *mut c_void) }
    
}

pub trait _wxTextEntryDialog : _wxDialog {
}

pub struct wxThinSplitterWindow(*mut c_void);
impl _wxThinSplitterWindow for wxThinSplitterWindow {}
impl _wxSplitterWindow for wxThinSplitterWindow {}
impl _wxWindow for wxThinSplitterWindow {}
impl _wxEvtHandler for wxThinSplitterWindow {}
impl _wxObject for wxThinSplitterWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxThinSplitterWindow {
    pub fn from(handle: *mut c_void) -> @wxThinSplitterWindow { @wxThinSplitterWindow(handle) }
    pub fn null() -> @wxThinSplitterWindow { wxThinSplitterWindow::from(0 as *mut c_void) }
    
}

pub trait _wxThinSplitterWindow : _wxSplitterWindow {
}

pub struct wxTimerBase(*mut c_void);
impl _wxTimerBase for wxTimerBase {}
impl _wxObject for wxTimerBase { fn handle(&self) -> *mut c_void { **self } }

impl wxTimerBase {
    pub fn from(handle: *mut c_void) -> @wxTimerBase { @wxTimerBase(handle) }
    pub fn null() -> @wxTimerBase { wxTimerBase::from(0 as *mut c_void) }
    
}

pub trait _wxTimerBase : _wxObject {
}

pub struct wxTimerEvent(*mut c_void);
impl _wxTimerEvent for wxTimerEvent {}
impl _wxEvent for wxTimerEvent {}
impl _wxObject for wxTimerEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxTimerEvent {
    pub fn from(handle: *mut c_void) -> @wxTimerEvent { @wxTimerEvent(handle) }
    pub fn null() -> @wxTimerEvent { wxTimerEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTimerEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.handle()) }
    }
}

pub struct wxTimerEx(*mut c_void);
impl _wxTimerEx for wxTimerEx {}
impl _wxTimer for wxTimerEx {}
impl _wxObject for wxTimerEx { fn handle(&self) -> *mut c_void { **self } }

impl wxTimerEx {
    pub fn from(handle: *mut c_void) -> @wxTimerEx { @wxTimerEx(handle) }
    pub fn null() -> @wxTimerEx { wxTimerEx::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxTimerEx {
        unsafe { @wxTimerEx(wxTimerEx_Create()) }
    }
}

pub trait _wxTimerEx : _wxTimer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn connect<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxTimerEx_Connect(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClosure(&self) -> @wxClosure {
        unsafe { @wxClosure(wxTimerEx_GetClosure(self.handle())) }
    }
}

pub struct wxTimerRunner(*mut c_void);
impl _wxTimerRunner for wxTimerRunner { fn handle(&self) -> *mut c_void { **self } }

impl wxTimerRunner {
    pub fn from(handle: *mut c_void) -> @wxTimerRunner { @wxTimerRunner(handle) }
    pub fn null() -> @wxTimerRunner { wxTimerRunner::from(0 as *mut c_void) }
    
}

pub trait _wxTimerRunner {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxTipProvider(*mut c_void);
impl _wxTipProvider for wxTipProvider { fn handle(&self) -> *mut c_void { **self } }

impl wxTipProvider {
    pub fn from(handle: *mut c_void) -> @wxTipProvider { @wxTipProvider(handle) }
    pub fn null() -> @wxTipProvider { wxTipProvider::from(0 as *mut c_void) }
    
}

pub trait _wxTipProvider {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxTipWindow(*mut c_void);
impl _wxTipWindow for wxTipWindow {}
impl _wxPopupTransientWindow for wxTipWindow {}
impl _wxPopupWindow for wxTipWindow {}
impl _wxWindow for wxTipWindow {}
impl _wxEvtHandler for wxTipWindow {}
impl _wxObject for wxTipWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxTipWindow {
    pub fn from(handle: *mut c_void) -> @wxTipWindow { @wxTipWindow(handle) }
    pub fn null() -> @wxTipWindow { wxTipWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, text: &str, maxLength: c_int) -> @wxTipWindow {
        let text = wxT(text);
        unsafe { @wxTipWindow(wxTipWindow_Create(parent.handle(), text.handle(), maxLength)) }
    }
}

pub trait _wxTipWindow : _wxPopupTransientWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTipWindowPtr(&self, windowPtr: *mut c_void) {
        unsafe { wxTipWindow_SetTipWindowPtr(self.handle(), windowPtr) }
    }
}

pub struct wxToolTip(*mut c_void);
impl _wxToolTip for wxToolTip {}
impl _wxObject for wxToolTip { fn handle(&self) -> *mut c_void { **self } }

impl wxToolTip {
    pub fn from(handle: *mut c_void) -> @wxToolTip { @wxToolTip(handle) }
    pub fn null() -> @wxToolTip { wxToolTip::from(0 as *mut c_void) }
    
}

pub trait _wxToolTip : _wxObject {
}

pub struct wxToolWindow(*mut c_void);
impl _wxToolWindow for wxToolWindow {}
impl _wxFrame for wxToolWindow {}
impl _wxTopLevelWindow for wxToolWindow {}
impl _wxWindow for wxToolWindow {}
impl _wxEvtHandler for wxToolWindow {}
impl _wxObject for wxToolWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxToolWindow {
    pub fn from(handle: *mut c_void) -> @wxToolWindow { @wxToolWindow(handle) }
    pub fn null() -> @wxToolWindow { wxToolWindow::from(0 as *mut c_void) }
    
}

pub trait _wxToolWindow : _wxFrame {
}

pub struct wxTreeCompanionWindow(*mut c_void);
impl _wxTreeCompanionWindow for wxTreeCompanionWindow {}
impl _wxWindow for wxTreeCompanionWindow {}
impl _wxEvtHandler for wxTreeCompanionWindow {}
impl _wxObject for wxTreeCompanionWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeCompanionWindow {
    pub fn from(handle: *mut c_void) -> @wxTreeCompanionWindow { @wxTreeCompanionWindow(handle) }
    pub fn null() -> @wxTreeCompanionWindow { wxTreeCompanionWindow::from(0 as *mut c_void) }
    
}

pub trait _wxTreeCompanionWindow : _wxWindow {
}

pub struct wxTreeLayout(*mut c_void);
impl _wxTreeLayout for wxTreeLayout {}
impl _wxObject for wxTreeLayout { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeLayout {
    pub fn from(handle: *mut c_void) -> @wxTreeLayout { @wxTreeLayout(handle) }
    pub fn null() -> @wxTreeLayout { wxTreeLayout::from(0 as *mut c_void) }
    
}

pub trait _wxTreeLayout : _wxObject {
}

pub struct wxTreeLayoutStored(*mut c_void);
impl _wxTreeLayoutStored for wxTreeLayoutStored {}
impl _wxTreeLayout for wxTreeLayoutStored {}
impl _wxObject for wxTreeLayoutStored { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeLayoutStored {
    pub fn from(handle: *mut c_void) -> @wxTreeLayoutStored { @wxTreeLayoutStored(handle) }
    pub fn null() -> @wxTreeLayoutStored { wxTreeLayoutStored::from(0 as *mut c_void) }
    
}

pub trait _wxTreeLayoutStored : _wxTreeLayout {
}

pub struct wxVariantData(*mut c_void);
impl _wxVariantData for wxVariantData {}
impl _wxObject for wxVariantData { fn handle(&self) -> *mut c_void { **self } }

impl wxVariantData {
    pub fn from(handle: *mut c_void) -> @wxVariantData { @wxVariantData(handle) }
    pub fn null() -> @wxVariantData { wxVariantData::from(0 as *mut c_void) }
    
}

pub trait _wxVariantData : _wxObject {
}

pub struct wxSound(*mut c_void);
impl _wxSound for wxSound {}
impl _wxEvtHandler for wxSound {}
impl _wxObject for wxSound { fn handle(&self) -> *mut c_void { **self } }

impl wxSound {
    pub fn from(handle: *mut c_void) -> @wxSound { @wxSound(handle) }
    pub fn null() -> @wxSound { wxSound::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(fileName: &str, isResource: c_int) -> @wxSound {
        let fileName = wxT(fileName);
        unsafe { @wxSound(wxSound_Create(fileName.handle(), isResource)) }
    }
}

pub trait _wxSound : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxSound_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play(&self, flag: c_int) -> c_int {
        unsafe { wxSound_Play(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) {
        unsafe { wxSound_Stop(self.handle()) }
    }
}

pub struct wxWizardEvent(*mut c_void);
impl _wxWizardEvent for wxWizardEvent {}
impl _wxNotifyEvent for wxWizardEvent {}
impl _wxCommandEvent for wxWizardEvent {}
impl _wxEvent for wxWizardEvent {}
impl _wxObject for wxWizardEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxWizardEvent {
    pub fn from(handle: *mut c_void) -> @wxWizardEvent { @wxWizardEvent(handle) }
    pub fn null() -> @wxWizardEvent { wxWizardEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWizardEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.handle()) }
    }
}

pub struct wxXmlResource(*mut c_void);
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn handle(&self) -> *mut c_void { **self } }

impl wxXmlResource {
    pub fn from(handle: *mut c_void) -> @wxXmlResource { @wxXmlResource(handle) }
    pub fn null() -> @wxXmlResource { wxXmlResource::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int) -> @wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Create(flags)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFile(filemask: &str, flags: c_int) -> @wxXmlResource {
        let filemask = wxT(filemask);
        unsafe { @wxXmlResource(wxXmlResource_CreateFromFile(filemask.handle(), flags)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> @wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Get()) }
    }
}

pub trait _wxXmlResource : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn attachUnknownControl<T: _wxControl, U: _wxWindow>(&self, control: &T, parent: &U) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.handle(), control.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.handle(), major, minor, release, revision) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDomain(&self) -> ~str {
        unsafe { wxString { handle: wxXmlResource_GetDomain(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxXmlResource_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getXRCID(&self, str_id: &str) -> c_int {
        let str_id = wxT(str_id);
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadBitmap<T: _wxBitmap>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadDialog<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxDialog {
        let name = wxT(name);
        unsafe { @wxDialog(wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFrame<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxFrame {
        let name = wxT(name);
        unsafe { @wxFrame(wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadIcon<T: _wxIcon>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadMenu(&self, name: &str) -> @wxMenu {
        let name = wxT(name);
        unsafe { @wxMenu(wxXmlResource_LoadMenu(self.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadMenuBar<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxMenuBar {
        let name = wxT(name);
        unsafe { @wxMenuBar(wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadPanel<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxPanel {
        let name = wxT(name);
        unsafe { @wxPanel(wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadToolBar<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxToolBar {
        let name = wxT(name);
        unsafe { @wxToolBar(wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self, str_id: &str) -> @wxSizer {
        let str_id = wxT(str_id);
        unsafe { @wxSizer(wxXmlResource_GetSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBoxSizer(&self, str_id: &str) -> @wxBoxSizer {
        let str_id = wxT(str_id);
        unsafe { @wxBoxSizer(wxXmlResource_GetBoxSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBoxSizer(&self, str_id: &str) -> @wxStaticBoxSizer {
        let str_id = wxT(str_id);
        unsafe { @wxStaticBoxSizer(wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGridSizer(&self, str_id: &str) -> @wxGridSizer {
        let str_id = wxT(str_id);
        unsafe { @wxGridSizer(wxXmlResource_GetGridSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlexGridSizer(&self, str_id: &str) -> @wxFlexGridSizer {
        let str_id = wxT(str_id);
        unsafe { @wxFlexGridSizer(wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapButton(&self, str_id: &str) -> @wxBitmapButton {
        let str_id = wxT(str_id);
        unsafe { @wxBitmapButton(wxXmlResource_GetBitmapButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButton(&self, str_id: &str) -> @wxButton {
        let str_id = wxT(str_id);
        unsafe { @wxButton(wxXmlResource_GetButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCalendarCtrl(&self, str_id: &str) -> @wxCalendarCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxCalendarCtrl(wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCheckBox(&self, str_id: &str) -> @wxCheckBox {
        let str_id = wxT(str_id);
        unsafe { @wxCheckBox(wxXmlResource_GetCheckBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCheckListBox(&self, str_id: &str) -> @wxCheckListBox {
        let str_id = wxT(str_id);
        unsafe { @wxCheckListBox(wxXmlResource_GetCheckListBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChoice(&self, str_id: &str) -> @wxChoice {
        let str_id = wxT(str_id);
        unsafe { @wxChoice(wxXmlResource_GetChoice(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getComboBox(&self, str_id: &str) -> @wxComboBox {
        let str_id = wxT(str_id);
        unsafe { @wxComboBox(wxXmlResource_GetComboBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGauge(&self, str_id: &str) -> @wxGauge {
        let str_id = wxT(str_id);
        unsafe { @wxGauge(wxXmlResource_GetGauge(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGrid(&self, str_id: &str) -> @wxGrid {
        let str_id = wxT(str_id);
        unsafe { @wxGrid(wxXmlResource_GetGrid(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHtmlWindow(&self, str_id: &str) -> @wxHtmlWindow {
        let str_id = wxT(str_id);
        unsafe { @wxHtmlWindow(wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getListBox(&self, str_id: &str) -> @wxListBox {
        let str_id = wxT(str_id);
        unsafe { @wxListBox(wxXmlResource_GetListBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getListCtrl(&self, str_id: &str) -> @wxListCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxListCtrl(wxXmlResource_GetListCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMDIChildFrame(&self, str_id: &str) -> @wxMDIChildFrame {
        let str_id = wxT(str_id);
        unsafe { @wxMDIChildFrame(wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMDIParentFrame(&self, str_id: &str) -> @wxMDIParentFrame {
        let str_id = wxT(str_id);
        unsafe { @wxMDIParentFrame(wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenu(&self, str_id: &str) -> @wxMenu {
        let str_id = wxT(str_id);
        unsafe { @wxMenu(wxXmlResource_GetMenu(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuBar(&self, str_id: &str) -> @wxMenuBar {
        let str_id = wxT(str_id);
        unsafe { @wxMenuBar(wxXmlResource_GetMenuBar(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuItem(&self, str_id: &str) -> @wxMenuItem {
        let str_id = wxT(str_id);
        unsafe { @wxMenuItem(wxXmlResource_GetMenuItem(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNotebook(&self, str_id: &str) -> @wxNotebook {
        let str_id = wxT(str_id);
        unsafe { @wxNotebook(wxXmlResource_GetNotebook(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPanel(&self, str_id: &str) -> @wxPanel {
        let str_id = wxT(str_id);
        unsafe { @wxPanel(wxXmlResource_GetPanel(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRadioButton(&self, str_id: &str) -> @wxRadioButton {
        let str_id = wxT(str_id);
        unsafe { @wxRadioButton(wxXmlResource_GetRadioButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRadioBox(&self, str_id: &str) -> @wxRadioBox {
        let str_id = wxT(str_id);
        unsafe { @wxRadioBox(wxXmlResource_GetRadioBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollBar(&self, str_id: &str) -> @wxScrollBar {
        let str_id = wxT(str_id);
        unsafe { @wxScrollBar(wxXmlResource_GetScrollBar(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrolledWindow(&self, str_id: &str) -> @wxScrolledWindow {
        let str_id = wxT(str_id);
        unsafe { @wxScrolledWindow(wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSlider(&self, str_id: &str) -> @wxSlider {
        let str_id = wxT(str_id);
        unsafe { @wxSlider(wxXmlResource_GetSlider(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpinButton(&self, str_id: &str) -> @wxSpinButton {
        let str_id = wxT(str_id);
        unsafe { @wxSpinButton(wxXmlResource_GetSpinButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpinCtrl(&self, str_id: &str) -> @wxSpinCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxSpinCtrl(wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSplitterWindow(&self, str_id: &str) -> @wxSplitterWindow {
        let str_id = wxT(str_id);
        unsafe { @wxSplitterWindow(wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBitmap(&self, str_id: &str) -> @wxStaticBitmap {
        let str_id = wxT(str_id);
        unsafe { @wxStaticBitmap(wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBox(&self, str_id: &str) -> @wxStaticBox {
        let str_id = wxT(str_id);
        unsafe { @wxStaticBox(wxXmlResource_GetStaticBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticLine(&self, str_id: &str) -> @wxStaticLine {
        let str_id = wxT(str_id);
        unsafe { @wxStaticLine(wxXmlResource_GetStaticLine(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticText(&self, str_id: &str) -> @wxStaticText {
        let str_id = wxT(str_id);
        unsafe { @wxStaticText(wxXmlResource_GetStaticText(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextCtrl(&self, str_id: &str) -> @wxTextCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxTextCtrl(wxXmlResource_GetTextCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTreeCtrl(&self, str_id: &str) -> @wxTreeCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxTreeCtrl(wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, res: &_wxXmlResource) -> @wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Set(self.handle(), res.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDomain(&self, domain: &str) {
        let domain = wxT(domain);
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyledTextCtrl(&self, str_id: &str) -> @wxStyledTextCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxStyledTextCtrl(wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle())) }
    }
}

pub struct wxXmlResourceHandler(*mut c_void);
impl _wxXmlResourceHandler for wxXmlResourceHandler {}
impl _wxObject for wxXmlResourceHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxXmlResourceHandler {
    pub fn from(handle: *mut c_void) -> @wxXmlResourceHandler { @wxXmlResourceHandler(handle) }
    pub fn null() -> @wxXmlResourceHandler { wxXmlResourceHandler::from(0 as *mut c_void) }
    
}

pub trait _wxXmlResourceHandler : _wxObject {
}

pub struct wxManagedPtr(*mut c_void);
impl _wxManagedPtr for wxManagedPtr { fn handle(&self) -> *mut c_void { **self } }

impl wxManagedPtr {
    pub fn from(handle: *mut c_void) -> @wxManagedPtr { @wxManagedPtr(handle) }
    pub fn null() -> @wxManagedPtr { wxManagedPtr::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getDeleteFunction() -> *mut c_void {
        unsafe { wxManagedPtr_GetDeleteFunction() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromObject<T: _wxObject>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromObject(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromDateTime<T: _wxDateTime>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromDateTime(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromGridCellCoordsArray<T: _wxGridCellCoordsArray>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromGridCellCoordsArray(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromBitmap(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromIcon<T: _wxIcon>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromIcon(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBrush<T: _wxBrush>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromBrush(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromColour<T: _wxColour>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromColour(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromCursor<T: _wxCursor>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromCursor(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFont<T: _wxFont>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromFont(obj.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromPen<T: _wxPen>(obj: &T) -> @wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromPen(obj.handle())) }
    }
}

pub trait _wxManagedPtr {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPtr(&self) -> *mut c_void {
        unsafe { wxManagedPtr_GetPtr(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn noFinalize(&self) {
        unsafe { wxManagedPtr_NoFinalize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn finalize(&self) {
        unsafe { wxManagedPtr_Finalize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxManagedPtr_Delete(self.handle()) }
    }
}

pub struct wxcPrintout(*mut c_void);
impl _wxcPrintout for wxcPrintout {}
impl _wxPrintout for wxcPrintout {}
impl _wxObject for wxcPrintout { fn handle(&self) -> *mut c_void { **self } }

impl wxcPrintout {
    pub fn from(handle: *mut c_void) -> @wxcPrintout { @wxcPrintout(handle) }
    pub fn null() -> @wxcPrintout { wxcPrintout::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(title: &str) -> @wxcPrintout {
        let title = wxT(title);
        unsafe { @wxcPrintout(wxcPrintout_Create(title.handle())) }
    }
}

pub trait _wxcPrintout : _wxPrintout {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEvtHandler(&self) -> @wxcPrintoutHandler {
        unsafe { @wxcPrintoutHandler(wxcPrintout_GetEvtHandler(self.handle())) }
    }
}

pub struct wxcPrintEvent(*mut c_void);
impl _wxcPrintEvent for wxcPrintEvent {}
impl _wxEvent for wxcPrintEvent {}
impl _wxObject for wxcPrintEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxcPrintEvent {
    pub fn from(handle: *mut c_void) -> @wxcPrintEvent { @wxcPrintEvent(handle) }
    pub fn null() -> @wxcPrintEvent { wxcPrintEvent::from(0 as *mut c_void) }
    
}

pub trait _wxcPrintEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintout(&self) -> @wxcPrintout {
        unsafe { @wxcPrintout(wxcPrintEvent_GetPrintout(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEndPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetEndPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getContinue(&self) -> c_int {
        unsafe { wxcPrintEvent_GetContinue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setContinue(&self, cont: c_int) {
        unsafe { wxcPrintEvent_SetContinue(self.handle(), cont) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintEvent_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
}

pub struct wxcPrintoutHandler(*mut c_void);
impl _wxcPrintoutHandler for wxcPrintoutHandler {}
impl _wxEvtHandler for wxcPrintoutHandler {}
impl _wxObject for wxcPrintoutHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxcPrintoutHandler {
    pub fn from(handle: *mut c_void) -> @wxcPrintoutHandler { @wxcPrintoutHandler(handle) }
    pub fn null() -> @wxcPrintoutHandler { wxcPrintoutHandler::from(0 as *mut c_void) }
    
}

pub trait _wxcPrintoutHandler : _wxEvtHandler {
}

pub struct wxStyledTextCtrl(*mut c_void);
impl _wxStyledTextCtrl for wxStyledTextCtrl {}
impl _wxControl for wxStyledTextCtrl {}
impl _wxWindow for wxStyledTextCtrl {}
impl _wxEvtHandler for wxStyledTextCtrl {}
impl _wxObject for wxStyledTextCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxStyledTextCtrl {
    pub fn from(handle: *mut c_void) -> @wxStyledTextCtrl { @wxStyledTextCtrl(handle) }
    pub fn null() -> @wxStyledTextCtrl { wxStyledTextCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> @wxStyledTextCtrl {
        let _txt = wxT(_txt);
        unsafe { @wxStyledTextCtrl(wxStyledTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, style)) }
    }
}

pub trait _wxStyledTextCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_AddText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addStyledText<T: _wxMemoryBuffer>(&self, data: &T) {
        unsafe { wxStyledTextCtrl_AddStyledText(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertText(&self, pos: c_int, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_InsertText(self.handle(), pos, text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearAll(&self) {
        unsafe { wxStyledTextCtrl_ClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearDocumentStyle(&self) {
        unsafe { wxStyledTextCtrl_ClearDocumentStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLength(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLength(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharAt(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetCharAt(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPos(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentPos(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAnchor(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetAnchor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyleAt(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetStyleAt(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn redo(&self) {
        unsafe { wxStyledTextCtrl_Redo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUndoCollection(&self, collectUndo: c_int) {
        unsafe { wxStyledTextCtrl_SetUndoCollection(self.handle(), collectUndo) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectAll(&self) {
        unsafe { wxStyledTextCtrl_SelectAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSavePoint(&self) {
        unsafe { wxStyledTextCtrl_SetSavePoint(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canRedo(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CanRedo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerLineFromHandle(&self, handle: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerLineFromHandle(self.handle(), handle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerDeleteHandle(&self, handle: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteHandle(self.handle(), handle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUndoCollection(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUndoCollection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getViewWhiteSpace(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetViewWhiteSpace(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setViewWhiteSpace(&self, viewWS: c_int) {
        unsafe { wxStyledTextCtrl_SetViewWhiteSpace(self.handle(), viewWS) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionFromPoint(&self, pt_x: c_int, pt_y: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromPoint(self.handle(), pt_x, pt_y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionFromPointClose(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromPointClose(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn gotoLine(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_GotoLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn gotoPos(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_GotoPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAnchor(&self, posAnchor: c_int) {
        unsafe { wxStyledTextCtrl_SetAnchor(self.handle(), posAnchor) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEndStyled(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEndStyled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertEOLs(&self, eolMode: c_int) {
        unsafe { wxStyledTextCtrl_ConvertEOLs(self.handle(), eolMode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEOLMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEOLMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEOLMode(&self, eolMode: c_int) {
        unsafe { wxStyledTextCtrl_SetEOLMode(self.handle(), eolMode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startStyling(&self, pos: c_int, mask: c_int) {
        unsafe { wxStyledTextCtrl_StartStyling(self.handle(), pos, mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyling(&self, length: c_int, style: c_int) {
        unsafe { wxStyledTextCtrl_SetStyling(self.handle(), length, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBufferedDraw(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetBufferedDraw(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBufferedDraw(&self, buffered: c_int) {
        unsafe { wxStyledTextCtrl_SetBufferedDraw(self.handle(), buffered) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTabWidth(&self, tabWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetTabWidth(self.handle(), tabWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTabWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTabWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCodePage(&self, codePage: c_int) {
        unsafe { wxStyledTextCtrl_SetCodePage(self.handle(), codePage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerDefine(&self, markerNumber: c_int, markerSymbol: c_int, foreground_r: uint8_t, foreground_g: uint8_t, foreground_b: uint8_t, background_r: uint8_t, background_g: uint8_t, background_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerDefine(self.handle(), markerNumber, markerSymbol, foreground_r, foreground_g, foreground_b, background_r, background_g, background_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerSetForeground(&self, markerNumber: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerSetForeground(self.handle(), markerNumber, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerSetBackground(&self, markerNumber: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerSetBackground(self.handle(), markerNumber, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerAdd(&self, line: c_int, markerNumber: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerAdd(self.handle(), line, markerNumber) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerDelete(&self, line: c_int, markerNumber: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDelete(self.handle(), line, markerNumber) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerDeleteAll(&self, markerNumber: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteAll(self.handle(), markerNumber) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerGet(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerGet(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerNext(&self, lineStart: c_int, markerMask: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerNext(self.handle(), lineStart, markerMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerPrevious(&self, lineStart: c_int, markerMask: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerPrevious(self.handle(), lineStart, markerMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn markerDefineBitmap<T: _wxBitmap>(&self, markerNumber: c_int, bmp: &T) {
        unsafe { wxStyledTextCtrl_MarkerDefineBitmap(self.handle(), markerNumber, bmp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginType(&self, margin: c_int, marginType: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginType(self.handle(), margin, marginType) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginType(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginType(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginWidth(&self, margin: c_int, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginWidth(self.handle(), margin, pixelWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginWidth(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginWidth(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginMask(&self, margin: c_int, mask: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginMask(self.handle(), margin, mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginMask(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginMask(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginSensitive(&self, margin: c_int, sensitive: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginSensitive(self.handle(), margin, sensitive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginSensitive(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginSensitive(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleClearAll(&self) {
        unsafe { wxStyledTextCtrl_StyleClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetForeground(&self, style: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_StyleSetForeground(self.handle(), style, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetBackground(&self, style: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_StyleSetBackground(self.handle(), style, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetBold(&self, style: c_int, bold: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetBold(self.handle(), style, bold) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetItalic(&self, style: c_int, italic: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetItalic(self.handle(), style, italic) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetSize(&self, style: c_int, sizePoints: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetSize(self.handle(), style, sizePoints) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetFaceName(&self, style: c_int, fontName: &str) {
        let fontName = wxT(fontName);
        unsafe { wxStyledTextCtrl_StyleSetFaceName(self.handle(), style, fontName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetEOLFilled(&self, style: c_int, filled: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetEOLFilled(self.handle(), style, filled) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleResetDefault(&self) {
        unsafe { wxStyledTextCtrl_StyleResetDefault(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetUnderline(&self, style: c_int, underline: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetUnderline(self.handle(), style, underline) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetCase(&self, style: c_int, caseForce: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetCase(self.handle(), style, caseForce) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetCharacterSet(&self, style: c_int, characterSet: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetCharacterSet(self.handle(), style, characterSet) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetHotSpot(&self, style: c_int, hotspot: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetHotSpot(self.handle(), style, hotspot) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelForeground(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelBackground(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaretForeground(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretForeground(self.handle(), fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cmdKeyAssign(&self, key: c_int, modifiers: c_int, cmd: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyAssign(self.handle(), key, modifiers, cmd) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cmdKeyClear(&self, key: c_int, modifiers: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyClear(self.handle(), key, modifiers) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cmdKeyClearAll(&self) {
        unsafe { wxStyledTextCtrl_CmdKeyClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyleBytes(&self, length: c_int, styleBytes: *mut c_char) {
        unsafe { wxStyledTextCtrl_SetStyleBytes(self.handle(), length, styleBytes) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetVisible(&self, style: c_int, visible: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetVisible(self.handle(), style, visible) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaretPeriod(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretPeriod(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaretPeriod(&self, periodMilliseconds: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretPeriod(self.handle(), periodMilliseconds) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWordChars(&self, characters: &str) {
        let characters = wxT(characters);
        unsafe { wxStyledTextCtrl_SetWordChars(self.handle(), characters.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginUndoAction(&self) {
        unsafe { wxStyledTextCtrl_BeginUndoAction(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endUndoAction(&self) {
        unsafe { wxStyledTextCtrl_EndUndoAction(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn indicatorSetStyle(&self, indic: c_int, style: c_int) {
        unsafe { wxStyledTextCtrl_IndicatorSetStyle(self.handle(), indic, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn indicatorGetStyle(&self, indic: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_IndicatorGetStyle(self.handle(), indic) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn indicatorSetForeground(&self, indic: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_IndicatorSetForeground(self.handle(), indic, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWhitespaceForeground(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWhitespaceBackground(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyleBits(&self, bits: c_int) {
        unsafe { wxStyledTextCtrl_SetStyleBits(self.handle(), bits) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyleBits(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetStyleBits(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLineState(&self, line: c_int, state: c_int) {
        unsafe { wxStyledTextCtrl_SetLineState(self.handle(), line, state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineState(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineState(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxLineState(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMaxLineState(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaretLineVisible(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretLineVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaretLineVisible(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretLineVisible(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetChangeable(&self, style: c_int, changeable: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetChangeable(self.handle(), style, changeable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompShow(&self, lenEntered: c_int, itemList: &str) {
        let itemList = wxT(itemList);
        unsafe { wxStyledTextCtrl_AutoCompShow(self.handle(), lenEntered, itemList.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompCancel(&self) {
        unsafe { wxStyledTextCtrl_AutoCompCancel(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompActive(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompActive(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompPosStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompPosStart(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompComplete(&self) {
        unsafe { wxStyledTextCtrl_AutoCompComplete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompStops(&self, characterSet: &str) {
        let characterSet = wxT(characterSet);
        unsafe { wxStyledTextCtrl_AutoCompStops(self.handle(), characterSet.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetSeparator(&self, separatorCharacter: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetSeparator(self.handle(), separatorCharacter) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetSeparator(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSelect(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_AutoCompSelect(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetCancelAtStart(&self, cancel: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetCancelAtStart(self.handle(), cancel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetCancelAtStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetCancelAtStart(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetFillUps(&self, characterSet: &str) {
        let characterSet = wxT(characterSet);
        unsafe { wxStyledTextCtrl_AutoCompSetFillUps(self.handle(), characterSet.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetChooseSingle(&self, chooseSingle: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetChooseSingle(self.handle(), chooseSingle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetChooseSingle(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetChooseSingle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetIgnoreCase(&self, ignoreCase: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetIgnoreCase(self.handle(), ignoreCase) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetIgnoreCase(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetIgnoreCase(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn userListShow(&self, listType: c_int, itemList: &str) {
        let itemList = wxT(itemList);
        unsafe { wxStyledTextCtrl_UserListShow(self.handle(), listType, itemList.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetAutoHide(&self, autoHide: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetAutoHide(self.handle(), autoHide) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetAutoHide(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetAutoHide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetDropRestOfWord(&self, dropRestOfWord: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetDropRestOfWord(self.handle(), dropRestOfWord) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetDropRestOfWord(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetDropRestOfWord(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn registerImage<T: _wxBitmap>(&self, type_: c_int, bmp: &T) {
        unsafe { wxStyledTextCtrl_RegisterImage(self.handle(), type_, bmp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearRegisteredImages(&self) {
        unsafe { wxStyledTextCtrl_ClearRegisteredImages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetTypeSeparator(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetTypeSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetTypeSeparator(&self, separatorCharacter: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetTypeSeparator(self.handle(), separatorCharacter) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIndent(&self, indentSize: c_int) {
        unsafe { wxStyledTextCtrl_SetIndent(self.handle(), indentSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndent(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUseTabs(&self, useTabs: c_int) {
        unsafe { wxStyledTextCtrl_SetUseTabs(self.handle(), useTabs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUseTabs(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUseTabs(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLineIndentation(&self, line: c_int, indentSize: c_int) {
        unsafe { wxStyledTextCtrl_SetLineIndentation(self.handle(), line, indentSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineIndentation(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineIndentation(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineIndentPosition(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineIndentPosition(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetColumn(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUseHorizontalScrollBar(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetUseHorizontalScrollBar(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUseHorizontalScrollBar(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUseHorizontalScrollBar(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIndentationGuides(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetIndentationGuides(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndentationGuides(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetIndentationGuides(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHighlightGuide(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_SetHighlightGuide(self.handle(), column) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHighlightGuide(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetHighlightGuide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineEndPosition(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineEndPosition(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCodePage(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCodePage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReadOnly(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetReadOnly(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentPos(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetCurrentPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelectionStart(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetSelectionStart(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectionStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSelectionStart(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelectionEnd(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetSelectionEnd(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectionEnd(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSelectionEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintMagnification(&self, magnification: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintMagnification(self.handle(), magnification) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintMagnification(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintMagnification(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintColourMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintColourMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintColourMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintColourMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findText(&self, minPos: c_int, maxPos: c_int, text: &str, flags: c_int) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_FindText(self.handle(), minPos, maxPos, text.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatRange<T: _wxDC, U: _wxDC, V: _wxRect, W: _wxRect>(&self, doDraw: c_int, startPos: c_int, endPos: c_int, draw: &T, target: &U, renderRect: &V, pageRect: &W) -> c_int {
        unsafe { wxStyledTextCtrl_FormatRange(self.handle(), doDraw, startPos, endPos, draw.handle(), target.handle(), renderRect.handle(), pageRect.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstVisibleLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetFirstVisibleLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineCount(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginLeft(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginLeft(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginLeft(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginLeft(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginRight(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginRight(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginRight(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginRight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getModify(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetModify(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_SetSelection(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideSelection(&self, normal: c_int) {
        unsafe { wxStyledTextCtrl_HideSelection(self.handle(), normal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineFromPosition(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_LineFromPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionFromLine(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineScroll(&self, columns: c_int, lines: c_int) {
        unsafe { wxStyledTextCtrl_LineScroll(self.handle(), columns, lines) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureCaretVisible(&self) {
        unsafe { wxStyledTextCtrl_EnsureCaretVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceSelection(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_ReplaceSelection(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReadOnly(&self, readOnly: c_int) {
        unsafe { wxStyledTextCtrl_SetReadOnly(self.handle(), readOnly) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canPaste(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CanPaste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canUndo(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CanUndo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn emptyUndoBuffer(&self) {
        unsafe { wxStyledTextCtrl_EmptyUndoBuffer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn undo(&self) {
        unsafe { wxStyledTextCtrl_Undo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cut(&self) {
        unsafe { wxStyledTextCtrl_Cut(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self) {
        unsafe { wxStyledTextCtrl_Copy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste(&self) {
        unsafe { wxStyledTextCtrl_Paste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxStyledTextCtrl_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextLength(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTextLength(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOvertype(&self, overtype: c_int) {
        unsafe { wxStyledTextCtrl_SetOvertype(self.handle(), overtype) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOvertype(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetOvertype(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaretWidth(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretWidth(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaretWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTargetStart(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetTargetStart(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTargetStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTargetStart(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTargetEnd(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetTargetEnd(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTargetEnd(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTargetEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceTarget(&self, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_ReplaceTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceTargetRE(&self, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn searchInTarget(&self, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_SearchInTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSearchFlags(&self, flags: c_int) {
        unsafe { wxStyledTextCtrl_SetSearchFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSearchFlags(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSearchFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipShow(&self, pos: c_int, definition: &str) {
        let definition = wxT(definition);
        unsafe { wxStyledTextCtrl_CallTipShow(self.handle(), pos, definition.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipCancel(&self) {
        unsafe { wxStyledTextCtrl_CallTipCancel(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipActive(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CallTipActive(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipPosAtStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CallTipPosAtStart(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipSetHighlight(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_CallTipSetHighlight(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipSetBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipSetForeground(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetForeground(self.handle(), fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn callTipSetForegroundHighlight(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetForegroundHighlight(self.handle(), fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn visibleFromDocLine(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_VisibleFromDocLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn docLineFromVisible(&self, lineDisplay: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_DocLineFromVisible(self.handle(), lineDisplay) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldLevel(&self, line: c_int, level: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldLevel(self.handle(), line, level) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFoldLevel(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldLevel(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastChild(&self, line: c_int, level: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLastChild(self.handle(), line, level) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFoldParent(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldParent(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showLines(&self, lineStart: c_int, lineEnd: c_int) {
        unsafe { wxStyledTextCtrl_ShowLines(self.handle(), lineStart, lineEnd) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideLines(&self, lineStart: c_int, lineEnd: c_int) {
        unsafe { wxStyledTextCtrl_HideLines(self.handle(), lineStart, lineEnd) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineVisible(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineVisible(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldExpanded(&self, line: c_int, expanded: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldExpanded(self.handle(), line, expanded) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFoldExpanded(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldExpanded(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toggleFold(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_ToggleFold(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisible(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_EnsureVisible(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldFlags(&self, flags: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisibleEnforcePolicy(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_EnsureVisibleEnforcePolicy(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTabIndents(&self, tabIndents: c_int) {
        unsafe { wxStyledTextCtrl_SetTabIndents(self.handle(), tabIndents) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTabIndents(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTabIndents(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackSpaceUnIndents(&self, bsUnIndents: c_int) {
        unsafe { wxStyledTextCtrl_SetBackSpaceUnIndents(self.handle(), bsUnIndents) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackSpaceUnIndents(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetBackSpaceUnIndents(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMouseDwellTime(&self, periodMilliseconds: c_int) {
        unsafe { wxStyledTextCtrl_SetMouseDwellTime(self.handle(), periodMilliseconds) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMouseDwellTime(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMouseDwellTime(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordStartPosition(&self, pos: c_int, onlyWordCharacters: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_WordStartPosition(self.handle(), pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordEndPosition(&self, pos: c_int, onlyWordCharacters: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_WordEndPosition(self.handle(), pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWrapMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetWrapMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWrapMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetWrapMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLayoutCache(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetLayoutCache(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLayoutCache(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLayoutCache(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollWidth(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetScrollWidth(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetScrollWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn textWidth(&self, style: c_int, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_TextWidth(self.handle(), style, text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEndAtLastLine(&self, endAtLastLine: c_int) {
        unsafe { wxStyledTextCtrl_SetEndAtLastLine(self.handle(), endAtLastLine) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEndAtLastLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEndAtLastLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn textHeight(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_TextHeight(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUseVerticalScrollBar(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetUseVerticalScrollBar(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUseVerticalScrollBar(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUseVerticalScrollBar(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_AppendText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTwoPhaseDraw(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTwoPhaseDraw(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTwoPhaseDraw(&self, twoPhase: c_int) {
        unsafe { wxStyledTextCtrl_SetTwoPhaseDraw(self.handle(), twoPhase) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn targetFromSelection(&self) {
        unsafe { wxStyledTextCtrl_TargetFromSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn linesJoin(&self) {
        unsafe { wxStyledTextCtrl_LinesJoin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn linesSplit(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_LinesSplit(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldMarginColour(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginColour(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldMarginHiColour(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginHiColour(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineDuplicate(&self) {
        unsafe { wxStyledTextCtrl_LineDuplicate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn homeDisplay(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplay(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn homeDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplayExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineEndDisplay(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplay(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineEndDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplayExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineCopy(&self) {
        unsafe { wxStyledTextCtrl_LineCopy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCaretInsideView(&self) {
        unsafe { wxStyledTextCtrl_MoveCaretInsideView(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lineLength(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_LineLength(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn braceHighlight(&self, pos1: c_int, pos2: c_int) {
        unsafe { wxStyledTextCtrl_BraceHighlight(self.handle(), pos1, pos2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn braceBadLight(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_BraceBadLight(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn braceMatch(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_BraceMatch(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getViewEOL(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetViewEOL(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setViewEOL(&self, visible: c_int) {
        unsafe { wxStyledTextCtrl_SetViewEOL(self.handle(), visible) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDocPointer<T: _wxSTCDoc>(&self, docPointer: &T) {
        unsafe { wxStyledTextCtrl_SetDocPointer(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setModEventMask(&self, mask: c_int) {
        unsafe { wxStyledTextCtrl_SetModEventMask(self.handle(), mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdgeColumn(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEdgeColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEdgeColumn(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_SetEdgeColumn(self.handle(), column) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdgeMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEdgeMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEdgeMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetEdgeMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEdgeColour(&self, edgeColour_r: uint8_t, edgeColour_g: uint8_t, edgeColour_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetEdgeColour(self.handle(), edgeColour_r, edgeColour_g, edgeColour_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn searchAnchor(&self) {
        unsafe { wxStyledTextCtrl_SearchAnchor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn searchNext(&self, flags: c_int, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_SearchNext(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn searchPrev(&self, flags: c_int, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_SearchPrev(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn linesOnScreen(&self) -> c_int {
        unsafe { wxStyledTextCtrl_LinesOnScreen(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn usePopUp(&self, allowPopUp: c_int) {
        unsafe { wxStyledTextCtrl_UsePopUp(self.handle(), allowPopUp) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectionIsRectangle(&self) -> c_int {
        unsafe { wxStyledTextCtrl_SelectionIsRectangle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setZoom(&self, zoom: c_int) {
        unsafe { wxStyledTextCtrl_SetZoom(self.handle(), zoom) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getZoom(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetZoom(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRefDocument<T: _wxSTCDoc>(&self, docPointer: &T) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn releaseDocument<T: _wxSTCDoc>(&self, docPointer: &T) {
        unsafe { wxStyledTextCtrl_ReleaseDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getModEventMask(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetModEventMask(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSTCFocus(&self, focus: c_int) {
        unsafe { wxStyledTextCtrl_SetSTCFocus(self.handle(), focus) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSTCFocus(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSTCFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatus(&self, statusCode: c_int) {
        unsafe { wxStyledTextCtrl_SetStatus(self.handle(), statusCode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStatus(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetStatus(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMouseDownCaptures(&self, captures: c_int) {
        unsafe { wxStyledTextCtrl_SetMouseDownCaptures(self.handle(), captures) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMouseDownCaptures(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMouseDownCaptures(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSTCCursor(&self, cursorType: c_int) {
        unsafe { wxStyledTextCtrl_SetSTCCursor(self.handle(), cursorType) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSTCCursor(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSTCCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setControlCharSymbol(&self, symbol: c_int) {
        unsafe { wxStyledTextCtrl_SetControlCharSymbol(self.handle(), symbol) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControlCharSymbol(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetControlCharSymbol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordPartLeft(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeft(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordPartLeftExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeftExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordPartRight(&self) {
        unsafe { wxStyledTextCtrl_WordPartRight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordPartRightExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartRightExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVisiblePolicy(&self, visiblePolicy: c_int, visibleSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetVisiblePolicy(self.handle(), visiblePolicy, visibleSlop) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delLineLeft(&self) {
        unsafe { wxStyledTextCtrl_DelLineLeft(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delLineRight(&self) {
        unsafe { wxStyledTextCtrl_DelLineRight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setXOffset(&self, newOffset: c_int) {
        unsafe { wxStyledTextCtrl_SetXOffset(self.handle(), newOffset) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getXOffset(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetXOffset(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn chooseCaretX(&self) {
        unsafe { wxStyledTextCtrl_ChooseCaretX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setXCaretPolicy(&self, caretPolicy: c_int, caretSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetXCaretPolicy(self.handle(), caretPolicy, caretSlop) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setYCaretPolicy(&self, caretPolicy: c_int, caretSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetYCaretPolicy(self.handle(), caretPolicy, caretSlop) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintWrapMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintWrapMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintWrapMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintWrapMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHotspotActiveForeground(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHotspotActiveBackground(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHotspotActiveUnderline(&self, underline: c_int) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveUnderline(self.handle(), underline) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionBefore(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionBefore(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionAfter(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionAfter(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copyRange(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_CopyRange(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copyText(&self, length: c_int, text: &str) {
        let text = wxT(text);
        unsafe { wxStyledTextCtrl_CopyText(self.handle(), length, text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startRecord(&self) {
        unsafe { wxStyledTextCtrl_StartRecord(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stopRecord(&self) {
        unsafe { wxStyledTextCtrl_StopRecord(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLexer(&self, lexer: c_int) {
        unsafe { wxStyledTextCtrl_SetLexer(self.handle(), lexer) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLexer(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLexer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn colourise(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_Colourise(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setProperty(&self, key: &str, value: &str) {
        let key = wxT(key);
        let value = wxT(value);
        unsafe { wxStyledTextCtrl_SetProperty(self.handle(), key.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setKeyWords(&self, keywordSet: c_int, keyWords: &str) {
        let keyWords = wxT(keyWords);
        unsafe { wxStyledTextCtrl_SetKeyWords(self.handle(), keywordSet, keyWords.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLexerLanguage(&self, language: &str) {
        let language = wxT(language);
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self.handle(), language.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetSpec(&self, styleNum: c_int, spec: &str) {
        let spec = wxT(spec);
        unsafe { wxStyledTextCtrl_StyleSetSpec(self.handle(), styleNum, spec.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetFont<T: _wxFont>(&self, styleNum: c_int, font: &T) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self.handle(), styleNum, font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetFontAttr(&self, styleNum: c_int, size: c_int, faceName: &str, bold: c_int, italic: c_int, underline: c_int) {
        let faceName = wxT(faceName);
        unsafe { wxStyledTextCtrl_StyleSetFontAttr(self.handle(), styleNum, size, faceName.handle(), bold, italic, underline) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cmdKeyExecute(&self, cmd: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyExecute(self.handle(), cmd) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargins(&self, left: c_int, right: c_int) {
        unsafe { wxStyledTextCtrl_SetMargins(self.handle(), left, right) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self, startPos: *mut c_int, endPos: *mut c_int) {
        unsafe { wxStyledTextCtrl_GetSelection(self.handle(), startPos, endPos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollToLine(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_ScrollToLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollToColumn(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_ScrollToColumn(self.handle(), column) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVScrollBar<T: _wxScrollBar>(&self, bar: &T) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHScrollBar<T: _wxScrollBar>(&self, bar: &T) {
        unsafe { wxStyledTextCtrl_SetHScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastKeydownProcessed(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLastKeydownProcessed(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLastKeydownProcessed(&self, val: c_int) {
        unsafe { wxStyledTextCtrl_SetLastKeydownProcessed(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, filename: &str) -> c_int {
        let filename = wxT(filename);
        unsafe { wxStyledTextCtrl_SaveFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, filename: &str) -> c_int {
        let filename = wxT(filename);
        unsafe { wxStyledTextCtrl_LoadFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn indicatorGetForeground(&self, indic: c_int) -> @wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_IndicatorGetForeground(self.handle(), indic)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaretLineBackground(&self) -> @wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_GetCaretLineBackground(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaretLineBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaretForeground(&self) -> @wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_GetCaretForeground(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLine(&self, line: c_int) -> ~str {
        unsafe { wxString { handle: wxStyledTextCtrl_GetLine(self.handle(), line) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { handle: wxStyledTextCtrl_GetText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextRange(&self, startPos: c_int, endPos: c_int) -> ~str {
        unsafe { wxString { handle: wxStyledTextCtrl_GetTextRange(self.handle(), startPos, endPos) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectedText(&self) -> ~str {
        unsafe { wxString { handle: wxStyledTextCtrl_GetSelectedText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn newDocument(&self) -> @wxSTCDoc {
        unsafe { @wxSTCDoc(wxStyledTextCtrl_CreateDocument(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdgeColour(&self) -> @wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_GetEdgeColour(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDocPointer(&self) -> @wxSTCDoc {
        unsafe { @wxSTCDoc(wxStyledTextCtrl_GetDocPointer(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pointFromPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxStyledTextCtrl_PointFromPosition(self.handle())) }
    }
}

pub struct wxSTCDoc(*mut c_void);
impl _wxSTCDoc for wxSTCDoc { fn handle(&self) -> *mut c_void { **self } }

impl wxSTCDoc {
    pub fn from(handle: *mut c_void) -> @wxSTCDoc { @wxSTCDoc(handle) }
    pub fn null() -> @wxSTCDoc { wxSTCDoc::from(0 as *mut c_void) }
    
}

pub trait _wxSTCDoc {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxStyledTextEvent(*mut c_void);
impl _wxStyledTextEvent for wxStyledTextEvent {}
impl _wxCommandEvent for wxStyledTextEvent {}
impl _wxEvent for wxStyledTextEvent {}
impl _wxObject for wxStyledTextEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxStyledTextEvent {
    pub fn from(handle: *mut c_void) -> @wxStyledTextEvent { @wxStyledTextEvent(handle) }
    pub fn null() -> @wxStyledTextEvent { wxStyledTextEvent::from(0 as *mut c_void) }
    
}

pub trait _wxStyledTextEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKey(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetKey(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getModifiers(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getModificationType(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetModificationType(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLength(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLength(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLinesAdded(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLinesAdded(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLine(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFoldLevelNow(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetFoldLevelNow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFoldLevelPrev(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetFoldLevelPrev(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMargin(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetMargin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMessage(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetMessage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWParam(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetWParam(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLParam(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLParam(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getListType(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetListType(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDragText(&self) -> ~str {
        unsafe { wxString { handle: wxStyledTextEvent_GetDragText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDragAllowMove(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetDragAllowMove(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDragResult(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetDragResult(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getShift(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetShift(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControl(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetControl(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlt(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetAlt(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { handle: wxStyledTextEvent_GetText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clone(&self) -> @wxStyledTextEvent {
        unsafe { @wxStyledTextEvent(wxStyledTextEvent_Clone(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxStyledTextEvent_SetPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setKey(&self, k: c_int) {
        unsafe { wxStyledTextEvent_SetKey(self.handle(), k) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setModifiers(&self, m: c_int) {
        unsafe { wxStyledTextEvent_SetModifiers(self.handle(), m) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setModificationType(&self, t: c_int) {
        unsafe { wxStyledTextEvent_SetModificationType(self.handle(), t) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, t: &str) {
        let t = wxT(t);
        unsafe { wxStyledTextEvent_SetText(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLength(&self, len: c_int) {
        unsafe { wxStyledTextEvent_SetLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLinesAdded(&self, num: c_int) {
        unsafe { wxStyledTextEvent_SetLinesAdded(self.handle(), num) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLine(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetLine(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldLevelNow(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetFoldLevelNow(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldLevelPrev(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetFoldLevelPrev(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargin(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetMargin(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMessage(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetMessage(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWParam(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetWParam(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLParam(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetLParam(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setListType(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetListType(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setX(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetX(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setY(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetY(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDragText(&self, val: &str) {
        let val = wxT(val);
        unsafe { wxStyledTextEvent_SetDragText(self.handle(), val.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDragAllowMove(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetDragAllowMove(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDragResult(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetDragResult(self.handle(), val) }
    }
}

pub struct wxSlider95(*mut c_void);
impl _wxSlider95 for wxSlider95 {}
impl _wxSlider for wxSlider95 {}
impl _wxControl for wxSlider95 {}
impl _wxWindow for wxSlider95 {}
impl _wxEvtHandler for wxSlider95 {}
impl _wxObject for wxSlider95 { fn handle(&self) -> *mut c_void { **self } }

impl wxSlider95 {
    pub fn from(handle: *mut c_void) -> @wxSlider95 { @wxSlider95(handle) }
    pub fn null() -> @wxSlider95 { wxSlider95::from(0 as *mut c_void) }
    
}

pub trait _wxSlider95 : _wxSlider {
}

pub struct wxSliderMSW(*mut c_void);
impl _wxSliderMSW for wxSliderMSW {}
impl _wxSlider for wxSliderMSW {}
impl _wxControl for wxSliderMSW {}
impl _wxWindow for wxSliderMSW {}
impl _wxEvtHandler for wxSliderMSW {}
impl _wxObject for wxSliderMSW { fn handle(&self) -> *mut c_void { **self } }

impl wxSliderMSW {
    pub fn from(handle: *mut c_void) -> @wxSliderMSW { @wxSliderMSW(handle) }
    pub fn null() -> @wxSliderMSW { wxSliderMSW::from(0 as *mut c_void) }
    
}

pub trait _wxSliderMSW : _wxSlider {
}

pub struct wxcTreeItemData(*mut c_void);
impl _wxcTreeItemData for wxcTreeItemData {}
impl _wxTreeItemData for wxcTreeItemData {}
impl _wxClientData for wxcTreeItemData { fn handle(&self) -> *mut c_void { **self } }

impl wxcTreeItemData {
    pub fn from(handle: *mut c_void) -> @wxcTreeItemData { @wxcTreeItemData(handle) }
    pub fn null() -> @wxcTreeItemData { wxcTreeItemData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxClosure>(closure: &T) -> @wxcTreeItemData {
        unsafe { @wxcTreeItemData(wxcTreeItemData_Create(closure.handle())) }
    }
}

pub trait _wxcTreeItemData : _wxTreeItemData {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { @wxClosure(wxcTreeItemData_GetClientClosure(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientClosure<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxcTreeItemData_SetClientClosure(self.handle(), closure.handle()) }
    }
}

pub struct wxcHtmlEvent(*mut c_void);
impl _wxcHtmlEvent for wxcHtmlEvent {}
impl _wxCommandEvent for wxcHtmlEvent {}
impl _wxEvent for wxcHtmlEvent {}
impl _wxObject for wxcHtmlEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxcHtmlEvent {
    pub fn from(handle: *mut c_void) -> @wxcHtmlEvent { @wxcHtmlEvent(handle) }
    pub fn null() -> @wxcHtmlEvent { wxcHtmlEvent::from(0 as *mut c_void) }
    
}

pub trait _wxcHtmlEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMouseEvent(&self) -> @wxMouseEvent {
        unsafe { @wxMouseEvent(wxcHtmlEvent_GetMouseEvent(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHtmlCell(&self) -> @wxHtmlCell {
        unsafe { @wxHtmlCell(wxcHtmlEvent_GetHtmlCell(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHtmlCellId(&self) -> ~str {
        unsafe { wxString { handle: wxcHtmlEvent_GetHtmlCellId(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHref(&self) -> ~str {
        unsafe { wxString { handle: wxcHtmlEvent_GetHref(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTarget(&self) -> ~str {
        unsafe { wxString { handle: wxcHtmlEvent_GetTarget(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxcHtmlEvent_GetLogicalPosition(self.handle())) }
    }
}

