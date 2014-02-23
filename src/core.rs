use std::libc::*;
use _unsafe::*;
use base::*;

pub struct WxrApp { ptr: *mut c_void }
impl TWxrApp for WxrApp {}
impl TWxApp for WxrApp {}
impl TWxEvtHandler for WxrApp {}
impl TWxObject for WxrApp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrApp {
    pub fn from(ptr: *mut c_void) -> WxrApp { WxrApp { ptr: ptr } }
    pub fn null() -> WxrApp { WxrApp::from(0 as *mut c_void) }
    
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    pub fn newLogTarget() -> WxrLog {
        unsafe { WxrLog { ptr: ELJApp_CreateLogTarget() } }
    }
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    pub fn displaySize() -> WxSize {
        unsafe { WxSize { ptr: ELJApp_DisplaySize() } }
    }
    pub fn enableTooltips(_enable: c_int) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    pub fn enableTopLevelWindows(_enb: c_int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    pub fn executeProcess<T: TWxProcess>(_cmd: &str, _snc: c_int, _prc: &T) -> c_int {
        let _cmd = wxT(_cmd);
        unsafe { ELJApp_ExecuteProcess(_cmd.ptr(), _snc, _prc.ptr()) }
    }
    pub fn exit() {
        unsafe { ELJApp_Exit() }
    }
    pub fn exitMainLoop() {
        unsafe { ELJApp_ExitMainLoop() }
    }
    pub fn findWindowById<T: TWxWindow>(_id: c_int, _prt: &T) -> *mut c_void {
        unsafe { ELJApp_FindWindowById(_id, _prt.ptr()) }
    }
    pub fn findWindowByLabel<T: TWxWindow>(_lbl: &str, _prt: &T) -> WxWindow {
        let _lbl = wxT(_lbl);
        unsafe { WxWindow { ptr: ELJApp_FindWindowByLabel(_lbl.ptr(), _prt.ptr()) } }
    }
    pub fn findWindowByName<T: TWxWindow>(_lbl: &str, _prt: &T) -> WxWindow {
        let _lbl = wxT(_lbl);
        unsafe { WxWindow { ptr: ELJApp_FindWindowByName(_lbl.ptr(), _prt.ptr()) } }
    }
    pub fn getApp() -> WxApp {
        unsafe { WxApp { ptr: ELJApp_GetApp() } }
    }
    pub fn getAppName() -> ~str {
        unsafe { WxString { ptr: ELJApp_GetAppName() }.to_str() }
    }
    pub fn getClassName() -> ~str {
        unsafe { WxString { ptr: ELJApp_GetClassName() }.to_str() }
    }
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    pub fn getOsDescription() -> ~str {
        unsafe { WxString { ptr: ELJApp_GetOsDescription() }.to_str() }
    }
    pub fn getOsVersion(_maj: *mut c_void, _min: *mut c_void) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    pub fn getTopWindow() -> WxWindow {
        unsafe { WxWindow { ptr: ELJApp_GetTopWindow() } }
    }
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    pub fn getUserHome(_usr: *mut c_void) -> ~str {
        unsafe { WxString { ptr: ELJApp_GetUserHome(_usr) }.to_str() }
    }
    pub fn getUserId() -> ~str {
        unsafe { WxString { ptr: ELJApp_GetUserId() }.to_str() }
    }
    pub fn getUserName() -> ~str {
        unsafe { WxString { ptr: ELJApp_GetUserName() }.to_str() }
    }
    pub fn getVendorName() -> ~str {
        unsafe { WxString { ptr: ELJApp_GetVendorName() }.to_str() }
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
    pub fn mousePosition() -> WxPoint {
        unsafe { WxPoint { ptr: ELJApp_MousePosition() } }
    }
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    pub fn safeYield<T: TWxWindow>(_win: &T) -> c_int {
        unsafe { ELJApp_SafeYield(_win.ptr()) }
    }
    pub fn setAppName(name: &str) {
        let name = wxT(name);
        unsafe { ELJApp_SetAppName(name.ptr()) }
    }
    pub fn setClassName(name: &str) {
        let name = wxT(name);
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
    pub fn setTopWindow<T: TWxWindow>(_wnd: &T) {
        unsafe { ELJApp_SetTopWindow(_wnd.ptr()) }
    }
    pub fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    pub fn setVendorName(name: &str) {
        let name = wxT(name);
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
    pub fn initializeC<T: TWxClosure>(closure: &T, _argc: c_int, _argv: *mut *mut c_char) {
        unsafe { ELJApp_InitializeC(closure.ptr(), _argc, _argv) }
    }
    pub fn getIdleInterval() -> c_int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    pub fn setIdleInterval(interval: c_int) {
        unsafe { ELJApp_SetIdleInterval(interval) }
    }
}

pub trait TWxrApp : TWxApp {
}

pub struct WxrArtProv { ptr: *mut c_void }
impl TWxrArtProv for WxrArtProv {}
impl TWxArtProvider for WxrArtProv {}
impl TWxObject for WxrArtProv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrArtProv {
    pub fn from(ptr: *mut c_void) -> WxrArtProv { WxrArtProv { ptr: ptr } }
    pub fn null() -> WxrArtProv { WxrArtProv::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _clb: *mut c_void) -> WxrArtProv {
        unsafe { WxrArtProv { ptr: ELJArtProv_Create(_obj, _clb) } }
    }
}

pub trait TWxrArtProv : TWxArtProvider {
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.ptr()) }
    }
}

pub struct WxrCommand { ptr: *mut c_void }
impl TWxrCommand for WxrCommand {}
impl TWxCommand for WxrCommand {}
impl TWxObject for WxrCommand { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrCommand {
    pub fn from(ptr: *mut c_void) -> WxrCommand { WxrCommand { ptr: ptr } }
    pub fn null() -> WxrCommand { WxrCommand::from(0 as *mut c_void) }
    
}

pub trait TWxrCommand : TWxCommand {
}

pub struct WxrDragDataObject { ptr: *mut c_void }
impl TWxrDragDataObject for WxrDragDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrDragDataObject {
    pub fn from(ptr: *mut c_void) -> WxrDragDataObject { WxrDragDataObject { ptr: ptr } }
    pub fn null() -> WxrDragDataObject { WxrDragDataObject::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fmt: &str, _func1: *mut c_void, _func2: *mut c_void, _func3: *mut c_void) -> WxrDragDataObject {
        let _fmt = wxT(_fmt);
        unsafe { WxrDragDataObject { ptr: ELJDragDataObject_Create(_obj, _fmt.ptr(), _func1, _func2, _func3) } }
    }
}

pub trait TWxrDragDataObject {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.ptr()) }
    }
}

pub struct WxrDropTarget { ptr: *mut c_void }
impl TWxrDropTarget for WxrDropTarget {}
impl TWxDropTarget for WxrDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrDropTarget {
    pub fn from(ptr: *mut c_void) -> WxrDropTarget { WxrDropTarget { ptr: ptr } }
    pub fn null() -> WxrDropTarget { WxrDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void) -> WxrDropTarget {
        unsafe { WxrDropTarget { ptr: ELJDropTarget_Create(_obj) } }
    }
}

pub trait TWxrDropTarget : TWxDropTarget {
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

pub struct WxrFileDropTarget { ptr: *mut c_void }
impl TWxrFileDropTarget for WxrFileDropTarget {}
impl TWxFileDropTarget for WxrFileDropTarget {}
impl TWxDropTarget for WxrFileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrFileDropTarget {
    pub fn from(ptr: *mut c_void) -> WxrFileDropTarget { WxrFileDropTarget { ptr: ptr } }
    pub fn null() -> WxrFileDropTarget { WxrFileDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> WxrFileDropTarget {
        unsafe { WxrFileDropTarget { ptr: ELJFileDropTarget_Create(_obj, _func) } }
    }
}

pub trait TWxrFileDropTarget : TWxFileDropTarget {
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

pub struct WxrLog { ptr: *mut c_void }
impl TWxrLog for WxrLog {}
impl TWxLog for WxrLog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrLog {
    pub fn from(ptr: *mut c_void) -> WxrLog { WxrLog { ptr: ptr } }
    pub fn null() -> WxrLog { WxrLog::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> WxrLog {
        unsafe { WxrLog { ptr: ELJLog_Create(_obj, _fnc) } }
    }
    pub fn getActiveTarget() -> *mut c_void {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

pub trait TWxrLog : TWxLog {
    fn enableLogging(&self, doIt: c_int) -> c_int {
        unsafe { ELJLog_EnableLogging(self.ptr(), doIt) }
    }
    fn isEnabled(&self) -> c_int {
        unsafe { ELJLog_IsEnabled(self.ptr()) }
    }
}

pub struct WxrPreviewControlBar { ptr: *mut c_void }
impl TWxrPreviewControlBar for WxrPreviewControlBar {}
impl TWxPreviewControlBar for WxrPreviewControlBar {}
impl TWxPanel for WxrPreviewControlBar {}
impl TWxWindow for WxrPreviewControlBar {}
impl TWxEvtHandler for WxrPreviewControlBar {}
impl TWxObject for WxrPreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrPreviewControlBar {
    pub fn from(ptr: *mut c_void) -> WxrPreviewControlBar { WxrPreviewControlBar { ptr: ptr } }
    pub fn null() -> WxrPreviewControlBar { WxrPreviewControlBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(preview: *mut c_void, buttons: c_int, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> WxrPreviewControlBar {
        unsafe { WxrPreviewControlBar { ptr: ELJPreviewControlBar_Create(preview, buttons, parent.ptr(), title, x, y, w, h, style) } }
    }
}

pub trait TWxrPreviewControlBar : TWxPreviewControlBar {
}

pub struct WxrPreviewFrame { ptr: *mut c_void }
impl TWxrPreviewFrame for WxrPreviewFrame {}
impl TWxPreviewFrame for WxrPreviewFrame {}
impl TWxFrame for WxrPreviewFrame {}
impl TWxTopLevelWindow for WxrPreviewFrame {}
impl TWxWindow for WxrPreviewFrame {}
impl TWxEvtHandler for WxrPreviewFrame {}
impl TWxObject for WxrPreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrPreviewFrame {
    pub fn from(ptr: *mut c_void) -> WxrPreviewFrame { WxrPreviewFrame { ptr: ptr } }
    pub fn null() -> WxrPreviewFrame { WxrPreviewFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_obj: *mut c_void, _init: *mut c_void, _create_canvas: *mut c_void, _create_toolbar: *mut c_void, preview: *mut c_void, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> WxrPreviewFrame {
        unsafe { WxrPreviewFrame { ptr: ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.ptr(), title, x, y, w, h, style) } }
    }
}

pub trait TWxrPreviewFrame : TWxPreviewFrame {
    fn getControlBar(&self) -> *mut c_void {
        unsafe { ELJPreviewFrame_GetControlBar(self.ptr()) }
    }
    fn getPreviewCanvas(&self) -> WxPreviewCanvas {
        unsafe { WxPreviewCanvas { ptr: ELJPreviewFrame_GetPreviewCanvas(self.ptr()) } }
    }
    fn getPrintPreview(&self) -> WxPrintPreview {
        unsafe { WxPrintPreview { ptr: ELJPreviewFrame_GetPrintPreview(self.ptr()) } }
    }
    fn setControlBar(&self, obj: *mut c_void) {
        unsafe { ELJPreviewFrame_SetControlBar(self.ptr(), obj) }
    }
    fn setPreviewCanvas<T: TWxPreviewCanvas>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.ptr(), obj.ptr()) }
    }
    fn setPrintPreview<T: TWxPrintPreview>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.ptr(), obj.ptr()) }
    }
}

pub struct WxrTextDropTarget { ptr: *mut c_void }
impl TWxrTextDropTarget for WxrTextDropTarget {}
impl TWxTextDropTarget for WxrTextDropTarget {}
impl TWxDropTarget for WxrTextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrTextDropTarget {
    pub fn from(ptr: *mut c_void) -> WxrTextDropTarget { WxrTextDropTarget { ptr: ptr } }
    pub fn null() -> WxrTextDropTarget { WxrTextDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> WxrTextDropTarget {
        unsafe { WxrTextDropTarget { ptr: ELJTextDropTarget_Create(_obj, _func) } }
    }
}

pub trait TWxrTextDropTarget : TWxTextDropTarget {
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

pub struct WxrTextValidator { ptr: *mut c_void }
impl TWxrTextValidator for WxrTextValidator {}
impl TWxTextValidator for WxrTextValidator {}
impl TWxValidator for WxrTextValidator {}
impl TWxEvtHandler for WxrTextValidator {}
impl TWxObject for WxrTextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrTextValidator {
    pub fn from(ptr: *mut c_void) -> WxrTextValidator { WxrTextValidator { ptr: ptr } }
    pub fn null() -> WxrTextValidator { WxrTextValidator::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void, _txt: *mut c_void, _stl: c_int) -> WxrTextValidator {
        unsafe { WxrTextValidator { ptr: ELJTextValidator_Create(_obj, _fnc, _txt, _stl) } }
    }
}

pub trait TWxrTextValidator : TWxTextValidator {
}

pub struct WxAcceleratorEntry { ptr: *mut c_void }
impl TWxAcceleratorEntry for WxAcceleratorEntry { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxAcceleratorEntry {
    pub fn from(ptr: *mut c_void) -> WxAcceleratorEntry { WxAcceleratorEntry { ptr: ptr } }
    pub fn null() -> WxAcceleratorEntry { WxAcceleratorEntry::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> WxAcceleratorEntry {
        unsafe { WxAcceleratorEntry { ptr: wxAcceleratorEntry_Create(flags, keyCode, cmd) } }
    }
}

pub trait TWxAcceleratorEntry {
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

pub struct WxAcceleratorTable { ptr: *mut c_void }
impl TWxAcceleratorTable for WxAcceleratorTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxAcceleratorTable {
    pub fn from(ptr: *mut c_void) -> WxAcceleratorTable { WxAcceleratorTable { ptr: ptr } }
    pub fn null() -> WxAcceleratorTable { WxAcceleratorTable::from(0 as *mut c_void) }
    
    pub fn new(n: c_int, entries: *mut c_void) -> WxAcceleratorTable {
        unsafe { WxAcceleratorTable { ptr: wxAcceleratorTable_Create(n, entries) } }
    }
}

pub trait TWxAcceleratorTable {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.ptr()) }
    }
}

pub struct WxActivateEvent { ptr: *mut c_void }
impl TWxActivateEvent for WxActivateEvent {}
impl TWxEvent for WxActivateEvent {}
impl TWxObject for WxActivateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxActivateEvent {
    pub fn from(ptr: *mut c_void) -> WxActivateEvent { WxActivateEvent { ptr: ptr } }
    pub fn null() -> WxActivateEvent { WxActivateEvent::from(0 as *mut c_void) }
    
}

pub trait TWxActivateEvent : TWxEvent {
    fn getActive(&self) -> c_int {
        unsafe { wxActivateEvent_GetActive(self.ptr()) }
    }
}

pub struct WxApp { ptr: *mut c_void }
impl TWxApp for WxApp {}
impl TWxEvtHandler for WxApp {}
impl TWxObject for WxApp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxApp {
    pub fn from(ptr: *mut c_void) -> WxApp { WxApp { ptr: ptr } }
    pub fn null() -> WxApp { WxApp::from(0 as *mut c_void) }
    
}

pub trait TWxApp : TWxEvtHandler {
}

pub struct WxArtProvider { ptr: *mut c_void }
impl TWxArtProvider for WxArtProvider {}
impl TWxObject for WxArtProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxArtProvider {
    pub fn from(ptr: *mut c_void) -> WxArtProvider { WxArtProvider { ptr: ptr } }
    pub fn null() -> WxArtProvider { WxArtProvider::from(0 as *mut c_void) }
    
}

pub trait TWxArtProvider : TWxObject {
}

pub struct WxAutoBufferedPaintDC { ptr: *mut c_void }
impl TWxAutoBufferedPaintDC for WxAutoBufferedPaintDC {}
impl TWxDC for WxAutoBufferedPaintDC {}
impl TWxObject for WxAutoBufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxAutoBufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> WxAutoBufferedPaintDC { WxAutoBufferedPaintDC { ptr: ptr } }
    pub fn null() -> WxAutoBufferedPaintDC { WxAutoBufferedPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(window: &T) -> WxAutoBufferedPaintDC {
        unsafe { WxAutoBufferedPaintDC { ptr: wxAutoBufferedPaintDC_Create(window.ptr()) } }
    }
}

pub trait TWxAutoBufferedPaintDC : TWxDC {
}

pub struct WxAutomationObject { ptr: *mut c_void }
impl TWxAutomationObject for WxAutomationObject {}
impl TWxObject for WxAutomationObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxAutomationObject {
    pub fn from(ptr: *mut c_void) -> WxAutomationObject { WxAutomationObject { ptr: ptr } }
    pub fn null() -> WxAutomationObject { WxAutomationObject::from(0 as *mut c_void) }
    
}

pub trait TWxAutomationObject : TWxObject {
}

pub struct WxBitmap { ptr: *mut c_void }
impl TWxBitmap for WxBitmap {}
impl TWxGDIObject for WxBitmap {}
impl TWxObject for WxBitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBitmap {
    pub fn from(ptr: *mut c_void) -> WxBitmap { WxBitmap { ptr: ptr } }
    pub fn null() -> WxBitmap { WxBitmap::from(0 as *mut c_void) }
    
    pub fn addHandler<T: TWxEvtHandler>(handler: &T) {
        unsafe { wxBitmap_AddHandler(handler.ptr()) }
    }
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    pub fn new(_data: *mut c_void, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> WxBitmap {
        unsafe { WxBitmap { ptr: wxBitmap_Create(_data, _type, _width, _height, _depth) } }
    }
    pub fn newDefault() -> WxBitmap {
        unsafe { WxBitmap { ptr: wxBitmap_CreateDefault() } }
    }
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> WxBitmap {
        unsafe { WxBitmap { ptr: wxBitmap_CreateEmpty(_width, _height, _depth) } }
    }
    pub fn newLoad(name: &str, type_: c_int) -> WxBitmap {
        let name = wxT(name);
        unsafe { WxBitmap { ptr: wxBitmap_CreateLoad(name.ptr(), type_) } }
    }
    pub fn findHandlerByName(name: &str) -> *mut c_void {
        let name = wxT(name);
        unsafe { wxBitmap_FindHandlerByName(name.ptr()) }
    }
    pub fn findHandlerByType(type_: c_int) -> *mut c_void {
        unsafe { wxBitmap_FindHandlerByType(type_) }
    }
    pub fn initStandardHandlers() {
        unsafe { wxBitmap_InitStandardHandlers() }
    }
    pub fn insertHandler<T: TWxEvtHandler>(handler: &T) {
        unsafe { wxBitmap_InsertHandler(handler.ptr()) }
    }
    pub fn removeHandler(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_RemoveHandler(name.ptr()) }
    }
    pub fn newFromImage<T: TWxImage>(image: &T, depth: c_int) -> WxBitmap {
        unsafe { WxBitmap { ptr: wxBitmap_CreateFromImage(image.ptr(), depth) } }
    }
}

pub trait TWxBitmap : TWxGDIObject {
    fn newFromXPM(&self) -> WxBitmap {
        unsafe { WxBitmap { ptr: wxBitmap_CreateFromXPM(self.ptr()) } }
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
    fn getMask(&self) -> WxMask {
        unsafe { WxMask { ptr: wxBitmap_GetMask(self.ptr()) } }
    }
    fn getSubBitmap<T: TWxBitmap>(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxBitmap_GetSubBitmap(self.ptr(), x, y, w, h, _ref.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.ptr()) }
    }
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_LoadFile(self.ptr(), name.ptr(), type_) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxBitmap_IsOk(self.ptr()) }
    }
    fn saveFile<T: TWxPalette>(&self, name: &str, type_: c_int, cmap: &T) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_SaveFile(self.ptr(), name.ptr(), type_, cmap.ptr()) }
    }
    fn setDepth(&self, d: c_int) {
        unsafe { wxBitmap_SetDepth(self.ptr(), d) }
    }
    fn setHeight(&self, h: c_int) {
        unsafe { wxBitmap_SetHeight(self.ptr(), h) }
    }
    fn setMask<T: TWxMask>(&self, mask: &T) {
        unsafe { wxBitmap_SetMask(self.ptr(), mask.ptr()) }
    }
    fn setWidth(&self, w: c_int) {
        unsafe { wxBitmap_SetWidth(self.ptr(), w) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxBitmap_IsStatic(self.ptr()) }
    }
}

pub struct WxBitmapButton { ptr: *mut c_void }
impl TWxBitmapButton for WxBitmapButton {}
impl TWxButton for WxBitmapButton {}
impl TWxControl for WxBitmapButton {}
impl TWxWindow for WxBitmapButton {}
impl TWxEvtHandler for WxBitmapButton {}
impl TWxObject for WxBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBitmapButton {
    pub fn from(ptr: *mut c_void) -> WxBitmapButton { WxBitmapButton { ptr: ptr } }
    pub fn null() -> WxBitmapButton { WxBitmapButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxBitmap>(_prt: &T, _id: c_int, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxBitmapButton {
        unsafe { WxBitmapButton { ptr: wxBitmapButton_Create(_prt.ptr(), _id, _bmp.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxBitmapButton : TWxButton {
    fn getBitmapDisabled<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapFocus<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapLabel<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapSelected<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapSelected(self.ptr(), _ref.ptr()) }
    }
    fn getMarginX(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginX(self.ptr()) }
    }
    fn getMarginY(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginY(self.ptr()) }
    }
    fn setBitmapDisabled<T: TWxBitmap>(&self, disabled: &T) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.ptr(), disabled.ptr()) }
    }
    fn setBitmapFocus<T: TWxBitmap>(&self, focus: &T) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.ptr(), focus.ptr()) }
    }
    fn setBitmapLabel<T: TWxBitmap>(&self, bitmap: &T) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.ptr(), bitmap.ptr()) }
    }
    fn setBitmapSelected<T: TWxBitmap>(&self, sel: &T) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.ptr(), sel.ptr()) }
    }
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.ptr(), x, y) }
    }
}

pub struct WxBitmapToggleButton { ptr: *mut c_void }
impl TWxBitmapToggleButton for WxBitmapToggleButton {}
impl TWxToggleButton for WxBitmapToggleButton {}
impl TWxControl for WxBitmapToggleButton {}
impl TWxWindow for WxBitmapToggleButton {}
impl TWxEvtHandler for WxBitmapToggleButton {}
impl TWxObject for WxBitmapToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBitmapToggleButton {
    pub fn from(ptr: *mut c_void) -> WxBitmapToggleButton { WxBitmapToggleButton { ptr: ptr } }
    pub fn null() -> WxBitmapToggleButton { WxBitmapToggleButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxBitmap>(parent: &T, id: c_int, _bmp: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> WxBitmapToggleButton {
        unsafe { WxBitmapToggleButton { ptr: wxBitmapToggleButton_Create(parent.ptr(), id, _bmp.ptr(), x, y, w, h, style) } }
    }
}

pub trait TWxBitmapToggleButton : TWxToggleButton {
    fn setBitmapLabel<T: TWxBitmap>(&self, _bmp: &T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.ptr(), _bmp.ptr()) }
    }
}

pub struct WxBitmapDataObject { ptr: *mut c_void }
impl TWxBitmapDataObject for WxBitmapDataObject {}
impl TWxDataObjectSimple for WxBitmapDataObject {}
impl TWxDataObject for WxBitmapDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBitmapDataObject {
    pub fn from(ptr: *mut c_void) -> WxBitmapDataObject { WxBitmapDataObject { ptr: ptr } }
    pub fn null() -> WxBitmapDataObject { WxBitmapDataObject::from(0 as *mut c_void) }
    
}

pub trait TWxBitmapDataObject : TWxDataObjectSimple {
}

pub struct WxBitmapHandler { ptr: *mut c_void }
impl TWxBitmapHandler for WxBitmapHandler {}
impl TWxObject for WxBitmapHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBitmapHandler {
    pub fn from(ptr: *mut c_void) -> WxBitmapHandler { WxBitmapHandler { ptr: ptr } }
    pub fn null() -> WxBitmapHandler { WxBitmapHandler::from(0 as *mut c_void) }
    
}

pub trait TWxBitmapHandler : TWxObject {
}

pub struct WxBoxSizer { ptr: *mut c_void }
impl TWxBoxSizer for WxBoxSizer {}
impl TWxSizer for WxBoxSizer {}
impl TWxObject for WxBoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBoxSizer {
    pub fn from(ptr: *mut c_void) -> WxBoxSizer { WxBoxSizer { ptr: ptr } }
    pub fn null() -> WxBoxSizer { WxBoxSizer::from(0 as *mut c_void) }
    
    pub fn new(orient: c_int) -> WxBoxSizer {
        unsafe { WxBoxSizer { ptr: wxBoxSizer_Create(orient) } }
    }
}

pub trait TWxBoxSizer : TWxSizer {
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.ptr()) }
    }
}

pub struct WxBrush { ptr: *mut c_void }
impl TWxBrush for WxBrush {}
impl TWxGDIObject for WxBrush {}
impl TWxObject for WxBrush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBrush {
    pub fn from(ptr: *mut c_void) -> WxBrush { WxBrush { ptr: ptr } }
    pub fn null() -> WxBrush { WxBrush::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxBrush {
        unsafe { WxBrush { ptr: wxBrush_CreateDefault() } }
    }
    pub fn newFromBitmap<T: TWxBitmap>(bitmap: &T) -> WxBrush {
        unsafe { WxBrush { ptr: wxBrush_CreateFromBitmap(bitmap.ptr()) } }
    }
    pub fn newFromColour<T: TWxColour>(col: &T, style: c_int) -> WxBrush {
        unsafe { WxBrush { ptr: wxBrush_CreateFromColour(col.ptr(), style) } }
    }
    pub fn newFromStock(id: c_int) -> WxBrush {
        unsafe { WxBrush { ptr: wxBrush_CreateFromStock(id) } }
    }
}

pub trait TWxBrush : TWxGDIObject {
    fn assign<T: TWxBrush>(&self, brush: &T) {
        unsafe { wxBrush_Assign(self.ptr(), brush.ptr()) }
    }
    fn getColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxBrush_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getStipple<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxBrush_GetStipple(self.ptr(), _ref.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.ptr()) }
    }
    fn isEqual<T: TWxBrush>(&self, brush: &T) -> c_int {
        unsafe { wxBrush_IsEqual(self.ptr(), brush.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxBrush_IsOk(self.ptr()) }
    }
    fn setColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxBrush_SetColour(self.ptr(), col.ptr()) }
    }
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxBrush_SetColourSingle(self.ptr(), r, g, b) }
    }
    fn setStipple<T: TWxBitmap>(&self, stipple: &T) {
        unsafe { wxBrush_SetStipple(self.ptr(), stipple.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxBrush_SetStyle(self.ptr(), style) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxBrush_IsStatic(self.ptr()) }
    }
}

pub struct WxBrushList { ptr: *mut c_void }
impl TWxBrushList for WxBrushList {}
impl TWxList for WxBrushList {}
impl TWxObject for WxBrushList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBrushList {
    pub fn from(ptr: *mut c_void) -> WxBrushList { WxBrushList { ptr: ptr } }
    pub fn null() -> WxBrushList { WxBrushList::from(0 as *mut c_void) }
    
}

pub trait TWxBrushList : TWxList {
}

pub struct WxBufferedDC { ptr: *mut c_void }
impl TWxBufferedDC for WxBufferedDC {}
impl TWxDC for WxBufferedDC {}
impl TWxObject for WxBufferedDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBufferedDC {
    pub fn from(ptr: *mut c_void) -> WxBufferedDC { WxBufferedDC { ptr: ptr } }
    pub fn null() -> WxBufferedDC { WxBufferedDC::from(0 as *mut c_void) }
    
    pub fn newByDCAndSize<T: TWxDC>(dc: &T, width: c_int, hight: c_int, style: c_int) -> WxBufferedDC {
        unsafe { WxBufferedDC { ptr: wxBufferedDC_CreateByDCAndSize(dc.ptr(), width, hight, style) } }
    }
    pub fn newByDCAndBitmap<T: TWxDC, U: TWxBitmap>(dc: &T, bitmap: &U, style: c_int) -> WxBufferedDC {
        unsafe { WxBufferedDC { ptr: wxBufferedDC_CreateByDCAndBitmap(dc.ptr(), bitmap.ptr(), style) } }
    }
}

pub trait TWxBufferedDC : TWxDC {
}

pub struct WxBufferedPaintDC { ptr: *mut c_void }
impl TWxBufferedPaintDC for WxBufferedPaintDC {}
impl TWxDC for WxBufferedPaintDC {}
impl TWxObject for WxBufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> WxBufferedPaintDC { WxBufferedPaintDC { ptr: ptr } }
    pub fn null() -> WxBufferedPaintDC { WxBufferedPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(window: &T, style: c_int) -> WxBufferedPaintDC {
        unsafe { WxBufferedPaintDC { ptr: wxBufferedPaintDC_Create(window.ptr(), style) } }
    }
    pub fn newWithBitmap<T: TWxWindow, U: TWxBitmap>(window: &T, bitmap: &U, style: c_int) -> WxBufferedPaintDC {
        unsafe { WxBufferedPaintDC { ptr: wxBufferedPaintDC_CreateWithBitmap(window.ptr(), bitmap.ptr(), style) } }
    }
}

pub trait TWxBufferedPaintDC : TWxDC {
}

pub struct WxBusyCursor { ptr: *mut c_void }
impl TWxBusyCursor for WxBusyCursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBusyCursor {
    pub fn from(ptr: *mut c_void) -> WxBusyCursor { WxBusyCursor { ptr: ptr } }
    pub fn null() -> WxBusyCursor { WxBusyCursor::from(0 as *mut c_void) }
    
    pub fn new() -> WxBusyCursor {
        unsafe { WxBusyCursor { ptr: wxBusyCursor_Create() } }
    }
}

pub trait TWxBusyCursor {
    fn ptr(&self) -> *mut c_void;
    
    fn newWithCursor(&self) -> *mut c_void {
        unsafe { wxBusyCursor_CreateWithCursor(self.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.ptr()) }
    }
}

pub struct WxBusyInfo { ptr: *mut c_void }
impl TWxBusyInfo for WxBusyInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxBusyInfo {
    pub fn from(ptr: *mut c_void) -> WxBusyInfo { WxBusyInfo { ptr: ptr } }
    pub fn null() -> WxBusyInfo { WxBusyInfo::from(0 as *mut c_void) }
    
    pub fn new(_txt: &str) -> WxBusyInfo {
        let _txt = wxT(_txt);
        unsafe { WxBusyInfo { ptr: wxBusyInfo_Create(_txt.ptr()) } }
    }
}

pub trait TWxBusyInfo {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.ptr()) }
    }
}

pub struct WxButton { ptr: *mut c_void }
impl TWxButton for WxButton {}
impl TWxControl for WxButton {}
impl TWxWindow for WxButton {}
impl TWxEvtHandler for WxButton {}
impl TWxObject for WxButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxButton {
    pub fn from(ptr: *mut c_void) -> WxButton { WxButton { ptr: ptr } }
    pub fn null() -> WxButton { WxButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxButton {
        let _txt = wxT(_txt);
        unsafe { WxButton { ptr: wxButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxButton : TWxControl {
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.ptr()) }
    }
}

pub struct WxCaret { ptr: *mut c_void }
impl TWxCaret for WxCaret { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCaret {
    pub fn from(ptr: *mut c_void) -> WxCaret { WxCaret { ptr: ptr } }
    pub fn null() -> WxCaret { WxCaret::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_wnd: &T, _wth: c_int, _hgt: c_int) -> WxCaret {
        unsafe { WxCaret { ptr: wxCaret_Create(_wnd.ptr(), _wth, _hgt) } }
    }
    pub fn getBlinkTime() -> c_int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    pub fn setBlinkTime(milliseconds: c_int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
    }
}

pub trait TWxCaret {
    fn ptr(&self) -> *mut c_void;
    
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxCaret_GetPosition(self.ptr()) } }
    }
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxCaret_GetSize(self.ptr()) } }
    }
    fn getWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxCaret_GetWindow(self.ptr()) } }
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

