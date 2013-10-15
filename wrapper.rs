use std::libc::*;
use native::*;

struct ELJApp(*u8);
impl _ELJApp for ELJApp {}
impl _wxApp for ELJApp {}
impl _wxEvtHandler for ELJApp {}
impl _wxObject for ELJApp { fn handle(&self) -> *u8 { **self } }

impl ELJApp {
    #[fixed_stack_segment]
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    pub fn newLogTarget() -> ELJLog {
        unsafe { ELJLog(ELJApp_CreateLogTarget()) }
    }
    #[fixed_stack_segment]
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    pub fn displaySize() -> wxSize {
        unsafe { wxSize(ELJApp_DisplaySize()) }
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
    pub fn executeProcess<T: _wxString, U: _wxProcess>(_cmd: T, _snc: c_int, _prc: U) -> c_int {
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
    pub fn findWindowById<T: _wxWindow>(_id: c_int, _prt: T) -> *u8 {
        unsafe { ELJApp_FindWindowById(_id, _prt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn findWindowByLabel<T: _wxString, U: _wxWindow>(_lbl: T, _prt: U) -> wxWindow {
        unsafe { wxWindow(ELJApp_FindWindowByLabel(_lbl.handle(), _prt.handle())) }
    }
    #[fixed_stack_segment]
    pub fn findWindowByName<T: _wxString, U: _wxWindow>(_lbl: T, _prt: U) -> wxWindow {
        unsafe { wxWindow(ELJApp_FindWindowByName(_lbl.handle(), _prt.handle())) }
    }
    #[fixed_stack_segment]
    pub fn getApp() -> wxApp {
        unsafe { wxApp(ELJApp_GetApp()) }
    }
    #[fixed_stack_segment]
    pub fn getAppName() -> wxString {
        unsafe { wxString(ELJApp_GetAppName()) }
    }
    #[fixed_stack_segment]
    pub fn getClassName() -> wxString {
        unsafe { wxString(ELJApp_GetClassName()) }
    }
    #[fixed_stack_segment]
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    pub fn getOsDescription() -> wxString {
        unsafe { wxString(ELJApp_GetOsDescription()) }
    }
    #[fixed_stack_segment]
    pub fn getOsVersion(_maj: *u8, _min: *u8) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    pub fn getTopWindow() -> wxWindow {
        unsafe { wxWindow(ELJApp_GetTopWindow()) }
    }
    #[fixed_stack_segment]
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    pub fn getUserHome(_usr: *u8) -> wxString {
        unsafe { wxString(ELJApp_GetUserHome(_usr)) }
    }
    #[fixed_stack_segment]
    pub fn getUserId() -> wxString {
        unsafe { wxString(ELJApp_GetUserId()) }
    }
    #[fixed_stack_segment]
    pub fn getUserName() -> wxString {
        unsafe { wxString(ELJApp_GetUserName()) }
    }
    #[fixed_stack_segment]
    pub fn getVendorName() -> wxString {
        unsafe { wxString(ELJApp_GetVendorName()) }
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
    pub fn mousePosition() -> wxPoint {
        unsafe { wxPoint(ELJApp_MousePosition()) }
    }
    #[fixed_stack_segment]
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    pub fn safeYield<T: _wxWindow>(_win: T) -> c_int {
        unsafe { ELJApp_SafeYield(_win.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setAppName<T: _wxString>(name: T) {
        unsafe { ELJApp_SetAppName(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setClassName<T: _wxString>(name: T) {
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
    pub fn setTopWindow<T: _wxWindow>(_wnd: T) {
        unsafe { ELJApp_SetTopWindow(_wnd.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    #[fixed_stack_segment]
    pub fn setVendorName<T: _wxString>(name: T) {
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
    pub fn initializeC<T: _wxClosure>(closure: T, _argc: c_int, _argv: *wchar_t) {
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
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _clb: *u8) -> ELJArtProv {
        unsafe { ELJArtProv(ELJArtProv_Create(_obj, _clb)) }
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
}

trait _ELJClient : _wxClient {
}

struct ELJCommand(*u8);
impl _ELJCommand for ELJCommand {}
impl _wxCommand for ELJCommand {}
impl _wxObject for ELJCommand { fn handle(&self) -> *u8 { **self } }

impl ELJCommand {
}

trait _ELJCommand : _wxCommand {
}

struct ELJConnection(*u8);
impl _ELJConnection for ELJConnection {}
impl _wxConnection for ELJConnection {}
impl _wxConnectionBase for ELJConnection {}
impl _wxObject for ELJConnection { fn handle(&self) -> *u8 { **self } }

impl ELJConnection {
}

trait _ELJConnection : _wxConnection {
}

struct ELJDragDataObject(*u8);
impl _ELJDragDataObject for ELJDragDataObject { fn handle(&self) -> *u8 { **self } }

impl ELJDragDataObject {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(_obj: *u8, _fmt: T, _func1: *u8, _func2: *u8, _func3: *u8) -> ELJDragDataObject {
        unsafe { ELJDragDataObject(ELJDragDataObject_Create(_obj, _fmt.handle(), _func1, _func2, _func3)) }
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
    #[fixed_stack_segment]
    pub fn new(_obj: *u8) -> ELJDropTarget {
        unsafe { ELJDropTarget(ELJDropTarget_Create(_obj)) }
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
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _func: *u8) -> ELJFileDropTarget {
        unsafe { ELJFileDropTarget(ELJFileDropTarget_Create(_obj, _func)) }
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
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _EifGetNumberRows: *u8, _EifGetNumberCols: *u8, _EifGetValue: *u8, _EifSetValue: *u8, _EifIsEmptyCell: *u8, _EifClear: *u8, _EifInsertRows: *u8, _EifAppendRows: *u8, _EifDeleteRows: *u8, _EifInsertCols: *u8, _EifAppendCols: *u8, _EifDeleteCols: *u8, _EifSetRowLabelValue: *u8, _EifSetColLabelValue: *u8, _EifGetRowLabelValue: *u8, _EifGetColLabelValue: *u8) -> ELJGridTable {
        unsafe { ELJGridTable(ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)) }
    }
}

trait _ELJGridTable : _wxGridTableBase {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJGridTable_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getView(&self) -> wxView {
        unsafe { wxView(ELJGridTable_GetView(self.handle())) }
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
}

trait _ELJLocale : _wxLocale {
}

struct ELJLog(*u8);
impl _ELJLog for ELJLog {}
impl _wxLog for ELJLog { fn handle(&self) -> *u8 { **self } }

impl ELJLog {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8) -> ELJLog {
        unsafe { ELJLog(ELJLog_Create(_obj, _fnc)) }
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
    fn isAllowedTraceMask<T: _wxMask>(&self, mask: T) -> bool {
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
}

trait _ELJMessageParameters {
    fn handle(&self) -> *u8;
    
}

struct ELJPlotCurve(*u8);
impl _ELJPlotCurve for ELJPlotCurve {}
impl _wxPlotCurve for ELJPlotCurve {}
impl _wxObject for ELJPlotCurve { fn handle(&self) -> *u8 { **self } }

impl ELJPlotCurve {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(preview: *u8, buttons: c_int, parent: T, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> ELJPreviewControlBar {
        unsafe { ELJPreviewControlBar(ELJPreviewControlBar_Create(preview, buttons, parent.handle(), title, x, y, w, h, style)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_obj: *u8, _init: *u8, _create_canvas: *u8, _create_toolbar: *u8, preview: *u8, parent: T, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> ELJPreviewFrame {
        unsafe { ELJPreviewFrame(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.handle(), title, x, y, w, h, style)) }
    }
}

trait _ELJPreviewFrame : _wxPreviewFrame {
    #[fixed_stack_segment]
    fn getControlBar(&self) -> *u8 {
        unsafe { ELJPreviewFrame_GetControlBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPreviewCanvas(&self) -> wxPreviewCanvas {
        unsafe { wxPreviewCanvas(ELJPreviewFrame_GetPreviewCanvas(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrintPreview(&self) -> wxPrintPreview {
        unsafe { wxPrintPreview(ELJPreviewFrame_GetPrintPreview(self.handle())) }
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
    fn setPreviewCanvas<T: _wxPreviewCanvas>(&self, obj: T) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintPreview<T: _wxPrintPreview>(&self, obj: T) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.handle(), obj.handle()) }
    }
}

struct ELJServer(*u8);
impl _ELJServer for ELJServer {}
impl _wxServer for ELJServer {}
impl _wxServerBase for ELJServer {}
impl _wxObject for ELJServer { fn handle(&self) -> *u8 { **self } }

impl ELJServer {
}

trait _ELJServer : _wxServer {
}

struct ELJTextDropTarget(*u8);
impl _ELJTextDropTarget for ELJTextDropTarget {}
impl _wxTextDropTarget for ELJTextDropTarget {}
impl _wxDropTarget for ELJTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl ELJTextDropTarget {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _func: *u8) -> ELJTextDropTarget {
        unsafe { ELJTextDropTarget(ELJTextDropTarget_Create(_obj, _func)) }
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
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8, _txt: *wchar_t, _stl: c_int) -> ELJTextValidator {
        unsafe { ELJTextValidator(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) }
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
}

trait _cbAntiflickerPlugin : _cbPluginBase {
}

struct cbBarDragPlugin(*u8);
impl _cbBarDragPlugin for cbBarDragPlugin {}
impl _cbPluginBase for cbBarDragPlugin {}
impl _wxEvtHandler for cbBarDragPlugin {}
impl _wxObject for cbBarDragPlugin { fn handle(&self) -> *u8 { **self } }

impl cbBarDragPlugin {
}

trait _cbBarDragPlugin : _cbPluginBase {
}

struct cbBarHintsPlugin(*u8);
impl _cbBarHintsPlugin for cbBarHintsPlugin {}
impl _cbPluginBase for cbBarHintsPlugin {}
impl _wxEvtHandler for cbBarHintsPlugin {}
impl _wxObject for cbBarHintsPlugin { fn handle(&self) -> *u8 { **self } }

impl cbBarHintsPlugin {
}

trait _cbBarHintsPlugin : _cbPluginBase {
}

struct cbBarInfo(*u8);
impl _cbBarInfo for cbBarInfo {}
impl _wxObject for cbBarInfo { fn handle(&self) -> *u8 { **self } }

impl cbBarInfo {
}

trait _cbBarInfo : _wxObject {
}

struct cbBarSpy(*u8);
impl _cbBarSpy for cbBarSpy {}
impl _wxEvtHandler for cbBarSpy {}
impl _wxObject for cbBarSpy { fn handle(&self) -> *u8 { **self } }

impl cbBarSpy {
}

trait _cbBarSpy : _wxEvtHandler {
}

struct cbCloseBox(*u8);
impl _cbCloseBox for cbCloseBox {}
impl _cbMiniButton for cbCloseBox {}
impl _wxObject for cbCloseBox { fn handle(&self) -> *u8 { **self } }

impl cbCloseBox {
}

trait _cbCloseBox : _cbMiniButton {
}

struct cbCollapseBox(*u8);
impl _cbCollapseBox for cbCollapseBox {}
impl _cbMiniButton for cbCollapseBox {}
impl _wxObject for cbCollapseBox { fn handle(&self) -> *u8 { **self } }

impl cbCollapseBox {
}

trait _cbCollapseBox : _cbMiniButton {
}

struct cbCommonPaneProperties(*u8);
impl _cbCommonPaneProperties for cbCommonPaneProperties {}
impl _wxObject for cbCommonPaneProperties { fn handle(&self) -> *u8 { **self } }

impl cbCommonPaneProperties {
}

trait _cbCommonPaneProperties : _wxObject {
}

struct cbCustomizeBarEvent(*u8);
impl _cbCustomizeBarEvent for cbCustomizeBarEvent {}
impl _cbPluginEvent for cbCustomizeBarEvent {}
impl _wxEvent for cbCustomizeBarEvent {}
impl _wxObject for cbCustomizeBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeBarEvent {
}

trait _cbCustomizeBarEvent : _cbPluginEvent {
}

struct cbCustomizeLayoutEvent(*u8);
impl _cbCustomizeLayoutEvent for cbCustomizeLayoutEvent {}
impl _cbPluginEvent for cbCustomizeLayoutEvent {}
impl _wxEvent for cbCustomizeLayoutEvent {}
impl _wxObject for cbCustomizeLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeLayoutEvent {
}

trait _cbCustomizeLayoutEvent : _cbPluginEvent {
}

struct cbDimHandlerBase(*u8);
impl _cbDimHandlerBase for cbDimHandlerBase {}
impl _wxObject for cbDimHandlerBase { fn handle(&self) -> *u8 { **self } }

impl cbDimHandlerBase {
}

trait _cbDimHandlerBase : _wxObject {
}

struct cbDimInfo(*u8);
impl _cbDimInfo for cbDimInfo {}
impl _wxObject for cbDimInfo { fn handle(&self) -> *u8 { **self } }

impl cbDimInfo {
}

trait _cbDimInfo : _wxObject {
}

struct cbDockBox(*u8);
impl _cbDockBox for cbDockBox {}
impl _cbMiniButton for cbDockBox {}
impl _wxObject for cbDockBox { fn handle(&self) -> *u8 { **self } }

impl cbDockBox {
}

trait _cbDockBox : _cbMiniButton {
}

struct cbDockPane(*u8);
impl _cbDockPane for cbDockPane {}
impl _wxObject for cbDockPane { fn handle(&self) -> *u8 { **self } }

impl cbDockPane {
}

trait _cbDockPane : _wxObject {
}

struct cbDrawBarDecorEvent(*u8);
impl _cbDrawBarDecorEvent for cbDrawBarDecorEvent {}
impl _cbPluginEvent for cbDrawBarDecorEvent {}
impl _wxEvent for cbDrawBarDecorEvent {}
impl _wxObject for cbDrawBarDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarDecorEvent {
}

trait _cbDrawBarDecorEvent : _cbPluginEvent {
}

struct cbDrawBarHandlesEvent(*u8);
impl _cbDrawBarHandlesEvent for cbDrawBarHandlesEvent {}
impl _cbPluginEvent for cbDrawBarHandlesEvent {}
impl _wxEvent for cbDrawBarHandlesEvent {}
impl _wxObject for cbDrawBarHandlesEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarHandlesEvent {
}

trait _cbDrawBarHandlesEvent : _cbPluginEvent {
}

struct cbDrawHintRectEvent(*u8);
impl _cbDrawHintRectEvent for cbDrawHintRectEvent {}
impl _cbPluginEvent for cbDrawHintRectEvent {}
impl _wxEvent for cbDrawHintRectEvent {}
impl _wxObject for cbDrawHintRectEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawHintRectEvent {
}

trait _cbDrawHintRectEvent : _cbPluginEvent {
}

struct cbDrawPaneBkGroundEvent(*u8);
impl _cbDrawPaneBkGroundEvent for cbDrawPaneBkGroundEvent {}
impl _cbPluginEvent for cbDrawPaneBkGroundEvent {}
impl _wxEvent for cbDrawPaneBkGroundEvent {}
impl _wxObject for cbDrawPaneBkGroundEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneBkGroundEvent {
}

trait _cbDrawPaneBkGroundEvent : _cbPluginEvent {
}

struct cbDrawPaneDecorEvent(*u8);
impl _cbDrawPaneDecorEvent for cbDrawPaneDecorEvent {}
impl _cbPluginEvent for cbDrawPaneDecorEvent {}
impl _wxEvent for cbDrawPaneDecorEvent {}
impl _wxObject for cbDrawPaneDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneDecorEvent {
}

trait _cbDrawPaneDecorEvent : _cbPluginEvent {
}

struct cbDrawRowBkGroundEvent(*u8);
impl _cbDrawRowBkGroundEvent for cbDrawRowBkGroundEvent {}
impl _cbPluginEvent for cbDrawRowBkGroundEvent {}
impl _wxEvent for cbDrawRowBkGroundEvent {}
impl _wxObject for cbDrawRowBkGroundEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowBkGroundEvent {
}

trait _cbDrawRowBkGroundEvent : _cbPluginEvent {
}

struct cbDrawRowDecorEvent(*u8);
impl _cbDrawRowDecorEvent for cbDrawRowDecorEvent {}
impl _cbPluginEvent for cbDrawRowDecorEvent {}
impl _wxEvent for cbDrawRowDecorEvent {}
impl _wxObject for cbDrawRowDecorEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowDecorEvent {
}

trait _cbDrawRowDecorEvent : _cbPluginEvent {
}

struct cbDrawRowHandlesEvent(*u8);
impl _cbDrawRowHandlesEvent for cbDrawRowHandlesEvent {}
impl _cbPluginEvent for cbDrawRowHandlesEvent {}
impl _wxEvent for cbDrawRowHandlesEvent {}
impl _wxObject for cbDrawRowHandlesEvent { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowHandlesEvent {
}

trait _cbDrawRowHandlesEvent : _cbPluginEvent {
}

struct cbDynToolBarDimHandler(*u8);
impl _cbDynToolBarDimHandler for cbDynToolBarDimHandler {}
impl _cbDimHandlerBase for cbDynToolBarDimHandler {}
impl _wxObject for cbDynToolBarDimHandler { fn handle(&self) -> *u8 { **self } }

impl cbDynToolBarDimHandler {
}

trait _cbDynToolBarDimHandler : _cbDimHandlerBase {
}

struct cbFinishDrawInAreaEvent(*u8);
impl _cbFinishDrawInAreaEvent for cbFinishDrawInAreaEvent {}
impl _cbPluginEvent for cbFinishDrawInAreaEvent {}
impl _wxEvent for cbFinishDrawInAreaEvent {}
impl _wxObject for cbFinishDrawInAreaEvent { fn handle(&self) -> *u8 { **self } }

impl cbFinishDrawInAreaEvent {
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
}

trait _cbFloatedBarWindow : _wxToolWindow {
}

struct cbGCUpdatesMgr(*u8);
impl _cbGCUpdatesMgr for cbGCUpdatesMgr {}
impl _cbSimpleUpdatesMgr for cbGCUpdatesMgr {}
impl _cbUpdatesManagerBase for cbGCUpdatesMgr {}
impl _wxObject for cbGCUpdatesMgr { fn handle(&self) -> *u8 { **self } }

impl cbGCUpdatesMgr {
}

trait _cbGCUpdatesMgr : _cbSimpleUpdatesMgr {
}

struct cbHintAnimationPlugin(*u8);
impl _cbHintAnimationPlugin for cbHintAnimationPlugin {}
impl _cbPluginBase for cbHintAnimationPlugin {}
impl _wxEvtHandler for cbHintAnimationPlugin {}
impl _wxObject for cbHintAnimationPlugin { fn handle(&self) -> *u8 { **self } }

impl cbHintAnimationPlugin {
}

trait _cbHintAnimationPlugin : _cbPluginBase {
}

struct cbInsertBarEvent(*u8);
impl _cbInsertBarEvent for cbInsertBarEvent {}
impl _cbPluginEvent for cbInsertBarEvent {}
impl _wxEvent for cbInsertBarEvent {}
impl _wxObject for cbInsertBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbInsertBarEvent {
}

trait _cbInsertBarEvent : _cbPluginEvent {
}

struct cbLayoutRowEvent(*u8);
impl _cbLayoutRowEvent for cbLayoutRowEvent {}
impl _cbPluginEvent for cbLayoutRowEvent {}
impl _wxEvent for cbLayoutRowEvent {}
impl _wxObject for cbLayoutRowEvent { fn handle(&self) -> *u8 { **self } }

impl cbLayoutRowEvent {
}

trait _cbLayoutRowEvent : _cbPluginEvent {
}

struct cbLeftDClickEvent(*u8);
impl _cbLeftDClickEvent for cbLeftDClickEvent {}
impl _cbPluginEvent for cbLeftDClickEvent {}
impl _wxEvent for cbLeftDClickEvent {}
impl _wxObject for cbLeftDClickEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftDClickEvent {
}

trait _cbLeftDClickEvent : _cbPluginEvent {
}

struct cbLeftDownEvent(*u8);
impl _cbLeftDownEvent for cbLeftDownEvent {}
impl _cbPluginEvent for cbLeftDownEvent {}
impl _wxEvent for cbLeftDownEvent {}
impl _wxObject for cbLeftDownEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftDownEvent {
}

trait _cbLeftDownEvent : _cbPluginEvent {
}

struct cbLeftUpEvent(*u8);
impl _cbLeftUpEvent for cbLeftUpEvent {}
impl _cbPluginEvent for cbLeftUpEvent {}
impl _wxEvent for cbLeftUpEvent {}
impl _wxObject for cbLeftUpEvent { fn handle(&self) -> *u8 { **self } }

impl cbLeftUpEvent {
}

trait _cbLeftUpEvent : _cbPluginEvent {
}

struct cbMiniButton(*u8);
impl _cbMiniButton for cbMiniButton {}
impl _wxObject for cbMiniButton { fn handle(&self) -> *u8 { **self } }

impl cbMiniButton {
}

trait _cbMiniButton : _wxObject {
}

struct cbMotionEvent(*u8);
impl _cbMotionEvent for cbMotionEvent {}
impl _cbPluginEvent for cbMotionEvent {}
impl _wxEvent for cbMotionEvent {}
impl _wxObject for cbMotionEvent { fn handle(&self) -> *u8 { **self } }

impl cbMotionEvent {
}

trait _cbMotionEvent : _cbPluginEvent {
}

struct cbPaneDrawPlugin(*u8);
impl _cbPaneDrawPlugin for cbPaneDrawPlugin {}
impl _cbPluginBase for cbPaneDrawPlugin {}
impl _wxEvtHandler for cbPaneDrawPlugin {}
impl _wxObject for cbPaneDrawPlugin { fn handle(&self) -> *u8 { **self } }

impl cbPaneDrawPlugin {
}

trait _cbPaneDrawPlugin : _cbPluginBase {
}

struct cbPluginBase(*u8);
impl _cbPluginBase for cbPluginBase {}
impl _wxEvtHandler for cbPluginBase {}
impl _wxObject for cbPluginBase { fn handle(&self) -> *u8 { **self } }

impl cbPluginBase {
}

trait _cbPluginBase : _wxEvtHandler {
}

struct cbPluginEvent(*u8);
impl _cbPluginEvent for cbPluginEvent {}
impl _wxEvent for cbPluginEvent {}
impl _wxObject for cbPluginEvent { fn handle(&self) -> *u8 { **self } }

impl cbPluginEvent {
}

trait _cbPluginEvent : _wxEvent {
}

struct cbRemoveBarEvent(*u8);
impl _cbRemoveBarEvent for cbRemoveBarEvent {}
impl _cbPluginEvent for cbRemoveBarEvent {}
impl _wxEvent for cbRemoveBarEvent {}
impl _wxObject for cbRemoveBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbRemoveBarEvent {
}

trait _cbRemoveBarEvent : _cbPluginEvent {
}

struct cbResizeBarEvent(*u8);
impl _cbResizeBarEvent for cbResizeBarEvent {}
impl _cbPluginEvent for cbResizeBarEvent {}
impl _wxEvent for cbResizeBarEvent {}
impl _wxObject for cbResizeBarEvent { fn handle(&self) -> *u8 { **self } }

impl cbResizeBarEvent {
}

trait _cbResizeBarEvent : _cbPluginEvent {
}

struct cbResizeRowEvent(*u8);
impl _cbResizeRowEvent for cbResizeRowEvent {}
impl _cbPluginEvent for cbResizeRowEvent {}
impl _wxEvent for cbResizeRowEvent {}
impl _wxObject for cbResizeRowEvent { fn handle(&self) -> *u8 { **self } }

impl cbResizeRowEvent {
}

trait _cbResizeRowEvent : _cbPluginEvent {
}

struct cbRightDownEvent(*u8);
impl _cbRightDownEvent for cbRightDownEvent {}
impl _cbPluginEvent for cbRightDownEvent {}
impl _wxEvent for cbRightDownEvent {}
impl _wxObject for cbRightDownEvent { fn handle(&self) -> *u8 { **self } }

impl cbRightDownEvent {
}

trait _cbRightDownEvent : _cbPluginEvent {
}

struct cbRightUpEvent(*u8);
impl _cbRightUpEvent for cbRightUpEvent {}
impl _cbPluginEvent for cbRightUpEvent {}
impl _wxEvent for cbRightUpEvent {}
impl _wxObject for cbRightUpEvent { fn handle(&self) -> *u8 { **self } }

impl cbRightUpEvent {
}

trait _cbRightUpEvent : _cbPluginEvent {
}

struct cbRowDragPlugin(*u8);
impl _cbRowDragPlugin for cbRowDragPlugin {}
impl _cbPluginBase for cbRowDragPlugin {}
impl _wxEvtHandler for cbRowDragPlugin {}
impl _wxObject for cbRowDragPlugin { fn handle(&self) -> *u8 { **self } }

impl cbRowDragPlugin {
}

trait _cbRowDragPlugin : _cbPluginBase {
}

struct cbRowInfo(*u8);
impl _cbRowInfo for cbRowInfo {}
impl _wxObject for cbRowInfo { fn handle(&self) -> *u8 { **self } }

impl cbRowInfo {
}

trait _cbRowInfo : _wxObject {
}

struct cbRowLayoutPlugin(*u8);
impl _cbRowLayoutPlugin for cbRowLayoutPlugin {}
impl _cbPluginBase for cbRowLayoutPlugin {}
impl _wxEvtHandler for cbRowLayoutPlugin {}
impl _wxObject for cbRowLayoutPlugin { fn handle(&self) -> *u8 { **self } }

impl cbRowLayoutPlugin {
}

trait _cbRowLayoutPlugin : _cbPluginBase {
}

struct cbSimpleCustomizationPlugin(*u8);
impl _cbSimpleCustomizationPlugin for cbSimpleCustomizationPlugin {}
impl _cbPluginBase for cbSimpleCustomizationPlugin {}
impl _wxEvtHandler for cbSimpleCustomizationPlugin {}
impl _wxObject for cbSimpleCustomizationPlugin { fn handle(&self) -> *u8 { **self } }

impl cbSimpleCustomizationPlugin {
}

trait _cbSimpleCustomizationPlugin : _cbPluginBase {
}

struct cbSimpleUpdatesMgr(*u8);
impl _cbSimpleUpdatesMgr for cbSimpleUpdatesMgr {}
impl _cbUpdatesManagerBase for cbSimpleUpdatesMgr {}
impl _wxObject for cbSimpleUpdatesMgr { fn handle(&self) -> *u8 { **self } }

impl cbSimpleUpdatesMgr {
}

trait _cbSimpleUpdatesMgr : _cbUpdatesManagerBase {
}

struct cbSizeBarWndEvent(*u8);
impl _cbSizeBarWndEvent for cbSizeBarWndEvent {}
impl _cbPluginEvent for cbSizeBarWndEvent {}
impl _wxEvent for cbSizeBarWndEvent {}
impl _wxObject for cbSizeBarWndEvent { fn handle(&self) -> *u8 { **self } }

impl cbSizeBarWndEvent {
}

trait _cbSizeBarWndEvent : _cbPluginEvent {
}

struct cbStartBarDraggingEvent(*u8);
impl _cbStartBarDraggingEvent for cbStartBarDraggingEvent {}
impl _cbPluginEvent for cbStartBarDraggingEvent {}
impl _wxEvent for cbStartBarDraggingEvent {}
impl _wxObject for cbStartBarDraggingEvent { fn handle(&self) -> *u8 { **self } }

impl cbStartBarDraggingEvent {
}

trait _cbStartBarDraggingEvent : _cbPluginEvent {
}

struct cbStartDrawInAreaEvent(*u8);
impl _cbStartDrawInAreaEvent for cbStartDrawInAreaEvent {}
impl _cbPluginEvent for cbStartDrawInAreaEvent {}
impl _wxEvent for cbStartDrawInAreaEvent {}
impl _wxObject for cbStartDrawInAreaEvent { fn handle(&self) -> *u8 { **self } }

impl cbStartDrawInAreaEvent {
}

trait _cbStartDrawInAreaEvent : _cbPluginEvent {
}

struct cbUpdatesManagerBase(*u8);
impl _cbUpdatesManagerBase for cbUpdatesManagerBase {}
impl _wxObject for cbUpdatesManagerBase { fn handle(&self) -> *u8 { **self } }

impl cbUpdatesManagerBase {
}

trait _cbUpdatesManagerBase : _wxObject {
}

struct wxAcceleratorEntry(*u8);
impl _wxAcceleratorEntry for wxAcceleratorEntry { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorEntry {
    #[fixed_stack_segment]
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> wxAcceleratorEntry {
        unsafe { wxAcceleratorEntry(wxAcceleratorEntry_Create(flags, keyCode, cmd)) }
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
    #[fixed_stack_segment]
    pub fn new(n: c_int, entries: *u8) -> wxAcceleratorTable {
        unsafe { wxAcceleratorTable(wxAcceleratorTable_Create(n, entries)) }
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
}

trait _wxApp : _wxEvtHandler {
}

struct wxArray(*u8);
impl _wxArray for wxArray { fn handle(&self) -> *u8 { **self } }

impl wxArray {
}

trait _wxArray {
    fn handle(&self) -> *u8;
    
}

struct wxArrayString(*u8);
impl _wxArrayString for wxArrayString {}
impl _wxArray for wxArrayString { fn handle(&self) -> *u8 { **self } }

impl wxArrayString {
}

trait _wxArrayString : _wxArray {
}

struct wxArtProvider(*u8);
impl _wxArtProvider for wxArtProvider {}
impl _wxObject for wxArtProvider { fn handle(&self) -> *u8 { **self } }

impl wxArtProvider {
}

trait _wxArtProvider : _wxObject {
}

struct wxAutoBufferedPaintDC(*u8);
impl _wxAutoBufferedPaintDC for wxAutoBufferedPaintDC {}
impl _wxDC for wxAutoBufferedPaintDC {}
impl _wxObject for wxAutoBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxAutoBufferedPaintDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(window: T) -> wxAutoBufferedPaintDC {
        unsafe { wxAutoBufferedPaintDC(wxAutoBufferedPaintDC_Create(window.handle())) }
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
}

trait _wxAutomationObject : _wxObject {
}

struct wxBitmap(*u8);
impl _wxBitmap for wxBitmap {}
impl _wxGDIObject for wxBitmap {}
impl _wxObject for wxBitmap { fn handle(&self) -> *u8 { **self } }

impl wxBitmap {
    #[fixed_stack_segment]
    pub fn addHandler<T: _wxEvtHandler>(handler: T) {
        unsafe { wxBitmap_AddHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    pub fn new(_data: *u8, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> wxBitmap {
        unsafe { wxBitmap(wxBitmap_Create(_data, _type, _width, _height, _depth)) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxBitmap {
        unsafe { wxBitmap(wxBitmap_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> wxBitmap {
        unsafe { wxBitmap(wxBitmap_CreateEmpty(_width, _height, _depth)) }
    }
    #[fixed_stack_segment]
    pub fn newLoad<T: _wxString>(name: T, type_: c_int) -> wxBitmap {
        unsafe { wxBitmap(wxBitmap_CreateLoad(name.handle(), type_)) }
    }
    #[fixed_stack_segment]
    pub fn findHandlerByName<T: _wxString>(name: T) -> *u8 {
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
    pub fn insertHandler<T: _wxEvtHandler>(handler: T) {
        unsafe { wxBitmap_InsertHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    pub fn removeHandler<T: _wxString>(name: T) -> bool {
        unsafe { wxBitmap_RemoveHandler(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newFromImage<T: _wxImage>(image: T, depth: c_int) -> wxBitmap {
        unsafe { wxBitmap(wxBitmap_CreateFromImage(image.handle(), depth)) }
    }
}

trait _wxBitmap : _wxGDIObject {
    #[fixed_stack_segment]
    fn newFromXPM(&self) -> wxBitmap {
        unsafe { wxBitmap(wxBitmap_CreateFromXPM(self.handle())) }
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
    fn getMask(&self) -> wxMask {
        unsafe { wxMask(wxBitmap_GetMask(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSubBitmap<T: _wxBitmap>(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: T) {
        unsafe { wxBitmap_GetSubBitmap(self.handle(), x, y, w, h, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile<T: _wxString>(&self, name: T, type_: c_int) -> c_int {
        unsafe { wxBitmap_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBitmap_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile<T: _wxString, U: _wxPalette>(&self, name: T, type_: c_int, cmap: U) -> c_int {
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
    fn setMask<T: _wxMask>(&self, mask: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: T, _id: c_int, _bmp: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxBitmapButton {
        unsafe { wxBitmapButton(wxBitmapButton_Create(_prt.handle(), _id, _bmp.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxBitmapButton : _wxButton {
    #[fixed_stack_segment]
    fn getBitmapDisabled<T: _wxBitmap>(&self, _ref: T) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapFocus<T: _wxBitmap>(&self, _ref: T) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapLabel<T: _wxBitmap>(&self, _ref: T) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapSelected<T: _wxBitmap>(&self, _ref: T) {
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
    fn setBitmapDisabled<T: _wxBitmap>(&self, disabled: T) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.handle(), disabled.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapFocus<T: _wxBitmap>(&self, focus: T) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.handle(), focus.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel<T: _wxBitmap>(&self, bitmap: T) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapSelected<T: _wxBitmap>(&self, sel: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxBitmap>(parent: T, id: c_int, _bmp: U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> wxBitmapToggleButton {
        unsafe { wxBitmapToggleButton(wxBitmapToggleButton_Create(parent.handle(), id, _bmp.handle(), x, y, w, h, style)) }
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
    fn setBitmapLabel<T: _wxBitmap>(&self, _bmp: T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.handle(), _bmp.handle()) }
    }
}

struct wxBitmapDataObject(*u8);
impl _wxBitmapDataObject for wxBitmapDataObject {}
impl _wxDataObjectSimple for wxBitmapDataObject {}
impl _wxDataObject for wxBitmapDataObject { fn handle(&self) -> *u8 { **self } }

impl wxBitmapDataObject {
}

trait _wxBitmapDataObject : _wxDataObjectSimple {
}

struct wxBitmapHandler(*u8);
impl _wxBitmapHandler for wxBitmapHandler {}
impl _wxObject for wxBitmapHandler { fn handle(&self) -> *u8 { **self } }

impl wxBitmapHandler {
}

trait _wxBitmapHandler : _wxObject {
}

struct wxBoxSizer(*u8);
impl _wxBoxSizer for wxBoxSizer {}
impl _wxSizer for wxBoxSizer {}
impl _wxObject for wxBoxSizer { fn handle(&self) -> *u8 { **self } }

impl wxBoxSizer {
    #[fixed_stack_segment]
    pub fn new(orient: c_int) -> wxBoxSizer {
        unsafe { wxBoxSizer(wxBoxSizer_Create(orient)) }
    }
}

trait _wxBoxSizer : _wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize(wxBoxSizer_CalcMin(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn newDefault() -> wxBrush {
        unsafe { wxBrush(wxBrush_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap<T: _wxBitmap>(bitmap: T) -> wxBrush {
        unsafe { wxBrush(wxBrush_CreateFromBitmap(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromColour<T: _wxColour>(col: T, style: c_int) -> wxBrush {
        unsafe { wxBrush(wxBrush_CreateFromColour(col.handle(), style)) }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> wxBrush {
        unsafe { wxBrush(wxBrush_CreateFromStock(id)) }
    }
}

trait _wxBrush : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign<T: _wxBrush>(&self, brush: T) {
        unsafe { wxBrush_Assign(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBrush_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxBrush_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStipple<T: _wxBitmap>(&self, _ref: T) {
        unsafe { wxBrush_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual<T: _wxBrush>(&self, brush: T) -> bool {
        unsafe { wxBrush_IsEqual(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBrush_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour<T: _wxColour>(&self, col: T) {
        unsafe { wxBrush_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxBrush_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    fn setStipple<T: _wxBitmap>(&self, stipple: T) {
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
}

trait _wxBrushList : _wxList {
}

struct wxBufferedDC(*u8);
impl _wxBufferedDC for wxBufferedDC {}
impl _wxDC for wxBufferedDC {}
impl _wxObject for wxBufferedDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedDC {
    #[fixed_stack_segment]
    pub fn newByDCAndSize<T: _wxDC>(dc: T, width: c_int, hight: c_int, style: c_int) -> wxBufferedDC {
        unsafe { wxBufferedDC(wxBufferedDC_CreateByDCAndSize(dc.handle(), width, hight, style)) }
    }
    #[fixed_stack_segment]
    pub fn newByDCAndBitmap<T: _wxDC, U: _wxBitmap>(dc: T, bitmap: U, style: c_int) -> wxBufferedDC {
        unsafe { wxBufferedDC(wxBufferedDC_CreateByDCAndBitmap(dc.handle(), bitmap.handle(), style)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(window: T, style: c_int) -> wxBufferedPaintDC {
        unsafe { wxBufferedPaintDC(wxBufferedPaintDC_Create(window.handle(), style)) }
    }
    #[fixed_stack_segment]
    pub fn newWithBitmap<T: _wxWindow, U: _wxBitmap>(window: T, bitmap: U, style: c_int) -> wxBufferedPaintDC {
        unsafe { wxBufferedPaintDC(wxBufferedPaintDC_CreateWithBitmap(window.handle(), bitmap.handle(), style)) }
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
}

trait _wxBufferedInputStream : _wxFilterInputStream {
}

struct wxBufferedOutputStream(*u8);
impl _wxBufferedOutputStream for wxBufferedOutputStream {}
impl _wxFilterOutputStream for wxBufferedOutputStream {}
impl _wxOutputStream for wxBufferedOutputStream {}
impl _wxStreamBase for wxBufferedOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedOutputStream {
}

trait _wxBufferedOutputStream : _wxFilterOutputStream {
}

struct wxBusyCursor(*u8);
impl _wxBusyCursor for wxBusyCursor { fn handle(&self) -> *u8 { **self } }

impl wxBusyCursor {
    #[fixed_stack_segment]
    pub fn new() -> wxBusyCursor {
        unsafe { wxBusyCursor(wxBusyCursor_Create()) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(_txt: T) -> wxBusyInfo {
        unsafe { wxBusyInfo(wxBusyInfo_Create(_txt.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxButton {
        unsafe { wxButton(wxButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxButton : _wxControl {
    #[fixed_stack_segment]
    fn setBackgroundColour<T: _wxColour>(&self, colour: T) -> c_int {
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
}

trait _wxCSConv : _wxMBConv {
}

struct wxCalculateLayoutEvent(*u8);
impl _wxCalculateLayoutEvent for wxCalculateLayoutEvent {}
impl _wxEvent for wxCalculateLayoutEvent {}
impl _wxObject for wxCalculateLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalculateLayoutEvent {
    #[fixed_stack_segment]
    pub fn new(id: c_int) -> wxCalculateLayoutEvent {
        unsafe { wxCalculateLayoutEvent(wxCalculateLayoutEvent_Create(id)) }
    }
}

trait _wxCalculateLayoutEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> wxRect {
        unsafe { wxRect(wxCalculateLayoutEvent_GetRect(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxDateTime>(_prt: T, _id: c_int, _dat: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxCalendarCtrl {
        unsafe { wxCalendarCtrl(wxCalendarCtrl_Create(_prt.handle(), _id, _dat.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    fn getHeaderColourBg<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourFg<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourBg<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourFg<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourBg<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourFg<T: _wxColour>(&self, _ref: T) {
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
    #[fixed_stack_segment]
    pub fn new(_ctxt: *u8, _cbck: *u8, _cbrd: *u8, _fnt: *u8, _brd: c_int) -> wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr(wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr(wxCalendarDateAttr_CreateDefault()) }
    }
}

trait _wxCalendarDateAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorderColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont<T: _wxFont>(&self, _ref: T) {
        unsafe { wxCalendarDateAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour<T: _wxColour>(&self, _ref: T) {
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
    fn setBackgroundColour<T: _wxColour>(&self, col: T) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    fn setBorderColour<T: _wxColour>(&self, col: T) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont<T: _wxFont>(&self, font: T) {
        unsafe { wxCalendarDateAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.handle(), holiday) }
    }
    #[fixed_stack_segment]
    fn setTextColour<T: _wxColour>(&self, col: T) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.handle(), col.handle()) }
    }
}

struct wxCalendarEvent(*u8);
impl _wxCalendarEvent for wxCalendarEvent {}
impl _wxCommandEvent for wxCalendarEvent {}
impl _wxEvent for wxCalendarEvent {}
impl _wxObject for wxCalendarEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalendarEvent {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_wnd: T, _wth: c_int, _hgt: c_int) -> wxCaret {
        unsafe { wxCaret(wxCaret_Create(_wnd.handle(), _wth, _hgt)) }
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
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxCaret_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxCaret_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxCaret_GetWindow(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxCheckBox {
        unsafe { wxCheckBox(wxCheckBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> wxCheckListBox {
        unsafe { wxCheckListBox(wxCheckListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> wxChoice {
        unsafe { wxChoice(wxChoice_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxChoice : _wxControl {
    #[fixed_stack_segment]
    fn append<T: _wxString>(&self, item: T) {
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
    fn findString<T: _wxString>(&self, s: T) -> c_int {
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
    fn getString(&self, n: c_int) -> wxString {
        unsafe { wxString(wxChoice_GetString(self.handle(), n)) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setString<T: _wxString>(&self, n: c_int, s: T) {
        unsafe { wxChoice_SetString(self.handle(), n, s.handle()) }
    }
}

struct wxClassInfo(*u8);
impl _wxClassInfo for wxClassInfo { fn handle(&self) -> *u8 { **self } }

impl wxClassInfo {
    #[fixed_stack_segment]
    pub fn findClass<T: _wxString>(_txt: T) -> wxClassInfo {
        unsafe { wxClassInfo(wxClassInfo_FindClass(_txt.handle())) }
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
    fn isKindOf<T: _wxString>(&self, _name: T) -> bool {
        unsafe { wxClassInfo_IsKindOf(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn getBaseClassName1(&self) -> wxString {
        unsafe { wxString(wxClassInfo_GetBaseClassName1(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getBaseClassName2(&self) -> wxString {
        unsafe { wxString(wxClassInfo_GetBaseClassName2(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getClassNameEx(&self) -> wxString {
        unsafe { wxString(wxClassInfo_GetClassNameEx(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isKindOfEx<T: _wxClassInfo>(&self, classInfo: T) -> bool {
        unsafe { wxClassInfo_IsKindOfEx(self.handle(), classInfo.handle()) }
    }
}

struct wxClient(*u8);
impl _wxClient for wxClient {}
impl _wxClientBase for wxClient {}
impl _wxObject for wxClient { fn handle(&self) -> *u8 { **self } }

impl wxClient {
}

trait _wxClient : _wxClientBase {
}

struct wxClientBase(*u8);
impl _wxClientBase for wxClientBase {}
impl _wxObject for wxClientBase { fn handle(&self) -> *u8 { **self } }

impl wxClientBase {
}

trait _wxClientBase : _wxObject {
}

struct wxClientDC(*u8);
impl _wxClientDC for wxClientDC {}
impl _wxWindowDC for wxClientDC {}
impl _wxDC for wxClientDC {}
impl _wxObject for wxClientDC { fn handle(&self) -> *u8 { **self } }

impl wxClientDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(win: T) -> wxClientDC {
        unsafe { wxClientDC(wxClientDC_Create(win.handle())) }
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
}

trait _wxClientData {
    fn handle(&self) -> *u8;
    
}

struct wxClientDataContainer(*u8);
impl _wxClientDataContainer for wxClientDataContainer { fn handle(&self) -> *u8 { **self } }

impl wxClientDataContainer {
}

trait _wxClientDataContainer {
    fn handle(&self) -> *u8;
    
}

struct wxClipboard(*u8);
impl _wxClipboard for wxClipboard {}
impl _wxObject for wxClipboard { fn handle(&self) -> *u8 { **self } }

impl wxClipboard {
    #[fixed_stack_segment]
    pub fn new() -> wxClipboard {
        unsafe { wxClipboard(wxClipboard_Create()) }
    }
}

trait _wxClipboard : _wxObject {
    #[fixed_stack_segment]
    fn addData<T: _wxDataObject>(&self, data: T) -> bool {
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
    fn getData<T: _wxDataObject>(&self, data: T) -> bool {
        unsafe { wxClipboard_GetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn isOpened(&self) -> bool {
        unsafe { wxClipboard_IsOpened(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSupported<T: _wxDataFormat>(&self, format: T) -> bool {
        unsafe { wxClipboard_IsSupported(self.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn open(&self) -> bool {
        unsafe { wxClipboard_Open(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setData<T: _wxDataObject>(&self, data: T) -> bool {
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
}

trait _wxCloseEvent : _wxEvent {
    #[fixed_stack_segment]
    fn canVeto(&self) -> bool {
        unsafe { wxCloseEvent_CanVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject<T: _wxObject>(&self, obj: T) {
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
    #[fixed_stack_segment]
    pub fn new(_fun_CEvent: *u8, _data: *u8) -> wxClosure {
        unsafe { wxClosure(wxClosure_Create(_fun_CEvent, _data)) }
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
    #[fixed_stack_segment]
    pub fn newByName<T: _wxString>(_name: T) -> wxColour {
        unsafe { wxColour(wxColour_CreateByName(_name.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newEmpty() -> wxColour {
        unsafe { wxColour(wxColour_CreateEmpty()) }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> wxColour {
        unsafe { wxColour(wxColour_CreateFromStock(id)) }
    }
    #[fixed_stack_segment]
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> wxColour {
        unsafe { wxColour(wxColour_CreateRGB(_red, _green, _blue, _alpha)) }
    }
    #[fixed_stack_segment]
    pub fn validName(_name: *wchar_t) -> bool {
        unsafe { wxColour_ValidName(_name) }
    }
    #[fixed_stack_segment]
    pub fn newFromInt(rgb: c_int) -> wxColour {
        unsafe { wxColour(wxColour_CreateFromInt(rgb)) }
    }
    #[fixed_stack_segment]
    pub fn newFromUnsignedInt(rgba: uint32_t) -> wxColour {
        unsafe { wxColour(wxColour_CreateFromUnsignedInt(rgba)) }
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
    fn setByName<T: _wxString>(&self, _name: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxColourData {
        unsafe { wxColourData(wxColourData_Create()) }
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
    fn getColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxColourData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCustomColour<T: _wxColour>(&self, i: c_int, _ref: T) {
        unsafe { wxColourData_GetCustomColour(self.handle(), i, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setChooseFull(&self, flag: bool) {
        unsafe { wxColourData_SetChooseFull(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxColourData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCustomColour<T: _wxColour>(&self, i: c_int, colour: T) {
        unsafe { wxColourData_SetCustomColour(self.handle(), i, colour.handle()) }
    }
}

struct wxColourDatabase(*u8);
impl _wxColourDatabase for wxColourDatabase {}
impl _wxList for wxColourDatabase {}
impl _wxObject for wxColourDatabase { fn handle(&self) -> *u8 { **self } }

impl wxColourDatabase {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxColourData>(_prt: T, col: U) -> wxColourDialog {
        unsafe { wxColourDialog(wxColourDialog_Create(_prt.handle(), col.handle())) }
    }
}

trait _wxColourDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getColourData<T: _wxColourData>(&self, _ref: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> wxComboBox {
        unsafe { wxComboBox(wxComboBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxComboBox : _wxChoice {
    #[fixed_stack_segment]
    fn append<T: _wxString>(&self, item: T) {
        unsafe { wxComboBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData<T: _wxString>(&self, item: T, d: *u8) {
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
    fn findString<T: _wxString>(&self, s: T) -> c_int {
        unsafe { wxComboBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> wxClientData {
        unsafe { wxClientData(wxComboBox_GetClientData(self.handle(), n)) }
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
    fn getString(&self, n: c_int) -> wxString {
        unsafe { wxString(wxComboBox_GetString(self.handle(), n)) }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> wxString {
        unsafe { wxString(wxComboBox_GetStringSelection(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> wxString {
        unsafe { wxString(wxComboBox_GetValue(self.handle())) }
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
    fn replace<T: _wxString>(&self, from: c_int, to: c_int, value: T) {
        unsafe { wxComboBox_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData<T: _wxClientData>(&self, n: c_int, clientData: T) {
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
}

trait _wxCommand : _wxObject {
}

struct wxCommandEvent(*u8);
impl _wxCommandEvent for wxCommandEvent {}
impl _wxEvent for wxCommandEvent {}
impl _wxObject for wxCommandEvent { fn handle(&self) -> *u8 { **self } }

impl wxCommandEvent {
    #[fixed_stack_segment]
    pub fn new(_typ: c_int, _id: c_int) -> wxCommandEvent {
        unsafe { wxCommandEvent(wxCommandEvent_Create(_typ, _id)) }
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
    fn getClientData(&self) -> wxClientData {
        unsafe { wxClientData(wxCommandEvent_GetClientData(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getClientObject(&self) -> wxClientData {
        unsafe { wxClientData(wxCommandEvent_GetClientObject(self.handle())) }
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
    fn getString(&self) -> wxString {
        unsafe { wxString(wxCommandEvent_GetString(self.handle())) }
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
    fn setClientData<T: _wxClientData>(&self, clientData: T) {
        unsafe { wxCommandEvent_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject<T: _wxClientData>(&self, clientObject: T) {
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
    fn setString<T: _wxString>(&self, s: T) {
        unsafe { wxCommandEvent_SetString(self.handle(), s.handle()) }
    }
}

struct wxCommandLineParser(*u8);
impl _wxCommandLineParser for wxCommandLineParser { fn handle(&self) -> *u8 { **self } }

impl wxCommandLineParser {
}

trait _wxCommandLineParser {
    fn handle(&self) -> *u8;
    
}

struct wxCommandProcessor(*u8);
impl _wxCommandProcessor for wxCommandProcessor {}
impl _wxObject for wxCommandProcessor { fn handle(&self) -> *u8 { **self } }

impl wxCommandProcessor {
}

trait _wxCommandProcessor : _wxObject {
}

struct wxCondition(*u8);
impl _wxCondition for wxCondition { fn handle(&self) -> *u8 { **self } }

impl wxCondition {
}

trait _wxCondition {
    fn handle(&self) -> *u8;
    
}

struct wxConfigBase(*u8);
impl _wxConfigBase for wxConfigBase { fn handle(&self) -> *u8 { **self } }

impl wxConfigBase {
    #[fixed_stack_segment]
    pub fn new() -> wxConfigBase {
        unsafe { wxConfigBase(wxConfigBase_Create()) }
    }
    #[fixed_stack_segment]
    pub fn get() -> wxConfigBase {
        unsafe { wxConfigBase(wxConfigBase_Get()) }
    }
    #[fixed_stack_segment]
    pub fn set<T: _wxConfigBase>(self_: T) {
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
    fn deleteEntry<T: _wxString>(&self, key: T, bDeleteGroupIfEmpty: bool) -> bool {
        unsafe { wxConfigBase_DeleteEntry(self.handle(), key.handle(), bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    fn deleteGroup<T: _wxString>(&self, key: T) -> bool {
        unsafe { wxConfigBase_DeleteGroup(self.handle(), key.handle()) }
    }
    #[fixed_stack_segment]
    fn exists<T: _wxString>(&self, strName: T) -> bool {
        unsafe { wxConfigBase_Exists(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn expandEnvVars<T: _wxString>(&self, str: T) -> wxString {
        unsafe { wxString(wxConfigBase_ExpandEnvVars(self.handle(), str.handle())) }
    }
    #[fixed_stack_segment]
    fn flush(&self, bCurrentOnly: bool) -> bool {
        unsafe { wxConfigBase_Flush(self.handle(), bCurrentOnly) }
    }
    #[fixed_stack_segment]
    fn getAppName(&self) -> wxString {
        unsafe { wxString(wxConfigBase_GetAppName(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getEntryType<T: _wxString>(&self, name: T) -> c_int {
        unsafe { wxConfigBase_GetEntryType(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstEntry(&self, lIndex: *u8) -> wxString {
        unsafe { wxString(wxConfigBase_GetFirstEntry(self.handle(), lIndex)) }
    }
    #[fixed_stack_segment]
    fn getFirstGroup(&self, lIndex: *u8) -> wxString {
        unsafe { wxString(wxConfigBase_GetFirstGroup(self.handle(), lIndex)) }
    }
    #[fixed_stack_segment]
    fn getNextEntry(&self, lIndex: *u8) -> wxString {
        unsafe { wxString(wxConfigBase_GetNextEntry(self.handle(), lIndex)) }
    }
    #[fixed_stack_segment]
    fn getNextGroup(&self, lIndex: *u8) -> wxString {
        unsafe { wxString(wxConfigBase_GetNextGroup(self.handle(), lIndex)) }
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
    fn getPath(&self) -> wxString {
        unsafe { wxString(wxConfigBase_GetPath(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVendorName(&self) -> wxString {
        unsafe { wxString(wxConfigBase_GetVendorName(self.handle())) }
    }
    #[fixed_stack_segment]
    fn hasEntry<T: _wxString>(&self, strName: T) -> bool {
        unsafe { wxConfigBase_HasEntry(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn hasGroup<T: _wxString>(&self, strName: T) -> bool {
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
    fn readBool<T: _wxString>(&self, key: T, defVal: bool) -> bool {
        unsafe { wxConfigBase_ReadBool(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readDouble<T: _wxString>(&self, key: T, defVal: c_double) -> c_double {
        unsafe { wxConfigBase_ReadDouble(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readInteger<T: _wxString>(&self, key: T, defVal: c_int) -> c_int {
        unsafe { wxConfigBase_ReadInteger(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readString<T: _wxString, U: _wxString>(&self, key: T, defVal: U) -> wxString {
        unsafe { wxString(wxConfigBase_ReadString(self.handle(), key.handle(), defVal.handle())) }
    }
    #[fixed_stack_segment]
    fn renameEntry<T: _wxString, U: _wxString>(&self, oldName: T, newName: U) -> bool {
        unsafe { wxConfigBase_RenameEntry(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn renameGroup<T: _wxString, U: _wxString>(&self, oldName: T, newName: U) -> bool {
        unsafe { wxConfigBase_RenameGroup(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn setAppName<T: _wxString>(&self, appName: T) {
        unsafe { wxConfigBase_SetAppName(self.handle(), appName.handle()) }
    }
    #[fixed_stack_segment]
    fn setExpandEnvVars(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    fn setPath<T: _wxString>(&self, strPath: T) {
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
    fn setVendorName<T: _wxString>(&self, vendorName: T) {
        unsafe { wxConfigBase_SetVendorName(self.handle(), vendorName.handle()) }
    }
    #[fixed_stack_segment]
    fn writeBool<T: _wxString>(&self, key: T, value: bool) -> bool {
        unsafe { wxConfigBase_WriteBool(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeDouble<T: _wxString>(&self, key: T, value: c_double) -> bool {
        unsafe { wxConfigBase_WriteDouble(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeInteger<T: _wxString>(&self, key: T, value: c_int) -> bool {
        unsafe { wxConfigBase_WriteInteger(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeLong<T: _wxString>(&self, key: T, value: c_long) -> bool {
        unsafe { wxConfigBase_WriteLong(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeString<T: _wxString, U: _wxString>(&self, key: T, value: U) -> bool {
        unsafe { wxConfigBase_WriteString(self.handle(), key.handle(), value.handle()) }
    }
}

struct wxConnection(*u8);
impl _wxConnection for wxConnection {}
impl _wxConnectionBase for wxConnection {}
impl _wxObject for wxConnection { fn handle(&self) -> *u8 { **self } }

impl wxConnection {
}

trait _wxConnection : _wxConnectionBase {
}

struct wxConnectionBase(*u8);
impl _wxConnectionBase for wxConnectionBase {}
impl _wxObject for wxConnectionBase { fn handle(&self) -> *u8 { **self } }

impl wxConnectionBase {
}

trait _wxConnectionBase : _wxObject {
}

struct wxContextHelp(*u8);
impl _wxContextHelp for wxContextHelp {}
impl _wxObject for wxContextHelp { fn handle(&self) -> *u8 { **self } }

impl wxContextHelp {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(win: T, beginHelp: bool) -> wxContextHelp {
        unsafe { wxContextHelp(wxContextHelp_Create(win.handle(), beginHelp)) }
    }
}

trait _wxContextHelp : _wxObject {
    #[fixed_stack_segment]
    fn beginContextHelp<T: _wxWindow>(&self, win: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(parent: T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> wxContextHelpButton {
        unsafe { wxContextHelpButton(wxContextHelpButton_Create(parent.handle(), id, x, y, w, h, style)) }
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
}

trait _wxControl : _wxWindow {
    #[fixed_stack_segment]
    fn command<T: _wxEvent>(&self, event: T) {
        unsafe { wxControl_Command(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> wxString {
        unsafe { wxString(wxControl_GetLabel(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setLabel<T: _wxString>(&self, text: T) {
        unsafe { wxControl_SetLabel(self.handle(), text.handle()) }
    }
}

struct wxCountingOutputStream(*u8);
impl _wxCountingOutputStream for wxCountingOutputStream {}
impl _wxOutputStream for wxCountingOutputStream {}
impl _wxStreamBase for wxCountingOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxCountingOutputStream {
}

trait _wxCountingOutputStream : _wxOutputStream {
}

struct wxCriticalSection(*u8);
impl _wxCriticalSection for wxCriticalSection { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSection {
}

trait _wxCriticalSection {
    fn handle(&self) -> *u8;
    
}

struct wxCriticalSectionLocker(*u8);
impl _wxCriticalSectionLocker for wxCriticalSectionLocker { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSectionLocker {
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
}

trait _wxCustomDataObject : _wxDataObjectSimple {
}

struct wxDC(*u8);
impl _wxDC for wxDC {}
impl _wxObject for wxDC { fn handle(&self) -> *u8 { **self } }

impl wxDC {
}

trait _wxDC : _wxObject {
    #[fixed_stack_segment]
    fn blit<T: _wxDC>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: bool) -> bool {
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
    fn drawBitmap<T: _wxBitmap>(&self, bmp: T, x: c_int, y: c_int, useMask: bool) {
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
    fn drawIcon<T: _wxIcon>(&self, icon: T, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.handle(), icon.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawLabel<T: _wxString>(&self, str: T, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        unsafe { wxDC_DrawLabel(self.handle(), str.handle(), x, y, w, h, align, indexAccel) }
    }
    #[fixed_stack_segment]
    fn drawLabelBitmap<T: _wxString, U: _wxBitmap>(&self, str: T, bmp: U, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> wxRect {
        unsafe { wxRect(wxDC_DrawLabelBitmap(self.handle(), str.handle(), bmp.handle(), x, y, w, h, align, indexAccel)) }
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
    fn drawRotatedText<T: _wxString>(&self, text: T, x: c_int, y: c_int, angle: c_double) {
        unsafe { wxDC_DrawRotatedText(self.handle(), text.handle(), x, y, angle) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.handle(), x, y, width, height, radius) }
    }
    #[fixed_stack_segment]
    fn drawText<T: _wxString>(&self, text: T, x: c_int, y: c_int) {
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
    fn floodFill<T: _wxColour>(&self, x: c_int, y: c_int, col: T, style: c_int) {
        unsafe { wxDC_FloodFill(self.handle(), x, y, col.handle(), style) }
    }
    #[fixed_stack_segment]
    fn getBackground<T: _wxBrush>(&self, _ref: T) {
        unsafe { wxDC_GetBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBrush<T: _wxBrush>(&self, _ref: T) {
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
    fn getFont<T: _wxFont>(&self, _ref: T) {
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
    fn getPPI(&self) -> wxSize {
        unsafe { wxSize(wxDC_GetPPI(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPen<T: _wxPen>(&self, _ref: T) {
        unsafe { wxDC_GetPen(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPixel<T: _wxColour>(&self, x: c_int, y: c_int, col: T) -> bool {
        unsafe { wxDC_GetPixel(self.handle(), x, y, col.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxDC_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSizeMM(&self) -> wxSize {
        unsafe { wxSize(wxDC_GetSizeMM(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTextBackground<T: _wxColour>(&self, _ref: T) {
        unsafe { wxDC_GetTextBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent<T: _wxString, U: _wxFont>(&self, string: T, w: *u8, h: *u8, descent: *u8, externalLeading: *u8, theFont: U) {
        unsafe { wxDC_GetTextExtent(self.handle(), string.handle(), w, h, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getMultiLineTextExtent<T: _wxString, U: _wxFont>(&self, string: T, w: *u8, h: *u8, heightLine: *u8, theFont: U) {
        unsafe { wxDC_GetMultiLineTextExtent(self.handle(), string.handle(), w, h, heightLine, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextForeground<T: _wxColour>(&self, _ref: T) {
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
    fn setBackground<T: _wxBrush>(&self, brush: T) {
        unsafe { wxDC_SetBackground(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn setBrush<T: _wxBrush>(&self, brush: T) {
        unsafe { wxDC_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn setClippingRegionFromRegion<T: _wxRegion>(&self, region: T) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setFont<T: _wxFont>(&self, font: T) {
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
    fn setPalette<T: _wxPalette>(&self, palette: T) {
        unsafe { wxDC_SetPalette(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen<T: _wxPen>(&self, pen: T) {
        unsafe { wxDC_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextBackground<T: _wxColour>(&self, colour: T) {
        unsafe { wxDC_SetTextBackground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextForeground<T: _wxColour>(&self, colour: T) {
        unsafe { wxDC_SetTextForeground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn startDoc<T: _wxString>(&self, msg: T) -> bool {
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
    fn getPixel2<T: _wxColour>(&self, x: c_int, y: c_int, col: T) {
        unsafe { wxDC_GetPixel2(self.handle(), x, y, col.handle()) }
    }
}

struct wxDCClipper(*u8);
impl _wxDCClipper for wxDCClipper { fn handle(&self) -> *u8 { **self } }

impl wxDCClipper {
}

trait _wxDCClipper {
    fn handle(&self) -> *u8;
    
}

struct wxDDEClient(*u8);
impl _wxDDEClient for wxDDEClient {}
impl _wxClientBase for wxDDEClient {}
impl _wxObject for wxDDEClient { fn handle(&self) -> *u8 { **self } }

impl wxDDEClient {
}

trait _wxDDEClient : _wxClientBase {
}

struct wxDDEConnection(*u8);
impl _wxDDEConnection for wxDDEConnection {}
impl _wxConnectionBase for wxDDEConnection {}
impl _wxObject for wxDDEConnection { fn handle(&self) -> *u8 { **self } }

impl wxDDEConnection {
}

trait _wxDDEConnection : _wxConnectionBase {
}

struct wxDDEServer(*u8);
impl _wxDDEServer for wxDDEServer {}
impl _wxServerBase for wxDDEServer {}
impl _wxObject for wxDDEServer { fn handle(&self) -> *u8 { **self } }

impl wxDDEServer {
}

trait _wxDDEServer : _wxServerBase {
}

struct wxDataFormat(*u8);
impl _wxDataFormat for wxDataFormat { fn handle(&self) -> *u8 { **self } }

impl wxDataFormat {
    #[fixed_stack_segment]
    pub fn newFromId<T: _wxString>(name: T) -> wxDataFormat {
        unsafe { wxDataFormat(wxDataFormat_CreateFromId(name.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromType(typ: c_int) -> wxDataFormat {
        unsafe { wxDataFormat(wxDataFormat_CreateFromType(typ)) }
    }
}

trait _wxDataFormat {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> wxString {
        unsafe { wxString(wxDataFormat_GetId(self.handle())) }
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
}

trait _wxDataInputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDataObject(*u8);
impl _wxDataObject for wxDataObject { fn handle(&self) -> *u8 { **self } }

impl wxDataObject {
}

trait _wxDataObject {
    fn handle(&self) -> *u8;
    
}

struct wxDataObjectComposite(*u8);
impl _wxDataObjectComposite for wxDataObjectComposite {}
impl _wxDataObject for wxDataObjectComposite { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectComposite {
    #[fixed_stack_segment]
    pub fn new() -> wxDataObjectComposite {
        unsafe { wxDataObjectComposite(wxDataObjectComposite_Create()) }
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
}

trait _wxDataObjectSimple : _wxDataObject {
}

struct wxDataOutputStream(*u8);
impl _wxDataOutputStream for wxDataOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataOutputStream {
}

trait _wxDataOutputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDatabase(*u8);
impl _wxDatabase for wxDatabase {}
impl _wxObject for wxDatabase { fn handle(&self) -> *u8 { **self } }

impl wxDatabase {
}

trait _wxDatabase : _wxObject {
}

struct wxDateTime(*u8);
impl _wxDateTime for wxDateTime { fn handle(&self) -> *u8 { **self } }

impl wxDateTime {
    #[fixed_stack_segment]
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    pub fn new() -> wxDateTime {
        unsafe { wxDateTime(wxDateTime_Create()) }
    }
    #[fixed_stack_segment]
    pub fn getAmString() -> wxString {
        unsafe { wxString(wxDateTime_GetAmString()) }
    }
    #[fixed_stack_segment]
    pub fn getBeginDST<T: _wxDateTime>(year: c_int, country: c_int, dt: T) {
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
    pub fn getEndDST<T: _wxDateTime>(year: c_int, country: c_int, dt: T) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getMonthName(month: c_int, flags: c_int) -> wxString {
        unsafe { wxString(wxDateTime_GetMonthName(month, flags)) }
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
    pub fn getPmString() -> wxString {
        unsafe { wxString(wxDateTime_GetPmString()) }
    }
    #[fixed_stack_segment]
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> wxString {
        unsafe { wxString(wxDateTime_GetWeekDayName(weekday, flags)) }
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
    fn addDate<T: _wxDateTime>(&self, diff: *u8, _ref: T) {
        unsafe { wxDateTime_AddDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.handle(), _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    fn addTime<T: _wxDateTime>(&self, diff: *u8, _ref: T) {
        unsafe { wxDateTime_AddTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.handle(), _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    fn format(&self, format: *u8, tz: c_int) -> wxString {
        unsafe { wxString(wxDateTime_Format(self.handle(), format, tz)) }
    }
    #[fixed_stack_segment]
    fn formatDate(&self) -> wxString {
        unsafe { wxString(wxDateTime_FormatDate(self.handle())) }
    }
    #[fixed_stack_segment]
    fn formatISODate(&self) -> wxString {
        unsafe { wxString(wxDateTime_FormatISODate(self.handle())) }
    }
    #[fixed_stack_segment]
    fn formatISOTime(&self) -> wxString {
        unsafe { wxString(wxDateTime_FormatISOTime(self.handle())) }
    }
    #[fixed_stack_segment]
    fn formatTime(&self) -> wxString {
        unsafe { wxString(wxDateTime_FormatTime(self.handle())) }
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
    fn getLastMonthDay<T: _wxDateTime>(&self, month: c_int, year: c_int, _ref: T) {
        unsafe { wxDateTime_GetLastMonthDay(self.handle(), month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastWeekDay<T: _wxDateTime>(&self, weekday: c_int, month: c_int, year: c_int, _ref: T) {
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
    fn getNextWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: T) {
        unsafe { wxDateTime_GetNextWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevWeekDay<T: _wxDateTime>(&self, weekday: c_int, _ref: T) {
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
    fn getWeekDay<T: _wxDateTime>(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: T) {
        unsafe { wxDateTime_GetWeekDay(self.handle(), weekday, n, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeekDayInSameWeek<T: _wxDateTime>(&self, weekday: c_int, _ref: T) {
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
    fn isBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: T, t2: U) -> bool {
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
    fn isEqualUpTo<T: _wxDateTime>(&self, dt: T, ts: *u8) -> bool {
        unsafe { wxDateTime_IsEqualUpTo(self.handle(), dt.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn isLaterThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsLaterThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn isSameDate<T: _wxDateTime>(&self, dt: T) -> bool {
        unsafe { wxDateTime_IsSameDate(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isSameTime<T: _wxDateTime>(&self, dt: T) -> bool {
        unsafe { wxDateTime_IsSameTime(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isStrictlyBetween<T: _wxDateTime, U: _wxDateTime>(&self, t1: T, t2: U) -> bool {
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
    fn parseTime<T: _wxTime>(&self, time: T) -> *u8 {
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
    fn subtractDate<T: _wxDateTime>(&self, diff: *u8, _ref: T) {
        unsafe { wxDateTime_SubtractDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractTime<T: _wxDateTime>(&self, diff: *u8, _ref: T) {
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
}

trait _wxDb {
    fn handle(&self) -> *u8;
    
}

struct wxDbColDef(*u8);
impl _wxDbColDef for wxDbColDef { fn handle(&self) -> *u8 { **self } }

impl wxDbColDef {
}

trait _wxDbColDef {
    fn handle(&self) -> *u8;
    
}

struct wxDbColFor(*u8);
impl _wxDbColFor for wxDbColFor { fn handle(&self) -> *u8 { **self } }

impl wxDbColFor {
}

trait _wxDbColFor {
    fn handle(&self) -> *u8;
    
}

struct wxDbColInf(*u8);
impl _wxDbColInf for wxDbColInf { fn handle(&self) -> *u8 { **self } }

impl wxDbColInf {
}

trait _wxDbColInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbConnectInf(*u8);
impl _wxDbConnectInf for wxDbConnectInf { fn handle(&self) -> *u8 { **self } }

impl wxDbConnectInf {
}

trait _wxDbConnectInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbInf(*u8);
impl _wxDbInf for wxDbInf { fn handle(&self) -> *u8 { **self } }

impl wxDbInf {
}

trait _wxDbInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbSqlTypeInfo(*u8);
impl _wxDbSqlTypeInfo for wxDbSqlTypeInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbSqlTypeInfo {
}

trait _wxDbSqlTypeInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDbTable(*u8);
impl _wxDbTable for wxDbTable { fn handle(&self) -> *u8 { **self } }

impl wxDbTable {
}

trait _wxDbTable {
    fn handle(&self) -> *u8;
    
}

struct wxDbTableInfo(*u8);
impl _wxDbTableInfo for wxDbTableInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbTableInfo {
}

trait _wxDbTableInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDebugContext(*u8);
impl _wxDebugContext for wxDebugContext { fn handle(&self) -> *u8 { **self } }

impl wxDebugContext {
}

trait _wxDebugContext {
    fn handle(&self) -> *u8;
    
}

struct wxDialUpEvent(*u8);
impl _wxDialUpEvent for wxDialUpEvent {}
impl _wxEvent for wxDialUpEvent {}
impl _wxObject for wxDialUpEvent { fn handle(&self) -> *u8 { **self } }

impl wxDialUpEvent {
}

trait _wxDialUpEvent : _wxEvent {
}

struct wxDialUpManager(*u8);
impl _wxDialUpManager for wxDialUpManager { fn handle(&self) -> *u8 { **self } }

impl wxDialUpManager {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxDialog {
        unsafe { wxDialog(wxDialog_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString, V: _wxString>(_prt: T, _msg: U, _dir: V, _lft: c_int, _top: c_int, _stl: c_int) -> wxDirDialog {
        unsafe { wxDirDialog(wxDirDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _lft, _top, _stl)) }
    }
}

trait _wxDirDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getMessage(&self) -> wxString {
        unsafe { wxString(wxDirDialog_GetMessage(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> wxString {
        unsafe { wxString(wxDirDialog_GetPath(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMessage<T: _wxString>(&self, msg: T) {
        unsafe { wxDirDialog_SetMessage(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath<T: _wxString>(&self, pth: T) {
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
}

trait _wxDirTraverser {
    fn handle(&self) -> *u8;
    
}

struct wxDllLoader(*u8);
impl _wxDllLoader for wxDllLoader { fn handle(&self) -> *u8 { **self } }

impl wxDllLoader {
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
}

trait _wxDocMDIParentFrame : _wxMDIParentFrame {
}

struct wxDocManager(*u8);
impl _wxDocManager for wxDocManager {}
impl _wxEvtHandler for wxDocManager {}
impl _wxObject for wxDocManager { fn handle(&self) -> *u8 { **self } }

impl wxDocManager {
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
}

trait _wxDocParentFrame : _wxFrame {
}

struct wxDocTemplate(*u8);
impl _wxDocTemplate for wxDocTemplate {}
impl _wxObject for wxDocTemplate { fn handle(&self) -> *u8 { **self } }

impl wxDocTemplate {
}

trait _wxDocTemplate : _wxObject {
}

struct wxDocument(*u8);
impl _wxDocument for wxDocument {}
impl _wxEvtHandler for wxDocument {}
impl _wxObject for wxDocument { fn handle(&self) -> *u8 { **self } }

impl wxDocument {
}

trait _wxDocument : _wxEvtHandler {
}

struct wxDragImage(*u8);
impl _wxDragImage for wxDragImage {}
impl _wxObject for wxDragImage { fn handle(&self) -> *u8 { **self } }

impl wxDragImage {
    #[fixed_stack_segment]
    pub fn new<T: _wxBitmap>(image: T, x: c_int, y: c_int) -> wxDragImage {
        unsafe { wxDragImage(wxDragImage_Create(image.handle(), x, y)) }
    }
}

trait _wxDragImage : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDragImage_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDragFullScreen<T: _wxWindow, U: _wxRect>(&self, x_pos: c_int, y_pos: c_int, window: T, fullScreen: bool, rect: U) -> bool {
        unsafe { wxDragImage_BeginDragFullScreen(self.handle(), x_pos, y_pos, window.handle(), fullScreen, rect.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDrag<T: _wxWindow, U: _wxWindow>(&self, x: c_int, y: c_int, window: T, boundingWindow: U) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxDrawControl {
        unsafe { wxDrawControl(wxDrawControl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxDrawWindow {
        unsafe { wxDrawWindow(wxDrawWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxDrawWindow : _wxWindow {
}

struct wxDropFilesEvent(*u8);
impl _wxDropFilesEvent for wxDropFilesEvent {}
impl _wxEvent for wxDropFilesEvent {}
impl _wxObject for wxDropFilesEvent { fn handle(&self) -> *u8 { **self } }

impl wxDropFilesEvent {
}

trait _wxDropFilesEvent : _wxEvent {
}

struct wxDropSource(*u8);
impl _wxDropSource for wxDropSource { fn handle(&self) -> *u8 { **self } }

impl wxDropSource {
}

trait _wxDropSource {
    fn handle(&self) -> *u8;
    
}

struct wxDropTarget(*u8);
impl _wxDropTarget for wxDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxDropTarget {
}

trait _wxDropTarget {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDataObject<T: _wxDataObject>(&self, _dat: T) {
        unsafe { wxDropTarget_SetDataObject(self.handle(), _dat.handle()) }
    }
}

struct wxDynToolInfo(*u8);
impl _wxDynToolInfo for wxDynToolInfo {}
impl _wxToolLayoutItem for wxDynToolInfo {}
impl _wxObject for wxDynToolInfo { fn handle(&self) -> *u8 { **self } }

impl wxDynToolInfo {
}

trait _wxDynToolInfo : _wxToolLayoutItem {
}

struct wxDynamicLibrary(*u8);
impl _wxDynamicLibrary for wxDynamicLibrary { fn handle(&self) -> *u8 { **self } }

impl wxDynamicLibrary {
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
}

trait _wxEditableListBox : _wxPanel {
}

struct wxEncodingConverter(*u8);
impl _wxEncodingConverter for wxEncodingConverter {}
impl _wxObject for wxEncodingConverter { fn handle(&self) -> *u8 { **self } }

impl wxEncodingConverter {
    #[fixed_stack_segment]
    pub fn new() -> wxEncodingConverter {
        unsafe { wxEncodingConverter(wxEncodingConverter_Create()) }
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
    fn getAllEquivalents<T: _wxList>(&self, enc: c_int, _lst: T) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.handle(), enc, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getPlatformEquivalents<T: _wxList>(&self, enc: c_int, platform: c_int, _lst: T) -> c_int {
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
}

trait _wxEraseEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxEraseEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getDC(&self) -> wxDC {
        unsafe { wxDC(wxEraseEvent_GetDC(self.handle())) }
    }
}

struct wxEvent(*u8);
impl _wxEvent for wxEvent {}
impl _wxObject for wxEvent { fn handle(&self) -> *u8 { **self } }

impl wxEvent {
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
    fn getEventObject(&self) -> wxObject {
        unsafe { wxObject(wxEvent_GetEventObject(self.handle())) }
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
    fn setEventObject<T: _wxObject>(&self, obj: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxEvtHandler {
        unsafe { wxEvtHandler(wxEvtHandler_Create()) }
    }
}

trait _wxEvtHandler : _wxObject {
    #[fixed_stack_segment]
    fn addPendingEvent<T: _wxEvent>(&self, event: T) {
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
    fn getNextHandler(&self) -> wxEvtHandler {
        unsafe { wxEvtHandler(wxEvtHandler_GetNextHandler(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPreviousHandler(&self) -> wxEvtHandler {
        unsafe { wxEvtHandler(wxEvtHandler_GetPreviousHandler(self.handle())) }
    }
    #[fixed_stack_segment]
    fn processEvent<T: _wxEvent>(&self, event: T) -> bool {
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
    fn setNextHandler<T: _wxEvtHandler>(&self, handler: T) {
        unsafe { wxEvtHandler_SetNextHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setPreviousHandler<T: _wxEvtHandler>(&self, handler: T) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self, id: c_int, type_: c_int) -> wxClosure {
        unsafe { wxClosure(wxEvtHandler_GetClosure(self.handle(), id, type_)) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> wxClosure {
        unsafe { wxClosure(wxEvtHandler_GetClientClosure(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setClientClosure<T: _wxClosure>(&self, closure: T) {
        unsafe { wxEvtHandler_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxExpr(*u8);
impl _wxExpr for wxExpr { fn handle(&self) -> *u8 { **self } }

impl wxExpr {
}

trait _wxExpr {
    fn handle(&self) -> *u8;
    
}

struct wxExprDatabase(*u8);
impl _wxExprDatabase for wxExprDatabase {}
impl _wxList for wxExprDatabase {}
impl _wxObject for wxExprDatabase { fn handle(&self) -> *u8 { **self } }

impl wxExprDatabase {
}

trait _wxExprDatabase : _wxList {
}

struct wxFFile(*u8);
impl _wxFFile for wxFFile { fn handle(&self) -> *u8 { **self } }

impl wxFFile {
}

trait _wxFFile {
    fn handle(&self) -> *u8;
    
}

struct wxFFileInputStream(*u8);
impl _wxFFileInputStream for wxFFileInputStream {}
impl _wxInputStream for wxFFileInputStream {}
impl _wxStreamBase for wxFFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileInputStream {
}

trait _wxFFileInputStream : _wxInputStream {
}

struct wxFFileOutputStream(*u8);
impl _wxFFileOutputStream for wxFFileOutputStream {}
impl _wxOutputStream for wxFFileOutputStream {}
impl _wxStreamBase for wxFFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileOutputStream {
}

trait _wxFFileOutputStream : _wxOutputStream {
}

struct wxFSFile(*u8);
impl _wxFSFile for wxFSFile {}
impl _wxObject for wxFSFile { fn handle(&self) -> *u8 { **self } }

impl wxFSFile {
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
}

trait _wxFTP : _wxProtocol {
}

struct wxFileDataObject(*u8);
impl _wxFileDataObject for wxFileDataObject {}
impl _wxDataObjectSimple for wxFileDataObject {}
impl _wxDataObject for wxFileDataObject { fn handle(&self) -> *u8 { **self } }

impl wxFileDataObject {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString, V: _wxString, W: _wxString, X: _wxString>(_prt: T, _msg: U, _dir: V, _fle: W, _wcd: X, _lft: c_int, _top: c_int, _stl: c_int) -> wxFileDialog {
        unsafe { wxFileDialog(wxFileDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _fle.handle(), _wcd.handle(), _lft, _top, _stl)) }
    }
}

trait _wxFileDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getDirectory(&self) -> wxString {
        unsafe { wxString(wxFileDialog_GetDirectory(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> wxString {
        unsafe { wxString(wxFileDialog_GetFilename(self.handle())) }
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
    fn getMessage(&self) -> wxString {
        unsafe { wxString(wxFileDialog_GetMessage(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> wxString {
        unsafe { wxString(wxFileDialog_GetPath(self.handle())) }
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
    fn getWildcard(&self) -> wxString {
        unsafe { wxString(wxFileDialog_GetWildcard(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setDirectory<T: _wxString>(&self, dir: T) {
        unsafe { wxFileDialog_SetDirectory(self.handle(), dir.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilename<T: _wxString>(&self, name: T) {
        unsafe { wxFileDialog_SetFilename(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.handle(), filterIndex) }
    }
    #[fixed_stack_segment]
    fn setMessage<T: _wxString>(&self, message: T) {
        unsafe { wxFileDialog_SetMessage(self.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath<T: _wxString>(&self, path: T) {
        unsafe { wxFileDialog_SetPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setWildcard<T: _wxString>(&self, wildCard: T) {
        unsafe { wxFileDialog_SetWildcard(self.handle(), wildCard.handle()) }
    }
}

struct wxFileDropTarget(*u8);
impl _wxFileDropTarget for wxFileDropTarget {}
impl _wxDropTarget for wxFileDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxFileDropTarget {
}

trait _wxFileDropTarget : _wxDropTarget {
}

struct wxFileHistory(*u8);
impl _wxFileHistory for wxFileHistory {}
impl _wxObject for wxFileHistory { fn handle(&self) -> *u8 { **self } }

impl wxFileHistory {
    #[fixed_stack_segment]
    pub fn new(maxFiles: c_int) -> wxFileHistory {
        unsafe { wxFileHistory(wxFileHistory_Create(maxFiles)) }
    }
}

trait _wxFileHistory : _wxObject {
    #[fixed_stack_segment]
    fn addFileToHistory<T: _wxString>(&self, file: T) {
        unsafe { wxFileHistory_AddFileToHistory(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn addFilesToMenu<T: _wxMenu>(&self, menu: T) {
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
    fn getHistoryFile(&self, i: c_int) -> wxString {
        unsafe { wxString(wxFileHistory_GetHistoryFile(self.handle(), i)) }
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
    fn load<T: _wxConfigBase>(&self, config: T) {
        unsafe { wxFileHistory_Load(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.handle(), i) }
    }
    #[fixed_stack_segment]
    fn removeMenu<T: _wxMenu>(&self, menu: T) {
        unsafe { wxFileHistory_RemoveMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn save<T: _wxConfigBase>(&self, config: T) {
        unsafe { wxFileHistory_Save(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn useMenu<T: _wxMenu>(&self, menu: T) {
        unsafe { wxFileHistory_UseMenu(self.handle(), menu.handle()) }
    }
}

struct wxFileInputStream(*u8);
impl _wxFileInputStream for wxFileInputStream {}
impl _wxInputStream for wxFileInputStream {}
impl _wxStreamBase for wxFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileInputStream {
}

trait _wxFileInputStream : _wxInputStream {
}

struct wxFileName(*u8);
impl _wxFileName for wxFileName { fn handle(&self) -> *u8 { **self } }

impl wxFileName {
}

trait _wxFileName {
    fn handle(&self) -> *u8;
    
}

struct wxFileOutputStream(*u8);
impl _wxFileOutputStream for wxFileOutputStream {}
impl _wxOutputStream for wxFileOutputStream {}
impl _wxStreamBase for wxFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileOutputStream {
}

trait _wxFileOutputStream : _wxOutputStream {
}

struct wxFileSystem(*u8);
impl _wxFileSystem for wxFileSystem {}
impl _wxObject for wxFileSystem { fn handle(&self) -> *u8 { **self } }

impl wxFileSystem {
}

trait _wxFileSystem : _wxObject {
}

struct wxFileSystemHandler(*u8);
impl _wxFileSystemHandler for wxFileSystemHandler {}
impl _wxObject for wxFileSystemHandler { fn handle(&self) -> *u8 { **self } }

impl wxFileSystemHandler {
}

trait _wxFileSystemHandler : _wxObject {
}

struct wxFileType(*u8);
impl _wxFileType for wxFileType { fn handle(&self) -> *u8 { **self } }

impl wxFileType {
}

trait _wxFileType {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn expandCommand(&self, _cmd: *u8, _params: *u8) -> wxString {
        unsafe { wxString(wxFileType_ExpandCommand(self.handle(), _cmd, _params)) }
    }
    #[fixed_stack_segment]
    fn getDescription(&self) -> wxString {
        unsafe { wxString(wxFileType_GetDescription(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getExtensions<T: _wxList>(&self, _lst: T) -> c_int {
        unsafe { wxFileType_GetExtensions(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon<T: _wxIcon>(&self, icon: T) -> c_int {
        unsafe { wxFileType_GetIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn getMimeType(&self) -> wxString {
        unsafe { wxString(wxFileType_GetMimeType(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getMimeTypes<T: _wxList>(&self, _lst: T) -> c_int {
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
}

trait _wxFilterInputStream : _wxInputStream {
}

struct wxFilterOutputStream(*u8);
impl _wxFilterOutputStream for wxFilterOutputStream {}
impl _wxOutputStream for wxFilterOutputStream {}
impl _wxStreamBase for wxFilterOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterOutputStream {
}

trait _wxFilterOutputStream : _wxOutputStream {
}

struct wxFindDialogEvent(*u8);
impl _wxFindDialogEvent for wxFindDialogEvent {}
impl _wxCommandEvent for wxFindDialogEvent {}
impl _wxEvent for wxFindDialogEvent {}
impl _wxObject for wxFindDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxFindDialogEvent {
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
    #[fixed_stack_segment]
    pub fn new(flags: c_int) -> wxFindReplaceData {
        unsafe { wxFindReplaceData(wxFindReplaceData_Create(flags)) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxFindReplaceData {
        unsafe { wxFindReplaceData(wxFindReplaceData_CreateDefault()) }
    }
}

trait _wxFindReplaceData : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFindReplaceData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFindString(&self) -> wxString {
        unsafe { wxString(wxFindReplaceData_GetFindString(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self) -> wxString {
        unsafe { wxString(wxFindReplaceData_GetReplaceString(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setFindString<T: _wxString>(&self, str: T) {
        unsafe { wxFindReplaceData_SetFindString(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn setReplaceString<T: _wxString>(&self, str: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxFindReplaceData, V: _wxString>(parent: T, data: U, title: V, style: c_int) -> wxFindReplaceDialog {
        unsafe { wxFindReplaceDialog(wxFindReplaceDialog_Create(parent.handle(), data.handle(), title.handle(), style)) }
    }
}

trait _wxFindReplaceDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getData(&self) -> wxFindReplaceData {
        unsafe { wxFindReplaceData(wxFindReplaceDialog_GetData(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setData<T: _wxFindReplaceData>(&self, data: T) {
        unsafe { wxFindReplaceDialog_SetData(self.handle(), data.handle()) }
    }
}

struct wxFlexGridSizer(*u8);
impl _wxFlexGridSizer for wxFlexGridSizer {}
impl _wxGridSizer for wxFlexGridSizer {}
impl _wxSizer for wxFlexGridSizer {}
impl _wxObject for wxFlexGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxFlexGridSizer {
    #[fixed_stack_segment]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> wxFlexGridSizer {
        unsafe { wxFlexGridSizer(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) }
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
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize(wxFlexGridSizer_CalcMin(self.handle())) }
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
}

trait _wxFocusEvent : _wxEvent {
}

struct wxFont(*u8);
impl _wxFont for wxFont {}
impl _wxGDIObject for wxFont {}
impl _wxObject for wxFont { fn handle(&self) -> *u8 { **self } }

impl wxFont {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: bool, face: T, enc: c_int) -> wxFont {
        unsafe { wxFont(wxFont_Create(pointSize, family, style, weight, underlined, face.handle(), enc)) }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> wxFont {
        unsafe { wxFont(wxFont_CreateFromStock(id)) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxFont {
        unsafe { wxFont(wxFont_CreateDefault()) }
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
    fn getFaceName(&self) -> wxString {
        unsafe { wxString(wxFont_GetFaceName(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFamilyString(&self) -> wxString {
        unsafe { wxString(wxFont_GetFamilyString(self.handle())) }
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
    fn getStyleString(&self) -> wxString {
        unsafe { wxString(wxFont_GetStyleString(self.handle())) }
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
    fn getWeightString(&self) -> wxString {
        unsafe { wxString(wxFont_GetWeightString(self.handle())) }
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
    fn setFaceName<T: _wxString>(&self, faceName: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxFontData {
        unsafe { wxFontData(wxFontData_Create()) }
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
    fn getChosenFont<T: _wxFont>(&self, ref_: T) {
        unsafe { wxFontData_GetChosenFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour<T: _wxColour>(&self, _ref: T) {
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
    fn getInitialFont<T: _wxFont>(&self, ref_: T) {
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
    fn setChosenFont<T: _wxFont>(&self, font: T) {
        unsafe { wxFontData_SetChosenFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxFontData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    fn setInitialFont<T: _wxFont>(&self, font: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxFontData>(_prt: T, fnt: U) -> wxFontDialog {
        unsafe { wxFontDialog(wxFontDialog_Create(_prt.handle(), fnt.handle())) }
    }
}

trait _wxFontDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getFontData<T: _wxFontData>(&self, _ref: T) {
        unsafe { wxFontDialog_GetFontData(self.handle(), _ref.handle()) }
    }
}

struct wxFontEnumerator(*u8);
impl _wxFontEnumerator for wxFontEnumerator { fn handle(&self) -> *u8 { **self } }

impl wxFontEnumerator {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8) -> wxFontEnumerator {
        unsafe { wxFontEnumerator(wxFontEnumerator_Create(_obj, _fnc)) }
    }
}

trait _wxFontEnumerator {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateEncodings<T: _wxString>(&self, facename: T) -> bool {
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
}

trait _wxFontList : _wxList {
}

struct wxFontMapper(*u8);
impl _wxFontMapper for wxFontMapper { fn handle(&self) -> *u8 { **self } }

impl wxFontMapper {
    #[fixed_stack_segment]
    pub fn new() -> wxFontMapper {
        unsafe { wxFontMapper(wxFontMapper_Create()) }
    }
}

trait _wxFontMapper {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getAltForEncoding<T: _wxString>(&self, encoding: c_int, alt_encoding: *u8, _buf: T) -> bool {
        unsafe { wxFontMapper_GetAltForEncoding(self.handle(), encoding, alt_encoding, _buf.handle()) }
    }
    #[fixed_stack_segment]
    fn isEncodingAvailable<T: _wxString>(&self, encoding: c_int, _buf: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxFrame {
        unsafe { wxFrame(wxFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxFrame : _wxTopLevelWindow {
    #[fixed_stack_segment]
    fn newStatusBar(&self, number: c_int, style: c_int) -> wxStatusBar {
        unsafe { wxStatusBar(wxFrame_CreateStatusBar(self.handle(), number, style)) }
    }
    #[fixed_stack_segment]
    fn newToolBar(&self, style: c_long) -> wxToolBar {
        unsafe { wxToolBar(wxFrame_CreateToolBar(self.handle(), style)) }
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
    fn getMenuBar(&self) -> wxMenuBar {
        unsafe { wxMenuBar(wxFrame_GetMenuBar(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getStatusBar(&self) -> wxStatusBar {
        unsafe { wxStatusBar(wxFrame_GetStatusBar(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getToolBar(&self) -> wxToolBar {
        unsafe { wxToolBar(wxFrame_GetToolBar(self.handle())) }
    }
    #[fixed_stack_segment]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMenuBar<T: _wxMenuBar>(&self, menubar: T) {
        unsafe { wxFrame_SetMenuBar(self.handle(), menubar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusBar<T: _wxStatusBar>(&self, statBar: T) {
        unsafe { wxFrame_SetStatusBar(self.handle(), statBar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusText<T: _wxString>(&self, _txt: T, _number: c_int) {
        unsafe { wxFrame_SetStatusText(self.handle(), _txt.handle(), _number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, _n: c_int, _widths_field: *u8) {
        unsafe { wxFrame_SetStatusWidths(self.handle(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    fn setToolBar<T: _wxToolBar>(&self, _toolbar: T) {
        unsafe { wxFrame_SetToolBar(self.handle(), _toolbar.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> wxString {
        unsafe { wxString(wxFrame_GetTitle(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setTitle<T: _wxString>(&self, _txt: T) {
        unsafe { wxFrame_SetTitle(self.handle(), _txt.handle()) }
    }
    #[fixed_stack_segment]
    fn setShape<T: _wxRegion>(&self, region: T) -> bool {
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
}

trait _wxFrameLayout : _wxEvtHandler {
}

struct wxGDIObject(*u8);
impl _wxGDIObject for wxGDIObject {}
impl _wxObject for wxGDIObject { fn handle(&self) -> *u8 { **self } }

impl wxGDIObject {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString, V: _wxPalette>(parent: T, windowID: c_int, attributes: *c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: U, palette: V) -> wxGLCanvas {
        unsafe { wxGLCanvas(wxGLCanvas_Create(parent.handle(), windowID, attributes, x, y, w, h, style, title.handle(), palette.handle())) }
    }
    #[fixed_stack_segment]
    pub fn isDisplaySupported(attributes: *c_int) -> bool {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    #[fixed_stack_segment]
    pub fn isExtensionSupported<T: _wxString>(extension: T) -> bool {
        unsafe { wxGLCanvas_IsExtensionSupported(extension.handle()) }
    }
}

trait _wxGLCanvas : _wxScrolledWindow {
    #[fixed_stack_segment]
    fn setColour<T: _wxColour>(&self, colour: T) -> bool {
        unsafe { wxGLCanvas_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrent<T: _wxGLContext>(&self, ctxt: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxGauge {
        unsafe { wxGauge(wxGauge_Create(_prt.handle(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) }
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
}

trait _wxGenericDirCtrl : _wxControl {
}

struct wxGenericValidator(*u8);
impl _wxGenericValidator for wxGenericValidator {}
impl _wxValidator for wxGenericValidator {}
impl _wxEvtHandler for wxGenericValidator {}
impl _wxObject for wxGenericValidator { fn handle(&self) -> *u8 { **self } }

impl wxGenericValidator {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxGrid {
        unsafe { wxGrid(wxGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> wxRect {
        unsafe { wxRect(wxGrid_BlockToDeviceRect(self.handle(), top, left, bottom, right)) }
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
    fn cellToRect(&self, row: c_int, col: c_int) -> wxRect {
        unsafe { wxRect(wxGrid_CellToRect(self.handle(), row, col)) }
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
    fn drawAllGridLines<T: _wxDC, U: _wxRegion>(&self, dc: T, reg: U) {
        unsafe { wxGrid_DrawAllGridLines(self.handle(), dc.handle(), reg.handle()) }
    }
    #[fixed_stack_segment]
    fn drawCell<T: _wxDC>(&self, dc: T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellBorder<T: _wxDC>(&self, dc: T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellHighlight<T: _wxDC, U: _wxGridCellAttr>(&self, dc: T, attr: U) {
        unsafe { wxGrid_DrawCellHighlight(self.handle(), dc.handle(), attr.handle()) }
    }
    #[fixed_stack_segment]
    fn drawColLabel<T: _wxDC>(&self, dc: T, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.handle(), dc.handle(), col) }
    }
    #[fixed_stack_segment]
    fn drawColLabels<T: _wxDC>(&self, dc: T) {
        unsafe { wxGrid_DrawColLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawGridSpace<T: _wxDC>(&self, dc: T) {
        unsafe { wxGrid_DrawGridSpace(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawRowLabel<T: _wxDC>(&self, dc: T, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.handle(), dc.handle(), row) }
    }
    #[fixed_stack_segment]
    fn drawRowLabels<T: _wxDC>(&self, dc: T) {
        unsafe { wxGrid_DrawRowLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawTextRectangle<T: _wxDC, U: _wxString>(&self, dc: T, txt: U, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
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
    fn getCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: T) {
        unsafe { wxGrid_GetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellEditor(&self, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor(wxGrid_GetCellEditor(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    fn getCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: T) {
        unsafe { wxGrid_GetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellHighlightColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGrid_GetCellHighlightColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellRenderer(&self, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer(wxGrid_GetCellRenderer(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    fn getCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: T) {
        unsafe { wxGrid_GetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellValue(&self, row: c_int, col: c_int) -> wxString {
        unsafe { wxString(wxGrid_GetCellValue(self.handle(), row, col)) }
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
    fn getColLabelValue(&self, col: c_int) -> wxString {
        unsafe { wxString(wxGrid_GetColLabelValue(self.handle(), col)) }
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
    fn getDefaultCellBackgroundColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellFont<T: _wxFont>(&self, _ref: T) {
        unsafe { wxGrid_GetDefaultCellFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellTextColour<T: _wxColour>(&self, _ref: T) {
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
    fn getDefaultEditor(&self) -> wxGridCellEditor {
        unsafe { wxGridCellEditor(wxGrid_GetDefaultEditor(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor(wxGrid_GetDefaultEditorForCell(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForType<T: _wxString>(&self, typeName: T) -> wxGridCellEditor {
        unsafe { wxGridCellEditor(wxGrid_GetDefaultEditorForType(self.handle(), typeName.handle())) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer(wxGrid_GetDefaultRenderer(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer(wxGrid_GetDefaultRendererForCell(self.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForType<T: _wxString>(&self, typeName: T) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer(wxGrid_GetDefaultRendererForType(self.handle(), typeName.handle())) }
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
    fn getGridLineColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGrid_GetGridLineColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelBackgroundColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelFont<T: _wxFont>(&self, _ref: T) {
        unsafe { wxGrid_GetLabelFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelTextColour<T: _wxColour>(&self, _ref: T) {
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
    fn getRowLabelValue(&self, row: c_int) -> wxString {
        unsafe { wxString(wxGrid_GetRowLabelValue(self.handle(), row)) }
    }
    #[fixed_stack_segment]
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.handle(), row) }
    }
    #[fixed_stack_segment]
    fn getSelectionBackground<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGrid_GetSelectionBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionForeground<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGrid_GetSelectionForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTable(&self) -> wxGridTableBase {
        unsafe { wxGridTableBase(wxGrid_GetTable(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTextBoxSize<T: _wxDC>(&self, dc: T, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int) {
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
    fn processTableMessage<T: _wxEvent>(&self, evt: T) -> bool {
        unsafe { wxGrid_ProcessTableMessage(self.handle(), evt.handle()) }
    }
    #[fixed_stack_segment]
    fn registerDataType<T: _wxString, U: _wxGridCellRenderer, V: _wxGridCellEditor>(&self, typeName: T, renderer: U, editor: V) {
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
    fn setCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: T) {
        unsafe { wxGrid_SetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellEditor<T: _wxGridCellEditor>(&self, row: c_int, col: c_int, editor: T) {
        unsafe { wxGrid_SetCellEditor(self.handle(), row, col, editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: T) {
        unsafe { wxGrid_SetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellHighlightColour<T: _wxColour>(&self, col: T) {
        unsafe { wxGrid_SetCellHighlightColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellRenderer<T: _wxGridCellRenderer>(&self, row: c_int, col: c_int, renderer: T) {
        unsafe { wxGrid_SetCellRenderer(self.handle(), row, col, renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: T) {
        unsafe { wxGrid_SetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellValue<T: _wxString>(&self, row: c_int, col: c_int, s: T) {
        unsafe { wxGrid_SetCellValue(self.handle(), row, col, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setColAttr<T: _wxGridCellAttr>(&self, col: c_int, attr: T) {
        unsafe { wxGrid_SetColAttr(self.handle(), col, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setColFormatCustom<T: _wxString>(&self, col: c_int, typeName: T) {
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
    fn setColLabelValue<T: _wxString>(&self, col: c_int, label: T) {
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
    fn setDefaultCellBackgroundColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellFont<T: _wxFont>(&self, font: T) {
        unsafe { wxGrid_SetDefaultCellFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellTextColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.handle(), width, resizeExistingCols) }
    }
    #[fixed_stack_segment]
    fn setDefaultEditor<T: _wxGridCellEditor>(&self, editor: T) {
        unsafe { wxGrid_SetDefaultEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultRenderer<T: _wxGridCellRenderer>(&self, renderer: T) {
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
    fn setGridLineColour<T: _wxColour>(&self, col: T) {
        unsafe { wxGrid_SetGridLineColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelBackgroundColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelFont<T: _wxFont>(&self, font: T) {
        unsafe { wxGrid_SetLabelFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTextColour<T: _wxColour>(&self, colour: T) {
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
    fn setRowAttr<T: _wxGridCellAttr>(&self, row: c_int, attr: T) {
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
    fn setRowLabelValue<T: _wxString>(&self, row: c_int, label: T) {
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
    fn setSelectionBackground<T: _wxColour>(&self, c: T) {
        unsafe { wxGrid_SetSelectionBackground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionForeground<T: _wxColour>(&self, c: T) {
        unsafe { wxGrid_SetSelectionForeground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.handle(), selmode) }
    }
    #[fixed_stack_segment]
    fn setTable<T: _wxGridTableBase>(&self, table: T, takeOwnership: bool, selmode: c_int) -> bool {
        unsafe { wxGrid_SetTable(self.handle(), table.handle(), takeOwnership, selmode) }
    }
    #[fixed_stack_segment]
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn stringToLines<T: _wxString>(&self, value: T, lines: *u8) -> c_int {
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
    fn getSelectedCells<T: _wxGridCellCoordsArray>(&self, _arr: T) {
        unsafe { wxGrid_GetSelectedCells(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockTopLeft<T: _wxGridCellCoordsArray>(&self, _arr: T) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockBottomRight<T: _wxGridCellCoordsArray>(&self, _arr: T) {
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
    #[fixed_stack_segment]
    pub fn ctor() -> wxGridCellAttr {
        unsafe { wxGridCellAttr(wxGridCellAttr_Ctor()) }
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
    fn getBackgroundColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditor<T: _wxGrid>(&self, grid: T, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor(wxGridCellAttr_GetEditor(self.handle(), grid.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    fn getFont<T: _wxFont>(&self, _ref: T) {
        unsafe { wxGridCellAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getRenderer<T: _wxGrid>(&self, grid: T, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer(wxGridCellAttr_GetRenderer(self.handle(), grid.handle(), row, col)) }
    }
    #[fixed_stack_segment]
    fn getTextColour<T: _wxColour>(&self, _ref: T) {
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
    fn setBackgroundColour<T: _wxColour>(&self, colBack: T) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefAttr<T: _wxGridCellAttr>(&self, defAttr: T) {
        unsafe { wxGridCellAttr_SetDefAttr(self.handle(), defAttr.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditor<T: _wxGridCellEditor>(&self, editor: T) {
        unsafe { wxGridCellAttr_SetEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont<T: _wxFont>(&self, font: T) {
        unsafe { wxGridCellAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, isReadOnly: bool) {
        unsafe { wxGridCellAttr_SetReadOnly(self.handle(), isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRenderer<T: _wxGridCellRenderer>(&self, renderer: T) {
        unsafe { wxGridCellAttr_SetRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour<T: _wxColour>(&self, colText: T) {
        unsafe { wxGridCellAttr_SetTextColour(self.handle(), colText.handle()) }
    }
}

struct wxGridCellBoolEditor(*u8);
impl _wxGridCellBoolEditor for wxGridCellBoolEditor {}
impl _wxGridCellEditor for wxGridCellBoolEditor {}
impl _wxGridCellWorker for wxGridCellBoolEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolEditor {
    #[fixed_stack_segment]
    pub fn ctor() -> wxGridCellBoolEditor {
        unsafe { wxGridCellBoolEditor(wxGridCellBoolEditor_Ctor()) }
    }
}

trait _wxGridCellBoolEditor : _wxGridCellEditor {
}

struct wxGridCellBoolRenderer(*u8);
impl _wxGridCellBoolRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellWorker for wxGridCellBoolRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolRenderer {
}

trait _wxGridCellBoolRenderer : _wxGridCellRenderer {
}

struct wxGridCellChoiceEditor(*u8);
impl _wxGridCellChoiceEditor for wxGridCellChoiceEditor {}
impl _wxGridCellEditor for wxGridCellChoiceEditor {}
impl _wxGridCellWorker for wxGridCellChoiceEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellChoiceEditor {
    #[fixed_stack_segment]
    pub fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> wxGridCellChoiceEditor {
        unsafe { wxGridCellChoiceEditor(wxGridCellChoiceEditor_Ctor(count, choices, allowOthers)) }
    }
}

trait _wxGridCellChoiceEditor : _wxGridCellEditor {
}

struct wxGridCellCoordsArray(*u8);
impl _wxGridCellCoordsArray for wxGridCellCoordsArray { fn handle(&self) -> *u8 { **self } }

impl wxGridCellCoordsArray {
    #[fixed_stack_segment]
    pub fn new() -> wxGridCellCoordsArray {
        unsafe { wxGridCellCoordsArray(wxGridCellCoordsArray_Create()) }
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
}

trait _wxGridCellEditor : _wxGridCellWorker {
    #[fixed_stack_segment]
    fn beginEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: T) {
        unsafe { wxGridCellEditor_BeginEdit(self.handle(), row, col, grid.handle()) }
    }
    #[fixed_stack_segment]
    fn new<T: _wxWindow, U: _wxEvtHandler>(&self, parent: T, id: c_int, evtHandler: U) {
        unsafe { wxGridCellEditor_Create(self.handle(), parent.handle(), id, evtHandler.handle()) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endEdit<T: _wxGrid, U: _wxString, V: _wxString>(&self, row: c_int, col: c_int, grid: T, oldStr: U, newStr: V) -> c_int {
        unsafe { wxGridCellEditor_EndEdit(self.handle(), row, col, grid.handle(), oldStr.handle(), newStr.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> wxControl {
        unsafe { wxControl(wxGridCellEditor_GetControl(self.handle())) }
    }
    #[fixed_stack_segment]
    fn handleReturn<T: _wxEvent>(&self, event: T) {
        unsafe { wxGridCellEditor_HandleReturn(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isAcceptedKey<T: _wxEvent>(&self, event: T) -> bool {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isCreated(&self) -> bool {
        unsafe { wxGridCellEditor_IsCreated(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paintBackground<T: _wxGridCellAttr>(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: T) {
        unsafe { wxGridCellEditor_PaintBackground(self.handle(), x, y, w, h, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setControl<T: _wxControl>(&self, control: T) {
        unsafe { wxGridCellEditor_SetControl(self.handle(), control.handle()) }
    }
    #[fixed_stack_segment]
    fn setParameters<T: _wxString>(&self, params: T) {
        unsafe { wxGridCellEditor_SetParameters(self.handle(), params.handle()) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn show<T: _wxGridCellAttr>(&self, show: c_int, attr: T) {
        unsafe { wxGridCellEditor_Show(self.handle(), show, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn startingKey<T: _wxEvent>(&self, event: T) {
        unsafe { wxGridCellEditor_StartingKey(self.handle(), event.handle()) }
    }
}

struct wxGridCellFloatEditor(*u8);
impl _wxGridCellFloatEditor for wxGridCellFloatEditor {}
impl _wxGridCellTextEditor for wxGridCellFloatEditor {}
impl _wxGridCellEditor for wxGridCellFloatEditor {}
impl _wxGridCellWorker for wxGridCellFloatEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellFloatEditor {
    #[fixed_stack_segment]
    pub fn ctor(width: c_int, precision: c_int) -> wxGridCellFloatEditor {
        unsafe { wxGridCellFloatEditor(wxGridCellFloatEditor_Ctor(width, precision)) }
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
}

trait _wxGridCellFloatRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellNumberEditor(*u8);
impl _wxGridCellNumberEditor for wxGridCellNumberEditor {}
impl _wxGridCellTextEditor for wxGridCellNumberEditor {}
impl _wxGridCellEditor for wxGridCellNumberEditor {}
impl _wxGridCellWorker for wxGridCellNumberEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellNumberEditor {
    #[fixed_stack_segment]
    pub fn ctor(min: c_int, max: c_int) -> wxGridCellNumberEditor {
        unsafe { wxGridCellNumberEditor(wxGridCellNumberEditor_Ctor(min, max)) }
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
    #[fixed_stack_segment]
    pub fn ctor() -> wxGridCellNumberRenderer {
        unsafe { wxGridCellNumberRenderer(wxGridCellNumberRenderer_Ctor()) }
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
    #[fixed_stack_segment]
    pub fn ctor() -> wxGridCellAutoWrapStringRenderer {
        unsafe { wxGridCellAutoWrapStringRenderer(wxGridCellAutoWrapStringRenderer_Ctor()) }
    }
}

trait _wxGridCellAutoWrapStringRenderer : _wxGridCellStringRenderer {
}

struct wxGridCellRenderer(*u8);
impl _wxGridCellRenderer for wxGridCellRenderer {}
impl _wxGridCellWorker for wxGridCellRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellRenderer {
}

trait _wxGridCellRenderer : _wxGridCellWorker {
}

struct wxGridCellStringRenderer(*u8);
impl _wxGridCellStringRenderer for wxGridCellStringRenderer {}
impl _wxGridCellRenderer for wxGridCellStringRenderer {}
impl _wxGridCellWorker for wxGridCellStringRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellStringRenderer {
}

trait _wxGridCellStringRenderer : _wxGridCellRenderer {
}

struct wxGridCellTextEditor(*u8);
impl _wxGridCellTextEditor for wxGridCellTextEditor {}
impl _wxGridCellEditor for wxGridCellTextEditor {}
impl _wxGridCellWorker for wxGridCellTextEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellTextEditor {
    #[fixed_stack_segment]
    pub fn ctor() -> wxGridCellTextEditor {
        unsafe { wxGridCellTextEditor(wxGridCellTextEditor_Ctor()) }
    }
}

trait _wxGridCellTextEditor : _wxGridCellEditor {
}

struct wxGridCellWorker(*u8);
impl _wxGridCellWorker for wxGridCellWorker { fn handle(&self) -> *u8 { **self } }

impl wxGridCellWorker {
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
}

trait _wxGridEditorCreatedEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> wxControl {
        unsafe { wxControl(wxGridEditorCreatedEvent_GetControl(self.handle())) }
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
    fn setControl<T: _wxControl>(&self, ctrl: T) {
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
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxGridEvent_GetPosition(self.handle())) }
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
}

trait _wxGridSizeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxGridSizeEvent_GetPosition(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> wxGridSizer {
        unsafe { wxGridSizer(wxGridSizer_Create(rows, cols, vgap, hgap)) }
    }
}

trait _wxGridSizer : _wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize(wxGridSizer_CalcMin(self.handle())) }
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
}

trait _wxHTTP : _wxProtocol {
}

struct wxHashMap(*u8);
impl _wxHashMap for wxHashMap { fn handle(&self) -> *u8 { **self } }

impl wxHashMap {
}

trait _wxHashMap {
    fn handle(&self) -> *u8;
    
}

struct wxHelpController(*u8);
impl _wxHelpController for wxHelpController {}
impl _wxHelpControllerBase for wxHelpController {}
impl _wxObject for wxHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHelpController {
}

trait _wxHelpController : _wxHelpControllerBase {
}

struct wxHelpControllerBase(*u8);
impl _wxHelpControllerBase for wxHelpControllerBase {}
impl _wxObject for wxHelpControllerBase { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerBase {
}

trait _wxHelpControllerBase : _wxObject {
}

struct wxHelpControllerHelpProvider(*u8);
impl _wxHelpControllerHelpProvider for wxHelpControllerHelpProvider {}
impl _wxSimpleHelpProvider for wxHelpControllerHelpProvider {}
impl _wxHelpProvider for wxHelpControllerHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerHelpProvider {
    #[fixed_stack_segment]
    pub fn new<T: _wxHelpControllerBase>(ctr: T) -> wxHelpControllerHelpProvider {
        unsafe { wxHelpControllerHelpProvider(wxHelpControllerHelpProvider_Create(ctr.handle())) }
    }
}

trait _wxHelpControllerHelpProvider : _wxSimpleHelpProvider {
    #[fixed_stack_segment]
    fn getHelpController(&self) -> wxHelpControllerBase {
        unsafe { wxHelpControllerBase(wxHelpControllerHelpProvider_GetHelpController(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setHelpController<T: _wxHelpController>(&self, hc: T) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.handle(), hc.handle()) }
    }
}

struct wxHelpEvent(*u8);
impl _wxHelpEvent for wxHelpEvent {}
impl _wxCommandEvent for wxHelpEvent {}
impl _wxEvent for wxHelpEvent {}
impl _wxObject for wxHelpEvent { fn handle(&self) -> *u8 { **self } }

impl wxHelpEvent {
}

trait _wxHelpEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getLink(&self) -> wxString {
        unsafe { wxString(wxHelpEvent_GetLink(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxHelpEvent_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> wxString {
        unsafe { wxString(wxHelpEvent_GetTarget(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setLink<T: _wxString>(&self, link: T) {
        unsafe { wxHelpEvent_SetLink(self.handle(), link.handle()) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setTarget<T: _wxString>(&self, target: T) {
        unsafe { wxHelpEvent_SetTarget(self.handle(), target.handle()) }
    }
}

struct wxHelpProvider(*u8);
impl _wxHelpProvider for wxHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpProvider {
    #[fixed_stack_segment]
    pub fn get() -> wxHelpProvider {
        unsafe { wxHelpProvider(wxHelpProvider_Get()) }
    }
}

trait _wxHelpProvider {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addHelp<T: _wxWindow, U: _wxString>(&self, window: T, text: U) {
        unsafe { wxHelpProvider_AddHelp(self.handle(), window.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addHelpById<T: _wxString>(&self, id: c_int, text: T) {
        unsafe { wxHelpProvider_AddHelpById(self.handle(), id, text.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelp<T: _wxWindow>(&self, window: T) -> wxString {
        unsafe { wxString(wxHelpProvider_GetHelp(self.handle(), window.handle())) }
    }
    #[fixed_stack_segment]
    fn removeHelp<T: _wxWindow>(&self, window: T) {
        unsafe { wxHelpProvider_RemoveHelp(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self) -> wxHelpProvider {
        unsafe { wxHelpProvider(wxHelpProvider_Set(self.handle())) }
    }
    #[fixed_stack_segment]
    fn showHelp<T: _wxWindow>(&self, window: T) -> bool {
        unsafe { wxHelpProvider_ShowHelp(self.handle(), window.handle()) }
    }
}

struct wxHtmlCell(*u8);
impl _wxHtmlCell for wxHtmlCell {}
impl _wxObject for wxHtmlCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlCell {
}

trait _wxHtmlCell : _wxObject {
}

struct wxHtmlColourCell(*u8);
impl _wxHtmlColourCell for wxHtmlColourCell {}
impl _wxHtmlCell for wxHtmlColourCell {}
impl _wxObject for wxHtmlColourCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlColourCell {
}

trait _wxHtmlColourCell : _wxHtmlCell {
}

struct wxHtmlContainerCell(*u8);
impl _wxHtmlContainerCell for wxHtmlContainerCell {}
impl _wxHtmlCell for wxHtmlContainerCell {}
impl _wxObject for wxHtmlContainerCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlContainerCell {
}

trait _wxHtmlContainerCell : _wxHtmlCell {
}

struct wxHtmlDCRenderer(*u8);
impl _wxHtmlDCRenderer for wxHtmlDCRenderer {}
impl _wxObject for wxHtmlDCRenderer { fn handle(&self) -> *u8 { **self } }

impl wxHtmlDCRenderer {
}

trait _wxHtmlDCRenderer : _wxObject {
}

struct wxHtmlEasyPrinting(*u8);
impl _wxHtmlEasyPrinting for wxHtmlEasyPrinting {}
impl _wxObject for wxHtmlEasyPrinting { fn handle(&self) -> *u8 { **self } }

impl wxHtmlEasyPrinting {
}

trait _wxHtmlEasyPrinting : _wxObject {
}

struct wxHtmlFilter(*u8);
impl _wxHtmlFilter for wxHtmlFilter {}
impl _wxObject for wxHtmlFilter { fn handle(&self) -> *u8 { **self } }

impl wxHtmlFilter {
}

trait _wxHtmlFilter : _wxObject {
}

struct wxHtmlHelpController(*u8);
impl _wxHtmlHelpController for wxHtmlHelpController {}
impl _wxHelpControllerBase for wxHtmlHelpController {}
impl _wxObject for wxHtmlHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpController {
    #[fixed_stack_segment]
    pub fn new(_style: c_int) -> wxHtmlHelpController {
        unsafe { wxHtmlHelpController(wxHtmlHelpController_Create(_style)) }
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
    fn displaySection<T: _wxString>(&self, section: T) -> bool {
        unsafe { wxHtmlHelpController_DisplaySection(self.handle(), section.handle()) }
    }
    #[fixed_stack_segment]
    fn displaySectionNumber(&self, sectionNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.handle(), sectionNo) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame(wxHtmlHelpController_GetFrame(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFrameParameters(&self, title: *u8, width: *c_int, height: *c_int, pos_x: *c_int, pos_y: *c_int, newFrameEachTime: *c_int) -> *u8 {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn initialize<T: _wxString>(&self, file: T) -> bool {
        unsafe { wxHtmlHelpController_Initialize(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn keywordSearch<T: _wxString>(&self, keyword: T) -> bool {
        unsafe { wxHtmlHelpController_KeywordSearch(self.handle(), keyword.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile<T: _wxString>(&self, file: T) -> bool {
        unsafe { wxHtmlHelpController_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn quit(&self) -> bool {
        unsafe { wxHtmlHelpController_Quit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization<T: _wxConfigBase, U: _wxString>(&self, cfg: T, path: U) {
        unsafe { wxHtmlHelpController_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setFrameParameters(&self, title: *u8, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: bool) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn setTempDir<T: _wxString>(&self, path: T) {
        unsafe { wxHtmlHelpController_SetTempDir(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitleFormat(&self, format: *u8) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.handle(), format) }
    }
    #[fixed_stack_segment]
    fn setViewer<T: _wxString>(&self, viewer: T, flags: c_int) {
        unsafe { wxHtmlHelpController_SetViewer(self.handle(), viewer.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn useConfig<T: _wxConfigBase, U: _wxString>(&self, config: T, rootpath: U) {
        unsafe { wxHtmlHelpController_UseConfig(self.handle(), config.handle(), rootpath.handle()) }
    }
    #[fixed_stack_segment]
    fn writeCustomization<T: _wxConfigBase, U: _wxString>(&self, cfg: T, path: U) {
        unsafe { wxHtmlHelpController_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxHtmlHelpData(*u8);
impl _wxHtmlHelpData for wxHtmlHelpData {}
impl _wxObject for wxHtmlHelpData { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpData {
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
}

trait _wxHtmlHelpFrame : _wxFrame {
}

struct wxHtmlLinkInfo(*u8);
impl _wxHtmlLinkInfo for wxHtmlLinkInfo {}
impl _wxObject for wxHtmlLinkInfo { fn handle(&self) -> *u8 { **self } }

impl wxHtmlLinkInfo {
}

trait _wxHtmlLinkInfo : _wxObject {
}

struct wxHtmlParser(*u8);
impl _wxHtmlParser for wxHtmlParser {}
impl _wxObject for wxHtmlParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlParser {
}

trait _wxHtmlParser : _wxObject {
}

struct wxHtmlPrintout(*u8);
impl _wxHtmlPrintout for wxHtmlPrintout {}
impl _wxPrintout for wxHtmlPrintout {}
impl _wxObject for wxHtmlPrintout { fn handle(&self) -> *u8 { **self } }

impl wxHtmlPrintout {
}

trait _wxHtmlPrintout : _wxPrintout {
}

struct wxHtmlTag(*u8);
impl _wxHtmlTag for wxHtmlTag {}
impl _wxObject for wxHtmlTag { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTag {
}

trait _wxHtmlTag : _wxObject {
}

struct wxHtmlTagHandler(*u8);
impl _wxHtmlTagHandler for wxHtmlTagHandler {}
impl _wxObject for wxHtmlTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagHandler {
}

trait _wxHtmlTagHandler : _wxObject {
}

struct wxHtmlTagsModule(*u8);
impl _wxHtmlTagsModule for wxHtmlTagsModule {}
impl _wxModule for wxHtmlTagsModule {}
impl _wxObject for wxHtmlTagsModule { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagsModule {
}

trait _wxHtmlTagsModule : _wxModule {
}

struct wxHtmlWidgetCell(*u8);
impl _wxHtmlWidgetCell for wxHtmlWidgetCell {}
impl _wxHtmlCell for wxHtmlWidgetCell {}
impl _wxObject for wxHtmlWidgetCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWidgetCell {
}

trait _wxHtmlWidgetCell : _wxHtmlCell {
}

struct wxHtmlWinParser(*u8);
impl _wxHtmlWinParser for wxHtmlWinParser {}
impl _wxHtmlParser for wxHtmlWinParser {}
impl _wxObject for wxHtmlWinParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinParser {
}

trait _wxHtmlWinParser : _wxHtmlParser {
}

struct wxHtmlWinTagHandler(*u8);
impl _wxHtmlWinTagHandler for wxHtmlWinTagHandler {}
impl _wxHtmlTagHandler for wxHtmlWinTagHandler {}
impl _wxObject for wxHtmlWinTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinTagHandler {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: U) -> wxHtmlWindow {
        unsafe { wxHtmlWindow(wxHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) }
    }
}

trait _wxHtmlWindow : _wxScrolledWindow {
    #[fixed_stack_segment]
    fn appendToPage<T: _wxString>(&self, source: T) -> bool {
        unsafe { wxHtmlWindow_AppendToPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn getInternalRepresentation(&self) -> wxHtmlContainerCell {
        unsafe { wxHtmlContainerCell(wxHtmlWindow_GetInternalRepresentation(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getOpenedAnchor(&self) -> wxString {
        unsafe { wxString(wxHtmlWindow_GetOpenedAnchor(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getOpenedPage(&self) -> wxString {
        unsafe { wxString(wxHtmlWindow_GetOpenedPage(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getOpenedPageTitle(&self) -> wxString {
        unsafe { wxString(wxHtmlWindow_GetOpenedPageTitle(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getRelatedFrame(&self) -> wxFrame {
        unsafe { wxFrame(wxHtmlWindow_GetRelatedFrame(self.handle())) }
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
    fn loadPage<T: _wxString>(&self, location: T) -> bool {
        unsafe { wxHtmlWindow_LoadPage(self.handle(), location.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization<T: _wxConfigBase, U: _wxString>(&self, cfg: T, path: U) {
        unsafe { wxHtmlWindow_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.handle(), b) }
    }
    #[fixed_stack_segment]
    fn setFonts<T: _wxString, U: _wxString>(&self, normal_face: T, fixed_face: U, sizes: *c_int) {
        unsafe { wxHtmlWindow_SetFonts(self.handle(), normal_face.handle(), fixed_face.handle(), sizes) }
    }
    #[fixed_stack_segment]
    fn setPage<T: _wxString>(&self, source: T) {
        unsafe { wxHtmlWindow_SetPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedFrame<T: _wxFrame, U: _wxString>(&self, frame: T, format: U) {
        unsafe { wxHtmlWindow_SetRelatedFrame(self.handle(), frame.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.handle(), bar) }
    }
    #[fixed_stack_segment]
    fn writeCustomization<T: _wxConfigBase, U: _wxString>(&self, cfg: T, path: U) {
        unsafe { wxHtmlWindow_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxIPV4address(*u8);
impl _wxIPV4address for wxIPV4address {}
impl _wxSockAddress for wxIPV4address {}
impl _wxObject for wxIPV4address { fn handle(&self) -> *u8 { **self } }

impl wxIPV4address {
}

trait _wxIPV4address : _wxSockAddress {
}

struct wxIcon(*u8);
impl _wxIcon for wxIcon {}
impl _wxBitmap for wxIcon {}
impl _wxGDIObject for wxIcon {}
impl _wxObject for wxIcon { fn handle(&self) -> *u8 { **self } }

impl wxIcon {
    #[fixed_stack_segment]
    pub fn newDefault() -> wxIcon {
        unsafe { wxIcon(wxIcon_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newLoad<T: _wxString>(name: T, type_: c_long, width: c_int, height: c_int) -> wxIcon {
        unsafe { wxIcon(wxIcon_CreateLoad(name.handle(), type_, width, height)) }
    }
}

trait _wxIcon : _wxBitmap {
    #[fixed_stack_segment]
    fn assign(&self, other: *u8) {
        unsafe { wxIcon_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn copyFromBitmap<T: _wxBitmap>(&self, bmp: T) {
        unsafe { wxIcon_CopyFromBitmap(self.handle(), bmp.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIcon_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn fromRaw(&self, width: c_int, height: c_int) -> wxIcon {
        unsafe { wxIcon(wxIcon_FromRaw(self.handle(), width, height)) }
    }
    #[fixed_stack_segment]
    fn fromXPM(&self) -> wxIcon {
        unsafe { wxIcon(wxIcon_FromXPM(self.handle())) }
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
    fn isEqual(&self, other: &_wxIcon) -> bool {
        unsafe { wxIcon_IsEqual(self.handle(), other.handle()) }
    }
    #[fixed_stack_segment]
    fn load<T: _wxString>(&self, name: T, type_: c_long, width: c_int, height: c_int) -> c_int {
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
    #[fixed_stack_segment]
    pub fn newDefault() -> wxIconBundle {
        unsafe { wxIconBundle(wxIconBundle_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newFromFile<T: _wxString>(file: T, type_: c_int) -> wxIconBundle {
        unsafe { wxIconBundle(wxIconBundle_CreateFromFile(file.handle(), type_)) }
    }
    #[fixed_stack_segment]
    pub fn newFromIcon<T: _wxIcon>(icon: T) -> wxIconBundle {
        unsafe { wxIconBundle(wxIconBundle_CreateFromIcon(icon.handle())) }
    }
}

trait _wxIconBundle {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addIcon<T: _wxIcon>(&self, icon: T) {
        unsafe { wxIconBundle_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addIconFromFile<T: _wxString>(&self, file: T, type_: c_int) {
        unsafe { wxIconBundle_AddIconFromFile(self.handle(), file.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn assign<T: _wxIconBundle>(&self, _ref: T) {
        unsafe { wxIconBundle_Assign(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon<T: _wxIcon>(&self, w: c_int, h: c_int, _ref: T) {
        unsafe { wxIconBundle_GetIcon(self.handle(), w, h, _ref.handle()) }
    }
}

struct wxIconizeEvent(*u8);
impl _wxIconizeEvent for wxIconizeEvent {}
impl _wxEvent for wxIconizeEvent {}
impl _wxObject for wxIconizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxIconizeEvent {
}

trait _wxIconizeEvent : _wxEvent {
}

struct wxIdleEvent(*u8);
impl _wxIdleEvent for wxIdleEvent {}
impl _wxEvent for wxIdleEvent {}
impl _wxObject for wxIdleEvent { fn handle(&self) -> *u8 { **self } }

impl wxIdleEvent {
}

trait _wxIdleEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject<T: _wxObject>(&self, object_dest: T) {
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
    #[fixed_stack_segment]
    pub fn canRead<T: _wxString>(name: T) -> bool {
        unsafe { wxImage_CanRead(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxImage {
        unsafe { wxImage(wxImage_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap<T: _wxBitmap>(bitmap: T) -> wxImage {
        unsafe { wxImage(wxImage_CreateFromBitmap(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromByteString(data: *char, length: c_int, type_: c_int) -> wxImage {
        unsafe { wxImage(wxImage_CreateFromByteString(data, length, type_)) }
    }
    #[fixed_stack_segment]
    pub fn newFromLazyByteString(data: *char, length: c_int, type_: c_int) -> wxImage {
        unsafe { wxImage(wxImage_CreateFromLazyByteString(data, length, type_)) }
    }
    #[fixed_stack_segment]
    pub fn newFromData(width: c_int, height: c_int, data: *u8) -> wxImage {
        unsafe { wxImage(wxImage_CreateFromData(width, height, data)) }
    }
    #[fixed_stack_segment]
    pub fn newFromFile<T: _wxString>(name: T) -> wxImage {
        unsafe { wxImage(wxImage_CreateFromFile(name.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newSized(width: c_int, height: c_int) -> wxImage {
        unsafe { wxImage(wxImage_CreateSized(width, height)) }
    }
    #[fixed_stack_segment]
    pub fn newFromDataEx(width: c_int, height: c_int, data: *u8, isStaticData: c_int) -> wxImage {
        unsafe { wxImage(wxImage_CreateFromDataEx(width, height, data, isStaticData)) }
    }
}

trait _wxImage : _wxObject {
    #[fixed_stack_segment]
    fn convertToBitmap<T: _wxBitmap>(&self, bitmap: T) {
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
    fn getSubImage<T: _wxImage>(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: T) {
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
    fn getOption<T: _wxString>(&self, name: T) -> wxString {
        unsafe { wxString(wxImage_GetOption(self.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn getOptionInt<T: _wxString>(&self, name: T) -> bool {
        unsafe { wxImage_GetOptionInt(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn hasOption<T: _wxString>(&self, name: T) -> bool {
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
    fn loadFile<T: _wxString>(&self, name: T, type_: c_int) -> bool {
        unsafe { wxImage_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn mirror<T: _wxImage>(&self, horizontally: c_int, image: T) {
        unsafe { wxImage_Mirror(self.handle(), horizontally, image.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxImage_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paste<T: _wxImage>(&self, image: T, x: c_int, y: c_int) {
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
    fn rotate<T: _wxImage>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *u8, image: T) {
        unsafe { wxImage_Rotate(self.handle(), angle, c_x, c_y, interpolating, offset_after_rotation, image.handle()) }
    }
    #[fixed_stack_segment]
    fn rotate90<T: _wxImage>(&self, clockwise: c_int, image: T) {
        unsafe { wxImage_Rotate90(self.handle(), clockwise, image.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile<T: _wxString>(&self, name: T, type_: c_int) -> bool {
        unsafe { wxImage_SaveFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn scale<T: _wxImage>(&self, width: c_int, height: c_int, image: T) {
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
    fn setOption<T: _wxString, U: _wxString>(&self, name: T, value: U) {
        unsafe { wxImage_SetOption(self.handle(), name.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setOptionInt<T: _wxString>(&self, name: T, value: c_int) {
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
}

trait _wxImageHandler : _wxObject {
}

struct wxImageList(*u8);
impl _wxImageList for wxImageList {}
impl _wxObject for wxImageList { fn handle(&self) -> *u8 { **self } }

impl wxImageList {
    #[fixed_stack_segment]
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> wxImageList {
        unsafe { wxImageList(wxImageList_Create(width, height, mask, initialCount)) }
    }
}

trait _wxImageList : _wxObject {
    #[fixed_stack_segment]
    fn addBitmap<T: _wxBitmap, U: _wxBitmap>(&self, bitmap: T, mask: U) -> c_int {
        unsafe { wxImageList_AddBitmap(self.handle(), bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn addIcon<T: _wxIcon>(&self, icon: T) -> c_int {
        unsafe { wxImageList_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addMasked<T: _wxBitmap, U: _wxColour>(&self, bitmap: T, maskColour: U) -> c_int {
        unsafe { wxImageList_AddMasked(self.handle(), bitmap.handle(), maskColour.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImageList_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn draw<T: _wxDC>(&self, index: c_int, dc: T, x: c_int, y: c_int, flags: c_int, solidBackground: bool) -> bool {
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
    fn replace<T: _wxBitmap, U: _wxBitmap>(&self, index: c_int, bitmap: T, mask: U) -> bool {
        unsafe { wxImageList_Replace(self.handle(), index, bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceIcon<T: _wxIcon>(&self, index: c_int, icon: T) -> bool {
        unsafe { wxImageList_ReplaceIcon(self.handle(), index, icon.handle()) }
    }
}

struct wxIndividualLayoutConstraint(*u8);
impl _wxIndividualLayoutConstraint for wxIndividualLayoutConstraint {}
impl _wxObject for wxIndividualLayoutConstraint { fn handle(&self) -> *u8 { **self } }

impl wxIndividualLayoutConstraint {
}

trait _wxIndividualLayoutConstraint : _wxObject {
    #[fixed_stack_segment]
    fn above<T: _wxWindow>(&self, sibling: T, marg: c_int) {
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
    fn below<T: _wxWindow>(&self, sibling: T, marg: c_int) {
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
    fn leftOf<T: _wxWindow>(&self, sibling: T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn percentOf<T: _wxWindow>(&self, otherW: T, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.handle(), otherW.handle(), wh, per) }
    }
    #[fixed_stack_segment]
    fn resetIfWin<T: _wxWindow>(&self, otherW: T) -> bool {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.handle(), otherW.handle()) }
    }
    #[fixed_stack_segment]
    fn rightOf<T: _wxWindow>(&self, sibling: T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn sameAs<T: _wxWindow>(&self, otherW: T, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.handle(), otherW.handle(), edge, marg) }
    }
    #[fixed_stack_segment]
    fn satisfyConstraint<T: _wxWindow>(&self, constraints: *u8, win: T) -> bool {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.handle(), constraints, win.handle()) }
    }
    #[fixed_stack_segment]
    fn set<T: _wxWindow>(&self, rel: c_int, otherW: T, otherE: c_int, val: c_int, marg: c_int) {
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
}

trait _wxInitDialogEvent : _wxEvent {
}

struct wxInputStream(*u8);
impl _wxInputStream for wxInputStream {}
impl _wxStreamBase for wxInputStream { fn handle(&self) -> *u8 { **self } }

impl wxInputStream {
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
}

trait _wxJoystick : _wxObject {
}

struct wxJoystickEvent(*u8);
impl _wxJoystickEvent for wxJoystickEvent {}
impl _wxEvent for wxJoystickEvent {}
impl _wxObject for wxJoystickEvent { fn handle(&self) -> *u8 { **self } }

impl wxJoystickEvent {
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
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxJoystickEvent_GetPosition(self.handle())) }
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
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxKeyEvent_GetPosition(self.handle())) }
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
}

trait _wxLEDNumberCtrl : _wxControl {
}

struct wxLayoutAlgorithm(*u8);
impl _wxLayoutAlgorithm for wxLayoutAlgorithm {}
impl _wxObject for wxLayoutAlgorithm { fn handle(&self) -> *u8 { **self } }

impl wxLayoutAlgorithm {
    #[fixed_stack_segment]
    pub fn new() -> wxLayoutAlgorithm {
        unsafe { wxLayoutAlgorithm(wxLayoutAlgorithm_Create()) }
    }
}

trait _wxLayoutAlgorithm : _wxObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLayoutAlgorithm_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layoutFrame<T: _wxFrame>(&self, frame: T, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.handle(), frame.handle(), mainWindow) }
    }
    #[fixed_stack_segment]
    fn layoutMDIFrame<T: _wxFrame>(&self, frame: T, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.handle(), frame.handle(), x, y, w, h, use_) }
    }
    #[fixed_stack_segment]
    fn layoutWindow<T: _wxFrame>(&self, frame: T, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.handle(), frame.handle(), mainWindow) }
    }
}

struct wxLayoutConstraints(*u8);
impl _wxLayoutConstraints for wxLayoutConstraints {}
impl _wxObject for wxLayoutConstraints { fn handle(&self) -> *u8 { **self } }

impl wxLayoutConstraints {
    #[fixed_stack_segment]
    pub fn new() -> wxLayoutConstraints {
        unsafe { wxLayoutConstraints(wxLayoutConstraints_Create()) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> wxListBox {
        unsafe { wxListBox(wxListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) }
    }
}

trait _wxListBox : _wxControl {
    #[fixed_stack_segment]
    fn append<T: _wxString>(&self, item: T) {
        unsafe { wxListBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData<T: _wxString>(&self, item: T, data: *u8) {
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
    fn findString<T: _wxString>(&self, s: T) -> c_int {
        unsafe { wxListBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> wxClientData {
        unsafe { wxClientData(wxListBox_GetClientData(self.handle(), n)) }
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
    fn getString(&self, n: c_int) -> wxString {
        unsafe { wxString(wxListBox_GetString(self.handle(), n)) }
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
    fn setClientData<T: _wxClientData>(&self, n: c_int, clientData: T) {
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
    fn setString<T: _wxString>(&self, n: c_int, s: T) {
        unsafe { wxListBox_SetString(self.handle(), n, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setStringSelection<T: _wxString>(&self, str: T, sel: bool) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxListCtrl {
        unsafe { wxListCtrl(wxListCtrl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    fn findItem<T: _wxString>(&self, start: c_int, str: T, partial: bool) -> c_int {
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
    fn getColumn<T: _wxListItem>(&self, col: c_int, item: T) -> bool {
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
    fn getEditControl(&self) -> wxTextCtrl {
        unsafe { wxTextCtrl(wxListCtrl_GetEditControl(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self, which: c_int) -> wxImageList {
        unsafe { wxImageList(wxListCtrl_GetImageList(self.handle(), which)) }
    }
    #[fixed_stack_segment]
    fn getItem<T: _wxListItem>(&self, info: T) -> bool {
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
    fn getItemFont(&self, item: c_long) -> wxFont {
        unsafe { wxFont(wxListCtrl_GetItemFont(self.handle(), item)) }
    }
    #[fixed_stack_segment]
    fn getItemPosition(&self, item: c_int) -> wxPoint {
        unsafe { wxPoint(wxListCtrl_GetItemPosition(self.handle(), item)) }
    }
    #[fixed_stack_segment]
    fn getItemRect(&self, item: c_int, code: c_int) -> wxRect {
        unsafe { wxRect(wxListCtrl_GetItemRect(self.handle(), item, code)) }
    }
    #[fixed_stack_segment]
    fn getItemSpacing(&self, isSmall: bool) -> wxSize {
        unsafe { wxSize(wxListCtrl_GetItemSpacing(self.handle(), isSmall)) }
    }
    #[fixed_stack_segment]
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.handle(), item, stateMask) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: c_int) -> wxString {
        unsafe { wxString(wxListCtrl_GetItemText(self.handle(), item)) }
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
    fn getTextColour<T: _wxColour>(&self, _ref: T) {
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
    fn insertColumn<T: _wxString>(&self, col: c_int, heading: T, format: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_InsertColumn(self.handle(), col, heading.handle(), format, width) }
    }
    #[fixed_stack_segment]
    fn insertColumnFromInfo<T: _wxListItem>(&self, col: c_int, info: T) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.handle(), col, info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem<T: _wxListItem>(&self, info: T) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithData<T: _wxString>(&self, index: c_int, label: T) -> c_int {
        unsafe { wxListCtrl_InsertItemWithData(self.handle(), index, label.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.handle(), index, imageIndex) }
    }
    #[fixed_stack_segment]
    fn insertItemWithLabel<T: _wxString>(&self, index: c_int, label: T, imageIndex: c_int) -> c_int {
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
    fn setBackgroundColour<T: _wxColour>(&self, col: T) {
        unsafe { wxListCtrl_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumn<T: _wxListItem>(&self, col: c_int, item: T) -> bool {
        unsafe { wxListCtrl_SetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> bool {
        unsafe { wxListCtrl_SetColumnWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour<T: _wxColour>(&self, col: T) -> c_int {
        unsafe { wxListCtrl_SetForegroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList<T: _wxImageList>(&self, imageList: T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.handle(), imageList.handle(), which) }
    }
    #[fixed_stack_segment]
    fn setItem<T: _wxString>(&self, index: c_int, col: c_int, label: T, imageId: c_int) -> bool {
        unsafe { wxListCtrl_SetItem(self.handle(), index, col, label.handle(), imageId) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: c_int, data: c_int) -> bool {
        unsafe { wxListCtrl_SetItemData(self.handle(), item, data) }
    }
    #[fixed_stack_segment]
    fn setItemFromInfo<T: _wxListItem>(&self, info: T) -> bool {
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
    fn setItemText<T: _wxString>(&self, item: c_int, str: T) {
        unsafe { wxListCtrl_SetItemText(self.handle(), item, str.handle()) }
    }
    #[fixed_stack_segment]
    fn setSingleStyle(&self, style: c_int, add: bool) {
        unsafe { wxListCtrl_SetSingleStyle(self.handle(), style, add) }
    }
    #[fixed_stack_segment]
    fn setTextColour<T: _wxColour>(&self, col: T) {
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
    fn assignImageList<T: _wxImageList>(&self, images: T, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.handle(), images.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getColumn2<T: _wxListItem>(&self, col: c_int, item: T) {
        unsafe { wxListCtrl_GetColumn2(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem2<T: _wxListItem>(&self, info: T) {
        unsafe { wxListCtrl_GetItem2(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemPosition2(&self, item: c_int) -> wxPoint {
        unsafe { wxPoint(wxListCtrl_GetItemPosition2(self.handle(), item)) }
    }
    #[fixed_stack_segment]
    fn sortItems2<T: _wxClosure>(&self, closure: T) -> bool {
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
    fn getItem<T: _wxListItem>(&self, _ref: T) {
        unsafe { wxListEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> wxString {
        unsafe { wxString(wxListEvent_GetLabel(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> wxPoint {
        unsafe { wxPoint(wxListEvent_GetPoint(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> wxString {
        unsafe { wxString(wxListEvent_GetText(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new() -> wxListItem {
        unsafe { wxListItem(wxListItem_Create()) }
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
    fn getBackgroundColour<T: _wxColour>(&self, _ref: T) {
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
    fn getFont<T: _wxFont>(&self, _ref: T) {
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
    fn getText(&self) -> wxString {
        unsafe { wxString(wxListItem_GetText(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTextColour<T: _wxColour>(&self, _ref: T) {
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
    fn setBackgroundColour<T: _wxColour>(&self, colBack: T) {
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
    fn setFont<T: _wxFont>(&self, font: T) {
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
    fn setText<T: _wxString>(&self, text: T) {
        unsafe { wxListItem_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour<T: _wxColour>(&self, colText: T) {
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
    #[fixed_stack_segment]
    pub fn new(_name: c_int, _flags: c_int) -> wxLocale {
        unsafe { wxLocale(wxLocale_Create(_name, _flags)) }
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
    fn getLocale(&self) -> wxLocale {
        unsafe { wxLocale(wxLocale_GetLocale(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> wxString {
        unsafe { wxString(wxLocale_GetName(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn getActiveTarget() -> wxLog {
        unsafe { wxLog(wxLog_GetActiveTarget()) }
    }
}

trait _wxLog {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addTraceMask<T: _wxString>(&self, str: T) {
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
    fn isAllowedTraceMask<T: _wxMask>(&self, mask: T) -> bool {
        unsafe { wxLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: c_int, szString: *wchar_t, t: c_int) {
        unsafe { wxLog_OnLog(self.handle(), level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask<T: _wxString>(&self, str: T) {
        unsafe { wxLog_RemoveTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxLog_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) -> wxLog {
        unsafe { wxLog(wxLog_SetActiveTarget(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxLog>(logger: T) -> wxLogChain {
        unsafe { wxLogChain(wxLogChain_Create(logger.handle())) }
    }
}

trait _wxLogChain : _wxLog {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLogChain_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOldLog(&self) -> wxLog {
        unsafe { wxLog(wxLogChain_GetOldLog(self.handle())) }
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
    fn setLog<T: _wxLog>(&self, logger: T) {
        unsafe { wxLogChain_SetLog(self.handle(), logger.handle()) }
    }
}

struct wxLogGUI(*u8);
impl _wxLogGUI for wxLogGUI {}
impl _wxLog for wxLogGUI { fn handle(&self) -> *u8 { **self } }

impl wxLogGUI {
}

trait _wxLogGUI : _wxLog {
}

struct wxLogNull(*u8);
impl _wxLogNull for wxLogNull {}
impl _wxLog for wxLogNull { fn handle(&self) -> *u8 { **self } }

impl wxLogNull {
    #[fixed_stack_segment]
    pub fn new() -> wxLogNull {
        unsafe { wxLogNull(wxLogNull_Create()) }
    }
}

trait _wxLogNull : _wxLog {
}

struct wxLogPassThrough(*u8);
impl _wxLogPassThrough for wxLogPassThrough {}
impl _wxLogChain for wxLogPassThrough {}
impl _wxLog for wxLogPassThrough { fn handle(&self) -> *u8 { **self } }

impl wxLogPassThrough {
}

trait _wxLogPassThrough : _wxLogChain {
}

struct wxLogStderr(*u8);
impl _wxLogStderr for wxLogStderr {}
impl _wxLog for wxLogStderr { fn handle(&self) -> *u8 { **self } }

impl wxLogStderr {
    #[fixed_stack_segment]
    pub fn new() -> wxLogStderr {
        unsafe { wxLogStderr(wxLogStderr_Create()) }
    }
    #[fixed_stack_segment]
    pub fn newStdOut() -> wxLogStderr {
        unsafe { wxLogStderr(wxLogStderr_CreateStdOut()) }
    }
}

trait _wxLogStderr : _wxLog {
}

struct wxLogStream(*u8);
impl _wxLogStream for wxLogStream {}
impl _wxLog for wxLogStream { fn handle(&self) -> *u8 { **self } }

impl wxLogStream {
}

trait _wxLogStream : _wxLog {
}

struct wxLogTextCtrl(*u8);
impl _wxLogTextCtrl for wxLogTextCtrl {}
impl _wxLog for wxLogTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxLogTextCtrl {
    #[fixed_stack_segment]
    pub fn new<T: _wxTextCtrl>(text: T) -> wxLogTextCtrl {
        unsafe { wxLogTextCtrl(wxLogTextCtrl_Create(text.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(parent: T, title: *wchar_t, showit: bool, passthrough: bool) -> wxLogWindow {
        unsafe { wxLogWindow(wxLogWindow_Create(parent.handle(), title, showit, passthrough)) }
    }
}

trait _wxLogWindow : _wxLogPassThrough {
    #[fixed_stack_segment]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame(wxLogWindow_GetFrame(self.handle())) }
    }
}

struct wxLongLong(*u8);
impl _wxLongLong for wxLongLong { fn handle(&self) -> *u8 { **self } }

impl wxLongLong {
}

trait _wxLongLong {
    fn handle(&self) -> *u8;
    
}

struct wxMBConv(*u8);
impl _wxMBConv for wxMBConv { fn handle(&self) -> *u8 { **self } }

impl wxMBConv {
}

trait _wxMBConv {
    fn handle(&self) -> *u8;
    
}

struct wxMBConvFile(*u8);
impl _wxMBConvFile for wxMBConvFile {}
impl _wxMBConv for wxMBConvFile { fn handle(&self) -> *u8 { **self } }

impl wxMBConvFile {
}

trait _wxMBConvFile : _wxMBConv {
}

struct wxMBConvUTF7(*u8);
impl _wxMBConvUTF7 for wxMBConvUTF7 {}
impl _wxMBConv for wxMBConvUTF7 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF7 {
}

trait _wxMBConvUTF7 : _wxMBConv {
}

struct wxMBConvUTF8(*u8);
impl _wxMBConvUTF8 for wxMBConvUTF8 {}
impl _wxMBConv for wxMBConvUTF8 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF8 {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxMDIChildFrame {
        unsafe { wxMDIChildFrame(wxMDIChildFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxMDIParentFrame {
        unsafe { wxMDIParentFrame(wxMDIParentFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    fn getActiveChild(&self) -> wxMDIChildFrame {
        unsafe { wxMDIChildFrame(wxMDIParentFrame_GetActiveChild(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getClientWindow(&self) -> wxMDIClientWindow {
        unsafe { wxMDIClientWindow(wxMDIParentFrame_GetClientWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getWindowMenu(&self) -> wxMenu {
        unsafe { wxMenu(wxMDIParentFrame_GetWindowMenu(self.handle())) }
    }
    #[fixed_stack_segment]
    fn onCreateClient(&self) -> wxMDIClientWindow {
        unsafe { wxMDIClientWindow(wxMDIParentFrame_OnCreateClient(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setWindowMenu<T: _wxMenu>(&self, menu: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxBitmap>(bitmap: T) -> wxMask {
        unsafe { wxMask(wxMask_Create(bitmap.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newColoured<T: _wxBitmap, U: _wxColour>(bitmap: T, colour: U) -> *u8 {
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
}

trait _wxMaximizeEvent : _wxEvent {
}

struct wxMemoryDC(*u8);
impl _wxMemoryDC for wxMemoryDC {}
impl _wxDC for wxMemoryDC {}
impl _wxObject for wxMemoryDC { fn handle(&self) -> *u8 { **self } }

impl wxMemoryDC {
    #[fixed_stack_segment]
    pub fn new() -> wxMemoryDC {
        unsafe { wxMemoryDC(wxMemoryDC_Create()) }
    }
    #[fixed_stack_segment]
    pub fn newCompatible<T: _wxDC>(dc: T) -> wxMemoryDC {
        unsafe { wxMemoryDC(wxMemoryDC_CreateCompatible(dc.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newWithBitmap<T: _wxBitmap>(bitmap: T) -> wxMemoryDC {
        unsafe { wxMemoryDC(wxMemoryDC_CreateWithBitmap(bitmap.handle())) }
    }
}

trait _wxMemoryDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMemoryDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selectObject<T: _wxBitmap>(&self, bitmap: T) {
        unsafe { wxMemoryDC_SelectObject(self.handle(), bitmap.handle()) }
    }
}

struct wxMemoryFSHandler(*u8);
impl _wxMemoryFSHandler for wxMemoryFSHandler {}
impl _wxFileSystemHandler for wxMemoryFSHandler {}
impl _wxObject for wxMemoryFSHandler { fn handle(&self) -> *u8 { **self } }

impl wxMemoryFSHandler {
}

trait _wxMemoryFSHandler : _wxFileSystemHandler {
}

struct wxMemoryInputStream(*u8);
impl _wxMemoryInputStream for wxMemoryInputStream {}
impl _wxInputStream for wxMemoryInputStream {}
impl _wxStreamBase for wxMemoryInputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryInputStream {
}

trait _wxMemoryInputStream : _wxInputStream {
}

struct wxMemoryOutputStream(*u8);
impl _wxMemoryOutputStream for wxMemoryOutputStream {}
impl _wxOutputStream for wxMemoryOutputStream {}
impl _wxStreamBase for wxMemoryOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryOutputStream {
}

trait _wxMemoryOutputStream : _wxOutputStream {
}

struct wxMenu(*u8);
impl _wxMenu for wxMenu {}
impl _wxEvtHandler for wxMenu {}
impl _wxObject for wxMenu { fn handle(&self) -> *u8 { **self } }

impl wxMenu {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(title: T, style: c_long) -> wxMenu {
        unsafe { wxMenu(wxMenu_Create(title.handle(), style)) }
    }
}

trait _wxMenu : _wxEvtHandler {
    #[fixed_stack_segment]
    fn append<T: _wxString, U: _wxString>(&self, id: c_int, text: T, help: U, isCheckable: bool) {
        unsafe { wxMenu_Append(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn appendItem<T: _wxMenuItem>(&self, _itm: T) {
        unsafe { wxMenu_AppendItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSub<T: _wxString, U: _wxMenu, V: _wxString>(&self, id: c_int, text: T, submenu: U, help: V) {
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
    fn deleteByItem<T: _wxMenuItem>(&self, _itm: T) {
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
    fn destroyByItem<T: _wxMenuItem>(&self, _itm: T) {
        unsafe { wxMenu_DestroyByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { wxMenu_Enable(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: c_int) -> wxMenuItem {
        unsafe { wxMenuItem(wxMenu_FindItem(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn findItemByLabel<T: _wxString>(&self, itemString: T) -> c_int {
        unsafe { wxMenu_FindItemByLabel(self.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> wxClientData {
        unsafe { wxClientData(wxMenu_GetClientData(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> wxString {
        unsafe { wxString(wxMenu_GetHelpString(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getInvokingWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxMenu_GetInvokingWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> wxString {
        unsafe { wxString(wxMenu_GetLabel(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMenuItems<T: _wxList>(&self, _lst: T) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> wxMenu {
        unsafe { wxMenu(wxMenu_GetParent(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> wxString {
        unsafe { wxString(wxMenu_GetTitle(self.handle())) }
    }
    #[fixed_stack_segment]
    fn insert<T: _wxString, U: _wxString>(&self, pos: size_t, id: c_int, text: T, help: U, isCheckable: bool) {
        unsafe { wxMenu_Insert(self.handle(), pos, id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn insertItem<T: _wxMenuItem>(&self, pos: size_t, _itm: T) {
        unsafe { wxMenu_InsertItem(self.handle(), pos, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSub<T: _wxString, U: _wxMenu, V: _wxString>(&self, pos: size_t, id: c_int, text: T, submenu: U, help: V) {
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
    fn prepend<T: _wxString, U: _wxString>(&self, id: c_int, text: T, help: U, isCheckable: bool) {
        unsafe { wxMenu_Prepend(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn prependItem<T: _wxMenuItem>(&self, _itm: T) {
        unsafe { wxMenu_PrependItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn prependSub<T: _wxString, U: _wxMenu, V: _wxString>(&self, id: c_int, text: T, submenu: U, help: V) {
        unsafe { wxMenu_PrependSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    fn removeById<T: _wxMenuItem>(&self, id: c_int, _itm: T) {
        unsafe { wxMenu_RemoveById(self.handle(), id, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn removeByItem(&self, item: *u8) {
        unsafe { wxMenu_RemoveByItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn setClientData<T: _wxClientData>(&self, clientData: T) {
        unsafe { wxMenu_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setEventHandler<T: _wxEvtHandler>(&self, handler: T) {
        unsafe { wxMenu_SetEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setHelpString<T: _wxString>(&self, id: c_int, helpString: T) {
        unsafe { wxMenu_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setInvokingWindow<T: _wxWindow>(&self, win: T) {
        unsafe { wxMenu_SetInvokingWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel<T: _wxString>(&self, id: c_int, label: T) {
        unsafe { wxMenu_SetLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setParent<T: _wxWindow>(&self, parent: T) {
        unsafe { wxMenu_SetParent(self.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitle<T: _wxString>(&self, title: T) {
        unsafe { wxMenu_SetTitle(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn updateUI(&self, source: *u8) {
        unsafe { wxMenu_UpdateUI(self.handle(), source) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> wxMenuBar {
        unsafe { wxMenuBar(wxMenu_GetMenuBar(self.handle())) }
    }
    #[fixed_stack_segment]
    fn appendRadioItem<T: _wxString, U: _wxString>(&self, id: c_int, text: T, help: U) {
        unsafe { wxMenu_AppendRadioItem(self.handle(), id, text.handle(), help.handle()) }
    }
}

struct wxMenuBar(*u8);
impl _wxMenuBar for wxMenuBar {}
impl _wxEvtHandler for wxMenuBar {}
impl _wxObject for wxMenuBar { fn handle(&self) -> *u8 { **self } }

impl wxMenuBar {
    #[fixed_stack_segment]
    pub fn new(_style: c_int) -> wxMenuBar {
        unsafe { wxMenuBar(wxMenuBar_Create(_style)) }
    }
}

trait _wxMenuBar : _wxEvtHandler {
    #[fixed_stack_segment]
    fn append<T: _wxMenu, U: _wxString>(&self, menu: T, title: U) -> c_int {
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
    fn findItem(&self, id: c_int) -> wxMenuItem {
        unsafe { wxMenuItem(wxMenuBar_FindItem(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn findMenu<T: _wxString>(&self, title: T) -> c_int {
        unsafe { wxMenuBar_FindMenu(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn findMenuItem<T: _wxString, U: _wxString>(&self, menuString: T, itemString: U) -> c_int {
        unsafe { wxMenuBar_FindMenuItem(self.handle(), menuString.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> wxString {
        unsafe { wxString(wxMenuBar_GetHelpString(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> wxString {
        unsafe { wxString(wxMenuBar_GetLabel(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getLabelTop(&self, pos: c_int) -> wxString {
        unsafe { wxString(wxMenuBar_GetLabelTop(self.handle(), pos)) }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, pos: c_int) -> wxMenu {
        unsafe { wxMenu(wxMenuBar_GetMenu(self.handle(), pos)) }
    }
    #[fixed_stack_segment]
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insert<T: _wxMenu, U: _wxString>(&self, pos: c_int, menu: T, title: U) -> c_int {
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
    fn remove(&self, pos: c_int) -> wxMenu {
        unsafe { wxMenu(wxMenuBar_Remove(self.handle(), pos)) }
    }
    #[fixed_stack_segment]
    fn replace<T: _wxMenu, U: _wxString>(&self, pos: c_int, menu: T, title: U) -> wxMenu {
        unsafe { wxMenu(wxMenuBar_Replace(self.handle(), pos, menu.handle(), title.handle())) }
    }
    #[fixed_stack_segment]
    fn setHelpString<T: _wxString>(&self, id: c_int, helpString: T) {
        unsafe { wxMenuBar_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel<T: _wxString>(&self, id: c_int, label: T) {
        unsafe { wxMenuBar_SetItemLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel<T: _wxString>(&self, s: T) {
        unsafe { wxMenuBar_SetLabel(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTop<T: _wxString>(&self, pos: c_int, label: T) {
        unsafe { wxMenuBar_SetLabelTop(self.handle(), pos, label.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame(wxMenuBar_GetFrame(self.handle())) }
    }
}

struct wxMenuEvent(*u8);
impl _wxMenuEvent for wxMenuEvent {}
impl _wxEvent for wxMenuEvent {}
impl _wxObject for wxMenuEvent { fn handle(&self) -> *u8 { **self } }

impl wxMenuEvent {
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
    #[fixed_stack_segment]
    pub fn new() -> wxMenuItem {
        unsafe { wxMenuItem(wxMenuItem_Create()) }
    }
    #[fixed_stack_segment]
    pub fn getLabelFromText(text: *wchar_t) -> wxString {
        unsafe { wxString(wxMenuItem_GetLabelFromText(text)) }
    }
    #[fixed_stack_segment]
    pub fn newSeparator() -> wxMenuItem {
        unsafe { wxMenuItem(wxMenuItem_CreateSeparator()) }
    }
    #[fixed_stack_segment]
    pub fn newEx<T: _wxString, U: _wxString, V: _wxMenu>(id: c_int, label: T, help: U, itemkind: c_int, submenu: V) -> wxMenuItem {
        unsafe { wxMenuItem(wxMenuItem_CreateEx(id, label.handle(), help.handle(), itemkind, submenu.handle())) }
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
    fn getHelp(&self) -> wxString {
        unsafe { wxString(wxMenuItem_GetHelp(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> wxString {
        unsafe { wxString(wxMenuItem_GetLabel(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getMenu(&self) -> wxMenu {
        unsafe { wxMenu(wxMenuItem_GetMenu(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSubMenu(&self) -> wxMenu {
        unsafe { wxMenu(wxMenuItem_GetSubMenu(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> wxString {
        unsafe { wxString(wxMenuItem_GetText(self.handle())) }
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
    fn setHelp<T: _wxString>(&self, str: T) {
        unsafe { wxMenuItem_SetHelp(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setSubMenu<T: _wxMenu>(&self, menu: T) {
        unsafe { wxMenuItem_SetSubMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn setText<T: _wxString>(&self, str: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString, V: _wxString>(_prt: T, _msg: U, _cap: V, _stl: c_int) -> wxMessageDialog {
        unsafe { wxMessageDialog(wxMessageDialog_Create(_prt.handle(), _msg.handle(), _cap.handle(), _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(_file: T) -> wxMetafile {
        unsafe { wxMetafile(wxMetafile_Create(_file.handle())) }
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
    fn play<T: _wxDC>(&self, _dc: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(_file: T) -> wxMetafileDC {
        unsafe { wxMetafileDC(wxMetafileDC_Create(_file.handle())) }
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
    #[fixed_stack_segment]
    pub fn new() -> wxMimeTypesManager {
        unsafe { wxMimeTypesManager(wxMimeTypesManager_Create()) }
    }
}

trait _wxMimeTypesManager {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addFallbacks(&self, _types: *u8) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.handle(), _types) }
    }
    #[fixed_stack_segment]
    fn enumAllFileTypes<T: _wxList>(&self, _lst: T) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromExtension<T: _wxString>(&self, _ext: T) -> wxFileType {
        unsafe { wxFileType(wxMimeTypesManager_GetFileTypeFromExtension(self.handle(), _ext.handle())) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromMimeType<T: _wxString>(&self, _name: T) -> wxFileType {
        unsafe { wxFileType(wxMimeTypesManager_GetFileTypeFromMimeType(self.handle(), _name.handle())) }
    }
    #[fixed_stack_segment]
    fn isOfType<T: _wxString, U: _wxString>(&self, _type: T, _wildcard: U) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxMiniFrame {
        unsafe { wxMiniFrame(wxMiniFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxMiniFrame : _wxFrame {
}

struct wxMirrorDC(*u8);
impl _wxMirrorDC for wxMirrorDC {}
impl _wxDC for wxMirrorDC {}
impl _wxObject for wxMirrorDC { fn handle(&self) -> *u8 { **self } }

impl wxMirrorDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxDC>(dc: T) -> wxMirrorDC {
        unsafe { wxMirrorDC(wxMirrorDC_Create(dc.handle())) }
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
}

trait _wxModule : _wxObject {
}

struct wxMouseCaptureChangedEvent(*u8);
impl _wxMouseCaptureChangedEvent for wxMouseCaptureChangedEvent {}
impl _wxEvent for wxMouseCaptureChangedEvent {}
impl _wxObject for wxMouseCaptureChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseCaptureChangedEvent {
}

trait _wxMouseCaptureChangedEvent : _wxEvent {
}

struct wxMouseEvent(*u8);
impl _wxMouseEvent for wxMouseEvent {}
impl _wxEvent for wxMouseEvent {}
impl _wxObject for wxMouseEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseEvent {
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
    fn getLogicalPosition<T: _wxDC>(&self, dc: T) -> wxPoint {
        unsafe { wxPoint(wxMouseEvent_GetLogicalPosition(self.handle(), dc.handle())) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxMouseEvent_GetPosition(self.handle())) }
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
}

trait _wxMoveEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxMoveEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxMoveEvent_GetPosition(self.handle())) }
    }
}

struct wxMultiCellCanvas(*u8);
impl _wxMultiCellCanvas for wxMultiCellCanvas {}
impl _wxFlexGridSizer for wxMultiCellCanvas {}
impl _wxGridSizer for wxMultiCellCanvas {}
impl _wxSizer for wxMultiCellCanvas {}
impl _wxObject for wxMultiCellCanvas { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellCanvas {
}

trait _wxMultiCellCanvas : _wxFlexGridSizer {
}

struct wxMultiCellItemHandle(*u8);
impl _wxMultiCellItemHandle for wxMultiCellItemHandle {}
impl _wxObject for wxMultiCellItemHandle { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellItemHandle {
}

trait _wxMultiCellItemHandle : _wxObject {
}

struct wxMultiCellSizer(*u8);
impl _wxMultiCellSizer for wxMultiCellSizer {}
impl _wxSizer for wxMultiCellSizer {}
impl _wxObject for wxMultiCellSizer { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellSizer {
}

trait _wxMultiCellSizer : _wxSizer {
}

struct wxMutex(*u8);
impl _wxMutex for wxMutex { fn handle(&self) -> *u8 { **self } }

impl wxMutex {
}

trait _wxMutex {
    fn handle(&self) -> *u8;
    
}

struct wxMutexLocker(*u8);
impl _wxMutexLocker for wxMutexLocker { fn handle(&self) -> *u8 { **self } }

impl wxMutexLocker {
}

trait _wxMutexLocker {
    fn handle(&self) -> *u8;
    
}

struct wxNavigationKeyEvent(*u8);
impl _wxNavigationKeyEvent for wxNavigationKeyEvent {}
impl _wxEvent for wxNavigationKeyEvent {}
impl _wxObject for wxNavigationKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNavigationKeyEvent {
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
    fn setCurrentFocus<T: _wxWindow>(&self, win: T) {
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
}

trait _wxNewBitmapButton : _wxPanel {
}

struct wxNodeBase(*u8);
impl _wxNodeBase for wxNodeBase { fn handle(&self) -> *u8 { **self } }

impl wxNodeBase {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxNotebook {
        unsafe { wxNotebook(wxNotebook_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxNotebook : _wxControl {
    #[fixed_stack_segment]
    fn addPage<T: _wxWindow, U: _wxString>(&self, pPage: T, strText: U, bSelect: bool, imageId: c_int) -> bool {
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
    fn getImageList(&self) -> wxImageList {
        unsafe { wxImageList(wxNotebook_GetImageList(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPage(&self, nPage: c_int) -> wxWindow {
        unsafe { wxWindow(wxNotebook_GetPage(self.handle(), nPage)) }
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
    fn getPageText(&self, nPage: c_int) -> wxString {
        unsafe { wxString(wxNotebook_GetPageText(self.handle(), nPage)) }
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
    fn insertPage<T: _wxWindow, U: _wxString>(&self, nPage: c_int, pPage: T, strText: U, bSelect: bool, imageId: c_int) -> bool {
        unsafe { wxNotebook_InsertPage(self.handle(), nPage, pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn removePage(&self, nPage: c_int) -> bool {
        unsafe { wxNotebook_RemovePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn setImageList<T: _wxImageList>(&self, imageList: T) {
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
    fn setPageText<T: _wxString>(&self, nPage: c_int, strText: T) -> bool {
        unsafe { wxNotebook_SetPageText(self.handle(), nPage, strText.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn assignImageList<T: _wxImageList>(&self, imageList: T) {
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
}

trait _wxNotebookEvent : _wxNotifyEvent {
}

struct wxNotifyEvent(*u8);
impl _wxNotifyEvent for wxNotifyEvent {}
impl _wxCommandEvent for wxNotifyEvent {}
impl _wxEvent for wxNotifyEvent {}
impl _wxObject for wxNotifyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNotifyEvent {
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
}

trait _wxObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> wxClosure {
        unsafe { wxClosure(wxObject_GetClientClosure(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setClientClosure<T: _wxClosure>(&self, closure: T) {
        unsafe { wxObject_SetClientClosure(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxObject_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClassInfo(&self) -> wxClassInfo {
        unsafe { wxClassInfo(wxObject_GetClassInfo(self.handle())) }
    }
    #[fixed_stack_segment]
    fn isKindOf<T: _wxClassInfo>(&self, classInfo: T) -> bool {
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
}

trait _wxObjectRefData {
    fn handle(&self) -> *u8;
    
}

struct wxOutputStream(*u8);
impl _wxOutputStream for wxOutputStream {}
impl _wxStreamBase for wxOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxOutputStream {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxPageSetupDialogData>(parent: T, data: U) -> wxPageSetupDialog {
        unsafe { wxPageSetupDialog(wxPageSetupDialog_Create(parent.handle(), data.handle())) }
    }
}

trait _wxPageSetupDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getPageSetupData<T: _wxPageSetupDialogData>(&self, _ref: T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.handle(), _ref.handle()) }
    }
}

struct wxPageSetupDialogData(*u8);
impl _wxPageSetupDialogData for wxPageSetupDialogData {}
impl _wxObject for wxPageSetupDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPageSetupDialogData {
    #[fixed_stack_segment]
    pub fn new() -> wxPageSetupDialogData {
        unsafe { wxPageSetupDialogData(wxPageSetupDialogData_Create()) }
    }
    #[fixed_stack_segment]
    pub fn newFromData<T: _wxPrintData>(printData: T) -> wxPageSetupDialogData {
        unsafe { wxPageSetupDialogData(wxPageSetupDialogData_CreateFromData(printData.handle())) }
    }
}

trait _wxPageSetupDialogData : _wxObject {
    #[fixed_stack_segment]
    fn assign<T: _wxPageSetupDialogData>(&self, data: T) {
        unsafe { wxPageSetupDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData<T: _wxPrintData>(&self, printData: T) {
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
    fn getMarginBottomRight(&self) -> wxPoint {
        unsafe { wxPoint(wxPageSetupDialogData_GetMarginBottomRight(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getMarginTopLeft(&self) -> wxPoint {
        unsafe { wxPoint(wxPageSetupDialogData_GetMarginTopLeft(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getMinMarginBottomRight(&self) -> wxPoint {
        unsafe { wxPoint(wxPageSetupDialogData_GetMinMarginBottomRight(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getMinMarginTopLeft(&self) -> wxPoint {
        unsafe { wxPoint(wxPageSetupDialogData_GetMinMarginTopLeft(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> wxSize {
        unsafe { wxSize(wxPageSetupDialogData_GetPaperSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrintData<T: _wxPrintData>(&self, _ref: T) {
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
    fn setPrintData<T: _wxPrintData>(&self, printData: T) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
}

struct wxPaintDC(*u8);
impl _wxPaintDC for wxPaintDC {}
impl _wxWindowDC for wxPaintDC {}
impl _wxDC for wxPaintDC {}
impl _wxObject for wxPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxPaintDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(win: T) -> wxPaintDC {
        unsafe { wxPaintDC(wxPaintDC_Create(win.handle())) }
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
}

trait _wxPaintEvent : _wxEvent {
}

struct wxPalette(*u8);
impl _wxPalette for wxPalette {}
impl _wxGDIObject for wxPalette {}
impl _wxObject for wxPalette { fn handle(&self) -> *u8 { **self } }

impl wxPalette {
    #[fixed_stack_segment]
    pub fn newDefault() -> wxPalette {
        unsafe { wxPalette(wxPalette_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newRGB(n: c_int, red: *u8, green: *u8, blue: *u8) -> wxPalette {
        unsafe { wxPalette(wxPalette_CreateRGB(n, red, green, blue)) }
    }
}

trait _wxPalette : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign<T: _wxPalette>(&self, palette: T) {
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
    fn isEqual<T: _wxPalette>(&self, palette: T) -> bool {
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
    fn setChangedWindow<T: _wxWindow>(&self, win: T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.handle(), win.handle()) }
    }
}

struct wxPanel(*u8);
impl _wxPanel for wxPanel {}
impl _wxWindow for wxPanel {}
impl _wxEvtHandler for wxPanel {}
impl _wxObject for wxPanel { fn handle(&self) -> *u8 { **self } }

impl wxPanel {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxPanel {
        unsafe { wxPanel(wxPanel_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
}

trait _wxPathList : _wxList {
}

struct wxPen(*u8);
impl _wxPen for wxPen {}
impl _wxGDIObject for wxPen {}
impl _wxObject for wxPen { fn handle(&self) -> *u8 { **self } }

impl wxPen {
    #[fixed_stack_segment]
    pub fn newDefault() -> wxPen {
        unsafe { wxPen(wxPen_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap<T: _wxBitmap>(stipple: T, width: c_int) -> wxPen {
        unsafe { wxPen(wxPen_CreateFromBitmap(stipple.handle(), width)) }
    }
    #[fixed_stack_segment]
    pub fn newFromColour<T: _wxColour>(col: T, width: c_int, style: c_int) -> wxPen {
        unsafe { wxPen(wxPen_CreateFromColour(col.handle(), width, style)) }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> wxPen {
        unsafe { wxPen(wxPen_CreateFromStock(id)) }
    }
}

trait _wxPen : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign<T: _wxPen>(&self, pen: T) {
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
    fn getColour<T: _wxColour>(&self, _ref: T) {
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
    fn getStipple<T: _wxBitmap>(&self, _ref: T) {
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
    fn isEqual<T: _wxPen>(&self, pen: T) -> bool {
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
    fn setColour<T: _wxColour>(&self, col: T) {
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
    fn setStipple<T: _wxBitmap>(&self, stipple: T) {
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
}

trait _wxPenList : _wxList {
}

struct wxPlotCurve(*u8);
impl _wxPlotCurve for wxPlotCurve {}
impl _wxObject for wxPlotCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotCurve {
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
}

trait _wxPlotEvent : _wxNotifyEvent {
}

struct wxPlotOnOffCurve(*u8);
impl _wxPlotOnOffCurve for wxPlotOnOffCurve {}
impl _wxObject for wxPlotOnOffCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotOnOffCurve {
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
}

trait _wxPlotWindow : _wxScrolledWindow {
}

struct wxPoint(*u8);
impl _wxPoint for wxPoint { fn handle(&self) -> *u8 { **self } }

impl wxPoint {
    #[fixed_stack_segment]
    pub fn new(xx: c_int, yy: c_int) -> wxPoint {
        unsafe { wxPoint(wxPoint_Create(xx, yy)) }
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
}

trait _wxPopupTransientWindow : _wxPopupWindow {
}

struct wxPopupWindow(*u8);
impl _wxPopupWindow for wxPopupWindow {}
impl _wxWindow for wxPopupWindow {}
impl _wxEvtHandler for wxPopupWindow {}
impl _wxObject for wxPopupWindow { fn handle(&self) -> *u8 { **self } }

impl wxPopupWindow {
}

trait _wxPopupWindow : _wxWindow {
}

struct wxPostScriptDC(*u8);
impl _wxPostScriptDC for wxPostScriptDC {}
impl _wxDC for wxPostScriptDC {}
impl _wxObject for wxPostScriptDC { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxPrintData>(data: T) -> wxPostScriptDC {
        unsafe { wxPostScriptDC(wxPostScriptDC_Create(data.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxPrintPreview, U: _wxWindow>(preview: T, parent: U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> wxPreviewCanvas {
        unsafe { wxPreviewCanvas(wxPreviewCanvas_Create(preview.handle(), parent.handle(), x, y, w, h, style)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxPrintPreview, U: _wxFrame, V: _wxString, W: _wxString>(preview: T, parent: U, title: V, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: W) -> wxPreviewFrame {
        unsafe { wxPreviewFrame(wxPreviewFrame_Create(preview.handle(), parent.handle(), title.handle(), x, y, width, height, style, name.handle())) }
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
    #[fixed_stack_segment]
    pub fn new() -> wxPrintData {
        unsafe { wxPrintData(wxPrintData_Create()) }
    }
}

trait _wxPrintData : _wxObject {
    #[fixed_stack_segment]
    fn assign<T: _wxPrintData>(&self, data: T) {
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
    fn getFilename(&self) -> wxString {
        unsafe { wxString(wxPrintData_GetFilename(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFontMetricPath(&self) -> wxString {
        unsafe { wxString(wxPrintData_GetFontMetricPath(self.handle())) }
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
    fn getPaperSize(&self) -> wxSize {
        unsafe { wxSize(wxPrintData_GetPaperSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPreviewCommand(&self) -> wxString {
        unsafe { wxString(wxPrintData_GetPreviewCommand(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterCommand(&self) -> wxString {
        unsafe { wxString(wxPrintData_GetPrinterCommand(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrinterName(&self) -> wxString {
        unsafe { wxString(wxPrintData_GetPrinterName(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrinterOptions(&self) -> wxString {
        unsafe { wxString(wxPrintData_GetPrinterOptions(self.handle())) }
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
    fn setFilename<T: _wxString>(&self, filename: T) {
        unsafe { wxPrintData_SetFilename(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn setFontMetricPath<T: _wxString>(&self, path: T) {
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
    fn setPreviewCommand<T: _wxCommand>(&self, command: T) {
        unsafe { wxPrintData_SetPreviewCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.handle(), printMode) }
    }
    #[fixed_stack_segment]
    fn setPrinterCommand<T: _wxCommand>(&self, command: T) {
        unsafe { wxPrintData_SetPrinterCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterName<T: _wxString>(&self, name: T) {
        unsafe { wxPrintData_SetPrinterName(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterOptions<T: _wxString>(&self, options: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxPostScriptPrintNativeData {
        unsafe { wxPostScriptPrintNativeData(wxPostScriptPrintNativeData_Create()) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxPrintDialogData>(parent: T, data: U) -> wxPrintDialog {
        unsafe { wxPrintDialog(wxPrintDialog_Create(parent.handle(), data.handle())) }
    }
}

trait _wxPrintDialog : _wxDialog {
    #[fixed_stack_segment]
    fn getPrintDC(&self) -> wxDC {
        unsafe { wxDC(wxPrintDialog_GetPrintDC(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrintData<T: _wxPrintData>(&self, _ref: T) {
        unsafe { wxPrintDialog_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self) -> wxPrintDialogData {
        unsafe { wxPrintDialogData(wxPrintDialog_GetPrintDialogData(self.handle())) }
    }
}

struct wxPrintDialogData(*u8);
impl _wxPrintDialogData for wxPrintDialogData {}
impl _wxObject for wxPrintDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialogData {
    #[fixed_stack_segment]
    pub fn newDefault() -> wxPrintDialogData {
        unsafe { wxPrintDialogData(wxPrintDialogData_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newFromData<T: _wxPrintData>(printData: T) -> wxPrintDialogData {
        unsafe { wxPrintDialogData(wxPrintDialogData_CreateFromData(printData.handle())) }
    }
}

trait _wxPrintDialogData : _wxObject {
    #[fixed_stack_segment]
    fn assign<T: _wxPrintDialogData>(&self, data: T) {
        unsafe { wxPrintDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData<T: _wxPrintData>(&self, data: T) {
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
    fn getPrintData<T: _wxPrintData>(&self, _ref: T) {
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
    fn setPrintData<T: _wxPrintData>(&self, printData: T) {
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
    #[fixed_stack_segment]
    pub fn newFromData<T: _wxPrintout, U: _wxPrintout, V: _wxPrintData>(printout: T, printoutForPrinting: U, data: V) -> wxPrintPreview {
        unsafe { wxPrintPreview(wxPrintPreview_CreateFromData(printout.handle(), printoutForPrinting.handle(), data.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromDialogData<T: _wxPrintout, U: _wxPrintout, V: _wxPrintDialogData>(printout: T, printoutForPrinting: U, data: V) -> wxPrintPreview {
        unsafe { wxPrintPreview(wxPrintPreview_CreateFromDialogData(printout.handle(), printoutForPrinting.handle(), data.handle())) }
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
    fn drawBlankPage<T: _wxPreviewCanvas, U: _wxDC>(&self, canvas: T, dc: U) -> bool {
        unsafe { wxPrintPreview_DrawBlankPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn getCanvas(&self) -> wxPreviewCanvas {
        unsafe { wxPreviewCanvas(wxPrintPreview_GetCanvas(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame(wxPrintPreview_GetFrame(self.handle())) }
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
    fn getPrintDialogData<T: _wxPrintDialogData>(&self, _ref: T) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintout(&self) -> wxPrintout {
        unsafe { wxPrintout(wxPrintPreview_GetPrintout(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrintoutForPrinting(&self) -> wxPrintout {
        unsafe { wxPrintout(wxPrintPreview_GetPrintoutForPrinting(self.handle())) }
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
    fn paintPage<T: _wxPrintPreview, U: _wxDC>(&self, canvas: T, dc: U) -> bool {
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
    fn setCanvas<T: _wxPreviewCanvas>(&self, canvas: T) {
        unsafe { wxPrintPreview_SetCanvas(self.handle(), canvas.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrentPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_SetCurrentPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    fn setFrame<T: _wxFrame>(&self, frame: T) {
        unsafe { wxPrintPreview_SetFrame(self.handle(), frame.handle()) }
    }
    #[fixed_stack_segment]
    fn setOk(&self, ok: bool) {
        unsafe { wxPrintPreview_SetOk(self.handle(), ok) }
    }
    #[fixed_stack_segment]
    fn setPrintout<T: _wxPrintout>(&self, printout: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxPrintDialogData>(data: T) -> wxPrinter {
        unsafe { wxPrinter(wxPrinter_Create(data.handle())) }
    }
}

trait _wxPrinter : _wxObject {
    #[fixed_stack_segment]
    fn newAbortWindow<T: _wxWindow, U: _wxPrintout>(&self, parent: T, printout: U) -> wxWindow {
        unsafe { wxWindow(wxPrinter_CreateAbortWindow(self.handle(), parent.handle(), printout.handle())) }
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
    fn getPrintDialogData<T: _wxPrintDialogData>(&self, _ref: T) {
        unsafe { wxPrinter_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn print<T: _wxWindow, U: _wxPrintout>(&self, parent: T, printout: U, prompt: bool) -> bool {
        unsafe { wxPrinter_Print(self.handle(), parent.handle(), printout.handle(), prompt) }
    }
    #[fixed_stack_segment]
    fn printDialog<T: _wxWindow>(&self, parent: T) -> wxDC {
        unsafe { wxDC(wxPrinter_PrintDialog(self.handle(), parent.handle())) }
    }
    #[fixed_stack_segment]
    fn reportError<T: _wxWindow, U: _wxPrintout, V: _wxString>(&self, parent: T, printout: U, message: V) {
        unsafe { wxPrinter_ReportError(self.handle(), parent.handle(), printout.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setup<T: _wxWindow>(&self, parent: T) -> bool {
        unsafe { wxPrinter_Setup(self.handle(), parent.handle()) }
    }
}

struct wxPrinterDC(*u8);
impl _wxPrinterDC for wxPrinterDC {}
impl _wxDC for wxPrinterDC {}
impl _wxObject for wxPrinterDC { fn handle(&self) -> *u8 { **self } }

impl wxPrinterDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxPrintData>(data: T) -> wxPrinterDC {
        unsafe { wxPrinterDC(wxPrinterDC_Create(data.handle())) }
    }
}

trait _wxPrinterDC : _wxDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinterDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperRect(&self) -> wxRect {
        unsafe { wxRect(wxPrinterDC_GetPaperRect(self.handle())) }
    }
}

struct wxPrintout(*u8);
impl _wxPrintout for wxPrintout {}
impl _wxObject for wxPrintout { fn handle(&self) -> *u8 { **self } }

impl wxPrintout {
}

trait _wxPrintout : _wxObject {
    #[fixed_stack_segment]
    fn getDC(&self) -> wxDC {
        unsafe { wxDC(wxPrintout_GetDC(self.handle())) }
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
    fn getTitle(&self) -> wxString {
        unsafe { wxString(wxPrintout_GetTitle(self.handle())) }
    }
    #[fixed_stack_segment]
    fn isPreview(&self) -> bool {
        unsafe { wxPrintout_IsPreview(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDC<T: _wxDC>(&self, dc: T) {
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
}

trait _wxPrivateDropTarget : _wxDropTarget {
}

struct wxProcess(*u8);
impl _wxProcess for wxProcess {}
impl _wxEvtHandler for wxProcess {}
impl _wxObject for wxProcess { fn handle(&self) -> *u8 { **self } }

impl wxProcess {
    #[fixed_stack_segment]
    pub fn newDefault<T: _wxWindow>(_prt: T, _id: c_int) -> wxProcess {
        unsafe { wxProcess(wxProcess_CreateDefault(_prt.handle(), _id)) }
    }
    #[fixed_stack_segment]
    pub fn newRedirect<T: _wxWindow>(_prt: T, _rdr: bool) -> wxProcess {
        unsafe { wxProcess(wxProcess_CreateRedirect(_prt.handle(), _rdr)) }
    }
    #[fixed_stack_segment]
    pub fn open<T: _wxString>(cmd: T, flags: c_int) -> wxProcess {
        unsafe { wxProcess(wxProcess_Open(cmd.handle(), flags)) }
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
    fn getErrorStream(&self) -> wxInputStream {
        unsafe { wxInputStream(wxProcess_GetErrorStream(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getInputStream(&self) -> wxInputStream {
        unsafe { wxInputStream(wxProcess_GetInputStream(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getOutputStream(&self) -> wxOutputStream {
        unsafe { wxOutputStream(wxProcess_GetOutputStream(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString, V: _wxWindow>(title: T, message: U, max: c_int, parent: V, style: c_int) -> wxProgressDialog {
        unsafe { wxProgressDialog(wxProgressDialog_Create(title.handle(), message.handle(), max, parent.handle(), style)) }
    }
}

trait _wxProgressDialog : _wxFrame {
    #[fixed_stack_segment]
    fn update(&self, value: c_int) -> bool {
        unsafe { wxProgressDialog_Update(self.handle(), value) }
    }
    #[fixed_stack_segment]
    fn updateWithMessage<T: _wxString>(&self, value: c_int, message: T) -> bool {
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
}

trait _wxProtocol : _wxSocketClient {
}

struct wxQuantize(*u8);
impl _wxQuantize for wxQuantize {}
impl _wxObject for wxQuantize { fn handle(&self) -> *u8 { **self } }

impl wxQuantize {
}

trait _wxQuantize : _wxObject {
}

struct wxQueryCol(*u8);
impl _wxQueryCol for wxQueryCol {}
impl _wxObject for wxQueryCol { fn handle(&self) -> *u8 { **self } }

impl wxQueryCol {
}

trait _wxQueryCol : _wxObject {
}

struct wxQueryField(*u8);
impl _wxQueryField for wxQueryField {}
impl _wxObject for wxQueryField { fn handle(&self) -> *u8 { **self } }

impl wxQueryField {
}

trait _wxQueryField : _wxObject {
}

struct wxQueryLayoutInfoEvent(*u8);
impl _wxQueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl _wxEvent for wxQueryLayoutInfoEvent {}
impl _wxObject for wxQueryLayoutInfoEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryLayoutInfoEvent {
    #[fixed_stack_segment]
    pub fn new(id: c_int) -> wxQueryLayoutInfoEvent {
        unsafe { wxQueryLayoutInfoEvent(wxQueryLayoutInfoEvent_Create(id)) }
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
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxQueryLayoutInfoEvent_GetSize(self.handle())) }
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
}

trait _wxQueryNewPaletteEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject<T: _wxObject>(&self, obj: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: c_int, _stl: c_int) -> wxRadioBox {
        unsafe { wxRadioBox(wxRadioBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) }
    }
}

trait _wxRadioBox : _wxControl {
    #[fixed_stack_segment]
    fn enableItem(&self, item: c_int, enable: bool) {
        unsafe { wxRadioBox_EnableItem(self.handle(), item, enable) }
    }
    #[fixed_stack_segment]
    fn findString<T: _wxString>(&self, s: T) -> c_int {
        unsafe { wxRadioBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemLabel(&self, item: c_int) -> wxString {
        unsafe { wxString(wxRadioBox_GetItemLabel(self.handle(), item)) }
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
    fn getStringSelection(&self) -> wxString {
        unsafe { wxString(wxRadioBox_GetStringSelection(self.handle())) }
    }
    #[fixed_stack_segment]
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBitmap<T: _wxBitmap>(&self, item: c_int, bitmap: T) {
        unsafe { wxRadioBox_SetItemBitmap(self.handle(), item, bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel<T: _wxString>(&self, item: c_int, label: T) {
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
    fn setStringSelection<T: _wxString>(&self, s: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxRadioButton {
        unsafe { wxRadioButton(wxRadioButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
}

trait _wxRealPoint {
    fn handle(&self) -> *u8;
    
}

struct wxRecordSet(*u8);
impl _wxRecordSet for wxRecordSet {}
impl _wxObject for wxRecordSet { fn handle(&self) -> *u8 { **self } }

impl wxRecordSet {
}

trait _wxRecordSet : _wxObject {
}

struct wxRect(*u8);
impl _wxRect for wxRect { fn handle(&self) -> *u8 { **self } }

impl wxRect {
}

trait _wxRect {
    fn handle(&self) -> *u8;
    
}

struct wxRegEx(*u8);
impl _wxRegEx for wxRegEx { fn handle(&self) -> *u8 { **self } }

impl wxRegEx {
}

trait _wxRegEx {
    fn handle(&self) -> *u8;
    
}

struct wxRegion(*u8);
impl _wxRegion for wxRegion {}
impl _wxGDIObject for wxRegion {}
impl _wxObject for wxRegion { fn handle(&self) -> *u8 { **self } }

impl wxRegion {
    #[fixed_stack_segment]
    pub fn newDefault() -> wxRegion {
        unsafe { wxRegion(wxRegion_CreateDefault()) }
    }
    #[fixed_stack_segment]
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> wxRegion {
        unsafe { wxRegion(wxRegion_CreateFromRect(x, y, w, h)) }
    }
}

trait _wxRegion : _wxGDIObject {
    #[fixed_stack_segment]
    fn assign<T: _wxRegion>(&self, region: T) {
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
    fn intersectRegion<T: _wxRegion>(&self, region: T) -> bool {
        unsafe { wxRegion_IntersectRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_SubtractRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn subtractRegion<T: _wxRegion>(&self, region: T) -> bool {
        unsafe { wxRegion_SubtractRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_UnionRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn unionRegion<T: _wxRegion>(&self, region: T) -> bool {
        unsafe { wxRegion_UnionRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_XorRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn xorRegion<T: _wxRegion>(&self, region: T) -> bool {
        unsafe { wxRegion_XorRegion(self.handle(), region.handle()) }
    }
}

struct wxRegionIterator(*u8);
impl _wxRegionIterator for wxRegionIterator {}
impl _wxObject for wxRegionIterator { fn handle(&self) -> *u8 { **self } }

impl wxRegionIterator {
    #[fixed_stack_segment]
    pub fn new() -> wxRegionIterator {
        unsafe { wxRegionIterator(wxRegionIterator_Create()) }
    }
    #[fixed_stack_segment]
    pub fn newFromRegion<T: _wxRegion>(region: T) -> wxRegionIterator {
        unsafe { wxRegionIterator(wxRegionIterator_CreateFromRegion(region.handle())) }
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
    fn resetToRegion<T: _wxRegion>(&self, region: T) {
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
}

trait _wxRemotelyScrolledTreeCtrl : _wxTreeCtrl {
}

struct wxSVGFileDC(*u8);
impl _wxSVGFileDC for wxSVGFileDC {}
impl _wxDC for wxSVGFileDC {}
impl _wxObject for wxSVGFileDC { fn handle(&self) -> *u8 { **self } }

impl wxSVGFileDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(fileName: T) -> wxSVGFileDC {
        unsafe { wxSVGFileDC(wxSVGFileDC_Create(fileName.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newWithSize<T: _wxString>(fileName: T, w: c_int, h: c_int) -> wxSVGFileDC {
        unsafe { wxSVGFileDC(wxSVGFileDC_CreateWithSize(fileName.handle(), w, h)) }
    }
    #[fixed_stack_segment]
    pub fn newWithSizeAndResolution<T: _wxString>(fileName: T, w: c_int, h: c_int, a_dpi: c_float) -> wxSVGFileDC {
        unsafe { wxSVGFileDC(wxSVGFileDC_CreateWithSizeAndResolution(fileName.handle(), w, h, a_dpi)) }
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
    #[fixed_stack_segment]
    pub fn new(id: c_int, edge: c_int) -> wxSashEvent {
        unsafe { wxSashEvent(wxSashEvent_Create(id, edge)) }
    }
}

trait _wxSashEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getDragRect(&self) -> wxRect {
        unsafe { wxRect(wxSashEvent_GetDragRect(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_par: T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxSashLayoutWindow {
        unsafe { wxSashLayoutWindow(wxSashLayoutWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_par: T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxSashWindow {
        unsafe { wxSashWindow(wxSashWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) }
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
}

trait _wxScopedArray {
    fn handle(&self) -> *u8;
    
}

struct wxScopedPtr(*u8);
impl _wxScopedPtr for wxScopedPtr { fn handle(&self) -> *u8 { **self } }

impl wxScopedPtr {
}

trait _wxScopedPtr {
    fn handle(&self) -> *u8;
    
}

struct wxScreenDC(*u8);
impl _wxScreenDC for wxScreenDC {}
impl _wxDC for wxScreenDC {}
impl _wxObject for wxScreenDC { fn handle(&self) -> *u8 { **self } }

impl wxScreenDC {
    #[fixed_stack_segment]
    pub fn new() -> wxScreenDC {
        unsafe { wxScreenDC(wxScreenDC_Create()) }
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
    fn startDrawingOnTopOfWin<T: _wxWindow>(&self, win: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxScrollBar {
        unsafe { wxScrollBar(wxScrollBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxScrolledWindow {
        unsafe { wxScrolledWindow(wxScrolledWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    fn getTargetWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxScrolledWindow_GetTargetWindow(self.handle())) }
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
    fn onDraw<T: _wxDC>(&self, dc: T) {
        unsafe { wxScrolledWindow_OnDraw(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn prepareDC<T: _wxDC>(&self, dc: T) {
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
    fn setTargetWindow<T: _wxWindow>(&self, target: T) {
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
}

trait _wxSemaphore {
    fn handle(&self) -> *u8;
    
}

struct wxServer(*u8);
impl _wxServer for wxServer {}
impl _wxServerBase for wxServer {}
impl _wxObject for wxServer { fn handle(&self) -> *u8 { **self } }

impl wxServer {
}

trait _wxServer : _wxServerBase {
}

struct wxServerBase(*u8);
impl _wxServerBase for wxServerBase {}
impl _wxObject for wxServerBase { fn handle(&self) -> *u8 { **self } }

impl wxServerBase {
}

trait _wxServerBase : _wxObject {
}

struct wxSetCursorEvent(*u8);
impl _wxSetCursorEvent for wxSetCursorEvent {}
impl _wxEvent for wxSetCursorEvent {}
impl _wxObject for wxSetCursorEvent { fn handle(&self) -> *u8 { **self } }

impl wxSetCursorEvent {
}

trait _wxSetCursorEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getCursor(&self) -> wxCursor {
        unsafe { wxCursor(wxSetCursorEvent_GetCursor(self.handle())) }
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
    fn setCursor<T: _wxCursor>(&self, cursor: T) {
        unsafe { wxSetCursorEvent_SetCursor(self.handle(), cursor.handle()) }
    }
}

struct wxShowEvent(*u8);
impl _wxShowEvent for wxShowEvent {}
impl _wxEvent for wxShowEvent {}
impl _wxObject for wxShowEvent { fn handle(&self) -> *u8 { **self } }

impl wxShowEvent {
}

trait _wxShowEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject<T: _wxObject>(&self, obj: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxSimpleHelpProvider {
        unsafe { wxSimpleHelpProvider(wxSimpleHelpProvider_Create()) }
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
}

trait _wxSingleChoiceDialog : _wxDialog {
}

struct wxSingleInstanceChecker(*u8);
impl _wxSingleInstanceChecker for wxSingleInstanceChecker { fn handle(&self) -> *u8 { **self } }

impl wxSingleInstanceChecker {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString>(_obj: *u8, name: T, path: U) -> bool {
        unsafe { wxSingleInstanceChecker_Create(_obj, name.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxSingleInstanceChecker {
        unsafe { wxSingleInstanceChecker(wxSingleInstanceChecker_CreateDefault()) }
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
    #[fixed_stack_segment]
    pub fn new(w: c_int, h: c_int) -> wxSize {
        unsafe { wxSize(wxSize_Create(w, h)) }
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
}

trait _wxSizeEvent : _wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxSizeEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxSizeEvent_GetSize(self.handle())) }
    }
}

struct wxSizer(*u8);
impl _wxSizer for wxSizer {}
impl _wxObject for wxSizer { fn handle(&self) -> *u8 { **self } }

impl wxSizer {
}

trait _wxSizer : _wxObject {
    #[fixed_stack_segment]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Add(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addSizer<T: _wxSizer>(&self, sizer: T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addWindow<T: _wxWindow>(&self, window: T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize(wxSizer_CalcMin(self.handle())) }
    }
    #[fixed_stack_segment]
    fn fit<T: _wxWindow>(&self, window: T) {
        unsafe { wxSizer_Fit(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> wxSize {
        unsafe { wxSize(wxSizer_GetMinSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxSizer_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxSizer_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Insert(self.handle(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertSizer<T: _wxSizer>(&self, before: c_int, sizer: T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertSizer(self.handle(), before, sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertWindow<T: _wxWindow>(&self, before: c_int, window: T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn prependSizer<T: _wxSizer>(&self, sizer: T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_PrependSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependWindow<T: _wxWindow>(&self, window: T, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn setItemMinSizeSizer<T: _wxSizer>(&self, sizer: T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.handle(), sizer.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeWindow<T: _wxWindow>(&self, window: T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.handle(), window.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setSizeHints<T: _wxWindow>(&self, window: T) {
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
    fn detachWindow<T: _wxWindow>(&self, window: T) -> bool {
        unsafe { wxSizer_DetachWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn detachSizer<T: _wxSizer>(&self, sizer: T) -> bool {
        unsafe { wxSizer_DetachSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn detach(&self, index: c_int) -> bool {
        unsafe { wxSizer_Detach(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn fitInside<T: _wxWindow>(&self, window: T) {
        unsafe { wxSizer_FitInside(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getContainingWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxSizer_GetContainingWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getItemWindow<T: _wxWindow>(&self, window: T, recursive: bool) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_GetItemWindow(self.handle(), window.handle(), recursive)) }
    }
    #[fixed_stack_segment]
    fn getItemSizer<T: _wxSizer>(&self, window: T, recursive: bool) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_GetItemSizer(self.handle(), window.handle(), recursive)) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, index: c_int) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_GetItem(self.handle(), index)) }
    }
    #[fixed_stack_segment]
    fn hideWindow<T: _wxWindow>(&self, window: T) -> bool {
        unsafe { wxSizer_HideWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn hideSizer<T: _wxSizer>(&self, sizer: T) -> bool {
        unsafe { wxSizer_HideSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn hide(&self, index: c_int) -> bool {
        unsafe { wxSizer_Hide(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn insertSpacer(&self, index: c_int, size: c_int) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_InsertSpacer(self.handle(), index, size)) }
    }
    #[fixed_stack_segment]
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_InsertStretchSpacer(self.handle(), index, prop)) }
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
    fn prependSpacer(&self, size: c_int) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_PrependSpacer(self.handle(), size)) }
    }
    #[fixed_stack_segment]
    fn prependStretchSpacer(&self, prop: c_int) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizer_PrependStretchSpacer(self.handle(), prop)) }
    }
    #[fixed_stack_segment]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, oldwin: T, newwin: U, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceWindow(self.handle(), oldwin.handle(), newwin.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replaceSizer<T: _wxSizer, U: _wxSizer>(&self, oldsz: T, newsz: U, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceSizer(self.handle(), oldsz.handle(), newsz.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replace<T: _wxSizerItem>(&self, oldindex: c_int, newitem: T) -> bool {
        unsafe { wxSizer_Replace(self.handle(), oldindex, newitem.handle()) }
    }
    #[fixed_stack_segment]
    fn setVirtualSizeHints<T: _wxWindow>(&self, window: T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn showWindow<T: _wxWindow>(&self, window: T, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowWindow(self.handle(), window.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn showSizer<T: _wxSizer>(&self, sizer: T, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowSizer(self.handle(), sizer.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn show<T: _wxSizer>(&self, sizer: T, index: c_int, show: bool) -> bool {
        unsafe { wxSizer_Show(self.handle(), sizer.handle(), index, show) }
    }
}

struct wxSizerItem(*u8);
impl _wxSizerItem for wxSizerItem {}
impl _wxObject for wxSizerItem { fn handle(&self) -> *u8 { **self } }

impl wxSizerItem {
    #[fixed_stack_segment]
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) -> wxSizerItem {
        unsafe { wxSizerItem(wxSizerItem_Create(width, height, option, flag, border, userData)) }
    }
    #[fixed_stack_segment]
    pub fn newInSizer<T: _wxSizer>(sizer: T, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInSizer(sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    pub fn newInWindow<T: _wxWindow>(window: T, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInWindow(window.handle(), option, flag, border, userData) }
    }
}

trait _wxSizerItem : _wxObject {
    #[fixed_stack_segment]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize(wxSizerItem_CalcMin(self.handle())) }
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
    fn getMinSize(&self) -> wxSize {
        unsafe { wxSize(wxSizerItem_GetMinSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxSizerItem_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxSizerItem_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> wxSizer {
        unsafe { wxSizer(wxSizerItem_GetSizer(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getUserData(&self) -> *u8 {
        unsafe { wxSizerItem_GetUserData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxSizerItem_GetWindow(self.handle())) }
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
    fn setSizer<T: _wxSizer>(&self, sizer: T) {
        unsafe { wxSizerItem_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setWindow<T: _wxWindow>(&self, window: T) {
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
    fn getRect(&self) -> wxRect {
        unsafe { wxRect(wxSizerItem_GetRect(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSpacer(&self) -> wxSize {
        unsafe { wxSize(wxSizerItem_GetSpacer(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> wxSlider {
        unsafe { wxSlider(wxSlider_Create(_prt.handle(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) }
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
}

trait _wxSockAddress : _wxObject {
}

struct wxSocketBase(*u8);
impl _wxSocketBase for wxSocketBase {}
impl _wxObject for wxSocketBase { fn handle(&self) -> *u8 { **self } }

impl wxSocketBase {
}

trait _wxSocketBase : _wxObject {
}

struct wxSocketClient(*u8);
impl _wxSocketClient for wxSocketClient {}
impl _wxSocketBase for wxSocketClient {}
impl _wxObject for wxSocketClient { fn handle(&self) -> *u8 { **self } }

impl wxSocketClient {
}

trait _wxSocketClient : _wxSocketBase {
}

struct wxSocketEvent(*u8);
impl _wxSocketEvent for wxSocketEvent {}
impl _wxEvent for wxSocketEvent {}
impl _wxObject for wxSocketEvent { fn handle(&self) -> *u8 { **self } }

impl wxSocketEvent {
}

trait _wxSocketEvent : _wxEvent {
}

struct wxSocketInputStream(*u8);
impl _wxSocketInputStream for wxSocketInputStream {}
impl _wxInputStream for wxSocketInputStream {}
impl _wxStreamBase for wxSocketInputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketInputStream {
}

trait _wxSocketInputStream : _wxInputStream {
}

struct wxSocketOutputStream(*u8);
impl _wxSocketOutputStream for wxSocketOutputStream {}
impl _wxOutputStream for wxSocketOutputStream {}
impl _wxStreamBase for wxSocketOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketOutputStream {
}

trait _wxSocketOutputStream : _wxOutputStream {
}

struct wxSocketServer(*u8);
impl _wxSocketServer for wxSocketServer {}
impl _wxSocketBase for wxSocketServer {}
impl _wxObject for wxSocketServer { fn handle(&self) -> *u8 { **self } }

impl wxSocketServer {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> wxSpinButton {
        unsafe { wxSpinButton(wxSpinButton_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> wxSpinCtrl {
        unsafe { wxSpinCtrl(wxSpinCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) }
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
}

trait _wxSplitterScrolledWindow : _wxScrolledWindow {
}

struct wxSplitterWindow(*u8);
impl _wxSplitterWindow for wxSplitterWindow {}
impl _wxWindow for wxSplitterWindow {}
impl _wxEvtHandler for wxSplitterWindow {}
impl _wxObject for wxSplitterWindow { fn handle(&self) -> *u8 { **self } }

impl wxSplitterWindow {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxSplitterWindow {
        unsafe { wxSplitterWindow(wxSplitterWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    fn getWindow1(&self) -> wxWindow {
        unsafe { wxWindow(wxSplitterWindow_GetWindow1(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getWindow2(&self) -> wxWindow {
        unsafe { wxWindow(wxSplitterWindow_GetWindow2(self.handle())) }
    }
    #[fixed_stack_segment]
    fn initialize<T: _wxWindow>(&self, window: T) {
        unsafe { wxSplitterWindow_Initialize(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn isSplit(&self) -> bool {
        unsafe { wxSplitterWindow_IsSplit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceWindow<T: _wxWindow, U: _wxWindow>(&self, winOld: T, winNew: U) -> bool {
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
    fn splitHorizontally<T: _wxWindow, U: _wxWindow>(&self, window1: T, window2: U, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitHorizontally(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn splitVertically<T: _wxWindow, U: _wxWindow>(&self, window1: T, window2: U, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitVertically(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn unsplit<T: _wxWindow>(&self, toRemove: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: T, _id: c_int, bitmap: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticBitmap {
        unsafe { wxStaticBitmap(wxStaticBitmap_Create(_prt.handle(), _id, bitmap.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxStaticBitmap : _wxControl {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStaticBitmap_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmap<T: _wxBitmap>(&self, _ref: T) {
        unsafe { wxStaticBitmap_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon<T: _wxIcon>(&self, _ref: T) {
        unsafe { wxStaticBitmap_GetIcon(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmap<T: _wxBitmap>(&self, bitmap: T) {
        unsafe { wxStaticBitmap_SetBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon<T: _wxIcon>(&self, icon: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticBox {
        unsafe { wxStaticBox(wxStaticBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxStaticBox>(box: T, orient: c_int) -> wxStaticBoxSizer {
        unsafe { wxStaticBoxSizer(wxStaticBoxSizer_Create(box.handle(), orient)) }
    }
}

trait _wxStaticBoxSizer : _wxBoxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> wxSize {
        unsafe { wxSize(wxStaticBoxSizer_CalcMin(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self) -> wxStaticBox {
        unsafe { wxStaticBox(wxStaticBoxSizer_GetStaticBox(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticLine {
        unsafe { wxStaticLine(wxStaticLine_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStaticText {
        unsafe { wxStaticText(wxStaticText_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxStatusBar {
        unsafe { wxStatusBar(wxStatusBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
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
    fn getStatusText(&self, number: c_int) -> wxString {
        unsafe { wxString(wxStatusBar_GetStatusText(self.handle(), number)) }
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
    fn setStatusText<T: _wxString>(&self, text: T, number: c_int) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxStopWatch {
        unsafe { wxStopWatch(wxStopWatch_Create()) }
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
}

trait _wxStreamBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStreamToTextRedirector(*u8);
impl _wxStreamToTextRedirector for wxStreamToTextRedirector { fn handle(&self) -> *u8 { **self } }

impl wxStreamToTextRedirector {
}

trait _wxStreamToTextRedirector {
    fn handle(&self) -> *u8;
    
}

struct wxString(*u8);
impl _wxString for wxString { fn handle(&self) -> *u8 { **self } }

impl wxString {
    #[fixed_stack_segment]
    pub fn new(buffer: *wchar_t) -> wxString {
        unsafe { wxString(wxString_Create(buffer)) }
    }
    #[fixed_stack_segment]
    pub fn newLen(buffer: *wchar_t, len: c_int) -> wxString {
        unsafe { wxString(wxString_CreateLen(buffer, len)) }
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
}

trait _wxStringBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStringClientData(*u8);
impl _wxStringClientData for wxStringClientData {}
impl _wxClientData for wxStringClientData { fn handle(&self) -> *u8 { **self } }

impl wxStringClientData {
}

trait _wxStringClientData : _wxClientData {
}

struct wxStringList(*u8);
impl _wxStringList for wxStringList {}
impl _wxList for wxStringList {}
impl _wxObject for wxStringList { fn handle(&self) -> *u8 { **self } }

impl wxStringList {
}

trait _wxStringList : _wxList {
}

struct wxStringTokenizer(*u8);
impl _wxStringTokenizer for wxStringTokenizer {}
impl _wxObject for wxStringTokenizer { fn handle(&self) -> *u8 { **self } }

impl wxStringTokenizer {
}

trait _wxStringTokenizer : _wxObject {
}

struct wxSysColourChangedEvent(*u8);
impl _wxSysColourChangedEvent for wxSysColourChangedEvent {}
impl _wxEvent for wxSysColourChangedEvent {}
impl _wxObject for wxSysColourChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxSysColourChangedEvent {
}

trait _wxSysColourChangedEvent : _wxEvent {
}

struct wxSystemOptions(*u8);
impl _wxSystemOptions for wxSystemOptions {}
impl _wxObject for wxSystemOptions { fn handle(&self) -> *u8 { **self } }

impl wxSystemOptions {
}

trait _wxSystemOptions : _wxObject {
}

struct wxSystemSettings(*u8);
impl _wxSystemSettings for wxSystemSettings {}
impl _wxObject for wxSystemSettings { fn handle(&self) -> *u8 { **self } }

impl wxSystemSettings {
    #[fixed_stack_segment]
    pub fn getColour<T: _wxColour>(index: c_int, _ref: T) {
        unsafe { wxSystemSettings_GetColour(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getFont<T: _wxFont>(index: c_int, _ref: T) {
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
}

trait _wxTabCtrl : _wxControl {
}

struct wxTabEvent(*u8);
impl _wxTabEvent for wxTabEvent {}
impl _wxCommandEvent for wxTabEvent {}
impl _wxEvent for wxTabEvent {}
impl _wxObject for wxTabEvent { fn handle(&self) -> *u8 { **self } }

impl wxTabEvent {
}

trait _wxTabEvent : _wxCommandEvent {
}

struct wxTablesInUse(*u8);
impl _wxTablesInUse for wxTablesInUse {}
impl _wxObject for wxTablesInUse { fn handle(&self) -> *u8 { **self } }

impl wxTablesInUse {
}

trait _wxTablesInUse : _wxObject {
}

struct wxTaskBarIcon(*u8);
impl _wxTaskBarIcon for wxTaskBarIcon {}
impl _wxEvtHandler for wxTaskBarIcon {}
impl _wxObject for wxTaskBarIcon { fn handle(&self) -> *u8 { **self } }

impl wxTaskBarIcon {
    #[fixed_stack_segment]
    pub fn new() -> wxTaskBarIcon {
        unsafe { wxTaskBarIcon(wxTaskBarIcon_Create()) }
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
    fn popupMenu<T: _wxMenu>(&self, menu: T) -> bool {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn removeIcon(&self) -> bool {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon<T: _wxIcon, U: _wxString>(&self, icon: T, text: U) -> bool {
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

struct wxTempFile(*u8);
impl _wxTempFile for wxTempFile { fn handle(&self) -> *u8 { **self } }

impl wxTempFile {
}

trait _wxTempFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextAttr(*u8);
impl _wxTextAttr for wxTextAttr { fn handle(&self) -> *u8 { **self } }

impl wxTextAttr {
    #[fixed_stack_segment]
    pub fn new<T: _wxColour, U: _wxColour, V: _wxFont>(colText: T, colBack: U, font: V) -> wxTextAttr {
        unsafe { wxTextAttr(wxTextAttr_Create(colText.handle(), colBack.handle(), font.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> wxTextAttr {
        unsafe { wxTextAttr(wxTextAttr_CreateDefault()) }
    }
}

trait _wxTextAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxTextAttr_GetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont<T: _wxFont>(&self, font: T) {
        unsafe { wxTextAttr_GetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour<T: _wxColour>(&self, colour: T) {
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
    fn setTextColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxTextAttr_SetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour<T: _wxColour>(&self, colour: T) {
        unsafe { wxTextAttr_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont<T: _wxFont>(&self, font: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> wxTextCtrl {
        unsafe { wxTextCtrl(wxTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxTextCtrl : _wxControl {
    #[fixed_stack_segment]
    fn appendText<T: _wxString>(&self, text: T) {
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
    fn changeValue<T: _wxString>(&self, text: T) {
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
    fn getLineText(&self, lineNo: c_long) -> wxString {
        unsafe { wxString(wxTextCtrl_GetLineText(self.handle(), lineNo)) }
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
    fn getValue(&self) -> wxString {
        unsafe { wxString(wxTextCtrl_GetValue(self.handle())) }
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
    fn loadFile<T: _wxString>(&self, file: T) -> bool {
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
    fn replace<T: _wxString>(&self, from: c_long, to: c_long, value: T) {
        unsafe { wxTextCtrl_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile<T: _wxString>(&self, file: T) -> bool {
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
    fn setValue<T: _wxString>(&self, value: T) {
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
    fn writeText<T: _wxString>(&self, text: T) {
        unsafe { wxTextCtrl_WriteText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn emulateKeyPress<T: _wxKeyEvent>(&self, keyevent: T) -> bool {
        unsafe { wxTextCtrl_EmulateKeyPress(self.handle(), keyevent.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultStyle(&self) -> wxTextAttr {
        unsafe { wxTextAttr(wxTextCtrl_GetDefaultStyle(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getRange(&self, from: c_long, to: c_long) -> wxString {
        unsafe { wxString(wxTextCtrl_GetRange(self.handle(), from, to)) }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> wxString {
        unsafe { wxString(wxTextCtrl_GetStringSelection(self.handle())) }
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
    fn setDefaultStyle<T: _wxTextAttr>(&self, style: T) -> bool {
        unsafe { wxTextCtrl_SetDefaultStyle(self.handle(), style.handle()) }
    }
    #[fixed_stack_segment]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    fn setStyle<T: _wxTextAttr>(&self, start: c_long, end: c_long, style: T) -> bool {
        unsafe { wxTextCtrl_SetStyle(self.handle(), start, end, style.handle()) }
    }
}

struct wxTextDataObject(*u8);
impl _wxTextDataObject for wxTextDataObject {}
impl _wxDataObjectSimple for wxTextDataObject {}
impl _wxDataObject for wxTextDataObject { fn handle(&self) -> *u8 { **self } }

impl wxTextDataObject {
}

trait _wxTextDataObject : _wxDataObjectSimple {
}

struct wxTextDropTarget(*u8);
impl _wxTextDropTarget for wxTextDropTarget {}
impl _wxDropTarget for wxTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxTextDropTarget {
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
}

trait _wxTextEntryDialog : _wxDialog {
}

struct wxTextFile(*u8);
impl _wxTextFile for wxTextFile { fn handle(&self) -> *u8 { **self } }

impl wxTextFile {
}

trait _wxTextFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextInputStream(*u8);
impl _wxTextInputStream for wxTextInputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextInputStream {
    #[fixed_stack_segment]
    pub fn new<T: _wxInputStream, U: _wxString>(inputStream: T, sep: U) -> wxTextInputStream {
        unsafe { wxTextInputStream(wxTextInputStream_Create(inputStream.handle(), sep.handle())) }
    }
}

trait _wxTextInputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readLine(&self) -> wxString {
        unsafe { wxString(wxTextInputStream_ReadLine(self.handle())) }
    }
}

struct wxTextOutputStream(*u8);
impl _wxTextOutputStream for wxTextOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextOutputStream {
    #[fixed_stack_segment]
    pub fn new<T: _wxOutputStream>(outputStream: T, mode: c_int) -> wxTextOutputStream {
        unsafe { wxTextOutputStream(wxTextOutputStream_Create(outputStream.handle(), mode)) }
    }
}

trait _wxTextOutputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn writeString<T: _wxString>(&self, txt: T) {
        unsafe { wxTextOutputStream_WriteString(self.handle(), txt.handle()) }
    }
}

struct wxTextValidator(*u8);
impl _wxTextValidator for wxTextValidator {}
impl _wxValidator for wxTextValidator {}
impl _wxEvtHandler for wxTextValidator {}
impl _wxObject for wxTextValidator { fn handle(&self) -> *u8 { **self } }

impl wxTextValidator {
    #[fixed_stack_segment]
    pub fn new(style: c_int, val: *u8) -> wxTextValidator {
        unsafe { wxTextValidator(wxTextValidator_Create(style, val)) }
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
    fn clone(&self) -> wxValidator {
        unsafe { wxValidator(wxTextValidator_Clone(self.handle())) }
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
    fn onChar<T: _wxEvent>(&self, event: T) {
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
}

trait _wxThinSplitterWindow : _wxSplitterWindow {
}

struct wxThread(*u8);
impl _wxThread for wxThread { fn handle(&self) -> *u8 { **self } }

impl wxThread {
}

trait _wxThread {
    fn handle(&self) -> *u8;
    
}

struct wxTime(*u8);
impl _wxTime for wxTime {}
impl _wxObject for wxTime { fn handle(&self) -> *u8 { **self } }

impl wxTime {
}

trait _wxTime : _wxObject {
}

struct wxTimeSpan(*u8);
impl _wxTimeSpan for wxTimeSpan { fn handle(&self) -> *u8 { **self } }

impl wxTimeSpan {
}

trait _wxTimeSpan {
    fn handle(&self) -> *u8;
    
}

struct wxTimer(*u8);
impl _wxTimer for wxTimer {}
impl _wxObject for wxTimer { fn handle(&self) -> *u8 { **self } }

impl wxTimer {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int) -> wxTimer {
        unsafe { wxTimer(wxTimer_Create(_prt.handle(), _id)) }
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
}

trait _wxTimerBase : _wxObject {
}

struct wxTimerEvent(*u8);
impl _wxTimerEvent for wxTimerEvent {}
impl _wxEvent for wxTimerEvent {}
impl _wxObject for wxTimerEvent { fn handle(&self) -> *u8 { **self } }

impl wxTimerEvent {
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
    #[fixed_stack_segment]
    pub fn new() -> wxTimerEx {
        unsafe { wxTimerEx(wxTimerEx_Create()) }
    }
}

trait _wxTimerEx : _wxTimer {
    #[fixed_stack_segment]
    fn connect<T: _wxClosure>(&self, closure: T) {
        unsafe { wxTimerEx_Connect(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self) -> wxClosure {
        unsafe { wxClosure(wxTimerEx_GetClosure(self.handle())) }
    }
}

struct wxTimerRunner(*u8);
impl _wxTimerRunner for wxTimerRunner { fn handle(&self) -> *u8 { **self } }

impl wxTimerRunner {
}

trait _wxTimerRunner {
    fn handle(&self) -> *u8;
    
}

struct wxTipProvider(*u8);
impl _wxTipProvider for wxTipProvider { fn handle(&self) -> *u8 { **self } }

impl wxTipProvider {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(parent: T, text: U, maxLength: c_int) -> wxTipWindow {
        unsafe { wxTipWindow(wxTipWindow_Create(parent.handle(), text.handle(), maxLength)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(parent: T, id: c_int, label: U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> wxToggleButton {
        unsafe { wxToggleButton(wxToggleButton_Create(parent.handle(), id, label.handle(), x, y, w, h, style)) }
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
    fn setLabel<T: _wxString>(&self, label: T) {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxToolBar {
        unsafe { wxToolBar(wxToolBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxToolBar : _wxToolBarBase {
    #[fixed_stack_segment]
    fn addControl<T: _wxControl>(&self, ctrl: T) -> bool {
        unsafe { wxToolBar_AddControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn addTool<T: _wxBitmap, U: _wxString, V: _wxString>(&self, id: c_int, bmp: T, shelp: U, lhelp: V) {
        unsafe { wxToolBar_AddTool(self.handle(), id, bmp.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn addToolEx<T: _wxBitmap, U: _wxBitmap, V: _wxObject, W: _wxString, X: _wxString>(&self, id: c_int, bmp1: T, bmp2: U, isToggle: bool, x: c_int, y: c_int, data: V, shelp: W, lhelp: X) {
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
    fn getMargins(&self) -> wxPoint {
        unsafe { wxPoint(wxToolBar_GetMargins(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getToolBitmapSize(&self) -> wxSize {
        unsafe { wxSize(wxToolBar_GetToolBitmapSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getToolClientData(&self, id: c_int) -> wxObject {
        unsafe { wxObject(wxToolBar_GetToolClientData(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getToolEnabled(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn getToolLongHelp(&self, id: c_int) -> wxString {
        unsafe { wxString(wxToolBar_GetToolLongHelp(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolShortHelp(&self, id: c_int) -> wxString {
        unsafe { wxString(wxToolBar_GetToolShortHelp(self.handle(), id)) }
    }
    #[fixed_stack_segment]
    fn getToolSize(&self) -> wxSize {
        unsafe { wxSize(wxToolBar_GetToolSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getToolState(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolState(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn insertControl<T: _wxControl>(&self, pos: c_int, ctrl: T) {
        unsafe { wxToolBar_InsertControl(self.handle(), pos, ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn insertTool<T: _wxBitmap, U: _wxBitmap, V: _wxObject, W: _wxString, X: _wxString>(&self, pos: c_int, id: c_int, bmp1: T, bmp2: U, isToggle: bool, data: V, shelp: W, lhelp: X) {
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
    fn setToolClientData<T: _wxObject>(&self, id: c_int, data: T) {
        unsafe { wxToolBar_SetToolClientData(self.handle(), id, data.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolLongHelp<T: _wxString>(&self, id: c_int, str: T) {
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
    fn setToolShortHelp<T: _wxString>(&self, id: c_int, str: T) {
        unsafe { wxToolBar_SetToolShortHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    fn toggleTool(&self, id: c_int, toggle: bool) {
        unsafe { wxToolBar_ToggleTool(self.handle(), id, toggle) }
    }
    #[fixed_stack_segment]
    fn addTool2<T: _wxString, U: _wxBitmap, V: _wxBitmap, W: _wxString, X: _wxString>(&self, toolId: c_int, label: T, bmp: U, bmpDisabled: V, itemKind: c_int, shortHelp: W, longHelp: X) {
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
}

trait _wxToolBarBase : _wxControl {
}

struct wxToolLayoutItem(*u8);
impl _wxToolLayoutItem for wxToolLayoutItem {}
impl _wxObject for wxToolLayoutItem { fn handle(&self) -> *u8 { **self } }

impl wxToolLayoutItem {
}

trait _wxToolLayoutItem : _wxObject {
}

struct wxToolTip(*u8);
impl _wxToolTip for wxToolTip {}
impl _wxObject for wxToolTip { fn handle(&self) -> *u8 { **self } }

impl wxToolTip {
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
}

trait _wxToolWindow : _wxFrame {
}

struct wxTopLevelWindow(*u8);
impl _wxTopLevelWindow for wxTopLevelWindow {}
impl _wxWindow for wxTopLevelWindow {}
impl _wxEvtHandler for wxTopLevelWindow {}
impl _wxObject for wxTopLevelWindow { fn handle(&self) -> *u8 { **self } }

impl wxTopLevelWindow {
}

trait _wxTopLevelWindow : _wxWindow {
    #[fixed_stack_segment]
    fn enableCloseButton(&self, enable: bool) -> bool {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getDefaultButton(&self) -> wxButton {
        unsafe { wxButton(wxTopLevelWindow_GetDefaultButton(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getDefaultItem(&self) -> wxWindow {
        unsafe { wxWindow(wxTopLevelWindow_GetDefaultItem(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self) -> wxIcon {
        unsafe { wxIcon(wxTopLevelWindow_GetIcon(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> wxString {
        unsafe { wxString(wxTopLevelWindow_GetTitle(self.handle())) }
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
    fn setDefaultButton<T: _wxButton>(&self, pBut: T) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultItem<T: _wxWindow>(&self, pBut: T) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon<T: _wxIcon>(&self, pIcon: T) {
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
    fn setTitle<T: _wxString>(&self, pString: T) {
        unsafe { wxTopLevelWindow_SetTitle(self.handle(), pString.handle()) }
    }
}

struct wxTreeCompanionWindow(*u8);
impl _wxTreeCompanionWindow for wxTreeCompanionWindow {}
impl _wxWindow for wxTreeCompanionWindow {}
impl _wxEvtHandler for wxTreeCompanionWindow {}
impl _wxObject for wxTreeCompanionWindow { fn handle(&self) -> *u8 { **self } }

impl wxTreeCompanionWindow {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_obj: *u8, _cmp: *u8, _prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxTreeCtrl {
        unsafe { wxTreeCtrl(wxTreeCtrl_Create(_obj, _cmp, _prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
    #[fixed_stack_segment]
    pub fn new2<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxTreeCtrl {
        unsafe { wxTreeCtrl(wxTreeCtrl_Create2(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxTreeCtrl : _wxControl {
    #[fixed_stack_segment]
    fn addRoot<T: _wxString, U: _wxTreeItemData, V: _wxTreeItemId>(&self, text: T, image: c_int, selectedImage: c_int, data: U, _item: V) {
        unsafe { wxTreeCtrl_AddRoot(self.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendItem<T: _wxTreeItemId, U: _wxString, V: _wxTreeItemData, W: _wxTreeItemId>(&self, parent: T, text: U, image: c_int, selectedImage: c_int, data: V, _item: W) {
        unsafe { wxTreeCtrl_AppendItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapse<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_Collapse(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapseAndReset<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn delete<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_Delete(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteChildren<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_DeleteChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn editLabel<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_EditLabel(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn endEditLabel<T: _wxTreeItemId>(&self, item: T, discardChanges: bool) {
        unsafe { wxTreeCtrl_EndEditLabel(self.handle(), item.handle(), discardChanges) }
    }
    #[fixed_stack_segment]
    fn ensureVisible<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_EnsureVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn expand<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_Expand(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getBoundingRect<T: _wxTreeItemId>(&self, item: T, textOnly: bool) -> wxRect {
        unsafe { wxRect(wxTreeCtrl_GetBoundingRect(self.handle(), item.handle(), textOnly)) }
    }
    #[fixed_stack_segment]
    fn getChildrenCount<T: _wxTreeItemId>(&self, item: T, recursively: bool) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.handle(), item.handle(), recursively) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> wxTextCtrl {
        unsafe { wxTextCtrl(wxTreeCtrl_GetEditControl(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFirstChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, cookie: *c_int, _item: U) {
        unsafe { wxTreeCtrl_GetFirstChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleItem<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> wxImageList {
        unsafe { wxImageList(wxTreeCtrl_GetImageList(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemData<T: _wxTreeItemId>(&self, item: T) -> *u8 {
        unsafe { wxTreeCtrl_GetItemData(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemImage<T: _wxTreeItemId>(&self, item: T, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.handle(), item.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getItemText<T: _wxTreeItemId>(&self, item: T) -> wxString {
        unsafe { wxString(wxTreeCtrl_GetItemText(self.handle(), item.handle())) }
    }
    #[fixed_stack_segment]
    fn getLastChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetLastChild(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextChild<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, cookie: *c_int, _item: U) {
        unsafe { wxTreeCtrl_GetNextChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextSibling<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetNextSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextVisible<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetNextVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetParent(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevSibling<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevVisible<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item: T, _item: U) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getRootItem<T: _wxTreeItemId>(&self, _item: T) {
        unsafe { wxTreeCtrl_GetRootItem(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection<T: _wxTreeItemId>(&self, _item: T) {
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
    fn getStateImageList(&self) -> wxImageList {
        unsafe { wxImageList(wxTreeCtrl_GetStateImageList(self.handle())) }
    }
    #[fixed_stack_segment]
    fn hitTest<T: _wxTreeItemId>(&self, _x: c_int, _y: c_int, flags: *c_int, _item: T) {
        unsafe { wxTreeCtrl_HitTest(self.handle(), _x, _y, flags, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem<T: _wxTreeItemId, U: _wxTreeItemId, V: _wxString, W: _wxTreeItemId>(&self, parent: T, idPrevious: U, text: V, image: c_int, selectedImage: c_int, data: *u8, _item: W) {
        unsafe { wxTreeCtrl_InsertItem(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex<T: _wxTreeItemId, U: _wxString, V: _wxTreeItemId>(&self, parent: T, index: c_int, text: U, image: c_int, selectedImage: c_int, data: *u8, _item: V) {
        unsafe { wxTreeCtrl_InsertItemByIndex(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn isBold<T: _wxTreeItemId>(&self, item: T) -> bool {
        unsafe { wxTreeCtrl_IsBold(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isExpanded<T: _wxTreeItemId>(&self, item: T) -> bool {
        unsafe { wxTreeCtrl_IsExpanded(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isSelected<T: _wxTreeItemId>(&self, item: T) -> bool {
        unsafe { wxTreeCtrl_IsSelected(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isVisible<T: _wxTreeItemId>(&self, item: T) -> bool {
        unsafe { wxTreeCtrl_IsVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn itemHasChildren<T: _wxTreeItemId>(&self, item: T) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn onCompareItems<T: _wxTreeItemId, U: _wxTreeItemId>(&self, item1: T, item2: U) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.handle(), item1.handle(), item2.handle()) }
    }
    #[fixed_stack_segment]
    fn prependItem<T: _wxTreeItemId, U: _wxString, V: _wxTreeItemId>(&self, parent: T, text: U, image: c_int, selectedImage: c_int, data: *u8, _item: V) {
        unsafe { wxTreeCtrl_PrependItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn scrollTo<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_ScrollTo(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn selectItem<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_SelectItem(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList<T: _wxImageList>(&self, imageList: T) {
        unsafe { wxTreeCtrl_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.handle(), indent) }
    }
    #[fixed_stack_segment]
    fn setItemBackgroundColour<T: _wxTreeItemId, U: _wxColour>(&self, item: T, col: U) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBold<T: _wxTreeItemId>(&self, item: T, bold: bool) {
        unsafe { wxTreeCtrl_SetItemBold(self.handle(), item.handle(), bold) }
    }
    #[fixed_stack_segment]
    fn setItemData<T: _wxTreeItemId>(&self, item: T, data: *u8) {
        unsafe { wxTreeCtrl_SetItemData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setItemDropHighlight<T: _wxTreeItemId>(&self, item: T, highlight: bool) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.handle(), item.handle(), highlight) }
    }
    #[fixed_stack_segment]
    fn setItemFont<T: _wxTreeItemId, U: _wxFont>(&self, item: T, font: U) {
        unsafe { wxTreeCtrl_SetItemFont(self.handle(), item.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemHasChildren<T: _wxTreeItemId>(&self, item: T, hasChildren: bool) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.handle(), item.handle(), hasChildren) }
    }
    #[fixed_stack_segment]
    fn setItemImage<T: _wxTreeItemId>(&self, item: T, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.handle(), item.handle(), image, which) }
    }
    #[fixed_stack_segment]
    fn setItemText<T: _wxTreeItemId, U: _wxString>(&self, item: T, text: U) {
        unsafe { wxTreeCtrl_SetItemText(self.handle(), item.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemTextColour<T: _wxTreeItemId, U: _wxColour>(&self, item: T, col: U) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.handle(), spacing) }
    }
    #[fixed_stack_segment]
    fn setStateImageList<T: _wxImageList>(&self, imageList: T) {
        unsafe { wxTreeCtrl_SetStateImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn sortChildren<T: _wxTreeItemId>(&self, item: T) {
        unsafe { wxTreeCtrl_SortChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn toggle<T: _wxTreeItemId>(&self, item: T) {
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
    fn insertItem2<T: _wxWindow, U: _wxTreeItemId, V: _wxString, W: _wxClosure, X: _wxTreeItemId>(&self, parent: T, idPrevious: U, text: V, image: c_int, selectedImage: c_int, closure: W, _item: X) {
        unsafe { wxTreeCtrl_InsertItem2(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex2<T: _wxWindow, U: _wxString, V: _wxClosure, W: _wxTreeItemId>(&self, parent: T, index: c_int, text: U, image: c_int, selectedImage: c_int, closure: V, _item: W) {
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemClientClosure<T: _wxTreeItemId>(&self, item: T) -> wxClosure {
        unsafe { wxClosure(wxTreeCtrl_GetItemClientClosure(self.handle(), item.handle())) }
    }
    #[fixed_stack_segment]
    fn setItemClientClosure<T: _wxTreeItemId, U: _wxClosure>(&self, item: T, closure: U) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.handle(), item.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn assignImageList<T: _wxImageList>(&self, imageList: T) {
        unsafe { wxTreeCtrl_AssignImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn assignStateImageList<T: _wxImageList>(&self, imageList: T) {
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
}

trait _wxTreeEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem<T: _wxTreeItemId>(&self, _ref: T) {
        unsafe { wxTreeEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> wxString {
        unsafe { wxString(wxTreeEvent_GetLabel(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getOldItem<T: _wxTreeItemId>(&self, _ref: T) {
        unsafe { wxTreeEvent_GetOldItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> wxPoint {
        unsafe { wxPoint(wxTreeEvent_GetPoint(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getKeyEvent(&self) -> wxKeyEvent {
        unsafe { wxKeyEvent(wxTreeEvent_GetKeyEvent(self.handle())) }
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
}

trait _wxTreeItemData : _wxClientData {
}

struct wxTreeItemId(*u8);
impl _wxTreeItemId for wxTreeItemId { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemId {
    #[fixed_stack_segment]
    pub fn new() -> wxTreeItemId {
        unsafe { wxTreeItemId(wxTreeItemId_Create()) }
    }
    #[fixed_stack_segment]
    pub fn newFromValue(value: intptr_t) -> wxTreeItemId {
        unsafe { wxTreeItemId(wxTreeItemId_CreateFromValue(value)) }
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
    fn clone(&self) -> wxTreeItemId {
        unsafe { wxTreeItemId(wxTreeItemId_Clone(self.handle())) }
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
}

trait _wxTreeLayout : _wxObject {
}

struct wxTreeLayoutStored(*u8);
impl _wxTreeLayoutStored for wxTreeLayoutStored {}
impl _wxTreeLayout for wxTreeLayoutStored {}
impl _wxObject for wxTreeLayoutStored { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayoutStored {
}

trait _wxTreeLayoutStored : _wxTreeLayout {
}

struct wxURL(*u8);
impl _wxURL for wxURL {}
impl _wxObject for wxURL { fn handle(&self) -> *u8 { **self } }

impl wxURL {
}

trait _wxURL : _wxObject {
}

struct wxUpdateUIEvent(*u8);
impl _wxUpdateUIEvent for wxUpdateUIEvent {}
impl _wxEvent for wxUpdateUIEvent {}
impl _wxObject for wxUpdateUIEvent { fn handle(&self) -> *u8 { **self } }

impl wxUpdateUIEvent {
}

trait _wxUpdateUIEvent : _wxEvent {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxUpdateUIEvent_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    fn copyObject<T: _wxObject>(&self, obj: T) {
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
    fn getText(&self) -> wxString {
        unsafe { wxString(wxUpdateUIEvent_GetText(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setText<T: _wxString>(&self, text: T) {
        unsafe { wxUpdateUIEvent_SetText(self.handle(), text.handle()) }
    }
}

struct wxValidator(*u8);
impl _wxValidator for wxValidator {}
impl _wxEvtHandler for wxValidator {}
impl _wxObject for wxValidator { fn handle(&self) -> *u8 { **self } }

impl wxValidator {
    #[fixed_stack_segment]
    pub fn new() -> wxValidator {
        unsafe { wxValidator(wxValidator_Create()) }
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
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxValidator_GetWindow(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setWindow<T: _wxWindow>(&self, win: T) {
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
    fn validate<T: _wxWindow>(&self, parent: T) -> bool {
        unsafe { wxValidator_Validate(self.handle(), parent.handle()) }
    }
}

struct wxVariant(*u8);
impl _wxVariant for wxVariant {}
impl _wxObject for wxVariant { fn handle(&self) -> *u8 { **self } }

impl wxVariant {
}

trait _wxVariant : _wxObject {
}

struct wxVariantData(*u8);
impl _wxVariantData for wxVariantData {}
impl _wxObject for wxVariantData { fn handle(&self) -> *u8 { **self } }

impl wxVariantData {
}

trait _wxVariantData : _wxObject {
}

struct wxView(*u8);
impl _wxView for wxView {}
impl _wxEvtHandler for wxView {}
impl _wxObject for wxView { fn handle(&self) -> *u8 { **self } }

impl wxView {
}

trait _wxView : _wxEvtHandler {
}

struct wxSound(*u8);
impl _wxSound for wxSound {}
impl _wxEvtHandler for wxSound {}
impl _wxObject for wxSound { fn handle(&self) -> *u8 { **self } }

impl wxSound {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(fileName: T, isResource: bool) -> wxSound {
        unsafe { wxSound(wxSound_Create(fileName.handle(), isResource)) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxWindow {
        unsafe { wxWindow(wxWindow_Create(_prt.handle(), _id, _x, _y, _w, _h, _stl)) }
    }
}

trait _wxWindow : _wxEvtHandler {
    #[fixed_stack_segment]
    fn addChild<T: _wxWindow>(&self, child: T) {
        unsafe { wxWindow_AddChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn addConstraintReference<T: _wxWindow>(&self, otherWin: T) {
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
    fn clientToScreen(&self, x: c_int, y: c_int) -> wxPoint {
        unsafe { wxPoint(wxWindow_ClientToScreen(self.handle(), x, y)) }
    }
    #[fixed_stack_segment]
    fn close(&self, _force: bool) -> bool {
        unsafe { wxWindow_Close(self.handle(), _force) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixels(&self) -> wxPoint {
        unsafe { wxPoint(wxWindow_ConvertDialogToPixels(self.handle())) }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialog(&self) -> wxPoint {
        unsafe { wxPoint(wxWindow_ConvertPixelsToDialog(self.handle())) }
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
    fn findFocus(&self) -> wxWindow {
        unsafe { wxWindow(wxWindow_FindFocus(self.handle())) }
    }
    #[fixed_stack_segment]
    fn findWindow<T: _wxString>(&self, name: T) -> wxWindow {
        unsafe { wxWindow(wxWindow_FindWindow(self.handle(), name.handle())) }
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
    fn getEffectiveMinSize(&self) -> wxSize {
        unsafe { wxSize(wxWindow_GetEffectiveMinSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour<T: _wxColour>(&self, _ref: T) {
        unsafe { wxWindow_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> wxSize {
        unsafe { wxSize(wxWindow_GetBestSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getCaret(&self) -> wxCaret {
        unsafe { wxCaret(wxWindow_GetCaret(self.handle())) }
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
    fn getClientData(&self) -> wxClientData {
        unsafe { wxClientData(wxWindow_GetClientData(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getClientSize(&self) -> wxSize {
        unsafe { wxSize(wxWindow_GetClientSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getConstraints(&self) -> wxLayoutConstraints {
        unsafe { wxLayoutConstraints(wxWindow_GetConstraints(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getConstraintsInvolvedIn(&self) -> *u8 {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCursor(&self) -> wxCursor {
        unsafe { wxCursor(wxWindow_GetCursor(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getDropTarget(&self) -> wxDropTarget {
        unsafe { wxDropTarget(wxWindow_GetDropTarget(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getEventHandler(&self) -> wxEvtHandler {
        unsafe { wxEvtHandler(wxWindow_GetEventHandler(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getFont<T: _wxFont>(&self, _ref: T) {
        unsafe { wxWindow_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getForegroundColour<T: _wxColour>(&self, _ref: T) {
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
    fn getLabel(&self) -> wxString {
        unsafe { wxString(wxWindow_GetLabel(self.handle())) }
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
    fn getName(&self) -> wxString {
        unsafe { wxString(wxWindow_GetName(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> wxWindow {
        unsafe { wxWindow(wxWindow_GetParent(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxWindow_GetPosition(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> wxRect {
        unsafe { wxRect(wxWindow_GetRect(self.handle())) }
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
    fn getSize(&self) -> wxSize {
        unsafe { wxSize(wxWindow_GetSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> wxSizer {
        unsafe { wxSizer(wxWindow_GetSizer(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTextExtent<T: _wxString, U: _wxFont>(&self, string: T, x: *c_int, y: *c_int, descent: *c_int, externalLeading: *c_int, theFont: U) {
        unsafe { wxWindow_GetTextExtent(self.handle(), string.handle(), x, y, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolTip(&self) -> wxString {
        unsafe { wxString(wxWindow_GetToolTip(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getUpdateRegion(&self) -> wxRegion {
        unsafe { wxRegion(wxWindow_GetUpdateRegion(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getValidator(&self) -> wxValidator {
        unsafe { wxValidator(wxWindow_GetValidator(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self) -> wxSize {
        unsafe { wxSize(wxWindow_GetVirtualSize(self.handle())) }
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
    fn popupMenu<T: _wxMenu>(&self, menu: T, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.handle(), menu.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn prepareDC<T: _wxDC>(&self, dc: T) {
        unsafe { wxWindow_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn pushEventHandler<T: _wxEvtHandler>(&self, handler: T) {
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
    fn removeChild<T: _wxWindow>(&self, child: T) {
        unsafe { wxWindow_RemoveChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn removeConstraintReference<T: _wxWindow>(&self, otherWin: T) {
        unsafe { wxWindow_RemoveConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    fn reparent<T: _wxWindow>(&self, _par: T) -> c_int {
        unsafe { wxWindow_Reparent(self.handle(), _par.handle()) }
    }
    #[fixed_stack_segment]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    fn screenToClient(&self, x: c_int, y: c_int) -> wxPoint {
        unsafe { wxPoint(wxWindow_ScreenToClient(self.handle(), x, y)) }
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
    fn setAcceleratorTable<T: _wxAcceleratorTable>(&self, accel: T) {
        unsafe { wxWindow_SetAcceleratorTable(self.handle(), accel.handle()) }
    }
    #[fixed_stack_segment]
    fn setAutoLayout(&self, autoLayout: bool) {
        unsafe { wxWindow_SetAutoLayout(self.handle(), autoLayout) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour<T: _wxColour>(&self, colour: T) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaret<T: _wxCaret>(&self, caret: T) {
        unsafe { wxWindow_SetCaret(self.handle(), caret.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData<T: _wxClientData>(&self, data: T) {
        unsafe { wxWindow_SetClientData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject<T: _wxClientData>(&self, data: T) {
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
    fn setConstraints<T: _wxLayoutConstraints>(&self, constraints: T) {
        unsafe { wxWindow_SetConstraints(self.handle(), constraints.handle()) }
    }
    #[fixed_stack_segment]
    fn setCursor<T: _wxCursor>(&self, cursor: T) -> c_int {
        unsafe { wxWindow_SetCursor(self.handle(), cursor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDropTarget<T: _wxDropTarget>(&self, dropTarget: T) {
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
    fn setFont<T: _wxFont>(&self, font: T) -> c_int {
        unsafe { wxWindow_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour<T: _wxColour>(&self, colour: T) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.handle(), _id) }
    }
    #[fixed_stack_segment]
    fn setLabel<T: _wxString>(&self, _title: T) {
        unsafe { wxWindow_SetLabel(self.handle(), _title.handle()) }
    }
    #[fixed_stack_segment]
    fn setName<T: _wxString>(&self, _name: T) {
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
    fn setSizer<T: _wxSizer>(&self, sizer: T) {
        unsafe { wxWindow_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolTip<T: _wxString>(&self, tip: T) {
        unsafe { wxWindow_SetToolTip(self.handle(), tip.handle()) }
    }
    #[fixed_stack_segment]
    fn setValidator<T: _wxValidator>(&self, validator: T) {
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
    fn convertDialogToPixelsEx(&self) -> wxPoint {
        unsafe { wxPoint(wxWindow_ConvertDialogToPixelsEx(self.handle())) }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialogEx(&self) -> wxPoint {
        unsafe { wxPoint(wxWindow_ConvertPixelsToDialogEx(self.handle())) }
    }
    #[fixed_stack_segment]
    fn screenToClient2(&self, x: c_int, y: c_int) -> wxPoint {
        unsafe { wxPoint(wxWindow_ScreenToClient2(self.handle(), x, y)) }
    }
}

struct wxWindowCreateEvent(*u8);
impl _wxWindowCreateEvent for wxWindowCreateEvent {}
impl _wxCommandEvent for wxWindowCreateEvent {}
impl _wxEvent for wxWindowCreateEvent {}
impl _wxObject for wxWindowCreateEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowCreateEvent {
}

trait _wxWindowCreateEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxWindowCreateEvent_GetWindow(self.handle())) }
    }
}

struct wxWindowDC(*u8);
impl _wxWindowDC for wxWindowDC {}
impl _wxDC for wxWindowDC {}
impl _wxObject for wxWindowDC { fn handle(&self) -> *u8 { **self } }

impl wxWindowDC {
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(win: T) -> wxWindowDC {
        unsafe { wxWindowDC(wxWindowDC_Create(win.handle())) }
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
}

trait _wxWindowDestroyEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> wxWindow {
        unsafe { wxWindow(wxWindowDestroyEvent_GetWindow(self.handle())) }
    }
}

struct wxWindowDisabler(*u8);
impl _wxWindowDisabler for wxWindowDisabler { fn handle(&self) -> *u8 { **self } }

impl wxWindowDisabler {
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
    #[fixed_stack_segment]
    pub fn chain<T: _wxWizardPageSimple, U: _wxWizardPageSimple>(f: T, s: U) {
        unsafe { wxWizard_Chain(f.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString, V: _wxBitmap>(_prt: T, _id: c_int, _txt: U, _bmp: V, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> wxWizard {
        unsafe { wxWizard(wxWizard_Create(_prt.handle(), _id, _txt.handle(), _bmp.handle(), _lft, _top, _wdt, _hgt)) }
    }
}

trait _wxWizard : _wxDialog {
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> wxWizardPage {
        unsafe { wxWizardPage(wxWizard_GetCurrentPage(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> wxSize {
        unsafe { wxSize(wxWizard_GetPageSize(self.handle())) }
    }
    #[fixed_stack_segment]
    fn runWizard<T: _wxWizardPage>(&self, firstPage: T) -> c_int {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWizard>(_prt: T) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple(wxWizardPageSimple_Create(_prt.handle())) }
    }
}

trait _wxWizardPageSimple : _wxWizardPage {
    #[fixed_stack_segment]
    fn getBitmap<T: _wxBitmap>(&self, _ref: T) {
        unsafe { wxWizardPageSimple_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getNext(&self) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple(wxWizardPageSimple_GetNext(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getPrev(&self) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple(wxWizardPageSimple_GetPrev(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setNext<T: _wxWizardPageSimple>(&self, next: T) {
        unsafe { wxWizardPageSimple_SetNext(self.handle(), next.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrev<T: _wxWizardPageSimple>(&self, prev: T) {
        unsafe { wxWizardPageSimple_SetPrev(self.handle(), prev.handle()) }
    }
}

struct wxXmlResource(*u8);
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn handle(&self) -> *u8 { **self } }

impl wxXmlResource {
    #[fixed_stack_segment]
    pub fn new(flags: c_int) -> wxXmlResource {
        unsafe { wxXmlResource(wxXmlResource_Create(flags)) }
    }
    #[fixed_stack_segment]
    pub fn newFromFile<T: _wxString>(filemask: T, flags: c_int) -> wxXmlResource {
        unsafe { wxXmlResource(wxXmlResource_CreateFromFile(filemask.handle(), flags)) }
    }
    #[fixed_stack_segment]
    pub fn get() -> wxXmlResource {
        unsafe { wxXmlResource(wxXmlResource_Get()) }
    }
}

trait _wxXmlResource : _wxObject {
    #[fixed_stack_segment]
    fn addHandler<T: _wxEvtHandler>(&self, handler: T) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn addSubclassFactory(&self, factory: *u8) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    #[fixed_stack_segment]
    fn attachUnknownControl<T: _wxControl, U: _wxWindow>(&self, control: T, parent: U) -> c_int {
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
    fn getDomain(&self) -> wxString {
        unsafe { wxString(wxXmlResource_GetDomain(self.handle())) }
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
    fn getXRCID<T: _wxString>(&self, str_id: T) -> c_int {
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    #[fixed_stack_segment]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insertHandler<T: _wxEvtHandler>(&self, handler: T) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn load<T: _wxString>(&self, filemask: T) -> bool {
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn loadBitmap<T: _wxString, U: _wxBitmap>(&self, name: T, _ref: U) {
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadDialog<T: _wxWindow, U: _wxString>(&self, parent: T, name: U) -> wxDialog {
        unsafe { wxDialog(wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn loadFrame<T: _wxWindow, U: _wxString>(&self, parent: T, name: U) -> wxFrame {
        unsafe { wxFrame(wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn loadIcon<T: _wxString, U: _wxIcon>(&self, name: T, _ref: U) {
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadMenu<T: _wxString>(&self, name: T) -> wxMenu {
        unsafe { wxMenu(wxXmlResource_LoadMenu(self.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn loadMenuBar<T: _wxWindow, U: _wxString>(&self, parent: T, name: U) -> wxMenuBar {
        unsafe { wxMenuBar(wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn loadPanel<T: _wxWindow, U: _wxString>(&self, parent: T, name: U) -> wxPanel {
        unsafe { wxPanel(wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn loadToolBar<T: _wxWindow, U: _wxString>(&self, parent: T, name: U) -> wxToolBar {
        unsafe { wxToolBar(wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    fn getSizer<T: _wxString>(&self, str_id: T) -> wxSizer {
        unsafe { wxSizer(wxXmlResource_GetSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getBoxSizer<T: _wxString>(&self, str_id: T) -> wxBoxSizer {
        unsafe { wxBoxSizer(wxXmlResource_GetBoxSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getStaticBoxSizer<T: _wxString>(&self, str_id: T) -> wxStaticBoxSizer {
        unsafe { wxStaticBoxSizer(wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getGridSizer<T: _wxString>(&self, str_id: T) -> wxGridSizer {
        unsafe { wxGridSizer(wxXmlResource_GetGridSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getFlexGridSizer<T: _wxString>(&self, str_id: T) -> wxFlexGridSizer {
        unsafe { wxFlexGridSizer(wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getBitmapButton<T: _wxString>(&self, str_id: T) -> wxBitmapButton {
        unsafe { wxBitmapButton(wxXmlResource_GetBitmapButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getButton<T: _wxString>(&self, str_id: T) -> wxButton {
        unsafe { wxButton(wxXmlResource_GetButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getCalendarCtrl<T: _wxString>(&self, str_id: T) -> wxCalendarCtrl {
        unsafe { wxCalendarCtrl(wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getCheckBox<T: _wxString>(&self, str_id: T) -> wxCheckBox {
        unsafe { wxCheckBox(wxXmlResource_GetCheckBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getCheckListBox<T: _wxString>(&self, str_id: T) -> wxCheckListBox {
        unsafe { wxCheckListBox(wxXmlResource_GetCheckListBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getChoice<T: _wxString>(&self, str_id: T) -> wxChoice {
        unsafe { wxChoice(wxXmlResource_GetChoice(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getComboBox<T: _wxString>(&self, str_id: T) -> wxComboBox {
        unsafe { wxComboBox(wxXmlResource_GetComboBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getGauge<T: _wxString>(&self, str_id: T) -> wxGauge {
        unsafe { wxGauge(wxXmlResource_GetGauge(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getGrid<T: _wxString>(&self, str_id: T) -> wxGrid {
        unsafe { wxGrid(wxXmlResource_GetGrid(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getHtmlWindow<T: _wxString>(&self, str_id: T) -> wxHtmlWindow {
        unsafe { wxHtmlWindow(wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getListBox<T: _wxString>(&self, str_id: T) -> wxListBox {
        unsafe { wxListBox(wxXmlResource_GetListBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getListCtrl<T: _wxString>(&self, str_id: T) -> wxListCtrl {
        unsafe { wxListCtrl(wxXmlResource_GetListCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getMDIChildFrame<T: _wxString>(&self, str_id: T) -> wxMDIChildFrame {
        unsafe { wxMDIChildFrame(wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getMDIParentFrame<T: _wxString>(&self, str_id: T) -> wxMDIParentFrame {
        unsafe { wxMDIParentFrame(wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getMenu<T: _wxString>(&self, str_id: T) -> wxMenu {
        unsafe { wxMenu(wxXmlResource_GetMenu(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getMenuBar<T: _wxString>(&self, str_id: T) -> wxMenuBar {
        unsafe { wxMenuBar(wxXmlResource_GetMenuBar(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getMenuItem<T: _wxString>(&self, str_id: T) -> wxMenuItem {
        unsafe { wxMenuItem(wxXmlResource_GetMenuItem(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getNotebook<T: _wxString>(&self, str_id: T) -> wxNotebook {
        unsafe { wxNotebook(wxXmlResource_GetNotebook(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getPanel<T: _wxString>(&self, str_id: T) -> wxPanel {
        unsafe { wxPanel(wxXmlResource_GetPanel(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getRadioButton<T: _wxString>(&self, str_id: T) -> wxRadioButton {
        unsafe { wxRadioButton(wxXmlResource_GetRadioButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getRadioBox<T: _wxString>(&self, str_id: T) -> wxRadioBox {
        unsafe { wxRadioBox(wxXmlResource_GetRadioBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getScrollBar<T: _wxString>(&self, str_id: T) -> wxScrollBar {
        unsafe { wxScrollBar(wxXmlResource_GetScrollBar(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow<T: _wxString>(&self, str_id: T) -> wxScrolledWindow {
        unsafe { wxScrolledWindow(wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getSlider<T: _wxString>(&self, str_id: T) -> wxSlider {
        unsafe { wxSlider(wxXmlResource_GetSlider(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getSpinButton<T: _wxString>(&self, str_id: T) -> wxSpinButton {
        unsafe { wxSpinButton(wxXmlResource_GetSpinButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getSpinCtrl<T: _wxString>(&self, str_id: T) -> wxSpinCtrl {
        unsafe { wxSpinCtrl(wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getSplitterWindow<T: _wxString>(&self, str_id: T) -> wxSplitterWindow {
        unsafe { wxSplitterWindow(wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getStaticBitmap<T: _wxString>(&self, str_id: T) -> wxStaticBitmap {
        unsafe { wxStaticBitmap(wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getStaticBox<T: _wxString>(&self, str_id: T) -> wxStaticBox {
        unsafe { wxStaticBox(wxXmlResource_GetStaticBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getStaticLine<T: _wxString>(&self, str_id: T) -> wxStaticLine {
        unsafe { wxStaticLine(wxXmlResource_GetStaticLine(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getStaticText<T: _wxString>(&self, str_id: T) -> wxStaticText {
        unsafe { wxStaticText(wxXmlResource_GetStaticText(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getTextCtrl<T: _wxString>(&self, str_id: T) -> wxTextCtrl {
        unsafe { wxTextCtrl(wxXmlResource_GetTextCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl<T: _wxString>(&self, str_id: T) -> wxTreeCtrl {
        unsafe { wxTreeCtrl(wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    fn unload<T: _wxString>(&self, filemask: T) -> bool {
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, res: &_wxXmlResource) -> wxXmlResource {
        unsafe { wxXmlResource(wxXmlResource_Set(self.handle(), res.handle())) }
    }
    #[fixed_stack_segment]
    fn setDomain<T: _wxString>(&self, domain: T) {
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn getStyledTextCtrl<T: _wxString>(&self, str_id: T) -> wxStyledTextCtrl {
        unsafe { wxStyledTextCtrl(wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle())) }
    }
}

struct wxXmlResourceHandler(*u8);
impl _wxXmlResourceHandler for wxXmlResourceHandler {}
impl _wxObject for wxXmlResourceHandler { fn handle(&self) -> *u8 { **self } }

impl wxXmlResourceHandler {
}

trait _wxXmlResourceHandler : _wxObject {
}

struct wxZipInputStream(*u8);
impl _wxZipInputStream for wxZipInputStream {}
impl _wxInputStream for wxZipInputStream {}
impl _wxStreamBase for wxZipInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZipInputStream {
}

trait _wxZipInputStream : _wxInputStream {
}

struct wxZlibInputStream(*u8);
impl _wxZlibInputStream for wxZlibInputStream {}
impl _wxFilterInputStream for wxZlibInputStream {}
impl _wxInputStream for wxZlibInputStream {}
impl _wxStreamBase for wxZlibInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibInputStream {
}

trait _wxZlibInputStream : _wxFilterInputStream {
}

struct wxZlibOutputStream(*u8);
impl _wxZlibOutputStream for wxZlibOutputStream {}
impl _wxFilterOutputStream for wxZlibOutputStream {}
impl _wxOutputStream for wxZlibOutputStream {}
impl _wxStreamBase for wxZlibOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibOutputStream {
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxPropertyGrid {
        unsafe { wxPropertyGrid(wxPropertyGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) }
    }
}

trait _wxPropertyGrid : _wxControl {
    #[fixed_stack_segment]
    fn append<T: _wxPGProperty>(&self, prop: T) -> wxPGProperty {
        unsafe { wxPGProperty(wxPropertyGrid_Append(self.handle(), prop.handle())) }
    }
    #[fixed_stack_segment]
    fn disableProperty<T: _wxString>(&self, propName: T) -> bool {
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
}

trait _wxPropertyGridEvent : _wxNotifyEvent {
    #[fixed_stack_segment]
    fn hasProperty(&self) -> bool {
        unsafe { wxPropertyGridEvent_HasProperty(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getProperty(&self) -> wxPGProperty {
        unsafe { wxPGProperty(wxPropertyGridEvent_GetProperty(self.handle())) }
    }
}

struct wxPGProperty(*u8);
impl _wxPGProperty for wxPGProperty {}
impl _wxObject for wxPGProperty { fn handle(&self) -> *u8 { **self } }

impl wxPGProperty {
}

trait _wxPGProperty : _wxObject {
    #[fixed_stack_segment]
    fn getLabel(&self) -> wxString {
        unsafe { wxString(wxPGProperty_GetLabel(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> wxString {
        unsafe { wxString(wxPGProperty_GetName(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getValueAsString(&self) -> wxString {
        unsafe { wxString(wxPGProperty_GetValueAsString(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getValueType(&self) -> wxString {
        unsafe { wxString(wxPGProperty_GetValueType(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setHelpString<T: _wxString>(&self, helpString: T) {
        unsafe { wxPGProperty_SetHelpString(self.handle(), helpString.handle()) }
    }
}

struct wxStringProperty(*u8);
impl _wxStringProperty for wxStringProperty {}
impl _wxPGProperty for wxStringProperty {}
impl _wxObject for wxStringProperty { fn handle(&self) -> *u8 { **self } }

impl wxStringProperty {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString, V: _wxString>(label: T, name: U, value: V) -> wxStringProperty {
        unsafe { wxStringProperty(wxStringProperty_Create(label.handle(), name.handle(), value.handle())) }
    }
}

trait _wxStringProperty : _wxPGProperty {
}

struct wxIntProperty(*u8);
impl _wxIntProperty for wxIntProperty {}
impl _wxPGProperty for wxIntProperty {}
impl _wxObject for wxIntProperty { fn handle(&self) -> *u8 { **self } }

impl wxIntProperty {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString>(label: T, name: U, value: c_int) -> wxIntProperty {
        unsafe { wxIntProperty(wxIntProperty_Create(label.handle(), name.handle(), value)) }
    }
}

trait _wxIntProperty : _wxPGProperty {
}

struct wxBoolProperty(*u8);
impl _wxBoolProperty for wxBoolProperty {}
impl _wxPGProperty for wxBoolProperty {}
impl _wxObject for wxBoolProperty { fn handle(&self) -> *u8 { **self } }

impl wxBoolProperty {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString>(label: T, name: U, value: bool) -> wxBoolProperty {
        unsafe { wxBoolProperty(wxBoolProperty_Create(label.handle(), name.handle(), value)) }
    }
}

trait _wxBoolProperty : _wxPGProperty {
}

struct wxFloatProperty(*u8);
impl _wxFloatProperty for wxFloatProperty {}
impl _wxPGProperty for wxFloatProperty {}
impl _wxObject for wxFloatProperty { fn handle(&self) -> *u8 { **self } }

impl wxFloatProperty {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString>(label: T, name: U, value: c_float) -> wxFloatProperty {
        unsafe { wxFloatProperty(wxFloatProperty_Create(label.handle(), name.handle(), value)) }
    }
}

trait _wxFloatProperty : _wxPGProperty {
}

struct wxDateProperty(*u8);
impl _wxDateProperty for wxDateProperty {}
impl _wxPGProperty for wxDateProperty {}
impl _wxObject for wxDateProperty { fn handle(&self) -> *u8 { **self } }

impl wxDateProperty {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString, V: _wxDateTime>(label: T, name: U, value: V) -> wxDateProperty {
        unsafe { wxDateProperty(wxDateProperty_Create(label.handle(), name.handle(), value.handle())) }
    }
}

trait _wxDateProperty : _wxPGProperty {
}

struct wxFileProperty(*u8);
impl _wxFileProperty for wxFileProperty {}
impl _wxPGProperty for wxFileProperty {}
impl _wxObject for wxFileProperty { fn handle(&self) -> *u8 { **self } }

impl wxFileProperty {
    #[fixed_stack_segment]
    pub fn new<T: _wxString, U: _wxString, V: _wxString>(label: T, name: U, value: V) -> wxFileProperty {
        unsafe { wxFileProperty(wxFileProperty_Create(label.handle(), name.handle(), value.handle())) }
    }
}

trait _wxFileProperty : _wxPGProperty {
}

struct wxPropertyCategory(*u8);
impl _wxPropertyCategory for wxPropertyCategory {}
impl _wxPGProperty for wxPropertyCategory {}
impl _wxObject for wxPropertyCategory { fn handle(&self) -> *u8 { **self } }

impl wxPropertyCategory {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(label: T) -> wxPropertyCategory {
        unsafe { wxPropertyCategory(wxPropertyCategory_Create(label.handle())) }
    }
}

trait _wxPropertyCategory : _wxPGProperty {
}

struct wxGenericDragImage(*u8);
impl _wxGenericDragImage for wxGenericDragImage {}
impl _wxDragImage for wxGenericDragImage {}
impl _wxObject for wxGenericDragImage { fn handle(&self) -> *u8 { **self } }

impl wxGenericDragImage {
    #[fixed_stack_segment]
    pub fn new<T: _wxCursor>(cursor: T) -> wxGenericDragImage {
        unsafe { wxGenericDragImage(wxGenericDragImage_Create(cursor.handle())) }
    }
}

trait _wxGenericDragImage : _wxDragImage {
    #[fixed_stack_segment]
    fn doDrawImage<T: _wxDC>(&self, dc: T, x: c_int, y: c_int) -> bool {
        unsafe { wxGenericDragImage_DoDrawImage(self.handle(), dc.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> wxRect {
        unsafe { wxRect(wxGenericDragImage_GetImageRect(self.handle(), x_pos, y_pos)) }
    }
    #[fixed_stack_segment]
    fn updateBackingFromWindow<T: _wxDC, U: _wxMemoryDC>(&self, windowDC: T, destDC: U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.handle(), windowDC.handle(), destDC.handle(), x, y, w, h, xdest, ydest, width, height) }
    }
}

struct wxGraphicsObject(*u8);
impl _wxGraphicsObject for wxGraphicsObject {}
impl _wxObject for wxGraphicsObject { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsObject {
    #[fixed_stack_segment]
    pub fn getRenderer() -> wxGraphicsRenderer {
        unsafe { wxGraphicsRenderer(wxGraphicsObject_GetRenderer()) }
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
    #[fixed_stack_segment]
    pub fn new() -> wxGraphicsBrush {
        unsafe { wxGraphicsBrush(wxGraphicsBrush_Create()) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindowDC>(dc: T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsContext_Create(dc.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromWindow<T: _wxWindow>(window: T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsContext_CreateFromWindow(window.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromNative(context: *u8) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsContext_CreateFromNative(context)) }
    }
    #[fixed_stack_segment]
    pub fn newFromNativeWindow(window: *u8) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsContext_CreateFromNativeWindow(window)) }
    }
}

trait _wxGraphicsContext : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsContext_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clip<T: _wxRegion>(&self, region: T) {
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
    fn drawBitmap<T: _wxBitmap>(&self, bmp: T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.handle(), bmp.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawIcon<T: _wxIcon>(&self, icon: T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.handle(), icon.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.handle(), n, x, y, style) }
    }
    #[fixed_stack_segment]
    fn drawPath<T: _wxGraphicsPath>(&self, path: T, style: c_int) {
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
    fn drawText<T: _wxString>(&self, text: T, x: c_double, y: c_double) {
        unsafe { wxGraphicsContext_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawTextWithAngle<T: _wxString>(&self, text: T, x: c_double, y: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.handle(), text.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    fn fillPath<T: _wxGraphicsPath>(&self, path: T, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    fn strokePath<T: _wxGraphicsPath>(&self, path: T) {
        unsafe { wxGraphicsContext_StrokePath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativeContext(&self) -> *u8 {
        unsafe { wxGraphicsContext_GetNativeContext(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent<T: _wxString>(&self, text: T, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double) {
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
    fn setTransform<T: _wxGraphicsMatrix>(&self, path: T) {
        unsafe { wxGraphicsContext_SetTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn concatTransform<T: _wxGraphicsMatrix>(&self, path: T) {
        unsafe { wxGraphicsContext_ConcatTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBrush<T: _wxBrush>(&self, brush: T) {
        unsafe { wxGraphicsContext_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsBrush<T: _wxGraphicsBrush>(&self, brush: T) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont<T: _wxFont, U: _wxColour>(&self, font: T, colour: U) {
        unsafe { wxGraphicsContext_SetFont(self.handle(), font.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsFont<T: _wxGraphicsFont>(&self, font: T) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen<T: _wxPen>(&self, pen: T) {
        unsafe { wxGraphicsContext_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsPen<T: _wxGraphicsPen>(&self, pen: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxGraphicsFont {
        unsafe { wxGraphicsFont(wxGraphicsFont_Create()) }
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
    #[fixed_stack_segment]
    pub fn new() -> wxGraphicsMatrix {
        unsafe { wxGraphicsMatrix(wxGraphicsMatrix_Create()) }
    }
}

trait _wxGraphicsMatrix : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsMatrix_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn concat<T: _wxGraphicsMatrix>(&self, t: T) {
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
    fn isEqual<T: _wxGraphicsMatrix>(&self, t: T) -> bool {
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
    #[fixed_stack_segment]
    pub fn new() -> wxGraphicsPath {
        unsafe { wxGraphicsPath(wxGraphicsPath_Create()) }
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
    fn addPath<T: _wxGraphicsPath>(&self, x: c_double, y: c_double, path: T) {
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
    fn transform<T: _wxGraphicsMatrix>(&self, matrix: T) {
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
    #[fixed_stack_segment]
    pub fn new() -> wxGraphicsPen {
        unsafe { wxGraphicsPen(wxGraphicsPen_Create()) }
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
    #[fixed_stack_segment]
    pub fn newContext<T: _wxWindowDC>(dc: T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsRenderer_CreateContext(dc.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newContextFromWindow<T: _wxWindow>(window: T) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsRenderer_CreateContextFromWindow(window.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newContextFromNativeContext(context: *u8) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeContext(context)) }
    }
    #[fixed_stack_segment]
    pub fn newContextFromNativeWindow(window: *u8) -> wxGraphicsContext {
        unsafe { wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) }
    }
}

trait _wxGraphicsRenderer : _wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsRenderer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> wxGraphicsRenderer {
        unsafe { wxGraphicsRenderer(wxGraphicsRenderer_GetDefaultRenderer(self.handle())) }
    }
}

struct wxGLContext(*u8);
impl _wxGLContext for wxGLContext {}
impl _wxObject for wxGLContext { fn handle(&self) -> *u8 { **self } }

impl wxGLContext {
    #[fixed_stack_segment]
    pub fn new<T: _wxGLCanvas, U: _wxGLContext>(win: T, other: U) -> wxGLContext {
        unsafe { wxGLContext(wxGLContext_Create(win.handle(), other.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromNull<T: _wxGLCanvas>(win: T) -> wxGLContext {
        unsafe { wxGLContext(wxGLContext_CreateFromNull(win.handle())) }
    }
}

trait _wxGLContext : _wxObject {
    #[fixed_stack_segment]
    fn setCurrent<T: _wxGLCanvas>(&self, win: T) -> bool {
        unsafe { wxGLContext_SetCurrent(self.handle(), win.handle()) }
    }
}

struct wxManagedPtr(*u8);
impl _wxManagedPtr for wxManagedPtr { fn handle(&self) -> *u8 { **self } }

impl wxManagedPtr {
    #[fixed_stack_segment]
    pub fn getDeleteFunction() -> *u8 {
        unsafe { wxManagedPtr_GetDeleteFunction() }
    }
    #[fixed_stack_segment]
    pub fn newFromObject<T: _wxObject>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromObject(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromDateTime<T: _wxDateTime>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromDateTime(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromGridCellCoordsArray<T: _wxGridCellCoordsArray>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromGridCellCoordsArray(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap<T: _wxBitmap>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromBitmap(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromIcon<T: _wxIcon>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromIcon(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromBrush<T: _wxBrush>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromBrush(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromColour<T: _wxColour>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromColour(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromCursor<T: _wxCursor>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromCursor(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromFont<T: _wxFont>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromFont(obj.handle())) }
    }
    #[fixed_stack_segment]
    pub fn newFromPen<T: _wxPen>(obj: T) -> wxManagedPtr {
        unsafe { wxManagedPtr(wxManagedPtr_CreateFromPen(obj.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString, V: _wxString, W: _wxString>(parent: T, windowID: c_int, fileName: U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: V, name: W) -> wxMediaCtrl {
        unsafe { wxMediaCtrl(wxMediaCtrl_Create(parent.handle(), windowID, fileName.handle(), x, y, w, h, style, szBackend.handle(), name.handle())) }
    }
}

trait _wxMediaCtrl : _wxWindow {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMediaCtrl_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> wxSize {
        unsafe { wxSize(wxMediaCtrl_GetBestSize(self.handle())) }
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
    fn load<T: _wxString>(&self, fileName: T) -> bool {
        unsafe { wxMediaCtrl_Load(self.handle(), fileName.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURI<T: _wxString>(&self, uri: T) -> bool {
        unsafe { wxMediaCtrl_LoadURI(self.handle(), uri.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURIWithProxy<T: _wxString, U: _wxString>(&self, uri: T, proxy: U) -> bool {
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
}

trait _wxMediaEvent : _wxNotifyEvent {
}

struct wxcPrintout(*u8);
impl _wxcPrintout for wxcPrintout {}
impl _wxPrintout for wxcPrintout {}
impl _wxObject for wxcPrintout { fn handle(&self) -> *u8 { **self } }

impl wxcPrintout {
    #[fixed_stack_segment]
    pub fn new<T: _wxString>(title: T) -> wxcPrintout {
        unsafe { wxcPrintout(wxcPrintout_Create(title.handle())) }
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
    fn getEvtHandler(&self) -> wxcPrintoutHandler {
        unsafe { wxcPrintoutHandler(wxcPrintout_GetEvtHandler(self.handle())) }
    }
}

struct wxcPrintEvent(*u8);
impl _wxcPrintEvent for wxcPrintEvent {}
impl _wxEvent for wxcPrintEvent {}
impl _wxObject for wxcPrintEvent { fn handle(&self) -> *u8 { **self } }

impl wxcPrintEvent {
}

trait _wxcPrintEvent : _wxEvent {
    #[fixed_stack_segment]
    fn getPrintout(&self) -> wxcPrintout {
        unsafe { wxcPrintout(wxcPrintEvent_GetPrintout(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _txt: U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> wxStyledTextCtrl {
        unsafe { wxStyledTextCtrl(wxStyledTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, style)) }
    }
}

trait _wxStyledTextCtrl : _wxControl {
    #[fixed_stack_segment]
    fn addText<T: _wxString>(&self, text: T) {
        unsafe { wxStyledTextCtrl_AddText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addStyledText<T: _wxMemoryBuffer>(&self, data: T) {
        unsafe { wxStyledTextCtrl_AddStyledText(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn insertText<T: _wxString>(&self, pos: c_int, text: T) {
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
    fn markerDefineBitmap<T: _wxBitmap>(&self, markerNumber: c_int, bmp: T) {
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
    fn styleSetFaceName<T: _wxString>(&self, style: c_int, fontName: T) {
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
    fn setWordChars<T: _wxString>(&self, characters: T) {
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
    fn autoCompShow<T: _wxString>(&self, lenEntered: c_int, itemList: T) {
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
    fn autoCompStops<T: _wxString>(&self, characterSet: T) {
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
    fn autoCompSelect<T: _wxString>(&self, text: T) {
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
    fn autoCompSetFillUps<T: _wxString>(&self, characterSet: T) {
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
    fn userListShow<T: _wxString>(&self, listType: c_int, itemList: T) {
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
    fn registerImage<T: _wxBitmap>(&self, type_: c_int, bmp: T) {
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
    fn findText<T: _wxString>(&self, minPos: c_int, maxPos: c_int, text: T, flags: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_FindText(self.handle(), minPos, maxPos, text.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn formatRange<T: _wxDC, U: _wxDC, V: _wxRect, W: _wxRect>(&self, doDraw: bool, startPos: c_int, endPos: c_int, draw: T, target: U, renderRect: V, pageRect: W) -> c_int {
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
    fn replaceSelection<T: _wxString>(&self, text: T) {
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
    fn setText<T: _wxString>(&self, text: T) {
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
    fn replaceTarget<T: _wxString>(&self, text: T) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceTargetRE<T: _wxString>(&self, text: T) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchInTarget<T: _wxString>(&self, text: T) -> c_int {
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
    fn callTipShow<T: _wxString>(&self, pos: c_int, definition: T) {
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
    fn textWidth<T: _wxString>(&self, style: c_int, text: T) -> c_int {
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
    fn appendText<T: _wxString>(&self, text: T) {
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
    fn setDocPointer<T: _wxSTCDoc>(&self, docPointer: T) {
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
    fn searchNext<T: _wxString>(&self, flags: c_int, text: T) -> c_int {
        unsafe { wxStyledTextCtrl_SearchNext(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchPrev<T: _wxString>(&self, flags: c_int, text: T) -> c_int {
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
    fn addRefDocument<T: _wxSTCDoc>(&self, docPointer: T) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    fn releaseDocument<T: _wxSTCDoc>(&self, docPointer: T) {
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
    fn copyText<T: _wxString>(&self, length: c_int, text: T) {
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
    fn setProperty<T: _wxString, U: _wxString>(&self, key: T, value: U) {
        unsafe { wxStyledTextCtrl_SetProperty(self.handle(), key.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setKeyWords<T: _wxString>(&self, keywordSet: c_int, keyWords: T) {
        unsafe { wxStyledTextCtrl_SetKeyWords(self.handle(), keywordSet, keyWords.handle()) }
    }
    #[fixed_stack_segment]
    fn setLexerLanguage<T: _wxString>(&self, language: T) {
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self.handle(), language.handle()) }
    }
    #[fixed_stack_segment]
    fn getCurrentLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetSpec<T: _wxString>(&self, styleNum: c_int, spec: T) {
        unsafe { wxStyledTextCtrl_StyleSetSpec(self.handle(), styleNum, spec.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFont<T: _wxFont>(&self, styleNum: c_int, font: T) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self.handle(), styleNum, font.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFontAttr<T: _wxString>(&self, styleNum: c_int, size: c_int, faceName: T, bold: bool, italic: bool, underline: bool) {
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
    fn setVScrollBar<T: _wxScrollBar>(&self, bar: T) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    fn setHScrollBar<T: _wxScrollBar>(&self, bar: T) {
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
    fn saveFile<T: _wxString>(&self, filename: T) -> bool {
        unsafe { wxStyledTextCtrl_SaveFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile<T: _wxString>(&self, filename: T) -> bool {
        unsafe { wxStyledTextCtrl_LoadFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn indicatorGetForeground(&self, indic: c_int) -> wxColour {
        unsafe { wxColour(wxStyledTextCtrl_IndicatorGetForeground(self.handle(), indic)) }
    }
    #[fixed_stack_segment]
    fn getCaretLineBackground(&self) -> wxColour {
        unsafe { wxColour(wxStyledTextCtrl_GetCaretLineBackground(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setCaretLineBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn getCaretForeground(&self) -> wxColour {
        unsafe { wxColour(wxStyledTextCtrl_GetCaretForeground(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getLine(&self, line: c_int) -> wxString {
        unsafe { wxString(wxStyledTextCtrl_GetLine(self.handle(), line)) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> wxString {
        unsafe { wxString(wxStyledTextCtrl_GetText(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTextRange(&self, startPos: c_int, endPos: c_int) -> wxString {
        unsafe { wxString(wxStyledTextCtrl_GetTextRange(self.handle(), startPos, endPos)) }
    }
    #[fixed_stack_segment]
    fn getSelectedText(&self) -> wxString {
        unsafe { wxString(wxStyledTextCtrl_GetSelectedText(self.handle())) }
    }
    #[fixed_stack_segment]
    fn newDocument(&self) -> wxSTCDoc {
        unsafe { wxSTCDoc(wxStyledTextCtrl_CreateDocument(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getEdgeColour(&self) -> wxColour {
        unsafe { wxColour(wxStyledTextCtrl_GetEdgeColour(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getDocPointer(&self) -> wxSTCDoc {
        unsafe { wxSTCDoc(wxStyledTextCtrl_GetDocPointer(self.handle())) }
    }
    #[fixed_stack_segment]
    fn pointFromPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxStyledTextCtrl_PointFromPosition(self.handle())) }
    }
}

struct wxSTCDoc(*u8);
impl _wxSTCDoc for wxSTCDoc { fn handle(&self) -> *u8 { **self } }

impl wxSTCDoc {
}

trait _wxSTCDoc {
    fn handle(&self) -> *u8;
    
}

struct wxMemoryBuffer(*u8);
impl _wxMemoryBuffer for wxMemoryBuffer { fn handle(&self) -> *u8 { **self } }

impl wxMemoryBuffer {
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
    fn getDragText(&self) -> wxString {
        unsafe { wxString(wxStyledTextEvent_GetDragText(self.handle())) }
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
    fn getText(&self) -> wxString {
        unsafe { wxString(wxStyledTextEvent_GetText(self.handle())) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> wxStyledTextEvent {
        unsafe { wxStyledTextEvent(wxStyledTextEvent_Clone(self.handle())) }
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
    fn setText<T: _wxString>(&self, t: T) {
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
    fn setDragText<T: _wxString>(&self, val: T) {
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
}

trait _wxSliderMSW : _wxSlider {
}

struct wxcTreeItemData(*u8);
impl _wxcTreeItemData for wxcTreeItemData {}
impl _wxTreeItemData for wxcTreeItemData {}
impl _wxClientData for wxcTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxcTreeItemData {
    #[fixed_stack_segment]
    pub fn new<T: _wxClosure>(closure: T) -> wxcTreeItemData {
        unsafe { wxcTreeItemData(wxcTreeItemData_Create(closure.handle())) }
    }
}

trait _wxcTreeItemData : _wxTreeItemData {
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> wxClosure {
        unsafe { wxClosure(wxcTreeItemData_GetClientClosure(self.handle())) }
    }
    #[fixed_stack_segment]
    fn setClientClosure<T: _wxClosure>(&self, closure: T) {
        unsafe { wxcTreeItemData_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxInputSink(*u8);
impl _wxInputSink for wxInputSink {}
impl _wxThread for wxInputSink { fn handle(&self) -> *u8 { **self } }

impl wxInputSink {
    #[fixed_stack_segment]
    pub fn new<T: _wxInputStream, U: _wxEvtHandler>(input: T, evtHandler: U, bufferLen: c_int) -> wxInputSink {
        unsafe { wxInputSink(wxInputSink_Create(input.handle(), evtHandler.handle(), bufferLen)) }
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
}

trait _wxcHtmlEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    fn getMouseEvent(&self) -> wxMouseEvent {
        unsafe { wxMouseEvent(wxcHtmlEvent_GetMouseEvent(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getHtmlCell(&self) -> wxHtmlCell {
        unsafe { wxHtmlCell(wxcHtmlEvent_GetHtmlCell(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getHtmlCellId(&self) -> wxString {
        unsafe { wxString(wxcHtmlEvent_GetHtmlCellId(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getHref(&self) -> wxString {
        unsafe { wxString(wxcHtmlEvent_GetHref(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> wxString {
        unsafe { wxString(wxcHtmlEvent_GetTarget(self.handle())) }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self) -> wxPoint {
        unsafe { wxPoint(wxcHtmlEvent_GetLogicalPosition(self.handle())) }
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
    #[fixed_stack_segment]
    pub fn new<T: _wxWindow, U: _wxString>(_prt: T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: U) -> wxcHtmlWindow {
        unsafe { wxcHtmlWindow(wxcHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) }
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
    #[fixed_stack_segment]
    pub fn ctor() -> wxGridCellTextEnterEditor {
        unsafe { wxGridCellTextEnterEditor(wxGridCellTextEnterEditor_Ctor()) }
    }
}

trait _wxGridCellTextEnterEditor : _wxGridCellTextEditor {
}

struct wxFileConfig(*u8);
impl _wxFileConfig for wxFileConfig {}
impl _wxConfigBase for wxFileConfig { fn handle(&self) -> *u8 { **self } }

impl wxFileConfig {
    #[fixed_stack_segment]
    pub fn new<T: _wxInputStream>(inp: T) -> wxFileConfig {
        unsafe { wxFileConfig(wxFileConfig_Create(inp.handle())) }
    }
}

trait _wxFileConfig : _wxConfigBase {
}

