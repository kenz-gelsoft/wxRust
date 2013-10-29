use std::libc::*;
use std::str;
use native::*;

#[link_args="-lwxc"]
extern {
    fn wxString_CreateUTF8(buffer: *u8) -> *u8;
    fn wxString_GetUtf8(wxs: *u8) -> *u8;
    fn wxCharBuffer_DataUtf8(wxcb: *u8) -> *c_char;
    fn wxCharBuffer_Delete(wxcb: *u8);
}

#[fixed_stack_segment]
#[inline(never)]
fn wxT(s: &str) -> wxString {
    unsafe {
        do s.to_c_str().with_ref |c_str| {
            wxString { handle: wxString_CreateUTF8(c_str as *u8) }
        }
    }
}

struct wxString { handle: *u8 }
impl Drop for wxString {
    fn drop(&self) {
        unsafe { wxString_Delete(self.handle); }
    }
}
impl wxString {
    fn handle(&self) -> *u8 { self.handle }
    fn to_str(&self) -> ~str {
        unsafe {
            let charBuffer = wxString_GetUtf8(self.handle);
            let utf8 = wxCharBuffer_DataUtf8(charBuffer);
            wxCharBuffer_Delete(charBuffer);
            str::raw::from_c_str(utf8)
        }
    }
}

struct ELJApp(*u8);
impl _ELJApp for ELJApp {}
impl _wxApp for ELJApp {}
impl _wxEvtHandler for ELJApp {}
impl _wxObject for ELJApp { fn handle(&self) -> *u8 { **self } }

impl ELJApp {
    pub fn from(handle: *u8) -> @ELJApp { @ELJApp(handle) }
    pub fn null() -> @ELJApp { ELJApp::from(0 as *u8) }
    
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
    pub fn enableTooltips(_enable: bool) {
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
    pub fn findWindowById<T: _wxWindow>(_id: c_int, _prt: &T) -> *u8 {
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
    pub fn getOsVersion(_maj: *u8, _min: *u8) -> c_int {
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
    pub fn getUserHome(_usr: *u8) -> ~str {
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
    pub fn initialized() -> bool {
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
    pub fn initializeC<T: _wxClosure>(closure: &T, _argc: c_int, _argv: *wchar_t) {
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

trait _ELJApp : _wxApp {
}

struct ELJArtProv(*u8);
impl _ELJArtProv for ELJArtProv {}
impl _wxArtProvider for ELJArtProv {}
impl _wxObject for ELJArtProv { fn handle(&self) -> *u8 { **self } }

impl ELJArtProv {
    pub fn from(handle: *u8) -> @ELJArtProv { @ELJArtProv(handle) }
    pub fn null() -> @ELJArtProv { ELJArtProv::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _clb: *u8) -> @ELJArtProv {
        unsafe { @ELJArtProv(ELJArtProv_Create(_obj, _clb)) }
    }
}

trait _ELJArtProv : _wxArtProvider {
    #[fixed_stack_segment]
    #[inline(never)]
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.handle()) }
    }
}

struct ELJClient(*u8);
impl _ELJClient for ELJClient {}
impl _wxClient for ELJClient {}
impl _wxClientBase for ELJClient {}
impl _wxObject for ELJClient { fn handle(&self) -> *u8 { **self } }

impl ELJClient {
    pub fn from(handle: *u8) -> @ELJClient { @ELJClient(handle) }
    pub fn null() -> @ELJClient { ELJClient::from(0 as *u8) }
    
}

trait _ELJClient : _wxClient {
}

struct ELJCommand(*u8);
impl _ELJCommand for ELJCommand {}
impl _wxCommand for ELJCommand {}
impl _wxObject for ELJCommand { fn handle(&self) -> *u8 { **self } }

impl ELJCommand {
    pub fn from(handle: *u8) -> @ELJCommand { @ELJCommand(handle) }
    pub fn null() -> @ELJCommand { ELJCommand::from(0 as *u8) }
    
}

trait _ELJCommand : _wxCommand {
}

struct ELJConnection(*u8);
impl _ELJConnection for ELJConnection {}
impl _wxConnection for ELJConnection {}
impl _wxConnectionBase for ELJConnection {}
impl _wxObject for ELJConnection { fn handle(&self) -> *u8 { **self } }

impl ELJConnection {
    pub fn from(handle: *u8) -> @ELJConnection { @ELJConnection(handle) }
    pub fn null() -> @ELJConnection { ELJConnection::from(0 as *u8) }
    
}

trait _ELJConnection : _wxConnection {
}

struct ELJDragDataObject(*u8);
impl _ELJDragDataObject for ELJDragDataObject { fn handle(&self) -> *u8 { **self } }

impl ELJDragDataObject {
    pub fn from(handle: *u8) -> @ELJDragDataObject { @ELJDragDataObject(handle) }
    pub fn null() -> @ELJDragDataObject { ELJDragDataObject::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _fmt: &str, _func1: *u8, _func2: *u8, _func3: *u8) -> @ELJDragDataObject {
        let _fmt = wxT(_fmt);
        unsafe { @ELJDragDataObject(ELJDragDataObject_Create(_obj, _fmt.handle(), _func1, _func2, _func3)) }
    }
}

trait _ELJDragDataObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.handle()) }
    }
}

struct ELJDropTarget(*u8);
impl _ELJDropTarget for ELJDropTarget {}
impl _wxDropTarget for ELJDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJDropTarget {
    pub fn from(handle: *u8) -> @ELJDropTarget { @ELJDropTarget(handle) }
    pub fn null() -> @ELJDropTarget { ELJDropTarget::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8) -> @ELJDropTarget {
        unsafe { @ELJDropTarget(ELJDropTarget_Create(_obj)) }
    }
}

trait _ELJDropTarget : _wxDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

struct ELJFileDropTarget(*u8);
impl _ELJFileDropTarget for ELJFileDropTarget {}
impl _wxFileDropTarget for ELJFileDropTarget {}
impl _wxDropTarget for ELJFileDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJFileDropTarget {
    pub fn from(handle: *u8) -> @ELJFileDropTarget { @ELJFileDropTarget(handle) }
    pub fn null() -> @ELJFileDropTarget { ELJFileDropTarget::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _func: *u8) -> @ELJFileDropTarget {
        unsafe { @ELJFileDropTarget(ELJFileDropTarget_Create(_obj, _func)) }
    }
}

trait _ELJFileDropTarget : _wxFileDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJFileDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

struct ELJGridTable(*u8);
impl _ELJGridTable for ELJGridTable {}
impl _wxGridTableBase for ELJGridTable {}
impl _wxObject for ELJGridTable { fn handle(&self) -> *u8 { **self } }

impl ELJGridTable {
    pub fn from(handle: *u8) -> @ELJGridTable { @ELJGridTable(handle) }
    pub fn null() -> @ELJGridTable { ELJGridTable::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _EifGetNumberRows: *u8, _EifGetNumberCols: *u8, _EifGetValue: *u8, _EifSetValue: *u8, _EifIsEmptyCell: *u8, _EifClear: *u8, _EifInsertRows: *u8, _EifAppendRows: *u8, _EifDeleteRows: *u8, _EifInsertCols: *u8, _EifAppendCols: *u8, _EifDeleteCols: *u8, _EifSetRowLabelValue: *u8, _EifSetColLabelValue: *u8, _EifGetRowLabelValue: *u8, _EifGetColLabelValue: *u8) -> @ELJGridTable {
        unsafe { @ELJGridTable(ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)) }
    }
}

trait _ELJGridTable : _wxGridTableBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getView(&self) -> @wxView {
        unsafe { @wxView(ELJGridTable_GetView(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *u8 {
        unsafe { ELJGridTable_SendTableMessage(self.handle(), id, val1, val2) }
    }
}

struct ELJLocale(*u8);
impl _ELJLocale for ELJLocale {}
impl _wxLocale for ELJLocale { fn handle(&self) -> *u8 { **self } }

impl ELJLocale {
    pub fn from(handle: *u8) -> @ELJLocale { @ELJLocale(handle) }
    pub fn null() -> @ELJLocale { ELJLocale::from(0 as *u8) }
    
}

trait _ELJLocale : _wxLocale {
}

struct ELJLog(*u8);
impl _ELJLog for ELJLog {}
impl _wxLog for ELJLog { fn handle(&self) -> *u8 { **self } }

impl ELJLog {
    pub fn from(handle: *u8) -> @ELJLog { @ELJLog(handle) }
    pub fn null() -> @ELJLog { ELJLog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _fnc: *u8) -> @ELJLog {
        unsafe { @ELJLog(ELJLog_Create(_obj, _fnc)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getActiveTarget() -> *u8 {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

trait _ELJLog : _wxLog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableLogging(&self, doIt: bool) -> c_int {
        unsafe { ELJLog_EnableLogging(self.handle(), doIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> bool {
        unsafe { ELJLog_IsEnabled(self.handle()) }
    }
}

struct ELJMessageParameters(*u8);
impl _ELJMessageParameters for ELJMessageParameters { fn handle(&self) -> *u8 { **self } }

impl ELJMessageParameters {
    pub fn from(handle: *u8) -> @ELJMessageParameters { @ELJMessageParameters(handle) }
    pub fn null() -> @ELJMessageParameters { ELJMessageParameters::from(0 as *u8) }
    
}

trait _ELJMessageParameters {
    fn handle(&self) -> *u8;
    
}

struct ELJPlotCurve(*u8);
impl _ELJPlotCurve for ELJPlotCurve {}
impl _wxPlotCurve for ELJPlotCurve {}
impl _wxObject for ELJPlotCurve { fn handle(&self) -> *u8 { **self } }

impl ELJPlotCurve {
    pub fn from(handle: *u8) -> @ELJPlotCurve { @ELJPlotCurve(handle) }
    pub fn null() -> @ELJPlotCurve { ELJPlotCurve::from(0 as *u8) }
    
}

trait _ELJPlotCurve : _wxPlotCurve {
}

struct ELJPreviewControlBar(*u8);
impl _ELJPreviewControlBar for ELJPreviewControlBar {}
impl _wxPreviewControlBar for ELJPreviewControlBar {}
impl _wxPanel for ELJPreviewControlBar {}
impl _wxWindow for ELJPreviewControlBar {}
impl _wxEvtHandler for ELJPreviewControlBar {}
impl _wxObject for ELJPreviewControlBar { fn handle(&self) -> *u8 { **self } }

impl ELJPreviewControlBar {
    pub fn from(handle: *u8) -> @ELJPreviewControlBar { @ELJPreviewControlBar(handle) }
    pub fn null() -> @ELJPreviewControlBar { ELJPreviewControlBar::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(preview: *u8, buttons: c_int, parent: &T, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewControlBar {
        unsafe { @ELJPreviewControlBar(ELJPreviewControlBar_Create(preview, buttons, parent.handle(), title, x, y, w, h, style)) }
    }
}

trait _ELJPreviewControlBar : _wxPreviewControlBar {
}

struct ELJPreviewFrame(*u8);
impl _ELJPreviewFrame for ELJPreviewFrame {}
impl _wxPreviewFrame for ELJPreviewFrame {}
impl _wxFrame for ELJPreviewFrame {}
impl _wxTopLevelWindow for ELJPreviewFrame {}
impl _wxWindow for ELJPreviewFrame {}
impl _wxEvtHandler for ELJPreviewFrame {}
impl _wxObject for ELJPreviewFrame { fn handle(&self) -> *u8 { **self } }

impl ELJPreviewFrame {
    pub fn from(handle: *u8) -> @ELJPreviewFrame { @ELJPreviewFrame(handle) }
    pub fn null() -> @ELJPreviewFrame { ELJPreviewFrame::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_obj: *u8, _init: *u8, _create_canvas: *u8, _create_toolbar: *u8, preview: *u8, parent: &T, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewFrame {
        unsafe { @ELJPreviewFrame(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.handle(), title, x, y, w, h, style)) }
    }
}

trait _ELJPreviewFrame : _wxPreviewFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControlBar(&self) -> *u8 {
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
    fn setControlBar(&self, obj: *u8) {
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

struct ELJServer(*u8);
impl _ELJServer for ELJServer {}
impl _wxServer for ELJServer {}
impl _wxServerBase for ELJServer {}
impl _wxObject for ELJServer { fn handle(&self) -> *u8 { **self } }

impl ELJServer {
    pub fn from(handle: *u8) -> @ELJServer { @ELJServer(handle) }
    pub fn null() -> @ELJServer { ELJServer::from(0 as *u8) }
    
}

trait _ELJServer : _wxServer {
}

struct ELJTextDropTarget(*u8);
impl _ELJTextDropTarget for ELJTextDropTarget {}
impl _wxTextDropTarget for ELJTextDropTarget {}
impl _wxDropTarget for ELJTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJTextDropTarget {
    pub fn from(handle: *u8) -> @ELJTextDropTarget { @ELJTextDropTarget(handle) }
    pub fn null() -> @ELJTextDropTarget { ELJTextDropTarget::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _func: *u8) -> @ELJTextDropTarget {
        unsafe { @ELJTextDropTarget(ELJTextDropTarget_Create(_obj, _func)) }
    }
}

trait _ELJTextDropTarget : _wxTextDropTarget {
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { ELJTextDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnData(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDragOver(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnDrop(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnEnter(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOnLeave(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

struct ELJTextValidator(*u8);
impl _ELJTextValidator for ELJTextValidator {}
impl _wxTextValidator for ELJTextValidator {}
impl _wxValidator for ELJTextValidator {}
impl _wxEvtHandler for ELJTextValidator {}
impl _wxObject for ELJTextValidator { fn handle(&self) -> *u8 { **self } }

impl ELJTextValidator {
    pub fn from(handle: *u8) -> @ELJTextValidator { @ELJTextValidator(handle) }
    pub fn null() -> @ELJTextValidator { ELJTextValidator::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _fnc: *u8, _txt: *wchar_t, _stl: c_int) -> @ELJTextValidator {
        unsafe { @ELJTextValidator(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) }
    }
}

trait _ELJTextValidator : _wxTextValidator {
}

struct cbAntiflickerPlugin(*u8);
impl _cbAntiflickerPlugin for cbAntiflickerPlugin {}
impl _cbPluginBase for cbAntiflickerPlugin {}
impl _wxEvtHandler for cbAntiflickerPlugin {}
impl _wxObject for cbAntiflickerPlugin { fn handle(&self) -> *u8 { **self } }

impl cbAntiflickerPlugin {
    pub fn from(handle: *u8) -> @cbAntiflickerPlugin { @cbAntiflickerPlugin(handle) }
    pub fn null() -> @cbAntiflickerPlugin { cbAntiflickerPlugin::from(0 as *u8) }
    
}

trait _cbAntiflickerPlugin : _cbPluginBase {
}

struct cbBarDragPlugin(*u8);
impl _cbBarDragPlugin for cbBarDragPlugin {}
impl _cbPluginBase for cbBarDragPlugin {}
impl _wxEvtHandler for cbBarDragPlugin {}
impl _wxObject for cbBarDragPlugin { fn handle(&self) -> *u8 { **self } }

impl cbBarDragPlugin {
    pub fn from(handle: *u8) -> @cbBarDragPlugin { @cbBarDragPlugin(handle) }
    pub fn null() -> @cbBarDragPlugin { cbBarDragPlugin::from(0 as *u8) }
    
}

trait _cbBarDragPlugin : _cbPluginBase {
}

struct cbBarHintsPlugin(*u8);
impl _cbBarHintsPlugin for cbBarHintsPlugin {}
impl _cbPluginBase for cbBarHintsPlugin {}
impl _wxEvtHandler for cbBarHintsPlugin {}
impl _wxObject for cbBarHintsPlugin { fn handle(&self) -> *u8 { **self } }

impl cbBarHintsPlugin {
    pub fn from(handle: *u8) -> @cbBarHintsPlugin { @cbBarHintsPlugin(handle) }
    pub fn null() -> @cbBarHintsPlugin { cbBarHintsPlugin::from(0 as *u8) }
    
}

trait _cbBarHintsPlugin : _cbPluginBase {
}

struct cbBarInfo(*u8);
impl _cbBarInfo for cbBarInfo {}
impl _wxObject for cbBarInfo { fn handle(&self) -> *u8 { **self } }

impl cbBarInfo {
    pub fn from(handle: *u8) -> @cbBarInfo { @cbBarInfo(handle) }
    pub fn null() -> @cbBarInfo { cbBarInfo::from(0 as *u8) }
    
}

trait _cbBarInfo : _wxObject {
}

struct cbBarSpy(*u8);
impl _cbBarSpy for cbBarSpy {}
impl _wxEvtHandler for cbBarSpy {}
impl _wxObject for cbBarSpy { fn handle(&self) -> *u8 { **self } }

impl cbBarSpy {
    pub fn from(handle: *u8) -> @cbBarSpy { @cbBarSpy(handle) }
    pub fn null() -> @cbBarSpy { cbBarSpy::from(0 as *u8) }
    
}

trait _cbBarSpy : _wxEvtHandler {
}

struct cbCloseBox(*u8);
impl _cbCloseBox for cbCloseBox {}
impl _cbMiniButton for cbCloseBox {}
impl _wxObject for cbCloseBox { fn handle(&self) -> *u8 { **self } }

impl cbCloseBox {
    pub fn from(handle: *u8) -> @cbCloseBox { @cbCloseBox(handle) }
    pub fn null() -> @cbCloseBox { cbCloseBox::from(0 as *u8) }
    
}

trait _cbCloseBox : _cbMiniButton {
}

struct cbCollapseBox(*u8);
impl _cbCollapseBox for cbCollapseBox {}
impl _cbMiniButton for cbCollapseBox {}
impl _wxObject for cbCollapseBox { fn handle(&self) -> *u8 { **self } }

impl cbCollapseBox {
    pub fn from(handle: *u8) -> @cbCollapseBox { @cbCollapseBox(handle) }
    pub fn null() -> @cbCollapseBox { cbCollapseBox::from(0 as *u8) }
    
}

trait _cbCollapseBox : _cbMiniButton {
}

struct cbCommonPaneProperties(*u8);
impl _cbCommonPaneProperties for cbCommonPaneProperties {}
impl _wxObject for cbCommonPaneProperties { fn handle(&self) -> *u8 { **self } }

impl cbCommonPaneProperties {
    pub fn from(handle: *u8) -> @cbCommonPaneProperties { @cbCommonPaneProperties(handle) }
    pub fn null() -> @cbCommonPaneProperties { cbCommonPaneProperties::from(0 as *u8) }
    
}

trait _cbCommonPaneProperties : _wxObject {
}

struct cbCustomizeBarEvent(*u8);
impl _cbCustomizeBarEvent for cbCustomizeBarEvent {}
impl _cbPluginEvent for cbCustomizeBarEvent {}
impl _wxEvent for cbCustomizeBarEvent {}
impl _wxObject for cbCustomizeBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeBarEvent {
    pub fn from(handle: *u8) -> @cbCustomizeBarEvent { @cbCustomizeBarEvent(handle) }
    pub fn null() -> @cbCustomizeBarEvent { cbCustomizeBarEvent::from(0 as *u8) }
    
}

trait _cbCustomizeBarEvent : _cbPluginEvent {
}

struct cbCustomizeLayoutEvent(*u8);
impl _cbCustomizeLayoutEvent for cbCustomizeLayoutEvent {}
impl _cbPluginEvent for cbCustomizeLayoutEvent {}
impl _wxEvent for cbCustomizeLayoutEvent {}
impl _wxObject for cbCustomizeLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeLayoutEvent {
    pub fn from(handle: *u8) -> @cbCustomizeLayoutEvent { @cbCustomizeLayoutEvent(handle) }
    pub fn null() -> @cbCustomizeLayoutEvent { cbCustomizeLayoutEvent::from(0 as *u8) }
    
}

trait _cbCustomizeLayoutEvent : _cbPluginEvent {
}

struct cbDimHandlerBase(*u8);
impl _cbDimHandlerBase for cbDimHandlerBase {}
impl _wxObject for cbDimHandlerBase { fn handle(&self) -> *u8 { **self } }

impl cbDimHandlerBase {
    pub fn from(handle: *u8) -> @cbDimHandlerBase { @cbDimHandlerBase(handle) }
    pub fn null() -> @cbDimHandlerBase { cbDimHandlerBase::from(0 as *u8) }
    
}

trait _cbDimHandlerBase : _wxObject {
}

struct cbDimInfo(*u8);
impl _cbDimInfo for cbDimInfo {}
impl _wxObject for cbDimInfo { fn handle(&self) -> *u8 { **self } }

impl cbDimInfo {
    pub fn from(handle: *u8) -> @cbDimInfo { @cbDimInfo(handle) }
    pub fn null() -> @cbDimInfo { cbDimInfo::from(0 as *u8) }
    
}

trait _cbDimInfo : _wxObject {
}

struct cbDockBox(*u8);
impl _cbDockBox for cbDockBox {}
impl _cbMiniButton for cbDockBox {}
impl _wxObject for cbDockBox { fn handle(&self) -> *u8 { **self } }

impl cbDockBox {
    pub fn from(handle: *u8) -> @cbDockBox { @cbDockBox(handle) }
    pub fn null() -> @cbDockBox { cbDockBox::from(0 as *u8) }
    
}

trait _cbDockBox : _cbMiniButton {
}

struct cbDockPane(*u8);
impl _cbDockPane for cbDockPane {}
impl _wxObject for cbDockPane { fn handle(&self) -> *u8 { **self } }

impl cbDockPane {
    pub fn from(handle: *u8) -> @cbDockPane { @cbDockPane(handle) }
    pub fn null() -> @cbDockPane { cbDockPane::from(0 as *u8) }
    
}

trait _cbDockPane : _wxObject {
}

struct cbDrawBarDecorEvent(*u8);
impl _cbDrawBarDecorEvent for cbDrawBarDecorEvent {}
impl _cbPluginEvent for cbDrawBarDecorEvent {}
impl _wxEvent for cbDrawBarDecorEvent {}
impl _wxObject for cbDrawBarDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarDecorEvent {
    pub fn from(handle: *u8) -> @cbDrawBarDecorEvent { @cbDrawBarDecorEvent(handle) }
    pub fn null() -> @cbDrawBarDecorEvent { cbDrawBarDecorEvent::from(0 as *u8) }
    
}

trait _cbDrawBarDecorEvent : _cbPluginEvent {
}

struct cbDrawBarHandlesEvent(*u8);
impl _cbDrawBarHandlesEvent for cbDrawBarHandlesEvent {}
impl _cbPluginEvent for cbDrawBarHandlesEvent {}
impl _wxEvent for cbDrawBarHandlesEvent {}
impl _wxObject for cbDrawBarHandlesEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarHandlesEvent {
    pub fn from(handle: *u8) -> @cbDrawBarHandlesEvent { @cbDrawBarHandlesEvent(handle) }
    pub fn null() -> @cbDrawBarHandlesEvent { cbDrawBarHandlesEvent::from(0 as *u8) }
    
}

trait _cbDrawBarHandlesEvent : _cbPluginEvent {
}

struct cbDrawHintRectEvent(*u8);
impl _cbDrawHintRectEvent for cbDrawHintRectEvent {}
impl _cbPluginEvent for cbDrawHintRectEvent {}
impl _wxEvent for cbDrawHintRectEvent {}
impl _wxObject for cbDrawHintRectEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawHintRectEvent {
    pub fn from(handle: *u8) -> @cbDrawHintRectEvent { @cbDrawHintRectEvent(handle) }
    pub fn null() -> @cbDrawHintRectEvent { cbDrawHintRectEvent::from(0 as *u8) }
    
}

trait _cbDrawHintRectEvent : _cbPluginEvent {
}

struct cbDrawPaneBkGroundEvent(*u8);
impl _cbDrawPaneBkGroundEvent for cbDrawPaneBkGroundEvent {}
impl _cbPluginEvent for cbDrawPaneBkGroundEvent {}
impl _wxEvent for cbDrawPaneBkGroundEvent {}
impl _wxObject for cbDrawPaneBkGroundEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneBkGroundEvent {
    pub fn from(handle: *u8) -> @cbDrawPaneBkGroundEvent { @cbDrawPaneBkGroundEvent(handle) }
    pub fn null() -> @cbDrawPaneBkGroundEvent { cbDrawPaneBkGroundEvent::from(0 as *u8) }
    
}

trait _cbDrawPaneBkGroundEvent : _cbPluginEvent {
}

struct cbDrawPaneDecorEvent(*u8);
impl _cbDrawPaneDecorEvent for cbDrawPaneDecorEvent {}
impl _cbPluginEvent for cbDrawPaneDecorEvent {}
impl _wxEvent for cbDrawPaneDecorEvent {}
impl _wxObject for cbDrawPaneDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneDecorEvent {
    pub fn from(handle: *u8) -> @cbDrawPaneDecorEvent { @cbDrawPaneDecorEvent(handle) }
    pub fn null() -> @cbDrawPaneDecorEvent { cbDrawPaneDecorEvent::from(0 as *u8) }
    
}

trait _cbDrawPaneDecorEvent : _cbPluginEvent {
}

struct cbDrawRowBkGroundEvent(*u8);
impl _cbDrawRowBkGroundEvent for cbDrawRowBkGroundEvent {}
impl _cbPluginEvent for cbDrawRowBkGroundEvent {}
impl _wxEvent for cbDrawRowBkGroundEvent {}
impl _wxObject for cbDrawRowBkGroundEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowBkGroundEvent {
    pub fn from(handle: *u8) -> @cbDrawRowBkGroundEvent { @cbDrawRowBkGroundEvent(handle) }
    pub fn null() -> @cbDrawRowBkGroundEvent { cbDrawRowBkGroundEvent::from(0 as *u8) }
    
}

trait _cbDrawRowBkGroundEvent : _cbPluginEvent {
}

struct cbDrawRowDecorEvent(*u8);
impl _cbDrawRowDecorEvent for cbDrawRowDecorEvent {}
impl _cbPluginEvent for cbDrawRowDecorEvent {}
impl _wxEvent for cbDrawRowDecorEvent {}
impl _wxObject for cbDrawRowDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowDecorEvent {
    pub fn from(handle: *u8) -> @cbDrawRowDecorEvent { @cbDrawRowDecorEvent(handle) }
    pub fn null() -> @cbDrawRowDecorEvent { cbDrawRowDecorEvent::from(0 as *u8) }
    
}

trait _cbDrawRowDecorEvent : _cbPluginEvent {
}

struct cbDrawRowHandlesEvent(*u8);
impl _cbDrawRowHandlesEvent for cbDrawRowHandlesEvent {}
impl _cbPluginEvent for cbDrawRowHandlesEvent {}
impl _wxEvent for cbDrawRowHandlesEvent {}
impl _wxObject for cbDrawRowHandlesEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowHandlesEvent {
    pub fn from(handle: *u8) -> @cbDrawRowHandlesEvent { @cbDrawRowHandlesEvent(handle) }
    pub fn null() -> @cbDrawRowHandlesEvent { cbDrawRowHandlesEvent::from(0 as *u8) }
    
}

trait _cbDrawRowHandlesEvent : _cbPluginEvent {
}

struct cbDynToolBarDimHandler(*u8);
impl _cbDynToolBarDimHandler for cbDynToolBarDimHandler {}
impl _cbDimHandlerBase for cbDynToolBarDimHandler {}
impl _wxObject for cbDynToolBarDimHandler { fn handle(&self) -> *u8 { **self } }

impl cbDynToolBarDimHandler {
    pub fn from(handle: *u8) -> @cbDynToolBarDimHandler { @cbDynToolBarDimHandler(handle) }
    pub fn null() -> @cbDynToolBarDimHandler { cbDynToolBarDimHandler::from(0 as *u8) }
    
}

trait _cbDynToolBarDimHandler : _cbDimHandlerBase {
}

struct cbFinishDrawInAreaEvent(*u8);
impl _cbFinishDrawInAreaEvent for cbFinishDrawInAreaEvent {}
impl _cbPluginEvent for cbFinishDrawInAreaEvent {}
impl _wxEvent for cbFinishDrawInAreaEvent {}
impl _wxObject for cbFinishDrawInAreaEvent { fn handle(&self) -> *u8 { **self } }

impl cbFinishDrawInAreaEvent {
    pub fn from(handle: *u8) -> @cbFinishDrawInAreaEvent { @cbFinishDrawInAreaEvent(handle) }
    pub fn null() -> @cbFinishDrawInAreaEvent { cbFinishDrawInAreaEvent::from(0 as *u8) }
    
}

trait _cbFinishDrawInAreaEvent : _cbPluginEvent {
}

struct cbFloatedBarWindow(*u8);
impl _cbFloatedBarWindow for cbFloatedBarWindow {}
impl _wxToolWindow for cbFloatedBarWindow {}
impl _wxFrame for cbFloatedBarWindow {}
impl _wxTopLevelWindow for cbFloatedBarWindow {}
impl _wxWindow for cbFloatedBarWindow {}
impl _wxEvtHandler for cbFloatedBarWindow {}
impl _wxObject for cbFloatedBarWindow { fn handle(&self) -> *u8 { **self } }

impl cbFloatedBarWindow {
    pub fn from(handle: *u8) -> @cbFloatedBarWindow { @cbFloatedBarWindow(handle) }
    pub fn null() -> @cbFloatedBarWindow { cbFloatedBarWindow::from(0 as *u8) }
    
}

trait _cbFloatedBarWindow : _wxToolWindow {
}

struct cbGCUpdatesMgr(*u8);
impl _cbGCUpdatesMgr for cbGCUpdatesMgr {}
impl _cbSimpleUpdatesMgr for cbGCUpdatesMgr {}
impl _cbUpdatesManagerBase for cbGCUpdatesMgr {}
impl _wxObject for cbGCUpdatesMgr { fn handle(&self) -> *u8 { **self } }

impl cbGCUpdatesMgr {
    pub fn from(handle: *u8) -> @cbGCUpdatesMgr { @cbGCUpdatesMgr(handle) }
    pub fn null() -> @cbGCUpdatesMgr { cbGCUpdatesMgr::from(0 as *u8) }
    
}

trait _cbGCUpdatesMgr : _cbSimpleUpdatesMgr {
}

struct cbHintAnimationPlugin(*u8);
impl _cbHintAnimationPlugin for cbHintAnimationPlugin {}
impl _cbPluginBase for cbHintAnimationPlugin {}
impl _wxEvtHandler for cbHintAnimationPlugin {}
impl _wxObject for cbHintAnimationPlugin { fn handle(&self) -> *u8 { **self } }

impl cbHintAnimationPlugin {
    pub fn from(handle: *u8) -> @cbHintAnimationPlugin { @cbHintAnimationPlugin(handle) }
    pub fn null() -> @cbHintAnimationPlugin { cbHintAnimationPlugin::from(0 as *u8) }
    
}

trait _cbHintAnimationPlugin : _cbPluginBase {
}

struct cbInsertBarEvent(*u8);
impl _cbInsertBarEvent for cbInsertBarEvent {}
impl _cbPluginEvent for cbInsertBarEvent {}
impl _wxEvent for cbInsertBarEvent {}
impl _wxObject for cbInsertBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbInsertBarEvent {
    pub fn from(handle: *u8) -> @cbInsertBarEvent { @cbInsertBarEvent(handle) }
    pub fn null() -> @cbInsertBarEvent { cbInsertBarEvent::from(0 as *u8) }
    
}

trait _cbInsertBarEvent : _cbPluginEvent {
}

struct cbLayoutRowEvent(*u8);
impl _cbLayoutRowEvent for cbLayoutRowEvent {}
impl _cbPluginEvent for cbLayoutRowEvent {}
impl _wxEvent for cbLayoutRowEvent {}
impl _wxObject for cbLayoutRowEvent { fn handle(&self) -> *u8 { **self } }

impl cbLayoutRowEvent {
    pub fn from(handle: *u8) -> @cbLayoutRowEvent { @cbLayoutRowEvent(handle) }
    pub fn null() -> @cbLayoutRowEvent { cbLayoutRowEvent::from(0 as *u8) }
    
}

trait _cbLayoutRowEvent : _cbPluginEvent {
}

struct cbLeftDClickEvent(*u8);
impl _cbLeftDClickEvent for cbLeftDClickEvent {}
impl _cbPluginEvent for cbLeftDClickEvent {}
impl _wxEvent for cbLeftDClickEvent {}
impl _wxObject for cbLeftDClickEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftDClickEvent {
    pub fn from(handle: *u8) -> @cbLeftDClickEvent { @cbLeftDClickEvent(handle) }
    pub fn null() -> @cbLeftDClickEvent { cbLeftDClickEvent::from(0 as *u8) }
    
}

trait _cbLeftDClickEvent : _cbPluginEvent {
}

struct cbLeftDownEvent(*u8);
impl _cbLeftDownEvent for cbLeftDownEvent {}
impl _cbPluginEvent for cbLeftDownEvent {}
impl _wxEvent for cbLeftDownEvent {}
impl _wxObject for cbLeftDownEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftDownEvent {
    pub fn from(handle: *u8) -> @cbLeftDownEvent { @cbLeftDownEvent(handle) }
    pub fn null() -> @cbLeftDownEvent { cbLeftDownEvent::from(0 as *u8) }
    
}

trait _cbLeftDownEvent : _cbPluginEvent {
}

struct cbLeftUpEvent(*u8);
impl _cbLeftUpEvent for cbLeftUpEvent {}
impl _cbPluginEvent for cbLeftUpEvent {}
impl _wxEvent for cbLeftUpEvent {}
impl _wxObject for cbLeftUpEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftUpEvent {
    pub fn from(handle: *u8) -> @cbLeftUpEvent { @cbLeftUpEvent(handle) }
    pub fn null() -> @cbLeftUpEvent { cbLeftUpEvent::from(0 as *u8) }
    
}

trait _cbLeftUpEvent : _cbPluginEvent {
}

struct cbMiniButton(*u8);
impl _cbMiniButton for cbMiniButton {}
impl _wxObject for cbMiniButton { fn handle(&self) -> *u8 { **self } }

impl cbMiniButton {
    pub fn from(handle: *u8) -> @cbMiniButton { @cbMiniButton(handle) }
    pub fn null() -> @cbMiniButton { cbMiniButton::from(0 as *u8) }
    
}

trait _cbMiniButton : _wxObject {
}

struct cbMotionEvent(*u8);
impl _cbMotionEvent for cbMotionEvent {}
impl _cbPluginEvent for cbMotionEvent {}
impl _wxEvent for cbMotionEvent {}
impl _wxObject for cbMotionEvent { fn handle(&self) -> *u8 { **self } }

impl cbMotionEvent {
    pub fn from(handle: *u8) -> @cbMotionEvent { @cbMotionEvent(handle) }
    pub fn null() -> @cbMotionEvent { cbMotionEvent::from(0 as *u8) }
    
}

trait _cbMotionEvent : _cbPluginEvent {
}

struct cbPaneDrawPlugin(*u8);
impl _cbPaneDrawPlugin for cbPaneDrawPlugin {}
impl _cbPluginBase for cbPaneDrawPlugin {}
impl _wxEvtHandler for cbPaneDrawPlugin {}
impl _wxObject for cbPaneDrawPlugin { fn handle(&self) -> *u8 { **self } }

impl cbPaneDrawPlugin {
    pub fn from(handle: *u8) -> @cbPaneDrawPlugin { @cbPaneDrawPlugin(handle) }
    pub fn null() -> @cbPaneDrawPlugin { cbPaneDrawPlugin::from(0 as *u8) }
    
}

trait _cbPaneDrawPlugin : _cbPluginBase {
}

struct cbPluginBase(*u8);
impl _cbPluginBase for cbPluginBase {}
impl _wxEvtHandler for cbPluginBase {}
impl _wxObject for cbPluginBase { fn handle(&self) -> *u8 { **self } }

impl cbPluginBase {
    pub fn from(handle: *u8) -> @cbPluginBase { @cbPluginBase(handle) }
    pub fn null() -> @cbPluginBase { cbPluginBase::from(0 as *u8) }
    
}

trait _cbPluginBase : _wxEvtHandler {
}

struct cbPluginEvent(*u8);
impl _cbPluginEvent for cbPluginEvent {}
impl _wxEvent for cbPluginEvent {}
impl _wxObject for cbPluginEvent { fn handle(&self) -> *u8 { **self } }

impl cbPluginEvent {
    pub fn from(handle: *u8) -> @cbPluginEvent { @cbPluginEvent(handle) }
    pub fn null() -> @cbPluginEvent { cbPluginEvent::from(0 as *u8) }
    
}

trait _cbPluginEvent : _wxEvent {
}

struct cbRemoveBarEvent(*u8);
impl _cbRemoveBarEvent for cbRemoveBarEvent {}
impl _cbPluginEvent for cbRemoveBarEvent {}
impl _wxEvent for cbRemoveBarEvent {}
impl _wxObject for cbRemoveBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbRemoveBarEvent {
    pub fn from(handle: *u8) -> @cbRemoveBarEvent { @cbRemoveBarEvent(handle) }
    pub fn null() -> @cbRemoveBarEvent { cbRemoveBarEvent::from(0 as *u8) }
    
}

trait _cbRemoveBarEvent : _cbPluginEvent {
}

struct cbResizeBarEvent(*u8);
impl _cbResizeBarEvent for cbResizeBarEvent {}
impl _cbPluginEvent for cbResizeBarEvent {}
impl _wxEvent for cbResizeBarEvent {}
impl _wxObject for cbResizeBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbResizeBarEvent {
    pub fn from(handle: *u8) -> @cbResizeBarEvent { @cbResizeBarEvent(handle) }
    pub fn null() -> @cbResizeBarEvent { cbResizeBarEvent::from(0 as *u8) }
    
}

trait _cbResizeBarEvent : _cbPluginEvent {
}

struct cbResizeRowEvent(*u8);
impl _cbResizeRowEvent for cbResizeRowEvent {}
impl _cbPluginEvent for cbResizeRowEvent {}
impl _wxEvent for cbResizeRowEvent {}
impl _wxObject for cbResizeRowEvent { fn handle(&self) -> *u8 { **self } }

impl cbResizeRowEvent {
    pub fn from(handle: *u8) -> @cbResizeRowEvent { @cbResizeRowEvent(handle) }
    pub fn null() -> @cbResizeRowEvent { cbResizeRowEvent::from(0 as *u8) }
    
}

trait _cbResizeRowEvent : _cbPluginEvent {
}

struct cbRightDownEvent(*u8);
impl _cbRightDownEvent for cbRightDownEvent {}
impl _cbPluginEvent for cbRightDownEvent {}
impl _wxEvent for cbRightDownEvent {}
impl _wxObject for cbRightDownEvent { fn handle(&self) -> *u8 { **self } }

impl cbRightDownEvent {
    pub fn from(handle: *u8) -> @cbRightDownEvent { @cbRightDownEvent(handle) }
    pub fn null() -> @cbRightDownEvent { cbRightDownEvent::from(0 as *u8) }
    
}

trait _cbRightDownEvent : _cbPluginEvent {
}

struct cbRightUpEvent(*u8);
impl _cbRightUpEvent for cbRightUpEvent {}
impl _cbPluginEvent for cbRightUpEvent {}
impl _wxEvent for cbRightUpEvent {}
impl _wxObject for cbRightUpEvent { fn handle(&self) -> *u8 { **self } }

impl cbRightUpEvent {
    pub fn from(handle: *u8) -> @cbRightUpEvent { @cbRightUpEvent(handle) }
    pub fn null() -> @cbRightUpEvent { cbRightUpEvent::from(0 as *u8) }
    
}

trait _cbRightUpEvent : _cbPluginEvent {
}

struct cbRowDragPlugin(*u8);
impl _cbRowDragPlugin for cbRowDragPlugin {}
impl _cbPluginBase for cbRowDragPlugin {}
impl _wxEvtHandler for cbRowDragPlugin {}
impl _wxObject for cbRowDragPlugin { fn handle(&self) -> *u8 { **self } }

impl cbRowDragPlugin {
    pub fn from(handle: *u8) -> @cbRowDragPlugin { @cbRowDragPlugin(handle) }
    pub fn null() -> @cbRowDragPlugin { cbRowDragPlugin::from(0 as *u8) }
    
}

trait _cbRowDragPlugin : _cbPluginBase {
}

struct cbRowInfo(*u8);
impl _cbRowInfo for cbRowInfo {}
impl _wxObject for cbRowInfo { fn handle(&self) -> *u8 { **self } }

impl cbRowInfo {
    pub fn from(handle: *u8) -> @cbRowInfo { @cbRowInfo(handle) }
    pub fn null() -> @cbRowInfo { cbRowInfo::from(0 as *u8) }
    
}

trait _cbRowInfo : _wxObject {
}

struct cbRowLayoutPlugin(*u8);
impl _cbRowLayoutPlugin for cbRowLayoutPlugin {}
impl _cbPluginBase for cbRowLayoutPlugin {}
impl _wxEvtHandler for cbRowLayoutPlugin {}
impl _wxObject for cbRowLayoutPlugin { fn handle(&self) -> *u8 { **self } }

impl cbRowLayoutPlugin {
    pub fn from(handle: *u8) -> @cbRowLayoutPlugin { @cbRowLayoutPlugin(handle) }
    pub fn null() -> @cbRowLayoutPlugin { cbRowLayoutPlugin::from(0 as *u8) }
    
}

trait _cbRowLayoutPlugin : _cbPluginBase {
}

struct cbSimpleCustomizationPlugin(*u8);
impl _cbSimpleCustomizationPlugin for cbSimpleCustomizationPlugin {}
impl _cbPluginBase for cbSimpleCustomizationPlugin {}
impl _wxEvtHandler for cbSimpleCustomizationPlugin {}
impl _wxObject for cbSimpleCustomizationPlugin { fn handle(&self) -> *u8 { **self } }

impl cbSimpleCustomizationPlugin {
    pub fn from(handle: *u8) -> @cbSimpleCustomizationPlugin { @cbSimpleCustomizationPlugin(handle) }
    pub fn null() -> @cbSimpleCustomizationPlugin { cbSimpleCustomizationPlugin::from(0 as *u8) }
    
}

trait _cbSimpleCustomizationPlugin : _cbPluginBase {
}

struct cbSimpleUpdatesMgr(*u8);
impl _cbSimpleUpdatesMgr for cbSimpleUpdatesMgr {}
impl _cbUpdatesManagerBase for cbSimpleUpdatesMgr {}
impl _wxObject for cbSimpleUpdatesMgr { fn handle(&self) -> *u8 { **self } }

impl cbSimpleUpdatesMgr {
    pub fn from(handle: *u8) -> @cbSimpleUpdatesMgr { @cbSimpleUpdatesMgr(handle) }
    pub fn null() -> @cbSimpleUpdatesMgr { cbSimpleUpdatesMgr::from(0 as *u8) }
    
}

trait _cbSimpleUpdatesMgr : _cbUpdatesManagerBase {
}

struct cbSizeBarWndEvent(*u8);
impl _cbSizeBarWndEvent for cbSizeBarWndEvent {}
impl _cbPluginEvent for cbSizeBarWndEvent {}
impl _wxEvent for cbSizeBarWndEvent {}
impl _wxObject for cbSizeBarWndEvent { fn handle(&self) -> *u8 { **self } }

impl cbSizeBarWndEvent {
    pub fn from(handle: *u8) -> @cbSizeBarWndEvent { @cbSizeBarWndEvent(handle) }
    pub fn null() -> @cbSizeBarWndEvent { cbSizeBarWndEvent::from(0 as *u8) }
    
}

trait _cbSizeBarWndEvent : _cbPluginEvent {
}

struct cbStartBarDraggingEvent(*u8);
impl _cbStartBarDraggingEvent for cbStartBarDraggingEvent {}
impl _cbPluginEvent for cbStartBarDraggingEvent {}
impl _wxEvent for cbStartBarDraggingEvent {}
impl _wxObject for cbStartBarDraggingEvent { fn handle(&self) -> *u8 { **self } }

impl cbStartBarDraggingEvent {
    pub fn from(handle: *u8) -> @cbStartBarDraggingEvent { @cbStartBarDraggingEvent(handle) }
    pub fn null() -> @cbStartBarDraggingEvent { cbStartBarDraggingEvent::from(0 as *u8) }
    
}

trait _cbStartBarDraggingEvent : _cbPluginEvent {
}

struct cbStartDrawInAreaEvent(*u8);
impl _cbStartDrawInAreaEvent for cbStartDrawInAreaEvent {}
impl _cbPluginEvent for cbStartDrawInAreaEvent {}
impl _wxEvent for cbStartDrawInAreaEvent {}
impl _wxObject for cbStartDrawInAreaEvent { fn handle(&self) -> *u8 { **self } }

impl cbStartDrawInAreaEvent {
    pub fn from(handle: *u8) -> @cbStartDrawInAreaEvent { @cbStartDrawInAreaEvent(handle) }
    pub fn null() -> @cbStartDrawInAreaEvent { cbStartDrawInAreaEvent::from(0 as *u8) }
    
}

trait _cbStartDrawInAreaEvent : _cbPluginEvent {
}

struct cbUpdatesManagerBase(*u8);
impl _cbUpdatesManagerBase for cbUpdatesManagerBase {}
impl _wxObject for cbUpdatesManagerBase { fn handle(&self) -> *u8 { **self } }

impl cbUpdatesManagerBase {
    pub fn from(handle: *u8) -> @cbUpdatesManagerBase { @cbUpdatesManagerBase(handle) }
    pub fn null() -> @cbUpdatesManagerBase { cbUpdatesManagerBase::from(0 as *u8) }
    
}

trait _cbUpdatesManagerBase : _wxObject {
}

struct wxAcceleratorEntry(*u8);
impl _wxAcceleratorEntry for wxAcceleratorEntry { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorEntry {
    pub fn from(handle: *u8) -> @wxAcceleratorEntry { @wxAcceleratorEntry(handle) }
    pub fn null() -> @wxAcceleratorEntry { wxAcceleratorEntry::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> @wxAcceleratorEntry {
        unsafe { @wxAcceleratorEntry(wxAcceleratorEntry_Create(flags, keyCode, cmd)) }
    }
}

trait _wxAcceleratorEntry {
    fn handle(&self) -> *u8;
    
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

struct wxAcceleratorTable(*u8);
impl _wxAcceleratorTable for wxAcceleratorTable { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorTable {
    pub fn from(handle: *u8) -> @wxAcceleratorTable { @wxAcceleratorTable(handle) }
    pub fn null() -> @wxAcceleratorTable { wxAcceleratorTable::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(n: c_int, entries: *u8) -> @wxAcceleratorTable {
        unsafe { @wxAcceleratorTable(wxAcceleratorTable_Create(n, entries)) }
    }
}

trait _wxAcceleratorTable {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.handle()) }
    }
}

struct wxActivateEvent(*u8);
impl _wxActivateEvent for wxActivateEvent {}
impl _wxEvent for wxActivateEvent {}
impl _wxObject for wxActivateEvent { fn handle(&self) -> *u8 { **self } }

impl wxActivateEvent {
    pub fn from(handle: *u8) -> @wxActivateEvent { @wxActivateEvent(handle) }
    pub fn null() -> @wxActivateEvent { wxActivateEvent::from(0 as *u8) }
    
}

trait _wxActivateEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getActive(&self) -> bool {
        unsafe { wxActivateEvent_GetActive(self.handle()) }
    }
}

struct wxApp(*u8);
impl _wxApp for wxApp {}
impl _wxEvtHandler for wxApp {}
impl _wxObject for wxApp { fn handle(&self) -> *u8 { **self } }

impl wxApp {
    pub fn from(handle: *u8) -> @wxApp { @wxApp(handle) }
    pub fn null() -> @wxApp { wxApp::from(0 as *u8) }
    
}

trait _wxApp : _wxEvtHandler {
}

struct wxArray(*u8);
impl _wxArray for wxArray { fn handle(&self) -> *u8 { **self } }

impl wxArray {
    pub fn from(handle: *u8) -> @wxArray { @wxArray(handle) }
    pub fn null() -> @wxArray { wxArray::from(0 as *u8) }
    
}

trait _wxArray {
    fn handle(&self) -> *u8;
    
}

struct wxArrayString(*u8);
impl _wxArrayString for wxArrayString {}
impl _wxArray for wxArrayString { fn handle(&self) -> *u8 { **self } }

impl wxArrayString {
    pub fn from(handle: *u8) -> @wxArrayString { @wxArrayString(handle) }
    pub fn null() -> @wxArrayString { wxArrayString::from(0 as *u8) }
    
}

trait _wxArrayString : _wxArray {
}

struct wxArtProvider(*u8);
impl _wxArtProvider for wxArtProvider {}
impl _wxObject for wxArtProvider { fn handle(&self) -> *u8 { **self } }

impl wxArtProvider {
    pub fn from(handle: *u8) -> @wxArtProvider { @wxArtProvider(handle) }
    pub fn null() -> @wxArtProvider { wxArtProvider::from(0 as *u8) }
    
}

trait _wxArtProvider : _wxObject {
}

struct wxAutoBufferedPaintDC(*u8);
impl _wxAutoBufferedPaintDC for wxAutoBufferedPaintDC {}
impl _wxDC for wxAutoBufferedPaintDC {}
impl _wxObject for wxAutoBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxAutoBufferedPaintDC {
    pub fn from(handle: *u8) -> @wxAutoBufferedPaintDC { @wxAutoBufferedPaintDC(handle) }
    pub fn null() -> @wxAutoBufferedPaintDC { wxAutoBufferedPaintDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(window: &T) -> @wxAutoBufferedPaintDC {
        unsafe { @wxAutoBufferedPaintDC(wxAutoBufferedPaintDC_Create(window.handle())) }
    }
}

trait _wxAutoBufferedPaintDC : _wxDC {
}

struct wxAutomationObject(*u8);
impl _wxAutomationObject for wxAutomationObject {}
impl _wxObject for wxAutomationObject { fn handle(&self) -> *u8 { **self } }

impl wxAutomationObject {
    pub fn from(handle: *u8) -> @wxAutomationObject { @wxAutomationObject(handle) }
    pub fn null() -> @wxAutomationObject { wxAutomationObject::from(0 as *u8) }
    
}

trait _wxAutomationObject : _wxObject {
}

struct wxBitmap(*u8);
impl _wxBitmap for wxBitmap {}
impl _wxGDIObject for wxBitmap {}
impl _wxObject for wxBitmap { fn handle(&self) -> *u8 { **self } }

impl wxBitmap {
    pub fn from(handle: *u8) -> @wxBitmap { @wxBitmap(handle) }
    pub fn null() -> @wxBitmap { wxBitmap::from(0 as *u8) }
    
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
    pub fn new(_data: *u8, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> @wxBitmap {
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
    pub fn findHandlerByName(name: &str) -> *u8 {
        let name = wxT(name);
        unsafe { wxBitmap_FindHandlerByName(name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findHandlerByType(type_: c_int) -> *u8 {
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
    pub fn removeHandler(name: &str) -> bool {
        let name = wxT(name);
        unsafe { wxBitmap_RemoveHandler(name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromImage<T: _wxImage>(image: &T, depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromImage(image.handle(), depth)) }
    }
}

trait _wxBitmap : _wxGDIObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newFromXPM(&self) -> @wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromXPM(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findHandlerByExtension(&self, type_: c_int) -> *u8 {
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
    fn isOk(&self) -> bool {
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
    fn isStatic(&self) -> bool {
        unsafe { wxBitmap_IsStatic(self.handle()) }
    }
}

struct wxBitmapButton(*u8);
impl _wxBitmapButton for wxBitmapButton {}
impl _wxButton for wxBitmapButton {}
impl _wxControl for wxBitmapButton {}
impl _wxWindow for wxBitmapButton {}
impl _wxEvtHandler for wxBitmapButton {}
impl _wxObject for wxBitmapButton { fn handle(&self) -> *u8 { **self } }

impl wxBitmapButton {
    pub fn from(handle: *u8) -> @wxBitmapButton { @wxBitmapButton(handle) }
    pub fn null() -> @wxBitmapButton { wxBitmapButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxBitmapButton {
        unsafe { @wxBitmapButton(wxBitmapButton_Create(_prt.handle(), _id, _bmp.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxBitmapButton : _wxButton {
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

struct wxBitmapToggleButton(*u8);
impl _wxBitmapToggleButton for wxBitmapToggleButton {}
impl _wxToggleButton for wxBitmapToggleButton {}
impl _wxControl for wxBitmapToggleButton {}
impl _wxWindow for wxBitmapToggleButton {}
impl _wxEvtHandler for wxBitmapToggleButton {}
impl _wxObject for wxBitmapToggleButton { fn handle(&self) -> *u8 { **self } }

impl wxBitmapToggleButton {
    pub fn from(handle: *u8) -> @wxBitmapToggleButton { @wxBitmapToggleButton(handle) }
    pub fn null() -> @wxBitmapToggleButton { wxBitmapToggleButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(parent: &T, id: c_int, _bmp: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxBitmapToggleButton {
        unsafe { @wxBitmapToggleButton(wxBitmapToggleButton_Create(parent.handle(), id, _bmp.handle(), x, y, w, h, style)) }
    }
}

trait _wxBitmapToggleButton : _wxToggleButton {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBitmapLabel<T: _wxBitmap>(&self, _bmp: &T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.handle(), _bmp.handle()) }
    }
}

struct wxBitmapDataObject(*u8);
impl _wxBitmapDataObject for wxBitmapDataObject {}
impl _wxDataObjectSimple for wxBitmapDataObject {}
impl _wxDataObject for wxBitmapDataObject { fn handle(&self) -> *u8 { **self } }

impl wxBitmapDataObject {
    pub fn from(handle: *u8) -> @wxBitmapDataObject { @wxBitmapDataObject(handle) }
    pub fn null() -> @wxBitmapDataObject { wxBitmapDataObject::from(0 as *u8) }
    
}

trait _wxBitmapDataObject : _wxDataObjectSimple {
}

struct wxBitmapHandler(*u8);
impl _wxBitmapHandler for wxBitmapHandler {}
impl _wxObject for wxBitmapHandler { fn handle(&self) -> *u8 { **self } }

impl wxBitmapHandler {
    pub fn from(handle: *u8) -> @wxBitmapHandler { @wxBitmapHandler(handle) }
    pub fn null() -> @wxBitmapHandler { wxBitmapHandler::from(0 as *u8) }
    
}

trait _wxBitmapHandler : _wxObject {
}

struct wxBoxSizer(*u8);
impl _wxBoxSizer for wxBoxSizer {}
impl _wxSizer for wxBoxSizer {}
impl _wxObject for wxBoxSizer { fn handle(&self) -> *u8 { **self } }

impl wxBoxSizer {
    pub fn from(handle: *u8) -> @wxBoxSizer { @wxBoxSizer(handle) }
    pub fn null() -> @wxBoxSizer { wxBoxSizer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(orient: c_int) -> @wxBoxSizer {
        unsafe { @wxBoxSizer(wxBoxSizer_Create(orient)) }
    }
}

trait _wxBoxSizer : _wxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.handle()) }
    }
}

struct wxBrush(*u8);
impl _wxBrush for wxBrush {}
impl _wxGDIObject for wxBrush {}
impl _wxObject for wxBrush { fn handle(&self) -> *u8 { **self } }

impl wxBrush {
    pub fn from(handle: *u8) -> @wxBrush { @wxBrush(handle) }
    pub fn null() -> @wxBrush { wxBrush::from(0 as *u8) }
    
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

trait _wxBrush : _wxGDIObject {
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
    fn isEqual<T: _wxBrush>(&self, brush: &T) -> bool {
        unsafe { wxBrush_IsEqual(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxBrush_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxBrush_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
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
    fn isStatic(&self) -> bool {
        unsafe { wxBrush_IsStatic(self.handle()) }
    }
}

struct wxBrushList(*u8);
impl _wxBrushList for wxBrushList {}
impl _wxList for wxBrushList {}
impl _wxObject for wxBrushList { fn handle(&self) -> *u8 { **self } }

impl wxBrushList {
    pub fn from(handle: *u8) -> @wxBrushList { @wxBrushList(handle) }
    pub fn null() -> @wxBrushList { wxBrushList::from(0 as *u8) }
    
}

trait _wxBrushList : _wxList {
}

struct wxBufferedDC(*u8);
impl _wxBufferedDC for wxBufferedDC {}
impl _wxDC for wxBufferedDC {}
impl _wxObject for wxBufferedDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedDC {
    pub fn from(handle: *u8) -> @wxBufferedDC { @wxBufferedDC(handle) }
    pub fn null() -> @wxBufferedDC { wxBufferedDC::from(0 as *u8) }
    
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

trait _wxBufferedDC : _wxDC {
}

struct wxBufferedPaintDC(*u8);
impl _wxBufferedPaintDC for wxBufferedPaintDC {}
impl _wxDC for wxBufferedPaintDC {}
impl _wxObject for wxBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedPaintDC {
    pub fn from(handle: *u8) -> @wxBufferedPaintDC { @wxBufferedPaintDC(handle) }
    pub fn null() -> @wxBufferedPaintDC { wxBufferedPaintDC::from(0 as *u8) }
    
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

trait _wxBufferedPaintDC : _wxDC {
}

struct wxBufferedInputStream(*u8);
impl _wxBufferedInputStream for wxBufferedInputStream {}
impl _wxFilterInputStream for wxBufferedInputStream {}
impl _wxInputStream for wxBufferedInputStream {}
impl _wxStreamBase for wxBufferedInputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedInputStream {
    pub fn from(handle: *u8) -> @wxBufferedInputStream { @wxBufferedInputStream(handle) }
    pub fn null() -> @wxBufferedInputStream { wxBufferedInputStream::from(0 as *u8) }
    
}

trait _wxBufferedInputStream : _wxFilterInputStream {
}

struct wxBufferedOutputStream(*u8);
impl _wxBufferedOutputStream for wxBufferedOutputStream {}
impl _wxFilterOutputStream for wxBufferedOutputStream {}
impl _wxOutputStream for wxBufferedOutputStream {}
impl _wxStreamBase for wxBufferedOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedOutputStream {
    pub fn from(handle: *u8) -> @wxBufferedOutputStream { @wxBufferedOutputStream(handle) }
    pub fn null() -> @wxBufferedOutputStream { wxBufferedOutputStream::from(0 as *u8) }
    
}

trait _wxBufferedOutputStream : _wxFilterOutputStream {
}

struct wxBusyCursor(*u8);
impl _wxBusyCursor for wxBusyCursor { fn handle(&self) -> *u8 { **self } }

impl wxBusyCursor {
    pub fn from(handle: *u8) -> @wxBusyCursor { @wxBusyCursor(handle) }
    pub fn null() -> @wxBusyCursor { wxBusyCursor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxBusyCursor {
        unsafe { @wxBusyCursor(wxBusyCursor_Create()) }
    }
}

trait _wxBusyCursor {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn newWithCursor(&self) -> *u8 {
        unsafe { wxBusyCursor_CreateWithCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.handle()) }
    }
}

struct wxBusyInfo(*u8);
impl _wxBusyInfo for wxBusyInfo { fn handle(&self) -> *u8 { **self } }

impl wxBusyInfo {
    pub fn from(handle: *u8) -> @wxBusyInfo { @wxBusyInfo(handle) }
    pub fn null() -> @wxBusyInfo { wxBusyInfo::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_txt: &str) -> @wxBusyInfo {
        let _txt = wxT(_txt);
        unsafe { @wxBusyInfo(wxBusyInfo_Create(_txt.handle())) }
    }
}

trait _wxBusyInfo {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.handle()) }
    }
}

struct wxButton(*u8);
impl _wxButton for wxButton {}
impl _wxControl for wxButton {}
impl _wxWindow for wxButton {}
impl _wxEvtHandler for wxButton {}
impl _wxObject for wxButton { fn handle(&self) -> *u8 { **self } }

impl wxButton {
    pub fn from(handle: *u8) -> @wxButton { @wxButton(handle) }
    pub fn null() -> @wxButton { wxButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxButton {
        let _txt = wxT(_txt);
        unsafe { @wxButton(wxButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.handle()) }
    }
}

struct wxCSConv(*u8);
impl _wxCSConv for wxCSConv {}
impl _wxMBConv for wxCSConv { fn handle(&self) -> *u8 { **self } }

impl wxCSConv {
    pub fn from(handle: *u8) -> @wxCSConv { @wxCSConv(handle) }
    pub fn null() -> @wxCSConv { wxCSConv::from(0 as *u8) }
    
}

trait _wxCSConv : _wxMBConv {
}

struct wxCalculateLayoutEvent(*u8);
impl _wxCalculateLayoutEvent for wxCalculateLayoutEvent {}
impl _wxEvent for wxCalculateLayoutEvent {}
impl _wxObject for wxCalculateLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalculateLayoutEvent {
    pub fn from(handle: *u8) -> @wxCalculateLayoutEvent { @wxCalculateLayoutEvent(handle) }
    pub fn null() -> @wxCalculateLayoutEvent { wxCalculateLayoutEvent::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(id: c_int) -> @wxCalculateLayoutEvent {
        unsafe { @wxCalculateLayoutEvent(wxCalculateLayoutEvent_Create(id)) }
    }
}

trait _wxCalculateLayoutEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRect(&self) -> @wxRect {
        unsafe { @wxRect(wxCalculateLayoutEvent_GetRect(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxCalculateLayoutEvent_SetRect(self.handle(), x, y, w, h) }
    }
}

struct wxCalendarCtrl(*u8);
impl _wxCalendarCtrl for wxCalendarCtrl {}
impl _wxControl for wxCalendarCtrl {}
impl _wxWindow for wxCalendarCtrl {}
impl _wxEvtHandler for wxCalendarCtrl {}
impl _wxObject for wxCalendarCtrl { fn handle(&self) -> *u8 { **self } }

impl wxCalendarCtrl {
    pub fn from(handle: *u8) -> @wxCalendarCtrl { @wxCalendarCtrl(handle) }
    pub fn null() -> @wxCalendarCtrl { wxCalendarCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxDateTime>(_prt: &T, _id: c_int, _dat: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxCalendarCtrl {
        unsafe { @wxCalendarCtrl(wxCalendarCtrl_Create(_prt.handle(), _id, _dat.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxCalendarCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableHolidayDisplay(&self, display: c_int) {
        unsafe { wxCalendarCtrl_EnableHolidayDisplay(self.handle(), display) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableMonthChange(&self, enable: bool) {
        unsafe { wxCalendarCtrl_EnableMonthChange(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAttr(&self, day: c_int) -> *u8 {
        unsafe { wxCalendarCtrl_GetAttr(self.handle(), day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDate(&self, date: *u8) {
        unsafe { wxCalendarCtrl_GetDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeaderColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeaderColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHighlightColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHighlightColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHolidayColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHolidayColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hitTest(&self, x: c_int, y: c_int, date: *u8, wd: *u8) -> c_int {
        unsafe { wxCalendarCtrl_HitTest(self.handle(), x, y, date, wd) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetAttr(&self, day: c_int) {
        unsafe { wxCalendarCtrl_ResetAttr(self.handle(), day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAttr(&self, day: c_int, attr: *u8) {
        unsafe { wxCalendarCtrl_SetAttr(self.handle(), day, attr) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDate(&self, date: *u8) {
        unsafe { wxCalendarCtrl_SetDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHeaderColours(&self, colFg: *u8, colBg: *u8) {
        unsafe { wxCalendarCtrl_SetHeaderColours(self.handle(), colFg, colBg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHighlightColours(&self, colFg: *u8, colBg: *u8) {
        unsafe { wxCalendarCtrl_SetHighlightColours(self.handle(), colFg, colBg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHoliday(&self, day: c_int) {
        unsafe { wxCalendarCtrl_SetHoliday(self.handle(), day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHolidayColours(&self, colFg: *u8, colBg: *u8) {
        unsafe { wxCalendarCtrl_SetHolidayColours(self.handle(), colFg, colBg) }
    }
}

struct wxCalendarDateAttr(*u8);
impl _wxCalendarDateAttr for wxCalendarDateAttr { fn handle(&self) -> *u8 { **self } }

impl wxCalendarDateAttr {
    pub fn from(handle: *u8) -> @wxCalendarDateAttr { @wxCalendarDateAttr(handle) }
    pub fn null() -> @wxCalendarDateAttr { wxCalendarDateAttr::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_ctxt: *u8, _cbck: *u8, _cbrd: *u8, _fnt: *u8, _brd: c_int) -> @wxCalendarDateAttr {
        unsafe { @wxCalendarDateAttr(wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxCalendarDateAttr {
        unsafe { @wxCalendarDateAttr(wxCalendarDateAttr_CreateDefault()) }
    }
}

trait _wxCalendarDateAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBorderColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBorder(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBorderColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBorderColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFont(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasTextColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isHoliday(&self) -> bool {
        unsafe { wxCalendarDateAttr_IsHoliday(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorderColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxCalendarDateAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.handle(), holiday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.handle(), col.handle()) }
    }
}

struct wxCalendarEvent(*u8);
impl _wxCalendarEvent for wxCalendarEvent {}
impl _wxCommandEvent for wxCalendarEvent {}
impl _wxEvent for wxCalendarEvent {}
impl _wxObject for wxCalendarEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalendarEvent {
    pub fn from(handle: *u8) -> @wxCalendarEvent { @wxCalendarEvent(handle) }
    pub fn null() -> @wxCalendarEvent { wxCalendarEvent::from(0 as *u8) }
    
}

trait _wxCalendarEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDate(&self, _dte: *u8) {
        unsafe { wxCalendarEvent_GetDate(self.handle(), _dte) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.handle()) }
    }
}

struct wxCaret(*u8);
impl _wxCaret for wxCaret { fn handle(&self) -> *u8 { **self } }

impl wxCaret {
    pub fn from(handle: *u8) -> @wxCaret { @wxCaret(handle) }
    pub fn null() -> @wxCaret { wxCaret::from(0 as *u8) }
    
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

trait _wxCaret {
    fn handle(&self) -> *u8;
    
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
    fn isOk(&self) -> bool {
        unsafe { wxCaret_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible(&self) -> bool {
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

struct wxCheckBox(*u8);
impl _wxCheckBox for wxCheckBox {}
impl _wxControl for wxCheckBox {}
impl _wxWindow for wxCheckBox {}
impl _wxEvtHandler for wxCheckBox {}
impl _wxObject for wxCheckBox { fn handle(&self) -> *u8 { **self } }

impl wxCheckBox {
    pub fn from(handle: *u8) -> @wxCheckBox { @wxCheckBox(handle) }
    pub fn null() -> @wxCheckBox { wxCheckBox::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxCheckBox {
        let _txt = wxT(_txt);
        unsafe { @wxCheckBox(wxCheckBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxCheckBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> bool {
        unsafe { wxCheckBox_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self.handle(), value) }
    }
}

struct wxCheckListBox(*u8);
impl _wxCheckListBox for wxCheckListBox {}
impl _wxListBox for wxCheckListBox {}
impl _wxControl for wxCheckListBox {}
impl _wxWindow for wxCheckListBox {}
impl _wxEvtHandler for wxCheckListBox {}
impl _wxObject for wxCheckListBox { fn handle(&self) -> *u8 { **self } }

impl wxCheckListBox {
    pub fn from(handle: *u8) -> @wxCheckListBox { @wxCheckListBox(handle) }
    pub fn null() -> @wxCheckListBox { wxCheckListBox::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxCheckListBox {
        unsafe { @wxCheckListBox(wxCheckListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxCheckListBox : _wxListBox {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, item: c_int, check: bool) {
        unsafe { wxCheckListBox_Check(self.handle(), item, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, item: c_int) -> bool {
        unsafe { wxCheckListBox_IsChecked(self.handle(), item) }
    }
}

struct wxChoice(*u8);
impl _wxChoice for wxChoice {}
impl _wxControl for wxChoice {}
impl _wxWindow for wxChoice {}
impl _wxEvtHandler for wxChoice {}
impl _wxObject for wxChoice { fn handle(&self) -> *u8 { **self } }

impl wxChoice {
    pub fn from(handle: *u8) -> @wxChoice { @wxChoice(handle) }
    pub fn null() -> @wxChoice { wxChoice::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxChoice {
        unsafe { @wxChoice(wxChoice_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxChoice : _wxControl {
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

struct wxClassInfo(*u8);
impl _wxClassInfo for wxClassInfo { fn handle(&self) -> *u8 { **self } }

impl wxClassInfo {
    pub fn from(handle: *u8) -> @wxClassInfo { @wxClassInfo(handle) }
    pub fn null() -> @wxClassInfo { wxClassInfo::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn findClass(_txt: &str) -> @wxClassInfo {
        let _txt = wxT(_txt);
        unsafe { @wxClassInfo(wxClassInfo_FindClass(_txt.handle())) }
    }
}

trait _wxClassInfo {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn newClassByName(&self) -> *u8 {
        unsafe { wxClassInfo_CreateClassByName(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClassName(&self) -> *u8 {
        unsafe { wxClassInfo_GetClassName(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isKindOf(&self, _name: &str) -> bool {
        let _name = wxT(_name);
        unsafe { wxClassInfo_IsKindOf(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBaseClassName1(&self) -> ~str {
        unsafe { wxString { handle: wxClassInfo_GetBaseClassName1(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBaseClassName2(&self) -> ~str {
        unsafe { wxString { handle: wxClassInfo_GetBaseClassName2(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClassNameEx(&self) -> ~str {
        unsafe { wxString { handle: wxClassInfo_GetClassNameEx(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isKindOfEx<T: _wxClassInfo>(&self, classInfo: &T) -> bool {
        unsafe { wxClassInfo_IsKindOfEx(self.handle(), classInfo.handle()) }
    }
}

struct wxClient(*u8);
impl _wxClient for wxClient {}
impl _wxClientBase for wxClient {}
impl _wxObject for wxClient { fn handle(&self) -> *u8 { **self } }

impl wxClient {
    pub fn from(handle: *u8) -> @wxClient { @wxClient(handle) }
    pub fn null() -> @wxClient { wxClient::from(0 as *u8) }
    
}

trait _wxClient : _wxClientBase {
}

struct wxClientBase(*u8);
impl _wxClientBase for wxClientBase {}
impl _wxObject for wxClientBase { fn handle(&self) -> *u8 { **self } }

impl wxClientBase {
    pub fn from(handle: *u8) -> @wxClientBase { @wxClientBase(handle) }
    pub fn null() -> @wxClientBase { wxClientBase::from(0 as *u8) }
    
}

trait _wxClientBase : _wxObject {
}

struct wxClientDC(*u8);
impl _wxClientDC for wxClientDC {}
impl _wxWindowDC for wxClientDC {}
impl _wxDC for wxClientDC {}
impl _wxObject for wxClientDC { fn handle(&self) -> *u8 { **self } }

impl wxClientDC {
    pub fn from(handle: *u8) -> @wxClientDC { @wxClientDC(handle) }
    pub fn null() -> @wxClientDC { wxClientDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> @wxClientDC {
        unsafe { @wxClientDC(wxClientDC_Create(win.handle())) }
    }
}

trait _wxClientDC : _wxWindowDC {
}

struct wxClientData(*u8);
impl _wxClientData for wxClientData { fn handle(&self) -> *u8 { **self } }

impl wxClientData {
    pub fn from(handle: *u8) -> @wxClientData { @wxClientData(handle) }
    pub fn null() -> @wxClientData { wxClientData::from(0 as *u8) }
    
}

trait _wxClientData {
    fn handle(&self) -> *u8;
    
}

struct wxClientDataContainer(*u8);
impl _wxClientDataContainer for wxClientDataContainer { fn handle(&self) -> *u8 { **self } }

impl wxClientDataContainer {
    pub fn from(handle: *u8) -> @wxClientDataContainer { @wxClientDataContainer(handle) }
    pub fn null() -> @wxClientDataContainer { wxClientDataContainer::from(0 as *u8) }
    
}

trait _wxClientDataContainer {
    fn handle(&self) -> *u8;
    
}

struct wxClipboard(*u8);
impl _wxClipboard for wxClipboard {}
impl _wxObject for wxClipboard { fn handle(&self) -> *u8 { **self } }

impl wxClipboard {
    pub fn from(handle: *u8) -> @wxClipboard { @wxClipboard(handle) }
    pub fn null() -> @wxClipboard { wxClipboard::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxClipboard {
        unsafe { @wxClipboard(wxClipboard_Create()) }
    }
}

trait _wxClipboard : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addData<T: _wxDataObject>(&self, data: &T) -> bool {
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
    fn flush(&self) -> bool {
        unsafe { wxClipboard_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData<T: _wxDataObject>(&self, data: &T) -> bool {
        unsafe { wxClipboard_GetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOpened(&self) -> bool {
        unsafe { wxClipboard_IsOpened(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSupported<T: _wxDataFormat>(&self, format: &T) -> bool {
        unsafe { wxClipboard_IsSupported(self.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn open(&self) -> bool {
        unsafe { wxClipboard_Open(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setData<T: _wxDataObject>(&self, data: &T) -> bool {
        unsafe { wxClipboard_SetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn usePrimarySelection(&self, primary: bool) {
        unsafe { wxClipboard_UsePrimarySelection(self.handle(), primary) }
    }
}

struct wxCloseEvent(*u8);
impl _wxCloseEvent for wxCloseEvent {}
impl _wxEvent for wxCloseEvent {}
impl _wxObject for wxCloseEvent { fn handle(&self) -> *u8 { **self } }

impl wxCloseEvent {
    pub fn from(handle: *u8) -> @wxCloseEvent { @wxCloseEvent(handle) }
    pub fn null() -> @wxCloseEvent { wxCloseEvent::from(0 as *u8) }
    
}

trait _wxCloseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn canVeto(&self) -> bool {
        unsafe { wxCloseEvent_CanVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLoggingOff(&self) -> bool {
        unsafe { wxCloseEvent_GetLoggingOff(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVeto(&self) -> bool {
        unsafe { wxCloseEvent_GetVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCanVeto(&self, canVeto: bool) {
        unsafe { wxCloseEvent_SetCanVeto(self.handle(), canVeto) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLoggingOff(&self, logOff: bool) {
        unsafe { wxCloseEvent_SetLoggingOff(self.handle(), logOff) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn veto(&self, veto: bool) {
        unsafe { wxCloseEvent_Veto(self.handle(), veto) }
    }
}

struct wxClosure(*u8);
impl _wxClosure for wxClosure {}
impl _wxObject for wxClosure { fn handle(&self) -> *u8 { **self } }

impl wxClosure {
    pub fn from(handle: *u8) -> @wxClosure { @wxClosure(handle) }
    pub fn null() -> @wxClosure { wxClosure::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_fun_CEvent: *u8, _data: *u8) -> @wxClosure {
        unsafe { @wxClosure(wxClosure_Create(_fun_CEvent, _data)) }
    }
}

trait _wxClosure : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> *u8 {
        unsafe { wxClosure_GetData(self.handle()) }
    }
}

struct wxColour(*u8);
impl _wxColour for wxColour {}
impl _wxObject for wxColour { fn handle(&self) -> *u8 { **self } }

impl wxColour {
    pub fn from(handle: *u8) -> @wxColour { @wxColour(handle) }
    pub fn null() -> @wxColour { wxColour::from(0 as *u8) }
    
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
    pub fn validName(_name: *wchar_t) -> bool {
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

trait _wxColour : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn alpha(&self) -> uint8_t {
        unsafe { wxColour_Alpha(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign(&self, other: *u8) {
        unsafe { wxColour_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn blue(&self) -> uint8_t {
        unsafe { wxColour_Blue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn copy(&self, _other: *u8) {
        unsafe { wxColour_Copy(self.handle(), _other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn green(&self) -> uint8_t {
        unsafe { wxColour_Green(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
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
    fn isStatic(&self) -> bool {
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

struct wxColourData(*u8);
impl _wxColourData for wxColourData {}
impl _wxObject for wxColourData { fn handle(&self) -> *u8 { **self } }

impl wxColourData {
    pub fn from(handle: *u8) -> @wxColourData { @wxColourData(handle) }
    pub fn null() -> @wxColourData { wxColourData::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxColourData {
        unsafe { @wxColourData(wxColourData_Create()) }
    }
}

trait _wxColourData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChooseFull(&self) -> bool {
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
    fn setChooseFull(&self, flag: bool) {
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

struct wxColourDatabase(*u8);
impl _wxColourDatabase for wxColourDatabase {}
impl _wxList for wxColourDatabase {}
impl _wxObject for wxColourDatabase { fn handle(&self) -> *u8 { **self } }

impl wxColourDatabase {
    pub fn from(handle: *u8) -> @wxColourDatabase { @wxColourDatabase(handle) }
    pub fn null() -> @wxColourDatabase { wxColourDatabase::from(0 as *u8) }
    
}

trait _wxColourDatabase : _wxList {
}

struct wxColourDialog(*u8);
impl _wxColourDialog for wxColourDialog {}
impl _wxDialog for wxColourDialog {}
impl _wxTopLevelWindow for wxColourDialog {}
impl _wxWindow for wxColourDialog {}
impl _wxEvtHandler for wxColourDialog {}
impl _wxObject for wxColourDialog { fn handle(&self) -> *u8 { **self } }

impl wxColourDialog {
    pub fn from(handle: *u8) -> @wxColourDialog { @wxColourDialog(handle) }
    pub fn null() -> @wxColourDialog { wxColourDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxColourData>(_prt: &T, col: &U) -> @wxColourDialog {
        unsafe { @wxColourDialog(wxColourDialog_Create(_prt.handle(), col.handle())) }
    }
}

trait _wxColourDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColourData<T: _wxColourData>(&self, _ref: &T) {
        unsafe { wxColourDialog_GetColourData(self.handle(), _ref.handle()) }
    }
}

struct wxComboBox(*u8);
impl _wxComboBox for wxComboBox {}
impl _wxChoice for wxComboBox {}
impl _wxControl for wxComboBox {}
impl _wxWindow for wxComboBox {}
impl _wxEvtHandler for wxComboBox {}
impl _wxObject for wxComboBox { fn handle(&self) -> *u8 { **self } }

impl wxComboBox {
    pub fn from(handle: *u8) -> @wxComboBox { @wxComboBox(handle) }
    pub fn null() -> @wxComboBox { wxComboBox::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxComboBox {
        let _txt = wxT(_txt);
        unsafe { @wxComboBox(wxComboBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxComboBox : _wxChoice {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendData(&self, item: &str, d: *u8) {
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
    fn setEditable(&self, editable: bool) {
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

struct wxCommand(*u8);
impl _wxCommand for wxCommand {}
impl _wxObject for wxCommand { fn handle(&self) -> *u8 { **self } }

impl wxCommand {
    pub fn from(handle: *u8) -> @wxCommand { @wxCommand(handle) }
    pub fn null() -> @wxCommand { wxCommand::from(0 as *u8) }
    
}

trait _wxCommand : _wxObject {
}

struct wxCommandEvent(*u8);
impl _wxCommandEvent for wxCommandEvent {}
impl _wxEvent for wxCommandEvent {}
impl _wxObject for wxCommandEvent { fn handle(&self) -> *u8 { **self } }

impl wxCommandEvent {
    pub fn from(handle: *u8) -> @wxCommandEvent { @wxCommandEvent(handle) }
    pub fn null() -> @wxCommandEvent { wxCommandEvent::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_typ: c_int, _id: c_int) -> @wxCommandEvent {
        unsafe { @wxCommandEvent(wxCommandEvent_Create(_typ, _id)) }
    }
}

trait _wxCommandEvent : _wxEvent {
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
    fn isChecked(&self) -> bool {
        unsafe { wxCommandEvent_IsChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelection(&self) -> bool {
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

struct wxCommandLineParser(*u8);
impl _wxCommandLineParser for wxCommandLineParser { fn handle(&self) -> *u8 { **self } }

impl wxCommandLineParser {
    pub fn from(handle: *u8) -> @wxCommandLineParser { @wxCommandLineParser(handle) }
    pub fn null() -> @wxCommandLineParser { wxCommandLineParser::from(0 as *u8) }
    
}

trait _wxCommandLineParser {
    fn handle(&self) -> *u8;
    
}

struct wxCommandProcessor(*u8);
impl _wxCommandProcessor for wxCommandProcessor {}
impl _wxObject for wxCommandProcessor { fn handle(&self) -> *u8 { **self } }

impl wxCommandProcessor {
    pub fn from(handle: *u8) -> @wxCommandProcessor { @wxCommandProcessor(handle) }
    pub fn null() -> @wxCommandProcessor { wxCommandProcessor::from(0 as *u8) }
    
}

trait _wxCommandProcessor : _wxObject {
}

struct wxCondition(*u8);
impl _wxCondition for wxCondition { fn handle(&self) -> *u8 { **self } }

impl wxCondition {
    pub fn from(handle: *u8) -> @wxCondition { @wxCondition(handle) }
    pub fn null() -> @wxCondition { wxCondition::from(0 as *u8) }
    
}

trait _wxCondition {
    fn handle(&self) -> *u8;
    
}

struct wxConfigBase(*u8);
impl _wxConfigBase for wxConfigBase { fn handle(&self) -> *u8 { **self } }

impl wxConfigBase {
    pub fn from(handle: *u8) -> @wxConfigBase { @wxConfigBase(handle) }
    pub fn null() -> @wxConfigBase { wxConfigBase::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> @wxConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Get()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn set<T: _wxConfigBase>(self_: &T) {
        unsafe { wxConfigBase_Set(self_.handle()) }
    }
}

trait _wxConfigBase {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAll(&self) -> bool {
        unsafe { wxConfigBase_DeleteAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteEntry(&self, key: &str, bDeleteGroupIfEmpty: bool) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_DeleteEntry(self.handle(), key.handle(), bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteGroup(&self, key: &str) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_DeleteGroup(self.handle(), key.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn exists(&self, strName: &str) -> bool {
        let strName = wxT(strName);
        unsafe { wxConfigBase_Exists(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expandEnvVars(&self, str: &str) -> ~str {
        let str = wxT(str);
        unsafe { wxString { handle: wxConfigBase_ExpandEnvVars(self.handle(), str.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn flush(&self, bCurrentOnly: bool) -> bool {
        unsafe { wxConfigBase_Flush(self.handle(), bCurrentOnly) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAppName(&self) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetAppName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEntryType(&self, name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxConfigBase_GetEntryType(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstEntry(&self, lIndex: *u8) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetFirstEntry(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFirstGroup(&self, lIndex: *u8) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetFirstGroup(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextEntry(&self, lIndex: *u8) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetNextEntry(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextGroup(&self, lIndex: *u8) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetNextGroup(self.handle(), lIndex) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfEntries(&self, bRecursive: bool) -> c_int {
        unsafe { wxConfigBase_GetNumberOfEntries(self.handle(), bRecursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberOfGroups(&self, bRecursive: bool) -> c_int {
        unsafe { wxConfigBase_GetNumberOfGroups(self.handle(), bRecursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPath(&self) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetPath(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVendorName(&self) -> ~str {
        unsafe { wxString { handle: wxConfigBase_GetVendorName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasEntry(&self, strName: &str) -> bool {
        let strName = wxT(strName);
        unsafe { wxConfigBase_HasEntry(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasGroup(&self, strName: &str) -> bool {
        let strName = wxT(strName);
        unsafe { wxConfigBase_HasGroup(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExpandingEnvVars(&self) -> bool {
        unsafe { wxConfigBase_IsExpandingEnvVars(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRecordingDefaults(&self) -> bool {
        unsafe { wxConfigBase_IsRecordingDefaults(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readBool(&self, key: &str, defVal: bool) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadBool(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readDouble(&self, key: &str, defVal: c_double) -> c_double {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadDouble(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readInteger(&self, key: &str, defVal: c_int) -> c_int {
        let key = wxT(key);
        unsafe { wxConfigBase_ReadInteger(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readString(&self, key: &str, defVal: &str) -> ~str {
        let key = wxT(key);
        let defVal = wxT(defVal);
        unsafe { wxString { handle: wxConfigBase_ReadString(self.handle(), key.handle(), defVal.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renameEntry(&self, oldName: &str, newName: &str) -> bool {
        let oldName = wxT(oldName);
        let newName = wxT(newName);
        unsafe { wxConfigBase_RenameEntry(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renameGroup(&self, oldName: &str, newName: &str) -> bool {
        let oldName = wxT(oldName);
        let newName = wxT(newName);
        unsafe { wxConfigBase_RenameGroup(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAppName(&self, appName: &str) {
        let appName = wxT(appName);
        unsafe { wxConfigBase_SetAppName(self.handle(), appName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExpandEnvVars(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPath(&self, strPath: &str) {
        let strPath = wxT(strPath);
        unsafe { wxConfigBase_SetPath(self.handle(), strPath.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRecordDefaults(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetRecordDefaults(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle(&self, style: c_int) {
        unsafe { wxConfigBase_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVendorName(&self, vendorName: &str) {
        let vendorName = wxT(vendorName);
        unsafe { wxConfigBase_SetVendorName(self.handle(), vendorName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeBool(&self, key: &str, value: bool) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteBool(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeDouble(&self, key: &str, value: c_double) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteDouble(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeInteger(&self, key: &str, value: c_int) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteInteger(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeLong(&self, key: &str, value: c_long) -> bool {
        let key = wxT(key);
        unsafe { wxConfigBase_WriteLong(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeString(&self, key: &str, value: &str) -> bool {
        let key = wxT(key);
        let value = wxT(value);
        unsafe { wxConfigBase_WriteString(self.handle(), key.handle(), value.handle()) }
    }
}

struct wxConnection(*u8);
impl _wxConnection for wxConnection {}
impl _wxConnectionBase for wxConnection {}
impl _wxObject for wxConnection { fn handle(&self) -> *u8 { **self } }

impl wxConnection {
    pub fn from(handle: *u8) -> @wxConnection { @wxConnection(handle) }
    pub fn null() -> @wxConnection { wxConnection::from(0 as *u8) }
    
}

trait _wxConnection : _wxConnectionBase {
}

struct wxConnectionBase(*u8);
impl _wxConnectionBase for wxConnectionBase {}
impl _wxObject for wxConnectionBase { fn handle(&self) -> *u8 { **self } }

impl wxConnectionBase {
    pub fn from(handle: *u8) -> @wxConnectionBase { @wxConnectionBase(handle) }
    pub fn null() -> @wxConnectionBase { wxConnectionBase::from(0 as *u8) }
    
}

trait _wxConnectionBase : _wxObject {
}

struct wxContextHelp(*u8);
impl _wxContextHelp for wxContextHelp {}
impl _wxObject for wxContextHelp { fn handle(&self) -> *u8 { **self } }

impl wxContextHelp {
    pub fn from(handle: *u8) -> @wxContextHelp { @wxContextHelp(handle) }
    pub fn null() -> @wxContextHelp { wxContextHelp::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T, beginHelp: bool) -> @wxContextHelp {
        unsafe { @wxContextHelp(wxContextHelp_Create(win.handle(), beginHelp)) }
    }
}

trait _wxContextHelp : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginContextHelp<T: _wxWindow>(&self, win: &T) -> bool {
        unsafe { wxContextHelp_BeginContextHelp(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endContextHelp(&self) -> bool {
        unsafe { wxContextHelp_EndContextHelp(self.handle()) }
    }
}

struct wxContextHelpButton(*u8);
impl _wxContextHelpButton for wxContextHelpButton {}
impl _wxBitmapButton for wxContextHelpButton {}
impl _wxButton for wxContextHelpButton {}
impl _wxControl for wxContextHelpButton {}
impl _wxWindow for wxContextHelpButton {}
impl _wxEvtHandler for wxContextHelpButton {}
impl _wxObject for wxContextHelpButton { fn handle(&self) -> *u8 { **self } }

impl wxContextHelpButton {
    pub fn from(handle: *u8) -> @wxContextHelpButton { @wxContextHelpButton(handle) }
    pub fn null() -> @wxContextHelpButton { wxContextHelpButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @wxContextHelpButton {
        unsafe { @wxContextHelpButton(wxContextHelpButton_Create(parent.handle(), id, x, y, w, h, style)) }
    }
}

trait _wxContextHelpButton : _wxBitmapButton {
}

struct wxControl(*u8);
impl _wxControl for wxControl {}
impl _wxWindow for wxControl {}
impl _wxEvtHandler for wxControl {}
impl _wxObject for wxControl { fn handle(&self) -> *u8 { **self } }

impl wxControl {
    pub fn from(handle: *u8) -> @wxControl { @wxControl(handle) }
    pub fn null() -> @wxControl { wxControl::from(0 as *u8) }
    
}

trait _wxControl : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn command<T: _wxEvent>(&self, event: &T) {
        unsafe { wxControl_Command(self.handle(), event.handle()) }
    }
}

struct wxCountingOutputStream(*u8);
impl _wxCountingOutputStream for wxCountingOutputStream {}
impl _wxOutputStream for wxCountingOutputStream {}
impl _wxStreamBase for wxCountingOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxCountingOutputStream {
    pub fn from(handle: *u8) -> @wxCountingOutputStream { @wxCountingOutputStream(handle) }
    pub fn null() -> @wxCountingOutputStream { wxCountingOutputStream::from(0 as *u8) }
    
}

trait _wxCountingOutputStream : _wxOutputStream {
}

struct wxCriticalSection(*u8);
impl _wxCriticalSection for wxCriticalSection { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSection {
    pub fn from(handle: *u8) -> @wxCriticalSection { @wxCriticalSection(handle) }
    pub fn null() -> @wxCriticalSection { wxCriticalSection::from(0 as *u8) }
    
}

trait _wxCriticalSection {
    fn handle(&self) -> *u8;
    
}

struct wxCriticalSectionLocker(*u8);
impl _wxCriticalSectionLocker for wxCriticalSectionLocker { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSectionLocker {
    pub fn from(handle: *u8) -> @wxCriticalSectionLocker { @wxCriticalSectionLocker(handle) }
    pub fn null() -> @wxCriticalSectionLocker { wxCriticalSectionLocker::from(0 as *u8) }
    
}

trait _wxCriticalSectionLocker {
    fn handle(&self) -> *u8;
    
}

struct wxCursor(*u8);
impl _wxCursor for wxCursor {}
impl _wxBitmap for wxCursor {}
impl _wxGDIObject for wxCursor {}
impl _wxObject for wxCursor { fn handle(&self) -> *u8 { **self } }

impl wxCursor {
    pub fn from(handle: *u8) -> @wxCursor { @wxCursor(handle) }
    pub fn null() -> @wxCursor { wxCursor::from(0 as *u8) }
    
}

trait _wxCursor : _wxBitmap {
}

struct wxCustomDataObject(*u8);
impl _wxCustomDataObject for wxCustomDataObject {}
impl _wxDataObjectSimple for wxCustomDataObject {}
impl _wxDataObject for wxCustomDataObject { fn handle(&self) -> *u8 { **self } }

impl wxCustomDataObject {
    pub fn from(handle: *u8) -> @wxCustomDataObject { @wxCustomDataObject(handle) }
    pub fn null() -> @wxCustomDataObject { wxCustomDataObject::from(0 as *u8) }
    
}

trait _wxCustomDataObject : _wxDataObjectSimple {
}

struct wxDC(*u8);
impl _wxDC for wxDC {}
impl _wxObject for wxDC { fn handle(&self) -> *u8 { **self } }

impl wxDC {
    pub fn from(handle: *u8) -> @wxDC { @wxDC(handle) }
    pub fn null() -> @wxDC { wxDC::from(0 as *u8) }
    
}

trait _wxDC : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn blit<T: _wxDC>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: &T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: bool) -> bool {
        unsafe { wxDC_Blit(self.handle(), xdest, ydest, width, height, source.handle(), xsrc, ysrc, rop, useMask) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcBoundingBox(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CalcBoundingBox(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canDrawBitmap(&self) -> bool {
        unsafe { wxDC_CanDrawBitmap(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canGetTextExtent(&self) -> bool {
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
    fn drawBitmap<T: _wxBitmap>(&self, bmp: &T, x: c_int, y: c_int, useMask: bool) {
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
    fn drawLines(&self, n: c_int, x: *u8, y: *u8, xoffset: c_int, yoffset: c_int) {
        unsafe { wxDC_DrawLines(self.handle(), n, x, y, xoffset, yoffset) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPoint(&self, x: c_int, y: c_int) {
        unsafe { wxDC_DrawPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPolygon(&self, n: c_int, x: *u8, y: *u8, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolygon(self.handle(), n, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawPolyPolygon(&self, n: c_int, count: *u8, x: *u8, y: *u8, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
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
    fn getClippingBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxDC_GetClippingBox(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDepth(&self) -> c_int {
        unsafe { wxDC_GetDepth(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDeviceOrigin(&self, _x: *c_int, _y: *c_int) {
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
    fn getLogicalOrigin(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxDC_GetLogicalOrigin(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLogicalScale(&self, _x: *c_double, _y: *c_double) {
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
    fn getPixel<T: _wxColour>(&self, x: c_int, y: c_int, col: &T) -> bool {
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
    fn getTextExtent<T: _wxFont>(&self, string: &str, w: *u8, h: *u8, descent: *u8, externalLeading: *u8, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetTextExtent(self.handle(), string.handle(), w, h, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMultiLineTextExtent<T: _wxFont>(&self, string: &str, w: *u8, h: *u8, heightLine: *u8, theFont: &T) {
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
    fn getUserScale(&self, x: *c_double, y: *c_double) {
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
    fn isOk(&self) -> bool {
        unsafe { wxDC_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetBoundingBox(&self) {
        unsafe { wxDC_ResetBoundingBox(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAxisOrientation(&self, xLeftRight: bool, yBottomUp: bool) {
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
    fn startDoc(&self, msg: &str) -> bool {
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

struct wxDCClipper(*u8);
impl _wxDCClipper for wxDCClipper { fn handle(&self) -> *u8 { **self } }

impl wxDCClipper {
    pub fn from(handle: *u8) -> @wxDCClipper { @wxDCClipper(handle) }
    pub fn null() -> @wxDCClipper { wxDCClipper::from(0 as *u8) }
    
}

trait _wxDCClipper {
    fn handle(&self) -> *u8;
    
}

struct wxDDEClient(*u8);
impl _wxDDEClient for wxDDEClient {}
impl _wxClientBase for wxDDEClient {}
impl _wxObject for wxDDEClient { fn handle(&self) -> *u8 { **self } }

impl wxDDEClient {
    pub fn from(handle: *u8) -> @wxDDEClient { @wxDDEClient(handle) }
    pub fn null() -> @wxDDEClient { wxDDEClient::from(0 as *u8) }
    
}

trait _wxDDEClient : _wxClientBase {
}

struct wxDDEConnection(*u8);
impl _wxDDEConnection for wxDDEConnection {}
impl _wxConnectionBase for wxDDEConnection {}
impl _wxObject for wxDDEConnection { fn handle(&self) -> *u8 { **self } }

impl wxDDEConnection {
    pub fn from(handle: *u8) -> @wxDDEConnection { @wxDDEConnection(handle) }
    pub fn null() -> @wxDDEConnection { wxDDEConnection::from(0 as *u8) }
    
}

trait _wxDDEConnection : _wxConnectionBase {
}

struct wxDDEServer(*u8);
impl _wxDDEServer for wxDDEServer {}
impl _wxServerBase for wxDDEServer {}
impl _wxObject for wxDDEServer { fn handle(&self) -> *u8 { **self } }

impl wxDDEServer {
    pub fn from(handle: *u8) -> @wxDDEServer { @wxDDEServer(handle) }
    pub fn null() -> @wxDDEServer { wxDDEServer::from(0 as *u8) }
    
}

trait _wxDDEServer : _wxServerBase {
}

struct wxDataFormat(*u8);
impl _wxDataFormat for wxDataFormat { fn handle(&self) -> *u8 { **self } }

impl wxDataFormat {
    pub fn from(handle: *u8) -> @wxDataFormat { @wxDataFormat(handle) }
    pub fn null() -> @wxDataFormat { wxDataFormat::from(0 as *u8) }
    
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

trait _wxDataFormat {
    fn handle(&self) -> *u8;
    
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
    fn isEqual(&self, other: *u8) -> bool {
        unsafe { wxDataFormat_IsEqual(self.handle(), other) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setId(&self, id: *u8) {
        unsafe { wxDataFormat_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setType(&self, typ: c_int) {
        unsafe { wxDataFormat_SetType(self.handle(), typ) }
    }
}

struct wxDataInputStream(*u8);
impl _wxDataInputStream for wxDataInputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataInputStream {
    pub fn from(handle: *u8) -> @wxDataInputStream { @wxDataInputStream(handle) }
    pub fn null() -> @wxDataInputStream { wxDataInputStream::from(0 as *u8) }
    
}

trait _wxDataInputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDataObject(*u8);
impl _wxDataObject for wxDataObject { fn handle(&self) -> *u8 { **self } }

impl wxDataObject {
    pub fn from(handle: *u8) -> @wxDataObject { @wxDataObject(handle) }
    pub fn null() -> @wxDataObject { wxDataObject::from(0 as *u8) }
    
}

trait _wxDataObject {
    fn handle(&self) -> *u8;
    
}

struct wxDataObjectComposite(*u8);
impl _wxDataObjectComposite for wxDataObjectComposite {}
impl _wxDataObject for wxDataObjectComposite { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectComposite {
    pub fn from(handle: *u8) -> @wxDataObjectComposite { @wxDataObjectComposite(handle) }
    pub fn null() -> @wxDataObjectComposite { wxDataObjectComposite::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxDataObjectComposite {
        unsafe { @wxDataObjectComposite(wxDataObjectComposite_Create()) }
    }
}

trait _wxDataObjectComposite : _wxDataObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn add(&self, _dat: *u8, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.handle(), _dat, _preferred) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.handle()) }
    }
}

struct wxDataObjectSimple(*u8);
impl _wxDataObjectSimple for wxDataObjectSimple {}
impl _wxDataObject for wxDataObjectSimple { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectSimple {
    pub fn from(handle: *u8) -> @wxDataObjectSimple { @wxDataObjectSimple(handle) }
    pub fn null() -> @wxDataObjectSimple { wxDataObjectSimple::from(0 as *u8) }
    
}

trait _wxDataObjectSimple : _wxDataObject {
}

struct wxDataOutputStream(*u8);
impl _wxDataOutputStream for wxDataOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataOutputStream {
    pub fn from(handle: *u8) -> @wxDataOutputStream { @wxDataOutputStream(handle) }
    pub fn null() -> @wxDataOutputStream { wxDataOutputStream::from(0 as *u8) }
    
}

trait _wxDataOutputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDatabase(*u8);
impl _wxDatabase for wxDatabase {}
impl _wxObject for wxDatabase { fn handle(&self) -> *u8 { **self } }

impl wxDatabase {
    pub fn from(handle: *u8) -> @wxDatabase { @wxDatabase(handle) }
    pub fn null() -> @wxDatabase { wxDatabase::from(0 as *u8) }
    
}

trait _wxDatabase : _wxObject {
}

struct wxDateTime(*u8);
impl _wxDateTime for wxDateTime { fn handle(&self) -> *u8 { **self } }

impl wxDateTime {
    pub fn from(handle: *u8) -> @wxDateTime { @wxDateTime(handle) }
    pub fn null() -> @wxDateTime { wxDateTime::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxDateTime {
        unsafe { @wxDateTime(wxDateTime_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getAmString() -> ~str {
        unsafe { wxString { handle: wxDateTime_GetAmString() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getBeginDST<T: _wxDateTime>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetBeginDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCentury(year: c_int) -> c_int {
        unsafe { wxDateTime_GetCentury(year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCountry() -> c_int {
        unsafe { wxDateTime_GetCountry() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCurrentMonth(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentMonth(cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getCurrentYear(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentYear(cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getEndDST<T: _wxDateTime>(year: c_int, country: c_int, dt: &T) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getMonthName(month: c_int, flags: c_int) -> ~str {
        unsafe { wxString { handle: wxDateTime_GetMonthName(month, flags) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getPmString() -> ~str {
        unsafe { wxString { handle: wxDateTime_GetPmString() }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> ~str {
        unsafe { wxString { handle: wxDateTime_GetWeekDayName(weekday, flags) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isDSTApplicable(year: c_int, country: c_int) -> bool {
        unsafe { wxDateTime_IsDSTApplicable(year, country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isLeapYear(year: c_int, cal: c_int) -> bool {
        unsafe { wxDateTime_IsLeapYear(year, cal) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isWestEuropeanCountry(country: c_int) -> bool {
        unsafe { wxDateTime_IsWestEuropeanCountry(country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setCountry(country: c_int) {
        unsafe { wxDateTime_SetCountry(country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn wxDateTime(hi_long: c_int, lo_long: c_int) -> *u8 {
        unsafe { wxDateTime_wxDateTime(hi_long, lo_long) }
    }
}

trait _wxDateTime {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addDate<T: _wxDateTime>(&self, diff: *u8, _ref: &T) {
        unsafe { wxDateTime_AddDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.handle(), _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTime<T: _wxDateTime>(&self, diff: *u8, _ref: &T) {
        unsafe { wxDateTime_AddTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.handle(), _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn format(&self, format: *u8, tz: c_int) -> ~str {
        unsafe { wxString { handle: wxDateTime_Format(self.handle(), format, tz) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatDate(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatDate(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatISODate(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatISODate(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatISOTime(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatISOTime(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn formatTime(&self) -> ~str {
        unsafe { wxString { handle: wxDateTime_FormatTime(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDay(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDay(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDayOfYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDayOfYear(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHour(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetHour(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastMonthDay<T: _wxDateTime>(&self, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastMonthDay(self.handle(), month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastWeekDay<T: _wxDateTime>(&self, weekday: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetLastWeekDay(self.handle(), weekday, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMillisecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMillisecond(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinute(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMinute(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMonth(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMonth(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNextWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetNextWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrevWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetPrevWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetSecond(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTicks(&self) -> time_t {
        unsafe { wxDateTime_GetTicks(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self, hi_long: *u8, lo_long: *u8) {
        unsafe { wxDateTime_GetValue(self.handle(), hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDay<T: _wxDateTime>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDay(self.handle(), weekday, n, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDayInSameWeek<T: _wxDateTime>(&self, weekday: c_int, _ref: &T) {
        unsafe { wxDateTime_GetWeekDayInSameWeek(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekDayTZ(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekDayTZ(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekOfMonth(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfMonth(self.handle(), flags, tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWeekOfYear(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfYear(self.handle(), flags, tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetYear(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: &T, t2: &U) -> bool {
        unsafe { wxDateTime_IsBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isDST(&self, country: c_int) -> bool {
        unsafe { wxDateTime_IsDST(self.handle(), country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEarlierThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsEarlierThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqualTo(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsEqualTo(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqualUpTo<T: _wxDateTime>(&self, dt: &T, ts: *u8) -> bool {
        unsafe { wxDateTime_IsEqualUpTo(self.handle(), dt.handle(), ts) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isLaterThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsLaterThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSameDate<T: _wxDateTime>(&self, dt: &T) -> bool {
        unsafe { wxDateTime_IsSameDate(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSameTime<T: _wxDateTime>(&self, dt: &T) -> bool {
        unsafe { wxDateTime_IsSameTime(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isStrictlyBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: &T, t2: &U) -> bool {
        unsafe { wxDateTime_IsStrictlyBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isValid(&self) -> bool {
        unsafe { wxDateTime_IsValid(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWorkDay(&self, country: c_int) -> bool {
        unsafe { wxDateTime_IsWorkDay(self.handle(), country) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_MakeGMT(self.handle(), noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_MakeTimezone(self.handle(), tz, noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn now(&self) {
        unsafe { wxDateTime_Now(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseDate(&self, date: *u8) -> *u8 {
        unsafe { wxDateTime_ParseDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseDateTime(&self, datetime: *u8) -> *u8 {
        unsafe { wxDateTime_ParseDateTime(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseFormat(&self, date: *u8, format: *u8, dateDef: *u8) -> *u8 {
        unsafe { wxDateTime_ParseFormat(self.handle(), date, format, dateDef) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseRfc822Date(&self, date: *u8) -> *u8 {
        unsafe { wxDateTime_ParseRfc822Date(self.handle(), date) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn parseTime<T: _wxTime>(&self, time: &T) -> *u8 {
        unsafe { wxDateTime_ParseTime(self.handle(), time.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resetTime(&self) {
        unsafe { wxDateTime_ResetTime(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, day: c_int, month: c_int, year: c_int, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_Set(self.handle(), day, month, year, hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDay(&self, day: c_int) {
        unsafe { wxDateTime_SetDay(self.handle(), day) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHour(&self, hour: c_int) {
        unsafe { wxDateTime_SetHour(self.handle(), hour) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMillisecond(&self, millisecond: c_int) {
        unsafe { wxDateTime_SetMillisecond(self.handle(), millisecond) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinute(&self, minute: c_int) {
        unsafe { wxDateTime_SetMinute(self.handle(), minute) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMonth(&self, month: c_int) {
        unsafe { wxDateTime_SetMonth(self.handle(), month) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSecond(&self, second: c_int) {
        unsafe { wxDateTime_SetSecond(self.handle(), second) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTime(&self, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_SetTime(self.handle(), hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToCurrent(&self) {
        unsafe { wxDateTime_SetToCurrent(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToLastMonthDay(&self, month: c_int, year: c_int) {
        unsafe { wxDateTime_SetToLastMonthDay(self.handle(), month, year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int) -> bool {
        unsafe { wxDateTime_SetToLastWeekDay(self.handle(), weekday, month, year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToNextWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToNextWeekDay(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToPrevWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToPrevWeekDay(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int) -> bool {
        unsafe { wxDateTime_SetToWeekDay(self.handle(), weekday, n, month, year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToWeekDayInSameWeek(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToWeekDayInSameWeek(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setYear(&self, year: c_int) {
        unsafe { wxDateTime_SetYear(self.handle(), year) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractDate<T: _wxDateTime>(&self, diff: *u8, _ref: &T) {
        unsafe { wxDateTime_SubtractDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractTime<T: _wxDateTime>(&self, diff: *u8, _ref: &T) {
        unsafe { wxDateTime_SubtractTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_ToGMT(self.handle(), noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn toTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_ToTimezone(self.handle(), tz, noDST) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn today(&self) {
        unsafe { wxDateTime_Today(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn uNow(&self) {
        unsafe { wxDateTime_UNow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxDateTime_Delete(self.handle()) }
    }
}

struct wxDb(*u8);
impl _wxDb for wxDb { fn handle(&self) -> *u8 { **self } }

impl wxDb {
    pub fn from(handle: *u8) -> @wxDb { @wxDb(handle) }
    pub fn null() -> @wxDb { wxDb::from(0 as *u8) }
    
}

trait _wxDb {
    fn handle(&self) -> *u8;
    
}

struct wxDbColDef(*u8);
impl _wxDbColDef for wxDbColDef { fn handle(&self) -> *u8 { **self } }

impl wxDbColDef {
    pub fn from(handle: *u8) -> @wxDbColDef { @wxDbColDef(handle) }
    pub fn null() -> @wxDbColDef { wxDbColDef::from(0 as *u8) }
    
}

trait _wxDbColDef {
    fn handle(&self) -> *u8;
    
}

struct wxDbColFor(*u8);
impl _wxDbColFor for wxDbColFor { fn handle(&self) -> *u8 { **self } }

impl wxDbColFor {
    pub fn from(handle: *u8) -> @wxDbColFor { @wxDbColFor(handle) }
    pub fn null() -> @wxDbColFor { wxDbColFor::from(0 as *u8) }
    
}

trait _wxDbColFor {
    fn handle(&self) -> *u8;
    
}

struct wxDbColInf(*u8);
impl _wxDbColInf for wxDbColInf { fn handle(&self) -> *u8 { **self } }

impl wxDbColInf {
    pub fn from(handle: *u8) -> @wxDbColInf { @wxDbColInf(handle) }
    pub fn null() -> @wxDbColInf { wxDbColInf::from(0 as *u8) }
    
}

trait _wxDbColInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbConnectInf(*u8);
impl _wxDbConnectInf for wxDbConnectInf { fn handle(&self) -> *u8 { **self } }

impl wxDbConnectInf {
    pub fn from(handle: *u8) -> @wxDbConnectInf { @wxDbConnectInf(handle) }
    pub fn null() -> @wxDbConnectInf { wxDbConnectInf::from(0 as *u8) }
    
}

trait _wxDbConnectInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbInf(*u8);
impl _wxDbInf for wxDbInf { fn handle(&self) -> *u8 { **self } }

impl wxDbInf {
    pub fn from(handle: *u8) -> @wxDbInf { @wxDbInf(handle) }
    pub fn null() -> @wxDbInf { wxDbInf::from(0 as *u8) }
    
}

trait _wxDbInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbSqlTypeInfo(*u8);
impl _wxDbSqlTypeInfo for wxDbSqlTypeInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbSqlTypeInfo {
    pub fn from(handle: *u8) -> @wxDbSqlTypeInfo { @wxDbSqlTypeInfo(handle) }
    pub fn null() -> @wxDbSqlTypeInfo { wxDbSqlTypeInfo::from(0 as *u8) }
    
}

trait _wxDbSqlTypeInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDbTable(*u8);
impl _wxDbTable for wxDbTable { fn handle(&self) -> *u8 { **self } }

impl wxDbTable {
    pub fn from(handle: *u8) -> @wxDbTable { @wxDbTable(handle) }
    pub fn null() -> @wxDbTable { wxDbTable::from(0 as *u8) }
    
}

trait _wxDbTable {
    fn handle(&self) -> *u8;
    
}

struct wxDbTableInfo(*u8);
impl _wxDbTableInfo for wxDbTableInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbTableInfo {
    pub fn from(handle: *u8) -> @wxDbTableInfo { @wxDbTableInfo(handle) }
    pub fn null() -> @wxDbTableInfo { wxDbTableInfo::from(0 as *u8) }
    
}

trait _wxDbTableInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDebugContext(*u8);
impl _wxDebugContext for wxDebugContext { fn handle(&self) -> *u8 { **self } }

impl wxDebugContext {
    pub fn from(handle: *u8) -> @wxDebugContext { @wxDebugContext(handle) }
    pub fn null() -> @wxDebugContext { wxDebugContext::from(0 as *u8) }
    
}

trait _wxDebugContext {
    fn handle(&self) -> *u8;
    
}

struct wxDialUpEvent(*u8);
impl _wxDialUpEvent for wxDialUpEvent {}
impl _wxEvent for wxDialUpEvent {}
impl _wxObject for wxDialUpEvent { fn handle(&self) -> *u8 { **self } }

impl wxDialUpEvent {
    pub fn from(handle: *u8) -> @wxDialUpEvent { @wxDialUpEvent(handle) }
    pub fn null() -> @wxDialUpEvent { wxDialUpEvent::from(0 as *u8) }
    
}

trait _wxDialUpEvent : _wxEvent {
}

struct wxDialUpManager(*u8);
impl _wxDialUpManager for wxDialUpManager { fn handle(&self) -> *u8 { **self } }

impl wxDialUpManager {
    pub fn from(handle: *u8) -> @wxDialUpManager { @wxDialUpManager(handle) }
    pub fn null() -> @wxDialUpManager { wxDialUpManager::from(0 as *u8) }
    
}

trait _wxDialUpManager {
    fn handle(&self) -> *u8;
    
}

struct wxDialog(*u8);
impl _wxDialog for wxDialog {}
impl _wxTopLevelWindow for wxDialog {}
impl _wxWindow for wxDialog {}
impl _wxEvtHandler for wxDialog {}
impl _wxObject for wxDialog { fn handle(&self) -> *u8 { **self } }

impl wxDialog {
    pub fn from(handle: *u8) -> @wxDialog { @wxDialog(handle) }
    pub fn null() -> @wxDialog { wxDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDialog {
        let _txt = wxT(_txt);
        unsafe { @wxDialog(wxDialog_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxDialog : _wxTopLevelWindow {
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
    fn isModal(&self) -> bool {
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

struct wxDirDialog(*u8);
impl _wxDirDialog for wxDirDialog {}
impl _wxDialog for wxDirDialog {}
impl _wxTopLevelWindow for wxDirDialog {}
impl _wxWindow for wxDirDialog {}
impl _wxEvtHandler for wxDirDialog {}
impl _wxObject for wxDirDialog { fn handle(&self) -> *u8 { **self } }

impl wxDirDialog {
    pub fn from(handle: *u8) -> @wxDirDialog { @wxDirDialog(handle) }
    pub fn null() -> @wxDirDialog { wxDirDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _dir: &str, _lft: c_int, _top: c_int, _stl: c_int) -> @wxDirDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        unsafe { @wxDirDialog(wxDirDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _lft, _top, _stl)) }
    }
}

trait _wxDirDialog : _wxDialog {
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

struct wxDirTraverser(*u8);
impl _wxDirTraverser for wxDirTraverser { fn handle(&self) -> *u8 { **self } }

impl wxDirTraverser {
    pub fn from(handle: *u8) -> @wxDirTraverser { @wxDirTraverser(handle) }
    pub fn null() -> @wxDirTraverser { wxDirTraverser::from(0 as *u8) }
    
}

trait _wxDirTraverser {
    fn handle(&self) -> *u8;
    
}

struct wxDllLoader(*u8);
impl _wxDllLoader for wxDllLoader { fn handle(&self) -> *u8 { **self } }

impl wxDllLoader {
    pub fn from(handle: *u8) -> @wxDllLoader { @wxDllLoader(handle) }
    pub fn null() -> @wxDllLoader { wxDllLoader::from(0 as *u8) }
    
}

trait _wxDllLoader {
    fn handle(&self) -> *u8;
    
}

struct wxDocChildFrame(*u8);
impl _wxDocChildFrame for wxDocChildFrame {}
impl _wxFrame for wxDocChildFrame {}
impl _wxTopLevelWindow for wxDocChildFrame {}
impl _wxWindow for wxDocChildFrame {}
impl _wxEvtHandler for wxDocChildFrame {}
impl _wxObject for wxDocChildFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocChildFrame {
    pub fn from(handle: *u8) -> @wxDocChildFrame { @wxDocChildFrame(handle) }
    pub fn null() -> @wxDocChildFrame { wxDocChildFrame::from(0 as *u8) }
    
}

trait _wxDocChildFrame : _wxFrame {
}

struct wxDocMDIChildFrame(*u8);
impl _wxDocMDIChildFrame for wxDocMDIChildFrame {}
impl _wxMDIChildFrame for wxDocMDIChildFrame {}
impl _wxFrame for wxDocMDIChildFrame {}
impl _wxTopLevelWindow for wxDocMDIChildFrame {}
impl _wxWindow for wxDocMDIChildFrame {}
impl _wxEvtHandler for wxDocMDIChildFrame {}
impl _wxObject for wxDocMDIChildFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocMDIChildFrame {
    pub fn from(handle: *u8) -> @wxDocMDIChildFrame { @wxDocMDIChildFrame(handle) }
    pub fn null() -> @wxDocMDIChildFrame { wxDocMDIChildFrame::from(0 as *u8) }
    
}

trait _wxDocMDIChildFrame : _wxMDIChildFrame {
}

struct wxDocMDIParentFrame(*u8);
impl _wxDocMDIParentFrame for wxDocMDIParentFrame {}
impl _wxMDIParentFrame for wxDocMDIParentFrame {}
impl _wxFrame for wxDocMDIParentFrame {}
impl _wxTopLevelWindow for wxDocMDIParentFrame {}
impl _wxWindow for wxDocMDIParentFrame {}
impl _wxEvtHandler for wxDocMDIParentFrame {}
impl _wxObject for wxDocMDIParentFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocMDIParentFrame {
    pub fn from(handle: *u8) -> @wxDocMDIParentFrame { @wxDocMDIParentFrame(handle) }
    pub fn null() -> @wxDocMDIParentFrame { wxDocMDIParentFrame::from(0 as *u8) }
    
}

trait _wxDocMDIParentFrame : _wxMDIParentFrame {
}

struct wxDocManager(*u8);
impl _wxDocManager for wxDocManager {}
impl _wxEvtHandler for wxDocManager {}
impl _wxObject for wxDocManager { fn handle(&self) -> *u8 { **self } }

impl wxDocManager {
    pub fn from(handle: *u8) -> @wxDocManager { @wxDocManager(handle) }
    pub fn null() -> @wxDocManager { wxDocManager::from(0 as *u8) }
    
}

trait _wxDocManager : _wxEvtHandler {
}

struct wxDocParentFrame(*u8);
impl _wxDocParentFrame for wxDocParentFrame {}
impl _wxFrame for wxDocParentFrame {}
impl _wxTopLevelWindow for wxDocParentFrame {}
impl _wxWindow for wxDocParentFrame {}
impl _wxEvtHandler for wxDocParentFrame {}
impl _wxObject for wxDocParentFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocParentFrame {
    pub fn from(handle: *u8) -> @wxDocParentFrame { @wxDocParentFrame(handle) }
    pub fn null() -> @wxDocParentFrame { wxDocParentFrame::from(0 as *u8) }
    
}

trait _wxDocParentFrame : _wxFrame {
}

struct wxDocTemplate(*u8);
impl _wxDocTemplate for wxDocTemplate {}
impl _wxObject for wxDocTemplate { fn handle(&self) -> *u8 { **self } }

impl wxDocTemplate {
    pub fn from(handle: *u8) -> @wxDocTemplate { @wxDocTemplate(handle) }
    pub fn null() -> @wxDocTemplate { wxDocTemplate::from(0 as *u8) }
    
}

trait _wxDocTemplate : _wxObject {
}

struct wxDocument(*u8);
impl _wxDocument for wxDocument {}
impl _wxEvtHandler for wxDocument {}
impl _wxObject for wxDocument { fn handle(&self) -> *u8 { **self } }

impl wxDocument {
    pub fn from(handle: *u8) -> @wxDocument { @wxDocument(handle) }
    pub fn null() -> @wxDocument { wxDocument::from(0 as *u8) }
    
}

trait _wxDocument : _wxEvtHandler {
}

struct wxDragImage(*u8);
impl _wxDragImage for wxDragImage {}
impl _wxObject for wxDragImage { fn handle(&self) -> *u8 { **self } }

impl wxDragImage {
    pub fn from(handle: *u8) -> @wxDragImage { @wxDragImage(handle) }
    pub fn null() -> @wxDragImage { wxDragImage::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxBitmap>(image: &T, x: c_int, y: c_int) -> @wxDragImage {
        unsafe { @wxDragImage(wxDragImage_Create(image.handle(), x, y)) }
    }
}

trait _wxDragImage : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginDragFullScreen<T: _wxWindow, U: _wxRect>(&self, x_pos: c_int, y_pos: c_int, window: &T, fullScreen: bool, rect: &U) -> bool {
        unsafe { wxDragImage_BeginDragFullScreen(self.handle(), x_pos, y_pos, window.handle(), fullScreen, rect.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginDrag<T: _wxWindow, U: _wxWindow>(&self, x: c_int, y: c_int, window: &T, boundingWindow: &U) -> bool {
        unsafe { wxDragImage_BeginDrag(self.handle(), x, y, window.handle(), boundingWindow.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDrag(&self) {
        unsafe { wxDragImage_EndDrag(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) -> bool {
        unsafe { wxDragImage_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn move(&self, x: c_int, y: c_int) -> bool {
        unsafe { wxDragImage_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show(&self) -> bool {
        unsafe { wxDragImage_Show(self.handle()) }
    }
}

struct wxDrawControl(*u8);
impl _wxDrawControl for wxDrawControl {}
impl _wxControl for wxDrawControl {}
impl _wxWindow for wxDrawControl {}
impl _wxEvtHandler for wxDrawControl {}
impl _wxObject for wxDrawControl { fn handle(&self) -> *u8 { **self } }

impl wxDrawControl {
    pub fn from(handle: *u8) -> @wxDrawControl { @wxDrawControl(handle) }
    pub fn null() -> @wxDrawControl { wxDrawControl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDrawControl {
        unsafe { @wxDrawControl(wxDrawControl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxDrawControl : _wxControl {
}

struct wxDrawWindow(*u8);
impl _wxDrawWindow for wxDrawWindow {}
impl _wxWindow for wxDrawWindow {}
impl _wxEvtHandler for wxDrawWindow {}
impl _wxObject for wxDrawWindow { fn handle(&self) -> *u8 { **self } }

impl wxDrawWindow {
    pub fn from(handle: *u8) -> @wxDrawWindow { @wxDrawWindow(handle) }
    pub fn null() -> @wxDrawWindow { wxDrawWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDrawWindow {
        unsafe { @wxDrawWindow(wxDrawWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxDrawWindow : _wxWindow {
}

struct wxDropFilesEvent(*u8);
impl _wxDropFilesEvent for wxDropFilesEvent {}
impl _wxEvent for wxDropFilesEvent {}
impl _wxObject for wxDropFilesEvent { fn handle(&self) -> *u8 { **self } }

impl wxDropFilesEvent {
    pub fn from(handle: *u8) -> @wxDropFilesEvent { @wxDropFilesEvent(handle) }
    pub fn null() -> @wxDropFilesEvent { wxDropFilesEvent::from(0 as *u8) }
    
}

trait _wxDropFilesEvent : _wxEvent {
}

struct wxDropSource(*u8);
impl _wxDropSource for wxDropSource { fn handle(&self) -> *u8 { **self } }

impl wxDropSource {
    pub fn from(handle: *u8) -> @wxDropSource { @wxDropSource(handle) }
    pub fn null() -> @wxDropSource { wxDropSource::from(0 as *u8) }
    
}

trait _wxDropSource {
    fn handle(&self) -> *u8;
    
}

struct wxDropTarget(*u8);
impl _wxDropTarget for wxDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxDropTarget {
    pub fn from(handle: *u8) -> @wxDropTarget { @wxDropTarget(handle) }
    pub fn null() -> @wxDropTarget { wxDropTarget::from(0 as *u8) }
    
}

trait _wxDropTarget {
    fn handle(&self) -> *u8;
    
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

struct wxDynToolInfo(*u8);
impl _wxDynToolInfo for wxDynToolInfo {}
impl _wxToolLayoutItem for wxDynToolInfo {}
impl _wxObject for wxDynToolInfo { fn handle(&self) -> *u8 { **self } }

impl wxDynToolInfo {
    pub fn from(handle: *u8) -> @wxDynToolInfo { @wxDynToolInfo(handle) }
    pub fn null() -> @wxDynToolInfo { wxDynToolInfo::from(0 as *u8) }
    
}

trait _wxDynToolInfo : _wxToolLayoutItem {
}

struct wxDynamicLibrary(*u8);
impl _wxDynamicLibrary for wxDynamicLibrary { fn handle(&self) -> *u8 { **self } }

impl wxDynamicLibrary {
    pub fn from(handle: *u8) -> @wxDynamicLibrary { @wxDynamicLibrary(handle) }
    pub fn null() -> @wxDynamicLibrary { wxDynamicLibrary::from(0 as *u8) }
    
}

trait _wxDynamicLibrary {
    fn handle(&self) -> *u8;
    
}

struct wxDynamicSashWindow(*u8);
impl _wxDynamicSashWindow for wxDynamicSashWindow {}
impl _wxWindow for wxDynamicSashWindow {}
impl _wxEvtHandler for wxDynamicSashWindow {}
impl _wxObject for wxDynamicSashWindow { fn handle(&self) -> *u8 { **self } }

impl wxDynamicSashWindow {
    pub fn from(handle: *u8) -> @wxDynamicSashWindow { @wxDynamicSashWindow(handle) }
    pub fn null() -> @wxDynamicSashWindow { wxDynamicSashWindow::from(0 as *u8) }
    
}

trait _wxDynamicSashWindow : _wxWindow {
}

struct wxDynamicToolBar(*u8);
impl _wxDynamicToolBar for wxDynamicToolBar {}
impl _wxToolBarBase for wxDynamicToolBar {}
impl _wxControl for wxDynamicToolBar {}
impl _wxWindow for wxDynamicToolBar {}
impl _wxEvtHandler for wxDynamicToolBar {}
impl _wxObject for wxDynamicToolBar { fn handle(&self) -> *u8 { **self } }

impl wxDynamicToolBar {
    pub fn from(handle: *u8) -> @wxDynamicToolBar { @wxDynamicToolBar(handle) }
    pub fn null() -> @wxDynamicToolBar { wxDynamicToolBar::from(0 as *u8) }
    
}

trait _wxDynamicToolBar : _wxToolBarBase {
}

struct wxEditableListBox(*u8);
impl _wxEditableListBox for wxEditableListBox {}
impl _wxPanel for wxEditableListBox {}
impl _wxWindow for wxEditableListBox {}
impl _wxEvtHandler for wxEditableListBox {}
impl _wxObject for wxEditableListBox { fn handle(&self) -> *u8 { **self } }

impl wxEditableListBox {
    pub fn from(handle: *u8) -> @wxEditableListBox { @wxEditableListBox(handle) }
    pub fn null() -> @wxEditableListBox { wxEditableListBox::from(0 as *u8) }
    
}

trait _wxEditableListBox : _wxPanel {
}

struct wxEncodingConverter(*u8);
impl _wxEncodingConverter for wxEncodingConverter {}
impl _wxObject for wxEncodingConverter { fn handle(&self) -> *u8 { **self } }

impl wxEncodingConverter {
    pub fn from(handle: *u8) -> @wxEncodingConverter { @wxEncodingConverter(handle) }
    pub fn null() -> @wxEncodingConverter { wxEncodingConverter::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxEncodingConverter {
        unsafe { @wxEncodingConverter(wxEncodingConverter_Create()) }
    }
}

trait _wxEncodingConverter : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn convert(&self, input: *u8, output: *u8) {
        unsafe { wxEncodingConverter_Convert(self.handle(), input, output) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllEquivalents<T: _wxList>(&self, enc: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.handle(), enc, _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPlatformEquivalents<T: _wxList>(&self, enc: c_int, platform: c_int, _lst: &T) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.handle(), enc, platform, _lst.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.handle(), input_enc, output_enc, method) }
    }
}

struct wxEraseEvent(*u8);
impl _wxEraseEvent for wxEraseEvent {}
impl _wxEvent for wxEraseEvent {}
impl _wxObject for wxEraseEvent { fn handle(&self) -> *u8 { **self } }

impl wxEraseEvent {
    pub fn from(handle: *u8) -> @wxEraseEvent { @wxEraseEvent(handle) }
    pub fn null() -> @wxEraseEvent { wxEraseEvent::from(0 as *u8) }
    
}

trait _wxEraseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDC(&self) -> @wxDC {
        unsafe { @wxDC(wxEraseEvent_GetDC(self.handle())) }
    }
}

struct wxEvent(*u8);
impl _wxEvent for wxEvent {}
impl _wxObject for wxEvent { fn handle(&self) -> *u8 { **self } }

impl wxEvent {
    pub fn from(handle: *u8) -> @wxEvent { @wxEvent(handle) }
    pub fn null() -> @wxEvent { wxEvent::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

trait _wxEvent : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn copyObject(&self, object_dest: *u8) {
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
    fn getSkipped(&self) -> bool {
        unsafe { wxEvent_GetSkipped(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTimestamp(&self) -> c_int {
        unsafe { wxEvent_GetTimestamp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCommandEvent(&self) -> bool {
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

struct wxEvtHandler(*u8);
impl _wxEvtHandler for wxEvtHandler {}
impl _wxObject for wxEvtHandler { fn handle(&self) -> *u8 { **self } }

impl wxEvtHandler {
    pub fn from(handle: *u8) -> @wxEvtHandler { @wxEvtHandler(handle) }
    pub fn null() -> @wxEvtHandler { wxEvtHandler::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_Create()) }
    }
}

trait _wxEvtHandler : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPendingEvent<T: _wxEvent>(&self, event: &T) {
        unsafe { wxEvtHandler_AddPendingEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn connect(&self, first: c_int, last: c_int, type_: c_int, data: *u8) -> c_int {
        unsafe { wxEvtHandler_Connect(self.handle(), first, last, type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disconnect(&self, first: c_int, last: c_int, type_: c_int, id: c_int) -> c_int {
        unsafe { wxEvtHandler_Disconnect(self.handle(), first, last, type_, id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEvtHandlerEnabled(&self) -> bool {
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
    fn processEvent<T: _wxEvent>(&self, event: &T) -> bool {
        unsafe { wxEvtHandler_ProcessEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEvtHandlerEnabled(&self, enabled: bool) {
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

struct wxExpr(*u8);
impl _wxExpr for wxExpr { fn handle(&self) -> *u8 { **self } }

impl wxExpr {
    pub fn from(handle: *u8) -> @wxExpr { @wxExpr(handle) }
    pub fn null() -> @wxExpr { wxExpr::from(0 as *u8) }
    
}

trait _wxExpr {
    fn handle(&self) -> *u8;
    
}

struct wxExprDatabase(*u8);
impl _wxExprDatabase for wxExprDatabase {}
impl _wxList for wxExprDatabase {}
impl _wxObject for wxExprDatabase { fn handle(&self) -> *u8 { **self } }

impl wxExprDatabase {
    pub fn from(handle: *u8) -> @wxExprDatabase { @wxExprDatabase(handle) }
    pub fn null() -> @wxExprDatabase { wxExprDatabase::from(0 as *u8) }
    
}

trait _wxExprDatabase : _wxList {
}

struct wxFFile(*u8);
impl _wxFFile for wxFFile { fn handle(&self) -> *u8 { **self } }

impl wxFFile {
    pub fn from(handle: *u8) -> @wxFFile { @wxFFile(handle) }
    pub fn null() -> @wxFFile { wxFFile::from(0 as *u8) }
    
}

trait _wxFFile {
    fn handle(&self) -> *u8;
    
}

struct wxFFileInputStream(*u8);
impl _wxFFileInputStream for wxFFileInputStream {}
impl _wxInputStream for wxFFileInputStream {}
impl _wxStreamBase for wxFFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileInputStream {
    pub fn from(handle: *u8) -> @wxFFileInputStream { @wxFFileInputStream(handle) }
    pub fn null() -> @wxFFileInputStream { wxFFileInputStream::from(0 as *u8) }
    
}

trait _wxFFileInputStream : _wxInputStream {
}

struct wxFFileOutputStream(*u8);
impl _wxFFileOutputStream for wxFFileOutputStream {}
impl _wxOutputStream for wxFFileOutputStream {}
impl _wxStreamBase for wxFFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileOutputStream {
    pub fn from(handle: *u8) -> @wxFFileOutputStream { @wxFFileOutputStream(handle) }
    pub fn null() -> @wxFFileOutputStream { wxFFileOutputStream::from(0 as *u8) }
    
}

trait _wxFFileOutputStream : _wxOutputStream {
}

struct wxFSFile(*u8);
impl _wxFSFile for wxFSFile {}
impl _wxObject for wxFSFile { fn handle(&self) -> *u8 { **self } }

impl wxFSFile {
    pub fn from(handle: *u8) -> @wxFSFile { @wxFSFile(handle) }
    pub fn null() -> @wxFSFile { wxFSFile::from(0 as *u8) }
    
}

trait _wxFSFile : _wxObject {
}

struct wxFTP(*u8);
impl _wxFTP for wxFTP {}
impl _wxProtocol for wxFTP {}
impl _wxSocketClient for wxFTP {}
impl _wxSocketBase for wxFTP {}
impl _wxObject for wxFTP { fn handle(&self) -> *u8 { **self } }

impl wxFTP {
    pub fn from(handle: *u8) -> @wxFTP { @wxFTP(handle) }
    pub fn null() -> @wxFTP { wxFTP::from(0 as *u8) }
    
}

trait _wxFTP : _wxProtocol {
}

struct wxFileDataObject(*u8);
impl _wxFileDataObject for wxFileDataObject {}
impl _wxDataObjectSimple for wxFileDataObject {}
impl _wxDataObject for wxFileDataObject { fn handle(&self) -> *u8 { **self } }

impl wxFileDataObject {
    pub fn from(handle: *u8) -> @wxFileDataObject { @wxFileDataObject(handle) }
    pub fn null() -> @wxFileDataObject { wxFileDataObject::from(0 as *u8) }
    
}

trait _wxFileDataObject : _wxDataObjectSimple {
}

struct wxFileDialog(*u8);
impl _wxFileDialog for wxFileDialog {}
impl _wxDialog for wxFileDialog {}
impl _wxTopLevelWindow for wxFileDialog {}
impl _wxWindow for wxFileDialog {}
impl _wxEvtHandler for wxFileDialog {}
impl _wxObject for wxFileDialog { fn handle(&self) -> *u8 { **self } }

impl wxFileDialog {
    pub fn from(handle: *u8) -> @wxFileDialog { @wxFileDialog(handle) }
    pub fn null() -> @wxFileDialog { wxFileDialog::from(0 as *u8) }
    
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

trait _wxFileDialog : _wxDialog {
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
    fn getFilenames(&self, paths: *wchar_t) -> c_int {
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
    fn getPaths(&self, paths: *wchar_t) -> c_int {
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

struct wxFileDropTarget(*u8);
impl _wxFileDropTarget for wxFileDropTarget {}
impl _wxDropTarget for wxFileDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxFileDropTarget {
    pub fn from(handle: *u8) -> @wxFileDropTarget { @wxFileDropTarget(handle) }
    pub fn null() -> @wxFileDropTarget { wxFileDropTarget::from(0 as *u8) }
    
}

trait _wxFileDropTarget : _wxDropTarget {
}

struct wxFileHistory(*u8);
impl _wxFileHistory for wxFileHistory {}
impl _wxObject for wxFileHistory { fn handle(&self) -> *u8 { **self } }

impl wxFileHistory {
    pub fn from(handle: *u8) -> @wxFileHistory { @wxFileHistory(handle) }
    pub fn null() -> @wxFileHistory { wxFileHistory::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(maxFiles: c_int) -> @wxFileHistory {
        unsafe { @wxFileHistory(wxFileHistory_Create(maxFiles)) }
    }
}

trait _wxFileHistory : _wxObject {
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
    fn getMenus(&self, _ref: *u8) -> c_int {
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

struct wxFileInputStream(*u8);
impl _wxFileInputStream for wxFileInputStream {}
impl _wxInputStream for wxFileInputStream {}
impl _wxStreamBase for wxFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileInputStream {
    pub fn from(handle: *u8) -> @wxFileInputStream { @wxFileInputStream(handle) }
    pub fn null() -> @wxFileInputStream { wxFileInputStream::from(0 as *u8) }
    
}

trait _wxFileInputStream : _wxInputStream {
}

struct wxFileName(*u8);
impl _wxFileName for wxFileName { fn handle(&self) -> *u8 { **self } }

impl wxFileName {
    pub fn from(handle: *u8) -> @wxFileName { @wxFileName(handle) }
    pub fn null() -> @wxFileName { wxFileName::from(0 as *u8) }
    
}

trait _wxFileName {
    fn handle(&self) -> *u8;
    
}

struct wxFileOutputStream(*u8);
impl _wxFileOutputStream for wxFileOutputStream {}
impl _wxOutputStream for wxFileOutputStream {}
impl _wxStreamBase for wxFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileOutputStream {
    pub fn from(handle: *u8) -> @wxFileOutputStream { @wxFileOutputStream(handle) }
    pub fn null() -> @wxFileOutputStream { wxFileOutputStream::from(0 as *u8) }
    
}

trait _wxFileOutputStream : _wxOutputStream {
}

struct wxFileSystem(*u8);
impl _wxFileSystem for wxFileSystem {}
impl _wxObject for wxFileSystem { fn handle(&self) -> *u8 { **self } }

impl wxFileSystem {
    pub fn from(handle: *u8) -> @wxFileSystem { @wxFileSystem(handle) }
    pub fn null() -> @wxFileSystem { wxFileSystem::from(0 as *u8) }
    
}

trait _wxFileSystem : _wxObject {
}

struct wxFileSystemHandler(*u8);
impl _wxFileSystemHandler for wxFileSystemHandler {}
impl _wxObject for wxFileSystemHandler { fn handle(&self) -> *u8 { **self } }

impl wxFileSystemHandler {
    pub fn from(handle: *u8) -> @wxFileSystemHandler { @wxFileSystemHandler(handle) }
    pub fn null() -> @wxFileSystemHandler { wxFileSystemHandler::from(0 as *u8) }
    
}

trait _wxFileSystemHandler : _wxObject {
}

struct wxFileType(*u8);
impl _wxFileType for wxFileType { fn handle(&self) -> *u8 { **self } }

impl wxFileType {
    pub fn from(handle: *u8) -> @wxFileType { @wxFileType(handle) }
    pub fn null() -> @wxFileType { wxFileType::from(0 as *u8) }
    
}

trait _wxFileType {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn expandCommand(&self, _cmd: *u8, _params: *u8) -> ~str {
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
    fn getOpenCommand(&self, _buf: *u8, _params: *u8) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.handle(), _buf, _params) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrintCommand(&self, _buf: *u8, _params: *u8) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.handle(), _buf, _params) }
    }
}

struct wxFilterInputStream(*u8);
impl _wxFilterInputStream for wxFilterInputStream {}
impl _wxInputStream for wxFilterInputStream {}
impl _wxStreamBase for wxFilterInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterInputStream {
    pub fn from(handle: *u8) -> @wxFilterInputStream { @wxFilterInputStream(handle) }
    pub fn null() -> @wxFilterInputStream { wxFilterInputStream::from(0 as *u8) }
    
}

trait _wxFilterInputStream : _wxInputStream {
}

struct wxFilterOutputStream(*u8);
impl _wxFilterOutputStream for wxFilterOutputStream {}
impl _wxOutputStream for wxFilterOutputStream {}
impl _wxStreamBase for wxFilterOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterOutputStream {
    pub fn from(handle: *u8) -> @wxFilterOutputStream { @wxFilterOutputStream(handle) }
    pub fn null() -> @wxFilterOutputStream { wxFilterOutputStream::from(0 as *u8) }
    
}

trait _wxFilterOutputStream : _wxOutputStream {
}

struct wxFindDialogEvent(*u8);
impl _wxFindDialogEvent for wxFindDialogEvent {}
impl _wxCommandEvent for wxFindDialogEvent {}
impl _wxEvent for wxFindDialogEvent {}
impl _wxObject for wxFindDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxFindDialogEvent {
    pub fn from(handle: *u8) -> @wxFindDialogEvent { @wxFindDialogEvent(handle) }
    pub fn null() -> @wxFindDialogEvent { wxFindDialogEvent::from(0 as *u8) }
    
}

trait _wxFindDialogEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFindString(&self, _ref: *u8) -> c_int {
        unsafe { wxFindDialogEvent_GetFindString(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindDialogEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getReplaceString(&self, _ref: *u8) -> c_int {
        unsafe { wxFindDialogEvent_GetReplaceString(self.handle(), _ref) }
    }
}

struct wxFindReplaceData(*u8);
impl _wxFindReplaceData for wxFindReplaceData {}
impl _wxObject for wxFindReplaceData { fn handle(&self) -> *u8 { **self } }

impl wxFindReplaceData {
    pub fn from(handle: *u8) -> @wxFindReplaceData { @wxFindReplaceData(handle) }
    pub fn null() -> @wxFindReplaceData { wxFindReplaceData::from(0 as *u8) }
    
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

trait _wxFindReplaceData : _wxObject {
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

struct wxFindReplaceDialog(*u8);
impl _wxFindReplaceDialog for wxFindReplaceDialog {}
impl _wxDialog for wxFindReplaceDialog {}
impl _wxTopLevelWindow for wxFindReplaceDialog {}
impl _wxWindow for wxFindReplaceDialog {}
impl _wxEvtHandler for wxFindReplaceDialog {}
impl _wxObject for wxFindReplaceDialog { fn handle(&self) -> *u8 { **self } }

impl wxFindReplaceDialog {
    pub fn from(handle: *u8) -> @wxFindReplaceDialog { @wxFindReplaceDialog(handle) }
    pub fn null() -> @wxFindReplaceDialog { wxFindReplaceDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxFindReplaceData>(parent: &T, data: &U, title: &str, style: c_int) -> @wxFindReplaceDialog {
        let title = wxT(title);
        unsafe { @wxFindReplaceDialog(wxFindReplaceDialog_Create(parent.handle(), data.handle(), title.handle(), style)) }
    }
}

trait _wxFindReplaceDialog : _wxDialog {
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

struct wxFlexGridSizer(*u8);
impl _wxFlexGridSizer for wxFlexGridSizer {}
impl _wxGridSizer for wxFlexGridSizer {}
impl _wxSizer for wxFlexGridSizer {}
impl _wxObject for wxFlexGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxFlexGridSizer {
    pub fn from(handle: *u8) -> @wxFlexGridSizer { @wxFlexGridSizer(handle) }
    pub fn null() -> @wxFlexGridSizer { wxFlexGridSizer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @wxFlexGridSizer {
        unsafe { @wxFlexGridSizer(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

trait _wxFlexGridSizer : _wxGridSizer {
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

struct wxFocusEvent(*u8);
impl _wxFocusEvent for wxFocusEvent {}
impl _wxEvent for wxFocusEvent {}
impl _wxObject for wxFocusEvent { fn handle(&self) -> *u8 { **self } }

impl wxFocusEvent {
    pub fn from(handle: *u8) -> @wxFocusEvent { @wxFocusEvent(handle) }
    pub fn null() -> @wxFocusEvent { wxFocusEvent::from(0 as *u8) }
    
}

trait _wxFocusEvent : _wxEvent {
}

struct wxFont(*u8);
impl _wxFont for wxFont {}
impl _wxGDIObject for wxFont {}
impl _wxObject for wxFont { fn handle(&self) -> *u8 { **self } }

impl wxFont {
    pub fn from(handle: *u8) -> @wxFont { @wxFont(handle) }
    pub fn null() -> @wxFont { wxFont::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: bool, face: &str, enc: c_int) -> @wxFont {
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

trait _wxFont : _wxGDIObject {
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
    fn isOk(&self) -> bool {
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
    fn isStatic(&self) -> bool {
        unsafe { wxFont_IsStatic(self.handle()) }
    }
}

struct wxFontData(*u8);
impl _wxFontData for wxFontData {}
impl _wxObject for wxFontData { fn handle(&self) -> *u8 { **self } }

impl wxFontData {
    pub fn from(handle: *u8) -> @wxFontData { @wxFontData(handle) }
    pub fn null() -> @wxFontData { wxFontData::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxFontData {
        unsafe { @wxFontData(wxFontData_Create()) }
    }
}

trait _wxFontData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableEffects(&self, flag: bool) {
        unsafe { wxFontData_EnableEffects(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllowSymbols(&self) -> bool {
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
    fn getEnableEffects(&self) -> bool {
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
    fn setAllowSymbols(&self, flag: bool) {
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
    fn setShowHelp(&self, flag: bool) {
        unsafe { wxFontData_SetShowHelp(self.handle(), flag) }
    }
}

struct wxFontDialog(*u8);
impl _wxFontDialog for wxFontDialog {}
impl _wxDialog for wxFontDialog {}
impl _wxTopLevelWindow for wxFontDialog {}
impl _wxWindow for wxFontDialog {}
impl _wxEvtHandler for wxFontDialog {}
impl _wxObject for wxFontDialog { fn handle(&self) -> *u8 { **self } }

impl wxFontDialog {
    pub fn from(handle: *u8) -> @wxFontDialog { @wxFontDialog(handle) }
    pub fn null() -> @wxFontDialog { wxFontDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxFontData>(_prt: &T, fnt: &U) -> @wxFontDialog {
        unsafe { @wxFontDialog(wxFontDialog_Create(_prt.handle(), fnt.handle())) }
    }
}

trait _wxFontDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFontData<T: _wxFontData>(&self, _ref: &T) {
        unsafe { wxFontDialog_GetFontData(self.handle(), _ref.handle()) }
    }
}

struct wxFontEnumerator(*u8);
impl _wxFontEnumerator for wxFontEnumerator { fn handle(&self) -> *u8 { **self } }

impl wxFontEnumerator {
    pub fn from(handle: *u8) -> @wxFontEnumerator { @wxFontEnumerator(handle) }
    pub fn null() -> @wxFontEnumerator { wxFontEnumerator::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, _fnc: *u8) -> @wxFontEnumerator {
        unsafe { @wxFontEnumerator(wxFontEnumerator_Create(_obj, _fnc)) }
    }
}

trait _wxFontEnumerator {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumerateEncodings(&self, facename: &str) -> bool {
        let facename = wxT(facename);
        unsafe { wxFontEnumerator_EnumerateEncodings(self.handle(), facename.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> bool {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.handle(), encoding, fixedWidthOnly) }
    }
}

struct wxFontList(*u8);
impl _wxFontList for wxFontList {}
impl _wxList for wxFontList {}
impl _wxObject for wxFontList { fn handle(&self) -> *u8 { **self } }

impl wxFontList {
    pub fn from(handle: *u8) -> @wxFontList { @wxFontList(handle) }
    pub fn null() -> @wxFontList { wxFontList::from(0 as *u8) }
    
}

trait _wxFontList : _wxList {
}

struct wxFontMapper(*u8);
impl _wxFontMapper for wxFontMapper { fn handle(&self) -> *u8 { **self } }

impl wxFontMapper {
    pub fn from(handle: *u8) -> @wxFontMapper { @wxFontMapper(handle) }
    pub fn null() -> @wxFontMapper { wxFontMapper::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxFontMapper {
        unsafe { @wxFontMapper(wxFontMapper_Create()) }
    }
}

trait _wxFontMapper {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *u8, _buf: &str) -> bool {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_GetAltForEncoding(self.handle(), encoding, alt_encoding, _buf.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEncodingAvailable(&self, encoding: c_int, _buf: &str) -> bool {
        let _buf = wxT(_buf);
        unsafe { wxFontMapper_IsEncodingAvailable(self.handle(), encoding, _buf.handle()) }
    }
}

struct wxFrame(*u8);
impl _wxFrame for wxFrame {}
impl _wxTopLevelWindow for wxFrame {}
impl _wxWindow for wxFrame {}
impl _wxEvtHandler for wxFrame {}
impl _wxObject for wxFrame { fn handle(&self) -> *u8 { **self } }

impl wxFrame {
    pub fn from(handle: *u8) -> @wxFrame { @wxFrame(handle) }
    pub fn null() -> @wxFrame { wxFrame::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxFrame {
        let _txt = wxT(_txt);
        unsafe { @wxFrame(wxFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxFrame : _wxTopLevelWindow {
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
    fn setStatusWidths(&self, _n: c_int, _widths_field: *u8) {
        unsafe { wxFrame_SetStatusWidths(self.handle(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToolBar<T: _wxToolBar>(&self, _toolbar: &T) {
        unsafe { wxFrame_SetToolBar(self.handle(), _toolbar.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShape<T: _wxRegion>(&self, region: &T) -> bool {
        unsafe { wxFrame_SetShape(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showFullScreen(&self, show: bool, style: c_int) -> bool {
        unsafe { wxFrame_ShowFullScreen(self.handle(), show, style) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isFullScreen(&self) -> bool {
        unsafe { wxFrame_IsFullScreen(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centre(&self, orientation: c_int) {
        unsafe { wxFrame_Centre(self.handle(), orientation) }
    }
}

struct wxFrameLayout(*u8);
impl _wxFrameLayout for wxFrameLayout {}
impl _wxEvtHandler for wxFrameLayout {}
impl _wxObject for wxFrameLayout { fn handle(&self) -> *u8 { **self } }

impl wxFrameLayout {
    pub fn from(handle: *u8) -> @wxFrameLayout { @wxFrameLayout(handle) }
    pub fn null() -> @wxFrameLayout { wxFrameLayout::from(0 as *u8) }
    
}

trait _wxFrameLayout : _wxEvtHandler {
}

struct wxGDIObject(*u8);
impl _wxGDIObject for wxGDIObject {}
impl _wxObject for wxGDIObject { fn handle(&self) -> *u8 { **self } }

impl wxGDIObject {
    pub fn from(handle: *u8) -> @wxGDIObject { @wxGDIObject(handle) }
    pub fn null() -> @wxGDIObject { wxGDIObject::from(0 as *u8) }
    
}

trait _wxGDIObject : _wxObject {
}

struct wxGLCanvas(*u8);
impl _wxGLCanvas for wxGLCanvas {}
impl _wxScrolledWindow for wxGLCanvas {}
impl _wxPanel for wxGLCanvas {}
impl _wxWindow for wxGLCanvas {}
impl _wxEvtHandler for wxGLCanvas {}
impl _wxObject for wxGLCanvas { fn handle(&self) -> *u8 { **self } }

impl wxGLCanvas {
    pub fn from(handle: *u8) -> @wxGLCanvas { @wxGLCanvas(handle) }
    pub fn null() -> @wxGLCanvas { wxGLCanvas::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPalette>(parent: &T, windowID: c_int, attributes: *c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: &str, palette: &U) -> @wxGLCanvas {
        let title = wxT(title);
        unsafe { @wxGLCanvas(wxGLCanvas_Create(parent.handle(), windowID, attributes, x, y, w, h, style, title.handle(), palette.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isDisplaySupported(attributes: *c_int) -> bool {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn isExtensionSupported(extension: &str) -> bool {
        let extension = wxT(extension);
        unsafe { wxGLCanvas_IsExtensionSupported(extension.handle()) }
    }
}

trait _wxGLCanvas : _wxScrolledWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColour<T: _wxColour>(&self, colour: &T) -> bool {
        unsafe { wxGLCanvas_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrent<T: _wxGLContext>(&self, ctxt: &T) -> bool {
        unsafe { wxGLCanvas_SetCurrent(self.handle(), ctxt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn swapBuffers(&self) -> bool {
        unsafe { wxGLCanvas_SwapBuffers(self.handle()) }
    }
}

struct wxGauge(*u8);
impl _wxGauge for wxGauge {}
impl _wxControl for wxGauge {}
impl _wxWindow for wxGauge {}
impl _wxEvtHandler for wxGauge {}
impl _wxObject for wxGauge { fn handle(&self) -> *u8 { **self } }

impl wxGauge {
    pub fn from(handle: *u8) -> @wxGauge { @wxGauge(handle) }
    pub fn null() -> @wxGauge { wxGauge::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxGauge {
        unsafe { @wxGauge(wxGauge_Create(_prt.handle(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxGauge : _wxControl {
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

struct wxGenericDirCtrl(*u8);
impl _wxGenericDirCtrl for wxGenericDirCtrl {}
impl _wxControl for wxGenericDirCtrl {}
impl _wxWindow for wxGenericDirCtrl {}
impl _wxEvtHandler for wxGenericDirCtrl {}
impl _wxObject for wxGenericDirCtrl { fn handle(&self) -> *u8 { **self } }

impl wxGenericDirCtrl {
    pub fn from(handle: *u8) -> @wxGenericDirCtrl { @wxGenericDirCtrl(handle) }
    pub fn null() -> @wxGenericDirCtrl { wxGenericDirCtrl::from(0 as *u8) }
    
}

trait _wxGenericDirCtrl : _wxControl {
}

struct wxGenericValidator(*u8);
impl _wxGenericValidator for wxGenericValidator {}
impl _wxValidator for wxGenericValidator {}
impl _wxEvtHandler for wxGenericValidator {}
impl _wxObject for wxGenericValidator { fn handle(&self) -> *u8 { **self } }

impl wxGenericValidator {
    pub fn from(handle: *u8) -> @wxGenericValidator { @wxGenericValidator(handle) }
    pub fn null() -> @wxGenericValidator { wxGenericValidator::from(0 as *u8) }
    
}

trait _wxGenericValidator : _wxValidator {
}

struct wxGrid(*u8);
impl _wxGrid for wxGrid {}
impl _wxScrolledWindow for wxGrid {}
impl _wxPanel for wxGrid {}
impl _wxWindow for wxGrid {}
impl _wxEvtHandler for wxGrid {}
impl _wxObject for wxGrid { fn handle(&self) -> *u8 { **self } }

impl wxGrid {
    pub fn from(handle: *u8) -> @wxGrid { @wxGrid(handle) }
    pub fn null() -> @wxGrid { wxGrid::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxGrid {
        unsafe { @wxGrid(wxGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxGrid : _wxScrolledWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendCols(&self, numCols: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_AppendCols(self.handle(), numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendRows(&self, numRows: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_AppendRows(self.handle(), numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoSize(&self) {
        unsafe { wxGrid_AutoSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoSizeColumn(&self, col: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumn(self.handle(), col, setAsMin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoSizeColumns(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumns(self.handle(), setAsMin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoSizeRow(&self, row: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRow(self.handle(), row, setAsMin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoSizeRows(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRows(self.handle(), setAsMin) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginBatch(&self) {
        unsafe { wxGrid_BeginBatch(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> @wxRect {
        unsafe { @wxRect(wxGrid_BlockToDeviceRect(self.handle(), top, left, bottom, right)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canDragColSize(&self) -> bool {
        unsafe { wxGrid_CanDragColSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canDragGridSize(&self) -> bool {
        unsafe { wxGrid_CanDragGridSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canDragRowSize(&self) -> bool {
        unsafe { wxGrid_CanDragRowSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canEnableCellControl(&self) -> bool {
        unsafe { wxGrid_CanEnableCellControl(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn cellToRect(&self, row: c_int, col: c_int) -> @wxRect {
        unsafe { @wxRect(wxGrid_CellToRect(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearGrid(&self) {
        unsafe { wxGrid_ClearGrid(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearSelection(&self) {
        unsafe { wxGrid_ClearSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn newGrid(&self, rows: c_int, cols: c_int, selmode: c_int) {
        unsafe { wxGrid_CreateGrid(self.handle(), rows, cols, selmode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteCols(&self, pos: c_int, numCols: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_DeleteCols(self.handle(), pos, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteRows(&self, pos: c_int, numRows: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_DeleteRows(self.handle(), pos, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableCellEditControl(&self) {
        unsafe { wxGrid_DisableCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableDragColSize(&self) {
        unsafe { wxGrid_DisableDragColSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableDragGridSize(&self) {
        unsafe { wxGrid_DisableDragGridSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableDragRowSize(&self) {
        unsafe { wxGrid_DisableDragRowSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawAllGridLines<T: _wxDC, U: _wxRegion>(&self, dc: &T, reg: &U) {
        unsafe { wxGrid_DrawAllGridLines(self.handle(), dc.handle(), reg.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCell<T: _wxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCellBorder<T: _wxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawCellHighlight<T: _wxDC, U: _wxGridCellAttr>(&self, dc: &T, attr: &U) {
        unsafe { wxGrid_DrawCellHighlight(self.handle(), dc.handle(), attr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawColLabel<T: _wxDC>(&self, dc: &T, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.handle(), dc.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawColLabels<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawColLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawGridSpace<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawGridSpace(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRowLabel<T: _wxDC>(&self, dc: &T, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.handle(), dc.handle(), row) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawRowLabels<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawRowLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawTextRectangle<T: _wxDC>(&self, dc: &T, txt: &str, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
        let txt = wxT(txt);
        unsafe { wxGrid_DrawTextRectangle(self.handle(), dc.handle(), txt.handle(), x, y, w, h, horizontalAlignment, verticalAlignment) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableCellEditControl(&self, enable: bool) {
        unsafe { wxGrid_EnableCellEditControl(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableDragColSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragColSize(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableDragGridSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragGridSize(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableDragRowSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragRowSize(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableEditing(&self, edit: c_int) {
        unsafe { wxGrid_EnableEditing(self.handle(), edit) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableGridLines(&self, enable: bool) {
        unsafe { wxGrid_EnableGridLines(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endBatch(&self) {
        unsafe { wxGrid_EndBatch(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBatchCount(&self) -> c_int {
        unsafe { wxGrid_GetBatchCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellAlignment(&self, row: c_int, col: c_int, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetCellAlignment(self.handle(), row, col, horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellEditor(&self, row: c_int, col: c_int) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetCellEditor(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_GetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellHighlightColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetCellHighlightColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellRenderer(&self, row: c_int, col: c_int) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetCellRenderer(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellValue(&self, row: c_int, col: c_int) -> ~str {
        unsafe { wxString { handle: wxGrid_GetCellValue(self.handle(), row, col) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColLabelAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetColLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetColLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColLabelValue(&self, col: c_int) -> ~str {
        unsafe { wxString { handle: wxGrid_GetColLabelValue(self.handle(), col) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColSize(&self, col: c_int) -> c_int {
        unsafe { wxGrid_GetColSize(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultCellAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetDefaultCellAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultCellBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultCellFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultCellTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultColSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultEditor(&self) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditor(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditorForCell(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultEditorForType(&self, typeName: &str) -> @wxGridCellEditor {
        let typeName = wxT(typeName);
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditorForType(self.handle(), typeName.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRenderer(&self) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRenderer(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRendererForCell(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRendererForType(&self, typeName: &str) -> @wxGridCellRenderer {
        let typeName = wxT(typeName);
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRendererForType(self.handle(), typeName.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRowSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGridCursorCol(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorCol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGridCursorRow(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorRow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGridLineColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetGridLineColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabelTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberCols(&self) -> c_int {
        unsafe { wxGrid_GetNumberCols(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNumberRows(&self) -> c_int {
        unsafe { wxGrid_GetNumberRows(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowLabelAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetRowLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetRowLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowLabelValue(&self, row: c_int) -> ~str {
        unsafe { wxString { handle: wxGrid_GetRowLabelValue(self.handle(), row) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.handle(), row) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectionBackground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectionForeground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTable(&self) -> @wxGridTableBase {
        unsafe { @wxGridTableBase(wxGrid_GetTable(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextBoxSize<T: _wxDC>(&self, dc: &T, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int) {
        unsafe { wxGrid_GetTextBoxSize(self.handle(), dc.handle(), count, lines, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn gridLinesEnabled(&self) -> c_int {
        unsafe { wxGrid_GridLinesEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideCellEditControl(&self) {
        unsafe { wxGrid_HideCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertCols(&self, pos: c_int, numCols: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_InsertCols(self.handle(), pos, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertRows(&self, pos: c_int, numRows: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_InsertRows(self.handle(), pos, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCellEditControlEnabled(&self) -> bool {
        unsafe { wxGrid_IsCellEditControlEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCellEditControlShown(&self) -> bool {
        unsafe { wxGrid_IsCellEditControlShown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCurrentCellReadOnly(&self) -> bool {
        unsafe { wxGrid_IsCurrentCellReadOnly(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEditable(&self) -> bool {
        unsafe { wxGrid_IsEditable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInSelection(&self, row: c_int, col: c_int) -> bool {
        unsafe { wxGrid_IsInSelection(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isReadOnly(&self, row: c_int, col: c_int) -> bool {
        unsafe { wxGrid_IsReadOnly(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelection(&self) -> bool {
        unsafe { wxGrid_IsSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible(&self, row: c_int, col: c_int, wholeCellVisible: bool) -> bool {
        unsafe { wxGrid_IsVisible(self.handle(), row, col, wholeCellVisible) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeCellVisible(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_MakeCellVisible(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorDown(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorDown(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorDownBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorDownBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorLeft(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorLeft(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorLeftBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorLeftBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorRight(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorRight(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorRightBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorRightBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorUp(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorUp(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveCursorUpBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorUpBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn movePageDown(&self) -> bool {
        unsafe { wxGrid_MovePageDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn movePageUp(&self) -> bool {
        unsafe { wxGrid_MovePageUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn processTableMessage<T: _wxEvent>(&self, evt: &T) -> bool {
        unsafe { wxGrid_ProcessTableMessage(self.handle(), evt.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn registerDataType<T: _wxGridCellRenderer, U: _wxGridCellEditor>(&self, typeName: &str, renderer: &T, editor: &U) {
        let typeName = wxT(typeName);
        unsafe { wxGrid_RegisterDataType(self.handle(), typeName.handle(), renderer.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveEditControlValue(&self) {
        unsafe { wxGrid_SaveEditControlValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectAll(&self) {
        unsafe { wxGrid_SelectAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectBlock(&self, topRow: c_int, leftCol: c_int, bottomRow: c_int, rightCol: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectBlock(self.handle(), topRow, leftCol, bottomRow, rightCol, addToSelected) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectCol(&self, col: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectCol(self.handle(), col, addToSelected) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectRow(&self, row: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectRow(self.handle(), row, addToSelected) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellAlignment(&self, row: c_int, col: c_int, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetCellAlignment(self.handle(), row, col, horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellEditor<T: _wxGridCellEditor>(&self, row: c_int, col: c_int, editor: &T) {
        unsafe { wxGrid_SetCellEditor(self.handle(), row, col, editor.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_SetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellHighlightColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxGrid_SetCellHighlightColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellRenderer<T: _wxGridCellRenderer>(&self, row: c_int, col: c_int, renderer: &T) {
        unsafe { wxGrid_SetCellRenderer(self.handle(), row, col, renderer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellValue(&self, row: c_int, col: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxGrid_SetCellValue(self.handle(), row, col, s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColAttr<T: _wxGridCellAttr>(&self, col: c_int, attr: &T) {
        unsafe { wxGrid_SetColAttr(self.handle(), col, attr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColFormatCustom(&self, col: c_int, typeName: &str) {
        let typeName = wxT(typeName);
        unsafe { wxGrid_SetColFormatCustom(self.handle(), col, typeName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColFormatFloat(&self, col: c_int, width: c_int, precision: c_int) {
        unsafe { wxGrid_SetColFormatFloat(self.handle(), col, width, precision) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColFormatNumber(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatNumber(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetColLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColLabelSize(&self, height: c_int) {
        unsafe { wxGrid_SetColLabelSize(self.handle(), height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColLabelValue(&self, col: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxGrid_SetColLabelValue(self.handle(), col, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColMinimalWidth(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColMinimalWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColSize(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColSize(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultCellAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetDefaultCellAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultCellBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultCellFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGrid_SetDefaultCellFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultCellTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.handle(), width, resizeExistingCols) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultEditor<T: _wxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGrid_SetDefaultEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultRenderer<T: _wxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGrid_SetDefaultRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultRowSize(&self, height: c_int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self.handle(), height, resizeExistingRows) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGridCursor(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_SetGridCursor(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setGridLineColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxGrid_SetGridLineColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabelBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabelFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGrid_SetLabelFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLabelTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMargins(&self, extraWidth: c_int, extraHeight: c_int) {
        unsafe { wxGrid_SetMargins(self.handle(), extraWidth, extraHeight) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReadOnly(&self, row: c_int, col: c_int, isReadOnly: bool) {
        unsafe { wxGrid_SetReadOnly(self.handle(), row, col, isReadOnly) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRowAttr<T: _wxGridCellAttr>(&self, row: c_int, attr: &T) {
        unsafe { wxGrid_SetRowAttr(self.handle(), row, attr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRowLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetRowLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRowLabelSize(&self, width: c_int) {
        unsafe { wxGrid_SetRowLabelSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRowLabelValue(&self, row: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxGrid_SetRowLabelValue(self.handle(), row, label.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRowMinimalHeight(&self, row: c_int, width: c_int) {
        unsafe { wxGrid_SetRowMinimalHeight(self.handle(), row, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRowSize(&self, row: c_int, height: c_int) {
        unsafe { wxGrid_SetRowSize(self.handle(), row, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelectionBackground<T: _wxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionBackground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelectionForeground<T: _wxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionForeground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.handle(), selmode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTable<T: _wxGridTableBase>(&self, table: &T, takeOwnership: bool, selmode: c_int) -> bool {
        unsafe { wxGrid_SetTable(self.handle(), table.handle(), takeOwnership, selmode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stringToLines(&self, value: &str, lines: *u8) -> c_int {
        let value = wxT(value);
        unsafe { wxGrid_StringToLines(self.handle(), value.handle(), lines) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xToCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToCol(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xToEdgeOfCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToEdgeOfCol(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xYToCell(&self, x: c_int, y: c_int, row: *c_int, col: *c_int) {
        unsafe { wxGrid_XYToCell(self.handle(), x, y, row, col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn yToEdgeOfRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToEdgeOfRow(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn yToRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToRow(self.handle(), y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectedCells<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectedCells(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectionBlockTopLeft<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectionBlockBottomRight<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockBottomRight(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectedRows(&self, _arr: *intptr_t) -> c_int {
        unsafe { wxGrid_GetSelectedRows(self.handle(), _arr) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelectedCols(&self, _arr: *intptr_t) -> c_int {
        unsafe { wxGrid_GetSelectedCols(self.handle(), _arr) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCellSize(&self, row: c_int, col: c_int, srow: *c_int, scol: *c_int) {
        unsafe { wxGrid_GetCellSize(self.handle(), row, col, srow, scol) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCellSize(&self, row: c_int, col: c_int, srow: c_int, scol: c_int) {
        unsafe { wxGrid_SetCellSize(self.handle(), row, col, srow, scol) }
    }
}

struct wxGridCellAttr(*u8);
impl _wxGridCellAttr for wxGridCellAttr { fn handle(&self) -> *u8 { **self } }

impl wxGridCellAttr {
    pub fn from(handle: *u8) -> @wxGridCellAttr { @wxGridCellAttr(handle) }
    pub fn null() -> @wxGridCellAttr { wxGridCellAttr::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor() -> @wxGridCellAttr {
        unsafe { @wxGridCellAttr(wxGridCellAttr_Ctor()) }
    }
}

trait _wxGridCellAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlignment(&self, hAlign: *c_int, vAlign: *c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.handle(), hAlign, vAlign) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEditor<T: _wxGrid>(&self, grid: &T, row: c_int, col: c_int) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGridCellAttr_GetEditor(self.handle(), grid.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRenderer<T: _wxGrid>(&self, grid: &T, row: c_int, col: c_int) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGridCellAttr_GetRenderer(self.handle(), grid.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasAlignment(&self) -> bool {
        unsafe { wxGridCellAttr_HasAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxGridCellAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasEditor(&self) -> bool {
        unsafe { wxGridCellAttr_HasEditor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFont(&self) -> bool {
        unsafe { wxGridCellAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasRenderer(&self) -> bool {
        unsafe { wxGridCellAttr_HasRenderer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasTextColour(&self) -> bool {
        unsafe { wxGridCellAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn incRef(&self) {
        unsafe { wxGridCellAttr_IncRef(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isReadOnly(&self) -> bool {
        unsafe { wxGridCellAttr_IsReadOnly(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAlignment(&self, hAlign: c_int, vAlign: c_int) {
        unsafe { wxGridCellAttr_SetAlignment(self.handle(), hAlign, vAlign) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackgroundColour<T: _wxColour>(&self, colBack: &T) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefAttr<T: _wxGridCellAttr>(&self, defAttr: &T) {
        unsafe { wxGridCellAttr_SetDefAttr(self.handle(), defAttr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEditor<T: _wxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGridCellAttr_SetEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGridCellAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setReadOnly(&self, isReadOnly: bool) {
        unsafe { wxGridCellAttr_SetReadOnly(self.handle(), isReadOnly) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRenderer<T: _wxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGridCellAttr_SetRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, colText: &T) {
        unsafe { wxGridCellAttr_SetTextColour(self.handle(), colText.handle()) }
    }
}

struct wxGridCellBoolEditor(*u8);
impl _wxGridCellBoolEditor for wxGridCellBoolEditor {}
impl _wxGridCellEditor for wxGridCellBoolEditor {}
impl _wxGridCellWorker for wxGridCellBoolEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolEditor {
    pub fn from(handle: *u8) -> @wxGridCellBoolEditor { @wxGridCellBoolEditor(handle) }
    pub fn null() -> @wxGridCellBoolEditor { wxGridCellBoolEditor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor() -> @wxGridCellBoolEditor {
        unsafe { @wxGridCellBoolEditor(wxGridCellBoolEditor_Ctor()) }
    }
}

trait _wxGridCellBoolEditor : _wxGridCellEditor {
}

struct wxGridCellBoolRenderer(*u8);
impl _wxGridCellBoolRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellWorker for wxGridCellBoolRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolRenderer {
    pub fn from(handle: *u8) -> @wxGridCellBoolRenderer { @wxGridCellBoolRenderer(handle) }
    pub fn null() -> @wxGridCellBoolRenderer { wxGridCellBoolRenderer::from(0 as *u8) }
    
}

trait _wxGridCellBoolRenderer : _wxGridCellRenderer {
}

struct wxGridCellChoiceEditor(*u8);
impl _wxGridCellChoiceEditor for wxGridCellChoiceEditor {}
impl _wxGridCellEditor for wxGridCellChoiceEditor {}
impl _wxGridCellWorker for wxGridCellChoiceEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellChoiceEditor {
    pub fn from(handle: *u8) -> @wxGridCellChoiceEditor { @wxGridCellChoiceEditor(handle) }
    pub fn null() -> @wxGridCellChoiceEditor { wxGridCellChoiceEditor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> @wxGridCellChoiceEditor {
        unsafe { @wxGridCellChoiceEditor(wxGridCellChoiceEditor_Ctor(count, choices, allowOthers)) }
    }
}

trait _wxGridCellChoiceEditor : _wxGridCellEditor {
}

struct wxGridCellCoordsArray(*u8);
impl _wxGridCellCoordsArray for wxGridCellCoordsArray { fn handle(&self) -> *u8 { **self } }

impl wxGridCellCoordsArray {
    pub fn from(handle: *u8) -> @wxGridCellCoordsArray { @wxGridCellCoordsArray(handle) }
    pub fn null() -> @wxGridCellCoordsArray { wxGridCellCoordsArray::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGridCellCoordsArray {
        unsafe { @wxGridCellCoordsArray(wxGridCellCoordsArray_Create()) }
    }
}

trait _wxGridCellCoordsArray {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxGridCellCoordsArray_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCount(&self) -> c_int {
        unsafe { wxGridCellCoordsArray_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn item(&self, _idx: c_int, _c: *c_int, _r: *c_int) {
        unsafe { wxGridCellCoordsArray_Item(self.handle(), _idx, _c, _r) }
    }
}

struct wxGridCellEditor(*u8);
impl _wxGridCellEditor for wxGridCellEditor {}
impl _wxGridCellWorker for wxGridCellEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellEditor {
    pub fn from(handle: *u8) -> @wxGridCellEditor { @wxGridCellEditor(handle) }
    pub fn null() -> @wxGridCellEditor { wxGridCellEditor::from(0 as *u8) }
    
}

trait _wxGridCellEditor : _wxGridCellWorker {
    #[fixed_stack_segment]
    #[inline(never)]
    fn beginEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: &T) {
        unsafe { wxGridCellEditor_BeginEdit(self.handle(), row, col, grid.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn new<T: _wxWindow, U: _wxEvtHandler>(&self, parent: &T, id: c_int, evtHandler: &U) {
        unsafe { wxGridCellEditor_Create(self.handle(), parent.handle(), id, evtHandler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: &T, oldStr: &str, newStr: &str) -> c_int {
        let oldStr = wxT(oldStr);
        let newStr = wxT(newStr);
        unsafe { wxGridCellEditor_EndEdit(self.handle(), row, col, grid.handle(), oldStr.handle(), newStr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControl(&self) -> @wxControl {
        unsafe { @wxControl(wxGridCellEditor_GetControl(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn handleReturn<T: _wxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_HandleReturn(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAcceptedKey<T: _wxEvent>(&self, event: &T) -> bool {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isCreated(&self) -> bool {
        unsafe { wxGridCellEditor_IsCreated(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paintBackground<T: _wxGridCellAttr>(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: &T) {
        unsafe { wxGridCellEditor_PaintBackground(self.handle(), x, y, w, h, attr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setControl<T: _wxControl>(&self, control: &T) {
        unsafe { wxGridCellEditor_SetControl(self.handle(), control.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setParameters(&self, params: &str) {
        let params = wxT(params);
        unsafe { wxGridCellEditor_SetParameters(self.handle(), params.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show<T: _wxGridCellAttr>(&self, show: c_int, attr: &T) {
        unsafe { wxGridCellEditor_Show(self.handle(), show, attr.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startingKey<T: _wxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_StartingKey(self.handle(), event.handle()) }
    }
}

struct wxGridCellFloatEditor(*u8);
impl _wxGridCellFloatEditor for wxGridCellFloatEditor {}
impl _wxGridCellTextEditor for wxGridCellFloatEditor {}
impl _wxGridCellEditor for wxGridCellFloatEditor {}
impl _wxGridCellWorker for wxGridCellFloatEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellFloatEditor {
    pub fn from(handle: *u8) -> @wxGridCellFloatEditor { @wxGridCellFloatEditor(handle) }
    pub fn null() -> @wxGridCellFloatEditor { wxGridCellFloatEditor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor(width: c_int, precision: c_int) -> @wxGridCellFloatEditor {
        unsafe { @wxGridCellFloatEditor(wxGridCellFloatEditor_Ctor(width, precision)) }
    }
}

trait _wxGridCellFloatEditor : _wxGridCellTextEditor {
}

struct wxGridCellFloatRenderer(*u8);
impl _wxGridCellFloatRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellStringRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellWorker for wxGridCellFloatRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellFloatRenderer {
    pub fn from(handle: *u8) -> @wxGridCellFloatRenderer { @wxGridCellFloatRenderer(handle) }
    pub fn null() -> @wxGridCellFloatRenderer { wxGridCellFloatRenderer::from(0 as *u8) }
    
}

trait _wxGridCellFloatRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellNumberEditor(*u8);
impl _wxGridCellNumberEditor for wxGridCellNumberEditor {}
impl _wxGridCellTextEditor for wxGridCellNumberEditor {}
impl _wxGridCellEditor for wxGridCellNumberEditor {}
impl _wxGridCellWorker for wxGridCellNumberEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellNumberEditor {
    pub fn from(handle: *u8) -> @wxGridCellNumberEditor { @wxGridCellNumberEditor(handle) }
    pub fn null() -> @wxGridCellNumberEditor { wxGridCellNumberEditor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor(min: c_int, max: c_int) -> @wxGridCellNumberEditor {
        unsafe { @wxGridCellNumberEditor(wxGridCellNumberEditor_Ctor(min, max)) }
    }
}

trait _wxGridCellNumberEditor : _wxGridCellTextEditor {
}

struct wxGridCellNumberRenderer(*u8);
impl _wxGridCellNumberRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellStringRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellWorker for wxGridCellNumberRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellNumberRenderer {
    pub fn from(handle: *u8) -> @wxGridCellNumberRenderer { @wxGridCellNumberRenderer(handle) }
    pub fn null() -> @wxGridCellNumberRenderer { wxGridCellNumberRenderer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor() -> @wxGridCellNumberRenderer {
        unsafe { @wxGridCellNumberRenderer(wxGridCellNumberRenderer_Ctor()) }
    }
}

trait _wxGridCellNumberRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellAutoWrapStringRenderer(*u8);
impl _wxGridCellAutoWrapStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellWorker for wxGridCellAutoWrapStringRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellAutoWrapStringRenderer {
    pub fn from(handle: *u8) -> @wxGridCellAutoWrapStringRenderer { @wxGridCellAutoWrapStringRenderer(handle) }
    pub fn null() -> @wxGridCellAutoWrapStringRenderer { wxGridCellAutoWrapStringRenderer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor() -> @wxGridCellAutoWrapStringRenderer {
        unsafe { @wxGridCellAutoWrapStringRenderer(wxGridCellAutoWrapStringRenderer_Ctor()) }
    }
}

trait _wxGridCellAutoWrapStringRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellRenderer(*u8);
impl _wxGridCellRenderer for wxGridCellRenderer {}
impl _wxGridCellWorker for wxGridCellRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellRenderer {
    pub fn from(handle: *u8) -> @wxGridCellRenderer { @wxGridCellRenderer(handle) }
    pub fn null() -> @wxGridCellRenderer { wxGridCellRenderer::from(0 as *u8) }
    
}

trait _wxGridCellRenderer : _wxGridCellWorker {
}

struct wxGridCellStringRenderer(*u8);
impl _wxGridCellStringRenderer for wxGridCellStringRenderer {}
impl _wxGridCellRenderer for wxGridCellStringRenderer {}
impl _wxGridCellWorker for wxGridCellStringRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellStringRenderer {
    pub fn from(handle: *u8) -> @wxGridCellStringRenderer { @wxGridCellStringRenderer(handle) }
    pub fn null() -> @wxGridCellStringRenderer { wxGridCellStringRenderer::from(0 as *u8) }
    
}

trait _wxGridCellStringRenderer : _wxGridCellRenderer {
}

struct wxGridCellTextEditor(*u8);
impl _wxGridCellTextEditor for wxGridCellTextEditor {}
impl _wxGridCellEditor for wxGridCellTextEditor {}
impl _wxGridCellWorker for wxGridCellTextEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellTextEditor {
    pub fn from(handle: *u8) -> @wxGridCellTextEditor { @wxGridCellTextEditor(handle) }
    pub fn null() -> @wxGridCellTextEditor { wxGridCellTextEditor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor() -> @wxGridCellTextEditor {
        unsafe { @wxGridCellTextEditor(wxGridCellTextEditor_Ctor()) }
    }
}

trait _wxGridCellTextEditor : _wxGridCellEditor {
}

struct wxGridCellWorker(*u8);
impl _wxGridCellWorker for wxGridCellWorker { fn handle(&self) -> *u8 { **self } }

impl wxGridCellWorker {
    pub fn from(handle: *u8) -> @wxGridCellWorker { @wxGridCellWorker(handle) }
    pub fn null() -> @wxGridCellWorker { wxGridCellWorker::from(0 as *u8) }
    
}

trait _wxGridCellWorker {
    fn handle(&self) -> *u8;
    
}

struct wxGridEditorCreatedEvent(*u8);
impl _wxGridEditorCreatedEvent for wxGridEditorCreatedEvent {}
impl _wxCommandEvent for wxGridEditorCreatedEvent {}
impl _wxEvent for wxGridEditorCreatedEvent {}
impl _wxObject for wxGridEditorCreatedEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridEditorCreatedEvent {
    pub fn from(handle: *u8) -> @wxGridEditorCreatedEvent { @wxGridEditorCreatedEvent(handle) }
    pub fn null() -> @wxGridEditorCreatedEvent { wxGridEditorCreatedEvent::from(0 as *u8) }
    
}

trait _wxGridEditorCreatedEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControl(&self) -> @wxControl {
        unsafe { @wxControl(wxGridEditorCreatedEvent_GetControl(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRow(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCol(&self, col: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setControl<T: _wxControl>(&self, ctrl: &T) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.handle(), row) }
    }
}

struct wxGridEvent(*u8);
impl _wxGridEvent for wxGridEvent {}
impl _wxNotifyEvent for wxGridEvent {}
impl _wxCommandEvent for wxGridEvent {}
impl _wxEvent for wxGridEvent {}
impl _wxObject for wxGridEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridEvent {
    pub fn from(handle: *u8) -> @wxGridEvent { @wxGridEvent(handle) }
    pub fn null() -> @wxGridEvent { wxGridEvent::from(0 as *u8) }
    
}

trait _wxGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> bool {
        unsafe { wxGridEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> bool {
        unsafe { wxGridEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxGridEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRow(&self) -> c_int {
        unsafe { wxGridEvent_GetRow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> bool {
        unsafe { wxGridEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selecting(&self) -> bool {
        unsafe { wxGridEvent_Selecting(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridEvent_ShiftDown(self.handle()) }
    }
}

struct wxGridRangeSelectEvent(*u8);
impl _wxGridRangeSelectEvent for wxGridRangeSelectEvent {}
impl _wxNotifyEvent for wxGridRangeSelectEvent {}
impl _wxCommandEvent for wxGridRangeSelectEvent {}
impl _wxEvent for wxGridRangeSelectEvent {}
impl _wxObject for wxGridRangeSelectEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridRangeSelectEvent {
    pub fn from(handle: *u8) -> @wxGridRangeSelectEvent { @wxGridRangeSelectEvent(handle) }
    pub fn null() -> @wxGridRangeSelectEvent { wxGridRangeSelectEvent::from(0 as *u8) }
    
}

trait _wxGridRangeSelectEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTopLeftCoords(&self, col: *c_int, row: *c_int) {
        unsafe { wxGridRangeSelectEvent_GetTopLeftCoords(self.handle(), col, row) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBottomRightCoords(&self, col: *c_int, row: *c_int) {
        unsafe { wxGridRangeSelectEvent_GetBottomRightCoords(self.handle(), col, row) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTopRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetTopRow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBottomRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetBottomRow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLeftCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetLeftCol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRightCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetRightCol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selecting(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_Selecting(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_ShiftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_AltDown(self.handle()) }
    }
}

struct wxGridSizeEvent(*u8);
impl _wxGridSizeEvent for wxGridSizeEvent {}
impl _wxNotifyEvent for wxGridSizeEvent {}
impl _wxCommandEvent for wxGridSizeEvent {}
impl _wxEvent for wxGridSizeEvent {}
impl _wxObject for wxGridSizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridSizeEvent {
    pub fn from(handle: *u8) -> @wxGridSizeEvent { @wxGridSizeEvent(handle) }
    pub fn null() -> @wxGridSizeEvent { wxGridSizeEvent::from(0 as *u8) }
    
}

trait _wxGridSizeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxGridSizeEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> bool {
        unsafe { wxGridSizeEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> bool {
        unsafe { wxGridSizeEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridSizeEvent_ShiftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> bool {
        unsafe { wxGridSizeEvent_AltDown(self.handle()) }
    }
}

struct wxGridSizer(*u8);
impl _wxGridSizer for wxGridSizer {}
impl _wxSizer for wxGridSizer {}
impl _wxObject for wxGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxGridSizer {
    pub fn from(handle: *u8) -> @wxGridSizer { @wxGridSizer(handle) }
    pub fn null() -> @wxGridSizer { wxGridSizer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @wxGridSizer {
        unsafe { @wxGridSizer(wxGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

trait _wxGridSizer : _wxSizer {
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

struct wxGridTableBase(*u8);
impl _wxGridTableBase for wxGridTableBase {}
impl _wxObject for wxGridTableBase { fn handle(&self) -> *u8 { **self } }

impl wxGridTableBase {
    pub fn from(handle: *u8) -> @wxGridTableBase { @wxGridTableBase(handle) }
    pub fn null() -> @wxGridTableBase { wxGridTableBase::from(0 as *u8) }
    
}

trait _wxGridTableBase : _wxObject {
}

struct wxHTTP(*u8);
impl _wxHTTP for wxHTTP {}
impl _wxProtocol for wxHTTP {}
impl _wxSocketClient for wxHTTP {}
impl _wxSocketBase for wxHTTP {}
impl _wxObject for wxHTTP { fn handle(&self) -> *u8 { **self } }

impl wxHTTP {
    pub fn from(handle: *u8) -> @wxHTTP { @wxHTTP(handle) }
    pub fn null() -> @wxHTTP { wxHTTP::from(0 as *u8) }
    
}

trait _wxHTTP : _wxProtocol {
}

struct wxHashMap(*u8);
impl _wxHashMap for wxHashMap { fn handle(&self) -> *u8 { **self } }

impl wxHashMap {
    pub fn from(handle: *u8) -> @wxHashMap { @wxHashMap(handle) }
    pub fn null() -> @wxHashMap { wxHashMap::from(0 as *u8) }
    
}

trait _wxHashMap {
    fn handle(&self) -> *u8;
    
}

struct wxHelpController(*u8);
impl _wxHelpController for wxHelpController {}
impl _wxHelpControllerBase for wxHelpController {}
impl _wxObject for wxHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHelpController {
    pub fn from(handle: *u8) -> @wxHelpController { @wxHelpController(handle) }
    pub fn null() -> @wxHelpController { wxHelpController::from(0 as *u8) }
    
}

trait _wxHelpController : _wxHelpControllerBase {
}

struct wxHelpControllerBase(*u8);
impl _wxHelpControllerBase for wxHelpControllerBase {}
impl _wxObject for wxHelpControllerBase { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerBase {
    pub fn from(handle: *u8) -> @wxHelpControllerBase { @wxHelpControllerBase(handle) }
    pub fn null() -> @wxHelpControllerBase { wxHelpControllerBase::from(0 as *u8) }
    
}

trait _wxHelpControllerBase : _wxObject {
}

struct wxHelpControllerHelpProvider(*u8);
impl _wxHelpControllerHelpProvider for wxHelpControllerHelpProvider {}
impl _wxSimpleHelpProvider for wxHelpControllerHelpProvider {}
impl _wxHelpProvider for wxHelpControllerHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerHelpProvider {
    pub fn from(handle: *u8) -> @wxHelpControllerHelpProvider { @wxHelpControllerHelpProvider(handle) }
    pub fn null() -> @wxHelpControllerHelpProvider { wxHelpControllerHelpProvider::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxHelpControllerBase>(ctr: &T) -> @wxHelpControllerHelpProvider {
        unsafe { @wxHelpControllerHelpProvider(wxHelpControllerHelpProvider_Create(ctr.handle())) }
    }
}

trait _wxHelpControllerHelpProvider : _wxSimpleHelpProvider {
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

struct wxHelpEvent(*u8);
impl _wxHelpEvent for wxHelpEvent {}
impl _wxCommandEvent for wxHelpEvent {}
impl _wxEvent for wxHelpEvent {}
impl _wxObject for wxHelpEvent { fn handle(&self) -> *u8 { **self } }

impl wxHelpEvent {
    pub fn from(handle: *u8) -> @wxHelpEvent { @wxHelpEvent(handle) }
    pub fn null() -> @wxHelpEvent { wxHelpEvent::from(0 as *u8) }
    
}

trait _wxHelpEvent : _wxCommandEvent {
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

struct wxHelpProvider(*u8);
impl _wxHelpProvider for wxHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpProvider {
    pub fn from(handle: *u8) -> @wxHelpProvider { @wxHelpProvider(handle) }
    pub fn null() -> @wxHelpProvider { wxHelpProvider::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> @wxHelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Get()) }
    }
}

trait _wxHelpProvider {
    fn handle(&self) -> *u8;
    
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
    fn showHelp<T: _wxWindow>(&self, window: &T) -> bool {
        unsafe { wxHelpProvider_ShowHelp(self.handle(), window.handle()) }
    }
}

struct wxHtmlCell(*u8);
impl _wxHtmlCell for wxHtmlCell {}
impl _wxObject for wxHtmlCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlCell {
    pub fn from(handle: *u8) -> @wxHtmlCell { @wxHtmlCell(handle) }
    pub fn null() -> @wxHtmlCell { wxHtmlCell::from(0 as *u8) }
    
}

trait _wxHtmlCell : _wxObject {
}

struct wxHtmlColourCell(*u8);
impl _wxHtmlColourCell for wxHtmlColourCell {}
impl _wxHtmlCell for wxHtmlColourCell {}
impl _wxObject for wxHtmlColourCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlColourCell {
    pub fn from(handle: *u8) -> @wxHtmlColourCell { @wxHtmlColourCell(handle) }
    pub fn null() -> @wxHtmlColourCell { wxHtmlColourCell::from(0 as *u8) }
    
}

trait _wxHtmlColourCell : _wxHtmlCell {
}

struct wxHtmlContainerCell(*u8);
impl _wxHtmlContainerCell for wxHtmlContainerCell {}
impl _wxHtmlCell for wxHtmlContainerCell {}
impl _wxObject for wxHtmlContainerCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlContainerCell {
    pub fn from(handle: *u8) -> @wxHtmlContainerCell { @wxHtmlContainerCell(handle) }
    pub fn null() -> @wxHtmlContainerCell { wxHtmlContainerCell::from(0 as *u8) }
    
}

trait _wxHtmlContainerCell : _wxHtmlCell {
}

struct wxHtmlDCRenderer(*u8);
impl _wxHtmlDCRenderer for wxHtmlDCRenderer {}
impl _wxObject for wxHtmlDCRenderer { fn handle(&self) -> *u8 { **self } }

impl wxHtmlDCRenderer {
    pub fn from(handle: *u8) -> @wxHtmlDCRenderer { @wxHtmlDCRenderer(handle) }
    pub fn null() -> @wxHtmlDCRenderer { wxHtmlDCRenderer::from(0 as *u8) }
    
}

trait _wxHtmlDCRenderer : _wxObject {
}

struct wxHtmlEasyPrinting(*u8);
impl _wxHtmlEasyPrinting for wxHtmlEasyPrinting {}
impl _wxObject for wxHtmlEasyPrinting { fn handle(&self) -> *u8 { **self } }

impl wxHtmlEasyPrinting {
    pub fn from(handle: *u8) -> @wxHtmlEasyPrinting { @wxHtmlEasyPrinting(handle) }
    pub fn null() -> @wxHtmlEasyPrinting { wxHtmlEasyPrinting::from(0 as *u8) }
    
}

trait _wxHtmlEasyPrinting : _wxObject {
}

struct wxHtmlFilter(*u8);
impl _wxHtmlFilter for wxHtmlFilter {}
impl _wxObject for wxHtmlFilter { fn handle(&self) -> *u8 { **self } }

impl wxHtmlFilter {
    pub fn from(handle: *u8) -> @wxHtmlFilter { @wxHtmlFilter(handle) }
    pub fn null() -> @wxHtmlFilter { wxHtmlFilter::from(0 as *u8) }
    
}

trait _wxHtmlFilter : _wxObject {
}

struct wxHtmlHelpController(*u8);
impl _wxHtmlHelpController for wxHtmlHelpController {}
impl _wxHelpControllerBase for wxHtmlHelpController {}
impl _wxObject for wxHtmlHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpController {
    pub fn from(handle: *u8) -> @wxHtmlHelpController { @wxHtmlHelpController(handle) }
    pub fn null() -> @wxHtmlHelpController { wxHtmlHelpController::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_style: c_int) -> @wxHtmlHelpController {
        unsafe { @wxHtmlHelpController(wxHtmlHelpController_Create(_style)) }
    }
}

trait _wxHtmlHelpController : _wxHelpControllerBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addBook(&self, book: *u8, show_wait_msg: c_int) -> bool {
        unsafe { wxHtmlHelpController_AddBook(self.handle(), book, show_wait_msg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn display(&self, x: *u8) -> c_int {
        unsafe { wxHtmlHelpController_Display(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displayBlock(&self, blockNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplayBlock(self.handle(), blockNo) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displayContents(&self) -> c_int {
        unsafe { wxHtmlHelpController_DisplayContents(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displayIndex(&self) -> c_int {
        unsafe { wxHtmlHelpController_DisplayIndex(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displayNumber(&self, id: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplayNumber(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displaySection(&self, section: &str) -> bool {
        let section = wxT(section);
        unsafe { wxHtmlHelpController_DisplaySection(self.handle(), section.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displaySectionNumber(&self, sectionNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.handle(), sectionNo) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrame(wxHtmlHelpController_GetFrame(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrameParameters(&self, title: *u8, width: *c_int, height: *c_int, pos_x: *c_int, pos_y: *c_int, newFrameEachTime: *c_int) -> *u8 {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self, file: &str) -> bool {
        let file = wxT(file);
        unsafe { wxHtmlHelpController_Initialize(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn keywordSearch(&self, keyword: &str) -> bool {
        let keyword = wxT(keyword);
        unsafe { wxHtmlHelpController_KeywordSearch(self.handle(), keyword.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, file: &str) -> bool {
        let file = wxT(file);
        unsafe { wxHtmlHelpController_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn quit(&self) -> bool {
        unsafe { wxHtmlHelpController_Quit(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFrameParameters(&self, title: *u8, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: bool) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTempDir(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_SetTempDir(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTitleFormat(&self, format: *u8) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.handle(), format) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setViewer(&self, viewer: &str, flags: c_int) {
        let viewer = wxT(viewer);
        unsafe { wxHtmlHelpController_SetViewer(self.handle(), viewer.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn useConfig<T: _wxConfigBase>(&self, config: &T, rootpath: &str) {
        let rootpath = wxT(rootpath);
        unsafe { wxHtmlHelpController_UseConfig(self.handle(), config.handle(), rootpath.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxHtmlHelpData(*u8);
impl _wxHtmlHelpData for wxHtmlHelpData {}
impl _wxObject for wxHtmlHelpData { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpData {
    pub fn from(handle: *u8) -> @wxHtmlHelpData { @wxHtmlHelpData(handle) }
    pub fn null() -> @wxHtmlHelpData { wxHtmlHelpData::from(0 as *u8) }
    
}

trait _wxHtmlHelpData : _wxObject {
}

struct wxHtmlHelpFrame(*u8);
impl _wxHtmlHelpFrame for wxHtmlHelpFrame {}
impl _wxFrame for wxHtmlHelpFrame {}
impl _wxTopLevelWindow for wxHtmlHelpFrame {}
impl _wxWindow for wxHtmlHelpFrame {}
impl _wxEvtHandler for wxHtmlHelpFrame {}
impl _wxObject for wxHtmlHelpFrame { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpFrame {
    pub fn from(handle: *u8) -> @wxHtmlHelpFrame { @wxHtmlHelpFrame(handle) }
    pub fn null() -> @wxHtmlHelpFrame { wxHtmlHelpFrame::from(0 as *u8) }
    
}

trait _wxHtmlHelpFrame : _wxFrame {
}

struct wxHtmlLinkInfo(*u8);
impl _wxHtmlLinkInfo for wxHtmlLinkInfo {}
impl _wxObject for wxHtmlLinkInfo { fn handle(&self) -> *u8 { **self } }

impl wxHtmlLinkInfo {
    pub fn from(handle: *u8) -> @wxHtmlLinkInfo { @wxHtmlLinkInfo(handle) }
    pub fn null() -> @wxHtmlLinkInfo { wxHtmlLinkInfo::from(0 as *u8) }
    
}

trait _wxHtmlLinkInfo : _wxObject {
}

struct wxHtmlParser(*u8);
impl _wxHtmlParser for wxHtmlParser {}
impl _wxObject for wxHtmlParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlParser {
    pub fn from(handle: *u8) -> @wxHtmlParser { @wxHtmlParser(handle) }
    pub fn null() -> @wxHtmlParser { wxHtmlParser::from(0 as *u8) }
    
}

trait _wxHtmlParser : _wxObject {
}

struct wxHtmlPrintout(*u8);
impl _wxHtmlPrintout for wxHtmlPrintout {}
impl _wxPrintout for wxHtmlPrintout {}
impl _wxObject for wxHtmlPrintout { fn handle(&self) -> *u8 { **self } }

impl wxHtmlPrintout {
    pub fn from(handle: *u8) -> @wxHtmlPrintout { @wxHtmlPrintout(handle) }
    pub fn null() -> @wxHtmlPrintout { wxHtmlPrintout::from(0 as *u8) }
    
}

trait _wxHtmlPrintout : _wxPrintout {
}

struct wxHtmlTag(*u8);
impl _wxHtmlTag for wxHtmlTag {}
impl _wxObject for wxHtmlTag { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTag {
    pub fn from(handle: *u8) -> @wxHtmlTag { @wxHtmlTag(handle) }
    pub fn null() -> @wxHtmlTag { wxHtmlTag::from(0 as *u8) }
    
}

trait _wxHtmlTag : _wxObject {
}

struct wxHtmlTagHandler(*u8);
impl _wxHtmlTagHandler for wxHtmlTagHandler {}
impl _wxObject for wxHtmlTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagHandler {
    pub fn from(handle: *u8) -> @wxHtmlTagHandler { @wxHtmlTagHandler(handle) }
    pub fn null() -> @wxHtmlTagHandler { wxHtmlTagHandler::from(0 as *u8) }
    
}

trait _wxHtmlTagHandler : _wxObject {
}

struct wxHtmlTagsModule(*u8);
impl _wxHtmlTagsModule for wxHtmlTagsModule {}
impl _wxModule for wxHtmlTagsModule {}
impl _wxObject for wxHtmlTagsModule { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagsModule {
    pub fn from(handle: *u8) -> @wxHtmlTagsModule { @wxHtmlTagsModule(handle) }
    pub fn null() -> @wxHtmlTagsModule { wxHtmlTagsModule::from(0 as *u8) }
    
}

trait _wxHtmlTagsModule : _wxModule {
}

struct wxHtmlWidgetCell(*u8);
impl _wxHtmlWidgetCell for wxHtmlWidgetCell {}
impl _wxHtmlCell for wxHtmlWidgetCell {}
impl _wxObject for wxHtmlWidgetCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWidgetCell {
    pub fn from(handle: *u8) -> @wxHtmlWidgetCell { @wxHtmlWidgetCell(handle) }
    pub fn null() -> @wxHtmlWidgetCell { wxHtmlWidgetCell::from(0 as *u8) }
    
}

trait _wxHtmlWidgetCell : _wxHtmlCell {
}

struct wxHtmlWinParser(*u8);
impl _wxHtmlWinParser for wxHtmlWinParser {}
impl _wxHtmlParser for wxHtmlWinParser {}
impl _wxObject for wxHtmlWinParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinParser {
    pub fn from(handle: *u8) -> @wxHtmlWinParser { @wxHtmlWinParser(handle) }
    pub fn null() -> @wxHtmlWinParser { wxHtmlWinParser::from(0 as *u8) }
    
}

trait _wxHtmlWinParser : _wxHtmlParser {
}

struct wxHtmlWinTagHandler(*u8);
impl _wxHtmlWinTagHandler for wxHtmlWinTagHandler {}
impl _wxHtmlTagHandler for wxHtmlWinTagHandler {}
impl _wxObject for wxHtmlWinTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinTagHandler {
    pub fn from(handle: *u8) -> @wxHtmlWinTagHandler { @wxHtmlWinTagHandler(handle) }
    pub fn null() -> @wxHtmlWinTagHandler { wxHtmlWinTagHandler::from(0 as *u8) }
    
}

trait _wxHtmlWinTagHandler : _wxHtmlTagHandler {
}

struct wxHtmlWindow(*u8);
impl _wxHtmlWindow for wxHtmlWindow {}
impl _wxScrolledWindow for wxHtmlWindow {}
impl _wxPanel for wxHtmlWindow {}
impl _wxWindow for wxHtmlWindow {}
impl _wxEvtHandler for wxHtmlWindow {}
impl _wxObject for wxHtmlWindow { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWindow {
    pub fn from(handle: *u8) -> @wxHtmlWindow { @wxHtmlWindow(handle) }
    pub fn null() -> @wxHtmlWindow { wxHtmlWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> @wxHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { @wxHtmlWindow(wxHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) }
    }
}

trait _wxHtmlWindow : _wxScrolledWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendToPage(&self, source: &str) -> bool {
        let source = wxT(source);
        unsafe { wxHtmlWindow_AppendToPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInternalRepresentation(&self) -> @wxHtmlContainerCell {
        unsafe { @wxHtmlContainerCell(wxHtmlWindow_GetInternalRepresentation(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOpenedAnchor(&self) -> ~str {
        unsafe { wxString { handle: wxHtmlWindow_GetOpenedAnchor(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOpenedPage(&self) -> ~str {
        unsafe { wxString { handle: wxHtmlWindow_GetOpenedPage(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOpenedPageTitle(&self) -> ~str {
        unsafe { wxString { handle: wxHtmlWindow_GetOpenedPageTitle(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRelatedFrame(&self) -> @wxFrame {
        unsafe { @wxFrame(wxHtmlWindow_GetRelatedFrame(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyBack(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryBack(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyCanBack(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryCanBack(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyCanForward(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryCanForward(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyClear(&self) {
        unsafe { wxHtmlWindow_HistoryClear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyForward(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryForward(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadPage(&self, location: &str) -> bool {
        let location = wxT(location);
        unsafe { wxHtmlWindow_LoadPage(self.handle(), location.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlWindow_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.handle(), b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFonts(&self, normal_face: &str, fixed_face: &str, sizes: *c_int) {
        let normal_face = wxT(normal_face);
        let fixed_face = wxT(fixed_face);
        unsafe { wxHtmlWindow_SetFonts(self.handle(), normal_face.handle(), fixed_face.handle(), sizes) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPage(&self, source: &str) {
        let source = wxT(source);
        unsafe { wxHtmlWindow_SetPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRelatedFrame<T: _wxFrame>(&self, frame: &T, format: &str) {
        let format = wxT(format);
        unsafe { wxHtmlWindow_SetRelatedFrame(self.handle(), frame.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.handle(), bar) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlWindow_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxIPV4address(*u8);
impl _wxIPV4address for wxIPV4address {}
impl _wxSockAddress for wxIPV4address {}
impl _wxObject for wxIPV4address { fn handle(&self) -> *u8 { **self } }

impl wxIPV4address {
    pub fn from(handle: *u8) -> @wxIPV4address { @wxIPV4address(handle) }
    pub fn null() -> @wxIPV4address { wxIPV4address::from(0 as *u8) }
    
}

trait _wxIPV4address : _wxSockAddress {
}

struct wxIcon(*u8);
impl _wxIcon for wxIcon {}
impl _wxBitmap for wxIcon {}
impl _wxGDIObject for wxIcon {}
impl _wxObject for wxIcon { fn handle(&self) -> *u8 { **self } }

impl wxIcon {
    pub fn from(handle: *u8) -> @wxIcon { @wxIcon(handle) }
    pub fn null() -> @wxIcon { wxIcon::from(0 as *u8) }
    
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

trait _wxIcon : _wxBitmap {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign(&self, other: *u8) {
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
    fn isEqual(&self, other: &_wxIcon) -> bool {
        unsafe { wxIcon_IsEqual(self.handle(), other.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, name: &str, type_: c_long, width: c_int, height: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxIcon_Load(self.handle(), name.handle(), type_, width, height) }
    }
}

struct wxIconBundle(*u8);
impl _wxIconBundle for wxIconBundle { fn handle(&self) -> *u8 { **self } }

impl wxIconBundle {
    pub fn from(handle: *u8) -> @wxIconBundle { @wxIconBundle(handle) }
    pub fn null() -> @wxIconBundle { wxIconBundle::from(0 as *u8) }
    
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

trait _wxIconBundle {
    fn handle(&self) -> *u8;
    
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

struct wxIconizeEvent(*u8);
impl _wxIconizeEvent for wxIconizeEvent {}
impl _wxEvent for wxIconizeEvent {}
impl _wxObject for wxIconizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxIconizeEvent {
    pub fn from(handle: *u8) -> @wxIconizeEvent { @wxIconizeEvent(handle) }
    pub fn null() -> @wxIconizeEvent { wxIconizeEvent::from(0 as *u8) }
    
}

trait _wxIconizeEvent : _wxEvent {
}

struct wxIdleEvent(*u8);
impl _wxIdleEvent for wxIdleEvent {}
impl _wxEvent for wxIdleEvent {}
impl _wxObject for wxIdleEvent { fn handle(&self) -> *u8 { **self } }

impl wxIdleEvent {
    pub fn from(handle: *u8) -> @wxIdleEvent { @wxIdleEvent(handle) }
    pub fn null() -> @wxIdleEvent { wxIdleEvent::from(0 as *u8) }
    
}

trait _wxIdleEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn moreRequested(&self) -> bool {
        unsafe { wxIdleEvent_MoreRequested(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn requestMore(&self, needMore: bool) {
        unsafe { wxIdleEvent_RequestMore(self.handle(), needMore) }
    }
}

struct wxImage(*u8);
impl _wxImage for wxImage {}
impl _wxObject for wxImage { fn handle(&self) -> *u8 { **self } }

impl wxImage {
    pub fn from(handle: *u8) -> @wxImage { @wxImage(handle) }
    pub fn null() -> @wxImage { wxImage::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn canRead(name: &str) -> bool {
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
    pub fn newFromByteString(data: *char, length: c_int, type_: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromByteString(data, length, type_)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromLazyByteString(data: *char, length: c_int, type_: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromLazyByteString(data, length, type_)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromData(width: c_int, height: c_int, data: *u8) -> @wxImage {
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
    pub fn newFromDataEx(width: c_int, height: c_int, data: *u8, isStaticData: c_int) -> @wxImage {
        unsafe { @wxImage(wxImage_CreateFromDataEx(width, height, data, isStaticData)) }
    }
}

trait _wxImage : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToBitmap<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxImage_ConvertToBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToByteString(&self, type_: c_int, data: *char) -> c_int {
        unsafe { wxImage_ConvertToByteString(self.handle(), type_, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn convertToLazyByteString(&self, type_: c_int, data: *char) -> c_int {
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
    fn getBlue(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetBlue(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getData(&self) -> *u8 {
        unsafe { wxImage_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGreen(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetGreen(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHeight(&self) -> c_int {
        unsafe { wxImage_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskBlue(&self) -> wchar_t {
        unsafe { wxImage_GetMaskBlue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskGreen(&self) -> wchar_t {
        unsafe { wxImage_GetMaskGreen(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaskRed(&self) -> wchar_t {
        unsafe { wxImage_GetMaskRed(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRed(&self, x: c_int, y: c_int) -> wchar_t {
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
    fn hasMask(&self) -> bool {
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
    fn getOptionInt(&self, name: &str) -> bool {
        let name = wxT(name);
        unsafe { wxImage_GetOptionInt(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasOption(&self, name: &str) -> bool {
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
    fn initializeFromData(&self, width: c_int, height: c_int, data: *u8) {
        unsafe { wxImage_InitializeFromData(self.handle(), width, height, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, name: &str, type_: c_int) -> bool {
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
    fn isOk(&self) -> bool {
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
    fn rotate<T: _wxImage>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *u8, image: &T) {
        unsafe { wxImage_Rotate(self.handle(), angle, c_x, c_y, interpolating, offset_after_rotation, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rotate90<T: _wxImage>(&self, clockwise: c_int, image: &T) {
        unsafe { wxImage_Rotate90(self.handle(), clockwise, image.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, name: &str, type_: c_int) -> bool {
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
    fn setData(&self, data: *u8) {
        unsafe { wxImage_SetData(self.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDataAndSize(&self, data: *u8, new_width: c_int, new_height: c_int) {
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

struct wxImageHandler(*u8);
impl _wxImageHandler for wxImageHandler {}
impl _wxObject for wxImageHandler { fn handle(&self) -> *u8 { **self } }

impl wxImageHandler {
    pub fn from(handle: *u8) -> @wxImageHandler { @wxImageHandler(handle) }
    pub fn null() -> @wxImageHandler { wxImageHandler::from(0 as *u8) }
    
}

trait _wxImageHandler : _wxObject {
}

struct wxImageList(*u8);
impl _wxImageList for wxImageList {}
impl _wxObject for wxImageList { fn handle(&self) -> *u8 { **self } }

impl wxImageList {
    pub fn from(handle: *u8) -> @wxImageList { @wxImageList(handle) }
    pub fn null() -> @wxImageList { wxImageList::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> @wxImageList {
        unsafe { @wxImageList(wxImageList_Create(width, height, mask, initialCount)) }
    }
}

trait _wxImageList : _wxObject {
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
    fn draw<T: _wxDC>(&self, index: c_int, dc: &T, x: c_int, y: c_int, flags: c_int, solidBackground: bool) -> bool {
        unsafe { wxImageList_Draw(self.handle(), index, dc.handle(), x, y, flags, solidBackground) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageCount(&self) -> c_int {
        unsafe { wxImageList_GetImageCount(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self, index: c_int, width: *c_int, height: *c_int) {
        unsafe { wxImageList_GetSize(self.handle(), index, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn remove(&self, index: c_int) -> bool {
        unsafe { wxImageList_Remove(self.handle(), index) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeAll(&self) -> bool {
        unsafe { wxImageList_RemoveAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxBitmap, U: _wxBitmap>(&self, index: c_int, bitmap: &T, mask: &U) -> bool {
        unsafe { wxImageList_Replace(self.handle(), index, bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceIcon<T: _wxIcon>(&self, index: c_int, icon: &T) -> bool {
        unsafe { wxImageList_ReplaceIcon(self.handle(), index, icon.handle()) }
    }
}

struct wxIndividualLayoutConstraint(*u8);
impl _wxIndividualLayoutConstraint for wxIndividualLayoutConstraint {}
impl _wxObject for wxIndividualLayoutConstraint { fn handle(&self) -> *u8 { **self } }

impl wxIndividualLayoutConstraint {
    pub fn from(handle: *u8) -> @wxIndividualLayoutConstraint { @wxIndividualLayoutConstraint(handle) }
    pub fn null() -> @wxIndividualLayoutConstraint { wxIndividualLayoutConstraint::from(0 as *u8) }
    
}

trait _wxIndividualLayoutConstraint : _wxObject {
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
    fn getDone(&self) -> bool {
        unsafe { wxIndividualLayoutConstraint_GetDone(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdge(&self, which: c_int, thisWin: *u8, other: *u8) -> c_int {
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
    fn getOtherWindow(&self) -> *u8 {
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
    fn resetIfWin<T: _wxWindow>(&self, otherW: &T) -> bool {
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
    fn satisfyConstraint<T: _wxWindow>(&self, constraints: *u8, win: &T) -> bool {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.handle(), constraints, win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set<T: _wxWindow>(&self, rel: c_int, otherW: &T, otherE: c_int, val: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Set(self.handle(), rel, otherW.handle(), otherE, val, marg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDone(&self, d: bool) {
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

struct wxInitDialogEvent(*u8);
impl _wxInitDialogEvent for wxInitDialogEvent {}
impl _wxEvent for wxInitDialogEvent {}
impl _wxObject for wxInitDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxInitDialogEvent {
    pub fn from(handle: *u8) -> @wxInitDialogEvent { @wxInitDialogEvent(handle) }
    pub fn null() -> @wxInitDialogEvent { wxInitDialogEvent::from(0 as *u8) }
    
}

trait _wxInitDialogEvent : _wxEvent {
}

struct wxInputStream(*u8);
impl _wxInputStream for wxInputStream {}
impl _wxStreamBase for wxInputStream { fn handle(&self) -> *u8 { **self } }

impl wxInputStream {
    pub fn from(handle: *u8) -> @wxInputStream { @wxInputStream(handle) }
    pub fn null() -> @wxInputStream { wxInputStream::from(0 as *u8) }
    
}

trait _wxInputStream : _wxStreamBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn eof(&self) -> bool {
        unsafe { wxInputStream_Eof(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getC(&self) -> wchar_t {
        unsafe { wxInputStream_GetC(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastRead(&self) -> c_int {
        unsafe { wxInputStream_LastRead(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn peek(&self) -> wchar_t {
        unsafe { wxInputStream_Peek(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn read(&self, buffer: *u8, size: c_int) {
        unsafe { wxInputStream_Read(self.handle(), buffer, size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn seekI(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxInputStream_SeekI(self.handle(), pos, mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tell(&self) -> c_int {
        unsafe { wxInputStream_Tell(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ungetBuffer(&self, buffer: *u8, size: c_int) -> c_int {
        unsafe { wxInputStream_UngetBuffer(self.handle(), buffer, size) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ungetch(&self, c: wchar_t) -> c_int {
        unsafe { wxInputStream_Ungetch(self.handle(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canRead(&self) -> bool {
        unsafe { wxInputStream_CanRead(self.handle()) }
    }
}

struct wxJoystick(*u8);
impl _wxJoystick for wxJoystick {}
impl _wxObject for wxJoystick { fn handle(&self) -> *u8 { **self } }

impl wxJoystick {
    pub fn from(handle: *u8) -> @wxJoystick { @wxJoystick(handle) }
    pub fn null() -> @wxJoystick { wxJoystick::from(0 as *u8) }
    
}

trait _wxJoystick : _wxObject {
}

struct wxJoystickEvent(*u8);
impl _wxJoystickEvent for wxJoystickEvent {}
impl _wxEvent for wxJoystickEvent {}
impl _wxObject for wxJoystickEvent { fn handle(&self) -> *u8 { **self } }

impl wxJoystickEvent {
    pub fn from(handle: *u8) -> @wxJoystickEvent { @wxJoystickEvent(handle) }
    pub fn null() -> @wxJoystickEvent { wxJoystickEvent::from(0 as *u8) }
    
}

trait _wxJoystickEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDown(&self, but: c_int) -> bool {
        unsafe { wxJoystickEvent_ButtonDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonIsDown(&self, but: c_int) -> bool {
        unsafe { wxJoystickEvent_ButtonIsDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonUp(&self, but: c_int) -> bool {
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
    fn isButton(&self) -> bool {
        unsafe { wxJoystickEvent_IsButton(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMove(&self) -> bool {
        unsafe { wxJoystickEvent_IsMove(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isZMove(&self) -> bool {
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

struct wxKeyEvent(*u8);
impl _wxKeyEvent for wxKeyEvent {}
impl _wxEvent for wxKeyEvent {}
impl _wxObject for wxKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxKeyEvent {
    pub fn from(handle: *u8) -> @wxKeyEvent { @wxKeyEvent(handle) }
    pub fn null() -> @wxKeyEvent { wxKeyEvent::from(0 as *u8) }
    
}

trait _wxKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> bool {
        unsafe { wxKeyEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> bool {
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
    fn hasModifiers(&self) -> bool {
        unsafe { wxKeyEvent_HasModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> bool {
        unsafe { wxKeyEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setKeyCode(&self, code: c_int) {
        unsafe { wxKeyEvent_SetKeyCode(self.handle(), code) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> bool {
        unsafe { wxKeyEvent_ShiftDown(self.handle()) }
    }
}

struct wxLEDNumberCtrl(*u8);
impl _wxLEDNumberCtrl for wxLEDNumberCtrl {}
impl _wxControl for wxLEDNumberCtrl {}
impl _wxWindow for wxLEDNumberCtrl {}
impl _wxEvtHandler for wxLEDNumberCtrl {}
impl _wxObject for wxLEDNumberCtrl { fn handle(&self) -> *u8 { **self } }

impl wxLEDNumberCtrl {
    pub fn from(handle: *u8) -> @wxLEDNumberCtrl { @wxLEDNumberCtrl(handle) }
    pub fn null() -> @wxLEDNumberCtrl { wxLEDNumberCtrl::from(0 as *u8) }
    
}

trait _wxLEDNumberCtrl : _wxControl {
}

struct wxLayoutAlgorithm(*u8);
impl _wxLayoutAlgorithm for wxLayoutAlgorithm {}
impl _wxObject for wxLayoutAlgorithm { fn handle(&self) -> *u8 { **self } }

impl wxLayoutAlgorithm {
    pub fn from(handle: *u8) -> @wxLayoutAlgorithm { @wxLayoutAlgorithm(handle) }
    pub fn null() -> @wxLayoutAlgorithm { wxLayoutAlgorithm::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxLayoutAlgorithm {
        unsafe { @wxLayoutAlgorithm(wxLayoutAlgorithm_Create()) }
    }
}

trait _wxLayoutAlgorithm : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutFrame<T: _wxFrame>(&self, frame: &T, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.handle(), frame.handle(), mainWindow) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutMDIFrame<T: _wxFrame>(&self, frame: &T, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.handle(), frame.handle(), x, y, w, h, use_) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutWindow<T: _wxFrame>(&self, frame: &T, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.handle(), frame.handle(), mainWindow) }
    }
}

struct wxLayoutConstraints(*u8);
impl _wxLayoutConstraints for wxLayoutConstraints {}
impl _wxObject for wxLayoutConstraints { fn handle(&self) -> *u8 { **self } }

impl wxLayoutConstraints {
    pub fn from(handle: *u8) -> @wxLayoutConstraints { @wxLayoutConstraints(handle) }
    pub fn null() -> @wxLayoutConstraints { wxLayoutConstraints::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxLayoutConstraints {
        unsafe { @wxLayoutConstraints(wxLayoutConstraints_Create()) }
    }
}

trait _wxLayoutConstraints : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn bottom(&self) -> *u8 {
        unsafe { wxLayoutConstraints_bottom(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centreX(&self) -> *u8 {
        unsafe { wxLayoutConstraints_centreX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn centreY(&self) -> *u8 {
        unsafe { wxLayoutConstraints_centreY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn height(&self) -> *u8 {
        unsafe { wxLayoutConstraints_height(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn left(&self) -> *u8 {
        unsafe { wxLayoutConstraints_left(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn right(&self) -> *u8 {
        unsafe { wxLayoutConstraints_right(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn top(&self) -> *u8 {
        unsafe { wxLayoutConstraints_top(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn width(&self) -> *u8 {
        unsafe { wxLayoutConstraints_width(self.handle()) }
    }
}

struct wxList(*u8);
impl _wxList for wxList {}
impl _wxObject for wxList { fn handle(&self) -> *u8 { **self } }

impl wxList {
    pub fn from(handle: *u8) -> @wxList { @wxList(handle) }
    pub fn null() -> @wxList { wxList::from(0 as *u8) }
    
}

trait _wxList : _wxObject {
}

struct wxListBox(*u8);
impl _wxListBox for wxListBox {}
impl _wxControl for wxListBox {}
impl _wxWindow for wxListBox {}
impl _wxEvtHandler for wxListBox {}
impl _wxObject for wxListBox { fn handle(&self) -> *u8 { **self } }

impl wxListBox {
    pub fn from(handle: *u8) -> @wxListBox { @wxListBox(handle) }
    pub fn null() -> @wxListBox { wxListBox::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxListBox {
        unsafe { @wxListBox(wxListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxListBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, item: &str) {
        let item = wxT(item);
        unsafe { wxListBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendData(&self, item: &str, data: *u8) {
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
    fn getSelections(&self, aSelections: *c_int, allocated: c_int) -> c_int {
        unsafe { wxListBox_GetSelections(self.handle(), aSelections, allocated) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, n: c_int) -> ~str {
        unsafe { wxString { handle: wxListBox_GetString(self.handle(), n) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItems(&self, items: *u8, pos: c_int, count: c_int) {
        unsafe { wxListBox_InsertItems(self.handle(), items, pos, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelected(&self, n: c_int) -> bool {
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
    fn setStringSelection(&self, str: &str, sel: bool) {
        let str = wxT(str);
        unsafe { wxListBox_SetStringSelection(self.handle(), str.handle(), sel) }
    }
}

struct wxListCtrl(*u8);
impl _wxListCtrl for wxListCtrl {}
impl _wxControl for wxListCtrl {}
impl _wxWindow for wxListCtrl {}
impl _wxEvtHandler for wxListCtrl {}
impl _wxObject for wxListCtrl { fn handle(&self) -> *u8 { **self } }

impl wxListCtrl {
    pub fn from(handle: *u8) -> @wxListCtrl { @wxListCtrl(handle) }
    pub fn null() -> @wxListCtrl { wxListCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxListCtrl {
        unsafe { @wxListCtrl(wxListCtrl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxListCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn arrange(&self, flag: c_int) -> bool {
        unsafe { wxListCtrl_Arrange(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllColumns(&self) -> bool {
        unsafe { wxListCtrl_DeleteAllColumns(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllItems(&self) -> bool {
        unsafe { wxListCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteColumn(&self, col: c_int) -> bool {
        unsafe { wxListCtrl_DeleteColumn(self.handle(), col) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteItem(&self, item: c_int) -> bool {
        unsafe { wxListCtrl_DeleteItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn editLabel(&self, item: c_int) {
        unsafe { wxListCtrl_EditLabel(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn endEditLabel(&self, cancel: c_int) -> bool {
        unsafe { wxListCtrl_EndEditLabel(self.handle(), cancel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn ensureVisible(&self, item: c_int) -> bool {
        unsafe { wxListCtrl_EnsureVisible(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn findItem(&self, start: c_int, str: &str, partial: bool) -> c_int {
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
    fn getColumn<T: _wxListItem>(&self, col: c_int, item: &T) -> bool {
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
    fn getItem<T: _wxListItem>(&self, info: &T) -> bool {
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
    fn getItemSpacing(&self, isSmall: bool) -> @wxSize {
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
    fn hitTest(&self, x: c_int, y: c_int, flags: *u8) -> c_int {
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
    fn isVirtual(&self) -> bool {
        unsafe { wxListCtrl_IsVirtual(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refreshItem(&self, item: c_long) {
        unsafe { wxListCtrl_RefreshItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn scrollList(&self, dx: c_int, dy: c_int) -> bool {
        unsafe { wxListCtrl_ScrollList(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumn<T: _wxListItem>(&self, col: c_int, item: &T) -> bool {
        unsafe { wxListCtrl_SetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> bool {
        unsafe { wxListCtrl_SetColumnWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setImageList<T: _wxImageList>(&self, imageList: &T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.handle(), imageList.handle(), which) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItem(&self, index: c_int, col: c_int, label: &str, imageId: c_int) -> bool {
        let label = wxT(label);
        unsafe { wxListCtrl_SetItem(self.handle(), index, col, label.handle(), imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemData(&self, item: c_int, data: c_int) -> bool {
        unsafe { wxListCtrl_SetItemData(self.handle(), item, data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemFromInfo<T: _wxListItem>(&self, info: &T) -> bool {
        unsafe { wxListCtrl_SetItemFromInfo(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemImage(&self, item: c_int, image: c_int, selImage: c_int) -> bool {
        unsafe { wxListCtrl_SetItemImage(self.handle(), item, image, selImage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemPosition(&self, item: c_int, x: c_int, y: c_int) -> bool {
        unsafe { wxListCtrl_SetItemPosition(self.handle(), item, x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemState(&self, item: c_int, state: c_int, stateMask: c_int) -> bool {
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
    fn setSingleStyle(&self, style: c_int, add: bool) {
        unsafe { wxListCtrl_SetSingleStyle(self.handle(), style, add) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTextColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxListCtrl_SetTextColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sortItems(&self, fn_: *u8, eif_obj: *u8) -> bool {
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
    fn sortItems2<T: _wxClosure>(&self, closure: &T) -> bool {
        unsafe { wxListCtrl_SortItems2(self.handle(), closure.handle()) }
    }
}

struct wxListEvent(*u8);
impl _wxListEvent for wxListEvent {}
impl _wxNotifyEvent for wxListEvent {}
impl _wxCommandEvent for wxListEvent {}
impl _wxEvent for wxListEvent {}
impl _wxObject for wxListEvent { fn handle(&self) -> *u8 { **self } }

impl wxListEvent {
    pub fn from(handle: *u8) -> @wxListEvent { @wxListEvent(handle) }
    pub fn null() -> @wxListEvent { wxListEvent::from(0 as *u8) }
    
}

trait _wxListEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn cancelled(&self) -> bool {
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

struct wxListItem(*u8);
impl _wxListItem for wxListItem {}
impl _wxObject for wxListItem { fn handle(&self) -> *u8 { **self } }

impl wxListItem {
    pub fn from(handle: *u8) -> @wxListItem { @wxListItem(handle) }
    pub fn null() -> @wxListItem { wxListItem::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxListItem {
        unsafe { @wxListItem(wxListItem_Create()) }
    }
}

trait _wxListItem : _wxObject {
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
    fn getAttributes(&self) -> *u8 {
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
    fn hasAttributes(&self) -> bool {
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
    fn setDataPointer(&self, data: *u8) {
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

struct wxLocale(*u8);
impl _wxLocale for wxLocale { fn handle(&self) -> *u8 { **self } }

impl wxLocale {
    pub fn from(handle: *u8) -> @wxLocale { @wxLocale(handle) }
    pub fn null() -> @wxLocale { wxLocale::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_name: c_int, _flags: c_int) -> @wxLocale {
        unsafe { @wxLocale(wxLocale_Create(_name, _flags)) }
    }
}

trait _wxLocale {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCatalog(&self, szDomain: *u8) -> c_int {
        unsafe { wxLocale_AddCatalog(self.handle(), szDomain) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addCatalogLookupPathPrefix(&self, prefix: *u8) {
        unsafe { wxLocale_AddCatalogLookupPathPrefix(self.handle(), prefix) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxLocale_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLocale(&self) -> @wxLocale {
        unsafe { @wxLocale(wxLocale_GetLocale(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { handle: wxLocale_GetName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getString(&self, szOrigString: *u8, szDomain: *u8) -> *wchar_t {
        unsafe { wxLocale_GetString(self.handle(), szOrigString, szDomain) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isLoaded(&self, szDomain: *u8) -> bool {
        unsafe { wxLocale_IsLoaded(self.handle(), szDomain) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxLocale_IsOk(self.handle()) }
    }
}

struct wxLog(*u8);
impl _wxLog for wxLog { fn handle(&self) -> *u8 { **self } }

impl wxLog {
    pub fn from(handle: *u8) -> @wxLog { @wxLog(handle) }
    pub fn null() -> @wxLog { wxLog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getActiveTarget() -> @wxLog {
        unsafe { @wxLog(wxLog_GetActiveTarget()) }
    }
}

trait _wxLog {
    fn handle(&self) -> *u8;
    
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
    fn getTimestamp(&self) -> *char {
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
    fn hasPendingMessages(&self) -> bool {
        unsafe { wxLog_HasPendingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAllowedTraceMask<T: _wxMask>(&self, mask: &T) -> bool {
        unsafe { wxLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn onLog(&self, level: c_int, szString: *wchar_t, t: c_int) {
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
    fn setTimestamp(&self, ts: *wchar_t) {
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

struct wxLogChain(*u8);
impl _wxLogChain for wxLogChain {}
impl _wxLog for wxLogChain { fn handle(&self) -> *u8 { **self } }

impl wxLogChain {
    pub fn from(handle: *u8) -> @wxLogChain { @wxLogChain(handle) }
    pub fn null() -> @wxLogChain { wxLogChain::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxLog>(logger: &T) -> @wxLogChain {
        unsafe { @wxLogChain(wxLogChain_Create(logger.handle())) }
    }
}

trait _wxLogChain : _wxLog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOldLog(&self) -> @wxLog {
        unsafe { @wxLog(wxLogChain_GetOldLog(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isPassingMessages(&self) -> bool {
        unsafe { wxLogChain_IsPassingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn passMessages(&self, bDoPass: bool) {
        unsafe { wxLogChain_PassMessages(self.handle(), bDoPass) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLog<T: _wxLog>(&self, logger: &T) {
        unsafe { wxLogChain_SetLog(self.handle(), logger.handle()) }
    }
}

struct wxLogGUI(*u8);
impl _wxLogGUI for wxLogGUI {}
impl _wxLog for wxLogGUI { fn handle(&self) -> *u8 { **self } }

impl wxLogGUI {
    pub fn from(handle: *u8) -> @wxLogGUI { @wxLogGUI(handle) }
    pub fn null() -> @wxLogGUI { wxLogGUI::from(0 as *u8) }
    
}

trait _wxLogGUI : _wxLog {
}

struct wxLogNull(*u8);
impl _wxLogNull for wxLogNull {}
impl _wxLog for wxLogNull { fn handle(&self) -> *u8 { **self } }

impl wxLogNull {
    pub fn from(handle: *u8) -> @wxLogNull { @wxLogNull(handle) }
    pub fn null() -> @wxLogNull { wxLogNull::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxLogNull {
        unsafe { @wxLogNull(wxLogNull_Create()) }
    }
}

trait _wxLogNull : _wxLog {
}

struct wxLogPassThrough(*u8);
impl _wxLogPassThrough for wxLogPassThrough {}
impl _wxLogChain for wxLogPassThrough {}
impl _wxLog for wxLogPassThrough { fn handle(&self) -> *u8 { **self } }

impl wxLogPassThrough {
    pub fn from(handle: *u8) -> @wxLogPassThrough { @wxLogPassThrough(handle) }
    pub fn null() -> @wxLogPassThrough { wxLogPassThrough::from(0 as *u8) }
    
}

trait _wxLogPassThrough : _wxLogChain {
}

struct wxLogStderr(*u8);
impl _wxLogStderr for wxLogStderr {}
impl _wxLog for wxLogStderr { fn handle(&self) -> *u8 { **self } }

impl wxLogStderr {
    pub fn from(handle: *u8) -> @wxLogStderr { @wxLogStderr(handle) }
    pub fn null() -> @wxLogStderr { wxLogStderr::from(0 as *u8) }
    
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

trait _wxLogStderr : _wxLog {
}

struct wxLogStream(*u8);
impl _wxLogStream for wxLogStream {}
impl _wxLog for wxLogStream { fn handle(&self) -> *u8 { **self } }

impl wxLogStream {
    pub fn from(handle: *u8) -> @wxLogStream { @wxLogStream(handle) }
    pub fn null() -> @wxLogStream { wxLogStream::from(0 as *u8) }
    
}

trait _wxLogStream : _wxLog {
}

struct wxLogTextCtrl(*u8);
impl _wxLogTextCtrl for wxLogTextCtrl {}
impl _wxLog for wxLogTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxLogTextCtrl {
    pub fn from(handle: *u8) -> @wxLogTextCtrl { @wxLogTextCtrl(handle) }
    pub fn null() -> @wxLogTextCtrl { wxLogTextCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxTextCtrl>(text: &T) -> @wxLogTextCtrl {
        unsafe { @wxLogTextCtrl(wxLogTextCtrl_Create(text.handle())) }
    }
}

trait _wxLogTextCtrl : _wxLog {
}

struct wxLogWindow(*u8);
impl _wxLogWindow for wxLogWindow {}
impl _wxLogPassThrough for wxLogWindow {}
impl _wxLogChain for wxLogWindow {}
impl _wxLog for wxLogWindow { fn handle(&self) -> *u8 { **self } }

impl wxLogWindow {
    pub fn from(handle: *u8) -> @wxLogWindow { @wxLogWindow(handle) }
    pub fn null() -> @wxLogWindow { wxLogWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, title: *wchar_t, showit: bool, passthrough: bool) -> @wxLogWindow {
        unsafe { @wxLogWindow(wxLogWindow_Create(parent.handle(), title, showit, passthrough)) }
    }
}

trait _wxLogWindow : _wxLogPassThrough {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrame(wxLogWindow_GetFrame(self.handle())) }
    }
}

struct wxLongLong(*u8);
impl _wxLongLong for wxLongLong { fn handle(&self) -> *u8 { **self } }

impl wxLongLong {
    pub fn from(handle: *u8) -> @wxLongLong { @wxLongLong(handle) }
    pub fn null() -> @wxLongLong { wxLongLong::from(0 as *u8) }
    
}

trait _wxLongLong {
    fn handle(&self) -> *u8;
    
}

struct wxMBConv(*u8);
impl _wxMBConv for wxMBConv { fn handle(&self) -> *u8 { **self } }

impl wxMBConv {
    pub fn from(handle: *u8) -> @wxMBConv { @wxMBConv(handle) }
    pub fn null() -> @wxMBConv { wxMBConv::from(0 as *u8) }
    
}

trait _wxMBConv {
    fn handle(&self) -> *u8;
    
}

struct wxMBConvFile(*u8);
impl _wxMBConvFile for wxMBConvFile {}
impl _wxMBConv for wxMBConvFile { fn handle(&self) -> *u8 { **self } }

impl wxMBConvFile {
    pub fn from(handle: *u8) -> @wxMBConvFile { @wxMBConvFile(handle) }
    pub fn null() -> @wxMBConvFile { wxMBConvFile::from(0 as *u8) }
    
}

trait _wxMBConvFile : _wxMBConv {
}

struct wxMBConvUTF7(*u8);
impl _wxMBConvUTF7 for wxMBConvUTF7 {}
impl _wxMBConv for wxMBConvUTF7 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF7 {
    pub fn from(handle: *u8) -> @wxMBConvUTF7 { @wxMBConvUTF7(handle) }
    pub fn null() -> @wxMBConvUTF7 { wxMBConvUTF7::from(0 as *u8) }
    
}

trait _wxMBConvUTF7 : _wxMBConv {
}

struct wxMBConvUTF8(*u8);
impl _wxMBConvUTF8 for wxMBConvUTF8 {}
impl _wxMBConv for wxMBConvUTF8 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF8 {
    pub fn from(handle: *u8) -> @wxMBConvUTF8 { @wxMBConvUTF8(handle) }
    pub fn null() -> @wxMBConvUTF8 { wxMBConvUTF8::from(0 as *u8) }
    
}

trait _wxMBConvUTF8 : _wxMBConv {
}

struct wxMDIChildFrame(*u8);
impl _wxMDIChildFrame for wxMDIChildFrame {}
impl _wxFrame for wxMDIChildFrame {}
impl _wxTopLevelWindow for wxMDIChildFrame {}
impl _wxWindow for wxMDIChildFrame {}
impl _wxEvtHandler for wxMDIChildFrame {}
impl _wxObject for wxMDIChildFrame { fn handle(&self) -> *u8 { **self } }

impl wxMDIChildFrame {
    pub fn from(handle: *u8) -> @wxMDIChildFrame { @wxMDIChildFrame(handle) }
    pub fn null() -> @wxMDIChildFrame { wxMDIChildFrame::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMDIChildFrame {
        let _txt = wxT(_txt);
        unsafe { @wxMDIChildFrame(wxMDIChildFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxMDIChildFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.handle()) }
    }
}

struct wxMDIClientWindow(*u8);
impl _wxMDIClientWindow for wxMDIClientWindow {}
impl _wxWindow for wxMDIClientWindow {}
impl _wxEvtHandler for wxMDIClientWindow {}
impl _wxObject for wxMDIClientWindow { fn handle(&self) -> *u8 { **self } }

impl wxMDIClientWindow {
    pub fn from(handle: *u8) -> @wxMDIClientWindow { @wxMDIClientWindow(handle) }
    pub fn null() -> @wxMDIClientWindow { wxMDIClientWindow::from(0 as *u8) }
    
}

trait _wxMDIClientWindow : _wxWindow {
}

struct wxMDIParentFrame(*u8);
impl _wxMDIParentFrame for wxMDIParentFrame {}
impl _wxFrame for wxMDIParentFrame {}
impl _wxTopLevelWindow for wxMDIParentFrame {}
impl _wxWindow for wxMDIParentFrame {}
impl _wxEvtHandler for wxMDIParentFrame {}
impl _wxObject for wxMDIParentFrame { fn handle(&self) -> *u8 { **self } }

impl wxMDIParentFrame {
    pub fn from(handle: *u8) -> @wxMDIParentFrame { @wxMDIParentFrame(handle) }
    pub fn null() -> @wxMDIParentFrame { wxMDIParentFrame::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMDIParentFrame {
        let _txt = wxT(_txt);
        unsafe { @wxMDIParentFrame(wxMDIParentFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxMDIParentFrame : _wxFrame {
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

struct wxMask(*u8);
impl _wxMask for wxMask {}
impl _wxObject for wxMask { fn handle(&self) -> *u8 { **self } }

impl wxMask {
    pub fn from(handle: *u8) -> @wxMask { @wxMask(handle) }
    pub fn null() -> @wxMask { wxMask::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxBitmap>(bitmap: &T) -> @wxMask {
        unsafe { @wxMask(wxMask_Create(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newColoured<T: _wxBitmap, U: _wxColour>(bitmap: &T, colour: &U) -> *u8 {
        unsafe { wxMask_CreateColoured(bitmap.handle(), colour.handle()) }
    }
}

trait _wxMask : _wxObject {
}

struct wxMaximizeEvent(*u8);
impl _wxMaximizeEvent for wxMaximizeEvent {}
impl _wxEvent for wxMaximizeEvent {}
impl _wxObject for wxMaximizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxMaximizeEvent {
    pub fn from(handle: *u8) -> @wxMaximizeEvent { @wxMaximizeEvent(handle) }
    pub fn null() -> @wxMaximizeEvent { wxMaximizeEvent::from(0 as *u8) }
    
}

trait _wxMaximizeEvent : _wxEvent {
}

struct wxMemoryDC(*u8);
impl _wxMemoryDC for wxMemoryDC {}
impl _wxDC for wxMemoryDC {}
impl _wxObject for wxMemoryDC { fn handle(&self) -> *u8 { **self } }

impl wxMemoryDC {
    pub fn from(handle: *u8) -> @wxMemoryDC { @wxMemoryDC(handle) }
    pub fn null() -> @wxMemoryDC { wxMemoryDC::from(0 as *u8) }
    
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

trait _wxMemoryDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectObject<T: _wxBitmap>(&self, bitmap: &T) {
        unsafe { wxMemoryDC_SelectObject(self.handle(), bitmap.handle()) }
    }
}

struct wxMemoryFSHandler(*u8);
impl _wxMemoryFSHandler for wxMemoryFSHandler {}
impl _wxFileSystemHandler for wxMemoryFSHandler {}
impl _wxObject for wxMemoryFSHandler { fn handle(&self) -> *u8 { **self } }

impl wxMemoryFSHandler {
    pub fn from(handle: *u8) -> @wxMemoryFSHandler { @wxMemoryFSHandler(handle) }
    pub fn null() -> @wxMemoryFSHandler { wxMemoryFSHandler::from(0 as *u8) }
    
}

trait _wxMemoryFSHandler : _wxFileSystemHandler {
}

struct wxMemoryInputStream(*u8);
impl _wxMemoryInputStream for wxMemoryInputStream {}
impl _wxInputStream for wxMemoryInputStream {}
impl _wxStreamBase for wxMemoryInputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryInputStream {
    pub fn from(handle: *u8) -> @wxMemoryInputStream { @wxMemoryInputStream(handle) }
    pub fn null() -> @wxMemoryInputStream { wxMemoryInputStream::from(0 as *u8) }
    
}

trait _wxMemoryInputStream : _wxInputStream {
}

struct wxMemoryOutputStream(*u8);
impl _wxMemoryOutputStream for wxMemoryOutputStream {}
impl _wxOutputStream for wxMemoryOutputStream {}
impl _wxStreamBase for wxMemoryOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryOutputStream {
    pub fn from(handle: *u8) -> @wxMemoryOutputStream { @wxMemoryOutputStream(handle) }
    pub fn null() -> @wxMemoryOutputStream { wxMemoryOutputStream::from(0 as *u8) }
    
}

trait _wxMemoryOutputStream : _wxOutputStream {
}

struct wxMenu(*u8);
impl _wxMenu for wxMenu {}
impl _wxEvtHandler for wxMenu {}
impl _wxObject for wxMenu { fn handle(&self) -> *u8 { **self } }

impl wxMenu {
    pub fn from(handle: *u8) -> @wxMenu { @wxMenu(handle) }
    pub fn null() -> @wxMenu { wxMenu::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(title: &str, style: c_long) -> @wxMenu {
        let title = wxT(title);
        unsafe { @wxMenu(wxMenu_Create(title.handle(), style)) }
    }
}

trait _wxMenu : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append(&self, id: c_int, text: &str, help: &str, isCheckable: bool) {
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
    fn check(&self, id: c_int, check: bool) {
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
    fn enable(&self, id: c_int, enable: bool) {
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
    fn insert(&self, pos: size_t, id: c_int, text: &str, help: &str, isCheckable: bool) {
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
    fn isAttached(&self) -> bool {
        unsafe { wxMenu_IsAttached(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self, id: c_int) -> bool {
        unsafe { wxMenu_IsChecked(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self, id: c_int) -> bool {
        unsafe { wxMenu_IsEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepend(&self, id: c_int, text: &str, help: &str, isCheckable: bool) {
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
    fn removeByItem(&self, item: *u8) {
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
    fn updateUI(&self, source: *u8) {
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

struct wxMenuBar(*u8);
impl _wxMenuBar for wxMenuBar {}
impl _wxEvtHandler for wxMenuBar {}
impl _wxObject for wxMenuBar { fn handle(&self) -> *u8 { **self } }

impl wxMenuBar {
    pub fn from(handle: *u8) -> @wxMenuBar { @wxMenuBar(handle) }
    pub fn null() -> @wxMenuBar { wxMenuBar::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_style: c_int) -> @wxMenuBar {
        unsafe { @wxMenuBar(wxMenuBar_Create(_style)) }
    }
}

trait _wxMenuBar : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append<T: _wxMenu>(&self, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Append(self.handle(), menu.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, id: c_int, check: bool) {
        unsafe { wxMenuBar_Check(self.handle(), id, check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePointer(&self) {
        unsafe { wxMenuBar_DeletePointer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: bool) -> c_int {
        unsafe { wxMenuBar_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableItem(&self, id: c_int, enable: bool) {
        unsafe { wxMenuBar_EnableItem(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableTop(&self, pos: c_int, enable: bool) {
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
    fn isChecked(&self, id: c_int) -> bool {
        unsafe { wxMenuBar_IsChecked(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self, id: c_int) -> bool {
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

struct wxMenuEvent(*u8);
impl _wxMenuEvent for wxMenuEvent {}
impl _wxEvent for wxMenuEvent {}
impl _wxObject for wxMenuEvent { fn handle(&self) -> *u8 { **self } }

impl wxMenuEvent {
    pub fn from(handle: *u8) -> @wxMenuEvent { @wxMenuEvent(handle) }
    pub fn null() -> @wxMenuEvent { wxMenuEvent::from(0 as *u8) }
    
}

trait _wxMenuEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.handle()) }
    }
}

struct wxMenuItem(*u8);
impl _wxMenuItem for wxMenuItem {}
impl _wxObject for wxMenuItem { fn handle(&self) -> *u8 { **self } }

impl wxMenuItem {
    pub fn from(handle: *u8) -> @wxMenuItem { @wxMenuItem(handle) }
    pub fn null() -> @wxMenuItem { wxMenuItem::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxMenuItem {
        unsafe { @wxMenuItem(wxMenuItem_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getLabelFromText(text: *wchar_t) -> ~str {
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

trait _wxMenuItem : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, check: bool) {
        unsafe { wxMenuItem_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: bool) {
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
    fn isCheckable(&self) -> bool {
        unsafe { wxMenuItem_IsCheckable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isChecked(&self) -> bool {
        unsafe { wxMenuItem_IsChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> bool {
        unsafe { wxMenuItem_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSeparator(&self) -> bool {
        unsafe { wxMenuItem_IsSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSubMenu(&self) -> bool {
        unsafe { wxMenuItem_IsSubMenu(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCheckable(&self, checkable: bool) {
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

struct wxMessageDialog(*u8);
impl _wxMessageDialog for wxMessageDialog {}
impl _wxDialog for wxMessageDialog {}
impl _wxTopLevelWindow for wxMessageDialog {}
impl _wxWindow for wxMessageDialog {}
impl _wxEvtHandler for wxMessageDialog {}
impl _wxObject for wxMessageDialog { fn handle(&self) -> *u8 { **self } }

impl wxMessageDialog {
    pub fn from(handle: *u8) -> @wxMessageDialog { @wxMessageDialog(handle) }
    pub fn null() -> @wxMessageDialog { wxMessageDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _msg: &str, _cap: &str, _stl: c_int) -> @wxMessageDialog {
        let _msg = wxT(_msg);
        let _cap = wxT(_cap);
        unsafe { @wxMessageDialog(wxMessageDialog_Create(_prt.handle(), _msg.handle(), _cap.handle(), _stl)) }
    }
}

trait _wxMessageDialog : _wxDialog {
}

struct wxMetafile(*u8);
impl _wxMetafile for wxMetafile {}
impl _wxObject for wxMetafile { fn handle(&self) -> *u8 { **self } }

impl wxMetafile {
    pub fn from(handle: *u8) -> @wxMetafile { @wxMetafile(handle) }
    pub fn null() -> @wxMetafile { wxMetafile::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_file: &str) -> @wxMetafile {
        let _file = wxT(_file);
        unsafe { @wxMetafile(wxMetafile_Create(_file.handle())) }
    }
}

trait _wxMetafile : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxMetafile_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play<T: _wxDC>(&self, _dc: &T) -> bool {
        unsafe { wxMetafile_Play(self.handle(), _dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClipboard(&self, width: c_int, height: c_int) -> bool {
        unsafe { wxMetafile_SetClipboard(self.handle(), width, height) }
    }
}

struct wxMetafileDC(*u8);
impl _wxMetafileDC for wxMetafileDC {}
impl _wxDC for wxMetafileDC {}
impl _wxObject for wxMetafileDC { fn handle(&self) -> *u8 { **self } }

impl wxMetafileDC {
    pub fn from(handle: *u8) -> @wxMetafileDC { @wxMetafileDC(handle) }
    pub fn null() -> @wxMetafileDC { wxMetafileDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_file: &str) -> @wxMetafileDC {
        let _file = wxT(_file);
        unsafe { @wxMetafileDC(wxMetafileDC_Create(_file.handle())) }
    }
}

trait _wxMetafileDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn close(&self) -> *u8 {
        unsafe { wxMetafileDC_Close(self.handle()) }
    }
}

struct wxMimeTypesManager(*u8);
impl _wxMimeTypesManager for wxMimeTypesManager { fn handle(&self) -> *u8 { **self } }

impl wxMimeTypesManager {
    pub fn from(handle: *u8) -> @wxMimeTypesManager { @wxMimeTypesManager(handle) }
    pub fn null() -> @wxMimeTypesManager { wxMimeTypesManager::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxMimeTypesManager {
        unsafe { @wxMimeTypesManager(wxMimeTypesManager_Create()) }
    }
}

trait _wxMimeTypesManager {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn addFallbacks(&self, _types: *u8) {
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
    fn isOfType(&self, _type: &str, _wildcard: &str) -> bool {
        let _type = wxT(_type);
        let _wildcard = wxT(_wildcard);
        unsafe { wxMimeTypesManager_IsOfType(self.handle(), _type.handle(), _wildcard.handle()) }
    }
}

struct wxMiniFrame(*u8);
impl _wxMiniFrame for wxMiniFrame {}
impl _wxFrame for wxMiniFrame {}
impl _wxTopLevelWindow for wxMiniFrame {}
impl _wxWindow for wxMiniFrame {}
impl _wxEvtHandler for wxMiniFrame {}
impl _wxObject for wxMiniFrame { fn handle(&self) -> *u8 { **self } }

impl wxMiniFrame {
    pub fn from(handle: *u8) -> @wxMiniFrame { @wxMiniFrame(handle) }
    pub fn null() -> @wxMiniFrame { wxMiniFrame::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMiniFrame {
        let _txt = wxT(_txt);
        unsafe { @wxMiniFrame(wxMiniFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxMiniFrame : _wxFrame {
}

struct wxMirrorDC(*u8);
impl _wxMirrorDC for wxMirrorDC {}
impl _wxDC for wxMirrorDC {}
impl _wxObject for wxMirrorDC { fn handle(&self) -> *u8 { **self } }

impl wxMirrorDC {
    pub fn from(handle: *u8) -> @wxMirrorDC { @wxMirrorDC(handle) }
    pub fn null() -> @wxMirrorDC { wxMirrorDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxDC>(dc: &T) -> @wxMirrorDC {
        unsafe { @wxMirrorDC(wxMirrorDC_Create(dc.handle())) }
    }
}

trait _wxMirrorDC : _wxDC {
}

struct wxModule(*u8);
impl _wxModule for wxModule {}
impl _wxObject for wxModule { fn handle(&self) -> *u8 { **self } }

impl wxModule {
    pub fn from(handle: *u8) -> @wxModule { @wxModule(handle) }
    pub fn null() -> @wxModule { wxModule::from(0 as *u8) }
    
}

trait _wxModule : _wxObject {
}

struct wxMouseCaptureChangedEvent(*u8);
impl _wxMouseCaptureChangedEvent for wxMouseCaptureChangedEvent {}
impl _wxEvent for wxMouseCaptureChangedEvent {}
impl _wxObject for wxMouseCaptureChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseCaptureChangedEvent {
    pub fn from(handle: *u8) -> @wxMouseCaptureChangedEvent { @wxMouseCaptureChangedEvent(handle) }
    pub fn null() -> @wxMouseCaptureChangedEvent { wxMouseCaptureChangedEvent::from(0 as *u8) }
    
}

trait _wxMouseCaptureChangedEvent : _wxEvent {
}

struct wxMouseEvent(*u8);
impl _wxMouseEvent for wxMouseEvent {}
impl _wxEvent for wxMouseEvent {}
impl _wxObject for wxMouseEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseEvent {
    pub fn from(handle: *u8) -> @wxMouseEvent { @wxMouseEvent(handle) }
    pub fn null() -> @wxMouseEvent { wxMouseEvent::from(0 as *u8) }
    
}

trait _wxMouseEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn altDown(&self) -> bool {
        unsafe { wxMouseEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn button(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_Button(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDClick(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonDClick(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonDown(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonIsDown(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonIsDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn buttonUp(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonUp(self.handle(), but) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn controlDown(&self) -> bool {
        unsafe { wxMouseEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn dragging(&self) -> bool {
        unsafe { wxMouseEvent_Dragging(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn entering(&self) -> bool {
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
    fn isButton(&self) -> bool {
        unsafe { wxMouseEvent_IsButton(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leaving(&self) -> bool {
        unsafe { wxMouseEvent_Leaving(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftDClick(&self) -> bool {
        unsafe { wxMouseEvent_LeftDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftDown(&self) -> bool {
        unsafe { wxMouseEvent_LeftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftIsDown(&self) -> bool {
        unsafe { wxMouseEvent_LeftIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn leftUp(&self) -> bool {
        unsafe { wxMouseEvent_LeftUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn metaDown(&self) -> bool {
        unsafe { wxMouseEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleDClick(&self) -> bool {
        unsafe { wxMouseEvent_MiddleDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleDown(&self) -> bool {
        unsafe { wxMouseEvent_MiddleDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleIsDown(&self) -> bool {
        unsafe { wxMouseEvent_MiddleIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn middleUp(&self) -> bool {
        unsafe { wxMouseEvent_MiddleUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn moving(&self) -> bool {
        unsafe { wxMouseEvent_Moving(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightDClick(&self) -> bool {
        unsafe { wxMouseEvent_RightDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightDown(&self) -> bool {
        unsafe { wxMouseEvent_RightDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightIsDown(&self) -> bool {
        unsafe { wxMouseEvent_RightIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn rightUp(&self) -> bool {
        unsafe { wxMouseEvent_RightUp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shiftDown(&self) -> bool {
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

struct wxMoveEvent(*u8);
impl _wxMoveEvent for wxMoveEvent {}
impl _wxEvent for wxMoveEvent {}
impl _wxObject for wxMoveEvent { fn handle(&self) -> *u8 { **self } }

impl wxMoveEvent {
    pub fn from(handle: *u8) -> @wxMoveEvent { @wxMoveEvent(handle) }
    pub fn null() -> @wxMoveEvent { wxMoveEvent::from(0 as *u8) }
    
}

trait _wxMoveEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPoint(wxMoveEvent_GetPosition(self.handle())) }
    }
}

struct wxMultiCellCanvas(*u8);
impl _wxMultiCellCanvas for wxMultiCellCanvas {}
impl _wxFlexGridSizer for wxMultiCellCanvas {}
impl _wxGridSizer for wxMultiCellCanvas {}
impl _wxSizer for wxMultiCellCanvas {}
impl _wxObject for wxMultiCellCanvas { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellCanvas {
    pub fn from(handle: *u8) -> @wxMultiCellCanvas { @wxMultiCellCanvas(handle) }
    pub fn null() -> @wxMultiCellCanvas { wxMultiCellCanvas::from(0 as *u8) }
    
}

trait _wxMultiCellCanvas : _wxFlexGridSizer {
}

struct wxMultiCellItemHandle(*u8);
impl _wxMultiCellItemHandle for wxMultiCellItemHandle {}
impl _wxObject for wxMultiCellItemHandle { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellItemHandle {
    pub fn from(handle: *u8) -> @wxMultiCellItemHandle { @wxMultiCellItemHandle(handle) }
    pub fn null() -> @wxMultiCellItemHandle { wxMultiCellItemHandle::from(0 as *u8) }
    
}

trait _wxMultiCellItemHandle : _wxObject {
}

struct wxMultiCellSizer(*u8);
impl _wxMultiCellSizer for wxMultiCellSizer {}
impl _wxSizer for wxMultiCellSizer {}
impl _wxObject for wxMultiCellSizer { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellSizer {
    pub fn from(handle: *u8) -> @wxMultiCellSizer { @wxMultiCellSizer(handle) }
    pub fn null() -> @wxMultiCellSizer { wxMultiCellSizer::from(0 as *u8) }
    
}

trait _wxMultiCellSizer : _wxSizer {
}

struct wxMutex(*u8);
impl _wxMutex for wxMutex { fn handle(&self) -> *u8 { **self } }

impl wxMutex {
    pub fn from(handle: *u8) -> @wxMutex { @wxMutex(handle) }
    pub fn null() -> @wxMutex { wxMutex::from(0 as *u8) }
    
}

trait _wxMutex {
    fn handle(&self) -> *u8;
    
}

struct wxMutexLocker(*u8);
impl _wxMutexLocker for wxMutexLocker { fn handle(&self) -> *u8 { **self } }

impl wxMutexLocker {
    pub fn from(handle: *u8) -> @wxMutexLocker { @wxMutexLocker(handle) }
    pub fn null() -> @wxMutexLocker { wxMutexLocker::from(0 as *u8) }
    
}

trait _wxMutexLocker {
    fn handle(&self) -> *u8;
    
}

struct wxNavigationKeyEvent(*u8);
impl _wxNavigationKeyEvent for wxNavigationKeyEvent {}
impl _wxEvent for wxNavigationKeyEvent {}
impl _wxObject for wxNavigationKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNavigationKeyEvent {
    pub fn from(handle: *u8) -> @wxNavigationKeyEvent { @wxNavigationKeyEvent(handle) }
    pub fn null() -> @wxNavigationKeyEvent { wxNavigationKeyEvent::from(0 as *u8) }
    
}

trait _wxNavigationKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentFocus(&self) -> *u8 {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirection(&self) -> bool {
        unsafe { wxNavigationKeyEvent_GetDirection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWindowChange(&self) -> bool {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentFocus<T: _wxWindow>(&self, win: &T) {
        unsafe { wxNavigationKeyEvent_SetCurrentFocus(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDirection(&self, bForward: bool) {
        unsafe { wxNavigationKeyEvent_SetDirection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWindowChange(&self, bIs: bool) {
        unsafe { wxNavigationKeyEvent_SetWindowChange(self.handle(), bIs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn shouldPropagate(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_ShouldPropagate(self.handle()) }
    }
}

struct wxNewBitmapButton(*u8);
impl _wxNewBitmapButton for wxNewBitmapButton {}
impl _wxPanel for wxNewBitmapButton {}
impl _wxWindow for wxNewBitmapButton {}
impl _wxEvtHandler for wxNewBitmapButton {}
impl _wxObject for wxNewBitmapButton { fn handle(&self) -> *u8 { **self } }

impl wxNewBitmapButton {
    pub fn from(handle: *u8) -> @wxNewBitmapButton { @wxNewBitmapButton(handle) }
    pub fn null() -> @wxNewBitmapButton { wxNewBitmapButton::from(0 as *u8) }
    
}

trait _wxNewBitmapButton : _wxPanel {
}

struct wxNodeBase(*u8);
impl _wxNodeBase for wxNodeBase { fn handle(&self) -> *u8 { **self } }

impl wxNodeBase {
    pub fn from(handle: *u8) -> @wxNodeBase { @wxNodeBase(handle) }
    pub fn null() -> @wxNodeBase { wxNodeBase::from(0 as *u8) }
    
}

trait _wxNodeBase {
    fn handle(&self) -> *u8;
    
}

struct wxNotebook(*u8);
impl _wxNotebook for wxNotebook {}
impl _wxControl for wxNotebook {}
impl _wxWindow for wxNotebook {}
impl _wxEvtHandler for wxNotebook {}
impl _wxObject for wxNotebook { fn handle(&self) -> *u8 { **self } }

impl wxNotebook {
    pub fn from(handle: *u8) -> @wxNotebook { @wxNotebook(handle) }
    pub fn null() -> @wxNotebook { wxNotebook::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxNotebook {
        unsafe { @wxNotebook(wxNotebook_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxNotebook : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addPage<T: _wxWindow>(&self, pPage: &T, strText: &str, bSelect: bool, imageId: c_int) -> bool {
        let strText = wxT(strText);
        unsafe { wxNotebook_AddPage(self.handle(), pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn advanceSelection(&self, bForward: bool) {
        unsafe { wxNotebook_AdvanceSelection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteAllPages(&self) -> bool {
        unsafe { wxNotebook_DeleteAllPages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deletePage(&self, nPage: c_int) -> bool {
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
    fn hitTest(&self, x: c_int, y: c_int, flags: *c_long) -> c_int {
        unsafe { wxNotebook_HitTest(self.handle(), x, y, flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertPage<T: _wxWindow>(&self, nPage: c_int, pPage: &T, strText: &str, bSelect: bool, imageId: c_int) -> bool {
        let strText = wxT(strText);
        unsafe { wxNotebook_InsertPage(self.handle(), nPage, pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removePage(&self, nPage: c_int) -> bool {
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
    fn setPageImage(&self, nPage: c_int, nImage: c_int) -> bool {
        unsafe { wxNotebook_SetPageImage(self.handle(), nPage, nImage) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSize(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPageSize(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageText(&self, nPage: c_int, strText: &str) -> bool {
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

struct wxNotebookEvent(*u8);
impl _wxNotebookEvent for wxNotebookEvent {}
impl _wxNotifyEvent for wxNotebookEvent {}
impl _wxCommandEvent for wxNotebookEvent {}
impl _wxEvent for wxNotebookEvent {}
impl _wxObject for wxNotebookEvent { fn handle(&self) -> *u8 { **self } }

impl wxNotebookEvent {
    pub fn from(handle: *u8) -> @wxNotebookEvent { @wxNotebookEvent(handle) }
    pub fn null() -> @wxNotebookEvent { wxNotebookEvent::from(0 as *u8) }
    
}

trait _wxNotebookEvent : _wxNotifyEvent {
}

struct wxNotifyEvent(*u8);
impl _wxNotifyEvent for wxNotifyEvent {}
impl _wxCommandEvent for wxNotifyEvent {}
impl _wxEvent for wxNotifyEvent {}
impl _wxObject for wxNotifyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNotifyEvent {
    pub fn from(handle: *u8) -> @wxNotifyEvent { @wxNotifyEvent(handle) }
    pub fn null() -> @wxNotifyEvent { wxNotifyEvent::from(0 as *u8) }
    
}

trait _wxNotifyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn allow(&self) {
        unsafe { wxNotifyEvent_Allow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAllowed(&self) -> bool {
        unsafe { wxNotifyEvent_IsAllowed(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn veto(&self) {
        unsafe { wxNotifyEvent_Veto(self.handle()) }
    }
}

struct wxObject(*u8);
impl _wxObject for wxObject { fn handle(&self) -> *u8 { **self } }

impl wxObject {
    pub fn from(handle: *u8) -> @wxObject { @wxObject(handle) }
    pub fn null() -> @wxObject { wxObject::from(0 as *u8) }
    
}

trait _wxObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { @wxClosure(wxObject_GetClientClosure(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setClientClosure<T: _wxClosure>(&self, closure: &T) {
        unsafe { wxObject_SetClientClosure(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxObject_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getClassInfo(&self) -> @wxClassInfo {
        unsafe { @wxClassInfo(wxObject_GetClassInfo(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isKindOf<T: _wxClassInfo>(&self, classInfo: &T) -> bool {
        unsafe { wxObject_IsKindOf(self.handle(), classInfo.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isScrolledWindow(&self) -> bool {
        unsafe { wxObject_IsScrolledWindow(self.handle()) }
    }
}

struct wxObjectRefData(*u8);
impl _wxObjectRefData for wxObjectRefData { fn handle(&self) -> *u8 { **self } }

impl wxObjectRefData {
    pub fn from(handle: *u8) -> @wxObjectRefData { @wxObjectRefData(handle) }
    pub fn null() -> @wxObjectRefData { wxObjectRefData::from(0 as *u8) }
    
}

trait _wxObjectRefData {
    fn handle(&self) -> *u8;
    
}

struct wxOutputStream(*u8);
impl _wxOutputStream for wxOutputStream {}
impl _wxStreamBase for wxOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxOutputStream {
    pub fn from(handle: *u8) -> @wxOutputStream { @wxOutputStream(handle) }
    pub fn null() -> @wxOutputStream { wxOutputStream::from(0 as *u8) }
    
}

trait _wxOutputStream : _wxStreamBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn lastWrite(&self) -> c_int {
        unsafe { wxOutputStream_LastWrite(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn putC(&self, c: wchar_t) {
        unsafe { wxOutputStream_PutC(self.handle(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn seek(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxOutputStream_Seek(self.handle(), pos, mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn sync(&self) {
        unsafe { wxOutputStream_Sync(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tell(&self) -> c_int {
        unsafe { wxOutputStream_Tell(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn write(&self, buffer: *u8, size: c_int) {
        unsafe { wxOutputStream_Write(self.handle(), buffer, size) }
    }
}

struct wxPageSetupDialog(*u8);
impl _wxPageSetupDialog for wxPageSetupDialog {}
impl _wxDialog for wxPageSetupDialog {}
impl _wxTopLevelWindow for wxPageSetupDialog {}
impl _wxWindow for wxPageSetupDialog {}
impl _wxEvtHandler for wxPageSetupDialog {}
impl _wxObject for wxPageSetupDialog { fn handle(&self) -> *u8 { **self } }

impl wxPageSetupDialog {
    pub fn from(handle: *u8) -> @wxPageSetupDialog { @wxPageSetupDialog(handle) }
    pub fn null() -> @wxPageSetupDialog { wxPageSetupDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPageSetupDialogData>(parent: &T, data: &U) -> @wxPageSetupDialog {
        unsafe { @wxPageSetupDialog(wxPageSetupDialog_Create(parent.handle(), data.handle())) }
    }
}

trait _wxPageSetupDialog : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSetupData<T: _wxPageSetupDialogData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.handle(), _ref.handle()) }
    }
}

struct wxPageSetupDialogData(*u8);
impl _wxPageSetupDialogData for wxPageSetupDialogData {}
impl _wxObject for wxPageSetupDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPageSetupDialogData {
    pub fn from(handle: *u8) -> @wxPageSetupDialogData { @wxPageSetupDialogData(handle) }
    pub fn null() -> @wxPageSetupDialogData { wxPageSetupDialogData::from(0 as *u8) }
    
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

trait _wxPageSetupDialogData : _wxObject {
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
    fn enableHelp(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableHelp(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableMargins(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableMargins(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableOrientation(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableOrientation(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePaper(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnablePaper(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePrinter(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnablePrinter(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultInfo(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetDefaultInfo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultMinMargins(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetDefaultMinMargins(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableHelp(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableMargins(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableMargins(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableOrientation(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePaper(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnablePaper(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePrinter(&self) -> bool {
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
    fn setDefaultInfo(&self, flag: bool) {
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
    fn setPaperId(&self, id: *u8) {
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

struct wxPaintDC(*u8);
impl _wxPaintDC for wxPaintDC {}
impl _wxWindowDC for wxPaintDC {}
impl _wxDC for wxPaintDC {}
impl _wxObject for wxPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxPaintDC {
    pub fn from(handle: *u8) -> @wxPaintDC { @wxPaintDC(handle) }
    pub fn null() -> @wxPaintDC { wxPaintDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> @wxPaintDC {
        unsafe { @wxPaintDC(wxPaintDC_Create(win.handle())) }
    }
}

trait _wxPaintDC : _wxWindowDC {
}

struct wxPaintEvent(*u8);
impl _wxPaintEvent for wxPaintEvent {}
impl _wxEvent for wxPaintEvent {}
impl _wxObject for wxPaintEvent { fn handle(&self) -> *u8 { **self } }

impl wxPaintEvent {
    pub fn from(handle: *u8) -> @wxPaintEvent { @wxPaintEvent(handle) }
    pub fn null() -> @wxPaintEvent { wxPaintEvent::from(0 as *u8) }
    
}

trait _wxPaintEvent : _wxEvent {
}

struct wxPalette(*u8);
impl _wxPalette for wxPalette {}
impl _wxGDIObject for wxPalette {}
impl _wxObject for wxPalette { fn handle(&self) -> *u8 { **self } }

impl wxPalette {
    pub fn from(handle: *u8) -> @wxPalette { @wxPalette(handle) }
    pub fn null() -> @wxPalette { wxPalette::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault() -> @wxPalette {
        unsafe { @wxPalette(wxPalette_CreateDefault()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRGB(n: c_int, red: *u8, green: *u8, blue: *u8) -> @wxPalette {
        unsafe { @wxPalette(wxPalette_CreateRGB(n, red, green, blue)) }
    }
}

trait _wxPalette : _wxGDIObject {
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
    fn getRGB(&self, pixel: c_int, red: *u8, green: *u8, blue: *u8) -> bool {
        unsafe { wxPalette_GetRGB(self.handle(), pixel, red, green, blue) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxPalette>(&self, palette: &T) -> bool {
        unsafe { wxPalette_IsEqual(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxPalette_IsOk(self.handle()) }
    }
}

struct wxPaletteChangedEvent(*u8);
impl _wxPaletteChangedEvent for wxPaletteChangedEvent {}
impl _wxEvent for wxPaletteChangedEvent {}
impl _wxObject for wxPaletteChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxPaletteChangedEvent {
    pub fn from(handle: *u8) -> @wxPaletteChangedEvent { @wxPaletteChangedEvent(handle) }
    pub fn null() -> @wxPaletteChangedEvent { wxPaletteChangedEvent::from(0 as *u8) }
    
}

trait _wxPaletteChangedEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChangedWindow(&self) -> *u8 {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setChangedWindow<T: _wxWindow>(&self, win: &T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.handle(), win.handle()) }
    }
}

struct wxPanel(*u8);
impl _wxPanel for wxPanel {}
impl _wxWindow for wxPanel {}
impl _wxEvtHandler for wxPanel {}
impl _wxObject for wxPanel { fn handle(&self) -> *u8 { **self } }

impl wxPanel {
    pub fn from(handle: *u8) -> @wxPanel { @wxPanel(handle) }
    pub fn null() -> @wxPanel { wxPanel::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxPanel {
        unsafe { @wxPanel(wxPanel_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxPanel : _wxWindow {
}

struct wxPathList(*u8);
impl _wxPathList for wxPathList {}
impl _wxList for wxPathList {}
impl _wxObject for wxPathList { fn handle(&self) -> *u8 { **self } }

impl wxPathList {
    pub fn from(handle: *u8) -> @wxPathList { @wxPathList(handle) }
    pub fn null() -> @wxPathList { wxPathList::from(0 as *u8) }
    
}

trait _wxPathList : _wxList {
}

struct wxPen(*u8);
impl _wxPen for wxPen {}
impl _wxGDIObject for wxPen {}
impl _wxObject for wxPen { fn handle(&self) -> *u8 { **self } }

impl wxPen {
    pub fn from(handle: *u8) -> @wxPen { @wxPen(handle) }
    pub fn null() -> @wxPen { wxPen::from(0 as *u8) }
    
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

trait _wxPen : _wxGDIObject {
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
    fn getDashes(&self, ptr: *u8) -> c_int {
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
    fn isEqual<T: _wxPen>(&self, pen: &T) -> bool {
        unsafe { wxPen_IsEqual(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
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
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxPen_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDashes(&self, nb_dashes: c_int, dash: *u8) {
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
    fn isStatic(&self) -> bool {
        unsafe { wxPen_IsStatic(self.handle()) }
    }
}

struct wxPenList(*u8);
impl _wxPenList for wxPenList {}
impl _wxList for wxPenList {}
impl _wxObject for wxPenList { fn handle(&self) -> *u8 { **self } }

impl wxPenList {
    pub fn from(handle: *u8) -> @wxPenList { @wxPenList(handle) }
    pub fn null() -> @wxPenList { wxPenList::from(0 as *u8) }
    
}

trait _wxPenList : _wxList {
}

struct wxPlotCurve(*u8);
impl _wxPlotCurve for wxPlotCurve {}
impl _wxObject for wxPlotCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotCurve {
    pub fn from(handle: *u8) -> @wxPlotCurve { @wxPlotCurve(handle) }
    pub fn null() -> @wxPlotCurve { wxPlotCurve::from(0 as *u8) }
    
}

trait _wxPlotCurve : _wxObject {
}

struct wxPlotEvent(*u8);
impl _wxPlotEvent for wxPlotEvent {}
impl _wxNotifyEvent for wxPlotEvent {}
impl _wxCommandEvent for wxPlotEvent {}
impl _wxEvent for wxPlotEvent {}
impl _wxObject for wxPlotEvent { fn handle(&self) -> *u8 { **self } }

impl wxPlotEvent {
    pub fn from(handle: *u8) -> @wxPlotEvent { @wxPlotEvent(handle) }
    pub fn null() -> @wxPlotEvent { wxPlotEvent::from(0 as *u8) }
    
}

trait _wxPlotEvent : _wxNotifyEvent {
}

struct wxPlotOnOffCurve(*u8);
impl _wxPlotOnOffCurve for wxPlotOnOffCurve {}
impl _wxObject for wxPlotOnOffCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotOnOffCurve {
    pub fn from(handle: *u8) -> @wxPlotOnOffCurve { @wxPlotOnOffCurve(handle) }
    pub fn null() -> @wxPlotOnOffCurve { wxPlotOnOffCurve::from(0 as *u8) }
    
}

trait _wxPlotOnOffCurve : _wxObject {
}

struct wxPlotWindow(*u8);
impl _wxPlotWindow for wxPlotWindow {}
impl _wxScrolledWindow for wxPlotWindow {}
impl _wxPanel for wxPlotWindow {}
impl _wxWindow for wxPlotWindow {}
impl _wxEvtHandler for wxPlotWindow {}
impl _wxObject for wxPlotWindow { fn handle(&self) -> *u8 { **self } }

impl wxPlotWindow {
    pub fn from(handle: *u8) -> @wxPlotWindow { @wxPlotWindow(handle) }
    pub fn null() -> @wxPlotWindow { wxPlotWindow::from(0 as *u8) }
    
}

trait _wxPlotWindow : _wxScrolledWindow {
}

struct wxPoint(*u8);
impl _wxPoint for wxPoint { fn handle(&self) -> *u8 { **self } }

impl wxPoint {
    pub fn from(handle: *u8) -> @wxPoint { @wxPoint(handle) }
    pub fn null() -> @wxPoint { wxPoint::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(xx: c_int, yy: c_int) -> @wxPoint {
        unsafe { @wxPoint(wxPoint_Create(xx, yy)) }
    }
}

trait _wxPoint {
    fn handle(&self) -> *u8;
    
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

struct wxPopupTransientWindow(*u8);
impl _wxPopupTransientWindow for wxPopupTransientWindow {}
impl _wxPopupWindow for wxPopupTransientWindow {}
impl _wxWindow for wxPopupTransientWindow {}
impl _wxEvtHandler for wxPopupTransientWindow {}
impl _wxObject for wxPopupTransientWindow { fn handle(&self) -> *u8 { **self } }

impl wxPopupTransientWindow {
    pub fn from(handle: *u8) -> @wxPopupTransientWindow { @wxPopupTransientWindow(handle) }
    pub fn null() -> @wxPopupTransientWindow { wxPopupTransientWindow::from(0 as *u8) }
    
}

trait _wxPopupTransientWindow : _wxPopupWindow {
}

struct wxPopupWindow(*u8);
impl _wxPopupWindow for wxPopupWindow {}
impl _wxWindow for wxPopupWindow {}
impl _wxEvtHandler for wxPopupWindow {}
impl _wxObject for wxPopupWindow { fn handle(&self) -> *u8 { **self } }

impl wxPopupWindow {
    pub fn from(handle: *u8) -> @wxPopupWindow { @wxPopupWindow(handle) }
    pub fn null() -> @wxPopupWindow { wxPopupWindow::from(0 as *u8) }
    
}

trait _wxPopupWindow : _wxWindow {
}

struct wxPostScriptDC(*u8);
impl _wxPostScriptDC for wxPostScriptDC {}
impl _wxDC for wxPostScriptDC {}
impl _wxObject for wxPostScriptDC { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptDC {
    pub fn from(handle: *u8) -> @wxPostScriptDC { @wxPostScriptDC(handle) }
    pub fn null() -> @wxPostScriptDC { wxPostScriptDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintData>(data: &T) -> @wxPostScriptDC {
        unsafe { @wxPostScriptDC(wxPostScriptDC_Create(data.handle())) }
    }
}

trait _wxPostScriptDC : _wxDC {
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

struct wxPreviewCanvas(*u8);
impl _wxPreviewCanvas for wxPreviewCanvas {}
impl _wxScrolledWindow for wxPreviewCanvas {}
impl _wxPanel for wxPreviewCanvas {}
impl _wxWindow for wxPreviewCanvas {}
impl _wxEvtHandler for wxPreviewCanvas {}
impl _wxObject for wxPreviewCanvas { fn handle(&self) -> *u8 { **self } }

impl wxPreviewCanvas {
    pub fn from(handle: *u8) -> @wxPreviewCanvas { @wxPreviewCanvas(handle) }
    pub fn null() -> @wxPreviewCanvas { wxPreviewCanvas::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintPreview, U: _wxWindow>(preview: &T, parent: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPreviewCanvas_Create(preview.handle(), parent.handle(), x, y, w, h, style)) }
    }
}

trait _wxPreviewCanvas : _wxScrolledWindow {
}

struct wxPreviewControlBar(*u8);
impl _wxPreviewControlBar for wxPreviewControlBar {}
impl _wxPanel for wxPreviewControlBar {}
impl _wxWindow for wxPreviewControlBar {}
impl _wxEvtHandler for wxPreviewControlBar {}
impl _wxObject for wxPreviewControlBar { fn handle(&self) -> *u8 { **self } }

impl wxPreviewControlBar {
    pub fn from(handle: *u8) -> @wxPreviewControlBar { @wxPreviewControlBar(handle) }
    pub fn null() -> @wxPreviewControlBar { wxPreviewControlBar::from(0 as *u8) }
    
}

trait _wxPreviewControlBar : _wxPanel {
}

struct wxPreviewFrame(*u8);
impl _wxPreviewFrame for wxPreviewFrame {}
impl _wxFrame for wxPreviewFrame {}
impl _wxTopLevelWindow for wxPreviewFrame {}
impl _wxWindow for wxPreviewFrame {}
impl _wxEvtHandler for wxPreviewFrame {}
impl _wxObject for wxPreviewFrame { fn handle(&self) -> *u8 { **self } }

impl wxPreviewFrame {
    pub fn from(handle: *u8) -> @wxPreviewFrame { @wxPreviewFrame(handle) }
    pub fn null() -> @wxPreviewFrame { wxPreviewFrame::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintPreview, U: _wxFrame>(preview: &T, parent: &U, title: &str, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: &str) -> @wxPreviewFrame {
        let title = wxT(title);
        let name = wxT(name);
        unsafe { @wxPreviewFrame(wxPreviewFrame_Create(preview.handle(), parent.handle(), title.handle(), x, y, width, height, style, name.handle())) }
    }
}

trait _wxPreviewFrame : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.handle()) }
    }
}

struct wxPrintData(*u8);
impl _wxPrintData for wxPrintData {}
impl _wxObject for wxPrintData { fn handle(&self) -> *u8 { **self } }

impl wxPrintData {
    pub fn from(handle: *u8) -> @wxPrintData { @wxPrintData(handle) }
    pub fn null() -> @wxPrintData { wxPrintData::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxPrintData {
        unsafe { @wxPrintData(wxPrintData_Create()) }
    }
}

trait _wxPrintData : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn assign<T: _wxPrintData>(&self, data: &T) {
        unsafe { wxPrintData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCollate(&self) -> bool {
        unsafe { wxPrintData_GetCollate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getColour(&self) -> bool {
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

struct wxPostScriptPrintNativeData(*u8);
impl _wxPostScriptPrintNativeData for wxPostScriptPrintNativeData {}
impl _wxObject for wxPostScriptPrintNativeData { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptPrintNativeData {
    pub fn from(handle: *u8) -> @wxPostScriptPrintNativeData { @wxPostScriptPrintNativeData(handle) }
    pub fn null() -> @wxPostScriptPrintNativeData { wxPostScriptPrintNativeData::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxPostScriptPrintNativeData {
        unsafe { @wxPostScriptPrintNativeData(wxPostScriptPrintNativeData_Create()) }
    }
}

trait _wxPostScriptPrintNativeData : _wxObject {
}

struct wxPrintDialog(*u8);
impl _wxPrintDialog for wxPrintDialog {}
impl _wxDialog for wxPrintDialog {}
impl _wxTopLevelWindow for wxPrintDialog {}
impl _wxWindow for wxPrintDialog {}
impl _wxEvtHandler for wxPrintDialog {}
impl _wxObject for wxPrintDialog { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialog {
    pub fn from(handle: *u8) -> @wxPrintDialog { @wxPrintDialog(handle) }
    pub fn null() -> @wxPrintDialog { wxPrintDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxPrintDialogData>(parent: &T, data: &U) -> @wxPrintDialog {
        unsafe { @wxPrintDialog(wxPrintDialog_Create(parent.handle(), data.handle())) }
    }
}

trait _wxPrintDialog : _wxDialog {
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

struct wxPrintDialogData(*u8);
impl _wxPrintDialogData for wxPrintDialogData {}
impl _wxObject for wxPrintDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialogData {
    pub fn from(handle: *u8) -> @wxPrintDialogData { @wxPrintDialogData(handle) }
    pub fn null() -> @wxPrintDialogData { wxPrintDialogData::from(0 as *u8) }
    
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

trait _wxPrintDialogData : _wxObject {
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
    fn enableHelp(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnableHelp(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePageNumbers(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnablePageNumbers(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enablePrintToFile(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnablePrintToFile(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableSelection(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnableSelection(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAllPages(&self) -> c_int {
        unsafe { wxPrintDialogData_GetAllPages(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCollate(&self) -> bool {
        unsafe { wxPrintDialogData_GetCollate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableHelp(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnableHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePageNumbers(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnablePageNumbers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnablePrintToFile(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnablePrintToFile(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnableSelection(&self) -> bool {
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
    fn getPrintToFile(&self) -> bool {
        unsafe { wxPrintDialogData_GetPrintToFile(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSelection(&self) -> bool {
        unsafe { wxPrintDialogData_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getToPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetToPage(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAllPages(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetAllPages(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCollate(&self, flag: bool) {
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
    fn setPrintToFile(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetPrintToFile(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetSelection(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setToPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetToPage(self.handle(), v) }
    }
}

struct wxPrintPreview(*u8);
impl _wxPrintPreview for wxPrintPreview {}
impl _wxObject for wxPrintPreview { fn handle(&self) -> *u8 { **self } }

impl wxPrintPreview {
    pub fn from(handle: *u8) -> @wxPrintPreview { @wxPrintPreview(handle) }
    pub fn null() -> @wxPrintPreview { wxPrintPreview::from(0 as *u8) }
    
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

trait _wxPrintPreview : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn drawBlankPage<T: _wxPreviewCanvas, U: _wxDC>(&self, canvas: &T, dc: &U) -> bool {
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
    fn isOk(&self) -> bool {
        unsafe { wxPrintPreview_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn paintPage<T: _wxPrintPreview, U: _wxDC>(&self, canvas: &T, dc: &U) -> bool {
        unsafe { wxPrintPreview_PaintPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn print(&self, interactive: bool) -> bool {
        unsafe { wxPrintPreview_Print(self.handle(), interactive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn renderPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_RenderPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCanvas<T: _wxPreviewCanvas>(&self, canvas: &T) {
        unsafe { wxPrintPreview_SetCanvas(self.handle(), canvas.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrentPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_SetCurrentPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFrame<T: _wxFrame>(&self, frame: &T) {
        unsafe { wxPrintPreview_SetFrame(self.handle(), frame.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOk(&self, ok: bool) {
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

struct wxPrinter(*u8);
impl _wxPrinter for wxPrinter {}
impl _wxObject for wxPrinter { fn handle(&self) -> *u8 { **self } }

impl wxPrinter {
    pub fn from(handle: *u8) -> @wxPrinter { @wxPrinter(handle) }
    pub fn null() -> @wxPrinter { wxPrinter::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintDialogData>(data: &T) -> @wxPrinter {
        unsafe { @wxPrinter(wxPrinter_Create(data.handle())) }
    }
}

trait _wxPrinter : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn newAbortWindow<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U) -> @wxWindow {
        unsafe { @wxWindow(wxPrinter_CreateAbortWindow(self.handle(), parent.handle(), printout.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAbort(&self) -> bool {
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
    fn print<T: _wxWindow, U: _wxPrintout>(&self, parent: &T, printout: &U, prompt: bool) -> bool {
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
    fn setup<T: _wxWindow>(&self, parent: &T) -> bool {
        unsafe { wxPrinter_Setup(self.handle(), parent.handle()) }
    }
}

struct wxPrinterDC(*u8);
impl _wxPrinterDC for wxPrinterDC {}
impl _wxDC for wxPrinterDC {}
impl _wxObject for wxPrinterDC { fn handle(&self) -> *u8 { **self } }

impl wxPrinterDC {
    pub fn from(handle: *u8) -> @wxPrinterDC { @wxPrinterDC(handle) }
    pub fn null() -> @wxPrinterDC { wxPrinterDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxPrintData>(data: &T) -> @wxPrinterDC {
        unsafe { @wxPrinterDC(wxPrinterDC_Create(data.handle())) }
    }
}

trait _wxPrinterDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaperRect(&self) -> @wxRect {
        unsafe { @wxRect(wxPrinterDC_GetPaperRect(self.handle())) }
    }
}

struct wxPrintout(*u8);
impl _wxPrintout for wxPrintout {}
impl _wxObject for wxPrintout { fn handle(&self) -> *u8 { **self } }

impl wxPrintout {
    pub fn from(handle: *u8) -> @wxPrintout { @wxPrintout(handle) }
    pub fn null() -> @wxPrintout { wxPrintout::from(0 as *u8) }
    
}

trait _wxPrintout : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDC(&self) -> @wxDC {
        unsafe { @wxDC(wxPrintout_GetDC(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPIPrinter(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxPrintout_GetPPIPrinter(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPPIScreen(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxPrintout_GetPPIScreen(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSizeMM(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxPrintout_GetPageSizeMM(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSizePixels(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxPrintout_GetPageSizePixels(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTitle(&self) -> ~str {
        unsafe { wxString { handle: wxPrintout_GetTitle(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isPreview(&self) -> bool {
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

struct wxPrivateDropTarget(*u8);
impl _wxPrivateDropTarget for wxPrivateDropTarget {}
impl _wxDropTarget for wxPrivateDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxPrivateDropTarget {
    pub fn from(handle: *u8) -> @wxPrivateDropTarget { @wxPrivateDropTarget(handle) }
    pub fn null() -> @wxPrivateDropTarget { wxPrivateDropTarget::from(0 as *u8) }
    
}

trait _wxPrivateDropTarget : _wxDropTarget {
}

struct wxProcess(*u8);
impl _wxProcess for wxProcess {}
impl _wxEvtHandler for wxProcess {}
impl _wxObject for wxProcess { fn handle(&self) -> *u8 { **self } }

impl wxProcess {
    pub fn from(handle: *u8) -> @wxProcess { @wxProcess(handle) }
    pub fn null() -> @wxProcess { wxProcess::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newDefault<T: _wxWindow>(_prt: &T, _id: c_int) -> @wxProcess {
        unsafe { @wxProcess(wxProcess_CreateDefault(_prt.handle(), _id)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newRedirect<T: _wxWindow>(_prt: &T, _rdr: bool) -> @wxProcess {
        unsafe { @wxProcess(wxProcess_CreateRedirect(_prt.handle(), _rdr)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn open(cmd: &str, flags: c_int) -> @wxProcess {
        let cmd = wxT(cmd);
        unsafe { @wxProcess(wxProcess_Open(cmd.handle(), flags)) }
    }
}

trait _wxProcess : _wxEvtHandler {
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
    fn isRedirected(&self) -> bool {
        unsafe { wxProcess_IsRedirected(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn redirect(&self) {
        unsafe { wxProcess_Redirect(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isErrorAvailable(&self) -> bool {
        unsafe { wxProcess_IsErrorAvailable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInputAvailable(&self) -> bool {
        unsafe { wxProcess_IsInputAvailable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isInputOpened(&self) -> bool {
        unsafe { wxProcess_IsInputOpened(self.handle()) }
    }
}

struct wxProcessEvent(*u8);
impl _wxProcessEvent for wxProcessEvent {}
impl _wxEvent for wxProcessEvent {}
impl _wxObject for wxProcessEvent { fn handle(&self) -> *u8 { **self } }

impl wxProcessEvent {
    pub fn from(handle: *u8) -> @wxProcessEvent { @wxProcessEvent(handle) }
    pub fn null() -> @wxProcessEvent { wxProcessEvent::from(0 as *u8) }
    
}

trait _wxProcessEvent : _wxEvent {
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

struct wxProgressDialog(*u8);
impl _wxProgressDialog for wxProgressDialog {}
impl _wxFrame for wxProgressDialog {}
impl _wxTopLevelWindow for wxProgressDialog {}
impl _wxWindow for wxProgressDialog {}
impl _wxEvtHandler for wxProgressDialog {}
impl _wxObject for wxProgressDialog { fn handle(&self) -> *u8 { **self } }

impl wxProgressDialog {
    pub fn from(handle: *u8) -> @wxProgressDialog { @wxProgressDialog(handle) }
    pub fn null() -> @wxProgressDialog { wxProgressDialog::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(title: &str, message: &str, max: c_int, parent: &T, style: c_int) -> @wxProgressDialog {
        let title = wxT(title);
        let message = wxT(message);
        unsafe { @wxProgressDialog(wxProgressDialog_Create(title.handle(), message.handle(), max, parent.handle(), style)) }
    }
}

trait _wxProgressDialog : _wxFrame {
    #[fixed_stack_segment]
    #[inline(never)]
    fn update(&self, value: c_int) -> bool {
        unsafe { wxProgressDialog_Update(self.handle(), value) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateWithMessage(&self, value: c_int, message: &str) -> bool {
        let message = wxT(message);
        unsafe { wxProgressDialog_UpdateWithMessage(self.handle(), value, message.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.handle()) }
    }
}

struct wxProtocol(*u8);
impl _wxProtocol for wxProtocol {}
impl _wxSocketClient for wxProtocol {}
impl _wxSocketBase for wxProtocol {}
impl _wxObject for wxProtocol { fn handle(&self) -> *u8 { **self } }

impl wxProtocol {
    pub fn from(handle: *u8) -> @wxProtocol { @wxProtocol(handle) }
    pub fn null() -> @wxProtocol { wxProtocol::from(0 as *u8) }
    
}

trait _wxProtocol : _wxSocketClient {
}

struct wxQuantize(*u8);
impl _wxQuantize for wxQuantize {}
impl _wxObject for wxQuantize { fn handle(&self) -> *u8 { **self } }

impl wxQuantize {
    pub fn from(handle: *u8) -> @wxQuantize { @wxQuantize(handle) }
    pub fn null() -> @wxQuantize { wxQuantize::from(0 as *u8) }
    
}

trait _wxQuantize : _wxObject {
}

struct wxQueryCol(*u8);
impl _wxQueryCol for wxQueryCol {}
impl _wxObject for wxQueryCol { fn handle(&self) -> *u8 { **self } }

impl wxQueryCol {
    pub fn from(handle: *u8) -> @wxQueryCol { @wxQueryCol(handle) }
    pub fn null() -> @wxQueryCol { wxQueryCol::from(0 as *u8) }
    
}

trait _wxQueryCol : _wxObject {
}

struct wxQueryField(*u8);
impl _wxQueryField for wxQueryField {}
impl _wxObject for wxQueryField { fn handle(&self) -> *u8 { **self } }

impl wxQueryField {
    pub fn from(handle: *u8) -> @wxQueryField { @wxQueryField(handle) }
    pub fn null() -> @wxQueryField { wxQueryField::from(0 as *u8) }
    
}

trait _wxQueryField : _wxObject {
}

struct wxQueryLayoutInfoEvent(*u8);
impl _wxQueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl _wxEvent for wxQueryLayoutInfoEvent {}
impl _wxObject for wxQueryLayoutInfoEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryLayoutInfoEvent {
    pub fn from(handle: *u8) -> @wxQueryLayoutInfoEvent { @wxQueryLayoutInfoEvent(handle) }
    pub fn null() -> @wxQueryLayoutInfoEvent { wxQueryLayoutInfoEvent::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(id: c_int) -> @wxQueryLayoutInfoEvent {
        unsafe { @wxQueryLayoutInfoEvent(wxQueryLayoutInfoEvent_Create(id)) }
    }
}

trait _wxQueryLayoutInfoEvent : _wxEvent {
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

struct wxQueryNewPaletteEvent(*u8);
impl _wxQueryNewPaletteEvent for wxQueryNewPaletteEvent {}
impl _wxEvent for wxQueryNewPaletteEvent {}
impl _wxObject for wxQueryNewPaletteEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryNewPaletteEvent {
    pub fn from(handle: *u8) -> @wxQueryNewPaletteEvent { @wxQueryNewPaletteEvent(handle) }
    pub fn null() -> @wxQueryNewPaletteEvent { wxQueryNewPaletteEvent::from(0 as *u8) }
    
}

trait _wxQueryNewPaletteEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPaletteRealized(&self) -> bool {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPaletteRealized(&self, realized: bool) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self.handle(), realized) }
    }
}

struct wxRadioBox(*u8);
impl _wxRadioBox for wxRadioBox {}
impl _wxControl for wxRadioBox {}
impl _wxWindow for wxRadioBox {}
impl _wxEvtHandler for wxRadioBox {}
impl _wxObject for wxRadioBox { fn handle(&self) -> *u8 { **self } }

impl wxRadioBox {
    pub fn from(handle: *u8) -> @wxRadioBox { @wxRadioBox(handle) }
    pub fn null() -> @wxRadioBox { wxRadioBox::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: c_int, _stl: c_int) -> @wxRadioBox {
        let _txt = wxT(_txt);
        unsafe { @wxRadioBox(wxRadioBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) }
    }
}

trait _wxRadioBox : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableItem(&self, item: c_int, enable: bool) {
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
    fn showItem(&self, item: c_int, show: bool) {
        unsafe { wxRadioBox_ShowItem(self.handle(), item, show) }
    }
}

struct wxRadioButton(*u8);
impl _wxRadioButton for wxRadioButton {}
impl _wxControl for wxRadioButton {}
impl _wxWindow for wxRadioButton {}
impl _wxEvtHandler for wxRadioButton {}
impl _wxObject for wxRadioButton { fn handle(&self) -> *u8 { **self } }

impl wxRadioButton {
    pub fn from(handle: *u8) -> @wxRadioButton { @wxRadioButton(handle) }
    pub fn null() -> @wxRadioButton { wxRadioButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxRadioButton {
        let _txt = wxT(_txt);
        unsafe { @wxRadioButton(wxRadioButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxRadioButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> bool {
        unsafe { wxRadioButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, value: bool) {
        unsafe { wxRadioButton_SetValue(self.handle(), value) }
    }
}

struct wxRealPoint(*u8);
impl _wxRealPoint for wxRealPoint { fn handle(&self) -> *u8 { **self } }

impl wxRealPoint {
    pub fn from(handle: *u8) -> @wxRealPoint { @wxRealPoint(handle) }
    pub fn null() -> @wxRealPoint { wxRealPoint::from(0 as *u8) }
    
}

trait _wxRealPoint {
    fn handle(&self) -> *u8;
    
}

struct wxRecordSet(*u8);
impl _wxRecordSet for wxRecordSet {}
impl _wxObject for wxRecordSet { fn handle(&self) -> *u8 { **self } }

impl wxRecordSet {
    pub fn from(handle: *u8) -> @wxRecordSet { @wxRecordSet(handle) }
    pub fn null() -> @wxRecordSet { wxRecordSet::from(0 as *u8) }
    
}

trait _wxRecordSet : _wxObject {
}

struct wxRect(*u8);
impl _wxRect for wxRect { fn handle(&self) -> *u8 { **self } }

impl wxRect {
    pub fn from(handle: *u8) -> @wxRect { @wxRect(handle) }
    pub fn null() -> @wxRect { wxRect::from(0 as *u8) }
    
}

trait _wxRect {
    fn handle(&self) -> *u8;
    
}

struct wxRegEx(*u8);
impl _wxRegEx for wxRegEx { fn handle(&self) -> *u8 { **self } }

impl wxRegEx {
    pub fn from(handle: *u8) -> @wxRegEx { @wxRegEx(handle) }
    pub fn null() -> @wxRegEx { wxRegEx::from(0 as *u8) }
    
}

trait _wxRegEx {
    fn handle(&self) -> *u8;
    
}

struct wxRegion(*u8);
impl _wxRegion for wxRegion {}
impl _wxGDIObject for wxRegion {}
impl _wxObject for wxRegion { fn handle(&self) -> *u8 { **self } }

impl wxRegion {
    pub fn from(handle: *u8) -> @wxRegion { @wxRegion(handle) }
    pub fn null() -> @wxRegion { wxRegion::from(0 as *u8) }
    
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

trait _wxRegion : _wxGDIObject {
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
    fn containsPoint(&self, x: c_int, y: c_int) -> bool {
        unsafe { wxRegion_ContainsPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_ContainsRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEmpty(&self) -> bool {
        unsafe { wxRegion_IsEmpty(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRegion_GetBox(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_IntersectRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn intersectRegion<T: _wxRegion>(&self, region: &T) -> bool {
        unsafe { wxRegion_IntersectRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_SubtractRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn subtractRegion<T: _wxRegion>(&self, region: &T) -> bool {
        unsafe { wxRegion_SubtractRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_UnionRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unionRegion<T: _wxRegion>(&self, region: &T) -> bool {
        unsafe { wxRegion_UnionRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_XorRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn xorRegion<T: _wxRegion>(&self, region: &T) -> bool {
        unsafe { wxRegion_XorRegion(self.handle(), region.handle()) }
    }
}

struct wxRegionIterator(*u8);
impl _wxRegionIterator for wxRegionIterator {}
impl _wxObject for wxRegionIterator { fn handle(&self) -> *u8 { **self } }

impl wxRegionIterator {
    pub fn from(handle: *u8) -> @wxRegionIterator { @wxRegionIterator(handle) }
    pub fn null() -> @wxRegionIterator { wxRegionIterator::from(0 as *u8) }
    
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

trait _wxRegionIterator : _wxObject {
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
    fn haveRects(&self) -> bool {
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

struct wxRemotelyScrolledTreeCtrl(*u8);
impl _wxRemotelyScrolledTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxControl for wxRemotelyScrolledTreeCtrl {}
impl _wxWindow for wxRemotelyScrolledTreeCtrl {}
impl _wxEvtHandler for wxRemotelyScrolledTreeCtrl {}
impl _wxObject for wxRemotelyScrolledTreeCtrl { fn handle(&self) -> *u8 { **self } }

impl wxRemotelyScrolledTreeCtrl {
    pub fn from(handle: *u8) -> @wxRemotelyScrolledTreeCtrl { @wxRemotelyScrolledTreeCtrl(handle) }
    pub fn null() -> @wxRemotelyScrolledTreeCtrl { wxRemotelyScrolledTreeCtrl::from(0 as *u8) }
    
}

trait _wxRemotelyScrolledTreeCtrl : _wxTreeCtrl {
}

struct wxSVGFileDC(*u8);
impl _wxSVGFileDC for wxSVGFileDC {}
impl _wxDC for wxSVGFileDC {}
impl _wxObject for wxSVGFileDC { fn handle(&self) -> *u8 { **self } }

impl wxSVGFileDC {
    pub fn from(handle: *u8) -> @wxSVGFileDC { @wxSVGFileDC(handle) }
    pub fn null() -> @wxSVGFileDC { wxSVGFileDC::from(0 as *u8) }
    
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

trait _wxSVGFileDC : _wxDC {
}

struct wxSashEvent(*u8);
impl _wxSashEvent for wxSashEvent {}
impl _wxEvent for wxSashEvent {}
impl _wxObject for wxSashEvent { fn handle(&self) -> *u8 { **self } }

impl wxSashEvent {
    pub fn from(handle: *u8) -> @wxSashEvent { @wxSashEvent(handle) }
    pub fn null() -> @wxSashEvent { wxSashEvent::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(id: c_int, edge: c_int) -> @wxSashEvent {
        unsafe { @wxSashEvent(wxSashEvent_Create(id, edge)) }
    }
}

trait _wxSashEvent : _wxEvent {
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

struct wxSashLayoutWindow(*u8);
impl _wxSashLayoutWindow for wxSashLayoutWindow {}
impl _wxSashWindow for wxSashLayoutWindow {}
impl _wxWindow for wxSashLayoutWindow {}
impl _wxEvtHandler for wxSashLayoutWindow {}
impl _wxObject for wxSashLayoutWindow { fn handle(&self) -> *u8 { **self } }

impl wxSashLayoutWindow {
    pub fn from(handle: *u8) -> @wxSashLayoutWindow { @wxSashLayoutWindow(handle) }
    pub fn null() -> @wxSashLayoutWindow { wxSashLayoutWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxSashLayoutWindow {
        unsafe { @wxSashLayoutWindow(wxSashLayoutWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) }
    }
}

trait _wxSashLayoutWindow : _wxSashWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlignment(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOrientation(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setAlignment(&self, align: c_int) {
        unsafe { wxSashLayoutWindow_SetAlignment(self.handle(), align) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultSize(&self, w: c_int, h: c_int) {
        unsafe { wxSashLayoutWindow_SetDefaultSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxSashLayoutWindow_SetOrientation(self.handle(), orient) }
    }
}

struct wxSashWindow(*u8);
impl _wxSashWindow for wxSashWindow {}
impl _wxWindow for wxSashWindow {}
impl _wxEvtHandler for wxSashWindow {}
impl _wxObject for wxSashWindow { fn handle(&self) -> *u8 { **self } }

impl wxSashWindow {
    pub fn from(handle: *u8) -> @wxSashWindow { @wxSashWindow(handle) }
    pub fn null() -> @wxSashWindow { wxSashWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxSashWindow {
        unsafe { @wxSashWindow(wxSashWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) }
    }
}

trait _wxSashWindow : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetDefaultBorderSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEdgeMargin(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_GetEdgeMargin(self.handle(), edge) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExtraBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetExtraBorderSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaximumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMaximumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinimumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeX(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMinimumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeY(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSashVisible(&self, edge: c_int) -> bool {
        unsafe { wxSashWindow_GetSashVisible(self.handle(), edge) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasBorder(&self, edge: c_int) -> bool {
        unsafe { wxSashWindow_HasBorder(self.handle(), edge) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetDefaultBorderSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExtraBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetExtraBorderSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaximumSizeX(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeX(self.handle(), max) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaximumSizeY(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeY(self.handle(), max) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinimumSizeX(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeX(self.handle(), min) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMinimumSizeY(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeY(self.handle(), min) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashBorder(&self, edge: c_int, border: bool) {
        unsafe { wxSashWindow_SetSashBorder(self.handle(), edge, border) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSashVisible(&self, edge: c_int, sash: bool) {
        unsafe { wxSashWindow_SetSashVisible(self.handle(), edge, sash) }
    }
}

struct wxScopedArray(*u8);
impl _wxScopedArray for wxScopedArray { fn handle(&self) -> *u8 { **self } }

impl wxScopedArray {
    pub fn from(handle: *u8) -> @wxScopedArray { @wxScopedArray(handle) }
    pub fn null() -> @wxScopedArray { wxScopedArray::from(0 as *u8) }
    
}

trait _wxScopedArray {
    fn handle(&self) -> *u8;
    
}

struct wxScopedPtr(*u8);
impl _wxScopedPtr for wxScopedPtr { fn handle(&self) -> *u8 { **self } }

impl wxScopedPtr {
    pub fn from(handle: *u8) -> @wxScopedPtr { @wxScopedPtr(handle) }
    pub fn null() -> @wxScopedPtr { wxScopedPtr::from(0 as *u8) }
    
}

trait _wxScopedPtr {
    fn handle(&self) -> *u8;
    
}

struct wxScreenDC(*u8);
impl _wxScreenDC for wxScreenDC {}
impl _wxDC for wxScreenDC {}
impl _wxObject for wxScreenDC { fn handle(&self) -> *u8 { **self } }

impl wxScreenDC {
    pub fn from(handle: *u8) -> @wxScreenDC { @wxScreenDC(handle) }
    pub fn null() -> @wxScreenDC { wxScreenDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxScreenDC {
        unsafe { @wxScreenDC(wxScreenDC_Create()) }
    }
}

trait _wxScreenDC : _wxDC {
    #[fixed_stack_segment]
    #[inline(never)]
    fn endDrawingOnTop(&self) -> bool {
        unsafe { wxScreenDC_EndDrawingOnTop(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTop(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn startDrawingOnTopOfWin<T: _wxWindow>(&self, win: &T) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.handle(), win.handle()) }
    }
}

struct wxScrollBar(*u8);
impl _wxScrollBar for wxScrollBar {}
impl _wxControl for wxScrollBar {}
impl _wxWindow for wxScrollBar {}
impl _wxEvtHandler for wxScrollBar {}
impl _wxObject for wxScrollBar { fn handle(&self) -> *u8 { **self } }

impl wxScrollBar {
    pub fn from(handle: *u8) -> @wxScrollBar { @wxScrollBar(handle) }
    pub fn null() -> @wxScrollBar { wxScrollBar::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxScrollBar {
        unsafe { @wxScrollBar(wxScrollBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxScrollBar : _wxControl {
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

struct wxScrollEvent(*u8);
impl _wxScrollEvent for wxScrollEvent {}
impl _wxEvent for wxScrollEvent {}
impl _wxObject for wxScrollEvent { fn handle(&self) -> *u8 { **self } }

impl wxScrollEvent {
    pub fn from(handle: *u8) -> @wxScrollEvent { @wxScrollEvent(handle) }
    pub fn null() -> @wxScrollEvent { wxScrollEvent::from(0 as *u8) }
    
}

trait _wxScrollEvent : _wxEvent {
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

struct wxScrollWinEvent(*u8);
impl _wxScrollWinEvent for wxScrollWinEvent {}
impl _wxEvent for wxScrollWinEvent {}
impl _wxObject for wxScrollWinEvent { fn handle(&self) -> *u8 { **self } }

impl wxScrollWinEvent {
    pub fn from(handle: *u8) -> @wxScrollWinEvent { @wxScrollWinEvent(handle) }
    pub fn null() -> @wxScrollWinEvent { wxScrollWinEvent::from(0 as *u8) }
    
}

trait _wxScrollWinEvent : _wxEvent {
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

struct wxScrolledWindow(*u8);
impl _wxScrolledWindow for wxScrolledWindow {}
impl _wxPanel for wxScrolledWindow {}
impl _wxWindow for wxScrolledWindow {}
impl _wxEvtHandler for wxScrolledWindow {}
impl _wxObject for wxScrolledWindow { fn handle(&self) -> *u8 { **self } }

impl wxScrolledWindow {
    pub fn from(handle: *u8) -> @wxScrolledWindow { @wxScrolledWindow(handle) }
    pub fn null() -> @wxScrolledWindow { wxScrolledWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxScrolledWindow {
        unsafe { @wxScrolledWindow(wxScrolledWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxScrolledWindow : _wxPanel {
    #[fixed_stack_segment]
    #[inline(never)]
    fn adjustScrollbars(&self) {
        unsafe { wxScrolledWindow_AdjustScrollbars(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int) {
        unsafe { wxScrolledWindow_CalcScrolledPosition(self.handle(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int) {
        unsafe { wxScrolledWindow_CalcUnscrolledPosition(self.handle(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableScrolling(&self, x_scrolling: bool, y_scrolling: bool) {
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
    fn getScrollPixelsPerUnit(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetScrollPixelsPerUnit(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTargetWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxScrolledWindow_GetTargetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getViewStart(&self, _x: *c_int, _y: *c_int) {
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
    fn setScrollbars(&self, pixelsPerUnitX: c_int, pixelsPerUnitY: c_int, noUnitsX: c_int, noUnitsY: c_int, xPos: c_int, yPos: c_int, noRefresh: bool) {
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
    fn viewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_ViewStart(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.handle(), xstep, ystep) }
    }
}

struct wxSemaphore(*u8);
impl _wxSemaphore for wxSemaphore { fn handle(&self) -> *u8 { **self } }

impl wxSemaphore {
    pub fn from(handle: *u8) -> @wxSemaphore { @wxSemaphore(handle) }
    pub fn null() -> @wxSemaphore { wxSemaphore::from(0 as *u8) }
    
}

trait _wxSemaphore {
    fn handle(&self) -> *u8;
    
}

struct wxServer(*u8);
impl _wxServer for wxServer {}
impl _wxServerBase for wxServer {}
impl _wxObject for wxServer { fn handle(&self) -> *u8 { **self } }

impl wxServer {
    pub fn from(handle: *u8) -> @wxServer { @wxServer(handle) }
    pub fn null() -> @wxServer { wxServer::from(0 as *u8) }
    
}

trait _wxServer : _wxServerBase {
}

struct wxServerBase(*u8);
impl _wxServerBase for wxServerBase {}
impl _wxObject for wxServerBase { fn handle(&self) -> *u8 { **self } }

impl wxServerBase {
    pub fn from(handle: *u8) -> @wxServerBase { @wxServerBase(handle) }
    pub fn null() -> @wxServerBase { wxServerBase::from(0 as *u8) }
    
}

trait _wxServerBase : _wxObject {
}

struct wxSetCursorEvent(*u8);
impl _wxSetCursorEvent for wxSetCursorEvent {}
impl _wxEvent for wxSetCursorEvent {}
impl _wxObject for wxSetCursorEvent { fn handle(&self) -> *u8 { **self } }

impl wxSetCursorEvent {
    pub fn from(handle: *u8) -> @wxSetCursorEvent { @wxSetCursorEvent(handle) }
    pub fn null() -> @wxSetCursorEvent { wxSetCursorEvent::from(0 as *u8) }
    
}

trait _wxSetCursorEvent : _wxEvent {
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
    fn hasCursor(&self) -> bool {
        unsafe { wxSetCursorEvent_HasCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCursor<T: _wxCursor>(&self, cursor: &T) {
        unsafe { wxSetCursorEvent_SetCursor(self.handle(), cursor.handle()) }
    }
}

struct wxShowEvent(*u8);
impl _wxShowEvent for wxShowEvent {}
impl _wxEvent for wxShowEvent {}
impl _wxObject for wxShowEvent { fn handle(&self) -> *u8 { **self } }

impl wxShowEvent {
    pub fn from(handle: *u8) -> @wxShowEvent { @wxShowEvent(handle) }
    pub fn null() -> @wxShowEvent { wxShowEvent::from(0 as *u8) }
    
}

trait _wxShowEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> bool {
        unsafe { wxShowEvent_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setShow(&self, show: bool) {
        unsafe { wxShowEvent_SetShow(self.handle(), show) }
    }
}

struct wxSimpleHelpProvider(*u8);
impl _wxSimpleHelpProvider for wxSimpleHelpProvider {}
impl _wxHelpProvider for wxSimpleHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxSimpleHelpProvider {
    pub fn from(handle: *u8) -> @wxSimpleHelpProvider { @wxSimpleHelpProvider(handle) }
    pub fn null() -> @wxSimpleHelpProvider { wxSimpleHelpProvider::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxSimpleHelpProvider {
        unsafe { @wxSimpleHelpProvider(wxSimpleHelpProvider_Create()) }
    }
}

trait _wxSimpleHelpProvider : _wxHelpProvider {
}

struct wxSingleChoiceDialog(*u8);
impl _wxSingleChoiceDialog for wxSingleChoiceDialog {}
impl _wxDialog for wxSingleChoiceDialog {}
impl _wxTopLevelWindow for wxSingleChoiceDialog {}
impl _wxWindow for wxSingleChoiceDialog {}
impl _wxEvtHandler for wxSingleChoiceDialog {}
impl _wxObject for wxSingleChoiceDialog { fn handle(&self) -> *u8 { **self } }

impl wxSingleChoiceDialog {
    pub fn from(handle: *u8) -> @wxSingleChoiceDialog { @wxSingleChoiceDialog(handle) }
    pub fn null() -> @wxSingleChoiceDialog { wxSingleChoiceDialog::from(0 as *u8) }
    
}

trait _wxSingleChoiceDialog : _wxDialog {
}

struct wxSingleInstanceChecker(*u8);
impl _wxSingleInstanceChecker for wxSingleInstanceChecker { fn handle(&self) -> *u8 { **self } }

impl wxSingleInstanceChecker {
    pub fn from(handle: *u8) -> @wxSingleInstanceChecker { @wxSingleInstanceChecker(handle) }
    pub fn null() -> @wxSingleInstanceChecker { wxSingleInstanceChecker::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_obj: *u8, name: &str, path: &str) -> bool {
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

trait _wxSingleInstanceChecker {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isAnotherRunning(&self) -> bool {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.handle()) }
    }
}

struct wxSize(*u8);
impl _wxSize for wxSize { fn handle(&self) -> *u8 { **self } }

impl wxSize {
    pub fn from(handle: *u8) -> @wxSize { @wxSize(handle) }
    pub fn null() -> @wxSize { wxSize::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(w: c_int, h: c_int) -> @wxSize {
        unsafe { @wxSize(wxSize_Create(w, h)) }
    }
}

trait _wxSize {
    fn handle(&self) -> *u8;
    
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

struct wxSizeEvent(*u8);
impl _wxSizeEvent for wxSizeEvent {}
impl _wxEvent for wxSizeEvent {}
impl _wxObject for wxSizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxSizeEvent {
    pub fn from(handle: *u8) -> @wxSizeEvent { @wxSizeEvent(handle) }
    pub fn null() -> @wxSizeEvent { wxSizeEvent::from(0 as *u8) }
    
}

trait _wxSizeEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSize(wxSizeEvent_GetSize(self.handle())) }
    }
}

struct wxSizer(*u8);
impl _wxSizer for wxSizer {}
impl _wxObject for wxSizer { fn handle(&self) -> *u8 { **self } }

impl wxSizer {
    pub fn from(handle: *u8) -> @wxSizer { @wxSizer(handle) }
    pub fn null() -> @wxSizer { wxSizer::from(0 as *u8) }
    
}

trait _wxSizer : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Add(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSizer<T: _wxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addWindow<T: _wxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
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
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Insert(self.handle(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertSizer<T: _wxSizer>(&self, before: c_int, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertSizer(self.handle(), before, sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertWindow<T: _wxWindow>(&self, before: c_int, window: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertWindow(self.handle(), before, window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Prepend(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependSizer<T: _wxSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_PrependSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn prependWindow<T: _wxWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn clear(&self, delete_windows: bool) {
        unsafe { wxSizer_Clear(self.handle(), delete_windows) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachWindow<T: _wxWindow>(&self, window: &T) -> bool {
        unsafe { wxSizer_DetachWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detachSizer<T: _wxSizer>(&self, sizer: &T) -> bool {
        unsafe { wxSizer_DetachSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn detach(&self, index: c_int) -> bool {
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
    fn getItemWindow<T: _wxWindow>(&self, window: &T, recursive: bool) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemWindow(self.handle(), window.handle(), recursive)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItemSizer<T: _wxSizer>(&self, window: &T, recursive: bool) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemSizer(self.handle(), window.handle(), recursive)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getItem(&self, index: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItem(self.handle(), index)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideWindow<T: _wxWindow>(&self, window: &T) -> bool {
        unsafe { wxSizer_HideWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideSizer<T: _wxSizer>(&self, sizer: &T) -> bool {
        unsafe { wxSizer_HideSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self, index: c_int) -> bool {
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
    fn isShownWindow(&self, window: *u8) -> bool {
        unsafe { wxSizer_IsShownWindow(self.handle(), window) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShownSizer(&self, sizer: *u8) -> bool {
        unsafe { wxSizer_IsShownSizer(self.handle(), sizer) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self, index: c_int) -> bool {
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
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, oldwin: &T, newwin: &U, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceWindow(self.handle(), oldwin.handle(), newwin.handle(), recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceSizer<T: _wxSizer, U: _wxSizer>(&self, oldsz: &T, newsz: &U, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceSizer(self.handle(), oldsz.handle(), newsz.handle(), recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replace<T: _wxSizerItem>(&self, oldindex: c_int, newitem: &T) -> bool {
        unsafe { wxSizer_Replace(self.handle(), oldindex, newitem.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVirtualSizeHints<T: _wxWindow>(&self, window: &T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showWindow<T: _wxWindow>(&self, window: &T, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowWindow(self.handle(), window.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showSizer<T: _wxSizer>(&self, sizer: &T, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowSizer(self.handle(), sizer.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn show<T: _wxSizer>(&self, sizer: &T, index: c_int, show: bool) -> bool {
        unsafe { wxSizer_Show(self.handle(), sizer.handle(), index, show) }
    }
}

struct wxSizerItem(*u8);
impl _wxSizerItem for wxSizerItem {}
impl _wxObject for wxSizerItem { fn handle(&self) -> *u8 { **self } }

impl wxSizerItem {
    pub fn from(handle: *u8) -> @wxSizerItem { @wxSizerItem(handle) }
    pub fn null() -> @wxSizerItem { wxSizerItem::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) -> @wxSizerItem {
        unsafe { @wxSizerItem(wxSizerItem_Create(width, height, option, flag, border, userData)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newInSizer<T: _wxSizer>(sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInSizer(sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newInWindow<T: _wxWindow>(window: &T, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInWindow(window.handle(), option, flag, border, userData) }
    }
}

trait _wxSizerItem : _wxObject {
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
    fn getUserData(&self) -> *u8 {
        unsafe { wxSizerItem_GetUserData(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxSizerItem_GetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSizer(&self) -> bool {
        unsafe { wxSizerItem_IsSizer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSpacer(&self) -> bool {
        unsafe { wxSizerItem_IsSpacer(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isWindow(&self) -> bool {
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

struct wxSlider(*u8);
impl _wxSlider for wxSlider {}
impl _wxControl for wxSlider {}
impl _wxWindow for wxSlider {}
impl _wxEvtHandler for wxSlider {}
impl _wxObject for wxSlider { fn handle(&self) -> *u8 { **self } }

impl wxSlider {
    pub fn from(handle: *u8) -> @wxSlider { @wxSlider(handle) }
    pub fn null() -> @wxSlider { wxSlider::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSlider {
        unsafe { @wxSlider(wxSlider_Create(_prt.handle(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxSlider : _wxControl {
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

struct wxSockAddress(*u8);
impl _wxSockAddress for wxSockAddress {}
impl _wxObject for wxSockAddress { fn handle(&self) -> *u8 { **self } }

impl wxSockAddress {
    pub fn from(handle: *u8) -> @wxSockAddress { @wxSockAddress(handle) }
    pub fn null() -> @wxSockAddress { wxSockAddress::from(0 as *u8) }
    
}

trait _wxSockAddress : _wxObject {
}

struct wxSocketBase(*u8);
impl _wxSocketBase for wxSocketBase {}
impl _wxObject for wxSocketBase { fn handle(&self) -> *u8 { **self } }

impl wxSocketBase {
    pub fn from(handle: *u8) -> @wxSocketBase { @wxSocketBase(handle) }
    pub fn null() -> @wxSocketBase { wxSocketBase::from(0 as *u8) }
    
}

trait _wxSocketBase : _wxObject {
}

struct wxSocketClient(*u8);
impl _wxSocketClient for wxSocketClient {}
impl _wxSocketBase for wxSocketClient {}
impl _wxObject for wxSocketClient { fn handle(&self) -> *u8 { **self } }

impl wxSocketClient {
    pub fn from(handle: *u8) -> @wxSocketClient { @wxSocketClient(handle) }
    pub fn null() -> @wxSocketClient { wxSocketClient::from(0 as *u8) }
    
}

trait _wxSocketClient : _wxSocketBase {
}

struct wxSocketEvent(*u8);
impl _wxSocketEvent for wxSocketEvent {}
impl _wxEvent for wxSocketEvent {}
impl _wxObject for wxSocketEvent { fn handle(&self) -> *u8 { **self } }

impl wxSocketEvent {
    pub fn from(handle: *u8) -> @wxSocketEvent { @wxSocketEvent(handle) }
    pub fn null() -> @wxSocketEvent { wxSocketEvent::from(0 as *u8) }
    
}

trait _wxSocketEvent : _wxEvent {
}

struct wxSocketInputStream(*u8);
impl _wxSocketInputStream for wxSocketInputStream {}
impl _wxInputStream for wxSocketInputStream {}
impl _wxStreamBase for wxSocketInputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketInputStream {
    pub fn from(handle: *u8) -> @wxSocketInputStream { @wxSocketInputStream(handle) }
    pub fn null() -> @wxSocketInputStream { wxSocketInputStream::from(0 as *u8) }
    
}

trait _wxSocketInputStream : _wxInputStream {
}

struct wxSocketOutputStream(*u8);
impl _wxSocketOutputStream for wxSocketOutputStream {}
impl _wxOutputStream for wxSocketOutputStream {}
impl _wxStreamBase for wxSocketOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketOutputStream {
    pub fn from(handle: *u8) -> @wxSocketOutputStream { @wxSocketOutputStream(handle) }
    pub fn null() -> @wxSocketOutputStream { wxSocketOutputStream::from(0 as *u8) }
    
}

trait _wxSocketOutputStream : _wxOutputStream {
}

struct wxSocketServer(*u8);
impl _wxSocketServer for wxSocketServer {}
impl _wxSocketBase for wxSocketServer {}
impl _wxObject for wxSocketServer { fn handle(&self) -> *u8 { **self } }

impl wxSocketServer {
    pub fn from(handle: *u8) -> @wxSocketServer { @wxSocketServer(handle) }
    pub fn null() -> @wxSocketServer { wxSocketServer::from(0 as *u8) }
    
}

trait _wxSocketServer : _wxSocketBase {
}

struct wxSpinButton(*u8);
impl _wxSpinButton for wxSpinButton {}
impl _wxControl for wxSpinButton {}
impl _wxWindow for wxSpinButton {}
impl _wxEvtHandler for wxSpinButton {}
impl _wxObject for wxSpinButton { fn handle(&self) -> *u8 { **self } }

impl wxSpinButton {
    pub fn from(handle: *u8) -> @wxSpinButton { @wxSpinButton(handle) }
    pub fn null() -> @wxSpinButton { wxSpinButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSpinButton {
        unsafe { @wxSpinButton(wxSpinButton_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxSpinButton : _wxControl {
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

struct wxSpinCtrl(*u8);
impl _wxSpinCtrl for wxSpinCtrl {}
impl _wxControl for wxSpinCtrl {}
impl _wxWindow for wxSpinCtrl {}
impl _wxEvtHandler for wxSpinCtrl {}
impl _wxObject for wxSpinCtrl { fn handle(&self) -> *u8 { **self } }

impl wxSpinCtrl {
    pub fn from(handle: *u8) -> @wxSpinCtrl { @wxSpinCtrl(handle) }
    pub fn null() -> @wxSpinCtrl { wxSpinCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> @wxSpinCtrl {
        let _txt = wxT(_txt);
        unsafe { @wxSpinCtrl(wxSpinCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) }
    }
}

trait _wxSpinCtrl : _wxControl {
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

struct wxSpinEvent(*u8);
impl _wxSpinEvent for wxSpinEvent {}
impl _wxNotifyEvent for wxSpinEvent {}
impl _wxCommandEvent for wxSpinEvent {}
impl _wxEvent for wxSpinEvent {}
impl _wxObject for wxSpinEvent { fn handle(&self) -> *u8 { **self } }

impl wxSpinEvent {
    pub fn from(handle: *u8) -> @wxSpinEvent { @wxSpinEvent(handle) }
    pub fn null() -> @wxSpinEvent { wxSpinEvent::from(0 as *u8) }
    
}

trait _wxSpinEvent : _wxNotifyEvent {
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

struct wxSplashScreen(*u8);
impl _wxSplashScreen for wxSplashScreen {}
impl _wxFrame for wxSplashScreen {}
impl _wxTopLevelWindow for wxSplashScreen {}
impl _wxWindow for wxSplashScreen {}
impl _wxEvtHandler for wxSplashScreen {}
impl _wxObject for wxSplashScreen { fn handle(&self) -> *u8 { **self } }

impl wxSplashScreen {
    pub fn from(handle: *u8) -> @wxSplashScreen { @wxSplashScreen(handle) }
    pub fn null() -> @wxSplashScreen { wxSplashScreen::from(0 as *u8) }
    
}

trait _wxSplashScreen : _wxFrame {
}

struct wxSplitterEvent(*u8);
impl _wxSplitterEvent for wxSplitterEvent {}
impl _wxNotifyEvent for wxSplitterEvent {}
impl _wxCommandEvent for wxSplitterEvent {}
impl _wxEvent for wxSplitterEvent {}
impl _wxObject for wxSplitterEvent { fn handle(&self) -> *u8 { **self } }

impl wxSplitterEvent {
    pub fn from(handle: *u8) -> @wxSplitterEvent { @wxSplitterEvent(handle) }
    pub fn null() -> @wxSplitterEvent { wxSplitterEvent::from(0 as *u8) }
    
}

trait _wxSplitterEvent : _wxNotifyEvent {
}

struct wxSplitterScrolledWindow(*u8);
impl _wxSplitterScrolledWindow for wxSplitterScrolledWindow {}
impl _wxScrolledWindow for wxSplitterScrolledWindow {}
impl _wxPanel for wxSplitterScrolledWindow {}
impl _wxWindow for wxSplitterScrolledWindow {}
impl _wxEvtHandler for wxSplitterScrolledWindow {}
impl _wxObject for wxSplitterScrolledWindow { fn handle(&self) -> *u8 { **self } }

impl wxSplitterScrolledWindow {
    pub fn from(handle: *u8) -> @wxSplitterScrolledWindow { @wxSplitterScrolledWindow(handle) }
    pub fn null() -> @wxSplitterScrolledWindow { wxSplitterScrolledWindow::from(0 as *u8) }
    
}

trait _wxSplitterScrolledWindow : _wxScrolledWindow {
}

struct wxSplitterWindow(*u8);
impl _wxSplitterWindow for wxSplitterWindow {}
impl _wxWindow for wxSplitterWindow {}
impl _wxEvtHandler for wxSplitterWindow {}
impl _wxObject for wxSplitterWindow { fn handle(&self) -> *u8 { **self } }

impl wxSplitterWindow {
    pub fn from(handle: *u8) -> @wxSplitterWindow { @wxSplitterWindow(handle) }
    pub fn null() -> @wxSplitterWindow { wxSplitterWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxSplitterWindow {
        unsafe { @wxSplitterWindow(wxSplitterWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxSplitterWindow : _wxWindow {
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
    fn isSplit(&self) -> bool {
        unsafe { wxSplitterWindow_IsSplit(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, winOld: &T, winNew: &U) -> bool {
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
    fn setSashPosition(&self, position: c_int, redraw: bool) {
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
    fn splitHorizontally<T: _wxWindow, U: _wxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitHorizontally(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn splitVertically<T: _wxWindow, U: _wxWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitVertically(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unsplit<T: _wxWindow>(&self, toRemove: &T) -> bool {
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

struct wxStaticBitmap(*u8);
impl _wxStaticBitmap for wxStaticBitmap {}
impl _wxControl for wxStaticBitmap {}
impl _wxWindow for wxStaticBitmap {}
impl _wxEvtHandler for wxStaticBitmap {}
impl _wxObject for wxStaticBitmap { fn handle(&self) -> *u8 { **self } }

impl wxStaticBitmap {
    pub fn from(handle: *u8) -> @wxStaticBitmap { @wxStaticBitmap(handle) }
    pub fn null() -> @wxStaticBitmap { wxStaticBitmap::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, bitmap: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticBitmap {
        unsafe { @wxStaticBitmap(wxStaticBitmap_Create(_prt.handle(), _id, bitmap.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxStaticBitmap : _wxControl {
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

struct wxStaticBox(*u8);
impl _wxStaticBox for wxStaticBox {}
impl _wxControl for wxStaticBox {}
impl _wxWindow for wxStaticBox {}
impl _wxEvtHandler for wxStaticBox {}
impl _wxObject for wxStaticBox { fn handle(&self) -> *u8 { **self } }

impl wxStaticBox {
    pub fn from(handle: *u8) -> @wxStaticBox { @wxStaticBox(handle) }
    pub fn null() -> @wxStaticBox { wxStaticBox::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticBox {
        let _txt = wxT(_txt);
        unsafe { @wxStaticBox(wxStaticBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxStaticBox : _wxControl {
}

struct wxStaticBoxSizer(*u8);
impl _wxStaticBoxSizer for wxStaticBoxSizer {}
impl _wxBoxSizer for wxStaticBoxSizer {}
impl _wxSizer for wxStaticBoxSizer {}
impl _wxObject for wxStaticBoxSizer { fn handle(&self) -> *u8 { **self } }

impl wxStaticBoxSizer {
    pub fn from(handle: *u8) -> @wxStaticBoxSizer { @wxStaticBoxSizer(handle) }
    pub fn null() -> @wxStaticBoxSizer { wxStaticBoxSizer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxStaticBox>(box: &T, orient: c_int) -> @wxStaticBoxSizer {
        unsafe { @wxStaticBoxSizer(wxStaticBoxSizer_Create(box.handle(), orient)) }
    }
}

trait _wxStaticBoxSizer : _wxBoxSizer {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBox(&self) -> @wxStaticBox {
        unsafe { @wxStaticBox(wxStaticBoxSizer_GetStaticBox(self.handle())) }
    }
}

struct wxStaticLine(*u8);
impl _wxStaticLine for wxStaticLine {}
impl _wxControl for wxStaticLine {}
impl _wxWindow for wxStaticLine {}
impl _wxEvtHandler for wxStaticLine {}
impl _wxObject for wxStaticLine { fn handle(&self) -> *u8 { **self } }

impl wxStaticLine {
    pub fn from(handle: *u8) -> @wxStaticLine { @wxStaticLine(handle) }
    pub fn null() -> @wxStaticLine { wxStaticLine::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticLine {
        unsafe { @wxStaticLine(wxStaticLine_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxStaticLine : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVertical(&self) -> bool {
        unsafe { wxStaticLine_IsVertical(self.handle()) }
    }
}

struct wxStaticText(*u8);
impl _wxStaticText for wxStaticText {}
impl _wxControl for wxStaticText {}
impl _wxWindow for wxStaticText {}
impl _wxEvtHandler for wxStaticText {}
impl _wxObject for wxStaticText { fn handle(&self) -> *u8 { **self } }

impl wxStaticText {
    pub fn from(handle: *u8) -> @wxStaticText { @wxStaticText(handle) }
    pub fn null() -> @wxStaticText { wxStaticText::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticText {
        let _txt = wxT(_txt);
        unsafe { @wxStaticText(wxStaticText_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxStaticText : _wxControl {
}

struct wxStatusBar(*u8);
impl _wxStatusBar for wxStatusBar {}
impl _wxWindow for wxStatusBar {}
impl _wxEvtHandler for wxStatusBar {}
impl _wxObject for wxStatusBar { fn handle(&self) -> *u8 { **self } }

impl wxStatusBar {
    pub fn from(handle: *u8) -> @wxStatusBar { @wxStatusBar(handle) }
    pub fn null() -> @wxStatusBar { wxStatusBar::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStatusBar {
        unsafe { @wxStatusBar(wxStatusBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxStatusBar : _wxWindow {
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
    fn setFieldsCount(&self, number: c_int, widths: *c_int) {
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
    fn setStatusWidths(&self, n: c_int, widths: *c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.handle(), n, widths) }
    }
}

struct wxStopWatch(*u8);
impl _wxStopWatch for wxStopWatch { fn handle(&self) -> *u8 { **self } }

impl wxStopWatch {
    pub fn from(handle: *u8) -> @wxStopWatch { @wxStopWatch(handle) }
    pub fn null() -> @wxStopWatch { wxStopWatch::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxStopWatch {
        unsafe { @wxStopWatch(wxStopWatch_Create()) }
    }
}

trait _wxStopWatch {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxStopWatch_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self, msec: c_int) {
        unsafe { wxStopWatch_Start(self.handle(), msec) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pause(&self) {
        unsafe { wxStopWatch_Pause(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn resume(&self) {
        unsafe { wxStopWatch_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn time(&self) -> c_int {
        unsafe { wxStopWatch_Time(self.handle()) }
    }
}

struct wxStreamBase(*u8);
impl _wxStreamBase for wxStreamBase { fn handle(&self) -> *u8 { **self } }

impl wxStreamBase {
    pub fn from(handle: *u8) -> @wxStreamBase { @wxStreamBase(handle) }
    pub fn null() -> @wxStreamBase { wxStreamBase::from(0 as *u8) }
    
}

trait _wxStreamBase {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLastError(&self) -> c_int {
        unsafe { wxStreamBase_GetLastError(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSize(&self) -> c_int {
        unsafe { wxStreamBase_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxStreamBase_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxStreamBase_Delete(self.handle()) }
    }
}

struct wxStreamBuffer(*u8);
impl _wxStreamBuffer for wxStreamBuffer { fn handle(&self) -> *u8 { **self } }

impl wxStreamBuffer {
    pub fn from(handle: *u8) -> @wxStreamBuffer { @wxStreamBuffer(handle) }
    pub fn null() -> @wxStreamBuffer { wxStreamBuffer::from(0 as *u8) }
    
}

trait _wxStreamBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStreamToTextRedirector(*u8);
impl _wxStreamToTextRedirector for wxStreamToTextRedirector { fn handle(&self) -> *u8 { **self } }

impl wxStreamToTextRedirector {
    pub fn from(handle: *u8) -> @wxStreamToTextRedirector { @wxStreamToTextRedirector(handle) }
    pub fn null() -> @wxStreamToTextRedirector { wxStreamToTextRedirector::from(0 as *u8) }
    
}

trait _wxStreamToTextRedirector {
    fn handle(&self) -> *u8;
    
}

struct wxStringBuffer(*u8);
impl _wxStringBuffer for wxStringBuffer { fn handle(&self) -> *u8 { **self } }

impl wxStringBuffer {
    pub fn from(handle: *u8) -> @wxStringBuffer { @wxStringBuffer(handle) }
    pub fn null() -> @wxStringBuffer { wxStringBuffer::from(0 as *u8) }
    
}

trait _wxStringBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStringClientData(*u8);
impl _wxStringClientData for wxStringClientData {}
impl _wxClientData for wxStringClientData { fn handle(&self) -> *u8 { **self } }

impl wxStringClientData {
    pub fn from(handle: *u8) -> @wxStringClientData { @wxStringClientData(handle) }
    pub fn null() -> @wxStringClientData { wxStringClientData::from(0 as *u8) }
    
}

trait _wxStringClientData : _wxClientData {
}

struct wxStringList(*u8);
impl _wxStringList for wxStringList {}
impl _wxList for wxStringList {}
impl _wxObject for wxStringList { fn handle(&self) -> *u8 { **self } }

impl wxStringList {
    pub fn from(handle: *u8) -> @wxStringList { @wxStringList(handle) }
    pub fn null() -> @wxStringList { wxStringList::from(0 as *u8) }
    
}

trait _wxStringList : _wxList {
}

struct wxStringTokenizer(*u8);
impl _wxStringTokenizer for wxStringTokenizer {}
impl _wxObject for wxStringTokenizer { fn handle(&self) -> *u8 { **self } }

impl wxStringTokenizer {
    pub fn from(handle: *u8) -> @wxStringTokenizer { @wxStringTokenizer(handle) }
    pub fn null() -> @wxStringTokenizer { wxStringTokenizer::from(0 as *u8) }
    
}

trait _wxStringTokenizer : _wxObject {
}

struct wxSysColourChangedEvent(*u8);
impl _wxSysColourChangedEvent for wxSysColourChangedEvent {}
impl _wxEvent for wxSysColourChangedEvent {}
impl _wxObject for wxSysColourChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxSysColourChangedEvent {
    pub fn from(handle: *u8) -> @wxSysColourChangedEvent { @wxSysColourChangedEvent(handle) }
    pub fn null() -> @wxSysColourChangedEvent { wxSysColourChangedEvent::from(0 as *u8) }
    
}

trait _wxSysColourChangedEvent : _wxEvent {
}

struct wxSystemOptions(*u8);
impl _wxSystemOptions for wxSystemOptions {}
impl _wxObject for wxSystemOptions { fn handle(&self) -> *u8 { **self } }

impl wxSystemOptions {
    pub fn from(handle: *u8) -> @wxSystemOptions { @wxSystemOptions(handle) }
    pub fn null() -> @wxSystemOptions { wxSystemOptions::from(0 as *u8) }
    
}

trait _wxSystemOptions : _wxObject {
}

struct wxSystemSettings(*u8);
impl _wxSystemSettings for wxSystemSettings {}
impl _wxObject for wxSystemSettings { fn handle(&self) -> *u8 { **self } }

impl wxSystemSettings {
    pub fn from(handle: *u8) -> @wxSystemSettings { @wxSystemSettings(handle) }
    pub fn null() -> @wxSystemSettings { wxSystemSettings::from(0 as *u8) }
    
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

trait _wxSystemSettings : _wxObject {
}

struct wxTabCtrl(*u8);
impl _wxTabCtrl for wxTabCtrl {}
impl _wxControl for wxTabCtrl {}
impl _wxWindow for wxTabCtrl {}
impl _wxEvtHandler for wxTabCtrl {}
impl _wxObject for wxTabCtrl { fn handle(&self) -> *u8 { **self } }

impl wxTabCtrl {
    pub fn from(handle: *u8) -> @wxTabCtrl { @wxTabCtrl(handle) }
    pub fn null() -> @wxTabCtrl { wxTabCtrl::from(0 as *u8) }
    
}

trait _wxTabCtrl : _wxControl {
}

struct wxTabEvent(*u8);
impl _wxTabEvent for wxTabEvent {}
impl _wxCommandEvent for wxTabEvent {}
impl _wxEvent for wxTabEvent {}
impl _wxObject for wxTabEvent { fn handle(&self) -> *u8 { **self } }

impl wxTabEvent {
    pub fn from(handle: *u8) -> @wxTabEvent { @wxTabEvent(handle) }
    pub fn null() -> @wxTabEvent { wxTabEvent::from(0 as *u8) }
    
}

trait _wxTabEvent : _wxCommandEvent {
}

struct wxTablesInUse(*u8);
impl _wxTablesInUse for wxTablesInUse {}
impl _wxObject for wxTablesInUse { fn handle(&self) -> *u8 { **self } }

impl wxTablesInUse {
    pub fn from(handle: *u8) -> @wxTablesInUse { @wxTablesInUse(handle) }
    pub fn null() -> @wxTablesInUse { wxTablesInUse::from(0 as *u8) }
    
}

trait _wxTablesInUse : _wxObject {
}

struct wxTaskBarIcon(*u8);
impl _wxTaskBarIcon for wxTaskBarIcon {}
impl _wxEvtHandler for wxTaskBarIcon {}
impl _wxObject for wxTaskBarIcon { fn handle(&self) -> *u8 { **self } }

impl wxTaskBarIcon {
    pub fn from(handle: *u8) -> @wxTaskBarIcon { @wxTaskBarIcon(handle) }
    pub fn null() -> @wxTaskBarIcon { wxTaskBarIcon::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxTaskBarIcon {
        unsafe { @wxTaskBarIcon(wxTaskBarIcon_Create()) }
    }
}

trait _wxTaskBarIcon : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIconInstalled(&self) -> bool {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxTaskBarIcon_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn popupMenu<T: _wxMenu>(&self, menu: &T) -> bool {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn removeIcon(&self) -> bool {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIcon<T: _wxIcon>(&self, icon: &T, text: &str) -> bool {
        let text = wxT(text);
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

struct wxTempFile(*u8);
impl _wxTempFile for wxTempFile { fn handle(&self) -> *u8 { **self } }

impl wxTempFile {
    pub fn from(handle: *u8) -> @wxTempFile { @wxTempFile(handle) }
    pub fn null() -> @wxTempFile { wxTempFile::from(0 as *u8) }
    
}

trait _wxTempFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextAttr(*u8);
impl _wxTextAttr for wxTextAttr { fn handle(&self) -> *u8 { **self } }

impl wxTextAttr {
    pub fn from(handle: *u8) -> @wxTextAttr { @wxTextAttr(handle) }
    pub fn null() -> @wxTextAttr { wxTextAttr::from(0 as *u8) }
    
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

trait _wxTextAttr {
    fn handle(&self) -> *u8;
    
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
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxTextAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasFont(&self) -> bool {
        unsafe { wxTextAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasTextColour(&self) -> bool {
        unsafe { wxTextAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isDefault(&self) -> bool {
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

struct wxTextCtrl(*u8);
impl _wxTextCtrl for wxTextCtrl {}
impl _wxControl for wxTextCtrl {}
impl _wxWindow for wxTextCtrl {}
impl _wxEvtHandler for wxTextCtrl {}
impl _wxObject for wxTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxTextCtrl {
    pub fn from(handle: *u8) -> @wxTextCtrl { @wxTextCtrl(handle) }
    pub fn null() -> @wxTextCtrl { wxTextCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxTextCtrl {
        let _txt = wxT(_txt);
        unsafe { @wxTextCtrl(wxTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxTextCtrl : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendText(&self, text: &str) {
        let text = wxT(text);
        unsafe { wxTextCtrl_AppendText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canCopy(&self) -> bool {
        unsafe { wxTextCtrl_CanCopy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canCut(&self) -> bool {
        unsafe { wxTextCtrl_CanCut(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canPaste(&self) -> bool {
        unsafe { wxTextCtrl_CanPaste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canRedo(&self) -> bool {
        unsafe { wxTextCtrl_CanRedo(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canUndo(&self) -> bool {
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
    fn getSelection(&self, from: *u8, to: *u8) {
        unsafe { wxTextCtrl_GetSelection(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> ~str {
        unsafe { wxString { handle: wxTextCtrl_GetValue(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEditable(&self) -> bool {
        unsafe { wxTextCtrl_IsEditable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isModified(&self) -> bool {
        unsafe { wxTextCtrl_IsModified(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, file: &str) -> bool {
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
    fn positionToXY(&self, pos: c_long, x: *c_long, y: *c_long) -> c_int {
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
    fn saveFile(&self, file: &str) -> bool {
        let file = wxT(file);
        unsafe { wxTextCtrl_SaveFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setEditable(&self, editable: bool) {
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
    fn emulateKeyPress<T: _wxKeyEvent>(&self, keyevent: &T) -> bool {
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
    fn isMultiLine(&self) -> bool {
        unsafe { wxTextCtrl_IsMultiLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSingleLine(&self) -> bool {
        unsafe { wxTextCtrl_IsSingleLine(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDefaultStyle<T: _wxTextAttr>(&self, style: &T) -> bool {
        unsafe { wxTextCtrl_SetDefaultStyle(self.handle(), style.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setStyle<T: _wxTextAttr>(&self, start: c_long, end: c_long, style: &T) -> bool {
        unsafe { wxTextCtrl_SetStyle(self.handle(), start, end, style.handle()) }
    }
}

struct wxTextDataObject(*u8);
impl _wxTextDataObject for wxTextDataObject {}
impl _wxDataObjectSimple for wxTextDataObject {}
impl _wxDataObject for wxTextDataObject { fn handle(&self) -> *u8 { **self } }

impl wxTextDataObject {
    pub fn from(handle: *u8) -> @wxTextDataObject { @wxTextDataObject(handle) }
    pub fn null() -> @wxTextDataObject { wxTextDataObject::from(0 as *u8) }
    
}

trait _wxTextDataObject : _wxDataObjectSimple {
}

struct wxTextDropTarget(*u8);
impl _wxTextDropTarget for wxTextDropTarget {}
impl _wxDropTarget for wxTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxTextDropTarget {
    pub fn from(handle: *u8) -> @wxTextDropTarget { @wxTextDropTarget(handle) }
    pub fn null() -> @wxTextDropTarget { wxTextDropTarget::from(0 as *u8) }
    
}

trait _wxTextDropTarget : _wxDropTarget {
}

struct wxTextEntryDialog(*u8);
impl _wxTextEntryDialog for wxTextEntryDialog {}
impl _wxDialog for wxTextEntryDialog {}
impl _wxTopLevelWindow for wxTextEntryDialog {}
impl _wxWindow for wxTextEntryDialog {}
impl _wxEvtHandler for wxTextEntryDialog {}
impl _wxObject for wxTextEntryDialog { fn handle(&self) -> *u8 { **self } }

impl wxTextEntryDialog {
    pub fn from(handle: *u8) -> @wxTextEntryDialog { @wxTextEntryDialog(handle) }
    pub fn null() -> @wxTextEntryDialog { wxTextEntryDialog::from(0 as *u8) }
    
}

trait _wxTextEntryDialog : _wxDialog {
}

struct wxTextFile(*u8);
impl _wxTextFile for wxTextFile { fn handle(&self) -> *u8 { **self } }

impl wxTextFile {
    pub fn from(handle: *u8) -> @wxTextFile { @wxTextFile(handle) }
    pub fn null() -> @wxTextFile { wxTextFile::from(0 as *u8) }
    
}

trait _wxTextFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextInputStream(*u8);
impl _wxTextInputStream for wxTextInputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextInputStream {
    pub fn from(handle: *u8) -> @wxTextInputStream { @wxTextInputStream(handle) }
    pub fn null() -> @wxTextInputStream { wxTextInputStream::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream>(inputStream: &T, sep: &str) -> @wxTextInputStream {
        let sep = wxT(sep);
        unsafe { @wxTextInputStream(wxTextInputStream_Create(inputStream.handle(), sep.handle())) }
    }
}

trait _wxTextInputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn readLine(&self) -> ~str {
        unsafe { wxString { handle: wxTextInputStream_ReadLine(self.handle()) }.to_str() }
    }
}

struct wxTextOutputStream(*u8);
impl _wxTextOutputStream for wxTextOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextOutputStream {
    pub fn from(handle: *u8) -> @wxTextOutputStream { @wxTextOutputStream(handle) }
    pub fn null() -> @wxTextOutputStream { wxTextOutputStream::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxOutputStream>(outputStream: &T, mode: c_int) -> @wxTextOutputStream {
        unsafe { @wxTextOutputStream(wxTextOutputStream_Create(outputStream.handle(), mode)) }
    }
}

trait _wxTextOutputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn writeString(&self, txt: &str) {
        let txt = wxT(txt);
        unsafe { wxTextOutputStream_WriteString(self.handle(), txt.handle()) }
    }
}

struct wxTextValidator(*u8);
impl _wxTextValidator for wxTextValidator {}
impl _wxValidator for wxTextValidator {}
impl _wxEvtHandler for wxTextValidator {}
impl _wxObject for wxTextValidator { fn handle(&self) -> *u8 { **self } }

impl wxTextValidator {
    pub fn from(handle: *u8) -> @wxTextValidator { @wxTextValidator(handle) }
    pub fn null() -> @wxTextValidator { wxTextValidator::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(style: c_int, val: *u8) -> @wxTextValidator {
        unsafe { @wxTextValidator(wxTextValidator_Create(style, val)) }
    }
}

trait _wxTextValidator : _wxValidator {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getExcludes(&self, _ref: *wchar_t) -> c_int {
        unsafe { wxTextValidator_GetExcludes(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIncludes(&self, _ref: *wchar_t) -> c_int {
        unsafe { wxTextValidator_GetIncludes(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setExcludes(&self, list: *wchar_t, count: c_int) {
        unsafe { wxTextValidator_SetExcludes(self.handle(), list, count) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIncludes(&self, list: *wchar_t, count: c_int) {
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

struct wxThinSplitterWindow(*u8);
impl _wxThinSplitterWindow for wxThinSplitterWindow {}
impl _wxSplitterWindow for wxThinSplitterWindow {}
impl _wxWindow for wxThinSplitterWindow {}
impl _wxEvtHandler for wxThinSplitterWindow {}
impl _wxObject for wxThinSplitterWindow { fn handle(&self) -> *u8 { **self } }

impl wxThinSplitterWindow {
    pub fn from(handle: *u8) -> @wxThinSplitterWindow { @wxThinSplitterWindow(handle) }
    pub fn null() -> @wxThinSplitterWindow { wxThinSplitterWindow::from(0 as *u8) }
    
}

trait _wxThinSplitterWindow : _wxSplitterWindow {
}

struct wxThread(*u8);
impl _wxThread for wxThread { fn handle(&self) -> *u8 { **self } }

impl wxThread {
    pub fn from(handle: *u8) -> @wxThread { @wxThread(handle) }
    pub fn null() -> @wxThread { wxThread::from(0 as *u8) }
    
}

trait _wxThread {
    fn handle(&self) -> *u8;
    
}

struct wxTime(*u8);
impl _wxTime for wxTime {}
impl _wxObject for wxTime { fn handle(&self) -> *u8 { **self } }

impl wxTime {
    pub fn from(handle: *u8) -> @wxTime { @wxTime(handle) }
    pub fn null() -> @wxTime { wxTime::from(0 as *u8) }
    
}

trait _wxTime : _wxObject {
}

struct wxTimeSpan(*u8);
impl _wxTimeSpan for wxTimeSpan { fn handle(&self) -> *u8 { **self } }

impl wxTimeSpan {
    pub fn from(handle: *u8) -> @wxTimeSpan { @wxTimeSpan(handle) }
    pub fn null() -> @wxTimeSpan { wxTimeSpan::from(0 as *u8) }
    
}

trait _wxTimeSpan {
    fn handle(&self) -> *u8;
    
}

struct wxTimer(*u8);
impl _wxTimer for wxTimer {}
impl _wxObject for wxTimer { fn handle(&self) -> *u8 { **self } }

impl wxTimer {
    pub fn from(handle: *u8) -> @wxTimer { @wxTimer(handle) }
    pub fn null() -> @wxTimer { wxTimer::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int) -> @wxTimer {
        unsafe { @wxTimer(wxTimer_Create(_prt.handle(), _id)) }
    }
}

trait _wxTimer : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimer_GetInterval(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOneShot(&self) -> bool {
        unsafe { wxTimer_IsOneShot(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isRuning(&self) -> bool {
        unsafe { wxTimer_IsRuning(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn start(&self, _int: c_int, _one: bool) -> bool {
        unsafe { wxTimer_Start(self.handle(), _int, _one) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) {
        unsafe { wxTimer_Stop(self.handle()) }
    }
}

struct wxTimerBase(*u8);
impl _wxTimerBase for wxTimerBase {}
impl _wxObject for wxTimerBase { fn handle(&self) -> *u8 { **self } }

impl wxTimerBase {
    pub fn from(handle: *u8) -> @wxTimerBase { @wxTimerBase(handle) }
    pub fn null() -> @wxTimerBase { wxTimerBase::from(0 as *u8) }
    
}

trait _wxTimerBase : _wxObject {
}

struct wxTimerEvent(*u8);
impl _wxTimerEvent for wxTimerEvent {}
impl _wxEvent for wxTimerEvent {}
impl _wxObject for wxTimerEvent { fn handle(&self) -> *u8 { **self } }

impl wxTimerEvent {
    pub fn from(handle: *u8) -> @wxTimerEvent { @wxTimerEvent(handle) }
    pub fn null() -> @wxTimerEvent { wxTimerEvent::from(0 as *u8) }
    
}

trait _wxTimerEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.handle()) }
    }
}

struct wxTimerEx(*u8);
impl _wxTimerEx for wxTimerEx {}
impl _wxTimer for wxTimerEx {}
impl _wxObject for wxTimerEx { fn handle(&self) -> *u8 { **self } }

impl wxTimerEx {
    pub fn from(handle: *u8) -> @wxTimerEx { @wxTimerEx(handle) }
    pub fn null() -> @wxTimerEx { wxTimerEx::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxTimerEx {
        unsafe { @wxTimerEx(wxTimerEx_Create()) }
    }
}

trait _wxTimerEx : _wxTimer {
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

struct wxTimerRunner(*u8);
impl _wxTimerRunner for wxTimerRunner { fn handle(&self) -> *u8 { **self } }

impl wxTimerRunner {
    pub fn from(handle: *u8) -> @wxTimerRunner { @wxTimerRunner(handle) }
    pub fn null() -> @wxTimerRunner { wxTimerRunner::from(0 as *u8) }
    
}

trait _wxTimerRunner {
    fn handle(&self) -> *u8;
    
}

struct wxTipProvider(*u8);
impl _wxTipProvider for wxTipProvider { fn handle(&self) -> *u8 { **self } }

impl wxTipProvider {
    pub fn from(handle: *u8) -> @wxTipProvider { @wxTipProvider(handle) }
    pub fn null() -> @wxTipProvider { wxTipProvider::from(0 as *u8) }
    
}

trait _wxTipProvider {
    fn handle(&self) -> *u8;
    
}

struct wxTipWindow(*u8);
impl _wxTipWindow for wxTipWindow {}
impl _wxPopupTransientWindow for wxTipWindow {}
impl _wxPopupWindow for wxTipWindow {}
impl _wxWindow for wxTipWindow {}
impl _wxEvtHandler for wxTipWindow {}
impl _wxObject for wxTipWindow { fn handle(&self) -> *u8 { **self } }

impl wxTipWindow {
    pub fn from(handle: *u8) -> @wxTipWindow { @wxTipWindow(handle) }
    pub fn null() -> @wxTipWindow { wxTipWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, text: &str, maxLength: c_int) -> @wxTipWindow {
        let text = wxT(text);
        unsafe { @wxTipWindow(wxTipWindow_Create(parent.handle(), text.handle(), maxLength)) }
    }
}

trait _wxTipWindow : _wxPopupTransientWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTipWindowPtr(&self, windowPtr: *u8) {
        unsafe { wxTipWindow_SetTipWindowPtr(self.handle(), windowPtr) }
    }
}

struct wxToggleButton(*u8);
impl _wxToggleButton for wxToggleButton {}
impl _wxControl for wxToggleButton {}
impl _wxWindow for wxToggleButton {}
impl _wxEvtHandler for wxToggleButton {}
impl _wxObject for wxToggleButton { fn handle(&self) -> *u8 { **self } }

impl wxToggleButton {
    pub fn from(handle: *u8) -> @wxToggleButton { @wxToggleButton(handle) }
    pub fn null() -> @wxToggleButton { wxToggleButton::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, id: c_int, label: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxToggleButton {
        let label = wxT(label);
        unsafe { @wxToggleButton(wxToggleButton_Create(parent.handle(), id, label.handle(), x, y, w, h, style)) }
    }
}

trait _wxToggleButton : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValue(&self) -> bool {
        unsafe { wxToggleButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setValue(&self, state: bool) {
        unsafe { wxToggleButton_SetValue(self.handle(), state) }
    }
}

struct wxToolBar(*u8);
impl _wxToolBar for wxToolBar {}
impl _wxToolBarBase for wxToolBar {}
impl _wxControl for wxToolBar {}
impl _wxWindow for wxToolBar {}
impl _wxEvtHandler for wxToolBar {}
impl _wxObject for wxToolBar { fn handle(&self) -> *u8 { **self } }

impl wxToolBar {
    pub fn from(handle: *u8) -> @wxToolBar { @wxToolBar(handle) }
    pub fn null() -> @wxToolBar { wxToolBar::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxToolBar {
        unsafe { @wxToolBar(wxToolBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxToolBar : _wxToolBarBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addControl<T: _wxControl>(&self, ctrl: &T) -> bool {
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
    fn addToolEx<T: _wxBitmap, U: _wxBitmap, V: _wxObject>(&self, id: c_int, bmp1: &T, bmp2: &U, isToggle: bool, x: c_int, y: c_int, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddToolEx(self.handle(), id, bmp1.handle(), bmp2.handle(), isToggle, x, y, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteTool(&self, id: c_int) -> bool {
        unsafe { wxToolBar_DeleteTool(self.handle(), id) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn deleteToolByPos(&self, pos: c_int) -> bool {
        unsafe { wxToolBar_DeleteToolByPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableTool(&self, id: c_int, enable: bool) {
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
    fn getToolEnabled(&self, id: c_int) -> bool {
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
    fn getToolState(&self, id: c_int) -> bool {
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
    fn insertTool<T: _wxBitmap, U: _wxBitmap, V: _wxObject>(&self, pos: c_int, id: c_int, bmp1: &T, bmp2: &U, isToggle: bool, data: &V, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_InsertTool(self.handle(), pos, id, bmp1.handle(), bmp2.handle(), isToggle, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn realize(&self) -> bool {
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
    fn toggleTool(&self, id: c_int, toggle: bool) {
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

struct wxToolBarBase(*u8);
impl _wxToolBarBase for wxToolBarBase {}
impl _wxControl for wxToolBarBase {}
impl _wxWindow for wxToolBarBase {}
impl _wxEvtHandler for wxToolBarBase {}
impl _wxObject for wxToolBarBase { fn handle(&self) -> *u8 { **self } }

impl wxToolBarBase {
    pub fn from(handle: *u8) -> @wxToolBarBase { @wxToolBarBase(handle) }
    pub fn null() -> @wxToolBarBase { wxToolBarBase::from(0 as *u8) }
    
}

trait _wxToolBarBase : _wxControl {
}

struct wxToolLayoutItem(*u8);
impl _wxToolLayoutItem for wxToolLayoutItem {}
impl _wxObject for wxToolLayoutItem { fn handle(&self) -> *u8 { **self } }

impl wxToolLayoutItem {
    pub fn from(handle: *u8) -> @wxToolLayoutItem { @wxToolLayoutItem(handle) }
    pub fn null() -> @wxToolLayoutItem { wxToolLayoutItem::from(0 as *u8) }
    
}

trait _wxToolLayoutItem : _wxObject {
}

struct wxToolTip(*u8);
impl _wxToolTip for wxToolTip {}
impl _wxObject for wxToolTip { fn handle(&self) -> *u8 { **self } }

impl wxToolTip {
    pub fn from(handle: *u8) -> @wxToolTip { @wxToolTip(handle) }
    pub fn null() -> @wxToolTip { wxToolTip::from(0 as *u8) }
    
}

trait _wxToolTip : _wxObject {
}

struct wxToolWindow(*u8);
impl _wxToolWindow for wxToolWindow {}
impl _wxFrame for wxToolWindow {}
impl _wxTopLevelWindow for wxToolWindow {}
impl _wxWindow for wxToolWindow {}
impl _wxEvtHandler for wxToolWindow {}
impl _wxObject for wxToolWindow { fn handle(&self) -> *u8 { **self } }

impl wxToolWindow {
    pub fn from(handle: *u8) -> @wxToolWindow { @wxToolWindow(handle) }
    pub fn null() -> @wxToolWindow { wxToolWindow::from(0 as *u8) }
    
}

trait _wxToolWindow : _wxFrame {
}

struct wxTopLevelWindow(*u8);
impl _wxTopLevelWindow for wxTopLevelWindow {}
impl _wxWindow for wxTopLevelWindow {}
impl _wxEvtHandler for wxTopLevelWindow {}
impl _wxObject for wxTopLevelWindow { fn handle(&self) -> *u8 { **self } }

impl wxTopLevelWindow {
    pub fn from(handle: *u8) -> @wxTopLevelWindow { @wxTopLevelWindow(handle) }
    pub fn null() -> @wxTopLevelWindow { wxTopLevelWindow::from(0 as *u8) }
    
}

trait _wxTopLevelWindow : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn enableCloseButton(&self, enable: bool) -> bool {
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
    fn iconize(&self, iconize: bool) -> bool {
        unsafe { wxTopLevelWindow_Iconize(self.handle(), iconize) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isActive(&self) -> bool {
        unsafe { wxTopLevelWindow_IsActive(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIconized(&self) -> bool {
        unsafe { wxTopLevelWindow_IsIconized(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isMaximized(&self) -> bool {
        unsafe { wxTopLevelWindow_IsMaximized(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn maximize(&self, maximize: bool) {
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
    fn setIcons(&self, _icons: *u8) {
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

struct wxTreeCompanionWindow(*u8);
impl _wxTreeCompanionWindow for wxTreeCompanionWindow {}
impl _wxWindow for wxTreeCompanionWindow {}
impl _wxEvtHandler for wxTreeCompanionWindow {}
impl _wxObject for wxTreeCompanionWindow { fn handle(&self) -> *u8 { **self } }

impl wxTreeCompanionWindow {
    pub fn from(handle: *u8) -> @wxTreeCompanionWindow { @wxTreeCompanionWindow(handle) }
    pub fn null() -> @wxTreeCompanionWindow { wxTreeCompanionWindow::from(0 as *u8) }
    
}

trait _wxTreeCompanionWindow : _wxWindow {
}

struct wxTreeCtrl(*u8);
impl _wxTreeCtrl for wxTreeCtrl {}
impl _wxControl for wxTreeCtrl {}
impl _wxWindow for wxTreeCtrl {}
impl _wxEvtHandler for wxTreeCtrl {}
impl _wxObject for wxTreeCtrl { fn handle(&self) -> *u8 { **self } }

impl wxTreeCtrl {
    pub fn from(handle: *u8) -> @wxTreeCtrl { @wxTreeCtrl(handle) }
    pub fn null() -> @wxTreeCtrl { wxTreeCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_obj: *u8, _cmp: *u8, _prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create(_obj, _cmp, _prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new2<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create2(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxTreeCtrl : _wxControl {
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
    fn endEditLabel<T: _wxTreeItemId>(&self, item: &T, discardChanges: bool) {
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
    fn getBoundingRect<T: _wxTreeItemId>(&self, item: &T, textOnly: bool) -> @wxRect {
        unsafe { @wxRect(wxTreeCtrl_GetBoundingRect(self.handle(), item.handle(), textOnly)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChildrenCount<T: _wxTreeItemId>(&self, item: &T, recursively: bool) -> c_int {
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
    fn getFirstChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, cookie: *c_int, _item: &U) {
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
    fn getItemData<T: _wxTreeItemId>(&self, item: &T) -> *u8 {
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
    fn getNextChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: &T, cookie: *c_int, _item: &U) {
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
    fn getSelections(&self, selections: *intptr_t) -> c_int {
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
    fn hitTest<T: _wxTreeItemId>(&self, _x: c_int, _y: c_int, flags: *c_int, _item: &T) {
        unsafe { wxTreeCtrl_HitTest(self.handle(), _x, _y, flags, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItem<T: _wxTreeItemId, U: _wxTreeItemId, V: _wxTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, data: *u8, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertItemByIndex<T: _wxTreeItemId, U: _wxTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, data: *u8, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBold<T: _wxTreeItemId>(&self, item: &T) -> bool {
        unsafe { wxTreeCtrl_IsBold(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExpanded<T: _wxTreeItemId>(&self, item: &T) -> bool {
        unsafe { wxTreeCtrl_IsExpanded(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isSelected<T: _wxTreeItemId>(&self, item: &T) -> bool {
        unsafe { wxTreeCtrl_IsSelected(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isVisible<T: _wxTreeItemId>(&self, item: &T) -> bool {
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
    fn prependItem<T: _wxTreeItemId, U: _wxTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: *u8, _item: &U) {
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
    fn setItemBold<T: _wxTreeItemId>(&self, item: &T, bold: bool) {
        unsafe { wxTreeCtrl_SetItemBold(self.handle(), item.handle(), bold) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemData<T: _wxTreeItemId>(&self, item: &T, data: *u8) {
        unsafe { wxTreeCtrl_SetItemData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemDropHighlight<T: _wxTreeItemId>(&self, item: &T, highlight: bool) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.handle(), item.handle(), highlight) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemFont<T: _wxTreeItemId, U: _wxFont>(&self, item: &T, font: &U) {
        unsafe { wxTreeCtrl_SetItemFont(self.handle(), item.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setItemHasChildren<T: _wxTreeItemId>(&self, item: &T, hasChildren: bool) {
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

struct wxTreeEvent(*u8);
impl _wxTreeEvent for wxTreeEvent {}
impl _wxNotifyEvent for wxTreeEvent {}
impl _wxCommandEvent for wxTreeEvent {}
impl _wxEvent for wxTreeEvent {}
impl _wxObject for wxTreeEvent { fn handle(&self) -> *u8 { **self } }

impl wxTreeEvent {
    pub fn from(handle: *u8) -> @wxTreeEvent { @wxTreeEvent(handle) }
    pub fn null() -> @wxTreeEvent { wxTreeEvent::from(0 as *u8) }
    
}

trait _wxTreeEvent : _wxNotifyEvent {
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

struct wxTreeItemData(*u8);
impl _wxTreeItemData for wxTreeItemData {}
impl _wxClientData for wxTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemData {
    pub fn from(handle: *u8) -> @wxTreeItemData { @wxTreeItemData(handle) }
    pub fn null() -> @wxTreeItemData { wxTreeItemData::from(0 as *u8) }
    
}

trait _wxTreeItemData : _wxClientData {
}

struct wxTreeItemId(*u8);
impl _wxTreeItemId for wxTreeItemId { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemId {
    pub fn from(handle: *u8) -> @wxTreeItemId { @wxTreeItemId(handle) }
    pub fn null() -> @wxTreeItemId { wxTreeItemId::from(0 as *u8) }
    
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

trait _wxTreeItemId {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
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

struct wxTreeLayout(*u8);
impl _wxTreeLayout for wxTreeLayout {}
impl _wxObject for wxTreeLayout { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayout {
    pub fn from(handle: *u8) -> @wxTreeLayout { @wxTreeLayout(handle) }
    pub fn null() -> @wxTreeLayout { wxTreeLayout::from(0 as *u8) }
    
}

trait _wxTreeLayout : _wxObject {
}

struct wxTreeLayoutStored(*u8);
impl _wxTreeLayoutStored for wxTreeLayoutStored {}
impl _wxTreeLayout for wxTreeLayoutStored {}
impl _wxObject for wxTreeLayoutStored { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayoutStored {
    pub fn from(handle: *u8) -> @wxTreeLayoutStored { @wxTreeLayoutStored(handle) }
    pub fn null() -> @wxTreeLayoutStored { wxTreeLayoutStored::from(0 as *u8) }
    
}

trait _wxTreeLayoutStored : _wxTreeLayout {
}

struct wxURL(*u8);
impl _wxURL for wxURL {}
impl _wxObject for wxURL { fn handle(&self) -> *u8 { **self } }

impl wxURL {
    pub fn from(handle: *u8) -> @wxURL { @wxURL(handle) }
    pub fn null() -> @wxURL { wxURL::from(0 as *u8) }
    
}

trait _wxURL : _wxObject {
}

struct wxUpdateUIEvent(*u8);
impl _wxUpdateUIEvent for wxUpdateUIEvent {}
impl _wxEvent for wxUpdateUIEvent {}
impl _wxObject for wxUpdateUIEvent { fn handle(&self) -> *u8 { **self } }

impl wxUpdateUIEvent {
    pub fn from(handle: *u8) -> @wxUpdateUIEvent { @wxUpdateUIEvent(handle) }
    pub fn null() -> @wxUpdateUIEvent { wxUpdateUIEvent::from(0 as *u8) }
    
}

trait _wxUpdateUIEvent : _wxEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn check(&self, check: bool) {
        unsafe { wxUpdateUIEvent_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self, enable: bool) {
        unsafe { wxUpdateUIEvent_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChecked(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getEnabled(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetChecked(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetEnabled(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSetText(&self) -> bool {
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

struct wxValidator(*u8);
impl _wxValidator for wxValidator {}
impl _wxEvtHandler for wxValidator {}
impl _wxObject for wxValidator { fn handle(&self) -> *u8 { **self } }

impl wxValidator {
    pub fn from(handle: *u8) -> @wxValidator { @wxValidator(handle) }
    pub fn null() -> @wxValidator { wxValidator::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxValidator {
        unsafe { @wxValidator(wxValidator_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn setBellOnError(doIt: bool) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

trait _wxValidator : _wxEvtHandler {
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
    fn transferFromWindow(&self) -> bool {
        unsafe { wxValidator_TransferFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferToWindow(&self) -> bool {
        unsafe { wxValidator_TransferToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn validate<T: _wxWindow>(&self, parent: &T) -> bool {
        unsafe { wxValidator_Validate(self.handle(), parent.handle()) }
    }
}

struct wxVariant(*u8);
impl _wxVariant for wxVariant {}
impl _wxObject for wxVariant { fn handle(&self) -> *u8 { **self } }

impl wxVariant {
    pub fn from(handle: *u8) -> @wxVariant { @wxVariant(handle) }
    pub fn null() -> @wxVariant { wxVariant::from(0 as *u8) }
    
}

trait _wxVariant : _wxObject {
}

struct wxVariantData(*u8);
impl _wxVariantData for wxVariantData {}
impl _wxObject for wxVariantData { fn handle(&self) -> *u8 { **self } }

impl wxVariantData {
    pub fn from(handle: *u8) -> @wxVariantData { @wxVariantData(handle) }
    pub fn null() -> @wxVariantData { wxVariantData::from(0 as *u8) }
    
}

trait _wxVariantData : _wxObject {
}

struct wxView(*u8);
impl _wxView for wxView {}
impl _wxEvtHandler for wxView {}
impl _wxObject for wxView { fn handle(&self) -> *u8 { **self } }

impl wxView {
    pub fn from(handle: *u8) -> @wxView { @wxView(handle) }
    pub fn null() -> @wxView { wxView::from(0 as *u8) }
    
}

trait _wxView : _wxEvtHandler {
}

struct wxSound(*u8);
impl _wxSound for wxSound {}
impl _wxEvtHandler for wxSound {}
impl _wxObject for wxSound { fn handle(&self) -> *u8 { **self } }

impl wxSound {
    pub fn from(handle: *u8) -> @wxSound { @wxSound(handle) }
    pub fn null() -> @wxSound { wxSound::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(fileName: &str, isResource: bool) -> @wxSound {
        let fileName = wxT(fileName);
        unsafe { @wxSound(wxSound_Create(fileName.handle(), isResource)) }
    }
}

trait _wxSound : _wxEvtHandler {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isOk(&self) -> bool {
        unsafe { wxSound_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play(&self, flag: c_int) -> bool {
        unsafe { wxSound_Play(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) {
        unsafe { wxSound_Stop(self.handle()) }
    }
}

struct wxWindow(*u8);
impl _wxWindow for wxWindow {}
impl _wxEvtHandler for wxWindow {}
impl _wxObject for wxWindow { fn handle(&self) -> *u8 { **self } }

impl wxWindow {
    pub fn from(handle: *u8) -> @wxWindow { @wxWindow(handle) }
    pub fn null() -> @wxWindow { wxWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxWindow {
        unsafe { @wxWindow(wxWindow_Create(_prt.handle(), _id, _x, _y, _w, _h, _stl)) }
    }
}

trait _wxWindow : _wxEvtHandler {
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
    fn close(&self, _force: bool) -> bool {
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
    fn destroy(&self) -> bool {
        unsafe { wxWindow_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn destroyChildren(&self) -> bool {
        unsafe { wxWindow_DestroyChildren(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disable(&self) -> bool {
        unsafe { wxWindow_Disable(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn doPhase(&self, phase: c_int) -> c_int {
        unsafe { wxWindow_DoPhase(self.handle(), phase) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn enable(&self) -> bool {
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
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
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
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getConstraints(&self) -> @wxLayoutConstraints {
        unsafe { @wxLayoutConstraints(wxWindow_GetConstraints(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getConstraintsInvolvedIn(&self) -> *u8 {
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
    fn getHandle(&self) -> *u8 {
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
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int) {
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
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self) -> @wxSizer {
        unsafe { @wxSizer(wxWindow_GetSizer(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent<T: _wxFont>(&self, string: &str, x: *c_int, y: *c_int, descent: *c_int, externalLeading: *c_int, theFont: &T) {
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
    fn hasFlag(&self, flag: c_int) -> bool {
        unsafe { wxWindow_HasFlag(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hide(&self) -> bool {
        unsafe { wxWindow_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initDialog(&self) {
        unsafe { wxWindow_InitDialog(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isBeingDeleted(&self) -> bool {
        unsafe { wxWindow_IsBeingDeleted(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEnabled(&self) -> bool {
        unsafe { wxWindow_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { wxWindow_IsExposed(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isShown(&self) -> bool {
        unsafe { wxWindow_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isTopLevel(&self) -> bool {
        unsafe { wxWindow_IsTopLevel(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layout(&self) -> c_int {
        unsafe { wxWindow_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutPhase1(&self, noChanges: *c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase1(self.handle(), noChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn layoutPhase2(&self, noChanges: *c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase2(self.handle(), noChanges) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn lower(&self) {
        unsafe { wxWindow_Lower(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn makeModal(&self, modal: bool) {
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
    fn popEventHandler(&self, deleteHandler: bool) -> *u8 {
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
    fn refresh(&self, eraseBackground: bool) {
        unsafe { wxWindow_Refresh(self.handle(), eraseBackground) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn refreshRect(&self, eraseBackground: bool, x: c_int, y: c_int, w: c_int, h: c_int) {
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
    fn setAutoLayout(&self, autoLayout: bool) {
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
    fn setScrollPos(&self, orient: c_int, pos: c_int, refresh: bool) {
        unsafe { wxWindow_SetScrollPos(self.handle(), orient, pos, refresh) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setScrollbar(&self, orient: c_int, pos: c_int, thumbVisible: c_int, range: c_int, refresh: bool) {
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
    fn show(&self) -> bool {
        unsafe { wxWindow_Show(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn thaw(&self) {
        unsafe { wxWindow_Thaw(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferDataFromWindow(&self) -> bool {
        unsafe { wxWindow_TransferDataFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transferDataToWindow(&self) -> bool {
        unsafe { wxWindow_TransferDataToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unsetConstraints(&self, c: *u8) {
        unsafe { wxWindow_UnsetConstraints(self.handle(), c) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateWindowUI(&self) {
        unsafe { wxWindow_UpdateWindowUI(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn validate(&self) -> bool {
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

struct wxWindowCreateEvent(*u8);
impl _wxWindowCreateEvent for wxWindowCreateEvent {}
impl _wxCommandEvent for wxWindowCreateEvent {}
impl _wxEvent for wxWindowCreateEvent {}
impl _wxObject for wxWindowCreateEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowCreateEvent {
    pub fn from(handle: *u8) -> @wxWindowCreateEvent { @wxWindowCreateEvent(handle) }
    pub fn null() -> @wxWindowCreateEvent { wxWindowCreateEvent::from(0 as *u8) }
    
}

trait _wxWindowCreateEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxWindowCreateEvent_GetWindow(self.handle())) }
    }
}

struct wxWindowDC(*u8);
impl _wxWindowDC for wxWindowDC {}
impl _wxDC for wxWindowDC {}
impl _wxObject for wxWindowDC { fn handle(&self) -> *u8 { **self } }

impl wxWindowDC {
    pub fn from(handle: *u8) -> @wxWindowDC { @wxWindowDC(handle) }
    pub fn null() -> @wxWindowDC { wxWindowDC::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(win: &T) -> @wxWindowDC {
        unsafe { @wxWindowDC(wxWindowDC_Create(win.handle())) }
    }
}

trait _wxWindowDC : _wxDC {
}

struct wxWindowDestroyEvent(*u8);
impl _wxWindowDestroyEvent for wxWindowDestroyEvent {}
impl _wxCommandEvent for wxWindowDestroyEvent {}
impl _wxEvent for wxWindowDestroyEvent {}
impl _wxObject for wxWindowDestroyEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowDestroyEvent {
    pub fn from(handle: *u8) -> @wxWindowDestroyEvent { @wxWindowDestroyEvent(handle) }
    pub fn null() -> @wxWindowDestroyEvent { wxWindowDestroyEvent::from(0 as *u8) }
    
}

trait _wxWindowDestroyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindow(wxWindowDestroyEvent_GetWindow(self.handle())) }
    }
}

struct wxWindowDisabler(*u8);
impl _wxWindowDisabler for wxWindowDisabler { fn handle(&self) -> *u8 { **self } }

impl wxWindowDisabler {
    pub fn from(handle: *u8) -> @wxWindowDisabler { @wxWindowDisabler(handle) }
    pub fn null() -> @wxWindowDisabler { wxWindowDisabler::from(0 as *u8) }
    
}

trait _wxWindowDisabler {
    fn handle(&self) -> *u8;
    
}

struct wxWizard(*u8);
impl _wxWizard for wxWizard {}
impl _wxDialog for wxWizard {}
impl _wxTopLevelWindow for wxWizard {}
impl _wxWindow for wxWizard {}
impl _wxEvtHandler for wxWizard {}
impl _wxObject for wxWizard { fn handle(&self) -> *u8 { **self } }

impl wxWizard {
    pub fn from(handle: *u8) -> @wxWizard { @wxWizard(handle) }
    pub fn null() -> @wxWizard { wxWizard::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn chain<T: _wxWizardPageSimple, U: _wxWizardPageSimple>(f: &T, s: &U) {
        unsafe { wxWizard_Chain(f.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, _txt: &str, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> @wxWizard {
        let _txt = wxT(_txt);
        unsafe { @wxWizard(wxWizard_Create(_prt.handle(), _id, _txt.handle(), _bmp.handle(), _lft, _top, _wdt, _hgt)) }
    }
}

trait _wxWizard : _wxDialog {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPage(&self) -> @wxWizardPage {
        unsafe { @wxWizardPage(wxWizard_GetCurrentPage(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPageSize(&self) -> @wxSize {
        unsafe { @wxSize(wxWizard_GetPageSize(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn runWizard<T: _wxWizardPage>(&self, firstPage: &T) -> c_int {
        unsafe { wxWizard_RunWizard(self.handle(), firstPage.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.handle(), w, h) }
    }
}

struct wxWizardEvent(*u8);
impl _wxWizardEvent for wxWizardEvent {}
impl _wxNotifyEvent for wxWizardEvent {}
impl _wxCommandEvent for wxWizardEvent {}
impl _wxEvent for wxWizardEvent {}
impl _wxObject for wxWizardEvent { fn handle(&self) -> *u8 { **self } }

impl wxWizardEvent {
    pub fn from(handle: *u8) -> @wxWizardEvent { @wxWizardEvent(handle) }
    pub fn null() -> @wxWizardEvent { wxWizardEvent::from(0 as *u8) }
    
}

trait _wxWizardEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.handle()) }
    }
}

struct wxWizardPage(*u8);
impl _wxWizardPage for wxWizardPage {}
impl _wxPanel for wxWizardPage {}
impl _wxWindow for wxWizardPage {}
impl _wxEvtHandler for wxWizardPage {}
impl _wxObject for wxWizardPage { fn handle(&self) -> *u8 { **self } }

impl wxWizardPage {
    pub fn from(handle: *u8) -> @wxWizardPage { @wxWizardPage(handle) }
    pub fn null() -> @wxWizardPage { wxWizardPage::from(0 as *u8) }
    
}

trait _wxWizardPage : _wxPanel {
}

struct wxWizardPageSimple(*u8);
impl _wxWizardPageSimple for wxWizardPageSimple {}
impl _wxWizardPage for wxWizardPageSimple {}
impl _wxPanel for wxWizardPageSimple {}
impl _wxWindow for wxWizardPageSimple {}
impl _wxEvtHandler for wxWizardPageSimple {}
impl _wxObject for wxWizardPageSimple { fn handle(&self) -> *u8 { **self } }

impl wxWizardPageSimple {
    pub fn from(handle: *u8) -> @wxWizardPageSimple { @wxWizardPageSimple(handle) }
    pub fn null() -> @wxWizardPageSimple { wxWizardPageSimple::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWizard>(_prt: &T) -> @wxWizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_Create(_prt.handle())) }
    }
}

trait _wxWizardPageSimple : _wxWizardPage {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmap<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxWizardPageSimple_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNext(&self) -> @wxWizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_GetNext(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPrev(&self) -> @wxWizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_GetPrev(self.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setNext<T: _wxWizardPageSimple>(&self, next: &T) {
        unsafe { wxWizardPageSimple_SetNext(self.handle(), next.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPrev<T: _wxWizardPageSimple>(&self, prev: &T) {
        unsafe { wxWizardPageSimple_SetPrev(self.handle(), prev.handle()) }
    }
}

struct wxXmlResource(*u8);
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn handle(&self) -> *u8 { **self } }

impl wxXmlResource {
    pub fn from(handle: *u8) -> @wxXmlResource { @wxXmlResource(handle) }
    pub fn null() -> @wxXmlResource { wxXmlResource::from(0 as *u8) }
    
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

trait _wxXmlResource : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSubclassFactory(&self, factory: *u8) {
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
    fn load(&self, filemask: &str) -> bool {
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
    fn unload(&self, filemask: &str) -> bool {
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

struct wxXmlResourceHandler(*u8);
impl _wxXmlResourceHandler for wxXmlResourceHandler {}
impl _wxObject for wxXmlResourceHandler { fn handle(&self) -> *u8 { **self } }

impl wxXmlResourceHandler {
    pub fn from(handle: *u8) -> @wxXmlResourceHandler { @wxXmlResourceHandler(handle) }
    pub fn null() -> @wxXmlResourceHandler { wxXmlResourceHandler::from(0 as *u8) }
    
}

trait _wxXmlResourceHandler : _wxObject {
}

struct wxZipInputStream(*u8);
impl _wxZipInputStream for wxZipInputStream {}
impl _wxInputStream for wxZipInputStream {}
impl _wxStreamBase for wxZipInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZipInputStream {
    pub fn from(handle: *u8) -> @wxZipInputStream { @wxZipInputStream(handle) }
    pub fn null() -> @wxZipInputStream { wxZipInputStream::from(0 as *u8) }
    
}

trait _wxZipInputStream : _wxInputStream {
}

struct wxZlibInputStream(*u8);
impl _wxZlibInputStream for wxZlibInputStream {}
impl _wxFilterInputStream for wxZlibInputStream {}
impl _wxInputStream for wxZlibInputStream {}
impl _wxStreamBase for wxZlibInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibInputStream {
    pub fn from(handle: *u8) -> @wxZlibInputStream { @wxZlibInputStream(handle) }
    pub fn null() -> @wxZlibInputStream { wxZlibInputStream::from(0 as *u8) }
    
}

trait _wxZlibInputStream : _wxFilterInputStream {
}

struct wxZlibOutputStream(*u8);
impl _wxZlibOutputStream for wxZlibOutputStream {}
impl _wxFilterOutputStream for wxZlibOutputStream {}
impl _wxOutputStream for wxZlibOutputStream {}
impl _wxStreamBase for wxZlibOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibOutputStream {
    pub fn from(handle: *u8) -> @wxZlibOutputStream { @wxZlibOutputStream(handle) }
    pub fn null() -> @wxZlibOutputStream { wxZlibOutputStream::from(0 as *u8) }
    
}

trait _wxZlibOutputStream : _wxFilterOutputStream {
}

struct wxPropertyGrid(*u8);
impl _wxPropertyGrid for wxPropertyGrid {}
impl _wxControl for wxPropertyGrid {}
impl _wxWindow for wxPropertyGrid {}
impl _wxEvtHandler for wxPropertyGrid {}
impl _wxObject for wxPropertyGrid { fn handle(&self) -> *u8 { **self } }

impl wxPropertyGrid {
    pub fn from(handle: *u8) -> @wxPropertyGrid { @wxPropertyGrid(handle) }
    pub fn null() -> @wxPropertyGrid { wxPropertyGrid::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxPropertyGrid {
        unsafe { @wxPropertyGrid(wxPropertyGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxPropertyGrid : _wxControl {
    #[fixed_stack_segment]
    #[inline(never)]
    fn append<T: _wxPGProperty>(&self, prop: &T) -> @wxPGProperty {
        unsafe { @wxPGProperty(wxPropertyGrid_Append(self.handle(), prop.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn disableProperty(&self, propName: &str) -> bool {
        let propName = wxT(propName);
        unsafe { wxPropertyGrid_DisableProperty(self.handle(), propName.handle()) }
    }
}

struct wxPropertyGridEvent(*u8);
impl _wxPropertyGridEvent for wxPropertyGridEvent {}
impl _wxNotifyEvent for wxPropertyGridEvent {}
impl _wxCommandEvent for wxPropertyGridEvent {}
impl _wxEvent for wxPropertyGridEvent {}
impl _wxObject for wxPropertyGridEvent { fn handle(&self) -> *u8 { **self } }

impl wxPropertyGridEvent {
    pub fn from(handle: *u8) -> @wxPropertyGridEvent { @wxPropertyGridEvent(handle) }
    pub fn null() -> @wxPropertyGridEvent { wxPropertyGridEvent::from(0 as *u8) }
    
}

trait _wxPropertyGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn hasProperty(&self) -> bool {
        unsafe { wxPropertyGridEvent_HasProperty(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getProperty(&self) -> @wxPGProperty {
        unsafe { @wxPGProperty(wxPropertyGridEvent_GetProperty(self.handle())) }
    }
}

struct wxPGProperty(*u8);
impl _wxPGProperty for wxPGProperty {}
impl _wxObject for wxPGProperty { fn handle(&self) -> *u8 { **self } }

impl wxPGProperty {
    pub fn from(handle: *u8) -> @wxPGProperty { @wxPGProperty(handle) }
    pub fn null() -> @wxPGProperty { wxPGProperty::from(0 as *u8) }
    
}

trait _wxPGProperty : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getLabel(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetLabel(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getName(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetName(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValueAsString(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetValueAsString(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getValueType(&self) -> ~str {
        unsafe { wxString { handle: wxPGProperty_GetValueType(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHelpString(&self, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxPGProperty_SetHelpString(self.handle(), helpString.handle()) }
    }
}

struct wxStringProperty(*u8);
impl _wxStringProperty for wxStringProperty {}
impl _wxPGProperty for wxStringProperty {}
impl _wxObject for wxStringProperty { fn handle(&self) -> *u8 { **self } }

impl wxStringProperty {
    pub fn from(handle: *u8) -> @wxStringProperty { @wxStringProperty(handle) }
    pub fn null() -> @wxStringProperty { wxStringProperty::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: &str) -> @wxStringProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { @wxStringProperty(wxStringProperty_Create(label.handle(), name.handle(), value.handle())) }
    }
}

trait _wxStringProperty : _wxPGProperty {
}

struct wxIntProperty(*u8);
impl _wxIntProperty for wxIntProperty {}
impl _wxPGProperty for wxIntProperty {}
impl _wxObject for wxIntProperty { fn handle(&self) -> *u8 { **self } }

impl wxIntProperty {
    pub fn from(handle: *u8) -> @wxIntProperty { @wxIntProperty(handle) }
    pub fn null() -> @wxIntProperty { wxIntProperty::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_int) -> @wxIntProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxIntProperty(wxIntProperty_Create(label.handle(), name.handle(), value)) }
    }
}

trait _wxIntProperty : _wxPGProperty {
}

struct wxBoolProperty(*u8);
impl _wxBoolProperty for wxBoolProperty {}
impl _wxPGProperty for wxBoolProperty {}
impl _wxObject for wxBoolProperty { fn handle(&self) -> *u8 { **self } }

impl wxBoolProperty {
    pub fn from(handle: *u8) -> @wxBoolProperty { @wxBoolProperty(handle) }
    pub fn null() -> @wxBoolProperty { wxBoolProperty::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: bool) -> @wxBoolProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxBoolProperty(wxBoolProperty_Create(label.handle(), name.handle(), value)) }
    }
}

trait _wxBoolProperty : _wxPGProperty {
}

struct wxFloatProperty(*u8);
impl _wxFloatProperty for wxFloatProperty {}
impl _wxPGProperty for wxFloatProperty {}
impl _wxObject for wxFloatProperty { fn handle(&self) -> *u8 { **self } }

impl wxFloatProperty {
    pub fn from(handle: *u8) -> @wxFloatProperty { @wxFloatProperty(handle) }
    pub fn null() -> @wxFloatProperty { wxFloatProperty::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: c_float) -> @wxFloatProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxFloatProperty(wxFloatProperty_Create(label.handle(), name.handle(), value)) }
    }
}

trait _wxFloatProperty : _wxPGProperty {
}

struct wxDateProperty(*u8);
impl _wxDateProperty for wxDateProperty {}
impl _wxPGProperty for wxDateProperty {}
impl _wxObject for wxDateProperty { fn handle(&self) -> *u8 { **self } }

impl wxDateProperty {
    pub fn from(handle: *u8) -> @wxDateProperty { @wxDateProperty(handle) }
    pub fn null() -> @wxDateProperty { wxDateProperty::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxDateTime>(label: &str, name: &str, value: &T) -> @wxDateProperty {
        let label = wxT(label);
        let name = wxT(name);
        unsafe { @wxDateProperty(wxDateProperty_Create(label.handle(), name.handle(), value.handle())) }
    }
}

trait _wxDateProperty : _wxPGProperty {
}

struct wxFileProperty(*u8);
impl _wxFileProperty for wxFileProperty {}
impl _wxPGProperty for wxFileProperty {}
impl _wxObject for wxFileProperty { fn handle(&self) -> *u8 { **self } }

impl wxFileProperty {
    pub fn from(handle: *u8) -> @wxFileProperty { @wxFileProperty(handle) }
    pub fn null() -> @wxFileProperty { wxFileProperty::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str, name: &str, value: &str) -> @wxFileProperty {
        let label = wxT(label);
        let name = wxT(name);
        let value = wxT(value);
        unsafe { @wxFileProperty(wxFileProperty_Create(label.handle(), name.handle(), value.handle())) }
    }
}

trait _wxFileProperty : _wxPGProperty {
}

struct wxPropertyCategory(*u8);
impl _wxPropertyCategory for wxPropertyCategory {}
impl _wxPGProperty for wxPropertyCategory {}
impl _wxObject for wxPropertyCategory { fn handle(&self) -> *u8 { **self } }

impl wxPropertyCategory {
    pub fn from(handle: *u8) -> @wxPropertyCategory { @wxPropertyCategory(handle) }
    pub fn null() -> @wxPropertyCategory { wxPropertyCategory::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(label: &str) -> @wxPropertyCategory {
        let label = wxT(label);
        unsafe { @wxPropertyCategory(wxPropertyCategory_Create(label.handle())) }
    }
}

trait _wxPropertyCategory : _wxPGProperty {
}

struct wxGenericDragImage(*u8);
impl _wxGenericDragImage for wxGenericDragImage {}
impl _wxDragImage for wxGenericDragImage {}
impl _wxObject for wxGenericDragImage { fn handle(&self) -> *u8 { **self } }

impl wxGenericDragImage {
    pub fn from(handle: *u8) -> @wxGenericDragImage { @wxGenericDragImage(handle) }
    pub fn null() -> @wxGenericDragImage { wxGenericDragImage::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxCursor>(cursor: &T) -> @wxGenericDragImage {
        unsafe { @wxGenericDragImage(wxGenericDragImage_Create(cursor.handle())) }
    }
}

trait _wxGenericDragImage : _wxDragImage {
    #[fixed_stack_segment]
    #[inline(never)]
    fn doDrawImage<T: _wxDC>(&self, dc: &T, x: c_int, y: c_int) -> bool {
        unsafe { wxGenericDragImage_DoDrawImage(self.handle(), dc.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> @wxRect {
        unsafe { @wxRect(wxGenericDragImage_GetImageRect(self.handle(), x_pos, y_pos)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn updateBackingFromWindow<T: _wxDC, U: _wxMemoryDC>(&self, windowDC: &T, destDC: &U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.handle(), windowDC.handle(), destDC.handle(), x, y, w, h, xdest, ydest, width, height) }
    }
}

struct wxGraphicsObject(*u8);
impl _wxGraphicsObject for wxGraphicsObject {}
impl _wxObject for wxGraphicsObject { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsObject {
    pub fn from(handle: *u8) -> @wxGraphicsObject { @wxGraphicsObject(handle) }
    pub fn null() -> @wxGraphicsObject { wxGraphicsObject::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getRenderer() -> @wxGraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsObject_GetRenderer()) }
    }
}

trait _wxGraphicsObject : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn isNull(&self) -> bool {
        unsafe { wxGraphicsObject_IsNull(self.handle()) }
    }
}

struct wxGraphicsBrush(*u8);
impl _wxGraphicsBrush for wxGraphicsBrush {}
impl _wxGraphicsObject for wxGraphicsBrush {}
impl _wxObject for wxGraphicsBrush { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsBrush {
    pub fn from(handle: *u8) -> @wxGraphicsBrush { @wxGraphicsBrush(handle) }
    pub fn null() -> @wxGraphicsBrush { wxGraphicsBrush::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsBrush {
        unsafe { @wxGraphicsBrush(wxGraphicsBrush_Create()) }
    }
}

trait _wxGraphicsBrush : _wxGraphicsObject {
}

struct wxGraphicsContext(*u8);
impl _wxGraphicsContext for wxGraphicsContext {}
impl _wxGraphicsObject for wxGraphicsContext {}
impl _wxObject for wxGraphicsContext { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsContext {
    pub fn from(handle: *u8) -> @wxGraphicsContext { @wxGraphicsContext(handle) }
    pub fn null() -> @wxGraphicsContext { wxGraphicsContext::from(0 as *u8) }
    
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
    pub fn newFromNative(context: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNative(context)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromNativeWindow(window: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNativeWindow(window)) }
    }
}

trait _wxGraphicsContext : _wxGraphicsObject {
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
    fn drawLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
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
    fn getNativeContext(&self) -> *u8 {
        unsafe { wxGraphicsContext_GetNativeContext(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextExtent(&self, text: &str, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double) {
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
    fn strokeLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.handle(), n, x, y, style) }
    }
}

struct wxGraphicsFont(*u8);
impl _wxGraphicsFont for wxGraphicsFont {}
impl _wxGraphicsObject for wxGraphicsFont {}
impl _wxObject for wxGraphicsFont { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsFont {
    pub fn from(handle: *u8) -> @wxGraphicsFont { @wxGraphicsFont(handle) }
    pub fn null() -> @wxGraphicsFont { wxGraphicsFont::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsFont {
        unsafe { @wxGraphicsFont(wxGraphicsFont_Create()) }
    }
}

trait _wxGraphicsFont : _wxGraphicsObject {
}

struct wxGraphicsMatrix(*u8);
impl _wxGraphicsMatrix for wxGraphicsMatrix {}
impl _wxGraphicsObject for wxGraphicsMatrix {}
impl _wxObject for wxGraphicsMatrix { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsMatrix {
    pub fn from(handle: *u8) -> @wxGraphicsMatrix { @wxGraphicsMatrix(handle) }
    pub fn null() -> @wxGraphicsMatrix { wxGraphicsMatrix::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsMatrix {
        unsafe { @wxGraphicsMatrix(wxGraphicsMatrix_Create()) }
    }
}

trait _wxGraphicsMatrix : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn concat<T: _wxGraphicsMatrix>(&self, t: &T) {
        unsafe { wxGraphicsMatrix_Concat(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn get(&self, a: *c_double, b: *c_double, c: *c_double, d: *c_double, tx: *c_double, ty: *c_double) {
        unsafe { wxGraphicsMatrix_Get(self.handle(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativeMatrix(&self) -> *u8 {
        unsafe { wxGraphicsMatrix_GetNativeMatrix(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn invert(&self) {
        unsafe { wxGraphicsMatrix_Invert(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isEqual<T: _wxGraphicsMatrix>(&self, t: &T) -> bool {
        unsafe { wxGraphicsMatrix_IsEqual(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn isIdentity(&self) -> bool {
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
    fn transformPoint(&self, x: *c_double, y: *c_double) {
        unsafe { wxGraphicsMatrix_TransformPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transformDistance(&self, dx: *c_double, dy: *c_double) {
        unsafe { wxGraphicsMatrix_TransformDistance(self.handle(), dx, dy) }
    }
}

struct wxGraphicsPath(*u8);
impl _wxGraphicsPath for wxGraphicsPath {}
impl _wxGraphicsObject for wxGraphicsPath {}
impl _wxObject for wxGraphicsPath { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsPath {
    pub fn from(handle: *u8) -> @wxGraphicsPath { @wxGraphicsPath(handle) }
    pub fn null() -> @wxGraphicsPath { wxGraphicsPath::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsPath {
        unsafe { @wxGraphicsPath(wxGraphicsPath_Create()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn unGetNativePath(p: *u8) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

trait _wxGraphicsPath : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn moveToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_MoveToPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: bool) {
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
    fn getBox(&self, x: *c_double, y: *c_double, w: *c_double, h: *c_double) {
        unsafe { wxGraphicsPath_GetBox(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCurrentPoint(&self, x: *c_double, y: *c_double) {
        unsafe { wxGraphicsPath_GetCurrentPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn transform<T: _wxGraphicsMatrix>(&self, matrix: &T) {
        unsafe { wxGraphicsPath_Transform(self.handle(), matrix.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNativePath(&self) -> *u8 {
        unsafe { wxGraphicsPath_GetNativePath(self.handle()) }
    }
}

struct wxGraphicsPen(*u8);
impl _wxGraphicsPen for wxGraphicsPen {}
impl _wxGraphicsObject for wxGraphicsPen {}
impl _wxObject for wxGraphicsPen { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsPen {
    pub fn from(handle: *u8) -> @wxGraphicsPen { @wxGraphicsPen(handle) }
    pub fn null() -> @wxGraphicsPen { wxGraphicsPen::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new() -> @wxGraphicsPen {
        unsafe { @wxGraphicsPen(wxGraphicsPen_Create()) }
    }
}

trait _wxGraphicsPen : _wxGraphicsObject {
}

struct wxGraphicsRenderer(*u8);
impl _wxGraphicsRenderer for wxGraphicsRenderer {}
impl _wxGraphicsObject for wxGraphicsRenderer {}
impl _wxObject for wxGraphicsRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsRenderer {
    pub fn from(handle: *u8) -> @wxGraphicsRenderer { @wxGraphicsRenderer(handle) }
    pub fn null() -> @wxGraphicsRenderer { wxGraphicsRenderer::from(0 as *u8) }
    
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
    pub fn newContextFromNativeContext(context: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeContext(context)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newContextFromNativeWindow(window: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) }
    }
}

trait _wxGraphicsRenderer : _wxGraphicsObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDefaultRenderer(&self) -> @wxGraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsRenderer_GetDefaultRenderer(self.handle())) }
    }
}

struct wxGLContext(*u8);
impl _wxGLContext for wxGLContext {}
impl _wxObject for wxGLContext { fn handle(&self) -> *u8 { **self } }

impl wxGLContext {
    pub fn from(handle: *u8) -> @wxGLContext { @wxGLContext(handle) }
    pub fn null() -> @wxGLContext { wxGLContext::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxGLCanvas, U: _wxGLContext>(win: &T, other: &U) -> @wxGLContext {
        unsafe { @wxGLContext(wxGLContext_Create(win.handle(), other.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromNull<T: _wxGLCanvas>(win: &T) -> @wxGLContext {
        unsafe { @wxGLContext(wxGLContext_CreateFromNull(win.handle())) }
    }
}

trait _wxGLContext : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCurrent<T: _wxGLCanvas>(&self, win: &T) -> bool {
        unsafe { wxGLContext_SetCurrent(self.handle(), win.handle()) }
    }
}

struct wxManagedPtr(*u8);
impl _wxManagedPtr for wxManagedPtr { fn handle(&self) -> *u8 { **self } }

impl wxManagedPtr {
    pub fn from(handle: *u8) -> @wxManagedPtr { @wxManagedPtr(handle) }
    pub fn null() -> @wxManagedPtr { wxManagedPtr::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn getDeleteFunction() -> *u8 {
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

trait _wxManagedPtr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPtr(&self) -> *u8 {
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

struct wxMediaCtrl(*u8);
impl _wxMediaCtrl for wxMediaCtrl {}
impl _wxWindow for wxMediaCtrl {}
impl _wxEvtHandler for wxMediaCtrl {}
impl _wxObject for wxMediaCtrl { fn handle(&self) -> *u8 { **self } }

impl wxMediaCtrl {
    pub fn from(handle: *u8) -> @wxMediaCtrl { @wxMediaCtrl(handle) }
    pub fn null() -> @wxMediaCtrl { wxMediaCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(parent: &T, windowID: c_int, fileName: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: &str, name: &str) -> @wxMediaCtrl {
        let fileName = wxT(fileName);
        let szBackend = wxT(szBackend);
        let name = wxT(name);
        unsafe { @wxMediaCtrl(wxMediaCtrl_Create(parent.handle(), windowID, fileName.handle(), x, y, w, h, style, szBackend.handle(), name.handle())) }
    }
}

trait _wxMediaCtrl : _wxWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPlaybackRate(&self) -> c_double {
        unsafe { wxMediaCtrl_GetPlaybackRate(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVolume(&self) -> c_double {
        unsafe { wxMediaCtrl_GetVolume(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getState(&self) -> c_int {
        unsafe { wxMediaCtrl_GetState(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn length(&self) -> i64 {
        unsafe { wxMediaCtrl_Length(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, fileName: &str) -> bool {
        let fileName = wxT(fileName);
        unsafe { wxMediaCtrl_Load(self.handle(), fileName.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadURI(&self, uri: &str) -> bool {
        let uri = wxT(uri);
        unsafe { wxMediaCtrl_LoadURI(self.handle(), uri.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadURIWithProxy(&self, uri: &str, proxy: &str) -> bool {
        let uri = wxT(uri);
        let proxy = wxT(proxy);
        unsafe { wxMediaCtrl_LoadURIWithProxy(self.handle(), uri.handle(), proxy.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn pause(&self) -> bool {
        unsafe { wxMediaCtrl_Pause(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn play(&self) -> bool {
        unsafe { wxMediaCtrl_Play(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn seek(&self, offsetWhere: i64, mode: c_int) -> i64 {
        unsafe { wxMediaCtrl_Seek(self.handle(), offsetWhere, mode) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPlaybackRate(&self, dRate: c_double) -> bool {
        unsafe { wxMediaCtrl_SetPlaybackRate(self.handle(), dRate) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setVolume(&self, dVolume: c_double) -> bool {
        unsafe { wxMediaCtrl_SetVolume(self.handle(), dVolume) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn showPlayerControls(&self, flags: c_int) -> bool {
        unsafe { wxMediaCtrl_ShowPlayerControls(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn stop(&self) -> bool {
        unsafe { wxMediaCtrl_Stop(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn tell(&self) -> i64 {
        unsafe { wxMediaCtrl_Tell(self.handle()) }
    }
}

struct wxMediaEvent(*u8);
impl _wxMediaEvent for wxMediaEvent {}
impl _wxNotifyEvent for wxMediaEvent {}
impl _wxCommandEvent for wxMediaEvent {}
impl _wxEvent for wxMediaEvent {}
impl _wxObject for wxMediaEvent { fn handle(&self) -> *u8 { **self } }

impl wxMediaEvent {
    pub fn from(handle: *u8) -> @wxMediaEvent { @wxMediaEvent(handle) }
    pub fn null() -> @wxMediaEvent { wxMediaEvent::from(0 as *u8) }
    
}

trait _wxMediaEvent : _wxNotifyEvent {
}

struct wxcPrintout(*u8);
impl _wxcPrintout for wxcPrintout {}
impl _wxPrintout for wxcPrintout {}
impl _wxObject for wxcPrintout { fn handle(&self) -> *u8 { **self } }

impl wxcPrintout {
    pub fn from(handle: *u8) -> @wxcPrintout { @wxcPrintout(handle) }
    pub fn null() -> @wxcPrintout { wxcPrintout::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(title: &str) -> @wxcPrintout {
        let title = wxT(title);
        unsafe { @wxcPrintout(wxcPrintout_Create(title.handle())) }
    }
}

trait _wxcPrintout : _wxPrintout {
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

struct wxcPrintEvent(*u8);
impl _wxcPrintEvent for wxcPrintEvent {}
impl _wxEvent for wxcPrintEvent {}
impl _wxObject for wxcPrintEvent { fn handle(&self) -> *u8 { **self } }

impl wxcPrintEvent {
    pub fn from(handle: *u8) -> @wxcPrintEvent { @wxcPrintEvent(handle) }
    pub fn null() -> @wxcPrintEvent { wxcPrintEvent::from(0 as *u8) }
    
}

trait _wxcPrintEvent : _wxEvent {
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
    fn getContinue(&self) -> bool {
        unsafe { wxcPrintEvent_GetContinue(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setContinue(&self, cont: bool) {
        unsafe { wxcPrintEvent_SetContinue(self.handle(), cont) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintEvent_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
}

struct wxcPrintoutHandler(*u8);
impl _wxcPrintoutHandler for wxcPrintoutHandler {}
impl _wxEvtHandler for wxcPrintoutHandler {}
impl _wxObject for wxcPrintoutHandler { fn handle(&self) -> *u8 { **self } }

impl wxcPrintoutHandler {
    pub fn from(handle: *u8) -> @wxcPrintoutHandler { @wxcPrintoutHandler(handle) }
    pub fn null() -> @wxcPrintoutHandler { wxcPrintoutHandler::from(0 as *u8) }
    
}

trait _wxcPrintoutHandler : _wxEvtHandler {
}

struct wxStyledTextCtrl(*u8);
impl _wxStyledTextCtrl for wxStyledTextCtrl {}
impl _wxControl for wxStyledTextCtrl {}
impl _wxWindow for wxStyledTextCtrl {}
impl _wxEvtHandler for wxStyledTextCtrl {}
impl _wxObject for wxStyledTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxStyledTextCtrl {
    pub fn from(handle: *u8) -> @wxStyledTextCtrl { @wxStyledTextCtrl(handle) }
    pub fn null() -> @wxStyledTextCtrl { wxStyledTextCtrl::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> @wxStyledTextCtrl {
        let _txt = wxT(_txt);
        unsafe { @wxStyledTextCtrl(wxStyledTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, style)) }
    }
}

trait _wxStyledTextCtrl : _wxControl {
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
    fn setUndoCollection(&self, collectUndo: bool) {
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
    fn canRedo(&self) -> bool {
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
    fn getUndoCollection(&self) -> bool {
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
    fn getBufferedDraw(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetBufferedDraw(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBufferedDraw(&self, buffered: bool) {
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
    fn setMarginSensitive(&self, margin: c_int, sensitive: bool) {
        unsafe { wxStyledTextCtrl_SetMarginSensitive(self.handle(), margin, sensitive) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMarginSensitive(&self, margin: c_int) -> bool {
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
    fn styleSetBold(&self, style: c_int, bold: bool) {
        unsafe { wxStyledTextCtrl_StyleSetBold(self.handle(), style, bold) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetItalic(&self, style: c_int, italic: bool) {
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
    fn styleSetEOLFilled(&self, style: c_int, filled: bool) {
        unsafe { wxStyledTextCtrl_StyleSetEOLFilled(self.handle(), style, filled) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleResetDefault(&self) {
        unsafe { wxStyledTextCtrl_StyleResetDefault(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetUnderline(&self, style: c_int, underline: bool) {
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
    fn styleSetHotSpot(&self, style: c_int, hotspot: bool) {
        unsafe { wxStyledTextCtrl_StyleSetHotSpot(self.handle(), style, hotspot) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelForeground(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelBackground(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
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
    fn setStyleBytes(&self, length: c_int, styleBytes: *char) {
        unsafe { wxStyledTextCtrl_SetStyleBytes(self.handle(), length, styleBytes) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetVisible(&self, style: c_int, visible: bool) {
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
    fn setWhitespaceForeground(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setWhitespaceBackground(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
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
    fn getCaretLineVisible(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetCaretLineVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setCaretLineVisible(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetCaretLineVisible(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn styleSetChangeable(&self, style: c_int, changeable: bool) {
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
    fn autoCompActive(&self) -> bool {
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
    fn autoCompSetCancelAtStart(&self, cancel: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetCancelAtStart(self.handle(), cancel) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetCancelAtStart(&self) -> bool {
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
    fn autoCompSetChooseSingle(&self, chooseSingle: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetChooseSingle(self.handle(), chooseSingle) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetChooseSingle(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetChooseSingle(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetIgnoreCase(&self, ignoreCase: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetIgnoreCase(self.handle(), ignoreCase) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetIgnoreCase(&self) -> bool {
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
    fn autoCompSetAutoHide(&self, autoHide: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetAutoHide(self.handle(), autoHide) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetAutoHide(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetAutoHide(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompSetDropRestOfWord(&self, dropRestOfWord: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetDropRestOfWord(self.handle(), dropRestOfWord) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn autoCompGetDropRestOfWord(&self) -> bool {
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
    fn setUseTabs(&self, useTabs: bool) {
        unsafe { wxStyledTextCtrl_SetUseTabs(self.handle(), useTabs) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUseTabs(&self) -> bool {
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
    fn setUseHorizontalScrollBar(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetUseHorizontalScrollBar(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUseHorizontalScrollBar(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseHorizontalScrollBar(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setIndentationGuides(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetIndentationGuides(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getIndentationGuides(&self) -> bool {
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
    fn getReadOnly(&self) -> bool {
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
    fn formatRange<T: _wxDC, U: _wxDC, V: _wxRect, W: _wxRect>(&self, doDraw: bool, startPos: c_int, endPos: c_int, draw: &T, target: &U, renderRect: &V, pageRect: &W) -> c_int {
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
    fn getModify(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetModify(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setSelection(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_SetSelection(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn hideSelection(&self, normal: bool) {
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
    fn setReadOnly(&self, readOnly: bool) {
        unsafe { wxStyledTextCtrl_SetReadOnly(self.handle(), readOnly) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canPaste(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanPaste(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn canUndo(&self) -> bool {
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
    fn setOvertype(&self, overtype: bool) {
        unsafe { wxStyledTextCtrl_SetOvertype(self.handle(), overtype) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getOvertype(&self) -> bool {
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
    fn callTipActive(&self) -> bool {
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
    fn getLineVisible(&self, line: c_int) -> bool {
        unsafe { wxStyledTextCtrl_GetLineVisible(self.handle(), line) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldExpanded(&self, line: c_int, expanded: bool) {
        unsafe { wxStyledTextCtrl_SetFoldExpanded(self.handle(), line, expanded) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFoldExpanded(&self, line: c_int) -> bool {
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
    fn setTabIndents(&self, tabIndents: bool) {
        unsafe { wxStyledTextCtrl_SetTabIndents(self.handle(), tabIndents) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTabIndents(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetTabIndents(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setBackSpaceUnIndents(&self, bsUnIndents: bool) {
        unsafe { wxStyledTextCtrl_SetBackSpaceUnIndents(self.handle(), bsUnIndents) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBackSpaceUnIndents(&self) -> bool {
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
    fn wordStartPosition(&self, pos: c_int, onlyWordCharacters: bool) -> c_int {
        unsafe { wxStyledTextCtrl_WordStartPosition(self.handle(), pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn wordEndPosition(&self, pos: c_int, onlyWordCharacters: bool) -> c_int {
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
    fn setEndAtLastLine(&self, endAtLastLine: bool) {
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
    fn setUseVerticalScrollBar(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetUseVerticalScrollBar(self.handle(), show) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getUseVerticalScrollBar(&self) -> bool {
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
    fn getTwoPhaseDraw(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetTwoPhaseDraw(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setTwoPhaseDraw(&self, twoPhase: bool) {
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
    fn setFoldMarginColour(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginColour(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFoldMarginHiColour(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
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
    fn getViewEOL(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetViewEOL(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setViewEOL(&self, visible: bool) {
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
    fn usePopUp(&self, allowPopUp: bool) {
        unsafe { wxStyledTextCtrl_UsePopUp(self.handle(), allowPopUp) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn selectionIsRectangle(&self) -> bool {
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
    fn setSTCFocus(&self, focus: bool) {
        unsafe { wxStyledTextCtrl_SetSTCFocus(self.handle(), focus) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSTCFocus(&self) -> bool {
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
    fn setMouseDownCaptures(&self, captures: bool) {
        unsafe { wxStyledTextCtrl_SetMouseDownCaptures(self.handle(), captures) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMouseDownCaptures(&self) -> bool {
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
    fn setHotspotActiveForeground(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHotspotActiveBackground(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setHotspotActiveUnderline(&self, underline: bool) {
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
    fn styleSetFontAttr(&self, styleNum: c_int, size: c_int, faceName: &str, bold: bool, italic: bool, underline: bool) {
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
    fn getSelection(&self, startPos: *c_int, endPos: *c_int) {
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
    fn getLastKeydownProcessed(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetLastKeydownProcessed(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setLastKeydownProcessed(&self, val: bool) {
        unsafe { wxStyledTextCtrl_SetLastKeydownProcessed(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn saveFile(&self, filename: &str) -> bool {
        let filename = wxT(filename);
        unsafe { wxStyledTextCtrl_SaveFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, filename: &str) -> bool {
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

struct wxSTCDoc(*u8);
impl _wxSTCDoc for wxSTCDoc { fn handle(&self) -> *u8 { **self } }

impl wxSTCDoc {
    pub fn from(handle: *u8) -> @wxSTCDoc { @wxSTCDoc(handle) }
    pub fn null() -> @wxSTCDoc { wxSTCDoc::from(0 as *u8) }
    
}

trait _wxSTCDoc {
    fn handle(&self) -> *u8;
    
}

struct wxMemoryBuffer(*u8);
impl _wxMemoryBuffer for wxMemoryBuffer { fn handle(&self) -> *u8 { **self } }

impl wxMemoryBuffer {
    pub fn from(handle: *u8) -> @wxMemoryBuffer { @wxMemoryBuffer(handle) }
    pub fn null() -> @wxMemoryBuffer { wxMemoryBuffer::from(0 as *u8) }
    
}

trait _wxMemoryBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStyledTextEvent(*u8);
impl _wxStyledTextEvent for wxStyledTextEvent {}
impl _wxCommandEvent for wxStyledTextEvent {}
impl _wxEvent for wxStyledTextEvent {}
impl _wxObject for wxStyledTextEvent { fn handle(&self) -> *u8 { **self } }

impl wxStyledTextEvent {
    pub fn from(handle: *u8) -> @wxStyledTextEvent { @wxStyledTextEvent(handle) }
    pub fn null() -> @wxStyledTextEvent { wxStyledTextEvent::from(0 as *u8) }
    
}

trait _wxStyledTextEvent : _wxCommandEvent {
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
    fn getDragAllowMove(&self) -> bool {
        unsafe { wxStyledTextEvent_GetDragAllowMove(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDragResult(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetDragResult(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getShift(&self) -> bool {
        unsafe { wxStyledTextEvent_GetShift(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getControl(&self) -> bool {
        unsafe { wxStyledTextEvent_GetControl(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getAlt(&self) -> bool {
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
    fn setDragAllowMove(&self, val: bool) {
        unsafe { wxStyledTextEvent_SetDragAllowMove(self.handle(), val) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDragResult(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetDragResult(self.handle(), val) }
    }
}

struct wxGauge95(*u8);
impl _wxGauge95 for wxGauge95 {}
impl _wxGauge for wxGauge95 {}
impl _wxControl for wxGauge95 {}
impl _wxWindow for wxGauge95 {}
impl _wxEvtHandler for wxGauge95 {}
impl _wxObject for wxGauge95 { fn handle(&self) -> *u8 { **self } }

impl wxGauge95 {
    pub fn from(handle: *u8) -> @wxGauge95 { @wxGauge95(handle) }
    pub fn null() -> @wxGauge95 { wxGauge95::from(0 as *u8) }
    
}

trait _wxGauge95 : _wxGauge {
}

struct wxGaugeMSW(*u8);
impl _wxGaugeMSW for wxGaugeMSW {}
impl _wxGauge for wxGaugeMSW {}
impl _wxControl for wxGaugeMSW {}
impl _wxWindow for wxGaugeMSW {}
impl _wxEvtHandler for wxGaugeMSW {}
impl _wxObject for wxGaugeMSW { fn handle(&self) -> *u8 { **self } }

impl wxGaugeMSW {
    pub fn from(handle: *u8) -> @wxGaugeMSW { @wxGaugeMSW(handle) }
    pub fn null() -> @wxGaugeMSW { wxGaugeMSW::from(0 as *u8) }
    
}

trait _wxGaugeMSW : _wxGauge {
}

struct wxSlider95(*u8);
impl _wxSlider95 for wxSlider95 {}
impl _wxSlider for wxSlider95 {}
impl _wxControl for wxSlider95 {}
impl _wxWindow for wxSlider95 {}
impl _wxEvtHandler for wxSlider95 {}
impl _wxObject for wxSlider95 { fn handle(&self) -> *u8 { **self } }

impl wxSlider95 {
    pub fn from(handle: *u8) -> @wxSlider95 { @wxSlider95(handle) }
    pub fn null() -> @wxSlider95 { wxSlider95::from(0 as *u8) }
    
}

trait _wxSlider95 : _wxSlider {
}

struct wxSliderMSW(*u8);
impl _wxSliderMSW for wxSliderMSW {}
impl _wxSlider for wxSliderMSW {}
impl _wxControl for wxSliderMSW {}
impl _wxWindow for wxSliderMSW {}
impl _wxEvtHandler for wxSliderMSW {}
impl _wxObject for wxSliderMSW { fn handle(&self) -> *u8 { **self } }

impl wxSliderMSW {
    pub fn from(handle: *u8) -> @wxSliderMSW { @wxSliderMSW(handle) }
    pub fn null() -> @wxSliderMSW { wxSliderMSW::from(0 as *u8) }
    
}

trait _wxSliderMSW : _wxSlider {
}

struct wxcTreeItemData(*u8);
impl _wxcTreeItemData for wxcTreeItemData {}
impl _wxTreeItemData for wxcTreeItemData {}
impl _wxClientData for wxcTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxcTreeItemData {
    pub fn from(handle: *u8) -> @wxcTreeItemData { @wxcTreeItemData(handle) }
    pub fn null() -> @wxcTreeItemData { wxcTreeItemData::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxClosure>(closure: &T) -> @wxcTreeItemData {
        unsafe { @wxcTreeItemData(wxcTreeItemData_Create(closure.handle())) }
    }
}

trait _wxcTreeItemData : _wxTreeItemData {
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

struct wxInputSink(*u8);
impl _wxInputSink for wxInputSink {}
impl _wxThread for wxInputSink { fn handle(&self) -> *u8 { **self } }

impl wxInputSink {
    pub fn from(handle: *u8) -> @wxInputSink { @wxInputSink(handle) }
    pub fn null() -> @wxInputSink { wxInputSink::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream, U: _wxEvtHandler>(input: &T, evtHandler: &U, bufferLen: c_int) -> @wxInputSink {
        unsafe { @wxInputSink(wxInputSink_Create(input.handle(), evtHandler.handle(), bufferLen)) }
    }
}

trait _wxInputSink : _wxThread {
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

struct wxInputSinkEvent(*u8);
impl _wxInputSinkEvent for wxInputSinkEvent {}
impl _wxEvent for wxInputSinkEvent {}
impl _wxObject for wxInputSinkEvent { fn handle(&self) -> *u8 { **self } }

impl wxInputSinkEvent {
    pub fn from(handle: *u8) -> @wxInputSinkEvent { @wxInputSinkEvent(handle) }
    pub fn null() -> @wxInputSinkEvent { wxInputSinkEvent::from(0 as *u8) }
    
}

trait _wxInputSinkEvent : _wxEvent {
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
    fn lastInput(&self) -> *char {
        unsafe { wxInputSinkEvent_LastInput(self.handle()) }
    }
}

struct wxcHtmlEvent(*u8);
impl _wxcHtmlEvent for wxcHtmlEvent {}
impl _wxCommandEvent for wxcHtmlEvent {}
impl _wxEvent for wxcHtmlEvent {}
impl _wxObject for wxcHtmlEvent { fn handle(&self) -> *u8 { **self } }

impl wxcHtmlEvent {
    pub fn from(handle: *u8) -> @wxcHtmlEvent { @wxcHtmlEvent(handle) }
    pub fn null() -> @wxcHtmlEvent { wxcHtmlEvent::from(0 as *u8) }
    
}

trait _wxcHtmlEvent : _wxCommandEvent {
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

struct wxcHtmlWindow(*u8);
impl _wxcHtmlWindow for wxcHtmlWindow {}
impl _wxHtmlWindow for wxcHtmlWindow {}
impl _wxScrolledWindow for wxcHtmlWindow {}
impl _wxPanel for wxcHtmlWindow {}
impl _wxWindow for wxcHtmlWindow {}
impl _wxEvtHandler for wxcHtmlWindow {}
impl _wxObject for wxcHtmlWindow { fn handle(&self) -> *u8 { **self } }

impl wxcHtmlWindow {
    pub fn from(handle: *u8) -> @wxcHtmlWindow { @wxcHtmlWindow(handle) }
    pub fn null() -> @wxcHtmlWindow { wxcHtmlWindow::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> @wxcHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { @wxcHtmlWindow(wxcHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) }
    }
}

trait _wxcHtmlWindow : _wxHtmlWindow {
}

struct wxGridCellTextEnterEditor(*u8);
impl _wxGridCellTextEnterEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellTextEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellWorker for wxGridCellTextEnterEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellTextEnterEditor {
    pub fn from(handle: *u8) -> @wxGridCellTextEnterEditor { @wxGridCellTextEnterEditor(handle) }
    pub fn null() -> @wxGridCellTextEnterEditor { wxGridCellTextEnterEditor::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn ctor() -> @wxGridCellTextEnterEditor {
        unsafe { @wxGridCellTextEnterEditor(wxGridCellTextEnterEditor_Ctor()) }
    }
}

trait _wxGridCellTextEnterEditor : _wxGridCellTextEditor {
}

struct wxFileConfig(*u8);
impl _wxFileConfig for wxFileConfig {}
impl _wxConfigBase for wxFileConfig { fn handle(&self) -> *u8 { **self } }

impl wxFileConfig {
    pub fn from(handle: *u8) -> @wxFileConfig { @wxFileConfig(handle) }
    pub fn null() -> @wxFileConfig { wxFileConfig::from(0 as *u8) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxInputStream>(inp: &T) -> @wxFileConfig {
        unsafe { @wxFileConfig(wxFileConfig_Create(inp.handle())) }
    }
}

trait _wxFileConfig : _wxConfigBase {
}