pub struct WxCheckBox { ptr: *mut c_void }
impl TWxCheckBox for WxCheckBox {}
impl TWxControl for WxCheckBox {}
impl TWxWindow for WxCheckBox {}
impl TWxEvtHandler for WxCheckBox {}
impl TWxObject for WxCheckBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCheckBox {
    pub fn from(ptr: *mut c_void) -> WxCheckBox { WxCheckBox { ptr: ptr } }
    pub fn null() -> WxCheckBox { WxCheckBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxCheckBox {
        let _txt = wxT(_txt);
        unsafe { WxCheckBox { ptr: wxCheckBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxCheckBox : TWxControl {
    fn getValue(&self) -> c_int {
        unsafe { wxCheckBox_GetValue(self.ptr()) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self.ptr(), value) }
    }
}

pub struct WxCheckListBox { ptr: *mut c_void }
impl TWxCheckListBox for WxCheckListBox {}
impl TWxListBox for WxCheckListBox {}
impl TWxControl for WxCheckListBox {}
impl TWxWindow for WxCheckListBox {}
impl TWxEvtHandler for WxCheckListBox {}
impl TWxObject for WxCheckListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCheckListBox {
    pub fn from(ptr: *mut c_void) -> WxCheckListBox { WxCheckListBox { ptr: ptr } }
    pub fn null() -> WxCheckListBox { WxCheckListBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> WxCheckListBox {
        unsafe { WxCheckListBox { ptr: wxCheckListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TWxCheckListBox : TWxListBox {
    fn check(&self, item: c_int, check: c_int) {
        unsafe { wxCheckListBox_Check(self.ptr(), item, check) }
    }
    fn isChecked(&self, item: c_int) -> c_int {
        unsafe { wxCheckListBox_IsChecked(self.ptr(), item) }
    }
}

pub struct WxChoice { ptr: *mut c_void }
impl TWxChoice for WxChoice {}
impl TWxControl for WxChoice {}
impl TWxWindow for WxChoice {}
impl TWxEvtHandler for WxChoice {}
impl TWxObject for WxChoice { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxChoice {
    pub fn from(ptr: *mut c_void) -> WxChoice { WxChoice { ptr: ptr } }
    pub fn null() -> WxChoice { WxChoice::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> WxChoice {
        unsafe { WxChoice { ptr: wxChoice_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TWxChoice : TWxControl {
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxChoice_Append(self.ptr(), item.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxChoice_Clear(self.ptr()) }
    }
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxChoice_FindString(self.ptr(), s.ptr()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxChoice_GetCount(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxChoice_GetSelection(self.ptr()) }
    }
    fn getString(&self, n: c_int) -> ~str {
        unsafe { WxString { ptr: wxChoice_GetString(self.ptr(), n) }.to_str() }
    }
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.ptr(), n) }
    }
    fn setString(&self, n: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxChoice_SetString(self.ptr(), n, s.ptr()) }
    }
}

pub struct WxClientDC { ptr: *mut c_void }
impl TWxClientDC for WxClientDC {}
impl TWxWindowDC for WxClientDC {}
impl TWxDC for WxClientDC {}
impl TWxObject for WxClientDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClientDC {
    pub fn from(ptr: *mut c_void) -> WxClientDC { WxClientDC { ptr: ptr } }
    pub fn null() -> WxClientDC { WxClientDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(win: &T) -> WxClientDC {
        unsafe { WxClientDC { ptr: wxClientDC_Create(win.ptr()) } }
    }
}

pub trait TWxClientDC : TWxWindowDC {
}

pub struct WxClipboard { ptr: *mut c_void }
impl TWxClipboard for WxClipboard {}
impl TWxObject for WxClipboard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxClipboard {
    pub fn from(ptr: *mut c_void) -> WxClipboard { WxClipboard { ptr: ptr } }
    pub fn null() -> WxClipboard { WxClipboard::from(0 as *mut c_void) }
    
    pub fn new() -> WxClipboard {
        unsafe { WxClipboard { ptr: wxClipboard_Create() } }
    }
}

pub trait TWxClipboard : TWxObject {
    fn addData<T: TWxDataObject>(&self, data: &T) -> c_int {
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
    fn getData<T: TWxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_GetData(self.ptr(), data.ptr()) }
    }
    fn isOpened(&self) -> c_int {
        unsafe { wxClipboard_IsOpened(self.ptr()) }
    }
    fn isSupported<T: TWxDataFormat>(&self, format: &T) -> c_int {
        unsafe { wxClipboard_IsSupported(self.ptr(), format.ptr()) }
    }
    fn open(&self) -> c_int {
        unsafe { wxClipboard_Open(self.ptr()) }
    }
    fn setData<T: TWxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_SetData(self.ptr(), data.ptr()) }
    }
    fn usePrimarySelection(&self, primary: c_int) {
        unsafe { wxClipboard_UsePrimarySelection(self.ptr(), primary) }
    }
}

pub struct WxCloseEvent { ptr: *mut c_void }
impl TWxCloseEvent for WxCloseEvent {}
impl TWxEvent for WxCloseEvent {}
impl TWxObject for WxCloseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCloseEvent {
    pub fn from(ptr: *mut c_void) -> WxCloseEvent { WxCloseEvent { ptr: ptr } }
    pub fn null() -> WxCloseEvent { WxCloseEvent::from(0 as *mut c_void) }
    
}

pub trait TWxCloseEvent : TWxEvent {
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

pub struct WxColour { ptr: *mut c_void }
impl TWxColour for WxColour {}
impl TWxObject for WxColour { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxColour {
    pub fn from(ptr: *mut c_void) -> WxColour { WxColour { ptr: ptr } }
    pub fn null() -> WxColour { WxColour::from(0 as *mut c_void) }
    
    pub fn newByName(_name: &str) -> WxColour {
        let _name = wxT(_name);
        unsafe { WxColour { ptr: wxColour_CreateByName(_name.ptr()) } }
    }
    pub fn newEmpty() -> WxColour {
        unsafe { WxColour { ptr: wxColour_CreateEmpty() } }
    }
    pub fn newFromStock(id: c_int) -> WxColour {
        unsafe { WxColour { ptr: wxColour_CreateFromStock(id) } }
    }
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> WxColour {
        unsafe { WxColour { ptr: wxColour_CreateRGB(_red, _green, _blue, _alpha) } }
    }
    pub fn validName(_name: *mut c_void) -> c_int {
        unsafe { wxColour_ValidName(_name) }
    }
    pub fn newFromInt(rgb: c_int) -> WxColour {
        unsafe { WxColour { ptr: wxColour_CreateFromInt(rgb) } }
    }
    pub fn newFromUnsignedInt(rgba: uint32_t) -> WxColour {
        unsafe { WxColour { ptr: wxColour_CreateFromUnsignedInt(rgba) } }
    }
}

pub trait TWxColour : TWxObject {
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
        let _name = wxT(_name);
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

pub struct WxColourData { ptr: *mut c_void }
impl TWxColourData for WxColourData {}
impl TWxObject for WxColourData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxColourData {
    pub fn from(ptr: *mut c_void) -> WxColourData { WxColourData { ptr: ptr } }
    pub fn null() -> WxColourData { WxColourData::from(0 as *mut c_void) }
    
    pub fn new() -> WxColourData {
        unsafe { WxColourData { ptr: wxColourData_Create() } }
    }
}

pub trait TWxColourData : TWxObject {
    fn getChooseFull(&self) -> c_int {
        unsafe { wxColourData_GetChooseFull(self.ptr()) }
    }
    fn getColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxColourData_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getCustomColour<T: TWxColour>(&self, i: c_int, _ref: &T) {
        unsafe { wxColourData_GetCustomColour(self.ptr(), i, _ref.ptr()) }
    }
    fn setChooseFull(&self, flag: c_int) {
        unsafe { wxColourData_SetChooseFull(self.ptr(), flag) }
    }
    fn setColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxColourData_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCustomColour<T: TWxColour>(&self, i: c_int, colour: &T) {
        unsafe { wxColourData_SetCustomColour(self.ptr(), i, colour.ptr()) }
    }
}

pub struct WxColourDatabase { ptr: *mut c_void }
impl TWxColourDatabase for WxColourDatabase {}
impl TWxList for WxColourDatabase {}
impl TWxObject for WxColourDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxColourDatabase {
    pub fn from(ptr: *mut c_void) -> WxColourDatabase { WxColourDatabase { ptr: ptr } }
    pub fn null() -> WxColourDatabase { WxColourDatabase::from(0 as *mut c_void) }
    
}

pub trait TWxColourDatabase : TWxList {
}

pub struct WxColourDialog { ptr: *mut c_void }
impl TWxColourDialog for WxColourDialog {}
impl TWxDialog for WxColourDialog {}
impl TWxTopLevelWindow for WxColourDialog {}
impl TWxWindow for WxColourDialog {}
impl TWxEvtHandler for WxColourDialog {}
impl TWxObject for WxColourDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxColourDialog {
    pub fn from(ptr: *mut c_void) -> WxColourDialog { WxColourDialog { ptr: ptr } }
    pub fn null() -> WxColourDialog { WxColourDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxColourData>(_prt: &T, col: &U) -> WxColourDialog {
        unsafe { WxColourDialog { ptr: wxColourDialog_Create(_prt.ptr(), col.ptr()) } }
    }
}

pub trait TWxColourDialog : TWxDialog {
    fn getColourData<T: TWxColourData>(&self, _ref: &T) {
        unsafe { wxColourDialog_GetColourData(self.ptr(), _ref.ptr()) }
    }
}

pub struct WxComboBox { ptr: *mut c_void }
impl TWxComboBox for WxComboBox {}
impl TWxChoice for WxComboBox {}
impl TWxControl for WxComboBox {}
impl TWxWindow for WxComboBox {}
impl TWxEvtHandler for WxComboBox {}
impl TWxObject for WxComboBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxComboBox {
    pub fn from(ptr: *mut c_void) -> WxComboBox { WxComboBox { ptr: ptr } }
    pub fn null() -> WxComboBox { WxComboBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> WxComboBox {
        let _txt = wxT(_txt);
        unsafe { WxComboBox { ptr: wxComboBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TWxComboBox : TWxChoice {
    fn appendData(&self, item: &str, d: *mut c_void) {
        let item = wxT(item);
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
        unsafe { WxString { ptr: wxComboBox_GetStringSelection(self.ptr()) }.to_str() }
    }
    fn getValue(&self) -> ~str {
        unsafe { WxString { ptr: wxComboBox_GetValue(self.ptr()) }.to_str() }
    }
    fn paste(&self) {
        unsafe { wxComboBox_Paste(self.ptr()) }
    }
    fn remove(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_Remove(self.ptr(), from, to) }
    }
    fn replace(&self, from: c_int, to: c_int, value: &str) {
        let value = wxT(value);
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

pub struct WxCommand { ptr: *mut c_void }
impl TWxCommand for WxCommand {}
impl TWxObject for WxCommand { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCommand {
    pub fn from(ptr: *mut c_void) -> WxCommand { WxCommand { ptr: ptr } }
    pub fn null() -> WxCommand { WxCommand::from(0 as *mut c_void) }
    
}

pub trait TWxCommand : TWxObject {
}

pub struct WxCommandEvent { ptr: *mut c_void }
impl TWxCommandEvent for WxCommandEvent {}
impl TWxEvent for WxCommandEvent {}
impl TWxObject for WxCommandEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCommandEvent {
    pub fn from(ptr: *mut c_void) -> WxCommandEvent { WxCommandEvent { ptr: ptr } }
    pub fn null() -> WxCommandEvent { WxCommandEvent::from(0 as *mut c_void) }
    
    pub fn new(_typ: c_int, _id: c_int) -> WxCommandEvent {
        unsafe { WxCommandEvent { ptr: wxCommandEvent_Create(_typ, _id) } }
    }
}

pub trait TWxCommandEvent : TWxEvent {
    fn getClientData(&self) -> WxClientData {
        unsafe { WxClientData { ptr: wxCommandEvent_GetClientData(self.ptr()) } }
    }
    fn getClientObject(&self) -> WxClientData {
        unsafe { WxClientData { ptr: wxCommandEvent_GetClientObject(self.ptr()) } }
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
        unsafe { WxString { ptr: wxCommandEvent_GetString(self.ptr()) }.to_str() }
    }
    fn isChecked(&self) -> c_int {
        unsafe { wxCommandEvent_IsChecked(self.ptr()) }
    }
    fn isSelection(&self) -> c_int {
        unsafe { wxCommandEvent_IsSelection(self.ptr()) }
    }
    fn setClientData<T: TWxClientData>(&self, clientData: &T) {
        unsafe { wxCommandEvent_SetClientData(self.ptr(), clientData.ptr()) }
    }
    fn setClientObject<T: TWxClientData>(&self, clientObject: &T) {
        unsafe { wxCommandEvent_SetClientObject(self.ptr(), clientObject.ptr()) }
    }
    fn setExtraLong(&self, extraLong: c_long) {
        unsafe { wxCommandEvent_SetExtraLong(self.ptr(), extraLong) }
    }
    fn setInt(&self, i: c_int) {
        unsafe { wxCommandEvent_SetInt(self.ptr(), i) }
    }
    fn setString(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxCommandEvent_SetString(self.ptr(), s.ptr()) }
    }
}

pub struct WxCommandProcessor { ptr: *mut c_void }
impl TWxCommandProcessor for WxCommandProcessor {}
impl TWxObject for WxCommandProcessor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCommandProcessor {
    pub fn from(ptr: *mut c_void) -> WxCommandProcessor { WxCommandProcessor { ptr: ptr } }
    pub fn null() -> WxCommandProcessor { WxCommandProcessor::from(0 as *mut c_void) }
    
}

pub trait TWxCommandProcessor : TWxObject {
}

pub struct WxContextHelp { ptr: *mut c_void }
impl TWxContextHelp for WxContextHelp {}
impl TWxObject for WxContextHelp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxContextHelp {
    pub fn from(ptr: *mut c_void) -> WxContextHelp { WxContextHelp { ptr: ptr } }
    pub fn null() -> WxContextHelp { WxContextHelp::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(win: &T, beginHelp: c_int) -> WxContextHelp {
        unsafe { WxContextHelp { ptr: wxContextHelp_Create(win.ptr(), beginHelp) } }
    }
}

pub trait TWxContextHelp : TWxObject {
    fn beginContextHelp<T: TWxWindow>(&self, win: &T) -> c_int {
        unsafe { wxContextHelp_BeginContextHelp(self.ptr(), win.ptr()) }
    }
    fn endContextHelp(&self) -> c_int {
        unsafe { wxContextHelp_EndContextHelp(self.ptr()) }
    }
}

pub struct WxContextHelpButton { ptr: *mut c_void }
impl TWxContextHelpButton for WxContextHelpButton {}
impl TWxBitmapButton for WxContextHelpButton {}
impl TWxButton for WxContextHelpButton {}
impl TWxControl for WxContextHelpButton {}
impl TWxWindow for WxContextHelpButton {}
impl TWxEvtHandler for WxContextHelpButton {}
impl TWxObject for WxContextHelpButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxContextHelpButton {
    pub fn from(ptr: *mut c_void) -> WxContextHelpButton { WxContextHelpButton { ptr: ptr } }
    pub fn null() -> WxContextHelpButton { WxContextHelpButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(parent: &T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> WxContextHelpButton {
        unsafe { WxContextHelpButton { ptr: wxContextHelpButton_Create(parent.ptr(), id, x, y, w, h, style) } }
    }
}

pub trait TWxContextHelpButton : TWxBitmapButton {
}

pub struct WxControl { ptr: *mut c_void }
impl TWxControl for WxControl {}
impl TWxWindow for WxControl {}
impl TWxEvtHandler for WxControl {}
impl TWxObject for WxControl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxControl {
    pub fn from(ptr: *mut c_void) -> WxControl { WxControl { ptr: ptr } }
    pub fn null() -> WxControl { WxControl::from(0 as *mut c_void) }
    
}

pub trait TWxControl : TWxWindow {
    fn command<T: TWxEvent>(&self, event: &T) {
        unsafe { wxControl_Command(self.ptr(), event.ptr()) }
    }
}

pub struct WxCursor { ptr: *mut c_void }
impl TWxCursor for WxCursor {}
impl TWxBitmap for WxCursor {}
impl TWxGDIObject for WxCursor {}
impl TWxObject for WxCursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCursor {
    pub fn from(ptr: *mut c_void) -> WxCursor { WxCursor { ptr: ptr } }
    pub fn null() -> WxCursor { WxCursor::from(0 as *mut c_void) }
    
}

pub trait TWxCursor : TWxBitmap {
}

pub struct WxCustomDataObject { ptr: *mut c_void }
impl TWxCustomDataObject for WxCustomDataObject {}
impl TWxDataObjectSimple for WxCustomDataObject {}
impl TWxDataObject for WxCustomDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCustomDataObject {
    pub fn from(ptr: *mut c_void) -> WxCustomDataObject { WxCustomDataObject { ptr: ptr } }
    pub fn null() -> WxCustomDataObject { WxCustomDataObject::from(0 as *mut c_void) }
    
}

pub trait TWxCustomDataObject : TWxDataObjectSimple {
}

pub struct WxDC { ptr: *mut c_void }
impl TWxDC for WxDC {}
impl TWxObject for WxDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDC {
    pub fn from(ptr: *mut c_void) -> WxDC { WxDC { ptr: ptr } }
    pub fn null() -> WxDC { WxDC::from(0 as *mut c_void) }
    
}

pub trait TWxDC : TWxObject {
    fn blit<T: TWxDC>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: &T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: c_int) -> c_int {
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
    fn drawBitmap<T: TWxBitmap>(&self, bmp: &T, x: c_int, y: c_int, useMask: c_int) {
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
    fn drawIcon<T: TWxIcon>(&self, icon: &T, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.ptr(), icon.ptr(), x, y) }
    }
    fn drawLabel(&self, str: &str, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        let str = wxT(str);
        unsafe { wxDC_DrawLabel(self.ptr(), str.ptr(), x, y, w, h, align, indexAccel) }
    }
    fn drawLabelBitmap<T: TWxBitmap>(&self, str: &str, bmp: &T, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> WxRect {
        let str = wxT(str);
        unsafe { WxRect { ptr: wxDC_DrawLabelBitmap(self.ptr(), str.ptr(), bmp.ptr(), x, y, w, h, align, indexAccel) } }
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
        let text = wxT(text);
        unsafe { wxDC_DrawRotatedText(self.ptr(), text.ptr(), x, y, angle) }
    }
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.ptr(), x, y, width, height, radius) }
    }
    fn drawText(&self, text: &str, x: c_int, y: c_int) {
        let text = wxT(text);
        unsafe { wxDC_DrawText(self.ptr(), text.ptr(), x, y) }
    }
    fn endDoc(&self) {
        unsafe { wxDC_EndDoc(self.ptr()) }
    }
    fn endPage(&self) {
        unsafe { wxDC_EndPage(self.ptr()) }
    }
    fn floodFill<T: TWxColour>(&self, x: c_int, y: c_int, col: &T, style: c_int) {
        unsafe { wxDC_FloodFill(self.ptr(), x, y, col.ptr(), style) }
    }
    fn getBackground<T: TWxBrush>(&self, _ref: &T) {
        unsafe { wxDC_GetBackground(self.ptr(), _ref.ptr()) }
    }
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.ptr()) }
    }
    fn getBrush<T: TWxBrush>(&self, _ref: &T) {
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
    fn getFont<T: TWxFont>(&self, _ref: &T) {
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
    fn getPPI(&self) -> WxSize {
        unsafe { WxSize { ptr: wxDC_GetPPI(self.ptr()) } }
    }
    fn getPen<T: TWxPen>(&self, _ref: &T) {
        unsafe { wxDC_GetPen(self.ptr(), _ref.ptr()) }
    }
    fn getPixel<T: TWxColour>(&self, x: c_int, y: c_int, col: &T) -> c_int {
        unsafe { wxDC_GetPixel(self.ptr(), x, y, col.ptr()) }
    }
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxDC_GetSize(self.ptr()) } }
    }
    fn getSizeMM(&self) -> WxSize {
        unsafe { WxSize { ptr: wxDC_GetSizeMM(self.ptr()) } }
    }
    fn getTextBackground<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxDC_GetTextBackground(self.ptr(), _ref.ptr()) }
    }
    fn getTextExtent<T: TWxFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, externalLeading: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetTextExtent(self.ptr(), string.ptr(), w, h, descent, externalLeading, theFont.ptr()) }
    }
    fn getMultiLineTextExtent<T: TWxFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, heightLine: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetMultiLineTextExtent(self.ptr(), string.ptr(), w, h, heightLine, theFont.ptr()) }
    }
    fn getTextForeground<T: TWxColour>(&self, _ref: &T) {
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
    fn setBackground<T: TWxBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBackground(self.ptr(), brush.ptr()) }
    }
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.ptr(), mode) }
    }
    fn setBrush<T: TWxBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBrush(self.ptr(), brush.ptr()) }
    }
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.ptr(), x, y, width, height) }
    }
    fn setClippingRegionFromRegion<T: TWxRegion>(&self, region: &T) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.ptr(), region.ptr()) }
    }
    fn setDeviceClippingRegion<T: TWxRegion>(&self, region: &T) {
        unsafe { wxDC_SetDeviceClippingRegion(self.ptr(), region.ptr()) }
    }
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.ptr(), x, y) }
    }
    fn setFont<T: TWxFont>(&self, font: &T) {
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
    fn setPalette<T: TWxPalette>(&self, palette: &T) {
        unsafe { wxDC_SetPalette(self.ptr(), palette.ptr()) }
    }
    fn setPen<T: TWxPen>(&self, pen: &T) {
        unsafe { wxDC_SetPen(self.ptr(), pen.ptr()) }
    }
    fn setTextBackground<T: TWxColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextBackground(self.ptr(), colour.ptr()) }
    }
    fn setTextForeground<T: TWxColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextForeground(self.ptr(), colour.ptr()) }
    }
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.ptr(), x, y) }
    }
    fn startDoc(&self, msg: &str) -> c_int {
        let msg = wxT(msg);
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
    fn getPixel2<T: TWxColour>(&self, x: c_int, y: c_int, col: &T) {
        unsafe { wxDC_GetPixel2(self.ptr(), x, y, col.ptr()) }
    }
}

pub struct WxDCClipper { ptr: *mut c_void }
impl TWxDCClipper for WxDCClipper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDCClipper {
    pub fn from(ptr: *mut c_void) -> WxDCClipper { WxDCClipper { ptr: ptr } }
    pub fn null() -> WxDCClipper { WxDCClipper::from(0 as *mut c_void) }
    
}

pub trait TWxDCClipper {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDataFormat { ptr: *mut c_void }
impl TWxDataFormat for WxDataFormat { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDataFormat {
    pub fn from(ptr: *mut c_void) -> WxDataFormat { WxDataFormat { ptr: ptr } }
    pub fn null() -> WxDataFormat { WxDataFormat::from(0 as *mut c_void) }
    
    pub fn newFromId(name: &str) -> WxDataFormat {
        let name = wxT(name);
        unsafe { WxDataFormat { ptr: wxDataFormat_CreateFromId(name.ptr()) } }
    }
    pub fn newFromType(typ: c_int) -> WxDataFormat {
        unsafe { WxDataFormat { ptr: wxDataFormat_CreateFromType(typ) } }
    }
}

pub trait TWxDataFormat {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.ptr()) }
    }
    fn getId(&self) -> ~str {
        unsafe { WxString { ptr: wxDataFormat_GetId(self.ptr()) }.to_str() }
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

pub struct WxDataObject { ptr: *mut c_void }
impl TWxDataObject for WxDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDataObject {
    pub fn from(ptr: *mut c_void) -> WxDataObject { WxDataObject { ptr: ptr } }
    pub fn null() -> WxDataObject { WxDataObject::from(0 as *mut c_void) }
    
}

pub trait TWxDataObject {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDataObjectComposite { ptr: *mut c_void }
impl TWxDataObjectComposite for WxDataObjectComposite {}
impl TWxDataObject for WxDataObjectComposite { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDataObjectComposite {
    pub fn from(ptr: *mut c_void) -> WxDataObjectComposite { WxDataObjectComposite { ptr: ptr } }
    pub fn null() -> WxDataObjectComposite { WxDataObjectComposite::from(0 as *mut c_void) }
    
    pub fn new() -> WxDataObjectComposite {
        unsafe { WxDataObjectComposite { ptr: wxDataObjectComposite_Create() } }
    }
}

pub trait TWxDataObjectComposite : TWxDataObject {
    fn add(&self, _dat: *mut c_void, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.ptr(), _dat, _preferred) }
    }
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.ptr()) }
    }
}

pub struct WxDataObjectSimple { ptr: *mut c_void }
impl TWxDataObjectSimple for WxDataObjectSimple {}
impl TWxDataObject for WxDataObjectSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDataObjectSimple {
    pub fn from(ptr: *mut c_void) -> WxDataObjectSimple { WxDataObjectSimple { ptr: ptr } }
    pub fn null() -> WxDataObjectSimple { WxDataObjectSimple::from(0 as *mut c_void) }
    
}

pub trait TWxDataObjectSimple : TWxDataObject {
}

pub struct WxDialUpEvent { ptr: *mut c_void }
impl TWxDialUpEvent for WxDialUpEvent {}
impl TWxEvent for WxDialUpEvent {}
impl TWxObject for WxDialUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDialUpEvent {
    pub fn from(ptr: *mut c_void) -> WxDialUpEvent { WxDialUpEvent { ptr: ptr } }
    pub fn null() -> WxDialUpEvent { WxDialUpEvent::from(0 as *mut c_void) }
    
}

pub trait TWxDialUpEvent : TWxEvent {
}

pub struct WxDialUpManager { ptr: *mut c_void }
impl TWxDialUpManager for WxDialUpManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDialUpManager {
    pub fn from(ptr: *mut c_void) -> WxDialUpManager { WxDialUpManager { ptr: ptr } }
    pub fn null() -> WxDialUpManager { WxDialUpManager::from(0 as *mut c_void) }
    
}

pub trait TWxDialUpManager {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDialog { ptr: *mut c_void }
impl TWxDialog for WxDialog {}
impl TWxTopLevelWindow for WxDialog {}
impl TWxWindow for WxDialog {}
impl TWxEvtHandler for WxDialog {}
impl TWxObject for WxDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDialog {
    pub fn from(ptr: *mut c_void) -> WxDialog { WxDialog { ptr: ptr } }
    pub fn null() -> WxDialog { WxDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxDialog {
        let _txt = wxT(_txt);
        unsafe { WxDialog { ptr: wxDialog_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxDialog : TWxTopLevelWindow {
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

pub struct WxDirDialog { ptr: *mut c_void }
impl TWxDirDialog for WxDirDialog {}
impl TWxDialog for WxDirDialog {}
impl TWxTopLevelWindow for WxDirDialog {}
impl TWxWindow for WxDirDialog {}
impl TWxEvtHandler for WxDirDialog {}
impl TWxObject for WxDirDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDirDialog {
    pub fn from(ptr: *mut c_void) -> WxDirDialog { WxDirDialog { ptr: ptr } }
    pub fn null() -> WxDirDialog { WxDirDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _msg: &str, _dir: &str, _lft: c_int, _top: c_int, _stl: c_int) -> WxDirDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        unsafe { WxDirDialog { ptr: wxDirDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _lft, _top, _stl) } }
    }
}

pub trait TWxDirDialog : TWxDialog {
    fn getMessage(&self) -> ~str {
        unsafe { WxString { ptr: wxDirDialog_GetMessage(self.ptr()) }.to_str() }
    }
    fn getPath(&self) -> ~str {
        unsafe { WxString { ptr: wxDirDialog_GetPath(self.ptr()) }.to_str() }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.ptr()) }
    }
    fn setMessage(&self, msg: &str) {
        let msg = wxT(msg);
        unsafe { wxDirDialog_SetMessage(self.ptr(), msg.ptr()) }
    }
    fn setPath(&self, pth: &str) {
        let pth = wxT(pth);
        unsafe { wxDirDialog_SetPath(self.ptr(), pth.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.ptr(), style) }
    }
}

pub struct WxDocChildFrame { ptr: *mut c_void }
impl TWxDocChildFrame for WxDocChildFrame {}
impl TWxFrame for WxDocChildFrame {}
impl TWxTopLevelWindow for WxDocChildFrame {}
impl TWxWindow for WxDocChildFrame {}
impl TWxEvtHandler for WxDocChildFrame {}
impl TWxObject for WxDocChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocChildFrame {
    pub fn from(ptr: *mut c_void) -> WxDocChildFrame { WxDocChildFrame { ptr: ptr } }
    pub fn null() -> WxDocChildFrame { WxDocChildFrame::from(0 as *mut c_void) }
    
}

pub trait TWxDocChildFrame : TWxFrame {
}

pub struct WxDocMDIChildFrame { ptr: *mut c_void }
impl TWxDocMDIChildFrame for WxDocMDIChildFrame {}
impl TWxMDIChildFrame for WxDocMDIChildFrame {}
impl TWxFrame for WxDocMDIChildFrame {}
impl TWxTopLevelWindow for WxDocMDIChildFrame {}
impl TWxWindow for WxDocMDIChildFrame {}
impl TWxEvtHandler for WxDocMDIChildFrame {}
impl TWxObject for WxDocMDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocMDIChildFrame {
    pub fn from(ptr: *mut c_void) -> WxDocMDIChildFrame { WxDocMDIChildFrame { ptr: ptr } }
    pub fn null() -> WxDocMDIChildFrame { WxDocMDIChildFrame::from(0 as *mut c_void) }
    
}

pub trait TWxDocMDIChildFrame : TWxMDIChildFrame {
}

pub struct WxDocMDIParentFrame { ptr: *mut c_void }
impl TWxDocMDIParentFrame for WxDocMDIParentFrame {}
impl TWxMDIParentFrame for WxDocMDIParentFrame {}
impl TWxFrame for WxDocMDIParentFrame {}
impl TWxTopLevelWindow for WxDocMDIParentFrame {}
impl TWxWindow for WxDocMDIParentFrame {}
impl TWxEvtHandler for WxDocMDIParentFrame {}
impl TWxObject for WxDocMDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocMDIParentFrame {
    pub fn from(ptr: *mut c_void) -> WxDocMDIParentFrame { WxDocMDIParentFrame { ptr: ptr } }
    pub fn null() -> WxDocMDIParentFrame { WxDocMDIParentFrame::from(0 as *mut c_void) }
    
}

pub trait TWxDocMDIParentFrame : TWxMDIParentFrame {
}

pub struct WxDocManager { ptr: *mut c_void }
impl TWxDocManager for WxDocManager {}
impl TWxEvtHandler for WxDocManager {}
impl TWxObject for WxDocManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocManager {
    pub fn from(ptr: *mut c_void) -> WxDocManager { WxDocManager { ptr: ptr } }
    pub fn null() -> WxDocManager { WxDocManager::from(0 as *mut c_void) }
    
}

pub trait TWxDocManager : TWxEvtHandler {
}

pub struct WxDocParentFrame { ptr: *mut c_void }
impl TWxDocParentFrame for WxDocParentFrame {}
impl TWxFrame for WxDocParentFrame {}
impl TWxTopLevelWindow for WxDocParentFrame {}
impl TWxWindow for WxDocParentFrame {}
impl TWxEvtHandler for WxDocParentFrame {}
impl TWxObject for WxDocParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocParentFrame {
    pub fn from(ptr: *mut c_void) -> WxDocParentFrame { WxDocParentFrame { ptr: ptr } }
    pub fn null() -> WxDocParentFrame { WxDocParentFrame::from(0 as *mut c_void) }
    
}

pub trait TWxDocParentFrame : TWxFrame {
}

pub struct WxDocTemplate { ptr: *mut c_void }
impl TWxDocTemplate for WxDocTemplate {}
impl TWxObject for WxDocTemplate { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocTemplate {
    pub fn from(ptr: *mut c_void) -> WxDocTemplate { WxDocTemplate { ptr: ptr } }
    pub fn null() -> WxDocTemplate { WxDocTemplate::from(0 as *mut c_void) }
    
}

pub trait TWxDocTemplate : TWxObject {
}

pub struct WxDocument { ptr: *mut c_void }
impl TWxDocument for WxDocument {}
impl TWxEvtHandler for WxDocument {}
impl TWxObject for WxDocument { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDocument {
    pub fn from(ptr: *mut c_void) -> WxDocument { WxDocument { ptr: ptr } }
    pub fn null() -> WxDocument { WxDocument::from(0 as *mut c_void) }
    
}

pub trait TWxDocument : TWxEvtHandler {
}

pub struct WxDragImage { ptr: *mut c_void }
impl TWxDragImage for WxDragImage {}
impl TWxObject for WxDragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDragImage {
    pub fn from(ptr: *mut c_void) -> WxDragImage { WxDragImage { ptr: ptr } }
    pub fn null() -> WxDragImage { WxDragImage::from(0 as *mut c_void) }
    
    pub fn new<T: TWxBitmap>(image: &T, x: c_int, y: c_int) -> WxDragImage {
        unsafe { WxDragImage { ptr: wxDragImage_Create(image.ptr(), x, y) } }
    }
}

pub trait TWxDragImage : TWxObject {
    fn beginDragFullScreen<T: TWxWindow, U: TWxRect>(&self, x_pos: c_int, y_pos: c_int, window: &T, fullScreen: c_int, rect: &U) -> c_int {
        unsafe { wxDragImage_BeginDragFullScreen(self.ptr(), x_pos, y_pos, window.ptr(), fullScreen, rect.ptr()) }
    }
    fn beginDrag<T: TWxWindow, U: TWxWindow>(&self, x: c_int, y: c_int, window: &T, boundingWindow: &U) -> c_int {
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

pub struct WxDrawControl { ptr: *mut c_void }
impl TWxDrawControl for WxDrawControl {}
impl TWxControl for WxDrawControl {}
impl TWxWindow for WxDrawControl {}
impl TWxEvtHandler for WxDrawControl {}
impl TWxObject for WxDrawControl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDrawControl {
    pub fn from(ptr: *mut c_void) -> WxDrawControl { WxDrawControl { ptr: ptr } }
    pub fn null() -> WxDrawControl { WxDrawControl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxDrawControl {
        unsafe { WxDrawControl { ptr: wxDrawControl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxDrawControl : TWxControl {
}

pub struct WxDrawWindow { ptr: *mut c_void }
impl TWxDrawWindow for WxDrawWindow {}
impl TWxWindow for WxDrawWindow {}
impl TWxEvtHandler for WxDrawWindow {}
impl TWxObject for WxDrawWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDrawWindow {
    pub fn from(ptr: *mut c_void) -> WxDrawWindow { WxDrawWindow { ptr: ptr } }
    pub fn null() -> WxDrawWindow { WxDrawWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxDrawWindow {
        unsafe { WxDrawWindow { ptr: wxDrawWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxDrawWindow : TWxWindow {
}

pub struct WxDropFilesEvent { ptr: *mut c_void }
impl TWxDropFilesEvent for WxDropFilesEvent {}
impl TWxEvent for WxDropFilesEvent {}
impl TWxObject for WxDropFilesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDropFilesEvent {
    pub fn from(ptr: *mut c_void) -> WxDropFilesEvent { WxDropFilesEvent { ptr: ptr } }
    pub fn null() -> WxDropFilesEvent { WxDropFilesEvent::from(0 as *mut c_void) }
    
}

pub trait TWxDropFilesEvent : TWxEvent {
}

pub struct WxDropSource { ptr: *mut c_void }
impl TWxDropSource for WxDropSource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDropSource {
    pub fn from(ptr: *mut c_void) -> WxDropSource { WxDropSource { ptr: ptr } }
    pub fn null() -> WxDropSource { WxDropSource::from(0 as *mut c_void) }
    
}

pub trait TWxDropSource {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxDropTarget { ptr: *mut c_void }
impl TWxDropTarget for WxDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxDropTarget {
    pub fn from(ptr: *mut c_void) -> WxDropTarget { WxDropTarget { ptr: ptr } }
    pub fn null() -> WxDropTarget { WxDropTarget::from(0 as *mut c_void) }
    
}

pub trait TWxDropTarget {
    fn ptr(&self) -> *mut c_void;
    
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.ptr()) }
    }
    fn setDataObject<T: TWxDataObject>(&self, _dat: &T) {
        unsafe { wxDropTarget_SetDataObject(self.ptr(), _dat.ptr()) }
    }
}

pub struct WxEraseEvent { ptr: *mut c_void }
impl TWxEraseEvent for WxEraseEvent {}
impl TWxEvent for WxEraseEvent {}
impl TWxObject for WxEraseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxEraseEvent {
    pub fn from(ptr: *mut c_void) -> WxEraseEvent { WxEraseEvent { ptr: ptr } }
    pub fn null() -> WxEraseEvent { WxEraseEvent::from(0 as *mut c_void) }
    
}

pub trait TWxEraseEvent : TWxEvent {
    fn getDC(&self) -> WxDC {
        unsafe { WxDC { ptr: wxEraseEvent_GetDC(self.ptr()) } }
    }
}

pub struct WxEvent { ptr: *mut c_void }
impl TWxEvent for WxEvent {}
impl TWxObject for WxEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxEvent {
    pub fn from(ptr: *mut c_void) -> WxEvent { WxEvent { ptr: ptr } }
    pub fn null() -> WxEvent { WxEvent::from(0 as *mut c_void) }
    
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

pub trait TWxEvent : TWxObject {
    fn copyObject(&self, object_dest: *mut c_void) {
        unsafe { wxEvent_CopyObject(self.ptr(), object_dest) }
    }
    fn getEventObject(&self) -> WxObject {
        unsafe { WxObject { ptr: wxEvent_GetEventObject(self.ptr()) } }
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
    fn setEventObject<T: TWxObject>(&self, obj: &T) {
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

pub struct WxEvtHandler { ptr: *mut c_void }
impl TWxEvtHandler for WxEvtHandler {}
impl TWxObject for WxEvtHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxEvtHandler {
    pub fn from(ptr: *mut c_void) -> WxEvtHandler { WxEvtHandler { ptr: ptr } }
    pub fn null() -> WxEvtHandler { WxEvtHandler::from(0 as *mut c_void) }
    
    pub fn new() -> WxEvtHandler {
        unsafe { WxEvtHandler { ptr: wxEvtHandler_Create() } }
    }
}

pub trait TWxEvtHandler : TWxObject {
    fn addPendingEvent<T: TWxEvent>(&self, event: &T) {
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
    fn getNextHandler(&self) -> WxEvtHandler {
        unsafe { WxEvtHandler { ptr: wxEvtHandler_GetNextHandler(self.ptr()) } }
    }
    fn getPreviousHandler(&self) -> WxEvtHandler {
        unsafe { WxEvtHandler { ptr: wxEvtHandler_GetPreviousHandler(self.ptr()) } }
    }
    fn processEvent<T: TWxEvent>(&self, event: &T) -> c_int {
        unsafe { wxEvtHandler_ProcessEvent(self.ptr(), event.ptr()) }
    }
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.ptr()) }
    }
    fn setEvtHandlerEnabled(&self, enabled: c_int) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self.ptr(), enabled) }
    }
    fn setNextHandler<T: TWxEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetNextHandler(self.ptr(), handler.ptr()) }
    }
    fn setPreviousHandler<T: TWxEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.ptr(), handler.ptr()) }
    }
    fn getClosure(&self, id: c_int, type_: c_int) -> WxClosure {
        unsafe { WxClosure { ptr: wxEvtHandler_GetClosure(self.ptr(), id, type_) } }
    }
}

pub struct WxFileDataObject { ptr: *mut c_void }
impl TWxFileDataObject for WxFileDataObject {}
impl TWxDataObjectSimple for WxFileDataObject {}
impl TWxDataObject for WxFileDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileDataObject {
    pub fn from(ptr: *mut c_void) -> WxFileDataObject { WxFileDataObject { ptr: ptr } }
    pub fn null() -> WxFileDataObject { WxFileDataObject::from(0 as *mut c_void) }
    
}

pub trait TWxFileDataObject : TWxDataObjectSimple {
}

pub struct WxFileDialog { ptr: *mut c_void }
impl TWxFileDialog for WxFileDialog {}
impl TWxDialog for WxFileDialog {}
impl TWxTopLevelWindow for WxFileDialog {}
impl TWxWindow for WxFileDialog {}
impl TWxEvtHandler for WxFileDialog {}
impl TWxObject for WxFileDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileDialog {
    pub fn from(ptr: *mut c_void) -> WxFileDialog { WxFileDialog { ptr: ptr } }
    pub fn null() -> WxFileDialog { WxFileDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _msg: &str, _dir: &str, _fle: &str, _wcd: &str, _lft: c_int, _top: c_int, _stl: c_int) -> WxFileDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        let _fle = wxT(_fle);
        let _wcd = wxT(_wcd);
        unsafe { WxFileDialog { ptr: wxFileDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _fle.ptr(), _wcd.ptr(), _lft, _top, _stl) } }
    }
}

pub trait TWxFileDialog : TWxDialog {
    fn getDirectory(&self) -> ~str {
        unsafe { WxString { ptr: wxFileDialog_GetDirectory(self.ptr()) }.to_str() }
    }
    fn getFilename(&self) -> ~str {
        unsafe { WxString { ptr: wxFileDialog_GetFilename(self.ptr()) }.to_str() }
    }
    fn getFilenames(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetFilenames(self.ptr(), paths) }
    }
    fn getFilterIndex(&self) -> c_int {
        unsafe { wxFileDialog_GetFilterIndex(self.ptr()) }
    }
    fn getMessage(&self) -> ~str {
        unsafe { WxString { ptr: wxFileDialog_GetMessage(self.ptr()) }.to_str() }
    }
    fn getPath(&self) -> ~str {
        unsafe { WxString { ptr: wxFileDialog_GetPath(self.ptr()) }.to_str() }
    }
    fn getPaths(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetPaths(self.ptr(), paths) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxFileDialog_GetStyle(self.ptr()) }
    }
    fn getWildcard(&self) -> ~str {
        unsafe { WxString { ptr: wxFileDialog_GetWildcard(self.ptr()) }.to_str() }
    }
    fn setDirectory(&self, dir: &str) {
        let dir = wxT(dir);
        unsafe { wxFileDialog_SetDirectory(self.ptr(), dir.ptr()) }
    }
    fn setFilename(&self, name: &str) {
        let name = wxT(name);
        unsafe { wxFileDialog_SetFilename(self.ptr(), name.ptr()) }
    }
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.ptr(), filterIndex) }
    }
    fn setMessage(&self, message: &str) {
        let message = wxT(message);
        unsafe { wxFileDialog_SetMessage(self.ptr(), message.ptr()) }
    }
    fn setPath(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxFileDialog_SetPath(self.ptr(), path.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.ptr(), style) }
    }
    fn setWildcard(&self, wildCard: &str) {
        let wildCard = wxT(wildCard);
        unsafe { wxFileDialog_SetWildcard(self.ptr(), wildCard.ptr()) }
    }
}

pub struct WxFileDropTarget { ptr: *mut c_void }
impl TWxFileDropTarget for WxFileDropTarget {}
impl TWxDropTarget for WxFileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileDropTarget {
    pub fn from(ptr: *mut c_void) -> WxFileDropTarget { WxFileDropTarget { ptr: ptr } }
    pub fn null() -> WxFileDropTarget { WxFileDropTarget::from(0 as *mut c_void) }
    
}

pub trait TWxFileDropTarget : TWxDropTarget {
}

pub struct WxFileHistory { ptr: *mut c_void }
impl TWxFileHistory for WxFileHistory {}
impl TWxObject for WxFileHistory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileHistory {
    pub fn from(ptr: *mut c_void) -> WxFileHistory { WxFileHistory { ptr: ptr } }
    pub fn null() -> WxFileHistory { WxFileHistory::from(0 as *mut c_void) }
    
