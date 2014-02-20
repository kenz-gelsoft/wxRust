use std::libc::*;
use _unsafe::*;
use base::*;

pub struct ELJApp { ptr: *mut c_void }
impl _ELJApp for ELJApp {}
impl _wxApp for ELJApp {}
impl _wxEvtHandler for ELJApp {}
impl _wxObject for ELJApp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJApp {
    pub fn from(ptr: *mut c_void) -> ELJApp { ELJApp { ptr: ptr } }
    pub fn null() -> ELJApp { ELJApp::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newLogTarget() -> ELJLog {
        unsafe { ELJLog { ptr: ELJApp_CreateLogTarget() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn displaySize() -> wxSize {
        unsafe { wxSize { ptr: ELJApp_DisplaySize() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn enableTooltips(_enable: c_int) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn enableTopLevelWindows(_enb: c_int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn executeProcess<T: _wxProcess>(_cmd: &str, _snc: c_int, _prc: &T) -> c_int {
        let _cmd = wxT(_cmd);
        unsafe { ELJApp_ExecuteProcess(_cmd.ptr(), _snc, _prc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn exit() {
        unsafe { ELJApp_Exit() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn exitMainLoop() {
        unsafe { ELJApp_ExitMainLoop() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findWindowById<T: _wxWindow>(_id: c_int, _prt: &T) -> *mut c_void {
        unsafe { ELJApp_FindWindowById(_id, _prt.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findWindowByLabel<T: _wxWindow>(_lbl: &str, _prt: &T) -> wxWindow {
        let _lbl = wxT(_lbl);
        unsafe { wxWindow { ptr: ELJApp_FindWindowByLabel(_lbl.ptr(), _prt.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findWindowByName<T: _wxWindow>(_lbl: &str, _prt: &T) -> wxWindow {
        let _lbl = wxT(_lbl);
        unsafe { wxWindow { ptr: ELJApp_FindWindowByName(_lbl.ptr(), _prt.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getApp() -> wxApp {
        unsafe { wxApp { ptr: ELJApp_GetApp() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getAppName() -> ~str {
        unsafe { wxString { ptr: ELJApp_GetAppName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getClassName() -> ~str {
        unsafe { wxString { ptr: ELJApp_GetClassName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getOsDescription() -> ~str {
        unsafe { wxString { ptr: ELJApp_GetOsDescription() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getOsVersion(_maj: *mut c_void, _min: *mut c_void) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getTopWindow() -> wxWindow {
        unsafe { wxWindow { ptr: ELJApp_GetTopWindow() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUserHome(_usr: *mut c_void) -> ~str {
        unsafe { wxString { ptr: ELJApp_GetUserHome(_usr) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUserId() -> ~str {
        unsafe { wxString { ptr: ELJApp_GetUserId() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUserName() -> ~str {
        unsafe { wxString { ptr: ELJApp_GetUserName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getVendorName() -> ~str {
        unsafe { wxString { ptr: ELJApp_GetVendorName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn initAllImageHandlers() {
        unsafe { ELJApp_InitAllImageHandlers() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn initialized() -> c_int {
        unsafe { ELJApp_Initialized() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn mainLoop() -> c_int {
        unsafe { ELJApp_MainLoop() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn mousePosition() -> wxPoint {
        unsafe { wxPoint { ptr: ELJApp_MousePosition() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn safeYield<T: _wxWindow>(_win: &T) -> c_int {
        unsafe { ELJApp_SafeYield(_win.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setAppName(name: &str) {
        let name = wxT(name);
        unsafe { ELJApp_SetAppName(name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setClassName(name: &str) {
        let name = wxT(name);
        unsafe { ELJApp_SetClassName(name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setExitOnFrameDelete(flag: c_int) {
        unsafe { ELJApp_SetExitOnFrameDelete(flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setPrintMode(mode: c_int) {
        unsafe { ELJApp_SetPrintMode(mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setTooltipDelay(_ms: c_int) {
        unsafe { ELJApp_SetTooltipDelay(_ms) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setTopWindow<T: _wxWindow>(_wnd: &T) {
        unsafe { ELJApp_SetTopWindow(_wnd.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setVendorName(name: &str) {
        let name = wxT(name);
        unsafe { ELJApp_SetVendorName(name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn sleep(_scs: c_int) {
        unsafe { ELJApp_Sleep(_scs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn milliSleep(_mscs: c_int) {
        unsafe { ELJApp_MilliSleep(_mscs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn yield_() -> c_int {
        unsafe { ELJApp_Yield() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isTerminating() -> c_int {
        unsafe { ELJApp_IsTerminating() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn initializeC<T: _wxClosure>(closure: &T, _argc: c_int, _argv: *mut *mut c_char) {
        unsafe { ELJApp_InitializeC(closure.ptr(), _argc, _argv) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getIdleInterval() -> c_int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setIdleInterval(interval: c_int) {
        unsafe { ELJApp_SetIdleInterval(interval) }
    }
}

pub trait _ELJApp : _wxApp {
}

pub struct ELJArtProv { ptr: *mut c_void }
impl _ELJArtProv for ELJArtProv {}
impl _wxArtProvider for ELJArtProv {}
impl _wxObject for ELJArtProv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJArtProv {
    pub fn from(ptr: *mut c_void) -> ELJArtProv { ELJArtProv { ptr: ptr } }
    pub fn null() -> ELJArtProv { ELJArtProv::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _clb: *mut c_void) -> ELJArtProv {
        unsafe { ELJArtProv { ptr: ELJArtProv_Create(_obj, _clb) } }
    }
}

pub trait _ELJArtProv : _wxArtProvider {
    #[fixed_stack_segment]
    #[inline(never)]
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.ptr()) }
    }
}

pub struct ELJCommand { ptr: *mut c_void }
impl _ELJCommand for ELJCommand {}
impl _wxCommand for ELJCommand {}
impl _wxObject for ELJCommand { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJCommand {
    pub fn from(ptr: *mut c_void) -> ELJCommand { ELJCommand { ptr: ptr } }
    pub fn null() -> ELJCommand { ELJCommand::from(0 as *mut c_void) }
    
}

pub trait _ELJCommand : _wxCommand {
}

pub struct ELJDragDataObject { ptr: *mut c_void }
impl _ELJDragDataObject for ELJDragDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJDragDataObject {
    pub fn from(ptr: *mut c_void) -> ELJDragDataObject { ELJDragDataObject { ptr: ptr } }
    pub fn null() -> ELJDragDataObject { ELJDragDataObject::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fmt: &str, _func1: *mut c_void, _func2: *mut c_void, _func3: *mut c_void) -> ELJDragDataObject {
        let _fmt = wxT(_fmt);
        unsafe { ELJDragDataObject { ptr: ELJDragDataObject_Create(_obj, _fmt.ptr(), _func1, _func2, _func3) } }
    }
}

pub trait _ELJDragDataObject {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.ptr()) }
    }
}

pub struct ELJDropTarget { ptr: *mut c_void }
impl _ELJDropTarget for ELJDropTarget {}
impl _wxDropTarget for ELJDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJDropTarget {
    pub fn from(ptr: *mut c_void) -> ELJDropTarget { ELJDropTarget { ptr: ptr } }
    pub fn null() -> ELJDropTarget { ELJDropTarget::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void) -> ELJDropTarget {
        unsafe { ELJDropTarget { ptr: ELJDropTarget_Create(_obj) } }
    }
}

pub trait _ELJDropTarget : _wxDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJDropTarget_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnData(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnDragOver(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnDrop(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnEnter(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJDropTarget_SetOnLeave(self.ptr(), _func) }
    }
}

pub struct ELJFileDropTarget { ptr: *mut c_void }
impl _ELJFileDropTarget for ELJFileDropTarget {}
impl _wxFileDropTarget for ELJFileDropTarget {}
impl _wxDropTarget for ELJFileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJFileDropTarget {
    pub fn from(ptr: *mut c_void) -> ELJFileDropTarget { ELJFileDropTarget { ptr: ptr } }
    pub fn null() -> ELJFileDropTarget { ELJFileDropTarget::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> ELJFileDropTarget {
        unsafe { ELJFileDropTarget { ptr: ELJFileDropTarget_Create(_obj, _func) } }
    }
}

pub trait _ELJFileDropTarget : _wxFileDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJFileDropTarget_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnData(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnDragOver(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnDrop(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnEnter(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJFileDropTarget_SetOnLeave(self.ptr(), _func) }
    }
}

pub struct ELJLog { ptr: *mut c_void }
impl _ELJLog for ELJLog {}
impl _wxLog for ELJLog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJLog {
    pub fn from(ptr: *mut c_void) -> ELJLog { ELJLog { ptr: ptr } }
    pub fn null() -> ELJLog { ELJLog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> ELJLog {
        unsafe { ELJLog { ptr: ELJLog_Create(_obj, _fnc) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getActiveTarget() -> *mut c_void {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

pub trait _ELJLog : _wxLog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableLogging(&self, doIt: c_int) -> c_int {
        unsafe { ELJLog_EnableLogging(self.ptr(), doIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> c_int {
        unsafe { ELJLog_IsEnabled(self.ptr()) }
    }
}

pub struct ELJPreviewControlBar { ptr: *mut c_void }
impl _ELJPreviewControlBar for ELJPreviewControlBar {}
impl _wxPreviewControlBar for ELJPreviewControlBar {}
impl _wxPanel for ELJPreviewControlBar {}
impl _wxWindow for ELJPreviewControlBar {}
impl _wxEvtHandler for ELJPreviewControlBar {}
impl _wxObject for ELJPreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJPreviewControlBar {
    pub fn from(ptr: *mut c_void) -> ELJPreviewControlBar { ELJPreviewControlBar { ptr: ptr } }
    pub fn null() -> ELJPreviewControlBar { ELJPreviewControlBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(preview: *mut c_void, buttons: c_int, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> ELJPreviewControlBar {
        unsafe { ELJPreviewControlBar { ptr: ELJPreviewControlBar_Create(preview, buttons, parent.ptr(), title, x, y, w, h, style) } }
    }
}

pub trait _ELJPreviewControlBar : _wxPreviewControlBar {
}

pub struct ELJPreviewFrame { ptr: *mut c_void }
impl _ELJPreviewFrame for ELJPreviewFrame {}
impl _wxPreviewFrame for ELJPreviewFrame {}
impl _wxFrame for ELJPreviewFrame {}
impl _wxTopLevelWindow for ELJPreviewFrame {}
impl _wxWindow for ELJPreviewFrame {}
impl _wxEvtHandler for ELJPreviewFrame {}
impl _wxObject for ELJPreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJPreviewFrame {
    pub fn from(ptr: *mut c_void) -> ELJPreviewFrame { ELJPreviewFrame { ptr: ptr } }
    pub fn null() -> ELJPreviewFrame { ELJPreviewFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_obj: *mut c_void, _init: *mut c_void, _create_canvas: *mut c_void, _create_toolbar: *mut c_void, preview: *mut c_void, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> ELJPreviewFrame {
        unsafe { ELJPreviewFrame { ptr: ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.ptr(), title, x, y, w, h, style) } }
    }
}

pub trait _ELJPreviewFrame : _wxPreviewFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControlBar(&self) -> *mut c_void {
        unsafe { ELJPreviewFrame_GetControlBar(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPreviewCanvas(&self) -> wxPreviewCanvas {
        unsafe { wxPreviewCanvas { ptr: ELJPreviewFrame_GetPreviewCanvas(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintPreview(&self) -> wxPrintPreview {
        unsafe { wxPrintPreview { ptr: ELJPreviewFrame_GetPrintPreview(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setControlBar(&self, obj: *mut c_void) {
        unsafe { ELJPreviewFrame_SetControlBar(self.ptr(), obj) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPreviewCanvas<T: _wxPreviewCanvas>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.ptr(), obj.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintPreview<T: _wxPrintPreview>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.ptr(), obj.ptr()) }
    }
}

pub struct ELJTextDropTarget { ptr: *mut c_void }
impl _ELJTextDropTarget for ELJTextDropTarget {}
impl _wxTextDropTarget for ELJTextDropTarget {}
impl _wxDropTarget for ELJTextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJTextDropTarget {
    pub fn from(ptr: *mut c_void) -> ELJTextDropTarget { ELJTextDropTarget { ptr: ptr } }
    pub fn null() -> ELJTextDropTarget { ELJTextDropTarget::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> ELJTextDropTarget {
        unsafe { ELJTextDropTarget { ptr: ELJTextDropTarget_Create(_obj, _func) } }
    }
}

pub trait _ELJTextDropTarget : _wxTextDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJTextDropTarget_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnData(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnDragOver(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnDrop(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnEnter(self.ptr(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *mut c_void) {
        unsafe { ELJTextDropTarget_SetOnLeave(self.ptr(), _func) }
    }
}

pub struct ELJTextValidator { ptr: *mut c_void }
impl _ELJTextValidator for ELJTextValidator {}
impl _wxTextValidator for ELJTextValidator {}
impl _wxValidator for ELJTextValidator {}
impl _wxEvtHandler for ELJTextValidator {}
impl _wxObject for ELJTextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ELJTextValidator {
    pub fn from(ptr: *mut c_void) -> ELJTextValidator { ELJTextValidator { ptr: ptr } }
    pub fn null() -> ELJTextValidator { ELJTextValidator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void, _txt: *mut c_void, _stl: c_int) -> ELJTextValidator {
        unsafe { ELJTextValidator { ptr: ELJTextValidator_Create(_obj, _fnc, _txt, _stl) } }
    }
}

pub trait _ELJTextValidator : _wxTextValidator {
}

pub struct wxAcceleratorEntry { ptr: *mut c_void }
impl _wxAcceleratorEntry for wxAcceleratorEntry { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxAcceleratorEntry {
    pub fn from(ptr: *mut c_void) -> wxAcceleratorEntry { wxAcceleratorEntry { ptr: ptr } }
    pub fn null() -> wxAcceleratorEntry { wxAcceleratorEntry::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> wxAcceleratorEntry {
        unsafe { wxAcceleratorEntry { ptr: wxAcceleratorEntry_Create(flags, keyCode, cmd) } }
    }
}

pub trait _wxAcceleratorEntry {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxAcceleratorEntry_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCommand(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetCommand(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetFlags(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKeyCode(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetKeyCode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, flags: c_int, keyCode: c_int, cmd: c_int) {
        unsafe { wxAcceleratorEntry_Set(self.ptr(), flags, keyCode, cmd) }
    }
}

pub struct wxAcceleratorTable { ptr: *mut c_void }
impl _wxAcceleratorTable for wxAcceleratorTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxAcceleratorTable {
    pub fn from(ptr: *mut c_void) -> wxAcceleratorTable { wxAcceleratorTable { ptr: ptr } }
    pub fn null() -> wxAcceleratorTable { wxAcceleratorTable::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(n: c_int, entries: *mut c_void) -> wxAcceleratorTable {
        unsafe { wxAcceleratorTable { ptr: wxAcceleratorTable_Create(n, entries) } }
    }
}

pub trait _wxAcceleratorTable {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.ptr()) }
    }
}

pub struct wxActivateEvent { ptr: *mut c_void }
impl _wxActivateEvent for wxActivateEvent {}
impl _wxEvent for wxActivateEvent {}
impl _wxObject for wxActivateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxActivateEvent {
    pub fn from(ptr: *mut c_void) -> wxActivateEvent { wxActivateEvent { ptr: ptr } }
    pub fn null() -> wxActivateEvent { wxActivateEvent::from(0 as *mut c_void) }
    
}

pub trait _wxActivateEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getActive(&self) -> c_int {
        unsafe { wxActivateEvent_GetActive(self.ptr()) }
    }
}

pub struct wxApp { ptr: *mut c_void }
impl _wxApp for wxApp {}
impl _wxEvtHandler for wxApp {}
impl _wxObject for wxApp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxApp {
    pub fn from(ptr: *mut c_void) -> wxApp { wxApp { ptr: ptr } }
    pub fn null() -> wxApp { wxApp::from(0 as *mut c_void) }
    
}

pub trait _wxApp : _wxEvtHandler {
}

pub struct wxArtProvider { ptr: *mut c_void }
impl _wxArtProvider for wxArtProvider {}
impl _wxObject for wxArtProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxArtProvider {
    pub fn from(ptr: *mut c_void) -> wxArtProvider { wxArtProvider { ptr: ptr } }
    pub fn null() -> wxArtProvider { wxArtProvider::from(0 as *mut c_void) }
    
}

pub trait _wxArtProvider : _wxObject {
}

pub struct wxAutoBufferedPaintDC { ptr: *mut c_void }
impl _wxAutoBufferedPaintDC for wxAutoBufferedPaintDC {}
impl _wxDC for wxAutoBufferedPaintDC {}
impl _wxObject for wxAutoBufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxAutoBufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> wxAutoBufferedPaintDC { wxAutoBufferedPaintDC { ptr: ptr } }
    pub fn null() -> wxAutoBufferedPaintDC { wxAutoBufferedPaintDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(window: &T) -> wxAutoBufferedPaintDC {
        unsafe { wxAutoBufferedPaintDC { ptr: wxAutoBufferedPaintDC_Create(window.ptr()) } }
    }
}

pub trait _wxAutoBufferedPaintDC : _wxDC {
}

pub struct wxAutomationObject { ptr: *mut c_void }
impl _wxAutomationObject for wxAutomationObject {}
impl _wxObject for wxAutomationObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxAutomationObject {
    pub fn from(ptr: *mut c_void) -> wxAutomationObject { wxAutomationObject { ptr: ptr } }
    pub fn null() -> wxAutomationObject { wxAutomationObject::from(0 as *mut c_void) }
    
}

pub trait _wxAutomationObject : _wxObject {
}

pub struct wxBitmap { ptr: *mut c_void }
impl _wxBitmap for wxBitmap {}
impl _wxGDIObject for wxBitmap {}
impl _wxObject for wxBitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBitmap {
    pub fn from(ptr: *mut c_void) -> wxBitmap { wxBitmap { ptr: ptr } }
    pub fn null() -> wxBitmap { wxBitmap::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn addHandler<T: _wxEvtHandler>(handler: &T) {
        unsafe { wxBitmap_AddHandler(handler.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_data: *mut c_void, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> wxBitmap {
        unsafe { wxBitmap { ptr: wxBitmap_Create(_data, _type, _width, _height, _depth) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxBitmap {
        unsafe { wxBitmap { ptr: wxBitmap_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> wxBitmap {
        unsafe { wxBitmap { ptr: wxBitmap_CreateEmpty(_width, _height, _depth) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newLoad(name: &str, type_: c_int) -> wxBitmap {
        let name = wxT(name);
        unsafe { wxBitmap { ptr: wxBitmap_CreateLoad(name.ptr(), type_) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findHandlerByName(name: &str) -> *mut c_void {
        let name = wxT(name);
        unsafe { wxBitmap_FindHandlerByName(name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findHandlerByType(type_: c_int) -> *mut c_void {
        unsafe { wxBitmap_FindHandlerByType(type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn initStandardHandlers() {
        unsafe { wxBitmap_InitStandardHandlers() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn insertHandler<T: _wxEvtHandler>(handler: &T) {
        unsafe { wxBitmap_InsertHandler(handler.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn removeHandler(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_RemoveHandler(name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromImage<T: _wxImage>(image: &T, depth: c_int) -> wxBitmap {
        unsafe { wxBitmap { ptr: wxBitmap_CreateFromImage(image.ptr(), depth) } }
    }
}

pub trait _wxBitmap : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newFromXPM(&self) -> wxBitmap {
        unsafe { wxBitmap { ptr: wxBitmap_CreateFromXPM(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findHandlerByExtension(&self, type_: c_int) -> *mut c_void {
        unsafe { wxBitmap_FindHandlerByExtension(self.ptr(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDepth(&self) -> c_int {
        unsafe { wxBitmap_GetDepth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxBitmap_GetHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMask(&self) -> wxMask {
        unsafe { wxMask { ptr: wxBitmap_GetMask(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSubBitmap<T: _wxBitmap>(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxBitmap_GetSubBitmap(self.ptr(), x, y, w, h, _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_LoadFile(self.ptr(), name.ptr(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxBitmap_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile<T: _wxPalette>(&self, name: &str, type_: c_int, cmap: &T) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_SaveFile(self.ptr(), name.ptr(), type_, cmap.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDepth(&self, d: c_int) {
        unsafe { wxBitmap_SetDepth(self.ptr(), d) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHeight(&self, h: c_int) {
        unsafe { wxBitmap_SetHeight(self.ptr(), h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMask<T: _wxMask>(&self, mask: &T) {
        unsafe { wxBitmap_SetMask(self.ptr(), mask.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, w: c_int) {
        unsafe { wxBitmap_SetWidth(self.ptr(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxBitmap_IsStatic(self.ptr()) }
    }
}

pub struct wxBitmapButton { ptr: *mut c_void }
impl _wxBitmapButton for wxBitmapButton {}
impl _wxButton for wxBitmapButton {}
impl _wxControl for wxBitmapButton {}
impl _wxWindow for wxBitmapButton {}
impl _wxEvtHandler for wxBitmapButton {}
impl _wxObject for wxBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBitmapButton {
    pub fn from(ptr: *mut c_void) -> wxBitmapButton { wxBitmapButton { ptr: ptr } }
    pub fn null() -> wxBitmapButton { wxBitmapButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxBitmapButton {
        unsafe { wxBitmapButton { ptr: wxBitmapButton_Create(_prt.ptr(), _id, _bmp.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxBitmapButton : _wxButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapDisabled<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapFocus<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapLabel<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapSelected<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapSelected(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginX(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginY(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapDisabled<T: _wxBitmap>(&self, disabled: &T) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.ptr(), disabled.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapFocus<T: _wxBitmap>(&self, focus: &T) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.ptr(), focus.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapLabel<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.ptr(), bitmap.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapSelected<T: _wxBitmap>(&self, sel: &T) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.ptr(), sel.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.ptr(), x, y) }
    }
}

pub struct wxBitmapToggleButton { ptr: *mut c_void }
impl _wxBitmapToggleButton for wxBitmapToggleButton {}
impl _wxToggleButton for wxBitmapToggleButton {}
impl _wxControl for wxBitmapToggleButton {}
impl _wxWindow for wxBitmapToggleButton {}
impl _wxEvtHandler for wxBitmapToggleButton {}
impl _wxObject for wxBitmapToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBitmapToggleButton {
    pub fn from(ptr: *mut c_void) -> wxBitmapToggleButton { wxBitmapToggleButton { ptr: ptr } }
    pub fn null() -> wxBitmapToggleButton { wxBitmapToggleButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(parent: &T, id: c_int, _bmp: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> wxBitmapToggleButton {
        unsafe { wxBitmapToggleButton { ptr: wxBitmapToggleButton_Create(parent.ptr(), id, _bmp.ptr(), x, y, w, h, style) } }
    }
}

pub trait _wxBitmapToggleButton : _wxToggleButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapLabel<T: _wxBitmap>(&self, _bmp: &T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.ptr(), _bmp.ptr()) }
    }
}

pub struct wxBitmapDataObject { ptr: *mut c_void }
impl _wxBitmapDataObject for wxBitmapDataObject {}
impl _wxDataObjectSimple for wxBitmapDataObject {}
impl _wxDataObject for wxBitmapDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBitmapDataObject {
    pub fn from(ptr: *mut c_void) -> wxBitmapDataObject { wxBitmapDataObject { ptr: ptr } }
    pub fn null() -> wxBitmapDataObject { wxBitmapDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxBitmapDataObject : _wxDataObjectSimple {
}

pub struct wxBitmapHandler { ptr: *mut c_void }
impl _wxBitmapHandler for wxBitmapHandler {}
impl _wxObject for wxBitmapHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBitmapHandler {
    pub fn from(ptr: *mut c_void) -> wxBitmapHandler { wxBitmapHandler { ptr: ptr } }
    pub fn null() -> wxBitmapHandler { wxBitmapHandler::from(0 as *mut c_void) }
    
}

pub trait _wxBitmapHandler : _wxObject {
}

pub struct wxBoxSizer { ptr: *mut c_void }
impl _wxBoxSizer for wxBoxSizer {}
impl _wxSizer for wxBoxSizer {}
impl _wxObject for wxBoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBoxSizer {
    pub fn from(ptr: *mut c_void) -> wxBoxSizer { wxBoxSizer { ptr: ptr } }
    pub fn null() -> wxBoxSizer { wxBoxSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(orient: c_int) -> wxBoxSizer {
        unsafe { wxBoxSizer { ptr: wxBoxSizer_Create(orient) } }
    }
}

pub trait _wxBoxSizer : _wxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.ptr()) }
    }
}

pub struct wxBrush { ptr: *mut c_void }
impl _wxBrush for wxBrush {}
impl _wxGDIObject for wxBrush {}
impl _wxObject for wxBrush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBrush {
    pub fn from(ptr: *mut c_void) -> wxBrush { wxBrush { ptr: ptr } }
    pub fn null() -> wxBrush { wxBrush::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxBrush {
        unsafe { wxBrush { ptr: wxBrush_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(bitmap: &T) -> wxBrush {
        unsafe { wxBrush { ptr: wxBrush_CreateFromBitmap(bitmap.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromColour<T: _wxColour>(col: &T, style: c_int) -> wxBrush {
        unsafe { wxBrush { ptr: wxBrush_CreateFromColour(col.ptr(), style) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> wxBrush {
        unsafe { wxBrush { ptr: wxBrush_CreateFromStock(id) } }
    }
}

pub trait _wxBrush : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxBrush_Assign(self.ptr(), brush.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxBrush_GetColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStipple<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBrush_GetStipple(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxBrush>(&self, brush: &T) -> c_int {
        unsafe { wxBrush_IsEqual(self.ptr(), brush.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxBrush_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxBrush_SetColour(self.ptr(), col.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxBrush_SetColourSingle(self.ptr(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStipple<T: _wxBitmap>(&self, stipple: &T) {
        unsafe { wxBrush_SetStipple(self.ptr(), stipple.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxBrush_SetStyle(self.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxBrush_IsStatic(self.ptr()) }
    }
}

pub struct wxBrushList { ptr: *mut c_void }
impl _wxBrushList for wxBrushList {}
impl _wxList for wxBrushList {}
impl _wxObject for wxBrushList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBrushList {
    pub fn from(ptr: *mut c_void) -> wxBrushList { wxBrushList { ptr: ptr } }
    pub fn null() -> wxBrushList { wxBrushList::from(0 as *mut c_void) }
    
}

pub trait _wxBrushList : _wxList {
}

pub struct wxBufferedDC { ptr: *mut c_void }
impl _wxBufferedDC for wxBufferedDC {}
impl _wxDC for wxBufferedDC {}
impl _wxObject for wxBufferedDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBufferedDC {
    pub fn from(ptr: *mut c_void) -> wxBufferedDC { wxBufferedDC { ptr: ptr } }
    pub fn null() -> wxBufferedDC { wxBufferedDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newByDCAndSize<T: _wxDC>(dc: &T, width: c_int, hight: c_int, style: c_int) -> wxBufferedDC {
        unsafe { wxBufferedDC { ptr: wxBufferedDC_CreateByDCAndSize(dc.ptr(), width, hight, style) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newByDCAndBitmap<T: _wxDC, U: _wxBitmap>(dc: &T, bitmap: &U, style: c_int) -> wxBufferedDC {
        unsafe { wxBufferedDC { ptr: wxBufferedDC_CreateByDCAndBitmap(dc.ptr(), bitmap.ptr(), style) } }
    }
}

pub trait _wxBufferedDC : _wxDC {
}

pub struct wxBufferedPaintDC { ptr: *mut c_void }
impl _wxBufferedPaintDC for wxBufferedPaintDC {}
impl _wxDC for wxBufferedPaintDC {}
impl _wxObject for wxBufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> wxBufferedPaintDC { wxBufferedPaintDC { ptr: ptr } }
    pub fn null() -> wxBufferedPaintDC { wxBufferedPaintDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(window: &T, style: c_int) -> wxBufferedPaintDC {
        unsafe { wxBufferedPaintDC { ptr: wxBufferedPaintDC_Create(window.ptr(), style) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithBitmap<T: _wxWindow, U: _wxBitmap>(window: &T, bitmap: &U, style: c_int) -> wxBufferedPaintDC {
        unsafe { wxBufferedPaintDC { ptr: wxBufferedPaintDC_CreateWithBitmap(window.ptr(), bitmap.ptr(), style) } }
    }
}

pub trait _wxBufferedPaintDC : _wxDC {
}

pub struct wxBusyCursor { ptr: *mut c_void }
impl _wxBusyCursor for wxBusyCursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBusyCursor {
    pub fn from(ptr: *mut c_void) -> wxBusyCursor { wxBusyCursor { ptr: ptr } }
    pub fn null() -> wxBusyCursor { wxBusyCursor::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxBusyCursor {
        unsafe { wxBusyCursor { ptr: wxBusyCursor_Create() } }
    }
}

pub trait _wxBusyCursor {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn newWithCursor(&self) -> *mut c_void {
        unsafe { wxBusyCursor_CreateWithCursor(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.ptr()) }
    }
}

pub struct wxBusyInfo { ptr: *mut c_void }
impl _wxBusyInfo for wxBusyInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxBusyInfo {
    pub fn from(ptr: *mut c_void) -> wxBusyInfo { wxBusyInfo { ptr: ptr } }
    pub fn null() -> wxBusyInfo { wxBusyInfo::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_txt: &str) -> wxBusyInfo {
        let _txt = wxT(_txt);
        unsafe { wxBusyInfo { ptr: wxBusyInfo_Create(_txt.ptr()) } }
    }
}

pub trait _wxBusyInfo {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.ptr()) }
    }
}

pub struct wxButton { ptr: *mut c_void }
impl _wxButton for wxButton {}
impl _wxControl for wxButton {}
impl _wxWindow for wxButton {}
impl _wxEvtHandler for wxButton {}
impl _wxObject for wxButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxButton {
    pub fn from(ptr: *mut c_void) -> wxButton { wxButton { ptr: ptr } }
    pub fn null() -> wxButton { wxButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxButton {
        let _txt = wxT(_txt);
        unsafe { wxButton { ptr: wxButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.ptr()) }
    }
}

pub struct wxCaret { ptr: *mut c_void }
impl _wxCaret for wxCaret { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCaret {
    pub fn from(ptr: *mut c_void) -> wxCaret { wxCaret { ptr: ptr } }
    pub fn null() -> wxCaret { wxCaret::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_wnd: &T, _wth: c_int, _hgt: c_int) -> wxCaret {
        unsafe { wxCaret { ptr: wxCaret_Create(_wnd.ptr(), _wth, _hgt) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getBlinkTime() -> c_int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setBlinkTime(milliseconds: c_int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
    }
}

pub trait _wxCaret {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxCaret_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxCaret_GetSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxCaret_GetWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) {
        unsafe { wxCaret_Hide(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxCaret_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible(&self) -> c_int {
        unsafe { wxCaret_IsVisible(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxCaret_Move(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSize(&self, width: c_int, height: c_int) {
        unsafe { wxCaret_SetSize(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) {
        unsafe { wxCaret_Show(self.ptr()) }
    }
}

pub struct wxCheckBox { ptr: *mut c_void }
impl _wxCheckBox for wxCheckBox {}
impl _wxControl for wxCheckBox {}
impl _wxWindow for wxCheckBox {}
impl _wxEvtHandler for wxCheckBox {}
impl _wxObject for wxCheckBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCheckBox {
    pub fn from(ptr: *mut c_void) -> wxCheckBox { wxCheckBox { ptr: ptr } }
    pub fn null() -> wxCheckBox { wxCheckBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxCheckBox {
        let _txt = wxT(_txt);
        unsafe { wxCheckBox { ptr: wxCheckBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxCheckBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxCheckBox_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self.ptr(), value) }
    }
}

pub struct wxCheckListBox { ptr: *mut c_void }
impl _wxCheckListBox for wxCheckListBox {}
impl _wxListBox for wxCheckListBox {}
impl _wxControl for wxCheckListBox {}
impl _wxWindow for wxCheckListBox {}
impl _wxEvtHandler for wxCheckListBox {}
impl _wxObject for wxCheckListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCheckListBox {
    pub fn from(ptr: *mut c_void) -> wxCheckListBox { wxCheckListBox { ptr: ptr } }
    pub fn null() -> wxCheckListBox { wxCheckListBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> wxCheckListBox {
        unsafe { wxCheckListBox { ptr: wxCheckListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait _wxCheckListBox : _wxListBox {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, item: c_int, check: c_int) {
        unsafe { wxCheckListBox_Check(self.ptr(), item, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, item: c_int) -> c_int {
        unsafe { wxCheckListBox_IsChecked(self.ptr(), item) }
    }
}

pub struct wxChoice { ptr: *mut c_void }
impl _wxChoice for wxChoice {}
impl _wxControl for wxChoice {}
impl _wxWindow for wxChoice {}
impl _wxEvtHandler for wxChoice {}
impl _wxObject for wxChoice { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxChoice {
    pub fn from(ptr: *mut c_void) -> wxChoice { wxChoice { ptr: ptr } }
    pub fn null() -> wxChoice { wxChoice::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> wxChoice {
        unsafe { wxChoice { ptr: wxChoice_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait _wxChoice : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxChoice_Append(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxChoice_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxChoice_FindString(self.ptr(), s.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxChoice_GetCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxChoice_GetSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, n: c_int) -> ~str {
        unsafe { wxString { ptr: wxChoice_GetString(self.ptr(), n) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.ptr(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setString(&self, n: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxChoice_SetString(self.ptr(), n, s.ptr()) }
    }
}

pub struct wxClientDC { ptr: *mut c_void }
impl _wxClientDC for wxClientDC {}
impl _wxWindowDC for wxClientDC {}
impl _wxDC for wxClientDC {}
impl _wxObject for wxClientDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClientDC {
    pub fn from(ptr: *mut c_void) -> wxClientDC { wxClientDC { ptr: ptr } }
    pub fn null() -> wxClientDC { wxClientDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> wxClientDC {
        unsafe { wxClientDC { ptr: wxClientDC_Create(win.ptr()) } }
    }
}

pub trait _wxClientDC : _wxWindowDC {
}

pub struct wxClipboard { ptr: *mut c_void }
impl _wxClipboard for wxClipboard {}
impl _wxObject for wxClipboard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxClipboard {
    pub fn from(ptr: *mut c_void) -> wxClipboard { wxClipboard { ptr: ptr } }
    pub fn null() -> wxClipboard { wxClipboard::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxClipboard {
        unsafe { wxClipboard { ptr: wxClipboard_Create() } }
    }
}

pub trait _wxClipboard : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addData<T: _wxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_AddData(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxClipboard_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self) {
        unsafe { wxClipboard_Close(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flush(&self) -> c_int {
        unsafe { wxClipboard_Flush(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData<T: _wxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_GetData(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOpened(&self) -> c_int {
        unsafe { wxClipboard_IsOpened(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSupported<T: _wxDataFormat>(&self, format: &T) -> c_int {
        unsafe { wxClipboard_IsSupported(self.ptr(), format.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn open(&self) -> c_int {
        unsafe { wxClipboard_Open(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData<T: _wxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_SetData(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn usePrimarySelection(&self, primary: c_int) {
        unsafe { wxClipboard_UsePrimarySelection(self.ptr(), primary) }
    }
}

pub struct wxCloseEvent { ptr: *mut c_void }
impl _wxCloseEvent for wxCloseEvent {}
impl _wxEvent for wxCloseEvent {}
impl _wxObject for wxCloseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCloseEvent {
    pub fn from(ptr: *mut c_void) -> wxCloseEvent { wxCloseEvent { ptr: ptr } }
    pub fn null() -> wxCloseEvent { wxCloseEvent::from(0 as *mut c_void) }
    
}

pub trait _wxCloseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn canVeto(&self) -> c_int {
        unsafe { wxCloseEvent_CanVeto(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLoggingOff(&self) -> c_int {
        unsafe { wxCloseEvent_GetLoggingOff(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVeto(&self) -> c_int {
        unsafe { wxCloseEvent_GetVeto(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCanVeto(&self, canVeto: c_int) {
        unsafe { wxCloseEvent_SetCanVeto(self.ptr(), canVeto) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLoggingOff(&self, logOff: c_int) {
        unsafe { wxCloseEvent_SetLoggingOff(self.ptr(), logOff) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn veto(&self, veto: c_int) {
        unsafe { wxCloseEvent_Veto(self.ptr(), veto) }
    }
}

pub struct wxColour { ptr: *mut c_void }
impl _wxColour for wxColour {}
impl _wxObject for wxColour { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxColour {
    pub fn from(ptr: *mut c_void) -> wxColour { wxColour { ptr: ptr } }
    pub fn null() -> wxColour { wxColour::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newByName(_name: &str) -> wxColour {
        let _name = wxT(_name);
        unsafe { wxColour { ptr: wxColour_CreateByName(_name.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEmpty() -> wxColour {
        unsafe { wxColour { ptr: wxColour_CreateEmpty() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> wxColour {
        unsafe { wxColour { ptr: wxColour_CreateFromStock(id) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> wxColour {
        unsafe { wxColour { ptr: wxColour_CreateRGB(_red, _green, _blue, _alpha) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn validName(_name: *mut c_void) -> c_int {
        unsafe { wxColour_ValidName(_name) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromInt(rgb: c_int) -> wxColour {
        unsafe { wxColour { ptr: wxColour_CreateFromInt(rgb) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromUnsignedInt(rgba: uint32_t) -> wxColour {
        unsafe { wxColour { ptr: wxColour_CreateFromUnsignedInt(rgba) } }
    }
}

pub trait _wxColour : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn alpha(&self) -> uint8_t {
        unsafe { wxColour_Alpha(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign(&self, other: *mut c_void) {
        unsafe { wxColour_Assign(self.ptr(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn blue(&self) -> uint8_t {
        unsafe { wxColour_Blue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self, _other: *mut c_void) {
        unsafe { wxColour_Copy(self.ptr(), _other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn green(&self) -> uint8_t {
        unsafe { wxColour_Green(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxColour_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn red(&self) -> uint8_t {
        unsafe { wxColour_Red(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, _red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) {
        unsafe { wxColour_Set(self.ptr(), _red, _green, _blue, _alpha) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setByName(&self, _name: &str) {
        let _name = wxT(_name);
        unsafe { wxColour_SetByName(self.ptr(), _name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxColour_IsStatic(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInt(&self) -> c_int {
        unsafe { wxColour_GetInt(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUnsignedInt(&self) -> uint32_t {
        unsafe { wxColour_GetUnsignedInt(self.ptr()) }
    }
}

pub struct wxColourData { ptr: *mut c_void }
impl _wxColourData for wxColourData {}
impl _wxObject for wxColourData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxColourData {
    pub fn from(ptr: *mut c_void) -> wxColourData { wxColourData { ptr: ptr } }
    pub fn null() -> wxColourData { wxColourData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxColourData {
        unsafe { wxColourData { ptr: wxColourData_Create() } }
    }
}

pub trait _wxColourData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChooseFull(&self) -> c_int {
        unsafe { wxColourData_GetChooseFull(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxColourData_GetColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCustomColour<T: _wxColour>(&self, i: c_int, _ref: &T) {
        unsafe { wxColourData_GetCustomColour(self.ptr(), i, _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChooseFull(&self, flag: c_int) {
        unsafe { wxColourData_SetChooseFull(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxColourData_SetColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCustomColour<T: _wxColour>(&self, i: c_int, colour: &T) {
        unsafe { wxColourData_SetCustomColour(self.ptr(), i, colour.ptr()) }
    }
}

pub struct wxColourDatabase { ptr: *mut c_void }
impl _wxColourDatabase for wxColourDatabase {}
impl _wxList for wxColourDatabase {}
impl _wxObject for wxColourDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxColourDatabase {
    pub fn from(ptr: *mut c_void) -> wxColourDatabase { wxColourDatabase { ptr: ptr } }
    pub fn null() -> wxColourDatabase { wxColourDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxColourDatabase : _wxList {
}

pub struct wxColourDialog { ptr: *mut c_void }
impl _wxColourDialog for wxColourDialog {}
impl _wxDialog for wxColourDialog {}
impl _wxTopLevelWindow for wxColourDialog {}
impl _wxWindow for wxColourDialog {}
impl _wxEvtHandler for wxColourDialog {}
impl _wxObject for wxColourDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxColourDialog {
    pub fn from(ptr: *mut c_void) -> wxColourDialog { wxColourDialog { ptr: ptr } }
    pub fn null() -> wxColourDialog { wxColourDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxColourData>(_prt: &T, col: &U) -> wxColourDialog {
        unsafe { wxColourDialog { ptr: wxColourDialog_Create(_prt.ptr(), col.ptr()) } }
    }
}

pub trait _wxColourDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColourData<T: _wxColourData>(&self, _ref: &T) {
        unsafe { wxColourDialog_GetColourData(self.ptr(), _ref.ptr()) }
    }
}

pub struct wxComboBox { ptr: *mut c_void }
impl _wxComboBox for wxComboBox {}
impl _wxChoice for wxComboBox {}
impl _wxControl for wxComboBox {}
impl _wxWindow for wxComboBox {}
impl _wxEvtHandler for wxComboBox {}
impl _wxObject for wxComboBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxComboBox {
    pub fn from(ptr: *mut c_void) -> wxComboBox { wxComboBox { ptr: ptr } }
    pub fn null() -> wxComboBox { wxComboBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> wxComboBox {
        let _txt = wxT(_txt);
        unsafe { wxComboBox { ptr: wxComboBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait _wxComboBox : _wxChoice {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendData(&self, item: &str, d: *mut c_void) {
        let item = wxT(item);
        unsafe { wxComboBox_AppendData(self.ptr(), item.ptr(), d) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self) {
        unsafe { wxComboBox_Copy(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cut(&self) {
        unsafe { wxComboBox_Cut(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInsertionPoint(&self) -> c_int {
        unsafe { wxComboBox_GetInsertionPoint(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastPosition(&self) -> c_int {
        unsafe { wxComboBox_GetLastPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStringSelection(&self) -> ~str {
        unsafe { wxString { ptr: wxComboBox_GetStringSelection(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> ~str {
        unsafe { wxString { ptr: wxComboBox_GetValue(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste(&self) {
        unsafe { wxComboBox_Paste(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_Remove(self.ptr(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace(&self, from: c_int, to: c_int, value: &str) {
        let value = wxT(value);
        unsafe { wxComboBox_Replace(self.ptr(), from, to, value.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEditable(&self, editable: c_int) {
        unsafe { wxComboBox_SetEditable(self.ptr(), editable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPoint(&self, pos: c_int) {
        unsafe { wxComboBox_SetInsertionPoint(self.ptr(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPointEnd(&self) {
        unsafe { wxComboBox_SetInsertionPointEnd(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextSelection(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_SetTextSelection(self.ptr(), from, to) }
    }
}

pub struct wxCommand { ptr: *mut c_void }
impl _wxCommand for wxCommand {}
impl _wxObject for wxCommand { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCommand {
    pub fn from(ptr: *mut c_void) -> wxCommand { wxCommand { ptr: ptr } }
    pub fn null() -> wxCommand { wxCommand::from(0 as *mut c_void) }
    
}

pub trait _wxCommand : _wxObject {
}

pub struct wxCommandEvent { ptr: *mut c_void }
impl _wxCommandEvent for wxCommandEvent {}
impl _wxEvent for wxCommandEvent {}
impl _wxObject for wxCommandEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCommandEvent {
    pub fn from(ptr: *mut c_void) -> wxCommandEvent { wxCommandEvent { ptr: ptr } }
    pub fn null() -> wxCommandEvent { wxCommandEvent::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_typ: c_int, _id: c_int) -> wxCommandEvent {
        unsafe { wxCommandEvent { ptr: wxCommandEvent_Create(_typ, _id) } }
    }
}

pub trait _wxCommandEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientData(&self) -> wxClientData {
        unsafe { wxClientData { ptr: wxCommandEvent_GetClientData(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientObject(&self) -> wxClientData {
        unsafe { wxClientData { ptr: wxCommandEvent_GetClientObject(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExtraLong(&self) -> c_long {
        unsafe { wxCommandEvent_GetExtraLong(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInt(&self) -> c_long {
        unsafe { wxCommandEvent_GetInt(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxCommandEvent_GetSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self) -> ~str {
        unsafe { wxString { ptr: wxCommandEvent_GetString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self) -> c_int {
        unsafe { wxCommandEvent_IsChecked(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelection(&self) -> c_int {
        unsafe { wxCommandEvent_IsSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientData<T: _wxClientData>(&self, clientData: &T) {
        unsafe { wxCommandEvent_SetClientData(self.ptr(), clientData.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientObject<T: _wxClientData>(&self, clientObject: &T) {
        unsafe { wxCommandEvent_SetClientObject(self.ptr(), clientObject.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExtraLong(&self, extraLong: c_long) {
        unsafe { wxCommandEvent_SetExtraLong(self.ptr(), extraLong) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInt(&self, i: c_int) {
        unsafe { wxCommandEvent_SetInt(self.ptr(), i) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setString(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxCommandEvent_SetString(self.ptr(), s.ptr()) }
    }
}

pub struct wxCommandProcessor { ptr: *mut c_void }
impl _wxCommandProcessor for wxCommandProcessor {}
impl _wxObject for wxCommandProcessor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCommandProcessor {
    pub fn from(ptr: *mut c_void) -> wxCommandProcessor { wxCommandProcessor { ptr: ptr } }
    pub fn null() -> wxCommandProcessor { wxCommandProcessor::from(0 as *mut c_void) }
    
}

pub trait _wxCommandProcessor : _wxObject {
}

pub struct wxContextHelp { ptr: *mut c_void }
impl _wxContextHelp for wxContextHelp {}
impl _wxObject for wxContextHelp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxContextHelp {
    pub fn from(ptr: *mut c_void) -> wxContextHelp { wxContextHelp { ptr: ptr } }
    pub fn null() -> wxContextHelp { wxContextHelp::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T, beginHelp: c_int) -> wxContextHelp {
        unsafe { wxContextHelp { ptr: wxContextHelp_Create(win.ptr(), beginHelp) } }
    }
}

pub trait _wxContextHelp : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginContextHelp<T: _wxWindow>(&self, win: &T) -> c_int {
        unsafe { wxContextHelp_BeginContextHelp(self.ptr(), win.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endContextHelp(&self) -> c_int {
        unsafe { wxContextHelp_EndContextHelp(self.ptr()) }
    }
}

pub struct wxContextHelpButton { ptr: *mut c_void }
impl _wxContextHelpButton for wxContextHelpButton {}
impl _wxBitmapButton for wxContextHelpButton {}
impl _wxButton for wxContextHelpButton {}
impl _wxControl for wxContextHelpButton {}
impl _wxWindow for wxContextHelpButton {}
impl _wxEvtHandler for wxContextHelpButton {}
impl _wxObject for wxContextHelpButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxContextHelpButton {
    pub fn from(ptr: *mut c_void) -> wxContextHelpButton { wxContextHelpButton { ptr: ptr } }
    pub fn null() -> wxContextHelpButton { wxContextHelpButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> wxContextHelpButton {
        unsafe { wxContextHelpButton { ptr: wxContextHelpButton_Create(parent.ptr(), id, x, y, w, h, style) } }
    }
}

pub trait _wxContextHelpButton : _wxBitmapButton {
}

pub struct wxControl { ptr: *mut c_void }
impl _wxControl for wxControl {}
impl _wxWindow for wxControl {}
impl _wxEvtHandler for wxControl {}
impl _wxObject for wxControl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxControl {
    pub fn from(ptr: *mut c_void) -> wxControl { wxControl { ptr: ptr } }
    pub fn null() -> wxControl { wxControl::from(0 as *mut c_void) }
    
}

pub trait _wxControl : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn command<T: _wxEvent>(&self, event: &T) {
        unsafe { wxControl_Command(self.ptr(), event.ptr()) }
    }
}

pub struct wxCursor { ptr: *mut c_void }
impl _wxCursor for wxCursor {}
impl _wxBitmap for wxCursor {}
impl _wxGDIObject for wxCursor {}
impl _wxObject for wxCursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCursor {
    pub fn from(ptr: *mut c_void) -> wxCursor { wxCursor { ptr: ptr } }
    pub fn null() -> wxCursor { wxCursor::from(0 as *mut c_void) }
    
}

pub trait _wxCursor : _wxBitmap {
}

pub struct wxCustomDataObject { ptr: *mut c_void }
impl _wxCustomDataObject for wxCustomDataObject {}
impl _wxDataObjectSimple for wxCustomDataObject {}
impl _wxDataObject for wxCustomDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCustomDataObject {
    pub fn from(ptr: *mut c_void) -> wxCustomDataObject { wxCustomDataObject { ptr: ptr } }
    pub fn null() -> wxCustomDataObject { wxCustomDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxCustomDataObject : _wxDataObjectSimple {
}

pub struct wxDC { ptr: *mut c_void }
impl _wxDC for wxDC {}
impl _wxObject for wxDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDC {
    pub fn from(ptr: *mut c_void) -> wxDC { wxDC { ptr: ptr } }
    pub fn null() -> wxDC { wxDC::from(0 as *mut c_void) }
    
}

pub trait _wxDC : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn blit<T: _wxDC>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: &T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: c_int) -> c_int {
        unsafe { wxDC_Blit(self.ptr(), xdest, ydest, width, height, source.ptr(), xsrc, ysrc, rop, useMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcBoundingBox(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CalcBoundingBox(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canDrawBitmap(&self) -> c_int {
        unsafe { wxDC_CanDrawBitmap(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canGetTextExtent(&self) -> c_int {
        unsafe { wxDC_CanGetTextExtent(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxDC_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn computeScaleAndOrigin(&self) {
        unsafe { wxDC_ComputeScaleAndOrigin(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn crossHair(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CrossHair(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyClippingRegion(&self) {
        unsafe { wxDC_DestroyClippingRegion(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalX(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalX(self.ptr(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalXRel(self.ptr(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalY(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalY(self.ptr(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalYRel(self.ptr(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawArc(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, xc: c_int, yc: c_int) {
        unsafe { wxDC_DrawArc(self.ptr(), x1, y1, x2, y2, xc, yc) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBitmap<T: _wxBitmap>(&self, bmp: &T, x: c_int, y: c_int, useMask: c_int) {
        unsafe { wxDC_DrawBitmap(self.ptr(), bmp.ptr(), x, y, useMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCheckMark(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawCheckMark(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCircle(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { wxDC_DrawCircle(self.ptr(), x, y, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawEllipse(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawEllipse(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawEllipticArc(&self, x: c_int, y: c_int, w: c_int, h: c_int, sa: c_double, ea: c_double) {
        unsafe { wxDC_DrawEllipticArc(self.ptr(), x, y, w, h, sa, ea) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawIcon<T: _wxIcon>(&self, icon: &T, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.ptr(), icon.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLabel(&self, str: &str, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        let str = wxT(str);
        unsafe { wxDC_DrawLabel(self.ptr(), str.ptr(), x, y, w, h, align, indexAccel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLabelBitmap<T: _wxBitmap>(&self, str: &str, bmp: &T, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> wxRect {
        let str = wxT(str);
        unsafe { wxRect { ptr: wxDC_DrawLabelBitmap(self.ptr(), str.ptr(), bmp.ptr(), x, y, w, h, align, indexAccel) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLine(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { wxDC_DrawLine(self.ptr(), x1, y1, x2, y2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLines(&self, n: c_int, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { wxDC_DrawLines(self.ptr(), n, x, y, xoffset, yoffset) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPoint(&self, x: c_int, y: c_int) {
        unsafe { wxDC_DrawPoint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPolygon(&self, n: c_int, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolygon(self.ptr(), n, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPolyPolygon(&self, n: c_int, count: *mut c_void, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolyPolygon(self.ptr(), n, count, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawRectangle(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRotatedText(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        let text = wxT(text);
        unsafe { wxDC_DrawRotatedText(self.ptr(), text.ptr(), x, y, angle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.ptr(), x, y, width, height, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawText(&self, text: &str, x: c_int, y: c_int) {
        let text = wxT(text);
        unsafe { wxDC_DrawText(self.ptr(), text.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDoc(&self) {
        unsafe { wxDC_EndDoc(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endPage(&self) {
        unsafe { wxDC_EndPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn floodFill<T: _wxColour>(&self, x: c_int, y: c_int, col: &T, style: c_int) {
        unsafe { wxDC_FloodFill(self.ptr(), x, y, col.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackground<T: _wxBrush>(&self, _ref: &T) {
        unsafe { wxDC_GetBackground(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBrush<T: _wxBrush>(&self, _ref: &T) {
        unsafe { wxDC_GetBrush(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharHeight(&self) -> c_int {
        unsafe { wxDC_GetCharHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharWidth(&self) -> c_int {
        unsafe { wxDC_GetCharWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClippingBox(&self, _x: *mut c_void, _y: *mut c_void, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxDC_GetClippingBox(self.ptr(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDepth(&self) -> c_int {
        unsafe { wxDC_GetDepth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDeviceOrigin(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxDC_GetDeviceOrigin(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxDC_GetFont(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalFunction(&self) -> c_int {
        unsafe { wxDC_GetLogicalFunction(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalOrigin(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxDC_GetLogicalOrigin(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalScale(&self, _x: *mut c_double, _y: *mut c_double) {
        unsafe { wxDC_GetLogicalScale(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMapMode(&self) -> c_int {
        unsafe { wxDC_GetMapMode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPI(&self) -> wxSize {
        unsafe { wxSize { ptr: wxDC_GetPPI(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPen<T: _wxPen>(&self, _ref: &T) {
        unsafe { wxDC_GetPen(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPixel<T: _wxColour>(&self, x: c_int, y: c_int, col: &T) -> c_int {
        unsafe { wxDC_GetPixel(self.ptr(), x, y, col.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxDC_GetSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizeMM(&self) -> wxSize {
        unsafe { wxSize { ptr: wxDC_GetSizeMM(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextBackground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxDC_GetTextBackground(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent<T: _wxFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, externalLeading: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetTextExtent(self.ptr(), string.ptr(), w, h, descent, externalLeading, theFont.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMultiLineTextExtent<T: _wxFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, heightLine: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetMultiLineTextExtent(self.ptr(), string.ptr(), w, h, heightLine, theFont.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextForeground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxDC_GetTextForeground(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserScale(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxDC_GetUserScale(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceX(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceX(self.ptr(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceXRel(self.ptr(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceY(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceY(self.ptr(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceYRel(self.ptr(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maxX(&self) -> c_int {
        unsafe { wxDC_MaxX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maxY(&self) -> c_int {
        unsafe { wxDC_MaxY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn minX(&self) -> c_int {
        unsafe { wxDC_MinX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn minY(&self) -> c_int {
        unsafe { wxDC_MinY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxDC_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetBoundingBox(&self) {
        unsafe { wxDC_ResetBoundingBox(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAxisOrientation(&self, xLeftRight: c_int, yBottomUp: c_int) {
        unsafe { wxDC_SetAxisOrientation(self.ptr(), xLeftRight, yBottomUp) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackground<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBackground(self.ptr(), brush.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.ptr(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBrush<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBrush(self.ptr(), brush.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClippingRegionFromRegion<T: _wxRegion>(&self, region: &T) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDeviceClippingRegion<T: _wxRegion>(&self, region: &T) {
        unsafe { wxDC_SetDeviceClippingRegion(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxDC_SetFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLogicalFunction(&self, function: c_int) {
        unsafe { wxDC_SetLogicalFunction(self.ptr(), function) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLogicalOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetLogicalOrigin(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLogicalScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetLogicalScale(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMapMode(&self, mode: c_int) {
        unsafe { wxDC_SetMapMode(self.ptr(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPalette<T: _wxPalette>(&self, palette: &T) {
        unsafe { wxDC_SetPalette(self.ptr(), palette.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPen<T: _wxPen>(&self, pen: &T) {
        unsafe { wxDC_SetPen(self.ptr(), pen.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextBackground<T: _wxColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextBackground(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextForeground<T: _wxColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextForeground(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDoc(&self, msg: &str) -> c_int {
        let msg = wxT(msg);
        unsafe { wxDC_StartDoc(self.ptr(), msg.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startPage(&self) {
        unsafe { wxDC_StartPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserScaleX(&self) -> c_double {
        unsafe { wxDC_GetUserScaleX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserScaleY(&self) -> c_double {
        unsafe { wxDC_GetUserScaleY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPixel2<T: _wxColour>(&self, x: c_int, y: c_int, col: &T) {
        unsafe { wxDC_GetPixel2(self.ptr(), x, y, col.ptr()) }
    }
}

pub struct wxDCClipper { ptr: *mut c_void }
impl _wxDCClipper for wxDCClipper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDCClipper {
    pub fn from(ptr: *mut c_void) -> wxDCClipper { wxDCClipper { ptr: ptr } }
    pub fn null() -> wxDCClipper { wxDCClipper::from(0 as *mut c_void) }
    
}

pub trait _wxDCClipper {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDataFormat { ptr: *mut c_void }
impl _wxDataFormat for wxDataFormat { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDataFormat {
    pub fn from(ptr: *mut c_void) -> wxDataFormat { wxDataFormat { ptr: ptr } }
    pub fn null() -> wxDataFormat { wxDataFormat::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromId(name: &str) -> wxDataFormat {
        let name = wxT(name);
        unsafe { wxDataFormat { ptr: wxDataFormat_CreateFromId(name.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromType(typ: c_int) -> wxDataFormat {
        unsafe { wxDataFormat { ptr: wxDataFormat_CreateFromType(typ) } }
    }
}

pub trait _wxDataFormat {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> ~str {
        unsafe { wxString { ptr: wxDataFormat_GetId(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getType(&self) -> c_int {
        unsafe { wxDataFormat_GetType(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual(&self, other: *mut c_void) -> c_int {
        unsafe { wxDataFormat_IsEqual(self.ptr(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: *mut c_void) {
        unsafe { wxDataFormat_SetId(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setType(&self, typ: c_int) {
        unsafe { wxDataFormat_SetType(self.ptr(), typ) }
    }
}

pub struct wxDataObject { ptr: *mut c_void }
impl _wxDataObject for wxDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDataObject {
    pub fn from(ptr: *mut c_void) -> wxDataObject { wxDataObject { ptr: ptr } }
    pub fn null() -> wxDataObject { wxDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxDataObject {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDataObjectComposite { ptr: *mut c_void }
impl _wxDataObjectComposite for wxDataObjectComposite {}
impl _wxDataObject for wxDataObjectComposite { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDataObjectComposite {
    pub fn from(ptr: *mut c_void) -> wxDataObjectComposite { wxDataObjectComposite { ptr: ptr } }
    pub fn null() -> wxDataObjectComposite { wxDataObjectComposite::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxDataObjectComposite {
        unsafe { wxDataObjectComposite { ptr: wxDataObjectComposite_Create() } }
    }
}

pub trait _wxDataObjectComposite : _wxDataObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn add(&self, _dat: *mut c_void, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.ptr(), _dat, _preferred) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.ptr()) }
    }
}

pub struct wxDataObjectSimple { ptr: *mut c_void }
impl _wxDataObjectSimple for wxDataObjectSimple {}
impl _wxDataObject for wxDataObjectSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDataObjectSimple {
    pub fn from(ptr: *mut c_void) -> wxDataObjectSimple { wxDataObjectSimple { ptr: ptr } }
    pub fn null() -> wxDataObjectSimple { wxDataObjectSimple::from(0 as *mut c_void) }
    
}

pub trait _wxDataObjectSimple : _wxDataObject {
}

pub struct wxDialUpEvent { ptr: *mut c_void }
impl _wxDialUpEvent for wxDialUpEvent {}
impl _wxEvent for wxDialUpEvent {}
impl _wxObject for wxDialUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDialUpEvent {
    pub fn from(ptr: *mut c_void) -> wxDialUpEvent { wxDialUpEvent { ptr: ptr } }
    pub fn null() -> wxDialUpEvent { wxDialUpEvent::from(0 as *mut c_void) }
    
}

pub trait _wxDialUpEvent : _wxEvent {
}

pub struct wxDialUpManager { ptr: *mut c_void }
impl _wxDialUpManager for wxDialUpManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDialUpManager {
    pub fn from(ptr: *mut c_void) -> wxDialUpManager { wxDialUpManager { ptr: ptr } }
    pub fn null() -> wxDialUpManager { wxDialUpManager::from(0 as *mut c_void) }
    
}

pub trait _wxDialUpManager {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDialog { ptr: *mut c_void }
impl _wxDialog for wxDialog {}
impl _wxTopLevelWindow for wxDialog {}
impl _wxWindow for wxDialog {}
impl _wxEvtHandler for wxDialog {}
impl _wxObject for wxDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDialog {
    pub fn from(ptr: *mut c_void) -> wxDialog { wxDialog { ptr: ptr } }
    pub fn null() -> wxDialog { wxDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxDialog {
        let _txt = wxT(_txt);
        unsafe { wxDialog { ptr: wxDialog_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxDialog : _wxTopLevelWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn endModal(&self, retCode: c_int) {
        unsafe { wxDialog_EndModal(self.ptr(), retCode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReturnCode(&self) -> c_int {
        unsafe { wxDialog_GetReturnCode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isModal(&self) -> c_int {
        unsafe { wxDialog_IsModal(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReturnCode(&self, returnCode: c_int) {
        unsafe { wxDialog_SetReturnCode(self.ptr(), returnCode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showModal(&self) -> c_int {
        unsafe { wxDialog_ShowModal(self.ptr()) }
    }
}

pub struct wxDirDialog { ptr: *mut c_void }
impl _wxDirDialog for wxDirDialog {}
impl _wxDialog for wxDirDialog {}
impl _wxTopLevelWindow for wxDirDialog {}
impl _wxWindow for wxDirDialog {}
impl _wxEvtHandler for wxDirDialog {}
impl _wxObject for wxDirDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDirDialog {
    pub fn from(ptr: *mut c_void) -> wxDirDialog { wxDirDialog { ptr: ptr } }
    pub fn null() -> wxDirDialog { wxDirDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _dir: &str, _lft: c_int, _top: c_int, _stl: c_int) -> wxDirDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        unsafe { wxDirDialog { ptr: wxDirDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _lft, _top, _stl) } }
    }
}

pub trait _wxDirDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMessage(&self) -> ~str {
        unsafe { wxString { ptr: wxDirDialog_GetMessage(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPath(&self) -> ~str {
        unsafe { wxString { ptr: wxDirDialog_GetPath(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMessage(&self, msg: &str) {
        let msg = wxT(msg);
        unsafe { wxDirDialog_SetMessage(self.ptr(), msg.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPath(&self, pth: &str) {
        let pth = wxT(pth);
        unsafe { wxDirDialog_SetPath(self.ptr(), pth.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.ptr(), style) }
    }
}

pub struct wxDocChildFrame { ptr: *mut c_void }
impl _wxDocChildFrame for wxDocChildFrame {}
impl _wxFrame for wxDocChildFrame {}
impl _wxTopLevelWindow for wxDocChildFrame {}
impl _wxWindow for wxDocChildFrame {}
impl _wxEvtHandler for wxDocChildFrame {}
impl _wxObject for wxDocChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocChildFrame {
    pub fn from(ptr: *mut c_void) -> wxDocChildFrame { wxDocChildFrame { ptr: ptr } }
    pub fn null() -> wxDocChildFrame { wxDocChildFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocChildFrame : _wxFrame {
}

pub struct wxDocMDIChildFrame { ptr: *mut c_void }
impl _wxDocMDIChildFrame for wxDocMDIChildFrame {}
impl _wxMDIChildFrame for wxDocMDIChildFrame {}
impl _wxFrame for wxDocMDIChildFrame {}
impl _wxTopLevelWindow for wxDocMDIChildFrame {}
impl _wxWindow for wxDocMDIChildFrame {}
impl _wxEvtHandler for wxDocMDIChildFrame {}
impl _wxObject for wxDocMDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocMDIChildFrame {
    pub fn from(ptr: *mut c_void) -> wxDocMDIChildFrame { wxDocMDIChildFrame { ptr: ptr } }
    pub fn null() -> wxDocMDIChildFrame { wxDocMDIChildFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocMDIChildFrame : _wxMDIChildFrame {
}

pub struct wxDocMDIParentFrame { ptr: *mut c_void }
impl _wxDocMDIParentFrame for wxDocMDIParentFrame {}
impl _wxMDIParentFrame for wxDocMDIParentFrame {}
impl _wxFrame for wxDocMDIParentFrame {}
impl _wxTopLevelWindow for wxDocMDIParentFrame {}
impl _wxWindow for wxDocMDIParentFrame {}
impl _wxEvtHandler for wxDocMDIParentFrame {}
impl _wxObject for wxDocMDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocMDIParentFrame {
    pub fn from(ptr: *mut c_void) -> wxDocMDIParentFrame { wxDocMDIParentFrame { ptr: ptr } }
    pub fn null() -> wxDocMDIParentFrame { wxDocMDIParentFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocMDIParentFrame : _wxMDIParentFrame {
}

pub struct wxDocManager { ptr: *mut c_void }
impl _wxDocManager for wxDocManager {}
impl _wxEvtHandler for wxDocManager {}
impl _wxObject for wxDocManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocManager {
    pub fn from(ptr: *mut c_void) -> wxDocManager { wxDocManager { ptr: ptr } }
    pub fn null() -> wxDocManager { wxDocManager::from(0 as *mut c_void) }
    
}

pub trait _wxDocManager : _wxEvtHandler {
}

pub struct wxDocParentFrame { ptr: *mut c_void }
impl _wxDocParentFrame for wxDocParentFrame {}
impl _wxFrame for wxDocParentFrame {}
impl _wxTopLevelWindow for wxDocParentFrame {}
impl _wxWindow for wxDocParentFrame {}
impl _wxEvtHandler for wxDocParentFrame {}
impl _wxObject for wxDocParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocParentFrame {
    pub fn from(ptr: *mut c_void) -> wxDocParentFrame { wxDocParentFrame { ptr: ptr } }
    pub fn null() -> wxDocParentFrame { wxDocParentFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocParentFrame : _wxFrame {
}

pub struct wxDocTemplate { ptr: *mut c_void }
impl _wxDocTemplate for wxDocTemplate {}
impl _wxObject for wxDocTemplate { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocTemplate {
    pub fn from(ptr: *mut c_void) -> wxDocTemplate { wxDocTemplate { ptr: ptr } }
    pub fn null() -> wxDocTemplate { wxDocTemplate::from(0 as *mut c_void) }
    
}

pub trait _wxDocTemplate : _wxObject {
}

pub struct wxDocument { ptr: *mut c_void }
impl _wxDocument for wxDocument {}
impl _wxEvtHandler for wxDocument {}
impl _wxObject for wxDocument { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDocument {
    pub fn from(ptr: *mut c_void) -> wxDocument { wxDocument { ptr: ptr } }
    pub fn null() -> wxDocument { wxDocument::from(0 as *mut c_void) }
    
}

pub trait _wxDocument : _wxEvtHandler {
}

pub struct wxDragImage { ptr: *mut c_void }
impl _wxDragImage for wxDragImage {}
impl _wxObject for wxDragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDragImage {
    pub fn from(ptr: *mut c_void) -> wxDragImage { wxDragImage { ptr: ptr } }
    pub fn null() -> wxDragImage { wxDragImage::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxBitmap>(image: &T, x: c_int, y: c_int) -> wxDragImage {
        unsafe { wxDragImage { ptr: wxDragImage_Create(image.ptr(), x, y) } }
    }
}

pub trait _wxDragImage : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginDragFullScreen<T: _wxWindow, U: _wxRect>(&self, x_pos: c_int, y_pos: c_int, window: &T, fullScreen: c_int, rect: &U) -> c_int {
        unsafe { wxDragImage_BeginDragFullScreen(self.ptr(), x_pos, y_pos, window.ptr(), fullScreen, rect.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginDrag<T: _wxWindow, U: _wxWindow>(&self, x: c_int, y: c_int, window: &T, boundingWindow: &U) -> c_int {
        unsafe { wxDragImage_BeginDrag(self.ptr(), x, y, window.ptr(), boundingWindow.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDrag(&self) {
        unsafe { wxDragImage_EndDrag(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) -> c_int {
        unsafe { wxDragImage_Hide(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxDragImage_Move(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) -> c_int {
        unsafe { wxDragImage_Show(self.ptr()) }
    }
}

pub struct wxDrawControl { ptr: *mut c_void }
impl _wxDrawControl for wxDrawControl {}
impl _wxControl for wxDrawControl {}
impl _wxWindow for wxDrawControl {}
impl _wxEvtHandler for wxDrawControl {}
impl _wxObject for wxDrawControl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDrawControl {
    pub fn from(ptr: *mut c_void) -> wxDrawControl { wxDrawControl { ptr: ptr } }
    pub fn null() -> wxDrawControl { wxDrawControl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxDrawControl {
        unsafe { wxDrawControl { ptr: wxDrawControl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxDrawControl : _wxControl {
}

pub struct wxDrawWindow { ptr: *mut c_void }
impl _wxDrawWindow for wxDrawWindow {}
impl _wxWindow for wxDrawWindow {}
impl _wxEvtHandler for wxDrawWindow {}
impl _wxObject for wxDrawWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDrawWindow {
    pub fn from(ptr: *mut c_void) -> wxDrawWindow { wxDrawWindow { ptr: ptr } }
    pub fn null() -> wxDrawWindow { wxDrawWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxDrawWindow {
        unsafe { wxDrawWindow { ptr: wxDrawWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxDrawWindow : _wxWindow {
}

pub struct wxDropFilesEvent { ptr: *mut c_void }
impl _wxDropFilesEvent for wxDropFilesEvent {}
impl _wxEvent for wxDropFilesEvent {}
impl _wxObject for wxDropFilesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDropFilesEvent {
    pub fn from(ptr: *mut c_void) -> wxDropFilesEvent { wxDropFilesEvent { ptr: ptr } }
    pub fn null() -> wxDropFilesEvent { wxDropFilesEvent::from(0 as *mut c_void) }
    
}

pub trait _wxDropFilesEvent : _wxEvent {
}

pub struct wxDropSource { ptr: *mut c_void }
impl _wxDropSource for wxDropSource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDropSource {
    pub fn from(ptr: *mut c_void) -> wxDropSource { wxDropSource { ptr: ptr } }
    pub fn null() -> wxDropSource { wxDropSource::from(0 as *mut c_void) }
    
}

pub trait _wxDropSource {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxDropTarget { ptr: *mut c_void }
impl _wxDropTarget for wxDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxDropTarget {
    pub fn from(ptr: *mut c_void) -> wxDropTarget { wxDropTarget { ptr: ptr } }
    pub fn null() -> wxDropTarget { wxDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxDropTarget {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataObject<T: _wxDataObject>(&self, _dat: &T) {
        unsafe { wxDropTarget_SetDataObject(self.ptr(), _dat.ptr()) }
    }
}

pub struct wxEraseEvent { ptr: *mut c_void }
impl _wxEraseEvent for wxEraseEvent {}
impl _wxEvent for wxEraseEvent {}
impl _wxObject for wxEraseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxEraseEvent {
    pub fn from(ptr: *mut c_void) -> wxEraseEvent { wxEraseEvent { ptr: ptr } }
    pub fn null() -> wxEraseEvent { wxEraseEvent::from(0 as *mut c_void) }
    
}

pub trait _wxEraseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDC(&self) -> wxDC {
        unsafe { wxDC { ptr: wxEraseEvent_GetDC(self.ptr()) } }
    }
}

pub struct wxEvent { ptr: *mut c_void }
impl _wxEvent for wxEvent {}
impl _wxObject for wxEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxEvent {
    pub fn from(ptr: *mut c_void) -> wxEvent { wxEvent { ptr: ptr } }
    pub fn null() -> wxEvent { wxEvent::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

pub trait _wxEvent : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn copyObject(&self, object_dest: *mut c_void) {
        unsafe { wxEvent_CopyObject(self.ptr(), object_dest) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEventObject(&self) -> wxObject {
        unsafe { wxObject { ptr: wxEvent_GetEventObject(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEventType(&self) -> c_int {
        unsafe { wxEvent_GetEventType(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxEvent_GetId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSkipped(&self) -> c_int {
        unsafe { wxEvent_GetSkipped(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTimestamp(&self) -> c_int {
        unsafe { wxEvent_GetTimestamp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCommandEvent(&self) -> c_int {
        unsafe { wxEvent_IsCommandEvent(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEventObject<T: _wxObject>(&self, obj: &T) {
        unsafe { wxEvent_SetEventObject(self.ptr(), obj.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEventType(&self, typ: c_int) {
        unsafe { wxEvent_SetEventType(self.ptr(), typ) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, Id: c_int) {
        unsafe { wxEvent_SetId(self.ptr(), Id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTimestamp(&self, ts: c_int) {
        unsafe { wxEvent_SetTimestamp(self.ptr(), ts) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn skip(&self) {
        unsafe { wxEvent_Skip(self.ptr()) }
    }
}

pub struct wxEvtHandler { ptr: *mut c_void }
impl _wxEvtHandler for wxEvtHandler {}
impl _wxObject for wxEvtHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxEvtHandler {
    pub fn from(ptr: *mut c_void) -> wxEvtHandler { wxEvtHandler { ptr: ptr } }
    pub fn null() -> wxEvtHandler { wxEvtHandler::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxEvtHandler {
        unsafe { wxEvtHandler { ptr: wxEvtHandler_Create() } }
    }
}

pub trait _wxEvtHandler : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPendingEvent<T: _wxEvent>(&self, event: &T) {
        unsafe { wxEvtHandler_AddPendingEvent(self.ptr(), event.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn connect(&self, first: c_int, last: c_int, type_: c_int, data: *mut c_void) -> c_int {
        unsafe { wxEvtHandler_Connect(self.ptr(), first, last, type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disconnect(&self, first: c_int, last: c_int, type_: c_int, id: c_int) -> c_int {
        unsafe { wxEvtHandler_Disconnect(self.ptr(), first, last, type_, id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEvtHandlerEnabled(&self) -> c_int {
        unsafe { wxEvtHandler_GetEvtHandlerEnabled(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextHandler(&self) -> wxEvtHandler {
        unsafe { wxEvtHandler { ptr: wxEvtHandler_GetNextHandler(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPreviousHandler(&self) -> wxEvtHandler {
        unsafe { wxEvtHandler { ptr: wxEvtHandler_GetPreviousHandler(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn processEvent<T: _wxEvent>(&self, event: &T) -> c_int {
        unsafe { wxEvtHandler_ProcessEvent(self.ptr(), event.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEvtHandlerEnabled(&self, enabled: c_int) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self.ptr(), enabled) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNextHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetNextHandler(self.ptr(), handler.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPreviousHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.ptr(), handler.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClosure(&self, id: c_int, type_: c_int) -> wxClosure {
        unsafe { wxClosure { ptr: wxEvtHandler_GetClosure(self.ptr(), id, type_) } }
    }
}

pub struct wxFileDataObject { ptr: *mut c_void }
impl _wxFileDataObject for wxFileDataObject {}
impl _wxDataObjectSimple for wxFileDataObject {}
impl _wxDataObject for wxFileDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileDataObject {
    pub fn from(ptr: *mut c_void) -> wxFileDataObject { wxFileDataObject { ptr: ptr } }
    pub fn null() -> wxFileDataObject { wxFileDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxFileDataObject : _wxDataObjectSimple {
}

pub struct wxFileDialog { ptr: *mut c_void }
impl _wxFileDialog for wxFileDialog {}
impl _wxDialog for wxFileDialog {}
impl _wxTopLevelWindow for wxFileDialog {}
impl _wxWindow for wxFileDialog {}
impl _wxEvtHandler for wxFileDialog {}
impl _wxObject for wxFileDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileDialog {
    pub fn from(ptr: *mut c_void) -> wxFileDialog { wxFileDialog { ptr: ptr } }
    pub fn null() -> wxFileDialog { wxFileDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _dir: &str, _fle: &str, _wcd: &str, _lft: c_int, _top: c_int, _stl: c_int) -> wxFileDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        let _fle = wxT(_fle);
        let _wcd = wxT(_wcd);
        unsafe { wxFileDialog { ptr: wxFileDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _fle.ptr(), _wcd.ptr(), _lft, _top, _stl) } }
    }
}

pub trait _wxFileDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirectory(&self) -> ~str {
        unsafe { wxString { ptr: wxFileDialog_GetDirectory(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilename(&self) -> ~str {
        unsafe { wxString { ptr: wxFileDialog_GetFilename(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilenames(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetFilenames(self.ptr(), paths) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilterIndex(&self) -> c_int {
        unsafe { wxFileDialog_GetFilterIndex(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMessage(&self) -> ~str {
        unsafe { wxString { ptr: wxFileDialog_GetMessage(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPath(&self) -> ~str {
        unsafe { wxString { ptr: wxFileDialog_GetPath(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaths(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetPaths(self.ptr(), paths) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxFileDialog_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWildcard(&self) -> ~str {
        unsafe { wxString { ptr: wxFileDialog_GetWildcard(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDirectory(&self, dir: &str) {
        let dir = wxT(dir);
        unsafe { wxFileDialog_SetDirectory(self.ptr(), dir.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFilename(&self, name: &str) {
        let name = wxT(name);
        unsafe { wxFileDialog_SetFilename(self.ptr(), name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.ptr(), filterIndex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMessage(&self, message: &str) {
        let message = wxT(message);
        unsafe { wxFileDialog_SetMessage(self.ptr(), message.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPath(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxFileDialog_SetPath(self.ptr(), path.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWildcard(&self, wildCard: &str) {
        let wildCard = wxT(wildCard);
        unsafe { wxFileDialog_SetWildcard(self.ptr(), wildCard.ptr()) }
    }
}

pub struct wxFileDropTarget { ptr: *mut c_void }
impl _wxFileDropTarget for wxFileDropTarget {}
impl _wxDropTarget for wxFileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileDropTarget {
    pub fn from(ptr: *mut c_void) -> wxFileDropTarget { wxFileDropTarget { ptr: ptr } }
    pub fn null() -> wxFileDropTarget { wxFileDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxFileDropTarget : _wxDropTarget {
}

pub struct wxFileHistory { ptr: *mut c_void }
impl _wxFileHistory for wxFileHistory {}
impl _wxObject for wxFileHistory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileHistory {
    pub fn from(ptr: *mut c_void) -> wxFileHistory { wxFileHistory { ptr: ptr } }
    pub fn null() -> wxFileHistory { wxFileHistory::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(maxFiles: c_int) -> wxFileHistory {
        unsafe { wxFileHistory { ptr: wxFileHistory_Create(maxFiles) } }
    }
}

pub trait _wxFileHistory : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFileToHistory(&self, file: &str) {
        let file = wxT(file);
        unsafe { wxFileHistory_AddFileToHistory(self.ptr(), file.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFilesToMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_AddFilesToMenu(self.ptr(), menu.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxFileHistory_GetCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHistoryFile(&self, i: c_int) -> ~str {
        unsafe { wxString { ptr: wxFileHistory_GetHistoryFile(self.ptr(), i) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxFiles(&self) -> c_int {
        unsafe { wxFileHistory_GetMaxFiles(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenus(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFileHistory_GetMenus(self.ptr(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load<T: _wxConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Load(self.ptr(), config.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.ptr(), i) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_RemoveMenu(self.ptr(), menu.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn save<T: _wxConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Save(self.ptr(), config.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn useMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_UseMenu(self.ptr(), menu.ptr()) }
    }
}

pub struct wxFileType { ptr: *mut c_void }
impl _wxFileType for wxFileType { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFileType {
    pub fn from(ptr: *mut c_void) -> wxFileType { wxFileType { ptr: ptr } }
    pub fn null() -> wxFileType { wxFileType::from(0 as *mut c_void) }
    
}

pub trait _wxFileType {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expandCommand(&self, _cmd: *mut c_void, _params: *mut c_void) -> ~str {
        unsafe { wxString { ptr: wxFileType_ExpandCommand(self.ptr(), _cmd, _params) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDescription(&self) -> ~str {
        unsafe { wxString { ptr: wxFileType_GetDescription(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExtensions<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetExtensions(self.ptr(), _lst.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon<T: _wxIcon>(&self, icon: &T) -> c_int {
        unsafe { wxFileType_GetIcon(self.ptr(), icon.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMimeType(&self) -> ~str {
        unsafe { wxString { ptr: wxFileType_GetMimeType(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMimeTypes<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetMimeTypes(self.ptr(), _lst.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOpenCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.ptr(), _buf, _params) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.ptr(), _buf, _params) }
    }
}

pub struct wxFindDialogEvent { ptr: *mut c_void }
impl _wxFindDialogEvent for wxFindDialogEvent {}
impl _wxCommandEvent for wxFindDialogEvent {}
impl _wxEvent for wxFindDialogEvent {}
impl _wxObject for wxFindDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFindDialogEvent {
    pub fn from(ptr: *mut c_void) -> wxFindDialogEvent { wxFindDialogEvent { ptr: ptr } }
    pub fn null() -> wxFindDialogEvent { wxFindDialogEvent::from(0 as *mut c_void) }
    
}

pub trait _wxFindDialogEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFindString(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFindDialogEvent_GetFindString(self.ptr(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindDialogEvent_GetFlags(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReplaceString(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFindDialogEvent_GetReplaceString(self.ptr(), _ref) }
    }
}

pub struct wxFindReplaceData { ptr: *mut c_void }
impl _wxFindReplaceData for wxFindReplaceData {}
impl _wxObject for wxFindReplaceData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFindReplaceData {
    pub fn from(ptr: *mut c_void) -> wxFindReplaceData { wxFindReplaceData { ptr: ptr } }
    pub fn null() -> wxFindReplaceData { wxFindReplaceData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int) -> wxFindReplaceData {
        unsafe { wxFindReplaceData { ptr: wxFindReplaceData_Create(flags) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxFindReplaceData {
        unsafe { wxFindReplaceData { ptr: wxFindReplaceData_CreateDefault() } }
    }
}

pub trait _wxFindReplaceData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFindString(&self) -> ~str {
        unsafe { wxString { ptr: wxFindReplaceData_GetFindString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReplaceString(&self) -> ~str {
        unsafe { wxString { ptr: wxFindReplaceData_GetReplaceString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFindString(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxFindReplaceData_SetFindString(self.ptr(), str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.ptr(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReplaceString(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxFindReplaceData_SetReplaceString(self.ptr(), str.ptr()) }
    }
}

pub struct wxFindReplaceDialog { ptr: *mut c_void }
impl _wxFindReplaceDialog for wxFindReplaceDialog {}
impl _wxDialog for wxFindReplaceDialog {}
impl _wxTopLevelWindow for wxFindReplaceDialog {}
impl _wxWindow for wxFindReplaceDialog {}
impl _wxEvtHandler for wxFindReplaceDialog {}
impl _wxObject for wxFindReplaceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFindReplaceDialog {
    pub fn from(ptr: *mut c_void) -> wxFindReplaceDialog { wxFindReplaceDialog { ptr: ptr } }
    pub fn null() -> wxFindReplaceDialog { wxFindReplaceDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxFindReplaceData>(parent: &T, data: &U, title: &str, style: c_int) -> wxFindReplaceDialog {
        let title = wxT(title);
        unsafe { wxFindReplaceDialog { ptr: wxFindReplaceDialog_Create(parent.ptr(), data.ptr(), title.ptr(), style) } }
    }
}

pub trait _wxFindReplaceDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> wxFindReplaceData {
        unsafe { wxFindReplaceData { ptr: wxFindReplaceDialog_GetData(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData<T: _wxFindReplaceData>(&self, data: &T) {
        unsafe { wxFindReplaceDialog_SetData(self.ptr(), data.ptr()) }
    }
}

pub struct wxFlexGridSizer { ptr: *mut c_void }
impl _wxFlexGridSizer for wxFlexGridSizer {}
impl _wxGridSizer for wxFlexGridSizer {}
impl _wxSizer for wxFlexGridSizer {}
impl _wxObject for wxFlexGridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFlexGridSizer {
    pub fn from(ptr: *mut c_void) -> wxFlexGridSizer { wxFlexGridSizer { ptr: ptr } }
    pub fn null() -> wxFlexGridSizer { wxFlexGridSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> wxFlexGridSizer {
        unsafe { wxFlexGridSizer { ptr: wxFlexGridSizer_Create(rows, cols, vgap, hgap) } }
    }
}

pub trait _wxFlexGridSizer : _wxGridSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self.ptr(), idx) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self.ptr(), idx) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableCol(self.ptr(), idx) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableRow(self.ptr(), idx) }
    }
}

pub struct wxFocusEvent { ptr: *mut c_void }
impl _wxFocusEvent for wxFocusEvent {}
impl _wxEvent for wxFocusEvent {}
impl _wxObject for wxFocusEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFocusEvent {
    pub fn from(ptr: *mut c_void) -> wxFocusEvent { wxFocusEvent { ptr: ptr } }
    pub fn null() -> wxFocusEvent { wxFocusEvent::from(0 as *mut c_void) }
    
}

pub trait _wxFocusEvent : _wxEvent {
}

pub struct wxFont { ptr: *mut c_void }
impl _wxFont for wxFont {}
impl _wxGDIObject for wxFont {}
impl _wxObject for wxFont { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFont {
    pub fn from(ptr: *mut c_void) -> wxFont { wxFont { ptr: ptr } }
    pub fn null() -> wxFont { wxFont::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: c_int, face: &str, enc: c_int) -> wxFont {
        let face = wxT(face);
        unsafe { wxFont { ptr: wxFont_Create(pointSize, family, style, weight, underlined, face.ptr(), enc) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> wxFont {
        unsafe { wxFont { ptr: wxFont_CreateFromStock(id) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxFont {
        unsafe { wxFont { ptr: wxFont_CreateDefault() } }
    }
}

pub trait _wxFont : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultEncoding(&self) -> c_int {
        unsafe { wxFont_GetDefaultEncoding(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEncoding(&self) -> c_int {
        unsafe { wxFont_GetEncoding(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFaceName(&self) -> ~str {
        unsafe { wxString { ptr: wxFont_GetFaceName(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFamilyString(&self) -> ~str {
        unsafe { wxString { ptr: wxFont_GetFamilyString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPointSize(&self) -> c_int {
        unsafe { wxFont_GetPointSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxFont_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyleString(&self) -> ~str {
        unsafe { wxString { ptr: wxFont_GetStyleString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUnderlined(&self) -> c_int {
        unsafe { wxFont_GetUnderlined(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeight(&self) -> c_int {
        unsafe { wxFont_GetWeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeightString(&self) -> ~str {
        unsafe { wxString { ptr: wxFont_GetWeightString(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxFont_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetDefaultEncoding(self.ptr(), encoding) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetEncoding(self.ptr(), encoding) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFaceName(&self, faceName: &str) {
        let faceName = wxT(faceName);
        unsafe { wxFont_SetFaceName(self.ptr(), faceName.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFamily(&self, family: c_int) {
        unsafe { wxFont_SetFamily(self.ptr(), family) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPointSize(&self, pointSize: c_int) {
        unsafe { wxFont_SetPointSize(self.ptr(), pointSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFont_SetStyle(self.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUnderlined(&self, underlined: c_int) {
        unsafe { wxFont_SetUnderlined(self.ptr(), underlined) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWeight(&self, weight: c_int) {
        unsafe { wxFont_SetWeight(self.ptr(), weight) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxFont_IsStatic(self.ptr()) }
    }
}

pub struct wxFontData { ptr: *mut c_void }
impl _wxFontData for wxFontData {}
impl _wxObject for wxFontData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFontData {
    pub fn from(ptr: *mut c_void) -> wxFontData { wxFontData { ptr: ptr } }
    pub fn null() -> wxFontData { wxFontData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxFontData {
        unsafe { wxFontData { ptr: wxFontData_Create() } }
    }
}

pub trait _wxFontData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableEffects(&self, flag: c_int) {
        unsafe { wxFontData_EnableEffects(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllowSymbols(&self) -> c_int {
        unsafe { wxFontData_GetAllowSymbols(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChosenFont<T: _wxFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetChosenFont(self.ptr(), ref_.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxFontData_GetColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableEffects(&self) -> c_int {
        unsafe { wxFontData_GetEnableEffects(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEncoding(&self) -> c_int {
        unsafe { wxFontData_GetEncoding(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInitialFont<T: _wxFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetInitialFont(self.ptr(), ref_.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getShowHelp(&self) -> c_int {
        unsafe { wxFontData_GetShowHelp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAllowSymbols(&self, flag: c_int) {
        unsafe { wxFontData_SetAllowSymbols(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChosenFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxFontData_SetChosenFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxFontData_SetColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.ptr(), encoding) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInitialFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxFontData_SetInitialFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, minRange: c_int, maxRange: c_int) {
        unsafe { wxFontData_SetRange(self.ptr(), minRange, maxRange) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShowHelp(&self, flag: c_int) {
        unsafe { wxFontData_SetShowHelp(self.ptr(), flag) }
    }
}

pub struct wxFontDialog { ptr: *mut c_void }
impl _wxFontDialog for wxFontDialog {}
impl _wxDialog for wxFontDialog {}
impl _wxTopLevelWindow for wxFontDialog {}
impl _wxWindow for wxFontDialog {}
impl _wxEvtHandler for wxFontDialog {}
impl _wxObject for wxFontDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFontDialog {
    pub fn from(ptr: *mut c_void) -> wxFontDialog { wxFontDialog { ptr: ptr } }
    pub fn null() -> wxFontDialog { wxFontDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxFontData>(_prt: &T, fnt: &U) -> wxFontDialog {
        unsafe { wxFontDialog { ptr: wxFontDialog_Create(_prt.ptr(), fnt.ptr()) } }
    }
}

pub trait _wxFontDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFontData<T: _wxFontData>(&self, _ref: &T) {
        unsafe { wxFontDialog_GetFontData(self.ptr(), _ref.ptr()) }
    }
}

pub struct wxFontEnumerator { ptr: *mut c_void }
impl _wxFontEnumerator for wxFontEnumerator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFontEnumerator {
    pub fn from(ptr: *mut c_void) -> wxFontEnumerator { wxFontEnumerator { ptr: ptr } }
    pub fn null() -> wxFontEnumerator { wxFontEnumerator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> wxFontEnumerator {
        unsafe { wxFontEnumerator { ptr: wxFontEnumerator_Create(_obj, _fnc) } }
    }
}

pub trait _wxFontEnumerator {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumerateEncodings(&self, facename: &str) -> c_int {
        let facename = wxT(facename);
        unsafe { wxFontEnumerator_EnumerateEncodings(self.ptr(), facename.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> c_int {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.ptr(), encoding, fixedWidthOnly) }
    }
}

pub struct wxFontList { ptr: *mut c_void }
impl _wxFontList for wxFontList {}
impl _wxList for wxFontList {}
impl _wxObject for wxFontList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFontList {
    pub fn from(ptr: *mut c_void) -> wxFontList { wxFontList { ptr: ptr } }
    pub fn null() -> wxFontList { wxFontList::from(0 as *mut c_void) }
    
}

pub trait _wxFontList : _wxList {
}

pub struct wxFontMapper { ptr: *mut c_void }
impl _wxFontMapper for wxFontMapper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFontMapper {
    pub fn from(ptr: *mut c_void) -> wxFontMapper { wxFontMapper { ptr: ptr } }
    pub fn null() -> wxFontMapper { wxFontMapper::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxFontMapper {
        unsafe { wxFontMapper { ptr: wxFontMapper_Create() } }
    }
}

pub trait _wxFontMapper {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *mut c_void, _buf: &str) -> c_int {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_GetAltForEncoding(self.ptr(), encoding, alt_encoding, _buf.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEncodingAvailable(&self, encoding: c_int, _buf: &str) -> c_int {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_IsEncodingAvailable(self.ptr(), encoding, _buf.ptr()) }
    }
}

pub struct wxFrame { ptr: *mut c_void }
impl _wxFrame for wxFrame {}
impl _wxTopLevelWindow for wxFrame {}
impl _wxWindow for wxFrame {}
impl _wxEvtHandler for wxFrame {}
impl _wxObject for wxFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxFrame {
    pub fn from(ptr: *mut c_void) -> wxFrame { wxFrame { ptr: ptr } }
    pub fn null() -> wxFrame { wxFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxFrame {
        let _txt = wxT(_txt);
        unsafe { wxFrame { ptr: wxFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxFrame : _wxTopLevelWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newStatusBar(&self, number: c_int, style: c_int) -> wxStatusBar {
        unsafe { wxStatusBar { ptr: wxFrame_CreateStatusBar(self.ptr(), number, style) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn newToolBar(&self, style: c_long) -> wxToolBar {
        unsafe { wxToolBar { ptr: wxFrame_CreateToolBar(self.ptr(), style) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientAreaOrigin_left(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientAreaOrigin_top(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuBar(&self) -> wxMenuBar {
        unsafe { wxMenuBar { ptr: wxFrame_GetMenuBar(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStatusBar(&self) -> wxStatusBar {
        unsafe { wxStatusBar { ptr: wxFrame_GetStatusBar(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolBar(&self) -> wxToolBar {
        unsafe { wxToolBar { ptr: wxFrame_GetToolBar(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMenuBar<T: _wxMenuBar>(&self, menubar: &T) {
        unsafe { wxFrame_SetMenuBar(self.ptr(), menubar.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusBar<T: _wxStatusBar>(&self, statBar: &T) {
        unsafe { wxFrame_SetStatusBar(self.ptr(), statBar.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusText(&self, _txt: &str, _number: c_int) {
        let _txt = wxT(_txt);
        unsafe { wxFrame_SetStatusText(self.ptr(), _txt.ptr(), _number) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusWidths(&self, _n: c_int, _widths_field: *mut c_void) {
        unsafe { wxFrame_SetStatusWidths(self.ptr(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolBar<T: _wxToolBar>(&self, _toolbar: &T) {
        unsafe { wxFrame_SetToolBar(self.ptr(), _toolbar.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShape<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxFrame_SetShape(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showFullScreen(&self, show: c_int, style: c_int) -> c_int {
        unsafe { wxFrame_ShowFullScreen(self.ptr(), show, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isFullScreen(&self) -> c_int {
        unsafe { wxFrame_IsFullScreen(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centre(&self, orientation: c_int) {
        unsafe { wxFrame_Centre(self.ptr(), orientation) }
    }
}

pub struct wxGDIObject { ptr: *mut c_void }
impl _wxGDIObject for wxGDIObject {}
impl _wxObject for wxGDIObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGDIObject {
    pub fn from(ptr: *mut c_void) -> wxGDIObject { wxGDIObject { ptr: ptr } }
    pub fn null() -> wxGDIObject { wxGDIObject::from(0 as *mut c_void) }
    
}

pub trait _wxGDIObject : _wxObject {
}

pub struct wxGauge { ptr: *mut c_void }
impl _wxGauge for wxGauge {}
impl _wxControl for wxGauge {}
impl _wxWindow for wxGauge {}
impl _wxEvtHandler for wxGauge {}
impl _wxObject for wxGauge { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGauge {
    pub fn from(ptr: *mut c_void) -> wxGauge { wxGauge { ptr: ptr } }
    pub fn null() -> wxGauge { wxGauge::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxGauge {
        unsafe { wxGauge { ptr: wxGauge_Create(_prt.ptr(), _id, _rng, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxGauge : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBezelFace(&self) -> c_int {
        unsafe { wxGauge_GetBezelFace(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRange(&self) -> c_int {
        unsafe { wxGauge_GetRange(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getShadowWidth(&self) -> c_int {
        unsafe { wxGauge_GetShadowWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxGauge_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBezelFace(&self, w: c_int) {
        unsafe { wxGauge_SetBezelFace(self.ptr(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, r: c_int) {
        unsafe { wxGauge_SetRange(self.ptr(), r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShadowWidth(&self, w: c_int) {
        unsafe { wxGauge_SetShadowWidth(self.ptr(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, pos: c_int) {
        unsafe { wxGauge_SetValue(self.ptr(), pos) }
    }
}

pub struct wxGenericDirCtrl { ptr: *mut c_void }
impl _wxGenericDirCtrl for wxGenericDirCtrl {}
impl _wxControl for wxGenericDirCtrl {}
impl _wxWindow for wxGenericDirCtrl {}
impl _wxEvtHandler for wxGenericDirCtrl {}
impl _wxObject for wxGenericDirCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGenericDirCtrl {
    pub fn from(ptr: *mut c_void) -> wxGenericDirCtrl { wxGenericDirCtrl { ptr: ptr } }
    pub fn null() -> wxGenericDirCtrl { wxGenericDirCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxGenericDirCtrl : _wxControl {
}

pub struct wxGenericValidator { ptr: *mut c_void }
impl _wxGenericValidator for wxGenericValidator {}
impl _wxValidator for wxGenericValidator {}
impl _wxEvtHandler for wxGenericValidator {}
impl _wxObject for wxGenericValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGenericValidator {
    pub fn from(ptr: *mut c_void) -> wxGenericValidator { wxGenericValidator { ptr: ptr } }
    pub fn null() -> wxGenericValidator { wxGenericValidator::from(0 as *mut c_void) }
    
}

pub trait _wxGenericValidator : _wxValidator {
}

pub struct wxGridSizer { ptr: *mut c_void }
impl _wxGridSizer for wxGridSizer {}
impl _wxSizer for wxGridSizer {}
impl _wxObject for wxGridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridSizer {
    pub fn from(ptr: *mut c_void) -> wxGridSizer { wxGridSizer { ptr: ptr } }
    pub fn null() -> wxGridSizer { wxGridSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> wxGridSizer {
        unsafe { wxGridSizer { ptr: wxGridSizer_Create(rows, cols, vgap, hgap) } }
    }
}

pub trait _wxGridSizer : _wxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCols(&self) -> c_int {
        unsafe { wxGridSizer_GetCols(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHGap(&self) -> c_int {
        unsafe { wxGridSizer_GetHGap(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRows(&self) -> c_int {
        unsafe { wxGridSizer_GetRows(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVGap(&self) -> c_int {
        unsafe { wxGridSizer_GetVGap(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCols(&self, cols: c_int) {
        unsafe { wxGridSizer_SetCols(self.ptr(), cols) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetHGap(self.ptr(), gap) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRows(&self, rows: c_int) {
        unsafe { wxGridSizer_SetRows(self.ptr(), rows) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetVGap(self.ptr(), gap) }
    }
}

pub struct wxHelpController { ptr: *mut c_void }
impl _wxHelpController for wxHelpController {}
impl _wxHelpControllerBase for wxHelpController {}
impl _wxObject for wxHelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHelpController {
    pub fn from(ptr: *mut c_void) -> wxHelpController { wxHelpController { ptr: ptr } }
    pub fn null() -> wxHelpController { wxHelpController::from(0 as *mut c_void) }
    
}

pub trait _wxHelpController : _wxHelpControllerBase {
}

pub struct wxHelpControllerBase { ptr: *mut c_void }
impl _wxHelpControllerBase for wxHelpControllerBase {}
impl _wxObject for wxHelpControllerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHelpControllerBase {
    pub fn from(ptr: *mut c_void) -> wxHelpControllerBase { wxHelpControllerBase { ptr: ptr } }
    pub fn null() -> wxHelpControllerBase { wxHelpControllerBase::from(0 as *mut c_void) }
    
}

pub trait _wxHelpControllerBase : _wxObject {
}

pub struct wxHelpControllerHelpProvider { ptr: *mut c_void }
impl _wxHelpControllerHelpProvider for wxHelpControllerHelpProvider {}
impl _wxSimpleHelpProvider for wxHelpControllerHelpProvider {}
impl _wxHelpProvider for wxHelpControllerHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHelpControllerHelpProvider {
    pub fn from(ptr: *mut c_void) -> wxHelpControllerHelpProvider { wxHelpControllerHelpProvider { ptr: ptr } }
    pub fn null() -> wxHelpControllerHelpProvider { wxHelpControllerHelpProvider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxHelpControllerBase>(ctr: &T) -> wxHelpControllerHelpProvider {
        unsafe { wxHelpControllerHelpProvider { ptr: wxHelpControllerHelpProvider_Create(ctr.ptr()) } }
    }
}

pub trait _wxHelpControllerHelpProvider : _wxSimpleHelpProvider {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelpController(&self) -> wxHelpControllerBase {
        unsafe { wxHelpControllerBase { ptr: wxHelpControllerHelpProvider_GetHelpController(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpController<T: _wxHelpController>(&self, hc: &T) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.ptr(), hc.ptr()) }
    }
}

pub struct wxHelpEvent { ptr: *mut c_void }
impl _wxHelpEvent for wxHelpEvent {}
impl _wxCommandEvent for wxHelpEvent {}
impl _wxEvent for wxHelpEvent {}
impl _wxObject for wxHelpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHelpEvent {
    pub fn from(ptr: *mut c_void) -> wxHelpEvent { wxHelpEvent { ptr: ptr } }
    pub fn null() -> wxHelpEvent { wxHelpEvent::from(0 as *mut c_void) }
    
}

pub trait _wxHelpEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLink(&self) -> ~str {
        unsafe { wxString { ptr: wxHelpEvent_GetLink(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxHelpEvent_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTarget(&self) -> ~str {
        unsafe { wxString { ptr: wxHelpEvent_GetTarget(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLink(&self, link: &str) {
        let link = wxT(link);
        unsafe { wxHelpEvent_SetLink(self.ptr(), link.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTarget(&self, target: &str) {
        let target = wxT(target);
        unsafe { wxHelpEvent_SetTarget(self.ptr(), target.ptr()) }
    }
}

pub struct wxHelpProvider { ptr: *mut c_void }
impl _wxHelpProvider for wxHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHelpProvider {
    pub fn from(ptr: *mut c_void) -> wxHelpProvider { wxHelpProvider { ptr: ptr } }
    pub fn null() -> wxHelpProvider { wxHelpProvider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> wxHelpProvider {
        unsafe { wxHelpProvider { ptr: wxHelpProvider_Get() } }
    }
}

pub trait _wxHelpProvider {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHelp<T: _wxWindow>(&self, window: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxHelpProvider_AddHelp(self.ptr(), window.ptr(), text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHelpById(&self, id: c_int, text: &str) {
        let text = wxT(text);
        unsafe { wxHelpProvider_AddHelpById(self.ptr(), id, text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelp<T: _wxWindow>(&self, window: &T) -> ~str {
        unsafe { wxString { ptr: wxHelpProvider_GetHelp(self.ptr(), window.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeHelp<T: _wxWindow>(&self, window: &T) {
        unsafe { wxHelpProvider_RemoveHelp(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self) -> wxHelpProvider {
        unsafe { wxHelpProvider { ptr: wxHelpProvider_Set(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showHelp<T: _wxWindow>(&self, window: &T) -> c_int {
        unsafe { wxHelpProvider_ShowHelp(self.ptr(), window.ptr()) }
    }
}

pub struct wxIcon { ptr: *mut c_void }
impl _wxIcon for wxIcon {}
impl _wxBitmap for wxIcon {}
impl _wxGDIObject for wxIcon {}
impl _wxObject for wxIcon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIcon {
    pub fn from(ptr: *mut c_void) -> wxIcon { wxIcon { ptr: ptr } }
    pub fn null() -> wxIcon { wxIcon::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxIcon {
        unsafe { wxIcon { ptr: wxIcon_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newLoad(name: &str, type_: c_long, width: c_int, height: c_int) -> wxIcon {
        let name = wxT(name);
        unsafe { wxIcon { ptr: wxIcon_CreateLoad(name.ptr(), type_, width, height) } }
    }
}

pub trait _wxIcon : _wxBitmap {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign(&self, other: *mut c_void) {
        unsafe { wxIcon_Assign(self.ptr(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copyFromBitmap<T: _wxBitmap>(&self, bmp: &T) {
        unsafe { wxIcon_CopyFromBitmap(self.ptr(), bmp.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fromRaw(&self, width: c_int, height: c_int) -> wxIcon {
        unsafe { wxIcon { ptr: wxIcon_FromRaw(self.ptr(), width, height) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fromXPM(&self) -> wxIcon {
        unsafe { wxIcon { ptr: wxIcon_FromXPM(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual(&self, other: &_wxIcon) -> c_int {
        unsafe { wxIcon_IsEqual(self.ptr(), other.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, name: &str, type_: c_long, width: c_int, height: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxIcon_Load(self.ptr(), name.ptr(), type_, width, height) }
    }
}

pub struct wxIconBundle { ptr: *mut c_void }
impl _wxIconBundle for wxIconBundle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIconBundle {
    pub fn from(ptr: *mut c_void) -> wxIconBundle { wxIconBundle { ptr: ptr } }
    pub fn null() -> wxIconBundle { wxIconBundle::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxIconBundle {
        unsafe { wxIconBundle { ptr: wxIconBundle_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFile(file: &str, type_: c_int) -> wxIconBundle {
        let file = wxT(file);
        unsafe { wxIconBundle { ptr: wxIconBundle_CreateFromFile(file.ptr(), type_) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromIcon<T: _wxIcon>(icon: &T) -> wxIconBundle {
        unsafe { wxIconBundle { ptr: wxIconBundle_CreateFromIcon(icon.ptr()) } }
    }
}

pub trait _wxIconBundle {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addIcon<T: _wxIcon>(&self, icon: &T) {
        unsafe { wxIconBundle_AddIcon(self.ptr(), icon.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addIconFromFile(&self, file: &str, type_: c_int) {
        let file = wxT(file);
        unsafe { wxIconBundle_AddIconFromFile(self.ptr(), file.ptr(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxIconBundle>(&self, _ref: &T) {
        unsafe { wxIconBundle_Assign(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon<T: _wxIcon>(&self, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxIconBundle_GetIcon(self.ptr(), w, h, _ref.ptr()) }
    }
}

pub struct wxIconizeEvent { ptr: *mut c_void }
impl _wxIconizeEvent for wxIconizeEvent {}
impl _wxEvent for wxIconizeEvent {}
impl _wxObject for wxIconizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIconizeEvent {
    pub fn from(ptr: *mut c_void) -> wxIconizeEvent { wxIconizeEvent { ptr: ptr } }
    pub fn null() -> wxIconizeEvent { wxIconizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxIconizeEvent : _wxEvent {
}

pub struct wxIdleEvent { ptr: *mut c_void }
impl _wxIdleEvent for wxIdleEvent {}
impl _wxEvent for wxIdleEvent {}
impl _wxObject for wxIdleEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIdleEvent {
    pub fn from(ptr: *mut c_void) -> wxIdleEvent { wxIdleEvent { ptr: ptr } }
    pub fn null() -> wxIdleEvent { wxIdleEvent::from(0 as *mut c_void) }
    
}

pub trait _wxIdleEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn moreRequested(&self) -> c_int {
        unsafe { wxIdleEvent_MoreRequested(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn requestMore(&self, needMore: c_int) {
        unsafe { wxIdleEvent_RequestMore(self.ptr(), needMore) }
    }
}

pub struct wxImage { ptr: *mut c_void }
impl _wxImage for wxImage {}
impl _wxObject for wxImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxImage {
    pub fn from(ptr: *mut c_void) -> wxImage { wxImage { ptr: ptr } }
    pub fn null() -> wxImage { wxImage::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn canRead(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_CanRead(name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(bitmap: &T) -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateFromBitmap(bitmap.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateFromByteString(data, length, type_) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromLazyByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateFromLazyByteString(data, length, type_) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData(width: c_int, height: c_int, data: *mut c_void) -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateFromData(width, height, data) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFile(name: &str) -> wxImage {
        let name = wxT(name);
        unsafe { wxImage { ptr: wxImage_CreateFromFile(name.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newSized(width: c_int, height: c_int) -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateSized(width, height) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromDataEx(width: c_int, height: c_int, data: *mut c_void, isStaticData: c_int) -> wxImage {
        unsafe { wxImage { ptr: wxImage_CreateFromDataEx(width, height, data, isStaticData) } }
    }
}

pub trait _wxImage : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToBitmap<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxImage_ConvertToBitmap(self.ptr(), bitmap.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToByteString(&self, type_: c_int, data: *mut c_char) -> c_int {
        unsafe { wxImage_ConvertToByteString(self.ptr(), type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToLazyByteString(&self, type_: c_int, data: *mut c_char) -> c_int {
        unsafe { wxImage_ConvertToLazyByteString(self.ptr(), type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn countColours(&self, stopafter: c_int) -> c_int {
        unsafe { wxImage_CountColours(self.ptr(), stopafter) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroy(&self) {
        unsafe { wxImage_Destroy(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBlue(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetBlue(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> *mut c_void {
        unsafe { wxImage_GetData(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGreen(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetGreen(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxImage_GetHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskBlue(&self) -> int8_t {
        unsafe { wxImage_GetMaskBlue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskGreen(&self) -> int8_t {
        unsafe { wxImage_GetMaskGreen(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskRed(&self) -> int8_t {
        unsafe { wxImage_GetMaskRed(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRed(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetRed(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSubImage<T: _wxImage>(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: &T) {
        unsafe { wxImage_GetSubImage(self.ptr(), x, y, w, h, image.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxImage_GetWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasMask(&self) -> c_int {
        unsafe { wxImage_HasMask(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOption(&self, name: &str) -> ~str {
        let name = wxT(name);
        unsafe { wxString { ptr: wxImage_GetOption(self.ptr(), name.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOptionInt(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_GetOptionInt(self.ptr(), name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasOption(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_HasOption(self.ptr(), name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Initialize(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initializeFromData(&self, width: c_int, height: c_int, data: *mut c_void) {
        unsafe { wxImage_InitializeFromData(self.ptr(), width, height, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_LoadFile(self.ptr(), name.ptr(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn mirror<T: _wxImage>(&self, horizontally: c_int, image: &T) {
        unsafe { wxImage_Mirror(self.ptr(), horizontally, image.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxImage_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste<T: _wxImage>(&self, image: &T, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self.ptr(), image.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace(&self, r1: uint8_t, g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t, b2: uint8_t) {
        unsafe { wxImage_Replace(self.ptr(), r1, g1, b1, r2, g2, b2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate<T: _wxImage>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *mut c_void, image: &T) {
        unsafe { wxImage_Rotate(self.ptr(), angle, c_x, c_y, interpolating, offset_after_rotation, image.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate90<T: _wxImage>(&self, clockwise: c_int, image: &T) {
        unsafe { wxImage_Rotate90(self.ptr(), clockwise, image.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_SaveFile(self.ptr(), name.ptr(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scale<T: _wxImage>(&self, width: c_int, height: c_int, image: &T) {
        unsafe { wxImage_Scale(self.ptr(), width, height, image.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData(&self, data: *mut c_void) {
        unsafe { wxImage_SetData(self.ptr(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataAndSize(&self, data: *mut c_void, new_width: c_int, new_height: c_int) {
        unsafe { wxImage_SetDataAndSize(self.ptr(), data, new_width, new_height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMask(&self, mask: c_int) {
        unsafe { wxImage_SetMask(self.ptr(), mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaskColour(&self, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetMaskColour(self.ptr(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOption(&self, name: &str, value: &str) {
        let name = wxT(name);
        let value = wxT(value);
        unsafe { wxImage_SetOption(self.ptr(), name.ptr(), value.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOptionInt(&self, name: &str, value: c_int) {
        let name = wxT(name);
        unsafe { wxImage_SetOptionInt(self.ptr(), name.ptr(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRGB(&self, x: c_int, y: c_int, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetRGB(self.ptr(), x, y, r, g, b) }
    }
}

pub struct wxImageHandler { ptr: *mut c_void }
impl _wxImageHandler for wxImageHandler {}
impl _wxObject for wxImageHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxImageHandler {
    pub fn from(ptr: *mut c_void) -> wxImageHandler { wxImageHandler { ptr: ptr } }
    pub fn null() -> wxImageHandler { wxImageHandler::from(0 as *mut c_void) }
    
}

pub trait _wxImageHandler : _wxObject {
}

pub struct wxImageList { ptr: *mut c_void }
impl _wxImageList for wxImageList {}
impl _wxObject for wxImageList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxImageList {
    pub fn from(ptr: *mut c_void) -> wxImageList { wxImageList { ptr: ptr } }
    pub fn null() -> wxImageList { wxImageList::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> wxImageList {
        unsafe { wxImageList { ptr: wxImageList_Create(width, height, mask, initialCount) } }
    }
}

pub trait _wxImageList : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addBitmap<T: _wxBitmap, U: _wxBitmap>(&self, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_AddBitmap(self.ptr(), bitmap.ptr(), mask.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addIcon<T: _wxIcon>(&self, icon: &T) -> c_int {
        unsafe { wxImageList_AddIcon(self.ptr(), icon.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addMasked<T: _wxBitmap, U: _wxColour>(&self, bitmap: &T, maskColour: &U) -> c_int {
        unsafe { wxImageList_AddMasked(self.ptr(), bitmap.ptr(), maskColour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn draw<T: _wxDC>(&self, index: c_int, dc: &T, x: c_int, y: c_int, flags: c_int, solidBackground: c_int) -> c_int {
        unsafe { wxImageList_Draw(self.ptr(), index, dc.ptr(), x, y, flags, solidBackground) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageCount(&self) -> c_int {
        unsafe { wxImageList_GetImageCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self, index: c_int, width: *mut c_int, height: *mut c_int) {
        unsafe { wxImageList_GetSize(self.ptr(), index, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, index: c_int) -> c_int {
        unsafe { wxImageList_Remove(self.ptr(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeAll(&self) -> c_int {
        unsafe { wxImageList_RemoveAll(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxBitmap, U: _wxBitmap>(&self, index: c_int, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_Replace(self.ptr(), index, bitmap.ptr(), mask.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceIcon<T: _wxIcon>(&self, index: c_int, icon: &T) -> c_int {
        unsafe { wxImageList_ReplaceIcon(self.ptr(), index, icon.ptr()) }
    }
}

pub struct wxIndividualLayoutConstraint { ptr: *mut c_void }
impl _wxIndividualLayoutConstraint for wxIndividualLayoutConstraint {}
impl _wxObject for wxIndividualLayoutConstraint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxIndividualLayoutConstraint {
    pub fn from(ptr: *mut c_void) -> wxIndividualLayoutConstraint { wxIndividualLayoutConstraint { ptr: ptr } }
    pub fn null() -> wxIndividualLayoutConstraint { wxIndividualLayoutConstraint::from(0 as *mut c_void) }
    
}

pub trait _wxIndividualLayoutConstraint : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn above<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Above(self.ptr(), sibling.ptr(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn absolute(&self, val: c_int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self.ptr(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn below<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Below(self.ptr(), sibling.ptr(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDone(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetDone(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdge(&self, which: c_int, thisWin: *mut c_void, other: *mut c_void) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetEdge(self.ptr(), which, thisWin, other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMargin(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMargin(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMyEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMyEdge(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOtherEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetOtherEdge(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOtherWindow(&self) -> *mut c_void {
        unsafe { wxIndividualLayoutConstraint_GetOtherWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPercent(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetPercent(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRelationship(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetRelationship(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftOf<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.ptr(), sibling.ptr(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn percentOf<T: _wxWindow>(&self, otherW: &T, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.ptr(), otherW.ptr(), wh, per) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetIfWin<T: _wxWindow>(&self, otherW: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.ptr(), otherW.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightOf<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.ptr(), sibling.ptr(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sameAs<T: _wxWindow>(&self, otherW: &T, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.ptr(), otherW.ptr(), edge, marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn satisfyConstraint<T: _wxWindow>(&self, constraints: *mut c_void, win: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.ptr(), constraints, win.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set<T: _wxWindow>(&self, rel: c_int, otherW: &T, otherE: c_int, val: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Set(self.ptr(), rel, otherW.ptr(), otherE, val, marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDone(&self, d: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetDone(self.ptr(), d) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEdge(&self, which: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetEdge(self.ptr(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargin(&self, m: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetMargin(self.ptr(), m) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRelationship(&self, r: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetRelationship(self.ptr(), r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, v: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetValue(self.ptr(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unconstrained(&self) {
        unsafe { wxIndividualLayoutConstraint_Unconstrained(self.ptr()) }
    }
}

pub struct wxInitDialogEvent { ptr: *mut c_void }
impl _wxInitDialogEvent for wxInitDialogEvent {}
impl _wxEvent for wxInitDialogEvent {}
impl _wxObject for wxInitDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxInitDialogEvent {
    pub fn from(ptr: *mut c_void) -> wxInitDialogEvent { wxInitDialogEvent { ptr: ptr } }
    pub fn null() -> wxInitDialogEvent { wxInitDialogEvent::from(0 as *mut c_void) }
    
}

pub trait _wxInitDialogEvent : _wxEvent {
}

pub struct wxJoystickEvent { ptr: *mut c_void }
impl _wxJoystickEvent for wxJoystickEvent {}
impl _wxEvent for wxJoystickEvent {}
impl _wxObject for wxJoystickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxJoystickEvent {
    pub fn from(ptr: *mut c_void) -> wxJoystickEvent { wxJoystickEvent { ptr: ptr } }
    pub fn null() -> wxJoystickEvent { wxJoystickEvent::from(0 as *mut c_void) }
    
}

pub trait _wxJoystickEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDown(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonDown(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonIsDown(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonIsDown(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonUp(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonUp(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButtonChange(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonChange(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButtonState(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonState(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getJoystick(&self) -> c_int {
        unsafe { wxJoystickEvent_GetJoystick(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxJoystickEvent_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getZPosition(&self) -> c_int {
        unsafe { wxJoystickEvent_GetZPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isButton(&self) -> c_int {
        unsafe { wxJoystickEvent_IsButton(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMove(&self) -> c_int {
        unsafe { wxJoystickEvent_IsMove(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isZMove(&self) -> c_int {
        unsafe { wxJoystickEvent_IsZMove(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setButtonChange(&self, change: c_int) {
        unsafe { wxJoystickEvent_SetButtonChange(self.ptr(), change) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setButtonState(&self, state: c_int) {
        unsafe { wxJoystickEvent_SetButtonState(self.ptr(), state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setJoystick(&self, stick: c_int) {
        unsafe { wxJoystickEvent_SetJoystick(self.ptr(), stick) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxJoystickEvent_SetPosition(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setZPosition(&self, zPos: c_int) {
        unsafe { wxJoystickEvent_SetZPosition(self.ptr(), zPos) }
    }
}

pub struct wxKeyEvent { ptr: *mut c_void }
impl _wxKeyEvent for wxKeyEvent {}
impl _wxEvent for wxKeyEvent {}
impl _wxObject for wxKeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxKeyEvent {
    pub fn from(ptr: *mut c_void) -> wxKeyEvent { wxKeyEvent { ptr: ptr } }
    pub fn null() -> wxKeyEvent { wxKeyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> c_int {
        unsafe { wxKeyEvent_AltDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> c_int {
        unsafe { wxKeyEvent_ControlDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKeyCode(&self) -> c_int {
        unsafe { wxKeyEvent_GetKeyCode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxKeyEvent_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxKeyEvent_GetX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxKeyEvent_GetY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_GetModifiers(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_HasModifiers(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> c_int {
        unsafe { wxKeyEvent_MetaDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setKeyCode(&self, code: c_int) {
        unsafe { wxKeyEvent_SetKeyCode(self.ptr(), code) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> c_int {
        unsafe { wxKeyEvent_ShiftDown(self.ptr()) }
    }
}

pub struct wxLayoutConstraints { ptr: *mut c_void }
impl _wxLayoutConstraints for wxLayoutConstraints {}
impl _wxObject for wxLayoutConstraints { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLayoutConstraints {
    pub fn from(ptr: *mut c_void) -> wxLayoutConstraints { wxLayoutConstraints { ptr: ptr } }
    pub fn null() -> wxLayoutConstraints { wxLayoutConstraints::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxLayoutConstraints {
        unsafe { wxLayoutConstraints { ptr: wxLayoutConstraints_Create() } }
    }
}

pub trait _wxLayoutConstraints : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn bottom(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_bottom(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centreX(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_centreX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centreY(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_centreY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn height(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_height(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn left(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_left(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn right(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_right(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn top(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_top(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn width(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_width(self.ptr()) }
    }
}

pub struct wxListBox { ptr: *mut c_void }
impl _wxListBox for wxListBox {}
impl _wxControl for wxListBox {}
impl _wxWindow for wxListBox {}
impl _wxEvtHandler for wxListBox {}
impl _wxObject for wxListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxListBox {
    pub fn from(ptr: *mut c_void) -> wxListBox { wxListBox { ptr: ptr } }
    pub fn null() -> wxListBox { wxListBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> wxListBox {
        unsafe { wxListBox { ptr: wxListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait _wxListBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxListBox_Append(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendData(&self, item: &str, data: *mut c_void) {
        let item = wxT(item);
        unsafe { wxListBox_AppendData(self.ptr(), item.ptr(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxListBox_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxListBox_FindString(self.ptr(), s.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxListBox_GetCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxListBox_GetSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelections(&self, aSelections: *mut c_int, allocated: c_int) -> c_int {
        unsafe { wxListBox_GetSelections(self.ptr(), aSelections, allocated) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, n: c_int) -> ~str {
        unsafe { wxString { ptr: wxListBox_GetString(self.ptr(), n) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItems(&self, items: *mut c_void, pos: c_int, count: c_int) {
        unsafe { wxListBox_InsertItems(self.ptr(), items, pos, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelected(&self, n: c_int) -> c_int {
        unsafe { wxListBox_IsSelected(self.ptr(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFirstItem(&self, n: c_int) {
        unsafe { wxListBox_SetFirstItem(self.ptr(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, n: c_int, select: c_int) {
        unsafe { wxListBox_SetSelection(self.ptr(), n, select) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setString(&self, n: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxListBox_SetString(self.ptr(), n, s.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStringSelection(&self, str: &str, sel: c_int) {
        let str = wxT(str);
        unsafe { wxListBox_SetStringSelection(self.ptr(), str.ptr(), sel) }
    }
}

pub struct wxListCtrl { ptr: *mut c_void }
impl _wxListCtrl for wxListCtrl {}
impl _wxControl for wxListCtrl {}
impl _wxWindow for wxListCtrl {}
impl _wxEvtHandler for wxListCtrl {}
impl _wxObject for wxListCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxListCtrl {
    pub fn from(ptr: *mut c_void) -> wxListCtrl { wxListCtrl { ptr: ptr } }
    pub fn null() -> wxListCtrl { wxListCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxListCtrl {
        unsafe { wxListCtrl { ptr: wxListCtrl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxListCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn arrange(&self, flag: c_int) -> c_int {
        unsafe { wxListCtrl_Arrange(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllColumns(&self) -> c_int {
        unsafe { wxListCtrl_DeleteAllColumns(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllItems(&self) -> c_int {
        unsafe { wxListCtrl_DeleteAllItems(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteColumn(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_DeleteColumn(self.ptr(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteItem(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_DeleteItem(self.ptr(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn editLabel(&self, item: c_int) {
        unsafe { wxListCtrl_EditLabel(self.ptr(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endEditLabel(&self, cancel: c_int) -> c_int {
        unsafe { wxListCtrl_EndEditLabel(self.ptr(), cancel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisible(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_EnsureVisible(self.ptr(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, start: c_int, str: &str, partial: c_int) -> c_int {
        let str = wxT(str);
        unsafe { wxListCtrl_FindItem(self.ptr(), start, str.ptr(), partial) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItemByData(&self, start: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByData(self.ptr(), start, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItemByPosition(&self, start: c_int, x: c_int, y: c_int, direction: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByPosition(self.ptr(), start, x, y, direction) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn<T: _wxListItem>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_GetColumn(self.ptr(), col, item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumnCount(&self) -> c_int {
        unsafe { wxListCtrl_GetColumnCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumnWidth(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_GetColumnWidth(self.ptr(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCountPerPage(&self) -> c_int {
        unsafe { wxListCtrl_GetCountPerPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEditControl(&self) -> wxTextCtrl {
        unsafe { wxTextCtrl { ptr: wxListCtrl_GetEditControl(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageList(&self, which: c_int) -> wxImageList {
        unsafe { wxImageList { ptr: wxListCtrl_GetImageList(self.ptr(), which) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem<T: _wxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_GetItem(self.ptr(), info.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetItemCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemData(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemData(self.ptr(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemFont(&self, item: c_long) -> wxFont {
        unsafe { wxFont { ptr: wxListCtrl_GetItemFont(self.ptr(), item) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemPosition(&self, item: c_int) -> wxPoint {
        unsafe { wxPoint { ptr: wxListCtrl_GetItemPosition(self.ptr(), item) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemRect(&self, item: c_int, code: c_int) -> wxRect {
        unsafe { wxRect { ptr: wxListCtrl_GetItemRect(self.ptr(), item, code) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemSpacing(&self, isSmall: c_int) -> wxSize {
        unsafe { wxSize { ptr: wxListCtrl_GetItemSpacing(self.ptr(), isSmall) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.ptr(), item, stateMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemText(&self, item: c_int) -> ~str {
        unsafe { wxString { ptr: wxListCtrl_GetItemText(self.ptr(), item) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextItem(&self, item: c_int, geometry: c_int, state: c_int) -> c_int {
        unsafe { wxListCtrl_GetNextItem(self.ptr(), item, geometry, state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectedItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetSelectedItemCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxListCtrl_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTopItem(&self) -> c_int {
        unsafe { wxListCtrl_GetTopItem(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_void) -> c_int {
        unsafe { wxListCtrl_HitTest(self.ptr(), x, y, flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertColumn(&self, col: c_int, heading: &str, format: c_int, width: c_int) -> c_int {
        let heading = wxT(heading);
        unsafe { wxListCtrl_InsertColumn(self.ptr(), col, heading.ptr(), format, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertColumnFromInfo<T: _wxListItem>(&self, col: c_int, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.ptr(), col, info.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.ptr(), info.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemWithData(&self, index: c_int, label: &str) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_InsertItemWithData(self.ptr(), index, label.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.ptr(), index, imageIndex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemWithLabel(&self, index: c_int, label: &str, imageIndex: c_int) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_InsertItemWithLabel(self.ptr(), index, label.ptr(), imageIndex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVirtual(&self) -> c_int {
        unsafe { wxListCtrl_IsVirtual(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refreshItem(&self, item: c_long) {
        unsafe { wxListCtrl_RefreshItem(self.ptr(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollList(&self, dx: c_int, dy: c_int) -> c_int {
        unsafe { wxListCtrl_ScrollList(self.ptr(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumn<T: _wxListItem>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_SetColumn(self.ptr(), col, item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_SetColumnWidth(self.ptr(), col, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.ptr(), imageList.ptr(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItem(&self, index: c_int, col: c_int, label: &str, imageId: c_int) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_SetItem(self.ptr(), index, col, label.ptr(), imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemData(&self, item: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemData(self.ptr(), item, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemFromInfo<T: _wxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_SetItemFromInfo(self.ptr(), info.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemImage(&self, item: c_int, image: c_int, selImage: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemImage(self.ptr(), item, image, selImage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemPosition(&self, item: c_int, x: c_int, y: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemPosition(self.ptr(), item, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemState(&self, item: c_int, state: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemState(self.ptr(), item, state, stateMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemText(&self, item: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxListCtrl_SetItemText(self.ptr(), item, str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSingleStyle(&self, style: c_int, add: c_int) {
        unsafe { wxListCtrl_SetSingleStyle(self.ptr(), style, add) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxListCtrl_SetTextColour(self.ptr(), col.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortItems(&self, fn_: *mut c_void, eif_obj: *mut c_void) -> c_int {
        unsafe { wxListCtrl_SortItems(self.ptr(), fn_, eif_obj) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignImageList<T: _wxImageList>(&self, images: &T, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.ptr(), images.ptr(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn2<T: _wxListItem>(&self, col: c_int, item: &T) {
        unsafe { wxListCtrl_GetColumn2(self.ptr(), col, item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem2<T: _wxListItem>(&self, info: &T) {
        unsafe { wxListCtrl_GetItem2(self.ptr(), info.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemPosition2(&self, item: c_int) -> wxPoint {
        unsafe { wxPoint { ptr: wxListCtrl_GetItemPosition2(self.ptr(), item) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortItems2<T: _wxClosure>(&self, closure: &T) -> c_int {
        unsafe { wxListCtrl_SortItems2(self.ptr(), closure.ptr()) }
    }
}

pub struct wxListEvent { ptr: *mut c_void }
impl _wxListEvent for wxListEvent {}
impl _wxNotifyEvent for wxListEvent {}
impl _wxCommandEvent for wxListEvent {}
impl _wxEvent for wxListEvent {}
impl _wxObject for wxListEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxListEvent {
    pub fn from(ptr: *mut c_void) -> wxListEvent { wxListEvent { ptr: ptr } }
    pub fn null() -> wxListEvent { wxListEvent::from(0 as *mut c_void) }
    
}

pub trait _wxListEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn cancelled(&self) -> c_int {
        unsafe { wxListEvent_Cancelled(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCode(&self) -> c_int {
        unsafe { wxListEvent_GetCode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn(&self) -> c_int {
        unsafe { wxListEvent_GetColumn(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> c_int {
        unsafe { wxListEvent_GetData(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImage(&self) -> c_int {
        unsafe { wxListEvent_GetImage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndex(&self) -> c_int {
        unsafe { wxListEvent_GetIndex(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem<T: _wxListItem>(&self, _ref: &T) {
        unsafe { wxListEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { ptr: wxListEvent_GetLabel(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPoint(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxListEvent_GetPoint(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { ptr: wxListEvent_GetText(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCacheFrom(&self) -> c_int {
        unsafe { wxListEvent_GetCacheFrom(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCacheTo(&self) -> c_int {
        unsafe { wxListEvent_GetCacheTo(self.ptr()) }
    }
}

pub struct wxListItem { ptr: *mut c_void }
impl _wxListItem for wxListItem {}
impl _wxObject for wxListItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxListItem {
    pub fn from(ptr: *mut c_void) -> wxListItem { wxListItem { ptr: ptr } }
    pub fn null() -> wxListItem { wxListItem::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxListItem {
        unsafe { wxListItem { ptr: wxListItem_Create() } }
    }
}

pub trait _wxListItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxListItem_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearAttributes(&self) {
        unsafe { wxListItem_ClearAttributes(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlign(&self) -> c_int {
        unsafe { wxListItem_GetAlign(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAttributes(&self) -> *mut c_void {
        unsafe { wxListItem_GetAttributes(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxListItem_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn(&self) -> c_int {
        unsafe { wxListItem_GetColumn(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> c_int {
        unsafe { wxListItem_GetData(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxListItem_GetFont(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxListItem_GetId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImage(&self) -> c_int {
        unsafe { wxListItem_GetImage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMask(&self) -> c_int {
        unsafe { wxListItem_GetMask(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getState(&self) -> c_int {
        unsafe { wxListItem_GetState(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { ptr: wxListItem_GetText(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxListItem_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxListItem_GetWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasAttributes(&self) -> c_int {
        unsafe { wxListItem_HasAttributes(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAlign(&self, align: c_int) {
        unsafe { wxListItem_SetAlign(self.ptr(), align) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colBack: &T) {
        unsafe { wxListItem_SetBackgroundColour(self.ptr(), colBack.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumn(&self, col: c_int) {
        unsafe { wxListItem_SetColumn(self.ptr(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData(&self, data: c_int) {
        unsafe { wxListItem_SetData(self.ptr(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataPointer(&self, data: *mut c_void) {
        unsafe { wxListItem_SetDataPointer(self.ptr(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxListItem_SetFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: c_int) {
        unsafe { wxListItem_SetId(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImage(&self, image: c_int) {
        unsafe { wxListItem_SetImage(self.ptr(), image) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMask(&self, mask: c_int) {
        unsafe { wxListItem_SetMask(self.ptr(), mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setState(&self, state: c_int) {
        unsafe { wxListItem_SetState(self.ptr(), state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStateMask(&self, stateMask: c_int) {
        unsafe { wxListItem_SetStateMask(self.ptr(), stateMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxListItem_SetText(self.ptr(), text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, colText: &T) {
        unsafe { wxListItem_SetTextColour(self.ptr(), colText.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.ptr(), width) }
    }
}

pub struct wxLog { ptr: *mut c_void }
impl _wxLog for wxLog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLog {
    pub fn from(ptr: *mut c_void) -> wxLog { wxLog { ptr: ptr } }
    pub fn null() -> wxLog { wxLog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getActiveTarget() -> wxLog {
        unsafe { wxLog { ptr: wxLog_GetActiveTarget() } }
    }
}

pub trait _wxLog {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTraceMask(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxLog_AddTraceMask(self.ptr(), str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxLog_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn dontCreateOnDemand(&self) {
        unsafe { wxLog_DontCreateOnDemand(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flush(&self) {
        unsafe { wxLog_Flush(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flushActive(&self) {
        unsafe { wxLog_FlushActive(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTimestamp(&self) -> *mut c_char {
        unsafe { wxLog_GetTimestamp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTraceMask(&self) -> c_int {
        unsafe { wxLog_GetTraceMask(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVerbose(&self) -> c_int {
        unsafe { wxLog_GetVerbose(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasPendingMessages(&self) -> c_int {
        unsafe { wxLog_HasPendingMessages(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAllowedTraceMask<T: _wxMask>(&self, mask: &T) -> c_int {
        unsafe { wxLog_IsAllowedTraceMask(self.ptr(), mask.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onLog(&self, level: c_int, szString: *mut c_void, t: c_int) {
        unsafe { wxLog_OnLog(self.ptr(), level, szString, t) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeTraceMask(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxLog_RemoveTraceMask(self.ptr(), str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxLog_Resume(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setActiveTarget(&self) -> wxLog {
        unsafe { wxLog { ptr: wxLog_SetActiveTarget(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTimestamp(&self, ts: *mut c_void) {
        unsafe { wxLog_SetTimestamp(self.ptr(), ts) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTraceMask(&self, ulMask: c_int) {
        unsafe { wxLog_SetTraceMask(self.ptr(), ulMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVerbose(&self, bVerbose: c_int) {
        unsafe { wxLog_SetVerbose(self.ptr(), bVerbose) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn suspend(&self) {
        unsafe { wxLog_Suspend(self.ptr()) }
    }
}

pub struct wxLogChain { ptr: *mut c_void }
impl _wxLogChain for wxLogChain {}
impl _wxLog for wxLogChain { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogChain {
    pub fn from(ptr: *mut c_void) -> wxLogChain { wxLogChain { ptr: ptr } }
    pub fn null() -> wxLogChain { wxLogChain::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxLog>(logger: &T) -> wxLogChain {
        unsafe { wxLogChain { ptr: wxLogChain_Create(logger.ptr()) } }
    }
}

pub trait _wxLogChain : _wxLog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOldLog(&self) -> wxLog {
        unsafe { wxLog { ptr: wxLogChain_GetOldLog(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isPassingMessages(&self) -> c_int {
        unsafe { wxLogChain_IsPassingMessages(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn passMessages(&self, bDoPass: c_int) {
        unsafe { wxLogChain_PassMessages(self.ptr(), bDoPass) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLog<T: _wxLog>(&self, logger: &T) {
        unsafe { wxLogChain_SetLog(self.ptr(), logger.ptr()) }
    }
}

pub struct wxLogGUI { ptr: *mut c_void }
impl _wxLogGUI for wxLogGUI {}
impl _wxLog for wxLogGUI { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogGUI {
    pub fn from(ptr: *mut c_void) -> wxLogGUI { wxLogGUI { ptr: ptr } }
    pub fn null() -> wxLogGUI { wxLogGUI::from(0 as *mut c_void) }
    
}

pub trait _wxLogGUI : _wxLog {
}

pub struct wxLogNull { ptr: *mut c_void }
impl _wxLogNull for wxLogNull {}
impl _wxLog for wxLogNull { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogNull {
    pub fn from(ptr: *mut c_void) -> wxLogNull { wxLogNull { ptr: ptr } }
    pub fn null() -> wxLogNull { wxLogNull::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxLogNull {
        unsafe { wxLogNull { ptr: wxLogNull_Create() } }
    }
}

pub trait _wxLogNull : _wxLog {
}

pub struct wxLogPassThrough { ptr: *mut c_void }
impl _wxLogPassThrough for wxLogPassThrough {}
impl _wxLogChain for wxLogPassThrough {}
impl _wxLog for wxLogPassThrough { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogPassThrough {
    pub fn from(ptr: *mut c_void) -> wxLogPassThrough { wxLogPassThrough { ptr: ptr } }
    pub fn null() -> wxLogPassThrough { wxLogPassThrough::from(0 as *mut c_void) }
    
}

pub trait _wxLogPassThrough : _wxLogChain {
}

pub struct wxLogStderr { ptr: *mut c_void }
impl _wxLogStderr for wxLogStderr {}
impl _wxLog for wxLogStderr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogStderr {
    pub fn from(ptr: *mut c_void) -> wxLogStderr { wxLogStderr { ptr: ptr } }
    pub fn null() -> wxLogStderr { wxLogStderr::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxLogStderr {
        unsafe { wxLogStderr { ptr: wxLogStderr_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newStdOut() -> wxLogStderr {
        unsafe { wxLogStderr { ptr: wxLogStderr_CreateStdOut() } }
    }
}

pub trait _wxLogStderr : _wxLog {
}

pub struct wxLogStream { ptr: *mut c_void }
impl _wxLogStream for wxLogStream {}
impl _wxLog for wxLogStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogStream {
    pub fn from(ptr: *mut c_void) -> wxLogStream { wxLogStream { ptr: ptr } }
    pub fn null() -> wxLogStream { wxLogStream::from(0 as *mut c_void) }
    
}

pub trait _wxLogStream : _wxLog {
}

pub struct wxLogTextCtrl { ptr: *mut c_void }
impl _wxLogTextCtrl for wxLogTextCtrl {}
impl _wxLog for wxLogTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogTextCtrl {
    pub fn from(ptr: *mut c_void) -> wxLogTextCtrl { wxLogTextCtrl { ptr: ptr } }
    pub fn null() -> wxLogTextCtrl { wxLogTextCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxTextCtrl>(text: &T) -> wxLogTextCtrl {
        unsafe { wxLogTextCtrl { ptr: wxLogTextCtrl_Create(text.ptr()) } }
    }
}

pub trait _wxLogTextCtrl : _wxLog {
}

pub struct wxLogWindow { ptr: *mut c_void }
impl _wxLogWindow for wxLogWindow {}
impl _wxLogPassThrough for wxLogWindow {}
impl _wxLogChain for wxLogWindow {}
impl _wxLog for wxLogWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLogWindow {
    pub fn from(ptr: *mut c_void) -> wxLogWindow { wxLogWindow { ptr: ptr } }
    pub fn null() -> wxLogWindow { wxLogWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, title: *mut int8_t, showit: c_int, passthrough: c_int) -> wxLogWindow {
        unsafe { wxLogWindow { ptr: wxLogWindow_Create(parent.ptr(), title, showit, passthrough) } }
    }
}

pub trait _wxLogWindow : _wxLogPassThrough {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame { ptr: wxLogWindow_GetFrame(self.ptr()) } }
    }
}

pub struct wxMDIChildFrame { ptr: *mut c_void }
impl _wxMDIChildFrame for wxMDIChildFrame {}
impl _wxFrame for wxMDIChildFrame {}
impl _wxTopLevelWindow for wxMDIChildFrame {}
impl _wxWindow for wxMDIChildFrame {}
impl _wxEvtHandler for wxMDIChildFrame {}
impl _wxObject for wxMDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMDIChildFrame {
    pub fn from(ptr: *mut c_void) -> wxMDIChildFrame { wxMDIChildFrame { ptr: ptr } }
    pub fn null() -> wxMDIChildFrame { wxMDIChildFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxMDIChildFrame {
        let _txt = wxT(_txt);
        unsafe { wxMDIChildFrame { ptr: wxMDIChildFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxMDIChildFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.ptr()) }
    }
}

pub struct wxMDIClientWindow { ptr: *mut c_void }
impl _wxMDIClientWindow for wxMDIClientWindow {}
impl _wxWindow for wxMDIClientWindow {}
impl _wxEvtHandler for wxMDIClientWindow {}
impl _wxObject for wxMDIClientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMDIClientWindow {
    pub fn from(ptr: *mut c_void) -> wxMDIClientWindow { wxMDIClientWindow { ptr: ptr } }
    pub fn null() -> wxMDIClientWindow { wxMDIClientWindow::from(0 as *mut c_void) }
    
}

pub trait _wxMDIClientWindow : _wxWindow {
}

pub struct wxMDIParentFrame { ptr: *mut c_void }
impl _wxMDIParentFrame for wxMDIParentFrame {}
impl _wxFrame for wxMDIParentFrame {}
impl _wxTopLevelWindow for wxMDIParentFrame {}
impl _wxWindow for wxMDIParentFrame {}
impl _wxEvtHandler for wxMDIParentFrame {}
impl _wxObject for wxMDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMDIParentFrame {
    pub fn from(ptr: *mut c_void) -> wxMDIParentFrame { wxMDIParentFrame { ptr: ptr } }
    pub fn null() -> wxMDIParentFrame { wxMDIParentFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxMDIParentFrame {
        let _txt = wxT(_txt);
        unsafe { wxMDIParentFrame { ptr: wxMDIParentFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxMDIParentFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn activateNext(&self) {
        unsafe { wxMDIParentFrame_ActivateNext(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn activatePrevious(&self) {
        unsafe { wxMDIParentFrame_ActivatePrevious(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn arrangeIcons(&self) {
        unsafe { wxMDIParentFrame_ArrangeIcons(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cascade(&self) {
        unsafe { wxMDIParentFrame_Cascade(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getActiveChild(&self) -> wxMDIChildFrame {
        unsafe { wxMDIChildFrame { ptr: wxMDIParentFrame_GetActiveChild(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientWindow(&self) -> wxMDIClientWindow {
        unsafe { wxMDIClientWindow { ptr: wxMDIParentFrame_GetClientWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindowMenu(&self) -> wxMenu {
        unsafe { wxMenu { ptr: wxMDIParentFrame_GetWindowMenu(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onCreateClient(&self) -> wxMDIClientWindow {
        unsafe { wxMDIClientWindow { ptr: wxMDIParentFrame_OnCreateClient(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.ptr(), menu.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.ptr()) }
    }
}

pub struct wxMask { ptr: *mut c_void }
impl _wxMask for wxMask {}
impl _wxObject for wxMask { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMask {
    pub fn from(ptr: *mut c_void) -> wxMask { wxMask { ptr: ptr } }
    pub fn null() -> wxMask { wxMask::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxBitmap>(bitmap: &T) -> wxMask {
        unsafe { wxMask { ptr: wxMask_Create(bitmap.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newColoured<T: _wxBitmap, U: _wxColour>(bitmap: &T, colour: &U) -> *mut c_void {
        unsafe { wxMask_CreateColoured(bitmap.ptr(), colour.ptr()) }
    }
}

pub trait _wxMask : _wxObject {
}

pub struct wxMaximizeEvent { ptr: *mut c_void }
impl _wxMaximizeEvent for wxMaximizeEvent {}
impl _wxEvent for wxMaximizeEvent {}
impl _wxObject for wxMaximizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMaximizeEvent {
    pub fn from(ptr: *mut c_void) -> wxMaximizeEvent { wxMaximizeEvent { ptr: ptr } }
    pub fn null() -> wxMaximizeEvent { wxMaximizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMaximizeEvent : _wxEvent {
}

pub struct wxMemoryDC { ptr: *mut c_void }
impl _wxMemoryDC for wxMemoryDC {}
impl _wxDC for wxMemoryDC {}
impl _wxObject for wxMemoryDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMemoryDC {
    pub fn from(ptr: *mut c_void) -> wxMemoryDC { wxMemoryDC { ptr: ptr } }
    pub fn null() -> wxMemoryDC { wxMemoryDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxMemoryDC {
        unsafe { wxMemoryDC { ptr: wxMemoryDC_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newCompatible<T: _wxDC>(dc: &T) -> wxMemoryDC {
        unsafe { wxMemoryDC { ptr: wxMemoryDC_CreateCompatible(dc.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithBitmap<T: _wxBitmap>(bitmap: &T) -> wxMemoryDC {
        unsafe { wxMemoryDC { ptr: wxMemoryDC_CreateWithBitmap(bitmap.ptr()) } }
    }
}

pub trait _wxMemoryDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectObject<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxMemoryDC_SelectObject(self.ptr(), bitmap.ptr()) }
    }
}

pub struct wxMenu { ptr: *mut c_void }
impl _wxMenu for wxMenu {}
impl _wxEvtHandler for wxMenu {}
impl _wxObject for wxMenu { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMenu {
    pub fn from(ptr: *mut c_void) -> wxMenu { wxMenu { ptr: ptr } }
    pub fn null() -> wxMenu { wxMenu::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(title: &str, style: c_long) -> wxMenu {
        let title = wxT(title);
        unsafe { wxMenu { ptr: wxMenu_Create(title.ptr(), style) } }
    }
}

pub trait _wxMenu : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Append(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_AppendItem(self.ptr(), _itm.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendSub<T: _wxMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_AppendSub(self.ptr(), id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn break_(&self) {
        unsafe { wxMenu_Break(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, id: c_int, check: c_int) {
        unsafe { wxMenu_Check(self.ptr(), id, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteById(&self, id: c_int) {
        unsafe { wxMenu_DeleteById(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteByItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DeleteByItem(self.ptr(), _itm.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyById(&self, id: c_int) {
        unsafe { wxMenu_DestroyById(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyByItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DestroyByItem(self.ptr(), _itm.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, id: c_int, enable: c_int) {
        unsafe { wxMenu_Enable(self.ptr(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, id: c_int) -> wxMenuItem {
        unsafe { wxMenuItem { ptr: wxMenu_FindItem(self.ptr(), id) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItemByLabel(&self, itemString: &str) -> c_int {
        let itemString = wxT(itemString);
        unsafe { wxMenu_FindItemByLabel(self.ptr(), itemString.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientData(&self) -> wxClientData {
        unsafe { wxClientData { ptr: wxMenu_GetClientData(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { wxString { ptr: wxMenu_GetHelpString(self.ptr(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInvokingWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxMenu_GetInvokingWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { wxString { ptr: wxMenu_GetLabel(self.ptr(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuItems<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.ptr(), _lst.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getParent(&self) -> wxMenu {
        unsafe { wxMenu { ptr: wxMenu_GetParent(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { ptr: wxMenu_GetTitle(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insert(&self, pos: size_t, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Insert(self.ptr(), pos, id, text.ptr(), help.ptr(), isCheckable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxMenuItem>(&self, pos: size_t, _itm: &T) {
        unsafe { wxMenu_InsertItem(self.ptr(), pos, _itm.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSub<T: _wxMenu>(&self, pos: size_t, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_InsertSub(self.ptr(), pos, id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAttached(&self) -> c_int {
        unsafe { wxMenu_IsAttached(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenu_IsChecked(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenu_IsEnabled(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepend(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Prepend(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_PrependItem(self.ptr(), _itm.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSub<T: _wxMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_PrependSub(self.ptr(), id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeById<T: _wxMenuItem>(&self, id: c_int, _itm: &T) {
        unsafe { wxMenu_RemoveById(self.ptr(), id, _itm.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeByItem(&self, item: *mut c_void) {
        unsafe { wxMenu_RemoveByItem(self.ptr(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientData<T: _wxClientData>(&self, clientData: &T) {
        unsafe { wxMenu_SetClientData(self.ptr(), clientData.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEventHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxMenu_SetEventHandler(self.ptr(), handler.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenu_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInvokingWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxMenu_SetInvokingWindow(self.ptr(), win.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenu_SetLabel(self.ptr(), id, label.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setParent<T: _wxWindow>(&self, parent: &T) {
        unsafe { wxMenu_SetParent(self.ptr(), parent.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTitle(&self, title: &str) {
        let title = wxT(title);
        unsafe { wxMenu_SetTitle(self.ptr(), title.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateUI(&self, source: *mut c_void) {
        unsafe { wxMenu_UpdateUI(self.ptr(), source) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuBar(&self) -> wxMenuBar {
        unsafe { wxMenuBar { ptr: wxMenu_GetMenuBar(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendRadioItem(&self, id: c_int, text: &str, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_AppendRadioItem(self.ptr(), id, text.ptr(), help.ptr()) }
    }
}

pub struct wxMenuBar { ptr: *mut c_void }
impl _wxMenuBar for wxMenuBar {}
impl _wxEvtHandler for wxMenuBar {}
impl _wxObject for wxMenuBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMenuBar {
    pub fn from(ptr: *mut c_void) -> wxMenuBar { wxMenuBar { ptr: ptr } }
    pub fn null() -> wxMenuBar { wxMenuBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_style: c_int) -> wxMenuBar {
        unsafe { wxMenuBar { ptr: wxMenuBar_Create(_style) } }
    }
}

pub trait _wxMenuBar : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append<T: _wxMenu>(&self, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Append(self.ptr(), menu.ptr(), title.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, id: c_int, check: c_int) {
        unsafe { wxMenuBar_Check(self.ptr(), id, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePointer(&self) {
        unsafe { wxMenuBar_DeletePointer(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: c_int) -> c_int {
        unsafe { wxMenuBar_Enable(self.ptr(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableItem(&self, id: c_int, enable: c_int) {
        unsafe { wxMenuBar_EnableItem(self.ptr(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableTop(&self, pos: c_int, enable: c_int) {
        unsafe { wxMenuBar_EnableTop(self.ptr(), pos, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, id: c_int) -> wxMenuItem {
        unsafe { wxMenuItem { ptr: wxMenuBar_FindItem(self.ptr(), id) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findMenu(&self, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_FindMenu(self.ptr(), title.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findMenuItem(&self, menuString: &str, itemString: &str) -> c_int {
        let menuString = wxT(menuString);
        let itemString = wxT(itemString);
        unsafe { wxMenuBar_FindMenuItem(self.ptr(), menuString.ptr(), itemString.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { wxString { ptr: wxMenuBar_GetHelpString(self.ptr(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { wxString { ptr: wxMenuBar_GetLabel(self.ptr(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelTop(&self, pos: c_int) -> ~str {
        unsafe { wxString { ptr: wxMenuBar_GetLabelTop(self.ptr(), pos) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenu(&self, pos: c_int) -> wxMenu {
        unsafe { wxMenu { ptr: wxMenuBar_GetMenu(self.ptr(), pos) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insert<T: _wxMenu>(&self, pos: c_int, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Insert(self.ptr(), pos, menu.ptr(), title.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsChecked(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsEnabled(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, pos: c_int) -> wxMenu {
        unsafe { wxMenu { ptr: wxMenuBar_Remove(self.ptr(), pos) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxMenu>(&self, pos: c_int, menu: &T, title: &str) -> wxMenu {
        let title = wxT(title);
        unsafe { wxMenu { ptr: wxMenuBar_Replace(self.ptr(), pos, menu.ptr(), title.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenuBar_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenuBar_SetItemLabel(self.ptr(), id, label.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabel(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxMenuBar_SetLabel(self.ptr(), s.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabelTop(&self, pos: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenuBar_SetLabelTop(self.ptr(), pos, label.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame { ptr: wxMenuBar_GetFrame(self.ptr()) } }
    }
}

pub struct wxMenuEvent { ptr: *mut c_void }
impl _wxMenuEvent for wxMenuEvent {}
impl _wxEvent for wxMenuEvent {}
impl _wxObject for wxMenuEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMenuEvent {
    pub fn from(ptr: *mut c_void) -> wxMenuEvent { wxMenuEvent { ptr: ptr } }
    pub fn null() -> wxMenuEvent { wxMenuEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMenuEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.ptr()) }
    }
}

pub struct wxMenuItem { ptr: *mut c_void }
impl _wxMenuItem for wxMenuItem {}
impl _wxObject for wxMenuItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMenuItem {
    pub fn from(ptr: *mut c_void) -> wxMenuItem { wxMenuItem { ptr: ptr } }
    pub fn null() -> wxMenuItem { wxMenuItem::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxMenuItem {
        unsafe { wxMenuItem { ptr: wxMenuItem_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getLabelFromText(text: *mut c_void) -> ~str {
        unsafe { wxString { ptr: wxMenuItem_GetLabelFromText(text) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newSeparator() -> wxMenuItem {
        unsafe { wxMenuItem { ptr: wxMenuItem_CreateSeparator() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEx<T: _wxMenu>(id: c_int, label: &str, help: &str, itemkind: c_int, submenu: &T) -> wxMenuItem {
        let label = wxT(label);
        let help = wxT(help);
        unsafe { wxMenuItem { ptr: wxMenuItem_CreateEx(id, label.ptr(), help.ptr(), itemkind, submenu.ptr()) } }
    }
}

pub trait _wxMenuItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, check: c_int) {
        unsafe { wxMenuItem_Check(self.ptr(), check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: c_int) {
        unsafe { wxMenuItem_Enable(self.ptr(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelp(&self) -> ~str {
        unsafe { wxString { ptr: wxMenuItem_GetHelp(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { ptr: wxMenuItem_GetLabel(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenu(&self) -> wxMenu {
        unsafe { wxMenu { ptr: wxMenuItem_GetMenu(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSubMenu(&self) -> wxMenu {
        unsafe { wxMenu { ptr: wxMenuItem_GetSubMenu(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { ptr: wxMenuItem_GetText(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCheckable(&self) -> c_int {
        unsafe { wxMenuItem_IsCheckable(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self) -> c_int {
        unsafe { wxMenuItem_IsChecked(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> c_int {
        unsafe { wxMenuItem_IsEnabled(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSeparator(&self) -> c_int {
        unsafe { wxMenuItem_IsSeparator(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSubMenu(&self) -> c_int {
        unsafe { wxMenuItem_IsSubMenu(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCheckable(&self, checkable: c_int) {
        unsafe { wxMenuItem_SetCheckable(self.ptr(), checkable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelp(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxMenuItem_SetHelp(self.ptr(), str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSubMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxMenuItem_SetSubMenu(self.ptr(), menu.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxMenuItem_SetText(self.ptr(), str.ptr()) }
    }
}

pub struct wxMessageDialog { ptr: *mut c_void }
impl _wxMessageDialog for wxMessageDialog {}
impl _wxDialog for wxMessageDialog {}
impl _wxTopLevelWindow for wxMessageDialog {}
impl _wxWindow for wxMessageDialog {}
impl _wxEvtHandler for wxMessageDialog {}
impl _wxObject for wxMessageDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMessageDialog {
    pub fn from(ptr: *mut c_void) -> wxMessageDialog { wxMessageDialog { ptr: ptr } }
    pub fn null() -> wxMessageDialog { wxMessageDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _cap: &str, _stl: c_int) -> wxMessageDialog {
        let _msg = wxT(_msg);
        let _cap = wxT(_cap);
        unsafe { wxMessageDialog { ptr: wxMessageDialog_Create(_prt.ptr(), _msg.ptr(), _cap.ptr(), _stl) } }
    }
}

pub trait _wxMessageDialog : _wxDialog {
}

pub struct wxMetafile { ptr: *mut c_void }
impl _wxMetafile for wxMetafile {}
impl _wxObject for wxMetafile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMetafile {
    pub fn from(ptr: *mut c_void) -> wxMetafile { wxMetafile { ptr: ptr } }
    pub fn null() -> wxMetafile { wxMetafile::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_file: &str) -> wxMetafile {
        let _file = wxT(_file);
        unsafe { wxMetafile { ptr: wxMetafile_Create(_file.ptr()) } }
    }
}

pub trait _wxMetafile : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxMetafile_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play<T: _wxDC>(&self, _dc: &T) -> c_int {
        unsafe { wxMetafile_Play(self.ptr(), _dc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClipboard(&self, width: c_int, height: c_int) -> c_int {
        unsafe { wxMetafile_SetClipboard(self.ptr(), width, height) }
    }
}

pub struct wxMetafileDC { ptr: *mut c_void }
impl _wxMetafileDC for wxMetafileDC {}
impl _wxDC for wxMetafileDC {}
impl _wxObject for wxMetafileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMetafileDC {
    pub fn from(ptr: *mut c_void) -> wxMetafileDC { wxMetafileDC { ptr: ptr } }
    pub fn null() -> wxMetafileDC { wxMetafileDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_file: &str) -> wxMetafileDC {
        let _file = wxT(_file);
        unsafe { wxMetafileDC { ptr: wxMetafileDC_Create(_file.ptr()) } }
    }
}

pub trait _wxMetafileDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self) -> *mut c_void {
        unsafe { wxMetafileDC_Close(self.ptr()) }
    }
}

pub struct wxMimeTypesManager { ptr: *mut c_void }
impl _wxMimeTypesManager for wxMimeTypesManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMimeTypesManager {
    pub fn from(ptr: *mut c_void) -> wxMimeTypesManager { wxMimeTypesManager { ptr: ptr } }
    pub fn null() -> wxMimeTypesManager { wxMimeTypesManager::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxMimeTypesManager {
        unsafe { wxMimeTypesManager { ptr: wxMimeTypesManager_Create() } }
    }
}

pub trait _wxMimeTypesManager {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFallbacks(&self, _types: *mut c_void) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.ptr(), _types) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumAllFileTypes<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.ptr(), _lst.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFileTypeFromExtension(&self, _ext: &str) -> wxFileType {
        let _ext = wxT(_ext);
        unsafe { wxFileType { ptr: wxMimeTypesManager_GetFileTypeFromExtension(self.ptr(), _ext.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFileTypeFromMimeType(&self, _name: &str) -> wxFileType {
        let _name = wxT(_name);
        unsafe { wxFileType { ptr: wxMimeTypesManager_GetFileTypeFromMimeType(self.ptr(), _name.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOfType(&self, _type: &str, _wildcard: &str) -> c_int {
        let _type = wxT(_type);
        let _wildcard = wxT(_wildcard);
        unsafe { wxMimeTypesManager_IsOfType(self.ptr(), _type.ptr(), _wildcard.ptr()) }
    }
}

pub struct wxMiniFrame { ptr: *mut c_void }
impl _wxMiniFrame for wxMiniFrame {}
impl _wxFrame for wxMiniFrame {}
impl _wxTopLevelWindow for wxMiniFrame {}
impl _wxWindow for wxMiniFrame {}
impl _wxEvtHandler for wxMiniFrame {}
impl _wxObject for wxMiniFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMiniFrame {
    pub fn from(ptr: *mut c_void) -> wxMiniFrame { wxMiniFrame { ptr: ptr } }
    pub fn null() -> wxMiniFrame { wxMiniFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxMiniFrame {
        let _txt = wxT(_txt);
        unsafe { wxMiniFrame { ptr: wxMiniFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxMiniFrame : _wxFrame {
}

pub struct wxMirrorDC { ptr: *mut c_void }
impl _wxMirrorDC for wxMirrorDC {}
impl _wxDC for wxMirrorDC {}
impl _wxObject for wxMirrorDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMirrorDC {
    pub fn from(ptr: *mut c_void) -> wxMirrorDC { wxMirrorDC { ptr: ptr } }
    pub fn null() -> wxMirrorDC { wxMirrorDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxDC>(dc: &T) -> wxMirrorDC {
        unsafe { wxMirrorDC { ptr: wxMirrorDC_Create(dc.ptr()) } }
    }
}

pub trait _wxMirrorDC : _wxDC {
}

pub struct wxMouseCaptureChangedEvent { ptr: *mut c_void }
impl _wxMouseCaptureChangedEvent for wxMouseCaptureChangedEvent {}
impl _wxEvent for wxMouseCaptureChangedEvent {}
impl _wxObject for wxMouseCaptureChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMouseCaptureChangedEvent {
    pub fn from(ptr: *mut c_void) -> wxMouseCaptureChangedEvent { wxMouseCaptureChangedEvent { ptr: ptr } }
    pub fn null() -> wxMouseCaptureChangedEvent { wxMouseCaptureChangedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMouseCaptureChangedEvent : _wxEvent {
}

pub struct wxMouseEvent { ptr: *mut c_void }
impl _wxMouseEvent for wxMouseEvent {}
impl _wxEvent for wxMouseEvent {}
impl _wxObject for wxMouseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMouseEvent {
    pub fn from(ptr: *mut c_void) -> wxMouseEvent { wxMouseEvent { ptr: ptr } }
    pub fn null() -> wxMouseEvent { wxMouseEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMouseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> c_int {
        unsafe { wxMouseEvent_AltDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn button(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_Button(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDClick(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonDClick(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDown(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonDown(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonIsDown(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonIsDown(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonUp(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonUp(self.ptr(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> c_int {
        unsafe { wxMouseEvent_ControlDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn dragging(&self) -> c_int {
        unsafe { wxMouseEvent_Dragging(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn entering(&self) -> c_int {
        unsafe { wxMouseEvent_Entering(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalPosition<T: _wxDC>(&self, dc: &T) -> wxPoint {
        unsafe { wxPoint { ptr: wxMouseEvent_GetLogicalPosition(self.ptr(), dc.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxMouseEvent_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxMouseEvent_GetX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxMouseEvent_GetY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isButton(&self) -> c_int {
        unsafe { wxMouseEvent_IsButton(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leaving(&self) -> c_int {
        unsafe { wxMouseEvent_Leaving(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftDClick(&self) -> c_int {
        unsafe { wxMouseEvent_LeftDClick(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftDown(&self) -> c_int {
        unsafe { wxMouseEvent_LeftDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_LeftIsDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftUp(&self) -> c_int {
        unsafe { wxMouseEvent_LeftUp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> c_int {
        unsafe { wxMouseEvent_MetaDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleDClick(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleDClick(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleDown(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleIsDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleUp(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleUp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moving(&self) -> c_int {
        unsafe { wxMouseEvent_Moving(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightDClick(&self) -> c_int {
        unsafe { wxMouseEvent_RightDClick(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightDown(&self) -> c_int {
        unsafe { wxMouseEvent_RightDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_RightIsDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightUp(&self) -> c_int {
        unsafe { wxMouseEvent_RightUp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> c_int {
        unsafe { wxMouseEvent_ShiftDown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWheelDelta(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelDelta(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWheelRotation(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelRotation(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButton(&self) -> c_int {
        unsafe { wxMouseEvent_GetButton(self.ptr()) }
    }
}

pub struct wxMoveEvent { ptr: *mut c_void }
impl _wxMoveEvent for wxMoveEvent {}
impl _wxEvent for wxMoveEvent {}
impl _wxObject for wxMoveEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxMoveEvent {
    pub fn from(ptr: *mut c_void) -> wxMoveEvent { wxMoveEvent { ptr: ptr } }
    pub fn null() -> wxMoveEvent { wxMoveEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMoveEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxMoveEvent_GetPosition(self.ptr()) } }
    }
}

pub struct wxNavigationKeyEvent { ptr: *mut c_void }
impl _wxNavigationKeyEvent for wxNavigationKeyEvent {}
impl _wxEvent for wxNavigationKeyEvent {}
impl _wxObject for wxNavigationKeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxNavigationKeyEvent {
    pub fn from(ptr: *mut c_void) -> wxNavigationKeyEvent { wxNavigationKeyEvent { ptr: ptr } }
    pub fn null() -> wxNavigationKeyEvent { wxNavigationKeyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxNavigationKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentFocus(&self) -> *mut c_void {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirection(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_GetDirection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWindowChange(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentFocus<T: _wxWindow>(&self, win: &T) {
        unsafe { wxNavigationKeyEvent_SetCurrentFocus(self.ptr(), win.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDirection(&self, bForward: c_int) {
        unsafe { wxNavigationKeyEvent_SetDirection(self.ptr(), bForward) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowChange(&self, bIs: c_int) {
        unsafe { wxNavigationKeyEvent_SetWindowChange(self.ptr(), bIs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shouldPropagate(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_ShouldPropagate(self.ptr()) }
    }
}

pub struct wxNotebook { ptr: *mut c_void }
impl _wxNotebook for wxNotebook {}
impl _wxControl for wxNotebook {}
impl _wxWindow for wxNotebook {}
impl _wxEvtHandler for wxNotebook {}
impl _wxObject for wxNotebook { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxNotebook {
    pub fn from(ptr: *mut c_void) -> wxNotebook { wxNotebook { ptr: ptr } }
    pub fn null() -> wxNotebook { wxNotebook::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxNotebook {
        unsafe { wxNotebook { ptr: wxNotebook_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxNotebook : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPage<T: _wxWindow>(&self, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_AddPage(self.ptr(), pPage.ptr(), strText.ptr(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn advanceSelection(&self, bForward: c_int) {
        unsafe { wxNotebook_AdvanceSelection(self.ptr(), bForward) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllPages(&self) -> c_int {
        unsafe { wxNotebook_DeleteAllPages(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_DeletePage(self.ptr(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageList(&self) -> wxImageList {
        unsafe { wxImageList { ptr: wxNotebook_GetImageList(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPage(&self, nPage: c_int) -> wxWindow {
        unsafe { wxWindow { ptr: wxNotebook_GetPage(self.ptr(), nPage) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageCount(&self) -> c_int {
        unsafe { wxNotebook_GetPageCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageImage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_GetPageImage(self.ptr(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageText(&self, nPage: c_int) -> ~str {
        unsafe { wxString { ptr: wxNotebook_GetPageText(self.ptr(), nPage) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowCount(&self) -> c_int {
        unsafe { wxNotebook_GetRowCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxNotebook_GetSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_long) -> c_int {
        unsafe { wxNotebook_HitTest(self.ptr(), x, y, flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertPage<T: _wxWindow>(&self, nPage: c_int, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_InsertPage(self.ptr(), nPage, pPage.ptr(), strText.ptr(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_RemovePage(self.ptr(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxNotebook_SetImageList(self.ptr(), imageList.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPadding(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPadding(self.ptr(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageImage(&self, nPage: c_int, nImage: c_int) -> c_int {
        unsafe { wxNotebook_SetPageImage(self.ptr(), nPage, nImage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSize(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPageSize(self.ptr(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageText(&self, nPage: c_int, strText: &str) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_SetPageText(self.ptr(), nPage, strText.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.ptr(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxNotebook_AssignImageList(self.ptr(), imageList.ptr()) }
    }
}

pub struct wxNotebookEvent { ptr: *mut c_void }
impl _wxNotebookEvent for wxNotebookEvent {}
impl _wxNotifyEvent for wxNotebookEvent {}
impl _wxCommandEvent for wxNotebookEvent {}
impl _wxEvent for wxNotebookEvent {}
impl _wxObject for wxNotebookEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxNotebookEvent {
    pub fn from(ptr: *mut c_void) -> wxNotebookEvent { wxNotebookEvent { ptr: ptr } }
    pub fn null() -> wxNotebookEvent { wxNotebookEvent::from(0 as *mut c_void) }
    
}

pub trait _wxNotebookEvent : _wxNotifyEvent {
}

pub struct wxNotifyEvent { ptr: *mut c_void }
impl _wxNotifyEvent for wxNotifyEvent {}
impl _wxCommandEvent for wxNotifyEvent {}
impl _wxEvent for wxNotifyEvent {}
impl _wxObject for wxNotifyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxNotifyEvent {
    pub fn from(ptr: *mut c_void) -> wxNotifyEvent { wxNotifyEvent { ptr: ptr } }
    pub fn null() -> wxNotifyEvent { wxNotifyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxNotifyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn allow(&self) {
        unsafe { wxNotifyEvent_Allow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAllowed(&self) -> c_int {
        unsafe { wxNotifyEvent_IsAllowed(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn veto(&self) {
        unsafe { wxNotifyEvent_Veto(self.ptr()) }
    }
}

pub struct wxPageSetupDialog { ptr: *mut c_void }
impl _wxPageSetupDialog for wxPageSetupDialog {}
impl _wxDialog for wxPageSetupDialog {}
impl _wxTopLevelWindow for wxPageSetupDialog {}
impl _wxWindow for wxPageSetupDialog {}
impl _wxEvtHandler for wxPageSetupDialog {}
impl _wxObject for wxPageSetupDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPageSetupDialog {
    pub fn from(ptr: *mut c_void) -> wxPageSetupDialog { wxPageSetupDialog { ptr: ptr } }
    pub fn null() -> wxPageSetupDialog { wxPageSetupDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPageSetupDialogData>(parent: &T, data: &U) -> wxPageSetupDialog {
        unsafe { wxPageSetupDialog { ptr: wxPageSetupDialog_Create(parent.ptr(), data.ptr()) } }
    }
}

pub trait _wxPageSetupDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSetupData<T: _wxPageSetupDialogData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.ptr(), _ref.ptr()) }
    }
}

pub struct wxPageSetupDialogData { ptr: *mut c_void }
impl _wxPageSetupDialogData for wxPageSetupDialogData {}
impl _wxObject for wxPageSetupDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPageSetupDialogData {
    pub fn from(ptr: *mut c_void) -> wxPageSetupDialogData { wxPageSetupDialogData { ptr: ptr } }
    pub fn null() -> wxPageSetupDialogData { wxPageSetupDialogData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxPageSetupDialogData {
        unsafe { wxPageSetupDialogData { ptr: wxPageSetupDialogData_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData<T: _wxPrintData>(printData: &T) -> wxPageSetupDialogData {
        unsafe { wxPageSetupDialogData { ptr: wxPageSetupDialogData_CreateFromData(printData.ptr()) } }
    }
}

pub trait _wxPageSetupDialogData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPageSetupDialogData>(&self, data: &T) {
        unsafe { wxPageSetupDialogData_Assign(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignData<T: _wxPrintData>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_AssignData(self.ptr(), printData.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calculateIdFromPaperSize(&self) {
        unsafe { wxPageSetupDialogData_CalculateIdFromPaperSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calculatePaperSizeFromId(&self) {
        unsafe { wxPageSetupDialogData_CalculatePaperSizeFromId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableHelp(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableHelp(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableMargins(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableOrientation(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableOrientation(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePaper(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnablePaper(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePrinter(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnablePrinter(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultInfo(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetDefaultInfo(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultMinMargins(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetDefaultMinMargins(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableHelp(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableHelp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableMargins(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableMargins(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableOrientation(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableOrientation(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePaper(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnablePaper(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePrinter(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnablePrinter(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginBottomRight(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxPageSetupDialogData_GetMarginBottomRight(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginTopLeft(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxPageSetupDialogData_GetMarginTopLeft(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinMarginBottomRight(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxPageSetupDialogData_GetMinMarginBottomRight(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinMarginTopLeft(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxPageSetupDialogData_GetMinMarginTopLeft(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxPageSetupDialogData_GetPaperSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintData<T: _wxPrintData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialogData_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultInfo(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultInfo(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultMinMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultMinMargins(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginBottomRight(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginTopLeft(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginBottomRight(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginTopLeft(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperId(&self, id: *mut c_void) {
        unsafe { wxPageSetupDialogData_SetPaperId(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSize(self.ptr(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperSizeId(&self, id: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSizeId(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintData<T: _wxPrintData>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.ptr(), printData.ptr()) }
    }
}

pub struct wxPaintDC { ptr: *mut c_void }
impl _wxPaintDC for wxPaintDC {}
impl _wxWindowDC for wxPaintDC {}
impl _wxDC for wxPaintDC {}
impl _wxObject for wxPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPaintDC {
    pub fn from(ptr: *mut c_void) -> wxPaintDC { wxPaintDC { ptr: ptr } }
    pub fn null() -> wxPaintDC { wxPaintDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> wxPaintDC {
        unsafe { wxPaintDC { ptr: wxPaintDC_Create(win.ptr()) } }
    }
}

pub trait _wxPaintDC : _wxWindowDC {
}

pub struct wxPaintEvent { ptr: *mut c_void }
impl _wxPaintEvent for wxPaintEvent {}
impl _wxEvent for wxPaintEvent {}
impl _wxObject for wxPaintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPaintEvent {
    pub fn from(ptr: *mut c_void) -> wxPaintEvent { wxPaintEvent { ptr: ptr } }
    pub fn null() -> wxPaintEvent { wxPaintEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPaintEvent : _wxEvent {
}

pub struct wxPalette { ptr: *mut c_void }
impl _wxPalette for wxPalette {}
impl _wxGDIObject for wxPalette {}
impl _wxObject for wxPalette { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPalette {
    pub fn from(ptr: *mut c_void) -> wxPalette { wxPalette { ptr: ptr } }
    pub fn null() -> wxPalette { wxPalette::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxPalette {
        unsafe { wxPalette { ptr: wxPalette_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRGB(n: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> wxPalette {
        unsafe { wxPalette { ptr: wxPalette_CreateRGB(n, red, green, blue) } }
    }
}

pub trait _wxPalette : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPalette>(&self, palette: &T) {
        unsafe { wxPalette_Assign(self.ptr(), palette.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPixel(&self, red: uint8_t, green: uint8_t, blue: uint8_t) -> c_int {
        unsafe { wxPalette_GetPixel(self.ptr(), red, green, blue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRGB(&self, pixel: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> c_int {
        unsafe { wxPalette_GetRGB(self.ptr(), pixel, red, green, blue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxPalette>(&self, palette: &T) -> c_int {
        unsafe { wxPalette_IsEqual(self.ptr(), palette.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxPalette_IsOk(self.ptr()) }
    }
}

pub struct wxPaletteChangedEvent { ptr: *mut c_void }
impl _wxPaletteChangedEvent for wxPaletteChangedEvent {}
impl _wxEvent for wxPaletteChangedEvent {}
impl _wxObject for wxPaletteChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPaletteChangedEvent {
    pub fn from(ptr: *mut c_void) -> wxPaletteChangedEvent { wxPaletteChangedEvent { ptr: ptr } }
    pub fn null() -> wxPaletteChangedEvent { wxPaletteChangedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPaletteChangedEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChangedWindow(&self) -> *mut c_void {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChangedWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.ptr(), win.ptr()) }
    }
}

pub struct wxPanel { ptr: *mut c_void }
impl _wxPanel for wxPanel {}
impl _wxWindow for wxPanel {}
impl _wxEvtHandler for wxPanel {}
impl _wxObject for wxPanel { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPanel {
    pub fn from(ptr: *mut c_void) -> wxPanel { wxPanel { ptr: ptr } }
    pub fn null() -> wxPanel { wxPanel::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxPanel {
        unsafe { wxPanel { ptr: wxPanel_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxPanel : _wxWindow {
}

pub struct wxPen { ptr: *mut c_void }
impl _wxPen for wxPen {}
impl _wxGDIObject for wxPen {}
impl _wxObject for wxPen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPen {
    pub fn from(ptr: *mut c_void) -> wxPen { wxPen { ptr: ptr } }
    pub fn null() -> wxPen { wxPen::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxPen {
        unsafe { wxPen { ptr: wxPen_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(stipple: &T, width: c_int) -> wxPen {
        unsafe { wxPen { ptr: wxPen_CreateFromBitmap(stipple.ptr(), width) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromColour<T: _wxColour>(col: &T, width: c_int, style: c_int) -> wxPen {
        unsafe { wxPen { ptr: wxPen_CreateFromColour(col.ptr(), width, style) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> wxPen {
        unsafe { wxPen { ptr: wxPen_CreateFromStock(id) } }
    }
}

pub trait _wxPen : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPen>(&self, pen: &T) {
        unsafe { wxPen_Assign(self.ptr(), pen.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCap(&self) -> c_int {
        unsafe { wxPen_GetCap(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxPen_GetColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDashes(&self, ptr: *mut c_void) -> c_int {
        unsafe { wxPen_GetDashes(self.ptr(), ptr) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getJoin(&self) -> c_int {
        unsafe { wxPen_GetJoin(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStipple<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxPen_GetStipple(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxPen_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxPen_GetWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxPen>(&self, pen: &T) -> c_int {
        unsafe { wxPen_IsEqual(self.ptr(), pen.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxPen_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCap(&self, cap: c_int) {
        unsafe { wxPen_SetCap(self.ptr(), cap) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxPen_SetColour(self.ptr(), col.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxPen_SetColourSingle(self.ptr(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDashes(&self, nb_dashes: c_int, dash: *mut c_void) {
        unsafe { wxPen_SetDashes(self.ptr(), nb_dashes, dash) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setJoin(&self, join: c_int) {
        unsafe { wxPen_SetJoin(self.ptr(), join) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStipple<T: _wxBitmap>(&self, stipple: &T) {
        unsafe { wxPen_SetStipple(self.ptr(), stipple.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxPen_SetStyle(self.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, width: c_int) {
        unsafe { wxPen_SetWidth(self.ptr(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxPen_IsStatic(self.ptr()) }
    }
}

pub struct wxPenList { ptr: *mut c_void }
impl _wxPenList for wxPenList {}
impl _wxList for wxPenList {}
impl _wxObject for wxPenList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPenList {
    pub fn from(ptr: *mut c_void) -> wxPenList { wxPenList { ptr: ptr } }
    pub fn null() -> wxPenList { wxPenList::from(0 as *mut c_void) }
    
}

pub trait _wxPenList : _wxList {
}

pub struct wxPoint { ptr: *mut c_void }
impl _wxPoint for wxPoint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPoint {
    pub fn from(ptr: *mut c_void) -> wxPoint { wxPoint { ptr: ptr } }
    pub fn null() -> wxPoint { wxPoint::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(xx: c_int, yy: c_int) -> wxPoint {
        unsafe { wxPoint { ptr: wxPoint_Create(xx, yy) } }
    }
}

pub trait _wxPoint {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxPoint_GetX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxPoint_GetY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setX(&self, w: c_int) {
        unsafe { wxPoint_SetX(self.ptr(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setY(&self, h: c_int) {
        unsafe { wxPoint_SetY(self.ptr(), h) }
    }
}

pub struct wxPopupTransientWindow { ptr: *mut c_void }
impl _wxPopupTransientWindow for wxPopupTransientWindow {}
impl _wxPopupWindow for wxPopupTransientWindow {}
impl _wxWindow for wxPopupTransientWindow {}
impl _wxEvtHandler for wxPopupTransientWindow {}
impl _wxObject for wxPopupTransientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPopupTransientWindow {
    pub fn from(ptr: *mut c_void) -> wxPopupTransientWindow { wxPopupTransientWindow { ptr: ptr } }
    pub fn null() -> wxPopupTransientWindow { wxPopupTransientWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPopupTransientWindow : _wxPopupWindow {
}

pub struct wxPopupWindow { ptr: *mut c_void }
impl _wxPopupWindow for wxPopupWindow {}
impl _wxWindow for wxPopupWindow {}
impl _wxEvtHandler for wxPopupWindow {}
impl _wxObject for wxPopupWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPopupWindow {
    pub fn from(ptr: *mut c_void) -> wxPopupWindow { wxPopupWindow { ptr: ptr } }
    pub fn null() -> wxPopupWindow { wxPopupWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPopupWindow : _wxWindow {
}

pub struct wxPostScriptDC { ptr: *mut c_void }
impl _wxPostScriptDC for wxPostScriptDC {}
impl _wxDC for wxPostScriptDC {}
impl _wxObject for wxPostScriptDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPostScriptDC {
    pub fn from(ptr: *mut c_void) -> wxPostScriptDC { wxPostScriptDC { ptr: ptr } }
    pub fn null() -> wxPostScriptDC { wxPostScriptDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintData>(data: &T) -> wxPostScriptDC {
        unsafe { wxPostScriptDC { ptr: wxPostScriptDC_Create(data.ptr()) } }
    }
}

pub trait _wxPostScriptDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setResolution(&self, ppi: c_int) {
        unsafe { wxPostScriptDC_SetResolution(self.ptr(), ppi) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getResolution(&self) -> c_int {
        unsafe { wxPostScriptDC_GetResolution(self.ptr()) }
    }
}

pub struct wxPreviewCanvas { ptr: *mut c_void }
impl _wxPreviewCanvas for wxPreviewCanvas {}
impl _wxScrolledWindow for wxPreviewCanvas {}
impl _wxPanel for wxPreviewCanvas {}
impl _wxWindow for wxPreviewCanvas {}
impl _wxEvtHandler for wxPreviewCanvas {}
impl _wxObject for wxPreviewCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPreviewCanvas {
    pub fn from(ptr: *mut c_void) -> wxPreviewCanvas { wxPreviewCanvas { ptr: ptr } }
    pub fn null() -> wxPreviewCanvas { wxPreviewCanvas::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintPreview, U: _wxWindow>(preview: &T, parent: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> wxPreviewCanvas {
        unsafe { wxPreviewCanvas { ptr: wxPreviewCanvas_Create(preview.ptr(), parent.ptr(), x, y, w, h, style) } }
    }
}

pub trait _wxPreviewCanvas : _wxScrolledWindow {
}

pub struct wxPreviewControlBar { ptr: *mut c_void }
impl _wxPreviewControlBar for wxPreviewControlBar {}
impl _wxPanel for wxPreviewControlBar {}
impl _wxWindow for wxPreviewControlBar {}
impl _wxEvtHandler for wxPreviewControlBar {}
impl _wxObject for wxPreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPreviewControlBar {
    pub fn from(ptr: *mut c_void) -> wxPreviewControlBar { wxPreviewControlBar { ptr: ptr } }
    pub fn null() -> wxPreviewControlBar { wxPreviewControlBar::from(0 as *mut c_void) }
    
}

pub trait _wxPreviewControlBar : _wxPanel {
}

pub struct wxPreviewFrame { ptr: *mut c_void }
impl _wxPreviewFrame for wxPreviewFrame {}
impl _wxFrame for wxPreviewFrame {}
impl _wxTopLevelWindow for wxPreviewFrame {}
impl _wxWindow for wxPreviewFrame {}
impl _wxEvtHandler for wxPreviewFrame {}
impl _wxObject for wxPreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPreviewFrame {
    pub fn from(ptr: *mut c_void) -> wxPreviewFrame { wxPreviewFrame { ptr: ptr } }
    pub fn null() -> wxPreviewFrame { wxPreviewFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintPreview, U: _wxFrame>(preview: &T, parent: &U, title: &str, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: &str) -> wxPreviewFrame {
        let title = wxT(title);
        let name = wxT(name);
        unsafe { wxPreviewFrame { ptr: wxPreviewFrame_Create(preview.ptr(), parent.ptr(), title.ptr(), x, y, width, height, style, name.ptr()) } }
    }
}

pub trait _wxPreviewFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.ptr()) }
    }
}

pub struct wxPrintData { ptr: *mut c_void }
impl _wxPrintData for wxPrintData {}
impl _wxObject for wxPrintData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrintData {
    pub fn from(ptr: *mut c_void) -> wxPrintData { wxPrintData { ptr: ptr } }
    pub fn null() -> wxPrintData { wxPrintData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxPrintData {
        unsafe { wxPrintData { ptr: wxPrintData_Create() } }
    }
}

pub trait _wxPrintData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPrintData>(&self, data: &T) {
        unsafe { wxPrintData_Assign(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCollate(&self) -> c_int {
        unsafe { wxPrintData_GetCollate(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour(&self) -> c_int {
        unsafe { wxPrintData_GetColour(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDuplex(&self) -> c_int {
        unsafe { wxPrintData_GetDuplex(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilename(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintData_GetFilename(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFontMetricPath(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintData_GetFontMetricPath(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintData_GetNoCopies(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxPrintData_GetOrientation(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPrintData_GetPaperId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxPrintData_GetPaperSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPreviewCommand(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintData_GetPreviewCommand(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterCommand(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintData_GetPrinterCommand(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterName(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintData_GetPrinterName(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterOptions(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintData_GetPrinterOptions(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterScaleX(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterScaleY(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterTranslateX(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterTranslateY(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getQuality(&self) -> c_int {
        unsafe { wxPrintData_GetQuality(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintData_SetCollate(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour(&self, colour: c_int) {
        unsafe { wxPrintData_SetColour(self.ptr(), colour) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDuplex(&self, duplex: c_int) {
        unsafe { wxPrintData_SetDuplex(self.ptr(), duplex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFilename(&self, filename: &str) {
        let filename = wxT(filename);
        unsafe { wxPrintData_SetFilename(self.ptr(), filename.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFontMetricPath(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxPrintData_SetFontMetricPath(self.ptr(), path.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintData_SetNoCopies(self.ptr(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxPrintData_SetOrientation(self.ptr(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperId(&self, sizeId: c_int) {
        unsafe { wxPrintData_SetPaperId(self.ptr(), sizeId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPrintData_SetPaperSize(self.ptr(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPreviewCommand<T: _wxCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPreviewCommand(self.ptr(), command.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.ptr(), printMode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterCommand<T: _wxCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPrinterCommand(self.ptr(), command.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterName(&self, name: &str) {
        let name = wxT(name);
        unsafe { wxPrintData_SetPrinterName(self.ptr(), name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterOptions(&self, options: &str) {
        let options = wxT(options);
        unsafe { wxPrintData_SetPrinterOptions(self.ptr(), options.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterScaleX(&self, x: c_double) {
        unsafe { wxPrintData_SetPrinterScaleX(self.ptr(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterScaleY(&self, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaleY(self.ptr(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterScaling(&self, x: c_double, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaling(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterTranslateX(&self, x: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateX(self.ptr(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterTranslateY(&self, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateY(self.ptr(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterTranslation(&self, x: c_int, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslation(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setQuality(&self, quality: c_int) {
        unsafe { wxPrintData_SetQuality(self.ptr(), quality) }
    }
}

pub struct wxPostScriptPrintNativeData { ptr: *mut c_void }
impl _wxPostScriptPrintNativeData for wxPostScriptPrintNativeData {}
impl _wxObject for wxPostScriptPrintNativeData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPostScriptPrintNativeData {
    pub fn from(ptr: *mut c_void) -> wxPostScriptPrintNativeData { wxPostScriptPrintNativeData { ptr: ptr } }
    pub fn null() -> wxPostScriptPrintNativeData { wxPostScriptPrintNativeData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxPostScriptPrintNativeData {
        unsafe { wxPostScriptPrintNativeData { ptr: wxPostScriptPrintNativeData_Create() } }
    }
}

pub trait _wxPostScriptPrintNativeData : _wxObject {
}

pub struct wxPrintDialog { ptr: *mut c_void }
impl _wxPrintDialog for wxPrintDialog {}
impl _wxDialog for wxPrintDialog {}
impl _wxTopLevelWindow for wxPrintDialog {}
impl _wxWindow for wxPrintDialog {}
impl _wxEvtHandler for wxPrintDialog {}
impl _wxObject for wxPrintDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrintDialog {
    pub fn from(ptr: *mut c_void) -> wxPrintDialog { wxPrintDialog { ptr: ptr } }
    pub fn null() -> wxPrintDialog { wxPrintDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPrintDialogData>(parent: &T, data: &U) -> wxPrintDialog {
        unsafe { wxPrintDialog { ptr: wxPrintDialog_Create(parent.ptr(), data.ptr()) } }
    }
}

pub trait _wxPrintDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDC(&self) -> wxDC {
        unsafe { wxDC { ptr: wxPrintDialog_GetPrintDC(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintData<T: _wxPrintData>(&self, _ref: &T) {
        unsafe { wxPrintDialog_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDialogData(&self) -> wxPrintDialogData {
        unsafe { wxPrintDialogData { ptr: wxPrintDialog_GetPrintDialogData(self.ptr()) } }
    }
}

pub struct wxPrintDialogData { ptr: *mut c_void }
impl _wxPrintDialogData for wxPrintDialogData {}
impl _wxObject for wxPrintDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrintDialogData {
    pub fn from(ptr: *mut c_void) -> wxPrintDialogData { wxPrintDialogData { ptr: ptr } }
    pub fn null() -> wxPrintDialogData { wxPrintDialogData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxPrintDialogData {
        unsafe { wxPrintDialogData { ptr: wxPrintDialogData_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData<T: _wxPrintData>(printData: &T) -> wxPrintDialogData {
        unsafe { wxPrintDialogData { ptr: wxPrintDialogData_CreateFromData(printData.ptr()) } }
    }
}

pub trait _wxPrintDialogData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPrintDialogData>(&self, data: &T) {
        unsafe { wxPrintDialogData_Assign(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignData<T: _wxPrintData>(&self, data: &T) {
        unsafe { wxPrintDialogData_AssignData(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableHelp(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnableHelp(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePageNumbers(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnablePageNumbers(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePrintToFile(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnablePrintToFile(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableSelection(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnableSelection(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllPages(&self) -> c_int {
        unsafe { wxPrintDialogData_GetAllPages(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCollate(&self) -> c_int {
        unsafe { wxPrintDialogData_GetCollate(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableHelp(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnableHelp(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePageNumbers(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnablePageNumbers(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePrintToFile(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnablePrintToFile(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableSelection(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnableSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFromPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetFromPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMaxPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMinPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintDialogData_GetNoCopies(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintData<T: _wxPrintData>(&self, _ref: &T) {
        unsafe { wxPrintDialogData_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintToFile(&self) -> c_int {
        unsafe { wxPrintDialogData_GetPrintToFile(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxPrintDialogData_GetSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetToPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAllPages(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetAllPages(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetCollate(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFromPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetFromPage(self.ptr(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMaxPage(self.ptr(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMinPage(self.ptr(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetNoCopies(self.ptr(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintData<T: _wxPrintData>(&self, printData: &T) {
        unsafe { wxPrintDialogData_SetPrintData(self.ptr(), printData.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintToFile(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetPrintToFile(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetSelection(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetToPage(self.ptr(), v) }
    }
}

pub struct wxPrintPreview { ptr: *mut c_void }
impl _wxPrintPreview for wxPrintPreview {}
impl _wxObject for wxPrintPreview { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrintPreview {
    pub fn from(ptr: *mut c_void) -> wxPrintPreview { wxPrintPreview { ptr: ptr } }
    pub fn null() -> wxPrintPreview { wxPrintPreview::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData<T: _wxPrintout, U: _wxPrintout, V: _wxPrintData>(printout: &T, printoutForPrinting: &U, data: &V) -> wxPrintPreview {
        unsafe { wxPrintPreview { ptr: wxPrintPreview_CreateFromData(printout.ptr(), printoutForPrinting.ptr(), data.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromDialogData<T: _wxPrintout, U: _wxPrintout, V: _wxPrintDialogData>(printout: &T, printoutForPrinting: &U, data: &V) -> wxPrintPreview {
        unsafe { wxPrintPreview { ptr: wxPrintPreview_CreateFromDialogData(printout.ptr(), printoutForPrinting.ptr(), data.ptr()) } }
    }
}

pub trait _wxPrintPreview : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBlankPage<T: _wxPreviewCanvas, U: _wxDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_DrawBlankPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCanvas(&self) -> wxPreviewCanvas {
        unsafe { wxPreviewCanvas { ptr: wxPrintPreview_GetCanvas(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame { ptr: wxPrintPreview_GetFrame(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMaxPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMinPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDialogData<T: _wxPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintout(&self) -> wxPrintout {
        unsafe { wxPrintout { ptr: wxPrintPreview_GetPrintout(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintoutForPrinting(&self) -> wxPrintout {
        unsafe { wxPrintout { ptr: wxPrintPreview_GetPrintoutForPrinting(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getZoom(&self) -> c_int {
        unsafe { wxPrintPreview_GetZoom(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxPrintPreview_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paintPage<T: _wxPrintPreview, U: _wxDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_PaintPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn print(&self, interactive: c_int) -> c_int {
        unsafe { wxPrintPreview_Print(self.ptr(), interactive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renderPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_RenderPage(self.ptr(), pageNum) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCanvas<T: _wxPreviewCanvas>(&self, canvas: &T) {
        unsafe { wxPrintPreview_SetCanvas(self.ptr(), canvas.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_SetCurrentPage(self.ptr(), pageNum) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFrame<T: _wxFrame>(&self, frame: &T) {
        unsafe { wxPrintPreview_SetFrame(self.ptr(), frame.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOk(&self, ok: c_int) {
        unsafe { wxPrintPreview_SetOk(self.ptr(), ok) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintout<T: _wxPrintout>(&self, printout: &T) {
        unsafe { wxPrintPreview_SetPrintout(self.ptr(), printout.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.ptr(), percent) }
    }
}

pub struct wxPrinter { ptr: *mut c_void }
impl _wxPrinter for wxPrinter {}
impl _wxObject for wxPrinter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrinter {
    pub fn from(ptr: *mut c_void) -> wxPrinter { wxPrinter { ptr: ptr } }
    pub fn null() -> wxPrinter { wxPrinter::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintDialogData>(data: &T) -> wxPrinter {
        unsafe { wxPrinter { ptr: wxPrinter_Create(data.ptr()) } }
    }
}

pub trait _wxPrinter : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newAbortWindow<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U) -> wxWindow {
        unsafe { wxWindow { ptr: wxPrinter_CreateAbortWindow(self.ptr(), parent.ptr(), printout.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAbort(&self) -> c_int {
        unsafe { wxPrinter_GetAbort(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastError(&self) -> c_int {
        unsafe { wxPrinter_GetLastError(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDialogData<T: _wxPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrinter_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn print<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U, prompt: c_int) -> c_int {
        unsafe { wxPrinter_Print(self.ptr(), parent.ptr(), printout.ptr(), prompt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn printDialog<T: _wxWindow>(&self, parent: &T) -> wxDC {
        unsafe { wxDC { ptr: wxPrinter_PrintDialog(self.ptr(), parent.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reportError<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U, message: &str) {
        let message = wxT(message);
        unsafe { wxPrinter_ReportError(self.ptr(), parent.ptr(), printout.ptr(), message.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setup<T: _wxWindow>(&self, parent: &T) -> c_int {
        unsafe { wxPrinter_Setup(self.ptr(), parent.ptr()) }
    }
}

pub struct wxPrinterDC { ptr: *mut c_void }
impl _wxPrinterDC for wxPrinterDC {}
impl _wxDC for wxPrinterDC {}
impl _wxObject for wxPrinterDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrinterDC {
    pub fn from(ptr: *mut c_void) -> wxPrinterDC { wxPrinterDC { ptr: ptr } }
    pub fn null() -> wxPrinterDC { wxPrinterDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintData>(data: &T) -> wxPrinterDC {
        unsafe { wxPrinterDC { ptr: wxPrinterDC_Create(data.ptr()) } }
    }
}

pub trait _wxPrinterDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperRect(&self) -> wxRect {
        unsafe { wxRect { ptr: wxPrinterDC_GetPaperRect(self.ptr()) } }
    }
}

pub struct wxPrintout { ptr: *mut c_void }
impl _wxPrintout for wxPrintout {}
impl _wxObject for wxPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrintout {
    pub fn from(ptr: *mut c_void) -> wxPrintout { wxPrintout { ptr: ptr } }
    pub fn null() -> wxPrintout { wxPrintout::from(0 as *mut c_void) }
    
}

pub trait _wxPrintout : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDC(&self) -> wxDC {
        unsafe { wxDC { ptr: wxPrintout_GetDC(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPIPrinter(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxPrintout_GetPPIPrinter(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPIScreen(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxPrintout_GetPPIScreen(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSizeMM(&self, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxPrintout_GetPageSizeMM(self.ptr(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSizePixels(&self, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxPrintout_GetPageSizePixels(self.ptr(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { ptr: wxPrintout_GetTitle(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isPreview(&self) -> c_int {
        unsafe { wxPrintout_IsPreview(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDC<T: _wxDC>(&self, dc: &T) {
        unsafe { wxPrintout_SetDC(self.ptr(), dc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPPIPrinter(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIPrinter(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPPIScreen(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIScreen(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSizeMM(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizeMM(self.ptr(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSizePixels(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizePixels(self.ptr(), w, h) }
    }
}

pub struct wxPrivateDropTarget { ptr: *mut c_void }
impl _wxPrivateDropTarget for wxPrivateDropTarget {}
impl _wxDropTarget for wxPrivateDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxPrivateDropTarget {
    pub fn from(ptr: *mut c_void) -> wxPrivateDropTarget { wxPrivateDropTarget { ptr: ptr } }
    pub fn null() -> wxPrivateDropTarget { wxPrivateDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxPrivateDropTarget : _wxDropTarget {
}

pub struct wxProcess { ptr: *mut c_void }
impl _wxProcess for wxProcess {}
impl _wxEvtHandler for wxProcess {}
impl _wxObject for wxProcess { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxProcess {
    pub fn from(ptr: *mut c_void) -> wxProcess { wxProcess { ptr: ptr } }
    pub fn null() -> wxProcess { wxProcess::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault<T: _wxWindow>(_prt: &T, _id: c_int) -> wxProcess {
        unsafe { wxProcess { ptr: wxProcess_CreateDefault(_prt.ptr(), _id) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRedirect<T: _wxWindow>(_prt: &T, _rdr: c_int) -> wxProcess {
        unsafe { wxProcess { ptr: wxProcess_CreateRedirect(_prt.ptr(), _rdr) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn open(cmd: &str, flags: c_int) -> wxProcess {
        let cmd = wxT(cmd);
        unsafe { wxProcess { ptr: wxProcess_Open(cmd.ptr(), flags) } }
    }
}

pub trait _wxProcess : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detach(&self) {
        unsafe { wxProcess_Detach(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getErrorStream(&self) -> wxInputStream {
        unsafe { wxInputStream { ptr: wxProcess_GetErrorStream(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInputStream(&self) -> wxInputStream {
        unsafe { wxInputStream { ptr: wxProcess_GetInputStream(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOutputStream(&self) -> wxOutputStream {
        unsafe { wxOutputStream { ptr: wxProcess_GetOutputStream(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRedirected(&self) -> c_int {
        unsafe { wxProcess_IsRedirected(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn redirect(&self) {
        unsafe { wxProcess_Redirect(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isErrorAvailable(&self) -> c_int {
        unsafe { wxProcess_IsErrorAvailable(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInputAvailable(&self) -> c_int {
        unsafe { wxProcess_IsInputAvailable(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInputOpened(&self) -> c_int {
        unsafe { wxProcess_IsInputOpened(self.ptr()) }
    }
}

pub struct wxProcessEvent { ptr: *mut c_void }
impl _wxProcessEvent for wxProcessEvent {}
impl _wxEvent for wxProcessEvent {}
impl _wxObject for wxProcessEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxProcessEvent {
    pub fn from(ptr: *mut c_void) -> wxProcessEvent { wxProcessEvent { ptr: ptr } }
    pub fn null() -> wxProcessEvent { wxProcessEvent::from(0 as *mut c_void) }
    
}

pub trait _wxProcessEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPid(&self) -> c_int {
        unsafe { wxProcessEvent_GetPid(self.ptr()) }
    }
}

pub struct wxProgressDialog { ptr: *mut c_void }
impl _wxProgressDialog for wxProgressDialog {}
impl _wxFrame for wxProgressDialog {}
impl _wxTopLevelWindow for wxProgressDialog {}
impl _wxWindow for wxProgressDialog {}
impl _wxEvtHandler for wxProgressDialog {}
impl _wxObject for wxProgressDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxProgressDialog {
    pub fn from(ptr: *mut c_void) -> wxProgressDialog { wxProgressDialog { ptr: ptr } }
    pub fn null() -> wxProgressDialog { wxProgressDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(title: &str, message: &str, max: c_int, parent: &T, style: c_int) -> wxProgressDialog {
        let title = wxT(title);
        let message = wxT(message);
        unsafe { wxProgressDialog { ptr: wxProgressDialog_Create(title.ptr(), message.ptr(), max, parent.ptr(), style) } }
    }
}

pub trait _wxProgressDialog : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn update(&self, value: c_int) -> c_int {
        unsafe { wxProgressDialog_Update(self.ptr(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateWithMessage(&self, value: c_int, message: &str) -> c_int {
        let message = wxT(message);
        unsafe { wxProgressDialog_UpdateWithMessage(self.ptr(), value, message.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.ptr()) }
    }
}

pub struct wxQuantize { ptr: *mut c_void }
impl _wxQuantize for wxQuantize {}
impl _wxObject for wxQuantize { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxQuantize {
    pub fn from(ptr: *mut c_void) -> wxQuantize { wxQuantize { ptr: ptr } }
    pub fn null() -> wxQuantize { wxQuantize::from(0 as *mut c_void) }
    
}

pub trait _wxQuantize : _wxObject {
}

pub struct wxQueryNewPaletteEvent { ptr: *mut c_void }
impl _wxQueryNewPaletteEvent for wxQueryNewPaletteEvent {}
impl _wxEvent for wxQueryNewPaletteEvent {}
impl _wxObject for wxQueryNewPaletteEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxQueryNewPaletteEvent {
    pub fn from(ptr: *mut c_void) -> wxQueryNewPaletteEvent { wxQueryNewPaletteEvent { ptr: ptr } }
    pub fn null() -> wxQueryNewPaletteEvent { wxQueryNewPaletteEvent::from(0 as *mut c_void) }
    
}

pub trait _wxQueryNewPaletteEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaletteRealized(&self) -> c_int {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaletteRealized(&self, realized: c_int) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self.ptr(), realized) }
    }
}

pub struct wxRadioBox { ptr: *mut c_void }
impl _wxRadioBox for wxRadioBox {}
impl _wxControl for wxRadioBox {}
impl _wxWindow for wxRadioBox {}
impl _wxEvtHandler for wxRadioBox {}
impl _wxObject for wxRadioBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRadioBox {
    pub fn from(ptr: *mut c_void) -> wxRadioBox { wxRadioBox { ptr: ptr } }
    pub fn null() -> wxRadioBox { wxRadioBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *mut *mut c_char, _dim: c_int, _stl: c_int) -> wxRadioBox {
        let _txt = wxT(_txt);
        unsafe { wxRadioBox { ptr: wxRadioBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl) } }
    }
}

pub trait _wxRadioBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableItem(&self, item: c_int, enable: c_int) {
        unsafe { wxRadioBox_EnableItem(self.ptr(), item, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxRadioBox_FindString(self.ptr(), s.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemLabel(&self, item: c_int) -> ~str {
        unsafe { wxString { ptr: wxRadioBox_GetItemLabel(self.ptr(), item) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfRowsOrCols(&self) -> c_int {
        unsafe { wxRadioBox_GetNumberOfRowsOrCols(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxRadioBox_GetSelection(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStringSelection(&self) -> ~str {
        unsafe { wxString { ptr: wxRadioBox_GetStringSelection(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemBitmap<T: _wxBitmap>(&self, item: c_int, bitmap: &T) {
        unsafe { wxRadioBox_SetItemBitmap(self.ptr(), item, bitmap.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemLabel(&self, item: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxRadioBox_SetItemLabel(self.ptr(), item, label.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNumberOfRowsOrCols(&self, n: c_int) {
        unsafe { wxRadioBox_SetNumberOfRowsOrCols(self.ptr(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, _n: c_int) {
        unsafe { wxRadioBox_SetSelection(self.ptr(), _n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStringSelection(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxRadioBox_SetStringSelection(self.ptr(), s.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showItem(&self, item: c_int, show: c_int) {
        unsafe { wxRadioBox_ShowItem(self.ptr(), item, show) }
    }
}

pub struct wxRadioButton { ptr: *mut c_void }
impl _wxRadioButton for wxRadioButton {}
impl _wxControl for wxRadioButton {}
impl _wxWindow for wxRadioButton {}
impl _wxEvtHandler for wxRadioButton {}
impl _wxObject for wxRadioButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRadioButton {
    pub fn from(ptr: *mut c_void) -> wxRadioButton { wxRadioButton { ptr: ptr } }
    pub fn null() -> wxRadioButton { wxRadioButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxRadioButton {
        let _txt = wxT(_txt);
        unsafe { wxRadioButton { ptr: wxRadioButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxRadioButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxRadioButton_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxRadioButton_SetValue(self.ptr(), value) }
    }
}

pub struct wxRealPoint { ptr: *mut c_void }
impl _wxRealPoint for wxRealPoint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRealPoint {
    pub fn from(ptr: *mut c_void) -> wxRealPoint { wxRealPoint { ptr: ptr } }
    pub fn null() -> wxRealPoint { wxRealPoint::from(0 as *mut c_void) }
    
}

pub trait _wxRealPoint {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxRect { ptr: *mut c_void }
impl _wxRect for wxRect { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRect {
    pub fn from(ptr: *mut c_void) -> wxRect { wxRect { ptr: ptr } }
    pub fn null() -> wxRect { wxRect::from(0 as *mut c_void) }
    
}

pub trait _wxRect {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxRegion { ptr: *mut c_void }
impl _wxRegion for wxRegion {}
impl _wxGDIObject for wxRegion {}
impl _wxObject for wxRegion { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRegion {
    pub fn from(ptr: *mut c_void) -> wxRegion { wxRegion { ptr: ptr } }
    pub fn null() -> wxRegion { wxRegion::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxRegion {
        unsafe { wxRegion { ptr: wxRegion_CreateDefault() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> wxRegion {
        unsafe { wxRegion { ptr: wxRegion_CreateFromRect(x, y, w, h) } }
    }
}

pub trait _wxRegion : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxRegion>(&self, region: &T) {
        unsafe { wxRegion_Assign(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxRegion_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn containsPoint(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxRegion_ContainsPoint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_ContainsRect(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEmpty(&self) -> c_int {
        unsafe { wxRegion_IsEmpty(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBox(&self, _x: *mut c_void, _y: *mut c_void, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxRegion_GetBox(self.ptr(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_IntersectRect(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn intersectRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_IntersectRegion(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_SubtractRect(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_SubtractRegion(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_UnionRect(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unionRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_UnionRegion(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_XorRect(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xorRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_XorRegion(self.ptr(), region.ptr()) }
    }
}

pub struct wxRegionIterator { ptr: *mut c_void }
impl _wxRegionIterator for wxRegionIterator {}
impl _wxObject for wxRegionIterator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRegionIterator {
    pub fn from(ptr: *mut c_void) -> wxRegionIterator { wxRegionIterator { ptr: ptr } }
    pub fn null() -> wxRegionIterator { wxRegionIterator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxRegionIterator {
        unsafe { wxRegionIterator { ptr: wxRegionIterator_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromRegion<T: _wxRegion>(region: &T) -> wxRegionIterator {
        unsafe { wxRegionIterator { ptr: wxRegionIterator_CreateFromRegion(region.ptr()) } }
    }
}

pub trait _wxRegionIterator : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxRegionIterator_GetHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxRegionIterator_GetWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxRegionIterator_GetX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxRegionIterator_GetY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn haveRects(&self) -> c_int {
        unsafe { wxRegionIterator_HaveRects(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn next(&self) {
        unsafe { wxRegionIterator_Next(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reset(&self) {
        unsafe { wxRegionIterator_Reset(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetToRegion<T: _wxRegion>(&self, region: &T) {
        unsafe { wxRegionIterator_ResetToRegion(self.ptr(), region.ptr()) }
    }
}

pub struct wxSVGFileDC { ptr: *mut c_void }
impl _wxSVGFileDC for wxSVGFileDC {}
impl _wxDC for wxSVGFileDC {}
impl _wxObject for wxSVGFileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSVGFileDC {
    pub fn from(ptr: *mut c_void) -> wxSVGFileDC { wxSVGFileDC { ptr: ptr } }
    pub fn null() -> wxSVGFileDC { wxSVGFileDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(fileName: &str) -> wxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { wxSVGFileDC { ptr: wxSVGFileDC_Create(fileName.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithSize(fileName: &str, w: c_int, h: c_int) -> wxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { wxSVGFileDC { ptr: wxSVGFileDC_CreateWithSize(fileName.ptr(), w, h) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithSizeAndResolution(fileName: &str, w: c_int, h: c_int, a_dpi: c_float) -> wxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { wxSVGFileDC { ptr: wxSVGFileDC_CreateWithSizeAndResolution(fileName.ptr(), w, h, a_dpi) } }
    }
}

pub trait _wxSVGFileDC : _wxDC {
}

pub struct wxScreenDC { ptr: *mut c_void }
impl _wxScreenDC for wxScreenDC {}
impl _wxDC for wxScreenDC {}
impl _wxObject for wxScreenDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScreenDC {
    pub fn from(ptr: *mut c_void) -> wxScreenDC { wxScreenDC { ptr: ptr } }
    pub fn null() -> wxScreenDC { wxScreenDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxScreenDC {
        unsafe { wxScreenDC { ptr: wxScreenDC_Create() } }
    }
}

pub trait _wxScreenDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDrawingOnTop(&self) -> c_int {
        unsafe { wxScreenDC_EndDrawingOnTop(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTop(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDrawingOnTopOfWin<T: _wxWindow>(&self, win: &T) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.ptr(), win.ptr()) }
    }
}

pub struct wxScrollBar { ptr: *mut c_void }
impl _wxScrollBar for wxScrollBar {}
impl _wxControl for wxScrollBar {}
impl _wxWindow for wxScrollBar {}
impl _wxEvtHandler for wxScrollBar {}
impl _wxObject for wxScrollBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScrollBar {
    pub fn from(ptr: *mut c_void) -> wxScrollBar { wxScrollBar { ptr: ptr } }
    pub fn null() -> wxScrollBar { wxScrollBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxScrollBar {
        unsafe { wxScrollBar { ptr: wxScrollBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxScrollBar : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSize(&self) -> c_int {
        unsafe { wxScrollBar_GetPageSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRange(&self) -> c_int {
        unsafe { wxScrollBar_GetRange(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getThumbPosition(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getThumbSize(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setThumbPosition(&self, viewStart: c_int) {
        unsafe { wxScrollBar_SetThumbPosition(self.ptr(), viewStart) }
    }
}

pub struct wxScrollEvent { ptr: *mut c_void }
impl _wxScrollEvent for wxScrollEvent {}
impl _wxEvent for wxScrollEvent {}
impl _wxObject for wxScrollEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScrollEvent {
    pub fn from(ptr: *mut c_void) -> wxScrollEvent { wxScrollEvent { ptr: ptr } }
    pub fn null() -> wxScrollEvent { wxScrollEvent::from(0 as *mut c_void) }
    
}

pub trait _wxScrollEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.ptr()) }
    }
}

pub struct wxScrollWinEvent { ptr: *mut c_void }
impl _wxScrollWinEvent for wxScrollWinEvent {}
impl _wxEvent for wxScrollWinEvent {}
impl _wxObject for wxScrollWinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScrollWinEvent {
    pub fn from(ptr: *mut c_void) -> wxScrollWinEvent { wxScrollWinEvent { ptr: ptr } }
    pub fn null() -> wxScrollWinEvent { wxScrollWinEvent::from(0 as *mut c_void) }
    
}

pub trait _wxScrollWinEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetOrientation(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxScrollWinEvent_SetOrientation(self.ptr(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxScrollWinEvent_SetPosition(self.ptr(), pos) }
    }
}

pub struct wxScrolledWindow { ptr: *mut c_void }
impl _wxScrolledWindow for wxScrolledWindow {}
impl _wxPanel for wxScrolledWindow {}
impl _wxWindow for wxScrolledWindow {}
impl _wxEvtHandler for wxScrolledWindow {}
impl _wxObject for wxScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxScrolledWindow {
    pub fn from(ptr: *mut c_void) -> wxScrolledWindow { wxScrolledWindow { ptr: ptr } }
    pub fn null() -> wxScrolledWindow { wxScrolledWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxScrolledWindow {
        unsafe { wxScrolledWindow { ptr: wxScrolledWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxScrolledWindow : _wxPanel {
    #[fixed_stack_segment]
    #[inline(never)]
    fn adjustScrollbars(&self) {
        unsafe { wxScrolledWindow_AdjustScrollbars(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *mut c_void, yy: *mut c_void) {
        unsafe { wxScrolledWindow_CalcScrolledPosition(self.ptr(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *mut c_void, yy: *mut c_void) {
        unsafe { wxScrolledWindow_CalcUnscrolledPosition(self.ptr(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableScrolling(&self, x_scrolling: c_int, y_scrolling: c_int) {
        unsafe { wxScrolledWindow_EnableScrolling(self.ptr(), x_scrolling, y_scrolling) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScaleX(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScaleY(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollPageSize(&self, orient: c_int) -> c_int {
        unsafe { wxScrolledWindow_GetScrollPageSize(self.ptr(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollPixelsPerUnit(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetScrollPixelsPerUnit(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTargetWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxScrolledWindow_GetTargetWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getViewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetViewStart(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onDraw<T: _wxDC>(&self, dc: &T) {
        unsafe { wxScrolledWindow_OnDraw(self.ptr(), dc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scroll(&self, x_pos: c_int, y_pos: c_int) {
        unsafe { wxScrolledWindow_Scroll(self.ptr(), x_pos, y_pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScale(&self, xs: c_double, ys: c_double) {
        unsafe { wxScrolledWindow_SetScale(self.ptr(), xs, ys) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollPageSize(&self, orient: c_int, pageSize: c_int) {
        unsafe { wxScrolledWindow_SetScrollPageSize(self.ptr(), orient, pageSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollbars(&self, pixelsPerUnitX: c_int, pixelsPerUnitY: c_int, noUnitsX: c_int, noUnitsY: c_int, xPos: c_int, yPos: c_int, noRefresh: c_int) {
        unsafe { wxScrolledWindow_SetScrollbars(self.ptr(), pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showScrollbars(&self, showh: c_int, showv: c_int) {
        unsafe { wxScrolledWindow_ShowScrollbars(self.ptr(), showh, showv) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTargetWindow<T: _wxWindow>(&self, target: &T) {
        unsafe { wxScrolledWindow_SetTargetWindow(self.ptr(), target.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn viewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_ViewStart(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.ptr(), xstep, ystep) }
    }
}

pub struct wxSetCursorEvent { ptr: *mut c_void }
impl _wxSetCursorEvent for wxSetCursorEvent {}
impl _wxEvent for wxSetCursorEvent {}
impl _wxObject for wxSetCursorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSetCursorEvent {
    pub fn from(ptr: *mut c_void) -> wxSetCursorEvent { wxSetCursorEvent { ptr: ptr } }
    pub fn null() -> wxSetCursorEvent { wxSetCursorEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSetCursorEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCursor(&self) -> wxCursor {
        unsafe { wxCursor { ptr: wxSetCursorEvent_GetCursor(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasCursor(&self) -> c_int {
        unsafe { wxSetCursorEvent_HasCursor(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCursor<T: _wxCursor>(&self, cursor: &T) {
        unsafe { wxSetCursorEvent_SetCursor(self.ptr(), cursor.ptr()) }
    }
}

pub struct wxShowEvent { ptr: *mut c_void }
impl _wxShowEvent for wxShowEvent {}
impl _wxEvent for wxShowEvent {}
impl _wxObject for wxShowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxShowEvent {
    pub fn from(ptr: *mut c_void) -> wxShowEvent { wxShowEvent { ptr: ptr } }
    pub fn null() -> wxShowEvent { wxShowEvent::from(0 as *mut c_void) }
    
}

pub trait _wxShowEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> c_int {
        unsafe { wxShowEvent_IsShown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShow(&self, show: c_int) {
        unsafe { wxShowEvent_SetShow(self.ptr(), show) }
    }
}

pub struct wxSimpleHelpProvider { ptr: *mut c_void }
impl _wxSimpleHelpProvider for wxSimpleHelpProvider {}
impl _wxHelpProvider for wxSimpleHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSimpleHelpProvider {
    pub fn from(ptr: *mut c_void) -> wxSimpleHelpProvider { wxSimpleHelpProvider { ptr: ptr } }
    pub fn null() -> wxSimpleHelpProvider { wxSimpleHelpProvider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxSimpleHelpProvider {
        unsafe { wxSimpleHelpProvider { ptr: wxSimpleHelpProvider_Create() } }
    }
}

pub trait _wxSimpleHelpProvider : _wxHelpProvider {
}

pub struct wxSingleChoiceDialog { ptr: *mut c_void }
impl _wxSingleChoiceDialog for wxSingleChoiceDialog {}
impl _wxDialog for wxSingleChoiceDialog {}
impl _wxTopLevelWindow for wxSingleChoiceDialog {}
impl _wxWindow for wxSingleChoiceDialog {}
impl _wxEvtHandler for wxSingleChoiceDialog {}
impl _wxObject for wxSingleChoiceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSingleChoiceDialog {
    pub fn from(ptr: *mut c_void) -> wxSingleChoiceDialog { wxSingleChoiceDialog { ptr: ptr } }
    pub fn null() -> wxSingleChoiceDialog { wxSingleChoiceDialog::from(0 as *mut c_void) }
    
}

pub trait _wxSingleChoiceDialog : _wxDialog {
}

pub struct wxSize { ptr: *mut c_void }
impl _wxSize for wxSize { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSize {
    pub fn from(ptr: *mut c_void) -> wxSize { wxSize { ptr: ptr } }
    pub fn null() -> wxSize { wxSize::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(w: c_int, h: c_int) -> wxSize {
        unsafe { wxSize { ptr: wxSize_Create(w, h) } }
    }
}

pub trait _wxSize {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxSize_GetHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxSize_GetWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHeight(&self, h: c_int) {
        unsafe { wxSize_SetHeight(self.ptr(), h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, w: c_int) {
        unsafe { wxSize_SetWidth(self.ptr(), w) }
    }
}

pub struct wxSizeEvent { ptr: *mut c_void }
impl _wxSizeEvent for wxSizeEvent {}
impl _wxEvent for wxSizeEvent {}
impl _wxObject for wxSizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSizeEvent {
    pub fn from(ptr: *mut c_void) -> wxSizeEvent { wxSizeEvent { ptr: ptr } }
    pub fn null() -> wxSizeEvent { wxSizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSizeEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizeEvent_GetSize(self.ptr()) } }
    }
}

pub struct wxSizer { ptr: *mut c_void }
impl _wxSizer for wxSizer {}
impl _wxObject for wxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSizer {
    pub fn from(ptr: *mut c_void) -> wxSizer { wxSizer { ptr: ptr } }
    pub fn null() -> wxSizer { wxSizer::from(0 as *mut c_void) }
    
}

pub trait _wxSizer : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Add(self.ptr(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSizer<T: _wxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addWindow<T: _wxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddWindow(self.ptr(), window.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizer_CalcMin(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fit<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_Fit(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.ptr(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizer_GetMinSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxSizer_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizer_GetSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Insert(self.ptr(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSizer<T: _wxSizer>(&self, before: c_int, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertSizer(self.ptr(), before, sizer.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertWindow<T: _wxWindow>(&self, before: c_int, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertWindow(self.ptr(), before, window.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Prepend(self.ptr(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSizer<T: _wxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependWindow<T: _wxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependWindow(self.ptr(), window.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn recalcSizes(&self) {
        unsafe { wxSizer_RecalcSizes(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDimension(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetDimension(self.ptr(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemMinSize(&self, pos: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSize(self.ptr(), pos, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemMinSizeSizer<T: _wxSizer>(&self, sizer: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.ptr(), sizer.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemMinSizeWindow<T: _wxWindow>(&self, window: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.ptr(), window.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizeHints<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_SetSizeHints(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddSpacer(self.ptr(), size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addStretchSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddStretchSpacer(self.ptr(), size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self, delete_windows: c_int) {
        unsafe { wxSizer_Clear(self.ptr(), delete_windows) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachWindow<T: _wxWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_DetachWindow(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachSizer<T: _wxSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_DetachSizer(self.ptr(), sizer.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detach(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Detach(self.ptr(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fitInside<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_FitInside(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getContainingWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxSizer_GetContainingWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemWindow<T: _wxWindow>(&self, window: &T, recursive: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_GetItemWindow(self.ptr(), window.ptr(), recursive) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemSizer<T: _wxSizer>(&self, window: &T, recursive: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_GetItemSizer(self.ptr(), window.ptr(), recursive) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem(&self, index: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_GetItem(self.ptr(), index) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideWindow<T: _wxWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_HideWindow(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideSizer<T: _wxSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_HideSizer(self.ptr(), sizer.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Hide(self.ptr(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSpacer(&self, index: c_int, size: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_InsertSpacer(self.ptr(), index, size) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_InsertStretchSpacer(self.ptr(), index, prop) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShownWindow(&self, window: *mut *mut c_void) -> c_int {
        unsafe { wxSizer_IsShownWindow(self.ptr(), window) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShownSizer(&self, sizer: *mut *mut c_void) -> c_int {
        unsafe { wxSizer_IsShownSizer(self.ptr(), sizer) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self, index: c_int) -> c_int {
        unsafe { wxSizer_IsShown(self.ptr(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSpacer(&self, size: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_PrependSpacer(self.ptr(), size) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependStretchSpacer(&self, prop: c_int) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizer_PrependStretchSpacer(self.ptr(), prop) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, oldwin: &T, newwin: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceWindow(self.ptr(), oldwin.ptr(), newwin.ptr(), recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceSizer<T: _wxSizer, U: _wxSizer>(&self, oldsz: &T, newsz: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceSizer(self.ptr(), oldsz.ptr(), newsz.ptr(), recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxSizerItem>(&self, oldindex: c_int, newitem: &T) -> c_int {
        unsafe { wxSizer_Replace(self.ptr(), oldindex, newitem.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVirtualSizeHints<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showWindow<T: _wxWindow>(&self, window: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowWindow(self.ptr(), window.ptr(), show, recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showSizer<T: _wxSizer>(&self, sizer: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowSizer(self.ptr(), sizer.ptr(), show, recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show<T: _wxSizer>(&self, sizer: &T, index: c_int, show: c_int) -> c_int {
        unsafe { wxSizer_Show(self.ptr(), sizer.ptr(), index, show) }
    }
}

pub struct wxSizerItem { ptr: *mut c_void }
impl _wxSizerItem for wxSizerItem {}
impl _wxObject for wxSizerItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSizerItem {
    pub fn from(ptr: *mut c_void) -> wxSizerItem { wxSizerItem { ptr: ptr } }
    pub fn null() -> wxSizerItem { wxSizerItem::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> wxSizerItem {
        unsafe { wxSizerItem { ptr: wxSizerItem_Create(width, height, option, flag, border, userData) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newInSizer<T: _wxSizer>(sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInSizer(sizer.ptr(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newInWindow<T: _wxWindow>(window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInWindow(window.ptr(), option, flag, border, userData) }
    }
}

pub trait _wxSizerItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizerItem_CalcMin(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorder(&self) -> c_int {
        unsafe { wxSizerItem_GetBorder(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlag(&self) -> c_int {
        unsafe { wxSizerItem_GetFlag(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizerItem_GetMinSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxSizerItem_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizerItem_GetSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self) -> wxSizer {
        unsafe { wxSizer { ptr: wxSizerItem_GetSizer(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserData(&self) -> *mut c_void {
        unsafe { wxSizerItem_GetUserData(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxSizerItem_GetWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSizer(&self) -> c_int {
        unsafe { wxSizerItem_IsSizer(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSpacer(&self) -> c_int {
        unsafe { wxSizerItem_IsSpacer(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWindow(&self) -> c_int {
        unsafe { wxSizerItem_IsWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorder(&self, border: c_int) {
        unsafe { wxSizerItem_SetBorder(self.ptr(), border) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDimension(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxSizerItem_SetDimension(self.ptr(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlag(&self, flag: c_int) {
        unsafe { wxSizerItem_SetFlag(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFloatRatio(&self, ratio: c_float) {
        unsafe { wxSizerItem_SetFloatRatio(self.ptr(), ratio) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInitSize(&self, x: c_int, y: c_int) {
        unsafe { wxSizerItem_SetInitSize(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRatio(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetRatio(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizer<T: _wxSizer>(&self, sizer: &T) {
        unsafe { wxSizerItem_SetSizer(self.ptr(), sizer.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindow<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizerItem_SetWindow(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteWindows(&self) {
        unsafe { wxSizerItem_DeleteWindows(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachSizer(&self) {
        unsafe { wxSizerItem_DetachSizer(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getProportion(&self) -> c_int {
        unsafe { wxSizerItem_GetProportion(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRect(&self) -> wxRect {
        unsafe { wxRect { ptr: wxSizerItem_GetRect(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpacer(&self) -> wxSize {
        unsafe { wxSize { ptr: wxSizerItem_GetSpacer(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> c_int {
        unsafe { wxSizerItem_IsShown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setProportion(&self, proportion: c_int) {
        unsafe { wxSizerItem_SetProportion(self.ptr(), proportion) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSpacer(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetSpacer(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self, show: c_int) {
        unsafe { wxSizerItem_Show(self.ptr(), show) }
    }
}

pub struct wxSlider { ptr: *mut c_void }
impl _wxSlider for wxSlider {}
impl _wxControl for wxSlider {}
impl _wxWindow for wxSlider {}
impl _wxEvtHandler for wxSlider {}
impl _wxObject for wxSlider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSlider {
    pub fn from(ptr: *mut c_void) -> wxSlider { wxSlider { ptr: ptr } }
    pub fn null() -> wxSlider { wxSlider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> wxSlider {
        unsafe { wxSlider { ptr: wxSlider_Create(_prt.ptr(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxSlider : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearSel(&self) {
        unsafe { wxSlider_ClearSel(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearTicks(&self) {
        unsafe { wxSlider_ClearTicks(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineSize(&self) -> c_int {
        unsafe { wxSlider_GetLineSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMax(&self) -> c_int {
        unsafe { wxSlider_GetMax(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMin(&self) -> c_int {
        unsafe { wxSlider_GetMin(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSize(&self) -> c_int {
        unsafe { wxSlider_GetPageSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelEnd(&self) -> c_int {
        unsafe { wxSlider_GetSelEnd(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelStart(&self) -> c_int {
        unsafe { wxSlider_GetSelStart(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getThumbLength(&self) -> c_int {
        unsafe { wxSlider_GetThumbLength(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTickFreq(&self) -> c_int {
        unsafe { wxSlider_GetTickFreq(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxSlider_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLineSize(&self, lineSize: c_int) {
        unsafe { wxSlider_SetLineSize(self.ptr(), lineSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSize(&self, pageSize: c_int) {
        unsafe { wxSlider_SetPageSize(self.ptr(), pageSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, minValue: c_int, maxValue: c_int) {
        unsafe { wxSlider_SetRange(self.ptr(), minValue, maxValue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, minPos: c_int, maxPos: c_int) {
        unsafe { wxSlider_SetSelection(self.ptr(), minPos, maxPos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setThumbLength(&self, len: c_int) {
        unsafe { wxSlider_SetThumbLength(self.ptr(), len) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTick(&self, tickPos: c_int) {
        unsafe { wxSlider_SetTick(self.ptr(), tickPos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTickFreq(&self, n: c_int, pos: c_int) {
        unsafe { wxSlider_SetTickFreq(self.ptr(), n, pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxSlider_SetValue(self.ptr(), value) }
    }
}

pub struct wxSpinButton { ptr: *mut c_void }
impl _wxSpinButton for wxSpinButton {}
impl _wxControl for wxSpinButton {}
impl _wxWindow for wxSpinButton {}
impl _wxEvtHandler for wxSpinButton {}
impl _wxObject for wxSpinButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSpinButton {
    pub fn from(ptr: *mut c_void) -> wxSpinButton { wxSpinButton { ptr: ptr } }
    pub fn null() -> wxSpinButton { wxSpinButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> wxSpinButton {
        unsafe { wxSpinButton { ptr: wxSpinButton_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxSpinButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMax(&self) -> c_int {
        unsafe { wxSpinButton_GetMax(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMin(&self) -> c_int {
        unsafe { wxSpinButton_GetMin(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxSpinButton_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, minVal: c_int, maxVal: c_int) {
        unsafe { wxSpinButton_SetRange(self.ptr(), minVal, maxVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, val: c_int) {
        unsafe { wxSpinButton_SetValue(self.ptr(), val) }
    }
}

pub struct wxSpinCtrl { ptr: *mut c_void }
impl _wxSpinCtrl for wxSpinCtrl {}
impl _wxControl for wxSpinCtrl {}
impl _wxWindow for wxSpinCtrl {}
impl _wxEvtHandler for wxSpinCtrl {}
impl _wxObject for wxSpinCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSpinCtrl {
    pub fn from(ptr: *mut c_void) -> wxSpinCtrl { wxSpinCtrl { ptr: ptr } }
    pub fn null() -> wxSpinCtrl { wxSpinCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> wxSpinCtrl {
        let _txt = wxT(_txt);
        unsafe { wxSpinCtrl { ptr: wxSpinCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init) } }
    }
}

pub trait _wxSpinCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMax(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMax(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMin(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMin(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxSpinCtrl_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, min_val: c_int, max_val: c_int) {
        unsafe { wxSpinCtrl_SetRange(self.ptr(), min_val, max_val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, val: c_int) {
        unsafe { wxSpinCtrl_SetValue(self.ptr(), val) }
    }
}

pub struct wxSpinEvent { ptr: *mut c_void }
impl _wxSpinEvent for wxSpinEvent {}
impl _wxNotifyEvent for wxSpinEvent {}
impl _wxCommandEvent for wxSpinEvent {}
impl _wxEvent for wxSpinEvent {}
impl _wxObject for wxSpinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSpinEvent {
    pub fn from(ptr: *mut c_void) -> wxSpinEvent { wxSpinEvent { ptr: ptr } }
    pub fn null() -> wxSpinEvent { wxSpinEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSpinEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxSpinEvent_SetPosition(self.ptr(), pos) }
    }
}

pub struct wxSplitterEvent { ptr: *mut c_void }
impl _wxSplitterEvent for wxSplitterEvent {}
impl _wxNotifyEvent for wxSplitterEvent {}
impl _wxCommandEvent for wxSplitterEvent {}
impl _wxEvent for wxSplitterEvent {}
impl _wxObject for wxSplitterEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSplitterEvent {
    pub fn from(ptr: *mut c_void) -> wxSplitterEvent { wxSplitterEvent { ptr: ptr } }
    pub fn null() -> wxSplitterEvent { wxSplitterEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSplitterEvent : _wxNotifyEvent {
}

pub struct wxSplitterWindow { ptr: *mut c_void }
impl _wxSplitterWindow for wxSplitterWindow {}
impl _wxWindow for wxSplitterWindow {}
impl _wxEvtHandler for wxSplitterWindow {}
impl _wxObject for wxSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSplitterWindow {
    pub fn from(ptr: *mut c_void) -> wxSplitterWindow { wxSplitterWindow { ptr: ptr } }
    pub fn null() -> wxSplitterWindow { wxSplitterWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxSplitterWindow {
        unsafe { wxSplitterWindow { ptr: wxSplitterWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxSplitterWindow : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetBorderSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinimumPaneSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetMinimumPaneSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashPosition(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSplitMode(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSplitMode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow1(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxSplitterWindow_GetWindow1(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow2(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxSplitterWindow_GetWindow2(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSplitterWindow_Initialize(self.ptr(), window.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSplit(&self) -> c_int {
        unsafe { wxSplitterWindow_IsSplit(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, winOld: &T, winNew: &U) -> c_int {
        unsafe { wxSplitterWindow_ReplaceWindow(self.ptr(), winOld.ptr(), winNew.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorderSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetBorderSize(self.ptr(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinimumPaneSize(&self, min: c_int) {
        unsafe { wxSplitterWindow_SetMinimumPaneSize(self.ptr(), min) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashPosition(&self, position: c_int, redraw: c_int) {
        unsafe { wxSplitterWindow_SetSashPosition(self.ptr(), position, redraw) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetSashSize(self.ptr(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSplitMode(&self, mode: c_int) {
        unsafe { wxSplitterWindow_SetSplitMode(self.ptr(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn splitHorizontally<T: _wxWindow, U: _wxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitHorizontally(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn splitVertically<T: _wxWindow, U: _wxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitVertically(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unsplit<T: _wxWindow>(&self, toRemove: &T) -> c_int {
        unsafe { wxSplitterWindow_Unsplit(self.ptr(), toRemove.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashGravity(&self, gravity: c_double) {
        unsafe { wxSplitterWindow_SetSashGravity(self.ptr(), gravity) }
    }
}

pub struct wxStaticBitmap { ptr: *mut c_void }
impl _wxStaticBitmap for wxStaticBitmap {}
impl _wxControl for wxStaticBitmap {}
impl _wxWindow for wxStaticBitmap {}
impl _wxEvtHandler for wxStaticBitmap {}
impl _wxObject for wxStaticBitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStaticBitmap {
    pub fn from(ptr: *mut c_void) -> wxStaticBitmap { wxStaticBitmap { ptr: ptr } }
    pub fn null() -> wxStaticBitmap { wxStaticBitmap::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, bitmap: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticBitmap {
        unsafe { wxStaticBitmap { ptr: wxStaticBitmap_Create(_prt.ptr(), _id, bitmap.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxStaticBitmap : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmap<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon<T: _wxIcon>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetIcon(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmap<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxStaticBitmap_SetBitmap(self.ptr(), bitmap.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcon<T: _wxIcon>(&self, icon: &T) {
        unsafe { wxStaticBitmap_SetIcon(self.ptr(), icon.ptr()) }
    }
}

pub struct wxStaticBox { ptr: *mut c_void }
impl _wxStaticBox for wxStaticBox {}
impl _wxControl for wxStaticBox {}
impl _wxWindow for wxStaticBox {}
impl _wxEvtHandler for wxStaticBox {}
impl _wxObject for wxStaticBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStaticBox {
    pub fn from(ptr: *mut c_void) -> wxStaticBox { wxStaticBox { ptr: ptr } }
    pub fn null() -> wxStaticBox { wxStaticBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticBox {
        let _txt = wxT(_txt);
        unsafe { wxStaticBox { ptr: wxStaticBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxStaticBox : _wxControl {
}

pub struct wxStaticBoxSizer { ptr: *mut c_void }
impl _wxStaticBoxSizer for wxStaticBoxSizer {}
impl _wxBoxSizer for wxStaticBoxSizer {}
impl _wxSizer for wxStaticBoxSizer {}
impl _wxObject for wxStaticBoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStaticBoxSizer {
    pub fn from(ptr: *mut c_void) -> wxStaticBoxSizer { wxStaticBoxSizer { ptr: ptr } }
    pub fn null() -> wxStaticBoxSizer { wxStaticBoxSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxStaticBox>(box_: &T, orient: c_int) -> wxStaticBoxSizer {
        unsafe { wxStaticBoxSizer { ptr: wxStaticBoxSizer_Create(box_.ptr(), orient) } }
    }
}

pub trait _wxStaticBoxSizer : _wxBoxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBox(&self) -> wxStaticBox {
        unsafe { wxStaticBox { ptr: wxStaticBoxSizer_GetStaticBox(self.ptr()) } }
    }
}

pub struct wxStaticLine { ptr: *mut c_void }
impl _wxStaticLine for wxStaticLine {}
impl _wxControl for wxStaticLine {}
impl _wxWindow for wxStaticLine {}
impl _wxEvtHandler for wxStaticLine {}
impl _wxObject for wxStaticLine { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStaticLine {
    pub fn from(ptr: *mut c_void) -> wxStaticLine { wxStaticLine { ptr: ptr } }
    pub fn null() -> wxStaticLine { wxStaticLine::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticLine {
        unsafe { wxStaticLine { ptr: wxStaticLine_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxStaticLine : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVertical(&self) -> c_int {
        unsafe { wxStaticLine_IsVertical(self.ptr()) }
    }
}

pub struct wxStaticText { ptr: *mut c_void }
impl _wxStaticText for wxStaticText {}
impl _wxControl for wxStaticText {}
impl _wxWindow for wxStaticText {}
impl _wxEvtHandler for wxStaticText {}
impl _wxObject for wxStaticText { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStaticText {
    pub fn from(ptr: *mut c_void) -> wxStaticText { wxStaticText { ptr: ptr } }
    pub fn null() -> wxStaticText { wxStaticText::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticText {
        let _txt = wxT(_txt);
        unsafe { wxStaticText { ptr: wxStaticText_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxStaticText : _wxControl {
}

pub struct wxStatusBar { ptr: *mut c_void }
impl _wxStatusBar for wxStatusBar {}
impl _wxWindow for wxStatusBar {}
impl _wxEvtHandler for wxStatusBar {}
impl _wxObject for wxStatusBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxStatusBar {
    pub fn from(ptr: *mut c_void) -> wxStatusBar { wxStatusBar { ptr: ptr } }
    pub fn null() -> wxStatusBar { wxStatusBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStatusBar {
        unsafe { wxStatusBar { ptr: wxStatusBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxStatusBar : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderX(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderX(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderY(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderY(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFieldsCount(&self) -> c_int {
        unsafe { wxStatusBar_GetFieldsCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStatusText(&self, number: c_int) -> ~str {
        unsafe { wxString { ptr: wxStatusBar_GetStatusText(self.ptr(), number) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFieldsCount(&self, number: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetFieldsCount(self.ptr(), number, widths) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinHeight(&self, height: c_int) {
        unsafe { wxStatusBar_SetMinHeight(self.ptr(), height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusText(&self, text: &str, number: c_int) {
        let text = wxT(text);
        unsafe { wxStatusBar_SetStatusText(self.ptr(), text.ptr(), number) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusWidths(&self, n: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.ptr(), n, widths) }
    }
}

pub struct wxSysColourChangedEvent { ptr: *mut c_void }
impl _wxSysColourChangedEvent for wxSysColourChangedEvent {}
impl _wxEvent for wxSysColourChangedEvent {}
impl _wxObject for wxSysColourChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSysColourChangedEvent {
    pub fn from(ptr: *mut c_void) -> wxSysColourChangedEvent { wxSysColourChangedEvent { ptr: ptr } }
    pub fn null() -> wxSysColourChangedEvent { wxSysColourChangedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxSysColourChangedEvent : _wxEvent {
}

pub struct wxSystemSettings { ptr: *mut c_void }
impl _wxSystemSettings for wxSystemSettings {}
impl _wxObject for wxSystemSettings { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSystemSettings {
    pub fn from(ptr: *mut c_void) -> wxSystemSettings { wxSystemSettings { ptr: ptr } }
    pub fn null() -> wxSystemSettings { wxSystemSettings::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getColour<T: _wxColour>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetColour(index, _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getFont<T: _wxFont>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetFont(index, _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getMetric(index: c_int) -> c_int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getScreenType() -> c_int {
        unsafe { wxSystemSettings_GetScreenType() }
    }
}

pub trait _wxSystemSettings : _wxObject {
}

pub struct wxTextAttr { ptr: *mut c_void }
impl _wxTextAttr for wxTextAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextAttr {
    pub fn from(ptr: *mut c_void) -> wxTextAttr { wxTextAttr { ptr: ptr } }
    pub fn null() -> wxTextAttr { wxTextAttr::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxColour, U: _wxColour, V: _wxFont>(colText: &T, colBack: &U, font: &V) -> wxTextAttr {
        unsafe { wxTextAttr { ptr: wxTextAttr_Create(colText.ptr(), colBack.ptr(), font.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> wxTextAttr {
        unsafe { wxTextAttr { ptr: wxTextAttr_CreateDefault() } }
    }
}

pub trait _wxTextAttr {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_GetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxTextAttr_GetFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_GetTextColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxTextAttr_HasBackgroundColour(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFont(&self) -> c_int {
        unsafe { wxTextAttr_HasFont(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasTextColour(&self) -> c_int {
        unsafe { wxTextAttr_HasTextColour(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isDefault(&self) -> c_int {
        unsafe { wxTextAttr_IsDefault(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetTextColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxTextAttr_SetFont(self.ptr(), font.ptr()) }
    }
}

pub struct wxTextCtrl { ptr: *mut c_void }
impl _wxTextCtrl for wxTextCtrl {}
impl _wxControl for wxTextCtrl {}
impl _wxWindow for wxTextCtrl {}
impl _wxEvtHandler for wxTextCtrl {}
impl _wxObject for wxTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextCtrl {
    pub fn from(ptr: *mut c_void) -> wxTextCtrl { wxTextCtrl { ptr: ptr } }
    pub fn null() -> wxTextCtrl { wxTextCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> wxTextCtrl {
        let _txt = wxT(_txt);
        unsafe { wxTextCtrl { ptr: wxTextCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxTextCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_AppendText(self.ptr(), text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canCopy(&self) -> c_int {
        unsafe { wxTextCtrl_CanCopy(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canCut(&self) -> c_int {
        unsafe { wxTextCtrl_CanCut(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canPaste(&self) -> c_int {
        unsafe { wxTextCtrl_CanPaste(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canRedo(&self) -> c_int {
        unsafe { wxTextCtrl_CanRedo(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canUndo(&self) -> c_int {
        unsafe { wxTextCtrl_CanUndo(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn changeValue(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_ChangeValue(self.ptr(), text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxTextCtrl_Clear(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self) {
        unsafe { wxTextCtrl_Copy(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cut(&self) {
        unsafe { wxTextCtrl_Cut(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn discardEdits(&self) {
        unsafe { wxTextCtrl_DiscardEdits(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInsertionPoint(&self) -> c_long {
        unsafe { wxTextCtrl_GetInsertionPoint(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastPosition(&self) -> c_long {
        unsafe { wxTextCtrl_GetLastPosition(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineLength(&self, lineNo: c_long) -> c_int {
        unsafe { wxTextCtrl_GetLineLength(self.ptr(), lineNo) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineText(&self, lineNo: c_long) -> ~str {
        unsafe { wxString { ptr: wxTextCtrl_GetLineText(self.ptr(), lineNo) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfLines(&self) -> c_int {
        unsafe { wxTextCtrl_GetNumberOfLines(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { wxTextCtrl_GetSelection(self.ptr(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> ~str {
        unsafe { wxString { ptr: wxTextCtrl_GetValue(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEditable(&self) -> c_int {
        unsafe { wxTextCtrl_IsEditable(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isModified(&self) -> c_int {
        unsafe { wxTextCtrl_IsModified(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxTextCtrl_LoadFile(self.ptr(), file.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste(&self) {
        unsafe { wxTextCtrl_Paste(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionToXY(&self, pos: c_long, x: *mut c_long, y: *mut c_long) -> c_int {
        unsafe { wxTextCtrl_PositionToXY(self.ptr(), pos, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn redo(&self) {
        unsafe { wxTextCtrl_Redo(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_Remove(self.ptr(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace(&self, from: c_long, to: c_long, value: &str) {
        let value = wxT(value);
        unsafe { wxTextCtrl_Replace(self.ptr(), from, to, value.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxTextCtrl_SaveFile(self.ptr(), file.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEditable(&self, editable: c_int) {
        unsafe { wxTextCtrl_SetEditable(self.ptr(), editable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPoint(&self, pos: c_long) {
        unsafe { wxTextCtrl_SetInsertionPoint(self.ptr(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPointEnd(&self) {
        unsafe { wxTextCtrl_SetInsertionPointEnd(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_SetSelection(self.ptr(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: &str) {
        let value = wxT(value);
        unsafe { wxTextCtrl_SetValue(self.ptr(), value.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showPosition(&self, pos: c_long) {
        unsafe { wxTextCtrl_ShowPosition(self.ptr(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn undo(&self) {
        unsafe { wxTextCtrl_Undo(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_WriteText(self.ptr(), text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn emulateKeyPress<T: _wxKeyEvent>(&self, keyevent: &T) -> c_int {
        unsafe { wxTextCtrl_EmulateKeyPress(self.ptr(), keyevent.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultStyle(&self) -> wxTextAttr {
        unsafe { wxTextAttr { ptr: wxTextCtrl_GetDefaultStyle(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRange(&self, from: c_long, to: c_long) -> ~str {
        unsafe { wxString { ptr: wxTextCtrl_GetRange(self.ptr(), from, to) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStringSelection(&self) -> ~str {
        unsafe { wxString { ptr: wxTextCtrl_GetStringSelection(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMultiLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsMultiLine(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSingleLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsSingleLine(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultStyle<T: _wxTextAttr>(&self, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetDefaultStyle(self.ptr(), style.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.ptr(), len) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle<T: _wxTextAttr>(&self, start: c_long, end: c_long, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetStyle(self.ptr(), start, end, style.ptr()) }
    }
}

pub struct wxTextDataObject { ptr: *mut c_void }
impl _wxTextDataObject for wxTextDataObject {}
impl _wxDataObjectSimple for wxTextDataObject {}
impl _wxDataObject for wxTextDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextDataObject {
    pub fn from(ptr: *mut c_void) -> wxTextDataObject { wxTextDataObject { ptr: ptr } }
    pub fn null() -> wxTextDataObject { wxTextDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxTextDataObject : _wxDataObjectSimple {
}

pub struct wxTextDropTarget { ptr: *mut c_void }
impl _wxTextDropTarget for wxTextDropTarget {}
impl _wxDropTarget for wxTextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextDropTarget {
    pub fn from(ptr: *mut c_void) -> wxTextDropTarget { wxTextDropTarget { ptr: ptr } }
    pub fn null() -> wxTextDropTarget { wxTextDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxTextDropTarget : _wxDropTarget {
}

pub struct wxTextEntryDialog { ptr: *mut c_void }
impl _wxTextEntryDialog for wxTextEntryDialog {}
impl _wxDialog for wxTextEntryDialog {}
impl _wxTopLevelWindow for wxTextEntryDialog {}
impl _wxWindow for wxTextEntryDialog {}
impl _wxEvtHandler for wxTextEntryDialog {}
impl _wxObject for wxTextEntryDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextEntryDialog {
    pub fn from(ptr: *mut c_void) -> wxTextEntryDialog { wxTextEntryDialog { ptr: ptr } }
    pub fn null() -> wxTextEntryDialog { wxTextEntryDialog::from(0 as *mut c_void) }
    
}

pub trait _wxTextEntryDialog : _wxDialog {
}

pub struct wxTextValidator { ptr: *mut c_void }
impl _wxTextValidator for wxTextValidator {}
impl _wxValidator for wxTextValidator {}
impl _wxEvtHandler for wxTextValidator {}
impl _wxObject for wxTextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTextValidator {
    pub fn from(ptr: *mut c_void) -> wxTextValidator { wxTextValidator { ptr: ptr } }
    pub fn null() -> wxTextValidator { wxTextValidator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(style: c_int, val: *mut c_void) -> wxTextValidator {
        unsafe { wxTextValidator { ptr: wxTextValidator_Create(style, val) } }
    }
}

pub trait _wxTextValidator : _wxValidator {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExcludes(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxTextValidator_GetExcludes(self.ptr(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIncludes(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxTextValidator_GetIncludes(self.ptr(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExcludes(&self, list: *mut c_void, count: c_int) {
        unsafe { wxTextValidator_SetExcludes(self.ptr(), list, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIncludes(&self, list: *mut c_void, count: c_int) {
        unsafe { wxTextValidator_SetIncludes(self.ptr(), list, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clone(&self) -> wxValidator {
        unsafe { wxValidator { ptr: wxTextValidator_Clone(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxTextValidator_GetStyle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onChar<T: _wxEvent>(&self, event: &T) {
        unsafe { wxTextValidator_OnChar(self.ptr(), event.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.ptr(), style) }
    }
}

pub struct wxTimer { ptr: *mut c_void }
impl _wxTimer for wxTimer {}
impl _wxObject for wxTimer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTimer {
    pub fn from(ptr: *mut c_void) -> wxTimer { wxTimer { ptr: ptr } }
    pub fn null() -> wxTimer { wxTimer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int) -> wxTimer {
        unsafe { wxTimer { ptr: wxTimer_Create(_prt.ptr(), _id) } }
    }
}

pub trait _wxTimer : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimer_GetInterval(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOneShot(&self) -> c_int {
        unsafe { wxTimer_IsOneShot(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRuning(&self) -> c_int {
        unsafe { wxTimer_IsRuning(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self, _int: c_int, _one: c_int) -> c_int {
        unsafe { wxTimer_Start(self.ptr(), _int, _one) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) {
        unsafe { wxTimer_Stop(self.ptr()) }
    }
}

pub struct wxTimerEvent { ptr: *mut c_void }
impl _wxTimerEvent for wxTimerEvent {}
impl _wxEvent for wxTimerEvent {}
impl _wxObject for wxTimerEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTimerEvent {
    pub fn from(ptr: *mut c_void) -> wxTimerEvent { wxTimerEvent { ptr: ptr } }
    pub fn null() -> wxTimerEvent { wxTimerEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTimerEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.ptr()) }
    }
}

pub struct wxTimerEx { ptr: *mut c_void }
impl _wxTimerEx for wxTimerEx {}
impl _wxTimer for wxTimerEx {}
impl _wxObject for wxTimerEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTimerEx {
    pub fn from(ptr: *mut c_void) -> wxTimerEx { wxTimerEx { ptr: ptr } }
    pub fn null() -> wxTimerEx { wxTimerEx::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxTimerEx {
        unsafe { wxTimerEx { ptr: wxTimerEx_Create() } }
    }
}

pub trait _wxTimerEx : _wxTimer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn connect<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxTimerEx_Connect(self.ptr(), closure.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClosure(&self) -> wxClosure {
        unsafe { wxClosure { ptr: wxTimerEx_GetClosure(self.ptr()) } }
    }
}

pub struct wxTimerRunner { ptr: *mut c_void }
impl _wxTimerRunner for wxTimerRunner { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTimerRunner {
    pub fn from(ptr: *mut c_void) -> wxTimerRunner { wxTimerRunner { ptr: ptr } }
    pub fn null() -> wxTimerRunner { wxTimerRunner::from(0 as *mut c_void) }
    
}

pub trait _wxTimerRunner {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxTipWindow { ptr: *mut c_void }
impl _wxTipWindow for wxTipWindow {}
impl _wxPopupTransientWindow for wxTipWindow {}
impl _wxPopupWindow for wxTipWindow {}
impl _wxWindow for wxTipWindow {}
impl _wxEvtHandler for wxTipWindow {}
impl _wxObject for wxTipWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTipWindow {
    pub fn from(ptr: *mut c_void) -> wxTipWindow { wxTipWindow { ptr: ptr } }
    pub fn null() -> wxTipWindow { wxTipWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, text: &str, maxLength: c_int) -> wxTipWindow {
        let text = wxT(text);
        unsafe { wxTipWindow { ptr: wxTipWindow_Create(parent.ptr(), text.ptr(), maxLength) } }
    }
}

pub trait _wxTipWindow : _wxPopupTransientWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTipWindowPtr(&self, windowPtr: *mut c_void) {
        unsafe { wxTipWindow_SetTipWindowPtr(self.ptr(), windowPtr) }
    }
}

pub struct wxToggleButton { ptr: *mut c_void }
impl _wxToggleButton for wxToggleButton {}
impl _wxControl for wxToggleButton {}
impl _wxWindow for wxToggleButton {}
impl _wxEvtHandler for wxToggleButton {}
impl _wxObject for wxToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxToggleButton {
    pub fn from(ptr: *mut c_void) -> wxToggleButton { wxToggleButton { ptr: ptr } }
    pub fn null() -> wxToggleButton { wxToggleButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, id: c_int, label: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> wxToggleButton {
        let label = wxT(label);
        unsafe { wxToggleButton { ptr: wxToggleButton_Create(parent.ptr(), id, label.ptr(), x, y, w, h, style) } }
    }
}

pub trait _wxToggleButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxToggleButton_GetValue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, state: c_int) {
        unsafe { wxToggleButton_SetValue(self.ptr(), state) }
    }
}

pub struct wxToolBar { ptr: *mut c_void }
impl _wxToolBar for wxToolBar {}
impl _wxToolBarBase for wxToolBar {}
impl _wxControl for wxToolBar {}
impl _wxWindow for wxToolBar {}
impl _wxEvtHandler for wxToolBar {}
impl _wxObject for wxToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxToolBar {
    pub fn from(ptr: *mut c_void) -> wxToolBar { wxToolBar { ptr: ptr } }
    pub fn null() -> wxToolBar { wxToolBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxToolBar {
        unsafe { wxToolBar { ptr: wxToolBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxToolBar : _wxToolBarBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addControl<T: _wxControl>(&self, ctrl: &T) -> c_int {
        unsafe { wxToolBar_AddControl(self.ptr(), ctrl.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTool<T: _wxBitmap>(&self, id: c_int, bmp: &T, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddTool(self.ptr(), id, bmp.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addToolEx<T: _wxBitmap, U: _wxBitmap, V: _wxObject>(&self, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, x: c_int, y: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddToolEx(self.ptr(), id, bmp1.ptr(), bmp2.ptr(), isToggle, x, y, data.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteTool(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_DeleteTool(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteToolByPos(&self, pos: c_int) -> c_int {
        unsafe { wxToolBar_DeleteToolByPos(self.ptr(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableTool(&self, id: c_int, enable: c_int) {
        unsafe { wxToolBar_EnableTool(self.ptr(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMargins(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxToolBar_GetMargins(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolBitmapSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxToolBar_GetToolBitmapSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolClientData(&self, id: c_int) -> wxObject {
        unsafe { wxObject { ptr: wxToolBar_GetToolClientData(self.ptr(), id) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolEnabled(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolEnabled(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolLongHelp(&self, id: c_int) -> ~str {
        unsafe { wxString { ptr: wxToolBar_GetToolLongHelp(self.ptr(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolShortHelp(&self, id: c_int) -> ~str {
        unsafe { wxString { ptr: wxToolBar_GetToolShortHelp(self.ptr(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxToolBar_GetToolSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolState(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolState(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertControl<T: _wxControl>(&self, pos: c_int, ctrl: &T) {
        unsafe { wxToolBar_InsertControl(self.ptr(), pos, ctrl.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.ptr(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertTool<T: _wxBitmap, U: _wxBitmap, V: _wxObject>(&self, pos: c_int, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_InsertTool(self.ptr(), pos, id, bmp1.ptr(), bmp2.ptr(), isToggle, data.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn realize(&self) -> c_int {
        unsafe { wxToolBar_Realize(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeTool(&self, id: c_int) {
        unsafe { wxToolBar_RemoveTool(self.ptr(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetMargins(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolBitmapSize(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetToolBitmapSize(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolClientData<T: _wxObject>(&self, id: c_int, data: &T) {
        unsafe { wxToolBar_SetToolClientData(self.ptr(), id, data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolLongHelp(&self, id: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxToolBar_SetToolLongHelp(self.ptr(), id, str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolPacking(&self, packing: c_int) {
        unsafe { wxToolBar_SetToolPacking(self.ptr(), packing) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolSeparation(&self, separation: c_int) {
        unsafe { wxToolBar_SetToolSeparation(self.ptr(), separation) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolShortHelp(&self, id: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxToolBar_SetToolShortHelp(self.ptr(), id, str.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toggleTool(&self, id: c_int, toggle: c_int) {
        unsafe { wxToolBar_ToggleTool(self.ptr(), id, toggle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTool2<T: _wxBitmap, U: _wxBitmap>(&self, toolId: c_int, label: &str, bmp: &T, bmpDisabled: &U, itemKind: c_int, shortHelp: &str, longHelp: &str) {
        let label = wxT(label);
        let shortHelp = wxT(shortHelp);
        let longHelp = wxT(longHelp);
        unsafe { wxToolBar_AddTool2(self.ptr(), toolId, label.ptr(), bmp.ptr(), bmpDisabled.ptr(), itemKind, shortHelp.ptr(), longHelp.ptr()) }
    }
}

pub struct wxToolBarBase { ptr: *mut c_void }
impl _wxToolBarBase for wxToolBarBase {}
impl _wxControl for wxToolBarBase {}
impl _wxWindow for wxToolBarBase {}
impl _wxEvtHandler for wxToolBarBase {}
impl _wxObject for wxToolBarBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxToolBarBase {
    pub fn from(ptr: *mut c_void) -> wxToolBarBase { wxToolBarBase { ptr: ptr } }
    pub fn null() -> wxToolBarBase { wxToolBarBase::from(0 as *mut c_void) }
    
}

pub trait _wxToolBarBase : _wxControl {
}

pub struct wxToolTip { ptr: *mut c_void }
impl _wxToolTip for wxToolTip {}
impl _wxObject for wxToolTip { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxToolTip {
    pub fn from(ptr: *mut c_void) -> wxToolTip { wxToolTip { ptr: ptr } }
    pub fn null() -> wxToolTip { wxToolTip::from(0 as *mut c_void) }
    
}

pub trait _wxToolTip : _wxObject {
}

pub struct wxTopLevelWindow { ptr: *mut c_void }
impl _wxTopLevelWindow for wxTopLevelWindow {}
impl _wxWindow for wxTopLevelWindow {}
impl _wxEvtHandler for wxTopLevelWindow {}
impl _wxObject for wxTopLevelWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTopLevelWindow {
    pub fn from(ptr: *mut c_void) -> wxTopLevelWindow { wxTopLevelWindow { ptr: ptr } }
    pub fn null() -> wxTopLevelWindow { wxTopLevelWindow::from(0 as *mut c_void) }
    
}

pub trait _wxTopLevelWindow : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableCloseButton(&self, enable: c_int) -> c_int {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.ptr(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultButton(&self) -> wxButton {
        unsafe { wxButton { ptr: wxTopLevelWindow_GetDefaultButton(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultItem(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxTopLevelWindow_GetDefaultItem(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon(&self) -> wxIcon {
        unsafe { wxIcon { ptr: wxTopLevelWindow_GetIcon(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { ptr: wxTopLevelWindow_GetTitle(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn iconize(&self, iconize: c_int) -> c_int {
        unsafe { wxTopLevelWindow_Iconize(self.ptr(), iconize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isActive(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsActive(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIconized(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsIconized(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMaximized(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsMaximized(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maximize(&self, maximize: c_int) {
        unsafe { wxTopLevelWindow_Maximize(self.ptr(), maximize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn requestUserAttention(&self, flags: c_int) {
        unsafe { wxTopLevelWindow_RequestUserAttention(self.ptr(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultButton<T: _wxButton>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.ptr(), pBut.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultItem<T: _wxWindow>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.ptr(), pBut.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcon<T: _wxIcon>(&self, pIcon: &T) {
        unsafe { wxTopLevelWindow_SetIcon(self.ptr(), pIcon.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcons(&self, _icons: *mut c_void) {
        unsafe { wxTopLevelWindow_SetIcons(self.ptr(), _icons) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMaxSize(self.ptr(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMinSize(self.ptr(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTitle(&self, pString: &str) {
        let pString = wxT(pString);
        unsafe { wxTopLevelWindow_SetTitle(self.ptr(), pString.ptr()) }
    }
}

pub struct wxTreeCtrl { ptr: *mut c_void }
impl _wxTreeCtrl for wxTreeCtrl {}
impl _wxControl for wxTreeCtrl {}
impl _wxWindow for wxTreeCtrl {}
impl _wxEvtHandler for wxTreeCtrl {}
impl _wxObject for wxTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeCtrl {
    pub fn from(ptr: *mut c_void) -> wxTreeCtrl { wxTreeCtrl { ptr: ptr } }
    pub fn null() -> wxTreeCtrl { wxTreeCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_obj: *mut c_void, _cmp: *mut c_void, _prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxTreeCtrl {
        unsafe { wxTreeCtrl { ptr: wxTreeCtrl_Create(_obj, _cmp, _prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new2<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxTreeCtrl {
        unsafe { wxTreeCtrl { ptr: wxTreeCtrl_Create2(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxTreeCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRoot<T: _wxTreeItemData, U: _wxTreeItemId>(&self, text: &str, image: c_int, selectedImage: c_int, data: &T, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AddRoot(self.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendItem<T: _wxTreeItemId, U: _wxTreeItemData, V: _wxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AppendItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn collapse<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Collapse(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn collapseAndReset<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteChildren<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_DeleteChildren(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn editLabel<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EditLabel(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endEditLabel<T: _wxTreeItemId>(&self, item: &T, discardChanges: c_int) {
        unsafe { wxTreeCtrl_EndEditLabel(self.ptr(), item.ptr(), discardChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisible<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EnsureVisible(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expand<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Expand(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBoundingRect<T: _wxTreeItemId>(&self, item: &T, textOnly: c_int) -> wxRect {
        unsafe { wxRect { ptr: wxTreeCtrl_GetBoundingRect(self.ptr(), item.ptr(), textOnly) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildrenCount<T: _wxTreeItemId>(&self, item: &T, recursively: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.ptr(), item.ptr(), recursively) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEditControl(&self) -> wxTextCtrl {
        unsafe { wxTextCtrl { ptr: wxTreeCtrl_GetEditControl(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstVisibleItem<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.ptr(), item.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageList(&self) -> wxImageList {
        unsafe { wxImageList { ptr: wxTreeCtrl_GetImageList(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemData<T: _wxTreeItemId>(&self, item: &T) -> *mut c_void {
        unsafe { wxTreeCtrl_GetItemData(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemImage<T: _wxTreeItemId>(&self, item: &T, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.ptr(), item.ptr(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemText<T: _wxTreeItemId>(&self, item: &T) -> ~str {
        unsafe { wxString { ptr: wxTreeCtrl_GetItemText(self.ptr(), item.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetLastChild(self.ptr(), item.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetNextChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextSibling<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextVisible<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrevSibling<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrevVisible<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRootItem<T: _wxTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetRootItem(self.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection<T: _wxTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetSelection(self.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelections(&self, selections: *mut c_void) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self.ptr(), selections) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpacing(&self) -> c_int {
        unsafe { wxTreeCtrl_GetSpacing(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStateImageList(&self) -> wxImageList {
        unsafe { wxImageList { ptr: wxTreeCtrl_GetStateImageList(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest<T: _wxTreeItemId>(&self, _x: c_int, _y: c_int, flags: *mut c_int, _item: &T) {
        unsafe { wxTreeCtrl_HitTest(self.ptr(), _x, _y, flags, _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxTreeItemId, U: _wxTreeItemId, V: _wxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemByIndex<T: _wxTreeItemId, U: _wxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBold<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsBold(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExpanded<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsExpanded(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelected<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsSelected(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsVisible(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn itemHasChildren<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onCompareItems<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item1: &T, item2: &U) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.ptr(), item1.ptr(), item2.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependItem<T: _wxTreeItemId, U: _wxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_PrependItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollTo<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_ScrollTo(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectItem<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SelectItem(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetImageList(self.ptr(), imageList.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.ptr(), indent) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemBackgroundColour<T: _wxTreeItemId, U: _wxColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemBold<T: _wxTreeItemId>(&self, item: &T, bold: c_int) {
        unsafe { wxTreeCtrl_SetItemBold(self.ptr(), item.ptr(), bold) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemData<T: _wxTreeItemId>(&self, item: &T, data: *mut c_void) {
        unsafe { wxTreeCtrl_SetItemData(self.ptr(), item.ptr(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemDropHighlight<T: _wxTreeItemId>(&self, item: &T, highlight: c_int) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.ptr(), item.ptr(), highlight) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemFont<T: _wxTreeItemId, U: _wxFont>(&self, item: &T, font: &U) {
        unsafe { wxTreeCtrl_SetItemFont(self.ptr(), item.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemHasChildren<T: _wxTreeItemId>(&self, item: &T, hasChildren: c_int) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.ptr(), item.ptr(), hasChildren) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemImage<T: _wxTreeItemId>(&self, item: &T, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.ptr(), item.ptr(), image, which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemText<T: _wxTreeItemId>(&self, item: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_SetItemText(self.ptr(), item.ptr(), text.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemTextColour<T: _wxTreeItemId, U: _wxColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.ptr(), spacing) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStateImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetStateImageList(self.ptr(), imageList.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortChildren<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SortChildren(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toggle<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Toggle(self.ptr(), item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem2<T: _wxWindow, U: _wxTreeItemId, V: _wxClosure, W: _wxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, closure: &V, _item: &W) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem2(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemByIndex2<T: _wxWindow, U: _wxClosure, V: _wxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, closure: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemClientClosure<T: _wxTreeItemId>(&self, item: &T) -> wxClosure {
        unsafe { wxClosure { ptr: wxTreeCtrl_GetItemClientClosure(self.ptr(), item.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemClientClosure<T: _wxTreeItemId, U: _wxClosure>(&self, item: &T, closure: &U) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.ptr(), item.ptr(), closure.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignImageList(self.ptr(), imageList.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignStateImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.ptr(), imageList.ptr()) }
    }
}

pub struct wxTreeEvent { ptr: *mut c_void }
impl _wxTreeEvent for wxTreeEvent {}
impl _wxNotifyEvent for wxTreeEvent {}
impl _wxCommandEvent for wxTreeEvent {}
impl _wxEvent for wxTreeEvent {}
impl _wxObject for wxTreeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeEvent {
    pub fn from(ptr: *mut c_void) -> wxTreeEvent { wxTreeEvent { ptr: ptr } }
    pub fn null() -> wxTreeEvent { wxTreeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTreeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem<T: _wxTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { ptr: wxTreeEvent_GetLabel(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOldItem<T: _wxTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetOldItem(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPoint(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxTreeEvent_GetPoint(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKeyEvent(&self) -> wxKeyEvent {
        unsafe { wxKeyEvent { ptr: wxTreeEvent_GetKeyEvent(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEditCancelled(&self) -> c_int {
        unsafe { wxTreeEvent_IsEditCancelled(self.ptr()) }
    }
}

pub struct wxTreeItemData { ptr: *mut c_void }
impl _wxTreeItemData for wxTreeItemData {}
impl _wxClientData for wxTreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeItemData {
    pub fn from(ptr: *mut c_void) -> wxTreeItemData { wxTreeItemData { ptr: ptr } }
    pub fn null() -> wxTreeItemData { wxTreeItemData::from(0 as *mut c_void) }
    
}

pub trait _wxTreeItemData : _wxClientData {
}

pub struct wxTreeItemId { ptr: *mut c_void }
impl _wxTreeItemId for wxTreeItemId { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTreeItemId {
    pub fn from(ptr: *mut c_void) -> wxTreeItemId { wxTreeItemId { ptr: ptr } }
    pub fn null() -> wxTreeItemId { wxTreeItemId::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxTreeItemId {
        unsafe { wxTreeItemId { ptr: wxTreeItemId_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromValue(value: intptr_t) -> wxTreeItemId {
        unsafe { wxTreeItemId { ptr: wxTreeItemId_CreateFromValue(value) } }
    }
}

pub trait _wxTreeItemId {
    fn ptr(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxTreeItemId_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clone(&self) -> wxTreeItemId {
        unsafe { wxTreeItemId { ptr: wxTreeItemId_Clone(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.ptr()) }
    }
}

pub struct wxUpdateUIEvent { ptr: *mut c_void }
impl _wxUpdateUIEvent for wxUpdateUIEvent {}
impl _wxEvent for wxUpdateUIEvent {}
impl _wxObject for wxUpdateUIEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxUpdateUIEvent {
    pub fn from(ptr: *mut c_void) -> wxUpdateUIEvent { wxUpdateUIEvent { ptr: ptr } }
    pub fn null() -> wxUpdateUIEvent { wxUpdateUIEvent::from(0 as *mut c_void) }
    
}

pub trait _wxUpdateUIEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, check: c_int) {
        unsafe { wxUpdateUIEvent_Check(self.ptr(), check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: c_int) {
        unsafe { wxUpdateUIEvent_Enable(self.ptr(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChecked(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetChecked(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnabled(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetEnabled(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetChecked(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetChecked(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetEnabled(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetEnabled(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetText(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetText(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { ptr: wxUpdateUIEvent_GetText(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxUpdateUIEvent_SetText(self.ptr(), text.ptr()) }
    }
}

pub struct wxValidator { ptr: *mut c_void }
impl _wxValidator for wxValidator {}
impl _wxEvtHandler for wxValidator {}
impl _wxObject for wxValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxValidator {
    pub fn from(ptr: *mut c_void) -> wxValidator { wxValidator { ptr: ptr } }
    pub fn null() -> wxValidator { wxValidator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxValidator {
        unsafe { wxValidator { ptr: wxValidator_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setBellOnError(doIt: c_int) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

pub trait _wxValidator : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxValidator_GetWindow(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxValidator_SetWindow(self.ptr(), win.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferFromWindow(&self) -> c_int {
        unsafe { wxValidator_TransferFromWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferToWindow(&self) -> c_int {
        unsafe { wxValidator_TransferToWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn validate<T: _wxWindow>(&self, parent: &T) -> c_int {
        unsafe { wxValidator_Validate(self.ptr(), parent.ptr()) }
    }
}

pub struct wxView { ptr: *mut c_void }
impl _wxView for wxView {}
impl _wxEvtHandler for wxView {}
impl _wxObject for wxView { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxView {
    pub fn from(ptr: *mut c_void) -> wxView { wxView { ptr: ptr } }
    pub fn null() -> wxView { wxView::from(0 as *mut c_void) }
    
}

pub trait _wxView : _wxEvtHandler {
}

pub struct wxSound { ptr: *mut c_void }
impl _wxSound for wxSound {}
impl _wxEvtHandler for wxSound {}
impl _wxObject for wxSound { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSound {
    pub fn from(ptr: *mut c_void) -> wxSound { wxSound { ptr: ptr } }
    pub fn null() -> wxSound { wxSound::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(fileName: &str, isResource: c_int) -> wxSound {
        let fileName = wxT(fileName);
        unsafe { wxSound { ptr: wxSound_Create(fileName.ptr(), isResource) } }
    }
}

pub trait _wxSound : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxSound_IsOk(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play(&self, flag: c_int) -> c_int {
        unsafe { wxSound_Play(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) {
        unsafe { wxSound_Stop(self.ptr()) }
    }
}

pub struct wxWindow { ptr: *mut c_void }
impl _wxWindow for wxWindow {}
impl _wxEvtHandler for wxWindow {}
impl _wxObject for wxWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWindow {
    pub fn from(ptr: *mut c_void) -> wxWindow { wxWindow { ptr: ptr } }
    pub fn null() -> wxWindow { wxWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxWindow {
        unsafe { wxWindow { ptr: wxWindow_Create(_prt.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait _wxWindow : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addChild<T: _wxWindow>(&self, child: &T) {
        unsafe { wxWindow_AddChild(self.ptr(), child.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addConstraintReference<T: _wxWindow>(&self, otherWin: &T) {
        unsafe { wxWindow_AddConstraintReference(self.ptr(), otherWin.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn captureMouse(&self) {
        unsafe { wxWindow_CaptureMouse(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn center(&self, direction: c_int) {
        unsafe { wxWindow_Center(self.ptr(), direction) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centerOnParent(&self, dir: c_int) {
        unsafe { wxWindow_CenterOnParent(self.ptr(), dir) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearBackground(&self) {
        unsafe { wxWindow_ClearBackground(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clientToScreen(&self, x: c_int, y: c_int) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ClientToScreen(self.ptr(), x, y) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self, _force: c_int) -> c_int {
        unsafe { wxWindow_Close(self.ptr(), _force) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertDialogToPixels(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ConvertDialogToPixels(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertPixelsToDialog(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ConvertPixelsToDialog(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteRelatedConstraints(&self) {
        unsafe { wxWindow_DeleteRelatedConstraints(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroy(&self) -> c_int {
        unsafe { wxWindow_Destroy(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyChildren(&self) -> c_int {
        unsafe { wxWindow_DestroyChildren(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disable(&self) -> c_int {
        unsafe { wxWindow_Disable(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn doPhase(&self, phase: c_int) -> c_int {
        unsafe { wxWindow_DoPhase(self.ptr(), phase) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self) -> c_int {
        unsafe { wxWindow_Enable(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findFocus(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxWindow_FindFocus(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findWindow(&self, name: &str) -> wxWindow {
        let name = wxT(name);
        unsafe { wxWindow { ptr: wxWindow_FindWindow(self.ptr(), name.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fit(&self) {
        unsafe { wxWindow_Fit(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fitInside(&self) {
        unsafe { wxWindow_FitInside(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn freeze(&self) {
        unsafe { wxWindow_Freeze(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEffectiveMinSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxWindow_GetEffectiveMinSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBestSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxWindow_GetBestSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaret(&self) -> wxCaret {
        unsafe { wxCaret { ptr: wxWindow_GetCaret(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharHeight(&self) -> c_int {
        unsafe { wxWindow_GetCharHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharWidth(&self) -> c_int {
        unsafe { wxWindow_GetCharWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxWindow_GetChildren(self.ptr(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientData(&self) -> wxClientData {
        unsafe { wxClientData { ptr: wxWindow_GetClientData(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxWindow_GetClientSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.ptr(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getConstraints(&self) -> wxLayoutConstraints {
        unsafe { wxLayoutConstraints { ptr: wxWindow_GetConstraints(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getConstraintsInvolvedIn(&self) -> *mut c_void {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCursor(&self) -> wxCursor {
        unsafe { wxCursor { ptr: wxWindow_GetCursor(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDropTarget(&self) -> wxDropTarget {
        unsafe { wxDropTarget { ptr: wxWindow_GetDropTarget(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEventHandler(&self) -> wxEvtHandler {
        unsafe { wxEvtHandler { ptr: wxWindow_GetEventHandler(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxWindow_GetFont(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getForegroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetForegroundColour(self.ptr(), _ref.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHandle(&self) -> *mut c_void {
        unsafe { wxWindow_GetHandle(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxWindow_GetId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { ptr: wxWindow_GetLabel(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelEmpty(&self) -> c_int {
        unsafe { wxWindow_GetLabelEmpty(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxHeight(&self) -> c_int {
        unsafe { wxWindow_GetMaxHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxWidth(&self) -> c_int {
        unsafe { wxWindow_GetMaxWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinHeight(&self) -> c_int {
        unsafe { wxWindow_GetMinHeight(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinWidth(&self) -> c_int {
        unsafe { wxWindow_GetMinWidth(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { ptr: wxWindow_GetName(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getParent(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxWindow_GetParent(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_GetPosition(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPositionConstraint(&self, _x: *mut c_int, _y: *mut c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.ptr(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRect(&self) -> wxRect {
        unsafe { wxRect { ptr: wxWindow_GetRect(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollPos(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollPos(self.ptr(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollRange(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollRange(self.ptr(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollThumb(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollThumb(self.ptr(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxWindow_GetSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.ptr(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self) -> wxSizer {
        unsafe { wxSizer { ptr: wxWindow_GetSizer(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent<T: _wxFont>(&self, string: &str, x: *mut c_int, y: *mut c_int, descent: *mut c_int, externalLeading: *mut c_int, theFont: &T) {
        let string = wxT(string);
        unsafe { wxWindow_GetTextExtent(self.ptr(), string.ptr(), x, y, descent, externalLeading, theFont.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolTip(&self) -> ~str {
        unsafe { wxString { ptr: wxWindow_GetToolTip(self.ptr()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUpdateRegion(&self) -> wxRegion {
        unsafe { wxRegion { ptr: wxWindow_GetUpdateRegion(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValidator(&self) -> wxValidator {
        unsafe { wxValidator { ptr: wxWindow_GetValidator(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVirtualSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxWindow_GetVirtualSize(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindowStyleFlag(&self) -> c_int {
        unsafe { wxWindow_GetWindowStyleFlag(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFlag(&self, flag: c_int) -> c_int {
        unsafe { wxWindow_HasFlag(self.ptr(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) -> c_int {
        unsafe { wxWindow_Hide(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initDialog(&self) {
        unsafe { wxWindow_InitDialog(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBeingDeleted(&self) -> c_int {
        unsafe { wxWindow_IsBeingDeleted(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> c_int {
        unsafe { wxWindow_IsEnabled(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxWindow_IsExposed(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> c_int {
        unsafe { wxWindow_IsShown(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isTopLevel(&self) -> c_int {
        unsafe { wxWindow_IsTopLevel(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layout(&self) -> c_int {
        unsafe { wxWindow_Layout(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutPhase1(&self, noChanges: *mut c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase1(self.ptr(), noChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutPhase2(&self, noChanges: *mut c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase2(self.ptr(), noChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lower(&self) {
        unsafe { wxWindow_Lower(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeModal(&self, modal: c_int) {
        unsafe { wxWindow_MakeModal(self.ptr(), modal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_Move(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveConstraint(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_MoveConstraint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn popEventHandler(&self, deleteHandler: c_int) -> *mut c_void {
        unsafe { wxWindow_PopEventHandler(self.ptr(), deleteHandler) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn popupMenu<T: _wxMenu>(&self, menu: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.ptr(), menu.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepareDC<T: _wxDC>(&self, dc: &T) {
        unsafe { wxWindow_PrepareDC(self.ptr(), dc.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pushEventHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxWindow_PushEventHandler(self.ptr(), handler.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn raise(&self) {
        unsafe { wxWindow_Raise(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refresh(&self, eraseBackground: c_int) {
        unsafe { wxWindow_Refresh(self.ptr(), eraseBackground) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refreshRect(&self, eraseBackground: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_RefreshRect(self.ptr(), eraseBackground, x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn releaseMouse(&self) {
        unsafe { wxWindow_ReleaseMouse(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeChild<T: _wxWindow>(&self, child: &T) {
        unsafe { wxWindow_RemoveChild(self.ptr(), child.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeConstraintReference<T: _wxWindow>(&self, otherWin: &T) {
        unsafe { wxWindow_RemoveConstraintReference(self.ptr(), otherWin.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reparent<T: _wxWindow>(&self, _par: &T) -> c_int {
        unsafe { wxWindow_Reparent(self.ptr(), _par.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn screenToClient(&self, x: c_int, y: c_int) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ScreenToClient(self.ptr(), x, y) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self.ptr(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self.ptr(), dx, dy, x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAcceleratorTable<T: _wxAcceleratorTable>(&self, accel: &T) {
        unsafe { wxWindow_SetAcceleratorTable(self.ptr(), accel.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAutoLayout(&self, autoLayout: c_int) {
        unsafe { wxWindow_SetAutoLayout(self.ptr(), autoLayout) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaret<T: _wxCaret>(&self, caret: &T) {
        unsafe { wxWindow_SetCaret(self.ptr(), caret.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientData<T: _wxClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientData(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientObject<T: _wxClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientObject(self.ptr(), data.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self.ptr(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setConstraintSizes(&self, recurse: c_int) {
        unsafe { wxWindow_SetConstraintSizes(self.ptr(), recurse) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setConstraints<T: _wxLayoutConstraints>(&self, constraints: &T) {
        unsafe { wxWindow_SetConstraints(self.ptr(), constraints.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCursor<T: _wxCursor>(&self, cursor: &T) -> c_int {
        unsafe { wxWindow_SetCursor(self.ptr(), cursor.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDropTarget<T: _wxDropTarget>(&self, dropTarget: &T) {
        unsafe { wxWindow_SetDropTarget(self.ptr(), dropTarget.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self.ptr(), exStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) -> c_int {
        unsafe { wxWindow_SetFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setForegroundColour<T: _wxColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.ptr(), _id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabel(&self, _title: &str) {
        let _title = wxT(_title);
        unsafe { wxWindow_SetLabel(self.ptr(), _title.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setName(&self, _name: &str) {
        let _name = wxT(_name);
        unsafe { wxWindow_SetName(self.ptr(), _name.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollPos(&self, orient: c_int, pos: c_int, refresh: c_int) {
        unsafe { wxWindow_SetScrollPos(self.ptr(), orient, pos, refresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollbar(&self, orient: c_int, pos: c_int, thumbVisible: c_int, range: c_int, refresh: c_int) {
        unsafe { wxWindow_SetScrollbar(self.ptr(), orient, pos, thumbVisible, range, refresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSize(&self, x: c_int, y: c_int, width: c_int, height: c_int, sizeFlags: c_int) {
        unsafe { wxWindow_SetSize(self.ptr(), x, y, width, height, sizeFlags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizeConstraint(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_SetSizeConstraint(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizeHints(&self, minW: c_int, minH: c_int, maxW: c_int, maxH: c_int, incW: c_int, incH: c_int) {
        unsafe { wxWindow_SetSizeHints(self.ptr(), minW, minH, maxW, maxH, incW, incH) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizer<T: _wxSizer>(&self, sizer: &T) {
        unsafe { wxWindow_SetSizer(self.ptr(), sizer.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolTip(&self, tip: &str) {
        let tip = wxT(tip);
        unsafe { wxWindow_SetToolTip(self.ptr(), tip.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValidator<T: _wxValidator>(&self, validator: &T) {
        unsafe { wxWindow_SetValidator(self.ptr(), validator.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowStyleFlag(&self, style: c_long) {
        unsafe { wxWindow_SetWindowStyleFlag(self.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) -> c_int {
        unsafe { wxWindow_Show(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn thaw(&self) {
        unsafe { wxWindow_Thaw(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferDataFromWindow(&self) -> c_int {
        unsafe { wxWindow_TransferDataFromWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferDataToWindow(&self) -> c_int {
        unsafe { wxWindow_TransferDataToWindow(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unsetConstraints(&self, c: *mut c_void) {
        unsafe { wxWindow_UnsetConstraints(self.ptr(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateWindowUI(&self) {
        unsafe { wxWindow_UpdateWindowUI(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn validate(&self) -> c_int {
        unsafe { wxWindow_Validate(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVirtualSize(&self, w: c_int, h: c_int) {
        unsafe { wxWindow_SetVirtualSize(self.ptr(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn warpPointer(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_WarpPointer(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertDialogToPixelsEx(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ConvertDialogToPixelsEx(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertPixelsToDialogEx(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ConvertPixelsToDialogEx(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn screenToClient2(&self, x: c_int, y: c_int) -> wxPoint {
        unsafe { wxPoint { ptr: wxWindow_ScreenToClient2(self.ptr(), x, y) } }
    }
}

pub struct wxWindowCreateEvent { ptr: *mut c_void }
impl _wxWindowCreateEvent for wxWindowCreateEvent {}
impl _wxCommandEvent for wxWindowCreateEvent {}
impl _wxEvent for wxWindowCreateEvent {}
impl _wxObject for wxWindowCreateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWindowCreateEvent {
    pub fn from(ptr: *mut c_void) -> wxWindowCreateEvent { wxWindowCreateEvent { ptr: ptr } }
    pub fn null() -> wxWindowCreateEvent { wxWindowCreateEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWindowCreateEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxWindowCreateEvent_GetWindow(self.ptr()) } }
    }
}

pub struct wxWindowDC { ptr: *mut c_void }
impl _wxWindowDC for wxWindowDC {}
impl _wxDC for wxWindowDC {}
impl _wxObject for wxWindowDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWindowDC {
    pub fn from(ptr: *mut c_void) -> wxWindowDC { wxWindowDC { ptr: ptr } }
    pub fn null() -> wxWindowDC { wxWindowDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> wxWindowDC {
        unsafe { wxWindowDC { ptr: wxWindowDC_Create(win.ptr()) } }
    }
}

pub trait _wxWindowDC : _wxDC {
}

pub struct wxWindowDestroyEvent { ptr: *mut c_void }
impl _wxWindowDestroyEvent for wxWindowDestroyEvent {}
impl _wxCommandEvent for wxWindowDestroyEvent {}
impl _wxEvent for wxWindowDestroyEvent {}
impl _wxObject for wxWindowDestroyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWindowDestroyEvent {
    pub fn from(ptr: *mut c_void) -> wxWindowDestroyEvent { wxWindowDestroyEvent { ptr: ptr } }
    pub fn null() -> wxWindowDestroyEvent { wxWindowDestroyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWindowDestroyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow { ptr: wxWindowDestroyEvent_GetWindow(self.ptr()) } }
    }
}

pub struct wxWindowDisabler { ptr: *mut c_void }
impl _wxWindowDisabler for wxWindowDisabler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWindowDisabler {
    pub fn from(ptr: *mut c_void) -> wxWindowDisabler { wxWindowDisabler { ptr: ptr } }
    pub fn null() -> wxWindowDisabler { wxWindowDisabler::from(0 as *mut c_void) }
    
}

pub trait _wxWindowDisabler {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxXmlResourceHandler { ptr: *mut c_void }
impl _wxXmlResourceHandler for wxXmlResourceHandler {}
impl _wxObject for wxXmlResourceHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxXmlResourceHandler {
    pub fn from(ptr: *mut c_void) -> wxXmlResourceHandler { wxXmlResourceHandler { ptr: ptr } }
    pub fn null() -> wxXmlResourceHandler { wxXmlResourceHandler::from(0 as *mut c_void) }
    
}

pub trait _wxXmlResourceHandler : _wxObject {
}

pub struct wxGenericDragImage { ptr: *mut c_void }
impl _wxGenericDragImage for wxGenericDragImage {}
impl _wxDragImage for wxGenericDragImage {}
impl _wxObject for wxGenericDragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGenericDragImage {
    pub fn from(ptr: *mut c_void) -> wxGenericDragImage { wxGenericDragImage { ptr: ptr } }
    pub fn null() -> wxGenericDragImage { wxGenericDragImage::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxCursor>(cursor: &T) -> wxGenericDragImage {
        unsafe { wxGenericDragImage { ptr: wxGenericDragImage_Create(cursor.ptr()) } }
    }
}

pub trait _wxGenericDragImage : _wxDragImage {
    #[fixed_stack_segment]
    #[inline(never)]
    fn doDrawImage<T: _wxDC>(&self, dc: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxGenericDragImage_DoDrawImage(self.ptr(), dc.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> wxRect {
        unsafe { wxRect { ptr: wxGenericDragImage_GetImageRect(self.ptr(), x_pos, y_pos) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateBackingFromWindow<T: _wxDC, U: _wxMemoryDC>(&self, windowDC: &T, destDC: &U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.ptr(), windowDC.ptr(), destDC.ptr(), x, y, w, h, xdest, ydest, width, height) }
    }
}

pub struct wxGraphicsObject { ptr: *mut c_void }
impl _wxGraphicsObject for wxGraphicsObject {}
impl _wxObject for wxGraphicsObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsObject {
    pub fn from(ptr: *mut c_void) -> wxGraphicsObject { wxGraphicsObject { ptr: ptr } }
    pub fn null() -> wxGraphicsObject { wxGraphicsObject::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getRenderer() -> wxGraphicsRenderer {
        unsafe { wxGraphicsRenderer { ptr: wxGraphicsObject_GetRenderer() } }
    }
}

pub trait _wxGraphicsObject : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isNull(&self) -> c_int {
        unsafe { wxGraphicsObject_IsNull(self.ptr()) }
    }
}

pub struct wxGraphicsBrush { ptr: *mut c_void }
impl _wxGraphicsBrush for wxGraphicsBrush {}
impl _wxGraphicsObject for wxGraphicsBrush {}
impl _wxObject for wxGraphicsBrush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsBrush {
    pub fn from(ptr: *mut c_void) -> wxGraphicsBrush { wxGraphicsBrush { ptr: ptr } }
    pub fn null() -> wxGraphicsBrush { wxGraphicsBrush::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxGraphicsBrush {
        unsafe { wxGraphicsBrush { ptr: wxGraphicsBrush_Create() } }
    }
}

pub trait _wxGraphicsBrush : _wxGraphicsObject {
}

pub struct wxGraphicsContext { ptr: *mut c_void }
impl _wxGraphicsContext for wxGraphicsContext {}
impl _wxGraphicsObject for wxGraphicsContext {}
impl _wxObject for wxGraphicsContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsContext {
    pub fn from(ptr: *mut c_void) -> wxGraphicsContext { wxGraphicsContext { ptr: ptr } }
    pub fn null() -> wxGraphicsContext { wxGraphicsContext::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindowDC>(dc: &T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsContext_Create(dc.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromWindow<T: _wxWindow>(window: &T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsContext_CreateFromWindow(window.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromNative(context: *mut c_void) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsContext_CreateFromNative(context) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromNativeWindow(window: *mut c_void) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsContext_CreateFromNativeWindow(window) } }
    }
}

pub trait _wxGraphicsContext : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn clip<T: _wxRegion>(&self, region: &T) {
        unsafe { wxGraphicsContext_Clip(self.ptr(), region.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBitmap<T: _wxBitmap>(&self, bmp: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.ptr(), bmp.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawIcon<T: _wxIcon>(&self, icon: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.ptr(), icon.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.ptr(), n, x, y, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPath<T: _wxGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_DrawPath(self.ptr(), path.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawRectangle(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawRoundedRectangle(self.ptr(), x, y, w, h, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawText(&self, text: &str, x: c_double, y: c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_DrawText(self.ptr(), text.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawTextWithAngle(&self, text: &str, x: c_double, y: c_double, radius: c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.ptr(), text.ptr(), x, y, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fillPath<T: _wxGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.ptr(), path.ptr(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn strokePath<T: _wxGraphicsPath>(&self, path: &T) {
        unsafe { wxGraphicsContext_StrokePath(self.ptr(), path.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativeContext(&self) -> *mut c_void {
        unsafe { wxGraphicsContext_GetNativeContext(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent(&self, text: &str, width: *mut c_double, height: *mut c_double, descent: *mut c_double, externalLeading: *mut c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_GetTextExtent(self.ptr(), text.ptr(), width, height, descent, externalLeading) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsContext_Rotate(self.ptr(), angle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsContext_Scale(self.ptr(), xScale, yScale) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsContext_Translate(self.ptr(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTransform<T: _wxGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_SetTransform(self.ptr(), path.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn concatTransform<T: _wxGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_ConcatTransform(self.ptr(), path.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBrush<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetBrush(self.ptr(), brush.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGraphicsBrush<T: _wxGraphicsBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.ptr(), brush.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont, U: _wxColour>(&self, font: &T, colour: &U) {
        unsafe { wxGraphicsContext_SetFont(self.ptr(), font.ptr(), colour.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGraphicsFont<T: _wxGraphicsFont>(&self, font: &T) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.ptr(), font.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPen<T: _wxPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetPen(self.ptr(), pen.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGraphicsPen<T: _wxGraphicsPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self.ptr(), pen.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self.ptr(), x1, y1, x2, y2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn strokeLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.ptr(), n, x, y, style) }
    }
}

pub struct wxGraphicsFont { ptr: *mut c_void }
impl _wxGraphicsFont for wxGraphicsFont {}
impl _wxGraphicsObject for wxGraphicsFont {}
impl _wxObject for wxGraphicsFont { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsFont {
    pub fn from(ptr: *mut c_void) -> wxGraphicsFont { wxGraphicsFont { ptr: ptr } }
    pub fn null() -> wxGraphicsFont { wxGraphicsFont::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxGraphicsFont {
        unsafe { wxGraphicsFont { ptr: wxGraphicsFont_Create() } }
    }
}

pub trait _wxGraphicsFont : _wxGraphicsObject {
}

pub struct wxGraphicsMatrix { ptr: *mut c_void }
impl _wxGraphicsMatrix for wxGraphicsMatrix {}
impl _wxGraphicsObject for wxGraphicsMatrix {}
impl _wxObject for wxGraphicsMatrix { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsMatrix {
    pub fn from(ptr: *mut c_void) -> wxGraphicsMatrix { wxGraphicsMatrix { ptr: ptr } }
    pub fn null() -> wxGraphicsMatrix { wxGraphicsMatrix::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxGraphicsMatrix {
        unsafe { wxGraphicsMatrix { ptr: wxGraphicsMatrix_Create() } }
    }
}

pub trait _wxGraphicsMatrix : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn concat<T: _wxGraphicsMatrix>(&self, t: &T) {
        unsafe { wxGraphicsMatrix_Concat(self.ptr(), t.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn get(&self, a: *mut c_double, b: *mut c_double, c: *mut c_double, d: *mut c_double, tx: *mut c_double, ty: *mut c_double) {
        unsafe { wxGraphicsMatrix_Get(self.ptr(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativeMatrix(&self) -> *mut c_void {
        unsafe { wxGraphicsMatrix_GetNativeMatrix(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn invert(&self) {
        unsafe { wxGraphicsMatrix_Invert(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxGraphicsMatrix>(&self, t: &T) -> c_int {
        unsafe { wxGraphicsMatrix_IsEqual(self.ptr(), t.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIdentity(&self) -> c_int {
        unsafe { wxGraphicsMatrix_IsIdentity(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsMatrix_Rotate(self.ptr(), angle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsMatrix_Scale(self.ptr(), xScale, yScale) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, a: c_double, b: c_double, c: c_double, d: c_double, tx: c_double, ty: c_double) {
        unsafe { wxGraphicsMatrix_Set(self.ptr(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsMatrix_Translate(self.ptr(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transformPoint(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxGraphicsMatrix_TransformPoint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transformDistance(&self, dx: *mut c_double, dy: *mut c_double) {
        unsafe { wxGraphicsMatrix_TransformDistance(self.ptr(), dx, dy) }
    }
}

pub struct wxGraphicsPath { ptr: *mut c_void }
impl _wxGraphicsPath for wxGraphicsPath {}
impl _wxGraphicsObject for wxGraphicsPath {}
impl _wxObject for wxGraphicsPath { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsPath {
    pub fn from(ptr: *mut c_void) -> wxGraphicsPath { wxGraphicsPath { ptr: ptr } }
    pub fn null() -> wxGraphicsPath { wxGraphicsPath::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxGraphicsPath {
        unsafe { wxGraphicsPath { ptr: wxGraphicsPath_Create() } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn unGetNativePath(p: *mut c_void) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

pub trait _wxGraphicsPath : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_MoveToPoint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: c_int) {
        unsafe { wxGraphicsPath_AddArc(self.ptr(), x, y, r, startAngle, endAngle, clockwise) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addArcToPoint(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddArcToPoint(self.ptr(), x1, y1, x2, y2, r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCircle(&self, x: c_double, y: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddCircle(self.ptr(), x, y, r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCurveToPoint(&self, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddCurveToPoint(self.ptr(), cx1, cy1, cx2, cy2, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddEllipse(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addLineToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddLineToPoint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPath<T: _wxGraphicsPath>(&self, x: c_double, y: c_double, path: &T) {
        unsafe { wxGraphicsPath_AddPath(self.ptr(), x, y, path.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addQuadCurveToPoint(&self, cx: c_double, cy: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddQuadCurveToPoint(self.ptr(), cx, cy, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddRectangle(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsPath_AddRoundedRectangle(self.ptr(), x, y, w, h, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn closeSubpath(&self) {
        unsafe { wxGraphicsPath_CloseSubpath(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn contains(&self, x: c_double, y: c_double, style: c_int) {
        unsafe { wxGraphicsPath_Contains(self.ptr(), x, y, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBox(&self, x: *mut c_double, y: *mut c_double, w: *mut c_double, h: *mut c_double) {
        unsafe { wxGraphicsPath_GetBox(self.ptr(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPoint(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxGraphicsPath_GetCurrentPoint(self.ptr(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transform<T: _wxGraphicsMatrix>(&self, matrix: &T) {
        unsafe { wxGraphicsPath_Transform(self.ptr(), matrix.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativePath(&self) -> *mut c_void {
        unsafe { wxGraphicsPath_GetNativePath(self.ptr()) }
    }
}

pub struct wxGraphicsPen { ptr: *mut c_void }
impl _wxGraphicsPen for wxGraphicsPen {}
impl _wxGraphicsObject for wxGraphicsPen {}
impl _wxObject for wxGraphicsPen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsPen {
    pub fn from(ptr: *mut c_void) -> wxGraphicsPen { wxGraphicsPen { ptr: ptr } }
    pub fn null() -> wxGraphicsPen { wxGraphicsPen::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> wxGraphicsPen {
        unsafe { wxGraphicsPen { ptr: wxGraphicsPen_Create() } }
    }
}

pub trait _wxGraphicsPen : _wxGraphicsObject {
}

pub struct wxGraphicsRenderer { ptr: *mut c_void }
impl _wxGraphicsRenderer for wxGraphicsRenderer {}
impl _wxGraphicsObject for wxGraphicsRenderer {}
impl _wxObject for wxGraphicsRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGraphicsRenderer {
    pub fn from(ptr: *mut c_void) -> wxGraphicsRenderer { wxGraphicsRenderer { ptr: ptr } }
    pub fn null() -> wxGraphicsRenderer { wxGraphicsRenderer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContext<T: _wxWindowDC>(dc: &T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsRenderer_CreateContext(dc.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromWindow<T: _wxWindow>(window: &T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromWindow(window.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromNativeContext(context: *mut c_void) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromNativeContext(context) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromNativeWindow(window: *mut c_void) -> wxGraphicsContext {
        unsafe { wxGraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromNativeWindow(window) } }
    }
}

pub trait _wxGraphicsRenderer : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRenderer(&self) -> wxGraphicsRenderer {
        unsafe { wxGraphicsRenderer { ptr: wxGraphicsRenderer_GetDefaultRenderer(self.ptr()) } }
    }
}

pub struct wxcPrintout { ptr: *mut c_void }
impl _wxcPrintout for wxcPrintout {}
impl _wxPrintout for wxcPrintout {}
impl _wxObject for wxcPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxcPrintout {
    pub fn from(ptr: *mut c_void) -> wxcPrintout { wxcPrintout { ptr: ptr } }
    pub fn null() -> wxcPrintout { wxcPrintout::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(title: &str) -> wxcPrintout {
        let title = wxT(title);
        unsafe { wxcPrintout { ptr: wxcPrintout_Create(title.ptr()) } }
    }
}

pub trait _wxcPrintout : _wxPrintout {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.ptr(), startPage, endPage, fromPage, toPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEvtHandler(&self) -> wxcPrintoutHandler {
        unsafe { wxcPrintoutHandler { ptr: wxcPrintout_GetEvtHandler(self.ptr()) } }
    }
}

pub struct wxcPrintEvent { ptr: *mut c_void }
impl _wxcPrintEvent for wxcPrintEvent {}
impl _wxEvent for wxcPrintEvent {}
impl _wxObject for wxcPrintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxcPrintEvent {
    pub fn from(ptr: *mut c_void) -> wxcPrintEvent { wxcPrintEvent { ptr: ptr } }
    pub fn null() -> wxcPrintEvent { wxcPrintEvent::from(0 as *mut c_void) }
    
}

pub trait _wxcPrintEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintout(&self) -> wxcPrintout {
        unsafe { wxcPrintout { ptr: wxcPrintEvent_GetPrintout(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEndPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetEndPage(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getContinue(&self) -> c_int {
        unsafe { wxcPrintEvent_GetContinue(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setContinue(&self, cont: c_int) {
        unsafe { wxcPrintEvent_SetContinue(self.ptr(), cont) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintEvent_SetPageLimits(self.ptr(), startPage, endPage, fromPage, toPage) }
    }
}

pub struct wxcPrintoutHandler { ptr: *mut c_void }
impl _wxcPrintoutHandler for wxcPrintoutHandler {}
impl _wxEvtHandler for wxcPrintoutHandler {}
impl _wxObject for wxcPrintoutHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxcPrintoutHandler {
    pub fn from(ptr: *mut c_void) -> wxcPrintoutHandler { wxcPrintoutHandler { ptr: ptr } }
    pub fn null() -> wxcPrintoutHandler { wxcPrintoutHandler::from(0 as *mut c_void) }
    
}

pub trait _wxcPrintoutHandler : _wxEvtHandler {
}

pub struct wxcTreeItemData { ptr: *mut c_void }
impl _wxcTreeItemData for wxcTreeItemData {}
impl _wxTreeItemData for wxcTreeItemData {}
impl _wxClientData for wxcTreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxcTreeItemData {
    pub fn from(ptr: *mut c_void) -> wxcTreeItemData { wxcTreeItemData { ptr: ptr } }
    pub fn null() -> wxcTreeItemData { wxcTreeItemData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxClosure>(closure: &T) -> wxcTreeItemData {
        unsafe { wxcTreeItemData { ptr: wxcTreeItemData_Create(closure.ptr()) } }
    }
}

pub trait _wxcTreeItemData : _wxTreeItemData {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientClosure(&self) -> wxClosure {
        unsafe { wxClosure { ptr: wxcTreeItemData_GetClientClosure(self.ptr()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientClosure<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxcTreeItemData_SetClientClosure(self.ptr(), closure.ptr()) }
    }
}

pub struct wxInputSink { ptr: *mut c_void }
impl _wxInputSink for wxInputSink {}
impl _wxThread for wxInputSink { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxInputSink {
    pub fn from(ptr: *mut c_void) -> wxInputSink { wxInputSink { ptr: ptr } }
    pub fn null() -> wxInputSink { wxInputSink::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream, U: _wxEvtHandler>(input: &T, evtHandler: &U, bufferLen: c_int) -> wxInputSink {
        unsafe { wxInputSink { ptr: wxInputSink_Create(input.ptr(), evtHandler.ptr(), bufferLen) } }
    }
}

pub trait _wxInputSink : _wxThread {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self) {
        unsafe { wxInputSink_Start(self.ptr()) }
    }
}

pub struct wxInputSinkEvent { ptr: *mut c_void }
impl _wxInputSinkEvent for wxInputSinkEvent {}
impl _wxEvent for wxInputSinkEvent {}
impl _wxObject for wxInputSinkEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxInputSinkEvent {
    pub fn from(ptr: *mut c_void) -> wxInputSinkEvent { wxInputSinkEvent { ptr: ptr } }
    pub fn null() -> wxInputSinkEvent { wxInputSinkEvent::from(0 as *mut c_void) }
    
}

pub trait _wxInputSinkEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastError(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastError(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastRead(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastRead(self.ptr()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastInput(&self) -> *mut c_char {
        unsafe { wxInputSinkEvent_LastInput(self.ptr()) }
    }
}

