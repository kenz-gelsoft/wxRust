use std::libc::*;
use _unsafe::*;
use base::*;

pub struct ELJApp(*mut c_void);
impl _ELJApp for ELJApp {}
impl _wxApp for ELJApp {}
impl _wxEvtHandler for ELJApp {}
impl _wxObject for ELJApp { fn handle(&self) -> *mut c_void { **self } }

impl ELJApp {
    pub fn from(handle: *mut c_void) -> @ELJApp { @ELJApp(handle) }
    pub fn null() -> @ELJApp { ELJApp::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newLogTarget() -> @ELJLog {
        unsafe { @ELJLog(ELJApp_CreateLogTarget()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn displaySize() -> @wxSize {
        unsafe { @wxSize(ELJApp_DisplaySize()) }
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
        unsafe { ELJApp_ExecuteProcess(_cmd.handle(), _snc, _prc.handle()) }
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
        unsafe { ELJApp_FindWindowById(_id, _prt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findWindowByLabel<T: _wxWindow>(_lbl: &str, _prt: &T) -> @wxWindow {
        let _lbl = wxT(_lbl);
        unsafe { @wxWindow(ELJApp_FindWindowByLabel(_lbl.handle(), _prt.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findWindowByName<T: _wxWindow>(_lbl: &str, _prt: &T) -> @wxWindow {
        let _lbl = wxT(_lbl);
        unsafe { @wxWindow(ELJApp_FindWindowByName(_lbl.handle(), _prt.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getApp() -> @wxApp {
        unsafe { @wxApp(ELJApp_GetApp()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getAppName() -> ~str {
        unsafe { wxString { handle: ELJApp_GetAppName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getClassName() -> ~str {
        unsafe { wxString { handle: ELJApp_GetClassName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getOsDescription() -> ~str {
        unsafe { wxString { handle: ELJApp_GetOsDescription() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getOsVersion(_maj: *mut c_void, _min: *mut c_void) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getTopWindow() -> @wxWindow {
        unsafe { @wxWindow(ELJApp_GetTopWindow()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUserHome(_usr: *mut c_void) -> ~str {
        unsafe { wxString { handle: ELJApp_GetUserHome(_usr) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUserId() -> ~str {
        unsafe { wxString { handle: ELJApp_GetUserId() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getUserName() -> ~str {
        unsafe { wxString { handle: ELJApp_GetUserName() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getVendorName() -> ~str {
        unsafe { wxString { handle: ELJApp_GetVendorName() }.to_str() }
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
    pub fn mousePosition() -> @wxPoint {
        unsafe { @wxPoint(ELJApp_MousePosition()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn safeYield<T: _wxWindow>(_win: &T) -> c_int {
        unsafe { ELJApp_SafeYield(_win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setAppName(name: &str) {
        let name = wxT(name);
        unsafe { ELJApp_SetAppName(name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setClassName(name: &str) {
        let name = wxT(name);
        unsafe { ELJApp_SetClassName(name.handle()) }
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
        unsafe { ELJApp_SetTopWindow(_wnd.handle()) }
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
        unsafe { ELJApp_SetVendorName(name.handle()) }
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
        unsafe { ELJApp_InitializeC(closure.handle(), _argc, _argv) }
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

pub struct ELJLog(*mut c_void);
impl _ELJLog for ELJLog {}
impl _wxLog for ELJLog { fn handle(&self) -> *mut c_void { **self } }

impl ELJLog {
    pub fn from(handle: *mut c_void) -> @ELJLog { @ELJLog(handle) }
    pub fn null() -> @ELJLog { ELJLog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> @ELJLog {
        unsafe { @ELJLog(ELJLog_Create(_obj, _fnc)) }
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
        unsafe { ELJLog_EnableLogging(self.handle(), doIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> c_int {
        unsafe { ELJLog_IsEnabled(self.handle()) }
    }
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

pub struct wxAcceleratorEntry(*mut c_void);
impl _wxAcceleratorEntry for wxAcceleratorEntry { fn handle(&self) -> *mut c_void { **self } }

impl wxAcceleratorEntry {
    pub fn from(handle: *mut c_void) -> @wxAcceleratorEntry { @wxAcceleratorEntry(handle) }
    pub fn null() -> @wxAcceleratorEntry { wxAcceleratorEntry::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> @wxAcceleratorEntry {
        unsafe { @wxAcceleratorEntry(wxAcceleratorEntry_Create(flags, keyCode, cmd)) }
    }
}

pub trait _wxAcceleratorEntry {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxAcceleratorEntry_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCommand(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetCommand(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKeyCode(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetKeyCode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, flags: c_int, keyCode: c_int, cmd: c_int) {
        unsafe { wxAcceleratorEntry_Set(self.handle(), flags, keyCode, cmd) }
    }
}

pub struct wxAcceleratorTable(*mut c_void);
impl _wxAcceleratorTable for wxAcceleratorTable { fn handle(&self) -> *mut c_void { **self } }

impl wxAcceleratorTable {
    pub fn from(handle: *mut c_void) -> @wxAcceleratorTable { @wxAcceleratorTable(handle) }
    pub fn null() -> @wxAcceleratorTable { wxAcceleratorTable::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(n: c_int, entries: *mut c_void) -> @wxAcceleratorTable {
        unsafe { @wxAcceleratorTable(wxAcceleratorTable_Create(n, entries)) }
    }
}

pub trait _wxAcceleratorTable {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.handle()) }
    }
}

pub struct wxActivateEvent(*mut c_void);
impl _wxActivateEvent for wxActivateEvent {}
impl _wxEvent for wxActivateEvent {}
impl _wxObject for wxActivateEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxActivateEvent {
    pub fn from(handle: *mut c_void) -> @wxActivateEvent { @wxActivateEvent(handle) }
    pub fn null() -> @wxActivateEvent { wxActivateEvent::from(0 as *mut c_void) }
    
}

pub trait _wxActivateEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getActive(&self) -> c_int {
        unsafe { wxActivateEvent_GetActive(self.handle()) }
    }
}

pub struct wxApp(*mut c_void);
impl _wxApp for wxApp {}
impl _wxEvtHandler for wxApp {}
impl _wxObject for wxApp { fn handle(&self) -> *mut c_void { **self } }

impl wxApp {
    pub fn from(handle: *mut c_void) -> @wxApp { @wxApp(handle) }
    pub fn null() -> @wxApp { wxApp::from(0 as *mut c_void) }
    
}

pub trait _wxApp : _wxEvtHandler {
}

pub struct wxArtProvider(*mut c_void);
impl _wxArtProvider for wxArtProvider {}
impl _wxObject for wxArtProvider { fn handle(&self) -> *mut c_void { **self } }

impl wxArtProvider {
    pub fn from(handle: *mut c_void) -> @wxArtProvider { @wxArtProvider(handle) }
    pub fn null() -> @wxArtProvider { wxArtProvider::from(0 as *mut c_void) }
    
}

pub trait _wxArtProvider : _wxObject {
}

pub struct wxAutoBufferedPaintDC(*mut c_void);
impl _wxAutoBufferedPaintDC for wxAutoBufferedPaintDC {}
impl _wxDC for wxAutoBufferedPaintDC {}
impl _wxObject for wxAutoBufferedPaintDC { fn handle(&self) -> *mut c_void { **self } }

impl wxAutoBufferedPaintDC {
    pub fn from(handle: *mut c_void) -> @wxAutoBufferedPaintDC { @wxAutoBufferedPaintDC(handle) }
    pub fn null() -> @wxAutoBufferedPaintDC { wxAutoBufferedPaintDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(window: &T) -> @wxAutoBufferedPaintDC {
        unsafe { @wxAutoBufferedPaintDC(wxAutoBufferedPaintDC_Create(window.handle())) }
    }
}

pub trait _wxAutoBufferedPaintDC : _wxDC {
}

pub struct wxAutomationObject(*mut c_void);
impl _wxAutomationObject for wxAutomationObject {}
impl _wxObject for wxAutomationObject { fn handle(&self) -> *mut c_void { **self } }

impl wxAutomationObject {
    pub fn from(handle: *mut c_void) -> @wxAutomationObject { @wxAutomationObject(handle) }
    pub fn null() -> @wxAutomationObject { wxAutomationObject::from(0 as *mut c_void) }
    
}

pub trait _wxAutomationObject : _wxObject {
}

pub struct wxBitmap(*mut c_void);
impl _wxBitmap for wxBitmap {}
impl _wxGDIObject for wxBitmap {}
impl _wxObject for wxBitmap { fn handle(&self) -> *mut c_void { **self } }

impl wxBitmap {
    pub fn from(handle: *mut c_void) -> @wxBitmap { @wxBitmap(handle) }
    pub fn null() -> @wxBitmap { wxBitmap::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn addHandler<T: _wxEvtHandler>(handler: &T) {
        unsafe { wxBitmap_AddHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_data: *mut c_void, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_Create(_data, _type, _width, _height, _depth)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateEmpty(_width, _height, _depth)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newLoad(name: &str, type_: c_int) -> @wxBitmap {
        let name = wxT(name);
        unsafe { @wxBitmap(wxBitmap_CreateLoad(name.handle(), type_)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findHandlerByName(name: &str) -> *mut c_void {
        let name = wxT(name);
        unsafe { wxBitmap_FindHandlerByName(name.handle()) }
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
        unsafe { wxBitmap_InsertHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn removeHandler(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_RemoveHandler(name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromImage<T: _wxImage>(image: &T, depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromImage(image.handle(), depth)) }
    }
}

pub trait _wxBitmap : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newFromXPM(&self) -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromXPM(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findHandlerByExtension(&self, type_: c_int) -> *mut c_void {
        unsafe { wxBitmap_FindHandlerByExtension(self.handle(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDepth(&self) -> c_int {
        unsafe { wxBitmap_GetDepth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxBitmap_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMask(&self) -> @wxMask {
        unsafe { @wxMask(wxBitmap_GetMask(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSubBitmap<T: _wxBitmap>(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxBitmap_GetSubBitmap(self.handle(), x, y, w, h, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxBitmap_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile<T: _wxPalette>(&self, name: &str, type_: c_int, cmap: &T) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_SaveFile(self.handle(), name.handle(), type_, cmap.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDepth(&self, d: c_int) {
        unsafe { wxBitmap_SetDepth(self.handle(), d) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHeight(&self, h: c_int) {
        unsafe { wxBitmap_SetHeight(self.handle(), h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMask<T: _wxMask>(&self, mask: &T) {
        unsafe { wxBitmap_SetMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, w: c_int) {
        unsafe { wxBitmap_SetWidth(self.handle(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxBitmap_IsStatic(self.handle()) }
    }
}

pub struct wxBitmapButton(*mut c_void);
impl _wxBitmapButton for wxBitmapButton {}
impl _wxButton for wxBitmapButton {}
impl _wxControl for wxBitmapButton {}
impl _wxWindow for wxBitmapButton {}
impl _wxEvtHandler for wxBitmapButton {}
impl _wxObject for wxBitmapButton { fn handle(&self) -> *mut c_void { **self } }

impl wxBitmapButton {
    pub fn from(handle: *mut c_void) -> @wxBitmapButton { @wxBitmapButton(handle) }
    pub fn null() -> @wxBitmapButton { wxBitmapButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxBitmapButton {
        unsafe { @wxBitmapButton(wxBitmapButton_Create(_prt.handle(), _id, _bmp.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxBitmapButton : _wxButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapDisabled<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapFocus<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapLabel<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapSelected<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapSelected(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginX(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginY(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapDisabled<T: _wxBitmap>(&self, disabled: &T) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.handle(), disabled.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapFocus<T: _wxBitmap>(&self, focus: &T) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.handle(), focus.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapLabel<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapSelected<T: _wxBitmap>(&self, sel: &T) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.handle(), sel.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.handle(), x, y) }
    }
}

pub struct wxBitmapToggleButton(*mut c_void);
impl _wxBitmapToggleButton for wxBitmapToggleButton {}
impl _wxToggleButton for wxBitmapToggleButton {}
impl _wxControl for wxBitmapToggleButton {}
impl _wxWindow for wxBitmapToggleButton {}
impl _wxEvtHandler for wxBitmapToggleButton {}
impl _wxObject for wxBitmapToggleButton { fn handle(&self) -> *mut c_void { **self } }

impl wxBitmapToggleButton {
    pub fn from(handle: *mut c_void) -> @wxBitmapToggleButton { @wxBitmapToggleButton(handle) }
    pub fn null() -> @wxBitmapToggleButton { wxBitmapToggleButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(parent: &T, id: c_int, _bmp: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxBitmapToggleButton {
        unsafe { @wxBitmapToggleButton(wxBitmapToggleButton_Create(parent.handle(), id, _bmp.handle(), x, y, w, h, style)) }
    }
}

pub trait _wxBitmapToggleButton : _wxToggleButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapLabel<T: _wxBitmap>(&self, _bmp: &T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.handle(), _bmp.handle()) }
    }
}

pub struct wxBitmapDataObject(*mut c_void);
impl _wxBitmapDataObject for wxBitmapDataObject {}
impl _wxDataObjectSimple for wxBitmapDataObject {}
impl _wxDataObject for wxBitmapDataObject { fn handle(&self) -> *mut c_void { **self } }

impl wxBitmapDataObject {
    pub fn from(handle: *mut c_void) -> @wxBitmapDataObject { @wxBitmapDataObject(handle) }
    pub fn null() -> @wxBitmapDataObject { wxBitmapDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxBitmapDataObject : _wxDataObjectSimple {
}

pub struct wxBitmapHandler(*mut c_void);
impl _wxBitmapHandler for wxBitmapHandler {}
impl _wxObject for wxBitmapHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxBitmapHandler {
    pub fn from(handle: *mut c_void) -> @wxBitmapHandler { @wxBitmapHandler(handle) }
    pub fn null() -> @wxBitmapHandler { wxBitmapHandler::from(0 as *mut c_void) }
    
}

pub trait _wxBitmapHandler : _wxObject {
}

pub struct wxBoxSizer(*mut c_void);
impl _wxBoxSizer for wxBoxSizer {}
impl _wxSizer for wxBoxSizer {}
impl _wxObject for wxBoxSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxBoxSizer {
    pub fn from(handle: *mut c_void) -> @wxBoxSizer { @wxBoxSizer(handle) }
    pub fn null() -> @wxBoxSizer { wxBoxSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(orient: c_int) -> @wxBoxSizer {
        unsafe { @wxBoxSizer(wxBoxSizer_Create(orient)) }
    }
}

pub trait _wxBoxSizer : _wxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.handle()) }
    }
}

pub struct wxBrush(*mut c_void);
impl _wxBrush for wxBrush {}
impl _wxGDIObject for wxBrush {}
impl _wxObject for wxBrush { fn handle(&self) -> *mut c_void { **self } }

impl wxBrush {
    pub fn from(handle: *mut c_void) -> @wxBrush { @wxBrush(handle) }
    pub fn null() -> @wxBrush { wxBrush::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxBrush {
        unsafe { @wxBrush(wxBrush_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(bitmap: &T) -> @wxBrush {
        unsafe { @wxBrush(wxBrush_CreateFromBitmap(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromColour<T: _wxColour>(col: &T, style: c_int) -> @wxBrush {
        unsafe { @wxBrush(wxBrush_CreateFromColour(col.handle(), style)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> @wxBrush {
        unsafe { @wxBrush(wxBrush_CreateFromStock(id)) }
    }
}

pub trait _wxBrush : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxBrush_Assign(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxBrush_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStipple<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxBrush_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxBrush>(&self, brush: &T) -> c_int {
        unsafe { wxBrush_IsEqual(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxBrush_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxBrush_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxBrush_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStipple<T: _wxBitmap>(&self, stipple: &T) {
        unsafe { wxBrush_SetStipple(self.handle(), stipple.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxBrush_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxBrush_IsStatic(self.handle()) }
    }
}

pub struct wxBrushList(*mut c_void);
impl _wxBrushList for wxBrushList {}
impl _wxList for wxBrushList {}
impl _wxObject for wxBrushList { fn handle(&self) -> *mut c_void { **self } }

impl wxBrushList {
    pub fn from(handle: *mut c_void) -> @wxBrushList { @wxBrushList(handle) }
    pub fn null() -> @wxBrushList { wxBrushList::from(0 as *mut c_void) }
    
}

pub trait _wxBrushList : _wxList {
}

pub struct wxBufferedDC(*mut c_void);
impl _wxBufferedDC for wxBufferedDC {}
impl _wxDC for wxBufferedDC {}
impl _wxObject for wxBufferedDC { fn handle(&self) -> *mut c_void { **self } }

impl wxBufferedDC {
    pub fn from(handle: *mut c_void) -> @wxBufferedDC { @wxBufferedDC(handle) }
    pub fn null() -> @wxBufferedDC { wxBufferedDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newByDCAndSize<T: _wxDC>(dc: &T, width: c_int, hight: c_int, style: c_int) -> @wxBufferedDC {
        unsafe { @wxBufferedDC(wxBufferedDC_CreateByDCAndSize(dc.handle(), width, hight, style)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newByDCAndBitmap<T: _wxDC, U: _wxBitmap>(dc: &T, bitmap: &U, style: c_int) -> @wxBufferedDC {
        unsafe { @wxBufferedDC(wxBufferedDC_CreateByDCAndBitmap(dc.handle(), bitmap.handle(), style)) }
    }
}

pub trait _wxBufferedDC : _wxDC {
}

pub struct wxBufferedPaintDC(*mut c_void);
impl _wxBufferedPaintDC for wxBufferedPaintDC {}
impl _wxDC for wxBufferedPaintDC {}
impl _wxObject for wxBufferedPaintDC { fn handle(&self) -> *mut c_void { **self } }

impl wxBufferedPaintDC {
    pub fn from(handle: *mut c_void) -> @wxBufferedPaintDC { @wxBufferedPaintDC(handle) }
    pub fn null() -> @wxBufferedPaintDC { wxBufferedPaintDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(window: &T, style: c_int) -> @wxBufferedPaintDC {
        unsafe { @wxBufferedPaintDC(wxBufferedPaintDC_Create(window.handle(), style)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithBitmap<T: _wxWindow, U: _wxBitmap>(window: &T, bitmap: &U, style: c_int) -> @wxBufferedPaintDC {
        unsafe { @wxBufferedPaintDC(wxBufferedPaintDC_CreateWithBitmap(window.handle(), bitmap.handle(), style)) }
    }
}

pub trait _wxBufferedPaintDC : _wxDC {
}

pub struct wxBusyCursor(*mut c_void);
impl _wxBusyCursor for wxBusyCursor { fn handle(&self) -> *mut c_void { **self } }

impl wxBusyCursor {
    pub fn from(handle: *mut c_void) -> @wxBusyCursor { @wxBusyCursor(handle) }
    pub fn null() -> @wxBusyCursor { wxBusyCursor::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxBusyCursor {
        unsafe { @wxBusyCursor(wxBusyCursor_Create()) }
    }
}

pub trait _wxBusyCursor {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn newWithCursor(&self) -> *mut c_void {
        unsafe { wxBusyCursor_CreateWithCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.handle()) }
    }
}

pub struct wxBusyInfo(*mut c_void);
impl _wxBusyInfo for wxBusyInfo { fn handle(&self) -> *mut c_void { **self } }

impl wxBusyInfo {
    pub fn from(handle: *mut c_void) -> @wxBusyInfo { @wxBusyInfo(handle) }
    pub fn null() -> @wxBusyInfo { wxBusyInfo::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_txt: &str) -> @wxBusyInfo {
        let _txt = wxT(_txt);
        unsafe { @wxBusyInfo(wxBusyInfo_Create(_txt.handle())) }
    }
}

pub trait _wxBusyInfo {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.handle()) }
    }
}

pub struct wxButton(*mut c_void);
impl _wxButton for wxButton {}
impl _wxControl for wxButton {}
impl _wxWindow for wxButton {}
impl _wxEvtHandler for wxButton {}
impl _wxObject for wxButton { fn handle(&self) -> *mut c_void { **self } }

impl wxButton {
    pub fn from(handle: *mut c_void) -> @wxButton { @wxButton(handle) }
    pub fn null() -> @wxButton { wxButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxButton {
        let _txt = wxT(_txt);
        unsafe { @wxButton(wxButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.handle()) }
    }
}

pub struct wxCaret(*mut c_void);
impl _wxCaret for wxCaret { fn handle(&self) -> *mut c_void { **self } }

impl wxCaret {
    pub fn from(handle: *mut c_void) -> @wxCaret { @wxCaret(handle) }
    pub fn null() -> @wxCaret { wxCaret::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_wnd: &T, _wth: c_int, _hgt: c_int) -> @wxCaret {
        unsafe { @wxCaret(wxCaret_Create(_wnd.handle(), _wth, _hgt)) }
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
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxCaret_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxCaret_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxCaret_GetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) {
        unsafe { wxCaret_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxCaret_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible(&self) -> c_int {
        unsafe { wxCaret_IsVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxCaret_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSize(&self, width: c_int, height: c_int) {
        unsafe { wxCaret_SetSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) {
        unsafe { wxCaret_Show(self.handle()) }
    }
}

pub struct wxCheckBox(*mut c_void);
impl _wxCheckBox for wxCheckBox {}
impl _wxControl for wxCheckBox {}
impl _wxWindow for wxCheckBox {}
impl _wxEvtHandler for wxCheckBox {}
impl _wxObject for wxCheckBox { fn handle(&self) -> *mut c_void { **self } }

impl wxCheckBox {
    pub fn from(handle: *mut c_void) -> @wxCheckBox { @wxCheckBox(handle) }
    pub fn null() -> @wxCheckBox { wxCheckBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxCheckBox {
        let _txt = wxT(_txt);
        unsafe { @wxCheckBox(wxCheckBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxCheckBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxCheckBox_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self.handle(), value) }
    }
}

pub struct wxCheckListBox(*mut c_void);
impl _wxCheckListBox for wxCheckListBox {}
impl _wxListBox for wxCheckListBox {}
impl _wxControl for wxCheckListBox {}
impl _wxWindow for wxCheckListBox {}
impl _wxEvtHandler for wxCheckListBox {}
impl _wxObject for wxCheckListBox { fn handle(&self) -> *mut c_void { **self } }

impl wxCheckListBox {
    pub fn from(handle: *mut c_void) -> @wxCheckListBox { @wxCheckListBox(handle) }
    pub fn null() -> @wxCheckListBox { wxCheckListBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> @wxCheckListBox {
        unsafe { @wxCheckListBox(wxCheckListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

pub trait _wxCheckListBox : _wxListBox {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, item: c_int, check: c_int) {
        unsafe { wxCheckListBox_Check(self.handle(), item, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, item: c_int) -> c_int {
        unsafe { wxCheckListBox_IsChecked(self.handle(), item) }
    }
}

pub struct wxChoice(*mut c_void);
impl _wxChoice for wxChoice {}
impl _wxControl for wxChoice {}
impl _wxWindow for wxChoice {}
impl _wxEvtHandler for wxChoice {}
impl _wxObject for wxChoice { fn handle(&self) -> *mut c_void { **self } }

impl wxChoice {
    pub fn from(handle: *mut c_void) -> @wxChoice { @wxChoice(handle) }
    pub fn null() -> @wxChoice { wxChoice::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> @wxChoice {
        unsafe { @wxChoice(wxChoice_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

pub trait _wxChoice : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxChoice_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxChoice_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxChoice_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxChoice_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxChoice_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, n: c_int) -> ~str {
        unsafe { wxString { handle: wxChoice_GetString(self.handle(), n) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.handle(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setString(&self, n: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxChoice_SetString(self.handle(), n, s.handle()) }
    }
}

pub struct wxClientDC(*mut c_void);
impl _wxClientDC for wxClientDC {}
impl _wxWindowDC for wxClientDC {}
impl _wxDC for wxClientDC {}
impl _wxObject for wxClientDC { fn handle(&self) -> *mut c_void { **self } }

impl wxClientDC {
    pub fn from(handle: *mut c_void) -> @wxClientDC { @wxClientDC(handle) }
    pub fn null() -> @wxClientDC { wxClientDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> @wxClientDC {
        unsafe { @wxClientDC(wxClientDC_Create(win.handle())) }
    }
}

pub trait _wxClientDC : _wxWindowDC {
}

pub struct wxClipboard(*mut c_void);
impl _wxClipboard for wxClipboard {}
impl _wxObject for wxClipboard { fn handle(&self) -> *mut c_void { **self } }

impl wxClipboard {
    pub fn from(handle: *mut c_void) -> @wxClipboard { @wxClipboard(handle) }
    pub fn null() -> @wxClipboard { wxClipboard::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxClipboard {
        unsafe { @wxClipboard(wxClipboard_Create()) }
    }
}

pub trait _wxClipboard : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addData<T: _wxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_AddData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxClipboard_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self) {
        unsafe { wxClipboard_Close(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flush(&self) -> c_int {
        unsafe { wxClipboard_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData<T: _wxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_GetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOpened(&self) -> c_int {
        unsafe { wxClipboard_IsOpened(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSupported<T: _wxDataFormat>(&self, format: &T) -> c_int {
        unsafe { wxClipboard_IsSupported(self.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn open(&self) -> c_int {
        unsafe { wxClipboard_Open(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData<T: _wxDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_SetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn usePrimarySelection(&self, primary: c_int) {
        unsafe { wxClipboard_UsePrimarySelection(self.handle(), primary) }
    }
}

pub struct wxCloseEvent(*mut c_void);
impl _wxCloseEvent for wxCloseEvent {}
impl _wxEvent for wxCloseEvent {}
impl _wxObject for wxCloseEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxCloseEvent {
    pub fn from(handle: *mut c_void) -> @wxCloseEvent { @wxCloseEvent(handle) }
    pub fn null() -> @wxCloseEvent { wxCloseEvent::from(0 as *mut c_void) }
    
}

pub trait _wxCloseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn canVeto(&self) -> c_int {
        unsafe { wxCloseEvent_CanVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLoggingOff(&self) -> c_int {
        unsafe { wxCloseEvent_GetLoggingOff(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVeto(&self) -> c_int {
        unsafe { wxCloseEvent_GetVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCanVeto(&self, canVeto: c_int) {
        unsafe { wxCloseEvent_SetCanVeto(self.handle(), canVeto) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLoggingOff(&self, logOff: c_int) {
        unsafe { wxCloseEvent_SetLoggingOff(self.handle(), logOff) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn veto(&self, veto: c_int) {
        unsafe { wxCloseEvent_Veto(self.handle(), veto) }
    }
}

pub struct wxColour(*mut c_void);
impl _wxColour for wxColour {}
impl _wxObject for wxColour { fn handle(&self) -> *mut c_void { **self } }

impl wxColour {
    pub fn from(handle: *mut c_void) -> @wxColour { @wxColour(handle) }
    pub fn null() -> @wxColour { wxColour::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newByName(_name: &str) -> @wxColour {
        let _name = wxT(_name);
        unsafe { @wxColour(wxColour_CreateByName(_name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEmpty() -> @wxColour {
        unsafe { @wxColour(wxColour_CreateEmpty()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> @wxColour {
        unsafe { @wxColour(wxColour_CreateFromStock(id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> @wxColour {
        unsafe { @wxColour(wxColour_CreateRGB(_red, _green, _blue, _alpha)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn validName(_name: *mut c_void) -> c_int {
        unsafe { wxColour_ValidName(_name) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromInt(rgb: c_int) -> @wxColour {
        unsafe { @wxColour(wxColour_CreateFromInt(rgb)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromUnsignedInt(rgba: uint32_t) -> @wxColour {
        unsafe { @wxColour(wxColour_CreateFromUnsignedInt(rgba)) }
    }
}

pub trait _wxColour : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn alpha(&self) -> uint8_t {
        unsafe { wxColour_Alpha(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign(&self, other: *mut c_void) {
        unsafe { wxColour_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn blue(&self) -> uint8_t {
        unsafe { wxColour_Blue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self, _other: *mut c_void) {
        unsafe { wxColour_Copy(self.handle(), _other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn green(&self) -> uint8_t {
        unsafe { wxColour_Green(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxColour_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn red(&self) -> uint8_t {
        unsafe { wxColour_Red(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, _red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) {
        unsafe { wxColour_Set(self.handle(), _red, _green, _blue, _alpha) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setByName(&self, _name: &str) {
        let _name = wxT(_name);
        unsafe { wxColour_SetByName(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxColour_IsStatic(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInt(&self) -> c_int {
        unsafe { wxColour_GetInt(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUnsignedInt(&self) -> uint32_t {
        unsafe { wxColour_GetUnsignedInt(self.handle()) }
    }
}

pub struct wxColourData(*mut c_void);
impl _wxColourData for wxColourData {}
impl _wxObject for wxColourData { fn handle(&self) -> *mut c_void { **self } }

impl wxColourData {
    pub fn from(handle: *mut c_void) -> @wxColourData { @wxColourData(handle) }
    pub fn null() -> @wxColourData { wxColourData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxColourData {
        unsafe { @wxColourData(wxColourData_Create()) }
    }
}

pub trait _wxColourData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChooseFull(&self) -> c_int {
        unsafe { wxColourData_GetChooseFull(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxColourData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCustomColour<T: _wxColour>(&self, i: c_int, _ref: &T) {
        unsafe { wxColourData_GetCustomColour(self.handle(), i, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChooseFull(&self, flag: c_int) {
        unsafe { wxColourData_SetChooseFull(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxColourData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCustomColour<T: _wxColour>(&self, i: c_int, colour: &T) {
        unsafe { wxColourData_SetCustomColour(self.handle(), i, colour.handle()) }
    }
}

pub struct wxColourDatabase(*mut c_void);
impl _wxColourDatabase for wxColourDatabase {}
impl _wxList for wxColourDatabase {}
impl _wxObject for wxColourDatabase { fn handle(&self) -> *mut c_void { **self } }

impl wxColourDatabase {
    pub fn from(handle: *mut c_void) -> @wxColourDatabase { @wxColourDatabase(handle) }
    pub fn null() -> @wxColourDatabase { wxColourDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxColourDatabase : _wxList {
}

pub struct wxColourDialog(*mut c_void);
impl _wxColourDialog for wxColourDialog {}
impl _wxDialog for wxColourDialog {}
impl _wxTopLevelWindow for wxColourDialog {}
impl _wxWindow for wxColourDialog {}
impl _wxEvtHandler for wxColourDialog {}
impl _wxObject for wxColourDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxColourDialog {
    pub fn from(handle: *mut c_void) -> @wxColourDialog { @wxColourDialog(handle) }
    pub fn null() -> @wxColourDialog { wxColourDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxColourData>(_prt: &T, col: &U) -> @wxColourDialog {
        unsafe { @wxColourDialog(wxColourDialog_Create(_prt.handle(), col.handle())) }
    }
}

pub trait _wxColourDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColourData<T: _wxColourData>(&self, _ref: &T) {
        unsafe { wxColourDialog_GetColourData(self.handle(), _ref.handle()) }
    }
}

pub struct wxComboBox(*mut c_void);
impl _wxComboBox for wxComboBox {}
impl _wxChoice for wxComboBox {}
impl _wxControl for wxComboBox {}
impl _wxWindow for wxComboBox {}
impl _wxEvtHandler for wxComboBox {}
impl _wxObject for wxComboBox { fn handle(&self) -> *mut c_void { **self } }

impl wxComboBox {
    pub fn from(handle: *mut c_void) -> @wxComboBox { @wxComboBox(handle) }
    pub fn null() -> @wxComboBox { wxComboBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> @wxComboBox {
        let _txt = wxT(_txt);
        unsafe { @wxComboBox(wxComboBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

pub trait _wxComboBox : _wxChoice {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendData(&self, item: &str, d: *mut c_void) {
        let item = wxT(item);
        unsafe { wxComboBox_AppendData(self.handle(), item.handle(), d) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self) {
        unsafe { wxComboBox_Copy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cut(&self) {
        unsafe { wxComboBox_Cut(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInsertionPoint(&self) -> c_int {
        unsafe { wxComboBox_GetInsertionPoint(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastPosition(&self) -> c_int {
        unsafe { wxComboBox_GetLastPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStringSelection(&self) -> ~str {
        unsafe { wxString { handle: wxComboBox_GetStringSelection(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> ~str {
        unsafe { wxString { handle: wxComboBox_GetValue(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste(&self) {
        unsafe { wxComboBox_Paste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_Remove(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace(&self, from: c_int, to: c_int, value: &str) {
        let value = wxT(value);
        unsafe { wxComboBox_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEditable(&self, editable: c_int) {
        unsafe { wxComboBox_SetEditable(self.handle(), editable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPoint(&self, pos: c_int) {
        unsafe { wxComboBox_SetInsertionPoint(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPointEnd(&self) {
        unsafe { wxComboBox_SetInsertionPointEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextSelection(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_SetTextSelection(self.handle(), from, to) }
    }
}

pub struct wxCommand(*mut c_void);
impl _wxCommand for wxCommand {}
impl _wxObject for wxCommand { fn handle(&self) -> *mut c_void { **self } }

impl wxCommand {
    pub fn from(handle: *mut c_void) -> @wxCommand { @wxCommand(handle) }
    pub fn null() -> @wxCommand { wxCommand::from(0 as *mut c_void) }
    
}

pub trait _wxCommand : _wxObject {
}

pub struct wxCommandEvent(*mut c_void);
impl _wxCommandEvent for wxCommandEvent {}
impl _wxEvent for wxCommandEvent {}
impl _wxObject for wxCommandEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxCommandEvent {
    pub fn from(handle: *mut c_void) -> @wxCommandEvent { @wxCommandEvent(handle) }
    pub fn null() -> @wxCommandEvent { wxCommandEvent::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_typ: c_int, _id: c_int) -> @wxCommandEvent {
        unsafe { @wxCommandEvent(wxCommandEvent_Create(_typ, _id)) }
    }
}

pub trait _wxCommandEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientData(&self) -> @wxClientData {
        unsafe { @wxClientData(wxCommandEvent_GetClientData(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientObject(&self) -> @wxClientData {
        unsafe { @wxClientData(wxCommandEvent_GetClientObject(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExtraLong(&self) -> c_long {
        unsafe { wxCommandEvent_GetExtraLong(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInt(&self) -> c_long {
        unsafe { wxCommandEvent_GetInt(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxCommandEvent_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self) -> ~str {
        unsafe { wxString { handle: wxCommandEvent_GetString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self) -> c_int {
        unsafe { wxCommandEvent_IsChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelection(&self) -> c_int {
        unsafe { wxCommandEvent_IsSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientData<T: _wxClientData>(&self, clientData: &T) {
        unsafe { wxCommandEvent_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientObject<T: _wxClientData>(&self, clientObject: &T) {
        unsafe { wxCommandEvent_SetClientObject(self.handle(), clientObject.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExtraLong(&self, extraLong: c_long) {
        unsafe { wxCommandEvent_SetExtraLong(self.handle(), extraLong) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInt(&self, i: c_int) {
        unsafe { wxCommandEvent_SetInt(self.handle(), i) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setString(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxCommandEvent_SetString(self.handle(), s.handle()) }
    }
}

pub struct wxCommandProcessor(*mut c_void);
impl _wxCommandProcessor for wxCommandProcessor {}
impl _wxObject for wxCommandProcessor { fn handle(&self) -> *mut c_void { **self } }

impl wxCommandProcessor {
    pub fn from(handle: *mut c_void) -> @wxCommandProcessor { @wxCommandProcessor(handle) }
    pub fn null() -> @wxCommandProcessor { wxCommandProcessor::from(0 as *mut c_void) }
    
}

pub trait _wxCommandProcessor : _wxObject {
}

pub struct wxContextHelp(*mut c_void);
impl _wxContextHelp for wxContextHelp {}
impl _wxObject for wxContextHelp { fn handle(&self) -> *mut c_void { **self } }

impl wxContextHelp {
    pub fn from(handle: *mut c_void) -> @wxContextHelp { @wxContextHelp(handle) }
    pub fn null() -> @wxContextHelp { wxContextHelp::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T, beginHelp: c_int) -> @wxContextHelp {
        unsafe { @wxContextHelp(wxContextHelp_Create(win.handle(), beginHelp)) }
    }
}

pub trait _wxContextHelp : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginContextHelp<T: _wxWindow>(&self, win: &T) -> c_int {
        unsafe { wxContextHelp_BeginContextHelp(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endContextHelp(&self) -> c_int {
        unsafe { wxContextHelp_EndContextHelp(self.handle()) }
    }
}

pub struct wxContextHelpButton(*mut c_void);
impl _wxContextHelpButton for wxContextHelpButton {}
impl _wxBitmapButton for wxContextHelpButton {}
impl _wxButton for wxContextHelpButton {}
impl _wxControl for wxContextHelpButton {}
impl _wxWindow for wxContextHelpButton {}
impl _wxEvtHandler for wxContextHelpButton {}
impl _wxObject for wxContextHelpButton { fn handle(&self) -> *mut c_void { **self } }

impl wxContextHelpButton {
    pub fn from(handle: *mut c_void) -> @wxContextHelpButton { @wxContextHelpButton(handle) }
    pub fn null() -> @wxContextHelpButton { wxContextHelpButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @wxContextHelpButton {
        unsafe { @wxContextHelpButton(wxContextHelpButton_Create(parent.handle(), id, x, y, w, h, style)) }
    }
}

pub trait _wxContextHelpButton : _wxBitmapButton {
}

pub struct wxControl(*mut c_void);
impl _wxControl for wxControl {}
impl _wxWindow for wxControl {}
impl _wxEvtHandler for wxControl {}
impl _wxObject for wxControl { fn handle(&self) -> *mut c_void { **self } }

impl wxControl {
    pub fn from(handle: *mut c_void) -> @wxControl { @wxControl(handle) }
    pub fn null() -> @wxControl { wxControl::from(0 as *mut c_void) }
    
}

pub trait _wxControl : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn command<T: _wxEvent>(&self, event: &T) {
        unsafe { wxControl_Command(self.handle(), event.handle()) }
    }
}

pub struct wxCursor(*mut c_void);
impl _wxCursor for wxCursor {}
impl _wxBitmap for wxCursor {}
impl _wxGDIObject for wxCursor {}
impl _wxObject for wxCursor { fn handle(&self) -> *mut c_void { **self } }

impl wxCursor {
    pub fn from(handle: *mut c_void) -> @wxCursor { @wxCursor(handle) }
    pub fn null() -> @wxCursor { wxCursor::from(0 as *mut c_void) }
    
}

pub trait _wxCursor : _wxBitmap {
}

pub struct wxCustomDataObject(*mut c_void);
impl _wxCustomDataObject for wxCustomDataObject {}
impl _wxDataObjectSimple for wxCustomDataObject {}
impl _wxDataObject for wxCustomDataObject { fn handle(&self) -> *mut c_void { **self } }

impl wxCustomDataObject {
    pub fn from(handle: *mut c_void) -> @wxCustomDataObject { @wxCustomDataObject(handle) }
    pub fn null() -> @wxCustomDataObject { wxCustomDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxCustomDataObject : _wxDataObjectSimple {
}

pub struct wxDC(*mut c_void);
impl _wxDC for wxDC {}
impl _wxObject for wxDC { fn handle(&self) -> *mut c_void { **self } }

impl wxDC {
    pub fn from(handle: *mut c_void) -> @wxDC { @wxDC(handle) }
    pub fn null() -> @wxDC { wxDC::from(0 as *mut c_void) }
    
}

pub trait _wxDC : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn blit<T: _wxDC>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: &T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: c_int) -> c_int {
        unsafe { wxDC_Blit(self.handle(), xdest, ydest, width, height, source.handle(), xsrc, ysrc, rop, useMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcBoundingBox(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CalcBoundingBox(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canDrawBitmap(&self) -> c_int {
        unsafe { wxDC_CanDrawBitmap(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canGetTextExtent(&self) -> c_int {
        unsafe { wxDC_CanGetTextExtent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxDC_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn computeScaleAndOrigin(&self) {
        unsafe { wxDC_ComputeScaleAndOrigin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn crossHair(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CrossHair(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyClippingRegion(&self) {
        unsafe { wxDC_DestroyClippingRegion(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalX(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalXRel(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalY(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deviceToLogicalYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalYRel(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawArc(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, xc: c_int, yc: c_int) {
        unsafe { wxDC_DrawArc(self.handle(), x1, y1, x2, y2, xc, yc) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBitmap<T: _wxBitmap>(&self, bmp: &T, x: c_int, y: c_int, useMask: c_int) {
        unsafe { wxDC_DrawBitmap(self.handle(), bmp.handle(), x, y, useMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCheckMark(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawCheckMark(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCircle(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { wxDC_DrawCircle(self.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawEllipse(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawEllipse(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawEllipticArc(&self, x: c_int, y: c_int, w: c_int, h: c_int, sa: c_double, ea: c_double) {
        unsafe { wxDC_DrawEllipticArc(self.handle(), x, y, w, h, sa, ea) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawIcon<T: _wxIcon>(&self, icon: &T, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.handle(), icon.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLabel(&self, str: &str, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        let str = wxT(str);
        unsafe { wxDC_DrawLabel(self.handle(), str.handle(), x, y, w, h, align, indexAccel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLabelBitmap<T: _wxBitmap>(&self, str: &str, bmp: &T, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> @wxRect {
        let str = wxT(str);
        unsafe { @wxRect(wxDC_DrawLabelBitmap(self.handle(), str.handle(), bmp.handle(), x, y, w, h, align, indexAccel)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLine(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { wxDC_DrawLine(self.handle(), x1, y1, x2, y2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLines(&self, n: c_int, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { wxDC_DrawLines(self.handle(), n, x, y, xoffset, yoffset) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPoint(&self, x: c_int, y: c_int) {
        unsafe { wxDC_DrawPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPolygon(&self, n: c_int, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolygon(self.handle(), n, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPolyPolygon(&self, n: c_int, count: *mut c_void, x: *mut c_void, y: *mut c_void, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolyPolygon(self.handle(), n, count, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawRectangle(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRotatedText(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        let text = wxT(text);
        unsafe { wxDC_DrawRotatedText(self.handle(), text.handle(), x, y, angle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.handle(), x, y, width, height, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawText(&self, text: &str, x: c_int, y: c_int) {
        let text = wxT(text);
        unsafe { wxDC_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDoc(&self) {
        unsafe { wxDC_EndDoc(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endPage(&self) {
        unsafe { wxDC_EndPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn floodFill<T: _wxColour>(&self, x: c_int, y: c_int, col: &T, style: c_int) {
        unsafe { wxDC_FloodFill(self.handle(), x, y, col.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackground<T: _wxBrush>(&self, _ref: &T) {
        unsafe { wxDC_GetBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBrush<T: _wxBrush>(&self, _ref: &T) {
        unsafe { wxDC_GetBrush(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharHeight(&self) -> c_int {
        unsafe { wxDC_GetCharHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharWidth(&self) -> c_int {
        unsafe { wxDC_GetCharWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClippingBox(&self, _x: *mut c_void, _y: *mut c_void, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxDC_GetClippingBox(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDepth(&self) -> c_int {
        unsafe { wxDC_GetDepth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDeviceOrigin(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxDC_GetDeviceOrigin(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxDC_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalFunction(&self) -> c_int {
        unsafe { wxDC_GetLogicalFunction(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalOrigin(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxDC_GetLogicalOrigin(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalScale(&self, _x: *mut c_double, _y: *mut c_double) {
        unsafe { wxDC_GetLogicalScale(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMapMode(&self) -> c_int {
        unsafe { wxDC_GetMapMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPI(&self) -> @wxSize {
        unsafe { @wxSize(wxDC_GetPPI(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPen<T: _wxPen>(&self, _ref: &T) {
        unsafe { wxDC_GetPen(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPixel<T: _wxColour>(&self, x: c_int, y: c_int, col: &T) -> c_int {
        unsafe { wxDC_GetPixel(self.handle(), x, y, col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxDC_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizeMM(&self) -> @wxSize {
        unsafe { @wxSize(wxDC_GetSizeMM(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextBackground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxDC_GetTextBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent<T: _wxFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, externalLeading: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetTextExtent(self.handle(), string.handle(), w, h, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMultiLineTextExtent<T: _wxFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, heightLine: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetMultiLineTextExtent(self.handle(), string.handle(), w, h, heightLine, theFont.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextForeground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxDC_GetTextForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserScale(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxDC_GetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceX(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceXRel(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceY(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn logicalToDeviceYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceYRel(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maxX(&self) -> c_int {
        unsafe { wxDC_MaxX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maxY(&self) -> c_int {
        unsafe { wxDC_MaxY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn minX(&self) -> c_int {
        unsafe { wxDC_MinX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn minY(&self) -> c_int {
        unsafe { wxDC_MinY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxDC_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetBoundingBox(&self) {
        unsafe { wxDC_ResetBoundingBox(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAxisOrientation(&self, xLeftRight: c_int, yBottomUp: c_int) {
        unsafe { wxDC_SetAxisOrientation(self.handle(), xLeftRight, yBottomUp) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackground<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBackground(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBrush<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClippingRegionFromRegion<T: _wxRegion>(&self, region: &T) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDeviceClippingRegion<T: _wxRegion>(&self, region: &T) {
        unsafe { wxDC_SetDeviceClippingRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxDC_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLogicalFunction(&self, function: c_int) {
        unsafe { wxDC_SetLogicalFunction(self.handle(), function) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLogicalOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetLogicalOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLogicalScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetLogicalScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMapMode(&self, mode: c_int) {
        unsafe { wxDC_SetMapMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPalette<T: _wxPalette>(&self, palette: &T) {
        unsafe { wxDC_SetPalette(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPen<T: _wxPen>(&self, pen: &T) {
        unsafe { wxDC_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextBackground<T: _wxColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextBackground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextForeground<T: _wxColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextForeground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDoc(&self, msg: &str) -> c_int {
        let msg = wxT(msg);
        unsafe { wxDC_StartDoc(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startPage(&self) {
        unsafe { wxDC_StartPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserScaleX(&self) -> c_double {
        unsafe { wxDC_GetUserScaleX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserScaleY(&self) -> c_double {
        unsafe { wxDC_GetUserScaleY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPixel2<T: _wxColour>(&self, x: c_int, y: c_int, col: &T) {
        unsafe { wxDC_GetPixel2(self.handle(), x, y, col.handle()) }
    }
}

pub struct wxDCClipper(*mut c_void);
impl _wxDCClipper for wxDCClipper { fn handle(&self) -> *mut c_void { **self } }

impl wxDCClipper {
    pub fn from(handle: *mut c_void) -> @wxDCClipper { @wxDCClipper(handle) }
    pub fn null() -> @wxDCClipper { wxDCClipper::from(0 as *mut c_void) }
    
}

pub trait _wxDCClipper {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDataFormat(*mut c_void);
impl _wxDataFormat for wxDataFormat { fn handle(&self) -> *mut c_void { **self } }

impl wxDataFormat {
    pub fn from(handle: *mut c_void) -> @wxDataFormat { @wxDataFormat(handle) }
    pub fn null() -> @wxDataFormat { wxDataFormat::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromId(name: &str) -> @wxDataFormat {
        let name = wxT(name);
        unsafe { @wxDataFormat(wxDataFormat_CreateFromId(name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromType(typ: c_int) -> @wxDataFormat {
        unsafe { @wxDataFormat(wxDataFormat_CreateFromType(typ)) }
    }
}

pub trait _wxDataFormat {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> ~str {
        unsafe { wxString { handle: wxDataFormat_GetId(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getType(&self) -> c_int {
        unsafe { wxDataFormat_GetType(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual(&self, other: *mut c_void) -> c_int {
        unsafe { wxDataFormat_IsEqual(self.handle(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: *mut c_void) {
        unsafe { wxDataFormat_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setType(&self, typ: c_int) {
        unsafe { wxDataFormat_SetType(self.handle(), typ) }
    }
}

pub struct wxDataObject(*mut c_void);
impl _wxDataObject for wxDataObject { fn handle(&self) -> *mut c_void { **self } }

impl wxDataObject {
    pub fn from(handle: *mut c_void) -> @wxDataObject { @wxDataObject(handle) }
    pub fn null() -> @wxDataObject { wxDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxDataObject {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDataObjectComposite(*mut c_void);
impl _wxDataObjectComposite for wxDataObjectComposite {}
impl _wxDataObject for wxDataObjectComposite { fn handle(&self) -> *mut c_void { **self } }

impl wxDataObjectComposite {
    pub fn from(handle: *mut c_void) -> @wxDataObjectComposite { @wxDataObjectComposite(handle) }
    pub fn null() -> @wxDataObjectComposite { wxDataObjectComposite::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxDataObjectComposite {
        unsafe { @wxDataObjectComposite(wxDataObjectComposite_Create()) }
    }
}

pub trait _wxDataObjectComposite : _wxDataObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn add(&self, _dat: *mut c_void, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.handle(), _dat, _preferred) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.handle()) }
    }
}

pub struct wxDataObjectSimple(*mut c_void);
impl _wxDataObjectSimple for wxDataObjectSimple {}
impl _wxDataObject for wxDataObjectSimple { fn handle(&self) -> *mut c_void { **self } }

impl wxDataObjectSimple {
    pub fn from(handle: *mut c_void) -> @wxDataObjectSimple { @wxDataObjectSimple(handle) }
    pub fn null() -> @wxDataObjectSimple { wxDataObjectSimple::from(0 as *mut c_void) }
    
}

pub trait _wxDataObjectSimple : _wxDataObject {
}

pub struct wxDialUpEvent(*mut c_void);
impl _wxDialUpEvent for wxDialUpEvent {}
impl _wxEvent for wxDialUpEvent {}
impl _wxObject for wxDialUpEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxDialUpEvent {
    pub fn from(handle: *mut c_void) -> @wxDialUpEvent { @wxDialUpEvent(handle) }
    pub fn null() -> @wxDialUpEvent { wxDialUpEvent::from(0 as *mut c_void) }
    
}

pub trait _wxDialUpEvent : _wxEvent {
}

pub struct wxDialUpManager(*mut c_void);
impl _wxDialUpManager for wxDialUpManager { fn handle(&self) -> *mut c_void { **self } }

impl wxDialUpManager {
    pub fn from(handle: *mut c_void) -> @wxDialUpManager { @wxDialUpManager(handle) }
    pub fn null() -> @wxDialUpManager { wxDialUpManager::from(0 as *mut c_void) }
    
}

pub trait _wxDialUpManager {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDialog(*mut c_void);
impl _wxDialog for wxDialog {}
impl _wxTopLevelWindow for wxDialog {}
impl _wxWindow for wxDialog {}
impl _wxEvtHandler for wxDialog {}
impl _wxObject for wxDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxDialog {
    pub fn from(handle: *mut c_void) -> @wxDialog { @wxDialog(handle) }
    pub fn null() -> @wxDialog { wxDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDialog {
        let _txt = wxT(_txt);
        unsafe { @wxDialog(wxDialog_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxDialog : _wxTopLevelWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn endModal(&self, retCode: c_int) {
        unsafe { wxDialog_EndModal(self.handle(), retCode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReturnCode(&self) -> c_int {
        unsafe { wxDialog_GetReturnCode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isModal(&self) -> c_int {
        unsafe { wxDialog_IsModal(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReturnCode(&self, returnCode: c_int) {
        unsafe { wxDialog_SetReturnCode(self.handle(), returnCode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showModal(&self) -> c_int {
        unsafe { wxDialog_ShowModal(self.handle()) }
    }
}

pub struct wxDirDialog(*mut c_void);
impl _wxDirDialog for wxDirDialog {}
impl _wxDialog for wxDirDialog {}
impl _wxTopLevelWindow for wxDirDialog {}
impl _wxWindow for wxDirDialog {}
impl _wxEvtHandler for wxDirDialog {}
impl _wxObject for wxDirDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxDirDialog {
    pub fn from(handle: *mut c_void) -> @wxDirDialog { @wxDirDialog(handle) }
    pub fn null() -> @wxDirDialog { wxDirDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _dir: &str, _lft: c_int, _top: c_int, _stl: c_int) -> @wxDirDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        unsafe { @wxDirDialog(wxDirDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _lft, _top, _stl)) }
    }
}

pub trait _wxDirDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMessage(&self) -> ~str {
        unsafe { wxString { handle: wxDirDialog_GetMessage(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPath(&self) -> ~str {
        unsafe { wxString { handle: wxDirDialog_GetPath(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMessage(&self, msg: &str) {
        let msg = wxT(msg);
        unsafe { wxDirDialog_SetMessage(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPath(&self, pth: &str) {
        let pth = wxT(pth);
        unsafe { wxDirDialog_SetPath(self.handle(), pth.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.handle(), style) }
    }
}

pub struct wxDocChildFrame(*mut c_void);
impl _wxDocChildFrame for wxDocChildFrame {}
impl _wxFrame for wxDocChildFrame {}
impl _wxTopLevelWindow for wxDocChildFrame {}
impl _wxWindow for wxDocChildFrame {}
impl _wxEvtHandler for wxDocChildFrame {}
impl _wxObject for wxDocChildFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxDocChildFrame {
    pub fn from(handle: *mut c_void) -> @wxDocChildFrame { @wxDocChildFrame(handle) }
    pub fn null() -> @wxDocChildFrame { wxDocChildFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocChildFrame : _wxFrame {
}

pub struct wxDocMDIChildFrame(*mut c_void);
impl _wxDocMDIChildFrame for wxDocMDIChildFrame {}
impl _wxMDIChildFrame for wxDocMDIChildFrame {}
impl _wxFrame for wxDocMDIChildFrame {}
impl _wxTopLevelWindow for wxDocMDIChildFrame {}
impl _wxWindow for wxDocMDIChildFrame {}
impl _wxEvtHandler for wxDocMDIChildFrame {}
impl _wxObject for wxDocMDIChildFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxDocMDIChildFrame {
    pub fn from(handle: *mut c_void) -> @wxDocMDIChildFrame { @wxDocMDIChildFrame(handle) }
    pub fn null() -> @wxDocMDIChildFrame { wxDocMDIChildFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocMDIChildFrame : _wxMDIChildFrame {
}

pub struct wxDocMDIParentFrame(*mut c_void);
impl _wxDocMDIParentFrame for wxDocMDIParentFrame {}
impl _wxMDIParentFrame for wxDocMDIParentFrame {}
impl _wxFrame for wxDocMDIParentFrame {}
impl _wxTopLevelWindow for wxDocMDIParentFrame {}
impl _wxWindow for wxDocMDIParentFrame {}
impl _wxEvtHandler for wxDocMDIParentFrame {}
impl _wxObject for wxDocMDIParentFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxDocMDIParentFrame {
    pub fn from(handle: *mut c_void) -> @wxDocMDIParentFrame { @wxDocMDIParentFrame(handle) }
    pub fn null() -> @wxDocMDIParentFrame { wxDocMDIParentFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocMDIParentFrame : _wxMDIParentFrame {
}

pub struct wxDocManager(*mut c_void);
impl _wxDocManager for wxDocManager {}
impl _wxEvtHandler for wxDocManager {}
impl _wxObject for wxDocManager { fn handle(&self) -> *mut c_void { **self } }

impl wxDocManager {
    pub fn from(handle: *mut c_void) -> @wxDocManager { @wxDocManager(handle) }
    pub fn null() -> @wxDocManager { wxDocManager::from(0 as *mut c_void) }
    
}

pub trait _wxDocManager : _wxEvtHandler {
}

pub struct wxDocParentFrame(*mut c_void);
impl _wxDocParentFrame for wxDocParentFrame {}
impl _wxFrame for wxDocParentFrame {}
impl _wxTopLevelWindow for wxDocParentFrame {}
impl _wxWindow for wxDocParentFrame {}
impl _wxEvtHandler for wxDocParentFrame {}
impl _wxObject for wxDocParentFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxDocParentFrame {
    pub fn from(handle: *mut c_void) -> @wxDocParentFrame { @wxDocParentFrame(handle) }
    pub fn null() -> @wxDocParentFrame { wxDocParentFrame::from(0 as *mut c_void) }
    
}

pub trait _wxDocParentFrame : _wxFrame {
}

pub struct wxDocTemplate(*mut c_void);
impl _wxDocTemplate for wxDocTemplate {}
impl _wxObject for wxDocTemplate { fn handle(&self) -> *mut c_void { **self } }

impl wxDocTemplate {
    pub fn from(handle: *mut c_void) -> @wxDocTemplate { @wxDocTemplate(handle) }
    pub fn null() -> @wxDocTemplate { wxDocTemplate::from(0 as *mut c_void) }
    
}

pub trait _wxDocTemplate : _wxObject {
}

pub struct wxDocument(*mut c_void);
impl _wxDocument for wxDocument {}
impl _wxEvtHandler for wxDocument {}
impl _wxObject for wxDocument { fn handle(&self) -> *mut c_void { **self } }

impl wxDocument {
    pub fn from(handle: *mut c_void) -> @wxDocument { @wxDocument(handle) }
    pub fn null() -> @wxDocument { wxDocument::from(0 as *mut c_void) }
    
}

pub trait _wxDocument : _wxEvtHandler {
}

pub struct wxDragImage(*mut c_void);
impl _wxDragImage for wxDragImage {}
impl _wxObject for wxDragImage { fn handle(&self) -> *mut c_void { **self } }

impl wxDragImage {
    pub fn from(handle: *mut c_void) -> @wxDragImage { @wxDragImage(handle) }
    pub fn null() -> @wxDragImage { wxDragImage::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxBitmap>(image: &T, x: c_int, y: c_int) -> @wxDragImage {
        unsafe { @wxDragImage(wxDragImage_Create(image.handle(), x, y)) }
    }
}

pub trait _wxDragImage : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginDragFullScreen<T: _wxWindow, U: _wxRect>(&self, x_pos: c_int, y_pos: c_int, window: &T, fullScreen: c_int, rect: &U) -> c_int {
        unsafe { wxDragImage_BeginDragFullScreen(self.handle(), x_pos, y_pos, window.handle(), fullScreen, rect.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginDrag<T: _wxWindow, U: _wxWindow>(&self, x: c_int, y: c_int, window: &T, boundingWindow: &U) -> c_int {
        unsafe { wxDragImage_BeginDrag(self.handle(), x, y, window.handle(), boundingWindow.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDrag(&self) {
        unsafe { wxDragImage_EndDrag(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) -> c_int {
        unsafe { wxDragImage_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxDragImage_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) -> c_int {
        unsafe { wxDragImage_Show(self.handle()) }
    }
}

pub struct wxDrawControl(*mut c_void);
impl _wxDrawControl for wxDrawControl {}
impl _wxControl for wxDrawControl {}
impl _wxWindow for wxDrawControl {}
impl _wxEvtHandler for wxDrawControl {}
impl _wxObject for wxDrawControl { fn handle(&self) -> *mut c_void { **self } }

impl wxDrawControl {
    pub fn from(handle: *mut c_void) -> @wxDrawControl { @wxDrawControl(handle) }
    pub fn null() -> @wxDrawControl { wxDrawControl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDrawControl {
        unsafe { @wxDrawControl(wxDrawControl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxDrawControl : _wxControl {
}

pub struct wxDrawWindow(*mut c_void);
impl _wxDrawWindow for wxDrawWindow {}
impl _wxWindow for wxDrawWindow {}
impl _wxEvtHandler for wxDrawWindow {}
impl _wxObject for wxDrawWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxDrawWindow {
    pub fn from(handle: *mut c_void) -> @wxDrawWindow { @wxDrawWindow(handle) }
    pub fn null() -> @wxDrawWindow { wxDrawWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDrawWindow {
        unsafe { @wxDrawWindow(wxDrawWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxDrawWindow : _wxWindow {
}

pub struct wxDropFilesEvent(*mut c_void);
impl _wxDropFilesEvent for wxDropFilesEvent {}
impl _wxEvent for wxDropFilesEvent {}
impl _wxObject for wxDropFilesEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxDropFilesEvent {
    pub fn from(handle: *mut c_void) -> @wxDropFilesEvent { @wxDropFilesEvent(handle) }
    pub fn null() -> @wxDropFilesEvent { wxDropFilesEvent::from(0 as *mut c_void) }
    
}

pub trait _wxDropFilesEvent : _wxEvent {
}

pub struct wxDropSource(*mut c_void);
impl _wxDropSource for wxDropSource { fn handle(&self) -> *mut c_void { **self } }

impl wxDropSource {
    pub fn from(handle: *mut c_void) -> @wxDropSource { @wxDropSource(handle) }
    pub fn null() -> @wxDropSource { wxDropSource::from(0 as *mut c_void) }
    
}

pub trait _wxDropSource {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxDropTarget(*mut c_void);
impl _wxDropTarget for wxDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl wxDropTarget {
    pub fn from(handle: *mut c_void) -> @wxDropTarget { @wxDropTarget(handle) }
    pub fn null() -> @wxDropTarget { wxDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxDropTarget {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataObject<T: _wxDataObject>(&self, _dat: &T) {
        unsafe { wxDropTarget_SetDataObject(self.handle(), _dat.handle()) }
    }
}

pub struct wxEraseEvent(*mut c_void);
impl _wxEraseEvent for wxEraseEvent {}
impl _wxEvent for wxEraseEvent {}
impl _wxObject for wxEraseEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxEraseEvent {
    pub fn from(handle: *mut c_void) -> @wxEraseEvent { @wxEraseEvent(handle) }
    pub fn null() -> @wxEraseEvent { wxEraseEvent::from(0 as *mut c_void) }
    
}

pub trait _wxEraseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDC(&self) -> @wxDC {
        unsafe { @wxDC(wxEraseEvent_GetDC(self.handle())) }
    }
}

pub struct wxEvent(*mut c_void);
impl _wxEvent for wxEvent {}
impl _wxObject for wxEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxEvent {
    pub fn from(handle: *mut c_void) -> @wxEvent { @wxEvent(handle) }
    pub fn null() -> @wxEvent { wxEvent::from(0 as *mut c_void) }
    
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
        unsafe { wxEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEventObject(&self) -> @wxObject {
        unsafe { @wxObject(wxEvent_GetEventObject(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEventType(&self) -> c_int {
        unsafe { wxEvent_GetEventType(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxEvent_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSkipped(&self) -> c_int {
        unsafe { wxEvent_GetSkipped(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTimestamp(&self) -> c_int {
        unsafe { wxEvent_GetTimestamp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCommandEvent(&self) -> c_int {
        unsafe { wxEvent_IsCommandEvent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEventObject<T: _wxObject>(&self, obj: &T) {
        unsafe { wxEvent_SetEventObject(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEventType(&self, typ: c_int) {
        unsafe { wxEvent_SetEventType(self.handle(), typ) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, Id: c_int) {
        unsafe { wxEvent_SetId(self.handle(), Id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTimestamp(&self, ts: c_int) {
        unsafe { wxEvent_SetTimestamp(self.handle(), ts) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn skip(&self) {
        unsafe { wxEvent_Skip(self.handle()) }
    }
}

pub struct wxEvtHandler(*mut c_void);
impl _wxEvtHandler for wxEvtHandler {}
impl _wxObject for wxEvtHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxEvtHandler {
    pub fn from(handle: *mut c_void) -> @wxEvtHandler { @wxEvtHandler(handle) }
    pub fn null() -> @wxEvtHandler { wxEvtHandler::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_Create()) }
    }
}

pub trait _wxEvtHandler : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPendingEvent<T: _wxEvent>(&self, event: &T) {
        unsafe { wxEvtHandler_AddPendingEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn connect(&self, first: c_int, last: c_int, type_: c_int, data: *mut c_void) -> c_int {
        unsafe { wxEvtHandler_Connect(self.handle(), first, last, type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disconnect(&self, first: c_int, last: c_int, type_: c_int, id: c_int) -> c_int {
        unsafe { wxEvtHandler_Disconnect(self.handle(), first, last, type_, id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEvtHandlerEnabled(&self) -> c_int {
        unsafe { wxEvtHandler_GetEvtHandlerEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextHandler(&self) -> @wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_GetNextHandler(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPreviousHandler(&self) -> @wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_GetPreviousHandler(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn processEvent<T: _wxEvent>(&self, event: &T) -> c_int {
        unsafe { wxEvtHandler_ProcessEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEvtHandlerEnabled(&self, enabled: c_int) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self.handle(), enabled) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNextHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetNextHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPreviousHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClosure(&self, id: c_int, type_: c_int) -> @wxClosure {
        unsafe { @wxClosure(wxEvtHandler_GetClosure(self.handle(), id, type_)) }
    }
}

pub struct wxFileDataObject(*mut c_void);
impl _wxFileDataObject for wxFileDataObject {}
impl _wxDataObjectSimple for wxFileDataObject {}
impl _wxDataObject for wxFileDataObject { fn handle(&self) -> *mut c_void { **self } }

impl wxFileDataObject {
    pub fn from(handle: *mut c_void) -> @wxFileDataObject { @wxFileDataObject(handle) }
    pub fn null() -> @wxFileDataObject { wxFileDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxFileDataObject : _wxDataObjectSimple {
}

pub struct wxFileDialog(*mut c_void);
impl _wxFileDialog for wxFileDialog {}
impl _wxDialog for wxFileDialog {}
impl _wxTopLevelWindow for wxFileDialog {}
impl _wxWindow for wxFileDialog {}
impl _wxEvtHandler for wxFileDialog {}
impl _wxObject for wxFileDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxFileDialog {
    pub fn from(handle: *mut c_void) -> @wxFileDialog { @wxFileDialog(handle) }
    pub fn null() -> @wxFileDialog { wxFileDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _dir: &str, _fle: &str, _wcd: &str, _lft: c_int, _top: c_int, _stl: c_int) -> @wxFileDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        let _fle = wxT(_fle);
        let _wcd = wxT(_wcd);
        unsafe { @wxFileDialog(wxFileDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _fle.handle(), _wcd.handle(), _lft, _top, _stl)) }
    }
}

pub trait _wxFileDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirectory(&self) -> ~str {
        unsafe { wxString { handle: wxFileDialog_GetDirectory(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilename(&self) -> ~str {
        unsafe { wxString { handle: wxFileDialog_GetFilename(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilenames(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetFilenames(self.handle(), paths) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilterIndex(&self) -> c_int {
        unsafe { wxFileDialog_GetFilterIndex(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMessage(&self) -> ~str {
        unsafe { wxString { handle: wxFileDialog_GetMessage(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPath(&self) -> ~str {
        unsafe { wxString { handle: wxFileDialog_GetPath(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaths(&self, paths: *mut c_void) -> c_int {
        unsafe { wxFileDialog_GetPaths(self.handle(), paths) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxFileDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWildcard(&self) -> ~str {
        unsafe { wxString { handle: wxFileDialog_GetWildcard(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDirectory(&self, dir: &str) {
        let dir = wxT(dir);
        unsafe { wxFileDialog_SetDirectory(self.handle(), dir.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFilename(&self, name: &str) {
        let name = wxT(name);
        unsafe { wxFileDialog_SetFilename(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.handle(), filterIndex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMessage(&self, message: &str) {
        let message = wxT(message);
        unsafe { wxFileDialog_SetMessage(self.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPath(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxFileDialog_SetPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWildcard(&self, wildCard: &str) {
        let wildCard = wxT(wildCard);
        unsafe { wxFileDialog_SetWildcard(self.handle(), wildCard.handle()) }
    }
}

pub struct wxFileDropTarget(*mut c_void);
impl _wxFileDropTarget for wxFileDropTarget {}
impl _wxDropTarget for wxFileDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl wxFileDropTarget {
    pub fn from(handle: *mut c_void) -> @wxFileDropTarget { @wxFileDropTarget(handle) }
    pub fn null() -> @wxFileDropTarget { wxFileDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxFileDropTarget : _wxDropTarget {
}

pub struct wxFileHistory(*mut c_void);
impl _wxFileHistory for wxFileHistory {}
impl _wxObject for wxFileHistory { fn handle(&self) -> *mut c_void { **self } }

impl wxFileHistory {
    pub fn from(handle: *mut c_void) -> @wxFileHistory { @wxFileHistory(handle) }
    pub fn null() -> @wxFileHistory { wxFileHistory::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(maxFiles: c_int) -> @wxFileHistory {
        unsafe { @wxFileHistory(wxFileHistory_Create(maxFiles)) }
    }
}

pub trait _wxFileHistory : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFileToHistory(&self, file: &str) {
        let file = wxT(file);
        unsafe { wxFileHistory_AddFileToHistory(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFilesToMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_AddFilesToMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxFileHistory_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHistoryFile(&self, i: c_int) -> ~str {
        unsafe { wxString { handle: wxFileHistory_GetHistoryFile(self.handle(), i) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxFiles(&self) -> c_int {
        unsafe { wxFileHistory_GetMaxFiles(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenus(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFileHistory_GetMenus(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load<T: _wxConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Load(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.handle(), i) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_RemoveMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn save<T: _wxConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Save(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn useMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_UseMenu(self.handle(), menu.handle()) }
    }
}

pub struct wxFileType(*mut c_void);
impl _wxFileType for wxFileType { fn handle(&self) -> *mut c_void { **self } }

impl wxFileType {
    pub fn from(handle: *mut c_void) -> @wxFileType { @wxFileType(handle) }
    pub fn null() -> @wxFileType { wxFileType::from(0 as *mut c_void) }
    
}

pub trait _wxFileType {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expandCommand(&self, _cmd: *mut c_void, _params: *mut c_void) -> ~str {
        unsafe { wxString { handle: wxFileType_ExpandCommand(self.handle(), _cmd, _params) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDescription(&self) -> ~str {
        unsafe { wxString { handle: wxFileType_GetDescription(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExtensions<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetExtensions(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon<T: _wxIcon>(&self, icon: &T) -> c_int {
        unsafe { wxFileType_GetIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMimeType(&self) -> ~str {
        unsafe { wxString { handle: wxFileType_GetMimeType(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMimeTypes<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetMimeTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOpenCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.handle(), _buf, _params) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.handle(), _buf, _params) }
    }
}

pub struct wxFindDialogEvent(*mut c_void);
impl _wxFindDialogEvent for wxFindDialogEvent {}
impl _wxCommandEvent for wxFindDialogEvent {}
impl _wxEvent for wxFindDialogEvent {}
impl _wxObject for wxFindDialogEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxFindDialogEvent {
    pub fn from(handle: *mut c_void) -> @wxFindDialogEvent { @wxFindDialogEvent(handle) }
    pub fn null() -> @wxFindDialogEvent { wxFindDialogEvent::from(0 as *mut c_void) }
    
}

pub trait _wxFindDialogEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFindString(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFindDialogEvent_GetFindString(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindDialogEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReplaceString(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxFindDialogEvent_GetReplaceString(self.handle(), _ref) }
    }
}

pub struct wxFindReplaceData(*mut c_void);
impl _wxFindReplaceData for wxFindReplaceData {}
impl _wxObject for wxFindReplaceData { fn handle(&self) -> *mut c_void { **self } }

impl wxFindReplaceData {
    pub fn from(handle: *mut c_void) -> @wxFindReplaceData { @wxFindReplaceData(handle) }
    pub fn null() -> @wxFindReplaceData { wxFindReplaceData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int) -> @wxFindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceData_Create(flags)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxFindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceData_CreateDefault()) }
    }
}

pub trait _wxFindReplaceData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFindString(&self) -> ~str {
        unsafe { wxString { handle: wxFindReplaceData_GetFindString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReplaceString(&self) -> ~str {
        unsafe { wxString { handle: wxFindReplaceData_GetReplaceString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFindString(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxFindReplaceData_SetFindString(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReplaceString(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxFindReplaceData_SetReplaceString(self.handle(), str.handle()) }
    }
}

pub struct wxFindReplaceDialog(*mut c_void);
impl _wxFindReplaceDialog for wxFindReplaceDialog {}
impl _wxDialog for wxFindReplaceDialog {}
impl _wxTopLevelWindow for wxFindReplaceDialog {}
impl _wxWindow for wxFindReplaceDialog {}
impl _wxEvtHandler for wxFindReplaceDialog {}
impl _wxObject for wxFindReplaceDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxFindReplaceDialog {
    pub fn from(handle: *mut c_void) -> @wxFindReplaceDialog { @wxFindReplaceDialog(handle) }
    pub fn null() -> @wxFindReplaceDialog { wxFindReplaceDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxFindReplaceData>(parent: &T, data: &U, title: &str, style: c_int) -> @wxFindReplaceDialog {
        let title = wxT(title);
        unsafe { @wxFindReplaceDialog(wxFindReplaceDialog_Create(parent.handle(), data.handle(), title.handle(), style)) }
    }
}

pub trait _wxFindReplaceDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> @wxFindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceDialog_GetData(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData<T: _wxFindReplaceData>(&self, data: &T) {
        unsafe { wxFindReplaceDialog_SetData(self.handle(), data.handle()) }
    }
}

pub struct wxFlexGridSizer(*mut c_void);
impl _wxFlexGridSizer for wxFlexGridSizer {}
impl _wxGridSizer for wxFlexGridSizer {}
impl _wxSizer for wxFlexGridSizer {}
impl _wxObject for wxFlexGridSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxFlexGridSizer {
    pub fn from(handle: *mut c_void) -> @wxFlexGridSizer { @wxFlexGridSizer(handle) }
    pub fn null() -> @wxFlexGridSizer { wxFlexGridSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @wxFlexGridSizer {
        unsafe { @wxFlexGridSizer(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

pub trait _wxFlexGridSizer : _wxGridSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableCol(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableRow(self.handle(), idx) }
    }
}

pub struct wxFocusEvent(*mut c_void);
impl _wxFocusEvent for wxFocusEvent {}
impl _wxEvent for wxFocusEvent {}
impl _wxObject for wxFocusEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxFocusEvent {
    pub fn from(handle: *mut c_void) -> @wxFocusEvent { @wxFocusEvent(handle) }
    pub fn null() -> @wxFocusEvent { wxFocusEvent::from(0 as *mut c_void) }
    
}

pub trait _wxFocusEvent : _wxEvent {
}

pub struct wxFont(*mut c_void);
impl _wxFont for wxFont {}
impl _wxGDIObject for wxFont {}
impl _wxObject for wxFont { fn handle(&self) -> *mut c_void { **self } }

impl wxFont {
    pub fn from(handle: *mut c_void) -> @wxFont { @wxFont(handle) }
    pub fn null() -> @wxFont { wxFont::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: c_int, face: &str, enc: c_int) -> @wxFont {
        let face = wxT(face);
        unsafe { @wxFont(wxFont_Create(pointSize, family, style, weight, underlined, face.handle(), enc)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> @wxFont {
        unsafe { @wxFont(wxFont_CreateFromStock(id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxFont {
        unsafe { @wxFont(wxFont_CreateDefault()) }
    }
}

pub trait _wxFont : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultEncoding(&self) -> c_int {
        unsafe { wxFont_GetDefaultEncoding(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEncoding(&self) -> c_int {
        unsafe { wxFont_GetEncoding(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFaceName(&self) -> ~str {
        unsafe { wxString { handle: wxFont_GetFaceName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFamilyString(&self) -> ~str {
        unsafe { wxString { handle: wxFont_GetFamilyString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPointSize(&self) -> c_int {
        unsafe { wxFont_GetPointSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxFont_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyleString(&self) -> ~str {
        unsafe { wxString { handle: wxFont_GetStyleString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUnderlined(&self) -> c_int {
        unsafe { wxFont_GetUnderlined(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeight(&self) -> c_int {
        unsafe { wxFont_GetWeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeightString(&self) -> ~str {
        unsafe { wxString { handle: wxFont_GetWeightString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxFont_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetDefaultEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFaceName(&self, faceName: &str) {
        let faceName = wxT(faceName);
        unsafe { wxFont_SetFaceName(self.handle(), faceName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFamily(&self, family: c_int) {
        unsafe { wxFont_SetFamily(self.handle(), family) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPointSize(&self, pointSize: c_int) {
        unsafe { wxFont_SetPointSize(self.handle(), pointSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFont_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setUnderlined(&self, underlined: c_int) {
        unsafe { wxFont_SetUnderlined(self.handle(), underlined) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWeight(&self, weight: c_int) {
        unsafe { wxFont_SetWeight(self.handle(), weight) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxFont_IsStatic(self.handle()) }
    }
}

pub struct wxFontData(*mut c_void);
impl _wxFontData for wxFontData {}
impl _wxObject for wxFontData { fn handle(&self) -> *mut c_void { **self } }

impl wxFontData {
    pub fn from(handle: *mut c_void) -> @wxFontData { @wxFontData(handle) }
    pub fn null() -> @wxFontData { wxFontData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxFontData {
        unsafe { @wxFontData(wxFontData_Create()) }
    }
}

pub trait _wxFontData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableEffects(&self, flag: c_int) {
        unsafe { wxFontData_EnableEffects(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllowSymbols(&self) -> c_int {
        unsafe { wxFontData_GetAllowSymbols(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChosenFont<T: _wxFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetChosenFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxFontData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableEffects(&self) -> c_int {
        unsafe { wxFontData_GetEnableEffects(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEncoding(&self) -> c_int {
        unsafe { wxFontData_GetEncoding(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInitialFont<T: _wxFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetInitialFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getShowHelp(&self) -> c_int {
        unsafe { wxFontData_GetShowHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAllowSymbols(&self, flag: c_int) {
        unsafe { wxFontData_SetAllowSymbols(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChosenFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxFontData_SetChosenFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxFontData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInitialFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxFontData_SetInitialFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, minRange: c_int, maxRange: c_int) {
        unsafe { wxFontData_SetRange(self.handle(), minRange, maxRange) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShowHelp(&self, flag: c_int) {
        unsafe { wxFontData_SetShowHelp(self.handle(), flag) }
    }
}

pub struct wxFontDialog(*mut c_void);
impl _wxFontDialog for wxFontDialog {}
impl _wxDialog for wxFontDialog {}
impl _wxTopLevelWindow for wxFontDialog {}
impl _wxWindow for wxFontDialog {}
impl _wxEvtHandler for wxFontDialog {}
impl _wxObject for wxFontDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxFontDialog {
    pub fn from(handle: *mut c_void) -> @wxFontDialog { @wxFontDialog(handle) }
    pub fn null() -> @wxFontDialog { wxFontDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxFontData>(_prt: &T, fnt: &U) -> @wxFontDialog {
        unsafe { @wxFontDialog(wxFontDialog_Create(_prt.handle(), fnt.handle())) }
    }
}

pub trait _wxFontDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFontData<T: _wxFontData>(&self, _ref: &T) {
        unsafe { wxFontDialog_GetFontData(self.handle(), _ref.handle()) }
    }
}

pub struct wxFontEnumerator(*mut c_void);
impl _wxFontEnumerator for wxFontEnumerator { fn handle(&self) -> *mut c_void { **self } }

impl wxFontEnumerator {
    pub fn from(handle: *mut c_void) -> @wxFontEnumerator { @wxFontEnumerator(handle) }
    pub fn null() -> @wxFontEnumerator { wxFontEnumerator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> @wxFontEnumerator {
        unsafe { @wxFontEnumerator(wxFontEnumerator_Create(_obj, _fnc)) }
    }
}

pub trait _wxFontEnumerator {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumerateEncodings(&self, facename: &str) -> c_int {
        let facename = wxT(facename);
        unsafe { wxFontEnumerator_EnumerateEncodings(self.handle(), facename.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> c_int {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.handle(), encoding, fixedWidthOnly) }
    }
}

pub struct wxFontList(*mut c_void);
impl _wxFontList for wxFontList {}
impl _wxList for wxFontList {}
impl _wxObject for wxFontList { fn handle(&self) -> *mut c_void { **self } }

impl wxFontList {
    pub fn from(handle: *mut c_void) -> @wxFontList { @wxFontList(handle) }
    pub fn null() -> @wxFontList { wxFontList::from(0 as *mut c_void) }
    
}

pub trait _wxFontList : _wxList {
}

pub struct wxFontMapper(*mut c_void);
impl _wxFontMapper for wxFontMapper { fn handle(&self) -> *mut c_void { **self } }

impl wxFontMapper {
    pub fn from(handle: *mut c_void) -> @wxFontMapper { @wxFontMapper(handle) }
    pub fn null() -> @wxFontMapper { wxFontMapper::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxFontMapper {
        unsafe { @wxFontMapper(wxFontMapper_Create()) }
    }
}

pub trait _wxFontMapper {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *mut c_void, _buf: &str) -> c_int {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_GetAltForEncoding(self.handle(), encoding, alt_encoding, _buf.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEncodingAvailable(&self, encoding: c_int, _buf: &str) -> c_int {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_IsEncodingAvailable(self.handle(), encoding, _buf.handle()) }
    }
}

pub struct wxFrame(*mut c_void);
impl _wxFrame for wxFrame {}
impl _wxTopLevelWindow for wxFrame {}
impl _wxWindow for wxFrame {}
impl _wxEvtHandler for wxFrame {}
impl _wxObject for wxFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxFrame {
    pub fn from(handle: *mut c_void) -> @wxFrame { @wxFrame(handle) }
    pub fn null() -> @wxFrame { wxFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxFrame {
        let _txt = wxT(_txt);
        unsafe { @wxFrame(wxFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxFrame : _wxTopLevelWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newStatusBar(&self, number: c_int, style: c_int) -> @wxStatusBar {
        unsafe { @wxStatusBar(wxFrame_CreateStatusBar(self.handle(), number, style)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn newToolBar(&self, style: c_long) -> @wxToolBar {
        unsafe { @wxToolBar(wxFrame_CreateToolBar(self.handle(), style)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientAreaOrigin_left(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientAreaOrigin_top(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuBar(&self) -> @wxMenuBar {
        unsafe { @wxMenuBar(wxFrame_GetMenuBar(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStatusBar(&self) -> @wxStatusBar {
        unsafe { @wxStatusBar(wxFrame_GetStatusBar(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolBar(&self) -> @wxToolBar {
        unsafe { @wxToolBar(wxFrame_GetToolBar(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMenuBar<T: _wxMenuBar>(&self, menubar: &T) {
        unsafe { wxFrame_SetMenuBar(self.handle(), menubar.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusBar<T: _wxStatusBar>(&self, statBar: &T) {
        unsafe { wxFrame_SetStatusBar(self.handle(), statBar.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusText(&self, _txt: &str, _number: c_int) {
        let _txt = wxT(_txt);
        unsafe { wxFrame_SetStatusText(self.handle(), _txt.handle(), _number) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusWidths(&self, _n: c_int, _widths_field: *mut c_void) {
        unsafe { wxFrame_SetStatusWidths(self.handle(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolBar<T: _wxToolBar>(&self, _toolbar: &T) {
        unsafe { wxFrame_SetToolBar(self.handle(), _toolbar.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShape<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxFrame_SetShape(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showFullScreen(&self, show: c_int, style: c_int) -> c_int {
        unsafe { wxFrame_ShowFullScreen(self.handle(), show, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isFullScreen(&self) -> c_int {
        unsafe { wxFrame_IsFullScreen(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centre(&self, orientation: c_int) {
        unsafe { wxFrame_Centre(self.handle(), orientation) }
    }
}

pub struct wxGDIObject(*mut c_void);
impl _wxGDIObject for wxGDIObject {}
impl _wxObject for wxGDIObject { fn handle(&self) -> *mut c_void { **self } }

impl wxGDIObject {
    pub fn from(handle: *mut c_void) -> @wxGDIObject { @wxGDIObject(handle) }
    pub fn null() -> @wxGDIObject { wxGDIObject::from(0 as *mut c_void) }
    
}

pub trait _wxGDIObject : _wxObject {
}

pub struct wxGauge(*mut c_void);
impl _wxGauge for wxGauge {}
impl _wxControl for wxGauge {}
impl _wxWindow for wxGauge {}
impl _wxEvtHandler for wxGauge {}
impl _wxObject for wxGauge { fn handle(&self) -> *mut c_void { **self } }

impl wxGauge {
    pub fn from(handle: *mut c_void) -> @wxGauge { @wxGauge(handle) }
    pub fn null() -> @wxGauge { wxGauge::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxGauge {
        unsafe { @wxGauge(wxGauge_Create(_prt.handle(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxGauge : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBezelFace(&self) -> c_int {
        unsafe { wxGauge_GetBezelFace(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRange(&self) -> c_int {
        unsafe { wxGauge_GetRange(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getShadowWidth(&self) -> c_int {
        unsafe { wxGauge_GetShadowWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxGauge_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBezelFace(&self, w: c_int) {
        unsafe { wxGauge_SetBezelFace(self.handle(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, r: c_int) {
        unsafe { wxGauge_SetRange(self.handle(), r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShadowWidth(&self, w: c_int) {
        unsafe { wxGauge_SetShadowWidth(self.handle(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, pos: c_int) {
        unsafe { wxGauge_SetValue(self.handle(), pos) }
    }
}

pub struct wxGenericDirCtrl(*mut c_void);
impl _wxGenericDirCtrl for wxGenericDirCtrl {}
impl _wxControl for wxGenericDirCtrl {}
impl _wxWindow for wxGenericDirCtrl {}
impl _wxEvtHandler for wxGenericDirCtrl {}
impl _wxObject for wxGenericDirCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxGenericDirCtrl {
    pub fn from(handle: *mut c_void) -> @wxGenericDirCtrl { @wxGenericDirCtrl(handle) }
    pub fn null() -> @wxGenericDirCtrl { wxGenericDirCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxGenericDirCtrl : _wxControl {
}

pub struct wxGenericValidator(*mut c_void);
impl _wxGenericValidator for wxGenericValidator {}
impl _wxValidator for wxGenericValidator {}
impl _wxEvtHandler for wxGenericValidator {}
impl _wxObject for wxGenericValidator { fn handle(&self) -> *mut c_void { **self } }

impl wxGenericValidator {
    pub fn from(handle: *mut c_void) -> @wxGenericValidator { @wxGenericValidator(handle) }
    pub fn null() -> @wxGenericValidator { wxGenericValidator::from(0 as *mut c_void) }
    
}

pub trait _wxGenericValidator : _wxValidator {
}

pub struct wxGridSizer(*mut c_void);
impl _wxGridSizer for wxGridSizer {}
impl _wxSizer for wxGridSizer {}
impl _wxObject for wxGridSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxGridSizer {
    pub fn from(handle: *mut c_void) -> @wxGridSizer { @wxGridSizer(handle) }
    pub fn null() -> @wxGridSizer { wxGridSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @wxGridSizer {
        unsafe { @wxGridSizer(wxGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

pub trait _wxGridSizer : _wxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCols(&self) -> c_int {
        unsafe { wxGridSizer_GetCols(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHGap(&self) -> c_int {
        unsafe { wxGridSizer_GetHGap(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRows(&self) -> c_int {
        unsafe { wxGridSizer_GetRows(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVGap(&self) -> c_int {
        unsafe { wxGridSizer_GetVGap(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCols(&self, cols: c_int) {
        unsafe { wxGridSizer_SetCols(self.handle(), cols) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetHGap(self.handle(), gap) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRows(&self, rows: c_int) {
        unsafe { wxGridSizer_SetRows(self.handle(), rows) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetVGap(self.handle(), gap) }
    }
}

pub struct wxHelpController(*mut c_void);
impl _wxHelpController for wxHelpController {}
impl _wxHelpControllerBase for wxHelpController {}
impl _wxObject for wxHelpController { fn handle(&self) -> *mut c_void { **self } }

impl wxHelpController {
    pub fn from(handle: *mut c_void) -> @wxHelpController { @wxHelpController(handle) }
    pub fn null() -> @wxHelpController { wxHelpController::from(0 as *mut c_void) }
    
}

pub trait _wxHelpController : _wxHelpControllerBase {
}

pub struct wxHelpControllerBase(*mut c_void);
impl _wxHelpControllerBase for wxHelpControllerBase {}
impl _wxObject for wxHelpControllerBase { fn handle(&self) -> *mut c_void { **self } }

impl wxHelpControllerBase {
    pub fn from(handle: *mut c_void) -> @wxHelpControllerBase { @wxHelpControllerBase(handle) }
    pub fn null() -> @wxHelpControllerBase { wxHelpControllerBase::from(0 as *mut c_void) }
    
}

pub trait _wxHelpControllerBase : _wxObject {
}

pub struct wxHelpControllerHelpProvider(*mut c_void);
impl _wxHelpControllerHelpProvider for wxHelpControllerHelpProvider {}
impl _wxSimpleHelpProvider for wxHelpControllerHelpProvider {}
impl _wxHelpProvider for wxHelpControllerHelpProvider { fn handle(&self) -> *mut c_void { **self } }

impl wxHelpControllerHelpProvider {
    pub fn from(handle: *mut c_void) -> @wxHelpControllerHelpProvider { @wxHelpControllerHelpProvider(handle) }
    pub fn null() -> @wxHelpControllerHelpProvider { wxHelpControllerHelpProvider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxHelpControllerBase>(ctr: &T) -> @wxHelpControllerHelpProvider {
        unsafe { @wxHelpControllerHelpProvider(wxHelpControllerHelpProvider_Create(ctr.handle())) }
    }
}

pub trait _wxHelpControllerHelpProvider : _wxSimpleHelpProvider {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelpController(&self) -> @wxHelpControllerBase {
        unsafe { @wxHelpControllerBase(wxHelpControllerHelpProvider_GetHelpController(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpController<T: _wxHelpController>(&self, hc: &T) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.handle(), hc.handle()) }
    }
}

pub struct wxHelpEvent(*mut c_void);
impl _wxHelpEvent for wxHelpEvent {}
impl _wxCommandEvent for wxHelpEvent {}
impl _wxEvent for wxHelpEvent {}
impl _wxObject for wxHelpEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxHelpEvent {
    pub fn from(handle: *mut c_void) -> @wxHelpEvent { @wxHelpEvent(handle) }
    pub fn null() -> @wxHelpEvent { wxHelpEvent::from(0 as *mut c_void) }
    
}

pub trait _wxHelpEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLink(&self) -> ~str {
        unsafe { wxString { handle: wxHelpEvent_GetLink(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxHelpEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTarget(&self) -> ~str {
        unsafe { wxString { handle: wxHelpEvent_GetTarget(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLink(&self, link: &str) {
        let link = wxT(link);
        unsafe { wxHelpEvent_SetLink(self.handle(), link.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTarget(&self, target: &str) {
        let target = wxT(target);
        unsafe { wxHelpEvent_SetTarget(self.handle(), target.handle()) }
    }
}

pub struct wxHelpProvider(*mut c_void);
impl _wxHelpProvider for wxHelpProvider { fn handle(&self) -> *mut c_void { **self } }

impl wxHelpProvider {
    pub fn from(handle: *mut c_void) -> @wxHelpProvider { @wxHelpProvider(handle) }
    pub fn null() -> @wxHelpProvider { wxHelpProvider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> @wxHelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Get()) }
    }
}

pub trait _wxHelpProvider {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHelp<T: _wxWindow>(&self, window: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxHelpProvider_AddHelp(self.handle(), window.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHelpById(&self, id: c_int, text: &str) {
        let text = wxT(text);
        unsafe { wxHelpProvider_AddHelpById(self.handle(), id, text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelp<T: _wxWindow>(&self, window: &T) -> ~str {
        unsafe { wxString { handle: wxHelpProvider_GetHelp(self.handle(), window.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeHelp<T: _wxWindow>(&self, window: &T) {
        unsafe { wxHelpProvider_RemoveHelp(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self) -> @wxHelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Set(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showHelp<T: _wxWindow>(&self, window: &T) -> c_int {
        unsafe { wxHelpProvider_ShowHelp(self.handle(), window.handle()) }
    }
}

pub struct wxIcon(*mut c_void);
impl _wxIcon for wxIcon {}
impl _wxBitmap for wxIcon {}
impl _wxGDIObject for wxIcon {}
impl _wxObject for wxIcon { fn handle(&self) -> *mut c_void { **self } }

impl wxIcon {
    pub fn from(handle: *mut c_void) -> @wxIcon { @wxIcon(handle) }
    pub fn null() -> @wxIcon { wxIcon::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxIcon {
        unsafe { @wxIcon(wxIcon_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newLoad(name: &str, type_: c_long, width: c_int, height: c_int) -> @wxIcon {
        let name = wxT(name);
        unsafe { @wxIcon(wxIcon_CreateLoad(name.handle(), type_, width, height)) }
    }
}

pub trait _wxIcon : _wxBitmap {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign(&self, other: *mut c_void) {
        unsafe { wxIcon_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copyFromBitmap<T: _wxBitmap>(&self, bmp: &T) {
        unsafe { wxIcon_CopyFromBitmap(self.handle(), bmp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fromRaw(&self, width: c_int, height: c_int) -> @wxIcon {
        unsafe { @wxIcon(wxIcon_FromRaw(self.handle(), width, height)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fromXPM(&self) -> @wxIcon {
        unsafe { @wxIcon(wxIcon_FromXPM(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual(&self, other: &_wxIcon) -> c_int {
        unsafe { wxIcon_IsEqual(self.handle(), other.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, name: &str, type_: c_long, width: c_int, height: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxIcon_Load(self.handle(), name.handle(), type_, width, height) }
    }
}

pub struct wxIconBundle(*mut c_void);
impl _wxIconBundle for wxIconBundle { fn handle(&self) -> *mut c_void { **self } }

impl wxIconBundle {
    pub fn from(handle: *mut c_void) -> @wxIconBundle { @wxIconBundle(handle) }
    pub fn null() -> @wxIconBundle { wxIconBundle::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxIconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFile(file: &str, type_: c_int) -> @wxIconBundle {
        let file = wxT(file);
        unsafe { @wxIconBundle(wxIconBundle_CreateFromFile(file.handle(), type_)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromIcon<T: _wxIcon>(icon: &T) -> @wxIconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateFromIcon(icon.handle())) }
    }
}

pub trait _wxIconBundle {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addIcon<T: _wxIcon>(&self, icon: &T) {
        unsafe { wxIconBundle_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addIconFromFile(&self, file: &str, type_: c_int) {
        let file = wxT(file);
        unsafe { wxIconBundle_AddIconFromFile(self.handle(), file.handle(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxIconBundle>(&self, _ref: &T) {
        unsafe { wxIconBundle_Assign(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon<T: _wxIcon>(&self, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxIconBundle_GetIcon(self.handle(), w, h, _ref.handle()) }
    }
}

pub struct wxIconizeEvent(*mut c_void);
impl _wxIconizeEvent for wxIconizeEvent {}
impl _wxEvent for wxIconizeEvent {}
impl _wxObject for wxIconizeEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxIconizeEvent {
    pub fn from(handle: *mut c_void) -> @wxIconizeEvent { @wxIconizeEvent(handle) }
    pub fn null() -> @wxIconizeEvent { wxIconizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxIconizeEvent : _wxEvent {
}

pub struct wxIdleEvent(*mut c_void);
impl _wxIdleEvent for wxIdleEvent {}
impl _wxEvent for wxIdleEvent {}
impl _wxObject for wxIdleEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxIdleEvent {
    pub fn from(handle: *mut c_void) -> @wxIdleEvent { @wxIdleEvent(handle) }
    pub fn null() -> @wxIdleEvent { wxIdleEvent::from(0 as *mut c_void) }
    
}

pub trait _wxIdleEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn moreRequested(&self) -> c_int {
        unsafe { wxIdleEvent_MoreRequested(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn requestMore(&self, needMore: c_int) {
        unsafe { wxIdleEvent_RequestMore(self.handle(), needMore) }
    }
}

pub struct wxImage(*mut c_void);
impl _wxImage for wxImage {}
impl _wxObject for wxImage { fn handle(&self) -> *mut c_void { **self } }

impl wxImage {
    pub fn from(handle: *mut c_void) -> @wxImage { @wxImage(handle) }
    pub fn null() -> @wxImage { wxImage::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn canRead(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_CanRead(name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxImage {
        unsafe { @wxImage(wxImage_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(bitmap: &T) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromBitmap(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromByteString(data, length, type_)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromLazyByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromLazyByteString(data, length, type_)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData(width: c_int, height: c_int, data: *mut c_void) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromData(width, height, data)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFile(name: &str) -> @wxImage {
        let name = wxT(name);
        unsafe { @wxImage(wxImage_CreateFromFile(name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newSized(width: c_int, height: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateSized(width, height)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromDataEx(width: c_int, height: c_int, data: *mut c_void, isStaticData: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromDataEx(width, height, data, isStaticData)) }
    }
}

pub trait _wxImage : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToBitmap<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxImage_ConvertToBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToByteString(&self, type_: c_int, data: *mut c_char) -> c_int {
        unsafe { wxImage_ConvertToByteString(self.handle(), type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToLazyByteString(&self, type_: c_int, data: *mut c_char) -> c_int {
        unsafe { wxImage_ConvertToLazyByteString(self.handle(), type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn countColours(&self, stopafter: c_int) -> c_int {
        unsafe { wxImage_CountColours(self.handle(), stopafter) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroy(&self) {
        unsafe { wxImage_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBlue(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetBlue(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> *mut c_void {
        unsafe { wxImage_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGreen(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetGreen(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxImage_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskBlue(&self) -> int8_t {
        unsafe { wxImage_GetMaskBlue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskGreen(&self) -> int8_t {
        unsafe { wxImage_GetMaskGreen(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskRed(&self) -> int8_t {
        unsafe { wxImage_GetMaskRed(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRed(&self, x: c_int, y: c_int) -> int8_t {
        unsafe { wxImage_GetRed(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSubImage<T: _wxImage>(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: &T) {
        unsafe { wxImage_GetSubImage(self.handle(), x, y, w, h, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxImage_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasMask(&self) -> c_int {
        unsafe { wxImage_HasMask(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOption(&self, name: &str) -> ~str {
        let name = wxT(name);
        unsafe { wxString { handle: wxImage_GetOption(self.handle(), name.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOptionInt(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_GetOptionInt(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasOption(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_HasOption(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Initialize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initializeFromData(&self, width: c_int, height: c_int, data: *mut c_void) {
        unsafe { wxImage_InitializeFromData(self.handle(), width, height, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn mirror<T: _wxImage>(&self, horizontally: c_int, image: &T) {
        unsafe { wxImage_Mirror(self.handle(), horizontally, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxImage_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste<T: _wxImage>(&self, image: &T, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self.handle(), image.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace(&self, r1: uint8_t, g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t, b2: uint8_t) {
        unsafe { wxImage_Replace(self.handle(), r1, g1, b1, r2, g2, b2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate<T: _wxImage>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *mut c_void, image: &T) {
        unsafe { wxImage_Rotate(self.handle(), angle, c_x, c_y, interpolating, offset_after_rotation, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate90<T: _wxImage>(&self, clockwise: c_int, image: &T) {
        unsafe { wxImage_Rotate90(self.handle(), clockwise, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_SaveFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scale<T: _wxImage>(&self, width: c_int, height: c_int, image: &T) {
        unsafe { wxImage_Scale(self.handle(), width, height, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData(&self, data: *mut c_void) {
        unsafe { wxImage_SetData(self.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataAndSize(&self, data: *mut c_void, new_width: c_int, new_height: c_int) {
        unsafe { wxImage_SetDataAndSize(self.handle(), data, new_width, new_height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMask(&self, mask: c_int) {
        unsafe { wxImage_SetMask(self.handle(), mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaskColour(&self, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetMaskColour(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOption(&self, name: &str, value: &str) {
        let name = wxT(name);
        let value = wxT(value);
        unsafe { wxImage_SetOption(self.handle(), name.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOptionInt(&self, name: &str, value: c_int) {
        let name = wxT(name);
        unsafe { wxImage_SetOptionInt(self.handle(), name.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRGB(&self, x: c_int, y: c_int, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetRGB(self.handle(), x, y, r, g, b) }
    }
}

pub struct wxImageHandler(*mut c_void);
impl _wxImageHandler for wxImageHandler {}
impl _wxObject for wxImageHandler { fn handle(&self) -> *mut c_void { **self } }

impl wxImageHandler {
    pub fn from(handle: *mut c_void) -> @wxImageHandler { @wxImageHandler(handle) }
    pub fn null() -> @wxImageHandler { wxImageHandler::from(0 as *mut c_void) }
    
}

pub trait _wxImageHandler : _wxObject {
}

pub struct wxImageList(*mut c_void);
impl _wxImageList for wxImageList {}
impl _wxObject for wxImageList { fn handle(&self) -> *mut c_void { **self } }

impl wxImageList {
    pub fn from(handle: *mut c_void) -> @wxImageList { @wxImageList(handle) }
    pub fn null() -> @wxImageList { wxImageList::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> @wxImageList {
        unsafe { @wxImageList(wxImageList_Create(width, height, mask, initialCount)) }
    }
}

pub trait _wxImageList : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addBitmap<T: _wxBitmap, U: _wxBitmap>(&self, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_AddBitmap(self.handle(), bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addIcon<T: _wxIcon>(&self, icon: &T) -> c_int {
        unsafe { wxImageList_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addMasked<T: _wxBitmap, U: _wxColour>(&self, bitmap: &T, maskColour: &U) -> c_int {
        unsafe { wxImageList_AddMasked(self.handle(), bitmap.handle(), maskColour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn draw<T: _wxDC>(&self, index: c_int, dc: &T, x: c_int, y: c_int, flags: c_int, solidBackground: c_int) -> c_int {
        unsafe { wxImageList_Draw(self.handle(), index, dc.handle(), x, y, flags, solidBackground) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageCount(&self) -> c_int {
        unsafe { wxImageList_GetImageCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self, index: c_int, width: *mut c_int, height: *mut c_int) {
        unsafe { wxImageList_GetSize(self.handle(), index, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, index: c_int) -> c_int {
        unsafe { wxImageList_Remove(self.handle(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeAll(&self) -> c_int {
        unsafe { wxImageList_RemoveAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxBitmap, U: _wxBitmap>(&self, index: c_int, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_Replace(self.handle(), index, bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceIcon<T: _wxIcon>(&self, index: c_int, icon: &T) -> c_int {
        unsafe { wxImageList_ReplaceIcon(self.handle(), index, icon.handle()) }
    }
}

pub struct wxIndividualLayoutConstraint(*mut c_void);
impl _wxIndividualLayoutConstraint for wxIndividualLayoutConstraint {}
impl _wxObject for wxIndividualLayoutConstraint { fn handle(&self) -> *mut c_void { **self } }

impl wxIndividualLayoutConstraint {
    pub fn from(handle: *mut c_void) -> @wxIndividualLayoutConstraint { @wxIndividualLayoutConstraint(handle) }
    pub fn null() -> @wxIndividualLayoutConstraint { wxIndividualLayoutConstraint::from(0 as *mut c_void) }
    
}

pub trait _wxIndividualLayoutConstraint : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn above<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Above(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn absolute(&self, val: c_int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn below<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Below(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDone(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetDone(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdge(&self, which: c_int, thisWin: *mut c_void, other: *mut c_void) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetEdge(self.handle(), which, thisWin, other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMargin(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMargin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMyEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMyEdge(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOtherEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetOtherEdge(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOtherWindow(&self) -> *mut c_void {
        unsafe { wxIndividualLayoutConstraint_GetOtherWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPercent(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetPercent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRelationship(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetRelationship(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftOf<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn percentOf<T: _wxWindow>(&self, otherW: &T, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.handle(), otherW.handle(), wh, per) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetIfWin<T: _wxWindow>(&self, otherW: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.handle(), otherW.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightOf<T: _wxWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sameAs<T: _wxWindow>(&self, otherW: &T, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.handle(), otherW.handle(), edge, marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn satisfyConstraint<T: _wxWindow>(&self, constraints: *mut c_void, win: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.handle(), constraints, win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set<T: _wxWindow>(&self, rel: c_int, otherW: &T, otherE: c_int, val: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Set(self.handle(), rel, otherW.handle(), otherE, val, marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDone(&self, d: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetDone(self.handle(), d) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEdge(&self, which: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetEdge(self.handle(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargin(&self, m: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetMargin(self.handle(), m) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRelationship(&self, r: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetRelationship(self.handle(), r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, v: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetValue(self.handle(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unconstrained(&self) {
        unsafe { wxIndividualLayoutConstraint_Unconstrained(self.handle()) }
    }
}

pub struct wxInitDialogEvent(*mut c_void);
impl _wxInitDialogEvent for wxInitDialogEvent {}
impl _wxEvent for wxInitDialogEvent {}
impl _wxObject for wxInitDialogEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxInitDialogEvent {
    pub fn from(handle: *mut c_void) -> @wxInitDialogEvent { @wxInitDialogEvent(handle) }
    pub fn null() -> @wxInitDialogEvent { wxInitDialogEvent::from(0 as *mut c_void) }
    
}

pub trait _wxInitDialogEvent : _wxEvent {
}

pub struct wxJoystickEvent(*mut c_void);
impl _wxJoystickEvent for wxJoystickEvent {}
impl _wxEvent for wxJoystickEvent {}
impl _wxObject for wxJoystickEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxJoystickEvent {
    pub fn from(handle: *mut c_void) -> @wxJoystickEvent { @wxJoystickEvent(handle) }
    pub fn null() -> @wxJoystickEvent { wxJoystickEvent::from(0 as *mut c_void) }
    
}

pub trait _wxJoystickEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDown(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonIsDown(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonIsDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonUp(&self, but: c_int) -> c_int {
        unsafe { wxJoystickEvent_ButtonUp(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButtonChange(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonChange(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButtonState(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonState(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getJoystick(&self) -> c_int {
        unsafe { wxJoystickEvent_GetJoystick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxJoystickEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getZPosition(&self) -> c_int {
        unsafe { wxJoystickEvent_GetZPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isButton(&self) -> c_int {
        unsafe { wxJoystickEvent_IsButton(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMove(&self) -> c_int {
        unsafe { wxJoystickEvent_IsMove(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isZMove(&self) -> c_int {
        unsafe { wxJoystickEvent_IsZMove(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setButtonChange(&self, change: c_int) {
        unsafe { wxJoystickEvent_SetButtonChange(self.handle(), change) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setButtonState(&self, state: c_int) {
        unsafe { wxJoystickEvent_SetButtonState(self.handle(), state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setJoystick(&self, stick: c_int) {
        unsafe { wxJoystickEvent_SetJoystick(self.handle(), stick) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxJoystickEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setZPosition(&self, zPos: c_int) {
        unsafe { wxJoystickEvent_SetZPosition(self.handle(), zPos) }
    }
}

pub struct wxKeyEvent(*mut c_void);
impl _wxKeyEvent for wxKeyEvent {}
impl _wxEvent for wxKeyEvent {}
impl _wxObject for wxKeyEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxKeyEvent {
    pub fn from(handle: *mut c_void) -> @wxKeyEvent { @wxKeyEvent(handle) }
    pub fn null() -> @wxKeyEvent { wxKeyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> c_int {
        unsafe { wxKeyEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> c_int {
        unsafe { wxKeyEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKeyCode(&self) -> c_int {
        unsafe { wxKeyEvent_GetKeyCode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxKeyEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxKeyEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxKeyEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_GetModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_HasModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> c_int {
        unsafe { wxKeyEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setKeyCode(&self, code: c_int) {
        unsafe { wxKeyEvent_SetKeyCode(self.handle(), code) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> c_int {
        unsafe { wxKeyEvent_ShiftDown(self.handle()) }
    }
}

pub struct wxLayoutConstraints(*mut c_void);
impl _wxLayoutConstraints for wxLayoutConstraints {}
impl _wxObject for wxLayoutConstraints { fn handle(&self) -> *mut c_void { **self } }

impl wxLayoutConstraints {
    pub fn from(handle: *mut c_void) -> @wxLayoutConstraints { @wxLayoutConstraints(handle) }
    pub fn null() -> @wxLayoutConstraints { wxLayoutConstraints::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxLayoutConstraints {
        unsafe { @wxLayoutConstraints(wxLayoutConstraints_Create()) }
    }
}

pub trait _wxLayoutConstraints : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn bottom(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_bottom(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centreX(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_centreX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centreY(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_centreY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn height(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_height(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn left(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_left(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn right(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_right(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn top(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_top(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn width(&self) -> *mut c_void {
        unsafe { wxLayoutConstraints_width(self.handle()) }
    }
}

pub struct wxListBox(*mut c_void);
impl _wxListBox for wxListBox {}
impl _wxControl for wxListBox {}
impl _wxWindow for wxListBox {}
impl _wxEvtHandler for wxListBox {}
impl _wxObject for wxListBox { fn handle(&self) -> *mut c_void { **self } }

impl wxListBox {
    pub fn from(handle: *mut c_void) -> @wxListBox { @wxListBox(handle) }
    pub fn null() -> @wxListBox { wxListBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> @wxListBox {
        unsafe { @wxListBox(wxListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

pub trait _wxListBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxListBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendData(&self, item: &str, data: *mut c_void) {
        let item = wxT(item);
        unsafe { wxListBox_AppendData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxListBox_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findString(&self, s: &str) -> c_int {
        let s = wxT(s);
        unsafe { wxListBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxListBox_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxListBox_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelections(&self, aSelections: *mut c_int, allocated: c_int) -> c_int {
        unsafe { wxListBox_GetSelections(self.handle(), aSelections, allocated) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, n: c_int) -> ~str {
        unsafe { wxString { handle: wxListBox_GetString(self.handle(), n) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItems(&self, items: *mut c_void, pos: c_int, count: c_int) {
        unsafe { wxListBox_InsertItems(self.handle(), items, pos, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelected(&self, n: c_int) -> c_int {
        unsafe { wxListBox_IsSelected(self.handle(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFirstItem(&self, n: c_int) {
        unsafe { wxListBox_SetFirstItem(self.handle(), n) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, n: c_int, select: c_int) {
        unsafe { wxListBox_SetSelection(self.handle(), n, select) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setString(&self, n: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxListBox_SetString(self.handle(), n, s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStringSelection(&self, str: &str, sel: c_int) {
        let str = wxT(str);
        unsafe { wxListBox_SetStringSelection(self.handle(), str.handle(), sel) }
    }
}

pub struct wxListCtrl(*mut c_void);
impl _wxListCtrl for wxListCtrl {}
impl _wxControl for wxListCtrl {}
impl _wxWindow for wxListCtrl {}
impl _wxEvtHandler for wxListCtrl {}
impl _wxObject for wxListCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxListCtrl {
    pub fn from(handle: *mut c_void) -> @wxListCtrl { @wxListCtrl(handle) }
    pub fn null() -> @wxListCtrl { wxListCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxListCtrl {
        unsafe { @wxListCtrl(wxListCtrl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxListCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn arrange(&self, flag: c_int) -> c_int {
        unsafe { wxListCtrl_Arrange(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllColumns(&self) -> c_int {
        unsafe { wxListCtrl_DeleteAllColumns(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllItems(&self) -> c_int {
        unsafe { wxListCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteColumn(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_DeleteColumn(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteItem(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_DeleteItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn editLabel(&self, item: c_int) {
        unsafe { wxListCtrl_EditLabel(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endEditLabel(&self, cancel: c_int) -> c_int {
        unsafe { wxListCtrl_EndEditLabel(self.handle(), cancel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisible(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_EnsureVisible(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, start: c_int, str: &str, partial: c_int) -> c_int {
        let str = wxT(str);
        unsafe { wxListCtrl_FindItem(self.handle(), start, str.handle(), partial) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItemByData(&self, start: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByData(self.handle(), start, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItemByPosition(&self, start: c_int, x: c_int, y: c_int, direction: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByPosition(self.handle(), start, x, y, direction) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn<T: _wxListItem>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_GetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumnCount(&self) -> c_int {
        unsafe { wxListCtrl_GetColumnCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumnWidth(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_GetColumnWidth(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCountPerPage(&self) -> c_int {
        unsafe { wxListCtrl_GetCountPerPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEditControl(&self) -> @wxTextCtrl {
        unsafe { @wxTextCtrl(wxListCtrl_GetEditControl(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageList(&self, which: c_int) -> @wxImageList {
        unsafe { @wxImageList(wxListCtrl_GetImageList(self.handle(), which)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem<T: _wxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_GetItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemData(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemData(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemFont(&self, item: c_long) -> @wxFont {
        unsafe { @wxFont(wxListCtrl_GetItemFont(self.handle(), item)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemPosition(&self, item: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxListCtrl_GetItemPosition(self.handle(), item)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemRect(&self, item: c_int, code: c_int) -> @wxRect {
        unsafe { @wxRect(wxListCtrl_GetItemRect(self.handle(), item, code)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemSpacing(&self, isSmall: c_int) -> @wxSize {
        unsafe { @wxSize(wxListCtrl_GetItemSpacing(self.handle(), isSmall)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.handle(), item, stateMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemText(&self, item: c_int) -> ~str {
        unsafe { wxString { handle: wxListCtrl_GetItemText(self.handle(), item) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextItem(&self, item: c_int, geometry: c_int, state: c_int) -> c_int {
        unsafe { wxListCtrl_GetNextItem(self.handle(), item, geometry, state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectedItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetSelectedItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxListCtrl_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTopItem(&self) -> c_int {
        unsafe { wxListCtrl_GetTopItem(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_void) -> c_int {
        unsafe { wxListCtrl_HitTest(self.handle(), x, y, flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertColumn(&self, col: c_int, heading: &str, format: c_int, width: c_int) -> c_int {
        let heading = wxT(heading);
        unsafe { wxListCtrl_InsertColumn(self.handle(), col, heading.handle(), format, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertColumnFromInfo<T: _wxListItem>(&self, col: c_int, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.handle(), col, info.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemWithData(&self, index: c_int, label: &str) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_InsertItemWithData(self.handle(), index, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.handle(), index, imageIndex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemWithLabel(&self, index: c_int, label: &str, imageIndex: c_int) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_InsertItemWithLabel(self.handle(), index, label.handle(), imageIndex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVirtual(&self) -> c_int {
        unsafe { wxListCtrl_IsVirtual(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refreshItem(&self, item: c_long) {
        unsafe { wxListCtrl_RefreshItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollList(&self, dx: c_int, dy: c_int) -> c_int {
        unsafe { wxListCtrl_ScrollList(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumn<T: _wxListItem>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_SetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_SetColumnWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.handle(), imageList.handle(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItem(&self, index: c_int, col: c_int, label: &str, imageId: c_int) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_SetItem(self.handle(), index, col, label.handle(), imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemData(&self, item: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemData(self.handle(), item, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemFromInfo<T: _wxListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_SetItemFromInfo(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemImage(&self, item: c_int, image: c_int, selImage: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemImage(self.handle(), item, image, selImage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemPosition(&self, item: c_int, x: c_int, y: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemPosition(self.handle(), item, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemState(&self, item: c_int, state: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemState(self.handle(), item, state, stateMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemText(&self, item: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxListCtrl_SetItemText(self.handle(), item, str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSingleStyle(&self, style: c_int, add: c_int) {
        unsafe { wxListCtrl_SetSingleStyle(self.handle(), style, add) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxListCtrl_SetTextColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortItems(&self, fn_: *mut c_void, eif_obj: *mut c_void) -> c_int {
        unsafe { wxListCtrl_SortItems(self.handle(), fn_, eif_obj) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignImageList<T: _wxImageList>(&self, images: &T, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.handle(), images.handle(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn2<T: _wxListItem>(&self, col: c_int, item: &T) {
        unsafe { wxListCtrl_GetColumn2(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem2<T: _wxListItem>(&self, info: &T) {
        unsafe { wxListCtrl_GetItem2(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemPosition2(&self, item: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxListCtrl_GetItemPosition2(self.handle(), item)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortItems2<T: _wxClosure>(&self, closure: &T) -> c_int {
        unsafe { wxListCtrl_SortItems2(self.handle(), closure.handle()) }
    }
}

pub struct wxListEvent(*mut c_void);
impl _wxListEvent for wxListEvent {}
impl _wxNotifyEvent for wxListEvent {}
impl _wxCommandEvent for wxListEvent {}
impl _wxEvent for wxListEvent {}
impl _wxObject for wxListEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxListEvent {
    pub fn from(handle: *mut c_void) -> @wxListEvent { @wxListEvent(handle) }
    pub fn null() -> @wxListEvent { wxListEvent::from(0 as *mut c_void) }
    
}

pub trait _wxListEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn cancelled(&self) -> c_int {
        unsafe { wxListEvent_Cancelled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCode(&self) -> c_int {
        unsafe { wxListEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn(&self) -> c_int {
        unsafe { wxListEvent_GetColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> c_int {
        unsafe { wxListEvent_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImage(&self) -> c_int {
        unsafe { wxListEvent_GetImage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndex(&self) -> c_int {
        unsafe { wxListEvent_GetIndex(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem<T: _wxListItem>(&self, _ref: &T) {
        unsafe { wxListEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { handle: wxListEvent_GetLabel(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPoint(&self) -> @wxPoint {
        unsafe { @wxPoint(wxListEvent_GetPoint(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { handle: wxListEvent_GetText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCacheFrom(&self) -> c_int {
        unsafe { wxListEvent_GetCacheFrom(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCacheTo(&self) -> c_int {
        unsafe { wxListEvent_GetCacheTo(self.handle()) }
    }
}

pub struct wxListItem(*mut c_void);
impl _wxListItem for wxListItem {}
impl _wxObject for wxListItem { fn handle(&self) -> *mut c_void { **self } }

impl wxListItem {
    pub fn from(handle: *mut c_void) -> @wxListItem { @wxListItem(handle) }
    pub fn null() -> @wxListItem { wxListItem::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxListItem {
        unsafe { @wxListItem(wxListItem_Create()) }
    }
}

pub trait _wxListItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxListItem_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearAttributes(&self) {
        unsafe { wxListItem_ClearAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlign(&self) -> c_int {
        unsafe { wxListItem_GetAlign(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAttributes(&self) -> *mut c_void {
        unsafe { wxListItem_GetAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxListItem_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColumn(&self) -> c_int {
        unsafe { wxListItem_GetColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> c_int {
        unsafe { wxListItem_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxListItem_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxListItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImage(&self) -> c_int {
        unsafe { wxListItem_GetImage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMask(&self) -> c_int {
        unsafe { wxListItem_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getState(&self) -> c_int {
        unsafe { wxListItem_GetState(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { handle: wxListItem_GetText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxListItem_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxListItem_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasAttributes(&self) -> c_int {
        unsafe { wxListItem_HasAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAlign(&self, align: c_int) {
        unsafe { wxListItem_SetAlign(self.handle(), align) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colBack: &T) {
        unsafe { wxListItem_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumn(&self, col: c_int) {
        unsafe { wxListItem_SetColumn(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData(&self, data: c_int) {
        unsafe { wxListItem_SetData(self.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataPointer(&self, data: *mut c_void) {
        unsafe { wxListItem_SetDataPointer(self.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxListItem_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: c_int) {
        unsafe { wxListItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImage(&self, image: c_int) {
        unsafe { wxListItem_SetImage(self.handle(), image) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMask(&self, mask: c_int) {
        unsafe { wxListItem_SetMask(self.handle(), mask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setState(&self, state: c_int) {
        unsafe { wxListItem_SetState(self.handle(), state) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStateMask(&self, stateMask: c_int) {
        unsafe { wxListItem_SetStateMask(self.handle(), stateMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxListItem_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, colText: &T) {
        unsafe { wxListItem_SetTextColour(self.handle(), colText.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.handle(), width) }
    }
}

pub struct wxLog(*mut c_void);
impl _wxLog for wxLog { fn handle(&self) -> *mut c_void { **self } }

impl wxLog {
    pub fn from(handle: *mut c_void) -> @wxLog { @wxLog(handle) }
    pub fn null() -> @wxLog { wxLog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getActiveTarget() -> @wxLog {
        unsafe { @wxLog(wxLog_GetActiveTarget()) }
    }
}

pub trait _wxLog {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTraceMask(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxLog_AddTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxLog_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn dontCreateOnDemand(&self) {
        unsafe { wxLog_DontCreateOnDemand(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flush(&self) {
        unsafe { wxLog_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flushActive(&self) {
        unsafe { wxLog_FlushActive(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTimestamp(&self) -> *mut c_char {
        unsafe { wxLog_GetTimestamp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTraceMask(&self) -> c_int {
        unsafe { wxLog_GetTraceMask(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVerbose(&self) -> c_int {
        unsafe { wxLog_GetVerbose(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasPendingMessages(&self) -> c_int {
        unsafe { wxLog_HasPendingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAllowedTraceMask<T: _wxMask>(&self, mask: &T) -> c_int {
        unsafe { wxLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onLog(&self, level: c_int, szString: *mut c_void, t: c_int) {
        unsafe { wxLog_OnLog(self.handle(), level, szString, t) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeTraceMask(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxLog_RemoveTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxLog_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setActiveTarget(&self) -> @wxLog {
        unsafe { @wxLog(wxLog_SetActiveTarget(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTimestamp(&self, ts: *mut c_void) {
        unsafe { wxLog_SetTimestamp(self.handle(), ts) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTraceMask(&self, ulMask: c_int) {
        unsafe { wxLog_SetTraceMask(self.handle(), ulMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVerbose(&self, bVerbose: c_int) {
        unsafe { wxLog_SetVerbose(self.handle(), bVerbose) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn suspend(&self) {
        unsafe { wxLog_Suspend(self.handle()) }
    }
}

pub struct wxLogChain(*mut c_void);
impl _wxLogChain for wxLogChain {}
impl _wxLog for wxLogChain { fn handle(&self) -> *mut c_void { **self } }

impl wxLogChain {
    pub fn from(handle: *mut c_void) -> @wxLogChain { @wxLogChain(handle) }
    pub fn null() -> @wxLogChain { wxLogChain::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxLog>(logger: &T) -> @wxLogChain {
        unsafe { @wxLogChain(wxLogChain_Create(logger.handle())) }
    }
}

pub trait _wxLogChain : _wxLog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOldLog(&self) -> @wxLog {
        unsafe { @wxLog(wxLogChain_GetOldLog(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isPassingMessages(&self) -> c_int {
        unsafe { wxLogChain_IsPassingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn passMessages(&self, bDoPass: c_int) {
        unsafe { wxLogChain_PassMessages(self.handle(), bDoPass) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLog<T: _wxLog>(&self, logger: &T) {
        unsafe { wxLogChain_SetLog(self.handle(), logger.handle()) }
    }
}

pub struct wxLogGUI(*mut c_void);
impl _wxLogGUI for wxLogGUI {}
impl _wxLog for wxLogGUI { fn handle(&self) -> *mut c_void { **self } }

impl wxLogGUI {
    pub fn from(handle: *mut c_void) -> @wxLogGUI { @wxLogGUI(handle) }
    pub fn null() -> @wxLogGUI { wxLogGUI::from(0 as *mut c_void) }
    
}

pub trait _wxLogGUI : _wxLog {
}

pub struct wxLogNull(*mut c_void);
impl _wxLogNull for wxLogNull {}
impl _wxLog for wxLogNull { fn handle(&self) -> *mut c_void { **self } }

impl wxLogNull {
    pub fn from(handle: *mut c_void) -> @wxLogNull { @wxLogNull(handle) }
    pub fn null() -> @wxLogNull { wxLogNull::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxLogNull {
        unsafe { @wxLogNull(wxLogNull_Create()) }
    }
}

pub trait _wxLogNull : _wxLog {
}

pub struct wxLogPassThrough(*mut c_void);
impl _wxLogPassThrough for wxLogPassThrough {}
impl _wxLogChain for wxLogPassThrough {}
impl _wxLog for wxLogPassThrough { fn handle(&self) -> *mut c_void { **self } }

impl wxLogPassThrough {
    pub fn from(handle: *mut c_void) -> @wxLogPassThrough { @wxLogPassThrough(handle) }
    pub fn null() -> @wxLogPassThrough { wxLogPassThrough::from(0 as *mut c_void) }
    
}

pub trait _wxLogPassThrough : _wxLogChain {
}

pub struct wxLogStderr(*mut c_void);
impl _wxLogStderr for wxLogStderr {}
impl _wxLog for wxLogStderr { fn handle(&self) -> *mut c_void { **self } }

impl wxLogStderr {
    pub fn from(handle: *mut c_void) -> @wxLogStderr { @wxLogStderr(handle) }
    pub fn null() -> @wxLogStderr { wxLogStderr::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxLogStderr {
        unsafe { @wxLogStderr(wxLogStderr_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newStdOut() -> @wxLogStderr {
        unsafe { @wxLogStderr(wxLogStderr_CreateStdOut()) }
    }
}

pub trait _wxLogStderr : _wxLog {
}

pub struct wxLogStream(*mut c_void);
impl _wxLogStream for wxLogStream {}
impl _wxLog for wxLogStream { fn handle(&self) -> *mut c_void { **self } }

impl wxLogStream {
    pub fn from(handle: *mut c_void) -> @wxLogStream { @wxLogStream(handle) }
    pub fn null() -> @wxLogStream { wxLogStream::from(0 as *mut c_void) }
    
}

pub trait _wxLogStream : _wxLog {
}

pub struct wxLogTextCtrl(*mut c_void);
impl _wxLogTextCtrl for wxLogTextCtrl {}
impl _wxLog for wxLogTextCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxLogTextCtrl {
    pub fn from(handle: *mut c_void) -> @wxLogTextCtrl { @wxLogTextCtrl(handle) }
    pub fn null() -> @wxLogTextCtrl { wxLogTextCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxTextCtrl>(text: &T) -> @wxLogTextCtrl {
        unsafe { @wxLogTextCtrl(wxLogTextCtrl_Create(text.handle())) }
    }
}

pub trait _wxLogTextCtrl : _wxLog {
}

pub struct wxLogWindow(*mut c_void);
impl _wxLogWindow for wxLogWindow {}
impl _wxLogPassThrough for wxLogWindow {}
impl _wxLogChain for wxLogWindow {}
impl _wxLog for wxLogWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxLogWindow {
    pub fn from(handle: *mut c_void) -> @wxLogWindow { @wxLogWindow(handle) }
    pub fn null() -> @wxLogWindow { wxLogWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, title: *mut int8_t, showit: c_int, passthrough: c_int) -> @wxLogWindow {
        unsafe { @wxLogWindow(wxLogWindow_Create(parent.handle(), title, showit, passthrough)) }
    }
}

pub trait _wxLogWindow : _wxLogPassThrough {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrame(wxLogWindow_GetFrame(self.handle())) }
    }
}

pub struct wxMDIChildFrame(*mut c_void);
impl _wxMDIChildFrame for wxMDIChildFrame {}
impl _wxFrame for wxMDIChildFrame {}
impl _wxTopLevelWindow for wxMDIChildFrame {}
impl _wxWindow for wxMDIChildFrame {}
impl _wxEvtHandler for wxMDIChildFrame {}
impl _wxObject for wxMDIChildFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxMDIChildFrame {
    pub fn from(handle: *mut c_void) -> @wxMDIChildFrame { @wxMDIChildFrame(handle) }
    pub fn null() -> @wxMDIChildFrame { wxMDIChildFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMDIChildFrame {
        let _txt = wxT(_txt);
        unsafe { @wxMDIChildFrame(wxMDIChildFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxMDIChildFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.handle()) }
    }
}

pub struct wxMDIClientWindow(*mut c_void);
impl _wxMDIClientWindow for wxMDIClientWindow {}
impl _wxWindow for wxMDIClientWindow {}
impl _wxEvtHandler for wxMDIClientWindow {}
impl _wxObject for wxMDIClientWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxMDIClientWindow {
    pub fn from(handle: *mut c_void) -> @wxMDIClientWindow { @wxMDIClientWindow(handle) }
    pub fn null() -> @wxMDIClientWindow { wxMDIClientWindow::from(0 as *mut c_void) }
    
}

pub trait _wxMDIClientWindow : _wxWindow {
}

pub struct wxMDIParentFrame(*mut c_void);
impl _wxMDIParentFrame for wxMDIParentFrame {}
impl _wxFrame for wxMDIParentFrame {}
impl _wxTopLevelWindow for wxMDIParentFrame {}
impl _wxWindow for wxMDIParentFrame {}
impl _wxEvtHandler for wxMDIParentFrame {}
impl _wxObject for wxMDIParentFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxMDIParentFrame {
    pub fn from(handle: *mut c_void) -> @wxMDIParentFrame { @wxMDIParentFrame(handle) }
    pub fn null() -> @wxMDIParentFrame { wxMDIParentFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMDIParentFrame {
        let _txt = wxT(_txt);
        unsafe { @wxMDIParentFrame(wxMDIParentFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxMDIParentFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn activateNext(&self) {
        unsafe { wxMDIParentFrame_ActivateNext(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn activatePrevious(&self) {
        unsafe { wxMDIParentFrame_ActivatePrevious(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn arrangeIcons(&self) {
        unsafe { wxMDIParentFrame_ArrangeIcons(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cascade(&self) {
        unsafe { wxMDIParentFrame_Cascade(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getActiveChild(&self) -> @wxMDIChildFrame {
        unsafe { @wxMDIChildFrame(wxMDIParentFrame_GetActiveChild(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientWindow(&self) -> @wxMDIClientWindow {
        unsafe { @wxMDIClientWindow(wxMDIParentFrame_GetClientWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindowMenu(&self) -> @wxMenu {
        unsafe { @wxMenu(wxMDIParentFrame_GetWindowMenu(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onCreateClient(&self) -> @wxMDIClientWindow {
        unsafe { @wxMDIClientWindow(wxMDIParentFrame_OnCreateClient(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.handle()) }
    }
}

pub struct wxMask(*mut c_void);
impl _wxMask for wxMask {}
impl _wxObject for wxMask { fn handle(&self) -> *mut c_void { **self } }

impl wxMask {
    pub fn from(handle: *mut c_void) -> @wxMask { @wxMask(handle) }
    pub fn null() -> @wxMask { wxMask::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxBitmap>(bitmap: &T) -> @wxMask {
        unsafe { @wxMask(wxMask_Create(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newColoured<T: _wxBitmap, U: _wxColour>(bitmap: &T, colour: &U) -> *mut c_void {
        unsafe { wxMask_CreateColoured(bitmap.handle(), colour.handle()) }
    }
}

pub trait _wxMask : _wxObject {
}

pub struct wxMaximizeEvent(*mut c_void);
impl _wxMaximizeEvent for wxMaximizeEvent {}
impl _wxEvent for wxMaximizeEvent {}
impl _wxObject for wxMaximizeEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxMaximizeEvent {
    pub fn from(handle: *mut c_void) -> @wxMaximizeEvent { @wxMaximizeEvent(handle) }
    pub fn null() -> @wxMaximizeEvent { wxMaximizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMaximizeEvent : _wxEvent {
}

pub struct wxMemoryDC(*mut c_void);
impl _wxMemoryDC for wxMemoryDC {}
impl _wxDC for wxMemoryDC {}
impl _wxObject for wxMemoryDC { fn handle(&self) -> *mut c_void { **self } }

impl wxMemoryDC {
    pub fn from(handle: *mut c_void) -> @wxMemoryDC { @wxMemoryDC(handle) }
    pub fn null() -> @wxMemoryDC { wxMemoryDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxMemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newCompatible<T: _wxDC>(dc: &T) -> @wxMemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_CreateCompatible(dc.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithBitmap<T: _wxBitmap>(bitmap: &T) -> @wxMemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_CreateWithBitmap(bitmap.handle())) }
    }
}

pub trait _wxMemoryDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectObject<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxMemoryDC_SelectObject(self.handle(), bitmap.handle()) }
    }
}

pub struct wxMenu(*mut c_void);
impl _wxMenu for wxMenu {}
impl _wxEvtHandler for wxMenu {}
impl _wxObject for wxMenu { fn handle(&self) -> *mut c_void { **self } }

impl wxMenu {
    pub fn from(handle: *mut c_void) -> @wxMenu { @wxMenu(handle) }
    pub fn null() -> @wxMenu { wxMenu::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(title: &str, style: c_long) -> @wxMenu {
        let title = wxT(title);
        unsafe { @wxMenu(wxMenu_Create(title.handle(), style)) }
    }
}

pub trait _wxMenu : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Append(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_AppendItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendSub<T: _wxMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_AppendSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn break_(&self) {
        unsafe { wxMenu_Break(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, id: c_int, check: c_int) {
        unsafe { wxMenu_Check(self.handle(), id, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteById(&self, id: c_int) {
        unsafe { wxMenu_DeleteById(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteByItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DeleteByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyById(&self, id: c_int) {
        unsafe { wxMenu_DestroyById(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyByItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DestroyByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, id: c_int, enable: c_int) {
        unsafe { wxMenu_Enable(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, id: c_int) -> @wxMenuItem {
        unsafe { @wxMenuItem(wxMenu_FindItem(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItemByLabel(&self, itemString: &str) -> c_int {
        let itemString = wxT(itemString);
        unsafe { wxMenu_FindItemByLabel(self.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientData(&self) -> @wxClientData {
        unsafe { @wxClientData(wxMenu_GetClientData(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { wxString { handle: wxMenu_GetHelpString(self.handle(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInvokingWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxMenu_GetInvokingWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { wxString { handle: wxMenu_GetLabel(self.handle(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuItems<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getParent(&self) -> @wxMenu {
        unsafe { @wxMenu(wxMenu_GetParent(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { handle: wxMenu_GetTitle(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insert(&self, pos: size_t, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Insert(self.handle(), pos, id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxMenuItem>(&self, pos: size_t, _itm: &T) {
        unsafe { wxMenu_InsertItem(self.handle(), pos, _itm.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSub<T: _wxMenu>(&self, pos: size_t, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_InsertSub(self.handle(), pos, id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAttached(&self) -> c_int {
        unsafe { wxMenu_IsAttached(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenu_IsChecked(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenu_IsEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepend(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Prepend(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependItem<T: _wxMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_PrependItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSub<T: _wxMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_PrependSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeById<T: _wxMenuItem>(&self, id: c_int, _itm: &T) {
        unsafe { wxMenu_RemoveById(self.handle(), id, _itm.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeByItem(&self, item: *mut c_void) {
        unsafe { wxMenu_RemoveByItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientData<T: _wxClientData>(&self, clientData: &T) {
        unsafe { wxMenu_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEventHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxMenu_SetEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenu_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInvokingWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxMenu_SetInvokingWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenu_SetLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setParent<T: _wxWindow>(&self, parent: &T) {
        unsafe { wxMenu_SetParent(self.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTitle(&self, title: &str) {
        let title = wxT(title);
        unsafe { wxMenu_SetTitle(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateUI(&self, source: *mut c_void) {
        unsafe { wxMenu_UpdateUI(self.handle(), source) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuBar(&self) -> @wxMenuBar {
        unsafe { @wxMenuBar(wxMenu_GetMenuBar(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendRadioItem(&self, id: c_int, text: &str, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_AppendRadioItem(self.handle(), id, text.handle(), help.handle()) }
    }
}

pub struct wxMenuBar(*mut c_void);
impl _wxMenuBar for wxMenuBar {}
impl _wxEvtHandler for wxMenuBar {}
impl _wxObject for wxMenuBar { fn handle(&self) -> *mut c_void { **self } }

impl wxMenuBar {
    pub fn from(handle: *mut c_void) -> @wxMenuBar { @wxMenuBar(handle) }
    pub fn null() -> @wxMenuBar { wxMenuBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_style: c_int) -> @wxMenuBar {
        unsafe { @wxMenuBar(wxMenuBar_Create(_style)) }
    }
}

pub trait _wxMenuBar : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append<T: _wxMenu>(&self, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Append(self.handle(), menu.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, id: c_int, check: c_int) {
        unsafe { wxMenuBar_Check(self.handle(), id, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePointer(&self) {
        unsafe { wxMenuBar_DeletePointer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: c_int) -> c_int {
        unsafe { wxMenuBar_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableItem(&self, id: c_int, enable: c_int) {
        unsafe { wxMenuBar_EnableItem(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableTop(&self, pos: c_int, enable: c_int) {
        unsafe { wxMenuBar_EnableTop(self.handle(), pos, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, id: c_int) -> @wxMenuItem {
        unsafe { @wxMenuItem(wxMenuBar_FindItem(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findMenu(&self, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_FindMenu(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findMenuItem(&self, menuString: &str, itemString: &str) -> c_int {
        let menuString = wxT(menuString);
        let itemString = wxT(itemString);
        unsafe { wxMenuBar_FindMenuItem(self.handle(), menuString.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { wxString { handle: wxMenuBar_GetHelpString(self.handle(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { wxString { handle: wxMenuBar_GetLabel(self.handle(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelTop(&self, pos: c_int) -> ~str {
        unsafe { wxString { handle: wxMenuBar_GetLabelTop(self.handle(), pos) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenu(&self, pos: c_int) -> @wxMenu {
        unsafe { @wxMenu(wxMenuBar_GetMenu(self.handle(), pos)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insert<T: _wxMenu>(&self, pos: c_int, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Insert(self.handle(), pos, menu.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsChecked(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, pos: c_int) -> @wxMenu {
        unsafe { @wxMenu(wxMenuBar_Remove(self.handle(), pos)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxMenu>(&self, pos: c_int, menu: &T, title: &str) -> @wxMenu {
        let title = wxT(title);
        unsafe { @wxMenu(wxMenuBar_Replace(self.handle(), pos, menu.handle(), title.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenuBar_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenuBar_SetItemLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabel(&self, s: &str) {
        let s = wxT(s);
        unsafe { wxMenuBar_SetLabel(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabelTop(&self, pos: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenuBar_SetLabelTop(self.handle(), pos, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrame(wxMenuBar_GetFrame(self.handle())) }
    }
}

pub struct wxMenuEvent(*mut c_void);
impl _wxMenuEvent for wxMenuEvent {}
impl _wxEvent for wxMenuEvent {}
impl _wxObject for wxMenuEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxMenuEvent {
    pub fn from(handle: *mut c_void) -> @wxMenuEvent { @wxMenuEvent(handle) }
    pub fn null() -> @wxMenuEvent { wxMenuEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMenuEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.handle()) }
    }
}

pub struct wxMenuItem(*mut c_void);
impl _wxMenuItem for wxMenuItem {}
impl _wxObject for wxMenuItem { fn handle(&self) -> *mut c_void { **self } }

impl wxMenuItem {
    pub fn from(handle: *mut c_void) -> @wxMenuItem { @wxMenuItem(handle) }
    pub fn null() -> @wxMenuItem { wxMenuItem::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxMenuItem {
        unsafe { @wxMenuItem(wxMenuItem_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getLabelFromText(text: *mut c_void) -> ~str {
        unsafe { wxString { handle: wxMenuItem_GetLabelFromText(text) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newSeparator() -> @wxMenuItem {
        unsafe { @wxMenuItem(wxMenuItem_CreateSeparator()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEx<T: _wxMenu>(id: c_int, label: &str, help: &str, itemkind: c_int, submenu: &T) -> @wxMenuItem {
        let label = wxT(label);
        let help = wxT(help);
        unsafe { @wxMenuItem(wxMenuItem_CreateEx(id, label.handle(), help.handle(), itemkind, submenu.handle())) }
    }
}

pub trait _wxMenuItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, check: c_int) {
        unsafe { wxMenuItem_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: c_int) {
        unsafe { wxMenuItem_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHelp(&self) -> ~str {
        unsafe { wxString { handle: wxMenuItem_GetHelp(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { handle: wxMenuItem_GetLabel(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenu(&self) -> @wxMenu {
        unsafe { @wxMenu(wxMenuItem_GetMenu(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSubMenu(&self) -> @wxMenu {
        unsafe { @wxMenu(wxMenuItem_GetSubMenu(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { handle: wxMenuItem_GetText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCheckable(&self) -> c_int {
        unsafe { wxMenuItem_IsCheckable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self) -> c_int {
        unsafe { wxMenuItem_IsChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> c_int {
        unsafe { wxMenuItem_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSeparator(&self) -> c_int {
        unsafe { wxMenuItem_IsSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSubMenu(&self) -> c_int {
        unsafe { wxMenuItem_IsSubMenu(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCheckable(&self, checkable: c_int) {
        unsafe { wxMenuItem_SetCheckable(self.handle(), checkable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelp(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxMenuItem_SetHelp(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSubMenu<T: _wxMenu>(&self, menu: &T) {
        unsafe { wxMenuItem_SetSubMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxMenuItem_SetText(self.handle(), str.handle()) }
    }
}

pub struct wxMessageDialog(*mut c_void);
impl _wxMessageDialog for wxMessageDialog {}
impl _wxDialog for wxMessageDialog {}
impl _wxTopLevelWindow for wxMessageDialog {}
impl _wxWindow for wxMessageDialog {}
impl _wxEvtHandler for wxMessageDialog {}
impl _wxObject for wxMessageDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxMessageDialog {
    pub fn from(handle: *mut c_void) -> @wxMessageDialog { @wxMessageDialog(handle) }
    pub fn null() -> @wxMessageDialog { wxMessageDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _cap: &str, _stl: c_int) -> @wxMessageDialog {
        let _msg = wxT(_msg);
        let _cap = wxT(_cap);
        unsafe { @wxMessageDialog(wxMessageDialog_Create(_prt.handle(), _msg.handle(), _cap.handle(), _stl)) }
    }
}

pub trait _wxMessageDialog : _wxDialog {
}

pub struct wxMetafile(*mut c_void);
impl _wxMetafile for wxMetafile {}
impl _wxObject for wxMetafile { fn handle(&self) -> *mut c_void { **self } }

impl wxMetafile {
    pub fn from(handle: *mut c_void) -> @wxMetafile { @wxMetafile(handle) }
    pub fn null() -> @wxMetafile { wxMetafile::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_file: &str) -> @wxMetafile {
        let _file = wxT(_file);
        unsafe { @wxMetafile(wxMetafile_Create(_file.handle())) }
    }
}

pub trait _wxMetafile : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxMetafile_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play<T: _wxDC>(&self, _dc: &T) -> c_int {
        unsafe { wxMetafile_Play(self.handle(), _dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClipboard(&self, width: c_int, height: c_int) -> c_int {
        unsafe { wxMetafile_SetClipboard(self.handle(), width, height) }
    }
}

pub struct wxMetafileDC(*mut c_void);
impl _wxMetafileDC for wxMetafileDC {}
impl _wxDC for wxMetafileDC {}
impl _wxObject for wxMetafileDC { fn handle(&self) -> *mut c_void { **self } }

impl wxMetafileDC {
    pub fn from(handle: *mut c_void) -> @wxMetafileDC { @wxMetafileDC(handle) }
    pub fn null() -> @wxMetafileDC { wxMetafileDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_file: &str) -> @wxMetafileDC {
        let _file = wxT(_file);
        unsafe { @wxMetafileDC(wxMetafileDC_Create(_file.handle())) }
    }
}

pub trait _wxMetafileDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self) -> *mut c_void {
        unsafe { wxMetafileDC_Close(self.handle()) }
    }
}

pub struct wxMimeTypesManager(*mut c_void);
impl _wxMimeTypesManager for wxMimeTypesManager { fn handle(&self) -> *mut c_void { **self } }

impl wxMimeTypesManager {
    pub fn from(handle: *mut c_void) -> @wxMimeTypesManager { @wxMimeTypesManager(handle) }
    pub fn null() -> @wxMimeTypesManager { wxMimeTypesManager::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxMimeTypesManager {
        unsafe { @wxMimeTypesManager(wxMimeTypesManager_Create()) }
    }
}

pub trait _wxMimeTypesManager {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFallbacks(&self, _types: *mut c_void) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.handle(), _types) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumAllFileTypes<T: _wxList>(&self, _lst: &T) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFileTypeFromExtension(&self, _ext: &str) -> @wxFileType {
        let _ext = wxT(_ext);
        unsafe { @wxFileType(wxMimeTypesManager_GetFileTypeFromExtension(self.handle(), _ext.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFileTypeFromMimeType(&self, _name: &str) -> @wxFileType {
        let _name = wxT(_name);
        unsafe { @wxFileType(wxMimeTypesManager_GetFileTypeFromMimeType(self.handle(), _name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOfType(&self, _type: &str, _wildcard: &str) -> c_int {
        let _type = wxT(_type);
        let _wildcard = wxT(_wildcard);
        unsafe { wxMimeTypesManager_IsOfType(self.handle(), _type.handle(), _wildcard.handle()) }
    }
}

pub struct wxMiniFrame(*mut c_void);
impl _wxMiniFrame for wxMiniFrame {}
impl _wxFrame for wxMiniFrame {}
impl _wxTopLevelWindow for wxMiniFrame {}
impl _wxWindow for wxMiniFrame {}
impl _wxEvtHandler for wxMiniFrame {}
impl _wxObject for wxMiniFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxMiniFrame {
    pub fn from(handle: *mut c_void) -> @wxMiniFrame { @wxMiniFrame(handle) }
    pub fn null() -> @wxMiniFrame { wxMiniFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMiniFrame {
        let _txt = wxT(_txt);
        unsafe { @wxMiniFrame(wxMiniFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxMiniFrame : _wxFrame {
}

pub struct wxMirrorDC(*mut c_void);
impl _wxMirrorDC for wxMirrorDC {}
impl _wxDC for wxMirrorDC {}
impl _wxObject for wxMirrorDC { fn handle(&self) -> *mut c_void { **self } }

impl wxMirrorDC {
    pub fn from(handle: *mut c_void) -> @wxMirrorDC { @wxMirrorDC(handle) }
    pub fn null() -> @wxMirrorDC { wxMirrorDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxDC>(dc: &T) -> @wxMirrorDC {
        unsafe { @wxMirrorDC(wxMirrorDC_Create(dc.handle())) }
    }
}

pub trait _wxMirrorDC : _wxDC {
}

pub struct wxMouseCaptureChangedEvent(*mut c_void);
impl _wxMouseCaptureChangedEvent for wxMouseCaptureChangedEvent {}
impl _wxEvent for wxMouseCaptureChangedEvent {}
impl _wxObject for wxMouseCaptureChangedEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxMouseCaptureChangedEvent {
    pub fn from(handle: *mut c_void) -> @wxMouseCaptureChangedEvent { @wxMouseCaptureChangedEvent(handle) }
    pub fn null() -> @wxMouseCaptureChangedEvent { wxMouseCaptureChangedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMouseCaptureChangedEvent : _wxEvent {
}

pub struct wxMouseEvent(*mut c_void);
impl _wxMouseEvent for wxMouseEvent {}
impl _wxEvent for wxMouseEvent {}
impl _wxObject for wxMouseEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxMouseEvent {
    pub fn from(handle: *mut c_void) -> @wxMouseEvent { @wxMouseEvent(handle) }
    pub fn null() -> @wxMouseEvent { wxMouseEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMouseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> c_int {
        unsafe { wxMouseEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn button(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_Button(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDClick(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonDClick(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDown(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonIsDown(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonIsDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonUp(&self, but: c_int) -> c_int {
        unsafe { wxMouseEvent_ButtonUp(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> c_int {
        unsafe { wxMouseEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn dragging(&self) -> c_int {
        unsafe { wxMouseEvent_Dragging(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn entering(&self) -> c_int {
        unsafe { wxMouseEvent_Entering(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalPosition<T: _wxDC>(&self, dc: &T) -> @wxPoint {
        unsafe { @wxPoint(wxMouseEvent_GetLogicalPosition(self.handle(), dc.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxMouseEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxMouseEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxMouseEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isButton(&self) -> c_int {
        unsafe { wxMouseEvent_IsButton(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leaving(&self) -> c_int {
        unsafe { wxMouseEvent_Leaving(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftDClick(&self) -> c_int {
        unsafe { wxMouseEvent_LeftDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftDown(&self) -> c_int {
        unsafe { wxMouseEvent_LeftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_LeftIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftUp(&self) -> c_int {
        unsafe { wxMouseEvent_LeftUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> c_int {
        unsafe { wxMouseEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleDClick(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleDown(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleUp(&self) -> c_int {
        unsafe { wxMouseEvent_MiddleUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moving(&self) -> c_int {
        unsafe { wxMouseEvent_Moving(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightDClick(&self) -> c_int {
        unsafe { wxMouseEvent_RightDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightDown(&self) -> c_int {
        unsafe { wxMouseEvent_RightDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightIsDown(&self) -> c_int {
        unsafe { wxMouseEvent_RightIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightUp(&self) -> c_int {
        unsafe { wxMouseEvent_RightUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> c_int {
        unsafe { wxMouseEvent_ShiftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWheelDelta(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelDelta(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWheelRotation(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelRotation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButton(&self) -> c_int {
        unsafe { wxMouseEvent_GetButton(self.handle()) }
    }
}

pub struct wxMoveEvent(*mut c_void);
impl _wxMoveEvent for wxMoveEvent {}
impl _wxEvent for wxMoveEvent {}
impl _wxObject for wxMoveEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxMoveEvent {
    pub fn from(handle: *mut c_void) -> @wxMoveEvent { @wxMoveEvent(handle) }
    pub fn null() -> @wxMoveEvent { wxMoveEvent::from(0 as *mut c_void) }
    
}

pub trait _wxMoveEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxMoveEvent_GetPosition(self.handle())) }
    }
}

pub struct wxNavigationKeyEvent(*mut c_void);
impl _wxNavigationKeyEvent for wxNavigationKeyEvent {}
impl _wxEvent for wxNavigationKeyEvent {}
impl _wxObject for wxNavigationKeyEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxNavigationKeyEvent {
    pub fn from(handle: *mut c_void) -> @wxNavigationKeyEvent { @wxNavigationKeyEvent(handle) }
    pub fn null() -> @wxNavigationKeyEvent { wxNavigationKeyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxNavigationKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentFocus(&self) -> *mut c_void {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirection(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_GetDirection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWindowChange(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentFocus<T: _wxWindow>(&self, win: &T) {
        unsafe { wxNavigationKeyEvent_SetCurrentFocus(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDirection(&self, bForward: c_int) {
        unsafe { wxNavigationKeyEvent_SetDirection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowChange(&self, bIs: c_int) {
        unsafe { wxNavigationKeyEvent_SetWindowChange(self.handle(), bIs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shouldPropagate(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_ShouldPropagate(self.handle()) }
    }
}

pub struct wxNotebook(*mut c_void);
impl _wxNotebook for wxNotebook {}
impl _wxControl for wxNotebook {}
impl _wxWindow for wxNotebook {}
impl _wxEvtHandler for wxNotebook {}
impl _wxObject for wxNotebook { fn handle(&self) -> *mut c_void { **self } }

impl wxNotebook {
    pub fn from(handle: *mut c_void) -> @wxNotebook { @wxNotebook(handle) }
    pub fn null() -> @wxNotebook { wxNotebook::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxNotebook {
        unsafe { @wxNotebook(wxNotebook_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxNotebook : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPage<T: _wxWindow>(&self, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_AddPage(self.handle(), pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn advanceSelection(&self, bForward: c_int) {
        unsafe { wxNotebook_AdvanceSelection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllPages(&self) -> c_int {
        unsafe { wxNotebook_DeleteAllPages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_DeletePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageList(&self) -> @wxImageList {
        unsafe { @wxImageList(wxNotebook_GetImageList(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPage(&self, nPage: c_int) -> @wxWindow {
        unsafe { @wxWindow(wxNotebook_GetPage(self.handle(), nPage)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageCount(&self) -> c_int {
        unsafe { wxNotebook_GetPageCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageImage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_GetPageImage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageText(&self, nPage: c_int) -> ~str {
        unsafe { wxString { handle: wxNotebook_GetPageText(self.handle(), nPage) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowCount(&self) -> c_int {
        unsafe { wxNotebook_GetRowCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxNotebook_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest(&self, x: c_int, y: c_int, flags: *mut c_long) -> c_int {
        unsafe { wxNotebook_HitTest(self.handle(), x, y, flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertPage<T: _wxWindow>(&self, nPage: c_int, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_InsertPage(self.handle(), nPage, pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_RemovePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxNotebook_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPadding(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPadding(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageImage(&self, nPage: c_int, nImage: c_int) -> c_int {
        unsafe { wxNotebook_SetPageImage(self.handle(), nPage, nImage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSize(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPageSize(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageText(&self, nPage: c_int, strText: &str) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_SetPageText(self.handle(), nPage, strText.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxNotebook_AssignImageList(self.handle(), imageList.handle()) }
    }
}

pub struct wxNotebookEvent(*mut c_void);
impl _wxNotebookEvent for wxNotebookEvent {}
impl _wxNotifyEvent for wxNotebookEvent {}
impl _wxCommandEvent for wxNotebookEvent {}
impl _wxEvent for wxNotebookEvent {}
impl _wxObject for wxNotebookEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxNotebookEvent {
    pub fn from(handle: *mut c_void) -> @wxNotebookEvent { @wxNotebookEvent(handle) }
    pub fn null() -> @wxNotebookEvent { wxNotebookEvent::from(0 as *mut c_void) }
    
}

pub trait _wxNotebookEvent : _wxNotifyEvent {
}

pub struct wxNotifyEvent(*mut c_void);
impl _wxNotifyEvent for wxNotifyEvent {}
impl _wxCommandEvent for wxNotifyEvent {}
impl _wxEvent for wxNotifyEvent {}
impl _wxObject for wxNotifyEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxNotifyEvent {
    pub fn from(handle: *mut c_void) -> @wxNotifyEvent { @wxNotifyEvent(handle) }
    pub fn null() -> @wxNotifyEvent { wxNotifyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxNotifyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn allow(&self) {
        unsafe { wxNotifyEvent_Allow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAllowed(&self) -> c_int {
        unsafe { wxNotifyEvent_IsAllowed(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn veto(&self) {
        unsafe { wxNotifyEvent_Veto(self.handle()) }
    }
}

pub struct wxPageSetupDialog(*mut c_void);
impl _wxPageSetupDialog for wxPageSetupDialog {}
impl _wxDialog for wxPageSetupDialog {}
impl _wxTopLevelWindow for wxPageSetupDialog {}
impl _wxWindow for wxPageSetupDialog {}
impl _wxEvtHandler for wxPageSetupDialog {}
impl _wxObject for wxPageSetupDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxPageSetupDialog {
    pub fn from(handle: *mut c_void) -> @wxPageSetupDialog { @wxPageSetupDialog(handle) }
    pub fn null() -> @wxPageSetupDialog { wxPageSetupDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPageSetupDialogData>(parent: &T, data: &U) -> @wxPageSetupDialog {
        unsafe { @wxPageSetupDialog(wxPageSetupDialog_Create(parent.handle(), data.handle())) }
    }
}

pub trait _wxPageSetupDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSetupData<T: _wxPageSetupDialogData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.handle(), _ref.handle()) }
    }
}

pub struct wxPageSetupDialogData(*mut c_void);
impl _wxPageSetupDialogData for wxPageSetupDialogData {}
impl _wxObject for wxPageSetupDialogData { fn handle(&self) -> *mut c_void { **self } }

impl wxPageSetupDialogData {
    pub fn from(handle: *mut c_void) -> @wxPageSetupDialogData { @wxPageSetupDialogData(handle) }
    pub fn null() -> @wxPageSetupDialogData { wxPageSetupDialogData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxPageSetupDialogData {
        unsafe { @wxPageSetupDialogData(wxPageSetupDialogData_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData<T: _wxPrintData>(printData: &T) -> @wxPageSetupDialogData {
        unsafe { @wxPageSetupDialogData(wxPageSetupDialogData_CreateFromData(printData.handle())) }
    }
}

pub trait _wxPageSetupDialogData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPageSetupDialogData>(&self, data: &T) {
        unsafe { wxPageSetupDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignData<T: _wxPrintData>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_AssignData(self.handle(), printData.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calculateIdFromPaperSize(&self) {
        unsafe { wxPageSetupDialogData_CalculateIdFromPaperSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calculatePaperSizeFromId(&self) {
        unsafe { wxPageSetupDialogData_CalculatePaperSizeFromId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableHelp(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableHelp(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableMargins(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableOrientation(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnableOrientation(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePaper(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnablePaper(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePrinter(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_EnablePrinter(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultInfo(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetDefaultInfo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultMinMargins(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetDefaultMinMargins(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableHelp(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableMargins(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableMargins(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableOrientation(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnableOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePaper(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnablePaper(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePrinter(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetEnablePrinter(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginBottomRight(&self) -> @wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMarginBottomRight(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginTopLeft(&self) -> @wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMarginTopLeft(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinMarginBottomRight(&self) -> @wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMinMarginBottomRight(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinMarginTopLeft(&self) -> @wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMinMarginTopLeft(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperSize(&self) -> @wxSize {
        unsafe { @wxSize(wxPageSetupDialogData_GetPaperSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintData<T: _wxPrintData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialogData_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultInfo(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultInfo(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultMinMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultMinMargins(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginBottomRight(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginTopLeft(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginBottomRight(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginTopLeft(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperId(&self, id: *mut c_void) {
        unsafe { wxPageSetupDialogData_SetPaperId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperSizeId(&self, id: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSizeId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintData<T: _wxPrintData>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
}

pub struct wxPaintDC(*mut c_void);
impl _wxPaintDC for wxPaintDC {}
impl _wxWindowDC for wxPaintDC {}
impl _wxDC for wxPaintDC {}
impl _wxObject for wxPaintDC { fn handle(&self) -> *mut c_void { **self } }

impl wxPaintDC {
    pub fn from(handle: *mut c_void) -> @wxPaintDC { @wxPaintDC(handle) }
    pub fn null() -> @wxPaintDC { wxPaintDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> @wxPaintDC {
        unsafe { @wxPaintDC(wxPaintDC_Create(win.handle())) }
    }
}

pub trait _wxPaintDC : _wxWindowDC {
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

pub struct wxPalette(*mut c_void);
impl _wxPalette for wxPalette {}
impl _wxGDIObject for wxPalette {}
impl _wxObject for wxPalette { fn handle(&self) -> *mut c_void { **self } }

impl wxPalette {
    pub fn from(handle: *mut c_void) -> @wxPalette { @wxPalette(handle) }
    pub fn null() -> @wxPalette { wxPalette::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxPalette {
        unsafe { @wxPalette(wxPalette_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRGB(n: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> @wxPalette {
        unsafe { @wxPalette(wxPalette_CreateRGB(n, red, green, blue)) }
    }
}

pub trait _wxPalette : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPalette>(&self, palette: &T) {
        unsafe { wxPalette_Assign(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPixel(&self, red: uint8_t, green: uint8_t, blue: uint8_t) -> c_int {
        unsafe { wxPalette_GetPixel(self.handle(), red, green, blue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRGB(&self, pixel: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> c_int {
        unsafe { wxPalette_GetRGB(self.handle(), pixel, red, green, blue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxPalette>(&self, palette: &T) -> c_int {
        unsafe { wxPalette_IsEqual(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxPalette_IsOk(self.handle()) }
    }
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

pub struct wxPanel(*mut c_void);
impl _wxPanel for wxPanel {}
impl _wxWindow for wxPanel {}
impl _wxEvtHandler for wxPanel {}
impl _wxObject for wxPanel { fn handle(&self) -> *mut c_void { **self } }

impl wxPanel {
    pub fn from(handle: *mut c_void) -> @wxPanel { @wxPanel(handle) }
    pub fn null() -> @wxPanel { wxPanel::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxPanel {
        unsafe { @wxPanel(wxPanel_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxPanel : _wxWindow {
}

pub struct wxPen(*mut c_void);
impl _wxPen for wxPen {}
impl _wxGDIObject for wxPen {}
impl _wxObject for wxPen { fn handle(&self) -> *mut c_void { **self } }

impl wxPen {
    pub fn from(handle: *mut c_void) -> @wxPen { @wxPen(handle) }
    pub fn null() -> @wxPen { wxPen::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxPen {
        unsafe { @wxPen(wxPen_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromBitmap<T: _wxBitmap>(stipple: &T, width: c_int) -> @wxPen {
        unsafe { @wxPen(wxPen_CreateFromBitmap(stipple.handle(), width)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromColour<T: _wxColour>(col: &T, width: c_int, style: c_int) -> @wxPen {
        unsafe { @wxPen(wxPen_CreateFromColour(col.handle(), width, style)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromStock(id: c_int) -> @wxPen {
        unsafe { @wxPen(wxPen_CreateFromStock(id)) }
    }
}

pub trait _wxPen : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPen>(&self, pen: &T) {
        unsafe { wxPen_Assign(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCap(&self) -> c_int {
        unsafe { wxPen_GetCap(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxPen_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDashes(&self, ptr: *mut c_void) -> c_int {
        unsafe { wxPen_GetDashes(self.handle(), ptr) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getJoin(&self) -> c_int {
        unsafe { wxPen_GetJoin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStipple<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxPen_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxPen_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxPen_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxPen>(&self, pen: &T) -> c_int {
        unsafe { wxPen_IsEqual(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxPen_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCap(&self, cap: c_int) {
        unsafe { wxPen_SetCap(self.handle(), cap) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxPen_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxPen_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDashes(&self, nb_dashes: c_int, dash: *mut c_void) {
        unsafe { wxPen_SetDashes(self.handle(), nb_dashes, dash) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setJoin(&self, join: c_int) {
        unsafe { wxPen_SetJoin(self.handle(), join) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStipple<T: _wxBitmap>(&self, stipple: &T) {
        unsafe { wxPen_SetStipple(self.handle(), stipple.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxPen_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, width: c_int) {
        unsafe { wxPen_SetWidth(self.handle(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStatic(&self) -> c_int {
        unsafe { wxPen_IsStatic(self.handle()) }
    }
}

pub struct wxPenList(*mut c_void);
impl _wxPenList for wxPenList {}
impl _wxList for wxPenList {}
impl _wxObject for wxPenList { fn handle(&self) -> *mut c_void { **self } }

impl wxPenList {
    pub fn from(handle: *mut c_void) -> @wxPenList { @wxPenList(handle) }
    pub fn null() -> @wxPenList { wxPenList::from(0 as *mut c_void) }
    
}

pub trait _wxPenList : _wxList {
}

pub struct wxPoint(*mut c_void);
impl _wxPoint for wxPoint { fn handle(&self) -> *mut c_void { **self } }

impl wxPoint {
    pub fn from(handle: *mut c_void) -> @wxPoint { @wxPoint(handle) }
    pub fn null() -> @wxPoint { wxPoint::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(xx: c_int, yy: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxPoint_Create(xx, yy)) }
    }
}

pub trait _wxPoint {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getX(&self) -> c_int {
        unsafe { wxPoint_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getY(&self) -> c_int {
        unsafe { wxPoint_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setX(&self, w: c_int) {
        unsafe { wxPoint_SetX(self.handle(), w) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setY(&self, h: c_int) {
        unsafe { wxPoint_SetY(self.handle(), h) }
    }
}

pub struct wxPopupTransientWindow(*mut c_void);
impl _wxPopupTransientWindow for wxPopupTransientWindow {}
impl _wxPopupWindow for wxPopupTransientWindow {}
impl _wxWindow for wxPopupTransientWindow {}
impl _wxEvtHandler for wxPopupTransientWindow {}
impl _wxObject for wxPopupTransientWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxPopupTransientWindow {
    pub fn from(handle: *mut c_void) -> @wxPopupTransientWindow { @wxPopupTransientWindow(handle) }
    pub fn null() -> @wxPopupTransientWindow { wxPopupTransientWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPopupTransientWindow : _wxPopupWindow {
}

pub struct wxPopupWindow(*mut c_void);
impl _wxPopupWindow for wxPopupWindow {}
impl _wxWindow for wxPopupWindow {}
impl _wxEvtHandler for wxPopupWindow {}
impl _wxObject for wxPopupWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxPopupWindow {
    pub fn from(handle: *mut c_void) -> @wxPopupWindow { @wxPopupWindow(handle) }
    pub fn null() -> @wxPopupWindow { wxPopupWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPopupWindow : _wxWindow {
}

pub struct wxPostScriptDC(*mut c_void);
impl _wxPostScriptDC for wxPostScriptDC {}
impl _wxDC for wxPostScriptDC {}
impl _wxObject for wxPostScriptDC { fn handle(&self) -> *mut c_void { **self } }

impl wxPostScriptDC {
    pub fn from(handle: *mut c_void) -> @wxPostScriptDC { @wxPostScriptDC(handle) }
    pub fn null() -> @wxPostScriptDC { wxPostScriptDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintData>(data: &T) -> @wxPostScriptDC {
        unsafe { @wxPostScriptDC(wxPostScriptDC_Create(data.handle())) }
    }
}

pub trait _wxPostScriptDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setResolution(&self, ppi: c_int) {
        unsafe { wxPostScriptDC_SetResolution(self.handle(), ppi) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getResolution(&self) -> c_int {
        unsafe { wxPostScriptDC_GetResolution(self.handle()) }
    }
}

pub struct wxPreviewCanvas(*mut c_void);
impl _wxPreviewCanvas for wxPreviewCanvas {}
impl _wxScrolledWindow for wxPreviewCanvas {}
impl _wxPanel for wxPreviewCanvas {}
impl _wxWindow for wxPreviewCanvas {}
impl _wxEvtHandler for wxPreviewCanvas {}
impl _wxObject for wxPreviewCanvas { fn handle(&self) -> *mut c_void { **self } }

impl wxPreviewCanvas {
    pub fn from(handle: *mut c_void) -> @wxPreviewCanvas { @wxPreviewCanvas(handle) }
    pub fn null() -> @wxPreviewCanvas { wxPreviewCanvas::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintPreview, U: _wxWindow>(preview: &T, parent: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPreviewCanvas_Create(preview.handle(), parent.handle(), x, y, w, h, style)) }
    }
}

pub trait _wxPreviewCanvas : _wxScrolledWindow {
}

pub struct wxPreviewControlBar(*mut c_void);
impl _wxPreviewControlBar for wxPreviewControlBar {}
impl _wxPanel for wxPreviewControlBar {}
impl _wxWindow for wxPreviewControlBar {}
impl _wxEvtHandler for wxPreviewControlBar {}
impl _wxObject for wxPreviewControlBar { fn handle(&self) -> *mut c_void { **self } }

impl wxPreviewControlBar {
    pub fn from(handle: *mut c_void) -> @wxPreviewControlBar { @wxPreviewControlBar(handle) }
    pub fn null() -> @wxPreviewControlBar { wxPreviewControlBar::from(0 as *mut c_void) }
    
}

pub trait _wxPreviewControlBar : _wxPanel {
}

pub struct wxPreviewFrame(*mut c_void);
impl _wxPreviewFrame for wxPreviewFrame {}
impl _wxFrame for wxPreviewFrame {}
impl _wxTopLevelWindow for wxPreviewFrame {}
impl _wxWindow for wxPreviewFrame {}
impl _wxEvtHandler for wxPreviewFrame {}
impl _wxObject for wxPreviewFrame { fn handle(&self) -> *mut c_void { **self } }

impl wxPreviewFrame {
    pub fn from(handle: *mut c_void) -> @wxPreviewFrame { @wxPreviewFrame(handle) }
    pub fn null() -> @wxPreviewFrame { wxPreviewFrame::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintPreview, U: _wxFrame>(preview: &T, parent: &U, title: &str, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: &str) -> @wxPreviewFrame {
        let title = wxT(title);
        let name = wxT(name);
        unsafe { @wxPreviewFrame(wxPreviewFrame_Create(preview.handle(), parent.handle(), title.handle(), x, y, width, height, style, name.handle())) }
    }
}

pub trait _wxPreviewFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.handle()) }
    }
}

pub struct wxPrintData(*mut c_void);
impl _wxPrintData for wxPrintData {}
impl _wxObject for wxPrintData { fn handle(&self) -> *mut c_void { **self } }

impl wxPrintData {
    pub fn from(handle: *mut c_void) -> @wxPrintData { @wxPrintData(handle) }
    pub fn null() -> @wxPrintData { wxPrintData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxPrintData {
        unsafe { @wxPrintData(wxPrintData_Create()) }
    }
}

pub trait _wxPrintData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPrintData>(&self, data: &T) {
        unsafe { wxPrintData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCollate(&self) -> c_int {
        unsafe { wxPrintData_GetCollate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour(&self) -> c_int {
        unsafe { wxPrintData_GetColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDuplex(&self) -> c_int {
        unsafe { wxPrintData_GetDuplex(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFilename(&self) -> ~str {
        unsafe { wxString { handle: wxPrintData_GetFilename(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFontMetricPath(&self) -> ~str {
        unsafe { wxString { handle: wxPrintData_GetFontMetricPath(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintData_GetNoCopies(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxPrintData_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPrintData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperSize(&self) -> @wxSize {
        unsafe { @wxSize(wxPrintData_GetPaperSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPreviewCommand(&self) -> ~str {
        unsafe { wxString { handle: wxPrintData_GetPreviewCommand(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterCommand(&self) -> ~str {
        unsafe { wxString { handle: wxPrintData_GetPrinterCommand(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterName(&self) -> ~str {
        unsafe { wxString { handle: wxPrintData_GetPrinterName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterOptions(&self) -> ~str {
        unsafe { wxString { handle: wxPrintData_GetPrinterOptions(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterScaleX(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterScaleY(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterTranslateX(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrinterTranslateY(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getQuality(&self) -> c_int {
        unsafe { wxPrintData_GetQuality(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintData_SetCollate(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour(&self, colour: c_int) {
        unsafe { wxPrintData_SetColour(self.handle(), colour) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDuplex(&self, duplex: c_int) {
        unsafe { wxPrintData_SetDuplex(self.handle(), duplex) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFilename(&self, filename: &str) {
        let filename = wxT(filename);
        unsafe { wxPrintData_SetFilename(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFontMetricPath(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxPrintData_SetFontMetricPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintData_SetNoCopies(self.handle(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxPrintData_SetOrientation(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperId(&self, sizeId: c_int) {
        unsafe { wxPrintData_SetPaperId(self.handle(), sizeId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPrintData_SetPaperSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPreviewCommand<T: _wxCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPreviewCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.handle(), printMode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterCommand<T: _wxCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPrinterCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterName(&self, name: &str) {
        let name = wxT(name);
        unsafe { wxPrintData_SetPrinterName(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterOptions(&self, options: &str) {
        let options = wxT(options);
        unsafe { wxPrintData_SetPrinterOptions(self.handle(), options.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterScaleX(&self, x: c_double) {
        unsafe { wxPrintData_SetPrinterScaleX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterScaleY(&self, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaleY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterScaling(&self, x: c_double, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaling(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterTranslateX(&self, x: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterTranslateY(&self, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrinterTranslation(&self, x: c_int, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslation(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setQuality(&self, quality: c_int) {
        unsafe { wxPrintData_SetQuality(self.handle(), quality) }
    }
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

pub struct wxPrintDialog(*mut c_void);
impl _wxPrintDialog for wxPrintDialog {}
impl _wxDialog for wxPrintDialog {}
impl _wxTopLevelWindow for wxPrintDialog {}
impl _wxWindow for wxPrintDialog {}
impl _wxEvtHandler for wxPrintDialog {}
impl _wxObject for wxPrintDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxPrintDialog {
    pub fn from(handle: *mut c_void) -> @wxPrintDialog { @wxPrintDialog(handle) }
    pub fn null() -> @wxPrintDialog { wxPrintDialog::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPrintDialogData>(parent: &T, data: &U) -> @wxPrintDialog {
        unsafe { @wxPrintDialog(wxPrintDialog_Create(parent.handle(), data.handle())) }
    }
}

pub trait _wxPrintDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDC(&self) -> @wxDC {
        unsafe { @wxDC(wxPrintDialog_GetPrintDC(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintData<T: _wxPrintData>(&self, _ref: &T) {
        unsafe { wxPrintDialog_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDialogData(&self) -> @wxPrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialog_GetPrintDialogData(self.handle())) }
    }
}

pub struct wxPrintDialogData(*mut c_void);
impl _wxPrintDialogData for wxPrintDialogData {}
impl _wxObject for wxPrintDialogData { fn handle(&self) -> *mut c_void { **self } }

impl wxPrintDialogData {
    pub fn from(handle: *mut c_void) -> @wxPrintDialogData { @wxPrintDialogData(handle) }
    pub fn null() -> @wxPrintDialogData { wxPrintDialogData::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxPrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialogData_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData<T: _wxPrintData>(printData: &T) -> @wxPrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialogData_CreateFromData(printData.handle())) }
    }
}

pub trait _wxPrintDialogData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPrintDialogData>(&self, data: &T) {
        unsafe { wxPrintDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignData<T: _wxPrintData>(&self, data: &T) {
        unsafe { wxPrintDialogData_AssignData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableHelp(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnableHelp(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePageNumbers(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnablePageNumbers(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePrintToFile(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnablePrintToFile(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableSelection(&self, flag: c_int) {
        unsafe { wxPrintDialogData_EnableSelection(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllPages(&self) -> c_int {
        unsafe { wxPrintDialogData_GetAllPages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCollate(&self) -> c_int {
        unsafe { wxPrintDialogData_GetCollate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableHelp(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnableHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePageNumbers(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnablePageNumbers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePrintToFile(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnablePrintToFile(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableSelection(&self) -> c_int {
        unsafe { wxPrintDialogData_GetEnableSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFromPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetFromPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMaxPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMinPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintDialogData_GetNoCopies(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintData<T: _wxPrintData>(&self, _ref: &T) {
        unsafe { wxPrintDialogData_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintToFile(&self) -> c_int {
        unsafe { wxPrintDialogData_GetPrintToFile(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> c_int {
        unsafe { wxPrintDialogData_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetToPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAllPages(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetAllPages(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetCollate(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFromPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetFromPage(self.handle(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMaxPage(self.handle(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMinPage(self.handle(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetNoCopies(self.handle(), v) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintData<T: _wxPrintData>(&self, printData: &T) {
        unsafe { wxPrintDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintToFile(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetPrintToFile(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, flag: c_int) {
        unsafe { wxPrintDialogData_SetSelection(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetToPage(self.handle(), v) }
    }
}

pub struct wxPrintPreview(*mut c_void);
impl _wxPrintPreview for wxPrintPreview {}
impl _wxObject for wxPrintPreview { fn handle(&self) -> *mut c_void { **self } }

impl wxPrintPreview {
    pub fn from(handle: *mut c_void) -> @wxPrintPreview { @wxPrintPreview(handle) }
    pub fn null() -> @wxPrintPreview { wxPrintPreview::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData<T: _wxPrintout, U: _wxPrintout, V: _wxPrintData>(printout: &T, printoutForPrinting: &U, data: &V) -> @wxPrintPreview {
        unsafe { @wxPrintPreview(wxPrintPreview_CreateFromData(printout.handle(), printoutForPrinting.handle(), data.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromDialogData<T: _wxPrintout, U: _wxPrintout, V: _wxPrintDialogData>(printout: &T, printoutForPrinting: &U, data: &V) -> @wxPrintPreview {
        unsafe { @wxPrintPreview(wxPrintPreview_CreateFromDialogData(printout.handle(), printoutForPrinting.handle(), data.handle())) }
    }
}

pub trait _wxPrintPreview : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBlankPage<T: _wxPreviewCanvas, U: _wxDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_DrawBlankPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCanvas(&self) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPrintPreview_GetCanvas(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrame(wxPrintPreview_GetFrame(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMaxPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMinPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDialogData<T: _wxPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintout(&self) -> @wxPrintout {
        unsafe { @wxPrintout(wxPrintPreview_GetPrintout(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintoutForPrinting(&self) -> @wxPrintout {
        unsafe { @wxPrintout(wxPrintPreview_GetPrintoutForPrinting(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getZoom(&self) -> c_int {
        unsafe { wxPrintPreview_GetZoom(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxPrintPreview_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paintPage<T: _wxPrintPreview, U: _wxDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_PaintPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn print(&self, interactive: c_int) -> c_int {
        unsafe { wxPrintPreview_Print(self.handle(), interactive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renderPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_RenderPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCanvas<T: _wxPreviewCanvas>(&self, canvas: &T) {
        unsafe { wxPrintPreview_SetCanvas(self.handle(), canvas.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_SetCurrentPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFrame<T: _wxFrame>(&self, frame: &T) {
        unsafe { wxPrintPreview_SetFrame(self.handle(), frame.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOk(&self, ok: c_int) {
        unsafe { wxPrintPreview_SetOk(self.handle(), ok) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrintout<T: _wxPrintout>(&self, printout: &T) {
        unsafe { wxPrintPreview_SetPrintout(self.handle(), printout.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.handle(), percent) }
    }
}

pub struct wxPrinter(*mut c_void);
impl _wxPrinter for wxPrinter {}
impl _wxObject for wxPrinter { fn handle(&self) -> *mut c_void { **self } }

impl wxPrinter {
    pub fn from(handle: *mut c_void) -> @wxPrinter { @wxPrinter(handle) }
    pub fn null() -> @wxPrinter { wxPrinter::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintDialogData>(data: &T) -> @wxPrinter {
        unsafe { @wxPrinter(wxPrinter_Create(data.handle())) }
    }
}

pub trait _wxPrinter : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newAbortWindow<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U) -> @wxWindow {
        unsafe { @wxWindow(wxPrinter_CreateAbortWindow(self.handle(), parent.handle(), printout.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAbort(&self) -> c_int {
        unsafe { wxPrinter_GetAbort(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastError(&self) -> c_int {
        unsafe { wxPrinter_GetLastError(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintDialogData<T: _wxPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrinter_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn print<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U, prompt: c_int) -> c_int {
        unsafe { wxPrinter_Print(self.handle(), parent.handle(), printout.handle(), prompt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn printDialog<T: _wxWindow>(&self, parent: &T) -> @wxDC {
        unsafe { @wxDC(wxPrinter_PrintDialog(self.handle(), parent.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reportError<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U, message: &str) {
        let message = wxT(message);
        unsafe { wxPrinter_ReportError(self.handle(), parent.handle(), printout.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setup<T: _wxWindow>(&self, parent: &T) -> c_int {
        unsafe { wxPrinter_Setup(self.handle(), parent.handle()) }
    }
}

pub struct wxPrinterDC(*mut c_void);
impl _wxPrinterDC for wxPrinterDC {}
impl _wxDC for wxPrinterDC {}
impl _wxObject for wxPrinterDC { fn handle(&self) -> *mut c_void { **self } }

impl wxPrinterDC {
    pub fn from(handle: *mut c_void) -> @wxPrinterDC { @wxPrinterDC(handle) }
    pub fn null() -> @wxPrinterDC { wxPrinterDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintData>(data: &T) -> @wxPrinterDC {
        unsafe { @wxPrinterDC(wxPrinterDC_Create(data.handle())) }
    }
}

pub trait _wxPrinterDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperRect(&self) -> @wxRect {
        unsafe { @wxRect(wxPrinterDC_GetPaperRect(self.handle())) }
    }
}

pub struct wxPrintout(*mut c_void);
impl _wxPrintout for wxPrintout {}
impl _wxObject for wxPrintout { fn handle(&self) -> *mut c_void { **self } }

impl wxPrintout {
    pub fn from(handle: *mut c_void) -> @wxPrintout { @wxPrintout(handle) }
    pub fn null() -> @wxPrintout { wxPrintout::from(0 as *mut c_void) }
    
}

pub trait _wxPrintout : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDC(&self) -> @wxDC {
        unsafe { @wxDC(wxPrintout_GetDC(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPIPrinter(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxPrintout_GetPPIPrinter(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPIScreen(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxPrintout_GetPPIScreen(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSizeMM(&self, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxPrintout_GetPageSizeMM(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSizePixels(&self, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxPrintout_GetPageSizePixels(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { handle: wxPrintout_GetTitle(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isPreview(&self) -> c_int {
        unsafe { wxPrintout_IsPreview(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDC<T: _wxDC>(&self, dc: &T) {
        unsafe { wxPrintout_SetDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPPIPrinter(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIPrinter(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPPIScreen(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIScreen(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSizeMM(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizeMM(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSizePixels(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizePixels(self.handle(), w, h) }
    }
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

pub struct wxProcess(*mut c_void);
impl _wxProcess for wxProcess {}
impl _wxEvtHandler for wxProcess {}
impl _wxObject for wxProcess { fn handle(&self) -> *mut c_void { **self } }

impl wxProcess {
    pub fn from(handle: *mut c_void) -> @wxProcess { @wxProcess(handle) }
    pub fn null() -> @wxProcess { wxProcess::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault<T: _wxWindow>(_prt: &T, _id: c_int) -> @wxProcess {
        unsafe { @wxProcess(wxProcess_CreateDefault(_prt.handle(), _id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRedirect<T: _wxWindow>(_prt: &T, _rdr: c_int) -> @wxProcess {
        unsafe { @wxProcess(wxProcess_CreateRedirect(_prt.handle(), _rdr)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn open(cmd: &str, flags: c_int) -> @wxProcess {
        let cmd = wxT(cmd);
        unsafe { @wxProcess(wxProcess_Open(cmd.handle(), flags)) }
    }
}

pub trait _wxProcess : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detach(&self) {
        unsafe { wxProcess_Detach(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getErrorStream(&self) -> @wxInputStream {
        unsafe { @wxInputStream(wxProcess_GetErrorStream(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInputStream(&self) -> @wxInputStream {
        unsafe { @wxInputStream(wxProcess_GetInputStream(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOutputStream(&self) -> @wxOutputStream {
        unsafe { @wxOutputStream(wxProcess_GetOutputStream(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRedirected(&self) -> c_int {
        unsafe { wxProcess_IsRedirected(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn redirect(&self) {
        unsafe { wxProcess_Redirect(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isErrorAvailable(&self) -> c_int {
        unsafe { wxProcess_IsErrorAvailable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInputAvailable(&self) -> c_int {
        unsafe { wxProcess_IsInputAvailable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInputOpened(&self) -> c_int {
        unsafe { wxProcess_IsInputOpened(self.handle()) }
    }
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

pub struct wxQuantize(*mut c_void);
impl _wxQuantize for wxQuantize {}
impl _wxObject for wxQuantize { fn handle(&self) -> *mut c_void { **self } }

impl wxQuantize {
    pub fn from(handle: *mut c_void) -> @wxQuantize { @wxQuantize(handle) }
    pub fn null() -> @wxQuantize { wxQuantize::from(0 as *mut c_void) }
    
}

pub trait _wxQuantize : _wxObject {
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

pub struct wxRadioButton(*mut c_void);
impl _wxRadioButton for wxRadioButton {}
impl _wxControl for wxRadioButton {}
impl _wxWindow for wxRadioButton {}
impl _wxEvtHandler for wxRadioButton {}
impl _wxObject for wxRadioButton { fn handle(&self) -> *mut c_void { **self } }

impl wxRadioButton {
    pub fn from(handle: *mut c_void) -> @wxRadioButton { @wxRadioButton(handle) }
    pub fn null() -> @wxRadioButton { wxRadioButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxRadioButton {
        let _txt = wxT(_txt);
        unsafe { @wxRadioButton(wxRadioButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxRadioButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxRadioButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxRadioButton_SetValue(self.handle(), value) }
    }
}

pub struct wxRealPoint(*mut c_void);
impl _wxRealPoint for wxRealPoint { fn handle(&self) -> *mut c_void { **self } }

impl wxRealPoint {
    pub fn from(handle: *mut c_void) -> @wxRealPoint { @wxRealPoint(handle) }
    pub fn null() -> @wxRealPoint { wxRealPoint::from(0 as *mut c_void) }
    
}

pub trait _wxRealPoint {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxRect(*mut c_void);
impl _wxRect for wxRect { fn handle(&self) -> *mut c_void { **self } }

impl wxRect {
    pub fn from(handle: *mut c_void) -> @wxRect { @wxRect(handle) }
    pub fn null() -> @wxRect { wxRect::from(0 as *mut c_void) }
    
}

pub trait _wxRect {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxRegion(*mut c_void);
impl _wxRegion for wxRegion {}
impl _wxGDIObject for wxRegion {}
impl _wxObject for wxRegion { fn handle(&self) -> *mut c_void { **self } }

impl wxRegion {
    pub fn from(handle: *mut c_void) -> @wxRegion { @wxRegion(handle) }
    pub fn null() -> @wxRegion { wxRegion::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxRegion {
        unsafe { @wxRegion(wxRegion_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> @wxRegion {
        unsafe { @wxRegion(wxRegion_CreateFromRect(x, y, w, h)) }
    }
}

pub trait _wxRegion : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxRegion>(&self, region: &T) {
        unsafe { wxRegion_Assign(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxRegion_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn containsPoint(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxRegion_ContainsPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_ContainsRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEmpty(&self) -> c_int {
        unsafe { wxRegion_IsEmpty(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBox(&self, _x: *mut c_void, _y: *mut c_void, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxRegion_GetBox(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_IntersectRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn intersectRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_IntersectRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_SubtractRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_SubtractRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_UnionRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unionRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_UnionRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_XorRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xorRegion<T: _wxRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_XorRegion(self.handle(), region.handle()) }
    }
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

pub struct wxSVGFileDC(*mut c_void);
impl _wxSVGFileDC for wxSVGFileDC {}
impl _wxDC for wxSVGFileDC {}
impl _wxObject for wxSVGFileDC { fn handle(&self) -> *mut c_void { **self } }

impl wxSVGFileDC {
    pub fn from(handle: *mut c_void) -> @wxSVGFileDC { @wxSVGFileDC(handle) }
    pub fn null() -> @wxSVGFileDC { wxSVGFileDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(fileName: &str) -> @wxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { @wxSVGFileDC(wxSVGFileDC_Create(fileName.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithSize(fileName: &str, w: c_int, h: c_int) -> @wxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { @wxSVGFileDC(wxSVGFileDC_CreateWithSize(fileName.handle(), w, h)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newWithSizeAndResolution(fileName: &str, w: c_int, h: c_int, a_dpi: c_float) -> @wxSVGFileDC {
        let fileName = wxT(fileName);
        unsafe { @wxSVGFileDC(wxSVGFileDC_CreateWithSizeAndResolution(fileName.handle(), w, h, a_dpi)) }
    }
}

pub trait _wxSVGFileDC : _wxDC {
}

pub struct wxScreenDC(*mut c_void);
impl _wxScreenDC for wxScreenDC {}
impl _wxDC for wxScreenDC {}
impl _wxObject for wxScreenDC { fn handle(&self) -> *mut c_void { **self } }

impl wxScreenDC {
    pub fn from(handle: *mut c_void) -> @wxScreenDC { @wxScreenDC(handle) }
    pub fn null() -> @wxScreenDC { wxScreenDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxScreenDC {
        unsafe { @wxScreenDC(wxScreenDC_Create()) }
    }
}

pub trait _wxScreenDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDrawingOnTop(&self) -> c_int {
        unsafe { wxScreenDC_EndDrawingOnTop(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTop(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDrawingOnTopOfWin<T: _wxWindow>(&self, win: &T) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.handle(), win.handle()) }
    }
}

pub struct wxScrollBar(*mut c_void);
impl _wxScrollBar for wxScrollBar {}
impl _wxControl for wxScrollBar {}
impl _wxWindow for wxScrollBar {}
impl _wxEvtHandler for wxScrollBar {}
impl _wxObject for wxScrollBar { fn handle(&self) -> *mut c_void { **self } }

impl wxScrollBar {
    pub fn from(handle: *mut c_void) -> @wxScrollBar { @wxScrollBar(handle) }
    pub fn null() -> @wxScrollBar { wxScrollBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxScrollBar {
        unsafe { @wxScrollBar(wxScrollBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxScrollBar : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSize(&self) -> c_int {
        unsafe { wxScrollBar_GetPageSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRange(&self) -> c_int {
        unsafe { wxScrollBar_GetRange(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getThumbPosition(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getThumbSize(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setThumbPosition(&self, viewStart: c_int) {
        unsafe { wxScrollBar_SetThumbPosition(self.handle(), viewStart) }
    }
}

pub struct wxScrollEvent(*mut c_void);
impl _wxScrollEvent for wxScrollEvent {}
impl _wxEvent for wxScrollEvent {}
impl _wxObject for wxScrollEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxScrollEvent {
    pub fn from(handle: *mut c_void) -> @wxScrollEvent { @wxScrollEvent(handle) }
    pub fn null() -> @wxScrollEvent { wxScrollEvent::from(0 as *mut c_void) }
    
}

pub trait _wxScrollEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.handle()) }
    }
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

pub struct wxScrolledWindow(*mut c_void);
impl _wxScrolledWindow for wxScrolledWindow {}
impl _wxPanel for wxScrolledWindow {}
impl _wxWindow for wxScrolledWindow {}
impl _wxEvtHandler for wxScrolledWindow {}
impl _wxObject for wxScrolledWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxScrolledWindow {
    pub fn from(handle: *mut c_void) -> @wxScrolledWindow { @wxScrolledWindow(handle) }
    pub fn null() -> @wxScrolledWindow { wxScrolledWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxScrolledWindow {
        unsafe { @wxScrolledWindow(wxScrolledWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxScrolledWindow : _wxPanel {
    #[fixed_stack_segment]
    #[inline(never)]
    fn adjustScrollbars(&self) {
        unsafe { wxScrolledWindow_AdjustScrollbars(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *mut c_void, yy: *mut c_void) {
        unsafe { wxScrolledWindow_CalcScrolledPosition(self.handle(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *mut c_void, yy: *mut c_void) {
        unsafe { wxScrolledWindow_CalcUnscrolledPosition(self.handle(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableScrolling(&self, x_scrolling: c_int, y_scrolling: c_int) {
        unsafe { wxScrolledWindow_EnableScrolling(self.handle(), x_scrolling, y_scrolling) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScaleX(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScaleY(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollPageSize(&self, orient: c_int) -> c_int {
        unsafe { wxScrolledWindow_GetScrollPageSize(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollPixelsPerUnit(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetScrollPixelsPerUnit(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTargetWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxScrolledWindow_GetTargetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getViewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetViewStart(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onDraw<T: _wxDC>(&self, dc: &T) {
        unsafe { wxScrolledWindow_OnDraw(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scroll(&self, x_pos: c_int, y_pos: c_int) {
        unsafe { wxScrolledWindow_Scroll(self.handle(), x_pos, y_pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScale(&self, xs: c_double, ys: c_double) {
        unsafe { wxScrolledWindow_SetScale(self.handle(), xs, ys) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollPageSize(&self, orient: c_int, pageSize: c_int) {
        unsafe { wxScrolledWindow_SetScrollPageSize(self.handle(), orient, pageSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollbars(&self, pixelsPerUnitX: c_int, pixelsPerUnitY: c_int, noUnitsX: c_int, noUnitsY: c_int, xPos: c_int, yPos: c_int, noRefresh: c_int) {
        unsafe { wxScrolledWindow_SetScrollbars(self.handle(), pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showScrollbars(&self, showh: c_int, showv: c_int) {
        unsafe { wxScrolledWindow_ShowScrollbars(self.handle(), showh, showv) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTargetWindow<T: _wxWindow>(&self, target: &T) {
        unsafe { wxScrolledWindow_SetTargetWindow(self.handle(), target.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn viewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_ViewStart(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.handle(), xstep, ystep) }
    }
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

pub struct wxSimpleHelpProvider(*mut c_void);
impl _wxSimpleHelpProvider for wxSimpleHelpProvider {}
impl _wxHelpProvider for wxSimpleHelpProvider { fn handle(&self) -> *mut c_void { **self } }

impl wxSimpleHelpProvider {
    pub fn from(handle: *mut c_void) -> @wxSimpleHelpProvider { @wxSimpleHelpProvider(handle) }
    pub fn null() -> @wxSimpleHelpProvider { wxSimpleHelpProvider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxSimpleHelpProvider {
        unsafe { @wxSimpleHelpProvider(wxSimpleHelpProvider_Create()) }
    }
}

pub trait _wxSimpleHelpProvider : _wxHelpProvider {
}

pub struct wxSingleChoiceDialog(*mut c_void);
impl _wxSingleChoiceDialog for wxSingleChoiceDialog {}
impl _wxDialog for wxSingleChoiceDialog {}
impl _wxTopLevelWindow for wxSingleChoiceDialog {}
impl _wxWindow for wxSingleChoiceDialog {}
impl _wxEvtHandler for wxSingleChoiceDialog {}
impl _wxObject for wxSingleChoiceDialog { fn handle(&self) -> *mut c_void { **self } }

impl wxSingleChoiceDialog {
    pub fn from(handle: *mut c_void) -> @wxSingleChoiceDialog { @wxSingleChoiceDialog(handle) }
    pub fn null() -> @wxSingleChoiceDialog { wxSingleChoiceDialog::from(0 as *mut c_void) }
    
}

pub trait _wxSingleChoiceDialog : _wxDialog {
}

pub struct wxSize(*mut c_void);
impl _wxSize for wxSize { fn handle(&self) -> *mut c_void { **self } }

impl wxSize {
    pub fn from(handle: *mut c_void) -> @wxSize { @wxSize(handle) }
    pub fn null() -> @wxSize { wxSize::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(w: c_int, h: c_int) -> @wxSize {
        unsafe { @wxSize(wxSize_Create(w, h)) }
    }
}

pub trait _wxSize {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxSize_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWidth(&self) -> c_int {
        unsafe { wxSize_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHeight(&self, h: c_int) {
        unsafe { wxSize_SetHeight(self.handle(), h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWidth(&self, w: c_int) {
        unsafe { wxSize_SetWidth(self.handle(), w) }
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

pub struct wxSizer(*mut c_void);
impl _wxSizer for wxSizer {}
impl _wxObject for wxSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxSizer {
    pub fn from(handle: *mut c_void) -> @wxSizer { @wxSizer(handle) }
    pub fn null() -> @wxSizer { wxSizer::from(0 as *mut c_void) }
    
}

pub trait _wxSizer : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Add(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSizer<T: _wxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addWindow<T: _wxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSize(wxSizer_CalcMin(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fit<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_Fit(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinSize(&self) -> @wxSize {
        unsafe { @wxSize(wxSizer_GetMinSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxSizer_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxSizer_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Insert(self.handle(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSizer<T: _wxSizer>(&self, before: c_int, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertSizer(self.handle(), before, sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertWindow<T: _wxWindow>(&self, before: c_int, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertWindow(self.handle(), before, window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Prepend(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSizer<T: _wxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependWindow<T: _wxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn recalcSizes(&self) {
        unsafe { wxSizer_RecalcSizes(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDimension(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetDimension(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemMinSize(&self, pos: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSize(self.handle(), pos, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemMinSizeSizer<T: _wxSizer>(&self, sizer: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.handle(), sizer.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemMinSizeWindow<T: _wxWindow>(&self, window: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.handle(), window.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizeHints<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_SetSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddSpacer(self.handle(), size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addStretchSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddStretchSpacer(self.handle(), size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self, delete_windows: c_int) {
        unsafe { wxSizer_Clear(self.handle(), delete_windows) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachWindow<T: _wxWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_DetachWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachSizer<T: _wxSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_DetachSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detach(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Detach(self.handle(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fitInside<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_FitInside(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getContainingWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxSizer_GetContainingWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemWindow<T: _wxWindow>(&self, window: &T, recursive: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemWindow(self.handle(), window.handle(), recursive)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemSizer<T: _wxSizer>(&self, window: &T, recursive: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemSizer(self.handle(), window.handle(), recursive)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem(&self, index: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItem(self.handle(), index)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideWindow<T: _wxWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_HideWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideSizer<T: _wxSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_HideSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Hide(self.handle(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSpacer(&self, index: c_int, size: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_InsertSpacer(self.handle(), index, size)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_InsertStretchSpacer(self.handle(), index, prop)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShownWindow(&self, window: *mut *mut c_void) -> c_int {
        unsafe { wxSizer_IsShownWindow(self.handle(), window) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShownSizer(&self, sizer: *mut *mut c_void) -> c_int {
        unsafe { wxSizer_IsShownSizer(self.handle(), sizer) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self, index: c_int) -> c_int {
        unsafe { wxSizer_IsShown(self.handle(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSpacer(&self, size: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_PrependSpacer(self.handle(), size)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependStretchSpacer(&self, prop: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_PrependStretchSpacer(self.handle(), prop)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, oldwin: &T, newwin: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceWindow(self.handle(), oldwin.handle(), newwin.handle(), recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceSizer<T: _wxSizer, U: _wxSizer>(&self, oldsz: &T, newsz: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceSizer(self.handle(), oldsz.handle(), newsz.handle(), recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxSizerItem>(&self, oldindex: c_int, newitem: &T) -> c_int {
        unsafe { wxSizer_Replace(self.handle(), oldindex, newitem.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVirtualSizeHints<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showWindow<T: _wxWindow>(&self, window: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowWindow(self.handle(), window.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showSizer<T: _wxSizer>(&self, sizer: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowSizer(self.handle(), sizer.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show<T: _wxSizer>(&self, sizer: &T, index: c_int, show: c_int) -> c_int {
        unsafe { wxSizer_Show(self.handle(), sizer.handle(), index, show) }
    }
}

pub struct wxSizerItem(*mut c_void);
impl _wxSizerItem for wxSizerItem {}
impl _wxObject for wxSizerItem { fn handle(&self) -> *mut c_void { **self } }

impl wxSizerItem {
    pub fn from(handle: *mut c_void) -> @wxSizerItem { @wxSizerItem(handle) }
    pub fn null() -> @wxSizerItem { wxSizerItem::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizerItem_Create(width, height, option, flag, border, userData)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newInSizer<T: _wxSizer>(sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInSizer(sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newInWindow<T: _wxWindow>(window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInWindow(window.handle(), option, flag, border, userData) }
    }
}

pub trait _wxSizerItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSize(wxSizerItem_CalcMin(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorder(&self) -> c_int {
        unsafe { wxSizerItem_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlag(&self) -> c_int {
        unsafe { wxSizerItem_GetFlag(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinSize(&self) -> @wxSize {
        unsafe { @wxSize(wxSizerItem_GetMinSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxSizerItem_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxSizerItem_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self) -> @wxSizer {
        unsafe { @wxSizer(wxSizerItem_GetSizer(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUserData(&self) -> *mut c_void {
        unsafe { wxSizerItem_GetUserData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxSizerItem_GetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSizer(&self) -> c_int {
        unsafe { wxSizerItem_IsSizer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSpacer(&self) -> c_int {
        unsafe { wxSizerItem_IsSpacer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWindow(&self) -> c_int {
        unsafe { wxSizerItem_IsWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorder(&self, border: c_int) {
        unsafe { wxSizerItem_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDimension(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxSizerItem_SetDimension(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlag(&self, flag: c_int) {
        unsafe { wxSizerItem_SetFlag(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFloatRatio(&self, ratio: c_float) {
        unsafe { wxSizerItem_SetFloatRatio(self.handle(), ratio) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInitSize(&self, x: c_int, y: c_int) {
        unsafe { wxSizerItem_SetInitSize(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRatio(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetRatio(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizer<T: _wxSizer>(&self, sizer: &T) {
        unsafe { wxSizerItem_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindow<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizerItem_SetWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteWindows(&self) {
        unsafe { wxSizerItem_DeleteWindows(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachSizer(&self) {
        unsafe { wxSizerItem_DetachSizer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getProportion(&self) -> c_int {
        unsafe { wxSizerItem_GetProportion(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRect(&self) -> @wxRect {
        unsafe { @wxRect(wxSizerItem_GetRect(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpacer(&self) -> @wxSize {
        unsafe { @wxSize(wxSizerItem_GetSpacer(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> c_int {
        unsafe { wxSizerItem_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setProportion(&self, proportion: c_int) {
        unsafe { wxSizerItem_SetProportion(self.handle(), proportion) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSpacer(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetSpacer(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self, show: c_int) {
        unsafe { wxSizerItem_Show(self.handle(), show) }
    }
}

pub struct wxSlider(*mut c_void);
impl _wxSlider for wxSlider {}
impl _wxControl for wxSlider {}
impl _wxWindow for wxSlider {}
impl _wxEvtHandler for wxSlider {}
impl _wxObject for wxSlider { fn handle(&self) -> *mut c_void { **self } }

impl wxSlider {
    pub fn from(handle: *mut c_void) -> @wxSlider { @wxSlider(handle) }
    pub fn null() -> @wxSlider { wxSlider::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSlider {
        unsafe { @wxSlider(wxSlider_Create(_prt.handle(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxSlider : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearSel(&self) {
        unsafe { wxSlider_ClearSel(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearTicks(&self) {
        unsafe { wxSlider_ClearTicks(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineSize(&self) -> c_int {
        unsafe { wxSlider_GetLineSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMax(&self) -> c_int {
        unsafe { wxSlider_GetMax(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMin(&self) -> c_int {
        unsafe { wxSlider_GetMin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSize(&self) -> c_int {
        unsafe { wxSlider_GetPageSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelEnd(&self) -> c_int {
        unsafe { wxSlider_GetSelEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelStart(&self) -> c_int {
        unsafe { wxSlider_GetSelStart(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getThumbLength(&self) -> c_int {
        unsafe { wxSlider_GetThumbLength(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTickFreq(&self) -> c_int {
        unsafe { wxSlider_GetTickFreq(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxSlider_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLineSize(&self, lineSize: c_int) {
        unsafe { wxSlider_SetLineSize(self.handle(), lineSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSize(&self, pageSize: c_int) {
        unsafe { wxSlider_SetPageSize(self.handle(), pageSize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, minValue: c_int, maxValue: c_int) {
        unsafe { wxSlider_SetRange(self.handle(), minValue, maxValue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, minPos: c_int, maxPos: c_int) {
        unsafe { wxSlider_SetSelection(self.handle(), minPos, maxPos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setThumbLength(&self, len: c_int) {
        unsafe { wxSlider_SetThumbLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTick(&self, tickPos: c_int) {
        unsafe { wxSlider_SetTick(self.handle(), tickPos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTickFreq(&self, n: c_int, pos: c_int) {
        unsafe { wxSlider_SetTickFreq(self.handle(), n, pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxSlider_SetValue(self.handle(), value) }
    }
}

pub struct wxSpinButton(*mut c_void);
impl _wxSpinButton for wxSpinButton {}
impl _wxControl for wxSpinButton {}
impl _wxWindow for wxSpinButton {}
impl _wxEvtHandler for wxSpinButton {}
impl _wxObject for wxSpinButton { fn handle(&self) -> *mut c_void { **self } }

impl wxSpinButton {
    pub fn from(handle: *mut c_void) -> @wxSpinButton { @wxSpinButton(handle) }
    pub fn null() -> @wxSpinButton { wxSpinButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSpinButton {
        unsafe { @wxSpinButton(wxSpinButton_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxSpinButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMax(&self) -> c_int {
        unsafe { wxSpinButton_GetMax(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMin(&self) -> c_int {
        unsafe { wxSpinButton_GetMin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxSpinButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, minVal: c_int, maxVal: c_int) {
        unsafe { wxSpinButton_SetRange(self.handle(), minVal, maxVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, val: c_int) {
        unsafe { wxSpinButton_SetValue(self.handle(), val) }
    }
}

pub struct wxSpinCtrl(*mut c_void);
impl _wxSpinCtrl for wxSpinCtrl {}
impl _wxControl for wxSpinCtrl {}
impl _wxWindow for wxSpinCtrl {}
impl _wxEvtHandler for wxSpinCtrl {}
impl _wxObject for wxSpinCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxSpinCtrl {
    pub fn from(handle: *mut c_void) -> @wxSpinCtrl { @wxSpinCtrl(handle) }
    pub fn null() -> @wxSpinCtrl { wxSpinCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> @wxSpinCtrl {
        let _txt = wxT(_txt);
        unsafe { @wxSpinCtrl(wxSpinCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) }
    }
}

pub trait _wxSpinCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMax(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMax(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMin(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMin(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxSpinCtrl_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRange(&self, min_val: c_int, max_val: c_int) {
        unsafe { wxSpinCtrl_SetRange(self.handle(), min_val, max_val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, val: c_int) {
        unsafe { wxSpinCtrl_SetValue(self.handle(), val) }
    }
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

pub struct wxStaticBox(*mut c_void);
impl _wxStaticBox for wxStaticBox {}
impl _wxControl for wxStaticBox {}
impl _wxWindow for wxStaticBox {}
impl _wxEvtHandler for wxStaticBox {}
impl _wxObject for wxStaticBox { fn handle(&self) -> *mut c_void { **self } }

impl wxStaticBox {
    pub fn from(handle: *mut c_void) -> @wxStaticBox { @wxStaticBox(handle) }
    pub fn null() -> @wxStaticBox { wxStaticBox::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticBox {
        let _txt = wxT(_txt);
        unsafe { @wxStaticBox(wxStaticBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxStaticBox : _wxControl {
}

pub struct wxStaticBoxSizer(*mut c_void);
impl _wxStaticBoxSizer for wxStaticBoxSizer {}
impl _wxBoxSizer for wxStaticBoxSizer {}
impl _wxSizer for wxStaticBoxSizer {}
impl _wxObject for wxStaticBoxSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxStaticBoxSizer {
    pub fn from(handle: *mut c_void) -> @wxStaticBoxSizer { @wxStaticBoxSizer(handle) }
    pub fn null() -> @wxStaticBoxSizer { wxStaticBoxSizer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxStaticBox>(box: &T, orient: c_int) -> @wxStaticBoxSizer {
        unsafe { @wxStaticBoxSizer(wxStaticBoxSizer_Create(box.handle(), orient)) }
    }
}

pub trait _wxStaticBoxSizer : _wxBoxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBox(&self) -> @wxStaticBox {
        unsafe { @wxStaticBox(wxStaticBoxSizer_GetStaticBox(self.handle())) }
    }
}

pub struct wxStaticLine(*mut c_void);
impl _wxStaticLine for wxStaticLine {}
impl _wxControl for wxStaticLine {}
impl _wxWindow for wxStaticLine {}
impl _wxEvtHandler for wxStaticLine {}
impl _wxObject for wxStaticLine { fn handle(&self) -> *mut c_void { **self } }

impl wxStaticLine {
    pub fn from(handle: *mut c_void) -> @wxStaticLine { @wxStaticLine(handle) }
    pub fn null() -> @wxStaticLine { wxStaticLine::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticLine {
        unsafe { @wxStaticLine(wxStaticLine_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxStaticLine : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVertical(&self) -> c_int {
        unsafe { wxStaticLine_IsVertical(self.handle()) }
    }
}

pub struct wxStaticText(*mut c_void);
impl _wxStaticText for wxStaticText {}
impl _wxControl for wxStaticText {}
impl _wxWindow for wxStaticText {}
impl _wxEvtHandler for wxStaticText {}
impl _wxObject for wxStaticText { fn handle(&self) -> *mut c_void { **self } }

impl wxStaticText {
    pub fn from(handle: *mut c_void) -> @wxStaticText { @wxStaticText(handle) }
    pub fn null() -> @wxStaticText { wxStaticText::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticText {
        let _txt = wxT(_txt);
        unsafe { @wxStaticText(wxStaticText_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxStaticText : _wxControl {
}

pub struct wxStatusBar(*mut c_void);
impl _wxStatusBar for wxStatusBar {}
impl _wxWindow for wxStatusBar {}
impl _wxEvtHandler for wxStatusBar {}
impl _wxObject for wxStatusBar { fn handle(&self) -> *mut c_void { **self } }

impl wxStatusBar {
    pub fn from(handle: *mut c_void) -> @wxStatusBar { @wxStatusBar(handle) }
    pub fn null() -> @wxStatusBar { wxStatusBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStatusBar {
        unsafe { @wxStatusBar(wxStatusBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxStatusBar : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderX(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderY(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFieldsCount(&self) -> c_int {
        unsafe { wxStatusBar_GetFieldsCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStatusText(&self, number: c_int) -> ~str {
        unsafe { wxString { handle: wxStatusBar_GetStatusText(self.handle(), number) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFieldsCount(&self, number: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetFieldsCount(self.handle(), number, widths) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinHeight(&self, height: c_int) {
        unsafe { wxStatusBar_SetMinHeight(self.handle(), height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusText(&self, text: &str, number: c_int) {
        let text = wxT(text);
        unsafe { wxStatusBar_SetStatusText(self.handle(), text.handle(), number) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStatusWidths(&self, n: c_int, widths: *mut c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.handle(), n, widths) }
    }
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

pub struct wxSystemSettings(*mut c_void);
impl _wxSystemSettings for wxSystemSettings {}
impl _wxObject for wxSystemSettings { fn handle(&self) -> *mut c_void { **self } }

impl wxSystemSettings {
    pub fn from(handle: *mut c_void) -> @wxSystemSettings { @wxSystemSettings(handle) }
    pub fn null() -> @wxSystemSettings { wxSystemSettings::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getColour<T: _wxColour>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetColour(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getFont<T: _wxFont>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetFont(index, _ref.handle()) }
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

pub struct wxTextAttr(*mut c_void);
impl _wxTextAttr for wxTextAttr { fn handle(&self) -> *mut c_void { **self } }

impl wxTextAttr {
    pub fn from(handle: *mut c_void) -> @wxTextAttr { @wxTextAttr(handle) }
    pub fn null() -> @wxTextAttr { wxTextAttr::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxColour, U: _wxColour, V: _wxFont>(colText: &T, colBack: &U, font: &V) -> @wxTextAttr {
        unsafe { @wxTextAttr(wxTextAttr_Create(colText.handle(), colBack.handle(), font.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxTextAttr {
        unsafe { @wxTextAttr(wxTextAttr_CreateDefault()) }
    }
}

pub trait _wxTextAttr {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_GetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxTextAttr_GetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_GetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxTextAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFont(&self) -> c_int {
        unsafe { wxTextAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasTextColour(&self) -> c_int {
        unsafe { wxTextAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isDefault(&self) -> c_int {
        unsafe { wxTextAttr_IsDefault(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxTextAttr_SetFont(self.handle(), font.handle()) }
    }
}

pub struct wxTextCtrl(*mut c_void);
impl _wxTextCtrl for wxTextCtrl {}
impl _wxControl for wxTextCtrl {}
impl _wxWindow for wxTextCtrl {}
impl _wxEvtHandler for wxTextCtrl {}
impl _wxObject for wxTextCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxTextCtrl {
    pub fn from(handle: *mut c_void) -> @wxTextCtrl { @wxTextCtrl(handle) }
    pub fn null() -> @wxTextCtrl { wxTextCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxTextCtrl {
        let _txt = wxT(_txt);
        unsafe { @wxTextCtrl(wxTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxTextCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_AppendText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canCopy(&self) -> c_int {
        unsafe { wxTextCtrl_CanCopy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canCut(&self) -> c_int {
        unsafe { wxTextCtrl_CanCut(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canPaste(&self) -> c_int {
        unsafe { wxTextCtrl_CanPaste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canRedo(&self) -> c_int {
        unsafe { wxTextCtrl_CanRedo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canUndo(&self) -> c_int {
        unsafe { wxTextCtrl_CanUndo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn changeValue(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_ChangeValue(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clear(&self) {
        unsafe { wxTextCtrl_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self) {
        unsafe { wxTextCtrl_Copy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cut(&self) {
        unsafe { wxTextCtrl_Cut(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn discardEdits(&self) {
        unsafe { wxTextCtrl_DiscardEdits(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInsertionPoint(&self) -> c_long {
        unsafe { wxTextCtrl_GetInsertionPoint(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastPosition(&self) -> c_long {
        unsafe { wxTextCtrl_GetLastPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineLength(&self, lineNo: c_long) -> c_int {
        unsafe { wxTextCtrl_GetLineLength(self.handle(), lineNo) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLineText(&self, lineNo: c_long) -> ~str {
        unsafe { wxString { handle: wxTextCtrl_GetLineText(self.handle(), lineNo) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfLines(&self) -> c_int {
        unsafe { wxTextCtrl_GetNumberOfLines(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { wxTextCtrl_GetSelection(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> ~str {
        unsafe { wxString { handle: wxTextCtrl_GetValue(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEditable(&self) -> c_int {
        unsafe { wxTextCtrl_IsEditable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isModified(&self) -> c_int {
        unsafe { wxTextCtrl_IsModified(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxTextCtrl_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paste(&self) {
        unsafe { wxTextCtrl_Paste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn positionToXY(&self, pos: c_long, x: *mut c_long, y: *mut c_long) -> c_int {
        unsafe { wxTextCtrl_PositionToXY(self.handle(), pos, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn redo(&self) {
        unsafe { wxTextCtrl_Redo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_Remove(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace(&self, from: c_long, to: c_long, value: &str) {
        let value = wxT(value);
        unsafe { wxTextCtrl_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxTextCtrl_SaveFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEditable(&self, editable: c_int) {
        unsafe { wxTextCtrl_SetEditable(self.handle(), editable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPoint(&self, pos: c_long) {
        unsafe { wxTextCtrl_SetInsertionPoint(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setInsertionPointEnd(&self) {
        unsafe { wxTextCtrl_SetInsertionPointEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_SetSelection(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: &str) {
        let value = wxT(value);
        unsafe { wxTextCtrl_SetValue(self.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showPosition(&self, pos: c_long) {
        unsafe { wxTextCtrl_ShowPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn undo(&self) {
        unsafe { wxTextCtrl_Undo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_WriteText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn emulateKeyPress<T: _wxKeyEvent>(&self, keyevent: &T) -> c_int {
        unsafe { wxTextCtrl_EmulateKeyPress(self.handle(), keyevent.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultStyle(&self) -> @wxTextAttr {
        unsafe { @wxTextAttr(wxTextCtrl_GetDefaultStyle(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRange(&self, from: c_long, to: c_long) -> ~str {
        unsafe { wxString { handle: wxTextCtrl_GetRange(self.handle(), from, to) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStringSelection(&self) -> ~str {
        unsafe { wxString { handle: wxTextCtrl_GetStringSelection(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMultiLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsMultiLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSingleLine(&self) -> c_int {
        unsafe { wxTextCtrl_IsSingleLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultStyle<T: _wxTextAttr>(&self, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetDefaultStyle(self.handle(), style.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle<T: _wxTextAttr>(&self, start: c_long, end: c_long, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetStyle(self.handle(), start, end, style.handle()) }
    }
}

pub struct wxTextDataObject(*mut c_void);
impl _wxTextDataObject for wxTextDataObject {}
impl _wxDataObjectSimple for wxTextDataObject {}
impl _wxDataObject for wxTextDataObject { fn handle(&self) -> *mut c_void { **self } }

impl wxTextDataObject {
    pub fn from(handle: *mut c_void) -> @wxTextDataObject { @wxTextDataObject(handle) }
    pub fn null() -> @wxTextDataObject { wxTextDataObject::from(0 as *mut c_void) }
    
}

pub trait _wxTextDataObject : _wxDataObjectSimple {
}

pub struct wxTextDropTarget(*mut c_void);
impl _wxTextDropTarget for wxTextDropTarget {}
impl _wxDropTarget for wxTextDropTarget { fn handle(&self) -> *mut c_void { **self } }

impl wxTextDropTarget {
    pub fn from(handle: *mut c_void) -> @wxTextDropTarget { @wxTextDropTarget(handle) }
    pub fn null() -> @wxTextDropTarget { wxTextDropTarget::from(0 as *mut c_void) }
    
}

pub trait _wxTextDropTarget : _wxDropTarget {
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

pub struct wxTextValidator(*mut c_void);
impl _wxTextValidator for wxTextValidator {}
impl _wxValidator for wxTextValidator {}
impl _wxEvtHandler for wxTextValidator {}
impl _wxObject for wxTextValidator { fn handle(&self) -> *mut c_void { **self } }

impl wxTextValidator {
    pub fn from(handle: *mut c_void) -> @wxTextValidator { @wxTextValidator(handle) }
    pub fn null() -> @wxTextValidator { wxTextValidator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(style: c_int, val: *mut c_void) -> @wxTextValidator {
        unsafe { @wxTextValidator(wxTextValidator_Create(style, val)) }
    }
}

pub trait _wxTextValidator : _wxValidator {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExcludes(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxTextValidator_GetExcludes(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIncludes(&self, _ref: *mut c_void) -> c_int {
        unsafe { wxTextValidator_GetIncludes(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExcludes(&self, list: *mut c_void, count: c_int) {
        unsafe { wxTextValidator_SetExcludes(self.handle(), list, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIncludes(&self, list: *mut c_void, count: c_int) {
        unsafe { wxTextValidator_SetIncludes(self.handle(), list, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clone(&self) -> @wxValidator {
        unsafe { @wxValidator(wxTextValidator_Clone(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxTextValidator_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onChar<T: _wxEvent>(&self, event: &T) {
        unsafe { wxTextValidator_OnChar(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.handle(), style) }
    }
}

pub struct wxTimer(*mut c_void);
impl _wxTimer for wxTimer {}
impl _wxObject for wxTimer { fn handle(&self) -> *mut c_void { **self } }

impl wxTimer {
    pub fn from(handle: *mut c_void) -> @wxTimer { @wxTimer(handle) }
    pub fn null() -> @wxTimer { wxTimer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int) -> @wxTimer {
        unsafe { @wxTimer(wxTimer_Create(_prt.handle(), _id)) }
    }
}

pub trait _wxTimer : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimer_GetInterval(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOneShot(&self) -> c_int {
        unsafe { wxTimer_IsOneShot(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRuning(&self) -> c_int {
        unsafe { wxTimer_IsRuning(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self, _int: c_int, _one: c_int) -> c_int {
        unsafe { wxTimer_Start(self.handle(), _int, _one) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) {
        unsafe { wxTimer_Stop(self.handle()) }
    }
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

pub struct wxToggleButton(*mut c_void);
impl _wxToggleButton for wxToggleButton {}
impl _wxControl for wxToggleButton {}
impl _wxWindow for wxToggleButton {}
impl _wxEvtHandler for wxToggleButton {}
impl _wxObject for wxToggleButton { fn handle(&self) -> *mut c_void { **self } }

impl wxToggleButton {
    pub fn from(handle: *mut c_void) -> @wxToggleButton { @wxToggleButton(handle) }
    pub fn null() -> @wxToggleButton { wxToggleButton::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, id: c_int, label: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxToggleButton {
        let label = wxT(label);
        unsafe { @wxToggleButton(wxToggleButton_Create(parent.handle(), id, label.handle(), x, y, w, h, style)) }
    }
}

pub trait _wxToggleButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> c_int {
        unsafe { wxToggleButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, state: c_int) {
        unsafe { wxToggleButton_SetValue(self.handle(), state) }
    }
}

pub struct wxToolBar(*mut c_void);
impl _wxToolBar for wxToolBar {}
impl _wxToolBarBase for wxToolBar {}
impl _wxControl for wxToolBar {}
impl _wxWindow for wxToolBar {}
impl _wxEvtHandler for wxToolBar {}
impl _wxObject for wxToolBar { fn handle(&self) -> *mut c_void { **self } }

impl wxToolBar {
    pub fn from(handle: *mut c_void) -> @wxToolBar { @wxToolBar(handle) }
    pub fn null() -> @wxToolBar { wxToolBar::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxToolBar {
        unsafe { @wxToolBar(wxToolBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxToolBar : _wxToolBarBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addControl<T: _wxControl>(&self, ctrl: &T) -> c_int {
        unsafe { wxToolBar_AddControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTool<T: _wxBitmap>(&self, id: c_int, bmp: &T, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddTool(self.handle(), id, bmp.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addToolEx<T: _wxBitmap, U: _wxBitmap, V: _wxObject>(&self, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, x: c_int, y: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddToolEx(self.handle(), id, bmp1.handle(), bmp2.handle(), isToggle, x, y, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteTool(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_DeleteTool(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteToolByPos(&self, pos: c_int) -> c_int {
        unsafe { wxToolBar_DeleteToolByPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableTool(&self, id: c_int, enable: c_int) {
        unsafe { wxToolBar_EnableTool(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMargins(&self) -> @wxPoint {
        unsafe { @wxPoint(wxToolBar_GetMargins(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolBitmapSize(&self) -> @wxSize {
        unsafe { @wxSize(wxToolBar_GetToolBitmapSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolClientData(&self, id: c_int) -> @wxObject {
        unsafe { @wxObject(wxToolBar_GetToolClientData(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolEnabled(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolLongHelp(&self, id: c_int) -> ~str {
        unsafe { wxString { handle: wxToolBar_GetToolLongHelp(self.handle(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolShortHelp(&self, id: c_int) -> ~str {
        unsafe { wxString { handle: wxToolBar_GetToolShortHelp(self.handle(), id) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolSize(&self) -> @wxSize {
        unsafe { @wxSize(wxToolBar_GetToolSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolState(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolState(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertControl<T: _wxControl>(&self, pos: c_int, ctrl: &T) {
        unsafe { wxToolBar_InsertControl(self.handle(), pos, ctrl.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertTool<T: _wxBitmap, U: _wxBitmap, V: _wxObject>(&self, pos: c_int, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_InsertTool(self.handle(), pos, id, bmp1.handle(), bmp2.handle(), isToggle, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn realize(&self) -> c_int {
        unsafe { wxToolBar_Realize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeTool(&self, id: c_int) {
        unsafe { wxToolBar_RemoveTool(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetMargins(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolBitmapSize(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetToolBitmapSize(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolClientData<T: _wxObject>(&self, id: c_int, data: &T) {
        unsafe { wxToolBar_SetToolClientData(self.handle(), id, data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolLongHelp(&self, id: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxToolBar_SetToolLongHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolPacking(&self, packing: c_int) {
        unsafe { wxToolBar_SetToolPacking(self.handle(), packing) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolSeparation(&self, separation: c_int) {
        unsafe { wxToolBar_SetToolSeparation(self.handle(), separation) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolShortHelp(&self, id: c_int, str: &str) {
        let str = wxT(str);
        unsafe { wxToolBar_SetToolShortHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toggleTool(&self, id: c_int, toggle: c_int) {
        unsafe { wxToolBar_ToggleTool(self.handle(), id, toggle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTool2<T: _wxBitmap, U: _wxBitmap>(&self, toolId: c_int, label: &str, bmp: &T, bmpDisabled: &U, itemKind: c_int, shortHelp: &str, longHelp: &str) {
        let label = wxT(label);
        let shortHelp = wxT(shortHelp);
        let longHelp = wxT(longHelp);
        unsafe { wxToolBar_AddTool2(self.handle(), toolId, label.handle(), bmp.handle(), bmpDisabled.handle(), itemKind, shortHelp.handle(), longHelp.handle()) }
    }
}

pub struct wxToolBarBase(*mut c_void);
impl _wxToolBarBase for wxToolBarBase {}
impl _wxControl for wxToolBarBase {}
impl _wxWindow for wxToolBarBase {}
impl _wxEvtHandler for wxToolBarBase {}
impl _wxObject for wxToolBarBase { fn handle(&self) -> *mut c_void { **self } }

impl wxToolBarBase {
    pub fn from(handle: *mut c_void) -> @wxToolBarBase { @wxToolBarBase(handle) }
    pub fn null() -> @wxToolBarBase { wxToolBarBase::from(0 as *mut c_void) }
    
}

pub trait _wxToolBarBase : _wxControl {
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

pub struct wxTopLevelWindow(*mut c_void);
impl _wxTopLevelWindow for wxTopLevelWindow {}
impl _wxWindow for wxTopLevelWindow {}
impl _wxEvtHandler for wxTopLevelWindow {}
impl _wxObject for wxTopLevelWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxTopLevelWindow {
    pub fn from(handle: *mut c_void) -> @wxTopLevelWindow { @wxTopLevelWindow(handle) }
    pub fn null() -> @wxTopLevelWindow { wxTopLevelWindow::from(0 as *mut c_void) }
    
}

pub trait _wxTopLevelWindow : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableCloseButton(&self, enable: c_int) -> c_int {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultButton(&self) -> @wxButton {
        unsafe { @wxButton(wxTopLevelWindow_GetDefaultButton(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultItem(&self) -> @wxWindow {
        unsafe { @wxWindow(wxTopLevelWindow_GetDefaultItem(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIcon(&self) -> @wxIcon {
        unsafe { @wxIcon(wxTopLevelWindow_GetIcon(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { handle: wxTopLevelWindow_GetTitle(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn iconize(&self, iconize: c_int) -> c_int {
        unsafe { wxTopLevelWindow_Iconize(self.handle(), iconize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isActive(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsActive(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIconized(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsIconized(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMaximized(&self) -> c_int {
        unsafe { wxTopLevelWindow_IsMaximized(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maximize(&self, maximize: c_int) {
        unsafe { wxTopLevelWindow_Maximize(self.handle(), maximize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn requestUserAttention(&self, flags: c_int) {
        unsafe { wxTopLevelWindow_RequestUserAttention(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultButton<T: _wxButton>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultItem<T: _wxWindow>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcon<T: _wxIcon>(&self, pIcon: &T) {
        unsafe { wxTopLevelWindow_SetIcon(self.handle(), pIcon.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcons(&self, _icons: *mut c_void) {
        unsafe { wxTopLevelWindow_SetIcons(self.handle(), _icons) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMaxSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMinSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTitle(&self, pString: &str) {
        let pString = wxT(pString);
        unsafe { wxTopLevelWindow_SetTitle(self.handle(), pString.handle()) }
    }
}

pub struct wxTreeCtrl(*mut c_void);
impl _wxTreeCtrl for wxTreeCtrl {}
impl _wxControl for wxTreeCtrl {}
impl _wxWindow for wxTreeCtrl {}
impl _wxEvtHandler for wxTreeCtrl {}
impl _wxObject for wxTreeCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeCtrl {
    pub fn from(handle: *mut c_void) -> @wxTreeCtrl { @wxTreeCtrl(handle) }
    pub fn null() -> @wxTreeCtrl { wxTreeCtrl::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_obj: *mut c_void, _cmp: *mut c_void, _prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create(_obj, _cmp, _prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new2<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create2(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

pub trait _wxTreeCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRoot<T: _wxTreeItemData, U: _wxTreeItemId>(&self, text: &str, image: c_int, selectedImage: c_int, data: &T, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AddRoot(self.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendItem<T: _wxTreeItemId, U: _wxTreeItemData, V: _wxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AppendItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn collapse<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Collapse(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn collapseAndReset<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteChildren<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_DeleteChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn editLabel<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EditLabel(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endEditLabel<T: _wxTreeItemId>(&self, item: &T, discardChanges: c_int) {
        unsafe { wxTreeCtrl_EndEditLabel(self.handle(), item.handle(), discardChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisible<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EnsureVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expand<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Expand(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBoundingRect<T: _wxTreeItemId>(&self, item: &T, textOnly: c_int) -> @wxRect {
        unsafe { @wxRect(wxTreeCtrl_GetBoundingRect(self.handle(), item.handle(), textOnly)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildrenCount<T: _wxTreeItemId>(&self, item: &T, recursively: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.handle(), item.handle(), recursively) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEditControl(&self) -> @wxTextCtrl {
        unsafe { @wxTextCtrl(wxTreeCtrl_GetEditControl(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstVisibleItem<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageList(&self) -> @wxImageList {
        unsafe { @wxImageList(wxTreeCtrl_GetImageList(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemData<T: _wxTreeItemId>(&self, item: &T) -> *mut c_void {
        unsafe { wxTreeCtrl_GetItemData(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemImage<T: _wxTreeItemId>(&self, item: &T, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.handle(), item.handle(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemText<T: _wxTreeItemId>(&self, item: &T) -> ~str {
        unsafe { wxString { handle: wxTreeCtrl_GetItemText(self.handle(), item.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetLastChild(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetNextChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextSibling<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextVisible<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrevSibling<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrevVisible<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRootItem<T: _wxTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetRootItem(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection<T: _wxTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetSelection(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelections(&self, selections: *mut c_void) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self.handle(), selections) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpacing(&self) -> c_int {
        unsafe { wxTreeCtrl_GetSpacing(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStateImageList(&self) -> @wxImageList {
        unsafe { @wxImageList(wxTreeCtrl_GetStateImageList(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest<T: _wxTreeItemId>(&self, _x: c_int, _y: c_int, flags: *mut c_int, _item: &T) {
        unsafe { wxTreeCtrl_HitTest(self.handle(), _x, _y, flags, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxTreeItemId, U: _wxTreeItemId, V: _wxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemByIndex<T: _wxTreeItemId, U: _wxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBold<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsBold(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExpanded<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsExpanded(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelected<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsSelected(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn itemHasChildren<T: _wxTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onCompareItems<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item1: &T, item2: &U) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.handle(), item1.handle(), item2.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependItem<T: _wxTreeItemId, U: _wxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_PrependItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollTo<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_ScrollTo(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectItem<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SelectItem(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.handle(), indent) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemBackgroundColour<T: _wxTreeItemId, U: _wxColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemBold<T: _wxTreeItemId>(&self, item: &T, bold: c_int) {
        unsafe { wxTreeCtrl_SetItemBold(self.handle(), item.handle(), bold) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemData<T: _wxTreeItemId>(&self, item: &T, data: *mut c_void) {
        unsafe { wxTreeCtrl_SetItemData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemDropHighlight<T: _wxTreeItemId>(&self, item: &T, highlight: c_int) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.handle(), item.handle(), highlight) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemFont<T: _wxTreeItemId, U: _wxFont>(&self, item: &T, font: &U) {
        unsafe { wxTreeCtrl_SetItemFont(self.handle(), item.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemHasChildren<T: _wxTreeItemId>(&self, item: &T, hasChildren: c_int) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.handle(), item.handle(), hasChildren) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemImage<T: _wxTreeItemId>(&self, item: &T, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.handle(), item.handle(), image, which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemText<T: _wxTreeItemId>(&self, item: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_SetItemText(self.handle(), item.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemTextColour<T: _wxTreeItemId, U: _wxColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.handle(), spacing) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStateImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetStateImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortChildren<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SortChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toggle<T: _wxTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Toggle(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem2<T: _wxWindow, U: _wxTreeItemId, V: _wxClosure, W: _wxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, closure: &V, _item: &W) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem2(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemByIndex2<T: _wxWindow, U: _wxClosure, V: _wxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, closure: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemClientClosure<T: _wxTreeItemId>(&self, item: &T) -> @wxClosure {
        unsafe { @wxClosure(wxTreeCtrl_GetItemClientClosure(self.handle(), item.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemClientClosure<T: _wxTreeItemId, U: _wxClosure>(&self, item: &T, closure: &U) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.handle(), item.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assignStateImageList<T: _wxImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.handle(), imageList.handle()) }
    }
}

pub struct wxTreeEvent(*mut c_void);
impl _wxTreeEvent for wxTreeEvent {}
impl _wxNotifyEvent for wxTreeEvent {}
impl _wxCommandEvent for wxTreeEvent {}
impl _wxEvent for wxTreeEvent {}
impl _wxObject for wxTreeEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeEvent {
    pub fn from(handle: *mut c_void) -> @wxTreeEvent { @wxTreeEvent(handle) }
    pub fn null() -> @wxTreeEvent { wxTreeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTreeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem<T: _wxTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { handle: wxTreeEvent_GetLabel(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOldItem<T: _wxTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetOldItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPoint(&self) -> @wxPoint {
        unsafe { @wxPoint(wxTreeEvent_GetPoint(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getKeyEvent(&self) -> @wxKeyEvent {
        unsafe { @wxKeyEvent(wxTreeEvent_GetKeyEvent(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEditCancelled(&self) -> c_int {
        unsafe { wxTreeEvent_IsEditCancelled(self.handle()) }
    }
}

pub struct wxTreeItemData(*mut c_void);
impl _wxTreeItemData for wxTreeItemData {}
impl _wxClientData for wxTreeItemData { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeItemData {
    pub fn from(handle: *mut c_void) -> @wxTreeItemData { @wxTreeItemData(handle) }
    pub fn null() -> @wxTreeItemData { wxTreeItemData::from(0 as *mut c_void) }
    
}

pub trait _wxTreeItemData : _wxClientData {
}

pub struct wxTreeItemId(*mut c_void);
impl _wxTreeItemId for wxTreeItemId { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeItemId {
    pub fn from(handle: *mut c_void) -> @wxTreeItemId { @wxTreeItemId(handle) }
    pub fn null() -> @wxTreeItemId { wxTreeItemId::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxTreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromValue(value: intptr_t) -> @wxTreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_CreateFromValue(value)) }
    }
}

pub trait _wxTreeItemId {
    fn handle(&self) -> *mut c_void;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> c_int {
        unsafe { wxTreeItemId_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clone(&self) -> @wxTreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_Clone(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.handle()) }
    }
}

pub struct wxUpdateUIEvent(*mut c_void);
impl _wxUpdateUIEvent for wxUpdateUIEvent {}
impl _wxEvent for wxUpdateUIEvent {}
impl _wxObject for wxUpdateUIEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxUpdateUIEvent {
    pub fn from(handle: *mut c_void) -> @wxUpdateUIEvent { @wxUpdateUIEvent(handle) }
    pub fn null() -> @wxUpdateUIEvent { wxUpdateUIEvent::from(0 as *mut c_void) }
    
}

pub trait _wxUpdateUIEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, check: c_int) {
        unsafe { wxUpdateUIEvent_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: c_int) {
        unsafe { wxUpdateUIEvent_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChecked(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnabled(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetChecked(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetEnabled(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetText(&self) -> c_int {
        unsafe { wxUpdateUIEvent_GetSetText(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getText(&self) -> ~str {
        unsafe { wxString { handle: wxUpdateUIEvent_GetText(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxUpdateUIEvent_SetText(self.handle(), text.handle()) }
    }
}

pub struct wxValidator(*mut c_void);
impl _wxValidator for wxValidator {}
impl _wxEvtHandler for wxValidator {}
impl _wxObject for wxValidator { fn handle(&self) -> *mut c_void { **self } }

impl wxValidator {
    pub fn from(handle: *mut c_void) -> @wxValidator { @wxValidator(handle) }
    pub fn null() -> @wxValidator { wxValidator::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxValidator {
        unsafe { @wxValidator(wxValidator_Create()) }
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
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxValidator_GetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxValidator_SetWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferFromWindow(&self) -> c_int {
        unsafe { wxValidator_TransferFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferToWindow(&self) -> c_int {
        unsafe { wxValidator_TransferToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn validate<T: _wxWindow>(&self, parent: &T) -> c_int {
        unsafe { wxValidator_Validate(self.handle(), parent.handle()) }
    }
}

pub struct wxView(*mut c_void);
impl _wxView for wxView {}
impl _wxEvtHandler for wxView {}
impl _wxObject for wxView { fn handle(&self) -> *mut c_void { **self } }

impl wxView {
    pub fn from(handle: *mut c_void) -> @wxView { @wxView(handle) }
    pub fn null() -> @wxView { wxView::from(0 as *mut c_void) }
    
}

pub trait _wxView : _wxEvtHandler {
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

pub struct wxWindow(*mut c_void);
impl _wxWindow for wxWindow {}
impl _wxEvtHandler for wxWindow {}
impl _wxObject for wxWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxWindow {
    pub fn from(handle: *mut c_void) -> @wxWindow { @wxWindow(handle) }
    pub fn null() -> @wxWindow { wxWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxWindow {
        unsafe { @wxWindow(wxWindow_Create(_prt.handle(), _id, _x, _y, _w, _h, _stl)) }
    }
}

pub trait _wxWindow : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addChild<T: _wxWindow>(&self, child: &T) {
        unsafe { wxWindow_AddChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addConstraintReference<T: _wxWindow>(&self, otherWin: &T) {
        unsafe { wxWindow_AddConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn captureMouse(&self) {
        unsafe { wxWindow_CaptureMouse(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn center(&self, direction: c_int) {
        unsafe { wxWindow_Center(self.handle(), direction) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centerOnParent(&self, dir: c_int) {
        unsafe { wxWindow_CenterOnParent(self.handle(), dir) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearBackground(&self) {
        unsafe { wxWindow_ClearBackground(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clientToScreen(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ClientToScreen(self.handle(), x, y)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self, _force: c_int) -> c_int {
        unsafe { wxWindow_Close(self.handle(), _force) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertDialogToPixels(&self) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertDialogToPixels(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertPixelsToDialog(&self) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertPixelsToDialog(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteRelatedConstraints(&self) {
        unsafe { wxWindow_DeleteRelatedConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroy(&self) -> c_int {
        unsafe { wxWindow_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyChildren(&self) -> c_int {
        unsafe { wxWindow_DestroyChildren(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disable(&self) -> c_int {
        unsafe { wxWindow_Disable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn doPhase(&self, phase: c_int) -> c_int {
        unsafe { wxWindow_DoPhase(self.handle(), phase) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self) -> c_int {
        unsafe { wxWindow_Enable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findFocus(&self) -> @wxWindow {
        unsafe { @wxWindow(wxWindow_FindFocus(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findWindow(&self, name: &str) -> @wxWindow {
        let name = wxT(name);
        unsafe { @wxWindow(wxWindow_FindWindow(self.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fit(&self) {
        unsafe { wxWindow_Fit(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fitInside(&self) {
        unsafe { wxWindow_FitInside(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn freeze(&self) {
        unsafe { wxWindow_Freeze(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEffectiveMinSize(&self) -> @wxSize {
        unsafe { @wxSize(wxWindow_GetEffectiveMinSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBestSize(&self) -> @wxSize {
        unsafe { @wxSize(wxWindow_GetBestSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCaret(&self) -> @wxCaret {
        unsafe { @wxCaret(wxWindow_GetCaret(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharHeight(&self) -> c_int {
        unsafe { wxWindow_GetCharHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCharWidth(&self) -> c_int {
        unsafe { wxWindow_GetCharWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxWindow_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientData(&self) -> @wxClientData {
        unsafe { @wxClientData(wxWindow_GetClientData(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientSize(&self) -> @wxSize {
        unsafe { @wxSize(wxWindow_GetClientSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getConstraints(&self) -> @wxLayoutConstraints {
        unsafe { @wxLayoutConstraints(wxWindow_GetConstraints(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getConstraintsInvolvedIn(&self) -> *mut c_void {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCursor(&self) -> @wxCursor {
        unsafe { @wxCursor(wxWindow_GetCursor(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDropTarget(&self) -> @wxDropTarget {
        unsafe { @wxDropTarget(wxWindow_GetDropTarget(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEventHandler(&self) -> @wxEvtHandler {
        unsafe { @wxEvtHandler(wxWindow_GetEventHandler(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxWindow_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getForegroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetForegroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHandle(&self) -> *mut c_void {
        unsafe { wxWindow_GetHandle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxWindow_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { handle: wxWindow_GetLabel(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelEmpty(&self) -> c_int {
        unsafe { wxWindow_GetLabelEmpty(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxHeight(&self) -> c_int {
        unsafe { wxWindow_GetMaxHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaxWidth(&self) -> c_int {
        unsafe { wxWindow_GetMaxWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinHeight(&self) -> c_int {
        unsafe { wxWindow_GetMinHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinWidth(&self) -> c_int {
        unsafe { wxWindow_GetMinWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { handle: wxWindow_GetName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getParent(&self) -> @wxWindow {
        unsafe { @wxWindow(wxWindow_GetParent(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPositionConstraint(&self, _x: *mut c_int, _y: *mut c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRect(&self) -> @wxRect {
        unsafe { @wxRect(wxWindow_GetRect(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollPos(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollPos(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollRange(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollRange(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollThumb(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollThumb(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxWindow_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self) -> @wxSizer {
        unsafe { @wxSizer(wxWindow_GetSizer(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent<T: _wxFont>(&self, string: &str, x: *mut c_int, y: *mut c_int, descent: *mut c_int, externalLeading: *mut c_int, theFont: &T) {
        let string = wxT(string);
        unsafe { wxWindow_GetTextExtent(self.handle(), string.handle(), x, y, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToolTip(&self) -> ~str {
        unsafe { wxString { handle: wxWindow_GetToolTip(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUpdateRegion(&self) -> @wxRegion {
        unsafe { @wxRegion(wxWindow_GetUpdateRegion(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValidator(&self) -> @wxValidator {
        unsafe { @wxValidator(wxWindow_GetValidator(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVirtualSize(&self) -> @wxSize {
        unsafe { @wxSize(wxWindow_GetVirtualSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindowStyleFlag(&self) -> c_int {
        unsafe { wxWindow_GetWindowStyleFlag(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFlag(&self, flag: c_int) -> c_int {
        unsafe { wxWindow_HasFlag(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) -> c_int {
        unsafe { wxWindow_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initDialog(&self) {
        unsafe { wxWindow_InitDialog(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBeingDeleted(&self) -> c_int {
        unsafe { wxWindow_IsBeingDeleted(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> c_int {
        unsafe { wxWindow_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxWindow_IsExposed(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> c_int {
        unsafe { wxWindow_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isTopLevel(&self) -> c_int {
        unsafe { wxWindow_IsTopLevel(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layout(&self) -> c_int {
        unsafe { wxWindow_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutPhase1(&self, noChanges: *mut c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase1(self.handle(), noChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutPhase2(&self, noChanges: *mut c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase2(self.handle(), noChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lower(&self) {
        unsafe { wxWindow_Lower(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeModal(&self, modal: c_int) {
        unsafe { wxWindow_MakeModal(self.handle(), modal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveConstraint(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_MoveConstraint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn popEventHandler(&self, deleteHandler: c_int) -> *mut c_void {
        unsafe { wxWindow_PopEventHandler(self.handle(), deleteHandler) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn popupMenu<T: _wxMenu>(&self, menu: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.handle(), menu.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepareDC<T: _wxDC>(&self, dc: &T) {
        unsafe { wxWindow_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pushEventHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxWindow_PushEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn raise(&self) {
        unsafe { wxWindow_Raise(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refresh(&self, eraseBackground: c_int) {
        unsafe { wxWindow_Refresh(self.handle(), eraseBackground) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refreshRect(&self, eraseBackground: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_RefreshRect(self.handle(), eraseBackground, x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn releaseMouse(&self) {
        unsafe { wxWindow_ReleaseMouse(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeChild<T: _wxWindow>(&self, child: &T) {
        unsafe { wxWindow_RemoveChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeConstraintReference<T: _wxWindow>(&self, otherWin: &T) {
        unsafe { wxWindow_RemoveConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reparent<T: _wxWindow>(&self, _par: &T) -> c_int {
        unsafe { wxWindow_Reparent(self.handle(), _par.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn screenToClient(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ScreenToClient(self.handle(), x, y)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self.handle(), dx, dy, x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAcceleratorTable<T: _wxAcceleratorTable>(&self, accel: &T) {
        unsafe { wxWindow_SetAcceleratorTable(self.handle(), accel.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAutoLayout(&self, autoLayout: c_int) {
        unsafe { wxWindow_SetAutoLayout(self.handle(), autoLayout) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaret<T: _wxCaret>(&self, caret: &T) {
        unsafe { wxWindow_SetCaret(self.handle(), caret.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientData<T: _wxClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientObject<T: _wxClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientObject(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setConstraintSizes(&self, recurse: c_int) {
        unsafe { wxWindow_SetConstraintSizes(self.handle(), recurse) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setConstraints<T: _wxLayoutConstraints>(&self, constraints: &T) {
        unsafe { wxWindow_SetConstraints(self.handle(), constraints.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCursor<T: _wxCursor>(&self, cursor: &T) -> c_int {
        unsafe { wxWindow_SetCursor(self.handle(), cursor.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDropTarget<T: _wxDropTarget>(&self, dropTarget: &T) {
        unsafe { wxWindow_SetDropTarget(self.handle(), dropTarget.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self.handle(), exStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) -> c_int {
        unsafe { wxWindow_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setForegroundColour<T: _wxColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.handle(), _id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabel(&self, _title: &str) {
        let _title = wxT(_title);
        unsafe { wxWindow_SetLabel(self.handle(), _title.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setName(&self, _name: &str) {
        let _name = wxT(_name);
        unsafe { wxWindow_SetName(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollPos(&self, orient: c_int, pos: c_int, refresh: c_int) {
        unsafe { wxWindow_SetScrollPos(self.handle(), orient, pos, refresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollbar(&self, orient: c_int, pos: c_int, thumbVisible: c_int, range: c_int, refresh: c_int) {
        unsafe { wxWindow_SetScrollbar(self.handle(), orient, pos, thumbVisible, range, refresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSize(&self, x: c_int, y: c_int, width: c_int, height: c_int, sizeFlags: c_int) {
        unsafe { wxWindow_SetSize(self.handle(), x, y, width, height, sizeFlags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizeConstraint(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_SetSizeConstraint(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizeHints(&self, minW: c_int, minH: c_int, maxW: c_int, maxH: c_int, incW: c_int, incH: c_int) {
        unsafe { wxWindow_SetSizeHints(self.handle(), minW, minH, maxW, maxH, incW, incH) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSizer<T: _wxSizer>(&self, sizer: &T) {
        unsafe { wxWindow_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolTip(&self, tip: &str) {
        let tip = wxT(tip);
        unsafe { wxWindow_SetToolTip(self.handle(), tip.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValidator<T: _wxValidator>(&self, validator: &T) {
        unsafe { wxWindow_SetValidator(self.handle(), validator.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowStyleFlag(&self, style: c_long) {
        unsafe { wxWindow_SetWindowStyleFlag(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) -> c_int {
        unsafe { wxWindow_Show(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn thaw(&self) {
        unsafe { wxWindow_Thaw(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferDataFromWindow(&self) -> c_int {
        unsafe { wxWindow_TransferDataFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferDataToWindow(&self) -> c_int {
        unsafe { wxWindow_TransferDataToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unsetConstraints(&self, c: *mut c_void) {
        unsafe { wxWindow_UnsetConstraints(self.handle(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateWindowUI(&self) {
        unsafe { wxWindow_UpdateWindowUI(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn validate(&self) -> c_int {
        unsafe { wxWindow_Validate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVirtualSize(&self, w: c_int, h: c_int) {
        unsafe { wxWindow_SetVirtualSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn warpPointer(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_WarpPointer(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertDialogToPixelsEx(&self) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertDialogToPixelsEx(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertPixelsToDialogEx(&self) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertPixelsToDialogEx(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn screenToClient2(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxWindow_ScreenToClient2(self.handle(), x, y)) }
    }
}

pub struct wxWindowCreateEvent(*mut c_void);
impl _wxWindowCreateEvent for wxWindowCreateEvent {}
impl _wxCommandEvent for wxWindowCreateEvent {}
impl _wxEvent for wxWindowCreateEvent {}
impl _wxObject for wxWindowCreateEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxWindowCreateEvent {
    pub fn from(handle: *mut c_void) -> @wxWindowCreateEvent { @wxWindowCreateEvent(handle) }
    pub fn null() -> @wxWindowCreateEvent { wxWindowCreateEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWindowCreateEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxWindowCreateEvent_GetWindow(self.handle())) }
    }
}

pub struct wxWindowDC(*mut c_void);
impl _wxWindowDC for wxWindowDC {}
impl _wxDC for wxWindowDC {}
impl _wxObject for wxWindowDC { fn handle(&self) -> *mut c_void { **self } }

impl wxWindowDC {
    pub fn from(handle: *mut c_void) -> @wxWindowDC { @wxWindowDC(handle) }
    pub fn null() -> @wxWindowDC { wxWindowDC::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> @wxWindowDC {
        unsafe { @wxWindowDC(wxWindowDC_Create(win.handle())) }
    }
}

pub trait _wxWindowDC : _wxDC {
}

pub struct wxWindowDestroyEvent(*mut c_void);
impl _wxWindowDestroyEvent for wxWindowDestroyEvent {}
impl _wxCommandEvent for wxWindowDestroyEvent {}
impl _wxEvent for wxWindowDestroyEvent {}
impl _wxObject for wxWindowDestroyEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxWindowDestroyEvent {
    pub fn from(handle: *mut c_void) -> @wxWindowDestroyEvent { @wxWindowDestroyEvent(handle) }
    pub fn null() -> @wxWindowDestroyEvent { wxWindowDestroyEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWindowDestroyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxWindowDestroyEvent_GetWindow(self.handle())) }
    }
}

pub struct wxWindowDisabler(*mut c_void);
impl _wxWindowDisabler for wxWindowDisabler { fn handle(&self) -> *mut c_void { **self } }

impl wxWindowDisabler {
    pub fn from(handle: *mut c_void) -> @wxWindowDisabler { @wxWindowDisabler(handle) }
    pub fn null() -> @wxWindowDisabler { wxWindowDisabler::from(0 as *mut c_void) }
    
}

pub trait _wxWindowDisabler {
    fn handle(&self) -> *mut c_void;
    
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

pub struct wxGenericDragImage(*mut c_void);
impl _wxGenericDragImage for wxGenericDragImage {}
impl _wxDragImage for wxGenericDragImage {}
impl _wxObject for wxGenericDragImage { fn handle(&self) -> *mut c_void { **self } }

impl wxGenericDragImage {
    pub fn from(handle: *mut c_void) -> @wxGenericDragImage { @wxGenericDragImage(handle) }
    pub fn null() -> @wxGenericDragImage { wxGenericDragImage::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxCursor>(cursor: &T) -> @wxGenericDragImage {
        unsafe { @wxGenericDragImage(wxGenericDragImage_Create(cursor.handle())) }
    }
}

pub trait _wxGenericDragImage : _wxDragImage {
    #[fixed_stack_segment]
    #[inline(never)]
    fn doDrawImage<T: _wxDC>(&self, dc: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxGenericDragImage_DoDrawImage(self.handle(), dc.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> @wxRect {
        unsafe { @wxRect(wxGenericDragImage_GetImageRect(self.handle(), x_pos, y_pos)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateBackingFromWindow<T: _wxDC, U: _wxMemoryDC>(&self, windowDC: &T, destDC: &U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.handle(), windowDC.handle(), destDC.handle(), x, y, w, h, xdest, ydest, width, height) }
    }
}

pub struct wxGraphicsObject(*mut c_void);
impl _wxGraphicsObject for wxGraphicsObject {}
impl _wxObject for wxGraphicsObject { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsObject {
    pub fn from(handle: *mut c_void) -> @wxGraphicsObject { @wxGraphicsObject(handle) }
    pub fn null() -> @wxGraphicsObject { wxGraphicsObject::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getRenderer() -> @wxGraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsObject_GetRenderer()) }
    }
}

pub trait _wxGraphicsObject : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isNull(&self) -> c_int {
        unsafe { wxGraphicsObject_IsNull(self.handle()) }
    }
}

pub struct wxGraphicsBrush(*mut c_void);
impl _wxGraphicsBrush for wxGraphicsBrush {}
impl _wxGraphicsObject for wxGraphicsBrush {}
impl _wxObject for wxGraphicsBrush { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsBrush {
    pub fn from(handle: *mut c_void) -> @wxGraphicsBrush { @wxGraphicsBrush(handle) }
    pub fn null() -> @wxGraphicsBrush { wxGraphicsBrush::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsBrush {
        unsafe { @wxGraphicsBrush(wxGraphicsBrush_Create()) }
    }
}

pub trait _wxGraphicsBrush : _wxGraphicsObject {
}

pub struct wxGraphicsContext(*mut c_void);
impl _wxGraphicsContext for wxGraphicsContext {}
impl _wxGraphicsObject for wxGraphicsContext {}
impl _wxObject for wxGraphicsContext { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsContext {
    pub fn from(handle: *mut c_void) -> @wxGraphicsContext { @wxGraphicsContext(handle) }
    pub fn null() -> @wxGraphicsContext { wxGraphicsContext::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindowDC>(dc: &T) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_Create(dc.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromWindow<T: _wxWindow>(window: &T) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromWindow(window.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromNative(context: *mut c_void) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNative(context)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromNativeWindow(window: *mut c_void) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNativeWindow(window)) }
    }
}

pub trait _wxGraphicsContext : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn clip<T: _wxRegion>(&self, region: &T) {
        unsafe { wxGraphicsContext_Clip(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBitmap<T: _wxBitmap>(&self, bmp: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.handle(), bmp.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawIcon<T: _wxIcon>(&self, icon: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.handle(), icon.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.handle(), n, x, y, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPath<T: _wxGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_DrawPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawRectangle(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawRoundedRectangle(self.handle(), x, y, w, h, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawText(&self, text: &str, x: c_double, y: c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawTextWithAngle(&self, text: &str, x: c_double, y: c_double, radius: c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.handle(), text.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn fillPath<T: _wxGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn strokePath<T: _wxGraphicsPath>(&self, path: &T) {
        unsafe { wxGraphicsContext_StrokePath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativeContext(&self) -> *mut c_void {
        unsafe { wxGraphicsContext_GetNativeContext(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent(&self, text: &str, width: *mut c_double, height: *mut c_double, descent: *mut c_double, externalLeading: *mut c_double) {
        let text = wxT(text);
        unsafe { wxGraphicsContext_GetTextExtent(self.handle(), text.handle(), width, height, descent, externalLeading) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsContext_Rotate(self.handle(), angle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsContext_Scale(self.handle(), xScale, yScale) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsContext_Translate(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTransform<T: _wxGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_SetTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn concatTransform<T: _wxGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_ConcatTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBrush<T: _wxBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGraphicsBrush<T: _wxGraphicsBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont, U: _wxColour>(&self, font: &T, colour: &U) {
        unsafe { wxGraphicsContext_SetFont(self.handle(), font.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGraphicsFont<T: _wxGraphicsFont>(&self, font: &T) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPen<T: _wxPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGraphicsPen<T: _wxGraphicsPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self.handle(), x1, y1, x2, y2) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn strokeLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.handle(), n, x, y, style) }
    }
}

pub struct wxGraphicsFont(*mut c_void);
impl _wxGraphicsFont for wxGraphicsFont {}
impl _wxGraphicsObject for wxGraphicsFont {}
impl _wxObject for wxGraphicsFont { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsFont {
    pub fn from(handle: *mut c_void) -> @wxGraphicsFont { @wxGraphicsFont(handle) }
    pub fn null() -> @wxGraphicsFont { wxGraphicsFont::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsFont {
        unsafe { @wxGraphicsFont(wxGraphicsFont_Create()) }
    }
}

pub trait _wxGraphicsFont : _wxGraphicsObject {
}

pub struct wxGraphicsMatrix(*mut c_void);
impl _wxGraphicsMatrix for wxGraphicsMatrix {}
impl _wxGraphicsObject for wxGraphicsMatrix {}
impl _wxObject for wxGraphicsMatrix { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsMatrix {
    pub fn from(handle: *mut c_void) -> @wxGraphicsMatrix { @wxGraphicsMatrix(handle) }
    pub fn null() -> @wxGraphicsMatrix { wxGraphicsMatrix::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsMatrix {
        unsafe { @wxGraphicsMatrix(wxGraphicsMatrix_Create()) }
    }
}

pub trait _wxGraphicsMatrix : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn concat<T: _wxGraphicsMatrix>(&self, t: &T) {
        unsafe { wxGraphicsMatrix_Concat(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn get(&self, a: *mut c_double, b: *mut c_double, c: *mut c_double, d: *mut c_double, tx: *mut c_double, ty: *mut c_double) {
        unsafe { wxGraphicsMatrix_Get(self.handle(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativeMatrix(&self) -> *mut c_void {
        unsafe { wxGraphicsMatrix_GetNativeMatrix(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn invert(&self) {
        unsafe { wxGraphicsMatrix_Invert(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxGraphicsMatrix>(&self, t: &T) -> c_int {
        unsafe { wxGraphicsMatrix_IsEqual(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIdentity(&self) -> c_int {
        unsafe { wxGraphicsMatrix_IsIdentity(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsMatrix_Rotate(self.handle(), angle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsMatrix_Scale(self.handle(), xScale, yScale) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, a: c_double, b: c_double, c: c_double, d: c_double, tx: c_double, ty: c_double) {
        unsafe { wxGraphicsMatrix_Set(self.handle(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsMatrix_Translate(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transformPoint(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxGraphicsMatrix_TransformPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transformDistance(&self, dx: *mut c_double, dy: *mut c_double) {
        unsafe { wxGraphicsMatrix_TransformDistance(self.handle(), dx, dy) }
    }
}

pub struct wxGraphicsPath(*mut c_void);
impl _wxGraphicsPath for wxGraphicsPath {}
impl _wxGraphicsObject for wxGraphicsPath {}
impl _wxObject for wxGraphicsPath { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsPath {
    pub fn from(handle: *mut c_void) -> @wxGraphicsPath { @wxGraphicsPath(handle) }
    pub fn null() -> @wxGraphicsPath { wxGraphicsPath::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsPath {
        unsafe { @wxGraphicsPath(wxGraphicsPath_Create()) }
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
        unsafe { wxGraphicsPath_MoveToPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: c_int) {
        unsafe { wxGraphicsPath_AddArc(self.handle(), x, y, r, startAngle, endAngle, clockwise) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addArcToPoint(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddArcToPoint(self.handle(), x1, y1, x2, y2, r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCircle(&self, x: c_double, y: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddCircle(self.handle(), x, y, r) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCurveToPoint(&self, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddCurveToPoint(self.handle(), cx1, cy1, cx2, cy2, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addLineToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddLineToPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPath<T: _wxGraphicsPath>(&self, x: c_double, y: c_double, path: &T) {
        unsafe { wxGraphicsPath_AddPath(self.handle(), x, y, path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addQuadCurveToPoint(&self, cx: c_double, cy: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddQuadCurveToPoint(self.handle(), cx, cy, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddRectangle(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsPath_AddRoundedRectangle(self.handle(), x, y, w, h, radius) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn closeSubpath(&self) {
        unsafe { wxGraphicsPath_CloseSubpath(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn contains(&self, x: c_double, y: c_double, style: c_int) {
        unsafe { wxGraphicsPath_Contains(self.handle(), x, y, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBox(&self, x: *mut c_double, y: *mut c_double, w: *mut c_double, h: *mut c_double) {
        unsafe { wxGraphicsPath_GetBox(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPoint(&self, x: *mut c_double, y: *mut c_double) {
        unsafe { wxGraphicsPath_GetCurrentPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transform<T: _wxGraphicsMatrix>(&self, matrix: &T) {
        unsafe { wxGraphicsPath_Transform(self.handle(), matrix.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativePath(&self) -> *mut c_void {
        unsafe { wxGraphicsPath_GetNativePath(self.handle()) }
    }
}

pub struct wxGraphicsPen(*mut c_void);
impl _wxGraphicsPen for wxGraphicsPen {}
impl _wxGraphicsObject for wxGraphicsPen {}
impl _wxObject for wxGraphicsPen { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsPen {
    pub fn from(handle: *mut c_void) -> @wxGraphicsPen { @wxGraphicsPen(handle) }
    pub fn null() -> @wxGraphicsPen { wxGraphicsPen::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsPen {
        unsafe { @wxGraphicsPen(wxGraphicsPen_Create()) }
    }
}

pub trait _wxGraphicsPen : _wxGraphicsObject {
}

pub struct wxGraphicsRenderer(*mut c_void);
impl _wxGraphicsRenderer for wxGraphicsRenderer {}
impl _wxGraphicsObject for wxGraphicsRenderer {}
impl _wxObject for wxGraphicsRenderer { fn handle(&self) -> *mut c_void { **self } }

impl wxGraphicsRenderer {
    pub fn from(handle: *mut c_void) -> @wxGraphicsRenderer { @wxGraphicsRenderer(handle) }
    pub fn null() -> @wxGraphicsRenderer { wxGraphicsRenderer::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContext<T: _wxWindowDC>(dc: &T) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContext(dc.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromWindow<T: _wxWindow>(window: &T) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromWindow(window.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromNativeContext(context: *mut c_void) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeContext(context)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromNativeWindow(window: *mut c_void) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) }
    }
}

pub trait _wxGraphicsRenderer : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRenderer(&self) -> @wxGraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsRenderer_GetDefaultRenderer(self.handle())) }
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

pub struct wxInputSink(*mut c_void);
impl _wxInputSink for wxInputSink {}
impl _wxThread for wxInputSink { fn handle(&self) -> *mut c_void { **self } }

impl wxInputSink {
    pub fn from(handle: *mut c_void) -> @wxInputSink { @wxInputSink(handle) }
    pub fn null() -> @wxInputSink { wxInputSink::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream, U: _wxEvtHandler>(input: &T, evtHandler: &U, bufferLen: c_int) -> @wxInputSink {
        unsafe { @wxInputSink(wxInputSink_Create(input.handle(), evtHandler.handle(), bufferLen)) }
    }
}

pub trait _wxInputSink : _wxThread {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self) {
        unsafe { wxInputSink_Start(self.handle()) }
    }
}

pub struct wxInputSinkEvent(*mut c_void);
impl _wxInputSinkEvent for wxInputSinkEvent {}
impl _wxEvent for wxInputSinkEvent {}
impl _wxObject for wxInputSinkEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxInputSinkEvent {
    pub fn from(handle: *mut c_void) -> @wxInputSinkEvent { @wxInputSinkEvent(handle) }
    pub fn null() -> @wxInputSinkEvent { wxInputSinkEvent::from(0 as *mut c_void) }
    
}

pub trait _wxInputSinkEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastError(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastError(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastRead(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastRead(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastInput(&self) -> *mut c_char {
        unsafe { wxInputSinkEvent_LastInput(self.handle()) }
    }
}