    pub fn new(maxFiles: c_int) -> WxFileHistory {
        unsafe { WxFileHistory { ptr: wxFileHistory_Create(maxFiles) } }
    }
}

pub trait TWxFileHistory : TWxObject {
    fn addFileToHistory(&self, file: &str) {
        let file = wxT(file);
        unsafe { wxFileHistory_AddFileToHistory(self.ptr(), file.ptr()) }
    }
    fn addFilesToMenu<T: TWxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_AddFilesToMenu(self.ptr(), menu.ptr()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxFileHistory_GetCount(self.ptr()) }
    }
    fn getHistoryFile(&self, i: c_int) -> ~str {
        unsafe { WxString { ptr: wxFileHistory_GetHistoryFile(self.ptr(), i) }.to_str() }
    }
    fn getMaxFiles(&self) -> c_int {
        unsafe { wxFileHistory_GetMaxFiles(self.ptr()) }
    }
    fn getMenus(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFileHistory_GetMenus(self.ptr(), _ref) }
    }
    fn load<T: TWxConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Load(self.ptr(), config.ptr()) }
    }
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.ptr(), i) }
    }
    fn removeMenu<T: TWxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_RemoveMenu(self.ptr(), menu.ptr()) }
    }
    fn save<T: TWxConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Save(self.ptr(), config.ptr()) }
    }
    fn useMenu<T: TWxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_UseMenu(self.ptr(), menu.ptr()) }
    }
}

pub struct WxFileType { ptr: *mut c_void }
impl TWxFileType for WxFileType { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFileType {
    pub fn from(ptr: *mut c_void) -> WxFileType { WxFileType { ptr: ptr } }
    pub fn null() -> WxFileType { WxFileType::from(0 as *mut c_void) }
    
}

pub trait TWxFileType {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.ptr()) }
    }
    fn expandCommand(&self, _cmd: *mut c_void, _params: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxFileType_ExpandCommand(self.ptr(), _cmd, _params) }.to_str() }
    }
    fn getDescription(&self) -> ~str {
        unsafe { WxString { ptr: wxFileType_GetDescription(self.ptr()) }.to_str() }
    }
    fn getExtensions<T: TWxList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetExtensions(self.ptr(), _lst.ptr()) }
    }
    fn getIcon<T: TWxIcon>(&self, icon: &T) -> c_int {
        unsafe { wxFileType_GetIcon(self.ptr(), icon.ptr()) }
    }
    fn getMimeType(&self) -> ~str {
        unsafe { WxString { ptr: wxFileType_GetMimeType(self.ptr()) }.to_str() }
    }
    fn getMimeTypes<T: TWxList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetMimeTypes(self.ptr(), _lst.ptr()) }
    }
    fn getOpenCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.ptr(), _buf, _params) }
    }
    fn getPrintCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.ptr(), _buf, _params) }
    }
}

pub struct WxFindDialogEvent { ptr: *mut c_void }
impl TWxFindDialogEvent for WxFindDialogEvent {}
impl TWxCommandEvent for WxFindDialogEvent {}
impl TWxEvent for WxFindDialogEvent {}
impl TWxObject for WxFindDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFindDialogEvent {
    pub fn from(ptr: *mut c_void) -> WxFindDialogEvent { WxFindDialogEvent { ptr: ptr } }
    pub fn null() -> WxFindDialogEvent { WxFindDialogEvent::from(0 as *mut c_void) }
    
}

pub trait TWxFindDialogEvent : TWxCommandEvent {
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

pub struct WxFindReplaceData { ptr: *mut c_void }
impl TWxFindReplaceData for WxFindReplaceData {}
impl TWxObject for WxFindReplaceData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFindReplaceData {
    pub fn from(ptr: *mut c_void) -> WxFindReplaceData { WxFindReplaceData { ptr: ptr } }
    pub fn null() -> WxFindReplaceData { WxFindReplaceData::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> WxFindReplaceData {
        unsafe { WxFindReplaceData { ptr: wxFindReplaceData_Create(flags) } }
    }
    pub fn newDefault() -> WxFindReplaceData {
        unsafe { WxFindReplaceData { ptr: wxFindReplaceData_CreateDefault() } }
    }
}

pub trait TWxFindReplaceData : TWxObject {
    fn getFindString(&self) -> ~str {
        unsafe { WxString { ptr: wxFindReplaceData_GetFindString(self.ptr()) }.to_str() }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.ptr()) }
    }
    fn getReplaceString(&self) -> ~str {
        unsafe { WxString { ptr: wxFindReplaceData_GetReplaceString(self.ptr()) }.to_str() }
    }
    fn setFindString(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxFindReplaceData_SetFindString(self.ptr(), str.ptr()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.ptr(), flags) }
    }
    fn setReplaceString(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxFindReplaceData_SetReplaceString(self.ptr(), str.ptr()) }
    }
}

pub struct WxFindReplaceDialog { ptr: *mut c_void }
impl TWxFindReplaceDialog for WxFindReplaceDialog {}
impl TWxDialog for WxFindReplaceDialog {}
impl TWxTopLevelWindow for WxFindReplaceDialog {}
impl TWxWindow for WxFindReplaceDialog {}
impl TWxEvtHandler for WxFindReplaceDialog {}
impl TWxObject for WxFindReplaceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFindReplaceDialog {
    pub fn from(ptr: *mut c_void) -> WxFindReplaceDialog { WxFindReplaceDialog { ptr: ptr } }
    pub fn null() -> WxFindReplaceDialog { WxFindReplaceDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxFindReplaceData>(parent: &T, data: &U, title: &str, style: c_int) -> WxFindReplaceDialog {
        let title = wxT(title);
        unsafe { WxFindReplaceDialog { ptr: wxFindReplaceDialog_Create(parent.ptr(), data.ptr(), title.ptr(), style) } }
    }
}

pub trait TWxFindReplaceDialog : TWxDialog {
    fn getData(&self) -> WxFindReplaceData {
        unsafe { WxFindReplaceData { ptr: wxFindReplaceDialog_GetData(self.ptr()) } }
    }
    fn setData<T: TWxFindReplaceData>(&self, data: &T) {
        unsafe { wxFindReplaceDialog_SetData(self.ptr(), data.ptr()) }
    }
}

pub struct WxFlexGridSizer { ptr: *mut c_void }
impl TWxFlexGridSizer for WxFlexGridSizer {}
impl TWxGridSizer for WxFlexGridSizer {}
impl TWxSizer for WxFlexGridSizer {}
impl TWxObject for WxFlexGridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFlexGridSizer {
    pub fn from(ptr: *mut c_void) -> WxFlexGridSizer { WxFlexGridSizer { ptr: ptr } }
    pub fn null() -> WxFlexGridSizer { WxFlexGridSizer::from(0 as *mut c_void) }
    
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> WxFlexGridSizer {
        unsafe { WxFlexGridSizer { ptr: wxFlexGridSizer_Create(rows, cols, vgap, hgap) } }
    }
}

pub trait TWxFlexGridSizer : TWxGridSizer {
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

pub struct WxFocusEvent { ptr: *mut c_void }
impl TWxFocusEvent for WxFocusEvent {}
impl TWxEvent for WxFocusEvent {}
impl TWxObject for WxFocusEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFocusEvent {
    pub fn from(ptr: *mut c_void) -> WxFocusEvent { WxFocusEvent { ptr: ptr } }
    pub fn null() -> WxFocusEvent { WxFocusEvent::from(0 as *mut c_void) }
    
}

pub trait TWxFocusEvent : TWxEvent {
}

pub struct WxFont { ptr: *mut c_void }
impl TWxFont for WxFont {}
impl TWxGDIObject for WxFont {}
impl TWxObject for WxFont { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFont {
    pub fn from(ptr: *mut c_void) -> WxFont { WxFont { ptr: ptr } }
    pub fn null() -> WxFont { WxFont::from(0 as *mut c_void) }
    
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: c_int, face: &str, enc: c_int) -> WxFont {
        let face = wxT(face);
        unsafe { WxFont { ptr: wxFont_Create(pointSize, family, style, weight, underlined, face.ptr(), enc) } }
    }
    pub fn newFromStock(id: c_int) -> WxFont {
        unsafe { WxFont { ptr: wxFont_CreateFromStock(id) } }
    }
    pub fn newDefault() -> WxFont {
        unsafe { WxFont { ptr: wxFont_CreateDefault() } }
    }
}

pub trait TWxFont : TWxGDIObject {
    fn getDefaultEncoding(&self) -> c_int {
        unsafe { wxFont_GetDefaultEncoding(self.ptr()) }
    }
    fn getEncoding(&self) -> c_int {
        unsafe { wxFont_GetEncoding(self.ptr()) }
    }
    fn getFaceName(&self) -> ~str {
        unsafe { WxString { ptr: wxFont_GetFaceName(self.ptr()) }.to_str() }
    }
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.ptr()) }
    }
    fn getFamilyString(&self) -> ~str {
        unsafe { WxString { ptr: wxFont_GetFamilyString(self.ptr()) }.to_str() }
    }
    fn getPointSize(&self) -> c_int {
        unsafe { wxFont_GetPointSize(self.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxFont_GetStyle(self.ptr()) }
    }
    fn getStyleString(&self) -> ~str {
        unsafe { WxString { ptr: wxFont_GetStyleString(self.ptr()) }.to_str() }
    }
    fn getUnderlined(&self) -> c_int {
        unsafe { wxFont_GetUnderlined(self.ptr()) }
    }
    fn getWeight(&self) -> c_int {
        unsafe { wxFont_GetWeight(self.ptr()) }
    }
    fn getWeightString(&self) -> ~str {
        unsafe { WxString { ptr: wxFont_GetWeightString(self.ptr()) }.to_str() }
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
        let faceName = wxT(faceName);
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

pub struct WxFontData { ptr: *mut c_void }
impl TWxFontData for WxFontData {}
impl TWxObject for WxFontData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFontData {
    pub fn from(ptr: *mut c_void) -> WxFontData { WxFontData { ptr: ptr } }
    pub fn null() -> WxFontData { WxFontData::from(0 as *mut c_void) }
    
    pub fn new() -> WxFontData {
        unsafe { WxFontData { ptr: wxFontData_Create() } }
    }
}

pub trait TWxFontData : TWxObject {
    fn enableEffects(&self, flag: c_int) {
        unsafe { wxFontData_EnableEffects(self.ptr(), flag) }
    }
    fn getAllowSymbols(&self) -> c_int {
        unsafe { wxFontData_GetAllowSymbols(self.ptr()) }
    }
    fn getChosenFont<T: TWxFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetChosenFont(self.ptr(), ref_.ptr()) }
    }
    fn getColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxFontData_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getEnableEffects(&self) -> c_int {
        unsafe { wxFontData_GetEnableEffects(self.ptr()) }
    }
    fn getEncoding(&self) -> c_int {
        unsafe { wxFontData_GetEncoding(self.ptr()) }
    }
    fn getInitialFont<T: TWxFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetInitialFont(self.ptr(), ref_.ptr()) }
    }
    fn getShowHelp(&self) -> c_int {
        unsafe { wxFontData_GetShowHelp(self.ptr()) }
    }
    fn setAllowSymbols(&self, flag: c_int) {
        unsafe { wxFontData_SetAllowSymbols(self.ptr(), flag) }
    }
    fn setChosenFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxFontData_SetChosenFont(self.ptr(), font.ptr()) }
    }
    fn setColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxFontData_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.ptr(), encoding) }
    }
    fn setInitialFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxFontData_SetInitialFont(self.ptr(), font.ptr()) }
    }
    fn setRange(&self, minRange: c_int, maxRange: c_int) {
        unsafe { wxFontData_SetRange(self.ptr(), minRange, maxRange) }
    }
    fn setShowHelp(&self, flag: c_int) {
        unsafe { wxFontData_SetShowHelp(self.ptr(), flag) }
    }
}

pub struct WxFontDialog { ptr: *mut c_void }
impl TWxFontDialog for WxFontDialog {}
impl TWxDialog for WxFontDialog {}
impl TWxTopLevelWindow for WxFontDialog {}
impl TWxWindow for WxFontDialog {}
impl TWxEvtHandler for WxFontDialog {}
impl TWxObject for WxFontDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFontDialog {
    pub fn from(ptr: *mut c_void) -> WxFontDialog { WxFontDialog { ptr: ptr } }
    pub fn null() -> WxFontDialog { WxFontDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxFontData>(_prt: &T, fnt: &U) -> WxFontDialog {
        unsafe { WxFontDialog { ptr: wxFontDialog_Create(_prt.ptr(), fnt.ptr()) } }
    }
}

pub trait TWxFontDialog : TWxDialog {
    fn getFontData<T: TWxFontData>(&self, _ref: &T) {
        unsafe { wxFontDialog_GetFontData(self.ptr(), _ref.ptr()) }
    }
}

pub struct WxFontEnumerator { ptr: *mut c_void }
impl TWxFontEnumerator for WxFontEnumerator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFontEnumerator {
    pub fn from(ptr: *mut c_void) -> WxFontEnumerator { WxFontEnumerator { ptr: ptr } }
    pub fn null() -> WxFontEnumerator { WxFontEnumerator::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> WxFontEnumerator {
        unsafe { WxFontEnumerator { ptr: wxFontEnumerator_Create(_obj, _fnc) } }
    }
}

pub trait TWxFontEnumerator {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.ptr()) }
    }
    fn enumerateEncodings(&self, facename: &str) -> c_int {
        let facename = wxT(facename);
        unsafe { wxFontEnumerator_EnumerateEncodings(self.ptr(), facename.ptr()) }
    }
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> c_int {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.ptr(), encoding, fixedWidthOnly) }
    }
}

pub struct WxFontList { ptr: *mut c_void }
impl TWxFontList for WxFontList {}
impl TWxList for WxFontList {}
impl TWxObject for WxFontList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFontList {
    pub fn from(ptr: *mut c_void) -> WxFontList { WxFontList { ptr: ptr } }
    pub fn null() -> WxFontList { WxFontList::from(0 as *mut c_void) }
    
}

pub trait TWxFontList : TWxList {
}

pub struct WxFontMapper { ptr: *mut c_void }
impl TWxFontMapper for WxFontMapper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFontMapper {
    pub fn from(ptr: *mut c_void) -> WxFontMapper { WxFontMapper { ptr: ptr } }
    pub fn null() -> WxFontMapper { WxFontMapper::from(0 as *mut c_void) }
    
    pub fn new() -> WxFontMapper {
        unsafe { WxFontMapper { ptr: wxFontMapper_Create() } }
    }
}

pub trait TWxFontMapper {
    fn ptr(&self) -> *mut c_void;
    
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *mut c_void, _buf: &str) -> c_int {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_GetAltForEncoding(self.ptr(), encoding, alt_encoding, _buf.ptr()) }
    }
    fn isEncodingAvailable(&self, encoding: c_int, _buf: &str) -> c_int {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_IsEncodingAvailable(self.ptr(), encoding, _buf.ptr()) }
    }
}

pub struct WxFrame { ptr: *mut c_void }
impl TWxFrame for WxFrame {}
impl TWxTopLevelWindow for WxFrame {}
impl TWxWindow for WxFrame {}
impl TWxEvtHandler for WxFrame {}
impl TWxObject for WxFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxFrame {
    pub fn from(ptr: *mut c_void) -> WxFrame { WxFrame { ptr: ptr } }
    pub fn null() -> WxFrame { WxFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxFrame {
        let _txt = wxT(_txt);
        unsafe { WxFrame { ptr: wxFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxFrame : TWxTopLevelWindow {
    fn newStatusBar(&self, number: c_int, style: c_int) -> WxStatusBar {
        unsafe { WxStatusBar { ptr: wxFrame_CreateStatusBar(self.ptr(), number, style) } }
    }
    fn newToolBar(&self, style: c_long) -> WxToolBar {
        unsafe { WxToolBar { ptr: wxFrame_CreateToolBar(self.ptr(), style) } }
    }
    fn getClientAreaOrigin_left(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self.ptr()) }
    }
    fn getClientAreaOrigin_top(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self.ptr()) }
    }
    fn getMenuBar(&self) -> WxMenuBar {
        unsafe { WxMenuBar { ptr: wxFrame_GetMenuBar(self.ptr()) } }
    }
    fn getStatusBar(&self) -> WxStatusBar {
        unsafe { WxStatusBar { ptr: wxFrame_GetStatusBar(self.ptr()) } }
    }
    fn getToolBar(&self) -> WxToolBar {
        unsafe { WxToolBar { ptr: wxFrame_GetToolBar(self.ptr()) } }
    }
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.ptr()) }
    }
    fn setMenuBar<T: TWxMenuBar>(&self, menubar: &T) {
        unsafe { wxFrame_SetMenuBar(self.ptr(), menubar.ptr()) }
    }
    fn setStatusBar<T: TWxStatusBar>(&self, statBar: &T) {
        unsafe { wxFrame_SetStatusBar(self.ptr(), statBar.ptr()) }
    }
    fn setStatusText(&self, _txt: &str, _number: c_int) {
        let _txt = wxT(_txt);
        unsafe { wxFrame_SetStatusText(self.ptr(), _txt.ptr(), _number) }
    }
    fn setStatusWidths(&self, _n: c_int, _widths_field: *mut c_void) {
        unsafe { wxFrame_SetStatusWidths(self.ptr(), _n, _widths_field) }
    }
    fn setToolBar<T: TWxToolBar>(&self, _toolbar: &T) {
        unsafe { wxFrame_SetToolBar(self.ptr(), _toolbar.ptr()) }
    }
    fn setShape<T: TWxRegion>(&self, region: &T) -> c_int {
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

pub struct WxGDIObject { ptr: *mut c_void }
impl TWxGDIObject for WxGDIObject {}
impl TWxObject for WxGDIObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGDIObject {
    pub fn from(ptr: *mut c_void) -> WxGDIObject { WxGDIObject { ptr: ptr } }
    pub fn null() -> WxGDIObject { WxGDIObject::from(0 as *mut c_void) }
    
}

pub trait TWxGDIObject : TWxObject {
}

pub struct WxGauge { ptr: *mut c_void }
impl TWxGauge for WxGauge {}
impl TWxControl for WxGauge {}
impl TWxWindow for WxGauge {}
impl TWxEvtHandler for WxGauge {}
impl TWxObject for WxGauge { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGauge {
    pub fn from(ptr: *mut c_void) -> WxGauge { WxGauge { ptr: ptr } }
    pub fn null() -> WxGauge { WxGauge::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxGauge {
        unsafe { WxGauge { ptr: wxGauge_Create(_prt.ptr(), _id, _rng, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxGauge : TWxControl {
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

pub struct WxGenericDirCtrl { ptr: *mut c_void }
impl TWxGenericDirCtrl for WxGenericDirCtrl {}
impl TWxControl for WxGenericDirCtrl {}
impl TWxWindow for WxGenericDirCtrl {}
impl TWxEvtHandler for WxGenericDirCtrl {}
impl TWxObject for WxGenericDirCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGenericDirCtrl {
    pub fn from(ptr: *mut c_void) -> WxGenericDirCtrl { WxGenericDirCtrl { ptr: ptr } }
    pub fn null() -> WxGenericDirCtrl { WxGenericDirCtrl::from(0 as *mut c_void) }
    
}

pub trait TWxGenericDirCtrl : TWxControl {
}

pub struct WxGenericValidator { ptr: *mut c_void }
impl TWxGenericValidator for WxGenericValidator {}
impl TWxValidator for WxGenericValidator {}
impl TWxEvtHandler for WxGenericValidator {}
impl TWxObject for WxGenericValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGenericValidator {
    pub fn from(ptr: *mut c_void) -> WxGenericValidator { WxGenericValidator { ptr: ptr } }
    pub fn null() -> WxGenericValidator { WxGenericValidator::from(0 as *mut c_void) }
    
}

pub trait TWxGenericValidator : TWxValidator {
}

pub struct WxGridSizer { ptr: *mut c_void }
impl TWxGridSizer for WxGridSizer {}
impl TWxSizer for WxGridSizer {}
impl TWxObject for WxGridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridSizer {
    pub fn from(ptr: *mut c_void) -> WxGridSizer { WxGridSizer { ptr: ptr } }
    pub fn null() -> WxGridSizer { WxGridSizer::from(0 as *mut c_void) }
    
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> WxGridSizer {
        unsafe { WxGridSizer { ptr: wxGridSizer_Create(rows, cols, vgap, hgap) } }
    }
}

pub trait TWxGridSizer : TWxSizer {
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

pub struct WxHelpController { ptr: *mut c_void }
impl TWxHelpController for WxHelpController {}
impl TWxHelpControllerBase for WxHelpController {}
impl TWxObject for WxHelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHelpController {
    pub fn from(ptr: *mut c_void) -> WxHelpController { WxHelpController { ptr: ptr } }
    pub fn null() -> WxHelpController { WxHelpController::from(0 as *mut c_void) }
    
}

pub trait TWxHelpController : TWxHelpControllerBase {
}

pub struct WxHelpControllerBase { ptr: *mut c_void }
impl TWxHelpControllerBase for WxHelpControllerBase {}
impl TWxObject for WxHelpControllerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHelpControllerBase {
    pub fn from(ptr: *mut c_void) -> WxHelpControllerBase { WxHelpControllerBase { ptr: ptr } }
    pub fn null() -> WxHelpControllerBase { WxHelpControllerBase::from(0 as *mut c_void) }
    
}

pub trait TWxHelpControllerBase : TWxObject {
}

pub struct WxHelpControllerHelpProvider { ptr: *mut c_void }
impl TWxHelpControllerHelpProvider for WxHelpControllerHelpProvider {}
impl TWxSimpleHelpProvider for WxHelpControllerHelpProvider {}
impl TWxHelpProvider for WxHelpControllerHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHelpControllerHelpProvider {
    pub fn from(ptr: *mut c_void) -> WxHelpControllerHelpProvider { WxHelpControllerHelpProvider { ptr: ptr } }
    pub fn null() -> WxHelpControllerHelpProvider { WxHelpControllerHelpProvider::from(0 as *mut c_void) }
    
    pub fn new<T: TWxHelpControllerBase>(ctr: &T) -> WxHelpControllerHelpProvider {
        unsafe { WxHelpControllerHelpProvider { ptr: wxHelpControllerHelpProvider_Create(ctr.ptr()) } }
    }
}

pub trait TWxHelpControllerHelpProvider : TWxSimpleHelpProvider {
    fn getHelpController(&self) -> WxHelpControllerBase {
        unsafe { WxHelpControllerBase { ptr: wxHelpControllerHelpProvider_GetHelpController(self.ptr()) } }
    }
    fn setHelpController<T: TWxHelpController>(&self, hc: &T) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.ptr(), hc.ptr()) }
    }
}

pub struct WxHelpEvent { ptr: *mut c_void }
impl TWxHelpEvent for WxHelpEvent {}
impl TWxCommandEvent for WxHelpEvent {}
impl TWxEvent for WxHelpEvent {}
impl TWxObject for WxHelpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHelpEvent {
    pub fn from(ptr: *mut c_void) -> WxHelpEvent { WxHelpEvent { ptr: ptr } }
    pub fn null() -> WxHelpEvent { WxHelpEvent::from(0 as *mut c_void) }
    
}

pub trait TWxHelpEvent : TWxCommandEvent {
    fn getLink(&self) -> ~str {
        unsafe { WxString { ptr: wxHelpEvent_GetLink(self.ptr()) }.to_str() }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxHelpEvent_GetPosition(self.ptr()) } }
    }
    fn getTarget(&self) -> ~str {
        unsafe { WxString { ptr: wxHelpEvent_GetTarget(self.ptr()) }.to_str() }
    }
    fn setLink(&self, link: &str) {
        let link = wxT(link);
        unsafe { wxHelpEvent_SetLink(self.ptr(), link.ptr()) }
    }
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.ptr(), x, y) }
    }
    fn setTarget(&self, target: &str) {
        let target = wxT(target);
        unsafe { wxHelpEvent_SetTarget(self.ptr(), target.ptr()) }
    }
}

pub struct WxHelpProvider { ptr: *mut c_void }
impl TWxHelpProvider for WxHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHelpProvider {
    pub fn from(ptr: *mut c_void) -> WxHelpProvider { WxHelpProvider { ptr: ptr } }
    pub fn null() -> WxHelpProvider { WxHelpProvider::from(0 as *mut c_void) }
    
    pub fn get() -> WxHelpProvider {
        unsafe { WxHelpProvider { ptr: wxHelpProvider_Get() } }
    }
}

pub trait TWxHelpProvider {
    fn ptr(&self) -> *mut c_void;
    
    fn addHelp<T: TWxWindow>(&self, window: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxHelpProvider_AddHelp(self.ptr(), window.ptr(), text.ptr()) }
    }
    fn addHelpById(&self, id: c_int, text: &str) {
        let text = wxT(text);
        unsafe { wxHelpProvider_AddHelpById(self.ptr(), id, text.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.ptr()) }
    }
    fn getHelp<T: TWxWindow>(&self, window: &T) -> ~str {
        unsafe { WxString { ptr: wxHelpProvider_GetHelp(self.ptr(), window.ptr()) }.to_str() }
    }
    fn removeHelp<T: TWxWindow>(&self, window: &T) {
        unsafe { wxHelpProvider_RemoveHelp(self.ptr(), window.ptr()) }
    }
    fn set(&self) -> WxHelpProvider {
        unsafe { WxHelpProvider { ptr: wxHelpProvider_Set(self.ptr()) } }
    }
    fn showHelp<T: TWxWindow>(&self, window: &T) -> c_int {
        unsafe { wxHelpProvider_ShowHelp(self.ptr(), window.ptr()) }
    }
}

pub struct WxIcon { ptr: *mut c_void }
impl TWxIcon for WxIcon {}
impl TWxBitmap for WxIcon {}
impl TWxGDIObject for WxIcon {}
impl TWxObject for WxIcon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIcon {
    pub fn from(ptr: *mut c_void) -> WxIcon { WxIcon { ptr: ptr } }
    pub fn null() -> WxIcon { WxIcon::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxIcon {
        unsafe { WxIcon { ptr: wxIcon_CreateDefault() } }
    }
    pub fn newLoad(name: &str, type_: c_long, width: c_int, height: c_int) -> WxIcon {
        let name = wxT(name);
        unsafe { WxIcon { ptr: wxIcon_CreateLoad(name.ptr(), type_, width, height) } }
    }
}

pub trait TWxIcon : TWxBitmap {
    fn assign(&self, other: *mut c_void) {
        unsafe { wxIcon_Assign(self.ptr(), other) }
    }
    fn copyFromBitmap<T: TWxBitmap>(&self, bmp: &T) {
        unsafe { wxIcon_CopyFromBitmap(self.ptr(), bmp.ptr()) }
    }
    fn fromRaw(&self, width: c_int, height: c_int) -> WxIcon {
        unsafe { WxIcon { ptr: wxIcon_FromRaw(self.ptr(), width, height) } }
    }
    fn fromXPM(&self) -> WxIcon {
        unsafe { WxIcon { ptr: wxIcon_FromXPM(self.ptr()) } }
    }
    fn isEqual(&self, other: &TWxIcon) -> c_int {
        unsafe { wxIcon_IsEqual(self.ptr(), other.ptr()) }
    }
    fn load(&self, name: &str, type_: c_long, width: c_int, height: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxIcon_Load(self.ptr(), name.ptr(), type_, width, height) }
    }
}

pub struct WxIconBundle { ptr: *mut c_void }
impl TWxIconBundle for WxIconBundle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIconBundle {
    pub fn from(ptr: *mut c_void) -> WxIconBundle { WxIconBundle { ptr: ptr } }
    pub fn null() -> WxIconBundle { WxIconBundle::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxIconBundle {
        unsafe { WxIconBundle { ptr: wxIconBundle_CreateDefault() } }
    }
    pub fn newFromFile(file: &str, type_: c_int) -> WxIconBundle {
        let file = wxT(file);
        unsafe { WxIconBundle { ptr: wxIconBundle_CreateFromFile(file.ptr(), type_) } }
    }
    pub fn newFromIcon<T: TWxIcon>(icon: &T) -> WxIconBundle {
        unsafe { WxIconBundle { ptr: wxIconBundle_CreateFromIcon(icon.ptr()) } }
    }
}

pub trait TWxIconBundle {
    fn ptr(&self) -> *mut c_void;
    
    fn addIcon<T: TWxIcon>(&self, icon: &T) {
        unsafe { wxIconBundle_AddIcon(self.ptr(), icon.ptr()) }
    }
    fn addIconFromFile(&self, file: &str, type_: c_int) {
        let file = wxT(file);
        unsafe { wxIconBundle_AddIconFromFile(self.ptr(), file.ptr(), type_) }
    }
    fn assign<T: TWxIconBundle>(&self, _ref: &T) {
        unsafe { wxIconBundle_Assign(self.ptr(), _ref.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.ptr()) }
    }
    fn getIcon<T: TWxIcon>(&self, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxIconBundle_GetIcon(self.ptr(), w, h, _ref.ptr()) }
    }
}

pub struct WxIconizeEvent { ptr: *mut c_void }
impl TWxIconizeEvent for WxIconizeEvent {}
impl TWxEvent for WxIconizeEvent {}
impl TWxObject for WxIconizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIconizeEvent {
    pub fn from(ptr: *mut c_void) -> WxIconizeEvent { WxIconizeEvent { ptr: ptr } }
    pub fn null() -> WxIconizeEvent { WxIconizeEvent::from(0 as *mut c_void) }
    
}

pub trait TWxIconizeEvent : TWxEvent {
}

pub struct WxIdleEvent { ptr: *mut c_void }
impl TWxIdleEvent for WxIdleEvent {}
impl TWxEvent for WxIdleEvent {}
impl TWxObject for WxIdleEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIdleEvent {
    pub fn from(ptr: *mut c_void) -> WxIdleEvent { WxIdleEvent { ptr: ptr } }
    pub fn null() -> WxIdleEvent { WxIdleEvent::from(0 as *mut c_void) }
    
}

pub trait TWxIdleEvent : TWxEvent {
    fn moreRequested(&self) -> c_int {
        unsafe { wxIdleEvent_MoreRequested(self.ptr()) }
    }
    fn requestMore(&self, needMore: c_int) {
        unsafe { wxIdleEvent_RequestMore(self.ptr(), needMore) }
    }
}

pub struct WxImage { ptr: *mut c_void }
impl TWxImage for WxImage {}
impl TWxObject for WxImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxImage {
    pub fn from(ptr: *mut c_void) -> WxImage { WxImage { ptr: ptr } }
    pub fn null() -> WxImage { WxImage::from(0 as *mut c_void) }
    
    pub fn canRead(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_CanRead(name.ptr()) }
    }
    pub fn newDefault() -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateDefault() } }
    }
    pub fn newFromBitmap<T: TWxBitmap>(bitmap: &T) -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateFromBitmap(bitmap.ptr()) } }
    }
    pub fn newFromByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateFromByteString(data, length, type_) } }
    }
    pub fn newFromLazyByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateFromLazyByteString(data, length, type_) } }
    }
    pub fn newFromData(width: c_int, height: c_int, data: *mut c_void) -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateFromData(width, height, data) } }
    }
    pub fn newFromFile(name: &str) -> WxImage {
        let name = wxT(name);
        unsafe { WxImage { ptr: wxImage_CreateFromFile(name.ptr()) } }
    }
    pub fn newSized(width: c_int, height: c_int) -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateSized(width, height) } }
    }
    pub fn newFromDataEx(width: c_int, height: c_int, data: *mut c_void, isStaticData: c_int) -> WxImage {
        unsafe { WxImage { ptr: wxImage_CreateFromDataEx(width, height, data, isStaticData) } }
    }
}

