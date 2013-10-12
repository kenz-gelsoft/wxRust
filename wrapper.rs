use std::libc::*;
use native::*;

struct ELJAppImpl(*u8);
impl ELJApp for ELJAppImpl {}
impl wxApp for ELJAppImpl {}
impl wxEvtHandler for ELJAppImpl {}
impl wxObject for ELJAppImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJApp : wxApp {
    #[fixed_stack_segment]
    fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    fn newLogTarget() -> @ELJLog {
        unsafe { @ELJLogImpl(ELJApp_CreateLogTarget()) as @ELJLog }
    }
    #[fixed_stack_segment]
    fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    fn displaySize() -> @wxSize {
        unsafe { @wxSizeImpl(ELJApp_DisplaySize()) as @wxSize }
    }
    #[fixed_stack_segment]
    fn enableTooltips(_enable: bool) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    #[fixed_stack_segment]
    fn enableTopLevelWindows(_enb: c_int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    #[fixed_stack_segment]
    fn executeProcess(_cmd: @wxString, _snc: c_int, _prc: @wxProcess) -> c_int {
        unsafe { ELJApp_ExecuteProcess(_cmd.handle(), _snc, _prc.handle()) }
    }
    #[fixed_stack_segment]
    fn exit() {
        unsafe { ELJApp_Exit() }
    }
    #[fixed_stack_segment]
    fn exitMainLoop() {
        unsafe { ELJApp_ExitMainLoop() }
    }
    #[fixed_stack_segment]
    fn findWindowById(_id: c_int, _prt: @wxWindow) -> *u8 {
        unsafe { ELJApp_FindWindowById(_id, _prt.handle()) }
    }
    #[fixed_stack_segment]
    fn findWindowByLabel(_lbl: @wxString, _prt: @wxWindow) -> @wxWindow {
        unsafe { @wxWindowImpl(ELJApp_FindWindowByLabel(_lbl.handle(), _prt.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn findWindowByName(_lbl: @wxString, _prt: @wxWindow) -> @wxWindow {
        unsafe { @wxWindowImpl(ELJApp_FindWindowByName(_lbl.handle(), _prt.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getApp() -> @wxApp {
        unsafe { @wxAppImpl(ELJApp_GetApp()) as @wxApp }
    }
    #[fixed_stack_segment]
    fn getAppName() -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetAppName()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getClassName() -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetClassName()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    fn getOsDescription() -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetOsDescription()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getOsVersion(_maj: *u8, _min: *u8) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    fn getTopWindow() -> @wxWindow {
        unsafe { @wxWindowImpl(ELJApp_GetTopWindow()) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    fn getUserHome(_usr: *u8) -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetUserHome(_usr)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getUserId() -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetUserId()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getUserName() -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetUserName()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getVendorName() -> @wxString {
        unsafe { @wxStringImpl(ELJApp_GetVendorName()) as @wxString }
    }
    #[fixed_stack_segment]
    fn initAllImageHandlers() {
        unsafe { ELJApp_InitAllImageHandlers() }
    }
    #[fixed_stack_segment]
    fn initialized() -> bool {
        unsafe { ELJApp_Initialized() }
    }
    #[fixed_stack_segment]
    fn mainLoop() -> c_int {
        unsafe { ELJApp_MainLoop() }
    }
    #[fixed_stack_segment]
    fn mousePosition() -> @wxPoint {
        unsafe { @wxPointImpl(ELJApp_MousePosition()) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    fn safeYield(_win: @wxWindow) -> c_int {
        unsafe { ELJApp_SafeYield(_win.handle()) }
    }
    #[fixed_stack_segment]
    fn setAppName(name: @wxString) {
        unsafe { ELJApp_SetAppName(name.handle()) }
    }
    #[fixed_stack_segment]
    fn setClassName(name: @wxString) {
        unsafe { ELJApp_SetClassName(name.handle()) }
    }
    #[fixed_stack_segment]
    fn setExitOnFrameDelete(flag: c_int) {
        unsafe { ELJApp_SetExitOnFrameDelete(flag) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(mode: c_int) {
        unsafe { ELJApp_SetPrintMode(mode) }
    }
    #[fixed_stack_segment]
    fn setTooltipDelay(_ms: c_int) {
        unsafe { ELJApp_SetTooltipDelay(_ms) }
    }
    #[fixed_stack_segment]
    fn setTopWindow(_wnd: @wxWindow) {
        unsafe { ELJApp_SetTopWindow(_wnd.handle()) }
    }
    #[fixed_stack_segment]
    fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    #[fixed_stack_segment]
    fn setVendorName(name: @wxString) {
        unsafe { ELJApp_SetVendorName(name.handle()) }
    }
    #[fixed_stack_segment]
    fn sleep(_scs: c_int) {
        unsafe { ELJApp_Sleep(_scs) }
    }
    #[fixed_stack_segment]
    fn milliSleep(_mscs: c_int) {
        unsafe { ELJApp_MilliSleep(_mscs) }
    }
    #[fixed_stack_segment]
    fn yield_() -> c_int {
        unsafe { ELJApp_Yield() }
    }
    #[fixed_stack_segment]
    fn isTerminating() -> c_int {
        unsafe { ELJApp_IsTerminating() }
    }
    #[fixed_stack_segment]
    fn initializeC(closure: @wxClosure, _argc: c_int, _argv: *wchar_t) {
        unsafe { ELJApp_InitializeC(closure.handle(), _argc, _argv) }
    }
    #[fixed_stack_segment]
    fn getIdleInterval() -> c_int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    #[fixed_stack_segment]
    fn setIdleInterval(interval: c_int) {
        unsafe { ELJApp_SetIdleInterval(interval) }
    }
}

struct ELJArtProvImpl(*u8);
impl ELJArtProv for ELJArtProvImpl {}
impl wxArtProvider for ELJArtProvImpl {}
impl wxObject for ELJArtProvImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJArtProv : wxArtProvider {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _clb: *u8) -> @ELJArtProv {
        unsafe { @ELJArtProvImpl(ELJArtProv_Create(_obj, _clb)) as @ELJArtProv }
    }
    #[fixed_stack_segment]
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.handle()) }
    }
}

struct ELJClientImpl(*u8);
impl ELJClient for ELJClientImpl {}
impl wxClient for ELJClientImpl {}
impl wxClientBase for ELJClientImpl {}
impl wxObject for ELJClientImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJClient : wxClient {
    #[fixed_stack_segment]
    fn new(_eobj: *u8, _cnct: *u8) -> @ELJClient {
        unsafe { @ELJClientImpl(ELJClient_Create(_eobj, _cnct)) as @ELJClient }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJClient_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn makeConnection(&self, host: @wxString, server: @wxServer, topic: @wxString) {
        unsafe { ELJClient_MakeConnection(self.handle(), host.handle(), server.handle(), topic.handle()) }
    }
}

struct ELJCommandImpl(*u8);
impl ELJCommand for ELJCommandImpl {}
impl wxCommand for ELJCommandImpl {}
impl wxObject for ELJCommandImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJCommand : wxCommand {
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { ELJCommand_CanUndo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_und: c_int, _nme: @wxString, _obj: *u8, _clb: *u8) -> @ELJCommand {
        unsafe { @ELJCommandImpl(ELJCommand_Create(_und, _nme.handle(), _obj, _clb)) as @ELJCommand }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJCommand_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { @wxStringImpl(ELJCommand_GetName(self.handle())) as @wxString }
    }
}

struct ELJConnectionImpl(*u8);
impl ELJConnection for ELJConnectionImpl {}
impl wxConnection for ELJConnectionImpl {}
impl wxConnectionBase for ELJConnectionImpl {}
impl wxObject for ELJConnectionImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJConnection : wxConnection {
    #[fixed_stack_segment]
    fn advise(&self, item: @wxString, data: *u8, size: c_int, format: c_int) -> c_int {
        unsafe { ELJConnection_Advise(self.handle(), item.handle(), data, size, format) }
    }
    #[fixed_stack_segment]
    fn compress(&self, on: c_int) {
        unsafe { ELJConnection_Compress(self.handle(), on) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *u8, buffer: *u8, size: c_int) -> @ELJConnection {
        unsafe { @ELJConnectionImpl(ELJConnection_Create(_obj, buffer, size)) as @ELJConnection }
    }
    #[fixed_stack_segment]
    fn newDefault(&self) -> @ELJConnection {
        unsafe { @ELJConnectionImpl(ELJConnection_CreateDefault(self.handle())) as @ELJConnection }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJConnection_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn disconnect(&self) -> bool {
        unsafe { ELJConnection_Disconnect(self.handle()) }
    }
    #[fixed_stack_segment]
    fn execute(&self, data: @wxString, size: c_int, format: c_int) -> bool {
        unsafe { ELJConnection_Execute(self.handle(), data.handle(), size, format) }
    }
    #[fixed_stack_segment]
    fn poke(&self, item: @wxString, data: *u8, size: c_int, format: c_int) -> bool {
        unsafe { ELJConnection_Poke(self.handle(), item.handle(), data, size, format) }
    }
    #[fixed_stack_segment]
    fn request(&self, item: @wxString, size: @wxSize, format: c_int) -> *u8 {
        unsafe { ELJConnection_Request(self.handle(), item.handle(), size.handle(), format) }
    }
    #[fixed_stack_segment]
    fn setOnAdvise(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnAdvise(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnDisconnect(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnDisconnect(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnExecute(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnExecute(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnPoke(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnPoke(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnRequest(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnRequest(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnStartAdvise(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnStartAdvise(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnStopAdvise(&self, _fnc: *u8) {
        unsafe { ELJConnection_SetOnStopAdvise(self.handle(), _fnc) }
    }
    #[fixed_stack_segment]
    fn startAdvise(&self, item: @wxString) -> bool {
        unsafe { ELJConnection_StartAdvise(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn stopAdvise(&self, item: @wxString) -> bool {
        unsafe { ELJConnection_StopAdvise(self.handle(), item.handle()) }
    }
}

struct ELJDragDataObjectImpl(*u8);
impl ELJDragDataObject for ELJDragDataObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJDragDataObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(_obj: *u8, _fmt: @wxString, _func1: *u8, _func2: *u8, _func3: *u8) -> @ELJDragDataObject {
        unsafe { @ELJDragDataObjectImpl(ELJDragDataObject_Create(_obj, _fmt.handle(), _func1, _func2, _func3)) as @ELJDragDataObject }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.handle()) }
    }
}

struct ELJDropTargetImpl(*u8);
impl ELJDropTarget for ELJDropTargetImpl {}
impl wxDropTarget for ELJDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJDropTarget : wxDropTarget {
    #[fixed_stack_segment]
    fn new(_obj: *u8) -> @ELJDropTarget {
        unsafe { @ELJDropTargetImpl(ELJDropTarget_Create(_obj)) as @ELJDropTarget }
    }
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

struct ELJFileDropTargetImpl(*u8);
impl ELJFileDropTarget for ELJFileDropTargetImpl {}
impl wxFileDropTarget for ELJFileDropTargetImpl {}
impl wxDropTarget for ELJFileDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJFileDropTarget : wxFileDropTarget {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _func: *u8) -> @ELJFileDropTarget {
        unsafe { @ELJFileDropTargetImpl(ELJFileDropTarget_Create(_obj, _func)) as @ELJFileDropTarget }
    }
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

struct ELJGridTableImpl(*u8);
impl ELJGridTable for ELJGridTableImpl {}
impl wxGridTableBase for ELJGridTableImpl {}
impl wxObject for ELJGridTableImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJGridTable : wxGridTableBase {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _EifGetNumberRows: *u8, _EifGetNumberCols: *u8, _EifGetValue: *u8, _EifSetValue: *u8, _EifIsEmptyCell: *u8, _EifClear: *u8, _EifInsertRows: *u8, _EifAppendRows: *u8, _EifDeleteRows: *u8, _EifInsertCols: *u8, _EifAppendCols: *u8, _EifDeleteCols: *u8, _EifSetRowLabelValue: *u8, _EifSetColLabelValue: *u8, _EifGetRowLabelValue: *u8, _EifGetColLabelValue: *u8) -> @ELJGridTable {
        unsafe { @ELJGridTableImpl(ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)) as @ELJGridTable }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJGridTable_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getView(&self) -> @wxView {
        unsafe { @wxViewImpl(ELJGridTable_GetView(self.handle())) as @wxView }
    }
    #[fixed_stack_segment]
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *u8 {
        unsafe { ELJGridTable_SendTableMessage(self.handle(), id, val1, val2) }
    }
}

struct ELJLocaleImpl(*u8);
impl ELJLocale for ELJLocaleImpl {}
impl wxLocale for ELJLocaleImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJLocale : wxLocale {
}

struct ELJLogImpl(*u8);
impl ELJLog for ELJLogImpl {}
impl wxLog for ELJLogImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJLog : wxLog {
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: *wchar_t) {
        unsafe { ELJLog_AddTraceMask(self.handle(), str) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *u8, _fnc: *u8) -> @ELJLog {
        unsafe { @ELJLogImpl(ELJLog_Create(_obj, _fnc)) as @ELJLog }
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
    fn getActiveTarget() -> *u8 {
        unsafe { ELJLog_GetActiveTarget() }
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
    fn isAllowedTraceMask(&self, mask: @wxMask) -> bool {
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

struct ELJMessageParametersImpl(*u8);
impl ELJMessageParameters for ELJMessageParametersImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJMessageParameters {
    fn handle(&self) -> *u8;
    
}

struct ELJPlotCurveImpl(*u8);
impl ELJPlotCurve for ELJPlotCurveImpl {}
impl wxPlotCurve for ELJPlotCurveImpl {}
impl wxObject for ELJPlotCurveImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJPlotCurve : wxPlotCurve {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _str: *u8, _end: *u8, _y: *u8, offsetY: c_int, startY: c_double, endY: c_double) -> @ELJPlotCurve {
        unsafe { @ELJPlotCurveImpl(ELJPlotCurve_Create(_obj, _str, _end, _y, offsetY, startY, endY)) as @ELJPlotCurve }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJPlotCurve_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEndY(&self) -> c_double {
        unsafe { ELJPlotCurve_GetEndY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOffsetY(&self) -> c_int {
        unsafe { ELJPlotCurve_GetOffsetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStartY(&self) -> c_double {
        unsafe { ELJPlotCurve_GetStartY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEndY(&self, endY: c_double) {
        unsafe { ELJPlotCurve_SetEndY(self.handle(), endY) }
    }
    #[fixed_stack_segment]
    fn setOffsetY(&self, offsetY: c_int) {
        unsafe { ELJPlotCurve_SetOffsetY(self.handle(), offsetY) }
    }
    #[fixed_stack_segment]
    fn setPenNormal(&self, pen: @wxPen) {
        unsafe { ELJPlotCurve_SetPenNormal(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setPenSelected(&self, pen: @wxPen) {
        unsafe { ELJPlotCurve_SetPenSelected(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setStartY(&self, startY: c_double) {
        unsafe { ELJPlotCurve_SetStartY(self.handle(), startY) }
    }
}

struct ELJPreviewControlBarImpl(*u8);
impl ELJPreviewControlBar for ELJPreviewControlBarImpl {}
impl wxPreviewControlBar for ELJPreviewControlBarImpl {}
impl wxPanel for ELJPreviewControlBarImpl {}
impl wxWindow for ELJPreviewControlBarImpl {}
impl wxEvtHandler for ELJPreviewControlBarImpl {}
impl wxObject for ELJPreviewControlBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJPreviewControlBar : wxPreviewControlBar {
    #[fixed_stack_segment]
    fn new(preview: *u8, buttons: c_int, parent: @wxWindow, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewControlBar {
        unsafe { @ELJPreviewControlBarImpl(ELJPreviewControlBar_Create(preview, buttons, parent.handle(), title, x, y, w, h, style)) as @ELJPreviewControlBar }
    }
}

struct ELJPreviewFrameImpl(*u8);
impl ELJPreviewFrame for ELJPreviewFrameImpl {}
impl wxPreviewFrame for ELJPreviewFrameImpl {}
impl wxFrame for ELJPreviewFrameImpl {}
impl wxTopLevelWindow for ELJPreviewFrameImpl {}
impl wxWindow for ELJPreviewFrameImpl {}
impl wxEvtHandler for ELJPreviewFrameImpl {}
impl wxObject for ELJPreviewFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJPreviewFrame : wxPreviewFrame {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _init: *u8, _create_canvas: *u8, _create_toolbar: *u8, preview: *u8, parent: @wxWindow, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewFrame {
        unsafe { @ELJPreviewFrameImpl(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.handle(), title, x, y, w, h, style)) as @ELJPreviewFrame }
    }
    #[fixed_stack_segment]
    fn getControlBar(&self) -> *u8 {
        unsafe { ELJPreviewFrame_GetControlBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPreviewCanvas(&self) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvasImpl(ELJPreviewFrame_GetPreviewCanvas(self.handle())) as @wxPreviewCanvas }
    }
    #[fixed_stack_segment]
    fn getPrintPreview(&self) -> @wxPrintPreview {
        unsafe { @wxPrintPreviewImpl(ELJPreviewFrame_GetPrintPreview(self.handle())) as @wxPrintPreview }
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
    fn setPreviewCanvas(&self, obj: @wxPreviewCanvas) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintPreview(&self, obj: @wxPrintPreview) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.handle(), obj.handle()) }
    }
}

struct ELJServerImpl(*u8);
impl ELJServer for ELJServerImpl {}
impl wxServer for ELJServerImpl {}
impl wxServerBase for ELJServerImpl {}
impl wxObject for ELJServerImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJServer : wxServer {
    #[fixed_stack_segment]
    fn new(_eobj: *u8, _cnct: *u8) -> @ELJServer {
        unsafe { @ELJServerImpl(ELJServer_Create(_eobj, _cnct)) as @ELJServer }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJServer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, name: @wxString) -> c_int {
        unsafe { ELJServer_Initialize(self.handle(), name.handle()) }
    }
}

struct ELJTextDropTargetImpl(*u8);
impl ELJTextDropTarget for ELJTextDropTargetImpl {}
impl wxTextDropTarget for ELJTextDropTargetImpl {}
impl wxDropTarget for ELJTextDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJTextDropTarget : wxTextDropTarget {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _func: *u8) -> @ELJTextDropTarget {
        unsafe { @ELJTextDropTargetImpl(ELJTextDropTarget_Create(_obj, _func)) as @ELJTextDropTarget }
    }
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

struct ELJTextValidatorImpl(*u8);
impl ELJTextValidator for ELJTextValidatorImpl {}
impl wxTextValidator for ELJTextValidatorImpl {}
impl wxValidator for ELJTextValidatorImpl {}
impl wxEvtHandler for ELJTextValidatorImpl {}
impl wxObject for ELJTextValidatorImpl { pub fn handle(&self) -> *u8 { **self } }

trait ELJTextValidator : wxTextValidator {
    #[fixed_stack_segment]
    fn new(_obj: *u8, _fnc: *u8, _txt: *wchar_t, _stl: c_int) -> @ELJTextValidator {
        unsafe { @ELJTextValidatorImpl(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) as @ELJTextValidator }
    }
}

struct cbAntiflickerPluginImpl(*u8);
impl cbAntiflickerPlugin for cbAntiflickerPluginImpl {}
impl cbPluginBase for cbAntiflickerPluginImpl {}
impl wxEvtHandler for cbAntiflickerPluginImpl {}
impl wxObject for cbAntiflickerPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbAntiflickerPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbAntiflickerPlugin {
        unsafe { @cbAntiflickerPluginImpl(cbAntiflickerPlugin_Create(pPanel, paneMask)) as @cbAntiflickerPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbAntiflickerPlugin {
        unsafe { @cbAntiflickerPluginImpl(cbAntiflickerPlugin_CreateDefault()) as @cbAntiflickerPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbAntiflickerPlugin_Delete(self.handle()) }
    }
}

struct cbBarDragPluginImpl(*u8);
impl cbBarDragPlugin for cbBarDragPluginImpl {}
impl cbPluginBase for cbBarDragPluginImpl {}
impl wxEvtHandler for cbBarDragPluginImpl {}
impl wxObject for cbBarDragPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbBarDragPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbBarDragPlugin {
        unsafe { @cbBarDragPluginImpl(cbBarDragPlugin_Create(pPanel, paneMask)) as @cbBarDragPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbBarDragPlugin {
        unsafe { @cbBarDragPluginImpl(cbBarDragPlugin_CreateDefault()) as @cbBarDragPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarDragPlugin_Delete(self.handle()) }
    }
}

struct cbBarHintsPluginImpl(*u8);
impl cbBarHintsPlugin for cbBarHintsPluginImpl {}
impl cbPluginBase for cbBarHintsPluginImpl {}
impl wxEvtHandler for cbBarHintsPluginImpl {}
impl wxObject for cbBarHintsPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbBarHintsPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbBarHintsPlugin {
        unsafe { @cbBarHintsPluginImpl(cbBarHintsPlugin_Create(pPanel, paneMask)) as @cbBarHintsPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbBarHintsPlugin {
        unsafe { @cbBarHintsPluginImpl(cbBarHintsPlugin_CreateDefault()) as @cbBarHintsPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarHintsPlugin_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setGrooveCount(&self, nGrooves: c_int) {
        unsafe { cbBarHintsPlugin_SetGrooveCount(self.handle(), nGrooves) }
    }
}

struct cbBarInfoImpl(*u8);
impl cbBarInfo for cbBarInfoImpl {}
impl wxObject for cbBarInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbBarInfo : wxObject {
    #[fixed_stack_segment]
    fn new() -> @cbBarInfo {
        unsafe { @cbBarInfoImpl(cbBarInfo_Create()) as @cbBarInfo }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarInfo_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isExpanded(&self) -> bool {
        unsafe { cbBarInfo_IsExpanded(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isFixed(&self) -> bool {
        unsafe { cbBarInfo_IsFixed(self.handle()) }
    }
}

struct cbBarSpyImpl(*u8);
impl cbBarSpy for cbBarSpyImpl {}
impl wxEvtHandler for cbBarSpyImpl {}
impl wxObject for cbBarSpyImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbBarSpy : wxEvtHandler {
    #[fixed_stack_segment]
    fn new(pPanel: *u8) -> @cbBarSpy {
        unsafe { @cbBarSpyImpl(cbBarSpy_Create(pPanel)) as @cbBarSpy }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbBarSpy {
        unsafe { @cbBarSpyImpl(cbBarSpy_CreateDefault()) as @cbBarSpy }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarSpy_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @wxEvent) -> c_int {
        unsafe { cbBarSpy_ProcessEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn setBarWindow(&self, pWnd: *u8) {
        unsafe { cbBarSpy_SetBarWindow(self.handle(), pWnd) }
    }
}

struct cbCloseBoxImpl(*u8);
impl cbCloseBox for cbCloseBoxImpl {}
impl cbMiniButton for cbCloseBoxImpl {}
impl wxObject for cbCloseBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbCloseBox : cbMiniButton {
    #[fixed_stack_segment]
    fn new() -> @cbCloseBox {
        unsafe { @cbCloseBoxImpl(cbCloseBox_Create()) as @cbCloseBox }
    }
}

struct cbCollapseBoxImpl(*u8);
impl cbCollapseBox for cbCollapseBoxImpl {}
impl cbMiniButton for cbCollapseBoxImpl {}
impl wxObject for cbCollapseBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbCollapseBox : cbMiniButton {
    #[fixed_stack_segment]
    fn new() -> @cbCollapseBox {
        unsafe { @cbCollapseBoxImpl(cbCollapseBox_Create()) as @cbCollapseBox }
    }
}

struct cbCommonPanePropertiesImpl(*u8);
impl cbCommonPaneProperties for cbCommonPanePropertiesImpl {}
impl wxObject for cbCommonPanePropertiesImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbCommonPaneProperties : wxObject {
    #[fixed_stack_segment]
    fn assign(&self, _other: *u8) {
        unsafe { cbCommonPaneProperties_Assign(self.handle(), _other) }
    }
    #[fixed_stack_segment]
    fn barCollapseIconsOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_BarCollapseIconsOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn barDragHintsOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_BarDragHintsOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn barFloatingOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_BarFloatingOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn colProportionsOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_ColProportionsOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbCommonPaneProperties {
        unsafe { @cbCommonPanePropertiesImpl(cbCommonPaneProperties_CreateDefault()) as @cbCommonPaneProperties }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbCommonPaneProperties_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn exactDockPredictionOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_ExactDockPredictionOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn minCBarDim(&self, _w: *c_int, _h: *c_int) {
        unsafe { cbCommonPaneProperties_MinCBarDim(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn nonDestructFrictionOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_NonDestructFrictionOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn outOfPaneDragOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_OutOfPaneDragOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn realTimeUpdatesOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_RealTimeUpdatesOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn resizeHandleSize(&self) -> c_int {
        unsafe { cbCommonPaneProperties_ResizeHandleSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rowProportionsOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_RowProportionsOn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setBarCollapseIconsOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetBarCollapseIconsOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setBarDragHintsOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetBarDragHintsOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setBarFloatingOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetBarFloatingOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setColProportionsOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetColProportionsOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setExactDockPredictionOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetExactDockPredictionOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setMinCBarDim(&self, _w: c_int, _h: c_int) {
        unsafe { cbCommonPaneProperties_SetMinCBarDim(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn setNonDestructFrictionOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetNonDestructFrictionOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setOutOfPaneDragOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetOutOfPaneDragOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setRealTimeUpdatesOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetRealTimeUpdatesOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setResizeHandleSize(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetResizeHandleSize(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setRowProportionsOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetRowProportionsOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn setShow3DPaneBorderOn(&self, _val: c_int) {
        unsafe { cbCommonPaneProperties_SetShow3DPaneBorderOn(self.handle(), _val) }
    }
    #[fixed_stack_segment]
    fn show3DPaneBorderOn(&self) -> c_int {
        unsafe { cbCommonPaneProperties_Show3DPaneBorderOn(self.handle()) }
    }
}

struct cbCustomizeBarEventImpl(*u8);
impl cbCustomizeBarEvent for cbCustomizeBarEventImpl {}
impl cbPluginEvent for cbCustomizeBarEventImpl {}
impl wxEvent for cbCustomizeBarEventImpl {}
impl wxObject for cbCustomizeBarEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbCustomizeBarEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbCustomizeBarEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clickPos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbCustomizeBarEvent_ClickPos(self.handle(), _x, _y) }
    }
}

struct cbCustomizeLayoutEventImpl(*u8);
impl cbCustomizeLayoutEvent for cbCustomizeLayoutEventImpl {}
impl cbPluginEvent for cbCustomizeLayoutEventImpl {}
impl wxEvent for cbCustomizeLayoutEventImpl {}
impl wxObject for cbCustomizeLayoutEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbCustomizeLayoutEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn clickPos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbCustomizeLayoutEvent_ClickPos(self.handle(), _x, _y) }
    }
}

struct cbDimHandlerBaseImpl(*u8);
impl cbDimHandlerBase for cbDimHandlerBaseImpl {}
impl wxObject for cbDimHandlerBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDimHandlerBase : wxObject {
}

struct cbDimInfoImpl(*u8);
impl cbDimInfo for cbDimInfoImpl {}
impl wxObject for cbDimInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDimInfo : wxObject {
    #[fixed_stack_segment]
    fn assign(&self, other: *u8) {
        unsafe { cbDimInfo_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn new(x: c_int, y: c_int, isFixed: bool, gap: c_int, pDimHandler: *u8) -> @cbDimInfo {
        unsafe { @cbDimInfoImpl(cbDimInfo_Create(x, y, isFixed, gap, pDimHandler)) as @cbDimInfo }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbDimInfo {
        unsafe { @cbDimInfoImpl(cbDimInfo_CreateDefault()) as @cbDimInfo }
    }
    #[fixed_stack_segment]
    fn newWithHandler(&self, isFixed: bool) -> *u8 {
        unsafe { cbDimInfo_CreateWithHandler(self.handle(), isFixed) }
    }
    #[fixed_stack_segment]
    fn newWithInfo(dh_x: c_int, dh_y: c_int, dv_x: c_int, dv_y: c_int, f_x: c_int, f_y: c_int, isFixed: bool, horizGap: c_int, vertGap: c_int, pDimHandler: *u8) -> *u8 {
        unsafe { cbDimInfo_CreateWithInfo(dh_x, dh_y, dv_x, dv_y, f_x, f_y, isFixed, horizGap, vertGap, pDimHandler) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbDimInfo_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDimHandler(&self) -> *u8 {
        unsafe { cbDimInfo_GetDimHandler(self.handle()) }
    }
}

struct cbDockBoxImpl(*u8);
impl cbDockBox for cbDockBoxImpl {}
impl cbMiniButton for cbDockBoxImpl {}
impl wxObject for cbDockBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDockBox : cbMiniButton {
    #[fixed_stack_segment]
    fn new() -> @cbDockBox {
        unsafe { @cbDockBoxImpl(cbDockBox_Create()) as @cbDockBox }
    }
}

struct cbDockPaneImpl(*u8);
impl cbDockPane for cbDockPaneImpl {}
impl wxObject for cbDockPaneImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDockPane : wxObject {
    #[fixed_stack_segment]
    fn barPresent(&self, pBar: *u8) -> c_int {
        unsafe { cbDockPane_BarPresent(self.handle(), pBar) }
    }
    #[fixed_stack_segment]
    fn new(alignment: c_int, pPanel: *u8) -> @cbDockPane {
        unsafe { @cbDockPaneImpl(cbDockPane_Create(alignment, pPanel)) as @cbDockPane }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbDockPane {
        unsafe { @cbDockPaneImpl(cbDockPane_CreateDefault()) as @cbDockPane }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbDockPane_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> c_int {
        unsafe { cbDockPane_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBarInfoByWindow(&self, pBarWnd: *u8) -> *u8 {
        unsafe { cbDockPane_GetBarInfoByWindow(self.handle(), pBarWnd) }
    }
    #[fixed_stack_segment]
    fn getBarResizeRange(&self, pBar: *u8, from: *u8, till: *u8, forLeftHandle: c_int) {
        unsafe { cbDockPane_GetBarResizeRange(self.handle(), pBar, from, till, forLeftHandle) }
    }
    #[fixed_stack_segment]
    fn getDockingState(&self) -> c_int {
        unsafe { cbDockPane_GetDockingState(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstRow(&self) -> *u8 {
        unsafe { cbDockPane_GetFirstRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaneHeight(&self) -> c_int {
        unsafe { cbDockPane_GetPaneHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRealRect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbDockPane_GetRealRect(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn getRowList(&self, _ref: *u8) -> c_int {
        unsafe { cbDockPane_GetRowList(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn getRowResizeRange(&self, pRow: *u8, from: *u8, till: *u8, forUpperHandle: c_int) {
        unsafe { cbDockPane_GetRowResizeRange(self.handle(), pRow, from, till, forUpperHandle) }
    }
    #[fixed_stack_segment]
    fn hitTestPaneItems(&self, x: c_int, y: c_int, ppRow: *u8, ppBar: *u8) -> c_int {
        unsafe { cbDockPane_HitTestPaneItems(self.handle(), x, y, ppRow, ppBar) }
    }
    #[fixed_stack_segment]
    fn insertBarByCoord(&self, pBar: *u8, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { cbDockPane_InsertBarByCoord(self.handle(), pBar, x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn insertBarByInfo(&self, pBarInfo: *u8) {
        unsafe { cbDockPane_InsertBarByInfo(self.handle(), pBarInfo) }
    }
    #[fixed_stack_segment]
    fn insertBarToRow(&self, pBar: *u8, pIntoRow: *u8) {
        unsafe { cbDockPane_InsertBarToRow(self.handle(), pBar, pIntoRow) }
    }
    #[fixed_stack_segment]
    fn insertRow(&self, pRow: *u8, pBeforeRow: *u8) {
        unsafe { cbDockPane_InsertRow(self.handle(), pRow, pBeforeRow) }
    }
    #[fixed_stack_segment]
    fn isHorizontal(&self) -> bool {
        unsafe { cbDockPane_IsHorizontal(self.handle()) }
    }
    #[fixed_stack_segment]
    fn matchesMask(&self, paneMask: c_int) -> c_int {
        unsafe { cbDockPane_MatchesMask(self.handle(), paneMask) }
    }
    #[fixed_stack_segment]
    fn removeBar(&self, pBar: *u8) {
        unsafe { cbDockPane_RemoveBar(self.handle(), pBar) }
    }
    #[fixed_stack_segment]
    fn removeRow(&self, pRow: *u8) {
        unsafe { cbDockPane_RemoveRow(self.handle(), pRow) }
    }
    #[fixed_stack_segment]
    fn setBoundsInParent(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { cbDockPane_SetBoundsInParent(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, top: c_int, bottom: c_int, left: c_int, right: c_int) {
        unsafe { cbDockPane_SetMargins(self.handle(), top, bottom, left, right) }
    }
    #[fixed_stack_segment]
    fn setPaneWidth(&self, width: c_int) {
        unsafe { cbDockPane_SetPaneWidth(self.handle(), width) }
    }
}

struct cbDrawBarDecorEventImpl(*u8);
impl cbDrawBarDecorEvent for cbDrawBarDecorEventImpl {}
impl cbPluginEvent for cbDrawBarDecorEventImpl {}
impl wxEvent for cbDrawBarDecorEventImpl {}
impl wxObject for cbDrawBarDecorEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawBarDecorEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbDrawBarDecorEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn boundsInParent(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbDrawBarDecorEvent_BoundsInParent(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawBarDecorEvent_Dc(self.handle()) }
    }
}

struct cbDrawBarHandlesEventImpl(*u8);
impl cbDrawBarHandlesEvent for cbDrawBarHandlesEventImpl {}
impl cbPluginEvent for cbDrawBarHandlesEventImpl {}
impl wxEvent for cbDrawBarHandlesEventImpl {}
impl wxObject for cbDrawBarHandlesEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawBarHandlesEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbDrawBarHandlesEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawBarHandlesEvent_Dc(self.handle()) }
    }
}

struct cbDrawHintRectEventImpl(*u8);
impl cbDrawHintRectEvent for cbDrawHintRectEventImpl {}
impl cbPluginEvent for cbDrawHintRectEventImpl {}
impl wxEvent for cbDrawHintRectEventImpl {}
impl wxObject for cbDrawHintRectEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawHintRectEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn eraseRect(&self) -> c_int {
        unsafe { cbDrawHintRectEvent_EraseRect(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isInClient(&self) -> bool {
        unsafe { cbDrawHintRectEvent_IsInClient(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lastTime(&self) -> c_int {
        unsafe { cbDrawHintRectEvent_LastTime(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbDrawHintRectEvent_Rect(self.handle(), _x, _y, _w, _h) }
    }
}

struct cbDrawPaneBkGroundEventImpl(*u8);
impl cbDrawPaneBkGroundEvent for cbDrawPaneBkGroundEventImpl {}
impl cbPluginEvent for cbDrawPaneBkGroundEventImpl {}
impl wxEvent for cbDrawPaneBkGroundEventImpl {}
impl wxObject for cbDrawPaneBkGroundEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawPaneBkGroundEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawPaneBkGroundEvent_Dc(self.handle()) }
    }
}

struct cbDrawPaneDecorEventImpl(*u8);
impl cbDrawPaneDecorEvent for cbDrawPaneDecorEventImpl {}
impl cbPluginEvent for cbDrawPaneDecorEventImpl {}
impl wxEvent for cbDrawPaneDecorEventImpl {}
impl wxObject for cbDrawPaneDecorEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawPaneDecorEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawPaneDecorEvent_Dc(self.handle()) }
    }
}

struct cbDrawRowBkGroundEventImpl(*u8);
impl cbDrawRowBkGroundEvent for cbDrawRowBkGroundEventImpl {}
impl cbPluginEvent for cbDrawRowBkGroundEventImpl {}
impl wxEvent for cbDrawRowBkGroundEventImpl {}
impl wxObject for cbDrawRowBkGroundEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawRowBkGroundEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawRowBkGroundEvent_Dc(self.handle()) }
    }
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbDrawRowBkGroundEvent_Row(self.handle()) }
    }
}

struct cbDrawRowDecorEventImpl(*u8);
impl cbDrawRowDecorEvent for cbDrawRowDecorEventImpl {}
impl cbPluginEvent for cbDrawRowDecorEventImpl {}
impl wxEvent for cbDrawRowDecorEventImpl {}
impl wxObject for cbDrawRowDecorEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawRowDecorEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawRowDecorEvent_Dc(self.handle()) }
    }
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbDrawRowDecorEvent_Row(self.handle()) }
    }
}

struct cbDrawRowHandlesEventImpl(*u8);
impl cbDrawRowHandlesEvent for cbDrawRowHandlesEventImpl {}
impl cbPluginEvent for cbDrawRowHandlesEventImpl {}
impl wxEvent for cbDrawRowHandlesEventImpl {}
impl wxObject for cbDrawRowHandlesEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDrawRowHandlesEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawRowHandlesEvent_Dc(self.handle()) }
    }
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbDrawRowHandlesEvent_Row(self.handle()) }
    }
}

struct cbDynToolBarDimHandlerImpl(*u8);
impl cbDynToolBarDimHandler for cbDynToolBarDimHandlerImpl {}
impl cbDimHandlerBase for cbDynToolBarDimHandlerImpl {}
impl wxObject for cbDynToolBarDimHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbDynToolBarDimHandler : cbDimHandlerBase {
    #[fixed_stack_segment]
    fn new() -> @cbDynToolBarDimHandler {
        unsafe { @cbDynToolBarDimHandlerImpl(cbDynToolBarDimHandler_Create()) as @cbDynToolBarDimHandler }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbDynToolBarDimHandler_Delete(self.handle()) }
    }
}

struct cbFinishDrawInAreaEventImpl(*u8);
impl cbFinishDrawInAreaEvent for cbFinishDrawInAreaEventImpl {}
impl cbPluginEvent for cbFinishDrawInAreaEventImpl {}
impl wxEvent for cbFinishDrawInAreaEventImpl {}
impl wxObject for cbFinishDrawInAreaEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbFinishDrawInAreaEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbFinishDrawInAreaEvent_Area(self.handle(), _x, _y, _w, _h) }
    }
}

struct cbFloatedBarWindowImpl(*u8);
impl cbFloatedBarWindow for cbFloatedBarWindowImpl {}
impl wxToolWindow for cbFloatedBarWindowImpl {}
impl wxFrame for cbFloatedBarWindowImpl {}
impl wxTopLevelWindow for cbFloatedBarWindowImpl {}
impl wxWindow for cbFloatedBarWindowImpl {}
impl wxEvtHandler for cbFloatedBarWindowImpl {}
impl wxObject for cbFloatedBarWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbFloatedBarWindow : wxToolWindow {
    #[fixed_stack_segment]
    fn new(_obj: *u8) -> @cbFloatedBarWindow {
        unsafe { @cbFloatedBarWindowImpl(cbFloatedBarWindow_Create(_obj)) as @cbFloatedBarWindow }
    }
    #[fixed_stack_segment]
    fn getBar(&self) -> *u8 {
        unsafe { cbFloatedBarWindow_GetBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn positionFloatedWnd(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { cbFloatedBarWindow_PositionFloatedWnd(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn setBar(&self, _bar: *u8) {
        unsafe { cbFloatedBarWindow_SetBar(self.handle(), _bar) }
    }
    #[fixed_stack_segment]
    fn setLayout(&self, _layout: *u8) {
        unsafe { cbFloatedBarWindow_SetLayout(self.handle(), _layout) }
    }
}

struct cbGCUpdatesMgrImpl(*u8);
impl cbGCUpdatesMgr for cbGCUpdatesMgrImpl {}
impl cbSimpleUpdatesMgr for cbGCUpdatesMgrImpl {}
impl cbUpdatesManagerBase for cbGCUpdatesMgrImpl {}
impl wxObject for cbGCUpdatesMgrImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbGCUpdatesMgr : cbSimpleUpdatesMgr {
    #[fixed_stack_segment]
    fn new(pPanel: *u8) -> @cbGCUpdatesMgr {
        unsafe { @cbGCUpdatesMgrImpl(cbGCUpdatesMgr_Create(pPanel)) as @cbGCUpdatesMgr }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbGCUpdatesMgr {
        unsafe { @cbGCUpdatesMgrImpl(cbGCUpdatesMgr_CreateDefault()) as @cbGCUpdatesMgr }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbGCUpdatesMgr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn updateNow(&self) {
        unsafe { cbGCUpdatesMgr_UpdateNow(self.handle()) }
    }
}

struct cbHintAnimationPluginImpl(*u8);
impl cbHintAnimationPlugin for cbHintAnimationPluginImpl {}
impl cbPluginBase for cbHintAnimationPluginImpl {}
impl wxEvtHandler for cbHintAnimationPluginImpl {}
impl wxObject for cbHintAnimationPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbHintAnimationPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbHintAnimationPlugin {
        unsafe { @cbHintAnimationPluginImpl(cbHintAnimationPlugin_Create(pPanel, paneMask)) as @cbHintAnimationPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbHintAnimationPlugin {
        unsafe { @cbHintAnimationPluginImpl(cbHintAnimationPlugin_CreateDefault()) as @cbHintAnimationPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbHintAnimationPlugin_Delete(self.handle()) }
    }
}

struct cbInsertBarEventImpl(*u8);
impl cbInsertBarEvent for cbInsertBarEventImpl {}
impl cbPluginEvent for cbInsertBarEventImpl {}
impl wxEvent for cbInsertBarEventImpl {}
impl wxObject for cbInsertBarEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbInsertBarEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbInsertBarEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbInsertBarEvent_Row(self.handle()) }
    }
}

struct cbLayoutRowEventImpl(*u8);
impl cbLayoutRowEvent for cbLayoutRowEventImpl {}
impl cbPluginEvent for cbLayoutRowEventImpl {}
impl wxEvent for cbLayoutRowEventImpl {}
impl wxObject for cbLayoutRowEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbLayoutRowEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbLayoutRowEvent_Row(self.handle()) }
    }
}

struct cbLeftDClickEventImpl(*u8);
impl cbLeftDClickEvent for cbLeftDClickEventImpl {}
impl cbPluginEvent for cbLeftDClickEventImpl {}
impl wxEvent for cbLeftDClickEventImpl {}
impl wxObject for cbLeftDClickEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbLeftDClickEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftDClickEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbLeftDownEventImpl(*u8);
impl cbLeftDownEvent for cbLeftDownEventImpl {}
impl cbPluginEvent for cbLeftDownEventImpl {}
impl wxEvent for cbLeftDownEventImpl {}
impl wxObject for cbLeftDownEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbLeftDownEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftDownEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbLeftUpEventImpl(*u8);
impl cbLeftUpEvent for cbLeftUpEventImpl {}
impl cbPluginEvent for cbLeftUpEventImpl {}
impl wxEvent for cbLeftUpEventImpl {}
impl wxObject for cbLeftUpEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbLeftUpEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftUpEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbMiniButtonImpl(*u8);
impl cbMiniButton for cbMiniButtonImpl {}
impl wxObject for cbMiniButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbMiniButton : wxObject {
    #[fixed_stack_segment]
    fn new() -> @cbMiniButton {
        unsafe { @cbMiniButtonImpl(cbMiniButton_Create()) as @cbMiniButton }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbMiniButton_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn dim(&self, _w: *c_int, _h: *c_int) {
        unsafe { cbMiniButton_Dim(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn dragStarted(&self) -> c_int {
        unsafe { cbMiniButton_DragStarted(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) {
        unsafe { cbMiniButton_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn enabled(&self) -> c_int {
        unsafe { cbMiniButton_Enabled(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int) -> c_int {
        unsafe { cbMiniButton_HitTest(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn isPressed(&self) -> bool {
        unsafe { cbMiniButton_IsPressed(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layout(&self) -> *u8 {
        unsafe { cbMiniButton_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn pane(&self) -> *u8 {
        unsafe { cbMiniButton_Pane(self.handle()) }
    }
    #[fixed_stack_segment]
    fn plugin(&self) -> *u8 {
        unsafe { cbMiniButton_Plugin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbMiniButton_Pos(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn pressed(&self) -> c_int {
        unsafe { cbMiniButton_Pressed(self.handle()) }
    }
    #[fixed_stack_segment]
    fn refresh(&self) {
        unsafe { cbMiniButton_Refresh(self.handle()) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { cbMiniButton_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPos(&self, x: c_int, y: c_int) {
        unsafe { cbMiniButton_SetPos(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn visible(&self) -> c_int {
        unsafe { cbMiniButton_Visible(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wasClicked(&self) -> c_int {
        unsafe { cbMiniButton_WasClicked(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wnd(&self) -> *u8 {
        unsafe { cbMiniButton_Wnd(self.handle()) }
    }
}

struct cbMotionEventImpl(*u8);
impl cbMotionEvent for cbMotionEventImpl {}
impl cbPluginEvent for cbMotionEventImpl {}
impl wxEvent for cbMotionEventImpl {}
impl wxObject for cbMotionEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbMotionEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbMotionEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbPaneDrawPluginImpl(*u8);
impl cbPaneDrawPlugin for cbPaneDrawPluginImpl {}
impl cbPluginBase for cbPaneDrawPluginImpl {}
impl wxEvtHandler for cbPaneDrawPluginImpl {}
impl wxObject for cbPaneDrawPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbPaneDrawPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbPaneDrawPlugin {
        unsafe { @cbPaneDrawPluginImpl(cbPaneDrawPlugin_Create(pPanel, paneMask)) as @cbPaneDrawPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbPaneDrawPlugin {
        unsafe { @cbPaneDrawPluginImpl(cbPaneDrawPlugin_CreateDefault()) as @cbPaneDrawPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbPaneDrawPlugin_Delete(self.handle()) }
    }
}

struct cbPluginBaseImpl(*u8);
impl cbPluginBase for cbPluginBaseImpl {}
impl wxEvtHandler for cbPluginBaseImpl {}
impl wxObject for cbPluginBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbPluginBase : wxEvtHandler {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbPluginBase_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaneMask(&self) -> c_int {
        unsafe { cbPluginBase_GetPaneMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isReady(&self) -> bool {
        unsafe { cbPluginBase_IsReady(self.handle()) }
    }
    #[fixed_stack_segment]
    fn plugin(_swt: c_int) -> *u8 {
        unsafe { cbPluginBase_Plugin(_swt) }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @wxEvent) -> c_int {
        unsafe { cbPluginBase_ProcessEvent(self.handle(), event.handle()) }
    }
}

struct cbPluginEventImpl(*u8);
impl cbPluginEvent for cbPluginEventImpl {}
impl wxEvent for cbPluginEventImpl {}
impl wxObject for cbPluginEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbPluginEvent : wxEvent {
    #[fixed_stack_segment]
    fn pane(&self) -> *u8 {
        unsafe { cbPluginEvent_Pane(self.handle()) }
    }
}

struct cbRemoveBarEventImpl(*u8);
impl cbRemoveBarEvent for cbRemoveBarEventImpl {}
impl cbPluginEvent for cbRemoveBarEventImpl {}
impl wxEvent for cbRemoveBarEventImpl {}
impl wxObject for cbRemoveBarEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbRemoveBarEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbRemoveBarEvent_Bar(self.handle()) }
    }
}

struct cbResizeBarEventImpl(*u8);
impl cbResizeBarEvent for cbResizeBarEventImpl {}
impl cbPluginEvent for cbResizeBarEventImpl {}
impl wxEvent for cbResizeBarEventImpl {}
impl wxObject for cbResizeBarEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbResizeBarEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbResizeBarEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbResizeBarEvent_Row(self.handle()) }
    }
}

struct cbResizeRowEventImpl(*u8);
impl cbResizeRowEvent for cbResizeRowEventImpl {}
impl cbPluginEvent for cbResizeRowEventImpl {}
impl wxEvent for cbResizeRowEventImpl {}
impl wxObject for cbResizeRowEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbResizeRowEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn forUpperHandle(&self) -> c_int {
        unsafe { cbResizeRowEvent_ForUpperHandle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn handleOfs(&self) -> c_int {
        unsafe { cbResizeRowEvent_HandleOfs(self.handle()) }
    }
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbResizeRowEvent_Row(self.handle()) }
    }
}

struct cbRightDownEventImpl(*u8);
impl cbRightDownEvent for cbRightDownEventImpl {}
impl cbPluginEvent for cbRightDownEventImpl {}
impl wxEvent for cbRightDownEventImpl {}
impl wxObject for cbRightDownEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbRightDownEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbRightDownEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbRightUpEventImpl(*u8);
impl cbRightUpEvent for cbRightUpEventImpl {}
impl cbPluginEvent for cbRightUpEventImpl {}
impl wxEvent for cbRightUpEventImpl {}
impl wxObject for cbRightUpEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbRightUpEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbRightUpEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbRowDragPluginImpl(*u8);
impl cbRowDragPlugin for cbRowDragPluginImpl {}
impl cbPluginBase for cbRowDragPluginImpl {}
impl wxEvtHandler for cbRowDragPluginImpl {}
impl wxObject for cbRowDragPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbRowDragPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbRowDragPlugin {
        unsafe { @cbRowDragPluginImpl(cbRowDragPlugin_Create(pPanel, paneMask)) as @cbRowDragPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbRowDragPlugin {
        unsafe { @cbRowDragPluginImpl(cbRowDragPlugin_CreateDefault()) as @cbRowDragPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbRowDragPlugin_Delete(self.handle()) }
    }
}

struct cbRowInfoImpl(*u8);
impl cbRowInfo for cbRowInfoImpl {}
impl wxObject for cbRowInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbRowInfo : wxObject {
    #[fixed_stack_segment]
    fn new() -> @cbRowInfo {
        unsafe { @cbRowInfoImpl(cbRowInfo_Create()) as @cbRowInfo }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbRowInfo_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstBar(&self) -> *u8 {
        unsafe { cbRowInfo_GetFirstBar(self.handle()) }
    }
}

struct cbRowLayoutPluginImpl(*u8);
impl cbRowLayoutPlugin for cbRowLayoutPluginImpl {}
impl cbPluginBase for cbRowLayoutPluginImpl {}
impl wxEvtHandler for cbRowLayoutPluginImpl {}
impl wxObject for cbRowLayoutPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbRowLayoutPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbRowLayoutPlugin {
        unsafe { @cbRowLayoutPluginImpl(cbRowLayoutPlugin_Create(pPanel, paneMask)) as @cbRowLayoutPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbRowLayoutPlugin {
        unsafe { @cbRowLayoutPluginImpl(cbRowLayoutPlugin_CreateDefault()) as @cbRowLayoutPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbRowLayoutPlugin_Delete(self.handle()) }
    }
}

struct cbSimpleCustomizationPluginImpl(*u8);
impl cbSimpleCustomizationPlugin for cbSimpleCustomizationPluginImpl {}
impl cbPluginBase for cbSimpleCustomizationPluginImpl {}
impl wxEvtHandler for cbSimpleCustomizationPluginImpl {}
impl wxObject for cbSimpleCustomizationPluginImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbSimpleCustomizationPlugin : cbPluginBase {
    #[fixed_stack_segment]
    fn new(pPanel: *u8, paneMask: c_int) -> @cbSimpleCustomizationPlugin {
        unsafe { @cbSimpleCustomizationPluginImpl(cbSimpleCustomizationPlugin_Create(pPanel, paneMask)) as @cbSimpleCustomizationPlugin }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbSimpleCustomizationPlugin {
        unsafe { @cbSimpleCustomizationPluginImpl(cbSimpleCustomizationPlugin_CreateDefault()) as @cbSimpleCustomizationPlugin }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbSimpleCustomizationPlugin_Delete(self.handle()) }
    }
}

struct cbSimpleUpdatesMgrImpl(*u8);
impl cbSimpleUpdatesMgr for cbSimpleUpdatesMgrImpl {}
impl cbUpdatesManagerBase for cbSimpleUpdatesMgrImpl {}
impl wxObject for cbSimpleUpdatesMgrImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbSimpleUpdatesMgr : cbUpdatesManagerBase {
}

struct cbSizeBarWndEventImpl(*u8);
impl cbSizeBarWndEvent for cbSizeBarWndEventImpl {}
impl cbPluginEvent for cbSizeBarWndEventImpl {}
impl wxEvent for cbSizeBarWndEventImpl {}
impl wxObject for cbSizeBarWndEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbSizeBarWndEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbSizeBarWndEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn boundsInParent(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbSizeBarWndEvent_BoundsInParent(self.handle(), _x, _y, _w, _h) }
    }
}

struct cbStartBarDraggingEventImpl(*u8);
impl cbStartBarDraggingEvent for cbStartBarDraggingEventImpl {}
impl cbPluginEvent for cbStartBarDraggingEventImpl {}
impl wxEvent for cbStartBarDraggingEventImpl {}
impl wxObject for cbStartBarDraggingEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbStartBarDraggingEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbStartBarDraggingEvent_Bar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbStartBarDraggingEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbStartDrawInAreaEventImpl(*u8);
impl cbStartDrawInAreaEvent for cbStartDrawInAreaEventImpl {}
impl cbPluginEvent for cbStartDrawInAreaEventImpl {}
impl wxEvent for cbStartDrawInAreaEventImpl {}
impl wxObject for cbStartDrawInAreaEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbStartDrawInAreaEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbStartDrawInAreaEvent_Area(self.handle(), _x, _y, _w, _h) }
    }
}

struct cbUpdatesManagerBaseImpl(*u8);
impl cbUpdatesManagerBase for cbUpdatesManagerBaseImpl {}
impl wxObject for cbUpdatesManagerBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait cbUpdatesManagerBase : wxObject {
}

struct wxAcceleratorEntryImpl(*u8);
impl wxAcceleratorEntry for wxAcceleratorEntryImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxAcceleratorEntry {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> @wxAcceleratorEntry {
        unsafe { @wxAcceleratorEntryImpl(wxAcceleratorEntry_Create(flags, keyCode, cmd)) as @wxAcceleratorEntry }
    }
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

struct wxAcceleratorTableImpl(*u8);
impl wxAcceleratorTable for wxAcceleratorTableImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxAcceleratorTable {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(n: c_int, entries: *u8) -> @wxAcceleratorTable {
        unsafe { @wxAcceleratorTableImpl(wxAcceleratorTable_Create(n, entries)) as @wxAcceleratorTable }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.handle()) }
    }
}

struct wxActivateEventImpl(*u8);
impl wxActivateEvent for wxActivateEventImpl {}
impl wxEvent for wxActivateEventImpl {}
impl wxObject for wxActivateEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxActivateEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxActivateEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getActive(&self) -> bool {
        unsafe { wxActivateEvent_GetActive(self.handle()) }
    }
}

struct wxAppImpl(*u8);
impl wxApp for wxAppImpl {}
impl wxEvtHandler for wxAppImpl {}
impl wxObject for wxAppImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxApp : wxEvtHandler {
}

struct wxArrayImpl(*u8);
impl wxArray for wxArrayImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxArray {
    fn handle(&self) -> *u8;
    
}

struct wxArrayStringImpl(*u8);
impl wxArrayString for wxArrayStringImpl {}
impl wxArray for wxArrayStringImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxArrayString : wxArray {
}

struct wxArtProviderImpl(*u8);
impl wxArtProvider for wxArtProviderImpl {}
impl wxObject for wxArtProviderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxArtProvider : wxObject {
}

struct wxAutoBufferedPaintDCImpl(*u8);
impl wxAutoBufferedPaintDC for wxAutoBufferedPaintDCImpl {}
impl wxDC for wxAutoBufferedPaintDCImpl {}
impl wxObject for wxAutoBufferedPaintDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxAutoBufferedPaintDC : wxDC {
    #[fixed_stack_segment]
    fn new(window: @wxWindow) -> @wxAutoBufferedPaintDC {
        unsafe { @wxAutoBufferedPaintDCImpl(wxAutoBufferedPaintDC_Create(window.handle())) as @wxAutoBufferedPaintDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAutoBufferedPaintDC_Delete(self.handle()) }
    }
}

struct wxAutomationObjectImpl(*u8);
impl wxAutomationObject for wxAutomationObjectImpl {}
impl wxObject for wxAutomationObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxAutomationObject : wxObject {
}

struct wxBitmapImpl(*u8);
impl wxBitmap for wxBitmapImpl {}
impl wxGDIObject for wxBitmapImpl {}
impl wxObject for wxBitmapImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBitmap : wxGDIObject {
    #[fixed_stack_segment]
    fn addHandler(handler: @wxEvtHandler) {
        unsafe { wxBitmap_AddHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    fn new(_data: *u8, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmapImpl(wxBitmap_Create(_data, _type, _width, _height, _depth)) as @wxBitmap }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxBitmap {
        unsafe { @wxBitmapImpl(wxBitmap_CreateDefault()) as @wxBitmap }
    }
    #[fixed_stack_segment]
    fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmapImpl(wxBitmap_CreateEmpty(_width, _height, _depth)) as @wxBitmap }
    }
    #[fixed_stack_segment]
    fn newFromXPM(&self) -> @wxBitmap {
        unsafe { @wxBitmapImpl(wxBitmap_CreateFromXPM(self.handle())) as @wxBitmap }
    }
    #[fixed_stack_segment]
    fn newLoad(name: @wxString, type_: c_int) -> @wxBitmap {
        unsafe { @wxBitmapImpl(wxBitmap_CreateLoad(name.handle(), type_)) as @wxBitmap }
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
    fn findHandlerByName(name: @wxString) -> *u8 {
        unsafe { wxBitmap_FindHandlerByName(name.handle()) }
    }
    #[fixed_stack_segment]
    fn findHandlerByType(type_: c_int) -> *u8 {
        unsafe { wxBitmap_FindHandlerByType(type_) }
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
    fn getMask(&self) -> @wxMask {
        unsafe { @wxMaskImpl(wxBitmap_GetMask(self.handle())) as @wxMask }
    }
    #[fixed_stack_segment]
    fn getSubBitmap(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: @wxBitmap) {
        unsafe { wxBitmap_GetSubBitmap(self.handle(), x, y, w, h, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn initStandardHandlers() {
        unsafe { wxBitmap_InitStandardHandlers() }
    }
    #[fixed_stack_segment]
    fn insertHandler(handler: @wxEvtHandler) {
        unsafe { wxBitmap_InsertHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, name: @wxString, type_: c_int) -> c_int {
        unsafe { wxBitmap_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBitmap_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn removeHandler(name: @wxString) -> bool {
        unsafe { wxBitmap_RemoveHandler(name.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @wxString, type_: c_int, cmap: @wxPalette) -> c_int {
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
    fn setMask(&self, mask: @wxMask) {
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
    #[fixed_stack_segment]
    fn newFromImage(image: @wxImage, depth: c_int) -> @wxBitmap {
        unsafe { @wxBitmapImpl(wxBitmap_CreateFromImage(image.handle(), depth)) as @wxBitmap }
    }
}

struct wxBitmapButtonImpl(*u8);
impl wxBitmapButton for wxBitmapButtonImpl {}
impl wxButton for wxBitmapButtonImpl {}
impl wxControl for wxBitmapButtonImpl {}
impl wxWindow for wxBitmapButtonImpl {}
impl wxEvtHandler for wxBitmapButtonImpl {}
impl wxObject for wxBitmapButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBitmapButton : wxButton {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _bmp: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxBitmapButton {
        unsafe { @wxBitmapButtonImpl(wxBitmapButton_Create(_prt.handle(), _id, _bmp.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxBitmapButton }
    }
    #[fixed_stack_segment]
    fn getBitmapDisabled(&self, _ref: @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapFocus(&self, _ref: @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapLabel(&self, _ref: @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapSelected(&self, _ref: @wxBitmap) {
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
    fn setBitmapDisabled(&self, disabled: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.handle(), disabled.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapFocus(&self, focus: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.handle(), focus.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel(&self, bitmap: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapSelected(&self, sel: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.handle(), sel.handle()) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.handle(), x, y) }
    }
}

struct wxBitmapToggleButtonImpl(*u8);
impl wxBitmapToggleButton for wxBitmapToggleButtonImpl {}
impl wxToggleButton for wxBitmapToggleButtonImpl {}
impl wxControl for wxBitmapToggleButtonImpl {}
impl wxWindow for wxBitmapToggleButtonImpl {}
impl wxEvtHandler for wxBitmapToggleButtonImpl {}
impl wxObject for wxBitmapToggleButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBitmapToggleButton : wxToggleButton {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, _bmp: @wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxBitmapToggleButton {
        unsafe { @wxBitmapToggleButtonImpl(wxBitmapToggleButton_Create(parent.handle(), id, _bmp.handle(), x, y, w, h, style)) as @wxBitmapToggleButton }
    }
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
    fn setBitmapLabel(&self, _bmp: @wxBitmap) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.handle(), _bmp.handle()) }
    }
}

struct wxBitmapDataObjectImpl(*u8);
impl wxBitmapDataObject for wxBitmapDataObjectImpl {}
impl wxDataObjectSimple for wxBitmapDataObjectImpl {}
impl wxDataObject for wxBitmapDataObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBitmapDataObject : wxDataObjectSimple {
}

struct wxBitmapHandlerImpl(*u8);
impl wxBitmapHandler for wxBitmapHandlerImpl {}
impl wxObject for wxBitmapHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBitmapHandler : wxObject {
}

struct wxBoxSizerImpl(*u8);
impl wxBoxSizer for wxBoxSizerImpl {}
impl wxSizer for wxBoxSizerImpl {}
impl wxObject for wxBoxSizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBoxSizer : wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxBoxSizer_CalcMin(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn new(orient: c_int) -> @wxBoxSizer {
        unsafe { @wxBoxSizerImpl(wxBoxSizer_Create(orient)) as @wxBoxSizer }
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

struct wxBrushImpl(*u8);
impl wxBrush for wxBrushImpl {}
impl wxGDIObject for wxBrushImpl {}
impl wxObject for wxBrushImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBrush : wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, brush: @wxBrush) {
        unsafe { wxBrush_Assign(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxBrush {
        unsafe { @wxBrushImpl(wxBrush_CreateDefault()) as @wxBrush }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(bitmap: @wxBitmap) -> @wxBrush {
        unsafe { @wxBrushImpl(wxBrush_CreateFromBitmap(bitmap.handle())) as @wxBrush }
    }
    #[fixed_stack_segment]
    fn newFromColour(col: @wxColour, style: c_int) -> @wxBrush {
        unsafe { @wxBrushImpl(wxBrush_CreateFromColour(col.handle(), style)) as @wxBrush }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: c_int) -> @wxBrush {
        unsafe { @wxBrushImpl(wxBrush_CreateFromStock(id)) as @wxBrush }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBrush_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @wxColour) {
        unsafe { wxBrush_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStipple(&self, _ref: @wxBitmap) {
        unsafe { wxBrush_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, brush: @wxBrush) -> bool {
        unsafe { wxBrush_IsEqual(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBrush_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, col: @wxColour) {
        unsafe { wxBrush_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxBrush_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    fn setStipple(&self, stipple: @wxBitmap) {
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

struct wxBrushListImpl(*u8);
impl wxBrushList for wxBrushListImpl {}
impl wxList for wxBrushListImpl {}
impl wxObject for wxBrushListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBrushList : wxList {
}

struct wxBufferedDCImpl(*u8);
impl wxBufferedDC for wxBufferedDCImpl {}
impl wxDC for wxBufferedDCImpl {}
impl wxObject for wxBufferedDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBufferedDC : wxDC {
    #[fixed_stack_segment]
    fn newByDCAndSize(dc: @wxDC, width: c_int, hight: c_int, style: c_int) -> @wxBufferedDC {
        unsafe { @wxBufferedDCImpl(wxBufferedDC_CreateByDCAndSize(dc.handle(), width, hight, style)) as @wxBufferedDC }
    }
    #[fixed_stack_segment]
    fn newByDCAndBitmap(dc: @wxDC, bitmap: @wxBitmap, style: c_int) -> @wxBufferedDC {
        unsafe { @wxBufferedDCImpl(wxBufferedDC_CreateByDCAndBitmap(dc.handle(), bitmap.handle(), style)) as @wxBufferedDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedDC_Delete(self.handle()) }
    }
}

struct wxBufferedPaintDCImpl(*u8);
impl wxBufferedPaintDC for wxBufferedPaintDCImpl {}
impl wxDC for wxBufferedPaintDCImpl {}
impl wxObject for wxBufferedPaintDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBufferedPaintDC : wxDC {
    #[fixed_stack_segment]
    fn new(window: @wxWindow, style: c_int) -> @wxBufferedPaintDC {
        unsafe { @wxBufferedPaintDCImpl(wxBufferedPaintDC_Create(window.handle(), style)) as @wxBufferedPaintDC }
    }
    #[fixed_stack_segment]
    fn newWithBitmap(window: @wxWindow, bitmap: @wxBitmap, style: c_int) -> @wxBufferedPaintDC {
        unsafe { @wxBufferedPaintDCImpl(wxBufferedPaintDC_CreateWithBitmap(window.handle(), bitmap.handle(), style)) as @wxBufferedPaintDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedPaintDC_Delete(self.handle()) }
    }
}

struct wxBufferedInputStreamImpl(*u8);
impl wxBufferedInputStream for wxBufferedInputStreamImpl {}
impl wxFilterInputStream for wxBufferedInputStreamImpl {}
impl wxInputStream for wxBufferedInputStreamImpl {}
impl wxStreamBase for wxBufferedInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBufferedInputStream : wxFilterInputStream {
}

struct wxBufferedOutputStreamImpl(*u8);
impl wxBufferedOutputStream for wxBufferedOutputStreamImpl {}
impl wxFilterOutputStream for wxBufferedOutputStreamImpl {}
impl wxOutputStream for wxBufferedOutputStreamImpl {}
impl wxStreamBase for wxBufferedOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBufferedOutputStream : wxFilterOutputStream {
}

struct wxBusyCursorImpl(*u8);
impl wxBusyCursor for wxBusyCursorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBusyCursor {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxBusyCursor {
        unsafe { @wxBusyCursorImpl(wxBusyCursor_Create()) as @wxBusyCursor }
    }
    #[fixed_stack_segment]
    fn newWithCursor(&self) -> *u8 {
        unsafe { wxBusyCursor_CreateWithCursor(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.handle()) }
    }
}

struct wxBusyInfoImpl(*u8);
impl wxBusyInfo for wxBusyInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBusyInfo {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(_txt: @wxString) -> @wxBusyInfo {
        unsafe { @wxBusyInfoImpl(wxBusyInfo_Create(_txt.handle())) as @wxBusyInfo }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.handle()) }
    }
}

struct wxButtonImpl(*u8);
impl wxButton for wxButtonImpl {}
impl wxControl for wxButtonImpl {}
impl wxWindow for wxButtonImpl {}
impl wxEvtHandler for wxButtonImpl {}
impl wxObject for wxButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxButton : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxButton {
        unsafe { @wxButtonImpl(wxButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxButton }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @wxColour) -> c_int {
        unsafe { wxButton_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.handle()) }
    }
}

struct wxCSConvImpl(*u8);
impl wxCSConv for wxCSConvImpl {}
impl wxMBConv for wxCSConvImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCSConv : wxMBConv {
}

struct wxCalculateLayoutEventImpl(*u8);
impl wxCalculateLayoutEvent for wxCalculateLayoutEventImpl {}
impl wxEvent for wxCalculateLayoutEventImpl {}
impl wxObject for wxCalculateLayoutEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCalculateLayoutEvent : wxEvent {
    #[fixed_stack_segment]
    fn new(id: c_int) -> @wxCalculateLayoutEvent {
        unsafe { @wxCalculateLayoutEventImpl(wxCalculateLayoutEvent_Create(id)) as @wxCalculateLayoutEvent }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @wxRect {
        unsafe { @wxRectImpl(wxCalculateLayoutEvent_GetRect(self.handle())) as @wxRect }
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

struct wxCalendarCtrlImpl(*u8);
impl wxCalendarCtrl for wxCalendarCtrlImpl {}
impl wxControl for wxCalendarCtrlImpl {}
impl wxWindow for wxCalendarCtrlImpl {}
impl wxEvtHandler for wxCalendarCtrlImpl {}
impl wxObject for wxCalendarCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCalendarCtrl : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _dat: @wxDateTime, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxCalendarCtrl {
        unsafe { @wxCalendarCtrlImpl(wxCalendarCtrl_Create(_prt.handle(), _id, _dat.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxCalendarCtrl }
    }
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
    fn getHeaderColourBg(&self, _ref: @wxColour) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourFg(&self, _ref: @wxColour) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourBg(&self, _ref: @wxColour) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourFg(&self, _ref: @wxColour) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourBg(&self, _ref: @wxColour) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourFg(&self, _ref: @wxColour) {
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

struct wxCalendarDateAttrImpl(*u8);
impl wxCalendarDateAttr for wxCalendarDateAttrImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCalendarDateAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(_ctxt: *u8, _cbck: *u8, _cbrd: *u8, _fnt: *u8, _brd: c_int) -> @wxCalendarDateAttr {
        unsafe { @wxCalendarDateAttrImpl(wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)) as @wxCalendarDateAttr }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxCalendarDateAttr {
        unsafe { @wxCalendarDateAttrImpl(wxCalendarDateAttr_CreateDefault()) as @wxCalendarDateAttr }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @wxColour) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorderColour(&self, _ref: @wxColour) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @wxFont) {
        unsafe { wxCalendarDateAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @wxColour) {
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
    fn setBackgroundColour(&self, col: @wxColour) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    fn setBorderColour(&self, col: @wxColour) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxCalendarDateAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.handle(), holiday) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @wxColour) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.handle(), col.handle()) }
    }
}

struct wxCalendarEventImpl(*u8);
impl wxCalendarEvent for wxCalendarEventImpl {}
impl wxCommandEvent for wxCalendarEventImpl {}
impl wxEvent for wxCalendarEventImpl {}
impl wxObject for wxCalendarEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCalendarEvent : wxCommandEvent {
    #[fixed_stack_segment]
    fn getDate(&self, _dte: *u8) {
        unsafe { wxCalendarEvent_GetDate(self.handle(), _dte) }
    }
    #[fixed_stack_segment]
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.handle()) }
    }
}

struct wxCaretImpl(*u8);
impl wxCaret for wxCaretImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCaret {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(_wnd: @wxWindow, _wth: c_int, _hgt: c_int) -> @wxCaret {
        unsafe { @wxCaretImpl(wxCaret_Create(_wnd.handle(), _wth, _hgt)) as @wxCaret }
    }
    #[fixed_stack_segment]
    fn getBlinkTime() -> c_int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxCaret_GetPosition(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxCaret_GetSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxCaret_GetWindow(self.handle())) as @wxWindow }
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
    fn setBlinkTime(milliseconds: c_int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
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

struct wxCheckBoxImpl(*u8);
impl wxCheckBox for wxCheckBoxImpl {}
impl wxControl for wxCheckBoxImpl {}
impl wxWindow for wxCheckBoxImpl {}
impl wxEvtHandler for wxCheckBoxImpl {}
impl wxObject for wxCheckBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCheckBox : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxCheckBox {
        unsafe { @wxCheckBoxImpl(wxCheckBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxCheckBox }
    }
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

struct wxCheckListBoxImpl(*u8);
impl wxCheckListBox for wxCheckListBoxImpl {}
impl wxListBox for wxCheckListBoxImpl {}
impl wxControl for wxCheckListBoxImpl {}
impl wxWindow for wxCheckListBoxImpl {}
impl wxEvtHandler for wxCheckListBoxImpl {}
impl wxObject for wxCheckListBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCheckListBox : wxListBox {
    #[fixed_stack_segment]
    fn check(&self, item: c_int, check: bool) {
        unsafe { wxCheckListBox_Check(self.handle(), item, check) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxCheckListBox {
        unsafe { @wxCheckListBoxImpl(wxCheckListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @wxCheckListBox }
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

struct wxChoiceImpl(*u8);
impl wxChoice for wxChoiceImpl {}
impl wxControl for wxChoiceImpl {}
impl wxWindow for wxChoiceImpl {}
impl wxEvtHandler for wxChoiceImpl {}
impl wxObject for wxChoiceImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxChoice : wxControl {
    #[fixed_stack_segment]
    fn append(&self, item: @wxString) {
        unsafe { wxChoice_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxChoice_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxChoice {
        unsafe { @wxChoiceImpl(wxChoice_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @wxChoice }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: c_int) {
        unsafe { wxChoice_Delete(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> c_int {
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
    fn getString(&self, n: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxChoice_GetString(self.handle(), n)) as @wxString }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setString(&self, n: c_int, s: @wxString) {
        unsafe { wxChoice_SetString(self.handle(), n, s.handle()) }
    }
}

struct wxClassInfoImpl(*u8);
impl wxClassInfo for wxClassInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClassInfo {
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
    fn isKindOf(&self, _name: @wxString) -> bool {
        unsafe { wxClassInfo_IsKindOf(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn findClass(_txt: @wxString) -> @wxClassInfo {
        unsafe { @wxClassInfoImpl(wxClassInfo_FindClass(_txt.handle())) as @wxClassInfo }
    }
    #[fixed_stack_segment]
    fn getBaseClassName1(&self) -> @wxString {
        unsafe { @wxStringImpl(wxClassInfo_GetBaseClassName1(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getBaseClassName2(&self) -> @wxString {
        unsafe { @wxStringImpl(wxClassInfo_GetBaseClassName2(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getClassNameEx(&self) -> @wxString {
        unsafe { @wxStringImpl(wxClassInfo_GetClassNameEx(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isKindOfEx(&self, classInfo: @wxClassInfo) -> bool {
        unsafe { wxClassInfo_IsKindOfEx(self.handle(), classInfo.handle()) }
    }
}

struct wxClientImpl(*u8);
impl wxClient for wxClientImpl {}
impl wxClientBase for wxClientImpl {}
impl wxObject for wxClientImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClient : wxClientBase {
}

struct wxClientBaseImpl(*u8);
impl wxClientBase for wxClientBaseImpl {}
impl wxObject for wxClientBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClientBase : wxObject {
}

struct wxClientDCImpl(*u8);
impl wxClientDC for wxClientDCImpl {}
impl wxWindowDC for wxClientDCImpl {}
impl wxDC for wxClientDCImpl {}
impl wxObject for wxClientDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClientDC : wxWindowDC {
    #[fixed_stack_segment]
    fn new(win: @wxWindow) -> @wxClientDC {
        unsafe { @wxClientDCImpl(wxClientDC_Create(win.handle())) as @wxClientDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxClientDC_Delete(self.handle()) }
    }
}

struct wxClientDataImpl(*u8);
impl wxClientData for wxClientDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClientData {
    fn handle(&self) -> *u8;
    
}

struct wxClientDataContainerImpl(*u8);
impl wxClientDataContainer for wxClientDataContainerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClientDataContainer {
    fn handle(&self) -> *u8;
    
}

struct wxClipboardImpl(*u8);
impl wxClipboard for wxClipboardImpl {}
impl wxObject for wxClipboardImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClipboard : wxObject {
    #[fixed_stack_segment]
    fn addData(&self, data: @wxDataObject) -> bool {
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
    fn new() -> @wxClipboard {
        unsafe { @wxClipboardImpl(wxClipboard_Create()) as @wxClipboard }
    }
    #[fixed_stack_segment]
    fn flush(&self) -> bool {
        unsafe { wxClipboard_Flush(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getData(&self, data: @wxDataObject) -> bool {
        unsafe { wxClipboard_GetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn isOpened(&self) -> bool {
        unsafe { wxClipboard_IsOpened(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSupported(&self, format: @wxDataFormat) -> bool {
        unsafe { wxClipboard_IsSupported(self.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn open(&self) -> bool {
        unsafe { wxClipboard_Open(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @wxDataObject) -> bool {
        unsafe { wxClipboard_SetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn usePrimarySelection(&self, primary: bool) {
        unsafe { wxClipboard_UsePrimarySelection(self.handle(), primary) }
    }
}

struct wxCloseEventImpl(*u8);
impl wxCloseEvent for wxCloseEventImpl {}
impl wxEvent for wxCloseEventImpl {}
impl wxObject for wxCloseEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCloseEvent : wxEvent {
    #[fixed_stack_segment]
    fn canVeto(&self) -> bool {
        unsafe { wxCloseEvent_CanVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
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

struct wxClosureImpl(*u8);
impl wxClosure for wxClosureImpl {}
impl wxObject for wxClosureImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxClosure : wxObject {
    #[fixed_stack_segment]
    fn new(_fun_CEvent: *u8, _data: *u8) -> @wxClosure {
        unsafe { @wxClosureImpl(wxClosure_Create(_fun_CEvent, _data)) as @wxClosure }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> *u8 {
        unsafe { wxClosure_GetData(self.handle()) }
    }
}

struct wxColourImpl(*u8);
impl wxColour for wxColourImpl {}
impl wxObject for wxColourImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxColour : wxObject {
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
    fn newByName(_name: @wxString) -> @wxColour {
        unsafe { @wxColourImpl(wxColour_CreateByName(_name.handle())) as @wxColour }
    }
    #[fixed_stack_segment]
    fn newEmpty() -> @wxColour {
        unsafe { @wxColourImpl(wxColour_CreateEmpty()) as @wxColour }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: c_int) -> @wxColour {
        unsafe { @wxColourImpl(wxColour_CreateFromStock(id)) as @wxColour }
    }
    #[fixed_stack_segment]
    fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> @wxColour {
        unsafe { @wxColourImpl(wxColour_CreateRGB(_red, _green, _blue, _alpha)) as @wxColour }
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
    fn setByName(&self, _name: @wxString) {
        unsafe { wxColour_SetByName(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn validName(_name: *wchar_t) -> bool {
        unsafe { wxColour_ValidName(_name) }
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
    fn newFromInt(rgb: c_int) -> @wxColour {
        unsafe { @wxColourImpl(wxColour_CreateFromInt(rgb)) as @wxColour }
    }
    #[fixed_stack_segment]
    fn getInt(&self) -> c_int {
        unsafe { wxColour_GetInt(self.handle()) }
    }
    #[fixed_stack_segment]
    fn newFromUnsignedInt(rgba: uint32_t) -> @wxColour {
        unsafe { @wxColourImpl(wxColour_CreateFromUnsignedInt(rgba)) as @wxColour }
    }
    #[fixed_stack_segment]
    fn getUnsignedInt(&self) -> uint32_t {
        unsafe { wxColour_GetUnsignedInt(self.handle()) }
    }
}

struct wxColourDataImpl(*u8);
impl wxColourData for wxColourDataImpl {}
impl wxObject for wxColourDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxColourData : wxObject {
    #[fixed_stack_segment]
    fn new() -> @wxColourData {
        unsafe { @wxColourDataImpl(wxColourData_Create()) as @wxColourData }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxColourData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getChooseFull(&self) -> bool {
        unsafe { wxColourData_GetChooseFull(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @wxColour) {
        unsafe { wxColourData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCustomColour(&self, i: c_int, _ref: @wxColour) {
        unsafe { wxColourData_GetCustomColour(self.handle(), i, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setChooseFull(&self, flag: bool) {
        unsafe { wxColourData_SetChooseFull(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @wxColour) {
        unsafe { wxColourData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCustomColour(&self, i: c_int, colour: @wxColour) {
        unsafe { wxColourData_SetCustomColour(self.handle(), i, colour.handle()) }
    }
}

struct wxColourDatabaseImpl(*u8);
impl wxColourDatabase for wxColourDatabaseImpl {}
impl wxList for wxColourDatabaseImpl {}
impl wxObject for wxColourDatabaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxColourDatabase : wxList {
}

struct wxColourDialogImpl(*u8);
impl wxColourDialog for wxColourDialogImpl {}
impl wxDialog for wxColourDialogImpl {}
impl wxTopLevelWindow for wxColourDialogImpl {}
impl wxWindow for wxColourDialogImpl {}
impl wxEvtHandler for wxColourDialogImpl {}
impl wxObject for wxColourDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxColourDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, col: @wxColourData) -> @wxColourDialog {
        unsafe { @wxColourDialogImpl(wxColourDialog_Create(_prt.handle(), col.handle())) as @wxColourDialog }
    }
    #[fixed_stack_segment]
    fn getColourData(&self, _ref: @wxColourData) {
        unsafe { wxColourDialog_GetColourData(self.handle(), _ref.handle()) }
    }
}

struct wxComboBoxImpl(*u8);
impl wxComboBox for wxComboBoxImpl {}
impl wxChoice for wxComboBoxImpl {}
impl wxControl for wxComboBoxImpl {}
impl wxWindow for wxComboBoxImpl {}
impl wxEvtHandler for wxComboBoxImpl {}
impl wxObject for wxComboBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxComboBox : wxChoice {
    #[fixed_stack_segment]
    fn append(&self, item: @wxString) {
        unsafe { wxComboBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @wxString, d: *u8) {
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
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxComboBox {
        unsafe { @wxComboBoxImpl(wxComboBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, str, _stl)) as @wxComboBox }
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
    fn findString(&self, s: @wxString) -> c_int {
        unsafe { wxComboBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxComboBox_GetClientData(self.handle(), n)) as @wxClientData }
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
    fn getString(&self, n: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxComboBox_GetString(self.handle(), n)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @wxString {
        unsafe { @wxStringImpl(wxComboBox_GetStringSelection(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> @wxString {
        unsafe { @wxStringImpl(wxComboBox_GetValue(self.handle())) as @wxString }
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
    fn replace(&self, from: c_int, to: c_int, value: @wxString) {
        unsafe { wxComboBox_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, n: c_int, clientData: @wxClientData) {
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

struct wxCommandImpl(*u8);
impl wxCommand for wxCommandImpl {}
impl wxObject for wxCommandImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCommand : wxObject {
}

struct wxCommandEventImpl(*u8);
impl wxCommandEvent for wxCommandEventImpl {}
impl wxEvent for wxCommandEventImpl {}
impl wxObject for wxCommandEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCommandEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxCommandEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn new(_typ: c_int, _id: c_int) -> @wxCommandEvent {
        unsafe { @wxCommandEventImpl(wxCommandEvent_Create(_typ, _id)) as @wxCommandEvent }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCommandEvent_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxCommandEvent_GetClientData(self.handle())) as @wxClientData }
    }
    #[fixed_stack_segment]
    fn getClientObject(&self) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxCommandEvent_GetClientObject(self.handle())) as @wxClientData }
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
    fn getString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxCommandEvent_GetString(self.handle())) as @wxString }
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
    fn setClientData(&self, clientData: @wxClientData) {
        unsafe { wxCommandEvent_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, clientObject: @wxClientData) {
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
    fn setString(&self, s: @wxString) {
        unsafe { wxCommandEvent_SetString(self.handle(), s.handle()) }
    }
}

struct wxCommandLineParserImpl(*u8);
impl wxCommandLineParser for wxCommandLineParserImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCommandLineParser {
    fn handle(&self) -> *u8;
    
}

struct wxCommandProcessorImpl(*u8);
impl wxCommandProcessor for wxCommandProcessorImpl {}
impl wxObject for wxCommandProcessorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCommandProcessor : wxObject {
    #[fixed_stack_segment]
    fn canRedo(&self) -> bool {
        unsafe { wxCommandProcessor_CanRedo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { wxCommandProcessor_CanUndo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearCommands(&self) {
        unsafe { wxCommandProcessor_ClearCommands(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCommandProcessor_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCommands(&self, _ref: *u8) -> c_int {
        unsafe { wxCommandProcessor_GetCommands(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn getEditMenu(&self) -> *u8 {
        unsafe { wxCommandProcessor_GetEditMenu(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaxCommands(&self) -> c_int {
        unsafe { wxCommandProcessor_GetMaxCommands(self.handle()) }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { wxCommandProcessor_Initialize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn redo(&self) -> c_int {
        unsafe { wxCommandProcessor_Redo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditMenu(&self, menu: @wxMenu) {
        unsafe { wxCommandProcessor_SetEditMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn setMenuStrings(&self) {
        unsafe { wxCommandProcessor_SetMenuStrings(self.handle()) }
    }
    #[fixed_stack_segment]
    fn submit(&self, command: @wxCommand, storeIt: c_int) -> c_int {
        unsafe { wxCommandProcessor_Submit(self.handle(), command.handle(), storeIt) }
    }
    #[fixed_stack_segment]
    fn undo(&self) -> c_int {
        unsafe { wxCommandProcessor_Undo(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wxCommandProcessor(maxCommands: c_int) -> *u8 {
        unsafe { wxCommandProcessor_wxCommandProcessor(maxCommands) }
    }
}

struct wxConditionImpl(*u8);
impl wxCondition for wxConditionImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCondition {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn broadcast(&self) {
        unsafe { wxCondition_Broadcast(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_mut: *u8) -> @wxCondition {
        unsafe { @wxConditionImpl(wxCondition_Create(_mut)) as @wxCondition }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCondition_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn signal(&self) {
        unsafe { wxCondition_Signal(self.handle()) }
    }
    #[fixed_stack_segment]
    fn wait(&self) {
        unsafe { wxCondition_Wait(self.handle()) }
    }
    #[fixed_stack_segment]
    fn waitFor(&self, sec: c_int, nsec: c_int) -> c_int {
        unsafe { wxCondition_WaitFor(self.handle(), sec, nsec) }
    }
}

struct wxConfigBaseImpl(*u8);
impl wxConfigBase for wxConfigBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxConfigBase {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxConfigBase {
        unsafe { @wxConfigBaseImpl(wxConfigBase_Create()) as @wxConfigBase }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAll(&self) -> bool {
        unsafe { wxConfigBase_DeleteAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteEntry(&self, key: @wxString, bDeleteGroupIfEmpty: bool) -> bool {
        unsafe { wxConfigBase_DeleteEntry(self.handle(), key.handle(), bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    fn deleteGroup(&self, key: @wxString) -> bool {
        unsafe { wxConfigBase_DeleteGroup(self.handle(), key.handle()) }
    }
    #[fixed_stack_segment]
    fn exists(&self, strName: @wxString) -> bool {
        unsafe { wxConfigBase_Exists(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn expandEnvVars(&self, str: @wxString) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_ExpandEnvVars(self.handle(), str.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn flush(&self, bCurrentOnly: bool) -> bool {
        unsafe { wxConfigBase_Flush(self.handle(), bCurrentOnly) }
    }
    #[fixed_stack_segment]
    fn getAppName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetAppName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getEntryType(&self, name: @wxString) -> c_int {
        unsafe { wxConfigBase_GetEntryType(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstEntry(&self, lIndex: *u8) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetFirstEntry(self.handle(), lIndex)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getFirstGroup(&self, lIndex: *u8) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetFirstGroup(self.handle(), lIndex)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getNextEntry(&self, lIndex: *u8) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetNextEntry(self.handle(), lIndex)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getNextGroup(&self, lIndex: *u8) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetNextGroup(self.handle(), lIndex)) as @wxString }
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
    fn getPath(&self) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetPath(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVendorName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_GetVendorName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn hasEntry(&self, strName: @wxString) -> bool {
        unsafe { wxConfigBase_HasEntry(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn hasGroup(&self, strName: @wxString) -> bool {
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
    fn readBool(&self, key: @wxString, defVal: bool) -> bool {
        unsafe { wxConfigBase_ReadBool(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readDouble(&self, key: @wxString, defVal: c_double) -> c_double {
        unsafe { wxConfigBase_ReadDouble(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readInteger(&self, key: @wxString, defVal: c_int) -> c_int {
        unsafe { wxConfigBase_ReadInteger(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readString(&self, key: @wxString, defVal: @wxString) -> @wxString {
        unsafe { @wxStringImpl(wxConfigBase_ReadString(self.handle(), key.handle(), defVal.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn renameEntry(&self, oldName: @wxString, newName: @wxString) -> bool {
        unsafe { wxConfigBase_RenameEntry(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn renameGroup(&self, oldName: @wxString, newName: @wxString) -> bool {
        unsafe { wxConfigBase_RenameGroup(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn setAppName(&self, appName: @wxString) {
        unsafe { wxConfigBase_SetAppName(self.handle(), appName.handle()) }
    }
    #[fixed_stack_segment]
    fn setExpandEnvVars(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, strPath: @wxString) {
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
    fn setVendorName(&self, vendorName: @wxString) {
        unsafe { wxConfigBase_SetVendorName(self.handle(), vendorName.handle()) }
    }
    #[fixed_stack_segment]
    fn writeBool(&self, key: @wxString, value: bool) -> bool {
        unsafe { wxConfigBase_WriteBool(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeDouble(&self, key: @wxString, value: c_double) -> bool {
        unsafe { wxConfigBase_WriteDouble(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeInteger(&self, key: @wxString, value: c_int) -> bool {
        unsafe { wxConfigBase_WriteInteger(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeLong(&self, key: @wxString, value: c_long) -> bool {
        unsafe { wxConfigBase_WriteLong(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, key: @wxString, value: @wxString) -> bool {
        unsafe { wxConfigBase_WriteString(self.handle(), key.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn get() -> @wxConfigBase {
        unsafe { @wxConfigBaseImpl(wxConfigBase_Get()) as @wxConfigBase }
    }
    #[fixed_stack_segment]
    fn set(self_: @wxConfigBase) {
        unsafe { wxConfigBase_Set(self_.handle()) }
    }
}

struct wxConnectionImpl(*u8);
impl wxConnection for wxConnectionImpl {}
impl wxConnectionBase for wxConnectionImpl {}
impl wxObject for wxConnectionImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxConnection : wxConnectionBase {
}

struct wxConnectionBaseImpl(*u8);
impl wxConnectionBase for wxConnectionBaseImpl {}
impl wxObject for wxConnectionBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxConnectionBase : wxObject {
}

struct wxContextHelpImpl(*u8);
impl wxContextHelp for wxContextHelpImpl {}
impl wxObject for wxContextHelpImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxContextHelp : wxObject {
    #[fixed_stack_segment]
    fn beginContextHelp(&self, win: @wxWindow) -> bool {
        unsafe { wxContextHelp_BeginContextHelp(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn new(win: @wxWindow, beginHelp: bool) -> @wxContextHelp {
        unsafe { @wxContextHelpImpl(wxContextHelp_Create(win.handle(), beginHelp)) as @wxContextHelp }
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

struct wxContextHelpButtonImpl(*u8);
impl wxContextHelpButton for wxContextHelpButtonImpl {}
impl wxBitmapButton for wxContextHelpButtonImpl {}
impl wxButton for wxContextHelpButtonImpl {}
impl wxControl for wxContextHelpButtonImpl {}
impl wxWindow for wxContextHelpButtonImpl {}
impl wxEvtHandler for wxContextHelpButtonImpl {}
impl wxObject for wxContextHelpButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxContextHelpButton : wxBitmapButton {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @wxContextHelpButton {
        unsafe { @wxContextHelpButtonImpl(wxContextHelpButton_Create(parent.handle(), id, x, y, w, h, style)) as @wxContextHelpButton }
    }
}

struct wxControlImpl(*u8);
impl wxControl for wxControlImpl {}
impl wxWindow for wxControlImpl {}
impl wxEvtHandler for wxControlImpl {}
impl wxObject for wxControlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxControl : wxWindow {
    #[fixed_stack_segment]
    fn command(&self, event: @wxEvent) {
        unsafe { wxControl_Command(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { @wxStringImpl(wxControl_GetLabel(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, text: @wxString) {
        unsafe { wxControl_SetLabel(self.handle(), text.handle()) }
    }
}

struct wxCountingOutputStreamImpl(*u8);
impl wxCountingOutputStream for wxCountingOutputStreamImpl {}
impl wxOutputStream for wxCountingOutputStreamImpl {}
impl wxStreamBase for wxCountingOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCountingOutputStream : wxOutputStream {
}

struct wxCriticalSectionImpl(*u8);
impl wxCriticalSection for wxCriticalSectionImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCriticalSection {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxCriticalSection {
        unsafe { @wxCriticalSectionImpl(wxCriticalSection_Create()) as @wxCriticalSection }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCriticalSection_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enter(&self) {
        unsafe { wxCriticalSection_Enter(self.handle()) }
    }
    #[fixed_stack_segment]
    fn leave(&self) {
        unsafe { wxCriticalSection_Leave(self.handle()) }
    }
}

struct wxCriticalSectionLockerImpl(*u8);
impl wxCriticalSectionLocker for wxCriticalSectionLockerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCriticalSectionLocker {
    fn handle(&self) -> *u8;
    
}

struct wxCursorImpl(*u8);
impl wxCursor for wxCursorImpl {}
impl wxBitmap for wxCursorImpl {}
impl wxGDIObject for wxCursorImpl {}
impl wxObject for wxCursorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCursor : wxBitmap {
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

struct wxCustomDataObjectImpl(*u8);
impl wxCustomDataObject for wxCustomDataObjectImpl {}
impl wxDataObjectSimple for wxCustomDataObjectImpl {}
impl wxDataObject for wxCustomDataObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxCustomDataObject : wxDataObjectSimple {
}

struct wxDCImpl(*u8);
impl wxDC for wxDCImpl {}
impl wxObject for wxDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDC : wxObject {
    #[fixed_stack_segment]
    fn blit(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: @wxDC, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: bool) -> bool {
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
    fn drawBitmap(&self, bmp: @wxBitmap, x: c_int, y: c_int, useMask: bool) {
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
    fn drawIcon(&self, icon: @wxIcon, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.handle(), icon.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawLabel(&self, str: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        unsafe { wxDC_DrawLabel(self.handle(), str.handle(), x, y, w, h, align, indexAccel) }
    }
    #[fixed_stack_segment]
    fn drawLabelBitmap(&self, str: @wxString, bmp: @wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> @wxRect {
        unsafe { @wxRectImpl(wxDC_DrawLabelBitmap(self.handle(), str.handle(), bmp.handle(), x, y, w, h, align, indexAccel)) as @wxRect }
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
    fn drawRotatedText(&self, text: @wxString, x: c_int, y: c_int, angle: c_double) {
        unsafe { wxDC_DrawRotatedText(self.handle(), text.handle(), x, y, angle) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.handle(), x, y, width, height, radius) }
    }
    #[fixed_stack_segment]
    fn drawText(&self, text: @wxString, x: c_int, y: c_int) {
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
    fn floodFill(&self, x: c_int, y: c_int, col: @wxColour, style: c_int) {
        unsafe { wxDC_FloodFill(self.handle(), x, y, col.handle(), style) }
    }
    #[fixed_stack_segment]
    fn getBackground(&self, _ref: @wxBrush) {
        unsafe { wxDC_GetBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBrush(&self, _ref: @wxBrush) {
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
    fn getFont(&self, _ref: @wxFont) {
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
    fn getPPI(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxDC_GetPPI(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getPen(&self, _ref: @wxPen) {
        unsafe { wxDC_GetPen(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPixel(&self, x: c_int, y: c_int, col: @wxColour) -> bool {
        unsafe { wxDC_GetPixel(self.handle(), x, y, col.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxDC_GetSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getSizeMM(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxDC_GetSizeMM(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getTextBackground(&self, _ref: @wxColour) {
        unsafe { wxDC_GetTextBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @wxString, w: *u8, h: *u8, descent: *u8, externalLeading: *u8, theFont: @wxFont) {
        unsafe { wxDC_GetTextExtent(self.handle(), string.handle(), w, h, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getMultiLineTextExtent(&self, string: @wxString, w: *u8, h: *u8, heightLine: *u8, theFont: @wxFont) {
        unsafe { wxDC_GetMultiLineTextExtent(self.handle(), string.handle(), w, h, heightLine, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextForeground(&self, _ref: @wxColour) {
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
    fn setBackground(&self, brush: @wxBrush) {
        unsafe { wxDC_SetBackground(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @wxBrush) {
        unsafe { wxDC_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn setClippingRegionFromRegion(&self, region: @wxRegion) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
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
    fn setPalette(&self, palette: @wxPalette) {
        unsafe { wxDC_SetPalette(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @wxPen) {
        unsafe { wxDC_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextBackground(&self, colour: @wxColour) {
        unsafe { wxDC_SetTextBackground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextForeground(&self, colour: @wxColour) {
        unsafe { wxDC_SetTextForeground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn startDoc(&self, msg: @wxString) -> bool {
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
    fn getPixel2(&self, x: c_int, y: c_int, col: @wxColour) {
        unsafe { wxDC_GetPixel2(self.handle(), x, y, col.handle()) }
    }
}

struct wxDCClipperImpl(*u8);
impl wxDCClipper for wxDCClipperImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDCClipper {
    fn handle(&self) -> *u8;
    
}

struct wxDDEClientImpl(*u8);
impl wxDDEClient for wxDDEClientImpl {}
impl wxClientBase for wxDDEClientImpl {}
impl wxObject for wxDDEClientImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDDEClient : wxClientBase {
}

struct wxDDEConnectionImpl(*u8);
impl wxDDEConnection for wxDDEConnectionImpl {}
impl wxConnectionBase for wxDDEConnectionImpl {}
impl wxObject for wxDDEConnectionImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDDEConnection : wxConnectionBase {
}

struct wxDDEServerImpl(*u8);
impl wxDDEServer for wxDDEServerImpl {}
impl wxServerBase for wxDDEServerImpl {}
impl wxObject for wxDDEServerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDDEServer : wxServerBase {
}

struct wxDataFormatImpl(*u8);
impl wxDataFormat for wxDataFormatImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDataFormat {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn newFromId(name: @wxString) -> @wxDataFormat {
        unsafe { @wxDataFormatImpl(wxDataFormat_CreateFromId(name.handle())) as @wxDataFormat }
    }
    #[fixed_stack_segment]
    fn newFromType(typ: c_int) -> @wxDataFormat {
        unsafe { @wxDataFormatImpl(wxDataFormat_CreateFromType(typ)) as @wxDataFormat }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDataFormat_GetId(self.handle())) as @wxString }
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

struct wxDataInputStreamImpl(*u8);
impl wxDataInputStream for wxDataInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDataInputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDataObjectImpl(*u8);
impl wxDataObject for wxDataObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDataObject {
    fn handle(&self) -> *u8;
    
}

struct wxDataObjectCompositeImpl(*u8);
impl wxDataObjectComposite for wxDataObjectCompositeImpl {}
impl wxDataObject for wxDataObjectCompositeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDataObjectComposite : wxDataObject {
    #[fixed_stack_segment]
    fn add(&self, _dat: *u8, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.handle(), _dat, _preferred) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxDataObjectComposite {
        unsafe { @wxDataObjectCompositeImpl(wxDataObjectComposite_Create()) as @wxDataObjectComposite }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.handle()) }
    }
}

struct wxDataObjectSimpleImpl(*u8);
impl wxDataObjectSimple for wxDataObjectSimpleImpl {}
impl wxDataObject for wxDataObjectSimpleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDataObjectSimple : wxDataObject {
}

struct wxDataOutputStreamImpl(*u8);
impl wxDataOutputStream for wxDataOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDataOutputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDatabaseImpl(*u8);
impl wxDatabase for wxDatabaseImpl {}
impl wxObject for wxDatabaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDatabase : wxObject {
}

struct wxDateTimeImpl(*u8);
impl wxDateTime for wxDateTimeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDateTime {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addDate(&self, diff: *u8, _ref: @wxDateTime) {
        unsafe { wxDateTime_AddDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.handle(), _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    fn addTime(&self, diff: *u8, _ref: @wxDateTime) {
        unsafe { wxDateTime_AddTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.handle(), _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxDateTime {
        unsafe { @wxDateTimeImpl(wxDateTime_Create()) as @wxDateTime }
    }
    #[fixed_stack_segment]
    fn format(&self, format: *u8, tz: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_Format(self.handle(), format, tz)) as @wxString }
    }
    #[fixed_stack_segment]
    fn formatDate(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_FormatDate(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn formatISODate(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_FormatISODate(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn formatISOTime(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_FormatISOTime(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn formatTime(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_FormatTime(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getAmString() -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_GetAmString()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getBeginDST(year: c_int, country: c_int, dt: @wxDateTime) {
        unsafe { wxDateTime_GetBeginDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    fn getCentury(year: c_int) -> c_int {
        unsafe { wxDateTime_GetCentury(year) }
    }
    #[fixed_stack_segment]
    fn getCountry() -> c_int {
        unsafe { wxDateTime_GetCountry() }
    }
    #[fixed_stack_segment]
    fn getCurrentMonth(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentMonth(cal) }
    }
    #[fixed_stack_segment]
    fn getCurrentYear(cal: c_int) -> c_int {
        unsafe { wxDateTime_GetCurrentYear(cal) }
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
    fn getEndDST(year: c_int, country: c_int, dt: @wxDateTime) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    fn getHour(&self, tz: c_int) -> c_int {
        unsafe { wxDateTime_GetHour(self.handle(), tz) }
    }
    #[fixed_stack_segment]
    fn getLastMonthDay(&self, month: c_int, year: c_int, _ref: @wxDateTime) {
        unsafe { wxDateTime_GetLastMonthDay(self.handle(), month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int, _ref: @wxDateTime) {
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
    fn getMonthName(month: c_int, flags: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_GetMonthName(month, flags)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getNextWeekDay(&self, weekday: c_int, _ref: @wxDateTime) {
        unsafe { wxDateTime_GetNextWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getNumberOfDays(year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    #[fixed_stack_segment]
    fn getNumberOfDaysMonth(month: c_int, year: c_int, cal: c_int) -> c_int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    #[fixed_stack_segment]
    fn getPmString() -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_GetPmString()) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPrevWeekDay(&self, weekday: c_int, _ref: @wxDateTime) {
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
    fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    fn getValue(&self, hi_long: *u8, lo_long: *u8) {
        unsafe { wxDateTime_GetValue(self.handle(), hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    fn getWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: @wxDateTime) {
        unsafe { wxDateTime_GetWeekDay(self.handle(), weekday, n, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeekDayInSameWeek(&self, weekday: c_int, _ref: @wxDateTime) {
        unsafe { wxDateTime_GetWeekDayInSameWeek(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeekDayName(weekday: c_int, flags: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxDateTime_GetWeekDayName(weekday, flags)) as @wxString }
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
    fn isBetween(&self, t1: @wxDateTime, t2: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    fn isDST(&self, country: c_int) -> bool {
        unsafe { wxDateTime_IsDST(self.handle(), country) }
    }
    #[fixed_stack_segment]
    fn isDSTApplicable(year: c_int, country: c_int) -> bool {
        unsafe { wxDateTime_IsDSTApplicable(year, country) }
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
    fn isEqualUpTo(&self, dt: @wxDateTime, ts: *u8) -> bool {
        unsafe { wxDateTime_IsEqualUpTo(self.handle(), dt.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn isGregorianDate(&self, country: c_int) -> bool {
        unsafe { wxDateTime_IsGregorianDate(self.handle(), country) }
    }
    #[fixed_stack_segment]
    fn isLaterThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsLaterThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn isLeapYear(year: c_int, cal: c_int) -> bool {
        unsafe { wxDateTime_IsLeapYear(year, cal) }
    }
    #[fixed_stack_segment]
    fn isSameDate(&self, dt: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsSameDate(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isSameTime(&self, dt: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsSameTime(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isStrictlyBetween(&self, t1: @wxDateTime, t2: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsStrictlyBetween(self.handle(), t1.handle(), t2.handle()) }
    }
    #[fixed_stack_segment]
    fn isValid(&self) -> bool {
        unsafe { wxDateTime_IsValid(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isWestEuropeanCountry(country: c_int) -> bool {
        unsafe { wxDateTime_IsWestEuropeanCountry(country) }
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
    fn parseTime(&self, time: @wxTime) -> *u8 {
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
    fn setCountry(country: c_int) {
        unsafe { wxDateTime_SetCountry(country) }
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
    fn subtractDate(&self, diff: *u8, _ref: @wxDateTime) {
        unsafe { wxDateTime_SubtractDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractTime(&self, diff: *u8, _ref: @wxDateTime) {
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
    fn wxDateTime(hi_long: c_int, lo_long: c_int) -> *u8 {
        unsafe { wxDateTime_wxDateTime(hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDateTime_Delete(self.handle()) }
    }
}

struct wxDbImpl(*u8);
impl wxDb for wxDbImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDb {
    fn handle(&self) -> *u8;
    
}

struct wxDbColDefImpl(*u8);
impl wxDbColDef for wxDbColDefImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbColDef {
    fn handle(&self) -> *u8;
    
}

struct wxDbColForImpl(*u8);
impl wxDbColFor for wxDbColForImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbColFor {
    fn handle(&self) -> *u8;
    
}

struct wxDbColInfImpl(*u8);
impl wxDbColInf for wxDbColInfImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbColInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbConnectInfImpl(*u8);
impl wxDbConnectInf for wxDbConnectInfImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbConnectInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbInfImpl(*u8);
impl wxDbInf for wxDbInfImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbSqlTypeInfoImpl(*u8);
impl wxDbSqlTypeInfo for wxDbSqlTypeInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbSqlTypeInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDbTableImpl(*u8);
impl wxDbTable for wxDbTableImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbTable {
    fn handle(&self) -> *u8;
    
}

struct wxDbTableInfoImpl(*u8);
impl wxDbTableInfo for wxDbTableInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDbTableInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDebugContextImpl(*u8);
impl wxDebugContext for wxDebugContextImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDebugContext {
    fn handle(&self) -> *u8;
    
}

struct wxDialUpEventImpl(*u8);
impl wxDialUpEvent for wxDialUpEventImpl {}
impl wxEvent for wxDialUpEventImpl {}
impl wxObject for wxDialUpEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDialUpEvent : wxEvent {
    #[fixed_stack_segment]
    fn isConnectedEvent(&self) -> bool {
        unsafe { wxDialUpEvent_IsConnectedEvent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOwnEvent(&self) -> bool {
        unsafe { wxDialUpEvent_IsOwnEvent(self.handle()) }
    }
}

struct wxDialUpManagerImpl(*u8);
impl wxDialUpManager for wxDialUpManagerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDialUpManager {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn cancelDialing(&self) -> bool {
        unsafe { wxDialUpManager_CancelDialing(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxDialUpManager {
        unsafe { @wxDialUpManagerImpl(wxDialUpManager_Create()) as @wxDialUpManager }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDialUpManager_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn dial(&self, nameOfISP: @wxString, username: @wxString, password: @wxString, async: bool) -> bool {
        unsafe { wxDialUpManager_Dial(self.handle(), nameOfISP.handle(), username.handle(), password.handle(), async) }
    }
    #[fixed_stack_segment]
    fn disableAutoCheckOnlineStatus(&self) {
        unsafe { wxDialUpManager_DisableAutoCheckOnlineStatus(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableAutoCheckOnlineStatus(&self, nSeconds: c_int) -> bool {
        unsafe { wxDialUpManager_EnableAutoCheckOnlineStatus(self.handle(), nSeconds) }
    }
    #[fixed_stack_segment]
    fn getISPNames(&self, _lst: @wxList) -> c_int {
        unsafe { wxDialUpManager_GetISPNames(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn hangUp(&self) -> bool {
        unsafe { wxDialUpManager_HangUp(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isAlwaysOnline(&self) -> bool {
        unsafe { wxDialUpManager_IsAlwaysOnline(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isDialing(&self) -> bool {
        unsafe { wxDialUpManager_IsDialing(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxDialUpManager_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOnline(&self) -> bool {
        unsafe { wxDialUpManager_IsOnline(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setConnectCommand(&self, commandDial: @wxString, commandHangup: @wxString) {
        unsafe { wxDialUpManager_SetConnectCommand(self.handle(), commandDial.handle(), commandHangup.handle()) }
    }
    #[fixed_stack_segment]
    fn setOnlineStatus(&self, isOnline: bool) {
        unsafe { wxDialUpManager_SetOnlineStatus(self.handle(), isOnline) }
    }
    #[fixed_stack_segment]
    fn setWellKnownHost(&self, hostname: @wxString, portno: c_int) {
        unsafe { wxDialUpManager_SetWellKnownHost(self.handle(), hostname.handle(), portno) }
    }
}

struct wxDialogImpl(*u8);
impl wxDialog for wxDialogImpl {}
impl wxTopLevelWindow for wxDialogImpl {}
impl wxWindow for wxDialogImpl {}
impl wxEvtHandler for wxDialogImpl {}
impl wxObject for wxDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDialog : wxTopLevelWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDialog {
        unsafe { @wxDialogImpl(wxDialog_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxDialog }
    }
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

struct wxDirDialogImpl(*u8);
impl wxDirDialog for wxDirDialogImpl {}
impl wxDialog for wxDirDialogImpl {}
impl wxTopLevelWindow for wxDirDialogImpl {}
impl wxWindow for wxDirDialogImpl {}
impl wxEvtHandler for wxDirDialogImpl {}
impl wxObject for wxDirDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDirDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _msg: @wxString, _dir: @wxString, _lft: c_int, _top: c_int, _stl: c_int) -> @wxDirDialog {
        unsafe { @wxDirDialogImpl(wxDirDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _lft, _top, _stl)) as @wxDirDialog }
    }
    #[fixed_stack_segment]
    fn getMessage(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDirDialog_GetMessage(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @wxString {
        unsafe { @wxStringImpl(wxDirDialog_GetPath(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, msg: @wxString) {
        unsafe { wxDirDialog_SetMessage(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, pth: @wxString) {
        unsafe { wxDirDialog_SetPath(self.handle(), pth.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.handle(), style) }
    }
}

struct wxDirTraverserImpl(*u8);
impl wxDirTraverser for wxDirTraverserImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDirTraverser {
    fn handle(&self) -> *u8;
    
}

struct wxDllLoaderImpl(*u8);
impl wxDllLoader for wxDllLoaderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDllLoader {
    fn handle(&self) -> *u8;
    
}

struct wxDocChildFrameImpl(*u8);
impl wxDocChildFrame for wxDocChildFrameImpl {}
impl wxFrame for wxDocChildFrameImpl {}
impl wxTopLevelWindow for wxDocChildFrameImpl {}
impl wxWindow for wxDocChildFrameImpl {}
impl wxEvtHandler for wxDocChildFrameImpl {}
impl wxObject for wxDocChildFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocChildFrame : wxFrame {
}

struct wxDocMDIChildFrameImpl(*u8);
impl wxDocMDIChildFrame for wxDocMDIChildFrameImpl {}
impl wxMDIChildFrame for wxDocMDIChildFrameImpl {}
impl wxFrame for wxDocMDIChildFrameImpl {}
impl wxTopLevelWindow for wxDocMDIChildFrameImpl {}
impl wxWindow for wxDocMDIChildFrameImpl {}
impl wxEvtHandler for wxDocMDIChildFrameImpl {}
impl wxObject for wxDocMDIChildFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocMDIChildFrame : wxMDIChildFrame {
}

struct wxDocMDIParentFrameImpl(*u8);
impl wxDocMDIParentFrame for wxDocMDIParentFrameImpl {}
impl wxMDIParentFrame for wxDocMDIParentFrameImpl {}
impl wxFrame for wxDocMDIParentFrameImpl {}
impl wxTopLevelWindow for wxDocMDIParentFrameImpl {}
impl wxWindow for wxDocMDIParentFrameImpl {}
impl wxEvtHandler for wxDocMDIParentFrameImpl {}
impl wxObject for wxDocMDIParentFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocMDIParentFrame : wxMDIParentFrame {
}

struct wxDocManagerImpl(*u8);
impl wxDocManager for wxDocManagerImpl {}
impl wxEvtHandler for wxDocManagerImpl {}
impl wxObject for wxDocManagerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocManager : wxEvtHandler {
}

struct wxDocParentFrameImpl(*u8);
impl wxDocParentFrame for wxDocParentFrameImpl {}
impl wxFrame for wxDocParentFrameImpl {}
impl wxTopLevelWindow for wxDocParentFrameImpl {}
impl wxWindow for wxDocParentFrameImpl {}
impl wxEvtHandler for wxDocParentFrameImpl {}
impl wxObject for wxDocParentFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocParentFrame : wxFrame {
}

struct wxDocTemplateImpl(*u8);
impl wxDocTemplate for wxDocTemplateImpl {}
impl wxObject for wxDocTemplateImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocTemplate : wxObject {
}

struct wxDocumentImpl(*u8);
impl wxDocument for wxDocumentImpl {}
impl wxEvtHandler for wxDocumentImpl {}
impl wxObject for wxDocumentImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDocument : wxEvtHandler {
}

struct wxDragImageImpl(*u8);
impl wxDragImage for wxDragImageImpl {}
impl wxObject for wxDragImageImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDragImage : wxObject {
    #[fixed_stack_segment]
    fn new(image: @wxBitmap, x: c_int, y: c_int) -> @wxDragImage {
        unsafe { @wxDragImageImpl(wxDragImage_Create(image.handle(), x, y)) as @wxDragImage }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDragImage_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDragFullScreen(&self, x_pos: c_int, y_pos: c_int, window: @wxWindow, fullScreen: bool, rect: @wxRect) -> bool {
        unsafe { wxDragImage_BeginDragFullScreen(self.handle(), x_pos, y_pos, window.handle(), fullScreen, rect.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDrag(&self, x: c_int, y: c_int, window: @wxWindow, boundingWindow: @wxWindow) -> bool {
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

struct wxDrawControlImpl(*u8);
impl wxDrawControl for wxDrawControlImpl {}
impl wxControl for wxDrawControlImpl {}
impl wxWindow for wxDrawControlImpl {}
impl wxEvtHandler for wxDrawControlImpl {}
impl wxObject for wxDrawControlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDrawControl : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDrawControl {
        unsafe { @wxDrawControlImpl(wxDrawControl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxDrawControl }
    }
}

struct wxDrawWindowImpl(*u8);
impl wxDrawWindow for wxDrawWindowImpl {}
impl wxWindow for wxDrawWindowImpl {}
impl wxEvtHandler for wxDrawWindowImpl {}
impl wxObject for wxDrawWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDrawWindow : wxWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxDrawWindow {
        unsafe { @wxDrawWindowImpl(wxDrawWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxDrawWindow }
    }
}

struct wxDropFilesEventImpl(*u8);
impl wxDropFilesEvent for wxDropFilesEventImpl {}
impl wxEvent for wxDropFilesEventImpl {}
impl wxObject for wxDropFilesEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDropFilesEvent : wxEvent {
}

struct wxDropSourceImpl(*u8);
impl wxDropSource for wxDropSourceImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDropSource {
    fn handle(&self) -> *u8;
    
}

struct wxDropTargetImpl(*u8);
impl wxDropTarget for wxDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDropTarget {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDataObject(&self, _dat: @wxDataObject) {
        unsafe { wxDropTarget_SetDataObject(self.handle(), _dat.handle()) }
    }
}

struct wxDynToolInfoImpl(*u8);
impl wxDynToolInfo for wxDynToolInfoImpl {}
impl wxToolLayoutItem for wxDynToolInfoImpl {}
impl wxObject for wxDynToolInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDynToolInfo : wxToolLayoutItem {
    #[fixed_stack_segment]
    fn index(&self) -> c_int {
        unsafe { wxDynToolInfo_Index(self.handle()) }
    }
    #[fixed_stack_segment]
    fn realSize(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxDynToolInfo_RealSize(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn pToolWnd(&self) -> *u8 {
        unsafe { wxDynToolInfo_pToolWnd(self.handle()) }
    }
}

struct wxDynamicLibraryImpl(*u8);
impl wxDynamicLibrary for wxDynamicLibraryImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDynamicLibrary {
    fn handle(&self) -> *u8;
    
}

struct wxDynamicSashWindowImpl(*u8);
impl wxDynamicSashWindow for wxDynamicSashWindowImpl {}
impl wxWindow for wxDynamicSashWindowImpl {}
impl wxEvtHandler for wxDynamicSashWindowImpl {}
impl wxObject for wxDynamicSashWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDynamicSashWindow : wxWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxDynamicSashWindow {
        unsafe { @wxDynamicSashWindowImpl(wxDynamicSashWindow_Create(parent.handle(), id, x, y, w, h, style)) as @wxDynamicSashWindow }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDynamicSashWindow_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHScrollBar(&self, child: @wxWindow) -> *u8 {
        unsafe { wxDynamicSashWindow_GetHScrollBar(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn getVScrollBar(&self, child: @wxWindow) -> *u8 {
        unsafe { wxDynamicSashWindow_GetVScrollBar(self.handle(), child.handle()) }
    }
}

struct wxDynamicToolBarImpl(*u8);
impl wxDynamicToolBar for wxDynamicToolBarImpl {}
impl wxToolBarBase for wxDynamicToolBarImpl {}
impl wxControl for wxDynamicToolBarImpl {}
impl wxWindow for wxDynamicToolBarImpl {}
impl wxEvtHandler for wxDynamicToolBarImpl {}
impl wxObject for wxDynamicToolBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDynamicToolBar : wxToolBarBase {
    #[fixed_stack_segment]
    fn addSeparator(&self, pSepartorWnd: *u8) {
        unsafe { wxDynamicToolBar_AddSeparator(self.handle(), pSepartorWnd) }
    }
    #[fixed_stack_segment]
    fn addTool(&self, toolIndex: c_int, pToolWindow: *u8, w: c_int, h: c_int) {
        unsafe { wxDynamicToolBar_AddTool(self.handle(), toolIndex, pToolWindow, w, h) }
    }
    #[fixed_stack_segment]
    fn addToolBitmap(&self, toolIndex: c_int, bitmap: @wxBitmap, pushedBitmap: *u8, toggle: c_int, x: c_int, y: c_int, clientData: @wxClientData, helpString1: *u8, helpString2: *u8) -> *u8 {
        unsafe { wxDynamicToolBar_AddToolBitmap(self.handle(), toolIndex, bitmap.handle(), pushedBitmap, toggle, x, y, clientData.handle(), helpString1, helpString2) }
    }
    #[fixed_stack_segment]
    fn addToolImage(&self, toolIndex: c_int, imageFileName: *u8, imageFileType: c_int, labelText: *u8, alignTextRight: c_int, isFlat: bool) {
        unsafe { wxDynamicToolBar_AddToolImage(self.handle(), toolIndex, imageFileName, imageFileType, labelText, alignTextRight, isFlat) }
    }
    #[fixed_stack_segment]
    fn addToolLabel(&self, toolIndex: c_int, labelBmp: *u8, labelText: *u8, alignTextRight: c_int, isFlat: bool) {
        unsafe { wxDynamicToolBar_AddToolLabel(self.handle(), toolIndex, labelBmp, labelText, alignTextRight, isFlat) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, orientation: c_int, RowsOrColumns: c_int) -> @wxDynamicToolBar {
        unsafe { @wxDynamicToolBarImpl(wxDynamicToolBar_Create(parent.handle(), id, x, y, w, h, style, orientation, RowsOrColumns)) as @wxDynamicToolBar }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxDynamicToolBar {
        unsafe { @wxDynamicToolBarImpl(wxDynamicToolBar_CreateDefault()) as @wxDynamicToolBar }
    }
    #[fixed_stack_segment]
    fn newDefaultLayout(&self) -> *u8 {
        unsafe { wxDynamicToolBar_CreateDefaultLayout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn newParams(&self, parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, orientation: c_int, RowsOrColumns: c_int) -> c_int {
        unsafe { wxDynamicToolBar_CreateParams(self.handle(), parent.handle(), id, x, y, w, h, style, orientation, RowsOrColumns) }
    }
    #[fixed_stack_segment]
    fn newTool(&self, id: c_int, label: *u8, bmpNormal: *u8, bmpDisabled: *u8, kind: c_int, clientData: @wxClientData, shortHelp: *u8, longHelp: *u8) -> *u8 {
        unsafe { wxDynamicToolBar_CreateTool(self.handle(), id, label, bmpNormal, bmpDisabled, kind, clientData.handle(), shortHelp, longHelp) }
    }
    #[fixed_stack_segment]
    fn newToolControl(&self, control: @wxControl) -> *u8 {
        unsafe { wxDynamicToolBar_CreateToolControl(self.handle(), control.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDynamicToolBar_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn doDeleteTool(&self, pos: c_int, tool: *u8) -> c_int {
        unsafe { wxDynamicToolBar_DoDeleteTool(self.handle(), pos, tool) }
    }
    #[fixed_stack_segment]
    fn doEnableTool(&self, tool: *u8, enable: bool) {
        unsafe { wxDynamicToolBar_DoEnableTool(self.handle(), tool, enable) }
    }
    #[fixed_stack_segment]
    fn doInsertTool(&self, pos: c_int, tool: *u8) -> c_int {
        unsafe { wxDynamicToolBar_DoInsertTool(self.handle(), pos, tool) }
    }
    #[fixed_stack_segment]
    fn doSetToggle(&self, tool: *u8, toggle: c_int) {
        unsafe { wxDynamicToolBar_DoSetToggle(self.handle(), tool, toggle) }
    }
    #[fixed_stack_segment]
    fn doToggleTool(&self, tool: *u8, toggle: c_int) {
        unsafe { wxDynamicToolBar_DoToggleTool(self.handle(), tool, toggle) }
    }
    #[fixed_stack_segment]
    fn drawSeparator(&self, info: *u8, dc: @wxDC) {
        unsafe { wxDynamicToolBar_DrawSeparator(self.handle(), info, dc.handle()) }
    }
    #[fixed_stack_segment]
    fn enableTool(&self, toolIndex: c_int, enable: bool) {
        unsafe { wxDynamicToolBar_EnableTool(self.handle(), toolIndex, enable) }
    }
    #[fixed_stack_segment]
    fn findToolForPosition(&self, x: c_int, y: c_int) -> *u8 {
        unsafe { wxDynamicToolBar_FindToolForPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getPreferredDim(&self, gw: c_int, gh: c_int, pw: *u8, ph: *u8) {
        unsafe { wxDynamicToolBar_GetPreferredDim(self.handle(), gw, gh, pw, ph) }
    }
    #[fixed_stack_segment]
    fn getToolInfo(&self, toolIndex: c_int) -> *u8 {
        unsafe { wxDynamicToolBar_GetToolInfo(self.handle(), toolIndex) }
    }
    #[fixed_stack_segment]
    fn layout(&self) -> c_int {
        unsafe { wxDynamicToolBar_Layout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn removeTool(&self, toolIndex: c_int) {
        unsafe { wxDynamicToolBar_RemoveTool(self.handle(), toolIndex) }
    }
    #[fixed_stack_segment]
    fn setLayout(&self, pLayout: *u8) {
        unsafe { wxDynamicToolBar_SetLayout(self.handle(), pLayout) }
    }
}

struct wxEditableListBoxImpl(*u8);
impl wxEditableListBox for wxEditableListBoxImpl {}
impl wxPanel for wxEditableListBoxImpl {}
impl wxWindow for wxEditableListBoxImpl {}
impl wxEvtHandler for wxEditableListBoxImpl {}
impl wxObject for wxEditableListBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxEditableListBox : wxPanel {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, label: *wchar_t, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxEditableListBox {
        unsafe { @wxEditableListBoxImpl(wxEditableListBox_Create(parent.handle(), id, label, x, y, w, h, style)) as @wxEditableListBox }
    }
    #[fixed_stack_segment]
    fn getDelButton(&self) -> *u8 {
        unsafe { wxEditableListBox_GetDelButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDownButton(&self) -> *u8 {
        unsafe { wxEditableListBox_GetDownButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditButton(&self) -> *u8 {
        unsafe { wxEditableListBox_GetEditButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getListCtrl(&self) -> @wxListCtrl {
        unsafe { @wxListCtrlImpl(wxEditableListBox_GetListCtrl(self.handle())) as @wxListCtrl }
    }
    #[fixed_stack_segment]
    fn getNewButton(&self) -> *u8 {
        unsafe { wxEditableListBox_GetNewButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStrings(&self, _ref: *wchar_t) -> c_int {
        unsafe { wxEditableListBox_GetStrings(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn getUpButton(&self) -> *u8 {
        unsafe { wxEditableListBox_GetUpButton(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setStrings(&self, strings: *u8, _n: c_int) {
        unsafe { wxEditableListBox_SetStrings(self.handle(), strings, _n) }
    }
}

struct wxEncodingConverterImpl(*u8);
impl wxEncodingConverter for wxEncodingConverterImpl {}
impl wxObject for wxEncodingConverterImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxEncodingConverter : wxObject {
    #[fixed_stack_segment]
    fn convert(&self, input: *u8, output: *u8) {
        unsafe { wxEncodingConverter_Convert(self.handle(), input, output) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxEncodingConverter {
        unsafe { @wxEncodingConverterImpl(wxEncodingConverter_Create()) as @wxEncodingConverter }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxEncodingConverter_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAllEquivalents(&self, enc: c_int, _lst: @wxList) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.handle(), enc, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getPlatformEquivalents(&self, enc: c_int, platform: c_int, _lst: @wxList) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.handle(), enc, platform, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.handle(), input_enc, output_enc, method) }
    }
}

struct wxEraseEventImpl(*u8);
impl wxEraseEvent for wxEraseEventImpl {}
impl wxEvent for wxEraseEventImpl {}
impl wxObject for wxEraseEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxEraseEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxEraseEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getDC(&self) -> @wxDC {
        unsafe { @wxDCImpl(wxEraseEvent_GetDC(self.handle())) as @wxDC }
    }
}

struct wxEventImpl(*u8);
impl wxEvent for wxEventImpl {}
impl wxObject for wxEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxEvent : wxObject {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn getEventObject(&self) -> @wxObject {
        unsafe { @wxObjectImpl(wxEvent_GetEventObject(self.handle())) as @wxObject }
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
    fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
    #[fixed_stack_segment]
    fn setEventObject(&self, obj: @wxObject) {
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

struct wxEvtHandlerImpl(*u8);
impl wxEvtHandler for wxEvtHandlerImpl {}
impl wxObject for wxEvtHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxEvtHandler : wxObject {
    #[fixed_stack_segment]
    fn addPendingEvent(&self, event: @wxEvent) {
        unsafe { wxEvtHandler_AddPendingEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn connect(&self, first: c_int, last: c_int, type_: c_int, data: *u8) -> c_int {
        unsafe { wxEvtHandler_Connect(self.handle(), first, last, type_, data) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxEvtHandler {
        unsafe { @wxEvtHandlerImpl(wxEvtHandler_Create()) as @wxEvtHandler }
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
    fn getNextHandler(&self) -> @wxEvtHandler {
        unsafe { @wxEvtHandlerImpl(wxEvtHandler_GetNextHandler(self.handle())) as @wxEvtHandler }
    }
    #[fixed_stack_segment]
    fn getPreviousHandler(&self) -> @wxEvtHandler {
        unsafe { @wxEvtHandlerImpl(wxEvtHandler_GetPreviousHandler(self.handle())) as @wxEvtHandler }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @wxEvent) -> bool {
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
    fn setNextHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxEvtHandler_SetNextHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setPreviousHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self, id: c_int, type_: c_int) -> @wxClosure {
        unsafe { @wxClosureImpl(wxEvtHandler_GetClosure(self.handle(), id, type_)) as @wxClosure }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { @wxClosureImpl(wxEvtHandler_GetClientClosure(self.handle())) as @wxClosure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @wxClosure) {
        unsafe { wxEvtHandler_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxExprImpl(*u8);
impl wxExpr for wxExprImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxExpr {
    fn handle(&self) -> *u8;
    
}

struct wxExprDatabaseImpl(*u8);
impl wxExprDatabase for wxExprDatabaseImpl {}
impl wxList for wxExprDatabaseImpl {}
impl wxObject for wxExprDatabaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxExprDatabase : wxList {
}

struct wxFFileImpl(*u8);
impl wxFFile for wxFFileImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFFile {
    fn handle(&self) -> *u8;
    
}

struct wxFFileInputStreamImpl(*u8);
impl wxFFileInputStream for wxFFileInputStreamImpl {}
impl wxInputStream for wxFFileInputStreamImpl {}
impl wxStreamBase for wxFFileInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFFileInputStream : wxInputStream {
}

struct wxFFileOutputStreamImpl(*u8);
impl wxFFileOutputStream for wxFFileOutputStreamImpl {}
impl wxOutputStream for wxFFileOutputStreamImpl {}
impl wxStreamBase for wxFFileOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFFileOutputStream : wxOutputStream {
}

struct wxFSFileImpl(*u8);
impl wxFSFile for wxFSFileImpl {}
impl wxObject for wxFSFileImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFSFile : wxObject {
}

struct wxFTPImpl(*u8);
impl wxFTP for wxFTPImpl {}
impl wxProtocol for wxFTPImpl {}
impl wxSocketClient for wxFTPImpl {}
impl wxSocketBase for wxFTPImpl {}
impl wxObject for wxFTPImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFTP : wxProtocol {
}

struct wxFileDataObjectImpl(*u8);
impl wxFileDataObject for wxFileDataObjectImpl {}
impl wxDataObjectSimple for wxFileDataObjectImpl {}
impl wxDataObject for wxFileDataObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileDataObject : wxDataObjectSimple {
}

struct wxFileDialogImpl(*u8);
impl wxFileDialog for wxFileDialogImpl {}
impl wxDialog for wxFileDialogImpl {}
impl wxTopLevelWindow for wxFileDialogImpl {}
impl wxWindow for wxFileDialogImpl {}
impl wxEvtHandler for wxFileDialogImpl {}
impl wxObject for wxFileDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _msg: @wxString, _dir: @wxString, _fle: @wxString, _wcd: @wxString, _lft: c_int, _top: c_int, _stl: c_int) -> @wxFileDialog {
        unsafe { @wxFileDialogImpl(wxFileDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _fle.handle(), _wcd.handle(), _lft, _top, _stl)) as @wxFileDialog }
    }
    #[fixed_stack_segment]
    fn getDirectory(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileDialog_GetDirectory(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileDialog_GetFilename(self.handle())) as @wxString }
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
    fn getMessage(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileDialog_GetMessage(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileDialog_GetPath(self.handle())) as @wxString }
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
    fn getWildcard(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileDialog_GetWildcard(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setDirectory(&self, dir: @wxString) {
        unsafe { wxFileDialog_SetDirectory(self.handle(), dir.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilename(&self, name: @wxString) {
        unsafe { wxFileDialog_SetFilename(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.handle(), filterIndex) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, message: @wxString) {
        unsafe { wxFileDialog_SetMessage(self.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, path: @wxString) {
        unsafe { wxFileDialog_SetPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setWildcard(&self, wildCard: @wxString) {
        unsafe { wxFileDialog_SetWildcard(self.handle(), wildCard.handle()) }
    }
}

struct wxFileDropTargetImpl(*u8);
impl wxFileDropTarget for wxFileDropTargetImpl {}
impl wxDropTarget for wxFileDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileDropTarget : wxDropTarget {
}

struct wxFileHistoryImpl(*u8);
impl wxFileHistory for wxFileHistoryImpl {}
impl wxObject for wxFileHistoryImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileHistory : wxObject {
    #[fixed_stack_segment]
    fn addFileToHistory(&self, file: @wxString) {
        unsafe { wxFileHistory_AddFileToHistory(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn addFilesToMenu(&self, menu: @wxMenu) {
        unsafe { wxFileHistory_AddFilesToMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn new(maxFiles: c_int) -> @wxFileHistory {
        unsafe { @wxFileHistoryImpl(wxFileHistory_Create(maxFiles)) as @wxFileHistory }
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
    fn getHistoryFile(&self, i: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxFileHistory_GetHistoryFile(self.handle(), i)) as @wxString }
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
    fn load(&self, config: @wxConfigBase) {
        unsafe { wxFileHistory_Load(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.handle(), i) }
    }
    #[fixed_stack_segment]
    fn removeMenu(&self, menu: @wxMenu) {
        unsafe { wxFileHistory_RemoveMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn save(&self, config: @wxConfigBase) {
        unsafe { wxFileHistory_Save(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn useMenu(&self, menu: @wxMenu) {
        unsafe { wxFileHistory_UseMenu(self.handle(), menu.handle()) }
    }
}

struct wxFileInputStreamImpl(*u8);
impl wxFileInputStream for wxFileInputStreamImpl {}
impl wxInputStream for wxFileInputStreamImpl {}
impl wxStreamBase for wxFileInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileInputStream : wxInputStream {
}

struct wxFileNameImpl(*u8);
impl wxFileName for wxFileNameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileName {
    fn handle(&self) -> *u8;
    
}

struct wxFileOutputStreamImpl(*u8);
impl wxFileOutputStream for wxFileOutputStreamImpl {}
impl wxOutputStream for wxFileOutputStreamImpl {}
impl wxStreamBase for wxFileOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileOutputStream : wxOutputStream {
}

struct wxFileSystemImpl(*u8);
impl wxFileSystem for wxFileSystemImpl {}
impl wxObject for wxFileSystemImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileSystem : wxObject {
}

struct wxFileSystemHandlerImpl(*u8);
impl wxFileSystemHandler for wxFileSystemHandlerImpl {}
impl wxObject for wxFileSystemHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileSystemHandler : wxObject {
}

struct wxFileTypeImpl(*u8);
impl wxFileType for wxFileTypeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileType {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn expandCommand(&self, _cmd: *u8, _params: *u8) -> @wxString {
        unsafe { @wxStringImpl(wxFileType_ExpandCommand(self.handle(), _cmd, _params)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getDescription(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileType_GetDescription(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getExtensions(&self, _lst: @wxList) -> c_int {
        unsafe { wxFileType_GetExtensions(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, icon: @wxIcon) -> c_int {
        unsafe { wxFileType_GetIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn getMimeType(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFileType_GetMimeType(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getMimeTypes(&self, _lst: @wxList) -> c_int {
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

struct wxFilterInputStreamImpl(*u8);
impl wxFilterInputStream for wxFilterInputStreamImpl {}
impl wxInputStream for wxFilterInputStreamImpl {}
impl wxStreamBase for wxFilterInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFilterInputStream : wxInputStream {
}

struct wxFilterOutputStreamImpl(*u8);
impl wxFilterOutputStream for wxFilterOutputStreamImpl {}
impl wxOutputStream for wxFilterOutputStreamImpl {}
impl wxStreamBase for wxFilterOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFilterOutputStream : wxOutputStream {
}

struct wxFindDialogEventImpl(*u8);
impl wxFindDialogEvent for wxFindDialogEventImpl {}
impl wxCommandEvent for wxFindDialogEventImpl {}
impl wxEvent for wxFindDialogEventImpl {}
impl wxObject for wxFindDialogEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFindDialogEvent : wxCommandEvent {
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

struct wxFindReplaceDataImpl(*u8);
impl wxFindReplaceData for wxFindReplaceDataImpl {}
impl wxObject for wxFindReplaceDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFindReplaceData : wxObject {
    #[fixed_stack_segment]
    fn new(flags: c_int) -> @wxFindReplaceData {
        unsafe { @wxFindReplaceDataImpl(wxFindReplaceData_Create(flags)) as @wxFindReplaceData }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxFindReplaceData {
        unsafe { @wxFindReplaceDataImpl(wxFindReplaceData_CreateDefault()) as @wxFindReplaceData }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFindReplaceData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFindString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFindReplaceData_GetFindString(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFindReplaceData_GetReplaceString(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setFindString(&self, str: @wxString) {
        unsafe { wxFindReplaceData_SetFindString(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn setReplaceString(&self, str: @wxString) {
        unsafe { wxFindReplaceData_SetReplaceString(self.handle(), str.handle()) }
    }
}

struct wxFindReplaceDialogImpl(*u8);
impl wxFindReplaceDialog for wxFindReplaceDialogImpl {}
impl wxDialog for wxFindReplaceDialogImpl {}
impl wxTopLevelWindow for wxFindReplaceDialogImpl {}
impl wxWindow for wxFindReplaceDialogImpl {}
impl wxEvtHandler for wxFindReplaceDialogImpl {}
impl wxObject for wxFindReplaceDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFindReplaceDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, data: @wxFindReplaceData, title: @wxString, style: c_int) -> @wxFindReplaceDialog {
        unsafe { @wxFindReplaceDialogImpl(wxFindReplaceDialog_Create(parent.handle(), data.handle(), title.handle(), style)) as @wxFindReplaceDialog }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> @wxFindReplaceData {
        unsafe { @wxFindReplaceDataImpl(wxFindReplaceDialog_GetData(self.handle())) as @wxFindReplaceData }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @wxFindReplaceData) {
        unsafe { wxFindReplaceDialog_SetData(self.handle(), data.handle()) }
    }
}

struct wxFlexGridSizerImpl(*u8);
impl wxFlexGridSizer for wxFlexGridSizerImpl {}
impl wxGridSizer for wxFlexGridSizerImpl {}
impl wxSizer for wxFlexGridSizerImpl {}
impl wxObject for wxFlexGridSizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFlexGridSizer : wxGridSizer {
    #[fixed_stack_segment]
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxFlexGridSizer_CalcMin(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @wxFlexGridSizer {
        unsafe { @wxFlexGridSizerImpl(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) as @wxFlexGridSizer }
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

struct wxFocusEventImpl(*u8);
impl wxFocusEvent for wxFocusEventImpl {}
impl wxEvent for wxFocusEventImpl {}
impl wxObject for wxFocusEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFocusEvent : wxEvent {
}

struct wxFontImpl(*u8);
impl wxFont for wxFontImpl {}
impl wxGDIObject for wxFontImpl {}
impl wxObject for wxFontImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFont : wxGDIObject {
    #[fixed_stack_segment]
    fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: bool, face: @wxString, enc: c_int) -> @wxFont {
        unsafe { @wxFontImpl(wxFont_Create(pointSize, family, style, weight, underlined, face.handle(), enc)) as @wxFont }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: c_int) -> @wxFont {
        unsafe { @wxFontImpl(wxFont_CreateFromStock(id)) as @wxFont }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxFont {
        unsafe { @wxFontImpl(wxFont_CreateDefault()) as @wxFont }
    }
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
    fn getFaceName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFont_GetFaceName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFamilyString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFont_GetFamilyString(self.handle())) as @wxString }
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
    fn getStyleString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFont_GetStyleString(self.handle())) as @wxString }
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
    fn getWeightString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFont_GetWeightString(self.handle())) as @wxString }
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
    fn setFaceName(&self, faceName: @wxString) {
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

struct wxFontDataImpl(*u8);
impl wxFontData for wxFontDataImpl {}
impl wxObject for wxFontDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFontData : wxObject {
    #[fixed_stack_segment]
    fn new() -> @wxFontData {
        unsafe { @wxFontDataImpl(wxFontData_Create()) as @wxFontData }
    }
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
    fn getChosenFont(&self, ref_: @wxFont) {
        unsafe { wxFontData_GetChosenFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @wxColour) {
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
    fn getInitialFont(&self, ref_: @wxFont) {
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
    fn setChosenFont(&self, font: @wxFont) {
        unsafe { wxFontData_SetChosenFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @wxColour) {
        unsafe { wxFontData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    fn setInitialFont(&self, font: @wxFont) {
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

struct wxFontDialogImpl(*u8);
impl wxFontDialog for wxFontDialogImpl {}
impl wxDialog for wxFontDialogImpl {}
impl wxTopLevelWindow for wxFontDialogImpl {}
impl wxWindow for wxFontDialogImpl {}
impl wxEvtHandler for wxFontDialogImpl {}
impl wxObject for wxFontDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFontDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, fnt: @wxFontData) -> @wxFontDialog {
        unsafe { @wxFontDialogImpl(wxFontDialog_Create(_prt.handle(), fnt.handle())) as @wxFontDialog }
    }
    #[fixed_stack_segment]
    fn getFontData(&self, _ref: @wxFontData) {
        unsafe { wxFontDialog_GetFontData(self.handle(), _ref.handle()) }
    }
}

struct wxFontEnumeratorImpl(*u8);
impl wxFontEnumerator for wxFontEnumeratorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFontEnumerator {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(_obj: *u8, _fnc: *u8) -> @wxFontEnumerator {
        unsafe { @wxFontEnumeratorImpl(wxFontEnumerator_Create(_obj, _fnc)) as @wxFontEnumerator }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateEncodings(&self, facename: @wxString) -> bool {
        unsafe { wxFontEnumerator_EnumerateEncodings(self.handle(), facename.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> bool {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.handle(), encoding, fixedWidthOnly) }
    }
}

struct wxFontListImpl(*u8);
impl wxFontList for wxFontListImpl {}
impl wxList for wxFontListImpl {}
impl wxObject for wxFontListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFontList : wxList {
}

struct wxFontMapperImpl(*u8);
impl wxFontMapper for wxFontMapperImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFontMapper {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxFontMapper {
        unsafe { @wxFontMapperImpl(wxFontMapper_Create()) as @wxFontMapper }
    }
    #[fixed_stack_segment]
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *u8, _buf: @wxString) -> bool {
        unsafe { wxFontMapper_GetAltForEncoding(self.handle(), encoding, alt_encoding, _buf.handle()) }
    }
    #[fixed_stack_segment]
    fn isEncodingAvailable(&self, encoding: c_int, _buf: @wxString) -> bool {
        unsafe { wxFontMapper_IsEncodingAvailable(self.handle(), encoding, _buf.handle()) }
    }
}

struct wxFrameImpl(*u8);
impl wxFrame for wxFrameImpl {}
impl wxTopLevelWindow for wxFrameImpl {}
impl wxWindow for wxFrameImpl {}
impl wxEvtHandler for wxFrameImpl {}
impl wxObject for wxFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFrame : wxTopLevelWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxFrame {
        unsafe { @wxFrameImpl(wxFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxFrame }
    }
    #[fixed_stack_segment]
    fn newStatusBar(&self, number: c_int, style: c_int) -> @wxStatusBar {
        unsafe { @wxStatusBarImpl(wxFrame_CreateStatusBar(self.handle(), number, style)) as @wxStatusBar }
    }
    #[fixed_stack_segment]
    fn newToolBar(&self, style: c_long) -> @wxToolBar {
        unsafe { @wxToolBarImpl(wxFrame_CreateToolBar(self.handle(), style)) as @wxToolBar }
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
    fn getMenuBar(&self) -> @wxMenuBar {
        unsafe { @wxMenuBarImpl(wxFrame_GetMenuBar(self.handle())) as @wxMenuBar }
    }
    #[fixed_stack_segment]
    fn getStatusBar(&self) -> @wxStatusBar {
        unsafe { @wxStatusBarImpl(wxFrame_GetStatusBar(self.handle())) as @wxStatusBar }
    }
    #[fixed_stack_segment]
    fn getToolBar(&self) -> @wxToolBar {
        unsafe { @wxToolBarImpl(wxFrame_GetToolBar(self.handle())) as @wxToolBar }
    }
    #[fixed_stack_segment]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMenuBar(&self, menubar: @wxMenuBar) {
        unsafe { wxFrame_SetMenuBar(self.handle(), menubar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusBar(&self, statBar: @wxStatusBar) {
        unsafe { wxFrame_SetStatusBar(self.handle(), statBar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusText(&self, _txt: @wxString, _number: c_int) {
        unsafe { wxFrame_SetStatusText(self.handle(), _txt.handle(), _number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, _n: c_int, _widths_field: *u8) {
        unsafe { wxFrame_SetStatusWidths(self.handle(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    fn setToolBar(&self, _toolbar: @wxToolBar) {
        unsafe { wxFrame_SetToolBar(self.handle(), _toolbar.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { @wxStringImpl(wxFrame_GetTitle(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, _txt: @wxString) {
        unsafe { wxFrame_SetTitle(self.handle(), _txt.handle()) }
    }
    #[fixed_stack_segment]
    fn setShape(&self, region: @wxRegion) -> bool {
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

struct wxFrameLayoutImpl(*u8);
impl wxFrameLayout for wxFrameLayoutImpl {}
impl wxEvtHandler for wxFrameLayoutImpl {}
impl wxObject for wxFrameLayoutImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFrameLayout : wxEvtHandler {
    #[fixed_stack_segment]
    fn activate(&self) {
        unsafe { wxFrameLayout_Activate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn addBar(&self, pBarWnd: *u8, dimInfo: *u8, alignment: c_int, rowNo: c_int, columnPos: c_int, name: *wchar_t, spyEvents: c_int, state: c_int) {
        unsafe { wxFrameLayout_AddBar(self.handle(), pBarWnd, dimInfo, alignment, rowNo, columnPos, name, spyEvents, state) }
    }
    #[fixed_stack_segment]
    fn addPlugin(&self, pPlInfo: *u8, paneMask: c_int) {
        unsafe { wxFrameLayout_AddPlugin(self.handle(), pPlInfo, paneMask) }
    }
    #[fixed_stack_segment]
    fn addPluginBefore(&self, pNextPlInfo: *u8, pPlInfo: *u8, paneMask: c_int) {
        unsafe { wxFrameLayout_AddPluginBefore(self.handle(), pNextPlInfo, pPlInfo, paneMask) }
    }
    #[fixed_stack_segment]
    fn applyBarProperties(&self, pBar: *u8) {
        unsafe { wxFrameLayout_ApplyBarProperties(self.handle(), pBar) }
    }
    #[fixed_stack_segment]
    fn captureEventsForPane(&self, toPane: *u8) {
        unsafe { wxFrameLayout_CaptureEventsForPane(self.handle(), toPane) }
    }
    #[fixed_stack_segment]
    fn captureEventsForPlugin(&self, pPlugin: *u8) {
        unsafe { wxFrameLayout_CaptureEventsForPlugin(self.handle(), pPlugin) }
    }
    #[fixed_stack_segment]
    fn new(pParentFrame: *u8, pFrameClient: *u8, activateNow: c_int) -> @wxFrameLayout {
        unsafe { @wxFrameLayoutImpl(wxFrameLayout_Create(pParentFrame, pFrameClient, activateNow)) as @wxFrameLayout }
    }
    #[fixed_stack_segment]
    fn deactivate(&self) {
        unsafe { wxFrameLayout_Deactivate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFrameLayout_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn destroyBarWindows(&self) {
        unsafe { wxFrameLayout_DestroyBarWindows(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableFloating(&self, enable: bool) {
        unsafe { wxFrameLayout_EnableFloating(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn findBarByName(&self, name: *wchar_t) -> *u8 {
        unsafe { wxFrameLayout_FindBarByName(self.handle(), name) }
    }
    #[fixed_stack_segment]
    fn findBarByWindow(&self, pWnd: *u8) -> *u8 {
        unsafe { wxFrameLayout_FindBarByWindow(self.handle(), pWnd) }
    }
    #[fixed_stack_segment]
    fn findPlugin(&self, pPlInfo: *u8) -> *u8 {
        unsafe { wxFrameLayout_FindPlugin(self.handle(), pPlInfo) }
    }
    #[fixed_stack_segment]
    fn firePluginEvent(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_FirePluginEvent(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn getBars(&self, _ref: *u8) -> c_int {
        unsafe { wxFrameLayout_GetBars(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn getClientHeight(&self) -> c_int {
        unsafe { wxFrameLayout_GetClientHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientRect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxFrameLayout_GetClientRect(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn getClientWidth(&self) -> c_int {
        unsafe { wxFrameLayout_GetClientWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrameClient(&self) -> *u8 {
        unsafe { wxFrameLayout_GetFrameClient(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPane(&self, alignment: c_int) -> *u8 {
        unsafe { wxFrameLayout_GetPane(self.handle(), alignment) }
    }
    #[fixed_stack_segment]
    fn getPaneProperties(&self, props: *u8, alignment: c_int) {
        unsafe { wxFrameLayout_GetPaneProperties(self.handle(), props, alignment) }
    }
    #[fixed_stack_segment]
    fn getParentFrame(&self) -> *u8 {
        unsafe { wxFrameLayout_GetParentFrame(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTopPlugin(&self) -> *u8 {
        unsafe { wxFrameLayout_GetTopPlugin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUpdatesManager(&self) -> *u8 {
        unsafe { wxFrameLayout_GetUpdatesManager(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasTopPlugin(&self) -> bool {
        unsafe { wxFrameLayout_HasTopPlugin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hideBarWindows(&self) {
        unsafe { wxFrameLayout_HideBarWindows(self.handle()) }
    }
    #[fixed_stack_segment]
    fn inverseVisibility(&self, pBar: *u8) {
        unsafe { wxFrameLayout_InverseVisibility(self.handle(), pBar) }
    }
    #[fixed_stack_segment]
    fn onLButtonDown(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnLButtonDown(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn onLButtonUp(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnLButtonUp(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn onLDblClick(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnLDblClick(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn onMouseMove(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnMouseMove(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn onRButtonDown(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnRButtonDown(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn onRButtonUp(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnRButtonUp(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn onSize(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnSize(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn popAllPlugins(&self) {
        unsafe { wxFrameLayout_PopAllPlugins(self.handle()) }
    }
    #[fixed_stack_segment]
    fn popPlugin(&self) {
        unsafe { wxFrameLayout_PopPlugin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn pushDefaultPlugins(&self) {
        unsafe { wxFrameLayout_PushDefaultPlugins(self.handle()) }
    }
    #[fixed_stack_segment]
    fn pushPlugin(&self, pPugin: *u8) {
        unsafe { wxFrameLayout_PushPlugin(self.handle(), pPugin) }
    }
    #[fixed_stack_segment]
    fn recalcLayout(&self, repositionBarsNow: c_int) {
        unsafe { wxFrameLayout_RecalcLayout(self.handle(), repositionBarsNow) }
    }
    #[fixed_stack_segment]
    fn redockBar(&self, pBar: *u8, x: c_int, y: c_int, w: c_int, h: c_int, pToPane: *u8, updateNow: c_int) -> c_int {
        unsafe { wxFrameLayout_RedockBar(self.handle(), pBar, x, y, w, h, pToPane, updateNow) }
    }
    #[fixed_stack_segment]
    fn refreshNow(&self, recalcLayout: c_int) {
        unsafe { wxFrameLayout_RefreshNow(self.handle(), recalcLayout) }
    }
    #[fixed_stack_segment]
    fn releaseEventsFromPane(&self, fromPane: *u8) {
        unsafe { wxFrameLayout_ReleaseEventsFromPane(self.handle(), fromPane) }
    }
    #[fixed_stack_segment]
    fn releaseEventsFromPlugin(&self, pPlugin: *u8) {
        unsafe { wxFrameLayout_ReleaseEventsFromPlugin(self.handle(), pPlugin) }
    }
    #[fixed_stack_segment]
    fn removeBar(&self, pBar: *u8) {
        unsafe { wxFrameLayout_RemoveBar(self.handle(), pBar) }
    }
    #[fixed_stack_segment]
    fn removePlugin(&self, pPlInfo: *u8) {
        unsafe { wxFrameLayout_RemovePlugin(self.handle(), pPlInfo) }
    }
    #[fixed_stack_segment]
    fn setBarState(&self, pBar: *u8, newStatem: c_int, updateNow: c_int) {
        unsafe { wxFrameLayout_SetBarState(self.handle(), pBar, newStatem, updateNow) }
    }
    #[fixed_stack_segment]
    fn setFrameClient(&self, pFrameClient: *u8) {
        unsafe { wxFrameLayout_SetFrameClient(self.handle(), pFrameClient) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, top: c_int, bottom: c_int, left: c_int, right: c_int, paneMask: c_int) {
        unsafe { wxFrameLayout_SetMargins(self.handle(), top, bottom, left, right, paneMask) }
    }
    #[fixed_stack_segment]
    fn setPaneBackground(&self, colour: @wxColour) {
        unsafe { wxFrameLayout_SetPaneBackground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setPaneProperties(&self, props: *u8, paneMask: c_int) {
        unsafe { wxFrameLayout_SetPaneProperties(self.handle(), props, paneMask) }
    }
    #[fixed_stack_segment]
    fn setTopPlugin(&self, pPlugin: *u8) {
        unsafe { wxFrameLayout_SetTopPlugin(self.handle(), pPlugin) }
    }
    #[fixed_stack_segment]
    fn setUpdatesManager(&self, pUMgr: *u8) {
        unsafe { wxFrameLayout_SetUpdatesManager(self.handle(), pUMgr) }
    }
}

struct wxGDIObjectImpl(*u8);
impl wxGDIObject for wxGDIObjectImpl {}
impl wxObject for wxGDIObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGDIObject : wxObject {
}

struct wxGLCanvasImpl(*u8);
impl wxGLCanvas for wxGLCanvasImpl {}
impl wxScrolledWindow for wxGLCanvasImpl {}
impl wxPanel for wxGLCanvasImpl {}
impl wxWindow for wxGLCanvasImpl {}
impl wxEvtHandler for wxGLCanvasImpl {}
impl wxObject for wxGLCanvasImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGLCanvas : wxScrolledWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, windowID: c_int, attributes: *c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: @wxString, palette: @wxPalette) -> @wxGLCanvas {
        unsafe { @wxGLCanvasImpl(wxGLCanvas_Create(parent.handle(), windowID, attributes, x, y, w, h, style, title.handle(), palette.handle())) as @wxGLCanvas }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @wxColour) -> bool {
        unsafe { wxGLCanvas_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, ctxt: @wxGLContext) -> bool {
        unsafe { wxGLCanvas_SetCurrent(self.handle(), ctxt.handle()) }
    }
    #[fixed_stack_segment]
    fn swapBuffers(&self) -> bool {
        unsafe { wxGLCanvas_SwapBuffers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isDisplaySupported(attributes: *c_int) -> bool {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    #[fixed_stack_segment]
    fn isExtensionSupported(extension: @wxString) -> bool {
        unsafe { wxGLCanvas_IsExtensionSupported(extension.handle()) }
    }
}

struct wxGaugeImpl(*u8);
impl wxGauge for wxGaugeImpl {}
impl wxControl for wxGaugeImpl {}
impl wxWindow for wxGaugeImpl {}
impl wxEvtHandler for wxGaugeImpl {}
impl wxObject for wxGaugeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGauge : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxGauge {
        unsafe { @wxGaugeImpl(wxGauge_Create(_prt.handle(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) as @wxGauge }
    }
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

struct wxGenericDirCtrlImpl(*u8);
impl wxGenericDirCtrl for wxGenericDirCtrlImpl {}
impl wxControl for wxGenericDirCtrlImpl {}
impl wxWindow for wxGenericDirCtrlImpl {}
impl wxEvtHandler for wxGenericDirCtrlImpl {}
impl wxObject for wxGenericDirCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGenericDirCtrl : wxControl {
}

struct wxGenericValidatorImpl(*u8);
impl wxGenericValidator for wxGenericValidatorImpl {}
impl wxValidator for wxGenericValidatorImpl {}
impl wxEvtHandler for wxGenericValidatorImpl {}
impl wxObject for wxGenericValidatorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGenericValidator : wxValidator {
}

struct wxGridImpl(*u8);
impl wxGrid for wxGridImpl {}
impl wxScrolledWindow for wxGridImpl {}
impl wxPanel for wxGridImpl {}
impl wxWindow for wxGridImpl {}
impl wxEvtHandler for wxGridImpl {}
impl wxObject for wxGridImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGrid : wxScrolledWindow {
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
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> @wxRect {
        unsafe { @wxRectImpl(wxGrid_BlockToDeviceRect(self.handle(), top, left, bottom, right)) as @wxRect }
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
    fn cellToRect(&self, row: c_int, col: c_int) -> @wxRect {
        unsafe { @wxRectImpl(wxGrid_CellToRect(self.handle(), row, col)) as @wxRect }
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
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxGrid {
        unsafe { @wxGridImpl(wxGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxGrid }
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
    fn drawAllGridLines(&self, dc: @wxDC, reg: @wxRegion) {
        unsafe { wxGrid_DrawAllGridLines(self.handle(), dc.handle(), reg.handle()) }
    }
    #[fixed_stack_segment]
    fn drawCell(&self, dc: @wxDC, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellBorder(&self, dc: @wxDC, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellHighlight(&self, dc: @wxDC, attr: @wxGridCellAttr) {
        unsafe { wxGrid_DrawCellHighlight(self.handle(), dc.handle(), attr.handle()) }
    }
    #[fixed_stack_segment]
    fn drawColLabel(&self, dc: @wxDC, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.handle(), dc.handle(), col) }
    }
    #[fixed_stack_segment]
    fn drawColLabels(&self, dc: @wxDC) {
        unsafe { wxGrid_DrawColLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawGridSpace(&self, dc: @wxDC) {
        unsafe { wxGrid_DrawGridSpace(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawRowLabel(&self, dc: @wxDC, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.handle(), dc.handle(), row) }
    }
    #[fixed_stack_segment]
    fn drawRowLabels(&self, dc: @wxDC) {
        unsafe { wxGrid_DrawRowLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawTextRectangle(&self, dc: @wxDC, txt: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
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
    fn getCellBackgroundColour(&self, row: c_int, col: c_int, colour: @wxColour) {
        unsafe { wxGrid_GetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellEditor(&self, row: c_int, col: c_int) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditorImpl(wxGrid_GetCellEditor(self.handle(), row, col)) as @wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getCellFont(&self, row: c_int, col: c_int, font: @wxFont) {
        unsafe { wxGrid_GetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellHighlightColour(&self, _ref: @wxColour) {
        unsafe { wxGrid_GetCellHighlightColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellRenderer(&self, row: c_int, col: c_int) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRendererImpl(wxGrid_GetCellRenderer(self.handle(), row, col)) as @wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getCellTextColour(&self, row: c_int, col: c_int, colour: @wxColour) {
        unsafe { wxGrid_GetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellValue(&self, row: c_int, col: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxGrid_GetCellValue(self.handle(), row, col)) as @wxString }
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
    fn getColLabelValue(&self, col: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxGrid_GetColLabelValue(self.handle(), col)) as @wxString }
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
    fn getDefaultCellBackgroundColour(&self, _ref: @wxColour) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellFont(&self, _ref: @wxFont) {
        unsafe { wxGrid_GetDefaultCellFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellTextColour(&self, _ref: @wxColour) {
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
    fn getDefaultEditor(&self) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditorImpl(wxGrid_GetDefaultEditor(self.handle())) as @wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditorImpl(wxGrid_GetDefaultEditorForCell(self.handle(), row, col)) as @wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForType(&self, typeName: @wxString) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditorImpl(wxGrid_GetDefaultEditorForType(self.handle(), typeName.handle())) as @wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRendererImpl(wxGrid_GetDefaultRenderer(self.handle())) as @wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRendererImpl(wxGrid_GetDefaultRendererForCell(self.handle(), row, col)) as @wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForType(&self, typeName: @wxString) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRendererImpl(wxGrid_GetDefaultRendererForType(self.handle(), typeName.handle())) as @wxGridCellRenderer }
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
    fn getGridLineColour(&self, _ref: @wxColour) {
        unsafe { wxGrid_GetGridLineColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelBackgroundColour(&self, _ref: @wxColour) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelFont(&self, _ref: @wxFont) {
        unsafe { wxGrid_GetLabelFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelTextColour(&self, _ref: @wxColour) {
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
    fn getRowLabelValue(&self, row: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxGrid_GetRowLabelValue(self.handle(), row)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.handle(), row) }
    }
    #[fixed_stack_segment]
    fn getSelectionBackground(&self, _ref: @wxColour) {
        unsafe { wxGrid_GetSelectionBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionForeground(&self, _ref: @wxColour) {
        unsafe { wxGrid_GetSelectionForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTable(&self) -> @wxGridTableBase {
        unsafe { @wxGridTableBaseImpl(wxGrid_GetTable(self.handle())) as @wxGridTableBase }
    }
    #[fixed_stack_segment]
    fn getTextBoxSize(&self, dc: @wxDC, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int) {
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
    fn processTableMessage(&self, evt: @wxEvent) -> bool {
        unsafe { wxGrid_ProcessTableMessage(self.handle(), evt.handle()) }
    }
    #[fixed_stack_segment]
    fn registerDataType(&self, typeName: @wxString, renderer: @wxGridCellRenderer, editor: @wxGridCellEditor) {
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
    fn setCellBackgroundColour(&self, row: c_int, col: c_int, colour: @wxColour) {
        unsafe { wxGrid_SetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellEditor(&self, row: c_int, col: c_int, editor: @wxGridCellEditor) {
        unsafe { wxGrid_SetCellEditor(self.handle(), row, col, editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellFont(&self, row: c_int, col: c_int, font: @wxFont) {
        unsafe { wxGrid_SetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellHighlightColour(&self, col: @wxColour) {
        unsafe { wxGrid_SetCellHighlightColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellRenderer(&self, row: c_int, col: c_int, renderer: @wxGridCellRenderer) {
        unsafe { wxGrid_SetCellRenderer(self.handle(), row, col, renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellTextColour(&self, row: c_int, col: c_int, colour: @wxColour) {
        unsafe { wxGrid_SetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellValue(&self, row: c_int, col: c_int, s: @wxString) {
        unsafe { wxGrid_SetCellValue(self.handle(), row, col, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setColAttr(&self, col: c_int, attr: @wxGridCellAttr) {
        unsafe { wxGrid_SetColAttr(self.handle(), col, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setColFormatCustom(&self, col: c_int, typeName: @wxString) {
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
    fn setColLabelValue(&self, col: c_int, label: @wxString) {
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
    fn setDefaultCellBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellFont(&self, font: @wxFont) {
        unsafe { wxGrid_SetDefaultCellFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellTextColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.handle(), width, resizeExistingCols) }
    }
    #[fixed_stack_segment]
    fn setDefaultEditor(&self, editor: @wxGridCellEditor) {
        unsafe { wxGrid_SetDefaultEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultRenderer(&self, renderer: @wxGridCellRenderer) {
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
    fn setGridLineColour(&self, col: @wxColour) {
        unsafe { wxGrid_SetGridLineColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelFont(&self, font: @wxFont) {
        unsafe { wxGrid_SetLabelFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTextColour(&self, colour: @wxColour) {
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
    fn setRowAttr(&self, row: c_int, attr: @wxGridCellAttr) {
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
    fn setRowLabelValue(&self, row: c_int, label: @wxString) {
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
    fn setSelectionBackground(&self, c: @wxColour) {
        unsafe { wxGrid_SetSelectionBackground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionForeground(&self, c: @wxColour) {
        unsafe { wxGrid_SetSelectionForeground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.handle(), selmode) }
    }
    #[fixed_stack_segment]
    fn setTable(&self, table: @wxGridTableBase, takeOwnership: bool, selmode: c_int) -> bool {
        unsafe { wxGrid_SetTable(self.handle(), table.handle(), takeOwnership, selmode) }
    }
    #[fixed_stack_segment]
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn stringToLines(&self, value: @wxString, lines: *u8) -> c_int {
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
    fn getSelectedCells(&self, _arr: @wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectedCells(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockTopLeft(&self, _arr: @wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockBottomRight(&self, _arr: @wxGridCellCoordsArray) {
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

struct wxGridCellAttrImpl(*u8);
impl wxGridCellAttr for wxGridCellAttrImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellAttr {
        unsafe { @wxGridCellAttrImpl(wxGridCellAttr_Ctor()) as @wxGridCellAttr }
    }
    #[fixed_stack_segment]
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self, hAlign: *c_int, vAlign: *c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.handle(), hAlign, vAlign) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @wxColour) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditor(&self, grid: @wxGrid, row: c_int, col: c_int) -> @wxGridCellEditor {
        unsafe { @wxGridCellEditorImpl(wxGridCellAttr_GetEditor(self.handle(), grid.handle(), row, col)) as @wxGridCellEditor }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @wxFont) {
        unsafe { wxGridCellAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getRenderer(&self, grid: @wxGrid, row: c_int, col: c_int) -> @wxGridCellRenderer {
        unsafe { @wxGridCellRendererImpl(wxGridCellAttr_GetRenderer(self.handle(), grid.handle(), row, col)) as @wxGridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @wxColour) {
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
    fn setBackgroundColour(&self, colBack: @wxColour) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefAttr(&self, defAttr: @wxGridCellAttr) {
        unsafe { wxGridCellAttr_SetDefAttr(self.handle(), defAttr.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditor(&self, editor: @wxGridCellEditor) {
        unsafe { wxGridCellAttr_SetEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxGridCellAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, isReadOnly: bool) {
        unsafe { wxGridCellAttr_SetReadOnly(self.handle(), isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRenderer(&self, renderer: @wxGridCellRenderer) {
        unsafe { wxGridCellAttr_SetRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @wxColour) {
        unsafe { wxGridCellAttr_SetTextColour(self.handle(), colText.handle()) }
    }
}

struct wxGridCellBoolEditorImpl(*u8);
impl wxGridCellBoolEditor for wxGridCellBoolEditorImpl {}
impl wxGridCellEditor for wxGridCellBoolEditorImpl {}
impl wxGridCellWorker for wxGridCellBoolEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellBoolEditor : wxGridCellEditor {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellBoolEditor {
        unsafe { @wxGridCellBoolEditorImpl(wxGridCellBoolEditor_Ctor()) as @wxGridCellBoolEditor }
    }
}

struct wxGridCellBoolRendererImpl(*u8);
impl wxGridCellBoolRenderer for wxGridCellBoolRendererImpl {}
impl wxGridCellRenderer for wxGridCellBoolRendererImpl {}
impl wxGridCellWorker for wxGridCellBoolRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellBoolRenderer : wxGridCellRenderer {
}

struct wxGridCellChoiceEditorImpl(*u8);
impl wxGridCellChoiceEditor for wxGridCellChoiceEditorImpl {}
impl wxGridCellEditor for wxGridCellChoiceEditorImpl {}
impl wxGridCellWorker for wxGridCellChoiceEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellChoiceEditor : wxGridCellEditor {
    #[fixed_stack_segment]
    fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> @wxGridCellChoiceEditor {
        unsafe { @wxGridCellChoiceEditorImpl(wxGridCellChoiceEditor_Ctor(count, choices, allowOthers)) as @wxGridCellChoiceEditor }
    }
}

struct wxGridCellCoordsArrayImpl(*u8);
impl wxGridCellCoordsArray for wxGridCellCoordsArrayImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellCoordsArray {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxGridCellCoordsArray {
        unsafe { @wxGridCellCoordsArrayImpl(wxGridCellCoordsArray_Create()) as @wxGridCellCoordsArray }
    }
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

struct wxGridCellEditorImpl(*u8);
impl wxGridCellEditor for wxGridCellEditorImpl {}
impl wxGridCellWorker for wxGridCellEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellEditor : wxGridCellWorker {
    #[fixed_stack_segment]
    fn beginEdit(&self, row: c_int, col: c_int, grid: @wxGrid) {
        unsafe { wxGridCellEditor_BeginEdit(self.handle(), row, col, grid.handle()) }
    }
    #[fixed_stack_segment]
    fn new(&self, parent: @wxWindow, id: c_int, evtHandler: @wxEvtHandler) {
        unsafe { wxGridCellEditor_Create(self.handle(), parent.handle(), id, evtHandler.handle()) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endEdit(&self, row: c_int, col: c_int, grid: @wxGrid, oldStr: @wxString, newStr: @wxString) -> c_int {
        unsafe { wxGridCellEditor_EndEdit(self.handle(), row, col, grid.handle(), oldStr.handle(), newStr.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @wxControl {
        unsafe { @wxControlImpl(wxGridCellEditor_GetControl(self.handle())) as @wxControl }
    }
    #[fixed_stack_segment]
    fn handleReturn(&self, event: @wxEvent) {
        unsafe { wxGridCellEditor_HandleReturn(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isAcceptedKey(&self, event: @wxEvent) -> bool {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isCreated(&self) -> bool {
        unsafe { wxGridCellEditor_IsCreated(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paintBackground(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: @wxGridCellAttr) {
        unsafe { wxGridCellEditor_PaintBackground(self.handle(), x, y, w, h, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setControl(&self, control: @wxControl) {
        unsafe { wxGridCellEditor_SetControl(self.handle(), control.handle()) }
    }
    #[fixed_stack_segment]
    fn setParameters(&self, params: @wxString) {
        unsafe { wxGridCellEditor_SetParameters(self.handle(), params.handle()) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn show(&self, show: c_int, attr: @wxGridCellAttr) {
        unsafe { wxGridCellEditor_Show(self.handle(), show, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn startingKey(&self, event: @wxEvent) {
        unsafe { wxGridCellEditor_StartingKey(self.handle(), event.handle()) }
    }
}

struct wxGridCellFloatEditorImpl(*u8);
impl wxGridCellFloatEditor for wxGridCellFloatEditorImpl {}
impl wxGridCellTextEditor for wxGridCellFloatEditorImpl {}
impl wxGridCellEditor for wxGridCellFloatEditorImpl {}
impl wxGridCellWorker for wxGridCellFloatEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellFloatEditor : wxGridCellTextEditor {
    #[fixed_stack_segment]
    fn ctor(width: c_int, precision: c_int) -> @wxGridCellFloatEditor {
        unsafe { @wxGridCellFloatEditorImpl(wxGridCellFloatEditor_Ctor(width, precision)) as @wxGridCellFloatEditor }
    }
}

struct wxGridCellFloatRendererImpl(*u8);
impl wxGridCellFloatRenderer for wxGridCellFloatRendererImpl {}
impl wxGridCellStringRenderer for wxGridCellFloatRendererImpl {}
impl wxGridCellRenderer for wxGridCellFloatRendererImpl {}
impl wxGridCellWorker for wxGridCellFloatRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellFloatRenderer : wxGridCellStringRenderer {
}

struct wxGridCellNumberEditorImpl(*u8);
impl wxGridCellNumberEditor for wxGridCellNumberEditorImpl {}
impl wxGridCellTextEditor for wxGridCellNumberEditorImpl {}
impl wxGridCellEditor for wxGridCellNumberEditorImpl {}
impl wxGridCellWorker for wxGridCellNumberEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellNumberEditor : wxGridCellTextEditor {
    #[fixed_stack_segment]
    fn ctor(min: c_int, max: c_int) -> @wxGridCellNumberEditor {
        unsafe { @wxGridCellNumberEditorImpl(wxGridCellNumberEditor_Ctor(min, max)) as @wxGridCellNumberEditor }
    }
}

struct wxGridCellNumberRendererImpl(*u8);
impl wxGridCellNumberRenderer for wxGridCellNumberRendererImpl {}
impl wxGridCellStringRenderer for wxGridCellNumberRendererImpl {}
impl wxGridCellRenderer for wxGridCellNumberRendererImpl {}
impl wxGridCellWorker for wxGridCellNumberRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellNumberRenderer : wxGridCellStringRenderer {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellNumberRenderer {
        unsafe { @wxGridCellNumberRendererImpl(wxGridCellNumberRenderer_Ctor()) as @wxGridCellNumberRenderer }
    }
}

struct wxGridCellAutoWrapStringRendererImpl(*u8);
impl wxGridCellAutoWrapStringRenderer for wxGridCellAutoWrapStringRendererImpl {}
impl wxGridCellStringRenderer for wxGridCellAutoWrapStringRendererImpl {}
impl wxGridCellRenderer for wxGridCellAutoWrapStringRendererImpl {}
impl wxGridCellWorker for wxGridCellAutoWrapStringRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellAutoWrapStringRenderer : wxGridCellStringRenderer {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellAutoWrapStringRenderer {
        unsafe { @wxGridCellAutoWrapStringRendererImpl(wxGridCellAutoWrapStringRenderer_Ctor()) as @wxGridCellAutoWrapStringRenderer }
    }
}

struct wxGridCellRendererImpl(*u8);
impl wxGridCellRenderer for wxGridCellRendererImpl {}
impl wxGridCellWorker for wxGridCellRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellRenderer : wxGridCellWorker {
}

struct wxGridCellStringRendererImpl(*u8);
impl wxGridCellStringRenderer for wxGridCellStringRendererImpl {}
impl wxGridCellRenderer for wxGridCellStringRendererImpl {}
impl wxGridCellWorker for wxGridCellStringRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellStringRenderer : wxGridCellRenderer {
}

struct wxGridCellTextEditorImpl(*u8);
impl wxGridCellTextEditor for wxGridCellTextEditorImpl {}
impl wxGridCellEditor for wxGridCellTextEditorImpl {}
impl wxGridCellWorker for wxGridCellTextEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellTextEditor : wxGridCellEditor {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellTextEditor {
        unsafe { @wxGridCellTextEditorImpl(wxGridCellTextEditor_Ctor()) as @wxGridCellTextEditor }
    }
}

struct wxGridCellWorkerImpl(*u8);
impl wxGridCellWorker for wxGridCellWorkerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellWorker {
    fn handle(&self) -> *u8;
    
}

struct wxGridEditorCreatedEventImpl(*u8);
impl wxGridEditorCreatedEvent for wxGridEditorCreatedEventImpl {}
impl wxCommandEvent for wxGridEditorCreatedEventImpl {}
impl wxEvent for wxGridEditorCreatedEventImpl {}
impl wxObject for wxGridEditorCreatedEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridEditorCreatedEvent : wxCommandEvent {
    #[fixed_stack_segment]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @wxControl {
        unsafe { @wxControlImpl(wxGridEditorCreatedEvent_GetControl(self.handle())) as @wxControl }
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
    fn setControl(&self, ctrl: @wxControl) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.handle(), row) }
    }
}

struct wxGridEventImpl(*u8);
impl wxGridEvent for wxGridEventImpl {}
impl wxNotifyEvent for wxGridEventImpl {}
impl wxCommandEvent for wxGridEventImpl {}
impl wxEvent for wxGridEventImpl {}
impl wxObject for wxGridEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridEvent : wxNotifyEvent {
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
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxGridEvent_GetPosition(self.handle())) as @wxPoint }
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

struct wxGridRangeSelectEventImpl(*u8);
impl wxGridRangeSelectEvent for wxGridRangeSelectEventImpl {}
impl wxNotifyEvent for wxGridRangeSelectEventImpl {}
impl wxCommandEvent for wxGridRangeSelectEventImpl {}
impl wxEvent for wxGridRangeSelectEventImpl {}
impl wxObject for wxGridRangeSelectEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridRangeSelectEvent : wxNotifyEvent {
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

struct wxGridSizeEventImpl(*u8);
impl wxGridSizeEvent for wxGridSizeEventImpl {}
impl wxNotifyEvent for wxGridSizeEventImpl {}
impl wxCommandEvent for wxGridSizeEventImpl {}
impl wxEvent for wxGridSizeEventImpl {}
impl wxObject for wxGridSizeEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridSizeEvent : wxNotifyEvent {
    #[fixed_stack_segment]
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxGridSizeEvent_GetPosition(self.handle())) as @wxPoint }
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

struct wxGridSizerImpl(*u8);
impl wxGridSizer for wxGridSizerImpl {}
impl wxSizer for wxGridSizerImpl {}
impl wxObject for wxGridSizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridSizer : wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxGridSizer_CalcMin(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @wxGridSizer {
        unsafe { @wxGridSizerImpl(wxGridSizer_Create(rows, cols, vgap, hgap)) as @wxGridSizer }
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

struct wxGridTableBaseImpl(*u8);
impl wxGridTableBase for wxGridTableBaseImpl {}
impl wxObject for wxGridTableBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridTableBase : wxObject {
}

struct wxHTTPImpl(*u8);
impl wxHTTP for wxHTTPImpl {}
impl wxProtocol for wxHTTPImpl {}
impl wxSocketClient for wxHTTPImpl {}
impl wxSocketBase for wxHTTPImpl {}
impl wxObject for wxHTTPImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHTTP : wxProtocol {
}

struct wxHashMapImpl(*u8);
impl wxHashMap for wxHashMapImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHashMap {
    fn handle(&self) -> *u8;
    
}

struct wxHelpControllerImpl(*u8);
impl wxHelpController for wxHelpControllerImpl {}
impl wxHelpControllerBase for wxHelpControllerImpl {}
impl wxObject for wxHelpControllerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHelpController : wxHelpControllerBase {
}

struct wxHelpControllerBaseImpl(*u8);
impl wxHelpControllerBase for wxHelpControllerBaseImpl {}
impl wxObject for wxHelpControllerBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHelpControllerBase : wxObject {
}

struct wxHelpControllerHelpProviderImpl(*u8);
impl wxHelpControllerHelpProvider for wxHelpControllerHelpProviderImpl {}
impl wxSimpleHelpProvider for wxHelpControllerHelpProviderImpl {}
impl wxHelpProvider for wxHelpControllerHelpProviderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHelpControllerHelpProvider : wxSimpleHelpProvider {
    #[fixed_stack_segment]
    fn new(ctr: @wxHelpControllerBase) -> @wxHelpControllerHelpProvider {
        unsafe { @wxHelpControllerHelpProviderImpl(wxHelpControllerHelpProvider_Create(ctr.handle())) as @wxHelpControllerHelpProvider }
    }
    #[fixed_stack_segment]
    fn getHelpController(&self) -> @wxHelpControllerBase {
        unsafe { @wxHelpControllerBaseImpl(wxHelpControllerHelpProvider_GetHelpController(self.handle())) as @wxHelpControllerBase }
    }
    #[fixed_stack_segment]
    fn setHelpController(&self, hc: @wxHelpController) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.handle(), hc.handle()) }
    }
}

struct wxHelpEventImpl(*u8);
impl wxHelpEvent for wxHelpEventImpl {}
impl wxCommandEvent for wxHelpEventImpl {}
impl wxEvent for wxHelpEventImpl {}
impl wxObject for wxHelpEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHelpEvent : wxCommandEvent {
    #[fixed_stack_segment]
    fn getLink(&self) -> @wxString {
        unsafe { @wxStringImpl(wxHelpEvent_GetLink(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxHelpEvent_GetPosition(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @wxString {
        unsafe { @wxStringImpl(wxHelpEvent_GetTarget(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setLink(&self, link: @wxString) {
        unsafe { wxHelpEvent_SetLink(self.handle(), link.handle()) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setTarget(&self, target: @wxString) {
        unsafe { wxHelpEvent_SetTarget(self.handle(), target.handle()) }
    }
}

struct wxHelpProviderImpl(*u8);
impl wxHelpProvider for wxHelpProviderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHelpProvider {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addHelp(&self, window: @wxWindow, text: @wxString) {
        unsafe { wxHelpProvider_AddHelp(self.handle(), window.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addHelpById(&self, id: c_int, text: @wxString) {
        unsafe { wxHelpProvider_AddHelpById(self.handle(), id, text.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn get() -> @wxHelpProvider {
        unsafe { @wxHelpProviderImpl(wxHelpProvider_Get()) as @wxHelpProvider }
    }
    #[fixed_stack_segment]
    fn getHelp(&self, window: @wxWindow) -> @wxString {
        unsafe { @wxStringImpl(wxHelpProvider_GetHelp(self.handle(), window.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn removeHelp(&self, window: @wxWindow) {
        unsafe { wxHelpProvider_RemoveHelp(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self) -> @wxHelpProvider {
        unsafe { @wxHelpProviderImpl(wxHelpProvider_Set(self.handle())) as @wxHelpProvider }
    }
    #[fixed_stack_segment]
    fn showHelp(&self, window: @wxWindow) -> bool {
        unsafe { wxHelpProvider_ShowHelp(self.handle(), window.handle()) }
    }
}

struct wxHtmlCellImpl(*u8);
impl wxHtmlCell for wxHtmlCellImpl {}
impl wxObject for wxHtmlCellImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlCell : wxObject {
}

struct wxHtmlColourCellImpl(*u8);
impl wxHtmlColourCell for wxHtmlColourCellImpl {}
impl wxHtmlCell for wxHtmlColourCellImpl {}
impl wxObject for wxHtmlColourCellImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlColourCell : wxHtmlCell {
}

struct wxHtmlContainerCellImpl(*u8);
impl wxHtmlContainerCell for wxHtmlContainerCellImpl {}
impl wxHtmlCell for wxHtmlContainerCellImpl {}
impl wxObject for wxHtmlContainerCellImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlContainerCell : wxHtmlCell {
}

struct wxHtmlDCRendererImpl(*u8);
impl wxHtmlDCRenderer for wxHtmlDCRendererImpl {}
impl wxObject for wxHtmlDCRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlDCRenderer : wxObject {
}

struct wxHtmlEasyPrintingImpl(*u8);
impl wxHtmlEasyPrinting for wxHtmlEasyPrintingImpl {}
impl wxObject for wxHtmlEasyPrintingImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlEasyPrinting : wxObject {
}

struct wxHtmlFilterImpl(*u8);
impl wxHtmlFilter for wxHtmlFilterImpl {}
impl wxObject for wxHtmlFilterImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlFilter : wxObject {
}

struct wxHtmlHelpControllerImpl(*u8);
impl wxHtmlHelpController for wxHtmlHelpControllerImpl {}
impl wxHelpControllerBase for wxHtmlHelpControllerImpl {}
impl wxObject for wxHtmlHelpControllerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlHelpController : wxHelpControllerBase {
    #[fixed_stack_segment]
    fn addBook(&self, book: *u8, show_wait_msg: c_int) -> bool {
        unsafe { wxHtmlHelpController_AddBook(self.handle(), book, show_wait_msg) }
    }
    #[fixed_stack_segment]
    fn new(_style: c_int) -> @wxHtmlHelpController {
        unsafe { @wxHtmlHelpControllerImpl(wxHtmlHelpController_Create(_style)) as @wxHtmlHelpController }
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
    fn displaySection(&self, section: @wxString) -> bool {
        unsafe { wxHtmlHelpController_DisplaySection(self.handle(), section.handle()) }
    }
    #[fixed_stack_segment]
    fn displaySectionNumber(&self, sectionNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.handle(), sectionNo) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrameImpl(wxHtmlHelpController_GetFrame(self.handle())) as @wxFrame }
    }
    #[fixed_stack_segment]
    fn getFrameParameters(&self, title: *u8, width: *c_int, height: *c_int, pos_x: *c_int, pos_y: *c_int, newFrameEachTime: *c_int) -> *u8 {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, file: @wxString) -> bool {
        unsafe { wxHtmlHelpController_Initialize(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn keywordSearch(&self, keyword: @wxString) -> bool {
        unsafe { wxHtmlHelpController_KeywordSearch(self.handle(), keyword.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, file: @wxString) -> bool {
        unsafe { wxHtmlHelpController_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn quit(&self) -> bool {
        unsafe { wxHtmlHelpController_Quit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlHelpController_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setFrameParameters(&self, title: *u8, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: bool) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn setTempDir(&self, path: @wxString) {
        unsafe { wxHtmlHelpController_SetTempDir(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitleFormat(&self, format: *u8) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.handle(), format) }
    }
    #[fixed_stack_segment]
    fn setViewer(&self, viewer: @wxString, flags: c_int) {
        unsafe { wxHtmlHelpController_SetViewer(self.handle(), viewer.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn useConfig(&self, config: @wxConfigBase, rootpath: @wxString) {
        unsafe { wxHtmlHelpController_UseConfig(self.handle(), config.handle(), rootpath.handle()) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlHelpController_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxHtmlHelpDataImpl(*u8);
impl wxHtmlHelpData for wxHtmlHelpDataImpl {}
impl wxObject for wxHtmlHelpDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlHelpData : wxObject {
}

struct wxHtmlHelpFrameImpl(*u8);
impl wxHtmlHelpFrame for wxHtmlHelpFrameImpl {}
impl wxFrame for wxHtmlHelpFrameImpl {}
impl wxTopLevelWindow for wxHtmlHelpFrameImpl {}
impl wxWindow for wxHtmlHelpFrameImpl {}
impl wxEvtHandler for wxHtmlHelpFrameImpl {}
impl wxObject for wxHtmlHelpFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlHelpFrame : wxFrame {
}

struct wxHtmlLinkInfoImpl(*u8);
impl wxHtmlLinkInfo for wxHtmlLinkInfoImpl {}
impl wxObject for wxHtmlLinkInfoImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlLinkInfo : wxObject {
}

struct wxHtmlParserImpl(*u8);
impl wxHtmlParser for wxHtmlParserImpl {}
impl wxObject for wxHtmlParserImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlParser : wxObject {
}

struct wxHtmlPrintoutImpl(*u8);
impl wxHtmlPrintout for wxHtmlPrintoutImpl {}
impl wxPrintout for wxHtmlPrintoutImpl {}
impl wxObject for wxHtmlPrintoutImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlPrintout : wxPrintout {
}

struct wxHtmlTagImpl(*u8);
impl wxHtmlTag for wxHtmlTagImpl {}
impl wxObject for wxHtmlTagImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlTag : wxObject {
}

struct wxHtmlTagHandlerImpl(*u8);
impl wxHtmlTagHandler for wxHtmlTagHandlerImpl {}
impl wxObject for wxHtmlTagHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlTagHandler : wxObject {
}

struct wxHtmlTagsModuleImpl(*u8);
impl wxHtmlTagsModule for wxHtmlTagsModuleImpl {}
impl wxModule for wxHtmlTagsModuleImpl {}
impl wxObject for wxHtmlTagsModuleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlTagsModule : wxModule {
}

struct wxHtmlWidgetCellImpl(*u8);
impl wxHtmlWidgetCell for wxHtmlWidgetCellImpl {}
impl wxHtmlCell for wxHtmlWidgetCellImpl {}
impl wxObject for wxHtmlWidgetCellImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlWidgetCell : wxHtmlCell {
}

struct wxHtmlWinParserImpl(*u8);
impl wxHtmlWinParser for wxHtmlWinParserImpl {}
impl wxHtmlParser for wxHtmlWinParserImpl {}
impl wxObject for wxHtmlWinParserImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlWinParser : wxHtmlParser {
}

struct wxHtmlWinTagHandlerImpl(*u8);
impl wxHtmlWinTagHandler for wxHtmlWinTagHandlerImpl {}
impl wxHtmlTagHandler for wxHtmlWinTagHandlerImpl {}
impl wxObject for wxHtmlWinTagHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlWinTagHandler : wxHtmlTagHandler {
}

struct wxHtmlWindowImpl(*u8);
impl wxHtmlWindow for wxHtmlWindowImpl {}
impl wxScrolledWindow for wxHtmlWindowImpl {}
impl wxPanel for wxHtmlWindowImpl {}
impl wxWindow for wxHtmlWindowImpl {}
impl wxEvtHandler for wxHtmlWindowImpl {}
impl wxObject for wxHtmlWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxHtmlWindow : wxScrolledWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: @wxString) -> @wxHtmlWindow {
        unsafe { @wxHtmlWindowImpl(wxHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) as @wxHtmlWindow }
    }
    #[fixed_stack_segment]
    fn appendToPage(&self, source: @wxString) -> bool {
        unsafe { wxHtmlWindow_AppendToPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn getInternalRepresentation(&self) -> @wxHtmlContainerCell {
        unsafe { @wxHtmlContainerCellImpl(wxHtmlWindow_GetInternalRepresentation(self.handle())) as @wxHtmlContainerCell }
    }
    #[fixed_stack_segment]
    fn getOpenedAnchor(&self) -> @wxString {
        unsafe { @wxStringImpl(wxHtmlWindow_GetOpenedAnchor(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getOpenedPage(&self) -> @wxString {
        unsafe { @wxStringImpl(wxHtmlWindow_GetOpenedPage(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getOpenedPageTitle(&self) -> @wxString {
        unsafe { @wxStringImpl(wxHtmlWindow_GetOpenedPageTitle(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getRelatedFrame(&self) -> @wxFrame {
        unsafe { @wxFrameImpl(wxHtmlWindow_GetRelatedFrame(self.handle())) as @wxFrame }
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
    fn loadPage(&self, location: @wxString) -> bool {
        unsafe { wxHtmlWindow_LoadPage(self.handle(), location.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlWindow_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.handle(), b) }
    }
    #[fixed_stack_segment]
    fn setFonts(&self, normal_face: @wxString, fixed_face: @wxString, sizes: *c_int) {
        unsafe { wxHtmlWindow_SetFonts(self.handle(), normal_face.handle(), fixed_face.handle(), sizes) }
    }
    #[fixed_stack_segment]
    fn setPage(&self, source: @wxString) {
        unsafe { wxHtmlWindow_SetPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedFrame(&self, frame: @wxFrame, format: @wxString) {
        unsafe { wxHtmlWindow_SetRelatedFrame(self.handle(), frame.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.handle(), bar) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlWindow_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxIPV4addressImpl(*u8);
impl wxIPV4address for wxIPV4addressImpl {}
impl wxSockAddress for wxIPV4addressImpl {}
impl wxObject for wxIPV4addressImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIPV4address : wxSockAddress {
}

struct wxIconImpl(*u8);
impl wxIcon for wxIconImpl {}
impl wxBitmap for wxIconImpl {}
impl wxGDIObject for wxIconImpl {}
impl wxObject for wxIconImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIcon : wxBitmap {
    #[fixed_stack_segment]
    fn assign(&self, other: *u8) {
        unsafe { wxIcon_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn copyFromBitmap(&self, bmp: @wxBitmap) {
        unsafe { wxIcon_CopyFromBitmap(self.handle(), bmp.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxIcon {
        unsafe { @wxIconImpl(wxIcon_CreateDefault()) as @wxIcon }
    }
    #[fixed_stack_segment]
    fn newLoad(name: @wxString, type_: c_long, width: c_int, height: c_int) -> @wxIcon {
        unsafe { @wxIconImpl(wxIcon_CreateLoad(name.handle(), type_, width, height)) as @wxIcon }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIcon_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn fromRaw(&self, width: c_int, height: c_int) -> @wxIcon {
        unsafe { @wxIconImpl(wxIcon_FromRaw(self.handle(), width, height)) as @wxIcon }
    }
    #[fixed_stack_segment]
    fn fromXPM(&self) -> @wxIcon {
        unsafe { @wxIconImpl(wxIcon_FromXPM(self.handle())) as @wxIcon }
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
    fn isEqual(&self, other: @wxIcon) -> bool {
        unsafe { wxIcon_IsEqual(self.handle(), other.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, name: @wxString, type_: c_long, width: c_int, height: c_int) -> c_int {
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

struct wxIconBundleImpl(*u8);
impl wxIconBundle for wxIconBundleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIconBundle {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @wxIcon) {
        unsafe { wxIconBundle_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addIconFromFile(&self, file: @wxString, type_: c_int) {
        unsafe { wxIconBundle_AddIconFromFile(self.handle(), file.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn assign(&self, _ref: @wxIconBundle) {
        unsafe { wxIconBundle_Assign(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxIconBundle {
        unsafe { @wxIconBundleImpl(wxIconBundle_CreateDefault()) as @wxIconBundle }
    }
    #[fixed_stack_segment]
    fn newFromFile(file: @wxString, type_: c_int) -> @wxIconBundle {
        unsafe { @wxIconBundleImpl(wxIconBundle_CreateFromFile(file.handle(), type_)) as @wxIconBundle }
    }
    #[fixed_stack_segment]
    fn newFromIcon(icon: @wxIcon) -> @wxIconBundle {
        unsafe { @wxIconBundleImpl(wxIconBundle_CreateFromIcon(icon.handle())) as @wxIconBundle }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, w: c_int, h: c_int, _ref: @wxIcon) {
        unsafe { wxIconBundle_GetIcon(self.handle(), w, h, _ref.handle()) }
    }
}

struct wxIconizeEventImpl(*u8);
impl wxIconizeEvent for wxIconizeEventImpl {}
impl wxEvent for wxIconizeEventImpl {}
impl wxObject for wxIconizeEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIconizeEvent : wxEvent {
}

struct wxIdleEventImpl(*u8);
impl wxIdleEvent for wxIdleEventImpl {}
impl wxEvent for wxIdleEventImpl {}
impl wxObject for wxIdleEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIdleEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: @wxObject) {
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

struct wxImageImpl(*u8);
impl wxImage for wxImageImpl {}
impl wxObject for wxImageImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxImage : wxObject {
    #[fixed_stack_segment]
    fn canRead(name: @wxString) -> bool {
        unsafe { wxImage_CanRead(name.handle()) }
    }
    #[fixed_stack_segment]
    fn convertToBitmap(&self, bitmap: @wxBitmap) {
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
    fn newDefault() -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateDefault()) as @wxImage }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(bitmap: @wxBitmap) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateFromBitmap(bitmap.handle())) as @wxImage }
    }
    #[fixed_stack_segment]
    fn newFromByteString(data: *char, length: c_int, type_: c_int) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateFromByteString(data, length, type_)) as @wxImage }
    }
    #[fixed_stack_segment]
    fn newFromLazyByteString(data: *char, length: c_int, type_: c_int) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateFromLazyByteString(data, length, type_)) as @wxImage }
    }
    #[fixed_stack_segment]
    fn newFromData(width: c_int, height: c_int, data: *u8) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateFromData(width, height, data)) as @wxImage }
    }
    #[fixed_stack_segment]
    fn newFromFile(name: @wxString) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateFromFile(name.handle())) as @wxImage }
    }
    #[fixed_stack_segment]
    fn newSized(width: c_int, height: c_int) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateSized(width, height)) as @wxImage }
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
    fn getSubImage(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: @wxImage) {
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
    fn getOption(&self, name: @wxString) -> @wxString {
        unsafe { @wxStringImpl(wxImage_GetOption(self.handle(), name.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getOptionInt(&self, name: @wxString) -> bool {
        unsafe { wxImage_GetOptionInt(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn hasOption(&self, name: @wxString) -> bool {
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
    fn loadFile(&self, name: @wxString, type_: c_int) -> bool {
        unsafe { wxImage_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn mirror(&self, horizontally: c_int, image: @wxImage) {
        unsafe { wxImage_Mirror(self.handle(), horizontally, image.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxImage_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paste(&self, image: @wxImage, x: c_int, y: c_int) {
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
    fn rotate(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *u8, image: @wxImage) {
        unsafe { wxImage_Rotate(self.handle(), angle, c_x, c_y, interpolating, offset_after_rotation, image.handle()) }
    }
    #[fixed_stack_segment]
    fn rotate90(&self, clockwise: c_int, image: @wxImage) {
        unsafe { wxImage_Rotate90(self.handle(), clockwise, image.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @wxString, type_: c_int) -> bool {
        unsafe { wxImage_SaveFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn scale(&self, width: c_int, height: c_int, image: @wxImage) {
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
    fn setOption(&self, name: @wxString, value: @wxString) {
        unsafe { wxImage_SetOption(self.handle(), name.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setOptionInt(&self, name: @wxString, value: c_int) {
        unsafe { wxImage_SetOptionInt(self.handle(), name.handle(), value) }
    }
    #[fixed_stack_segment]
    fn setRGB(&self, x: c_int, y: c_int, r: uint8_t, g: uint8_t, b: uint8_t) {
        unsafe { wxImage_SetRGB(self.handle(), x, y, r, g, b) }
    }
    #[fixed_stack_segment]
    fn newFromDataEx(width: c_int, height: c_int, data: *u8, isStaticData: c_int) -> @wxImage {
        unsafe { @wxImageImpl(wxImage_CreateFromDataEx(width, height, data, isStaticData)) as @wxImage }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImage_Delete(self.handle()) }
    }
}

struct wxImageHandlerImpl(*u8);
impl wxImageHandler for wxImageHandlerImpl {}
impl wxObject for wxImageHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxImageHandler : wxObject {
}

struct wxImageListImpl(*u8);
impl wxImageList for wxImageListImpl {}
impl wxObject for wxImageListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxImageList : wxObject {
    #[fixed_stack_segment]
    fn addBitmap(&self, bitmap: @wxBitmap, mask: @wxBitmap) -> c_int {
        unsafe { wxImageList_AddBitmap(self.handle(), bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @wxIcon) -> c_int {
        unsafe { wxImageList_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addMasked(&self, bitmap: @wxBitmap, maskColour: @wxColour) -> c_int {
        unsafe { wxImageList_AddMasked(self.handle(), bitmap.handle(), maskColour.handle()) }
    }
    #[fixed_stack_segment]
    fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> @wxImageList {
        unsafe { @wxImageListImpl(wxImageList_Create(width, height, mask, initialCount)) as @wxImageList }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImageList_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn draw(&self, index: c_int, dc: @wxDC, x: c_int, y: c_int, flags: c_int, solidBackground: bool) -> bool {
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
    fn replace(&self, index: c_int, bitmap: @wxBitmap, mask: @wxBitmap) -> bool {
        unsafe { wxImageList_Replace(self.handle(), index, bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceIcon(&self, index: c_int, icon: @wxIcon) -> bool {
        unsafe { wxImageList_ReplaceIcon(self.handle(), index, icon.handle()) }
    }
}

struct wxIndividualLayoutConstraintImpl(*u8);
impl wxIndividualLayoutConstraint for wxIndividualLayoutConstraintImpl {}
impl wxObject for wxIndividualLayoutConstraintImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIndividualLayoutConstraint : wxObject {
    #[fixed_stack_segment]
    fn above(&self, sibling: @wxWindow, marg: c_int) {
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
    fn below(&self, sibling: @wxWindow, marg: c_int) {
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
    fn leftOf(&self, sibling: @wxWindow, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn percentOf(&self, otherW: @wxWindow, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.handle(), otherW.handle(), wh, per) }
    }
    #[fixed_stack_segment]
    fn resetIfWin(&self, otherW: @wxWindow) -> bool {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.handle(), otherW.handle()) }
    }
    #[fixed_stack_segment]
    fn rightOf(&self, sibling: @wxWindow, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn sameAs(&self, otherW: @wxWindow, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.handle(), otherW.handle(), edge, marg) }
    }
    #[fixed_stack_segment]
    fn satisfyConstraint(&self, constraints: *u8, win: @wxWindow) -> bool {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.handle(), constraints, win.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, rel: c_int, otherW: @wxWindow, otherE: c_int, val: c_int, marg: c_int) {
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

struct wxInitDialogEventImpl(*u8);
impl wxInitDialogEvent for wxInitDialogEventImpl {}
impl wxEvent for wxInitDialogEventImpl {}
impl wxObject for wxInitDialogEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxInitDialogEvent : wxEvent {
}

struct wxInputStreamImpl(*u8);
impl wxInputStream for wxInputStreamImpl {}
impl wxStreamBase for wxInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxInputStream : wxStreamBase {
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

struct wxJoystickImpl(*u8);
impl wxJoystick for wxJoystickImpl {}
impl wxObject for wxJoystickImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxJoystick : wxObject {
    #[fixed_stack_segment]
    fn new(joystick: c_int) -> @wxJoystick {
        unsafe { @wxJoystickImpl(wxJoystick_Create(joystick)) as @wxJoystick }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxJoystick_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getButtonState(&self) -> c_int {
        unsafe { wxJoystick_GetButtonState(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getManufacturerId(&self) -> c_int {
        unsafe { wxJoystick_GetManufacturerId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaxAxes(&self) -> c_int {
        unsafe { wxJoystick_GetMaxAxes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMaxButtons(&self) -> c_int {
        unsafe { wxJoystick_GetMaxButtons(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMovementThreshold(&self) -> c_int {
        unsafe { wxJoystick_GetMovementThreshold(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getNumberAxes(&self) -> c_int {
        unsafe { wxJoystick_GetNumberAxes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getNumberButtons(&self) -> c_int {
        unsafe { wxJoystick_GetNumberButtons(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getNumberJoysticks(&self) -> c_int {
        unsafe { wxJoystick_GetNumberJoysticks(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPOVCTSPosition(&self) -> c_int {
        unsafe { wxJoystick_GetPOVCTSPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPOVPosition(&self) -> c_int {
        unsafe { wxJoystick_GetPOVPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPollingMax(&self) -> c_int {
        unsafe { wxJoystick_GetPollingMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPollingMin(&self) -> c_int {
        unsafe { wxJoystick_GetPollingMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxJoystick_GetPosition(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getProductId(&self) -> c_int {
        unsafe { wxJoystick_GetProductId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getProductName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxJoystick_GetProductName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getRudderMax(&self) -> c_int {
        unsafe { wxJoystick_GetRudderMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRudderMin(&self) -> c_int {
        unsafe { wxJoystick_GetRudderMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRudderPosition(&self) -> c_int {
        unsafe { wxJoystick_GetRudderPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUMax(&self) -> c_int {
        unsafe { wxJoystick_GetUMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUMin(&self) -> c_int {
        unsafe { wxJoystick_GetUMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUPosition(&self) -> c_int {
        unsafe { wxJoystick_GetUPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVMax(&self) -> c_int {
        unsafe { wxJoystick_GetVMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVMin(&self) -> c_int {
        unsafe { wxJoystick_GetVMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVPosition(&self) -> c_int {
        unsafe { wxJoystick_GetVPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getXMax(&self) -> c_int {
        unsafe { wxJoystick_GetXMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getXMin(&self) -> c_int {
        unsafe { wxJoystick_GetXMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getYMax(&self) -> c_int {
        unsafe { wxJoystick_GetYMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getYMin(&self) -> c_int {
        unsafe { wxJoystick_GetYMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getZMax(&self) -> c_int {
        unsafe { wxJoystick_GetZMax(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getZMin(&self) -> c_int {
        unsafe { wxJoystick_GetZMin(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getZPosition(&self) -> c_int {
        unsafe { wxJoystick_GetZPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasPOV(&self) -> bool {
        unsafe { wxJoystick_HasPOV(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasPOV4Dir(&self) -> bool {
        unsafe { wxJoystick_HasPOV4Dir(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasPOVCTS(&self) -> bool {
        unsafe { wxJoystick_HasPOVCTS(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasRudder(&self) -> bool {
        unsafe { wxJoystick_HasRudder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasU(&self) -> bool {
        unsafe { wxJoystick_HasU(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasV(&self) -> bool {
        unsafe { wxJoystick_HasV(self.handle()) }
    }
    #[fixed_stack_segment]
    fn hasZ(&self) -> bool {
        unsafe { wxJoystick_HasZ(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxJoystick_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn releaseCapture(&self) -> c_int {
        unsafe { wxJoystick_ReleaseCapture(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCapture(&self, win: @wxWindow, pollingFreq: c_int) -> c_int {
        unsafe { wxJoystick_SetCapture(self.handle(), win.handle(), pollingFreq) }
    }
    #[fixed_stack_segment]
    fn setMovementThreshold(&self, threshold: c_int) {
        unsafe { wxJoystick_SetMovementThreshold(self.handle(), threshold) }
    }
}

struct wxJoystickEventImpl(*u8);
impl wxJoystickEvent for wxJoystickEventImpl {}
impl wxEvent for wxJoystickEventImpl {}
impl wxObject for wxJoystickEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxJoystickEvent : wxEvent {
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
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxJoystickEvent_GetPosition(self.handle())) as @wxPoint }
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

struct wxKeyEventImpl(*u8);
impl wxKeyEvent for wxKeyEventImpl {}
impl wxEvent for wxKeyEventImpl {}
impl wxObject for wxKeyEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxKeyEvent : wxEvent {
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
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxKeyEvent_GetPosition(self.handle())) as @wxPoint }
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

struct wxLEDNumberCtrlImpl(*u8);
impl wxLEDNumberCtrl for wxLEDNumberCtrlImpl {}
impl wxControl for wxLEDNumberCtrlImpl {}
impl wxWindow for wxLEDNumberCtrlImpl {}
impl wxEvtHandler for wxLEDNumberCtrlImpl {}
impl wxObject for wxLEDNumberCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLEDNumberCtrl : wxControl {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxLEDNumberCtrl {
        unsafe { @wxLEDNumberCtrlImpl(wxLEDNumberCtrl_Create(parent.handle(), id, x, y, w, h, style)) as @wxLEDNumberCtrl }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> c_int {
        unsafe { wxLEDNumberCtrl_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDrawFaded(&self) -> c_int {
        unsafe { wxLEDNumberCtrl_GetDrawFaded(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getValue(&self, _ref: *u8) -> c_int {
        unsafe { wxLEDNumberCtrl_GetValue(self.handle(), _ref) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, Alignment: c_int, Redraw: c_int) {
        unsafe { wxLEDNumberCtrl_SetAlignment(self.handle(), Alignment, Redraw) }
    }
    #[fixed_stack_segment]
    fn setDrawFaded(&self, DrawFaded: c_int, Redraw: c_int) {
        unsafe { wxLEDNumberCtrl_SetDrawFaded(self.handle(), DrawFaded, Redraw) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, Value: *u8, Redraw: c_int) {
        unsafe { wxLEDNumberCtrl_SetValue(self.handle(), Value, Redraw) }
    }
}

struct wxLayoutAlgorithmImpl(*u8);
impl wxLayoutAlgorithm for wxLayoutAlgorithmImpl {}
impl wxObject for wxLayoutAlgorithmImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLayoutAlgorithm : wxObject {
    #[fixed_stack_segment]
    fn new() -> @wxLayoutAlgorithm {
        unsafe { @wxLayoutAlgorithmImpl(wxLayoutAlgorithm_Create()) as @wxLayoutAlgorithm }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLayoutAlgorithm_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layoutFrame(&self, frame: @wxFrame, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.handle(), frame.handle(), mainWindow) }
    }
    #[fixed_stack_segment]
    fn layoutMDIFrame(&self, frame: @wxFrame, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.handle(), frame.handle(), x, y, w, h, use_) }
    }
    #[fixed_stack_segment]
    fn layoutWindow(&self, frame: @wxFrame, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.handle(), frame.handle(), mainWindow) }
    }
}

struct wxLayoutConstraintsImpl(*u8);
impl wxLayoutConstraints for wxLayoutConstraintsImpl {}
impl wxObject for wxLayoutConstraintsImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLayoutConstraints : wxObject {
    #[fixed_stack_segment]
    fn new() -> @wxLayoutConstraints {
        unsafe { @wxLayoutConstraintsImpl(wxLayoutConstraints_Create()) as @wxLayoutConstraints }
    }
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

struct wxListImpl(*u8);
impl wxList for wxListImpl {}
impl wxObject for wxListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxList : wxObject {
}

struct wxListBoxImpl(*u8);
impl wxListBox for wxListBoxImpl {}
impl wxControl for wxListBoxImpl {}
impl wxWindow for wxListBoxImpl {}
impl wxEvtHandler for wxListBoxImpl {}
impl wxObject for wxListBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxListBox : wxControl {
    #[fixed_stack_segment]
    fn append(&self, item: @wxString) {
        unsafe { wxListBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @wxString, data: *u8) {
        unsafe { wxListBox_AppendData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxListBox_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @wxListBox {
        unsafe { @wxListBoxImpl(wxListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @wxListBox }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: c_int) {
        unsafe { wxListBox_Delete(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> c_int {
        unsafe { wxListBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxListBox_GetClientData(self.handle(), n)) as @wxClientData }
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
    fn getString(&self, n: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxListBox_GetString(self.handle(), n)) as @wxString }
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
    fn setClientData(&self, n: c_int, clientData: @wxClientData) {
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
    fn setString(&self, n: c_int, s: @wxString) {
        unsafe { wxListBox_SetString(self.handle(), n, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setStringSelection(&self, str: @wxString, sel: bool) {
        unsafe { wxListBox_SetStringSelection(self.handle(), str.handle(), sel) }
    }
}

struct wxListCtrlImpl(*u8);
impl wxListCtrl for wxListCtrlImpl {}
impl wxControl for wxListCtrlImpl {}
impl wxWindow for wxListCtrlImpl {}
impl wxEvtHandler for wxListCtrlImpl {}
impl wxObject for wxListCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxListCtrl : wxControl {
    #[fixed_stack_segment]
    fn arrange(&self, flag: c_int) -> bool {
        unsafe { wxListCtrl_Arrange(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxListCtrl {
        unsafe { @wxListCtrlImpl(wxListCtrl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxListCtrl }
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
    fn findItem(&self, start: c_int, str: @wxString, partial: bool) -> c_int {
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
    fn getColumn(&self, col: c_int, item: @wxListItem) -> bool {
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
    fn getEditControl(&self) -> @wxTextCtrl {
        unsafe { @wxTextCtrlImpl(wxListCtrl_GetEditControl(self.handle())) as @wxTextCtrl }
    }
    #[fixed_stack_segment]
    fn getImageList(&self, which: c_int) -> @wxImageList {
        unsafe { @wxImageListImpl(wxListCtrl_GetImageList(self.handle(), which)) as @wxImageList }
    }
    #[fixed_stack_segment]
    fn getItem(&self, info: @wxListItem) -> bool {
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
    fn getItemFont(&self, item: c_long) -> @wxFont {
        unsafe { @wxFontImpl(wxListCtrl_GetItemFont(self.handle(), item)) as @wxFont }
    }
    #[fixed_stack_segment]
    fn getItemPosition(&self, item: c_int) -> @wxPoint {
        unsafe { @wxPointImpl(wxListCtrl_GetItemPosition(self.handle(), item)) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getItemRect(&self, item: c_int, code: c_int) -> @wxRect {
        unsafe { @wxRectImpl(wxListCtrl_GetItemRect(self.handle(), item, code)) as @wxRect }
    }
    #[fixed_stack_segment]
    fn getItemSpacing(&self, isSmall: bool) -> @wxSize {
        unsafe { @wxSizeImpl(wxListCtrl_GetItemSpacing(self.handle(), isSmall)) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.handle(), item, stateMask) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxListCtrl_GetItemText(self.handle(), item)) as @wxString }
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
    fn getTextColour(&self, _ref: @wxColour) {
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
    fn insertColumn(&self, col: c_int, heading: @wxString, format: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_InsertColumn(self.handle(), col, heading.handle(), format, width) }
    }
    #[fixed_stack_segment]
    fn insertColumnFromInfo(&self, col: c_int, info: @wxListItem) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.handle(), col, info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, info: @wxListItem) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithData(&self, index: c_int, label: @wxString) -> c_int {
        unsafe { wxListCtrl_InsertItemWithData(self.handle(), index, label.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.handle(), index, imageIndex) }
    }
    #[fixed_stack_segment]
    fn insertItemWithLabel(&self, index: c_int, label: @wxString, imageIndex: c_int) -> c_int {
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
    fn setBackgroundColour(&self, col: @wxColour) {
        unsafe { wxListCtrl_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumn(&self, col: c_int, item: @wxListItem) -> bool {
        unsafe { wxListCtrl_SetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> bool {
        unsafe { wxListCtrl_SetColumnWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, col: @wxColour) -> c_int {
        unsafe { wxListCtrl_SetForegroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @wxImageList, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.handle(), imageList.handle(), which) }
    }
    #[fixed_stack_segment]
    fn setItem(&self, index: c_int, col: c_int, label: @wxString, imageId: c_int) -> bool {
        unsafe { wxListCtrl_SetItem(self.handle(), index, col, label.handle(), imageId) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: c_int, data: c_int) -> bool {
        unsafe { wxListCtrl_SetItemData(self.handle(), item, data) }
    }
    #[fixed_stack_segment]
    fn setItemFromInfo(&self, info: @wxListItem) -> bool {
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
    fn setItemText(&self, item: c_int, str: @wxString) {
        unsafe { wxListCtrl_SetItemText(self.handle(), item, str.handle()) }
    }
    #[fixed_stack_segment]
    fn setSingleStyle(&self, style: c_int, add: bool) {
        unsafe { wxListCtrl_SetSingleStyle(self.handle(), style, add) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @wxColour) {
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
    fn assignImageList(&self, images: @wxImageList, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.handle(), images.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getColumn2(&self, col: c_int, item: @wxListItem) {
        unsafe { wxListCtrl_GetColumn2(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem2(&self, info: @wxListItem) {
        unsafe { wxListCtrl_GetItem2(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemPosition2(&self, item: c_int) -> @wxPoint {
        unsafe { @wxPointImpl(wxListCtrl_GetItemPosition2(self.handle(), item)) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn sortItems2(&self, closure: @wxClosure) -> bool {
        unsafe { wxListCtrl_SortItems2(self.handle(), closure.handle()) }
    }
}

struct wxListEventImpl(*u8);
impl wxListEvent for wxListEventImpl {}
impl wxNotifyEvent for wxListEventImpl {}
impl wxCommandEvent for wxListEventImpl {}
impl wxEvent for wxListEventImpl {}
impl wxObject for wxListEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxListEvent : wxNotifyEvent {
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
    fn getItem(&self, _ref: @wxListItem) {
        unsafe { wxListEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { @wxStringImpl(wxListEvent_GetLabel(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxListEvent_GetPoint(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxListEvent_GetText(self.handle())) as @wxString }
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

struct wxListItemImpl(*u8);
impl wxListItem for wxListItemImpl {}
impl wxObject for wxListItemImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxListItem : wxObject {
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxListItem_Clear(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearAttributes(&self) {
        unsafe { wxListItem_ClearAttributes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxListItem {
        unsafe { @wxListItemImpl(wxListItem_Create()) as @wxListItem }
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
    fn getBackgroundColour(&self, _ref: @wxColour) {
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
    fn getFont(&self, _ref: @wxFont) {
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
    fn getText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxListItem_GetText(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @wxColour) {
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
    fn setBackgroundColour(&self, colBack: @wxColour) {
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
    fn setFont(&self, font: @wxFont) {
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
    fn setText(&self, text: @wxString) {
        unsafe { wxListItem_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @wxColour) {
        unsafe { wxListItem_SetTextColour(self.handle(), colText.handle()) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.handle(), width) }
    }
}

struct wxLocaleImpl(*u8);
impl wxLocale for wxLocaleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLocale {
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
    fn new(_name: c_int, _flags: c_int) -> @wxLocale {
        unsafe { @wxLocaleImpl(wxLocale_Create(_name, _flags)) as @wxLocale }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLocale_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLocale(&self) -> @wxLocale {
        unsafe { @wxLocaleImpl(wxLocale_GetLocale(self.handle())) as @wxLocale }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxLocale_GetName(self.handle())) as @wxString }
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

struct wxLogImpl(*u8);
impl wxLog for wxLogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLog {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: @wxString) {
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
    fn getActiveTarget() -> @wxLog {
        unsafe { @wxLogImpl(wxLog_GetActiveTarget()) as @wxLog }
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
    fn isAllowedTraceMask(&self, mask: @wxMask) -> bool {
        unsafe { wxLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: c_int, szString: *wchar_t, t: c_int) {
        unsafe { wxLog_OnLog(self.handle(), level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask(&self, str: @wxString) {
        unsafe { wxLog_RemoveTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxLog_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) -> @wxLog {
        unsafe { @wxLogImpl(wxLog_SetActiveTarget(self.handle())) as @wxLog }
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

struct wxLogChainImpl(*u8);
impl wxLogChain for wxLogChainImpl {}
impl wxLog for wxLogChainImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogChain : wxLog {
    #[fixed_stack_segment]
    fn new(logger: @wxLog) -> @wxLogChain {
        unsafe { @wxLogChainImpl(wxLogChain_Create(logger.handle())) as @wxLogChain }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLogChain_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOldLog(&self) -> @wxLog {
        unsafe { @wxLogImpl(wxLogChain_GetOldLog(self.handle())) as @wxLog }
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
    fn setLog(&self, logger: @wxLog) {
        unsafe { wxLogChain_SetLog(self.handle(), logger.handle()) }
    }
}

struct wxLogGUIImpl(*u8);
impl wxLogGUI for wxLogGUIImpl {}
impl wxLog for wxLogGUIImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogGUI : wxLog {
}

struct wxLogNullImpl(*u8);
impl wxLogNull for wxLogNullImpl {}
impl wxLog for wxLogNullImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogNull : wxLog {
    #[fixed_stack_segment]
    fn new() -> @wxLogNull {
        unsafe { @wxLogNullImpl(wxLogNull_Create()) as @wxLogNull }
    }
}

struct wxLogPassThroughImpl(*u8);
impl wxLogPassThrough for wxLogPassThroughImpl {}
impl wxLogChain for wxLogPassThroughImpl {}
impl wxLog for wxLogPassThroughImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogPassThrough : wxLogChain {
}

struct wxLogStderrImpl(*u8);
impl wxLogStderr for wxLogStderrImpl {}
impl wxLog for wxLogStderrImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogStderr : wxLog {
    #[fixed_stack_segment]
    fn new() -> @wxLogStderr {
        unsafe { @wxLogStderrImpl(wxLogStderr_Create()) as @wxLogStderr }
    }
    #[fixed_stack_segment]
    fn newStdOut() -> @wxLogStderr {
        unsafe { @wxLogStderrImpl(wxLogStderr_CreateStdOut()) as @wxLogStderr }
    }
}

struct wxLogStreamImpl(*u8);
impl wxLogStream for wxLogStreamImpl {}
impl wxLog for wxLogStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogStream : wxLog {
}

struct wxLogTextCtrlImpl(*u8);
impl wxLogTextCtrl for wxLogTextCtrlImpl {}
impl wxLog for wxLogTextCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogTextCtrl : wxLog {
    #[fixed_stack_segment]
    fn new(text: @wxTextCtrl) -> @wxLogTextCtrl {
        unsafe { @wxLogTextCtrlImpl(wxLogTextCtrl_Create(text.handle())) as @wxLogTextCtrl }
    }
}

struct wxLogWindowImpl(*u8);
impl wxLogWindow for wxLogWindowImpl {}
impl wxLogPassThrough for wxLogWindowImpl {}
impl wxLogChain for wxLogWindowImpl {}
impl wxLog for wxLogWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLogWindow : wxLogPassThrough {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, title: *wchar_t, showit: bool, passthrough: bool) -> @wxLogWindow {
        unsafe { @wxLogWindowImpl(wxLogWindow_Create(parent.handle(), title, showit, passthrough)) as @wxLogWindow }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrameImpl(wxLogWindow_GetFrame(self.handle())) as @wxFrame }
    }
}

struct wxLongLongImpl(*u8);
impl wxLongLong for wxLongLongImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxLongLong {
    fn handle(&self) -> *u8;
    
}

struct wxMBConvImpl(*u8);
impl wxMBConv for wxMBConvImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMBConv {
    fn handle(&self) -> *u8;
    
}

struct wxMBConvFileImpl(*u8);
impl wxMBConvFile for wxMBConvFileImpl {}
impl wxMBConv for wxMBConvFileImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMBConvFile : wxMBConv {
}

struct wxMBConvUTF7Impl(*u8);
impl wxMBConvUTF7 for wxMBConvUTF7Impl {}
impl wxMBConv for wxMBConvUTF7Impl { pub fn handle(&self) -> *u8 { **self } }

trait wxMBConvUTF7 : wxMBConv {
}

struct wxMBConvUTF8Impl(*u8);
impl wxMBConvUTF8 for wxMBConvUTF8Impl {}
impl wxMBConv for wxMBConvUTF8Impl { pub fn handle(&self) -> *u8 { **self } }

trait wxMBConvUTF8 : wxMBConv {
}

struct wxMDIChildFrameImpl(*u8);
impl wxMDIChildFrame for wxMDIChildFrameImpl {}
impl wxFrame for wxMDIChildFrameImpl {}
impl wxTopLevelWindow for wxMDIChildFrameImpl {}
impl wxWindow for wxMDIChildFrameImpl {}
impl wxEvtHandler for wxMDIChildFrameImpl {}
impl wxObject for wxMDIChildFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMDIChildFrame : wxFrame {
    #[fixed_stack_segment]
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMDIChildFrame {
        unsafe { @wxMDIChildFrameImpl(wxMDIChildFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxMDIChildFrame }
    }
}

struct wxMDIClientWindowImpl(*u8);
impl wxMDIClientWindow for wxMDIClientWindowImpl {}
impl wxWindow for wxMDIClientWindowImpl {}
impl wxEvtHandler for wxMDIClientWindowImpl {}
impl wxObject for wxMDIClientWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMDIClientWindow : wxWindow {
}

struct wxMDIParentFrameImpl(*u8);
impl wxMDIParentFrame for wxMDIParentFrameImpl {}
impl wxFrame for wxMDIParentFrameImpl {}
impl wxTopLevelWindow for wxMDIParentFrameImpl {}
impl wxWindow for wxMDIParentFrameImpl {}
impl wxEvtHandler for wxMDIParentFrameImpl {}
impl wxObject for wxMDIParentFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMDIParentFrame : wxFrame {
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
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMDIParentFrame {
        unsafe { @wxMDIParentFrameImpl(wxMDIParentFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxMDIParentFrame }
    }
    #[fixed_stack_segment]
    fn getActiveChild(&self) -> @wxMDIChildFrame {
        unsafe { @wxMDIChildFrameImpl(wxMDIParentFrame_GetActiveChild(self.handle())) as @wxMDIChildFrame }
    }
    #[fixed_stack_segment]
    fn getClientWindow(&self) -> @wxMDIClientWindow {
        unsafe { @wxMDIClientWindowImpl(wxMDIParentFrame_GetClientWindow(self.handle())) as @wxMDIClientWindow }
    }
    #[fixed_stack_segment]
    fn getWindowMenu(&self) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMDIParentFrame_GetWindowMenu(self.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn onCreateClient(&self) -> @wxMDIClientWindow {
        unsafe { @wxMDIClientWindowImpl(wxMDIParentFrame_OnCreateClient(self.handle())) as @wxMDIClientWindow }
    }
    #[fixed_stack_segment]
    fn setWindowMenu(&self, menu: @wxMenu) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.handle()) }
    }
}

struct wxMaskImpl(*u8);
impl wxMask for wxMaskImpl {}
impl wxObject for wxMaskImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMask : wxObject {
    #[fixed_stack_segment]
    fn new(bitmap: @wxBitmap) -> @wxMask {
        unsafe { @wxMaskImpl(wxMask_Create(bitmap.handle())) as @wxMask }
    }
    #[fixed_stack_segment]
    fn newColoured(bitmap: @wxBitmap, colour: @wxColour) -> *u8 {
        unsafe { wxMask_CreateColoured(bitmap.handle(), colour.handle()) }
    }
}

struct wxMaximizeEventImpl(*u8);
impl wxMaximizeEvent for wxMaximizeEventImpl {}
impl wxEvent for wxMaximizeEventImpl {}
impl wxObject for wxMaximizeEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMaximizeEvent : wxEvent {
}

struct wxMemoryDCImpl(*u8);
impl wxMemoryDC for wxMemoryDCImpl {}
impl wxDC for wxMemoryDCImpl {}
impl wxObject for wxMemoryDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMemoryDC : wxDC {
    #[fixed_stack_segment]
    fn new() -> @wxMemoryDC {
        unsafe { @wxMemoryDCImpl(wxMemoryDC_Create()) as @wxMemoryDC }
    }
    #[fixed_stack_segment]
    fn newCompatible(dc: @wxDC) -> @wxMemoryDC {
        unsafe { @wxMemoryDCImpl(wxMemoryDC_CreateCompatible(dc.handle())) as @wxMemoryDC }
    }
    #[fixed_stack_segment]
    fn newWithBitmap(bitmap: @wxBitmap) -> @wxMemoryDC {
        unsafe { @wxMemoryDCImpl(wxMemoryDC_CreateWithBitmap(bitmap.handle())) as @wxMemoryDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMemoryDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selectObject(&self, bitmap: @wxBitmap) {
        unsafe { wxMemoryDC_SelectObject(self.handle(), bitmap.handle()) }
    }
}

struct wxMemoryFSHandlerImpl(*u8);
impl wxMemoryFSHandler for wxMemoryFSHandlerImpl {}
impl wxFileSystemHandler for wxMemoryFSHandlerImpl {}
impl wxObject for wxMemoryFSHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMemoryFSHandler : wxFileSystemHandler {
}

struct wxMemoryInputStreamImpl(*u8);
impl wxMemoryInputStream for wxMemoryInputStreamImpl {}
impl wxInputStream for wxMemoryInputStreamImpl {}
impl wxStreamBase for wxMemoryInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMemoryInputStream : wxInputStream {
}

struct wxMemoryOutputStreamImpl(*u8);
impl wxMemoryOutputStream for wxMemoryOutputStreamImpl {}
impl wxOutputStream for wxMemoryOutputStreamImpl {}
impl wxStreamBase for wxMemoryOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMemoryOutputStream : wxOutputStream {
}

struct wxMenuImpl(*u8);
impl wxMenu for wxMenuImpl {}
impl wxEvtHandler for wxMenuImpl {}
impl wxObject for wxMenuImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMenu : wxEvtHandler {
    #[fixed_stack_segment]
    fn append(&self, id: c_int, text: @wxString, help: @wxString, isCheckable: bool) {
        unsafe { wxMenu_Append(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_AppendItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSub(&self, id: c_int, text: @wxString, submenu: @wxMenu, help: @wxString) {
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
    fn new(title: @wxString, style: c_long) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenu_Create(title.handle(), style)) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn deleteById(&self, id: c_int) {
        unsafe { wxMenu_DeleteById(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn deleteByItem(&self, _itm: @wxMenuItem) {
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
    fn destroyByItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_DestroyByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { wxMenu_Enable(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: c_int) -> @wxMenuItem {
        unsafe { @wxMenuItemImpl(wxMenu_FindItem(self.handle(), id)) as @wxMenuItem }
    }
    #[fixed_stack_segment]
    fn findItemByLabel(&self, itemString: @wxString) -> c_int {
        unsafe { wxMenu_FindItemByLabel(self.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxMenu_GetClientData(self.handle())) as @wxClientData }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxMenu_GetHelpString(self.handle(), id)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getInvokingWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxMenu_GetInvokingWindow(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxMenu_GetLabel(self.handle(), id)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMenuItems(&self, _lst: @wxList) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenu_GetParent(self.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { @wxStringImpl(wxMenu_GetTitle(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: size_t, id: c_int, text: @wxString, help: @wxString, isCheckable: bool) {
        unsafe { wxMenu_Insert(self.handle(), pos, id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, pos: size_t, _itm: @wxMenuItem) {
        unsafe { wxMenu_InsertItem(self.handle(), pos, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSub(&self, pos: size_t, id: c_int, text: @wxString, submenu: @wxMenu, help: @wxString) {
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
    fn prepend(&self, id: c_int, text: @wxString, help: @wxString, isCheckable: bool) {
        unsafe { wxMenu_Prepend(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_PrependItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn prependSub(&self, id: c_int, text: @wxString, submenu: @wxMenu, help: @wxString) {
        unsafe { wxMenu_PrependSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    fn removeById(&self, id: c_int, _itm: @wxMenuItem) {
        unsafe { wxMenu_RemoveById(self.handle(), id, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn removeByItem(&self, item: *u8) {
        unsafe { wxMenu_RemoveByItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, clientData: @wxClientData) {
        unsafe { wxMenu_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setEventHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxMenu_SetEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: c_int, helpString: @wxString) {
        unsafe { wxMenu_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setInvokingWindow(&self, win: @wxWindow) {
        unsafe { wxMenu_SetInvokingWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, id: c_int, label: @wxString) {
        unsafe { wxMenu_SetLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setParent(&self, parent: @wxWindow) {
        unsafe { wxMenu_SetParent(self.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, title: @wxString) {
        unsafe { wxMenu_SetTitle(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn updateUI(&self, source: *u8) {
        unsafe { wxMenu_UpdateUI(self.handle(), source) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> @wxMenuBar {
        unsafe { @wxMenuBarImpl(wxMenu_GetMenuBar(self.handle())) as @wxMenuBar }
    }
    #[fixed_stack_segment]
    fn appendRadioItem(&self, id: c_int, text: @wxString, help: @wxString) {
        unsafe { wxMenu_AppendRadioItem(self.handle(), id, text.handle(), help.handle()) }
    }
}

struct wxMenuBarImpl(*u8);
impl wxMenuBar for wxMenuBarImpl {}
impl wxEvtHandler for wxMenuBarImpl {}
impl wxObject for wxMenuBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMenuBar : wxEvtHandler {
    #[fixed_stack_segment]
    fn append(&self, menu: @wxMenu, title: @wxString) -> c_int {
        unsafe { wxMenuBar_Append(self.handle(), menu.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn check(&self, id: c_int, check: bool) {
        unsafe { wxMenuBar_Check(self.handle(), id, check) }
    }
    #[fixed_stack_segment]
    fn new(_style: c_int) -> @wxMenuBar {
        unsafe { @wxMenuBarImpl(wxMenuBar_Create(_style)) as @wxMenuBar }
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
    fn findItem(&self, id: c_int) -> @wxMenuItem {
        unsafe { @wxMenuItemImpl(wxMenuBar_FindItem(self.handle(), id)) as @wxMenuItem }
    }
    #[fixed_stack_segment]
    fn findMenu(&self, title: @wxString) -> c_int {
        unsafe { wxMenuBar_FindMenu(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn findMenuItem(&self, menuString: @wxString, itemString: @wxString) -> c_int {
        unsafe { wxMenuBar_FindMenuItem(self.handle(), menuString.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxMenuBar_GetHelpString(self.handle(), id)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxMenuBar_GetLabel(self.handle(), id)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getLabelTop(&self, pos: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxMenuBar_GetLabelTop(self.handle(), pos)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, pos: c_int) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenuBar_GetMenu(self.handle(), pos)) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: c_int, menu: @wxMenu, title: @wxString) -> c_int {
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
    fn remove(&self, pos: c_int) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenuBar_Remove(self.handle(), pos)) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn replace(&self, pos: c_int, menu: @wxMenu, title: @wxString) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenuBar_Replace(self.handle(), pos, menu.handle(), title.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: c_int, helpString: @wxString) {
        unsafe { wxMenuBar_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, id: c_int, label: @wxString) {
        unsafe { wxMenuBar_SetItemLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, s: @wxString) {
        unsafe { wxMenuBar_SetLabel(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTop(&self, pos: c_int, label: @wxString) {
        unsafe { wxMenuBar_SetLabelTop(self.handle(), pos, label.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrameImpl(wxMenuBar_GetFrame(self.handle())) as @wxFrame }
    }
}

struct wxMenuEventImpl(*u8);
impl wxMenuEvent for wxMenuEventImpl {}
impl wxEvent for wxMenuEventImpl {}
impl wxObject for wxMenuEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMenuEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxMenuEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.handle()) }
    }
}

struct wxMenuItemImpl(*u8);
impl wxMenuItem for wxMenuItemImpl {}
impl wxObject for wxMenuItemImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMenuItem : wxObject {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxMenuItem_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxMenuItem {
        unsafe { @wxMenuItemImpl(wxMenuItem_Create()) as @wxMenuItem }
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
    fn getHelp(&self) -> @wxString {
        unsafe { @wxStringImpl(wxMenuItem_GetHelp(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { @wxStringImpl(wxMenuItem_GetLabel(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getLabelFromText(text: *wchar_t) -> @wxString {
        unsafe { @wxStringImpl(wxMenuItem_GetLabelFromText(text)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getMenu(&self) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenuItem_GetMenu(self.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn getSubMenu(&self) -> @wxMenu {
        unsafe { @wxMenuImpl(wxMenuItem_GetSubMenu(self.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxMenuItem_GetText(self.handle())) as @wxString }
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
    fn setHelp(&self, str: @wxString) {
        unsafe { wxMenuItem_SetHelp(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setSubMenu(&self, menu: @wxMenu) {
        unsafe { wxMenuItem_SetSubMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn setText(&self, str: @wxString) {
        unsafe { wxMenuItem_SetText(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn newSeparator() -> @wxMenuItem {
        unsafe { @wxMenuItemImpl(wxMenuItem_CreateSeparator()) as @wxMenuItem }
    }
    #[fixed_stack_segment]
    fn newEx(id: c_int, label: @wxString, help: @wxString, itemkind: c_int, submenu: @wxMenu) -> @wxMenuItem {
        unsafe { @wxMenuItemImpl(wxMenuItem_CreateEx(id, label.handle(), help.handle(), itemkind, submenu.handle())) as @wxMenuItem }
    }
}

struct wxMessageDialogImpl(*u8);
impl wxMessageDialog for wxMessageDialogImpl {}
impl wxDialog for wxMessageDialogImpl {}
impl wxTopLevelWindow for wxMessageDialogImpl {}
impl wxWindow for wxMessageDialogImpl {}
impl wxEvtHandler for wxMessageDialogImpl {}
impl wxObject for wxMessageDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMessageDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _msg: @wxString, _cap: @wxString, _stl: c_int) -> @wxMessageDialog {
        unsafe { @wxMessageDialogImpl(wxMessageDialog_Create(_prt.handle(), _msg.handle(), _cap.handle(), _stl)) as @wxMessageDialog }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMessageDialog_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn showModal(&self) -> c_int {
        unsafe { wxMessageDialog_ShowModal(self.handle()) }
    }
}

struct wxMetafileImpl(*u8);
impl wxMetafile for wxMetafileImpl {}
impl wxObject for wxMetafileImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMetafile : wxObject {
    #[fixed_stack_segment]
    fn new(_file: @wxString) -> @wxMetafile {
        unsafe { @wxMetafileImpl(wxMetafile_Create(_file.handle())) as @wxMetafile }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafile_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxMetafile_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn play(&self, _dc: @wxDC) -> bool {
        unsafe { wxMetafile_Play(self.handle(), _dc.handle()) }
    }
    #[fixed_stack_segment]
    fn setClipboard(&self, width: c_int, height: c_int) -> bool {
        unsafe { wxMetafile_SetClipboard(self.handle(), width, height) }
    }
}

struct wxMetafileDCImpl(*u8);
impl wxMetafileDC for wxMetafileDCImpl {}
impl wxDC for wxMetafileDCImpl {}
impl wxObject for wxMetafileDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMetafileDC : wxDC {
    #[fixed_stack_segment]
    fn close(&self) -> *u8 {
        unsafe { wxMetafileDC_Close(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_file: @wxString) -> @wxMetafileDC {
        unsafe { @wxMetafileDCImpl(wxMetafileDC_Create(_file.handle())) as @wxMetafileDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafileDC_Delete(self.handle()) }
    }
}

struct wxMimeTypesManagerImpl(*u8);
impl wxMimeTypesManager for wxMimeTypesManagerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMimeTypesManager {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addFallbacks(&self, _types: *u8) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.handle(), _types) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxMimeTypesManager {
        unsafe { @wxMimeTypesManagerImpl(wxMimeTypesManager_Create()) as @wxMimeTypesManager }
    }
    #[fixed_stack_segment]
    fn enumAllFileTypes(&self, _lst: @wxList) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromExtension(&self, _ext: @wxString) -> @wxFileType {
        unsafe { @wxFileTypeImpl(wxMimeTypesManager_GetFileTypeFromExtension(self.handle(), _ext.handle())) as @wxFileType }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromMimeType(&self, _name: @wxString) -> @wxFileType {
        unsafe { @wxFileTypeImpl(wxMimeTypesManager_GetFileTypeFromMimeType(self.handle(), _name.handle())) as @wxFileType }
    }
    #[fixed_stack_segment]
    fn isOfType(&self, _type: @wxString, _wildcard: @wxString) -> bool {
        unsafe { wxMimeTypesManager_IsOfType(self.handle(), _type.handle(), _wildcard.handle()) }
    }
}

struct wxMiniFrameImpl(*u8);
impl wxMiniFrame for wxMiniFrameImpl {}
impl wxFrame for wxMiniFrameImpl {}
impl wxTopLevelWindow for wxMiniFrameImpl {}
impl wxWindow for wxMiniFrameImpl {}
impl wxEvtHandler for wxMiniFrameImpl {}
impl wxObject for wxMiniFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMiniFrame : wxFrame {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxMiniFrame {
        unsafe { @wxMiniFrameImpl(wxMiniFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxMiniFrame }
    }
}

struct wxMirrorDCImpl(*u8);
impl wxMirrorDC for wxMirrorDCImpl {}
impl wxDC for wxMirrorDCImpl {}
impl wxObject for wxMirrorDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMirrorDC : wxDC {
    #[fixed_stack_segment]
    fn new(dc: @wxDC) -> @wxMirrorDC {
        unsafe { @wxMirrorDCImpl(wxMirrorDC_Create(dc.handle())) as @wxMirrorDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMirrorDC_Delete(self.handle()) }
    }
}

struct wxModuleImpl(*u8);
impl wxModule for wxModuleImpl {}
impl wxObject for wxModuleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxModule : wxObject {
}

struct wxMouseCaptureChangedEventImpl(*u8);
impl wxMouseCaptureChangedEvent for wxMouseCaptureChangedEventImpl {}
impl wxEvent for wxMouseCaptureChangedEventImpl {}
impl wxObject for wxMouseCaptureChangedEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMouseCaptureChangedEvent : wxEvent {
}

struct wxMouseEventImpl(*u8);
impl wxMouseEvent for wxMouseEventImpl {}
impl wxEvent for wxMouseEventImpl {}
impl wxObject for wxMouseEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMouseEvent : wxEvent {
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
    fn getLogicalPosition(&self, dc: @wxDC) -> @wxPoint {
        unsafe { @wxPointImpl(wxMouseEvent_GetLogicalPosition(self.handle(), dc.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxMouseEvent_GetPosition(self.handle())) as @wxPoint }
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

struct wxMoveEventImpl(*u8);
impl wxMoveEvent for wxMoveEventImpl {}
impl wxEvent for wxMoveEventImpl {}
impl wxObject for wxMoveEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMoveEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxMoveEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxMoveEvent_GetPosition(self.handle())) as @wxPoint }
    }
}

struct wxMultiCellCanvasImpl(*u8);
impl wxMultiCellCanvas for wxMultiCellCanvasImpl {}
impl wxFlexGridSizer for wxMultiCellCanvasImpl {}
impl wxGridSizer for wxMultiCellCanvasImpl {}
impl wxSizer for wxMultiCellCanvasImpl {}
impl wxObject for wxMultiCellCanvasImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMultiCellCanvas : wxFlexGridSizer {
    #[fixed_stack_segment]
    fn add(&self, win: @wxWindow, row: c_int, col: c_int) {
        unsafe { wxMultiCellCanvas_Add(self.handle(), win.handle(), row, col) }
    }
    #[fixed_stack_segment]
    fn calculateConstraints(&self) {
        unsafe { wxMultiCellCanvas_CalculateConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, numRows: c_int, numCols: c_int) -> @wxMultiCellCanvas {
        unsafe { @wxMultiCellCanvasImpl(wxMultiCellCanvas_Create(parent.handle(), numRows, numCols)) as @wxMultiCellCanvas }
    }
    #[fixed_stack_segment]
    fn maxCols(&self) -> c_int {
        unsafe { wxMultiCellCanvas_MaxCols(self.handle()) }
    }
    #[fixed_stack_segment]
    fn maxRows(&self) -> c_int {
        unsafe { wxMultiCellCanvas_MaxRows(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMinCellSize(&self, w: c_int, h: c_int) {
        unsafe { wxMultiCellCanvas_SetMinCellSize(self.handle(), w, h) }
    }
}

struct wxMultiCellItemHandleImpl(*u8);
impl wxMultiCellItemHandle for wxMultiCellItemHandleImpl {}
impl wxObject for wxMultiCellItemHandleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMultiCellItemHandle : wxObject {
    #[fixed_stack_segment]
    fn new(row: c_int, column: c_int, height: c_int, width: c_int, sx: c_int, sy: c_int, style: c_int, wx: c_int, wy: c_int, align: c_int) -> @wxMultiCellItemHandle {
        unsafe { @wxMultiCellItemHandleImpl(wxMultiCellItemHandle_Create(row, column, height, width, sx, sy, style, wx, wy, align)) as @wxMultiCellItemHandle }
    }
    #[fixed_stack_segment]
    fn newWithSize(&self, row: c_int, column: c_int, sx: c_int, sy: c_int, style: c_int, wx: c_int, wy: c_int, align: c_int) -> *u8 {
        unsafe { wxMultiCellItemHandle_CreateWithSize(self.handle(), row, column, sx, sy, style, wx, wy, align) }
    }
    #[fixed_stack_segment]
    fn newWithStyle(&self, row: c_int, column: c_int, style: c_int, wx: c_int, wy: c_int, align: c_int) -> *u8 {
        unsafe { wxMultiCellItemHandle_CreateWithStyle(self.handle(), row, column, style, wx, wy, align) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> c_int {
        unsafe { wxMultiCellItemHandle_GetAlignment(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self) -> c_int {
        unsafe { wxMultiCellItemHandle_GetColumn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> c_int {
        unsafe { wxMultiCellItemHandle_GetHeight(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLocalSize(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxMultiCellItemHandle_GetLocalSize(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getRow(&self) -> c_int {
        unsafe { wxMultiCellItemHandle_GetRow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxMultiCellItemHandle_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeight(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxMultiCellItemHandle_GetWeight(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxMultiCellItemHandle_GetWidth(self.handle()) }
    }
}

struct wxMultiCellSizerImpl(*u8);
impl wxMultiCellSizer for wxMultiCellSizerImpl {}
impl wxSizer for wxMultiCellSizerImpl {}
impl wxObject for wxMultiCellSizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMultiCellSizer : wxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxMultiCellSizer_CalcMin(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn new(rows: c_int, cols: c_int) -> @wxMultiCellSizer {
        unsafe { @wxMultiCellSizerImpl(wxMultiCellSizer_Create(rows, cols)) as @wxMultiCellSizer }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMultiCellSizer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enableGridLines(&self, win: @wxWindow) -> c_int {
        unsafe { wxMultiCellSizer_EnableGridLines(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxMultiCellSizer_RecalcSizes(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, column: c_int, colSize: c_int, expandable: c_int) -> c_int {
        unsafe { wxMultiCellSizer_SetColumnWidth(self.handle(), column, colSize, expandable) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellSize(&self, w: c_int, h: c_int) -> c_int {
        unsafe { wxMultiCellSizer_SetDefaultCellSize(self.handle(), w, h) }
    }
    #[fixed_stack_segment]
    fn setGridPen(&self, pen: @wxPen) -> c_int {
        unsafe { wxMultiCellSizer_SetGridPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setRowHeight(&self, row: c_int, rowSize: c_int, expandable: c_int) -> c_int {
        unsafe { wxMultiCellSizer_SetRowHeight(self.handle(), row, rowSize, expandable) }
    }
}

struct wxMutexImpl(*u8);
impl wxMutex for wxMutexImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMutex {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxMutex {
        unsafe { @wxMutexImpl(wxMutex_Create()) as @wxMutex }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMutex_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isLocked(&self) -> bool {
        unsafe { wxMutex_IsLocked(self.handle()) }
    }
    #[fixed_stack_segment]
    fn lock(&self) -> c_int {
        unsafe { wxMutex_Lock(self.handle()) }
    }
    #[fixed_stack_segment]
    fn tryLock(&self) -> c_int {
        unsafe { wxMutex_TryLock(self.handle()) }
    }
    #[fixed_stack_segment]
    fn unlock(&self) -> c_int {
        unsafe { wxMutex_Unlock(self.handle()) }
    }
}

struct wxMutexLockerImpl(*u8);
impl wxMutexLocker for wxMutexLockerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMutexLocker {
    fn handle(&self) -> *u8;
    
}

struct wxNavigationKeyEventImpl(*u8);
impl wxNavigationKeyEvent for wxNavigationKeyEventImpl {}
impl wxEvent for wxNavigationKeyEventImpl {}
impl wxObject for wxNavigationKeyEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxNavigationKeyEvent : wxEvent {
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
    fn setCurrentFocus(&self, win: @wxWindow) {
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

struct wxNewBitmapButtonImpl(*u8);
impl wxNewBitmapButton for wxNewBitmapButtonImpl {}
impl wxPanel for wxNewBitmapButtonImpl {}
impl wxWindow for wxNewBitmapButtonImpl {}
impl wxEvtHandler for wxNewBitmapButtonImpl {}
impl wxObject for wxNewBitmapButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxNewBitmapButton : wxPanel {
    #[fixed_stack_segment]
    fn new(labelBitmap: *u8, labelText: *u8, alignText: c_int, isFlat: bool, firedEventType: c_int, marginX: c_int, marginY: c_int, textToLabelGap: c_int, isSticky: bool) -> @wxNewBitmapButton {
        unsafe { @wxNewBitmapButtonImpl(wxNewBitmapButton_Create(labelBitmap, labelText, alignText, isFlat, firedEventType, marginX, marginY, textToLabelGap, isSticky)) as @wxNewBitmapButton }
    }
    #[fixed_stack_segment]
    fn newFromFile(&self, bitmapFileType: c_int, labelText: *u8, alignText: c_int, isFlat: bool, firedEventType: c_int, marginX: c_int, marginY: c_int, textToLabelGap: c_int, isSticky: bool) -> @wxNewBitmapButton {
        unsafe { @wxNewBitmapButtonImpl(wxNewBitmapButton_CreateFromFile(self.handle(), bitmapFileType, labelText, alignText, isFlat, firedEventType, marginX, marginY, textToLabelGap, isSticky)) as @wxNewBitmapButton }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxNewBitmapButton_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawDecorations(&self, dc: @wxDC) {
        unsafe { wxNewBitmapButton_DrawDecorations(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawLabel(&self, dc: @wxDC) {
        unsafe { wxNewBitmapButton_DrawLabel(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> c_int {
        unsafe { wxNewBitmapButton_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn realize(&self, _prt: @wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxNewBitmapButton_Realize(self.handle(), _prt.handle(), _id, _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn renderAllLabelImages(&self) {
        unsafe { wxNewBitmapButton_RenderAllLabelImages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn renderLabelImage(&self, destBmp: *u8, srcBmp: *u8, isEnabled: bool, isPressed: bool) {
        unsafe { wxNewBitmapButton_RenderLabelImage(self.handle(), destBmp, srcBmp, isEnabled, isPressed) }
    }
    #[fixed_stack_segment]
    fn renderLabelImages(&self) {
        unsafe { wxNewBitmapButton_RenderLabelImages(self.handle()) }
    }
    #[fixed_stack_segment]
    fn reshape(&self) {
        unsafe { wxNewBitmapButton_Reshape(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setAlignments(&self, alignText: c_int, marginX: c_int, marginY: c_int, textToLabelGap: c_int) {
        unsafe { wxNewBitmapButton_SetAlignments(self.handle(), alignText, marginX, marginY, textToLabelGap) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, labelBitmap: *u8, labelText: *u8) {
        unsafe { wxNewBitmapButton_SetLabel(self.handle(), labelBitmap, labelText) }
    }
}

struct wxNodeBaseImpl(*u8);
impl wxNodeBase for wxNodeBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxNodeBase {
    fn handle(&self) -> *u8;
    
}

struct wxNotebookImpl(*u8);
impl wxNotebook for wxNotebookImpl {}
impl wxControl for wxNotebookImpl {}
impl wxWindow for wxNotebookImpl {}
impl wxEvtHandler for wxNotebookImpl {}
impl wxObject for wxNotebookImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxNotebook : wxControl {
    #[fixed_stack_segment]
    fn addPage(&self, pPage: @wxWindow, strText: @wxString, bSelect: bool, imageId: c_int) -> bool {
        unsafe { wxNotebook_AddPage(self.handle(), pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn advanceSelection(&self, bForward: bool) {
        unsafe { wxNotebook_AdvanceSelection(self.handle(), bForward) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxNotebook {
        unsafe { @wxNotebookImpl(wxNotebook_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxNotebook }
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
    fn getImageList(&self) -> @wxImageList {
        unsafe { @wxImageListImpl(wxNotebook_GetImageList(self.handle())) as @wxImageList }
    }
    #[fixed_stack_segment]
    fn getPage(&self, nPage: c_int) -> @wxWindow {
        unsafe { @wxWindowImpl(wxNotebook_GetPage(self.handle(), nPage)) as @wxWindow }
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
    fn getPageText(&self, nPage: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxNotebook_GetPageText(self.handle(), nPage)) as @wxString }
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
    fn insertPage(&self, nPage: c_int, pPage: @wxWindow, strText: @wxString, bSelect: bool, imageId: c_int) -> bool {
        unsafe { wxNotebook_InsertPage(self.handle(), nPage, pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn removePage(&self, nPage: c_int) -> bool {
        unsafe { wxNotebook_RemovePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @wxImageList) {
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
    fn setPageText(&self, nPage: c_int, strText: @wxString) -> bool {
        unsafe { wxNotebook_SetPageText(self.handle(), nPage, strText.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @wxImageList) {
        unsafe { wxNotebook_AssignImageList(self.handle(), imageList.handle()) }
    }
}

struct wxNotebookEventImpl(*u8);
impl wxNotebookEvent for wxNotebookEventImpl {}
impl wxNotifyEvent for wxNotebookEventImpl {}
impl wxCommandEvent for wxNotebookEventImpl {}
impl wxEvent for wxNotebookEventImpl {}
impl wxObject for wxNotebookEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxNotebookEvent : wxNotifyEvent {
}

struct wxNotifyEventImpl(*u8);
impl wxNotifyEvent for wxNotifyEventImpl {}
impl wxCommandEvent for wxNotifyEventImpl {}
impl wxEvent for wxNotifyEventImpl {}
impl wxObject for wxNotifyEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxNotifyEvent : wxCommandEvent {
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

struct wxObjectImpl(*u8);
impl wxObject for wxObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { @wxClosureImpl(wxObject_GetClientClosure(self.handle())) as @wxClosure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @wxClosure) {
        unsafe { wxObject_SetClientClosure(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxObject_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClassInfo(&self) -> @wxClassInfo {
        unsafe { @wxClassInfoImpl(wxObject_GetClassInfo(self.handle())) as @wxClassInfo }
    }
    #[fixed_stack_segment]
    fn isKindOf(&self, classInfo: @wxClassInfo) -> bool {
        unsafe { wxObject_IsKindOf(self.handle(), classInfo.handle()) }
    }
    #[fixed_stack_segment]
    fn isScrolledWindow(&self) -> bool {
        unsafe { wxObject_IsScrolledWindow(self.handle()) }
    }
}

struct wxObjectRefDataImpl(*u8);
impl wxObjectRefData for wxObjectRefDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxObjectRefData {
    fn handle(&self) -> *u8;
    
}

struct wxOutputStreamImpl(*u8);
impl wxOutputStream for wxOutputStreamImpl {}
impl wxStreamBase for wxOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxOutputStream : wxStreamBase {
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

struct wxPageSetupDialogImpl(*u8);
impl wxPageSetupDialog for wxPageSetupDialogImpl {}
impl wxDialog for wxPageSetupDialogImpl {}
impl wxTopLevelWindow for wxPageSetupDialogImpl {}
impl wxWindow for wxPageSetupDialogImpl {}
impl wxEvtHandler for wxPageSetupDialogImpl {}
impl wxObject for wxPageSetupDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPageSetupDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, data: @wxPageSetupDialogData) -> @wxPageSetupDialog {
        unsafe { @wxPageSetupDialogImpl(wxPageSetupDialog_Create(parent.handle(), data.handle())) as @wxPageSetupDialog }
    }
    #[fixed_stack_segment]
    fn getPageSetupData(&self, _ref: @wxPageSetupDialogData) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.handle(), _ref.handle()) }
    }
}

struct wxPageSetupDialogDataImpl(*u8);
impl wxPageSetupDialogData for wxPageSetupDialogDataImpl {}
impl wxObject for wxPageSetupDialogDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPageSetupDialogData : wxObject {
    #[fixed_stack_segment]
    fn assign(&self, data: @wxPageSetupDialogData) {
        unsafe { wxPageSetupDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, printData: @wxPrintData) {
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
    fn new() -> @wxPageSetupDialogData {
        unsafe { @wxPageSetupDialogDataImpl(wxPageSetupDialogData_Create()) as @wxPageSetupDialogData }
    }
    #[fixed_stack_segment]
    fn newFromData(printData: @wxPrintData) -> @wxPageSetupDialogData {
        unsafe { @wxPageSetupDialogDataImpl(wxPageSetupDialogData_CreateFromData(printData.handle())) as @wxPageSetupDialogData }
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
    fn getMarginBottomRight(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxPageSetupDialogData_GetMarginBottomRight(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getMarginTopLeft(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxPageSetupDialogData_GetMarginTopLeft(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getMinMarginBottomRight(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxPageSetupDialogData_GetMinMarginBottomRight(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getMinMarginTopLeft(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxPageSetupDialogData_GetMinMarginTopLeft(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxPageSetupDialogData_GetPaperSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @wxPrintData) {
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
    fn setPrintData(&self, printData: @wxPrintData) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
}

struct wxPaintDCImpl(*u8);
impl wxPaintDC for wxPaintDCImpl {}
impl wxWindowDC for wxPaintDCImpl {}
impl wxDC for wxPaintDCImpl {}
impl wxObject for wxPaintDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPaintDC : wxWindowDC {
    #[fixed_stack_segment]
    fn new(win: @wxWindow) -> @wxPaintDC {
        unsafe { @wxPaintDCImpl(wxPaintDC_Create(win.handle())) as @wxPaintDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPaintDC_Delete(self.handle()) }
    }
}

struct wxPaintEventImpl(*u8);
impl wxPaintEvent for wxPaintEventImpl {}
impl wxEvent for wxPaintEventImpl {}
impl wxObject for wxPaintEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPaintEvent : wxEvent {
}

struct wxPaletteImpl(*u8);
impl wxPalette for wxPaletteImpl {}
impl wxGDIObject for wxPaletteImpl {}
impl wxObject for wxPaletteImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPalette : wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, palette: @wxPalette) {
        unsafe { wxPalette_Assign(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxPalette {
        unsafe { @wxPaletteImpl(wxPalette_CreateDefault()) as @wxPalette }
    }
    #[fixed_stack_segment]
    fn newRGB(n: c_int, red: *u8, green: *u8, blue: *u8) -> @wxPalette {
        unsafe { @wxPaletteImpl(wxPalette_CreateRGB(n, red, green, blue)) as @wxPalette }
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
    fn isEqual(&self, palette: @wxPalette) -> bool {
        unsafe { wxPalette_IsEqual(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPalette_IsOk(self.handle()) }
    }
}

struct wxPaletteChangedEventImpl(*u8);
impl wxPaletteChangedEvent for wxPaletteChangedEventImpl {}
impl wxEvent for wxPaletteChangedEventImpl {}
impl wxObject for wxPaletteChangedEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPaletteChangedEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxPaletteChangedEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getChangedWindow(&self) -> *u8 {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setChangedWindow(&self, win: @wxWindow) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.handle(), win.handle()) }
    }
}

struct wxPanelImpl(*u8);
impl wxPanel for wxPanelImpl {}
impl wxWindow for wxPanelImpl {}
impl wxEvtHandler for wxPanelImpl {}
impl wxObject for wxPanelImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPanel : wxWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxPanel {
        unsafe { @wxPanelImpl(wxPanel_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxPanel }
    }
    #[fixed_stack_segment]
    fn initDialog(&self) {
        unsafe { wxPanel_InitDialog(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setFocus(&self) {
        unsafe { wxPanel_SetFocus(self.handle()) }
    }
}

struct wxPathListImpl(*u8);
impl wxPathList for wxPathListImpl {}
impl wxList for wxPathListImpl {}
impl wxObject for wxPathListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPathList : wxList {
}

struct wxPenImpl(*u8);
impl wxPen for wxPenImpl {}
impl wxGDIObject for wxPenImpl {}
impl wxObject for wxPenImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPen : wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, pen: @wxPen) {
        unsafe { wxPen_Assign(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxPen {
        unsafe { @wxPenImpl(wxPen_CreateDefault()) as @wxPen }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(stipple: @wxBitmap, width: c_int) -> @wxPen {
        unsafe { @wxPenImpl(wxPen_CreateFromBitmap(stipple.handle(), width)) as @wxPen }
    }
    #[fixed_stack_segment]
    fn newFromColour(col: @wxColour, width: c_int, style: c_int) -> @wxPen {
        unsafe { @wxPenImpl(wxPen_CreateFromColour(col.handle(), width, style)) as @wxPen }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: c_int) -> @wxPen {
        unsafe { @wxPenImpl(wxPen_CreateFromStock(id)) as @wxPen }
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
    fn getColour(&self, _ref: @wxColour) {
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
    fn getStipple(&self, _ref: @wxBitmap) {
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
    fn isEqual(&self, pen: @wxPen) -> bool {
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
    fn setColour(&self, col: @wxColour) {
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
    fn setStipple(&self, stipple: @wxBitmap) {
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

struct wxPenListImpl(*u8);
impl wxPenList for wxPenListImpl {}
impl wxList for wxPenListImpl {}
impl wxObject for wxPenListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPenList : wxList {
}

struct wxPlotCurveImpl(*u8);
impl wxPlotCurve for wxPlotCurveImpl {}
impl wxObject for wxPlotCurveImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPlotCurve : wxObject {
}

struct wxPlotEventImpl(*u8);
impl wxPlotEvent for wxPlotEventImpl {}
impl wxNotifyEvent for wxPlotEventImpl {}
impl wxCommandEvent for wxPlotEventImpl {}
impl wxEvent for wxPlotEventImpl {}
impl wxObject for wxPlotEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPlotEvent : wxNotifyEvent {
    #[fixed_stack_segment]
    fn getCurve(&self) -> *u8 {
        unsafe { wxPlotEvent_GetCurve(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxPlotEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> c_double {
        unsafe { wxPlotEvent_GetZoom(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxPlotEvent_SetPosition(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, zoom: c_double) {
        unsafe { wxPlotEvent_SetZoom(self.handle(), zoom) }
    }
}

struct wxPlotOnOffCurveImpl(*u8);
impl wxPlotOnOffCurve for wxPlotOnOffCurveImpl {}
impl wxObject for wxPlotOnOffCurveImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPlotOnOffCurve : wxObject {
    #[fixed_stack_segment]
    fn add(&self, on: c_int, off: c_int, clientData: @wxClientData) {
        unsafe { wxPlotOnOffCurve_Add(self.handle(), on, off, clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn new(offsetY: c_int) -> @wxPlotOnOffCurve {
        unsafe { @wxPlotOnOffCurveImpl(wxPlotOnOffCurve_Create(offsetY)) as @wxPlotOnOffCurve }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPlotOnOffCurve_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawOffLine(&self, dc: @wxDC, y: c_int, start: c_int, end: c_int) {
        unsafe { wxPlotOnOffCurve_DrawOffLine(self.handle(), dc.handle(), y, start, end) }
    }
    #[fixed_stack_segment]
    fn drawOnLine(&self, dc: @wxDC, y: c_int, start: c_int, end: c_int, clientData: @wxClientData) {
        unsafe { wxPlotOnOffCurve_DrawOnLine(self.handle(), dc.handle(), y, start, end, clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn getAt(&self, index: c_int) -> *u8 {
        unsafe { wxPlotOnOffCurve_GetAt(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, index: c_int) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxPlotOnOffCurve_GetClientData(self.handle(), index)) as @wxClientData }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxPlotOnOffCurve_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEndX(&self) -> c_int {
        unsafe { wxPlotOnOffCurve_GetEndX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOff(&self, index: c_int) -> c_int {
        unsafe { wxPlotOnOffCurve_GetOff(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn getOffsetY(&self) -> c_int {
        unsafe { wxPlotOnOffCurve_GetOffsetY(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOn(&self, index: c_int) -> c_int {
        unsafe { wxPlotOnOffCurve_GetOn(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn getStartX(&self) -> c_int {
        unsafe { wxPlotOnOffCurve_GetStartX(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setOffsetY(&self, offsetY: c_int) {
        unsafe { wxPlotOnOffCurve_SetOffsetY(self.handle(), offsetY) }
    }
}

struct wxPlotWindowImpl(*u8);
impl wxPlotWindow for wxPlotWindowImpl {}
impl wxScrolledWindow for wxPlotWindowImpl {}
impl wxPanel for wxPlotWindowImpl {}
impl wxWindow for wxPlotWindowImpl {}
impl wxEvtHandler for wxPlotWindowImpl {}
impl wxObject for wxPlotWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPlotWindow : wxScrolledWindow {
    #[fixed_stack_segment]
    fn add(&self, curve: @wxPlotCurve) {
        unsafe { wxPlotWindow_Add(self.handle(), curve.handle()) }
    }
    #[fixed_stack_segment]
    fn addOnOff(&self, curve: @wxPlotCurve) {
        unsafe { wxPlotWindow_AddOnOff(self.handle(), curve.handle()) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, flags: c_int) -> @wxPlotWindow {
        unsafe { @wxPlotWindowImpl(wxPlotWindow_Create(parent.handle(), id, x, y, w, h, flags)) as @wxPlotWindow }
    }
    #[fixed_stack_segment]
    fn delete(&self, curve: @wxPlotCurve) {
        unsafe { wxPlotWindow_Delete(self.handle(), curve.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteOnOff(&self, curve: @wxPlotOnOffCurve) {
        unsafe { wxPlotWindow_DeleteOnOff(self.handle(), curve.handle()) }
    }
    #[fixed_stack_segment]
    fn enlarge(&self, curve: @wxPlotCurve, factor: c_double) {
        unsafe { wxPlotWindow_Enlarge(self.handle(), curve.handle(), factor) }
    }
    #[fixed_stack_segment]
    fn getAt(&self, n: c_int) -> @wxPlotCurve {
        unsafe { @wxPlotCurveImpl(wxPlotWindow_GetAt(self.handle(), n)) as @wxPlotCurve }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxPlotWindow_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCurrent(&self) -> @wxPlotCurve {
        unsafe { @wxPlotCurveImpl(wxPlotWindow_GetCurrent(self.handle())) as @wxPlotCurve }
    }
    #[fixed_stack_segment]
    fn getEnlargeAroundWindowCentre(&self) -> c_int {
        unsafe { wxPlotWindow_GetEnlargeAroundWindowCentre(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOnOffCurveAt(&self, n: c_int) -> @wxPlotOnOffCurve {
        unsafe { @wxPlotOnOffCurveImpl(wxPlotWindow_GetOnOffCurveAt(self.handle(), n)) as @wxPlotOnOffCurve }
    }
    #[fixed_stack_segment]
    fn getOnOffCurveCount(&self) -> c_int {
        unsafe { wxPlotWindow_GetOnOffCurveCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getScrollOnThumbRelease(&self) -> c_int {
        unsafe { wxPlotWindow_GetScrollOnThumbRelease(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getUnitsPerValue(&self) -> c_double {
        unsafe { wxPlotWindow_GetUnitsPerValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> c_double {
        unsafe { wxPlotWindow_GetZoom(self.handle()) }
    }
    #[fixed_stack_segment]
    fn move(&self, curve: @wxPlotCurve, pixels_up: c_int) {
        unsafe { wxPlotWindow_Move(self.handle(), curve.handle(), pixels_up) }
    }
    #[fixed_stack_segment]
    fn redrawEverything(&self) {
        unsafe { wxPlotWindow_RedrawEverything(self.handle()) }
    }
    #[fixed_stack_segment]
    fn redrawXAxis(&self) {
        unsafe { wxPlotWindow_RedrawXAxis(self.handle()) }
    }
    #[fixed_stack_segment]
    fn redrawYAxis(&self) {
        unsafe { wxPlotWindow_RedrawYAxis(self.handle()) }
    }
    #[fixed_stack_segment]
    fn resetScrollbar(&self) {
        unsafe { wxPlotWindow_ResetScrollbar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, current: @wxPlotCurve) {
        unsafe { wxPlotWindow_SetCurrent(self.handle(), current.handle()) }
    }
    #[fixed_stack_segment]
    fn setEnlargeAroundWindowCentre(&self, enlargeAroundWindowCentre: c_int) {
        unsafe { wxPlotWindow_SetEnlargeAroundWindowCentre(self.handle(), enlargeAroundWindowCentre) }
    }
    #[fixed_stack_segment]
    fn setScrollOnThumbRelease(&self, scrollOnThumbRelease: c_int) {
        unsafe { wxPlotWindow_SetScrollOnThumbRelease(self.handle(), scrollOnThumbRelease) }
    }
    #[fixed_stack_segment]
    fn setUnitsPerValue(&self, upv: c_double) {
        unsafe { wxPlotWindow_SetUnitsPerValue(self.handle(), upv) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, zoom: c_double) {
        unsafe { wxPlotWindow_SetZoom(self.handle(), zoom) }
    }
}

struct wxPointImpl(*u8);
impl wxPoint for wxPointImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPoint {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(xx: c_int, yy: c_int) -> @wxPoint {
        unsafe { @wxPointImpl(wxPoint_Create(xx, yy)) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxPoint_Destroy(self.handle()) }
    }
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

struct wxPopupTransientWindowImpl(*u8);
impl wxPopupTransientWindow for wxPopupTransientWindowImpl {}
impl wxPopupWindow for wxPopupTransientWindowImpl {}
impl wxWindow for wxPopupTransientWindowImpl {}
impl wxEvtHandler for wxPopupTransientWindowImpl {}
impl wxObject for wxPopupTransientWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPopupTransientWindow : wxPopupWindow {
}

struct wxPopupWindowImpl(*u8);
impl wxPopupWindow for wxPopupWindowImpl {}
impl wxWindow for wxPopupWindowImpl {}
impl wxEvtHandler for wxPopupWindowImpl {}
impl wxObject for wxPopupWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPopupWindow : wxWindow {
}

struct wxPostScriptDCImpl(*u8);
impl wxPostScriptDC for wxPostScriptDCImpl {}
impl wxDC for wxPostScriptDCImpl {}
impl wxObject for wxPostScriptDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPostScriptDC : wxDC {
    #[fixed_stack_segment]
    fn new(data: @wxPrintData) -> @wxPostScriptDC {
        unsafe { @wxPostScriptDCImpl(wxPostScriptDC_Create(data.handle())) as @wxPostScriptDC }
    }
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

struct wxPreviewCanvasImpl(*u8);
impl wxPreviewCanvas for wxPreviewCanvasImpl {}
impl wxScrolledWindow for wxPreviewCanvasImpl {}
impl wxPanel for wxPreviewCanvasImpl {}
impl wxWindow for wxPreviewCanvasImpl {}
impl wxEvtHandler for wxPreviewCanvasImpl {}
impl wxObject for wxPreviewCanvasImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPreviewCanvas : wxScrolledWindow {
    #[fixed_stack_segment]
    fn new(preview: @wxPrintPreview, parent: @wxWindow, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvasImpl(wxPreviewCanvas_Create(preview.handle(), parent.handle(), x, y, w, h, style)) as @wxPreviewCanvas }
    }
}

struct wxPreviewControlBarImpl(*u8);
impl wxPreviewControlBar for wxPreviewControlBarImpl {}
impl wxPanel for wxPreviewControlBarImpl {}
impl wxWindow for wxPreviewControlBarImpl {}
impl wxEvtHandler for wxPreviewControlBarImpl {}
impl wxObject for wxPreviewControlBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPreviewControlBar : wxPanel {
}

struct wxPreviewFrameImpl(*u8);
impl wxPreviewFrame for wxPreviewFrameImpl {}
impl wxFrame for wxPreviewFrameImpl {}
impl wxTopLevelWindow for wxPreviewFrameImpl {}
impl wxWindow for wxPreviewFrameImpl {}
impl wxEvtHandler for wxPreviewFrameImpl {}
impl wxObject for wxPreviewFrameImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPreviewFrame : wxFrame {
    #[fixed_stack_segment]
    fn new(preview: @wxPrintPreview, parent: @wxFrame, title: @wxString, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: @wxString) -> @wxPreviewFrame {
        unsafe { @wxPreviewFrameImpl(wxPreviewFrame_Create(preview.handle(), parent.handle(), title.handle(), x, y, width, height, style, name.handle())) as @wxPreviewFrame }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPreviewFrame_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.handle()) }
    }
}

struct wxPrintDataImpl(*u8);
impl wxPrintData for wxPrintDataImpl {}
impl wxObject for wxPrintDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrintData : wxObject {
    #[fixed_stack_segment]
    fn assign(&self, data: @wxPrintData) {
        unsafe { wxPrintData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxPrintData {
        unsafe { @wxPrintDataImpl(wxPrintData_Create()) as @wxPrintData }
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
    fn getFilename(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintData_GetFilename(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getFontMetricPath(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintData_GetFontMetricPath(self.handle())) as @wxString }
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
    fn getPaperSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxPrintData_GetPaperSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getPreviewCommand(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintData_GetPreviewCommand(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterCommand(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintData_GetPrinterCommand(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPrinterName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintData_GetPrinterName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getPrinterOptions(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintData_GetPrinterOptions(self.handle())) as @wxString }
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
    fn setFilename(&self, filename: @wxString) {
        unsafe { wxPrintData_SetFilename(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn setFontMetricPath(&self, path: @wxString) {
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
    fn setPreviewCommand(&self, command: @wxCommand) {
        unsafe { wxPrintData_SetPreviewCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.handle(), printMode) }
    }
    #[fixed_stack_segment]
    fn setPrinterCommand(&self, command: @wxCommand) {
        unsafe { wxPrintData_SetPrinterCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterName(&self, name: @wxString) {
        unsafe { wxPrintData_SetPrinterName(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterOptions(&self, options: @wxString) {
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

struct wxPostScriptPrintNativeDataImpl(*u8);
impl wxPostScriptPrintNativeData for wxPostScriptPrintNativeDataImpl {}
impl wxObject for wxPostScriptPrintNativeDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPostScriptPrintNativeData : wxObject {
    #[fixed_stack_segment]
    fn new() -> @wxPostScriptPrintNativeData {
        unsafe { @wxPostScriptPrintNativeDataImpl(wxPostScriptPrintNativeData_Create()) as @wxPostScriptPrintNativeData }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPostScriptPrintNativeData_Delete(self.handle()) }
    }
}

struct wxPrintDialogImpl(*u8);
impl wxPrintDialog for wxPrintDialogImpl {}
impl wxDialog for wxPrintDialogImpl {}
impl wxTopLevelWindow for wxPrintDialogImpl {}
impl wxWindow for wxPrintDialogImpl {}
impl wxEvtHandler for wxPrintDialogImpl {}
impl wxObject for wxPrintDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrintDialog : wxDialog {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, data: @wxPrintDialogData) -> @wxPrintDialog {
        unsafe { @wxPrintDialogImpl(wxPrintDialog_Create(parent.handle(), data.handle())) as @wxPrintDialog }
    }
    #[fixed_stack_segment]
    fn getPrintDC(&self) -> @wxDC {
        unsafe { @wxDCImpl(wxPrintDialog_GetPrintDC(self.handle())) as @wxDC }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @wxPrintData) {
        unsafe { wxPrintDialog_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self) -> @wxPrintDialogData {
        unsafe { @wxPrintDialogDataImpl(wxPrintDialog_GetPrintDialogData(self.handle())) as @wxPrintDialogData }
    }
}

struct wxPrintDialogDataImpl(*u8);
impl wxPrintDialogData for wxPrintDialogDataImpl {}
impl wxObject for wxPrintDialogDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrintDialogData : wxObject {
    #[fixed_stack_segment]
    fn assign(&self, data: @wxPrintDialogData) {
        unsafe { wxPrintDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, data: @wxPrintData) {
        unsafe { wxPrintDialogData_AssignData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxPrintDialogData {
        unsafe { @wxPrintDialogDataImpl(wxPrintDialogData_CreateDefault()) as @wxPrintDialogData }
    }
    #[fixed_stack_segment]
    fn newFromData(printData: @wxPrintData) -> @wxPrintDialogData {
        unsafe { @wxPrintDialogDataImpl(wxPrintDialogData_CreateFromData(printData.handle())) as @wxPrintDialogData }
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
    fn getPrintData(&self, _ref: @wxPrintData) {
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
    fn setPrintData(&self, printData: @wxPrintData) {
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

struct wxPrintPreviewImpl(*u8);
impl wxPrintPreview for wxPrintPreviewImpl {}
impl wxObject for wxPrintPreviewImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrintPreview : wxObject {
    #[fixed_stack_segment]
    fn newFromData(printout: @wxPrintout, printoutForPrinting: @wxPrintout, data: @wxPrintData) -> @wxPrintPreview {
        unsafe { @wxPrintPreviewImpl(wxPrintPreview_CreateFromData(printout.handle(), printoutForPrinting.handle(), data.handle())) as @wxPrintPreview }
    }
    #[fixed_stack_segment]
    fn newFromDialogData(printout: @wxPrintout, printoutForPrinting: @wxPrintout, data: @wxPrintDialogData) -> @wxPrintPreview {
        unsafe { @wxPrintPreviewImpl(wxPrintPreview_CreateFromDialogData(printout.handle(), printoutForPrinting.handle(), data.handle())) as @wxPrintPreview }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintPreview_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawBlankPage(&self, canvas: @wxPreviewCanvas, dc: @wxDC) -> bool {
        unsafe { wxPrintPreview_DrawBlankPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn getCanvas(&self) -> @wxPreviewCanvas {
        unsafe { @wxPreviewCanvasImpl(wxPrintPreview_GetCanvas(self.handle())) as @wxPreviewCanvas }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { @wxFrameImpl(wxPrintPreview_GetFrame(self.handle())) as @wxFrame }
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
    fn getPrintDialogData(&self, _ref: @wxPrintDialogData) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @wxPrintout {
        unsafe { @wxPrintoutImpl(wxPrintPreview_GetPrintout(self.handle())) as @wxPrintout }
    }
    #[fixed_stack_segment]
    fn getPrintoutForPrinting(&self) -> @wxPrintout {
        unsafe { @wxPrintoutImpl(wxPrintPreview_GetPrintoutForPrinting(self.handle())) as @wxPrintout }
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
    fn paintPage(&self, canvas: @wxPrintPreview, dc: @wxDC) -> bool {
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
    fn setCanvas(&self, canvas: @wxPreviewCanvas) {
        unsafe { wxPrintPreview_SetCanvas(self.handle(), canvas.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrentPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_SetCurrentPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    fn setFrame(&self, frame: @wxFrame) {
        unsafe { wxPrintPreview_SetFrame(self.handle(), frame.handle()) }
    }
    #[fixed_stack_segment]
    fn setOk(&self, ok: bool) {
        unsafe { wxPrintPreview_SetOk(self.handle(), ok) }
    }
    #[fixed_stack_segment]
    fn setPrintout(&self, printout: @wxPrintout) {
        unsafe { wxPrintPreview_SetPrintout(self.handle(), printout.handle()) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.handle(), percent) }
    }
}

struct wxPrinterImpl(*u8);
impl wxPrinter for wxPrinterImpl {}
impl wxObject for wxPrinterImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrinter : wxObject {
    #[fixed_stack_segment]
    fn new(data: @wxPrintDialogData) -> @wxPrinter {
        unsafe { @wxPrinterImpl(wxPrinter_Create(data.handle())) as @wxPrinter }
    }
    #[fixed_stack_segment]
    fn newAbortWindow(&self, parent: @wxWindow, printout: @wxPrintout) -> @wxWindow {
        unsafe { @wxWindowImpl(wxPrinter_CreateAbortWindow(self.handle(), parent.handle(), printout.handle())) as @wxWindow }
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
    fn getPrintDialogData(&self, _ref: @wxPrintDialogData) {
        unsafe { wxPrinter_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn print(&self, parent: @wxWindow, printout: @wxPrintout, prompt: bool) -> bool {
        unsafe { wxPrinter_Print(self.handle(), parent.handle(), printout.handle(), prompt) }
    }
    #[fixed_stack_segment]
    fn printDialog(&self, parent: @wxWindow) -> @wxDC {
        unsafe { @wxDCImpl(wxPrinter_PrintDialog(self.handle(), parent.handle())) as @wxDC }
    }
    #[fixed_stack_segment]
    fn reportError(&self, parent: @wxWindow, printout: @wxPrintout, message: @wxString) {
        unsafe { wxPrinter_ReportError(self.handle(), parent.handle(), printout.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setup(&self, parent: @wxWindow) -> bool {
        unsafe { wxPrinter_Setup(self.handle(), parent.handle()) }
    }
}

struct wxPrinterDCImpl(*u8);
impl wxPrinterDC for wxPrinterDCImpl {}
impl wxDC for wxPrinterDCImpl {}
impl wxObject for wxPrinterDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrinterDC : wxDC {
    #[fixed_stack_segment]
    fn new(data: @wxPrintData) -> @wxPrinterDC {
        unsafe { @wxPrinterDCImpl(wxPrinterDC_Create(data.handle())) as @wxPrinterDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinterDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperRect(&self) -> @wxRect {
        unsafe { @wxRectImpl(wxPrinterDC_GetPaperRect(self.handle())) as @wxRect }
    }
}

struct wxPrintoutImpl(*u8);
impl wxPrintout for wxPrintoutImpl {}
impl wxObject for wxPrintoutImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrintout : wxObject {
    #[fixed_stack_segment]
    fn getDC(&self) -> @wxDC {
        unsafe { @wxDCImpl(wxPrintout_GetDC(self.handle())) as @wxDC }
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
    fn getTitle(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPrintout_GetTitle(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn isPreview(&self) -> bool {
        unsafe { wxPrintout_IsPreview(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDC(&self, dc: @wxDC) {
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

struct wxPrivateDropTargetImpl(*u8);
impl wxPrivateDropTarget for wxPrivateDropTargetImpl {}
impl wxDropTarget for wxPrivateDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPrivateDropTarget : wxDropTarget {
}

struct wxProcessImpl(*u8);
impl wxProcess for wxProcessImpl {}
impl wxEvtHandler for wxProcessImpl {}
impl wxObject for wxProcessImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxProcess : wxEvtHandler {
    #[fixed_stack_segment]
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault(_prt: @wxWindow, _id: c_int) -> @wxProcess {
        unsafe { @wxProcessImpl(wxProcess_CreateDefault(_prt.handle(), _id)) as @wxProcess }
    }
    #[fixed_stack_segment]
    fn newRedirect(_prt: @wxWindow, _rdr: bool) -> @wxProcess {
        unsafe { @wxProcessImpl(wxProcess_CreateRedirect(_prt.handle(), _rdr)) as @wxProcess }
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
    fn getErrorStream(&self) -> @wxInputStream {
        unsafe { @wxInputStreamImpl(wxProcess_GetErrorStream(self.handle())) as @wxInputStream }
    }
    #[fixed_stack_segment]
    fn getInputStream(&self) -> @wxInputStream {
        unsafe { @wxInputStreamImpl(wxProcess_GetInputStream(self.handle())) as @wxInputStream }
    }
    #[fixed_stack_segment]
    fn getOutputStream(&self) -> @wxOutputStream {
        unsafe { @wxOutputStreamImpl(wxProcess_GetOutputStream(self.handle())) as @wxOutputStream }
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
    fn open(cmd: @wxString, flags: c_int) -> @wxProcess {
        unsafe { @wxProcessImpl(wxProcess_Open(cmd.handle(), flags)) as @wxProcess }
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

struct wxProcessEventImpl(*u8);
impl wxProcessEvent for wxProcessEventImpl {}
impl wxEvent for wxProcessEventImpl {}
impl wxObject for wxProcessEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxProcessEvent : wxEvent {
    #[fixed_stack_segment]
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPid(&self) -> c_int {
        unsafe { wxProcessEvent_GetPid(self.handle()) }
    }
}

struct wxProgressDialogImpl(*u8);
impl wxProgressDialog for wxProgressDialogImpl {}
impl wxFrame for wxProgressDialogImpl {}
impl wxTopLevelWindow for wxProgressDialogImpl {}
impl wxWindow for wxProgressDialogImpl {}
impl wxEvtHandler for wxProgressDialogImpl {}
impl wxObject for wxProgressDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxProgressDialog : wxFrame {
    #[fixed_stack_segment]
    fn new(title: @wxString, message: @wxString, max: c_int, parent: @wxWindow, style: c_int) -> @wxProgressDialog {
        unsafe { @wxProgressDialogImpl(wxProgressDialog_Create(title.handle(), message.handle(), max, parent.handle(), style)) as @wxProgressDialog }
    }
    #[fixed_stack_segment]
    fn update(&self, value: c_int) -> bool {
        unsafe { wxProgressDialog_Update(self.handle(), value) }
    }
    #[fixed_stack_segment]
    fn updateWithMessage(&self, value: c_int, message: @wxString) -> bool {
        unsafe { wxProgressDialog_UpdateWithMessage(self.handle(), value, message.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.handle()) }
    }
}

struct wxProtocolImpl(*u8);
impl wxProtocol for wxProtocolImpl {}
impl wxSocketClient for wxProtocolImpl {}
impl wxSocketBase for wxProtocolImpl {}
impl wxObject for wxProtocolImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxProtocol : wxSocketClient {
}

struct wxQuantizeImpl(*u8);
impl wxQuantize for wxQuantizeImpl {}
impl wxObject for wxQuantizeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxQuantize : wxObject {
}

struct wxQueryColImpl(*u8);
impl wxQueryCol for wxQueryColImpl {}
impl wxObject for wxQueryColImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxQueryCol : wxObject {
}

struct wxQueryFieldImpl(*u8);
impl wxQueryField for wxQueryFieldImpl {}
impl wxObject for wxQueryFieldImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxQueryField : wxObject {
}

struct wxQueryLayoutInfoEventImpl(*u8);
impl wxQueryLayoutInfoEvent for wxQueryLayoutInfoEventImpl {}
impl wxEvent for wxQueryLayoutInfoEventImpl {}
impl wxObject for wxQueryLayoutInfoEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxQueryLayoutInfoEvent : wxEvent {
    #[fixed_stack_segment]
    fn new(id: c_int) -> @wxQueryLayoutInfoEvent {
        unsafe { @wxQueryLayoutInfoEventImpl(wxQueryLayoutInfoEvent_Create(id)) as @wxQueryLayoutInfoEvent }
    }
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
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxQueryLayoutInfoEvent_GetSize(self.handle())) as @wxSize }
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

struct wxQueryNewPaletteEventImpl(*u8);
impl wxQueryNewPaletteEvent for wxQueryNewPaletteEventImpl {}
impl wxEvent for wxQueryNewPaletteEventImpl {}
impl wxObject for wxQueryNewPaletteEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxQueryNewPaletteEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
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

struct wxRadioBoxImpl(*u8);
impl wxRadioBox for wxRadioBoxImpl {}
impl wxControl for wxRadioBoxImpl {}
impl wxWindow for wxRadioBoxImpl {}
impl wxEvtHandler for wxRadioBoxImpl {}
impl wxObject for wxRadioBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRadioBox : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: c_int, _stl: c_int) -> @wxRadioBox {
        unsafe { @wxRadioBoxImpl(wxRadioBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) as @wxRadioBox }
    }
    #[fixed_stack_segment]
    fn enableItem(&self, item: c_int, enable: bool) {
        unsafe { wxRadioBox_EnableItem(self.handle(), item, enable) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> c_int {
        unsafe { wxRadioBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemLabel(&self, item: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxRadioBox_GetItemLabel(self.handle(), item)) as @wxString }
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
    fn getStringSelection(&self) -> @wxString {
        unsafe { @wxStringImpl(wxRadioBox_GetStringSelection(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBitmap(&self, item: c_int, bitmap: @wxBitmap) {
        unsafe { wxRadioBox_SetItemBitmap(self.handle(), item, bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, item: c_int, label: @wxString) {
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
    fn setStringSelection(&self, s: @wxString) {
        unsafe { wxRadioBox_SetStringSelection(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn showItem(&self, item: c_int, show: bool) {
        unsafe { wxRadioBox_ShowItem(self.handle(), item, show) }
    }
}

struct wxRadioButtonImpl(*u8);
impl wxRadioButton for wxRadioButtonImpl {}
impl wxControl for wxRadioButtonImpl {}
impl wxWindow for wxRadioButtonImpl {}
impl wxEvtHandler for wxRadioButtonImpl {}
impl wxObject for wxRadioButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRadioButton : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxRadioButton {
        unsafe { @wxRadioButtonImpl(wxRadioButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxRadioButton }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxRadioButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: bool) {
        unsafe { wxRadioButton_SetValue(self.handle(), value) }
    }
}

struct wxRealPointImpl(*u8);
impl wxRealPoint for wxRealPointImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRealPoint {
    fn handle(&self) -> *u8;
    
}

struct wxRecordSetImpl(*u8);
impl wxRecordSet for wxRecordSetImpl {}
impl wxObject for wxRecordSetImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRecordSet : wxObject {
}

struct wxRectImpl(*u8);
impl wxRect for wxRectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRect {
    fn handle(&self) -> *u8;
    
}

struct wxRegExImpl(*u8);
impl wxRegEx for wxRegExImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRegEx {
    fn handle(&self) -> *u8;
    
}

struct wxRegionImpl(*u8);
impl wxRegion for wxRegionImpl {}
impl wxGDIObject for wxRegionImpl {}
impl wxObject for wxRegionImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRegion : wxGDIObject {
    #[fixed_stack_segment]
    fn assign(&self, region: @wxRegion) {
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
    fn newDefault() -> @wxRegion {
        unsafe { @wxRegionImpl(wxRegion_CreateDefault()) as @wxRegion }
    }
    #[fixed_stack_segment]
    fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> @wxRegion {
        unsafe { @wxRegionImpl(wxRegion_CreateFromRect(x, y, w, h)) as @wxRegion }
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
    fn intersectRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_IntersectRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_SubtractRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn subtractRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_SubtractRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_UnionRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn unionRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_UnionRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_XorRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn xorRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_XorRegion(self.handle(), region.handle()) }
    }
}

struct wxRegionIteratorImpl(*u8);
impl wxRegionIterator for wxRegionIteratorImpl {}
impl wxObject for wxRegionIteratorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRegionIterator : wxObject {
    #[fixed_stack_segment]
    fn new() -> @wxRegionIterator {
        unsafe { @wxRegionIteratorImpl(wxRegionIterator_Create()) as @wxRegionIterator }
    }
    #[fixed_stack_segment]
    fn newFromRegion(region: @wxRegion) -> @wxRegionIterator {
        unsafe { @wxRegionIteratorImpl(wxRegionIterator_CreateFromRegion(region.handle())) as @wxRegionIterator }
    }
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
    fn resetToRegion(&self, region: @wxRegion) {
        unsafe { wxRegionIterator_ResetToRegion(self.handle(), region.handle()) }
    }
}

struct wxRemotelyScrolledTreeCtrlImpl(*u8);
impl wxRemotelyScrolledTreeCtrl for wxRemotelyScrolledTreeCtrlImpl {}
impl wxTreeCtrl for wxRemotelyScrolledTreeCtrlImpl {}
impl wxControl for wxRemotelyScrolledTreeCtrlImpl {}
impl wxWindow for wxRemotelyScrolledTreeCtrlImpl {}
impl wxEvtHandler for wxRemotelyScrolledTreeCtrlImpl {}
impl wxObject for wxRemotelyScrolledTreeCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxRemotelyScrolledTreeCtrl : wxTreeCtrl {
    #[fixed_stack_segment]
    fn adjustRemoteScrollbars(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars(self.handle()) }
    }
    #[fixed_stack_segment]
    fn calcTreeSize(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_CalcTreeSize(self.handle(), _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn calcTreeSizeItem(&self, id: *u8, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem(self.handle(), id, _x, _y, _w, _h) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *u8, _cmp: *u8, parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxRemotelyScrolledTreeCtrl {
        unsafe { @wxRemotelyScrolledTreeCtrlImpl(wxRemotelyScrolledTreeCtrl_Create(_obj, _cmp, parent.handle(), id, x, y, w, h, style)) as @wxRemotelyScrolledTreeCtrl }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCompanionWindow(&self) -> *u8 {
        unsafe { wxRemotelyScrolledTreeCtrl_GetCompanionWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getScrollPos(&self, orient: c_int) -> c_int {
        unsafe { wxRemotelyScrolledTreeCtrl_GetScrollPos(self.handle(), orient) }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow(&self) -> @wxScrolledWindow {
        unsafe { @wxScrolledWindowImpl(wxRemotelyScrolledTreeCtrl_GetScrolledWindow(self.handle())) as @wxScrolledWindow }
    }
    #[fixed_stack_segment]
    fn getViewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_GetViewStart(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn hideVScrollbar(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_HideVScrollbar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @wxDC) {
        unsafe { wxRemotelyScrolledTreeCtrl_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn scrollToLine(&self, posHoriz: c_int, posVert: c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_ScrollToLine(self.handle(), posHoriz, posVert) }
    }
    #[fixed_stack_segment]
    fn setCompanionWindow(&self, companion: *u8) {
        unsafe { wxRemotelyScrolledTreeCtrl_SetCompanionWindow(self.handle(), companion) }
    }
    #[fixed_stack_segment]
    fn setScrollbars(&self, pixelsPerUnitX: c_int, pixelsPerUnitY: c_int, noUnitsX: c_int, noUnitsY: c_int, xPos: c_int, yPos: c_int, noRefresh: c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_SetScrollbars(self.handle(), pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
}

struct wxSVGFileDCImpl(*u8);
impl wxSVGFileDC for wxSVGFileDCImpl {}
impl wxDC for wxSVGFileDCImpl {}
impl wxObject for wxSVGFileDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSVGFileDC : wxDC {
    #[fixed_stack_segment]
    fn new(fileName: @wxString) -> @wxSVGFileDC {
        unsafe { @wxSVGFileDCImpl(wxSVGFileDC_Create(fileName.handle())) as @wxSVGFileDC }
    }
    #[fixed_stack_segment]
    fn newWithSize(fileName: @wxString, w: c_int, h: c_int) -> @wxSVGFileDC {
        unsafe { @wxSVGFileDCImpl(wxSVGFileDC_CreateWithSize(fileName.handle(), w, h)) as @wxSVGFileDC }
    }
    #[fixed_stack_segment]
    fn newWithSizeAndResolution(fileName: @wxString, w: c_int, h: c_int, a_dpi: c_float) -> @wxSVGFileDC {
        unsafe { @wxSVGFileDCImpl(wxSVGFileDC_CreateWithSizeAndResolution(fileName.handle(), w, h, a_dpi)) as @wxSVGFileDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSVGFileDC_Delete(self.handle()) }
    }
}

struct wxSashEventImpl(*u8);
impl wxSashEvent for wxSashEventImpl {}
impl wxEvent for wxSashEventImpl {}
impl wxObject for wxSashEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSashEvent : wxEvent {
    #[fixed_stack_segment]
    fn new(id: c_int, edge: c_int) -> @wxSashEvent {
        unsafe { @wxSashEventImpl(wxSashEvent_Create(id, edge)) as @wxSashEvent }
    }
    #[fixed_stack_segment]
    fn getDragRect(&self) -> @wxRect {
        unsafe { @wxRectImpl(wxSashEvent_GetDragRect(self.handle())) as @wxRect }
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

struct wxSashLayoutWindowImpl(*u8);
impl wxSashLayoutWindow for wxSashLayoutWindowImpl {}
impl wxSashWindow for wxSashLayoutWindowImpl {}
impl wxWindow for wxSashLayoutWindowImpl {}
impl wxEvtHandler for wxSashLayoutWindowImpl {}
impl wxObject for wxSashLayoutWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSashLayoutWindow : wxSashWindow {
    #[fixed_stack_segment]
    fn new(_par: @wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxSashLayoutWindow {
        unsafe { @wxSashLayoutWindowImpl(wxSashLayoutWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) as @wxSashLayoutWindow }
    }
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

struct wxSashWindowImpl(*u8);
impl wxSashWindow for wxSashWindowImpl {}
impl wxWindow for wxSashWindowImpl {}
impl wxEvtHandler for wxSashWindowImpl {}
impl wxObject for wxSashWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSashWindow : wxWindow {
    #[fixed_stack_segment]
    fn new(_par: @wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxSashWindow {
        unsafe { @wxSashWindowImpl(wxSashWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) as @wxSashWindow }
    }
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

struct wxScopedArrayImpl(*u8);
impl wxScopedArray for wxScopedArrayImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScopedArray {
    fn handle(&self) -> *u8;
    
}

struct wxScopedPtrImpl(*u8);
impl wxScopedPtr for wxScopedPtrImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScopedPtr {
    fn handle(&self) -> *u8;
    
}

struct wxScreenDCImpl(*u8);
impl wxScreenDC for wxScreenDCImpl {}
impl wxDC for wxScreenDCImpl {}
impl wxObject for wxScreenDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScreenDC : wxDC {
    #[fixed_stack_segment]
    fn new() -> @wxScreenDC {
        unsafe { @wxScreenDCImpl(wxScreenDC_Create()) as @wxScreenDC }
    }
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
    fn startDrawingOnTopOfWin(&self, win: @wxWindow) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.handle(), win.handle()) }
    }
}

struct wxScrollBarImpl(*u8);
impl wxScrollBar for wxScrollBarImpl {}
impl wxControl for wxScrollBarImpl {}
impl wxWindow for wxScrollBarImpl {}
impl wxEvtHandler for wxScrollBarImpl {}
impl wxObject for wxScrollBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScrollBar : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxScrollBar {
        unsafe { @wxScrollBarImpl(wxScrollBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxScrollBar }
    }
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

struct wxScrollEventImpl(*u8);
impl wxScrollEvent for wxScrollEventImpl {}
impl wxEvent for wxScrollEventImpl {}
impl wxObject for wxScrollEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScrollEvent : wxEvent {
    #[fixed_stack_segment]
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.handle()) }
    }
}

struct wxScrollWinEventImpl(*u8);
impl wxScrollWinEvent for wxScrollWinEventImpl {}
impl wxEvent for wxScrollWinEventImpl {}
impl wxObject for wxScrollWinEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScrollWinEvent : wxEvent {
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

struct wxScrolledWindowImpl(*u8);
impl wxScrolledWindow for wxScrolledWindowImpl {}
impl wxPanel for wxScrolledWindowImpl {}
impl wxWindow for wxScrolledWindowImpl {}
impl wxEvtHandler for wxScrolledWindowImpl {}
impl wxObject for wxScrolledWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxScrolledWindow : wxPanel {
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
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxScrolledWindow {
        unsafe { @wxScrolledWindowImpl(wxScrolledWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxScrolledWindow }
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
    fn getTargetWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxScrolledWindow_GetTargetWindow(self.handle())) as @wxWindow }
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
    fn onDraw(&self, dc: @wxDC) {
        unsafe { wxScrolledWindow_OnDraw(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @wxDC) {
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
    fn setTargetWindow(&self, target: @wxWindow) {
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

struct wxSemaphoreImpl(*u8);
impl wxSemaphore for wxSemaphoreImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSemaphore {
    fn handle(&self) -> *u8;
    
}

struct wxServerImpl(*u8);
impl wxServer for wxServerImpl {}
impl wxServerBase for wxServerImpl {}
impl wxObject for wxServerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxServer : wxServerBase {
}

struct wxServerBaseImpl(*u8);
impl wxServerBase for wxServerBaseImpl {}
impl wxObject for wxServerBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxServerBase : wxObject {
}

struct wxSetCursorEventImpl(*u8);
impl wxSetCursorEvent for wxSetCursorEventImpl {}
impl wxEvent for wxSetCursorEventImpl {}
impl wxObject for wxSetCursorEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSetCursorEvent : wxEvent {
    #[fixed_stack_segment]
    fn getCursor(&self) -> @wxCursor {
        unsafe { @wxCursorImpl(wxSetCursorEvent_GetCursor(self.handle())) as @wxCursor }
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
    fn setCursor(&self, cursor: @wxCursor) {
        unsafe { wxSetCursorEvent_SetCursor(self.handle(), cursor.handle()) }
    }
}

struct wxShowEventImpl(*u8);
impl wxShowEvent for wxShowEventImpl {}
impl wxEvent for wxShowEventImpl {}
impl wxObject for wxShowEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxShowEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
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

struct wxSimpleHelpProviderImpl(*u8);
impl wxSimpleHelpProvider for wxSimpleHelpProviderImpl {}
impl wxHelpProvider for wxSimpleHelpProviderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSimpleHelpProvider : wxHelpProvider {
    #[fixed_stack_segment]
    fn new() -> @wxSimpleHelpProvider {
        unsafe { @wxSimpleHelpProviderImpl(wxSimpleHelpProvider_Create()) as @wxSimpleHelpProvider }
    }
}

struct wxSingleChoiceDialogImpl(*u8);
impl wxSingleChoiceDialog for wxSingleChoiceDialogImpl {}
impl wxDialog for wxSingleChoiceDialogImpl {}
impl wxTopLevelWindow for wxSingleChoiceDialogImpl {}
impl wxWindow for wxSingleChoiceDialogImpl {}
impl wxEvtHandler for wxSingleChoiceDialogImpl {}
impl wxObject for wxSingleChoiceDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSingleChoiceDialog : wxDialog {
}

struct wxSingleInstanceCheckerImpl(*u8);
impl wxSingleInstanceChecker for wxSingleInstanceCheckerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSingleInstanceChecker {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(_obj: *u8, name: @wxString, path: @wxString) -> bool {
        unsafe { wxSingleInstanceChecker_Create(_obj, name.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxSingleInstanceChecker {
        unsafe { @wxSingleInstanceCheckerImpl(wxSingleInstanceChecker_CreateDefault()) as @wxSingleInstanceChecker }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isAnotherRunning(&self) -> bool {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self.handle()) }
    }
}

struct wxSizeImpl(*u8);
impl wxSize for wxSizeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSize {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(w: c_int, h: c_int) -> @wxSize {
        unsafe { @wxSizeImpl(wxSize_Create(w, h)) as @wxSize }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxSize_Destroy(self.handle()) }
    }
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

struct wxSizeEventImpl(*u8);
impl wxSizeEvent for wxSizeEventImpl {}
impl wxEvent for wxSizeEventImpl {}
impl wxObject for wxSizeEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSizeEvent : wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxSizeEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizeEvent_GetSize(self.handle())) as @wxSize }
    }
}

struct wxSizerImpl(*u8);
impl wxSizer for wxSizerImpl {}
impl wxObject for wxSizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSizer : wxObject {
    #[fixed_stack_segment]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Add(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addSizer(&self, sizer: @wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addWindow(&self, window: @wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizer_CalcMin(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn fit(&self, window: @wxWindow) {
        unsafe { wxSizer_Fit(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizer_GetMinSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxSizer_GetPosition(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizer_GetSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Insert(self.handle(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertSizer(&self, before: c_int, sizer: @wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertSizer(self.handle(), before, sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertWindow(&self, before: c_int, window: @wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn prependSizer(&self, sizer: @wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_PrependSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependWindow(&self, window: @wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn setItemMinSizeSizer(&self, sizer: @wxSizer, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.handle(), sizer.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeWindow(&self, window: @wxWindow, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.handle(), window.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setSizeHints(&self, window: @wxWindow) {
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
    fn detachWindow(&self, window: @wxWindow) -> bool {
        unsafe { wxSizer_DetachWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn detachSizer(&self, sizer: @wxSizer) -> bool {
        unsafe { wxSizer_DetachSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn detach(&self, index: c_int) -> bool {
        unsafe { wxSizer_Detach(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn fitInside(&self, window: @wxWindow) {
        unsafe { wxSizer_FitInside(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getContainingWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxSizer_GetContainingWindow(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getItemWindow(&self, window: @wxWindow, recursive: bool) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_GetItemWindow(self.handle(), window.handle(), recursive)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn getItemSizer(&self, window: @wxSizer, recursive: bool) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_GetItemSizer(self.handle(), window.handle(), recursive)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn getItem(&self, index: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_GetItem(self.handle(), index)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn hideWindow(&self, window: @wxWindow) -> bool {
        unsafe { wxSizer_HideWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn hideSizer(&self, sizer: @wxSizer) -> bool {
        unsafe { wxSizer_HideSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn hide(&self, index: c_int) -> bool {
        unsafe { wxSizer_Hide(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn insertSpacer(&self, index: c_int, size: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_InsertSpacer(self.handle(), index, size)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_InsertStretchSpacer(self.handle(), index, prop)) as @wxSizerItem }
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
    fn prependSpacer(&self, size: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_PrependSpacer(self.handle(), size)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn prependStretchSpacer(&self, prop: c_int) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizer_PrependStretchSpacer(self.handle(), prop)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, oldwin: @wxWindow, newwin: @wxWindow, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceWindow(self.handle(), oldwin.handle(), newwin.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replaceSizer(&self, oldsz: @wxSizer, newsz: @wxSizer, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceSizer(self.handle(), oldsz.handle(), newsz.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replace(&self, oldindex: c_int, newitem: @wxSizerItem) -> bool {
        unsafe { wxSizer_Replace(self.handle(), oldindex, newitem.handle()) }
    }
    #[fixed_stack_segment]
    fn setVirtualSizeHints(&self, window: @wxWindow) {
        unsafe { wxSizer_SetVirtualSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn showWindow(&self, window: @wxWindow, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowWindow(self.handle(), window.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn showSizer(&self, sizer: @wxSizer, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowSizer(self.handle(), sizer.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn show(&self, sizer: @wxSizer, index: c_int, show: bool) -> bool {
        unsafe { wxSizer_Show(self.handle(), sizer.handle(), index, show) }
    }
}

struct wxSizerItemImpl(*u8);
impl wxSizerItem for wxSizerItemImpl {}
impl wxObject for wxSizerItemImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSizerItem : wxObject {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizerItem_CalcMin(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) -> @wxSizerItem {
        unsafe { @wxSizerItemImpl(wxSizerItem_Create(width, height, option, flag, border, userData)) as @wxSizerItem }
    }
    #[fixed_stack_segment]
    fn newInSizer(sizer: @wxSizer, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInSizer(sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn newInWindow(window: @wxWindow, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInWindow(window.handle(), option, flag, border, userData) }
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
    fn getMinSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizerItem_GetMinSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxSizerItem_GetPosition(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizerItem_GetSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @wxSizer {
        unsafe { @wxSizerImpl(wxSizerItem_GetSizer(self.handle())) as @wxSizer }
    }
    #[fixed_stack_segment]
    fn getUserData(&self) -> *u8 {
        unsafe { wxSizerItem_GetUserData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxSizerItem_GetWindow(self.handle())) as @wxWindow }
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
    fn setSizer(&self, sizer: @wxSizer) {
        unsafe { wxSizerItem_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, window: @wxWindow) {
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
    fn getRect(&self) -> @wxRect {
        unsafe { @wxRectImpl(wxSizerItem_GetRect(self.handle())) as @wxRect }
    }
    #[fixed_stack_segment]
    fn getSpacer(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxSizerItem_GetSpacer(self.handle())) as @wxSize }
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

struct wxSliderImpl(*u8);
impl wxSlider for wxSliderImpl {}
impl wxControl for wxSliderImpl {}
impl wxWindow for wxSliderImpl {}
impl wxEvtHandler for wxSliderImpl {}
impl wxObject for wxSliderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSlider : wxControl {
    #[fixed_stack_segment]
    fn clearSel(&self) {
        unsafe { wxSlider_ClearSel(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clearTicks(&self) {
        unsafe { wxSlider_ClearTicks(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSlider {
        unsafe { @wxSliderImpl(wxSlider_Create(_prt.handle(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) as @wxSlider }
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

struct wxSockAddressImpl(*u8);
impl wxSockAddress for wxSockAddressImpl {}
impl wxObject for wxSockAddressImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSockAddress : wxObject {
}

struct wxSocketBaseImpl(*u8);
impl wxSocketBase for wxSocketBaseImpl {}
impl wxObject for wxSocketBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSocketBase : wxObject {
}

struct wxSocketClientImpl(*u8);
impl wxSocketClient for wxSocketClientImpl {}
impl wxSocketBase for wxSocketClientImpl {}
impl wxObject for wxSocketClientImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSocketClient : wxSocketBase {
}

struct wxSocketEventImpl(*u8);
impl wxSocketEvent for wxSocketEventImpl {}
impl wxEvent for wxSocketEventImpl {}
impl wxObject for wxSocketEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSocketEvent : wxEvent {
}

struct wxSocketInputStreamImpl(*u8);
impl wxSocketInputStream for wxSocketInputStreamImpl {}
impl wxInputStream for wxSocketInputStreamImpl {}
impl wxStreamBase for wxSocketInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSocketInputStream : wxInputStream {
}

struct wxSocketOutputStreamImpl(*u8);
impl wxSocketOutputStream for wxSocketOutputStreamImpl {}
impl wxOutputStream for wxSocketOutputStreamImpl {}
impl wxStreamBase for wxSocketOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSocketOutputStream : wxOutputStream {
}

struct wxSocketServerImpl(*u8);
impl wxSocketServer for wxSocketServerImpl {}
impl wxSocketBase for wxSocketServerImpl {}
impl wxObject for wxSocketServerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSocketServer : wxSocketBase {
}

struct wxSpinButtonImpl(*u8);
impl wxSpinButton for wxSpinButtonImpl {}
impl wxControl for wxSpinButtonImpl {}
impl wxWindow for wxSpinButtonImpl {}
impl wxEvtHandler for wxSpinButtonImpl {}
impl wxObject for wxSpinButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSpinButton : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSpinButton {
        unsafe { @wxSpinButtonImpl(wxSpinButton_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxSpinButton }
    }
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

struct wxSpinCtrlImpl(*u8);
impl wxSpinCtrl for wxSpinCtrlImpl {}
impl wxControl for wxSpinCtrlImpl {}
impl wxWindow for wxSpinCtrlImpl {}
impl wxEvtHandler for wxSpinCtrlImpl {}
impl wxObject for wxSpinCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSpinCtrl : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> @wxSpinCtrl {
        unsafe { @wxSpinCtrlImpl(wxSpinCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) as @wxSpinCtrl }
    }
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

struct wxSpinEventImpl(*u8);
impl wxSpinEvent for wxSpinEventImpl {}
impl wxNotifyEvent for wxSpinEventImpl {}
impl wxCommandEvent for wxSpinEventImpl {}
impl wxEvent for wxSpinEventImpl {}
impl wxObject for wxSpinEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSpinEvent : wxNotifyEvent {
    #[fixed_stack_segment]
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: c_int) {
        unsafe { wxSpinEvent_SetPosition(self.handle(), pos) }
    }
}

struct wxSplashScreenImpl(*u8);
impl wxSplashScreen for wxSplashScreenImpl {}
impl wxFrame for wxSplashScreenImpl {}
impl wxTopLevelWindow for wxSplashScreenImpl {}
impl wxWindow for wxSplashScreenImpl {}
impl wxEvtHandler for wxSplashScreenImpl {}
impl wxObject for wxSplashScreenImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSplashScreen : wxFrame {
}

struct wxSplitterEventImpl(*u8);
impl wxSplitterEvent for wxSplitterEventImpl {}
impl wxNotifyEvent for wxSplitterEventImpl {}
impl wxCommandEvent for wxSplitterEventImpl {}
impl wxEvent for wxSplitterEventImpl {}
impl wxObject for wxSplitterEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSplitterEvent : wxNotifyEvent {
}

struct wxSplitterScrolledWindowImpl(*u8);
impl wxSplitterScrolledWindow for wxSplitterScrolledWindowImpl {}
impl wxScrolledWindow for wxSplitterScrolledWindowImpl {}
impl wxPanel for wxSplitterScrolledWindowImpl {}
impl wxWindow for wxSplitterScrolledWindowImpl {}
impl wxEvtHandler for wxSplitterScrolledWindowImpl {}
impl wxObject for wxSplitterScrolledWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSplitterScrolledWindow : wxScrolledWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxSplitterScrolledWindow {
        unsafe { @wxSplitterScrolledWindowImpl(wxSplitterScrolledWindow_Create(parent.handle(), id, x, y, w, h, style)) as @wxSplitterScrolledWindow }
    }
}

struct wxSplitterWindowImpl(*u8);
impl wxSplitterWindow for wxSplitterWindowImpl {}
impl wxWindow for wxSplitterWindowImpl {}
impl wxEvtHandler for wxSplitterWindowImpl {}
impl wxObject for wxSplitterWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSplitterWindow : wxWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxSplitterWindow {
        unsafe { @wxSplitterWindowImpl(wxSplitterWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxSplitterWindow }
    }
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
    fn getWindow1(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxSplitterWindow_GetWindow1(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getWindow2(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxSplitterWindow_GetWindow2(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn initialize(&self, window: @wxWindow) {
        unsafe { wxSplitterWindow_Initialize(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn isSplit(&self) -> bool {
        unsafe { wxSplitterWindow_IsSplit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, winOld: @wxWindow, winNew: @wxWindow) -> bool {
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
    fn splitHorizontally(&self, window1: @wxWindow, window2: @wxWindow, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitHorizontally(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn splitVertically(&self, window1: @wxWindow, window2: @wxWindow, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitVertically(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn unsplit(&self, toRemove: @wxWindow) -> bool {
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

struct wxStaticBitmapImpl(*u8);
impl wxStaticBitmap for wxStaticBitmapImpl {}
impl wxControl for wxStaticBitmapImpl {}
impl wxWindow for wxStaticBitmapImpl {}
impl wxEvtHandler for wxStaticBitmapImpl {}
impl wxObject for wxStaticBitmapImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStaticBitmap : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, bitmap: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticBitmap {
        unsafe { @wxStaticBitmapImpl(wxStaticBitmap_Create(_prt.handle(), _id, bitmap.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxStaticBitmap }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStaticBitmap_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmap(&self, _ref: @wxBitmap) {
        unsafe { wxStaticBitmap_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, _ref: @wxIcon) {
        unsafe { wxStaticBitmap_GetIcon(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmap(&self, bitmap: @wxBitmap) {
        unsafe { wxStaticBitmap_SetBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @wxIcon) {
        unsafe { wxStaticBitmap_SetIcon(self.handle(), icon.handle()) }
    }
}

struct wxStaticBoxImpl(*u8);
impl wxStaticBox for wxStaticBoxImpl {}
impl wxControl for wxStaticBoxImpl {}
impl wxWindow for wxStaticBoxImpl {}
impl wxEvtHandler for wxStaticBoxImpl {}
impl wxObject for wxStaticBoxImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStaticBox : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticBox {
        unsafe { @wxStaticBoxImpl(wxStaticBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxStaticBox }
    }
}

struct wxStaticBoxSizerImpl(*u8);
impl wxStaticBoxSizer for wxStaticBoxSizerImpl {}
impl wxBoxSizer for wxStaticBoxSizerImpl {}
impl wxSizer for wxStaticBoxSizerImpl {}
impl wxObject for wxStaticBoxSizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStaticBoxSizer : wxBoxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxStaticBoxSizer_CalcMin(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn new(box: @wxStaticBox, orient: c_int) -> @wxStaticBoxSizer {
        unsafe { @wxStaticBoxSizerImpl(wxStaticBoxSizer_Create(box.handle(), orient)) as @wxStaticBoxSizer }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self) -> @wxStaticBox {
        unsafe { @wxStaticBoxImpl(wxStaticBoxSizer_GetStaticBox(self.handle())) as @wxStaticBox }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxStaticBoxSizer_RecalcSizes(self.handle()) }
    }
}

struct wxStaticLineImpl(*u8);
impl wxStaticLine for wxStaticLineImpl {}
impl wxControl for wxStaticLineImpl {}
impl wxWindow for wxStaticLineImpl {}
impl wxEvtHandler for wxStaticLineImpl {}
impl wxObject for wxStaticLineImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStaticLine : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticLine {
        unsafe { @wxStaticLineImpl(wxStaticLine_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxStaticLine }
    }
    #[fixed_stack_segment]
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isVertical(&self) -> bool {
        unsafe { wxStaticLine_IsVertical(self.handle()) }
    }
}

struct wxStaticTextImpl(*u8);
impl wxStaticText for wxStaticTextImpl {}
impl wxControl for wxStaticTextImpl {}
impl wxWindow for wxStaticTextImpl {}
impl wxEvtHandler for wxStaticTextImpl {}
impl wxObject for wxStaticTextImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStaticText : wxControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStaticText {
        unsafe { @wxStaticTextImpl(wxStaticText_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxStaticText }
    }
}

struct wxStatusBarImpl(*u8);
impl wxStatusBar for wxStatusBarImpl {}
impl wxWindow for wxStatusBarImpl {}
impl wxEvtHandler for wxStatusBarImpl {}
impl wxObject for wxStatusBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStatusBar : wxWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxStatusBar {
        unsafe { @wxStatusBarImpl(wxStatusBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxStatusBar }
    }
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
    fn getStatusText(&self, number: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxStatusBar_GetStatusText(self.handle(), number)) as @wxString }
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
    fn setStatusText(&self, text: @wxString, number: c_int) {
        unsafe { wxStatusBar_SetStatusText(self.handle(), text.handle(), number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, n: c_int, widths: *c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.handle(), n, widths) }
    }
}

struct wxStopWatchImpl(*u8);
impl wxStopWatch for wxStopWatchImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStopWatch {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxStopWatch {
        unsafe { @wxStopWatchImpl(wxStopWatch_Create()) as @wxStopWatch }
    }
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

struct wxStreamBaseImpl(*u8);
impl wxStreamBase for wxStreamBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStreamBase {
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

struct wxStreamBufferImpl(*u8);
impl wxStreamBuffer for wxStreamBufferImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStreamBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStreamToTextRedirectorImpl(*u8);
impl wxStreamToTextRedirector for wxStreamToTextRedirectorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStreamToTextRedirector {
    fn handle(&self) -> *u8;
    
}

struct wxStringImpl(*u8);
impl wxString for wxStringImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxString {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(buffer: *wchar_t) -> @wxString {
        unsafe { @wxStringImpl(wxString_Create(buffer)) as @wxString }
    }
    #[fixed_stack_segment]
    fn newLen(buffer: *wchar_t, len: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxString_CreateLen(buffer, len)) as @wxString }
    }
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

struct wxStringBufferImpl(*u8);
impl wxStringBuffer for wxStringBufferImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStringBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStringClientDataImpl(*u8);
impl wxStringClientData for wxStringClientDataImpl {}
impl wxClientData for wxStringClientDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStringClientData : wxClientData {
}

struct wxStringListImpl(*u8);
impl wxStringList for wxStringListImpl {}
impl wxList for wxStringListImpl {}
impl wxObject for wxStringListImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStringList : wxList {
}

struct wxStringTokenizerImpl(*u8);
impl wxStringTokenizer for wxStringTokenizerImpl {}
impl wxObject for wxStringTokenizerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStringTokenizer : wxObject {
}

struct wxSysColourChangedEventImpl(*u8);
impl wxSysColourChangedEvent for wxSysColourChangedEventImpl {}
impl wxEvent for wxSysColourChangedEventImpl {}
impl wxObject for wxSysColourChangedEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSysColourChangedEvent : wxEvent {
}

struct wxSystemOptionsImpl(*u8);
impl wxSystemOptions for wxSystemOptionsImpl {}
impl wxObject for wxSystemOptionsImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSystemOptions : wxObject {
}

struct wxSystemSettingsImpl(*u8);
impl wxSystemSettings for wxSystemSettingsImpl {}
impl wxObject for wxSystemSettingsImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSystemSettings : wxObject {
    #[fixed_stack_segment]
    fn getColour(index: c_int, _ref: @wxColour) {
        unsafe { wxSystemSettings_GetColour(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(index: c_int, _ref: @wxFont) {
        unsafe { wxSystemSettings_GetFont(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getMetric(index: c_int) -> c_int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    #[fixed_stack_segment]
    fn getScreenType() -> c_int {
        unsafe { wxSystemSettings_GetScreenType() }
    }
}

struct wxTabCtrlImpl(*u8);
impl wxTabCtrl for wxTabCtrlImpl {}
impl wxControl for wxTabCtrlImpl {}
impl wxWindow for wxTabCtrlImpl {}
impl wxEvtHandler for wxTabCtrlImpl {}
impl wxObject for wxTabCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTabCtrl : wxControl {
}

struct wxTabEventImpl(*u8);
impl wxTabEvent for wxTabEventImpl {}
impl wxCommandEvent for wxTabEventImpl {}
impl wxEvent for wxTabEventImpl {}
impl wxObject for wxTabEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTabEvent : wxCommandEvent {
}

struct wxTablesInUseImpl(*u8);
impl wxTablesInUse for wxTablesInUseImpl {}
impl wxObject for wxTablesInUseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTablesInUse : wxObject {
}

struct wxTaskBarIconImpl(*u8);
impl wxTaskBarIcon for wxTaskBarIconImpl {}
impl wxEvtHandler for wxTaskBarIconImpl {}
impl wxObject for wxTaskBarIconImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTaskBarIcon : wxEvtHandler {
    #[fixed_stack_segment]
    fn new() -> @wxTaskBarIcon {
        unsafe { @wxTaskBarIconImpl(wxTaskBarIcon_Create()) as @wxTaskBarIcon }
    }
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
    fn popupMenu(&self, menu: @wxMenu) -> bool {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn removeIcon(&self) -> bool {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @wxIcon, text: @wxString) -> bool {
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

struct wxTempFileImpl(*u8);
impl wxTempFile for wxTempFileImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTempFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextAttrImpl(*u8);
impl wxTextAttr for wxTextAttrImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(colText: @wxColour, colBack: @wxColour, font: @wxFont) -> @wxTextAttr {
        unsafe { @wxTextAttrImpl(wxTextAttr_Create(colText.handle(), colBack.handle(), font.handle())) as @wxTextAttr }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxTextAttr {
        unsafe { @wxTextAttrImpl(wxTextAttr_CreateDefault()) as @wxTextAttr }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxTextAttr_GetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, font: @wxFont) {
        unsafe { wxTextAttr_GetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, colour: @wxColour) {
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
    fn setTextColour(&self, colour: @wxColour) {
        unsafe { wxTextAttr_SetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxTextAttr_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxTextAttr_SetFont(self.handle(), font.handle()) }
    }
}

struct wxTextCtrlImpl(*u8);
impl wxTextCtrl for wxTextCtrlImpl {}
impl wxControl for wxTextCtrlImpl {}
impl wxWindow for wxTextCtrlImpl {}
impl wxEvtHandler for wxTextCtrlImpl {}
impl wxObject for wxTextCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextCtrl : wxControl {
    #[fixed_stack_segment]
    fn appendText(&self, text: @wxString) {
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
    fn changeValue(&self, text: @wxString) {
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
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxTextCtrl {
        unsafe { @wxTextCtrlImpl(wxTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @wxTextCtrl }
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
    fn getLineText(&self, lineNo: c_long) -> @wxString {
        unsafe { @wxStringImpl(wxTextCtrl_GetLineText(self.handle(), lineNo)) as @wxString }
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
    fn getValue(&self) -> @wxString {
        unsafe { @wxStringImpl(wxTextCtrl_GetValue(self.handle())) as @wxString }
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
    fn loadFile(&self, file: @wxString) -> bool {
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
    fn replace(&self, from: c_long, to: c_long, value: @wxString) {
        unsafe { wxTextCtrl_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, file: @wxString) -> bool {
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
    fn setValue(&self, value: @wxString) {
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
    fn writeText(&self, text: @wxString) {
        unsafe { wxTextCtrl_WriteText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn emulateKeyPress(&self, keyevent: @wxKeyEvent) -> bool {
        unsafe { wxTextCtrl_EmulateKeyPress(self.handle(), keyevent.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultStyle(&self) -> @wxTextAttr {
        unsafe { @wxTextAttrImpl(wxTextCtrl_GetDefaultStyle(self.handle())) as @wxTextAttr }
    }
    #[fixed_stack_segment]
    fn getRange(&self, from: c_long, to: c_long) -> @wxString {
        unsafe { @wxStringImpl(wxTextCtrl_GetRange(self.handle(), from, to)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @wxString {
        unsafe { @wxStringImpl(wxTextCtrl_GetStringSelection(self.handle())) as @wxString }
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
    fn setDefaultStyle(&self, style: @wxTextAttr) -> bool {
        unsafe { wxTextCtrl_SetDefaultStyle(self.handle(), style.handle()) }
    }
    #[fixed_stack_segment]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, start: c_long, end: c_long, style: @wxTextAttr) -> bool {
        unsafe { wxTextCtrl_SetStyle(self.handle(), start, end, style.handle()) }
    }
}

struct wxTextDataObjectImpl(*u8);
impl wxTextDataObject for wxTextDataObjectImpl {}
impl wxDataObjectSimple for wxTextDataObjectImpl {}
impl wxDataObject for wxTextDataObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextDataObject : wxDataObjectSimple {
}

struct wxTextDropTargetImpl(*u8);
impl wxTextDropTarget for wxTextDropTargetImpl {}
impl wxDropTarget for wxTextDropTargetImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextDropTarget : wxDropTarget {
}

struct wxTextEntryDialogImpl(*u8);
impl wxTextEntryDialog for wxTextEntryDialogImpl {}
impl wxDialog for wxTextEntryDialogImpl {}
impl wxTopLevelWindow for wxTextEntryDialogImpl {}
impl wxWindow for wxTextEntryDialogImpl {}
impl wxEvtHandler for wxTextEntryDialogImpl {}
impl wxObject for wxTextEntryDialogImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextEntryDialog : wxDialog {
}

struct wxTextFileImpl(*u8);
impl wxTextFile for wxTextFileImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextInputStreamImpl(*u8);
impl wxTextInputStream for wxTextInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextInputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(inputStream: @wxInputStream, sep: @wxString) -> @wxTextInputStream {
        unsafe { @wxTextInputStreamImpl(wxTextInputStream_Create(inputStream.handle(), sep.handle())) as @wxTextInputStream }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readLine(&self) -> @wxString {
        unsafe { @wxStringImpl(wxTextInputStream_ReadLine(self.handle())) as @wxString }
    }
}

struct wxTextOutputStreamImpl(*u8);
impl wxTextOutputStream for wxTextOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextOutputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new(outputStream: @wxOutputStream, mode: c_int) -> @wxTextOutputStream {
        unsafe { @wxTextOutputStreamImpl(wxTextOutputStream_Create(outputStream.handle(), mode)) as @wxTextOutputStream }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, txt: @wxString) {
        unsafe { wxTextOutputStream_WriteString(self.handle(), txt.handle()) }
    }
}

struct wxTextValidatorImpl(*u8);
impl wxTextValidator for wxTextValidatorImpl {}
impl wxValidator for wxTextValidatorImpl {}
impl wxEvtHandler for wxTextValidatorImpl {}
impl wxObject for wxTextValidatorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTextValidator : wxValidator {
    #[fixed_stack_segment]
    fn new(style: c_int, val: *u8) -> @wxTextValidator {
        unsafe { @wxTextValidatorImpl(wxTextValidator_Create(style, val)) as @wxTextValidator }
    }
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
    fn clone(&self) -> @wxValidator {
        unsafe { @wxValidatorImpl(wxTextValidator_Clone(self.handle())) as @wxValidator }
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
    fn onChar(&self, event: @wxEvent) {
        unsafe { wxTextValidator_OnChar(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.handle(), style) }
    }
}

struct wxThinSplitterWindowImpl(*u8);
impl wxThinSplitterWindow for wxThinSplitterWindowImpl {}
impl wxSplitterWindow for wxThinSplitterWindowImpl {}
impl wxWindow for wxThinSplitterWindowImpl {}
impl wxEvtHandler for wxThinSplitterWindowImpl {}
impl wxObject for wxThinSplitterWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxThinSplitterWindow : wxSplitterWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxThinSplitterWindow {
        unsafe { @wxThinSplitterWindowImpl(wxThinSplitterWindow_Create(parent.handle(), id, x, y, w, h, style)) as @wxThinSplitterWindow }
    }
    #[fixed_stack_segment]
    fn drawSash(&self, dc: @wxDC) {
        unsafe { wxThinSplitterWindow_DrawSash(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn sashHitTest(&self, x: c_int, y: c_int, tolerance: c_int) -> c_int {
        unsafe { wxThinSplitterWindow_SashHitTest(self.handle(), x, y, tolerance) }
    }
    #[fixed_stack_segment]
    fn sizeWindows(&self) {
        unsafe { wxThinSplitterWindow_SizeWindows(self.handle()) }
    }
}

struct wxThreadImpl(*u8);
impl wxThread for wxThreadImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxThread {
    fn handle(&self) -> *u8;
    
}

struct wxTimeImpl(*u8);
impl wxTime for wxTimeImpl {}
impl wxObject for wxTimeImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTime : wxObject {
}

struct wxTimeSpanImpl(*u8);
impl wxTimeSpan for wxTimeSpanImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTimeSpan {
    fn handle(&self) -> *u8;
    
}

struct wxTimerImpl(*u8);
impl wxTimer for wxTimerImpl {}
impl wxObject for wxTimerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTimer : wxObject {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int) -> @wxTimer {
        unsafe { @wxTimerImpl(wxTimer_Create(_prt.handle(), _id)) as @wxTimer }
    }
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

struct wxTimerBaseImpl(*u8);
impl wxTimerBase for wxTimerBaseImpl {}
impl wxObject for wxTimerBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTimerBase : wxObject {
}

struct wxTimerEventImpl(*u8);
impl wxTimerEvent for wxTimerEventImpl {}
impl wxEvent for wxTimerEventImpl {}
impl wxObject for wxTimerEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTimerEvent : wxEvent {
    #[fixed_stack_segment]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.handle()) }
    }
}

struct wxTimerExImpl(*u8);
impl wxTimerEx for wxTimerExImpl {}
impl wxTimer for wxTimerExImpl {}
impl wxObject for wxTimerExImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTimerEx : wxTimer {
    #[fixed_stack_segment]
    fn connect(&self, closure: @wxClosure) {
        unsafe { wxTimerEx_Connect(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxTimerEx {
        unsafe { @wxTimerExImpl(wxTimerEx_Create()) as @wxTimerEx }
    }
    #[fixed_stack_segment]
    fn getClosure(&self) -> @wxClosure {
        unsafe { @wxClosureImpl(wxTimerEx_GetClosure(self.handle())) as @wxClosure }
    }
}

struct wxTimerRunnerImpl(*u8);
impl wxTimerRunner for wxTimerRunnerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTimerRunner {
    fn handle(&self) -> *u8;
    
}

struct wxTipProviderImpl(*u8);
impl wxTipProvider for wxTipProviderImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTipProvider {
    fn handle(&self) -> *u8;
    
}

struct wxTipWindowImpl(*u8);
impl wxTipWindow for wxTipWindowImpl {}
impl wxPopupTransientWindow for wxTipWindowImpl {}
impl wxPopupWindow for wxTipWindowImpl {}
impl wxWindow for wxTipWindowImpl {}
impl wxEvtHandler for wxTipWindowImpl {}
impl wxObject for wxTipWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTipWindow : wxPopupTransientWindow {
    #[fixed_stack_segment]
    fn close(&self) {
        unsafe { wxTipWindow_Close(self.handle()) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, text: @wxString, maxLength: c_int) -> @wxTipWindow {
        unsafe { @wxTipWindowImpl(wxTipWindow_Create(parent.handle(), text.handle(), maxLength)) as @wxTipWindow }
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

struct wxToggleButtonImpl(*u8);
impl wxToggleButton for wxToggleButtonImpl {}
impl wxControl for wxToggleButtonImpl {}
impl wxWindow for wxToggleButtonImpl {}
impl wxEvtHandler for wxToggleButtonImpl {}
impl wxObject for wxToggleButtonImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxToggleButton : wxControl {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, label: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxToggleButton {
        unsafe { @wxToggleButtonImpl(wxToggleButton_Create(parent.handle(), id, label.handle(), x, y, w, h, style)) as @wxToggleButton }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> bool {
        unsafe { wxToggleButton_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxToggleButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, label: @wxString) {
        unsafe { wxToggleButton_SetLabel(self.handle(), label.handle()) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, state: bool) {
        unsafe { wxToggleButton_SetValue(self.handle(), state) }
    }
}

struct wxToolBarImpl(*u8);
impl wxToolBar for wxToolBarImpl {}
impl wxToolBarBase for wxToolBarImpl {}
impl wxControl for wxToolBarImpl {}
impl wxWindow for wxToolBarImpl {}
impl wxEvtHandler for wxToolBarImpl {}
impl wxObject for wxToolBarImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxToolBar : wxToolBarBase {
    #[fixed_stack_segment]
    fn addControl(&self, ctrl: @wxControl) -> bool {
        unsafe { wxToolBar_AddControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn addTool(&self, id: c_int, bmp: @wxBitmap, shelp: @wxString, lhelp: @wxString) {
        unsafe { wxToolBar_AddTool(self.handle(), id, bmp.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn addToolEx(&self, id: c_int, bmp1: @wxBitmap, bmp2: @wxBitmap, isToggle: bool, x: c_int, y: c_int, data: @wxObject, shelp: @wxString, lhelp: @wxString) {
        unsafe { wxToolBar_AddToolEx(self.handle(), id, bmp1.handle(), bmp2.handle(), isToggle, x, y, data.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxToolBar {
        unsafe { @wxToolBarImpl(wxToolBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxToolBar }
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
    fn getMargins(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxToolBar_GetMargins(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getToolBitmapSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxToolBar_GetToolBitmapSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getToolClientData(&self, id: c_int) -> @wxObject {
        unsafe { @wxObjectImpl(wxToolBar_GetToolClientData(self.handle(), id)) as @wxObject }
    }
    #[fixed_stack_segment]
    fn getToolEnabled(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn getToolLongHelp(&self, id: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxToolBar_GetToolLongHelp(self.handle(), id)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolShortHelp(&self, id: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxToolBar_GetToolShortHelp(self.handle(), id)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getToolSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxToolBar_GetToolSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getToolState(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolState(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn insertControl(&self, pos: c_int, ctrl: @wxControl) {
        unsafe { wxToolBar_InsertControl(self.handle(), pos, ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn insertTool(&self, pos: c_int, id: c_int, bmp1: @wxBitmap, bmp2: @wxBitmap, isToggle: bool, data: @wxObject, shelp: @wxString, lhelp: @wxString) {
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
    fn setToolClientData(&self, id: c_int, data: @wxObject) {
        unsafe { wxToolBar_SetToolClientData(self.handle(), id, data.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolLongHelp(&self, id: c_int, str: @wxString) {
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
    fn setToolShortHelp(&self, id: c_int, str: @wxString) {
        unsafe { wxToolBar_SetToolShortHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    fn toggleTool(&self, id: c_int, toggle: bool) {
        unsafe { wxToolBar_ToggleTool(self.handle(), id, toggle) }
    }
    #[fixed_stack_segment]
    fn addTool2(&self, toolId: c_int, label: @wxString, bmp: @wxBitmap, bmpDisabled: @wxBitmap, itemKind: c_int, shortHelp: @wxString, longHelp: @wxString) {
        unsafe { wxToolBar_AddTool2(self.handle(), toolId, label.handle(), bmp.handle(), bmpDisabled.handle(), itemKind, shortHelp.handle(), longHelp.handle()) }
    }
}

struct wxToolBarBaseImpl(*u8);
impl wxToolBarBase for wxToolBarBaseImpl {}
impl wxControl for wxToolBarBaseImpl {}
impl wxWindow for wxToolBarBaseImpl {}
impl wxEvtHandler for wxToolBarBaseImpl {}
impl wxObject for wxToolBarBaseImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxToolBarBase : wxControl {
}

struct wxToolLayoutItemImpl(*u8);
impl wxToolLayoutItem for wxToolLayoutItemImpl {}
impl wxObject for wxToolLayoutItemImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxToolLayoutItem : wxObject {
    #[fixed_stack_segment]
    fn isSeparator(&self) -> bool {
        unsafe { wxToolLayoutItem_IsSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxToolLayoutItem_Rect(self.handle(), _x, _y, _w, _h) }
    }
}

struct wxToolTipImpl(*u8);
impl wxToolTip for wxToolTipImpl {}
impl wxObject for wxToolTipImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxToolTip : wxObject {
}

struct wxToolWindowImpl(*u8);
impl wxToolWindow for wxToolWindowImpl {}
impl wxFrame for wxToolWindowImpl {}
impl wxTopLevelWindow for wxToolWindowImpl {}
impl wxWindow for wxToolWindowImpl {}
impl wxEvtHandler for wxToolWindowImpl {}
impl wxObject for wxToolWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxToolWindow : wxFrame {
    #[fixed_stack_segment]
    fn addMiniButton(&self, _btn: *u8) {
        unsafe { wxToolWindow_AddMiniButton(self.handle(), _btn) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *u8, _btn: *u8, _ttl: *u8) -> @wxToolWindow {
        unsafe { @wxToolWindowImpl(wxToolWindow_Create(_obj, _btn, _ttl)) as @wxToolWindow }
    }
    #[fixed_stack_segment]
    fn getClient(&self) -> @wxClient {
        unsafe { @wxClientImpl(wxToolWindow_GetClient(self.handle())) as @wxClient }
    }
    #[fixed_stack_segment]
    fn setClient(&self, _wnd: @wxWindow) {
        unsafe { wxToolWindow_SetClient(self.handle(), _wnd.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitleFont(&self, _fnt: *u8) {
        unsafe { wxToolWindow_SetTitleFont(self.handle(), _fnt) }
    }
}

struct wxTopLevelWindowImpl(*u8);
impl wxTopLevelWindow for wxTopLevelWindowImpl {}
impl wxWindow for wxTopLevelWindowImpl {}
impl wxEvtHandler for wxTopLevelWindowImpl {}
impl wxObject for wxTopLevelWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTopLevelWindow : wxWindow {
    #[fixed_stack_segment]
    fn enableCloseButton(&self, enable: bool) -> bool {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getDefaultButton(&self) -> @wxButton {
        unsafe { @wxButtonImpl(wxTopLevelWindow_GetDefaultButton(self.handle())) as @wxButton }
    }
    #[fixed_stack_segment]
    fn getDefaultItem(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxTopLevelWindow_GetDefaultItem(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getIcon(&self) -> @wxIcon {
        unsafe { @wxIconImpl(wxTopLevelWindow_GetIcon(self.handle())) as @wxIcon }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { @wxStringImpl(wxTopLevelWindow_GetTitle(self.handle())) as @wxString }
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
    fn setDefaultButton(&self, pBut: @wxButton) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultItem(&self, pBut: @wxWindow) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, pIcon: @wxIcon) {
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
    fn setTitle(&self, pString: @wxString) {
        unsafe { wxTopLevelWindow_SetTitle(self.handle(), pString.handle()) }
    }
}

struct wxTreeCompanionWindowImpl(*u8);
impl wxTreeCompanionWindow for wxTreeCompanionWindowImpl {}
impl wxWindow for wxTreeCompanionWindowImpl {}
impl wxEvtHandler for wxTreeCompanionWindowImpl {}
impl wxObject for wxTreeCompanionWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeCompanionWindow : wxWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @wxTreeCompanionWindow {
        unsafe { @wxTreeCompanionWindowImpl(wxTreeCompanionWindow_Create(parent.handle(), id, x, y, w, h, style)) as @wxTreeCompanionWindow }
    }
    #[fixed_stack_segment]
    fn drawItem(&self, dc: @wxDC, id: *u8, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTreeCompanionWindow_DrawItem(self.handle(), dc.handle(), id, x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl(&self) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrlImpl(wxTreeCompanionWindow_GetTreeCtrl(self.handle())) as @wxTreeCtrl }
    }
    #[fixed_stack_segment]
    fn setTreeCtrl(&self, treeCtrl: @wxTreeCtrl) {
        unsafe { wxTreeCompanionWindow_SetTreeCtrl(self.handle(), treeCtrl.handle()) }
    }
}

struct wxTreeCtrlImpl(*u8);
impl wxTreeCtrl for wxTreeCtrlImpl {}
impl wxControl for wxTreeCtrlImpl {}
impl wxWindow for wxTreeCtrlImpl {}
impl wxEvtHandler for wxTreeCtrlImpl {}
impl wxObject for wxTreeCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeCtrl : wxControl {
    #[fixed_stack_segment]
    fn addRoot(&self, text: @wxString, image: c_int, selectedImage: c_int, data: @wxTreeItemData, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_AddRoot(self.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, parent: @wxTreeItemId, text: @wxString, image: c_int, selectedImage: c_int, data: @wxTreeItemData, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_AppendItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapse(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Collapse(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapseAndReset(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *u8, _cmp: *u8, _prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrlImpl(wxTreeCtrl_Create(_obj, _cmp, _prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxTreeCtrl }
    }
    #[fixed_stack_segment]
    fn delete(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Delete(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteChildren(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_DeleteChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn editLabel(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_EditLabel(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn endEditLabel(&self, item: @wxTreeItemId, discardChanges: bool) {
        unsafe { wxTreeCtrl_EndEditLabel(self.handle(), item.handle(), discardChanges) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_EnsureVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn expand(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Expand(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getBoundingRect(&self, item: @wxTreeItemId, textOnly: bool) -> @wxRect {
        unsafe { @wxRectImpl(wxTreeCtrl_GetBoundingRect(self.handle(), item.handle(), textOnly)) as @wxRect }
    }
    #[fixed_stack_segment]
    fn getChildrenCount(&self, item: @wxTreeItemId, recursively: bool) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.handle(), item.handle(), recursively) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> @wxTextCtrl {
        unsafe { @wxTextCtrlImpl(wxTreeCtrl_GetEditControl(self.handle())) as @wxTextCtrl }
    }
    #[fixed_stack_segment]
    fn getFirstChild(&self, item: @wxTreeItemId, cookie: *c_int, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetFirstChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleItem(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> @wxImageList {
        unsafe { @wxImageListImpl(wxTreeCtrl_GetImageList(self.handle())) as @wxImageList }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemData(&self, item: @wxTreeItemId) -> *u8 {
        unsafe { wxTreeCtrl_GetItemData(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemImage(&self, item: @wxTreeItemId, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.handle(), item.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: @wxTreeItemId) -> @wxString {
        unsafe { @wxStringImpl(wxTreeCtrl_GetItemText(self.handle(), item.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getLastChild(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetLastChild(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextChild(&self, item: @wxTreeItemId, cookie: *c_int, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextSibling(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextVisible(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetParent(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevSibling(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevVisible(&self, item: @wxTreeItemId, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getRootItem(&self, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetRootItem(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, _item: @wxTreeItemId) {
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
    fn getStateImageList(&self) -> @wxImageList {
        unsafe { @wxImageListImpl(wxTreeCtrl_GetStateImageList(self.handle())) as @wxImageList }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, _x: c_int, _y: c_int, flags: *c_int, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_HitTest(self.handle(), _x, _y, flags, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, parent: @wxTreeItemId, idPrevious: @wxTreeItemId, text: @wxString, image: c_int, selectedImage: c_int, data: *u8, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItem(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex(&self, parent: @wxTreeItemId, index: c_int, text: @wxString, image: c_int, selectedImage: c_int, data: *u8, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn isBold(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsBold(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isExpanded(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsExpanded(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isSelected(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsSelected(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn itemHasChildren(&self, item: @wxTreeItemId) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn onCompareItems(&self, item1: @wxTreeItemId, item2: @wxTreeItemId) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.handle(), item1.handle(), item2.handle()) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, parent: @wxTreeItemId, text: @wxString, image: c_int, selectedImage: c_int, data: *u8, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_PrependItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn scrollTo(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_ScrollTo(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn selectItem(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_SelectItem(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.handle(), indent) }
    }
    #[fixed_stack_segment]
    fn setItemBackgroundColour(&self, item: @wxTreeItemId, col: @wxColour) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBold(&self, item: @wxTreeItemId, bold: bool) {
        unsafe { wxTreeCtrl_SetItemBold(self.handle(), item.handle(), bold) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: @wxTreeItemId, data: *u8) {
        unsafe { wxTreeCtrl_SetItemData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setItemDropHighlight(&self, item: @wxTreeItemId, highlight: bool) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.handle(), item.handle(), highlight) }
    }
    #[fixed_stack_segment]
    fn setItemFont(&self, item: @wxTreeItemId, font: @wxFont) {
        unsafe { wxTreeCtrl_SetItemFont(self.handle(), item.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemHasChildren(&self, item: @wxTreeItemId, hasChildren: bool) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.handle(), item.handle(), hasChildren) }
    }
    #[fixed_stack_segment]
    fn setItemImage(&self, item: @wxTreeItemId, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.handle(), item.handle(), image, which) }
    }
    #[fixed_stack_segment]
    fn setItemText(&self, item: @wxTreeItemId, text: @wxString) {
        unsafe { wxTreeCtrl_SetItemText(self.handle(), item.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemTextColour(&self, item: @wxTreeItemId, col: @wxColour) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.handle(), spacing) }
    }
    #[fixed_stack_segment]
    fn setStateImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_SetStateImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn sortChildren(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_SortChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn toggle(&self, item: @wxTreeItemId) {
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
    fn new2(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrlImpl(wxTreeCtrl_Create2(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxTreeCtrl }
    }
    #[fixed_stack_segment]
    fn insertItem2(&self, parent: @wxWindow, idPrevious: @wxTreeItemId, text: @wxString, image: c_int, selectedImage: c_int, closure: @wxClosure, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItem2(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex2(&self, parent: @wxWindow, index: c_int, text: @wxString, image: c_int, selectedImage: c_int, closure: @wxClosure, _item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemClientClosure(&self, item: @wxTreeItemId) -> @wxClosure {
        unsafe { @wxClosureImpl(wxTreeCtrl_GetItemClientClosure(self.handle(), item.handle())) as @wxClosure }
    }
    #[fixed_stack_segment]
    fn setItemClientClosure(&self, item: @wxTreeItemId, closure: @wxClosure) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.handle(), item.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_AssignImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn assignStateImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.handle(), imageList.handle()) }
    }
}

struct wxTreeEventImpl(*u8);
impl wxTreeEvent for wxTreeEventImpl {}
impl wxNotifyEvent for wxTreeEventImpl {}
impl wxCommandEvent for wxTreeEventImpl {}
impl wxEvent for wxTreeEventImpl {}
impl wxObject for wxTreeEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeEvent : wxNotifyEvent {
    #[fixed_stack_segment]
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, _ref: @wxTreeItemId) {
        unsafe { wxTreeEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { @wxStringImpl(wxTreeEvent_GetLabel(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getOldItem(&self, _ref: @wxTreeItemId) {
        unsafe { wxTreeEvent_GetOldItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxTreeEvent_GetPoint(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getKeyEvent(&self) -> @wxKeyEvent {
        unsafe { @wxKeyEventImpl(wxTreeEvent_GetKeyEvent(self.handle())) as @wxKeyEvent }
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

struct wxTreeItemDataImpl(*u8);
impl wxTreeItemData for wxTreeItemDataImpl {}
impl wxClientData for wxTreeItemDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeItemData : wxClientData {
}

struct wxTreeItemIdImpl(*u8);
impl wxTreeItemId for wxTreeItemIdImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeItemId {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn new() -> @wxTreeItemId {
        unsafe { @wxTreeItemIdImpl(wxTreeItemId_Create()) as @wxTreeItemId }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxTreeItemId_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @wxTreeItemId {
        unsafe { @wxTreeItemIdImpl(wxTreeItemId_Clone(self.handle())) as @wxTreeItemId }
    }
    #[fixed_stack_segment]
    fn newFromValue(value: intptr_t) -> @wxTreeItemId {
        unsafe { @wxTreeItemIdImpl(wxTreeItemId_CreateFromValue(value)) as @wxTreeItemId }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.handle()) }
    }
}

struct wxTreeLayoutImpl(*u8);
impl wxTreeLayout for wxTreeLayoutImpl {}
impl wxObject for wxTreeLayoutImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeLayout : wxObject {
}

struct wxTreeLayoutStoredImpl(*u8);
impl wxTreeLayoutStored for wxTreeLayoutStoredImpl {}
impl wxTreeLayout for wxTreeLayoutStoredImpl {}
impl wxObject for wxTreeLayoutStoredImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxTreeLayoutStored : wxTreeLayout {
}

struct wxURLImpl(*u8);
impl wxURL for wxURLImpl {}
impl wxObject for wxURLImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxURL : wxObject {
}

struct wxUpdateUIEventImpl(*u8);
impl wxUpdateUIEvent for wxUpdateUIEventImpl {}
impl wxEvent for wxUpdateUIEventImpl {}
impl wxObject for wxUpdateUIEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxUpdateUIEvent : wxEvent {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxUpdateUIEvent_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
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
    fn getText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxUpdateUIEvent_GetText(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @wxString) {
        unsafe { wxUpdateUIEvent_SetText(self.handle(), text.handle()) }
    }
}

struct wxValidatorImpl(*u8);
impl wxValidator for wxValidatorImpl {}
impl wxEvtHandler for wxValidatorImpl {}
impl wxObject for wxValidatorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxValidator : wxEvtHandler {
    #[fixed_stack_segment]
    fn new() -> @wxValidator {
        unsafe { @wxValidatorImpl(wxValidator_Create()) as @wxValidator }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxValidator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxValidator_GetWindow(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn setBellOnError(doIt: bool) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, win: @wxWindow) {
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
    fn validate(&self, parent: @wxWindow) -> bool {
        unsafe { wxValidator_Validate(self.handle(), parent.handle()) }
    }
}

struct wxVariantImpl(*u8);
impl wxVariant for wxVariantImpl {}
impl wxObject for wxVariantImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxVariant : wxObject {
}

struct wxVariantDataImpl(*u8);
impl wxVariantData for wxVariantDataImpl {}
impl wxObject for wxVariantDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxVariantData : wxObject {
}

struct wxViewImpl(*u8);
impl wxView for wxViewImpl {}
impl wxEvtHandler for wxViewImpl {}
impl wxObject for wxViewImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxView : wxEvtHandler {
}

struct wxSoundImpl(*u8);
impl wxSound for wxSoundImpl {}
impl wxEvtHandler for wxSoundImpl {}
impl wxObject for wxSoundImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSound : wxEvtHandler {
    #[fixed_stack_segment]
    fn new(fileName: @wxString, isResource: bool) -> @wxSound {
        unsafe { @wxSoundImpl(wxSound_Create(fileName.handle(), isResource)) as @wxSound }
    }
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

struct wxWindowImpl(*u8);
impl wxWindow for wxWindowImpl {}
impl wxEvtHandler for wxWindowImpl {}
impl wxObject for wxWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWindow : wxEvtHandler {
    #[fixed_stack_segment]
    fn addChild(&self, child: @wxWindow) {
        unsafe { wxWindow_AddChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn addConstraintReference(&self, otherWin: @wxWindow) {
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
    fn clientToScreen(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ClientToScreen(self.handle(), x, y)) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn close(&self, _force: bool) -> bool {
        unsafe { wxWindow_Close(self.handle(), _force) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixels(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ConvertDialogToPixels(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialog(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ConvertPixelsToDialog(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @wxWindow {
        unsafe { @wxWindowImpl(wxWindow_Create(_prt.handle(), _id, _x, _y, _w, _h, _stl)) as @wxWindow }
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
    fn findFocus(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxWindow_FindFocus(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn findWindow(&self, name: @wxString) -> @wxWindow {
        unsafe { @wxWindowImpl(wxWindow_FindWindow(self.handle(), name.handle())) as @wxWindow }
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
    fn getEffectiveMinSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxWindow_GetEffectiveMinSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @wxColour) {
        unsafe { wxWindow_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxWindow_GetBestSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getCaret(&self) -> @wxCaret {
        unsafe { @wxCaretImpl(wxWindow_GetCaret(self.handle())) as @wxCaret }
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
    fn getClientData(&self) -> @wxClientData {
        unsafe { @wxClientDataImpl(wxWindow_GetClientData(self.handle())) as @wxClientData }
    }
    #[fixed_stack_segment]
    fn getClientSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxWindow_GetClientSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getConstraints(&self) -> @wxLayoutConstraints {
        unsafe { @wxLayoutConstraintsImpl(wxWindow_GetConstraints(self.handle())) as @wxLayoutConstraints }
    }
    #[fixed_stack_segment]
    fn getConstraintsInvolvedIn(&self) -> *u8 {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCursor(&self) -> @wxCursor {
        unsafe { @wxCursorImpl(wxWindow_GetCursor(self.handle())) as @wxCursor }
    }
    #[fixed_stack_segment]
    fn getDropTarget(&self) -> @wxDropTarget {
        unsafe { @wxDropTargetImpl(wxWindow_GetDropTarget(self.handle())) as @wxDropTarget }
    }
    #[fixed_stack_segment]
    fn getEventHandler(&self) -> @wxEvtHandler {
        unsafe { @wxEvtHandlerImpl(wxWindow_GetEventHandler(self.handle())) as @wxEvtHandler }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @wxFont) {
        unsafe { wxWindow_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getForegroundColour(&self, _ref: @wxColour) {
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
    fn getLabel(&self) -> @wxString {
        unsafe { @wxStringImpl(wxWindow_GetLabel(self.handle())) as @wxString }
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
    fn getName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxWindow_GetName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxWindow_GetParent(self.handle())) as @wxWindow }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_GetPosition(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @wxRect {
        unsafe { @wxRectImpl(wxWindow_GetRect(self.handle())) as @wxRect }
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
    fn getSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxWindow_GetSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @wxSizer {
        unsafe { @wxSizerImpl(wxWindow_GetSizer(self.handle())) as @wxSizer }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @wxString, x: *c_int, y: *c_int, descent: *c_int, externalLeading: *c_int, theFont: @wxFont) {
        unsafe { wxWindow_GetTextExtent(self.handle(), string.handle(), x, y, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolTip(&self) -> @wxString {
        unsafe { @wxStringImpl(wxWindow_GetToolTip(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getUpdateRegion(&self) -> @wxRegion {
        unsafe { @wxRegionImpl(wxWindow_GetUpdateRegion(self.handle())) as @wxRegion }
    }
    #[fixed_stack_segment]
    fn getValidator(&self) -> @wxValidator {
        unsafe { @wxValidatorImpl(wxWindow_GetValidator(self.handle())) as @wxValidator }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxWindow_GetVirtualSize(self.handle())) as @wxSize }
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
    fn popupMenu(&self, menu: @wxMenu, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.handle(), menu.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @wxDC) {
        unsafe { wxWindow_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn pushEventHandler(&self, handler: @wxEvtHandler) {
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
    fn removeChild(&self, child: @wxWindow) {
        unsafe { wxWindow_RemoveChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn removeConstraintReference(&self, otherWin: @wxWindow) {
        unsafe { wxWindow_RemoveConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    fn reparent(&self, _par: @wxWindow) -> c_int {
        unsafe { wxWindow_Reparent(self.handle(), _par.handle()) }
    }
    #[fixed_stack_segment]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    fn screenToClient(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ScreenToClient(self.handle(), x, y)) as @wxPoint }
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
    fn setAcceleratorTable(&self, accel: @wxAcceleratorTable) {
        unsafe { wxWindow_SetAcceleratorTable(self.handle(), accel.handle()) }
    }
    #[fixed_stack_segment]
    fn setAutoLayout(&self, autoLayout: bool) {
        unsafe { wxWindow_SetAutoLayout(self.handle(), autoLayout) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @wxColour) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaret(&self, caret: @wxCaret) {
        unsafe { wxWindow_SetCaret(self.handle(), caret.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, data: @wxClientData) {
        unsafe { wxWindow_SetClientData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, data: @wxClientData) {
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
    fn setConstraints(&self, constraints: @wxLayoutConstraints) {
        unsafe { wxWindow_SetConstraints(self.handle(), constraints.handle()) }
    }
    #[fixed_stack_segment]
    fn setCursor(&self, cursor: @wxCursor) -> c_int {
        unsafe { wxWindow_SetCursor(self.handle(), cursor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDropTarget(&self, dropTarget: @wxDropTarget) {
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
    fn setFont(&self, font: @wxFont) -> c_int {
        unsafe { wxWindow_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, colour: @wxColour) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.handle(), _id) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, _title: @wxString) {
        unsafe { wxWindow_SetLabel(self.handle(), _title.handle()) }
    }
    #[fixed_stack_segment]
    fn setName(&self, _name: @wxString) {
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
    fn setSizer(&self, sizer: @wxSizer) {
        unsafe { wxWindow_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolTip(&self, tip: @wxString) {
        unsafe { wxWindow_SetToolTip(self.handle(), tip.handle()) }
    }
    #[fixed_stack_segment]
    fn setValidator(&self, validator: @wxValidator) {
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
    fn convertDialogToPixelsEx(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ConvertDialogToPixelsEx(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialogEx(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ConvertPixelsToDialogEx(self.handle())) as @wxPoint }
    }
    #[fixed_stack_segment]
    fn screenToClient2(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { @wxPointImpl(wxWindow_ScreenToClient2(self.handle(), x, y)) as @wxPoint }
    }
}

struct wxWindowCreateEventImpl(*u8);
impl wxWindowCreateEvent for wxWindowCreateEventImpl {}
impl wxCommandEvent for wxWindowCreateEventImpl {}
impl wxEvent for wxWindowCreateEventImpl {}
impl wxObject for wxWindowCreateEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWindowCreateEvent : wxCommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxWindowCreateEvent_GetWindow(self.handle())) as @wxWindow }
    }
}

struct wxWindowDCImpl(*u8);
impl wxWindowDC for wxWindowDCImpl {}
impl wxDC for wxWindowDCImpl {}
impl wxObject for wxWindowDCImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWindowDC : wxDC {
    #[fixed_stack_segment]
    fn new(win: @wxWindow) -> @wxWindowDC {
        unsafe { @wxWindowDCImpl(wxWindowDC_Create(win.handle())) as @wxWindowDC }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxWindowDC_Delete(self.handle()) }
    }
}

struct wxWindowDestroyEventImpl(*u8);
impl wxWindowDestroyEvent for wxWindowDestroyEventImpl {}
impl wxCommandEvent for wxWindowDestroyEventImpl {}
impl wxEvent for wxWindowDestroyEventImpl {}
impl wxObject for wxWindowDestroyEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWindowDestroyEvent : wxCommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { @wxWindowImpl(wxWindowDestroyEvent_GetWindow(self.handle())) as @wxWindow }
    }
}

struct wxWindowDisablerImpl(*u8);
impl wxWindowDisabler for wxWindowDisablerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWindowDisabler {
    fn handle(&self) -> *u8;
    
}

struct wxWizardImpl(*u8);
impl wxWizard for wxWizardImpl {}
impl wxDialog for wxWizardImpl {}
impl wxTopLevelWindow for wxWizardImpl {}
impl wxWindow for wxWizardImpl {}
impl wxEvtHandler for wxWizardImpl {}
impl wxObject for wxWizardImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWizard : wxDialog {
    #[fixed_stack_segment]
    fn chain(f: @wxWizardPageSimple, s: @wxWizardPageSimple) {
        unsafe { wxWizard_Chain(f.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _bmp: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> @wxWizard {
        unsafe { @wxWizardImpl(wxWizard_Create(_prt.handle(), _id, _txt.handle(), _bmp.handle(), _lft, _top, _wdt, _hgt)) as @wxWizard }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> @wxWizardPage {
        unsafe { @wxWizardPageImpl(wxWizard_GetCurrentPage(self.handle())) as @wxWizardPage }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxWizard_GetPageSize(self.handle())) as @wxSize }
    }
    #[fixed_stack_segment]
    fn runWizard(&self, firstPage: @wxWizardPage) -> c_int {
        unsafe { wxWizard_RunWizard(self.handle(), firstPage.handle()) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.handle(), w, h) }
    }
}

struct wxWizardEventImpl(*u8);
impl wxWizardEvent for wxWizardEventImpl {}
impl wxNotifyEvent for wxWizardEventImpl {}
impl wxCommandEvent for wxWizardEventImpl {}
impl wxEvent for wxWizardEventImpl {}
impl wxObject for wxWizardEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWizardEvent : wxNotifyEvent {
    #[fixed_stack_segment]
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.handle()) }
    }
}

struct wxWizardPageImpl(*u8);
impl wxWizardPage for wxWizardPageImpl {}
impl wxPanel for wxWizardPageImpl {}
impl wxWindow for wxWizardPageImpl {}
impl wxEvtHandler for wxWizardPageImpl {}
impl wxObject for wxWizardPageImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWizardPage : wxPanel {
}

struct wxWizardPageSimpleImpl(*u8);
impl wxWizardPageSimple for wxWizardPageSimpleImpl {}
impl wxWizardPage for wxWizardPageSimpleImpl {}
impl wxPanel for wxWizardPageSimpleImpl {}
impl wxWindow for wxWizardPageSimpleImpl {}
impl wxEvtHandler for wxWizardPageSimpleImpl {}
impl wxObject for wxWizardPageSimpleImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxWizardPageSimple : wxWizardPage {
    #[fixed_stack_segment]
    fn new(_prt: @wxWizard) -> @wxWizardPageSimple {
        unsafe { @wxWizardPageSimpleImpl(wxWizardPageSimple_Create(_prt.handle())) as @wxWizardPageSimple }
    }
    #[fixed_stack_segment]
    fn getBitmap(&self, _ref: @wxBitmap) {
        unsafe { wxWizardPageSimple_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getNext(&self) -> @wxWizardPageSimple {
        unsafe { @wxWizardPageSimpleImpl(wxWizardPageSimple_GetNext(self.handle())) as @wxWizardPageSimple }
    }
    #[fixed_stack_segment]
    fn getPrev(&self) -> @wxWizardPageSimple {
        unsafe { @wxWizardPageSimpleImpl(wxWizardPageSimple_GetPrev(self.handle())) as @wxWizardPageSimple }
    }
    #[fixed_stack_segment]
    fn setNext(&self, next: @wxWizardPageSimple) {
        unsafe { wxWizardPageSimple_SetNext(self.handle(), next.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrev(&self, prev: @wxWizardPageSimple) {
        unsafe { wxWizardPageSimple_SetPrev(self.handle(), prev.handle()) }
    }
}

struct wxXmlResourceImpl(*u8);
impl wxXmlResource for wxXmlResourceImpl {}
impl wxObject for wxXmlResourceImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxXmlResource : wxObject {
    #[fixed_stack_segment]
    fn addHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn addSubclassFactory(&self, factory: *u8) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    #[fixed_stack_segment]
    fn attachUnknownControl(&self, control: @wxControl, parent: @wxWindow) -> c_int {
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
    fn new(flags: c_int) -> @wxXmlResource {
        unsafe { @wxXmlResourceImpl(wxXmlResource_Create(flags)) as @wxXmlResource }
    }
    #[fixed_stack_segment]
    fn newFromFile(filemask: @wxString, flags: c_int) -> @wxXmlResource {
        unsafe { @wxXmlResourceImpl(wxXmlResource_CreateFromFile(filemask.handle(), flags)) as @wxXmlResource }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxXmlResource_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn get() -> @wxXmlResource {
        unsafe { @wxXmlResourceImpl(wxXmlResource_Get()) as @wxXmlResource }
    }
    #[fixed_stack_segment]
    fn getDomain(&self) -> @wxString {
        unsafe { @wxStringImpl(wxXmlResource_GetDomain(self.handle())) as @wxString }
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
    fn getXRCID(&self, str_id: @wxString) -> c_int {
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    #[fixed_stack_segment]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insertHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, filemask: @wxString) -> bool {
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn loadBitmap(&self, name: @wxString, _ref: @wxBitmap) {
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadDialog(&self, parent: @wxWindow, name: @wxString) -> @wxDialog {
        unsafe { @wxDialogImpl(wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle())) as @wxDialog }
    }
    #[fixed_stack_segment]
    fn loadFrame(&self, parent: @wxWindow, name: @wxString) -> @wxFrame {
        unsafe { @wxFrameImpl(wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle())) as @wxFrame }
    }
    #[fixed_stack_segment]
    fn loadIcon(&self, name: @wxString, _ref: @wxIcon) {
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadMenu(&self, name: @wxString) -> @wxMenu {
        unsafe { @wxMenuImpl(wxXmlResource_LoadMenu(self.handle(), name.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn loadMenuBar(&self, parent: @wxWindow, name: @wxString) -> @wxMenuBar {
        unsafe { @wxMenuBarImpl(wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle())) as @wxMenuBar }
    }
    #[fixed_stack_segment]
    fn loadPanel(&self, parent: @wxWindow, name: @wxString) -> @wxPanel {
        unsafe { @wxPanelImpl(wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle())) as @wxPanel }
    }
    #[fixed_stack_segment]
    fn loadToolBar(&self, parent: @wxWindow, name: @wxString) -> @wxToolBar {
        unsafe { @wxToolBarImpl(wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle())) as @wxToolBar }
    }
    #[fixed_stack_segment]
    fn getSizer(&self, str_id: @wxString) -> @wxSizer {
        unsafe { @wxSizerImpl(wxXmlResource_GetSizer(self.handle(), str_id.handle())) as @wxSizer }
    }
    #[fixed_stack_segment]
    fn getBoxSizer(&self, str_id: @wxString) -> @wxBoxSizer {
        unsafe { @wxBoxSizerImpl(wxXmlResource_GetBoxSizer(self.handle(), str_id.handle())) as @wxBoxSizer }
    }
    #[fixed_stack_segment]
    fn getStaticBoxSizer(&self, str_id: @wxString) -> @wxStaticBoxSizer {
        unsafe { @wxStaticBoxSizerImpl(wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle())) as @wxStaticBoxSizer }
    }
    #[fixed_stack_segment]
    fn getGridSizer(&self, str_id: @wxString) -> @wxGridSizer {
        unsafe { @wxGridSizerImpl(wxXmlResource_GetGridSizer(self.handle(), str_id.handle())) as @wxGridSizer }
    }
    #[fixed_stack_segment]
    fn getFlexGridSizer(&self, str_id: @wxString) -> @wxFlexGridSizer {
        unsafe { @wxFlexGridSizerImpl(wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle())) as @wxFlexGridSizer }
    }
    #[fixed_stack_segment]
    fn getBitmapButton(&self, str_id: @wxString) -> @wxBitmapButton {
        unsafe { @wxBitmapButtonImpl(wxXmlResource_GetBitmapButton(self.handle(), str_id.handle())) as @wxBitmapButton }
    }
    #[fixed_stack_segment]
    fn getButton(&self, str_id: @wxString) -> @wxButton {
        unsafe { @wxButtonImpl(wxXmlResource_GetButton(self.handle(), str_id.handle())) as @wxButton }
    }
    #[fixed_stack_segment]
    fn getCalendarCtrl(&self, str_id: @wxString) -> @wxCalendarCtrl {
        unsafe { @wxCalendarCtrlImpl(wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle())) as @wxCalendarCtrl }
    }
    #[fixed_stack_segment]
    fn getCheckBox(&self, str_id: @wxString) -> @wxCheckBox {
        unsafe { @wxCheckBoxImpl(wxXmlResource_GetCheckBox(self.handle(), str_id.handle())) as @wxCheckBox }
    }
    #[fixed_stack_segment]
    fn getCheckListBox(&self, str_id: @wxString) -> @wxCheckListBox {
        unsafe { @wxCheckListBoxImpl(wxXmlResource_GetCheckListBox(self.handle(), str_id.handle())) as @wxCheckListBox }
    }
    #[fixed_stack_segment]
    fn getChoice(&self, str_id: @wxString) -> @wxChoice {
        unsafe { @wxChoiceImpl(wxXmlResource_GetChoice(self.handle(), str_id.handle())) as @wxChoice }
    }
    #[fixed_stack_segment]
    fn getComboBox(&self, str_id: @wxString) -> @wxComboBox {
        unsafe { @wxComboBoxImpl(wxXmlResource_GetComboBox(self.handle(), str_id.handle())) as @wxComboBox }
    }
    #[fixed_stack_segment]
    fn getGauge(&self, str_id: @wxString) -> @wxGauge {
        unsafe { @wxGaugeImpl(wxXmlResource_GetGauge(self.handle(), str_id.handle())) as @wxGauge }
    }
    #[fixed_stack_segment]
    fn getGrid(&self, str_id: @wxString) -> @wxGrid {
        unsafe { @wxGridImpl(wxXmlResource_GetGrid(self.handle(), str_id.handle())) as @wxGrid }
    }
    #[fixed_stack_segment]
    fn getHtmlWindow(&self, str_id: @wxString) -> @wxHtmlWindow {
        unsafe { @wxHtmlWindowImpl(wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle())) as @wxHtmlWindow }
    }
    #[fixed_stack_segment]
    fn getListBox(&self, str_id: @wxString) -> @wxListBox {
        unsafe { @wxListBoxImpl(wxXmlResource_GetListBox(self.handle(), str_id.handle())) as @wxListBox }
    }
    #[fixed_stack_segment]
    fn getListCtrl(&self, str_id: @wxString) -> @wxListCtrl {
        unsafe { @wxListCtrlImpl(wxXmlResource_GetListCtrl(self.handle(), str_id.handle())) as @wxListCtrl }
    }
    #[fixed_stack_segment]
    fn getMDIChildFrame(&self, str_id: @wxString) -> @wxMDIChildFrame {
        unsafe { @wxMDIChildFrameImpl(wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle())) as @wxMDIChildFrame }
    }
    #[fixed_stack_segment]
    fn getMDIParentFrame(&self, str_id: @wxString) -> @wxMDIParentFrame {
        unsafe { @wxMDIParentFrameImpl(wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle())) as @wxMDIParentFrame }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, str_id: @wxString) -> @wxMenu {
        unsafe { @wxMenuImpl(wxXmlResource_GetMenu(self.handle(), str_id.handle())) as @wxMenu }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self, str_id: @wxString) -> @wxMenuBar {
        unsafe { @wxMenuBarImpl(wxXmlResource_GetMenuBar(self.handle(), str_id.handle())) as @wxMenuBar }
    }
    #[fixed_stack_segment]
    fn getMenuItem(&self, str_id: @wxString) -> @wxMenuItem {
        unsafe { @wxMenuItemImpl(wxXmlResource_GetMenuItem(self.handle(), str_id.handle())) as @wxMenuItem }
    }
    #[fixed_stack_segment]
    fn getNotebook(&self, str_id: @wxString) -> @wxNotebook {
        unsafe { @wxNotebookImpl(wxXmlResource_GetNotebook(self.handle(), str_id.handle())) as @wxNotebook }
    }
    #[fixed_stack_segment]
    fn getPanel(&self, str_id: @wxString) -> @wxPanel {
        unsafe { @wxPanelImpl(wxXmlResource_GetPanel(self.handle(), str_id.handle())) as @wxPanel }
    }
    #[fixed_stack_segment]
    fn getRadioButton(&self, str_id: @wxString) -> @wxRadioButton {
        unsafe { @wxRadioButtonImpl(wxXmlResource_GetRadioButton(self.handle(), str_id.handle())) as @wxRadioButton }
    }
    #[fixed_stack_segment]
    fn getRadioBox(&self, str_id: @wxString) -> @wxRadioBox {
        unsafe { @wxRadioBoxImpl(wxXmlResource_GetRadioBox(self.handle(), str_id.handle())) as @wxRadioBox }
    }
    #[fixed_stack_segment]
    fn getScrollBar(&self, str_id: @wxString) -> @wxScrollBar {
        unsafe { @wxScrollBarImpl(wxXmlResource_GetScrollBar(self.handle(), str_id.handle())) as @wxScrollBar }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow(&self, str_id: @wxString) -> @wxScrolledWindow {
        unsafe { @wxScrolledWindowImpl(wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle())) as @wxScrolledWindow }
    }
    #[fixed_stack_segment]
    fn getSlider(&self, str_id: @wxString) -> @wxSlider {
        unsafe { @wxSliderImpl(wxXmlResource_GetSlider(self.handle(), str_id.handle())) as @wxSlider }
    }
    #[fixed_stack_segment]
    fn getSpinButton(&self, str_id: @wxString) -> @wxSpinButton {
        unsafe { @wxSpinButtonImpl(wxXmlResource_GetSpinButton(self.handle(), str_id.handle())) as @wxSpinButton }
    }
    #[fixed_stack_segment]
    fn getSpinCtrl(&self, str_id: @wxString) -> @wxSpinCtrl {
        unsafe { @wxSpinCtrlImpl(wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle())) as @wxSpinCtrl }
    }
    #[fixed_stack_segment]
    fn getSplitterWindow(&self, str_id: @wxString) -> @wxSplitterWindow {
        unsafe { @wxSplitterWindowImpl(wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle())) as @wxSplitterWindow }
    }
    #[fixed_stack_segment]
    fn getStaticBitmap(&self, str_id: @wxString) -> @wxStaticBitmap {
        unsafe { @wxStaticBitmapImpl(wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle())) as @wxStaticBitmap }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self, str_id: @wxString) -> @wxStaticBox {
        unsafe { @wxStaticBoxImpl(wxXmlResource_GetStaticBox(self.handle(), str_id.handle())) as @wxStaticBox }
    }
    #[fixed_stack_segment]
    fn getStaticLine(&self, str_id: @wxString) -> @wxStaticLine {
        unsafe { @wxStaticLineImpl(wxXmlResource_GetStaticLine(self.handle(), str_id.handle())) as @wxStaticLine }
    }
    #[fixed_stack_segment]
    fn getStaticText(&self, str_id: @wxString) -> @wxStaticText {
        unsafe { @wxStaticTextImpl(wxXmlResource_GetStaticText(self.handle(), str_id.handle())) as @wxStaticText }
    }
    #[fixed_stack_segment]
    fn getTextCtrl(&self, str_id: @wxString) -> @wxTextCtrl {
        unsafe { @wxTextCtrlImpl(wxXmlResource_GetTextCtrl(self.handle(), str_id.handle())) as @wxTextCtrl }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl(&self, str_id: @wxString) -> @wxTreeCtrl {
        unsafe { @wxTreeCtrlImpl(wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle())) as @wxTreeCtrl }
    }
    #[fixed_stack_segment]
    fn unload(&self, filemask: @wxString) -> bool {
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, res: @wxXmlResource) -> @wxXmlResource {
        unsafe { @wxXmlResourceImpl(wxXmlResource_Set(self.handle(), res.handle())) as @wxXmlResource }
    }
    #[fixed_stack_segment]
    fn setDomain(&self, domain: @wxString) {
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn getStyledTextCtrl(&self, str_id: @wxString) -> @wxStyledTextCtrl {
        unsafe { @wxStyledTextCtrlImpl(wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle())) as @wxStyledTextCtrl }
    }
}

struct wxXmlResourceHandlerImpl(*u8);
impl wxXmlResourceHandler for wxXmlResourceHandlerImpl {}
impl wxObject for wxXmlResourceHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxXmlResourceHandler : wxObject {
}

struct wxZipInputStreamImpl(*u8);
impl wxZipInputStream for wxZipInputStreamImpl {}
impl wxInputStream for wxZipInputStreamImpl {}
impl wxStreamBase for wxZipInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxZipInputStream : wxInputStream {
}

struct wxZlibInputStreamImpl(*u8);
impl wxZlibInputStream for wxZlibInputStreamImpl {}
impl wxFilterInputStream for wxZlibInputStreamImpl {}
impl wxInputStream for wxZlibInputStreamImpl {}
impl wxStreamBase for wxZlibInputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxZlibInputStream : wxFilterInputStream {
}

struct wxZlibOutputStreamImpl(*u8);
impl wxZlibOutputStream for wxZlibOutputStreamImpl {}
impl wxFilterOutputStream for wxZlibOutputStreamImpl {}
impl wxOutputStream for wxZlibOutputStreamImpl {}
impl wxStreamBase for wxZlibOutputStreamImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxZlibOutputStream : wxFilterOutputStream {
}

struct wxPropertyGridImpl(*u8);
impl wxPropertyGrid for wxPropertyGridImpl {}
impl wxControl for wxPropertyGridImpl {}
impl wxWindow for wxPropertyGridImpl {}
impl wxEvtHandler for wxPropertyGridImpl {}
impl wxObject for wxPropertyGridImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPropertyGrid : wxControl {
    #[fixed_stack_segment]
    fn append(&self, prop: @wxPGProperty) -> @wxPGProperty {
        unsafe { @wxPGPropertyImpl(wxPropertyGrid_Append(self.handle(), prop.handle())) as @wxPGProperty }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @wxPropertyGrid {
        unsafe { @wxPropertyGridImpl(wxPropertyGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @wxPropertyGrid }
    }
    #[fixed_stack_segment]
    fn disableProperty(&self, propName: @wxString) -> bool {
        unsafe { wxPropertyGrid_DisableProperty(self.handle(), propName.handle()) }
    }
}

struct wxPropertyGridEventImpl(*u8);
impl wxPropertyGridEvent for wxPropertyGridEventImpl {}
impl wxNotifyEvent for wxPropertyGridEventImpl {}
impl wxCommandEvent for wxPropertyGridEventImpl {}
impl wxEvent for wxPropertyGridEventImpl {}
impl wxObject for wxPropertyGridEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPropertyGridEvent : wxNotifyEvent {
    #[fixed_stack_segment]
    fn hasProperty(&self) -> bool {
        unsafe { wxPropertyGridEvent_HasProperty(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getProperty(&self) -> @wxPGProperty {
        unsafe { @wxPGPropertyImpl(wxPropertyGridEvent_GetProperty(self.handle())) as @wxPGProperty }
    }
}

struct wxPGPropertyImpl(*u8);
impl wxPGProperty for wxPGPropertyImpl {}
impl wxObject for wxPGPropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPGProperty : wxObject {
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPGProperty_GetLabel(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPGProperty_GetName(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getValueAsString(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPGProperty_GetValueAsString(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getValueType(&self) -> @wxString {
        unsafe { @wxStringImpl(wxPGProperty_GetValueType(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, helpString: @wxString) {
        unsafe { wxPGProperty_SetHelpString(self.handle(), helpString.handle()) }
    }
}

struct wxStringPropertyImpl(*u8);
impl wxStringProperty for wxStringPropertyImpl {}
impl wxPGProperty for wxStringPropertyImpl {}
impl wxObject for wxStringPropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStringProperty : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: @wxString) -> @wxStringProperty {
        unsafe { @wxStringPropertyImpl(wxStringProperty_Create(label.handle(), name.handle(), value.handle())) as @wxStringProperty }
    }
}

struct wxIntPropertyImpl(*u8);
impl wxIntProperty for wxIntPropertyImpl {}
impl wxPGProperty for wxIntPropertyImpl {}
impl wxObject for wxIntPropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxIntProperty : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: c_int) -> @wxIntProperty {
        unsafe { @wxIntPropertyImpl(wxIntProperty_Create(label.handle(), name.handle(), value)) as @wxIntProperty }
    }
}

struct wxBoolPropertyImpl(*u8);
impl wxBoolProperty for wxBoolPropertyImpl {}
impl wxPGProperty for wxBoolPropertyImpl {}
impl wxObject for wxBoolPropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxBoolProperty : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: bool) -> @wxBoolProperty {
        unsafe { @wxBoolPropertyImpl(wxBoolProperty_Create(label.handle(), name.handle(), value)) as @wxBoolProperty }
    }
}

struct wxFloatPropertyImpl(*u8);
impl wxFloatProperty for wxFloatPropertyImpl {}
impl wxPGProperty for wxFloatPropertyImpl {}
impl wxObject for wxFloatPropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFloatProperty : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: c_float) -> @wxFloatProperty {
        unsafe { @wxFloatPropertyImpl(wxFloatProperty_Create(label.handle(), name.handle(), value)) as @wxFloatProperty }
    }
}

struct wxDatePropertyImpl(*u8);
impl wxDateProperty for wxDatePropertyImpl {}
impl wxPGProperty for wxDatePropertyImpl {}
impl wxObject for wxDatePropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxDateProperty : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: @wxDateTime) -> @wxDateProperty {
        unsafe { @wxDatePropertyImpl(wxDateProperty_Create(label.handle(), name.handle(), value.handle())) as @wxDateProperty }
    }
}

struct wxFilePropertyImpl(*u8);
impl wxFileProperty for wxFilePropertyImpl {}
impl wxPGProperty for wxFilePropertyImpl {}
impl wxObject for wxFilePropertyImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileProperty : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: @wxString) -> @wxFileProperty {
        unsafe { @wxFilePropertyImpl(wxFileProperty_Create(label.handle(), name.handle(), value.handle())) as @wxFileProperty }
    }
}

struct wxPropertyCategoryImpl(*u8);
impl wxPropertyCategory for wxPropertyCategoryImpl {}
impl wxPGProperty for wxPropertyCategoryImpl {}
impl wxObject for wxPropertyCategoryImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxPropertyCategory : wxPGProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString) -> @wxPropertyCategory {
        unsafe { @wxPropertyCategoryImpl(wxPropertyCategory_Create(label.handle())) as @wxPropertyCategory }
    }
}

struct wxGenericDragImageImpl(*u8);
impl wxGenericDragImage for wxGenericDragImageImpl {}
impl wxDragImage for wxGenericDragImageImpl {}
impl wxObject for wxGenericDragImageImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGenericDragImage : wxDragImage {
    #[fixed_stack_segment]
    fn new(cursor: @wxCursor) -> @wxGenericDragImage {
        unsafe { @wxGenericDragImageImpl(wxGenericDragImage_Create(cursor.handle())) as @wxGenericDragImage }
    }
    #[fixed_stack_segment]
    fn doDrawImage(&self, dc: @wxDC, x: c_int, y: c_int) -> bool {
        unsafe { wxGenericDragImage_DoDrawImage(self.handle(), dc.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> @wxRect {
        unsafe { @wxRectImpl(wxGenericDragImage_GetImageRect(self.handle(), x_pos, y_pos)) as @wxRect }
    }
    #[fixed_stack_segment]
    fn updateBackingFromWindow(&self, windowDC: @wxDC, destDC: @wxMemoryDC, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.handle(), windowDC.handle(), destDC.handle(), x, y, w, h, xdest, ydest, width, height) }
    }
}

struct wxGraphicsObjectImpl(*u8);
impl wxGraphicsObject for wxGraphicsObjectImpl {}
impl wxObject for wxGraphicsObjectImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsObject : wxObject {
    #[fixed_stack_segment]
    fn getRenderer() -> @wxGraphicsRenderer {
        unsafe { @wxGraphicsRendererImpl(wxGraphicsObject_GetRenderer()) as @wxGraphicsRenderer }
    }
    #[fixed_stack_segment]
    fn isNull(&self) -> bool {
        unsafe { wxGraphicsObject_IsNull(self.handle()) }
    }
}

struct wxGraphicsBrushImpl(*u8);
impl wxGraphicsBrush for wxGraphicsBrushImpl {}
impl wxGraphicsObject for wxGraphicsBrushImpl {}
impl wxObject for wxGraphicsBrushImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsBrush : wxGraphicsObject {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsBrush {
        unsafe { @wxGraphicsBrushImpl(wxGraphicsBrush_Create()) as @wxGraphicsBrush }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsBrush_Delete(self.handle()) }
    }
}

struct wxGraphicsContextImpl(*u8);
impl wxGraphicsContext for wxGraphicsContextImpl {}
impl wxGraphicsObject for wxGraphicsContextImpl {}
impl wxObject for wxGraphicsContextImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsContext : wxGraphicsObject {
    #[fixed_stack_segment]
    fn new(dc: @wxWindowDC) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsContext_Create(dc.handle())) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn newFromWindow(window: @wxWindow) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsContext_CreateFromWindow(window.handle())) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsContext_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn newFromNative(context: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsContext_CreateFromNative(context)) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn newFromNativeWindow(window: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsContext_CreateFromNativeWindow(window)) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn clip(&self, region: @wxRegion) {
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
    fn drawBitmap(&self, bmp: @wxBitmap, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.handle(), bmp.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawIcon(&self, icon: @wxIcon, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.handle(), icon.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.handle(), n, x, y, style) }
    }
    #[fixed_stack_segment]
    fn drawPath(&self, path: @wxGraphicsPath, style: c_int) {
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
    fn drawText(&self, text: @wxString, x: c_double, y: c_double) {
        unsafe { wxGraphicsContext_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawTextWithAngle(&self, text: @wxString, x: c_double, y: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.handle(), text.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    fn fillPath(&self, path: @wxGraphicsPath, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    fn strokePath(&self, path: @wxGraphicsPath) {
        unsafe { wxGraphicsContext_StrokePath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativeContext(&self) -> *u8 {
        unsafe { wxGraphicsContext_GetNativeContext(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, text: @wxString, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double) {
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
    fn setTransform(&self, path: @wxGraphicsMatrix) {
        unsafe { wxGraphicsContext_SetTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn concatTransform(&self, path: @wxGraphicsMatrix) {
        unsafe { wxGraphicsContext_ConcatTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @wxBrush) {
        unsafe { wxGraphicsContext_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsBrush(&self, brush: @wxGraphicsBrush) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont, colour: @wxColour) {
        unsafe { wxGraphicsContext_SetFont(self.handle(), font.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsFont(&self, font: @wxGraphicsFont) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @wxPen) {
        unsafe { wxGraphicsContext_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsPen(&self, pen: @wxGraphicsPen) {
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

struct wxGraphicsFontImpl(*u8);
impl wxGraphicsFont for wxGraphicsFontImpl {}
impl wxGraphicsObject for wxGraphicsFontImpl {}
impl wxObject for wxGraphicsFontImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsFont : wxGraphicsObject {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsFont {
        unsafe { @wxGraphicsFontImpl(wxGraphicsFont_Create()) as @wxGraphicsFont }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsFont_Delete(self.handle()) }
    }
}

struct wxGraphicsMatrixImpl(*u8);
impl wxGraphicsMatrix for wxGraphicsMatrixImpl {}
impl wxGraphicsObject for wxGraphicsMatrixImpl {}
impl wxObject for wxGraphicsMatrixImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsMatrix : wxGraphicsObject {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsMatrix {
        unsafe { @wxGraphicsMatrixImpl(wxGraphicsMatrix_Create()) as @wxGraphicsMatrix }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsMatrix_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn concat(&self, t: @wxGraphicsMatrix) {
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
    fn isEqual(&self, t: @wxGraphicsMatrix) -> bool {
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

struct wxGraphicsPathImpl(*u8);
impl wxGraphicsPath for wxGraphicsPathImpl {}
impl wxGraphicsObject for wxGraphicsPathImpl {}
impl wxObject for wxGraphicsPathImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsPath : wxGraphicsObject {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsPath {
        unsafe { @wxGraphicsPathImpl(wxGraphicsPath_Create()) as @wxGraphicsPath }
    }
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
    fn addPath(&self, x: c_double, y: c_double, path: @wxGraphicsPath) {
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
    fn transform(&self, matrix: @wxGraphicsMatrix) {
        unsafe { wxGraphicsPath_Transform(self.handle(), matrix.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativePath(&self) -> *u8 {
        unsafe { wxGraphicsPath_GetNativePath(self.handle()) }
    }
    #[fixed_stack_segment]
    fn unGetNativePath(p: *u8) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

struct wxGraphicsPenImpl(*u8);
impl wxGraphicsPen for wxGraphicsPenImpl {}
impl wxGraphicsObject for wxGraphicsPenImpl {}
impl wxObject for wxGraphicsPenImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsPen : wxGraphicsObject {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsPen {
        unsafe { @wxGraphicsPenImpl(wxGraphicsPen_Create()) as @wxGraphicsPen }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsPen_Delete(self.handle()) }
    }
}

struct wxGraphicsRendererImpl(*u8);
impl wxGraphicsRenderer for wxGraphicsRendererImpl {}
impl wxGraphicsObject for wxGraphicsRendererImpl {}
impl wxObject for wxGraphicsRendererImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGraphicsRenderer : wxGraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsRenderer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @wxGraphicsRenderer {
        unsafe { @wxGraphicsRendererImpl(wxGraphicsRenderer_GetDefaultRenderer(self.handle())) as @wxGraphicsRenderer }
    }
    #[fixed_stack_segment]
    fn newContext(dc: @wxWindowDC) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsRenderer_CreateContext(dc.handle())) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn newContextFromWindow(window: @wxWindow) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsRenderer_CreateContextFromWindow(window.handle())) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn newContextFromNativeContext(context: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsRenderer_CreateContextFromNativeContext(context)) as @wxGraphicsContext }
    }
    #[fixed_stack_segment]
    fn newContextFromNativeWindow(window: *u8) -> @wxGraphicsContext {
        unsafe { @wxGraphicsContextImpl(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) as @wxGraphicsContext }
    }
}

struct wxGLContextImpl(*u8);
impl wxGLContext for wxGLContextImpl {}
impl wxObject for wxGLContextImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGLContext : wxObject {
    #[fixed_stack_segment]
    fn new(win: @wxGLCanvas, other: @wxGLContext) -> @wxGLContext {
        unsafe { @wxGLContextImpl(wxGLContext_Create(win.handle(), other.handle())) as @wxGLContext }
    }
    #[fixed_stack_segment]
    fn newFromNull(win: @wxGLCanvas) -> @wxGLContext {
        unsafe { @wxGLContextImpl(wxGLContext_CreateFromNull(win.handle())) as @wxGLContext }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, win: @wxGLCanvas) -> bool {
        unsafe { wxGLContext_SetCurrent(self.handle(), win.handle()) }
    }
}

struct wxManagedPtrImpl(*u8);
impl wxManagedPtr for wxManagedPtrImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxManagedPtr {
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
    #[fixed_stack_segment]
    fn getDeleteFunction() -> *u8 {
        unsafe { wxManagedPtr_GetDeleteFunction() }
    }
    #[fixed_stack_segment]
    fn newFromObject(obj: @wxObject) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromObject(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromDateTime(obj: @wxDateTime) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromDateTime(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromGridCellCoordsArray(obj: @wxGridCellCoordsArray) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromGridCellCoordsArray(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(obj: @wxBitmap) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromBitmap(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromIcon(obj: @wxIcon) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromIcon(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromBrush(obj: @wxBrush) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromBrush(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromColour(obj: @wxColour) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromColour(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromCursor(obj: @wxCursor) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromCursor(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromFont(obj: @wxFont) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromFont(obj.handle())) as @wxManagedPtr }
    }
    #[fixed_stack_segment]
    fn newFromPen(obj: @wxPen) -> @wxManagedPtr {
        unsafe { @wxManagedPtrImpl(wxManagedPtr_CreateFromPen(obj.handle())) as @wxManagedPtr }
    }
}

struct wxMediaCtrlImpl(*u8);
impl wxMediaCtrl for wxMediaCtrlImpl {}
impl wxWindow for wxMediaCtrlImpl {}
impl wxEvtHandler for wxMediaCtrlImpl {}
impl wxObject for wxMediaCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMediaCtrl : wxWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, windowID: c_int, fileName: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: @wxString, name: @wxString) -> @wxMediaCtrl {
        unsafe { @wxMediaCtrlImpl(wxMediaCtrl_Create(parent.handle(), windowID, fileName.handle(), x, y, w, h, style, szBackend.handle(), name.handle())) as @wxMediaCtrl }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMediaCtrl_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @wxSize {
        unsafe { @wxSizeImpl(wxMediaCtrl_GetBestSize(self.handle())) as @wxSize }
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
    fn load(&self, fileName: @wxString) -> bool {
        unsafe { wxMediaCtrl_Load(self.handle(), fileName.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURI(&self, uri: @wxString) -> bool {
        unsafe { wxMediaCtrl_LoadURI(self.handle(), uri.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURIWithProxy(&self, uri: @wxString, proxy: @wxString) -> bool {
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

struct wxMediaEventImpl(*u8);
impl wxMediaEvent for wxMediaEventImpl {}
impl wxNotifyEvent for wxMediaEventImpl {}
impl wxCommandEvent for wxMediaEventImpl {}
impl wxEvent for wxMediaEventImpl {}
impl wxObject for wxMediaEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMediaEvent : wxNotifyEvent {
}

struct wxcPrintoutImpl(*u8);
impl wxcPrintout for wxcPrintoutImpl {}
impl wxPrintout for wxcPrintoutImpl {}
impl wxObject for wxcPrintoutImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxcPrintout : wxPrintout {
    #[fixed_stack_segment]
    fn new(title: @wxString) -> @wxcPrintout {
        unsafe { @wxcPrintoutImpl(wxcPrintout_Create(title.handle())) as @wxcPrintout }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxcPrintout_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
    #[fixed_stack_segment]
    fn getEvtHandler(&self) -> @wxcPrintoutHandler {
        unsafe { @wxcPrintoutHandlerImpl(wxcPrintout_GetEvtHandler(self.handle())) as @wxcPrintoutHandler }
    }
}

struct wxcPrintEventImpl(*u8);
impl wxcPrintEvent for wxcPrintEventImpl {}
impl wxEvent for wxcPrintEventImpl {}
impl wxObject for wxcPrintEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxcPrintEvent : wxEvent {
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @wxcPrintout {
        unsafe { @wxcPrintoutImpl(wxcPrintEvent_GetPrintout(self.handle())) as @wxcPrintout }
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

struct wxcPrintoutHandlerImpl(*u8);
impl wxcPrintoutHandler for wxcPrintoutHandlerImpl {}
impl wxEvtHandler for wxcPrintoutHandlerImpl {}
impl wxObject for wxcPrintoutHandlerImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxcPrintoutHandler : wxEvtHandler {
}

struct wxStyledTextCtrlImpl(*u8);
impl wxStyledTextCtrl for wxStyledTextCtrlImpl {}
impl wxControl for wxStyledTextCtrlImpl {}
impl wxWindow for wxStyledTextCtrlImpl {}
impl wxEvtHandler for wxStyledTextCtrlImpl {}
impl wxObject for wxStyledTextCtrlImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStyledTextCtrl : wxControl {
    #[fixed_stack_segment]
    fn addText(&self, text: @wxString) {
        unsafe { wxStyledTextCtrl_AddText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addStyledText(&self, data: @wxMemoryBuffer) {
        unsafe { wxStyledTextCtrl_AddStyledText(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn insertText(&self, pos: c_int, text: @wxString) {
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
    fn markerDefineBitmap(&self, markerNumber: c_int, bmp: @wxBitmap) {
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
    fn styleSetFaceName(&self, style: c_int, fontName: @wxString) {
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
    fn setWordChars(&self, characters: @wxString) {
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
    fn autoCompShow(&self, lenEntered: c_int, itemList: @wxString) {
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
    fn autoCompStops(&self, characterSet: @wxString) {
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
    fn autoCompSelect(&self, text: @wxString) {
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
    fn autoCompSetFillUps(&self, characterSet: @wxString) {
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
    fn userListShow(&self, listType: c_int, itemList: @wxString) {
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
    fn registerImage(&self, type_: c_int, bmp: @wxBitmap) {
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
    fn findText(&self, minPos: c_int, maxPos: c_int, text: @wxString, flags: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_FindText(self.handle(), minPos, maxPos, text.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn formatRange(&self, doDraw: bool, startPos: c_int, endPos: c_int, draw: @wxDC, target: @wxDC, renderRect: @wxRect, pageRect: @wxRect) -> c_int {
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
    fn replaceSelection(&self, text: @wxString) {
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
    fn setText(&self, text: @wxString) {
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
    fn replaceTarget(&self, text: @wxString) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceTargetRE(&self, text: @wxString) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchInTarget(&self, text: @wxString) -> c_int {
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
    fn callTipShow(&self, pos: c_int, definition: @wxString) {
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
    fn textWidth(&self, style: c_int, text: @wxString) -> c_int {
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
    fn appendText(&self, text: @wxString) {
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
    fn setDocPointer(&self, docPointer: @wxSTCDoc) {
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
    fn searchNext(&self, flags: c_int, text: @wxString) -> c_int {
        unsafe { wxStyledTextCtrl_SearchNext(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchPrev(&self, flags: c_int, text: @wxString) -> c_int {
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
    fn addRefDocument(&self, docPointer: @wxSTCDoc) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    fn releaseDocument(&self, docPointer: @wxSTCDoc) {
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
    fn copyText(&self, length: c_int, text: @wxString) {
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
    fn setProperty(&self, key: @wxString, value: @wxString) {
        unsafe { wxStyledTextCtrl_SetProperty(self.handle(), key.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setKeyWords(&self, keywordSet: c_int, keyWords: @wxString) {
        unsafe { wxStyledTextCtrl_SetKeyWords(self.handle(), keywordSet, keyWords.handle()) }
    }
    #[fixed_stack_segment]
    fn setLexerLanguage(&self, language: @wxString) {
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self.handle(), language.handle()) }
    }
    #[fixed_stack_segment]
    fn getCurrentLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetSpec(&self, styleNum: c_int, spec: @wxString) {
        unsafe { wxStyledTextCtrl_StyleSetSpec(self.handle(), styleNum, spec.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFont(&self, styleNum: c_int, font: @wxFont) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self.handle(), styleNum, font.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFontAttr(&self, styleNum: c_int, size: c_int, faceName: @wxString, bold: bool, italic: bool, underline: bool) {
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
    fn setVScrollBar(&self, bar: @wxScrollBar) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    fn setHScrollBar(&self, bar: @wxScrollBar) {
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
    fn saveFile(&self, filename: @wxString) -> bool {
        unsafe { wxStyledTextCtrl_SaveFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, filename: @wxString) -> bool {
        unsafe { wxStyledTextCtrl_LoadFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> @wxStyledTextCtrl {
        unsafe { @wxStyledTextCtrlImpl(wxStyledTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, style)) as @wxStyledTextCtrl }
    }
    #[fixed_stack_segment]
    fn indicatorGetForeground(&self, indic: c_int) -> @wxColour {
        unsafe { @wxColourImpl(wxStyledTextCtrl_IndicatorGetForeground(self.handle(), indic)) as @wxColour }
    }
    #[fixed_stack_segment]
    fn getCaretLineBackground(&self) -> @wxColour {
        unsafe { @wxColourImpl(wxStyledTextCtrl_GetCaretLineBackground(self.handle())) as @wxColour }
    }
    #[fixed_stack_segment]
    fn setCaretLineBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn getCaretForeground(&self) -> @wxColour {
        unsafe { @wxColourImpl(wxStyledTextCtrl_GetCaretForeground(self.handle())) as @wxColour }
    }
    #[fixed_stack_segment]
    fn getLine(&self, line: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxStyledTextCtrl_GetLine(self.handle(), line)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxStyledTextCtrl_GetText(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getTextRange(&self, startPos: c_int, endPos: c_int) -> @wxString {
        unsafe { @wxStringImpl(wxStyledTextCtrl_GetTextRange(self.handle(), startPos, endPos)) as @wxString }
    }
    #[fixed_stack_segment]
    fn getSelectedText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxStyledTextCtrl_GetSelectedText(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn newDocument(&self) -> @wxSTCDoc {
        unsafe { @wxSTCDocImpl(wxStyledTextCtrl_CreateDocument(self.handle())) as @wxSTCDoc }
    }
    #[fixed_stack_segment]
    fn getEdgeColour(&self) -> @wxColour {
        unsafe { @wxColourImpl(wxStyledTextCtrl_GetEdgeColour(self.handle())) as @wxColour }
    }
    #[fixed_stack_segment]
    fn getDocPointer(&self) -> @wxSTCDoc {
        unsafe { @wxSTCDocImpl(wxStyledTextCtrl_GetDocPointer(self.handle())) as @wxSTCDoc }
    }
    #[fixed_stack_segment]
    fn pointFromPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxStyledTextCtrl_PointFromPosition(self.handle())) as @wxPoint }
    }
}

struct wxSTCDocImpl(*u8);
impl wxSTCDoc for wxSTCDocImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSTCDoc {
    fn handle(&self) -> *u8;
    
}

struct wxMemoryBufferImpl(*u8);
impl wxMemoryBuffer for wxMemoryBufferImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxMemoryBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStyledTextEventImpl(*u8);
impl wxStyledTextEvent for wxStyledTextEventImpl {}
impl wxCommandEvent for wxStyledTextEventImpl {}
impl wxEvent for wxStyledTextEventImpl {}
impl wxObject for wxStyledTextEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxStyledTextEvent : wxCommandEvent {
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
    fn getDragText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxStyledTextEvent_GetDragText(self.handle())) as @wxString }
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
    fn getText(&self) -> @wxString {
        unsafe { @wxStringImpl(wxStyledTextEvent_GetText(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @wxStyledTextEvent {
        unsafe { @wxStyledTextEventImpl(wxStyledTextEvent_Clone(self.handle())) as @wxStyledTextEvent }
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
    fn setText(&self, t: @wxString) {
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
    fn setDragText(&self, val: @wxString) {
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

struct wxGauge95Impl(*u8);
impl wxGauge95 for wxGauge95Impl {}
impl wxGauge for wxGauge95Impl {}
impl wxControl for wxGauge95Impl {}
impl wxWindow for wxGauge95Impl {}
impl wxEvtHandler for wxGauge95Impl {}
impl wxObject for wxGauge95Impl { pub fn handle(&self) -> *u8 { **self } }

trait wxGauge95 : wxGauge {
}

struct wxGaugeMSWImpl(*u8);
impl wxGaugeMSW for wxGaugeMSWImpl {}
impl wxGauge for wxGaugeMSWImpl {}
impl wxControl for wxGaugeMSWImpl {}
impl wxWindow for wxGaugeMSWImpl {}
impl wxEvtHandler for wxGaugeMSWImpl {}
impl wxObject for wxGaugeMSWImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGaugeMSW : wxGauge {
}

struct wxSlider95Impl(*u8);
impl wxSlider95 for wxSlider95Impl {}
impl wxSlider for wxSlider95Impl {}
impl wxControl for wxSlider95Impl {}
impl wxWindow for wxSlider95Impl {}
impl wxEvtHandler for wxSlider95Impl {}
impl wxObject for wxSlider95Impl { pub fn handle(&self) -> *u8 { **self } }

trait wxSlider95 : wxSlider {
}

struct wxSliderMSWImpl(*u8);
impl wxSliderMSW for wxSliderMSWImpl {}
impl wxSlider for wxSliderMSWImpl {}
impl wxControl for wxSliderMSWImpl {}
impl wxWindow for wxSliderMSWImpl {}
impl wxEvtHandler for wxSliderMSWImpl {}
impl wxObject for wxSliderMSWImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxSliderMSW : wxSlider {
}

struct wxcTreeItemDataImpl(*u8);
impl wxcTreeItemData for wxcTreeItemDataImpl {}
impl wxTreeItemData for wxcTreeItemDataImpl {}
impl wxClientData for wxcTreeItemDataImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxcTreeItemData : wxTreeItemData {
    #[fixed_stack_segment]
    fn new(closure: @wxClosure) -> @wxcTreeItemData {
        unsafe { @wxcTreeItemDataImpl(wxcTreeItemData_Create(closure.handle())) as @wxcTreeItemData }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { @wxClosureImpl(wxcTreeItemData_GetClientClosure(self.handle())) as @wxClosure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @wxClosure) {
        unsafe { wxcTreeItemData_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxInputSinkImpl(*u8);
impl wxInputSink for wxInputSinkImpl {}
impl wxThread for wxInputSinkImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxInputSink : wxThread {
    #[fixed_stack_segment]
    fn new(input: @wxInputStream, evtHandler: @wxEvtHandler, bufferLen: c_int) -> @wxInputSink {
        unsafe { @wxInputSinkImpl(wxInputSink_Create(input.handle(), evtHandler.handle(), bufferLen)) as @wxInputSink }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn start(&self) {
        unsafe { wxInputSink_Start(self.handle()) }
    }
}

struct wxInputSinkEventImpl(*u8);
impl wxInputSinkEvent for wxInputSinkEventImpl {}
impl wxEvent for wxInputSinkEventImpl {}
impl wxObject for wxInputSinkEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxInputSinkEvent : wxEvent {
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

struct wxcHtmlEventImpl(*u8);
impl wxcHtmlEvent for wxcHtmlEventImpl {}
impl wxCommandEvent for wxcHtmlEventImpl {}
impl wxEvent for wxcHtmlEventImpl {}
impl wxObject for wxcHtmlEventImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxcHtmlEvent : wxCommandEvent {
    #[fixed_stack_segment]
    fn getMouseEvent(&self) -> @wxMouseEvent {
        unsafe { @wxMouseEventImpl(wxcHtmlEvent_GetMouseEvent(self.handle())) as @wxMouseEvent }
    }
    #[fixed_stack_segment]
    fn getHtmlCell(&self) -> @wxHtmlCell {
        unsafe { @wxHtmlCellImpl(wxcHtmlEvent_GetHtmlCell(self.handle())) as @wxHtmlCell }
    }
    #[fixed_stack_segment]
    fn getHtmlCellId(&self) -> @wxString {
        unsafe { @wxStringImpl(wxcHtmlEvent_GetHtmlCellId(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getHref(&self) -> @wxString {
        unsafe { @wxStringImpl(wxcHtmlEvent_GetHref(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @wxString {
        unsafe { @wxStringImpl(wxcHtmlEvent_GetTarget(self.handle())) as @wxString }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self) -> @wxPoint {
        unsafe { @wxPointImpl(wxcHtmlEvent_GetLogicalPosition(self.handle())) as @wxPoint }
    }
}

struct wxcHtmlWindowImpl(*u8);
impl wxcHtmlWindow for wxcHtmlWindowImpl {}
impl wxHtmlWindow for wxcHtmlWindowImpl {}
impl wxScrolledWindow for wxcHtmlWindowImpl {}
impl wxPanel for wxcHtmlWindowImpl {}
impl wxWindow for wxcHtmlWindowImpl {}
impl wxEvtHandler for wxcHtmlWindowImpl {}
impl wxObject for wxcHtmlWindowImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxcHtmlWindow : wxHtmlWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: @wxString) -> @wxcHtmlWindow {
        unsafe { @wxcHtmlWindowImpl(wxcHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) as @wxcHtmlWindow }
    }
}

struct wxGridCellTextEnterEditorImpl(*u8);
impl wxGridCellTextEnterEditor for wxGridCellTextEnterEditorImpl {}
impl wxGridCellTextEditor for wxGridCellTextEnterEditorImpl {}
impl wxGridCellEditor for wxGridCellTextEnterEditorImpl {}
impl wxGridCellWorker for wxGridCellTextEnterEditorImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxGridCellTextEnterEditor : wxGridCellTextEditor {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellTextEnterEditor {
        unsafe { @wxGridCellTextEnterEditorImpl(wxGridCellTextEnterEditor_Ctor()) as @wxGridCellTextEnterEditor }
    }
}

struct wxFileConfigImpl(*u8);
impl wxFileConfig for wxFileConfigImpl {}
impl wxConfigBase for wxFileConfigImpl { pub fn handle(&self) -> *u8 { **self } }

trait wxFileConfig : wxConfigBase {
    #[fixed_stack_segment]
    fn new(inp: @wxInputStream) -> @wxFileConfig {
        unsafe { @wxFileConfigImpl(wxFileConfig_Create(inp.handle())) as @wxFileConfig }
    }
}

