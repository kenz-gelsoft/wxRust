use libc::*;
use _unsafe::*;
use base::*;

/// The wxRust-specific derived class of [wxApp](http://docs.wxwidgets.org/3.0/classwx_app.html).
pub struct RustApp { ptr: *mut c_void }
impl RustAppMethods for RustApp {}
impl AppMethods for RustApp {}
impl EvtHandlerMethods for RustApp {}
impl ObjectMethods for RustApp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustApp {
    pub fn from(ptr: *mut c_void) -> RustApp { RustApp { ptr: ptr } }
    pub fn null() -> RustApp { RustApp::from(0 as *mut c_void) }
    
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    pub fn newLogTarget() -> RustLog {
        unsafe { RustLog::from(ELJApp_CreateLogTarget()) }
    }
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    pub fn displaySize() -> Size {
        unsafe { Size::from(ELJApp_DisplaySize()) }
    }
    pub fn enableTooltips(_enable: c_int) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    pub fn enableTopLevelWindows(_enb: c_int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    pub fn executeProcess<T: ProcessMethods>(_cmd: &str, _snc: c_int, _prc: &T) -> c_int {
        let _cmd = strToString(_cmd);
        unsafe { ELJApp_ExecuteProcess(_cmd.ptr(), _snc, _prc.ptr()) }
    }
    pub fn exit() {
        unsafe { ELJApp_Exit() }
    }
    pub fn exitMainLoop() {
        unsafe { ELJApp_ExitMainLoop() }
    }
    pub fn findWindowById<T: WindowMethods>(_id: c_int, _prt: &T) -> *mut c_void {
        unsafe { ELJApp_FindWindowById(_id, _prt.ptr()) }
    }
    pub fn findWindowByLabel<T: WindowMethods>(_lbl: &str, _prt: &T) -> Window {
        let _lbl = strToString(_lbl);
        unsafe { Window::from(ELJApp_FindWindowByLabel(_lbl.ptr(), _prt.ptr())) }
    }
    pub fn findWindowByName<T: WindowMethods>(_lbl: &str, _prt: &T) -> Window {
        let _lbl = strToString(_lbl);
        unsafe { Window::from(ELJApp_FindWindowByName(_lbl.ptr(), _prt.ptr())) }
    }
    pub fn getApp() -> App {
        unsafe { App::from(ELJApp_GetApp()) }
    }
    pub fn getAppName() -> ~str {
        unsafe { String::from(ELJApp_GetAppName()).to_str() }
    }
    pub fn getClassName() -> ~str {
        unsafe { String::from(ELJApp_GetClassName()).to_str() }
    }
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    pub fn getOsDescription() -> ~str {
        unsafe { String::from(ELJApp_GetOsDescription()).to_str() }
    }
    pub fn getOsVersion(_maj: *mut c_void, _min: *mut c_void) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    pub fn getTopWindow() -> Window {
        unsafe { Window::from(ELJApp_GetTopWindow()) }
    }
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    pub fn getUserHome(_usr: *mut c_void) -> ~str {
        unsafe { String::from(ELJApp_GetUserHome(_usr)).to_str() }
    }
    pub fn getUserId() -> ~str {
        unsafe { String::from(ELJApp_GetUserId()).to_str() }
    }
    pub fn getUserName() -> ~str {
        unsafe { String::from(ELJApp_GetUserName()).to_str() }
    }
    pub fn getVendorName() -> ~str {
        unsafe { String::from(ELJApp_GetVendorName()).to_str() }
    }
    pub fn initAllImageHandlers() {
        unsafe { ELJApp_InitAllImageHandlers() }
    }
    pub fn initialized() -> c_int {
        unsafe { ELJApp_Initialized() }
    }
    pub fn mainLoop() -> c_int {
        unsafe { ELJApp_MainLoop() }
    }
    pub fn mousePosition() -> Point {
        unsafe { Point::from(ELJApp_MousePosition()) }
    }
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    pub fn safeYield<T: WindowMethods>(_win: &T) -> c_int {
        unsafe { ELJApp_SafeYield(_win.ptr()) }
    }
    pub fn setAppName(name: &str) {
        let name = strToString(name);
        unsafe { ELJApp_SetAppName(name.ptr()) }
    }
    pub fn setClassName(name: &str) {
        let name = strToString(name);
        unsafe { ELJApp_SetClassName(name.ptr()) }
    }
    pub fn setExitOnFrameDelete(flag: c_int) {
        unsafe { ELJApp_SetExitOnFrameDelete(flag) }
    }
    pub fn setPrintMode(mode: c_int) {
        unsafe { ELJApp_SetPrintMode(mode) }
    }
    pub fn setTooltipDelay(_ms: c_int) {
        unsafe { ELJApp_SetTooltipDelay(_ms) }
    }
    pub fn setTopWindow<T: WindowMethods>(_wnd: &T) {
        unsafe { ELJApp_SetTopWindow(_wnd.ptr()) }
    }
    pub fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    pub fn setVendorName(name: &str) {
        let name = strToString(name);
        unsafe { ELJApp_SetVendorName(name.ptr()) }
    }
    pub fn sleep(_scs: c_int) {
        unsafe { ELJApp_Sleep(_scs) }
    }
    pub fn milliSleep(_mscs: c_int) {
        unsafe { ELJApp_MilliSleep(_mscs) }
    }
    pub fn yield_() -> c_int {
        unsafe { ELJApp_Yield() }
    }
    pub fn isTerminating() -> c_int {
        unsafe { ELJApp_IsTerminating() }
    }
    pub fn initializeC<T: ClosureMethods>(closure: &T, _argc: c_int, _argv: *mut *mut c_char) {
        unsafe { ELJApp_InitializeC(closure.ptr(), _argc, _argv) }
    }
    pub fn getIdleInterval() -> c_int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    pub fn setIdleInterval(interval: c_int) {
        unsafe { ELJApp_SetIdleInterval(interval) }
    }
}

/// Methods of the wxRust-specific derived class of [wxApp](http://docs.wxwidgets.org/3.0/classwx_app.html).
pub trait RustAppMethods : AppMethods {
}

/// The wxRust-specific derived class of [wxArtProvider](http://docs.wxwidgets.org/3.0/classwx_art_provider.html).
pub struct RustArtProv { ptr: *mut c_void }
impl RustArtProvMethods for RustArtProv {}
impl ArtProviderMethods for RustArtProv {}
impl ObjectMethods for RustArtProv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustArtProv {
    pub fn from(ptr: *mut c_void) -> RustArtProv { RustArtProv { ptr: ptr } }
    pub fn null() -> RustArtProv { RustArtProv::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _clb: *mut c_void) -> RustArtProv {
        unsafe { RustArtProv::from(ELJArtProv_Create(_obj, _clb)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxArtProvider](http://docs.wxwidgets.org/3.0/classwx_art_provider.html).
pub trait RustArtProvMethods : ArtProviderMethods {
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.ptr()) }
    }
}

/// The wxRust-specific derived class of [wxCommand](http://docs.wxwidgets.org/3.0/classwx_command.html).
pub struct RustCommand { ptr: *mut c_void }
impl RustCommandMethods for RustCommand {}
impl CommandMethods for RustCommand {}
impl ObjectMethods for RustCommand { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustCommand {
    pub fn from(ptr: *mut c_void) -> RustCommand { RustCommand { ptr: ptr } }
    pub fn null() -> RustCommand { RustCommand::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxCommand](http://docs.wxwidgets.org/3.0/classwx_command.html).
pub trait RustCommandMethods : CommandMethods {
}

pub struct RustDragDataObject { ptr: *mut c_void }
impl RustDragDataObjectMethods for RustDragDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustDragDataObject {
    pub fn from(ptr: *mut c_void) -> RustDragDataObject { RustDragDataObject { ptr: ptr } }
    pub fn null() -> RustDragDataObject { RustDragDataObject::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fmt: &str, _func1: *mut c_void, _func2: *mut c_void, _func3: *mut c_void) -> RustDragDataObject {
        let _fmt = strToString(_fmt);
        unsafe { RustDragDataObject::from(ELJDragDataObject_Create(_obj, _fmt.ptr(), _func1, _func2, _func3)) }
    }
}

pub trait RustDragDataObjectMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.ptr()) }
    }
}

/// The wxRust-specific derived class of [wxDropTarget](http://docs.wxwidgets.org/3.0/classwx_drop_target.html).
pub struct RustDropTarget { ptr: *mut c_void }
impl RustDropTargetMethods for RustDropTarget {}
impl DropTargetMethods for RustDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustDropTarget {
    pub fn from(ptr: *mut c_void) -> RustDropTarget { RustDropTarget { ptr: ptr } }
    pub fn null() -> RustDropTarget { RustDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void) -> RustDropTarget {
        unsafe { RustDropTarget::from(ELJDropTarget_Create(_obj)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxDropTarget](http://docs.wxwidgets.org/3.0/classwx_drop_target.html).
pub trait RustDropTargetMethods : DropTargetMethods {
    fn delete(&self) {
        unsafe { ELJDropTarget_Delete(self.ptr()) }
    }
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnData(self.ptr(), _func) }
    }
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnDragOver(self.ptr(), _func) }
    }
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnDrop(self.ptr(), _func) }
    }
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnEnter(self.ptr(), _func) }
    }
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnLeave(self.ptr(), _func) }
    }
}

/// The wxRust-specific derived class of [wxFileDropTarget](http://docs.wxwidgets.org/3.0/classwx_file_drop_target.html).
pub struct RustFileDropTarget { ptr: *mut c_void }
impl RustFileDropTargetMethods for RustFileDropTarget {}
impl FileDropTargetMethods for RustFileDropTarget {}
impl DropTargetMethods for RustFileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustFileDropTarget {
    pub fn from(ptr: *mut c_void) -> RustFileDropTarget { RustFileDropTarget { ptr: ptr } }
    pub fn null() -> RustFileDropTarget { RustFileDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> RustFileDropTarget {
        unsafe { RustFileDropTarget::from(ELJFileDropTarget_Create(_obj, _func)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxFileDropTarget](http://docs.wxwidgets.org/3.0/classwx_file_drop_target.html).
pub trait RustFileDropTargetMethods : FileDropTargetMethods {
    fn delete(&self) {
        unsafe { ELJFileDropTarget_Delete(self.ptr()) }
    }
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnData(self.ptr(), _func) }
    }
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnDragOver(self.ptr(), _func) }
    }
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnDrop(self.ptr(), _func) }
    }
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnEnter(self.ptr(), _func) }
    }
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnLeave(self.ptr(), _func) }
    }
}

/// The wxRust-specific derived class of [wxLog](http://docs.wxwidgets.org/3.0/classwx_log.html).
pub struct RustLog { ptr: *mut c_void }
impl RustLogMethods for RustLog {}
impl LogMethods for RustLog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustLog {
    pub fn from(ptr: *mut c_void) -> RustLog { RustLog { ptr: ptr } }
    pub fn null() -> RustLog { RustLog::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> RustLog {
        unsafe { RustLog::from(ELJLog_Create(_obj, _fnc)) }
    }
    pub fn getActiveTarget() -> *mut c_void {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

/// Methods of the wxRust-specific derived class of [wxLog](http://docs.wxwidgets.org/3.0/classwx_log.html).
pub trait RustLogMethods : LogMethods {
    fn enableLogging(&self, doIt: c_int) -> c_int {
        unsafe { ELJLog_EnableLogging(self.ptr(), doIt) }
    }
    fn isEnabled(&self) -> c_int {
        unsafe { ELJLog_IsEnabled(self.ptr()) }
    }
}

/// The wxRust-specific derived class of [wxPreviewControlBar](http://docs.wxwidgets.org/3.0/classwx_preview_control_bar.html).
pub struct RustPreviewControlBar { ptr: *mut c_void }
impl RustPreviewControlBarMethods for RustPreviewControlBar {}
impl PreviewControlBarMethods for RustPreviewControlBar {}
impl PanelMethods for RustPreviewControlBar {}
impl WindowMethods for RustPreviewControlBar {}
impl EvtHandlerMethods for RustPreviewControlBar {}
impl ObjectMethods for RustPreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPreviewControlBar {
    pub fn from(ptr: *mut c_void) -> RustPreviewControlBar { RustPreviewControlBar { ptr: ptr } }
    pub fn null() -> RustPreviewControlBar { RustPreviewControlBar::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(preview: *mut c_void, buttons: c_int, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> RustPreviewControlBar {
        unsafe { RustPreviewControlBar::from(ELJPreviewControlBar_Create(preview, buttons, parent.ptr(), title, x, y, w, h, style)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxPreviewControlBar](http://docs.wxwidgets.org/3.0/classwx_preview_control_bar.html).
pub trait RustPreviewControlBarMethods : PreviewControlBarMethods {
}

/// The wxRust-specific derived class of [wxPreviewFrame](http://docs.wxwidgets.org/3.0/classwx_preview_frame.html).
pub struct RustPreviewFrame { ptr: *mut c_void }
impl RustPreviewFrameMethods for RustPreviewFrame {}
impl PreviewFrameMethods for RustPreviewFrame {}
impl FrameMethods for RustPreviewFrame {}
impl TopLevelWindowMethods for RustPreviewFrame {}
impl WindowMethods for RustPreviewFrame {}
impl EvtHandlerMethods for RustPreviewFrame {}
impl ObjectMethods for RustPreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPreviewFrame {
    pub fn from(ptr: *mut c_void) -> RustPreviewFrame { RustPreviewFrame { ptr: ptr } }
    pub fn null() -> RustPreviewFrame { RustPreviewFrame::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_obj: *mut c_void, _init: *mut c_void, _create_canvas: *mut c_void, _create_toolbar: *mut c_void, preview: *mut c_void, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> RustPreviewFrame {
        unsafe { RustPreviewFrame::from(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.ptr(), title, x, y, w, h, style)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxPreviewFrame](http://docs.wxwidgets.org/3.0/classwx_preview_frame.html).
pub trait RustPreviewFrameMethods : PreviewFrameMethods {
    fn getControlBar(&self) -> *mut c_void {
        unsafe { ELJPreviewFrame_GetControlBar(self.ptr()) }
    }
    fn getPreviewCanvas(&self) -> PreviewCanvas {
        unsafe { PreviewCanvas::from(ELJPreviewFrame_GetPreviewCanvas(self.ptr())) }
    }
    fn getPrintPreview(&self) -> PrintPreview {
        unsafe { PrintPreview::from(ELJPreviewFrame_GetPrintPreview(self.ptr())) }
    }
    fn setControlBar(&self, obj: *mut c_void) {
        unsafe { ELJPreviewFrame_SetControlBar(self.ptr(), obj) }
    }
    fn setPreviewCanvas<T: PreviewCanvasMethods>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.ptr(), obj.ptr()) }
    }
    fn setPrintPreview<T: PrintPreviewMethods>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.ptr(), obj.ptr()) }
    }
}

/// The wxRust-specific derived class of [wxTextDropTarget](http://docs.wxwidgets.org/3.0/classwx_text_drop_target.html).
pub struct RustTextDropTarget { ptr: *mut c_void }
impl RustTextDropTargetMethods for RustTextDropTarget {}
impl TextDropTargetMethods for RustTextDropTarget {}
impl DropTargetMethods for RustTextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustTextDropTarget {
    pub fn from(ptr: *mut c_void) -> RustTextDropTarget { RustTextDropTarget { ptr: ptr } }
    pub fn null() -> RustTextDropTarget { RustTextDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> RustTextDropTarget {
        unsafe { RustTextDropTarget::from(ELJTextDropTarget_Create(_obj, _func)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxTextDropTarget](http://docs.wxwidgets.org/3.0/classwx_text_drop_target.html).
pub trait RustTextDropTargetMethods : TextDropTargetMethods {
    fn delete(&self) {
        unsafe { ELJTextDropTarget_Delete(self.ptr()) }
    }
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnData(self.ptr(), _func) }
    }
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnDragOver(self.ptr(), _func) }
    }
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnDrop(self.ptr(), _func) }
    }
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnEnter(self.ptr(), _func) }
    }
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnLeave(self.ptr(), _func) }
    }
}

/// The wxRust-specific derived class of [wxTextValidator](http://docs.wxwidgets.org/3.0/classwx_text_validator.html).
pub struct RustTextValidator { ptr: *mut c_void }
impl RustTextValidatorMethods for RustTextValidator {}
impl TextValidatorMethods for RustTextValidator {}
impl ValidatorMethods for RustTextValidator {}
impl EvtHandlerMethods for RustTextValidator {}
impl ObjectMethods for RustTextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustTextValidator {
    pub fn from(ptr: *mut c_void) -> RustTextValidator { RustTextValidator { ptr: ptr } }
    pub fn null() -> RustTextValidator { RustTextValidator::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void, _txt: *mut c_void, _stl: c_int) -> RustTextValidator {
        unsafe { RustTextValidator::from(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) }
    }
}

/// Methods of the wxRust-specific derived class of [wxTextValidator](http://docs.wxwidgets.org/3.0/classwx_text_validator.html).
pub trait RustTextValidatorMethods : TextValidatorMethods {
}

/// Wraps the wxWidgets' [wxAcceleratorEntry](http://docs.wxwidgets.org/3.0/classwx_accelerator_entry.html) class.
pub struct AcceleratorEntry { ptr: *mut c_void }
impl AcceleratorEntryMethods for AcceleratorEntry { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AcceleratorEntry {
    pub fn from(ptr: *mut c_void) -> AcceleratorEntry { AcceleratorEntry { ptr: ptr } }
    pub fn null() -> AcceleratorEntry { AcceleratorEntry::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> AcceleratorEntry {
        unsafe { AcceleratorEntry::from(wxAcceleratorEntry_Create(flags, keyCode, cmd)) }
    }
}

/// Methods of the wxWidgets' [wxAcceleratorEntry](http://docs.wxwidgets.org/3.0/classwx_accelerator_entry.html) class.
pub trait AcceleratorEntryMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxAcceleratorEntry_Delete(self.ptr()) }
    }
    fn getCommand(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetCommand(self.ptr()) }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetFlags(self.ptr()) }
    }
    fn getKeyCode(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetKeyCode(self.ptr()) }
    }
    fn set(&self, flags: c_int, keyCode: c_int, cmd: c_int) {
        unsafe { wxAcceleratorEntry_Set(self.ptr(), flags, keyCode, cmd) }
    }
}

/// Wraps the wxWidgets' [wxAcceleratorTable](http://docs.wxwidgets.org/3.0/classwx_accelerator_table.html) class.
pub struct AcceleratorTable { ptr: *mut c_void }
impl AcceleratorTableMethods for AcceleratorTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AcceleratorTable {
    pub fn from(ptr: *mut c_void) -> AcceleratorTable { AcceleratorTable { ptr: ptr } }
    pub fn null() -> AcceleratorTable { AcceleratorTable::from(0 as *mut c_void) }
    
    pub fn new(n: c_int, entries: *mut c_void) -> AcceleratorTable {
        unsafe { AcceleratorTable::from(wxAcceleratorTable_Create(n, entries)) }
    }
}

/// Methods of the wxWidgets' [wxAcceleratorTable](http://docs.wxwidgets.org/3.0/classwx_accelerator_table.html) class.
pub trait AcceleratorTableMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxActivateEvent](http://docs.wxwidgets.org/3.0/classwx_activate_event.html) class.
pub struct ActivateEvent { ptr: *mut c_void }
impl ActivateEventMethods for ActivateEvent {}
impl EventMethods for ActivateEvent {}
impl ObjectMethods for ActivateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ActivateEvent {
    pub fn from(ptr: *mut c_void) -> ActivateEvent { ActivateEvent { ptr: ptr } }
    pub fn null() -> ActivateEvent { ActivateEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxActivateEvent](http://docs.wxwidgets.org/3.0/classwx_activate_event.html) class.
pub trait ActivateEventMethods : EventMethods {
    fn getActive(&self) -> c_int {
        unsafe { wxActivateEvent_GetActive(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxApp](http://docs.wxwidgets.org/3.0/classwx_app.html) class.
/// Rather use the wxRust-specific [RustApp](struct.RustApp.html) class.
pub struct App { ptr: *mut c_void }
impl AppMethods for App {}
impl EvtHandlerMethods for App {}
impl ObjectMethods for App { fn ptr(&self) -> *mut c_void { self.ptr } }

impl App {
    pub fn from(ptr: *mut c_void) -> App { App { ptr: ptr } }
    pub fn null() -> App { App::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxApp](http://docs.wxwidgets.org/3.0/classwx_app.html) class.
pub trait AppMethods : EvtHandlerMethods {
}

/// Wraps the wxWidgets' [wxArtProvider](http://docs.wxwidgets.org/3.0/classwx_art_provider.html) class.
pub struct ArtProvider { ptr: *mut c_void }
impl ArtProviderMethods for ArtProvider {}
impl ObjectMethods for ArtProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ArtProvider {
    pub fn from(ptr: *mut c_void) -> ArtProvider { ArtProvider { ptr: ptr } }
    pub fn null() -> ArtProvider { ArtProvider::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxArtProvider](http://docs.wxwidgets.org/3.0/classwx_art_provider.html) class.
pub trait ArtProviderMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxAutoBufferedPaintDC](http://docs.wxwidgets.org/3.0/classwx_auto_buffered_paint_dc.html) class.
pub struct AutoBufferedPaintDC { ptr: *mut c_void }
impl AutoBufferedPaintDCMethods for AutoBufferedPaintDC {}
impl DCMethods for AutoBufferedPaintDC {}
impl ObjectMethods for AutoBufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AutoBufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> AutoBufferedPaintDC { AutoBufferedPaintDC { ptr: ptr } }
    pub fn null() -> AutoBufferedPaintDC { AutoBufferedPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(window: &T) -> AutoBufferedPaintDC {
        unsafe { AutoBufferedPaintDC::from(wxAutoBufferedPaintDC_Create(window.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxAutoBufferedPaintDC](http://docs.wxwidgets.org/3.0/classwx_auto_buffered_paint_dc.html) class.
pub trait AutoBufferedPaintDCMethods : DCMethods {
}

/// Wraps the wxWidgets' [wxAutomationObject](http://docs.wxwidgets.org/3.0/classwx_automation_object.html) class.
pub struct AutomationObject { ptr: *mut c_void }
impl AutomationObjectMethods for AutomationObject {}
impl ObjectMethods for AutomationObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AutomationObject {
    pub fn from(ptr: *mut c_void) -> AutomationObject { AutomationObject { ptr: ptr } }
    pub fn null() -> AutomationObject { AutomationObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxAutomationObject](http://docs.wxwidgets.org/3.0/classwx_automation_object.html) class.
pub trait AutomationObjectMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxBitmap](http://docs.wxwidgets.org/3.0/classwx_bitmap.html) class.
pub struct Bitmap { ptr: *mut c_void }
impl BitmapMethods for Bitmap {}
impl GDIObjectMethods for Bitmap {}
impl ObjectMethods for Bitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Bitmap {
    pub fn from(ptr: *mut c_void) -> Bitmap { Bitmap { ptr: ptr } }
    pub fn null() -> Bitmap { Bitmap::from(0 as *mut c_void) }
    
    pub fn addHandler<T: EvtHandlerMethods>(handler: &T) {
        unsafe { wxBitmap_AddHandler(handler.ptr()) }
    }
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    pub fn new(_data: *mut c_void, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> Bitmap {
        unsafe { Bitmap::from(wxBitmap_Create(_data, _type, _width, _height, _depth)) }
    }
    pub fn newDefault() -> Bitmap {
        unsafe { Bitmap::from(wxBitmap_CreateDefault()) }
    }
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> Bitmap {
        unsafe { Bitmap::from(wxBitmap_CreateEmpty(_width, _height, _depth)) }
    }
    pub fn newLoad(name: &str, type_: c_int) -> Bitmap {
        let name = strToString(name);
        unsafe { Bitmap::from(wxBitmap_CreateLoad(name.ptr(), type_)) }
    }
    pub fn findHandlerByName(name: &str) -> *mut c_void {
        let name = strToString(name);
        unsafe { wxBitmap_FindHandlerByName(name.ptr()) }
    }
    pub fn findHandlerByType(type_: c_int) -> *mut c_void {
        unsafe { wxBitmap_FindHandlerByType(type_) }
    }
    pub fn initStandardHandlers() {
        unsafe { wxBitmap_InitStandardHandlers() }
    }
    pub fn insertHandler<T: EvtHandlerMethods>(handler: &T) {
        unsafe { wxBitmap_InsertHandler(handler.ptr()) }
    }
    pub fn removeHandler(name: &str) -> c_int {
        let name = strToString(name);
        unsafe { wxBitmap_RemoveHandler(name.ptr()) }
    }
    pub fn newFromImage<T: ImageMethods>(image: &T, depth: c_int) -> Bitmap {
        unsafe { Bitmap::from(wxBitmap_CreateFromImage(image.ptr(), depth)) }
    }
}

/// Methods of the wxWidgets' [wxBitmap](http://docs.wxwidgets.org/3.0/classwx_bitmap.html) class.
pub trait BitmapMethods : GDIObjectMethods {
    fn newFromXPM(&self) -> Bitmap {
        unsafe { Bitmap::from(wxBitmap_CreateFromXPM(self.ptr())) }
    }
    fn findHandlerByExtension(&self, type_: c_int) -> *mut c_void {
        unsafe { wxBitmap_FindHandlerByExtension(self.ptr(), type_) }
    }
    fn getDepth(&self) -> c_int {
        unsafe { wxBitmap_GetDepth(self.ptr()) }
    }
    fn getHeight(&self) -> c_int {
        unsafe { wxBitmap_GetHeight(self.ptr()) }
    }
    fn getMask(&self) -> Mask {
        unsafe { Mask::from(wxBitmap_GetMask(self.ptr())) }
    }
    fn getSubBitmap<T: BitmapMethods>(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxBitmap_GetSubBitmap(self.ptr(), x, y, w, h, _ref.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.ptr()) }
    }
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = strToString(name);
        unsafe { wxBitmap_LoadFile(self.ptr(), name.ptr(), type_) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxBitmap_IsOk(self.ptr()) }
    }
    fn saveFile<T: PaletteMethods>(&self, name: &str, type_: c_int, cmap: &T) -> c_int {
        let name = strToString(name);
        unsafe { wxBitmap_SaveFile(self.ptr(), name.ptr(), type_, cmap.ptr()) }
    }
    fn setDepth(&self, d: c_int) {
        unsafe { wxBitmap_SetDepth(self.ptr(), d) }
    }
    fn setHeight(&self, h: c_int) {
        unsafe { wxBitmap_SetHeight(self.ptr(), h) }
    }
    fn setMask<T: MaskMethods>(&self, mask: &T) {
        unsafe { wxBitmap_SetMask(self.ptr(), mask.ptr()) }
    }
    fn setWidth(&self, w: c_int) {
        unsafe { wxBitmap_SetWidth(self.ptr(), w) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxBitmap_IsStatic(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxBitmapButton](http://docs.wxwidgets.org/3.0/classwx_bitmap_button.html) class.
pub struct BitmapButton { ptr: *mut c_void }
impl BitmapButtonMethods for BitmapButton {}
impl ButtonMethods for BitmapButton {}
impl ControlMethods for BitmapButton {}
impl WindowMethods for BitmapButton {}
impl EvtHandlerMethods for BitmapButton {}
impl ObjectMethods for BitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapButton {
    pub fn from(ptr: *mut c_void) -> BitmapButton { BitmapButton { ptr: ptr } }
    pub fn null() -> BitmapButton { BitmapButton::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: BitmapMethods>(_prt: &T, _id: c_int, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> BitmapButton {
        unsafe { BitmapButton::from(wxBitmapButton_Create(_prt.ptr(), _id, _bmp.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxBitmapButton](http://docs.wxwidgets.org/3.0/classwx_bitmap_button.html) class.
pub trait BitmapButtonMethods : ButtonMethods {
    fn getBitmapDisabled<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapFocus<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapLabel<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapSelected<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapSelected(self.ptr(), _ref.ptr()) }
    }
    fn getMarginX(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginX(self.ptr()) }
    }
    fn getMarginY(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginY(self.ptr()) }
    }
    fn setBitmapDisabled<T: BitmapMethods>(&self, disabled: &T) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.ptr(), disabled.ptr()) }
    }
    fn setBitmapFocus<T: BitmapMethods>(&self, focus: &T) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.ptr(), focus.ptr()) }
    }
    fn setBitmapLabel<T: BitmapMethods>(&self, bitmap: &T) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.ptr(), bitmap.ptr()) }
    }
    fn setBitmapSelected<T: BitmapMethods>(&self, sel: &T) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.ptr(), sel.ptr()) }
    }
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.ptr(), x, y) }
    }
}

/// Wraps the wxWidgets' [wxBitmapToggleButton](http://docs.wxwidgets.org/3.0/classwx_bitmap_toggle_button.html) class.
pub struct BitmapToggleButton { ptr: *mut c_void }
impl BitmapToggleButtonMethods for BitmapToggleButton {}
impl ToggleButtonMethods for BitmapToggleButton {}
impl ControlMethods for BitmapToggleButton {}
impl WindowMethods for BitmapToggleButton {}
impl EvtHandlerMethods for BitmapToggleButton {}
impl ObjectMethods for BitmapToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapToggleButton {
    pub fn from(ptr: *mut c_void) -> BitmapToggleButton { BitmapToggleButton { ptr: ptr } }
    pub fn null() -> BitmapToggleButton { BitmapToggleButton::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: BitmapMethods>(parent: &T, id: c_int, _bmp: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> BitmapToggleButton {
        unsafe { BitmapToggleButton::from(wxBitmapToggleButton_Create(parent.ptr(), id, _bmp.ptr(), x, y, w, h, style)) }
    }
}

/// Methods of the wxWidgets' [wxBitmapToggleButton](http://docs.wxwidgets.org/3.0/classwx_bitmap_toggle_button.html) class.
pub trait BitmapToggleButtonMethods : ToggleButtonMethods {
    fn setBitmapLabel<T: BitmapMethods>(&self, _bmp: &T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.ptr(), _bmp.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxBitmapDataObject](http://docs.wxwidgets.org/3.0/classwx_bitmap_data_object.html) class.
pub struct BitmapDataObject { ptr: *mut c_void }
impl BitmapDataObjectMethods for BitmapDataObject {}
impl DataObjectSimpleMethods for BitmapDataObject {}
impl DataObjectMethods for BitmapDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapDataObject {
    pub fn from(ptr: *mut c_void) -> BitmapDataObject { BitmapDataObject { ptr: ptr } }
    pub fn null() -> BitmapDataObject { BitmapDataObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxBitmapDataObject](http://docs.wxwidgets.org/3.0/classwx_bitmap_data_object.html) class.
pub trait BitmapDataObjectMethods : DataObjectSimpleMethods {
}

/// Wraps the wxWidgets' [wxBitmapHandler](http://docs.wxwidgets.org/3.0/classwx_bitmap_handler.html) class.
pub struct BitmapHandler { ptr: *mut c_void }
impl BitmapHandlerMethods for BitmapHandler {}
impl ObjectMethods for BitmapHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapHandler {
    pub fn from(ptr: *mut c_void) -> BitmapHandler { BitmapHandler { ptr: ptr } }
    pub fn null() -> BitmapHandler { BitmapHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxBitmapHandler](http://docs.wxwidgets.org/3.0/classwx_bitmap_handler.html) class.
pub trait BitmapHandlerMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxBoxSizer](http://docs.wxwidgets.org/3.0/classwx_box_sizer.html) class.
pub struct BoxSizer { ptr: *mut c_void }
impl BoxSizerMethods for BoxSizer {}
impl SizerMethods for BoxSizer {}
impl ObjectMethods for BoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BoxSizer {
    pub fn from(ptr: *mut c_void) -> BoxSizer { BoxSizer { ptr: ptr } }
    pub fn null() -> BoxSizer { BoxSizer::from(0 as *mut c_void) }
    
    pub fn new(orient: c_int) -> BoxSizer {
        unsafe { BoxSizer::from(wxBoxSizer_Create(orient)) }
    }
}

/// Methods of the wxWidgets' [wxBoxSizer](http://docs.wxwidgets.org/3.0/classwx_box_sizer.html) class.
pub trait BoxSizerMethods : SizerMethods {
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxBrush](http://docs.wxwidgets.org/3.0/classwx_brush.html) class.
pub struct Brush { ptr: *mut c_void }
impl BrushMethods for Brush {}
impl GDIObjectMethods for Brush {}
impl ObjectMethods for Brush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Brush {
    pub fn from(ptr: *mut c_void) -> Brush { Brush { ptr: ptr } }
    pub fn null() -> Brush { Brush::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Brush {
        unsafe { Brush::from(wxBrush_CreateDefault()) }
    }
    pub fn newFromBitmap<T: BitmapMethods>(bitmap: &T) -> Brush {
        unsafe { Brush::from(wxBrush_CreateFromBitmap(bitmap.ptr())) }
    }
    pub fn newFromColour<T: ColourMethods>(col: &T, style: c_int) -> Brush {
        unsafe { Brush::from(wxBrush_CreateFromColour(col.ptr(), style)) }
    }
    pub fn newFromStock(id: c_int) -> Brush {
        unsafe { Brush::from(wxBrush_CreateFromStock(id)) }
    }
}

/// Methods of the wxWidgets' [wxBrush](http://docs.wxwidgets.org/3.0/classwx_brush.html) class.
pub trait BrushMethods : GDIObjectMethods {
    fn assign<T: BrushMethods>(&self, brush: &T) {
        unsafe { wxBrush_Assign(self.ptr(), brush.ptr()) }
    }
    fn getColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxBrush_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getStipple<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxBrush_GetStipple(self.ptr(), _ref.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.ptr()) }
    }
    fn isEqual<T: BrushMethods>(&self, brush: &T) -> c_int {
        unsafe { wxBrush_IsEqual(self.ptr(), brush.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxBrush_IsOk(self.ptr()) }
    }
    fn setColour<T: ColourMethods>(&self, col: &T) {
        unsafe { wxBrush_SetColour(self.ptr(), col.ptr()) }
    }
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxBrush_SetColourSingle(self.ptr(), r, g, b) }
    }
    fn setStipple<T: BitmapMethods>(&self, stipple: &T) {
        unsafe { wxBrush_SetStipple(self.ptr(), stipple.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxBrush_SetStyle(self.ptr(), style) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxBrush_IsStatic(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxBrushList](http://docs.wxwidgets.org/3.0/classwx_brush_list.html) class.
pub struct BrushList { ptr: *mut c_void }
impl BrushListMethods for BrushList {}
impl ListMethods for BrushList {}
impl ObjectMethods for BrushList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BrushList {
    pub fn from(ptr: *mut c_void) -> BrushList { BrushList { ptr: ptr } }
    pub fn null() -> BrushList { BrushList::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxBrushList](http://docs.wxwidgets.org/3.0/classwx_brush_list.html) class.
pub trait BrushListMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxBufferedDC](http://docs.wxwidgets.org/3.0/classwx_buffered_dc.html) class.
pub struct BufferedDC { ptr: *mut c_void }
impl BufferedDCMethods for BufferedDC {}
impl DCMethods for BufferedDC {}
impl ObjectMethods for BufferedDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedDC {
    pub fn from(ptr: *mut c_void) -> BufferedDC { BufferedDC { ptr: ptr } }
    pub fn null() -> BufferedDC { BufferedDC::from(0 as *mut c_void) }
    
    pub fn newByDCAndSize<T: DCMethods>(dc: &T, width: c_int, hight: c_int, style: c_int) -> BufferedDC {
        unsafe { BufferedDC::from(wxBufferedDC_CreateByDCAndSize(dc.ptr(), width, hight, style)) }
    }
    pub fn newByDCAndBitmap<T: DCMethods, U: BitmapMethods>(dc: &T, bitmap: &U, style: c_int) -> BufferedDC {
        unsafe { BufferedDC::from(wxBufferedDC_CreateByDCAndBitmap(dc.ptr(), bitmap.ptr(), style)) }
    }
}

/// Methods of the wxWidgets' [wxBufferedDC](http://docs.wxwidgets.org/3.0/classwx_buffered_dc.html) class.
pub trait BufferedDCMethods : DCMethods {
}

/// Wraps the wxWidgets' [wxBufferedPaintDC](http://docs.wxwidgets.org/3.0/classwx_buffered_paint_dc.html) class.
pub struct BufferedPaintDC { ptr: *mut c_void }
impl BufferedPaintDCMethods for BufferedPaintDC {}
impl DCMethods for BufferedPaintDC {}
impl ObjectMethods for BufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> BufferedPaintDC { BufferedPaintDC { ptr: ptr } }
    pub fn null() -> BufferedPaintDC { BufferedPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(window: &T, style: c_int) -> BufferedPaintDC {
        unsafe { BufferedPaintDC::from(wxBufferedPaintDC_Create(window.ptr(), style)) }
    }
    pub fn newWithBitmap<T: WindowMethods, U: BitmapMethods>(window: &T, bitmap: &U, style: c_int) -> BufferedPaintDC {
        unsafe { BufferedPaintDC::from(wxBufferedPaintDC_CreateWithBitmap(window.ptr(), bitmap.ptr(), style)) }
    }
}

/// Methods of the wxWidgets' [wxBufferedPaintDC](http://docs.wxwidgets.org/3.0/classwx_buffered_paint_dc.html) class.
pub trait BufferedPaintDCMethods : DCMethods {
}

/// Wraps the wxWidgets' [wxBusyCursor](http://docs.wxwidgets.org/3.0/classwx_busy_cursor.html) class.
pub struct BusyCursor { ptr: *mut c_void }
impl BusyCursorMethods for BusyCursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BusyCursor {
    pub fn from(ptr: *mut c_void) -> BusyCursor { BusyCursor { ptr: ptr } }
    pub fn null() -> BusyCursor { BusyCursor::from(0 as *mut c_void) }
    
    pub fn new() -> BusyCursor {
        unsafe { BusyCursor::from(wxBusyCursor_Create()) }
    }
}

/// Methods of the wxWidgets' [wxBusyCursor](http://docs.wxwidgets.org/3.0/classwx_busy_cursor.html) class.
pub trait BusyCursorMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn newWithCursor(&self) -> *mut c_void {
        unsafe { wxBusyCursor_CreateWithCursor(self.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxBusyInfo](http://docs.wxwidgets.org/3.0/classwx_busy_info.html) class.
pub struct BusyInfo { ptr: *mut c_void }
impl BusyInfoMethods for BusyInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BusyInfo {
    pub fn from(ptr: *mut c_void) -> BusyInfo { BusyInfo { ptr: ptr } }
    pub fn null() -> BusyInfo { BusyInfo::from(0 as *mut c_void) }
    
    pub fn new(_txt: &str) -> BusyInfo {
        let _txt = strToString(_txt);
        unsafe { BusyInfo::from(wxBusyInfo_Create(_txt.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxBusyInfo](http://docs.wxwidgets.org/3.0/classwx_busy_info.html) class.
pub trait BusyInfoMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxButton](http://docs.wxwidgets.org/3.0/classwx_button.html) class.
pub struct Button { ptr: *mut c_void }
impl ButtonMethods for Button {}
impl ControlMethods for Button {}
impl WindowMethods for Button {}
impl EvtHandlerMethods for Button {}
impl ObjectMethods for Button { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Button {
    pub fn from(ptr: *mut c_void) -> Button { Button { ptr: ptr } }
    pub fn null() -> Button { Button::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Button {
        let _txt = strToString(_txt);
        unsafe { Button::from(wxButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxButton](http://docs.wxwidgets.org/3.0/classwx_button.html) class.
pub trait ButtonMethods : ControlMethods {
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxCaret](http://docs.wxwidgets.org/3.0/classwx_caret.html) class.
pub struct Caret { ptr: *mut c_void }
impl CaretMethods for Caret { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Caret {
    pub fn from(ptr: *mut c_void) -> Caret { Caret { ptr: ptr } }
    pub fn null() -> Caret { Caret::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_wnd: &T, _wth: c_int, _hgt: c_int) -> Caret {
        unsafe { Caret::from(wxCaret_Create(_wnd.ptr(), _wth, _hgt)) }
    }
    pub fn getBlinkTime() -> c_int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    pub fn setBlinkTime(milliseconds: c_int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
    }
}

/// Methods of the wxWidgets' [wxCaret](http://docs.wxwidgets.org/3.0/classwx_caret.html) class.
pub trait CaretMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxCaret_GetPosition(self.ptr())) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size::from(wxCaret_GetSize(self.ptr())) }
    }
    fn getWindow(&self) -> Window {
        unsafe { Window::from(wxCaret_GetWindow(self.ptr())) }
    }
    fn hide(&self) {
        unsafe { wxCaret_Hide(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxCaret_IsOk(self.ptr()) }
    }
    fn isVisible(&self) -> c_int {
        unsafe { wxCaret_IsVisible(self.ptr()) }
    }
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxCaret_Move(self.ptr(), x, y) }
    }
    fn setSize(&self, width: c_int, height: c_int) {
        unsafe { wxCaret_SetSize(self.ptr(), width, height) }
    }
    fn show(&self) {
        unsafe { wxCaret_Show(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxCheckBox](http://docs.wxwidgets.org/3.0/classwx_check_box.html) class.
pub struct CheckBox { ptr: *mut c_void }
impl CheckBoxMethods for CheckBox {}
impl ControlMethods for CheckBox {}
impl WindowMethods for CheckBox {}
impl EvtHandlerMethods for CheckBox {}
impl ObjectMethods for CheckBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CheckBox {
    pub fn from(ptr: *mut c_void) -> CheckBox { CheckBox { ptr: ptr } }
    pub fn null() -> CheckBox { CheckBox::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> CheckBox {
        let _txt = strToString(_txt);
        unsafe { CheckBox::from(wxCheckBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxCheckBox](http://docs.wxwidgets.org/3.0/classwx_check_box.html) class.
pub trait CheckBoxMethods : ControlMethods {
    fn getValue(&self) -> c_int {
        unsafe { wxCheckBox_GetValue(self.ptr()) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self.ptr(), value) }
    }
}

/// Wraps the wxWidgets' [wxCheckListBox](http://docs.wxwidgets.org/3.0/classwx_check_list_box.html) class.
pub struct CheckListBox { ptr: *mut c_void }
impl CheckListBoxMethods for CheckListBox {}
impl ListBoxMethods for CheckListBox {}
impl ControlMethods for CheckListBox {}
impl WindowMethods for CheckListBox {}
impl EvtHandlerMethods for CheckListBox {}
impl ObjectMethods for CheckListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CheckListBox {
    pub fn from(ptr: *mut c_void) -> CheckListBox { CheckListBox { ptr: ptr } }
    pub fn null() -> CheckListBox { CheckListBox::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> CheckListBox {
        unsafe { CheckListBox::from(wxCheckListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxCheckListBox](http://docs.wxwidgets.org/3.0/classwx_check_list_box.html) class.
pub trait CheckListBoxMethods : ListBoxMethods {
    fn check(&self, item: c_int, check: c_int) {
        unsafe { wxCheckListBox_Check(self.ptr(), item, check) }
    }
    fn isChecked(&self, item: c_int) -> c_int {
        unsafe { wxCheckListBox_IsChecked(self.ptr(), item) }
    }
}

/// Wraps the wxWidgets' [wxChoice](http://docs.wxwidgets.org/3.0/classwx_choice.html) class.
pub struct Choice { ptr: *mut c_void }
impl ChoiceMethods for Choice {}
impl ControlMethods for Choice {}
impl WindowMethods for Choice {}
impl EvtHandlerMethods for Choice {}
impl ObjectMethods for Choice { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Choice {
    pub fn from(ptr: *mut c_void) -> Choice { Choice { ptr: ptr } }
    pub fn null() -> Choice { Choice::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> Choice {
        unsafe { Choice::from(wxChoice_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxChoice](http://docs.wxwidgets.org/3.0/classwx_choice.html) class.
pub trait ChoiceMethods : ControlMethods {
    fn append(&self, item: &str) {
        let item = strToString(item);
        unsafe { wxChoice_Append(self.ptr(), item.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxChoice_Clear(self.ptr()) }
    }
    fn findString(&self, s: &str) -> c_int {
        let s = strToString(s);
        unsafe { wxChoice_FindString(self.ptr(), s.ptr()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxChoice_GetCount(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxChoice_GetSelection(self.ptr()) }
    }
    fn getString(&self, n: c_int) -> ~str {
        unsafe { String::from(wxChoice_GetString(self.ptr(), n)).to_str() }
    }
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.ptr(), n) }
    }
    fn setString(&self, n: c_int, s: &str) {
        let s = strToString(s);
        unsafe { wxChoice_SetString(self.ptr(), n, s.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxClientDC](http://docs.wxwidgets.org/3.0/classwx_client_dc.html) class.
pub struct ClientDC { ptr: *mut c_void }
impl ClientDCMethods for ClientDC {}
impl WindowDCMethods for ClientDC {}
impl DCMethods for ClientDC {}
impl ObjectMethods for ClientDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientDC {
    pub fn from(ptr: *mut c_void) -> ClientDC { ClientDC { ptr: ptr } }
    pub fn null() -> ClientDC { ClientDC::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(win: &T) -> ClientDC {
        unsafe { ClientDC::from(wxClientDC_Create(win.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxClientDC](http://docs.wxwidgets.org/3.0/classwx_client_dc.html) class.
pub trait ClientDCMethods : WindowDCMethods {
}

/// Wraps the wxWidgets' [wxClipboard](http://docs.wxwidgets.org/3.0/classwx_clipboard.html) class.
pub struct Clipboard { ptr: *mut c_void }
impl ClipboardMethods for Clipboard {}
impl ObjectMethods for Clipboard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Clipboard {
    pub fn from(ptr: *mut c_void) -> Clipboard { Clipboard { ptr: ptr } }
    pub fn null() -> Clipboard { Clipboard::from(0 as *mut c_void) }
    
    pub fn new() -> Clipboard {
        unsafe { Clipboard::from(wxClipboard_Create()) }
    }
}

/// Methods of the wxWidgets' [wxClipboard](http://docs.wxwidgets.org/3.0/classwx_clipboard.html) class.
pub trait ClipboardMethods : ObjectMethods {
    fn addData<T: DataObjectMethods>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_AddData(self.ptr(), data.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxClipboard_Clear(self.ptr()) }
    }
    fn close(&self) {
        unsafe { wxClipboard_Close(self.ptr()) }
    }
    fn flush(&self) -> c_int {
        unsafe { wxClipboard_Flush(self.ptr()) }
    }
    fn getData<T: DataObjectMethods>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_GetData(self.ptr(), data.ptr()) }
    }
    fn isOpened(&self) -> c_int {
        unsafe { wxClipboard_IsOpened(self.ptr()) }
    }
    fn isSupported<T: DataFormatMethods>(&self, format: &T) -> c_int {
        unsafe { wxClipboard_IsSupported(self.ptr(), format.ptr()) }
    }
    fn open(&self) -> c_int {
        unsafe { wxClipboard_Open(self.ptr()) }
    }
    fn setData<T: DataObjectMethods>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_SetData(self.ptr(), data.ptr()) }
    }
    fn usePrimarySelection(&self, primary: c_int) {
        unsafe { wxClipboard_UsePrimarySelection(self.ptr(), primary) }
    }
}

/// Wraps the wxWidgets' [wxCloseEvent](http://docs.wxwidgets.org/3.0/classwx_close_event.html) class.
pub struct CloseEvent { ptr: *mut c_void }
impl CloseEventMethods for CloseEvent {}
impl EventMethods for CloseEvent {}
impl ObjectMethods for CloseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CloseEvent {
    pub fn from(ptr: *mut c_void) -> CloseEvent { CloseEvent { ptr: ptr } }
    pub fn null() -> CloseEvent { CloseEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCloseEvent](http://docs.wxwidgets.org/3.0/classwx_close_event.html) class.
pub trait CloseEventMethods : EventMethods {
    fn canVeto(&self) -> c_int {
        unsafe { wxCloseEvent_CanVeto(self.ptr()) }
    }
    fn getLoggingOff(&self) -> c_int {
        unsafe { wxCloseEvent_GetLoggingOff(self.ptr()) }
    }
    fn getVeto(&self) -> c_int {
        unsafe { wxCloseEvent_GetVeto(self.ptr()) }
    }
    fn setCanVeto(&self, canVeto: c_int) {
        unsafe { wxCloseEvent_SetCanVeto(self.ptr(), canVeto) }
    }
    fn setLoggingOff(&self, logOff: c_int) {
        unsafe { wxCloseEvent_SetLoggingOff(self.ptr(), logOff) }
    }
    fn veto(&self, veto: c_int) {
        unsafe { wxCloseEvent_Veto(self.ptr(), veto) }
    }
}

/// Wraps the wxWidgets' [wxColour](http://docs.wxwidgets.org/3.0/classwx_colour.html) class.
pub struct Colour { ptr: *mut c_void }
impl ColourMethods for Colour {}
impl ObjectMethods for Colour { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Colour {
    pub fn from(ptr: *mut c_void) -> Colour { Colour { ptr: ptr } }
    pub fn null() -> Colour { Colour::from(0 as *mut c_void) }
    
    pub fn newByName(_name: &str) -> Colour {
        let _name = strToString(_name);
        unsafe { Colour::from(wxColour_CreateByName(_name.ptr())) }
    }
    pub fn newEmpty() -> Colour {
        unsafe { Colour::from(wxColour_CreateEmpty()) }
    }
    pub fn newFromStock(id: c_int) -> Colour {
        unsafe { Colour::from(wxColour_CreateFromStock(id)) }
    }
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> Colour {
        unsafe { Colour::from(wxColour_CreateRGB(_red, _green, _blue, _alpha)) }
    }
    pub fn validName(_name: *mut c_void) -> c_int {
        unsafe { wxColour_ValidName(_name) }
    }
    pub fn newFromInt(rgb: c_int) -> Colour {
        unsafe { Colour::from(wxColour_CreateFromInt(rgb)) }
    }
    pub fn newFromUnsignedInt(rgba: uint32_t) -> Colour {
        unsafe { Colour::from(wxColour_CreateFromUnsignedInt(rgba)) }
    }
}

/// Methods of the wxWidgets' [wxColour](http://docs.wxwidgets.org/3.0/classwx_colour.html) class.
pub trait ColourMethods : ObjectMethods {
    fn alpha(&self) -> uint8_t {
        unsafe { wxColour_Alpha(self.ptr()) }
    }
    fn assign(&self, other: *mut c_void) {
        unsafe { wxColour_Assign(self.ptr(), other) }
    }
    fn blue(&self) -> uint8_t {
        unsafe { wxColour_Blue(self.ptr()) }
    }
    fn copy(&self, _other: *mut c_void) {
        unsafe { wxColour_Copy(self.ptr(), _other) }
    }
    fn green(&self) -> uint8_t {
        unsafe { wxColour_Green(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxColour_IsOk(self.ptr()) }
    }
    fn red(&self) -> uint8_t {
        unsafe { wxColour_Red(self.ptr()) }
    }
    fn set(&self, _red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) {
        unsafe { wxColour_Set(self.ptr(), _red, _green, _blue, _alpha) }
    }
    fn setByName(&self, _name: &str) {
        let _name = strToString(_name);
        unsafe { wxColour_SetByName(self.ptr(), _name.ptr()) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxColour_IsStatic(self.ptr()) }
    }
    fn getInt(&self) -> c_int {
        unsafe { wxColour_GetInt(self.ptr()) }
    }
    fn getUnsignedInt(&self) -> uint32_t {
        unsafe { wxColour_GetUnsignedInt(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxColourData](http://docs.wxwidgets.org/3.0/classwx_colour_data.html) class.
pub struct ColourData { ptr: *mut c_void }
impl ColourDataMethods for ColourData {}
impl ObjectMethods for ColourData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ColourData {
    pub fn from(ptr: *mut c_void) -> ColourData { ColourData { ptr: ptr } }
    pub fn null() -> ColourData { ColourData::from(0 as *mut c_void) }
    
    pub fn new() -> ColourData {
        unsafe { ColourData::from(wxColourData_Create()) }
    }
}

/// Methods of the wxWidgets' [wxColourData](http://docs.wxwidgets.org/3.0/classwx_colour_data.html) class.
pub trait ColourDataMethods : ObjectMethods {
    fn getChooseFull(&self) -> c_int {
        unsafe { wxColourData_GetChooseFull(self.ptr()) }
    }
    fn getColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxColourData_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getCustomColour<T: ColourMethods>(&self, i: c_int, _ref: &T) {
        unsafe { wxColourData_GetCustomColour(self.ptr(), i, _ref.ptr()) }
    }
    fn setChooseFull(&self, flag: c_int) {
        unsafe { wxColourData_SetChooseFull(self.ptr(), flag) }
    }
    fn setColour<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxColourData_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCustomColour<T: ColourMethods>(&self, i: c_int, colour: &T) {
        unsafe { wxColourData_SetCustomColour(self.ptr(), i, colour.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxColourDatabase](http://docs.wxwidgets.org/3.0/classwx_colour_database.html) class.
pub struct ColourDatabase { ptr: *mut c_void }
impl ColourDatabaseMethods for ColourDatabase {}
impl ListMethods for ColourDatabase {}
impl ObjectMethods for ColourDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ColourDatabase {
    pub fn from(ptr: *mut c_void) -> ColourDatabase { ColourDatabase { ptr: ptr } }
    pub fn null() -> ColourDatabase { ColourDatabase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxColourDatabase](http://docs.wxwidgets.org/3.0/classwx_colour_database.html) class.
pub trait ColourDatabaseMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxColourDialog](http://docs.wxwidgets.org/3.0/classwx_colour_dialog.html) class.
pub struct ColourDialog { ptr: *mut c_void }
impl ColourDialogMethods for ColourDialog {}
impl DialogMethods for ColourDialog {}
impl TopLevelWindowMethods for ColourDialog {}
impl WindowMethods for ColourDialog {}
impl EvtHandlerMethods for ColourDialog {}
impl ObjectMethods for ColourDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ColourDialog {
    pub fn from(ptr: *mut c_void) -> ColourDialog { ColourDialog { ptr: ptr } }
    pub fn null() -> ColourDialog { ColourDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: ColourDataMethods>(_prt: &T, col: &U) -> ColourDialog {
        unsafe { ColourDialog::from(wxColourDialog_Create(_prt.ptr(), col.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxColourDialog](http://docs.wxwidgets.org/3.0/classwx_colour_dialog.html) class.
pub trait ColourDialogMethods : DialogMethods {
    fn getColourData<T: ColourDataMethods>(&self, _ref: &T) {
        unsafe { wxColourDialog_GetColourData(self.ptr(), _ref.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxComboBox](http://docs.wxwidgets.org/3.0/classwx_combo_box.html) class.
pub struct ComboBox { ptr: *mut c_void }
impl ComboBoxMethods for ComboBox {}
impl ChoiceMethods for ComboBox {}
impl ControlMethods for ComboBox {}
impl WindowMethods for ComboBox {}
impl EvtHandlerMethods for ComboBox {}
impl ObjectMethods for ComboBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ComboBox {
    pub fn from(ptr: *mut c_void) -> ComboBox { ComboBox { ptr: ptr } }
    pub fn null() -> ComboBox { ComboBox::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> ComboBox {
        let _txt = strToString(_txt);
        unsafe { ComboBox::from(wxComboBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxComboBox](http://docs.wxwidgets.org/3.0/classwx_combo_box.html) class.
pub trait ComboBoxMethods : ChoiceMethods {
    fn appendData(&self, item: &str, d: *mut c_void) {
        let item = strToString(item);
        unsafe { wxComboBox_AppendData(self.ptr(), item.ptr(), d) }
    }
    fn copy(&self) {
        unsafe { wxComboBox_Copy(self.ptr()) }
    }
    fn cut(&self) {
        unsafe { wxComboBox_Cut(self.ptr()) }
    }
    fn getInsertionPoint(&self) -> c_int {
        unsafe { wxComboBox_GetInsertionPoint(self.ptr()) }
    }
    fn getLastPosition(&self) -> c_int {
        unsafe { wxComboBox_GetLastPosition(self.ptr()) }
    }
    fn getStringSelection(&self) -> ~str {
        unsafe { String::from(wxComboBox_GetStringSelection(self.ptr())).to_str() }
    }
    fn getValue(&self) -> ~str {
        unsafe { String::from(wxComboBox_GetValue(self.ptr())).to_str() }
    }
    fn paste(&self) {
        unsafe { wxComboBox_Paste(self.ptr()) }
    }
    fn remove(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_Remove(self.ptr(), from, to) }
    }
    fn replace(&self, from: c_int, to: c_int, value: &str) {
        let value = strToString(value);
        unsafe { wxComboBox_Replace(self.ptr(), from, to, value.ptr()) }
    }
    fn setEditable(&self, editable: c_int) {
        unsafe { wxComboBox_SetEditable(self.ptr(), editable) }
    }
    fn setInsertionPoint(&self, pos: c_int) {
        unsafe { wxComboBox_SetInsertionPoint(self.ptr(), pos) }
    }
    fn setInsertionPointEnd(&self) {
        unsafe { wxComboBox_SetInsertionPointEnd(self.ptr()) }
    }
    fn setTextSelection(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_SetTextSelection(self.ptr(), from, to) }
    }
}

/// Wraps the wxWidgets' [wxCommand](http://docs.wxwidgets.org/3.0/classwx_command.html) class.
/// Rather use the wxRust-specific [RustCommand](struct.RustCommand.html) class.
pub struct Command { ptr: *mut c_void }
impl CommandMethods for Command {}
impl ObjectMethods for Command { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Command {
    pub fn from(ptr: *mut c_void) -> Command { Command { ptr: ptr } }
    pub fn null() -> Command { Command::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCommand](http://docs.wxwidgets.org/3.0/classwx_command.html) class.
pub trait CommandMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxCommandEvent](http://docs.wxwidgets.org/3.0/classwx_command_event.html) class.
pub struct CommandEvent { ptr: *mut c_void }
impl CommandEventMethods for CommandEvent {}
impl EventMethods for CommandEvent {}
impl ObjectMethods for CommandEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CommandEvent {
    pub fn from(ptr: *mut c_void) -> CommandEvent { CommandEvent { ptr: ptr } }
    pub fn null() -> CommandEvent { CommandEvent::from(0 as *mut c_void) }
    
    pub fn new(_typ: c_int, _id: c_int) -> CommandEvent {
        unsafe { CommandEvent::from(wxCommandEvent_Create(_typ, _id)) }
    }
}

/// Methods of the wxWidgets' [wxCommandEvent](http://docs.wxwidgets.org/3.0/classwx_command_event.html) class.
pub trait CommandEventMethods : EventMethods {
    fn getClientData(&self) -> ClientData {
        unsafe { ClientData::from(wxCommandEvent_GetClientData(self.ptr())) }
    }
    fn getClientObject(&self) -> ClientData {
        unsafe { ClientData::from(wxCommandEvent_GetClientObject(self.ptr())) }
    }
    fn getExtraLong(&self) -> c_long {
        unsafe { wxCommandEvent_GetExtraLong(self.ptr()) }
    }
    fn getInt(&self) -> c_long {
        unsafe { wxCommandEvent_GetInt(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxCommandEvent_GetSelection(self.ptr()) }
    }
    fn getString(&self) -> ~str {
        unsafe { String::from(wxCommandEvent_GetString(self.ptr())).to_str() }
    }
    fn isChecked(&self) -> c_int {
        unsafe { wxCommandEvent_IsChecked(self.ptr()) }
    }
    fn isSelection(&self) -> c_int {
        unsafe { wxCommandEvent_IsSelection(self.ptr()) }
    }
    fn setClientData<T: ClientDataMethods>(&self, clientData: &T) {
        unsafe { wxCommandEvent_SetClientData(self.ptr(), clientData.ptr()) }
    }
    fn setClientObject<T: ClientDataMethods>(&self, clientObject: &T) {
        unsafe { wxCommandEvent_SetClientObject(self.ptr(), clientObject.ptr()) }
    }
    fn setExtraLong(&self, extraLong: c_long) {
        unsafe { wxCommandEvent_SetExtraLong(self.ptr(), extraLong) }
    }
    fn setInt(&self, i: c_int) {
        unsafe { wxCommandEvent_SetInt(self.ptr(), i) }
    }
    fn setString(&self, s: &str) {
        let s = strToString(s);
        unsafe { wxCommandEvent_SetString(self.ptr(), s.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxCommandProcessor](http://docs.wxwidgets.org/3.0/classwx_command_processor.html) class.
pub struct CommandProcessor { ptr: *mut c_void }
impl CommandProcessorMethods for CommandProcessor {}
impl ObjectMethods for CommandProcessor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CommandProcessor {
    pub fn from(ptr: *mut c_void) -> CommandProcessor { CommandProcessor { ptr: ptr } }
    pub fn null() -> CommandProcessor { CommandProcessor::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCommandProcessor](http://docs.wxwidgets.org/3.0/classwx_command_processor.html) class.
pub trait CommandProcessorMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxContextHelp](http://docs.wxwidgets.org/3.0/classwx_context_help.html) class.
pub struct ContextHelp { ptr: *mut c_void }
impl ContextHelpMethods for ContextHelp {}
impl ObjectMethods for ContextHelp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ContextHelp {
    pub fn from(ptr: *mut c_void) -> ContextHelp { ContextHelp { ptr: ptr } }
    pub fn null() -> ContextHelp { ContextHelp::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(win: &T, beginHelp: c_int) -> ContextHelp {
        unsafe { ContextHelp::from(wxContextHelp_Create(win.ptr(), beginHelp)) }
    }
}

/// Methods of the wxWidgets' [wxContextHelp](http://docs.wxwidgets.org/3.0/classwx_context_help.html) class.
pub trait ContextHelpMethods : ObjectMethods {
    fn beginContextHelp<T: WindowMethods>(&self, win: &T) -> c_int {
        unsafe { wxContextHelp_BeginContextHelp(self.ptr(), win.ptr()) }
    }
    fn endContextHelp(&self) -> c_int {
        unsafe { wxContextHelp_EndContextHelp(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxContextHelpButton](http://docs.wxwidgets.org/3.0/classwx_context_help_button.html) class.
pub struct ContextHelpButton { ptr: *mut c_void }
impl ContextHelpButtonMethods for ContextHelpButton {}
impl BitmapButtonMethods for ContextHelpButton {}
impl ButtonMethods for ContextHelpButton {}
impl ControlMethods for ContextHelpButton {}
impl WindowMethods for ContextHelpButton {}
impl EvtHandlerMethods for ContextHelpButton {}
impl ObjectMethods for ContextHelpButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ContextHelpButton {
    pub fn from(ptr: *mut c_void) -> ContextHelpButton { ContextHelpButton { ptr: ptr } }
    pub fn null() -> ContextHelpButton { ContextHelpButton::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(parent: &T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> ContextHelpButton {
        unsafe { ContextHelpButton::from(wxContextHelpButton_Create(parent.ptr(), id, x, y, w, h, style)) }
    }
}

/// Methods of the wxWidgets' [wxContextHelpButton](http://docs.wxwidgets.org/3.0/classwx_context_help_button.html) class.
pub trait ContextHelpButtonMethods : BitmapButtonMethods {
}

/// Wraps the wxWidgets' [wxControl](http://docs.wxwidgets.org/3.0/classwx_control.html) class.
pub struct Control { ptr: *mut c_void }
impl ControlMethods for Control {}
impl WindowMethods for Control {}
impl EvtHandlerMethods for Control {}
impl ObjectMethods for Control { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Control {
    pub fn from(ptr: *mut c_void) -> Control { Control { ptr: ptr } }
    pub fn null() -> Control { Control::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxControl](http://docs.wxwidgets.org/3.0/classwx_control.html) class.
pub trait ControlMethods : WindowMethods {
    fn command<T: EventMethods>(&self, event: &T) {
        unsafe { wxControl_Command(self.ptr(), event.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxCursor](http://docs.wxwidgets.org/3.0/classwx_cursor.html) class.
pub struct Cursor { ptr: *mut c_void }
impl CursorMethods for Cursor {}
impl BitmapMethods for Cursor {}
impl GDIObjectMethods for Cursor {}
impl ObjectMethods for Cursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Cursor {
    pub fn from(ptr: *mut c_void) -> Cursor { Cursor { ptr: ptr } }
    pub fn null() -> Cursor { Cursor::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCursor](http://docs.wxwidgets.org/3.0/classwx_cursor.html) class.
pub trait CursorMethods : BitmapMethods {
}

/// Wraps the wxWidgets' [wxCustomDataObject](http://docs.wxwidgets.org/3.0/classwx_custom_data_object.html) class.
pub struct CustomDataObject { ptr: *mut c_void }
impl CustomDataObjectMethods for CustomDataObject {}
impl DataObjectSimpleMethods for CustomDataObject {}
impl DataObjectMethods for CustomDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CustomDataObject {
    pub fn from(ptr: *mut c_void) -> CustomDataObject { CustomDataObject { ptr: ptr } }
    pub fn null() -> CustomDataObject { CustomDataObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxCustomDataObject](http://docs.wxwidgets.org/3.0/classwx_custom_data_object.html) class.
pub trait CustomDataObjectMethods : DataObjectSimpleMethods {
}

/// Wraps the wxWidgets' [wxDC](http://docs.wxwidgets.org/3.0/classwx_dc.html) class.
pub struct DC { ptr: *mut c_void }
impl DCMethods for DC {}
impl ObjectMethods for DC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DC {
    pub fn from(ptr: *mut c_void) -> DC { DC { ptr: ptr } }
    pub fn null() -> DC { DC::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDC](http://docs.wxwidgets.org/3.0/classwx_dc.html) class.
pub trait DCMethods : ObjectMethods {
    fn blit<T: DCMethods>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: &T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: c_int) -> c_int {
        unsafe { wxDC_Blit(self.ptr(), xdest, ydest, width, height, source.ptr(), xsrc, ysrc, rop, useMask) }
    }
    fn calcBoundingBox(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CalcBoundingBox(self.ptr(), x, y) }
    }
    fn canDrawBitmap(&self) -> c_int {
        unsafe { wxDC_CanDrawBitmap(self.ptr()) }
    }
    fn canGetTextExtent(&self) -> c_int {
        unsafe { wxDC_CanGetTextExtent(self.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxDC_Clear(self.ptr()) }
    }
    fn computeScaleAndOrigin(&self) {
        unsafe { wxDC_ComputeScaleAndOrigin(self.ptr()) }
    }
    fn crossHair(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CrossHair(self.ptr(), x, y) }
    }
    fn destroyClippingRegion(&self) {
        unsafe { wxDC_DestroyClippingRegion(self.ptr()) }
    }
    fn deviceToLogicalX(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalX(self.ptr(), x) }
    }
    fn deviceToLogicalXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalXRel(self.ptr(), x) }
    }
    fn deviceToLogicalY(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalY(self.ptr(), y) }
    }
    fn deviceToLogicalYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalYRel(self.ptr(), y) }
    }
    fn drawArc(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, xc: c_int, yc: c_int) {
        unsafe { wxDC_DrawArc(self.ptr(), x1, y1, x2, y2, xc, yc) }
    }
    fn drawBitmap<T: BitmapMethods>(&self, bmp: &T, x: c_int, y: c_int, useMask: c_int) {
        unsafe { wxDC_DrawBitmap(self.ptr(), bmp.ptr(), x, y, useMask) }
    }
    fn drawCheckMark(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawCheckMark(self.ptr(), x, y, width, height) }
    }
    fn drawCircle(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { wxDC_DrawCircle(self.ptr(), x, y, radius) }
    }
    fn drawEllipse(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawEllipse(self.ptr(), x, y, width, height) }
    }
    fn drawEllipticArc(&self, x: c_int, y: c_int, w: c_int, h: c_int, sa: c_double, ea: c_double) {
        unsafe { wxDC_DrawEllipticArc(self.ptr(), x, y, w, h, sa, ea) }
    }
    fn drawIcon<T: IconMethods>(&self, icon: &T, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.ptr(), icon.ptr(), x, y) }
    }
    fn drawLabel(&self, str: &str, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        let str = strToString(str);
        unsafe { wxDC_DrawLabel(self.ptr(), str.ptr(), x, y, w, h, align, indexAccel) }
    }
    fn drawLabelBitmap<T: BitmapMethods>(&self, str: &str, bmp: &T, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> Rect {
        let str = strToString(str);
        unsafe { Rect::from(wxDC_DrawLabelBitmap(self.ptr(), str.ptr(), bmp.ptr(), x, y, w, h, align, indexAccel)) }
    }
    fn drawLine(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { wxDC_DrawLine(self.ptr(), x1, y1, x2, y2) }
    }
    fn drawLines(&self, n: c_int, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { wxDC_DrawLines(self.ptr(), n, x, y, xoffset, yoffset) }
    }
    fn drawPoint(&self, x: c_int, y: c_int) {
        unsafe { wxDC_DrawPoint(self.ptr(), x, y) }
    }
    fn drawPolygon(&self, n: c_int, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolygon(self.ptr(), n, x, y, xoffset, yoffset, fillStyle) }
    }
    fn drawPolyPolygon(&self, n: c_int, count: *mut c_void, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolyPolygon(self.ptr(), n, count, x, y, xoffset, yoffset, fillStyle) }
    }
    fn drawRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawRectangle(self.ptr(), x, y, width, height) }
    }
    fn drawRotatedText(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        let text = strToString(text);
        unsafe { wxDC_DrawRotatedText(self.ptr(), text.ptr(), x, y, angle) }
    }
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.ptr(), x, y, width, height, radius) }
    }
    fn drawText(&self, text: &str, x: c_int, y: c_int) {
        let text = strToString(text);
        unsafe { wxDC_DrawText(self.ptr(), text.ptr(), x, y) }
    }
    fn endDoc(&self) {
        unsafe { wxDC_EndDoc(self.ptr()) }
    }
    fn endPage(&self) {
        unsafe { wxDC_EndPage(self.ptr()) }
    }
    fn floodFill<T: ColourMethods>(&self, x: c_int, y: c_int, col: &T, style: c_int) {
        unsafe { wxDC_FloodFill(self.ptr(), x, y, col.ptr(), style) }
    }
    fn getBackground<T: BrushMethods>(&self, _ref: &T) {
        unsafe { wxDC_GetBackground(self.ptr(), _ref.ptr()) }
    }
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.ptr()) }
    }
    fn getBrush<T: BrushMethods>(&self, _ref: &T) {
        unsafe { wxDC_GetBrush(self.ptr(), _ref.ptr()) }
    }
    fn getCharHeight(&self) -> c_int {
        unsafe { wxDC_GetCharHeight(self.ptr()) }
    }
    fn getCharWidth(&self) -> c_int {
        unsafe { wxDC_GetCharWidth(self.ptr()) }
    }
    fn getClippingBox(&self, _x: *mut c_void, _y: *mut c_void, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxDC_GetClippingBox(self.ptr(), _x, _y, _w, _h) }
    }
    fn getDepth(&self) -> c_int {
        unsafe { wxDC_GetDepth(self.ptr()) }
    }
    fn getDeviceOrigin(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxDC_GetDeviceOrigin(self.ptr(), _x, _y) }
    }
    fn getFont<T: FontMethods>(&self, _ref: &T) {
        unsafe { wxDC_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getLogicalFunction(&self) -> c_int {
        unsafe { wxDC_GetLogicalFunction(self.ptr()) }
    }
    fn getLogicalOrigin(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxDC_GetLogicalOrigin(self.ptr(), _x, _y) }
    }
    fn getLogicalScale(&self, _x: *mut c_double, _y: *mut c_double) {
        unsafe { wxDC_GetLogicalScale(self.ptr(), _x, _y) }
    }
    fn getMapMode(&self) -> c_int {
        unsafe { wxDC_GetMapMode(self.ptr()) }
    }
    fn getPPI(&self) -> Size {
        unsafe { Size::from(wxDC_GetPPI(self.ptr())) }
    }
    fn getPen<T: PenMethods>(&self, _ref: &T) {
        unsafe { wxDC_GetPen(self.ptr(), _ref.ptr()) }
    }
    fn getPixel<T: ColourMethods>(&self, x: c_int, y: c_int, col: &T) -> c_int {
        unsafe { wxDC_GetPixel(self.ptr(), x, y, col.ptr()) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size::from(wxDC_GetSize(self.ptr())) }
    }
    fn getSizeMM(&self) -> Size {
        unsafe { Size::from(wxDC_GetSizeMM(self.ptr())) }
    }
    fn getTextBackground<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxDC_GetTextBackground(self.ptr(), _ref.ptr()) }
    }
    fn getTextExtent<T: FontMethods>(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, externalLeading: *mut c_void, theFont: &T) {
        let string = strToString(string);
        unsafe { wxDC_GetTextExtent(self.ptr(), string.ptr(), w, h, descent, externalLeading, theFont.ptr()) }
    }
    fn getMultiLineTextExtent<T: FontMethods>(&self, string: &str, w: *mut c_void, h: *mut c_void, heightLine: *mut c_void, theFont: &T) {
        let string = strToString(string);
        unsafe { wxDC_GetMultiLineTextExtent(self.ptr(), string.ptr(), w, h, heightLine, theFont.ptr()) }
    }
    fn getTextForeground<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxDC_GetTextForeground(self.ptr(), _ref.ptr()) }
    }
    fn getUserScale(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxDC_GetUserScale(self.ptr(), x, y) }
    }
    fn logicalToDeviceX(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceX(self.ptr(), x) }
    }
    fn logicalToDeviceXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceXRel(self.ptr(), x) }
    }
    fn logicalToDeviceY(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceY(self.ptr(), y) }
    }
    fn logicalToDeviceYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceYRel(self.ptr(), y) }
    }
    fn maxX(&self) -> c_int {
        unsafe { wxDC_MaxX(self.ptr()) }
    }
    fn maxY(&self) -> c_int {
        unsafe { wxDC_MaxY(self.ptr()) }
    }
    fn minX(&self) -> c_int {
        unsafe { wxDC_MinX(self.ptr()) }
    }
    fn minY(&self) -> c_int {
        unsafe { wxDC_MinY(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxDC_IsOk(self.ptr()) }
    }
    fn resetBoundingBox(&self) {
        unsafe { wxDC_ResetBoundingBox(self.ptr()) }
    }
    fn setAxisOrientation(&self, xLeftRight: c_int, yBottomUp: c_int) {
        unsafe { wxDC_SetAxisOrientation(self.ptr(), xLeftRight, yBottomUp) }
    }
    fn setBackground<T: BrushMethods>(&self, brush: &T) {
        unsafe { wxDC_SetBackground(self.ptr(), brush.ptr()) }
    }
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.ptr(), mode) }
    }
    fn setBrush<T: BrushMethods>(&self, brush: &T) {
        unsafe { wxDC_SetBrush(self.ptr(), brush.ptr()) }
    }
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.ptr(), x, y, width, height) }
    }
    fn setClippingRegionFromRegion<T: RegionMethods>(&self, region: &T) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.ptr(), region.ptr()) }
    }
    fn setDeviceClippingRegion<T: RegionMethods>(&self, region: &T) {
        unsafe { wxDC_SetDeviceClippingRegion(self.ptr(), region.ptr()) }
    }
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.ptr(), x, y) }
    }
    fn setFont<T: FontMethods>(&self, font: &T) {
        unsafe { wxDC_SetFont(self.ptr(), font.ptr()) }
    }
    fn setLogicalFunction(&self, function: c_int) {
        unsafe { wxDC_SetLogicalFunction(self.ptr(), function) }
    }
    fn setLogicalOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetLogicalOrigin(self.ptr(), x, y) }
    }
    fn setLogicalScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetLogicalScale(self.ptr(), x, y) }
    }
    fn setMapMode(&self, mode: c_int) {
        unsafe { wxDC_SetMapMode(self.ptr(), mode) }
    }
    fn setPalette<T: PaletteMethods>(&self, palette: &T) {
        unsafe { wxDC_SetPalette(self.ptr(), palette.ptr()) }
    }
    fn setPen<T: PenMethods>(&self, pen: &T) {
        unsafe { wxDC_SetPen(self.ptr(), pen.ptr()) }
    }
    fn setTextBackground<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxDC_SetTextBackground(self.ptr(), colour.ptr()) }
    }
    fn setTextForeground<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxDC_SetTextForeground(self.ptr(), colour.ptr()) }
    }
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.ptr(), x, y) }
    }
    fn startDoc(&self, msg: &str) -> c_int {
        let msg = strToString(msg);
        unsafe { wxDC_StartDoc(self.ptr(), msg.ptr()) }
    }
    fn startPage(&self) {
        unsafe { wxDC_StartPage(self.ptr()) }
    }
    fn getUserScaleX(&self) -> c_double {
        unsafe { wxDC_GetUserScaleX(self.ptr()) }
    }
    fn getUserScaleY(&self) -> c_double {
        unsafe { wxDC_GetUserScaleY(self.ptr()) }
    }
    fn getPixel2<T: ColourMethods>(&self, x: c_int, y: c_int, col: &T) {
        unsafe { wxDC_GetPixel2(self.ptr(), x, y, col.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxDCClipper](http://docs.wxwidgets.org/3.0/classwx_dcc_lipper.html) class.
pub struct DCClipper { ptr: *mut c_void }
impl DCClipperMethods for DCClipper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DCClipper {
    pub fn from(ptr: *mut c_void) -> DCClipper { DCClipper { ptr: ptr } }
    pub fn null() -> DCClipper { DCClipper::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDCClipper](http://docs.wxwidgets.org/3.0/classwx_dcc_lipper.html) class.
pub trait DCClipperMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDataFormat](http://docs.wxwidgets.org/3.0/classwx_data_format.html) class.
pub struct DataFormat { ptr: *mut c_void }
impl DataFormatMethods for DataFormat { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataFormat {
    pub fn from(ptr: *mut c_void) -> DataFormat { DataFormat { ptr: ptr } }
    pub fn null() -> DataFormat { DataFormat::from(0 as *mut c_void) }
    
    pub fn newFromId(name: &str) -> DataFormat {
        let name = strToString(name);
        unsafe { DataFormat::from(wxDataFormat_CreateFromId(name.ptr())) }
    }
    pub fn newFromType(typ: c_int) -> DataFormat {
        unsafe { DataFormat::from(wxDataFormat_CreateFromType(typ)) }
    }
}

/// Methods of the wxWidgets' [wxDataFormat](http://docs.wxwidgets.org/3.0/classwx_data_format.html) class.
pub trait DataFormatMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.ptr()) }
    }
    fn getId(&self) -> ~str {
        unsafe { String::from(wxDataFormat_GetId(self.ptr())).to_str() }
    }
    fn getType(&self) -> c_int {
        unsafe { wxDataFormat_GetType(self.ptr()) }
    }
    fn isEqual(&self, other: *mut c_void) -> c_int {
        unsafe { wxDataFormat_IsEqual(self.ptr(), other) }
    }
    fn setId(&self, id: *mut c_void) {
        unsafe { wxDataFormat_SetId(self.ptr(), id) }
    }
    fn setType(&self, typ: c_int) {
        unsafe { wxDataFormat_SetType(self.ptr(), typ) }
    }
}

/// Wraps the wxWidgets' [wxDataObject](http://docs.wxwidgets.org/3.0/classwx_data_object.html) class.
pub struct DataObject { ptr: *mut c_void }
impl DataObjectMethods for DataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataObject {
    pub fn from(ptr: *mut c_void) -> DataObject { DataObject { ptr: ptr } }
    pub fn null() -> DataObject { DataObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDataObject](http://docs.wxwidgets.org/3.0/classwx_data_object.html) class.
pub trait DataObjectMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDataObjectComposite](http://docs.wxwidgets.org/3.0/classwx_data_object_composite.html) class.
pub struct DataObjectComposite { ptr: *mut c_void }
impl DataObjectCompositeMethods for DataObjectComposite {}
impl DataObjectMethods for DataObjectComposite { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataObjectComposite {
    pub fn from(ptr: *mut c_void) -> DataObjectComposite { DataObjectComposite { ptr: ptr } }
    pub fn null() -> DataObjectComposite { DataObjectComposite::from(0 as *mut c_void) }
    
    pub fn new() -> DataObjectComposite {
        unsafe { DataObjectComposite::from(wxDataObjectComposite_Create()) }
    }
}

/// Methods of the wxWidgets' [wxDataObjectComposite](http://docs.wxwidgets.org/3.0/classwx_data_object_composite.html) class.
pub trait DataObjectCompositeMethods : DataObjectMethods {
    fn add(&self, _dat: *mut c_void, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.ptr(), _dat, _preferred) }
    }
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxDataObjectSimple](http://docs.wxwidgets.org/3.0/classwx_data_object_simple.html) class.
pub struct DataObjectSimple { ptr: *mut c_void }
impl DataObjectSimpleMethods for DataObjectSimple {}
impl DataObjectMethods for DataObjectSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataObjectSimple {
    pub fn from(ptr: *mut c_void) -> DataObjectSimple { DataObjectSimple { ptr: ptr } }
    pub fn null() -> DataObjectSimple { DataObjectSimple::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDataObjectSimple](http://docs.wxwidgets.org/3.0/classwx_data_object_simple.html) class.
pub trait DataObjectSimpleMethods : DataObjectMethods {
}

/// Wraps the wxWidgets' [wxDialUpEvent](http://docs.wxwidgets.org/3.0/classwx_dial_up_event.html) class.
pub struct DialUpEvent { ptr: *mut c_void }
impl DialUpEventMethods for DialUpEvent {}
impl EventMethods for DialUpEvent {}
impl ObjectMethods for DialUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DialUpEvent {
    pub fn from(ptr: *mut c_void) -> DialUpEvent { DialUpEvent { ptr: ptr } }
    pub fn null() -> DialUpEvent { DialUpEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDialUpEvent](http://docs.wxwidgets.org/3.0/classwx_dial_up_event.html) class.
pub trait DialUpEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxDialUpManager](http://docs.wxwidgets.org/3.0/classwx_dial_up_manager.html) class.
pub struct DialUpManager { ptr: *mut c_void }
impl DialUpManagerMethods for DialUpManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DialUpManager {
    pub fn from(ptr: *mut c_void) -> DialUpManager { DialUpManager { ptr: ptr } }
    pub fn null() -> DialUpManager { DialUpManager::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDialUpManager](http://docs.wxwidgets.org/3.0/classwx_dial_up_manager.html) class.
pub trait DialUpManagerMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDialog](http://docs.wxwidgets.org/3.0/classwx_dialog.html) class.
pub struct Dialog { ptr: *mut c_void }
impl DialogMethods for Dialog {}
impl TopLevelWindowMethods for Dialog {}
impl WindowMethods for Dialog {}
impl EvtHandlerMethods for Dialog {}
impl ObjectMethods for Dialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Dialog {
    pub fn from(ptr: *mut c_void) -> Dialog { Dialog { ptr: ptr } }
    pub fn null() -> Dialog { Dialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Dialog {
        let _txt = strToString(_txt);
        unsafe { Dialog::from(wxDialog_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxDialog](http://docs.wxwidgets.org/3.0/classwx_dialog.html) class.
pub trait DialogMethods : TopLevelWindowMethods {
    fn endModal(&self, retCode: c_int) {
        unsafe { wxDialog_EndModal(self.ptr(), retCode) }
    }
    fn getReturnCode(&self) -> c_int {
        unsafe { wxDialog_GetReturnCode(self.ptr()) }
    }
    fn isModal(&self) -> c_int {
        unsafe { wxDialog_IsModal(self.ptr()) }
    }
    fn setReturnCode(&self, returnCode: c_int) {
        unsafe { wxDialog_SetReturnCode(self.ptr(), returnCode) }
    }
    fn showModal(&self) -> c_int {
        unsafe { wxDialog_ShowModal(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxDirDialog](http://docs.wxwidgets.org/3.0/classwx_dir_dialog.html) class.
pub struct DirDialog { ptr: *mut c_void }
impl DirDialogMethods for DirDialog {}
impl DialogMethods for DirDialog {}
impl TopLevelWindowMethods for DirDialog {}
impl WindowMethods for DirDialog {}
impl EvtHandlerMethods for DirDialog {}
impl ObjectMethods for DirDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DirDialog {
    pub fn from(ptr: *mut c_void) -> DirDialog { DirDialog { ptr: ptr } }
    pub fn null() -> DirDialog { DirDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _msg: &str, _dir: &str, _lft: c_int, _top: c_int, _stl: c_int) -> DirDialog {
        let _msg = strToString(_msg);
        let _dir = strToString(_dir);
        unsafe { DirDialog::from(wxDirDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _lft, _top, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxDirDialog](http://docs.wxwidgets.org/3.0/classwx_dir_dialog.html) class.
pub trait DirDialogMethods : DialogMethods {
    fn getMessage(&self) -> ~str {
        unsafe { String::from(wxDirDialog_GetMessage(self.ptr())).to_str() }
    }
    fn getPath(&self) -> ~str {
        unsafe { String::from(wxDirDialog_GetPath(self.ptr())).to_str() }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.ptr()) }
    }
    fn setMessage(&self, msg: &str) {
        let msg = strToString(msg);
        unsafe { wxDirDialog_SetMessage(self.ptr(), msg.ptr()) }
    }
    fn setPath(&self, pth: &str) {
        let pth = strToString(pth);
        unsafe { wxDirDialog_SetPath(self.ptr(), pth.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.ptr(), style) }
    }
}

/// Wraps the wxWidgets' [wxDocChildFrame](http://docs.wxwidgets.org/3.0/classwx_doc_child_frame.html) class.
pub struct DocChildFrame { ptr: *mut c_void }
impl DocChildFrameMethods for DocChildFrame {}
impl FrameMethods for DocChildFrame {}
impl TopLevelWindowMethods for DocChildFrame {}
impl WindowMethods for DocChildFrame {}
impl EvtHandlerMethods for DocChildFrame {}
impl ObjectMethods for DocChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocChildFrame {
    pub fn from(ptr: *mut c_void) -> DocChildFrame { DocChildFrame { ptr: ptr } }
    pub fn null() -> DocChildFrame { DocChildFrame::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocChildFrame](http://docs.wxwidgets.org/3.0/classwx_doc_child_frame.html) class.
pub trait DocChildFrameMethods : FrameMethods {
}

/// Wraps the wxWidgets' [wxDocMDIChildFrame](http://docs.wxwidgets.org/3.0/classwx_doc_mdic_hild_frame.html) class.
pub struct DocMDIChildFrame { ptr: *mut c_void }
impl DocMDIChildFrameMethods for DocMDIChildFrame {}
impl MDIChildFrameMethods for DocMDIChildFrame {}
impl FrameMethods for DocMDIChildFrame {}
impl TopLevelWindowMethods for DocMDIChildFrame {}
impl WindowMethods for DocMDIChildFrame {}
impl EvtHandlerMethods for DocMDIChildFrame {}
impl ObjectMethods for DocMDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocMDIChildFrame {
    pub fn from(ptr: *mut c_void) -> DocMDIChildFrame { DocMDIChildFrame { ptr: ptr } }
    pub fn null() -> DocMDIChildFrame { DocMDIChildFrame::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocMDIChildFrame](http://docs.wxwidgets.org/3.0/classwx_doc_mdic_hild_frame.html) class.
pub trait DocMDIChildFrameMethods : MDIChildFrameMethods {
}

/// Wraps the wxWidgets' [wxDocMDIParentFrame](http://docs.wxwidgets.org/3.0/classwx_doc_mdip_arent_frame.html) class.
pub struct DocMDIParentFrame { ptr: *mut c_void }
impl DocMDIParentFrameMethods for DocMDIParentFrame {}
impl MDIParentFrameMethods for DocMDIParentFrame {}
impl FrameMethods for DocMDIParentFrame {}
impl TopLevelWindowMethods for DocMDIParentFrame {}
impl WindowMethods for DocMDIParentFrame {}
impl EvtHandlerMethods for DocMDIParentFrame {}
impl ObjectMethods for DocMDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocMDIParentFrame {
    pub fn from(ptr: *mut c_void) -> DocMDIParentFrame { DocMDIParentFrame { ptr: ptr } }
    pub fn null() -> DocMDIParentFrame { DocMDIParentFrame::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocMDIParentFrame](http://docs.wxwidgets.org/3.0/classwx_doc_mdip_arent_frame.html) class.
pub trait DocMDIParentFrameMethods : MDIParentFrameMethods {
}

/// Wraps the wxWidgets' [wxDocManager](http://docs.wxwidgets.org/3.0/classwx_doc_manager.html) class.
pub struct DocManager { ptr: *mut c_void }
impl DocManagerMethods for DocManager {}
impl EvtHandlerMethods for DocManager {}
impl ObjectMethods for DocManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocManager {
    pub fn from(ptr: *mut c_void) -> DocManager { DocManager { ptr: ptr } }
    pub fn null() -> DocManager { DocManager::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocManager](http://docs.wxwidgets.org/3.0/classwx_doc_manager.html) class.
pub trait DocManagerMethods : EvtHandlerMethods {
}

/// Wraps the wxWidgets' [wxDocParentFrame](http://docs.wxwidgets.org/3.0/classwx_doc_parent_frame.html) class.
pub struct DocParentFrame { ptr: *mut c_void }
impl DocParentFrameMethods for DocParentFrame {}
impl FrameMethods for DocParentFrame {}
impl TopLevelWindowMethods for DocParentFrame {}
impl WindowMethods for DocParentFrame {}
impl EvtHandlerMethods for DocParentFrame {}
impl ObjectMethods for DocParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocParentFrame {
    pub fn from(ptr: *mut c_void) -> DocParentFrame { DocParentFrame { ptr: ptr } }
    pub fn null() -> DocParentFrame { DocParentFrame::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocParentFrame](http://docs.wxwidgets.org/3.0/classwx_doc_parent_frame.html) class.
pub trait DocParentFrameMethods : FrameMethods {
}

/// Wraps the wxWidgets' [wxDocTemplate](http://docs.wxwidgets.org/3.0/classwx_doc_template.html) class.
pub struct DocTemplate { ptr: *mut c_void }
impl DocTemplateMethods for DocTemplate {}
impl ObjectMethods for DocTemplate { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocTemplate {
    pub fn from(ptr: *mut c_void) -> DocTemplate { DocTemplate { ptr: ptr } }
    pub fn null() -> DocTemplate { DocTemplate::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocTemplate](http://docs.wxwidgets.org/3.0/classwx_doc_template.html) class.
pub trait DocTemplateMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxDocument](http://docs.wxwidgets.org/3.0/classwx_document.html) class.
pub struct Document { ptr: *mut c_void }
impl DocumentMethods for Document {}
impl EvtHandlerMethods for Document {}
impl ObjectMethods for Document { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Document {
    pub fn from(ptr: *mut c_void) -> Document { Document { ptr: ptr } }
    pub fn null() -> Document { Document::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDocument](http://docs.wxwidgets.org/3.0/classwx_document.html) class.
pub trait DocumentMethods : EvtHandlerMethods {
}

/// Wraps the wxWidgets' [wxDragImage](http://docs.wxwidgets.org/3.0/classwx_drag_image.html) class.
pub struct DragImage { ptr: *mut c_void }
impl DragImageMethods for DragImage {}
impl ObjectMethods for DragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DragImage {
    pub fn from(ptr: *mut c_void) -> DragImage { DragImage { ptr: ptr } }
    pub fn null() -> DragImage { DragImage::from(0 as *mut c_void) }
    
    pub fn new<T: BitmapMethods>(image: &T, x: c_int, y: c_int) -> DragImage {
        unsafe { DragImage::from(wxDragImage_Create(image.ptr(), x, y)) }
    }
}

/// Methods of the wxWidgets' [wxDragImage](http://docs.wxwidgets.org/3.0/classwx_drag_image.html) class.
pub trait DragImageMethods : ObjectMethods {
    fn beginDragFullScreen<T: WindowMethods, U: RectMethods>(&self, x_pos: c_int, y_pos: c_int, window: &T, fullScreen: c_int, rect: &U) -> c_int {
        unsafe { wxDragImage_BeginDragFullScreen(self.ptr(), x_pos, y_pos, window.ptr(), fullScreen, rect.ptr()) }
    }
    fn beginDrag<T: WindowMethods, U: WindowMethods>(&self, x: c_int, y: c_int, window: &T, boundingWindow: &U) -> c_int {
        unsafe { wxDragImage_BeginDrag(self.ptr(), x, y, window.ptr(), boundingWindow.ptr()) }
    }
    fn endDrag(&self) {
        unsafe { wxDragImage_EndDrag(self.ptr()) }
    }
    fn hide(&self) -> c_int {
        unsafe { wxDragImage_Hide(self.ptr()) }
    }
    fn move(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxDragImage_Move(self.ptr(), x, y) }
    }
    fn show(&self) -> c_int {
        unsafe { wxDragImage_Show(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxDrawControl](http://docs.wxwidgets.org/3.0/classwx_draw_control.html) class.
pub struct DrawControl { ptr: *mut c_void }
impl DrawControlMethods for DrawControl {}
impl ControlMethods for DrawControl {}
impl WindowMethods for DrawControl {}
impl EvtHandlerMethods for DrawControl {}
impl ObjectMethods for DrawControl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DrawControl {
    pub fn from(ptr: *mut c_void) -> DrawControl { DrawControl { ptr: ptr } }
    pub fn null() -> DrawControl { DrawControl::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> DrawControl {
        unsafe { DrawControl::from(wxDrawControl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxDrawControl](http://docs.wxwidgets.org/3.0/classwx_draw_control.html) class.
pub trait DrawControlMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxDrawWindow](http://docs.wxwidgets.org/3.0/classwx_draw_window.html) class.
pub struct DrawWindow { ptr: *mut c_void }
impl DrawWindowMethods for DrawWindow {}
impl WindowMethods for DrawWindow {}
impl EvtHandlerMethods for DrawWindow {}
impl ObjectMethods for DrawWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DrawWindow {
    pub fn from(ptr: *mut c_void) -> DrawWindow { DrawWindow { ptr: ptr } }
    pub fn null() -> DrawWindow { DrawWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> DrawWindow {
        unsafe { DrawWindow::from(wxDrawWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxDrawWindow](http://docs.wxwidgets.org/3.0/classwx_draw_window.html) class.
pub trait DrawWindowMethods : WindowMethods {
}

/// Wraps the wxWidgets' [wxDropFilesEvent](http://docs.wxwidgets.org/3.0/classwx_drop_files_event.html) class.
pub struct DropFilesEvent { ptr: *mut c_void }
impl DropFilesEventMethods for DropFilesEvent {}
impl EventMethods for DropFilesEvent {}
impl ObjectMethods for DropFilesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DropFilesEvent {
    pub fn from(ptr: *mut c_void) -> DropFilesEvent { DropFilesEvent { ptr: ptr } }
    pub fn null() -> DropFilesEvent { DropFilesEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDropFilesEvent](http://docs.wxwidgets.org/3.0/classwx_drop_files_event.html) class.
pub trait DropFilesEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxDropSource](http://docs.wxwidgets.org/3.0/classwx_drop_source.html) class.
pub struct DropSource { ptr: *mut c_void }
impl DropSourceMethods for DropSource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DropSource {
    pub fn from(ptr: *mut c_void) -> DropSource { DropSource { ptr: ptr } }
    pub fn null() -> DropSource { DropSource::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDropSource](http://docs.wxwidgets.org/3.0/classwx_drop_source.html) class.
pub trait DropSourceMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxDropTarget](http://docs.wxwidgets.org/3.0/classwx_drop_target.html) class.
/// Rather use the wxRust-specific [RustDropTarget](struct.RustDropTarget.html) class.
pub struct DropTarget { ptr: *mut c_void }
impl DropTargetMethods for DropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DropTarget {
    pub fn from(ptr: *mut c_void) -> DropTarget { DropTarget { ptr: ptr } }
    pub fn null() -> DropTarget { DropTarget::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDropTarget](http://docs.wxwidgets.org/3.0/classwx_drop_target.html) class.
pub trait DropTargetMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.ptr()) }
    }
    fn setDataObject<T: DataObjectMethods>(&self, _dat: &T) {
        unsafe { wxDropTarget_SetDataObject(self.ptr(), _dat.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxEraseEvent](http://docs.wxwidgets.org/3.0/classwx_erase_event.html) class.
pub struct EraseEvent { ptr: *mut c_void }
impl EraseEventMethods for EraseEvent {}
impl EventMethods for EraseEvent {}
impl ObjectMethods for EraseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EraseEvent {
    pub fn from(ptr: *mut c_void) -> EraseEvent { EraseEvent { ptr: ptr } }
    pub fn null() -> EraseEvent { EraseEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxEraseEvent](http://docs.wxwidgets.org/3.0/classwx_erase_event.html) class.
pub trait EraseEventMethods : EventMethods {
    fn getDC(&self) -> DC {
        unsafe { DC::from(wxEraseEvent_GetDC(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxEvent](http://docs.wxwidgets.org/3.0/classwx_event.html) class.
pub struct Event { ptr: *mut c_void }
impl EventMethods for Event {}
impl ObjectMethods for Event { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Event {
    pub fn from(ptr: *mut c_void) -> Event { Event { ptr: ptr } }
    pub fn null() -> Event { Event::from(0 as *mut c_void) }
    
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

/// Methods of the wxWidgets' [wxEvent](http://docs.wxwidgets.org/3.0/classwx_event.html) class.
pub trait EventMethods : ObjectMethods {
    fn copyObject(&self, object_dest: *mut c_void) {
        unsafe { wxEvent_CopyObject(self.ptr(), object_dest) }
    }
    fn getEventObject(&self) -> Object {
        unsafe { Object::from(wxEvent_GetEventObject(self.ptr())) }
    }
    fn getEventType(&self) -> c_int {
        unsafe { wxEvent_GetEventType(self.ptr()) }
    }
    fn getId(&self) -> c_int {
        unsafe { wxEvent_GetId(self.ptr()) }
    }
    fn getSkipped(&self) -> c_int {
        unsafe { wxEvent_GetSkipped(self.ptr()) }
    }
    fn getTimestamp(&self) -> c_int {
        unsafe { wxEvent_GetTimestamp(self.ptr()) }
    }
    fn isCommandEvent(&self) -> c_int {
        unsafe { wxEvent_IsCommandEvent(self.ptr()) }
    }
    fn setEventObject<T: ObjectMethods>(&self, obj: &T) {
        unsafe { wxEvent_SetEventObject(self.ptr(), obj.ptr()) }
    }
    fn setEventType(&self, typ: c_int) {
        unsafe { wxEvent_SetEventType(self.ptr(), typ) }
    }
    fn setId(&self, Id: c_int) {
        unsafe { wxEvent_SetId(self.ptr(), Id) }
    }
    fn setTimestamp(&self, ts: c_int) {
        unsafe { wxEvent_SetTimestamp(self.ptr(), ts) }
    }
    fn skip(&self) {
        unsafe { wxEvent_Skip(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxEvtHandler](http://docs.wxwidgets.org/3.0/classwx_evt_handler.html) class.
pub struct EvtHandler { ptr: *mut c_void }
impl EvtHandlerMethods for EvtHandler {}
impl ObjectMethods for EvtHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EvtHandler {
    pub fn from(ptr: *mut c_void) -> EvtHandler { EvtHandler { ptr: ptr } }
    pub fn null() -> EvtHandler { EvtHandler::from(0 as *mut c_void) }
    
    pub fn new() -> EvtHandler {
        unsafe { EvtHandler::from(wxEvtHandler_Create()) }
    }
}

/// Methods of the wxWidgets' [wxEvtHandler](http://docs.wxwidgets.org/3.0/classwx_evt_handler.html) class.
pub trait EvtHandlerMethods : ObjectMethods {
    fn addPendingEvent<T: EventMethods>(&self, event: &T) {
        unsafe { wxEvtHandler_AddPendingEvent(self.ptr(), event.ptr()) }
    }
    fn connect(&self, first: c_int, last: c_int, type_: c_int, data: *mut c_void) -> c_int {
        unsafe { wxEvtHandler_Connect(self.ptr(), first, last, type_, data) }
    }
    fn disconnect(&self, first: c_int, last: c_int, type_: c_int, id: c_int) -> c_int {
        unsafe { wxEvtHandler_Disconnect(self.ptr(), first, last, type_, id) }
    }
    fn getEvtHandlerEnabled(&self) -> c_int {
        unsafe { wxEvtHandler_GetEvtHandlerEnabled(self.ptr()) }
    }
    fn getNextHandler(&self) -> EvtHandler {
        unsafe { EvtHandler::from(wxEvtHandler_GetNextHandler(self.ptr())) }
    }
    fn getPreviousHandler(&self) -> EvtHandler {
        unsafe { EvtHandler::from(wxEvtHandler_GetPreviousHandler(self.ptr())) }
    }
    fn processEvent<T: EventMethods>(&self, event: &T) -> c_int {
        unsafe { wxEvtHandler_ProcessEvent(self.ptr(), event.ptr()) }
    }
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.ptr()) }
    }
    fn setEvtHandlerEnabled(&self, enabled: c_int) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self.ptr(), enabled) }
    }
    fn setNextHandler<T: EvtHandlerMethods>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetNextHandler(self.ptr(), handler.ptr()) }
    }
    fn setPreviousHandler<T: EvtHandlerMethods>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.ptr(), handler.ptr()) }
    }
    fn getClosure(&self, id: c_int, type_: c_int) -> Closure {
        unsafe { Closure::from(wxEvtHandler_GetClosure(self.ptr(), id, type_)) }
    }
}

/// Wraps the wxWidgets' [wxFileDataObject](http://docs.wxwidgets.org/3.0/classwx_file_data_object.html) class.
pub struct FileDataObject { ptr: *mut c_void }
impl FileDataObjectMethods for FileDataObject {}
impl DataObjectSimpleMethods for FileDataObject {}
impl DataObjectMethods for FileDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileDataObject {
    pub fn from(ptr: *mut c_void) -> FileDataObject { FileDataObject { ptr: ptr } }
    pub fn null() -> FileDataObject { FileDataObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileDataObject](http://docs.wxwidgets.org/3.0/classwx_file_data_object.html) class.
pub trait FileDataObjectMethods : DataObjectSimpleMethods {
}

/// Wraps the wxWidgets' [wxFileDialog](http://docs.wxwidgets.org/3.0/classwx_file_dialog.html) class.
pub struct FileDialog { ptr: *mut c_void }
impl FileDialogMethods for FileDialog {}
impl DialogMethods for FileDialog {}
impl TopLevelWindowMethods for FileDialog {}
impl WindowMethods for FileDialog {}
impl EvtHandlerMethods for FileDialog {}
impl ObjectMethods for FileDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileDialog {
    pub fn from(ptr: *mut c_void) -> FileDialog { FileDialog { ptr: ptr } }
    pub fn null() -> FileDialog { FileDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _msg: &str, _dir: &str, _fle: &str, _wcd: &str, _lft: c_int, _top: c_int, _stl: c_int) -> FileDialog {
        let _msg = strToString(_msg);
        let _dir = strToString(_dir);
        let _fle = strToString(_fle);
        let _wcd = strToString(_wcd);
        unsafe { FileDialog::from(wxFileDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _fle.ptr(), _wcd.ptr(), _lft, _top, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxFileDialog](http://docs.wxwidgets.org/3.0/classwx_file_dialog.html) class.
pub trait FileDialogMethods : DialogMethods {
    fn getDirectory(&self) -> ~str {
        unsafe { String::from(wxFileDialog_GetDirectory(self.ptr())).to_str() }
    }
    fn getFilename(&self) -> ~str {
        unsafe { String::from(wxFileDialog_GetFilename(self.ptr())).to_str() }
    }
    fn getFilenames(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetFilenames(self.ptr(), paths) }
    }
    fn getFilterIndex(&self) -> c_int {
        unsafe { wxFileDialog_GetFilterIndex(self.ptr()) }
    }
    fn getMessage(&self) -> ~str {
        unsafe { String::from(wxFileDialog_GetMessage(self.ptr())).to_str() }
    }
    fn getPath(&self) -> ~str {
        unsafe { String::from(wxFileDialog_GetPath(self.ptr())).to_str() }
    }
    fn getPaths(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetPaths(self.ptr(), paths) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxFileDialog_GetStyle(self.ptr()) }
    }
    fn getWildcard(&self) -> ~str {
        unsafe { String::from(wxFileDialog_GetWildcard(self.ptr())).to_str() }
    }
    fn setDirectory(&self, dir: &str) {
        let dir = strToString(dir);
        unsafe { wxFileDialog_SetDirectory(self.ptr(), dir.ptr()) }
    }
    fn setFilename(&self, name: &str) {
        let name = strToString(name);
        unsafe { wxFileDialog_SetFilename(self.ptr(), name.ptr()) }
    }
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.ptr(), filterIndex) }
    }
    fn setMessage(&self, message: &str) {
        let message = strToString(message);
        unsafe { wxFileDialog_SetMessage(self.ptr(), message.ptr()) }
    }
    fn setPath(&self, path: &str) {
        let path = strToString(path);
        unsafe { wxFileDialog_SetPath(self.ptr(), path.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.ptr(), style) }
    }
    fn setWildcard(&self, wildCard: &str) {
        let wildCard = strToString(wildCard);
        unsafe { wxFileDialog_SetWildcard(self.ptr(), wildCard.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFileDropTarget](http://docs.wxwidgets.org/3.0/classwx_file_drop_target.html) class.
/// Rather use the wxRust-specific [RustFileDropTarget](struct.RustFileDropTarget.html) class.
pub struct FileDropTarget { ptr: *mut c_void }
impl FileDropTargetMethods for FileDropTarget {}
impl DropTargetMethods for FileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileDropTarget {
    pub fn from(ptr: *mut c_void) -> FileDropTarget { FileDropTarget { ptr: ptr } }
    pub fn null() -> FileDropTarget { FileDropTarget::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileDropTarget](http://docs.wxwidgets.org/3.0/classwx_file_drop_target.html) class.
pub trait FileDropTargetMethods : DropTargetMethods {
}

/// Wraps the wxWidgets' [wxFileHistory](http://docs.wxwidgets.org/3.0/classwx_file_history.html) class.
pub struct FileHistory { ptr: *mut c_void }
impl FileHistoryMethods for FileHistory {}
impl ObjectMethods for FileHistory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileHistory {
    pub fn from(ptr: *mut c_void) -> FileHistory { FileHistory { ptr: ptr } }
    pub fn null() -> FileHistory { FileHistory::from(0 as *mut c_void) }
    
    pub fn new(maxFiles: c_int) -> FileHistory {
        unsafe { FileHistory::from(wxFileHistory_Create(maxFiles)) }
    }
}

/// Methods of the wxWidgets' [wxFileHistory](http://docs.wxwidgets.org/3.0/classwx_file_history.html) class.
pub trait FileHistoryMethods : ObjectMethods {
    fn addFileToHistory(&self, file: &str) {
        let file = strToString(file);
        unsafe { wxFileHistory_AddFileToHistory(self.ptr(), file.ptr()) }
    }
    fn addFilesToMenu<T: MenuMethods>(&self, menu: &T) {
        unsafe { wxFileHistory_AddFilesToMenu(self.ptr(), menu.ptr()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxFileHistory_GetCount(self.ptr()) }
    }
    fn getHistoryFile(&self, i: c_int) -> ~str {
        unsafe { String::from(wxFileHistory_GetHistoryFile(self.ptr(), i)).to_str() }
    }
    fn getMaxFiles(&self) -> c_int {
        unsafe { wxFileHistory_GetMaxFiles(self.ptr()) }
    }
    fn getMenus(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFileHistory_GetMenus(self.ptr(), _ref) }
    }
    fn load<T: ConfigBaseMethods>(&self, config: &T) {
        unsafe { wxFileHistory_Load(self.ptr(), config.ptr()) }
    }
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.ptr(), i) }
    }
    fn removeMenu<T: MenuMethods>(&self, menu: &T) {
        unsafe { wxFileHistory_RemoveMenu(self.ptr(), menu.ptr()) }
    }
    fn save<T: ConfigBaseMethods>(&self, config: &T) {
        unsafe { wxFileHistory_Save(self.ptr(), config.ptr()) }
    }
    fn useMenu<T: MenuMethods>(&self, menu: &T) {
        unsafe { wxFileHistory_UseMenu(self.ptr(), menu.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFileType](http://docs.wxwidgets.org/3.0/classwx_file_type.html) class.
pub struct FileType { ptr: *mut c_void }
impl FileTypeMethods for FileType { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileType {
    pub fn from(ptr: *mut c_void) -> FileType { FileType { ptr: ptr } }
    pub fn null() -> FileType { FileType::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFileType](http://docs.wxwidgets.org/3.0/classwx_file_type.html) class.
pub trait FileTypeMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.ptr()) }
    }
    fn expandCommand(&self, _cmd: *mut c_void, _params: *mut c_void) -> ~str {
        unsafe { String::from(wxFileType_ExpandCommand(self.ptr(), _cmd, _params)).to_str() }
    }
    fn getDescription(&self) -> ~str {
        unsafe { String::from(wxFileType_GetDescription(self.ptr())).to_str() }
    }
    fn getExtensions<T: ListMethods>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetExtensions(self.ptr(), _lst.ptr()) }
    }
    fn getIcon<T: IconMethods>(&self, icon: &T) -> c_int {
        unsafe { wxFileType_GetIcon(self.ptr(), icon.ptr()) }
    }
    fn getMimeType(&self) -> ~str {
        unsafe { String::from(wxFileType_GetMimeType(self.ptr())).to_str() }
    }
    fn getMimeTypes<T: ListMethods>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetMimeTypes(self.ptr(), _lst.ptr()) }
    }
    fn getOpenCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.ptr(), _buf, _params) }
    }
    fn getPrintCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.ptr(), _buf, _params) }
    }
}

/// Wraps the wxWidgets' [wxFindDialogEvent](http://docs.wxwidgets.org/3.0/classwx_find_dialog_event.html) class.
pub struct FindDialogEvent { ptr: *mut c_void }
impl FindDialogEventMethods for FindDialogEvent {}
impl CommandEventMethods for FindDialogEvent {}
impl EventMethods for FindDialogEvent {}
impl ObjectMethods for FindDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FindDialogEvent {
    pub fn from(ptr: *mut c_void) -> FindDialogEvent { FindDialogEvent { ptr: ptr } }
    pub fn null() -> FindDialogEvent { FindDialogEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFindDialogEvent](http://docs.wxwidgets.org/3.0/classwx_find_dialog_event.html) class.
pub trait FindDialogEventMethods : CommandEventMethods {
    fn getFindString(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFindDialogEvent_GetFindString(self.ptr(), _ref) }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxFindDialogEvent_GetFlags(self.ptr()) }
    }
    fn getReplaceString(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFindDialogEvent_GetReplaceString(self.ptr(), _ref) }
    }
}

/// Wraps the wxWidgets' [wxFindReplaceData](http://docs.wxwidgets.org/3.0/classwx_find_replace_data.html) class.
pub struct FindReplaceData { ptr: *mut c_void }
impl FindReplaceDataMethods for FindReplaceData {}
impl ObjectMethods for FindReplaceData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FindReplaceData {
    pub fn from(ptr: *mut c_void) -> FindReplaceData { FindReplaceData { ptr: ptr } }
    pub fn null() -> FindReplaceData { FindReplaceData::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> FindReplaceData {
        unsafe { FindReplaceData::from(wxFindReplaceData_Create(flags)) }
    }
    pub fn newDefault() -> FindReplaceData {
        unsafe { FindReplaceData::from(wxFindReplaceData_CreateDefault()) }
    }
}

/// Methods of the wxWidgets' [wxFindReplaceData](http://docs.wxwidgets.org/3.0/classwx_find_replace_data.html) class.
pub trait FindReplaceDataMethods : ObjectMethods {
    fn getFindString(&self) -> ~str {
        unsafe { String::from(wxFindReplaceData_GetFindString(self.ptr())).to_str() }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.ptr()) }
    }
    fn getReplaceString(&self) -> ~str {
        unsafe { String::from(wxFindReplaceData_GetReplaceString(self.ptr())).to_str() }
    }
    fn setFindString(&self, str: &str) {
        let str = strToString(str);
        unsafe { wxFindReplaceData_SetFindString(self.ptr(), str.ptr()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.ptr(), flags) }
    }
    fn setReplaceString(&self, str: &str) {
        let str = strToString(str);
        unsafe { wxFindReplaceData_SetReplaceString(self.ptr(), str.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFindReplaceDialog](http://docs.wxwidgets.org/3.0/classwx_find_replace_dialog.html) class.
pub struct FindReplaceDialog { ptr: *mut c_void }
impl FindReplaceDialogMethods for FindReplaceDialog {}
impl DialogMethods for FindReplaceDialog {}
impl TopLevelWindowMethods for FindReplaceDialog {}
impl WindowMethods for FindReplaceDialog {}
impl EvtHandlerMethods for FindReplaceDialog {}
impl ObjectMethods for FindReplaceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FindReplaceDialog {
    pub fn from(ptr: *mut c_void) -> FindReplaceDialog { FindReplaceDialog { ptr: ptr } }
    pub fn null() -> FindReplaceDialog { FindReplaceDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: FindReplaceDataMethods>(parent: &T, data: &U, title: &str, style: c_int) -> FindReplaceDialog {
        let title = strToString(title);
        unsafe { FindReplaceDialog::from(wxFindReplaceDialog_Create(parent.ptr(), data.ptr(), title.ptr(), style)) }
    }
}

/// Methods of the wxWidgets' [wxFindReplaceDialog](http://docs.wxwidgets.org/3.0/classwx_find_replace_dialog.html) class.
pub trait FindReplaceDialogMethods : DialogMethods {
    fn getData(&self) -> FindReplaceData {
        unsafe { FindReplaceData::from(wxFindReplaceDialog_GetData(self.ptr())) }
    }
    fn setData<T: FindReplaceDataMethods>(&self, data: &T) {
        unsafe { wxFindReplaceDialog_SetData(self.ptr(), data.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFlexGridSizer](http://docs.wxwidgets.org/3.0/classwx_flex_grid_sizer.html) class.
pub struct FlexGridSizer { ptr: *mut c_void }
impl FlexGridSizerMethods for FlexGridSizer {}
impl GridSizerMethods for FlexGridSizer {}
impl SizerMethods for FlexGridSizer {}
impl ObjectMethods for FlexGridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FlexGridSizer {
    pub fn from(ptr: *mut c_void) -> FlexGridSizer { FlexGridSizer { ptr: ptr } }
    pub fn null() -> FlexGridSizer { FlexGridSizer::from(0 as *mut c_void) }
    
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> FlexGridSizer {
        unsafe { FlexGridSizer::from(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

/// Methods of the wxWidgets' [wxFlexGridSizer](http://docs.wxwidgets.org/3.0/classwx_flex_grid_sizer.html) class.
pub trait FlexGridSizerMethods : GridSizerMethods {
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self.ptr(), idx) }
    }
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self.ptr(), idx) }
    }
    fn removeGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableCol(self.ptr(), idx) }
    }
    fn removeGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableRow(self.ptr(), idx) }
    }
}

/// Wraps the wxWidgets' [wxFocusEvent](http://docs.wxwidgets.org/3.0/classwx_focus_event.html) class.
pub struct FocusEvent { ptr: *mut c_void }
impl FocusEventMethods for FocusEvent {}
impl EventMethods for FocusEvent {}
impl ObjectMethods for FocusEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FocusEvent {
    pub fn from(ptr: *mut c_void) -> FocusEvent { FocusEvent { ptr: ptr } }
    pub fn null() -> FocusEvent { FocusEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFocusEvent](http://docs.wxwidgets.org/3.0/classwx_focus_event.html) class.
pub trait FocusEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxFont](http://docs.wxwidgets.org/3.0/classwx_font.html) class.
pub struct Font { ptr: *mut c_void }
impl FontMethods for Font {}
impl GDIObjectMethods for Font {}
impl ObjectMethods for Font { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Font {
    pub fn from(ptr: *mut c_void) -> Font { Font { ptr: ptr } }
    pub fn null() -> Font { Font::from(0 as *mut c_void) }
    
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: c_int, face: &str, enc: c_int) -> Font {
        let face = strToString(face);
        unsafe { Font::from(wxFont_Create(pointSize, family, style, weight, underlined, face.ptr(), enc)) }
    }
    pub fn newFromStock(id: c_int) -> Font {
        unsafe { Font::from(wxFont_CreateFromStock(id)) }
    }
    pub fn newDefault() -> Font {
        unsafe { Font::from(wxFont_CreateDefault()) }
    }
}

/// Methods of the wxWidgets' [wxFont](http://docs.wxwidgets.org/3.0/classwx_font.html) class.
pub trait FontMethods : GDIObjectMethods {
    fn getDefaultEncoding(&self) -> c_int {
        unsafe { wxFont_GetDefaultEncoding(self.ptr()) }
    }
    fn getEncoding(&self) -> c_int {
        unsafe { wxFont_GetEncoding(self.ptr()) }
    }
    fn getFaceName(&self) -> ~str {
        unsafe { String::from(wxFont_GetFaceName(self.ptr())).to_str() }
    }
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.ptr()) }
    }
    fn getFamilyString(&self) -> ~str {
        unsafe { String::from(wxFont_GetFamilyString(self.ptr())).to_str() }
    }
    fn getPointSize(&self) -> c_int {
        unsafe { wxFont_GetPointSize(self.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxFont_GetStyle(self.ptr()) }
    }
    fn getStyleString(&self) -> ~str {
        unsafe { String::from(wxFont_GetStyleString(self.ptr())).to_str() }
    }
    fn getUnderlined(&self) -> c_int {
        unsafe { wxFont_GetUnderlined(self.ptr()) }
    }
    fn getWeight(&self) -> c_int {
        unsafe { wxFont_GetWeight(self.ptr()) }
    }
    fn getWeightString(&self) -> ~str {
        unsafe { String::from(wxFont_GetWeightString(self.ptr())).to_str() }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxFont_IsOk(self.ptr()) }
    }
    fn setDefaultEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetDefaultEncoding(self.ptr(), encoding) }
    }
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetEncoding(self.ptr(), encoding) }
    }
    fn setFaceName(&self, faceName: &str) {
        let faceName = strToString(faceName);
        unsafe { wxFont_SetFaceName(self.ptr(), faceName.ptr()) }
    }
    fn setFamily(&self, family: c_int) {
        unsafe { wxFont_SetFamily(self.ptr(), family) }
    }
    fn setPointSize(&self, pointSize: c_int) {
        unsafe { wxFont_SetPointSize(self.ptr(), pointSize) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxFont_SetStyle(self.ptr(), style) }
    }
    fn setUnderlined(&self, underlined: c_int) {
        unsafe { wxFont_SetUnderlined(self.ptr(), underlined) }
    }
    fn setWeight(&self, weight: c_int) {
        unsafe { wxFont_SetWeight(self.ptr(), weight) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxFont_IsStatic(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFontData](http://docs.wxwidgets.org/3.0/classwx_font_data.html) class.
pub struct FontData { ptr: *mut c_void }
impl FontDataMethods for FontData {}
impl ObjectMethods for FontData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontData {
    pub fn from(ptr: *mut c_void) -> FontData { FontData { ptr: ptr } }
    pub fn null() -> FontData { FontData::from(0 as *mut c_void) }
    
    pub fn new() -> FontData {
        unsafe { FontData::from(wxFontData_Create()) }
    }
}

/// Methods of the wxWidgets' [wxFontData](http://docs.wxwidgets.org/3.0/classwx_font_data.html) class.
pub trait FontDataMethods : ObjectMethods {
    fn enableEffects(&self, flag: c_int) {
        unsafe { wxFontData_EnableEffects(self.ptr(), flag) }
    }
    fn getAllowSymbols(&self) -> c_int {
        unsafe { wxFontData_GetAllowSymbols(self.ptr()) }
    }
    fn getChosenFont<T: FontMethods>(&self, ref_: &T) {
        unsafe { wxFontData_GetChosenFont(self.ptr(), ref_.ptr()) }
    }
    fn getColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxFontData_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getEnableEffects(&self) -> c_int {
        unsafe { wxFontData_GetEnableEffects(self.ptr()) }
    }
    fn getEncoding(&self) -> c_int {
        unsafe { wxFontData_GetEncoding(self.ptr()) }
    }
    fn getInitialFont<T: FontMethods>(&self, ref_: &T) {
        unsafe { wxFontData_GetInitialFont(self.ptr(), ref_.ptr()) }
    }
    fn getShowHelp(&self) -> c_int {
        unsafe { wxFontData_GetShowHelp(self.ptr()) }
    }
    fn setAllowSymbols(&self, flag: c_int) {
        unsafe { wxFontData_SetAllowSymbols(self.ptr(), flag) }
    }
    fn setChosenFont<T: FontMethods>(&self, font: &T) {
        unsafe { wxFontData_SetChosenFont(self.ptr(), font.ptr()) }
    }
    fn setColour<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxFontData_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.ptr(), encoding) }
    }
    fn setInitialFont<T: FontMethods>(&self, font: &T) {
        unsafe { wxFontData_SetInitialFont(self.ptr(), font.ptr()) }
    }
    fn setRange(&self, minRange: c_int, maxRange: c_int) {
        unsafe { wxFontData_SetRange(self.ptr(), minRange, maxRange) }
    }
    fn setShowHelp(&self, flag: c_int) {
        unsafe { wxFontData_SetShowHelp(self.ptr(), flag) }
    }
}

/// Wraps the wxWidgets' [wxFontDialog](http://docs.wxwidgets.org/3.0/classwx_font_dialog.html) class.
pub struct FontDialog { ptr: *mut c_void }
impl FontDialogMethods for FontDialog {}
impl DialogMethods for FontDialog {}
impl TopLevelWindowMethods for FontDialog {}
impl WindowMethods for FontDialog {}
impl EvtHandlerMethods for FontDialog {}
impl ObjectMethods for FontDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontDialog {
    pub fn from(ptr: *mut c_void) -> FontDialog { FontDialog { ptr: ptr } }
    pub fn null() -> FontDialog { FontDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: FontDataMethods>(_prt: &T, fnt: &U) -> FontDialog {
        unsafe { FontDialog::from(wxFontDialog_Create(_prt.ptr(), fnt.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxFontDialog](http://docs.wxwidgets.org/3.0/classwx_font_dialog.html) class.
pub trait FontDialogMethods : DialogMethods {
    fn getFontData<T: FontDataMethods>(&self, _ref: &T) {
        unsafe { wxFontDialog_GetFontData(self.ptr(), _ref.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFontEnumerator](http://docs.wxwidgets.org/3.0/classwx_font_enumerator.html) class.
pub struct FontEnumerator { ptr: *mut c_void }
impl FontEnumeratorMethods for FontEnumerator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontEnumerator {
    pub fn from(ptr: *mut c_void) -> FontEnumerator { FontEnumerator { ptr: ptr } }
    pub fn null() -> FontEnumerator { FontEnumerator::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> FontEnumerator {
        unsafe { FontEnumerator::from(wxFontEnumerator_Create(_obj, _fnc)) }
    }
}

/// Methods of the wxWidgets' [wxFontEnumerator](http://docs.wxwidgets.org/3.0/classwx_font_enumerator.html) class.
pub trait FontEnumeratorMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.ptr()) }
    }
    fn enumerateEncodings(&self, facename: &str) -> c_int {
        let facename = strToString(facename);
        unsafe { wxFontEnumerator_EnumerateEncodings(self.ptr(), facename.ptr()) }
    }
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> c_int {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.ptr(), encoding, fixedWidthOnly) }
    }
}

/// Wraps the wxWidgets' [wxFontList](http://docs.wxwidgets.org/3.0/classwx_font_list.html) class.
pub struct FontList { ptr: *mut c_void }
impl FontListMethods for FontList {}
impl ListMethods for FontList {}
impl ObjectMethods for FontList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontList {
    pub fn from(ptr: *mut c_void) -> FontList { FontList { ptr: ptr } }
    pub fn null() -> FontList { FontList::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFontList](http://docs.wxwidgets.org/3.0/classwx_font_list.html) class.
pub trait FontListMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxFontMapper](http://docs.wxwidgets.org/3.0/classwx_font_mapper.html) class.
pub struct FontMapper { ptr: *mut c_void }
impl FontMapperMethods for FontMapper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontMapper {
    pub fn from(ptr: *mut c_void) -> FontMapper { FontMapper { ptr: ptr } }
    pub fn null() -> FontMapper { FontMapper::from(0 as *mut c_void) }
    
    pub fn new() -> FontMapper {
        unsafe { FontMapper::from(wxFontMapper_Create()) }
    }
}

/// Methods of the wxWidgets' [wxFontMapper](http://docs.wxwidgets.org/3.0/classwx_font_mapper.html) class.
pub trait FontMapperMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *mut c_void, _buf: &str) -> c_int {
        let _buf = strToString(_buf);
        unsafe { wxFontMapper_GetAltForEncoding(self.ptr(), encoding, alt_encoding, _buf.ptr()) }
    }
    fn isEncodingAvailable(&self, encoding: c_int, _buf: &str) -> c_int {
        let _buf = strToString(_buf);
        unsafe { wxFontMapper_IsEncodingAvailable(self.ptr(), encoding, _buf.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxFrame](http://docs.wxwidgets.org/3.0/classwx_frame.html) class.
pub struct Frame { ptr: *mut c_void }
impl FrameMethods for Frame {}
impl TopLevelWindowMethods for Frame {}
impl WindowMethods for Frame {}
impl EvtHandlerMethods for Frame {}
impl ObjectMethods for Frame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Frame {
    pub fn from(ptr: *mut c_void) -> Frame { Frame { ptr: ptr } }
    pub fn null() -> Frame { Frame::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Frame {
        let _txt = strToString(_txt);
        unsafe { Frame::from(wxFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxFrame](http://docs.wxwidgets.org/3.0/classwx_frame.html) class.
pub trait FrameMethods : TopLevelWindowMethods {
    fn newStatusBar(&self, number: c_int, style: c_int) -> StatusBar {
        unsafe { StatusBar::from(wxFrame_CreateStatusBar(self.ptr(), number, style)) }
    }
    fn newToolBar(&self, style: c_long) -> ToolBar {
        unsafe { ToolBar::from(wxFrame_CreateToolBar(self.ptr(), style)) }
    }
    fn getClientAreaOrigin_left(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self.ptr()) }
    }
    fn getClientAreaOrigin_top(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self.ptr()) }
    }
    fn getMenuBar(&self) -> MenuBar {
        unsafe { MenuBar::from(wxFrame_GetMenuBar(self.ptr())) }
    }
    fn getStatusBar(&self) -> StatusBar {
        unsafe { StatusBar::from(wxFrame_GetStatusBar(self.ptr())) }
    }
    fn getToolBar(&self) -> ToolBar {
        unsafe { ToolBar::from(wxFrame_GetToolBar(self.ptr())) }
    }
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.ptr()) }
    }
    fn setMenuBar<T: MenuBarMethods>(&self, menubar: &T) {
        unsafe { wxFrame_SetMenuBar(self.ptr(), menubar.ptr()) }
    }
    fn setStatusBar<T: StatusBarMethods>(&self, statBar: &T) {
        unsafe { wxFrame_SetStatusBar(self.ptr(), statBar.ptr()) }
    }
    fn setStatusText(&self, _txt: &str, _number: c_int) {
        let _txt = strToString(_txt);
        unsafe { wxFrame_SetStatusText(self.ptr(), _txt.ptr(), _number) }
    }
    fn setStatusWidths(&self, _n: c_int, _widths_field: *mut c_void) {
        unsafe { wxFrame_SetStatusWidths(self.ptr(), _n, _widths_field) }
    }
    fn setToolBar<T: ToolBarMethods>(&self, _toolbar: &T) {
        unsafe { wxFrame_SetToolBar(self.ptr(), _toolbar.ptr()) }
    }
    fn setShape<T: RegionMethods>(&self, region: &T) -> c_int {
        unsafe { wxFrame_SetShape(self.ptr(), region.ptr()) }
    }
    fn showFullScreen(&self, show: c_int, style: c_int) -> c_int {
        unsafe { wxFrame_ShowFullScreen(self.ptr(), show, style) }
    }
    fn isFullScreen(&self) -> c_int {
        unsafe { wxFrame_IsFullScreen(self.ptr()) }
    }
    fn centre(&self, orientation: c_int) {
        unsafe { wxFrame_Centre(self.ptr(), orientation) }
    }
}

/// Wraps the wxWidgets' [wxGDIObject](http://docs.wxwidgets.org/3.0/classwx_gdio_bject.html) class.
pub struct GDIObject { ptr: *mut c_void }
impl GDIObjectMethods for GDIObject {}
impl ObjectMethods for GDIObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GDIObject {
    pub fn from(ptr: *mut c_void) -> GDIObject { GDIObject { ptr: ptr } }
    pub fn null() -> GDIObject { GDIObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGDIObject](http://docs.wxwidgets.org/3.0/classwx_gdio_bject.html) class.
pub trait GDIObjectMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxGauge](http://docs.wxwidgets.org/3.0/classwx_gauge.html) class.
pub struct Gauge { ptr: *mut c_void }
impl GaugeMethods for Gauge {}
impl ControlMethods for Gauge {}
impl WindowMethods for Gauge {}
impl EvtHandlerMethods for Gauge {}
impl ObjectMethods for Gauge { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Gauge {
    pub fn from(ptr: *mut c_void) -> Gauge { Gauge { ptr: ptr } }
    pub fn null() -> Gauge { Gauge::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Gauge {
        unsafe { Gauge::from(wxGauge_Create(_prt.ptr(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxGauge](http://docs.wxwidgets.org/3.0/classwx_gauge.html) class.
pub trait GaugeMethods : ControlMethods {
    fn getBezelFace(&self) -> c_int {
        unsafe { wxGauge_GetBezelFace(self.ptr()) }
    }
    fn getRange(&self) -> c_int {
        unsafe { wxGauge_GetRange(self.ptr()) }
    }
    fn getShadowWidth(&self) -> c_int {
        unsafe { wxGauge_GetShadowWidth(self.ptr()) }
    }
    fn getValue(&self) -> c_int {
        unsafe { wxGauge_GetValue(self.ptr()) }
    }
    fn setBezelFace(&self, w: c_int) {
        unsafe { wxGauge_SetBezelFace(self.ptr(), w) }
    }
    fn setRange(&self, r: c_int) {
        unsafe { wxGauge_SetRange(self.ptr(), r) }
    }
    fn setShadowWidth(&self, w: c_int) {
        unsafe { wxGauge_SetShadowWidth(self.ptr(), w) }
    }
    fn setValue(&self, pos: c_int) {
        unsafe { wxGauge_SetValue(self.ptr(), pos) }
    }
}

/// Wraps the wxWidgets' [wxGenericDirCtrl](http://docs.wxwidgets.org/3.0/classwx_generic_dir_ctrl.html) class.
pub struct GenericDirCtrl { ptr: *mut c_void }
impl GenericDirCtrlMethods for GenericDirCtrl {}
impl ControlMethods for GenericDirCtrl {}
impl WindowMethods for GenericDirCtrl {}
impl EvtHandlerMethods for GenericDirCtrl {}
impl ObjectMethods for GenericDirCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GenericDirCtrl {
    pub fn from(ptr: *mut c_void) -> GenericDirCtrl { GenericDirCtrl { ptr: ptr } }
    pub fn null() -> GenericDirCtrl { GenericDirCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGenericDirCtrl](http://docs.wxwidgets.org/3.0/classwx_generic_dir_ctrl.html) class.
pub trait GenericDirCtrlMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxGenericValidator](http://docs.wxwidgets.org/3.0/classwx_generic_validator.html) class.
pub struct GenericValidator { ptr: *mut c_void }
impl GenericValidatorMethods for GenericValidator {}
impl ValidatorMethods for GenericValidator {}
impl EvtHandlerMethods for GenericValidator {}
impl ObjectMethods for GenericValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GenericValidator {
    pub fn from(ptr: *mut c_void) -> GenericValidator { GenericValidator { ptr: ptr } }
    pub fn null() -> GenericValidator { GenericValidator::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGenericValidator](http://docs.wxwidgets.org/3.0/classwx_generic_validator.html) class.
pub trait GenericValidatorMethods : ValidatorMethods {
}

/// Wraps the wxWidgets' [wxGridSizer](http://docs.wxwidgets.org/3.0/classwx_grid_sizer.html) class.
pub struct GridSizer { ptr: *mut c_void }
impl GridSizerMethods for GridSizer {}
impl SizerMethods for GridSizer {}
impl ObjectMethods for GridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridSizer {
    pub fn from(ptr: *mut c_void) -> GridSizer { GridSizer { ptr: ptr } }
    pub fn null() -> GridSizer { GridSizer::from(0 as *mut c_void) }
    
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> GridSizer {
        unsafe { GridSizer::from(wxGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

/// Methods of the wxWidgets' [wxGridSizer](http://docs.wxwidgets.org/3.0/classwx_grid_sizer.html) class.
pub trait GridSizerMethods : SizerMethods {
    fn getCols(&self) -> c_int {
        unsafe { wxGridSizer_GetCols(self.ptr()) }
    }
    fn getHGap(&self) -> c_int {
        unsafe { wxGridSizer_GetHGap(self.ptr()) }
    }
    fn getRows(&self) -> c_int {
        unsafe { wxGridSizer_GetRows(self.ptr()) }
    }
    fn getVGap(&self) -> c_int {
        unsafe { wxGridSizer_GetVGap(self.ptr()) }
    }
    fn setCols(&self, cols: c_int) {
        unsafe { wxGridSizer_SetCols(self.ptr(), cols) }
    }
    fn setHGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetHGap(self.ptr(), gap) }
    }
    fn setRows(&self, rows: c_int) {
        unsafe { wxGridSizer_SetRows(self.ptr(), rows) }
    }
    fn setVGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetVGap(self.ptr(), gap) }
    }
}

/// Wraps the wxWidgets' [wxHelpController](http://docs.wxwidgets.org/3.0/classwx_help_controller.html) class.
pub struct HelpController { ptr: *mut c_void }
impl HelpControllerMethods for HelpController {}
impl HelpControllerBaseMethods for HelpController {}
impl ObjectMethods for HelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpController {
    pub fn from(ptr: *mut c_void) -> HelpController { HelpController { ptr: ptr } }
    pub fn null() -> HelpController { HelpController::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHelpController](http://docs.wxwidgets.org/3.0/classwx_help_controller.html) class.
pub trait HelpControllerMethods : HelpControllerBaseMethods {
}

/// Wraps the wxWidgets' [wxHelpControllerBase](http://docs.wxwidgets.org/3.0/classwx_help_controller_base.html) class.
pub struct HelpControllerBase { ptr: *mut c_void }
impl HelpControllerBaseMethods for HelpControllerBase {}
impl ObjectMethods for HelpControllerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpControllerBase {
    pub fn from(ptr: *mut c_void) -> HelpControllerBase { HelpControllerBase { ptr: ptr } }
    pub fn null() -> HelpControllerBase { HelpControllerBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHelpControllerBase](http://docs.wxwidgets.org/3.0/classwx_help_controller_base.html) class.
pub trait HelpControllerBaseMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHelpControllerHelpProvider](http://docs.wxwidgets.org/3.0/classwx_help_controller_help_provider.html) class.
pub struct HelpControllerHelpProvider { ptr: *mut c_void }
impl HelpControllerHelpProviderMethods for HelpControllerHelpProvider {}
impl SimpleHelpProviderMethods for HelpControllerHelpProvider {}
impl HelpProviderMethods for HelpControllerHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpControllerHelpProvider {
    pub fn from(ptr: *mut c_void) -> HelpControllerHelpProvider { HelpControllerHelpProvider { ptr: ptr } }
    pub fn null() -> HelpControllerHelpProvider { HelpControllerHelpProvider::from(0 as *mut c_void) }
    
    pub fn new<T: HelpControllerBaseMethods>(ctr: &T) -> HelpControllerHelpProvider {
        unsafe { HelpControllerHelpProvider::from(wxHelpControllerHelpProvider_Create(ctr.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxHelpControllerHelpProvider](http://docs.wxwidgets.org/3.0/classwx_help_controller_help_provider.html) class.
pub trait HelpControllerHelpProviderMethods : SimpleHelpProviderMethods {
    fn getHelpController(&self) -> HelpControllerBase {
        unsafe { HelpControllerBase::from(wxHelpControllerHelpProvider_GetHelpController(self.ptr())) }
    }
    fn setHelpController<T: HelpControllerMethods>(&self, hc: &T) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.ptr(), hc.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxHelpEvent](http://docs.wxwidgets.org/3.0/classwx_help_event.html) class.
pub struct HelpEvent { ptr: *mut c_void }
impl HelpEventMethods for HelpEvent {}
impl CommandEventMethods for HelpEvent {}
impl EventMethods for HelpEvent {}
impl ObjectMethods for HelpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpEvent {
    pub fn from(ptr: *mut c_void) -> HelpEvent { HelpEvent { ptr: ptr } }
    pub fn null() -> HelpEvent { HelpEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHelpEvent](http://docs.wxwidgets.org/3.0/classwx_help_event.html) class.
pub trait HelpEventMethods : CommandEventMethods {
    fn getLink(&self) -> ~str {
        unsafe { String::from(wxHelpEvent_GetLink(self.ptr())).to_str() }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxHelpEvent_GetPosition(self.ptr())) }
    }
    fn getTarget(&self) -> ~str {
        unsafe { String::from(wxHelpEvent_GetTarget(self.ptr())).to_str() }
    }
    fn setLink(&self, link: &str) {
        let link = strToString(link);
        unsafe { wxHelpEvent_SetLink(self.ptr(), link.ptr()) }
    }
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.ptr(), x, y) }
    }
    fn setTarget(&self, target: &str) {
        let target = strToString(target);
        unsafe { wxHelpEvent_SetTarget(self.ptr(), target.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxHelpProvider](http://docs.wxwidgets.org/3.0/classwx_help_provider.html) class.
pub struct HelpProvider { ptr: *mut c_void }
impl HelpProviderMethods for HelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpProvider {
    pub fn from(ptr: *mut c_void) -> HelpProvider { HelpProvider { ptr: ptr } }
    pub fn null() -> HelpProvider { HelpProvider::from(0 as *mut c_void) }
    
    pub fn get() -> HelpProvider {
        unsafe { HelpProvider::from(wxHelpProvider_Get()) }
    }
}

/// Methods of the wxWidgets' [wxHelpProvider](http://docs.wxwidgets.org/3.0/classwx_help_provider.html) class.
pub trait HelpProviderMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn addHelp<T: WindowMethods>(&self, window: &T, text: &str) {
        let text = strToString(text);
        unsafe { wxHelpProvider_AddHelp(self.ptr(), window.ptr(), text.ptr()) }
    }
    fn addHelpById(&self, id: c_int, text: &str) {
        let text = strToString(text);
        unsafe { wxHelpProvider_AddHelpById(self.ptr(), id, text.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.ptr()) }
    }
    fn getHelp<T: WindowMethods>(&self, window: &T) -> ~str {
        unsafe { String::from(wxHelpProvider_GetHelp(self.ptr(), window.ptr())).to_str() }
    }
    fn removeHelp<T: WindowMethods>(&self, window: &T) {
        unsafe { wxHelpProvider_RemoveHelp(self.ptr(), window.ptr()) }
    }
    fn set(&self) -> HelpProvider {
        unsafe { HelpProvider::from(wxHelpProvider_Set(self.ptr())) }
    }
    fn showHelp<T: WindowMethods>(&self, window: &T) -> c_int {
        unsafe { wxHelpProvider_ShowHelp(self.ptr(), window.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxIcon](http://docs.wxwidgets.org/3.0/classwx_icon.html) class.
pub struct Icon { ptr: *mut c_void }
impl IconMethods for Icon {}
impl BitmapMethods for Icon {}
impl GDIObjectMethods for Icon {}
impl ObjectMethods for Icon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Icon {
    pub fn from(ptr: *mut c_void) -> Icon { Icon { ptr: ptr } }
    pub fn null() -> Icon { Icon::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Icon {
        unsafe { Icon::from(wxIcon_CreateDefault()) }
    }
    pub fn newLoad(name: &str, type_: c_long, width: c_int, height: c_int) -> Icon {
        let name = strToString(name);
        unsafe { Icon::from(wxIcon_CreateLoad(name.ptr(), type_, width, height)) }
    }
}

/// Methods of the wxWidgets' [wxIcon](http://docs.wxwidgets.org/3.0/classwx_icon.html) class.
pub trait IconMethods : BitmapMethods {
    fn assign(&self, other: *mut c_void) {
        unsafe { wxIcon_Assign(self.ptr(), other) }
    }
    fn copyFromBitmap<T: BitmapMethods>(&self, bmp: &T) {
        unsafe { wxIcon_CopyFromBitmap(self.ptr(), bmp.ptr()) }
    }
    fn fromRaw(&self, width: c_int, height: c_int) -> Icon {
        unsafe { Icon::from(wxIcon_FromRaw(self.ptr(), width, height)) }
    }
    fn fromXPM(&self) -> Icon {
        unsafe { Icon::from(wxIcon_FromXPM(self.ptr())) }
    }
    fn isEqual(&self, other: &IconMethods) -> c_int {
        unsafe { wxIcon_IsEqual(self.ptr(), other.ptr()) }
    }
    fn load(&self, name: &str, type_: c_long, width: c_int, height: c_int) -> c_int {
        let name = strToString(name);
        unsafe { wxIcon_Load(self.ptr(), name.ptr(), type_, width, height) }
    }
}

/// Wraps the wxWidgets' [wxIconBundle](http://docs.wxwidgets.org/3.0/classwx_icon_bundle.html) class.
pub struct IconBundle { ptr: *mut c_void }
impl IconBundleMethods for IconBundle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IconBundle {
    pub fn from(ptr: *mut c_void) -> IconBundle { IconBundle { ptr: ptr } }
    pub fn null() -> IconBundle { IconBundle::from(0 as *mut c_void) }
    
    pub fn newDefault() -> IconBundle {
        unsafe { IconBundle::from(wxIconBundle_CreateDefault()) }
    }
    pub fn newFromFile(file: &str, type_: c_int) -> IconBundle {
        let file = strToString(file);
        unsafe { IconBundle::from(wxIconBundle_CreateFromFile(file.ptr(), type_)) }
    }
    pub fn newFromIcon<T: IconMethods>(icon: &T) -> IconBundle {
        unsafe { IconBundle::from(wxIconBundle_CreateFromIcon(icon.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxIconBundle](http://docs.wxwidgets.org/3.0/classwx_icon_bundle.html) class.
pub trait IconBundleMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn addIcon<T: IconMethods>(&self, icon: &T) {
        unsafe { wxIconBundle_AddIcon(self.ptr(), icon.ptr()) }
    }
    fn addIconFromFile(&self, file: &str, type_: c_int) {
        let file = strToString(file);
        unsafe { wxIconBundle_AddIconFromFile(self.ptr(), file.ptr(), type_) }
    }
    fn assign<T: IconBundleMethods>(&self, _ref: &T) {
        unsafe { wxIconBundle_Assign(self.ptr(), _ref.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.ptr()) }
    }
    fn getIcon<T: IconMethods>(&self, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxIconBundle_GetIcon(self.ptr(), w, h, _ref.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxIconizeEvent](http://docs.wxwidgets.org/3.0/classwx_iconize_event.html) class.
pub struct IconizeEvent { ptr: *mut c_void }
impl IconizeEventMethods for IconizeEvent {}
impl EventMethods for IconizeEvent {}
impl ObjectMethods for IconizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IconizeEvent {
    pub fn from(ptr: *mut c_void) -> IconizeEvent { IconizeEvent { ptr: ptr } }
    pub fn null() -> IconizeEvent { IconizeEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxIconizeEvent](http://docs.wxwidgets.org/3.0/classwx_iconize_event.html) class.
pub trait IconizeEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxIdleEvent](http://docs.wxwidgets.org/3.0/classwx_idle_event.html) class.
pub struct IdleEvent { ptr: *mut c_void }
impl IdleEventMethods for IdleEvent {}
impl EventMethods for IdleEvent {}
impl ObjectMethods for IdleEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IdleEvent {
    pub fn from(ptr: *mut c_void) -> IdleEvent { IdleEvent { ptr: ptr } }
    pub fn null() -> IdleEvent { IdleEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxIdleEvent](http://docs.wxwidgets.org/3.0/classwx_idle_event.html) class.
pub trait IdleEventMethods : EventMethods {
    fn moreRequested(&self) -> c_int {
        unsafe { wxIdleEvent_MoreRequested(self.ptr()) }
    }
    fn requestMore(&self, needMore: c_int) {
        unsafe { wxIdleEvent_RequestMore(self.ptr(), needMore) }
    }
}

/// Wraps the wxWidgets' [wxImage](http://docs.wxwidgets.org/3.0/classwx_image.html) class.
pub struct Image { ptr: *mut c_void }
impl ImageMethods for Image {}
impl ObjectMethods for Image { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Image {
    pub fn from(ptr: *mut c_void) -> Image { Image { ptr: ptr } }
    pub fn null() -> Image { Image::from(0 as *mut c_void) }
    
    pub fn canRead(name: &str) -> c_int {
        let name = strToString(name);
        unsafe { wxImage_CanRead(name.ptr()) }
    }
    pub fn newDefault() -> Image {
        unsafe { Image::from(wxImage_CreateDefault()) }
    }
    pub fn newFromBitmap<T: BitmapMethods>(bitmap: &T) -> Image {
        unsafe { Image::from(wxImage_CreateFromBitmap(bitmap.ptr())) }
    }
    pub fn newFromByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> Image {
        unsafe { Image::from(wxImage_CreateFromByteString(data, length, type_)) }
    }
    pub fn newFromLazyByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> Image {
        unsafe { Image::from(wxImage_CreateFromLazyByteString(data, length, type_)) }
    }
    pub fn newFromData(width: c_int, height: c_int, data: *mut c_void) -> Image {
        unsafe { Image::from(wxImage_CreateFromData(width, height, data)) }
    }
    pub fn newFromFile(name: &str) -> Image {
        let name = strToString(name);
        unsafe { Image::from(wxImage_CreateFromFile(name.ptr())) }
    }
    pub fn newSized(width: c_int, height: c_int) -> Image {
        unsafe { Image::from(wxImage_CreateSized(width, height)) }
    }
    pub fn newFromDataEx(width: c_int, height: c_int, data: *mut c_void, isStaticData: c_int) -> Image {
        unsafe { Image::from(wxImage_CreateFromDataEx(width, height, data, isStaticData)) }
    }
}

/// Methods of the wxWidgets' [wxImage](http://docs.wxwidgets.org/3.0/classwx_image.html) class.
pub trait ImageMethods : ObjectMethods {
    fn convertToBitmap<T: BitmapMethods>(&self, bitmap: &T) {
        unsafe { wxImage_ConvertToBitmap(self.ptr(), bitmap.ptr()) }
    }
    fn convertToByteString(&self, type_: c_int, data: *mut c_char) -> c_int {
        unsafe { wxImage_ConvertToByteString(self.ptr(), type_, data) }
    }
    fn convertToLazyByteString(&self, type_: c_int, data: *mut c_char) -> c_int {
        unsafe { wxImage_ConvertToLazyByteString(self.ptr(), type_, data) }
    }
    fn countColours(&self, stopafter: c_int) -> c_int {
        unsafe { wxImage_CountColours(self.ptr(), stopafter) }
    }
    fn destroy(&self) {
        unsafe { wxImage_Destroy(self.ptr()) }
    }
    fn getBlue(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetBlue(self.ptr(), x, y) }
    }
    fn getData(&self) -> *mut c_void {
        unsafe { wxImage_GetData(self.ptr()) }
    }
    fn getGreen(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetGreen(self.ptr(), x, y) }
    }
    fn getHeight(&self) -> c_int {
        unsafe { wxImage_GetHeight(self.ptr()) }
    }
    fn getMaskBlue(&self) -> int8_t {
        unsafe { wxImage_GetMaskBlue(self.ptr()) }
    }
    fn getMaskGreen(&self) -> int8_t {
        unsafe { wxImage_GetMaskGreen(self.ptr()) }
    }
    fn getMaskRed(&self) -> int8_t {
        unsafe { wxImage_GetMaskRed(self.ptr()) }
    }
    fn getRed(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetRed(self.ptr(), x, y) }
    }
    fn getSubImage<T: ImageMethods>(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: &T) {
        unsafe { wxImage_GetSubImage(self.ptr(), x, y, w, h, image.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxImage_GetWidth(self.ptr()) }
    }
    fn hasMask(&self) -> c_int {
        unsafe { wxImage_HasMask(self.ptr()) }
    }
    fn getOption(&self, name: &str) -> ~str {
        let name = strToString(name);
        unsafe { String::from(wxImage_GetOption(self.ptr(), name.ptr())).to_str() }
    }
    fn getOptionInt(&self, name: &str) -> c_int {
        let name = strToString(name);
        unsafe { wxImage_GetOptionInt(self.ptr(), name.ptr()) }
    }
    fn hasOption(&self, name: &str) -> c_int {
        let name = strToString(name);
        unsafe { wxImage_HasOption(self.ptr(), name.ptr()) }
    }
    fn initialize(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Initialize(self.ptr(), width, height) }
    }
    fn initializeFromData(&self, width: c_int, height: c_int, data: *mut c_void) {
        unsafe { wxImage_InitializeFromData(self.ptr(), width, height, data) }
    }
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = strToString(name);
        unsafe { wxImage_LoadFile(self.ptr(), name.ptr(), type_) }
    }
    fn mirror<T: ImageMethods>(&self, horizontally: c_int, image: &T) {
        unsafe { wxImage_Mirror(self.ptr(), horizontally, image.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxImage_IsOk(self.ptr()) }
    }
    fn paste<T: ImageMethods>(&self, image: &T, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self.ptr(), image.ptr(), x, y) }
    }
    fn replace(&self, r1: uint8_t, g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t, b2: uint8_t) {
        unsafe { wxImage_Replace(self.ptr(), r1, g1, b1, r2, g2, b2) }
    }
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self.ptr(), width, height) }
    }
    fn rotate<T: ImageMethods>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *mut c_void, image: &T) {
        unsafe { wxImage_Rotate(self.ptr(), angle, c_x, c_y, interpolating, offset_after_rotation, image.ptr()) }
    }
    fn rotate90<T: ImageMethods>(&self, clockwise: c_int, image: &T) {
        unsafe { wxImage_Rotate90(self.ptr(), clockwise, image.ptr()) }
    }
    fn saveFile(&self, name: &str, type_: c_int) -> c_int {
        let name = strToString(name);
        unsafe { wxImage_SaveFile(self.ptr(), name.ptr(), type_) }
    }
    fn scale<T: ImageMethods>(&self, width: c_int, height: c_int, image: &T) {
        unsafe { wxImage_Scale(self.ptr(), width, height, image.ptr()) }
    }
    fn setData(&self, data: *mut c_void) {
        unsafe { wxImage_SetData(self.ptr(), data) }
    }
    fn setDataAndSize(&self, data: *mut c_void, new_width: c_int, new_height: c_int) {
        unsafe { wxImage_SetDataAndSize(self.ptr(), data, new_width, new_height) }
    }
    fn setMask(&self, mask: c_int) {
        unsafe { wxImage_SetMask(self.ptr(), mask) }
    }
    fn setMaskColour(&self, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetMaskColour(self.ptr(), r, g, b) }
    }
    fn setOption(&self, name: &str, value: &str) {
        let name = strToString(name);
        let value = strToString(value);
        unsafe { wxImage_SetOption(self.ptr(), name.ptr(), value.ptr()) }
    }
    fn setOptionInt(&self, name: &str, value: c_int) {
        let name = strToString(name);
        unsafe { wxImage_SetOptionInt(self.ptr(), name.ptr(), value) }
    }
    fn setRGB(&self, x: c_int, y: c_int, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetRGB(self.ptr(), x, y, r, g, b) }
    }
}

/// Wraps the wxWidgets' [wxImageHandler](http://docs.wxwidgets.org/3.0/classwx_image_handler.html) class.
pub struct ImageHandler { ptr: *mut c_void }
impl ImageHandlerMethods for ImageHandler {}
impl ObjectMethods for ImageHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ImageHandler {
    pub fn from(ptr: *mut c_void) -> ImageHandler { ImageHandler { ptr: ptr } }
    pub fn null() -> ImageHandler { ImageHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxImageHandler](http://docs.wxwidgets.org/3.0/classwx_image_handler.html) class.
pub trait ImageHandlerMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxImageList](http://docs.wxwidgets.org/3.0/classwx_image_list.html) class.
pub struct ImageList { ptr: *mut c_void }
impl ImageListMethods for ImageList {}
impl ObjectMethods for ImageList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ImageList {
    pub fn from(ptr: *mut c_void) -> ImageList { ImageList { ptr: ptr } }
    pub fn null() -> ImageList { ImageList::from(0 as *mut c_void) }
    
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> ImageList {
        unsafe { ImageList::from(wxImageList_Create(width, height, mask, initialCount)) }
    }
}

/// Methods of the wxWidgets' [wxImageList](http://docs.wxwidgets.org/3.0/classwx_image_list.html) class.
pub trait ImageListMethods : ObjectMethods {
    fn addBitmap<T: BitmapMethods, U: BitmapMethods>(&self, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_AddBitmap(self.ptr(), bitmap.ptr(), mask.ptr()) }
    }
    fn addIcon<T: IconMethods>(&self, icon: &T) -> c_int {
        unsafe { wxImageList_AddIcon(self.ptr(), icon.ptr()) }
    }
    fn addMasked<T: BitmapMethods, U: ColourMethods>(&self, bitmap: &T, maskColour: &U) -> c_int {
        unsafe { wxImageList_AddMasked(self.ptr(), bitmap.ptr(), maskColour.ptr()) }
    }
    fn draw<T: DCMethods>(&self, index: c_int, dc: &T, x: c_int, y: c_int, flags: c_int, solidBackground: c_int) -> c_int {
        unsafe { wxImageList_Draw(self.ptr(), index, dc.ptr(), x, y, flags, solidBackground) }
    }
    fn getImageCount(&self) -> c_int {
        unsafe { wxImageList_GetImageCount(self.ptr()) }
    }
    fn getSize(&self, index: c_int, width: *mut c_int, height: *mut c_int) {
        unsafe { wxImageList_GetSize(self.ptr(), index, width, height) }
    }
    fn remove(&self, index: c_int) -> c_int {
        unsafe { wxImageList_Remove(self.ptr(), index) }
    }
    fn removeAll(&self) -> c_int {
        unsafe { wxImageList_RemoveAll(self.ptr()) }
    }
    fn replace<T: BitmapMethods, U: BitmapMethods>(&self, index: c_int, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_Replace(self.ptr(), index, bitmap.ptr(), mask.ptr()) }
    }
    fn replaceIcon<T: IconMethods>(&self, index: c_int, icon: &T) -> c_int {
        unsafe { wxImageList_ReplaceIcon(self.ptr(), index, icon.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxIndividualLayoutConstraint](http://docs.wxwidgets.org/3.0/classwx_individual_layout_constraint.html) class.
pub struct IndividualLayoutConstraint { ptr: *mut c_void }
impl IndividualLayoutConstraintMethods for IndividualLayoutConstraint {}
impl ObjectMethods for IndividualLayoutConstraint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IndividualLayoutConstraint {
    pub fn from(ptr: *mut c_void) -> IndividualLayoutConstraint { IndividualLayoutConstraint { ptr: ptr } }
    pub fn null() -> IndividualLayoutConstraint { IndividualLayoutConstraint::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxIndividualLayoutConstraint](http://docs.wxwidgets.org/3.0/classwx_individual_layout_constraint.html) class.
pub trait IndividualLayoutConstraintMethods : ObjectMethods {
    fn above<T: WindowMethods>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Above(self.ptr(), sibling.ptr(), marg) }
    }
    fn absolute(&self, val: c_int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self.ptr(), val) }
    }
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self.ptr()) }
    }
    fn below<T: WindowMethods>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Below(self.ptr(), sibling.ptr(), marg) }
    }
    fn getDone(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetDone(self.ptr()) }
    }
    fn getEdge(&self, which: c_int, thisWin: *mut c_void, other: *mut c_void) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetEdge(self.ptr(), which, thisWin, other) }
    }
    fn getMargin(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMargin(self.ptr()) }
    }
    fn getMyEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMyEdge(self.ptr()) }
    }
    fn getOtherEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetOtherEdge(self.ptr()) }
    }
    fn getOtherWindow(&self) -> *mut c_void {
        unsafe { wxIndividualLayoutConstraint_GetOtherWindow(self.ptr()) }
    }
    fn getPercent(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetPercent(self.ptr()) }
    }
    fn getRelationship(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetRelationship(self.ptr()) }
    }
    fn getValue(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetValue(self.ptr()) }
    }
    fn leftOf<T: WindowMethods>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.ptr(), sibling.ptr(), marg) }
    }
    fn percentOf<T: WindowMethods>(&self, otherW: &T, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.ptr(), otherW.ptr(), wh, per) }
    }
    fn resetIfWin<T: WindowMethods>(&self, otherW: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.ptr(), otherW.ptr()) }
    }
    fn rightOf<T: WindowMethods>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.ptr(), sibling.ptr(), marg) }
    }
    fn sameAs<T: WindowMethods>(&self, otherW: &T, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.ptr(), otherW.ptr(), edge, marg) }
    }
    fn satisfyConstraint<T: WindowMethods>(&self, constraints: *mut c_void, win: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.ptr(), constraints, win.ptr()) }
    }
    fn set<T: WindowMethods>(&self, rel: c_int, otherW: &T, otherE: c_int, val: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Set(self.ptr(), rel, otherW.ptr(), otherE, val, marg) }
    }
    fn setDone(&self, d: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetDone(self.ptr(), d) }
    }
    fn setEdge(&self, which: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetEdge(self.ptr(), which) }
    }
    fn setMargin(&self, m: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetMargin(self.ptr(), m) }
    }
    fn setRelationship(&self, r: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetRelationship(self.ptr(), r) }
    }
    fn setValue(&self, v: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetValue(self.ptr(), v) }
    }
    fn unconstrained(&self) {
        unsafe { wxIndividualLayoutConstraint_Unconstrained(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxInitDialogEvent](http://docs.wxwidgets.org/3.0/classwx_init_dialog_event.html) class.
pub struct InitDialogEvent { ptr: *mut c_void }
impl InitDialogEventMethods for InitDialogEvent {}
impl EventMethods for InitDialogEvent {}
impl ObjectMethods for InitDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InitDialogEvent {
    pub fn from(ptr: *mut c_void) -> InitDialogEvent { InitDialogEvent { ptr: ptr } }
    pub fn null() -> InitDialogEvent { InitDialogEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxInitDialogEvent](http://docs.wxwidgets.org/3.0/classwx_init_dialog_event.html) class.
pub trait InitDialogEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxJoystickEvent](http://docs.wxwidgets.org/3.0/classwx_joystick_event.html) class.
pub struct JoystickEvent { ptr: *mut c_void }
impl JoystickEventMethods for JoystickEvent {}
impl EventMethods for JoystickEvent {}
impl ObjectMethods for JoystickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl JoystickEvent {
    pub fn from(ptr: *mut c_void) -> JoystickEvent { JoystickEvent { ptr: ptr } }
    pub fn null() -> JoystickEvent { JoystickEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxJoystickEvent](http://docs.wxwidgets.org/3.0/classwx_joystick_event.html) class.
pub trait JoystickEventMethods : EventMethods {
    fn buttonDown(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonDown(self.ptr(), but) }
    }
    fn buttonIsDown(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonIsDown(self.ptr(), but) }
    }
    fn buttonUp(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonUp(self.ptr(), but) }
    }
    fn getButtonChange(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonChange(self.ptr()) }
    }
    fn getButtonState(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonState(self.ptr()) }
    }
    fn getJoystick(&self) -> c_int {
        unsafe { wxJoystickEvent_GetJoystick(self.ptr()) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxJoystickEvent_GetPosition(self.ptr())) }
    }
    fn getZPosition(&self) -> c_int {
        unsafe { wxJoystickEvent_GetZPosition(self.ptr()) }
    }
    fn isButton(&self) -> c_int {
        unsafe { wxJoystickEvent_IsButton(self.ptr()) }
    }
    fn isMove(&self) -> c_int {
        unsafe { wxJoystickEvent_IsMove(self.ptr()) }
    }
    fn isZMove(&self) -> c_int {
        unsafe { wxJoystickEvent_IsZMove(self.ptr()) }
    }
    fn setButtonChange(&self, change: c_int) {
        unsafe { wxJoystickEvent_SetButtonChange(self.ptr(), change) }
    }
    fn setButtonState(&self, state: c_int) {
        unsafe { wxJoystickEvent_SetButtonState(self.ptr(), state) }
    }
    fn setJoystick(&self, stick: c_int) {
        unsafe { wxJoystickEvent_SetJoystick(self.ptr(), stick) }
    }
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxJoystickEvent_SetPosition(self.ptr(), x, y) }
    }
    fn setZPosition(&self, zPos: c_int) {
        unsafe { wxJoystickEvent_SetZPosition(self.ptr(), zPos) }
    }
}

/// Wraps the wxWidgets' [wxKeyEvent](http://docs.wxwidgets.org/3.0/classwx_key_event.html) class.
pub struct KeyEvent { ptr: *mut c_void }
impl KeyEventMethods for KeyEvent {}
impl EventMethods for KeyEvent {}
impl ObjectMethods for KeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl KeyEvent {
    pub fn from(ptr: *mut c_void) -> KeyEvent { KeyEvent { ptr: ptr } }
    pub fn null() -> KeyEvent { KeyEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxKeyEvent](http://docs.wxwidgets.org/3.0/classwx_key_event.html) class.
pub trait KeyEventMethods : EventMethods {
    fn altDown(&self) -> c_int {
        unsafe { wxKeyEvent_AltDown(self.ptr()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxKeyEvent_ControlDown(self.ptr()) }
    }
    fn getKeyCode(&self) -> c_int {
        unsafe { wxKeyEvent_GetKeyCode(self.ptr()) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxKeyEvent_GetPosition(self.ptr())) }
    }
    fn getX(&self) -> c_int {
        unsafe { wxKeyEvent_GetX(self.ptr()) }
    }
    fn getY(&self) -> c_int {
        unsafe { wxKeyEvent_GetY(self.ptr()) }
    }
    fn getModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_GetModifiers(self.ptr()) }
    }
    fn hasModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_HasModifiers(self.ptr()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxKeyEvent_MetaDown(self.ptr()) }
    }
    fn setKeyCode(&self, code: c_int) {
        unsafe { wxKeyEvent_SetKeyCode(self.ptr(), code) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxKeyEvent_ShiftDown(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxLayoutConstraints](http://docs.wxwidgets.org/3.0/classwx_layout_constraints.html) class.
pub struct LayoutConstraints { ptr: *mut c_void }
impl LayoutConstraintsMethods for LayoutConstraints {}
impl ObjectMethods for LayoutConstraints { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LayoutConstraints {
    pub fn from(ptr: *mut c_void) -> LayoutConstraints { LayoutConstraints { ptr: ptr } }
    pub fn null() -> LayoutConstraints { LayoutConstraints::from(0 as *mut c_void) }
    
    pub fn new() -> LayoutConstraints {
        unsafe { LayoutConstraints::from(wxLayoutConstraints_Create()) }
    }
}

/// Methods of the wxWidgets' [wxLayoutConstraints](http://docs.wxwidgets.org/3.0/classwx_layout_constraints.html) class.
pub trait LayoutConstraintsMethods : ObjectMethods {
    fn bottom(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_bottom(self.ptr()) }
    }
    fn centreX(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_centreX(self.ptr()) }
    }
    fn centreY(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_centreY(self.ptr()) }
    }
    fn height(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_height(self.ptr()) }
    }
    fn left(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_left(self.ptr()) }
    }
    fn right(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_right(self.ptr()) }
    }
    fn top(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_top(self.ptr()) }
    }
    fn width(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_width(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxListBox](http://docs.wxwidgets.org/3.0/classwx_list_box.html) class.
pub struct ListBox { ptr: *mut c_void }
impl ListBoxMethods for ListBox {}
impl ControlMethods for ListBox {}
impl WindowMethods for ListBox {}
impl EvtHandlerMethods for ListBox {}
impl ObjectMethods for ListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListBox {
    pub fn from(ptr: *mut c_void) -> ListBox { ListBox { ptr: ptr } }
    pub fn null() -> ListBox { ListBox::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> ListBox {
        unsafe { ListBox::from(wxListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxListBox](http://docs.wxwidgets.org/3.0/classwx_list_box.html) class.
pub trait ListBoxMethods : ControlMethods {
    fn append(&self, item: &str) {
        let item = strToString(item);
        unsafe { wxListBox_Append(self.ptr(), item.ptr()) }
    }
    fn appendData(&self, item: &str, data: *mut c_void) {
        let item = strToString(item);
        unsafe { wxListBox_AppendData(self.ptr(), item.ptr(), data) }
    }
    fn clear(&self) {
        unsafe { wxListBox_Clear(self.ptr()) }
    }
    fn findString(&self, s: &str) -> c_int {
        let s = strToString(s);
        unsafe { wxListBox_FindString(self.ptr(), s.ptr()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxListBox_GetCount(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxListBox_GetSelection(self.ptr()) }
    }
    fn getSelections(&self, aSelections: *mut c_int, allocated: c_int) -> c_int {
        unsafe { wxListBox_GetSelections(self.ptr(), aSelections, allocated) }
    }
    fn getString(&self, n: c_int) -> ~str {
        unsafe { String::from(wxListBox_GetString(self.ptr(), n)).to_str() }
    }
    fn insertItems(&self, items: *mut c_void, pos: c_int, count: c_int) {
        unsafe { wxListBox_InsertItems(self.ptr(), items, pos, count) }
    }
    fn isSelected(&self, n: c_int) -> c_int {
        unsafe { wxListBox_IsSelected(self.ptr(), n) }
    }
    fn setFirstItem(&self, n: c_int) {
        unsafe { wxListBox_SetFirstItem(self.ptr(), n) }
    }
    fn setSelection(&self, n: c_int, select: c_int) {
        unsafe { wxListBox_SetSelection(self.ptr(), n, select) }
    }
    fn setString(&self, n: c_int, s: &str) {
        let s = strToString(s);
        unsafe { wxListBox_SetString(self.ptr(), n, s.ptr()) }
    }
    fn setStringSelection(&self, str: &str, sel: c_int) {
        let str = strToString(str);
        unsafe { wxListBox_SetStringSelection(self.ptr(), str.ptr(), sel) }
    }
}

/// Wraps the wxWidgets' [wxListCtrl](http://docs.wxwidgets.org/3.0/classwx_list_ctrl.html) class.
pub struct ListCtrl { ptr: *mut c_void }
impl ListCtrlMethods for ListCtrl {}
impl ControlMethods for ListCtrl {}
impl WindowMethods for ListCtrl {}
impl EvtHandlerMethods for ListCtrl {}
impl ObjectMethods for ListCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListCtrl {
    pub fn from(ptr: *mut c_void) -> ListCtrl { ListCtrl { ptr: ptr } }
    pub fn null() -> ListCtrl { ListCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ListCtrl {
        unsafe { ListCtrl::from(wxListCtrl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxListCtrl](http://docs.wxwidgets.org/3.0/classwx_list_ctrl.html) class.
pub trait ListCtrlMethods : ControlMethods {
    fn arrange(&self, flag: c_int) -> c_int {
        unsafe { wxListCtrl_Arrange(self.ptr(), flag) }
    }
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self.ptr()) }
    }
    fn deleteAllColumns(&self) -> c_int {
        unsafe { wxListCtrl_DeleteAllColumns(self.ptr()) }
    }
    fn deleteAllItems(&self) -> c_int {
        unsafe { wxListCtrl_DeleteAllItems(self.ptr()) }
    }
    fn deleteColumn(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_DeleteColumn(self.ptr(), col) }
    }
    fn deleteItem(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_DeleteItem(self.ptr(), item) }
    }
    fn editLabel(&self, item: c_int) {
        unsafe { wxListCtrl_EditLabel(self.ptr(), item) }
    }
    fn endEditLabel(&self, cancel: c_int) -> c_int {
        unsafe { wxListCtrl_EndEditLabel(self.ptr(), cancel) }
    }
    fn ensureVisible(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_EnsureVisible(self.ptr(), item) }
    }
    fn findItem(&self, start: c_int, str: &str, partial: c_int) -> c_int {
        let str = strToString(str);
        unsafe { wxListCtrl_FindItem(self.ptr(), start, str.ptr(), partial) }
    }
    fn findItemByData(&self, start: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByData(self.ptr(), start, data) }
    }
    fn findItemByPosition(&self, start: c_int, x: c_int, y: c_int, direction: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByPosition(self.ptr(), start, x, y, direction) }
    }
    fn getColumn<T: ListItemMethods>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_GetColumn(self.ptr(), col, item.ptr()) }
    }
    fn getColumnCount(&self) -> c_int {
        unsafe { wxListCtrl_GetColumnCount(self.ptr()) }
    }
    fn getColumnWidth(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_GetColumnWidth(self.ptr(), col) }
    }
    fn getCountPerPage(&self) -> c_int {
        unsafe { wxListCtrl_GetCountPerPage(self.ptr()) }
    }
    fn getEditControl(&self) -> TextCtrl {
        unsafe { TextCtrl::from(wxListCtrl_GetEditControl(self.ptr())) }
    }
    fn getImageList(&self, which: c_int) -> ImageList {
        unsafe { ImageList::from(wxListCtrl_GetImageList(self.ptr(), which)) }
    }
    fn getItem<T: ListItemMethods>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_GetItem(self.ptr(), info.ptr()) }
    }
    fn getItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetItemCount(self.ptr()) }
    }
    fn getItemData(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemData(self.ptr(), item) }
    }
    fn getItemFont(&self, item: c_long) -> Font {
        unsafe { Font::from(wxListCtrl_GetItemFont(self.ptr(), item)) }
    }
    fn getItemPosition(&self, item: c_int) -> Point {
        unsafe { Point::from(wxListCtrl_GetItemPosition(self.ptr(), item)) }
    }
    fn getItemRect(&self, item: c_int, code: c_int) -> Rect {
        unsafe { Rect::from(wxListCtrl_GetItemRect(self.ptr(), item, code)) }
    }
    fn getItemSpacing(&self, isSmall: c_int) -> Size {
        unsafe { Size::from(wxListCtrl_GetItemSpacing(self.ptr(), isSmall)) }
    }
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.ptr(), item, stateMask) }
    }
    fn getItemText(&self, item: c_int) -> ~str {
        unsafe { String::from(wxListCtrl_GetItemText(self.ptr(), item)).to_str() }
    }
    fn getNextItem(&self, item: c_int, geometry: c_int, state: c_int) -> c_int {
        unsafe { wxListCtrl_GetNextItem(self.ptr(), item, geometry, state) }
    }
    fn getSelectedItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetSelectedItemCount(self.ptr()) }
    }
    fn getTextColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxListCtrl_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getTopItem(&self) -> c_int {
        unsafe { wxListCtrl_GetTopItem(self.ptr()) }
    }
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_void) -> c_int {
        unsafe { wxListCtrl_HitTest(self.ptr(), x, y, flags) }
    }
    fn insertColumn(&self, col: c_int, heading: &str, format: c_int, width: c_int) -> c_int {
        let heading = strToString(heading);
        unsafe { wxListCtrl_InsertColumn(self.ptr(), col, heading.ptr(), format, width) }
    }
    fn insertColumnFromInfo<T: ListItemMethods>(&self, col: c_int, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.ptr(), col, info.ptr()) }
    }
    fn insertItem<T: ListItemMethods>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.ptr(), info.ptr()) }
    }
    fn insertItemWithData(&self, index: c_int, label: &str) -> c_int {
        let label = strToString(label);
        unsafe { wxListCtrl_InsertItemWithData(self.ptr(), index, label.ptr()) }
    }
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.ptr(), index, imageIndex) }
    }
    fn insertItemWithLabel(&self, index: c_int, label: &str, imageIndex: c_int) -> c_int {
        let label = strToString(label);
        unsafe { wxListCtrl_InsertItemWithLabel(self.ptr(), index, label.ptr(), imageIndex) }
    }
    fn isVirtual(&self) -> c_int {
        unsafe { wxListCtrl_IsVirtual(self.ptr()) }
    }
    fn refreshItem(&self, item: c_long) {
        unsafe { wxListCtrl_RefreshItem(self.ptr(), item) }
    }
    fn scrollList(&self, dx: c_int, dy: c_int) -> c_int {
        unsafe { wxListCtrl_ScrollList(self.ptr(), dx, dy) }
    }
    fn setColumn<T: ListItemMethods>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_SetColumn(self.ptr(), col, item.ptr()) }
    }
    fn setColumnWidth(&self, col: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_SetColumnWidth(self.ptr(), col, width) }
    }
    fn setImageList<T: ImageListMethods>(&self, imageList: &T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.ptr(), imageList.ptr(), which) }
    }
    fn setItem(&self, index: c_int, col: c_int, label: &str, imageId: c_int) -> c_int {
        let label = strToString(label);
        unsafe { wxListCtrl_SetItem(self.ptr(), index, col, label.ptr(), imageId) }
    }
    fn setItemData(&self, item: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemData(self.ptr(), item, data) }
    }
    fn setItemFromInfo<T: ListItemMethods>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_SetItemFromInfo(self.ptr(), info.ptr()) }
    }
    fn setItemImage(&self, item: c_int, image: c_int, selImage: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemImage(self.ptr(), item, image, selImage) }
    }
    fn setItemPosition(&self, item: c_int, x: c_int, y: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemPosition(self.ptr(), item, x, y) }
    }
    fn setItemState(&self, item: c_int, state: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemState(self.ptr(), item, state, stateMask) }
    }
    fn setItemText(&self, item: c_int, str: &str) {
        let str = strToString(str);
        unsafe { wxListCtrl_SetItemText(self.ptr(), item, str.ptr()) }
    }
    fn setSingleStyle(&self, style: c_int, add: c_int) {
        unsafe { wxListCtrl_SetSingleStyle(self.ptr(), style, add) }
    }
    fn setTextColour<T: ColourMethods>(&self, col: &T) {
        unsafe { wxListCtrl_SetTextColour(self.ptr(), col.ptr()) }
    }
    fn sortItems(&self, fn_: *mut c_void, eif_obj: *mut c_void) -> c_int {
        unsafe { wxListCtrl_SortItems(self.ptr(), fn_, eif_obj) }
    }
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self.ptr()) }
    }
    fn assignImageList<T: ImageListMethods>(&self, images: &T, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.ptr(), images.ptr(), which) }
    }
    fn getColumn2<T: ListItemMethods>(&self, col: c_int, item: &T) {
        unsafe { wxListCtrl_GetColumn2(self.ptr(), col, item.ptr()) }
    }
    fn getItem2<T: ListItemMethods>(&self, info: &T) {
        unsafe { wxListCtrl_GetItem2(self.ptr(), info.ptr()) }
    }
    fn getItemPosition2(&self, item: c_int) -> Point {
        unsafe { Point::from(wxListCtrl_GetItemPosition2(self.ptr(), item)) }
    }
    fn sortItems2<T: ClosureMethods>(&self, closure: &T) -> c_int {
        unsafe { wxListCtrl_SortItems2(self.ptr(), closure.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxListEvent](http://docs.wxwidgets.org/3.0/classwx_list_event.html) class.
pub struct ListEvent { ptr: *mut c_void }
impl ListEventMethods for ListEvent {}
impl NotifyEventMethods for ListEvent {}
impl CommandEventMethods for ListEvent {}
impl EventMethods for ListEvent {}
impl ObjectMethods for ListEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListEvent {
    pub fn from(ptr: *mut c_void) -> ListEvent { ListEvent { ptr: ptr } }
    pub fn null() -> ListEvent { ListEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxListEvent](http://docs.wxwidgets.org/3.0/classwx_list_event.html) class.
pub trait ListEventMethods : NotifyEventMethods {
    fn cancelled(&self) -> c_int {
        unsafe { wxListEvent_Cancelled(self.ptr()) }
    }
    fn getCode(&self) -> c_int {
        unsafe { wxListEvent_GetCode(self.ptr()) }
    }
    fn getColumn(&self) -> c_int {
        unsafe { wxListEvent_GetColumn(self.ptr()) }
    }
    fn getData(&self) -> c_int {
        unsafe { wxListEvent_GetData(self.ptr()) }
    }
    fn getImage(&self) -> c_int {
        unsafe { wxListEvent_GetImage(self.ptr()) }
    }
    fn getIndex(&self) -> c_int {
        unsafe { wxListEvent_GetIndex(self.ptr()) }
    }
    fn getItem<T: ListItemMethods>(&self, _ref: &T) {
        unsafe { wxListEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { String::from(wxListEvent_GetLabel(self.ptr())).to_str() }
    }
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.ptr()) }
    }
    fn getPoint(&self) -> Point {
        unsafe { Point::from(wxListEvent_GetPoint(self.ptr())) }
    }
    fn getText(&self) -> ~str {
        unsafe { String::from(wxListEvent_GetText(self.ptr())).to_str() }
    }
    fn getCacheFrom(&self) -> c_int {
        unsafe { wxListEvent_GetCacheFrom(self.ptr()) }
    }
    fn getCacheTo(&self) -> c_int {
        unsafe { wxListEvent_GetCacheTo(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxListItem](http://docs.wxwidgets.org/3.0/classwx_list_item.html) class.
pub struct ListItem { ptr: *mut c_void }
impl ListItemMethods for ListItem {}
impl ObjectMethods for ListItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListItem {
    pub fn from(ptr: *mut c_void) -> ListItem { ListItem { ptr: ptr } }
    pub fn null() -> ListItem { ListItem::from(0 as *mut c_void) }
    
    pub fn new() -> ListItem {
        unsafe { ListItem::from(wxListItem_Create()) }
    }
}

/// Methods of the wxWidgets' [wxListItem](http://docs.wxwidgets.org/3.0/classwx_list_item.html) class.
pub trait ListItemMethods : ObjectMethods {
    fn clear(&self) {
        unsafe { wxListItem_Clear(self.ptr()) }
    }
    fn clearAttributes(&self) {
        unsafe { wxListItem_ClearAttributes(self.ptr()) }
    }
    fn getAlign(&self) -> c_int {
        unsafe { wxListItem_GetAlign(self.ptr()) }
    }
    fn getAttributes(&self) -> *mut c_void {
        unsafe { wxListItem_GetAttributes(self.ptr()) }
    }
    fn getBackgroundColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxListItem_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getColumn(&self) -> c_int {
        unsafe { wxListItem_GetColumn(self.ptr()) }
    }
    fn getData(&self) -> c_int {
        unsafe { wxListItem_GetData(self.ptr()) }
    }
    fn getFont<T: FontMethods>(&self, _ref: &T) {
        unsafe { wxListItem_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getId(&self) -> c_int {
        unsafe { wxListItem_GetId(self.ptr()) }
    }
    fn getImage(&self) -> c_int {
        unsafe { wxListItem_GetImage(self.ptr()) }
    }
    fn getMask(&self) -> c_int {
        unsafe { wxListItem_GetMask(self.ptr()) }
    }
    fn getState(&self) -> c_int {
        unsafe { wxListItem_GetState(self.ptr()) }
    }
    fn getText(&self) -> ~str {
        unsafe { String::from(wxListItem_GetText(self.ptr())).to_str() }
    }
    fn getTextColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxListItem_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxListItem_GetWidth(self.ptr()) }
    }
    fn hasAttributes(&self) -> c_int {
        unsafe { wxListItem_HasAttributes(self.ptr()) }
    }
    fn setAlign(&self, align: c_int) {
        unsafe { wxListItem_SetAlign(self.ptr(), align) }
    }
    fn setBackgroundColour<T: ColourMethods>(&self, colBack: &T) {
        unsafe { wxListItem_SetBackgroundColour(self.ptr(), colBack.ptr()) }
    }
    fn setColumn(&self, col: c_int) {
        unsafe { wxListItem_SetColumn(self.ptr(), col) }
    }
    fn setData(&self, data: c_int) {
        unsafe { wxListItem_SetData(self.ptr(), data) }
    }
    fn setDataPointer(&self, data: *mut c_void) {
        unsafe { wxListItem_SetDataPointer(self.ptr(), data) }
    }
    fn setFont<T: FontMethods>(&self, font: &T) {
        unsafe { wxListItem_SetFont(self.ptr(), font.ptr()) }
    }
    fn setId(&self, id: c_int) {
        unsafe { wxListItem_SetId(self.ptr(), id) }
    }
    fn setImage(&self, image: c_int) {
        unsafe { wxListItem_SetImage(self.ptr(), image) }
    }
    fn setMask(&self, mask: c_int) {
        unsafe { wxListItem_SetMask(self.ptr(), mask) }
    }
    fn setState(&self, state: c_int) {
        unsafe { wxListItem_SetState(self.ptr(), state) }
    }
    fn setStateMask(&self, stateMask: c_int) {
        unsafe { wxListItem_SetStateMask(self.ptr(), stateMask) }
    }
    fn setText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxListItem_SetText(self.ptr(), text.ptr()) }
    }
    fn setTextColour<T: ColourMethods>(&self, colText: &T) {
        unsafe { wxListItem_SetTextColour(self.ptr(), colText.ptr()) }
    }
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.ptr(), width) }
    }
}

/// Wraps the wxWidgets' [wxLog](http://docs.wxwidgets.org/3.0/classwx_log.html) class.
/// Rather use the wxRust-specific [RustLog](struct.RustLog.html) class.
pub struct Log { ptr: *mut c_void }
impl LogMethods for Log { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Log {
    pub fn from(ptr: *mut c_void) -> Log { Log { ptr: ptr } }
    pub fn null() -> Log { Log::from(0 as *mut c_void) }
    
    pub fn getActiveTarget() -> Log {
        unsafe { Log::from(wxLog_GetActiveTarget()) }
    }
}

/// Methods of the wxWidgets' [wxLog](http://docs.wxwidgets.org/3.0/classwx_log.html) class.
pub trait LogMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn addTraceMask(&self, str: &str) {
        let str = strToString(str);
        unsafe { wxLog_AddTraceMask(self.ptr(), str.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxLog_Delete(self.ptr()) }
    }
    fn dontCreateOnDemand(&self) {
        unsafe { wxLog_DontCreateOnDemand(self.ptr()) }
    }
    fn flush(&self) {
        unsafe { wxLog_Flush(self.ptr()) }
    }
    fn flushActive(&self) {
        unsafe { wxLog_FlushActive(self.ptr()) }
    }
    fn getTimestamp(&self) -> *mut c_char {
        unsafe { wxLog_GetTimestamp(self.ptr()) }
    }
    fn getTraceMask(&self) -> c_int {
        unsafe { wxLog_GetTraceMask(self.ptr()) }
    }
    fn getVerbose(&self) -> c_int {
        unsafe { wxLog_GetVerbose(self.ptr()) }
    }
    fn hasPendingMessages(&self) -> c_int {
        unsafe { wxLog_HasPendingMessages(self.ptr()) }
    }
    fn isAllowedTraceMask<T: MaskMethods>(&self, mask: &T) -> c_int {
        unsafe { wxLog_IsAllowedTraceMask(self.ptr(), mask.ptr()) }
    }
    fn onLog(&self, level: c_int, szString: *mut c_void, t: c_int) {
        unsafe { wxLog_OnLog(self.ptr(), level, szString, t) }
    }
    fn removeTraceMask(&self, str: &str) {
        let str = strToString(str);
        unsafe { wxLog_RemoveTraceMask(self.ptr(), str.ptr()) }
    }
    fn resume(&self) {
        unsafe { wxLog_Resume(self.ptr()) }
    }
    fn setActiveTarget(&self) -> Log {
        unsafe { Log::from(wxLog_SetActiveTarget(self.ptr())) }
    }
    fn setTimestamp(&self, ts: *mut c_void) {
        unsafe { wxLog_SetTimestamp(self.ptr(), ts) }
    }
    fn setTraceMask(&self, ulMask: c_int) {
        unsafe { wxLog_SetTraceMask(self.ptr(), ulMask) }
    }
    fn setVerbose(&self, bVerbose: c_int) {
        unsafe { wxLog_SetVerbose(self.ptr(), bVerbose) }
    }
    fn suspend(&self) {
        unsafe { wxLog_Suspend(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxLogChain](http://docs.wxwidgets.org/3.0/classwx_log_chain.html) class.
pub struct LogChain { ptr: *mut c_void }
impl LogChainMethods for LogChain {}
impl LogMethods for LogChain { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogChain {
    pub fn from(ptr: *mut c_void) -> LogChain { LogChain { ptr: ptr } }
    pub fn null() -> LogChain { LogChain::from(0 as *mut c_void) }
    
    pub fn new<T: LogMethods>(logger: &T) -> LogChain {
        unsafe { LogChain::from(wxLogChain_Create(logger.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxLogChain](http://docs.wxwidgets.org/3.0/classwx_log_chain.html) class.
pub trait LogChainMethods : LogMethods {
    fn getOldLog(&self) -> Log {
        unsafe { Log::from(wxLogChain_GetOldLog(self.ptr())) }
    }
    fn isPassingMessages(&self) -> c_int {
        unsafe { wxLogChain_IsPassingMessages(self.ptr()) }
    }
    fn passMessages(&self, bDoPass: c_int) {
        unsafe { wxLogChain_PassMessages(self.ptr(), bDoPass) }
    }
    fn setLog<T: LogMethods>(&self, logger: &T) {
        unsafe { wxLogChain_SetLog(self.ptr(), logger.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxLogGUI](http://docs.wxwidgets.org/3.0/classwx_log_gui.html) class.
pub struct LogGUI { ptr: *mut c_void }
impl LogGUIMethods for LogGUI {}
impl LogMethods for LogGUI { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogGUI {
    pub fn from(ptr: *mut c_void) -> LogGUI { LogGUI { ptr: ptr } }
    pub fn null() -> LogGUI { LogGUI::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxLogGUI](http://docs.wxwidgets.org/3.0/classwx_log_gui.html) class.
pub trait LogGUIMethods : LogMethods {
}

/// Wraps the wxWidgets' [wxLogNull](http://docs.wxwidgets.org/3.0/classwx_log_null.html) class.
pub struct LogNull { ptr: *mut c_void }
impl LogNullMethods for LogNull {}
impl LogMethods for LogNull { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogNull {
    pub fn from(ptr: *mut c_void) -> LogNull { LogNull { ptr: ptr } }
    pub fn null() -> LogNull { LogNull::from(0 as *mut c_void) }
    
    pub fn new() -> LogNull {
        unsafe { LogNull::from(wxLogNull_Create()) }
    }
}

/// Methods of the wxWidgets' [wxLogNull](http://docs.wxwidgets.org/3.0/classwx_log_null.html) class.
pub trait LogNullMethods : LogMethods {
}

/// Wraps the wxWidgets' [wxLogPassThrough](http://docs.wxwidgets.org/3.0/classwx_log_pass_through.html) class.
pub struct LogPassThrough { ptr: *mut c_void }
impl LogPassThroughMethods for LogPassThrough {}
impl LogChainMethods for LogPassThrough {}
impl LogMethods for LogPassThrough { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogPassThrough {
    pub fn from(ptr: *mut c_void) -> LogPassThrough { LogPassThrough { ptr: ptr } }
    pub fn null() -> LogPassThrough { LogPassThrough::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxLogPassThrough](http://docs.wxwidgets.org/3.0/classwx_log_pass_through.html) class.
pub trait LogPassThroughMethods : LogChainMethods {
}

/// Wraps the wxWidgets' [wxLogStderr](http://docs.wxwidgets.org/3.0/classwx_log_stderr.html) class.
pub struct LogStderr { ptr: *mut c_void }
impl LogStderrMethods for LogStderr {}
impl LogMethods for LogStderr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogStderr {
    pub fn from(ptr: *mut c_void) -> LogStderr { LogStderr { ptr: ptr } }
    pub fn null() -> LogStderr { LogStderr::from(0 as *mut c_void) }
    
    pub fn new() -> LogStderr {
        unsafe { LogStderr::from(wxLogStderr_Create()) }
    }
    pub fn newStdOut() -> LogStderr {
        unsafe { LogStderr::from(wxLogStderr_CreateStdOut()) }
    }
}

/// Methods of the wxWidgets' [wxLogStderr](http://docs.wxwidgets.org/3.0/classwx_log_stderr.html) class.
pub trait LogStderrMethods : LogMethods {
}

/// Wraps the wxWidgets' [wxLogStream](http://docs.wxwidgets.org/3.0/classwx_log_stream.html) class.
pub struct LogStream { ptr: *mut c_void }
impl LogStreamMethods for LogStream {}
impl LogMethods for LogStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogStream {
    pub fn from(ptr: *mut c_void) -> LogStream { LogStream { ptr: ptr } }
    pub fn null() -> LogStream { LogStream::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxLogStream](http://docs.wxwidgets.org/3.0/classwx_log_stream.html) class.
pub trait LogStreamMethods : LogMethods {
}

/// Wraps the wxWidgets' [wxLogTextCtrl](http://docs.wxwidgets.org/3.0/classwx_log_text_ctrl.html) class.
pub struct LogTextCtrl { ptr: *mut c_void }
impl LogTextCtrlMethods for LogTextCtrl {}
impl LogMethods for LogTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogTextCtrl {
    pub fn from(ptr: *mut c_void) -> LogTextCtrl { LogTextCtrl { ptr: ptr } }
    pub fn null() -> LogTextCtrl { LogTextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TextCtrlMethods>(text: &T) -> LogTextCtrl {
        unsafe { LogTextCtrl::from(wxLogTextCtrl_Create(text.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxLogTextCtrl](http://docs.wxwidgets.org/3.0/classwx_log_text_ctrl.html) class.
pub trait LogTextCtrlMethods : LogMethods {
}

/// Wraps the wxWidgets' [wxLogWindow](http://docs.wxwidgets.org/3.0/classwx_log_window.html) class.
pub struct LogWindow { ptr: *mut c_void }
impl LogWindowMethods for LogWindow {}
impl LogPassThroughMethods for LogWindow {}
impl LogChainMethods for LogWindow {}
impl LogMethods for LogWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogWindow {
    pub fn from(ptr: *mut c_void) -> LogWindow { LogWindow { ptr: ptr } }
    pub fn null() -> LogWindow { LogWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(parent: &T, title: *mut int8_t, showit: c_int, passthrough: c_int) -> LogWindow {
        unsafe { LogWindow::from(wxLogWindow_Create(parent.ptr(), title, showit, passthrough)) }
    }
}

/// Methods of the wxWidgets' [wxLogWindow](http://docs.wxwidgets.org/3.0/classwx_log_window.html) class.
pub trait LogWindowMethods : LogPassThroughMethods {
    fn getFrame(&self) -> Frame {
        unsafe { Frame::from(wxLogWindow_GetFrame(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxMDIChildFrame](http://docs.wxwidgets.org/3.0/classwx_mdic_hild_frame.html) class.
pub struct MDIChildFrame { ptr: *mut c_void }
impl MDIChildFrameMethods for MDIChildFrame {}
impl FrameMethods for MDIChildFrame {}
impl TopLevelWindowMethods for MDIChildFrame {}
impl WindowMethods for MDIChildFrame {}
impl EvtHandlerMethods for MDIChildFrame {}
impl ObjectMethods for MDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MDIChildFrame {
    pub fn from(ptr: *mut c_void) -> MDIChildFrame { MDIChildFrame { ptr: ptr } }
    pub fn null() -> MDIChildFrame { MDIChildFrame::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> MDIChildFrame {
        let _txt = strToString(_txt);
        unsafe { MDIChildFrame::from(wxMDIChildFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxMDIChildFrame](http://docs.wxwidgets.org/3.0/classwx_mdic_hild_frame.html) class.
pub trait MDIChildFrameMethods : FrameMethods {
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMDIClientWindow](http://docs.wxwidgets.org/3.0/classwx_mdic_lient_window.html) class.
pub struct MDIClientWindow { ptr: *mut c_void }
impl MDIClientWindowMethods for MDIClientWindow {}
impl WindowMethods for MDIClientWindow {}
impl EvtHandlerMethods for MDIClientWindow {}
impl ObjectMethods for MDIClientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MDIClientWindow {
    pub fn from(ptr: *mut c_void) -> MDIClientWindow { MDIClientWindow { ptr: ptr } }
    pub fn null() -> MDIClientWindow { MDIClientWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMDIClientWindow](http://docs.wxwidgets.org/3.0/classwx_mdic_lient_window.html) class.
pub trait MDIClientWindowMethods : WindowMethods {
}

/// Wraps the wxWidgets' [wxMDIParentFrame](http://docs.wxwidgets.org/3.0/classwx_mdip_arent_frame.html) class.
pub struct MDIParentFrame { ptr: *mut c_void }
impl MDIParentFrameMethods for MDIParentFrame {}
impl FrameMethods for MDIParentFrame {}
impl TopLevelWindowMethods for MDIParentFrame {}
impl WindowMethods for MDIParentFrame {}
impl EvtHandlerMethods for MDIParentFrame {}
impl ObjectMethods for MDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MDIParentFrame {
    pub fn from(ptr: *mut c_void) -> MDIParentFrame { MDIParentFrame { ptr: ptr } }
    pub fn null() -> MDIParentFrame { MDIParentFrame::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> MDIParentFrame {
        let _txt = strToString(_txt);
        unsafe { MDIParentFrame::from(wxMDIParentFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxMDIParentFrame](http://docs.wxwidgets.org/3.0/classwx_mdip_arent_frame.html) class.
pub trait MDIParentFrameMethods : FrameMethods {
    fn activateNext(&self) {
        unsafe { wxMDIParentFrame_ActivateNext(self.ptr()) }
    }
    fn activatePrevious(&self) {
        unsafe { wxMDIParentFrame_ActivatePrevious(self.ptr()) }
    }
    fn arrangeIcons(&self) {
        unsafe { wxMDIParentFrame_ArrangeIcons(self.ptr()) }
    }
    fn cascade(&self) {
        unsafe { wxMDIParentFrame_Cascade(self.ptr()) }
    }
    fn getActiveChild(&self) -> MDIChildFrame {
        unsafe { MDIChildFrame::from(wxMDIParentFrame_GetActiveChild(self.ptr())) }
    }
    fn getClientWindow(&self) -> MDIClientWindow {
        unsafe { MDIClientWindow::from(wxMDIParentFrame_GetClientWindow(self.ptr())) }
    }
    fn getWindowMenu(&self) -> Menu {
        unsafe { Menu::from(wxMDIParentFrame_GetWindowMenu(self.ptr())) }
    }
    fn onCreateClient(&self) -> MDIClientWindow {
        unsafe { MDIClientWindow::from(wxMDIParentFrame_OnCreateClient(self.ptr())) }
    }
    fn setWindowMenu<T: MenuMethods>(&self, menu: &T) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.ptr(), menu.ptr()) }
    }
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMask](http://docs.wxwidgets.org/3.0/classwx_mask.html) class.
pub struct Mask { ptr: *mut c_void }
impl MaskMethods for Mask {}
impl ObjectMethods for Mask { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Mask {
    pub fn from(ptr: *mut c_void) -> Mask { Mask { ptr: ptr } }
    pub fn null() -> Mask { Mask::from(0 as *mut c_void) }
    
    pub fn new<T: BitmapMethods>(bitmap: &T) -> Mask {
        unsafe { Mask::from(wxMask_Create(bitmap.ptr())) }
    }
    pub fn newColoured<T: BitmapMethods, U: ColourMethods>(bitmap: &T, colour: &U) -> *mut c_void {
        unsafe { wxMask_CreateColoured(bitmap.ptr(), colour.ptr()) }
    }
}

/// Methods of the wxWidgets' [wxMask](http://docs.wxwidgets.org/3.0/classwx_mask.html) class.
pub trait MaskMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxMaximizeEvent](http://docs.wxwidgets.org/3.0/classwx_maximize_event.html) class.
pub struct MaximizeEvent { ptr: *mut c_void }
impl MaximizeEventMethods for MaximizeEvent {}
impl EventMethods for MaximizeEvent {}
impl ObjectMethods for MaximizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MaximizeEvent {
    pub fn from(ptr: *mut c_void) -> MaximizeEvent { MaximizeEvent { ptr: ptr } }
    pub fn null() -> MaximizeEvent { MaximizeEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMaximizeEvent](http://docs.wxwidgets.org/3.0/classwx_maximize_event.html) class.
pub trait MaximizeEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxMemoryDC](http://docs.wxwidgets.org/3.0/classwx_memory_dc.html) class.
pub struct MemoryDC { ptr: *mut c_void }
impl MemoryDCMethods for MemoryDC {}
impl DCMethods for MemoryDC {}
impl ObjectMethods for MemoryDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryDC {
    pub fn from(ptr: *mut c_void) -> MemoryDC { MemoryDC { ptr: ptr } }
    pub fn null() -> MemoryDC { MemoryDC::from(0 as *mut c_void) }
    
    pub fn new() -> MemoryDC {
        unsafe { MemoryDC::from(wxMemoryDC_Create()) }
    }
    pub fn newCompatible<T: DCMethods>(dc: &T) -> MemoryDC {
        unsafe { MemoryDC::from(wxMemoryDC_CreateCompatible(dc.ptr())) }
    }
    pub fn newWithBitmap<T: BitmapMethods>(bitmap: &T) -> MemoryDC {
        unsafe { MemoryDC::from(wxMemoryDC_CreateWithBitmap(bitmap.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxMemoryDC](http://docs.wxwidgets.org/3.0/classwx_memory_dc.html) class.
pub trait MemoryDCMethods : DCMethods {
    fn selectObject<T: BitmapMethods>(&self, bitmap: &T) {
        unsafe { wxMemoryDC_SelectObject(self.ptr(), bitmap.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMenu](http://docs.wxwidgets.org/3.0/classwx_menu.html) class.
pub struct Menu { ptr: *mut c_void }
impl MenuMethods for Menu {}
impl EvtHandlerMethods for Menu {}
impl ObjectMethods for Menu { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Menu {
    pub fn from(ptr: *mut c_void) -> Menu { Menu { ptr: ptr } }
    pub fn null() -> Menu { Menu::from(0 as *mut c_void) }
    
    pub fn new(title: &str, style: c_long) -> Menu {
        let title = strToString(title);
        unsafe { Menu::from(wxMenu_Create(title.ptr(), style)) }
    }
}

/// Methods of the wxWidgets' [wxMenu](http://docs.wxwidgets.org/3.0/classwx_menu.html) class.
pub trait MenuMethods : EvtHandlerMethods {
    fn append(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_Append(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn appendItem<T: MenuItemMethods>(&self, _itm: &T) {
        unsafe { wxMenu_AppendItem(self.ptr(), _itm.ptr()) }
    }
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.ptr()) }
    }
    fn appendSub<T: MenuMethods>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_AppendSub(self.ptr(), id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    fn break_(&self) {
        unsafe { wxMenu_Break(self.ptr()) }
    }
    fn check(&self, id: c_int, check: c_int) {
        unsafe { wxMenu_Check(self.ptr(), id, check) }
    }
    fn deleteById(&self, id: c_int) {
        unsafe { wxMenu_DeleteById(self.ptr(), id) }
    }
    fn deleteByItem<T: MenuItemMethods>(&self, _itm: &T) {
        unsafe { wxMenu_DeleteByItem(self.ptr(), _itm.ptr()) }
    }
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self.ptr()) }
    }
    fn destroyById(&self, id: c_int) {
        unsafe { wxMenu_DestroyById(self.ptr(), id) }
    }
    fn destroyByItem<T: MenuItemMethods>(&self, _itm: &T) {
        unsafe { wxMenu_DestroyByItem(self.ptr(), _itm.ptr()) }
    }
    fn enable(&self, id: c_int, enable: c_int) {
        unsafe { wxMenu_Enable(self.ptr(), id, enable) }
    }
    fn findItem(&self, id: c_int) -> MenuItem {
        unsafe { MenuItem::from(wxMenu_FindItem(self.ptr(), id)) }
    }
    fn findItemByLabel(&self, itemString: &str) -> c_int {
        let itemString = strToString(itemString);
        unsafe { wxMenu_FindItemByLabel(self.ptr(), itemString.ptr()) }
    }
    fn getClientData(&self) -> ClientData {
        unsafe { ClientData::from(wxMenu_GetClientData(self.ptr())) }
    }
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { String::from(wxMenu_GetHelpString(self.ptr(), id)).to_str() }
    }
    fn getInvokingWindow(&self) -> Window {
        unsafe { Window::from(wxMenu_GetInvokingWindow(self.ptr())) }
    }
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { String::from(wxMenu_GetLabel(self.ptr(), id)).to_str() }
    }
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.ptr()) }
    }
    fn getMenuItems<T: ListMethods>(&self, _lst: &T) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.ptr(), _lst.ptr()) }
    }
    fn getParent(&self) -> Menu {
        unsafe { Menu::from(wxMenu_GetParent(self.ptr())) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.ptr()) }
    }
    fn getTitle(&self) -> ~str {
        unsafe { String::from(wxMenu_GetTitle(self.ptr())).to_str() }
    }
    fn insert(&self, pos: size_t, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_Insert(self.ptr(), pos, id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn insertItem<T: MenuItemMethods>(&self, pos: size_t, _itm: &T) {
        unsafe { wxMenu_InsertItem(self.ptr(), pos, _itm.ptr()) }
    }
    fn insertSub<T: MenuMethods>(&self, pos: size_t, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_InsertSub(self.ptr(), pos, id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    fn isAttached(&self) -> c_int {
        unsafe { wxMenu_IsAttached(self.ptr()) }
    }
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenu_IsChecked(self.ptr(), id) }
    }
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenu_IsEnabled(self.ptr(), id) }
    }
    fn prepend(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_Prepend(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn prependItem<T: MenuItemMethods>(&self, _itm: &T) {
        unsafe { wxMenu_PrependItem(self.ptr(), _itm.ptr()) }
    }
    fn prependSub<T: MenuMethods>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_PrependSub(self.ptr(), id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    fn removeById<T: MenuItemMethods>(&self, id: c_int, _itm: &T) {
        unsafe { wxMenu_RemoveById(self.ptr(), id, _itm.ptr()) }
    }
    fn removeByItem(&self, item: *mut c_void) {
        unsafe { wxMenu_RemoveByItem(self.ptr(), item) }
    }
    fn setClientData<T: ClientDataMethods>(&self, clientData: &T) {
        unsafe { wxMenu_SetClientData(self.ptr(), clientData.ptr()) }
    }
    fn setEventHandler<T: EvtHandlerMethods>(&self, handler: &T) {
        unsafe { wxMenu_SetEventHandler(self.ptr(), handler.ptr()) }
    }
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = strToString(helpString);
        unsafe { wxMenu_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    fn setInvokingWindow<T: WindowMethods>(&self, win: &T) {
        unsafe { wxMenu_SetInvokingWindow(self.ptr(), win.ptr()) }
    }
    fn setLabel(&self, id: c_int, label: &str) {
        let label = strToString(label);
        unsafe { wxMenu_SetLabel(self.ptr(), id, label.ptr()) }
    }
    fn setParent<T: WindowMethods>(&self, parent: &T) {
        unsafe { wxMenu_SetParent(self.ptr(), parent.ptr()) }
    }
    fn setTitle(&self, title: &str) {
        let title = strToString(title);
        unsafe { wxMenu_SetTitle(self.ptr(), title.ptr()) }
    }
    fn updateUI(&self, source: *mut c_void) {
        unsafe { wxMenu_UpdateUI(self.ptr(), source) }
    }
    fn getMenuBar(&self) -> MenuBar {
        unsafe { MenuBar::from(wxMenu_GetMenuBar(self.ptr())) }
    }
    fn appendRadioItem(&self, id: c_int, text: &str, help: &str) {
        let text = strToString(text);
        let help = strToString(help);
        unsafe { wxMenu_AppendRadioItem(self.ptr(), id, text.ptr(), help.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMenuBar](http://docs.wxwidgets.org/3.0/classwx_menu_bar.html) class.
pub struct MenuBar { ptr: *mut c_void }
impl MenuBarMethods for MenuBar {}
impl EvtHandlerMethods for MenuBar {}
impl ObjectMethods for MenuBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MenuBar {
    pub fn from(ptr: *mut c_void) -> MenuBar { MenuBar { ptr: ptr } }
    pub fn null() -> MenuBar { MenuBar::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> MenuBar {
        unsafe { MenuBar::from(wxMenuBar_Create(_style)) }
    }
}

/// Methods of the wxWidgets' [wxMenuBar](http://docs.wxwidgets.org/3.0/classwx_menu_bar.html) class.
pub trait MenuBarMethods : EvtHandlerMethods {
    fn append<T: MenuMethods>(&self, menu: &T, title: &str) -> c_int {
        let title = strToString(title);
        unsafe { wxMenuBar_Append(self.ptr(), menu.ptr(), title.ptr()) }
    }
    fn check(&self, id: c_int, check: c_int) {
        unsafe { wxMenuBar_Check(self.ptr(), id, check) }
    }
    fn deletePointer(&self) {
        unsafe { wxMenuBar_DeletePointer(self.ptr()) }
    }
    fn enable(&self, enable: c_int) -> c_int {
        unsafe { wxMenuBar_Enable(self.ptr(), enable) }
    }
    fn enableItem(&self, id: c_int, enable: c_int) {
        unsafe { wxMenuBar_EnableItem(self.ptr(), id, enable) }
    }
    fn enableTop(&self, pos: c_int, enable: c_int) {
        unsafe { wxMenuBar_EnableTop(self.ptr(), pos, enable) }
    }
    fn findItem(&self, id: c_int) -> MenuItem {
        unsafe { MenuItem::from(wxMenuBar_FindItem(self.ptr(), id)) }
    }
    fn findMenu(&self, title: &str) -> c_int {
        let title = strToString(title);
        unsafe { wxMenuBar_FindMenu(self.ptr(), title.ptr()) }
    }
    fn findMenuItem(&self, menuString: &str, itemString: &str) -> c_int {
        let menuString = strToString(menuString);
        let itemString = strToString(itemString);
        unsafe { wxMenuBar_FindMenuItem(self.ptr(), menuString.ptr(), itemString.ptr()) }
    }
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { String::from(wxMenuBar_GetHelpString(self.ptr(), id)).to_str() }
    }
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { String::from(wxMenuBar_GetLabel(self.ptr(), id)).to_str() }
    }
    fn getLabelTop(&self, pos: c_int) -> ~str {
        unsafe { String::from(wxMenuBar_GetLabelTop(self.ptr(), pos)).to_str() }
    }
    fn getMenu(&self, pos: c_int) -> Menu {
        unsafe { Menu::from(wxMenuBar_GetMenu(self.ptr(), pos)) }
    }
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.ptr()) }
    }
    fn insert<T: MenuMethods>(&self, pos: c_int, menu: &T, title: &str) -> c_int {
        let title = strToString(title);
        unsafe { wxMenuBar_Insert(self.ptr(), pos, menu.ptr(), title.ptr()) }
    }
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsChecked(self.ptr(), id) }
    }
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsEnabled(self.ptr(), id) }
    }
    fn remove(&self, pos: c_int) -> Menu {
        unsafe { Menu::from(wxMenuBar_Remove(self.ptr(), pos)) }
    }
    fn replace<T: MenuMethods>(&self, pos: c_int, menu: &T, title: &str) -> Menu {
        let title = strToString(title);
        unsafe { Menu::from(wxMenuBar_Replace(self.ptr(), pos, menu.ptr(), title.ptr())) }
    }
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = strToString(helpString);
        unsafe { wxMenuBar_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    fn setItemLabel(&self, id: c_int, label: &str) {
        let label = strToString(label);
        unsafe { wxMenuBar_SetItemLabel(self.ptr(), id, label.ptr()) }
    }
    fn setLabel(&self, s: &str) {
        let s = strToString(s);
        unsafe { wxMenuBar_SetLabel(self.ptr(), s.ptr()) }
    }
    fn setLabelTop(&self, pos: c_int, label: &str) {
        let label = strToString(label);
        unsafe { wxMenuBar_SetLabelTop(self.ptr(), pos, label.ptr()) }
    }
    fn getFrame(&self) -> Frame {
        unsafe { Frame::from(wxMenuBar_GetFrame(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxMenuEvent](http://docs.wxwidgets.org/3.0/classwx_menu_event.html) class.
pub struct MenuEvent { ptr: *mut c_void }
impl MenuEventMethods for MenuEvent {}
impl EventMethods for MenuEvent {}
impl ObjectMethods for MenuEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MenuEvent {
    pub fn from(ptr: *mut c_void) -> MenuEvent { MenuEvent { ptr: ptr } }
    pub fn null() -> MenuEvent { MenuEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMenuEvent](http://docs.wxwidgets.org/3.0/classwx_menu_event.html) class.
pub trait MenuEventMethods : EventMethods {
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMenuItem](http://docs.wxwidgets.org/3.0/classwx_menu_item.html) class.
pub struct MenuItem { ptr: *mut c_void }
impl MenuItemMethods for MenuItem {}
impl ObjectMethods for MenuItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MenuItem {
    pub fn from(ptr: *mut c_void) -> MenuItem { MenuItem { ptr: ptr } }
    pub fn null() -> MenuItem { MenuItem::from(0 as *mut c_void) }
    
    pub fn new() -> MenuItem {
        unsafe { MenuItem::from(wxMenuItem_Create()) }
    }
    pub fn getLabelFromText(text: *mut c_void) -> ~str {
        unsafe { String::from(wxMenuItem_GetLabelFromText(text)).to_str() }
    }
    pub fn newSeparator() -> MenuItem {
        unsafe { MenuItem::from(wxMenuItem_CreateSeparator()) }
    }
    pub fn newEx<T: MenuMethods>(id: c_int, label: &str, help: &str, itemkind: c_int, submenu: &T) -> MenuItem {
        let label = strToString(label);
        let help = strToString(help);
        unsafe { MenuItem::from(wxMenuItem_CreateEx(id, label.ptr(), help.ptr(), itemkind, submenu.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxMenuItem](http://docs.wxwidgets.org/3.0/classwx_menu_item.html) class.
pub trait MenuItemMethods : ObjectMethods {
    fn check(&self, check: c_int) {
        unsafe { wxMenuItem_Check(self.ptr(), check) }
    }
    fn enable(&self, enable: c_int) {
        unsafe { wxMenuItem_Enable(self.ptr(), enable) }
    }
    fn getHelp(&self) -> ~str {
        unsafe { String::from(wxMenuItem_GetHelp(self.ptr())).to_str() }
    }
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { String::from(wxMenuItem_GetLabel(self.ptr())).to_str() }
    }
    fn getMenu(&self) -> Menu {
        unsafe { Menu::from(wxMenuItem_GetMenu(self.ptr())) }
    }
    fn getSubMenu(&self) -> Menu {
        unsafe { Menu::from(wxMenuItem_GetSubMenu(self.ptr())) }
    }
    fn getText(&self) -> ~str {
        unsafe { String::from(wxMenuItem_GetText(self.ptr())).to_str() }
    }
    fn isCheckable(&self) -> c_int {
        unsafe { wxMenuItem_IsCheckable(self.ptr()) }
    }
    fn isChecked(&self) -> c_int {
        unsafe { wxMenuItem_IsChecked(self.ptr()) }
    }
    fn isEnabled(&self) -> c_int {
        unsafe { wxMenuItem_IsEnabled(self.ptr()) }
    }
    fn isSeparator(&self) -> c_int {
        unsafe { wxMenuItem_IsSeparator(self.ptr()) }
    }
    fn isSubMenu(&self) -> c_int {
        unsafe { wxMenuItem_IsSubMenu(self.ptr()) }
    }
    fn setCheckable(&self, checkable: c_int) {
        unsafe { wxMenuItem_SetCheckable(self.ptr(), checkable) }
    }
    fn setHelp(&self, str: &str) {
        let str = strToString(str);
        unsafe { wxMenuItem_SetHelp(self.ptr(), str.ptr()) }
    }
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.ptr(), id) }
    }
    fn setSubMenu<T: MenuMethods>(&self, menu: &T) {
        unsafe { wxMenuItem_SetSubMenu(self.ptr(), menu.ptr()) }
    }
    fn setText(&self, str: &str) {
        let str = strToString(str);
        unsafe { wxMenuItem_SetText(self.ptr(), str.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMessageDialog](http://docs.wxwidgets.org/3.0/classwx_message_dialog.html) class.
pub struct MessageDialog { ptr: *mut c_void }
impl MessageDialogMethods for MessageDialog {}
impl DialogMethods for MessageDialog {}
impl TopLevelWindowMethods for MessageDialog {}
impl WindowMethods for MessageDialog {}
impl EvtHandlerMethods for MessageDialog {}
impl ObjectMethods for MessageDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MessageDialog {
    pub fn from(ptr: *mut c_void) -> MessageDialog { MessageDialog { ptr: ptr } }
    pub fn null() -> MessageDialog { MessageDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _msg: &str, _cap: &str, _stl: c_int) -> MessageDialog {
        let _msg = strToString(_msg);
        let _cap = strToString(_cap);
        unsafe { MessageDialog::from(wxMessageDialog_Create(_prt.ptr(), _msg.ptr(), _cap.ptr(), _stl)) }
    }
}

/// Methods of the wxWidgets' [wxMessageDialog](http://docs.wxwidgets.org/3.0/classwx_message_dialog.html) class.
pub trait MessageDialogMethods : DialogMethods {
}

/// Wraps the wxWidgets' [wxMetafile](http://docs.wxwidgets.org/3.0/classwx_metafile.html) class.
pub struct Metafile { ptr: *mut c_void }
impl MetafileMethods for Metafile {}
impl ObjectMethods for Metafile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Metafile {
    pub fn from(ptr: *mut c_void) -> Metafile { Metafile { ptr: ptr } }
    pub fn null() -> Metafile { Metafile::from(0 as *mut c_void) }
    
    pub fn new(_file: &str) -> Metafile {
        let _file = strToString(_file);
        unsafe { Metafile::from(wxMetafile_Create(_file.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxMetafile](http://docs.wxwidgets.org/3.0/classwx_metafile.html) class.
pub trait MetafileMethods : ObjectMethods {
    fn isOk(&self) -> c_int {
        unsafe { wxMetafile_IsOk(self.ptr()) }
    }
    fn play<T: DCMethods>(&self, _dc: &T) -> c_int {
        unsafe { wxMetafile_Play(self.ptr(), _dc.ptr()) }
    }
    fn setClipboard(&self, width: c_int, height: c_int) -> c_int {
        unsafe { wxMetafile_SetClipboard(self.ptr(), width, height) }
    }
}

/// Wraps the wxWidgets' [wxMetafileDC](http://docs.wxwidgets.org/3.0/classwx_metafile_dc.html) class.
pub struct MetafileDC { ptr: *mut c_void }
impl MetafileDCMethods for MetafileDC {}
impl DCMethods for MetafileDC {}
impl ObjectMethods for MetafileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MetafileDC {
    pub fn from(ptr: *mut c_void) -> MetafileDC { MetafileDC { ptr: ptr } }
    pub fn null() -> MetafileDC { MetafileDC::from(0 as *mut c_void) }
    
    pub fn new(_file: &str) -> MetafileDC {
        let _file = strToString(_file);
        unsafe { MetafileDC::from(wxMetafileDC_Create(_file.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxMetafileDC](http://docs.wxwidgets.org/3.0/classwx_metafile_dc.html) class.
pub trait MetafileDCMethods : DCMethods {
    fn close(&self) -> *mut c_void {
        unsafe { wxMetafileDC_Close(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMimeTypesManager](http://docs.wxwidgets.org/3.0/classwx_mime_types_manager.html) class.
pub struct MimeTypesManager { ptr: *mut c_void }
impl MimeTypesManagerMethods for MimeTypesManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MimeTypesManager {
    pub fn from(ptr: *mut c_void) -> MimeTypesManager { MimeTypesManager { ptr: ptr } }
    pub fn null() -> MimeTypesManager { MimeTypesManager::from(0 as *mut c_void) }
    
    pub fn new() -> MimeTypesManager {
        unsafe { MimeTypesManager::from(wxMimeTypesManager_Create()) }
    }
}

/// Methods of the wxWidgets' [wxMimeTypesManager](http://docs.wxwidgets.org/3.0/classwx_mime_types_manager.html) class.
pub trait MimeTypesManagerMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn addFallbacks(&self, _types: *mut c_void) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.ptr(), _types) }
    }
    fn enumAllFileTypes<T: ListMethods>(&self, _lst: &T) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.ptr(), _lst.ptr()) }
    }
    fn getFileTypeFromExtension(&self, _ext: &str) -> FileType {
        let _ext = strToString(_ext);
        unsafe { FileType::from(wxMimeTypesManager_GetFileTypeFromExtension(self.ptr(), _ext.ptr())) }
    }
    fn getFileTypeFromMimeType(&self, _name: &str) -> FileType {
        let _name = strToString(_name);
        unsafe { FileType::from(wxMimeTypesManager_GetFileTypeFromMimeType(self.ptr(), _name.ptr())) }
    }
    fn isOfType(&self, _type: &str, _wildcard: &str) -> c_int {
        let _type = strToString(_type);
        let _wildcard = strToString(_wildcard);
        unsafe { wxMimeTypesManager_IsOfType(self.ptr(), _type.ptr(), _wildcard.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMiniFrame](http://docs.wxwidgets.org/3.0/classwx_mini_frame.html) class.
pub struct MiniFrame { ptr: *mut c_void }
impl MiniFrameMethods for MiniFrame {}
impl FrameMethods for MiniFrame {}
impl TopLevelWindowMethods for MiniFrame {}
impl WindowMethods for MiniFrame {}
impl EvtHandlerMethods for MiniFrame {}
impl ObjectMethods for MiniFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MiniFrame {
    pub fn from(ptr: *mut c_void) -> MiniFrame { MiniFrame { ptr: ptr } }
    pub fn null() -> MiniFrame { MiniFrame::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> MiniFrame {
        let _txt = strToString(_txt);
        unsafe { MiniFrame::from(wxMiniFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxMiniFrame](http://docs.wxwidgets.org/3.0/classwx_mini_frame.html) class.
pub trait MiniFrameMethods : FrameMethods {
}

/// Wraps the wxWidgets' [wxMirrorDC](http://docs.wxwidgets.org/3.0/classwx_mirror_dc.html) class.
pub struct MirrorDC { ptr: *mut c_void }
impl MirrorDCMethods for MirrorDC {}
impl DCMethods for MirrorDC {}
impl ObjectMethods for MirrorDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MirrorDC {
    pub fn from(ptr: *mut c_void) -> MirrorDC { MirrorDC { ptr: ptr } }
    pub fn null() -> MirrorDC { MirrorDC::from(0 as *mut c_void) }
    
    pub fn new<T: DCMethods>(dc: &T) -> MirrorDC {
        unsafe { MirrorDC::from(wxMirrorDC_Create(dc.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxMirrorDC](http://docs.wxwidgets.org/3.0/classwx_mirror_dc.html) class.
pub trait MirrorDCMethods : DCMethods {
}

/// Wraps the wxWidgets' [wxMouseCaptureChangedEvent](http://docs.wxwidgets.org/3.0/classwx_mouse_capture_changed_event.html) class.
pub struct MouseCaptureChangedEvent { ptr: *mut c_void }
impl MouseCaptureChangedEventMethods for MouseCaptureChangedEvent {}
impl EventMethods for MouseCaptureChangedEvent {}
impl ObjectMethods for MouseCaptureChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MouseCaptureChangedEvent {
    pub fn from(ptr: *mut c_void) -> MouseCaptureChangedEvent { MouseCaptureChangedEvent { ptr: ptr } }
    pub fn null() -> MouseCaptureChangedEvent { MouseCaptureChangedEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMouseCaptureChangedEvent](http://docs.wxwidgets.org/3.0/classwx_mouse_capture_changed_event.html) class.
pub trait MouseCaptureChangedEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxMouseEvent](http://docs.wxwidgets.org/3.0/classwx_mouse_event.html) class.
pub struct MouseEvent { ptr: *mut c_void }
impl MouseEventMethods for MouseEvent {}
impl EventMethods for MouseEvent {}
impl ObjectMethods for MouseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MouseEvent {
    pub fn from(ptr: *mut c_void) -> MouseEvent { MouseEvent { ptr: ptr } }
    pub fn null() -> MouseEvent { MouseEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMouseEvent](http://docs.wxwidgets.org/3.0/classwx_mouse_event.html) class.
pub trait MouseEventMethods : EventMethods {
    fn altDown(&self) -> c_int {
        unsafe { wxMouseEvent_AltDown(self.ptr()) }
    }
    fn button(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_Button(self.ptr(), but) }
    }
    fn buttonDClick(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonDClick(self.ptr(), but) }
    }
    fn buttonDown(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonDown(self.ptr(), but) }
    }
    fn buttonIsDown(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonIsDown(self.ptr(), but) }
    }
    fn buttonUp(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonUp(self.ptr(), but) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxMouseEvent_ControlDown(self.ptr()) }
    }
    fn dragging(&self) -> c_int {
        unsafe { wxMouseEvent_Dragging(self.ptr()) }
    }
    fn entering(&self) -> c_int {
        unsafe { wxMouseEvent_Entering(self.ptr()) }
    }
    fn getLogicalPosition<T: DCMethods>(&self, dc: &T) -> Point {
        unsafe { Point::from(wxMouseEvent_GetLogicalPosition(self.ptr(), dc.ptr())) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxMouseEvent_GetPosition(self.ptr())) }
    }
    fn getX(&self) -> c_int {
        unsafe { wxMouseEvent_GetX(self.ptr()) }
    }
    fn getY(&self) -> c_int {
        unsafe { wxMouseEvent_GetY(self.ptr()) }
    }
    fn isButton(&self) -> c_int {
        unsafe { wxMouseEvent_IsButton(self.ptr()) }
    }
    fn leaving(&self) -> c_int {
        unsafe { wxMouseEvent_Leaving(self.ptr()) }
    }
    fn leftDClick(&self) -> c_int {
        unsafe { wxMouseEvent_LeftDClick(self.ptr()) }
    }
    fn leftDown(&self) -> c_int {
        unsafe { wxMouseEvent_LeftDown(self.ptr()) }
    }
    fn leftIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_LeftIsDown(self.ptr()) }
    }
    fn leftUp(&self) -> c_int {
        unsafe { wxMouseEvent_LeftUp(self.ptr()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxMouseEvent_MetaDown(self.ptr()) }
    }
    fn middleDClick(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleDClick(self.ptr()) }
    }
    fn middleDown(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleDown(self.ptr()) }
    }
    fn middleIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleIsDown(self.ptr()) }
    }
    fn middleUp(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleUp(self.ptr()) }
    }
    fn moving(&self) -> c_int {
        unsafe { wxMouseEvent_Moving(self.ptr()) }
    }
    fn rightDClick(&self) -> c_int {
        unsafe { wxMouseEvent_RightDClick(self.ptr()) }
    }
    fn rightDown(&self) -> c_int {
        unsafe { wxMouseEvent_RightDown(self.ptr()) }
    }
    fn rightIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_RightIsDown(self.ptr()) }
    }
    fn rightUp(&self) -> c_int {
        unsafe { wxMouseEvent_RightUp(self.ptr()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxMouseEvent_ShiftDown(self.ptr()) }
    }
    fn getWheelDelta(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelDelta(self.ptr()) }
    }
    fn getWheelRotation(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelRotation(self.ptr()) }
    }
    fn getButton(&self) -> c_int {
        unsafe { wxMouseEvent_GetButton(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxMoveEvent](http://docs.wxwidgets.org/3.0/classwx_move_event.html) class.
pub struct MoveEvent { ptr: *mut c_void }
impl MoveEventMethods for MoveEvent {}
impl EventMethods for MoveEvent {}
impl ObjectMethods for MoveEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MoveEvent {
    pub fn from(ptr: *mut c_void) -> MoveEvent { MoveEvent { ptr: ptr } }
    pub fn null() -> MoveEvent { MoveEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMoveEvent](http://docs.wxwidgets.org/3.0/classwx_move_event.html) class.
pub trait MoveEventMethods : EventMethods {
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxMoveEvent_GetPosition(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxNavigationKeyEvent](http://docs.wxwidgets.org/3.0/classwx_navigation_key_event.html) class.
pub struct NavigationKeyEvent { ptr: *mut c_void }
impl NavigationKeyEventMethods for NavigationKeyEvent {}
impl EventMethods for NavigationKeyEvent {}
impl ObjectMethods for NavigationKeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NavigationKeyEvent {
    pub fn from(ptr: *mut c_void) -> NavigationKeyEvent { NavigationKeyEvent { ptr: ptr } }
    pub fn null() -> NavigationKeyEvent { NavigationKeyEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxNavigationKeyEvent](http://docs.wxwidgets.org/3.0/classwx_navigation_key_event.html) class.
pub trait NavigationKeyEventMethods : EventMethods {
    fn getCurrentFocus(&self) -> *mut c_void {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.ptr()) }
    }
    fn getDirection(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_GetDirection(self.ptr()) }
    }
    fn isWindowChange(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.ptr()) }
    }
    fn setCurrentFocus<T: WindowMethods>(&self, win: &T) {
        unsafe { wxNavigationKeyEvent_SetCurrentFocus(self.ptr(), win.ptr()) }
    }
    fn setDirection(&self, bForward: c_int) {
        unsafe { wxNavigationKeyEvent_SetDirection(self.ptr(), bForward) }
    }
    fn setWindowChange(&self, bIs: c_int) {
        unsafe { wxNavigationKeyEvent_SetWindowChange(self.ptr(), bIs) }
    }
    fn shouldPropagate(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_ShouldPropagate(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxNotebook](http://docs.wxwidgets.org/3.0/classwx_notebook.html) class.
pub struct Notebook { ptr: *mut c_void }
impl NotebookMethods for Notebook {}
impl ControlMethods for Notebook {}
impl WindowMethods for Notebook {}
impl EvtHandlerMethods for Notebook {}
impl ObjectMethods for Notebook { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Notebook {
    pub fn from(ptr: *mut c_void) -> Notebook { Notebook { ptr: ptr } }
    pub fn null() -> Notebook { Notebook::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Notebook {
        unsafe { Notebook::from(wxNotebook_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxNotebook](http://docs.wxwidgets.org/3.0/classwx_notebook.html) class.
pub trait NotebookMethods : ControlMethods {
    fn addPage<T: WindowMethods>(&self, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = strToString(strText);
        unsafe { wxNotebook_AddPage(self.ptr(), pPage.ptr(), strText.ptr(), bSelect, imageId) }
    }
    fn advanceSelection(&self, bForward: c_int) {
        unsafe { wxNotebook_AdvanceSelection(self.ptr(), bForward) }
    }
    fn deleteAllPages(&self) -> c_int {
        unsafe { wxNotebook_DeleteAllPages(self.ptr()) }
    }
    fn deletePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_DeletePage(self.ptr(), nPage) }
    }
    fn getImageList(&self) -> ImageList {
        unsafe { ImageList::from(wxNotebook_GetImageList(self.ptr())) }
    }
    fn getPage(&self, nPage: c_int) -> Window {
        unsafe { Window::from(wxNotebook_GetPage(self.ptr(), nPage)) }
    }
    fn getPageCount(&self) -> c_int {
        unsafe { wxNotebook_GetPageCount(self.ptr()) }
    }
    fn getPageImage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_GetPageImage(self.ptr(), nPage) }
    }
    fn getPageText(&self, nPage: c_int) -> ~str {
        unsafe { String::from(wxNotebook_GetPageText(self.ptr(), nPage)).to_str() }
    }
    fn getRowCount(&self) -> c_int {
        unsafe { wxNotebook_GetRowCount(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxNotebook_GetSelection(self.ptr()) }
    }
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_long) -> c_int {
        unsafe { wxNotebook_HitTest(self.ptr(), x, y, flags) }
    }
    fn insertPage<T: WindowMethods>(&self, nPage: c_int, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = strToString(strText);
        unsafe { wxNotebook_InsertPage(self.ptr(), nPage, pPage.ptr(), strText.ptr(), bSelect, imageId) }
    }
    fn removePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_RemovePage(self.ptr(), nPage) }
    }
    fn setImageList<T: ImageListMethods>(&self, imageList: &T) {
        unsafe { wxNotebook_SetImageList(self.ptr(), imageList.ptr()) }
    }
    fn setPadding(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPadding(self.ptr(), _w, _h) }
    }
    fn setPageImage(&self, nPage: c_int, nImage: c_int) -> c_int {
        unsafe { wxNotebook_SetPageImage(self.ptr(), nPage, nImage) }
    }
    fn setPageSize(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPageSize(self.ptr(), _w, _h) }
    }
    fn setPageText(&self, nPage: c_int, strText: &str) -> c_int {
        let strText = strToString(strText);
        unsafe { wxNotebook_SetPageText(self.ptr(), nPage, strText.ptr()) }
    }
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.ptr(), nPage) }
    }
    fn assignImageList<T: ImageListMethods>(&self, imageList: &T) {
        unsafe { wxNotebook_AssignImageList(self.ptr(), imageList.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxNotebookEvent](http://docs.wxwidgets.org/3.0/classwx_notebook_event.html) class.
pub struct NotebookEvent { ptr: *mut c_void }
impl NotebookEventMethods for NotebookEvent {}
impl NotifyEventMethods for NotebookEvent {}
impl CommandEventMethods for NotebookEvent {}
impl EventMethods for NotebookEvent {}
impl ObjectMethods for NotebookEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NotebookEvent {
    pub fn from(ptr: *mut c_void) -> NotebookEvent { NotebookEvent { ptr: ptr } }
    pub fn null() -> NotebookEvent { NotebookEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxNotebookEvent](http://docs.wxwidgets.org/3.0/classwx_notebook_event.html) class.
pub trait NotebookEventMethods : NotifyEventMethods {
}

/// Wraps the wxWidgets' [wxNotifyEvent](http://docs.wxwidgets.org/3.0/classwx_notify_event.html) class.
pub struct NotifyEvent { ptr: *mut c_void }
impl NotifyEventMethods for NotifyEvent {}
impl CommandEventMethods for NotifyEvent {}
impl EventMethods for NotifyEvent {}
impl ObjectMethods for NotifyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NotifyEvent {
    pub fn from(ptr: *mut c_void) -> NotifyEvent { NotifyEvent { ptr: ptr } }
    pub fn null() -> NotifyEvent { NotifyEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxNotifyEvent](http://docs.wxwidgets.org/3.0/classwx_notify_event.html) class.
pub trait NotifyEventMethods : CommandEventMethods {
    fn allow(&self) {
        unsafe { wxNotifyEvent_Allow(self.ptr()) }
    }
    fn isAllowed(&self) -> c_int {
        unsafe { wxNotifyEvent_IsAllowed(self.ptr()) }
    }
    fn veto(&self) {
        unsafe { wxNotifyEvent_Veto(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPageSetupDialog](http://docs.wxwidgets.org/3.0/classwx_page_setup_dialog.html) class.
pub struct PageSetupDialog { ptr: *mut c_void }
impl PageSetupDialogMethods for PageSetupDialog {}
impl DialogMethods for PageSetupDialog {}
impl TopLevelWindowMethods for PageSetupDialog {}
impl WindowMethods for PageSetupDialog {}
impl EvtHandlerMethods for PageSetupDialog {}
impl ObjectMethods for PageSetupDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PageSetupDialog {
    pub fn from(ptr: *mut c_void) -> PageSetupDialog { PageSetupDialog { ptr: ptr } }
    pub fn null() -> PageSetupDialog { PageSetupDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: PageSetupDialogDataMethods>(parent: &T, data: &U) -> PageSetupDialog {
        unsafe { PageSetupDialog::from(wxPageSetupDialog_Create(parent.ptr(), data.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPageSetupDialog](http://docs.wxwidgets.org/3.0/classwx_page_setup_dialog.html) class.
pub trait PageSetupDialogMethods : DialogMethods {
    fn getPageSetupData<T: PageSetupDialogDataMethods>(&self, _ref: &T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.ptr(), _ref.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPageSetupDialogData](http://docs.wxwidgets.org/3.0/classwx_page_setup_dialog_data.html) class.
pub struct PageSetupDialogData { ptr: *mut c_void }
impl PageSetupDialogDataMethods for PageSetupDialogData {}
impl ObjectMethods for PageSetupDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PageSetupDialogData {
    pub fn from(ptr: *mut c_void) -> PageSetupDialogData { PageSetupDialogData { ptr: ptr } }
    pub fn null() -> PageSetupDialogData { PageSetupDialogData::from(0 as *mut c_void) }
    
    pub fn new() -> PageSetupDialogData {
        unsafe { PageSetupDialogData::from(wxPageSetupDialogData_Create()) }
    }
    pub fn newFromData<T: PrintDataMethods>(printData: &T) -> PageSetupDialogData {
        unsafe { PageSetupDialogData::from(wxPageSetupDialogData_CreateFromData(printData.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPageSetupDialogData](http://docs.wxwidgets.org/3.0/classwx_page_setup_dialog_data.html) class.
pub trait PageSetupDialogDataMethods : ObjectMethods {
    fn assign<T: PageSetupDialogDataMethods>(&self, data: &T) {
        unsafe { wxPageSetupDialogData_Assign(self.ptr(), data.ptr()) }
    }
    fn assignData<T: PrintDataMethods>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_AssignData(self.ptr(), printData.ptr()) }
    }
    fn calculateIdFromPaperSize(&self) {
        unsafe { wxPageSetupDialogData_CalculateIdFromPaperSize(self.ptr()) }
    }
    fn calculatePaperSizeFromId(&self) {
        unsafe { wxPageSetupDialogData_CalculatePaperSizeFromId(self.ptr()) }
    }
    fn enableHelp(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableHelp(self.ptr(), flag) }
    }
    fn enableMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableMargins(self.ptr(), flag) }
    }
    fn enableOrientation(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableOrientation(self.ptr(), flag) }
    }
    fn enablePaper(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnablePaper(self.ptr(), flag) }
    }
    fn enablePrinter(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnablePrinter(self.ptr(), flag) }
    }
    fn getDefaultInfo(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetDefaultInfo(self.ptr()) }
    }
    fn getDefaultMinMargins(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetDefaultMinMargins(self.ptr()) }
    }
    fn getEnableHelp(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableHelp(self.ptr()) }
    }
    fn getEnableMargins(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableMargins(self.ptr()) }
    }
    fn getEnableOrientation(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableOrientation(self.ptr()) }
    }
    fn getEnablePaper(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnablePaper(self.ptr()) }
    }
    fn getEnablePrinter(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnablePrinter(self.ptr()) }
    }
    fn getMarginBottomRight(&self) -> Point {
        unsafe { Point::from(wxPageSetupDialogData_GetMarginBottomRight(self.ptr())) }
    }
    fn getMarginTopLeft(&self) -> Point {
        unsafe { Point::from(wxPageSetupDialogData_GetMarginTopLeft(self.ptr())) }
    }
    fn getMinMarginBottomRight(&self) -> Point {
        unsafe { Point::from(wxPageSetupDialogData_GetMinMarginBottomRight(self.ptr())) }
    }
    fn getMinMarginTopLeft(&self) -> Point {
        unsafe { Point::from(wxPageSetupDialogData_GetMinMarginTopLeft(self.ptr())) }
    }
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.ptr()) }
    }
    fn getPaperSize(&self) -> Size {
        unsafe { Size::from(wxPageSetupDialogData_GetPaperSize(self.ptr())) }
    }
    fn getPrintData<T: PrintDataMethods>(&self, _ref: &T) {
        unsafe { wxPageSetupDialogData_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    fn setDefaultInfo(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultInfo(self.ptr(), flag) }
    }
    fn setDefaultMinMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultMinMargins(self.ptr(), flag) }
    }
    fn setMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginBottomRight(self.ptr(), x, y) }
    }
    fn setMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginTopLeft(self.ptr(), x, y) }
    }
    fn setMinMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginBottomRight(self.ptr(), x, y) }
    }
    fn setMinMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginTopLeft(self.ptr(), x, y) }
    }
    fn setPaperId(&self, id: *mut c_void) {
        unsafe { wxPageSetupDialogData_SetPaperId(self.ptr(), id) }
    }
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSize(self.ptr(), w, h) }
    }
    fn setPaperSizeId(&self, id: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSizeId(self.ptr(), id) }
    }
    fn setPrintData<T: PrintDataMethods>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.ptr(), printData.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPaintDC](http://docs.wxwidgets.org/3.0/classwx_paint_dc.html) class.
pub struct PaintDC { ptr: *mut c_void }
impl PaintDCMethods for PaintDC {}
impl WindowDCMethods for PaintDC {}
impl DCMethods for PaintDC {}
impl ObjectMethods for PaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PaintDC {
    pub fn from(ptr: *mut c_void) -> PaintDC { PaintDC { ptr: ptr } }
    pub fn null() -> PaintDC { PaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(win: &T) -> PaintDC {
        unsafe { PaintDC::from(wxPaintDC_Create(win.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPaintDC](http://docs.wxwidgets.org/3.0/classwx_paint_dc.html) class.
pub trait PaintDCMethods : WindowDCMethods {
}

/// Wraps the wxWidgets' [wxPaintEvent](http://docs.wxwidgets.org/3.0/classwx_paint_event.html) class.
pub struct PaintEvent { ptr: *mut c_void }
impl PaintEventMethods for PaintEvent {}
impl EventMethods for PaintEvent {}
impl ObjectMethods for PaintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PaintEvent {
    pub fn from(ptr: *mut c_void) -> PaintEvent { PaintEvent { ptr: ptr } }
    pub fn null() -> PaintEvent { PaintEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPaintEvent](http://docs.wxwidgets.org/3.0/classwx_paint_event.html) class.
pub trait PaintEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxPalette](http://docs.wxwidgets.org/3.0/classwx_palette.html) class.
pub struct Palette { ptr: *mut c_void }
impl PaletteMethods for Palette {}
impl GDIObjectMethods for Palette {}
impl ObjectMethods for Palette { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Palette {
    pub fn from(ptr: *mut c_void) -> Palette { Palette { ptr: ptr } }
    pub fn null() -> Palette { Palette::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Palette {
        unsafe { Palette::from(wxPalette_CreateDefault()) }
    }
    pub fn newRGB(n: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> Palette {
        unsafe { Palette::from(wxPalette_CreateRGB(n, red, green, blue)) }
    }
}

/// Methods of the wxWidgets' [wxPalette](http://docs.wxwidgets.org/3.0/classwx_palette.html) class.
pub trait PaletteMethods : GDIObjectMethods {
    fn assign<T: PaletteMethods>(&self, palette: &T) {
        unsafe { wxPalette_Assign(self.ptr(), palette.ptr()) }
    }
    fn getPixel(&self, red: uint8_t, green: uint8_t, blue: uint8_t) -> c_int {
        unsafe { wxPalette_GetPixel(self.ptr(), red, green, blue) }
    }
    fn getRGB(&self, pixel: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> c_int {
        unsafe { wxPalette_GetRGB(self.ptr(), pixel, red, green, blue) }
    }
    fn isEqual<T: PaletteMethods>(&self, palette: &T) -> c_int {
        unsafe { wxPalette_IsEqual(self.ptr(), palette.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPalette_IsOk(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPaletteChangedEvent](http://docs.wxwidgets.org/3.0/classwx_palette_changed_event.html) class.
pub struct PaletteChangedEvent { ptr: *mut c_void }
impl PaletteChangedEventMethods for PaletteChangedEvent {}
impl EventMethods for PaletteChangedEvent {}
impl ObjectMethods for PaletteChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PaletteChangedEvent {
    pub fn from(ptr: *mut c_void) -> PaletteChangedEvent { PaletteChangedEvent { ptr: ptr } }
    pub fn null() -> PaletteChangedEvent { PaletteChangedEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPaletteChangedEvent](http://docs.wxwidgets.org/3.0/classwx_palette_changed_event.html) class.
pub trait PaletteChangedEventMethods : EventMethods {
    fn getChangedWindow(&self) -> *mut c_void {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.ptr()) }
    }
    fn setChangedWindow<T: WindowMethods>(&self, win: &T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.ptr(), win.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPanel](http://docs.wxwidgets.org/3.0/classwx_panel.html) class.
pub struct Panel { ptr: *mut c_void }
impl PanelMethods for Panel {}
impl WindowMethods for Panel {}
impl EvtHandlerMethods for Panel {}
impl ObjectMethods for Panel { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Panel {
    pub fn from(ptr: *mut c_void) -> Panel { Panel { ptr: ptr } }
    pub fn null() -> Panel { Panel::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Panel {
        unsafe { Panel::from(wxPanel_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxPanel](http://docs.wxwidgets.org/3.0/classwx_panel.html) class.
pub trait PanelMethods : WindowMethods {
}

/// Wraps the wxWidgets' [wxPen](http://docs.wxwidgets.org/3.0/classwx_pen.html) class.
pub struct Pen { ptr: *mut c_void }
impl PenMethods for Pen {}
impl GDIObjectMethods for Pen {}
impl ObjectMethods for Pen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Pen {
    pub fn from(ptr: *mut c_void) -> Pen { Pen { ptr: ptr } }
    pub fn null() -> Pen { Pen::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Pen {
        unsafe { Pen::from(wxPen_CreateDefault()) }
    }
    pub fn newFromBitmap<T: BitmapMethods>(stipple: &T, width: c_int) -> Pen {
        unsafe { Pen::from(wxPen_CreateFromBitmap(stipple.ptr(), width)) }
    }
    pub fn newFromColour<T: ColourMethods>(col: &T, width: c_int, style: c_int) -> Pen {
        unsafe { Pen::from(wxPen_CreateFromColour(col.ptr(), width, style)) }
    }
    pub fn newFromStock(id: c_int) -> Pen {
        unsafe { Pen::from(wxPen_CreateFromStock(id)) }
    }
}

/// Methods of the wxWidgets' [wxPen](http://docs.wxwidgets.org/3.0/classwx_pen.html) class.
pub trait PenMethods : GDIObjectMethods {
    fn assign<T: PenMethods>(&self, pen: &T) {
        unsafe { wxPen_Assign(self.ptr(), pen.ptr()) }
    }
    fn getCap(&self) -> c_int {
        unsafe { wxPen_GetCap(self.ptr()) }
    }
    fn getColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxPen_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getDashes(&self, ptr: *mut c_void) -> c_int {
        unsafe { wxPen_GetDashes(self.ptr(), ptr) }
    }
    fn getJoin(&self) -> c_int {
        unsafe { wxPen_GetJoin(self.ptr()) }
    }
    fn getStipple<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxPen_GetStipple(self.ptr(), _ref.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxPen_GetStyle(self.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxPen_GetWidth(self.ptr()) }
    }
    fn isEqual<T: PenMethods>(&self, pen: &T) -> c_int {
        unsafe { wxPen_IsEqual(self.ptr(), pen.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPen_IsOk(self.ptr()) }
    }
    fn setCap(&self, cap: c_int) {
        unsafe { wxPen_SetCap(self.ptr(), cap) }
    }
    fn setColour<T: ColourMethods>(&self, col: &T) {
        unsafe { wxPen_SetColour(self.ptr(), col.ptr()) }
    }
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxPen_SetColourSingle(self.ptr(), r, g, b) }
    }
    fn setDashes(&self, nb_dashes: c_int, dash: *mut c_void) {
        unsafe { wxPen_SetDashes(self.ptr(), nb_dashes, dash) }
    }
    fn setJoin(&self, join: c_int) {
        unsafe { wxPen_SetJoin(self.ptr(), join) }
    }
    fn setStipple<T: BitmapMethods>(&self, stipple: &T) {
        unsafe { wxPen_SetStipple(self.ptr(), stipple.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxPen_SetStyle(self.ptr(), style) }
    }
    fn setWidth(&self, width: c_int) {
        unsafe { wxPen_SetWidth(self.ptr(), width) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxPen_IsStatic(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPenList](http://docs.wxwidgets.org/3.0/classwx_pen_list.html) class.
pub struct PenList { ptr: *mut c_void }
impl PenListMethods for PenList {}
impl ListMethods for PenList {}
impl ObjectMethods for PenList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PenList {
    pub fn from(ptr: *mut c_void) -> PenList { PenList { ptr: ptr } }
    pub fn null() -> PenList { PenList::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPenList](http://docs.wxwidgets.org/3.0/classwx_pen_list.html) class.
pub trait PenListMethods : ListMethods {
}

/// Wraps the wxWidgets' [wxPoint](http://docs.wxwidgets.org/3.0/classwx_point.html) class.
pub struct Point { ptr: *mut c_void }
impl PointMethods for Point { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Point {
    pub fn from(ptr: *mut c_void) -> Point { Point { ptr: ptr } }
    pub fn null() -> Point { Point::from(0 as *mut c_void) }
    
    pub fn new(xx: c_int, yy: c_int) -> Point {
        unsafe { Point::from(wxPoint_Create(xx, yy)) }
    }
}

/// Methods of the wxWidgets' [wxPoint](http://docs.wxwidgets.org/3.0/classwx_point.html) class.
pub trait PointMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn getX(&self) -> c_int {
        unsafe { wxPoint_GetX(self.ptr()) }
    }
    fn getY(&self) -> c_int {
        unsafe { wxPoint_GetY(self.ptr()) }
    }
    fn setX(&self, w: c_int) {
        unsafe { wxPoint_SetX(self.ptr(), w) }
    }
    fn setY(&self, h: c_int) {
        unsafe { wxPoint_SetY(self.ptr(), h) }
    }
}

/// Wraps the wxWidgets' [wxPopupTransientWindow](http://docs.wxwidgets.org/3.0/classwx_popup_transient_window.html) class.
pub struct PopupTransientWindow { ptr: *mut c_void }
impl PopupTransientWindowMethods for PopupTransientWindow {}
impl PopupWindowMethods for PopupTransientWindow {}
impl WindowMethods for PopupTransientWindow {}
impl EvtHandlerMethods for PopupTransientWindow {}
impl ObjectMethods for PopupTransientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PopupTransientWindow {
    pub fn from(ptr: *mut c_void) -> PopupTransientWindow { PopupTransientWindow { ptr: ptr } }
    pub fn null() -> PopupTransientWindow { PopupTransientWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPopupTransientWindow](http://docs.wxwidgets.org/3.0/classwx_popup_transient_window.html) class.
pub trait PopupTransientWindowMethods : PopupWindowMethods {
}

/// Wraps the wxWidgets' [wxPopupWindow](http://docs.wxwidgets.org/3.0/classwx_popup_window.html) class.
pub struct PopupWindow { ptr: *mut c_void }
impl PopupWindowMethods for PopupWindow {}
impl WindowMethods for PopupWindow {}
impl EvtHandlerMethods for PopupWindow {}
impl ObjectMethods for PopupWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PopupWindow {
    pub fn from(ptr: *mut c_void) -> PopupWindow { PopupWindow { ptr: ptr } }
    pub fn null() -> PopupWindow { PopupWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPopupWindow](http://docs.wxwidgets.org/3.0/classwx_popup_window.html) class.
pub trait PopupWindowMethods : WindowMethods {
}

/// Wraps the wxWidgets' [wxPostScriptDC](http://docs.wxwidgets.org/3.0/classwx_post_script_dc.html) class.
pub struct PostScriptDC { ptr: *mut c_void }
impl PostScriptDCMethods for PostScriptDC {}
impl DCMethods for PostScriptDC {}
impl ObjectMethods for PostScriptDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PostScriptDC {
    pub fn from(ptr: *mut c_void) -> PostScriptDC { PostScriptDC { ptr: ptr } }
    pub fn null() -> PostScriptDC { PostScriptDC::from(0 as *mut c_void) }
    
    pub fn new<T: PrintDataMethods>(data: &T) -> PostScriptDC {
        unsafe { PostScriptDC::from(wxPostScriptDC_Create(data.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPostScriptDC](http://docs.wxwidgets.org/3.0/classwx_post_script_dc.html) class.
pub trait PostScriptDCMethods : DCMethods {
    fn setResolution(&self, ppi: c_int) {
        unsafe { wxPostScriptDC_SetResolution(self.ptr(), ppi) }
    }
    fn getResolution(&self) -> c_int {
        unsafe { wxPostScriptDC_GetResolution(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPreviewCanvas](http://docs.wxwidgets.org/3.0/classwx_preview_canvas.html) class.
pub struct PreviewCanvas { ptr: *mut c_void }
impl PreviewCanvasMethods for PreviewCanvas {}
impl ScrolledWindowMethods for PreviewCanvas {}
impl PanelMethods for PreviewCanvas {}
impl WindowMethods for PreviewCanvas {}
impl EvtHandlerMethods for PreviewCanvas {}
impl ObjectMethods for PreviewCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PreviewCanvas {
    pub fn from(ptr: *mut c_void) -> PreviewCanvas { PreviewCanvas { ptr: ptr } }
    pub fn null() -> PreviewCanvas { PreviewCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: PrintPreviewMethods, U: WindowMethods>(preview: &T, parent: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> PreviewCanvas {
        unsafe { PreviewCanvas::from(wxPreviewCanvas_Create(preview.ptr(), parent.ptr(), x, y, w, h, style)) }
    }
}

/// Methods of the wxWidgets' [wxPreviewCanvas](http://docs.wxwidgets.org/3.0/classwx_preview_canvas.html) class.
pub trait PreviewCanvasMethods : ScrolledWindowMethods {
}

/// Wraps the wxWidgets' [wxPreviewControlBar](http://docs.wxwidgets.org/3.0/classwx_preview_control_bar.html) class.
/// Rather use the wxRust-specific [RustPreviewControlBar](struct.RustPreviewControlBar.html) class.
pub struct PreviewControlBar { ptr: *mut c_void }
impl PreviewControlBarMethods for PreviewControlBar {}
impl PanelMethods for PreviewControlBar {}
impl WindowMethods for PreviewControlBar {}
impl EvtHandlerMethods for PreviewControlBar {}
impl ObjectMethods for PreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PreviewControlBar {
    pub fn from(ptr: *mut c_void) -> PreviewControlBar { PreviewControlBar { ptr: ptr } }
    pub fn null() -> PreviewControlBar { PreviewControlBar::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPreviewControlBar](http://docs.wxwidgets.org/3.0/classwx_preview_control_bar.html) class.
pub trait PreviewControlBarMethods : PanelMethods {
}

/// Wraps the wxWidgets' [wxPreviewFrame](http://docs.wxwidgets.org/3.0/classwx_preview_frame.html) class.
/// Rather use the wxRust-specific [RustPreviewFrame](struct.RustPreviewFrame.html) class.
pub struct PreviewFrame { ptr: *mut c_void }
impl PreviewFrameMethods for PreviewFrame {}
impl FrameMethods for PreviewFrame {}
impl TopLevelWindowMethods for PreviewFrame {}
impl WindowMethods for PreviewFrame {}
impl EvtHandlerMethods for PreviewFrame {}
impl ObjectMethods for PreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PreviewFrame {
    pub fn from(ptr: *mut c_void) -> PreviewFrame { PreviewFrame { ptr: ptr } }
    pub fn null() -> PreviewFrame { PreviewFrame::from(0 as *mut c_void) }
    
    pub fn new<T: PrintPreviewMethods, U: FrameMethods>(preview: &T, parent: &U, title: &str, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: &str) -> PreviewFrame {
        let title = strToString(title);
        let name = strToString(name);
        unsafe { PreviewFrame::from(wxPreviewFrame_Create(preview.ptr(), parent.ptr(), title.ptr(), x, y, width, height, style, name.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPreviewFrame](http://docs.wxwidgets.org/3.0/classwx_preview_frame.html) class.
pub trait PreviewFrameMethods : FrameMethods {
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPrintData](http://docs.wxwidgets.org/3.0/classwx_print_data.html) class.
pub struct PrintData { ptr: *mut c_void }
impl PrintDataMethods for PrintData {}
impl ObjectMethods for PrintData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintData {
    pub fn from(ptr: *mut c_void) -> PrintData { PrintData { ptr: ptr } }
    pub fn null() -> PrintData { PrintData::from(0 as *mut c_void) }
    
    pub fn new() -> PrintData {
        unsafe { PrintData::from(wxPrintData_Create()) }
    }
}

/// Methods of the wxWidgets' [wxPrintData](http://docs.wxwidgets.org/3.0/classwx_print_data.html) class.
pub trait PrintDataMethods : ObjectMethods {
    fn assign<T: PrintDataMethods>(&self, data: &T) {
        unsafe { wxPrintData_Assign(self.ptr(), data.ptr()) }
    }
    fn getCollate(&self) -> c_int {
        unsafe { wxPrintData_GetCollate(self.ptr()) }
    }
    fn getColour(&self) -> c_int {
        unsafe { wxPrintData_GetColour(self.ptr()) }
    }
    fn getDuplex(&self) -> c_int {
        unsafe { wxPrintData_GetDuplex(self.ptr()) }
    }
    fn getFilename(&self) -> ~str {
        unsafe { String::from(wxPrintData_GetFilename(self.ptr())).to_str() }
    }
    fn getFontMetricPath(&self) -> ~str {
        unsafe { String::from(wxPrintData_GetFontMetricPath(self.ptr())).to_str() }
    }
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintData_GetNoCopies(self.ptr()) }
    }
    fn getOrientation(&self) -> c_int {
        unsafe { wxPrintData_GetOrientation(self.ptr()) }
    }
    fn getPaperId(&self) -> c_int {
        unsafe { wxPrintData_GetPaperId(self.ptr()) }
    }
    fn getPaperSize(&self) -> Size {
        unsafe { Size::from(wxPrintData_GetPaperSize(self.ptr())) }
    }
    fn getPreviewCommand(&self) -> ~str {
        unsafe { String::from(wxPrintData_GetPreviewCommand(self.ptr())).to_str() }
    }
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.ptr()) }
    }
    fn getPrinterCommand(&self) -> ~str {
        unsafe { String::from(wxPrintData_GetPrinterCommand(self.ptr())).to_str() }
    }
    fn getPrinterName(&self) -> ~str {
        unsafe { String::from(wxPrintData_GetPrinterName(self.ptr())).to_str() }
    }
    fn getPrinterOptions(&self) -> ~str {
        unsafe { String::from(wxPrintData_GetPrinterOptions(self.ptr())).to_str() }
    }
    fn getPrinterScaleX(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleX(self.ptr()) }
    }
    fn getPrinterScaleY(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleY(self.ptr()) }
    }
    fn getPrinterTranslateX(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateX(self.ptr()) }
    }
    fn getPrinterTranslateY(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateY(self.ptr()) }
    }
    fn getQuality(&self) -> c_int {
        unsafe { wxPrintData_GetQuality(self.ptr()) }
    }
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintData_SetCollate(self.ptr(), flag) }
    }
    fn setColour(&self, colour: c_int) {
        unsafe { wxPrintData_SetColour(self.ptr(), colour) }
    }
    fn setDuplex(&self, duplex: c_int) {
        unsafe { wxPrintData_SetDuplex(self.ptr(), duplex) }
    }
    fn setFilename(&self, filename: &str) {
        let filename = strToString(filename);
        unsafe { wxPrintData_SetFilename(self.ptr(), filename.ptr()) }
    }
    fn setFontMetricPath(&self, path: &str) {
        let path = strToString(path);
        unsafe { wxPrintData_SetFontMetricPath(self.ptr(), path.ptr()) }
    }
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintData_SetNoCopies(self.ptr(), v) }
    }
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxPrintData_SetOrientation(self.ptr(), orient) }
    }
    fn setPaperId(&self, sizeId: c_int) {
        unsafe { wxPrintData_SetPaperId(self.ptr(), sizeId) }
    }
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPrintData_SetPaperSize(self.ptr(), w, h) }
    }
    fn setPreviewCommand<T: CommandMethods>(&self, command: &T) {
        unsafe { wxPrintData_SetPreviewCommand(self.ptr(), command.ptr()) }
    }
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.ptr(), printMode) }
    }
    fn setPrinterCommand<T: CommandMethods>(&self, command: &T) {
        unsafe { wxPrintData_SetPrinterCommand(self.ptr(), command.ptr()) }
    }
    fn setPrinterName(&self, name: &str) {
        let name = strToString(name);
        unsafe { wxPrintData_SetPrinterName(self.ptr(), name.ptr()) }
    }
    fn setPrinterOptions(&self, options: &str) {
        let options = strToString(options);
        unsafe { wxPrintData_SetPrinterOptions(self.ptr(), options.ptr()) }
    }
    fn setPrinterScaleX(&self, x: c_double) {
        unsafe { wxPrintData_SetPrinterScaleX(self.ptr(), x) }
    }
    fn setPrinterScaleY(&self, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaleY(self.ptr(), y) }
    }
    fn setPrinterScaling(&self, x: c_double, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaling(self.ptr(), x, y) }
    }
    fn setPrinterTranslateX(&self, x: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateX(self.ptr(), x) }
    }
    fn setPrinterTranslateY(&self, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateY(self.ptr(), y) }
    }
    fn setPrinterTranslation(&self, x: c_int, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslation(self.ptr(), x, y) }
    }
    fn setQuality(&self, quality: c_int) {
        unsafe { wxPrintData_SetQuality(self.ptr(), quality) }
    }
}

/// Wraps the wxWidgets' [wxPostScriptPrintNativeData](http://docs.wxwidgets.org/3.0/classwx_post_script_print_native_data.html) class.
pub struct PostScriptPrintNativeData { ptr: *mut c_void }
impl PostScriptPrintNativeDataMethods for PostScriptPrintNativeData {}
impl ObjectMethods for PostScriptPrintNativeData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PostScriptPrintNativeData {
    pub fn from(ptr: *mut c_void) -> PostScriptPrintNativeData { PostScriptPrintNativeData { ptr: ptr } }
    pub fn null() -> PostScriptPrintNativeData { PostScriptPrintNativeData::from(0 as *mut c_void) }
    
    pub fn new() -> PostScriptPrintNativeData {
        unsafe { PostScriptPrintNativeData::from(wxPostScriptPrintNativeData_Create()) }
    }
}

/// Methods of the wxWidgets' [wxPostScriptPrintNativeData](http://docs.wxwidgets.org/3.0/classwx_post_script_print_native_data.html) class.
pub trait PostScriptPrintNativeDataMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxPrintDialog](http://docs.wxwidgets.org/3.0/classwx_print_dialog.html) class.
pub struct PrintDialog { ptr: *mut c_void }
impl PrintDialogMethods for PrintDialog {}
impl DialogMethods for PrintDialog {}
impl TopLevelWindowMethods for PrintDialog {}
impl WindowMethods for PrintDialog {}
impl EvtHandlerMethods for PrintDialog {}
impl ObjectMethods for PrintDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintDialog {
    pub fn from(ptr: *mut c_void) -> PrintDialog { PrintDialog { ptr: ptr } }
    pub fn null() -> PrintDialog { PrintDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: PrintDialogDataMethods>(parent: &T, data: &U) -> PrintDialog {
        unsafe { PrintDialog::from(wxPrintDialog_Create(parent.ptr(), data.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPrintDialog](http://docs.wxwidgets.org/3.0/classwx_print_dialog.html) class.
pub trait PrintDialogMethods : DialogMethods {
    fn getPrintDC(&self) -> DC {
        unsafe { DC::from(wxPrintDialog_GetPrintDC(self.ptr())) }
    }
    fn getPrintData<T: PrintDataMethods>(&self, _ref: &T) {
        unsafe { wxPrintDialog_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintDialogData(&self) -> PrintDialogData {
        unsafe { PrintDialogData::from(wxPrintDialog_GetPrintDialogData(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxPrintDialogData](http://docs.wxwidgets.org/3.0/classwx_print_dialog_data.html) class.
pub struct PrintDialogData { ptr: *mut c_void }
impl PrintDialogDataMethods for PrintDialogData {}
impl ObjectMethods for PrintDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintDialogData {
    pub fn from(ptr: *mut c_void) -> PrintDialogData { PrintDialogData { ptr: ptr } }
    pub fn null() -> PrintDialogData { PrintDialogData::from(0 as *mut c_void) }
    
    pub fn newDefault() -> PrintDialogData {
        unsafe { PrintDialogData::from(wxPrintDialogData_CreateDefault()) }
    }
    pub fn newFromData<T: PrintDataMethods>(printData: &T) -> PrintDialogData {
        unsafe { PrintDialogData::from(wxPrintDialogData_CreateFromData(printData.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPrintDialogData](http://docs.wxwidgets.org/3.0/classwx_print_dialog_data.html) class.
pub trait PrintDialogDataMethods : ObjectMethods {
    fn assign<T: PrintDialogDataMethods>(&self, data: &T) {
        unsafe { wxPrintDialogData_Assign(self.ptr(), data.ptr()) }
    }
    fn assignData<T: PrintDataMethods>(&self, data: &T) {
        unsafe { wxPrintDialogData_AssignData(self.ptr(), data.ptr()) }
    }
    fn enableHelp(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnableHelp(self.ptr(), flag) }
    }
    fn enablePageNumbers(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnablePageNumbers(self.ptr(), flag) }
    }
    fn enablePrintToFile(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnablePrintToFile(self.ptr(), flag) }
    }
    fn enableSelection(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnableSelection(self.ptr(), flag) }
    }
    fn getAllPages(&self) -> c_int {
        unsafe { wxPrintDialogData_GetAllPages(self.ptr()) }
    }
    fn getCollate(&self) -> c_int {
        unsafe { wxPrintDialogData_GetCollate(self.ptr()) }
    }
    fn getEnableHelp(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnableHelp(self.ptr()) }
    }
    fn getEnablePageNumbers(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnablePageNumbers(self.ptr()) }
    }
    fn getEnablePrintToFile(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnablePrintToFile(self.ptr()) }
    }
    fn getEnableSelection(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnableSelection(self.ptr()) }
    }
    fn getFromPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetFromPage(self.ptr()) }
    }
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMaxPage(self.ptr()) }
    }
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMinPage(self.ptr()) }
    }
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintDialogData_GetNoCopies(self.ptr()) }
    }
    fn getPrintData<T: PrintDataMethods>(&self, _ref: &T) {
        unsafe { wxPrintDialogData_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintToFile(&self) -> c_int {
        unsafe { wxPrintDialogData_GetPrintToFile(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxPrintDialogData_GetSelection(self.ptr()) }
    }
    fn getToPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetToPage(self.ptr()) }
    }
    fn setAllPages(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetAllPages(self.ptr(), flag) }
    }
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetCollate(self.ptr(), flag) }
    }
    fn setFromPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetFromPage(self.ptr(), v) }
    }
    fn setMaxPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMaxPage(self.ptr(), v) }
    }
    fn setMinPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMinPage(self.ptr(), v) }
    }
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetNoCopies(self.ptr(), v) }
    }
    fn setPrintData<T: PrintDataMethods>(&self, printData: &T) {
        unsafe { wxPrintDialogData_SetPrintData(self.ptr(), printData.ptr()) }
    }
    fn setPrintToFile(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetPrintToFile(self.ptr(), flag) }
    }
    fn setSelection(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetSelection(self.ptr(), flag) }
    }
    fn setToPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetToPage(self.ptr(), v) }
    }
}

/// Wraps the wxWidgets' [wxPrintPreview](http://docs.wxwidgets.org/3.0/classwx_print_preview.html) class.
pub struct PrintPreview { ptr: *mut c_void }
impl PrintPreviewMethods for PrintPreview {}
impl ObjectMethods for PrintPreview { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintPreview {
    pub fn from(ptr: *mut c_void) -> PrintPreview { PrintPreview { ptr: ptr } }
    pub fn null() -> PrintPreview { PrintPreview::from(0 as *mut c_void) }
    
    pub fn newFromData<T: PrintoutMethods, U: PrintoutMethods, V: PrintDataMethods>(printout: &T, printoutForPrinting: &U, data: &V) -> PrintPreview {
        unsafe { PrintPreview::from(wxPrintPreview_CreateFromData(printout.ptr(), printoutForPrinting.ptr(), data.ptr())) }
    }
    pub fn newFromDialogData<T: PrintoutMethods, U: PrintoutMethods, V: PrintDialogDataMethods>(printout: &T, printoutForPrinting: &U, data: &V) -> PrintPreview {
        unsafe { PrintPreview::from(wxPrintPreview_CreateFromDialogData(printout.ptr(), printoutForPrinting.ptr(), data.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPrintPreview](http://docs.wxwidgets.org/3.0/classwx_print_preview.html) class.
pub trait PrintPreviewMethods : ObjectMethods {
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.ptr()) }
    }
    fn drawBlankPage<T: PreviewCanvasMethods, U: DCMethods>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_DrawBlankPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    fn getCanvas(&self) -> PreviewCanvas {
        unsafe { PreviewCanvas::from(wxPrintPreview_GetCanvas(self.ptr())) }
    }
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.ptr()) }
    }
    fn getFrame(&self) -> Frame {
        unsafe { Frame::from(wxPrintPreview_GetFrame(self.ptr())) }
    }
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMaxPage(self.ptr()) }
    }
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMinPage(self.ptr()) }
    }
    fn getPrintDialogData<T: PrintDialogDataMethods>(&self, _ref: &T) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintout(&self) -> Printout {
        unsafe { Printout::from(wxPrintPreview_GetPrintout(self.ptr())) }
    }
    fn getPrintoutForPrinting(&self) -> Printout {
        unsafe { Printout::from(wxPrintPreview_GetPrintoutForPrinting(self.ptr())) }
    }
    fn getZoom(&self) -> c_int {
        unsafe { wxPrintPreview_GetZoom(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPrintPreview_IsOk(self.ptr()) }
    }
    fn paintPage<T: PrintPreviewMethods, U: DCMethods>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_PaintPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    fn print(&self, interactive: c_int) -> c_int {
        unsafe { wxPrintPreview_Print(self.ptr(), interactive) }
    }
    fn renderPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_RenderPage(self.ptr(), pageNum) }
    }
    fn setCanvas<T: PreviewCanvasMethods>(&self, canvas: &T) {
        unsafe { wxPrintPreview_SetCanvas(self.ptr(), canvas.ptr()) }
    }
    fn setCurrentPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_SetCurrentPage(self.ptr(), pageNum) }
    }
    fn setFrame<T: FrameMethods>(&self, frame: &T) {
        unsafe { wxPrintPreview_SetFrame(self.ptr(), frame.ptr()) }
    }
    fn setOk(&self, ok: c_int) {
        unsafe { wxPrintPreview_SetOk(self.ptr(), ok) }
    }
    fn setPrintout<T: PrintoutMethods>(&self, printout: &T) {
        unsafe { wxPrintPreview_SetPrintout(self.ptr(), printout.ptr()) }
    }
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.ptr(), percent) }
    }
}

/// Wraps the wxWidgets' [wxPrinter](http://docs.wxwidgets.org/3.0/classwx_printer.html) class.
pub struct Printer { ptr: *mut c_void }
impl PrinterMethods for Printer {}
impl ObjectMethods for Printer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Printer {
    pub fn from(ptr: *mut c_void) -> Printer { Printer { ptr: ptr } }
    pub fn null() -> Printer { Printer::from(0 as *mut c_void) }
    
    pub fn new<T: PrintDialogDataMethods>(data: &T) -> Printer {
        unsafe { Printer::from(wxPrinter_Create(data.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPrinter](http://docs.wxwidgets.org/3.0/classwx_printer.html) class.
pub trait PrinterMethods : ObjectMethods {
    fn newAbortWindow<T: WindowMethods, U: PrintoutMethods>(&self, parent: &T, printout: &U) -> Window {
        unsafe { Window::from(wxPrinter_CreateAbortWindow(self.ptr(), parent.ptr(), printout.ptr())) }
    }
    fn getAbort(&self) -> c_int {
        unsafe { wxPrinter_GetAbort(self.ptr()) }
    }
    fn getLastError(&self) -> c_int {
        unsafe { wxPrinter_GetLastError(self.ptr()) }
    }
    fn getPrintDialogData<T: PrintDialogDataMethods>(&self, _ref: &T) {
        unsafe { wxPrinter_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    fn print<T: WindowMethods, U: PrintoutMethods>(&self, parent: &T, printout: &U, prompt: c_int) -> c_int {
        unsafe { wxPrinter_Print(self.ptr(), parent.ptr(), printout.ptr(), prompt) }
    }
    fn printDialog<T: WindowMethods>(&self, parent: &T) -> DC {
        unsafe { DC::from(wxPrinter_PrintDialog(self.ptr(), parent.ptr())) }
    }
    fn reportError<T: WindowMethods, U: PrintoutMethods>(&self, parent: &T, printout: &U, message: &str) {
        let message = strToString(message);
        unsafe { wxPrinter_ReportError(self.ptr(), parent.ptr(), printout.ptr(), message.ptr()) }
    }
    fn setup<T: WindowMethods>(&self, parent: &T) -> c_int {
        unsafe { wxPrinter_Setup(self.ptr(), parent.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxPrinterDC](http://docs.wxwidgets.org/3.0/classwx_printer_dc.html) class.
pub struct PrinterDC { ptr: *mut c_void }
impl PrinterDCMethods for PrinterDC {}
impl DCMethods for PrinterDC {}
impl ObjectMethods for PrinterDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrinterDC {
    pub fn from(ptr: *mut c_void) -> PrinterDC { PrinterDC { ptr: ptr } }
    pub fn null() -> PrinterDC { PrinterDC::from(0 as *mut c_void) }
    
    pub fn new<T: PrintDataMethods>(data: &T) -> PrinterDC {
        unsafe { PrinterDC::from(wxPrinterDC_Create(data.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxPrinterDC](http://docs.wxwidgets.org/3.0/classwx_printer_dc.html) class.
pub trait PrinterDCMethods : DCMethods {
    fn getPaperRect(&self) -> Rect {
        unsafe { Rect::from(wxPrinterDC_GetPaperRect(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxPrintout](http://docs.wxwidgets.org/3.0/classwx_printout.html) class.
/// Rather use the wxRust-specific [CPrintout](struct.CPrintout.html) class.
pub struct Printout { ptr: *mut c_void }
impl PrintoutMethods for Printout {}
impl ObjectMethods for Printout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Printout {
    pub fn from(ptr: *mut c_void) -> Printout { Printout { ptr: ptr } }
    pub fn null() -> Printout { Printout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPrintout](http://docs.wxwidgets.org/3.0/classwx_printout.html) class.
pub trait PrintoutMethods : ObjectMethods {
    fn getDC(&self) -> DC {
        unsafe { DC::from(wxPrintout_GetDC(self.ptr())) }
    }
    fn getPPIPrinter(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxPrintout_GetPPIPrinter(self.ptr(), _x, _y) }
    }
    fn getPPIScreen(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxPrintout_GetPPIScreen(self.ptr(), _x, _y) }
    }
    fn getPageSizeMM(&self, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxPrintout_GetPageSizeMM(self.ptr(), _w, _h) }
    }
    fn getPageSizePixels(&self, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxPrintout_GetPageSizePixels(self.ptr(), _w, _h) }
    }
    fn getTitle(&self) -> ~str {
        unsafe { String::from(wxPrintout_GetTitle(self.ptr())).to_str() }
    }
    fn isPreview(&self) -> c_int {
        unsafe { wxPrintout_IsPreview(self.ptr()) }
    }
    fn setDC<T: DCMethods>(&self, dc: &T) {
        unsafe { wxPrintout_SetDC(self.ptr(), dc.ptr()) }
    }
    fn setPPIPrinter(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIPrinter(self.ptr(), x, y) }
    }
    fn setPPIScreen(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIScreen(self.ptr(), x, y) }
    }
    fn setPageSizeMM(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizeMM(self.ptr(), w, h) }
    }
    fn setPageSizePixels(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizePixels(self.ptr(), w, h) }
    }
}

/// Wraps the wxWidgets' [wxPrivateDropTarget](http://docs.wxwidgets.org/3.0/classwx_private_drop_target.html) class.
pub struct PrivateDropTarget { ptr: *mut c_void }
impl PrivateDropTargetMethods for PrivateDropTarget {}
impl DropTargetMethods for PrivateDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrivateDropTarget {
    pub fn from(ptr: *mut c_void) -> PrivateDropTarget { PrivateDropTarget { ptr: ptr } }
    pub fn null() -> PrivateDropTarget { PrivateDropTarget::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPrivateDropTarget](http://docs.wxwidgets.org/3.0/classwx_private_drop_target.html) class.
pub trait PrivateDropTargetMethods : DropTargetMethods {
}

/// Wraps the wxWidgets' [wxProcess](http://docs.wxwidgets.org/3.0/classwx_process.html) class.
pub struct Process { ptr: *mut c_void }
impl ProcessMethods for Process {}
impl EvtHandlerMethods for Process {}
impl ObjectMethods for Process { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Process {
    pub fn from(ptr: *mut c_void) -> Process { Process { ptr: ptr } }
    pub fn null() -> Process { Process::from(0 as *mut c_void) }
    
    pub fn newDefault<T: WindowMethods>(_prt: &T, _id: c_int) -> Process {
        unsafe { Process::from(wxProcess_CreateDefault(_prt.ptr(), _id)) }
    }
    pub fn newRedirect<T: WindowMethods>(_prt: &T, _rdr: c_int) -> Process {
        unsafe { Process::from(wxProcess_CreateRedirect(_prt.ptr(), _rdr)) }
    }
    pub fn open(cmd: &str, flags: c_int) -> Process {
        let cmd = strToString(cmd);
        unsafe { Process::from(wxProcess_Open(cmd.ptr(), flags)) }
    }
}

/// Methods of the wxWidgets' [wxProcess](http://docs.wxwidgets.org/3.0/classwx_process.html) class.
pub trait ProcessMethods : EvtHandlerMethods {
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.ptr()) }
    }
    fn detach(&self) {
        unsafe { wxProcess_Detach(self.ptr()) }
    }
    fn getErrorStream(&self) -> InputStream {
        unsafe { InputStream::from(wxProcess_GetErrorStream(self.ptr())) }
    }
    fn getInputStream(&self) -> InputStream {
        unsafe { InputStream::from(wxProcess_GetInputStream(self.ptr())) }
    }
    fn getOutputStream(&self) -> OutputStream {
        unsafe { OutputStream::from(wxProcess_GetOutputStream(self.ptr())) }
    }
    fn isRedirected(&self) -> c_int {
        unsafe { wxProcess_IsRedirected(self.ptr()) }
    }
    fn redirect(&self) {
        unsafe { wxProcess_Redirect(self.ptr()) }
    }
    fn isErrorAvailable(&self) -> c_int {
        unsafe { wxProcess_IsErrorAvailable(self.ptr()) }
    }
    fn isInputAvailable(&self) -> c_int {
        unsafe { wxProcess_IsInputAvailable(self.ptr()) }
    }
    fn isInputOpened(&self) -> c_int {
        unsafe { wxProcess_IsInputOpened(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxProcessEvent](http://docs.wxwidgets.org/3.0/classwx_process_event.html) class.
pub struct ProcessEvent { ptr: *mut c_void }
impl ProcessEventMethods for ProcessEvent {}
impl EventMethods for ProcessEvent {}
impl ObjectMethods for ProcessEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ProcessEvent {
    pub fn from(ptr: *mut c_void) -> ProcessEvent { ProcessEvent { ptr: ptr } }
    pub fn null() -> ProcessEvent { ProcessEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxProcessEvent](http://docs.wxwidgets.org/3.0/classwx_process_event.html) class.
pub trait ProcessEventMethods : EventMethods {
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.ptr()) }
    }
    fn getPid(&self) -> c_int {
        unsafe { wxProcessEvent_GetPid(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxProgressDialog](http://docs.wxwidgets.org/3.0/classwx_progress_dialog.html) class.
pub struct ProgressDialog { ptr: *mut c_void }
impl ProgressDialogMethods for ProgressDialog {}
impl FrameMethods for ProgressDialog {}
impl TopLevelWindowMethods for ProgressDialog {}
impl WindowMethods for ProgressDialog {}
impl EvtHandlerMethods for ProgressDialog {}
impl ObjectMethods for ProgressDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ProgressDialog {
    pub fn from(ptr: *mut c_void) -> ProgressDialog { ProgressDialog { ptr: ptr } }
    pub fn null() -> ProgressDialog { ProgressDialog::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(title: &str, message: &str, max: c_int, parent: &T, style: c_int) -> ProgressDialog {
        let title = strToString(title);
        let message = strToString(message);
        unsafe { ProgressDialog::from(wxProgressDialog_Create(title.ptr(), message.ptr(), max, parent.ptr(), style)) }
    }
}

/// Methods of the wxWidgets' [wxProgressDialog](http://docs.wxwidgets.org/3.0/classwx_progress_dialog.html) class.
pub trait ProgressDialogMethods : FrameMethods {
    fn update(&self, value: c_int) -> c_int {
        unsafe { wxProgressDialog_Update(self.ptr(), value) }
    }
    fn updateWithMessage(&self, value: c_int, message: &str) -> c_int {
        let message = strToString(message);
        unsafe { wxProgressDialog_UpdateWithMessage(self.ptr(), value, message.ptr()) }
    }
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxQuantize](http://docs.wxwidgets.org/3.0/classwx_quantize.html) class.
pub struct Quantize { ptr: *mut c_void }
impl QuantizeMethods for Quantize {}
impl ObjectMethods for Quantize { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Quantize {
    pub fn from(ptr: *mut c_void) -> Quantize { Quantize { ptr: ptr } }
    pub fn null() -> Quantize { Quantize::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxQuantize](http://docs.wxwidgets.org/3.0/classwx_quantize.html) class.
pub trait QuantizeMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxQueryNewPaletteEvent](http://docs.wxwidgets.org/3.0/classwx_query_new_palette_event.html) class.
pub struct QueryNewPaletteEvent { ptr: *mut c_void }
impl QueryNewPaletteEventMethods for QueryNewPaletteEvent {}
impl EventMethods for QueryNewPaletteEvent {}
impl ObjectMethods for QueryNewPaletteEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl QueryNewPaletteEvent {
    pub fn from(ptr: *mut c_void) -> QueryNewPaletteEvent { QueryNewPaletteEvent { ptr: ptr } }
    pub fn null() -> QueryNewPaletteEvent { QueryNewPaletteEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxQueryNewPaletteEvent](http://docs.wxwidgets.org/3.0/classwx_query_new_palette_event.html) class.
pub trait QueryNewPaletteEventMethods : EventMethods {
    fn getPaletteRealized(&self) -> c_int {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.ptr()) }
    }
    fn setPaletteRealized(&self, realized: c_int) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self.ptr(), realized) }
    }
}

/// Wraps the wxWidgets' [wxRadioBox](http://docs.wxwidgets.org/3.0/classwx_radio_box.html) class.
pub struct RadioBox { ptr: *mut c_void }
impl RadioBoxMethods for RadioBox {}
impl ControlMethods for RadioBox {}
impl WindowMethods for RadioBox {}
impl EvtHandlerMethods for RadioBox {}
impl ObjectMethods for RadioBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RadioBox {
    pub fn from(ptr: *mut c_void) -> RadioBox { RadioBox { ptr: ptr } }
    pub fn null() -> RadioBox { RadioBox::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *mut *mut c_char, _dim: c_int, _stl: c_int) -> RadioBox {
        let _txt = strToString(_txt);
        unsafe { RadioBox::from(wxRadioBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxRadioBox](http://docs.wxwidgets.org/3.0/classwx_radio_box.html) class.
pub trait RadioBoxMethods : ControlMethods {
    fn enableItem(&self, item: c_int, enable: c_int) {
        unsafe { wxRadioBox_EnableItem(self.ptr(), item, enable) }
    }
    fn findString(&self, s: &str) -> c_int {
        let s = strToString(s);
        unsafe { wxRadioBox_FindString(self.ptr(), s.ptr()) }
    }
    fn getItemLabel(&self, item: c_int) -> ~str {
        unsafe { String::from(wxRadioBox_GetItemLabel(self.ptr(), item)).to_str() }
    }
    fn getNumberOfRowsOrCols(&self) -> c_int {
        unsafe { wxRadioBox_GetNumberOfRowsOrCols(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxRadioBox_GetSelection(self.ptr()) }
    }
    fn getStringSelection(&self) -> ~str {
        unsafe { String::from(wxRadioBox_GetStringSelection(self.ptr())).to_str() }
    }
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.ptr()) }
    }
    fn setItemBitmap<T: BitmapMethods>(&self, item: c_int, bitmap: &T) {
        unsafe { wxRadioBox_SetItemBitmap(self.ptr(), item, bitmap.ptr()) }
    }
    fn setItemLabel(&self, item: c_int, label: &str) {
        let label = strToString(label);
        unsafe { wxRadioBox_SetItemLabel(self.ptr(), item, label.ptr()) }
    }
    fn setNumberOfRowsOrCols(&self, n: c_int) {
        unsafe { wxRadioBox_SetNumberOfRowsOrCols(self.ptr(), n) }
    }
    fn setSelection(&self, _n: c_int) {
        unsafe { wxRadioBox_SetSelection(self.ptr(), _n) }
    }
    fn setStringSelection(&self, s: &str) {
        let s = strToString(s);
        unsafe { wxRadioBox_SetStringSelection(self.ptr(), s.ptr()) }
    }
    fn showItem(&self, item: c_int, show: c_int) {
        unsafe { wxRadioBox_ShowItem(self.ptr(), item, show) }
    }
}

/// Wraps the wxWidgets' [wxRadioButton](http://docs.wxwidgets.org/3.0/classwx_radio_button.html) class.
pub struct RadioButton { ptr: *mut c_void }
impl RadioButtonMethods for RadioButton {}
impl ControlMethods for RadioButton {}
impl WindowMethods for RadioButton {}
impl EvtHandlerMethods for RadioButton {}
impl ObjectMethods for RadioButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RadioButton {
    pub fn from(ptr: *mut c_void) -> RadioButton { RadioButton { ptr: ptr } }
    pub fn null() -> RadioButton { RadioButton::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> RadioButton {
        let _txt = strToString(_txt);
        unsafe { RadioButton::from(wxRadioButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxRadioButton](http://docs.wxwidgets.org/3.0/classwx_radio_button.html) class.
pub trait RadioButtonMethods : ControlMethods {
    fn getValue(&self) -> c_int {
        unsafe { wxRadioButton_GetValue(self.ptr()) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxRadioButton_SetValue(self.ptr(), value) }
    }
}

/// Wraps the wxWidgets' [wxRealPoint](http://docs.wxwidgets.org/3.0/classwx_real_point.html) class.
pub struct RealPoint { ptr: *mut c_void }
impl RealPointMethods for RealPoint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RealPoint {
    pub fn from(ptr: *mut c_void) -> RealPoint { RealPoint { ptr: ptr } }
    pub fn null() -> RealPoint { RealPoint::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxRealPoint](http://docs.wxwidgets.org/3.0/classwx_real_point.html) class.
pub trait RealPointMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxRect](http://docs.wxwidgets.org/3.0/classwx_rect.html) class.
pub struct Rect { ptr: *mut c_void }
impl RectMethods for Rect { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Rect {
    pub fn from(ptr: *mut c_void) -> Rect { Rect { ptr: ptr } }
    pub fn null() -> Rect { Rect::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxRect](http://docs.wxwidgets.org/3.0/classwx_rect.html) class.
pub trait RectMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxRegion](http://docs.wxwidgets.org/3.0/classwx_region.html) class.
pub struct Region { ptr: *mut c_void }
impl RegionMethods for Region {}
impl GDIObjectMethods for Region {}
impl ObjectMethods for Region { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Region {
    pub fn from(ptr: *mut c_void) -> Region { Region { ptr: ptr } }
    pub fn null() -> Region { Region::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Region {
        unsafe { Region::from(wxRegion_CreateDefault()) }
    }
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> Region {
        unsafe { Region::from(wxRegion_CreateFromRect(x, y, w, h)) }
    }
}

/// Methods of the wxWidgets' [wxRegion](http://docs.wxwidgets.org/3.0/classwx_region.html) class.
pub trait RegionMethods : GDIObjectMethods {
    fn assign<T: RegionMethods>(&self, region: &T) {
        unsafe { wxRegion_Assign(self.ptr(), region.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxRegion_Clear(self.ptr()) }
    }
    fn containsPoint(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxRegion_ContainsPoint(self.ptr(), x, y) }
    }
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_ContainsRect(self.ptr(), x, y, width, height) }
    }
    fn isEmpty(&self) -> c_int {
        unsafe { wxRegion_IsEmpty(self.ptr()) }
    }
    fn getBox(&self, _x: *mut c_void, _y: *mut c_void, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxRegion_GetBox(self.ptr(), _x, _y, _w, _h) }
    }
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_IntersectRect(self.ptr(), x, y, width, height) }
    }
    fn intersectRegion<T: RegionMethods>(&self, region: &T) -> c_int {
        unsafe { wxRegion_IntersectRegion(self.ptr(), region.ptr()) }
    }
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_SubtractRect(self.ptr(), x, y, width, height) }
    }
    fn subtractRegion<T: RegionMethods>(&self, region: &T) -> c_int {
        unsafe { wxRegion_SubtractRegion(self.ptr(), region.ptr()) }
    }
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_UnionRect(self.ptr(), x, y, width, height) }
    }
    fn unionRegion<T: RegionMethods>(&self, region: &T) -> c_int {
        unsafe { wxRegion_UnionRegion(self.ptr(), region.ptr()) }
    }
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_XorRect(self.ptr(), x, y, width, height) }
    }
    fn xorRegion<T: RegionMethods>(&self, region: &T) -> c_int {
        unsafe { wxRegion_XorRegion(self.ptr(), region.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxRegionIterator](http://docs.wxwidgets.org/3.0/classwx_region_iterator.html) class.
pub struct RegionIterator { ptr: *mut c_void }
impl RegionIteratorMethods for RegionIterator {}
impl ObjectMethods for RegionIterator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RegionIterator {
    pub fn from(ptr: *mut c_void) -> RegionIterator { RegionIterator { ptr: ptr } }
    pub fn null() -> RegionIterator { RegionIterator::from(0 as *mut c_void) }
    
    pub fn new() -> RegionIterator {
        unsafe { RegionIterator::from(wxRegionIterator_Create()) }
    }
    pub fn newFromRegion<T: RegionMethods>(region: &T) -> RegionIterator {
        unsafe { RegionIterator::from(wxRegionIterator_CreateFromRegion(region.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxRegionIterator](http://docs.wxwidgets.org/3.0/classwx_region_iterator.html) class.
pub trait RegionIteratorMethods : ObjectMethods {
    fn getHeight(&self) -> c_int {
        unsafe { wxRegionIterator_GetHeight(self.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxRegionIterator_GetWidth(self.ptr()) }
    }
    fn getX(&self) -> c_int {
        unsafe { wxRegionIterator_GetX(self.ptr()) }
    }
    fn getY(&self) -> c_int {
        unsafe { wxRegionIterator_GetY(self.ptr()) }
    }
    fn haveRects(&self) -> c_int {
        unsafe { wxRegionIterator_HaveRects(self.ptr()) }
    }
    fn next(&self) {
        unsafe { wxRegionIterator_Next(self.ptr()) }
    }
    fn reset(&self) {
        unsafe { wxRegionIterator_Reset(self.ptr()) }
    }
    fn resetToRegion<T: RegionMethods>(&self, region: &T) {
        unsafe { wxRegionIterator_ResetToRegion(self.ptr(), region.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxSVGFileDC](http://docs.wxwidgets.org/3.0/classwx_svgf_ile_dc.html) class.
pub struct SVGFileDC { ptr: *mut c_void }
impl SVGFileDCMethods for SVGFileDC {}
impl DCMethods for SVGFileDC {}
impl ObjectMethods for SVGFileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SVGFileDC {
    pub fn from(ptr: *mut c_void) -> SVGFileDC { SVGFileDC { ptr: ptr } }
    pub fn null() -> SVGFileDC { SVGFileDC::from(0 as *mut c_void) }
    
    pub fn new(fileName: &str) -> SVGFileDC {
        let fileName = strToString(fileName);
        unsafe { SVGFileDC::from(wxSVGFileDC_Create(fileName.ptr())) }
    }
    pub fn newWithSize(fileName: &str, w: c_int, h: c_int) -> SVGFileDC {
        let fileName = strToString(fileName);
        unsafe { SVGFileDC::from(wxSVGFileDC_CreateWithSize(fileName.ptr(), w, h)) }
    }
    pub fn newWithSizeAndResolution(fileName: &str, w: c_int, h: c_int, a_dpi: c_float) -> SVGFileDC {
        let fileName = strToString(fileName);
        unsafe { SVGFileDC::from(wxSVGFileDC_CreateWithSizeAndResolution(fileName.ptr(), w, h, a_dpi)) }
    }
}

/// Methods of the wxWidgets' [wxSVGFileDC](http://docs.wxwidgets.org/3.0/classwx_svgf_ile_dc.html) class.
pub trait SVGFileDCMethods : DCMethods {
}

/// Wraps the wxWidgets' [wxScreenDC](http://docs.wxwidgets.org/3.0/classwx_screen_dc.html) class.
pub struct ScreenDC { ptr: *mut c_void }
impl ScreenDCMethods for ScreenDC {}
impl DCMethods for ScreenDC {}
impl ObjectMethods for ScreenDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScreenDC {
    pub fn from(ptr: *mut c_void) -> ScreenDC { ScreenDC { ptr: ptr } }
    pub fn null() -> ScreenDC { ScreenDC::from(0 as *mut c_void) }
    
    pub fn new() -> ScreenDC {
        unsafe { ScreenDC::from(wxScreenDC_Create()) }
    }
}

/// Methods of the wxWidgets' [wxScreenDC](http://docs.wxwidgets.org/3.0/classwx_screen_dc.html) class.
pub trait ScreenDCMethods : DCMethods {
    fn endDrawingOnTop(&self) -> c_int {
        unsafe { wxScreenDC_EndDrawingOnTop(self.ptr()) }
    }
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTop(self.ptr(), x, y, w, h) }
    }
    fn startDrawingOnTopOfWin<T: WindowMethods>(&self, win: &T) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.ptr(), win.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxScrollBar](http://docs.wxwidgets.org/3.0/classwx_scroll_bar.html) class.
pub struct ScrollBar { ptr: *mut c_void }
impl ScrollBarMethods for ScrollBar {}
impl ControlMethods for ScrollBar {}
impl WindowMethods for ScrollBar {}
impl EvtHandlerMethods for ScrollBar {}
impl ObjectMethods for ScrollBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrollBar {
    pub fn from(ptr: *mut c_void) -> ScrollBar { ScrollBar { ptr: ptr } }
    pub fn null() -> ScrollBar { ScrollBar::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ScrollBar {
        unsafe { ScrollBar::from(wxScrollBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxScrollBar](http://docs.wxwidgets.org/3.0/classwx_scroll_bar.html) class.
pub trait ScrollBarMethods : ControlMethods {
    fn getPageSize(&self) -> c_int {
        unsafe { wxScrollBar_GetPageSize(self.ptr()) }
    }
    fn getRange(&self) -> c_int {
        unsafe { wxScrollBar_GetRange(self.ptr()) }
    }
    fn getThumbPosition(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbPosition(self.ptr()) }
    }
    fn getThumbSize(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbSize(self.ptr()) }
    }
    fn setThumbPosition(&self, viewStart: c_int) {
        unsafe { wxScrollBar_SetThumbPosition(self.ptr(), viewStart) }
    }
}

/// Wraps the wxWidgets' [wxScrollEvent](http://docs.wxwidgets.org/3.0/classwx_scroll_event.html) class.
pub struct ScrollEvent { ptr: *mut c_void }
impl ScrollEventMethods for ScrollEvent {}
impl EventMethods for ScrollEvent {}
impl ObjectMethods for ScrollEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrollEvent {
    pub fn from(ptr: *mut c_void) -> ScrollEvent { ScrollEvent { ptr: ptr } }
    pub fn null() -> ScrollEvent { ScrollEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxScrollEvent](http://docs.wxwidgets.org/3.0/classwx_scroll_event.html) class.
pub trait ScrollEventMethods : EventMethods {
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.ptr()) }
    }
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxScrollWinEvent](http://docs.wxwidgets.org/3.0/classwx_scroll_win_event.html) class.
pub struct ScrollWinEvent { ptr: *mut c_void }
impl ScrollWinEventMethods for ScrollWinEvent {}
impl EventMethods for ScrollWinEvent {}
impl ObjectMethods for ScrollWinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrollWinEvent {
    pub fn from(ptr: *mut c_void) -> ScrollWinEvent { ScrollWinEvent { ptr: ptr } }
    pub fn null() -> ScrollWinEvent { ScrollWinEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxScrollWinEvent](http://docs.wxwidgets.org/3.0/classwx_scroll_win_event.html) class.
pub trait ScrollWinEventMethods : EventMethods {
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetOrientation(self.ptr()) }
    }
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetPosition(self.ptr()) }
    }
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxScrollWinEvent_SetOrientation(self.ptr(), orient) }
    }
    fn setPosition(&self, pos: c_int) {
        unsafe { wxScrollWinEvent_SetPosition(self.ptr(), pos) }
    }
}

/// Wraps the wxWidgets' [wxScrolledWindow](http://docs.wxwidgets.org/3.0/classwx_scrolled_window.html) class.
pub struct ScrolledWindow { ptr: *mut c_void }
impl ScrolledWindowMethods for ScrolledWindow {}
impl PanelMethods for ScrolledWindow {}
impl WindowMethods for ScrolledWindow {}
impl EvtHandlerMethods for ScrolledWindow {}
impl ObjectMethods for ScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrolledWindow {
    pub fn from(ptr: *mut c_void) -> ScrolledWindow { ScrolledWindow { ptr: ptr } }
    pub fn null() -> ScrolledWindow { ScrolledWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ScrolledWindow {
        unsafe { ScrolledWindow::from(wxScrolledWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxScrolledWindow](http://docs.wxwidgets.org/3.0/classwx_scrolled_window.html) class.
pub trait ScrolledWindowMethods : PanelMethods {
    fn adjustScrollbars(&self) {
        unsafe { wxScrolledWindow_AdjustScrollbars(self.ptr()) }
    }
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *mut c_void, yy: *mut c_void) {
        unsafe { wxScrolledWindow_CalcScrolledPosition(self.ptr(), x, y, xx, yy) }
    }
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *mut c_void, yy: *mut c_void) {
        unsafe { wxScrolledWindow_CalcUnscrolledPosition(self.ptr(), x, y, xx, yy) }
    }
    fn enableScrolling(&self, x_scrolling: c_int, y_scrolling: c_int) {
        unsafe { wxScrolledWindow_EnableScrolling(self.ptr(), x_scrolling, y_scrolling) }
    }
    fn getScaleX(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleX(self.ptr()) }
    }
    fn getScaleY(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleY(self.ptr()) }
    }
    fn getScrollPageSize(&self, orient: c_int) -> c_int {
        unsafe { wxScrolledWindow_GetScrollPageSize(self.ptr(), orient) }
    }
    fn getScrollPixelsPerUnit(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetScrollPixelsPerUnit(self.ptr(), _x, _y) }
    }
    fn getTargetWindow(&self) -> Window {
        unsafe { Window::from(wxScrolledWindow_GetTargetWindow(self.ptr())) }
    }
    fn getViewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetViewStart(self.ptr(), _x, _y) }
    }
    fn onDraw<T: DCMethods>(&self, dc: &T) {
        unsafe { wxScrolledWindow_OnDraw(self.ptr(), dc.ptr()) }
    }
    fn scroll(&self, x_pos: c_int, y_pos: c_int) {
        unsafe { wxScrolledWindow_Scroll(self.ptr(), x_pos, y_pos) }
    }
    fn setScale(&self, xs: c_double, ys: c_double) {
        unsafe { wxScrolledWindow_SetScale(self.ptr(), xs, ys) }
    }
    fn setScrollPageSize(&self, orient: c_int, pageSize: c_int) {
        unsafe { wxScrolledWindow_SetScrollPageSize(self.ptr(), orient, pageSize) }
    }
    fn setScrollbars(&self, pixelsPerUnitX: c_int, pixelsPerUnitY: c_int, noUnitsX: c_int, noUnitsY: c_int, xPos: c_int, yPos: c_int, noRefresh: c_int) {
        unsafe { wxScrolledWindow_SetScrollbars(self.ptr(), pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
    fn showScrollbars(&self, showh: c_int, showv: c_int) {
        unsafe { wxScrolledWindow_ShowScrollbars(self.ptr(), showh, showv) }
    }
    fn setTargetWindow<T: WindowMethods>(&self, target: &T) {
        unsafe { wxScrolledWindow_SetTargetWindow(self.ptr(), target.ptr()) }
    }
    fn viewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_ViewStart(self.ptr(), _x, _y) }
    }
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.ptr(), xstep, ystep) }
    }
}

/// Wraps the wxWidgets' [wxSetCursorEvent](http://docs.wxwidgets.org/3.0/classwx_set_cursor_event.html) class.
pub struct SetCursorEvent { ptr: *mut c_void }
impl SetCursorEventMethods for SetCursorEvent {}
impl EventMethods for SetCursorEvent {}
impl ObjectMethods for SetCursorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SetCursorEvent {
    pub fn from(ptr: *mut c_void) -> SetCursorEvent { SetCursorEvent { ptr: ptr } }
    pub fn null() -> SetCursorEvent { SetCursorEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSetCursorEvent](http://docs.wxwidgets.org/3.0/classwx_set_cursor_event.html) class.
pub trait SetCursorEventMethods : EventMethods {
    fn getCursor(&self) -> Cursor {
        unsafe { Cursor::from(wxSetCursorEvent_GetCursor(self.ptr())) }
    }
    fn getX(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetX(self.ptr()) }
    }
    fn getY(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetY(self.ptr()) }
    }
    fn hasCursor(&self) -> c_int {
        unsafe { wxSetCursorEvent_HasCursor(self.ptr()) }
    }
    fn setCursor<T: CursorMethods>(&self, cursor: &T) {
        unsafe { wxSetCursorEvent_SetCursor(self.ptr(), cursor.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxShowEvent](http://docs.wxwidgets.org/3.0/classwx_show_event.html) class.
pub struct ShowEvent { ptr: *mut c_void }
impl ShowEventMethods for ShowEvent {}
impl EventMethods for ShowEvent {}
impl ObjectMethods for ShowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ShowEvent {
    pub fn from(ptr: *mut c_void) -> ShowEvent { ShowEvent { ptr: ptr } }
    pub fn null() -> ShowEvent { ShowEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxShowEvent](http://docs.wxwidgets.org/3.0/classwx_show_event.html) class.
pub trait ShowEventMethods : EventMethods {
    fn isShown(&self) -> c_int {
        unsafe { wxShowEvent_IsShown(self.ptr()) }
    }
    fn setShow(&self, show: c_int) {
        unsafe { wxShowEvent_SetShow(self.ptr(), show) }
    }
}

/// Wraps the wxWidgets' [wxSimpleHelpProvider](http://docs.wxwidgets.org/3.0/classwx_simple_help_provider.html) class.
pub struct SimpleHelpProvider { ptr: *mut c_void }
impl SimpleHelpProviderMethods for SimpleHelpProvider {}
impl HelpProviderMethods for SimpleHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SimpleHelpProvider {
    pub fn from(ptr: *mut c_void) -> SimpleHelpProvider { SimpleHelpProvider { ptr: ptr } }
    pub fn null() -> SimpleHelpProvider { SimpleHelpProvider::from(0 as *mut c_void) }
    
    pub fn new() -> SimpleHelpProvider {
        unsafe { SimpleHelpProvider::from(wxSimpleHelpProvider_Create()) }
    }
}

/// Methods of the wxWidgets' [wxSimpleHelpProvider](http://docs.wxwidgets.org/3.0/classwx_simple_help_provider.html) class.
pub trait SimpleHelpProviderMethods : HelpProviderMethods {
}

/// Wraps the wxWidgets' [wxSingleChoiceDialog](http://docs.wxwidgets.org/3.0/classwx_single_choice_dialog.html) class.
pub struct SingleChoiceDialog { ptr: *mut c_void }
impl SingleChoiceDialogMethods for SingleChoiceDialog {}
impl DialogMethods for SingleChoiceDialog {}
impl TopLevelWindowMethods for SingleChoiceDialog {}
impl WindowMethods for SingleChoiceDialog {}
impl EvtHandlerMethods for SingleChoiceDialog {}
impl ObjectMethods for SingleChoiceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SingleChoiceDialog {
    pub fn from(ptr: *mut c_void) -> SingleChoiceDialog { SingleChoiceDialog { ptr: ptr } }
    pub fn null() -> SingleChoiceDialog { SingleChoiceDialog::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSingleChoiceDialog](http://docs.wxwidgets.org/3.0/classwx_single_choice_dialog.html) class.
pub trait SingleChoiceDialogMethods : DialogMethods {
}

/// Wraps the wxWidgets' [wxSize](http://docs.wxwidgets.org/3.0/classwx_size.html) class.
pub struct Size { ptr: *mut c_void }
impl SizeMethods for Size { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Size {
    pub fn from(ptr: *mut c_void) -> Size { Size { ptr: ptr } }
    pub fn null() -> Size { Size::from(0 as *mut c_void) }
    
    pub fn new(w: c_int, h: c_int) -> Size {
        unsafe { Size::from(wxSize_Create(w, h)) }
    }
}

/// Methods of the wxWidgets' [wxSize](http://docs.wxwidgets.org/3.0/classwx_size.html) class.
pub trait SizeMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn getHeight(&self) -> c_int {
        unsafe { wxSize_GetHeight(self.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxSize_GetWidth(self.ptr()) }
    }
    fn setHeight(&self, h: c_int) {
        unsafe { wxSize_SetHeight(self.ptr(), h) }
    }
    fn setWidth(&self, w: c_int) {
        unsafe { wxSize_SetWidth(self.ptr(), w) }
    }
}

/// Wraps the wxWidgets' [wxSizeEvent](http://docs.wxwidgets.org/3.0/classwx_size_event.html) class.
pub struct SizeEvent { ptr: *mut c_void }
impl SizeEventMethods for SizeEvent {}
impl EventMethods for SizeEvent {}
impl ObjectMethods for SizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SizeEvent {
    pub fn from(ptr: *mut c_void) -> SizeEvent { SizeEvent { ptr: ptr } }
    pub fn null() -> SizeEvent { SizeEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSizeEvent](http://docs.wxwidgets.org/3.0/classwx_size_event.html) class.
pub trait SizeEventMethods : EventMethods {
    fn getSize(&self) -> Size {
        unsafe { Size::from(wxSizeEvent_GetSize(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxSizer](http://docs.wxwidgets.org/3.0/classwx_sizer.html) class.
pub struct Sizer { ptr: *mut c_void }
impl SizerMethods for Sizer {}
impl ObjectMethods for Sizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Sizer {
    pub fn from(ptr: *mut c_void) -> Sizer { Sizer { ptr: ptr } }
    pub fn null() -> Sizer { Sizer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSizer](http://docs.wxwidgets.org/3.0/classwx_sizer.html) class.
pub trait SizerMethods : ObjectMethods {
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Add(self.ptr(), width, height, option, flag, border, userData) }
    }
    fn addSizer<T: SizerMethods>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    fn addWindow<T: WindowMethods>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddWindow(self.ptr(), window.ptr(), option, flag, border, userData) }
    }
    fn calcMin(&self) -> Size {
        unsafe { Size::from(wxSizer_CalcMin(self.ptr())) }
    }
    fn fit<T: WindowMethods>(&self, window: &T) {
        unsafe { wxSizer_Fit(self.ptr(), window.ptr()) }
    }
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.ptr(), _res, _cnt) }
    }
    fn getMinSize(&self) -> Size {
        unsafe { Size::from(wxSizer_GetMinSize(self.ptr())) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxSizer_GetPosition(self.ptr())) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size::from(wxSizer_GetSize(self.ptr())) }
    }
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Insert(self.ptr(), before, width, height, option, flag, border, userData) }
    }
    fn insertSizer<T: SizerMethods>(&self, before: c_int, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertSizer(self.ptr(), before, sizer.ptr(), option, flag, border, userData) }
    }
    fn insertWindow<T: WindowMethods>(&self, before: c_int, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertWindow(self.ptr(), before, window.ptr(), option, flag, border, userData) }
    }
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.ptr()) }
    }
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Prepend(self.ptr(), width, height, option, flag, border, userData) }
    }
    fn prependSizer<T: SizerMethods>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    fn prependWindow<T: WindowMethods>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependWindow(self.ptr(), window.ptr(), option, flag, border, userData) }
    }
    fn recalcSizes(&self) {
        unsafe { wxSizer_RecalcSizes(self.ptr()) }
    }
    fn setDimension(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetDimension(self.ptr(), x, y, width, height) }
    }
    fn setItemMinSize(&self, pos: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSize(self.ptr(), pos, width, height) }
    }
    fn setItemMinSizeSizer<T: SizerMethods>(&self, sizer: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.ptr(), sizer.ptr(), width, height) }
    }
    fn setItemMinSizeWindow<T: WindowMethods>(&self, window: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.ptr(), window.ptr(), width, height) }
    }
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.ptr(), width, height) }
    }
    fn setSizeHints<T: WindowMethods>(&self, window: &T) {
        unsafe { wxSizer_SetSizeHints(self.ptr(), window.ptr()) }
    }
    fn addSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddSpacer(self.ptr(), size) }
    }
    fn addStretchSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddStretchSpacer(self.ptr(), size) }
    }
    fn clear(&self, delete_windows: c_int) {
        unsafe { wxSizer_Clear(self.ptr(), delete_windows) }
    }
    fn detachWindow<T: WindowMethods>(&self, window: &T) -> c_int {
        unsafe { wxSizer_DetachWindow(self.ptr(), window.ptr()) }
    }
    fn detachSizer<T: SizerMethods>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_DetachSizer(self.ptr(), sizer.ptr()) }
    }
    fn detach(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Detach(self.ptr(), index) }
    }
    fn fitInside<T: WindowMethods>(&self, window: &T) {
        unsafe { wxSizer_FitInside(self.ptr(), window.ptr()) }
    }
    fn getContainingWindow(&self) -> Window {
        unsafe { Window::from(wxSizer_GetContainingWindow(self.ptr())) }
    }
    fn getItemWindow<T: WindowMethods>(&self, window: &T, recursive: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_GetItemWindow(self.ptr(), window.ptr(), recursive)) }
    }
    fn getItemSizer<T: SizerMethods>(&self, window: &T, recursive: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_GetItemSizer(self.ptr(), window.ptr(), recursive)) }
    }
    fn getItem(&self, index: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_GetItem(self.ptr(), index)) }
    }
    fn hideWindow<T: WindowMethods>(&self, window: &T) -> c_int {
        unsafe { wxSizer_HideWindow(self.ptr(), window.ptr()) }
    }
    fn hideSizer<T: SizerMethods>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_HideSizer(self.ptr(), sizer.ptr()) }
    }
    fn hide(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Hide(self.ptr(), index) }
    }
    fn insertSpacer(&self, index: c_int, size: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_InsertSpacer(self.ptr(), index, size)) }
    }
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_InsertStretchSpacer(self.ptr(), index, prop)) }
    }
    fn isShownWindow(&self, window: *mut *mut c_void) -> c_int {
        unsafe { wxSizer_IsShownWindow(self.ptr(), window) }
    }
    fn isShownSizer(&self, sizer: *mut *mut c_void) -> c_int {
        unsafe { wxSizer_IsShownSizer(self.ptr(), sizer) }
    }
    fn isShown(&self, index: c_int) -> c_int {
        unsafe { wxSizer_IsShown(self.ptr(), index) }
    }
    fn prependSpacer(&self, size: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_PrependSpacer(self.ptr(), size)) }
    }
    fn prependStretchSpacer(&self, prop: c_int) -> SizerItem {
        unsafe { SizerItem::from(wxSizer_PrependStretchSpacer(self.ptr(), prop)) }
    }
    fn replaceWindow<T: WindowMethods, U: WindowMethods>(&self, oldwin: &T, newwin: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceWindow(self.ptr(), oldwin.ptr(), newwin.ptr(), recursive) }
    }
    fn replaceSizer<T: SizerMethods, U: SizerMethods>(&self, oldsz: &T, newsz: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceSizer(self.ptr(), oldsz.ptr(), newsz.ptr(), recursive) }
    }
    fn replace<T: SizerItemMethods>(&self, oldindex: c_int, newitem: &T) -> c_int {
        unsafe { wxSizer_Replace(self.ptr(), oldindex, newitem.ptr()) }
    }
    fn setVirtualSizeHints<T: WindowMethods>(&self, window: &T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.ptr(), window.ptr()) }
    }
    fn showWindow<T: WindowMethods>(&self, window: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowWindow(self.ptr(), window.ptr(), show, recursive) }
    }
    fn showSizer<T: SizerMethods>(&self, sizer: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowSizer(self.ptr(), sizer.ptr(), show, recursive) }
    }
    fn show<T: SizerMethods>(&self, sizer: &T, index: c_int, show: c_int) -> c_int {
        unsafe { wxSizer_Show(self.ptr(), sizer.ptr(), index, show) }
    }
}

/// Wraps the wxWidgets' [wxSizerItem](http://docs.wxwidgets.org/3.0/classwx_sizer_item.html) class.
pub struct SizerItem { ptr: *mut c_void }
impl SizerItemMethods for SizerItem {}
impl ObjectMethods for SizerItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SizerItem {
    pub fn from(ptr: *mut c_void) -> SizerItem { SizerItem { ptr: ptr } }
    pub fn null() -> SizerItem { SizerItem::from(0 as *mut c_void) }
    
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> SizerItem {
        unsafe { SizerItem::from(wxSizerItem_Create(width, height, option, flag, border, userData)) }
    }
    pub fn newInSizer<T: SizerMethods>(sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInSizer(sizer.ptr(), option, flag, border, userData) }
    }
    pub fn newInWindow<T: WindowMethods>(window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInWindow(window.ptr(), option, flag, border, userData) }
    }
}

/// Methods of the wxWidgets' [wxSizerItem](http://docs.wxwidgets.org/3.0/classwx_sizer_item.html) class.
pub trait SizerItemMethods : ObjectMethods {
    fn calcMin(&self) -> Size {
        unsafe { Size::from(wxSizerItem_CalcMin(self.ptr())) }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxSizerItem_GetBorder(self.ptr()) }
    }
    fn getFlag(&self) -> c_int {
        unsafe { wxSizerItem_GetFlag(self.ptr()) }
    }
    fn getMinSize(&self) -> Size {
        unsafe { Size::from(wxSizerItem_GetMinSize(self.ptr())) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxSizerItem_GetPosition(self.ptr())) }
    }
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.ptr()) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size::from(wxSizerItem_GetSize(self.ptr())) }
    }
    fn getSizer(&self) -> Sizer {
        unsafe { Sizer::from(wxSizerItem_GetSizer(self.ptr())) }
    }
    fn getUserData(&self) -> *mut c_void {
        unsafe { wxSizerItem_GetUserData(self.ptr()) }
    }
    fn getWindow(&self) -> Window {
        unsafe { Window::from(wxSizerItem_GetWindow(self.ptr())) }
    }
    fn isSizer(&self) -> c_int {
        unsafe { wxSizerItem_IsSizer(self.ptr()) }
    }
    fn isSpacer(&self) -> c_int {
        unsafe { wxSizerItem_IsSpacer(self.ptr()) }
    }
    fn isWindow(&self) -> c_int {
        unsafe { wxSizerItem_IsWindow(self.ptr()) }
    }
    fn setBorder(&self, border: c_int) {
        unsafe { wxSizerItem_SetBorder(self.ptr(), border) }
    }
    fn setDimension(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxSizerItem_SetDimension(self.ptr(), _x, _y, _w, _h) }
    }
    fn setFlag(&self, flag: c_int) {
        unsafe { wxSizerItem_SetFlag(self.ptr(), flag) }
    }
    fn setFloatRatio(&self, ratio: c_float) {
        unsafe { wxSizerItem_SetFloatRatio(self.ptr(), ratio) }
    }
    fn setInitSize(&self, x: c_int, y: c_int) {
        unsafe { wxSizerItem_SetInitSize(self.ptr(), x, y) }
    }
    fn setRatio(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetRatio(self.ptr(), width, height) }
    }
    fn setSizer<T: SizerMethods>(&self, sizer: &T) {
        unsafe { wxSizerItem_SetSizer(self.ptr(), sizer.ptr()) }
    }
    fn setWindow<T: WindowMethods>(&self, window: &T) {
        unsafe { wxSizerItem_SetWindow(self.ptr(), window.ptr()) }
    }
    fn deleteWindows(&self) {
        unsafe { wxSizerItem_DeleteWindows(self.ptr()) }
    }
    fn detachSizer(&self) {
        unsafe { wxSizerItem_DetachSizer(self.ptr()) }
    }
    fn getProportion(&self) -> c_int {
        unsafe { wxSizerItem_GetProportion(self.ptr()) }
    }
    fn getRect(&self) -> Rect {
        unsafe { Rect::from(wxSizerItem_GetRect(self.ptr())) }
    }
    fn getSpacer(&self) -> Size {
        unsafe { Size::from(wxSizerItem_GetSpacer(self.ptr())) }
    }
    fn isShown(&self) -> c_int {
        unsafe { wxSizerItem_IsShown(self.ptr()) }
    }
    fn setProportion(&self, proportion: c_int) {
        unsafe { wxSizerItem_SetProportion(self.ptr(), proportion) }
    }
    fn setSpacer(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetSpacer(self.ptr(), width, height) }
    }
    fn show(&self, show: c_int) {
        unsafe { wxSizerItem_Show(self.ptr(), show) }
    }
}

/// Wraps the wxWidgets' [wxSlider](http://docs.wxwidgets.org/3.0/classwx_slider.html) class.
pub struct Slider { ptr: *mut c_void }
impl SliderMethods for Slider {}
impl ControlMethods for Slider {}
impl WindowMethods for Slider {}
impl EvtHandlerMethods for Slider {}
impl ObjectMethods for Slider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Slider {
    pub fn from(ptr: *mut c_void) -> Slider { Slider { ptr: ptr } }
    pub fn null() -> Slider { Slider::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> Slider {
        unsafe { Slider::from(wxSlider_Create(_prt.ptr(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxSlider](http://docs.wxwidgets.org/3.0/classwx_slider.html) class.
pub trait SliderMethods : ControlMethods {
    fn clearSel(&self) {
        unsafe { wxSlider_ClearSel(self.ptr()) }
    }
    fn clearTicks(&self) {
        unsafe { wxSlider_ClearTicks(self.ptr()) }
    }
    fn getLineSize(&self) -> c_int {
        unsafe { wxSlider_GetLineSize(self.ptr()) }
    }
    fn getMax(&self) -> c_int {
        unsafe { wxSlider_GetMax(self.ptr()) }
    }
    fn getMin(&self) -> c_int {
        unsafe { wxSlider_GetMin(self.ptr()) }
    }
    fn getPageSize(&self) -> c_int {
        unsafe { wxSlider_GetPageSize(self.ptr()) }
    }
    fn getSelEnd(&self) -> c_int {
        unsafe { wxSlider_GetSelEnd(self.ptr()) }
    }
    fn getSelStart(&self) -> c_int {
        unsafe { wxSlider_GetSelStart(self.ptr()) }
    }
    fn getThumbLength(&self) -> c_int {
        unsafe { wxSlider_GetThumbLength(self.ptr()) }
    }
    fn getTickFreq(&self) -> c_int {
        unsafe { wxSlider_GetTickFreq(self.ptr()) }
    }
    fn getValue(&self) -> c_int {
        unsafe { wxSlider_GetValue(self.ptr()) }
    }
    fn setLineSize(&self, lineSize: c_int) {
        unsafe { wxSlider_SetLineSize(self.ptr(), lineSize) }
    }
    fn setPageSize(&self, pageSize: c_int) {
        unsafe { wxSlider_SetPageSize(self.ptr(), pageSize) }
    }
    fn setRange(&self, minValue: c_int, maxValue: c_int) {
        unsafe { wxSlider_SetRange(self.ptr(), minValue, maxValue) }
    }
    fn setSelection(&self, minPos: c_int, maxPos: c_int) {
        unsafe { wxSlider_SetSelection(self.ptr(), minPos, maxPos) }
    }
    fn setThumbLength(&self, len: c_int) {
        unsafe { wxSlider_SetThumbLength(self.ptr(), len) }
    }
    fn setTick(&self, tickPos: c_int) {
        unsafe { wxSlider_SetTick(self.ptr(), tickPos) }
    }
    fn setTickFreq(&self, n: c_int, pos: c_int) {
        unsafe { wxSlider_SetTickFreq(self.ptr(), n, pos) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxSlider_SetValue(self.ptr(), value) }
    }
}

/// Wraps the wxWidgets' [wxSpinButton](http://docs.wxwidgets.org/3.0/classwx_spin_button.html) class.
pub struct SpinButton { ptr: *mut c_void }
impl SpinButtonMethods for SpinButton {}
impl ControlMethods for SpinButton {}
impl WindowMethods for SpinButton {}
impl EvtHandlerMethods for SpinButton {}
impl ObjectMethods for SpinButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SpinButton {
    pub fn from(ptr: *mut c_void) -> SpinButton { SpinButton { ptr: ptr } }
    pub fn null() -> SpinButton { SpinButton::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> SpinButton {
        unsafe { SpinButton::from(wxSpinButton_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxSpinButton](http://docs.wxwidgets.org/3.0/classwx_spin_button.html) class.
pub trait SpinButtonMethods : ControlMethods {
    fn getMax(&self) -> c_int {
        unsafe { wxSpinButton_GetMax(self.ptr()) }
    }
    fn getMin(&self) -> c_int {
        unsafe { wxSpinButton_GetMin(self.ptr()) }
    }
    fn getValue(&self) -> c_int {
        unsafe { wxSpinButton_GetValue(self.ptr()) }
    }
    fn setRange(&self, minVal: c_int, maxVal: c_int) {
        unsafe { wxSpinButton_SetRange(self.ptr(), minVal, maxVal) }
    }
    fn setValue(&self, val: c_int) {
        unsafe { wxSpinButton_SetValue(self.ptr(), val) }
    }
}

/// Wraps the wxWidgets' [wxSpinCtrl](http://docs.wxwidgets.org/3.0/classwx_spin_ctrl.html) class.
pub struct SpinCtrl { ptr: *mut c_void }
impl SpinCtrlMethods for SpinCtrl {}
impl ControlMethods for SpinCtrl {}
impl WindowMethods for SpinCtrl {}
impl EvtHandlerMethods for SpinCtrl {}
impl ObjectMethods for SpinCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SpinCtrl {
    pub fn from(ptr: *mut c_void) -> SpinCtrl { SpinCtrl { ptr: ptr } }
    pub fn null() -> SpinCtrl { SpinCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> SpinCtrl {
        let _txt = strToString(_txt);
        unsafe { SpinCtrl::from(wxSpinCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) }
    }
}

/// Methods of the wxWidgets' [wxSpinCtrl](http://docs.wxwidgets.org/3.0/classwx_spin_ctrl.html) class.
pub trait SpinCtrlMethods : ControlMethods {
    fn getMax(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMax(self.ptr()) }
    }
    fn getMin(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMin(self.ptr()) }
    }
    fn getValue(&self) -> c_int {
        unsafe { wxSpinCtrl_GetValue(self.ptr()) }
    }
    fn setRange(&self, min_val: c_int, max_val: c_int) {
        unsafe { wxSpinCtrl_SetRange(self.ptr(), min_val, max_val) }
    }
    fn setValue(&self, val: c_int) {
        unsafe { wxSpinCtrl_SetValue(self.ptr(), val) }
    }
}

/// Wraps the wxWidgets' [wxSpinEvent](http://docs.wxwidgets.org/3.0/classwx_spin_event.html) class.
pub struct SpinEvent { ptr: *mut c_void }
impl SpinEventMethods for SpinEvent {}
impl NotifyEventMethods for SpinEvent {}
impl CommandEventMethods for SpinEvent {}
impl EventMethods for SpinEvent {}
impl ObjectMethods for SpinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SpinEvent {
    pub fn from(ptr: *mut c_void) -> SpinEvent { SpinEvent { ptr: ptr } }
    pub fn null() -> SpinEvent { SpinEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSpinEvent](http://docs.wxwidgets.org/3.0/classwx_spin_event.html) class.
pub trait SpinEventMethods : NotifyEventMethods {
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.ptr()) }
    }
    fn setPosition(&self, pos: c_int) {
        unsafe { wxSpinEvent_SetPosition(self.ptr(), pos) }
    }
}

/// Wraps the wxWidgets' [wxSplitterEvent](http://docs.wxwidgets.org/3.0/classwx_splitter_event.html) class.
pub struct SplitterEvent { ptr: *mut c_void }
impl SplitterEventMethods for SplitterEvent {}
impl NotifyEventMethods for SplitterEvent {}
impl CommandEventMethods for SplitterEvent {}
impl EventMethods for SplitterEvent {}
impl ObjectMethods for SplitterEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterEvent {
    pub fn from(ptr: *mut c_void) -> SplitterEvent { SplitterEvent { ptr: ptr } }
    pub fn null() -> SplitterEvent { SplitterEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSplitterEvent](http://docs.wxwidgets.org/3.0/classwx_splitter_event.html) class.
pub trait SplitterEventMethods : NotifyEventMethods {
}

/// Wraps the wxWidgets' [wxSplitterWindow](http://docs.wxwidgets.org/3.0/classwx_splitter_window.html) class.
pub struct SplitterWindow { ptr: *mut c_void }
impl SplitterWindowMethods for SplitterWindow {}
impl WindowMethods for SplitterWindow {}
impl EvtHandlerMethods for SplitterWindow {}
impl ObjectMethods for SplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterWindow {
    pub fn from(ptr: *mut c_void) -> SplitterWindow { SplitterWindow { ptr: ptr } }
    pub fn null() -> SplitterWindow { SplitterWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> SplitterWindow {
        unsafe { SplitterWindow::from(wxSplitterWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxSplitterWindow](http://docs.wxwidgets.org/3.0/classwx_splitter_window.html) class.
pub trait SplitterWindowMethods : WindowMethods {
    fn getBorderSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetBorderSize(self.ptr()) }
    }
    fn getMinimumPaneSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetMinimumPaneSize(self.ptr()) }
    }
    fn getSashPosition(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashPosition(self.ptr()) }
    }
    fn getSashSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashSize(self.ptr()) }
    }
    fn getSplitMode(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSplitMode(self.ptr()) }
    }
    fn getWindow1(&self) -> Window {
        unsafe { Window::from(wxSplitterWindow_GetWindow1(self.ptr())) }
    }
    fn getWindow2(&self) -> Window {
        unsafe { Window::from(wxSplitterWindow_GetWindow2(self.ptr())) }
    }
    fn initialize<T: WindowMethods>(&self, window: &T) {
        unsafe { wxSplitterWindow_Initialize(self.ptr(), window.ptr()) }
    }
    fn isSplit(&self) -> c_int {
        unsafe { wxSplitterWindow_IsSplit(self.ptr()) }
    }
    fn replaceWindow<T: WindowMethods, U: WindowMethods>(&self, winOld: &T, winNew: &U) -> c_int {
        unsafe { wxSplitterWindow_ReplaceWindow(self.ptr(), winOld.ptr(), winNew.ptr()) }
    }
    fn setBorderSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetBorderSize(self.ptr(), width) }
    }
    fn setMinimumPaneSize(&self, min: c_int) {
        unsafe { wxSplitterWindow_SetMinimumPaneSize(self.ptr(), min) }
    }
    fn setSashPosition(&self, position: c_int, redraw: c_int) {
        unsafe { wxSplitterWindow_SetSashPosition(self.ptr(), position, redraw) }
    }
    fn setSashSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetSashSize(self.ptr(), width) }
    }
    fn setSplitMode(&self, mode: c_int) {
        unsafe { wxSplitterWindow_SetSplitMode(self.ptr(), mode) }
    }
    fn splitHorizontally<T: WindowMethods, U: WindowMethods>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitHorizontally(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    fn splitVertically<T: WindowMethods, U: WindowMethods>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitVertically(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    fn unsplit<T: WindowMethods>(&self, toRemove: &T) -> c_int {
        unsafe { wxSplitterWindow_Unsplit(self.ptr(), toRemove.ptr()) }
    }
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self.ptr()) }
    }
    fn setSashGravity(&self, gravity: c_double) {
        unsafe { wxSplitterWindow_SetSashGravity(self.ptr(), gravity) }
    }
}

/// Wraps the wxWidgets' [wxStaticBitmap](http://docs.wxwidgets.org/3.0/classwx_static_bitmap.html) class.
pub struct StaticBitmap { ptr: *mut c_void }
impl StaticBitmapMethods for StaticBitmap {}
impl ControlMethods for StaticBitmap {}
impl WindowMethods for StaticBitmap {}
impl EvtHandlerMethods for StaticBitmap {}
impl ObjectMethods for StaticBitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticBitmap {
    pub fn from(ptr: *mut c_void) -> StaticBitmap { StaticBitmap { ptr: ptr } }
    pub fn null() -> StaticBitmap { StaticBitmap::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods, U: BitmapMethods>(_prt: &T, _id: c_int, bitmap: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticBitmap {
        unsafe { StaticBitmap::from(wxStaticBitmap_Create(_prt.ptr(), _id, bitmap.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxStaticBitmap](http://docs.wxwidgets.org/3.0/classwx_static_bitmap.html) class.
pub trait StaticBitmapMethods : ControlMethods {
    fn getBitmap<T: BitmapMethods>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    fn getIcon<T: IconMethods>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetIcon(self.ptr(), _ref.ptr()) }
    }
    fn setBitmap<T: BitmapMethods>(&self, bitmap: &T) {
        unsafe { wxStaticBitmap_SetBitmap(self.ptr(), bitmap.ptr()) }
    }
    fn setIcon<T: IconMethods>(&self, icon: &T) {
        unsafe { wxStaticBitmap_SetIcon(self.ptr(), icon.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxStaticBox](http://docs.wxwidgets.org/3.0/classwx_static_box.html) class.
pub struct StaticBox { ptr: *mut c_void }
impl StaticBoxMethods for StaticBox {}
impl ControlMethods for StaticBox {}
impl WindowMethods for StaticBox {}
impl EvtHandlerMethods for StaticBox {}
impl ObjectMethods for StaticBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticBox {
    pub fn from(ptr: *mut c_void) -> StaticBox { StaticBox { ptr: ptr } }
    pub fn null() -> StaticBox { StaticBox::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticBox {
        let _txt = strToString(_txt);
        unsafe { StaticBox::from(wxStaticBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxStaticBox](http://docs.wxwidgets.org/3.0/classwx_static_box.html) class.
pub trait StaticBoxMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxStaticBoxSizer](http://docs.wxwidgets.org/3.0/classwx_static_box_sizer.html) class.
pub struct StaticBoxSizer { ptr: *mut c_void }
impl StaticBoxSizerMethods for StaticBoxSizer {}
impl BoxSizerMethods for StaticBoxSizer {}
impl SizerMethods for StaticBoxSizer {}
impl ObjectMethods for StaticBoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticBoxSizer {
    pub fn from(ptr: *mut c_void) -> StaticBoxSizer { StaticBoxSizer { ptr: ptr } }
    pub fn null() -> StaticBoxSizer { StaticBoxSizer::from(0 as *mut c_void) }
    
    pub fn new<T: StaticBoxMethods>(box_: &T, orient: c_int) -> StaticBoxSizer {
        unsafe { StaticBoxSizer::from(wxStaticBoxSizer_Create(box_.ptr(), orient)) }
    }
}

/// Methods of the wxWidgets' [wxStaticBoxSizer](http://docs.wxwidgets.org/3.0/classwx_static_box_sizer.html) class.
pub trait StaticBoxSizerMethods : BoxSizerMethods {
    fn getStaticBox(&self) -> StaticBox {
        unsafe { StaticBox::from(wxStaticBoxSizer_GetStaticBox(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxStaticLine](http://docs.wxwidgets.org/3.0/classwx_static_line.html) class.
pub struct StaticLine { ptr: *mut c_void }
impl StaticLineMethods for StaticLine {}
impl ControlMethods for StaticLine {}
impl WindowMethods for StaticLine {}
impl EvtHandlerMethods for StaticLine {}
impl ObjectMethods for StaticLine { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticLine {
    pub fn from(ptr: *mut c_void) -> StaticLine { StaticLine { ptr: ptr } }
    pub fn null() -> StaticLine { StaticLine::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticLine {
        unsafe { StaticLine::from(wxStaticLine_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxStaticLine](http://docs.wxwidgets.org/3.0/classwx_static_line.html) class.
pub trait StaticLineMethods : ControlMethods {
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.ptr()) }
    }
    fn isVertical(&self) -> c_int {
        unsafe { wxStaticLine_IsVertical(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxStaticText](http://docs.wxwidgets.org/3.0/classwx_static_text.html) class.
pub struct StaticText { ptr: *mut c_void }
impl StaticTextMethods for StaticText {}
impl ControlMethods for StaticText {}
impl WindowMethods for StaticText {}
impl EvtHandlerMethods for StaticText {}
impl ObjectMethods for StaticText { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticText {
    pub fn from(ptr: *mut c_void) -> StaticText { StaticText { ptr: ptr } }
    pub fn null() -> StaticText { StaticText::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticText {
        let _txt = strToString(_txt);
        unsafe { StaticText::from(wxStaticText_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxStaticText](http://docs.wxwidgets.org/3.0/classwx_static_text.html) class.
pub trait StaticTextMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxStatusBar](http://docs.wxwidgets.org/3.0/classwx_status_bar.html) class.
pub struct StatusBar { ptr: *mut c_void }
impl StatusBarMethods for StatusBar {}
impl WindowMethods for StatusBar {}
impl EvtHandlerMethods for StatusBar {}
impl ObjectMethods for StatusBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StatusBar {
    pub fn from(ptr: *mut c_void) -> StatusBar { StatusBar { ptr: ptr } }
    pub fn null() -> StatusBar { StatusBar::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StatusBar {
        unsafe { StatusBar::from(wxStatusBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxStatusBar](http://docs.wxwidgets.org/3.0/classwx_status_bar.html) class.
pub trait StatusBarMethods : WindowMethods {
    fn getBorderX(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderX(self.ptr()) }
    }
    fn getBorderY(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderY(self.ptr()) }
    }
    fn getFieldsCount(&self) -> c_int {
        unsafe { wxStatusBar_GetFieldsCount(self.ptr()) }
    }
    fn getStatusText(&self, number: c_int) -> ~str {
        unsafe { String::from(wxStatusBar_GetStatusText(self.ptr(), number)).to_str() }
    }
    fn setFieldsCount(&self, number: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetFieldsCount(self.ptr(), number, widths) }
    }
    fn setMinHeight(&self, height: c_int) {
        unsafe { wxStatusBar_SetMinHeight(self.ptr(), height) }
    }
    fn setStatusText(&self, text: &str, number: c_int) {
        let text = strToString(text);
        unsafe { wxStatusBar_SetStatusText(self.ptr(), text.ptr(), number) }
    }
    fn setStatusWidths(&self, n: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.ptr(), n, widths) }
    }
}

/// Wraps the wxWidgets' [wxSysColourChangedEvent](http://docs.wxwidgets.org/3.0/classwx_sys_colour_changed_event.html) class.
pub struct SysColourChangedEvent { ptr: *mut c_void }
impl SysColourChangedEventMethods for SysColourChangedEvent {}
impl EventMethods for SysColourChangedEvent {}
impl ObjectMethods for SysColourChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SysColourChangedEvent {
    pub fn from(ptr: *mut c_void) -> SysColourChangedEvent { SysColourChangedEvent { ptr: ptr } }
    pub fn null() -> SysColourChangedEvent { SysColourChangedEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSysColourChangedEvent](http://docs.wxwidgets.org/3.0/classwx_sys_colour_changed_event.html) class.
pub trait SysColourChangedEventMethods : EventMethods {
}

/// Wraps the wxWidgets' [wxSystemSettings](http://docs.wxwidgets.org/3.0/classwx_system_settings.html) class.
pub struct SystemSettings { ptr: *mut c_void }
impl SystemSettingsMethods for SystemSettings {}
impl ObjectMethods for SystemSettings { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SystemSettings {
    pub fn from(ptr: *mut c_void) -> SystemSettings { SystemSettings { ptr: ptr } }
    pub fn null() -> SystemSettings { SystemSettings::from(0 as *mut c_void) }
    
    pub fn getColour<T: ColourMethods>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetColour(index, _ref.ptr()) }
    }
    pub fn getFont<T: FontMethods>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetFont(index, _ref.ptr()) }
    }
    pub fn getMetric(index: c_int) -> c_int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    pub fn getScreenType() -> c_int {
        unsafe { wxSystemSettings_GetScreenType() }
    }
}

/// Methods of the wxWidgets' [wxSystemSettings](http://docs.wxwidgets.org/3.0/classwx_system_settings.html) class.
pub trait SystemSettingsMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxTextAttr](http://docs.wxwidgets.org/3.0/classwx_text_attr.html) class.
pub struct TextAttr { ptr: *mut c_void }
impl TextAttrMethods for TextAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextAttr {
    pub fn from(ptr: *mut c_void) -> TextAttr { TextAttr { ptr: ptr } }
    pub fn null() -> TextAttr { TextAttr::from(0 as *mut c_void) }
    
    pub fn new<T: ColourMethods, U: ColourMethods, V: FontMethods>(colText: &T, colBack: &U, font: &V) -> TextAttr {
        unsafe { TextAttr::from(wxTextAttr_Create(colText.ptr(), colBack.ptr(), font.ptr())) }
    }
    pub fn newDefault() -> TextAttr {
        unsafe { TextAttr::from(wxTextAttr_CreateDefault()) }
    }
}

/// Methods of the wxWidgets' [wxTextAttr](http://docs.wxwidgets.org/3.0/classwx_text_attr.html) class.
pub trait TextAttrMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.ptr()) }
    }
    fn getBackgroundColour<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxTextAttr_GetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn getFont<T: FontMethods>(&self, font: &T) {
        unsafe { wxTextAttr_GetFont(self.ptr(), font.ptr()) }
    }
    fn getTextColour<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxTextAttr_GetTextColour(self.ptr(), colour.ptr()) }
    }
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxTextAttr_HasBackgroundColour(self.ptr()) }
    }
    fn hasFont(&self) -> c_int {
        unsafe { wxTextAttr_HasFont(self.ptr()) }
    }
    fn hasTextColour(&self) -> c_int {
        unsafe { wxTextAttr_HasTextColour(self.ptr()) }
    }
    fn isDefault(&self) -> c_int {
        unsafe { wxTextAttr_IsDefault(self.ptr()) }
    }
    fn setTextColour<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxTextAttr_SetTextColour(self.ptr(), colour.ptr()) }
    }
    fn setBackgroundColour<T: ColourMethods>(&self, colour: &T) {
        unsafe { wxTextAttr_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setFont<T: FontMethods>(&self, font: &T) {
        unsafe { wxTextAttr_SetFont(self.ptr(), font.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTextCtrl](http://docs.wxwidgets.org/3.0/classwx_text_ctrl.html) class.
pub struct TextCtrl { ptr: *mut c_void }
impl TextCtrlMethods for TextCtrl {}
impl ControlMethods for TextCtrl {}
impl WindowMethods for TextCtrl {}
impl EvtHandlerMethods for TextCtrl {}
impl ObjectMethods for TextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextCtrl {
    pub fn from(ptr: *mut c_void) -> TextCtrl { TextCtrl { ptr: ptr } }
    pub fn null() -> TextCtrl { TextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> TextCtrl {
        let _txt = strToString(_txt);
        unsafe { TextCtrl::from(wxTextCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxTextCtrl](http://docs.wxwidgets.org/3.0/classwx_text_ctrl.html) class.
pub trait TextCtrlMethods : ControlMethods {
    fn appendText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxTextCtrl_AppendText(self.ptr(), text.ptr()) }
    }
    fn canCopy(&self) -> c_int {
        unsafe { wxTextCtrl_CanCopy(self.ptr()) }
    }
    fn canCut(&self) -> c_int {
        unsafe { wxTextCtrl_CanCut(self.ptr()) }
    }
    fn canPaste(&self) -> c_int {
        unsafe { wxTextCtrl_CanPaste(self.ptr()) }
    }
    fn canRedo(&self) -> c_int {
        unsafe { wxTextCtrl_CanRedo(self.ptr()) }
    }
    fn canUndo(&self) -> c_int {
        unsafe { wxTextCtrl_CanUndo(self.ptr()) }
    }
    fn changeValue(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxTextCtrl_ChangeValue(self.ptr(), text.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxTextCtrl_Clear(self.ptr()) }
    }
    fn copy(&self) {
        unsafe { wxTextCtrl_Copy(self.ptr()) }
    }
    fn cut(&self) {
        unsafe { wxTextCtrl_Cut(self.ptr()) }
    }
    fn discardEdits(&self) {
        unsafe { wxTextCtrl_DiscardEdits(self.ptr()) }
    }
    fn getInsertionPoint(&self) -> c_long {
        unsafe { wxTextCtrl_GetInsertionPoint(self.ptr()) }
    }
    fn getLastPosition(&self) -> c_long {
        unsafe { wxTextCtrl_GetLastPosition(self.ptr()) }
    }
    fn getLineLength(&self, lineNo: c_long) -> c_int {
        unsafe { wxTextCtrl_GetLineLength(self.ptr(), lineNo) }
    }
    fn getLineText(&self, lineNo: c_long) -> ~str {
        unsafe { String::from(wxTextCtrl_GetLineText(self.ptr(), lineNo)).to_str() }
    }
    fn getNumberOfLines(&self) -> c_int {
        unsafe { wxTextCtrl_GetNumberOfLines(self.ptr()) }
    }
    fn getSelection(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { wxTextCtrl_GetSelection(self.ptr(), from, to) }
    }
    fn getValue(&self) -> ~str {
        unsafe { String::from(wxTextCtrl_GetValue(self.ptr())).to_str() }
    }
    fn isEditable(&self) -> c_int {
        unsafe { wxTextCtrl_IsEditable(self.ptr()) }
    }
    fn isModified(&self) -> c_int {
        unsafe { wxTextCtrl_IsModified(self.ptr()) }
    }
    fn loadFile(&self, file: &str) -> c_int {
        let file = strToString(file);
        unsafe { wxTextCtrl_LoadFile(self.ptr(), file.ptr()) }
    }
    fn paste(&self) {
        unsafe { wxTextCtrl_Paste(self.ptr()) }
    }
    fn positionToXY(&self, pos: c_long, x: *mut c_long, y: *mut c_long) -> c_int {
        unsafe { wxTextCtrl_PositionToXY(self.ptr(), pos, x, y) }
    }
    fn redo(&self) {
        unsafe { wxTextCtrl_Redo(self.ptr()) }
    }
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_Remove(self.ptr(), from, to) }
    }
    fn replace(&self, from: c_long, to: c_long, value: &str) {
        let value = strToString(value);
        unsafe { wxTextCtrl_Replace(self.ptr(), from, to, value.ptr()) }
    }
    fn saveFile(&self, file: &str) -> c_int {
        let file = strToString(file);
        unsafe { wxTextCtrl_SaveFile(self.ptr(), file.ptr()) }
    }
    fn setEditable(&self, editable: c_int) {
        unsafe { wxTextCtrl_SetEditable(self.ptr(), editable) }
    }
    fn setInsertionPoint(&self, pos: c_long) {
        unsafe { wxTextCtrl_SetInsertionPoint(self.ptr(), pos) }
    }
    fn setInsertionPointEnd(&self) {
        unsafe { wxTextCtrl_SetInsertionPointEnd(self.ptr()) }
    }
    fn setSelection(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_SetSelection(self.ptr(), from, to) }
    }
    fn setValue(&self, value: &str) {
        let value = strToString(value);
        unsafe { wxTextCtrl_SetValue(self.ptr(), value.ptr()) }
    }
    fn showPosition(&self, pos: c_long) {
        unsafe { wxTextCtrl_ShowPosition(self.ptr(), pos) }
    }
    fn undo(&self) {
        unsafe { wxTextCtrl_Undo(self.ptr()) }
    }
    fn writeText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxTextCtrl_WriteText(self.ptr(), text.ptr()) }
    }
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.ptr(), x, y) }
    }
    fn emulateKeyPress<T: KeyEventMethods>(&self, keyevent: &T) -> c_int {
        unsafe { wxTextCtrl_EmulateKeyPress(self.ptr(), keyevent.ptr()) }
    }
    fn getDefaultStyle(&self) -> TextAttr {
        unsafe { TextAttr::from(wxTextCtrl_GetDefaultStyle(self.ptr())) }
    }
    fn getRange(&self, from: c_long, to: c_long) -> ~str {
        unsafe { String::from(wxTextCtrl_GetRange(self.ptr(), from, to)).to_str() }
    }
    fn getStringSelection(&self) -> ~str {
        unsafe { String::from(wxTextCtrl_GetStringSelection(self.ptr())).to_str() }
    }
    fn isMultiLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsMultiLine(self.ptr()) }
    }
    fn isSingleLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsSingleLine(self.ptr()) }
    }
    fn setDefaultStyle<T: TextAttrMethods>(&self, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetDefaultStyle(self.ptr(), style.ptr()) }
    }
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.ptr(), len) }
    }
    fn setStyle<T: TextAttrMethods>(&self, start: c_long, end: c_long, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetStyle(self.ptr(), start, end, style.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTextDataObject](http://docs.wxwidgets.org/3.0/classwx_text_data_object.html) class.
pub struct TextDataObject { ptr: *mut c_void }
impl TextDataObjectMethods for TextDataObject {}
impl DataObjectSimpleMethods for TextDataObject {}
impl DataObjectMethods for TextDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextDataObject {
    pub fn from(ptr: *mut c_void) -> TextDataObject { TextDataObject { ptr: ptr } }
    pub fn null() -> TextDataObject { TextDataObject::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTextDataObject](http://docs.wxwidgets.org/3.0/classwx_text_data_object.html) class.
pub trait TextDataObjectMethods : DataObjectSimpleMethods {
}

/// Wraps the wxWidgets' [wxTextDropTarget](http://docs.wxwidgets.org/3.0/classwx_text_drop_target.html) class.
/// Rather use the wxRust-specific [RustTextDropTarget](struct.RustTextDropTarget.html) class.
pub struct TextDropTarget { ptr: *mut c_void }
impl TextDropTargetMethods for TextDropTarget {}
impl DropTargetMethods for TextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextDropTarget {
    pub fn from(ptr: *mut c_void) -> TextDropTarget { TextDropTarget { ptr: ptr } }
    pub fn null() -> TextDropTarget { TextDropTarget::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTextDropTarget](http://docs.wxwidgets.org/3.0/classwx_text_drop_target.html) class.
pub trait TextDropTargetMethods : DropTargetMethods {
}

/// Wraps the wxWidgets' [wxTextEntryDialog](http://docs.wxwidgets.org/3.0/classwx_text_entry_dialog.html) class.
pub struct TextEntryDialog { ptr: *mut c_void }
impl TextEntryDialogMethods for TextEntryDialog {}
impl DialogMethods for TextEntryDialog {}
impl TopLevelWindowMethods for TextEntryDialog {}
impl WindowMethods for TextEntryDialog {}
impl EvtHandlerMethods for TextEntryDialog {}
impl ObjectMethods for TextEntryDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextEntryDialog {
    pub fn from(ptr: *mut c_void) -> TextEntryDialog { TextEntryDialog { ptr: ptr } }
    pub fn null() -> TextEntryDialog { TextEntryDialog::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTextEntryDialog](http://docs.wxwidgets.org/3.0/classwx_text_entry_dialog.html) class.
pub trait TextEntryDialogMethods : DialogMethods {
}

/// Wraps the wxWidgets' [wxTextValidator](http://docs.wxwidgets.org/3.0/classwx_text_validator.html) class.
/// Rather use the wxRust-specific [RustTextValidator](struct.RustTextValidator.html) class.
pub struct TextValidator { ptr: *mut c_void }
impl TextValidatorMethods for TextValidator {}
impl ValidatorMethods for TextValidator {}
impl EvtHandlerMethods for TextValidator {}
impl ObjectMethods for TextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextValidator {
    pub fn from(ptr: *mut c_void) -> TextValidator { TextValidator { ptr: ptr } }
    pub fn null() -> TextValidator { TextValidator::from(0 as *mut c_void) }
    
    pub fn new(style: c_int, val: *mut c_void) -> TextValidator {
        unsafe { TextValidator::from(wxTextValidator_Create(style, val)) }
    }
}

/// Methods of the wxWidgets' [wxTextValidator](http://docs.wxwidgets.org/3.0/classwx_text_validator.html) class.
pub trait TextValidatorMethods : ValidatorMethods {
    fn getExcludes(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxTextValidator_GetExcludes(self.ptr(), _ref) }
    }
    fn getIncludes(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxTextValidator_GetIncludes(self.ptr(), _ref) }
    }
    fn setExcludes(&self, list: *mut c_void, count: c_int) {
        unsafe { wxTextValidator_SetExcludes(self.ptr(), list, count) }
    }
    fn setIncludes(&self, list: *mut c_void, count: c_int) {
        unsafe { wxTextValidator_SetIncludes(self.ptr(), list, count) }
    }
    fn clone(&self) -> Validator {
        unsafe { Validator::from(wxTextValidator_Clone(self.ptr())) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxTextValidator_GetStyle(self.ptr()) }
    }
    fn onChar<T: EventMethods>(&self, event: &T) {
        unsafe { wxTextValidator_OnChar(self.ptr(), event.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.ptr(), style) }
    }
}

/// Wraps the wxWidgets' [wxTimer](http://docs.wxwidgets.org/3.0/classwx_timer.html) class.
pub struct Timer { ptr: *mut c_void }
impl TimerMethods for Timer {}
impl ObjectMethods for Timer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Timer {
    pub fn from(ptr: *mut c_void) -> Timer { Timer { ptr: ptr } }
    pub fn null() -> Timer { Timer::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int) -> Timer {
        unsafe { Timer::from(wxTimer_Create(_prt.ptr(), _id)) }
    }
}

/// Methods of the wxWidgets' [wxTimer](http://docs.wxwidgets.org/3.0/classwx_timer.html) class.
pub trait TimerMethods : ObjectMethods {
    fn getInterval(&self) -> c_int {
        unsafe { wxTimer_GetInterval(self.ptr()) }
    }
    fn isOneShot(&self) -> c_int {
        unsafe { wxTimer_IsOneShot(self.ptr()) }
    }
    fn isRuning(&self) -> c_int {
        unsafe { wxTimer_IsRuning(self.ptr()) }
    }
    fn start(&self, _int: c_int, _one: c_int) -> c_int {
        unsafe { wxTimer_Start(self.ptr(), _int, _one) }
    }
    fn stop(&self) {
        unsafe { wxTimer_Stop(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTimerEvent](http://docs.wxwidgets.org/3.0/classwx_timer_event.html) class.
pub struct TimerEvent { ptr: *mut c_void }
impl TimerEventMethods for TimerEvent {}
impl EventMethods for TimerEvent {}
impl ObjectMethods for TimerEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerEvent {
    pub fn from(ptr: *mut c_void) -> TimerEvent { TimerEvent { ptr: ptr } }
    pub fn null() -> TimerEvent { TimerEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTimerEvent](http://docs.wxwidgets.org/3.0/classwx_timer_event.html) class.
pub trait TimerEventMethods : EventMethods {
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTimerEx](http://docs.wxwidgets.org/3.0/classwx_timer_ex.html) class.
pub struct TimerEx { ptr: *mut c_void }
impl TimerExMethods for TimerEx {}
impl TimerMethods for TimerEx {}
impl ObjectMethods for TimerEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerEx {
    pub fn from(ptr: *mut c_void) -> TimerEx { TimerEx { ptr: ptr } }
    pub fn null() -> TimerEx { TimerEx::from(0 as *mut c_void) }
    
    pub fn new() -> TimerEx {
        unsafe { TimerEx::from(wxTimerEx_Create()) }
    }
}

/// Methods of the wxWidgets' [wxTimerEx](http://docs.wxwidgets.org/3.0/classwx_timer_ex.html) class.
pub trait TimerExMethods : TimerMethods {
    fn connect<T: ClosureMethods>(&self, closure: &T) {
        unsafe { wxTimerEx_Connect(self.ptr(), closure.ptr()) }
    }
    fn getClosure(&self) -> Closure {
        unsafe { Closure::from(wxTimerEx_GetClosure(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxTimerRunner](http://docs.wxwidgets.org/3.0/classwx_timer_runner.html) class.
pub struct TimerRunner { ptr: *mut c_void }
impl TimerRunnerMethods for TimerRunner { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerRunner {
    pub fn from(ptr: *mut c_void) -> TimerRunner { TimerRunner { ptr: ptr } }
    pub fn null() -> TimerRunner { TimerRunner::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTimerRunner](http://docs.wxwidgets.org/3.0/classwx_timer_runner.html) class.
pub trait TimerRunnerMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxTipWindow](http://docs.wxwidgets.org/3.0/classwx_tip_window.html) class.
pub struct TipWindow { ptr: *mut c_void }
impl TipWindowMethods for TipWindow {}
impl PopupTransientWindowMethods for TipWindow {}
impl PopupWindowMethods for TipWindow {}
impl WindowMethods for TipWindow {}
impl EvtHandlerMethods for TipWindow {}
impl ObjectMethods for TipWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TipWindow {
    pub fn from(ptr: *mut c_void) -> TipWindow { TipWindow { ptr: ptr } }
    pub fn null() -> TipWindow { TipWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(parent: &T, text: &str, maxLength: c_int) -> TipWindow {
        let text = strToString(text);
        unsafe { TipWindow::from(wxTipWindow_Create(parent.ptr(), text.ptr(), maxLength)) }
    }
}

/// Methods of the wxWidgets' [wxTipWindow](http://docs.wxwidgets.org/3.0/classwx_tip_window.html) class.
pub trait TipWindowMethods : PopupTransientWindowMethods {
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.ptr(), x, y, w, h) }
    }
    fn setTipWindowPtr(&self, windowPtr: *mut c_void) {
        unsafe { wxTipWindow_SetTipWindowPtr(self.ptr(), windowPtr) }
    }
}

/// Wraps the wxWidgets' [wxToggleButton](http://docs.wxwidgets.org/3.0/classwx_toggle_button.html) class.
pub struct ToggleButton { ptr: *mut c_void }
impl ToggleButtonMethods for ToggleButton {}
impl ControlMethods for ToggleButton {}
impl WindowMethods for ToggleButton {}
impl EvtHandlerMethods for ToggleButton {}
impl ObjectMethods for ToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToggleButton {
    pub fn from(ptr: *mut c_void) -> ToggleButton { ToggleButton { ptr: ptr } }
    pub fn null() -> ToggleButton { ToggleButton::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(parent: &T, id: c_int, label: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> ToggleButton {
        let label = strToString(label);
        unsafe { ToggleButton::from(wxToggleButton_Create(parent.ptr(), id, label.ptr(), x, y, w, h, style)) }
    }
}

/// Methods of the wxWidgets' [wxToggleButton](http://docs.wxwidgets.org/3.0/classwx_toggle_button.html) class.
pub trait ToggleButtonMethods : ControlMethods {
    fn getValue(&self) -> c_int {
        unsafe { wxToggleButton_GetValue(self.ptr()) }
    }
    fn setValue(&self, state: c_int) {
        unsafe { wxToggleButton_SetValue(self.ptr(), state) }
    }
}

/// Wraps the wxWidgets' [wxToolBar](http://docs.wxwidgets.org/3.0/classwx_tool_bar.html) class.
pub struct ToolBar { ptr: *mut c_void }
impl ToolBarMethods for ToolBar {}
impl ToolBarBaseMethods for ToolBar {}
impl ControlMethods for ToolBar {}
impl WindowMethods for ToolBar {}
impl EvtHandlerMethods for ToolBar {}
impl ObjectMethods for ToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolBar {
    pub fn from(ptr: *mut c_void) -> ToolBar { ToolBar { ptr: ptr } }
    pub fn null() -> ToolBar { ToolBar::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ToolBar {
        unsafe { ToolBar::from(wxToolBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxToolBar](http://docs.wxwidgets.org/3.0/classwx_tool_bar.html) class.
pub trait ToolBarMethods : ToolBarBaseMethods {
    fn addControl<T: ControlMethods>(&self, ctrl: &T) -> c_int {
        unsafe { wxToolBar_AddControl(self.ptr(), ctrl.ptr()) }
    }
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.ptr()) }
    }
    fn addTool<T: BitmapMethods>(&self, id: c_int, bmp: &T, shelp: &str, lhelp: &str) {
        let shelp = strToString(shelp);
        let lhelp = strToString(lhelp);
        unsafe { wxToolBar_AddTool(self.ptr(), id, bmp.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    fn addToolEx<T: BitmapMethods, U: BitmapMethods, V: ObjectMethods>(&self, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, x: c_int, y: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = strToString(shelp);
        let lhelp = strToString(lhelp);
        unsafe { wxToolBar_AddToolEx(self.ptr(), id, bmp1.ptr(), bmp2.ptr(), isToggle, x, y, data.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    fn deleteTool(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_DeleteTool(self.ptr(), id) }
    }
    fn deleteToolByPos(&self, pos: c_int) -> c_int {
        unsafe { wxToolBar_DeleteToolByPos(self.ptr(), pos) }
    }
    fn enableTool(&self, id: c_int, enable: c_int) {
        unsafe { wxToolBar_EnableTool(self.ptr(), id, enable) }
    }
    fn getMargins(&self) -> Point {
        unsafe { Point::from(wxToolBar_GetMargins(self.ptr())) }
    }
    fn getToolBitmapSize(&self) -> Size {
        unsafe { Size::from(wxToolBar_GetToolBitmapSize(self.ptr())) }
    }
    fn getToolClientData(&self, id: c_int) -> Object {
        unsafe { Object::from(wxToolBar_GetToolClientData(self.ptr(), id)) }
    }
    fn getToolEnabled(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolEnabled(self.ptr(), id) }
    }
    fn getToolLongHelp(&self, id: c_int) -> ~str {
        unsafe { String::from(wxToolBar_GetToolLongHelp(self.ptr(), id)).to_str() }
    }
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.ptr()) }
    }
    fn getToolShortHelp(&self, id: c_int) -> ~str {
        unsafe { String::from(wxToolBar_GetToolShortHelp(self.ptr(), id)).to_str() }
    }
    fn getToolSize(&self) -> Size {
        unsafe { Size::from(wxToolBar_GetToolSize(self.ptr())) }
    }
    fn getToolState(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolState(self.ptr(), id) }
    }
    fn insertControl<T: ControlMethods>(&self, pos: c_int, ctrl: &T) {
        unsafe { wxToolBar_InsertControl(self.ptr(), pos, ctrl.ptr()) }
    }
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.ptr(), pos) }
    }
    fn insertTool<T: BitmapMethods, U: BitmapMethods, V: ObjectMethods>(&self, pos: c_int, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = strToString(shelp);
        let lhelp = strToString(lhelp);
        unsafe { wxToolBar_InsertTool(self.ptr(), pos, id, bmp1.ptr(), bmp2.ptr(), isToggle, data.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    fn realize(&self) -> c_int {
        unsafe { wxToolBar_Realize(self.ptr()) }
    }
    fn removeTool(&self, id: c_int) {
        unsafe { wxToolBar_RemoveTool(self.ptr(), id) }
    }
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetMargins(self.ptr(), x, y) }
    }
    fn setToolBitmapSize(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetToolBitmapSize(self.ptr(), x, y) }
    }
    fn setToolClientData<T: ObjectMethods>(&self, id: c_int, data: &T) {
        unsafe { wxToolBar_SetToolClientData(self.ptr(), id, data.ptr()) }
    }
    fn setToolLongHelp(&self, id: c_int, str: &str) {
        let str = strToString(str);
        unsafe { wxToolBar_SetToolLongHelp(self.ptr(), id, str.ptr()) }
    }
    fn setToolPacking(&self, packing: c_int) {
        unsafe { wxToolBar_SetToolPacking(self.ptr(), packing) }
    }
    fn setToolSeparation(&self, separation: c_int) {
        unsafe { wxToolBar_SetToolSeparation(self.ptr(), separation) }
    }
    fn setToolShortHelp(&self, id: c_int, str: &str) {
        let str = strToString(str);
        unsafe { wxToolBar_SetToolShortHelp(self.ptr(), id, str.ptr()) }
    }
    fn toggleTool(&self, id: c_int, toggle: c_int) {
        unsafe { wxToolBar_ToggleTool(self.ptr(), id, toggle) }
    }
    fn addTool2<T: BitmapMethods, U: BitmapMethods>(&self, toolId: c_int, label: &str, bmp: &T, bmpDisabled: &U, itemKind: c_int, shortHelp: &str, longHelp: &str) {
        let label = strToString(label);
        let shortHelp = strToString(shortHelp);
        let longHelp = strToString(longHelp);
        unsafe { wxToolBar_AddTool2(self.ptr(), toolId, label.ptr(), bmp.ptr(), bmpDisabled.ptr(), itemKind, shortHelp.ptr(), longHelp.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxToolBarBase](http://docs.wxwidgets.org/3.0/classwx_tool_bar_base.html) class.
pub struct ToolBarBase { ptr: *mut c_void }
impl ToolBarBaseMethods for ToolBarBase {}
impl ControlMethods for ToolBarBase {}
impl WindowMethods for ToolBarBase {}
impl EvtHandlerMethods for ToolBarBase {}
impl ObjectMethods for ToolBarBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolBarBase {
    pub fn from(ptr: *mut c_void) -> ToolBarBase { ToolBarBase { ptr: ptr } }
    pub fn null() -> ToolBarBase { ToolBarBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxToolBarBase](http://docs.wxwidgets.org/3.0/classwx_tool_bar_base.html) class.
pub trait ToolBarBaseMethods : ControlMethods {
}

/// Wraps the wxWidgets' [wxToolTip](http://docs.wxwidgets.org/3.0/classwx_tool_tip.html) class.
pub struct ToolTip { ptr: *mut c_void }
impl ToolTipMethods for ToolTip {}
impl ObjectMethods for ToolTip { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolTip {
    pub fn from(ptr: *mut c_void) -> ToolTip { ToolTip { ptr: ptr } }
    pub fn null() -> ToolTip { ToolTip::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxToolTip](http://docs.wxwidgets.org/3.0/classwx_tool_tip.html) class.
pub trait ToolTipMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxTopLevelWindow](http://docs.wxwidgets.org/3.0/classwx_top_level_window.html) class.
pub struct TopLevelWindow { ptr: *mut c_void }
impl TopLevelWindowMethods for TopLevelWindow {}
impl WindowMethods for TopLevelWindow {}
impl EvtHandlerMethods for TopLevelWindow {}
impl ObjectMethods for TopLevelWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TopLevelWindow {
    pub fn from(ptr: *mut c_void) -> TopLevelWindow { TopLevelWindow { ptr: ptr } }
    pub fn null() -> TopLevelWindow { TopLevelWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTopLevelWindow](http://docs.wxwidgets.org/3.0/classwx_top_level_window.html) class.
pub trait TopLevelWindowMethods : WindowMethods {
    fn enableCloseButton(&self, enable: c_int) -> c_int {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.ptr(), enable) }
    }
    fn getDefaultButton(&self) -> Button {
        unsafe { Button::from(wxTopLevelWindow_GetDefaultButton(self.ptr())) }
    }
    fn getDefaultItem(&self) -> Window {
        unsafe { Window::from(wxTopLevelWindow_GetDefaultItem(self.ptr())) }
    }
    fn getIcon(&self) -> Icon {
        unsafe { Icon::from(wxTopLevelWindow_GetIcon(self.ptr())) }
    }
    fn getTitle(&self) -> ~str {
        unsafe { String::from(wxTopLevelWindow_GetTitle(self.ptr())).to_str() }
    }
    fn iconize(&self, iconize: c_int) -> c_int {
        unsafe { wxTopLevelWindow_Iconize(self.ptr(), iconize) }
    }
    fn isActive(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsActive(self.ptr()) }
    }
    fn isIconized(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsIconized(self.ptr()) }
    }
    fn isMaximized(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsMaximized(self.ptr()) }
    }
    fn maximize(&self, maximize: c_int) {
        unsafe { wxTopLevelWindow_Maximize(self.ptr(), maximize) }
    }
    fn requestUserAttention(&self, flags: c_int) {
        unsafe { wxTopLevelWindow_RequestUserAttention(self.ptr(), flags) }
    }
    fn setDefaultButton<T: ButtonMethods>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.ptr(), pBut.ptr()) }
    }
    fn setDefaultItem<T: WindowMethods>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.ptr(), pBut.ptr()) }
    }
    fn setIcon<T: IconMethods>(&self, pIcon: &T) {
        unsafe { wxTopLevelWindow_SetIcon(self.ptr(), pIcon.ptr()) }
    }
    fn setIcons(&self, _icons: *mut c_void) {
        unsafe { wxTopLevelWindow_SetIcons(self.ptr(), _icons) }
    }
    fn setMaxSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMaxSize(self.ptr(), w, h) }
    }
    fn setMinSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMinSize(self.ptr(), w, h) }
    }
    fn setTitle(&self, pString: &str) {
        let pString = strToString(pString);
        unsafe { wxTopLevelWindow_SetTitle(self.ptr(), pString.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTreeCtrl](http://docs.wxwidgets.org/3.0/classwx_tree_ctrl.html) class.
pub struct TreeCtrl { ptr: *mut c_void }
impl TreeCtrlMethods for TreeCtrl {}
impl ControlMethods for TreeCtrl {}
impl WindowMethods for TreeCtrl {}
impl EvtHandlerMethods for TreeCtrl {}
impl ObjectMethods for TreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeCtrl {
    pub fn from(ptr: *mut c_void) -> TreeCtrl { TreeCtrl { ptr: ptr } }
    pub fn null() -> TreeCtrl { TreeCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_obj: *mut c_void, _cmp: *mut c_void, _prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> TreeCtrl {
        unsafe { TreeCtrl::from(wxTreeCtrl_Create(_obj, _cmp, _prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
    pub fn new2<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> TreeCtrl {
        unsafe { TreeCtrl::from(wxTreeCtrl_Create2(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxTreeCtrl](http://docs.wxwidgets.org/3.0/classwx_tree_ctrl.html) class.
pub trait TreeCtrlMethods : ControlMethods {
    fn addRoot<T: TreeItemDataMethods, U: TreeItemIdMethods>(&self, text: &str, image: c_int, selectedImage: c_int, data: &T, _item: &U) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_AddRoot(self.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    fn appendItem<T: TreeItemIdMethods, U: TreeItemDataMethods, V: TreeItemIdMethods>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: &U, _item: &V) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_AppendItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    fn collapse<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_Collapse(self.ptr(), item.ptr()) }
    }
    fn collapseAndReset<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.ptr(), item.ptr()) }
    }
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.ptr()) }
    }
    fn deleteChildren<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_DeleteChildren(self.ptr(), item.ptr()) }
    }
    fn editLabel<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_EditLabel(self.ptr(), item.ptr()) }
    }
    fn endEditLabel<T: TreeItemIdMethods>(&self, item: &T, discardChanges: c_int) {
        unsafe { wxTreeCtrl_EndEditLabel(self.ptr(), item.ptr(), discardChanges) }
    }
    fn ensureVisible<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_EnsureVisible(self.ptr(), item.ptr()) }
    }
    fn expand<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_Expand(self.ptr(), item.ptr()) }
    }
    fn getBoundingRect<T: TreeItemIdMethods>(&self, item: &T, textOnly: c_int) -> Rect {
        unsafe { Rect::from(wxTreeCtrl_GetBoundingRect(self.ptr(), item.ptr(), textOnly)) }
    }
    fn getChildrenCount<T: TreeItemIdMethods>(&self, item: &T, recursively: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.ptr(), item.ptr(), recursively) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.ptr()) }
    }
    fn getEditControl(&self) -> TextCtrl {
        unsafe { TextCtrl::from(wxTreeCtrl_GetEditControl(self.ptr())) }
    }
    fn getFirstChild<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    fn getFirstVisibleItem<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getImageList(&self) -> ImageList {
        unsafe { ImageList::from(wxTreeCtrl_GetImageList(self.ptr())) }
    }
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.ptr()) }
    }
    fn getItemData<T: TreeItemIdMethods>(&self, item: &T) -> *mut c_void {
        unsafe { wxTreeCtrl_GetItemData(self.ptr(), item.ptr()) }
    }
    fn getItemImage<T: TreeItemIdMethods>(&self, item: &T, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.ptr(), item.ptr(), which) }
    }
    fn getItemText<T: TreeItemIdMethods>(&self, item: &T) -> ~str {
        unsafe { String::from(wxTreeCtrl_GetItemText(self.ptr(), item.ptr())).to_str() }
    }
    fn getLastChild<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetLastChild(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getNextChild<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetNextChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    fn getNextSibling<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getNextVisible<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getPrevSibling<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getPrevVisible<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getRootItem<T: TreeItemIdMethods>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetRootItem(self.ptr(), _item.ptr()) }
    }
    fn getSelection<T: TreeItemIdMethods>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetSelection(self.ptr(), _item.ptr()) }
    }
    fn getSelections(&self, selections: *mut c_void) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self.ptr(), selections) }
    }
    fn getSpacing(&self) -> c_int {
        unsafe { wxTreeCtrl_GetSpacing(self.ptr()) }
    }
    fn getStateImageList(&self) -> ImageList {
        unsafe { ImageList::from(wxTreeCtrl_GetStateImageList(self.ptr())) }
    }
    fn hitTest<T: TreeItemIdMethods>(&self, _x: c_int, _y: c_int, flags: *mut c_int, _item: &T) {
        unsafe { wxTreeCtrl_HitTest(self.ptr(), _x, _y, flags, _item.ptr()) }
    }
    fn insertItem<T: TreeItemIdMethods, U: TreeItemIdMethods, V: TreeItemIdMethods>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &V) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_InsertItem(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn insertItemByIndex<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_InsertItemByIndex(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn isBold<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsBold(self.ptr(), item.ptr()) }
    }
    fn isExpanded<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsExpanded(self.ptr(), item.ptr()) }
    }
    fn isSelected<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsSelected(self.ptr(), item.ptr()) }
    }
    fn isVisible<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsVisible(self.ptr(), item.ptr()) }
    }
    fn itemHasChildren<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.ptr(), item.ptr()) }
    }
    fn onCompareItems<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, item1: &T, item2: &U) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.ptr(), item1.ptr(), item2.ptr()) }
    }
    fn prependItem<T: TreeItemIdMethods, U: TreeItemIdMethods>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_PrependItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn scrollTo<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_ScrollTo(self.ptr(), item.ptr()) }
    }
    fn selectItem<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_SelectItem(self.ptr(), item.ptr()) }
    }
    fn setImageList<T: ImageListMethods>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetImageList(self.ptr(), imageList.ptr()) }
    }
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.ptr(), indent) }
    }
    fn setItemBackgroundColour<T: TreeItemIdMethods, U: ColourMethods>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    fn setItemBold<T: TreeItemIdMethods>(&self, item: &T, bold: c_int) {
        unsafe { wxTreeCtrl_SetItemBold(self.ptr(), item.ptr(), bold) }
    }
    fn setItemData<T: TreeItemIdMethods>(&self, item: &T, data: *mut c_void) {
        unsafe { wxTreeCtrl_SetItemData(self.ptr(), item.ptr(), data) }
    }
    fn setItemDropHighlight<T: TreeItemIdMethods>(&self, item: &T, highlight: c_int) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.ptr(), item.ptr(), highlight) }
    }
    fn setItemFont<T: TreeItemIdMethods, U: FontMethods>(&self, item: &T, font: &U) {
        unsafe { wxTreeCtrl_SetItemFont(self.ptr(), item.ptr(), font.ptr()) }
    }
    fn setItemHasChildren<T: TreeItemIdMethods>(&self, item: &T, hasChildren: c_int) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.ptr(), item.ptr(), hasChildren) }
    }
    fn setItemImage<T: TreeItemIdMethods>(&self, item: &T, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.ptr(), item.ptr(), image, which) }
    }
    fn setItemText<T: TreeItemIdMethods>(&self, item: &T, text: &str) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_SetItemText(self.ptr(), item.ptr(), text.ptr()) }
    }
    fn setItemTextColour<T: TreeItemIdMethods, U: ColourMethods>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.ptr(), spacing) }
    }
    fn setStateImageList<T: ImageListMethods>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetStateImageList(self.ptr(), imageList.ptr()) }
    }
    fn sortChildren<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_SortChildren(self.ptr(), item.ptr()) }
    }
    fn toggle<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe { wxTreeCtrl_Toggle(self.ptr(), item.ptr()) }
    }
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self.ptr()) }
    }
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self.ptr()) }
    }
    fn insertItem2<T: WindowMethods, U: TreeItemIdMethods, V: ClosureMethods, W: TreeItemIdMethods>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, closure: &V, _item: &W) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_InsertItem2(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    fn insertItemByIndex2<T: WindowMethods, U: ClosureMethods, V: TreeItemIdMethods>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, closure: &U, _item: &V) {
        let text = strToString(text);
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    fn getItemClientClosure<T: TreeItemIdMethods>(&self, item: &T) -> Closure {
        unsafe { Closure::from(wxTreeCtrl_GetItemClientClosure(self.ptr(), item.ptr())) }
    }
    fn setItemClientClosure<T: TreeItemIdMethods, U: ClosureMethods>(&self, item: &T, closure: &U) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.ptr(), item.ptr(), closure.ptr()) }
    }
    fn assignImageList<T: ImageListMethods>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignImageList(self.ptr(), imageList.ptr()) }
    }
    fn assignStateImageList<T: ImageListMethods>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.ptr(), imageList.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTreeEvent](http://docs.wxwidgets.org/3.0/classwx_tree_event.html) class.
pub struct TreeEvent { ptr: *mut c_void }
impl TreeEventMethods for TreeEvent {}
impl NotifyEventMethods for TreeEvent {}
impl CommandEventMethods for TreeEvent {}
impl EventMethods for TreeEvent {}
impl ObjectMethods for TreeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeEvent {
    pub fn from(ptr: *mut c_void) -> TreeEvent { TreeEvent { ptr: ptr } }
    pub fn null() -> TreeEvent { TreeEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeEvent](http://docs.wxwidgets.org/3.0/classwx_tree_event.html) class.
pub trait TreeEventMethods : NotifyEventMethods {
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.ptr()) }
    }
    fn getItem<T: TreeItemIdMethods>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { String::from(wxTreeEvent_GetLabel(self.ptr())).to_str() }
    }
    fn getOldItem<T: TreeItemIdMethods>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetOldItem(self.ptr(), _ref.ptr()) }
    }
    fn getPoint(&self) -> Point {
        unsafe { Point::from(wxTreeEvent_GetPoint(self.ptr())) }
    }
    fn getKeyEvent(&self) -> KeyEvent {
        unsafe { KeyEvent::from(wxTreeEvent_GetKeyEvent(self.ptr())) }
    }
    fn isEditCancelled(&self) -> c_int {
        unsafe { wxTreeEvent_IsEditCancelled(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxTreeItemData](http://docs.wxwidgets.org/3.0/classwx_tree_item_data.html) class.
/// Rather use the wxRust-specific [CTreeItemData](struct.CTreeItemData.html) class.
pub struct TreeItemData { ptr: *mut c_void }
impl TreeItemDataMethods for TreeItemData {}
impl ClientDataMethods for TreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeItemData {
    pub fn from(ptr: *mut c_void) -> TreeItemData { TreeItemData { ptr: ptr } }
    pub fn null() -> TreeItemData { TreeItemData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeItemData](http://docs.wxwidgets.org/3.0/classwx_tree_item_data.html) class.
pub trait TreeItemDataMethods : ClientDataMethods {
}

/// Wraps the wxWidgets' [wxTreeItemId](http://docs.wxwidgets.org/3.0/classwx_tree_item_id.html) class.
pub struct TreeItemId { ptr: *mut c_void }
impl TreeItemIdMethods for TreeItemId { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeItemId {
    pub fn from(ptr: *mut c_void) -> TreeItemId { TreeItemId { ptr: ptr } }
    pub fn null() -> TreeItemId { TreeItemId::from(0 as *mut c_void) }
    
    pub fn new() -> TreeItemId {
        unsafe { TreeItemId::from(wxTreeItemId_Create()) }
    }
    pub fn newFromValue(value: intptr_t) -> TreeItemId {
        unsafe { TreeItemId::from(wxTreeItemId_CreateFromValue(value)) }
    }
}

/// Methods of the wxWidgets' [wxTreeItemId](http://docs.wxwidgets.org/3.0/classwx_tree_item_id.html) class.
pub trait TreeItemIdMethods {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTreeItemId_IsOk(self.ptr()) }
    }
    fn clone(&self) -> TreeItemId {
        unsafe { TreeItemId::from(wxTreeItemId_Clone(self.ptr())) }
    }
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxUpdateUIEvent](http://docs.wxwidgets.org/3.0/classwx_update_uie_vent.html) class.
pub struct UpdateUIEvent { ptr: *mut c_void }
impl UpdateUIEventMethods for UpdateUIEvent {}
impl EventMethods for UpdateUIEvent {}
impl ObjectMethods for UpdateUIEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl UpdateUIEvent {
    pub fn from(ptr: *mut c_void) -> UpdateUIEvent { UpdateUIEvent { ptr: ptr } }
    pub fn null() -> UpdateUIEvent { UpdateUIEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxUpdateUIEvent](http://docs.wxwidgets.org/3.0/classwx_update_uie_vent.html) class.
pub trait UpdateUIEventMethods : EventMethods {
    fn check(&self, check: c_int) {
        unsafe { wxUpdateUIEvent_Check(self.ptr(), check) }
    }
    fn enable(&self, enable: c_int) {
        unsafe { wxUpdateUIEvent_Enable(self.ptr(), enable) }
    }
    fn getChecked(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetChecked(self.ptr()) }
    }
    fn getEnabled(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetEnabled(self.ptr()) }
    }
    fn getSetChecked(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetChecked(self.ptr()) }
    }
    fn getSetEnabled(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetEnabled(self.ptr()) }
    }
    fn getSetText(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetText(self.ptr()) }
    }
    fn getText(&self) -> ~str {
        unsafe { String::from(wxUpdateUIEvent_GetText(self.ptr())).to_str() }
    }
    fn setText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxUpdateUIEvent_SetText(self.ptr(), text.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxValidator](http://docs.wxwidgets.org/3.0/classwx_validator.html) class.
pub struct Validator { ptr: *mut c_void }
impl ValidatorMethods for Validator {}
impl EvtHandlerMethods for Validator {}
impl ObjectMethods for Validator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Validator {
    pub fn from(ptr: *mut c_void) -> Validator { Validator { ptr: ptr } }
    pub fn null() -> Validator { Validator::from(0 as *mut c_void) }
    
    pub fn new() -> Validator {
        unsafe { Validator::from(wxValidator_Create()) }
    }
    pub fn setBellOnError(doIt: c_int) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

/// Methods of the wxWidgets' [wxValidator](http://docs.wxwidgets.org/3.0/classwx_validator.html) class.
pub trait ValidatorMethods : EvtHandlerMethods {
    fn getWindow(&self) -> Window {
        unsafe { Window::from(wxValidator_GetWindow(self.ptr())) }
    }
    fn setWindow<T: WindowMethods>(&self, win: &T) {
        unsafe { wxValidator_SetWindow(self.ptr(), win.ptr()) }
    }
    fn transferFromWindow(&self) -> c_int {
        unsafe { wxValidator_TransferFromWindow(self.ptr()) }
    }
    fn transferToWindow(&self) -> c_int {
        unsafe { wxValidator_TransferToWindow(self.ptr()) }
    }
    fn validate<T: WindowMethods>(&self, parent: &T) -> c_int {
        unsafe { wxValidator_Validate(self.ptr(), parent.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxView](http://docs.wxwidgets.org/3.0/classwx_view.html) class.
pub struct View { ptr: *mut c_void }
impl ViewMethods for View {}
impl EvtHandlerMethods for View {}
impl ObjectMethods for View { fn ptr(&self) -> *mut c_void { self.ptr } }

impl View {
    pub fn from(ptr: *mut c_void) -> View { View { ptr: ptr } }
    pub fn null() -> View { View::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxView](http://docs.wxwidgets.org/3.0/classwx_view.html) class.
pub trait ViewMethods : EvtHandlerMethods {
}

/// Wraps the wxWidgets' [wxSound](http://docs.wxwidgets.org/3.0/classwx_sound.html) class.
pub struct Sound { ptr: *mut c_void }
impl SoundMethods for Sound {}
impl EvtHandlerMethods for Sound {}
impl ObjectMethods for Sound { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Sound {
    pub fn from(ptr: *mut c_void) -> Sound { Sound { ptr: ptr } }
    pub fn null() -> Sound { Sound::from(0 as *mut c_void) }
    
    pub fn new(fileName: &str, isResource: c_int) -> Sound {
        let fileName = strToString(fileName);
        unsafe { Sound::from(wxSound_Create(fileName.ptr(), isResource)) }
    }
}

/// Methods of the wxWidgets' [wxSound](http://docs.wxwidgets.org/3.0/classwx_sound.html) class.
pub trait SoundMethods : EvtHandlerMethods {
    fn isOk(&self) -> c_int {
        unsafe { wxSound_IsOk(self.ptr()) }
    }
    fn play(&self, flag: c_int) -> c_int {
        unsafe { wxSound_Play(self.ptr(), flag) }
    }
    fn stop(&self) {
        unsafe { wxSound_Stop(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxWindow](http://docs.wxwidgets.org/3.0/classwx_window.html) class.
pub struct Window { ptr: *mut c_void }
impl WindowMethods for Window {}
impl EvtHandlerMethods for Window {}
impl ObjectMethods for Window { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Window {
    pub fn from(ptr: *mut c_void) -> Window { Window { ptr: ptr } }
    pub fn null() -> Window { Window::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> Window {
        unsafe { Window::from(wxWindow_Create(_prt.ptr(), _id, _x, _y, _w, _h, _stl)) }
    }
}

/// Methods of the wxWidgets' [wxWindow](http://docs.wxwidgets.org/3.0/classwx_window.html) class.
pub trait WindowMethods : EvtHandlerMethods {
    fn addChild<T: WindowMethods>(&self, child: &T) {
        unsafe { wxWindow_AddChild(self.ptr(), child.ptr()) }
    }
    fn addConstraintReference<T: WindowMethods>(&self, otherWin: &T) {
        unsafe { wxWindow_AddConstraintReference(self.ptr(), otherWin.ptr()) }
    }
    fn captureMouse(&self) {
        unsafe { wxWindow_CaptureMouse(self.ptr()) }
    }
    fn center(&self, direction: c_int) {
        unsafe { wxWindow_Center(self.ptr(), direction) }
    }
    fn centerOnParent(&self, dir: c_int) {
        unsafe { wxWindow_CenterOnParent(self.ptr(), dir) }
    }
    fn clearBackground(&self) {
        unsafe { wxWindow_ClearBackground(self.ptr()) }
    }
    fn clientToScreen(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from(wxWindow_ClientToScreen(self.ptr(), x, y)) }
    }
    fn close(&self, _force: c_int) -> c_int {
        unsafe { wxWindow_Close(self.ptr(), _force) }
    }
    fn convertDialogToPixels(&self) -> Point {
        unsafe { Point::from(wxWindow_ConvertDialogToPixels(self.ptr())) }
    }
    fn convertPixelsToDialog(&self) -> Point {
        unsafe { Point::from(wxWindow_ConvertPixelsToDialog(self.ptr())) }
    }
    fn deleteRelatedConstraints(&self) {
        unsafe { wxWindow_DeleteRelatedConstraints(self.ptr()) }
    }
    fn destroy(&self) -> c_int {
        unsafe { wxWindow_Destroy(self.ptr()) }
    }
    fn destroyChildren(&self) -> c_int {
        unsafe { wxWindow_DestroyChildren(self.ptr()) }
    }
    fn disable(&self) -> c_int {
        unsafe { wxWindow_Disable(self.ptr()) }
    }
    fn doPhase(&self, phase: c_int) -> c_int {
        unsafe { wxWindow_DoPhase(self.ptr(), phase) }
    }
    fn enable(&self) -> c_int {
        unsafe { wxWindow_Enable(self.ptr()) }
    }
    fn findFocus(&self) -> Window {
        unsafe { Window::from(wxWindow_FindFocus(self.ptr())) }
    }
    fn findWindow(&self, name: &str) -> Window {
        let name = strToString(name);
        unsafe { Window::from(wxWindow_FindWindow(self.ptr(), name.ptr())) }
    }
    fn fit(&self) {
        unsafe { wxWindow_Fit(self.ptr()) }
    }
    fn fitInside(&self) {
        unsafe { wxWindow_FitInside(self.ptr()) }
    }
    fn freeze(&self) {
        unsafe { wxWindow_Freeze(self.ptr()) }
    }
    fn getEffectiveMinSize(&self) -> Size {
        unsafe { Size::from(wxWindow_GetEffectiveMinSize(self.ptr())) }
    }
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.ptr()) }
    }
    fn getBackgroundColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxWindow_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getBestSize(&self) -> Size {
        unsafe { Size::from(wxWindow_GetBestSize(self.ptr())) }
    }
    fn getCaret(&self) -> Caret {
        unsafe { Caret::from(wxWindow_GetCaret(self.ptr())) }
    }
    fn getCharHeight(&self) -> c_int {
        unsafe { wxWindow_GetCharHeight(self.ptr()) }
    }
    fn getCharWidth(&self) -> c_int {
        unsafe { wxWindow_GetCharWidth(self.ptr()) }
    }
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxWindow_GetChildren(self.ptr(), _res, _cnt) }
    }
    fn getClientData(&self) -> ClientData {
        unsafe { ClientData::from(wxWindow_GetClientData(self.ptr())) }
    }
    fn getClientSize(&self) -> Size {
        unsafe { Size::from(wxWindow_GetClientSize(self.ptr())) }
    }
    fn getClientSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.ptr(), _w, _h) }
    }
    fn getConstraints(&self) -> LayoutConstraints {
        unsafe { LayoutConstraints::from(wxWindow_GetConstraints(self.ptr())) }
    }
    fn getConstraintsInvolvedIn(&self) -> *mut c_void {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.ptr()) }
    }
    fn getCursor(&self) -> Cursor {
        unsafe { Cursor::from(wxWindow_GetCursor(self.ptr())) }
    }
    fn getDropTarget(&self) -> DropTarget {
        unsafe { DropTarget::from(wxWindow_GetDropTarget(self.ptr())) }
    }
    fn getEventHandler(&self) -> EvtHandler {
        unsafe { EvtHandler::from(wxWindow_GetEventHandler(self.ptr())) }
    }
    fn getFont<T: FontMethods>(&self, _ref: &T) {
        unsafe { wxWindow_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getForegroundColour<T: ColourMethods>(&self, _ref: &T) {
        unsafe { wxWindow_GetForegroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getHandle(&self) -> *mut c_void {
        unsafe { wxWindow_GetHandle(self.ptr()) }
    }
    fn getId(&self) -> c_int {
        unsafe { wxWindow_GetId(self.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { String::from(wxWindow_GetLabel(self.ptr())).to_str() }
    }
    fn getLabelEmpty(&self) -> c_int {
        unsafe { wxWindow_GetLabelEmpty(self.ptr()) }
    }
    fn getMaxHeight(&self) -> c_int {
        unsafe { wxWindow_GetMaxHeight(self.ptr()) }
    }
    fn getMaxWidth(&self) -> c_int {
        unsafe { wxWindow_GetMaxWidth(self.ptr()) }
    }
    fn getMinHeight(&self) -> c_int {
        unsafe { wxWindow_GetMinHeight(self.ptr()) }
    }
    fn getMinWidth(&self) -> c_int {
        unsafe { wxWindow_GetMinWidth(self.ptr()) }
    }
    fn getName(&self) -> ~str {
        unsafe { String::from(wxWindow_GetName(self.ptr())).to_str() }
    }
    fn getParent(&self) -> Window {
        unsafe { Window::from(wxWindow_GetParent(self.ptr())) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point::from(wxWindow_GetPosition(self.ptr())) }
    }
    fn getPositionConstraint(&self, _x: *mut c_int, _y: *mut c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.ptr(), _x, _y) }
    }
    fn getRect(&self) -> Rect {
        unsafe { Rect::from(wxWindow_GetRect(self.ptr())) }
    }
    fn getScrollPos(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollPos(self.ptr(), orient) }
    }
    fn getScrollRange(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollRange(self.ptr(), orient) }
    }
    fn getScrollThumb(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollThumb(self.ptr(), orient) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size::from(wxWindow_GetSize(self.ptr())) }
    }
    fn getSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.ptr(), _w, _h) }
    }
    fn getSizer(&self) -> Sizer {
        unsafe { Sizer::from(wxWindow_GetSizer(self.ptr())) }
    }
    fn getTextExtent<T: FontMethods>(&self, string: &str, x: *mut c_int, y: *mut c_int, descent: *mut c_int, externalLeading: *mut c_int, theFont: &T) {
        let string = strToString(string);
        unsafe { wxWindow_GetTextExtent(self.ptr(), string.ptr(), x, y, descent, externalLeading, theFont.ptr()) }
    }
    fn getToolTip(&self) -> ~str {
        unsafe { String::from(wxWindow_GetToolTip(self.ptr())).to_str() }
    }
    fn getUpdateRegion(&self) -> Region {
        unsafe { Region::from(wxWindow_GetUpdateRegion(self.ptr())) }
    }
    fn getValidator(&self) -> Validator {
        unsafe { Validator::from(wxWindow_GetValidator(self.ptr())) }
    }
    fn getVirtualSize(&self) -> Size {
        unsafe { Size::from(wxWindow_GetVirtualSize(self.ptr())) }
    }
    fn getWindowStyleFlag(&self) -> c_int {
        unsafe { wxWindow_GetWindowStyleFlag(self.ptr()) }
    }
    fn hasFlag(&self, flag: c_int) -> c_int {
        unsafe { wxWindow_HasFlag(self.ptr(), flag) }
    }
    fn hide(&self) -> c_int {
        unsafe { wxWindow_Hide(self.ptr()) }
    }
    fn initDialog(&self) {
        unsafe { wxWindow_InitDialog(self.ptr()) }
    }
    fn isBeingDeleted(&self) -> c_int {
        unsafe { wxWindow_IsBeingDeleted(self.ptr()) }
    }
    fn isEnabled(&self) -> c_int {
        unsafe { wxWindow_IsEnabled(self.ptr()) }
    }
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxWindow_IsExposed(self.ptr(), x, y, w, h) }
    }
    fn isShown(&self) -> c_int {
        unsafe { wxWindow_IsShown(self.ptr()) }
    }
    fn isTopLevel(&self) -> c_int {
        unsafe { wxWindow_IsTopLevel(self.ptr()) }
    }
    fn layout(&self) -> c_int {
        unsafe { wxWindow_Layout(self.ptr()) }
    }
    fn layoutPhase1(&self, noChanges: *mut c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase1(self.ptr(), noChanges) }
    }
    fn layoutPhase2(&self, noChanges: *mut c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase2(self.ptr(), noChanges) }
    }
    fn lower(&self) {
        unsafe { wxWindow_Lower(self.ptr()) }
    }
    fn makeModal(&self, modal: c_int) {
        unsafe { wxWindow_MakeModal(self.ptr(), modal) }
    }
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_Move(self.ptr(), x, y) }
    }
    fn moveConstraint(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_MoveConstraint(self.ptr(), x, y) }
    }
    fn popEventHandler(&self, deleteHandler: c_int) -> *mut c_void {
        unsafe { wxWindow_PopEventHandler(self.ptr(), deleteHandler) }
    }
    fn popupMenu<T: MenuMethods>(&self, menu: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.ptr(), menu.ptr(), x, y) }
    }
    fn prepareDC<T: DCMethods>(&self, dc: &T) {
        unsafe { wxWindow_PrepareDC(self.ptr(), dc.ptr()) }
    }
    fn pushEventHandler<T: EvtHandlerMethods>(&self, handler: &T) {
        unsafe { wxWindow_PushEventHandler(self.ptr(), handler.ptr()) }
    }
    fn raise(&self) {
        unsafe { wxWindow_Raise(self.ptr()) }
    }
    fn refresh(&self, eraseBackground: c_int) {
        unsafe { wxWindow_Refresh(self.ptr(), eraseBackground) }
    }
    fn refreshRect(&self, eraseBackground: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_RefreshRect(self.ptr(), eraseBackground, x, y, w, h) }
    }
    fn releaseMouse(&self) {
        unsafe { wxWindow_ReleaseMouse(self.ptr()) }
    }
    fn removeChild<T: WindowMethods>(&self, child: &T) {
        unsafe { wxWindow_RemoveChild(self.ptr(), child.ptr()) }
    }
    fn removeConstraintReference<T: WindowMethods>(&self, otherWin: &T) {
        unsafe { wxWindow_RemoveConstraintReference(self.ptr(), otherWin.ptr()) }
    }
    fn reparent<T: WindowMethods>(&self, _par: &T) -> c_int {
        unsafe { wxWindow_Reparent(self.ptr(), _par.ptr()) }
    }
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.ptr()) }
    }
    fn screenToClient(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from(wxWindow_ScreenToClient(self.ptr(), x, y)) }
    }
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self.ptr(), dx, dy) }
    }
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self.ptr(), dx, dy, x, y, w, h) }
    }
    fn setAcceleratorTable<T: AcceleratorTableMethods>(&self, accel: &T) {
        unsafe { wxWindow_SetAcceleratorTable(self.ptr(), accel.ptr()) }
    }
    fn setAutoLayout(&self, autoLayout: c_int) {
        unsafe { wxWindow_SetAutoLayout(self.ptr(), autoLayout) }
    }
    fn setBackgroundColour<T: ColourMethods>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setCaret<T: CaretMethods>(&self, caret: &T) {
        unsafe { wxWindow_SetCaret(self.ptr(), caret.ptr()) }
    }
    fn setClientData<T: ClientDataMethods>(&self, data: &T) {
        unsafe { wxWindow_SetClientData(self.ptr(), data.ptr()) }
    }
    fn setClientObject<T: ClientDataMethods>(&self, data: &T) {
        unsafe { wxWindow_SetClientObject(self.ptr(), data.ptr()) }
    }
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self.ptr(), width, height) }
    }
    fn setConstraintSizes(&self, recurse: c_int) {
        unsafe { wxWindow_SetConstraintSizes(self.ptr(), recurse) }
    }
    fn setConstraints<T: LayoutConstraintsMethods>(&self, constraints: &T) {
        unsafe { wxWindow_SetConstraints(self.ptr(), constraints.ptr()) }
    }
    fn setCursor<T: CursorMethods>(&self, cursor: &T) -> c_int {
        unsafe { wxWindow_SetCursor(self.ptr(), cursor.ptr()) }
    }
    fn setDropTarget<T: DropTargetMethods>(&self, dropTarget: &T) {
        unsafe { wxWindow_SetDropTarget(self.ptr(), dropTarget.ptr()) }
    }
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self.ptr(), exStyle) }
    }
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self.ptr()) }
    }
    fn setFont<T: FontMethods>(&self, font: &T) -> c_int {
        unsafe { wxWindow_SetFont(self.ptr(), font.ptr()) }
    }
    fn setForegroundColour<T: ColourMethods>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.ptr(), colour.ptr()) }
    }
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.ptr(), _id) }
    }
    fn setLabel(&self, _title: &str) {
        let _title = strToString(_title);
        unsafe { wxWindow_SetLabel(self.ptr(), _title.ptr()) }
    }
    fn setName(&self, _name: &str) {
        let _name = strToString(_name);
        unsafe { wxWindow_SetName(self.ptr(), _name.ptr()) }
    }
    fn setScrollPos(&self, orient: c_int, pos: c_int, refresh: c_int) {
        unsafe { wxWindow_SetScrollPos(self.ptr(), orient, pos, refresh) }
    }
    fn setScrollbar(&self, orient: c_int, pos: c_int, thumbVisible: c_int, range: c_int, refresh: c_int) {
        unsafe { wxWindow_SetScrollbar(self.ptr(), orient, pos, thumbVisible, range, refresh) }
    }
    fn setSize(&self, x: c_int, y: c_int, width: c_int, height: c_int, sizeFlags: c_int) {
        unsafe { wxWindow_SetSize(self.ptr(), x, y, width, height, sizeFlags) }
    }
    fn setSizeConstraint(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_SetSizeConstraint(self.ptr(), x, y, w, h) }
    }
    fn setSizeHints(&self, minW: c_int, minH: c_int, maxW: c_int, maxH: c_int, incW: c_int, incH: c_int) {
        unsafe { wxWindow_SetSizeHints(self.ptr(), minW, minH, maxW, maxH, incW, incH) }
    }
    fn setSizer<T: SizerMethods>(&self, sizer: &T) {
        unsafe { wxWindow_SetSizer(self.ptr(), sizer.ptr()) }
    }
    fn setToolTip(&self, tip: &str) {
        let tip = strToString(tip);
        unsafe { wxWindow_SetToolTip(self.ptr(), tip.ptr()) }
    }
    fn setValidator<T: ValidatorMethods>(&self, validator: &T) {
        unsafe { wxWindow_SetValidator(self.ptr(), validator.ptr()) }
    }
    fn setWindowStyleFlag(&self, style: c_long) {
        unsafe { wxWindow_SetWindowStyleFlag(self.ptr(), style) }
    }
    fn show(&self) -> c_int {
        unsafe { wxWindow_Show(self.ptr()) }
    }
    fn thaw(&self) {
        unsafe { wxWindow_Thaw(self.ptr()) }
    }
    fn transferDataFromWindow(&self) -> c_int {
        unsafe { wxWindow_TransferDataFromWindow(self.ptr()) }
    }
    fn transferDataToWindow(&self) -> c_int {
        unsafe { wxWindow_TransferDataToWindow(self.ptr()) }
    }
    fn unsetConstraints(&self, c: *mut c_void) {
        unsafe { wxWindow_UnsetConstraints(self.ptr(), c) }
    }
    fn updateWindowUI(&self) {
        unsafe { wxWindow_UpdateWindowUI(self.ptr()) }
    }
    fn validate(&self) -> c_int {
        unsafe { wxWindow_Validate(self.ptr()) }
    }
    fn setVirtualSize(&self, w: c_int, h: c_int) {
        unsafe { wxWindow_SetVirtualSize(self.ptr(), w, h) }
    }
    fn warpPointer(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_WarpPointer(self.ptr(), x, y) }
    }
    fn convertDialogToPixelsEx(&self) -> Point {
        unsafe { Point::from(wxWindow_ConvertDialogToPixelsEx(self.ptr())) }
    }
    fn convertPixelsToDialogEx(&self) -> Point {
        unsafe { Point::from(wxWindow_ConvertPixelsToDialogEx(self.ptr())) }
    }
    fn screenToClient2(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from(wxWindow_ScreenToClient2(self.ptr(), x, y)) }
    }
}

/// Wraps the wxWidgets' [wxWindowCreateEvent](http://docs.wxwidgets.org/3.0/classwx_window_create_event.html) class.
pub struct WindowCreateEvent { ptr: *mut c_void }
impl WindowCreateEventMethods for WindowCreateEvent {}
impl CommandEventMethods for WindowCreateEvent {}
impl EventMethods for WindowCreateEvent {}
impl ObjectMethods for WindowCreateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowCreateEvent {
    pub fn from(ptr: *mut c_void) -> WindowCreateEvent { WindowCreateEvent { ptr: ptr } }
    pub fn null() -> WindowCreateEvent { WindowCreateEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxWindowCreateEvent](http://docs.wxwidgets.org/3.0/classwx_window_create_event.html) class.
pub trait WindowCreateEventMethods : CommandEventMethods {
    fn getWindow(&self) -> Window {
        unsafe { Window::from(wxWindowCreateEvent_GetWindow(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxWindowDC](http://docs.wxwidgets.org/3.0/classwx_window_dc.html) class.
pub struct WindowDC { ptr: *mut c_void }
impl WindowDCMethods for WindowDC {}
impl DCMethods for WindowDC {}
impl ObjectMethods for WindowDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowDC {
    pub fn from(ptr: *mut c_void) -> WindowDC { WindowDC { ptr: ptr } }
    pub fn null() -> WindowDC { WindowDC::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(win: &T) -> WindowDC {
        unsafe { WindowDC::from(wxWindowDC_Create(win.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxWindowDC](http://docs.wxwidgets.org/3.0/classwx_window_dc.html) class.
pub trait WindowDCMethods : DCMethods {
}

/// Wraps the wxWidgets' [wxWindowDestroyEvent](http://docs.wxwidgets.org/3.0/classwx_window_destroy_event.html) class.
pub struct WindowDestroyEvent { ptr: *mut c_void }
impl WindowDestroyEventMethods for WindowDestroyEvent {}
impl CommandEventMethods for WindowDestroyEvent {}
impl EventMethods for WindowDestroyEvent {}
impl ObjectMethods for WindowDestroyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowDestroyEvent {
    pub fn from(ptr: *mut c_void) -> WindowDestroyEvent { WindowDestroyEvent { ptr: ptr } }
    pub fn null() -> WindowDestroyEvent { WindowDestroyEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxWindowDestroyEvent](http://docs.wxwidgets.org/3.0/classwx_window_destroy_event.html) class.
pub trait WindowDestroyEventMethods : CommandEventMethods {
    fn getWindow(&self) -> Window {
        unsafe { Window::from(wxWindowDestroyEvent_GetWindow(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxWindowDisabler](http://docs.wxwidgets.org/3.0/classwx_window_disabler.html) class.
pub struct WindowDisabler { ptr: *mut c_void }
impl WindowDisablerMethods for WindowDisabler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowDisabler {
    pub fn from(ptr: *mut c_void) -> WindowDisabler { WindowDisabler { ptr: ptr } }
    pub fn null() -> WindowDisabler { WindowDisabler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxWindowDisabler](http://docs.wxwidgets.org/3.0/classwx_window_disabler.html) class.
pub trait WindowDisablerMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxXmlResourceHandler](http://docs.wxwidgets.org/3.0/classwx_xml_resource_handler.html) class.
pub struct XmlResourceHandler { ptr: *mut c_void }
impl XmlResourceHandlerMethods for XmlResourceHandler {}
impl ObjectMethods for XmlResourceHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl XmlResourceHandler {
    pub fn from(ptr: *mut c_void) -> XmlResourceHandler { XmlResourceHandler { ptr: ptr } }
    pub fn null() -> XmlResourceHandler { XmlResourceHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxXmlResourceHandler](http://docs.wxwidgets.org/3.0/classwx_xml_resource_handler.html) class.
pub trait XmlResourceHandlerMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxGenericDragImage](http://docs.wxwidgets.org/3.0/classwx_generic_drag_image.html) class.
pub struct GenericDragImage { ptr: *mut c_void }
impl GenericDragImageMethods for GenericDragImage {}
impl DragImageMethods for GenericDragImage {}
impl ObjectMethods for GenericDragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GenericDragImage {
    pub fn from(ptr: *mut c_void) -> GenericDragImage { GenericDragImage { ptr: ptr } }
    pub fn null() -> GenericDragImage { GenericDragImage::from(0 as *mut c_void) }
    
    pub fn new<T: CursorMethods>(cursor: &T) -> GenericDragImage {
        unsafe { GenericDragImage::from(wxGenericDragImage_Create(cursor.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxGenericDragImage](http://docs.wxwidgets.org/3.0/classwx_generic_drag_image.html) class.
pub trait GenericDragImageMethods : DragImageMethods {
    fn doDrawImage<T: DCMethods>(&self, dc: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxGenericDragImage_DoDrawImage(self.ptr(), dc.ptr(), x, y) }
    }
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> Rect {
        unsafe { Rect::from(wxGenericDragImage_GetImageRect(self.ptr(), x_pos, y_pos)) }
    }
    fn updateBackingFromWindow<T: DCMethods, U: MemoryDCMethods>(&self, windowDC: &T, destDC: &U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.ptr(), windowDC.ptr(), destDC.ptr(), x, y, w, h, xdest, ydest, width, height) }
    }
}

/// Wraps the wxWidgets' [wxGraphicsObject](http://docs.wxwidgets.org/3.0/classwx_graphics_object.html) class.
pub struct GraphicsObject { ptr: *mut c_void }
impl GraphicsObjectMethods for GraphicsObject {}
impl ObjectMethods for GraphicsObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsObject {
    pub fn from(ptr: *mut c_void) -> GraphicsObject { GraphicsObject { ptr: ptr } }
    pub fn null() -> GraphicsObject { GraphicsObject::from(0 as *mut c_void) }
    
    pub fn getRenderer() -> GraphicsRenderer {
        unsafe { GraphicsRenderer::from(wxGraphicsObject_GetRenderer()) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsObject](http://docs.wxwidgets.org/3.0/classwx_graphics_object.html) class.
pub trait GraphicsObjectMethods : ObjectMethods {
    fn isNull(&self) -> c_int {
        unsafe { wxGraphicsObject_IsNull(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxGraphicsBrush](http://docs.wxwidgets.org/3.0/classwx_graphics_brush.html) class.
pub struct GraphicsBrush { ptr: *mut c_void }
impl GraphicsBrushMethods for GraphicsBrush {}
impl GraphicsObjectMethods for GraphicsBrush {}
impl ObjectMethods for GraphicsBrush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsBrush {
    pub fn from(ptr: *mut c_void) -> GraphicsBrush { GraphicsBrush { ptr: ptr } }
    pub fn null() -> GraphicsBrush { GraphicsBrush::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsBrush {
        unsafe { GraphicsBrush::from(wxGraphicsBrush_Create()) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsBrush](http://docs.wxwidgets.org/3.0/classwx_graphics_brush.html) class.
pub trait GraphicsBrushMethods : GraphicsObjectMethods {
}

/// Wraps the wxWidgets' [wxGraphicsContext](http://docs.wxwidgets.org/3.0/classwx_graphics_context.html) class.
pub struct GraphicsContext { ptr: *mut c_void }
impl GraphicsContextMethods for GraphicsContext {}
impl GraphicsObjectMethods for GraphicsContext {}
impl ObjectMethods for GraphicsContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsContext {
    pub fn from(ptr: *mut c_void) -> GraphicsContext { GraphicsContext { ptr: ptr } }
    pub fn null() -> GraphicsContext { GraphicsContext::from(0 as *mut c_void) }
    
    pub fn new<T: WindowDCMethods>(dc: &T) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsContext_Create(dc.ptr())) }
    }
    pub fn newFromWindow<T: WindowMethods>(window: &T) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsContext_CreateFromWindow(window.ptr())) }
    }
    pub fn newFromNative(context: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsContext_CreateFromNative(context)) }
    }
    pub fn newFromNativeWindow(window: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsContext_CreateFromNativeWindow(window)) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsContext](http://docs.wxwidgets.org/3.0/classwx_graphics_context.html) class.
pub trait GraphicsContextMethods : GraphicsObjectMethods {
    fn clip<T: RegionMethods>(&self, region: &T) {
        unsafe { wxGraphicsContext_Clip(self.ptr(), region.ptr()) }
    }
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self.ptr(), x, y, w, h) }
    }
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self.ptr()) }
    }
    fn drawBitmap<T: BitmapMethods>(&self, bmp: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.ptr(), bmp.ptr(), x, y, w, h) }
    }
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.ptr(), x, y, w, h) }
    }
    fn drawIcon<T: IconMethods>(&self, icon: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.ptr(), icon.ptr(), x, y, w, h) }
    }
    fn drawLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.ptr(), n, x, y, style) }
    }
    fn drawPath<T: GraphicsPathMethods>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_DrawPath(self.ptr(), path.ptr(), style) }
    }
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawRectangle(self.ptr(), x, y, w, h) }
    }
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawRoundedRectangle(self.ptr(), x, y, w, h, radius) }
    }
    fn drawText(&self, text: &str, x: c_double, y: c_double) {
        let text = strToString(text);
        unsafe { wxGraphicsContext_DrawText(self.ptr(), text.ptr(), x, y) }
    }
    fn drawTextWithAngle(&self, text: &str, x: c_double, y: c_double, radius: c_double) {
        let text = strToString(text);
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.ptr(), text.ptr(), x, y, radius) }
    }
    fn fillPath<T: GraphicsPathMethods>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.ptr(), path.ptr(), style) }
    }
    fn strokePath<T: GraphicsPathMethods>(&self, path: &T) {
        unsafe { wxGraphicsContext_StrokePath(self.ptr(), path.ptr()) }
    }
    fn getNativeContext(&self) -> *mut c_void {
        unsafe { wxGraphicsContext_GetNativeContext(self.ptr()) }
    }
    fn getTextExtent(&self, text: &str, width: *mut c_double, height: *mut c_double, descent: *mut c_double, externalLeading: *mut c_double) {
        let text = strToString(text);
        unsafe { wxGraphicsContext_GetTextExtent(self.ptr(), text.ptr(), width, height, descent, externalLeading) }
    }
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsContext_Rotate(self.ptr(), angle) }
    }
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsContext_Scale(self.ptr(), xScale, yScale) }
    }
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsContext_Translate(self.ptr(), dx, dy) }
    }
    fn setTransform<T: GraphicsMatrixMethods>(&self, path: &T) {
        unsafe { wxGraphicsContext_SetTransform(self.ptr(), path.ptr()) }
    }
    fn concatTransform<T: GraphicsMatrixMethods>(&self, path: &T) {
        unsafe { wxGraphicsContext_ConcatTransform(self.ptr(), path.ptr()) }
    }
    fn setBrush<T: BrushMethods>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetBrush(self.ptr(), brush.ptr()) }
    }
    fn setGraphicsBrush<T: GraphicsBrushMethods>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.ptr(), brush.ptr()) }
    }
    fn setFont<T: FontMethods, U: ColourMethods>(&self, font: &T, colour: &U) {
        unsafe { wxGraphicsContext_SetFont(self.ptr(), font.ptr(), colour.ptr()) }
    }
    fn setGraphicsFont<T: GraphicsFontMethods>(&self, font: &T) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.ptr(), font.ptr()) }
    }
    fn setPen<T: PenMethods>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetPen(self.ptr(), pen.ptr()) }
    }
    fn setGraphicsPen<T: GraphicsPenMethods>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self.ptr(), pen.ptr()) }
    }
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self.ptr(), x1, y1, x2, y2) }
    }
    fn strokeLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.ptr(), n, x, y, style) }
    }
}

/// Wraps the wxWidgets' [wxGraphicsFont](http://docs.wxwidgets.org/3.0/classwx_graphics_font.html) class.
pub struct GraphicsFont { ptr: *mut c_void }
impl GraphicsFontMethods for GraphicsFont {}
impl GraphicsObjectMethods for GraphicsFont {}
impl ObjectMethods for GraphicsFont { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsFont {
    pub fn from(ptr: *mut c_void) -> GraphicsFont { GraphicsFont { ptr: ptr } }
    pub fn null() -> GraphicsFont { GraphicsFont::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsFont {
        unsafe { GraphicsFont::from(wxGraphicsFont_Create()) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsFont](http://docs.wxwidgets.org/3.0/classwx_graphics_font.html) class.
pub trait GraphicsFontMethods : GraphicsObjectMethods {
}

/// Wraps the wxWidgets' [wxGraphicsMatrix](http://docs.wxwidgets.org/3.0/classwx_graphics_matrix.html) class.
pub struct GraphicsMatrix { ptr: *mut c_void }
impl GraphicsMatrixMethods for GraphicsMatrix {}
impl GraphicsObjectMethods for GraphicsMatrix {}
impl ObjectMethods for GraphicsMatrix { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsMatrix {
    pub fn from(ptr: *mut c_void) -> GraphicsMatrix { GraphicsMatrix { ptr: ptr } }
    pub fn null() -> GraphicsMatrix { GraphicsMatrix::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsMatrix {
        unsafe { GraphicsMatrix::from(wxGraphicsMatrix_Create()) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsMatrix](http://docs.wxwidgets.org/3.0/classwx_graphics_matrix.html) class.
pub trait GraphicsMatrixMethods : GraphicsObjectMethods {
    fn concat<T: GraphicsMatrixMethods>(&self, t: &T) {
        unsafe { wxGraphicsMatrix_Concat(self.ptr(), t.ptr()) }
    }
    fn get(&self, a: *mut c_double, b: *mut c_double, c: *mut c_double, d: *mut c_double, tx: *mut c_double, ty: *mut c_double) {
        unsafe { wxGraphicsMatrix_Get(self.ptr(), a, b, c, d, tx, ty) }
    }
    fn getNativeMatrix(&self) -> *mut c_void {
        unsafe { wxGraphicsMatrix_GetNativeMatrix(self.ptr()) }
    }
    fn invert(&self) {
        unsafe { wxGraphicsMatrix_Invert(self.ptr()) }
    }
    fn isEqual<T: GraphicsMatrixMethods>(&self, t: &T) -> c_int {
        unsafe { wxGraphicsMatrix_IsEqual(self.ptr(), t.ptr()) }
    }
    fn isIdentity(&self) -> c_int {
        unsafe { wxGraphicsMatrix_IsIdentity(self.ptr()) }
    }
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsMatrix_Rotate(self.ptr(), angle) }
    }
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsMatrix_Scale(self.ptr(), xScale, yScale) }
    }
    fn set(&self, a: c_double, b: c_double, c: c_double, d: c_double, tx: c_double, ty: c_double) {
        unsafe { wxGraphicsMatrix_Set(self.ptr(), a, b, c, d, tx, ty) }
    }
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsMatrix_Translate(self.ptr(), dx, dy) }
    }
    fn transformPoint(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxGraphicsMatrix_TransformPoint(self.ptr(), x, y) }
    }
    fn transformDistance(&self, dx: *mut c_double, dy: *mut c_double) {
        unsafe { wxGraphicsMatrix_TransformDistance(self.ptr(), dx, dy) }
    }
}

/// Wraps the wxWidgets' [wxGraphicsPath](http://docs.wxwidgets.org/3.0/classwx_graphics_path.html) class.
pub struct GraphicsPath { ptr: *mut c_void }
impl GraphicsPathMethods for GraphicsPath {}
impl GraphicsObjectMethods for GraphicsPath {}
impl ObjectMethods for GraphicsPath { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsPath {
    pub fn from(ptr: *mut c_void) -> GraphicsPath { GraphicsPath { ptr: ptr } }
    pub fn null() -> GraphicsPath { GraphicsPath::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsPath {
        unsafe { GraphicsPath::from(wxGraphicsPath_Create()) }
    }
    pub fn unGetNativePath(p: *mut c_void) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsPath](http://docs.wxwidgets.org/3.0/classwx_graphics_path.html) class.
pub trait GraphicsPathMethods : GraphicsObjectMethods {
    fn moveToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_MoveToPoint(self.ptr(), x, y) }
    }
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: c_int) {
        unsafe { wxGraphicsPath_AddArc(self.ptr(), x, y, r, startAngle, endAngle, clockwise) }
    }
    fn addArcToPoint(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddArcToPoint(self.ptr(), x1, y1, x2, y2, r) }
    }
    fn addCircle(&self, x: c_double, y: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddCircle(self.ptr(), x, y, r) }
    }
    fn addCurveToPoint(&self, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddCurveToPoint(self.ptr(), cx1, cy1, cx2, cy2, x, y) }
    }
    fn addEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddEllipse(self.ptr(), x, y, w, h) }
    }
    fn addLineToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddLineToPoint(self.ptr(), x, y) }
    }
    fn addPath<T: GraphicsPathMethods>(&self, x: c_double, y: c_double, path: &T) {
        unsafe { wxGraphicsPath_AddPath(self.ptr(), x, y, path.ptr()) }
    }
    fn addQuadCurveToPoint(&self, cx: c_double, cy: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddQuadCurveToPoint(self.ptr(), cx, cy, x, y) }
    }
    fn addRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddRectangle(self.ptr(), x, y, w, h) }
    }
    fn addRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsPath_AddRoundedRectangle(self.ptr(), x, y, w, h, radius) }
    }
    fn closeSubpath(&self) {
        unsafe { wxGraphicsPath_CloseSubpath(self.ptr()) }
    }
    fn contains(&self, x: c_double, y: c_double, style: c_int) {
        unsafe { wxGraphicsPath_Contains(self.ptr(), x, y, style) }
    }
    fn getBox(&self, x: *mut c_double, y: *mut c_double, w: *mut c_double, h: *mut c_double) {
        unsafe { wxGraphicsPath_GetBox(self.ptr(), x, y, w, h) }
    }
    fn getCurrentPoint(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxGraphicsPath_GetCurrentPoint(self.ptr(), x, y) }
    }
    fn transform<T: GraphicsMatrixMethods>(&self, matrix: &T) {
        unsafe { wxGraphicsPath_Transform(self.ptr(), matrix.ptr()) }
    }
    fn getNativePath(&self) -> *mut c_void {
        unsafe { wxGraphicsPath_GetNativePath(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxGraphicsPen](http://docs.wxwidgets.org/3.0/classwx_graphics_pen.html) class.
pub struct GraphicsPen { ptr: *mut c_void }
impl GraphicsPenMethods for GraphicsPen {}
impl GraphicsObjectMethods for GraphicsPen {}
impl ObjectMethods for GraphicsPen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsPen {
    pub fn from(ptr: *mut c_void) -> GraphicsPen { GraphicsPen { ptr: ptr } }
    pub fn null() -> GraphicsPen { GraphicsPen::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsPen {
        unsafe { GraphicsPen::from(wxGraphicsPen_Create()) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsPen](http://docs.wxwidgets.org/3.0/classwx_graphics_pen.html) class.
pub trait GraphicsPenMethods : GraphicsObjectMethods {
}

/// Wraps the wxWidgets' [wxGraphicsRenderer](http://docs.wxwidgets.org/3.0/classwx_graphics_renderer.html) class.
pub struct GraphicsRenderer { ptr: *mut c_void }
impl GraphicsRendererMethods for GraphicsRenderer {}
impl GraphicsObjectMethods for GraphicsRenderer {}
impl ObjectMethods for GraphicsRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsRenderer {
    pub fn from(ptr: *mut c_void) -> GraphicsRenderer { GraphicsRenderer { ptr: ptr } }
    pub fn null() -> GraphicsRenderer { GraphicsRenderer::from(0 as *mut c_void) }
    
    pub fn newContext<T: WindowDCMethods>(dc: &T) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsRenderer_CreateContext(dc.ptr())) }
    }
    pub fn newContextFromWindow<T: WindowMethods>(window: &T) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsRenderer_CreateContextFromWindow(window.ptr())) }
    }
    pub fn newContextFromNativeContext(context: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsRenderer_CreateContextFromNativeContext(context)) }
    }
    pub fn newContextFromNativeWindow(window: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext::from(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) }
    }
}

/// Methods of the wxWidgets' [wxGraphicsRenderer](http://docs.wxwidgets.org/3.0/classwx_graphics_renderer.html) class.
pub trait GraphicsRendererMethods : GraphicsObjectMethods {
    fn getDefaultRenderer(&self) -> GraphicsRenderer {
        unsafe { GraphicsRenderer::from(wxGraphicsRenderer_GetDefaultRenderer(self.ptr())) }
    }
}

/// The wxRust-specific derived class of [wxPrintout](http://docs.wxwidgets.org/3.0/classwx_printout.html).
pub struct RustPrintout { ptr: *mut c_void }
impl RustPrintoutMethods for RustPrintout {}
impl PrintoutMethods for RustPrintout {}
impl ObjectMethods for RustPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPrintout {
    pub fn from(ptr: *mut c_void) -> RustPrintout { RustPrintout { ptr: ptr } }
    pub fn null() -> RustPrintout { RustPrintout::from(0 as *mut c_void) }
    
    pub fn new(title: &str) -> RustPrintout {
        let title = strToString(title);
        unsafe { RustPrintout::from(wxcPrintout_Create(title.ptr())) }
    }
}

/// Methods of the wxRust-specific derived class of [wxPrintout](http://docs.wxwidgets.org/3.0/classwx_printout.html).
pub trait RustPrintoutMethods : PrintoutMethods {
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.ptr(), startPage, endPage, fromPage, toPage) }
    }
    fn getEvtHandler(&self) -> RustPrintoutHandler {
        unsafe { RustPrintoutHandler::from(wxcPrintout_GetEvtHandler(self.ptr())) }
    }
}

/// The wxRust-specific derived class of [wxEvent](http://docs.wxwidgets.org/3.0/classwx_event.html).
pub struct RustPrintEvent { ptr: *mut c_void }
impl RustPrintEventMethods for RustPrintEvent {}
impl EventMethods for RustPrintEvent {}
impl ObjectMethods for RustPrintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPrintEvent {
    pub fn from(ptr: *mut c_void) -> RustPrintEvent { RustPrintEvent { ptr: ptr } }
    pub fn null() -> RustPrintEvent { RustPrintEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxEvent](http://docs.wxwidgets.org/3.0/classwx_event.html).
pub trait RustPrintEventMethods : EventMethods {
    fn getPrintout(&self) -> RustPrintout {
        unsafe { RustPrintout::from(wxcPrintEvent_GetPrintout(self.ptr())) }
    }
    fn getPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetPage(self.ptr()) }
    }
    fn getEndPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetEndPage(self.ptr()) }
    }
    fn getContinue(&self) -> c_int {
        unsafe { wxcPrintEvent_GetContinue(self.ptr()) }
    }
    fn setContinue(&self, cont: c_int) {
        unsafe { wxcPrintEvent_SetContinue(self.ptr(), cont) }
    }
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintEvent_SetPageLimits(self.ptr(), startPage, endPage, fromPage, toPage) }
    }
}

/// The wxRust-specific derived class of [wxEvtHandler](http://docs.wxwidgets.org/3.0/classwx_evt_handler.html).
pub struct RustPrintoutHandler { ptr: *mut c_void }
impl RustPrintoutHandlerMethods for RustPrintoutHandler {}
impl EvtHandlerMethods for RustPrintoutHandler {}
impl ObjectMethods for RustPrintoutHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPrintoutHandler {
    pub fn from(ptr: *mut c_void) -> RustPrintoutHandler { RustPrintoutHandler { ptr: ptr } }
    pub fn null() -> RustPrintoutHandler { RustPrintoutHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxEvtHandler](http://docs.wxwidgets.org/3.0/classwx_evt_handler.html).
pub trait RustPrintoutHandlerMethods : EvtHandlerMethods {
}

/// The wxRust-specific derived class of [wxTreeItemData](http://docs.wxwidgets.org/3.0/classwx_tree_item_data.html).
pub struct RustTreeItemData { ptr: *mut c_void }
impl RustTreeItemDataMethods for RustTreeItemData {}
impl TreeItemDataMethods for RustTreeItemData {}
impl ClientDataMethods for RustTreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustTreeItemData {
    pub fn from(ptr: *mut c_void) -> RustTreeItemData { RustTreeItemData { ptr: ptr } }
    pub fn null() -> RustTreeItemData { RustTreeItemData::from(0 as *mut c_void) }
    
    pub fn new<T: ClosureMethods>(closure: &T) -> RustTreeItemData {
        unsafe { RustTreeItemData::from(wxcTreeItemData_Create(closure.ptr())) }
    }
}

/// Methods of the wxRust-specific derived class of [wxTreeItemData](http://docs.wxwidgets.org/3.0/classwx_tree_item_data.html).
pub trait RustTreeItemDataMethods : TreeItemDataMethods {
    fn getClientClosure(&self) -> Closure {
        unsafe { Closure::from(wxcTreeItemData_GetClientClosure(self.ptr())) }
    }
    fn setClientClosure<T: ClosureMethods>(&self, closure: &T) {
        unsafe { wxcTreeItemData_SetClientClosure(self.ptr(), closure.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxInputSink](http://docs.wxwidgets.org/3.0/classwx_input_sink.html) class.
pub struct InputSink { ptr: *mut c_void }
impl InputSinkMethods for InputSink {}
impl ThreadMethods for InputSink { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InputSink {
    pub fn from(ptr: *mut c_void) -> InputSink { InputSink { ptr: ptr } }
    pub fn null() -> InputSink { InputSink::from(0 as *mut c_void) }
    
    pub fn new<T: InputStreamMethods, U: EvtHandlerMethods>(input: &T, evtHandler: &U, bufferLen: c_int) -> InputSink {
        unsafe { InputSink::from(wxInputSink_Create(input.ptr(), evtHandler.ptr(), bufferLen)) }
    }
}

/// Methods of the wxWidgets' [wxInputSink](http://docs.wxwidgets.org/3.0/classwx_input_sink.html) class.
pub trait InputSinkMethods : ThreadMethods {
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.ptr()) }
    }
    fn start(&self) {
        unsafe { wxInputSink_Start(self.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxInputSinkEvent](http://docs.wxwidgets.org/3.0/classwx_input_sink_event.html) class.
pub struct InputSinkEvent { ptr: *mut c_void }
impl InputSinkEventMethods for InputSinkEvent {}
impl EventMethods for InputSinkEvent {}
impl ObjectMethods for InputSinkEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InputSinkEvent {
    pub fn from(ptr: *mut c_void) -> InputSinkEvent { InputSinkEvent { ptr: ptr } }
    pub fn null() -> InputSinkEvent { InputSinkEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxInputSinkEvent](http://docs.wxwidgets.org/3.0/classwx_input_sink_event.html) class.
pub trait InputSinkEventMethods : EventMethods {
    fn lastError(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastError(self.ptr()) }
    }
    fn lastRead(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastRead(self.ptr()) }
    }
    fn lastInput(&self) -> *mut c_char {
        unsafe { wxInputSinkEvent_LastInput(self.ptr()) }
    }
}