pub trait TWxImage : TWxObject {
    fn convertToBitmap<T: TWxBitmap>(&self, bitmap: &T) {
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
    fn getSubImage<T: TWxImage>(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: &T) {
        unsafe { wxImage_GetSubImage(self.ptr(), x, y, w, h, image.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxImage_GetWidth(self.ptr()) }
    }
    fn hasMask(&self) -> c_int {
        unsafe { wxImage_HasMask(self.ptr()) }
    }
    fn getOption(&self, name: &str) -> ~str {
        let name = wxT(name);
        unsafe { WxString { ptr: wxImage_GetOption(self.ptr(), name.ptr()) }.to_str() }
    }
    fn getOptionInt(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_GetOptionInt(self.ptr(), name.ptr()) }
    }
    fn hasOption(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_HasOption(self.ptr(), name.ptr()) }
    }
    fn initialize(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Initialize(self.ptr(), width, height) }
    }
    fn initializeFromData(&self, width: c_int, height: c_int, data: *mut c_void) {
        unsafe { wxImage_InitializeFromData(self.ptr(), width, height, data) }
    }
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_LoadFile(self.ptr(), name.ptr(), type_) }
    }
    fn mirror<T: TWxImage>(&self, horizontally: c_int, image: &T) {
        unsafe { wxImage_Mirror(self.ptr(), horizontally, image.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxImage_IsOk(self.ptr()) }
    }
    fn paste<T: TWxImage>(&self, image: &T, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self.ptr(), image.ptr(), x, y) }
    }
    fn replace(&self, r1: uint8_t, g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t, b2: uint8_t) {
        unsafe { wxImage_Replace(self.ptr(), r1, g1, b1, r2, g2, b2) }
    }
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self.ptr(), width, height) }
    }
    fn rotate<T: TWxImage>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *mut c_void, image: &T) {
        unsafe { wxImage_Rotate(self.ptr(), angle, c_x, c_y, interpolating, offset_after_rotation, image.ptr()) }
    }
    fn rotate90<T: TWxImage>(&self, clockwise: c_int, image: &T) {
        unsafe { wxImage_Rotate90(self.ptr(), clockwise, image.ptr()) }
    }
    fn saveFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_SaveFile(self.ptr(), name.ptr(), type_) }
    }
    fn scale<T: TWxImage>(&self, width: c_int, height: c_int, image: &T) {
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
        let name = wxT(name);
        let value = wxT(value);
        unsafe { wxImage_SetOption(self.ptr(), name.ptr(), value.ptr()) }
    }
    fn setOptionInt(&self, name: &str, value: c_int) {
        let name = wxT(name);
        unsafe { wxImage_SetOptionInt(self.ptr(), name.ptr(), value) }
    }
    fn setRGB(&self, x: c_int, y: c_int, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetRGB(self.ptr(), x, y, r, g, b) }
    }
}

pub struct WxImageHandler { ptr: *mut c_void }
impl TWxImageHandler for WxImageHandler {}
impl TWxObject for WxImageHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxImageHandler {
    pub fn from(ptr: *mut c_void) -> WxImageHandler { WxImageHandler { ptr: ptr } }
    pub fn null() -> WxImageHandler { WxImageHandler::from(0 as *mut c_void) }
    
}

pub trait TWxImageHandler : TWxObject {
}

pub struct WxImageList { ptr: *mut c_void }
impl TWxImageList for WxImageList {}
impl TWxObject for WxImageList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxImageList {
    pub fn from(ptr: *mut c_void) -> WxImageList { WxImageList { ptr: ptr } }
    pub fn null() -> WxImageList { WxImageList::from(0 as *mut c_void) }
    
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> WxImageList {
        unsafe { WxImageList { ptr: wxImageList_Create(width, height, mask, initialCount) } }
    }
}

pub trait TWxImageList : TWxObject {
    fn addBitmap<T: TWxBitmap, U: TWxBitmap>(&self, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_AddBitmap(self.ptr(), bitmap.ptr(), mask.ptr()) }
    }
    fn addIcon<T: TWxIcon>(&self, icon: &T) -> c_int {
        unsafe { wxImageList_AddIcon(self.ptr(), icon.ptr()) }
    }
    fn addMasked<T: TWxBitmap, U: TWxColour>(&self, bitmap: &T, maskColour: &U) -> c_int {
        unsafe { wxImageList_AddMasked(self.ptr(), bitmap.ptr(), maskColour.ptr()) }
    }
    fn draw<T: TWxDC>(&self, index: c_int, dc: &T, x: c_int, y: c_int, flags: c_int, solidBackground: c_int) -> c_int {
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
    fn replace<T: TWxBitmap, U: TWxBitmap>(&self, index: c_int, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_Replace(self.ptr(), index, bitmap.ptr(), mask.ptr()) }
    }
    fn replaceIcon<T: TWxIcon>(&self, index: c_int, icon: &T) -> c_int {
        unsafe { wxImageList_ReplaceIcon(self.ptr(), index, icon.ptr()) }
    }
}

pub struct WxIndividualLayoutConstraint { ptr: *mut c_void }
impl TWxIndividualLayoutConstraint for WxIndividualLayoutConstraint {}
impl TWxObject for WxIndividualLayoutConstraint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxIndividualLayoutConstraint {
    pub fn from(ptr: *mut c_void) -> WxIndividualLayoutConstraint { WxIndividualLayoutConstraint { ptr: ptr } }
    pub fn null() -> WxIndividualLayoutConstraint { WxIndividualLayoutConstraint::from(0 as *mut c_void) }
    
}

pub trait TWxIndividualLayoutConstraint : TWxObject {
    fn above<T: TWxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Above(self.ptr(), sibling.ptr(), marg) }
    }
    fn absolute(&self, val: c_int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self.ptr(), val) }
    }
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self.ptr()) }
    }
    fn below<T: TWxWindow>(&self, sibling: &T, marg: c_int) {
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
    fn leftOf<T: TWxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.ptr(), sibling.ptr(), marg) }
    }
    fn percentOf<T: TWxWindow>(&self, otherW: &T, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.ptr(), otherW.ptr(), wh, per) }
    }
    fn resetIfWin<T: TWxWindow>(&self, otherW: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.ptr(), otherW.ptr()) }
    }
    fn rightOf<T: TWxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.ptr(), sibling.ptr(), marg) }
    }
    fn sameAs<T: TWxWindow>(&self, otherW: &T, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.ptr(), otherW.ptr(), edge, marg) }
    }
    fn satisfyConstraint<T: TWxWindow>(&self, constraints: *mut c_void, win: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.ptr(), constraints, win.ptr()) }
    }
    fn set<T: TWxWindow>(&self, rel: c_int, otherW: &T, otherE: c_int, val: c_int, marg: c_int) {
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

pub struct WxInitDialogEvent { ptr: *mut c_void }
impl TWxInitDialogEvent for WxInitDialogEvent {}
impl TWxEvent for WxInitDialogEvent {}
impl TWxObject for WxInitDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxInitDialogEvent {
    pub fn from(ptr: *mut c_void) -> WxInitDialogEvent { WxInitDialogEvent { ptr: ptr } }
    pub fn null() -> WxInitDialogEvent { WxInitDialogEvent::from(0 as *mut c_void) }
    
}

pub trait TWxInitDialogEvent : TWxEvent {
}

pub struct WxJoystickEvent { ptr: *mut c_void }
impl TWxJoystickEvent for WxJoystickEvent {}
impl TWxEvent for WxJoystickEvent {}
impl TWxObject for WxJoystickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxJoystickEvent {
    pub fn from(ptr: *mut c_void) -> WxJoystickEvent { WxJoystickEvent { ptr: ptr } }
    pub fn null() -> WxJoystickEvent { WxJoystickEvent::from(0 as *mut c_void) }
    
}

pub trait TWxJoystickEvent : TWxEvent {
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
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxJoystickEvent_GetPosition(self.ptr()) } }
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

pub struct WxKeyEvent { ptr: *mut c_void }
impl TWxKeyEvent for WxKeyEvent {}
impl TWxEvent for WxKeyEvent {}
impl TWxObject for WxKeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxKeyEvent {
    pub fn from(ptr: *mut c_void) -> WxKeyEvent { WxKeyEvent { ptr: ptr } }
    pub fn null() -> WxKeyEvent { WxKeyEvent::from(0 as *mut c_void) }
    
}

pub trait TWxKeyEvent : TWxEvent {
    fn altDown(&self) -> c_int {
        unsafe { wxKeyEvent_AltDown(self.ptr()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxKeyEvent_ControlDown(self.ptr()) }
    }
    fn getKeyCode(&self) -> c_int {
        unsafe { wxKeyEvent_GetKeyCode(self.ptr()) }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxKeyEvent_GetPosition(self.ptr()) } }
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

pub struct WxLayoutConstraints { ptr: *mut c_void }
impl TWxLayoutConstraints for WxLayoutConstraints {}
impl TWxObject for WxLayoutConstraints { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLayoutConstraints {
    pub fn from(ptr: *mut c_void) -> WxLayoutConstraints { WxLayoutConstraints { ptr: ptr } }
    pub fn null() -> WxLayoutConstraints { WxLayoutConstraints::from(0 as *mut c_void) }
    
    pub fn new() -> WxLayoutConstraints {
        unsafe { WxLayoutConstraints { ptr: wxLayoutConstraints_Create() } }
    }
}

pub trait TWxLayoutConstraints : TWxObject {
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

pub struct WxListBox { ptr: *mut c_void }
impl TWxListBox for WxListBox {}
impl TWxControl for WxListBox {}
impl TWxWindow for WxListBox {}
impl TWxEvtHandler for WxListBox {}
impl TWxObject for WxListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxListBox {
    pub fn from(ptr: *mut c_void) -> WxListBox { WxListBox { ptr: ptr } }
    pub fn null() -> WxListBox { WxListBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> WxListBox {
        unsafe { WxListBox { ptr: wxListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TWxListBox : TWxControl {
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxListBox_Append(self.ptr(), item.ptr()) }
    }
    fn appendData(&self, item: &str, data: *mut c_void) {
        let item = wxT(item);
        unsafe { wxListBox_AppendData(self.ptr(), item.ptr(), data) }
    }
    fn clear(&self) {
        unsafe { wxListBox_Clear(self.ptr()) }
    }
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
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
        unsafe { WxString { ptr: wxListBox_GetString(self.ptr(), n) }.to_str() }
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
        let s = wxT(s);
        unsafe { wxListBox_SetString(self.ptr(), n, s.ptr()) }
    }
    fn setStringSelection(&self, str: &str, sel: c_int) {
        let str = wxT(str);
        unsafe { wxListBox_SetStringSelection(self.ptr(), str.ptr(), sel) }
    }
}

pub struct WxListCtrl { ptr: *mut c_void }
impl TWxListCtrl for WxListCtrl {}
impl TWxControl for WxListCtrl {}
impl TWxWindow for WxListCtrl {}
impl TWxEvtHandler for WxListCtrl {}
impl TWxObject for WxListCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxListCtrl {
    pub fn from(ptr: *mut c_void) -> WxListCtrl { WxListCtrl { ptr: ptr } }
    pub fn null() -> WxListCtrl { WxListCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxListCtrl {
        unsafe { WxListCtrl { ptr: wxListCtrl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxListCtrl : TWxControl {
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
        let str = wxT(str);
        unsafe { wxListCtrl_FindItem(self.ptr(), start, str.ptr(), partial) }
    }
    fn findItemByData(&self, start: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByData(self.ptr(), start, data) }
    }
    fn findItemByPosition(&self, start: c_int, x: c_int, y: c_int, direction: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByPosition(self.ptr(), start, x, y, direction) }
    }
    fn getColumn<T: TWxListItem>(&self, col: c_int, item: &T) -> c_int {
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
    fn getEditControl(&self) -> WxTextCtrl {
        unsafe { WxTextCtrl { ptr: wxListCtrl_GetEditControl(self.ptr()) } }
    }
    fn getImageList(&self, which: c_int) -> WxImageList {
        unsafe { WxImageList { ptr: wxListCtrl_GetImageList(self.ptr(), which) } }
    }
    fn getItem<T: TWxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_GetItem(self.ptr(), info.ptr()) }
    }
    fn getItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetItemCount(self.ptr()) }
    }
    fn getItemData(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemData(self.ptr(), item) }
    }
    fn getItemFont(&self, item: c_long) -> WxFont {
        unsafe { WxFont { ptr: wxListCtrl_GetItemFont(self.ptr(), item) } }
    }
    fn getItemPosition(&self, item: c_int) -> WxPoint {
        unsafe { WxPoint { ptr: wxListCtrl_GetItemPosition(self.ptr(), item) } }
    }
    fn getItemRect(&self, item: c_int, code: c_int) -> WxRect {
        unsafe { WxRect { ptr: wxListCtrl_GetItemRect(self.ptr(), item, code) } }
    }
    fn getItemSpacing(&self, isSmall: c_int) -> WxSize {
        unsafe { WxSize { ptr: wxListCtrl_GetItemSpacing(self.ptr(), isSmall) } }
    }
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.ptr(), item, stateMask) }
    }
    fn getItemText(&self, item: c_int) -> ~str {
        unsafe { WxString { ptr: wxListCtrl_GetItemText(self.ptr(), item) }.to_str() }
    }
    fn getNextItem(&self, item: c_int, geometry: c_int, state: c_int) -> c_int {
        unsafe { wxListCtrl_GetNextItem(self.ptr(), item, geometry, state) }
    }
    fn getSelectedItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetSelectedItemCount(self.ptr()) }
    }
    fn getTextColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxListCtrl_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getTopItem(&self) -> c_int {
        unsafe { wxListCtrl_GetTopItem(self.ptr()) }
    }
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_void) -> c_int {
        unsafe { wxListCtrl_HitTest(self.ptr(), x, y, flags) }
    }
    fn insertColumn(&self, col: c_int, heading: &str, format: c_int, width: c_int) -> c_int {
        let heading = wxT(heading);
        unsafe { wxListCtrl_InsertColumn(self.ptr(), col, heading.ptr(), format, width) }
    }
    fn insertColumnFromInfo<T: TWxListItem>(&self, col: c_int, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.ptr(), col, info.ptr()) }
    }
    fn insertItem<T: TWxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.ptr(), info.ptr()) }
    }
    fn insertItemWithData(&self, index: c_int, label: &str) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_InsertItemWithData(self.ptr(), index, label.ptr()) }
    }
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.ptr(), index, imageIndex) }
    }
    fn insertItemWithLabel(&self, index: c_int, label: &str, imageIndex: c_int) -> c_int {
        let label = wxT(label);
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
    fn setColumn<T: TWxListItem>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_SetColumn(self.ptr(), col, item.ptr()) }
    }
    fn setColumnWidth(&self, col: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_SetColumnWidth(self.ptr(), col, width) }
    }
    fn setImageList<T: TWxImageList>(&self, imageList: &T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.ptr(), imageList.ptr(), which) }
    }
    fn setItem(&self, index: c_int, col: c_int, label: &str, imageId: c_int) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_SetItem(self.ptr(), index, col, label.ptr(), imageId) }
    }
    fn setItemData(&self, item: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemData(self.ptr(), item, data) }
    }
    fn setItemFromInfo<T: TWxListItem>(&self, info: &T) -> c_int {
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
        let str = wxT(str);
        unsafe { wxListCtrl_SetItemText(self.ptr(), item, str.ptr()) }
    }
    fn setSingleStyle(&self, style: c_int, add: c_int) {
        unsafe { wxListCtrl_SetSingleStyle(self.ptr(), style, add) }
    }
    fn setTextColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxListCtrl_SetTextColour(self.ptr(), col.ptr()) }
    }
    fn sortItems(&self, fn_: *mut c_void, eif_obj: *mut c_void) -> c_int {
        unsafe { wxListCtrl_SortItems(self.ptr(), fn_, eif_obj) }
    }
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self.ptr()) }
    }
    fn assignImageList<T: TWxImageList>(&self, images: &T, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.ptr(), images.ptr(), which) }
    }
    fn getColumn2<T: TWxListItem>(&self, col: c_int, item: &T) {
        unsafe { wxListCtrl_GetColumn2(self.ptr(), col, item.ptr()) }
    }
    fn getItem2<T: TWxListItem>(&self, info: &T) {
        unsafe { wxListCtrl_GetItem2(self.ptr(), info.ptr()) }
    }
    fn getItemPosition2(&self, item: c_int) -> WxPoint {
        unsafe { WxPoint { ptr: wxListCtrl_GetItemPosition2(self.ptr(), item) } }
    }
    fn sortItems2<T: TWxClosure>(&self, closure: &T) -> c_int {
        unsafe { wxListCtrl_SortItems2(self.ptr(), closure.ptr()) }
    }
}

pub struct WxListEvent { ptr: *mut c_void }
impl TWxListEvent for WxListEvent {}
impl TWxNotifyEvent for WxListEvent {}
impl TWxCommandEvent for WxListEvent {}
impl TWxEvent for WxListEvent {}
impl TWxObject for WxListEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxListEvent {
    pub fn from(ptr: *mut c_void) -> WxListEvent { WxListEvent { ptr: ptr } }
    pub fn null() -> WxListEvent { WxListEvent::from(0 as *mut c_void) }
    
}

pub trait TWxListEvent : TWxNotifyEvent {
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
    fn getItem<T: TWxListItem>(&self, _ref: &T) {
        unsafe { wxListEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxListEvent_GetLabel(self.ptr()) }.to_str() }
    }
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.ptr()) }
    }
    fn getPoint(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxListEvent_GetPoint(self.ptr()) } }
    }
    fn getText(&self) -> ~str {
        unsafe { WxString { ptr: wxListEvent_GetText(self.ptr()) }.to_str() }
    }
    fn getCacheFrom(&self) -> c_int {
        unsafe { wxListEvent_GetCacheFrom(self.ptr()) }
    }
    fn getCacheTo(&self) -> c_int {
        unsafe { wxListEvent_GetCacheTo(self.ptr()) }
    }
}

pub struct WxListItem { ptr: *mut c_void }
impl TWxListItem for WxListItem {}
impl TWxObject for WxListItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxListItem {
    pub fn from(ptr: *mut c_void) -> WxListItem { WxListItem { ptr: ptr } }
    pub fn null() -> WxListItem { WxListItem::from(0 as *mut c_void) }
    
    pub fn new() -> WxListItem {
        unsafe { WxListItem { ptr: wxListItem_Create() } }
    }
}

pub trait TWxListItem : TWxObject {
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
    fn getBackgroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxListItem_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getColumn(&self) -> c_int {
        unsafe { wxListItem_GetColumn(self.ptr()) }
    }
    fn getData(&self) -> c_int {
        unsafe { wxListItem_GetData(self.ptr()) }
    }
    fn getFont<T: TWxFont>(&self, _ref: &T) {
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
        unsafe { WxString { ptr: wxListItem_GetText(self.ptr()) }.to_str() }
    }
    fn getTextColour<T: TWxColour>(&self, _ref: &T) {
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
    fn setBackgroundColour<T: TWxColour>(&self, colBack: &T) {
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
    fn setFont<T: TWxFont>(&self, font: &T) {
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
        let text = wxT(text);
        unsafe { wxListItem_SetText(self.ptr(), text.ptr()) }
    }
    fn setTextColour<T: TWxColour>(&self, colText: &T) {
        unsafe { wxListItem_SetTextColour(self.ptr(), colText.ptr()) }
    }
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.ptr(), width) }
    }
}

pub struct WxLog { ptr: *mut c_void }
impl TWxLog for WxLog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLog {
    pub fn from(ptr: *mut c_void) -> WxLog { WxLog { ptr: ptr } }
    pub fn null() -> WxLog { WxLog::from(0 as *mut c_void) }
    
    pub fn getActiveTarget() -> WxLog {
        unsafe { WxLog { ptr: wxLog_GetActiveTarget() } }
    }
}

pub trait TWxLog {
    fn ptr(&self) -> *mut c_void;
    
    fn addTraceMask(&self, str: &str) {
        let str = wxT(str);
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
    fn isAllowedTraceMask<T: TWxMask>(&self, mask: &T) -> c_int {
        unsafe { wxLog_IsAllowedTraceMask(self.ptr(), mask.ptr()) }
    }
    fn onLog(&self, level: c_int, szString: *mut c_void, t: c_int) {
        unsafe { wxLog_OnLog(self.ptr(), level, szString, t) }
    }
    fn removeTraceMask(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxLog_RemoveTraceMask(self.ptr(), str.ptr()) }
    }
    fn resume(&self) {
        unsafe { wxLog_Resume(self.ptr()) }
    }
    fn setActiveTarget(&self) -> WxLog {
        unsafe { WxLog { ptr: wxLog_SetActiveTarget(self.ptr()) } }
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

pub struct WxLogChain { ptr: *mut c_void }
impl TWxLogChain for WxLogChain {}
impl TWxLog for WxLogChain { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogChain {
    pub fn from(ptr: *mut c_void) -> WxLogChain { WxLogChain { ptr: ptr } }
    pub fn null() -> WxLogChain { WxLogChain::from(0 as *mut c_void) }
    
    pub fn new<T: TWxLog>(logger: &T) -> WxLogChain {
        unsafe { WxLogChain { ptr: wxLogChain_Create(logger.ptr()) } }
    }
}

pub trait TWxLogChain : TWxLog {
    fn getOldLog(&self) -> WxLog {
        unsafe { WxLog { ptr: wxLogChain_GetOldLog(self.ptr()) } }
    }
    fn isPassingMessages(&self) -> c_int {
        unsafe { wxLogChain_IsPassingMessages(self.ptr()) }
    }
    fn passMessages(&self, bDoPass: c_int) {
        unsafe { wxLogChain_PassMessages(self.ptr(), bDoPass) }
    }
    fn setLog<T: TWxLog>(&self, logger: &T) {
        unsafe { wxLogChain_SetLog(self.ptr(), logger.ptr()) }
    }
}

pub struct WxLogGUI { ptr: *mut c_void }
impl TWxLogGUI for WxLogGUI {}
impl TWxLog for WxLogGUI { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogGUI {
    pub fn from(ptr: *mut c_void) -> WxLogGUI { WxLogGUI { ptr: ptr } }
    pub fn null() -> WxLogGUI { WxLogGUI::from(0 as *mut c_void) }
    
}

pub trait TWxLogGUI : TWxLog {
}

pub struct WxLogNull { ptr: *mut c_void }
impl TWxLogNull for WxLogNull {}
impl TWxLog for WxLogNull { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogNull {
    pub fn from(ptr: *mut c_void) -> WxLogNull { WxLogNull { ptr: ptr } }
    pub fn null() -> WxLogNull { WxLogNull::from(0 as *mut c_void) }
    
    pub fn new() -> WxLogNull {
        unsafe { WxLogNull { ptr: wxLogNull_Create() } }
    }
}

pub trait TWxLogNull : TWxLog {
}

pub struct WxLogPassThrough { ptr: *mut c_void }
impl TWxLogPassThrough for WxLogPassThrough {}
impl TWxLogChain for WxLogPassThrough {}
impl TWxLog for WxLogPassThrough { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogPassThrough {
    pub fn from(ptr: *mut c_void) -> WxLogPassThrough { WxLogPassThrough { ptr: ptr } }
    pub fn null() -> WxLogPassThrough { WxLogPassThrough::from(0 as *mut c_void) }
    
}

pub trait TWxLogPassThrough : TWxLogChain {
}

pub struct WxLogStderr { ptr: *mut c_void }
impl TWxLogStderr for WxLogStderr {}
impl TWxLog for WxLogStderr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogStderr {
    pub fn from(ptr: *mut c_void) -> WxLogStderr { WxLogStderr { ptr: ptr } }
    pub fn null() -> WxLogStderr { WxLogStderr::from(0 as *mut c_void) }
    
    pub fn new() -> WxLogStderr {
        unsafe { WxLogStderr { ptr: wxLogStderr_Create() } }
    }
    pub fn newStdOut() -> WxLogStderr {
        unsafe { WxLogStderr { ptr: wxLogStderr_CreateStdOut() } }
    }
}

pub trait TWxLogStderr : TWxLog {
}

pub struct WxLogStream { ptr: *mut c_void }
impl TWxLogStream for WxLogStream {}
impl TWxLog for WxLogStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogStream {
    pub fn from(ptr: *mut c_void) -> WxLogStream { WxLogStream { ptr: ptr } }
    pub fn null() -> WxLogStream { WxLogStream::from(0 as *mut c_void) }
    
}

pub trait TWxLogStream : TWxLog {
}

pub struct WxLogTextCtrl { ptr: *mut c_void }
impl TWxLogTextCtrl for WxLogTextCtrl {}
impl TWxLog for WxLogTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogTextCtrl {
    pub fn from(ptr: *mut c_void) -> WxLogTextCtrl { WxLogTextCtrl { ptr: ptr } }
    pub fn null() -> WxLogTextCtrl { WxLogTextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxTextCtrl>(text: &T) -> WxLogTextCtrl {
        unsafe { WxLogTextCtrl { ptr: wxLogTextCtrl_Create(text.ptr()) } }
    }
}

pub trait TWxLogTextCtrl : TWxLog {
}

pub struct WxLogWindow { ptr: *mut c_void }
impl TWxLogWindow for WxLogWindow {}
impl TWxLogPassThrough for WxLogWindow {}
impl TWxLogChain for WxLogWindow {}
impl TWxLog for WxLogWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLogWindow {
    pub fn from(ptr: *mut c_void) -> WxLogWindow { WxLogWindow { ptr: ptr } }
    pub fn null() -> WxLogWindow { WxLogWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(parent: &T, title: *mut int8_t, showit: c_int, passthrough: c_int) -> WxLogWindow {
        unsafe { WxLogWindow { ptr: wxLogWindow_Create(parent.ptr(), title, showit, passthrough) } }
    }
}

pub trait TWxLogWindow : TWxLogPassThrough {
    fn getFrame(&self) -> WxFrame {
        unsafe { WxFrame { ptr: wxLogWindow_GetFrame(self.ptr()) } }
    }
}

pub struct WxMDIChildFrame { ptr: *mut c_void }
impl TWxMDIChildFrame for WxMDIChildFrame {}
impl TWxFrame for WxMDIChildFrame {}
impl TWxTopLevelWindow for WxMDIChildFrame {}
impl TWxWindow for WxMDIChildFrame {}
impl TWxEvtHandler for WxMDIChildFrame {}
impl TWxObject for WxMDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMDIChildFrame {
    pub fn from(ptr: *mut c_void) -> WxMDIChildFrame { WxMDIChildFrame { ptr: ptr } }
    pub fn null() -> WxMDIChildFrame { WxMDIChildFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxMDIChildFrame {
        let _txt = wxT(_txt);
        unsafe { WxMDIChildFrame { ptr: wxMDIChildFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxMDIChildFrame : TWxFrame {
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.ptr()) }
    }
}

pub struct WxMDIClientWindow { ptr: *mut c_void }
impl TWxMDIClientWindow for WxMDIClientWindow {}
impl TWxWindow for WxMDIClientWindow {}
impl TWxEvtHandler for WxMDIClientWindow {}
impl TWxObject for WxMDIClientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMDIClientWindow {
    pub fn from(ptr: *mut c_void) -> WxMDIClientWindow { WxMDIClientWindow { ptr: ptr } }
    pub fn null() -> WxMDIClientWindow { WxMDIClientWindow::from(0 as *mut c_void) }
    
}

pub trait TWxMDIClientWindow : TWxWindow {
}

pub struct WxMDIParentFrame { ptr: *mut c_void }
impl TWxMDIParentFrame for WxMDIParentFrame {}
impl TWxFrame for WxMDIParentFrame {}
impl TWxTopLevelWindow for WxMDIParentFrame {}
impl TWxWindow for WxMDIParentFrame {}
impl TWxEvtHandler for WxMDIParentFrame {}
impl TWxObject for WxMDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMDIParentFrame {
    pub fn from(ptr: *mut c_void) -> WxMDIParentFrame { WxMDIParentFrame { ptr: ptr } }
    pub fn null() -> WxMDIParentFrame { WxMDIParentFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxMDIParentFrame {
        let _txt = wxT(_txt);
        unsafe { WxMDIParentFrame { ptr: wxMDIParentFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxMDIParentFrame : TWxFrame {
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
    fn getActiveChild(&self) -> WxMDIChildFrame {
        unsafe { WxMDIChildFrame { ptr: wxMDIParentFrame_GetActiveChild(self.ptr()) } }
    }
    fn getClientWindow(&self) -> WxMDIClientWindow {
        unsafe { WxMDIClientWindow { ptr: wxMDIParentFrame_GetClientWindow(self.ptr()) } }
    }
    fn getWindowMenu(&self) -> WxMenu {
        unsafe { WxMenu { ptr: wxMDIParentFrame_GetWindowMenu(self.ptr()) } }
    }
    fn onCreateClient(&self) -> WxMDIClientWindow {
        unsafe { WxMDIClientWindow { ptr: wxMDIParentFrame_OnCreateClient(self.ptr()) } }
    }
    fn setWindowMenu<T: TWxMenu>(&self, menu: &T) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.ptr(), menu.ptr()) }
    }
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.ptr()) }
    }
}

pub struct WxMask { ptr: *mut c_void }
impl TWxMask for WxMask {}
impl TWxObject for WxMask { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMask {
    pub fn from(ptr: *mut c_void) -> WxMask { WxMask { ptr: ptr } }
    pub fn null() -> WxMask { WxMask::from(0 as *mut c_void) }
    
    pub fn new<T: TWxBitmap>(bitmap: &T) -> WxMask {
        unsafe { WxMask { ptr: wxMask_Create(bitmap.ptr()) } }
    }
    pub fn newColoured<T: TWxBitmap, U: TWxColour>(bitmap: &T, colour: &U) -> *mut c_void {
        unsafe { wxMask_CreateColoured(bitmap.ptr(), colour.ptr()) }
    }
}

pub trait TWxMask : TWxObject {
}

pub struct WxMaximizeEvent { ptr: *mut c_void }
impl TWxMaximizeEvent for WxMaximizeEvent {}
impl TWxEvent for WxMaximizeEvent {}
impl TWxObject for WxMaximizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMaximizeEvent {
    pub fn from(ptr: *mut c_void) -> WxMaximizeEvent { WxMaximizeEvent { ptr: ptr } }
    pub fn null() -> WxMaximizeEvent { WxMaximizeEvent::from(0 as *mut c_void) }
    
}

pub trait TWxMaximizeEvent : TWxEvent {
}

pub struct WxMemoryDC { ptr: *mut c_void }
impl TWxMemoryDC for WxMemoryDC {}
impl TWxDC for WxMemoryDC {}
impl TWxObject for WxMemoryDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMemoryDC {
    pub fn from(ptr: *mut c_void) -> WxMemoryDC { WxMemoryDC { ptr: ptr } }
    pub fn null() -> WxMemoryDC { WxMemoryDC::from(0 as *mut c_void) }
    
    pub fn new() -> WxMemoryDC {
        unsafe { WxMemoryDC { ptr: wxMemoryDC_Create() } }
    }
    pub fn newCompatible<T: TWxDC>(dc: &T) -> WxMemoryDC {
        unsafe { WxMemoryDC { ptr: wxMemoryDC_CreateCompatible(dc.ptr()) } }
    }
    pub fn newWithBitmap<T: TWxBitmap>(bitmap: &T) -> WxMemoryDC {
        unsafe { WxMemoryDC { ptr: wxMemoryDC_CreateWithBitmap(bitmap.ptr()) } }
    }
}

pub trait TWxMemoryDC : TWxDC {
    fn selectObject<T: TWxBitmap>(&self, bitmap: &T) {
        unsafe { wxMemoryDC_SelectObject(self.ptr(), bitmap.ptr()) }
    }
}

pub struct WxMenu { ptr: *mut c_void }
impl TWxMenu for WxMenu {}
impl TWxEvtHandler for WxMenu {}
impl TWxObject for WxMenu { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMenu {
    pub fn from(ptr: *mut c_void) -> WxMenu { WxMenu { ptr: ptr } }
    pub fn null() -> WxMenu { WxMenu::from(0 as *mut c_void) }
    
    pub fn new(title: &str, style: c_long) -> WxMenu {
        let title = wxT(title);
        unsafe { WxMenu { ptr: wxMenu_Create(title.ptr(), style) } }
    }
}

pub trait TWxMenu : TWxEvtHandler {
    fn append(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Append(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn appendItem<T: TWxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_AppendItem(self.ptr(), _itm.ptr()) }
    }
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.ptr()) }
    }
    fn appendSub<T: TWxMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
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
    fn deleteByItem<T: TWxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DeleteByItem(self.ptr(), _itm.ptr()) }
    }
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self.ptr()) }
    }
    fn destroyById(&self, id: c_int) {
        unsafe { wxMenu_DestroyById(self.ptr(), id) }
    }
    fn destroyByItem<T: TWxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DestroyByItem(self.ptr(), _itm.ptr()) }
    }
    fn enable(&self, id: c_int, enable: c_int) {
        unsafe { wxMenu_Enable(self.ptr(), id, enable) }
    }
    fn findItem(&self, id: c_int) -> WxMenuItem {
        unsafe { WxMenuItem { ptr: wxMenu_FindItem(self.ptr(), id) } }
    }
    fn findItemByLabel(&self, itemString: &str) -> c_int {
        let itemString = wxT(itemString);
        unsafe { wxMenu_FindItemByLabel(self.ptr(), itemString.ptr()) }
    }
    fn getClientData(&self) -> WxClientData {
        unsafe { WxClientData { ptr: wxMenu_GetClientData(self.ptr()) } }
    }
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenu_GetHelpString(self.ptr(), id) }.to_str() }
    }
    fn getInvokingWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxMenu_GetInvokingWindow(self.ptr()) } }
    }
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenu_GetLabel(self.ptr(), id) }.to_str() }
    }
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.ptr()) }
    }
    fn getMenuItems<T: TWxList>(&self, _lst: &T) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.ptr(), _lst.ptr()) }
    }
    fn getParent(&self) -> WxMenu {
        unsafe { WxMenu { ptr: wxMenu_GetParent(self.ptr()) } }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.ptr()) }
    }
    fn getTitle(&self) -> ~str {
        unsafe { WxString { ptr: wxMenu_GetTitle(self.ptr()) }.to_str() }
    }
    fn insert(&self, pos: size_t, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Insert(self.ptr(), pos, id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn insertItem<T: TWxMenuItem>(&self, pos: size_t, _itm: &T) {
        unsafe { wxMenu_InsertItem(self.ptr(), pos, _itm.ptr()) }
    }
    fn insertSub<T: TWxMenu>(&self, pos: size_t, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
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
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Prepend(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn prependItem<T: TWxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_PrependItem(self.ptr(), _itm.ptr()) }
    }
    fn prependSub<T: TWxMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_PrependSub(self.ptr(), id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    fn removeById<T: TWxMenuItem>(&self, id: c_int, _itm: &T) {
        unsafe { wxMenu_RemoveById(self.ptr(), id, _itm.ptr()) }
    }
    fn removeByItem(&self, item: *mut c_void) {
        unsafe { wxMenu_RemoveByItem(self.ptr(), item) }
    }
    fn setClientData<T: TWxClientData>(&self, clientData: &T) {
        unsafe { wxMenu_SetClientData(self.ptr(), clientData.ptr()) }
    }
    fn setEventHandler<T: TWxEvtHandler>(&self, handler: &T) {
        unsafe { wxMenu_SetEventHandler(self.ptr(), handler.ptr()) }
    }
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenu_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    fn setInvokingWindow<T: TWxWindow>(&self, win: &T) {
        unsafe { wxMenu_SetInvokingWindow(self.ptr(), win.ptr()) }
    }
    fn setLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenu_SetLabel(self.ptr(), id, label.ptr()) }
    }
    fn setParent<T: TWxWindow>(&self, parent: &T) {
        unsafe { wxMenu_SetParent(self.ptr(), parent.ptr()) }
    }
    fn setTitle(&self, title: &str) {
        let title = wxT(title);
        unsafe { wxMenu_SetTitle(self.ptr(), title.ptr()) }
    }
    fn updateUI(&self, source: *mut c_void) {
        unsafe { wxMenu_UpdateUI(self.ptr(), source) }
    }
    fn getMenuBar(&self) -> WxMenuBar {
        unsafe { WxMenuBar { ptr: wxMenu_GetMenuBar(self.ptr()) } }
    }
    fn appendRadioItem(&self, id: c_int, text: &str, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_AppendRadioItem(self.ptr(), id, text.ptr(), help.ptr()) }
    }
}

