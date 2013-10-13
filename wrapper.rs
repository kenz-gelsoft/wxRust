use std::libc::*;
use native::*;

struct ELJApp(*u8);
impl _ELJApp for ELJApp {}
impl _wxApp for ELJApp {}
impl _wxEvtHandler for ELJApp {}
impl _wxObject for ELJApp { fn handle(&self) -> *u8 { **self } }

impl ELJApp {
    pub fn from(handle: *u8) -> @_ELJApp {
        @ELJApp(handle) as @_ELJApp
    }
    
    #[fixed_stack_segment]
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    pub fn newLogTarget() -> @_ELJLog {
        unsafe { @ELJLog(ELJApp_CreateLogTarget()) as @_ELJLog }
    }
    #[fixed_stack_segment]
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    pub fn displaySize() -> @_wxSize {
        unsafe { @wxSize(ELJApp_DisplaySize()) as @_wxSize }
    }
    #[fixed_stack_segment]
    pub fn enableTooltips(_enable: bool) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    #[fixed_stack_segment]
    pub fn enableTopLevelWindows(_enb: c_int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    #[fixed_stack_segment]
    pub fn executeProcess(_cmd: @_wxString, _snc: c_int, _prc: @_wxProcess) -> c_int {
        unsafe { ELJApp_ExecuteProcess(_cmd.handle(), _snc, _prc.handle()) }
    }
    #[fixed_stack_segment]
    pub fn exit() {
        unsafe { ELJApp_Exit() }
    }
    #[fixed_stack_segment]
    pub fn exitMainLoop() {
        unsafe { ELJApp_ExitMainLoop() }
    }
    #[fixed_stack_segment]
    pub fn findWindowById(_id: c_int, _prt: @_wxWindow) -> *u8 {
        unsafe { ELJApp_FindWindowById(_id, _prt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn findWindowByLabel(_lbl: @_wxString, _prt: @_wxWindow) -> @_wxWindow {
        unsafe { @wxWindow(ELJApp_FindWindowByLabel(_lbl.handle(), _prt.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    pub fn findWindowByName(_lbl: @_wxString, _prt: @_wxWindow) -> @_wxWindow {
        unsafe { @wxWindow(ELJApp_FindWindowByName(_lbl.handle(), _prt.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    pub fn getApp() -> @_wxApp {
        unsafe { @wxApp(ELJApp_GetApp()) as @_wxApp }
    }
    #[fixed_stack_segment]
    pub fn getAppName() -> @_wxString {
        unsafe { @wxString(ELJApp_GetAppName()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getClassName() -> @_wxString {
        unsafe { @wxString(ELJApp_GetClassName()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    pub fn getOsDescription() -> @_wxString {
        unsafe { @wxString(ELJApp_GetOsDescription()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getOsVersion(_maj: *u8, _min: *u8) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    pub fn getTopWindow() -> @_wxWindow {
        unsafe { @wxWindow(ELJApp_GetTopWindow()) as @_wxWindow }
    }
    #[fixed_stack_segment]
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    pub fn getUserHome(_usr: *u8) -> @_wxString {
        unsafe { @wxString(ELJApp_GetUserHome(_usr)) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getUserId() -> @_wxString {
        unsafe { @wxString(ELJApp_GetUserId()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getUserName() -> @_wxString {
        unsafe { @wxString(ELJApp_GetUserName()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getVendorName() -> @_wxString {
        unsafe { @wxString(ELJApp_GetVendorName()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn initAllImageHandlers() {
        unsafe { ELJApp_InitAllImageHandlers() }
    }
    #[fixed_stack_segment]
    pub fn initialized() -> bool {
        unsafe { ELJApp_Initialized() }
    }
    #[fixed_stack_segment]
    pub fn mainLoop() -> c_int {
        unsafe { ELJApp_MainLoop() }
    }
    #[fixed_stack_segment]
    pub fn mousePosition() -> @_wxPoint {
        unsafe { @wxPoint(ELJApp_MousePosition()) as @_wxPoint }
    }
    #[fixed_stack_segment]
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    pub fn safeYield(_win: @_wxWindow) -> c_int {
        unsafe { ELJApp_SafeYield(_win.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setAppName(name: @_wxString) {
        unsafe { ELJApp_SetAppName(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setClassName(name: @_wxString) {
        unsafe { ELJApp_SetClassName(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setExitOnFrameDelete(flag: c_int) {
        unsafe { ELJApp_SetExitOnFrameDelete(flag) }
    }
    #[fixed_stack_segment]
    pub fn setPrintMode(mode: c_int) {
        unsafe { ELJApp_SetPrintMode(mode) }
    }
    #[fixed_stack_segment]
    pub fn setTooltipDelay(_ms: c_int) {
        unsafe { ELJApp_SetTooltipDelay(_ms) }
    }
    #[fixed_stack_segment]
    pub fn setTopWindow(_wnd: @_wxWindow) {
        unsafe { ELJApp_SetTopWindow(_wnd.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    #[fixed_stack_segment]
    pub fn setVendorName(name: @_wxString) {
        unsafe { ELJApp_SetVendorName(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn sleep(_scs: c_int) {
        unsafe { ELJApp_Sleep(_scs) }
    }
    #[fixed_stack_segment]
    pub fn milliSleep(_mscs: c_int) {
        unsafe { ELJApp_MilliSleep(_mscs) }
    }
    #[fixed_stack_segment]
    pub fn yield_() -> c_int {
        unsafe { ELJApp_Yield() }
    }
    #[fixed_stack_segment]
    pub fn isTerminating() -> c_int {
        unsafe { ELJApp_IsTerminating() }
    }
    #[fixed_stack_segment]
    pub fn initializeC(closure: @_wxClosure, _argc: c_int, _argv: *wchar_t) {
        unsafe { ELJApp_InitializeC(closure.handle(), _argc, _argv) }
    }
    #[fixed_stack_segment]
    pub fn getIdleInterval() -> c_int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_ELJArtProv {
        @ELJArtProv(handle) as @_ELJArtProv
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _clb: *u8) -> @_ELJArtProv {
        unsafe { @ELJArtProv(ELJArtProv_Create(_obj, _clb)) as @_ELJArtProv }
    }
}

trait _ELJArtProv : _wxArtProvider {
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_ELJClient {
        @ELJClient(handle) as @_ELJClient
    }
    
}

trait _ELJClient : _wxClient {
}

struct ELJCommand(*u8);
impl _ELJCommand for ELJCommand {}
impl _wxCommand for ELJCommand {}
impl _wxObject for ELJCommand { fn handle(&self) -> *u8 { **self } }

impl ELJCommand {
    pub fn from(handle: *u8) -> @_ELJCommand {
        @ELJCommand(handle) as @_ELJCommand
    }
    
}

trait _ELJCommand : _wxCommand {
}

struct ELJConnection(*u8);
impl _ELJConnection for ELJConnection {}
impl _wxConnection for ELJConnection {}
impl _wxConnectionBase for ELJConnection {}
impl _wxObject for ELJConnection { fn handle(&self) -> *u8 { **self } }

impl ELJConnection {
    pub fn from(handle: *u8) -> @_ELJConnection {
        @ELJConnection(handle) as @_ELJConnection
    }
    
}

trait _ELJConnection : _wxConnection {
}

struct ELJDragDataObject(*u8);
impl _ELJDragDataObject for ELJDragDataObject { fn handle(&self) -> *u8 { **self } }

impl ELJDragDataObject {
    pub fn from(handle: *u8) -> @_ELJDragDataObject {
        @ELJDragDataObject(handle) as @_ELJDragDataObject
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fmt: @_wxString, _func1: *u8, _func2: *u8, _func3: *u8) -> @_ELJDragDataObject {
        unsafe { @ELJDragDataObject(ELJDragDataObject_Create(_obj, _fmt.handle(), _func1, _func2, _func3)) as @_ELJDragDataObject }
    }
}

trait _ELJDragDataObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.handle()) }
    }
}

struct ELJDropTarget(*u8);
impl _ELJDropTarget for ELJDropTarget {}
impl _wxDropTarget for ELJDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJDropTarget {
    pub fn from(handle: *u8) -> @_ELJDropTarget {
        @ELJDropTarget(handle) as @_ELJDropTarget
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8) -> @_ELJDropTarget {
        unsafe { @ELJDropTarget(ELJDropTarget_Create(_obj)) as @_ELJDropTarget }
    }
}

trait _ELJDropTarget : _wxDropTarget {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setOnData(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnDragOver(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnDrop(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnEnter(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnLeave(&self, _func: *u8) {
        unsafe { ELJDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

struct ELJFileDropTarget(*u8);
impl _ELJFileDropTarget for ELJFileDropTarget {}
impl _wxFileDropTarget for ELJFileDropTarget {}
impl _wxDropTarget for ELJFileDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJFileDropTarget {
    pub fn from(handle: *u8) -> @_ELJFileDropTarget {
        @ELJFileDropTarget(handle) as @_ELJFileDropTarget
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _func: *u8) -> @_ELJFileDropTarget {
        unsafe { @ELJFileDropTarget(ELJFileDropTarget_Create(_obj, _func)) as @_ELJFileDropTarget }
    }
}

trait _ELJFileDropTarget : _wxFileDropTarget {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJFileDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setOnData(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnDragOver(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnDrop(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnEnter(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnLeave(&self, _func: *u8) {
        unsafe { ELJFileDropTarget_SetOnLeave(self.handle(), _func) }
    }
}

struct ELJGridTable(*u8);
impl _ELJGridTable for ELJGridTable {}
impl _wxGridTableBase for ELJGridTable {}
impl _wxObject for ELJGridTable { fn handle(&self) -> *u8 { **self } }

impl ELJGridTable {
    pub fn from(handle: *u8) -> @_ELJGridTable {
        @ELJGridTable(handle) as @_ELJGridTable
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _EifGetNumberRows: *u8, _EifGetNumberCols: *u8, _EifGetValue: *u8, _EifSetValue: *u8, _EifIsEmptyCell: *u8, _EifClear: *u8, _EifInsertRows: *u8, _EifAppendRows: *u8, _EifDeleteRows: *u8, _EifInsertCols: *u8, _EifAppendCols: *u8, _EifDeleteCols: *u8, _EifSetRowLabelValue: *u8, _EifSetColLabelValue: *u8, _EifGetRowLabelValue: *u8, _EifGetColLabelValue: *u8) -> @_ELJGridTable {
        unsafe { @ELJGridTable(ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)) as @_ELJGridTable }
    }
}

trait _ELJGridTable : _wxGridTableBase {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJGridTable_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getView(&self) -> @_wxView {
        unsafe { @wxView(ELJGridTable_GetView(self.handle())) as @_wxView }
    }
    #[fixed_stack_segment]
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *u8 {
        unsafe { ELJGridTable_SendTableMessage(self.handle(), id, val1, val2) }
    }
}

struct ELJLocale(*u8);
impl _ELJLocale for ELJLocale {}
impl _wxLocale for ELJLocale { fn handle(&self) -> *u8 { **self } }

impl ELJLocale {
    pub fn from(handle: *u8) -> @_ELJLocale {
        @ELJLocale(handle) as @_ELJLocale
    }
    
}

trait _ELJLocale : _wxLocale {
}

struct ELJLog(*u8);
impl _ELJLog for ELJLog {}
impl _wxLog for ELJLog { fn handle(&self) -> *u8 { **self } }

impl ELJLog {
    pub fn from(handle: *u8) -> @_ELJLog {
        @ELJLog(handle) as @_ELJLog
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8) -> @_ELJLog {
        unsafe { @ELJLog(ELJLog_Create(_obj, _fnc)) as @_ELJLog }
    }
    #[fixed_stack_segment]
    pub fn getActiveTarget() -> *u8 {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

trait _ELJLog : _wxLog {
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: *wchar_t) {
        unsafe { ELJLog_AddTraceMask(self.handle(), str) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJLog_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn dontCreateOnDemand(&self) {
        unsafe { ELJLog_DontCreateOnDemand(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableLogging(&self, doIt: bool) -> c_int {
        unsafe { ELJLog_EnableLogging(self.handle(), doIt) }
    }
    #[fixed_stack_segment]
    fn flush(&self) {
        unsafe { ELJLog_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    fn flushActive(&self) {
        unsafe { ELJLog_FlushActive(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTimestamp(&self) -> *u8 {
        unsafe { ELJLog_GetTimestamp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTraceMask(&self) -> c_int {
        unsafe { ELJLog_GetTraceMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVerbose(&self) -> c_int {
        unsafe { ELJLog_GetVerbose(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasPendingMessages(&self) -> bool {
        unsafe { ELJLog_HasPendingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isAllowedTraceMask(&self, mask: @_wxMask) -> bool {
        unsafe { ELJLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self) -> bool {
        unsafe { ELJLog_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: c_int, szString: *u8, t: c_int) {
        unsafe { ELJLog_OnLog(self.handle(), level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask(&self, str: *wchar_t) {
        unsafe { ELJLog_RemoveTraceMask(self.handle(), str) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { ELJLog_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) -> *u8 {
        unsafe { ELJLog_SetActiveTarget(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setTimestamp(&self, ts: *u8) {
        unsafe { ELJLog_SetTimestamp(self.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn setTraceMask(&self, ulMask: c_int) {
        unsafe { ELJLog_SetTraceMask(self.handle(), ulMask) }
    }
    #[fixed_stack_segment]
    fn setVerbose(&self, bVerbose: c_int) {
        unsafe { ELJLog_SetVerbose(self.handle(), bVerbose) }
    }
    #[fixed_stack_segment]
    fn suspend(&self) {
        unsafe { ELJLog_Suspend(self.handle()) }
    }
}

struct ELJMessageParameters(*u8);
impl _ELJMessageParameters for ELJMessageParameters { fn handle(&self) -> *u8 { **self } }

impl ELJMessageParameters {
    pub fn from(handle: *u8) -> @_ELJMessageParameters {
        @ELJMessageParameters(handle) as @_ELJMessageParameters
    }
    
}

trait _ELJMessageParameters {
    fn handle(&self) -> *u8;
    
}

struct ELJPlotCurve(*u8);
impl _ELJPlotCurve for ELJPlotCurve {}
impl _wxPlotCurve for ELJPlotCurve {}
impl _wxObject for ELJPlotCurve { fn handle(&self) -> *u8 { **self } }

impl ELJPlotCurve {
    pub fn from(handle: *u8) -> @_ELJPlotCurve {
        @ELJPlotCurve(handle) as @_ELJPlotCurve
    }
    
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
    pub fn from(handle: *u8) -> @_ELJPreviewControlBar {
        @ELJPreviewControlBar(handle) as @_ELJPreviewControlBar
    }
    
    #[fixed_stack_segment]
    pub fn new(preview: *u8, buttons: c_int, parent: @_wxWindow, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @_ELJPreviewControlBar {
        unsafe { @ELJPreviewControlBar(ELJPreviewControlBar_Create(preview, buttons, parent.handle(), title, x, y, w, h, style)) as @_ELJPreviewControlBar }
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
    pub fn from(handle: *u8) -> @_ELJPreviewFrame {
        @ELJPreviewFrame(handle) as @_ELJPreviewFrame
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _init: *u8, _create_canvas: *u8, _create_toolbar: *u8, preview: *u8, parent: @_wxWindow, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @_ELJPreviewFrame {
        unsafe { @ELJPreviewFrame(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.handle(), title, x, y, w, h, style)) as @_ELJPreviewFrame }
    }
}

trait _ELJPreviewFrame : _wxPreviewFrame {
    #[fixed_stack_segment]
    fn getControlBar(&self) -> *u8 {
        unsafe { ELJPreviewFrame_GetControlBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPreviewCanvas(&self) -> @_wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(ELJPreviewFrame_GetPreviewCanvas(self.handle())) as @_wxPreviewCanvas }
    }
    #[fixed_stack_segment]
    fn getPrintPreview(&self) -> @_wxPrintPreview {
        unsafe { @wxPrintPreview(ELJPreviewFrame_GetPrintPreview(self.handle())) as @_wxPrintPreview }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { ELJPreviewFrame_Initialize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setControlBar(&self, obj: *u8) {
        unsafe { ELJPreviewFrame_SetControlBar(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn setPreviewCanvas(&self, obj: @_wxPreviewCanvas) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintPreview(&self, obj: @_wxPrintPreview) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.handle(), obj.handle()) }
    }
}

struct ELJServer(*u8);
impl _ELJServer for ELJServer {}
impl _wxServer for ELJServer {}
impl _wxServerBase for ELJServer {}
impl _wxObject for ELJServer { fn handle(&self) -> *u8 { **self } }

impl ELJServer {
    pub fn from(handle: *u8) -> @_ELJServer {
        @ELJServer(handle) as @_ELJServer
    }
    
}

trait _ELJServer : _wxServer {
}

struct ELJTextDropTarget(*u8);
impl _ELJTextDropTarget for ELJTextDropTarget {}
impl _wxTextDropTarget for ELJTextDropTarget {}
impl _wxDropTarget for ELJTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJTextDropTarget {
    pub fn from(handle: *u8) -> @_ELJTextDropTarget {
        @ELJTextDropTarget(handle) as @_ELJTextDropTarget
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _func: *u8) -> @_ELJTextDropTarget {
        unsafe { @ELJTextDropTarget(ELJTextDropTarget_Create(_obj, _func)) as @_ELJTextDropTarget }
    }
}

trait _ELJTextDropTarget : _wxTextDropTarget {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJTextDropTarget_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setOnData(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnData(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnDragOver(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnDragOver(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnDrop(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnDrop(self.handle(), _func) }
    }
    #[fixed_stack_segment]
    fn setOnEnter(&self, _func: *u8) {
        unsafe { ELJTextDropTarget_SetOnEnter(self.handle(), _func) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_ELJTextValidator {
        @ELJTextValidator(handle) as @_ELJTextValidator
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8, _txt: *wchar_t, _stl: c_int) -> @_ELJTextValidator {
        unsafe { @ELJTextValidator(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) as @_ELJTextValidator }
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
    pub fn from(handle: *u8) -> @_cbAntiflickerPlugin {
        @cbAntiflickerPlugin(handle) as @_cbAntiflickerPlugin
    }
    
}

trait _cbAntiflickerPlugin : _cbPluginBase {
}

struct cbBarDragPlugin(*u8);
impl _cbBarDragPlugin for cbBarDragPlugin {}
impl _cbPluginBase for cbBarDragPlugin {}
impl _wxEvtHandler for cbBarDragPlugin {}
impl _wxObject for cbBarDragPlugin { fn handle(&self) -> *u8 { **self } }

impl cbBarDragPlugin {
    pub fn from(handle: *u8) -> @_cbBarDragPlugin {
        @cbBarDragPlugin(handle) as @_cbBarDragPlugin
    }
    
}

trait _cbBarDragPlugin : _cbPluginBase {
}

struct cbBarHintsPlugin(*u8);
impl _cbBarHintsPlugin for cbBarHintsPlugin {}
impl _cbPluginBase for cbBarHintsPlugin {}
impl _wxEvtHandler for cbBarHintsPlugin {}
impl _wxObject for cbBarHintsPlugin { fn handle(&self) -> *u8 { **self } }

impl cbBarHintsPlugin {
    pub fn from(handle: *u8) -> @_cbBarHintsPlugin {
        @cbBarHintsPlugin(handle) as @_cbBarHintsPlugin
    }
    
}

trait _cbBarHintsPlugin : _cbPluginBase {
}

struct cbBarInfo(*u8);
impl _cbBarInfo for cbBarInfo {}
impl _wxObject for cbBarInfo { fn handle(&self) -> *u8 { **self } }

impl cbBarInfo {
    pub fn from(handle: *u8) -> @_cbBarInfo {
        @cbBarInfo(handle) as @_cbBarInfo
    }
    
}

trait _cbBarInfo : _wxObject {
}

struct cbBarSpy(*u8);
impl _cbBarSpy for cbBarSpy {}
impl _wxEvtHandler for cbBarSpy {}
impl _wxObject for cbBarSpy { fn handle(&self) -> *u8 { **self } }

impl cbBarSpy {
    pub fn from(handle: *u8) -> @_cbBarSpy {
        @cbBarSpy(handle) as @_cbBarSpy
    }
    
}

trait _cbBarSpy : _wxEvtHandler {
}

struct cbCloseBox(*u8);
impl _cbCloseBox for cbCloseBox {}
impl _cbMiniButton for cbCloseBox {}
impl _wxObject for cbCloseBox { fn handle(&self) -> *u8 { **self } }

impl cbCloseBox {
    pub fn from(handle: *u8) -> @_cbCloseBox {
        @cbCloseBox(handle) as @_cbCloseBox
    }
    
}

trait _cbCloseBox : _cbMiniButton {
}

struct cbCollapseBox(*u8);
impl _cbCollapseBox for cbCollapseBox {}
impl _cbMiniButton for cbCollapseBox {}
impl _wxObject for cbCollapseBox { fn handle(&self) -> *u8 { **self } }

impl cbCollapseBox {
    pub fn from(handle: *u8) -> @_cbCollapseBox {
        @cbCollapseBox(handle) as @_cbCollapseBox
    }
    
}

trait _cbCollapseBox : _cbMiniButton {
}

struct cbCommonPaneProperties(*u8);
impl _cbCommonPaneProperties for cbCommonPaneProperties {}
impl _wxObject for cbCommonPaneProperties { fn handle(&self) -> *u8 { **self } }

impl cbCommonPaneProperties {
    pub fn from(handle: *u8) -> @_cbCommonPaneProperties {
        @cbCommonPaneProperties(handle) as @_cbCommonPaneProperties
    }
    
}

trait _cbCommonPaneProperties : _wxObject {
}

struct cbCustomizeBarEvent(*u8);
impl _cbCustomizeBarEvent for cbCustomizeBarEvent {}
impl _cbPluginEvent for cbCustomizeBarEvent {}
impl _wxEvent for cbCustomizeBarEvent {}
impl _wxObject for cbCustomizeBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeBarEvent {
    pub fn from(handle: *u8) -> @_cbCustomizeBarEvent {
        @cbCustomizeBarEvent(handle) as @_cbCustomizeBarEvent
    }
    
}

trait _cbCustomizeBarEvent : _cbPluginEvent {
}

struct cbCustomizeLayoutEvent(*u8);
impl _cbCustomizeLayoutEvent for cbCustomizeLayoutEvent {}
impl _cbPluginEvent for cbCustomizeLayoutEvent {}
impl _wxEvent for cbCustomizeLayoutEvent {}
impl _wxObject for cbCustomizeLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeLayoutEvent {
    pub fn from(handle: *u8) -> @_cbCustomizeLayoutEvent {
        @cbCustomizeLayoutEvent(handle) as @_cbCustomizeLayoutEvent
    }
    
}

trait _cbCustomizeLayoutEvent : _cbPluginEvent {
}

struct cbDimHandlerBase(*u8);
impl _cbDimHandlerBase for cbDimHandlerBase {}
impl _wxObject for cbDimHandlerBase { fn handle(&self) -> *u8 { **self } }

impl cbDimHandlerBase {
    pub fn from(handle: *u8) -> @_cbDimHandlerBase {
        @cbDimHandlerBase(handle) as @_cbDimHandlerBase
    }
    
}

trait _cbDimHandlerBase : _wxObject {
}

struct cbDimInfo(*u8);
impl _cbDimInfo for cbDimInfo {}
impl _wxObject for cbDimInfo { fn handle(&self) -> *u8 { **self } }

impl cbDimInfo {
    pub fn from(handle: *u8) -> @_cbDimInfo {
        @cbDimInfo(handle) as @_cbDimInfo
    }
    
}

trait _cbDimInfo : _wxObject {
}

struct cbDockBox(*u8);
impl _cbDockBox for cbDockBox {}
impl _cbMiniButton for cbDockBox {}
impl _wxObject for cbDockBox { fn handle(&self) -> *u8 { **self } }

impl cbDockBox {
    pub fn from(handle: *u8) -> @_cbDockBox {
        @cbDockBox(handle) as @_cbDockBox
    }
    
}

trait _cbDockBox : _cbMiniButton {
}

struct cbDockPane(*u8);
impl _cbDockPane for cbDockPane {}
impl _wxObject for cbDockPane { fn handle(&self) -> *u8 { **self } }

impl cbDockPane {
    pub fn from(handle: *u8) -> @_cbDockPane {
        @cbDockPane(handle) as @_cbDockPane
    }
    
}

trait _cbDockPane : _wxObject {
}

struct cbDrawBarDecorEvent(*u8);
impl _cbDrawBarDecorEvent for cbDrawBarDecorEvent {}
impl _cbPluginEvent for cbDrawBarDecorEvent {}
impl _wxEvent for cbDrawBarDecorEvent {}
impl _wxObject for cbDrawBarDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarDecorEvent {
    pub fn from(handle: *u8) -> @_cbDrawBarDecorEvent {
        @cbDrawBarDecorEvent(handle) as @_cbDrawBarDecorEvent
    }
    
}

trait _cbDrawBarDecorEvent : _cbPluginEvent {
}

struct cbDrawBarHandlesEvent(*u8);
impl _cbDrawBarHandlesEvent for cbDrawBarHandlesEvent {}
impl _cbPluginEvent for cbDrawBarHandlesEvent {}
impl _wxEvent for cbDrawBarHandlesEvent {}
impl _wxObject for cbDrawBarHandlesEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarHandlesEvent {
    pub fn from(handle: *u8) -> @_cbDrawBarHandlesEvent {
        @cbDrawBarHandlesEvent(handle) as @_cbDrawBarHandlesEvent
    }
    
}

trait _cbDrawBarHandlesEvent : _cbPluginEvent {
}

struct cbDrawHintRectEvent(*u8);
impl _cbDrawHintRectEvent for cbDrawHintRectEvent {}
impl _cbPluginEvent for cbDrawHintRectEvent {}
impl _wxEvent for cbDrawHintRectEvent {}
impl _wxObject for cbDrawHintRectEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawHintRectEvent {
    pub fn from(handle: *u8) -> @_cbDrawHintRectEvent {
        @cbDrawHintRectEvent(handle) as @_cbDrawHintRectEvent
    }
    
}

trait _cbDrawHintRectEvent : _cbPluginEvent {
}

struct cbDrawPaneBkGroundEvent(*u8);
impl _cbDrawPaneBkGroundEvent for cbDrawPaneBkGroundEvent {}
impl _cbPluginEvent for cbDrawPaneBkGroundEvent {}
impl _wxEvent for cbDrawPaneBkGroundEvent {}
impl _wxObject for cbDrawPaneBkGroundEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneBkGroundEvent {
    pub fn from(handle: *u8) -> @_cbDrawPaneBkGroundEvent {
        @cbDrawPaneBkGroundEvent(handle) as @_cbDrawPaneBkGroundEvent
    }
    
}

trait _cbDrawPaneBkGroundEvent : _cbPluginEvent {
}

struct cbDrawPaneDecorEvent(*u8);
impl _cbDrawPaneDecorEvent for cbDrawPaneDecorEvent {}
impl _cbPluginEvent for cbDrawPaneDecorEvent {}
impl _wxEvent for cbDrawPaneDecorEvent {}
impl _wxObject for cbDrawPaneDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneDecorEvent {
    pub fn from(handle: *u8) -> @_cbDrawPaneDecorEvent {
        @cbDrawPaneDecorEvent(handle) as @_cbDrawPaneDecorEvent
    }
    
}

trait _cbDrawPaneDecorEvent : _cbPluginEvent {
}

struct cbDrawRowBkGroundEvent(*u8);
impl _cbDrawRowBkGroundEvent for cbDrawRowBkGroundEvent {}
impl _cbPluginEvent for cbDrawRowBkGroundEvent {}
impl _wxEvent for cbDrawRowBkGroundEvent {}
impl _wxObject for cbDrawRowBkGroundEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowBkGroundEvent {
    pub fn from(handle: *u8) -> @_cbDrawRowBkGroundEvent {
        @cbDrawRowBkGroundEvent(handle) as @_cbDrawRowBkGroundEvent
    }
    
}

trait _cbDrawRowBkGroundEvent : _cbPluginEvent {
}

struct cbDrawRowDecorEvent(*u8);
impl _cbDrawRowDecorEvent for cbDrawRowDecorEvent {}
impl _cbPluginEvent for cbDrawRowDecorEvent {}
impl _wxEvent for cbDrawRowDecorEvent {}
impl _wxObject for cbDrawRowDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowDecorEvent {
    pub fn from(handle: *u8) -> @_cbDrawRowDecorEvent {
        @cbDrawRowDecorEvent(handle) as @_cbDrawRowDecorEvent
    }
    
}

trait _cbDrawRowDecorEvent : _cbPluginEvent {
}

struct cbDrawRowHandlesEvent(*u8);
impl _cbDrawRowHandlesEvent for cbDrawRowHandlesEvent {}
impl _cbPluginEvent for cbDrawRowHandlesEvent {}
impl _wxEvent for cbDrawRowHandlesEvent {}
impl _wxObject for cbDrawRowHandlesEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowHandlesEvent {
    pub fn from(handle: *u8) -> @_cbDrawRowHandlesEvent {
        @cbDrawRowHandlesEvent(handle) as @_cbDrawRowHandlesEvent
    }
    
}

trait _cbDrawRowHandlesEvent : _cbPluginEvent {
}

struct cbDynToolBarDimHandler(*u8);
impl _cbDynToolBarDimHandler for cbDynToolBarDimHandler {}
impl _cbDimHandlerBase for cbDynToolBarDimHandler {}
impl _wxObject for cbDynToolBarDimHandler { fn handle(&self) -> *u8 { **self } }

impl cbDynToolBarDimHandler {
    pub fn from(handle: *u8) -> @_cbDynToolBarDimHandler {
        @cbDynToolBarDimHandler(handle) as @_cbDynToolBarDimHandler
    }
    
}

trait _cbDynToolBarDimHandler : _cbDimHandlerBase {
}

struct cbFinishDrawInAreaEvent(*u8);
impl _cbFinishDrawInAreaEvent for cbFinishDrawInAreaEvent {}
impl _cbPluginEvent for cbFinishDrawInAreaEvent {}
impl _wxEvent for cbFinishDrawInAreaEvent {}
impl _wxObject for cbFinishDrawInAreaEvent { fn handle(&self) -> *u8 { **self } }

impl cbFinishDrawInAreaEvent {
    pub fn from(handle: *u8) -> @_cbFinishDrawInAreaEvent {
        @cbFinishDrawInAreaEvent(handle) as @_cbFinishDrawInAreaEvent
    }
    
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
    pub fn from(handle: *u8) -> @_cbFloatedBarWindow {
        @cbFloatedBarWindow(handle) as @_cbFloatedBarWindow
    }
    
}

trait _cbFloatedBarWindow : _wxToolWindow {
}

struct cbGCUpdatesMgr(*u8);
impl _cbGCUpdatesMgr for cbGCUpdatesMgr {}
impl _cbSimpleUpdatesMgr for cbGCUpdatesMgr {}
impl _cbUpdatesManagerBase for cbGCUpdatesMgr {}
impl _wxObject for cbGCUpdatesMgr { fn handle(&self) -> *u8 { **self } }

impl cbGCUpdatesMgr {
    pub fn from(handle: *u8) -> @_cbGCUpdatesMgr {
        @cbGCUpdatesMgr(handle) as @_cbGCUpdatesMgr
    }
    
}

trait _cbGCUpdatesMgr : _cbSimpleUpdatesMgr {
}

struct cbHintAnimationPlugin(*u8);
impl _cbHintAnimationPlugin for cbHintAnimationPlugin {}
impl _cbPluginBase for cbHintAnimationPlugin {}
impl _wxEvtHandler for cbHintAnimationPlugin {}
impl _wxObject for cbHintAnimationPlugin { fn handle(&self) -> *u8 { **self } }

impl cbHintAnimationPlugin {
    pub fn from(handle: *u8) -> @_cbHintAnimationPlugin {
        @cbHintAnimationPlugin(handle) as @_cbHintAnimationPlugin
    }
    
}

trait _cbHintAnimationPlugin : _cbPluginBase {
}

struct cbInsertBarEvent(*u8);
impl _cbInsertBarEvent for cbInsertBarEvent {}
impl _cbPluginEvent for cbInsertBarEvent {}
impl _wxEvent for cbInsertBarEvent {}
impl _wxObject for cbInsertBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbInsertBarEvent {
    pub fn from(handle: *u8) -> @_cbInsertBarEvent {
        @cbInsertBarEvent(handle) as @_cbInsertBarEvent
    }
    
}

trait _cbInsertBarEvent : _cbPluginEvent {
}

struct cbLayoutRowEvent(*u8);
impl _cbLayoutRowEvent for cbLayoutRowEvent {}
impl _cbPluginEvent for cbLayoutRowEvent {}
impl _wxEvent for cbLayoutRowEvent {}
impl _wxObject for cbLayoutRowEvent { fn handle(&self) -> *u8 { **self } }

impl cbLayoutRowEvent {
    pub fn from(handle: *u8) -> @_cbLayoutRowEvent {
        @cbLayoutRowEvent(handle) as @_cbLayoutRowEvent
    }
    
}

trait _cbLayoutRowEvent : _cbPluginEvent {
}

struct cbLeftDClickEvent(*u8);
impl _cbLeftDClickEvent for cbLeftDClickEvent {}
impl _cbPluginEvent for cbLeftDClickEvent {}
impl _wxEvent for cbLeftDClickEvent {}
impl _wxObject for cbLeftDClickEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftDClickEvent {
    pub fn from(handle: *u8) -> @_cbLeftDClickEvent {
        @cbLeftDClickEvent(handle) as @_cbLeftDClickEvent
    }
    
}

trait _cbLeftDClickEvent : _cbPluginEvent {
}

struct cbLeftDownEvent(*u8);
impl _cbLeftDownEvent for cbLeftDownEvent {}
impl _cbPluginEvent for cbLeftDownEvent {}
impl _wxEvent for cbLeftDownEvent {}
impl _wxObject for cbLeftDownEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftDownEvent {
    pub fn from(handle: *u8) -> @_cbLeftDownEvent {
        @cbLeftDownEvent(handle) as @_cbLeftDownEvent
    }
    
}

trait _cbLeftDownEvent : _cbPluginEvent {
}

struct cbLeftUpEvent(*u8);
impl _cbLeftUpEvent for cbLeftUpEvent {}
impl _cbPluginEvent for cbLeftUpEvent {}
impl _wxEvent for cbLeftUpEvent {}
impl _wxObject for cbLeftUpEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftUpEvent {
    pub fn from(handle: *u8) -> @_cbLeftUpEvent {
        @cbLeftUpEvent(handle) as @_cbLeftUpEvent
    }
    
}

trait _cbLeftUpEvent : _cbPluginEvent {
}

struct cbMiniButton(*u8);
impl _cbMiniButton for cbMiniButton {}
impl _wxObject for cbMiniButton { fn handle(&self) -> *u8 { **self } }

impl cbMiniButton {
    pub fn from(handle: *u8) -> @_cbMiniButton {
        @cbMiniButton(handle) as @_cbMiniButton
    }
    
}

trait _cbMiniButton : _wxObject {
}

struct cbMotionEvent(*u8);
impl _cbMotionEvent for cbMotionEvent {}
impl _cbPluginEvent for cbMotionEvent {}
impl _wxEvent for cbMotionEvent {}
impl _wxObject for cbMotionEvent { fn handle(&self) -> *u8 { **self } }

impl cbMotionEvent {
    pub fn from(handle: *u8) -> @_cbMotionEvent {
        @cbMotionEvent(handle) as @_cbMotionEvent
    }
    
}

trait _cbMotionEvent : _cbPluginEvent {
}

struct cbPaneDrawPlugin(*u8);
impl _cbPaneDrawPlugin for cbPaneDrawPlugin {}
impl _cbPluginBase for cbPaneDrawPlugin {}
impl _wxEvtHandler for cbPaneDrawPlugin {}
impl _wxObject for cbPaneDrawPlugin { fn handle(&self) -> *u8 { **self } }

impl cbPaneDrawPlugin {
    pub fn from(handle: *u8) -> @_cbPaneDrawPlugin {
        @cbPaneDrawPlugin(handle) as @_cbPaneDrawPlugin
    }
    
}

trait _cbPaneDrawPlugin : _cbPluginBase {
}

struct cbPluginBase(*u8);
impl _cbPluginBase for cbPluginBase {}
impl _wxEvtHandler for cbPluginBase {}
impl _wxObject for cbPluginBase { fn handle(&self) -> *u8 { **self } }

impl cbPluginBase {
    pub fn from(handle: *u8) -> @_cbPluginBase {
        @cbPluginBase(handle) as @_cbPluginBase
    }
    
}

trait _cbPluginBase : _wxEvtHandler {
}

struct cbPluginEvent(*u8);
impl _cbPluginEvent for cbPluginEvent {}
impl _wxEvent for cbPluginEvent {}
impl _wxObject for cbPluginEvent { fn handle(&self) -> *u8 { **self } }

impl cbPluginEvent {
    pub fn from(handle: *u8) -> @_cbPluginEvent {
        @cbPluginEvent(handle) as @_cbPluginEvent
    }
    
}

trait _cbPluginEvent : _wxEvent {
}

struct cbRemoveBarEvent(*u8);
impl _cbRemoveBarEvent for cbRemoveBarEvent {}
impl _cbPluginEvent for cbRemoveBarEvent {}
impl _wxEvent for cbRemoveBarEvent {}
impl _wxObject for cbRemoveBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbRemoveBarEvent {
    pub fn from(handle: *u8) -> @_cbRemoveBarEvent {
        @cbRemoveBarEvent(handle) as @_cbRemoveBarEvent
    }
    
}

trait _cbRemoveBarEvent : _cbPluginEvent {
}

struct cbResizeBarEvent(*u8);
impl _cbResizeBarEvent for cbResizeBarEvent {}
impl _cbPluginEvent for cbResizeBarEvent {}
impl _wxEvent for cbResizeBarEvent {}
impl _wxObject for cbResizeBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbResizeBarEvent {
    pub fn from(handle: *u8) -> @_cbResizeBarEvent {
        @cbResizeBarEvent(handle) as @_cbResizeBarEvent
    }
    
}

trait _cbResizeBarEvent : _cbPluginEvent {
}

struct cbResizeRowEvent(*u8);
impl _cbResizeRowEvent for cbResizeRowEvent {}
impl _cbPluginEvent for cbResizeRowEvent {}
impl _wxEvent for cbResizeRowEvent {}
impl _wxObject for cbResizeRowEvent { fn handle(&self) -> *u8 { **self } }

impl cbResizeRowEvent {
    pub fn from(handle: *u8) -> @_cbResizeRowEvent {
        @cbResizeRowEvent(handle) as @_cbResizeRowEvent
    }
    
}

trait _cbResizeRowEvent : _cbPluginEvent {
}

struct cbRightDownEvent(*u8);
impl _cbRightDownEvent for cbRightDownEvent {}
impl _cbPluginEvent for cbRightDownEvent {}
impl _wxEvent for cbRightDownEvent {}
impl _wxObject for cbRightDownEvent { fn handle(&self) -> *u8 { **self } }

impl cbRightDownEvent {
    pub fn from(handle: *u8) -> @_cbRightDownEvent {
        @cbRightDownEvent(handle) as @_cbRightDownEvent
    }
    
}

trait _cbRightDownEvent : _cbPluginEvent {
}

struct cbRightUpEvent(*u8);
impl _cbRightUpEvent for cbRightUpEvent {}
impl _cbPluginEvent for cbRightUpEvent {}
impl _wxEvent for cbRightUpEvent {}
impl _wxObject for cbRightUpEvent { fn handle(&self) -> *u8 { **self } }

impl cbRightUpEvent {
    pub fn from(handle: *u8) -> @_cbRightUpEvent {
        @cbRightUpEvent(handle) as @_cbRightUpEvent
    }
    
}

trait _cbRightUpEvent : _cbPluginEvent {
}

struct cbRowDragPlugin(*u8);
impl _cbRowDragPlugin for cbRowDragPlugin {}
impl _cbPluginBase for cbRowDragPlugin {}
impl _wxEvtHandler for cbRowDragPlugin {}
impl _wxObject for cbRowDragPlugin { fn handle(&self) -> *u8 { **self } }

impl cbRowDragPlugin {
    pub fn from(handle: *u8) -> @_cbRowDragPlugin {
        @cbRowDragPlugin(handle) as @_cbRowDragPlugin
    }
    
}

trait _cbRowDragPlugin : _cbPluginBase {
}

struct cbRowInfo(*u8);
impl _cbRowInfo for cbRowInfo {}
impl _wxObject for cbRowInfo { fn handle(&self) -> *u8 { **self } }

impl cbRowInfo {
    pub fn from(handle: *u8) -> @_cbRowInfo {
        @cbRowInfo(handle) as @_cbRowInfo
    }
    
}

trait _cbRowInfo : _wxObject {
}

struct cbRowLayoutPlugin(*u8);
impl _cbRowLayoutPlugin for cbRowLayoutPlugin {}
impl _cbPluginBase for cbRowLayoutPlugin {}
impl _wxEvtHandler for cbRowLayoutPlugin {}
impl _wxObject for cbRowLayoutPlugin { fn handle(&self) -> *u8 { **self } }

impl cbRowLayoutPlugin {
    pub fn from(handle: *u8) -> @_cbRowLayoutPlugin {
        @cbRowLayoutPlugin(handle) as @_cbRowLayoutPlugin
    }
    
}

trait _cbRowLayoutPlugin : _cbPluginBase {
}

struct cbSimpleCustomizationPlugin(*u8);
impl _cbSimpleCustomizationPlugin for cbSimpleCustomizationPlugin {}
impl _cbPluginBase for cbSimpleCustomizationPlugin {}
impl _wxEvtHandler for cbSimpleCustomizationPlugin {}
impl _wxObject for cbSimpleCustomizationPlugin { fn handle(&self) -> *u8 { **self } }

impl cbSimpleCustomizationPlugin {
    pub fn from(handle: *u8) -> @_cbSimpleCustomizationPlugin {
        @cbSimpleCustomizationPlugin(handle) as @_cbSimpleCustomizationPlugin
    }
    
}

trait _cbSimpleCustomizationPlugin : _cbPluginBase {
}

struct cbSimpleUpdatesMgr(*u8);
impl _cbSimpleUpdatesMgr for cbSimpleUpdatesMgr {}
impl _cbUpdatesManagerBase for cbSimpleUpdatesMgr {}
impl _wxObject for cbSimpleUpdatesMgr { fn handle(&self) -> *u8 { **self } }

impl cbSimpleUpdatesMgr {
    pub fn from(handle: *u8) -> @_cbSimpleUpdatesMgr {
        @cbSimpleUpdatesMgr(handle) as @_cbSimpleUpdatesMgr
    }
    
}

trait _cbSimpleUpdatesMgr : _cbUpdatesManagerBase {
}

struct cbSizeBarWndEvent(*u8);
impl _cbSizeBarWndEvent for cbSizeBarWndEvent {}
impl _cbPluginEvent for cbSizeBarWndEvent {}
impl _wxEvent for cbSizeBarWndEvent {}
impl _wxObject for cbSizeBarWndEvent { fn handle(&self) -> *u8 { **self } }

impl cbSizeBarWndEvent {
    pub fn from(handle: *u8) -> @_cbSizeBarWndEvent {
        @cbSizeBarWndEvent(handle) as @_cbSizeBarWndEvent
    }
    
}

trait _cbSizeBarWndEvent : _cbPluginEvent {
}

struct cbStartBarDraggingEvent(*u8);
impl _cbStartBarDraggingEvent for cbStartBarDraggingEvent {}
impl _cbPluginEvent for cbStartBarDraggingEvent {}
impl _wxEvent for cbStartBarDraggingEvent {}
impl _wxObject for cbStartBarDraggingEvent { fn handle(&self) -> *u8 { **self } }

impl cbStartBarDraggingEvent {
    pub fn from(handle: *u8) -> @_cbStartBarDraggingEvent {
        @cbStartBarDraggingEvent(handle) as @_cbStartBarDraggingEvent
    }
    
}

trait _cbStartBarDraggingEvent : _cbPluginEvent {
}

struct cbStartDrawInAreaEvent(*u8);
impl _cbStartDrawInAreaEvent for cbStartDrawInAreaEvent {}
impl _cbPluginEvent for cbStartDrawInAreaEvent {}
impl _wxEvent for cbStartDrawInAreaEvent {}
impl _wxObject for cbStartDrawInAreaEvent { fn handle(&self) -> *u8 { **self } }

impl cbStartDrawInAreaEvent {
    pub fn from(handle: *u8) -> @_cbStartDrawInAreaEvent {
        @cbStartDrawInAreaEvent(handle) as @_cbStartDrawInAreaEvent
    }
    
}

trait _cbStartDrawInAreaEvent : _cbPluginEvent {
}

struct cbUpdatesManagerBase(*u8);
impl _cbUpdatesManagerBase for cbUpdatesManagerBase {}
impl _wxObject for cbUpdatesManagerBase { fn handle(&self) -> *u8 { **self } }

impl cbUpdatesManagerBase {
    pub fn from(handle: *u8) -> @_cbUpdatesManagerBase {
        @cbUpdatesManagerBase(handle) as @_cbUpdatesManagerBase
    }
    
}

trait _cbUpdatesManagerBase : _wxObject {
}

struct wxAcceleratorEntry(*u8);
impl _wxAcceleratorEntry for wxAcceleratorEntry { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorEntry {
    pub fn from(handle: *u8) -> @_wxAcceleratorEntry {
        @wxAcceleratorEntry(handle) as @_wxAcceleratorEntry
    }
    
    #[fixed_stack_segment]
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> @_wxAcceleratorEntry {
        unsafe { @wxAcceleratorEntry(wxAcceleratorEntry_Create(flags, keyCode, cmd)) as @_wxAcceleratorEntry }
    }
}

trait _wxAcceleratorEntry {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAcceleratorEntry_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCommand(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetCommand(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getKeyCode(&self) -> c_int {
        unsafe { wxAcceleratorEntry_GetKeyCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, flags: c_int, keyCode: c_int, cmd: c_int) {
        unsafe { wxAcceleratorEntry_Set(self.handle(), flags, keyCode, cmd) }
    }
}

struct wxAcceleratorTable(*u8);
impl _wxAcceleratorTable for wxAcceleratorTable { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorTable {
    pub fn from(handle: *u8) -> @_wxAcceleratorTable {
        @wxAcceleratorTable(handle) as @_wxAcceleratorTable
    }
    
    #[fixed_stack_segment]
    pub fn new(n: c_int, entries: *u8) -> @_wxAcceleratorTable {
        unsafe { @wxAcceleratorTable(wxAcceleratorTable_Create(n, entries)) as @_wxAcceleratorTable }
    }
}

trait _wxAcceleratorTable {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.handle()) }
    }
}

struct wxActivateEvent(*u8);
impl _wxActivateEvent for wxActivateEvent {}
impl _wxEvent for wxActivateEvent {}
impl _wxObject for wxActivateEvent { fn handle(&self) -> *u8 { **self } }

impl wxActivateEvent {
    pub fn from(handle: *u8) -> @_wxActivateEvent {
        @wxActivateEvent(handle) as @_wxActivateEvent
    }
    
}

trait _wxActivateEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxActivateEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getActive(&self) -> bool {
        unsafe { wxActivateEvent_GetActive(self.handle()) }
    }
}

struct wxApp(*u8);
impl _wxApp for wxApp {}
impl _wxEvtHandler for wxApp {}
impl _wxObject for wxApp { fn handle(&self) -> *u8 { **self } }

impl wxApp {
    pub fn from(handle: *u8) -> @_wxApp {
        @wxApp(handle) as @_wxApp
    }
    
}

trait _wxApp : _wxEvtHandler {
}

struct wxArray(*u8);
impl _wxArray for wxArray { fn handle(&self) -> *u8 { **self } }

impl wxArray {
    pub fn from(handle: *u8) -> @_wxArray {
        @wxArray(handle) as @_wxArray
    }
    
}

trait _wxArray {
    fn handle(&self) -> *u8;
    
}

struct wxArrayString(*u8);
impl _wxArrayString for wxArrayString {}
impl _wxArray for wxArrayString { fn handle(&self) -> *u8 { **self } }

impl wxArrayString {
    pub fn from(handle: *u8) -> @_wxArrayString {
        @wxArrayString(handle) as @_wxArrayString
    }
    
}

trait _wxArrayString : _wxArray {
}

struct wxArtProvider(*u8);
impl _wxArtProvider for wxArtProvider {}
impl _wxObject for wxArtProvider { fn handle(&self) -> *u8 { **self } }

impl wxArtProvider {
    pub fn from(handle: *u8) -> @_wxArtProvider {
        @wxArtProvider(handle) as @_wxArtProvider
    }
    
}

trait _wxArtProvider : _wxObject {
}

struct wxAutoBufferedPaintDC(*u8);
impl _wxAutoBufferedPaintDC for wxAutoBufferedPaintDC {}
impl _wxDC for wxAutoBufferedPaintDC {}
impl _wxObject for wxAutoBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxAutoBufferedPaintDC {
    pub fn from(handle: *u8) -> @_wxAutoBufferedPaintDC {
        @wxAutoBufferedPaintDC(handle) as @_wxAutoBufferedPaintDC
    }
    
    #[fixed_stack_segment]
    pub fn new(window: @_wxWindow) -> @_wxAutoBufferedPaintDC {
        unsafe { @wxAutoBufferedPaintDC(wxAutoBufferedPaintDC_Create(window.handle())) as @_wxAutoBufferedPaintDC }
    }
}

trait _wxAutoBufferedPaintDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAutoBufferedPaintDC_Delete(self.handle()) }
    }
}

struct wxAutomationObject(*u8);
impl _wxAutomationObject for wxAutomationObject {}
impl _wxObject for wxAutomationObject { fn handle(&self) -> *u8 { **self } }

impl wxAutomationObject {
    pub fn from(handle: *u8) -> @_wxAutomationObject {
        @wxAutomationObject(handle) as @_wxAutomationObject
    }
    
}

trait _wxAutomationObject : _wxObject {
}

struct wxBitmap(*u8);
impl _wxBitmap for wxBitmap {}
impl _wxGDIObject for wxBitmap {}
impl _wxObject for wxBitmap { fn handle(&self) -> *u8 { **self } }

impl wxBitmap {
    pub fn from(handle: *u8) -> @_wxBitmap {
        @wxBitmap(handle) as @_wxBitmap
    }
    
    #[fixed_stack_segment]
    pub fn addHandler(handler: @_wxEvtHandler) {
        unsafe { wxBitmap_AddHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    pub fn new(_data: *u8, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> @_wxBitmap {
        unsafe { @wxBitmap(wxBitmap_Create(_data, _type, _width, _height, _depth)) as @_wxBitmap }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateDefault()) as @_wxBitmap }
    }
    #[fixed_stack_segment]
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> @_wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateEmpty(_width, _height, _depth)) as @_wxBitmap }
    }
    #[fixed_stack_segment]
    pub fn newLoad(name: @_wxString, type_: c_int) -> @_wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateLoad(name.handle(), type_)) as @_wxBitmap }
    }
    #[fixed_stack_segment]
    pub fn findHandlerByName(name: @_wxString) -> *u8 {
        unsafe { wxBitmap_FindHandlerByName(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn findHandlerByType(type_: c_int) -> *u8 {
        unsafe { wxBitmap_FindHandlerByType(type_) }
    }
    #[fixed_stack_segment]
    pub fn initStandardHandlers() {
        unsafe { wxBitmap_InitStandardHandlers() }
    }
    #[fixed_stack_segment]
    pub fn insertHandler(handler: @_wxEvtHandler) {
        unsafe { wxBitmap_InsertHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    pub fn removeHandler(name: @_wxString) -> bool {
        unsafe { wxBitmap_RemoveHandler(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newFromImage(image: @_wxImage, depth: c_int) -> @_wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromImage(image.handle(), depth)) as @_wxBitmap }
    }
}

trait _wxBitmap : _wxGDIObject {
    #[fixed_stack_segment]
    fn newFromXPM(&self) -> @_wxBitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromXPM(self.handle())) as @_wxBitmap }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBitmap_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn findHandlerByExtension(&self, type_: c_int) -> *u8 {
        unsafe { wxBitmap_FindHandlerByExtension(self.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn getDepth(&self) -> c_int {
        unsafe { wxBitmap_GetDepth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> c_int {
        unsafe { wxBitmap_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> @_wxMask {
        unsafe { @wxMask(wxBitmap_GetMask(self.handle())) as @_wxMask }
    }
    #[fixed_stack_segment]
    fn getSubBitmap(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: @_wxBitmap) {
        unsafe { wxBitmap_GetSubBitmap(self.handle(), x, y, w, h, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, name: @_wxString, type_: c_int) -> c_int {
        unsafe { wxBitmap_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBitmap_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @_wxString, type_: c_int, cmap: @_wxPalette) -> c_int {
        unsafe { wxBitmap_SaveFile(self.handle(), name.handle(), type_, cmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setDepth(&self, d: c_int) {
        unsafe { wxBitmap_SetDepth(self.handle(), d) }
    }
    #[fixed_stack_segment]
    fn setHeight(&self, h: c_int) {
        unsafe { wxBitmap_SetHeight(self.handle(), h) }
    }
    #[fixed_stack_segment]
    fn setMask(&self, mask: @_wxMask) {
        unsafe { wxBitmap_SetMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, w: c_int) {
        unsafe { wxBitmap_SetWidth(self.handle(), w) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxBitmap_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxBitmapButton {
        @wxBitmapButton(handle) as @_wxBitmapButton
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _bmp: @_wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxBitmapButton {
        unsafe { @wxBitmapButton(wxBitmapButton_Create(_prt.handle(), _id, _bmp.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxBitmapButton }
    }
}

trait _wxBitmapButton : _wxButton {
    #[fixed_stack_segment]
    fn getBitmapDisabled(&self, _ref: @_wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapFocus(&self, _ref: @_wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapLabel(&self, _ref: @_wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapSelected(&self, _ref: @_wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapSelected(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getMarginX(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMarginY(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapDisabled(&self, disabled: @_wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.handle(), disabled.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapFocus(&self, focus: @_wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.handle(), focus.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel(&self, bitmap: @_wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapSelected(&self, sel: @_wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.handle(), sel.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxBitmapToggleButton {
        @wxBitmapToggleButton(handle) as @_wxBitmapToggleButton
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, id: c_int, _bmp: @_wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @_wxBitmapToggleButton {
        unsafe { @wxBitmapToggleButton(wxBitmapToggleButton_Create(parent.handle(), id, _bmp.handle(), x, y, w, h, style)) as @_wxBitmapToggleButton }
    }
}

trait _wxBitmapToggleButton : _wxToggleButton {
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> bool {
        unsafe { wxBitmapToggleButton_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxBitmapToggleButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, state: bool) {
        unsafe { wxBitmapToggleButton_SetValue(self.handle(), state) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel(&self, _bmp: @_wxBitmap) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.handle(), _bmp.handle()) }
    }
}

struct wxBitmapDataObject(*u8);
impl _wxBitmapDataObject for wxBitmapDataObject {}
impl _wxDataObjectSimple for wxBitmapDataObject {}
impl _wxDataObject for wxBitmapDataObject { fn handle(&self) -> *u8 { **self } }

impl wxBitmapDataObject {
    pub fn from(handle: *u8) -> @_wxBitmapDataObject {
        @wxBitmapDataObject(handle) as @_wxBitmapDataObject
    }
    
}

trait _wxBitmapDataObject : _wxDataObjectSimple {
}

struct wxBitmapHandler(*u8);
impl _wxBitmapHandler for wxBitmapHandler {}
impl _wxObject for wxBitmapHandler { fn handle(&self) -> *u8 { **self } }

impl wxBitmapHandler {
    pub fn from(handle: *u8) -> @_wxBitmapHandler {
        @wxBitmapHandler(handle) as @_wxBitmapHandler
    }
    
}

trait _wxBitmapHandler : _wxObject {
}

struct wxBoxSizer(*u8);
impl _wxBoxSizer for wxBoxSizer {}
impl _wxSizer for wxBoxSizer {}
impl _wxObject for wxBoxSizer { fn handle(&self) -> *u8 { **self } }

impl wxBoxSizer {
    pub fn from(handle: *u8) -> @_wxBoxSizer {
        @wxBoxSizer(handle) as @_wxBoxSizer
    }
    
    #[fixed_stack_segment]
    pub fn new(orient: c_int) -> @_wxBoxSizer {
        unsafe { @wxBoxSizer(wxBoxSizer_Create(orient)) as @_wxBoxSizer }
    }
}

trait _wxBoxSizer : _wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @_wxSize {
        unsafe { @wxSize(wxBoxSizer_CalcMin(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxBoxSizer_RecalcSizes(self.handle()) }
    }
}

struct wxBrush(*u8);
impl _wxBrush for wxBrush {}
impl _wxGDIObject for wxBrush {}
impl _wxObject for wxBrush { fn handle(&self) -> *u8 { **self } }

impl wxBrush {
    pub fn from(handle: *u8) -> @_wxBrush {
        @wxBrush(handle) as @_wxBrush
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxBrush {
        unsafe { @wxBrush(wxBrush_CreateDefault()) as @_wxBrush }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(bitmap: @_wxBitmap) -> @_wxBrush {
        unsafe { @wxBrush(wxBrush_CreateFromBitmap(bitmap.handle())) as @_wxBrush }
    }
    #[fixed_stack_segment]
    pub fn newFromColour(col: @_wxColour, style: c_int) -> @_wxBrush {
        unsafe { @wxBrush(wxBrush_CreateFromColour(col.handle(), style)) as @_wxBrush }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @_wxBrush {
        unsafe { @wxBrush(wxBrush_CreateFromStock(id)) as @_wxBrush }
    }
}

trait _wxBrush : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, brush: @_wxBrush) {
        unsafe { wxBrush_Assign(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBrush_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @_wxColour) {
        unsafe { wxBrush_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStipple(&self, _ref: @_wxBitmap) {
        unsafe { wxBrush_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, brush: @_wxBrush) -> bool {
        unsafe { wxBrush_IsEqual(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBrush_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, col: @_wxColour) {
        unsafe { wxBrush_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxBrush_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    fn setStipple(&self, stipple: @_wxBitmap) {
        unsafe { wxBrush_SetStipple(self.handle(), stipple.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxBrush_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxBrush_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxBrush_IsStatic(self.handle()) }
    }
}

struct wxBrushList(*u8);
impl _wxBrushList for wxBrushList {}
impl _wxList for wxBrushList {}
impl _wxObject for wxBrushList { fn handle(&self) -> *u8 { **self } }

impl wxBrushList {
    pub fn from(handle: *u8) -> @_wxBrushList {
        @wxBrushList(handle) as @_wxBrushList
    }
    
}

trait _wxBrushList : _wxList {
}

struct wxBufferedDC(*u8);
impl _wxBufferedDC for wxBufferedDC {}
impl _wxDC for wxBufferedDC {}
impl _wxObject for wxBufferedDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedDC {
    pub fn from(handle: *u8) -> @_wxBufferedDC {
        @wxBufferedDC(handle) as @_wxBufferedDC
    }
    
    #[fixed_stack_segment]
    pub fn newByDCAndSize(dc: @_wxDC, width: c_int, hight: c_int, style: c_int) -> @_wxBufferedDC {
        unsafe { @wxBufferedDC(wxBufferedDC_CreateByDCAndSize(dc.handle(), width, hight, style)) as @_wxBufferedDC }
    }
    #[fixed_stack_segment]
    pub fn newByDCAndBitmap(dc: @_wxDC, bitmap: @_wxBitmap, style: c_int) -> @_wxBufferedDC {
        unsafe { @wxBufferedDC(wxBufferedDC_CreateByDCAndBitmap(dc.handle(), bitmap.handle(), style)) as @_wxBufferedDC }
    }
}

trait _wxBufferedDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedDC_Delete(self.handle()) }
    }
}

struct wxBufferedPaintDC(*u8);
impl _wxBufferedPaintDC for wxBufferedPaintDC {}
impl _wxDC for wxBufferedPaintDC {}
impl _wxObject for wxBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedPaintDC {
    pub fn from(handle: *u8) -> @_wxBufferedPaintDC {
        @wxBufferedPaintDC(handle) as @_wxBufferedPaintDC
    }
    
    #[fixed_stack_segment]
    pub fn new(window: @_wxWindow, style: c_int) -> @_wxBufferedPaintDC {
        unsafe { @wxBufferedPaintDC(wxBufferedPaintDC_Create(window.handle(), style)) as @_wxBufferedPaintDC }
    }
    #[fixed_stack_segment]
    pub fn newWithBitmap(window: @_wxWindow, bitmap: @_wxBitmap, style: c_int) -> @_wxBufferedPaintDC {
        unsafe { @wxBufferedPaintDC(wxBufferedPaintDC_CreateWithBitmap(window.handle(), bitmap.handle(), style)) as @_wxBufferedPaintDC }
    }
}

trait _wxBufferedPaintDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedPaintDC_Delete(self.handle()) }
    }
}

struct wxBufferedInputStream(*u8);
impl _wxBufferedInputStream for wxBufferedInputStream {}
impl _wxFilterInputStream for wxBufferedInputStream {}
impl _wxInputStream for wxBufferedInputStream {}
impl _wxStreamBase for wxBufferedInputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedInputStream {
    pub fn from(handle: *u8) -> @_wxBufferedInputStream {
        @wxBufferedInputStream(handle) as @_wxBufferedInputStream
    }
    
}

trait _wxBufferedInputStream : _wxFilterInputStream {
}

struct wxBufferedOutputStream(*u8);
impl _wxBufferedOutputStream for wxBufferedOutputStream {}
impl _wxFilterOutputStream for wxBufferedOutputStream {}
impl _wxOutputStream for wxBufferedOutputStream {}
impl _wxStreamBase for wxBufferedOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedOutputStream {
    pub fn from(handle: *u8) -> @_wxBufferedOutputStream {
        @wxBufferedOutputStream(handle) as @_wxBufferedOutputStream
    }
    
}

trait _wxBufferedOutputStream : _wxFilterOutputStream {
}

struct wxBusyCursor(*u8);
impl _wxBusyCursor for wxBusyCursor { fn handle(&self) -> *u8 { **self } }

impl wxBusyCursor {
    pub fn from(handle: *u8) -> @_wxBusyCursor {
        @wxBusyCursor(handle) as @_wxBusyCursor
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxBusyCursor {
        unsafe { @wxBusyCursor(wxBusyCursor_Create()) as @_wxBusyCursor }
    }
}

trait _wxBusyCursor {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn newWithCursor(&self) -> *u8 {
        unsafe { wxBusyCursor_CreateWithCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.handle()) }
    }
}

struct wxBusyInfo(*u8);
impl _wxBusyInfo for wxBusyInfo { fn handle(&self) -> *u8 { **self } }

impl wxBusyInfo {
    pub fn from(handle: *u8) -> @_wxBusyInfo {
        @wxBusyInfo(handle) as @_wxBusyInfo
    }
    
    #[fixed_stack_segment]
    pub fn new(_txt: @_wxString) -> @_wxBusyInfo {
        unsafe { @wxBusyInfo(wxBusyInfo_Create(_txt.handle())) as @_wxBusyInfo }
    }
}

trait _wxBusyInfo {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxButton {
        @wxButton(handle) as @_wxButton
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxButton {
        unsafe { @wxButton(wxButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxButton }
    }
}

trait _wxButton : _wxControl {
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @_wxColour) -> c_int {
        unsafe { wxButton_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.handle()) }
    }
}

struct wxCSConv(*u8);
impl _wxCSConv for wxCSConv {}
impl _wxMBConv for wxCSConv { fn handle(&self) -> *u8 { **self } }

impl wxCSConv {
    pub fn from(handle: *u8) -> @_wxCSConv {
        @wxCSConv(handle) as @_wxCSConv
    }
    
}

trait _wxCSConv : _wxMBConv {
}

struct wxCalculateLayoutEvent(*u8);
impl _wxCalculateLayoutEvent for wxCalculateLayoutEvent {}
impl _wxEvent for wxCalculateLayoutEvent {}
impl _wxObject for wxCalculateLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalculateLayoutEvent {
    pub fn from(handle: *u8) -> @_wxCalculateLayoutEvent {
        @wxCalculateLayoutEvent(handle) as @_wxCalculateLayoutEvent
    }
    
    #[fixed_stack_segment]
    pub fn new(id: c_int) -> @_wxCalculateLayoutEvent {
        unsafe { @wxCalculateLayoutEvent(wxCalculateLayoutEvent_Create(id)) as @_wxCalculateLayoutEvent }
    }
}

trait _wxCalculateLayoutEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @_wxRect {
        unsafe { @wxRect(wxCalculateLayoutEvent_GetRect(self.handle())) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxCalendarCtrl {
        @wxCalendarCtrl(handle) as @_wxCalendarCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _dat: @_wxDateTime, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxCalendarCtrl {
        unsafe { @wxCalendarCtrl(wxCalendarCtrl_Create(_prt.handle(), _id, _dat.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxCalendarCtrl }
    }
}

trait _wxCalendarCtrl : _wxControl {
    #[fixed_stack_segment]
    fn enableHolidayDisplay(&self, display: c_int) {
        unsafe { wxCalendarCtrl_EnableHolidayDisplay(self.handle(), display) }
    }
    #[fixed_stack_segment]
    fn enableMonthChange(&self, enable: bool) {
        unsafe { wxCalendarCtrl_EnableMonthChange(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getAttr(&self, day: c_int) -> *u8 {
        unsafe { wxCalendarCtrl_GetAttr(self.handle(), day) }
    }
    #[fixed_stack_segment]
    fn getDate(&self, date: *u8) {
        unsafe { wxCalendarCtrl_GetDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourBg(&self, _ref: @_wxColour) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourFg(&self, _ref: @_wxColour) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourBg(&self, _ref: @_wxColour) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourFg(&self, _ref: @_wxColour) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourBg(&self, _ref: @_wxColour) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourFg(&self, _ref: @_wxColour) {
        unsafe { wxCalendarCtrl_GetHolidayColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int, date: *u8, wd: *u8) -> c_int {
        unsafe { wxCalendarCtrl_HitTest(self.handle(), x, y, date, wd) }
    }
    #[fixed_stack_segment]
    fn resetAttr(&self, day: c_int) {
        unsafe { wxCalendarCtrl_ResetAttr(self.handle(), day) }
    }
    #[fixed_stack_segment]
    fn setAttr(&self, day: c_int, attr: *u8) {
        unsafe { wxCalendarCtrl_SetAttr(self.handle(), day, attr) }
    }
    #[fixed_stack_segment]
    fn setDate(&self, date: *u8) {
        unsafe { wxCalendarCtrl_SetDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    fn setHeaderColours(&self, colFg: *u8, colBg: *u8) {
        unsafe { wxCalendarCtrl_SetHeaderColours(self.handle(), colFg, colBg) }
    }
    #[fixed_stack_segment]
    fn setHighlightColours(&self, colFg: *u8, colBg: *u8) {
        unsafe { wxCalendarCtrl_SetHighlightColours(self.handle(), colFg, colBg) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, day: c_int) {
        unsafe { wxCalendarCtrl_SetHoliday(self.handle(), day) }
    }
    #[fixed_stack_segment]
    fn setHolidayColours(&self, colFg: *u8, colBg: *u8) {
        unsafe { wxCalendarCtrl_SetHolidayColours(self.handle(), colFg, colBg) }
    }
}

struct wxCalendarDateAttr(*u8);
impl _wxCalendarDateAttr for wxCalendarDateAttr { fn handle(&self) -> *u8 { **self } }

impl wxCalendarDateAttr {
    pub fn from(handle: *u8) -> @_wxCalendarDateAttr {
        @wxCalendarDateAttr(handle) as @_wxCalendarDateAttr
    }
    
    #[fixed_stack_segment]
    pub fn new(_ctxt: *u8, _cbck: *u8, _cbrd: *u8, _fnt: *u8, _brd: c_int) -> @_wxCalendarDateAttr {
        unsafe { @wxCalendarDateAttr(wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)) as @_wxCalendarDateAttr }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxCalendarDateAttr {
        unsafe { @wxCalendarDateAttr(wxCalendarDateAttr_CreateDefault()) as @_wxCalendarDateAttr }
    }
}

trait _wxCalendarDateAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @_wxColour) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorderColour(&self, _ref: @_wxColour) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @_wxFont) {
        unsafe { wxCalendarDateAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @_wxColour) {
        unsafe { wxCalendarDateAttr_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasBorder(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasBorderColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBorderColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasFont(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasTextColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isHoliday(&self) -> bool {
        unsafe { wxCalendarDateAttr_IsHoliday(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, col: @_wxColour) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    fn setBorderColour(&self, col: @_wxColour) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont) {
        unsafe { wxCalendarDateAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.handle(), holiday) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @_wxColour) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.handle(), col.handle()) }
    }
}

struct wxCalendarEvent(*u8);
impl _wxCalendarEvent for wxCalendarEvent {}
impl _wxCommandEvent for wxCalendarEvent {}
impl _wxEvent for wxCalendarEvent {}
impl _wxObject for wxCalendarEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalendarEvent {
    pub fn from(handle: *u8) -> @_wxCalendarEvent {
        @wxCalendarEvent(handle) as @_wxCalendarEvent
    }
    
}

trait _wxCalendarEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getDate(&self, _dte: *u8) {
        unsafe { wxCalendarEvent_GetDate(self.handle(), _dte) }
    }
    #[fixed_stack_segment]
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.handle()) }
    }
}

struct wxCaret(*u8);
impl _wxCaret for wxCaret { fn handle(&self) -> *u8 { **self } }

impl wxCaret {
    pub fn from(handle: *u8) -> @_wxCaret {
        @wxCaret(handle) as @_wxCaret
    }
    
    #[fixed_stack_segment]
    pub fn new(_wnd: @_wxWindow, _wth: c_int, _hgt: c_int) -> @_wxCaret {
        unsafe { @wxCaret(wxCaret_Create(_wnd.handle(), _wth, _hgt)) as @_wxCaret }
    }
    #[fixed_stack_segment]
    pub fn getBlinkTime() -> c_int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    #[fixed_stack_segment]
    pub fn setBlinkTime(milliseconds: c_int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
    }
}

trait _wxCaret {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxCaret_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxCaret_GetSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxCaret_GetWindow(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn hide(&self) {
        unsafe { wxCaret_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxCaret_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self) -> bool {
        unsafe { wxCaret_IsVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxCaret_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, width: c_int, height: c_int) {
        unsafe { wxCaret_SetSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxCheckBox {
        @wxCheckBox(handle) as @_wxCheckBox
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxCheckBox {
        unsafe { @wxCheckBox(wxCheckBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxCheckBox }
    }
}

trait _wxCheckBox : _wxControl {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCheckBox_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxCheckBox_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxCheckListBox {
        @wxCheckListBox(handle) as @_wxCheckListBox
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @_wxCheckListBox {
        unsafe { @wxCheckListBox(wxCheckListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @_wxCheckListBox }
    }
}

trait _wxCheckListBox : _wxListBox {
    #[fixed_stack_segment]
    fn check(&self, item: c_int, check: bool) {
        unsafe { wxCheckListBox_Check(self.handle(), item, check) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCheckListBox_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxChoice {
        @wxChoice(handle) as @_wxChoice
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @_wxChoice {
        unsafe { @wxChoice(wxChoice_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @_wxChoice }
    }
}

trait _wxChoice : _wxControl {
    #[fixed_stack_segment]
    fn append(&self, item: @_wxString) {
        unsafe { wxChoice_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxChoice_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: c_int) {
        unsafe { wxChoice_Delete(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @_wxString) -> c_int {
        unsafe { wxChoice_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxChoice_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> c_int {
        unsafe { wxChoice_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getString(&self, n: c_int) -> @_wxString {
        unsafe { @wxString(wxChoice_GetString(self.handle(), n)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setString(&self, n: c_int, s: @_wxString) {
        unsafe { wxChoice_SetString(self.handle(), n, s.handle()) }
    }
}

struct wxClassInfo(*u8);
impl _wxClassInfo for wxClassInfo { fn handle(&self) -> *u8 { **self } }

impl wxClassInfo {
    pub fn from(handle: *u8) -> @_wxClassInfo {
        @wxClassInfo(handle) as @_wxClassInfo
    }
    
    #[fixed_stack_segment]
    pub fn findClass(_txt: @_wxString) -> @_wxClassInfo {
        unsafe { @wxClassInfo(wxClassInfo_FindClass(_txt.handle())) as @_wxClassInfo }
    }
}

trait _wxClassInfo {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn newClassByName(&self) -> *u8 {
        unsafe { wxClassInfo_CreateClassByName(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClassName(&self) -> *u8 {
        unsafe { wxClassInfo_GetClassName(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isKindOf(&self, _name: @_wxString) -> bool {
        unsafe { wxClassInfo_IsKindOf(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn getBaseClassName1(&self) -> @_wxString {
        unsafe { @wxString(wxClassInfo_GetBaseClassName1(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getBaseClassName2(&self) -> @_wxString {
        unsafe { @wxString(wxClassInfo_GetBaseClassName2(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getClassNameEx(&self) -> @_wxString {
        unsafe { @wxString(wxClassInfo_GetClassNameEx(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isKindOfEx(&self, classInfo: @_wxClassInfo) -> bool {
        unsafe { wxClassInfo_IsKindOfEx(self.handle(), classInfo.handle()) }
    }
}

struct wxClient(*u8);
impl _wxClient for wxClient {}
impl _wxClientBase for wxClient {}
impl _wxObject for wxClient { fn handle(&self) -> *u8 { **self } }

impl wxClient {
    pub fn from(handle: *u8) -> @_wxClient {
        @wxClient(handle) as @_wxClient
    }
    
}

trait _wxClient : _wxClientBase {
}

struct wxClientBase(*u8);
impl _wxClientBase for wxClientBase {}
impl _wxObject for wxClientBase { fn handle(&self) -> *u8 { **self } }

impl wxClientBase {
    pub fn from(handle: *u8) -> @_wxClientBase {
        @wxClientBase(handle) as @_wxClientBase
    }
    
}

trait _wxClientBase : _wxObject {
}

struct wxClientDC(*u8);
impl _wxClientDC for wxClientDC {}
impl _wxWindowDC for wxClientDC {}
impl _wxDC for wxClientDC {}
impl _wxObject for wxClientDC { fn handle(&self) -> *u8 { **self } }

impl wxClientDC {
    pub fn from(handle: *u8) -> @_wxClientDC {
        @wxClientDC(handle) as @_wxClientDC
    }
    
    #[fixed_stack_segment]
    pub fn new(win: @_wxWindow) -> @_wxClientDC {
        unsafe { @wxClientDC(wxClientDC_Create(win.handle())) as @_wxClientDC }
    }
}

trait _wxClientDC : _wxWindowDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxClientDC_Delete(self.handle()) }
    }
}

struct wxClientData(*u8);
impl _wxClientData for wxClientData { fn handle(&self) -> *u8 { **self } }

impl wxClientData {
    pub fn from(handle: *u8) -> @_wxClientData {
        @wxClientData(handle) as @_wxClientData
    }
    
}

trait _wxClientData {
    fn handle(&self) -> *u8;
    
}

struct wxClientDataContainer(*u8);
impl _wxClientDataContainer for wxClientDataContainer { fn handle(&self) -> *u8 { **self } }

impl wxClientDataContainer {
    pub fn from(handle: *u8) -> @_wxClientDataContainer {
        @wxClientDataContainer(handle) as @_wxClientDataContainer
    }
    
}

trait _wxClientDataContainer {
    fn handle(&self) -> *u8;
    
}

struct wxClipboard(*u8);
impl _wxClipboard for wxClipboard {}
impl _wxObject for wxClipboard { fn handle(&self) -> *u8 { **self } }

impl wxClipboard {
    pub fn from(handle: *u8) -> @_wxClipboard {
        @wxClipboard(handle) as @_wxClipboard
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxClipboard {
        unsafe { @wxClipboard(wxClipboard_Create()) as @_wxClipboard }
    }
}

trait _wxClipboard : _wxObject {
    #[fixed_stack_segment]
    fn addData(&self, data: @_wxDataObject) -> bool {
        unsafe { wxClipboard_AddData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxClipboard_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn close(&self) {
        unsafe { wxClipboard_Close(self.handle()) }
    }
    #[fixed_stack_segment]
    fn flush(&self) -> bool {
        unsafe { wxClipboard_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getData(&self, data: @_wxDataObject) -> bool {
        unsafe { wxClipboard_GetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn isOpened(&self) -> bool {
        unsafe { wxClipboard_IsOpened(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSupported(&self, format: @_wxDataFormat) -> bool {
        unsafe { wxClipboard_IsSupported(self.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn open(&self) -> bool {
        unsafe { wxClipboard_Open(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @_wxDataObject) -> bool {
        unsafe { wxClipboard_SetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn usePrimarySelection(&self, primary: bool) {
        unsafe { wxClipboard_UsePrimarySelection(self.handle(), primary) }
    }
}

struct wxCloseEvent(*u8);
impl _wxCloseEvent for wxCloseEvent {}
impl _wxEvent for wxCloseEvent {}
impl _wxObject for wxCloseEvent { fn handle(&self) -> *u8 { **self } }

impl wxCloseEvent {
    pub fn from(handle: *u8) -> @_wxCloseEvent {
        @wxCloseEvent(handle) as @_wxCloseEvent
    }
    
}

trait _wxCloseEvent : _wxEvent {
    #[fixed_stack_segment]
    fn canVeto(&self) -> bool {
        unsafe { wxCloseEvent_CanVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @_wxObject) {
        unsafe { wxCloseEvent_CopyObject(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn getLoggingOff(&self) -> bool {
        unsafe { wxCloseEvent_GetLoggingOff(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVeto(&self) -> bool {
        unsafe { wxCloseEvent_GetVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCanVeto(&self, canVeto: bool) {
        unsafe { wxCloseEvent_SetCanVeto(self.handle(), canVeto) }
    }
    #[fixed_stack_segment]
    fn setLoggingOff(&self, logOff: bool) {
        unsafe { wxCloseEvent_SetLoggingOff(self.handle(), logOff) }
    }
    #[fixed_stack_segment]
    fn veto(&self, veto: bool) {
        unsafe { wxCloseEvent_Veto(self.handle(), veto) }
    }
}

struct wxClosure(*u8);
impl _wxClosure for wxClosure {}
impl _wxObject for wxClosure { fn handle(&self) -> *u8 { **self } }

impl wxClosure {
    pub fn from(handle: *u8) -> @_wxClosure {
        @wxClosure(handle) as @_wxClosure
    }
    
    #[fixed_stack_segment]
    pub fn new(_fun_CEvent: *u8, _data: *u8) -> @_wxClosure {
        unsafe { @wxClosure(wxClosure_Create(_fun_CEvent, _data)) as @_wxClosure }
    }
}

trait _wxClosure : _wxObject {
    #[fixed_stack_segment]
    fn getData(&self) -> *u8 {
        unsafe { wxClosure_GetData(self.handle()) }
    }
}

struct wxColour(*u8);
impl _wxColour for wxColour {}
impl _wxObject for wxColour { fn handle(&self) -> *u8 { **self } }

impl wxColour {
    pub fn from(handle: *u8) -> @_wxColour {
        @wxColour(handle) as @_wxColour
    }
    
    #[fixed_stack_segment]
    pub fn newByName(_name: @_wxString) -> @_wxColour {
        unsafe { @wxColour(wxColour_CreateByName(_name.handle())) as @_wxColour }
    }
    #[fixed_stack_segment]
    pub fn newEmpty() -> @_wxColour {
        unsafe { @wxColour(wxColour_CreateEmpty()) as @_wxColour }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @_wxColour {
        unsafe { @wxColour(wxColour_CreateFromStock(id)) as @_wxColour }
    }
    #[fixed_stack_segment]
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> @_wxColour {
        unsafe { @wxColour(wxColour_CreateRGB(_red, _green, _blue, _alpha)) as @_wxColour }
    }
    #[fixed_stack_segment]
    pub fn validName(_name: *wchar_t) -> bool {
        unsafe { wxColour_ValidName(_name) }
    }
    #[fixed_stack_segment]
    pub fn newFromInt(rgb: c_int) -> @_wxColour {
        unsafe { @wxColour(wxColour_CreateFromInt(rgb)) as @_wxColour }
    }
    #[fixed_stack_segment]
    pub fn newFromUnsignedInt(rgba: uint32_t) -> @_wxColour {
        unsafe { @wxColour(wxColour_CreateFromUnsignedInt(rgba)) as @_wxColour }
    }
}

trait _wxColour : _wxObject {
    #[fixed_stack_segment]
    fn alpha(&self) -> uint8_t {
        unsafe { wxColour_Alpha(self.handle()) }
    }
    #[fixed_stack_segment]
    fn assign(&self, other: *u8) {
        unsafe { wxColour_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn blue(&self) -> uint8_t {
        unsafe { wxColour_Blue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copy(&self, _other: *u8) {
        unsafe { wxColour_Copy(self.handle(), _other) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxColour_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn green(&self) -> uint8_t {
        unsafe { wxColour_Green(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxColour_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn red(&self) -> uint8_t {
        unsafe { wxColour_Red(self.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, _red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) {
        unsafe { wxColour_Set(self.handle(), _red, _green, _blue, _alpha) }
    }
    #[fixed_stack_segment]
    fn setByName(&self, _name: @_wxString) {
        unsafe { wxColour_SetByName(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxColour_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxColour_IsStatic(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getInt(&self) -> c_int {
        unsafe { wxColour_GetInt(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUnsignedInt(&self) -> uint32_t {
        unsafe { wxColour_GetUnsignedInt(self.handle()) }
    }
}

struct wxColourData(*u8);
impl _wxColourData for wxColourData {}
impl _wxObject for wxColourData { fn handle(&self) -> *u8 { **self } }

impl wxColourData {
    pub fn from(handle: *u8) -> @_wxColourData {
        @wxColourData(handle) as @_wxColourData
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxColourData {
        unsafe { @wxColourData(wxColourData_Create()) as @_wxColourData }
    }
}

trait _wxColourData : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxColourData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getChooseFull(&self) -> bool {
        unsafe { wxColourData_GetChooseFull(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @_wxColour) {
        unsafe { wxColourData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCustomColour(&self, i: c_int, _ref: @_wxColour) {
        unsafe { wxColourData_GetCustomColour(self.handle(), i, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setChooseFull(&self, flag: bool) {
        unsafe { wxColourData_SetChooseFull(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @_wxColour) {
        unsafe { wxColourData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCustomColour(&self, i: c_int, colour: @_wxColour) {
        unsafe { wxColourData_SetCustomColour(self.handle(), i, colour.handle()) }
    }
}

struct wxColourDatabase(*u8);
impl _wxColourDatabase for wxColourDatabase {}
impl _wxList for wxColourDatabase {}
impl _wxObject for wxColourDatabase { fn handle(&self) -> *u8 { **self } }

impl wxColourDatabase {
    pub fn from(handle: *u8) -> @_wxColourDatabase {
        @wxColourDatabase(handle) as @_wxColourDatabase
    }
    
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
    pub fn from(handle: *u8) -> @_wxColourDialog {
        @wxColourDialog(handle) as @_wxColourDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, col: @_wxColourData) -> @_wxColourDialog {
        unsafe { @wxColourDialog(wxColourDialog_Create(_prt.handle(), col.handle())) as @_wxColourDialog }
    }
}

trait _wxColourDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getColourData(&self, _ref: @_wxColourData) {
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
    pub fn from(handle: *u8) -> @_wxComboBox {
        @wxComboBox(handle) as @_wxComboBox
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @_wxComboBox {
        unsafe { @wxComboBox(wxComboBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, str, _stl)) as @_wxComboBox }
    }
}

trait _wxComboBox : _wxChoice {
    #[fixed_stack_segment]
    fn append(&self, item: @_wxString) {
        unsafe { wxComboBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @_wxString, d: *u8) {
        unsafe { wxComboBox_AppendData(self.handle(), item.handle(), d) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxComboBox_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copy(&self) {
        unsafe { wxComboBox_Copy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn cut(&self) {
        unsafe { wxComboBox_Cut(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: c_int) {
        unsafe { wxComboBox_Delete(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @_wxString) -> c_int {
        unsafe { wxComboBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> @_wxClientData {
        unsafe { @wxClientData(wxComboBox_GetClientData(self.handle(), n)) as @_wxClientData }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxComboBox_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getInsertionPoint(&self) -> c_int {
        unsafe { wxComboBox_GetInsertionPoint(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastPosition(&self) -> c_int {
        unsafe { wxComboBox_GetLastPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> c_int {
        unsafe { wxComboBox_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getString(&self, n: c_int) -> @_wxString {
        unsafe { @wxString(wxComboBox_GetString(self.handle(), n)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @_wxString {
        unsafe { @wxString(wxComboBox_GetStringSelection(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> @_wxString {
        unsafe { @wxString(wxComboBox_GetValue(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn paste(&self) {
        unsafe { wxComboBox_Paste(self.handle()) }
    }
    #[fixed_stack_segment]
    fn remove(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_Remove(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    fn replace(&self, from: c_int, to: c_int, value: @_wxString) {
        unsafe { wxComboBox_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, n: c_int, clientData: @_wxClientData) {
        unsafe { wxComboBox_SetClientData(self.handle(), n, clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditable(&self, editable: bool) {
        unsafe { wxComboBox_SetEditable(self.handle(), editable) }
    }
    #[fixed_stack_segment]
    fn setInsertionPoint(&self, pos: c_int) {
        unsafe { wxComboBox_SetInsertionPoint(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setInsertionPointEnd(&self) {
        unsafe { wxComboBox_SetInsertionPointEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: c_int) {
        unsafe { wxComboBox_SetSelection(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setTextSelection(&self, from: c_int, to: c_int) {
        unsafe { wxComboBox_SetTextSelection(self.handle(), from, to) }
    }
}

struct wxCommand(*u8);
impl _wxCommand for wxCommand {}
impl _wxObject for wxCommand { fn handle(&self) -> *u8 { **self } }

impl wxCommand {
    pub fn from(handle: *u8) -> @_wxCommand {
        @wxCommand(handle) as @_wxCommand
    }
    
}

trait _wxCommand : _wxObject {
}

struct wxCommandEvent(*u8);
impl _wxCommandEvent for wxCommandEvent {}
impl _wxEvent for wxCommandEvent {}
impl _wxObject for wxCommandEvent { fn handle(&self) -> *u8 { **self } }

impl wxCommandEvent {
    pub fn from(handle: *u8) -> @_wxCommandEvent {
        @wxCommandEvent(handle) as @_wxCommandEvent
    }
    
    #[fixed_stack_segment]
    pub fn new(_typ: c_int, _id: c_int) -> @_wxCommandEvent {
        unsafe { @wxCommandEvent(wxCommandEvent_Create(_typ, _id)) as @_wxCommandEvent }
    }
}

trait _wxCommandEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxCommandEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCommandEvent_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @_wxClientData {
        unsafe { @wxClientData(wxCommandEvent_GetClientData(self.handle())) as @_wxClientData }
    }
    #[fixed_stack_segment]
    fn getClientObject(&self) -> @_wxClientData {
        unsafe { @wxClientData(wxCommandEvent_GetClientObject(self.handle())) as @_wxClientData }
    }
    #[fixed_stack_segment]
    fn getExtraLong(&self) -> c_long {
        unsafe { wxCommandEvent_GetExtraLong(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getInt(&self) -> c_long {
        unsafe { wxCommandEvent_GetInt(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> c_int {
        unsafe { wxCommandEvent_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getString(&self) -> @_wxString {
        unsafe { @wxString(wxCommandEvent_GetString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn isChecked(&self) -> bool {
        unsafe { wxCommandEvent_IsChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSelection(&self) -> bool {
        unsafe { wxCommandEvent_IsSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, clientData: @_wxClientData) {
        unsafe { wxCommandEvent_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, clientObject: @_wxClientData) {
        unsafe { wxCommandEvent_SetClientObject(self.handle(), clientObject.handle()) }
    }
    #[fixed_stack_segment]
    fn setExtraLong(&self, extraLong: c_long) {
        unsafe { wxCommandEvent_SetExtraLong(self.handle(), extraLong) }
    }
    #[fixed_stack_segment]
    fn setInt(&self, i: c_int) {
        unsafe { wxCommandEvent_SetInt(self.handle(), i) }
    }
    #[fixed_stack_segment]
    fn setString(&self, s: @_wxString) {
        unsafe { wxCommandEvent_SetString(self.handle(), s.handle()) }
    }
}

struct wxCommandLineParser(*u8);
impl _wxCommandLineParser for wxCommandLineParser { fn handle(&self) -> *u8 { **self } }

impl wxCommandLineParser {
    pub fn from(handle: *u8) -> @_wxCommandLineParser {
        @wxCommandLineParser(handle) as @_wxCommandLineParser
    }
    
}

trait _wxCommandLineParser {
    fn handle(&self) -> *u8;
    
}

struct wxCommandProcessor(*u8);
impl _wxCommandProcessor for wxCommandProcessor {}
impl _wxObject for wxCommandProcessor { fn handle(&self) -> *u8 { **self } }

impl wxCommandProcessor {
    pub fn from(handle: *u8) -> @_wxCommandProcessor {
        @wxCommandProcessor(handle) as @_wxCommandProcessor
    }
    
}

trait _wxCommandProcessor : _wxObject {
}

struct wxCondition(*u8);
impl _wxCondition for wxCondition { fn handle(&self) -> *u8 { **self } }

impl wxCondition {
    pub fn from(handle: *u8) -> @_wxCondition {
        @wxCondition(handle) as @_wxCondition
    }
    
}

trait _wxCondition {
    fn handle(&self) -> *u8;
    
}

struct wxConfigBase(*u8);
impl _wxConfigBase for wxConfigBase { fn handle(&self) -> *u8 { **self } }

impl wxConfigBase {
    pub fn from(handle: *u8) -> @_wxConfigBase {
        @wxConfigBase(handle) as @_wxConfigBase
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Create()) as @_wxConfigBase }
    }
    #[fixed_stack_segment]
    pub fn get() -> @_wxConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Get()) as @_wxConfigBase }
    }
    #[fixed_stack_segment]
    pub fn set(self_: @_wxConfigBase) {
        unsafe { wxConfigBase_Set(self_.handle()) }
    }
}

trait _wxConfigBase {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAll(&self) -> bool {
        unsafe { wxConfigBase_DeleteAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteEntry(&self, key: @_wxString, bDeleteGroupIfEmpty: bool) -> bool {
        unsafe { wxConfigBase_DeleteEntry(self.handle(), key.handle(), bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    fn deleteGroup(&self, key: @_wxString) -> bool {
        unsafe { wxConfigBase_DeleteGroup(self.handle(), key.handle()) }
    }
    #[fixed_stack_segment]
    fn exists(&self, strName: @_wxString) -> bool {
        unsafe { wxConfigBase_Exists(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn expandEnvVars(&self, str: @_wxString) -> @_wxString {
        unsafe { @wxString(wxConfigBase_ExpandEnvVars(self.handle(), str.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn flush(&self, bCurrentOnly: bool) -> bool {
        unsafe { wxConfigBase_Flush(self.handle(), bCurrentOnly) }
    }
    #[fixed_stack_segment]
    fn getAppName(&self) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetAppName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getEntryType(&self, name: @_wxString) -> c_int {
        unsafe { wxConfigBase_GetEntryType(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstEntry(&self, lIndex: *u8) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetFirstEntry(self.handle(), lIndex)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFirstGroup(&self, lIndex: *u8) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetFirstGroup(self.handle(), lIndex)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNextEntry(&self, lIndex: *u8) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetNextEntry(self.handle(), lIndex)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNextGroup(&self, lIndex: *u8) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetNextGroup(self.handle(), lIndex)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNumberOfEntries(&self, bRecursive: bool) -> c_int {
        unsafe { wxConfigBase_GetNumberOfEntries(self.handle(), bRecursive) }
    }
    #[fixed_stack_segment]
    fn getNumberOfGroups(&self, bRecursive: bool) -> c_int {
        unsafe { wxConfigBase_GetNumberOfGroups(self.handle(), bRecursive) }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetPath(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVendorName(&self) -> @_wxString {
        unsafe { @wxString(wxConfigBase_GetVendorName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn hasEntry(&self, strName: @_wxString) -> bool {
        unsafe { wxConfigBase_HasEntry(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn hasGroup(&self, strName: @_wxString) -> bool {
        unsafe { wxConfigBase_HasGroup(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn isExpandingEnvVars(&self) -> bool {
        unsafe { wxConfigBase_IsExpandingEnvVars(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isRecordingDefaults(&self) -> bool {
        unsafe { wxConfigBase_IsRecordingDefaults(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readBool(&self, key: @_wxString, defVal: bool) -> bool {
        unsafe { wxConfigBase_ReadBool(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readDouble(&self, key: @_wxString, defVal: c_double) -> c_double {
        unsafe { wxConfigBase_ReadDouble(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readInteger(&self, key: @_wxString, defVal: c_int) -> c_int {
        unsafe { wxConfigBase_ReadInteger(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readString(&self, key: @_wxString, defVal: @_wxString) -> @_wxString {
        unsafe { @wxString(wxConfigBase_ReadString(self.handle(), key.handle(), defVal.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn renameEntry(&self, oldName: @_wxString, newName: @_wxString) -> bool {
        unsafe { wxConfigBase_RenameEntry(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn renameGroup(&self, oldName: @_wxString, newName: @_wxString) -> bool {
        unsafe { wxConfigBase_RenameGroup(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn setAppName(&self, appName: @_wxString) {
        unsafe { wxConfigBase_SetAppName(self.handle(), appName.handle()) }
    }
    #[fixed_stack_segment]
    fn setExpandEnvVars(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, strPath: @_wxString) {
        unsafe { wxConfigBase_SetPath(self.handle(), strPath.handle()) }
    }
    #[fixed_stack_segment]
    fn setRecordDefaults(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetRecordDefaults(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxConfigBase_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setVendorName(&self, vendorName: @_wxString) {
        unsafe { wxConfigBase_SetVendorName(self.handle(), vendorName.handle()) }
    }
    #[fixed_stack_segment]
    fn writeBool(&self, key: @_wxString, value: bool) -> bool {
        unsafe { wxConfigBase_WriteBool(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeDouble(&self, key: @_wxString, value: c_double) -> bool {
        unsafe { wxConfigBase_WriteDouble(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeInteger(&self, key: @_wxString, value: c_int) -> bool {
        unsafe { wxConfigBase_WriteInteger(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeLong(&self, key: @_wxString, value: c_long) -> bool {
        unsafe { wxConfigBase_WriteLong(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, key: @_wxString, value: @_wxString) -> bool {
        unsafe { wxConfigBase_WriteString(self.handle(), key.handle(), value.handle()) }
    }
}

struct wxConnection(*u8);
impl _wxConnection for wxConnection {}
impl _wxConnectionBase for wxConnection {}
impl _wxObject for wxConnection { fn handle(&self) -> *u8 { **self } }

impl wxConnection {
    pub fn from(handle: *u8) -> @_wxConnection {
        @wxConnection(handle) as @_wxConnection
    }
    
}

trait _wxConnection : _wxConnectionBase {
}

struct wxConnectionBase(*u8);
impl _wxConnectionBase for wxConnectionBase {}
impl _wxObject for wxConnectionBase { fn handle(&self) -> *u8 { **self } }

impl wxConnectionBase {
    pub fn from(handle: *u8) -> @_wxConnectionBase {
        @wxConnectionBase(handle) as @_wxConnectionBase
    }
    
}

trait _wxConnectionBase : _wxObject {
}

struct wxContextHelp(*u8);
impl _wxContextHelp for wxContextHelp {}
impl _wxObject for wxContextHelp { fn handle(&self) -> *u8 { **self } }

impl wxContextHelp {
    pub fn from(handle: *u8) -> @_wxContextHelp {
        @wxContextHelp(handle) as @_wxContextHelp
    }
    
    #[fixed_stack_segment]
    pub fn new(win: @_wxWindow, beginHelp: bool) -> @_wxContextHelp {
        unsafe { @wxContextHelp(wxContextHelp_Create(win.handle(), beginHelp)) as @_wxContextHelp }
    }
}

trait _wxContextHelp : _wxObject {
    #[fixed_stack_segment]
    fn beginContextHelp(&self, win: @_wxWindow) -> bool {
        unsafe { wxContextHelp_BeginContextHelp(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxContextHelp_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxContextHelpButton {
        @wxContextHelpButton(handle) as @_wxContextHelpButton
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @_wxContextHelpButton {
        unsafe { @wxContextHelpButton(wxContextHelpButton_Create(parent.handle(), id, x, y, w, h, style)) as @_wxContextHelpButton }
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
    pub fn from(handle: *u8) -> @_wxControl {
        @wxControl(handle) as @_wxControl
    }
    
}

trait _wxControl : _wxWindow {
    #[fixed_stack_segment]
    fn command(&self, event: @_wxEvent) {
        unsafe { wxControl_Command(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @_wxString {
        unsafe { @wxString(wxControl_GetLabel(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, text: @_wxString) {
        unsafe { wxControl_SetLabel(self.handle(), text.handle()) }
    }
}

struct wxCountingOutputStream(*u8);
impl _wxCountingOutputStream for wxCountingOutputStream {}
impl _wxOutputStream for wxCountingOutputStream {}
impl _wxStreamBase for wxCountingOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxCountingOutputStream {
    pub fn from(handle: *u8) -> @_wxCountingOutputStream {
        @wxCountingOutputStream(handle) as @_wxCountingOutputStream
    }
    
}

trait _wxCountingOutputStream : _wxOutputStream {
}

struct wxCriticalSection(*u8);
impl _wxCriticalSection for wxCriticalSection { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSection {
    pub fn from(handle: *u8) -> @_wxCriticalSection {
        @wxCriticalSection(handle) as @_wxCriticalSection
    }
    
}

trait _wxCriticalSection {
    fn handle(&self) -> *u8;
    
}

struct wxCriticalSectionLocker(*u8);
impl _wxCriticalSectionLocker for wxCriticalSectionLocker { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSectionLocker {
    pub fn from(handle: *u8) -> @_wxCriticalSectionLocker {
        @wxCriticalSectionLocker(handle) as @_wxCriticalSectionLocker
    }
    
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
    pub fn from(handle: *u8) -> @_wxCursor {
        @wxCursor(handle) as @_wxCursor
    }
    
}

trait _wxCursor : _wxBitmap {
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxCursor_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxCursor_IsStatic(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCursor_Delete(self.handle()) }
    }
}

struct wxCustomDataObject(*u8);
impl _wxCustomDataObject for wxCustomDataObject {}
impl _wxDataObjectSimple for wxCustomDataObject {}
impl _wxDataObject for wxCustomDataObject { fn handle(&self) -> *u8 { **self } }

impl wxCustomDataObject {
    pub fn from(handle: *u8) -> @_wxCustomDataObject {
        @wxCustomDataObject(handle) as @_wxCustomDataObject
    }
    
}

trait _wxCustomDataObject : _wxDataObjectSimple {
}

struct wxDC(*u8);
impl _wxDC for wxDC {}
impl _wxObject for wxDC { fn handle(&self) -> *u8 { **self } }

impl wxDC {
    pub fn from(handle: *u8) -> @_wxDC {
        @wxDC(handle) as @_wxDC
    }
    
}

trait _wxDC : _wxObject {
    #[fixed_stack_segment]
    fn blit(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: @_wxDC, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: bool) -> bool {
        unsafe { wxDC_Blit(self.handle(), xdest, ydest, width, height, source.handle(), xsrc, ysrc, rop, useMask) }
    }
    #[fixed_stack_segment]
    fn calcBoundingBox(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CalcBoundingBox(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn canDrawBitmap(&self) -> bool {
        unsafe { wxDC_CanDrawBitmap(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canGetTextExtent(&self) -> bool {
        unsafe { wxDC_CanGetTextExtent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxDC_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn computeScaleAndOrigin(&self) {
        unsafe { wxDC_ComputeScaleAndOrigin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn crossHair(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CrossHair(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn destroyClippingRegion(&self) {
        unsafe { wxDC_DestroyClippingRegion(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalX(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalXRel(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalY(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_DeviceToLogicalYRel(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn drawArc(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, xc: c_int, yc: c_int) {
        unsafe { wxDC_DrawArc(self.handle(), x1, y1, x2, y2, xc, yc) }
    }
    #[fixed_stack_segment]
    fn drawBitmap(&self, bmp: @_wxBitmap, x: c_int, y: c_int, useMask: bool) {
        unsafe { wxDC_DrawBitmap(self.handle(), bmp.handle(), x, y, useMask) }
    }
    #[fixed_stack_segment]
    fn drawCheckMark(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawCheckMark(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn drawCircle(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { wxDC_DrawCircle(self.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawEllipse(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn drawEllipticArc(&self, x: c_int, y: c_int, w: c_int, h: c_int, sa: c_double, ea: c_double) {
        unsafe { wxDC_DrawEllipticArc(self.handle(), x, y, w, h, sa, ea) }
    }
    #[fixed_stack_segment]
    fn drawIcon(&self, icon: @_wxIcon, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.handle(), icon.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawLabel(&self, str: @_wxString, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        unsafe { wxDC_DrawLabel(self.handle(), str.handle(), x, y, w, h, align, indexAccel) }
    }
    #[fixed_stack_segment]
    fn drawLabelBitmap(&self, str: @_wxString, bmp: @_wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> @_wxRect {
        unsafe { @wxRect(wxDC_DrawLabelBitmap(self.handle(), str.handle(), bmp.handle(), x, y, w, h, align, indexAccel)) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn drawLine(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { wxDC_DrawLine(self.handle(), x1, y1, x2, y2) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: c_int, x: *u8, y: *u8, xoffset: c_int, yoffset: c_int) {
        unsafe { wxDC_DrawLines(self.handle(), n, x, y, xoffset, yoffset) }
    }
    #[fixed_stack_segment]
    fn drawPoint(&self, x: c_int, y: c_int) {
        unsafe { wxDC_DrawPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawPolygon(&self, n: c_int, x: *u8, y: *u8, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolygon(self.handle(), n, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    fn drawPolyPolygon(&self, n: c_int, count: *u8, x: *u8, y: *u8, xoffset: c_int, yoffset: c_int, fillStyle: c_int) {
        unsafe { wxDC_DrawPolyPolygon(self.handle(), n, count, x, y, xoffset, yoffset, fillStyle) }
    }
    #[fixed_stack_segment]
    fn drawRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawRectangle(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn drawRotatedText(&self, text: @_wxString, x: c_int, y: c_int, angle: c_double) {
        unsafe { wxDC_DrawRotatedText(self.handle(), text.handle(), x, y, angle) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.handle(), x, y, width, height, radius) }
    }
    #[fixed_stack_segment]
    fn drawText(&self, text: @_wxString, x: c_int, y: c_int) {
        unsafe { wxDC_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn endDoc(&self) {
        unsafe { wxDC_EndDoc(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endPage(&self) {
        unsafe { wxDC_EndPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn floodFill(&self, x: c_int, y: c_int, col: @_wxColour, style: c_int) {
        unsafe { wxDC_FloodFill(self.handle(), x, y, col.handle(), style) }
    }
    #[fixed_stack_segment]
    fn getBackground(&self, _ref: @_wxBrush) {
        unsafe { wxDC_GetBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBrush(&self, _ref: @_wxBrush) {
        unsafe { wxDC_GetBrush(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCharHeight(&self) -> c_int {
        unsafe { wxDC_GetCharHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCharWidth(&self) -> c_int {
        unsafe { wxDC_GetCharWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClippingBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxDC_GetClippingBox(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn getDepth(&self) -> c_int {
        unsafe { wxDC_GetDepth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDeviceOrigin(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxDC_GetDeviceOrigin(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @_wxFont) {
        unsafe { wxDC_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLogicalFunction(&self) -> c_int {
        unsafe { wxDC_GetLogicalFunction(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLogicalOrigin(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxDC_GetLogicalOrigin(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getLogicalScale(&self, _x: *c_double, _y: *c_double) {
        unsafe { wxDC_GetLogicalScale(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getMapMode(&self) -> c_int {
        unsafe { wxDC_GetMapMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPPI(&self) -> @_wxSize {
        unsafe { @wxSize(wxDC_GetPPI(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getPen(&self, _ref: @_wxPen) {
        unsafe { wxDC_GetPen(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPixel(&self, x: c_int, y: c_int, col: @_wxColour) -> bool {
        unsafe { wxDC_GetPixel(self.handle(), x, y, col.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxDC_GetSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getSizeMM(&self) -> @_wxSize {
        unsafe { @wxSize(wxDC_GetSizeMM(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getTextBackground(&self, _ref: @_wxColour) {
        unsafe { wxDC_GetTextBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @_wxString, w: *u8, h: *u8, descent: *u8, externalLeading: *u8, theFont: @_wxFont) {
        unsafe { wxDC_GetTextExtent(self.handle(), string.handle(), w, h, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getMultiLineTextExtent(&self, string: @_wxString, w: *u8, h: *u8, heightLine: *u8, theFont: @_wxFont) {
        unsafe { wxDC_GetMultiLineTextExtent(self.handle(), string.handle(), w, h, heightLine, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextForeground(&self, _ref: @_wxColour) {
        unsafe { wxDC_GetTextForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getUserScale(&self, x: *c_double, y: *c_double) {
        unsafe { wxDC_GetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceX(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceXRel(&self, x: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceXRel(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceY(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceYRel(&self, y: c_int) -> c_int {
        unsafe { wxDC_LogicalToDeviceYRel(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn maxX(&self) -> c_int {
        unsafe { wxDC_MaxX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn maxY(&self) -> c_int {
        unsafe { wxDC_MaxY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn minX(&self) -> c_int {
        unsafe { wxDC_MinX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn minY(&self) -> c_int {
        unsafe { wxDC_MinY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxDC_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn resetBoundingBox(&self) {
        unsafe { wxDC_ResetBoundingBox(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAxisOrientation(&self, xLeftRight: bool, yBottomUp: bool) {
        unsafe { wxDC_SetAxisOrientation(self.handle(), xLeftRight, yBottomUp) }
    }
    #[fixed_stack_segment]
    fn setBackground(&self, brush: @_wxBrush) {
        unsafe { wxDC_SetBackground(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @_wxBrush) {
        unsafe { wxDC_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn setClippingRegionFromRegion(&self, region: @_wxRegion) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont) {
        unsafe { wxDC_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setLogicalFunction(&self, function: c_int) {
        unsafe { wxDC_SetLogicalFunction(self.handle(), function) }
    }
    #[fixed_stack_segment]
    fn setLogicalOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetLogicalOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setLogicalScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetLogicalScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setMapMode(&self, mode: c_int) {
        unsafe { wxDC_SetMapMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn setPalette(&self, palette: @_wxPalette) {
        unsafe { wxDC_SetPalette(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @_wxPen) {
        unsafe { wxDC_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextBackground(&self, colour: @_wxColour) {
        unsafe { wxDC_SetTextBackground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextForeground(&self, colour: @_wxColour) {
        unsafe { wxDC_SetTextForeground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn startDoc(&self, msg: @_wxString) -> bool {
        unsafe { wxDC_StartDoc(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    fn startPage(&self) {
        unsafe { wxDC_StartPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUserScaleX(&self) -> c_double {
        unsafe { wxDC_GetUserScaleX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUserScaleY(&self) -> c_double {
        unsafe { wxDC_GetUserScaleY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPixel2(&self, x: c_int, y: c_int, col: @_wxColour) {
        unsafe { wxDC_GetPixel2(self.handle(), x, y, col.handle()) }
    }
}

struct wxDCClipper(*u8);
impl _wxDCClipper for wxDCClipper { fn handle(&self) -> *u8 { **self } }

impl wxDCClipper {
    pub fn from(handle: *u8) -> @_wxDCClipper {
        @wxDCClipper(handle) as @_wxDCClipper
    }
    
}

trait _wxDCClipper {
    fn handle(&self) -> *u8;
    
}

struct wxDDEClient(*u8);
impl _wxDDEClient for wxDDEClient {}
impl _wxClientBase for wxDDEClient {}
impl _wxObject for wxDDEClient { fn handle(&self) -> *u8 { **self } }

impl wxDDEClient {
    pub fn from(handle: *u8) -> @_wxDDEClient {
        @wxDDEClient(handle) as @_wxDDEClient
    }
    
}

trait _wxDDEClient : _wxClientBase {
}

struct wxDDEConnection(*u8);
impl _wxDDEConnection for wxDDEConnection {}
impl _wxConnectionBase for wxDDEConnection {}
impl _wxObject for wxDDEConnection { fn handle(&self) -> *u8 { **self } }

impl wxDDEConnection {
    pub fn from(handle: *u8) -> @_wxDDEConnection {
        @wxDDEConnection(handle) as @_wxDDEConnection
    }
    
}

trait _wxDDEConnection : _wxConnectionBase {
}

struct wxDDEServer(*u8);
impl _wxDDEServer for wxDDEServer {}
impl _wxServerBase for wxDDEServer {}
impl _wxObject for wxDDEServer { fn handle(&self) -> *u8 { **self } }

impl wxDDEServer {
    pub fn from(handle: *u8) -> @_wxDDEServer {
        @wxDDEServer(handle) as @_wxDDEServer
    }
    
}

trait _wxDDEServer : _wxServerBase {
}

struct wxDataFormat(*u8);
impl _wxDataFormat for wxDataFormat { fn handle(&self) -> *u8 { **self } }

impl wxDataFormat {
    pub fn from(handle: *u8) -> @_wxDataFormat {
        @wxDataFormat(handle) as @_wxDataFormat
    }
    
    #[fixed_stack_segment]
    pub fn newFromId(name: @_wxString) -> @_wxDataFormat {
        unsafe { @wxDataFormat(wxDataFormat_CreateFromId(name.handle())) as @_wxDataFormat }
    }
    #[fixed_stack_segment]
    pub fn newFromType(typ: c_int) -> @_wxDataFormat {
        unsafe { @wxDataFormat(wxDataFormat_CreateFromType(typ)) as @_wxDataFormat }
    }
}

trait _wxDataFormat {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> @_wxString {
        unsafe { @wxString(wxDataFormat_GetId(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getType(&self) -> c_int {
        unsafe { wxDataFormat_GetType(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, other: *u8) -> bool {
        unsafe { wxDataFormat_IsEqual(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: *u8) {
        unsafe { wxDataFormat_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setType(&self, typ: c_int) {
        unsafe { wxDataFormat_SetType(self.handle(), typ) }
    }
}

struct wxDataInputStream(*u8);
impl _wxDataInputStream for wxDataInputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataInputStream {
    pub fn from(handle: *u8) -> @_wxDataInputStream {
        @wxDataInputStream(handle) as @_wxDataInputStream
    }
    
}

trait _wxDataInputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDataObject(*u8);
impl _wxDataObject for wxDataObject { fn handle(&self) -> *u8 { **self } }

impl wxDataObject {
    pub fn from(handle: *u8) -> @_wxDataObject {
        @wxDataObject(handle) as @_wxDataObject
    }
    
}

trait _wxDataObject {
    fn handle(&self) -> *u8;
    
}

struct wxDataObjectComposite(*u8);
impl _wxDataObjectComposite for wxDataObjectComposite {}
impl _wxDataObject for wxDataObjectComposite { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectComposite {
    pub fn from(handle: *u8) -> @_wxDataObjectComposite {
        @wxDataObjectComposite(handle) as @_wxDataObjectComposite
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxDataObjectComposite {
        unsafe { @wxDataObjectComposite(wxDataObjectComposite_Create()) as @_wxDataObjectComposite }
    }
}

trait _wxDataObjectComposite : _wxDataObject {
    #[fixed_stack_segment]
    fn add(&self, _dat: *u8, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.handle(), _dat, _preferred) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.handle()) }
    }
}

struct wxDataObjectSimple(*u8);
impl _wxDataObjectSimple for wxDataObjectSimple {}
impl _wxDataObject for wxDataObjectSimple { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectSimple {
    pub fn from(handle: *u8) -> @_wxDataObjectSimple {
        @wxDataObjectSimple(handle) as @_wxDataObjectSimple
    }
    
}

trait _wxDataObjectSimple : _wxDataObject {
}

struct wxDataOutputStream(*u8);
impl _wxDataOutputStream for wxDataOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataOutputStream {
    pub fn from(handle: *u8) -> @_wxDataOutputStream {
        @wxDataOutputStream(handle) as @_wxDataOutputStream
    }
    
}

trait _wxDataOutputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDatabase(*u8);
impl _wxDatabase for wxDatabase {}
impl _wxObject for wxDatabase { fn handle(&self) -> *u8 { **self } }

impl wxDatabase {
    pub fn from(handle: *u8) -> @_wxDatabase {
        @wxDatabase(handle) as @_wxDatabase
    }
    
}

trait _wxDatabase : _wxObject {
}

struct wxDateTime(*u8);
impl _wxDateTime for wxDateTime { fn handle(&self) -> *u8 { **self } }

impl wxDateTime {
    pub fn from(handle: *u8) -> @_wxDateTime {
        @wxDateTime(handle) as @_wxDateTime
    }
    
    #[fixed_stack_segment]
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    pub fn new() -> @_wxDateTime {
        unsafe { @wxDateTime(wxDateTime_Create()) as @_wxDateTime }
    }
    #[fixed_stack_segment]
    pub fn getAmString() -> @_wxString {
        unsafe { @wxString(wxDateTime_GetAmString()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getBeginDST(year: c_int, country: c_int, dt: @_wxDateTime) {
        unsafe { wxDateTime_GetBeginDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getCentury(year: c_int) -> c_int {
        unsafe { wxDateTime_GetCentury(year) }
    }
    #[fixed_stack_segment]
    pub fn getCountry() -> c_int {
        unsafe { wxDateTime_GetCountry() }
    }
    #[fixed_stack_segment]
    pub fn getCurrentMonth(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentMonth(cal) }
    }
    #[fixed_stack_segment]
    pub fn getCurrentYear(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentYear(cal) }
    }
    #[fixed_stack_segment]
    pub fn getEndDST(year: c_int, country: c_int, dt: @_wxDateTime) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getMonthName(month: c_int, flags: c_int) -> @_wxString {
        unsafe { @wxString(wxDateTime_GetMonthName(month, flags)) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    #[fixed_stack_segment]
    pub fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    #[fixed_stack_segment]
    pub fn getPmString() -> @_wxString {
        unsafe { @wxString(wxDateTime_GetPmString()) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> @_wxString {
        unsafe { @wxString(wxDateTime_GetWeekDayName(weekday, flags)) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn isDSTApplicable(year: c_int, country: c_int) -> bool {
        unsafe { wxDateTime_IsDSTApplicable(year, country) }
    }
    #[fixed_stack_segment]
    pub fn isLeapYear(year: c_int, cal: c_int) -> bool {
        unsafe { wxDateTime_IsLeapYear(year, cal) }
    }
    #[fixed_stack_segment]
    pub fn isWestEuropeanCountry(country: c_int) -> bool {
        unsafe { wxDateTime_IsWestEuropeanCountry(country) }
    }
    #[fixed_stack_segment]
    pub fn setCountry(country: c_int) {
        unsafe { wxDateTime_SetCountry(country) }
    }
    #[fixed_stack_segment]
    pub fn wxDateTime(hi_long: c_int, lo_long: c_int) -> *u8 {
        unsafe { wxDateTime_wxDateTime(hi_long, lo_long) }
    }
}

trait _wxDateTime {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addDate(&self, diff: *u8, _ref: @_wxDateTime) {
        unsafe { wxDateTime_AddDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.handle(), _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    fn addTime(&self, diff: *u8, _ref: @_wxDateTime) {
        unsafe { wxDateTime_AddTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.handle(), _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    fn format(&self, format: *u8, tz: c_int) -> @_wxString {
        unsafe { @wxString(wxDateTime_Format(self.handle(), format, tz)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn formatDate(&self) -> @_wxString {
        unsafe { @wxString(wxDateTime_FormatDate(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn formatISODate(&self) -> @_wxString {
        unsafe { @wxString(wxDateTime_FormatISODate(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn formatISOTime(&self) -> @_wxString {
        unsafe { @wxString(wxDateTime_FormatISOTime(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn formatTime(&self) -> @_wxString {
        unsafe { @wxString(wxDateTime_FormatTime(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getDay(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDay(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getDayOfYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetDayOfYear(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getHour(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetHour(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getLastMonthDay(&self, month: c_int, year: c_int, _ref: @_wxDateTime) {
        unsafe { wxDateTime_GetLastMonthDay(self.handle(), month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int, _ref: @_wxDateTime) {
        unsafe { wxDateTime_GetLastWeekDay(self.handle(), weekday, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getMillisecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMillisecond(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getMinute(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMinute(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getMonth(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetMonth(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getNextWeekDay(&self, weekday: c_int, _ref: @_wxDateTime) {
        unsafe { wxDateTime_GetNextWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevWeekDay(&self, weekday: c_int, _ref: @_wxDateTime) {
        unsafe { wxDateTime_GetPrevWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getSecond(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetSecond(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getTicks(&self) -> time_t {
        unsafe { wxDateTime_GetTicks(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self, hi_long: *u8, lo_long: *u8) {
        unsafe { wxDateTime_GetValue(self.handle(), hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    fn getWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: @_wxDateTime) {
        unsafe { wxDateTime_GetWeekDay(self.handle(), weekday, n, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeekDayInSameWeek(&self, weekday: c_int, _ref: @_wxDateTime) {
        unsafe { wxDateTime_GetWeekDayInSameWeek(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeekDayTZ(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekDayTZ(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getWeekOfMonth(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfMonth(self.handle(), flags, tz) }
    }
    #[fixed_stack_segment]
    fn getWeekOfYear(&self, flags: c_int, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetWeekOfYear(self.handle(), flags, tz) }
    }
    #[fixed_stack_segment]
    fn getYear(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetYear(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn isBetween(&self, t1: @_wxDateTime, t2: @_wxDateTime) -> bool {
        unsafe { wxDateTime_IsBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    fn isDST(&self, country: c_int) -> bool {
        unsafe { wxDateTime_IsDST(self.handle(), country) }
    }
    #[fixed_stack_segment]
    fn isEarlierThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsEarlierThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn isEqualTo(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsEqualTo(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn isEqualUpTo(&self, dt: @_wxDateTime, ts: *u8) -> bool {
        unsafe { wxDateTime_IsEqualUpTo(self.handle(), dt.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn isLaterThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsLaterThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn isSameDate(&self, dt: @_wxDateTime) -> bool {
        unsafe { wxDateTime_IsSameDate(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isSameTime(&self, dt: @_wxDateTime) -> bool {
        unsafe { wxDateTime_IsSameTime(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isStrictlyBetween(&self, t1: @_wxDateTime, t2: @_wxDateTime) -> bool {
        unsafe { wxDateTime_IsStrictlyBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    fn isValid(&self) -> bool {
        unsafe { wxDateTime_IsValid(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isWorkDay(&self, country: c_int) -> bool {
        unsafe { wxDateTime_IsWorkDay(self.handle(), country) }
    }
    #[fixed_stack_segment]
    fn makeGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_MakeGMT(self.handle(), noDST) }
    }
    #[fixed_stack_segment]
    fn makeTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_MakeTimezone(self.handle(), tz, noDST) }
    }
    #[fixed_stack_segment]
    fn now(&self) {
        unsafe { wxDateTime_Now(self.handle()) }
    }
    #[fixed_stack_segment]
    fn parseDate(&self, date: *u8) -> *u8 {
        unsafe { wxDateTime_ParseDate(self.handle(), date) }
    }
    #[fixed_stack_segment]
    fn parseDateTime(&self, datetime: *u8) -> *u8 {
        unsafe { wxDateTime_ParseDateTime(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn parseFormat(&self, date: *u8, format: *u8, dateDef: *u8) -> *u8 {
        unsafe { wxDateTime_ParseFormat(self.handle(), date, format, dateDef) }
    }
    #[fixed_stack_segment]
    fn parseRfc822Date(&self, date: *u8) -> *u8 {
        unsafe { wxDateTime_ParseRfc822Date(self.handle(), date) }
    }
    #[fixed_stack_segment]
    fn parseTime(&self, time: @_wxTime) -> *u8 {
        unsafe { wxDateTime_ParseTime(self.handle(), time.handle()) }
    }
    #[fixed_stack_segment]
    fn resetTime(&self) {
        unsafe { wxDateTime_ResetTime(self.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, day: c_int, month: c_int, year: c_int, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_Set(self.handle(), day, month, year, hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    fn setDay(&self, day: c_int) {
        unsafe { wxDateTime_SetDay(self.handle(), day) }
    }
    #[fixed_stack_segment]
    fn setHour(&self, hour: c_int) {
        unsafe { wxDateTime_SetHour(self.handle(), hour) }
    }
    #[fixed_stack_segment]
    fn setMillisecond(&self, millisecond: c_int) {
        unsafe { wxDateTime_SetMillisecond(self.handle(), millisecond) }
    }
    #[fixed_stack_segment]
    fn setMinute(&self, minute: c_int) {
        unsafe { wxDateTime_SetMinute(self.handle(), minute) }
    }
    #[fixed_stack_segment]
    fn setMonth(&self, month: c_int) {
        unsafe { wxDateTime_SetMonth(self.handle(), month) }
    }
    #[fixed_stack_segment]
    fn setSecond(&self, second: c_int) {
        unsafe { wxDateTime_SetSecond(self.handle(), second) }
    }
    #[fixed_stack_segment]
    fn setTime(&self, hour: c_int, minute: c_int, second: c_int, millisec: c_int) {
        unsafe { wxDateTime_SetTime(self.handle(), hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    fn setToCurrent(&self) {
        unsafe { wxDateTime_SetToCurrent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setToLastMonthDay(&self, month: c_int, year: c_int) {
        unsafe { wxDateTime_SetToLastMonthDay(self.handle(), month, year) }
    }
    #[fixed_stack_segment]
    fn setToLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int) -> bool {
        unsafe { wxDateTime_SetToLastWeekDay(self.handle(), weekday, month, year) }
    }
    #[fixed_stack_segment]
    fn setToNextWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToNextWeekDay(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    fn setToPrevWeekDay(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToPrevWeekDay(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    fn setToWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int) -> bool {
        unsafe { wxDateTime_SetToWeekDay(self.handle(), weekday, n, month, year) }
    }
    #[fixed_stack_segment]
    fn setToWeekDayInSameWeek(&self, weekday: c_int) {
        unsafe { wxDateTime_SetToWeekDayInSameWeek(self.handle(), weekday) }
    }
    #[fixed_stack_segment]
    fn setYear(&self, year: c_int) {
        unsafe { wxDateTime_SetYear(self.handle(), year) }
    }
    #[fixed_stack_segment]
    fn subtractDate(&self, diff: *u8, _ref: @_wxDateTime) {
        unsafe { wxDateTime_SubtractDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractTime(&self, diff: *u8, _ref: @_wxDateTime) {
        unsafe { wxDateTime_SubtractTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn toGMT(&self, noDST: c_int) {
        unsafe { wxDateTime_ToGMT(self.handle(), noDST) }
    }
    #[fixed_stack_segment]
    fn toTimezone(&self, tz: c_int, noDST: c_int) {
        unsafe { wxDateTime_ToTimezone(self.handle(), tz, noDST) }
    }
    #[fixed_stack_segment]
    fn today(&self) {
        unsafe { wxDateTime_Today(self.handle()) }
    }
    #[fixed_stack_segment]
    fn uNow(&self) {
        unsafe { wxDateTime_UNow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDateTime_Delete(self.handle()) }
    }
}

struct wxDb(*u8);
impl _wxDb for wxDb { fn handle(&self) -> *u8 { **self } }

impl wxDb {
    pub fn from(handle: *u8) -> @_wxDb {
        @wxDb(handle) as @_wxDb
    }
    
}

trait _wxDb {
    fn handle(&self) -> *u8;
    
}

struct wxDbColDef(*u8);
impl _wxDbColDef for wxDbColDef { fn handle(&self) -> *u8 { **self } }

impl wxDbColDef {
    pub fn from(handle: *u8) -> @_wxDbColDef {
        @wxDbColDef(handle) as @_wxDbColDef
    }
    
}

trait _wxDbColDef {
    fn handle(&self) -> *u8;
    
}

struct wxDbColFor(*u8);
impl _wxDbColFor for wxDbColFor { fn handle(&self) -> *u8 { **self } }

impl wxDbColFor {
    pub fn from(handle: *u8) -> @_wxDbColFor {
        @wxDbColFor(handle) as @_wxDbColFor
    }
    
}

trait _wxDbColFor {
    fn handle(&self) -> *u8;
    
}

struct wxDbColInf(*u8);
impl _wxDbColInf for wxDbColInf { fn handle(&self) -> *u8 { **self } }

impl wxDbColInf {
    pub fn from(handle: *u8) -> @_wxDbColInf {
        @wxDbColInf(handle) as @_wxDbColInf
    }
    
}

trait _wxDbColInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbConnectInf(*u8);
impl _wxDbConnectInf for wxDbConnectInf { fn handle(&self) -> *u8 { **self } }

impl wxDbConnectInf {
    pub fn from(handle: *u8) -> @_wxDbConnectInf {
        @wxDbConnectInf(handle) as @_wxDbConnectInf
    }
    
}

trait _wxDbConnectInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbInf(*u8);
impl _wxDbInf for wxDbInf { fn handle(&self) -> *u8 { **self } }

impl wxDbInf {
    pub fn from(handle: *u8) -> @_wxDbInf {
        @wxDbInf(handle) as @_wxDbInf
    }
    
}

trait _wxDbInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbSqlTypeInfo(*u8);
impl _wxDbSqlTypeInfo for wxDbSqlTypeInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbSqlTypeInfo {
    pub fn from(handle: *u8) -> @_wxDbSqlTypeInfo {
        @wxDbSqlTypeInfo(handle) as @_wxDbSqlTypeInfo
    }
    
}

trait _wxDbSqlTypeInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDbTable(*u8);
impl _wxDbTable for wxDbTable { fn handle(&self) -> *u8 { **self } }

impl wxDbTable {
    pub fn from(handle: *u8) -> @_wxDbTable {
        @wxDbTable(handle) as @_wxDbTable
    }
    
}

trait _wxDbTable {
    fn handle(&self) -> *u8;
    
}

struct wxDbTableInfo(*u8);
impl _wxDbTableInfo for wxDbTableInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbTableInfo {
    pub fn from(handle: *u8) -> @_wxDbTableInfo {
        @wxDbTableInfo(handle) as @_wxDbTableInfo
    }
    
}

trait _wxDbTableInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDebugContext(*u8);
impl _wxDebugContext for wxDebugContext { fn handle(&self) -> *u8 { **self } }

impl wxDebugContext {
    pub fn from(handle: *u8) -> @_wxDebugContext {
        @wxDebugContext(handle) as @_wxDebugContext
    }
    
}

trait _wxDebugContext {
    fn handle(&self) -> *u8;
    
}

struct wxDialUpEvent(*u8);
impl _wxDialUpEvent for wxDialUpEvent {}
impl _wxEvent for wxDialUpEvent {}
impl _wxObject for wxDialUpEvent { fn handle(&self) -> *u8 { **self } }

impl wxDialUpEvent {
    pub fn from(handle: *u8) -> @_wxDialUpEvent {
        @wxDialUpEvent(handle) as @_wxDialUpEvent
    }
    
}

trait _wxDialUpEvent : _wxEvent {
}

struct wxDialUpManager(*u8);
impl _wxDialUpManager for wxDialUpManager { fn handle(&self) -> *u8 { **self } }

impl wxDialUpManager {
    pub fn from(handle: *u8) -> @_wxDialUpManager {
        @wxDialUpManager(handle) as @_wxDialUpManager
    }
    
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
    pub fn from(handle: *u8) -> @_wxDialog {
        @wxDialog(handle) as @_wxDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxDialog {
        unsafe { @wxDialog(wxDialog_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxDialog }
    }
}

trait _wxDialog : _wxTopLevelWindow {
    #[fixed_stack_segment]
    fn endModal(&self, retCode: c_int) {
        unsafe { wxDialog_EndModal(self.handle(), retCode) }
    }
    #[fixed_stack_segment]
    fn getReturnCode(&self) -> c_int {
        unsafe { wxDialog_GetReturnCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isModal(&self) -> bool {
        unsafe { wxDialog_IsModal(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setReturnCode(&self, returnCode: c_int) {
        unsafe { wxDialog_SetReturnCode(self.handle(), returnCode) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxDirDialog {
        @wxDirDialog(handle) as @_wxDirDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _msg: @_wxString, _dir: @_wxString, _lft: c_int, _top: c_int, _stl: c_int) -> @_wxDirDialog {
        unsafe { @wxDirDialog(wxDirDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _lft, _top, _stl)) as @_wxDirDialog }
    }
}

trait _wxDirDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getMessage(&self) -> @_wxString {
        unsafe { @wxString(wxDirDialog_GetMessage(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @_wxString {
        unsafe { @wxString(wxDirDialog_GetPath(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, msg: @_wxString) {
        unsafe { wxDirDialog_SetMessage(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, pth: @_wxString) {
        unsafe { wxDirDialog_SetPath(self.handle(), pth.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.handle(), style) }
    }
}

struct wxDirTraverser(*u8);
impl _wxDirTraverser for wxDirTraverser { fn handle(&self) -> *u8 { **self } }

impl wxDirTraverser {
    pub fn from(handle: *u8) -> @_wxDirTraverser {
        @wxDirTraverser(handle) as @_wxDirTraverser
    }
    
}

trait _wxDirTraverser {
    fn handle(&self) -> *u8;
    
}

struct wxDllLoader(*u8);
impl _wxDllLoader for wxDllLoader { fn handle(&self) -> *u8 { **self } }

impl wxDllLoader {
    pub fn from(handle: *u8) -> @_wxDllLoader {
        @wxDllLoader(handle) as @_wxDllLoader
    }
    
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
    pub fn from(handle: *u8) -> @_wxDocChildFrame {
        @wxDocChildFrame(handle) as @_wxDocChildFrame
    }
    
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
    pub fn from(handle: *u8) -> @_wxDocMDIChildFrame {
        @wxDocMDIChildFrame(handle) as @_wxDocMDIChildFrame
    }
    
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
    pub fn from(handle: *u8) -> @_wxDocMDIParentFrame {
        @wxDocMDIParentFrame(handle) as @_wxDocMDIParentFrame
    }
    
}

trait _wxDocMDIParentFrame : _wxMDIParentFrame {
}

struct wxDocManager(*u8);
impl _wxDocManager for wxDocManager {}
impl _wxEvtHandler for wxDocManager {}
impl _wxObject for wxDocManager { fn handle(&self) -> *u8 { **self } }

impl wxDocManager {
    pub fn from(handle: *u8) -> @_wxDocManager {
        @wxDocManager(handle) as @_wxDocManager
    }
    
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
    pub fn from(handle: *u8) -> @_wxDocParentFrame {
        @wxDocParentFrame(handle) as @_wxDocParentFrame
    }
    
}

trait _wxDocParentFrame : _wxFrame {
}

struct wxDocTemplate(*u8);
impl _wxDocTemplate for wxDocTemplate {}
impl _wxObject for wxDocTemplate { fn handle(&self) -> *u8 { **self } }

impl wxDocTemplate {
    pub fn from(handle: *u8) -> @_wxDocTemplate {
        @wxDocTemplate(handle) as @_wxDocTemplate
    }
    
}

trait _wxDocTemplate : _wxObject {
}

struct wxDocument(*u8);
impl _wxDocument for wxDocument {}
impl _wxEvtHandler for wxDocument {}
impl _wxObject for wxDocument { fn handle(&self) -> *u8 { **self } }

impl wxDocument {
    pub fn from(handle: *u8) -> @_wxDocument {
        @wxDocument(handle) as @_wxDocument
    }
    
}

trait _wxDocument : _wxEvtHandler {
}

struct wxDragImage(*u8);
impl _wxDragImage for wxDragImage {}
impl _wxObject for wxDragImage { fn handle(&self) -> *u8 { **self } }

impl wxDragImage {
    pub fn from(handle: *u8) -> @_wxDragImage {
        @wxDragImage(handle) as @_wxDragImage
    }
    
    #[fixed_stack_segment]
    pub fn new(image: @_wxBitmap, x: c_int, y: c_int) -> @_wxDragImage {
        unsafe { @wxDragImage(wxDragImage_Create(image.handle(), x, y)) as @_wxDragImage }
    }
}

trait _wxDragImage : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDragImage_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDragFullScreen(&self, x_pos: c_int, y_pos: c_int, window: @_wxWindow, fullScreen: bool, rect: @_wxRect) -> bool {
        unsafe { wxDragImage_BeginDragFullScreen(self.handle(), x_pos, y_pos, window.handle(), fullScreen, rect.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDrag(&self, x: c_int, y: c_int, window: @_wxWindow, boundingWindow: @_wxWindow) -> bool {
        unsafe { wxDragImage_BeginDrag(self.handle(), x, y, window.handle(), boundingWindow.handle()) }
    }
    #[fixed_stack_segment]
    fn endDrag(&self) {
        unsafe { wxDragImage_EndDrag(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hide(&self) -> bool {
        unsafe { wxDragImage_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    fn move(&self, x: c_int, y: c_int) -> bool {
        unsafe { wxDragImage_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxDrawControl {
        @wxDrawControl(handle) as @_wxDrawControl
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxDrawControl {
        unsafe { @wxDrawControl(wxDrawControl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxDrawControl }
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
    pub fn from(handle: *u8) -> @_wxDrawWindow {
        @wxDrawWindow(handle) as @_wxDrawWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxDrawWindow {
        unsafe { @wxDrawWindow(wxDrawWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxDrawWindow }
    }
}

trait _wxDrawWindow : _wxWindow {
}

struct wxDropFilesEvent(*u8);
impl _wxDropFilesEvent for wxDropFilesEvent {}
impl _wxEvent for wxDropFilesEvent {}
impl _wxObject for wxDropFilesEvent { fn handle(&self) -> *u8 { **self } }

impl wxDropFilesEvent {
    pub fn from(handle: *u8) -> @_wxDropFilesEvent {
        @wxDropFilesEvent(handle) as @_wxDropFilesEvent
    }
    
}

trait _wxDropFilesEvent : _wxEvent {
}

struct wxDropSource(*u8);
impl _wxDropSource for wxDropSource { fn handle(&self) -> *u8 { **self } }

impl wxDropSource {
    pub fn from(handle: *u8) -> @_wxDropSource {
        @wxDropSource(handle) as @_wxDropSource
    }
    
}

trait _wxDropSource {
    fn handle(&self) -> *u8;
    
}

struct wxDropTarget(*u8);
impl _wxDropTarget for wxDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxDropTarget {
    pub fn from(handle: *u8) -> @_wxDropTarget {
        @wxDropTarget(handle) as @_wxDropTarget
    }
    
}

trait _wxDropTarget {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDataObject(&self, _dat: @_wxDataObject) {
        unsafe { wxDropTarget_SetDataObject(self.handle(), _dat.handle()) }
    }
}

struct wxDynToolInfo(*u8);
impl _wxDynToolInfo for wxDynToolInfo {}
impl _wxToolLayoutItem for wxDynToolInfo {}
impl _wxObject for wxDynToolInfo { fn handle(&self) -> *u8 { **self } }

impl wxDynToolInfo {
    pub fn from(handle: *u8) -> @_wxDynToolInfo {
        @wxDynToolInfo(handle) as @_wxDynToolInfo
    }
    
}

trait _wxDynToolInfo : _wxToolLayoutItem {
}

struct wxDynamicLibrary(*u8);
impl _wxDynamicLibrary for wxDynamicLibrary { fn handle(&self) -> *u8 { **self } }

impl wxDynamicLibrary {
    pub fn from(handle: *u8) -> @_wxDynamicLibrary {
        @wxDynamicLibrary(handle) as @_wxDynamicLibrary
    }
    
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
    pub fn from(handle: *u8) -> @_wxDynamicSashWindow {
        @wxDynamicSashWindow(handle) as @_wxDynamicSashWindow
    }
    
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
    pub fn from(handle: *u8) -> @_wxDynamicToolBar {
        @wxDynamicToolBar(handle) as @_wxDynamicToolBar
    }
    
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
    pub fn from(handle: *u8) -> @_wxEditableListBox {
        @wxEditableListBox(handle) as @_wxEditableListBox
    }
    
}

trait _wxEditableListBox : _wxPanel {
}

struct wxEncodingConverter(*u8);
impl _wxEncodingConverter for wxEncodingConverter {}
impl _wxObject for wxEncodingConverter { fn handle(&self) -> *u8 { **self } }

impl wxEncodingConverter {
    pub fn from(handle: *u8) -> @_wxEncodingConverter {
        @wxEncodingConverter(handle) as @_wxEncodingConverter
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxEncodingConverter {
        unsafe { @wxEncodingConverter(wxEncodingConverter_Create()) as @_wxEncodingConverter }
    }
}

trait _wxEncodingConverter : _wxObject {
    #[fixed_stack_segment]
    fn convert(&self, input: *u8, output: *u8) {
        unsafe { wxEncodingConverter_Convert(self.handle(), input, output) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxEncodingConverter_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAllEquivalents(&self, enc: c_int, _lst: @_wxList) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.handle(), enc, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getPlatformEquivalents(&self, enc: c_int, platform: c_int, _lst: @_wxList) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.handle(), enc, platform, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.handle(), input_enc, output_enc, method) }
    }
}

struct wxEraseEvent(*u8);
impl _wxEraseEvent for wxEraseEvent {}
impl _wxEvent for wxEraseEvent {}
impl _wxObject for wxEraseEvent { fn handle(&self) -> *u8 { **self } }

impl wxEraseEvent {
    pub fn from(handle: *u8) -> @_wxEraseEvent {
        @wxEraseEvent(handle) as @_wxEraseEvent
    }
    
}

trait _wxEraseEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxEraseEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getDC(&self) -> @_wxDC {
        unsafe { @wxDC(wxEraseEvent_GetDC(self.handle())) as @_wxDC }
    }
}

struct wxEvent(*u8);
impl _wxEvent for wxEvent {}
impl _wxObject for wxEvent { fn handle(&self) -> *u8 { **self } }

impl wxEvent {
    pub fn from(handle: *u8) -> @_wxEvent {
        @wxEvent(handle) as @_wxEvent
    }
    
    #[fixed_stack_segment]
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

trait _wxEvent : _wxObject {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn getEventObject(&self) -> @_wxObject {
        unsafe { @wxObject(wxEvent_GetEventObject(self.handle())) as @_wxObject }
    }
    #[fixed_stack_segment]
    fn getEventType(&self) -> c_int {
        unsafe { wxEvent_GetEventType(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxEvent_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSkipped(&self) -> bool {
        unsafe { wxEvent_GetSkipped(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTimestamp(&self) -> c_int {
        unsafe { wxEvent_GetTimestamp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isCommandEvent(&self) -> bool {
        unsafe { wxEvent_IsCommandEvent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEventObject(&self, obj: @_wxObject) {
        unsafe { wxEvent_SetEventObject(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn setEventType(&self, typ: c_int) {
        unsafe { wxEvent_SetEventType(self.handle(), typ) }
    }
    #[fixed_stack_segment]
    fn setId(&self, Id: c_int) {
        unsafe { wxEvent_SetId(self.handle(), Id) }
    }
    #[fixed_stack_segment]
    fn setTimestamp(&self, ts: c_int) {
        unsafe { wxEvent_SetTimestamp(self.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn skip(&self) {
        unsafe { wxEvent_Skip(self.handle()) }
    }
}

struct wxEvtHandler(*u8);
impl _wxEvtHandler for wxEvtHandler {}
impl _wxObject for wxEvtHandler { fn handle(&self) -> *u8 { **self } }

impl wxEvtHandler {
    pub fn from(handle: *u8) -> @_wxEvtHandler {
        @wxEvtHandler(handle) as @_wxEvtHandler
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_Create()) as @_wxEvtHandler }
    }
}

trait _wxEvtHandler : _wxObject {
    #[fixed_stack_segment]
    fn addPendingEvent(&self, event: @_wxEvent) {
        unsafe { wxEvtHandler_AddPendingEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn connect(&self, first: c_int, last: c_int, type_: c_int, data: *u8) -> c_int {
        unsafe { wxEvtHandler_Connect(self.handle(), first, last, type_, data) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxEvtHandler_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn disconnect(&self, first: c_int, last: c_int, type_: c_int, id: c_int) -> c_int {
        unsafe { wxEvtHandler_Disconnect(self.handle(), first, last, type_, id) }
    }
    #[fixed_stack_segment]
    fn getEvtHandlerEnabled(&self) -> bool {
        unsafe { wxEvtHandler_GetEvtHandlerEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextHandler(&self) -> @_wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_GetNextHandler(self.handle())) as @_wxEvtHandler }
    }
    #[fixed_stack_segment]
    fn getPreviousHandler(&self) -> @_wxEvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_GetPreviousHandler(self.handle())) as @_wxEvtHandler }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @_wxEvent) -> bool {
        unsafe { wxEvtHandler_ProcessEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEvtHandlerEnabled(&self, enabled: bool) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self.handle(), enabled) }
    }
    #[fixed_stack_segment]
    fn setNextHandler(&self, handler: @_wxEvtHandler) {
        unsafe { wxEvtHandler_SetNextHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setPreviousHandler(&self, handler: @_wxEvtHandler) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self, id: c_int, type_: c_int) -> @_wxClosure {
        unsafe { @wxClosure(wxEvtHandler_GetClosure(self.handle(), id, type_)) as @_wxClosure }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @_wxClosure {
        unsafe { @wxClosure(wxEvtHandler_GetClientClosure(self.handle())) as @_wxClosure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @_wxClosure) {
        unsafe { wxEvtHandler_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxExpr(*u8);
impl _wxExpr for wxExpr { fn handle(&self) -> *u8 { **self } }

impl wxExpr {
    pub fn from(handle: *u8) -> @_wxExpr {
        @wxExpr(handle) as @_wxExpr
    }
    
}

trait _wxExpr {
    fn handle(&self) -> *u8;
    
}

struct wxExprDatabase(*u8);
impl _wxExprDatabase for wxExprDatabase {}
impl _wxList for wxExprDatabase {}
impl _wxObject for wxExprDatabase { fn handle(&self) -> *u8 { **self } }

impl wxExprDatabase {
    pub fn from(handle: *u8) -> @_wxExprDatabase {
        @wxExprDatabase(handle) as @_wxExprDatabase
    }
    
}

trait _wxExprDatabase : _wxList {
}

struct wxFFile(*u8);
impl _wxFFile for wxFFile { fn handle(&self) -> *u8 { **self } }

impl wxFFile {
    pub fn from(handle: *u8) -> @_wxFFile {
        @wxFFile(handle) as @_wxFFile
    }
    
}

trait _wxFFile {
    fn handle(&self) -> *u8;
    
}

struct wxFFileInputStream(*u8);
impl _wxFFileInputStream for wxFFileInputStream {}
impl _wxInputStream for wxFFileInputStream {}
impl _wxStreamBase for wxFFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileInputStream {
    pub fn from(handle: *u8) -> @_wxFFileInputStream {
        @wxFFileInputStream(handle) as @_wxFFileInputStream
    }
    
}

trait _wxFFileInputStream : _wxInputStream {
}

struct wxFFileOutputStream(*u8);
impl _wxFFileOutputStream for wxFFileOutputStream {}
impl _wxOutputStream for wxFFileOutputStream {}
impl _wxStreamBase for wxFFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileOutputStream {
    pub fn from(handle: *u8) -> @_wxFFileOutputStream {
        @wxFFileOutputStream(handle) as @_wxFFileOutputStream
    }
    
}

trait _wxFFileOutputStream : _wxOutputStream {
}

struct wxFSFile(*u8);
impl _wxFSFile for wxFSFile {}
impl _wxObject for wxFSFile { fn handle(&self) -> *u8 { **self } }

impl wxFSFile {
    pub fn from(handle: *u8) -> @_wxFSFile {
        @wxFSFile(handle) as @_wxFSFile
    }
    
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
    pub fn from(handle: *u8) -> @_wxFTP {
        @wxFTP(handle) as @_wxFTP
    }
    
}

trait _wxFTP : _wxProtocol {
}

struct wxFileDataObject(*u8);
impl _wxFileDataObject for wxFileDataObject {}
impl _wxDataObjectSimple for wxFileDataObject {}
impl _wxDataObject for wxFileDataObject { fn handle(&self) -> *u8 { **self } }

impl wxFileDataObject {
    pub fn from(handle: *u8) -> @_wxFileDataObject {
        @wxFileDataObject(handle) as @_wxFileDataObject
    }
    
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
    pub fn from(handle: *u8) -> @_wxFileDialog {
        @wxFileDialog(handle) as @_wxFileDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _msg: @_wxString, _dir: @_wxString, _fle: @_wxString, _wcd: @_wxString, _lft: c_int, _top: c_int, _stl: c_int) -> @_wxFileDialog {
        unsafe { @wxFileDialog(wxFileDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _fle.handle(), _wcd.handle(), _lft, _top, _stl)) as @_wxFileDialog }
    }
}

trait _wxFileDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getDirectory(&self) -> @_wxString {
        unsafe { @wxString(wxFileDialog_GetDirectory(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> @_wxString {
        unsafe { @wxString(wxFileDialog_GetFilename(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFilenames(&self, paths: *wchar_t) -> c_int {
        unsafe { wxFileDialog_GetFilenames(self.handle(), paths) }
    }
    #[fixed_stack_segment]
    fn getFilterIndex(&self) -> c_int {
        unsafe { wxFileDialog_GetFilterIndex(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMessage(&self) -> @_wxString {
        unsafe { @wxString(wxFileDialog_GetMessage(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @_wxString {
        unsafe { @wxString(wxFileDialog_GetPath(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPaths(&self, paths: *wchar_t) -> c_int {
        unsafe { wxFileDialog_GetPaths(self.handle(), paths) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxFileDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWildcard(&self) -> @_wxString {
        unsafe { @wxString(wxFileDialog_GetWildcard(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setDirectory(&self, dir: @_wxString) {
        unsafe { wxFileDialog_SetDirectory(self.handle(), dir.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilename(&self, name: @_wxString) {
        unsafe { wxFileDialog_SetFilename(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.handle(), filterIndex) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, message: @_wxString) {
        unsafe { wxFileDialog_SetMessage(self.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, path: @_wxString) {
        unsafe { wxFileDialog_SetPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setWildcard(&self, wildCard: @_wxString) {
        unsafe { wxFileDialog_SetWildcard(self.handle(), wildCard.handle()) }
    }
}

struct wxFileDropTarget(*u8);
impl _wxFileDropTarget for wxFileDropTarget {}
impl _wxDropTarget for wxFileDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxFileDropTarget {
    pub fn from(handle: *u8) -> @_wxFileDropTarget {
        @wxFileDropTarget(handle) as @_wxFileDropTarget
    }
    
}

trait _wxFileDropTarget : _wxDropTarget {
}

struct wxFileHistory(*u8);
impl _wxFileHistory for wxFileHistory {}
impl _wxObject for wxFileHistory { fn handle(&self) -> *u8 { **self } }

impl wxFileHistory {
    pub fn from(handle: *u8) -> @_wxFileHistory {
        @wxFileHistory(handle) as @_wxFileHistory
    }
    
    #[fixed_stack_segment]
    pub fn new(maxFiles: c_int) -> @_wxFileHistory {
        unsafe { @wxFileHistory(wxFileHistory_Create(maxFiles)) as @_wxFileHistory }
    }
}

trait _wxFileHistory : _wxObject {
    #[fixed_stack_segment]
    fn addFileToHistory(&self, file: @_wxString) {
        unsafe { wxFileHistory_AddFileToHistory(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn addFilesToMenu(&self, menu: @_wxMenu) {
        unsafe { wxFileHistory_AddFilesToMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileHistory_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxFileHistory_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHistoryFile(&self, i: c_int) -> @_wxString {
        unsafe { @wxString(wxFileHistory_GetHistoryFile(self.handle(), i)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getMaxFiles(&self) -> c_int {
        unsafe { wxFileHistory_GetMaxFiles(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMenus(&self, _ref: *u8) -> c_int {
        unsafe { wxFileHistory_GetMenus(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn load(&self, config: @_wxConfigBase) {
        unsafe { wxFileHistory_Load(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.handle(), i) }
    }
    #[fixed_stack_segment]
    fn removeMenu(&self, menu: @_wxMenu) {
        unsafe { wxFileHistory_RemoveMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn save(&self, config: @_wxConfigBase) {
        unsafe { wxFileHistory_Save(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn useMenu(&self, menu: @_wxMenu) {
        unsafe { wxFileHistory_UseMenu(self.handle(), menu.handle()) }
    }
}

struct wxFileInputStream(*u8);
impl _wxFileInputStream for wxFileInputStream {}
impl _wxInputStream for wxFileInputStream {}
impl _wxStreamBase for wxFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileInputStream {
    pub fn from(handle: *u8) -> @_wxFileInputStream {
        @wxFileInputStream(handle) as @_wxFileInputStream
    }
    
}

trait _wxFileInputStream : _wxInputStream {
}

struct wxFileName(*u8);
impl _wxFileName for wxFileName { fn handle(&self) -> *u8 { **self } }

impl wxFileName {
    pub fn from(handle: *u8) -> @_wxFileName {
        @wxFileName(handle) as @_wxFileName
    }
    
}

trait _wxFileName {
    fn handle(&self) -> *u8;
    
}

struct wxFileOutputStream(*u8);
impl _wxFileOutputStream for wxFileOutputStream {}
impl _wxOutputStream for wxFileOutputStream {}
impl _wxStreamBase for wxFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileOutputStream {
    pub fn from(handle: *u8) -> @_wxFileOutputStream {
        @wxFileOutputStream(handle) as @_wxFileOutputStream
    }
    
}

trait _wxFileOutputStream : _wxOutputStream {
}

struct wxFileSystem(*u8);
impl _wxFileSystem for wxFileSystem {}
impl _wxObject for wxFileSystem { fn handle(&self) -> *u8 { **self } }

impl wxFileSystem {
    pub fn from(handle: *u8) -> @_wxFileSystem {
        @wxFileSystem(handle) as @_wxFileSystem
    }
    
}

trait _wxFileSystem : _wxObject {
}

struct wxFileSystemHandler(*u8);
impl _wxFileSystemHandler for wxFileSystemHandler {}
impl _wxObject for wxFileSystemHandler { fn handle(&self) -> *u8 { **self } }

impl wxFileSystemHandler {
    pub fn from(handle: *u8) -> @_wxFileSystemHandler {
        @wxFileSystemHandler(handle) as @_wxFileSystemHandler
    }
    
}

trait _wxFileSystemHandler : _wxObject {
}

struct wxFileType(*u8);
impl _wxFileType for wxFileType { fn handle(&self) -> *u8 { **self } }

impl wxFileType {
    pub fn from(handle: *u8) -> @_wxFileType {
        @wxFileType(handle) as @_wxFileType
    }
    
}

trait _wxFileType {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn expandCommand(&self, _cmd: *u8, _params: *u8) -> @_wxString {
        unsafe { @wxString(wxFileType_ExpandCommand(self.handle(), _cmd, _params)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getDescription(&self) -> @_wxString {
        unsafe { @wxString(wxFileType_GetDescription(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getExtensions(&self, _lst: @_wxList) -> c_int {
        unsafe { wxFileType_GetExtensions(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, icon: @_wxIcon) -> c_int {
        unsafe { wxFileType_GetIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn getMimeType(&self) -> @_wxString {
        unsafe { @wxString(wxFileType_GetMimeType(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getMimeTypes(&self, _lst: @_wxList) -> c_int {
        unsafe { wxFileType_GetMimeTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getOpenCommand(&self, _buf: *u8, _params: *u8) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.handle(), _buf, _params) }
    }
    #[fixed_stack_segment]
    fn getPrintCommand(&self, _buf: *u8, _params: *u8) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.handle(), _buf, _params) }
    }
}

struct wxFilterInputStream(*u8);
impl _wxFilterInputStream for wxFilterInputStream {}
impl _wxInputStream for wxFilterInputStream {}
impl _wxStreamBase for wxFilterInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterInputStream {
    pub fn from(handle: *u8) -> @_wxFilterInputStream {
        @wxFilterInputStream(handle) as @_wxFilterInputStream
    }
    
}

trait _wxFilterInputStream : _wxInputStream {
}

struct wxFilterOutputStream(*u8);
impl _wxFilterOutputStream for wxFilterOutputStream {}
impl _wxOutputStream for wxFilterOutputStream {}
impl _wxStreamBase for wxFilterOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterOutputStream {
    pub fn from(handle: *u8) -> @_wxFilterOutputStream {
        @wxFilterOutputStream(handle) as @_wxFilterOutputStream
    }
    
}

trait _wxFilterOutputStream : _wxOutputStream {
}

struct wxFindDialogEvent(*u8);
impl _wxFindDialogEvent for wxFindDialogEvent {}
impl _wxCommandEvent for wxFindDialogEvent {}
impl _wxEvent for wxFindDialogEvent {}
impl _wxObject for wxFindDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxFindDialogEvent {
    pub fn from(handle: *u8) -> @_wxFindDialogEvent {
        @wxFindDialogEvent(handle) as @_wxFindDialogEvent
    }
    
}

trait _wxFindDialogEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getFindString(&self, _ref: *u8) -> c_int {
        unsafe { wxFindDialogEvent_GetFindString(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindDialogEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self, _ref: *u8) -> c_int {
        unsafe { wxFindDialogEvent_GetReplaceString(self.handle(), _ref) }
    }
}

struct wxFindReplaceData(*u8);
impl _wxFindReplaceData for wxFindReplaceData {}
impl _wxObject for wxFindReplaceData { fn handle(&self) -> *u8 { **self } }

impl wxFindReplaceData {
    pub fn from(handle: *u8) -> @_wxFindReplaceData {
        @wxFindReplaceData(handle) as @_wxFindReplaceData
    }
    
    #[fixed_stack_segment]
    pub fn new(flags: c_int) -> @_wxFindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceData_Create(flags)) as @_wxFindReplaceData }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxFindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceData_CreateDefault()) as @_wxFindReplaceData }
    }
}

trait _wxFindReplaceData : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFindReplaceData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFindString(&self) -> @_wxString {
        unsafe { @wxString(wxFindReplaceData_GetFindString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self) -> @_wxString {
        unsafe { @wxString(wxFindReplaceData_GetReplaceString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setFindString(&self, str: @_wxString) {
        unsafe { wxFindReplaceData_SetFindString(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn setReplaceString(&self, str: @_wxString) {
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
    pub fn from(handle: *u8) -> @_wxFindReplaceDialog {
        @wxFindReplaceDialog(handle) as @_wxFindReplaceDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, data: @_wxFindReplaceData, title: @_wxString, style: c_int) -> @_wxFindReplaceDialog {
        unsafe { @wxFindReplaceDialog(wxFindReplaceDialog_Create(parent.handle(), data.handle(), title.handle(), style)) as @_wxFindReplaceDialog }
    }
}

trait _wxFindReplaceDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getData(&self) -> @_wxFindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceDialog_GetData(self.handle())) as @_wxFindReplaceData }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @_wxFindReplaceData) {
        unsafe { wxFindReplaceDialog_SetData(self.handle(), data.handle()) }
    }
}

struct wxFlexGridSizer(*u8);
impl _wxFlexGridSizer for wxFlexGridSizer {}
impl _wxGridSizer for wxFlexGridSizer {}
impl _wxSizer for wxFlexGridSizer {}
impl _wxObject for wxFlexGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxFlexGridSizer {
    pub fn from(handle: *u8) -> @_wxFlexGridSizer {
        @wxFlexGridSizer(handle) as @_wxFlexGridSizer
    }
    
    #[fixed_stack_segment]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @_wxFlexGridSizer {
        unsafe { @wxFlexGridSizer(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) as @_wxFlexGridSizer }
    }
}

trait _wxFlexGridSizer : _wxGridSizer {
    #[fixed_stack_segment]
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @_wxSize {
        unsafe { @wxSize(wxFlexGridSizer_CalcMin(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxFlexGridSizer_RecalcSizes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn removeGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableCol(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn removeGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableRow(self.handle(), idx) }
    }
}

struct wxFocusEvent(*u8);
impl _wxFocusEvent for wxFocusEvent {}
impl _wxEvent for wxFocusEvent {}
impl _wxObject for wxFocusEvent { fn handle(&self) -> *u8 { **self } }

impl wxFocusEvent {
    pub fn from(handle: *u8) -> @_wxFocusEvent {
        @wxFocusEvent(handle) as @_wxFocusEvent
    }
    
}

trait _wxFocusEvent : _wxEvent {
}

struct wxFont(*u8);
impl _wxFont for wxFont {}
impl _wxGDIObject for wxFont {}
impl _wxObject for wxFont { fn handle(&self) -> *u8 { **self } }

impl wxFont {
    pub fn from(handle: *u8) -> @_wxFont {
        @wxFont(handle) as @_wxFont
    }
    
    #[fixed_stack_segment]
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: bool, face: @_wxString, enc: c_int) -> @_wxFont {
        unsafe { @wxFont(wxFont_Create(pointSize, family, style, weight, underlined, face.handle(), enc)) as @_wxFont }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @_wxFont {
        unsafe { @wxFont(wxFont_CreateFromStock(id)) as @_wxFont }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxFont {
        unsafe { @wxFont(wxFont_CreateDefault()) as @_wxFont }
    }
}

trait _wxFont : _wxGDIObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFont_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultEncoding(&self) -> c_int {
        unsafe { wxFont_GetDefaultEncoding(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEncoding(&self) -> c_int {
        unsafe { wxFont_GetEncoding(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFaceName(&self) -> @_wxString {
        unsafe { @wxString(wxFont_GetFaceName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFamilyString(&self) -> @_wxString {
        unsafe { @wxString(wxFont_GetFamilyString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPointSize(&self) -> c_int {
        unsafe { wxFont_GetPointSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxFont_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyleString(&self) -> @_wxString {
        unsafe { @wxString(wxFont_GetStyleString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getUnderlined(&self) -> c_int {
        unsafe { wxFont_GetUnderlined(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeight(&self) -> c_int {
        unsafe { wxFont_GetWeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeightString(&self) -> @_wxString {
        unsafe { @wxString(wxFont_GetWeightString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxFont_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetDefaultEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFont_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    fn setFaceName(&self, faceName: @_wxString) {
        unsafe { wxFont_SetFaceName(self.handle(), faceName.handle()) }
    }
    #[fixed_stack_segment]
    fn setFamily(&self, family: c_int) {
        unsafe { wxFont_SetFamily(self.handle(), family) }
    }
    #[fixed_stack_segment]
    fn setPointSize(&self, pointSize: c_int) {
        unsafe { wxFont_SetPointSize(self.handle(), pointSize) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFont_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setUnderlined(&self, underlined: c_int) {
        unsafe { wxFont_SetUnderlined(self.handle(), underlined) }
    }
    #[fixed_stack_segment]
    fn setWeight(&self, weight: c_int) {
        unsafe { wxFont_SetWeight(self.handle(), weight) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxFont_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxFont_IsStatic(self.handle()) }
    }
}

struct wxFontData(*u8);
impl _wxFontData for wxFontData {}
impl _wxObject for wxFontData { fn handle(&self) -> *u8 { **self } }

impl wxFontData {
    pub fn from(handle: *u8) -> @_wxFontData {
        @wxFontData(handle) as @_wxFontData
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxFontData {
        unsafe { @wxFontData(wxFontData_Create()) as @_wxFontData }
    }
}

trait _wxFontData : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableEffects(&self, flag: bool) {
        unsafe { wxFontData_EnableEffects(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn getAllowSymbols(&self) -> bool {
        unsafe { wxFontData_GetAllowSymbols(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getChosenFont(&self, ref_: @_wxFont) {
        unsafe { wxFontData_GetChosenFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @_wxColour) {
        unsafe { wxFontData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnableEffects(&self) -> bool {
        unsafe { wxFontData_GetEnableEffects(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEncoding(&self) -> c_int {
        unsafe { wxFontData_GetEncoding(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getInitialFont(&self, ref_: @_wxFont) {
        unsafe { wxFontData_GetInitialFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    fn getShowHelp(&self) -> c_int {
        unsafe { wxFontData_GetShowHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAllowSymbols(&self, flag: bool) {
        unsafe { wxFontData_SetAllowSymbols(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setChosenFont(&self, font: @_wxFont) {
        unsafe { wxFontData_SetChosenFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @_wxColour) {
        unsafe { wxFontData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    fn setInitialFont(&self, font: @_wxFont) {
        unsafe { wxFontData_SetInitialFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, minRange: c_int, maxRange: c_int) {
        unsafe { wxFontData_SetRange(self.handle(), minRange, maxRange) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxFontDialog {
        @wxFontDialog(handle) as @_wxFontDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, fnt: @_wxFontData) -> @_wxFontDialog {
        unsafe { @wxFontDialog(wxFontDialog_Create(_prt.handle(), fnt.handle())) as @_wxFontDialog }
    }
}

trait _wxFontDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getFontData(&self, _ref: @_wxFontData) {
        unsafe { wxFontDialog_GetFontData(self.handle(), _ref.handle()) }
    }
}

struct wxFontEnumerator(*u8);
impl _wxFontEnumerator for wxFontEnumerator { fn handle(&self) -> *u8 { **self } }

impl wxFontEnumerator {
    pub fn from(handle: *u8) -> @_wxFontEnumerator {
        @wxFontEnumerator(handle) as @_wxFontEnumerator
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8) -> @_wxFontEnumerator {
        unsafe { @wxFontEnumerator(wxFontEnumerator_Create(_obj, _fnc)) as @_wxFontEnumerator }
    }
}

trait _wxFontEnumerator {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateEncodings(&self, facename: @_wxString) -> bool {
        unsafe { wxFontEnumerator_EnumerateEncodings(self.handle(), facename.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> bool {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.handle(), encoding, fixedWidthOnly) }
    }
}

struct wxFontList(*u8);
impl _wxFontList for wxFontList {}
impl _wxList for wxFontList {}
impl _wxObject for wxFontList { fn handle(&self) -> *u8 { **self } }

impl wxFontList {
    pub fn from(handle: *u8) -> @_wxFontList {
        @wxFontList(handle) as @_wxFontList
    }
    
}

trait _wxFontList : _wxList {
}

struct wxFontMapper(*u8);
impl _wxFontMapper for wxFontMapper { fn handle(&self) -> *u8 { **self } }

impl wxFontMapper {
    pub fn from(handle: *u8) -> @_wxFontMapper {
        @wxFontMapper(handle) as @_wxFontMapper
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxFontMapper {
        unsafe { @wxFontMapper(wxFontMapper_Create()) as @_wxFontMapper }
    }
}

trait _wxFontMapper {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *u8, _buf: @_wxString) -> bool {
        unsafe { wxFontMapper_GetAltForEncoding(self.handle(), encoding, alt_encoding, _buf.handle()) }
    }
    #[fixed_stack_segment]
    fn isEncodingAvailable(&self, encoding: c_int, _buf: @_wxString) -> bool {
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
    pub fn from(handle: *u8) -> @_wxFrame {
        @wxFrame(handle) as @_wxFrame
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxFrame {
        unsafe { @wxFrame(wxFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxFrame }
    }
}

trait _wxFrame : _wxTopLevelWindow {
    #[fixed_stack_segment]
    fn newStatusBar(&self, number: c_int, style: c_int) -> @_wxStatusBar {
        unsafe { @wxStatusBar(wxFrame_CreateStatusBar(self.handle(), number, style)) as @_wxStatusBar }
    }
    #[fixed_stack_segment]
    fn newToolBar(&self, style: c_long) -> @_wxToolBar {
        unsafe { @wxToolBar(wxFrame_CreateToolBar(self.handle(), style)) as @_wxToolBar }
    }
    #[fixed_stack_segment]
    fn getClientAreaOrigin_left(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientAreaOrigin_top(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> @_wxMenuBar {
        unsafe { @wxMenuBar(wxFrame_GetMenuBar(self.handle())) as @_wxMenuBar }
    }
    #[fixed_stack_segment]
    fn getStatusBar(&self) -> @_wxStatusBar {
        unsafe { @wxStatusBar(wxFrame_GetStatusBar(self.handle())) as @_wxStatusBar }
    }
    #[fixed_stack_segment]
    fn getToolBar(&self) -> @_wxToolBar {
        unsafe { @wxToolBar(wxFrame_GetToolBar(self.handle())) as @_wxToolBar }
    }
    #[fixed_stack_segment]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMenuBar(&self, menubar: @_wxMenuBar) {
        unsafe { wxFrame_SetMenuBar(self.handle(), menubar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusBar(&self, statBar: @_wxStatusBar) {
        unsafe { wxFrame_SetStatusBar(self.handle(), statBar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusText(&self, _txt: @_wxString, _number: c_int) {
        unsafe { wxFrame_SetStatusText(self.handle(), _txt.handle(), _number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, _n: c_int, _widths_field: *u8) {
        unsafe { wxFrame_SetStatusWidths(self.handle(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    fn setToolBar(&self, _toolbar: @_wxToolBar) {
        unsafe { wxFrame_SetToolBar(self.handle(), _toolbar.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @_wxString {
        unsafe { @wxString(wxFrame_GetTitle(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, _txt: @_wxString) {
        unsafe { wxFrame_SetTitle(self.handle(), _txt.handle()) }
    }
    #[fixed_stack_segment]
    fn setShape(&self, region: @_wxRegion) -> bool {
        unsafe { wxFrame_SetShape(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn showFullScreen(&self, show: bool, style: c_int) -> bool {
        unsafe { wxFrame_ShowFullScreen(self.handle(), show, style) }
    }
    #[fixed_stack_segment]
    fn isFullScreen(&self) -> bool {
        unsafe { wxFrame_IsFullScreen(self.handle()) }
    }
    #[fixed_stack_segment]
    fn centre(&self, orientation: c_int) {
        unsafe { wxFrame_Centre(self.handle(), orientation) }
    }
}

struct wxFrameLayout(*u8);
impl _wxFrameLayout for wxFrameLayout {}
impl _wxEvtHandler for wxFrameLayout {}
impl _wxObject for wxFrameLayout { fn handle(&self) -> *u8 { **self } }

impl wxFrameLayout {
    pub fn from(handle: *u8) -> @_wxFrameLayout {
        @wxFrameLayout(handle) as @_wxFrameLayout
    }
    
}

trait _wxFrameLayout : _wxEvtHandler {
}

struct wxGDIObject(*u8);
impl _wxGDIObject for wxGDIObject {}
impl _wxObject for wxGDIObject { fn handle(&self) -> *u8 { **self } }

impl wxGDIObject {
    pub fn from(handle: *u8) -> @_wxGDIObject {
        @wxGDIObject(handle) as @_wxGDIObject
    }
    
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
    pub fn from(handle: *u8) -> @_wxGLCanvas {
        @wxGLCanvas(handle) as @_wxGLCanvas
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, windowID: c_int, attributes: *c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: @_wxString, palette: @_wxPalette) -> @_wxGLCanvas {
        unsafe { @wxGLCanvas(wxGLCanvas_Create(parent.handle(), windowID, attributes, x, y, w, h, style, title.handle(), palette.handle())) as @_wxGLCanvas }
    }
    #[fixed_stack_segment]
    pub fn isDisplaySupported(attributes: *c_int) -> bool {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    #[fixed_stack_segment]
    pub fn isExtensionSupported(extension: @_wxString) -> bool {
        unsafe { wxGLCanvas_IsExtensionSupported(extension.handle()) }
    }
}

trait _wxGLCanvas : _wxScrolledWindow {
    #[fixed_stack_segment]
    fn setColour(&self, colour: @_wxColour) -> bool {
        unsafe { wxGLCanvas_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, ctxt: @_wxGLContext) -> bool {
        unsafe { wxGLCanvas_SetCurrent(self.handle(), ctxt.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxGauge {
        @wxGauge(handle) as @_wxGauge
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxGauge {
        unsafe { @wxGauge(wxGauge_Create(_prt.handle(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) as @_wxGauge }
    }
}

trait _wxGauge : _wxControl {
    #[fixed_stack_segment]
    fn getBezelFace(&self) -> c_int {
        unsafe { wxGauge_GetBezelFace(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRange(&self) -> c_int {
        unsafe { wxGauge_GetRange(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getShadowWidth(&self) -> c_int {
        unsafe { wxGauge_GetShadowWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> c_int {
        unsafe { wxGauge_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBezelFace(&self, w: c_int) {
        unsafe { wxGauge_SetBezelFace(self.handle(), w) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, r: c_int) {
        unsafe { wxGauge_SetRange(self.handle(), r) }
    }
    #[fixed_stack_segment]
    fn setShadowWidth(&self, w: c_int) {
        unsafe { wxGauge_SetShadowWidth(self.handle(), w) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxGenericDirCtrl {
        @wxGenericDirCtrl(handle) as @_wxGenericDirCtrl
    }
    
}

trait _wxGenericDirCtrl : _wxControl {
}

struct wxGenericValidator(*u8);
impl _wxGenericValidator for wxGenericValidator {}
impl _wxValidator for wxGenericValidator {}
impl _wxEvtHandler for wxGenericValidator {}
impl _wxObject for wxGenericValidator { fn handle(&self) -> *u8 { **self } }

impl wxGenericValidator {
    pub fn from(handle: *u8) -> @_wxGenericValidator {
        @wxGenericValidator(handle) as @_wxGenericValidator
    }
    
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
    pub fn from(handle: *u8) -> @_wxGrid {
        @wxGrid(handle) as @_wxGrid
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxGrid {
        unsafe { @wxGrid(wxGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxGrid }
    }
}

trait _wxGrid : _wxScrolledWindow {
    #[fixed_stack_segment]
    fn appendCols(&self, numCols: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_AppendCols(self.handle(), numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    fn appendRows(&self, numRows: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_AppendRows(self.handle(), numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    fn autoSize(&self) {
        unsafe { wxGrid_AutoSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoSizeColumn(&self, col: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumn(self.handle(), col, setAsMin) }
    }
    #[fixed_stack_segment]
    fn autoSizeColumns(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumns(self.handle(), setAsMin) }
    }
    #[fixed_stack_segment]
    fn autoSizeRow(&self, row: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRow(self.handle(), row, setAsMin) }
    }
    #[fixed_stack_segment]
    fn autoSizeRows(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRows(self.handle(), setAsMin) }
    }
    #[fixed_stack_segment]
    fn beginBatch(&self) {
        unsafe { wxGrid_BeginBatch(self.handle()) }
    }
    #[fixed_stack_segment]
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> @_wxRect {
        unsafe { @wxRect(wxGrid_BlockToDeviceRect(self.handle(), top, left, bottom, right)) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn canDragColSize(&self) -> bool {
        unsafe { wxGrid_CanDragColSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canDragGridSize(&self) -> bool {
        unsafe { wxGrid_CanDragGridSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canDragRowSize(&self) -> bool {
        unsafe { wxGrid_CanDragRowSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canEnableCellControl(&self) -> bool {
        unsafe { wxGrid_CanEnableCellControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn cellToRect(&self, row: c_int, col: c_int) -> @_wxRect {
        unsafe { @wxRect(wxGrid_CellToRect(self.handle(), row, col)) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn clearGrid(&self) {
        unsafe { wxGrid_ClearGrid(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearSelection(&self) {
        unsafe { wxGrid_ClearSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn newGrid(&self, rows: c_int, cols: c_int, selmode: c_int) {
        unsafe { wxGrid_CreateGrid(self.handle(), rows, cols, selmode) }
    }
    #[fixed_stack_segment]
    fn deleteCols(&self, pos: c_int, numCols: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_DeleteCols(self.handle(), pos, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    fn deleteRows(&self, pos: c_int, numRows: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_DeleteRows(self.handle(), pos, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    fn disableCellEditControl(&self) {
        unsafe { wxGrid_DisableCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn disableDragColSize(&self) {
        unsafe { wxGrid_DisableDragColSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn disableDragGridSize(&self) {
        unsafe { wxGrid_DisableDragGridSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn disableDragRowSize(&self) {
        unsafe { wxGrid_DisableDragRowSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawAllGridLines(&self, dc: @_wxDC, reg: @_wxRegion) {
        unsafe { wxGrid_DrawAllGridLines(self.handle(), dc.handle(), reg.handle()) }
    }
    #[fixed_stack_segment]
    fn drawCell(&self, dc: @_wxDC, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellBorder(&self, dc: @_wxDC, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellHighlight(&self, dc: @_wxDC, attr: @_wxGridCellAttr) {
        unsafe { wxGrid_DrawCellHighlight(self.handle(), dc.handle(), attr.handle()) }
    }
    #[fixed_stack_segment]
    fn drawColLabel(&self, dc: @_wxDC, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.handle(), dc.handle(), col) }
    }
    #[fixed_stack_segment]
    fn drawColLabels(&self, dc: @_wxDC) {
        unsafe { wxGrid_DrawColLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawGridSpace(&self, dc: @_wxDC) {
        unsafe { wxGrid_DrawGridSpace(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawRowLabel(&self, dc: @_wxDC, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.handle(), dc.handle(), row) }
    }
    #[fixed_stack_segment]
    fn drawRowLabels(&self, dc: @_wxDC) {
        unsafe { wxGrid_DrawRowLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawTextRectangle(&self, dc: @_wxDC, txt: @_wxString, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
        unsafe { wxGrid_DrawTextRectangle(self.handle(), dc.handle(), txt.handle(), x, y, w, h, horizontalAlignment, verticalAlignment) }
    }
    #[fixed_stack_segment]
    fn enableCellEditControl(&self, enable: bool) {
        unsafe { wxGrid_EnableCellEditControl(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn enableDragColSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragColSize(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn enableDragGridSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragGridSize(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn enableDragRowSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragRowSize(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn enableEditing(&self, edit: c_int) {
        unsafe { wxGrid_EnableEditing(self.handle(), edit) }
    }
    #[fixed_stack_segment]
    fn enableGridLines(&self, enable: bool) {
        unsafe { wxGrid_EnableGridLines(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn endBatch(&self) {
        unsafe { wxGrid_EndBatch(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBatchCount(&self) -> c_int {
        unsafe { wxGrid_GetBatchCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellAlignment(&self, row: c_int, col: c_int, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetCellAlignment(self.handle(), row, col, horiz, vert) }
    }
    #[fixed_stack_segment]
    fn getCellBackgroundColour(&self, row: c_int, col: c_int, colour: @_wxColour) {
        unsafe { wxGrid_GetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellEditor(&self, row: c_int, col: c_int) -> @_wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetCellEditor(self.handle(), row, col)) as @_wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getCellFont(&self, row: c_int, col: c_int, font: @_wxFont) {
        unsafe { wxGrid_GetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellHighlightColour(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetCellHighlightColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellRenderer(&self, row: c_int, col: c_int) -> @_wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetCellRenderer(self.handle(), row, col)) as @_wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getCellTextColour(&self, row: c_int, col: c_int, colour: @_wxColour) {
        unsafe { wxGrid_GetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellValue(&self, row: c_int, col: c_int) -> @_wxString {
        unsafe { @wxString(wxGrid_GetCellValue(self.handle(), row, col)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getColLabelAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetColLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    fn getColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetColLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColLabelValue(&self, col: c_int) -> @_wxString {
        unsafe { @wxString(wxGrid_GetColLabelValue(self.handle(), col)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getColSize(&self, col: c_int) -> c_int {
        unsafe { wxGrid_GetColSize(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetDefaultCellAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellBackgroundColour(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellFont(&self, _ref: @_wxFont) {
        unsafe { wxGrid_GetDefaultCellFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellTextColour(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetDefaultCellTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultColSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultEditor(&self) -> @_wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditor(self.handle())) as @_wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> @_wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditorForCell(self.handle(), row, col)) as @_wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForType(&self, typeName: @_wxString) -> @_wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditorForType(self.handle(), typeName.handle())) as @_wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @_wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRenderer(self.handle())) as @_wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> @_wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRendererForCell(self.handle(), row, col)) as @_wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForType(&self, typeName: @_wxString) -> @_wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRendererForType(self.handle(), typeName.handle())) as @_wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultRowSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getGridCursorCol(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getGridCursorRow(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getGridLineColour(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetGridLineColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelBackgroundColour(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelFont(&self, _ref: @_wxFont) {
        unsafe { wxGrid_GetLabelFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelTextColour(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetLabelTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getNumberCols(&self) -> c_int {
        unsafe { wxGrid_GetNumberCols(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getNumberRows(&self) -> c_int {
        unsafe { wxGrid_GetNumberRows(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRowLabelAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetRowLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    fn getRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetRowLabelSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRowLabelValue(&self, row: c_int) -> @_wxString {
        unsafe { @wxString(wxGrid_GetRowLabelValue(self.handle(), row)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.handle(), row) }
    }
    #[fixed_stack_segment]
    fn getSelectionBackground(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetSelectionBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionForeground(&self, _ref: @_wxColour) {
        unsafe { wxGrid_GetSelectionForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTable(&self) -> @_wxGridTableBase {
        unsafe { @wxGridTableBase(wxGrid_GetTable(self.handle())) as @_wxGridTableBase }
    }
    #[fixed_stack_segment]
    fn getTextBoxSize(&self, dc: @_wxDC, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int) {
        unsafe { wxGrid_GetTextBoxSize(self.handle(), dc.handle(), count, lines, _w, _h) }
    }
    #[fixed_stack_segment]
    fn gridLinesEnabled(&self) -> c_int {
        unsafe { wxGrid_GridLinesEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hideCellEditControl(&self) {
        unsafe { wxGrid_HideCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insertCols(&self, pos: c_int, numCols: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_InsertCols(self.handle(), pos, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    fn insertRows(&self, pos: c_int, numRows: c_int, updateLabels: bool) -> bool {
        unsafe { wxGrid_InsertRows(self.handle(), pos, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    fn isCellEditControlEnabled(&self) -> bool {
        unsafe { wxGrid_IsCellEditControlEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isCellEditControlShown(&self) -> bool {
        unsafe { wxGrid_IsCellEditControlShown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isCurrentCellReadOnly(&self) -> bool {
        unsafe { wxGrid_IsCurrentCellReadOnly(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEditable(&self) -> bool {
        unsafe { wxGrid_IsEditable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isInSelection(&self, row: c_int, col: c_int) -> bool {
        unsafe { wxGrid_IsInSelection(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    fn isReadOnly(&self, row: c_int, col: c_int) -> bool {
        unsafe { wxGrid_IsReadOnly(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    fn isSelection(&self) -> bool {
        unsafe { wxGrid_IsSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self, row: c_int, col: c_int, wholeCellVisible: bool) -> bool {
        unsafe { wxGrid_IsVisible(self.handle(), row, col, wholeCellVisible) }
    }
    #[fixed_stack_segment]
    fn makeCellVisible(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_MakeCellVisible(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    fn moveCursorDown(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorDown(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorDownBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorDownBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorLeft(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorLeft(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorLeftBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorLeftBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorRight(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorRight(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorRightBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorRightBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorUp(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorUp(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorUpBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorUpBlock(self.handle(), expandSelection) }
    }
    #[fixed_stack_segment]
    fn movePageDown(&self) -> bool {
        unsafe { wxGrid_MovePageDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn movePageUp(&self) -> bool {
        unsafe { wxGrid_MovePageUp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn processTableMessage(&self, evt: @_wxEvent) -> bool {
        unsafe { wxGrid_ProcessTableMessage(self.handle(), evt.handle()) }
    }
    #[fixed_stack_segment]
    fn registerDataType(&self, typeName: @_wxString, renderer: @_wxGridCellRenderer, editor: @_wxGridCellEditor) {
        unsafe { wxGrid_RegisterDataType(self.handle(), typeName.handle(), renderer.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn saveEditControlValue(&self) {
        unsafe { wxGrid_SaveEditControlValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selectAll(&self) {
        unsafe { wxGrid_SelectAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selectBlock(&self, topRow: c_int, leftCol: c_int, bottomRow: c_int, rightCol: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectBlock(self.handle(), topRow, leftCol, bottomRow, rightCol, addToSelected) }
    }
    #[fixed_stack_segment]
    fn selectCol(&self, col: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectCol(self.handle(), col, addToSelected) }
    }
    #[fixed_stack_segment]
    fn selectRow(&self, row: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectRow(self.handle(), row, addToSelected) }
    }
    #[fixed_stack_segment]
    fn setCellAlignment(&self, row: c_int, col: c_int, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetCellAlignment(self.handle(), row, col, horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setCellBackgroundColour(&self, row: c_int, col: c_int, colour: @_wxColour) {
        unsafe { wxGrid_SetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellEditor(&self, row: c_int, col: c_int, editor: @_wxGridCellEditor) {
        unsafe { wxGrid_SetCellEditor(self.handle(), row, col, editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellFont(&self, row: c_int, col: c_int, font: @_wxFont) {
        unsafe { wxGrid_SetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellHighlightColour(&self, col: @_wxColour) {
        unsafe { wxGrid_SetCellHighlightColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellRenderer(&self, row: c_int, col: c_int, renderer: @_wxGridCellRenderer) {
        unsafe { wxGrid_SetCellRenderer(self.handle(), row, col, renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellTextColour(&self, row: c_int, col: c_int, colour: @_wxColour) {
        unsafe { wxGrid_SetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellValue(&self, row: c_int, col: c_int, s: @_wxString) {
        unsafe { wxGrid_SetCellValue(self.handle(), row, col, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setColAttr(&self, col: c_int, attr: @_wxGridCellAttr) {
        unsafe { wxGrid_SetColAttr(self.handle(), col, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setColFormatCustom(&self, col: c_int, typeName: @_wxString) {
        unsafe { wxGrid_SetColFormatCustom(self.handle(), col, typeName.handle()) }
    }
    #[fixed_stack_segment]
    fn setColFormatFloat(&self, col: c_int, width: c_int, precision: c_int) {
        unsafe { wxGrid_SetColFormatFloat(self.handle(), col, width, precision) }
    }
    #[fixed_stack_segment]
    fn setColFormatNumber(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatNumber(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setColLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetColLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setColLabelSize(&self, height: c_int) {
        unsafe { wxGrid_SetColLabelSize(self.handle(), height) }
    }
    #[fixed_stack_segment]
    fn setColLabelValue(&self, col: c_int, label: @_wxString) {
        unsafe { wxGrid_SetColLabelValue(self.handle(), col, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setColMinimalWidth(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColMinimalWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    fn setColSize(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColSize(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetDefaultCellAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellBackgroundColour(&self, colour: @_wxColour) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellFont(&self, font: @_wxFont) {
        unsafe { wxGrid_SetDefaultCellFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellTextColour(&self, colour: @_wxColour) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.handle(), width, resizeExistingCols) }
    }
    #[fixed_stack_segment]
    fn setDefaultEditor(&self, editor: @_wxGridCellEditor) {
        unsafe { wxGrid_SetDefaultEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultRenderer(&self, renderer: @_wxGridCellRenderer) {
        unsafe { wxGrid_SetDefaultRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultRowSize(&self, height: c_int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self.handle(), height, resizeExistingRows) }
    }
    #[fixed_stack_segment]
    fn setGridCursor(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_SetGridCursor(self.handle(), row, col) }
    }
    #[fixed_stack_segment]
    fn setGridLineColour(&self, col: @_wxColour) {
        unsafe { wxGrid_SetGridLineColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelBackgroundColour(&self, colour: @_wxColour) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelFont(&self, font: @_wxFont) {
        unsafe { wxGrid_SetLabelFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTextColour(&self, colour: @_wxColour) {
        unsafe { wxGrid_SetLabelTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, extraWidth: c_int, extraHeight: c_int) {
        unsafe { wxGrid_SetMargins(self.handle(), extraWidth, extraHeight) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, row: c_int, col: c_int, isReadOnly: bool) {
        unsafe { wxGrid_SetReadOnly(self.handle(), row, col, isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRowAttr(&self, row: c_int, attr: @_wxGridCellAttr) {
        unsafe { wxGrid_SetRowAttr(self.handle(), row, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn setRowLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetRowLabelAlignment(self.handle(), horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setRowLabelSize(&self, width: c_int) {
        unsafe { wxGrid_SetRowLabelSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn setRowLabelValue(&self, row: c_int, label: @_wxString) {
        unsafe { wxGrid_SetRowLabelValue(self.handle(), row, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setRowMinimalHeight(&self, row: c_int, width: c_int) {
        unsafe { wxGrid_SetRowMinimalHeight(self.handle(), row, width) }
    }
    #[fixed_stack_segment]
    fn setRowSize(&self, row: c_int, height: c_int) {
        unsafe { wxGrid_SetRowSize(self.handle(), row, height) }
    }
    #[fixed_stack_segment]
    fn setSelectionBackground(&self, c: @_wxColour) {
        unsafe { wxGrid_SetSelectionBackground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionForeground(&self, c: @_wxColour) {
        unsafe { wxGrid_SetSelectionForeground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.handle(), selmode) }
    }
    #[fixed_stack_segment]
    fn setTable(&self, table: @_wxGridTableBase, takeOwnership: bool, selmode: c_int) -> bool {
        unsafe { wxGrid_SetTable(self.handle(), table.handle(), takeOwnership, selmode) }
    }
    #[fixed_stack_segment]
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn stringToLines(&self, value: @_wxString, lines: *u8) -> c_int {
        unsafe { wxGrid_StringToLines(self.handle(), value.handle(), lines) }
    }
    #[fixed_stack_segment]
    fn xToCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToCol(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn xToEdgeOfCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToEdgeOfCol(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn xYToCell(&self, x: c_int, y: c_int, row: *c_int, col: *c_int) {
        unsafe { wxGrid_XYToCell(self.handle(), x, y, row, col) }
    }
    #[fixed_stack_segment]
    fn yToEdgeOfRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToEdgeOfRow(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn yToRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToRow(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn getSelectedCells(&self, _arr: @_wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectedCells(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockTopLeft(&self, _arr: @_wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockBottomRight(&self, _arr: @_wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectionBlockBottomRight(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectedRows(&self, _arr: *intptr_t) -> c_int {
        unsafe { wxGrid_GetSelectedRows(self.handle(), _arr) }
    }
    #[fixed_stack_segment]
    fn getSelectedCols(&self, _arr: *intptr_t) -> c_int {
        unsafe { wxGrid_GetSelectedCols(self.handle(), _arr) }
    }
    #[fixed_stack_segment]
    fn getCellSize(&self, row: c_int, col: c_int, srow: *c_int, scol: *c_int) {
        unsafe { wxGrid_GetCellSize(self.handle(), row, col, srow, scol) }
    }
    #[fixed_stack_segment]
    fn setCellSize(&self, row: c_int, col: c_int, srow: c_int, scol: c_int) {
        unsafe { wxGrid_SetCellSize(self.handle(), row, col, srow, scol) }
    }
}

struct wxGridCellAttr(*u8);
impl _wxGridCellAttr for wxGridCellAttr { fn handle(&self) -> *u8 { **self } }

impl wxGridCellAttr {
    pub fn from(handle: *u8) -> @_wxGridCellAttr {
        @wxGridCellAttr(handle) as @_wxGridCellAttr
    }
    
    #[fixed_stack_segment]
    pub fn ctor() -> @_wxGridCellAttr {
        unsafe { @wxGridCellAttr(wxGridCellAttr_Ctor()) as @_wxGridCellAttr }
    }
}

trait _wxGridCellAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self, hAlign: *c_int, vAlign: *c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.handle(), hAlign, vAlign) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @_wxColour) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditor(&self, grid: @_wxGrid, row: c_int, col: c_int) -> @_wxGridCellEditor {
        unsafe { @wxGridCellEditor(wxGridCellAttr_GetEditor(self.handle(), grid.handle(), row, col)) as @_wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @_wxFont) {
        unsafe { wxGridCellAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getRenderer(&self, grid: @_wxGrid, row: c_int, col: c_int) -> @_wxGridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGridCellAttr_GetRenderer(self.handle(), grid.handle(), row, col)) as @_wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @_wxColour) {
        unsafe { wxGridCellAttr_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn hasAlignment(&self) -> bool {
        unsafe { wxGridCellAttr_HasAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxGridCellAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasEditor(&self) -> bool {
        unsafe { wxGridCellAttr_HasEditor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasFont(&self) -> bool {
        unsafe { wxGridCellAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasRenderer(&self) -> bool {
        unsafe { wxGridCellAttr_HasRenderer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasTextColour(&self) -> bool {
        unsafe { wxGridCellAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn incRef(&self) {
        unsafe { wxGridCellAttr_IncRef(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isReadOnly(&self) -> bool {
        unsafe { wxGridCellAttr_IsReadOnly(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, hAlign: c_int, vAlign: c_int) {
        unsafe { wxGridCellAttr_SetAlignment(self.handle(), hAlign, vAlign) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colBack: @_wxColour) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefAttr(&self, defAttr: @_wxGridCellAttr) {
        unsafe { wxGridCellAttr_SetDefAttr(self.handle(), defAttr.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditor(&self, editor: @_wxGridCellEditor) {
        unsafe { wxGridCellAttr_SetEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont) {
        unsafe { wxGridCellAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, isReadOnly: bool) {
        unsafe { wxGridCellAttr_SetReadOnly(self.handle(), isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRenderer(&self, renderer: @_wxGridCellRenderer) {
        unsafe { wxGridCellAttr_SetRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @_wxColour) {
        unsafe { wxGridCellAttr_SetTextColour(self.handle(), colText.handle()) }
    }
}

struct wxGridCellBoolEditor(*u8);
impl _wxGridCellBoolEditor for wxGridCellBoolEditor {}
impl _wxGridCellEditor for wxGridCellBoolEditor {}
impl _wxGridCellWorker for wxGridCellBoolEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolEditor {
    pub fn from(handle: *u8) -> @_wxGridCellBoolEditor {
        @wxGridCellBoolEditor(handle) as @_wxGridCellBoolEditor
    }
    
    #[fixed_stack_segment]
    pub fn ctor() -> @_wxGridCellBoolEditor {
        unsafe { @wxGridCellBoolEditor(wxGridCellBoolEditor_Ctor()) as @_wxGridCellBoolEditor }
    }
}

trait _wxGridCellBoolEditor : _wxGridCellEditor {
}

struct wxGridCellBoolRenderer(*u8);
impl _wxGridCellBoolRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellWorker for wxGridCellBoolRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolRenderer {
    pub fn from(handle: *u8) -> @_wxGridCellBoolRenderer {
        @wxGridCellBoolRenderer(handle) as @_wxGridCellBoolRenderer
    }
    
}

trait _wxGridCellBoolRenderer : _wxGridCellRenderer {
}

struct wxGridCellChoiceEditor(*u8);
impl _wxGridCellChoiceEditor for wxGridCellChoiceEditor {}
impl _wxGridCellEditor for wxGridCellChoiceEditor {}
impl _wxGridCellWorker for wxGridCellChoiceEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellChoiceEditor {
    pub fn from(handle: *u8) -> @_wxGridCellChoiceEditor {
        @wxGridCellChoiceEditor(handle) as @_wxGridCellChoiceEditor
    }
    
    #[fixed_stack_segment]
    pub fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> @_wxGridCellChoiceEditor {
        unsafe { @wxGridCellChoiceEditor(wxGridCellChoiceEditor_Ctor(count, choices, allowOthers)) as @_wxGridCellChoiceEditor }
    }
}

trait _wxGridCellChoiceEditor : _wxGridCellEditor {
}

struct wxGridCellCoordsArray(*u8);
impl _wxGridCellCoordsArray for wxGridCellCoordsArray { fn handle(&self) -> *u8 { **self } }

impl wxGridCellCoordsArray {
    pub fn from(handle: *u8) -> @_wxGridCellCoordsArray {
        @wxGridCellCoordsArray(handle) as @_wxGridCellCoordsArray
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxGridCellCoordsArray {
        unsafe { @wxGridCellCoordsArray(wxGridCellCoordsArray_Create()) as @_wxGridCellCoordsArray }
    }
}

trait _wxGridCellCoordsArray {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGridCellCoordsArray_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxGridCellCoordsArray_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn item(&self, _idx: c_int, _c: *c_int, _r: *c_int) {
        unsafe { wxGridCellCoordsArray_Item(self.handle(), _idx, _c, _r) }
    }
}

struct wxGridCellEditor(*u8);
impl _wxGridCellEditor for wxGridCellEditor {}
impl _wxGridCellWorker for wxGridCellEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellEditor {
    pub fn from(handle: *u8) -> @_wxGridCellEditor {
        @wxGridCellEditor(handle) as @_wxGridCellEditor
    }
    
}

trait _wxGridCellEditor : _wxGridCellWorker {
    #[fixed_stack_segment]
    fn beginEdit(&self, row: c_int, col: c_int, grid: @_wxGrid) {
        unsafe { wxGridCellEditor_BeginEdit(self.handle(), row, col, grid.handle()) }
    }
    #[fixed_stack_segment]
    fn new(&self, parent: @_wxWindow, id: c_int, evtHandler: @_wxEvtHandler) {
        unsafe { wxGridCellEditor_Create(self.handle(), parent.handle(), id, evtHandler.handle()) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endEdit(&self, row: c_int, col: c_int, grid: @_wxGrid, oldStr: @_wxString, newStr: @_wxString) -> c_int {
        unsafe { wxGridCellEditor_EndEdit(self.handle(), row, col, grid.handle(), oldStr.handle(), newStr.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @_wxControl {
        unsafe { @wxControl(wxGridCellEditor_GetControl(self.handle())) as @_wxControl }
    }
    #[fixed_stack_segment]
    fn handleReturn(&self, event: @_wxEvent) {
        unsafe { wxGridCellEditor_HandleReturn(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isAcceptedKey(&self, event: @_wxEvent) -> bool {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isCreated(&self) -> bool {
        unsafe { wxGridCellEditor_IsCreated(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paintBackground(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: @_wxGridCellAttr) {
        unsafe { wxGridCellEditor_PaintBackground(self.handle(), x, y, w, h, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setControl(&self, control: @_wxControl) {
        unsafe { wxGridCellEditor_SetControl(self.handle(), control.handle()) }
    }
    #[fixed_stack_segment]
    fn setParameters(&self, params: @_wxString) {
        unsafe { wxGridCellEditor_SetParameters(self.handle(), params.handle()) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn show(&self, show: c_int, attr: @_wxGridCellAttr) {
        unsafe { wxGridCellEditor_Show(self.handle(), show, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn startingKey(&self, event: @_wxEvent) {
        unsafe { wxGridCellEditor_StartingKey(self.handle(), event.handle()) }
    }
}

struct wxGridCellFloatEditor(*u8);
impl _wxGridCellFloatEditor for wxGridCellFloatEditor {}
impl _wxGridCellTextEditor for wxGridCellFloatEditor {}
impl _wxGridCellEditor for wxGridCellFloatEditor {}
impl _wxGridCellWorker for wxGridCellFloatEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellFloatEditor {
    pub fn from(handle: *u8) -> @_wxGridCellFloatEditor {
        @wxGridCellFloatEditor(handle) as @_wxGridCellFloatEditor
    }
    
    #[fixed_stack_segment]
    pub fn ctor(width: c_int, precision: c_int) -> @_wxGridCellFloatEditor {
        unsafe { @wxGridCellFloatEditor(wxGridCellFloatEditor_Ctor(width, precision)) as @_wxGridCellFloatEditor }
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
    pub fn from(handle: *u8) -> @_wxGridCellFloatRenderer {
        @wxGridCellFloatRenderer(handle) as @_wxGridCellFloatRenderer
    }
    
}

trait _wxGridCellFloatRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellNumberEditor(*u8);
impl _wxGridCellNumberEditor for wxGridCellNumberEditor {}
impl _wxGridCellTextEditor for wxGridCellNumberEditor {}
impl _wxGridCellEditor for wxGridCellNumberEditor {}
impl _wxGridCellWorker for wxGridCellNumberEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellNumberEditor {
    pub fn from(handle: *u8) -> @_wxGridCellNumberEditor {
        @wxGridCellNumberEditor(handle) as @_wxGridCellNumberEditor
    }
    
    #[fixed_stack_segment]
    pub fn ctor(min: c_int, max: c_int) -> @_wxGridCellNumberEditor {
        unsafe { @wxGridCellNumberEditor(wxGridCellNumberEditor_Ctor(min, max)) as @_wxGridCellNumberEditor }
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
    pub fn from(handle: *u8) -> @_wxGridCellNumberRenderer {
        @wxGridCellNumberRenderer(handle) as @_wxGridCellNumberRenderer
    }
    
    #[fixed_stack_segment]
    pub fn ctor() -> @_wxGridCellNumberRenderer {
        unsafe { @wxGridCellNumberRenderer(wxGridCellNumberRenderer_Ctor()) as @_wxGridCellNumberRenderer }
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
    pub fn from(handle: *u8) -> @_wxGridCellAutoWrapStringRenderer {
        @wxGridCellAutoWrapStringRenderer(handle) as @_wxGridCellAutoWrapStringRenderer
    }
    
    #[fixed_stack_segment]
    pub fn ctor() -> @_wxGridCellAutoWrapStringRenderer {
        unsafe { @wxGridCellAutoWrapStringRenderer(wxGridCellAutoWrapStringRenderer_Ctor()) as @_wxGridCellAutoWrapStringRenderer }
    }
}

trait _wxGridCellAutoWrapStringRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellRenderer(*u8);
impl _wxGridCellRenderer for wxGridCellRenderer {}
impl _wxGridCellWorker for wxGridCellRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellRenderer {
    pub fn from(handle: *u8) -> @_wxGridCellRenderer {
        @wxGridCellRenderer(handle) as @_wxGridCellRenderer
    }
    
}

trait _wxGridCellRenderer : _wxGridCellWorker {
}

struct wxGridCellStringRenderer(*u8);
impl _wxGridCellStringRenderer for wxGridCellStringRenderer {}
impl _wxGridCellRenderer for wxGridCellStringRenderer {}
impl _wxGridCellWorker for wxGridCellStringRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellStringRenderer {
    pub fn from(handle: *u8) -> @_wxGridCellStringRenderer {
        @wxGridCellStringRenderer(handle) as @_wxGridCellStringRenderer
    }
    
}

trait _wxGridCellStringRenderer : _wxGridCellRenderer {
}

struct wxGridCellTextEditor(*u8);
impl _wxGridCellTextEditor for wxGridCellTextEditor {}
impl _wxGridCellEditor for wxGridCellTextEditor {}
impl _wxGridCellWorker for wxGridCellTextEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellTextEditor {
    pub fn from(handle: *u8) -> @_wxGridCellTextEditor {
        @wxGridCellTextEditor(handle) as @_wxGridCellTextEditor
    }
    
    #[fixed_stack_segment]
    pub fn ctor() -> @_wxGridCellTextEditor {
        unsafe { @wxGridCellTextEditor(wxGridCellTextEditor_Ctor()) as @_wxGridCellTextEditor }
    }
}

trait _wxGridCellTextEditor : _wxGridCellEditor {
}

struct wxGridCellWorker(*u8);
impl _wxGridCellWorker for wxGridCellWorker { fn handle(&self) -> *u8 { **self } }

impl wxGridCellWorker {
    pub fn from(handle: *u8) -> @_wxGridCellWorker {
        @wxGridCellWorker(handle) as @_wxGridCellWorker
    }
    
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
    pub fn from(handle: *u8) -> @_wxGridEditorCreatedEvent {
        @wxGridEditorCreatedEvent(handle) as @_wxGridEditorCreatedEvent
    }
    
}

trait _wxGridEditorCreatedEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @_wxControl {
        unsafe { @wxControl(wxGridEditorCreatedEvent_GetControl(self.handle())) as @_wxControl }
    }
    #[fixed_stack_segment]
    fn getRow(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCol(&self, col: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setControl(&self, ctrl: @_wxControl) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxGridEvent {
        @wxGridEvent(handle) as @_wxGridEvent
    }
    
}

trait _wxGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxGridEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxGridEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxGridEvent_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getRow(&self) -> c_int {
        unsafe { wxGridEvent_GetRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxGridEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selecting(&self) -> bool {
        unsafe { wxGridEvent_Selecting(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxGridRangeSelectEvent {
        @wxGridRangeSelectEvent(handle) as @_wxGridRangeSelectEvent
    }
    
}

trait _wxGridRangeSelectEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn getTopLeftCoords(&self, col: *c_int, row: *c_int) {
        unsafe { wxGridRangeSelectEvent_GetTopLeftCoords(self.handle(), col, row) }
    }
    #[fixed_stack_segment]
    fn getBottomRightCoords(&self, col: *c_int, row: *c_int) {
        unsafe { wxGridRangeSelectEvent_GetBottomRightCoords(self.handle(), col, row) }
    }
    #[fixed_stack_segment]
    fn getTopRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetTopRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBottomRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetBottomRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLeftCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetLeftCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRightCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetRightCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selecting(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_Selecting(self.handle()) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_ShiftDown(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxGridSizeEvent {
        @wxGridSizeEvent(handle) as @_wxGridSizeEvent
    }
    
}

trait _wxGridSizeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxGridSizeEvent_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxGridSizeEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxGridSizeEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridSizeEvent_ShiftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxGridSizeEvent_AltDown(self.handle()) }
    }
}

struct wxGridSizer(*u8);
impl _wxGridSizer for wxGridSizer {}
impl _wxSizer for wxGridSizer {}
impl _wxObject for wxGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxGridSizer {
    pub fn from(handle: *u8) -> @_wxGridSizer {
        @wxGridSizer(handle) as @_wxGridSizer
    }
    
    #[fixed_stack_segment]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @_wxGridSizer {
        unsafe { @wxGridSizer(wxGridSizer_Create(rows, cols, vgap, hgap)) as @_wxGridSizer }
    }
}

trait _wxGridSizer : _wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @_wxSize {
        unsafe { @wxSize(wxGridSizer_CalcMin(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getCols(&self) -> c_int {
        unsafe { wxGridSizer_GetCols(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHGap(&self) -> c_int {
        unsafe { wxGridSizer_GetHGap(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRows(&self) -> c_int {
        unsafe { wxGridSizer_GetRows(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVGap(&self) -> c_int {
        unsafe { wxGridSizer_GetVGap(self.handle()) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxGridSizer_RecalcSizes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCols(&self, cols: c_int) {
        unsafe { wxGridSizer_SetCols(self.handle(), cols) }
    }
    #[fixed_stack_segment]
    fn setHGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetHGap(self.handle(), gap) }
    }
    #[fixed_stack_segment]
    fn setRows(&self, rows: c_int) {
        unsafe { wxGridSizer_SetRows(self.handle(), rows) }
    }
    #[fixed_stack_segment]
    fn setVGap(&self, gap: c_int) {
        unsafe { wxGridSizer_SetVGap(self.handle(), gap) }
    }
}

struct wxGridTableBase(*u8);
impl _wxGridTableBase for wxGridTableBase {}
impl _wxObject for wxGridTableBase { fn handle(&self) -> *u8 { **self } }

impl wxGridTableBase {
    pub fn from(handle: *u8) -> @_wxGridTableBase {
        @wxGridTableBase(handle) as @_wxGridTableBase
    }
    
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
    pub fn from(handle: *u8) -> @_wxHTTP {
        @wxHTTP(handle) as @_wxHTTP
    }
    
}

trait _wxHTTP : _wxProtocol {
}

struct wxHashMap(*u8);
impl _wxHashMap for wxHashMap { fn handle(&self) -> *u8 { **self } }

impl wxHashMap {
    pub fn from(handle: *u8) -> @_wxHashMap {
        @wxHashMap(handle) as @_wxHashMap
    }
    
}

trait _wxHashMap {
    fn handle(&self) -> *u8;
    
}

struct wxHelpController(*u8);
impl _wxHelpController for wxHelpController {}
impl _wxHelpControllerBase for wxHelpController {}
impl _wxObject for wxHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHelpController {
    pub fn from(handle: *u8) -> @_wxHelpController {
        @wxHelpController(handle) as @_wxHelpController
    }
    
}

trait _wxHelpController : _wxHelpControllerBase {
}

struct wxHelpControllerBase(*u8);
impl _wxHelpControllerBase for wxHelpControllerBase {}
impl _wxObject for wxHelpControllerBase { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerBase {
    pub fn from(handle: *u8) -> @_wxHelpControllerBase {
        @wxHelpControllerBase(handle) as @_wxHelpControllerBase
    }
    
}

trait _wxHelpControllerBase : _wxObject {
}

struct wxHelpControllerHelpProvider(*u8);
impl _wxHelpControllerHelpProvider for wxHelpControllerHelpProvider {}
impl _wxSimpleHelpProvider for wxHelpControllerHelpProvider {}
impl _wxHelpProvider for wxHelpControllerHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerHelpProvider {
    pub fn from(handle: *u8) -> @_wxHelpControllerHelpProvider {
        @wxHelpControllerHelpProvider(handle) as @_wxHelpControllerHelpProvider
    }
    
    #[fixed_stack_segment]
    pub fn new(ctr: @_wxHelpControllerBase) -> @_wxHelpControllerHelpProvider {
        unsafe { @wxHelpControllerHelpProvider(wxHelpControllerHelpProvider_Create(ctr.handle())) as @_wxHelpControllerHelpProvider }
    }
}

trait _wxHelpControllerHelpProvider : _wxSimpleHelpProvider {
    #[fixed_stack_segment]
    fn getHelpController(&self) -> @_wxHelpControllerBase {
        unsafe { @wxHelpControllerBase(wxHelpControllerHelpProvider_GetHelpController(self.handle())) as @_wxHelpControllerBase }
    }
    #[fixed_stack_segment]
    fn setHelpController(&self, hc: @_wxHelpController) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.handle(), hc.handle()) }
    }
}

struct wxHelpEvent(*u8);
impl _wxHelpEvent for wxHelpEvent {}
impl _wxCommandEvent for wxHelpEvent {}
impl _wxEvent for wxHelpEvent {}
impl _wxObject for wxHelpEvent { fn handle(&self) -> *u8 { **self } }

impl wxHelpEvent {
    pub fn from(handle: *u8) -> @_wxHelpEvent {
        @wxHelpEvent(handle) as @_wxHelpEvent
    }
    
}

trait _wxHelpEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getLink(&self) -> @_wxString {
        unsafe { @wxString(wxHelpEvent_GetLink(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxHelpEvent_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @_wxString {
        unsafe { @wxString(wxHelpEvent_GetTarget(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setLink(&self, link: @_wxString) {
        unsafe { wxHelpEvent_SetLink(self.handle(), link.handle()) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setTarget(&self, target: @_wxString) {
        unsafe { wxHelpEvent_SetTarget(self.handle(), target.handle()) }
    }
}

struct wxHelpProvider(*u8);
impl _wxHelpProvider for wxHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpProvider {
    pub fn from(handle: *u8) -> @_wxHelpProvider {
        @wxHelpProvider(handle) as @_wxHelpProvider
    }
    
    #[fixed_stack_segment]
    pub fn get() -> @_wxHelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Get()) as @_wxHelpProvider }
    }
}

trait _wxHelpProvider {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addHelp(&self, window: @_wxWindow, text: @_wxString) {
        unsafe { wxHelpProvider_AddHelp(self.handle(), window.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addHelpById(&self, id: c_int, text: @_wxString) {
        unsafe { wxHelpProvider_AddHelpById(self.handle(), id, text.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelp(&self, window: @_wxWindow) -> @_wxString {
        unsafe { @wxString(wxHelpProvider_GetHelp(self.handle(), window.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn removeHelp(&self, window: @_wxWindow) {
        unsafe { wxHelpProvider_RemoveHelp(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self) -> @_wxHelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Set(self.handle())) as @_wxHelpProvider }
    }
    #[fixed_stack_segment]
    fn showHelp(&self, window: @_wxWindow) -> bool {
        unsafe { wxHelpProvider_ShowHelp(self.handle(), window.handle()) }
    }
}

struct wxHtmlCell(*u8);
impl _wxHtmlCell for wxHtmlCell {}
impl _wxObject for wxHtmlCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlCell {
    pub fn from(handle: *u8) -> @_wxHtmlCell {
        @wxHtmlCell(handle) as @_wxHtmlCell
    }
    
}

trait _wxHtmlCell : _wxObject {
}

struct wxHtmlColourCell(*u8);
impl _wxHtmlColourCell for wxHtmlColourCell {}
impl _wxHtmlCell for wxHtmlColourCell {}
impl _wxObject for wxHtmlColourCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlColourCell {
    pub fn from(handle: *u8) -> @_wxHtmlColourCell {
        @wxHtmlColourCell(handle) as @_wxHtmlColourCell
    }
    
}

trait _wxHtmlColourCell : _wxHtmlCell {
}

struct wxHtmlContainerCell(*u8);
impl _wxHtmlContainerCell for wxHtmlContainerCell {}
impl _wxHtmlCell for wxHtmlContainerCell {}
impl _wxObject for wxHtmlContainerCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlContainerCell {
    pub fn from(handle: *u8) -> @_wxHtmlContainerCell {
        @wxHtmlContainerCell(handle) as @_wxHtmlContainerCell
    }
    
}

trait _wxHtmlContainerCell : _wxHtmlCell {
}

struct wxHtmlDCRenderer(*u8);
impl _wxHtmlDCRenderer for wxHtmlDCRenderer {}
impl _wxObject for wxHtmlDCRenderer { fn handle(&self) -> *u8 { **self } }

impl wxHtmlDCRenderer {
    pub fn from(handle: *u8) -> @_wxHtmlDCRenderer {
        @wxHtmlDCRenderer(handle) as @_wxHtmlDCRenderer
    }
    
}

trait _wxHtmlDCRenderer : _wxObject {
}

struct wxHtmlEasyPrinting(*u8);
impl _wxHtmlEasyPrinting for wxHtmlEasyPrinting {}
impl _wxObject for wxHtmlEasyPrinting { fn handle(&self) -> *u8 { **self } }

impl wxHtmlEasyPrinting {
    pub fn from(handle: *u8) -> @_wxHtmlEasyPrinting {
        @wxHtmlEasyPrinting(handle) as @_wxHtmlEasyPrinting
    }
    
}

trait _wxHtmlEasyPrinting : _wxObject {
}

struct wxHtmlFilter(*u8);
impl _wxHtmlFilter for wxHtmlFilter {}
impl _wxObject for wxHtmlFilter { fn handle(&self) -> *u8 { **self } }

impl wxHtmlFilter {
    pub fn from(handle: *u8) -> @_wxHtmlFilter {
        @wxHtmlFilter(handle) as @_wxHtmlFilter
    }
    
}

trait _wxHtmlFilter : _wxObject {
}

struct wxHtmlHelpController(*u8);
impl _wxHtmlHelpController for wxHtmlHelpController {}
impl _wxHelpControllerBase for wxHtmlHelpController {}
impl _wxObject for wxHtmlHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpController {
    pub fn from(handle: *u8) -> @_wxHtmlHelpController {
        @wxHtmlHelpController(handle) as @_wxHtmlHelpController
    }
    
    #[fixed_stack_segment]
    pub fn new(_style: c_int) -> @_wxHtmlHelpController {
        unsafe { @wxHtmlHelpController(wxHtmlHelpController_Create(_style)) as @_wxHtmlHelpController }
    }
}

trait _wxHtmlHelpController : _wxHelpControllerBase {
    #[fixed_stack_segment]
    fn addBook(&self, book: *u8, show_wait_msg: c_int) -> bool {
        unsafe { wxHtmlHelpController_AddBook(self.handle(), book, show_wait_msg) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHtmlHelpController_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn display(&self, x: *u8) -> c_int {
        unsafe { wxHtmlHelpController_Display(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn displayBlock(&self, blockNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplayBlock(self.handle(), blockNo) }
    }
    #[fixed_stack_segment]
    fn displayContents(&self) -> c_int {
        unsafe { wxHtmlHelpController_DisplayContents(self.handle()) }
    }
    #[fixed_stack_segment]
    fn displayIndex(&self) -> c_int {
        unsafe { wxHtmlHelpController_DisplayIndex(self.handle()) }
    }
    #[fixed_stack_segment]
    fn displayNumber(&self, id: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplayNumber(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn displaySection(&self, section: @_wxString) -> bool {
        unsafe { wxHtmlHelpController_DisplaySection(self.handle(), section.handle()) }
    }
    #[fixed_stack_segment]
    fn displaySectionNumber(&self, sectionNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.handle(), sectionNo) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @_wxFrame {
        unsafe { @wxFrame(wxHtmlHelpController_GetFrame(self.handle())) as @_wxFrame }
    }
    #[fixed_stack_segment]
    fn getFrameParameters(&self, title: *u8, width: *c_int, height: *c_int, pos_x: *c_int, pos_y: *c_int, newFrameEachTime: *c_int) -> *u8 {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, file: @_wxString) -> bool {
        unsafe { wxHtmlHelpController_Initialize(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn keywordSearch(&self, keyword: @_wxString) -> bool {
        unsafe { wxHtmlHelpController_KeywordSearch(self.handle(), keyword.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, file: @_wxString) -> bool {
        unsafe { wxHtmlHelpController_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn quit(&self) -> bool {
        unsafe { wxHtmlHelpController_Quit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @_wxConfigBase, path: @_wxString) {
        unsafe { wxHtmlHelpController_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setFrameParameters(&self, title: *u8, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: bool) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn setTempDir(&self, path: @_wxString) {
        unsafe { wxHtmlHelpController_SetTempDir(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitleFormat(&self, format: *u8) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.handle(), format) }
    }
    #[fixed_stack_segment]
    fn setViewer(&self, viewer: @_wxString, flags: c_int) {
        unsafe { wxHtmlHelpController_SetViewer(self.handle(), viewer.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn useConfig(&self, config: @_wxConfigBase, rootpath: @_wxString) {
        unsafe { wxHtmlHelpController_UseConfig(self.handle(), config.handle(), rootpath.handle()) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @_wxConfigBase, path: @_wxString) {
        unsafe { wxHtmlHelpController_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxHtmlHelpData(*u8);
impl _wxHtmlHelpData for wxHtmlHelpData {}
impl _wxObject for wxHtmlHelpData { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpData {
    pub fn from(handle: *u8) -> @_wxHtmlHelpData {
        @wxHtmlHelpData(handle) as @_wxHtmlHelpData
    }
    
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
    pub fn from(handle: *u8) -> @_wxHtmlHelpFrame {
        @wxHtmlHelpFrame(handle) as @_wxHtmlHelpFrame
    }
    
}

trait _wxHtmlHelpFrame : _wxFrame {
}

struct wxHtmlLinkInfo(*u8);
impl _wxHtmlLinkInfo for wxHtmlLinkInfo {}
impl _wxObject for wxHtmlLinkInfo { fn handle(&self) -> *u8 { **self } }

impl wxHtmlLinkInfo {
    pub fn from(handle: *u8) -> @_wxHtmlLinkInfo {
        @wxHtmlLinkInfo(handle) as @_wxHtmlLinkInfo
    }
    
}

trait _wxHtmlLinkInfo : _wxObject {
}

struct wxHtmlParser(*u8);
impl _wxHtmlParser for wxHtmlParser {}
impl _wxObject for wxHtmlParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlParser {
    pub fn from(handle: *u8) -> @_wxHtmlParser {
        @wxHtmlParser(handle) as @_wxHtmlParser
    }
    
}

trait _wxHtmlParser : _wxObject {
}

struct wxHtmlPrintout(*u8);
impl _wxHtmlPrintout for wxHtmlPrintout {}
impl _wxPrintout for wxHtmlPrintout {}
impl _wxObject for wxHtmlPrintout { fn handle(&self) -> *u8 { **self } }

impl wxHtmlPrintout {
    pub fn from(handle: *u8) -> @_wxHtmlPrintout {
        @wxHtmlPrintout(handle) as @_wxHtmlPrintout
    }
    
}

trait _wxHtmlPrintout : _wxPrintout {
}

struct wxHtmlTag(*u8);
impl _wxHtmlTag for wxHtmlTag {}
impl _wxObject for wxHtmlTag { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTag {
    pub fn from(handle: *u8) -> @_wxHtmlTag {
        @wxHtmlTag(handle) as @_wxHtmlTag
    }
    
}

trait _wxHtmlTag : _wxObject {
}

struct wxHtmlTagHandler(*u8);
impl _wxHtmlTagHandler for wxHtmlTagHandler {}
impl _wxObject for wxHtmlTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagHandler {
    pub fn from(handle: *u8) -> @_wxHtmlTagHandler {
        @wxHtmlTagHandler(handle) as @_wxHtmlTagHandler
    }
    
}

trait _wxHtmlTagHandler : _wxObject {
}

struct wxHtmlTagsModule(*u8);
impl _wxHtmlTagsModule for wxHtmlTagsModule {}
impl _wxModule for wxHtmlTagsModule {}
impl _wxObject for wxHtmlTagsModule { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagsModule {
    pub fn from(handle: *u8) -> @_wxHtmlTagsModule {
        @wxHtmlTagsModule(handle) as @_wxHtmlTagsModule
    }
    
}

trait _wxHtmlTagsModule : _wxModule {
}

struct wxHtmlWidgetCell(*u8);
impl _wxHtmlWidgetCell for wxHtmlWidgetCell {}
impl _wxHtmlCell for wxHtmlWidgetCell {}
impl _wxObject for wxHtmlWidgetCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWidgetCell {
    pub fn from(handle: *u8) -> @_wxHtmlWidgetCell {
        @wxHtmlWidgetCell(handle) as @_wxHtmlWidgetCell
    }
    
}

trait _wxHtmlWidgetCell : _wxHtmlCell {
}

struct wxHtmlWinParser(*u8);
impl _wxHtmlWinParser for wxHtmlWinParser {}
impl _wxHtmlParser for wxHtmlWinParser {}
impl _wxObject for wxHtmlWinParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinParser {
    pub fn from(handle: *u8) -> @_wxHtmlWinParser {
        @wxHtmlWinParser(handle) as @_wxHtmlWinParser
    }
    
}

trait _wxHtmlWinParser : _wxHtmlParser {
}

struct wxHtmlWinTagHandler(*u8);
impl _wxHtmlWinTagHandler for wxHtmlWinTagHandler {}
impl _wxHtmlTagHandler for wxHtmlWinTagHandler {}
impl _wxObject for wxHtmlWinTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinTagHandler {
    pub fn from(handle: *u8) -> @_wxHtmlWinTagHandler {
        @wxHtmlWinTagHandler(handle) as @_wxHtmlWinTagHandler
    }
    
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
    pub fn from(handle: *u8) -> @_wxHtmlWindow {
        @wxHtmlWindow(handle) as @_wxHtmlWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: @_wxString) -> @_wxHtmlWindow {
        unsafe { @wxHtmlWindow(wxHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) as @_wxHtmlWindow }
    }
}

trait _wxHtmlWindow : _wxScrolledWindow {
    #[fixed_stack_segment]
    fn appendToPage(&self, source: @_wxString) -> bool {
        unsafe { wxHtmlWindow_AppendToPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn getInternalRepresentation(&self) -> @_wxHtmlContainerCell {
        unsafe { @wxHtmlContainerCell(wxHtmlWindow_GetInternalRepresentation(self.handle())) as @_wxHtmlContainerCell }
    }
    #[fixed_stack_segment]
    fn getOpenedAnchor(&self) -> @_wxString {
        unsafe { @wxString(wxHtmlWindow_GetOpenedAnchor(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getOpenedPage(&self) -> @_wxString {
        unsafe { @wxString(wxHtmlWindow_GetOpenedPage(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getOpenedPageTitle(&self) -> @_wxString {
        unsafe { @wxString(wxHtmlWindow_GetOpenedPageTitle(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getRelatedFrame(&self) -> @_wxFrame {
        unsafe { @wxFrame(wxHtmlWindow_GetRelatedFrame(self.handle())) as @_wxFrame }
    }
    #[fixed_stack_segment]
    fn historyBack(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryBack(self.handle()) }
    }
    #[fixed_stack_segment]
    fn historyCanBack(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryCanBack(self.handle()) }
    }
    #[fixed_stack_segment]
    fn historyCanForward(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryCanForward(self.handle()) }
    }
    #[fixed_stack_segment]
    fn historyClear(&self) {
        unsafe { wxHtmlWindow_HistoryClear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn historyForward(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryForward(self.handle()) }
    }
    #[fixed_stack_segment]
    fn loadPage(&self, location: @_wxString) -> bool {
        unsafe { wxHtmlWindow_LoadPage(self.handle(), location.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @_wxConfigBase, path: @_wxString) {
        unsafe { wxHtmlWindow_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.handle(), b) }
    }
    #[fixed_stack_segment]
    fn setFonts(&self, normal_face: @_wxString, fixed_face: @_wxString, sizes: *c_int) {
        unsafe { wxHtmlWindow_SetFonts(self.handle(), normal_face.handle(), fixed_face.handle(), sizes) }
    }
    #[fixed_stack_segment]
    fn setPage(&self, source: @_wxString) {
        unsafe { wxHtmlWindow_SetPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedFrame(&self, frame: @_wxFrame, format: @_wxString) {
        unsafe { wxHtmlWindow_SetRelatedFrame(self.handle(), frame.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.handle(), bar) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @_wxConfigBase, path: @_wxString) {
        unsafe { wxHtmlWindow_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxIPV4address(*u8);
impl _wxIPV4address for wxIPV4address {}
impl _wxSockAddress for wxIPV4address {}
impl _wxObject for wxIPV4address { fn handle(&self) -> *u8 { **self } }

impl wxIPV4address {
    pub fn from(handle: *u8) -> @_wxIPV4address {
        @wxIPV4address(handle) as @_wxIPV4address
    }
    
}

trait _wxIPV4address : _wxSockAddress {
}

struct wxIcon(*u8);
impl _wxIcon for wxIcon {}
impl _wxBitmap for wxIcon {}
impl _wxGDIObject for wxIcon {}
impl _wxObject for wxIcon { fn handle(&self) -> *u8 { **self } }

impl wxIcon {
    pub fn from(handle: *u8) -> @_wxIcon {
        @wxIcon(handle) as @_wxIcon
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxIcon {
        unsafe { @wxIcon(wxIcon_CreateDefault()) as @_wxIcon }
    }
    #[fixed_stack_segment]
    pub fn newLoad(name: @_wxString, type_: c_long, width: c_int, height: c_int) -> @_wxIcon {
        unsafe { @wxIcon(wxIcon_CreateLoad(name.handle(), type_, width, height)) as @_wxIcon }
    }
}

trait _wxIcon : _wxBitmap {
    #[fixed_stack_segment]
    fn assign(&self, other: *u8) {
        unsafe { wxIcon_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn copyFromBitmap(&self, bmp: @_wxBitmap) {
        unsafe { wxIcon_CopyFromBitmap(self.handle(), bmp.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIcon_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn fromRaw(&self, width: c_int, height: c_int) -> @_wxIcon {
        unsafe { @wxIcon(wxIcon_FromRaw(self.handle(), width, height)) as @_wxIcon }
    }
    #[fixed_stack_segment]
    fn fromXPM(&self) -> @_wxIcon {
        unsafe { @wxIcon(wxIcon_FromXPM(self.handle())) as @_wxIcon }
    }
    #[fixed_stack_segment]
    fn getDepth(&self) -> c_int {
        unsafe { wxIcon_GetDepth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> c_int {
        unsafe { wxIcon_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxIcon_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, other: @_wxIcon) -> bool {
        unsafe { wxIcon_IsEqual(self.handle(), other.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, name: @_wxString, type_: c_long, width: c_int, height: c_int) -> c_int {
        unsafe { wxIcon_Load(self.handle(), name.handle(), type_, width, height) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxIcon_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDepth(&self, depth: c_int) {
        unsafe { wxIcon_SetDepth(self.handle(), depth) }
    }
    #[fixed_stack_segment]
    fn setHeight(&self, height: c_int) {
        unsafe { wxIcon_SetHeight(self.handle(), height) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: c_int) {
        unsafe { wxIcon_SetWidth(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxIcon_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxIcon_IsStatic(self.handle()) }
    }
}

struct wxIconBundle(*u8);
impl _wxIconBundle for wxIconBundle { fn handle(&self) -> *u8 { **self } }

impl wxIconBundle {
    pub fn from(handle: *u8) -> @_wxIconBundle {
        @wxIconBundle(handle) as @_wxIconBundle
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxIconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateDefault()) as @_wxIconBundle }
    }
    #[fixed_stack_segment]
    pub fn newFromFile(file: @_wxString, type_: c_int) -> @_wxIconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateFromFile(file.handle(), type_)) as @_wxIconBundle }
    }
    #[fixed_stack_segment]
    pub fn newFromIcon(icon: @_wxIcon) -> @_wxIconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateFromIcon(icon.handle())) as @_wxIconBundle }
    }
}

trait _wxIconBundle {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @_wxIcon) {
        unsafe { wxIconBundle_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addIconFromFile(&self, file: @_wxString, type_: c_int) {
        unsafe { wxIconBundle_AddIconFromFile(self.handle(), file.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn assign(&self, _ref: @_wxIconBundle) {
        unsafe { wxIconBundle_Assign(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, w: c_int, h: c_int, _ref: @_wxIcon) {
        unsafe { wxIconBundle_GetIcon(self.handle(), w, h, _ref.handle()) }
    }
}

struct wxIconizeEvent(*u8);
impl _wxIconizeEvent for wxIconizeEvent {}
impl _wxEvent for wxIconizeEvent {}
impl _wxObject for wxIconizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxIconizeEvent {
    pub fn from(handle: *u8) -> @_wxIconizeEvent {
        @wxIconizeEvent(handle) as @_wxIconizeEvent
    }
    
}

trait _wxIconizeEvent : _wxEvent {
}

struct wxIdleEvent(*u8);
impl _wxIdleEvent for wxIdleEvent {}
impl _wxEvent for wxIdleEvent {}
impl _wxObject for wxIdleEvent { fn handle(&self) -> *u8 { **self } }

impl wxIdleEvent {
    pub fn from(handle: *u8) -> @_wxIdleEvent {
        @wxIdleEvent(handle) as @_wxIdleEvent
    }
    
}

trait _wxIdleEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: @_wxObject) {
        unsafe { wxIdleEvent_CopyObject(self.handle(), object_dest.handle()) }
    }
    #[fixed_stack_segment]
    fn moreRequested(&self) -> bool {
        unsafe { wxIdleEvent_MoreRequested(self.handle()) }
    }
    #[fixed_stack_segment]
    fn requestMore(&self, needMore: bool) {
        unsafe { wxIdleEvent_RequestMore(self.handle(), needMore) }
    }
}

struct wxImage(*u8);
impl _wxImage for wxImage {}
impl _wxObject for wxImage { fn handle(&self) -> *u8 { **self } }

impl wxImage {
    pub fn from(handle: *u8) -> @_wxImage {
        @wxImage(handle) as @_wxImage
    }
    
    #[fixed_stack_segment]
    pub fn canRead(name: @_wxString) -> bool {
        unsafe { wxImage_CanRead(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateDefault()) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(bitmap: @_wxBitmap) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateFromBitmap(bitmap.handle())) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newFromByteString(data: *char, length: c_int, type_: c_int) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateFromByteString(data, length, type_)) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newFromLazyByteString(data: *char, length: c_int, type_: c_int) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateFromLazyByteString(data, length, type_)) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newFromData(width: c_int, height: c_int, data: *u8) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateFromData(width, height, data)) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newFromFile(name: @_wxString) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateFromFile(name.handle())) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newSized(width: c_int, height: c_int) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateSized(width, height)) as @_wxImage }
    }
    #[fixed_stack_segment]
    pub fn newFromDataEx(width: c_int, height: c_int, data: *u8, isStaticData: c_int) -> @_wxImage {
        unsafe { @wxImage(wxImage_CreateFromDataEx(width, height, data, isStaticData)) as @_wxImage }
    }
}

trait _wxImage : _wxObject {
    #[fixed_stack_segment]
    fn convertToBitmap(&self, bitmap: @_wxBitmap) {
        unsafe { wxImage_ConvertToBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn convertToByteString(&self, type_: c_int, data: *char) -> c_int {
        unsafe { wxImage_ConvertToByteString(self.handle(), type_, data) }
    }
    #[fixed_stack_segment]
    fn convertToLazyByteString(&self, type_: c_int, data: *char) -> c_int {
        unsafe { wxImage_ConvertToLazyByteString(self.handle(), type_, data) }
    }
    #[fixed_stack_segment]
    fn countColours(&self, stopafter: c_int) -> c_int {
        unsafe { wxImage_CountColours(self.handle(), stopafter) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxImage_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBlue(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetBlue(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> *u8 {
        unsafe { wxImage_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getGreen(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetGreen(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> c_int {
        unsafe { wxImage_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaskBlue(&self) -> wchar_t {
        unsafe { wxImage_GetMaskBlue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaskGreen(&self) -> wchar_t {
        unsafe { wxImage_GetMaskGreen(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaskRed(&self) -> wchar_t {
        unsafe { wxImage_GetMaskRed(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRed(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetRed(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getSubImage(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: @_wxImage) {
        unsafe { wxImage_GetSubImage(self.handle(), x, y, w, h, image.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxImage_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasMask(&self) -> bool {
        unsafe { wxImage_HasMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOption(&self, name: @_wxString) -> @_wxString {
        unsafe { @wxString(wxImage_GetOption(self.handle(), name.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getOptionInt(&self, name: @_wxString) -> bool {
        unsafe { wxImage_GetOptionInt(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn hasOption(&self, name: @_wxString) -> bool {
        unsafe { wxImage_HasOption(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Initialize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn initializeFromData(&self, width: c_int, height: c_int, data: *u8) {
        unsafe { wxImage_InitializeFromData(self.handle(), width, height, data) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, name: @_wxString, type_: c_int) -> bool {
        unsafe { wxImage_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn mirror(&self, horizontally: c_int, image: @_wxImage) {
        unsafe { wxImage_Mirror(self.handle(), horizontally, image.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxImage_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paste(&self, image: @_wxImage, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self.handle(), image.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn replace(&self, r1: uint8_t, g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t, b2: uint8_t) {
        unsafe { wxImage_Replace(self.handle(), r1, g1, b1, r2, g2, b2) }
    }
    #[fixed_stack_segment]
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn rotate(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *u8, image: @_wxImage) {
        unsafe { wxImage_Rotate(self.handle(), angle, c_x, c_y, interpolating, offset_after_rotation, image.handle()) }
    }
    #[fixed_stack_segment]
    fn rotate90(&self, clockwise: c_int, image: @_wxImage) {
        unsafe { wxImage_Rotate90(self.handle(), clockwise, image.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @_wxString, type_: c_int) -> bool {
        unsafe { wxImage_SaveFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn scale(&self, width: c_int, height: c_int, image: @_wxImage) {
        unsafe { wxImage_Scale(self.handle(), width, height, image.handle()) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: *u8) {
        unsafe { wxImage_SetData(self.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setDataAndSize(&self, data: *u8, new_width: c_int, new_height: c_int) {
        unsafe { wxImage_SetDataAndSize(self.handle(), data, new_width, new_height) }
    }
    #[fixed_stack_segment]
    fn setMask(&self, mask: c_int) {
        unsafe { wxImage_SetMask(self.handle(), mask) }
    }
    #[fixed_stack_segment]
    fn setMaskColour(&self, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetMaskColour(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    fn setOption(&self, name: @_wxString, value: @_wxString) {
        unsafe { wxImage_SetOption(self.handle(), name.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setOptionInt(&self, name: @_wxString, value: c_int) {
        unsafe { wxImage_SetOptionInt(self.handle(), name.handle(), value) }
    }
    #[fixed_stack_segment]
    fn setRGB(&self, x: c_int, y: c_int, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetRGB(self.handle(), x, y, r, g, b) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImage_Delete(self.handle()) }
    }
}

struct wxImageHandler(*u8);
impl _wxImageHandler for wxImageHandler {}
impl _wxObject for wxImageHandler { fn handle(&self) -> *u8 { **self } }

impl wxImageHandler {
    pub fn from(handle: *u8) -> @_wxImageHandler {
        @wxImageHandler(handle) as @_wxImageHandler
    }
    
}

trait _wxImageHandler : _wxObject {
}

struct wxImageList(*u8);
impl _wxImageList for wxImageList {}
impl _wxObject for wxImageList { fn handle(&self) -> *u8 { **self } }

impl wxImageList {
    pub fn from(handle: *u8) -> @_wxImageList {
        @wxImageList(handle) as @_wxImageList
    }
    
    #[fixed_stack_segment]
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> @_wxImageList {
        unsafe { @wxImageList(wxImageList_Create(width, height, mask, initialCount)) as @_wxImageList }
    }
}

trait _wxImageList : _wxObject {
    #[fixed_stack_segment]
    fn addBitmap(&self, bitmap: @_wxBitmap, mask: @_wxBitmap) -> c_int {
        unsafe { wxImageList_AddBitmap(self.handle(), bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @_wxIcon) -> c_int {
        unsafe { wxImageList_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addMasked(&self, bitmap: @_wxBitmap, maskColour: @_wxColour) -> c_int {
        unsafe { wxImageList_AddMasked(self.handle(), bitmap.handle(), maskColour.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImageList_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn draw(&self, index: c_int, dc: @_wxDC, x: c_int, y: c_int, flags: c_int, solidBackground: bool) -> bool {
        unsafe { wxImageList_Draw(self.handle(), index, dc.handle(), x, y, flags, solidBackground) }
    }
    #[fixed_stack_segment]
    fn getImageCount(&self) -> c_int {
        unsafe { wxImageList_GetImageCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self, index: c_int, width: *c_int, height: *c_int) {
        unsafe { wxImageList_GetSize(self.handle(), index, width, height) }
    }
    #[fixed_stack_segment]
    fn remove(&self, index: c_int) -> bool {
        unsafe { wxImageList_Remove(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn removeAll(&self) -> bool {
        unsafe { wxImageList_RemoveAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replace(&self, index: c_int, bitmap: @_wxBitmap, mask: @_wxBitmap) -> bool {
        unsafe { wxImageList_Replace(self.handle(), index, bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceIcon(&self, index: c_int, icon: @_wxIcon) -> bool {
        unsafe { wxImageList_ReplaceIcon(self.handle(), index, icon.handle()) }
    }
}

struct wxIndividualLayoutConstraint(*u8);
impl _wxIndividualLayoutConstraint for wxIndividualLayoutConstraint {}
impl _wxObject for wxIndividualLayoutConstraint { fn handle(&self) -> *u8 { **self } }

impl wxIndividualLayoutConstraint {
    pub fn from(handle: *u8) -> @_wxIndividualLayoutConstraint {
        @wxIndividualLayoutConstraint(handle) as @_wxIndividualLayoutConstraint
    }
    
}

trait _wxIndividualLayoutConstraint : _wxObject {
    #[fixed_stack_segment]
    fn above(&self, sibling: @_wxWindow, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Above(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn absolute(&self, val: c_int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self.handle()) }
    }
    #[fixed_stack_segment]
    fn below(&self, sibling: @_wxWindow, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Below(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn getDone(&self) -> bool {
        unsafe { wxIndividualLayoutConstraint_GetDone(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEdge(&self, which: c_int, thisWin: *u8, other: *u8) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetEdge(self.handle(), which, thisWin, other) }
    }
    #[fixed_stack_segment]
    fn getMargin(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMargin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMyEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetMyEdge(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOtherEdge(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetOtherEdge(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOtherWindow(&self) -> *u8 {
        unsafe { wxIndividualLayoutConstraint_GetOtherWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPercent(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetPercent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRelationship(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetRelationship(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> c_int {
        unsafe { wxIndividualLayoutConstraint_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leftOf(&self, sibling: @_wxWindow, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn percentOf(&self, otherW: @_wxWindow, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.handle(), otherW.handle(), wh, per) }
    }
    #[fixed_stack_segment]
    fn resetIfWin(&self, otherW: @_wxWindow) -> bool {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.handle(), otherW.handle()) }
    }
    #[fixed_stack_segment]
    fn rightOf(&self, sibling: @_wxWindow, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn sameAs(&self, otherW: @_wxWindow, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.handle(), otherW.handle(), edge, marg) }
    }
    #[fixed_stack_segment]
    fn satisfyConstraint(&self, constraints: *u8, win: @_wxWindow) -> bool {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.handle(), constraints, win.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, rel: c_int, otherW: @_wxWindow, otherE: c_int, val: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Set(self.handle(), rel, otherW.handle(), otherE, val, marg) }
    }
    #[fixed_stack_segment]
    fn setDone(&self, d: bool) {
        unsafe { wxIndividualLayoutConstraint_SetDone(self.handle(), d) }
    }
    #[fixed_stack_segment]
    fn setEdge(&self, which: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetEdge(self.handle(), which) }
    }
    #[fixed_stack_segment]
    fn setMargin(&self, m: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetMargin(self.handle(), m) }
    }
    #[fixed_stack_segment]
    fn setRelationship(&self, r: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetRelationship(self.handle(), r) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, v: c_int) {
        unsafe { wxIndividualLayoutConstraint_SetValue(self.handle(), v) }
    }
    #[fixed_stack_segment]
    fn unconstrained(&self) {
        unsafe { wxIndividualLayoutConstraint_Unconstrained(self.handle()) }
    }
}

struct wxInitDialogEvent(*u8);
impl _wxInitDialogEvent for wxInitDialogEvent {}
impl _wxEvent for wxInitDialogEvent {}
impl _wxObject for wxInitDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxInitDialogEvent {
    pub fn from(handle: *u8) -> @_wxInitDialogEvent {
        @wxInitDialogEvent(handle) as @_wxInitDialogEvent
    }
    
}

trait _wxInitDialogEvent : _wxEvent {
}

struct wxInputStream(*u8);
impl _wxInputStream for wxInputStream {}
impl _wxStreamBase for wxInputStream { fn handle(&self) -> *u8 { **self } }

impl wxInputStream {
    pub fn from(handle: *u8) -> @_wxInputStream {
        @wxInputStream(handle) as @_wxInputStream
    }
    
}

trait _wxInputStream : _wxStreamBase {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn eof(&self) -> bool {
        unsafe { wxInputStream_Eof(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getC(&self) -> wchar_t {
        unsafe { wxInputStream_GetC(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lastRead(&self) -> c_int {
        unsafe { wxInputStream_LastRead(self.handle()) }
    }
    #[fixed_stack_segment]
    fn peek(&self) -> wchar_t {
        unsafe { wxInputStream_Peek(self.handle()) }
    }
    #[fixed_stack_segment]
    fn read(&self, buffer: *u8, size: c_int) {
        unsafe { wxInputStream_Read(self.handle(), buffer, size) }
    }
    #[fixed_stack_segment]
    fn seekI(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxInputStream_SeekI(self.handle(), pos, mode) }
    }
    #[fixed_stack_segment]
    fn tell(&self) -> c_int {
        unsafe { wxInputStream_Tell(self.handle()) }
    }
    #[fixed_stack_segment]
    fn ungetBuffer(&self, buffer: *u8, size: c_int) -> c_int {
        unsafe { wxInputStream_UngetBuffer(self.handle(), buffer, size) }
    }
    #[fixed_stack_segment]
    fn ungetch(&self, c: wchar_t) -> c_int {
        unsafe { wxInputStream_Ungetch(self.handle(), c) }
    }
    #[fixed_stack_segment]
    fn canRead(&self) -> bool {
        unsafe { wxInputStream_CanRead(self.handle()) }
    }
}

struct wxJoystick(*u8);
impl _wxJoystick for wxJoystick {}
impl _wxObject for wxJoystick { fn handle(&self) -> *u8 { **self } }

impl wxJoystick {
    pub fn from(handle: *u8) -> @_wxJoystick {
        @wxJoystick(handle) as @_wxJoystick
    }
    
}

trait _wxJoystick : _wxObject {
}

struct wxJoystickEvent(*u8);
impl _wxJoystickEvent for wxJoystickEvent {}
impl _wxEvent for wxJoystickEvent {}
impl _wxObject for wxJoystickEvent { fn handle(&self) -> *u8 { **self } }

impl wxJoystickEvent {
    pub fn from(handle: *u8) -> @_wxJoystickEvent {
        @wxJoystickEvent(handle) as @_wxJoystickEvent
    }
    
}

trait _wxJoystickEvent : _wxEvent {
    #[fixed_stack_segment]
    fn buttonDown(&self, but: c_int) -> bool {
        unsafe { wxJoystickEvent_ButtonDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn buttonIsDown(&self, but: c_int) -> bool {
        unsafe { wxJoystickEvent_ButtonIsDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn buttonUp(&self, but: c_int) -> bool {
        unsafe { wxJoystickEvent_ButtonUp(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxJoystickEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getButtonChange(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonChange(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getButtonState(&self) -> c_int {
        unsafe { wxJoystickEvent_GetButtonState(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getJoystick(&self) -> c_int {
        unsafe { wxJoystickEvent_GetJoystick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxJoystickEvent_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getZPosition(&self) -> c_int {
        unsafe { wxJoystickEvent_GetZPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isButton(&self) -> bool {
        unsafe { wxJoystickEvent_IsButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isMove(&self) -> bool {
        unsafe { wxJoystickEvent_IsMove(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isZMove(&self) -> bool {
        unsafe { wxJoystickEvent_IsZMove(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setButtonChange(&self, change: c_int) {
        unsafe { wxJoystickEvent_SetButtonChange(self.handle(), change) }
    }
    #[fixed_stack_segment]
    fn setButtonState(&self, state: c_int) {
        unsafe { wxJoystickEvent_SetButtonState(self.handle(), state) }
    }
    #[fixed_stack_segment]
    fn setJoystick(&self, stick: c_int) {
        unsafe { wxJoystickEvent_SetJoystick(self.handle(), stick) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxJoystickEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setZPosition(&self, zPos: c_int) {
        unsafe { wxJoystickEvent_SetZPosition(self.handle(), zPos) }
    }
}

struct wxKeyEvent(*u8);
impl _wxKeyEvent for wxKeyEvent {}
impl _wxEvent for wxKeyEvent {}
impl _wxObject for wxKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxKeyEvent {
    pub fn from(handle: *u8) -> @_wxKeyEvent {
        @wxKeyEvent(handle) as @_wxKeyEvent
    }
    
}

trait _wxKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxKeyEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxKeyEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxKeyEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getKeyCode(&self) -> c_int {
        unsafe { wxKeyEvent_GetKeyCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxKeyEvent_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> c_int {
        unsafe { wxKeyEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> c_int {
        unsafe { wxKeyEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getModifiers(&self) -> c_int {
        unsafe { wxKeyEvent_GetModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasModifiers(&self) -> bool {
        unsafe { wxKeyEvent_HasModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxKeyEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setKeyCode(&self, code: c_int) {
        unsafe { wxKeyEvent_SetKeyCode(self.handle(), code) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxLEDNumberCtrl {
        @wxLEDNumberCtrl(handle) as @_wxLEDNumberCtrl
    }
    
}

trait _wxLEDNumberCtrl : _wxControl {
}

struct wxLayoutAlgorithm(*u8);
impl _wxLayoutAlgorithm for wxLayoutAlgorithm {}
impl _wxObject for wxLayoutAlgorithm { fn handle(&self) -> *u8 { **self } }

impl wxLayoutAlgorithm {
    pub fn from(handle: *u8) -> @_wxLayoutAlgorithm {
        @wxLayoutAlgorithm(handle) as @_wxLayoutAlgorithm
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxLayoutAlgorithm {
        unsafe { @wxLayoutAlgorithm(wxLayoutAlgorithm_Create()) as @_wxLayoutAlgorithm }
    }
}

trait _wxLayoutAlgorithm : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLayoutAlgorithm_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layoutFrame(&self, frame: @_wxFrame, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.handle(), frame.handle(), mainWindow) }
    }
    #[fixed_stack_segment]
    fn layoutMDIFrame(&self, frame: @_wxFrame, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.handle(), frame.handle(), x, y, w, h, use_) }
    }
    #[fixed_stack_segment]
    fn layoutWindow(&self, frame: @_wxFrame, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.handle(), frame.handle(), mainWindow) }
    }
}

struct wxLayoutConstraints(*u8);
impl _wxLayoutConstraints for wxLayoutConstraints {}
impl _wxObject for wxLayoutConstraints { fn handle(&self) -> *u8 { **self } }

impl wxLayoutConstraints {
    pub fn from(handle: *u8) -> @_wxLayoutConstraints {
        @wxLayoutConstraints(handle) as @_wxLayoutConstraints
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxLayoutConstraints {
        unsafe { @wxLayoutConstraints(wxLayoutConstraints_Create()) as @_wxLayoutConstraints }
    }
}

trait _wxLayoutConstraints : _wxObject {
    #[fixed_stack_segment]
    fn bottom(&self) -> *u8 {
        unsafe { wxLayoutConstraints_bottom(self.handle()) }
    }
    #[fixed_stack_segment]
    fn centreX(&self) -> *u8 {
        unsafe { wxLayoutConstraints_centreX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn centreY(&self) -> *u8 {
        unsafe { wxLayoutConstraints_centreY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn height(&self) -> *u8 {
        unsafe { wxLayoutConstraints_height(self.handle()) }
    }
    #[fixed_stack_segment]
    fn left(&self) -> *u8 {
        unsafe { wxLayoutConstraints_left(self.handle()) }
    }
    #[fixed_stack_segment]
    fn right(&self) -> *u8 {
        unsafe { wxLayoutConstraints_right(self.handle()) }
    }
    #[fixed_stack_segment]
    fn top(&self) -> *u8 {
        unsafe { wxLayoutConstraints_top(self.handle()) }
    }
    #[fixed_stack_segment]
    fn width(&self) -> *u8 {
        unsafe { wxLayoutConstraints_width(self.handle()) }
    }
}

struct wxList(*u8);
impl _wxList for wxList {}
impl _wxObject for wxList { fn handle(&self) -> *u8 { **self } }

impl wxList {
    pub fn from(handle: *u8) -> @_wxList {
        @wxList(handle) as @_wxList
    }
    
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
    pub fn from(handle: *u8) -> @_wxListBox {
        @wxListBox(handle) as @_wxListBox
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @_wxListBox {
        unsafe { @wxListBox(wxListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @_wxListBox }
    }
}

trait _wxListBox : _wxControl {
    #[fixed_stack_segment]
    fn append(&self, item: @_wxString) {
        unsafe { wxListBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @_wxString, data: *u8) {
        unsafe { wxListBox_AppendData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxListBox_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: c_int) {
        unsafe { wxListBox_Delete(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @_wxString) -> c_int {
        unsafe { wxListBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> @_wxClientData {
        unsafe { @wxClientData(wxListBox_GetClientData(self.handle(), n)) as @_wxClientData }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxListBox_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> c_int {
        unsafe { wxListBox_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelections(&self, aSelections: *c_int, allocated: c_int) -> c_int {
        unsafe { wxListBox_GetSelections(self.handle(), aSelections, allocated) }
    }
    #[fixed_stack_segment]
    fn getString(&self, n: c_int) -> @_wxString {
        unsafe { @wxString(wxListBox_GetString(self.handle(), n)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn insertItems(&self, items: *u8, pos: c_int, count: c_int) {
        unsafe { wxListBox_InsertItems(self.handle(), items, pos, count) }
    }
    #[fixed_stack_segment]
    fn isSelected(&self, n: c_int) -> bool {
        unsafe { wxListBox_IsSelected(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, n: c_int, clientData: @_wxClientData) {
        unsafe { wxListBox_SetClientData(self.handle(), n, clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setFirstItem(&self, n: c_int) {
        unsafe { wxListBox_SetFirstItem(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: c_int, select: c_int) {
        unsafe { wxListBox_SetSelection(self.handle(), n, select) }
    }
    #[fixed_stack_segment]
    fn setString(&self, n: c_int, s: @_wxString) {
        unsafe { wxListBox_SetString(self.handle(), n, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setStringSelection(&self, str: @_wxString, sel: bool) {
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
    pub fn from(handle: *u8) -> @_wxListCtrl {
        @wxListCtrl(handle) as @_wxListCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxListCtrl {
        unsafe { @wxListCtrl(wxListCtrl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxListCtrl }
    }
}

trait _wxListCtrl : _wxControl {
    #[fixed_stack_segment]
    fn arrange(&self, flag: c_int) -> bool {
        unsafe { wxListCtrl_Arrange(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAllColumns(&self) -> bool {
        unsafe { wxListCtrl_DeleteAllColumns(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) -> bool {
        unsafe { wxListCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteColumn(&self, col: c_int) -> bool {
        unsafe { wxListCtrl_DeleteColumn(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn deleteItem(&self, item: c_int) -> bool {
        unsafe { wxListCtrl_DeleteItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn editLabel(&self, item: c_int) {
        unsafe { wxListCtrl_EditLabel(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn endEditLabel(&self, cancel: c_int) -> bool {
        unsafe { wxListCtrl_EndEditLabel(self.handle(), cancel) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, item: c_int) -> bool {
        unsafe { wxListCtrl_EnsureVisible(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, start: c_int, str: @_wxString, partial: bool) -> c_int {
        unsafe { wxListCtrl_FindItem(self.handle(), start, str.handle(), partial) }
    }
    #[fixed_stack_segment]
    fn findItemByData(&self, start: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByData(self.handle(), start, data) }
    }
    #[fixed_stack_segment]
    fn findItemByPosition(&self, start: c_int, x: c_int, y: c_int, direction: c_int) -> c_int {
        unsafe { wxListCtrl_FindItemByPosition(self.handle(), start, x, y, direction) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self, col: c_int, item: @_wxListItem) -> bool {
        unsafe { wxListCtrl_GetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn getColumnCount(&self) -> c_int {
        unsafe { wxListCtrl_GetColumnCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColumnWidth(&self, col: c_int) -> c_int {
        unsafe { wxListCtrl_GetColumnWidth(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn getCountPerPage(&self) -> c_int {
        unsafe { wxListCtrl_GetCountPerPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> @_wxTextCtrl {
        unsafe { @wxTextCtrl(wxListCtrl_GetEditControl(self.handle())) as @_wxTextCtrl }
    }
    #[fixed_stack_segment]
    fn getImageList(&self, which: c_int) -> @_wxImageList {
        unsafe { @wxImageList(wxListCtrl_GetImageList(self.handle(), which)) as @_wxImageList }
    }
    #[fixed_stack_segment]
    fn getItem(&self, info: @_wxListItem) -> bool {
        unsafe { wxListCtrl_GetItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemData(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemData(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn getItemFont(&self, item: c_long) -> @_wxFont {
        unsafe { @wxFont(wxListCtrl_GetItemFont(self.handle(), item)) as @_wxFont }
    }
    #[fixed_stack_segment]
    fn getItemPosition(&self, item: c_int) -> @_wxPoint {
        unsafe { @wxPoint(wxListCtrl_GetItemPosition(self.handle(), item)) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getItemRect(&self, item: c_int, code: c_int) -> @_wxRect {
        unsafe { @wxRect(wxListCtrl_GetItemRect(self.handle(), item, code)) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn getItemSpacing(&self, isSmall: bool) -> @_wxSize {
        unsafe { @wxSize(wxListCtrl_GetItemSpacing(self.handle(), isSmall)) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.handle(), item, stateMask) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: c_int) -> @_wxString {
        unsafe { @wxString(wxListCtrl_GetItemText(self.handle(), item)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNextItem(&self, item: c_int, geometry: c_int, state: c_int) -> c_int {
        unsafe { wxListCtrl_GetNextItem(self.handle(), item, geometry, state) }
    }
    #[fixed_stack_segment]
    fn getSelectedItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetSelectedItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @_wxColour) {
        unsafe { wxListCtrl_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTopItem(&self) -> c_int {
        unsafe { wxListCtrl_GetTopItem(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int, flags: *u8) -> c_int {
        unsafe { wxListCtrl_HitTest(self.handle(), x, y, flags) }
    }
    #[fixed_stack_segment]
    fn insertColumn(&self, col: c_int, heading: @_wxString, format: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_InsertColumn(self.handle(), col, heading.handle(), format, width) }
    }
    #[fixed_stack_segment]
    fn insertColumnFromInfo(&self, col: c_int, info: @_wxListItem) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.handle(), col, info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, info: @_wxListItem) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithData(&self, index: c_int, label: @_wxString) -> c_int {
        unsafe { wxListCtrl_InsertItemWithData(self.handle(), index, label.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.handle(), index, imageIndex) }
    }
    #[fixed_stack_segment]
    fn insertItemWithLabel(&self, index: c_int, label: @_wxString, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithLabel(self.handle(), index, label.handle(), imageIndex) }
    }
    #[fixed_stack_segment]
    fn isVirtual(&self) -> bool {
        unsafe { wxListCtrl_IsVirtual(self.handle()) }
    }
    #[fixed_stack_segment]
    fn refreshItem(&self, item: c_long) {
        unsafe { wxListCtrl_RefreshItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn scrollList(&self, dx: c_int, dy: c_int) -> bool {
        unsafe { wxListCtrl_ScrollList(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, col: @_wxColour) {
        unsafe { wxListCtrl_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumn(&self, col: c_int, item: @_wxListItem) -> bool {
        unsafe { wxListCtrl_SetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> bool {
        unsafe { wxListCtrl_SetColumnWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, col: @_wxColour) -> c_int {
        unsafe { wxListCtrl_SetForegroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @_wxImageList, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.handle(), imageList.handle(), which) }
    }
    #[fixed_stack_segment]
    fn setItem(&self, index: c_int, col: c_int, label: @_wxString, imageId: c_int) -> bool {
        unsafe { wxListCtrl_SetItem(self.handle(), index, col, label.handle(), imageId) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: c_int, data: c_int) -> bool {
        unsafe { wxListCtrl_SetItemData(self.handle(), item, data) }
    }
    #[fixed_stack_segment]
    fn setItemFromInfo(&self, info: @_wxListItem) -> bool {
        unsafe { wxListCtrl_SetItemFromInfo(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemImage(&self, item: c_int, image: c_int, selImage: c_int) -> bool {
        unsafe { wxListCtrl_SetItemImage(self.handle(), item, image, selImage) }
    }
    #[fixed_stack_segment]
    fn setItemPosition(&self, item: c_int, x: c_int, y: c_int) -> bool {
        unsafe { wxListCtrl_SetItemPosition(self.handle(), item, x, y) }
    }
    #[fixed_stack_segment]
    fn setItemState(&self, item: c_int, state: c_int, stateMask: c_int) -> bool {
        unsafe { wxListCtrl_SetItemState(self.handle(), item, state, stateMask) }
    }
    #[fixed_stack_segment]
    fn setItemText(&self, item: c_int, str: @_wxString) {
        unsafe { wxListCtrl_SetItemText(self.handle(), item, str.handle()) }
    }
    #[fixed_stack_segment]
    fn setSingleStyle(&self, style: c_int, add: bool) {
        unsafe { wxListCtrl_SetSingleStyle(self.handle(), style, add) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @_wxColour) {
        unsafe { wxListCtrl_SetTextColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setWindowStyleFlag(&self, style: c_int) {
        unsafe { wxListCtrl_SetWindowStyleFlag(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn sortItems(&self, fn_: *u8, eif_obj: *u8) -> bool {
        unsafe { wxListCtrl_SortItems(self.handle(), fn_, eif_obj) }
    }
    #[fixed_stack_segment]
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, images: @_wxImageList, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.handle(), images.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getColumn2(&self, col: c_int, item: @_wxListItem) {
        unsafe { wxListCtrl_GetColumn2(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem2(&self, info: @_wxListItem) {
        unsafe { wxListCtrl_GetItem2(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemPosition2(&self, item: c_int) -> @_wxPoint {
        unsafe { @wxPoint(wxListCtrl_GetItemPosition2(self.handle(), item)) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn sortItems2(&self, closure: @_wxClosure) -> bool {
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
    pub fn from(handle: *u8) -> @_wxListEvent {
        @wxListEvent(handle) as @_wxListEvent
    }
    
}

trait _wxListEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn cancelled(&self) -> bool {
        unsafe { wxListEvent_Cancelled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCode(&self) -> c_int {
        unsafe { wxListEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self) -> c_int {
        unsafe { wxListEvent_GetColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> c_int {
        unsafe { wxListEvent_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getImage(&self) -> c_int {
        unsafe { wxListEvent_GetImage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getIndex(&self) -> c_int {
        unsafe { wxListEvent_GetIndex(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, _ref: @_wxListItem) {
        unsafe { wxListEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @_wxString {
        unsafe { @wxString(wxListEvent_GetLabel(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxListEvent_GetPoint(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @_wxString {
        unsafe { @wxString(wxListEvent_GetText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getCacheFrom(&self) -> c_int {
        unsafe { wxListEvent_GetCacheFrom(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCacheTo(&self) -> c_int {
        unsafe { wxListEvent_GetCacheTo(self.handle()) }
    }
}

struct wxListItem(*u8);
impl _wxListItem for wxListItem {}
impl _wxObject for wxListItem { fn handle(&self) -> *u8 { **self } }

impl wxListItem {
    pub fn from(handle: *u8) -> @_wxListItem {
        @wxListItem(handle) as @_wxListItem
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxListItem {
        unsafe { @wxListItem(wxListItem_Create()) as @_wxListItem }
    }
}

trait _wxListItem : _wxObject {
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxListItem_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearAttributes(&self) {
        unsafe { wxListItem_ClearAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxListItem_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAlign(&self) -> c_int {
        unsafe { wxListItem_GetAlign(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAttributes(&self) -> *u8 {
        unsafe { wxListItem_GetAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @_wxColour) {
        unsafe { wxListItem_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self) -> c_int {
        unsafe { wxListItem_GetColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> c_int {
        unsafe { wxListItem_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @_wxFont) {
        unsafe { wxListItem_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxListItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getImage(&self) -> c_int {
        unsafe { wxListItem_GetImage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> c_int {
        unsafe { wxListItem_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getState(&self) -> c_int {
        unsafe { wxListItem_GetState(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @_wxString {
        unsafe { @wxString(wxListItem_GetText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @_wxColour) {
        unsafe { wxListItem_GetTextColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxListItem_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasAttributes(&self) -> bool {
        unsafe { wxListItem_HasAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAlign(&self, align: c_int) {
        unsafe { wxListItem_SetAlign(self.handle(), align) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colBack: @_wxColour) {
        unsafe { wxListItem_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumn(&self, col: c_int) {
        unsafe { wxListItem_SetColumn(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: c_int) {
        unsafe { wxListItem_SetData(self.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setDataPointer(&self, data: *u8) {
        unsafe { wxListItem_SetDataPointer(self.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont) {
        unsafe { wxListItem_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: c_int) {
        unsafe { wxListItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setImage(&self, image: c_int) {
        unsafe { wxListItem_SetImage(self.handle(), image) }
    }
    #[fixed_stack_segment]
    fn setMask(&self, mask: c_int) {
        unsafe { wxListItem_SetMask(self.handle(), mask) }
    }
    #[fixed_stack_segment]
    fn setState(&self, state: c_int) {
        unsafe { wxListItem_SetState(self.handle(), state) }
    }
    #[fixed_stack_segment]
    fn setStateMask(&self, stateMask: c_int) {
        unsafe { wxListItem_SetStateMask(self.handle(), stateMask) }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @_wxString) {
        unsafe { wxListItem_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @_wxColour) {
        unsafe { wxListItem_SetTextColour(self.handle(), colText.handle()) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.handle(), width) }
    }
}

struct wxLocale(*u8);
impl _wxLocale for wxLocale { fn handle(&self) -> *u8 { **self } }

impl wxLocale {
    pub fn from(handle: *u8) -> @_wxLocale {
        @wxLocale(handle) as @_wxLocale
    }
    
    #[fixed_stack_segment]
    pub fn new(_name: c_int, _flags: c_int) -> @_wxLocale {
        unsafe { @wxLocale(wxLocale_Create(_name, _flags)) as @_wxLocale }
    }
}

trait _wxLocale {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addCatalog(&self, szDomain: *u8) -> c_int {
        unsafe { wxLocale_AddCatalog(self.handle(), szDomain) }
    }
    #[fixed_stack_segment]
    fn addCatalogLookupPathPrefix(&self, prefix: *u8) {
        unsafe { wxLocale_AddCatalogLookupPathPrefix(self.handle(), prefix) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLocale_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLocale(&self) -> @_wxLocale {
        unsafe { @wxLocale(wxLocale_GetLocale(self.handle())) as @_wxLocale }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @_wxString {
        unsafe { @wxString(wxLocale_GetName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getString(&self, szOrigString: *u8, szDomain: *u8) -> *wchar_t {
        unsafe { wxLocale_GetString(self.handle(), szOrigString, szDomain) }
    }
    #[fixed_stack_segment]
    fn isLoaded(&self, szDomain: *u8) -> bool {
        unsafe { wxLocale_IsLoaded(self.handle(), szDomain) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxLocale_IsOk(self.handle()) }
    }
}

struct wxLog(*u8);
impl _wxLog for wxLog { fn handle(&self) -> *u8 { **self } }

impl wxLog {
    pub fn from(handle: *u8) -> @_wxLog {
        @wxLog(handle) as @_wxLog
    }
    
    #[fixed_stack_segment]
    pub fn getActiveTarget() -> @_wxLog {
        unsafe { @wxLog(wxLog_GetActiveTarget()) as @_wxLog }
    }
}

trait _wxLog {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: @_wxString) {
        unsafe { wxLog_AddTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLog_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn dontCreateOnDemand(&self) {
        unsafe { wxLog_DontCreateOnDemand(self.handle()) }
    }
    #[fixed_stack_segment]
    fn flush(&self) {
        unsafe { wxLog_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    fn flushActive(&self) {
        unsafe { wxLog_FlushActive(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTimestamp(&self) -> *char {
        unsafe { wxLog_GetTimestamp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTraceMask(&self) -> c_int {
        unsafe { wxLog_GetTraceMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVerbose(&self) -> c_int {
        unsafe { wxLog_GetVerbose(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasPendingMessages(&self) -> bool {
        unsafe { wxLog_HasPendingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isAllowedTraceMask(&self, mask: @_wxMask) -> bool {
        unsafe { wxLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: c_int, szString: *wchar_t, t: c_int) {
        unsafe { wxLog_OnLog(self.handle(), level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask(&self, str: @_wxString) {
        unsafe { wxLog_RemoveTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxLog_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) -> @_wxLog {
        unsafe { @wxLog(wxLog_SetActiveTarget(self.handle())) as @_wxLog }
    }
    #[fixed_stack_segment]
    fn setTimestamp(&self, ts: *wchar_t) {
        unsafe { wxLog_SetTimestamp(self.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn setTraceMask(&self, ulMask: c_int) {
        unsafe { wxLog_SetTraceMask(self.handle(), ulMask) }
    }
    #[fixed_stack_segment]
    fn setVerbose(&self, bVerbose: c_int) {
        unsafe { wxLog_SetVerbose(self.handle(), bVerbose) }
    }
    #[fixed_stack_segment]
    fn suspend(&self) {
        unsafe { wxLog_Suspend(self.handle()) }
    }
}

struct wxLogChain(*u8);
impl _wxLogChain for wxLogChain {}
impl _wxLog for wxLogChain { fn handle(&self) -> *u8 { **self } }

impl wxLogChain {
    pub fn from(handle: *u8) -> @_wxLogChain {
        @wxLogChain(handle) as @_wxLogChain
    }
    
    #[fixed_stack_segment]
    pub fn new(logger: @_wxLog) -> @_wxLogChain {
        unsafe { @wxLogChain(wxLogChain_Create(logger.handle())) as @_wxLogChain }
    }
}

trait _wxLogChain : _wxLog {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLogChain_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOldLog(&self) -> @_wxLog {
        unsafe { @wxLog(wxLogChain_GetOldLog(self.handle())) as @_wxLog }
    }
    #[fixed_stack_segment]
    fn isPassingMessages(&self) -> bool {
        unsafe { wxLogChain_IsPassingMessages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn passMessages(&self, bDoPass: bool) {
        unsafe { wxLogChain_PassMessages(self.handle(), bDoPass) }
    }
    #[fixed_stack_segment]
    fn setLog(&self, logger: @_wxLog) {
        unsafe { wxLogChain_SetLog(self.handle(), logger.handle()) }
    }
}

struct wxLogGUI(*u8);
impl _wxLogGUI for wxLogGUI {}
impl _wxLog for wxLogGUI { fn handle(&self) -> *u8 { **self } }

impl wxLogGUI {
    pub fn from(handle: *u8) -> @_wxLogGUI {
        @wxLogGUI(handle) as @_wxLogGUI
    }
    
}

trait _wxLogGUI : _wxLog {
}

struct wxLogNull(*u8);
impl _wxLogNull for wxLogNull {}
impl _wxLog for wxLogNull { fn handle(&self) -> *u8 { **self } }

impl wxLogNull {
    pub fn from(handle: *u8) -> @_wxLogNull {
        @wxLogNull(handle) as @_wxLogNull
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxLogNull {
        unsafe { @wxLogNull(wxLogNull_Create()) as @_wxLogNull }
    }
}

trait _wxLogNull : _wxLog {
}

struct wxLogPassThrough(*u8);
impl _wxLogPassThrough for wxLogPassThrough {}
impl _wxLogChain for wxLogPassThrough {}
impl _wxLog for wxLogPassThrough { fn handle(&self) -> *u8 { **self } }

impl wxLogPassThrough {
    pub fn from(handle: *u8) -> @_wxLogPassThrough {
        @wxLogPassThrough(handle) as @_wxLogPassThrough
    }
    
}

trait _wxLogPassThrough : _wxLogChain {
}

struct wxLogStderr(*u8);
impl _wxLogStderr for wxLogStderr {}
impl _wxLog for wxLogStderr { fn handle(&self) -> *u8 { **self } }

impl wxLogStderr {
    pub fn from(handle: *u8) -> @_wxLogStderr {
        @wxLogStderr(handle) as @_wxLogStderr
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxLogStderr {
        unsafe { @wxLogStderr(wxLogStderr_Create()) as @_wxLogStderr }
    }
    #[fixed_stack_segment]
    pub fn newStdOut() -> @_wxLogStderr {
        unsafe { @wxLogStderr(wxLogStderr_CreateStdOut()) as @_wxLogStderr }
    }
}

trait _wxLogStderr : _wxLog {
}

struct wxLogStream(*u8);
impl _wxLogStream for wxLogStream {}
impl _wxLog for wxLogStream { fn handle(&self) -> *u8 { **self } }

impl wxLogStream {
    pub fn from(handle: *u8) -> @_wxLogStream {
        @wxLogStream(handle) as @_wxLogStream
    }
    
}

trait _wxLogStream : _wxLog {
}

struct wxLogTextCtrl(*u8);
impl _wxLogTextCtrl for wxLogTextCtrl {}
impl _wxLog for wxLogTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxLogTextCtrl {
    pub fn from(handle: *u8) -> @_wxLogTextCtrl {
        @wxLogTextCtrl(handle) as @_wxLogTextCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(text: @_wxTextCtrl) -> @_wxLogTextCtrl {
        unsafe { @wxLogTextCtrl(wxLogTextCtrl_Create(text.handle())) as @_wxLogTextCtrl }
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
    pub fn from(handle: *u8) -> @_wxLogWindow {
        @wxLogWindow(handle) as @_wxLogWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, title: *wchar_t, showit: bool, passthrough: bool) -> @_wxLogWindow {
        unsafe { @wxLogWindow(wxLogWindow_Create(parent.handle(), title, showit, passthrough)) as @_wxLogWindow }
    }
}

trait _wxLogWindow : _wxLogPassThrough {
    #[fixed_stack_segment]
    fn getFrame(&self) -> @_wxFrame {
        unsafe { @wxFrame(wxLogWindow_GetFrame(self.handle())) as @_wxFrame }
    }
}

struct wxLongLong(*u8);
impl _wxLongLong for wxLongLong { fn handle(&self) -> *u8 { **self } }

impl wxLongLong {
    pub fn from(handle: *u8) -> @_wxLongLong {
        @wxLongLong(handle) as @_wxLongLong
    }
    
}

trait _wxLongLong {
    fn handle(&self) -> *u8;
    
}

struct wxMBConv(*u8);
impl _wxMBConv for wxMBConv { fn handle(&self) -> *u8 { **self } }

impl wxMBConv {
    pub fn from(handle: *u8) -> @_wxMBConv {
        @wxMBConv(handle) as @_wxMBConv
    }
    
}

trait _wxMBConv {
    fn handle(&self) -> *u8;
    
}

struct wxMBConvFile(*u8);
impl _wxMBConvFile for wxMBConvFile {}
impl _wxMBConv for wxMBConvFile { fn handle(&self) -> *u8 { **self } }

impl wxMBConvFile {
    pub fn from(handle: *u8) -> @_wxMBConvFile {
        @wxMBConvFile(handle) as @_wxMBConvFile
    }
    
}

trait _wxMBConvFile : _wxMBConv {
}

struct wxMBConvUTF7(*u8);
impl _wxMBConvUTF7 for wxMBConvUTF7 {}
impl _wxMBConv for wxMBConvUTF7 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF7 {
    pub fn from(handle: *u8) -> @_wxMBConvUTF7 {
        @wxMBConvUTF7(handle) as @_wxMBConvUTF7
    }
    
}

trait _wxMBConvUTF7 : _wxMBConv {
}

struct wxMBConvUTF8(*u8);
impl _wxMBConvUTF8 for wxMBConvUTF8 {}
impl _wxMBConv for wxMBConvUTF8 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF8 {
    pub fn from(handle: *u8) -> @_wxMBConvUTF8 {
        @wxMBConvUTF8(handle) as @_wxMBConvUTF8
    }
    
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
    pub fn from(handle: *u8) -> @_wxMDIChildFrame {
        @wxMDIChildFrame(handle) as @_wxMDIChildFrame
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxMDIChildFrame {
        unsafe { @wxMDIChildFrame(wxMDIChildFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxMDIChildFrame }
    }
}

trait _wxMDIChildFrame : _wxFrame {
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxMDIClientWindow {
        @wxMDIClientWindow(handle) as @_wxMDIClientWindow
    }
    
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
    pub fn from(handle: *u8) -> @_wxMDIParentFrame {
        @wxMDIParentFrame(handle) as @_wxMDIParentFrame
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxMDIParentFrame {
        unsafe { @wxMDIParentFrame(wxMDIParentFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxMDIParentFrame }
    }
}

trait _wxMDIParentFrame : _wxFrame {
    #[fixed_stack_segment]
    fn activateNext(&self) {
        unsafe { wxMDIParentFrame_ActivateNext(self.handle()) }
    }
    #[fixed_stack_segment]
    fn activatePrevious(&self) {
        unsafe { wxMDIParentFrame_ActivatePrevious(self.handle()) }
    }
    #[fixed_stack_segment]
    fn arrangeIcons(&self) {
        unsafe { wxMDIParentFrame_ArrangeIcons(self.handle()) }
    }
    #[fixed_stack_segment]
    fn cascade(&self) {
        unsafe { wxMDIParentFrame_Cascade(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getActiveChild(&self) -> @_wxMDIChildFrame {
        unsafe { @wxMDIChildFrame(wxMDIParentFrame_GetActiveChild(self.handle())) as @_wxMDIChildFrame }
    }
    #[fixed_stack_segment]
    fn getClientWindow(&self) -> @_wxMDIClientWindow {
        unsafe { @wxMDIClientWindow(wxMDIParentFrame_GetClientWindow(self.handle())) as @_wxMDIClientWindow }
    }
    #[fixed_stack_segment]
    fn getWindowMenu(&self) -> @_wxMenu {
        unsafe { @wxMenu(wxMDIParentFrame_GetWindowMenu(self.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn onCreateClient(&self) -> @_wxMDIClientWindow {
        unsafe { @wxMDIClientWindow(wxMDIParentFrame_OnCreateClient(self.handle())) as @_wxMDIClientWindow }
    }
    #[fixed_stack_segment]
    fn setWindowMenu(&self, menu: @_wxMenu) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.handle()) }
    }
}

struct wxMask(*u8);
impl _wxMask for wxMask {}
impl _wxObject for wxMask { fn handle(&self) -> *u8 { **self } }

impl wxMask {
    pub fn from(handle: *u8) -> @_wxMask {
        @wxMask(handle) as @_wxMask
    }
    
    #[fixed_stack_segment]
    pub fn new(bitmap: @_wxBitmap) -> @_wxMask {
        unsafe { @wxMask(wxMask_Create(bitmap.handle())) as @_wxMask }
    }
    #[fixed_stack_segment]
    pub fn newColoured(bitmap: @_wxBitmap, colour: @_wxColour) -> *u8 {
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
    pub fn from(handle: *u8) -> @_wxMaximizeEvent {
        @wxMaximizeEvent(handle) as @_wxMaximizeEvent
    }
    
}

trait _wxMaximizeEvent : _wxEvent {
}

struct wxMemoryDC(*u8);
impl _wxMemoryDC for wxMemoryDC {}
impl _wxDC for wxMemoryDC {}
impl _wxObject for wxMemoryDC { fn handle(&self) -> *u8 { **self } }

impl wxMemoryDC {
    pub fn from(handle: *u8) -> @_wxMemoryDC {
        @wxMemoryDC(handle) as @_wxMemoryDC
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxMemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_Create()) as @_wxMemoryDC }
    }
    #[fixed_stack_segment]
    pub fn newCompatible(dc: @_wxDC) -> @_wxMemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_CreateCompatible(dc.handle())) as @_wxMemoryDC }
    }
    #[fixed_stack_segment]
    pub fn newWithBitmap(bitmap: @_wxBitmap) -> @_wxMemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_CreateWithBitmap(bitmap.handle())) as @_wxMemoryDC }
    }
}

trait _wxMemoryDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMemoryDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selectObject(&self, bitmap: @_wxBitmap) {
        unsafe { wxMemoryDC_SelectObject(self.handle(), bitmap.handle()) }
    }
}

struct wxMemoryFSHandler(*u8);
impl _wxMemoryFSHandler for wxMemoryFSHandler {}
impl _wxFileSystemHandler for wxMemoryFSHandler {}
impl _wxObject for wxMemoryFSHandler { fn handle(&self) -> *u8 { **self } }

impl wxMemoryFSHandler {
    pub fn from(handle: *u8) -> @_wxMemoryFSHandler {
        @wxMemoryFSHandler(handle) as @_wxMemoryFSHandler
    }
    
}

trait _wxMemoryFSHandler : _wxFileSystemHandler {
}

struct wxMemoryInputStream(*u8);
impl _wxMemoryInputStream for wxMemoryInputStream {}
impl _wxInputStream for wxMemoryInputStream {}
impl _wxStreamBase for wxMemoryInputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryInputStream {
    pub fn from(handle: *u8) -> @_wxMemoryInputStream {
        @wxMemoryInputStream(handle) as @_wxMemoryInputStream
    }
    
}

trait _wxMemoryInputStream : _wxInputStream {
}

struct wxMemoryOutputStream(*u8);
impl _wxMemoryOutputStream for wxMemoryOutputStream {}
impl _wxOutputStream for wxMemoryOutputStream {}
impl _wxStreamBase for wxMemoryOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryOutputStream {
    pub fn from(handle: *u8) -> @_wxMemoryOutputStream {
        @wxMemoryOutputStream(handle) as @_wxMemoryOutputStream
    }
    
}

trait _wxMemoryOutputStream : _wxOutputStream {
}

struct wxMenu(*u8);
impl _wxMenu for wxMenu {}
impl _wxEvtHandler for wxMenu {}
impl _wxObject for wxMenu { fn handle(&self) -> *u8 { **self } }

impl wxMenu {
    pub fn from(handle: *u8) -> @_wxMenu {
        @wxMenu(handle) as @_wxMenu
    }
    
    #[fixed_stack_segment]
    pub fn new(title: @_wxString, style: c_long) -> @_wxMenu {
        unsafe { @wxMenu(wxMenu_Create(title.handle(), style)) as @_wxMenu }
    }
}

trait _wxMenu : _wxEvtHandler {
    #[fixed_stack_segment]
    fn append(&self, id: c_int, text: @_wxString, help: @_wxString, isCheckable: bool) {
        unsafe { wxMenu_Append(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, _itm: @_wxMenuItem) {
        unsafe { wxMenu_AppendItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSub(&self, id: c_int, text: @_wxString, submenu: @_wxMenu, help: @_wxString) {
        unsafe { wxMenu_AppendSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    fn break_(&self) {
        unsafe { wxMenu_Break(self.handle()) }
    }
    #[fixed_stack_segment]
    fn check(&self, id: c_int, check: bool) {
        unsafe { wxMenu_Check(self.handle(), id, check) }
    }
    #[fixed_stack_segment]
    fn deleteById(&self, id: c_int) {
        unsafe { wxMenu_DeleteById(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn deleteByItem(&self, _itm: @_wxMenuItem) {
        unsafe { wxMenu_DeleteByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn destroyById(&self, id: c_int) {
        unsafe { wxMenu_DestroyById(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn destroyByItem(&self, _itm: @_wxMenuItem) {
        unsafe { wxMenu_DestroyByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { wxMenu_Enable(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: c_int) -> @_wxMenuItem {
        unsafe { @wxMenuItem(wxMenu_FindItem(self.handle(), id)) as @_wxMenuItem }
    }
    #[fixed_stack_segment]
    fn findItemByLabel(&self, itemString: @_wxString) -> c_int {
        unsafe { wxMenu_FindItemByLabel(self.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @_wxClientData {
        unsafe { @wxClientData(wxMenu_GetClientData(self.handle())) as @_wxClientData }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> @_wxString {
        unsafe { @wxString(wxMenu_GetHelpString(self.handle(), id)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getInvokingWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxMenu_GetInvokingWindow(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> @_wxString {
        unsafe { @wxString(wxMenu_GetLabel(self.handle(), id)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMenuItems(&self, _lst: @_wxList) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @_wxMenu {
        unsafe { @wxMenu(wxMenu_GetParent(self.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @_wxString {
        unsafe { @wxString(wxMenu_GetTitle(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: size_t, id: c_int, text: @_wxString, help: @_wxString, isCheckable: bool) {
        unsafe { wxMenu_Insert(self.handle(), pos, id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, pos: size_t, _itm: @_wxMenuItem) {
        unsafe { wxMenu_InsertItem(self.handle(), pos, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSub(&self, pos: size_t, id: c_int, text: @_wxString, submenu: @_wxMenu, help: @_wxString) {
        unsafe { wxMenu_InsertSub(self.handle(), pos, id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    fn isAttached(&self) -> bool {
        unsafe { wxMenu_IsAttached(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self, id: c_int) -> bool {
        unsafe { wxMenu_IsChecked(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self, id: c_int) -> bool {
        unsafe { wxMenu_IsEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn prepend(&self, id: c_int, text: @_wxString, help: @_wxString, isCheckable: bool) {
        unsafe { wxMenu_Prepend(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, _itm: @_wxMenuItem) {
        unsafe { wxMenu_PrependItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn prependSub(&self, id: c_int, text: @_wxString, submenu: @_wxMenu, help: @_wxString) {
        unsafe { wxMenu_PrependSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    fn removeById(&self, id: c_int, _itm: @_wxMenuItem) {
        unsafe { wxMenu_RemoveById(self.handle(), id, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn removeByItem(&self, item: *u8) {
        unsafe { wxMenu_RemoveByItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, clientData: @_wxClientData) {
        unsafe { wxMenu_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setEventHandler(&self, handler: @_wxEvtHandler) {
        unsafe { wxMenu_SetEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: c_int, helpString: @_wxString) {
        unsafe { wxMenu_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setInvokingWindow(&self, win: @_wxWindow) {
        unsafe { wxMenu_SetInvokingWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, id: c_int, label: @_wxString) {
        unsafe { wxMenu_SetLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setParent(&self, parent: @_wxWindow) {
        unsafe { wxMenu_SetParent(self.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, title: @_wxString) {
        unsafe { wxMenu_SetTitle(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn updateUI(&self, source: *u8) {
        unsafe { wxMenu_UpdateUI(self.handle(), source) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> @_wxMenuBar {
        unsafe { @wxMenuBar(wxMenu_GetMenuBar(self.handle())) as @_wxMenuBar }
    }
    #[fixed_stack_segment]
    fn appendRadioItem(&self, id: c_int, text: @_wxString, help: @_wxString) {
        unsafe { wxMenu_AppendRadioItem(self.handle(), id, text.handle(), help.handle()) }
    }
}

struct wxMenuBar(*u8);
impl _wxMenuBar for wxMenuBar {}
impl _wxEvtHandler for wxMenuBar {}
impl _wxObject for wxMenuBar { fn handle(&self) -> *u8 { **self } }

impl wxMenuBar {
    pub fn from(handle: *u8) -> @_wxMenuBar {
        @wxMenuBar(handle) as @_wxMenuBar
    }
    
    #[fixed_stack_segment]
    pub fn new(_style: c_int) -> @_wxMenuBar {
        unsafe { @wxMenuBar(wxMenuBar_Create(_style)) as @_wxMenuBar }
    }
}

trait _wxMenuBar : _wxEvtHandler {
    #[fixed_stack_segment]
    fn append(&self, menu: @_wxMenu, title: @_wxString) -> c_int {
        unsafe { wxMenuBar_Append(self.handle(), menu.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn check(&self, id: c_int, check: bool) {
        unsafe { wxMenuBar_Check(self.handle(), id, check) }
    }
    #[fixed_stack_segment]
    fn deletePointer(&self) {
        unsafe { wxMenuBar_DeletePointer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> c_int {
        unsafe { wxMenuBar_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn enableItem(&self, id: c_int, enable: bool) {
        unsafe { wxMenuBar_EnableItem(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    fn enableTop(&self, pos: c_int, enable: bool) {
        unsafe { wxMenuBar_EnableTop(self.handle(), pos, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: c_int) -> @_wxMenuItem {
        unsafe { @wxMenuItem(wxMenuBar_FindItem(self.handle(), id)) as @_wxMenuItem }
    }
    #[fixed_stack_segment]
    fn findMenu(&self, title: @_wxString) -> c_int {
        unsafe { wxMenuBar_FindMenu(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn findMenuItem(&self, menuString: @_wxString, itemString: @_wxString) -> c_int {
        unsafe { wxMenuBar_FindMenuItem(self.handle(), menuString.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> @_wxString {
        unsafe { @wxString(wxMenuBar_GetHelpString(self.handle(), id)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> @_wxString {
        unsafe { @wxString(wxMenuBar_GetLabel(self.handle(), id)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getLabelTop(&self, pos: c_int) -> @_wxString {
        unsafe { @wxString(wxMenuBar_GetLabelTop(self.handle(), pos)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, pos: c_int) -> @_wxMenu {
        unsafe { @wxMenu(wxMenuBar_GetMenu(self.handle(), pos)) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: c_int, menu: @_wxMenu, title: @_wxString) -> c_int {
        unsafe { wxMenuBar_Insert(self.handle(), pos, menu.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self, id: c_int) -> bool {
        unsafe { wxMenuBar_IsChecked(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self, id: c_int) -> bool {
        unsafe { wxMenuBar_IsEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn remove(&self, pos: c_int) -> @_wxMenu {
        unsafe { @wxMenu(wxMenuBar_Remove(self.handle(), pos)) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn replace(&self, pos: c_int, menu: @_wxMenu, title: @_wxString) -> @_wxMenu {
        unsafe { @wxMenu(wxMenuBar_Replace(self.handle(), pos, menu.handle(), title.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: c_int, helpString: @_wxString) {
        unsafe { wxMenuBar_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, id: c_int, label: @_wxString) {
        unsafe { wxMenuBar_SetItemLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, s: @_wxString) {
        unsafe { wxMenuBar_SetLabel(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTop(&self, pos: c_int, label: @_wxString) {
        unsafe { wxMenuBar_SetLabelTop(self.handle(), pos, label.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @_wxFrame {
        unsafe { @wxFrame(wxMenuBar_GetFrame(self.handle())) as @_wxFrame }
    }
}

struct wxMenuEvent(*u8);
impl _wxMenuEvent for wxMenuEvent {}
impl _wxEvent for wxMenuEvent {}
impl _wxObject for wxMenuEvent { fn handle(&self) -> *u8 { **self } }

impl wxMenuEvent {
    pub fn from(handle: *u8) -> @_wxMenuEvent {
        @wxMenuEvent(handle) as @_wxMenuEvent
    }
    
}

trait _wxMenuEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxMenuEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.handle()) }
    }
}

struct wxMenuItem(*u8);
impl _wxMenuItem for wxMenuItem {}
impl _wxObject for wxMenuItem { fn handle(&self) -> *u8 { **self } }

impl wxMenuItem {
    pub fn from(handle: *u8) -> @_wxMenuItem {
        @wxMenuItem(handle) as @_wxMenuItem
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxMenuItem {
        unsafe { @wxMenuItem(wxMenuItem_Create()) as @_wxMenuItem }
    }
    #[fixed_stack_segment]
    pub fn getLabelFromText(text: *wchar_t) -> @_wxString {
        unsafe { @wxString(wxMenuItem_GetLabelFromText(text)) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn newSeparator() -> @_wxMenuItem {
        unsafe { @wxMenuItem(wxMenuItem_CreateSeparator()) as @_wxMenuItem }
    }
    #[fixed_stack_segment]
    pub fn newEx(id: c_int, label: @_wxString, help: @_wxString, itemkind: c_int, submenu: @_wxMenu) -> @_wxMenuItem {
        unsafe { @wxMenuItem(wxMenuItem_CreateEx(id, label.handle(), help.handle(), itemkind, submenu.handle())) as @_wxMenuItem }
    }
}

trait _wxMenuItem : _wxObject {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxMenuItem_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMenuItem_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) {
        unsafe { wxMenuItem_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getHelp(&self) -> @_wxString {
        unsafe { @wxString(wxMenuItem_GetHelp(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @_wxString {
        unsafe { @wxString(wxMenuItem_GetLabel(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getMenu(&self) -> @_wxMenu {
        unsafe { @wxMenu(wxMenuItem_GetMenu(self.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn getSubMenu(&self) -> @_wxMenu {
        unsafe { @wxMenu(wxMenuItem_GetSubMenu(self.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @_wxString {
        unsafe { @wxString(wxMenuItem_GetText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn isCheckable(&self) -> bool {
        unsafe { wxMenuItem_IsCheckable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self) -> bool {
        unsafe { wxMenuItem_IsChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self) -> bool {
        unsafe { wxMenuItem_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSeparator(&self) -> bool {
        unsafe { wxMenuItem_IsSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSubMenu(&self) -> bool {
        unsafe { wxMenuItem_IsSubMenu(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCheckable(&self, checkable: bool) {
        unsafe { wxMenuItem_SetCheckable(self.handle(), checkable) }
    }
    #[fixed_stack_segment]
    fn setHelp(&self, str: @_wxString) {
        unsafe { wxMenuItem_SetHelp(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setSubMenu(&self, menu: @_wxMenu) {
        unsafe { wxMenuItem_SetSubMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn setText(&self, str: @_wxString) {
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
    pub fn from(handle: *u8) -> @_wxMessageDialog {
        @wxMessageDialog(handle) as @_wxMessageDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _msg: @_wxString, _cap: @_wxString, _stl: c_int) -> @_wxMessageDialog {
        unsafe { @wxMessageDialog(wxMessageDialog_Create(_prt.handle(), _msg.handle(), _cap.handle(), _stl)) as @_wxMessageDialog }
    }
}

trait _wxMessageDialog : _wxDialog {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMessageDialog_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn showModal(&self) -> c_int {
        unsafe { wxMessageDialog_ShowModal(self.handle()) }
    }
}

struct wxMetafile(*u8);
impl _wxMetafile for wxMetafile {}
impl _wxObject for wxMetafile { fn handle(&self) -> *u8 { **self } }

impl wxMetafile {
    pub fn from(handle: *u8) -> @_wxMetafile {
        @wxMetafile(handle) as @_wxMetafile
    }
    
    #[fixed_stack_segment]
    pub fn new(_file: @_wxString) -> @_wxMetafile {
        unsafe { @wxMetafile(wxMetafile_Create(_file.handle())) as @_wxMetafile }
    }
}

trait _wxMetafile : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafile_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxMetafile_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn play(&self, _dc: @_wxDC) -> bool {
        unsafe { wxMetafile_Play(self.handle(), _dc.handle()) }
    }
    #[fixed_stack_segment]
    fn setClipboard(&self, width: c_int, height: c_int) -> bool {
        unsafe { wxMetafile_SetClipboard(self.handle(), width, height) }
    }
}

struct wxMetafileDC(*u8);
impl _wxMetafileDC for wxMetafileDC {}
impl _wxDC for wxMetafileDC {}
impl _wxObject for wxMetafileDC { fn handle(&self) -> *u8 { **self } }

impl wxMetafileDC {
    pub fn from(handle: *u8) -> @_wxMetafileDC {
        @wxMetafileDC(handle) as @_wxMetafileDC
    }
    
    #[fixed_stack_segment]
    pub fn new(_file: @_wxString) -> @_wxMetafileDC {
        unsafe { @wxMetafileDC(wxMetafileDC_Create(_file.handle())) as @_wxMetafileDC }
    }
}

trait _wxMetafileDC : _wxDC {
    #[fixed_stack_segment]
    fn close(&self) -> *u8 {
        unsafe { wxMetafileDC_Close(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafileDC_Delete(self.handle()) }
    }
}

struct wxMimeTypesManager(*u8);
impl _wxMimeTypesManager for wxMimeTypesManager { fn handle(&self) -> *u8 { **self } }

impl wxMimeTypesManager {
    pub fn from(handle: *u8) -> @_wxMimeTypesManager {
        @wxMimeTypesManager(handle) as @_wxMimeTypesManager
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxMimeTypesManager {
        unsafe { @wxMimeTypesManager(wxMimeTypesManager_Create()) as @_wxMimeTypesManager }
    }
}

trait _wxMimeTypesManager {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addFallbacks(&self, _types: *u8) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.handle(), _types) }
    }
    #[fixed_stack_segment]
    fn enumAllFileTypes(&self, _lst: @_wxList) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromExtension(&self, _ext: @_wxString) -> @_wxFileType {
        unsafe { @wxFileType(wxMimeTypesManager_GetFileTypeFromExtension(self.handle(), _ext.handle())) as @_wxFileType }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromMimeType(&self, _name: @_wxString) -> @_wxFileType {
        unsafe { @wxFileType(wxMimeTypesManager_GetFileTypeFromMimeType(self.handle(), _name.handle())) as @_wxFileType }
    }
    #[fixed_stack_segment]
    fn isOfType(&self, _type: @_wxString, _wildcard: @_wxString) -> bool {
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
    pub fn from(handle: *u8) -> @_wxMiniFrame {
        @wxMiniFrame(handle) as @_wxMiniFrame
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxMiniFrame {
        unsafe { @wxMiniFrame(wxMiniFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxMiniFrame }
    }
}

trait _wxMiniFrame : _wxFrame {
}

struct wxMirrorDC(*u8);
impl _wxMirrorDC for wxMirrorDC {}
impl _wxDC for wxMirrorDC {}
impl _wxObject for wxMirrorDC { fn handle(&self) -> *u8 { **self } }

impl wxMirrorDC {
    pub fn from(handle: *u8) -> @_wxMirrorDC {
        @wxMirrorDC(handle) as @_wxMirrorDC
    }
    
    #[fixed_stack_segment]
    pub fn new(dc: @_wxDC) -> @_wxMirrorDC {
        unsafe { @wxMirrorDC(wxMirrorDC_Create(dc.handle())) as @_wxMirrorDC }
    }
}

trait _wxMirrorDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMirrorDC_Delete(self.handle()) }
    }
}

struct wxModule(*u8);
impl _wxModule for wxModule {}
impl _wxObject for wxModule { fn handle(&self) -> *u8 { **self } }

impl wxModule {
    pub fn from(handle: *u8) -> @_wxModule {
        @wxModule(handle) as @_wxModule
    }
    
}

trait _wxModule : _wxObject {
}

struct wxMouseCaptureChangedEvent(*u8);
impl _wxMouseCaptureChangedEvent for wxMouseCaptureChangedEvent {}
impl _wxEvent for wxMouseCaptureChangedEvent {}
impl _wxObject for wxMouseCaptureChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseCaptureChangedEvent {
    pub fn from(handle: *u8) -> @_wxMouseCaptureChangedEvent {
        @wxMouseCaptureChangedEvent(handle) as @_wxMouseCaptureChangedEvent
    }
    
}

trait _wxMouseCaptureChangedEvent : _wxEvent {
}

struct wxMouseEvent(*u8);
impl _wxMouseEvent for wxMouseEvent {}
impl _wxEvent for wxMouseEvent {}
impl _wxObject for wxMouseEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseEvent {
    pub fn from(handle: *u8) -> @_wxMouseEvent {
        @wxMouseEvent(handle) as @_wxMouseEvent
    }
    
}

trait _wxMouseEvent : _wxEvent {
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxMouseEvent_AltDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn button(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_Button(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn buttonDClick(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonDClick(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn buttonDown(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn buttonIsDown(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonIsDown(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn buttonUp(&self, but: c_int) -> bool {
        unsafe { wxMouseEvent_ButtonUp(self.handle(), but) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxMouseEvent_ControlDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxMouseEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn dragging(&self) -> bool {
        unsafe { wxMouseEvent_Dragging(self.handle()) }
    }
    #[fixed_stack_segment]
    fn entering(&self) -> bool {
        unsafe { wxMouseEvent_Entering(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self, dc: @_wxDC) -> @_wxPoint {
        unsafe { @wxPoint(wxMouseEvent_GetLogicalPosition(self.handle(), dc.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxMouseEvent_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> c_int {
        unsafe { wxMouseEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> c_int {
        unsafe { wxMouseEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isButton(&self) -> bool {
        unsafe { wxMouseEvent_IsButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leaving(&self) -> bool {
        unsafe { wxMouseEvent_Leaving(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leftDClick(&self) -> bool {
        unsafe { wxMouseEvent_LeftDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leftDown(&self) -> bool {
        unsafe { wxMouseEvent_LeftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leftIsDown(&self) -> bool {
        unsafe { wxMouseEvent_LeftIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leftUp(&self) -> bool {
        unsafe { wxMouseEvent_LeftUp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxMouseEvent_MetaDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn middleDClick(&self) -> bool {
        unsafe { wxMouseEvent_MiddleDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn middleDown(&self) -> bool {
        unsafe { wxMouseEvent_MiddleDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn middleIsDown(&self) -> bool {
        unsafe { wxMouseEvent_MiddleIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn middleUp(&self) -> bool {
        unsafe { wxMouseEvent_MiddleUp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn moving(&self) -> bool {
        unsafe { wxMouseEvent_Moving(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rightDClick(&self) -> bool {
        unsafe { wxMouseEvent_RightDClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rightDown(&self) -> bool {
        unsafe { wxMouseEvent_RightDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rightIsDown(&self) -> bool {
        unsafe { wxMouseEvent_RightIsDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rightUp(&self) -> bool {
        unsafe { wxMouseEvent_RightUp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxMouseEvent_ShiftDown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWheelDelta(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelDelta(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWheelRotation(&self) -> c_int {
        unsafe { wxMouseEvent_GetWheelRotation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getButton(&self) -> c_int {
        unsafe { wxMouseEvent_GetButton(self.handle()) }
    }
}

struct wxMoveEvent(*u8);
impl _wxMoveEvent for wxMoveEvent {}
impl _wxEvent for wxMoveEvent {}
impl _wxObject for wxMoveEvent { fn handle(&self) -> *u8 { **self } }

impl wxMoveEvent {
    pub fn from(handle: *u8) -> @_wxMoveEvent {
        @wxMoveEvent(handle) as @_wxMoveEvent
    }
    
}

trait _wxMoveEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxMoveEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxMoveEvent_GetPosition(self.handle())) as @_wxPoint }
    }
}

struct wxMultiCellCanvas(*u8);
impl _wxMultiCellCanvas for wxMultiCellCanvas {}
impl _wxFlexGridSizer for wxMultiCellCanvas {}
impl _wxGridSizer for wxMultiCellCanvas {}
impl _wxSizer for wxMultiCellCanvas {}
impl _wxObject for wxMultiCellCanvas { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellCanvas {
    pub fn from(handle: *u8) -> @_wxMultiCellCanvas {
        @wxMultiCellCanvas(handle) as @_wxMultiCellCanvas
    }
    
}

trait _wxMultiCellCanvas : _wxFlexGridSizer {
}

struct wxMultiCellItemHandle(*u8);
impl _wxMultiCellItemHandle for wxMultiCellItemHandle {}
impl _wxObject for wxMultiCellItemHandle { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellItemHandle {
    pub fn from(handle: *u8) -> @_wxMultiCellItemHandle {
        @wxMultiCellItemHandle(handle) as @_wxMultiCellItemHandle
    }
    
}

trait _wxMultiCellItemHandle : _wxObject {
}

struct wxMultiCellSizer(*u8);
impl _wxMultiCellSizer for wxMultiCellSizer {}
impl _wxSizer for wxMultiCellSizer {}
impl _wxObject for wxMultiCellSizer { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellSizer {
    pub fn from(handle: *u8) -> @_wxMultiCellSizer {
        @wxMultiCellSizer(handle) as @_wxMultiCellSizer
    }
    
}

trait _wxMultiCellSizer : _wxSizer {
}

struct wxMutex(*u8);
impl _wxMutex for wxMutex { fn handle(&self) -> *u8 { **self } }

impl wxMutex {
    pub fn from(handle: *u8) -> @_wxMutex {
        @wxMutex(handle) as @_wxMutex
    }
    
}

trait _wxMutex {
    fn handle(&self) -> *u8;
    
}

struct wxMutexLocker(*u8);
impl _wxMutexLocker for wxMutexLocker { fn handle(&self) -> *u8 { **self } }

impl wxMutexLocker {
    pub fn from(handle: *u8) -> @_wxMutexLocker {
        @wxMutexLocker(handle) as @_wxMutexLocker
    }
    
}

trait _wxMutexLocker {
    fn handle(&self) -> *u8;
    
}

struct wxNavigationKeyEvent(*u8);
impl _wxNavigationKeyEvent for wxNavigationKeyEvent {}
impl _wxEvent for wxNavigationKeyEvent {}
impl _wxObject for wxNavigationKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNavigationKeyEvent {
    pub fn from(handle: *u8) -> @_wxNavigationKeyEvent {
        @wxNavigationKeyEvent(handle) as @_wxNavigationKeyEvent
    }
    
}

trait _wxNavigationKeyEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getCurrentFocus(&self) -> *u8 {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDirection(&self) -> bool {
        unsafe { wxNavigationKeyEvent_GetDirection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isWindowChange(&self) -> bool {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrentFocus(&self, win: @_wxWindow) {
        unsafe { wxNavigationKeyEvent_SetCurrentFocus(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn setDirection(&self, bForward: bool) {
        unsafe { wxNavigationKeyEvent_SetDirection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    fn setWindowChange(&self, bIs: bool) {
        unsafe { wxNavigationKeyEvent_SetWindowChange(self.handle(), bIs) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxNewBitmapButton {
        @wxNewBitmapButton(handle) as @_wxNewBitmapButton
    }
    
}

trait _wxNewBitmapButton : _wxPanel {
}

struct wxNodeBase(*u8);
impl _wxNodeBase for wxNodeBase { fn handle(&self) -> *u8 { **self } }

impl wxNodeBase {
    pub fn from(handle: *u8) -> @_wxNodeBase {
        @wxNodeBase(handle) as @_wxNodeBase
    }
    
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
    pub fn from(handle: *u8) -> @_wxNotebook {
        @wxNotebook(handle) as @_wxNotebook
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxNotebook {
        unsafe { @wxNotebook(wxNotebook_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxNotebook }
    }
}

trait _wxNotebook : _wxControl {
    #[fixed_stack_segment]
    fn addPage(&self, pPage: @_wxWindow, strText: @_wxString, bSelect: bool, imageId: c_int) -> bool {
        unsafe { wxNotebook_AddPage(self.handle(), pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn advanceSelection(&self, bForward: bool) {
        unsafe { wxNotebook_AdvanceSelection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    fn deleteAllPages(&self) -> bool {
        unsafe { wxNotebook_DeleteAllPages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deletePage(&self, nPage: c_int) -> bool {
        unsafe { wxNotebook_DeletePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> @_wxImageList {
        unsafe { @wxImageList(wxNotebook_GetImageList(self.handle())) as @_wxImageList }
    }
    #[fixed_stack_segment]
    fn getPage(&self, nPage: c_int) -> @_wxWindow {
        unsafe { @wxWindow(wxNotebook_GetPage(self.handle(), nPage)) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getPageCount(&self) -> c_int {
        unsafe { wxNotebook_GetPageCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPageImage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_GetPageImage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn getPageText(&self, nPage: c_int) -> @_wxString {
        unsafe { @wxString(wxNotebook_GetPageText(self.handle(), nPage)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getRowCount(&self) -> c_int {
        unsafe { wxNotebook_GetRowCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> c_int {
        unsafe { wxNotebook_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int, flags: *c_long) -> c_int {
        unsafe { wxNotebook_HitTest(self.handle(), x, y, flags) }
    }
    #[fixed_stack_segment]
    fn insertPage(&self, nPage: c_int, pPage: @_wxWindow, strText: @_wxString, bSelect: bool, imageId: c_int) -> bool {
        unsafe { wxNotebook_InsertPage(self.handle(), nPage, pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn removePage(&self, nPage: c_int) -> bool {
        unsafe { wxNotebook_RemovePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @_wxImageList) {
        unsafe { wxNotebook_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn setPadding(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPadding(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn setPageImage(&self, nPage: c_int, nImage: c_int) -> bool {
        unsafe { wxNotebook_SetPageImage(self.handle(), nPage, nImage) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPageSize(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn setPageText(&self, nPage: c_int, strText: @_wxString) -> bool {
        unsafe { wxNotebook_SetPageText(self.handle(), nPage, strText.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @_wxImageList) {
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
    pub fn from(handle: *u8) -> @_wxNotebookEvent {
        @wxNotebookEvent(handle) as @_wxNotebookEvent
    }
    
}

trait _wxNotebookEvent : _wxNotifyEvent {
}

struct wxNotifyEvent(*u8);
impl _wxNotifyEvent for wxNotifyEvent {}
impl _wxCommandEvent for wxNotifyEvent {}
impl _wxEvent for wxNotifyEvent {}
impl _wxObject for wxNotifyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNotifyEvent {
    pub fn from(handle: *u8) -> @_wxNotifyEvent {
        @wxNotifyEvent(handle) as @_wxNotifyEvent
    }
    
}

trait _wxNotifyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn allow(&self) {
        unsafe { wxNotifyEvent_Allow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxNotifyEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn isAllowed(&self) -> bool {
        unsafe { wxNotifyEvent_IsAllowed(self.handle()) }
    }
    #[fixed_stack_segment]
    fn veto(&self) {
        unsafe { wxNotifyEvent_Veto(self.handle()) }
    }
}

struct wxObject(*u8);
impl _wxObject for wxObject { fn handle(&self) -> *u8 { **self } }

impl wxObject {
    pub fn from(handle: *u8) -> @_wxObject {
        @wxObject(handle) as @_wxObject
    }
    
}

trait _wxObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @_wxClosure {
        unsafe { @wxClosure(wxObject_GetClientClosure(self.handle())) as @_wxClosure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @_wxClosure) {
        unsafe { wxObject_SetClientClosure(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxObject_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClassInfo(&self) -> @_wxClassInfo {
        unsafe { @wxClassInfo(wxObject_GetClassInfo(self.handle())) as @_wxClassInfo }
    }
    #[fixed_stack_segment]
    fn isKindOf(&self, classInfo: @_wxClassInfo) -> bool {
        unsafe { wxObject_IsKindOf(self.handle(), classInfo.handle()) }
    }
    #[fixed_stack_segment]
    fn isScrolledWindow(&self) -> bool {
        unsafe { wxObject_IsScrolledWindow(self.handle()) }
    }
}

struct wxObjectRefData(*u8);
impl _wxObjectRefData for wxObjectRefData { fn handle(&self) -> *u8 { **self } }

impl wxObjectRefData {
    pub fn from(handle: *u8) -> @_wxObjectRefData {
        @wxObjectRefData(handle) as @_wxObjectRefData
    }
    
}

trait _wxObjectRefData {
    fn handle(&self) -> *u8;
    
}

struct wxOutputStream(*u8);
impl _wxOutputStream for wxOutputStream {}
impl _wxStreamBase for wxOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxOutputStream {
    pub fn from(handle: *u8) -> @_wxOutputStream {
        @wxOutputStream(handle) as @_wxOutputStream
    }
    
}

trait _wxOutputStream : _wxStreamBase {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lastWrite(&self) -> c_int {
        unsafe { wxOutputStream_LastWrite(self.handle()) }
    }
    #[fixed_stack_segment]
    fn putC(&self, c: wchar_t) {
        unsafe { wxOutputStream_PutC(self.handle(), c) }
    }
    #[fixed_stack_segment]
    fn seek(&self, pos: c_int, mode: c_int) -> c_int {
        unsafe { wxOutputStream_Seek(self.handle(), pos, mode) }
    }
    #[fixed_stack_segment]
    fn sync(&self) {
        unsafe { wxOutputStream_Sync(self.handle()) }
    }
    #[fixed_stack_segment]
    fn tell(&self) -> c_int {
        unsafe { wxOutputStream_Tell(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxPageSetupDialog {
        @wxPageSetupDialog(handle) as @_wxPageSetupDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, data: @_wxPageSetupDialogData) -> @_wxPageSetupDialog {
        unsafe { @wxPageSetupDialog(wxPageSetupDialog_Create(parent.handle(), data.handle())) as @_wxPageSetupDialog }
    }
}

trait _wxPageSetupDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getPageSetupData(&self, _ref: @_wxPageSetupDialogData) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.handle(), _ref.handle()) }
    }
}

struct wxPageSetupDialogData(*u8);
impl _wxPageSetupDialogData for wxPageSetupDialogData {}
impl _wxObject for wxPageSetupDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPageSetupDialogData {
    pub fn from(handle: *u8) -> @_wxPageSetupDialogData {
        @wxPageSetupDialogData(handle) as @_wxPageSetupDialogData
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxPageSetupDialogData {
        unsafe { @wxPageSetupDialogData(wxPageSetupDialogData_Create()) as @_wxPageSetupDialogData }
    }
    #[fixed_stack_segment]
    pub fn newFromData(printData: @_wxPrintData) -> @_wxPageSetupDialogData {
        unsafe { @wxPageSetupDialogData(wxPageSetupDialogData_CreateFromData(printData.handle())) as @_wxPageSetupDialogData }
    }
}

trait _wxPageSetupDialogData : _wxObject {
    #[fixed_stack_segment]
    fn assign(&self, data: @_wxPageSetupDialogData) {
        unsafe { wxPageSetupDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, printData: @_wxPrintData) {
        unsafe { wxPageSetupDialogData_AssignData(self.handle(), printData.handle()) }
    }
    #[fixed_stack_segment]
    fn calculateIdFromPaperSize(&self) {
        unsafe { wxPageSetupDialogData_CalculateIdFromPaperSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn calculatePaperSizeFromId(&self) {
        unsafe { wxPageSetupDialogData_CalculatePaperSizeFromId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPageSetupDialogData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableHelp(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableHelp(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enableMargins(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableMargins(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enableOrientation(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableOrientation(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enablePaper(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnablePaper(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enablePrinter(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnablePrinter(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn getDefaultInfo(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetDefaultInfo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultMinMargins(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetDefaultMinMargins(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnableHelp(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnableMargins(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableMargins(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnableOrientation(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnablePaper(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnablePaper(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnablePrinter(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnablePrinter(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMarginBottomRight(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMarginBottomRight(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getMarginTopLeft(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMarginTopLeft(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getMinMarginBottomRight(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMinMarginBottomRight(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getMinMarginTopLeft(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMinMarginTopLeft(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxPageSetupDialogData_GetPaperSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @_wxPrintData) {
        unsafe { wxPageSetupDialogData_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultInfo(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_SetDefaultInfo(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setDefaultMinMargins(&self, flag: c_int) {
        unsafe { wxPageSetupDialogData_SetDefaultMinMargins(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginBottomRight(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginTopLeft(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setMinMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginBottomRight(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setMinMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginTopLeft(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setPaperId(&self, id: *u8) {
        unsafe { wxPageSetupDialogData_SetPaperId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn setPaperSizeId(&self, id: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSizeId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setPrintData(&self, printData: @_wxPrintData) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
}

struct wxPaintDC(*u8);
impl _wxPaintDC for wxPaintDC {}
impl _wxWindowDC for wxPaintDC {}
impl _wxDC for wxPaintDC {}
impl _wxObject for wxPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxPaintDC {
    pub fn from(handle: *u8) -> @_wxPaintDC {
        @wxPaintDC(handle) as @_wxPaintDC
    }
    
    #[fixed_stack_segment]
    pub fn new(win: @_wxWindow) -> @_wxPaintDC {
        unsafe { @wxPaintDC(wxPaintDC_Create(win.handle())) as @_wxPaintDC }
    }
}

trait _wxPaintDC : _wxWindowDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPaintDC_Delete(self.handle()) }
    }
}

struct wxPaintEvent(*u8);
impl _wxPaintEvent for wxPaintEvent {}
impl _wxEvent for wxPaintEvent {}
impl _wxObject for wxPaintEvent { fn handle(&self) -> *u8 { **self } }

impl wxPaintEvent {
    pub fn from(handle: *u8) -> @_wxPaintEvent {
        @wxPaintEvent(handle) as @_wxPaintEvent
    }
    
}

trait _wxPaintEvent : _wxEvent {
}

struct wxPalette(*u8);
impl _wxPalette for wxPalette {}
impl _wxGDIObject for wxPalette {}
impl _wxObject for wxPalette { fn handle(&self) -> *u8 { **self } }

impl wxPalette {
    pub fn from(handle: *u8) -> @_wxPalette {
        @wxPalette(handle) as @_wxPalette
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxPalette {
        unsafe { @wxPalette(wxPalette_CreateDefault()) as @_wxPalette }
    }
    #[fixed_stack_segment]
    pub fn newRGB(n: c_int, red: *u8, green: *u8, blue: *u8) -> @_wxPalette {
        unsafe { @wxPalette(wxPalette_CreateRGB(n, red, green, blue)) as @_wxPalette }
    }
}

trait _wxPalette : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, palette: @_wxPalette) {
        unsafe { wxPalette_Assign(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPalette_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPixel(&self, red: uint8_t, green: uint8_t, blue: uint8_t) -> c_int {
        unsafe { wxPalette_GetPixel(self.handle(), red, green, blue) }
    }
    #[fixed_stack_segment]
    fn getRGB(&self, pixel: c_int, red: *u8, green: *u8, blue: *u8) -> bool {
        unsafe { wxPalette_GetRGB(self.handle(), pixel, red, green, blue) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, palette: @_wxPalette) -> bool {
        unsafe { wxPalette_IsEqual(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPalette_IsOk(self.handle()) }
    }
}

struct wxPaletteChangedEvent(*u8);
impl _wxPaletteChangedEvent for wxPaletteChangedEvent {}
impl _wxEvent for wxPaletteChangedEvent {}
impl _wxObject for wxPaletteChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxPaletteChangedEvent {
    pub fn from(handle: *u8) -> @_wxPaletteChangedEvent {
        @wxPaletteChangedEvent(handle) as @_wxPaletteChangedEvent
    }
    
}

trait _wxPaletteChangedEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxPaletteChangedEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getChangedWindow(&self) -> *u8 {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setChangedWindow(&self, win: @_wxWindow) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.handle(), win.handle()) }
    }
}

struct wxPanel(*u8);
impl _wxPanel for wxPanel {}
impl _wxWindow for wxPanel {}
impl _wxEvtHandler for wxPanel {}
impl _wxObject for wxPanel { fn handle(&self) -> *u8 { **self } }

impl wxPanel {
    pub fn from(handle: *u8) -> @_wxPanel {
        @wxPanel(handle) as @_wxPanel
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxPanel {
        unsafe { @wxPanel(wxPanel_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxPanel }
    }
}

trait _wxPanel : _wxWindow {
    #[fixed_stack_segment]
    fn initDialog(&self) {
        unsafe { wxPanel_InitDialog(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setFocus(&self) {
        unsafe { wxPanel_SetFocus(self.handle()) }
    }
}

struct wxPathList(*u8);
impl _wxPathList for wxPathList {}
impl _wxList for wxPathList {}
impl _wxObject for wxPathList { fn handle(&self) -> *u8 { **self } }

impl wxPathList {
    pub fn from(handle: *u8) -> @_wxPathList {
        @wxPathList(handle) as @_wxPathList
    }
    
}

trait _wxPathList : _wxList {
}

struct wxPen(*u8);
impl _wxPen for wxPen {}
impl _wxGDIObject for wxPen {}
impl _wxObject for wxPen { fn handle(&self) -> *u8 { **self } }

impl wxPen {
    pub fn from(handle: *u8) -> @_wxPen {
        @wxPen(handle) as @_wxPen
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxPen {
        unsafe { @wxPen(wxPen_CreateDefault()) as @_wxPen }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(stipple: @_wxBitmap, width: c_int) -> @_wxPen {
        unsafe { @wxPen(wxPen_CreateFromBitmap(stipple.handle(), width)) as @_wxPen }
    }
    #[fixed_stack_segment]
    pub fn newFromColour(col: @_wxColour, width: c_int, style: c_int) -> @_wxPen {
        unsafe { @wxPen(wxPen_CreateFromColour(col.handle(), width, style)) as @_wxPen }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @_wxPen {
        unsafe { @wxPen(wxPen_CreateFromStock(id)) as @_wxPen }
    }
}

trait _wxPen : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, pen: @_wxPen) {
        unsafe { wxPen_Assign(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPen_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCap(&self) -> c_int {
        unsafe { wxPen_GetCap(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @_wxColour) {
        unsafe { wxPen_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDashes(&self, ptr: *u8) -> c_int {
        unsafe { wxPen_GetDashes(self.handle(), ptr) }
    }
    #[fixed_stack_segment]
    fn getJoin(&self) -> c_int {
        unsafe { wxPen_GetJoin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStipple(&self, _ref: @_wxBitmap) {
        unsafe { wxPen_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxPen_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxPen_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, pen: @_wxPen) -> bool {
        unsafe { wxPen_IsEqual(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPen_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCap(&self, cap: c_int) {
        unsafe { wxPen_SetCap(self.handle(), cap) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, col: @_wxColour) {
        unsafe { wxPen_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxPen_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    fn setDashes(&self, nb_dashes: c_int, dash: *u8) {
        unsafe { wxPen_SetDashes(self.handle(), nb_dashes, dash) }
    }
    #[fixed_stack_segment]
    fn setJoin(&self, join: c_int) {
        unsafe { wxPen_SetJoin(self.handle(), join) }
    }
    #[fixed_stack_segment]
    fn setStipple(&self, stipple: @_wxBitmap) {
        unsafe { wxPen_SetStipple(self.handle(), stipple.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxPen_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: c_int) {
        unsafe { wxPen_SetWidth(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxPen_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxPen_IsStatic(self.handle()) }
    }
}

struct wxPenList(*u8);
impl _wxPenList for wxPenList {}
impl _wxList for wxPenList {}
impl _wxObject for wxPenList { fn handle(&self) -> *u8 { **self } }

impl wxPenList {
    pub fn from(handle: *u8) -> @_wxPenList {
        @wxPenList(handle) as @_wxPenList
    }
    
}

trait _wxPenList : _wxList {
}

struct wxPlotCurve(*u8);
impl _wxPlotCurve for wxPlotCurve {}
impl _wxObject for wxPlotCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotCurve {
    pub fn from(handle: *u8) -> @_wxPlotCurve {
        @wxPlotCurve(handle) as @_wxPlotCurve
    }
    
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
    pub fn from(handle: *u8) -> @_wxPlotEvent {
        @wxPlotEvent(handle) as @_wxPlotEvent
    }
    
}

trait _wxPlotEvent : _wxNotifyEvent {
}

struct wxPlotOnOffCurve(*u8);
impl _wxPlotOnOffCurve for wxPlotOnOffCurve {}
impl _wxObject for wxPlotOnOffCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotOnOffCurve {
    pub fn from(handle: *u8) -> @_wxPlotOnOffCurve {
        @wxPlotOnOffCurve(handle) as @_wxPlotOnOffCurve
    }
    
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
    pub fn from(handle: *u8) -> @_wxPlotWindow {
        @wxPlotWindow(handle) as @_wxPlotWindow
    }
    
}

trait _wxPlotWindow : _wxScrolledWindow {
}

struct wxPoint(*u8);
impl _wxPoint for wxPoint { fn handle(&self) -> *u8 { **self } }

impl wxPoint {
    pub fn from(handle: *u8) -> @_wxPoint {
        @wxPoint(handle) as @_wxPoint
    }
    
    #[fixed_stack_segment]
    pub fn new(xx: c_int, yy: c_int) -> @_wxPoint {
        unsafe { @wxPoint(wxPoint_Create(xx, yy)) as @_wxPoint }
    }
}

trait _wxPoint {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getX(&self) -> c_int {
        unsafe { wxPoint_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> c_int {
        unsafe { wxPoint_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setX(&self, w: c_int) {
        unsafe { wxPoint_SetX(self.handle(), w) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxPopupTransientWindow {
        @wxPopupTransientWindow(handle) as @_wxPopupTransientWindow
    }
    
}

trait _wxPopupTransientWindow : _wxPopupWindow {
}

struct wxPopupWindow(*u8);
impl _wxPopupWindow for wxPopupWindow {}
impl _wxWindow for wxPopupWindow {}
impl _wxEvtHandler for wxPopupWindow {}
impl _wxObject for wxPopupWindow { fn handle(&self) -> *u8 { **self } }

impl wxPopupWindow {
    pub fn from(handle: *u8) -> @_wxPopupWindow {
        @wxPopupWindow(handle) as @_wxPopupWindow
    }
    
}

trait _wxPopupWindow : _wxWindow {
}

struct wxPostScriptDC(*u8);
impl _wxPostScriptDC for wxPostScriptDC {}
impl _wxDC for wxPostScriptDC {}
impl _wxObject for wxPostScriptDC { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptDC {
    pub fn from(handle: *u8) -> @_wxPostScriptDC {
        @wxPostScriptDC(handle) as @_wxPostScriptDC
    }
    
    #[fixed_stack_segment]
    pub fn new(data: @_wxPrintData) -> @_wxPostScriptDC {
        unsafe { @wxPostScriptDC(wxPostScriptDC_Create(data.handle())) as @_wxPostScriptDC }
    }
}

trait _wxPostScriptDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPostScriptDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setResolution(&self, ppi: c_int) {
        unsafe { wxPostScriptDC_SetResolution(self.handle(), ppi) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxPreviewCanvas {
        @wxPreviewCanvas(handle) as @_wxPreviewCanvas
    }
    
    #[fixed_stack_segment]
    pub fn new(preview: @_wxPrintPreview, parent: @_wxWindow, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @_wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPreviewCanvas_Create(preview.handle(), parent.handle(), x, y, w, h, style)) as @_wxPreviewCanvas }
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
    pub fn from(handle: *u8) -> @_wxPreviewControlBar {
        @wxPreviewControlBar(handle) as @_wxPreviewControlBar
    }
    
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
    pub fn from(handle: *u8) -> @_wxPreviewFrame {
        @wxPreviewFrame(handle) as @_wxPreviewFrame
    }
    
    #[fixed_stack_segment]
    pub fn new(preview: @_wxPrintPreview, parent: @_wxFrame, title: @_wxString, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: @_wxString) -> @_wxPreviewFrame {
        unsafe { @wxPreviewFrame(wxPreviewFrame_Create(preview.handle(), parent.handle(), title.handle(), x, y, width, height, style, name.handle())) as @_wxPreviewFrame }
    }
}

trait _wxPreviewFrame : _wxFrame {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPreviewFrame_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.handle()) }
    }
}

struct wxPrintData(*u8);
impl _wxPrintData for wxPrintData {}
impl _wxObject for wxPrintData { fn handle(&self) -> *u8 { **self } }

impl wxPrintData {
    pub fn from(handle: *u8) -> @_wxPrintData {
        @wxPrintData(handle) as @_wxPrintData
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxPrintData {
        unsafe { @wxPrintData(wxPrintData_Create()) as @_wxPrintData }
    }
}

trait _wxPrintData : _wxObject {
    #[fixed_stack_segment]
    fn assign(&self, data: @_wxPrintData) {
        unsafe { wxPrintData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCollate(&self) -> bool {
        unsafe { wxPrintData_GetCollate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self) -> bool {
        unsafe { wxPrintData_GetColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDuplex(&self) -> c_int {
        unsafe { wxPrintData_GetDuplex(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> @_wxString {
        unsafe { @wxString(wxPrintData_GetFilename(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFontMetricPath(&self) -> @_wxString {
        unsafe { @wxString(wxPrintData_GetFontMetricPath(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintData_GetNoCopies(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxPrintData_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPrintData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxPrintData_GetPaperSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getPreviewCommand(&self) -> @_wxString {
        unsafe { @wxString(wxPrintData_GetPreviewCommand(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterCommand(&self) -> @_wxString {
        unsafe { @wxString(wxPrintData_GetPrinterCommand(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPrinterName(&self) -> @_wxString {
        unsafe { @wxString(wxPrintData_GetPrinterName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPrinterOptions(&self) -> @_wxString {
        unsafe { @wxString(wxPrintData_GetPrinterOptions(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getPrinterScaleX(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterScaleY(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterTranslateX(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterTranslateY(&self) -> c_int {
        unsafe { wxPrintData_GetPrinterTranslateY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getQuality(&self) -> c_int {
        unsafe { wxPrintData_GetQuality(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintData_SetCollate(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: c_int) {
        unsafe { wxPrintData_SetColour(self.handle(), colour) }
    }
    #[fixed_stack_segment]
    fn setDuplex(&self, duplex: c_int) {
        unsafe { wxPrintData_SetDuplex(self.handle(), duplex) }
    }
    #[fixed_stack_segment]
    fn setFilename(&self, filename: @_wxString) {
        unsafe { wxPrintData_SetFilename(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn setFontMetricPath(&self, path: @_wxString) {
        unsafe { wxPrintData_SetFontMetricPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintData_SetNoCopies(self.handle(), v) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxPrintData_SetOrientation(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn setPaperId(&self, sizeId: c_int) {
        unsafe { wxPrintData_SetPaperId(self.handle(), sizeId) }
    }
    #[fixed_stack_segment]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPrintData_SetPaperSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn setPreviewCommand(&self, command: @_wxCommand) {
        unsafe { wxPrintData_SetPreviewCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.handle(), printMode) }
    }
    #[fixed_stack_segment]
    fn setPrinterCommand(&self, command: @_wxCommand) {
        unsafe { wxPrintData_SetPrinterCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterName(&self, name: @_wxString) {
        unsafe { wxPrintData_SetPrinterName(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterOptions(&self, options: @_wxString) {
        unsafe { wxPrintData_SetPrinterOptions(self.handle(), options.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterScaleX(&self, x: c_double) {
        unsafe { wxPrintData_SetPrinterScaleX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn setPrinterScaleY(&self, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaleY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn setPrinterScaling(&self, x: c_double, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaling(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setPrinterTranslateX(&self, x: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateX(self.handle(), x) }
    }
    #[fixed_stack_segment]
    fn setPrinterTranslateY(&self, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslateY(self.handle(), y) }
    }
    #[fixed_stack_segment]
    fn setPrinterTranslation(&self, x: c_int, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslation(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setQuality(&self, quality: c_int) {
        unsafe { wxPrintData_SetQuality(self.handle(), quality) }
    }
}

struct wxPostScriptPrintNativeData(*u8);
impl _wxPostScriptPrintNativeData for wxPostScriptPrintNativeData {}
impl _wxObject for wxPostScriptPrintNativeData { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptPrintNativeData {
    pub fn from(handle: *u8) -> @_wxPostScriptPrintNativeData {
        @wxPostScriptPrintNativeData(handle) as @_wxPostScriptPrintNativeData
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxPostScriptPrintNativeData {
        unsafe { @wxPostScriptPrintNativeData(wxPostScriptPrintNativeData_Create()) as @_wxPostScriptPrintNativeData }
    }
}

trait _wxPostScriptPrintNativeData : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPostScriptPrintNativeData_Delete(self.handle()) }
    }
}

struct wxPrintDialog(*u8);
impl _wxPrintDialog for wxPrintDialog {}
impl _wxDialog for wxPrintDialog {}
impl _wxTopLevelWindow for wxPrintDialog {}
impl _wxWindow for wxPrintDialog {}
impl _wxEvtHandler for wxPrintDialog {}
impl _wxObject for wxPrintDialog { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialog {
    pub fn from(handle: *u8) -> @_wxPrintDialog {
        @wxPrintDialog(handle) as @_wxPrintDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, data: @_wxPrintDialogData) -> @_wxPrintDialog {
        unsafe { @wxPrintDialog(wxPrintDialog_Create(parent.handle(), data.handle())) as @_wxPrintDialog }
    }
}

trait _wxPrintDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getPrintDC(&self) -> @_wxDC {
        unsafe { @wxDC(wxPrintDialog_GetPrintDC(self.handle())) as @_wxDC }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @_wxPrintData) {
        unsafe { wxPrintDialog_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self) -> @_wxPrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialog_GetPrintDialogData(self.handle())) as @_wxPrintDialogData }
    }
}

struct wxPrintDialogData(*u8);
impl _wxPrintDialogData for wxPrintDialogData {}
impl _wxObject for wxPrintDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialogData {
    pub fn from(handle: *u8) -> @_wxPrintDialogData {
        @wxPrintDialogData(handle) as @_wxPrintDialogData
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxPrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialogData_CreateDefault()) as @_wxPrintDialogData }
    }
    #[fixed_stack_segment]
    pub fn newFromData(printData: @_wxPrintData) -> @_wxPrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialogData_CreateFromData(printData.handle())) as @_wxPrintDialogData }
    }
}

trait _wxPrintDialogData : _wxObject {
    #[fixed_stack_segment]
    fn assign(&self, data: @_wxPrintDialogData) {
        unsafe { wxPrintDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, data: @_wxPrintData) {
        unsafe { wxPrintDialogData_AssignData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintDialogData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableHelp(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnableHelp(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enablePageNumbers(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnablePageNumbers(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enablePrintToFile(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnablePrintToFile(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn enableSelection(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnableSelection(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn getAllPages(&self) -> c_int {
        unsafe { wxPrintDialogData_GetAllPages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCollate(&self) -> bool {
        unsafe { wxPrintDialogData_GetCollate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnableHelp(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnableHelp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnablePageNumbers(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnablePageNumbers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnablePrintToFile(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnablePrintToFile(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnableSelection(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnableSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFromPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetFromPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMaxPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetMinPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getNoCopies(&self) -> c_int {
        unsafe { wxPrintDialogData_GetNoCopies(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @_wxPrintData) {
        unsafe { wxPrintDialogData_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintToFile(&self) -> bool {
        unsafe { wxPrintDialogData_GetPrintToFile(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> bool {
        unsafe { wxPrintDialogData_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getToPage(&self) -> c_int {
        unsafe { wxPrintDialogData_GetToPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAllPages(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetAllPages(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setCollate(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetCollate(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setFromPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetFromPage(self.handle(), v) }
    }
    #[fixed_stack_segment]
    fn setMaxPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMaxPage(self.handle(), v) }
    }
    #[fixed_stack_segment]
    fn setMinPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetMinPage(self.handle(), v) }
    }
    #[fixed_stack_segment]
    fn setNoCopies(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetNoCopies(self.handle(), v) }
    }
    #[fixed_stack_segment]
    fn setPrintData(&self, printData: @_wxPrintData) {
        unsafe { wxPrintDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintToFile(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetPrintToFile(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetSelection(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setToPage(&self, v: c_int) {
        unsafe { wxPrintDialogData_SetToPage(self.handle(), v) }
    }
}

struct wxPrintPreview(*u8);
impl _wxPrintPreview for wxPrintPreview {}
impl _wxObject for wxPrintPreview { fn handle(&self) -> *u8 { **self } }

impl wxPrintPreview {
    pub fn from(handle: *u8) -> @_wxPrintPreview {
        @wxPrintPreview(handle) as @_wxPrintPreview
    }
    
    #[fixed_stack_segment]
    pub fn newFromData(printout: @_wxPrintout, printoutForPrinting: @_wxPrintout, data: @_wxPrintData) -> @_wxPrintPreview {
        unsafe { @wxPrintPreview(wxPrintPreview_CreateFromData(printout.handle(), printoutForPrinting.handle(), data.handle())) as @_wxPrintPreview }
    }
    #[fixed_stack_segment]
    pub fn newFromDialogData(printout: @_wxPrintout, printoutForPrinting: @_wxPrintout, data: @_wxPrintDialogData) -> @_wxPrintPreview {
        unsafe { @wxPrintPreview(wxPrintPreview_CreateFromDialogData(printout.handle(), printoutForPrinting.handle(), data.handle())) as @_wxPrintPreview }
    }
}

trait _wxPrintPreview : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintPreview_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawBlankPage(&self, canvas: @_wxPreviewCanvas, dc: @_wxDC) -> bool {
        unsafe { wxPrintPreview_DrawBlankPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn getCanvas(&self) -> @_wxPreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPrintPreview_GetCanvas(self.handle())) as @_wxPreviewCanvas }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @_wxFrame {
        unsafe { @wxFrame(wxPrintPreview_GetFrame(self.handle())) as @_wxFrame }
    }
    #[fixed_stack_segment]
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMaxPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMinPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self, _ref: @_wxPrintDialogData) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @_wxPrintout {
        unsafe { @wxPrintout(wxPrintPreview_GetPrintout(self.handle())) as @_wxPrintout }
    }
    #[fixed_stack_segment]
    fn getPrintoutForPrinting(&self) -> @_wxPrintout {
        unsafe { @wxPrintout(wxPrintPreview_GetPrintoutForPrinting(self.handle())) as @_wxPrintout }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> c_int {
        unsafe { wxPrintPreview_GetZoom(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPrintPreview_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paintPage(&self, canvas: @_wxPrintPreview, dc: @_wxDC) -> bool {
        unsafe { wxPrintPreview_PaintPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn print(&self, interactive: bool) -> bool {
        unsafe { wxPrintPreview_Print(self.handle(), interactive) }
    }
    #[fixed_stack_segment]
    fn renderPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_RenderPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    fn setCanvas(&self, canvas: @_wxPreviewCanvas) {
        unsafe { wxPrintPreview_SetCanvas(self.handle(), canvas.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrentPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_SetCurrentPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    fn setFrame(&self, frame: @_wxFrame) {
        unsafe { wxPrintPreview_SetFrame(self.handle(), frame.handle()) }
    }
    #[fixed_stack_segment]
    fn setOk(&self, ok: bool) {
        unsafe { wxPrintPreview_SetOk(self.handle(), ok) }
    }
    #[fixed_stack_segment]
    fn setPrintout(&self, printout: @_wxPrintout) {
        unsafe { wxPrintPreview_SetPrintout(self.handle(), printout.handle()) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.handle(), percent) }
    }
}

struct wxPrinter(*u8);
impl _wxPrinter for wxPrinter {}
impl _wxObject for wxPrinter { fn handle(&self) -> *u8 { **self } }

impl wxPrinter {
    pub fn from(handle: *u8) -> @_wxPrinter {
        @wxPrinter(handle) as @_wxPrinter
    }
    
    #[fixed_stack_segment]
    pub fn new(data: @_wxPrintDialogData) -> @_wxPrinter {
        unsafe { @wxPrinter(wxPrinter_Create(data.handle())) as @_wxPrinter }
    }
}

trait _wxPrinter : _wxObject {
    #[fixed_stack_segment]
    fn newAbortWindow(&self, parent: @_wxWindow, printout: @_wxPrintout) -> @_wxWindow {
        unsafe { @wxWindow(wxPrinter_CreateAbortWindow(self.handle(), parent.handle(), printout.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinter_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAbort(&self) -> bool {
        unsafe { wxPrinter_GetAbort(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastError(&self) -> c_int {
        unsafe { wxPrinter_GetLastError(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self, _ref: @_wxPrintDialogData) {
        unsafe { wxPrinter_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn print(&self, parent: @_wxWindow, printout: @_wxPrintout, prompt: bool) -> bool {
        unsafe { wxPrinter_Print(self.handle(), parent.handle(), printout.handle(), prompt) }
    }
    #[fixed_stack_segment]
    fn printDialog(&self, parent: @_wxWindow) -> @_wxDC {
        unsafe { @wxDC(wxPrinter_PrintDialog(self.handle(), parent.handle())) as @_wxDC }
    }
    #[fixed_stack_segment]
    fn reportError(&self, parent: @_wxWindow, printout: @_wxPrintout, message: @_wxString) {
        unsafe { wxPrinter_ReportError(self.handle(), parent.handle(), printout.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setup(&self, parent: @_wxWindow) -> bool {
        unsafe { wxPrinter_Setup(self.handle(), parent.handle()) }
    }
}

struct wxPrinterDC(*u8);
impl _wxPrinterDC for wxPrinterDC {}
impl _wxDC for wxPrinterDC {}
impl _wxObject for wxPrinterDC { fn handle(&self) -> *u8 { **self } }

impl wxPrinterDC {
    pub fn from(handle: *u8) -> @_wxPrinterDC {
        @wxPrinterDC(handle) as @_wxPrinterDC
    }
    
    #[fixed_stack_segment]
    pub fn new(data: @_wxPrintData) -> @_wxPrinterDC {
        unsafe { @wxPrinterDC(wxPrinterDC_Create(data.handle())) as @_wxPrinterDC }
    }
}

trait _wxPrinterDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinterDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperRect(&self) -> @_wxRect {
        unsafe { @wxRect(wxPrinterDC_GetPaperRect(self.handle())) as @_wxRect }
    }
}

struct wxPrintout(*u8);
impl _wxPrintout for wxPrintout {}
impl _wxObject for wxPrintout { fn handle(&self) -> *u8 { **self } }

impl wxPrintout {
    pub fn from(handle: *u8) -> @_wxPrintout {
        @wxPrintout(handle) as @_wxPrintout
    }
    
}

trait _wxPrintout : _wxObject {
    #[fixed_stack_segment]
    fn getDC(&self) -> @_wxDC {
        unsafe { @wxDC(wxPrintout_GetDC(self.handle())) as @_wxDC }
    }
    #[fixed_stack_segment]
    fn getPPIPrinter(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxPrintout_GetPPIPrinter(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getPPIScreen(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxPrintout_GetPPIScreen(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getPageSizeMM(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxPrintout_GetPageSizeMM(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getPageSizePixels(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxPrintout_GetPageSizePixels(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @_wxString {
        unsafe { @wxString(wxPrintout_GetTitle(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn isPreview(&self) -> bool {
        unsafe { wxPrintout_IsPreview(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDC(&self, dc: @_wxDC) {
        unsafe { wxPrintout_SetDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn setPPIPrinter(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIPrinter(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setPPIScreen(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIScreen(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setPageSizeMM(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizeMM(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn setPageSizePixels(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizePixels(self.handle(), w, h) }
    }
}

struct wxPrivateDropTarget(*u8);
impl _wxPrivateDropTarget for wxPrivateDropTarget {}
impl _wxDropTarget for wxPrivateDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxPrivateDropTarget {
    pub fn from(handle: *u8) -> @_wxPrivateDropTarget {
        @wxPrivateDropTarget(handle) as @_wxPrivateDropTarget
    }
    
}

trait _wxPrivateDropTarget : _wxDropTarget {
}

struct wxProcess(*u8);
impl _wxProcess for wxProcess {}
impl _wxEvtHandler for wxProcess {}
impl _wxObject for wxProcess { fn handle(&self) -> *u8 { **self } }

impl wxProcess {
    pub fn from(handle: *u8) -> @_wxProcess {
        @wxProcess(handle) as @_wxProcess
    }
    
    #[fixed_stack_segment]
    pub fn newDefault(_prt: @_wxWindow, _id: c_int) -> @_wxProcess {
        unsafe { @wxProcess(wxProcess_CreateDefault(_prt.handle(), _id)) as @_wxProcess }
    }
    #[fixed_stack_segment]
    pub fn newRedirect(_prt: @_wxWindow, _rdr: bool) -> @_wxProcess {
        unsafe { @wxProcess(wxProcess_CreateRedirect(_prt.handle(), _rdr)) as @_wxProcess }
    }
    #[fixed_stack_segment]
    pub fn open(cmd: @_wxString, flags: c_int) -> @_wxProcess {
        unsafe { @wxProcess(wxProcess_Open(cmd.handle(), flags)) as @_wxProcess }
    }
}

trait _wxProcess : _wxEvtHandler {
    #[fixed_stack_segment]
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxProcess_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn detach(&self) {
        unsafe { wxProcess_Detach(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getErrorStream(&self) -> @_wxInputStream {
        unsafe { @wxInputStream(wxProcess_GetErrorStream(self.handle())) as @_wxInputStream }
    }
    #[fixed_stack_segment]
    fn getInputStream(&self) -> @_wxInputStream {
        unsafe { @wxInputStream(wxProcess_GetInputStream(self.handle())) as @_wxInputStream }
    }
    #[fixed_stack_segment]
    fn getOutputStream(&self) -> @_wxOutputStream {
        unsafe { @wxOutputStream(wxProcess_GetOutputStream(self.handle())) as @_wxOutputStream }
    }
    #[fixed_stack_segment]
    fn isRedirected(&self) -> bool {
        unsafe { wxProcess_IsRedirected(self.handle()) }
    }
    #[fixed_stack_segment]
    fn redirect(&self) {
        unsafe { wxProcess_Redirect(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isErrorAvailable(&self) -> bool {
        unsafe { wxProcess_IsErrorAvailable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isInputAvailable(&self) -> bool {
        unsafe { wxProcess_IsInputAvailable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isInputOpened(&self) -> bool {
        unsafe { wxProcess_IsInputOpened(self.handle()) }
    }
}

struct wxProcessEvent(*u8);
impl _wxProcessEvent for wxProcessEvent {}
impl _wxEvent for wxProcessEvent {}
impl _wxObject for wxProcessEvent { fn handle(&self) -> *u8 { **self } }

impl wxProcessEvent {
    pub fn from(handle: *u8) -> @_wxProcessEvent {
        @wxProcessEvent(handle) as @_wxProcessEvent
    }
    
}

trait _wxProcessEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxProgressDialog {
        @wxProgressDialog(handle) as @_wxProgressDialog
    }
    
    #[fixed_stack_segment]
    pub fn new(title: @_wxString, message: @_wxString, max: c_int, parent: @_wxWindow, style: c_int) -> @_wxProgressDialog {
        unsafe { @wxProgressDialog(wxProgressDialog_Create(title.handle(), message.handle(), max, parent.handle(), style)) as @_wxProgressDialog }
    }
}

trait _wxProgressDialog : _wxFrame {
    #[fixed_stack_segment]
    fn update(&self, value: c_int) -> bool {
        unsafe { wxProgressDialog_Update(self.handle(), value) }
    }
    #[fixed_stack_segment]
    fn updateWithMessage(&self, value: c_int, message: @_wxString) -> bool {
        unsafe { wxProgressDialog_UpdateWithMessage(self.handle(), value, message.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxProtocol {
        @wxProtocol(handle) as @_wxProtocol
    }
    
}

trait _wxProtocol : _wxSocketClient {
}

struct wxQuantize(*u8);
impl _wxQuantize for wxQuantize {}
impl _wxObject for wxQuantize { fn handle(&self) -> *u8 { **self } }

impl wxQuantize {
    pub fn from(handle: *u8) -> @_wxQuantize {
        @wxQuantize(handle) as @_wxQuantize
    }
    
}

trait _wxQuantize : _wxObject {
}

struct wxQueryCol(*u8);
impl _wxQueryCol for wxQueryCol {}
impl _wxObject for wxQueryCol { fn handle(&self) -> *u8 { **self } }

impl wxQueryCol {
    pub fn from(handle: *u8) -> @_wxQueryCol {
        @wxQueryCol(handle) as @_wxQueryCol
    }
    
}

trait _wxQueryCol : _wxObject {
}

struct wxQueryField(*u8);
impl _wxQueryField for wxQueryField {}
impl _wxObject for wxQueryField { fn handle(&self) -> *u8 { **self } }

impl wxQueryField {
    pub fn from(handle: *u8) -> @_wxQueryField {
        @wxQueryField(handle) as @_wxQueryField
    }
    
}

trait _wxQueryField : _wxObject {
}

struct wxQueryLayoutInfoEvent(*u8);
impl _wxQueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl _wxEvent for wxQueryLayoutInfoEvent {}
impl _wxObject for wxQueryLayoutInfoEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryLayoutInfoEvent {
    pub fn from(handle: *u8) -> @_wxQueryLayoutInfoEvent {
        @wxQueryLayoutInfoEvent(handle) as @_wxQueryLayoutInfoEvent
    }
    
    #[fixed_stack_segment]
    pub fn new(id: c_int) -> @_wxQueryLayoutInfoEvent {
        unsafe { @wxQueryLayoutInfoEvent(wxQueryLayoutInfoEvent_Create(id)) as @_wxQueryLayoutInfoEvent }
    }
}

trait _wxQueryLayoutInfoEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getAlignment(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRequestedLength(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetRequestedLength(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxQueryLayoutInfoEvent_GetSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, align: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetAlignment(self.handle(), align) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetOrientation(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn setRequestedLength(&self, length: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetRequestedLength(self.handle(), length) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, w: c_int, h: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetSize(self.handle(), w, h) }
    }
}

struct wxQueryNewPaletteEvent(*u8);
impl _wxQueryNewPaletteEvent for wxQueryNewPaletteEvent {}
impl _wxEvent for wxQueryNewPaletteEvent {}
impl _wxObject for wxQueryNewPaletteEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryNewPaletteEvent {
    pub fn from(handle: *u8) -> @_wxQueryNewPaletteEvent {
        @wxQueryNewPaletteEvent(handle) as @_wxQueryNewPaletteEvent
    }
    
}

trait _wxQueryNewPaletteEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @_wxObject) {
        unsafe { wxQueryNewPaletteEvent_CopyObject(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaletteRealized(&self) -> bool {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxRadioBox {
        @wxRadioBox(handle) as @_wxRadioBox
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: c_int, _stl: c_int) -> @_wxRadioBox {
        unsafe { @wxRadioBox(wxRadioBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) as @_wxRadioBox }
    }
}

trait _wxRadioBox : _wxControl {
    #[fixed_stack_segment]
    fn enableItem(&self, item: c_int, enable: bool) {
        unsafe { wxRadioBox_EnableItem(self.handle(), item, enable) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @_wxString) -> c_int {
        unsafe { wxRadioBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemLabel(&self, item: c_int) -> @_wxString {
        unsafe { @wxString(wxRadioBox_GetItemLabel(self.handle(), item)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNumberOfRowsOrCols(&self) -> c_int {
        unsafe { wxRadioBox_GetNumberOfRowsOrCols(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> c_int {
        unsafe { wxRadioBox_GetSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @_wxString {
        unsafe { @wxString(wxRadioBox_GetStringSelection(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBitmap(&self, item: c_int, bitmap: @_wxBitmap) {
        unsafe { wxRadioBox_SetItemBitmap(self.handle(), item, bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, item: c_int, label: @_wxString) {
        unsafe { wxRadioBox_SetItemLabel(self.handle(), item, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setNumberOfRowsOrCols(&self, n: c_int) {
        unsafe { wxRadioBox_SetNumberOfRowsOrCols(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, _n: c_int) {
        unsafe { wxRadioBox_SetSelection(self.handle(), _n) }
    }
    #[fixed_stack_segment]
    fn setStringSelection(&self, s: @_wxString) {
        unsafe { wxRadioBox_SetStringSelection(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxRadioButton {
        @wxRadioButton(handle) as @_wxRadioButton
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxRadioButton {
        unsafe { @wxRadioButton(wxRadioButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxRadioButton }
    }
}

trait _wxRadioButton : _wxControl {
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxRadioButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: bool) {
        unsafe { wxRadioButton_SetValue(self.handle(), value) }
    }
}

struct wxRealPoint(*u8);
impl _wxRealPoint for wxRealPoint { fn handle(&self) -> *u8 { **self } }

impl wxRealPoint {
    pub fn from(handle: *u8) -> @_wxRealPoint {
        @wxRealPoint(handle) as @_wxRealPoint
    }
    
}

trait _wxRealPoint {
    fn handle(&self) -> *u8;
    
}

struct wxRecordSet(*u8);
impl _wxRecordSet for wxRecordSet {}
impl _wxObject for wxRecordSet { fn handle(&self) -> *u8 { **self } }

impl wxRecordSet {
    pub fn from(handle: *u8) -> @_wxRecordSet {
        @wxRecordSet(handle) as @_wxRecordSet
    }
    
}

trait _wxRecordSet : _wxObject {
}

struct wxRect(*u8);
impl _wxRect for wxRect { fn handle(&self) -> *u8 { **self } }

impl wxRect {
    pub fn from(handle: *u8) -> @_wxRect {
        @wxRect(handle) as @_wxRect
    }
    
}

trait _wxRect {
    fn handle(&self) -> *u8;
    
}

struct wxRegEx(*u8);
impl _wxRegEx for wxRegEx { fn handle(&self) -> *u8 { **self } }

impl wxRegEx {
    pub fn from(handle: *u8) -> @_wxRegEx {
        @wxRegEx(handle) as @_wxRegEx
    }
    
}

trait _wxRegEx {
    fn handle(&self) -> *u8;
    
}

struct wxRegion(*u8);
impl _wxRegion for wxRegion {}
impl _wxGDIObject for wxRegion {}
impl _wxObject for wxRegion { fn handle(&self) -> *u8 { **self } }

impl wxRegion {
    pub fn from(handle: *u8) -> @_wxRegion {
        @wxRegion(handle) as @_wxRegion
    }
    
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxRegion {
        unsafe { @wxRegion(wxRegion_CreateDefault()) as @_wxRegion }
    }
    #[fixed_stack_segment]
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> @_wxRegion {
        unsafe { @wxRegion(wxRegion_CreateFromRect(x, y, w, h)) as @_wxRegion }
    }
}

trait _wxRegion : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, region: @_wxRegion) {
        unsafe { wxRegion_Assign(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxRegion_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn containsPoint(&self, x: c_int, y: c_int) -> bool {
        unsafe { wxRegion_ContainsPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_ContainsRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxRegion_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEmpty(&self) -> bool {
        unsafe { wxRegion_IsEmpty(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRegion_GetBox(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_IntersectRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn intersectRegion(&self, region: @_wxRegion) -> bool {
        unsafe { wxRegion_IntersectRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_SubtractRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn subtractRegion(&self, region: @_wxRegion) -> bool {
        unsafe { wxRegion_SubtractRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_UnionRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn unionRegion(&self, region: @_wxRegion) -> bool {
        unsafe { wxRegion_UnionRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_XorRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn xorRegion(&self, region: @_wxRegion) -> bool {
        unsafe { wxRegion_XorRegion(self.handle(), region.handle()) }
    }
}

struct wxRegionIterator(*u8);
impl _wxRegionIterator for wxRegionIterator {}
impl _wxObject for wxRegionIterator { fn handle(&self) -> *u8 { **self } }

impl wxRegionIterator {
    pub fn from(handle: *u8) -> @_wxRegionIterator {
        @wxRegionIterator(handle) as @_wxRegionIterator
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxRegionIterator {
        unsafe { @wxRegionIterator(wxRegionIterator_Create()) as @_wxRegionIterator }
    }
    #[fixed_stack_segment]
    pub fn newFromRegion(region: @_wxRegion) -> @_wxRegionIterator {
        unsafe { @wxRegionIterator(wxRegionIterator_CreateFromRegion(region.handle())) as @_wxRegionIterator }
    }
}

trait _wxRegionIterator : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxRegionIterator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> c_int {
        unsafe { wxRegionIterator_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxRegionIterator_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> c_int {
        unsafe { wxRegionIterator_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> c_int {
        unsafe { wxRegionIterator_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn haveRects(&self) -> bool {
        unsafe { wxRegionIterator_HaveRects(self.handle()) }
    }
    #[fixed_stack_segment]
    fn next(&self) {
        unsafe { wxRegionIterator_Next(self.handle()) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxRegionIterator_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn resetToRegion(&self, region: @_wxRegion) {
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
    pub fn from(handle: *u8) -> @_wxRemotelyScrolledTreeCtrl {
        @wxRemotelyScrolledTreeCtrl(handle) as @_wxRemotelyScrolledTreeCtrl
    }
    
}

trait _wxRemotelyScrolledTreeCtrl : _wxTreeCtrl {
}

struct wxSVGFileDC(*u8);
impl _wxSVGFileDC for wxSVGFileDC {}
impl _wxDC for wxSVGFileDC {}
impl _wxObject for wxSVGFileDC { fn handle(&self) -> *u8 { **self } }

impl wxSVGFileDC {
    pub fn from(handle: *u8) -> @_wxSVGFileDC {
        @wxSVGFileDC(handle) as @_wxSVGFileDC
    }
    
    #[fixed_stack_segment]
    pub fn new(fileName: @_wxString) -> @_wxSVGFileDC {
        unsafe { @wxSVGFileDC(wxSVGFileDC_Create(fileName.handle())) as @_wxSVGFileDC }
    }
    #[fixed_stack_segment]
    pub fn newWithSize(fileName: @_wxString, w: c_int, h: c_int) -> @_wxSVGFileDC {
        unsafe { @wxSVGFileDC(wxSVGFileDC_CreateWithSize(fileName.handle(), w, h)) as @_wxSVGFileDC }
    }
    #[fixed_stack_segment]
    pub fn newWithSizeAndResolution(fileName: @_wxString, w: c_int, h: c_int, a_dpi: c_float) -> @_wxSVGFileDC {
        unsafe { @wxSVGFileDC(wxSVGFileDC_CreateWithSizeAndResolution(fileName.handle(), w, h, a_dpi)) as @_wxSVGFileDC }
    }
}

trait _wxSVGFileDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSVGFileDC_Delete(self.handle()) }
    }
}

struct wxSashEvent(*u8);
impl _wxSashEvent for wxSashEvent {}
impl _wxEvent for wxSashEvent {}
impl _wxObject for wxSashEvent { fn handle(&self) -> *u8 { **self } }

impl wxSashEvent {
    pub fn from(handle: *u8) -> @_wxSashEvent {
        @wxSashEvent(handle) as @_wxSashEvent
    }
    
    #[fixed_stack_segment]
    pub fn new(id: c_int, edge: c_int) -> @_wxSashEvent {
        unsafe { @wxSashEvent(wxSashEvent_Create(id, edge)) as @_wxSashEvent }
    }
}

trait _wxSashEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getDragRect(&self) -> @_wxRect {
        unsafe { @wxRect(wxSashEvent_GetDragRect(self.handle())) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn getDragStatus(&self) -> c_int {
        unsafe { wxSashEvent_GetDragStatus(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEdge(&self) -> c_int {
        unsafe { wxSashEvent_GetEdge(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDragRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxSashEvent_SetDragRect(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn setDragStatus(&self, status: c_int) {
        unsafe { wxSashEvent_SetDragStatus(self.handle(), status) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxSashLayoutWindow {
        @wxSashLayoutWindow(handle) as @_wxSashLayoutWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_par: @_wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @_wxSashLayoutWindow {
        unsafe { @wxSashLayoutWindow(wxSashLayoutWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) as @_wxSashLayoutWindow }
    }
}

trait _wxSashLayoutWindow : _wxSashWindow {
    #[fixed_stack_segment]
    fn getAlignment(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, align: c_int) {
        unsafe { wxSashLayoutWindow_SetAlignment(self.handle(), align) }
    }
    #[fixed_stack_segment]
    fn setDefaultSize(&self, w: c_int, h: c_int) {
        unsafe { wxSashLayoutWindow_SetDefaultSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxSashWindow {
        @wxSashWindow(handle) as @_wxSashWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_par: @_wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @_wxSashWindow {
        unsafe { @wxSashWindow(wxSashWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) as @_wxSashWindow }
    }
}

trait _wxSashWindow : _wxWindow {
    #[fixed_stack_segment]
    fn getDefaultBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetDefaultBorderSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEdgeMargin(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_GetEdgeMargin(self.handle(), edge) }
    }
    #[fixed_stack_segment]
    fn getExtraBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetExtraBorderSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaximumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaximumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinimumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinimumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSashVisible(&self, edge: c_int) -> bool {
        unsafe { wxSashWindow_GetSashVisible(self.handle(), edge) }
    }
    #[fixed_stack_segment]
    fn hasBorder(&self, edge: c_int) -> bool {
        unsafe { wxSashWindow_HasBorder(self.handle(), edge) }
    }
    #[fixed_stack_segment]
    fn setDefaultBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetDefaultBorderSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn setExtraBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetExtraBorderSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn setMaximumSizeX(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeX(self.handle(), max) }
    }
    #[fixed_stack_segment]
    fn setMaximumSizeY(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeY(self.handle(), max) }
    }
    #[fixed_stack_segment]
    fn setMinimumSizeX(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeX(self.handle(), min) }
    }
    #[fixed_stack_segment]
    fn setMinimumSizeY(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeY(self.handle(), min) }
    }
    #[fixed_stack_segment]
    fn setSashBorder(&self, edge: c_int, border: bool) {
        unsafe { wxSashWindow_SetSashBorder(self.handle(), edge, border) }
    }
    #[fixed_stack_segment]
    fn setSashVisible(&self, edge: c_int, sash: bool) {
        unsafe { wxSashWindow_SetSashVisible(self.handle(), edge, sash) }
    }
}

struct wxScopedArray(*u8);
impl _wxScopedArray for wxScopedArray { fn handle(&self) -> *u8 { **self } }

impl wxScopedArray {
    pub fn from(handle: *u8) -> @_wxScopedArray {
        @wxScopedArray(handle) as @_wxScopedArray
    }
    
}

trait _wxScopedArray {
    fn handle(&self) -> *u8;
    
}

struct wxScopedPtr(*u8);
impl _wxScopedPtr for wxScopedPtr { fn handle(&self) -> *u8 { **self } }

impl wxScopedPtr {
    pub fn from(handle: *u8) -> @_wxScopedPtr {
        @wxScopedPtr(handle) as @_wxScopedPtr
    }
    
}

trait _wxScopedPtr {
    fn handle(&self) -> *u8;
    
}

struct wxScreenDC(*u8);
impl _wxScreenDC for wxScreenDC {}
impl _wxDC for wxScreenDC {}
impl _wxObject for wxScreenDC { fn handle(&self) -> *u8 { **self } }

impl wxScreenDC {
    pub fn from(handle: *u8) -> @_wxScreenDC {
        @wxScreenDC(handle) as @_wxScreenDC
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxScreenDC {
        unsafe { @wxScreenDC(wxScreenDC_Create()) as @_wxScreenDC }
    }
}

trait _wxScreenDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxScreenDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endDrawingOnTop(&self) -> bool {
        unsafe { wxScreenDC_EndDrawingOnTop(self.handle()) }
    }
    #[fixed_stack_segment]
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTop(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn startDrawingOnTopOfWin(&self, win: @_wxWindow) -> bool {
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
    pub fn from(handle: *u8) -> @_wxScrollBar {
        @wxScrollBar(handle) as @_wxScrollBar
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxScrollBar {
        unsafe { @wxScrollBar(wxScrollBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxScrollBar }
    }
}

trait _wxScrollBar : _wxControl {
    #[fixed_stack_segment]
    fn getPageSize(&self) -> c_int {
        unsafe { wxScrollBar_GetPageSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRange(&self) -> c_int {
        unsafe { wxScrollBar_GetRange(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getThumbPosition(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getThumbSize(&self) -> c_int {
        unsafe { wxScrollBar_GetThumbSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setScrollbar(&self, position: c_int, thumbSize: c_int, range: c_int, pageSize: c_int, refresh: bool) {
        unsafe { wxScrollBar_SetScrollbar(self.handle(), position, thumbSize, range, pageSize, refresh) }
    }
    #[fixed_stack_segment]
    fn setThumbPosition(&self, viewStart: c_int) {
        unsafe { wxScrollBar_SetThumbPosition(self.handle(), viewStart) }
    }
}

struct wxScrollEvent(*u8);
impl _wxScrollEvent for wxScrollEvent {}
impl _wxEvent for wxScrollEvent {}
impl _wxObject for wxScrollEvent { fn handle(&self) -> *u8 { **self } }

impl wxScrollEvent {
    pub fn from(handle: *u8) -> @_wxScrollEvent {
        @wxScrollEvent(handle) as @_wxScrollEvent
    }
    
}

trait _wxScrollEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.handle()) }
    }
}

struct wxScrollWinEvent(*u8);
impl _wxScrollWinEvent for wxScrollWinEvent {}
impl _wxEvent for wxScrollWinEvent {}
impl _wxObject for wxScrollWinEvent { fn handle(&self) -> *u8 { **self } }

impl wxScrollWinEvent {
    pub fn from(handle: *u8) -> @_wxScrollWinEvent {
        @wxScrollWinEvent(handle) as @_wxScrollWinEvent
    }
    
}

trait _wxScrollWinEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollWinEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxScrollWinEvent_SetOrientation(self.handle(), orient) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxScrolledWindow {
        @wxScrolledWindow(handle) as @_wxScrolledWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxScrolledWindow {
        unsafe { @wxScrolledWindow(wxScrolledWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxScrolledWindow }
    }
}

trait _wxScrolledWindow : _wxPanel {
    #[fixed_stack_segment]
    fn adjustScrollbars(&self) {
        unsafe { wxScrolledWindow_AdjustScrollbars(self.handle()) }
    }
    #[fixed_stack_segment]
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int) {
        unsafe { wxScrolledWindow_CalcScrolledPosition(self.handle(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int) {
        unsafe { wxScrolledWindow_CalcUnscrolledPosition(self.handle(), x, y, xx, yy) }
    }
    #[fixed_stack_segment]
    fn enableScrolling(&self, x_scrolling: bool, y_scrolling: bool) {
        unsafe { wxScrolledWindow_EnableScrolling(self.handle(), x_scrolling, y_scrolling) }
    }
    #[fixed_stack_segment]
    fn getScaleX(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getScaleY(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getScrollPageSize(&self, orient: c_int) -> c_int {
        unsafe { wxScrolledWindow_GetScrollPageSize(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn getScrollPixelsPerUnit(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetScrollPixelsPerUnit(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getTargetWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxScrolledWindow_GetTargetWindow(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getViewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetViewStart(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetVirtualSize(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn onDraw(&self, dc: @_wxDC) {
        unsafe { wxScrolledWindow_OnDraw(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @_wxDC) {
        unsafe { wxScrolledWindow_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn scroll(&self, x_pos: c_int, y_pos: c_int) {
        unsafe { wxScrolledWindow_Scroll(self.handle(), x_pos, y_pos) }
    }
    #[fixed_stack_segment]
    fn setScale(&self, xs: c_double, ys: c_double) {
        unsafe { wxScrolledWindow_SetScale(self.handle(), xs, ys) }
    }
    #[fixed_stack_segment]
    fn setScrollPageSize(&self, orient: c_int, pageSize: c_int) {
        unsafe { wxScrolledWindow_SetScrollPageSize(self.handle(), orient, pageSize) }
    }
    #[fixed_stack_segment]
    fn setScrollbars(&self, pixelsPerUnitX: c_int, pixelsPerUnitY: c_int, noUnitsX: c_int, noUnitsY: c_int, xPos: c_int, yPos: c_int, noRefresh: bool) {
        unsafe { wxScrolledWindow_SetScrollbars(self.handle(), pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
    #[fixed_stack_segment]
    fn showScrollbars(&self, showh: c_int, showv: c_int) {
        unsafe { wxScrolledWindow_ShowScrollbars(self.handle(), showh, showv) }
    }
    #[fixed_stack_segment]
    fn setTargetWindow(&self, target: @_wxWindow) {
        unsafe { wxScrolledWindow_SetTargetWindow(self.handle(), target.handle()) }
    }
    #[fixed_stack_segment]
    fn viewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_ViewStart(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.handle(), xstep, ystep) }
    }
}

struct wxSemaphore(*u8);
impl _wxSemaphore for wxSemaphore { fn handle(&self) -> *u8 { **self } }

impl wxSemaphore {
    pub fn from(handle: *u8) -> @_wxSemaphore {
        @wxSemaphore(handle) as @_wxSemaphore
    }
    
}

trait _wxSemaphore {
    fn handle(&self) -> *u8;
    
}

struct wxServer(*u8);
impl _wxServer for wxServer {}
impl _wxServerBase for wxServer {}
impl _wxObject for wxServer { fn handle(&self) -> *u8 { **self } }

impl wxServer {
    pub fn from(handle: *u8) -> @_wxServer {
        @wxServer(handle) as @_wxServer
    }
    
}

trait _wxServer : _wxServerBase {
}

struct wxServerBase(*u8);
impl _wxServerBase for wxServerBase {}
impl _wxObject for wxServerBase { fn handle(&self) -> *u8 { **self } }

impl wxServerBase {
    pub fn from(handle: *u8) -> @_wxServerBase {
        @wxServerBase(handle) as @_wxServerBase
    }
    
}

trait _wxServerBase : _wxObject {
}

struct wxSetCursorEvent(*u8);
impl _wxSetCursorEvent for wxSetCursorEvent {}
impl _wxEvent for wxSetCursorEvent {}
impl _wxObject for wxSetCursorEvent { fn handle(&self) -> *u8 { **self } }

impl wxSetCursorEvent {
    pub fn from(handle: *u8) -> @_wxSetCursorEvent {
        @wxSetCursorEvent(handle) as @_wxSetCursorEvent
    }
    
}

trait _wxSetCursorEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getCursor(&self) -> @_wxCursor {
        unsafe { @wxCursor(wxSetCursorEvent_GetCursor(self.handle())) as @_wxCursor }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> c_int {
        unsafe { wxSetCursorEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasCursor(&self) -> bool {
        unsafe { wxSetCursorEvent_HasCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCursor(&self, cursor: @_wxCursor) {
        unsafe { wxSetCursorEvent_SetCursor(self.handle(), cursor.handle()) }
    }
}

struct wxShowEvent(*u8);
impl _wxShowEvent for wxShowEvent {}
impl _wxEvent for wxShowEvent {}
impl _wxObject for wxShowEvent { fn handle(&self) -> *u8 { **self } }

impl wxShowEvent {
    pub fn from(handle: *u8) -> @_wxShowEvent {
        @wxShowEvent(handle) as @_wxShowEvent
    }
    
}

trait _wxShowEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @_wxObject) {
        unsafe { wxShowEvent_CopyObject(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn isShown(&self) -> bool {
        unsafe { wxShowEvent_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setShow(&self, show: bool) {
        unsafe { wxShowEvent_SetShow(self.handle(), show) }
    }
}

struct wxSimpleHelpProvider(*u8);
impl _wxSimpleHelpProvider for wxSimpleHelpProvider {}
impl _wxHelpProvider for wxSimpleHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxSimpleHelpProvider {
    pub fn from(handle: *u8) -> @_wxSimpleHelpProvider {
        @wxSimpleHelpProvider(handle) as @_wxSimpleHelpProvider
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxSimpleHelpProvider {
        unsafe { @wxSimpleHelpProvider(wxSimpleHelpProvider_Create()) as @_wxSimpleHelpProvider }
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
    pub fn from(handle: *u8) -> @_wxSingleChoiceDialog {
        @wxSingleChoiceDialog(handle) as @_wxSingleChoiceDialog
    }
    
}

trait _wxSingleChoiceDialog : _wxDialog {
}

struct wxSingleInstanceChecker(*u8);
impl _wxSingleInstanceChecker for wxSingleInstanceChecker { fn handle(&self) -> *u8 { **self } }

impl wxSingleInstanceChecker {
    pub fn from(handle: *u8) -> @_wxSingleInstanceChecker {
        @wxSingleInstanceChecker(handle) as @_wxSingleInstanceChecker
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, name: @_wxString, path: @_wxString) -> bool {
        unsafe { wxSingleInstanceChecker_Create(_obj, name.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxSingleInstanceChecker {
        unsafe { @wxSingleInstanceChecker(wxSingleInstanceChecker_CreateDefault()) as @_wxSingleInstanceChecker }
    }
}

trait _wxSingleInstanceChecker {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isAnotherRunning(&self) -> bool {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.handle()) }
    }
}

struct wxSize(*u8);
impl _wxSize for wxSize { fn handle(&self) -> *u8 { **self } }

impl wxSize {
    pub fn from(handle: *u8) -> @_wxSize {
        @wxSize(handle) as @_wxSize
    }
    
    #[fixed_stack_segment]
    pub fn new(w: c_int, h: c_int) -> @_wxSize {
        unsafe { @wxSize(wxSize_Create(w, h)) as @_wxSize }
    }
}

trait _wxSize {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getHeight(&self) -> c_int {
        unsafe { wxSize_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxSize_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setHeight(&self, h: c_int) {
        unsafe { wxSize_SetHeight(self.handle(), h) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, w: c_int) {
        unsafe { wxSize_SetWidth(self.handle(), w) }
    }
}

struct wxSizeEvent(*u8);
impl _wxSizeEvent for wxSizeEvent {}
impl _wxEvent for wxSizeEvent {}
impl _wxObject for wxSizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxSizeEvent {
    pub fn from(handle: *u8) -> @_wxSizeEvent {
        @wxSizeEvent(handle) as @_wxSizeEvent
    }
    
}

trait _wxSizeEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxSizeEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizeEvent_GetSize(self.handle())) as @_wxSize }
    }
}

struct wxSizer(*u8);
impl _wxSizer for wxSizer {}
impl _wxObject for wxSizer { fn handle(&self) -> *u8 { **self } }

impl wxSizer {
    pub fn from(handle: *u8) -> @_wxSizer {
        @wxSizer(handle) as @_wxSizer
    }
    
}

trait _wxSizer : _wxObject {
    #[fixed_stack_segment]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Add(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addSizer(&self, sizer: @_wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addWindow(&self, window: @_wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizer_CalcMin(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn fit(&self, window: @_wxWindow) {
        unsafe { wxSizer_Fit(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizer_GetMinSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxSizer_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizer_GetSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Insert(self.handle(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertSizer(&self, before: c_int, sizer: @_wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertSizer(self.handle(), before, sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertWindow(&self, before: c_int, window: @_wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertWindow(self.handle(), before, window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Prepend(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependSizer(&self, sizer: @_wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_PrependSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependWindow(&self, window: @_wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_PrependWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxSizer_RecalcSizes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDimension(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetDimension(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn setItemMinSize(&self, pos: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSize(self.handle(), pos, width, height) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeSizer(&self, sizer: @_wxSizer, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.handle(), sizer.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeWindow(&self, window: @_wxWindow, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.handle(), window.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setSizeHints(&self, window: @_wxWindow) {
        unsafe { wxSizer_SetSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn addSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddSpacer(self.handle(), size) }
    }
    #[fixed_stack_segment]
    fn addStretchSpacer(&self, size: c_int) {
        unsafe { wxSizer_AddStretchSpacer(self.handle(), size) }
    }
    #[fixed_stack_segment]
    fn clear(&self, delete_windows: bool) {
        unsafe { wxSizer_Clear(self.handle(), delete_windows) }
    }
    #[fixed_stack_segment]
    fn detachWindow(&self, window: @_wxWindow) -> bool {
        unsafe { wxSizer_DetachWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn detachSizer(&self, sizer: @_wxSizer) -> bool {
        unsafe { wxSizer_DetachSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn detach(&self, index: c_int) -> bool {
        unsafe { wxSizer_Detach(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn fitInside(&self, window: @_wxWindow) {
        unsafe { wxSizer_FitInside(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getContainingWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxSizer_GetContainingWindow(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getItemWindow(&self, window: @_wxWindow, recursive: bool) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemWindow(self.handle(), window.handle(), recursive)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn getItemSizer(&self, window: @_wxSizer, recursive: bool) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemSizer(self.handle(), window.handle(), recursive)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn getItem(&self, index: c_int) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItem(self.handle(), index)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn hideWindow(&self, window: @_wxWindow) -> bool {
        unsafe { wxSizer_HideWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn hideSizer(&self, sizer: @_wxSizer) -> bool {
        unsafe { wxSizer_HideSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn hide(&self, index: c_int) -> bool {
        unsafe { wxSizer_Hide(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn insertSpacer(&self, index: c_int, size: c_int) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_InsertSpacer(self.handle(), index, size)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_InsertStretchSpacer(self.handle(), index, prop)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn isShownWindow(&self, window: *u8) -> bool {
        unsafe { wxSizer_IsShownWindow(self.handle(), window) }
    }
    #[fixed_stack_segment]
    fn isShownSizer(&self, sizer: *u8) -> bool {
        unsafe { wxSizer_IsShownSizer(self.handle(), sizer) }
    }
    #[fixed_stack_segment]
    fn isShown(&self, index: c_int) -> bool {
        unsafe { wxSizer_IsShown(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn prependSpacer(&self, size: c_int) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_PrependSpacer(self.handle(), size)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn prependStretchSpacer(&self, prop: c_int) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizer_PrependStretchSpacer(self.handle(), prop)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, oldwin: @_wxWindow, newwin: @_wxWindow, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceWindow(self.handle(), oldwin.handle(), newwin.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replaceSizer(&self, oldsz: @_wxSizer, newsz: @_wxSizer, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceSizer(self.handle(), oldsz.handle(), newsz.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replace(&self, oldindex: c_int, newitem: @_wxSizerItem) -> bool {
        unsafe { wxSizer_Replace(self.handle(), oldindex, newitem.handle()) }
    }
    #[fixed_stack_segment]
    fn setVirtualSizeHints(&self, window: @_wxWindow) {
        unsafe { wxSizer_SetVirtualSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn showWindow(&self, window: @_wxWindow, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowWindow(self.handle(), window.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn showSizer(&self, sizer: @_wxSizer, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowSizer(self.handle(), sizer.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn show(&self, sizer: @_wxSizer, index: c_int, show: bool) -> bool {
        unsafe { wxSizer_Show(self.handle(), sizer.handle(), index, show) }
    }
}

struct wxSizerItem(*u8);
impl _wxSizerItem for wxSizerItem {}
impl _wxObject for wxSizerItem { fn handle(&self) -> *u8 { **self } }

impl wxSizerItem {
    pub fn from(handle: *u8) -> @_wxSizerItem {
        @wxSizerItem(handle) as @_wxSizerItem
    }
    
    #[fixed_stack_segment]
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) -> @_wxSizerItem {
        unsafe { @wxSizerItem(wxSizerItem_Create(width, height, option, flag, border, userData)) as @_wxSizerItem }
    }
    #[fixed_stack_segment]
    pub fn newInSizer(sizer: @_wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInSizer(sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    pub fn newInWindow(window: @_wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInWindow(window.handle(), option, flag, border, userData) }
    }
}

trait _wxSizerItem : _wxObject {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizerItem_CalcMin(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> c_int {
        unsafe { wxSizerItem_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFlag(&self) -> c_int {
        unsafe { wxSizerItem_GetFlag(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizerItem_GetMinSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxSizerItem_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizerItem_GetSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @_wxSizer {
        unsafe { @wxSizer(wxSizerItem_GetSizer(self.handle())) as @_wxSizer }
    }
    #[fixed_stack_segment]
    fn getUserData(&self) -> *u8 {
        unsafe { wxSizerItem_GetUserData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxSizerItem_GetWindow(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn isSizer(&self) -> bool {
        unsafe { wxSizerItem_IsSizer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSpacer(&self) -> bool {
        unsafe { wxSizerItem_IsSpacer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isWindow(&self) -> bool {
        unsafe { wxSizerItem_IsWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: c_int) {
        unsafe { wxSizerItem_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    fn setDimension(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxSizerItem_SetDimension(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn setFlag(&self, flag: c_int) {
        unsafe { wxSizerItem_SetFlag(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setFloatRatio(&self, ratio: c_float) {
        unsafe { wxSizerItem_SetFloatRatio(self.handle(), ratio) }
    }
    #[fixed_stack_segment]
    fn setInitSize(&self, x: c_int, y: c_int) {
        unsafe { wxSizerItem_SetInitSize(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setRatio(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetRatio(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setSizer(&self, sizer: @_wxSizer) {
        unsafe { wxSizerItem_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, window: @_wxWindow) {
        unsafe { wxSizerItem_SetWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSizerItem_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteWindows(&self) {
        unsafe { wxSizerItem_DeleteWindows(self.handle()) }
    }
    #[fixed_stack_segment]
    fn detachSizer(&self) {
        unsafe { wxSizerItem_DetachSizer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getProportion(&self) -> c_int {
        unsafe { wxSizerItem_GetProportion(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @_wxRect {
        unsafe { @wxRect(wxSizerItem_GetRect(self.handle())) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn getSpacer(&self) -> @_wxSize {
        unsafe { @wxSize(wxSizerItem_GetSpacer(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn isShown(&self) -> c_int {
        unsafe { wxSizerItem_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setProportion(&self, proportion: c_int) {
        unsafe { wxSizerItem_SetProportion(self.handle(), proportion) }
    }
    #[fixed_stack_segment]
    fn setSpacer(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetSpacer(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxSlider {
        @wxSlider(handle) as @_wxSlider
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @_wxSlider {
        unsafe { @wxSlider(wxSlider_Create(_prt.handle(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) as @_wxSlider }
    }
}

trait _wxSlider : _wxControl {
    #[fixed_stack_segment]
    fn clearSel(&self) {
        unsafe { wxSlider_ClearSel(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearTicks(&self) {
        unsafe { wxSlider_ClearTicks(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLineSize(&self) -> c_int {
        unsafe { wxSlider_GetLineSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMax(&self) -> c_int {
        unsafe { wxSlider_GetMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMin(&self) -> c_int {
        unsafe { wxSlider_GetMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> c_int {
        unsafe { wxSlider_GetPageSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelEnd(&self) -> c_int {
        unsafe { wxSlider_GetSelEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelStart(&self) -> c_int {
        unsafe { wxSlider_GetSelStart(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getThumbLength(&self) -> c_int {
        unsafe { wxSlider_GetThumbLength(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTickFreq(&self) -> c_int {
        unsafe { wxSlider_GetTickFreq(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> c_int {
        unsafe { wxSlider_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLineSize(&self, lineSize: c_int) {
        unsafe { wxSlider_SetLineSize(self.handle(), lineSize) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, pageSize: c_int) {
        unsafe { wxSlider_SetPageSize(self.handle(), pageSize) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, minValue: c_int, maxValue: c_int) {
        unsafe { wxSlider_SetRange(self.handle(), minValue, maxValue) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, minPos: c_int, maxPos: c_int) {
        unsafe { wxSlider_SetSelection(self.handle(), minPos, maxPos) }
    }
    #[fixed_stack_segment]
    fn setThumbLength(&self, len: c_int) {
        unsafe { wxSlider_SetThumbLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    fn setTick(&self, tickPos: c_int) {
        unsafe { wxSlider_SetTick(self.handle(), tickPos) }
    }
    #[fixed_stack_segment]
    fn setTickFreq(&self, n: c_int, pos: c_int) {
        unsafe { wxSlider_SetTickFreq(self.handle(), n, pos) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: c_int) {
        unsafe { wxSlider_SetValue(self.handle(), value) }
    }
}

struct wxSockAddress(*u8);
impl _wxSockAddress for wxSockAddress {}
impl _wxObject for wxSockAddress { fn handle(&self) -> *u8 { **self } }

impl wxSockAddress {
    pub fn from(handle: *u8) -> @_wxSockAddress {
        @wxSockAddress(handle) as @_wxSockAddress
    }
    
}

trait _wxSockAddress : _wxObject {
}

struct wxSocketBase(*u8);
impl _wxSocketBase for wxSocketBase {}
impl _wxObject for wxSocketBase { fn handle(&self) -> *u8 { **self } }

impl wxSocketBase {
    pub fn from(handle: *u8) -> @_wxSocketBase {
        @wxSocketBase(handle) as @_wxSocketBase
    }
    
}

trait _wxSocketBase : _wxObject {
}

struct wxSocketClient(*u8);
impl _wxSocketClient for wxSocketClient {}
impl _wxSocketBase for wxSocketClient {}
impl _wxObject for wxSocketClient { fn handle(&self) -> *u8 { **self } }

impl wxSocketClient {
    pub fn from(handle: *u8) -> @_wxSocketClient {
        @wxSocketClient(handle) as @_wxSocketClient
    }
    
}

trait _wxSocketClient : _wxSocketBase {
}

struct wxSocketEvent(*u8);
impl _wxSocketEvent for wxSocketEvent {}
impl _wxEvent for wxSocketEvent {}
impl _wxObject for wxSocketEvent { fn handle(&self) -> *u8 { **self } }

impl wxSocketEvent {
    pub fn from(handle: *u8) -> @_wxSocketEvent {
        @wxSocketEvent(handle) as @_wxSocketEvent
    }
    
}

trait _wxSocketEvent : _wxEvent {
}

struct wxSocketInputStream(*u8);
impl _wxSocketInputStream for wxSocketInputStream {}
impl _wxInputStream for wxSocketInputStream {}
impl _wxStreamBase for wxSocketInputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketInputStream {
    pub fn from(handle: *u8) -> @_wxSocketInputStream {
        @wxSocketInputStream(handle) as @_wxSocketInputStream
    }
    
}

trait _wxSocketInputStream : _wxInputStream {
}

struct wxSocketOutputStream(*u8);
impl _wxSocketOutputStream for wxSocketOutputStream {}
impl _wxOutputStream for wxSocketOutputStream {}
impl _wxStreamBase for wxSocketOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketOutputStream {
    pub fn from(handle: *u8) -> @_wxSocketOutputStream {
        @wxSocketOutputStream(handle) as @_wxSocketOutputStream
    }
    
}

trait _wxSocketOutputStream : _wxOutputStream {
}

struct wxSocketServer(*u8);
impl _wxSocketServer for wxSocketServer {}
impl _wxSocketBase for wxSocketServer {}
impl _wxObject for wxSocketServer { fn handle(&self) -> *u8 { **self } }

impl wxSocketServer {
    pub fn from(handle: *u8) -> @_wxSocketServer {
        @wxSocketServer(handle) as @_wxSocketServer
    }
    
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
    pub fn from(handle: *u8) -> @_wxSpinButton {
        @wxSpinButton(handle) as @_wxSpinButton
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @_wxSpinButton {
        unsafe { @wxSpinButton(wxSpinButton_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxSpinButton }
    }
}

trait _wxSpinButton : _wxControl {
    #[fixed_stack_segment]
    fn getMax(&self) -> c_int {
        unsafe { wxSpinButton_GetMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMin(&self) -> c_int {
        unsafe { wxSpinButton_GetMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> c_int {
        unsafe { wxSpinButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, minVal: c_int, maxVal: c_int) {
        unsafe { wxSpinButton_SetRange(self.handle(), minVal, maxVal) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxSpinCtrl {
        @wxSpinCtrl(handle) as @_wxSpinCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> @_wxSpinCtrl {
        unsafe { @wxSpinCtrl(wxSpinCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) as @_wxSpinCtrl }
    }
}

trait _wxSpinCtrl : _wxControl {
    #[fixed_stack_segment]
    fn getMax(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMin(&self) -> c_int {
        unsafe { wxSpinCtrl_GetMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> c_int {
        unsafe { wxSpinCtrl_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, min_val: c_int, max_val: c_int) {
        unsafe { wxSpinCtrl_SetRange(self.handle(), min_val, max_val) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxSpinEvent {
        @wxSpinEvent(handle) as @_wxSpinEvent
    }
    
}

trait _wxSpinEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxSplashScreen {
        @wxSplashScreen(handle) as @_wxSplashScreen
    }
    
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
    pub fn from(handle: *u8) -> @_wxSplitterEvent {
        @wxSplitterEvent(handle) as @_wxSplitterEvent
    }
    
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
    pub fn from(handle: *u8) -> @_wxSplitterScrolledWindow {
        @wxSplitterScrolledWindow(handle) as @_wxSplitterScrolledWindow
    }
    
}

trait _wxSplitterScrolledWindow : _wxScrolledWindow {
}

struct wxSplitterWindow(*u8);
impl _wxSplitterWindow for wxSplitterWindow {}
impl _wxWindow for wxSplitterWindow {}
impl _wxEvtHandler for wxSplitterWindow {}
impl _wxObject for wxSplitterWindow { fn handle(&self) -> *u8 { **self } }

impl wxSplitterWindow {
    pub fn from(handle: *u8) -> @_wxSplitterWindow {
        @wxSplitterWindow(handle) as @_wxSplitterWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxSplitterWindow {
        unsafe { @wxSplitterWindow(wxSplitterWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxSplitterWindow }
    }
}

trait _wxSplitterWindow : _wxWindow {
    #[fixed_stack_segment]
    fn getBorderSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetBorderSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinimumPaneSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetMinimumPaneSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSashPosition(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSashSize(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSashSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSplitMode(&self) -> c_int {
        unsafe { wxSplitterWindow_GetSplitMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow1(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxSplitterWindow_GetWindow1(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getWindow2(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxSplitterWindow_GetWindow2(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn initialize(&self, window: @_wxWindow) {
        unsafe { wxSplitterWindow_Initialize(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn isSplit(&self) -> bool {
        unsafe { wxSplitterWindow_IsSplit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, winOld: @_wxWindow, winNew: @_wxWindow) -> bool {
        unsafe { wxSplitterWindow_ReplaceWindow(self.handle(), winOld.handle(), winNew.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorderSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetBorderSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn setMinimumPaneSize(&self, min: c_int) {
        unsafe { wxSplitterWindow_SetMinimumPaneSize(self.handle(), min) }
    }
    #[fixed_stack_segment]
    fn setSashPosition(&self, position: c_int, redraw: bool) {
        unsafe { wxSplitterWindow_SetSashPosition(self.handle(), position, redraw) }
    }
    #[fixed_stack_segment]
    fn setSashSize(&self, width: c_int) {
        unsafe { wxSplitterWindow_SetSashSize(self.handle(), width) }
    }
    #[fixed_stack_segment]
    fn setSplitMode(&self, mode: c_int) {
        unsafe { wxSplitterWindow_SetSplitMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn splitHorizontally(&self, window1: @_wxWindow, window2: @_wxWindow, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitHorizontally(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn splitVertically(&self, window1: @_wxWindow, window2: @_wxWindow, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitVertically(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn unsplit(&self, toRemove: @_wxWindow) -> bool {
        unsafe { wxSplitterWindow_Unsplit(self.handle(), toRemove.handle()) }
    }
    #[fixed_stack_segment]
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxStaticBitmap {
        @wxStaticBitmap(handle) as @_wxStaticBitmap
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, bitmap: @_wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxStaticBitmap {
        unsafe { @wxStaticBitmap(wxStaticBitmap_Create(_prt.handle(), _id, bitmap.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxStaticBitmap }
    }
}

trait _wxStaticBitmap : _wxControl {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStaticBitmap_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmap(&self, _ref: @_wxBitmap) {
        unsafe { wxStaticBitmap_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, _ref: @_wxIcon) {
        unsafe { wxStaticBitmap_GetIcon(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmap(&self, bitmap: @_wxBitmap) {
        unsafe { wxStaticBitmap_SetBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @_wxIcon) {
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
    pub fn from(handle: *u8) -> @_wxStaticBox {
        @wxStaticBox(handle) as @_wxStaticBox
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxStaticBox {
        unsafe { @wxStaticBox(wxStaticBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxStaticBox }
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
    pub fn from(handle: *u8) -> @_wxStaticBoxSizer {
        @wxStaticBoxSizer(handle) as @_wxStaticBoxSizer
    }
    
    #[fixed_stack_segment]
    pub fn new(box: @_wxStaticBox, orient: c_int) -> @_wxStaticBoxSizer {
        unsafe { @wxStaticBoxSizer(wxStaticBoxSizer_Create(box.handle(), orient)) as @_wxStaticBoxSizer }
    }
}

trait _wxStaticBoxSizer : _wxBoxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @_wxSize {
        unsafe { @wxSize(wxStaticBoxSizer_CalcMin(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self) -> @_wxStaticBox {
        unsafe { @wxStaticBox(wxStaticBoxSizer_GetStaticBox(self.handle())) as @_wxStaticBox }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxStaticBoxSizer_RecalcSizes(self.handle()) }
    }
}

struct wxStaticLine(*u8);
impl _wxStaticLine for wxStaticLine {}
impl _wxControl for wxStaticLine {}
impl _wxWindow for wxStaticLine {}
impl _wxEvtHandler for wxStaticLine {}
impl _wxObject for wxStaticLine { fn handle(&self) -> *u8 { **self } }

impl wxStaticLine {
    pub fn from(handle: *u8) -> @_wxStaticLine {
        @wxStaticLine(handle) as @_wxStaticLine
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxStaticLine {
        unsafe { @wxStaticLine(wxStaticLine_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxStaticLine }
    }
}

trait _wxStaticLine : _wxControl {
    #[fixed_stack_segment]
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxStaticText {
        @wxStaticText(handle) as @_wxStaticText
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxStaticText {
        unsafe { @wxStaticText(wxStaticText_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxStaticText }
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
    pub fn from(handle: *u8) -> @_wxStatusBar {
        @wxStatusBar(handle) as @_wxStatusBar
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxStatusBar {
        unsafe { @wxStatusBar(wxStatusBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxStatusBar }
    }
}

trait _wxStatusBar : _wxWindow {
    #[fixed_stack_segment]
    fn getBorderX(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorderY(&self) -> c_int {
        unsafe { wxStatusBar_GetBorderY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFieldsCount(&self) -> c_int {
        unsafe { wxStatusBar_GetFieldsCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStatusText(&self, number: c_int) -> @_wxString {
        unsafe { @wxString(wxStatusBar_GetStatusText(self.handle(), number)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setFieldsCount(&self, number: c_int, widths: *c_int) {
        unsafe { wxStatusBar_SetFieldsCount(self.handle(), number, widths) }
    }
    #[fixed_stack_segment]
    fn setMinHeight(&self, height: c_int) {
        unsafe { wxStatusBar_SetMinHeight(self.handle(), height) }
    }
    #[fixed_stack_segment]
    fn setStatusText(&self, text: @_wxString, number: c_int) {
        unsafe { wxStatusBar_SetStatusText(self.handle(), text.handle(), number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, n: c_int, widths: *c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.handle(), n, widths) }
    }
}

struct wxStopWatch(*u8);
impl _wxStopWatch for wxStopWatch { fn handle(&self) -> *u8 { **self } }

impl wxStopWatch {
    pub fn from(handle: *u8) -> @_wxStopWatch {
        @wxStopWatch(handle) as @_wxStopWatch
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxStopWatch {
        unsafe { @wxStopWatch(wxStopWatch_Create()) as @_wxStopWatch }
    }
}

trait _wxStopWatch {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStopWatch_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn start(&self, msec: c_int) {
        unsafe { wxStopWatch_Start(self.handle(), msec) }
    }
    #[fixed_stack_segment]
    fn pause(&self) {
        unsafe { wxStopWatch_Pause(self.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxStopWatch_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn time(&self) -> c_int {
        unsafe { wxStopWatch_Time(self.handle()) }
    }
}

struct wxStreamBase(*u8);
impl _wxStreamBase for wxStreamBase { fn handle(&self) -> *u8 { **self } }

impl wxStreamBase {
    pub fn from(handle: *u8) -> @_wxStreamBase {
        @wxStreamBase(handle) as @_wxStreamBase
    }
    
}

trait _wxStreamBase {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getLastError(&self) -> c_int {
        unsafe { wxStreamBase_GetLastError(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> c_int {
        unsafe { wxStreamBase_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxStreamBase_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStreamBase_Delete(self.handle()) }
    }
}

struct wxStreamBuffer(*u8);
impl _wxStreamBuffer for wxStreamBuffer { fn handle(&self) -> *u8 { **self } }

impl wxStreamBuffer {
    pub fn from(handle: *u8) -> @_wxStreamBuffer {
        @wxStreamBuffer(handle) as @_wxStreamBuffer
    }
    
}

trait _wxStreamBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStreamToTextRedirector(*u8);
impl _wxStreamToTextRedirector for wxStreamToTextRedirector { fn handle(&self) -> *u8 { **self } }

impl wxStreamToTextRedirector {
    pub fn from(handle: *u8) -> @_wxStreamToTextRedirector {
        @wxStreamToTextRedirector(handle) as @_wxStreamToTextRedirector
    }
    
}

trait _wxStreamToTextRedirector {
    fn handle(&self) -> *u8;
    
}

struct wxString(*u8);
impl _wxString for wxString { fn handle(&self) -> *u8 { **self } }

impl wxString {
    pub fn from(handle: *u8) -> @_wxString {
        @wxString(handle) as @_wxString
    }
    
    #[fixed_stack_segment]
    pub fn new(buffer: *wchar_t) -> @_wxString {
        unsafe { @wxString(wxString_Create(buffer)) as @_wxString }
    }
    #[fixed_stack_segment]
    pub fn newLen(buffer: *wchar_t, len: c_int) -> @_wxString {
        unsafe { @wxString(wxString_CreateLen(buffer, len)) as @_wxString }
    }
}

trait _wxString {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxString_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getString(&self, buffer: *wchar_t) -> c_int {
        unsafe { wxString_GetString(self.handle(), buffer) }
    }
    #[fixed_stack_segment]
    fn length(&self) -> size_t {
        unsafe { wxString_Length(self.handle()) }
    }
}

struct wxStringBuffer(*u8);
impl _wxStringBuffer for wxStringBuffer { fn handle(&self) -> *u8 { **self } }

impl wxStringBuffer {
    pub fn from(handle: *u8) -> @_wxStringBuffer {
        @wxStringBuffer(handle) as @_wxStringBuffer
    }
    
}

trait _wxStringBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStringClientData(*u8);
impl _wxStringClientData for wxStringClientData {}
impl _wxClientData for wxStringClientData { fn handle(&self) -> *u8 { **self } }

impl wxStringClientData {
    pub fn from(handle: *u8) -> @_wxStringClientData {
        @wxStringClientData(handle) as @_wxStringClientData
    }
    
}

trait _wxStringClientData : _wxClientData {
}

struct wxStringList(*u8);
impl _wxStringList for wxStringList {}
impl _wxList for wxStringList {}
impl _wxObject for wxStringList { fn handle(&self) -> *u8 { **self } }

impl wxStringList {
    pub fn from(handle: *u8) -> @_wxStringList {
        @wxStringList(handle) as @_wxStringList
    }
    
}

trait _wxStringList : _wxList {
}

struct wxStringTokenizer(*u8);
impl _wxStringTokenizer for wxStringTokenizer {}
impl _wxObject for wxStringTokenizer { fn handle(&self) -> *u8 { **self } }

impl wxStringTokenizer {
    pub fn from(handle: *u8) -> @_wxStringTokenizer {
        @wxStringTokenizer(handle) as @_wxStringTokenizer
    }
    
}

trait _wxStringTokenizer : _wxObject {
}

struct wxSysColourChangedEvent(*u8);
impl _wxSysColourChangedEvent for wxSysColourChangedEvent {}
impl _wxEvent for wxSysColourChangedEvent {}
impl _wxObject for wxSysColourChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxSysColourChangedEvent {
    pub fn from(handle: *u8) -> @_wxSysColourChangedEvent {
        @wxSysColourChangedEvent(handle) as @_wxSysColourChangedEvent
    }
    
}

trait _wxSysColourChangedEvent : _wxEvent {
}

struct wxSystemOptions(*u8);
impl _wxSystemOptions for wxSystemOptions {}
impl _wxObject for wxSystemOptions { fn handle(&self) -> *u8 { **self } }

impl wxSystemOptions {
    pub fn from(handle: *u8) -> @_wxSystemOptions {
        @wxSystemOptions(handle) as @_wxSystemOptions
    }
    
}

trait _wxSystemOptions : _wxObject {
}

struct wxSystemSettings(*u8);
impl _wxSystemSettings for wxSystemSettings {}
impl _wxObject for wxSystemSettings { fn handle(&self) -> *u8 { **self } }

impl wxSystemSettings {
    pub fn from(handle: *u8) -> @_wxSystemSettings {
        @wxSystemSettings(handle) as @_wxSystemSettings
    }
    
    #[fixed_stack_segment]
    pub fn getColour(index: c_int, _ref: @_wxColour) {
        unsafe { wxSystemSettings_GetColour(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getFont(index: c_int, _ref: @_wxFont) {
        unsafe { wxSystemSettings_GetFont(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getMetric(index: c_int) -> c_int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxTabCtrl {
        @wxTabCtrl(handle) as @_wxTabCtrl
    }
    
}

trait _wxTabCtrl : _wxControl {
}

struct wxTabEvent(*u8);
impl _wxTabEvent for wxTabEvent {}
impl _wxCommandEvent for wxTabEvent {}
impl _wxEvent for wxTabEvent {}
impl _wxObject for wxTabEvent { fn handle(&self) -> *u8 { **self } }

impl wxTabEvent {
    pub fn from(handle: *u8) -> @_wxTabEvent {
        @wxTabEvent(handle) as @_wxTabEvent
    }
    
}

trait _wxTabEvent : _wxCommandEvent {
}

struct wxTablesInUse(*u8);
impl _wxTablesInUse for wxTablesInUse {}
impl _wxObject for wxTablesInUse { fn handle(&self) -> *u8 { **self } }

impl wxTablesInUse {
    pub fn from(handle: *u8) -> @_wxTablesInUse {
        @wxTablesInUse(handle) as @_wxTablesInUse
    }
    
}

trait _wxTablesInUse : _wxObject {
}

struct wxTaskBarIcon(*u8);
impl _wxTaskBarIcon for wxTaskBarIcon {}
impl _wxEvtHandler for wxTaskBarIcon {}
impl _wxObject for wxTaskBarIcon { fn handle(&self) -> *u8 { **self } }

impl wxTaskBarIcon {
    pub fn from(handle: *u8) -> @_wxTaskBarIcon {
        @wxTaskBarIcon(handle) as @_wxTaskBarIcon
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxTaskBarIcon {
        unsafe { @wxTaskBarIcon(wxTaskBarIcon_Create()) as @_wxTaskBarIcon }
    }
}

trait _wxTaskBarIcon : _wxEvtHandler {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTaskBarIcon_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isIconInstalled(&self) -> bool {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxTaskBarIcon_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn popupMenu(&self, menu: @_wxMenu) -> bool {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn removeIcon(&self) -> bool {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @_wxIcon, text: @_wxString) -> bool {
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

struct wxTempFile(*u8);
impl _wxTempFile for wxTempFile { fn handle(&self) -> *u8 { **self } }

impl wxTempFile {
    pub fn from(handle: *u8) -> @_wxTempFile {
        @wxTempFile(handle) as @_wxTempFile
    }
    
}

trait _wxTempFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextAttr(*u8);
impl _wxTextAttr for wxTextAttr { fn handle(&self) -> *u8 { **self } }

impl wxTextAttr {
    pub fn from(handle: *u8) -> @_wxTextAttr {
        @wxTextAttr(handle) as @_wxTextAttr
    }
    
    #[fixed_stack_segment]
    pub fn new(colText: @_wxColour, colBack: @_wxColour, font: @_wxFont) -> @_wxTextAttr {
        unsafe { @wxTextAttr(wxTextAttr_Create(colText.handle(), colBack.handle(), font.handle())) as @_wxTextAttr }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @_wxTextAttr {
        unsafe { @wxTextAttr(wxTextAttr_CreateDefault()) as @_wxTextAttr }
    }
}

trait _wxTextAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, colour: @_wxColour) {
        unsafe { wxTextAttr_GetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, font: @_wxFont) {
        unsafe { wxTextAttr_GetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, colour: @_wxColour) {
        unsafe { wxTextAttr_GetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxTextAttr_HasBackgroundColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasFont(&self) -> bool {
        unsafe { wxTextAttr_HasFont(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasTextColour(&self) -> bool {
        unsafe { wxTextAttr_HasTextColour(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isDefault(&self) -> bool {
        unsafe { wxTextAttr_IsDefault(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colour: @_wxColour) {
        unsafe { wxTextAttr_SetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @_wxColour) {
        unsafe { wxTextAttr_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont) {
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
    pub fn from(handle: *u8) -> @_wxTextCtrl {
        @wxTextCtrl(handle) as @_wxTextCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @_wxTextCtrl {
        unsafe { @wxTextCtrl(wxTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @_wxTextCtrl }
    }
}

trait _wxTextCtrl : _wxControl {
    #[fixed_stack_segment]
    fn appendText(&self, text: @_wxString) {
        unsafe { wxTextCtrl_AppendText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn canCopy(&self) -> bool {
        unsafe { wxTextCtrl_CanCopy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canCut(&self) -> bool {
        unsafe { wxTextCtrl_CanCut(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canPaste(&self) -> bool {
        unsafe { wxTextCtrl_CanPaste(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canRedo(&self) -> bool {
        unsafe { wxTextCtrl_CanRedo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { wxTextCtrl_CanUndo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn changeValue(&self, text: @_wxString) {
        unsafe { wxTextCtrl_ChangeValue(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxTextCtrl_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copy(&self) {
        unsafe { wxTextCtrl_Copy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn cut(&self) {
        unsafe { wxTextCtrl_Cut(self.handle()) }
    }
    #[fixed_stack_segment]
    fn discardEdits(&self) {
        unsafe { wxTextCtrl_DiscardEdits(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getInsertionPoint(&self) -> c_long {
        unsafe { wxTextCtrl_GetInsertionPoint(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastPosition(&self) -> c_long {
        unsafe { wxTextCtrl_GetLastPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLineLength(&self, lineNo: c_long) -> c_int {
        unsafe { wxTextCtrl_GetLineLength(self.handle(), lineNo) }
    }
    #[fixed_stack_segment]
    fn getLineText(&self, lineNo: c_long) -> @_wxString {
        unsafe { @wxString(wxTextCtrl_GetLineText(self.handle(), lineNo)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getNumberOfLines(&self) -> c_int {
        unsafe { wxTextCtrl_GetNumberOfLines(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, from: *u8, to: *u8) {
        unsafe { wxTextCtrl_GetSelection(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> @_wxString {
        unsafe { @wxString(wxTextCtrl_GetValue(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn isEditable(&self) -> bool {
        unsafe { wxTextCtrl_IsEditable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isModified(&self) -> bool {
        unsafe { wxTextCtrl_IsModified(self.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, file: @_wxString) -> bool {
        unsafe { wxTextCtrl_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn paste(&self) {
        unsafe { wxTextCtrl_Paste(self.handle()) }
    }
    #[fixed_stack_segment]
    fn positionToXY(&self, pos: c_long, x: *c_long, y: *c_long) -> c_int {
        unsafe { wxTextCtrl_PositionToXY(self.handle(), pos, x, y) }
    }
    #[fixed_stack_segment]
    fn redo(&self) {
        unsafe { wxTextCtrl_Redo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_Remove(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    fn replace(&self, from: c_long, to: c_long, value: @_wxString) {
        unsafe { wxTextCtrl_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, file: @_wxString) -> bool {
        unsafe { wxTextCtrl_SaveFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditable(&self, editable: bool) {
        unsafe { wxTextCtrl_SetEditable(self.handle(), editable) }
    }
    #[fixed_stack_segment]
    fn setInsertionPoint(&self, pos: c_long) {
        unsafe { wxTextCtrl_SetInsertionPoint(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setInsertionPointEnd(&self) {
        unsafe { wxTextCtrl_SetInsertionPointEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_SetSelection(self.handle(), from, to) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: @_wxString) {
        unsafe { wxTextCtrl_SetValue(self.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn showPosition(&self, pos: c_long) {
        unsafe { wxTextCtrl_ShowPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn undo(&self) {
        unsafe { wxTextCtrl_Undo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn writeText(&self, text: @_wxString) {
        unsafe { wxTextCtrl_WriteText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn emulateKeyPress(&self, keyevent: @_wxKeyEvent) -> bool {
        unsafe { wxTextCtrl_EmulateKeyPress(self.handle(), keyevent.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultStyle(&self) -> @_wxTextAttr {
        unsafe { @wxTextAttr(wxTextCtrl_GetDefaultStyle(self.handle())) as @_wxTextAttr }
    }
    #[fixed_stack_segment]
    fn getRange(&self, from: c_long, to: c_long) -> @_wxString {
        unsafe { @wxString(wxTextCtrl_GetRange(self.handle(), from, to)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @_wxString {
        unsafe { @wxString(wxTextCtrl_GetStringSelection(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn isMultiLine(&self) -> bool {
        unsafe { wxTextCtrl_IsMultiLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSingleLine(&self) -> bool {
        unsafe { wxTextCtrl_IsSingleLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultStyle(&self, style: @_wxTextAttr) -> bool {
        unsafe { wxTextCtrl_SetDefaultStyle(self.handle(), style.handle()) }
    }
    #[fixed_stack_segment]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, start: c_long, end: c_long, style: @_wxTextAttr) -> bool {
        unsafe { wxTextCtrl_SetStyle(self.handle(), start, end, style.handle()) }
    }
}

struct wxTextDataObject(*u8);
impl _wxTextDataObject for wxTextDataObject {}
impl _wxDataObjectSimple for wxTextDataObject {}
impl _wxDataObject for wxTextDataObject { fn handle(&self) -> *u8 { **self } }

impl wxTextDataObject {
    pub fn from(handle: *u8) -> @_wxTextDataObject {
        @wxTextDataObject(handle) as @_wxTextDataObject
    }
    
}

trait _wxTextDataObject : _wxDataObjectSimple {
}

struct wxTextDropTarget(*u8);
impl _wxTextDropTarget for wxTextDropTarget {}
impl _wxDropTarget for wxTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxTextDropTarget {
    pub fn from(handle: *u8) -> @_wxTextDropTarget {
        @wxTextDropTarget(handle) as @_wxTextDropTarget
    }
    
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
    pub fn from(handle: *u8) -> @_wxTextEntryDialog {
        @wxTextEntryDialog(handle) as @_wxTextEntryDialog
    }
    
}

trait _wxTextEntryDialog : _wxDialog {
}

struct wxTextFile(*u8);
impl _wxTextFile for wxTextFile { fn handle(&self) -> *u8 { **self } }

impl wxTextFile {
    pub fn from(handle: *u8) -> @_wxTextFile {
        @wxTextFile(handle) as @_wxTextFile
    }
    
}

trait _wxTextFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextInputStream(*u8);
impl _wxTextInputStream for wxTextInputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextInputStream {
    pub fn from(handle: *u8) -> @_wxTextInputStream {
        @wxTextInputStream(handle) as @_wxTextInputStream
    }
    
    #[fixed_stack_segment]
    pub fn new(inputStream: @_wxInputStream, sep: @_wxString) -> @_wxTextInputStream {
        unsafe { @wxTextInputStream(wxTextInputStream_Create(inputStream.handle(), sep.handle())) as @_wxTextInputStream }
    }
}

trait _wxTextInputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readLine(&self) -> @_wxString {
        unsafe { @wxString(wxTextInputStream_ReadLine(self.handle())) as @_wxString }
    }
}

struct wxTextOutputStream(*u8);
impl _wxTextOutputStream for wxTextOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextOutputStream {
    pub fn from(handle: *u8) -> @_wxTextOutputStream {
        @wxTextOutputStream(handle) as @_wxTextOutputStream
    }
    
    #[fixed_stack_segment]
    pub fn new(outputStream: @_wxOutputStream, mode: c_int) -> @_wxTextOutputStream {
        unsafe { @wxTextOutputStream(wxTextOutputStream_Create(outputStream.handle(), mode)) as @_wxTextOutputStream }
    }
}

trait _wxTextOutputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, txt: @_wxString) {
        unsafe { wxTextOutputStream_WriteString(self.handle(), txt.handle()) }
    }
}

struct wxTextValidator(*u8);
impl _wxTextValidator for wxTextValidator {}
impl _wxValidator for wxTextValidator {}
impl _wxEvtHandler for wxTextValidator {}
impl _wxObject for wxTextValidator { fn handle(&self) -> *u8 { **self } }

impl wxTextValidator {
    pub fn from(handle: *u8) -> @_wxTextValidator {
        @wxTextValidator(handle) as @_wxTextValidator
    }
    
    #[fixed_stack_segment]
    pub fn new(style: c_int, val: *u8) -> @_wxTextValidator {
        unsafe { @wxTextValidator(wxTextValidator_Create(style, val)) as @_wxTextValidator }
    }
}

trait _wxTextValidator : _wxValidator {
    #[fixed_stack_segment]
    fn getExcludes(&self, _ref: *wchar_t) -> c_int {
        unsafe { wxTextValidator_GetExcludes(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn getIncludes(&self, _ref: *wchar_t) -> c_int {
        unsafe { wxTextValidator_GetIncludes(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn setExcludes(&self, list: *wchar_t, count: c_int) {
        unsafe { wxTextValidator_SetExcludes(self.handle(), list, count) }
    }
    #[fixed_stack_segment]
    fn setIncludes(&self, list: *wchar_t, count: c_int) {
        unsafe { wxTextValidator_SetIncludes(self.handle(), list, count) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @_wxValidator {
        unsafe { @wxValidator(wxTextValidator_Clone(self.handle())) as @_wxValidator }
    }
    #[fixed_stack_segment]
    fn transferToWindow(&self) -> bool {
        unsafe { wxTextValidator_TransferToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn transferFromWindow(&self) -> bool {
        unsafe { wxTextValidator_TransferFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxTextValidator_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn onChar(&self, event: @_wxEvent) {
        unsafe { wxTextValidator_OnChar(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxThinSplitterWindow {
        @wxThinSplitterWindow(handle) as @_wxThinSplitterWindow
    }
    
}

trait _wxThinSplitterWindow : _wxSplitterWindow {
}

struct wxThread(*u8);
impl _wxThread for wxThread { fn handle(&self) -> *u8 { **self } }

impl wxThread {
    pub fn from(handle: *u8) -> @_wxThread {
        @wxThread(handle) as @_wxThread
    }
    
}

trait _wxThread {
    fn handle(&self) -> *u8;
    
}

struct wxTime(*u8);
impl _wxTime for wxTime {}
impl _wxObject for wxTime { fn handle(&self) -> *u8 { **self } }

impl wxTime {
    pub fn from(handle: *u8) -> @_wxTime {
        @wxTime(handle) as @_wxTime
    }
    
}

trait _wxTime : _wxObject {
}

struct wxTimeSpan(*u8);
impl _wxTimeSpan for wxTimeSpan { fn handle(&self) -> *u8 { **self } }

impl wxTimeSpan {
    pub fn from(handle: *u8) -> @_wxTimeSpan {
        @wxTimeSpan(handle) as @_wxTimeSpan
    }
    
}

trait _wxTimeSpan {
    fn handle(&self) -> *u8;
    
}

struct wxTimer(*u8);
impl _wxTimer for wxTimer {}
impl _wxObject for wxTimer { fn handle(&self) -> *u8 { **self } }

impl wxTimer {
    pub fn from(handle: *u8) -> @_wxTimer {
        @wxTimer(handle) as @_wxTimer
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int) -> @_wxTimer {
        unsafe { @wxTimer(wxTimer_Create(_prt.handle(), _id)) as @_wxTimer }
    }
}

trait _wxTimer : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTimer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimer_GetInterval(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOneShot(&self) -> bool {
        unsafe { wxTimer_IsOneShot(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isRuning(&self) -> bool {
        unsafe { wxTimer_IsRuning(self.handle()) }
    }
    #[fixed_stack_segment]
    fn start(&self, _int: c_int, _one: bool) -> bool {
        unsafe { wxTimer_Start(self.handle(), _int, _one) }
    }
    #[fixed_stack_segment]
    fn stop(&self) {
        unsafe { wxTimer_Stop(self.handle()) }
    }
}

struct wxTimerBase(*u8);
impl _wxTimerBase for wxTimerBase {}
impl _wxObject for wxTimerBase { fn handle(&self) -> *u8 { **self } }

impl wxTimerBase {
    pub fn from(handle: *u8) -> @_wxTimerBase {
        @wxTimerBase(handle) as @_wxTimerBase
    }
    
}

trait _wxTimerBase : _wxObject {
}

struct wxTimerEvent(*u8);
impl _wxTimerEvent for wxTimerEvent {}
impl _wxEvent for wxTimerEvent {}
impl _wxObject for wxTimerEvent { fn handle(&self) -> *u8 { **self } }

impl wxTimerEvent {
    pub fn from(handle: *u8) -> @_wxTimerEvent {
        @wxTimerEvent(handle) as @_wxTimerEvent
    }
    
}

trait _wxTimerEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.handle()) }
    }
}

struct wxTimerEx(*u8);
impl _wxTimerEx for wxTimerEx {}
impl _wxTimer for wxTimerEx {}
impl _wxObject for wxTimerEx { fn handle(&self) -> *u8 { **self } }

impl wxTimerEx {
    pub fn from(handle: *u8) -> @_wxTimerEx {
        @wxTimerEx(handle) as @_wxTimerEx
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxTimerEx {
        unsafe { @wxTimerEx(wxTimerEx_Create()) as @_wxTimerEx }
    }
}

trait _wxTimerEx : _wxTimer {
    #[fixed_stack_segment]
    fn connect(&self, closure: @_wxClosure) {
        unsafe { wxTimerEx_Connect(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self) -> @_wxClosure {
        unsafe { @wxClosure(wxTimerEx_GetClosure(self.handle())) as @_wxClosure }
    }
}

struct wxTimerRunner(*u8);
impl _wxTimerRunner for wxTimerRunner { fn handle(&self) -> *u8 { **self } }

impl wxTimerRunner {
    pub fn from(handle: *u8) -> @_wxTimerRunner {
        @wxTimerRunner(handle) as @_wxTimerRunner
    }
    
}

trait _wxTimerRunner {
    fn handle(&self) -> *u8;
    
}

struct wxTipProvider(*u8);
impl _wxTipProvider for wxTipProvider { fn handle(&self) -> *u8 { **self } }

impl wxTipProvider {
    pub fn from(handle: *u8) -> @_wxTipProvider {
        @wxTipProvider(handle) as @_wxTipProvider
    }
    
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
    pub fn from(handle: *u8) -> @_wxTipWindow {
        @wxTipWindow(handle) as @_wxTipWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, text: @_wxString, maxLength: c_int) -> @_wxTipWindow {
        unsafe { @wxTipWindow(wxTipWindow_Create(parent.handle(), text.handle(), maxLength)) as @_wxTipWindow }
    }
}

trait _wxTipWindow : _wxPopupTransientWindow {
    #[fixed_stack_segment]
    fn close(&self) {
        unsafe { wxTipWindow_Close(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxToggleButton {
        @wxToggleButton(handle) as @_wxToggleButton
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, id: c_int, label: @_wxString, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @_wxToggleButton {
        unsafe { @wxToggleButton(wxToggleButton_Create(parent.handle(), id, label.handle(), x, y, w, h, style)) as @_wxToggleButton }
    }
}

trait _wxToggleButton : _wxControl {
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> bool {
        unsafe { wxToggleButton_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxToggleButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, label: @_wxString) {
        unsafe { wxToggleButton_SetLabel(self.handle(), label.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxToolBar {
        @wxToolBar(handle) as @_wxToolBar
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxToolBar {
        unsafe { @wxToolBar(wxToolBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxToolBar }
    }
}

trait _wxToolBar : _wxToolBarBase {
    #[fixed_stack_segment]
    fn addControl(&self, ctrl: @_wxControl) -> bool {
        unsafe { wxToolBar_AddControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn addTool(&self, id: c_int, bmp: @_wxBitmap, shelp: @_wxString, lhelp: @_wxString) {
        unsafe { wxToolBar_AddTool(self.handle(), id, bmp.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn addToolEx(&self, id: c_int, bmp1: @_wxBitmap, bmp2: @_wxBitmap, isToggle: bool, x: c_int, y: c_int, data: @_wxObject, shelp: @_wxString, lhelp: @_wxString) {
        unsafe { wxToolBar_AddToolEx(self.handle(), id, bmp1.handle(), bmp2.handle(), isToggle, x, y, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxToolBar_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteTool(&self, id: c_int) -> bool {
        unsafe { wxToolBar_DeleteTool(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn deleteToolByPos(&self, pos: c_int) -> bool {
        unsafe { wxToolBar_DeleteToolByPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn enableTool(&self, id: c_int, enable: bool) {
        unsafe { wxToolBar_EnableTool(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    fn getMargins(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxToolBar_GetMargins(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getToolBitmapSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxToolBar_GetToolBitmapSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getToolClientData(&self, id: c_int) -> @_wxObject {
        unsafe { @wxObject(wxToolBar_GetToolClientData(self.handle(), id)) as @_wxObject }
    }
    #[fixed_stack_segment]
    fn getToolEnabled(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn getToolLongHelp(&self, id: c_int) -> @_wxString {
        unsafe { @wxString(wxToolBar_GetToolLongHelp(self.handle(), id)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolShortHelp(&self, id: c_int) -> @_wxString {
        unsafe { @wxString(wxToolBar_GetToolShortHelp(self.handle(), id)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getToolSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxToolBar_GetToolSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getToolState(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolState(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn insertControl(&self, pos: c_int, ctrl: @_wxControl) {
        unsafe { wxToolBar_InsertControl(self.handle(), pos, ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn insertTool(&self, pos: c_int, id: c_int, bmp1: @_wxBitmap, bmp2: @_wxBitmap, isToggle: bool, data: @_wxObject, shelp: @_wxString, lhelp: @_wxString) {
        unsafe { wxToolBar_InsertTool(self.handle(), pos, id, bmp1.handle(), bmp2.handle(), isToggle, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn realize(&self) -> bool {
        unsafe { wxToolBar_Realize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn removeTool(&self, id: c_int) {
        unsafe { wxToolBar_RemoveTool(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetMargins(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setToolBitmapSize(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetToolBitmapSize(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setToolClientData(&self, id: c_int, data: @_wxObject) {
        unsafe { wxToolBar_SetToolClientData(self.handle(), id, data.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolLongHelp(&self, id: c_int, str: @_wxString) {
        unsafe { wxToolBar_SetToolLongHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolPacking(&self, packing: c_int) {
        unsafe { wxToolBar_SetToolPacking(self.handle(), packing) }
    }
    #[fixed_stack_segment]
    fn setToolSeparation(&self, separation: c_int) {
        unsafe { wxToolBar_SetToolSeparation(self.handle(), separation) }
    }
    #[fixed_stack_segment]
    fn setToolShortHelp(&self, id: c_int, str: @_wxString) {
        unsafe { wxToolBar_SetToolShortHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    fn toggleTool(&self, id: c_int, toggle: bool) {
        unsafe { wxToolBar_ToggleTool(self.handle(), id, toggle) }
    }
    #[fixed_stack_segment]
    fn addTool2(&self, toolId: c_int, label: @_wxString, bmp: @_wxBitmap, bmpDisabled: @_wxBitmap, itemKind: c_int, shortHelp: @_wxString, longHelp: @_wxString) {
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
    pub fn from(handle: *u8) -> @_wxToolBarBase {
        @wxToolBarBase(handle) as @_wxToolBarBase
    }
    
}

trait _wxToolBarBase : _wxControl {
}

struct wxToolLayoutItem(*u8);
impl _wxToolLayoutItem for wxToolLayoutItem {}
impl _wxObject for wxToolLayoutItem { fn handle(&self) -> *u8 { **self } }

impl wxToolLayoutItem {
    pub fn from(handle: *u8) -> @_wxToolLayoutItem {
        @wxToolLayoutItem(handle) as @_wxToolLayoutItem
    }
    
}

trait _wxToolLayoutItem : _wxObject {
}

struct wxToolTip(*u8);
impl _wxToolTip for wxToolTip {}
impl _wxObject for wxToolTip { fn handle(&self) -> *u8 { **self } }

impl wxToolTip {
    pub fn from(handle: *u8) -> @_wxToolTip {
        @wxToolTip(handle) as @_wxToolTip
    }
    
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
    pub fn from(handle: *u8) -> @_wxToolWindow {
        @wxToolWindow(handle) as @_wxToolWindow
    }
    
}

trait _wxToolWindow : _wxFrame {
}

struct wxTopLevelWindow(*u8);
impl _wxTopLevelWindow for wxTopLevelWindow {}
impl _wxWindow for wxTopLevelWindow {}
impl _wxEvtHandler for wxTopLevelWindow {}
impl _wxObject for wxTopLevelWindow { fn handle(&self) -> *u8 { **self } }

impl wxTopLevelWindow {
    pub fn from(handle: *u8) -> @_wxTopLevelWindow {
        @wxTopLevelWindow(handle) as @_wxTopLevelWindow
    }
    
}

trait _wxTopLevelWindow : _wxWindow {
    #[fixed_stack_segment]
    fn enableCloseButton(&self, enable: bool) -> bool {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getDefaultButton(&self) -> @_wxButton {
        unsafe { @wxButton(wxTopLevelWindow_GetDefaultButton(self.handle())) as @_wxButton }
    }
    #[fixed_stack_segment]
    fn getDefaultItem(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxTopLevelWindow_GetDefaultItem(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getIcon(&self) -> @_wxIcon {
        unsafe { @wxIcon(wxTopLevelWindow_GetIcon(self.handle())) as @_wxIcon }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @_wxString {
        unsafe { @wxString(wxTopLevelWindow_GetTitle(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn iconize(&self, iconize: bool) -> bool {
        unsafe { wxTopLevelWindow_Iconize(self.handle(), iconize) }
    }
    #[fixed_stack_segment]
    fn isActive(&self) -> bool {
        unsafe { wxTopLevelWindow_IsActive(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isIconized(&self) -> bool {
        unsafe { wxTopLevelWindow_IsIconized(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isMaximized(&self) -> bool {
        unsafe { wxTopLevelWindow_IsMaximized(self.handle()) }
    }
    #[fixed_stack_segment]
    fn maximize(&self, maximize: bool) {
        unsafe { wxTopLevelWindow_Maximize(self.handle(), maximize) }
    }
    #[fixed_stack_segment]
    fn requestUserAttention(&self, flags: c_int) {
        unsafe { wxTopLevelWindow_RequestUserAttention(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn setDefaultButton(&self, pBut: @_wxButton) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultItem(&self, pBut: @_wxWindow) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, pIcon: @_wxIcon) {
        unsafe { wxTopLevelWindow_SetIcon(self.handle(), pIcon.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcons(&self, _icons: *u8) {
        unsafe { wxTopLevelWindow_SetIcons(self.handle(), _icons) }
    }
    #[fixed_stack_segment]
    fn setMaxSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMaxSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMinSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, pString: @_wxString) {
        unsafe { wxTopLevelWindow_SetTitle(self.handle(), pString.handle()) }
    }
}

struct wxTreeCompanionWindow(*u8);
impl _wxTreeCompanionWindow for wxTreeCompanionWindow {}
impl _wxWindow for wxTreeCompanionWindow {}
impl _wxEvtHandler for wxTreeCompanionWindow {}
impl _wxObject for wxTreeCompanionWindow { fn handle(&self) -> *u8 { **self } }

impl wxTreeCompanionWindow {
    pub fn from(handle: *u8) -> @_wxTreeCompanionWindow {
        @wxTreeCompanionWindow(handle) as @_wxTreeCompanionWindow
    }
    
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
    pub fn from(handle: *u8) -> @_wxTreeCtrl {
        @wxTreeCtrl(handle) as @_wxTreeCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _cmp: *u8, _prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create(_obj, _cmp, _prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxTreeCtrl }
    }
    #[fixed_stack_segment]
    pub fn new2(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create2(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxTreeCtrl }
    }
}

trait _wxTreeCtrl : _wxControl {
    #[fixed_stack_segment]
    fn addRoot(&self, text: @_wxString, image: c_int, selectedImage: c_int, data: @_wxTreeItemData, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_AddRoot(self.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, parent: @_wxTreeItemId, text: @_wxString, image: c_int, selectedImage: c_int, data: @_wxTreeItemData, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_AppendItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapse(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_Collapse(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapseAndReset(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_Delete(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteChildren(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_DeleteChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn editLabel(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_EditLabel(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn endEditLabel(&self, item: @_wxTreeItemId, discardChanges: bool) {
        unsafe { wxTreeCtrl_EndEditLabel(self.handle(), item.handle(), discardChanges) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_EnsureVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn expand(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_Expand(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getBoundingRect(&self, item: @_wxTreeItemId, textOnly: bool) -> @_wxRect {
        unsafe { @wxRect(wxTreeCtrl_GetBoundingRect(self.handle(), item.handle(), textOnly)) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn getChildrenCount(&self, item: @_wxTreeItemId, recursively: bool) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.handle(), item.handle(), recursively) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> @_wxTextCtrl {
        unsafe { @wxTextCtrl(wxTreeCtrl_GetEditControl(self.handle())) as @_wxTextCtrl }
    }
    #[fixed_stack_segment]
    fn getFirstChild(&self, item: @_wxTreeItemId, cookie: *c_int, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetFirstChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleItem(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> @_wxImageList {
        unsafe { @wxImageList(wxTreeCtrl_GetImageList(self.handle())) as @_wxImageList }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemData(&self, item: @_wxTreeItemId) -> *u8 {
        unsafe { wxTreeCtrl_GetItemData(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemImage(&self, item: @_wxTreeItemId, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.handle(), item.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: @_wxTreeItemId) -> @_wxString {
        unsafe { @wxString(wxTreeCtrl_GetItemText(self.handle(), item.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getLastChild(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetLastChild(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextChild(&self, item: @_wxTreeItemId, cookie: *c_int, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextSibling(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextVisible(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetParent(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevSibling(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevVisible(&self, item: @_wxTreeItemId, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getRootItem(&self, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetRootItem(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_GetSelection(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelections(&self, selections: *intptr_t) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self.handle(), selections) }
    }
    #[fixed_stack_segment]
    fn getSpacing(&self) -> c_int {
        unsafe { wxTreeCtrl_GetSpacing(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStateImageList(&self) -> @_wxImageList {
        unsafe { @wxImageList(wxTreeCtrl_GetStateImageList(self.handle())) as @_wxImageList }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, _x: c_int, _y: c_int, flags: *c_int, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_HitTest(self.handle(), _x, _y, flags, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, parent: @_wxTreeItemId, idPrevious: @_wxTreeItemId, text: @_wxString, image: c_int, selectedImage: c_int, data: *u8, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItem(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex(&self, parent: @_wxTreeItemId, index: c_int, text: @_wxString, image: c_int, selectedImage: c_int, data: *u8, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn isBold(&self, item: @_wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsBold(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isExpanded(&self, item: @_wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsExpanded(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isSelected(&self, item: @_wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsSelected(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self, item: @_wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn itemHasChildren(&self, item: @_wxTreeItemId) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn onCompareItems(&self, item1: @_wxTreeItemId, item2: @_wxTreeItemId) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.handle(), item1.handle(), item2.handle()) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, parent: @_wxTreeItemId, text: @_wxString, image: c_int, selectedImage: c_int, data: *u8, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_PrependItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn scrollTo(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_ScrollTo(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn selectItem(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_SelectItem(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @_wxImageList) {
        unsafe { wxTreeCtrl_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.handle(), indent) }
    }
    #[fixed_stack_segment]
    fn setItemBackgroundColour(&self, item: @_wxTreeItemId, col: @_wxColour) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBold(&self, item: @_wxTreeItemId, bold: bool) {
        unsafe { wxTreeCtrl_SetItemBold(self.handle(), item.handle(), bold) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: @_wxTreeItemId, data: *u8) {
        unsafe { wxTreeCtrl_SetItemData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setItemDropHighlight(&self, item: @_wxTreeItemId, highlight: bool) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.handle(), item.handle(), highlight) }
    }
    #[fixed_stack_segment]
    fn setItemFont(&self, item: @_wxTreeItemId, font: @_wxFont) {
        unsafe { wxTreeCtrl_SetItemFont(self.handle(), item.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemHasChildren(&self, item: @_wxTreeItemId, hasChildren: bool) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.handle(), item.handle(), hasChildren) }
    }
    #[fixed_stack_segment]
    fn setItemImage(&self, item: @_wxTreeItemId, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.handle(), item.handle(), image, which) }
    }
    #[fixed_stack_segment]
    fn setItemText(&self, item: @_wxTreeItemId, text: @_wxString) {
        unsafe { wxTreeCtrl_SetItemText(self.handle(), item.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemTextColour(&self, item: @_wxTreeItemId, col: @_wxColour) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.handle(), spacing) }
    }
    #[fixed_stack_segment]
    fn setStateImageList(&self, imageList: @_wxImageList) {
        unsafe { wxTreeCtrl_SetStateImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn sortChildren(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_SortChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn toggle(&self, item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_Toggle(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self.handle()) }
    }
    #[fixed_stack_segment]
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem2(&self, parent: @_wxWindow, idPrevious: @_wxTreeItemId, text: @_wxString, image: c_int, selectedImage: c_int, closure: @_wxClosure, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItem2(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex2(&self, parent: @_wxWindow, index: c_int, text: @_wxString, image: c_int, selectedImage: c_int, closure: @_wxClosure, _item: @_wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemClientClosure(&self, item: @_wxTreeItemId) -> @_wxClosure {
        unsafe { @wxClosure(wxTreeCtrl_GetItemClientClosure(self.handle(), item.handle())) as @_wxClosure }
    }
    #[fixed_stack_segment]
    fn setItemClientClosure(&self, item: @_wxTreeItemId, closure: @_wxClosure) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.handle(), item.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @_wxImageList) {
        unsafe { wxTreeCtrl_AssignImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn assignStateImageList(&self, imageList: @_wxImageList) {
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
    pub fn from(handle: *u8) -> @_wxTreeEvent {
        @wxTreeEvent(handle) as @_wxTreeEvent
    }
    
}

trait _wxTreeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, _ref: @_wxTreeItemId) {
        unsafe { wxTreeEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @_wxString {
        unsafe { @wxString(wxTreeEvent_GetLabel(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getOldItem(&self, _ref: @_wxTreeItemId) {
        unsafe { wxTreeEvent_GetOldItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxTreeEvent_GetPoint(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getKeyEvent(&self) -> @_wxKeyEvent {
        unsafe { @wxKeyEvent(wxTreeEvent_GetKeyEvent(self.handle())) as @_wxKeyEvent }
    }
    #[fixed_stack_segment]
    fn isEditCancelled(&self) -> c_int {
        unsafe { wxTreeEvent_IsEditCancelled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn allow(&self) {
        unsafe { wxTreeEvent_Allow(self.handle()) }
    }
}

struct wxTreeItemData(*u8);
impl _wxTreeItemData for wxTreeItemData {}
impl _wxClientData for wxTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemData {
    pub fn from(handle: *u8) -> @_wxTreeItemData {
        @wxTreeItemData(handle) as @_wxTreeItemData
    }
    
}

trait _wxTreeItemData : _wxClientData {
}

struct wxTreeItemId(*u8);
impl _wxTreeItemId for wxTreeItemId { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemId {
    pub fn from(handle: *u8) -> @_wxTreeItemId {
        @wxTreeItemId(handle) as @_wxTreeItemId
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxTreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_Create()) as @_wxTreeItemId }
    }
    #[fixed_stack_segment]
    pub fn newFromValue(value: intptr_t) -> @_wxTreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_CreateFromValue(value)) as @_wxTreeItemId }
    }
}

trait _wxTreeItemId {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxTreeItemId_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @_wxTreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_Clone(self.handle())) as @_wxTreeItemId }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.handle()) }
    }
}

struct wxTreeLayout(*u8);
impl _wxTreeLayout for wxTreeLayout {}
impl _wxObject for wxTreeLayout { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayout {
    pub fn from(handle: *u8) -> @_wxTreeLayout {
        @wxTreeLayout(handle) as @_wxTreeLayout
    }
    
}

trait _wxTreeLayout : _wxObject {
}

struct wxTreeLayoutStored(*u8);
impl _wxTreeLayoutStored for wxTreeLayoutStored {}
impl _wxTreeLayout for wxTreeLayoutStored {}
impl _wxObject for wxTreeLayoutStored { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayoutStored {
    pub fn from(handle: *u8) -> @_wxTreeLayoutStored {
        @wxTreeLayoutStored(handle) as @_wxTreeLayoutStored
    }
    
}

trait _wxTreeLayoutStored : _wxTreeLayout {
}

struct wxURL(*u8);
impl _wxURL for wxURL {}
impl _wxObject for wxURL { fn handle(&self) -> *u8 { **self } }

impl wxURL {
    pub fn from(handle: *u8) -> @_wxURL {
        @wxURL(handle) as @_wxURL
    }
    
}

trait _wxURL : _wxObject {
}

struct wxUpdateUIEvent(*u8);
impl _wxUpdateUIEvent for wxUpdateUIEvent {}
impl _wxEvent for wxUpdateUIEvent {}
impl _wxObject for wxUpdateUIEvent { fn handle(&self) -> *u8 { **self } }

impl wxUpdateUIEvent {
    pub fn from(handle: *u8) -> @_wxUpdateUIEvent {
        @wxUpdateUIEvent(handle) as @_wxUpdateUIEvent
    }
    
}

trait _wxUpdateUIEvent : _wxEvent {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxUpdateUIEvent_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @_wxObject) {
        unsafe { wxUpdateUIEvent_CopyObject(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) {
        unsafe { wxUpdateUIEvent_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getChecked(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEnabled(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSetChecked(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetChecked(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSetEnabled(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSetText(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetText(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @_wxString {
        unsafe { @wxString(wxUpdateUIEvent_GetText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @_wxString) {
        unsafe { wxUpdateUIEvent_SetText(self.handle(), text.handle()) }
    }
}

struct wxValidator(*u8);
impl _wxValidator for wxValidator {}
impl _wxEvtHandler for wxValidator {}
impl _wxObject for wxValidator { fn handle(&self) -> *u8 { **self } }

impl wxValidator {
    pub fn from(handle: *u8) -> @_wxValidator {
        @wxValidator(handle) as @_wxValidator
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxValidator {
        unsafe { @wxValidator(wxValidator_Create()) as @_wxValidator }
    }
    #[fixed_stack_segment]
    pub fn setBellOnError(doIt: bool) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

trait _wxValidator : _wxEvtHandler {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxValidator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxValidator_GetWindow(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, win: @_wxWindow) {
        unsafe { wxValidator_SetWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn transferFromWindow(&self) -> bool {
        unsafe { wxValidator_TransferFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn transferToWindow(&self) -> bool {
        unsafe { wxValidator_TransferToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn validate(&self, parent: @_wxWindow) -> bool {
        unsafe { wxValidator_Validate(self.handle(), parent.handle()) }
    }
}

struct wxVariant(*u8);
impl _wxVariant for wxVariant {}
impl _wxObject for wxVariant { fn handle(&self) -> *u8 { **self } }

impl wxVariant {
    pub fn from(handle: *u8) -> @_wxVariant {
        @wxVariant(handle) as @_wxVariant
    }
    
}

trait _wxVariant : _wxObject {
}

struct wxVariantData(*u8);
impl _wxVariantData for wxVariantData {}
impl _wxObject for wxVariantData { fn handle(&self) -> *u8 { **self } }

impl wxVariantData {
    pub fn from(handle: *u8) -> @_wxVariantData {
        @wxVariantData(handle) as @_wxVariantData
    }
    
}

trait _wxVariantData : _wxObject {
}

struct wxView(*u8);
impl _wxView for wxView {}
impl _wxEvtHandler for wxView {}
impl _wxObject for wxView { fn handle(&self) -> *u8 { **self } }

impl wxView {
    pub fn from(handle: *u8) -> @_wxView {
        @wxView(handle) as @_wxView
    }
    
}

trait _wxView : _wxEvtHandler {
}

struct wxSound(*u8);
impl _wxSound for wxSound {}
impl _wxEvtHandler for wxSound {}
impl _wxObject for wxSound { fn handle(&self) -> *u8 { **self } }

impl wxSound {
    pub fn from(handle: *u8) -> @_wxSound {
        @wxSound(handle) as @_wxSound
    }
    
    #[fixed_stack_segment]
    pub fn new(fileName: @_wxString, isResource: bool) -> @_wxSound {
        unsafe { @wxSound(wxSound_Create(fileName.handle(), isResource)) as @_wxSound }
    }
}

trait _wxSound : _wxEvtHandler {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSound_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxSound_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn play(&self, flag: c_int) -> bool {
        unsafe { wxSound_Play(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn stop(&self) {
        unsafe { wxSound_Stop(self.handle()) }
    }
}

struct wxWindow(*u8);
impl _wxWindow for wxWindow {}
impl _wxEvtHandler for wxWindow {}
impl _wxObject for wxWindow { fn handle(&self) -> *u8 { **self } }

impl wxWindow {
    pub fn from(handle: *u8) -> @_wxWindow {
        @wxWindow(handle) as @_wxWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @_wxWindow {
        unsafe { @wxWindow(wxWindow_Create(_prt.handle(), _id, _x, _y, _w, _h, _stl)) as @_wxWindow }
    }
}

trait _wxWindow : _wxEvtHandler {
    #[fixed_stack_segment]
    fn addChild(&self, child: @_wxWindow) {
        unsafe { wxWindow_AddChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn addConstraintReference(&self, otherWin: @_wxWindow) {
        unsafe { wxWindow_AddConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    fn captureMouse(&self) {
        unsafe { wxWindow_CaptureMouse(self.handle()) }
    }
    #[fixed_stack_segment]
    fn center(&self, direction: c_int) {
        unsafe { wxWindow_Center(self.handle(), direction) }
    }
    #[fixed_stack_segment]
    fn centerOnParent(&self, dir: c_int) {
        unsafe { wxWindow_CenterOnParent(self.handle(), dir) }
    }
    #[fixed_stack_segment]
    fn clearBackground(&self) {
        unsafe { wxWindow_ClearBackground(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clientToScreen(&self, x: c_int, y: c_int) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ClientToScreen(self.handle(), x, y)) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn close(&self, _force: bool) -> bool {
        unsafe { wxWindow_Close(self.handle(), _force) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixels(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertDialogToPixels(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialog(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertPixelsToDialog(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn deleteRelatedConstraints(&self) {
        unsafe { wxWindow_DeleteRelatedConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) -> bool {
        unsafe { wxWindow_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn destroyChildren(&self) -> bool {
        unsafe { wxWindow_DestroyChildren(self.handle()) }
    }
    #[fixed_stack_segment]
    fn disable(&self) -> bool {
        unsafe { wxWindow_Disable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn doPhase(&self, phase: c_int) -> c_int {
        unsafe { wxWindow_DoPhase(self.handle(), phase) }
    }
    #[fixed_stack_segment]
    fn enable(&self) -> bool {
        unsafe { wxWindow_Enable(self.handle()) }
    }
    #[fixed_stack_segment]
    fn findFocus(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxWindow_FindFocus(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn findWindow(&self, name: @_wxString) -> @_wxWindow {
        unsafe { @wxWindow(wxWindow_FindWindow(self.handle(), name.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn fit(&self) {
        unsafe { wxWindow_Fit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn fitInside(&self) {
        unsafe { wxWindow_FitInside(self.handle()) }
    }
    #[fixed_stack_segment]
    fn freeze(&self) {
        unsafe { wxWindow_Freeze(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEffectiveMinSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxWindow_GetEffectiveMinSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @_wxColour) {
        unsafe { wxWindow_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxWindow_GetBestSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getCaret(&self) -> @_wxCaret {
        unsafe { @wxCaret(wxWindow_GetCaret(self.handle())) as @_wxCaret }
    }
    #[fixed_stack_segment]
    fn getCharHeight(&self) -> c_int {
        unsafe { wxWindow_GetCharHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCharWidth(&self) -> c_int {
        unsafe { wxWindow_GetCharWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
        unsafe { wxWindow_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @_wxClientData {
        unsafe { @wxClientData(wxWindow_GetClientData(self.handle())) as @_wxClientData }
    }
    #[fixed_stack_segment]
    fn getClientSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxWindow_GetClientSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getConstraints(&self) -> @_wxLayoutConstraints {
        unsafe { @wxLayoutConstraints(wxWindow_GetConstraints(self.handle())) as @_wxLayoutConstraints }
    }
    #[fixed_stack_segment]
    fn getConstraintsInvolvedIn(&self) -> *u8 {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCursor(&self) -> @_wxCursor {
        unsafe { @wxCursor(wxWindow_GetCursor(self.handle())) as @_wxCursor }
    }
    #[fixed_stack_segment]
    fn getDropTarget(&self) -> @_wxDropTarget {
        unsafe { @wxDropTarget(wxWindow_GetDropTarget(self.handle())) as @_wxDropTarget }
    }
    #[fixed_stack_segment]
    fn getEventHandler(&self) -> @_wxEvtHandler {
        unsafe { @wxEvtHandler(wxWindow_GetEventHandler(self.handle())) as @_wxEvtHandler }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @_wxFont) {
        unsafe { wxWindow_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getForegroundColour(&self, _ref: @_wxColour) {
        unsafe { wxWindow_GetForegroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHandle(&self) -> *u8 {
        unsafe { wxWindow_GetHandle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxWindow_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @_wxString {
        unsafe { @wxString(wxWindow_GetLabel(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getLabelEmpty(&self) -> c_int {
        unsafe { wxWindow_GetLabelEmpty(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaxHeight(&self) -> c_int {
        unsafe { wxWindow_GetMaxHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaxWidth(&self) -> c_int {
        unsafe { wxWindow_GetMaxWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinHeight(&self) -> c_int {
        unsafe { wxWindow_GetMinHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMinWidth(&self) -> c_int {
        unsafe { wxWindow_GetMinWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @_wxString {
        unsafe { @wxString(wxWindow_GetName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxWindow_GetParent(self.handle())) as @_wxWindow }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_GetPosition(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @_wxRect {
        unsafe { @wxRect(wxWindow_GetRect(self.handle())) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn getScrollPos(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollPos(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn getScrollRange(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollRange(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn getScrollThumb(&self, orient: c_int) -> c_int {
        unsafe { wxWindow_GetScrollThumb(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxWindow_GetSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @_wxSizer {
        unsafe { @wxSizer(wxWindow_GetSizer(self.handle())) as @_wxSizer }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @_wxString, x: *c_int, y: *c_int, descent: *c_int, externalLeading: *c_int, theFont: @_wxFont) {
        unsafe { wxWindow_GetTextExtent(self.handle(), string.handle(), x, y, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolTip(&self) -> @_wxString {
        unsafe { @wxString(wxWindow_GetToolTip(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getUpdateRegion(&self) -> @_wxRegion {
        unsafe { @wxRegion(wxWindow_GetUpdateRegion(self.handle())) as @_wxRegion }
    }
    #[fixed_stack_segment]
    fn getValidator(&self) -> @_wxValidator {
        unsafe { @wxValidator(wxWindow_GetValidator(self.handle())) as @_wxValidator }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxWindow_GetVirtualSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getWindowStyleFlag(&self) -> c_int {
        unsafe { wxWindow_GetWindowStyleFlag(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasFlag(&self, flag: c_int) -> bool {
        unsafe { wxWindow_HasFlag(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn hide(&self) -> bool {
        unsafe { wxWindow_Hide(self.handle()) }
    }
    #[fixed_stack_segment]
    fn initDialog(&self) {
        unsafe { wxWindow_InitDialog(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isBeingDeleted(&self) -> bool {
        unsafe { wxWindow_IsBeingDeleted(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self) -> bool {
        unsafe { wxWindow_IsEnabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { wxWindow_IsExposed(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn isShown(&self) -> bool {
        unsafe { wxWindow_IsShown(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isTopLevel(&self) -> bool {
        unsafe { wxWindow_IsTopLevel(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layout(&self) -> c_int {
        unsafe { wxWindow_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layoutPhase1(&self, noChanges: *c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase1(self.handle(), noChanges) }
    }
    #[fixed_stack_segment]
    fn layoutPhase2(&self, noChanges: *c_int) -> c_int {
        unsafe { wxWindow_LayoutPhase2(self.handle(), noChanges) }
    }
    #[fixed_stack_segment]
    fn lower(&self) {
        unsafe { wxWindow_Lower(self.handle()) }
    }
    #[fixed_stack_segment]
    fn makeModal(&self, modal: bool) {
        unsafe { wxWindow_MakeModal(self.handle(), modal) }
    }
    #[fixed_stack_segment]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_Move(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn moveConstraint(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_MoveConstraint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn popEventHandler(&self, deleteHandler: bool) -> *u8 {
        unsafe { wxWindow_PopEventHandler(self.handle(), deleteHandler) }
    }
    #[fixed_stack_segment]
    fn popupMenu(&self, menu: @_wxMenu, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.handle(), menu.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @_wxDC) {
        unsafe { wxWindow_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn pushEventHandler(&self, handler: @_wxEvtHandler) {
        unsafe { wxWindow_PushEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn raise(&self) {
        unsafe { wxWindow_Raise(self.handle()) }
    }
    #[fixed_stack_segment]
    fn refresh(&self, eraseBackground: bool) {
        unsafe { wxWindow_Refresh(self.handle(), eraseBackground) }
    }
    #[fixed_stack_segment]
    fn refreshRect(&self, eraseBackground: bool, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_RefreshRect(self.handle(), eraseBackground, x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn releaseMouse(&self) {
        unsafe { wxWindow_ReleaseMouse(self.handle()) }
    }
    #[fixed_stack_segment]
    fn removeChild(&self, child: @_wxWindow) {
        unsafe { wxWindow_RemoveChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn removeConstraintReference(&self, otherWin: @_wxWindow) {
        unsafe { wxWindow_RemoveConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    fn reparent(&self, _par: @_wxWindow) -> c_int {
        unsafe { wxWindow_Reparent(self.handle(), _par.handle()) }
    }
    #[fixed_stack_segment]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    fn screenToClient(&self, x: c_int, y: c_int) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ScreenToClient(self.handle(), x, y)) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self.handle(), dx, dy, x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn setAcceleratorTable(&self, accel: @_wxAcceleratorTable) {
        unsafe { wxWindow_SetAcceleratorTable(self.handle(), accel.handle()) }
    }
    #[fixed_stack_segment]
    fn setAutoLayout(&self, autoLayout: bool) {
        unsafe { wxWindow_SetAutoLayout(self.handle(), autoLayout) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @_wxColour) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaret(&self, caret: @_wxCaret) {
        unsafe { wxWindow_SetCaret(self.handle(), caret.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, data: @_wxClientData) {
        unsafe { wxWindow_SetClientData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, data: @_wxClientData) {
        unsafe { wxWindow_SetClientObject(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setConstraintSizes(&self, recurse: c_int) {
        unsafe { wxWindow_SetConstraintSizes(self.handle(), recurse) }
    }
    #[fixed_stack_segment]
    fn setConstraints(&self, constraints: @_wxLayoutConstraints) {
        unsafe { wxWindow_SetConstraints(self.handle(), constraints.handle()) }
    }
    #[fixed_stack_segment]
    fn setCursor(&self, cursor: @_wxCursor) -> c_int {
        unsafe { wxWindow_SetCursor(self.handle(), cursor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDropTarget(&self, dropTarget: @_wxDropTarget) {
        unsafe { wxWindow_SetDropTarget(self.handle(), dropTarget.handle()) }
    }
    #[fixed_stack_segment]
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self.handle(), exStyle) }
    }
    #[fixed_stack_segment]
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont) -> c_int {
        unsafe { wxWindow_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, colour: @_wxColour) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.handle(), _id) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, _title: @_wxString) {
        unsafe { wxWindow_SetLabel(self.handle(), _title.handle()) }
    }
    #[fixed_stack_segment]
    fn setName(&self, _name: @_wxString) {
        unsafe { wxWindow_SetName(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn setScrollPos(&self, orient: c_int, pos: c_int, refresh: bool) {
        unsafe { wxWindow_SetScrollPos(self.handle(), orient, pos, refresh) }
    }
    #[fixed_stack_segment]
    fn setScrollbar(&self, orient: c_int, pos: c_int, thumbVisible: c_int, range: c_int, refresh: bool) {
        unsafe { wxWindow_SetScrollbar(self.handle(), orient, pos, thumbVisible, range, refresh) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, width: c_int, height: c_int, sizeFlags: c_int) {
        unsafe { wxWindow_SetSize(self.handle(), x, y, width, height, sizeFlags) }
    }
    #[fixed_stack_segment]
    fn setSizeConstraint(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_SetSizeConstraint(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn setSizeHints(&self, minW: c_int, minH: c_int, maxW: c_int, maxH: c_int, incW: c_int, incH: c_int) {
        unsafe { wxWindow_SetSizeHints(self.handle(), minW, minH, maxW, maxH, incW, incH) }
    }
    #[fixed_stack_segment]
    fn setSizer(&self, sizer: @_wxSizer) {
        unsafe { wxWindow_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolTip(&self, tip: @_wxString) {
        unsafe { wxWindow_SetToolTip(self.handle(), tip.handle()) }
    }
    #[fixed_stack_segment]
    fn setValidator(&self, validator: @_wxValidator) {
        unsafe { wxWindow_SetValidator(self.handle(), validator.handle()) }
    }
    #[fixed_stack_segment]
    fn setWindowStyleFlag(&self, style: c_long) {
        unsafe { wxWindow_SetWindowStyleFlag(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn show(&self) -> bool {
        unsafe { wxWindow_Show(self.handle()) }
    }
    #[fixed_stack_segment]
    fn thaw(&self) {
        unsafe { wxWindow_Thaw(self.handle()) }
    }
    #[fixed_stack_segment]
    fn transferDataFromWindow(&self) -> bool {
        unsafe { wxWindow_TransferDataFromWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn transferDataToWindow(&self) -> bool {
        unsafe { wxWindow_TransferDataToWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn unsetConstraints(&self, c: *u8) {
        unsafe { wxWindow_UnsetConstraints(self.handle(), c) }
    }
    #[fixed_stack_segment]
    fn updateWindowUI(&self) {
        unsafe { wxWindow_UpdateWindowUI(self.handle()) }
    }
    #[fixed_stack_segment]
    fn validate(&self) -> bool {
        unsafe { wxWindow_Validate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setVirtualSize(&self, w: c_int, h: c_int) {
        unsafe { wxWindow_SetVirtualSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn warpPointer(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_WarpPointer(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixelsEx(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertDialogToPixelsEx(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialogEx(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ConvertPixelsToDialogEx(self.handle())) as @_wxPoint }
    }
    #[fixed_stack_segment]
    fn screenToClient2(&self, x: c_int, y: c_int) -> @_wxPoint {
        unsafe { @wxPoint(wxWindow_ScreenToClient2(self.handle(), x, y)) as @_wxPoint }
    }
}

struct wxWindowCreateEvent(*u8);
impl _wxWindowCreateEvent for wxWindowCreateEvent {}
impl _wxCommandEvent for wxWindowCreateEvent {}
impl _wxEvent for wxWindowCreateEvent {}
impl _wxObject for wxWindowCreateEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowCreateEvent {
    pub fn from(handle: *u8) -> @_wxWindowCreateEvent {
        @wxWindowCreateEvent(handle) as @_wxWindowCreateEvent
    }
    
}

trait _wxWindowCreateEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxWindowCreateEvent_GetWindow(self.handle())) as @_wxWindow }
    }
}

struct wxWindowDC(*u8);
impl _wxWindowDC for wxWindowDC {}
impl _wxDC for wxWindowDC {}
impl _wxObject for wxWindowDC { fn handle(&self) -> *u8 { **self } }

impl wxWindowDC {
    pub fn from(handle: *u8) -> @_wxWindowDC {
        @wxWindowDC(handle) as @_wxWindowDC
    }
    
    #[fixed_stack_segment]
    pub fn new(win: @_wxWindow) -> @_wxWindowDC {
        unsafe { @wxWindowDC(wxWindowDC_Create(win.handle())) as @_wxWindowDC }
    }
}

trait _wxWindowDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxWindowDC_Delete(self.handle()) }
    }
}

struct wxWindowDestroyEvent(*u8);
impl _wxWindowDestroyEvent for wxWindowDestroyEvent {}
impl _wxCommandEvent for wxWindowDestroyEvent {}
impl _wxEvent for wxWindowDestroyEvent {}
impl _wxObject for wxWindowDestroyEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowDestroyEvent {
    pub fn from(handle: *u8) -> @_wxWindowDestroyEvent {
        @wxWindowDestroyEvent(handle) as @_wxWindowDestroyEvent
    }
    
}

trait _wxWindowDestroyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @_wxWindow {
        unsafe { @wxWindow(wxWindowDestroyEvent_GetWindow(self.handle())) as @_wxWindow }
    }
}

struct wxWindowDisabler(*u8);
impl _wxWindowDisabler for wxWindowDisabler { fn handle(&self) -> *u8 { **self } }

impl wxWindowDisabler {
    pub fn from(handle: *u8) -> @_wxWindowDisabler {
        @wxWindowDisabler(handle) as @_wxWindowDisabler
    }
    
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
    pub fn from(handle: *u8) -> @_wxWizard {
        @wxWizard(handle) as @_wxWizard
    }
    
    #[fixed_stack_segment]
    pub fn chain(f: @_wxWizardPageSimple, s: @_wxWizardPageSimple) {
        unsafe { wxWizard_Chain(f.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _bmp: @_wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> @_wxWizard {
        unsafe { @wxWizard(wxWizard_Create(_prt.handle(), _id, _txt.handle(), _bmp.handle(), _lft, _top, _wdt, _hgt)) as @_wxWizard }
    }
}

trait _wxWizard : _wxDialog {
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> @_wxWizardPage {
        unsafe { @wxWizardPage(wxWizard_GetCurrentPage(self.handle())) as @_wxWizardPage }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxWizard_GetPageSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn runWizard(&self, firstPage: @_wxWizardPage) -> c_int {
        unsafe { wxWizard_RunWizard(self.handle(), firstPage.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxWizardEvent {
        @wxWizardEvent(handle) as @_wxWizardEvent
    }
    
}

trait _wxWizardEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxWizardPage {
        @wxWizardPage(handle) as @_wxWizardPage
    }
    
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
    pub fn from(handle: *u8) -> @_wxWizardPageSimple {
        @wxWizardPageSimple(handle) as @_wxWizardPageSimple
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWizard) -> @_wxWizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_Create(_prt.handle())) as @_wxWizardPageSimple }
    }
}

trait _wxWizardPageSimple : _wxWizardPage {
    #[fixed_stack_segment]
    fn getBitmap(&self, _ref: @_wxBitmap) {
        unsafe { wxWizardPageSimple_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getNext(&self) -> @_wxWizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_GetNext(self.handle())) as @_wxWizardPageSimple }
    }
    #[fixed_stack_segment]
    fn getPrev(&self) -> @_wxWizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_GetPrev(self.handle())) as @_wxWizardPageSimple }
    }
    #[fixed_stack_segment]
    fn setNext(&self, next: @_wxWizardPageSimple) {
        unsafe { wxWizardPageSimple_SetNext(self.handle(), next.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrev(&self, prev: @_wxWizardPageSimple) {
        unsafe { wxWizardPageSimple_SetPrev(self.handle(), prev.handle()) }
    }
}

struct wxXmlResource(*u8);
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn handle(&self) -> *u8 { **self } }

impl wxXmlResource {
    pub fn from(handle: *u8) -> @_wxXmlResource {
        @wxXmlResource(handle) as @_wxXmlResource
    }
    
    #[fixed_stack_segment]
    pub fn new(flags: c_int) -> @_wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Create(flags)) as @_wxXmlResource }
    }
    #[fixed_stack_segment]
    pub fn newFromFile(filemask: @_wxString, flags: c_int) -> @_wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_CreateFromFile(filemask.handle(), flags)) as @_wxXmlResource }
    }
    #[fixed_stack_segment]
    pub fn get() -> @_wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Get()) as @_wxXmlResource }
    }
}

trait _wxXmlResource : _wxObject {
    #[fixed_stack_segment]
    fn addHandler(&self, handler: @_wxEvtHandler) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn addSubclassFactory(&self, factory: *u8) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    #[fixed_stack_segment]
    fn attachUnknownControl(&self, control: @_wxControl, parent: @_wxWindow) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.handle(), control.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.handle(), major, minor, release, revision) }
    }
    #[fixed_stack_segment]
    fn getDomain(&self) -> @_wxString {
        unsafe { @wxString(wxXmlResource_GetDomain(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxXmlResource_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getXRCID(&self, str_id: @_wxString) -> c_int {
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    #[fixed_stack_segment]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insertHandler(&self, handler: @_wxEvtHandler) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, filemask: @_wxString) -> bool {
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn loadBitmap(&self, name: @_wxString, _ref: @_wxBitmap) {
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadDialog(&self, parent: @_wxWindow, name: @_wxString) -> @_wxDialog {
        unsafe { @wxDialog(wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle())) as @_wxDialog }
    }
    #[fixed_stack_segment]
    fn loadFrame(&self, parent: @_wxWindow, name: @_wxString) -> @_wxFrame {
        unsafe { @wxFrame(wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle())) as @_wxFrame }
    }
    #[fixed_stack_segment]
    fn loadIcon(&self, name: @_wxString, _ref: @_wxIcon) {
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadMenu(&self, name: @_wxString) -> @_wxMenu {
        unsafe { @wxMenu(wxXmlResource_LoadMenu(self.handle(), name.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn loadMenuBar(&self, parent: @_wxWindow, name: @_wxString) -> @_wxMenuBar {
        unsafe { @wxMenuBar(wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle())) as @_wxMenuBar }
    }
    #[fixed_stack_segment]
    fn loadPanel(&self, parent: @_wxWindow, name: @_wxString) -> @_wxPanel {
        unsafe { @wxPanel(wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle())) as @_wxPanel }
    }
    #[fixed_stack_segment]
    fn loadToolBar(&self, parent: @_wxWindow, name: @_wxString) -> @_wxToolBar {
        unsafe { @wxToolBar(wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle())) as @_wxToolBar }
    }
    #[fixed_stack_segment]
    fn getSizer(&self, str_id: @_wxString) -> @_wxSizer {
        unsafe { @wxSizer(wxXmlResource_GetSizer(self.handle(), str_id.handle())) as @_wxSizer }
    }
    #[fixed_stack_segment]
    fn getBoxSizer(&self, str_id: @_wxString) -> @_wxBoxSizer {
        unsafe { @wxBoxSizer(wxXmlResource_GetBoxSizer(self.handle(), str_id.handle())) as @_wxBoxSizer }
    }
    #[fixed_stack_segment]
    fn getStaticBoxSizer(&self, str_id: @_wxString) -> @_wxStaticBoxSizer {
        unsafe { @wxStaticBoxSizer(wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle())) as @_wxStaticBoxSizer }
    }
    #[fixed_stack_segment]
    fn getGridSizer(&self, str_id: @_wxString) -> @_wxGridSizer {
        unsafe { @wxGridSizer(wxXmlResource_GetGridSizer(self.handle(), str_id.handle())) as @_wxGridSizer }
    }
    #[fixed_stack_segment]
    fn getFlexGridSizer(&self, str_id: @_wxString) -> @_wxFlexGridSizer {
        unsafe { @wxFlexGridSizer(wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle())) as @_wxFlexGridSizer }
    }
    #[fixed_stack_segment]
    fn getBitmapButton(&self, str_id: @_wxString) -> @_wxBitmapButton {
        unsafe { @wxBitmapButton(wxXmlResource_GetBitmapButton(self.handle(), str_id.handle())) as @_wxBitmapButton }
    }
    #[fixed_stack_segment]
    fn getButton(&self, str_id: @_wxString) -> @_wxButton {
        unsafe { @wxButton(wxXmlResource_GetButton(self.handle(), str_id.handle())) as @_wxButton }
    }
    #[fixed_stack_segment]
    fn getCalendarCtrl(&self, str_id: @_wxString) -> @_wxCalendarCtrl {
        unsafe { @wxCalendarCtrl(wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle())) as @_wxCalendarCtrl }
    }
    #[fixed_stack_segment]
    fn getCheckBox(&self, str_id: @_wxString) -> @_wxCheckBox {
        unsafe { @wxCheckBox(wxXmlResource_GetCheckBox(self.handle(), str_id.handle())) as @_wxCheckBox }
    }
    #[fixed_stack_segment]
    fn getCheckListBox(&self, str_id: @_wxString) -> @_wxCheckListBox {
        unsafe { @wxCheckListBox(wxXmlResource_GetCheckListBox(self.handle(), str_id.handle())) as @_wxCheckListBox }
    }
    #[fixed_stack_segment]
    fn getChoice(&self, str_id: @_wxString) -> @_wxChoice {
        unsafe { @wxChoice(wxXmlResource_GetChoice(self.handle(), str_id.handle())) as @_wxChoice }
    }
    #[fixed_stack_segment]
    fn getComboBox(&self, str_id: @_wxString) -> @_wxComboBox {
        unsafe { @wxComboBox(wxXmlResource_GetComboBox(self.handle(), str_id.handle())) as @_wxComboBox }
    }
    #[fixed_stack_segment]
    fn getGauge(&self, str_id: @_wxString) -> @_wxGauge {
        unsafe { @wxGauge(wxXmlResource_GetGauge(self.handle(), str_id.handle())) as @_wxGauge }
    }
    #[fixed_stack_segment]
    fn getGrid(&self, str_id: @_wxString) -> @_wxGrid {
        unsafe { @wxGrid(wxXmlResource_GetGrid(self.handle(), str_id.handle())) as @_wxGrid }
    }
    #[fixed_stack_segment]
    fn getHtmlWindow(&self, str_id: @_wxString) -> @_wxHtmlWindow {
        unsafe { @wxHtmlWindow(wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle())) as @_wxHtmlWindow }
    }
    #[fixed_stack_segment]
    fn getListBox(&self, str_id: @_wxString) -> @_wxListBox {
        unsafe { @wxListBox(wxXmlResource_GetListBox(self.handle(), str_id.handle())) as @_wxListBox }
    }
    #[fixed_stack_segment]
    fn getListCtrl(&self, str_id: @_wxString) -> @_wxListCtrl {
        unsafe { @wxListCtrl(wxXmlResource_GetListCtrl(self.handle(), str_id.handle())) as @_wxListCtrl }
    }
    #[fixed_stack_segment]
    fn getMDIChildFrame(&self, str_id: @_wxString) -> @_wxMDIChildFrame {
        unsafe { @wxMDIChildFrame(wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle())) as @_wxMDIChildFrame }
    }
    #[fixed_stack_segment]
    fn getMDIParentFrame(&self, str_id: @_wxString) -> @_wxMDIParentFrame {
        unsafe { @wxMDIParentFrame(wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle())) as @_wxMDIParentFrame }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, str_id: @_wxString) -> @_wxMenu {
        unsafe { @wxMenu(wxXmlResource_GetMenu(self.handle(), str_id.handle())) as @_wxMenu }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self, str_id: @_wxString) -> @_wxMenuBar {
        unsafe { @wxMenuBar(wxXmlResource_GetMenuBar(self.handle(), str_id.handle())) as @_wxMenuBar }
    }
    #[fixed_stack_segment]
    fn getMenuItem(&self, str_id: @_wxString) -> @_wxMenuItem {
        unsafe { @wxMenuItem(wxXmlResource_GetMenuItem(self.handle(), str_id.handle())) as @_wxMenuItem }
    }
    #[fixed_stack_segment]
    fn getNotebook(&self, str_id: @_wxString) -> @_wxNotebook {
        unsafe { @wxNotebook(wxXmlResource_GetNotebook(self.handle(), str_id.handle())) as @_wxNotebook }
    }
    #[fixed_stack_segment]
    fn getPanel(&self, str_id: @_wxString) -> @_wxPanel {
        unsafe { @wxPanel(wxXmlResource_GetPanel(self.handle(), str_id.handle())) as @_wxPanel }
    }
    #[fixed_stack_segment]
    fn getRadioButton(&self, str_id: @_wxString) -> @_wxRadioButton {
        unsafe { @wxRadioButton(wxXmlResource_GetRadioButton(self.handle(), str_id.handle())) as @_wxRadioButton }
    }
    #[fixed_stack_segment]
    fn getRadioBox(&self, str_id: @_wxString) -> @_wxRadioBox {
        unsafe { @wxRadioBox(wxXmlResource_GetRadioBox(self.handle(), str_id.handle())) as @_wxRadioBox }
    }
    #[fixed_stack_segment]
    fn getScrollBar(&self, str_id: @_wxString) -> @_wxScrollBar {
        unsafe { @wxScrollBar(wxXmlResource_GetScrollBar(self.handle(), str_id.handle())) as @_wxScrollBar }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow(&self, str_id: @_wxString) -> @_wxScrolledWindow {
        unsafe { @wxScrolledWindow(wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle())) as @_wxScrolledWindow }
    }
    #[fixed_stack_segment]
    fn getSlider(&self, str_id: @_wxString) -> @_wxSlider {
        unsafe { @wxSlider(wxXmlResource_GetSlider(self.handle(), str_id.handle())) as @_wxSlider }
    }
    #[fixed_stack_segment]
    fn getSpinButton(&self, str_id: @_wxString) -> @_wxSpinButton {
        unsafe { @wxSpinButton(wxXmlResource_GetSpinButton(self.handle(), str_id.handle())) as @_wxSpinButton }
    }
    #[fixed_stack_segment]
    fn getSpinCtrl(&self, str_id: @_wxString) -> @_wxSpinCtrl {
        unsafe { @wxSpinCtrl(wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle())) as @_wxSpinCtrl }
    }
    #[fixed_stack_segment]
    fn getSplitterWindow(&self, str_id: @_wxString) -> @_wxSplitterWindow {
        unsafe { @wxSplitterWindow(wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle())) as @_wxSplitterWindow }
    }
    #[fixed_stack_segment]
    fn getStaticBitmap(&self, str_id: @_wxString) -> @_wxStaticBitmap {
        unsafe { @wxStaticBitmap(wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle())) as @_wxStaticBitmap }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self, str_id: @_wxString) -> @_wxStaticBox {
        unsafe { @wxStaticBox(wxXmlResource_GetStaticBox(self.handle(), str_id.handle())) as @_wxStaticBox }
    }
    #[fixed_stack_segment]
    fn getStaticLine(&self, str_id: @_wxString) -> @_wxStaticLine {
        unsafe { @wxStaticLine(wxXmlResource_GetStaticLine(self.handle(), str_id.handle())) as @_wxStaticLine }
    }
    #[fixed_stack_segment]
    fn getStaticText(&self, str_id: @_wxString) -> @_wxStaticText {
        unsafe { @wxStaticText(wxXmlResource_GetStaticText(self.handle(), str_id.handle())) as @_wxStaticText }
    }
    #[fixed_stack_segment]
    fn getTextCtrl(&self, str_id: @_wxString) -> @_wxTextCtrl {
        unsafe { @wxTextCtrl(wxXmlResource_GetTextCtrl(self.handle(), str_id.handle())) as @_wxTextCtrl }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl(&self, str_id: @_wxString) -> @_wxTreeCtrl {
        unsafe { @wxTreeCtrl(wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle())) as @_wxTreeCtrl }
    }
    #[fixed_stack_segment]
    fn unload(&self, filemask: @_wxString) -> bool {
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, res: @_wxXmlResource) -> @_wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Set(self.handle(), res.handle())) as @_wxXmlResource }
    }
    #[fixed_stack_segment]
    fn setDomain(&self, domain: @_wxString) {
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn getStyledTextCtrl(&self, str_id: @_wxString) -> @_wxStyledTextCtrl {
        unsafe { @wxStyledTextCtrl(wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle())) as @_wxStyledTextCtrl }
    }
}

struct wxXmlResourceHandler(*u8);
impl _wxXmlResourceHandler for wxXmlResourceHandler {}
impl _wxObject for wxXmlResourceHandler { fn handle(&self) -> *u8 { **self } }

impl wxXmlResourceHandler {
    pub fn from(handle: *u8) -> @_wxXmlResourceHandler {
        @wxXmlResourceHandler(handle) as @_wxXmlResourceHandler
    }
    
}

trait _wxXmlResourceHandler : _wxObject {
}

struct wxZipInputStream(*u8);
impl _wxZipInputStream for wxZipInputStream {}
impl _wxInputStream for wxZipInputStream {}
impl _wxStreamBase for wxZipInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZipInputStream {
    pub fn from(handle: *u8) -> @_wxZipInputStream {
        @wxZipInputStream(handle) as @_wxZipInputStream
    }
    
}

trait _wxZipInputStream : _wxInputStream {
}

struct wxZlibInputStream(*u8);
impl _wxZlibInputStream for wxZlibInputStream {}
impl _wxFilterInputStream for wxZlibInputStream {}
impl _wxInputStream for wxZlibInputStream {}
impl _wxStreamBase for wxZlibInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibInputStream {
    pub fn from(handle: *u8) -> @_wxZlibInputStream {
        @wxZlibInputStream(handle) as @_wxZlibInputStream
    }
    
}

trait _wxZlibInputStream : _wxFilterInputStream {
}

struct wxZlibOutputStream(*u8);
impl _wxZlibOutputStream for wxZlibOutputStream {}
impl _wxFilterOutputStream for wxZlibOutputStream {}
impl _wxOutputStream for wxZlibOutputStream {}
impl _wxStreamBase for wxZlibOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibOutputStream {
    pub fn from(handle: *u8) -> @_wxZlibOutputStream {
        @wxZlibOutputStream(handle) as @_wxZlibOutputStream
    }
    
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
    pub fn from(handle: *u8) -> @_wxPropertyGrid {
        @wxPropertyGrid(handle) as @_wxPropertyGrid
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @_wxPropertyGrid {
        unsafe { @wxPropertyGrid(wxPropertyGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @_wxPropertyGrid }
    }
}

trait _wxPropertyGrid : _wxControl {
    #[fixed_stack_segment]
    fn append(&self, prop: @_wxPGProperty) -> @_wxPGProperty {
        unsafe { @wxPGProperty(wxPropertyGrid_Append(self.handle(), prop.handle())) as @_wxPGProperty }
    }
    #[fixed_stack_segment]
    fn disableProperty(&self, propName: @_wxString) -> bool {
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
    pub fn from(handle: *u8) -> @_wxPropertyGridEvent {
        @wxPropertyGridEvent(handle) as @_wxPropertyGridEvent
    }
    
}

trait _wxPropertyGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn hasProperty(&self) -> bool {
        unsafe { wxPropertyGridEvent_HasProperty(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getProperty(&self) -> @_wxPGProperty {
        unsafe { @wxPGProperty(wxPropertyGridEvent_GetProperty(self.handle())) as @_wxPGProperty }
    }
}

struct wxPGProperty(*u8);
impl _wxPGProperty for wxPGProperty {}
impl _wxObject for wxPGProperty { fn handle(&self) -> *u8 { **self } }

impl wxPGProperty {
    pub fn from(handle: *u8) -> @_wxPGProperty {
        @wxPGProperty(handle) as @_wxPGProperty
    }
    
}

trait _wxPGProperty : _wxObject {
    #[fixed_stack_segment]
    fn getLabel(&self) -> @_wxString {
        unsafe { @wxString(wxPGProperty_GetLabel(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @_wxString {
        unsafe { @wxString(wxPGProperty_GetName(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getValueAsString(&self) -> @_wxString {
        unsafe { @wxString(wxPGProperty_GetValueAsString(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getValueType(&self) -> @_wxString {
        unsafe { @wxString(wxPGProperty_GetValueType(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, helpString: @_wxString) {
        unsafe { wxPGProperty_SetHelpString(self.handle(), helpString.handle()) }
    }
}

struct wxStringProperty(*u8);
impl _wxStringProperty for wxStringProperty {}
impl _wxPGProperty for wxStringProperty {}
impl _wxObject for wxStringProperty { fn handle(&self) -> *u8 { **self } }

impl wxStringProperty {
    pub fn from(handle: *u8) -> @_wxStringProperty {
        @wxStringProperty(handle) as @_wxStringProperty
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString, name: @_wxString, value: @_wxString) -> @_wxStringProperty {
        unsafe { @wxStringProperty(wxStringProperty_Create(label.handle(), name.handle(), value.handle())) as @_wxStringProperty }
    }
}

trait _wxStringProperty : _wxPGProperty {
}

struct wxIntProperty(*u8);
impl _wxIntProperty for wxIntProperty {}
impl _wxPGProperty for wxIntProperty {}
impl _wxObject for wxIntProperty { fn handle(&self) -> *u8 { **self } }

impl wxIntProperty {
    pub fn from(handle: *u8) -> @_wxIntProperty {
        @wxIntProperty(handle) as @_wxIntProperty
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString, name: @_wxString, value: c_int) -> @_wxIntProperty {
        unsafe { @wxIntProperty(wxIntProperty_Create(label.handle(), name.handle(), value)) as @_wxIntProperty }
    }
}

trait _wxIntProperty : _wxPGProperty {
}

struct wxBoolProperty(*u8);
impl _wxBoolProperty for wxBoolProperty {}
impl _wxPGProperty for wxBoolProperty {}
impl _wxObject for wxBoolProperty { fn handle(&self) -> *u8 { **self } }

impl wxBoolProperty {
    pub fn from(handle: *u8) -> @_wxBoolProperty {
        @wxBoolProperty(handle) as @_wxBoolProperty
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString, name: @_wxString, value: bool) -> @_wxBoolProperty {
        unsafe { @wxBoolProperty(wxBoolProperty_Create(label.handle(), name.handle(), value)) as @_wxBoolProperty }
    }
}

trait _wxBoolProperty : _wxPGProperty {
}

struct wxFloatProperty(*u8);
impl _wxFloatProperty for wxFloatProperty {}
impl _wxPGProperty for wxFloatProperty {}
impl _wxObject for wxFloatProperty { fn handle(&self) -> *u8 { **self } }

impl wxFloatProperty {
    pub fn from(handle: *u8) -> @_wxFloatProperty {
        @wxFloatProperty(handle) as @_wxFloatProperty
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString, name: @_wxString, value: c_float) -> @_wxFloatProperty {
        unsafe { @wxFloatProperty(wxFloatProperty_Create(label.handle(), name.handle(), value)) as @_wxFloatProperty }
    }
}

trait _wxFloatProperty : _wxPGProperty {
}

struct wxDateProperty(*u8);
impl _wxDateProperty for wxDateProperty {}
impl _wxPGProperty for wxDateProperty {}
impl _wxObject for wxDateProperty { fn handle(&self) -> *u8 { **self } }

impl wxDateProperty {
    pub fn from(handle: *u8) -> @_wxDateProperty {
        @wxDateProperty(handle) as @_wxDateProperty
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString, name: @_wxString, value: @_wxDateTime) -> @_wxDateProperty {
        unsafe { @wxDateProperty(wxDateProperty_Create(label.handle(), name.handle(), value.handle())) as @_wxDateProperty }
    }
}

trait _wxDateProperty : _wxPGProperty {
}

struct wxFileProperty(*u8);
impl _wxFileProperty for wxFileProperty {}
impl _wxPGProperty for wxFileProperty {}
impl _wxObject for wxFileProperty { fn handle(&self) -> *u8 { **self } }

impl wxFileProperty {
    pub fn from(handle: *u8) -> @_wxFileProperty {
        @wxFileProperty(handle) as @_wxFileProperty
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString, name: @_wxString, value: @_wxString) -> @_wxFileProperty {
        unsafe { @wxFileProperty(wxFileProperty_Create(label.handle(), name.handle(), value.handle())) as @_wxFileProperty }
    }
}

trait _wxFileProperty : _wxPGProperty {
}

struct wxPropertyCategory(*u8);
impl _wxPropertyCategory for wxPropertyCategory {}
impl _wxPGProperty for wxPropertyCategory {}
impl _wxObject for wxPropertyCategory { fn handle(&self) -> *u8 { **self } }

impl wxPropertyCategory {
    pub fn from(handle: *u8) -> @_wxPropertyCategory {
        @wxPropertyCategory(handle) as @_wxPropertyCategory
    }
    
    #[fixed_stack_segment]
    pub fn new(label: @_wxString) -> @_wxPropertyCategory {
        unsafe { @wxPropertyCategory(wxPropertyCategory_Create(label.handle())) as @_wxPropertyCategory }
    }
}

trait _wxPropertyCategory : _wxPGProperty {
}

struct wxGenericDragImage(*u8);
impl _wxGenericDragImage for wxGenericDragImage {}
impl _wxDragImage for wxGenericDragImage {}
impl _wxObject for wxGenericDragImage { fn handle(&self) -> *u8 { **self } }

impl wxGenericDragImage {
    pub fn from(handle: *u8) -> @_wxGenericDragImage {
        @wxGenericDragImage(handle) as @_wxGenericDragImage
    }
    
    #[fixed_stack_segment]
    pub fn new(cursor: @_wxCursor) -> @_wxGenericDragImage {
        unsafe { @wxGenericDragImage(wxGenericDragImage_Create(cursor.handle())) as @_wxGenericDragImage }
    }
}

trait _wxGenericDragImage : _wxDragImage {
    #[fixed_stack_segment]
    fn doDrawImage(&self, dc: @_wxDC, x: c_int, y: c_int) -> bool {
        unsafe { wxGenericDragImage_DoDrawImage(self.handle(), dc.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> @_wxRect {
        unsafe { @wxRect(wxGenericDragImage_GetImageRect(self.handle(), x_pos, y_pos)) as @_wxRect }
    }
    #[fixed_stack_segment]
    fn updateBackingFromWindow(&self, windowDC: @_wxDC, destDC: @_wxMemoryDC, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.handle(), windowDC.handle(), destDC.handle(), x, y, w, h, xdest, ydest, width, height) }
    }
}

struct wxGraphicsObject(*u8);
impl _wxGraphicsObject for wxGraphicsObject {}
impl _wxObject for wxGraphicsObject { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsObject {
    pub fn from(handle: *u8) -> @_wxGraphicsObject {
        @wxGraphicsObject(handle) as @_wxGraphicsObject
    }
    
    #[fixed_stack_segment]
    pub fn getRenderer() -> @_wxGraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsObject_GetRenderer()) as @_wxGraphicsRenderer }
    }
}

trait _wxGraphicsObject : _wxObject {
    #[fixed_stack_segment]
    fn isNull(&self) -> bool {
        unsafe { wxGraphicsObject_IsNull(self.handle()) }
    }
}

struct wxGraphicsBrush(*u8);
impl _wxGraphicsBrush for wxGraphicsBrush {}
impl _wxGraphicsObject for wxGraphicsBrush {}
impl _wxObject for wxGraphicsBrush { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsBrush {
    pub fn from(handle: *u8) -> @_wxGraphicsBrush {
        @wxGraphicsBrush(handle) as @_wxGraphicsBrush
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxGraphicsBrush {
        unsafe { @wxGraphicsBrush(wxGraphicsBrush_Create()) as @_wxGraphicsBrush }
    }
}

trait _wxGraphicsBrush : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsBrush_Delete(self.handle()) }
    }
}

struct wxGraphicsContext(*u8);
impl _wxGraphicsContext for wxGraphicsContext {}
impl _wxGraphicsObject for wxGraphicsContext {}
impl _wxObject for wxGraphicsContext { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsContext {
    pub fn from(handle: *u8) -> @_wxGraphicsContext {
        @wxGraphicsContext(handle) as @_wxGraphicsContext
    }
    
    #[fixed_stack_segment]
    pub fn new(dc: @_wxWindowDC) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_Create(dc.handle())) as @_wxGraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newFromWindow(window: @_wxWindow) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromWindow(window.handle())) as @_wxGraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newFromNative(context: *u8) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNative(context)) as @_wxGraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newFromNativeWindow(window: *u8) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNativeWindow(window)) as @_wxGraphicsContext }
    }
}

trait _wxGraphicsContext : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsContext_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clip(&self, region: @_wxRegion) {
        unsafe { wxGraphicsContext_Clip(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawBitmap(&self, bmp: @_wxBitmap, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.handle(), bmp.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawIcon(&self, icon: @_wxIcon, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.handle(), icon.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.handle(), n, x, y, style) }
    }
    #[fixed_stack_segment]
    fn drawPath(&self, path: @_wxGraphicsPath, style: c_int) {
        unsafe { wxGraphicsContext_DrawPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawRectangle(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawRoundedRectangle(self.handle(), x, y, w, h, radius) }
    }
    #[fixed_stack_segment]
    fn drawText(&self, text: @_wxString, x: c_double, y: c_double) {
        unsafe { wxGraphicsContext_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawTextWithAngle(&self, text: @_wxString, x: c_double, y: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.handle(), text.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    fn fillPath(&self, path: @_wxGraphicsPath, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    fn strokePath(&self, path: @_wxGraphicsPath) {
        unsafe { wxGraphicsContext_StrokePath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativeContext(&self) -> *u8 {
        unsafe { wxGraphicsContext_GetNativeContext(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, text: @_wxString, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double) {
        unsafe { wxGraphicsContext_GetTextExtent(self.handle(), text.handle(), width, height, descent, externalLeading) }
    }
    #[fixed_stack_segment]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsContext_Rotate(self.handle(), angle) }
    }
    #[fixed_stack_segment]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsContext_Scale(self.handle(), xScale, yScale) }
    }
    #[fixed_stack_segment]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsContext_Translate(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    fn setTransform(&self, path: @_wxGraphicsMatrix) {
        unsafe { wxGraphicsContext_SetTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn concatTransform(&self, path: @_wxGraphicsMatrix) {
        unsafe { wxGraphicsContext_ConcatTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @_wxBrush) {
        unsafe { wxGraphicsContext_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsBrush(&self, brush: @_wxGraphicsBrush) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @_wxFont, colour: @_wxColour) {
        unsafe { wxGraphicsContext_SetFont(self.handle(), font.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsFont(&self, font: @_wxGraphicsFont) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @_wxPen) {
        unsafe { wxGraphicsContext_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsPen(&self, pen: @_wxGraphicsPen) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self.handle(), x1, y1, x2, y2) }
    }
    #[fixed_stack_segment]
    fn strokeLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.handle(), n, x, y, style) }
    }
}

struct wxGraphicsFont(*u8);
impl _wxGraphicsFont for wxGraphicsFont {}
impl _wxGraphicsObject for wxGraphicsFont {}
impl _wxObject for wxGraphicsFont { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsFont {
    pub fn from(handle: *u8) -> @_wxGraphicsFont {
        @wxGraphicsFont(handle) as @_wxGraphicsFont
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxGraphicsFont {
        unsafe { @wxGraphicsFont(wxGraphicsFont_Create()) as @_wxGraphicsFont }
    }
}

trait _wxGraphicsFont : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsFont_Delete(self.handle()) }
    }
}

struct wxGraphicsMatrix(*u8);
impl _wxGraphicsMatrix for wxGraphicsMatrix {}
impl _wxGraphicsObject for wxGraphicsMatrix {}
impl _wxObject for wxGraphicsMatrix { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsMatrix {
    pub fn from(handle: *u8) -> @_wxGraphicsMatrix {
        @wxGraphicsMatrix(handle) as @_wxGraphicsMatrix
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxGraphicsMatrix {
        unsafe { @wxGraphicsMatrix(wxGraphicsMatrix_Create()) as @_wxGraphicsMatrix }
    }
}

trait _wxGraphicsMatrix : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsMatrix_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn concat(&self, t: @_wxGraphicsMatrix) {
        unsafe { wxGraphicsMatrix_Concat(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    fn get(&self, a: *c_double, b: *c_double, c: *c_double, d: *c_double, tx: *c_double, ty: *c_double) {
        unsafe { wxGraphicsMatrix_Get(self.handle(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    fn getNativeMatrix(&self) -> *u8 {
        unsafe { wxGraphicsMatrix_GetNativeMatrix(self.handle()) }
    }
    #[fixed_stack_segment]
    fn invert(&self) {
        unsafe { wxGraphicsMatrix_Invert(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, t: @_wxGraphicsMatrix) -> bool {
        unsafe { wxGraphicsMatrix_IsEqual(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    fn isIdentity(&self) -> bool {
        unsafe { wxGraphicsMatrix_IsIdentity(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsMatrix_Rotate(self.handle(), angle) }
    }
    #[fixed_stack_segment]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsMatrix_Scale(self.handle(), xScale, yScale) }
    }
    #[fixed_stack_segment]
    fn set(&self, a: c_double, b: c_double, c: c_double, d: c_double, tx: c_double, ty: c_double) {
        unsafe { wxGraphicsMatrix_Set(self.handle(), a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsMatrix_Translate(self.handle(), dx, dy) }
    }
    #[fixed_stack_segment]
    fn transformPoint(&self, x: *c_double, y: *c_double) {
        unsafe { wxGraphicsMatrix_TransformPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn transformDistance(&self, dx: *c_double, dy: *c_double) {
        unsafe { wxGraphicsMatrix_TransformDistance(self.handle(), dx, dy) }
    }
}

struct wxGraphicsPath(*u8);
impl _wxGraphicsPath for wxGraphicsPath {}
impl _wxGraphicsObject for wxGraphicsPath {}
impl _wxObject for wxGraphicsPath { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsPath {
    pub fn from(handle: *u8) -> @_wxGraphicsPath {
        @wxGraphicsPath(handle) as @_wxGraphicsPath
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxGraphicsPath {
        unsafe { @wxGraphicsPath(wxGraphicsPath_Create()) as @_wxGraphicsPath }
    }
    #[fixed_stack_segment]
    pub fn unGetNativePath(p: *u8) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

trait _wxGraphicsPath : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsPath_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn moveToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_MoveToPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: bool) {
        unsafe { wxGraphicsPath_AddArc(self.handle(), x, y, r, startAngle, endAngle, clockwise) }
    }
    #[fixed_stack_segment]
    fn addArcToPoint(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddArcToPoint(self.handle(), x1, y1, x2, y2, r) }
    }
    #[fixed_stack_segment]
    fn addCircle(&self, x: c_double, y: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddCircle(self.handle(), x, y, r) }
    }
    #[fixed_stack_segment]
    fn addCurveToPoint(&self, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddCurveToPoint(self.handle(), cx1, cy1, cx2, cy2, x, y) }
    }
    #[fixed_stack_segment]
    fn addEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn addLineToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddLineToPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn addPath(&self, x: c_double, y: c_double, path: @_wxGraphicsPath) {
        unsafe { wxGraphicsPath_AddPath(self.handle(), x, y, path.handle()) }
    }
    #[fixed_stack_segment]
    fn addQuadCurveToPoint(&self, cx: c_double, cy: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddQuadCurveToPoint(self.handle(), cx, cy, x, y) }
    }
    #[fixed_stack_segment]
    fn addRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddRectangle(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn addRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsPath_AddRoundedRectangle(self.handle(), x, y, w, h, radius) }
    }
    #[fixed_stack_segment]
    fn closeSubpath(&self) {
        unsafe { wxGraphicsPath_CloseSubpath(self.handle()) }
    }
    #[fixed_stack_segment]
    fn contains(&self, x: c_double, y: c_double, style: c_int) {
        unsafe { wxGraphicsPath_Contains(self.handle(), x, y, style) }
    }
    #[fixed_stack_segment]
    fn getBox(&self, x: *c_double, y: *c_double, w: *c_double, h: *c_double) {
        unsafe { wxGraphicsPath_GetBox(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn getCurrentPoint(&self, x: *c_double, y: *c_double) {
        unsafe { wxGraphicsPath_GetCurrentPoint(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn transform(&self, matrix: @_wxGraphicsMatrix) {
        unsafe { wxGraphicsPath_Transform(self.handle(), matrix.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativePath(&self) -> *u8 {
        unsafe { wxGraphicsPath_GetNativePath(self.handle()) }
    }
}

struct wxGraphicsPen(*u8);
impl _wxGraphicsPen for wxGraphicsPen {}
impl _wxGraphicsObject for wxGraphicsPen {}
impl _wxObject for wxGraphicsPen { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsPen {
    pub fn from(handle: *u8) -> @_wxGraphicsPen {
        @wxGraphicsPen(handle) as @_wxGraphicsPen
    }
    
    #[fixed_stack_segment]
    pub fn new() -> @_wxGraphicsPen {
        unsafe { @wxGraphicsPen(wxGraphicsPen_Create()) as @_wxGraphicsPen }
    }
}

trait _wxGraphicsPen : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsPen_Delete(self.handle()) }
    }
}

struct wxGraphicsRenderer(*u8);
impl _wxGraphicsRenderer for wxGraphicsRenderer {}
impl _wxGraphicsObject for wxGraphicsRenderer {}
impl _wxObject for wxGraphicsRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsRenderer {
    pub fn from(handle: *u8) -> @_wxGraphicsRenderer {
        @wxGraphicsRenderer(handle) as @_wxGraphicsRenderer
    }
    
    #[fixed_stack_segment]
    pub fn newContext(dc: @_wxWindowDC) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContext(dc.handle())) as @_wxGraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newContextFromWindow(window: @_wxWindow) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromWindow(window.handle())) as @_wxGraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newContextFromNativeContext(context: *u8) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeContext(context)) as @_wxGraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newContextFromNativeWindow(window: *u8) -> @_wxGraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) as @_wxGraphicsContext }
    }
}

trait _wxGraphicsRenderer : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsRenderer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @_wxGraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsRenderer_GetDefaultRenderer(self.handle())) as @_wxGraphicsRenderer }
    }
}

struct wxGLContext(*u8);
impl _wxGLContext for wxGLContext {}
impl _wxObject for wxGLContext { fn handle(&self) -> *u8 { **self } }

impl wxGLContext {
    pub fn from(handle: *u8) -> @_wxGLContext {
        @wxGLContext(handle) as @_wxGLContext
    }
    
    #[fixed_stack_segment]
    pub fn new(win: @_wxGLCanvas, other: @_wxGLContext) -> @_wxGLContext {
        unsafe { @wxGLContext(wxGLContext_Create(win.handle(), other.handle())) as @_wxGLContext }
    }
    #[fixed_stack_segment]
    pub fn newFromNull(win: @_wxGLCanvas) -> @_wxGLContext {
        unsafe { @wxGLContext(wxGLContext_CreateFromNull(win.handle())) as @_wxGLContext }
    }
}

trait _wxGLContext : _wxObject {
    #[fixed_stack_segment]
    fn setCurrent(&self, win: @_wxGLCanvas) -> bool {
        unsafe { wxGLContext_SetCurrent(self.handle(), win.handle()) }
    }
}

struct wxManagedPtr(*u8);
impl _wxManagedPtr for wxManagedPtr { fn handle(&self) -> *u8 { **self } }

impl wxManagedPtr {
    pub fn from(handle: *u8) -> @_wxManagedPtr {
        @wxManagedPtr(handle) as @_wxManagedPtr
    }
    
    #[fixed_stack_segment]
    pub fn getDeleteFunction() -> *u8 {
        unsafe { wxManagedPtr_GetDeleteFunction() }
    }
    #[fixed_stack_segment]
    pub fn newFromObject(obj: @_wxObject) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromObject(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromDateTime(obj: @_wxDateTime) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromDateTime(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromGridCellCoordsArray(obj: @_wxGridCellCoordsArray) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromGridCellCoordsArray(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(obj: @_wxBitmap) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromBitmap(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromIcon(obj: @_wxIcon) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromIcon(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromBrush(obj: @_wxBrush) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromBrush(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromColour(obj: @_wxColour) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromColour(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromCursor(obj: @_wxCursor) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromCursor(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromFont(obj: @_wxFont) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromFont(obj.handle())) as @_wxManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromPen(obj: @_wxPen) -> @_wxManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromPen(obj.handle())) as @_wxManagedPtr }
    }
}

trait _wxManagedPtr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getPtr(&self) -> *u8 {
        unsafe { wxManagedPtr_GetPtr(self.handle()) }
    }
    #[fixed_stack_segment]
    fn noFinalize(&self) {
        unsafe { wxManagedPtr_NoFinalize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn finalize(&self) {
        unsafe { wxManagedPtr_Finalize(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxMediaCtrl {
        @wxMediaCtrl(handle) as @_wxMediaCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(parent: @_wxWindow, windowID: c_int, fileName: @_wxString, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: @_wxString, name: @_wxString) -> @_wxMediaCtrl {
        unsafe { @wxMediaCtrl(wxMediaCtrl_Create(parent.handle(), windowID, fileName.handle(), x, y, w, h, style, szBackend.handle(), name.handle())) as @_wxMediaCtrl }
    }
}

trait _wxMediaCtrl : _wxWindow {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMediaCtrl_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @_wxSize {
        unsafe { @wxSize(wxMediaCtrl_GetBestSize(self.handle())) as @_wxSize }
    }
    #[fixed_stack_segment]
    fn getPlaybackRate(&self) -> c_double {
        unsafe { wxMediaCtrl_GetPlaybackRate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVolume(&self) -> c_double {
        unsafe { wxMediaCtrl_GetVolume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getState(&self) -> c_int {
        unsafe { wxMediaCtrl_GetState(self.handle()) }
    }
    #[fixed_stack_segment]
    fn length(&self) -> i64 {
        unsafe { wxMediaCtrl_Length(self.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, fileName: @_wxString) -> bool {
        unsafe { wxMediaCtrl_Load(self.handle(), fileName.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURI(&self, uri: @_wxString) -> bool {
        unsafe { wxMediaCtrl_LoadURI(self.handle(), uri.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURIWithProxy(&self, uri: @_wxString, proxy: @_wxString) -> bool {
        unsafe { wxMediaCtrl_LoadURIWithProxy(self.handle(), uri.handle(), proxy.handle()) }
    }
    #[fixed_stack_segment]
    fn pause(&self) -> bool {
        unsafe { wxMediaCtrl_Pause(self.handle()) }
    }
    #[fixed_stack_segment]
    fn play(&self) -> bool {
        unsafe { wxMediaCtrl_Play(self.handle()) }
    }
    #[fixed_stack_segment]
    fn seek(&self, offsetWhere: i64, mode: c_int) -> i64 {
        unsafe { wxMediaCtrl_Seek(self.handle(), offsetWhere, mode) }
    }
    #[fixed_stack_segment]
    fn setPlaybackRate(&self, dRate: c_double) -> bool {
        unsafe { wxMediaCtrl_SetPlaybackRate(self.handle(), dRate) }
    }
    #[fixed_stack_segment]
    fn setVolume(&self, dVolume: c_double) -> bool {
        unsafe { wxMediaCtrl_SetVolume(self.handle(), dVolume) }
    }
    #[fixed_stack_segment]
    fn showPlayerControls(&self, flags: c_int) -> bool {
        unsafe { wxMediaCtrl_ShowPlayerControls(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn stop(&self) -> bool {
        unsafe { wxMediaCtrl_Stop(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxMediaEvent {
        @wxMediaEvent(handle) as @_wxMediaEvent
    }
    
}

trait _wxMediaEvent : _wxNotifyEvent {
}

struct wxcPrintout(*u8);
impl _wxcPrintout for wxcPrintout {}
impl _wxPrintout for wxcPrintout {}
impl _wxObject for wxcPrintout { fn handle(&self) -> *u8 { **self } }

impl wxcPrintout {
    pub fn from(handle: *u8) -> @_wxcPrintout {
        @wxcPrintout(handle) as @_wxcPrintout
    }
    
    #[fixed_stack_segment]
    pub fn new(title: @_wxString) -> @_wxcPrintout {
        unsafe { @wxcPrintout(wxcPrintout_Create(title.handle())) as @_wxcPrintout }
    }
}

trait _wxcPrintout : _wxPrintout {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxcPrintout_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
    #[fixed_stack_segment]
    fn getEvtHandler(&self) -> @_wxcPrintoutHandler {
        unsafe { @wxcPrintoutHandler(wxcPrintout_GetEvtHandler(self.handle())) as @_wxcPrintoutHandler }
    }
}

struct wxcPrintEvent(*u8);
impl _wxcPrintEvent for wxcPrintEvent {}
impl _wxEvent for wxcPrintEvent {}
impl _wxObject for wxcPrintEvent { fn handle(&self) -> *u8 { **self } }

impl wxcPrintEvent {
    pub fn from(handle: *u8) -> @_wxcPrintEvent {
        @wxcPrintEvent(handle) as @_wxcPrintEvent
    }
    
}

trait _wxcPrintEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @_wxcPrintout {
        unsafe { @wxcPrintout(wxcPrintEvent_GetPrintout(self.handle())) as @_wxcPrintout }
    }
    #[fixed_stack_segment]
    fn getPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEndPage(&self) -> c_int {
        unsafe { wxcPrintEvent_GetEndPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getContinue(&self) -> bool {
        unsafe { wxcPrintEvent_GetContinue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setContinue(&self, cont: bool) {
        unsafe { wxcPrintEvent_SetContinue(self.handle(), cont) }
    }
    #[fixed_stack_segment]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintEvent_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
}

struct wxcPrintoutHandler(*u8);
impl _wxcPrintoutHandler for wxcPrintoutHandler {}
impl _wxEvtHandler for wxcPrintoutHandler {}
impl _wxObject for wxcPrintoutHandler { fn handle(&self) -> *u8 { **self } }

impl wxcPrintoutHandler {
    pub fn from(handle: *u8) -> @_wxcPrintoutHandler {
        @wxcPrintoutHandler(handle) as @_wxcPrintoutHandler
    }
    
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
    pub fn from(handle: *u8) -> @_wxStyledTextCtrl {
        @wxStyledTextCtrl(handle) as @_wxStyledTextCtrl
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _txt: @_wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> @_wxStyledTextCtrl {
        unsafe { @wxStyledTextCtrl(wxStyledTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, style)) as @_wxStyledTextCtrl }
    }
}

trait _wxStyledTextCtrl : _wxControl {
    #[fixed_stack_segment]
    fn addText(&self, text: @_wxString) {
        unsafe { wxStyledTextCtrl_AddText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addStyledText(&self, data: @_wxMemoryBuffer) {
        unsafe { wxStyledTextCtrl_AddStyledText(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn insertText(&self, pos: c_int, text: @_wxString) {
        unsafe { wxStyledTextCtrl_InsertText(self.handle(), pos, text.handle()) }
    }
    #[fixed_stack_segment]
    fn clearAll(&self) {
        unsafe { wxStyledTextCtrl_ClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearDocumentStyle(&self) {
        unsafe { wxStyledTextCtrl_ClearDocumentStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLength(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLength(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCharAt(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetCharAt(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn getCurrentPos(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentPos(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAnchor(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetAnchor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyleAt(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetStyleAt(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn redo(&self) {
        unsafe { wxStyledTextCtrl_Redo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setUndoCollection(&self, collectUndo: bool) {
        unsafe { wxStyledTextCtrl_SetUndoCollection(self.handle(), collectUndo) }
    }
    #[fixed_stack_segment]
    fn selectAll(&self) {
        unsafe { wxStyledTextCtrl_SelectAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSavePoint(&self) {
        unsafe { wxStyledTextCtrl_SetSavePoint(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canRedo(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanRedo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn markerLineFromHandle(&self, handle: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerLineFromHandle(self.handle(), handle) }
    }
    #[fixed_stack_segment]
    fn markerDeleteHandle(&self, handle: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteHandle(self.handle(), handle) }
    }
    #[fixed_stack_segment]
    fn getUndoCollection(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUndoCollection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getViewWhiteSpace(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetViewWhiteSpace(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setViewWhiteSpace(&self, viewWS: c_int) {
        unsafe { wxStyledTextCtrl_SetViewWhiteSpace(self.handle(), viewWS) }
    }
    #[fixed_stack_segment]
    fn positionFromPoint(&self, pt_x: c_int, pt_y: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromPoint(self.handle(), pt_x, pt_y) }
    }
    #[fixed_stack_segment]
    fn positionFromPointClose(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromPointClose(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn gotoLine(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_GotoLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn gotoPos(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_GotoPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setAnchor(&self, posAnchor: c_int) {
        unsafe { wxStyledTextCtrl_SetAnchor(self.handle(), posAnchor) }
    }
    #[fixed_stack_segment]
    fn getEndStyled(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEndStyled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn convertEOLs(&self, eolMode: c_int) {
        unsafe { wxStyledTextCtrl_ConvertEOLs(self.handle(), eolMode) }
    }
    #[fixed_stack_segment]
    fn getEOLMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEOLMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEOLMode(&self, eolMode: c_int) {
        unsafe { wxStyledTextCtrl_SetEOLMode(self.handle(), eolMode) }
    }
    #[fixed_stack_segment]
    fn startStyling(&self, pos: c_int, mask: c_int) {
        unsafe { wxStyledTextCtrl_StartStyling(self.handle(), pos, mask) }
    }
    #[fixed_stack_segment]
    fn setStyling(&self, length: c_int, style: c_int) {
        unsafe { wxStyledTextCtrl_SetStyling(self.handle(), length, style) }
    }
    #[fixed_stack_segment]
    fn getBufferedDraw(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetBufferedDraw(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBufferedDraw(&self, buffered: bool) {
        unsafe { wxStyledTextCtrl_SetBufferedDraw(self.handle(), buffered) }
    }
    #[fixed_stack_segment]
    fn setTabWidth(&self, tabWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetTabWidth(self.handle(), tabWidth) }
    }
    #[fixed_stack_segment]
    fn getTabWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTabWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCodePage(&self, codePage: c_int) {
        unsafe { wxStyledTextCtrl_SetCodePage(self.handle(), codePage) }
    }
    #[fixed_stack_segment]
    fn markerDefine(&self, markerNumber: c_int, markerSymbol: c_int, foreground_r: uint8_t, foreground_g: uint8_t, foreground_b: uint8_t, background_r: uint8_t, background_g: uint8_t, background_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerDefine(self.handle(), markerNumber, markerSymbol, foreground_r, foreground_g, foreground_b, background_r, background_g, background_b) }
    }
    #[fixed_stack_segment]
    fn markerSetForeground(&self, markerNumber: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerSetForeground(self.handle(), markerNumber, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn markerSetBackground(&self, markerNumber: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerSetBackground(self.handle(), markerNumber, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn markerAdd(&self, line: c_int, markerNumber: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerAdd(self.handle(), line, markerNumber) }
    }
    #[fixed_stack_segment]
    fn markerDelete(&self, line: c_int, markerNumber: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDelete(self.handle(), line, markerNumber) }
    }
    #[fixed_stack_segment]
    fn markerDeleteAll(&self, markerNumber: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteAll(self.handle(), markerNumber) }
    }
    #[fixed_stack_segment]
    fn markerGet(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerGet(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn markerNext(&self, lineStart: c_int, markerMask: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerNext(self.handle(), lineStart, markerMask) }
    }
    #[fixed_stack_segment]
    fn markerPrevious(&self, lineStart: c_int, markerMask: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerPrevious(self.handle(), lineStart, markerMask) }
    }
    #[fixed_stack_segment]
    fn markerDefineBitmap(&self, markerNumber: c_int, bmp: @_wxBitmap) {
        unsafe { wxStyledTextCtrl_MarkerDefineBitmap(self.handle(), markerNumber, bmp.handle()) }
    }
    #[fixed_stack_segment]
    fn setMarginType(&self, margin: c_int, marginType: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginType(self.handle(), margin, marginType) }
    }
    #[fixed_stack_segment]
    fn getMarginType(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginType(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    fn setMarginWidth(&self, margin: c_int, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginWidth(self.handle(), margin, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getMarginWidth(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginWidth(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    fn setMarginMask(&self, margin: c_int, mask: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginMask(self.handle(), margin, mask) }
    }
    #[fixed_stack_segment]
    fn getMarginMask(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginMask(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    fn setMarginSensitive(&self, margin: c_int, sensitive: bool) {
        unsafe { wxStyledTextCtrl_SetMarginSensitive(self.handle(), margin, sensitive) }
    }
    #[fixed_stack_segment]
    fn getMarginSensitive(&self, margin: c_int) -> bool {
        unsafe { wxStyledTextCtrl_GetMarginSensitive(self.handle(), margin) }
    }
    #[fixed_stack_segment]
    fn styleClearAll(&self) {
        unsafe { wxStyledTextCtrl_StyleClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetForeground(&self, style: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_StyleSetForeground(self.handle(), style, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn styleSetBackground(&self, style: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_StyleSetBackground(self.handle(), style, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn styleSetBold(&self, style: c_int, bold: bool) {
        unsafe { wxStyledTextCtrl_StyleSetBold(self.handle(), style, bold) }
    }
    #[fixed_stack_segment]
    fn styleSetItalic(&self, style: c_int, italic: bool) {
        unsafe { wxStyledTextCtrl_StyleSetItalic(self.handle(), style, italic) }
    }
    #[fixed_stack_segment]
    fn styleSetSize(&self, style: c_int, sizePoints: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetSize(self.handle(), style, sizePoints) }
    }
    #[fixed_stack_segment]
    fn styleSetFaceName(&self, style: c_int, fontName: @_wxString) {
        unsafe { wxStyledTextCtrl_StyleSetFaceName(self.handle(), style, fontName.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetEOLFilled(&self, style: c_int, filled: bool) {
        unsafe { wxStyledTextCtrl_StyleSetEOLFilled(self.handle(), style, filled) }
    }
    #[fixed_stack_segment]
    fn styleResetDefault(&self) {
        unsafe { wxStyledTextCtrl_StyleResetDefault(self.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetUnderline(&self, style: c_int, underline: bool) {
        unsafe { wxStyledTextCtrl_StyleSetUnderline(self.handle(), style, underline) }
    }
    #[fixed_stack_segment]
    fn styleSetCase(&self, style: c_int, caseForce: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetCase(self.handle(), style, caseForce) }
    }
    #[fixed_stack_segment]
    fn styleSetCharacterSet(&self, style: c_int, characterSet: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetCharacterSet(self.handle(), style, characterSet) }
    }
    #[fixed_stack_segment]
    fn styleSetHotSpot(&self, style: c_int, hotspot: bool) {
        unsafe { wxStyledTextCtrl_StyleSetHotSpot(self.handle(), style, hotspot) }
    }
    #[fixed_stack_segment]
    fn setSelForeground(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn setSelBackground(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn setCaretForeground(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretForeground(self.handle(), fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn cmdKeyAssign(&self, key: c_int, modifiers: c_int, cmd: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyAssign(self.handle(), key, modifiers, cmd) }
    }
    #[fixed_stack_segment]
    fn cmdKeyClear(&self, key: c_int, modifiers: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyClear(self.handle(), key, modifiers) }
    }
    #[fixed_stack_segment]
    fn cmdKeyClearAll(&self) {
        unsafe { wxStyledTextCtrl_CmdKeyClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyleBytes(&self, length: c_int, styleBytes: *char) {
        unsafe { wxStyledTextCtrl_SetStyleBytes(self.handle(), length, styleBytes) }
    }
    #[fixed_stack_segment]
    fn styleSetVisible(&self, style: c_int, visible: bool) {
        unsafe { wxStyledTextCtrl_StyleSetVisible(self.handle(), style, visible) }
    }
    #[fixed_stack_segment]
    fn getCaretPeriod(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretPeriod(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaretPeriod(&self, periodMilliseconds: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretPeriod(self.handle(), periodMilliseconds) }
    }
    #[fixed_stack_segment]
    fn setWordChars(&self, characters: @_wxString) {
        unsafe { wxStyledTextCtrl_SetWordChars(self.handle(), characters.handle()) }
    }
    #[fixed_stack_segment]
    fn beginUndoAction(&self) {
        unsafe { wxStyledTextCtrl_BeginUndoAction(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endUndoAction(&self) {
        unsafe { wxStyledTextCtrl_EndUndoAction(self.handle()) }
    }
    #[fixed_stack_segment]
    fn indicatorSetStyle(&self, indic: c_int, style: c_int) {
        unsafe { wxStyledTextCtrl_IndicatorSetStyle(self.handle(), indic, style) }
    }
    #[fixed_stack_segment]
    fn indicatorGetStyle(&self, indic: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_IndicatorGetStyle(self.handle(), indic) }
    }
    #[fixed_stack_segment]
    fn indicatorSetForeground(&self, indic: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_IndicatorSetForeground(self.handle(), indic, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn setWhitespaceForeground(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn setWhitespaceBackground(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn setStyleBits(&self, bits: c_int) {
        unsafe { wxStyledTextCtrl_SetStyleBits(self.handle(), bits) }
    }
    #[fixed_stack_segment]
    fn getStyleBits(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetStyleBits(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLineState(&self, line: c_int, state: c_int) {
        unsafe { wxStyledTextCtrl_SetLineState(self.handle(), line, state) }
    }
    #[fixed_stack_segment]
    fn getLineState(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineState(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn getMaxLineState(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMaxLineState(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCaretLineVisible(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetCaretLineVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaretLineVisible(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetCaretLineVisible(self.handle(), show) }
    }
    #[fixed_stack_segment]
    fn styleSetChangeable(&self, style: c_int, changeable: bool) {
        unsafe { wxStyledTextCtrl_StyleSetChangeable(self.handle(), style, changeable) }
    }
    #[fixed_stack_segment]
    fn autoCompShow(&self, lenEntered: c_int, itemList: @_wxString) {
        unsafe { wxStyledTextCtrl_AutoCompShow(self.handle(), lenEntered, itemList.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompCancel(&self) {
        unsafe { wxStyledTextCtrl_AutoCompCancel(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompActive(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompActive(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompPosStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompPosStart(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompComplete(&self) {
        unsafe { wxStyledTextCtrl_AutoCompComplete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompStops(&self, characterSet: @_wxString) {
        unsafe { wxStyledTextCtrl_AutoCompStops(self.handle(), characterSet.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetSeparator(&self, separatorCharacter: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetSeparator(self.handle(), separatorCharacter) }
    }
    #[fixed_stack_segment]
    fn autoCompGetSeparator(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSelect(&self, text: @_wxString) {
        unsafe { wxStyledTextCtrl_AutoCompSelect(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetCancelAtStart(&self, cancel: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetCancelAtStart(self.handle(), cancel) }
    }
    #[fixed_stack_segment]
    fn autoCompGetCancelAtStart(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetCancelAtStart(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetFillUps(&self, characterSet: @_wxString) {
        unsafe { wxStyledTextCtrl_AutoCompSetFillUps(self.handle(), characterSet.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetChooseSingle(&self, chooseSingle: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetChooseSingle(self.handle(), chooseSingle) }
    }
    #[fixed_stack_segment]
    fn autoCompGetChooseSingle(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetChooseSingle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetIgnoreCase(&self, ignoreCase: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetIgnoreCase(self.handle(), ignoreCase) }
    }
    #[fixed_stack_segment]
    fn autoCompGetIgnoreCase(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetIgnoreCase(self.handle()) }
    }
    #[fixed_stack_segment]
    fn userListShow(&self, listType: c_int, itemList: @_wxString) {
        unsafe { wxStyledTextCtrl_UserListShow(self.handle(), listType, itemList.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetAutoHide(&self, autoHide: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetAutoHide(self.handle(), autoHide) }
    }
    #[fixed_stack_segment]
    fn autoCompGetAutoHide(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetAutoHide(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetDropRestOfWord(&self, dropRestOfWord: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetDropRestOfWord(self.handle(), dropRestOfWord) }
    }
    #[fixed_stack_segment]
    fn autoCompGetDropRestOfWord(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetDropRestOfWord(self.handle()) }
    }
    #[fixed_stack_segment]
    fn registerImage(&self, type_: c_int, bmp: @_wxBitmap) {
        unsafe { wxStyledTextCtrl_RegisterImage(self.handle(), type_, bmp.handle()) }
    }
    #[fixed_stack_segment]
    fn clearRegisteredImages(&self) {
        unsafe { wxStyledTextCtrl_ClearRegisteredImages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompGetTypeSeparator(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetTypeSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn autoCompSetTypeSeparator(&self, separatorCharacter: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetTypeSeparator(self.handle(), separatorCharacter) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indentSize: c_int) {
        unsafe { wxStyledTextCtrl_SetIndent(self.handle(), indentSize) }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setUseTabs(&self, useTabs: bool) {
        unsafe { wxStyledTextCtrl_SetUseTabs(self.handle(), useTabs) }
    }
    #[fixed_stack_segment]
    fn getUseTabs(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseTabs(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLineIndentation(&self, line: c_int, indentSize: c_int) {
        unsafe { wxStyledTextCtrl_SetLineIndentation(self.handle(), line, indentSize) }
    }
    #[fixed_stack_segment]
    fn getLineIndentation(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineIndentation(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn getLineIndentPosition(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineIndentPosition(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetColumn(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setUseHorizontalScrollBar(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetUseHorizontalScrollBar(self.handle(), show) }
    }
    #[fixed_stack_segment]
    fn getUseHorizontalScrollBar(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseHorizontalScrollBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setIndentationGuides(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetIndentationGuides(self.handle(), show) }
    }
    #[fixed_stack_segment]
    fn getIndentationGuides(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetIndentationGuides(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setHighlightGuide(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_SetHighlightGuide(self.handle(), column) }
    }
    #[fixed_stack_segment]
    fn getHighlightGuide(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetHighlightGuide(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLineEndPosition(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineEndPosition(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn getCodePage(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCodePage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getReadOnly(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetReadOnly(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrentPos(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetCurrentPos(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setSelectionStart(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetSelectionStart(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn getSelectionStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSelectionStart(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionEnd(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetSelectionEnd(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn getSelectionEnd(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSelectionEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintMagnification(&self, magnification: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintMagnification(self.handle(), magnification) }
    }
    #[fixed_stack_segment]
    fn getPrintMagnification(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintMagnification(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintColourMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintColourMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn getPrintColourMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintColourMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn findText(&self, minPos: c_int, maxPos: c_int, text: @_wxString, flags: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_FindText(self.handle(), minPos, maxPos, text.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn formatRange(&self, doDraw: bool, startPos: c_int, endPos: c_int, draw: @_wxDC, target: @_wxDC, renderRect: @_wxRect, pageRect: @_wxRect) -> c_int {
        unsafe { wxStyledTextCtrl_FormatRange(self.handle(), doDraw, startPos, endPos, draw.handle(), target.handle(), renderRect.handle(), pageRect.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetFirstVisibleLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLineCount(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMarginLeft(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginLeft(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getMarginLeft(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginLeft(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMarginRight(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginRight(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getMarginRight(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginRight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getModify(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetModify(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_SetSelection(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    fn hideSelection(&self, normal: bool) {
        unsafe { wxStyledTextCtrl_HideSelection(self.handle(), normal) }
    }
    #[fixed_stack_segment]
    fn lineFromPosition(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_LineFromPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn positionFromLine(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn lineScroll(&self, columns: c_int, lines: c_int) {
        unsafe { wxStyledTextCtrl_LineScroll(self.handle(), columns, lines) }
    }
    #[fixed_stack_segment]
    fn ensureCaretVisible(&self) {
        unsafe { wxStyledTextCtrl_EnsureCaretVisible(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceSelection(&self, text: @_wxString) {
        unsafe { wxStyledTextCtrl_ReplaceSelection(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, readOnly: bool) {
        unsafe { wxStyledTextCtrl_SetReadOnly(self.handle(), readOnly) }
    }
    #[fixed_stack_segment]
    fn canPaste(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanPaste(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanUndo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn emptyUndoBuffer(&self) {
        unsafe { wxStyledTextCtrl_EmptyUndoBuffer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn undo(&self) {
        unsafe { wxStyledTextCtrl_Undo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn cut(&self) {
        unsafe { wxStyledTextCtrl_Cut(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copy(&self) {
        unsafe { wxStyledTextCtrl_Copy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paste(&self) {
        unsafe { wxStyledTextCtrl_Paste(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxStyledTextCtrl_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @_wxString) {
        unsafe { wxStyledTextCtrl_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextLength(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTextLength(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setOvertype(&self, overtype: bool) {
        unsafe { wxStyledTextCtrl_SetOvertype(self.handle(), overtype) }
    }
    #[fixed_stack_segment]
    fn getOvertype(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetOvertype(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaretWidth(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretWidth(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getCaretWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setTargetStart(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetTargetStart(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn getTargetStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTargetStart(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setTargetEnd(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetTargetEnd(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn getTargetEnd(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTargetEnd(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceTarget(&self, text: @_wxString) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceTargetRE(&self, text: @_wxString) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchInTarget(&self, text: @_wxString) -> c_int {
        unsafe { wxStyledTextCtrl_SearchInTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setSearchFlags(&self, flags: c_int) {
        unsafe { wxStyledTextCtrl_SetSearchFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn getSearchFlags(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSearchFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn callTipShow(&self, pos: c_int, definition: @_wxString) {
        unsafe { wxStyledTextCtrl_CallTipShow(self.handle(), pos, definition.handle()) }
    }
    #[fixed_stack_segment]
    fn callTipCancel(&self) {
        unsafe { wxStyledTextCtrl_CallTipCancel(self.handle()) }
    }
    #[fixed_stack_segment]
    fn callTipActive(&self) -> bool {
        unsafe { wxStyledTextCtrl_CallTipActive(self.handle()) }
    }
    #[fixed_stack_segment]
    fn callTipPosAtStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CallTipPosAtStart(self.handle()) }
    }
    #[fixed_stack_segment]
    fn callTipSetHighlight(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_CallTipSetHighlight(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    fn callTipSetBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn callTipSetForeground(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetForeground(self.handle(), fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn callTipSetForegroundHighlight(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetForegroundHighlight(self.handle(), fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn visibleFromDocLine(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_VisibleFromDocLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn docLineFromVisible(&self, lineDisplay: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_DocLineFromVisible(self.handle(), lineDisplay) }
    }
    #[fixed_stack_segment]
    fn setFoldLevel(&self, line: c_int, level: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldLevel(self.handle(), line, level) }
    }
    #[fixed_stack_segment]
    fn getFoldLevel(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldLevel(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn getLastChild(&self, line: c_int, level: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLastChild(self.handle(), line, level) }
    }
    #[fixed_stack_segment]
    fn getFoldParent(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldParent(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn showLines(&self, lineStart: c_int, lineEnd: c_int) {
        unsafe { wxStyledTextCtrl_ShowLines(self.handle(), lineStart, lineEnd) }
    }
    #[fixed_stack_segment]
    fn hideLines(&self, lineStart: c_int, lineEnd: c_int) {
        unsafe { wxStyledTextCtrl_HideLines(self.handle(), lineStart, lineEnd) }
    }
    #[fixed_stack_segment]
    fn getLineVisible(&self, line: c_int) -> bool {
        unsafe { wxStyledTextCtrl_GetLineVisible(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn setFoldExpanded(&self, line: c_int, expanded: bool) {
        unsafe { wxStyledTextCtrl_SetFoldExpanded(self.handle(), line, expanded) }
    }
    #[fixed_stack_segment]
    fn getFoldExpanded(&self, line: c_int) -> bool {
        unsafe { wxStyledTextCtrl_GetFoldExpanded(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn toggleFold(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_ToggleFold(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_EnsureVisible(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn setFoldFlags(&self, flags: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn ensureVisibleEnforcePolicy(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_EnsureVisibleEnforcePolicy(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn setTabIndents(&self, tabIndents: bool) {
        unsafe { wxStyledTextCtrl_SetTabIndents(self.handle(), tabIndents) }
    }
    #[fixed_stack_segment]
    fn getTabIndents(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetTabIndents(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackSpaceUnIndents(&self, bsUnIndents: bool) {
        unsafe { wxStyledTextCtrl_SetBackSpaceUnIndents(self.handle(), bsUnIndents) }
    }
    #[fixed_stack_segment]
    fn getBackSpaceUnIndents(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetBackSpaceUnIndents(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMouseDwellTime(&self, periodMilliseconds: c_int) {
        unsafe { wxStyledTextCtrl_SetMouseDwellTime(self.handle(), periodMilliseconds) }
    }
    #[fixed_stack_segment]
    fn getMouseDwellTime(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMouseDwellTime(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wordStartPosition(&self, pos: c_int, onlyWordCharacters: bool) -> c_int {
        unsafe { wxStyledTextCtrl_WordStartPosition(self.handle(), pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    fn wordEndPosition(&self, pos: c_int, onlyWordCharacters: bool) -> c_int {
        unsafe { wxStyledTextCtrl_WordEndPosition(self.handle(), pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    fn setWrapMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetWrapMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn getWrapMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetWrapMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLayoutCache(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetLayoutCache(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn getLayoutCache(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLayoutCache(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setScrollWidth(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetScrollWidth(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getScrollWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetScrollWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn textWidth(&self, style: c_int, text: @_wxString) -> c_int {
        unsafe { wxStyledTextCtrl_TextWidth(self.handle(), style, text.handle()) }
    }
    #[fixed_stack_segment]
    fn setEndAtLastLine(&self, endAtLastLine: bool) {
        unsafe { wxStyledTextCtrl_SetEndAtLastLine(self.handle(), endAtLastLine) }
    }
    #[fixed_stack_segment]
    fn getEndAtLastLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEndAtLastLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn textHeight(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_TextHeight(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn setUseVerticalScrollBar(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetUseVerticalScrollBar(self.handle(), show) }
    }
    #[fixed_stack_segment]
    fn getUseVerticalScrollBar(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseVerticalScrollBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn appendText(&self, text: @_wxString) {
        unsafe { wxStyledTextCtrl_AppendText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn getTwoPhaseDraw(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetTwoPhaseDraw(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setTwoPhaseDraw(&self, twoPhase: bool) {
        unsafe { wxStyledTextCtrl_SetTwoPhaseDraw(self.handle(), twoPhase) }
    }
    #[fixed_stack_segment]
    fn targetFromSelection(&self) {
        unsafe { wxStyledTextCtrl_TargetFromSelection(self.handle()) }
    }
    #[fixed_stack_segment]
    fn linesJoin(&self) {
        unsafe { wxStyledTextCtrl_LinesJoin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn linesSplit(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_LinesSplit(self.handle(), pixelWidth) }
    }
    #[fixed_stack_segment]
    fn setFoldMarginColour(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginColour(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn setFoldMarginHiColour(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginHiColour(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn lineDuplicate(&self) {
        unsafe { wxStyledTextCtrl_LineDuplicate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn homeDisplay(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplay(self.handle()) }
    }
    #[fixed_stack_segment]
    fn homeDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplayExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lineEndDisplay(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplay(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lineEndDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplayExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lineCopy(&self) {
        unsafe { wxStyledTextCtrl_LineCopy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn moveCaretInsideView(&self) {
        unsafe { wxStyledTextCtrl_MoveCaretInsideView(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lineLength(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_LineLength(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn braceHighlight(&self, pos1: c_int, pos2: c_int) {
        unsafe { wxStyledTextCtrl_BraceHighlight(self.handle(), pos1, pos2) }
    }
    #[fixed_stack_segment]
    fn braceBadLight(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_BraceBadLight(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn braceMatch(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_BraceMatch(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn getViewEOL(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetViewEOL(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setViewEOL(&self, visible: bool) {
        unsafe { wxStyledTextCtrl_SetViewEOL(self.handle(), visible) }
    }
    #[fixed_stack_segment]
    fn setDocPointer(&self, docPointer: @_wxSTCDoc) {
        unsafe { wxStyledTextCtrl_SetDocPointer(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    fn setModEventMask(&self, mask: c_int) {
        unsafe { wxStyledTextCtrl_SetModEventMask(self.handle(), mask) }
    }
    #[fixed_stack_segment]
    fn getEdgeColumn(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEdgeColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEdgeColumn(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_SetEdgeColumn(self.handle(), column) }
    }
    #[fixed_stack_segment]
    fn getEdgeMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEdgeMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEdgeMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetEdgeMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn setEdgeColour(&self, edgeColour_r: uint8_t, edgeColour_g: uint8_t, edgeColour_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetEdgeColour(self.handle(), edgeColour_r, edgeColour_g, edgeColour_b) }
    }
    #[fixed_stack_segment]
    fn searchAnchor(&self) {
        unsafe { wxStyledTextCtrl_SearchAnchor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn searchNext(&self, flags: c_int, text: @_wxString) -> c_int {
        unsafe { wxStyledTextCtrl_SearchNext(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchPrev(&self, flags: c_int, text: @_wxString) -> c_int {
        unsafe { wxStyledTextCtrl_SearchPrev(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    fn linesOnScreen(&self) -> c_int {
        unsafe { wxStyledTextCtrl_LinesOnScreen(self.handle()) }
    }
    #[fixed_stack_segment]
    fn usePopUp(&self, allowPopUp: bool) {
        unsafe { wxStyledTextCtrl_UsePopUp(self.handle(), allowPopUp) }
    }
    #[fixed_stack_segment]
    fn selectionIsRectangle(&self) -> bool {
        unsafe { wxStyledTextCtrl_SelectionIsRectangle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, zoom: c_int) {
        unsafe { wxStyledTextCtrl_SetZoom(self.handle(), zoom) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetZoom(self.handle()) }
    }
    #[fixed_stack_segment]
    fn addRefDocument(&self, docPointer: @_wxSTCDoc) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    fn releaseDocument(&self, docPointer: @_wxSTCDoc) {
        unsafe { wxStyledTextCtrl_ReleaseDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    fn getModEventMask(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetModEventMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSTCFocus(&self, focus: bool) {
        unsafe { wxStyledTextCtrl_SetSTCFocus(self.handle(), focus) }
    }
    #[fixed_stack_segment]
    fn getSTCFocus(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetSTCFocus(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatus(&self, statusCode: c_int) {
        unsafe { wxStyledTextCtrl_SetStatus(self.handle(), statusCode) }
    }
    #[fixed_stack_segment]
    fn getStatus(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetStatus(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMouseDownCaptures(&self, captures: bool) {
        unsafe { wxStyledTextCtrl_SetMouseDownCaptures(self.handle(), captures) }
    }
    #[fixed_stack_segment]
    fn getMouseDownCaptures(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetMouseDownCaptures(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setSTCCursor(&self, cursorType: c_int) {
        unsafe { wxStyledTextCtrl_SetSTCCursor(self.handle(), cursorType) }
    }
    #[fixed_stack_segment]
    fn getSTCCursor(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSTCCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setControlCharSymbol(&self, symbol: c_int) {
        unsafe { wxStyledTextCtrl_SetControlCharSymbol(self.handle(), symbol) }
    }
    #[fixed_stack_segment]
    fn getControlCharSymbol(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetControlCharSymbol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wordPartLeft(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeft(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wordPartLeftExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeftExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wordPartRight(&self) {
        unsafe { wxStyledTextCtrl_WordPartRight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wordPartRightExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartRightExtend(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setVisiblePolicy(&self, visiblePolicy: c_int, visibleSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetVisiblePolicy(self.handle(), visiblePolicy, visibleSlop) }
    }
    #[fixed_stack_segment]
    fn delLineLeft(&self) {
        unsafe { wxStyledTextCtrl_DelLineLeft(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delLineRight(&self) {
        unsafe { wxStyledTextCtrl_DelLineRight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setXOffset(&self, newOffset: c_int) {
        unsafe { wxStyledTextCtrl_SetXOffset(self.handle(), newOffset) }
    }
    #[fixed_stack_segment]
    fn getXOffset(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetXOffset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn chooseCaretX(&self) {
        unsafe { wxStyledTextCtrl_ChooseCaretX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setXCaretPolicy(&self, caretPolicy: c_int, caretSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetXCaretPolicy(self.handle(), caretPolicy, caretSlop) }
    }
    #[fixed_stack_segment]
    fn setYCaretPolicy(&self, caretPolicy: c_int, caretSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetYCaretPolicy(self.handle(), caretPolicy, caretSlop) }
    }
    #[fixed_stack_segment]
    fn setPrintWrapMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintWrapMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn getPrintWrapMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintWrapMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setHotspotActiveForeground(&self, useSetting: bool, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveForeground(self.handle(), useSetting, fore_r, fore_g, fore_b) }
    }
    #[fixed_stack_segment]
    fn setHotspotActiveBackground(&self, useSetting: bool, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveBackground(self.handle(), useSetting, back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn setHotspotActiveUnderline(&self, underline: bool) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveUnderline(self.handle(), underline) }
    }
    #[fixed_stack_segment]
    fn positionBefore(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionBefore(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn positionAfter(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionAfter(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn copyRange(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_CopyRange(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    fn copyText(&self, length: c_int, text: @_wxString) {
        unsafe { wxStyledTextCtrl_CopyText(self.handle(), length, text.handle()) }
    }
    #[fixed_stack_segment]
    fn startRecord(&self) {
        unsafe { wxStyledTextCtrl_StartRecord(self.handle()) }
    }
    #[fixed_stack_segment]
    fn stopRecord(&self) {
        unsafe { wxStyledTextCtrl_StopRecord(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLexer(&self, lexer: c_int) {
        unsafe { wxStyledTextCtrl_SetLexer(self.handle(), lexer) }
    }
    #[fixed_stack_segment]
    fn getLexer(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLexer(self.handle()) }
    }
    #[fixed_stack_segment]
    fn colourise(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_Colourise(self.handle(), start, end) }
    }
    #[fixed_stack_segment]
    fn setProperty(&self, key: @_wxString, value: @_wxString) {
        unsafe { wxStyledTextCtrl_SetProperty(self.handle(), key.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setKeyWords(&self, keywordSet: c_int, keyWords: @_wxString) {
        unsafe { wxStyledTextCtrl_SetKeyWords(self.handle(), keywordSet, keyWords.handle()) }
    }
    #[fixed_stack_segment]
    fn setLexerLanguage(&self, language: @_wxString) {
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self.handle(), language.handle()) }
    }
    #[fixed_stack_segment]
    fn getCurrentLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetSpec(&self, styleNum: c_int, spec: @_wxString) {
        unsafe { wxStyledTextCtrl_StyleSetSpec(self.handle(), styleNum, spec.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFont(&self, styleNum: c_int, font: @_wxFont) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self.handle(), styleNum, font.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFontAttr(&self, styleNum: c_int, size: c_int, faceName: @_wxString, bold: bool, italic: bool, underline: bool) {
        unsafe { wxStyledTextCtrl_StyleSetFontAttr(self.handle(), styleNum, size, faceName.handle(), bold, italic, underline) }
    }
    #[fixed_stack_segment]
    fn cmdKeyExecute(&self, cmd: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyExecute(self.handle(), cmd) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, left: c_int, right: c_int) {
        unsafe { wxStyledTextCtrl_SetMargins(self.handle(), left, right) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, startPos: *c_int, endPos: *c_int) {
        unsafe { wxStyledTextCtrl_GetSelection(self.handle(), startPos, endPos) }
    }
    #[fixed_stack_segment]
    fn scrollToLine(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_ScrollToLine(self.handle(), line) }
    }
    #[fixed_stack_segment]
    fn scrollToColumn(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_ScrollToColumn(self.handle(), column) }
    }
    #[fixed_stack_segment]
    fn setVScrollBar(&self, bar: @_wxScrollBar) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    fn setHScrollBar(&self, bar: @_wxScrollBar) {
        unsafe { wxStyledTextCtrl_SetHScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastKeydownProcessed(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetLastKeydownProcessed(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLastKeydownProcessed(&self, val: bool) {
        unsafe { wxStyledTextCtrl_SetLastKeydownProcessed(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, filename: @_wxString) -> bool {
        unsafe { wxStyledTextCtrl_SaveFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, filename: @_wxString) -> bool {
        unsafe { wxStyledTextCtrl_LoadFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn indicatorGetForeground(&self, indic: c_int) -> @_wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_IndicatorGetForeground(self.handle(), indic)) as @_wxColour }
    }
    #[fixed_stack_segment]
    fn getCaretLineBackground(&self) -> @_wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_GetCaretLineBackground(self.handle())) as @_wxColour }
    }
    #[fixed_stack_segment]
    fn setCaretLineBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn getCaretForeground(&self) -> @_wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_GetCaretForeground(self.handle())) as @_wxColour }
    }
    #[fixed_stack_segment]
    fn getLine(&self, line: c_int) -> @_wxString {
        unsafe { @wxString(wxStyledTextCtrl_GetLine(self.handle(), line)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @_wxString {
        unsafe { @wxString(wxStyledTextCtrl_GetText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getTextRange(&self, startPos: c_int, endPos: c_int) -> @_wxString {
        unsafe { @wxString(wxStyledTextCtrl_GetTextRange(self.handle(), startPos, endPos)) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getSelectedText(&self) -> @_wxString {
        unsafe { @wxString(wxStyledTextCtrl_GetSelectedText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn newDocument(&self) -> @_wxSTCDoc {
        unsafe { @wxSTCDoc(wxStyledTextCtrl_CreateDocument(self.handle())) as @_wxSTCDoc }
    }
    #[fixed_stack_segment]
    fn getEdgeColour(&self) -> @_wxColour {
        unsafe { @wxColour(wxStyledTextCtrl_GetEdgeColour(self.handle())) as @_wxColour }
    }
    #[fixed_stack_segment]
    fn getDocPointer(&self) -> @_wxSTCDoc {
        unsafe { @wxSTCDoc(wxStyledTextCtrl_GetDocPointer(self.handle())) as @_wxSTCDoc }
    }
    #[fixed_stack_segment]
    fn pointFromPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxStyledTextCtrl_PointFromPosition(self.handle())) as @_wxPoint }
    }
}

struct wxSTCDoc(*u8);
impl _wxSTCDoc for wxSTCDoc { fn handle(&self) -> *u8 { **self } }

impl wxSTCDoc {
    pub fn from(handle: *u8) -> @_wxSTCDoc {
        @wxSTCDoc(handle) as @_wxSTCDoc
    }
    
}

trait _wxSTCDoc {
    fn handle(&self) -> *u8;
    
}

struct wxMemoryBuffer(*u8);
impl _wxMemoryBuffer for wxMemoryBuffer { fn handle(&self) -> *u8 { **self } }

impl wxMemoryBuffer {
    pub fn from(handle: *u8) -> @_wxMemoryBuffer {
        @wxMemoryBuffer(handle) as @_wxMemoryBuffer
    }
    
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
    pub fn from(handle: *u8) -> @_wxStyledTextEvent {
        @wxStyledTextEvent(handle) as @_wxStyledTextEvent
    }
    
}

trait _wxStyledTextEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getKey(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetKey(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getModifiers(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetModifiers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getModificationType(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetModificationType(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLength(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLength(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLinesAdded(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLinesAdded(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLine(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFoldLevelNow(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetFoldLevelNow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFoldLevelPrev(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetFoldLevelPrev(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMargin(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetMargin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMessage(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetMessage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWParam(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetWParam(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLParam(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLParam(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getListType(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetListType(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDragText(&self) -> @_wxString {
        unsafe { @wxString(wxStyledTextEvent_GetDragText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getDragAllowMove(&self) -> bool {
        unsafe { wxStyledTextEvent_GetDragAllowMove(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDragResult(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetDragResult(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getShift(&self) -> bool {
        unsafe { wxStyledTextEvent_GetShift(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> bool {
        unsafe { wxStyledTextEvent_GetControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAlt(&self) -> bool {
        unsafe { wxStyledTextEvent_GetAlt(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @_wxString {
        unsafe { @wxString(wxStyledTextEvent_GetText(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @_wxStyledTextEvent {
        unsafe { @wxStyledTextEvent(wxStyledTextEvent_Clone(self.handle())) as @_wxStyledTextEvent }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxStyledTextEvent_SetPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setKey(&self, k: c_int) {
        unsafe { wxStyledTextEvent_SetKey(self.handle(), k) }
    }
    #[fixed_stack_segment]
    fn setModifiers(&self, m: c_int) {
        unsafe { wxStyledTextEvent_SetModifiers(self.handle(), m) }
    }
    #[fixed_stack_segment]
    fn setModificationType(&self, t: c_int) {
        unsafe { wxStyledTextEvent_SetModificationType(self.handle(), t) }
    }
    #[fixed_stack_segment]
    fn setText(&self, t: @_wxString) {
        unsafe { wxStyledTextEvent_SetText(self.handle(), t.handle()) }
    }
    #[fixed_stack_segment]
    fn setLength(&self, len: c_int) {
        unsafe { wxStyledTextEvent_SetLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    fn setLinesAdded(&self, num: c_int) {
        unsafe { wxStyledTextEvent_SetLinesAdded(self.handle(), num) }
    }
    #[fixed_stack_segment]
    fn setLine(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetLine(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setFoldLevelNow(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetFoldLevelNow(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setFoldLevelPrev(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetFoldLevelPrev(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setMargin(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetMargin(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetMessage(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setWParam(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetWParam(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setLParam(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetLParam(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setListType(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetListType(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setX(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetX(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setY(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetY(self.handle(), val) }
    }
    #[fixed_stack_segment]
    fn setDragText(&self, val: @_wxString) {
        unsafe { wxStyledTextEvent_SetDragText(self.handle(), val.handle()) }
    }
    #[fixed_stack_segment]
    fn setDragAllowMove(&self, val: bool) {
        unsafe { wxStyledTextEvent_SetDragAllowMove(self.handle(), val) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxGauge95 {
        @wxGauge95(handle) as @_wxGauge95
    }
    
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
    pub fn from(handle: *u8) -> @_wxGaugeMSW {
        @wxGaugeMSW(handle) as @_wxGaugeMSW
    }
    
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
    pub fn from(handle: *u8) -> @_wxSlider95 {
        @wxSlider95(handle) as @_wxSlider95
    }
    
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
    pub fn from(handle: *u8) -> @_wxSliderMSW {
        @wxSliderMSW(handle) as @_wxSliderMSW
    }
    
}

trait _wxSliderMSW : _wxSlider {
}

struct wxcTreeItemData(*u8);
impl _wxcTreeItemData for wxcTreeItemData {}
impl _wxTreeItemData for wxcTreeItemData {}
impl _wxClientData for wxcTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxcTreeItemData {
    pub fn from(handle: *u8) -> @_wxcTreeItemData {
        @wxcTreeItemData(handle) as @_wxcTreeItemData
    }
    
    #[fixed_stack_segment]
    pub fn new(closure: @_wxClosure) -> @_wxcTreeItemData {
        unsafe { @wxcTreeItemData(wxcTreeItemData_Create(closure.handle())) as @_wxcTreeItemData }
    }
}

trait _wxcTreeItemData : _wxTreeItemData {
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @_wxClosure {
        unsafe { @wxClosure(wxcTreeItemData_GetClientClosure(self.handle())) as @_wxClosure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @_wxClosure) {
        unsafe { wxcTreeItemData_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxInputSink(*u8);
impl _wxInputSink for wxInputSink {}
impl _wxThread for wxInputSink { fn handle(&self) -> *u8 { **self } }

impl wxInputSink {
    pub fn from(handle: *u8) -> @_wxInputSink {
        @wxInputSink(handle) as @_wxInputSink
    }
    
    #[fixed_stack_segment]
    pub fn new(input: @_wxInputStream, evtHandler: @_wxEvtHandler, bufferLen: c_int) -> @_wxInputSink {
        unsafe { @wxInputSink(wxInputSink_Create(input.handle(), evtHandler.handle(), bufferLen)) as @_wxInputSink }
    }
}

trait _wxInputSink : _wxThread {
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn start(&self) {
        unsafe { wxInputSink_Start(self.handle()) }
    }
}

struct wxInputSinkEvent(*u8);
impl _wxInputSinkEvent for wxInputSinkEvent {}
impl _wxEvent for wxInputSinkEvent {}
impl _wxObject for wxInputSinkEvent { fn handle(&self) -> *u8 { **self } }

impl wxInputSinkEvent {
    pub fn from(handle: *u8) -> @_wxInputSinkEvent {
        @wxInputSinkEvent(handle) as @_wxInputSinkEvent
    }
    
}

trait _wxInputSinkEvent : _wxEvent {
    #[fixed_stack_segment]
    fn lastError(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastError(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lastRead(&self) -> c_int {
        unsafe { wxInputSinkEvent_LastRead(self.handle()) }
    }
    #[fixed_stack_segment]
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
    pub fn from(handle: *u8) -> @_wxcHtmlEvent {
        @wxcHtmlEvent(handle) as @_wxcHtmlEvent
    }
    
}

trait _wxcHtmlEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getMouseEvent(&self) -> @_wxMouseEvent {
        unsafe { @wxMouseEvent(wxcHtmlEvent_GetMouseEvent(self.handle())) as @_wxMouseEvent }
    }
    #[fixed_stack_segment]
    fn getHtmlCell(&self) -> @_wxHtmlCell {
        unsafe { @wxHtmlCell(wxcHtmlEvent_GetHtmlCell(self.handle())) as @_wxHtmlCell }
    }
    #[fixed_stack_segment]
    fn getHtmlCellId(&self) -> @_wxString {
        unsafe { @wxString(wxcHtmlEvent_GetHtmlCellId(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getHref(&self) -> @_wxString {
        unsafe { @wxString(wxcHtmlEvent_GetHref(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @_wxString {
        unsafe { @wxString(wxcHtmlEvent_GetTarget(self.handle())) as @_wxString }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self) -> @_wxPoint {
        unsafe { @wxPoint(wxcHtmlEvent_GetLogicalPosition(self.handle())) as @_wxPoint }
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
    pub fn from(handle: *u8) -> @_wxcHtmlWindow {
        @wxcHtmlWindow(handle) as @_wxcHtmlWindow
    }
    
    #[fixed_stack_segment]
    pub fn new(_prt: @_wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: @_wxString) -> @_wxcHtmlWindow {
        unsafe { @wxcHtmlWindow(wxcHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) as @_wxcHtmlWindow }
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
    pub fn from(handle: *u8) -> @_wxGridCellTextEnterEditor {
        @wxGridCellTextEnterEditor(handle) as @_wxGridCellTextEnterEditor
    }
    
    #[fixed_stack_segment]
    pub fn ctor() -> @_wxGridCellTextEnterEditor {
        unsafe { @wxGridCellTextEnterEditor(wxGridCellTextEnterEditor_Ctor()) as @_wxGridCellTextEnterEditor }
    }
}

trait _wxGridCellTextEnterEditor : _wxGridCellTextEditor {
}

struct wxFileConfig(*u8);
impl _wxFileConfig for wxFileConfig {}
impl _wxConfigBase for wxFileConfig { fn handle(&self) -> *u8 { **self } }

impl wxFileConfig {
    pub fn from(handle: *u8) -> @_wxFileConfig {
        @wxFileConfig(handle) as @_wxFileConfig
    }
    
    #[fixed_stack_segment]
    pub fn new(inp: @_wxInputStream) -> @_wxFileConfig {
        unsafe { @wxFileConfig(wxFileConfig_Create(inp.handle())) as @_wxFileConfig }
    }
}

trait _wxFileConfig : _wxConfigBase {
}