pub struct WxMenuBar { ptr: *mut c_void }
impl TWxMenuBar for WxMenuBar {}
impl TWxEvtHandler for WxMenuBar {}
impl TWxObject for WxMenuBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMenuBar {
    pub fn from(ptr: *mut c_void) -> WxMenuBar { WxMenuBar { ptr: ptr } }
    pub fn null() -> WxMenuBar { WxMenuBar::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> WxMenuBar {
        unsafe { WxMenuBar { ptr: wxMenuBar_Create(_style) } }
    }
}

pub trait TWxMenuBar : TWxEvtHandler {
    fn append<T: TWxMenu>(&self, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
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
    fn findItem(&self, id: c_int) -> WxMenuItem {
        unsafe { WxMenuItem { ptr: wxMenuBar_FindItem(self.ptr(), id) } }
    }
    fn findMenu(&self, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_FindMenu(self.ptr(), title.ptr()) }
    }
    fn findMenuItem(&self, menuString: &str, itemString: &str) -> c_int {
        let menuString = wxT(menuString);
        let itemString = wxT(itemString);
        unsafe { wxMenuBar_FindMenuItem(self.ptr(), menuString.ptr(), itemString.ptr()) }
    }
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenuBar_GetHelpString(self.ptr(), id) }.to_str() }
    }
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenuBar_GetLabel(self.ptr(), id) }.to_str() }
    }
    fn getLabelTop(&self, pos: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenuBar_GetLabelTop(self.ptr(), pos) }.to_str() }
    }
    fn getMenu(&self, pos: c_int) -> WxMenu {
        unsafe { WxMenu { ptr: wxMenuBar_GetMenu(self.ptr(), pos) } }
    }
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.ptr()) }
    }
    fn insert<T: TWxMenu>(&self, pos: c_int, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Insert(self.ptr(), pos, menu.ptr(), title.ptr()) }
    }
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsChecked(self.ptr(), id) }
    }
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsEnabled(self.ptr(), id) }
    }
    fn remove(&self, pos: c_int) -> WxMenu {
        unsafe { WxMenu { ptr: wxMenuBar_Remove(self.ptr(), pos) } }
    }
    fn replace<T: TWxMenu>(&self, pos: c_int, menu: &T, title: &str) -> WxMenu {
        let title = wxT(title);
        unsafe { WxMenu { ptr: wxMenuBar_Replace(self.ptr(), pos, menu.ptr(), title.ptr()) } }
    }
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenuBar_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    fn setItemLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenuBar_SetItemLabel(self.ptr(), id, label.ptr()) }
    }
    fn setLabel(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxMenuBar_SetLabel(self.ptr(), s.ptr()) }
    }
    fn setLabelTop(&self, pos: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenuBar_SetLabelTop(self.ptr(), pos, label.ptr()) }
    }
    fn getFrame(&self) -> WxFrame {
        unsafe { WxFrame { ptr: wxMenuBar_GetFrame(self.ptr()) } }
    }
}

pub struct WxMenuEvent { ptr: *mut c_void }
impl TWxMenuEvent for WxMenuEvent {}
impl TWxEvent for WxMenuEvent {}
impl TWxObject for WxMenuEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMenuEvent {
    pub fn from(ptr: *mut c_void) -> WxMenuEvent { WxMenuEvent { ptr: ptr } }
    pub fn null() -> WxMenuEvent { WxMenuEvent::from(0 as *mut c_void) }
    
}

pub trait TWxMenuEvent : TWxEvent {
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.ptr()) }
    }
}

pub struct WxMenuItem { ptr: *mut c_void }
impl TWxMenuItem for WxMenuItem {}
impl TWxObject for WxMenuItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMenuItem {
    pub fn from(ptr: *mut c_void) -> WxMenuItem { WxMenuItem { ptr: ptr } }
    pub fn null() -> WxMenuItem { WxMenuItem::from(0 as *mut c_void) }
    
    pub fn new() -> WxMenuItem {
        unsafe { WxMenuItem { ptr: wxMenuItem_Create() } }
    }
    pub fn getLabelFromText(text: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxMenuItem_GetLabelFromText(text) }.to_str() }
    }
    pub fn newSeparator() -> WxMenuItem {
        unsafe { WxMenuItem { ptr: wxMenuItem_CreateSeparator() } }
    }
    pub fn newEx<T: TWxMenu>(id: c_int, label: &str, help: &str, itemkind: c_int, submenu: &T) -> WxMenuItem {
        let label = wxT(label);
        let help = wxT(help);
        unsafe { WxMenuItem { ptr: wxMenuItem_CreateEx(id, label.ptr(), help.ptr(), itemkind, submenu.ptr()) } }
    }
}

pub trait TWxMenuItem : TWxObject {
    fn check(&self, check: c_int) {
        unsafe { wxMenuItem_Check(self.ptr(), check) }
    }
    fn enable(&self, enable: c_int) {
        unsafe { wxMenuItem_Enable(self.ptr(), enable) }
    }
    fn getHelp(&self) -> ~str {
        unsafe { WxString { ptr: wxMenuItem_GetHelp(self.ptr()) }.to_str() }
    }
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxMenuItem_GetLabel(self.ptr()) }.to_str() }
    }
    fn getMenu(&self) -> WxMenu {
        unsafe { WxMenu { ptr: wxMenuItem_GetMenu(self.ptr()) } }
    }
    fn getSubMenu(&self) -> WxMenu {
        unsafe { WxMenu { ptr: wxMenuItem_GetSubMenu(self.ptr()) } }
    }
    fn getText(&self) -> ~str {
        unsafe { WxString { ptr: wxMenuItem_GetText(self.ptr()) }.to_str() }
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
        let str = wxT(str);
        unsafe { wxMenuItem_SetHelp(self.ptr(), str.ptr()) }
    }
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.ptr(), id) }
    }
    fn setSubMenu<T: TWxMenu>(&self, menu: &T) {
        unsafe { wxMenuItem_SetSubMenu(self.ptr(), menu.ptr()) }
    }
    fn setText(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxMenuItem_SetText(self.ptr(), str.ptr()) }
    }
}

pub struct WxMessageDialog { ptr: *mut c_void }
impl TWxMessageDialog for WxMessageDialog {}
impl TWxDialog for WxMessageDialog {}
impl TWxTopLevelWindow for WxMessageDialog {}
impl TWxWindow for WxMessageDialog {}
impl TWxEvtHandler for WxMessageDialog {}
impl TWxObject for WxMessageDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMessageDialog {
    pub fn from(ptr: *mut c_void) -> WxMessageDialog { WxMessageDialog { ptr: ptr } }
    pub fn null() -> WxMessageDialog { WxMessageDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _msg: &str, _cap: &str, _stl: c_int) -> WxMessageDialog {
        let _msg = wxT(_msg);
        let _cap = wxT(_cap);
        unsafe { WxMessageDialog { ptr: wxMessageDialog_Create(_prt.ptr(), _msg.ptr(), _cap.ptr(), _stl) } }
    }
}

pub trait TWxMessageDialog : TWxDialog {
}

pub struct WxMetafile { ptr: *mut c_void }
impl TWxMetafile for WxMetafile {}
impl TWxObject for WxMetafile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMetafile {
    pub fn from(ptr: *mut c_void) -> WxMetafile { WxMetafile { ptr: ptr } }
    pub fn null() -> WxMetafile { WxMetafile::from(0 as *mut c_void) }
    
    pub fn new(_file: &str) -> WxMetafile {
        let _file = wxT(_file);
        unsafe { WxMetafile { ptr: wxMetafile_Create(_file.ptr()) } }
    }
}

pub trait TWxMetafile : TWxObject {
    fn isOk(&self) -> c_int {
        unsafe { wxMetafile_IsOk(self.ptr()) }
    }
    fn play<T: TWxDC>(&self, _dc: &T) -> c_int {
        unsafe { wxMetafile_Play(self.ptr(), _dc.ptr()) }
    }
    fn setClipboard(&self, width: c_int, height: c_int) -> c_int {
        unsafe { wxMetafile_SetClipboard(self.ptr(), width, height) }
    }
}

pub struct WxMetafileDC { ptr: *mut c_void }
impl TWxMetafileDC for WxMetafileDC {}
impl TWxDC for WxMetafileDC {}
impl TWxObject for WxMetafileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMetafileDC {
    pub fn from(ptr: *mut c_void) -> WxMetafileDC { WxMetafileDC { ptr: ptr } }
    pub fn null() -> WxMetafileDC { WxMetafileDC::from(0 as *mut c_void) }
    
    pub fn new(_file: &str) -> WxMetafileDC {
        let _file = wxT(_file);
        unsafe { WxMetafileDC { ptr: wxMetafileDC_Create(_file.ptr()) } }
    }
}

pub trait TWxMetafileDC : TWxDC {
    fn close(&self) -> *mut c_void {
        unsafe { wxMetafileDC_Close(self.ptr()) }
    }
}

pub struct WxMimeTypesManager { ptr: *mut c_void }
impl TWxMimeTypesManager for WxMimeTypesManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMimeTypesManager {
    pub fn from(ptr: *mut c_void) -> WxMimeTypesManager { WxMimeTypesManager { ptr: ptr } }
    pub fn null() -> WxMimeTypesManager { WxMimeTypesManager::from(0 as *mut c_void) }
    
    pub fn new() -> WxMimeTypesManager {
        unsafe { WxMimeTypesManager { ptr: wxMimeTypesManager_Create() } }
    }
}

pub trait TWxMimeTypesManager {
    fn ptr(&self) -> *mut c_void;
    
    fn addFallbacks(&self, _types: *mut c_void) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.ptr(), _types) }
    }
    fn enumAllFileTypes<T: TWxList>(&self, _lst: &T) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.ptr(), _lst.ptr()) }
    }
    fn getFileTypeFromExtension(&self, _ext: &str) -> WxFileType {
        let _ext = wxT(_ext);
        unsafe { WxFileType { ptr: wxMimeTypesManager_GetFileTypeFromExtension(self.ptr(), _ext.ptr()) } }
    }
    fn getFileTypeFromMimeType(&self, _name: &str) -> WxFileType {
        let _name = wxT(_name);
        unsafe { WxFileType { ptr: wxMimeTypesManager_GetFileTypeFromMimeType(self.ptr(), _name.ptr()) } }
    }
    fn isOfType(&self, _type: &str, _wildcard: &str) -> c_int {
        let _type = wxT(_type);
        let _wildcard = wxT(_wildcard);
        unsafe { wxMimeTypesManager_IsOfType(self.ptr(), _type.ptr(), _wildcard.ptr()) }
    }
}

pub struct WxMiniFrame { ptr: *mut c_void }
impl TWxMiniFrame for WxMiniFrame {}
impl TWxFrame for WxMiniFrame {}
impl TWxTopLevelWindow for WxMiniFrame {}
impl TWxWindow for WxMiniFrame {}
impl TWxEvtHandler for WxMiniFrame {}
impl TWxObject for WxMiniFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMiniFrame {
    pub fn from(ptr: *mut c_void) -> WxMiniFrame { WxMiniFrame { ptr: ptr } }
    pub fn null() -> WxMiniFrame { WxMiniFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxMiniFrame {
        let _txt = wxT(_txt);
        unsafe { WxMiniFrame { ptr: wxMiniFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxMiniFrame : TWxFrame {
}

pub struct WxMirrorDC { ptr: *mut c_void }
impl TWxMirrorDC for WxMirrorDC {}
impl TWxDC for WxMirrorDC {}
impl TWxObject for WxMirrorDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMirrorDC {
    pub fn from(ptr: *mut c_void) -> WxMirrorDC { WxMirrorDC { ptr: ptr } }
    pub fn null() -> WxMirrorDC { WxMirrorDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxDC>(dc: &T) -> WxMirrorDC {
        unsafe { WxMirrorDC { ptr: wxMirrorDC_Create(dc.ptr()) } }
    }
}

pub trait TWxMirrorDC : TWxDC {
}

pub struct WxMouseCaptureChangedEvent { ptr: *mut c_void }
impl TWxMouseCaptureChangedEvent for WxMouseCaptureChangedEvent {}
impl TWxEvent for WxMouseCaptureChangedEvent {}
impl TWxObject for WxMouseCaptureChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMouseCaptureChangedEvent {
    pub fn from(ptr: *mut c_void) -> WxMouseCaptureChangedEvent { WxMouseCaptureChangedEvent { ptr: ptr } }
    pub fn null() -> WxMouseCaptureChangedEvent { WxMouseCaptureChangedEvent::from(0 as *mut c_void) }
    
}

pub trait TWxMouseCaptureChangedEvent : TWxEvent {
}

pub struct WxMouseEvent { ptr: *mut c_void }
impl TWxMouseEvent for WxMouseEvent {}
impl TWxEvent for WxMouseEvent {}
impl TWxObject for WxMouseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMouseEvent {
    pub fn from(ptr: *mut c_void) -> WxMouseEvent { WxMouseEvent { ptr: ptr } }
    pub fn null() -> WxMouseEvent { WxMouseEvent::from(0 as *mut c_void) }
    
}

pub trait TWxMouseEvent : TWxEvent {
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
    fn getLogicalPosition<T: TWxDC>(&self, dc: &T) -> WxPoint {
        unsafe { WxPoint { ptr: wxMouseEvent_GetLogicalPosition(self.ptr(), dc.ptr()) } }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxMouseEvent_GetPosition(self.ptr()) } }
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

pub struct WxMoveEvent { ptr: *mut c_void }
impl TWxMoveEvent for WxMoveEvent {}
impl TWxEvent for WxMoveEvent {}
impl TWxObject for WxMoveEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxMoveEvent {
    pub fn from(ptr: *mut c_void) -> WxMoveEvent { WxMoveEvent { ptr: ptr } }
    pub fn null() -> WxMoveEvent { WxMoveEvent::from(0 as *mut c_void) }
    
}

pub trait TWxMoveEvent : TWxEvent {
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxMoveEvent_GetPosition(self.ptr()) } }
    }
}

pub struct WxNavigationKeyEvent { ptr: *mut c_void }
impl TWxNavigationKeyEvent for WxNavigationKeyEvent {}
impl TWxEvent for WxNavigationKeyEvent {}
impl TWxObject for WxNavigationKeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxNavigationKeyEvent {
    pub fn from(ptr: *mut c_void) -> WxNavigationKeyEvent { WxNavigationKeyEvent { ptr: ptr } }
    pub fn null() -> WxNavigationKeyEvent { WxNavigationKeyEvent::from(0 as *mut c_void) }
    
}

pub trait TWxNavigationKeyEvent : TWxEvent {
    fn getCurrentFocus(&self) -> *mut c_void {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.ptr()) }
    }
    fn getDirection(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_GetDirection(self.ptr()) }
    }
    fn isWindowChange(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.ptr()) }
    }
    fn setCurrentFocus<T: TWxWindow>(&self, win: &T) {
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

pub struct WxNotebook { ptr: *mut c_void }
impl TWxNotebook for WxNotebook {}
impl TWxControl for WxNotebook {}
impl TWxWindow for WxNotebook {}
impl TWxEvtHandler for WxNotebook {}
impl TWxObject for WxNotebook { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxNotebook {
    pub fn from(ptr: *mut c_void) -> WxNotebook { WxNotebook { ptr: ptr } }
    pub fn null() -> WxNotebook { WxNotebook::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxNotebook {
        unsafe { WxNotebook { ptr: wxNotebook_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxNotebook : TWxControl {
    fn addPage<T: TWxWindow>(&self, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
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
    fn getImageList(&self) -> WxImageList {
        unsafe { WxImageList { ptr: wxNotebook_GetImageList(self.ptr()) } }
    }
    fn getPage(&self, nPage: c_int) -> WxWindow {
        unsafe { WxWindow { ptr: wxNotebook_GetPage(self.ptr(), nPage) } }
    }
    fn getPageCount(&self) -> c_int {
        unsafe { wxNotebook_GetPageCount(self.ptr()) }
    }
    fn getPageImage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_GetPageImage(self.ptr(), nPage) }
    }
    fn getPageText(&self, nPage: c_int) -> ~str {
        unsafe { WxString { ptr: wxNotebook_GetPageText(self.ptr(), nPage) }.to_str() }
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
    fn insertPage<T: TWxWindow>(&self, nPage: c_int, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_InsertPage(self.ptr(), nPage, pPage.ptr(), strText.ptr(), bSelect, imageId) }
    }
    fn removePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_RemovePage(self.ptr(), nPage) }
    }
    fn setImageList<T: TWxImageList>(&self, imageList: &T) {
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
        let strText = wxT(strText);
        unsafe { wxNotebook_SetPageText(self.ptr(), nPage, strText.ptr()) }
    }
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.ptr(), nPage) }
    }
    fn assignImageList<T: TWxImageList>(&self, imageList: &T) {
        unsafe { wxNotebook_AssignImageList(self.ptr(), imageList.ptr()) }
    }
}

pub struct WxNotebookEvent { ptr: *mut c_void }
impl TWxNotebookEvent for WxNotebookEvent {}
impl TWxNotifyEvent for WxNotebookEvent {}
impl TWxCommandEvent for WxNotebookEvent {}
impl TWxEvent for WxNotebookEvent {}
impl TWxObject for WxNotebookEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxNotebookEvent {
    pub fn from(ptr: *mut c_void) -> WxNotebookEvent { WxNotebookEvent { ptr: ptr } }
    pub fn null() -> WxNotebookEvent { WxNotebookEvent::from(0 as *mut c_void) }
    
}

pub trait TWxNotebookEvent : TWxNotifyEvent {
}

pub struct WxNotifyEvent { ptr: *mut c_void }
impl TWxNotifyEvent for WxNotifyEvent {}
impl TWxCommandEvent for WxNotifyEvent {}
impl TWxEvent for WxNotifyEvent {}
impl TWxObject for WxNotifyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxNotifyEvent {
    pub fn from(ptr: *mut c_void) -> WxNotifyEvent { WxNotifyEvent { ptr: ptr } }
    pub fn null() -> WxNotifyEvent { WxNotifyEvent::from(0 as *mut c_void) }
    
}

pub trait TWxNotifyEvent : TWxCommandEvent {
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

pub struct WxPageSetupDialog { ptr: *mut c_void }
impl TWxPageSetupDialog for WxPageSetupDialog {}
impl TWxDialog for WxPageSetupDialog {}
impl TWxTopLevelWindow for WxPageSetupDialog {}
impl TWxWindow for WxPageSetupDialog {}
impl TWxEvtHandler for WxPageSetupDialog {}
impl TWxObject for WxPageSetupDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPageSetupDialog {
    pub fn from(ptr: *mut c_void) -> WxPageSetupDialog { WxPageSetupDialog { ptr: ptr } }
    pub fn null() -> WxPageSetupDialog { WxPageSetupDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxPageSetupDialogData>(parent: &T, data: &U) -> WxPageSetupDialog {
        unsafe { WxPageSetupDialog { ptr: wxPageSetupDialog_Create(parent.ptr(), data.ptr()) } }
    }
}

pub trait TWxPageSetupDialog : TWxDialog {
    fn getPageSetupData<T: TWxPageSetupDialogData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.ptr(), _ref.ptr()) }
    }
}

pub struct WxPageSetupDialogData { ptr: *mut c_void }
impl TWxPageSetupDialogData for WxPageSetupDialogData {}
impl TWxObject for WxPageSetupDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPageSetupDialogData {
    pub fn from(ptr: *mut c_void) -> WxPageSetupDialogData { WxPageSetupDialogData { ptr: ptr } }
    pub fn null() -> WxPageSetupDialogData { WxPageSetupDialogData::from(0 as *mut c_void) }
    
    pub fn new() -> WxPageSetupDialogData {
        unsafe { WxPageSetupDialogData { ptr: wxPageSetupDialogData_Create() } }
    }
    pub fn newFromData<T: TWxPrintData>(printData: &T) -> WxPageSetupDialogData {
        unsafe { WxPageSetupDialogData { ptr: wxPageSetupDialogData_CreateFromData(printData.ptr()) } }
    }
}

pub trait TWxPageSetupDialogData : TWxObject {
    fn assign<T: TWxPageSetupDialogData>(&self, data: &T) {
        unsafe { wxPageSetupDialogData_Assign(self.ptr(), data.ptr()) }
    }
    fn assignData<T: TWxPrintData>(&self, printData: &T) {
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
    fn getMarginBottomRight(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxPageSetupDialogData_GetMarginBottomRight(self.ptr()) } }
    }
    fn getMarginTopLeft(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxPageSetupDialogData_GetMarginTopLeft(self.ptr()) } }
    }
    fn getMinMarginBottomRight(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxPageSetupDialogData_GetMinMarginBottomRight(self.ptr()) } }
    }
    fn getMinMarginTopLeft(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxPageSetupDialogData_GetMinMarginTopLeft(self.ptr()) } }
    }
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.ptr()) }
    }
    fn getPaperSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxPageSetupDialogData_GetPaperSize(self.ptr()) } }
    }
    fn getPrintData<T: TWxPrintData>(&self, _ref: &T) {
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
    fn setPrintData<T: TWxPrintData>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.ptr(), printData.ptr()) }
    }
}

pub struct WxPaintDC { ptr: *mut c_void }
impl TWxPaintDC for WxPaintDC {}
impl TWxWindowDC for WxPaintDC {}
impl TWxDC for WxPaintDC {}
impl TWxObject for WxPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPaintDC {
    pub fn from(ptr: *mut c_void) -> WxPaintDC { WxPaintDC { ptr: ptr } }
    pub fn null() -> WxPaintDC { WxPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(win: &T) -> WxPaintDC {
        unsafe { WxPaintDC { ptr: wxPaintDC_Create(win.ptr()) } }
    }
}

pub trait TWxPaintDC : TWxWindowDC {
}

pub struct WxPaintEvent { ptr: *mut c_void }
impl TWxPaintEvent for WxPaintEvent {}
impl TWxEvent for WxPaintEvent {}
impl TWxObject for WxPaintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPaintEvent {
    pub fn from(ptr: *mut c_void) -> WxPaintEvent { WxPaintEvent { ptr: ptr } }
    pub fn null() -> WxPaintEvent { WxPaintEvent::from(0 as *mut c_void) }
    
}

pub trait TWxPaintEvent : TWxEvent {
}

pub struct WxPalette { ptr: *mut c_void }
impl TWxPalette for WxPalette {}
impl TWxGDIObject for WxPalette {}
impl TWxObject for WxPalette { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPalette {
    pub fn from(ptr: *mut c_void) -> WxPalette { WxPalette { ptr: ptr } }
    pub fn null() -> WxPalette { WxPalette::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxPalette {
        unsafe { WxPalette { ptr: wxPalette_CreateDefault() } }
    }
    pub fn newRGB(n: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> WxPalette {
        unsafe { WxPalette { ptr: wxPalette_CreateRGB(n, red, green, blue) } }
    }
}

pub trait TWxPalette : TWxGDIObject {
    fn assign<T: TWxPalette>(&self, palette: &T) {
        unsafe { wxPalette_Assign(self.ptr(), palette.ptr()) }
    }
    fn getPixel(&self, red: uint8_t, green: uint8_t, blue: uint8_t) -> c_int {
        unsafe { wxPalette_GetPixel(self.ptr(), red, green, blue) }
    }
    fn getRGB(&self, pixel: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> c_int {
        unsafe { wxPalette_GetRGB(self.ptr(), pixel, red, green, blue) }
    }
    fn isEqual<T: TWxPalette>(&self, palette: &T) -> c_int {
        unsafe { wxPalette_IsEqual(self.ptr(), palette.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPalette_IsOk(self.ptr()) }
    }
}

pub struct WxPaletteChangedEvent { ptr: *mut c_void }
impl TWxPaletteChangedEvent for WxPaletteChangedEvent {}
impl TWxEvent for WxPaletteChangedEvent {}
impl TWxObject for WxPaletteChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPaletteChangedEvent {
    pub fn from(ptr: *mut c_void) -> WxPaletteChangedEvent { WxPaletteChangedEvent { ptr: ptr } }
    pub fn null() -> WxPaletteChangedEvent { WxPaletteChangedEvent::from(0 as *mut c_void) }
    
}

pub trait TWxPaletteChangedEvent : TWxEvent {
    fn getChangedWindow(&self) -> *mut c_void {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.ptr()) }
    }
    fn setChangedWindow<T: TWxWindow>(&self, win: &T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.ptr(), win.ptr()) }
    }
}

pub struct WxPanel { ptr: *mut c_void }
impl TWxPanel for WxPanel {}
impl TWxWindow for WxPanel {}
impl TWxEvtHandler for WxPanel {}
impl TWxObject for WxPanel { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPanel {
    pub fn from(ptr: *mut c_void) -> WxPanel { WxPanel { ptr: ptr } }
    pub fn null() -> WxPanel { WxPanel::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxPanel {
        unsafe { WxPanel { ptr: wxPanel_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxPanel : TWxWindow {
}

pub struct WxPen { ptr: *mut c_void }
impl TWxPen for WxPen {}
impl TWxGDIObject for WxPen {}
impl TWxObject for WxPen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPen {
    pub fn from(ptr: *mut c_void) -> WxPen { WxPen { ptr: ptr } }
    pub fn null() -> WxPen { WxPen::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxPen {
        unsafe { WxPen { ptr: wxPen_CreateDefault() } }
    }
    pub fn newFromBitmap<T: TWxBitmap>(stipple: &T, width: c_int) -> WxPen {
        unsafe { WxPen { ptr: wxPen_CreateFromBitmap(stipple.ptr(), width) } }
    }
    pub fn newFromColour<T: TWxColour>(col: &T, width: c_int, style: c_int) -> WxPen {
        unsafe { WxPen { ptr: wxPen_CreateFromColour(col.ptr(), width, style) } }
    }
    pub fn newFromStock(id: c_int) -> WxPen {
        unsafe { WxPen { ptr: wxPen_CreateFromStock(id) } }
    }
}

pub trait TWxPen : TWxGDIObject {
    fn assign<T: TWxPen>(&self, pen: &T) {
        unsafe { wxPen_Assign(self.ptr(), pen.ptr()) }
    }
    fn getCap(&self) -> c_int {
        unsafe { wxPen_GetCap(self.ptr()) }
    }
    fn getColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxPen_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getDashes(&self, ptr: *mut c_void) -> c_int {
        unsafe { wxPen_GetDashes(self.ptr(), ptr) }
    }
    fn getJoin(&self) -> c_int {
        unsafe { wxPen_GetJoin(self.ptr()) }
    }
    fn getStipple<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxPen_GetStipple(self.ptr(), _ref.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxPen_GetStyle(self.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxPen_GetWidth(self.ptr()) }
    }
    fn isEqual<T: TWxPen>(&self, pen: &T) -> c_int {
        unsafe { wxPen_IsEqual(self.ptr(), pen.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPen_IsOk(self.ptr()) }
    }
    fn setCap(&self, cap: c_int) {
        unsafe { wxPen_SetCap(self.ptr(), cap) }
    }
    fn setColour<T: TWxColour>(&self, col: &T) {
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
    fn setStipple<T: TWxBitmap>(&self, stipple: &T) {
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

pub struct WxPenList { ptr: *mut c_void }
impl TWxPenList for WxPenList {}
impl TWxList for WxPenList {}
impl TWxObject for WxPenList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPenList {
    pub fn from(ptr: *mut c_void) -> WxPenList { WxPenList { ptr: ptr } }
    pub fn null() -> WxPenList { WxPenList::from(0 as *mut c_void) }
    
}

pub trait TWxPenList : TWxList {
}

pub struct WxPoint { ptr: *mut c_void }
impl TWxPoint for WxPoint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPoint {
    pub fn from(ptr: *mut c_void) -> WxPoint { WxPoint { ptr: ptr } }
    pub fn null() -> WxPoint { WxPoint::from(0 as *mut c_void) }
    
    pub fn new(xx: c_int, yy: c_int) -> WxPoint {
        unsafe { WxPoint { ptr: wxPoint_Create(xx, yy) } }
    }
}

pub trait TWxPoint {
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

pub struct WxPopupTransientWindow { ptr: *mut c_void }
impl TWxPopupTransientWindow for WxPopupTransientWindow {}
impl TWxPopupWindow for WxPopupTransientWindow {}
impl TWxWindow for WxPopupTransientWindow {}
impl TWxEvtHandler for WxPopupTransientWindow {}
impl TWxObject for WxPopupTransientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPopupTransientWindow {
    pub fn from(ptr: *mut c_void) -> WxPopupTransientWindow { WxPopupTransientWindow { ptr: ptr } }
    pub fn null() -> WxPopupTransientWindow { WxPopupTransientWindow::from(0 as *mut c_void) }
    
}

pub trait TWxPopupTransientWindow : TWxPopupWindow {
}

pub struct WxPopupWindow { ptr: *mut c_void }
impl TWxPopupWindow for WxPopupWindow {}
impl TWxWindow for WxPopupWindow {}
impl TWxEvtHandler for WxPopupWindow {}
impl TWxObject for WxPopupWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPopupWindow {
    pub fn from(ptr: *mut c_void) -> WxPopupWindow { WxPopupWindow { ptr: ptr } }
    pub fn null() -> WxPopupWindow { WxPopupWindow::from(0 as *mut c_void) }
    
}

pub trait TWxPopupWindow : TWxWindow {
}

pub struct WxPostScriptDC { ptr: *mut c_void }
impl TWxPostScriptDC for WxPostScriptDC {}
impl TWxDC for WxPostScriptDC {}
impl TWxObject for WxPostScriptDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPostScriptDC {
    pub fn from(ptr: *mut c_void) -> WxPostScriptDC { WxPostScriptDC { ptr: ptr } }
    pub fn null() -> WxPostScriptDC { WxPostScriptDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxPrintData>(data: &T) -> WxPostScriptDC {
        unsafe { WxPostScriptDC { ptr: wxPostScriptDC_Create(data.ptr()) } }
    }
}

pub trait TWxPostScriptDC : TWxDC {
    fn setResolution(&self, ppi: c_int) {
        unsafe { wxPostScriptDC_SetResolution(self.ptr(), ppi) }
    }
    fn getResolution(&self) -> c_int {
        unsafe { wxPostScriptDC_GetResolution(self.ptr()) }
    }
}

pub struct WxPreviewCanvas { ptr: *mut c_void }
impl TWxPreviewCanvas for WxPreviewCanvas {}
impl TWxScrolledWindow for WxPreviewCanvas {}
impl TWxPanel for WxPreviewCanvas {}
impl TWxWindow for WxPreviewCanvas {}
impl TWxEvtHandler for WxPreviewCanvas {}
impl TWxObject for WxPreviewCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPreviewCanvas {
    pub fn from(ptr: *mut c_void) -> WxPreviewCanvas { WxPreviewCanvas { ptr: ptr } }
    pub fn null() -> WxPreviewCanvas { WxPreviewCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: TWxPrintPreview, U: TWxWindow>(preview: &T, parent: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> WxPreviewCanvas {
        unsafe { WxPreviewCanvas { ptr: wxPreviewCanvas_Create(preview.ptr(), parent.ptr(), x, y, w, h, style) } }
    }
}

pub trait TWxPreviewCanvas : TWxScrolledWindow {
}

pub struct WxPreviewControlBar { ptr: *mut c_void }
impl TWxPreviewControlBar for WxPreviewControlBar {}
impl TWxPanel for WxPreviewControlBar {}
impl TWxWindow for WxPreviewControlBar {}
impl TWxEvtHandler for WxPreviewControlBar {}
impl TWxObject for WxPreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPreviewControlBar {
    pub fn from(ptr: *mut c_void) -> WxPreviewControlBar { WxPreviewControlBar { ptr: ptr } }
    pub fn null() -> WxPreviewControlBar { WxPreviewControlBar::from(0 as *mut c_void) }
    
}

pub trait TWxPreviewControlBar : TWxPanel {
}

pub struct WxPreviewFrame { ptr: *mut c_void }
impl TWxPreviewFrame for WxPreviewFrame {}
impl TWxFrame for WxPreviewFrame {}
impl TWxTopLevelWindow for WxPreviewFrame {}
impl TWxWindow for WxPreviewFrame {}
impl TWxEvtHandler for WxPreviewFrame {}
impl TWxObject for WxPreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPreviewFrame {
    pub fn from(ptr: *mut c_void) -> WxPreviewFrame { WxPreviewFrame { ptr: ptr } }
    pub fn null() -> WxPreviewFrame { WxPreviewFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWxPrintPreview, U: TWxFrame>(preview: &T, parent: &U, title: &str, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: &str) -> WxPreviewFrame {
        let title = wxT(title);
        let name = wxT(name);
        unsafe { WxPreviewFrame { ptr: wxPreviewFrame_Create(preview.ptr(), parent.ptr(), title.ptr(), x, y, width, height, style, name.ptr()) } }
    }
}

pub trait TWxPreviewFrame : TWxFrame {
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.ptr()) }
    }
}

pub struct WxPrintData { ptr: *mut c_void }
impl TWxPrintData for WxPrintData {}
impl TWxObject for WxPrintData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrintData {
    pub fn from(ptr: *mut c_void) -> WxPrintData { WxPrintData { ptr: ptr } }
    pub fn null() -> WxPrintData { WxPrintData::from(0 as *mut c_void) }
    
    pub fn new() -> WxPrintData {
        unsafe { WxPrintData { ptr: wxPrintData_Create() } }
    }
}

pub trait TWxPrintData : TWxObject {
    fn assign<T: TWxPrintData>(&self, data: &T) {
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
        unsafe { WxString { ptr: wxPrintData_GetFilename(self.ptr()) }.to_str() }
    }
    fn getFontMetricPath(&self) -> ~str {
        unsafe { WxString { ptr: wxPrintData_GetFontMetricPath(self.ptr()) }.to_str() }
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
    fn getPaperSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxPrintData_GetPaperSize(self.ptr()) } }
    }
    fn getPreviewCommand(&self) -> ~str {
        unsafe { WxString { ptr: wxPrintData_GetPreviewCommand(self.ptr()) }.to_str() }
    }
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.ptr()) }
    }
    fn getPrinterCommand(&self) -> ~str {
        unsafe { WxString { ptr: wxPrintData_GetPrinterCommand(self.ptr()) }.to_str() }
    }
    fn getPrinterName(&self) -> ~str {
        unsafe { WxString { ptr: wxPrintData_GetPrinterName(self.ptr()) }.to_str() }
    }
    fn getPrinterOptions(&self) -> ~str {
        unsafe { WxString { ptr: wxPrintData_GetPrinterOptions(self.ptr()) }.to_str() }
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
        let filename = wxT(filename);
        unsafe { wxPrintData_SetFilename(self.ptr(), filename.ptr()) }
    }
    fn setFontMetricPath(&self, path: &str) {
        let path = wxT(path);
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
    fn setPreviewCommand<T: TWxCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPreviewCommand(self.ptr(), command.ptr()) }
    }
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.ptr(), printMode) }
    }
    fn setPrinterCommand<T: TWxCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPrinterCommand(self.ptr(), command.ptr()) }
    }
    fn setPrinterName(&self, name: &str) {
        let name = wxT(name);
        unsafe { wxPrintData_SetPrinterName(self.ptr(), name.ptr()) }
    }
    fn setPrinterOptions(&self, options: &str) {
        let options = wxT(options);
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

pub struct WxPostScriptPrintNativeData { ptr: *mut c_void }
impl TWxPostScriptPrintNativeData for WxPostScriptPrintNativeData {}
impl TWxObject for WxPostScriptPrintNativeData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPostScriptPrintNativeData {
    pub fn from(ptr: *mut c_void) -> WxPostScriptPrintNativeData { WxPostScriptPrintNativeData { ptr: ptr } }
    pub fn null() -> WxPostScriptPrintNativeData { WxPostScriptPrintNativeData::from(0 as *mut c_void) }
    
    pub fn new() -> WxPostScriptPrintNativeData {
        unsafe { WxPostScriptPrintNativeData { ptr: wxPostScriptPrintNativeData_Create() } }
    }
}

pub trait TWxPostScriptPrintNativeData : TWxObject {
}

pub struct WxPrintDialog { ptr: *mut c_void }
impl TWxPrintDialog for WxPrintDialog {}
impl TWxDialog for WxPrintDialog {}
impl TWxTopLevelWindow for WxPrintDialog {}
impl TWxWindow for WxPrintDialog {}
impl TWxEvtHandler for WxPrintDialog {}
impl TWxObject for WxPrintDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrintDialog {
    pub fn from(ptr: *mut c_void) -> WxPrintDialog { WxPrintDialog { ptr: ptr } }
    pub fn null() -> WxPrintDialog { WxPrintDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxPrintDialogData>(parent: &T, data: &U) -> WxPrintDialog {
        unsafe { WxPrintDialog { ptr: wxPrintDialog_Create(parent.ptr(), data.ptr()) } }
    }
}

pub trait TWxPrintDialog : TWxDialog {
    fn getPrintDC(&self) -> WxDC {
        unsafe { WxDC { ptr: wxPrintDialog_GetPrintDC(self.ptr()) } }
    }
    fn getPrintData<T: TWxPrintData>(&self, _ref: &T) {
        unsafe { wxPrintDialog_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintDialogData(&self) -> WxPrintDialogData {
        unsafe { WxPrintDialogData { ptr: wxPrintDialog_GetPrintDialogData(self.ptr()) } }
    }
}

pub struct WxPrintDialogData { ptr: *mut c_void }
impl TWxPrintDialogData for WxPrintDialogData {}
impl TWxObject for WxPrintDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrintDialogData {
    pub fn from(ptr: *mut c_void) -> WxPrintDialogData { WxPrintDialogData { ptr: ptr } }
    pub fn null() -> WxPrintDialogData { WxPrintDialogData::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxPrintDialogData {
        unsafe { WxPrintDialogData { ptr: wxPrintDialogData_CreateDefault() } }
    }
    pub fn newFromData<T: TWxPrintData>(printData: &T) -> WxPrintDialogData {
        unsafe { WxPrintDialogData { ptr: wxPrintDialogData_CreateFromData(printData.ptr()) } }
    }
}

pub trait TWxPrintDialogData : TWxObject {
    fn assign<T: TWxPrintDialogData>(&self, data: &T) {
        unsafe { wxPrintDialogData_Assign(self.ptr(), data.ptr()) }
    }
    fn assignData<T: TWxPrintData>(&self, data: &T) {
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
    fn getPrintData<T: TWxPrintData>(&self, _ref: &T) {
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
    fn setPrintData<T: TWxPrintData>(&self, printData: &T) {
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

pub struct WxPrintPreview { ptr: *mut c_void }
impl TWxPrintPreview for WxPrintPreview {}
impl TWxObject for WxPrintPreview { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrintPreview {
    pub fn from(ptr: *mut c_void) -> WxPrintPreview { WxPrintPreview { ptr: ptr } }
    pub fn null() -> WxPrintPreview { WxPrintPreview::from(0 as *mut c_void) }
    
    pub fn newFromData<T: TWxPrintout, U: TWxPrintout, V: TWxPrintData>(printout: &T, printoutForPrinting: &U, data: &V) -> WxPrintPreview {
        unsafe { WxPrintPreview { ptr: wxPrintPreview_CreateFromData(printout.ptr(), printoutForPrinting.ptr(), data.ptr()) } }
    }
    pub fn newFromDialogData<T: TWxPrintout, U: TWxPrintout, V: TWxPrintDialogData>(printout: &T, printoutForPrinting: &U, data: &V) -> WxPrintPreview {
        unsafe { WxPrintPreview { ptr: wxPrintPreview_CreateFromDialogData(printout.ptr(), printoutForPrinting.ptr(), data.ptr()) } }
    }
}

pub trait TWxPrintPreview : TWxObject {
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.ptr()) }
    }
    fn drawBlankPage<T: TWxPreviewCanvas, U: TWxDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_DrawBlankPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    fn getCanvas(&self) -> WxPreviewCanvas {
        unsafe { WxPreviewCanvas { ptr: wxPrintPreview_GetCanvas(self.ptr()) } }
    }
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.ptr()) }
    }
    fn getFrame(&self) -> WxFrame {
        unsafe { WxFrame { ptr: wxPrintPreview_GetFrame(self.ptr()) } }
    }
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMaxPage(self.ptr()) }
    }
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMinPage(self.ptr()) }
    }
    fn getPrintDialogData<T: TWxPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintout(&self) -> WxPrintout {
        unsafe { WxPrintout { ptr: wxPrintPreview_GetPrintout(self.ptr()) } }
    }
    fn getPrintoutForPrinting(&self) -> WxPrintout {
        unsafe { WxPrintout { ptr: wxPrintPreview_GetPrintoutForPrinting(self.ptr()) } }
    }
    fn getZoom(&self) -> c_int {
        unsafe { wxPrintPreview_GetZoom(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPrintPreview_IsOk(self.ptr()) }
    }
    fn paintPage<T: TWxPrintPreview, U: TWxDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_PaintPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    fn print(&self, interactive: c_int) -> c_int {
        unsafe { wxPrintPreview_Print(self.ptr(), interactive) }
    }
    fn renderPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_RenderPage(self.ptr(), pageNum) }
    }
    fn setCanvas<T: TWxPreviewCanvas>(&self, canvas: &T) {
        unsafe { wxPrintPreview_SetCanvas(self.ptr(), canvas.ptr()) }
    }
    fn setCurrentPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_SetCurrentPage(self.ptr(), pageNum) }
    }
    fn setFrame<T: TWxFrame>(&self, frame: &T) {
        unsafe { wxPrintPreview_SetFrame(self.ptr(), frame.ptr()) }
    }
    fn setOk(&self, ok: c_int) {
        unsafe { wxPrintPreview_SetOk(self.ptr(), ok) }
    }
    fn setPrintout<T: TWxPrintout>(&self, printout: &T) {
        unsafe { wxPrintPreview_SetPrintout(self.ptr(), printout.ptr()) }
    }
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.ptr(), percent) }
    }
}

pub struct WxPrinter { ptr: *mut c_void }
impl TWxPrinter for WxPrinter {}
impl TWxObject for WxPrinter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrinter {
    pub fn from(ptr: *mut c_void) -> WxPrinter { WxPrinter { ptr: ptr } }
    pub fn null() -> WxPrinter { WxPrinter::from(0 as *mut c_void) }
    
    pub fn new<T: TWxPrintDialogData>(data: &T) -> WxPrinter {
        unsafe { WxPrinter { ptr: wxPrinter_Create(data.ptr()) } }
    }
}

pub trait TWxPrinter : TWxObject {
    fn newAbortWindow<T: TWxWindow, U: TWxPrintout>(&self, parent: &T, printout: &U) -> WxWindow {
        unsafe { WxWindow { ptr: wxPrinter_CreateAbortWindow(self.ptr(), parent.ptr(), printout.ptr()) } }
    }
    fn getAbort(&self) -> c_int {
        unsafe { wxPrinter_GetAbort(self.ptr()) }
    }
    fn getLastError(&self) -> c_int {
        unsafe { wxPrinter_GetLastError(self.ptr()) }
    }
    fn getPrintDialogData<T: TWxPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrinter_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    fn print<T: TWxWindow, U: TWxPrintout>(&self, parent: &T, printout: &U, prompt: c_int) -> c_int {
        unsafe { wxPrinter_Print(self.ptr(), parent.ptr(), printout.ptr(), prompt) }
    }
    fn printDialog<T: TWxWindow>(&self, parent: &T) -> WxDC {
        unsafe { WxDC { ptr: wxPrinter_PrintDialog(self.ptr(), parent.ptr()) } }
    }
    fn reportError<T: TWxWindow, U: TWxPrintout>(&self, parent: &T, printout: &U, message: &str) {
        let message = wxT(message);
        unsafe { wxPrinter_ReportError(self.ptr(), parent.ptr(), printout.ptr(), message.ptr()) }
    }
    fn setup<T: TWxWindow>(&self, parent: &T) -> c_int {
        unsafe { wxPrinter_Setup(self.ptr(), parent.ptr()) }
    }
}

pub struct WxPrinterDC { ptr: *mut c_void }
impl TWxPrinterDC for WxPrinterDC {}
impl TWxDC for WxPrinterDC {}
impl TWxObject for WxPrinterDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrinterDC {
    pub fn from(ptr: *mut c_void) -> WxPrinterDC { WxPrinterDC { ptr: ptr } }
    pub fn null() -> WxPrinterDC { WxPrinterDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxPrintData>(data: &T) -> WxPrinterDC {
        unsafe { WxPrinterDC { ptr: wxPrinterDC_Create(data.ptr()) } }
    }
}

pub trait TWxPrinterDC : TWxDC {
    fn getPaperRect(&self) -> WxRect {
        unsafe { WxRect { ptr: wxPrinterDC_GetPaperRect(self.ptr()) } }
    }
}

pub struct WxPrintout { ptr: *mut c_void }
impl TWxPrintout for WxPrintout {}
impl TWxObject for WxPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrintout {
    pub fn from(ptr: *mut c_void) -> WxPrintout { WxPrintout { ptr: ptr } }
    pub fn null() -> WxPrintout { WxPrintout::from(0 as *mut c_void) }
    
}

pub trait TWxPrintout : TWxObject {
    fn getDC(&self) -> WxDC {
        unsafe { WxDC { ptr: wxPrintout_GetDC(self.ptr()) } }
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
        unsafe { WxString { ptr: wxPrintout_GetTitle(self.ptr()) }.to_str() }
    }
    fn isPreview(&self) -> c_int {
        unsafe { wxPrintout_IsPreview(self.ptr()) }
    }
    fn setDC<T: TWxDC>(&self, dc: &T) {
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

pub struct WxPrivateDropTarget { ptr: *mut c_void }
impl TWxPrivateDropTarget for WxPrivateDropTarget {}
impl TWxDropTarget for WxPrivateDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxPrivateDropTarget {
    pub fn from(ptr: *mut c_void) -> WxPrivateDropTarget { WxPrivateDropTarget { ptr: ptr } }
    pub fn null() -> WxPrivateDropTarget { WxPrivateDropTarget::from(0 as *mut c_void) }
    
}

pub trait TWxPrivateDropTarget : TWxDropTarget {
}

pub struct WxProcess { ptr: *mut c_void }
impl TWxProcess for WxProcess {}
impl TWxEvtHandler for WxProcess {}
impl TWxObject for WxProcess { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxProcess {
    pub fn from(ptr: *mut c_void) -> WxProcess { WxProcess { ptr: ptr } }
    pub fn null() -> WxProcess { WxProcess::from(0 as *mut c_void) }
    
    pub fn newDefault<T: TWxWindow>(_prt: &T, _id: c_int) -> WxProcess {
        unsafe { WxProcess { ptr: wxProcess_CreateDefault(_prt.ptr(), _id) } }
    }
    pub fn newRedirect<T: TWxWindow>(_prt: &T, _rdr: c_int) -> WxProcess {
        unsafe { WxProcess { ptr: wxProcess_CreateRedirect(_prt.ptr(), _rdr) } }
    }
    pub fn open(cmd: &str, flags: c_int) -> WxProcess {
        let cmd = wxT(cmd);
        unsafe { WxProcess { ptr: wxProcess_Open(cmd.ptr(), flags) } }
    }
}

pub trait TWxProcess : TWxEvtHandler {
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.ptr()) }
    }
    fn detach(&self) {
        unsafe { wxProcess_Detach(self.ptr()) }
    }
    fn getErrorStream(&self) -> WxInputStream {
        unsafe { WxInputStream { ptr: wxProcess_GetErrorStream(self.ptr()) } }
    }
    fn getInputStream(&self) -> WxInputStream {
        unsafe { WxInputStream { ptr: wxProcess_GetInputStream(self.ptr()) } }
    }
    fn getOutputStream(&self) -> WxOutputStream {
        unsafe { WxOutputStream { ptr: wxProcess_GetOutputStream(self.ptr()) } }
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

pub struct WxProcessEvent { ptr: *mut c_void }
impl TWxProcessEvent for WxProcessEvent {}
impl TWxEvent for WxProcessEvent {}
impl TWxObject for WxProcessEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxProcessEvent {
    pub fn from(ptr: *mut c_void) -> WxProcessEvent { WxProcessEvent { ptr: ptr } }
    pub fn null() -> WxProcessEvent { WxProcessEvent::from(0 as *mut c_void) }
    
}

pub trait TWxProcessEvent : TWxEvent {
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.ptr()) }
    }
    fn getPid(&self) -> c_int {
        unsafe { wxProcessEvent_GetPid(self.ptr()) }
    }
}

pub struct WxProgressDialog { ptr: *mut c_void }
impl TWxProgressDialog for WxProgressDialog {}
impl TWxFrame for WxProgressDialog {}
impl TWxTopLevelWindow for WxProgressDialog {}
impl TWxWindow for WxProgressDialog {}
impl TWxEvtHandler for WxProgressDialog {}
impl TWxObject for WxProgressDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxProgressDialog {
    pub fn from(ptr: *mut c_void) -> WxProgressDialog { WxProgressDialog { ptr: ptr } }
    pub fn null() -> WxProgressDialog { WxProgressDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(title: &str, message: &str, max: c_int, parent: &T, style: c_int) -> WxProgressDialog {
        let title = wxT(title);
        let message = wxT(message);
        unsafe { WxProgressDialog { ptr: wxProgressDialog_Create(title.ptr(), message.ptr(), max, parent.ptr(), style) } }
    }
}

pub trait TWxProgressDialog : TWxFrame {
    fn update(&self, value: c_int) -> c_int {
        unsafe { wxProgressDialog_Update(self.ptr(), value) }
    }
    fn updateWithMessage(&self, value: c_int, message: &str) -> c_int {
        let message = wxT(message);
        unsafe { wxProgressDialog_UpdateWithMessage(self.ptr(), value, message.ptr()) }
    }
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.ptr()) }
    }
}

pub struct WxQuantize { ptr: *mut c_void }
impl TWxQuantize for WxQuantize {}
impl TWxObject for WxQuantize { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxQuantize {
    pub fn from(ptr: *mut c_void) -> WxQuantize { WxQuantize { ptr: ptr } }
    pub fn null() -> WxQuantize { WxQuantize::from(0 as *mut c_void) }
    
}

pub trait TWxQuantize : TWxObject {
}

pub struct WxQueryNewPaletteEvent { ptr: *mut c_void }
impl TWxQueryNewPaletteEvent for WxQueryNewPaletteEvent {}
impl TWxEvent for WxQueryNewPaletteEvent {}
impl TWxObject for WxQueryNewPaletteEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxQueryNewPaletteEvent {
    pub fn from(ptr: *mut c_void) -> WxQueryNewPaletteEvent { WxQueryNewPaletteEvent { ptr: ptr } }
    pub fn null() -> WxQueryNewPaletteEvent { WxQueryNewPaletteEvent::from(0 as *mut c_void) }
    
}

pub trait TWxQueryNewPaletteEvent : TWxEvent {
    fn getPaletteRealized(&self) -> c_int {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.ptr()) }
    }
    fn setPaletteRealized(&self, realized: c_int) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self.ptr(), realized) }
    }
}

pub struct WxRadioBox { ptr: *mut c_void }
impl TWxRadioBox for WxRadioBox {}
impl TWxControl for WxRadioBox {}
impl TWxWindow for WxRadioBox {}
impl TWxEvtHandler for WxRadioBox {}
impl TWxObject for WxRadioBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRadioBox {
    pub fn from(ptr: *mut c_void) -> WxRadioBox { WxRadioBox { ptr: ptr } }
    pub fn null() -> WxRadioBox { WxRadioBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *mut *mut c_char, _dim: c_int, _stl: c_int) -> WxRadioBox {
        let _txt = wxT(_txt);
        unsafe { WxRadioBox { ptr: wxRadioBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl) } }
    }
}

pub trait TWxRadioBox : TWxControl {
    fn enableItem(&self, item: c_int, enable: c_int) {
        unsafe { wxRadioBox_EnableItem(self.ptr(), item, enable) }
    }
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxRadioBox_FindString(self.ptr(), s.ptr()) }
    }
    fn getItemLabel(&self, item: c_int) -> ~str {
        unsafe { WxString { ptr: wxRadioBox_GetItemLabel(self.ptr(), item) }.to_str() }
    }
    fn getNumberOfRowsOrCols(&self) -> c_int {
        unsafe { wxRadioBox_GetNumberOfRowsOrCols(self.ptr()) }
    }
    fn getSelection(&self) -> c_int {
        unsafe { wxRadioBox_GetSelection(self.ptr()) }
    }
    fn getStringSelection(&self) -> ~str {
        unsafe { WxString { ptr: wxRadioBox_GetStringSelection(self.ptr()) }.to_str() }
    }
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.ptr()) }
    }
    fn setItemBitmap<T: TWxBitmap>(&self, item: c_int, bitmap: &T) {
        unsafe { wxRadioBox_SetItemBitmap(self.ptr(), item, bitmap.ptr()) }
    }
    fn setItemLabel(&self, item: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxRadioBox_SetItemLabel(self.ptr(), item, label.ptr()) }
    }
    fn setNumberOfRowsOrCols(&self, n: c_int) {
        unsafe { wxRadioBox_SetNumberOfRowsOrCols(self.ptr(), n) }
    }
    fn setSelection(&self, _n: c_int) {
        unsafe { wxRadioBox_SetSelection(self.ptr(), _n) }
    }
    fn setStringSelection(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxRadioBox_SetStringSelection(self.ptr(), s.ptr()) }
    }
    fn showItem(&self, item: c_int, show: c_int) {
        unsafe { wxRadioBox_ShowItem(self.ptr(), item, show) }
    }
}

pub struct WxRadioButton { ptr: *mut c_void }
impl TWxRadioButton for WxRadioButton {}
impl TWxControl for WxRadioButton {}
impl TWxWindow for WxRadioButton {}
impl TWxEvtHandler for WxRadioButton {}
impl TWxObject for WxRadioButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRadioButton {
    pub fn from(ptr: *mut c_void) -> WxRadioButton { WxRadioButton { ptr: ptr } }
    pub fn null() -> WxRadioButton { WxRadioButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxRadioButton {
        let _txt = wxT(_txt);
        unsafe { WxRadioButton { ptr: wxRadioButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxRadioButton : TWxControl {
    fn getValue(&self) -> c_int {
        unsafe { wxRadioButton_GetValue(self.ptr()) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxRadioButton_SetValue(self.ptr(), value) }
    }
}

pub struct WxRealPoint { ptr: *mut c_void }
impl TWxRealPoint for WxRealPoint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRealPoint {
    pub fn from(ptr: *mut c_void) -> WxRealPoint { WxRealPoint { ptr: ptr } }
    pub fn null() -> WxRealPoint { WxRealPoint::from(0 as *mut c_void) }
    
}

pub trait TWxRealPoint {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxRect { ptr: *mut c_void }
impl TWxRect for WxRect { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRect {
    pub fn from(ptr: *mut c_void) -> WxRect { WxRect { ptr: ptr } }
    pub fn null() -> WxRect { WxRect::from(0 as *mut c_void) }
    
}

pub trait TWxRect {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxRegion { ptr: *mut c_void }
impl TWxRegion for WxRegion {}
impl TWxGDIObject for WxRegion {}
impl TWxObject for WxRegion { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRegion {
    pub fn from(ptr: *mut c_void) -> WxRegion { WxRegion { ptr: ptr } }
    pub fn null() -> WxRegion { WxRegion::from(0 as *mut c_void) }
    
    pub fn newDefault() -> WxRegion {
        unsafe { WxRegion { ptr: wxRegion_CreateDefault() } }
    }
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> WxRegion {
        unsafe { WxRegion { ptr: wxRegion_CreateFromRect(x, y, w, h) } }
    }
}

pub trait TWxRegion : TWxGDIObject {
    fn assign<T: TWxRegion>(&self, region: &T) {
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
    fn intersectRegion<T: TWxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_IntersectRegion(self.ptr(), region.ptr()) }
    }
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_SubtractRect(self.ptr(), x, y, width, height) }
    }
    fn subtractRegion<T: TWxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_SubtractRegion(self.ptr(), region.ptr()) }
    }
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_UnionRect(self.ptr(), x, y, width, height) }
    }
    fn unionRegion<T: TWxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_UnionRegion(self.ptr(), region.ptr()) }
    }
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_XorRect(self.ptr(), x, y, width, height) }
    }
    fn xorRegion<T: TWxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_XorRegion(self.ptr(), region.ptr()) }
    }
}

pub struct WxRegionIterator { ptr: *mut c_void }
impl TWxRegionIterator for WxRegionIterator {}
impl TWxObject for WxRegionIterator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxRegionIterator {
    pub fn from(ptr: *mut c_void) -> WxRegionIterator { WxRegionIterator { ptr: ptr } }
    pub fn null() -> WxRegionIterator { WxRegionIterator::from(0 as *mut c_void) }
    
    pub fn new() -> WxRegionIterator {
        unsafe { WxRegionIterator { ptr: wxRegionIterator_Create() } }
    }
    pub fn newFromRegion<T: TWxRegion>(region: &T) -> WxRegionIterator {
        unsafe { WxRegionIterator { ptr: wxRegionIterator_CreateFromRegion(region.ptr()) } }
    }
}

pub trait TWxRegionIterator : TWxObject {
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
    fn resetToRegion<T: TWxRegion>(&self, region: &T) {
        unsafe { wxRegionIterator_ResetToRegion(self.ptr(), region.ptr()) }
    }
}

pub struct WxSVGFileDC { ptr: *mut c_void }
impl TWxSVGFileDC for WxSVGFileDC {}
impl TWxDC for WxSVGFileDC {}
impl TWxObject for WxSVGFileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSVGFileDC {
    pub fn from(ptr: *mut c_void) -> WxSVGFileDC { WxSVGFileDC { ptr: ptr } }
    pub fn null() -> WxSVGFileDC { WxSVGFileDC::from(0 as *mut c_void) }
    
    pub fn new(fileName: &str) -> WxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { WxSVGFileDC { ptr: wxSVGFileDC_Create(fileName.ptr()) } }
    }
    pub fn newWithSize(fileName: &str, w: c_int, h: c_int) -> WxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { WxSVGFileDC { ptr: wxSVGFileDC_CreateWithSize(fileName.ptr(), w, h) } }
    }
    pub fn newWithSizeAndResolution(fileName: &str, w: c_int, h: c_int, a_dpi: c_float) -> WxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { WxSVGFileDC { ptr: wxSVGFileDC_CreateWithSizeAndResolution(fileName.ptr(), w, h, a_dpi) } }
    }
}

pub trait TWxSVGFileDC : TWxDC {
}

pub struct WxScreenDC { ptr: *mut c_void }
impl TWxScreenDC for WxScreenDC {}
impl TWxDC for WxScreenDC {}
impl TWxObject for WxScreenDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScreenDC {
    pub fn from(ptr: *mut c_void) -> WxScreenDC { WxScreenDC { ptr: ptr } }
    pub fn null() -> WxScreenDC { WxScreenDC::from(0 as *mut c_void) }
    
    pub fn new() -> WxScreenDC {
        unsafe { WxScreenDC { ptr: wxScreenDC_Create() } }
    }
}

pub trait TWxScreenDC : TWxDC {
    fn endDrawingOnTop(&self) -> c_int {
        unsafe { wxScreenDC_EndDrawingOnTop(self.ptr()) }
    }
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTop(self.ptr(), x, y, w, h) }
    }
    fn startDrawingOnTopOfWin<T: TWxWindow>(&self, win: &T) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.ptr(), win.ptr()) }
    }
}

pub struct WxScrollBar { ptr: *mut c_void }
impl TWxScrollBar for WxScrollBar {}
impl TWxControl for WxScrollBar {}
impl TWxWindow for WxScrollBar {}
impl TWxEvtHandler for WxScrollBar {}
impl TWxObject for WxScrollBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScrollBar {
    pub fn from(ptr: *mut c_void) -> WxScrollBar { WxScrollBar { ptr: ptr } }
    pub fn null() -> WxScrollBar { WxScrollBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxScrollBar {
        unsafe { WxScrollBar { ptr: wxScrollBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxScrollBar : TWxControl {
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

pub struct WxScrollEvent { ptr: *mut c_void }
impl TWxScrollEvent for WxScrollEvent {}
impl TWxEvent for WxScrollEvent {}
impl TWxObject for WxScrollEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScrollEvent {
    pub fn from(ptr: *mut c_void) -> WxScrollEvent { WxScrollEvent { ptr: ptr } }
    pub fn null() -> WxScrollEvent { WxScrollEvent::from(0 as *mut c_void) }
    
}

pub trait TWxScrollEvent : TWxEvent {
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.ptr()) }
    }
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.ptr()) }
    }
}

pub struct WxScrollWinEvent { ptr: *mut c_void }
impl TWxScrollWinEvent for WxScrollWinEvent {}
impl TWxEvent for WxScrollWinEvent {}
impl TWxObject for WxScrollWinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScrollWinEvent {
    pub fn from(ptr: *mut c_void) -> WxScrollWinEvent { WxScrollWinEvent { ptr: ptr } }
    pub fn null() -> WxScrollWinEvent { WxScrollWinEvent::from(0 as *mut c_void) }
    
}

pub trait TWxScrollWinEvent : TWxEvent {
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

pub struct WxScrolledWindow { ptr: *mut c_void }
impl TWxScrolledWindow for WxScrolledWindow {}
impl TWxPanel for WxScrolledWindow {}
impl TWxWindow for WxScrolledWindow {}
impl TWxEvtHandler for WxScrolledWindow {}
impl TWxObject for WxScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxScrolledWindow {
    pub fn from(ptr: *mut c_void) -> WxScrolledWindow { WxScrolledWindow { ptr: ptr } }
    pub fn null() -> WxScrolledWindow { WxScrolledWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxScrolledWindow {
        unsafe { WxScrolledWindow { ptr: wxScrolledWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxScrolledWindow : TWxPanel {
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
    fn getTargetWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxScrolledWindow_GetTargetWindow(self.ptr()) } }
    }
    fn getViewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetViewStart(self.ptr(), _x, _y) }
    }
    fn onDraw<T: TWxDC>(&self, dc: &T) {
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
    fn setTargetWindow<T: TWxWindow>(&self, target: &T) {
        unsafe { wxScrolledWindow_SetTargetWindow(self.ptr(), target.ptr()) }
    }
    fn viewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_ViewStart(self.ptr(), _x, _y) }
    }
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.ptr(), xstep, ystep) }
    }
}

pub struct WxSetCursorEvent { ptr: *mut c_void }
impl TWxSetCursorEvent for WxSetCursorEvent {}
impl TWxEvent for WxSetCursorEvent {}
impl TWxObject for WxSetCursorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSetCursorEvent {
    pub fn from(ptr: *mut c_void) -> WxSetCursorEvent { WxSetCursorEvent { ptr: ptr } }
    pub fn null() -> WxSetCursorEvent { WxSetCursorEvent::from(0 as *mut c_void) }
    
}

pub trait TWxSetCursorEvent : TWxEvent {
    fn getCursor(&self) -> WxCursor {
        unsafe { WxCursor { ptr: wxSetCursorEvent_GetCursor(self.ptr()) } }
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
    fn setCursor<T: TWxCursor>(&self, cursor: &T) {
        unsafe { wxSetCursorEvent_SetCursor(self.ptr(), cursor.ptr()) }
    }
}

pub struct WxShowEvent { ptr: *mut c_void }
impl TWxShowEvent for WxShowEvent {}
impl TWxEvent for WxShowEvent {}
impl TWxObject for WxShowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxShowEvent {
    pub fn from(ptr: *mut c_void) -> WxShowEvent { WxShowEvent { ptr: ptr } }
    pub fn null() -> WxShowEvent { WxShowEvent::from(0 as *mut c_void) }
    
}

pub trait TWxShowEvent : TWxEvent {
    fn isShown(&self) -> c_int {
        unsafe { wxShowEvent_IsShown(self.ptr()) }
    }
    fn setShow(&self, show: c_int) {
        unsafe { wxShowEvent_SetShow(self.ptr(), show) }
    }
}

pub struct WxSimpleHelpProvider { ptr: *mut c_void }
impl TWxSimpleHelpProvider for WxSimpleHelpProvider {}
impl TWxHelpProvider for WxSimpleHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSimpleHelpProvider {
    pub fn from(ptr: *mut c_void) -> WxSimpleHelpProvider { WxSimpleHelpProvider { ptr: ptr } }
    pub fn null() -> WxSimpleHelpProvider { WxSimpleHelpProvider::from(0 as *mut c_void) }
    
    pub fn new() -> WxSimpleHelpProvider {
        unsafe { WxSimpleHelpProvider { ptr: wxSimpleHelpProvider_Create() } }
    }
}

pub trait TWxSimpleHelpProvider : TWxHelpProvider {
}

pub struct WxSingleChoiceDialog { ptr: *mut c_void }
impl TWxSingleChoiceDialog for WxSingleChoiceDialog {}
impl TWxDialog for WxSingleChoiceDialog {}
impl TWxTopLevelWindow for WxSingleChoiceDialog {}
impl TWxWindow for WxSingleChoiceDialog {}
impl TWxEvtHandler for WxSingleChoiceDialog {}
impl TWxObject for WxSingleChoiceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSingleChoiceDialog {
    pub fn from(ptr: *mut c_void) -> WxSingleChoiceDialog { WxSingleChoiceDialog { ptr: ptr } }
    pub fn null() -> WxSingleChoiceDialog { WxSingleChoiceDialog::from(0 as *mut c_void) }
    
}

pub trait TWxSingleChoiceDialog : TWxDialog {
}

pub struct WxSize { ptr: *mut c_void }
impl TWxSize for WxSize { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSize {
    pub fn from(ptr: *mut c_void) -> WxSize { WxSize { ptr: ptr } }
    pub fn null() -> WxSize { WxSize::from(0 as *mut c_void) }
    
    pub fn new(w: c_int, h: c_int) -> WxSize {
        unsafe { WxSize { ptr: wxSize_Create(w, h) } }
    }
}

pub trait TWxSize {
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

pub struct WxSizeEvent { ptr: *mut c_void }
impl TWxSizeEvent for WxSizeEvent {}
impl TWxEvent for WxSizeEvent {}
impl TWxObject for WxSizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSizeEvent {
    pub fn from(ptr: *mut c_void) -> WxSizeEvent { WxSizeEvent { ptr: ptr } }
    pub fn null() -> WxSizeEvent { WxSizeEvent::from(0 as *mut c_void) }
    
}

pub trait TWxSizeEvent : TWxEvent {
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizeEvent_GetSize(self.ptr()) } }
    }
}

pub struct WxSizer { ptr: *mut c_void }
impl TWxSizer for WxSizer {}
impl TWxObject for WxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSizer {
    pub fn from(ptr: *mut c_void) -> WxSizer { WxSizer { ptr: ptr } }
    pub fn null() -> WxSizer { WxSizer::from(0 as *mut c_void) }
    
}

pub trait TWxSizer : TWxObject {
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Add(self.ptr(), width, height, option, flag, border, userData) }
    }
    fn addSizer<T: TWxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    fn addWindow<T: TWxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddWindow(self.ptr(), window.ptr(), option, flag, border, userData) }
    }
    fn calcMin(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizer_CalcMin(self.ptr()) } }
    }
    fn fit<T: TWxWindow>(&self, window: &T) {
        unsafe { wxSizer_Fit(self.ptr(), window.ptr()) }
    }
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.ptr(), _res, _cnt) }
    }
    fn getMinSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizer_GetMinSize(self.ptr()) } }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxSizer_GetPosition(self.ptr()) } }
    }
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizer_GetSize(self.ptr()) } }
    }
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Insert(self.ptr(), before, width, height, option, flag, border, userData) }
    }
    fn insertSizer<T: TWxSizer>(&self, before: c_int, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertSizer(self.ptr(), before, sizer.ptr(), option, flag, border, userData) }
    }
    fn insertWindow<T: TWxWindow>(&self, before: c_int, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertWindow(self.ptr(), before, window.ptr(), option, flag, border, userData) }
    }
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.ptr()) }
    }
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Prepend(self.ptr(), width, height, option, flag, border, userData) }
    }
    fn prependSizer<T: TWxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    fn prependWindow<T: TWxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
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
    fn setItemMinSizeSizer<T: TWxSizer>(&self, sizer: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.ptr(), sizer.ptr(), width, height) }
    }
    fn setItemMinSizeWindow<T: TWxWindow>(&self, window: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.ptr(), window.ptr(), width, height) }
    }
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.ptr(), width, height) }
    }
    fn setSizeHints<T: TWxWindow>(&self, window: &T) {
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
    fn detachWindow<T: TWxWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_DetachWindow(self.ptr(), window.ptr()) }
    }
    fn detachSizer<T: TWxSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_DetachSizer(self.ptr(), sizer.ptr()) }
    }
    fn detach(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Detach(self.ptr(), index) }
    }
    fn fitInside<T: TWxWindow>(&self, window: &T) {
        unsafe { wxSizer_FitInside(self.ptr(), window.ptr()) }
    }
    fn getContainingWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxSizer_GetContainingWindow(self.ptr()) } }
    }
    fn getItemWindow<T: TWxWindow>(&self, window: &T, recursive: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_GetItemWindow(self.ptr(), window.ptr(), recursive) } }
    }
    fn getItemSizer<T: TWxSizer>(&self, window: &T, recursive: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_GetItemSizer(self.ptr(), window.ptr(), recursive) } }
    }
    fn getItem(&self, index: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_GetItem(self.ptr(), index) } }
    }
    fn hideWindow<T: TWxWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_HideWindow(self.ptr(), window.ptr()) }
    }
    fn hideSizer<T: TWxSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_HideSizer(self.ptr(), sizer.ptr()) }
    }
    fn hide(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Hide(self.ptr(), index) }
    }
    fn insertSpacer(&self, index: c_int, size: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_InsertSpacer(self.ptr(), index, size) } }
    }
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_InsertStretchSpacer(self.ptr(), index, prop) } }
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
    fn prependSpacer(&self, size: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_PrependSpacer(self.ptr(), size) } }
    }
    fn prependStretchSpacer(&self, prop: c_int) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizer_PrependStretchSpacer(self.ptr(), prop) } }
    }
    fn replaceWindow<T: TWxWindow, U: TWxWindow>(&self, oldwin: &T, newwin: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceWindow(self.ptr(), oldwin.ptr(), newwin.ptr(), recursive) }
    }
    fn replaceSizer<T: TWxSizer, U: TWxSizer>(&self, oldsz: &T, newsz: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceSizer(self.ptr(), oldsz.ptr(), newsz.ptr(), recursive) }
    }
    fn replace<T: TWxSizerItem>(&self, oldindex: c_int, newitem: &T) -> c_int {
        unsafe { wxSizer_Replace(self.ptr(), oldindex, newitem.ptr()) }
    }
    fn setVirtualSizeHints<T: TWxWindow>(&self, window: &T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.ptr(), window.ptr()) }
    }
    fn showWindow<T: TWxWindow>(&self, window: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowWindow(self.ptr(), window.ptr(), show, recursive) }
    }
    fn showSizer<T: TWxSizer>(&self, sizer: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowSizer(self.ptr(), sizer.ptr(), show, recursive) }
    }
    fn show<T: TWxSizer>(&self, sizer: &T, index: c_int, show: c_int) -> c_int {
        unsafe { wxSizer_Show(self.ptr(), sizer.ptr(), index, show) }
    }
}

pub struct WxSizerItem { ptr: *mut c_void }
impl TWxSizerItem for WxSizerItem {}
impl TWxObject for WxSizerItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSizerItem {
    pub fn from(ptr: *mut c_void) -> WxSizerItem { WxSizerItem { ptr: ptr } }
    pub fn null() -> WxSizerItem { WxSizerItem::from(0 as *mut c_void) }
    
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> WxSizerItem {
        unsafe { WxSizerItem { ptr: wxSizerItem_Create(width, height, option, flag, border, userData) } }
    }
    pub fn newInSizer<T: TWxSizer>(sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInSizer(sizer.ptr(), option, flag, border, userData) }
    }
    pub fn newInWindow<T: TWxWindow>(window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInWindow(window.ptr(), option, flag, border, userData) }
    }
}

pub trait TWxSizerItem : TWxObject {
    fn calcMin(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizerItem_CalcMin(self.ptr()) } }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxSizerItem_GetBorder(self.ptr()) }
    }
    fn getFlag(&self) -> c_int {
        unsafe { wxSizerItem_GetFlag(self.ptr()) }
    }
    fn getMinSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizerItem_GetMinSize(self.ptr()) } }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxSizerItem_GetPosition(self.ptr()) } }
    }
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.ptr()) }
    }
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizerItem_GetSize(self.ptr()) } }
    }
    fn getSizer(&self) -> WxSizer {
        unsafe { WxSizer { ptr: wxSizerItem_GetSizer(self.ptr()) } }
    }
    fn getUserData(&self) -> *mut c_void {
        unsafe { wxSizerItem_GetUserData(self.ptr()) }
    }
    fn getWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxSizerItem_GetWindow(self.ptr()) } }
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
    fn setSizer<T: TWxSizer>(&self, sizer: &T) {
        unsafe { wxSizerItem_SetSizer(self.ptr(), sizer.ptr()) }
    }
    fn setWindow<T: TWxWindow>(&self, window: &T) {
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
    fn getRect(&self) -> WxRect {
        unsafe { WxRect { ptr: wxSizerItem_GetRect(self.ptr()) } }
    }
    fn getSpacer(&self) -> WxSize {
        unsafe { WxSize { ptr: wxSizerItem_GetSpacer(self.ptr()) } }
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

pub struct WxSlider { ptr: *mut c_void }
impl TWxSlider for WxSlider {}
impl TWxControl for WxSlider {}
impl TWxWindow for WxSlider {}
impl TWxEvtHandler for WxSlider {}
impl TWxObject for WxSlider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSlider {
    pub fn from(ptr: *mut c_void) -> WxSlider { WxSlider { ptr: ptr } }
    pub fn null() -> WxSlider { WxSlider::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> WxSlider {
        unsafe { WxSlider { ptr: wxSlider_Create(_prt.ptr(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxSlider : TWxControl {
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

pub struct WxSpinButton { ptr: *mut c_void }
impl TWxSpinButton for WxSpinButton {}
impl TWxControl for WxSpinButton {}
impl TWxWindow for WxSpinButton {}
impl TWxEvtHandler for WxSpinButton {}
impl TWxObject for WxSpinButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSpinButton {
    pub fn from(ptr: *mut c_void) -> WxSpinButton { WxSpinButton { ptr: ptr } }
    pub fn null() -> WxSpinButton { WxSpinButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> WxSpinButton {
        unsafe { WxSpinButton { ptr: wxSpinButton_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxSpinButton : TWxControl {
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

pub struct WxSpinCtrl { ptr: *mut c_void }
impl TWxSpinCtrl for WxSpinCtrl {}
impl TWxControl for WxSpinCtrl {}
impl TWxWindow for WxSpinCtrl {}
impl TWxEvtHandler for WxSpinCtrl {}
impl TWxObject for WxSpinCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSpinCtrl {
    pub fn from(ptr: *mut c_void) -> WxSpinCtrl { WxSpinCtrl { ptr: ptr } }
    pub fn null() -> WxSpinCtrl { WxSpinCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> WxSpinCtrl {
        let _txt = wxT(_txt);
        unsafe { WxSpinCtrl { ptr: wxSpinCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init) } }
    }
}

pub trait TWxSpinCtrl : TWxControl {
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

pub struct WxSpinEvent { ptr: *mut c_void }
impl TWxSpinEvent for WxSpinEvent {}
impl TWxNotifyEvent for WxSpinEvent {}
impl TWxCommandEvent for WxSpinEvent {}
impl TWxEvent for WxSpinEvent {}
impl TWxObject for WxSpinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSpinEvent {
    pub fn from(ptr: *mut c_void) -> WxSpinEvent { WxSpinEvent { ptr: ptr } }
    pub fn null() -> WxSpinEvent { WxSpinEvent::from(0 as *mut c_void) }
    
}

pub trait TWxSpinEvent : TWxNotifyEvent {
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.ptr()) }
    }
    fn setPosition(&self, pos: c_int) {
        unsafe { wxSpinEvent_SetPosition(self.ptr(), pos) }
    }
}

pub struct WxSplitterEvent { ptr: *mut c_void }
impl TWxSplitterEvent for WxSplitterEvent {}
impl TWxNotifyEvent for WxSplitterEvent {}
impl TWxCommandEvent for WxSplitterEvent {}
impl TWxEvent for WxSplitterEvent {}
impl TWxObject for WxSplitterEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSplitterEvent {
    pub fn from(ptr: *mut c_void) -> WxSplitterEvent { WxSplitterEvent { ptr: ptr } }
    pub fn null() -> WxSplitterEvent { WxSplitterEvent::from(0 as *mut c_void) }
    
}

pub trait TWxSplitterEvent : TWxNotifyEvent {
}

pub struct WxSplitterWindow { ptr: *mut c_void }
impl TWxSplitterWindow for WxSplitterWindow {}
impl TWxWindow for WxSplitterWindow {}
impl TWxEvtHandler for WxSplitterWindow {}
impl TWxObject for WxSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSplitterWindow {
    pub fn from(ptr: *mut c_void) -> WxSplitterWindow { WxSplitterWindow { ptr: ptr } }
    pub fn null() -> WxSplitterWindow { WxSplitterWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxSplitterWindow {
        unsafe { WxSplitterWindow { ptr: wxSplitterWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxSplitterWindow : TWxWindow {
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
    fn getWindow1(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxSplitterWindow_GetWindow1(self.ptr()) } }
    }
    fn getWindow2(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxSplitterWindow_GetWindow2(self.ptr()) } }
    }
    fn initialize<T: TWxWindow>(&self, window: &T) {
        unsafe { wxSplitterWindow_Initialize(self.ptr(), window.ptr()) }
    }
    fn isSplit(&self) -> c_int {
        unsafe { wxSplitterWindow_IsSplit(self.ptr()) }
    }
    fn replaceWindow<T: TWxWindow, U: TWxWindow>(&self, winOld: &T, winNew: &U) -> c_int {
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
    fn splitHorizontally<T: TWxWindow, U: TWxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitHorizontally(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    fn splitVertically<T: TWxWindow, U: TWxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitVertically(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    fn unsplit<T: TWxWindow>(&self, toRemove: &T) -> c_int {
        unsafe { wxSplitterWindow_Unsplit(self.ptr(), toRemove.ptr()) }
    }
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self.ptr()) }
    }
    fn setSashGravity(&self, gravity: c_double) {
        unsafe { wxSplitterWindow_SetSashGravity(self.ptr(), gravity) }
    }
}

pub struct WxStaticBitmap { ptr: *mut c_void }
impl TWxStaticBitmap for WxStaticBitmap {}
impl TWxControl for WxStaticBitmap {}
impl TWxWindow for WxStaticBitmap {}
impl TWxEvtHandler for WxStaticBitmap {}
impl TWxObject for WxStaticBitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStaticBitmap {
    pub fn from(ptr: *mut c_void) -> WxStaticBitmap { WxStaticBitmap { ptr: ptr } }
    pub fn null() -> WxStaticBitmap { WxStaticBitmap::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxBitmap>(_prt: &T, _id: c_int, bitmap: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxStaticBitmap {
        unsafe { WxStaticBitmap { ptr: wxStaticBitmap_Create(_prt.ptr(), _id, bitmap.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxStaticBitmap : TWxControl {
    fn getBitmap<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    fn getIcon<T: TWxIcon>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetIcon(self.ptr(), _ref.ptr()) }
    }
    fn setBitmap<T: TWxBitmap>(&self, bitmap: &T) {
        unsafe { wxStaticBitmap_SetBitmap(self.ptr(), bitmap.ptr()) }
    }
    fn setIcon<T: TWxIcon>(&self, icon: &T) {
        unsafe { wxStaticBitmap_SetIcon(self.ptr(), icon.ptr()) }
    }
}

pub struct WxStaticBox { ptr: *mut c_void }
impl TWxStaticBox for WxStaticBox {}
impl TWxControl for WxStaticBox {}
impl TWxWindow for WxStaticBox {}
impl TWxEvtHandler for WxStaticBox {}
impl TWxObject for WxStaticBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStaticBox {
    pub fn from(ptr: *mut c_void) -> WxStaticBox { WxStaticBox { ptr: ptr } }
    pub fn null() -> WxStaticBox { WxStaticBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxStaticBox {
        let _txt = wxT(_txt);
        unsafe { WxStaticBox { ptr: wxStaticBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxStaticBox : TWxControl {
}

pub struct WxStaticBoxSizer { ptr: *mut c_void }
impl TWxStaticBoxSizer for WxStaticBoxSizer {}
impl TWxBoxSizer for WxStaticBoxSizer {}
impl TWxSizer for WxStaticBoxSizer {}
impl TWxObject for WxStaticBoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStaticBoxSizer {
    pub fn from(ptr: *mut c_void) -> WxStaticBoxSizer { WxStaticBoxSizer { ptr: ptr } }
    pub fn null() -> WxStaticBoxSizer { WxStaticBoxSizer::from(0 as *mut c_void) }
    
    pub fn new<T: TWxStaticBox>(box_: &T, orient: c_int) -> WxStaticBoxSizer {
        unsafe { WxStaticBoxSizer { ptr: wxStaticBoxSizer_Create(box_.ptr(), orient) } }
    }
}

pub trait TWxStaticBoxSizer : TWxBoxSizer {
    fn getStaticBox(&self) -> WxStaticBox {
        unsafe { WxStaticBox { ptr: wxStaticBoxSizer_GetStaticBox(self.ptr()) } }
    }
}

pub struct WxStaticLine { ptr: *mut c_void }
impl TWxStaticLine for WxStaticLine {}
impl TWxControl for WxStaticLine {}
impl TWxWindow for WxStaticLine {}
impl TWxEvtHandler for WxStaticLine {}
impl TWxObject for WxStaticLine { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStaticLine {
    pub fn from(ptr: *mut c_void) -> WxStaticLine { WxStaticLine { ptr: ptr } }
    pub fn null() -> WxStaticLine { WxStaticLine::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxStaticLine {
        unsafe { WxStaticLine { ptr: wxStaticLine_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxStaticLine : TWxControl {
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.ptr()) }
    }
    fn isVertical(&self) -> c_int {
        unsafe { wxStaticLine_IsVertical(self.ptr()) }
    }
}

pub struct WxStaticText { ptr: *mut c_void }
impl TWxStaticText for WxStaticText {}
impl TWxControl for WxStaticText {}
impl TWxWindow for WxStaticText {}
impl TWxEvtHandler for WxStaticText {}
impl TWxObject for WxStaticText { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStaticText {
    pub fn from(ptr: *mut c_void) -> WxStaticText { WxStaticText { ptr: ptr } }
    pub fn null() -> WxStaticText { WxStaticText::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxStaticText {
        let _txt = wxT(_txt);
        unsafe { WxStaticText { ptr: wxStaticText_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxStaticText : TWxControl {
}

pub struct WxStatusBar { ptr: *mut c_void }
impl TWxStatusBar for WxStatusBar {}
impl TWxWindow for WxStatusBar {}
impl TWxEvtHandler for WxStatusBar {}
impl TWxObject for WxStatusBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxStatusBar {
    pub fn from(ptr: *mut c_void) -> WxStatusBar { WxStatusBar { ptr: ptr } }
    pub fn null() -> WxStatusBar { WxStatusBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxStatusBar {
        unsafe { WxStatusBar { ptr: wxStatusBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxStatusBar : TWxWindow {
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
        unsafe { WxString { ptr: wxStatusBar_GetStatusText(self.ptr(), number) }.to_str() }
    }
    fn setFieldsCount(&self, number: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetFieldsCount(self.ptr(), number, widths) }
    }
    fn setMinHeight(&self, height: c_int) {
        unsafe { wxStatusBar_SetMinHeight(self.ptr(), height) }
    }
    fn setStatusText(&self, text: &str, number: c_int) {
        let text = wxT(text);
        unsafe { wxStatusBar_SetStatusText(self.ptr(), text.ptr(), number) }
    }
    fn setStatusWidths(&self, n: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.ptr(), n, widths) }
    }
}

pub struct WxSysColourChangedEvent { ptr: *mut c_void }
impl TWxSysColourChangedEvent for WxSysColourChangedEvent {}
impl TWxEvent for WxSysColourChangedEvent {}
impl TWxObject for WxSysColourChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSysColourChangedEvent {
    pub fn from(ptr: *mut c_void) -> WxSysColourChangedEvent { WxSysColourChangedEvent { ptr: ptr } }
    pub fn null() -> WxSysColourChangedEvent { WxSysColourChangedEvent::from(0 as *mut c_void) }
    
}

pub trait TWxSysColourChangedEvent : TWxEvent {
}

pub struct WxSystemSettings { ptr: *mut c_void }
impl TWxSystemSettings for WxSystemSettings {}
impl TWxObject for WxSystemSettings { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSystemSettings {
    pub fn from(ptr: *mut c_void) -> WxSystemSettings { WxSystemSettings { ptr: ptr } }
    pub fn null() -> WxSystemSettings { WxSystemSettings::from(0 as *mut c_void) }
    
    pub fn getColour<T: TWxColour>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetColour(index, _ref.ptr()) }
    }
    pub fn getFont<T: TWxFont>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetFont(index, _ref.ptr()) }
    }
    pub fn getMetric(index: c_int) -> c_int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    pub fn getScreenType() -> c_int {
        unsafe { wxSystemSettings_GetScreenType() }
    }
}

pub trait TWxSystemSettings : TWxObject {
}

pub struct WxTextAttr { ptr: *mut c_void }
impl TWxTextAttr for WxTextAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextAttr {
    pub fn from(ptr: *mut c_void) -> WxTextAttr { WxTextAttr { ptr: ptr } }
    pub fn null() -> WxTextAttr { WxTextAttr::from(0 as *mut c_void) }
    
    pub fn new<T: TWxColour, U: TWxColour, V: TWxFont>(colText: &T, colBack: &U, font: &V) -> WxTextAttr {
        unsafe { WxTextAttr { ptr: wxTextAttr_Create(colText.ptr(), colBack.ptr(), font.ptr()) } }
    }
    pub fn newDefault() -> WxTextAttr {
        unsafe { WxTextAttr { ptr: wxTextAttr_CreateDefault() } }
    }
}

pub trait TWxTextAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.ptr()) }
    }
    fn getBackgroundColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_GetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn getFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxTextAttr_GetFont(self.ptr(), font.ptr()) }
    }
    fn getTextColour<T: TWxColour>(&self, colour: &T) {
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
    fn setTextColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetTextColour(self.ptr(), colour.ptr()) }
    }
    fn setBackgroundColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxTextAttr_SetFont(self.ptr(), font.ptr()) }
    }
}

pub struct WxTextCtrl { ptr: *mut c_void }
impl TWxTextCtrl for WxTextCtrl {}
impl TWxControl for WxTextCtrl {}
impl TWxWindow for WxTextCtrl {}
impl TWxEvtHandler for WxTextCtrl {}
impl TWxObject for WxTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextCtrl {
    pub fn from(ptr: *mut c_void) -> WxTextCtrl { WxTextCtrl { ptr: ptr } }
    pub fn null() -> WxTextCtrl { WxTextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> WxTextCtrl {
        let _txt = wxT(_txt);
        unsafe { WxTextCtrl { ptr: wxTextCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxTextCtrl : TWxControl {
    fn appendText(&self, text: &str) {
        let text = wxT(text);
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
        let text = wxT(text);
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
        unsafe { WxString { ptr: wxTextCtrl_GetLineText(self.ptr(), lineNo) }.to_str() }
    }
    fn getNumberOfLines(&self) -> c_int {
        unsafe { wxTextCtrl_GetNumberOfLines(self.ptr()) }
    }
    fn getSelection(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { wxTextCtrl_GetSelection(self.ptr(), from, to) }
    }
    fn getValue(&self) -> ~str {
        unsafe { WxString { ptr: wxTextCtrl_GetValue(self.ptr()) }.to_str() }
    }
    fn isEditable(&self) -> c_int {
        unsafe { wxTextCtrl_IsEditable(self.ptr()) }
    }
    fn isModified(&self) -> c_int {
        unsafe { wxTextCtrl_IsModified(self.ptr()) }
    }
    fn loadFile(&self, file: &str) -> c_int {
        let file = wxT(file);
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
        let value = wxT(value);
        unsafe { wxTextCtrl_Replace(self.ptr(), from, to, value.ptr()) }
    }
    fn saveFile(&self, file: &str) -> c_int {
        let file = wxT(file);
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
        let value = wxT(value);
        unsafe { wxTextCtrl_SetValue(self.ptr(), value.ptr()) }
    }
    fn showPosition(&self, pos: c_long) {
        unsafe { wxTextCtrl_ShowPosition(self.ptr(), pos) }
    }
    fn undo(&self) {
        unsafe { wxTextCtrl_Undo(self.ptr()) }
    }
    fn writeText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_WriteText(self.ptr(), text.ptr()) }
    }
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.ptr(), x, y) }
    }
    fn emulateKeyPress<T: TWxKeyEvent>(&self, keyevent: &T) -> c_int {
        unsafe { wxTextCtrl_EmulateKeyPress(self.ptr(), keyevent.ptr()) }
    }
    fn getDefaultStyle(&self) -> WxTextAttr {
        unsafe { WxTextAttr { ptr: wxTextCtrl_GetDefaultStyle(self.ptr()) } }
    }
    fn getRange(&self, from: c_long, to: c_long) -> ~str {
        unsafe { WxString { ptr: wxTextCtrl_GetRange(self.ptr(), from, to) }.to_str() }
    }
    fn getStringSelection(&self) -> ~str {
        unsafe { WxString { ptr: wxTextCtrl_GetStringSelection(self.ptr()) }.to_str() }
    }
    fn isMultiLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsMultiLine(self.ptr()) }
    }
    fn isSingleLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsSingleLine(self.ptr()) }
    }
    fn setDefaultStyle<T: TWxTextAttr>(&self, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetDefaultStyle(self.ptr(), style.ptr()) }
    }
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.ptr(), len) }
    }
    fn setStyle<T: TWxTextAttr>(&self, start: c_long, end: c_long, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetStyle(self.ptr(), start, end, style.ptr()) }
    }
}

pub struct WxTextDataObject { ptr: *mut c_void }
impl TWxTextDataObject for WxTextDataObject {}
impl TWxDataObjectSimple for WxTextDataObject {}
impl TWxDataObject for WxTextDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextDataObject {
    pub fn from(ptr: *mut c_void) -> WxTextDataObject { WxTextDataObject { ptr: ptr } }
    pub fn null() -> WxTextDataObject { WxTextDataObject::from(0 as *mut c_void) }
    
}

pub trait TWxTextDataObject : TWxDataObjectSimple {
}

pub struct WxTextDropTarget { ptr: *mut c_void }
impl TWxTextDropTarget for WxTextDropTarget {}
impl TWxDropTarget for WxTextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextDropTarget {
    pub fn from(ptr: *mut c_void) -> WxTextDropTarget { WxTextDropTarget { ptr: ptr } }
    pub fn null() -> WxTextDropTarget { WxTextDropTarget::from(0 as *mut c_void) }
    
}

pub trait TWxTextDropTarget : TWxDropTarget {
}

pub struct WxTextEntryDialog { ptr: *mut c_void }
impl TWxTextEntryDialog for WxTextEntryDialog {}
impl TWxDialog for WxTextEntryDialog {}
impl TWxTopLevelWindow for WxTextEntryDialog {}
impl TWxWindow for WxTextEntryDialog {}
impl TWxEvtHandler for WxTextEntryDialog {}
impl TWxObject for WxTextEntryDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextEntryDialog {
    pub fn from(ptr: *mut c_void) -> WxTextEntryDialog { WxTextEntryDialog { ptr: ptr } }
    pub fn null() -> WxTextEntryDialog { WxTextEntryDialog::from(0 as *mut c_void) }
    
}

pub trait TWxTextEntryDialog : TWxDialog {
}

pub struct WxTextValidator { ptr: *mut c_void }
impl TWxTextValidator for WxTextValidator {}
impl TWxValidator for WxTextValidator {}
impl TWxEvtHandler for WxTextValidator {}
impl TWxObject for WxTextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTextValidator {
    pub fn from(ptr: *mut c_void) -> WxTextValidator { WxTextValidator { ptr: ptr } }
    pub fn null() -> WxTextValidator { WxTextValidator::from(0 as *mut c_void) }
    
    pub fn new(style: c_int, val: *mut c_void) -> WxTextValidator {
        unsafe { WxTextValidator { ptr: wxTextValidator_Create(style, val) } }
    }
}

pub trait TWxTextValidator : TWxValidator {
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
    fn clone(&self) -> WxValidator {
        unsafe { WxValidator { ptr: wxTextValidator_Clone(self.ptr()) } }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxTextValidator_GetStyle(self.ptr()) }
    }
    fn onChar<T: TWxEvent>(&self, event: &T) {
        unsafe { wxTextValidator_OnChar(self.ptr(), event.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.ptr(), style) }
    }
}

pub struct WxTimer { ptr: *mut c_void }
impl TWxTimer for WxTimer {}
impl TWxObject for WxTimer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTimer {
    pub fn from(ptr: *mut c_void) -> WxTimer { WxTimer { ptr: ptr } }
    pub fn null() -> WxTimer { WxTimer::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int) -> WxTimer {
        unsafe { WxTimer { ptr: wxTimer_Create(_prt.ptr(), _id) } }
    }
}

pub trait TWxTimer : TWxObject {
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

pub struct WxTimerEvent { ptr: *mut c_void }
impl TWxTimerEvent for WxTimerEvent {}
impl TWxEvent for WxTimerEvent {}
impl TWxObject for WxTimerEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTimerEvent {
    pub fn from(ptr: *mut c_void) -> WxTimerEvent { WxTimerEvent { ptr: ptr } }
    pub fn null() -> WxTimerEvent { WxTimerEvent::from(0 as *mut c_void) }
    
}

pub trait TWxTimerEvent : TWxEvent {
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.ptr()) }
    }
}

pub struct WxTimerEx { ptr: *mut c_void }
impl TWxTimerEx for WxTimerEx {}
impl TWxTimer for WxTimerEx {}
impl TWxObject for WxTimerEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTimerEx {
    pub fn from(ptr: *mut c_void) -> WxTimerEx { WxTimerEx { ptr: ptr } }
    pub fn null() -> WxTimerEx { WxTimerEx::from(0 as *mut c_void) }
    
    pub fn new() -> WxTimerEx {
        unsafe { WxTimerEx { ptr: wxTimerEx_Create() } }
    }
}

pub trait TWxTimerEx : TWxTimer {
    fn connect<T: TWxClosure>(&self, closure: &T) {
        unsafe { wxTimerEx_Connect(self.ptr(), closure.ptr()) }
    }
    fn getClosure(&self) -> WxClosure {
        unsafe { WxClosure { ptr: wxTimerEx_GetClosure(self.ptr()) } }
    }
}

pub struct WxTimerRunner { ptr: *mut c_void }
impl TWxTimerRunner for WxTimerRunner { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTimerRunner {
    pub fn from(ptr: *mut c_void) -> WxTimerRunner { WxTimerRunner { ptr: ptr } }
    pub fn null() -> WxTimerRunner { WxTimerRunner::from(0 as *mut c_void) }
    
}

pub trait TWxTimerRunner {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxTipWindow { ptr: *mut c_void }
impl TWxTipWindow for WxTipWindow {}
impl TWxPopupTransientWindow for WxTipWindow {}
impl TWxPopupWindow for WxTipWindow {}
impl TWxWindow for WxTipWindow {}
impl TWxEvtHandler for WxTipWindow {}
impl TWxObject for WxTipWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTipWindow {
    pub fn from(ptr: *mut c_void) -> WxTipWindow { WxTipWindow { ptr: ptr } }
    pub fn null() -> WxTipWindow { WxTipWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(parent: &T, text: &str, maxLength: c_int) -> WxTipWindow {
        let text = wxT(text);
        unsafe { WxTipWindow { ptr: wxTipWindow_Create(parent.ptr(), text.ptr(), maxLength) } }
    }
}

pub trait TWxTipWindow : TWxPopupTransientWindow {
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.ptr(), x, y, w, h) }
    }
    fn setTipWindowPtr(&self, windowPtr: *mut c_void) {
        unsafe { wxTipWindow_SetTipWindowPtr(self.ptr(), windowPtr) }
    }
}

pub struct WxToggleButton { ptr: *mut c_void }
impl TWxToggleButton for WxToggleButton {}
impl TWxControl for WxToggleButton {}
impl TWxWindow for WxToggleButton {}
impl TWxEvtHandler for WxToggleButton {}
impl TWxObject for WxToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxToggleButton {
    pub fn from(ptr: *mut c_void) -> WxToggleButton { WxToggleButton { ptr: ptr } }
    pub fn null() -> WxToggleButton { WxToggleButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(parent: &T, id: c_int, label: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> WxToggleButton {
        let label = wxT(label);
        unsafe { WxToggleButton { ptr: wxToggleButton_Create(parent.ptr(), id, label.ptr(), x, y, w, h, style) } }
    }
}

pub trait TWxToggleButton : TWxControl {
    fn getValue(&self) -> c_int {
        unsafe { wxToggleButton_GetValue(self.ptr()) }
    }
    fn setValue(&self, state: c_int) {
        unsafe { wxToggleButton_SetValue(self.ptr(), state) }
    }
}

pub struct WxToolBar { ptr: *mut c_void }
impl TWxToolBar for WxToolBar {}
impl TWxToolBarBase for WxToolBar {}
impl TWxControl for WxToolBar {}
impl TWxWindow for WxToolBar {}
impl TWxEvtHandler for WxToolBar {}
impl TWxObject for WxToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxToolBar {
    pub fn from(ptr: *mut c_void) -> WxToolBar { WxToolBar { ptr: ptr } }
    pub fn null() -> WxToolBar { WxToolBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxToolBar {
        unsafe { WxToolBar { ptr: wxToolBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxToolBar : TWxToolBarBase {
    fn addControl<T: TWxControl>(&self, ctrl: &T) -> c_int {
        unsafe { wxToolBar_AddControl(self.ptr(), ctrl.ptr()) }
    }
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.ptr()) }
    }
    fn addTool<T: TWxBitmap>(&self, id: c_int, bmp: &T, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddTool(self.ptr(), id, bmp.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    fn addToolEx<T: TWxBitmap, U: TWxBitmap, V: TWxObject>(&self, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, x: c_int, y: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
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
    fn getMargins(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxToolBar_GetMargins(self.ptr()) } }
    }
    fn getToolBitmapSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxToolBar_GetToolBitmapSize(self.ptr()) } }
    }
    fn getToolClientData(&self, id: c_int) -> WxObject {
        unsafe { WxObject { ptr: wxToolBar_GetToolClientData(self.ptr(), id) } }
    }
    fn getToolEnabled(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolEnabled(self.ptr(), id) }
    }
    fn getToolLongHelp(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxToolBar_GetToolLongHelp(self.ptr(), id) }.to_str() }
    }
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.ptr()) }
    }
    fn getToolShortHelp(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxToolBar_GetToolShortHelp(self.ptr(), id) }.to_str() }
    }
    fn getToolSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxToolBar_GetToolSize(self.ptr()) } }
    }
    fn getToolState(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolState(self.ptr(), id) }
    }
    fn insertControl<T: TWxControl>(&self, pos: c_int, ctrl: &T) {
        unsafe { wxToolBar_InsertControl(self.ptr(), pos, ctrl.ptr()) }
    }
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.ptr(), pos) }
    }
    fn insertTool<T: TWxBitmap, U: TWxBitmap, V: TWxObject>(&self, pos: c_int, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
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
    fn setToolClientData<T: TWxObject>(&self, id: c_int, data: &T) {
        unsafe { wxToolBar_SetToolClientData(self.ptr(), id, data.ptr()) }
    }
    fn setToolLongHelp(&self, id: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxToolBar_SetToolLongHelp(self.ptr(), id, str.ptr()) }
    }
    fn setToolPacking(&self, packing: c_int) {
        unsafe { wxToolBar_SetToolPacking(self.ptr(), packing) }
    }
    fn setToolSeparation(&self, separation: c_int) {
        unsafe { wxToolBar_SetToolSeparation(self.ptr(), separation) }
    }
    fn setToolShortHelp(&self, id: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxToolBar_SetToolShortHelp(self.ptr(), id, str.ptr()) }
    }
    fn toggleTool(&self, id: c_int, toggle: c_int) {
        unsafe { wxToolBar_ToggleTool(self.ptr(), id, toggle) }
    }
    fn addTool2<T: TWxBitmap, U: TWxBitmap>(&self, toolId: c_int, label: &str, bmp: &T, bmpDisabled: &U, itemKind: c_int, shortHelp: &str, longHelp: &str) {
        let label = wxT(label);
        let shortHelp = wxT(shortHelp);
        let longHelp = wxT(longHelp);
        unsafe { wxToolBar_AddTool2(self.ptr(), toolId, label.ptr(), bmp.ptr(), bmpDisabled.ptr(), itemKind, shortHelp.ptr(), longHelp.ptr()) }
    }
}

pub struct WxToolBarBase { ptr: *mut c_void }
impl TWxToolBarBase for WxToolBarBase {}
impl TWxControl for WxToolBarBase {}
impl TWxWindow for WxToolBarBase {}
impl TWxEvtHandler for WxToolBarBase {}
impl TWxObject for WxToolBarBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxToolBarBase {
    pub fn from(ptr: *mut c_void) -> WxToolBarBase { WxToolBarBase { ptr: ptr } }
    pub fn null() -> WxToolBarBase { WxToolBarBase::from(0 as *mut c_void) }
    
}

pub trait TWxToolBarBase : TWxControl {
}

pub struct WxToolTip { ptr: *mut c_void }
impl TWxToolTip for WxToolTip {}
impl TWxObject for WxToolTip { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxToolTip {
    pub fn from(ptr: *mut c_void) -> WxToolTip { WxToolTip { ptr: ptr } }
    pub fn null() -> WxToolTip { WxToolTip::from(0 as *mut c_void) }
    
}

pub trait TWxToolTip : TWxObject {
}

pub struct WxTopLevelWindow { ptr: *mut c_void }
impl TWxTopLevelWindow for WxTopLevelWindow {}
impl TWxWindow for WxTopLevelWindow {}
impl TWxEvtHandler for WxTopLevelWindow {}
impl TWxObject for WxTopLevelWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTopLevelWindow {
    pub fn from(ptr: *mut c_void) -> WxTopLevelWindow { WxTopLevelWindow { ptr: ptr } }
    pub fn null() -> WxTopLevelWindow { WxTopLevelWindow::from(0 as *mut c_void) }
    
}

pub trait TWxTopLevelWindow : TWxWindow {
    fn enableCloseButton(&self, enable: c_int) -> c_int {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.ptr(), enable) }
    }
    fn getDefaultButton(&self) -> WxButton {
        unsafe { WxButton { ptr: wxTopLevelWindow_GetDefaultButton(self.ptr()) } }
    }
    fn getDefaultItem(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxTopLevelWindow_GetDefaultItem(self.ptr()) } }
    }
    fn getIcon(&self) -> WxIcon {
        unsafe { WxIcon { ptr: wxTopLevelWindow_GetIcon(self.ptr()) } }
    }
    fn getTitle(&self) -> ~str {
        unsafe { WxString { ptr: wxTopLevelWindow_GetTitle(self.ptr()) }.to_str() }
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
    fn setDefaultButton<T: TWxButton>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.ptr(), pBut.ptr()) }
    }
    fn setDefaultItem<T: TWxWindow>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.ptr(), pBut.ptr()) }
    }
    fn setIcon<T: TWxIcon>(&self, pIcon: &T) {
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
        let pString = wxT(pString);
        unsafe { wxTopLevelWindow_SetTitle(self.ptr(), pString.ptr()) }
    }
}

pub struct WxTreeCtrl { ptr: *mut c_void }
impl TWxTreeCtrl for WxTreeCtrl {}
impl TWxControl for WxTreeCtrl {}
impl TWxWindow for WxTreeCtrl {}
impl TWxEvtHandler for WxTreeCtrl {}
impl TWxObject for WxTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeCtrl {
    pub fn from(ptr: *mut c_void) -> WxTreeCtrl { WxTreeCtrl { ptr: ptr } }
    pub fn null() -> WxTreeCtrl { WxTreeCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_obj: *mut c_void, _cmp: *mut c_void, _prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxTreeCtrl {
        unsafe { WxTreeCtrl { ptr: wxTreeCtrl_Create(_obj, _cmp, _prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
    pub fn new2<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxTreeCtrl {
        unsafe { WxTreeCtrl { ptr: wxTreeCtrl_Create2(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxTreeCtrl : TWxControl {
    fn addRoot<T: TWxTreeItemData, U: TWxTreeItemId>(&self, text: &str, image: c_int, selectedImage: c_int, data: &T, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AddRoot(self.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    fn appendItem<T: TWxTreeItemId, U: TWxTreeItemData, V: TWxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AppendItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    fn collapse<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Collapse(self.ptr(), item.ptr()) }
    }
    fn collapseAndReset<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.ptr(), item.ptr()) }
    }
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.ptr()) }
    }
    fn deleteChildren<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_DeleteChildren(self.ptr(), item.ptr()) }
    }
    fn editLabel<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EditLabel(self.ptr(), item.ptr()) }
    }
    fn endEditLabel<T: TWxTreeItemId>(&self, item: &T, discardChanges: c_int) {
        unsafe { wxTreeCtrl_EndEditLabel(self.ptr(), item.ptr(), discardChanges) }
    }
    fn ensureVisible<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EnsureVisible(self.ptr(), item.ptr()) }
    }
    fn expand<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Expand(self.ptr(), item.ptr()) }
    }
    fn getBoundingRect<T: TWxTreeItemId>(&self, item: &T, textOnly: c_int) -> WxRect {
        unsafe { WxRect { ptr: wxTreeCtrl_GetBoundingRect(self.ptr(), item.ptr(), textOnly) } }
    }
    fn getChildrenCount<T: TWxTreeItemId>(&self, item: &T, recursively: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.ptr(), item.ptr(), recursively) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.ptr()) }
    }
    fn getEditControl(&self) -> WxTextCtrl {
        unsafe { WxTextCtrl { ptr: wxTreeCtrl_GetEditControl(self.ptr()) } }
    }
    fn getFirstChild<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    fn getFirstVisibleItem<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getImageList(&self) -> WxImageList {
        unsafe { WxImageList { ptr: wxTreeCtrl_GetImageList(self.ptr()) } }
    }
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.ptr()) }
    }
    fn getItemData<T: TWxTreeItemId>(&self, item: &T) -> *mut c_void {
        unsafe { wxTreeCtrl_GetItemData(self.ptr(), item.ptr()) }
    }
    fn getItemImage<T: TWxTreeItemId>(&self, item: &T, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.ptr(), item.ptr(), which) }
    }
    fn getItemText<T: TWxTreeItemId>(&self, item: &T) -> ~str {
        unsafe { WxString { ptr: wxTreeCtrl_GetItemText(self.ptr(), item.ptr()) }.to_str() }
    }
    fn getLastChild<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetLastChild(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getNextChild<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetNextChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    fn getNextSibling<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getNextVisible<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getPrevSibling<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getPrevVisible<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getRootItem<T: TWxTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetRootItem(self.ptr(), _item.ptr()) }
    }
    fn getSelection<T: TWxTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetSelection(self.ptr(), _item.ptr()) }
    }
    fn getSelections(&self, selections: *mut c_void) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self.ptr(), selections) }
    }
    fn getSpacing(&self) -> c_int {
        unsafe { wxTreeCtrl_GetSpacing(self.ptr()) }
    }
    fn getStateImageList(&self) -> WxImageList {
        unsafe { WxImageList { ptr: wxTreeCtrl_GetStateImageList(self.ptr()) } }
    }
    fn hitTest<T: TWxTreeItemId>(&self, _x: c_int, _y: c_int, flags: *mut c_int, _item: &T) {
        unsafe { wxTreeCtrl_HitTest(self.ptr(), _x, _y, flags, _item.ptr()) }
    }
    fn insertItem<T: TWxTreeItemId, U: TWxTreeItemId, V: TWxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn insertItemByIndex<T: TWxTreeItemId, U: TWxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn isBold<T: TWxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsBold(self.ptr(), item.ptr()) }
    }
    fn isExpanded<T: TWxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsExpanded(self.ptr(), item.ptr()) }
    }
    fn isSelected<T: TWxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsSelected(self.ptr(), item.ptr()) }
    }
    fn isVisible<T: TWxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsVisible(self.ptr(), item.ptr()) }
    }
    fn itemHasChildren<T: TWxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.ptr(), item.ptr()) }
    }
    fn onCompareItems<T: TWxTreeItemId, U: TWxTreeItemId>(&self, item1: &T, item2: &U) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.ptr(), item1.ptr(), item2.ptr()) }
    }
    fn prependItem<T: TWxTreeItemId, U: TWxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_PrependItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn scrollTo<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_ScrollTo(self.ptr(), item.ptr()) }
    }
    fn selectItem<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SelectItem(self.ptr(), item.ptr()) }
    }
    fn setImageList<T: TWxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetImageList(self.ptr(), imageList.ptr()) }
    }
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.ptr(), indent) }
    }
    fn setItemBackgroundColour<T: TWxTreeItemId, U: TWxColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    fn setItemBold<T: TWxTreeItemId>(&self, item: &T, bold: c_int) {
        unsafe { wxTreeCtrl_SetItemBold(self.ptr(), item.ptr(), bold) }
    }
    fn setItemData<T: TWxTreeItemId>(&self, item: &T, data: *mut c_void) {
        unsafe { wxTreeCtrl_SetItemData(self.ptr(), item.ptr(), data) }
    }
    fn setItemDropHighlight<T: TWxTreeItemId>(&self, item: &T, highlight: c_int) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.ptr(), item.ptr(), highlight) }
    }
    fn setItemFont<T: TWxTreeItemId, U: TWxFont>(&self, item: &T, font: &U) {
        unsafe { wxTreeCtrl_SetItemFont(self.ptr(), item.ptr(), font.ptr()) }
    }
    fn setItemHasChildren<T: TWxTreeItemId>(&self, item: &T, hasChildren: c_int) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.ptr(), item.ptr(), hasChildren) }
    }
    fn setItemImage<T: TWxTreeItemId>(&self, item: &T, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.ptr(), item.ptr(), image, which) }
    }
    fn setItemText<T: TWxTreeItemId>(&self, item: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_SetItemText(self.ptr(), item.ptr(), text.ptr()) }
    }
    fn setItemTextColour<T: TWxTreeItemId, U: TWxColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.ptr(), spacing) }
    }
    fn setStateImageList<T: TWxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetStateImageList(self.ptr(), imageList.ptr()) }
    }
    fn sortChildren<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SortChildren(self.ptr(), item.ptr()) }
    }
    fn toggle<T: TWxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Toggle(self.ptr(), item.ptr()) }
    }
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self.ptr()) }
    }
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self.ptr()) }
    }
    fn insertItem2<T: TWxWindow, U: TWxTreeItemId, V: TWxClosure, W: TWxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, closure: &V, _item: &W) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem2(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    fn insertItemByIndex2<T: TWxWindow, U: TWxClosure, V: TWxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, closure: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    fn getItemClientClosure<T: TWxTreeItemId>(&self, item: &T) -> WxClosure {
        unsafe { WxClosure { ptr: wxTreeCtrl_GetItemClientClosure(self.ptr(), item.ptr()) } }
    }
    fn setItemClientClosure<T: TWxTreeItemId, U: TWxClosure>(&self, item: &T, closure: &U) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.ptr(), item.ptr(), closure.ptr()) }
    }
    fn assignImageList<T: TWxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignImageList(self.ptr(), imageList.ptr()) }
    }
    fn assignStateImageList<T: TWxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.ptr(), imageList.ptr()) }
    }
}

pub struct WxTreeEvent { ptr: *mut c_void }
impl TWxTreeEvent for WxTreeEvent {}
impl TWxNotifyEvent for WxTreeEvent {}
impl TWxCommandEvent for WxTreeEvent {}
impl TWxEvent for WxTreeEvent {}
impl TWxObject for WxTreeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeEvent {
    pub fn from(ptr: *mut c_void) -> WxTreeEvent { WxTreeEvent { ptr: ptr } }
    pub fn null() -> WxTreeEvent { WxTreeEvent::from(0 as *mut c_void) }
    
}

pub trait TWxTreeEvent : TWxNotifyEvent {
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.ptr()) }
    }
    fn getItem<T: TWxTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxTreeEvent_GetLabel(self.ptr()) }.to_str() }
    }
    fn getOldItem<T: TWxTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetOldItem(self.ptr(), _ref.ptr()) }
    }
    fn getPoint(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxTreeEvent_GetPoint(self.ptr()) } }
    }
    fn getKeyEvent(&self) -> WxKeyEvent {
        unsafe { WxKeyEvent { ptr: wxTreeEvent_GetKeyEvent(self.ptr()) } }
    }
    fn isEditCancelled(&self) -> c_int {
        unsafe { wxTreeEvent_IsEditCancelled(self.ptr()) }
    }
}

pub struct WxTreeItemData { ptr: *mut c_void }
impl TWxTreeItemData for WxTreeItemData {}
impl TWxClientData for WxTreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeItemData {
    pub fn from(ptr: *mut c_void) -> WxTreeItemData { WxTreeItemData { ptr: ptr } }
    pub fn null() -> WxTreeItemData { WxTreeItemData::from(0 as *mut c_void) }
    
}

pub trait TWxTreeItemData : TWxClientData {
}

pub struct WxTreeItemId { ptr: *mut c_void }
impl TWxTreeItemId for WxTreeItemId { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTreeItemId {
    pub fn from(ptr: *mut c_void) -> WxTreeItemId { WxTreeItemId { ptr: ptr } }
    pub fn null() -> WxTreeItemId { WxTreeItemId::from(0 as *mut c_void) }
    
    pub fn new() -> WxTreeItemId {
        unsafe { WxTreeItemId { ptr: wxTreeItemId_Create() } }
    }
    pub fn newFromValue(value: intptr_t) -> WxTreeItemId {
        unsafe { WxTreeItemId { ptr: wxTreeItemId_CreateFromValue(value) } }
    }
}

pub trait TWxTreeItemId {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTreeItemId_IsOk(self.ptr()) }
    }
    fn clone(&self) -> WxTreeItemId {
        unsafe { WxTreeItemId { ptr: wxTreeItemId_Clone(self.ptr()) } }
    }
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.ptr()) }
    }
}

pub struct WxUpdateUIEvent { ptr: *mut c_void }
impl TWxUpdateUIEvent for WxUpdateUIEvent {}
impl TWxEvent for WxUpdateUIEvent {}
impl TWxObject for WxUpdateUIEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxUpdateUIEvent {
    pub fn from(ptr: *mut c_void) -> WxUpdateUIEvent { WxUpdateUIEvent { ptr: ptr } }
    pub fn null() -> WxUpdateUIEvent { WxUpdateUIEvent::from(0 as *mut c_void) }
    
}

pub trait TWxUpdateUIEvent : TWxEvent {
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
        unsafe { WxString { ptr: wxUpdateUIEvent_GetText(self.ptr()) }.to_str() }
    }
    fn setText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxUpdateUIEvent_SetText(self.ptr(), text.ptr()) }
    }
}

pub struct WxValidator { ptr: *mut c_void }
impl TWxValidator for WxValidator {}
impl TWxEvtHandler for WxValidator {}
impl TWxObject for WxValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxValidator {
    pub fn from(ptr: *mut c_void) -> WxValidator { WxValidator { ptr: ptr } }
    pub fn null() -> WxValidator { WxValidator::from(0 as *mut c_void) }
    
    pub fn new() -> WxValidator {
        unsafe { WxValidator { ptr: wxValidator_Create() } }
    }
    pub fn setBellOnError(doIt: c_int) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

pub trait TWxValidator : TWxEvtHandler {
    fn getWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxValidator_GetWindow(self.ptr()) } }
    }
    fn setWindow<T: TWxWindow>(&self, win: &T) {
        unsafe { wxValidator_SetWindow(self.ptr(), win.ptr()) }
    }
    fn transferFromWindow(&self) -> c_int {
        unsafe { wxValidator_TransferFromWindow(self.ptr()) }
    }
    fn transferToWindow(&self) -> c_int {
        unsafe { wxValidator_TransferToWindow(self.ptr()) }
    }
    fn validate<T: TWxWindow>(&self, parent: &T) -> c_int {
        unsafe { wxValidator_Validate(self.ptr(), parent.ptr()) }
    }
}

pub struct WxView { ptr: *mut c_void }
impl TWxView for WxView {}
impl TWxEvtHandler for WxView {}
impl TWxObject for WxView { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxView {
    pub fn from(ptr: *mut c_void) -> WxView { WxView { ptr: ptr } }
    pub fn null() -> WxView { WxView::from(0 as *mut c_void) }
    
}

pub trait TWxView : TWxEvtHandler {
}

pub struct WxSound { ptr: *mut c_void }
impl TWxSound for WxSound {}
impl TWxEvtHandler for WxSound {}
impl TWxObject for WxSound { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSound {
    pub fn from(ptr: *mut c_void) -> WxSound { WxSound { ptr: ptr } }
    pub fn null() -> WxSound { WxSound::from(0 as *mut c_void) }
    
    pub fn new(fileName: &str, isResource: c_int) -> WxSound {
        let fileName = wxT(fileName);
        unsafe { WxSound { ptr: wxSound_Create(fileName.ptr(), isResource) } }
    }
}

pub trait TWxSound : TWxEvtHandler {
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

pub struct WxWindow { ptr: *mut c_void }
impl TWxWindow for WxWindow {}
impl TWxEvtHandler for WxWindow {}
impl TWxObject for WxWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWindow {
    pub fn from(ptr: *mut c_void) -> WxWindow { WxWindow { ptr: ptr } }
    pub fn null() -> WxWindow { WxWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> WxWindow {
        unsafe { WxWindow { ptr: wxWindow_Create(_prt.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait TWxWindow : TWxEvtHandler {
    fn addChild<T: TWxWindow>(&self, child: &T) {
        unsafe { wxWindow_AddChild(self.ptr(), child.ptr()) }
    }
    fn addConstraintReference<T: TWxWindow>(&self, otherWin: &T) {
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
    fn clientToScreen(&self, x: c_int, y: c_int) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ClientToScreen(self.ptr(), x, y) } }
    }
    fn close(&self, _force: c_int) -> c_int {
        unsafe { wxWindow_Close(self.ptr(), _force) }
    }
    fn convertDialogToPixels(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ConvertDialogToPixels(self.ptr()) } }
    }
    fn convertPixelsToDialog(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ConvertPixelsToDialog(self.ptr()) } }
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
    fn findFocus(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxWindow_FindFocus(self.ptr()) } }
    }
    fn findWindow(&self, name: &str) -> WxWindow {
        let name = wxT(name);
        unsafe { WxWindow { ptr: wxWindow_FindWindow(self.ptr(), name.ptr()) } }
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
    fn getEffectiveMinSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxWindow_GetEffectiveMinSize(self.ptr()) } }
    }
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.ptr()) }
    }
    fn getBackgroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getBestSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxWindow_GetBestSize(self.ptr()) } }
    }
    fn getCaret(&self) -> WxCaret {
        unsafe { WxCaret { ptr: wxWindow_GetCaret(self.ptr()) } }
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
    fn getClientData(&self) -> WxClientData {
        unsafe { WxClientData { ptr: wxWindow_GetClientData(self.ptr()) } }
    }
    fn getClientSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxWindow_GetClientSize(self.ptr()) } }
    }
    fn getClientSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.ptr(), _w, _h) }
    }
    fn getConstraints(&self) -> WxLayoutConstraints {
        unsafe { WxLayoutConstraints { ptr: wxWindow_GetConstraints(self.ptr()) } }
    }
    fn getConstraintsInvolvedIn(&self) -> *mut c_void {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.ptr()) }
    }
    fn getCursor(&self) -> WxCursor {
        unsafe { WxCursor { ptr: wxWindow_GetCursor(self.ptr()) } }
    }
    fn getDropTarget(&self) -> WxDropTarget {
        unsafe { WxDropTarget { ptr: wxWindow_GetDropTarget(self.ptr()) } }
    }
    fn getEventHandler(&self) -> WxEvtHandler {
        unsafe { WxEvtHandler { ptr: wxWindow_GetEventHandler(self.ptr()) } }
    }
    fn getFont<T: TWxFont>(&self, _ref: &T) {
        unsafe { wxWindow_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getForegroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetForegroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getHandle(&self) -> *mut c_void {
        unsafe { wxWindow_GetHandle(self.ptr()) }
    }
    fn getId(&self) -> c_int {
        unsafe { wxWindow_GetId(self.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxWindow_GetLabel(self.ptr()) }.to_str() }
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
        unsafe { WxString { ptr: wxWindow_GetName(self.ptr()) }.to_str() }
    }
    fn getParent(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxWindow_GetParent(self.ptr()) } }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_GetPosition(self.ptr()) } }
    }
    fn getPositionConstraint(&self, _x: *mut c_int, _y: *mut c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.ptr(), _x, _y) }
    }
    fn getRect(&self) -> WxRect {
        unsafe { WxRect { ptr: wxWindow_GetRect(self.ptr()) } }
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
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxWindow_GetSize(self.ptr()) } }
    }
    fn getSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.ptr(), _w, _h) }
    }
    fn getSizer(&self) -> WxSizer {
        unsafe { WxSizer { ptr: wxWindow_GetSizer(self.ptr()) } }
    }
    fn getTextExtent<T: TWxFont>(&self, string: &str, x: *mut c_int, y: *mut c_int, descent: *mut c_int, externalLeading: *mut c_int, theFont: &T) {
        let string = wxT(string);
        unsafe { wxWindow_GetTextExtent(self.ptr(), string.ptr(), x, y, descent, externalLeading, theFont.ptr()) }
    }
    fn getToolTip(&self) -> ~str {
        unsafe { WxString { ptr: wxWindow_GetToolTip(self.ptr()) }.to_str() }
    }
    fn getUpdateRegion(&self) -> WxRegion {
        unsafe { WxRegion { ptr: wxWindow_GetUpdateRegion(self.ptr()) } }
    }
    fn getValidator(&self) -> WxValidator {
        unsafe { WxValidator { ptr: wxWindow_GetValidator(self.ptr()) } }
    }
    fn getVirtualSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxWindow_GetVirtualSize(self.ptr()) } }
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
    fn popupMenu<T: TWxMenu>(&self, menu: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.ptr(), menu.ptr(), x, y) }
    }
    fn prepareDC<T: TWxDC>(&self, dc: &T) {
        unsafe { wxWindow_PrepareDC(self.ptr(), dc.ptr()) }
    }
    fn pushEventHandler<T: TWxEvtHandler>(&self, handler: &T) {
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
    fn removeChild<T: TWxWindow>(&self, child: &T) {
        unsafe { wxWindow_RemoveChild(self.ptr(), child.ptr()) }
    }
    fn removeConstraintReference<T: TWxWindow>(&self, otherWin: &T) {
        unsafe { wxWindow_RemoveConstraintReference(self.ptr(), otherWin.ptr()) }
    }
    fn reparent<T: TWxWindow>(&self, _par: &T) -> c_int {
        unsafe { wxWindow_Reparent(self.ptr(), _par.ptr()) }
    }
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.ptr()) }
    }
    fn screenToClient(&self, x: c_int, y: c_int) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ScreenToClient(self.ptr(), x, y) } }
    }
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self.ptr(), dx, dy) }
    }
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self.ptr(), dx, dy, x, y, w, h) }
    }
    fn setAcceleratorTable<T: TWxAcceleratorTable>(&self, accel: &T) {
        unsafe { wxWindow_SetAcceleratorTable(self.ptr(), accel.ptr()) }
    }
    fn setAutoLayout(&self, autoLayout: c_int) {
        unsafe { wxWindow_SetAutoLayout(self.ptr(), autoLayout) }
    }
    fn setBackgroundColour<T: TWxColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setCaret<T: TWxCaret>(&self, caret: &T) {
        unsafe { wxWindow_SetCaret(self.ptr(), caret.ptr()) }
    }
    fn setClientData<T: TWxClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientData(self.ptr(), data.ptr()) }
    }
    fn setClientObject<T: TWxClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientObject(self.ptr(), data.ptr()) }
    }
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self.ptr(), width, height) }
    }
    fn setConstraintSizes(&self, recurse: c_int) {
        unsafe { wxWindow_SetConstraintSizes(self.ptr(), recurse) }
    }
    fn setConstraints<T: TWxLayoutConstraints>(&self, constraints: &T) {
        unsafe { wxWindow_SetConstraints(self.ptr(), constraints.ptr()) }
    }
    fn setCursor<T: TWxCursor>(&self, cursor: &T) -> c_int {
        unsafe { wxWindow_SetCursor(self.ptr(), cursor.ptr()) }
    }
    fn setDropTarget<T: TWxDropTarget>(&self, dropTarget: &T) {
        unsafe { wxWindow_SetDropTarget(self.ptr(), dropTarget.ptr()) }
    }
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self.ptr(), exStyle) }
    }
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self.ptr()) }
    }
    fn setFont<T: TWxFont>(&self, font: &T) -> c_int {
        unsafe { wxWindow_SetFont(self.ptr(), font.ptr()) }
    }
    fn setForegroundColour<T: TWxColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.ptr(), colour.ptr()) }
    }
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.ptr(), _id) }
    }
    fn setLabel(&self, _title: &str) {
        let _title = wxT(_title);
        unsafe { wxWindow_SetLabel(self.ptr(), _title.ptr()) }
    }
    fn setName(&self, _name: &str) {
        let _name = wxT(_name);
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
    fn setSizer<T: TWxSizer>(&self, sizer: &T) {
        unsafe { wxWindow_SetSizer(self.ptr(), sizer.ptr()) }
    }
    fn setToolTip(&self, tip: &str) {
        let tip = wxT(tip);
        unsafe { wxWindow_SetToolTip(self.ptr(), tip.ptr()) }
    }
    fn setValidator<T: TWxValidator>(&self, validator: &T) {
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
    fn convertDialogToPixelsEx(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ConvertDialogToPixelsEx(self.ptr()) } }
    }
    fn convertPixelsToDialogEx(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ConvertPixelsToDialogEx(self.ptr()) } }
    }
    fn screenToClient2(&self, x: c_int, y: c_int) -> WxPoint {
        unsafe { WxPoint { ptr: wxWindow_ScreenToClient2(self.ptr(), x, y) } }
    }
}

pub struct WxWindowCreateEvent { ptr: *mut c_void }
impl TWxWindowCreateEvent for WxWindowCreateEvent {}
impl TWxCommandEvent for WxWindowCreateEvent {}
impl TWxEvent for WxWindowCreateEvent {}
impl TWxObject for WxWindowCreateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWindowCreateEvent {
    pub fn from(ptr: *mut c_void) -> WxWindowCreateEvent { WxWindowCreateEvent { ptr: ptr } }
    pub fn null() -> WxWindowCreateEvent { WxWindowCreateEvent::from(0 as *mut c_void) }
    
}

pub trait TWxWindowCreateEvent : TWxCommandEvent {
    fn getWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxWindowCreateEvent_GetWindow(self.ptr()) } }
    }
}

pub struct WxWindowDC { ptr: *mut c_void }
impl TWxWindowDC for WxWindowDC {}
impl TWxDC for WxWindowDC {}
impl TWxObject for WxWindowDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWindowDC {
    pub fn from(ptr: *mut c_void) -> WxWindowDC { WxWindowDC { ptr: ptr } }
    pub fn null() -> WxWindowDC { WxWindowDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(win: &T) -> WxWindowDC {
        unsafe { WxWindowDC { ptr: wxWindowDC_Create(win.ptr()) } }
    }
}

pub trait TWxWindowDC : TWxDC {
}

pub struct WxWindowDestroyEvent { ptr: *mut c_void }
impl TWxWindowDestroyEvent for WxWindowDestroyEvent {}
impl TWxCommandEvent for WxWindowDestroyEvent {}
impl TWxEvent for WxWindowDestroyEvent {}
impl TWxObject for WxWindowDestroyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWindowDestroyEvent {
    pub fn from(ptr: *mut c_void) -> WxWindowDestroyEvent { WxWindowDestroyEvent { ptr: ptr } }
    pub fn null() -> WxWindowDestroyEvent { WxWindowDestroyEvent::from(0 as *mut c_void) }
    
}

pub trait TWxWindowDestroyEvent : TWxCommandEvent {
    fn getWindow(&self) -> WxWindow {
        unsafe { WxWindow { ptr: wxWindowDestroyEvent_GetWindow(self.ptr()) } }
    }
}

pub struct WxWindowDisabler { ptr: *mut c_void }
impl TWxWindowDisabler for WxWindowDisabler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWindowDisabler {
    pub fn from(ptr: *mut c_void) -> WxWindowDisabler { WxWindowDisabler { ptr: ptr } }
    pub fn null() -> WxWindowDisabler { WxWindowDisabler::from(0 as *mut c_void) }
    
}

pub trait TWxWindowDisabler {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxXmlResourceHandler { ptr: *mut c_void }
impl TWxXmlResourceHandler for WxXmlResourceHandler {}
impl TWxObject for WxXmlResourceHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxXmlResourceHandler {
    pub fn from(ptr: *mut c_void) -> WxXmlResourceHandler { WxXmlResourceHandler { ptr: ptr } }
    pub fn null() -> WxXmlResourceHandler { WxXmlResourceHandler::from(0 as *mut c_void) }
    
}

pub trait TWxXmlResourceHandler : TWxObject {
}

pub struct WxGenericDragImage { ptr: *mut c_void }
impl TWxGenericDragImage for WxGenericDragImage {}
impl TWxDragImage for WxGenericDragImage {}
impl TWxObject for WxGenericDragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGenericDragImage {
    pub fn from(ptr: *mut c_void) -> WxGenericDragImage { WxGenericDragImage { ptr: ptr } }
    pub fn null() -> WxGenericDragImage { WxGenericDragImage::from(0 as *mut c_void) }
    
    pub fn new<T: TWxCursor>(cursor: &T) -> WxGenericDragImage {
        unsafe { WxGenericDragImage { ptr: wxGenericDragImage_Create(cursor.ptr()) } }
    }
}

pub trait TWxGenericDragImage : TWxDragImage {
    fn doDrawImage<T: TWxDC>(&self, dc: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxGenericDragImage_DoDrawImage(self.ptr(), dc.ptr(), x, y) }
    }
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> WxRect {
        unsafe { WxRect { ptr: wxGenericDragImage_GetImageRect(self.ptr(), x_pos, y_pos) } }
    }
    fn updateBackingFromWindow<T: TWxDC, U: TWxMemoryDC>(&self, windowDC: &T, destDC: &U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.ptr(), windowDC.ptr(), destDC.ptr(), x, y, w, h, xdest, ydest, width, height) }
    }
}

pub struct WxGraphicsObject { ptr: *mut c_void }
impl TWxGraphicsObject for WxGraphicsObject {}
impl TWxObject for WxGraphicsObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsObject {
    pub fn from(ptr: *mut c_void) -> WxGraphicsObject { WxGraphicsObject { ptr: ptr } }
    pub fn null() -> WxGraphicsObject { WxGraphicsObject::from(0 as *mut c_void) }
    
    pub fn getRenderer() -> WxGraphicsRenderer {
        unsafe { WxGraphicsRenderer { ptr: wxGraphicsObject_GetRenderer() } }
    }
}

pub trait TWxGraphicsObject : TWxObject {
    fn isNull(&self) -> c_int {
        unsafe { wxGraphicsObject_IsNull(self.ptr()) }
    }
}

pub struct WxGraphicsBrush { ptr: *mut c_void }
impl TWxGraphicsBrush for WxGraphicsBrush {}
impl TWxGraphicsObject for WxGraphicsBrush {}
impl TWxObject for WxGraphicsBrush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsBrush {
    pub fn from(ptr: *mut c_void) -> WxGraphicsBrush { WxGraphicsBrush { ptr: ptr } }
    pub fn null() -> WxGraphicsBrush { WxGraphicsBrush::from(0 as *mut c_void) }
    
    pub fn new() -> WxGraphicsBrush {
        unsafe { WxGraphicsBrush { ptr: wxGraphicsBrush_Create() } }
    }
}

pub trait TWxGraphicsBrush : TWxGraphicsObject {
}

pub struct WxGraphicsContext { ptr: *mut c_void }
impl TWxGraphicsContext for WxGraphicsContext {}
impl TWxGraphicsObject for WxGraphicsContext {}
impl TWxObject for WxGraphicsContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsContext {
    pub fn from(ptr: *mut c_void) -> WxGraphicsContext { WxGraphicsContext { ptr: ptr } }
    pub fn null() -> WxGraphicsContext { WxGraphicsContext::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindowDC>(dc: &T) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsContext_Create(dc.ptr()) } }
    }
    pub fn newFromWindow<T: TWxWindow>(window: &T) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsContext_CreateFromWindow(window.ptr()) } }
    }
    pub fn newFromNative(context: *mut c_void) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsContext_CreateFromNative(context) } }
    }
    pub fn newFromNativeWindow(window: *mut c_void) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsContext_CreateFromNativeWindow(window) } }
    }
}

pub trait TWxGraphicsContext : TWxGraphicsObject {
    fn clip<T: TWxRegion>(&self, region: &T) {
        unsafe { wxGraphicsContext_Clip(self.ptr(), region.ptr()) }
    }
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self.ptr(), x, y, w, h) }
    }
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self.ptr()) }
    }
    fn drawBitmap<T: TWxBitmap>(&self, bmp: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.ptr(), bmp.ptr(), x, y, w, h) }
    }
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.ptr(), x, y, w, h) }
    }
    fn drawIcon<T: TWxIcon>(&self, icon: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.ptr(), icon.ptr(), x, y, w, h) }
    }
    fn drawLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.ptr(), n, x, y, style) }
    }
    fn drawPath<T: TWxGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_DrawPath(self.ptr(), path.ptr(), style) }
    }
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawRectangle(self.ptr(), x, y, w, h) }
    }
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawRoundedRectangle(self.ptr(), x, y, w, h, radius) }
    }
    fn drawText(&self, text: &str, x: c_double, y: c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_DrawText(self.ptr(), text.ptr(), x, y) }
    }
    fn drawTextWithAngle(&self, text: &str, x: c_double, y: c_double, radius: c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.ptr(), text.ptr(), x, y, radius) }
    }
    fn fillPath<T: TWxGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.ptr(), path.ptr(), style) }
    }
    fn strokePath<T: TWxGraphicsPath>(&self, path: &T) {
        unsafe { wxGraphicsContext_StrokePath(self.ptr(), path.ptr()) }
    }
    fn getNativeContext(&self) -> *mut c_void {
        unsafe { wxGraphicsContext_GetNativeContext(self.ptr()) }
    }
    fn getTextExtent(&self, text: &str, width: *mut c_double, height: *mut c_double, descent: *mut c_double, externalLeading: *mut c_double) {
        let text = wxT(text);
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
    fn setTransform<T: TWxGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_SetTransform(self.ptr(), path.ptr()) }
    }
    fn concatTransform<T: TWxGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_ConcatTransform(self.ptr(), path.ptr()) }
    }
    fn setBrush<T: TWxBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetBrush(self.ptr(), brush.ptr()) }
    }
    fn setGraphicsBrush<T: TWxGraphicsBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.ptr(), brush.ptr()) }
    }
    fn setFont<T: TWxFont, U: TWxColour>(&self, font: &T, colour: &U) {
        unsafe { wxGraphicsContext_SetFont(self.ptr(), font.ptr(), colour.ptr()) }
    }
    fn setGraphicsFont<T: TWxGraphicsFont>(&self, font: &T) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.ptr(), font.ptr()) }
    }
    fn setPen<T: TWxPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetPen(self.ptr(), pen.ptr()) }
    }
    fn setGraphicsPen<T: TWxGraphicsPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self.ptr(), pen.ptr()) }
    }
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self.ptr(), x1, y1, x2, y2) }
    }
    fn strokeLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.ptr(), n, x, y, style) }
    }
}

pub struct WxGraphicsFont { ptr: *mut c_void }
impl TWxGraphicsFont for WxGraphicsFont {}
impl TWxGraphicsObject for WxGraphicsFont {}
impl TWxObject for WxGraphicsFont { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsFont {
    pub fn from(ptr: *mut c_void) -> WxGraphicsFont { WxGraphicsFont { ptr: ptr } }
    pub fn null() -> WxGraphicsFont { WxGraphicsFont::from(0 as *mut c_void) }
    
    pub fn new() -> WxGraphicsFont {
        unsafe { WxGraphicsFont { ptr: wxGraphicsFont_Create() } }
    }
}

pub trait TWxGraphicsFont : TWxGraphicsObject {
}

pub struct WxGraphicsMatrix { ptr: *mut c_void }
impl TWxGraphicsMatrix for WxGraphicsMatrix {}
impl TWxGraphicsObject for WxGraphicsMatrix {}
impl TWxObject for WxGraphicsMatrix { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsMatrix {
    pub fn from(ptr: *mut c_void) -> WxGraphicsMatrix { WxGraphicsMatrix { ptr: ptr } }
    pub fn null() -> WxGraphicsMatrix { WxGraphicsMatrix::from(0 as *mut c_void) }
    
    pub fn new() -> WxGraphicsMatrix {
        unsafe { WxGraphicsMatrix { ptr: wxGraphicsMatrix_Create() } }
    }
}

pub trait TWxGraphicsMatrix : TWxGraphicsObject {
    fn concat<T: TWxGraphicsMatrix>(&self, t: &T) {
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
    fn isEqual<T: TWxGraphicsMatrix>(&self, t: &T) -> c_int {
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

pub struct WxGraphicsPath { ptr: *mut c_void }
impl TWxGraphicsPath for WxGraphicsPath {}
impl TWxGraphicsObject for WxGraphicsPath {}
impl TWxObject for WxGraphicsPath { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsPath {
    pub fn from(ptr: *mut c_void) -> WxGraphicsPath { WxGraphicsPath { ptr: ptr } }
    pub fn null() -> WxGraphicsPath { WxGraphicsPath::from(0 as *mut c_void) }
    
    pub fn new() -> WxGraphicsPath {
        unsafe { WxGraphicsPath { ptr: wxGraphicsPath_Create() } }
    }
    pub fn unGetNativePath(p: *mut c_void) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

pub trait TWxGraphicsPath : TWxGraphicsObject {
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
    fn addPath<T: TWxGraphicsPath>(&self, x: c_double, y: c_double, path: &T) {
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
    fn transform<T: TWxGraphicsMatrix>(&self, matrix: &T) {
        unsafe { wxGraphicsPath_Transform(self.ptr(), matrix.ptr()) }
    }
    fn getNativePath(&self) -> *mut c_void {
        unsafe { wxGraphicsPath_GetNativePath(self.ptr()) }
    }
}

pub struct WxGraphicsPen { ptr: *mut c_void }
impl TWxGraphicsPen for WxGraphicsPen {}
impl TWxGraphicsObject for WxGraphicsPen {}
impl TWxObject for WxGraphicsPen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsPen {
    pub fn from(ptr: *mut c_void) -> WxGraphicsPen { WxGraphicsPen { ptr: ptr } }
    pub fn null() -> WxGraphicsPen { WxGraphicsPen::from(0 as *mut c_void) }
    
    pub fn new() -> WxGraphicsPen {
        unsafe { WxGraphicsPen { ptr: wxGraphicsPen_Create() } }
    }
}

pub trait TWxGraphicsPen : TWxGraphicsObject {
}

pub struct WxGraphicsRenderer { ptr: *mut c_void }
impl TWxGraphicsRenderer for WxGraphicsRenderer {}
impl TWxGraphicsObject for WxGraphicsRenderer {}
impl TWxObject for WxGraphicsRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGraphicsRenderer {
    pub fn from(ptr: *mut c_void) -> WxGraphicsRenderer { WxGraphicsRenderer { ptr: ptr } }
    pub fn null() -> WxGraphicsRenderer { WxGraphicsRenderer::from(0 as *mut c_void) }
    
    pub fn newContext<T: TWxWindowDC>(dc: &T) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsRenderer_CreateContext(dc.ptr()) } }
    }
    pub fn newContextFromWindow<T: TWxWindow>(window: &T) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromWindow(window.ptr()) } }
    }
    pub fn newContextFromNativeContext(context: *mut c_void) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromNativeContext(context) } }
    }
    pub fn newContextFromNativeWindow(window: *mut c_void) -> WxGraphicsContext {
        unsafe { WxGraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromNativeWindow(window) } }
    }
}

pub trait TWxGraphicsRenderer : TWxGraphicsObject {
    fn getDefaultRenderer(&self) -> WxGraphicsRenderer {
        unsafe { WxGraphicsRenderer { ptr: wxGraphicsRenderer_GetDefaultRenderer(self.ptr()) } }
    }
}

pub struct WxcPrintout { ptr: *mut c_void }
impl TWxcPrintout for WxcPrintout {}
impl TWxPrintout for WxcPrintout {}
impl TWxObject for WxcPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxcPrintout {
    pub fn from(ptr: *mut c_void) -> WxcPrintout { WxcPrintout { ptr: ptr } }
    pub fn null() -> WxcPrintout { WxcPrintout::from(0 as *mut c_void) }
    
    pub fn new(title: &str) -> WxcPrintout {
        let title = wxT(title);
        unsafe { WxcPrintout { ptr: wxcPrintout_Create(title.ptr()) } }
    }
}

pub trait TWxcPrintout : TWxPrintout {
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.ptr(), startPage, endPage, fromPage, toPage) }
    }
    fn getEvtHandler(&self) -> WxcPrintoutHandler {
        unsafe { WxcPrintoutHandler { ptr: wxcPrintout_GetEvtHandler(self.ptr()) } }
    }
}

pub struct WxcPrintEvent { ptr: *mut c_void }
impl TWxcPrintEvent for WxcPrintEvent {}
impl TWxEvent for WxcPrintEvent {}
impl TWxObject for WxcPrintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxcPrintEvent {
    pub fn from(ptr: *mut c_void) -> WxcPrintEvent { WxcPrintEvent { ptr: ptr } }
    pub fn null() -> WxcPrintEvent { WxcPrintEvent::from(0 as *mut c_void) }
    
}

pub trait TWxcPrintEvent : TWxEvent {
    fn getPrintout(&self) -> WxcPrintout {
        unsafe { WxcPrintout { ptr: wxcPrintEvent_GetPrintout(self.ptr()) } }
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

pub struct WxcPrintoutHandler { ptr: *mut c_void }
impl TWxcPrintoutHandler for WxcPrintoutHandler {}
impl TWxEvtHandler for WxcPrintoutHandler {}
impl TWxObject for WxcPrintoutHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxcPrintoutHandler {
    pub fn from(ptr: *mut c_void) -> WxcPrintoutHandler { WxcPrintoutHandler { ptr: ptr } }
    pub fn null() -> WxcPrintoutHandler { WxcPrintoutHandler::from(0 as *mut c_void) }
    
}

pub trait TWxcPrintoutHandler : TWxEvtHandler {
}

pub struct WxcTreeItemData { ptr: *mut c_void }
impl TWxcTreeItemData for WxcTreeItemData {}
impl TWxTreeItemData for WxcTreeItemData {}
impl TWxClientData for WxcTreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxcTreeItemData {
    pub fn from(ptr: *mut c_void) -> WxcTreeItemData { WxcTreeItemData { ptr: ptr } }
    pub fn null() -> WxcTreeItemData { WxcTreeItemData::from(0 as *mut c_void) }
    
    pub fn new<T: TWxClosure>(closure: &T) -> WxcTreeItemData {
        unsafe { WxcTreeItemData { ptr: wxcTreeItemData_Create(closure.ptr()) } }
    }
}

pub trait TWxcTreeItemData : TWxTreeItemData {
    fn getClientClosure(&self) -> WxClosure {
        unsafe { WxClosure { ptr: wxcTreeItemData_GetClientClosure(self.ptr()) } }
    }
    fn setClientClosure<T: TWxClosure>(&self, closure: &T) {
        unsafe { wxcTreeItemData_SetClientClosure(self.ptr(), closure.ptr()) }
    }
}

pub struct WxInputSink { ptr: *mut c_void }
impl TWxInputSink for WxInputSink {}
impl TWxThread for WxInputSink { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxInputSink {
    pub fn from(ptr: *mut c_void) -> WxInputSink { WxInputSink { ptr: ptr } }
    pub fn null() -> WxInputSink { WxInputSink::from(0 as *mut c_void) }
    
    pub fn new<T: TWxInputStream, U: TWxEvtHandler>(input: &T, evtHandler: &U, bufferLen: c_int) -> WxInputSink {
        unsafe { WxInputSink { ptr: wxInputSink_Create(input.ptr(), evtHandler.ptr(), bufferLen) } }
    }
}

pub trait TWxInputSink : TWxThread {
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.ptr()) }
    }
    fn start(&self) {
        unsafe { wxInputSink_Start(self.ptr()) }
    }
}

pub struct WxInputSinkEvent { ptr: *mut c_void }
impl TWxInputSinkEvent for WxInputSinkEvent {}
impl TWxEvent for WxInputSinkEvent {}
impl TWxObject for WxInputSinkEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxInputSinkEvent {
    pub fn from(ptr: *mut c_void) -> WxInputSinkEvent { WxInputSinkEvent { ptr: ptr } }
    pub fn null() -> WxInputSinkEvent { WxInputSinkEvent::from(0 as *mut c_void) }
    
}

pub trait TWxInputSinkEvent : TWxEvent {
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

