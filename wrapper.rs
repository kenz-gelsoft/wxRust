use std::libc::*;
use native::*;

trait ELJApp {
    #[fixed_stack_segment]
    fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    fn newLogTarget() -> @ELJLog {
        unsafe { ELJApp_CreateLogTarget() }
    }
    #[fixed_stack_segment]
    fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    fn displaySize() -> @wxSize {
        unsafe { ELJApp_DisplaySize() }
    }
    #[fixed_stack_segment]
    fn enableTooltips(_enable: bool) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    #[fixed_stack_segment]
    fn enableTopLevelWindows(_enb: int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    #[fixed_stack_segment]
    fn executeProcess(_cmd: @wxString, _snc: int, _prc: @wxProcess) -> int {
        unsafe { ELJApp_ExecuteProcess(_cmd, _snc, _prc) }
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
    fn findWindowById(_id: int, _prt: @wxWindow) {
        unsafe { ELJApp_FindWindowById(_id, _prt) }
    }
    #[fixed_stack_segment]
    fn findWindowByLabel(_lbl: @wxString, _prt: @wxWindow) -> @wxWindow {
        unsafe { ELJApp_FindWindowByLabel(_lbl, _prt) }
    }
    #[fixed_stack_segment]
    fn findWindowByName(_lbl: @wxString, _prt: @wxWindow) -> @wxWindow {
        unsafe { ELJApp_FindWindowByName(_lbl, _prt) }
    }
    #[fixed_stack_segment]
    fn getApp() -> @wxApp {
        unsafe { ELJApp_GetApp() }
    }
    #[fixed_stack_segment]
    fn getAppName() -> @wxString {
        unsafe { ELJApp_GetAppName() }
    }
    #[fixed_stack_segment]
    fn getClassName() -> @wxString {
        unsafe { ELJApp_GetClassName() }
    }
    #[fixed_stack_segment]
    fn getExitOnFrameDelete() -> int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    fn getOsDescription() -> @wxString {
        unsafe { ELJApp_GetOsDescription() }
    }
    #[fixed_stack_segment]
    fn getOsVersion(_maj: *c_void, _min: *c_void) -> int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    fn getTopWindow() -> @wxWindow {
        unsafe { ELJApp_GetTopWindow() }
    }
    #[fixed_stack_segment]
    fn getUseBestVisual() -> int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    fn getUserHome(_usr: *c_void) -> @wxString {
        unsafe { ELJApp_GetUserHome(_usr) }
    }
    #[fixed_stack_segment]
    fn getUserId() -> @wxString {
        unsafe { ELJApp_GetUserId() }
    }
    #[fixed_stack_segment]
    fn getUserName() -> @wxString {
        unsafe { ELJApp_GetUserName() }
    }
    #[fixed_stack_segment]
    fn getVendorName() -> @wxString {
        unsafe { ELJApp_GetVendorName() }
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
    fn mainLoop() -> int {
        unsafe { ELJApp_MainLoop() }
    }
    #[fixed_stack_segment]
    fn mousePosition() -> @wxPoint {
        unsafe { ELJApp_MousePosition() }
    }
    #[fixed_stack_segment]
    fn pending() -> int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    fn safeYield(_win: @wxWindow) -> int {
        unsafe { ELJApp_SafeYield(_win) }
    }
    #[fixed_stack_segment]
    fn setAppName(name: @wxString) {
        unsafe { ELJApp_SetAppName(name) }
    }
    #[fixed_stack_segment]
    fn setClassName(name: @wxString) {
        unsafe { ELJApp_SetClassName(name) }
    }
    #[fixed_stack_segment]
    fn setExitOnFrameDelete(flag: int) {
        unsafe { ELJApp_SetExitOnFrameDelete(flag) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(mode: int) {
        unsafe { ELJApp_SetPrintMode(mode) }
    }
    #[fixed_stack_segment]
    fn setTooltipDelay(_ms: int) {
        unsafe { ELJApp_SetTooltipDelay(_ms) }
    }
    #[fixed_stack_segment]
    fn setTopWindow(_wnd: @wxWindow) {
        unsafe { ELJApp_SetTopWindow(_wnd) }
    }
    #[fixed_stack_segment]
    fn setUseBestVisual(flag: int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    #[fixed_stack_segment]
    fn setVendorName(name: @wxString) {
        unsafe { ELJApp_SetVendorName(name) }
    }
    #[fixed_stack_segment]
    fn sleep(_scs: int) {
        unsafe { ELJApp_Sleep(_scs) }
    }
    #[fixed_stack_segment]
    fn milliSleep(_mscs: int) {
        unsafe { ELJApp_MilliSleep(_mscs) }
    }
    #[fixed_stack_segment]
    fn yield_() -> int {
        unsafe { ELJApp_Yield() }
    }
    #[fixed_stack_segment]
    fn isTerminating() -> c_int {
        unsafe { ELJApp_IsTerminating() }
    }
    #[fixed_stack_segment]
    fn initializeC(closure: @wxClosure, _argc: int, _argv: **wchar_t) {
        unsafe { ELJApp_InitializeC(closure, _argc, _argv) }
    }
    #[fixed_stack_segment]
    fn getIdleInterval() -> int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    #[fixed_stack_segment]
    fn setIdleInterval(interval: int) {
        unsafe { ELJApp_SetIdleInterval(interval) }
    }
}
trait ELJArtProv {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _clb: *c_void) -> @ELJArtProv {
        unsafe { ELJArtProv_Create(_obj, _clb) }
    }
    #[fixed_stack_segment]
    fn release(&self) {
        unsafe { ELJArtProv_Release(self) }
    }
}
trait ELJClient {
    #[fixed_stack_segment]
    fn new(_eobj: *c_void, _cnct: *c_void) -> @ELJClient {
        unsafe { ELJClient_Create(_eobj, _cnct) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJClient_Delete(self) }
    }
    #[fixed_stack_segment]
    fn makeConnection(&self, host: @wxString, server: @wxServer, topic: @wxString) {
        unsafe { ELJClient_MakeConnection(self, host, server, topic) }
    }
}
trait ELJCommand {
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { ELJCommand_CanUndo(self) }
    }
    #[fixed_stack_segment]
    fn new(_und: int, _nme: @wxString, _obj: *c_void, _clb: *c_void) -> @ELJCommand {
        unsafe { ELJCommand_Create(_und, _nme, _obj, _clb) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJCommand_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { ELJCommand_GetName(self) }
    }
}
trait ELJConnection {
    #[fixed_stack_segment]
    fn advise(&self, item: @wxString, data: *c_void, size: int, format: int) -> int {
        unsafe { ELJConnection_Advise(self, item, data, size, format) }
    }
    #[fixed_stack_segment]
    fn compress(&self, on: int) {
        unsafe { ELJConnection_Compress(self, on) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *c_void, buffer: *c_void, size: int) -> @ELJConnection {
        unsafe { ELJConnection_Create(_obj, buffer, size) }
    }
    #[fixed_stack_segment]
    fn newDefault(&self) -> @ELJConnection {
        unsafe { ELJConnection_CreateDefault(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJConnection_Delete(self) }
    }
    #[fixed_stack_segment]
    fn disconnect(&self) -> bool {
        unsafe { ELJConnection_Disconnect(self) }
    }
    #[fixed_stack_segment]
    fn execute(&self, data: @wxString, size: int, format: int) -> bool {
        unsafe { ELJConnection_Execute(self, data, size, format) }
    }
    #[fixed_stack_segment]
    fn poke(&self, item: @wxString, data: *c_void, size: int, format: int) -> bool {
        unsafe { ELJConnection_Poke(self, item, data, size, format) }
    }
    #[fixed_stack_segment]
    fn request(&self, item: @wxString, size: @wxSize, format: int) {
        unsafe { ELJConnection_Request(self, item, size, format) }
    }
    #[fixed_stack_segment]
    fn setOnAdvise(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnAdvise(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnDisconnect(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnDisconnect(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnExecute(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnExecute(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnPoke(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnPoke(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnRequest(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnRequest(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnStartAdvise(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnStartAdvise(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn setOnStopAdvise(&self, _fnc: *c_void) {
        unsafe { ELJConnection_SetOnStopAdvise(self, _fnc) }
    }
    #[fixed_stack_segment]
    fn startAdvise(&self, item: @wxString) -> bool {
        unsafe { ELJConnection_StartAdvise(self, item) }
    }
    #[fixed_stack_segment]
    fn stopAdvise(&self, item: @wxString) -> bool {
        unsafe { ELJConnection_StopAdvise(self, item) }
    }
}
trait ELJDragDataObject {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _fmt: @wxString, _func1: *c_void, _func2: *c_void, _func3: *c_void) -> @ELJDragDataObject {
        unsafe { ELJDragDataObject_Create(_obj, _fmt, _func1, _func2, _func3) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self) }
    }
}
trait ELJDropTarget {
    #[fixed_stack_segment]
    fn new(_obj: *c_void) -> @ELJDropTarget {
        unsafe { ELJDropTarget_Create(_obj) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJDropTarget_Delete(self) }
    }
    #[fixed_stack_segment]
    fn setOnData(&self, _func: *c_void) {
        unsafe { ELJDropTarget_SetOnData(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnDragOver(&self, _func: *c_void) {
        unsafe { ELJDropTarget_SetOnDragOver(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnDrop(&self, _func: *c_void) {
        unsafe { ELJDropTarget_SetOnDrop(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnEnter(&self, _func: *c_void) {
        unsafe { ELJDropTarget_SetOnEnter(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnLeave(&self, _func: *c_void) {
        unsafe { ELJDropTarget_SetOnLeave(self, _func) }
    }
}
trait ELJFileDropTarget {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _func: *c_void) -> @ELJFileDropTarget {
        unsafe { ELJFileDropTarget_Create(_obj, _func) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJFileDropTarget_Delete(self) }
    }
    #[fixed_stack_segment]
    fn setOnData(&self, _func: *c_void) {
        unsafe { ELJFileDropTarget_SetOnData(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnDragOver(&self, _func: *c_void) {
        unsafe { ELJFileDropTarget_SetOnDragOver(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnDrop(&self, _func: *c_void) {
        unsafe { ELJFileDropTarget_SetOnDrop(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnEnter(&self, _func: *c_void) {
        unsafe { ELJFileDropTarget_SetOnEnter(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnLeave(&self, _func: *c_void) {
        unsafe { ELJFileDropTarget_SetOnLeave(self, _func) }
    }
}
trait ELJGridTable {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _EifGetNumberRows: *c_void, _EifGetNumberCols: *c_void, _EifGetValue: *c_void, _EifSetValue: *c_void, _EifIsEmptyCell: *c_void, _EifClear: *c_void, _EifInsertRows: *c_void, _EifAppendRows: *c_void, _EifDeleteRows: *c_void, _EifInsertCols: *c_void, _EifAppendCols: *c_void, _EifDeleteCols: *c_void, _EifSetRowLabelValue: *c_void, _EifSetColLabelValue: *c_void, _EifGetRowLabelValue: *c_void, _EifGetColLabelValue: *c_void) -> @ELJGridTable {
        unsafe { ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJGridTable_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getView(&self) -> @wxView {
        unsafe { ELJGridTable_GetView(self) }
    }
    #[fixed_stack_segment]
    fn sendTableMessage(&self, id: int, val1: int, val2: int) {
        unsafe { ELJGridTable_SendTableMessage(self, id, val1, val2) }
    }
}
trait ELJLocale {
}
trait ELJLog {
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: *wchar_t) {
        unsafe { ELJLog_AddTraceMask(self, str) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _fnc: *c_void) -> @ELJLog {
        unsafe { ELJLog_Create(_obj, _fnc) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJLog_Delete(self) }
    }
    #[fixed_stack_segment]
    fn dontCreateOnDemand(&self) {
        unsafe { ELJLog_DontCreateOnDemand(self) }
    }
    #[fixed_stack_segment]
    fn enableLogging(&self, doIt: bool) -> int {
        unsafe { ELJLog_EnableLogging(self, doIt) }
    }
    #[fixed_stack_segment]
    fn flush(&self) {
        unsafe { ELJLog_Flush(self) }
    }
    #[fixed_stack_segment]
    fn flushActive(&self) {
        unsafe { ELJLog_FlushActive(self) }
    }
    #[fixed_stack_segment]
    fn getActiveTarget() {
        unsafe { ELJLog_GetActiveTarget() }
    }
    #[fixed_stack_segment]
    fn getTimestamp(&self) {
        unsafe { ELJLog_GetTimestamp(self) }
    }
    #[fixed_stack_segment]
    fn getTraceMask(&self) -> int {
        unsafe { ELJLog_GetTraceMask(self) }
    }
    #[fixed_stack_segment]
    fn getVerbose(&self) -> int {
        unsafe { ELJLog_GetVerbose(self) }
    }
    #[fixed_stack_segment]
    fn hasPendingMessages(&self) -> bool {
        unsafe { ELJLog_HasPendingMessages(self) }
    }
    #[fixed_stack_segment]
    fn isAllowedTraceMask(&self, mask: @wxMask) -> bool {
        unsafe { ELJLog_IsAllowedTraceMask(self, mask) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self) -> bool {
        unsafe { ELJLog_IsEnabled(self) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: int, szString: *c_void, t: int) {
        unsafe { ELJLog_OnLog(self, level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask(&self, str: *wchar_t) {
        unsafe { ELJLog_RemoveTraceMask(self, str) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { ELJLog_Resume(self) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) {
        unsafe { ELJLog_SetActiveTarget(self) }
    }
    #[fixed_stack_segment]
    fn setTimestamp(&self, ts: *c_void) {
        unsafe { ELJLog_SetTimestamp(self, ts) }
    }
    #[fixed_stack_segment]
    fn setTraceMask(&self, ulMask: int) {
        unsafe { ELJLog_SetTraceMask(self, ulMask) }
    }
    #[fixed_stack_segment]
    fn setVerbose(&self, bVerbose: int) {
        unsafe { ELJLog_SetVerbose(self, bVerbose) }
    }
    #[fixed_stack_segment]
    fn suspend(&self) {
        unsafe { ELJLog_Suspend(self) }
    }
}
trait ELJMessageParameters {
}
trait ELJPlotCurve {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _str: *c_void, _end: *c_void, _y: *c_void, offsetY: int, startY: c_double, endY: c_double) -> @ELJPlotCurve {
        unsafe { ELJPlotCurve_Create(_obj, _str, _end, _y, offsetY, startY, endY) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJPlotCurve_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getEndY(&self) -> c_double {
        unsafe { ELJPlotCurve_GetEndY(self) }
    }
    #[fixed_stack_segment]
    fn getOffsetY(&self) -> int {
        unsafe { ELJPlotCurve_GetOffsetY(self) }
    }
    #[fixed_stack_segment]
    fn getStartY(&self) -> c_double {
        unsafe { ELJPlotCurve_GetStartY(self) }
    }
    #[fixed_stack_segment]
    fn setEndY(&self, endY: c_double) {
        unsafe { ELJPlotCurve_SetEndY(self, endY) }
    }
    #[fixed_stack_segment]
    fn setOffsetY(&self, offsetY: int) {
        unsafe { ELJPlotCurve_SetOffsetY(self, offsetY) }
    }
    #[fixed_stack_segment]
    fn setPenNormal(&self, pen: @wxPen) {
        unsafe { ELJPlotCurve_SetPenNormal(self, pen) }
    }
    #[fixed_stack_segment]
    fn setPenSelected(&self, pen: @wxPen) {
        unsafe { ELJPlotCurve_SetPenSelected(self, pen) }
    }
    #[fixed_stack_segment]
    fn setStartY(&self, startY: c_double) {
        unsafe { ELJPlotCurve_SetStartY(self, startY) }
    }
}
trait ELJPreviewControlBar {
    #[fixed_stack_segment]
    fn new(preview: *c_void, buttons: int, parent: @wxWindow, title: *c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @ELJPreviewControlBar {
        unsafe { ELJPreviewControlBar_Create(preview, buttons, parent, title, _arg4, style) }
    }
}
trait ELJPreviewFrame {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _init: *c_void, _create_canvas: *c_void, _create_toolbar: *c_void, preview: *c_void, parent: @wxWindow, title: *c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @ELJPreviewFrame {
        unsafe { ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent, title, _arg7, style) }
    }
    #[fixed_stack_segment]
    fn getControlBar(&self) {
        unsafe { ELJPreviewFrame_GetControlBar(self) }
    }
    #[fixed_stack_segment]
    fn getPreviewCanvas(&self) -> @wxPreviewCanvas {
        unsafe { ELJPreviewFrame_GetPreviewCanvas(self) }
    }
    #[fixed_stack_segment]
    fn getPrintPreview(&self) -> @wxPrintPreview {
        unsafe { ELJPreviewFrame_GetPrintPreview(self) }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { ELJPreviewFrame_Initialize(self) }
    }
    #[fixed_stack_segment]
    fn setControlBar(&self, obj: *c_void) {
        unsafe { ELJPreviewFrame_SetControlBar(self, obj) }
    }
    #[fixed_stack_segment]
    fn setPreviewCanvas(&self, obj: @wxPreviewCanvas) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self, obj) }
    }
    #[fixed_stack_segment]
    fn setPrintPreview(&self, obj: @wxPrintPreview) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self, obj) }
    }
}
trait ELJServer {
    #[fixed_stack_segment]
    fn new(_eobj: *c_void, _cnct: *c_void) -> @ELJServer {
        unsafe { ELJServer_Create(_eobj, _cnct) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJServer_Delete(self) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, name: @wxString) -> int {
        unsafe { ELJServer_Initialize(self, name) }
    }
}
trait ELJTextDropTarget {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _func: *c_void) -> @ELJTextDropTarget {
        unsafe { ELJTextDropTarget_Create(_obj, _func) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJTextDropTarget_Delete(self) }
    }
    #[fixed_stack_segment]
    fn setOnData(&self, _func: *c_void) {
        unsafe { ELJTextDropTarget_SetOnData(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnDragOver(&self, _func: *c_void) {
        unsafe { ELJTextDropTarget_SetOnDragOver(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnDrop(&self, _func: *c_void) {
        unsafe { ELJTextDropTarget_SetOnDrop(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnEnter(&self, _func: *c_void) {
        unsafe { ELJTextDropTarget_SetOnEnter(self, _func) }
    }
    #[fixed_stack_segment]
    fn setOnLeave(&self, _func: *c_void) {
        unsafe { ELJTextDropTarget_SetOnLeave(self, _func) }
    }
}
trait ELJTextValidator {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _fnc: *c_void, _txt: *wchar_t, _stl: int) -> @ELJTextValidator {
        unsafe { ELJTextValidator_Create(_obj, _fnc, _txt, _stl) }
    }
}
trait cbAntiflickerPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbAntiflickerPlugin {
        unsafe { cbAntiflickerPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbAntiflickerPlugin {
        unsafe { cbAntiflickerPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbAntiflickerPlugin_Delete(self) }
    }
}
trait cbBarDragPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbBarDragPlugin {
        unsafe { cbBarDragPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbBarDragPlugin {
        unsafe { cbBarDragPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarDragPlugin_Delete(self) }
    }
}
trait cbBarHintsPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbBarHintsPlugin {
        unsafe { cbBarHintsPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbBarHintsPlugin {
        unsafe { cbBarHintsPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarHintsPlugin_Delete(self) }
    }
    #[fixed_stack_segment]
    fn setGrooveCount(&self, nGrooves: int) {
        unsafe { cbBarHintsPlugin_SetGrooveCount(self, nGrooves) }
    }
}
trait cbBarInfo {
    #[fixed_stack_segment]
    fn new() -> @cbBarInfo {
        unsafe { cbBarInfo_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarInfo_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isExpanded(&self) -> bool {
        unsafe { cbBarInfo_IsExpanded(self) }
    }
    #[fixed_stack_segment]
    fn isFixed(&self) -> bool {
        unsafe { cbBarInfo_IsFixed(self) }
    }
}
trait cbBarSpy {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void) -> @cbBarSpy {
        unsafe { cbBarSpy_Create(pPanel) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbBarSpy {
        unsafe { cbBarSpy_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbBarSpy_Delete(self) }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @wxEvent) -> int {
        unsafe { cbBarSpy_ProcessEvent(self, event) }
    }
    #[fixed_stack_segment]
    fn setBarWindow(&self, pWnd: *c_void) {
        unsafe { cbBarSpy_SetBarWindow(self, pWnd) }
    }
}
trait cbCloseBox {
    #[fixed_stack_segment]
    fn new() -> @cbCloseBox {
        unsafe { cbCloseBox_Create() }
    }
}
trait cbCollapseBox {
    #[fixed_stack_segment]
    fn new() -> @cbCollapseBox {
        unsafe { cbCollapseBox_Create() }
    }
}
trait cbCommonPaneProperties {
    #[fixed_stack_segment]
    fn assign(&self, _other: *c_void) {
        unsafe { cbCommonPaneProperties_Assign(self, _other) }
    }
    #[fixed_stack_segment]
    fn barCollapseIconsOn(&self) -> int {
        unsafe { cbCommonPaneProperties_BarCollapseIconsOn(self) }
    }
    #[fixed_stack_segment]
    fn barDragHintsOn(&self) -> int {
        unsafe { cbCommonPaneProperties_BarDragHintsOn(self) }
    }
    #[fixed_stack_segment]
    fn barFloatingOn(&self) -> int {
        unsafe { cbCommonPaneProperties_BarFloatingOn(self) }
    }
    #[fixed_stack_segment]
    fn colProportionsOn(&self) -> int {
        unsafe { cbCommonPaneProperties_ColProportionsOn(self) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbCommonPaneProperties {
        unsafe { cbCommonPaneProperties_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbCommonPaneProperties_Delete(self) }
    }
    #[fixed_stack_segment]
    fn exactDockPredictionOn(&self) -> int {
        unsafe { cbCommonPaneProperties_ExactDockPredictionOn(self) }
    }
    #[fixed_stack_segment]
    fn minCBarDim(&self, _w: *c_int, _h: *c_int) {
        unsafe { cbCommonPaneProperties_MinCBarDim(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn nonDestructFrictionOn(&self) -> int {
        unsafe { cbCommonPaneProperties_NonDestructFrictionOn(self) }
    }
    #[fixed_stack_segment]
    fn outOfPaneDragOn(&self) -> int {
        unsafe { cbCommonPaneProperties_OutOfPaneDragOn(self) }
    }
    #[fixed_stack_segment]
    fn realTimeUpdatesOn(&self) -> int {
        unsafe { cbCommonPaneProperties_RealTimeUpdatesOn(self) }
    }
    #[fixed_stack_segment]
    fn resizeHandleSize(&self) -> int {
        unsafe { cbCommonPaneProperties_ResizeHandleSize(self) }
    }
    #[fixed_stack_segment]
    fn rowProportionsOn(&self) -> int {
        unsafe { cbCommonPaneProperties_RowProportionsOn(self) }
    }
    #[fixed_stack_segment]
    fn setBarCollapseIconsOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetBarCollapseIconsOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setBarDragHintsOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetBarDragHintsOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setBarFloatingOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetBarFloatingOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setColProportionsOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetColProportionsOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setExactDockPredictionOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetExactDockPredictionOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setMinCBarDim(&self, _w: c_int, _h: c_int) {
        unsafe { cbCommonPaneProperties_SetMinCBarDim(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setNonDestructFrictionOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetNonDestructFrictionOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setOutOfPaneDragOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetOutOfPaneDragOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setRealTimeUpdatesOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetRealTimeUpdatesOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setResizeHandleSize(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetResizeHandleSize(self, _val) }
    }
    #[fixed_stack_segment]
    fn setRowProportionsOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetRowProportionsOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn setShow3DPaneBorderOn(&self, _val: int) {
        unsafe { cbCommonPaneProperties_SetShow3DPaneBorderOn(self, _val) }
    }
    #[fixed_stack_segment]
    fn show3DPaneBorderOn(&self) -> int {
        unsafe { cbCommonPaneProperties_Show3DPaneBorderOn(self) }
    }
}
trait cbCustomizeBarEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbCustomizeBarEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn clickPos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbCustomizeBarEvent_ClickPos(self, _arg1) }
    }
}
trait cbCustomizeLayoutEvent {
    #[fixed_stack_segment]
    fn clickPos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbCustomizeLayoutEvent_ClickPos(self, _arg1) }
    }
}
trait cbDimHandlerBase {
}
trait cbDimInfo {
    #[fixed_stack_segment]
    fn assign(&self, other: *c_void) {
        unsafe { cbDimInfo_Assign(self, other) }
    }
    #[fixed_stack_segment]
    fn new(x: c_int, y: c_int, isFixed: bool, gap: int, pDimHandler: *c_void) -> @cbDimInfo {
        unsafe { cbDimInfo_Create(_arg0, isFixed, gap, pDimHandler) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbDimInfo {
        unsafe { cbDimInfo_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newWithHandler(&self, isFixed: bool) {
        unsafe { cbDimInfo_CreateWithHandler(self, isFixed) }
    }
    #[fixed_stack_segment]
    fn newWithInfo(dh_x: int, dh_y: int, dv_x: int, dv_y: int, f_x: int, f_y: int, isFixed: bool, horizGap: int, vertGap: int, pDimHandler: *c_void) {
        unsafe { cbDimInfo_CreateWithInfo(dh_x, dh_y, dv_x, dv_y, f_x, f_y, isFixed, horizGap, vertGap, pDimHandler) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbDimInfo_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getDimHandler(&self) {
        unsafe { cbDimInfo_GetDimHandler(self) }
    }
}
trait cbDockBox {
    #[fixed_stack_segment]
    fn new() -> @cbDockBox {
        unsafe { cbDockBox_Create() }
    }
}
trait cbDockPane {
    #[fixed_stack_segment]
    fn barPresent(&self, pBar: *c_void) -> int {
        unsafe { cbDockPane_BarPresent(self, pBar) }
    }
    #[fixed_stack_segment]
    fn new(alignment: int, pPanel: *c_void) -> @cbDockPane {
        unsafe { cbDockPane_Create(alignment, pPanel) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbDockPane {
        unsafe { cbDockPane_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbDockPane_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> int {
        unsafe { cbDockPane_GetAlignment(self) }
    }
    #[fixed_stack_segment]
    fn getBarInfoByWindow(&self, pBarWnd: *c_void) {
        unsafe { cbDockPane_GetBarInfoByWindow(self, pBarWnd) }
    }
    #[fixed_stack_segment]
    fn getBarResizeRange(&self, pBar: *c_void, from: *c_void, till: *c_void, forLeftHandle: int) {
        unsafe { cbDockPane_GetBarResizeRange(self, pBar, from, till, forLeftHandle) }
    }
    #[fixed_stack_segment]
    fn getDockingState(&self) -> int {
        unsafe { cbDockPane_GetDockingState(self) }
    }
    #[fixed_stack_segment]
    fn getFirstRow(&self) {
        unsafe { cbDockPane_GetFirstRow(self) }
    }
    #[fixed_stack_segment]
    fn getPaneHeight(&self) -> int {
        unsafe { cbDockPane_GetPaneHeight(self) }
    }
    #[fixed_stack_segment]
    fn getRealRect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbDockPane_GetRealRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getRowList(&self, _ref: *c_void) -> int {
        unsafe { cbDockPane_GetRowList(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getRowResizeRange(&self, pRow: *c_void, from: *c_void, till: *c_void, forUpperHandle: int) {
        unsafe { cbDockPane_GetRowResizeRange(self, pRow, from, till, forUpperHandle) }
    }
    #[fixed_stack_segment]
    fn hitTestPaneItems(&self, x: c_int, y: c_int, ppRow: *c_void, ppBar: *c_void) -> int {
        unsafe { cbDockPane_HitTestPaneItems(self, _arg1, ppRow, ppBar) }
    }
    #[fixed_stack_segment]
    fn insertBarByCoord(&self, pBar: *c_void, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { cbDockPane_InsertBarByCoord(self, pBar, _arg2) }
    }
    #[fixed_stack_segment]
    fn insertBarByInfo(&self, pBarInfo: *c_void) {
        unsafe { cbDockPane_InsertBarByInfo(self, pBarInfo) }
    }
    #[fixed_stack_segment]
    fn insertBarToRow(&self, pBar: *c_void, pIntoRow: *c_void) {
        unsafe { cbDockPane_InsertBarToRow(self, pBar, pIntoRow) }
    }
    #[fixed_stack_segment]
    fn insertRow(&self, pRow: *c_void, pBeforeRow: *c_void) {
        unsafe { cbDockPane_InsertRow(self, pRow, pBeforeRow) }
    }
    #[fixed_stack_segment]
    fn isHorizontal(&self) -> bool {
        unsafe { cbDockPane_IsHorizontal(self) }
    }
    #[fixed_stack_segment]
    fn matchesMask(&self, paneMask: int) -> int {
        unsafe { cbDockPane_MatchesMask(self, paneMask) }
    }
    #[fixed_stack_segment]
    fn removeBar(&self, pBar: *c_void) {
        unsafe { cbDockPane_RemoveBar(self, pBar) }
    }
    #[fixed_stack_segment]
    fn removeRow(&self, pRow: *c_void) {
        unsafe { cbDockPane_RemoveRow(self, pRow) }
    }
    #[fixed_stack_segment]
    fn setBoundsInParent(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { cbDockPane_SetBoundsInParent(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, top: int, bottom: int, left: int, right: int) {
        unsafe { cbDockPane_SetMargins(self, top, bottom, left, right) }
    }
    #[fixed_stack_segment]
    fn setPaneWidth(&self, width: int) {
        unsafe { cbDockPane_SetPaneWidth(self, width) }
    }
}
trait cbDrawBarDecorEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbDrawBarDecorEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn boundsInParent(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbDrawBarDecorEvent_BoundsInParent(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawBarDecorEvent_Dc(self) }
    }
}
trait cbDrawBarHandlesEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbDrawBarHandlesEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawBarHandlesEvent_Dc(self) }
    }
}
trait cbDrawHintRectEvent {
    #[fixed_stack_segment]
    fn eraseRect(&self) -> int {
        unsafe { cbDrawHintRectEvent_EraseRect(self) }
    }
    #[fixed_stack_segment]
    fn isInClient(&self) -> bool {
        unsafe { cbDrawHintRectEvent_IsInClient(self) }
    }
    #[fixed_stack_segment]
    fn lastTime(&self) -> int {
        unsafe { cbDrawHintRectEvent_LastTime(self) }
    }
    #[fixed_stack_segment]
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbDrawHintRectEvent_Rect(self, _arg1) }
    }
}
trait cbDrawPaneBkGroundEvent {
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawPaneBkGroundEvent_Dc(self) }
    }
}
trait cbDrawPaneDecorEvent {
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawPaneDecorEvent_Dc(self) }
    }
}
trait cbDrawRowBkGroundEvent {
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawRowBkGroundEvent_Dc(self) }
    }
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbDrawRowBkGroundEvent_Row(self) }
    }
}
trait cbDrawRowDecorEvent {
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawRowDecorEvent_Dc(self) }
    }
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbDrawRowDecorEvent_Row(self) }
    }
}
trait cbDrawRowHandlesEvent {
    #[fixed_stack_segment]
    fn dc(&self) {
        unsafe { cbDrawRowHandlesEvent_Dc(self) }
    }
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbDrawRowHandlesEvent_Row(self) }
    }
}
trait cbDynToolBarDimHandler {
    #[fixed_stack_segment]
    fn new() -> @cbDynToolBarDimHandler {
        unsafe { cbDynToolBarDimHandler_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbDynToolBarDimHandler_Delete(self) }
    }
}
trait cbFinishDrawInAreaEvent {
    #[fixed_stack_segment]
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbFinishDrawInAreaEvent_Area(self, _arg1) }
    }
}
trait cbFloatedBarWindow {
    #[fixed_stack_segment]
    fn new(_obj: *c_void) -> @cbFloatedBarWindow {
        unsafe { cbFloatedBarWindow_Create(_obj) }
    }
    #[fixed_stack_segment]
    fn getBar(&self) {
        unsafe { cbFloatedBarWindow_GetBar(self) }
    }
    #[fixed_stack_segment]
    fn positionFloatedWnd(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { cbFloatedBarWindow_PositionFloatedWnd(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setBar(&self, _bar: *c_void) {
        unsafe { cbFloatedBarWindow_SetBar(self, _bar) }
    }
    #[fixed_stack_segment]
    fn setLayout(&self, _layout: *c_void) {
        unsafe { cbFloatedBarWindow_SetLayout(self, _layout) }
    }
}
trait cbGCUpdatesMgr {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void) -> @cbGCUpdatesMgr {
        unsafe { cbGCUpdatesMgr_Create(pPanel) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbGCUpdatesMgr {
        unsafe { cbGCUpdatesMgr_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbGCUpdatesMgr_Delete(self) }
    }
    #[fixed_stack_segment]
    fn updateNow(&self) {
        unsafe { cbGCUpdatesMgr_UpdateNow(self) }
    }
}
trait cbHintAnimationPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbHintAnimationPlugin {
        unsafe { cbHintAnimationPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbHintAnimationPlugin {
        unsafe { cbHintAnimationPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbHintAnimationPlugin_Delete(self) }
    }
}
trait cbInsertBarEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbInsertBarEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbInsertBarEvent_Row(self) }
    }
}
trait cbLayoutRowEvent {
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbLayoutRowEvent_Row(self) }
    }
}
trait cbLeftDClickEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftDClickEvent_Pos(self, _arg1) }
    }
}
trait cbLeftDownEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftDownEvent_Pos(self, _arg1) }
    }
}
trait cbLeftUpEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftUpEvent_Pos(self, _arg1) }
    }
}
trait cbMiniButton {
    #[fixed_stack_segment]
    fn new() -> @cbMiniButton {
        unsafe { cbMiniButton_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbMiniButton_Delete(self) }
    }
    #[fixed_stack_segment]
    fn dim(&self, _w: *c_int, _h: *c_int) {
        unsafe { cbMiniButton_Dim(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn dragStarted(&self) -> int {
        unsafe { cbMiniButton_DragStarted(self) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) {
        unsafe { cbMiniButton_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn enabled(&self) -> int {
        unsafe { cbMiniButton_Enabled(self) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int) -> int {
        unsafe { cbMiniButton_HitTest(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn isPressed(&self) -> bool {
        unsafe { cbMiniButton_IsPressed(self) }
    }
    #[fixed_stack_segment]
    fn layout(&self) {
        unsafe { cbMiniButton_Layout(self) }
    }
    #[fixed_stack_segment]
    fn pane(&self) {
        unsafe { cbMiniButton_Pane(self) }
    }
    #[fixed_stack_segment]
    fn plugin(&self) {
        unsafe { cbMiniButton_Plugin(self) }
    }
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbMiniButton_Pos(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn pressed(&self) -> int {
        unsafe { cbMiniButton_Pressed(self) }
    }
    #[fixed_stack_segment]
    fn refresh(&self) {
        unsafe { cbMiniButton_Refresh(self) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { cbMiniButton_Reset(self) }
    }
    #[fixed_stack_segment]
    fn setPos(&self, x: c_int, y: c_int) {
        unsafe { cbMiniButton_SetPos(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn visible(&self) -> int {
        unsafe { cbMiniButton_Visible(self) }
    }
    #[fixed_stack_segment]
    fn wasClicked(&self) -> int {
        unsafe { cbMiniButton_WasClicked(self) }
    }
    #[fixed_stack_segment]
    fn wnd(&self) {
        unsafe { cbMiniButton_Wnd(self) }
    }
}
trait cbMotionEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbMotionEvent_Pos(self, _arg1) }
    }
}
trait cbPaneDrawPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbPaneDrawPlugin {
        unsafe { cbPaneDrawPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbPaneDrawPlugin {
        unsafe { cbPaneDrawPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbPaneDrawPlugin_Delete(self) }
    }
}
trait cbPluginBase {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbPluginBase_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getPaneMask(&self) -> int {
        unsafe { cbPluginBase_GetPaneMask(self) }
    }
    #[fixed_stack_segment]
    fn isReady(&self) -> bool {
        unsafe { cbPluginBase_IsReady(self) }
    }
    #[fixed_stack_segment]
    fn plugin(_swt: int) {
        unsafe { cbPluginBase_Plugin(_swt) }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @wxEvent) -> int {
        unsafe { cbPluginBase_ProcessEvent(self, event) }
    }
}
trait cbPluginEvent {
    #[fixed_stack_segment]
    fn pane(&self) {
        unsafe { cbPluginEvent_Pane(self) }
    }
}
trait cbRemoveBarEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbRemoveBarEvent_Bar(self) }
    }
}
trait cbResizeBarEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbResizeBarEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbResizeBarEvent_Row(self) }
    }
}
trait cbResizeRowEvent {
    #[fixed_stack_segment]
    fn forUpperHandle(&self) -> int {
        unsafe { cbResizeRowEvent_ForUpperHandle(self) }
    }
    #[fixed_stack_segment]
    fn handleOfs(&self) -> int {
        unsafe { cbResizeRowEvent_HandleOfs(self) }
    }
    #[fixed_stack_segment]
    fn row(&self) {
        unsafe { cbResizeRowEvent_Row(self) }
    }
}
trait cbRightDownEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbRightDownEvent_Pos(self, _arg1) }
    }
}
trait cbRightUpEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbRightUpEvent_Pos(self, _arg1) }
    }
}
trait cbRowDragPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbRowDragPlugin {
        unsafe { cbRowDragPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbRowDragPlugin {
        unsafe { cbRowDragPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbRowDragPlugin_Delete(self) }
    }
}
trait cbRowInfo {
    #[fixed_stack_segment]
    fn new() -> @cbRowInfo {
        unsafe { cbRowInfo_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbRowInfo_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getFirstBar(&self) {
        unsafe { cbRowInfo_GetFirstBar(self) }
    }
}
trait cbRowLayoutPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbRowLayoutPlugin {
        unsafe { cbRowLayoutPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbRowLayoutPlugin {
        unsafe { cbRowLayoutPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbRowLayoutPlugin_Delete(self) }
    }
}
trait cbSimpleCustomizationPlugin {
    #[fixed_stack_segment]
    fn new(pPanel: *c_void, paneMask: int) -> @cbSimpleCustomizationPlugin {
        unsafe { cbSimpleCustomizationPlugin_Create(pPanel, paneMask) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @cbSimpleCustomizationPlugin {
        unsafe { cbSimpleCustomizationPlugin_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { cbSimpleCustomizationPlugin_Delete(self) }
    }
}
trait cbSimpleUpdatesMgr {
}
trait cbSizeBarWndEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbSizeBarWndEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn boundsInParent(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbSizeBarWndEvent_BoundsInParent(self, _arg1) }
    }
}
trait cbStartBarDraggingEvent {
    #[fixed_stack_segment]
    fn bar(&self) {
        unsafe { cbStartBarDraggingEvent_Bar(self) }
    }
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbStartBarDraggingEvent_Pos(self, _arg1) }
    }
}
trait cbStartDrawInAreaEvent {
    #[fixed_stack_segment]
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbStartDrawInAreaEvent_Area(self, _arg1) }
    }
}
trait cbUpdatesManagerBase {
}
trait wxAcceleratorEntry {
    #[fixed_stack_segment]
    fn new(flags: int, keyCode: int, cmd: int) -> @wxAcceleratorEntry {
        unsafe { wxAcceleratorEntry_Create(flags, keyCode, cmd) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAcceleratorEntry_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCommand(&self) -> int {
        unsafe { wxAcceleratorEntry_GetCommand(self) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> int {
        unsafe { wxAcceleratorEntry_GetFlags(self) }
    }
    #[fixed_stack_segment]
    fn getKeyCode(&self) -> int {
        unsafe { wxAcceleratorEntry_GetKeyCode(self) }
    }
    #[fixed_stack_segment]
    fn set(&self, flags: int, keyCode: int, cmd: int) {
        unsafe { wxAcceleratorEntry_Set(self, flags, keyCode, cmd) }
    }
}
trait wxAcceleratorTable {
    #[fixed_stack_segment]
    fn new(n: int, entries: *c_void) -> @wxAcceleratorTable {
        unsafe { wxAcceleratorTable_Create(n, entries) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self) }
    }
}
trait wxActivateEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxActivateEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getActive(&self) -> bool {
        unsafe { wxActivateEvent_GetActive(self) }
    }
}
trait wxApp {
}
trait wxArray {
}
trait wxArrayString {
}
trait wxArtProvider {
}
trait wxAutoBufferedPaintDC {
    #[fixed_stack_segment]
    fn new(window: @wxWindow) -> @wxAutoBufferedPaintDC {
        unsafe { wxAutoBufferedPaintDC_Create(window) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAutoBufferedPaintDC_Delete(self) }
    }
}
trait wxAutomationObject {
}
trait wxBitmap {
    #[fixed_stack_segment]
    fn addHandler(handler: @wxEvtHandler) {
        unsafe { wxBitmap_AddHandler(handler) }
    }
    #[fixed_stack_segment]
    fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    fn new(_data: *c_void, _type: int, _width: c_int, _height: c_int, _depth: int) -> @wxBitmap {
        unsafe { wxBitmap_Create(_data, _type, _arg2, _depth) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxBitmap {
        unsafe { wxBitmap_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newEmpty(_width: c_int, _height: c_int, _depth: int) -> @wxBitmap {
        unsafe { wxBitmap_CreateEmpty(_arg0, _depth) }
    }
    #[fixed_stack_segment]
    fn newFromXPM(&self) -> @wxBitmap {
        unsafe { wxBitmap_CreateFromXPM(self) }
    }
    #[fixed_stack_segment]
    fn newLoad(name: @wxString, type_: int) -> @wxBitmap {
        unsafe { wxBitmap_CreateLoad(name, type_) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBitmap_Delete(self) }
    }
    #[fixed_stack_segment]
    fn findHandlerByExtension(&self, type_: int) {
        unsafe { wxBitmap_FindHandlerByExtension(self, type_) }
    }
    #[fixed_stack_segment]
    fn findHandlerByName(name: @wxString) {
        unsafe { wxBitmap_FindHandlerByName(name) }
    }
    #[fixed_stack_segment]
    fn findHandlerByType(type_: int) {
        unsafe { wxBitmap_FindHandlerByType(type_) }
    }
    #[fixed_stack_segment]
    fn getDepth(&self) -> int {
        unsafe { wxBitmap_GetDepth(self) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> int {
        unsafe { wxBitmap_GetHeight(self) }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> @wxMask {
        unsafe { wxBitmap_GetMask(self) }
    }
    #[fixed_stack_segment]
    fn getSubBitmap(&self, x: c_int, y: c_int, w: c_int, h: c_int, @wxBitmap) {
        unsafe { wxBitmap_GetSubBitmap(self, _arg1, _ref) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxBitmap_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn initStandardHandlers() {
        unsafe { wxBitmap_InitStandardHandlers() }
    }
    #[fixed_stack_segment]
    fn insertHandler(handler: @wxEvtHandler) {
        unsafe { wxBitmap_InsertHandler(handler) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, name: @wxString, type_: int) -> int {
        unsafe { wxBitmap_LoadFile(self, name, type_) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBitmap_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn removeHandler(name: @wxString) -> bool {
        unsafe { wxBitmap_RemoveHandler(name) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @wxString, type_: int, cmap: @wxPalette) -> int {
        unsafe { wxBitmap_SaveFile(self, name, type_, cmap) }
    }
    #[fixed_stack_segment]
    fn setDepth(&self, d: int) {
        unsafe { wxBitmap_SetDepth(self, d) }
    }
    #[fixed_stack_segment]
    fn setHeight(&self, h: int) {
        unsafe { wxBitmap_SetHeight(self, h) }
    }
    #[fixed_stack_segment]
    fn setMask(&self, mask: @wxMask) {
        unsafe { wxBitmap_SetMask(self, mask) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, w: int) {
        unsafe { wxBitmap_SetWidth(self, w) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxBitmap_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxBitmap_IsStatic(self) }
    }
    #[fixed_stack_segment]
    fn newFromImage(image: @wxImage, depth: int) -> @wxBitmap {
        unsafe { wxBitmap_CreateFromImage(image, depth) }
    }
}
trait wxBitmapButton {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _bmp: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxBitmapButton {
        unsafe { wxBitmapButton_Create(_prt, _id, _bmp, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn getBitmapDisabled(&self, @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getBitmapFocus(&self, @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapFocus(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getBitmapLabel(&self, @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapLabel(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getBitmapSelected(&self, @wxBitmap) {
        unsafe { wxBitmapButton_GetBitmapSelected(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getMarginX(&self) -> int {
        unsafe { wxBitmapButton_GetMarginX(self) }
    }
    #[fixed_stack_segment]
    fn getMarginY(&self) -> int {
        unsafe { wxBitmapButton_GetMarginY(self) }
    }
    #[fixed_stack_segment]
    fn setBitmapDisabled(&self, disabled: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self, disabled) }
    }
    #[fixed_stack_segment]
    fn setBitmapFocus(&self, focus: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapFocus(self, focus) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel(&self, bitmap: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapLabel(self, bitmap) }
    }
    #[fixed_stack_segment]
    fn setBitmapSelected(&self, sel: @wxBitmap) {
        unsafe { wxBitmapButton_SetBitmapSelected(self, sel) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self, _arg1) }
    }
}
trait wxBitmapToggleButton {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, _bmp: @wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxBitmapToggleButton {
        unsafe { wxBitmapToggleButton_Create(parent, id, _bmp, _arg3, style) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> bool {
        unsafe { wxBitmapToggleButton_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxBitmapToggleButton_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, state: bool) {
        unsafe { wxBitmapToggleButton_SetValue(self, state) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel(&self, _bmp: @wxBitmap) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self, _bmp) }
    }
}
trait wxBitmapDataObject {
}
trait wxBitmapHandler {
}
trait wxBoxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { wxBoxSizer_CalcMin(self) }
    }
    #[fixed_stack_segment]
    fn new(orient: int) -> @wxBoxSizer {
        unsafe { wxBoxSizer_Create(orient) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> int {
        unsafe { wxBoxSizer_GetOrientation(self) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxBoxSizer_RecalcSizes(self) }
    }
}
trait wxBrush {
    #[fixed_stack_segment]
    fn assign(&self, brush: @wxBrush) {
        unsafe { wxBrush_Assign(self, brush) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxBrush {
        unsafe { wxBrush_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(bitmap: @wxBitmap) -> @wxBrush {
        unsafe { wxBrush_CreateFromBitmap(bitmap) }
    }
    #[fixed_stack_segment]
    fn newFromColour(col: @wxColour, style: int) -> @wxBrush {
        unsafe { wxBrush_CreateFromColour(col, style) }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: int) -> @wxBrush {
        unsafe { wxBrush_CreateFromStock(id) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBrush_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, @wxColour) {
        unsafe { wxBrush_GetColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getStipple(&self, @wxBitmap) {
        unsafe { wxBrush_GetStipple(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxBrush_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, brush: @wxBrush) -> bool {
        unsafe { wxBrush_IsEqual(self, brush) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBrush_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, col: @wxColour) {
        unsafe { wxBrush_SetColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxBrush_SetColourSingle(self, r, g, b) }
    }
    #[fixed_stack_segment]
    fn setStipple(&self, stipple: @wxBitmap) {
        unsafe { wxBrush_SetStipple(self, stipple) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxBrush_SetStyle(self, style) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxBrush_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxBrush_IsStatic(self) }
    }
}
trait wxBrushList {
}
trait wxBufferedDC {
    #[fixed_stack_segment]
    fn newByDCAndSize(dc: @wxDC, width: c_int, hight: c_int, style: int) -> @wxBufferedDC {
        unsafe { wxBufferedDC_CreateByDCAndSize(dc, _arg1, style) }
    }
    #[fixed_stack_segment]
    fn newByDCAndBitmap(dc: @wxDC, bitmap: @wxBitmap, style: int) -> @wxBufferedDC {
        unsafe { wxBufferedDC_CreateByDCAndBitmap(dc, bitmap, style) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedDC_Delete(self) }
    }
}
trait wxBufferedPaintDC {
    #[fixed_stack_segment]
    fn new(window: @wxWindow, style: int) -> @wxBufferedPaintDC {
        unsafe { wxBufferedPaintDC_Create(window, style) }
    }
    #[fixed_stack_segment]
    fn newWithBitmap(window: @wxWindow, bitmap: @wxBitmap, style: int) -> @wxBufferedPaintDC {
        unsafe { wxBufferedPaintDC_CreateWithBitmap(window, bitmap, style) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedPaintDC_Delete(self) }
    }
}
trait wxBufferedInputStream {
}
trait wxBufferedOutputStream {
}
trait wxBusyCursor {
    #[fixed_stack_segment]
    fn new() -> @wxBusyCursor {
        unsafe { wxBusyCursor_Create() }
    }
    #[fixed_stack_segment]
    fn newWithCursor(&self) {
        unsafe { wxBusyCursor_CreateWithCursor(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self) }
    }
}
trait wxBusyInfo {
    #[fixed_stack_segment]
    fn new(_txt: @wxString) -> @wxBusyInfo {
        unsafe { wxBusyInfo_Create(_txt) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self) }
    }
}
trait wxButton {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxButton {
        unsafe { wxButton_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @wxColour) -> int {
        unsafe { wxButton_SetBackgroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self) }
    }
}
trait wxCSConv {
}
trait wxCalculateLayoutEvent {
    #[fixed_stack_segment]
    fn new(id: int) -> @wxCalculateLayoutEvent {
        unsafe { wxCalculateLayoutEvent_Create(id) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @wxRect {
        unsafe { wxCalculateLayoutEvent_GetRect(self) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self, flags) }
    }
    #[fixed_stack_segment]
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxCalculateLayoutEvent_SetRect(self, _arg1) }
    }
}
trait wxCalendarCtrl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _dat: @wxDateTime, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxCalendarCtrl {
        unsafe { wxCalendarCtrl_Create(_prt, _id, _dat, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn enableHolidayDisplay(&self, display: int) {
        unsafe { wxCalendarCtrl_EnableHolidayDisplay(self, display) }
    }
    #[fixed_stack_segment]
    fn enableMonthChange(&self, enable: bool) {
        unsafe { wxCalendarCtrl_EnableMonthChange(self, enable) }
    }
    #[fixed_stack_segment]
    fn getAttr(&self, day: int) {
        unsafe { wxCalendarCtrl_GetAttr(self, day) }
    }
    #[fixed_stack_segment]
    fn getDate(&self, date: *c_void) {
        unsafe { wxCalendarCtrl_GetDate(self, date) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourBg(&self, @wxColour) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourFg(&self, @wxColour) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourBg(&self, @wxColour) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourFg(&self, @wxColour) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourBg(&self, @wxColour) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourFg(&self, @wxColour) {
        unsafe { wxCalendarCtrl_GetHolidayColourFg(self, _ref) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int, date: *c_void, wd: *c_void) -> int {
        unsafe { wxCalendarCtrl_HitTest(self, _arg1, date, wd) }
    }
    #[fixed_stack_segment]
    fn resetAttr(&self, day: int) {
        unsafe { wxCalendarCtrl_ResetAttr(self, day) }
    }
    #[fixed_stack_segment]
    fn setAttr(&self, day: int, attr: *c_void) {
        unsafe { wxCalendarCtrl_SetAttr(self, day, attr) }
    }
    #[fixed_stack_segment]
    fn setDate(&self, date: *c_void) {
        unsafe { wxCalendarCtrl_SetDate(self, date) }
    }
    #[fixed_stack_segment]
    fn setHeaderColours(&self, colFg: *c_void, colBg: *c_void) {
        unsafe { wxCalendarCtrl_SetHeaderColours(self, colFg, colBg) }
    }
    #[fixed_stack_segment]
    fn setHighlightColours(&self, colFg: *c_void, colBg: *c_void) {
        unsafe { wxCalendarCtrl_SetHighlightColours(self, colFg, colBg) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, day: int) {
        unsafe { wxCalendarCtrl_SetHoliday(self, day) }
    }
    #[fixed_stack_segment]
    fn setHolidayColours(&self, colFg: *c_void, colBg: *c_void) {
        unsafe { wxCalendarCtrl_SetHolidayColours(self, colFg, colBg) }
    }
}
trait wxCalendarDateAttr {
    #[fixed_stack_segment]
    fn new(_ctxt: *c_void, _cbck: *c_void, _cbrd: *c_void, _fnt: *c_void, _brd: int) -> @wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, @wxColour) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> int {
        unsafe { wxCalendarDateAttr_GetBorder(self) }
    }
    #[fixed_stack_segment]
    fn getBorderColour(&self, @wxColour) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, @wxFont) {
        unsafe { wxCalendarDateAttr_GetFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, @wxColour) {
        unsafe { wxCalendarDateAttr_GetTextColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBackgroundColour(self) }
    }
    #[fixed_stack_segment]
    fn hasBorder(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBorder(self) }
    }
    #[fixed_stack_segment]
    fn hasBorderColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasBorderColour(self) }
    }
    #[fixed_stack_segment]
    fn hasFont(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasFont(self) }
    }
    #[fixed_stack_segment]
    fn hasTextColour(&self) -> bool {
        unsafe { wxCalendarDateAttr_HasTextColour(self) }
    }
    #[fixed_stack_segment]
    fn isHoliday(&self) -> bool {
        unsafe { wxCalendarDateAttr_IsHoliday(self) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, col: @wxColour) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: int) {
        unsafe { wxCalendarDateAttr_SetBorder(self, border) }
    }
    #[fixed_stack_segment]
    fn setBorderColour(&self, col: @wxColour) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxCalendarDateAttr_SetFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, holiday: int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self, holiday) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @wxColour) {
        unsafe { wxCalendarDateAttr_SetTextColour(self, col) }
    }
}
trait wxCalendarEvent {
    #[fixed_stack_segment]
    fn getDate(&self, _dte: *c_void) {
        unsafe { wxCalendarEvent_GetDate(self, _dte) }
    }
    #[fixed_stack_segment]
    fn getWeekDay(&self) -> int {
        unsafe { wxCalendarEvent_GetWeekDay(self) }
    }
}
trait wxCaret {
    #[fixed_stack_segment]
    fn new(_wnd: @wxWindow, _wth: int, _hgt: int) -> @wxCaret {
        unsafe { wxCaret_Create(_wnd, _wth, _hgt) }
    }
    #[fixed_stack_segment]
    fn getBlinkTime() -> int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxCaret_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxCaret_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { wxCaret_GetWindow(self) }
    }
    #[fixed_stack_segment]
    fn hide(&self) {
        unsafe { wxCaret_Hide(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxCaret_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self) -> bool {
        unsafe { wxCaret_IsVisible(self) }
    }
    #[fixed_stack_segment]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxCaret_Move(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setBlinkTime(milliseconds: int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, width: c_int, height: c_int) {
        unsafe { wxCaret_SetSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn show(&self) {
        unsafe { wxCaret_Show(self) }
    }
}
trait wxCheckBox {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxCheckBox {
        unsafe { wxCheckBox_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCheckBox_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxCheckBox_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self, value) }
    }
}
trait wxCheckListBox {
    #[fixed_stack_segment]
    fn check(&self, item: int, check: bool) {
        unsafe { wxCheckListBox_Check(self, item, check) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxCheckListBox {
        unsafe { wxCheckListBox_Create(_prt, _id, _arg2, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCheckListBox_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self, item: int) -> bool {
        unsafe { wxCheckListBox_IsChecked(self, item) }
    }
}
trait wxChoice {
    #[fixed_stack_segment]
    fn append(&self, item: @wxString) {
        unsafe { wxChoice_Append(self, item) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxChoice_Clear(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxChoice {
        unsafe { wxChoice_Create(_prt, _id, _arg2, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: int) {
        unsafe { wxChoice_Delete(self, n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> int {
        unsafe { wxChoice_FindString(self, s) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxChoice_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> int {
        unsafe { wxChoice_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn getString(&self, n: int) -> @wxString {
        unsafe { wxChoice_GetString(self, n) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: int) {
        unsafe { wxChoice_SetSelection(self, n) }
    }
    #[fixed_stack_segment]
    fn setString(&self, n: int, s: @wxString) {
        unsafe { wxChoice_SetString(self, n, s) }
    }
}
trait wxClassInfo {
    #[fixed_stack_segment]
    fn newClassByName(&self) {
        unsafe { wxClassInfo_CreateClassByName(self) }
    }
    #[fixed_stack_segment]
    fn getClassName(&self) {
        unsafe { wxClassInfo_GetClassName(self) }
    }
    #[fixed_stack_segment]
    fn isKindOf(&self, _name: @wxString) -> bool {
        unsafe { wxClassInfo_IsKindOf(self, _name) }
    }
    #[fixed_stack_segment]
    fn findClass(_txt: @wxString) -> @wxClassInfo {
        unsafe { wxClassInfo_FindClass(_txt) }
    }
    #[fixed_stack_segment]
    fn getBaseClassName1(&self) -> @wxString {
        unsafe { wxClassInfo_GetBaseClassName1(self) }
    }
    #[fixed_stack_segment]
    fn getBaseClassName2(&self) -> @wxString {
        unsafe { wxClassInfo_GetBaseClassName2(self) }
    }
    #[fixed_stack_segment]
    fn getClassNameEx(&self) -> @wxString {
        unsafe { wxClassInfo_GetClassNameEx(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> int {
        unsafe { wxClassInfo_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn isKindOfEx(&self, classInfo: @wxClassInfo) -> bool {
        unsafe { wxClassInfo_IsKindOfEx(self, classInfo) }
    }
}
trait wxClient {
}
trait wxClientBase {
}
trait wxClientDC {
    #[fixed_stack_segment]
    fn new(win: @wxWindow) -> @wxClientDC {
        unsafe { wxClientDC_Create(win) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxClientDC_Delete(self) }
    }
}
trait wxClientData {
}
trait wxClientDataContainer {
}
trait wxClipboard {
    #[fixed_stack_segment]
    fn addData(&self, data: @wxDataObject) -> bool {
        unsafe { wxClipboard_AddData(self, data) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxClipboard_Clear(self) }
    }
    #[fixed_stack_segment]
    fn close(&self) {
        unsafe { wxClipboard_Close(self) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxClipboard {
        unsafe { wxClipboard_Create() }
    }
    #[fixed_stack_segment]
    fn flush(&self) -> bool {
        unsafe { wxClipboard_Flush(self) }
    }
    #[fixed_stack_segment]
    fn getData(&self, data: @wxDataObject) -> bool {
        unsafe { wxClipboard_GetData(self, data) }
    }
    #[fixed_stack_segment]
    fn isOpened(&self) -> bool {
        unsafe { wxClipboard_IsOpened(self) }
    }
    #[fixed_stack_segment]
    fn isSupported(&self, format: @wxDataFormat) -> bool {
        unsafe { wxClipboard_IsSupported(self, format) }
    }
    #[fixed_stack_segment]
    fn open(&self) -> bool {
        unsafe { wxClipboard_Open(self) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @wxDataObject) -> bool {
        unsafe { wxClipboard_SetData(self, data) }
    }
    #[fixed_stack_segment]
    fn usePrimarySelection(&self, primary: bool) {
        unsafe { wxClipboard_UsePrimarySelection(self, primary) }
    }
}
trait wxCloseEvent {
    #[fixed_stack_segment]
    fn canVeto(&self) -> bool {
        unsafe { wxCloseEvent_CanVeto(self) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
        unsafe { wxCloseEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getLoggingOff(&self) -> bool {
        unsafe { wxCloseEvent_GetLoggingOff(self) }
    }
    #[fixed_stack_segment]
    fn getVeto(&self) -> bool {
        unsafe { wxCloseEvent_GetVeto(self) }
    }
    #[fixed_stack_segment]
    fn setCanVeto(&self, canVeto: bool) {
        unsafe { wxCloseEvent_SetCanVeto(self, canVeto) }
    }
    #[fixed_stack_segment]
    fn setLoggingOff(&self, logOff: bool) {
        unsafe { wxCloseEvent_SetLoggingOff(self, logOff) }
    }
    #[fixed_stack_segment]
    fn veto(&self, veto: bool) {
        unsafe { wxCloseEvent_Veto(self, veto) }
    }
}
trait wxClosure {
    #[fixed_stack_segment]
    fn new(_fun_CEvent: *c_void, _data: *c_void) -> @wxClosure {
        unsafe { wxClosure_Create(_fun_CEvent, _data) }
    }
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxClosure_GetData(self) }
    }
}
trait wxColour {
    #[fixed_stack_segment]
    fn alpha(&self) -> uint8_t {
        unsafe { wxColour_Alpha(self) }
    }
    #[fixed_stack_segment]
    fn assign(&self, other: *c_void) {
        unsafe { wxColour_Assign(self, other) }
    }
    #[fixed_stack_segment]
    fn blue(&self) -> uint8_t {
        unsafe { wxColour_Blue(self) }
    }
    #[fixed_stack_segment]
    fn copy(&self, _other: *c_void) {
        unsafe { wxColour_Copy(self, _other) }
    }
    #[fixed_stack_segment]
    fn newByName(_name: @wxString) -> @wxColour {
        unsafe { wxColour_CreateByName(_name) }
    }
    #[fixed_stack_segment]
    fn newEmpty() -> @wxColour {
        unsafe { wxColour_CreateEmpty() }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: int) -> @wxColour {
        unsafe { wxColour_CreateFromStock(id) }
    }
    #[fixed_stack_segment]
    fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> @wxColour {
        unsafe { wxColour_CreateRGB(_red, _green, _blue, _alpha) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxColour_Delete(self) }
    }
    #[fixed_stack_segment]
    fn green(&self) -> uint8_t {
        unsafe { wxColour_Green(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxColour_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn red(&self) -> uint8_t {
        unsafe { wxColour_Red(self) }
    }
    #[fixed_stack_segment]
    fn set(&self, _red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) {
        unsafe { wxColour_Set(self, _red, _green, _blue, _alpha) }
    }
    #[fixed_stack_segment]
    fn setByName(&self, _name: @wxString) {
        unsafe { wxColour_SetByName(self, _name) }
    }
    #[fixed_stack_segment]
    fn validName(_name: *wchar_t) -> bool {
        unsafe { wxColour_ValidName(_name) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxColour_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxColour_IsStatic(self) }
    }
    #[fixed_stack_segment]
    fn newFromInt(rgb: int) -> @wxColour {
        unsafe { wxColour_CreateFromInt(rgb) }
    }
    #[fixed_stack_segment]
    fn getInt(&self) -> int {
        unsafe { wxColour_GetInt(self) }
    }
    #[fixed_stack_segment]
    fn newFromUnsignedInt(rgba: uint32_t) -> @wxColour {
        unsafe { wxColour_CreateFromUnsignedInt(rgba) }
    }
    #[fixed_stack_segment]
    fn getUnsignedInt(&self) -> uint32_t {
        unsafe { wxColour_GetUnsignedInt(self) }
    }
}
trait wxColourData {
    #[fixed_stack_segment]
    fn new() -> @wxColourData {
        unsafe { wxColourData_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxColourData_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getChooseFull(&self) -> bool {
        unsafe { wxColourData_GetChooseFull(self) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, @wxColour) {
        unsafe { wxColourData_GetColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getCustomColour(&self, i: int, @wxColour) {
        unsafe { wxColourData_GetCustomColour(self, i, _ref) }
    }
    #[fixed_stack_segment]
    fn setChooseFull(&self, flag: bool) {
        unsafe { wxColourData_SetChooseFull(self, flag) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @wxColour) {
        unsafe { wxColourData_SetColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setCustomColour(&self, i: int, colour: @wxColour) {
        unsafe { wxColourData_SetCustomColour(self, i, colour) }
    }
}
trait wxColourDatabase {
}
trait wxColourDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, col: @wxColourData) -> @wxColourDialog {
        unsafe { wxColourDialog_Create(_prt, col) }
    }
    #[fixed_stack_segment]
    fn getColourData(&self, @wxColourData) {
        unsafe { wxColourDialog_GetColourData(self, _ref) }
    }
}
trait wxComboBox {
    #[fixed_stack_segment]
    fn append(&self, item: @wxString) {
        unsafe { wxComboBox_Append(self, item) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @wxString, d: *c_void) {
        unsafe { wxComboBox_AppendData(self, item, d) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxComboBox_Clear(self) }
    }
    #[fixed_stack_segment]
    fn copy(&self) {
        unsafe { wxComboBox_Copy(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxComboBox {
        unsafe { wxComboBox_Create(_prt, _id, _txt, _arg3, _arg4, _stl) }
    }
    #[fixed_stack_segment]
    fn cut(&self) {
        unsafe { wxComboBox_Cut(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: int) {
        unsafe { wxComboBox_Delete(self, n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> int {
        unsafe { wxComboBox_FindString(self, s) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: int) -> @wxClientData {
        unsafe { wxComboBox_GetClientData(self, n) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxComboBox_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getInsertionPoint(&self) -> int {
        unsafe { wxComboBox_GetInsertionPoint(self) }
    }
    #[fixed_stack_segment]
    fn getLastPosition(&self) -> int {
        unsafe { wxComboBox_GetLastPosition(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> int {
        unsafe { wxComboBox_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn getString(&self, n: int) -> @wxString {
        unsafe { wxComboBox_GetString(self, n) }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @wxString {
        unsafe { wxComboBox_GetStringSelection(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> @wxString {
        unsafe { wxComboBox_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn paste(&self) {
        unsafe { wxComboBox_Paste(self) }
    }
    #[fixed_stack_segment]
    fn remove(&self, from: int, to: int) {
        unsafe { wxComboBox_Remove(self, from, to) }
    }
    #[fixed_stack_segment]
    fn replace(&self, from: int, to: int, value: @wxString) {
        unsafe { wxComboBox_Replace(self, from, to, value) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, n: int, clientData: @wxClientData) {
        unsafe { wxComboBox_SetClientData(self, n, clientData) }
    }
    #[fixed_stack_segment]
    fn setEditable(&self, editable: bool) {
        unsafe { wxComboBox_SetEditable(self, editable) }
    }
    #[fixed_stack_segment]
    fn setInsertionPoint(&self, pos: int) {
        unsafe { wxComboBox_SetInsertionPoint(self, pos) }
    }
    #[fixed_stack_segment]
    fn setInsertionPointEnd(&self) {
        unsafe { wxComboBox_SetInsertionPointEnd(self) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: int) {
        unsafe { wxComboBox_SetSelection(self, n) }
    }
    #[fixed_stack_segment]
    fn setTextSelection(&self, from: int, to: int) {
        unsafe { wxComboBox_SetTextSelection(self, from, to) }
    }
}
trait wxCommand {
}
trait wxCommandEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *c_void) {
        unsafe { wxCommandEvent_CopyObject(self, object_dest) }
    }
    #[fixed_stack_segment]
    fn new(_typ: int, _id: int) -> @wxCommandEvent {
        unsafe { wxCommandEvent_Create(_typ, _id) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCommandEvent_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @wxClientData {
        unsafe { wxCommandEvent_GetClientData(self) }
    }
    #[fixed_stack_segment]
    fn getClientObject(&self) -> @wxClientData {
        unsafe { wxCommandEvent_GetClientObject(self) }
    }
    #[fixed_stack_segment]
    fn getExtraLong(&self) -> c_long {
        unsafe { wxCommandEvent_GetExtraLong(self) }
    }
    #[fixed_stack_segment]
    fn getInt(&self) -> c_long {
        unsafe { wxCommandEvent_GetInt(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> int {
        unsafe { wxCommandEvent_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn getString(&self) -> @wxString {
        unsafe { wxCommandEvent_GetString(self) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self) -> bool {
        unsafe { wxCommandEvent_IsChecked(self) }
    }
    #[fixed_stack_segment]
    fn isSelection(&self) -> bool {
        unsafe { wxCommandEvent_IsSelection(self) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, clientData: @wxClientData) {
        unsafe { wxCommandEvent_SetClientData(self, clientData) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, clientObject: @wxClientData) {
        unsafe { wxCommandEvent_SetClientObject(self, clientObject) }
    }
    #[fixed_stack_segment]
    fn setExtraLong(&self, extraLong: c_long) {
        unsafe { wxCommandEvent_SetExtraLong(self, extraLong) }
    }
    #[fixed_stack_segment]
    fn setInt(&self, i: int) {
        unsafe { wxCommandEvent_SetInt(self, i) }
    }
    #[fixed_stack_segment]
    fn setString(&self, s: @wxString) {
        unsafe { wxCommandEvent_SetString(self, s) }
    }
}
trait wxCommandLineParser {
}
trait wxCommandProcessor {
    #[fixed_stack_segment]
    fn canRedo(&self) -> bool {
        unsafe { wxCommandProcessor_CanRedo(self) }
    }
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { wxCommandProcessor_CanUndo(self) }
    }
    #[fixed_stack_segment]
    fn clearCommands(&self) {
        unsafe { wxCommandProcessor_ClearCommands(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCommandProcessor_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCommands(&self, _ref: *c_void) -> int {
        unsafe { wxCommandProcessor_GetCommands(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getEditMenu(&self) {
        unsafe { wxCommandProcessor_GetEditMenu(self) }
    }
    #[fixed_stack_segment]
    fn getMaxCommands(&self) -> int {
        unsafe { wxCommandProcessor_GetMaxCommands(self) }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { wxCommandProcessor_Initialize(self) }
    }
    #[fixed_stack_segment]
    fn redo(&self) -> int {
        unsafe { wxCommandProcessor_Redo(self) }
    }
    #[fixed_stack_segment]
    fn setEditMenu(&self, menu: @wxMenu) {
        unsafe { wxCommandProcessor_SetEditMenu(self, menu) }
    }
    #[fixed_stack_segment]
    fn setMenuStrings(&self) {
        unsafe { wxCommandProcessor_SetMenuStrings(self) }
    }
    #[fixed_stack_segment]
    fn submit(&self, command: @wxCommand, storeIt: int) -> int {
        unsafe { wxCommandProcessor_Submit(self, command, storeIt) }
    }
    #[fixed_stack_segment]
    fn undo(&self) -> int {
        unsafe { wxCommandProcessor_Undo(self) }
    }
    #[fixed_stack_segment]
    fn wxCommandProcessor(maxCommands: int) {
        unsafe { wxCommandProcessor_wxCommandProcessor(maxCommands) }
    }
}
trait wxCondition {
    #[fixed_stack_segment]
    fn broadcast(&self) {
        unsafe { wxCondition_Broadcast(self) }
    }
    #[fixed_stack_segment]
    fn new(_mut: *c_void) -> @wxCondition {
        unsafe { wxCondition_Create(_mut) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCondition_Delete(self) }
    }
    #[fixed_stack_segment]
    fn signal(&self) {
        unsafe { wxCondition_Signal(self) }
    }
    #[fixed_stack_segment]
    fn wait(&self) {
        unsafe { wxCondition_Wait(self) }
    }
    #[fixed_stack_segment]
    fn waitFor(&self, sec: int, nsec: int) -> int {
        unsafe { wxCondition_WaitFor(self, sec, nsec) }
    }
}
trait wxConfigBase {
    #[fixed_stack_segment]
    fn new() -> @wxConfigBase {
        unsafe { wxConfigBase_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxConfigBase_Delete(self) }
    }
    #[fixed_stack_segment]
    fn deleteAll(&self) -> bool {
        unsafe { wxConfigBase_DeleteAll(self) }
    }
    #[fixed_stack_segment]
    fn deleteEntry(&self, key: @wxString, bDeleteGroupIfEmpty: bool) -> bool {
        unsafe { wxConfigBase_DeleteEntry(self, key, bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    fn deleteGroup(&self, key: @wxString) -> bool {
        unsafe { wxConfigBase_DeleteGroup(self, key) }
    }
    #[fixed_stack_segment]
    fn exists(&self, strName: @wxString) -> bool {
        unsafe { wxConfigBase_Exists(self, strName) }
    }
    #[fixed_stack_segment]
    fn expandEnvVars(&self, str: @wxString) -> @wxString {
        unsafe { wxConfigBase_ExpandEnvVars(self, str) }
    }
    #[fixed_stack_segment]
    fn flush(&self, bCurrentOnly: bool) -> bool {
        unsafe { wxConfigBase_Flush(self, bCurrentOnly) }
    }
    #[fixed_stack_segment]
    fn getAppName(&self) -> @wxString {
        unsafe { wxConfigBase_GetAppName(self) }
    }
    #[fixed_stack_segment]
    fn getEntryType(&self, name: @wxString) -> int {
        unsafe { wxConfigBase_GetEntryType(self, name) }
    }
    #[fixed_stack_segment]
    fn getFirstEntry(&self, lIndex: *c_void) -> @wxString {
        unsafe { wxConfigBase_GetFirstEntry(self, lIndex) }
    }
    #[fixed_stack_segment]
    fn getFirstGroup(&self, lIndex: *c_void) -> @wxString {
        unsafe { wxConfigBase_GetFirstGroup(self, lIndex) }
    }
    #[fixed_stack_segment]
    fn getNextEntry(&self, lIndex: *c_void) -> @wxString {
        unsafe { wxConfigBase_GetNextEntry(self, lIndex) }
    }
    #[fixed_stack_segment]
    fn getNextGroup(&self, lIndex: *c_void) -> @wxString {
        unsafe { wxConfigBase_GetNextGroup(self, lIndex) }
    }
    #[fixed_stack_segment]
    fn getNumberOfEntries(&self, bRecursive: bool) -> int {
        unsafe { wxConfigBase_GetNumberOfEntries(self, bRecursive) }
    }
    #[fixed_stack_segment]
    fn getNumberOfGroups(&self, bRecursive: bool) -> int {
        unsafe { wxConfigBase_GetNumberOfGroups(self, bRecursive) }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @wxString {
        unsafe { wxConfigBase_GetPath(self) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxConfigBase_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn getVendorName(&self) -> @wxString {
        unsafe { wxConfigBase_GetVendorName(self) }
    }
    #[fixed_stack_segment]
    fn hasEntry(&self, strName: @wxString) -> bool {
        unsafe { wxConfigBase_HasEntry(self, strName) }
    }
    #[fixed_stack_segment]
    fn hasGroup(&self, strName: @wxString) -> bool {
        unsafe { wxConfigBase_HasGroup(self, strName) }
    }
    #[fixed_stack_segment]
    fn isExpandingEnvVars(&self) -> bool {
        unsafe { wxConfigBase_IsExpandingEnvVars(self) }
    }
    #[fixed_stack_segment]
    fn isRecordingDefaults(&self) -> bool {
        unsafe { wxConfigBase_IsRecordingDefaults(self) }
    }
    #[fixed_stack_segment]
    fn readBool(&self, key: @wxString, defVal: bool) -> bool {
        unsafe { wxConfigBase_ReadBool(self, key, defVal) }
    }
    #[fixed_stack_segment]
    fn readDouble(&self, key: @wxString, defVal: c_double) -> c_double {
        unsafe { wxConfigBase_ReadDouble(self, key, defVal) }
    }
    #[fixed_stack_segment]
    fn readInteger(&self, key: @wxString, defVal: int) -> int {
        unsafe { wxConfigBase_ReadInteger(self, key, defVal) }
    }
    #[fixed_stack_segment]
    fn readString(&self, key: @wxString, defVal: @wxString) -> @wxString {
        unsafe { wxConfigBase_ReadString(self, key, defVal) }
    }
    #[fixed_stack_segment]
    fn renameEntry(&self, oldName: @wxString, newName: @wxString) -> bool {
        unsafe { wxConfigBase_RenameEntry(self, oldName, newName) }
    }
    #[fixed_stack_segment]
    fn renameGroup(&self, oldName: @wxString, newName: @wxString) -> bool {
        unsafe { wxConfigBase_RenameGroup(self, oldName, newName) }
    }
    #[fixed_stack_segment]
    fn setAppName(&self, appName: @wxString) {
        unsafe { wxConfigBase_SetAppName(self, appName) }
    }
    #[fixed_stack_segment]
    fn setExpandEnvVars(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetExpandEnvVars(self, bDoIt) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, strPath: @wxString) {
        unsafe { wxConfigBase_SetPath(self, strPath) }
    }
    #[fixed_stack_segment]
    fn setRecordDefaults(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetRecordDefaults(self, bDoIt) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxConfigBase_SetStyle(self, style) }
    }
    #[fixed_stack_segment]
    fn setVendorName(&self, vendorName: @wxString) {
        unsafe { wxConfigBase_SetVendorName(self, vendorName) }
    }
    #[fixed_stack_segment]
    fn writeBool(&self, key: @wxString, value: bool) -> bool {
        unsafe { wxConfigBase_WriteBool(self, key, value) }
    }
    #[fixed_stack_segment]
    fn writeDouble(&self, key: @wxString, value: c_double) -> bool {
        unsafe { wxConfigBase_WriteDouble(self, key, value) }
    }
    #[fixed_stack_segment]
    fn writeInteger(&self, key: @wxString, value: int) -> bool {
        unsafe { wxConfigBase_WriteInteger(self, key, value) }
    }
    #[fixed_stack_segment]
    fn writeLong(&self, key: @wxString, value: c_long) -> bool {
        unsafe { wxConfigBase_WriteLong(self, key, value) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, key: @wxString, value: @wxString) -> bool {
        unsafe { wxConfigBase_WriteString(self, key, value) }
    }
    #[fixed_stack_segment]
    fn get() -> @wxConfigBase {
        unsafe { wxConfigBase_Get() }
    }
    #[fixed_stack_segment]
    fn set(self_: @wxConfigBase) {
        unsafe { wxConfigBase_Set(self_) }
    }
}
trait wxConnection {
}
trait wxConnectionBase {
}
trait wxContextHelp {
    #[fixed_stack_segment]
    fn beginContextHelp(&self, win: @wxWindow) -> bool {
        unsafe { wxContextHelp_BeginContextHelp(self, win) }
    }
    #[fixed_stack_segment]
    fn new(win: @wxWindow, beginHelp: bool) -> @wxContextHelp {
        unsafe { wxContextHelp_Create(win, beginHelp) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxContextHelp_Delete(self) }
    }
    #[fixed_stack_segment]
    fn endContextHelp(&self) -> bool {
        unsafe { wxContextHelp_EndContextHelp(self) }
    }
}
trait wxContextHelpButton {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @wxContextHelpButton {
        unsafe { wxContextHelpButton_Create(parent, id, _arg2, style) }
    }
}
trait wxControl {
    #[fixed_stack_segment]
    fn command(&self, event: @wxEvent) {
        unsafe { wxControl_Command(self, event) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { wxControl_GetLabel(self) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, text: @wxString) {
        unsafe { wxControl_SetLabel(self, text) }
    }
}
trait wxCountingOutputStream {
}
trait wxCriticalSection {
    #[fixed_stack_segment]
    fn new() -> @wxCriticalSection {
        unsafe { wxCriticalSection_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCriticalSection_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enter(&self) {
        unsafe { wxCriticalSection_Enter(self) }
    }
    #[fixed_stack_segment]
    fn leave(&self) {
        unsafe { wxCriticalSection_Leave(self) }
    }
}
trait wxCriticalSectionLocker {
}
trait wxCursor {
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxCursor_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxCursor_IsStatic(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCursor_Delete(self) }
    }
}
trait wxCustomDataObject {
}
trait wxDC {
    #[fixed_stack_segment]
    fn blit(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: @wxDC, xsrc: c_int, ysrc: c_int, rop: int, useMask: bool) -> bool {
        unsafe { wxDC_Blit(self, _arg1, source, _arg3, rop, useMask) }
    }
    #[fixed_stack_segment]
    fn calcBoundingBox(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CalcBoundingBox(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn canDrawBitmap(&self) -> bool {
        unsafe { wxDC_CanDrawBitmap(self) }
    }
    #[fixed_stack_segment]
    fn canGetTextExtent(&self) -> bool {
        unsafe { wxDC_CanGetTextExtent(self) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxDC_Clear(self) }
    }
    #[fixed_stack_segment]
    fn computeScaleAndOrigin(&self) {
        unsafe { wxDC_ComputeScaleAndOrigin(self) }
    }
    #[fixed_stack_segment]
    fn crossHair(&self, x: c_int, y: c_int) {
        unsafe { wxDC_CrossHair(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDC_Delete(self) }
    }
    #[fixed_stack_segment]
    fn destroyClippingRegion(&self) {
        unsafe { wxDC_DestroyClippingRegion(self) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalX(&self, x: int) -> int {
        unsafe { wxDC_DeviceToLogicalX(self, x) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalXRel(&self, x: int) -> int {
        unsafe { wxDC_DeviceToLogicalXRel(self, x) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalY(&self, y: int) -> int {
        unsafe { wxDC_DeviceToLogicalY(self, y) }
    }
    #[fixed_stack_segment]
    fn deviceToLogicalYRel(&self, y: int) -> int {
        unsafe { wxDC_DeviceToLogicalYRel(self, y) }
    }
    #[fixed_stack_segment]
    fn drawArc(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, xc: c_int, yc: c_int) {
        unsafe { wxDC_DrawArc(self, _arg1, _arg2, _arg3) }
    }
    #[fixed_stack_segment]
    fn drawBitmap(&self, bmp: @wxBitmap, x: c_int, y: c_int, useMask: bool) {
        unsafe { wxDC_DrawBitmap(self, bmp, _arg2, useMask) }
    }
    #[fixed_stack_segment]
    fn drawCheckMark(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawCheckMark(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn drawCircle(&self, x: c_int, y: c_int, radius: int) {
        unsafe { wxDC_DrawCircle(self, _arg1, radius) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawEllipse(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn drawEllipticArc(&self, x: c_int, y: c_int, w: c_int, h: c_int, sa: c_double, ea: c_double) {
        unsafe { wxDC_DrawEllipticArc(self, _arg1, sa, ea) }
    }
    #[fixed_stack_segment]
    fn drawIcon(&self, icon: @wxIcon, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self, icon, _arg2) }
    }
    #[fixed_stack_segment]
    fn drawLabel(&self, str: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, align: int, indexAccel: int) {
        unsafe { wxDC_DrawLabel(self, str, _arg2, align, indexAccel) }
    }
    #[fixed_stack_segment]
    fn drawLabelBitmap(&self, str: @wxString, bmp: @wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, align: int, indexAccel: int) -> @wxRect {
        unsafe { wxDC_DrawLabelBitmap(self, str, bmp, _arg3, align, indexAccel) }
    }
    #[fixed_stack_segment]
    fn drawLine(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { wxDC_DrawLine(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: int, x: *c_void, y: *c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { wxDC_DrawLines(self, n, x, y, _arg4) }
    }
    #[fixed_stack_segment]
    fn drawPoint(&self, x: c_int, y: c_int) {
        unsafe { wxDC_DrawPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn drawPolygon(&self, n: int, x: *c_void, y: *c_void, xoffset: c_int, yoffset: c_int, fillStyle: int) {
        unsafe { wxDC_DrawPolygon(self, n, x, y, _arg4, fillStyle) }
    }
    #[fixed_stack_segment]
    fn drawPolyPolygon(&self, n: int, count: *c_void, x: *c_void, y: *c_void, xoffset: c_int, yoffset: c_int, fillStyle: int) {
        unsafe { wxDC_DrawPolyPolygon(self, n, count, x, y, _arg5, fillStyle) }
    }
    #[fixed_stack_segment]
    fn drawRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_DrawRectangle(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn drawRotatedText(&self, text: @wxString, x: c_int, y: c_int, angle: c_double) {
        unsafe { wxDC_DrawRotatedText(self, text, _arg2, angle) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self, _arg1, radius) }
    }
    #[fixed_stack_segment]
    fn drawText(&self, text: @wxString, x: c_int, y: c_int) {
        unsafe { wxDC_DrawText(self, text, _arg2) }
    }
    #[fixed_stack_segment]
    fn endDoc(&self) {
        unsafe { wxDC_EndDoc(self) }
    }
    #[fixed_stack_segment]
    fn endPage(&self) {
        unsafe { wxDC_EndPage(self) }
    }
    #[fixed_stack_segment]
    fn floodFill(&self, x: c_int, y: c_int, col: @wxColour, style: int) {
        unsafe { wxDC_FloodFill(self, _arg1, col, style) }
    }
    #[fixed_stack_segment]
    fn getBackground(&self, @wxBrush) {
        unsafe { wxDC_GetBackground(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getBackgroundMode(&self) -> int {
        unsafe { wxDC_GetBackgroundMode(self) }
    }
    #[fixed_stack_segment]
    fn getBrush(&self, @wxBrush) {
        unsafe { wxDC_GetBrush(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getCharHeight(&self) -> int {
        unsafe { wxDC_GetCharHeight(self) }
    }
    #[fixed_stack_segment]
    fn getCharWidth(&self) -> int {
        unsafe { wxDC_GetCharWidth(self) }
    }
    #[fixed_stack_segment]
    fn getClippingBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxDC_GetClippingBox(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getDepth(&self) -> int {
        unsafe { wxDC_GetDepth(self) }
    }
    #[fixed_stack_segment]
    fn getDeviceOrigin(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxDC_GetDeviceOrigin(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, @wxFont) {
        unsafe { wxDC_GetFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getLogicalFunction(&self) -> int {
        unsafe { wxDC_GetLogicalFunction(self) }
    }
    #[fixed_stack_segment]
    fn getLogicalOrigin(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxDC_GetLogicalOrigin(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getLogicalScale(&self, _x: *c_double, _y: *c_double) {
        unsafe { wxDC_GetLogicalScale(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getMapMode(&self) -> int {
        unsafe { wxDC_GetMapMode(self) }
    }
    #[fixed_stack_segment]
    fn getPPI(&self) -> @wxSize {
        unsafe { wxDC_GetPPI(self) }
    }
    #[fixed_stack_segment]
    fn getPen(&self, @wxPen) {
        unsafe { wxDC_GetPen(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getPixel(&self, x: c_int, y: c_int, col: @wxColour) -> bool {
        unsafe { wxDC_GetPixel(self, _arg1, col) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxDC_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn getSizeMM(&self) -> @wxSize {
        unsafe { wxDC_GetSizeMM(self) }
    }
    #[fixed_stack_segment]
    fn getTextBackground(&self, @wxColour) {
        unsafe { wxDC_GetTextBackground(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @wxString, w: *c_void, h: *c_void, descent: *c_void, externalLeading: *c_void, theFont: @wxFont) {
        unsafe { wxDC_GetTextExtent(self, string, w, h, descent, externalLeading, theFont) }
    }
    #[fixed_stack_segment]
    fn getMultiLineTextExtent(&self, string: @wxString, w: *c_void, h: *c_void, heightLine: *c_void, theFont: @wxFont) {
        unsafe { wxDC_GetMultiLineTextExtent(self, string, w, h, heightLine, theFont) }
    }
    #[fixed_stack_segment]
    fn getTextForeground(&self, @wxColour) {
        unsafe { wxDC_GetTextForeground(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getUserScale(&self, x: *c_double, y: *c_double) {
        unsafe { wxDC_GetUserScale(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceX(&self, x: int) -> int {
        unsafe { wxDC_LogicalToDeviceX(self, x) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceXRel(&self, x: int) -> int {
        unsafe { wxDC_LogicalToDeviceXRel(self, x) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceY(&self, y: int) -> int {
        unsafe { wxDC_LogicalToDeviceY(self, y) }
    }
    #[fixed_stack_segment]
    fn logicalToDeviceYRel(&self, y: int) -> int {
        unsafe { wxDC_LogicalToDeviceYRel(self, y) }
    }
    #[fixed_stack_segment]
    fn maxX(&self) -> int {
        unsafe { wxDC_MaxX(self) }
    }
    #[fixed_stack_segment]
    fn maxY(&self) -> int {
        unsafe { wxDC_MaxY(self) }
    }
    #[fixed_stack_segment]
    fn minX(&self) -> int {
        unsafe { wxDC_MinX(self) }
    }
    #[fixed_stack_segment]
    fn minY(&self) -> int {
        unsafe { wxDC_MinY(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxDC_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn resetBoundingBox(&self) {
        unsafe { wxDC_ResetBoundingBox(self) }
    }
    #[fixed_stack_segment]
    fn setAxisOrientation(&self, xLeftRight: bool, yBottomUp: bool) {
        unsafe { wxDC_SetAxisOrientation(self, xLeftRight, yBottomUp) }
    }
    #[fixed_stack_segment]
    fn setBackground(&self, brush: @wxBrush) {
        unsafe { wxDC_SetBackground(self, brush) }
    }
    #[fixed_stack_segment]
    fn setBackgroundMode(&self, mode: int) {
        unsafe { wxDC_SetBackgroundMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @wxBrush) {
        unsafe { wxDC_SetBrush(self, brush) }
    }
    #[fixed_stack_segment]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setClippingRegionFromRegion(&self, region: @wxRegion) {
        unsafe { wxDC_SetClippingRegionFromRegion(self, region) }
    }
    #[fixed_stack_segment]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxDC_SetFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setLogicalFunction(&self, function: int) {
        unsafe { wxDC_SetLogicalFunction(self, function) }
    }
    #[fixed_stack_segment]
    fn setLogicalOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetLogicalOrigin(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setLogicalScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetLogicalScale(self, x, y) }
    }
    #[fixed_stack_segment]
    fn setMapMode(&self, mode: int) {
        unsafe { wxDC_SetMapMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn setPalette(&self, palette: @wxPalette) {
        unsafe { wxDC_SetPalette(self, palette) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @wxPen) {
        unsafe { wxDC_SetPen(self, pen) }
    }
    #[fixed_stack_segment]
    fn setTextBackground(&self, colour: @wxColour) {
        unsafe { wxDC_SetTextBackground(self, colour) }
    }
    #[fixed_stack_segment]
    fn setTextForeground(&self, colour: @wxColour) {
        unsafe { wxDC_SetTextForeground(self, colour) }
    }
    #[fixed_stack_segment]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self, x, y) }
    }
    #[fixed_stack_segment]
    fn startDoc(&self, msg: @wxString) -> bool {
        unsafe { wxDC_StartDoc(self, msg) }
    }
    #[fixed_stack_segment]
    fn startPage(&self) {
        unsafe { wxDC_StartPage(self) }
    }
    #[fixed_stack_segment]
    fn getUserScaleX(&self) -> c_double {
        unsafe { wxDC_GetUserScaleX(self) }
    }
    #[fixed_stack_segment]
    fn getUserScaleY(&self) -> c_double {
        unsafe { wxDC_GetUserScaleY(self) }
    }
    #[fixed_stack_segment]
    fn getPixel2(&self, x: c_int, y: c_int, @wxColour) {
        unsafe { wxDC_GetPixel2(self, _arg1, col) }
    }
}
trait wxDCClipper {
}
trait wxDDEClient {
}
trait wxDDEConnection {
}
trait wxDDEServer {
}
trait wxDataFormat {
    #[fixed_stack_segment]
    fn newFromId(name: @wxString) -> @wxDataFormat {
        unsafe { wxDataFormat_CreateFromId(name) }
    }
    #[fixed_stack_segment]
    fn newFromType(typ: int) -> @wxDataFormat {
        unsafe { wxDataFormat_CreateFromType(typ) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> @wxString {
        unsafe { wxDataFormat_GetId(self) }
    }
    #[fixed_stack_segment]
    fn getType(&self) -> int {
        unsafe { wxDataFormat_GetType(self) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, other: *c_void) -> bool {
        unsafe { wxDataFormat_IsEqual(self, other) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: *c_void) {
        unsafe { wxDataFormat_SetId(self, id) }
    }
    #[fixed_stack_segment]
    fn setType(&self, typ: int) {
        unsafe { wxDataFormat_SetType(self, typ) }
    }
}
trait wxDataInputStream {
}
trait wxDataObject {
}
trait wxDataObjectComposite {
    #[fixed_stack_segment]
    fn add(&self, _dat: *c_void, _preferred: int) {
        unsafe { wxDataObjectComposite_Add(self, _dat, _preferred) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxDataObjectComposite {
        unsafe { wxDataObjectComposite_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self) }
    }
}
trait wxDataObjectSimple {
}
trait wxDataOutputStream {
}
trait wxDatabase {
}
trait wxDateTime {
    #[fixed_stack_segment]
    fn addDate(&self, diff: *c_void, @wxDateTime) {
        unsafe { wxDateTime_AddDate(self, diff, _ref) }
    }
    #[fixed_stack_segment]
    fn addDateValues(&self, _yrs: int, _mnt: int, _wek: int, _day: int) {
        unsafe { wxDateTime_AddDateValues(self, _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    fn addTime(&self, diff: *c_void, @wxDateTime) {
        unsafe { wxDateTime_AddTime(self, diff, _ref) }
    }
    #[fixed_stack_segment]
    fn addTimeValues(&self, _hrs: int, _min: int, _sec: int, _mls: int) {
        unsafe { wxDateTime_AddTimeValues(self, _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    fn convertYearToBC(year: int) -> int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxDateTime {
        unsafe { wxDateTime_Create() }
    }
    #[fixed_stack_segment]
    fn format(&self, format: *c_void, tz: int) -> @wxString {
        unsafe { wxDateTime_Format(self, format, tz) }
    }
    #[fixed_stack_segment]
    fn formatDate(&self) -> @wxString {
        unsafe { wxDateTime_FormatDate(self) }
    }
    #[fixed_stack_segment]
    fn formatISODate(&self) -> @wxString {
        unsafe { wxDateTime_FormatISODate(self) }
    }
    #[fixed_stack_segment]
    fn formatISOTime(&self) -> @wxString {
        unsafe { wxDateTime_FormatISOTime(self) }
    }
    #[fixed_stack_segment]
    fn formatTime(&self) -> @wxString {
        unsafe { wxDateTime_FormatTime(self) }
    }
    #[fixed_stack_segment]
    fn getAmString() -> @wxString {
        unsafe { wxDateTime_GetAmString() }
    }
    #[fixed_stack_segment]
    fn getBeginDST(year: int, country: int, dt: @wxDateTime) {
        unsafe { wxDateTime_GetBeginDST(year, country, dt) }
    }
    #[fixed_stack_segment]
    fn getCentury(year: int) -> int {
        unsafe { wxDateTime_GetCentury(year) }
    }
    #[fixed_stack_segment]
    fn getCountry() -> int {
        unsafe { wxDateTime_GetCountry() }
    }
    #[fixed_stack_segment]
    fn getCurrentMonth(cal: int) -> int {
        unsafe { wxDateTime_GetCurrentMonth(cal) }
    }
    #[fixed_stack_segment]
    fn getCurrentYear(cal: int) -> int {
        unsafe { wxDateTime_GetCurrentYear(cal) }
    }
    #[fixed_stack_segment]
    fn getDay(&self, tz: int) -> int {
        unsafe { wxDateTime_GetDay(self, tz) }
    }
    #[fixed_stack_segment]
    fn getDayOfYear(&self, tz: int) -> int {
        unsafe { wxDateTime_GetDayOfYear(self, tz) }
    }
    #[fixed_stack_segment]
    fn getEndDST(year: int, country: int, dt: @wxDateTime) {
        unsafe { wxDateTime_GetEndDST(year, country, dt) }
    }
    #[fixed_stack_segment]
    fn getHour(&self, tz: int) -> int {
        unsafe { wxDateTime_GetHour(self, tz) }
    }
    #[fixed_stack_segment]
    fn getLastMonthDay(&self, month: int, year: int, @wxDateTime) {
        unsafe { wxDateTime_GetLastMonthDay(self, month, year, _ref) }
    }
    #[fixed_stack_segment]
    fn getLastWeekDay(&self, weekday: int, month: int, year: int, @wxDateTime) {
        unsafe { wxDateTime_GetLastWeekDay(self, weekday, month, year, _ref) }
    }
    #[fixed_stack_segment]
    fn getMillisecond(&self, tz: int) -> int {
        unsafe { wxDateTime_GetMillisecond(self, tz) }
    }
    #[fixed_stack_segment]
    fn getMinute(&self, tz: int) -> int {
        unsafe { wxDateTime_GetMinute(self, tz) }
    }
    #[fixed_stack_segment]
    fn getMonth(&self, tz: int) -> int {
        unsafe { wxDateTime_GetMonth(self, tz) }
    }
    #[fixed_stack_segment]
    fn getMonthName(month: int, flags: int) -> @wxString {
        unsafe { wxDateTime_GetMonthName(month, flags) }
    }
    #[fixed_stack_segment]
    fn getNextWeekDay(&self, weekday: int, @wxDateTime) {
        unsafe { wxDateTime_GetNextWeekDay(self, weekday, _ref) }
    }
    #[fixed_stack_segment]
    fn getNumberOfDays(year: int, cal: int) -> int {
        unsafe { wxDateTime_GetNumberOfDays(year, cal) }
    }
    #[fixed_stack_segment]
    fn getNumberOfDaysMonth(month: int, year: int, cal: int) -> int {
        unsafe { wxDateTime_GetNumberOfDaysMonth(month, year, cal) }
    }
    #[fixed_stack_segment]
    fn getPmString() -> @wxString {
        unsafe { wxDateTime_GetPmString() }
    }
    #[fixed_stack_segment]
    fn getPrevWeekDay(&self, weekday: int, @wxDateTime) {
        unsafe { wxDateTime_GetPrevWeekDay(self, weekday, _ref) }
    }
    #[fixed_stack_segment]
    fn getSecond(&self, tz: int) -> int {
        unsafe { wxDateTime_GetSecond(self, tz) }
    }
    #[fixed_stack_segment]
    fn getTicks(&self) -> time_t {
        unsafe { wxDateTime_GetTicks(self) }
    }
    #[fixed_stack_segment]
    fn getTimeNow() -> int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    fn getValue(&self, hi_long: *c_void, lo_long: *c_void) {
        unsafe { wxDateTime_GetValue(self, hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    fn getWeekDay(&self, weekday: int, n: int, month: int, year: int, @wxDateTime) {
        unsafe { wxDateTime_GetWeekDay(self, weekday, n, month, year, _ref) }
    }
    #[fixed_stack_segment]
    fn getWeekDayInSameWeek(&self, weekday: int, @wxDateTime) {
        unsafe { wxDateTime_GetWeekDayInSameWeek(self, weekday, _ref) }
    }
    #[fixed_stack_segment]
    fn getWeekDayName(weekday: int, flags: int) -> @wxString {
        unsafe { wxDateTime_GetWeekDayName(weekday, flags) }
    }
    #[fixed_stack_segment]
    fn getWeekDayTZ(&self, tz: int) -> int {
        unsafe { wxDateTime_GetWeekDayTZ(self, tz) }
    }
    #[fixed_stack_segment]
    fn getWeekOfMonth(&self, flags: int, tz: int) -> int {
        unsafe { wxDateTime_GetWeekOfMonth(self, flags, tz) }
    }
    #[fixed_stack_segment]
    fn getWeekOfYear(&self, flags: int, tz: int) -> int {
        unsafe { wxDateTime_GetWeekOfYear(self, flags, tz) }
    }
    #[fixed_stack_segment]
    fn getYear(&self, tz: int) -> int {
        unsafe { wxDateTime_GetYear(self, tz) }
    }
    #[fixed_stack_segment]
    fn isBetween(&self, t1: @wxDateTime, t2: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsBetween(self, t1, t2) }
    }
    #[fixed_stack_segment]
    fn isDST(&self, country: int) -> bool {
        unsafe { wxDateTime_IsDST(self, country) }
    }
    #[fixed_stack_segment]
    fn isDSTApplicable(year: int, country: int) -> bool {
        unsafe { wxDateTime_IsDSTApplicable(year, country) }
    }
    #[fixed_stack_segment]
    fn isEarlierThan(&self, datetime: *c_void) -> bool {
        unsafe { wxDateTime_IsEarlierThan(self, datetime) }
    }
    #[fixed_stack_segment]
    fn isEqualTo(&self, datetime: *c_void) -> bool {
        unsafe { wxDateTime_IsEqualTo(self, datetime) }
    }
    #[fixed_stack_segment]
    fn isEqualUpTo(&self, dt: @wxDateTime, ts: *c_void) -> bool {
        unsafe { wxDateTime_IsEqualUpTo(self, dt, ts) }
    }
    #[fixed_stack_segment]
    fn isGregorianDate(&self, country: int) -> bool {
        unsafe { wxDateTime_IsGregorianDate(self, country) }
    }
    #[fixed_stack_segment]
    fn isLaterThan(&self, datetime: *c_void) -> bool {
        unsafe { wxDateTime_IsLaterThan(self, datetime) }
    }
    #[fixed_stack_segment]
    fn isLeapYear(year: int, cal: int) -> bool {
        unsafe { wxDateTime_IsLeapYear(year, cal) }
    }
    #[fixed_stack_segment]
    fn isSameDate(&self, dt: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsSameDate(self, dt) }
    }
    #[fixed_stack_segment]
    fn isSameTime(&self, dt: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsSameTime(self, dt) }
    }
    #[fixed_stack_segment]
    fn isStrictlyBetween(&self, t1: @wxDateTime, t2: @wxDateTime) -> bool {
        unsafe { wxDateTime_IsStrictlyBetween(self, t1, t2) }
    }
    #[fixed_stack_segment]
    fn isValid(&self) -> bool {
        unsafe { wxDateTime_IsValid(self) }
    }
    #[fixed_stack_segment]
    fn isWestEuropeanCountry(country: int) -> bool {
        unsafe { wxDateTime_IsWestEuropeanCountry(country) }
    }
    #[fixed_stack_segment]
    fn isWorkDay(&self, country: int) -> bool {
        unsafe { wxDateTime_IsWorkDay(self, country) }
    }
    #[fixed_stack_segment]
    fn makeGMT(&self, noDST: int) {
        unsafe { wxDateTime_MakeGMT(self, noDST) }
    }
    #[fixed_stack_segment]
    fn makeTimezone(&self, tz: int, noDST: int) {
        unsafe { wxDateTime_MakeTimezone(self, tz, noDST) }
    }
    #[fixed_stack_segment]
    fn now(&self) {
        unsafe { wxDateTime_Now(self) }
    }
    #[fixed_stack_segment]
    fn parseDate(&self, date: *c_void) {
        unsafe { wxDateTime_ParseDate(self, date) }
    }
    #[fixed_stack_segment]
    fn parseDateTime(&self, datetime: *c_void) {
        unsafe { wxDateTime_ParseDateTime(self, datetime) }
    }
    #[fixed_stack_segment]
    fn parseFormat(&self, date: *c_void, format: *c_void, dateDef: *c_void) {
        unsafe { wxDateTime_ParseFormat(self, date, format, dateDef) }
    }
    #[fixed_stack_segment]
    fn parseRfc822Date(&self, date: *c_void) {
        unsafe { wxDateTime_ParseRfc822Date(self, date) }
    }
    #[fixed_stack_segment]
    fn parseTime(&self, time: @wxTime) {
        unsafe { wxDateTime_ParseTime(self, time) }
    }
    #[fixed_stack_segment]
    fn resetTime(&self) {
        unsafe { wxDateTime_ResetTime(self) }
    }
    #[fixed_stack_segment]
    fn set(&self, day: int, month: int, year: int, hour: int, minute: int, second: int, millisec: int) {
        unsafe { wxDateTime_Set(self, day, month, year, hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    fn setCountry(country: int) {
        unsafe { wxDateTime_SetCountry(country) }
    }
    #[fixed_stack_segment]
    fn setDay(&self, day: int) {
        unsafe { wxDateTime_SetDay(self, day) }
    }
    #[fixed_stack_segment]
    fn setHour(&self, hour: int) {
        unsafe { wxDateTime_SetHour(self, hour) }
    }
    #[fixed_stack_segment]
    fn setMillisecond(&self, millisecond: int) {
        unsafe { wxDateTime_SetMillisecond(self, millisecond) }
    }
    #[fixed_stack_segment]
    fn setMinute(&self, minute: int) {
        unsafe { wxDateTime_SetMinute(self, minute) }
    }
    #[fixed_stack_segment]
    fn setMonth(&self, month: int) {
        unsafe { wxDateTime_SetMonth(self, month) }
    }
    #[fixed_stack_segment]
    fn setSecond(&self, second: int) {
        unsafe { wxDateTime_SetSecond(self, second) }
    }
    #[fixed_stack_segment]
    fn setTime(&self, hour: int, minute: int, second: int, millisec: int) {
        unsafe { wxDateTime_SetTime(self, hour, minute, second, millisec) }
    }
    #[fixed_stack_segment]
    fn setToCurrent(&self) {
        unsafe { wxDateTime_SetToCurrent(self) }
    }
    #[fixed_stack_segment]
    fn setToLastMonthDay(&self, month: int, year: int) {
        unsafe { wxDateTime_SetToLastMonthDay(self, month, year) }
    }
    #[fixed_stack_segment]
    fn setToLastWeekDay(&self, weekday: int, month: int, year: int) -> bool {
        unsafe { wxDateTime_SetToLastWeekDay(self, weekday, month, year) }
    }
    #[fixed_stack_segment]
    fn setToNextWeekDay(&self, weekday: int) {
        unsafe { wxDateTime_SetToNextWeekDay(self, weekday) }
    }
    #[fixed_stack_segment]
    fn setToPrevWeekDay(&self, weekday: int) {
        unsafe { wxDateTime_SetToPrevWeekDay(self, weekday) }
    }
    #[fixed_stack_segment]
    fn setToWeekDay(&self, weekday: int, n: int, month: int, year: int) -> bool {
        unsafe { wxDateTime_SetToWeekDay(self, weekday, n, month, year) }
    }
    #[fixed_stack_segment]
    fn setToWeekDayInSameWeek(&self, weekday: int) {
        unsafe { wxDateTime_SetToWeekDayInSameWeek(self, weekday) }
    }
    #[fixed_stack_segment]
    fn setYear(&self, year: int) {
        unsafe { wxDateTime_SetYear(self, year) }
    }
    #[fixed_stack_segment]
    fn subtractDate(&self, diff: *c_void, @wxDateTime) {
        unsafe { wxDateTime_SubtractDate(self, diff, _ref) }
    }
    #[fixed_stack_segment]
    fn subtractTime(&self, diff: *c_void, @wxDateTime) {
        unsafe { wxDateTime_SubtractTime(self, diff, _ref) }
    }
    #[fixed_stack_segment]
    fn toGMT(&self, noDST: int) {
        unsafe { wxDateTime_ToGMT(self, noDST) }
    }
    #[fixed_stack_segment]
    fn toTimezone(&self, tz: int, noDST: int) {
        unsafe { wxDateTime_ToTimezone(self, tz, noDST) }
    }
    #[fixed_stack_segment]
    fn today(&self) {
        unsafe { wxDateTime_Today(self) }
    }
    #[fixed_stack_segment]
    fn uNow(&self) {
        unsafe { wxDateTime_UNow(self) }
    }
    #[fixed_stack_segment]
    fn wxDateTime(hi_long: int, lo_long: int) {
        unsafe { wxDateTime_wxDateTime(hi_long, lo_long) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDateTime_Delete(self) }
    }
}
trait wxDb {
}
trait wxDbColDef {
}
trait wxDbColFor {
}
trait wxDbColInf {
}
trait wxDbConnectInf {
}
trait wxDbInf {
}
trait wxDbSqlTypeInfo {
}
trait wxDbTable {
}
trait wxDbTableInfo {
}
trait wxDebugContext {
}
trait wxDialUpEvent {
    #[fixed_stack_segment]
    fn isConnectedEvent(&self) -> bool {
        unsafe { wxDialUpEvent_IsConnectedEvent(self) }
    }
    #[fixed_stack_segment]
    fn isOwnEvent(&self) -> bool {
        unsafe { wxDialUpEvent_IsOwnEvent(self) }
    }
}
trait wxDialUpManager {
    #[fixed_stack_segment]
    fn cancelDialing(&self) -> bool {
        unsafe { wxDialUpManager_CancelDialing(self) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxDialUpManager {
        unsafe { wxDialUpManager_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDialUpManager_Delete(self) }
    }
    #[fixed_stack_segment]
    fn dial(&self, nameOfISP: @wxString, username: @wxString, password: @wxString, async: bool) -> bool {
        unsafe { wxDialUpManager_Dial(self, nameOfISP, username, password, async) }
    }
    #[fixed_stack_segment]
    fn disableAutoCheckOnlineStatus(&self) {
        unsafe { wxDialUpManager_DisableAutoCheckOnlineStatus(self) }
    }
    #[fixed_stack_segment]
    fn enableAutoCheckOnlineStatus(&self, nSeconds: int) -> bool {
        unsafe { wxDialUpManager_EnableAutoCheckOnlineStatus(self, nSeconds) }
    }
    #[fixed_stack_segment]
    fn getISPNames(&self, _lst: @wxList) -> int {
        unsafe { wxDialUpManager_GetISPNames(self, _lst) }
    }
    #[fixed_stack_segment]
    fn hangUp(&self) -> bool {
        unsafe { wxDialUpManager_HangUp(self) }
    }
    #[fixed_stack_segment]
    fn isAlwaysOnline(&self) -> bool {
        unsafe { wxDialUpManager_IsAlwaysOnline(self) }
    }
    #[fixed_stack_segment]
    fn isDialing(&self) -> bool {
        unsafe { wxDialUpManager_IsDialing(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxDialUpManager_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn isOnline(&self) -> bool {
        unsafe { wxDialUpManager_IsOnline(self) }
    }
    #[fixed_stack_segment]
    fn setConnectCommand(&self, commandDial: @wxString, commandHangup: @wxString) {
        unsafe { wxDialUpManager_SetConnectCommand(self, commandDial, commandHangup) }
    }
    #[fixed_stack_segment]
    fn setOnlineStatus(&self, isOnline: bool) {
        unsafe { wxDialUpManager_SetOnlineStatus(self, isOnline) }
    }
    #[fixed_stack_segment]
    fn setWellKnownHost(&self, hostname: @wxString, portno: int) {
        unsafe { wxDialUpManager_SetWellKnownHost(self, hostname, portno) }
    }
}
trait wxDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxDialog {
        unsafe { wxDialog_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn endModal(&self, retCode: int) {
        unsafe { wxDialog_EndModal(self, retCode) }
    }
    #[fixed_stack_segment]
    fn getReturnCode(&self) -> int {
        unsafe { wxDialog_GetReturnCode(self) }
    }
    #[fixed_stack_segment]
    fn isModal(&self) -> bool {
        unsafe { wxDialog_IsModal(self) }
    }
    #[fixed_stack_segment]
    fn setReturnCode(&self, returnCode: int) {
        unsafe { wxDialog_SetReturnCode(self, returnCode) }
    }
    #[fixed_stack_segment]
    fn showModal(&self) -> int {
        unsafe { wxDialog_ShowModal(self) }
    }
}
trait wxDirDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _msg: @wxString, _dir: @wxString, _lft: c_int, _top: c_int, _stl: int) -> @wxDirDialog {
        unsafe { wxDirDialog_Create(_prt, _msg, _dir, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn getMessage(&self) -> @wxString {
        unsafe { wxDirDialog_GetMessage(self) }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @wxString {
        unsafe { wxDirDialog_GetPath(self) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxDirDialog_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, msg: @wxString) {
        unsafe { wxDirDialog_SetMessage(self, msg) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, pth: @wxString) {
        unsafe { wxDirDialog_SetPath(self, pth) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxDirDialog_SetStyle(self, style) }
    }
}
trait wxDirTraverser {
}
trait wxDllLoader {
}
trait wxDocChildFrame {
}
trait wxDocMDIChildFrame {
}
trait wxDocMDIParentFrame {
}
trait wxDocManager {
}
trait wxDocParentFrame {
}
trait wxDocTemplate {
}
trait wxDocument {
}
trait wxDragImage {
    #[fixed_stack_segment]
    fn new(image: @wxBitmap, x: int, y: int) -> @wxDragImage {
        unsafe { wxDragImage_Create(image, x, y) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDragImage_Delete(self) }
    }
    #[fixed_stack_segment]
    fn beginDragFullScreen(&self, x_pos: int, y_pos: int, window: @wxWindow, fullScreen: bool, rect: @wxRect) -> bool {
        unsafe { wxDragImage_BeginDragFullScreen(self, x_pos, y_pos, window, fullScreen, rect) }
    }
    #[fixed_stack_segment]
    fn beginDrag(&self, x: int, y: int, window: @wxWindow, boundingWindow: @wxWindow) -> bool {
        unsafe { wxDragImage_BeginDrag(self, x, y, window, boundingWindow) }
    }
    #[fixed_stack_segment]
    fn endDrag(&self) {
        unsafe { wxDragImage_EndDrag(self) }
    }
    #[fixed_stack_segment]
    fn hide(&self) -> bool {
        unsafe { wxDragImage_Hide(self) }
    }
    #[fixed_stack_segment]
    fn move(&self, x: int, y: int) -> bool {
        unsafe { wxDragImage_Move(self, x, y) }
    }
    #[fixed_stack_segment]
    fn show(&self) -> bool {
        unsafe { wxDragImage_Show(self) }
    }
}
trait wxDrawControl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxDrawControl {
        unsafe { wxDrawControl_Create(_prt, _id, _arg2, _stl) }
    }
}
trait wxDrawWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxDrawWindow {
        unsafe { wxDrawWindow_Create(_prt, _id, _arg2, _stl) }
    }
}
trait wxDropFilesEvent {
}
trait wxDropSource {
}
trait wxDropTarget {
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self) }
    }
    #[fixed_stack_segment]
    fn setDataObject(&self, _dat: @wxDataObject) {
        unsafe { wxDropTarget_SetDataObject(self, _dat) }
    }
}
trait wxDynToolInfo {
    #[fixed_stack_segment]
    fn index(&self) -> int {
        unsafe { wxDynToolInfo_Index(self) }
    }
    #[fixed_stack_segment]
    fn realSize(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxDynToolInfo_RealSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn pToolWnd(&self) {
        unsafe { wxDynToolInfo_pToolWnd(self) }
    }
}
trait wxDynamicLibrary {
}
trait wxDynamicSashWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxDynamicSashWindow {
        unsafe { wxDynamicSashWindow_Create(parent, id, _arg2, style) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDynamicSashWindow_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getHScrollBar(&self, child: @wxWindow) {
        unsafe { wxDynamicSashWindow_GetHScrollBar(self, child) }
    }
    #[fixed_stack_segment]
    fn getVScrollBar(&self, child: @wxWindow) {
        unsafe { wxDynamicSashWindow_GetVScrollBar(self, child) }
    }
}
trait wxDynamicToolBar {
    #[fixed_stack_segment]
    fn addSeparator(&self, pSepartorWnd: *c_void) {
        unsafe { wxDynamicToolBar_AddSeparator(self, pSepartorWnd) }
    }
    #[fixed_stack_segment]
    fn addTool(&self, toolIndex: int, pToolWindow: *c_void, w: c_int, h: c_int) {
        unsafe { wxDynamicToolBar_AddTool(self, toolIndex, pToolWindow, _arg3) }
    }
    #[fixed_stack_segment]
    fn addToolBitmap(&self, toolIndex: int, bitmap: @wxBitmap, pushedBitmap: *c_void, toggle: int, x: c_int, y: c_int, clientData: @wxClientData, helpString1: *c_void, helpString2: *c_void) {
        unsafe { wxDynamicToolBar_AddToolBitmap(self, toolIndex, bitmap, pushedBitmap, toggle, _arg5, clientData, helpString1, helpString2) }
    }
    #[fixed_stack_segment]
    fn addToolImage(&self, toolIndex: int, imageFileName: *c_void, imageFileType: int, labelText: *c_void, alignTextRight: int, isFlat: bool) {
        unsafe { wxDynamicToolBar_AddToolImage(self, toolIndex, imageFileName, imageFileType, labelText, alignTextRight, isFlat) }
    }
    #[fixed_stack_segment]
    fn addToolLabel(&self, toolIndex: int, labelBmp: *c_void, labelText: *c_void, alignTextRight: int, isFlat: bool) {
        unsafe { wxDynamicToolBar_AddToolLabel(self, toolIndex, labelBmp, labelText, alignTextRight, isFlat) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int, orientation: int, RowsOrColumns: int) -> @wxDynamicToolBar {
        unsafe { wxDynamicToolBar_Create(parent, id, _arg2, style, orientation, RowsOrColumns) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxDynamicToolBar {
        unsafe { wxDynamicToolBar_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newDefaultLayout(&self) {
        unsafe { wxDynamicToolBar_CreateDefaultLayout(self) }
    }
    #[fixed_stack_segment]
    fn newParams(&self, parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int, orientation: int, RowsOrColumns: int) -> int {
        unsafe { wxDynamicToolBar_CreateParams(self, parent, id, _arg3, style, orientation, RowsOrColumns) }
    }
    #[fixed_stack_segment]
    fn newTool(&self, id: int, label: *c_void, bmpNormal: *c_void, bmpDisabled: *c_void, kind: int, clientData: @wxClientData, shortHelp: *c_void, longHelp: *c_void) {
        unsafe { wxDynamicToolBar_CreateTool(self, id, label, bmpNormal, bmpDisabled, kind, clientData, shortHelp, longHelp) }
    }
    #[fixed_stack_segment]
    fn newToolControl(&self, control: @wxControl) {
        unsafe { wxDynamicToolBar_CreateToolControl(self, control) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDynamicToolBar_Delete(self) }
    }
    #[fixed_stack_segment]
    fn doDeleteTool(&self, pos: int, tool: *c_void) -> int {
        unsafe { wxDynamicToolBar_DoDeleteTool(self, pos, tool) }
    }
    #[fixed_stack_segment]
    fn doEnableTool(&self, tool: *c_void, enable: bool) {
        unsafe { wxDynamicToolBar_DoEnableTool(self, tool, enable) }
    }
    #[fixed_stack_segment]
    fn doInsertTool(&self, pos: int, tool: *c_void) -> int {
        unsafe { wxDynamicToolBar_DoInsertTool(self, pos, tool) }
    }
    #[fixed_stack_segment]
    fn doSetToggle(&self, tool: *c_void, toggle: int) {
        unsafe { wxDynamicToolBar_DoSetToggle(self, tool, toggle) }
    }
    #[fixed_stack_segment]
    fn doToggleTool(&self, tool: *c_void, toggle: int) {
        unsafe { wxDynamicToolBar_DoToggleTool(self, tool, toggle) }
    }
    #[fixed_stack_segment]
    fn drawSeparator(&self, info: *c_void, dc: @wxDC) {
        unsafe { wxDynamicToolBar_DrawSeparator(self, info, dc) }
    }
    #[fixed_stack_segment]
    fn enableTool(&self, toolIndex: int, enable: bool) {
        unsafe { wxDynamicToolBar_EnableTool(self, toolIndex, enable) }
    }
    #[fixed_stack_segment]
    fn findToolForPosition(&self, x: c_int, y: c_int) {
        unsafe { wxDynamicToolBar_FindToolForPosition(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getPreferredDim(&self, gw: int, gh: int, pw: *c_void, ph: *c_void) {
        unsafe { wxDynamicToolBar_GetPreferredDim(self, gw, gh, pw, ph) }
    }
    #[fixed_stack_segment]
    fn getToolInfo(&self, toolIndex: int) {
        unsafe { wxDynamicToolBar_GetToolInfo(self, toolIndex) }
    }
    #[fixed_stack_segment]
    fn layout(&self) -> int {
        unsafe { wxDynamicToolBar_Layout(self) }
    }
    #[fixed_stack_segment]
    fn removeTool(&self, toolIndex: int) {
        unsafe { wxDynamicToolBar_RemoveTool(self, toolIndex) }
    }
    #[fixed_stack_segment]
    fn setLayout(&self, pLayout: *c_void) {
        unsafe { wxDynamicToolBar_SetLayout(self, pLayout) }
    }
}
trait wxEditableListBox {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, label: *wchar_t, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxEditableListBox {
        unsafe { wxEditableListBox_Create(parent, id, label, _arg3, style) }
    }
    #[fixed_stack_segment]
    fn getDelButton(&self) {
        unsafe { wxEditableListBox_GetDelButton(self) }
    }
    #[fixed_stack_segment]
    fn getDownButton(&self) {
        unsafe { wxEditableListBox_GetDownButton(self) }
    }
    #[fixed_stack_segment]
    fn getEditButton(&self) {
        unsafe { wxEditableListBox_GetEditButton(self) }
    }
    #[fixed_stack_segment]
    fn getListCtrl(&self) -> @wxListCtrl {
        unsafe { wxEditableListBox_GetListCtrl(self) }
    }
    #[fixed_stack_segment]
    fn getNewButton(&self) {
        unsafe { wxEditableListBox_GetNewButton(self) }
    }
    #[fixed_stack_segment]
    fn getStrings(&self, _ref: **wchar_t) -> c_int {
        unsafe { wxEditableListBox_GetStrings(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getUpButton(&self) {
        unsafe { wxEditableListBox_GetUpButton(self) }
    }
    #[fixed_stack_segment]
    fn setStrings(&self, strings: *c_void, _n: int) {
        unsafe { wxEditableListBox_SetStrings(self, strings, _n) }
    }
}
trait wxEncodingConverter {
    #[fixed_stack_segment]
    fn convert(&self, input: *c_void, output: *c_void) {
        unsafe { wxEncodingConverter_Convert(self, input, output) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxEncodingConverter {
        unsafe { wxEncodingConverter_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxEncodingConverter_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getAllEquivalents(&self, enc: int, _lst: @wxList) -> int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self, enc, _lst) }
    }
    #[fixed_stack_segment]
    fn getPlatformEquivalents(&self, enc: int, platform: int, _lst: @wxList) -> int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self, enc, platform, _lst) }
    }
    #[fixed_stack_segment]
    fn init(&self, input_enc: int, output_enc: int, method: int) -> int {
        unsafe { wxEncodingConverter_Init(self, input_enc, output_enc, method) }
    }
}
trait wxEraseEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxEraseEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getDC(&self) -> @wxDC {
        unsafe { wxEraseEvent_GetDC(self) }
    }
}
trait wxEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *c_void) {
        unsafe { wxEvent_CopyObject(self, object_dest) }
    }
    #[fixed_stack_segment]
    fn getEventObject(&self) -> @wxObject {
        unsafe { wxEvent_GetEventObject(self) }
    }
    #[fixed_stack_segment]
    fn getEventType(&self) -> int {
        unsafe { wxEvent_GetEventType(self) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> int {
        unsafe { wxEvent_GetId(self) }
    }
    #[fixed_stack_segment]
    fn getSkipped(&self) -> bool {
        unsafe { wxEvent_GetSkipped(self) }
    }
    #[fixed_stack_segment]
    fn getTimestamp(&self) -> int {
        unsafe { wxEvent_GetTimestamp(self) }
    }
    #[fixed_stack_segment]
    fn isCommandEvent(&self) -> bool {
        unsafe { wxEvent_IsCommandEvent(self) }
    }
    #[fixed_stack_segment]
    fn newEventType() -> int {
        unsafe { wxEvent_NewEventType() }
    }
    #[fixed_stack_segment]
    fn setEventObject(&self, obj: @wxObject) {
        unsafe { wxEvent_SetEventObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn setEventType(&self, typ: int) {
        unsafe { wxEvent_SetEventType(self, typ) }
    }
    #[fixed_stack_segment]
    fn setId(&self, Id: int) {
        unsafe { wxEvent_SetId(self, Id) }
    }
    #[fixed_stack_segment]
    fn setTimestamp(&self, ts: int) {
        unsafe { wxEvent_SetTimestamp(self, ts) }
    }
    #[fixed_stack_segment]
    fn skip(&self) {
        unsafe { wxEvent_Skip(self) }
    }
}
trait wxEvtHandler {
    #[fixed_stack_segment]
    fn addPendingEvent(&self, event: @wxEvent) {
        unsafe { wxEvtHandler_AddPendingEvent(self, event) }
    }
    #[fixed_stack_segment]
    fn connect(&self, first: int, last: int, type_: int, data: *c_void) -> int {
        unsafe { wxEvtHandler_Connect(self, first, last, type_, data) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxEvtHandler {
        unsafe { wxEvtHandler_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxEvtHandler_Delete(self) }
    }
    #[fixed_stack_segment]
    fn disconnect(&self, first: int, last: int, type_: int, id: int) -> int {
        unsafe { wxEvtHandler_Disconnect(self, first, last, type_, id) }
    }
    #[fixed_stack_segment]
    fn getEvtHandlerEnabled(&self) -> bool {
        unsafe { wxEvtHandler_GetEvtHandlerEnabled(self) }
    }
    #[fixed_stack_segment]
    fn getNextHandler(&self) -> @wxEvtHandler {
        unsafe { wxEvtHandler_GetNextHandler(self) }
    }
    #[fixed_stack_segment]
    fn getPreviousHandler(&self) -> @wxEvtHandler {
        unsafe { wxEvtHandler_GetPreviousHandler(self) }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @wxEvent) -> bool {
        unsafe { wxEvtHandler_ProcessEvent(self, event) }
    }
    #[fixed_stack_segment]
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self) }
    }
    #[fixed_stack_segment]
    fn setEvtHandlerEnabled(&self, enabled: bool) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self, enabled) }
    }
    #[fixed_stack_segment]
    fn setNextHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxEvtHandler_SetNextHandler(self, handler) }
    }
    #[fixed_stack_segment]
    fn setPreviousHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxEvtHandler_SetPreviousHandler(self, handler) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self, id: int, type_: int) -> @wxClosure {
        unsafe { wxEvtHandler_GetClosure(self, id, type_) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { wxEvtHandler_GetClientClosure(self) }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @wxClosure) {
        unsafe { wxEvtHandler_SetClientClosure(self, closure) }
    }
}
trait wxExpr {
}
trait wxExprDatabase {
}
trait wxFFile {
}
trait wxFFileInputStream {
}
trait wxFFileOutputStream {
}
trait wxFSFile {
}
trait wxFTP {
}
trait wxFileDataObject {
}
trait wxFileDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _msg: @wxString, _dir: @wxString, _fle: @wxString, _wcd: @wxString, _lft: c_int, _top: c_int, _stl: int) -> @wxFileDialog {
        unsafe { wxFileDialog_Create(_prt, _msg, _dir, _fle, _wcd, _arg5, _stl) }
    }
    #[fixed_stack_segment]
    fn getDirectory(&self) -> @wxString {
        unsafe { wxFileDialog_GetDirectory(self) }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> @wxString {
        unsafe { wxFileDialog_GetFilename(self) }
    }
    #[fixed_stack_segment]
    fn getFilenames(&self, paths: **wchar_t) -> c_int {
        unsafe { wxFileDialog_GetFilenames(self, paths) }
    }
    #[fixed_stack_segment]
    fn getFilterIndex(&self) -> int {
        unsafe { wxFileDialog_GetFilterIndex(self) }
    }
    #[fixed_stack_segment]
    fn getMessage(&self) -> @wxString {
        unsafe { wxFileDialog_GetMessage(self) }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @wxString {
        unsafe { wxFileDialog_GetPath(self) }
    }
    #[fixed_stack_segment]
    fn getPaths(&self, paths: **wchar_t) -> c_int {
        unsafe { wxFileDialog_GetPaths(self, paths) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxFileDialog_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn getWildcard(&self) -> @wxString {
        unsafe { wxFileDialog_GetWildcard(self) }
    }
    #[fixed_stack_segment]
    fn setDirectory(&self, dir: @wxString) {
        unsafe { wxFileDialog_SetDirectory(self, dir) }
    }
    #[fixed_stack_segment]
    fn setFilename(&self, name: @wxString) {
        unsafe { wxFileDialog_SetFilename(self, name) }
    }
    #[fixed_stack_segment]
    fn setFilterIndex(&self, filterIndex: int) {
        unsafe { wxFileDialog_SetFilterIndex(self, filterIndex) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, message: @wxString) {
        unsafe { wxFileDialog_SetMessage(self, message) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, path: @wxString) {
        unsafe { wxFileDialog_SetPath(self, path) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxFileDialog_SetStyle(self, style) }
    }
    #[fixed_stack_segment]
    fn setWildcard(&self, wildCard: @wxString) {
        unsafe { wxFileDialog_SetWildcard(self, wildCard) }
    }
}
trait wxFileDropTarget {
}
trait wxFileHistory {
    #[fixed_stack_segment]
    fn addFileToHistory(&self, file: @wxString) {
        unsafe { wxFileHistory_AddFileToHistory(self, file) }
    }
    #[fixed_stack_segment]
    fn addFilesToMenu(&self, menu: @wxMenu) {
        unsafe { wxFileHistory_AddFilesToMenu(self, menu) }
    }
    #[fixed_stack_segment]
    fn new(maxFiles: int) -> @wxFileHistory {
        unsafe { wxFileHistory_Create(maxFiles) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileHistory_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxFileHistory_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getHistoryFile(&self, i: int) -> @wxString {
        unsafe { wxFileHistory_GetHistoryFile(self, i) }
    }
    #[fixed_stack_segment]
    fn getMaxFiles(&self) -> int {
        unsafe { wxFileHistory_GetMaxFiles(self) }
    }
    #[fixed_stack_segment]
    fn getMenus(&self, ~[@wxMenu]) -> c_int {
        unsafe { wxFileHistory_GetMenus(self, _ref) }
    }
    #[fixed_stack_segment]
    fn load(&self, config: @wxConfigBase) {
        unsafe { wxFileHistory_Load(self, config) }
    }
    #[fixed_stack_segment]
    fn removeFileFromHistory(&self, i: int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self, i) }
    }
    #[fixed_stack_segment]
    fn removeMenu(&self, menu: @wxMenu) {
        unsafe { wxFileHistory_RemoveMenu(self, menu) }
    }
    #[fixed_stack_segment]
    fn save(&self, config: @wxConfigBase) {
        unsafe { wxFileHistory_Save(self, config) }
    }
    #[fixed_stack_segment]
    fn useMenu(&self, menu: @wxMenu) {
        unsafe { wxFileHistory_UseMenu(self, menu) }
    }
}
trait wxFileInputStream {
}
trait wxFileName {
}
trait wxFileOutputStream {
}
trait wxFileSystem {
}
trait wxFileSystemHandler {
}
trait wxFileType {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self) }
    }
    #[fixed_stack_segment]
    fn expandCommand(&self, _cmd: *c_void, _params: *c_void) -> @wxString {
        unsafe { wxFileType_ExpandCommand(self, _cmd, _params) }
    }
    #[fixed_stack_segment]
    fn getDescription(&self) -> @wxString {
        unsafe { wxFileType_GetDescription(self) }
    }
    #[fixed_stack_segment]
    fn getExtensions(&self, _lst: @wxList) -> int {
        unsafe { wxFileType_GetExtensions(self, _lst) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, icon: @wxIcon) -> int {
        unsafe { wxFileType_GetIcon(self, icon) }
    }
    #[fixed_stack_segment]
    fn getMimeType(&self) -> @wxString {
        unsafe { wxFileType_GetMimeType(self) }
    }
    #[fixed_stack_segment]
    fn getMimeTypes(&self, _lst: @wxList) -> int {
        unsafe { wxFileType_GetMimeTypes(self, _lst) }
    }
    #[fixed_stack_segment]
    fn getOpenCommand(&self, _buf: *c_void, _params: *c_void) -> int {
        unsafe { wxFileType_GetOpenCommand(self, _buf, _params) }
    }
    #[fixed_stack_segment]
    fn getPrintCommand(&self, _buf: *c_void, _params: *c_void) -> int {
        unsafe { wxFileType_GetPrintCommand(self, _buf, _params) }
    }
}
trait wxFilterInputStream {
}
trait wxFilterOutputStream {
}
trait wxFindDialogEvent {
    #[fixed_stack_segment]
    fn getFindString(&self, _ref: *c_void) -> int {
        unsafe { wxFindDialogEvent_GetFindString(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> int {
        unsafe { wxFindDialogEvent_GetFlags(self) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self, _ref: *c_void) -> int {
        unsafe { wxFindDialogEvent_GetReplaceString(self, _ref) }
    }
}
trait wxFindReplaceData {
    #[fixed_stack_segment]
    fn new(flags: int) -> @wxFindReplaceData {
        unsafe { wxFindReplaceData_Create(flags) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxFindReplaceData {
        unsafe { wxFindReplaceData_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFindReplaceData_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getFindString(&self) -> @wxString {
        unsafe { wxFindReplaceData_GetFindString(self) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> int {
        unsafe { wxFindReplaceData_GetFlags(self) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self) -> @wxString {
        unsafe { wxFindReplaceData_GetReplaceString(self) }
    }
    #[fixed_stack_segment]
    fn setFindString(&self, str: @wxString) {
        unsafe { wxFindReplaceData_SetFindString(self, str) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: int) {
        unsafe { wxFindReplaceData_SetFlags(self, flags) }
    }
    #[fixed_stack_segment]
    fn setReplaceString(&self, str: @wxString) {
        unsafe { wxFindReplaceData_SetReplaceString(self, str) }
    }
}
trait wxFindReplaceDialog {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, data: @wxFindReplaceData, title: @wxString, style: int) -> @wxFindReplaceDialog {
        unsafe { wxFindReplaceDialog_Create(parent, data, title, style) }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> @wxFindReplaceData {
        unsafe { wxFindReplaceDialog_GetData(self) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @wxFindReplaceData) {
        unsafe { wxFindReplaceDialog_SetData(self, data) }
    }
}
trait wxFlexGridSizer {
    #[fixed_stack_segment]
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self, idx) }
    }
    #[fixed_stack_segment]
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self, idx) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { wxFlexGridSizer_CalcMin(self) }
    }
    #[fixed_stack_segment]
    fn new(rows: int, cols: int, vgap: int, hgap: int) -> @wxFlexGridSizer {
        unsafe { wxFlexGridSizer_Create(rows, cols, vgap, hgap) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxFlexGridSizer_RecalcSizes(self) }
    }
    #[fixed_stack_segment]
    fn removeGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableCol(self, idx) }
    }
    #[fixed_stack_segment]
    fn removeGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_RemoveGrowableRow(self, idx) }
    }
}
trait wxFocusEvent {
}
trait wxFont {
    #[fixed_stack_segment]
    fn new(pointSize: int, family: int, style: int, weight: int, underlined: bool, face: @wxString, enc: int) -> @wxFont {
        unsafe { wxFont_Create(pointSize, family, style, weight, underlined, face, enc) }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: int) -> @wxFont {
        unsafe { wxFont_CreateFromStock(id) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxFont {
        unsafe { wxFont_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFont_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultEncoding(&self) -> int {
        unsafe { wxFont_GetDefaultEncoding(self) }
    }
    #[fixed_stack_segment]
    fn getEncoding(&self) -> int {
        unsafe { wxFont_GetEncoding(self) }
    }
    #[fixed_stack_segment]
    fn getFaceName(&self) -> @wxString {
        unsafe { wxFont_GetFaceName(self) }
    }
    #[fixed_stack_segment]
    fn getFamily(&self) -> int {
        unsafe { wxFont_GetFamily(self) }
    }
    #[fixed_stack_segment]
    fn getFamilyString(&self) -> @wxString {
        unsafe { wxFont_GetFamilyString(self) }
    }
    #[fixed_stack_segment]
    fn getPointSize(&self) -> int {
        unsafe { wxFont_GetPointSize(self) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxFont_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn getStyleString(&self) -> @wxString {
        unsafe { wxFont_GetStyleString(self) }
    }
    #[fixed_stack_segment]
    fn getUnderlined(&self) -> int {
        unsafe { wxFont_GetUnderlined(self) }
    }
    #[fixed_stack_segment]
    fn getWeight(&self) -> int {
        unsafe { wxFont_GetWeight(self) }
    }
    #[fixed_stack_segment]
    fn getWeightString(&self) -> @wxString {
        unsafe { wxFont_GetWeightString(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxFont_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn setDefaultEncoding(&self, encoding: int) {
        unsafe { wxFont_SetDefaultEncoding(self, encoding) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: int) {
        unsafe { wxFont_SetEncoding(self, encoding) }
    }
    #[fixed_stack_segment]
    fn setFaceName(&self, faceName: @wxString) {
        unsafe { wxFont_SetFaceName(self, faceName) }
    }
    #[fixed_stack_segment]
    fn setFamily(&self, family: int) {
        unsafe { wxFont_SetFamily(self, family) }
    }
    #[fixed_stack_segment]
    fn setPointSize(&self, pointSize: int) {
        unsafe { wxFont_SetPointSize(self, pointSize) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxFont_SetStyle(self, style) }
    }
    #[fixed_stack_segment]
    fn setUnderlined(&self, underlined: int) {
        unsafe { wxFont_SetUnderlined(self, underlined) }
    }
    #[fixed_stack_segment]
    fn setWeight(&self, weight: int) {
        unsafe { wxFont_SetWeight(self, weight) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxFont_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxFont_IsStatic(self) }
    }
}
trait wxFontData {
    #[fixed_stack_segment]
    fn new() -> @wxFontData {
        unsafe { wxFontData_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontData_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enableEffects(&self, flag: bool) {
        unsafe { wxFontData_EnableEffects(self, flag) }
    }
    #[fixed_stack_segment]
    fn getAllowSymbols(&self) -> bool {
        unsafe { wxFontData_GetAllowSymbols(self) }
    }
    #[fixed_stack_segment]
    fn getChosenFont(&self, @wxFont) {
        unsafe { wxFontData_GetChosenFont(self, ref_) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, @wxColour) {
        unsafe { wxFontData_GetColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getEnableEffects(&self) -> bool {
        unsafe { wxFontData_GetEnableEffects(self) }
    }
    #[fixed_stack_segment]
    fn getEncoding(&self) -> int {
        unsafe { wxFontData_GetEncoding(self) }
    }
    #[fixed_stack_segment]
    fn getInitialFont(&self, @wxFont) {
        unsafe { wxFontData_GetInitialFont(self, ref_) }
    }
    #[fixed_stack_segment]
    fn getShowHelp(&self) -> int {
        unsafe { wxFontData_GetShowHelp(self) }
    }
    #[fixed_stack_segment]
    fn setAllowSymbols(&self, flag: bool) {
        unsafe { wxFontData_SetAllowSymbols(self, flag) }
    }
    #[fixed_stack_segment]
    fn setChosenFont(&self, font: @wxFont) {
        unsafe { wxFontData_SetChosenFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @wxColour) {
        unsafe { wxFontData_SetColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: int) {
        unsafe { wxFontData_SetEncoding(self, encoding) }
    }
    #[fixed_stack_segment]
    fn setInitialFont(&self, font: @wxFont) {
        unsafe { wxFontData_SetInitialFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, minRange: int, maxRange: int) {
        unsafe { wxFontData_SetRange(self, minRange, maxRange) }
    }
    #[fixed_stack_segment]
    fn setShowHelp(&self, flag: bool) {
        unsafe { wxFontData_SetShowHelp(self, flag) }
    }
}
trait wxFontDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, fnt: @wxFontData) -> @wxFontDialog {
        unsafe { wxFontDialog_Create(_prt, fnt) }
    }
    #[fixed_stack_segment]
    fn getFontData(&self, @wxFontData) {
        unsafe { wxFontDialog_GetFontData(self, _ref) }
    }
}
trait wxFontEnumerator {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _fnc: *c_void) -> @wxFontEnumerator {
        unsafe { wxFontEnumerator_Create(_obj, _fnc) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enumerateEncodings(&self, facename: @wxString) -> bool {
        unsafe { wxFontEnumerator_EnumerateEncodings(self, facename) }
    }
    #[fixed_stack_segment]
    fn enumerateFacenames(&self, encoding: int, fixedWidthOnly: int) -> bool {
        unsafe { wxFontEnumerator_EnumerateFacenames(self, encoding, fixedWidthOnly) }
    }
}
trait wxFontList {
}
trait wxFontMapper {
    #[fixed_stack_segment]
    fn new() -> @wxFontMapper {
        unsafe { wxFontMapper_Create() }
    }
    #[fixed_stack_segment]
    fn getAltForEncoding(&self, encoding: int, alt_encoding: *c_void, _buf: @wxString) -> bool {
        unsafe { wxFontMapper_GetAltForEncoding(self, encoding, alt_encoding, _buf) }
    }
    #[fixed_stack_segment]
    fn isEncodingAvailable(&self, encoding: int, _buf: @wxString) -> bool {
        unsafe { wxFontMapper_IsEncodingAvailable(self, encoding, _buf) }
    }
}
trait wxFrame {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxFrame {
        unsafe { wxFrame_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn newStatusBar(&self, number: int, style: int) -> @wxStatusBar {
        unsafe { wxFrame_CreateStatusBar(self, number, style) }
    }
    #[fixed_stack_segment]
    fn newToolBar(&self, style: c_long) -> @wxToolBar {
        unsafe { wxFrame_CreateToolBar(self, style) }
    }
    #[fixed_stack_segment]
    fn getClientAreaOrigin_left(&self) -> int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self) }
    }
    #[fixed_stack_segment]
    fn getClientAreaOrigin_top(&self) -> int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> @wxMenuBar {
        unsafe { wxFrame_GetMenuBar(self) }
    }
    #[fixed_stack_segment]
    fn getStatusBar(&self) -> @wxStatusBar {
        unsafe { wxFrame_GetStatusBar(self) }
    }
    #[fixed_stack_segment]
    fn getToolBar(&self) -> @wxToolBar {
        unsafe { wxFrame_GetToolBar(self) }
    }
    #[fixed_stack_segment]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self) }
    }
    #[fixed_stack_segment]
    fn setMenuBar(&self, menubar: @wxMenuBar) {
        unsafe { wxFrame_SetMenuBar(self, menubar) }
    }
    #[fixed_stack_segment]
    fn setStatusBar(&self, statBar: @wxStatusBar) {
        unsafe { wxFrame_SetStatusBar(self, statBar) }
    }
    #[fixed_stack_segment]
    fn setStatusText(&self, _txt: @wxString, _number: int) {
        unsafe { wxFrame_SetStatusText(self, _txt, _number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, _n: int, _widths_field: *c_void) {
        unsafe { wxFrame_SetStatusWidths(self, _n, _widths_field) }
    }
    #[fixed_stack_segment]
    fn setToolBar(&self, _toolbar: @wxToolBar) {
        unsafe { wxFrame_SetToolBar(self, _toolbar) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { wxFrame_GetTitle(self) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, _txt: @wxString) {
        unsafe { wxFrame_SetTitle(self, _txt) }
    }
    #[fixed_stack_segment]
    fn setShape(&self, region: @wxRegion) -> bool {
        unsafe { wxFrame_SetShape(self, region) }
    }
    #[fixed_stack_segment]
    fn showFullScreen(&self, show: bool, style: int) -> bool {
        unsafe { wxFrame_ShowFullScreen(self, show, style) }
    }
    #[fixed_stack_segment]
    fn isFullScreen(&self) -> bool {
        unsafe { wxFrame_IsFullScreen(self) }
    }
    #[fixed_stack_segment]
    fn centre(&self, orientation: int) {
        unsafe { wxFrame_Centre(self, orientation) }
    }
}
trait wxFrameLayout {
    #[fixed_stack_segment]
    fn activate(&self) {
        unsafe { wxFrameLayout_Activate(self) }
    }
    #[fixed_stack_segment]
    fn addBar(&self, pBarWnd: *c_void, dimInfo: *c_void, alignment: int, rowNo: int, columnPos: int, name: *wchar_t, spyEvents: int, state: int) {
        unsafe { wxFrameLayout_AddBar(self, pBarWnd, dimInfo, alignment, rowNo, columnPos, name, spyEvents, state) }
    }
    #[fixed_stack_segment]
    fn addPlugin(&self, pPlInfo: *c_void, paneMask: int) {
        unsafe { wxFrameLayout_AddPlugin(self, pPlInfo, paneMask) }
    }
    #[fixed_stack_segment]
    fn addPluginBefore(&self, pNextPlInfo: *c_void, pPlInfo: *c_void, paneMask: int) {
        unsafe { wxFrameLayout_AddPluginBefore(self, pNextPlInfo, pPlInfo, paneMask) }
    }
    #[fixed_stack_segment]
    fn applyBarProperties(&self, pBar: *c_void) {
        unsafe { wxFrameLayout_ApplyBarProperties(self, pBar) }
    }
    #[fixed_stack_segment]
    fn captureEventsForPane(&self, toPane: *c_void) {
        unsafe { wxFrameLayout_CaptureEventsForPane(self, toPane) }
    }
    #[fixed_stack_segment]
    fn captureEventsForPlugin(&self, pPlugin: *c_void) {
        unsafe { wxFrameLayout_CaptureEventsForPlugin(self, pPlugin) }
    }
    #[fixed_stack_segment]
    fn new(pParentFrame: *c_void, pFrameClient: *c_void, activateNow: int) -> @wxFrameLayout {
        unsafe { wxFrameLayout_Create(pParentFrame, pFrameClient, activateNow) }
    }
    #[fixed_stack_segment]
    fn deactivate(&self) {
        unsafe { wxFrameLayout_Deactivate(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFrameLayout_Delete(self) }
    }
    #[fixed_stack_segment]
    fn destroyBarWindows(&self) {
        unsafe { wxFrameLayout_DestroyBarWindows(self) }
    }
    #[fixed_stack_segment]
    fn enableFloating(&self, enable: bool) {
        unsafe { wxFrameLayout_EnableFloating(self, enable) }
    }
    #[fixed_stack_segment]
    fn findBarByName(&self, name: *wchar_t) {
        unsafe { wxFrameLayout_FindBarByName(self, name) }
    }
    #[fixed_stack_segment]
    fn findBarByWindow(&self, pWnd: *c_void) {
        unsafe { wxFrameLayout_FindBarByWindow(self, pWnd) }
    }
    #[fixed_stack_segment]
    fn findPlugin(&self, pPlInfo: *c_void) {
        unsafe { wxFrameLayout_FindPlugin(self, pPlInfo) }
    }
    #[fixed_stack_segment]
    fn firePluginEvent(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_FirePluginEvent(self, event) }
    }
    #[fixed_stack_segment]
    fn getBars(&self, _ref: *c_void) -> int {
        unsafe { wxFrameLayout_GetBars(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getClientHeight(&self) -> int {
        unsafe { wxFrameLayout_GetClientHeight(self) }
    }
    #[fixed_stack_segment]
    fn getClientRect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxFrameLayout_GetClientRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getClientWidth(&self) -> int {
        unsafe { wxFrameLayout_GetClientWidth(self) }
    }
    #[fixed_stack_segment]
    fn getFrameClient(&self) {
        unsafe { wxFrameLayout_GetFrameClient(self) }
    }
    #[fixed_stack_segment]
    fn getPane(&self, alignment: int) {
        unsafe { wxFrameLayout_GetPane(self, alignment) }
    }
    #[fixed_stack_segment]
    fn getPaneProperties(&self, props: *c_void, alignment: int) {
        unsafe { wxFrameLayout_GetPaneProperties(self, props, alignment) }
    }
    #[fixed_stack_segment]
    fn getParentFrame(&self) {
        unsafe { wxFrameLayout_GetParentFrame(self) }
    }
    #[fixed_stack_segment]
    fn getTopPlugin(&self) {
        unsafe { wxFrameLayout_GetTopPlugin(self) }
    }
    #[fixed_stack_segment]
    fn getUpdatesManager(&self) {
        unsafe { wxFrameLayout_GetUpdatesManager(self) }
    }
    #[fixed_stack_segment]
    fn hasTopPlugin(&self) -> bool {
        unsafe { wxFrameLayout_HasTopPlugin(self) }
    }
    #[fixed_stack_segment]
    fn hideBarWindows(&self) {
        unsafe { wxFrameLayout_HideBarWindows(self) }
    }
    #[fixed_stack_segment]
    fn inverseVisibility(&self, pBar: *c_void) {
        unsafe { wxFrameLayout_InverseVisibility(self, pBar) }
    }
    #[fixed_stack_segment]
    fn onLButtonDown(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnLButtonDown(self, event) }
    }
    #[fixed_stack_segment]
    fn onLButtonUp(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnLButtonUp(self, event) }
    }
    #[fixed_stack_segment]
    fn onLDblClick(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnLDblClick(self, event) }
    }
    #[fixed_stack_segment]
    fn onMouseMove(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnMouseMove(self, event) }
    }
    #[fixed_stack_segment]
    fn onRButtonDown(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnRButtonDown(self, event) }
    }
    #[fixed_stack_segment]
    fn onRButtonUp(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnRButtonUp(self, event) }
    }
    #[fixed_stack_segment]
    fn onSize(&self, event: @wxEvent) {
        unsafe { wxFrameLayout_OnSize(self, event) }
    }
    #[fixed_stack_segment]
    fn popAllPlugins(&self) {
        unsafe { wxFrameLayout_PopAllPlugins(self) }
    }
    #[fixed_stack_segment]
    fn popPlugin(&self) {
        unsafe { wxFrameLayout_PopPlugin(self) }
    }
    #[fixed_stack_segment]
    fn pushDefaultPlugins(&self) {
        unsafe { wxFrameLayout_PushDefaultPlugins(self) }
    }
    #[fixed_stack_segment]
    fn pushPlugin(&self, pPugin: *c_void) {
        unsafe { wxFrameLayout_PushPlugin(self, pPugin) }
    }
    #[fixed_stack_segment]
    fn recalcLayout(&self, repositionBarsNow: int) {
        unsafe { wxFrameLayout_RecalcLayout(self, repositionBarsNow) }
    }
    #[fixed_stack_segment]
    fn redockBar(&self, pBar: *c_void, x: c_int, y: c_int, w: c_int, h: c_int, pToPane: *c_void, updateNow: int) -> int {
        unsafe { wxFrameLayout_RedockBar(self, pBar, _arg2, pToPane, updateNow) }
    }
    #[fixed_stack_segment]
    fn refreshNow(&self, recalcLayout: int) {
        unsafe { wxFrameLayout_RefreshNow(self, recalcLayout) }
    }
    #[fixed_stack_segment]
    fn releaseEventsFromPane(&self, fromPane: *c_void) {
        unsafe { wxFrameLayout_ReleaseEventsFromPane(self, fromPane) }
    }
    #[fixed_stack_segment]
    fn releaseEventsFromPlugin(&self, pPlugin: *c_void) {
        unsafe { wxFrameLayout_ReleaseEventsFromPlugin(self, pPlugin) }
    }
    #[fixed_stack_segment]
    fn removeBar(&self, pBar: *c_void) {
        unsafe { wxFrameLayout_RemoveBar(self, pBar) }
    }
    #[fixed_stack_segment]
    fn removePlugin(&self, pPlInfo: *c_void) {
        unsafe { wxFrameLayout_RemovePlugin(self, pPlInfo) }
    }
    #[fixed_stack_segment]
    fn setBarState(&self, pBar: *c_void, newStatem: int, updateNow: int) {
        unsafe { wxFrameLayout_SetBarState(self, pBar, newStatem, updateNow) }
    }
    #[fixed_stack_segment]
    fn setFrameClient(&self, pFrameClient: *c_void) {
        unsafe { wxFrameLayout_SetFrameClient(self, pFrameClient) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, top: int, bottom: int, left: int, right: int, paneMask: int) {
        unsafe { wxFrameLayout_SetMargins(self, top, bottom, left, right, paneMask) }
    }
    #[fixed_stack_segment]
    fn setPaneBackground(&self, colour: @wxColour) {
        unsafe { wxFrameLayout_SetPaneBackground(self, colour) }
    }
    #[fixed_stack_segment]
    fn setPaneProperties(&self, props: *c_void, paneMask: int) {
        unsafe { wxFrameLayout_SetPaneProperties(self, props, paneMask) }
    }
    #[fixed_stack_segment]
    fn setTopPlugin(&self, pPlugin: *c_void) {
        unsafe { wxFrameLayout_SetTopPlugin(self, pPlugin) }
    }
    #[fixed_stack_segment]
    fn setUpdatesManager(&self, pUMgr: *c_void) {
        unsafe { wxFrameLayout_SetUpdatesManager(self, pUMgr) }
    }
}
trait wxGDIObject {
}
trait wxGLCanvas {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, windowID: int, attributes: *int, x: c_int, y: c_int, w: c_int, h: c_int, style: int, title: @wxString, palette: @wxPalette) -> @wxGLCanvas {
        unsafe { wxGLCanvas_Create(parent, windowID, attributes, _arg3, style, title, palette) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @wxColour) -> bool {
        unsafe { wxGLCanvas_SetColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, ctxt: @wxGLContext) -> bool {
        unsafe { wxGLCanvas_SetCurrent(self, ctxt) }
    }
    #[fixed_stack_segment]
    fn swapBuffers(&self) -> bool {
        unsafe { wxGLCanvas_SwapBuffers(self) }
    }
    #[fixed_stack_segment]
    fn isDisplaySupported(attributes: *int) -> bool {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    #[fixed_stack_segment]
    fn isExtensionSupported(extension: @wxString) -> bool {
        unsafe { wxGLCanvas_IsExtensionSupported(extension) }
    }
}
trait wxGauge {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _rng: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxGauge {
        unsafe { wxGauge_Create(_prt, _id, _rng, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn getBezelFace(&self) -> int {
        unsafe { wxGauge_GetBezelFace(self) }
    }
    #[fixed_stack_segment]
    fn getRange(&self) -> int {
        unsafe { wxGauge_GetRange(self) }
    }
    #[fixed_stack_segment]
    fn getShadowWidth(&self) -> int {
        unsafe { wxGauge_GetShadowWidth(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> int {
        unsafe { wxGauge_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setBezelFace(&self, w: int) {
        unsafe { wxGauge_SetBezelFace(self, w) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, r: int) {
        unsafe { wxGauge_SetRange(self, r) }
    }
    #[fixed_stack_segment]
    fn setShadowWidth(&self, w: int) {
        unsafe { wxGauge_SetShadowWidth(self, w) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, pos: int) {
        unsafe { wxGauge_SetValue(self, pos) }
    }
}
trait wxGenericDirCtrl {
}
trait wxGenericValidator {
}
trait wxGrid {
    #[fixed_stack_segment]
    fn appendCols(&self, numCols: int, updateLabels: bool) -> bool {
        unsafe { wxGrid_AppendCols(self, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    fn appendRows(&self, numRows: int, updateLabels: bool) -> bool {
        unsafe { wxGrid_AppendRows(self, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    fn autoSize(&self) {
        unsafe { wxGrid_AutoSize(self) }
    }
    #[fixed_stack_segment]
    fn autoSizeColumn(&self, col: int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumn(self, col, setAsMin) }
    }
    #[fixed_stack_segment]
    fn autoSizeColumns(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumns(self, setAsMin) }
    }
    #[fixed_stack_segment]
    fn autoSizeRow(&self, row: int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRow(self, row, setAsMin) }
    }
    #[fixed_stack_segment]
    fn autoSizeRows(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRows(self, setAsMin) }
    }
    #[fixed_stack_segment]
    fn beginBatch(&self) {
        unsafe { wxGrid_BeginBatch(self) }
    }
    #[fixed_stack_segment]
    fn blockToDeviceRect(&self, top: int, left: int, bottom: int, right: int) -> @wxRect {
        unsafe { wxGrid_BlockToDeviceRect(self, top, left, bottom, right) }
    }
    #[fixed_stack_segment]
    fn canDragColSize(&self) -> bool {
        unsafe { wxGrid_CanDragColSize(self) }
    }
    #[fixed_stack_segment]
    fn canDragGridSize(&self) -> bool {
        unsafe { wxGrid_CanDragGridSize(self) }
    }
    #[fixed_stack_segment]
    fn canDragRowSize(&self) -> bool {
        unsafe { wxGrid_CanDragRowSize(self) }
    }
    #[fixed_stack_segment]
    fn canEnableCellControl(&self) -> bool {
        unsafe { wxGrid_CanEnableCellControl(self) }
    }
    #[fixed_stack_segment]
    fn cellToRect(&self, row: int, col: int) -> @wxRect {
        unsafe { wxGrid_CellToRect(self, row, col) }
    }
    #[fixed_stack_segment]
    fn clearGrid(&self) {
        unsafe { wxGrid_ClearGrid(self) }
    }
    #[fixed_stack_segment]
    fn clearSelection(&self) {
        unsafe { wxGrid_ClearSelection(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxGrid {
        unsafe { wxGrid_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn newGrid(&self, rows: int, cols: int, selmode: int) {
        unsafe { wxGrid_CreateGrid(self, rows, cols, selmode) }
    }
    #[fixed_stack_segment]
    fn deleteCols(&self, pos: int, numCols: int, updateLabels: bool) -> bool {
        unsafe { wxGrid_DeleteCols(self, pos, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    fn deleteRows(&self, pos: int, numRows: int, updateLabels: bool) -> bool {
        unsafe { wxGrid_DeleteRows(self, pos, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    fn disableCellEditControl(&self) {
        unsafe { wxGrid_DisableCellEditControl(self) }
    }
    #[fixed_stack_segment]
    fn disableDragColSize(&self) {
        unsafe { wxGrid_DisableDragColSize(self) }
    }
    #[fixed_stack_segment]
    fn disableDragGridSize(&self) {
        unsafe { wxGrid_DisableDragGridSize(self) }
    }
    #[fixed_stack_segment]
    fn disableDragRowSize(&self) {
        unsafe { wxGrid_DisableDragRowSize(self) }
    }
    #[fixed_stack_segment]
    fn drawAllGridLines(&self, dc: @wxDC, reg: @wxRegion) {
        unsafe { wxGrid_DrawAllGridLines(self, dc, reg) }
    }
    #[fixed_stack_segment]
    fn drawCell(&self, dc: @wxDC, _row: int, _col: int) {
        unsafe { wxGrid_DrawCell(self, dc, _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellBorder(&self, dc: @wxDC, _row: int, _col: int) {
        unsafe { wxGrid_DrawCellBorder(self, dc, _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellHighlight(&self, dc: @wxDC, attr: @wxGridCellAttr) {
        unsafe { wxGrid_DrawCellHighlight(self, dc, attr) }
    }
    #[fixed_stack_segment]
    fn drawColLabel(&self, dc: @wxDC, col: int) {
        unsafe { wxGrid_DrawColLabel(self, dc, col) }
    }
    #[fixed_stack_segment]
    fn drawColLabels(&self, dc: @wxDC) {
        unsafe { wxGrid_DrawColLabels(self, dc) }
    }
    #[fixed_stack_segment]
    fn drawGridSpace(&self, dc: @wxDC) {
        unsafe { wxGrid_DrawGridSpace(self, dc) }
    }
    #[fixed_stack_segment]
    fn drawRowLabel(&self, dc: @wxDC, row: int) {
        unsafe { wxGrid_DrawRowLabel(self, dc, row) }
    }
    #[fixed_stack_segment]
    fn drawRowLabels(&self, dc: @wxDC) {
        unsafe { wxGrid_DrawRowLabels(self, dc) }
    }
    #[fixed_stack_segment]
    fn drawTextRectangle(&self, dc: @wxDC, txt: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: int, verticalAlignment: int) {
        unsafe { wxGrid_DrawTextRectangle(self, dc, txt, _arg3, horizontalAlignment, verticalAlignment) }
    }
    #[fixed_stack_segment]
    fn enableCellEditControl(&self, enable: bool) {
        unsafe { wxGrid_EnableCellEditControl(self, enable) }
    }
    #[fixed_stack_segment]
    fn enableDragColSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragColSize(self, enable) }
    }
    #[fixed_stack_segment]
    fn enableDragGridSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragGridSize(self, enable) }
    }
    #[fixed_stack_segment]
    fn enableDragRowSize(&self, enable: bool) {
        unsafe { wxGrid_EnableDragRowSize(self, enable) }
    }
    #[fixed_stack_segment]
    fn enableEditing(&self, edit: c_int) {
        unsafe { wxGrid_EnableEditing(self, edit) }
    }
    #[fixed_stack_segment]
    fn enableGridLines(&self, enable: bool) {
        unsafe { wxGrid_EnableGridLines(self, enable) }
    }
    #[fixed_stack_segment]
    fn endBatch(&self) {
        unsafe { wxGrid_EndBatch(self) }
    }
    #[fixed_stack_segment]
    fn getBatchCount(&self) -> int {
        unsafe { wxGrid_GetBatchCount(self) }
    }
    #[fixed_stack_segment]
    fn getCellAlignment(&self, row: int, col: int, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetCellAlignment(self, row, col, _arg3) }
    }
    #[fixed_stack_segment]
    fn getCellBackgroundColour(&self, row: int, col: int, colour: @wxColour) {
        unsafe { wxGrid_GetCellBackgroundColour(self, row, col, colour) }
    }
    #[fixed_stack_segment]
    fn getCellEditor(&self, row: int, col: int) -> @wxGridCellEditor {
        unsafe { wxGrid_GetCellEditor(self, row, col) }
    }
    #[fixed_stack_segment]
    fn getCellFont(&self, row: int, col: int, font: @wxFont) {
        unsafe { wxGrid_GetCellFont(self, row, col, font) }
    }
    #[fixed_stack_segment]
    fn getCellHighlightColour(&self, @wxColour) {
        unsafe { wxGrid_GetCellHighlightColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getCellRenderer(&self, row: int, col: int) -> @wxGridCellRenderer {
        unsafe { wxGrid_GetCellRenderer(self, row, col) }
    }
    #[fixed_stack_segment]
    fn getCellTextColour(&self, row: int, col: int, colour: @wxColour) {
        unsafe { wxGrid_GetCellTextColour(self, row, col, colour) }
    }
    #[fixed_stack_segment]
    fn getCellValue(&self, row: int, col: int) -> @wxString {
        unsafe { wxGrid_GetCellValue(self, row, col) }
    }
    #[fixed_stack_segment]
    fn getColLabelAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetColLabelAlignment(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getColLabelSize(&self) -> int {
        unsafe { wxGrid_GetColLabelSize(self) }
    }
    #[fixed_stack_segment]
    fn getColLabelValue(&self, col: int) -> @wxString {
        unsafe { wxGrid_GetColLabelValue(self, col) }
    }
    #[fixed_stack_segment]
    fn getColSize(&self, col: int) -> int {
        unsafe { wxGrid_GetColSize(self, col) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetDefaultCellAlignment(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellBackgroundColour(&self, @wxColour) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellFont(&self, @wxFont) {
        unsafe { wxGrid_GetDefaultCellFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellTextColour(&self, @wxColour) {
        unsafe { wxGrid_GetDefaultCellTextColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getDefaultColLabelSize(&self) -> int {
        unsafe { wxGrid_GetDefaultColLabelSize(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultColSize(&self) -> int {
        unsafe { wxGrid_GetDefaultColSize(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultEditor(&self) -> @wxGridCellEditor {
        unsafe { wxGrid_GetDefaultEditor(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForCell(&self, row: int, col: int) -> @wxGridCellEditor {
        unsafe { wxGrid_GetDefaultEditorForCell(self, row, col) }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForType(&self, typeName: @wxString) -> @wxGridCellEditor {
        unsafe { wxGrid_GetDefaultEditorForType(self, typeName) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @wxGridCellRenderer {
        unsafe { wxGrid_GetDefaultRenderer(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForCell(&self, row: int, col: int) -> @wxGridCellRenderer {
        unsafe { wxGrid_GetDefaultRendererForCell(self, row, col) }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForType(&self, typeName: @wxString) -> @wxGridCellRenderer {
        unsafe { wxGrid_GetDefaultRendererForType(self, typeName) }
    }
    #[fixed_stack_segment]
    fn getDefaultRowLabelSize(&self) -> int {
        unsafe { wxGrid_GetDefaultRowLabelSize(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultRowSize(&self) -> int {
        unsafe { wxGrid_GetDefaultRowSize(self) }
    }
    #[fixed_stack_segment]
    fn getGridCursorCol(&self) -> int {
        unsafe { wxGrid_GetGridCursorCol(self) }
    }
    #[fixed_stack_segment]
    fn getGridCursorRow(&self) -> int {
        unsafe { wxGrid_GetGridCursorRow(self) }
    }
    #[fixed_stack_segment]
    fn getGridLineColour(&self, @wxColour) {
        unsafe { wxGrid_GetGridLineColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getLabelBackgroundColour(&self, @wxColour) {
        unsafe { wxGrid_GetLabelBackgroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getLabelFont(&self, @wxFont) {
        unsafe { wxGrid_GetLabelFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getLabelTextColour(&self, @wxColour) {
        unsafe { wxGrid_GetLabelTextColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getNumberCols(&self) -> int {
        unsafe { wxGrid_GetNumberCols(self) }
    }
    #[fixed_stack_segment]
    fn getNumberRows(&self) -> int {
        unsafe { wxGrid_GetNumberRows(self) }
    }
    #[fixed_stack_segment]
    fn getRowLabelAlignment(&self, horiz: *c_int, vert: *c_int) {
        unsafe { wxGrid_GetRowLabelAlignment(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getRowLabelSize(&self) -> int {
        unsafe { wxGrid_GetRowLabelSize(self) }
    }
    #[fixed_stack_segment]
    fn getRowLabelValue(&self, row: int) -> @wxString {
        unsafe { wxGrid_GetRowLabelValue(self, row) }
    }
    #[fixed_stack_segment]
    fn getRowSize(&self, row: int) -> int {
        unsafe { wxGrid_GetRowSize(self, row) }
    }
    #[fixed_stack_segment]
    fn getSelectionBackground(&self, @wxColour) {
        unsafe { wxGrid_GetSelectionBackground(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getSelectionForeground(&self, @wxColour) {
        unsafe { wxGrid_GetSelectionForeground(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getTable(&self) -> @wxGridTableBase {
        unsafe { wxGrid_GetTable(self) }
    }
    #[fixed_stack_segment]
    fn getTextBoxSize(&self, dc: @wxDC, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int) {
        unsafe { wxGrid_GetTextBoxSize(self, dc, _arg2, _arg3) }
    }
    #[fixed_stack_segment]
    fn gridLinesEnabled(&self) -> int {
        unsafe { wxGrid_GridLinesEnabled(self) }
    }
    #[fixed_stack_segment]
    fn hideCellEditControl(&self) {
        unsafe { wxGrid_HideCellEditControl(self) }
    }
    #[fixed_stack_segment]
    fn insertCols(&self, pos: int, numCols: int, updateLabels: bool) -> bool {
        unsafe { wxGrid_InsertCols(self, pos, numCols, updateLabels) }
    }
    #[fixed_stack_segment]
    fn insertRows(&self, pos: int, numRows: int, updateLabels: bool) -> bool {
        unsafe { wxGrid_InsertRows(self, pos, numRows, updateLabels) }
    }
    #[fixed_stack_segment]
    fn isCellEditControlEnabled(&self) -> bool {
        unsafe { wxGrid_IsCellEditControlEnabled(self) }
    }
    #[fixed_stack_segment]
    fn isCellEditControlShown(&self) -> bool {
        unsafe { wxGrid_IsCellEditControlShown(self) }
    }
    #[fixed_stack_segment]
    fn isCurrentCellReadOnly(&self) -> bool {
        unsafe { wxGrid_IsCurrentCellReadOnly(self) }
    }
    #[fixed_stack_segment]
    fn isEditable(&self) -> bool {
        unsafe { wxGrid_IsEditable(self) }
    }
    #[fixed_stack_segment]
    fn isInSelection(&self, row: int, col: int) -> bool {
        unsafe { wxGrid_IsInSelection(self, row, col) }
    }
    #[fixed_stack_segment]
    fn isReadOnly(&self, row: int, col: int) -> bool {
        unsafe { wxGrid_IsReadOnly(self, row, col) }
    }
    #[fixed_stack_segment]
    fn isSelection(&self) -> bool {
        unsafe { wxGrid_IsSelection(self) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self, row: int, col: int, wholeCellVisible: bool) -> bool {
        unsafe { wxGrid_IsVisible(self, row, col, wholeCellVisible) }
    }
    #[fixed_stack_segment]
    fn makeCellVisible(&self, row: int, col: int) {
        unsafe { wxGrid_MakeCellVisible(self, row, col) }
    }
    #[fixed_stack_segment]
    fn moveCursorDown(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorDown(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorDownBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorDownBlock(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorLeft(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorLeft(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorLeftBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorLeftBlock(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorRight(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorRight(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorRightBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorRightBlock(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorUp(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorUp(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn moveCursorUpBlock(&self, expandSelection: bool) -> bool {
        unsafe { wxGrid_MoveCursorUpBlock(self, expandSelection) }
    }
    #[fixed_stack_segment]
    fn movePageDown(&self) -> bool {
        unsafe { wxGrid_MovePageDown(self) }
    }
    #[fixed_stack_segment]
    fn movePageUp(&self) -> bool {
        unsafe { wxGrid_MovePageUp(self) }
    }
    #[fixed_stack_segment]
    fn processTableMessage(&self, evt: @wxEvent) -> bool {
        unsafe { wxGrid_ProcessTableMessage(self, evt) }
    }
    #[fixed_stack_segment]
    fn registerDataType(&self, typeName: @wxString, renderer: @wxGridCellRenderer, editor: @wxGridCellEditor) {
        unsafe { wxGrid_RegisterDataType(self, typeName, renderer, editor) }
    }
    #[fixed_stack_segment]
    fn saveEditControlValue(&self) {
        unsafe { wxGrid_SaveEditControlValue(self) }
    }
    #[fixed_stack_segment]
    fn selectAll(&self) {
        unsafe { wxGrid_SelectAll(self) }
    }
    #[fixed_stack_segment]
    fn selectBlock(&self, topRow: int, leftCol: int, bottomRow: int, rightCol: int, addToSelected: c_int) {
        unsafe { wxGrid_SelectBlock(self, topRow, leftCol, bottomRow, rightCol, addToSelected) }
    }
    #[fixed_stack_segment]
    fn selectCol(&self, col: int, addToSelected: c_int) {
        unsafe { wxGrid_SelectCol(self, col, addToSelected) }
    }
    #[fixed_stack_segment]
    fn selectRow(&self, row: int, addToSelected: c_int) {
        unsafe { wxGrid_SelectRow(self, row, addToSelected) }
    }
    #[fixed_stack_segment]
    fn setCellAlignment(&self, row: int, col: int, horiz: int, vert: int) {
        unsafe { wxGrid_SetCellAlignment(self, row, col, horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setCellBackgroundColour(&self, row: int, col: int, colour: @wxColour) {
        unsafe { wxGrid_SetCellBackgroundColour(self, row, col, colour) }
    }
    #[fixed_stack_segment]
    fn setCellEditor(&self, row: int, col: int, editor: @wxGridCellEditor) {
        unsafe { wxGrid_SetCellEditor(self, row, col, editor) }
    }
    #[fixed_stack_segment]
    fn setCellFont(&self, row: int, col: int, font: @wxFont) {
        unsafe { wxGrid_SetCellFont(self, row, col, font) }
    }
    #[fixed_stack_segment]
    fn setCellHighlightColour(&self, col: @wxColour) {
        unsafe { wxGrid_SetCellHighlightColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setCellRenderer(&self, row: int, col: int, renderer: @wxGridCellRenderer) {
        unsafe { wxGrid_SetCellRenderer(self, row, col, renderer) }
    }
    #[fixed_stack_segment]
    fn setCellTextColour(&self, row: int, col: int, colour: @wxColour) {
        unsafe { wxGrid_SetCellTextColour(self, row, col, colour) }
    }
    #[fixed_stack_segment]
    fn setCellValue(&self, row: int, col: int, s: @wxString) {
        unsafe { wxGrid_SetCellValue(self, row, col, s) }
    }
    #[fixed_stack_segment]
    fn setColAttr(&self, col: int, attr: @wxGridCellAttr) {
        unsafe { wxGrid_SetColAttr(self, col, attr) }
    }
    #[fixed_stack_segment]
    fn setColFormatBool(&self, col: int) {
        unsafe { wxGrid_SetColFormatBool(self, col) }
    }
    #[fixed_stack_segment]
    fn setColFormatCustom(&self, col: int, typeName: @wxString) {
        unsafe { wxGrid_SetColFormatCustom(self, col, typeName) }
    }
    #[fixed_stack_segment]
    fn setColFormatFloat(&self, col: int, width: int, precision: int) {
        unsafe { wxGrid_SetColFormatFloat(self, col, width, precision) }
    }
    #[fixed_stack_segment]
    fn setColFormatNumber(&self, col: int) {
        unsafe { wxGrid_SetColFormatNumber(self, col) }
    }
    #[fixed_stack_segment]
    fn setColLabelAlignment(&self, horiz: int, vert: int) {
        unsafe { wxGrid_SetColLabelAlignment(self, horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setColLabelSize(&self, height: int) {
        unsafe { wxGrid_SetColLabelSize(self, height) }
    }
    #[fixed_stack_segment]
    fn setColLabelValue(&self, col: int, label: @wxString) {
        unsafe { wxGrid_SetColLabelValue(self, col, label) }
    }
    #[fixed_stack_segment]
    fn setColMinimalWidth(&self, col: int, width: int) {
        unsafe { wxGrid_SetColMinimalWidth(self, col, width) }
    }
    #[fixed_stack_segment]
    fn setColSize(&self, col: int, width: int) {
        unsafe { wxGrid_SetColSize(self, col, width) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellAlignment(&self, horiz: int, vert: int) {
        unsafe { wxGrid_SetDefaultCellAlignment(self, horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellFont(&self, font: @wxFont) {
        unsafe { wxGrid_SetDefaultCellFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellTextColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetDefaultCellTextColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setDefaultColSize(&self, width: int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self, width, resizeExistingCols) }
    }
    #[fixed_stack_segment]
    fn setDefaultEditor(&self, editor: @wxGridCellEditor) {
        unsafe { wxGrid_SetDefaultEditor(self, editor) }
    }
    #[fixed_stack_segment]
    fn setDefaultRenderer(&self, renderer: @wxGridCellRenderer) {
        unsafe { wxGrid_SetDefaultRenderer(self, renderer) }
    }
    #[fixed_stack_segment]
    fn setDefaultRowSize(&self, height: int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self, height, resizeExistingRows) }
    }
    #[fixed_stack_segment]
    fn setGridCursor(&self, row: int, col: int) {
        unsafe { wxGrid_SetGridCursor(self, row, col) }
    }
    #[fixed_stack_segment]
    fn setGridLineColour(&self, col: @wxColour) {
        unsafe { wxGrid_SetGridLineColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setLabelBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetLabelBackgroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setLabelFont(&self, font: @wxFont) {
        unsafe { wxGrid_SetLabelFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setLabelTextColour(&self, colour: @wxColour) {
        unsafe { wxGrid_SetLabelTextColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, extraWidth: int, extraHeight: int) {
        unsafe { wxGrid_SetMargins(self, extraWidth, extraHeight) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, row: int, col: int, isReadOnly: bool) {
        unsafe { wxGrid_SetReadOnly(self, row, col, isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRowAttr(&self, row: int, attr: @wxGridCellAttr) {
        unsafe { wxGrid_SetRowAttr(self, row, attr) }
    }
    #[fixed_stack_segment]
    fn setRowLabelAlignment(&self, horiz: int, vert: int) {
        unsafe { wxGrid_SetRowLabelAlignment(self, horiz, vert) }
    }
    #[fixed_stack_segment]
    fn setRowLabelSize(&self, width: int) {
        unsafe { wxGrid_SetRowLabelSize(self, width) }
    }
    #[fixed_stack_segment]
    fn setRowLabelValue(&self, row: int, label: @wxString) {
        unsafe { wxGrid_SetRowLabelValue(self, row, label) }
    }
    #[fixed_stack_segment]
    fn setRowMinimalHeight(&self, row: int, width: int) {
        unsafe { wxGrid_SetRowMinimalHeight(self, row, width) }
    }
    #[fixed_stack_segment]
    fn setRowSize(&self, row: int, height: int) {
        unsafe { wxGrid_SetRowSize(self, row, height) }
    }
    #[fixed_stack_segment]
    fn setSelectionBackground(&self, c: @wxColour) {
        unsafe { wxGrid_SetSelectionBackground(self, c) }
    }
    #[fixed_stack_segment]
    fn setSelectionForeground(&self, c: @wxColour) {
        unsafe { wxGrid_SetSelectionForeground(self, c) }
    }
    #[fixed_stack_segment]
    fn setSelectionMode(&self, selmode: int) {
        unsafe { wxGrid_SetSelectionMode(self, selmode) }
    }
    #[fixed_stack_segment]
    fn setTable(&self, table: @wxGridTableBase, takeOwnership: bool, selmode: int) -> bool {
        unsafe { wxGrid_SetTable(self, table, takeOwnership, selmode) }
    }
    #[fixed_stack_segment]
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self) }
    }
    #[fixed_stack_segment]
    fn stringToLines(&self, value: @wxString, lines: *c_void) -> int {
        unsafe { wxGrid_StringToLines(self, value, lines) }
    }
    #[fixed_stack_segment]
    fn xToCol(&self, x: int) -> int {
        unsafe { wxGrid_XToCol(self, x) }
    }
    #[fixed_stack_segment]
    fn xToEdgeOfCol(&self, x: int) -> int {
        unsafe { wxGrid_XToEdgeOfCol(self, x) }
    }
    #[fixed_stack_segment]
    fn xYToCell(&self, x: c_int, y: c_int, row: *c_int, col: *c_int) {
        unsafe { wxGrid_XYToCell(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn yToEdgeOfRow(&self, y: int) -> int {
        unsafe { wxGrid_YToEdgeOfRow(self, y) }
    }
    #[fixed_stack_segment]
    fn yToRow(&self, y: int) -> int {
        unsafe { wxGrid_YToRow(self, y) }
    }
    #[fixed_stack_segment]
    fn getSelectedCells(&self, @wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectedCells(self, _arr) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockTopLeft(&self, @wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self, _arr) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockBottomRight(&self, @wxGridCellCoordsArray) {
        unsafe { wxGrid_GetSelectionBlockBottomRight(self, _arr) }
    }
    #[fixed_stack_segment]
    fn getSelectedRows(&self, _arr: *intptr_t) -> c_int {
        unsafe { wxGrid_GetSelectedRows(self, _arr) }
    }
    #[fixed_stack_segment]
    fn getSelectedCols(&self, _arr: *intptr_t) -> c_int {
        unsafe { wxGrid_GetSelectedCols(self, _arr) }
    }
    #[fixed_stack_segment]
    fn getCellSize(&self, row: int, col: int, srow: *c_int, scol: *c_int) {
        unsafe { wxGrid_GetCellSize(self, row, col, _arg3) }
    }
    #[fixed_stack_segment]
    fn setCellSize(&self, row: int, col: int, srow: c_int, scol: c_int) {
        unsafe { wxGrid_SetCellSize(self, row, col, _arg3) }
    }
}
trait wxGridCellAttr {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellAttr {
        unsafe { wxGridCellAttr_Ctor() }
    }
    #[fixed_stack_segment]
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self, hAlign: *c_int, vAlign: *c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, @wxColour) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getEditor(&self, grid: @wxGrid, row: int, col: int) -> @wxGridCellEditor {
        unsafe { wxGridCellAttr_GetEditor(self, grid, row, col) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, @wxFont) {
        unsafe { wxGridCellAttr_GetFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getRenderer(&self, grid: @wxGrid, row: int, col: int) -> @wxGridCellRenderer {
        unsafe { wxGridCellAttr_GetRenderer(self, grid, row, col) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, @wxColour) {
        unsafe { wxGridCellAttr_GetTextColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn hasAlignment(&self) -> bool {
        unsafe { wxGridCellAttr_HasAlignment(self) }
    }
    #[fixed_stack_segment]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxGridCellAttr_HasBackgroundColour(self) }
    }
    #[fixed_stack_segment]
    fn hasEditor(&self) -> bool {
        unsafe { wxGridCellAttr_HasEditor(self) }
    }
    #[fixed_stack_segment]
    fn hasFont(&self) -> bool {
        unsafe { wxGridCellAttr_HasFont(self) }
    }
    #[fixed_stack_segment]
    fn hasRenderer(&self) -> bool {
        unsafe { wxGridCellAttr_HasRenderer(self) }
    }
    #[fixed_stack_segment]
    fn hasTextColour(&self) -> bool {
        unsafe { wxGridCellAttr_HasTextColour(self) }
    }
    #[fixed_stack_segment]
    fn incRef(&self) {
        unsafe { wxGridCellAttr_IncRef(self) }
    }
    #[fixed_stack_segment]
    fn isReadOnly(&self) -> bool {
        unsafe { wxGridCellAttr_IsReadOnly(self) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, hAlign: int, vAlign: int) {
        unsafe { wxGridCellAttr_SetAlignment(self, hAlign, vAlign) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colBack: @wxColour) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self, colBack) }
    }
    #[fixed_stack_segment]
    fn setDefAttr(&self, defAttr: @wxGridCellAttr) {
        unsafe { wxGridCellAttr_SetDefAttr(self, defAttr) }
    }
    #[fixed_stack_segment]
    fn setEditor(&self, editor: @wxGridCellEditor) {
        unsafe { wxGridCellAttr_SetEditor(self, editor) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxGridCellAttr_SetFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, isReadOnly: bool) {
        unsafe { wxGridCellAttr_SetReadOnly(self, isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRenderer(&self, renderer: @wxGridCellRenderer) {
        unsafe { wxGridCellAttr_SetRenderer(self, renderer) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @wxColour) {
        unsafe { wxGridCellAttr_SetTextColour(self, colText) }
    }
}
trait wxGridCellBoolEditor {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellBoolEditor {
        unsafe { wxGridCellBoolEditor_Ctor() }
    }
}
trait wxGridCellBoolRenderer {
}
trait wxGridCellChoiceEditor {
    #[fixed_stack_segment]
    fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> @wxGridCellChoiceEditor {
        unsafe { wxGridCellChoiceEditor_Ctor(_arg0, allowOthers) }
    }
}
trait wxGridCellCoordsArray {
    #[fixed_stack_segment]
    fn new() -> @wxGridCellCoordsArray {
        unsafe { wxGridCellCoordsArray_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGridCellCoordsArray_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxGridCellCoordsArray_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn item(&self, _idx: int, _c: *c_int, _r: *c_int) {
        unsafe { wxGridCellCoordsArray_Item(self, _idx, _arg2) }
    }
}
trait wxGridCellEditor {
    #[fixed_stack_segment]
    fn beginEdit(&self, row: int, col: int, grid: @wxGrid) {
        unsafe { wxGridCellEditor_BeginEdit(self, row, col, grid) }
    }
    #[fixed_stack_segment]
    fn new(&self, parent: @wxWindow, id: int, evtHandler: @wxEvtHandler) {
        unsafe { wxGridCellEditor_Create(self, parent, id, evtHandler) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self) }
    }
    #[fixed_stack_segment]
    fn endEdit(&self, row: int, col: int, grid: @wxGrid, oldStr: @wxString, newStr: @wxString) -> int {
        unsafe { wxGridCellEditor_EndEdit(self, row, col, grid, oldStr, newStr) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @wxControl {
        unsafe { wxGridCellEditor_GetControl(self) }
    }
    #[fixed_stack_segment]
    fn handleReturn(&self, event: @wxEvent) {
        unsafe { wxGridCellEditor_HandleReturn(self, event) }
    }
    #[fixed_stack_segment]
    fn isAcceptedKey(&self, event: @wxEvent) -> bool {
        unsafe { wxGridCellEditor_IsAcceptedKey(self, event) }
    }
    #[fixed_stack_segment]
    fn isCreated(&self) -> bool {
        unsafe { wxGridCellEditor_IsCreated(self) }
    }
    #[fixed_stack_segment]
    fn paintBackground(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: @wxGridCellAttr) {
        unsafe { wxGridCellEditor_PaintBackground(self, _arg1, attr) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self) }
    }
    #[fixed_stack_segment]
    fn setControl(&self, control: @wxControl) {
        unsafe { wxGridCellEditor_SetControl(self, control) }
    }
    #[fixed_stack_segment]
    fn setParameters(&self, params: @wxString) {
        unsafe { wxGridCellEditor_SetParameters(self, params) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn show(&self, show: c_int, attr: @wxGridCellAttr) {
        unsafe { wxGridCellEditor_Show(self, show, attr) }
    }
    #[fixed_stack_segment]
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self) }
    }
    #[fixed_stack_segment]
    fn startingKey(&self, event: @wxEvent) {
        unsafe { wxGridCellEditor_StartingKey(self, event) }
    }
}
trait wxGridCellFloatEditor {
    #[fixed_stack_segment]
    fn ctor(width: int, precision: int) -> @wxGridCellFloatEditor {
        unsafe { wxGridCellFloatEditor_Ctor(width, precision) }
    }
}
trait wxGridCellFloatRenderer {
}
trait wxGridCellNumberEditor {
    #[fixed_stack_segment]
    fn ctor(min: int, max: int) -> @wxGridCellNumberEditor {
        unsafe { wxGridCellNumberEditor_Ctor(min, max) }
    }
}
trait wxGridCellNumberRenderer {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellNumberRenderer {
        unsafe { wxGridCellNumberRenderer_Ctor() }
    }
}
trait wxGridCellAutoWrapStringRenderer {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellAutoWrapStringRenderer {
        unsafe { wxGridCellAutoWrapStringRenderer_Ctor() }
    }
}
trait wxGridCellRenderer {
}
trait wxGridCellStringRenderer {
}
trait wxGridCellTextEditor {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellTextEditor {
        unsafe { wxGridCellTextEditor_Ctor() }
    }
}
trait wxGridCellWorker {
}
trait wxGridEditorCreatedEvent {
    #[fixed_stack_segment]
    fn getCol(&self) -> int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @wxControl {
        unsafe { wxGridEditorCreatedEvent_GetControl(self) }
    }
    #[fixed_stack_segment]
    fn getRow(&self) -> int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self) }
    }
    #[fixed_stack_segment]
    fn setCol(&self, col: int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self, col) }
    }
    #[fixed_stack_segment]
    fn setControl(&self, ctrl: @wxControl) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self, ctrl) }
    }
    #[fixed_stack_segment]
    fn setRow(&self, row: int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self, row) }
    }
}
trait wxGridEvent {
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxGridEvent_AltDown(self) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxGridEvent_ControlDown(self) }
    }
    #[fixed_stack_segment]
    fn getCol(&self) -> int {
        unsafe { wxGridEvent_GetCol(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxGridEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getRow(&self) -> int {
        unsafe { wxGridEvent_GetRow(self) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxGridEvent_MetaDown(self) }
    }
    #[fixed_stack_segment]
    fn selecting(&self) -> bool {
        unsafe { wxGridEvent_Selecting(self) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridEvent_ShiftDown(self) }
    }
}
trait wxGridRangeSelectEvent {
    #[fixed_stack_segment]
    fn getTopLeftCoords(&self, col: *c_int, row: *c_int) {
        unsafe { wxGridRangeSelectEvent_GetTopLeftCoords(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getBottomRightCoords(&self, col: *c_int, row: *c_int) {
        unsafe { wxGridRangeSelectEvent_GetBottomRightCoords(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getTopRow(&self) -> int {
        unsafe { wxGridRangeSelectEvent_GetTopRow(self) }
    }
    #[fixed_stack_segment]
    fn getBottomRow(&self) -> int {
        unsafe { wxGridRangeSelectEvent_GetBottomRow(self) }
    }
    #[fixed_stack_segment]
    fn getLeftCol(&self) -> int {
        unsafe { wxGridRangeSelectEvent_GetLeftCol(self) }
    }
    #[fixed_stack_segment]
    fn getRightCol(&self) -> int {
        unsafe { wxGridRangeSelectEvent_GetRightCol(self) }
    }
    #[fixed_stack_segment]
    fn selecting(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_Selecting(self) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_ControlDown(self) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_MetaDown(self) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_ShiftDown(self) }
    }
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxGridRangeSelectEvent_AltDown(self) }
    }
}
trait wxGridSizeEvent {
    #[fixed_stack_segment]
    fn getRowOrCol(&self) -> int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxGridSizeEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxGridSizeEvent_ControlDown(self) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxGridSizeEvent_MetaDown(self) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxGridSizeEvent_ShiftDown(self) }
    }
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxGridSizeEvent_AltDown(self) }
    }
}
trait wxGridSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { wxGridSizer_CalcMin(self) }
    }
    #[fixed_stack_segment]
    fn new(rows: int, cols: int, vgap: int, hgap: int) -> @wxGridSizer {
        unsafe { wxGridSizer_Create(rows, cols, vgap, hgap) }
    }
    #[fixed_stack_segment]
    fn getCols(&self) -> int {
        unsafe { wxGridSizer_GetCols(self) }
    }
    #[fixed_stack_segment]
    fn getHGap(&self) -> int {
        unsafe { wxGridSizer_GetHGap(self) }
    }
    #[fixed_stack_segment]
    fn getRows(&self) -> int {
        unsafe { wxGridSizer_GetRows(self) }
    }
    #[fixed_stack_segment]
    fn getVGap(&self) -> int {
        unsafe { wxGridSizer_GetVGap(self) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxGridSizer_RecalcSizes(self) }
    }
    #[fixed_stack_segment]
    fn setCols(&self, cols: int) {
        unsafe { wxGridSizer_SetCols(self, cols) }
    }
    #[fixed_stack_segment]
    fn setHGap(&self, gap: int) {
        unsafe { wxGridSizer_SetHGap(self, gap) }
    }
    #[fixed_stack_segment]
    fn setRows(&self, rows: int) {
        unsafe { wxGridSizer_SetRows(self, rows) }
    }
    #[fixed_stack_segment]
    fn setVGap(&self, gap: int) {
        unsafe { wxGridSizer_SetVGap(self, gap) }
    }
}
trait wxGridTableBase {
}
trait wxHTTP {
}
trait wxHashMap {
}
trait wxHelpController {
}
trait wxHelpControllerBase {
}
trait wxHelpControllerHelpProvider {
    #[fixed_stack_segment]
    fn new(ctr: @wxHelpControllerBase) -> @wxHelpControllerHelpProvider {
        unsafe { wxHelpControllerHelpProvider_Create(ctr) }
    }
    #[fixed_stack_segment]
    fn getHelpController(&self) -> @wxHelpControllerBase {
        unsafe { wxHelpControllerHelpProvider_GetHelpController(self) }
    }
    #[fixed_stack_segment]
    fn setHelpController(&self, hc: @wxHelpController) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self, hc) }
    }
}
trait wxHelpEvent {
    #[fixed_stack_segment]
    fn getLink(&self) -> @wxString {
        unsafe { wxHelpEvent_GetLink(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxHelpEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @wxString {
        unsafe { wxHelpEvent_GetTarget(self) }
    }
    #[fixed_stack_segment]
    fn setLink(&self, link: @wxString) {
        unsafe { wxHelpEvent_SetLink(self, link) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setTarget(&self, target: @wxString) {
        unsafe { wxHelpEvent_SetTarget(self, target) }
    }
}
trait wxHelpProvider {
    #[fixed_stack_segment]
    fn addHelp(&self, window: @wxWindow, text: @wxString) {
        unsafe { wxHelpProvider_AddHelp(self, window, text) }
    }
    #[fixed_stack_segment]
    fn addHelpById(&self, id: int, text: @wxString) {
        unsafe { wxHelpProvider_AddHelpById(self, id, text) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self) }
    }
    #[fixed_stack_segment]
    fn get() -> @wxHelpProvider {
        unsafe { wxHelpProvider_Get() }
    }
    #[fixed_stack_segment]
    fn getHelp(&self, window: @wxWindow) -> @wxString {
        unsafe { wxHelpProvider_GetHelp(self, window) }
    }
    #[fixed_stack_segment]
    fn removeHelp(&self, window: @wxWindow) {
        unsafe { wxHelpProvider_RemoveHelp(self, window) }
    }
    #[fixed_stack_segment]
    fn set(&self) -> @wxHelpProvider {
        unsafe { wxHelpProvider_Set(self) }
    }
    #[fixed_stack_segment]
    fn showHelp(&self, window: @wxWindow) -> bool {
        unsafe { wxHelpProvider_ShowHelp(self, window) }
    }
}
trait wxHtmlCell {
}
trait wxHtmlColourCell {
}
trait wxHtmlContainerCell {
}
trait wxHtmlDCRenderer {
}
trait wxHtmlEasyPrinting {
}
trait wxHtmlFilter {
}
trait wxHtmlHelpController {
    #[fixed_stack_segment]
    fn addBook(&self, book: *c_void, show_wait_msg: int) -> bool {
        unsafe { wxHtmlHelpController_AddBook(self, book, show_wait_msg) }
    }
    #[fixed_stack_segment]
    fn new(_style: int) -> @wxHtmlHelpController {
        unsafe { wxHtmlHelpController_Create(_style) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHtmlHelpController_Delete(self) }
    }
    #[fixed_stack_segment]
    fn display(&self, x: *c_void) -> int {
        unsafe { wxHtmlHelpController_Display(self, x) }
    }
    #[fixed_stack_segment]
    fn displayBlock(&self, blockNo: int) -> bool {
        unsafe { wxHtmlHelpController_DisplayBlock(self, blockNo) }
    }
    #[fixed_stack_segment]
    fn displayContents(&self) -> int {
        unsafe { wxHtmlHelpController_DisplayContents(self) }
    }
    #[fixed_stack_segment]
    fn displayIndex(&self) -> int {
        unsafe { wxHtmlHelpController_DisplayIndex(self) }
    }
    #[fixed_stack_segment]
    fn displayNumber(&self, id: int) -> int {
        unsafe { wxHtmlHelpController_DisplayNumber(self, id) }
    }
    #[fixed_stack_segment]
    fn displaySection(&self, section: @wxString) -> bool {
        unsafe { wxHtmlHelpController_DisplaySection(self, section) }
    }
    #[fixed_stack_segment]
    fn displaySectionNumber(&self, sectionNo: int) -> bool {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self, sectionNo) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { wxHtmlHelpController_GetFrame(self) }
    }
    #[fixed_stack_segment]
    fn getFrameParameters(&self, title: *c_void, width: *int, height: *int, pos_x: *int, pos_y: *int, newFrameEachTime: *int) {
        unsafe { wxHtmlHelpController_GetFrameParameters(self, title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, file: @wxString) -> bool {
        unsafe { wxHtmlHelpController_Initialize(self, file) }
    }
    #[fixed_stack_segment]
    fn keywordSearch(&self, keyword: @wxString) -> bool {
        unsafe { wxHtmlHelpController_KeywordSearch(self, keyword) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, file: @wxString) -> bool {
        unsafe { wxHtmlHelpController_LoadFile(self, file) }
    }
    #[fixed_stack_segment]
    fn quit(&self) -> bool {
        unsafe { wxHtmlHelpController_Quit(self) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlHelpController_ReadCustomization(self, cfg, path) }
    }
    #[fixed_stack_segment]
    fn setFrameParameters(&self, title: *c_void, width: c_int, height: c_int, pos_x: int, pos_y: int, newFrameEachTime: bool) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self, title, _arg2, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn setTempDir(&self, path: @wxString) {
        unsafe { wxHtmlHelpController_SetTempDir(self, path) }
    }
    #[fixed_stack_segment]
    fn setTitleFormat(&self, format: *c_void) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self, format) }
    }
    #[fixed_stack_segment]
    fn setViewer(&self, viewer: @wxString, flags: int) {
        unsafe { wxHtmlHelpController_SetViewer(self, viewer, flags) }
    }
    #[fixed_stack_segment]
    fn useConfig(&self, config: @wxConfigBase, rootpath: @wxString) {
        unsafe { wxHtmlHelpController_UseConfig(self, config, rootpath) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlHelpController_WriteCustomization(self, cfg, path) }
    }
}
trait wxHtmlHelpData {
}
trait wxHtmlHelpFrame {
}
trait wxHtmlLinkInfo {
}
trait wxHtmlParser {
}
trait wxHtmlPrintout {
}
trait wxHtmlTag {
}
trait wxHtmlTagHandler {
}
trait wxHtmlTagsModule {
}
trait wxHtmlWidgetCell {
}
trait wxHtmlWinParser {
}
trait wxHtmlWinTagHandler {
}
trait wxHtmlWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int, _txt: @wxString) -> @wxHtmlWindow {
        unsafe { wxHtmlWindow_Create(_prt, _id, _arg2, _stl, _txt) }
    }
    #[fixed_stack_segment]
    fn appendToPage(&self, source: @wxString) -> bool {
        unsafe { wxHtmlWindow_AppendToPage(self, source) }
    }
    #[fixed_stack_segment]
    fn getInternalRepresentation(&self) -> @wxHtmlContainerCell {
        unsafe { wxHtmlWindow_GetInternalRepresentation(self) }
    }
    #[fixed_stack_segment]
    fn getOpenedAnchor(&self) -> @wxString {
        unsafe { wxHtmlWindow_GetOpenedAnchor(self) }
    }
    #[fixed_stack_segment]
    fn getOpenedPage(&self) -> @wxString {
        unsafe { wxHtmlWindow_GetOpenedPage(self) }
    }
    #[fixed_stack_segment]
    fn getOpenedPageTitle(&self) -> @wxString {
        unsafe { wxHtmlWindow_GetOpenedPageTitle(self) }
    }
    #[fixed_stack_segment]
    fn getRelatedFrame(&self) -> @wxFrame {
        unsafe { wxHtmlWindow_GetRelatedFrame(self) }
    }
    #[fixed_stack_segment]
    fn historyBack(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryBack(self) }
    }
    #[fixed_stack_segment]
    fn historyCanBack(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryCanBack(self) }
    }
    #[fixed_stack_segment]
    fn historyCanForward(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryCanForward(self) }
    }
    #[fixed_stack_segment]
    fn historyClear(&self) {
        unsafe { wxHtmlWindow_HistoryClear(self) }
    }
    #[fixed_stack_segment]
    fn historyForward(&self) -> bool {
        unsafe { wxHtmlWindow_HistoryForward(self) }
    }
    #[fixed_stack_segment]
    fn loadPage(&self, location: @wxString) -> bool {
        unsafe { wxHtmlWindow_LoadPage(self, location) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlWindow_ReadCustomization(self, cfg, path) }
    }
    #[fixed_stack_segment]
    fn setBorders(&self, b: int) {
        unsafe { wxHtmlWindow_SetBorders(self, b) }
    }
    #[fixed_stack_segment]
    fn setFonts(&self, normal_face: @wxString, fixed_face: @wxString, sizes: *int) {
        unsafe { wxHtmlWindow_SetFonts(self, normal_face, fixed_face, sizes) }
    }
    #[fixed_stack_segment]
    fn setPage(&self, source: @wxString) {
        unsafe { wxHtmlWindow_SetPage(self, source) }
    }
    #[fixed_stack_segment]
    fn setRelatedFrame(&self, frame: @wxFrame, format: @wxString) {
        unsafe { wxHtmlWindow_SetRelatedFrame(self, frame, format) }
    }
    #[fixed_stack_segment]
    fn setRelatedStatusBar(&self, bar: int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self, bar) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @wxConfigBase, path: @wxString) {
        unsafe { wxHtmlWindow_WriteCustomization(self, cfg, path) }
    }
}
trait wxIPV4address {
}
trait wxIcon {
    #[fixed_stack_segment]
    fn assign(&self, other: *c_void) {
        unsafe { wxIcon_Assign(self, other) }
    }
    #[fixed_stack_segment]
    fn copyFromBitmap(&self, bmp: @wxBitmap) {
        unsafe { wxIcon_CopyFromBitmap(self, bmp) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxIcon {
        unsafe { wxIcon_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newLoad(name: @wxString, type_: c_long, width: c_int, height: c_int) -> @wxIcon {
        unsafe { wxIcon_CreateLoad(name, type_, _arg2) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIcon_Delete(self) }
    }
    #[fixed_stack_segment]
    fn fromRaw(&self, width: c_int, height: c_int) -> @wxIcon {
        unsafe { wxIcon_FromRaw(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn fromXPM(&self) -> @wxIcon {
        unsafe { wxIcon_FromXPM(self) }
    }
    #[fixed_stack_segment]
    fn getDepth(&self) -> int {
        unsafe { wxIcon_GetDepth(self) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> int {
        unsafe { wxIcon_GetHeight(self) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxIcon_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, other: @wxIcon) -> bool {
        unsafe { wxIcon_IsEqual(self, other) }
    }
    #[fixed_stack_segment]
    fn load(&self, name: @wxString, type_: c_long, width: c_int, height: c_int) -> int {
        unsafe { wxIcon_Load(self, name, type_, _arg3) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxIcon_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn setDepth(&self, depth: int) {
        unsafe { wxIcon_SetDepth(self, depth) }
    }
    #[fixed_stack_segment]
    fn setHeight(&self, height: int) {
        unsafe { wxIcon_SetHeight(self, height) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: int) {
        unsafe { wxIcon_SetWidth(self, width) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxIcon_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxIcon_IsStatic(self) }
    }
}
trait wxIconBundle {
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @wxIcon) {
        unsafe { wxIconBundle_AddIcon(self, icon) }
    }
    #[fixed_stack_segment]
    fn addIconFromFile(&self, file: @wxString, type_: int) {
        unsafe { wxIconBundle_AddIconFromFile(self, file, type_) }
    }
    #[fixed_stack_segment]
    fn assign(&self, @wxIconBundle) {
        unsafe { wxIconBundle_Assign(self, _ref) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxIconBundle {
        unsafe { wxIconBundle_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newFromFile(file: @wxString, type_: int) -> @wxIconBundle {
        unsafe { wxIconBundle_CreateFromFile(file, type_) }
    }
    #[fixed_stack_segment]
    fn newFromIcon(icon: @wxIcon) -> @wxIconBundle {
        unsafe { wxIconBundle_CreateFromIcon(icon) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, w: c_int, h: c_int, @wxIcon) {
        unsafe { wxIconBundle_GetIcon(self, _arg1, _ref) }
    }
}
trait wxIconizeEvent {
}
trait wxIdleEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: @wxObject) {
        unsafe { wxIdleEvent_CopyObject(self, object_dest) }
    }
    #[fixed_stack_segment]
    fn moreRequested(&self) -> bool {
        unsafe { wxIdleEvent_MoreRequested(self) }
    }
    #[fixed_stack_segment]
    fn requestMore(&self, needMore: bool) {
        unsafe { wxIdleEvent_RequestMore(self, needMore) }
    }
}
trait wxImage {
    #[fixed_stack_segment]
    fn canRead(name: @wxString) -> bool {
        unsafe { wxImage_CanRead(name) }
    }
    #[fixed_stack_segment]
    fn convertToBitmap(&self, @wxBitmap) {
        unsafe { wxImage_ConvertToBitmap(self, bitmap) }
    }
    #[fixed_stack_segment]
    fn convertToByteString(&self, type_: int, data: *c_char) -> c_int {
        unsafe { wxImage_ConvertToByteString(self, type_, data) }
    }
    #[fixed_stack_segment]
    fn convertToLazyByteString(&self, type_: int, data: *c_char) -> c_int {
        unsafe { wxImage_ConvertToLazyByteString(self, type_, data) }
    }
    #[fixed_stack_segment]
    fn countColours(&self, stopafter: int) -> int {
        unsafe { wxImage_CountColours(self, stopafter) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxImage {
        unsafe { wxImage_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(bitmap: @wxBitmap) -> @wxImage {
        unsafe { wxImage_CreateFromBitmap(bitmap) }
    }
    #[fixed_stack_segment]
    fn newFromByteString(data: **char, length: c_int, type_: int) -> @wxImage {
        unsafe { wxImage_CreateFromByteString(_arg0, type_) }
    }
    #[fixed_stack_segment]
    fn newFromLazyByteString(data: **char, length: c_int, type_: int) -> @wxImage {
        unsafe { wxImage_CreateFromLazyByteString(_arg0, type_) }
    }
    #[fixed_stack_segment]
    fn newFromData(width: c_int, height: c_int, data: *c_void) -> @wxImage {
        unsafe { wxImage_CreateFromData(_arg0, data) }
    }
    #[fixed_stack_segment]
    fn newFromFile(name: @wxString) -> @wxImage {
        unsafe { wxImage_CreateFromFile(name) }
    }
    #[fixed_stack_segment]
    fn newSized(width: c_int, height: c_int) -> @wxImage {
        unsafe { wxImage_CreateSized(_arg0) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxImage_Destroy(self) }
    }
    #[fixed_stack_segment]
    fn getBlue(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetBlue(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxImage_GetData(self) }
    }
    #[fixed_stack_segment]
    fn getGreen(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetGreen(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> int {
        unsafe { wxImage_GetHeight(self) }
    }
    #[fixed_stack_segment]
    fn getMaskBlue(&self) -> wchar_t {
        unsafe { wxImage_GetMaskBlue(self) }
    }
    #[fixed_stack_segment]
    fn getMaskGreen(&self) -> wchar_t {
        unsafe { wxImage_GetMaskGreen(self) }
    }
    #[fixed_stack_segment]
    fn getMaskRed(&self) -> wchar_t {
        unsafe { wxImage_GetMaskRed(self) }
    }
    #[fixed_stack_segment]
    fn getRed(&self, x: c_int, y: c_int) -> wchar_t {
        unsafe { wxImage_GetRed(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getSubImage(&self, x: c_int, y: c_int, w: c_int, h: c_int, @wxImage) {
        unsafe { wxImage_GetSubImage(self, _arg1, image) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxImage_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn hasMask(&self) -> bool {
        unsafe { wxImage_HasMask(self) }
    }
    #[fixed_stack_segment]
    fn getOption(&self, name: @wxString) -> @wxString {
        unsafe { wxImage_GetOption(self, name) }
    }
    #[fixed_stack_segment]
    fn getOptionInt(&self, name: @wxString) -> bool {
        unsafe { wxImage_GetOptionInt(self, name) }
    }
    #[fixed_stack_segment]
    fn hasOption(&self, name: @wxString) -> bool {
        unsafe { wxImage_HasOption(self, name) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Initialize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn initializeFromData(&self, width: c_int, height: c_int, data: *c_void) {
        unsafe { wxImage_InitializeFromData(self, _arg1, data) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, name: @wxString, type_: int) -> bool {
        unsafe { wxImage_LoadFile(self, name, type_) }
    }
    #[fixed_stack_segment]
    fn mirror(&self, horizontally: c_int, @wxImage) {
        unsafe { wxImage_Mirror(self, horizontally, image) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxImage_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn paste(&self, image: @wxImage, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self, image, _arg2) }
    }
    #[fixed_stack_segment]
    fn replace(&self, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) {
        unsafe { wxImage_Replace(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn rotate(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *c_void, @wxImage) {
        unsafe { wxImage_Rotate(self, angle, _arg2, interpolating, offset_after_rotation, image) }
    }
    #[fixed_stack_segment]
    fn rotate90(&self, clockwise: c_int, @wxImage) {
        unsafe { wxImage_Rotate90(self, clockwise, image) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @wxString, type_: int) -> bool {
        unsafe { wxImage_SaveFile(self, name, type_) }
    }
    #[fixed_stack_segment]
    fn scale(&self, width: c_int, height: c_int, @wxImage) {
        unsafe { wxImage_Scale(self, _arg1, image) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: *c_void) {
        unsafe { wxImage_SetData(self, data) }
    }
    #[fixed_stack_segment]
    fn setDataAndSize(&self, data: *c_void, new_width: c_int, new_height: c_int) {
        unsafe { wxImage_SetDataAndSize(self, data, _arg2) }
    }
    #[fixed_stack_segment]
    fn setMask(&self, mask: int) {
        unsafe { wxImage_SetMask(self, mask) }
    }
    #[fixed_stack_segment]
    fn setMaskColour(&self, r: u8, g: u8, b: u8) {
        unsafe { wxImage_SetMaskColour(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setOption(&self, name: @wxString, value: @wxString) {
        unsafe { wxImage_SetOption(self, name, value) }
    }
    #[fixed_stack_segment]
    fn setOptionInt(&self, name: @wxString, value: int) {
        unsafe { wxImage_SetOptionInt(self, name, value) }
    }
    #[fixed_stack_segment]
    fn setRGB(&self, x: c_int, y: c_int, r: u8, g: u8, b: u8) {
        unsafe { wxImage_SetRGB(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn newFromDataEx(width: c_int, height: c_int, data: *c_void, isStaticData: c_int) -> @wxImage {
        unsafe { wxImage_CreateFromDataEx(_arg0, data, isStaticData) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImage_Delete(self) }
    }
}
trait wxImageHandler {
}
trait wxImageList {
    #[fixed_stack_segment]
    fn addBitmap(&self, bitmap: @wxBitmap, mask: @wxBitmap) -> int {
        unsafe { wxImageList_AddBitmap(self, bitmap, mask) }
    }
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @wxIcon) -> int {
        unsafe { wxImageList_AddIcon(self, icon) }
    }
    #[fixed_stack_segment]
    fn addMasked(&self, bitmap: @wxBitmap, maskColour: @wxColour) -> int {
        unsafe { wxImageList_AddMasked(self, bitmap, maskColour) }
    }
    #[fixed_stack_segment]
    fn new(width: c_int, height: c_int, mask: c_int, initialCount: int) -> @wxImageList {
        unsafe { wxImageList_Create(_arg0, mask, initialCount) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImageList_Delete(self) }
    }
    #[fixed_stack_segment]
    fn draw(&self, index: int, dc: @wxDC, x: c_int, y: c_int, flags: int, solidBackground: bool) -> bool {
        unsafe { wxImageList_Draw(self, index, dc, _arg3, flags, solidBackground) }
    }
    #[fixed_stack_segment]
    fn getImageCount(&self) -> int {
        unsafe { wxImageList_GetImageCount(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self, index: int, width: *c_int, height: *c_int) {
        unsafe { wxImageList_GetSize(self, index, _arg2) }
    }
    #[fixed_stack_segment]
    fn remove(&self, index: int) -> bool {
        unsafe { wxImageList_Remove(self, index) }
    }
    #[fixed_stack_segment]
    fn removeAll(&self) -> bool {
        unsafe { wxImageList_RemoveAll(self) }
    }
    #[fixed_stack_segment]
    fn replace(&self, index: int, bitmap: @wxBitmap, mask: @wxBitmap) -> bool {
        unsafe { wxImageList_Replace(self, index, bitmap, mask) }
    }
    #[fixed_stack_segment]
    fn replaceIcon(&self, index: int, icon: @wxIcon) -> bool {
        unsafe { wxImageList_ReplaceIcon(self, index, icon) }
    }
}
trait wxIndividualLayoutConstraint {
    #[fixed_stack_segment]
    fn above(&self, sibling: @wxWindow, marg: int) {
        unsafe { wxIndividualLayoutConstraint_Above(self, sibling, marg) }
    }
    #[fixed_stack_segment]
    fn absolute(&self, val: int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self, val) }
    }
    #[fixed_stack_segment]
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self) }
    }
    #[fixed_stack_segment]
    fn below(&self, sibling: @wxWindow, marg: int) {
        unsafe { wxIndividualLayoutConstraint_Below(self, sibling, marg) }
    }
    #[fixed_stack_segment]
    fn getDone(&self) -> bool {
        unsafe { wxIndividualLayoutConstraint_GetDone(self) }
    }
    #[fixed_stack_segment]
    fn getEdge(&self, which: int, thisWin: *c_void, other: *c_void) -> int {
        unsafe { wxIndividualLayoutConstraint_GetEdge(self, which, thisWin, other) }
    }
    #[fixed_stack_segment]
    fn getMargin(&self) -> int {
        unsafe { wxIndividualLayoutConstraint_GetMargin(self) }
    }
    #[fixed_stack_segment]
    fn getMyEdge(&self) -> int {
        unsafe { wxIndividualLayoutConstraint_GetMyEdge(self) }
    }
    #[fixed_stack_segment]
    fn getOtherEdge(&self) -> int {
        unsafe { wxIndividualLayoutConstraint_GetOtherEdge(self) }
    }
    #[fixed_stack_segment]
    fn getOtherWindow(&self) {
        unsafe { wxIndividualLayoutConstraint_GetOtherWindow(self) }
    }
    #[fixed_stack_segment]
    fn getPercent(&self) -> int {
        unsafe { wxIndividualLayoutConstraint_GetPercent(self) }
    }
    #[fixed_stack_segment]
    fn getRelationship(&self) -> int {
        unsafe { wxIndividualLayoutConstraint_GetRelationship(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> int {
        unsafe { wxIndividualLayoutConstraint_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn leftOf(&self, sibling: @wxWindow, marg: int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self, sibling, marg) }
    }
    #[fixed_stack_segment]
    fn percentOf(&self, otherW: @wxWindow, wh: int, per: int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self, otherW, wh, per) }
    }
    #[fixed_stack_segment]
    fn resetIfWin(&self, otherW: @wxWindow) -> bool {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self, otherW) }
    }
    #[fixed_stack_segment]
    fn rightOf(&self, sibling: @wxWindow, marg: int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self, sibling, marg) }
    }
    #[fixed_stack_segment]
    fn sameAs(&self, otherW: @wxWindow, edge: int, marg: int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self, otherW, edge, marg) }
    }
    #[fixed_stack_segment]
    fn satisfyConstraint(&self, constraints: *c_void, win: @wxWindow) -> bool {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self, constraints, win) }
    }
    #[fixed_stack_segment]
    fn set(&self, rel: int, otherW: @wxWindow, otherE: int, val: int, marg: int) {
        unsafe { wxIndividualLayoutConstraint_Set(self, rel, otherW, otherE, val, marg) }
    }
    #[fixed_stack_segment]
    fn setDone(&self, d: bool) {
        unsafe { wxIndividualLayoutConstraint_SetDone(self, d) }
    }
    #[fixed_stack_segment]
    fn setEdge(&self, which: int) {
        unsafe { wxIndividualLayoutConstraint_SetEdge(self, which) }
    }
    #[fixed_stack_segment]
    fn setMargin(&self, m: int) {
        unsafe { wxIndividualLayoutConstraint_SetMargin(self, m) }
    }
    #[fixed_stack_segment]
    fn setRelationship(&self, r: int) {
        unsafe { wxIndividualLayoutConstraint_SetRelationship(self, r) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, v: int) {
        unsafe { wxIndividualLayoutConstraint_SetValue(self, v) }
    }
    #[fixed_stack_segment]
    fn unconstrained(&self) {
        unsafe { wxIndividualLayoutConstraint_Unconstrained(self) }
    }
}
trait wxInitDialogEvent {
}
trait wxInputStream {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxInputStream_Delete(self) }
    }
    #[fixed_stack_segment]
    fn eof(&self) -> bool {
        unsafe { wxInputStream_Eof(self) }
    }
    #[fixed_stack_segment]
    fn getC(&self) -> wchar_t {
        unsafe { wxInputStream_GetC(self) }
    }
    #[fixed_stack_segment]
    fn lastRead(&self) -> int {
        unsafe { wxInputStream_LastRead(self) }
    }
    #[fixed_stack_segment]
    fn peek(&self) -> wchar_t {
        unsafe { wxInputStream_Peek(self) }
    }
    #[fixed_stack_segment]
    fn read(&self, buffer: *c_void, size: int) {
        unsafe { wxInputStream_Read(self, buffer, size) }
    }
    #[fixed_stack_segment]
    fn seekI(&self, pos: int, mode: int) -> int {
        unsafe { wxInputStream_SeekI(self, pos, mode) }
    }
    #[fixed_stack_segment]
    fn tell(&self) -> int {
        unsafe { wxInputStream_Tell(self) }
    }
    #[fixed_stack_segment]
    fn ungetBuffer(&self, buffer: *c_void, size: int) -> int {
        unsafe { wxInputStream_UngetBuffer(self, buffer, size) }
    }
    #[fixed_stack_segment]
    fn ungetch(&self, c: wchar_t) -> int {
        unsafe { wxInputStream_Ungetch(self, c) }
    }
    #[fixed_stack_segment]
    fn canRead(&self) -> bool {
        unsafe { wxInputStream_CanRead(self) }
    }
}
trait wxJoystick {
    #[fixed_stack_segment]
    fn new(joystick: int) -> @wxJoystick {
        unsafe { wxJoystick_Create(joystick) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxJoystick_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getButtonState(&self) -> int {
        unsafe { wxJoystick_GetButtonState(self) }
    }
    #[fixed_stack_segment]
    fn getManufacturerId(&self) -> int {
        unsafe { wxJoystick_GetManufacturerId(self) }
    }
    #[fixed_stack_segment]
    fn getMaxAxes(&self) -> int {
        unsafe { wxJoystick_GetMaxAxes(self) }
    }
    #[fixed_stack_segment]
    fn getMaxButtons(&self) -> int {
        unsafe { wxJoystick_GetMaxButtons(self) }
    }
    #[fixed_stack_segment]
    fn getMovementThreshold(&self) -> int {
        unsafe { wxJoystick_GetMovementThreshold(self) }
    }
    #[fixed_stack_segment]
    fn getNumberAxes(&self) -> int {
        unsafe { wxJoystick_GetNumberAxes(self) }
    }
    #[fixed_stack_segment]
    fn getNumberButtons(&self) -> int {
        unsafe { wxJoystick_GetNumberButtons(self) }
    }
    #[fixed_stack_segment]
    fn getNumberJoysticks(&self) -> int {
        unsafe { wxJoystick_GetNumberJoysticks(self) }
    }
    #[fixed_stack_segment]
    fn getPOVCTSPosition(&self) -> int {
        unsafe { wxJoystick_GetPOVCTSPosition(self) }
    }
    #[fixed_stack_segment]
    fn getPOVPosition(&self) -> int {
        unsafe { wxJoystick_GetPOVPosition(self) }
    }
    #[fixed_stack_segment]
    fn getPollingMax(&self) -> int {
        unsafe { wxJoystick_GetPollingMax(self) }
    }
    #[fixed_stack_segment]
    fn getPollingMin(&self) -> int {
        unsafe { wxJoystick_GetPollingMin(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxJoystick_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getProductId(&self) -> int {
        unsafe { wxJoystick_GetProductId(self) }
    }
    #[fixed_stack_segment]
    fn getProductName(&self) -> @wxString {
        unsafe { wxJoystick_GetProductName(self) }
    }
    #[fixed_stack_segment]
    fn getRudderMax(&self) -> int {
        unsafe { wxJoystick_GetRudderMax(self) }
    }
    #[fixed_stack_segment]
    fn getRudderMin(&self) -> int {
        unsafe { wxJoystick_GetRudderMin(self) }
    }
    #[fixed_stack_segment]
    fn getRudderPosition(&self) -> int {
        unsafe { wxJoystick_GetRudderPosition(self) }
    }
    #[fixed_stack_segment]
    fn getUMax(&self) -> int {
        unsafe { wxJoystick_GetUMax(self) }
    }
    #[fixed_stack_segment]
    fn getUMin(&self) -> int {
        unsafe { wxJoystick_GetUMin(self) }
    }
    #[fixed_stack_segment]
    fn getUPosition(&self) -> int {
        unsafe { wxJoystick_GetUPosition(self) }
    }
    #[fixed_stack_segment]
    fn getVMax(&self) -> int {
        unsafe { wxJoystick_GetVMax(self) }
    }
    #[fixed_stack_segment]
    fn getVMin(&self) -> int {
        unsafe { wxJoystick_GetVMin(self) }
    }
    #[fixed_stack_segment]
    fn getVPosition(&self) -> int {
        unsafe { wxJoystick_GetVPosition(self) }
    }
    #[fixed_stack_segment]
    fn getXMax(&self) -> int {
        unsafe { wxJoystick_GetXMax(self) }
    }
    #[fixed_stack_segment]
    fn getXMin(&self) -> int {
        unsafe { wxJoystick_GetXMin(self) }
    }
    #[fixed_stack_segment]
    fn getYMax(&self) -> int {
        unsafe { wxJoystick_GetYMax(self) }
    }
    #[fixed_stack_segment]
    fn getYMin(&self) -> int {
        unsafe { wxJoystick_GetYMin(self) }
    }
    #[fixed_stack_segment]
    fn getZMax(&self) -> int {
        unsafe { wxJoystick_GetZMax(self) }
    }
    #[fixed_stack_segment]
    fn getZMin(&self) -> int {
        unsafe { wxJoystick_GetZMin(self) }
    }
    #[fixed_stack_segment]
    fn getZPosition(&self) -> int {
        unsafe { wxJoystick_GetZPosition(self) }
    }
    #[fixed_stack_segment]
    fn hasPOV(&self) -> bool {
        unsafe { wxJoystick_HasPOV(self) }
    }
    #[fixed_stack_segment]
    fn hasPOV4Dir(&self) -> bool {
        unsafe { wxJoystick_HasPOV4Dir(self) }
    }
    #[fixed_stack_segment]
    fn hasPOVCTS(&self) -> bool {
        unsafe { wxJoystick_HasPOVCTS(self) }
    }
    #[fixed_stack_segment]
    fn hasRudder(&self) -> bool {
        unsafe { wxJoystick_HasRudder(self) }
    }
    #[fixed_stack_segment]
    fn hasU(&self) -> bool {
        unsafe { wxJoystick_HasU(self) }
    }
    #[fixed_stack_segment]
    fn hasV(&self) -> bool {
        unsafe { wxJoystick_HasV(self) }
    }
    #[fixed_stack_segment]
    fn hasZ(&self) -> bool {
        unsafe { wxJoystick_HasZ(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxJoystick_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn releaseCapture(&self) -> int {
        unsafe { wxJoystick_ReleaseCapture(self) }
    }
    #[fixed_stack_segment]
    fn setCapture(&self, win: @wxWindow, pollingFreq: int) -> int {
        unsafe { wxJoystick_SetCapture(self, win, pollingFreq) }
    }
    #[fixed_stack_segment]
    fn setMovementThreshold(&self, threshold: int) {
        unsafe { wxJoystick_SetMovementThreshold(self, threshold) }
    }
}
trait wxJoystickEvent {
    #[fixed_stack_segment]
    fn buttonDown(&self, but: int) -> bool {
        unsafe { wxJoystickEvent_ButtonDown(self, but) }
    }
    #[fixed_stack_segment]
    fn buttonIsDown(&self, but: int) -> bool {
        unsafe { wxJoystickEvent_ButtonIsDown(self, but) }
    }
    #[fixed_stack_segment]
    fn buttonUp(&self, but: int) -> bool {
        unsafe { wxJoystickEvent_ButtonUp(self, but) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxJoystickEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getButtonChange(&self) -> int {
        unsafe { wxJoystickEvent_GetButtonChange(self) }
    }
    #[fixed_stack_segment]
    fn getButtonState(&self) -> int {
        unsafe { wxJoystickEvent_GetButtonState(self) }
    }
    #[fixed_stack_segment]
    fn getJoystick(&self) -> int {
        unsafe { wxJoystickEvent_GetJoystick(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxJoystickEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getZPosition(&self) -> int {
        unsafe { wxJoystickEvent_GetZPosition(self) }
    }
    #[fixed_stack_segment]
    fn isButton(&self) -> bool {
        unsafe { wxJoystickEvent_IsButton(self) }
    }
    #[fixed_stack_segment]
    fn isMove(&self) -> bool {
        unsafe { wxJoystickEvent_IsMove(self) }
    }
    #[fixed_stack_segment]
    fn isZMove(&self) -> bool {
        unsafe { wxJoystickEvent_IsZMove(self) }
    }
    #[fixed_stack_segment]
    fn setButtonChange(&self, change: int) {
        unsafe { wxJoystickEvent_SetButtonChange(self, change) }
    }
    #[fixed_stack_segment]
    fn setButtonState(&self, state: int) {
        unsafe { wxJoystickEvent_SetButtonState(self, state) }
    }
    #[fixed_stack_segment]
    fn setJoystick(&self, stick: int) {
        unsafe { wxJoystickEvent_SetJoystick(self, stick) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxJoystickEvent_SetPosition(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setZPosition(&self, zPos: int) {
        unsafe { wxJoystickEvent_SetZPosition(self, zPos) }
    }
}
trait wxKeyEvent {
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxKeyEvent_AltDown(self) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxKeyEvent_ControlDown(self) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxKeyEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getKeyCode(&self) -> int {
        unsafe { wxKeyEvent_GetKeyCode(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxKeyEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> int {
        unsafe { wxKeyEvent_GetX(self) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> int {
        unsafe { wxKeyEvent_GetY(self) }
    }
    #[fixed_stack_segment]
    fn getModifiers(&self) -> int {
        unsafe { wxKeyEvent_GetModifiers(self) }
    }
    #[fixed_stack_segment]
    fn hasModifiers(&self) -> bool {
        unsafe { wxKeyEvent_HasModifiers(self) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxKeyEvent_MetaDown(self) }
    }
    #[fixed_stack_segment]
    fn setKeyCode(&self, code: int) {
        unsafe { wxKeyEvent_SetKeyCode(self, code) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxKeyEvent_ShiftDown(self) }
    }
}
trait wxLEDNumberCtrl {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxLEDNumberCtrl {
        unsafe { wxLEDNumberCtrl_Create(parent, id, _arg2, style) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> int {
        unsafe { wxLEDNumberCtrl_GetAlignment(self) }
    }
    #[fixed_stack_segment]
    fn getDrawFaded(&self) -> int {
        unsafe { wxLEDNumberCtrl_GetDrawFaded(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self, _ref: *c_void) -> int {
        unsafe { wxLEDNumberCtrl_GetValue(self, _ref) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, Alignment: int, Redraw: int) {
        unsafe { wxLEDNumberCtrl_SetAlignment(self, Alignment, Redraw) }
    }
    #[fixed_stack_segment]
    fn setDrawFaded(&self, DrawFaded: int, Redraw: int) {
        unsafe { wxLEDNumberCtrl_SetDrawFaded(self, DrawFaded, Redraw) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, Value: *c_void, Redraw: int) {
        unsafe { wxLEDNumberCtrl_SetValue(self, Value, Redraw) }
    }
}
trait wxLayoutAlgorithm {
    #[fixed_stack_segment]
    fn new() -> @wxLayoutAlgorithm {
        unsafe { wxLayoutAlgorithm_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLayoutAlgorithm_Delete(self) }
    }
    #[fixed_stack_segment]
    fn layoutFrame(&self, frame: @wxFrame, mainWindow: *c_void) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self, frame, mainWindow) }
    }
    #[fixed_stack_segment]
    fn layoutMDIFrame(&self, frame: @wxFrame, x: c_int, y: c_int, w: c_int, h: c_int, use_: int) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self, frame, _arg2, use_) }
    }
    #[fixed_stack_segment]
    fn layoutWindow(&self, frame: @wxFrame, mainWindow: *c_void) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self, frame, mainWindow) }
    }
}
trait wxLayoutConstraints {
    #[fixed_stack_segment]
    fn new() -> @wxLayoutConstraints {
        unsafe { wxLayoutConstraints_Create() }
    }
    #[fixed_stack_segment]
    fn bottom(&self) {
        unsafe { wxLayoutConstraints_bottom(self) }
    }
    #[fixed_stack_segment]
    fn centreX(&self) {
        unsafe { wxLayoutConstraints_centreX(self) }
    }
    #[fixed_stack_segment]
    fn centreY(&self) {
        unsafe { wxLayoutConstraints_centreY(self) }
    }
    #[fixed_stack_segment]
    fn height(&self) {
        unsafe { wxLayoutConstraints_height(self) }
    }
    #[fixed_stack_segment]
    fn left(&self) {
        unsafe { wxLayoutConstraints_left(self) }
    }
    #[fixed_stack_segment]
    fn right(&self) {
        unsafe { wxLayoutConstraints_right(self) }
    }
    #[fixed_stack_segment]
    fn top(&self) {
        unsafe { wxLayoutConstraints_top(self) }
    }
    #[fixed_stack_segment]
    fn width(&self) {
        unsafe { wxLayoutConstraints_width(self) }
    }
}
trait wxList {
}
trait wxListBox {
    #[fixed_stack_segment]
    fn append(&self, item: @wxString) {
        unsafe { wxListBox_Append(self, item) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @wxString, data: *c_void) {
        unsafe { wxListBox_AppendData(self, item, data) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxListBox_Clear(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxListBox {
        unsafe { wxListBox_Create(_prt, _id, _arg2, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self, n: int) {
        unsafe { wxListBox_Delete(self, n) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> int {
        unsafe { wxListBox_FindString(self, s) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: int) -> @wxClientData {
        unsafe { wxListBox_GetClientData(self, n) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxListBox_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> int {
        unsafe { wxListBox_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn getSelections(&self, aSelections: *int, allocated: int) -> int {
        unsafe { wxListBox_GetSelections(self, aSelections, allocated) }
    }
    #[fixed_stack_segment]
    fn getString(&self, n: int) -> @wxString {
        unsafe { wxListBox_GetString(self, n) }
    }
    #[fixed_stack_segment]
    fn insertItems(&self, items: *c_void, pos: int, count: int) {
        unsafe { wxListBox_InsertItems(self, items, pos, count) }
    }
    #[fixed_stack_segment]
    fn isSelected(&self, n: int) -> bool {
        unsafe { wxListBox_IsSelected(self, n) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, n: int, clientData: @wxClientData) {
        unsafe { wxListBox_SetClientData(self, n, clientData) }
    }
    #[fixed_stack_segment]
    fn setFirstItem(&self, n: int) {
        unsafe { wxListBox_SetFirstItem(self, n) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: int, select: c_int) {
        unsafe { wxListBox_SetSelection(self, n, select) }
    }
    #[fixed_stack_segment]
    fn setString(&self, n: int, s: @wxString) {
        unsafe { wxListBox_SetString(self, n, s) }
    }
    #[fixed_stack_segment]
    fn setStringSelection(&self, str: @wxString, sel: bool) {
        unsafe { wxListBox_SetStringSelection(self, str, sel) }
    }
}
trait wxListCtrl {
    #[fixed_stack_segment]
    fn arrange(&self, flag: int) -> bool {
        unsafe { wxListCtrl_Arrange(self, flag) }
    }
    #[fixed_stack_segment]
    fn clearAll(&self) {
        unsafe { wxListCtrl_ClearAll(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxListCtrl {
        unsafe { wxListCtrl_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn deleteAllColumns(&self) -> bool {
        unsafe { wxListCtrl_DeleteAllColumns(self) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) -> bool {
        unsafe { wxListCtrl_DeleteAllItems(self) }
    }
    #[fixed_stack_segment]
    fn deleteColumn(&self, col: int) -> bool {
        unsafe { wxListCtrl_DeleteColumn(self, col) }
    }
    #[fixed_stack_segment]
    fn deleteItem(&self, item: int) -> bool {
        unsafe { wxListCtrl_DeleteItem(self, item) }
    }
    #[fixed_stack_segment]
    fn editLabel(&self, item: int) {
        unsafe { wxListCtrl_EditLabel(self, item) }
    }
    #[fixed_stack_segment]
    fn endEditLabel(&self, cancel: int) -> bool {
        unsafe { wxListCtrl_EndEditLabel(self, cancel) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, item: int) -> bool {
        unsafe { wxListCtrl_EnsureVisible(self, item) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, start: int, str: @wxString, partial: bool) -> int {
        unsafe { wxListCtrl_FindItem(self, start, str, partial) }
    }
    #[fixed_stack_segment]
    fn findItemByData(&self, start: int, data: int) -> int {
        unsafe { wxListCtrl_FindItemByData(self, start, data) }
    }
    #[fixed_stack_segment]
    fn findItemByPosition(&self, start: int, x: c_int, y: c_int, direction: int) -> int {
        unsafe { wxListCtrl_FindItemByPosition(self, start, _arg2, direction) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self, col: int, item: @wxListItem) -> bool {
        unsafe { wxListCtrl_GetColumn(self, col, item) }
    }
    #[fixed_stack_segment]
    fn getColumnCount(&self) -> int {
        unsafe { wxListCtrl_GetColumnCount(self) }
    }
    #[fixed_stack_segment]
    fn getColumnWidth(&self, col: int) -> int {
        unsafe { wxListCtrl_GetColumnWidth(self, col) }
    }
    #[fixed_stack_segment]
    fn getCountPerPage(&self) -> int {
        unsafe { wxListCtrl_GetCountPerPage(self) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> @wxTextCtrl {
        unsafe { wxListCtrl_GetEditControl(self) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self, which: int) -> @wxImageList {
        unsafe { wxListCtrl_GetImageList(self, which) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, info: @wxListItem) -> bool {
        unsafe { wxListCtrl_GetItem(self, info) }
    }
    #[fixed_stack_segment]
    fn getItemCount(&self) -> int {
        unsafe { wxListCtrl_GetItemCount(self) }
    }
    #[fixed_stack_segment]
    fn getItemData(&self, item: int) -> int {
        unsafe { wxListCtrl_GetItemData(self, item) }
    }
    #[fixed_stack_segment]
    fn getItemFont(&self, item: c_long) -> @wxFont {
        unsafe { wxListCtrl_GetItemFont(self, item) }
    }
    #[fixed_stack_segment]
    fn getItemPosition(&self, item: int) -> @wxPoint {
        unsafe { wxListCtrl_GetItemPosition(self, item) }
    }
    #[fixed_stack_segment]
    fn getItemRect(&self, item: int, code: int) -> @wxRect {
        unsafe { wxListCtrl_GetItemRect(self, item, code) }
    }
    #[fixed_stack_segment]
    fn getItemSpacing(&self, isSmall: bool) -> @wxSize {
        unsafe { wxListCtrl_GetItemSpacing(self, isSmall) }
    }
    #[fixed_stack_segment]
    fn getItemState(&self, item: int, stateMask: int) -> int {
        unsafe { wxListCtrl_GetItemState(self, item, stateMask) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: int) -> @wxString {
        unsafe { wxListCtrl_GetItemText(self, item) }
    }
    #[fixed_stack_segment]
    fn getNextItem(&self, item: int, geometry: int, state: int) -> int {
        unsafe { wxListCtrl_GetNextItem(self, item, geometry, state) }
    }
    #[fixed_stack_segment]
    fn getSelectedItemCount(&self) -> int {
        unsafe { wxListCtrl_GetSelectedItemCount(self) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, @wxColour) {
        unsafe { wxListCtrl_GetTextColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getTopItem(&self) -> int {
        unsafe { wxListCtrl_GetTopItem(self) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int, flags: *c_void) -> int {
        unsafe { wxListCtrl_HitTest(self, _arg1, flags) }
    }
    #[fixed_stack_segment]
    fn insertColumn(&self, col: int, heading: @wxString, format: int, width: int) -> int {
        unsafe { wxListCtrl_InsertColumn(self, col, heading, format, width) }
    }
    #[fixed_stack_segment]
    fn insertColumnFromInfo(&self, col: int, info: @wxListItem) -> int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self, col, info) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, info: @wxListItem) -> int {
        unsafe { wxListCtrl_InsertItem(self, info) }
    }
    #[fixed_stack_segment]
    fn insertItemWithData(&self, index: int, label: @wxString) -> int {
        unsafe { wxListCtrl_InsertItemWithData(self, index, label) }
    }
    #[fixed_stack_segment]
    fn insertItemWithImage(&self, index: int, imageIndex: int) -> int {
        unsafe { wxListCtrl_InsertItemWithImage(self, index, imageIndex) }
    }
    #[fixed_stack_segment]
    fn insertItemWithLabel(&self, index: int, label: @wxString, imageIndex: int) -> int {
        unsafe { wxListCtrl_InsertItemWithLabel(self, index, label, imageIndex) }
    }
    #[fixed_stack_segment]
    fn isVirtual(&self) -> bool {
        unsafe { wxListCtrl_IsVirtual(self) }
    }
    #[fixed_stack_segment]
    fn refreshItem(&self, item: c_long) {
        unsafe { wxListCtrl_RefreshItem(self, item) }
    }
    #[fixed_stack_segment]
    fn scrollList(&self, dx: c_int, dy: c_int) -> bool {
        unsafe { wxListCtrl_ScrollList(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, col: @wxColour) {
        unsafe { wxListCtrl_SetBackgroundColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setColumn(&self, col: int, item: @wxListItem) -> bool {
        unsafe { wxListCtrl_SetColumn(self, col, item) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, col: int, width: int) -> bool {
        unsafe { wxListCtrl_SetColumnWidth(self, col, width) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, col: @wxColour) -> int {
        unsafe { wxListCtrl_SetForegroundColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @wxImageList, which: int) {
        unsafe { wxListCtrl_SetImageList(self, imageList, which) }
    }
    #[fixed_stack_segment]
    fn setItem(&self, index: int, col: int, label: @wxString, imageId: int) -> bool {
        unsafe { wxListCtrl_SetItem(self, index, col, label, imageId) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: int, data: int) -> bool {
        unsafe { wxListCtrl_SetItemData(self, item, data) }
    }
    #[fixed_stack_segment]
    fn setItemFromInfo(&self, info: @wxListItem) -> bool {
        unsafe { wxListCtrl_SetItemFromInfo(self, info) }
    }
    #[fixed_stack_segment]
    fn setItemImage(&self, item: int, image: int, selImage: int) -> bool {
        unsafe { wxListCtrl_SetItemImage(self, item, image, selImage) }
    }
    #[fixed_stack_segment]
    fn setItemPosition(&self, item: int, x: c_int, y: c_int) -> bool {
        unsafe { wxListCtrl_SetItemPosition(self, item, _arg2) }
    }
    #[fixed_stack_segment]
    fn setItemState(&self, item: int, state: int, stateMask: int) -> bool {
        unsafe { wxListCtrl_SetItemState(self, item, state, stateMask) }
    }
    #[fixed_stack_segment]
    fn setItemText(&self, item: int, str: @wxString) {
        unsafe { wxListCtrl_SetItemText(self, item, str) }
    }
    #[fixed_stack_segment]
    fn setSingleStyle(&self, style: int, add: bool) {
        unsafe { wxListCtrl_SetSingleStyle(self, style, add) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @wxColour) {
        unsafe { wxListCtrl_SetTextColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setWindowStyleFlag(&self, style: int) {
        unsafe { wxListCtrl_SetWindowStyleFlag(self, style) }
    }
    #[fixed_stack_segment]
    fn sortItems(&self, fn_: *c_void, eif_obj: *c_void) -> bool {
        unsafe { wxListCtrl_SortItems(self, fn_, eif_obj) }
    }
    #[fixed_stack_segment]
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, images: @wxImageList, which: int) {
        unsafe { wxListCtrl_AssignImageList(self, images, which) }
    }
    #[fixed_stack_segment]
    fn getColumn2(&self, col: int, @wxListItem) {
        unsafe { wxListCtrl_GetColumn2(self, col, item) }
    }
    #[fixed_stack_segment]
    fn getItem2(&self, @wxListItem) {
        unsafe { wxListCtrl_GetItem2(self, info) }
    }
    #[fixed_stack_segment]
    fn getItemPosition2(&self, item: int) -> @wxPoint {
        unsafe { wxListCtrl_GetItemPosition2(self, item) }
    }
    #[fixed_stack_segment]
    fn sortItems2(&self, closure: @wxClosure) -> bool {
        unsafe { wxListCtrl_SortItems2(self, closure) }
    }
}
trait wxListEvent {
    #[fixed_stack_segment]
    fn cancelled(&self) -> bool {
        unsafe { wxListEvent_Cancelled(self) }
    }
    #[fixed_stack_segment]
    fn getCode(&self) -> int {
        unsafe { wxListEvent_GetCode(self) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self) -> int {
        unsafe { wxListEvent_GetColumn(self) }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> int {
        unsafe { wxListEvent_GetData(self) }
    }
    #[fixed_stack_segment]
    fn getImage(&self) -> int {
        unsafe { wxListEvent_GetImage(self) }
    }
    #[fixed_stack_segment]
    fn getIndex(&self) -> int {
        unsafe { wxListEvent_GetIndex(self) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, @wxListItem) {
        unsafe { wxListEvent_GetItem(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { wxListEvent_GetLabel(self) }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> int {
        unsafe { wxListEvent_GetMask(self) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @wxPoint {
        unsafe { wxListEvent_GetPoint(self) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { wxListEvent_GetText(self) }
    }
    #[fixed_stack_segment]
    fn getCacheFrom(&self) -> int {
        unsafe { wxListEvent_GetCacheFrom(self) }
    }
    #[fixed_stack_segment]
    fn getCacheTo(&self) -> int {
        unsafe { wxListEvent_GetCacheTo(self) }
    }
}
trait wxListItem {
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxListItem_Clear(self) }
    }
    #[fixed_stack_segment]
    fn clearAttributes(&self) {
        unsafe { wxListItem_ClearAttributes(self) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxListItem {
        unsafe { wxListItem_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxListItem_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getAlign(&self) -> int {
        unsafe { wxListItem_GetAlign(self) }
    }
    #[fixed_stack_segment]
    fn getAttributes(&self) {
        unsafe { wxListItem_GetAttributes(self) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, @wxColour) {
        unsafe { wxListItem_GetBackgroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self) -> int {
        unsafe { wxListItem_GetColumn(self) }
    }
    #[fixed_stack_segment]
    fn getData(&self) -> int {
        unsafe { wxListItem_GetData(self) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, @wxFont) {
        unsafe { wxListItem_GetFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> int {
        unsafe { wxListItem_GetId(self) }
    }
    #[fixed_stack_segment]
    fn getImage(&self) -> int {
        unsafe { wxListItem_GetImage(self) }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> int {
        unsafe { wxListItem_GetMask(self) }
    }
    #[fixed_stack_segment]
    fn getState(&self) -> int {
        unsafe { wxListItem_GetState(self) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { wxListItem_GetText(self) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, @wxColour) {
        unsafe { wxListItem_GetTextColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxListItem_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn hasAttributes(&self) -> bool {
        unsafe { wxListItem_HasAttributes(self) }
    }
    #[fixed_stack_segment]
    fn setAlign(&self, align: int) {
        unsafe { wxListItem_SetAlign(self, align) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colBack: @wxColour) {
        unsafe { wxListItem_SetBackgroundColour(self, colBack) }
    }
    #[fixed_stack_segment]
    fn setColumn(&self, col: int) {
        unsafe { wxListItem_SetColumn(self, col) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: int) {
        unsafe { wxListItem_SetData(self, data) }
    }
    #[fixed_stack_segment]
    fn setDataPointer(&self, data: *c_void) {
        unsafe { wxListItem_SetDataPointer(self, data) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxListItem_SetFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: int) {
        unsafe { wxListItem_SetId(self, id) }
    }
    #[fixed_stack_segment]
    fn setImage(&self, image: int) {
        unsafe { wxListItem_SetImage(self, image) }
    }
    #[fixed_stack_segment]
    fn setMask(&self, mask: int) {
        unsafe { wxListItem_SetMask(self, mask) }
    }
    #[fixed_stack_segment]
    fn setState(&self, state: int) {
        unsafe { wxListItem_SetState(self, state) }
    }
    #[fixed_stack_segment]
    fn setStateMask(&self, stateMask: int) {
        unsafe { wxListItem_SetStateMask(self, stateMask) }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @wxString) {
        unsafe { wxListItem_SetText(self, text) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @wxColour) {
        unsafe { wxListItem_SetTextColour(self, colText) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: int) {
        unsafe { wxListItem_SetWidth(self, width) }
    }
}
trait wxLocale {
    #[fixed_stack_segment]
    fn addCatalog(&self, szDomain: *c_void) -> int {
        unsafe { wxLocale_AddCatalog(self, szDomain) }
    }
    #[fixed_stack_segment]
    fn addCatalogLookupPathPrefix(&self, prefix: *c_void) {
        unsafe { wxLocale_AddCatalogLookupPathPrefix(self, prefix) }
    }
    #[fixed_stack_segment]
    fn new(_name: int, _flags: int) -> @wxLocale {
        unsafe { wxLocale_Create(_name, _flags) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLocale_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getLocale(&self) -> @wxLocale {
        unsafe { wxLocale_GetLocale(self) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { wxLocale_GetName(self) }
    }
    #[fixed_stack_segment]
    fn getString(&self, szOrigString: *c_void, szDomain: *c_void) -> *wchar_t {
        unsafe { wxLocale_GetString(self, szOrigString, szDomain) }
    }
    #[fixed_stack_segment]
    fn isLoaded(&self, szDomain: *c_void) -> bool {
        unsafe { wxLocale_IsLoaded(self, szDomain) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxLocale_IsOk(self) }
    }
}
trait wxLog {
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: @wxString) {
        unsafe { wxLog_AddTraceMask(self, str) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLog_Delete(self) }
    }
    #[fixed_stack_segment]
    fn dontCreateOnDemand(&self) {
        unsafe { wxLog_DontCreateOnDemand(self) }
    }
    #[fixed_stack_segment]
    fn flush(&self) {
        unsafe { wxLog_Flush(self) }
    }
    #[fixed_stack_segment]
    fn flushActive(&self) {
        unsafe { wxLog_FlushActive(self) }
    }
    #[fixed_stack_segment]
    fn getActiveTarget() -> @wxLog {
        unsafe { wxLog_GetActiveTarget() }
    }
    #[fixed_stack_segment]
    fn getTimestamp(&self) -> *char {
        unsafe { wxLog_GetTimestamp(self) }
    }
    #[fixed_stack_segment]
    fn getTraceMask(&self) -> int {
        unsafe { wxLog_GetTraceMask(self) }
    }
    #[fixed_stack_segment]
    fn getVerbose(&self) -> int {
        unsafe { wxLog_GetVerbose(self) }
    }
    #[fixed_stack_segment]
    fn hasPendingMessages(&self) -> bool {
        unsafe { wxLog_HasPendingMessages(self) }
    }
    #[fixed_stack_segment]
    fn isAllowedTraceMask(&self, mask: @wxMask) -> bool {
        unsafe { wxLog_IsAllowedTraceMask(self, mask) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: int, szString: *wchar_t, t: int) {
        unsafe { wxLog_OnLog(self, level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask(&self, str: @wxString) {
        unsafe { wxLog_RemoveTraceMask(self, str) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxLog_Resume(self) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) -> @wxLog {
        unsafe { wxLog_SetActiveTarget(self) }
    }
    #[fixed_stack_segment]
    fn setTimestamp(&self, ts: *wchar_t) {
        unsafe { wxLog_SetTimestamp(self, ts) }
    }
    #[fixed_stack_segment]
    fn setTraceMask(&self, ulMask: int) {
        unsafe { wxLog_SetTraceMask(self, ulMask) }
    }
    #[fixed_stack_segment]
    fn setVerbose(&self, bVerbose: c_int) {
        unsafe { wxLog_SetVerbose(self, bVerbose) }
    }
    #[fixed_stack_segment]
    fn suspend(&self) {
        unsafe { wxLog_Suspend(self) }
    }
}
trait wxLogChain {
    #[fixed_stack_segment]
    fn new(logger: @wxLog) -> @wxLogChain {
        unsafe { wxLogChain_Create(logger) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLogChain_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getOldLog(&self) -> @wxLog {
        unsafe { wxLogChain_GetOldLog(self) }
    }
    #[fixed_stack_segment]
    fn isPassingMessages(&self) -> bool {
        unsafe { wxLogChain_IsPassingMessages(self) }
    }
    #[fixed_stack_segment]
    fn passMessages(&self, bDoPass: bool) {
        unsafe { wxLogChain_PassMessages(self, bDoPass) }
    }
    #[fixed_stack_segment]
    fn setLog(&self, logger: @wxLog) {
        unsafe { wxLogChain_SetLog(self, logger) }
    }
}
trait wxLogGUI {
}
trait wxLogNull {
    #[fixed_stack_segment]
    fn new() -> @wxLogNull {
        unsafe { wxLogNull_Create() }
    }
}
trait wxLogPassThrough {
}
trait wxLogStderr {
    #[fixed_stack_segment]
    fn new() -> @wxLogStderr {
        unsafe { wxLogStderr_Create() }
    }
    #[fixed_stack_segment]
    fn newStdOut() -> @wxLogStderr {
        unsafe { wxLogStderr_CreateStdOut() }
    }
}
trait wxLogStream {
}
trait wxLogTextCtrl {
    #[fixed_stack_segment]
    fn new(text: @wxTextCtrl) -> @wxLogTextCtrl {
        unsafe { wxLogTextCtrl_Create(text) }
    }
}
trait wxLogWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, title: *wchar_t, showit: bool, passthrough: bool) -> @wxLogWindow {
        unsafe { wxLogWindow_Create(parent, title, showit, passthrough) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { wxLogWindow_GetFrame(self) }
    }
}
trait wxLongLong {
}
trait wxMBConv {
}
trait wxMBConvFile {
}
trait wxMBConvUTF7 {
}
trait wxMBConvUTF8 {
}
trait wxMDIChildFrame {
    #[fixed_stack_segment]
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxMDIChildFrame {
        unsafe { wxMDIChildFrame_Create(_prt, _id, _txt, _arg3, _stl) }
    }
}
trait wxMDIClientWindow {
}
trait wxMDIParentFrame {
    #[fixed_stack_segment]
    fn activateNext(&self) {
        unsafe { wxMDIParentFrame_ActivateNext(self) }
    }
    #[fixed_stack_segment]
    fn activatePrevious(&self) {
        unsafe { wxMDIParentFrame_ActivatePrevious(self) }
    }
    #[fixed_stack_segment]
    fn arrangeIcons(&self) {
        unsafe { wxMDIParentFrame_ArrangeIcons(self) }
    }
    #[fixed_stack_segment]
    fn cascade(&self) {
        unsafe { wxMDIParentFrame_Cascade(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxMDIParentFrame {
        unsafe { wxMDIParentFrame_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn getActiveChild(&self) -> @wxMDIChildFrame {
        unsafe { wxMDIParentFrame_GetActiveChild(self) }
    }
    #[fixed_stack_segment]
    fn getClientWindow(&self) -> @wxMDIClientWindow {
        unsafe { wxMDIParentFrame_GetClientWindow(self) }
    }
    #[fixed_stack_segment]
    fn getWindowMenu(&self) -> @wxMenu {
        unsafe { wxMDIParentFrame_GetWindowMenu(self) }
    }
    #[fixed_stack_segment]
    fn onCreateClient(&self) -> @wxMDIClientWindow {
        unsafe { wxMDIParentFrame_OnCreateClient(self) }
    }
    #[fixed_stack_segment]
    fn setWindowMenu(&self, menu: @wxMenu) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self, menu) }
    }
    #[fixed_stack_segment]
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self) }
    }
}
trait wxMask {
    #[fixed_stack_segment]
    fn new(bitmap: @wxBitmap) -> @wxMask {
        unsafe { wxMask_Create(bitmap) }
    }
    #[fixed_stack_segment]
    fn newColoured(bitmap: @wxBitmap, colour: @wxColour) {
        unsafe { wxMask_CreateColoured(bitmap, colour) }
    }
}
trait wxMaximizeEvent {
}
trait wxMemoryDC {
    #[fixed_stack_segment]
    fn new() -> @wxMemoryDC {
        unsafe { wxMemoryDC_Create() }
    }
    #[fixed_stack_segment]
    fn newCompatible(dc: @wxDC) -> @wxMemoryDC {
        unsafe { wxMemoryDC_CreateCompatible(dc) }
    }
    #[fixed_stack_segment]
    fn newWithBitmap(bitmap: @wxBitmap) -> @wxMemoryDC {
        unsafe { wxMemoryDC_CreateWithBitmap(bitmap) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMemoryDC_Delete(self) }
    }
    #[fixed_stack_segment]
    fn selectObject(&self, bitmap: @wxBitmap) {
        unsafe { wxMemoryDC_SelectObject(self, bitmap) }
    }
}
trait wxMemoryFSHandler {
}
trait wxMemoryInputStream {
}
trait wxMemoryOutputStream {
}
trait wxMenu {
    #[fixed_stack_segment]
    fn append(&self, id: int, text: @wxString, help: @wxString, isCheckable: bool) {
        unsafe { wxMenu_Append(self, id, text, help, isCheckable) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_AppendItem(self, _itm) }
    }
    #[fixed_stack_segment]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self) }
    }
    #[fixed_stack_segment]
    fn appendSub(&self, id: int, text: @wxString, submenu: @wxMenu, help: @wxString) {
        unsafe { wxMenu_AppendSub(self, id, text, submenu, help) }
    }
    #[fixed_stack_segment]
    fn break_(&self) {
        unsafe { wxMenu_Break(self) }
    }
    #[fixed_stack_segment]
    fn check(&self, id: int, check: bool) {
        unsafe { wxMenu_Check(self, id, check) }
    }
    #[fixed_stack_segment]
    fn new(title: @wxString, style: c_long) -> @wxMenu {
        unsafe { wxMenu_Create(title, style) }
    }
    #[fixed_stack_segment]
    fn deleteById(&self, id: int) {
        unsafe { wxMenu_DeleteById(self, id) }
    }
    #[fixed_stack_segment]
    fn deleteByItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_DeleteByItem(self, _itm) }
    }
    #[fixed_stack_segment]
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self) }
    }
    #[fixed_stack_segment]
    fn destroyById(&self, id: int) {
        unsafe { wxMenu_DestroyById(self, id) }
    }
    #[fixed_stack_segment]
    fn destroyByItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_DestroyByItem(self, _itm) }
    }
    #[fixed_stack_segment]
    fn enable(&self, id: int, enable: bool) {
        unsafe { wxMenu_Enable(self, id, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: int) -> @wxMenuItem {
        unsafe { wxMenu_FindItem(self, id) }
    }
    #[fixed_stack_segment]
    fn findItemByLabel(&self, itemString: @wxString) -> int {
        unsafe { wxMenu_FindItemByLabel(self, itemString) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @wxClientData {
        unsafe { wxMenu_GetClientData(self) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: int) -> @wxString {
        unsafe { wxMenu_GetHelpString(self, id) }
    }
    #[fixed_stack_segment]
    fn getInvokingWindow(&self) -> @wxWindow {
        unsafe { wxMenu_GetInvokingWindow(self) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: int) -> @wxString {
        unsafe { wxMenu_GetLabel(self, id) }
    }
    #[fixed_stack_segment]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self) }
    }
    #[fixed_stack_segment]
    fn getMenuItems(&self, _lst: @wxList) -> int {
        unsafe { wxMenu_GetMenuItems(self, _lst) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @wxMenu {
        unsafe { wxMenu_GetParent(self) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxMenu_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { wxMenu_GetTitle(self) }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: size_t, id: int, text: @wxString, help: @wxString, isCheckable: bool) {
        unsafe { wxMenu_Insert(self, pos, id, text, help, isCheckable) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, pos: size_t, _itm: @wxMenuItem) {
        unsafe { wxMenu_InsertItem(self, pos, _itm) }
    }
    #[fixed_stack_segment]
    fn insertSub(&self, pos: size_t, id: int, text: @wxString, submenu: @wxMenu, help: @wxString) {
        unsafe { wxMenu_InsertSub(self, pos, id, text, submenu, help) }
    }
    #[fixed_stack_segment]
    fn isAttached(&self) -> bool {
        unsafe { wxMenu_IsAttached(self) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self, id: int) -> bool {
        unsafe { wxMenu_IsChecked(self, id) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self, id: int) -> bool {
        unsafe { wxMenu_IsEnabled(self, id) }
    }
    #[fixed_stack_segment]
    fn prepend(&self, id: int, text: @wxString, help: @wxString, isCheckable: bool) {
        unsafe { wxMenu_Prepend(self, id, text, help, isCheckable) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, _itm: @wxMenuItem) {
        unsafe { wxMenu_PrependItem(self, _itm) }
    }
    #[fixed_stack_segment]
    fn prependSub(&self, id: int, text: @wxString, submenu: @wxMenu, help: @wxString) {
        unsafe { wxMenu_PrependSub(self, id, text, submenu, help) }
    }
    #[fixed_stack_segment]
    fn removeById(&self, id: int, _itm: @wxMenuItem) {
        unsafe { wxMenu_RemoveById(self, id, _itm) }
    }
    #[fixed_stack_segment]
    fn removeByItem(&self, item: *c_void) {
        unsafe { wxMenu_RemoveByItem(self, item) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, clientData: @wxClientData) {
        unsafe { wxMenu_SetClientData(self, clientData) }
    }
    #[fixed_stack_segment]
    fn setEventHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxMenu_SetEventHandler(self, handler) }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: int, helpString: @wxString) {
        unsafe { wxMenu_SetHelpString(self, id, helpString) }
    }
    #[fixed_stack_segment]
    fn setInvokingWindow(&self, win: @wxWindow) {
        unsafe { wxMenu_SetInvokingWindow(self, win) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, id: int, label: @wxString) {
        unsafe { wxMenu_SetLabel(self, id, label) }
    }
    #[fixed_stack_segment]
    fn setParent(&self, parent: @wxWindow) {
        unsafe { wxMenu_SetParent(self, parent) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, title: @wxString) {
        unsafe { wxMenu_SetTitle(self, title) }
    }
    #[fixed_stack_segment]
    fn updateUI(&self, source: *c_void) {
        unsafe { wxMenu_UpdateUI(self, source) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> @wxMenuBar {
        unsafe { wxMenu_GetMenuBar(self) }
    }
    #[fixed_stack_segment]
    fn appendRadioItem(&self, id: int, text: @wxString, help: @wxString) {
        unsafe { wxMenu_AppendRadioItem(self, id, text, help) }
    }
}
trait wxMenuBar {
    #[fixed_stack_segment]
    fn append(&self, menu: @wxMenu, title: @wxString) -> int {
        unsafe { wxMenuBar_Append(self, menu, title) }
    }
    #[fixed_stack_segment]
    fn check(&self, id: int, check: bool) {
        unsafe { wxMenuBar_Check(self, id, check) }
    }
    #[fixed_stack_segment]
    fn new(_style: int) -> @wxMenuBar {
        unsafe { wxMenuBar_Create(_style) }
    }
    #[fixed_stack_segment]
    fn deletePointer(&self) {
        unsafe { wxMenuBar_DeletePointer(self) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> int {
        unsafe { wxMenuBar_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn enableItem(&self, id: int, enable: bool) {
        unsafe { wxMenuBar_EnableItem(self, id, enable) }
    }
    #[fixed_stack_segment]
    fn enableTop(&self, pos: int, enable: bool) {
        unsafe { wxMenuBar_EnableTop(self, pos, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: int) -> @wxMenuItem {
        unsafe { wxMenuBar_FindItem(self, id) }
    }
    #[fixed_stack_segment]
    fn findMenu(&self, title: @wxString) -> int {
        unsafe { wxMenuBar_FindMenu(self, title) }
    }
    #[fixed_stack_segment]
    fn findMenuItem(&self, menuString: @wxString, itemString: @wxString) -> int {
        unsafe { wxMenuBar_FindMenuItem(self, menuString, itemString) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: int) -> @wxString {
        unsafe { wxMenuBar_GetHelpString(self, id) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: int) -> @wxString {
        unsafe { wxMenuBar_GetLabel(self, id) }
    }
    #[fixed_stack_segment]
    fn getLabelTop(&self, pos: int) -> @wxString {
        unsafe { wxMenuBar_GetLabelTop(self, pos) }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, pos: int) -> @wxMenu {
        unsafe { wxMenuBar_GetMenu(self, pos) }
    }
    #[fixed_stack_segment]
    fn getMenuCount(&self) -> int {
        unsafe { wxMenuBar_GetMenuCount(self) }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: int, menu: @wxMenu, title: @wxString) -> int {
        unsafe { wxMenuBar_Insert(self, pos, menu, title) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self, id: int) -> bool {
        unsafe { wxMenuBar_IsChecked(self, id) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self, id: int) -> bool {
        unsafe { wxMenuBar_IsEnabled(self, id) }
    }
    #[fixed_stack_segment]
    fn remove(&self, pos: int) -> @wxMenu {
        unsafe { wxMenuBar_Remove(self, pos) }
    }
    #[fixed_stack_segment]
    fn replace(&self, pos: int, menu: @wxMenu, title: @wxString) -> @wxMenu {
        unsafe { wxMenuBar_Replace(self, pos, menu, title) }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: int, helpString: @wxString) {
        unsafe { wxMenuBar_SetHelpString(self, id, helpString) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, id: int, label: @wxString) {
        unsafe { wxMenuBar_SetItemLabel(self, id, label) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, s: @wxString) {
        unsafe { wxMenuBar_SetLabel(self, s) }
    }
    #[fixed_stack_segment]
    fn setLabelTop(&self, pos: int, label: @wxString) {
        unsafe { wxMenuBar_SetLabelTop(self, pos, label) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { wxMenuBar_GetFrame(self) }
    }
}
trait wxMenuEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxMenuEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getMenuId(&self) -> int {
        unsafe { wxMenuEvent_GetMenuId(self) }
    }
}
trait wxMenuItem {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxMenuItem_Check(self, check) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxMenuItem {
        unsafe { wxMenuItem_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMenuItem_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) {
        unsafe { wxMenuItem_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn getHelp(&self) -> @wxString {
        unsafe { wxMenuItem_GetHelp(self) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> int {
        unsafe { wxMenuItem_GetId(self) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { wxMenuItem_GetLabel(self) }
    }
    #[fixed_stack_segment]
    fn getLabelFromText(text: *wchar_t) -> @wxString {
        unsafe { wxMenuItem_GetLabelFromText(text) }
    }
    #[fixed_stack_segment]
    fn getMenu(&self) -> @wxMenu {
        unsafe { wxMenuItem_GetMenu(self) }
    }
    #[fixed_stack_segment]
    fn getSubMenu(&self) -> @wxMenu {
        unsafe { wxMenuItem_GetSubMenu(self) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { wxMenuItem_GetText(self) }
    }
    #[fixed_stack_segment]
    fn isCheckable(&self) -> bool {
        unsafe { wxMenuItem_IsCheckable(self) }
    }
    #[fixed_stack_segment]
    fn isChecked(&self) -> bool {
        unsafe { wxMenuItem_IsChecked(self) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self) -> bool {
        unsafe { wxMenuItem_IsEnabled(self) }
    }
    #[fixed_stack_segment]
    fn isSeparator(&self) -> bool {
        unsafe { wxMenuItem_IsSeparator(self) }
    }
    #[fixed_stack_segment]
    fn isSubMenu(&self) -> bool {
        unsafe { wxMenuItem_IsSubMenu(self) }
    }
    #[fixed_stack_segment]
    fn setCheckable(&self, checkable: bool) {
        unsafe { wxMenuItem_SetCheckable(self, checkable) }
    }
    #[fixed_stack_segment]
    fn setHelp(&self, str: @wxString) {
        unsafe { wxMenuItem_SetHelp(self, str) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: int) {
        unsafe { wxMenuItem_SetId(self, id) }
    }
    #[fixed_stack_segment]
    fn setSubMenu(&self, menu: @wxMenu) {
        unsafe { wxMenuItem_SetSubMenu(self, menu) }
    }
    #[fixed_stack_segment]
    fn setText(&self, str: @wxString) {
        unsafe { wxMenuItem_SetText(self, str) }
    }
    #[fixed_stack_segment]
    fn newSeparator() -> @wxMenuItem {
        unsafe { wxMenuItem_CreateSeparator() }
    }
    #[fixed_stack_segment]
    fn newEx(id: int, label: @wxString, help: @wxString, itemkind: int, submenu: @wxMenu) -> @wxMenuItem {
        unsafe { wxMenuItem_CreateEx(id, label, help, itemkind, submenu) }
    }
}
trait wxMessageDialog {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _msg: @wxString, _cap: @wxString, _stl: int) -> @wxMessageDialog {
        unsafe { wxMessageDialog_Create(_prt, _msg, _cap, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMessageDialog_Delete(self) }
    }
    #[fixed_stack_segment]
    fn showModal(&self) -> int {
        unsafe { wxMessageDialog_ShowModal(self) }
    }
}
trait wxMetafile {
    #[fixed_stack_segment]
    fn new(_file: @wxString) -> @wxMetafile {
        unsafe { wxMetafile_Create(_file) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafile_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxMetafile_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn play(&self, _dc: @wxDC) -> bool {
        unsafe { wxMetafile_Play(self, _dc) }
    }
    #[fixed_stack_segment]
    fn setClipboard(&self, width: c_int, height: c_int) -> bool {
        unsafe { wxMetafile_SetClipboard(self, _arg1) }
    }
}
trait wxMetafileDC {
    #[fixed_stack_segment]
    fn close(&self) {
        unsafe { wxMetafileDC_Close(self) }
    }
    #[fixed_stack_segment]
    fn new(_file: @wxString) -> @wxMetafileDC {
        unsafe { wxMetafileDC_Create(_file) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafileDC_Delete(self) }
    }
}
trait wxMimeTypesManager {
    #[fixed_stack_segment]
    fn addFallbacks(&self, _types: *c_void) {
        unsafe { wxMimeTypesManager_AddFallbacks(self, _types) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxMimeTypesManager {
        unsafe { wxMimeTypesManager_Create() }
    }
    #[fixed_stack_segment]
    fn enumAllFileTypes(&self, _lst: @wxList) -> int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self, _lst) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromExtension(&self, _ext: @wxString) -> @wxFileType {
        unsafe { wxMimeTypesManager_GetFileTypeFromExtension(self, _ext) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromMimeType(&self, _name: @wxString) -> @wxFileType {
        unsafe { wxMimeTypesManager_GetFileTypeFromMimeType(self, _name) }
    }
    #[fixed_stack_segment]
    fn isOfType(&self, _type: @wxString, _wildcard: @wxString) -> bool {
        unsafe { wxMimeTypesManager_IsOfType(self, _type, _wildcard) }
    }
}
trait wxMiniFrame {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxMiniFrame {
        unsafe { wxMiniFrame_Create(_prt, _id, _txt, _arg3, _stl) }
    }
}
trait wxMirrorDC {
    #[fixed_stack_segment]
    fn new(dc: @wxDC) -> @wxMirrorDC {
        unsafe { wxMirrorDC_Create(dc) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMirrorDC_Delete(self) }
    }
}
trait wxModule {
}
trait wxMouseCaptureChangedEvent {
}
trait wxMouseEvent {
    #[fixed_stack_segment]
    fn altDown(&self) -> bool {
        unsafe { wxMouseEvent_AltDown(self) }
    }
    #[fixed_stack_segment]
    fn button(&self, but: int) -> bool {
        unsafe { wxMouseEvent_Button(self, but) }
    }
    #[fixed_stack_segment]
    fn buttonDClick(&self, but: int) -> bool {
        unsafe { wxMouseEvent_ButtonDClick(self, but) }
    }
    #[fixed_stack_segment]
    fn buttonDown(&self, but: int) -> bool {
        unsafe { wxMouseEvent_ButtonDown(self, but) }
    }
    #[fixed_stack_segment]
    fn buttonIsDown(&self, but: int) -> bool {
        unsafe { wxMouseEvent_ButtonIsDown(self, but) }
    }
    #[fixed_stack_segment]
    fn buttonUp(&self, but: int) -> bool {
        unsafe { wxMouseEvent_ButtonUp(self, but) }
    }
    #[fixed_stack_segment]
    fn controlDown(&self) -> bool {
        unsafe { wxMouseEvent_ControlDown(self) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *c_void) {
        unsafe { wxMouseEvent_CopyObject(self, object_dest) }
    }
    #[fixed_stack_segment]
    fn dragging(&self) -> bool {
        unsafe { wxMouseEvent_Dragging(self) }
    }
    #[fixed_stack_segment]
    fn entering(&self) -> bool {
        unsafe { wxMouseEvent_Entering(self) }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self, dc: @wxDC) -> @wxPoint {
        unsafe { wxMouseEvent_GetLogicalPosition(self, dc) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxMouseEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> int {
        unsafe { wxMouseEvent_GetX(self) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> int {
        unsafe { wxMouseEvent_GetY(self) }
    }
    #[fixed_stack_segment]
    fn isButton(&self) -> bool {
        unsafe { wxMouseEvent_IsButton(self) }
    }
    #[fixed_stack_segment]
    fn leaving(&self) -> bool {
        unsafe { wxMouseEvent_Leaving(self) }
    }
    #[fixed_stack_segment]
    fn leftDClick(&self) -> bool {
        unsafe { wxMouseEvent_LeftDClick(self) }
    }
    #[fixed_stack_segment]
    fn leftDown(&self) -> bool {
        unsafe { wxMouseEvent_LeftDown(self) }
    }
    #[fixed_stack_segment]
    fn leftIsDown(&self) -> bool {
        unsafe { wxMouseEvent_LeftIsDown(self) }
    }
    #[fixed_stack_segment]
    fn leftUp(&self) -> bool {
        unsafe { wxMouseEvent_LeftUp(self) }
    }
    #[fixed_stack_segment]
    fn metaDown(&self) -> bool {
        unsafe { wxMouseEvent_MetaDown(self) }
    }
    #[fixed_stack_segment]
    fn middleDClick(&self) -> bool {
        unsafe { wxMouseEvent_MiddleDClick(self) }
    }
    #[fixed_stack_segment]
    fn middleDown(&self) -> bool {
        unsafe { wxMouseEvent_MiddleDown(self) }
    }
    #[fixed_stack_segment]
    fn middleIsDown(&self) -> bool {
        unsafe { wxMouseEvent_MiddleIsDown(self) }
    }
    #[fixed_stack_segment]
    fn middleUp(&self) -> bool {
        unsafe { wxMouseEvent_MiddleUp(self) }
    }
    #[fixed_stack_segment]
    fn moving(&self) -> bool {
        unsafe { wxMouseEvent_Moving(self) }
    }
    #[fixed_stack_segment]
    fn rightDClick(&self) -> bool {
        unsafe { wxMouseEvent_RightDClick(self) }
    }
    #[fixed_stack_segment]
    fn rightDown(&self) -> bool {
        unsafe { wxMouseEvent_RightDown(self) }
    }
    #[fixed_stack_segment]
    fn rightIsDown(&self) -> bool {
        unsafe { wxMouseEvent_RightIsDown(self) }
    }
    #[fixed_stack_segment]
    fn rightUp(&self) -> bool {
        unsafe { wxMouseEvent_RightUp(self) }
    }
    #[fixed_stack_segment]
    fn shiftDown(&self) -> bool {
        unsafe { wxMouseEvent_ShiftDown(self) }
    }
    #[fixed_stack_segment]
    fn getWheelDelta(&self) -> int {
        unsafe { wxMouseEvent_GetWheelDelta(self) }
    }
    #[fixed_stack_segment]
    fn getWheelRotation(&self) -> int {
        unsafe { wxMouseEvent_GetWheelRotation(self) }
    }
    #[fixed_stack_segment]
    fn getButton(&self) -> int {
        unsafe { wxMouseEvent_GetButton(self) }
    }
}
trait wxMoveEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxMoveEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxMoveEvent_GetPosition(self) }
    }
}
trait wxMultiCellCanvas {
    #[fixed_stack_segment]
    fn add(&self, win: @wxWindow, row: int, col: int) {
        unsafe { wxMultiCellCanvas_Add(self, win, row, col) }
    }
    #[fixed_stack_segment]
    fn calculateConstraints(&self) {
        unsafe { wxMultiCellCanvas_CalculateConstraints(self) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, numRows: int, numCols: int) -> @wxMultiCellCanvas {
        unsafe { wxMultiCellCanvas_Create(parent, numRows, numCols) }
    }
    #[fixed_stack_segment]
    fn maxCols(&self) -> int {
        unsafe { wxMultiCellCanvas_MaxCols(self) }
    }
    #[fixed_stack_segment]
    fn maxRows(&self) -> int {
        unsafe { wxMultiCellCanvas_MaxRows(self) }
    }
    #[fixed_stack_segment]
    fn setMinCellSize(&self, w: c_int, h: c_int) {
        unsafe { wxMultiCellCanvas_SetMinCellSize(self, _arg1) }
    }
}
trait wxMultiCellItemHandle {
    #[fixed_stack_segment]
    fn new(row: int, column: int, height: int, width: int, sx: int, sy: int, style: int, wx: int, wy: int, align: int) -> @wxMultiCellItemHandle {
        unsafe { wxMultiCellItemHandle_Create(row, column, height, width, sx, sy, style, wx, wy, align) }
    }
    #[fixed_stack_segment]
    fn newWithSize(&self, row: int, column: int, sx: int, sy: int, style: int, wx: int, wy: int, align: int) {
        unsafe { wxMultiCellItemHandle_CreateWithSize(self, row, column, sx, sy, style, wx, wy, align) }
    }
    #[fixed_stack_segment]
    fn newWithStyle(&self, row: int, column: int, style: int, wx: int, wy: int, align: int) {
        unsafe { wxMultiCellItemHandle_CreateWithStyle(self, row, column, style, wx, wy, align) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> int {
        unsafe { wxMultiCellItemHandle_GetAlignment(self) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self) -> int {
        unsafe { wxMultiCellItemHandle_GetColumn(self) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> int {
        unsafe { wxMultiCellItemHandle_GetHeight(self) }
    }
    #[fixed_stack_segment]
    fn getLocalSize(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxMultiCellItemHandle_GetLocalSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getRow(&self) -> int {
        unsafe { wxMultiCellItemHandle_GetRow(self) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxMultiCellItemHandle_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn getWeight(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxMultiCellItemHandle_GetWeight(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxMultiCellItemHandle_GetWidth(self) }
    }
}
trait wxMultiCellSizer {
    #[fixed_stack_segment]
    fn calcMin(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxMultiCellSizer_CalcMin(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn new(rows: int, cols: int) -> @wxMultiCellSizer {
        unsafe { wxMultiCellSizer_Create(rows, cols) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMultiCellSizer_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enableGridLines(&self, win: @wxWindow) -> int {
        unsafe { wxMultiCellSizer_EnableGridLines(self, win) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxMultiCellSizer_RecalcSizes(self) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, column: int, colSize: int, expandable: int) -> int {
        unsafe { wxMultiCellSizer_SetColumnWidth(self, column, colSize, expandable) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellSize(&self, w: c_int, h: c_int) -> int {
        unsafe { wxMultiCellSizer_SetDefaultCellSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setGridPen(&self, pen: @wxPen) -> int {
        unsafe { wxMultiCellSizer_SetGridPen(self, pen) }
    }
    #[fixed_stack_segment]
    fn setRowHeight(&self, row: int, rowSize: int, expandable: int) -> int {
        unsafe { wxMultiCellSizer_SetRowHeight(self, row, rowSize, expandable) }
    }
}
trait wxMutex {
    #[fixed_stack_segment]
    fn new() -> @wxMutex {
        unsafe { wxMutex_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMutex_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isLocked(&self) -> bool {
        unsafe { wxMutex_IsLocked(self) }
    }
    #[fixed_stack_segment]
    fn lock(&self) -> int {
        unsafe { wxMutex_Lock(self) }
    }
    #[fixed_stack_segment]
    fn tryLock(&self) -> int {
        unsafe { wxMutex_TryLock(self) }
    }
    #[fixed_stack_segment]
    fn unlock(&self) -> int {
        unsafe { wxMutex_Unlock(self) }
    }
}
trait wxMutexLocker {
}
trait wxNavigationKeyEvent {
    #[fixed_stack_segment]
    fn getCurrentFocus(&self) {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self) }
    }
    #[fixed_stack_segment]
    fn getDirection(&self) -> bool {
        unsafe { wxNavigationKeyEvent_GetDirection(self) }
    }
    #[fixed_stack_segment]
    fn isWindowChange(&self) -> bool {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self) }
    }
    #[fixed_stack_segment]
    fn setCurrentFocus(&self, win: @wxWindow) {
        unsafe { wxNavigationKeyEvent_SetCurrentFocus(self, win) }
    }
    #[fixed_stack_segment]
    fn setDirection(&self, bForward: bool) {
        unsafe { wxNavigationKeyEvent_SetDirection(self, bForward) }
    }
    #[fixed_stack_segment]
    fn setWindowChange(&self, bIs: bool) {
        unsafe { wxNavigationKeyEvent_SetWindowChange(self, bIs) }
    }
    #[fixed_stack_segment]
    fn shouldPropagate(&self) -> int {
        unsafe { wxNavigationKeyEvent_ShouldPropagate(self) }
    }
}
trait wxNewBitmapButton {
    #[fixed_stack_segment]
    fn new(labelBitmap: *c_void, labelText: *c_void, alignText: int, isFlat: bool, firedEventType: int, marginX: int, marginY: int, textToLabelGap: int, isSticky: bool) -> @wxNewBitmapButton {
        unsafe { wxNewBitmapButton_Create(labelBitmap, labelText, alignText, isFlat, firedEventType, marginX, marginY, textToLabelGap, isSticky) }
    }
    #[fixed_stack_segment]
    fn newFromFile(&self, bitmapFileType: int, labelText: *c_void, alignText: int, isFlat: bool, firedEventType: int, marginX: int, marginY: int, textToLabelGap: int, isSticky: bool) -> @wxNewBitmapButton {
        unsafe { wxNewBitmapButton_CreateFromFile(self, bitmapFileType, labelText, alignText, isFlat, firedEventType, marginX, marginY, textToLabelGap, isSticky) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxNewBitmapButton_Delete(self) }
    }
    #[fixed_stack_segment]
    fn drawDecorations(&self, dc: @wxDC) {
        unsafe { wxNewBitmapButton_DrawDecorations(self, dc) }
    }
    #[fixed_stack_segment]
    fn drawLabel(&self, dc: @wxDC) {
        unsafe { wxNewBitmapButton_DrawLabel(self, dc) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> int {
        unsafe { wxNewBitmapButton_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn realize(&self, _prt: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxNewBitmapButton_Realize(self, _prt, _id, _arg3) }
    }
    #[fixed_stack_segment]
    fn renderAllLabelImages(&self) {
        unsafe { wxNewBitmapButton_RenderAllLabelImages(self) }
    }
    #[fixed_stack_segment]
    fn renderLabelImage(&self, destBmp: *c_void, srcBmp: *c_void, isEnabled: bool, isPressed: bool) {
        unsafe { wxNewBitmapButton_RenderLabelImage(self, destBmp, srcBmp, isEnabled, isPressed) }
    }
    #[fixed_stack_segment]
    fn renderLabelImages(&self) {
        unsafe { wxNewBitmapButton_RenderLabelImages(self) }
    }
    #[fixed_stack_segment]
    fn reshape(&self) {
        unsafe { wxNewBitmapButton_Reshape(self) }
    }
    #[fixed_stack_segment]
    fn setAlignments(&self, alignText: int, marginX: int, marginY: int, textToLabelGap: int) {
        unsafe { wxNewBitmapButton_SetAlignments(self, alignText, marginX, marginY, textToLabelGap) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, labelBitmap: *c_void, labelText: *c_void) {
        unsafe { wxNewBitmapButton_SetLabel(self, labelBitmap, labelText) }
    }
}
trait wxNodeBase {
}
trait wxNotebook {
    #[fixed_stack_segment]
    fn addPage(&self, pPage: @wxWindow, strText: @wxString, bSelect: bool, imageId: int) -> bool {
        unsafe { wxNotebook_AddPage(self, pPage, strText, bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn advanceSelection(&self, bForward: bool) {
        unsafe { wxNotebook_AdvanceSelection(self, bForward) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxNotebook {
        unsafe { wxNotebook_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn deleteAllPages(&self) -> bool {
        unsafe { wxNotebook_DeleteAllPages(self) }
    }
    #[fixed_stack_segment]
    fn deletePage(&self, nPage: int) -> bool {
        unsafe { wxNotebook_DeletePage(self, nPage) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> @wxImageList {
        unsafe { wxNotebook_GetImageList(self) }
    }
    #[fixed_stack_segment]
    fn getPage(&self, nPage: int) -> @wxWindow {
        unsafe { wxNotebook_GetPage(self, nPage) }
    }
    #[fixed_stack_segment]
    fn getPageCount(&self) -> int {
        unsafe { wxNotebook_GetPageCount(self) }
    }
    #[fixed_stack_segment]
    fn getPageImage(&self, nPage: int) -> int {
        unsafe { wxNotebook_GetPageImage(self, nPage) }
    }
    #[fixed_stack_segment]
    fn getPageText(&self, nPage: int) -> @wxString {
        unsafe { wxNotebook_GetPageText(self, nPage) }
    }
    #[fixed_stack_segment]
    fn getRowCount(&self) -> int {
        unsafe { wxNotebook_GetRowCount(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> int {
        unsafe { wxNotebook_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, x: c_int, y: c_int, flags: *c_long) -> int {
        unsafe { wxNotebook_HitTest(self, _arg1, flags) }
    }
    #[fixed_stack_segment]
    fn insertPage(&self, nPage: int, pPage: @wxWindow, strText: @wxString, bSelect: bool, imageId: int) -> bool {
        unsafe { wxNotebook_InsertPage(self, nPage, pPage, strText, bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn removePage(&self, nPage: int) -> bool {
        unsafe { wxNotebook_RemovePage(self, nPage) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @wxImageList) {
        unsafe { wxNotebook_SetImageList(self, imageList) }
    }
    #[fixed_stack_segment]
    fn setPadding(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPadding(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPageImage(&self, nPage: int, nImage: int) -> bool {
        unsafe { wxNotebook_SetPageImage(self, nPage, nImage) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, _w: c_int, _h: c_int) {
        unsafe { wxNotebook_SetPageSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPageText(&self, nPage: int, strText: @wxString) -> bool {
        unsafe { wxNotebook_SetPageText(self, nPage, strText) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, nPage: int) -> int {
        unsafe { wxNotebook_SetSelection(self, nPage) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @wxImageList) {
        unsafe { wxNotebook_AssignImageList(self, imageList) }
    }
}
trait wxNotebookEvent {
}
trait wxNotifyEvent {
    #[fixed_stack_segment]
    fn allow(&self) {
        unsafe { wxNotifyEvent_Allow(self) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *c_void) {
        unsafe { wxNotifyEvent_CopyObject(self, object_dest) }
    }
    #[fixed_stack_segment]
    fn isAllowed(&self) -> bool {
        unsafe { wxNotifyEvent_IsAllowed(self) }
    }
    #[fixed_stack_segment]
    fn veto(&self) {
        unsafe { wxNotifyEvent_Veto(self) }
    }
}
trait wxObject {
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { wxObject_GetClientClosure(self) }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @wxClosure) {
        unsafe { wxObject_SetClientClosure(self, closure) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxObject_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getClassInfo(&self) -> @wxClassInfo {
        unsafe { wxObject_GetClassInfo(self) }
    }
    #[fixed_stack_segment]
    fn isKindOf(&self, classInfo: @wxClassInfo) -> bool {
        unsafe { wxObject_IsKindOf(self, classInfo) }
    }
    #[fixed_stack_segment]
    fn isScrolledWindow(&self) -> bool {
        unsafe { wxObject_IsScrolledWindow(self) }
    }
}
trait wxObjectRefData {
}
trait wxOutputStream {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxOutputStream_Delete(self) }
    }
    #[fixed_stack_segment]
    fn lastWrite(&self) -> int {
        unsafe { wxOutputStream_LastWrite(self) }
    }
    #[fixed_stack_segment]
    fn putC(&self, c: wchar_t) {
        unsafe { wxOutputStream_PutC(self, c) }
    }
    #[fixed_stack_segment]
    fn seek(&self, pos: int, mode: int) -> int {
        unsafe { wxOutputStream_Seek(self, pos, mode) }
    }
    #[fixed_stack_segment]
    fn sync(&self) {
        unsafe { wxOutputStream_Sync(self) }
    }
    #[fixed_stack_segment]
    fn tell(&self) -> int {
        unsafe { wxOutputStream_Tell(self) }
    }
    #[fixed_stack_segment]
    fn write(&self, buffer: *c_void, size: int) {
        unsafe { wxOutputStream_Write(self, buffer, size) }
    }
}
trait wxPageSetupDialog {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, data: @wxPageSetupDialogData) -> @wxPageSetupDialog {
        unsafe { wxPageSetupDialog_Create(parent, data) }
    }
    #[fixed_stack_segment]
    fn getPageSetupData(&self, @wxPageSetupDialogData) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self, _ref) }
    }
}
trait wxPageSetupDialogData {
    #[fixed_stack_segment]
    fn assign(&self, @wxPageSetupDialogData) {
        unsafe { wxPageSetupDialogData_Assign(self, data) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, printData: @wxPrintData) {
        unsafe { wxPageSetupDialogData_AssignData(self, printData) }
    }
    #[fixed_stack_segment]
    fn calculateIdFromPaperSize(&self) {
        unsafe { wxPageSetupDialogData_CalculateIdFromPaperSize(self) }
    }
    #[fixed_stack_segment]
    fn calculatePaperSizeFromId(&self) {
        unsafe { wxPageSetupDialogData_CalculatePaperSizeFromId(self) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxPageSetupDialogData {
        unsafe { wxPageSetupDialogData_Create() }
    }
    #[fixed_stack_segment]
    fn newFromData(printData: @wxPrintData) -> @wxPageSetupDialogData {
        unsafe { wxPageSetupDialogData_CreateFromData(printData) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPageSetupDialogData_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enableHelp(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableHelp(self, flag) }
    }
    #[fixed_stack_segment]
    fn enableMargins(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableMargins(self, flag) }
    }
    #[fixed_stack_segment]
    fn enableOrientation(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnableOrientation(self, flag) }
    }
    #[fixed_stack_segment]
    fn enablePaper(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnablePaper(self, flag) }
    }
    #[fixed_stack_segment]
    fn enablePrinter(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_EnablePrinter(self, flag) }
    }
    #[fixed_stack_segment]
    fn getDefaultInfo(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetDefaultInfo(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultMinMargins(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetDefaultMinMargins(self) }
    }
    #[fixed_stack_segment]
    fn getEnableHelp(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableHelp(self) }
    }
    #[fixed_stack_segment]
    fn getEnableMargins(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableMargins(self) }
    }
    #[fixed_stack_segment]
    fn getEnableOrientation(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnableOrientation(self) }
    }
    #[fixed_stack_segment]
    fn getEnablePaper(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnablePaper(self) }
    }
    #[fixed_stack_segment]
    fn getEnablePrinter(&self) -> bool {
        unsafe { wxPageSetupDialogData_GetEnablePrinter(self) }
    }
    #[fixed_stack_segment]
    fn getMarginBottomRight(&self) -> @wxPoint {
        unsafe { wxPageSetupDialogData_GetMarginBottomRight(self) }
    }
    #[fixed_stack_segment]
    fn getMarginTopLeft(&self) -> @wxPoint {
        unsafe { wxPageSetupDialogData_GetMarginTopLeft(self) }
    }
    #[fixed_stack_segment]
    fn getMinMarginBottomRight(&self) -> @wxPoint {
        unsafe { wxPageSetupDialogData_GetMinMarginBottomRight(self) }
    }
    #[fixed_stack_segment]
    fn getMinMarginTopLeft(&self) -> @wxPoint {
        unsafe { wxPageSetupDialogData_GetMinMarginTopLeft(self) }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> int {
        unsafe { wxPageSetupDialogData_GetPaperId(self) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> @wxSize {
        unsafe { wxPageSetupDialogData_GetPaperSize(self) }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, @wxPrintData) {
        unsafe { wxPageSetupDialogData_GetPrintData(self, _ref) }
    }
    #[fixed_stack_segment]
    fn setDefaultInfo(&self, flag: bool) {
        unsafe { wxPageSetupDialogData_SetDefaultInfo(self, flag) }
    }
    #[fixed_stack_segment]
    fn setDefaultMinMargins(&self, flag: int) {
        unsafe { wxPageSetupDialogData_SetDefaultMinMargins(self, flag) }
    }
    #[fixed_stack_segment]
    fn setMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginBottomRight(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMarginTopLeft(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setMinMarginBottomRight(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginBottomRight(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setMinMarginTopLeft(&self, x: c_int, y: c_int) {
        unsafe { wxPageSetupDialogData_SetMinMarginTopLeft(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPaperId(&self, id: *c_void) {
        unsafe { wxPageSetupDialogData_SetPaperId(self, id) }
    }
    #[fixed_stack_segment]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPageSetupDialogData_SetPaperSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPaperSizeId(&self, id: int) {
        unsafe { wxPageSetupDialogData_SetPaperSizeId(self, id) }
    }
    #[fixed_stack_segment]
    fn setPrintData(&self, printData: @wxPrintData) {
        unsafe { wxPageSetupDialogData_SetPrintData(self, printData) }
    }
}
trait wxPaintDC {
    #[fixed_stack_segment]
    fn new(win: @wxWindow) -> @wxPaintDC {
        unsafe { wxPaintDC_Create(win) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPaintDC_Delete(self) }
    }
}
trait wxPaintEvent {
}
trait wxPalette {
    #[fixed_stack_segment]
    fn assign(&self, palette: @wxPalette) {
        unsafe { wxPalette_Assign(self, palette) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxPalette {
        unsafe { wxPalette_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newRGB(n: int, red: *c_void, green: *c_void, blue: *c_void) -> @wxPalette {
        unsafe { wxPalette_CreateRGB(n, red, green, blue) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPalette_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getPixel(&self, red: u8, green: u8, blue: u8) -> int {
        unsafe { wxPalette_GetPixel(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getRGB(&self, pixel: int, red: *c_void, green: *c_void, blue: *c_void) -> bool {
        unsafe { wxPalette_GetRGB(self, pixel, red, green, blue) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, palette: @wxPalette) -> bool {
        unsafe { wxPalette_IsEqual(self, palette) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPalette_IsOk(self) }
    }
}
trait wxPaletteChangedEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxPaletteChangedEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getChangedWindow(&self) {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self) }
    }
    #[fixed_stack_segment]
    fn setChangedWindow(&self, win: @wxWindow) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self, win) }
    }
}
trait wxPanel {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxPanel {
        unsafe { wxPanel_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn initDialog(&self) {
        unsafe { wxPanel_InitDialog(self) }
    }
    #[fixed_stack_segment]
    fn setFocus(&self) {
        unsafe { wxPanel_SetFocus(self) }
    }
}
trait wxPathList {
}
trait wxPen {
    #[fixed_stack_segment]
    fn assign(&self, pen: @wxPen) {
        unsafe { wxPen_Assign(self, pen) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxPen {
        unsafe { wxPen_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(stipple: @wxBitmap, width: int) -> @wxPen {
        unsafe { wxPen_CreateFromBitmap(stipple, width) }
    }
    #[fixed_stack_segment]
    fn newFromColour(col: @wxColour, width: int, style: int) -> @wxPen {
        unsafe { wxPen_CreateFromColour(col, width, style) }
    }
    #[fixed_stack_segment]
    fn newFromStock(id: int) -> @wxPen {
        unsafe { wxPen_CreateFromStock(id) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPen_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCap(&self) -> int {
        unsafe { wxPen_GetCap(self) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, @wxColour) {
        unsafe { wxPen_GetColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getDashes(&self, ptr: *c_void) -> int {
        unsafe { wxPen_GetDashes(self, ptr) }
    }
    #[fixed_stack_segment]
    fn getJoin(&self) -> int {
        unsafe { wxPen_GetJoin(self) }
    }
    #[fixed_stack_segment]
    fn getStipple(&self, @wxBitmap) {
        unsafe { wxPen_GetStipple(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxPen_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxPen_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, pen: @wxPen) -> bool {
        unsafe { wxPen_IsEqual(self, pen) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPen_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn setCap(&self, cap: int) {
        unsafe { wxPen_SetCap(self, cap) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, col: @wxColour) {
        unsafe { wxPen_SetColour(self, col) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxPen_SetColourSingle(self, r, g, b) }
    }
    #[fixed_stack_segment]
    fn setDashes(&self, nb_dashes: int, dash: *c_void) {
        unsafe { wxPen_SetDashes(self, nb_dashes, dash) }
    }
    #[fixed_stack_segment]
    fn setJoin(&self, join: int) {
        unsafe { wxPen_SetJoin(self, join) }
    }
    #[fixed_stack_segment]
    fn setStipple(&self, stipple: @wxBitmap) {
        unsafe { wxPen_SetStipple(self, stipple) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxPen_SetStyle(self, style) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: int) {
        unsafe { wxPen_SetWidth(self, width) }
    }
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxPen_SafeDelete(self) }
    }
    #[fixed_stack_segment]
    fn isStatic(&self) -> bool {
        unsafe { wxPen_IsStatic(self) }
    }
}
trait wxPenList {
}
trait wxPlotCurve {
}
trait wxPlotEvent {
    #[fixed_stack_segment]
    fn getCurve(&self) {
        unsafe { wxPlotEvent_GetCurve(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> int {
        unsafe { wxPlotEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> c_double {
        unsafe { wxPlotEvent_GetZoom(self) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: int) {
        unsafe { wxPlotEvent_SetPosition(self, pos) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, zoom: c_double) {
        unsafe { wxPlotEvent_SetZoom(self, zoom) }
    }
}
trait wxPlotOnOffCurve {
    #[fixed_stack_segment]
    fn add(&self, on: int, off: int, clientData: @wxClientData) {
        unsafe { wxPlotOnOffCurve_Add(self, on, off, clientData) }
    }
    #[fixed_stack_segment]
    fn new(offsetY: int) -> @wxPlotOnOffCurve {
        unsafe { wxPlotOnOffCurve_Create(offsetY) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPlotOnOffCurve_Delete(self) }
    }
    #[fixed_stack_segment]
    fn drawOffLine(&self, dc: @wxDC, y: int, start: int, end: int) {
        unsafe { wxPlotOnOffCurve_DrawOffLine(self, dc, y, start, end) }
    }
    #[fixed_stack_segment]
    fn drawOnLine(&self, dc: @wxDC, y: int, start: int, end: int, clientData: @wxClientData) {
        unsafe { wxPlotOnOffCurve_DrawOnLine(self, dc, y, start, end, clientData) }
    }
    #[fixed_stack_segment]
    fn getAt(&self, index: int) {
        unsafe { wxPlotOnOffCurve_GetAt(self, index) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, index: int) -> @wxClientData {
        unsafe { wxPlotOnOffCurve_GetClientData(self, index) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxPlotOnOffCurve_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getEndX(&self) -> int {
        unsafe { wxPlotOnOffCurve_GetEndX(self) }
    }
    #[fixed_stack_segment]
    fn getOff(&self, index: int) -> int {
        unsafe { wxPlotOnOffCurve_GetOff(self, index) }
    }
    #[fixed_stack_segment]
    fn getOffsetY(&self) -> int {
        unsafe { wxPlotOnOffCurve_GetOffsetY(self) }
    }
    #[fixed_stack_segment]
    fn getOn(&self, index: int) -> int {
        unsafe { wxPlotOnOffCurve_GetOn(self, index) }
    }
    #[fixed_stack_segment]
    fn getStartX(&self) -> int {
        unsafe { wxPlotOnOffCurve_GetStartX(self) }
    }
    #[fixed_stack_segment]
    fn setOffsetY(&self, offsetY: int) {
        unsafe { wxPlotOnOffCurve_SetOffsetY(self, offsetY) }
    }
}
trait wxPlotWindow {
    #[fixed_stack_segment]
    fn add(&self, curve: @wxPlotCurve) {
        unsafe { wxPlotWindow_Add(self, curve) }
    }
    #[fixed_stack_segment]
    fn addOnOff(&self, curve: @wxPlotCurve) {
        unsafe { wxPlotWindow_AddOnOff(self, curve) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, flags: int) -> @wxPlotWindow {
        unsafe { wxPlotWindow_Create(parent, id, _arg2, flags) }
    }
    #[fixed_stack_segment]
    fn delete(&self, curve: @wxPlotCurve) {
        unsafe { wxPlotWindow_Delete(self, curve) }
    }
    #[fixed_stack_segment]
    fn deleteOnOff(&self, curve: @wxPlotOnOffCurve) {
        unsafe { wxPlotWindow_DeleteOnOff(self, curve) }
    }
    #[fixed_stack_segment]
    fn enlarge(&self, curve: @wxPlotCurve, factor: c_double) {
        unsafe { wxPlotWindow_Enlarge(self, curve, factor) }
    }
    #[fixed_stack_segment]
    fn getAt(&self, n: int) -> @wxPlotCurve {
        unsafe { wxPlotWindow_GetAt(self, n) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxPlotWindow_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getCurrent(&self) -> @wxPlotCurve {
        unsafe { wxPlotWindow_GetCurrent(self) }
    }
    #[fixed_stack_segment]
    fn getEnlargeAroundWindowCentre(&self) -> int {
        unsafe { wxPlotWindow_GetEnlargeAroundWindowCentre(self) }
    }
    #[fixed_stack_segment]
    fn getOnOffCurveAt(&self, n: int) -> @wxPlotOnOffCurve {
        unsafe { wxPlotWindow_GetOnOffCurveAt(self, n) }
    }
    #[fixed_stack_segment]
    fn getOnOffCurveCount(&self) -> int {
        unsafe { wxPlotWindow_GetOnOffCurveCount(self) }
    }
    #[fixed_stack_segment]
    fn getScrollOnThumbRelease(&self) -> int {
        unsafe { wxPlotWindow_GetScrollOnThumbRelease(self) }
    }
    #[fixed_stack_segment]
    fn getUnitsPerValue(&self) -> c_double {
        unsafe { wxPlotWindow_GetUnitsPerValue(self) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> c_double {
        unsafe { wxPlotWindow_GetZoom(self) }
    }
    #[fixed_stack_segment]
    fn move(&self, curve: @wxPlotCurve, pixels_up: int) {
        unsafe { wxPlotWindow_Move(self, curve, pixels_up) }
    }
    #[fixed_stack_segment]
    fn redrawEverything(&self) {
        unsafe { wxPlotWindow_RedrawEverything(self) }
    }
    #[fixed_stack_segment]
    fn redrawXAxis(&self) {
        unsafe { wxPlotWindow_RedrawXAxis(self) }
    }
    #[fixed_stack_segment]
    fn redrawYAxis(&self) {
        unsafe { wxPlotWindow_RedrawYAxis(self) }
    }
    #[fixed_stack_segment]
    fn resetScrollbar(&self) {
        unsafe { wxPlotWindow_ResetScrollbar(self) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, current: @wxPlotCurve) {
        unsafe { wxPlotWindow_SetCurrent(self, current) }
    }
    #[fixed_stack_segment]
    fn setEnlargeAroundWindowCentre(&self, enlargeAroundWindowCentre: int) {
        unsafe { wxPlotWindow_SetEnlargeAroundWindowCentre(self, enlargeAroundWindowCentre) }
    }
    #[fixed_stack_segment]
    fn setScrollOnThumbRelease(&self, scrollOnThumbRelease: int) {
        unsafe { wxPlotWindow_SetScrollOnThumbRelease(self, scrollOnThumbRelease) }
    }
    #[fixed_stack_segment]
    fn setUnitsPerValue(&self, upv: c_double) {
        unsafe { wxPlotWindow_SetUnitsPerValue(self, upv) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, zoom: c_double) {
        unsafe { wxPlotWindow_SetZoom(self, zoom) }
    }
}
trait wxPoint {
    #[fixed_stack_segment]
    fn new(xx: c_int, yy: c_int) -> @wxPoint {
        unsafe { wxPoint_Create(_arg0) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxPoint_Destroy(self) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> int {
        unsafe { wxPoint_GetX(self) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> int {
        unsafe { wxPoint_GetY(self) }
    }
    #[fixed_stack_segment]
    fn setX(&self, w: int) {
        unsafe { wxPoint_SetX(self, w) }
    }
    #[fixed_stack_segment]
    fn setY(&self, h: int) {
        unsafe { wxPoint_SetY(self, h) }
    }
}
trait wxPopupTransientWindow {
}
trait wxPopupWindow {
}
trait wxPostScriptDC {
    #[fixed_stack_segment]
    fn new(data: @wxPrintData) -> @wxPostScriptDC {
        unsafe { wxPostScriptDC_Create(data) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPostScriptDC_Delete(self) }
    }
    #[fixed_stack_segment]
    fn setResolution(&self, ppi: int) {
        unsafe { wxPostScriptDC_SetResolution(self, ppi) }
    }
    #[fixed_stack_segment]
    fn getResolution(&self) -> int {
        unsafe { wxPostScriptDC_GetResolution(self) }
    }
}
trait wxPreviewCanvas {
    #[fixed_stack_segment]
    fn new(preview: @wxPrintPreview, parent: @wxWindow, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxPreviewCanvas {
        unsafe { wxPreviewCanvas_Create(preview, parent, _arg2, style) }
    }
}
trait wxPreviewControlBar {
}
trait wxPreviewFrame {
    #[fixed_stack_segment]
    fn new(preview: @wxPrintPreview, parent: @wxFrame, title: @wxString, x: c_int, y: c_int, width: c_int, height: c_int, style: int, name: @wxString) -> @wxPreviewFrame {
        unsafe { wxPreviewFrame_Create(preview, parent, title, _arg3, style, name) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPreviewFrame_Delete(self) }
    }
    #[fixed_stack_segment]
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self) }
    }
}
trait wxPrintData {
    #[fixed_stack_segment]
    fn assign(&self, data: @wxPrintData) {
        unsafe { wxPrintData_Assign(self, data) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxPrintData {
        unsafe { wxPrintData_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintData_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCollate(&self) -> bool {
        unsafe { wxPrintData_GetCollate(self) }
    }
    #[fixed_stack_segment]
    fn getColour(&self) -> bool {
        unsafe { wxPrintData_GetColour(self) }
    }
    #[fixed_stack_segment]
    fn getDuplex(&self) -> int {
        unsafe { wxPrintData_GetDuplex(self) }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> @wxString {
        unsafe { wxPrintData_GetFilename(self) }
    }
    #[fixed_stack_segment]
    fn getFontMetricPath(&self) -> @wxString {
        unsafe { wxPrintData_GetFontMetricPath(self) }
    }
    #[fixed_stack_segment]
    fn getNoCopies(&self) -> int {
        unsafe { wxPrintData_GetNoCopies(self) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> int {
        unsafe { wxPrintData_GetOrientation(self) }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> int {
        unsafe { wxPrintData_GetPaperId(self) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> @wxSize {
        unsafe { wxPrintData_GetPaperSize(self) }
    }
    #[fixed_stack_segment]
    fn getPreviewCommand(&self) -> @wxString {
        unsafe { wxPrintData_GetPreviewCommand(self) }
    }
    #[fixed_stack_segment]
    fn getPrintMode(&self) -> int {
        unsafe { wxPrintData_GetPrintMode(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterCommand(&self) -> @wxString {
        unsafe { wxPrintData_GetPrinterCommand(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterName(&self) -> @wxString {
        unsafe { wxPrintData_GetPrinterName(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterOptions(&self) -> @wxString {
        unsafe { wxPrintData_GetPrinterOptions(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterScaleX(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleX(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterScaleY(&self) -> c_double {
        unsafe { wxPrintData_GetPrinterScaleY(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterTranslateX(&self) -> int {
        unsafe { wxPrintData_GetPrinterTranslateX(self) }
    }
    #[fixed_stack_segment]
    fn getPrinterTranslateY(&self) -> int {
        unsafe { wxPrintData_GetPrinterTranslateY(self) }
    }
    #[fixed_stack_segment]
    fn getQuality(&self) -> int {
        unsafe { wxPrintData_GetQuality(self) }
    }
    #[fixed_stack_segment]
    fn setCollate(&self, flag: c_int) {
        unsafe { wxPrintData_SetCollate(self, flag) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: c_int) {
        unsafe { wxPrintData_SetColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setDuplex(&self, duplex: int) {
        unsafe { wxPrintData_SetDuplex(self, duplex) }
    }
    #[fixed_stack_segment]
    fn setFilename(&self, filename: @wxString) {
        unsafe { wxPrintData_SetFilename(self, filename) }
    }
    #[fixed_stack_segment]
    fn setFontMetricPath(&self, path: @wxString) {
        unsafe { wxPrintData_SetFontMetricPath(self, path) }
    }
    #[fixed_stack_segment]
    fn setNoCopies(&self, v: int) {
        unsafe { wxPrintData_SetNoCopies(self, v) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: int) {
        unsafe { wxPrintData_SetOrientation(self, orient) }
    }
    #[fixed_stack_segment]
    fn setPaperId(&self, sizeId: int) {
        unsafe { wxPrintData_SetPaperId(self, sizeId) }
    }
    #[fixed_stack_segment]
    fn setPaperSize(&self, w: c_int, h: c_int) {
        unsafe { wxPrintData_SetPaperSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPreviewCommand(&self, command: @wxCommand) {
        unsafe { wxPrintData_SetPreviewCommand(self, command) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(&self, printMode: int) {
        unsafe { wxPrintData_SetPrintMode(self, printMode) }
    }
    #[fixed_stack_segment]
    fn setPrinterCommand(&self, command: @wxCommand) {
        unsafe { wxPrintData_SetPrinterCommand(self, command) }
    }
    #[fixed_stack_segment]
    fn setPrinterName(&self, name: @wxString) {
        unsafe { wxPrintData_SetPrinterName(self, name) }
    }
    #[fixed_stack_segment]
    fn setPrinterOptions(&self, options: @wxString) {
        unsafe { wxPrintData_SetPrinterOptions(self, options) }
    }
    #[fixed_stack_segment]
    fn setPrinterScaleX(&self, x: c_double) {
        unsafe { wxPrintData_SetPrinterScaleX(self, x) }
    }
    #[fixed_stack_segment]
    fn setPrinterScaleY(&self, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaleY(self, y) }
    }
    #[fixed_stack_segment]
    fn setPrinterScaling(&self, x: c_double, y: c_double) {
        unsafe { wxPrintData_SetPrinterScaling(self, x, y) }
    }
    #[fixed_stack_segment]
    fn setPrinterTranslateX(&self, x: int) {
        unsafe { wxPrintData_SetPrinterTranslateX(self, x) }
    }
    #[fixed_stack_segment]
    fn setPrinterTranslateY(&self, y: int) {
        unsafe { wxPrintData_SetPrinterTranslateY(self, y) }
    }
    #[fixed_stack_segment]
    fn setPrinterTranslation(&self, x: c_int, y: c_int) {
        unsafe { wxPrintData_SetPrinterTranslation(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setQuality(&self, quality: int) {
        unsafe { wxPrintData_SetQuality(self, quality) }
    }
}
trait wxPostScriptPrintNativeData {
    #[fixed_stack_segment]
    fn new() -> @wxPostScriptPrintNativeData {
        unsafe { wxPostScriptPrintNativeData_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPostScriptPrintNativeData_Delete(self) }
    }
}
trait wxPrintDialog {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, data: @wxPrintDialogData) -> @wxPrintDialog {
        unsafe { wxPrintDialog_Create(parent, data) }
    }
    #[fixed_stack_segment]
    fn getPrintDC(&self) -> @wxDC {
        unsafe { wxPrintDialog_GetPrintDC(self) }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, @wxPrintData) {
        unsafe { wxPrintDialog_GetPrintData(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self) -> @wxPrintDialogData {
        unsafe { wxPrintDialog_GetPrintDialogData(self) }
    }
}
trait wxPrintDialogData {
    #[fixed_stack_segment]
    fn assign(&self, data: @wxPrintDialogData) {
        unsafe { wxPrintDialogData_Assign(self, data) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, data: @wxPrintData) {
        unsafe { wxPrintDialogData_AssignData(self, data) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxPrintDialogData {
        unsafe { wxPrintDialogData_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newFromData(printData: @wxPrintData) -> @wxPrintDialogData {
        unsafe { wxPrintDialogData_CreateFromData(printData) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintDialogData_Delete(self) }
    }
    #[fixed_stack_segment]
    fn enableHelp(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnableHelp(self, flag) }
    }
    #[fixed_stack_segment]
    fn enablePageNumbers(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnablePageNumbers(self, flag) }
    }
    #[fixed_stack_segment]
    fn enablePrintToFile(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnablePrintToFile(self, flag) }
    }
    #[fixed_stack_segment]
    fn enableSelection(&self, flag: bool) {
        unsafe { wxPrintDialogData_EnableSelection(self, flag) }
    }
    #[fixed_stack_segment]
    fn getAllPages(&self) -> int {
        unsafe { wxPrintDialogData_GetAllPages(self) }
    }
    #[fixed_stack_segment]
    fn getCollate(&self) -> bool {
        unsafe { wxPrintDialogData_GetCollate(self) }
    }
    #[fixed_stack_segment]
    fn getEnableHelp(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnableHelp(self) }
    }
    #[fixed_stack_segment]
    fn getEnablePageNumbers(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnablePageNumbers(self) }
    }
    #[fixed_stack_segment]
    fn getEnablePrintToFile(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnablePrintToFile(self) }
    }
    #[fixed_stack_segment]
    fn getEnableSelection(&self) -> bool {
        unsafe { wxPrintDialogData_GetEnableSelection(self) }
    }
    #[fixed_stack_segment]
    fn getFromPage(&self) -> int {
        unsafe { wxPrintDialogData_GetFromPage(self) }
    }
    #[fixed_stack_segment]
    fn getMaxPage(&self) -> int {
        unsafe { wxPrintDialogData_GetMaxPage(self) }
    }
    #[fixed_stack_segment]
    fn getMinPage(&self) -> int {
        unsafe { wxPrintDialogData_GetMinPage(self) }
    }
    #[fixed_stack_segment]
    fn getNoCopies(&self) -> int {
        unsafe { wxPrintDialogData_GetNoCopies(self) }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, @wxPrintData) {
        unsafe { wxPrintDialogData_GetPrintData(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getPrintToFile(&self) -> bool {
        unsafe { wxPrintDialogData_GetPrintToFile(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> bool {
        unsafe { wxPrintDialogData_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn getToPage(&self) -> int {
        unsafe { wxPrintDialogData_GetToPage(self) }
    }
    #[fixed_stack_segment]
    fn setAllPages(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetAllPages(self, flag) }
    }
    #[fixed_stack_segment]
    fn setCollate(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetCollate(self, flag) }
    }
    #[fixed_stack_segment]
    fn setFromPage(&self, v: int) {
        unsafe { wxPrintDialogData_SetFromPage(self, v) }
    }
    #[fixed_stack_segment]
    fn setMaxPage(&self, v: int) {
        unsafe { wxPrintDialogData_SetMaxPage(self, v) }
    }
    #[fixed_stack_segment]
    fn setMinPage(&self, v: int) {
        unsafe { wxPrintDialogData_SetMinPage(self, v) }
    }
    #[fixed_stack_segment]
    fn setNoCopies(&self, v: int) {
        unsafe { wxPrintDialogData_SetNoCopies(self, v) }
    }
    #[fixed_stack_segment]
    fn setPrintData(&self, printData: @wxPrintData) {
        unsafe { wxPrintDialogData_SetPrintData(self, printData) }
    }
    #[fixed_stack_segment]
    fn setPrintToFile(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetPrintToFile(self, flag) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, flag: bool) {
        unsafe { wxPrintDialogData_SetSelection(self, flag) }
    }
    #[fixed_stack_segment]
    fn setToPage(&self, v: int) {
        unsafe { wxPrintDialogData_SetToPage(self, v) }
    }
}
trait wxPrintPreview {
    #[fixed_stack_segment]
    fn newFromData(printout: @wxPrintout, printoutForPrinting: @wxPrintout, data: @wxPrintData) -> @wxPrintPreview {
        unsafe { wxPrintPreview_CreateFromData(printout, printoutForPrinting, data) }
    }
    #[fixed_stack_segment]
    fn newFromDialogData(printout: @wxPrintout, printoutForPrinting: @wxPrintout, data: @wxPrintDialogData) -> @wxPrintPreview {
        unsafe { wxPrintPreview_CreateFromDialogData(printout, printoutForPrinting, data) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintPreview_Delete(self) }
    }
    #[fixed_stack_segment]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self) }
    }
    #[fixed_stack_segment]
    fn drawBlankPage(&self, canvas: @wxPreviewCanvas, dc: @wxDC) -> bool {
        unsafe { wxPrintPreview_DrawBlankPage(self, canvas, dc) }
    }
    #[fixed_stack_segment]
    fn getCanvas(&self) -> @wxPreviewCanvas {
        unsafe { wxPrintPreview_GetCanvas(self) }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> int {
        unsafe { wxPrintPreview_GetCurrentPage(self) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @wxFrame {
        unsafe { wxPrintPreview_GetFrame(self) }
    }
    #[fixed_stack_segment]
    fn getMaxPage(&self) -> int {
        unsafe { wxPrintPreview_GetMaxPage(self) }
    }
    #[fixed_stack_segment]
    fn getMinPage(&self) -> int {
        unsafe { wxPrintPreview_GetMinPage(self) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self, @wxPrintDialogData) {
        unsafe { wxPrintPreview_GetPrintDialogData(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @wxPrintout {
        unsafe { wxPrintPreview_GetPrintout(self) }
    }
    #[fixed_stack_segment]
    fn getPrintoutForPrinting(&self) -> @wxPrintout {
        unsafe { wxPrintPreview_GetPrintoutForPrinting(self) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> int {
        unsafe { wxPrintPreview_GetZoom(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPrintPreview_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn paintPage(&self, canvas: @wxPrintPreview, dc: @wxDC) -> bool {
        unsafe { wxPrintPreview_PaintPage(self, canvas, dc) }
    }
    #[fixed_stack_segment]
    fn print(&self, interactive: bool) -> bool {
        unsafe { wxPrintPreview_Print(self, interactive) }
    }
    #[fixed_stack_segment]
    fn renderPage(&self, pageNum: int) -> bool {
        unsafe { wxPrintPreview_RenderPage(self, pageNum) }
    }
    #[fixed_stack_segment]
    fn setCanvas(&self, canvas: @wxPreviewCanvas) {
        unsafe { wxPrintPreview_SetCanvas(self, canvas) }
    }
    #[fixed_stack_segment]
    fn setCurrentPage(&self, pageNum: int) -> bool {
        unsafe { wxPrintPreview_SetCurrentPage(self, pageNum) }
    }
    #[fixed_stack_segment]
    fn setFrame(&self, frame: @wxFrame) {
        unsafe { wxPrintPreview_SetFrame(self, frame) }
    }
    #[fixed_stack_segment]
    fn setOk(&self, ok: bool) {
        unsafe { wxPrintPreview_SetOk(self, ok) }
    }
    #[fixed_stack_segment]
    fn setPrintout(&self, printout: @wxPrintout) {
        unsafe { wxPrintPreview_SetPrintout(self, printout) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, percent: int) {
        unsafe { wxPrintPreview_SetZoom(self, percent) }
    }
}
trait wxPrinter {
    #[fixed_stack_segment]
    fn new(data: @wxPrintDialogData) -> @wxPrinter {
        unsafe { wxPrinter_Create(data) }
    }
    #[fixed_stack_segment]
    fn newAbortWindow(&self, parent: @wxWindow, printout: @wxPrintout) -> @wxWindow {
        unsafe { wxPrinter_CreateAbortWindow(self, parent, printout) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinter_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getAbort(&self) -> bool {
        unsafe { wxPrinter_GetAbort(self) }
    }
    #[fixed_stack_segment]
    fn getLastError(&self) -> int {
        unsafe { wxPrinter_GetLastError(self) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self, @wxPrintDialogData) {
        unsafe { wxPrinter_GetPrintDialogData(self, _ref) }
    }
    #[fixed_stack_segment]
    fn print(&self, parent: @wxWindow, printout: @wxPrintout, prompt: bool) -> bool {
        unsafe { wxPrinter_Print(self, parent, printout, prompt) }
    }
    #[fixed_stack_segment]
    fn printDialog(&self, parent: @wxWindow) -> @wxDC {
        unsafe { wxPrinter_PrintDialog(self, parent) }
    }
    #[fixed_stack_segment]
    fn reportError(&self, parent: @wxWindow, printout: @wxPrintout, message: @wxString) {
        unsafe { wxPrinter_ReportError(self, parent, printout, message) }
    }
    #[fixed_stack_segment]
    fn setup(&self, parent: @wxWindow) -> bool {
        unsafe { wxPrinter_Setup(self, parent) }
    }
}
trait wxPrinterDC {
    #[fixed_stack_segment]
    fn new(data: @wxPrintData) -> @wxPrinterDC {
        unsafe { wxPrinterDC_Create(data) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinterDC_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getPaperRect(&self) -> @wxRect {
        unsafe { wxPrinterDC_GetPaperRect(self) }
    }
}
trait wxPrintout {
    #[fixed_stack_segment]
    fn getDC(&self) -> @wxDC {
        unsafe { wxPrintout_GetDC(self) }
    }
    #[fixed_stack_segment]
    fn getPPIPrinter(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxPrintout_GetPPIPrinter(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getPPIScreen(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxPrintout_GetPPIScreen(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getPageSizeMM(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxPrintout_GetPageSizeMM(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getPageSizePixels(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxPrintout_GetPageSizePixels(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { wxPrintout_GetTitle(self) }
    }
    #[fixed_stack_segment]
    fn isPreview(&self) -> bool {
        unsafe { wxPrintout_IsPreview(self) }
    }
    #[fixed_stack_segment]
    fn setDC(&self, dc: @wxDC) {
        unsafe { wxPrintout_SetDC(self, dc) }
    }
    #[fixed_stack_segment]
    fn setPPIPrinter(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIPrinter(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPPIScreen(&self, x: c_int, y: c_int) {
        unsafe { wxPrintout_SetPPIScreen(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPageSizeMM(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizeMM(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setPageSizePixels(&self, w: c_int, h: c_int) {
        unsafe { wxPrintout_SetPageSizePixels(self, _arg1) }
    }
}
trait wxPrivateDropTarget {
}
trait wxProcess {
    #[fixed_stack_segment]
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self) }
    }
    #[fixed_stack_segment]
    fn newDefault(_prt: @wxWindow, _id: int) -> @wxProcess {
        unsafe { wxProcess_CreateDefault(_prt, _id) }
    }
    #[fixed_stack_segment]
    fn newRedirect(_prt: @wxWindow, _rdr: bool) -> @wxProcess {
        unsafe { wxProcess_CreateRedirect(_prt, _rdr) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxProcess_Delete(self) }
    }
    #[fixed_stack_segment]
    fn detach(&self) {
        unsafe { wxProcess_Detach(self) }
    }
    #[fixed_stack_segment]
    fn getErrorStream(&self) -> @wxInputStream {
        unsafe { wxProcess_GetErrorStream(self) }
    }
    #[fixed_stack_segment]
    fn getInputStream(&self) -> @wxInputStream {
        unsafe { wxProcess_GetInputStream(self) }
    }
    #[fixed_stack_segment]
    fn getOutputStream(&self) -> @wxOutputStream {
        unsafe { wxProcess_GetOutputStream(self) }
    }
    #[fixed_stack_segment]
    fn isRedirected(&self) -> bool {
        unsafe { wxProcess_IsRedirected(self) }
    }
    #[fixed_stack_segment]
    fn redirect(&self) {
        unsafe { wxProcess_Redirect(self) }
    }
    #[fixed_stack_segment]
    fn open(cmd: @wxString, flags: int) -> @wxProcess {
        unsafe { wxProcess_Open(cmd, flags) }
    }
    #[fixed_stack_segment]
    fn isErrorAvailable(&self) -> bool {
        unsafe { wxProcess_IsErrorAvailable(self) }
    }
    #[fixed_stack_segment]
    fn isInputAvailable(&self) -> bool {
        unsafe { wxProcess_IsInputAvailable(self) }
    }
    #[fixed_stack_segment]
    fn isInputOpened(&self) -> bool {
        unsafe { wxProcess_IsInputOpened(self) }
    }
}
trait wxProcessEvent {
    #[fixed_stack_segment]
    fn getExitCode(&self) -> int {
        unsafe { wxProcessEvent_GetExitCode(self) }
    }
    #[fixed_stack_segment]
    fn getPid(&self) -> int {
        unsafe { wxProcessEvent_GetPid(self) }
    }
}
trait wxProgressDialog {
    #[fixed_stack_segment]
    fn new(title: @wxString, message: @wxString, max: int, parent: @wxWindow, style: int) -> @wxProgressDialog {
        unsafe { wxProgressDialog_Create(title, message, max, parent, style) }
    }
    #[fixed_stack_segment]
    fn update(&self, value: int) -> bool {
        unsafe { wxProgressDialog_Update(self, value) }
    }
    #[fixed_stack_segment]
    fn updateWithMessage(&self, value: int, message: @wxString) -> bool {
        unsafe { wxProgressDialog_UpdateWithMessage(self, value, message) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self) }
    }
}
trait wxProtocol {
}
trait wxQuantize {
}
trait wxQueryCol {
}
trait wxQueryField {
}
trait wxQueryLayoutInfoEvent {
    #[fixed_stack_segment]
    fn new(id: int) -> @wxQueryLayoutInfoEvent {
        unsafe { wxQueryLayoutInfoEvent_Create(id) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> int {
        unsafe { wxQueryLayoutInfoEvent_GetAlignment(self) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> int {
        unsafe { wxQueryLayoutInfoEvent_GetFlags(self) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> int {
        unsafe { wxQueryLayoutInfoEvent_GetOrientation(self) }
    }
    #[fixed_stack_segment]
    fn getRequestedLength(&self) -> int {
        unsafe { wxQueryLayoutInfoEvent_GetRequestedLength(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxQueryLayoutInfoEvent_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, align: int) {
        unsafe { wxQueryLayoutInfoEvent_SetAlignment(self, align) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: int) {
        unsafe { wxQueryLayoutInfoEvent_SetFlags(self, flags) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: int) {
        unsafe { wxQueryLayoutInfoEvent_SetOrientation(self, orient) }
    }
    #[fixed_stack_segment]
    fn setRequestedLength(&self, length: int) {
        unsafe { wxQueryLayoutInfoEvent_SetRequestedLength(self, length) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, w: c_int, h: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetSize(self, _arg1) }
    }
}
trait wxQueryNewPaletteEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
        unsafe { wxQueryNewPaletteEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getPaletteRealized(&self) -> bool {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self) }
    }
    #[fixed_stack_segment]
    fn setPaletteRealized(&self, realized: bool) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self, realized) }
    }
}
trait wxRadioBox {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: int, _stl: int) -> @wxRadioBox {
        unsafe { wxRadioBox_Create(_prt, _id, _txt, _arg3, _arg4, _dim, _stl) }
    }
    #[fixed_stack_segment]
    fn enableItem(&self, item: int, enable: bool) {
        unsafe { wxRadioBox_EnableItem(self, item, enable) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @wxString) -> int {
        unsafe { wxRadioBox_FindString(self, s) }
    }
    #[fixed_stack_segment]
    fn getItemLabel(&self, item: int) -> @wxString {
        unsafe { wxRadioBox_GetItemLabel(self, item) }
    }
    #[fixed_stack_segment]
    fn getNumberOfRowsOrCols(&self) -> int {
        unsafe { wxRadioBox_GetNumberOfRowsOrCols(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self) -> int {
        unsafe { wxRadioBox_GetSelection(self) }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @wxString {
        unsafe { wxRadioBox_GetStringSelection(self) }
    }
    #[fixed_stack_segment]
    fn number(&self) -> int {
        unsafe { wxRadioBox_Number(self) }
    }
    #[fixed_stack_segment]
    fn setItemBitmap(&self, item: int, bitmap: @wxBitmap) {
        unsafe { wxRadioBox_SetItemBitmap(self, item, bitmap) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, item: int, label: @wxString) {
        unsafe { wxRadioBox_SetItemLabel(self, item, label) }
    }
    #[fixed_stack_segment]
    fn setNumberOfRowsOrCols(&self, n: int) {
        unsafe { wxRadioBox_SetNumberOfRowsOrCols(self, n) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, _n: int) {
        unsafe { wxRadioBox_SetSelection(self, _n) }
    }
    #[fixed_stack_segment]
    fn setStringSelection(&self, s: @wxString) {
        unsafe { wxRadioBox_SetStringSelection(self, s) }
    }
    #[fixed_stack_segment]
    fn showItem(&self, item: int, show: bool) {
        unsafe { wxRadioBox_ShowItem(self, item, show) }
    }
}
trait wxRadioButton {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxRadioButton {
        unsafe { wxRadioButton_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxRadioButton_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: bool) {
        unsafe { wxRadioButton_SetValue(self, value) }
    }
}
trait wxRealPoint {
}
trait wxRecordSet {
}
trait wxRect {
}
trait wxRegEx {
}
trait wxRegion {
    #[fixed_stack_segment]
    fn assign(&self, region: @wxRegion) {
        unsafe { wxRegion_Assign(self, region) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxRegion_Clear(self) }
    }
    #[fixed_stack_segment]
    fn containsPoint(&self, x: c_int, y: c_int) -> bool {
        unsafe { wxRegion_ContainsPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_ContainsRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxRegion {
        unsafe { wxRegion_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> @wxRegion {
        unsafe { wxRegion_CreateFromRect(_arg0) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxRegion_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isEmpty(&self) -> bool {
        unsafe { wxRegion_IsEmpty(self) }
    }
    #[fixed_stack_segment]
    fn getBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRegion_GetBox(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_IntersectRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn intersectRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_IntersectRegion(self, region) }
    }
    #[fixed_stack_segment]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_SubtractRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn subtractRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_SubtractRegion(self, region) }
    }
    #[fixed_stack_segment]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_UnionRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn unionRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_UnionRegion(self, region) }
    }
    #[fixed_stack_segment]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_XorRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn xorRegion(&self, region: @wxRegion) -> bool {
        unsafe { wxRegion_XorRegion(self, region) }
    }
}
trait wxRegionIterator {
    #[fixed_stack_segment]
    fn new() -> @wxRegionIterator {
        unsafe { wxRegionIterator_Create() }
    }
    #[fixed_stack_segment]
    fn newFromRegion(region: @wxRegion) -> @wxRegionIterator {
        unsafe { wxRegionIterator_CreateFromRegion(region) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxRegionIterator_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> int {
        unsafe { wxRegionIterator_GetHeight(self) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxRegionIterator_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> int {
        unsafe { wxRegionIterator_GetX(self) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> int {
        unsafe { wxRegionIterator_GetY(self) }
    }
    #[fixed_stack_segment]
    fn haveRects(&self) -> bool {
        unsafe { wxRegionIterator_HaveRects(self) }
    }
    #[fixed_stack_segment]
    fn next(&self) {
        unsafe { wxRegionIterator_Next(self) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxRegionIterator_Reset(self) }
    }
    #[fixed_stack_segment]
    fn resetToRegion(&self, region: @wxRegion) {
        unsafe { wxRegionIterator_ResetToRegion(self, region) }
    }
}
trait wxRemotelyScrolledTreeCtrl {
    #[fixed_stack_segment]
    fn adjustRemoteScrollbars(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars(self) }
    }
    #[fixed_stack_segment]
    fn calcTreeSize(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_CalcTreeSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn calcTreeSizeItem(&self, id: *c_void, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem(self, id, _arg2) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _cmp: *c_void, parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxRemotelyScrolledTreeCtrl {
        unsafe { wxRemotelyScrolledTreeCtrl_Create(_obj, _cmp, parent, id, _arg4, style) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getCompanionWindow(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_GetCompanionWindow(self) }
    }
    #[fixed_stack_segment]
    fn getScrollPos(&self, orient: int) -> int {
        unsafe { wxRemotelyScrolledTreeCtrl_GetScrollPos(self, orient) }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow(&self) -> @wxScrolledWindow {
        unsafe { wxRemotelyScrolledTreeCtrl_GetScrolledWindow(self) }
    }
    #[fixed_stack_segment]
    fn getViewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxRemotelyScrolledTreeCtrl_GetViewStart(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn hideVScrollbar(&self) {
        unsafe { wxRemotelyScrolledTreeCtrl_HideVScrollbar(self) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @wxDC) {
        unsafe { wxRemotelyScrolledTreeCtrl_PrepareDC(self, dc) }
    }
    #[fixed_stack_segment]
    fn scrollToLine(&self, posHoriz: int, posVert: int) {
        unsafe { wxRemotelyScrolledTreeCtrl_ScrollToLine(self, posHoriz, posVert) }
    }
    #[fixed_stack_segment]
    fn setCompanionWindow(&self, companion: *c_void) {
        unsafe { wxRemotelyScrolledTreeCtrl_SetCompanionWindow(self, companion) }
    }
    #[fixed_stack_segment]
    fn setScrollbars(&self, pixelsPerUnitX: int, pixelsPerUnitY: int, noUnitsX: int, noUnitsY: int, xPos: int, yPos: int, noRefresh: int) {
        unsafe { wxRemotelyScrolledTreeCtrl_SetScrollbars(self, pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
}
trait wxSVGFileDC {
    #[fixed_stack_segment]
    fn new(fileName: @wxString) -> @wxSVGFileDC {
        unsafe { wxSVGFileDC_Create(fileName) }
    }
    #[fixed_stack_segment]
    fn newWithSize(fileName: @wxString, w: c_int, h: c_int) -> @wxSVGFileDC {
        unsafe { wxSVGFileDC_CreateWithSize(fileName, _arg1) }
    }
    #[fixed_stack_segment]
    fn newWithSizeAndResolution(fileName: @wxString, w: c_int, h: c_int, a_dpi: c_float) -> @wxSVGFileDC {
        unsafe { wxSVGFileDC_CreateWithSizeAndResolution(fileName, _arg1, a_dpi) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSVGFileDC_Delete(self) }
    }
}
trait wxSashEvent {
    #[fixed_stack_segment]
    fn new(id: int, edge: int) -> @wxSashEvent {
        unsafe { wxSashEvent_Create(id, edge) }
    }
    #[fixed_stack_segment]
    fn getDragRect(&self) -> @wxRect {
        unsafe { wxSashEvent_GetDragRect(self) }
    }
    #[fixed_stack_segment]
    fn getDragStatus(&self) -> int {
        unsafe { wxSashEvent_GetDragStatus(self) }
    }
    #[fixed_stack_segment]
    fn getEdge(&self) -> int {
        unsafe { wxSashEvent_GetEdge(self) }
    }
    #[fixed_stack_segment]
    fn setDragRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxSashEvent_SetDragRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setDragStatus(&self, status: int) {
        unsafe { wxSashEvent_SetDragStatus(self, status) }
    }
    #[fixed_stack_segment]
    fn setEdge(&self, edge: int) {
        unsafe { wxSashEvent_SetEdge(self, edge) }
    }
}
trait wxSashLayoutWindow {
    #[fixed_stack_segment]
    fn new(_par: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: int) -> @wxSashLayoutWindow {
        unsafe { wxSashLayoutWindow_Create(_par, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getAlignment(&self) -> int {
        unsafe { wxSashLayoutWindow_GetAlignment(self) }
    }
    #[fixed_stack_segment]
    fn getOrientation(&self) -> int {
        unsafe { wxSashLayoutWindow_GetOrientation(self) }
    }
    #[fixed_stack_segment]
    fn setAlignment(&self, align: int) {
        unsafe { wxSashLayoutWindow_SetAlignment(self, align) }
    }
    #[fixed_stack_segment]
    fn setDefaultSize(&self, w: c_int, h: c_int) {
        unsafe { wxSashLayoutWindow_SetDefaultSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: int) {
        unsafe { wxSashLayoutWindow_SetOrientation(self, orient) }
    }
}
trait wxSashWindow {
    #[fixed_stack_segment]
    fn new(_par: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: int) -> @wxSashWindow {
        unsafe { wxSashWindow_Create(_par, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getDefaultBorderSize(&self) -> int {
        unsafe { wxSashWindow_GetDefaultBorderSize(self) }
    }
    #[fixed_stack_segment]
    fn getEdgeMargin(&self, edge: int) -> int {
        unsafe { wxSashWindow_GetEdgeMargin(self, edge) }
    }
    #[fixed_stack_segment]
    fn getExtraBorderSize(&self) -> int {
        unsafe { wxSashWindow_GetExtraBorderSize(self) }
    }
    #[fixed_stack_segment]
    fn getMaximumSizeX(&self) -> int {
        unsafe { wxSashWindow_GetMaximumSizeX(self) }
    }
    #[fixed_stack_segment]
    fn getMaximumSizeY(&self) -> int {
        unsafe { wxSashWindow_GetMaximumSizeY(self) }
    }
    #[fixed_stack_segment]
    fn getMinimumSizeX(&self) -> int {
        unsafe { wxSashWindow_GetMinimumSizeX(self) }
    }
    #[fixed_stack_segment]
    fn getMinimumSizeY(&self) -> int {
        unsafe { wxSashWindow_GetMinimumSizeY(self) }
    }
    #[fixed_stack_segment]
    fn getSashVisible(&self, edge: int) -> bool {
        unsafe { wxSashWindow_GetSashVisible(self, edge) }
    }
    #[fixed_stack_segment]
    fn hasBorder(&self, edge: int) -> bool {
        unsafe { wxSashWindow_HasBorder(self, edge) }
    }
    #[fixed_stack_segment]
    fn setDefaultBorderSize(&self, width: int) {
        unsafe { wxSashWindow_SetDefaultBorderSize(self, width) }
    }
    #[fixed_stack_segment]
    fn setExtraBorderSize(&self, width: int) {
        unsafe { wxSashWindow_SetExtraBorderSize(self, width) }
    }
    #[fixed_stack_segment]
    fn setMaximumSizeX(&self, max: int) {
        unsafe { wxSashWindow_SetMaximumSizeX(self, max) }
    }
    #[fixed_stack_segment]
    fn setMaximumSizeY(&self, max: int) {
        unsafe { wxSashWindow_SetMaximumSizeY(self, max) }
    }
    #[fixed_stack_segment]
    fn setMinimumSizeX(&self, min: int) {
        unsafe { wxSashWindow_SetMinimumSizeX(self, min) }
    }
    #[fixed_stack_segment]
    fn setMinimumSizeY(&self, min: int) {
        unsafe { wxSashWindow_SetMinimumSizeY(self, min) }
    }
    #[fixed_stack_segment]
    fn setSashBorder(&self, edge: int, border: bool) {
        unsafe { wxSashWindow_SetSashBorder(self, edge, border) }
    }
    #[fixed_stack_segment]
    fn setSashVisible(&self, edge: int, sash: bool) {
        unsafe { wxSashWindow_SetSashVisible(self, edge, sash) }
    }
}
trait wxScopedArray {
}
trait wxScopedPtr {
}
trait wxScreenDC {
    #[fixed_stack_segment]
    fn new() -> @wxScreenDC {
        unsafe { wxScreenDC_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxScreenDC_Delete(self) }
    }
    #[fixed_stack_segment]
    fn endDrawingOnTop(&self) -> bool {
        unsafe { wxScreenDC_EndDrawingOnTop(self) }
    }
    #[fixed_stack_segment]
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTop(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn startDrawingOnTopOfWin(&self, win: @wxWindow) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self, win) }
    }
}
trait wxScrollBar {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxScrollBar {
        unsafe { wxScrollBar_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> int {
        unsafe { wxScrollBar_GetPageSize(self) }
    }
    #[fixed_stack_segment]
    fn getRange(&self) -> int {
        unsafe { wxScrollBar_GetRange(self) }
    }
    #[fixed_stack_segment]
    fn getThumbPosition(&self) -> int {
        unsafe { wxScrollBar_GetThumbPosition(self) }
    }
    #[fixed_stack_segment]
    fn getThumbSize(&self) -> int {
        unsafe { wxScrollBar_GetThumbSize(self) }
    }
    #[fixed_stack_segment]
    fn setScrollbar(&self, position: int, thumbSize: int, range: int, pageSize: int, refresh: bool) {
        unsafe { wxScrollBar_SetScrollbar(self, position, thumbSize, range, pageSize, refresh) }
    }
    #[fixed_stack_segment]
    fn setThumbPosition(&self, viewStart: int) {
        unsafe { wxScrollBar_SetThumbPosition(self, viewStart) }
    }
}
trait wxScrollEvent {
    #[fixed_stack_segment]
    fn getOrientation(&self) -> int {
        unsafe { wxScrollEvent_GetOrientation(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> int {
        unsafe { wxScrollEvent_GetPosition(self) }
    }
}
trait wxScrollWinEvent {
    #[fixed_stack_segment]
    fn getOrientation(&self) -> int {
        unsafe { wxScrollWinEvent_GetOrientation(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> int {
        unsafe { wxScrollWinEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn setOrientation(&self, orient: int) {
        unsafe { wxScrollWinEvent_SetOrientation(self, orient) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: int) {
        unsafe { wxScrollWinEvent_SetPosition(self, pos) }
    }
}
trait wxScrolledWindow {
    #[fixed_stack_segment]
    fn adjustScrollbars(&self) {
        unsafe { wxScrolledWindow_AdjustScrollbars(self) }
    }
    #[fixed_stack_segment]
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int) {
        unsafe { wxScrolledWindow_CalcScrolledPosition(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int) {
        unsafe { wxScrolledWindow_CalcUnscrolledPosition(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxScrolledWindow {
        unsafe { wxScrolledWindow_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn enableScrolling(&self, x_scrolling: bool, y_scrolling: bool) {
        unsafe { wxScrolledWindow_EnableScrolling(self, x_scrolling, y_scrolling) }
    }
    #[fixed_stack_segment]
    fn getScaleX(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleX(self) }
    }
    #[fixed_stack_segment]
    fn getScaleY(&self) -> c_double {
        unsafe { wxScrolledWindow_GetScaleY(self) }
    }
    #[fixed_stack_segment]
    fn getScrollPageSize(&self, orient: int) -> int {
        unsafe { wxScrolledWindow_GetScrollPageSize(self, orient) }
    }
    #[fixed_stack_segment]
    fn getScrollPixelsPerUnit(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetScrollPixelsPerUnit(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getTargetWindow(&self) -> @wxWindow {
        unsafe { wxScrolledWindow_GetTargetWindow(self) }
    }
    #[fixed_stack_segment]
    fn getViewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetViewStart(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_GetVirtualSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn onDraw(&self, dc: @wxDC) {
        unsafe { wxScrolledWindow_OnDraw(self, dc) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @wxDC) {
        unsafe { wxScrolledWindow_PrepareDC(self, dc) }
    }
    #[fixed_stack_segment]
    fn scroll(&self, x_pos: c_int, y_pos: c_int) {
        unsafe { wxScrolledWindow_Scroll(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setScale(&self, xs: c_double, ys: c_double) {
        unsafe { wxScrolledWindow_SetScale(self, xs, ys) }
    }
    #[fixed_stack_segment]
    fn setScrollPageSize(&self, orient: int, pageSize: int) {
        unsafe { wxScrolledWindow_SetScrollPageSize(self, orient, pageSize) }
    }
    #[fixed_stack_segment]
    fn setScrollbars(&self, pixelsPerUnitX: int, pixelsPerUnitY: int, noUnitsX: int, noUnitsY: int, xPos: int, yPos: int, noRefresh: bool) {
        unsafe { wxScrolledWindow_SetScrollbars(self, pixelsPerUnitX, pixelsPerUnitY, noUnitsX, noUnitsY, xPos, yPos, noRefresh) }
    }
    #[fixed_stack_segment]
    fn showScrollbars(&self, showh: int, showv: int) {
        unsafe { wxScrolledWindow_ShowScrollbars(self, showh, showv) }
    }
    #[fixed_stack_segment]
    fn setTargetWindow(&self, target: @wxWindow) {
        unsafe { wxScrolledWindow_SetTargetWindow(self, target) }
    }
    #[fixed_stack_segment]
    fn viewStart(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxScrolledWindow_ViewStart(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setScrollRate(&self, xstep: int, ystep: int) {
        unsafe { wxScrolledWindow_SetScrollRate(self, xstep, ystep) }
    }
}
trait wxSemaphore {
}
trait wxServer {
}
trait wxServerBase {
}
trait wxSetCursorEvent {
    #[fixed_stack_segment]
    fn getCursor(&self) -> @wxCursor {
        unsafe { wxSetCursorEvent_GetCursor(self) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> int {
        unsafe { wxSetCursorEvent_GetX(self) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> int {
        unsafe { wxSetCursorEvent_GetY(self) }
    }
    #[fixed_stack_segment]
    fn hasCursor(&self) -> bool {
        unsafe { wxSetCursorEvent_HasCursor(self) }
    }
    #[fixed_stack_segment]
    fn setCursor(&self, cursor: @wxCursor) {
        unsafe { wxSetCursorEvent_SetCursor(self, cursor) }
    }
}
trait wxShowEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
        unsafe { wxShowEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn isShown(&self) -> bool {
        unsafe { wxShowEvent_IsShown(self) }
    }
    #[fixed_stack_segment]
    fn setShow(&self, show: bool) {
        unsafe { wxShowEvent_SetShow(self, show) }
    }
}
trait wxSimpleHelpProvider {
    #[fixed_stack_segment]
    fn new() -> @wxSimpleHelpProvider {
        unsafe { wxSimpleHelpProvider_Create() }
    }
}
trait wxSingleChoiceDialog {
}
trait wxSingleInstanceChecker {
    #[fixed_stack_segment]
    fn new(_obj: *c_void, name: @wxString, path: @wxString) -> bool {
        unsafe { wxSingleInstanceChecker_Create(_obj, name, path) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxSingleInstanceChecker {
        unsafe { wxSingleInstanceChecker_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSingleInstanceChecker_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isAnotherRunning(&self) -> bool {
        unsafe { wxSingleInstanceChecker_IsAnotherRunning(self) }
    }
}
trait wxSize {
    #[fixed_stack_segment]
    fn new(w: c_int, h: c_int) -> @wxSize {
        unsafe { wxSize_Create(_arg0) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxSize_Destroy(self) }
    }
    #[fixed_stack_segment]
    fn getHeight(&self) -> int {
        unsafe { wxSize_GetHeight(self) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> int {
        unsafe { wxSize_GetWidth(self) }
    }
    #[fixed_stack_segment]
    fn setHeight(&self, h: int) {
        unsafe { wxSize_SetHeight(self, h) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, w: int) {
        unsafe { wxSize_SetWidth(self, w) }
    }
}
trait wxSizeEvent {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *c_void) {
        unsafe { wxSizeEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxSizeEvent_GetSize(self) }
    }
}
trait wxSizer {
    #[fixed_stack_segment]
    fn add(&self, width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_Add(self, _arg1, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addSizer(&self, sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_AddSizer(self, sizer, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addWindow(&self, window: @wxWindow, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_AddWindow(self, window, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { wxSizer_CalcMin(self) }
    }
    #[fixed_stack_segment]
    fn fit(&self, window: @wxWindow) {
        unsafe { wxSizer_Fit(self, window) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *c_void, _cnt: int) -> int {
        unsafe { wxSizer_GetChildren(self, _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> @wxSize {
        unsafe { wxSizer_GetMinSize(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxSizer_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxSizer_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn insert(&self, before: int, width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_Insert(self, before, _arg2, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertSizer(&self, before: int, sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_InsertSizer(self, before, sizer, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertWindow(&self, before: int, window: @wxWindow, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_InsertWindow(self, before, window, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn layout(&self) {
        unsafe { wxSizer_Layout(self) }
    }
    #[fixed_stack_segment]
    fn prepend(&self, width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_Prepend(self, _arg1, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependSizer(&self, sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_PrependSizer(self, sizer, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependWindow(&self, window: @wxWindow, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizer_PrependWindow(self, window, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxSizer_RecalcSizes(self) }
    }
    #[fixed_stack_segment]
    fn setDimension(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetDimension(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setItemMinSize(&self, pos: int, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSize(self, pos, _arg2) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeSizer(&self, sizer: @wxSizer, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self, sizer, _arg2) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeWindow(&self, window: @wxWindow, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self, window, _arg2) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setSizeHints(&self, window: @wxWindow) {
        unsafe { wxSizer_SetSizeHints(self, window) }
    }
    #[fixed_stack_segment]
    fn addSpacer(&self, size: int) {
        unsafe { wxSizer_AddSpacer(self, size) }
    }
    #[fixed_stack_segment]
    fn addStretchSpacer(&self, size: int) {
        unsafe { wxSizer_AddStretchSpacer(self, size) }
    }
    #[fixed_stack_segment]
    fn clear(&self, delete_windows: bool) {
        unsafe { wxSizer_Clear(self, delete_windows) }
    }
    #[fixed_stack_segment]
    fn detachWindow(&self, window: @wxWindow) -> bool {
        unsafe { wxSizer_DetachWindow(self, window) }
    }
    #[fixed_stack_segment]
    fn detachSizer(&self, sizer: @wxSizer) -> bool {
        unsafe { wxSizer_DetachSizer(self, sizer) }
    }
    #[fixed_stack_segment]
    fn detach(&self, index: int) -> bool {
        unsafe { wxSizer_Detach(self, index) }
    }
    #[fixed_stack_segment]
    fn fitInside(&self, window: @wxWindow) {
        unsafe { wxSizer_FitInside(self, window) }
    }
    #[fixed_stack_segment]
    fn getContainingWindow(&self) -> @wxWindow {
        unsafe { wxSizer_GetContainingWindow(self) }
    }
    #[fixed_stack_segment]
    fn getItemWindow(&self, window: @wxWindow, recursive: bool) -> @wxSizerItem {
        unsafe { wxSizer_GetItemWindow(self, window, recursive) }
    }
    #[fixed_stack_segment]
    fn getItemSizer(&self, window: @wxSizer, recursive: bool) -> @wxSizerItem {
        unsafe { wxSizer_GetItemSizer(self, window, recursive) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, index: int) -> @wxSizerItem {
        unsafe { wxSizer_GetItem(self, index) }
    }
    #[fixed_stack_segment]
    fn hideWindow(&self, window: @wxWindow) -> bool {
        unsafe { wxSizer_HideWindow(self, window) }
    }
    #[fixed_stack_segment]
    fn hideSizer(&self, sizer: @wxSizer) -> bool {
        unsafe { wxSizer_HideSizer(self, sizer) }
    }
    #[fixed_stack_segment]
    fn hide(&self, index: int) -> bool {
        unsafe { wxSizer_Hide(self, index) }
    }
    #[fixed_stack_segment]
    fn insertSpacer(&self, index: int, size: int) -> @wxSizerItem {
        unsafe { wxSizer_InsertSpacer(self, index, size) }
    }
    #[fixed_stack_segment]
    fn insertStretchSpacer(&self, index: int, prop: int) -> @wxSizerItem {
        unsafe { wxSizer_InsertStretchSpacer(self, index, prop) }
    }
    #[fixed_stack_segment]
    fn isShownWindow(&self, window: *@wxWindow) -> bool {
        unsafe { wxSizer_IsShownWindow(self, window) }
    }
    #[fixed_stack_segment]
    fn isShownSizer(&self, sizer: *@wxSizer) -> bool {
        unsafe { wxSizer_IsShownSizer(self, sizer) }
    }
    #[fixed_stack_segment]
    fn isShown(&self, index: int) -> bool {
        unsafe { wxSizer_IsShown(self, index) }
    }
    #[fixed_stack_segment]
    fn prependSpacer(&self, size: int) -> @wxSizerItem {
        unsafe { wxSizer_PrependSpacer(self, size) }
    }
    #[fixed_stack_segment]
    fn prependStretchSpacer(&self, prop: int) -> @wxSizerItem {
        unsafe { wxSizer_PrependStretchSpacer(self, prop) }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, oldwin: @wxWindow, newwin: @wxWindow, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceWindow(self, oldwin, newwin, recursive) }
    }
    #[fixed_stack_segment]
    fn replaceSizer(&self, oldsz: @wxSizer, newsz: @wxSizer, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceSizer(self, oldsz, newsz, recursive) }
    }
    #[fixed_stack_segment]
    fn replace(&self, oldindex: int, newitem: @wxSizerItem) -> bool {
        unsafe { wxSizer_Replace(self, oldindex, newitem) }
    }
    #[fixed_stack_segment]
    fn setVirtualSizeHints(&self, window: @wxWindow) {
        unsafe { wxSizer_SetVirtualSizeHints(self, window) }
    }
    #[fixed_stack_segment]
    fn showWindow(&self, window: @wxWindow, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowWindow(self, window, show, recursive) }
    }
    #[fixed_stack_segment]
    fn showSizer(&self, sizer: @wxSizer, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowSizer(self, sizer, show, recursive) }
    }
    #[fixed_stack_segment]
    fn show(&self, sizer: @wxSizer, index: int, show: bool) -> bool {
        unsafe { wxSizer_Show(self, sizer, index, show) }
    }
}
trait wxSizerItem {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { wxSizerItem_CalcMin(self) }
    }
    #[fixed_stack_segment]
    fn new(width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void) -> @wxSizerItem {
        unsafe { wxSizerItem_Create(_arg0, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn newInSizer(sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizerItem_CreateInSizer(sizer, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn newInWindow(window: @wxWindow, option: int, flag: int, border: int, userData: *c_void) {
        unsafe { wxSizerItem_CreateInWindow(window, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> int {
        unsafe { wxSizerItem_GetBorder(self) }
    }
    #[fixed_stack_segment]
    fn getFlag(&self) -> int {
        unsafe { wxSizerItem_GetFlag(self) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> @wxSize {
        unsafe { wxSizerItem_GetMinSize(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxSizerItem_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxSizerItem_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @wxSizer {
        unsafe { wxSizerItem_GetSizer(self) }
    }
    #[fixed_stack_segment]
    fn getUserData(&self) {
        unsafe { wxSizerItem_GetUserData(self) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { wxSizerItem_GetWindow(self) }
    }
    #[fixed_stack_segment]
    fn isSizer(&self) -> bool {
        unsafe { wxSizerItem_IsSizer(self) }
    }
    #[fixed_stack_segment]
    fn isSpacer(&self) -> bool {
        unsafe { wxSizerItem_IsSpacer(self) }
    }
    #[fixed_stack_segment]
    fn isWindow(&self) -> bool {
        unsafe { wxSizerItem_IsWindow(self) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: int) {
        unsafe { wxSizerItem_SetBorder(self, border) }
    }
    #[fixed_stack_segment]
    fn setDimension(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int) {
        unsafe { wxSizerItem_SetDimension(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setFlag(&self, flag: int) {
        unsafe { wxSizerItem_SetFlag(self, flag) }
    }
    #[fixed_stack_segment]
    fn setFloatRatio(&self, ratio: c_float) {
        unsafe { wxSizerItem_SetFloatRatio(self, ratio) }
    }
    #[fixed_stack_segment]
    fn setInitSize(&self, x: c_int, y: c_int) {
        unsafe { wxSizerItem_SetInitSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setRatio(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetRatio(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setSizer(&self, sizer: @wxSizer) {
        unsafe { wxSizerItem_SetSizer(self, sizer) }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, window: @wxWindow) {
        unsafe { wxSizerItem_SetWindow(self, window) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSizerItem_Delete(self) }
    }
    #[fixed_stack_segment]
    fn deleteWindows(&self) {
        unsafe { wxSizerItem_DeleteWindows(self) }
    }
    #[fixed_stack_segment]
    fn detachSizer(&self) {
        unsafe { wxSizerItem_DetachSizer(self) }
    }
    #[fixed_stack_segment]
    fn getProportion(&self) -> int {
        unsafe { wxSizerItem_GetProportion(self) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @wxRect {
        unsafe { wxSizerItem_GetRect(self) }
    }
    #[fixed_stack_segment]
    fn getSpacer(&self) -> @wxSize {
        unsafe { wxSizerItem_GetSpacer(self) }
    }
    #[fixed_stack_segment]
    fn isShown(&self) -> int {
        unsafe { wxSizerItem_IsShown(self) }
    }
    #[fixed_stack_segment]
    fn setProportion(&self, proportion: int) {
        unsafe { wxSizerItem_SetProportion(self, proportion) }
    }
    #[fixed_stack_segment]
    fn setSpacer(&self, width: c_int, height: c_int) {
        unsafe { wxSizerItem_SetSpacer(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn show(&self, show: int) {
        unsafe { wxSizerItem_Show(self, show) }
    }
}
trait wxSlider {
    #[fixed_stack_segment]
    fn clearSel(&self) {
        unsafe { wxSlider_ClearSel(self) }
    }
    #[fixed_stack_segment]
    fn clearTicks(&self) {
        unsafe { wxSlider_ClearTicks(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _init: int, _min: int, _max: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSlider {
        unsafe { wxSlider_Create(_prt, _id, _init, _min, _max, _arg5, _stl) }
    }
    #[fixed_stack_segment]
    fn getLineSize(&self) -> int {
        unsafe { wxSlider_GetLineSize(self) }
    }
    #[fixed_stack_segment]
    fn getMax(&self) -> int {
        unsafe { wxSlider_GetMax(self) }
    }
    #[fixed_stack_segment]
    fn getMin(&self) -> int {
        unsafe { wxSlider_GetMin(self) }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> int {
        unsafe { wxSlider_GetPageSize(self) }
    }
    #[fixed_stack_segment]
    fn getSelEnd(&self) -> int {
        unsafe { wxSlider_GetSelEnd(self) }
    }
    #[fixed_stack_segment]
    fn getSelStart(&self) -> int {
        unsafe { wxSlider_GetSelStart(self) }
    }
    #[fixed_stack_segment]
    fn getThumbLength(&self) -> int {
        unsafe { wxSlider_GetThumbLength(self) }
    }
    #[fixed_stack_segment]
    fn getTickFreq(&self) -> int {
        unsafe { wxSlider_GetTickFreq(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> int {
        unsafe { wxSlider_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setLineSize(&self, lineSize: int) {
        unsafe { wxSlider_SetLineSize(self, lineSize) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, pageSize: int) {
        unsafe { wxSlider_SetPageSize(self, pageSize) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, minValue: int, maxValue: int) {
        unsafe { wxSlider_SetRange(self, minValue, maxValue) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, minPos: int, maxPos: int) {
        unsafe { wxSlider_SetSelection(self, minPos, maxPos) }
    }
    #[fixed_stack_segment]
    fn setThumbLength(&self, len: int) {
        unsafe { wxSlider_SetThumbLength(self, len) }
    }
    #[fixed_stack_segment]
    fn setTick(&self, tickPos: int) {
        unsafe { wxSlider_SetTick(self, tickPos) }
    }
    #[fixed_stack_segment]
    fn setTickFreq(&self, n: int, pos: int) {
        unsafe { wxSlider_SetTickFreq(self, n, pos) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: int) {
        unsafe { wxSlider_SetValue(self, value) }
    }
}
trait wxSockAddress {
}
trait wxSocketBase {
}
trait wxSocketClient {
}
trait wxSocketEvent {
}
trait wxSocketInputStream {
}
trait wxSocketOutputStream {
}
trait wxSocketServer {
}
trait wxSpinButton {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSpinButton {
        unsafe { wxSpinButton_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getMax(&self) -> int {
        unsafe { wxSpinButton_GetMax(self) }
    }
    #[fixed_stack_segment]
    fn getMin(&self) -> int {
        unsafe { wxSpinButton_GetMin(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> int {
        unsafe { wxSpinButton_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, minVal: int, maxVal: int) {
        unsafe { wxSpinButton_SetRange(self, minVal, maxVal) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, val: int) {
        unsafe { wxSpinButton_SetValue(self, val) }
    }
}
trait wxSpinCtrl {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: int, _max: int, _init: int) -> @wxSpinCtrl {
        unsafe { wxSpinCtrl_Create(_prt, _id, _txt, _arg3, _stl, _min, _max, _init) }
    }
    #[fixed_stack_segment]
    fn getMax(&self) -> int {
        unsafe { wxSpinCtrl_GetMax(self) }
    }
    #[fixed_stack_segment]
    fn getMin(&self) -> int {
        unsafe { wxSpinCtrl_GetMin(self) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> int {
        unsafe { wxSpinCtrl_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setRange(&self, min_val: int, max_val: int) {
        unsafe { wxSpinCtrl_SetRange(self, min_val, max_val) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, val: int) {
        unsafe { wxSpinCtrl_SetValue(self, val) }
    }
}
trait wxSpinEvent {
    #[fixed_stack_segment]
    fn getPosition(&self) -> int {
        unsafe { wxSpinEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: int) {
        unsafe { wxSpinEvent_SetPosition(self, pos) }
    }
}
trait wxSplashScreen {
}
trait wxSplitterEvent {
}
trait wxSplitterScrolledWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxSplitterScrolledWindow {
        unsafe { wxSplitterScrolledWindow_Create(parent, id, _arg2, style) }
    }
}
trait wxSplitterWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxSplitterWindow {
        unsafe { wxSplitterWindow_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getBorderSize(&self) -> int {
        unsafe { wxSplitterWindow_GetBorderSize(self) }
    }
    #[fixed_stack_segment]
    fn getMinimumPaneSize(&self) -> int {
        unsafe { wxSplitterWindow_GetMinimumPaneSize(self) }
    }
    #[fixed_stack_segment]
    fn getSashPosition(&self) -> int {
        unsafe { wxSplitterWindow_GetSashPosition(self) }
    }
    #[fixed_stack_segment]
    fn getSashSize(&self) -> int {
        unsafe { wxSplitterWindow_GetSashSize(self) }
    }
    #[fixed_stack_segment]
    fn getSplitMode(&self) -> int {
        unsafe { wxSplitterWindow_GetSplitMode(self) }
    }
    #[fixed_stack_segment]
    fn getWindow1(&self) -> @wxWindow {
        unsafe { wxSplitterWindow_GetWindow1(self) }
    }
    #[fixed_stack_segment]
    fn getWindow2(&self) -> @wxWindow {
        unsafe { wxSplitterWindow_GetWindow2(self) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, window: @wxWindow) {
        unsafe { wxSplitterWindow_Initialize(self, window) }
    }
    #[fixed_stack_segment]
    fn isSplit(&self) -> bool {
        unsafe { wxSplitterWindow_IsSplit(self) }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, winOld: @wxWindow, winNew: @wxWindow) -> bool {
        unsafe { wxSplitterWindow_ReplaceWindow(self, winOld, winNew) }
    }
    #[fixed_stack_segment]
    fn setBorderSize(&self, width: int) {
        unsafe { wxSplitterWindow_SetBorderSize(self, width) }
    }
    #[fixed_stack_segment]
    fn setMinimumPaneSize(&self, min: int) {
        unsafe { wxSplitterWindow_SetMinimumPaneSize(self, min) }
    }
    #[fixed_stack_segment]
    fn setSashPosition(&self, position: int, redraw: bool) {
        unsafe { wxSplitterWindow_SetSashPosition(self, position, redraw) }
    }
    #[fixed_stack_segment]
    fn setSashSize(&self, width: int) {
        unsafe { wxSplitterWindow_SetSashSize(self, width) }
    }
    #[fixed_stack_segment]
    fn setSplitMode(&self, mode: int) {
        unsafe { wxSplitterWindow_SetSplitMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn splitHorizontally(&self, window1: @wxWindow, window2: @wxWindow, sashPosition: int) -> bool {
        unsafe { wxSplitterWindow_SplitHorizontally(self, window1, window2, sashPosition) }
    }
    #[fixed_stack_segment]
    fn splitVertically(&self, window1: @wxWindow, window2: @wxWindow, sashPosition: int) -> bool {
        unsafe { wxSplitterWindow_SplitVertically(self, window1, window2, sashPosition) }
    }
    #[fixed_stack_segment]
    fn unsplit(&self, toRemove: @wxWindow) -> bool {
        unsafe { wxSplitterWindow_Unsplit(self, toRemove) }
    }
    #[fixed_stack_segment]
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self) }
    }
    #[fixed_stack_segment]
    fn setSashGravity(&self, gravity: c_double) {
        unsafe { wxSplitterWindow_SetSashGravity(self, gravity) }
    }
}
trait wxStaticBitmap {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, bitmap: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticBitmap {
        unsafe { wxStaticBitmap_Create(_prt, _id, bitmap, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStaticBitmap_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getBitmap(&self, @wxBitmap) {
        unsafe { wxStaticBitmap_GetBitmap(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, @wxIcon) {
        unsafe { wxStaticBitmap_GetIcon(self, _ref) }
    }
    #[fixed_stack_segment]
    fn setBitmap(&self, bitmap: @wxBitmap) {
        unsafe { wxStaticBitmap_SetBitmap(self, bitmap) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @wxIcon) {
        unsafe { wxStaticBitmap_SetIcon(self, icon) }
    }
}
trait wxStaticBox {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticBox {
        unsafe { wxStaticBox_Create(_prt, _id, _txt, _arg3, _stl) }
    }
}
trait wxStaticBoxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @wxSize {
        unsafe { wxStaticBoxSizer_CalcMin(self) }
    }
    #[fixed_stack_segment]
    fn new(box: @wxStaticBox, orient: int) -> @wxStaticBoxSizer {
        unsafe { wxStaticBoxSizer_Create(box, orient) }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self) -> @wxStaticBox {
        unsafe { wxStaticBoxSizer_GetStaticBox(self) }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxStaticBoxSizer_RecalcSizes(self) }
    }
}
trait wxStaticLine {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticLine {
        unsafe { wxStaticLine_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getDefaultSize(&self) -> int {
        unsafe { wxStaticLine_GetDefaultSize(self) }
    }
    #[fixed_stack_segment]
    fn isVertical(&self) -> bool {
        unsafe { wxStaticLine_IsVertical(self) }
    }
}
trait wxStaticText {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticText {
        unsafe { wxStaticText_Create(_prt, _id, _txt, _arg3, _stl) }
    }
}
trait wxStatusBar {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStatusBar {
        unsafe { wxStatusBar_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn getBorderX(&self) -> int {
        unsafe { wxStatusBar_GetBorderX(self) }
    }
    #[fixed_stack_segment]
    fn getBorderY(&self) -> int {
        unsafe { wxStatusBar_GetBorderY(self) }
    }
    #[fixed_stack_segment]
    fn getFieldsCount(&self) -> int {
        unsafe { wxStatusBar_GetFieldsCount(self) }
    }
    #[fixed_stack_segment]
    fn getStatusText(&self, number: int) -> @wxString {
        unsafe { wxStatusBar_GetStatusText(self, number) }
    }
    #[fixed_stack_segment]
    fn setFieldsCount(&self, number: int, widths: *int) {
        unsafe { wxStatusBar_SetFieldsCount(self, number, widths) }
    }
    #[fixed_stack_segment]
    fn setMinHeight(&self, height: int) {
        unsafe { wxStatusBar_SetMinHeight(self, height) }
    }
    #[fixed_stack_segment]
    fn setStatusText(&self, text: @wxString, number: int) {
        unsafe { wxStatusBar_SetStatusText(self, text, number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, n: int, widths: *int) {
        unsafe { wxStatusBar_SetStatusWidths(self, n, widths) }
    }
}
trait wxStopWatch {
    #[fixed_stack_segment]
    fn new() -> @wxStopWatch {
        unsafe { wxStopWatch_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStopWatch_Delete(self) }
    }
    #[fixed_stack_segment]
    fn start(&self, msec: int) {
        unsafe { wxStopWatch_Start(self, msec) }
    }
    #[fixed_stack_segment]
    fn pause(&self) {
        unsafe { wxStopWatch_Pause(self) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxStopWatch_Resume(self) }
    }
    #[fixed_stack_segment]
    fn time(&self) -> int {
        unsafe { wxStopWatch_Time(self) }
    }
}
trait wxStreamBase {
    #[fixed_stack_segment]
    fn getLastError(&self) -> int {
        unsafe { wxStreamBase_GetLastError(self) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> int {
        unsafe { wxStreamBase_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxStreamBase_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStreamBase_Delete(self) }
    }
}
trait wxStreamBuffer {
}
trait wxStreamToTextRedirector {
}
trait wxString {
    #[fixed_stack_segment]
    fn new(buffer: *wchar_t) -> @wxString {
        unsafe { wxString_Create(buffer) }
    }
    #[fixed_stack_segment]
    fn newLen(buffer: *wchar_t, len: int) -> @wxString {
        unsafe { wxString_CreateLen(buffer, len) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxString_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getString(&self, buffer: **wchar_t) -> c_int {
        unsafe { wxString_GetString(self, buffer) }
    }
    #[fixed_stack_segment]
    fn length(&self) -> size_t {
        unsafe { wxString_Length(self) }
    }
}
trait wxStringBuffer {
}
trait wxStringClientData {
}
trait wxStringList {
}
trait wxStringTokenizer {
}
trait wxSysColourChangedEvent {
}
trait wxSystemOptions {
}
trait wxSystemSettings {
    #[fixed_stack_segment]
    fn getColour(index: int, @wxColour) {
        unsafe { wxSystemSettings_GetColour(index, _ref) }
    }
    #[fixed_stack_segment]
    fn getFont(index: int, @wxFont) {
        unsafe { wxSystemSettings_GetFont(index, _ref) }
    }
    #[fixed_stack_segment]
    fn getMetric(index: int) -> int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    #[fixed_stack_segment]
    fn getScreenType() -> int {
        unsafe { wxSystemSettings_GetScreenType() }
    }
}
trait wxTabCtrl {
}
trait wxTabEvent {
}
trait wxTablesInUse {
}
trait wxTaskBarIcon {
    #[fixed_stack_segment]
    fn new() -> @wxTaskBarIcon {
        unsafe { wxTaskBarIcon_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTaskBarIcon_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isIconInstalled(&self) -> bool {
        unsafe { wxTaskBarIcon_IsIconInstalled(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxTaskBarIcon_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn popupMenu(&self, menu: @wxMenu) -> bool {
        unsafe { wxTaskBarIcon_PopupMenu(self, menu) }
    }
    #[fixed_stack_segment]
    fn removeIcon(&self) -> bool {
        unsafe { wxTaskBarIcon_RemoveIcon(self) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @wxIcon, text: @wxString) -> bool {
        unsafe { wxTaskBarIcon_SetIcon(self, icon, text) }
    }
}
trait wxTempFile {
}
trait wxTextAttr {
    #[fixed_stack_segment]
    fn new(colText: @wxColour, colBack: @wxColour, font: @wxFont) -> @wxTextAttr {
        unsafe { wxTextAttr_Create(colText, colBack, font) }
    }
    #[fixed_stack_segment]
    fn newDefault() -> @wxTextAttr {
        unsafe { wxTextAttr_CreateDefault() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, @wxColour) {
        unsafe { wxTextAttr_GetBackgroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, @wxFont) {
        unsafe { wxTextAttr_GetFont(self, font) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, @wxColour) {
        unsafe { wxTextAttr_GetTextColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn hasBackgroundColour(&self) -> bool {
        unsafe { wxTextAttr_HasBackgroundColour(self) }
    }
    #[fixed_stack_segment]
    fn hasFont(&self) -> bool {
        unsafe { wxTextAttr_HasFont(self) }
    }
    #[fixed_stack_segment]
    fn hasTextColour(&self) -> bool {
        unsafe { wxTextAttr_HasTextColour(self) }
    }
    #[fixed_stack_segment]
    fn isDefault(&self) -> bool {
        unsafe { wxTextAttr_IsDefault(self) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colour: @wxColour) {
        unsafe { wxTextAttr_SetTextColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @wxColour) {
        unsafe { wxTextAttr_SetBackgroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) {
        unsafe { wxTextAttr_SetFont(self, font) }
    }
}
trait wxTextCtrl {
    #[fixed_stack_segment]
    fn appendText(&self, text: @wxString) {
        unsafe { wxTextCtrl_AppendText(self, text) }
    }
    #[fixed_stack_segment]
    fn canCopy(&self) -> bool {
        unsafe { wxTextCtrl_CanCopy(self) }
    }
    #[fixed_stack_segment]
    fn canCut(&self) -> bool {
        unsafe { wxTextCtrl_CanCut(self) }
    }
    #[fixed_stack_segment]
    fn canPaste(&self) -> bool {
        unsafe { wxTextCtrl_CanPaste(self) }
    }
    #[fixed_stack_segment]
    fn canRedo(&self) -> bool {
        unsafe { wxTextCtrl_CanRedo(self) }
    }
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { wxTextCtrl_CanUndo(self) }
    }
    #[fixed_stack_segment]
    fn changeValue(&self, text: @wxString) {
        unsafe { wxTextCtrl_ChangeValue(self, text) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxTextCtrl_Clear(self) }
    }
    #[fixed_stack_segment]
    fn copy(&self) {
        unsafe { wxTextCtrl_Copy(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxTextCtrl {
        unsafe { wxTextCtrl_Create(_prt, _id, _txt, _arg3, _stl) }
    }
    #[fixed_stack_segment]
    fn cut(&self) {
        unsafe { wxTextCtrl_Cut(self) }
    }
    #[fixed_stack_segment]
    fn discardEdits(&self) {
        unsafe { wxTextCtrl_DiscardEdits(self) }
    }
    #[fixed_stack_segment]
    fn getInsertionPoint(&self) -> c_long {
        unsafe { wxTextCtrl_GetInsertionPoint(self) }
    }
    #[fixed_stack_segment]
    fn getLastPosition(&self) -> c_long {
        unsafe { wxTextCtrl_GetLastPosition(self) }
    }
    #[fixed_stack_segment]
    fn getLineLength(&self, lineNo: c_long) -> int {
        unsafe { wxTextCtrl_GetLineLength(self, lineNo) }
    }
    #[fixed_stack_segment]
    fn getLineText(&self, lineNo: c_long) -> @wxString {
        unsafe { wxTextCtrl_GetLineText(self, lineNo) }
    }
    #[fixed_stack_segment]
    fn getNumberOfLines(&self) -> int {
        unsafe { wxTextCtrl_GetNumberOfLines(self) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, from: *c_void, to: *c_void) {
        unsafe { wxTextCtrl_GetSelection(self, from, to) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> @wxString {
        unsafe { wxTextCtrl_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn isEditable(&self) -> bool {
        unsafe { wxTextCtrl_IsEditable(self) }
    }
    #[fixed_stack_segment]
    fn isModified(&self) -> bool {
        unsafe { wxTextCtrl_IsModified(self) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, file: @wxString) -> bool {
        unsafe { wxTextCtrl_LoadFile(self, file) }
    }
    #[fixed_stack_segment]
    fn paste(&self) {
        unsafe { wxTextCtrl_Paste(self) }
    }
    #[fixed_stack_segment]
    fn positionToXY(&self, pos: c_long, x: *c_long, y: *c_long) -> int {
        unsafe { wxTextCtrl_PositionToXY(self, pos, x, y) }
    }
    #[fixed_stack_segment]
    fn redo(&self) {
        unsafe { wxTextCtrl_Redo(self) }
    }
    #[fixed_stack_segment]
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_Remove(self, from, to) }
    }
    #[fixed_stack_segment]
    fn replace(&self, from: c_long, to: c_long, value: @wxString) {
        unsafe { wxTextCtrl_Replace(self, from, to, value) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, file: @wxString) -> bool {
        unsafe { wxTextCtrl_SaveFile(self, file) }
    }
    #[fixed_stack_segment]
    fn setEditable(&self, editable: bool) {
        unsafe { wxTextCtrl_SetEditable(self, editable) }
    }
    #[fixed_stack_segment]
    fn setInsertionPoint(&self, pos: c_long) {
        unsafe { wxTextCtrl_SetInsertionPoint(self, pos) }
    }
    #[fixed_stack_segment]
    fn setInsertionPointEnd(&self) {
        unsafe { wxTextCtrl_SetInsertionPointEnd(self) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, from: c_long, to: c_long) {
        unsafe { wxTextCtrl_SetSelection(self, from, to) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, value: @wxString) {
        unsafe { wxTextCtrl_SetValue(self, value) }
    }
    #[fixed_stack_segment]
    fn showPosition(&self, pos: c_long) {
        unsafe { wxTextCtrl_ShowPosition(self, pos) }
    }
    #[fixed_stack_segment]
    fn undo(&self) {
        unsafe { wxTextCtrl_Undo(self) }
    }
    #[fixed_stack_segment]
    fn writeText(&self, text: @wxString) {
        unsafe { wxTextCtrl_WriteText(self, text) }
    }
    #[fixed_stack_segment]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn emulateKeyPress(&self, keyevent: @wxKeyEvent) -> bool {
        unsafe { wxTextCtrl_EmulateKeyPress(self, keyevent) }
    }
    #[fixed_stack_segment]
    fn getDefaultStyle(&self) -> @wxTextAttr {
        unsafe { wxTextCtrl_GetDefaultStyle(self) }
    }
    #[fixed_stack_segment]
    fn getRange(&self, from: c_long, to: c_long) -> @wxString {
        unsafe { wxTextCtrl_GetRange(self, from, to) }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @wxString {
        unsafe { wxTextCtrl_GetStringSelection(self) }
    }
    #[fixed_stack_segment]
    fn isMultiLine(&self) -> bool {
        unsafe { wxTextCtrl_IsMultiLine(self) }
    }
    #[fixed_stack_segment]
    fn isSingleLine(&self) -> bool {
        unsafe { wxTextCtrl_IsSingleLine(self) }
    }
    #[fixed_stack_segment]
    fn setDefaultStyle(&self, style: @wxTextAttr) -> bool {
        unsafe { wxTextCtrl_SetDefaultStyle(self, style) }
    }
    #[fixed_stack_segment]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self, len) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, start: c_long, end: c_long, style: @wxTextAttr) -> bool {
        unsafe { wxTextCtrl_SetStyle(self, start, end, style) }
    }
}
trait wxTextDataObject {
}
trait wxTextDropTarget {
}
trait wxTextEntryDialog {
}
trait wxTextFile {
}
trait wxTextInputStream {
    #[fixed_stack_segment]
    fn new(inputStream: @wxInputStream, sep: @wxString) -> @wxTextInputStream {
        unsafe { wxTextInputStream_Create(inputStream, sep) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self) }
    }
    #[fixed_stack_segment]
    fn readLine(&self) -> @wxString {
        unsafe { wxTextInputStream_ReadLine(self) }
    }
}
trait wxTextOutputStream {
    #[fixed_stack_segment]
    fn new(outputStream: @wxOutputStream, mode: int) -> @wxTextOutputStream {
        unsafe { wxTextOutputStream_Create(outputStream, mode) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, txt: @wxString) {
        unsafe { wxTextOutputStream_WriteString(self, txt) }
    }
}
trait wxTextValidator {
    #[fixed_stack_segment]
    fn new(style: int, val: *c_void) -> @wxTextValidator {
        unsafe { wxTextValidator_Create(style, val) }
    }
    #[fixed_stack_segment]
    fn getExcludes(&self, _ref: **wchar_t) -> c_int {
        unsafe { wxTextValidator_GetExcludes(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getIncludes(&self, _ref: **wchar_t) -> c_int {
        unsafe { wxTextValidator_GetIncludes(self, _ref) }
    }
    #[fixed_stack_segment]
    fn setExcludes(&self, list: *wchar_t, count: int) {
        unsafe { wxTextValidator_SetExcludes(self, list, count) }
    }
    #[fixed_stack_segment]
    fn setIncludes(&self, list: *wchar_t, count: int) {
        unsafe { wxTextValidator_SetIncludes(self, list, count) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @wxValidator {
        unsafe { wxTextValidator_Clone(self) }
    }
    #[fixed_stack_segment]
    fn transferToWindow(&self) -> bool {
        unsafe { wxTextValidator_TransferToWindow(self) }
    }
    #[fixed_stack_segment]
    fn transferFromWindow(&self) -> bool {
        unsafe { wxTextValidator_TransferFromWindow(self) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> int {
        unsafe { wxTextValidator_GetStyle(self) }
    }
    #[fixed_stack_segment]
    fn onChar(&self, event: @wxEvent) {
        unsafe { wxTextValidator_OnChar(self, event) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: int) {
        unsafe { wxTextValidator_SetStyle(self, style) }
    }
}
trait wxThinSplitterWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxThinSplitterWindow {
        unsafe { wxThinSplitterWindow_Create(parent, id, _arg2, style) }
    }
    #[fixed_stack_segment]
    fn drawSash(&self, dc: @wxDC) {
        unsafe { wxThinSplitterWindow_DrawSash(self, dc) }
    }
    #[fixed_stack_segment]
    fn sashHitTest(&self, x: c_int, y: c_int, tolerance: int) -> int {
        unsafe { wxThinSplitterWindow_SashHitTest(self, _arg1, tolerance) }
    }
    #[fixed_stack_segment]
    fn sizeWindows(&self) {
        unsafe { wxThinSplitterWindow_SizeWindows(self) }
    }
}
trait wxThread {
}
trait wxTime {
}
trait wxTimeSpan {
}
trait wxTimer {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int) -> @wxTimer {
        unsafe { wxTimer_Create(_prt, _id) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTimer_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getInterval(&self) -> int {
        unsafe { wxTimer_GetInterval(self) }
    }
    #[fixed_stack_segment]
    fn isOneShot(&self) -> bool {
        unsafe { wxTimer_IsOneShot(self) }
    }
    #[fixed_stack_segment]
    fn isRuning(&self) -> bool {
        unsafe { wxTimer_IsRuning(self) }
    }
    #[fixed_stack_segment]
    fn start(&self, _int: int, _one: bool) -> bool {
        unsafe { wxTimer_Start(self, _int, _one) }
    }
    #[fixed_stack_segment]
    fn stop(&self) {
        unsafe { wxTimer_Stop(self) }
    }
}
trait wxTimerBase {
}
trait wxTimerEvent {
    #[fixed_stack_segment]
    fn getInterval(&self) -> int {
        unsafe { wxTimerEvent_GetInterval(self) }
    }
}
trait wxTimerEx {
    #[fixed_stack_segment]
    fn connect(&self, closure: @wxClosure) {
        unsafe { wxTimerEx_Connect(self, closure) }
    }
    #[fixed_stack_segment]
    fn new() -> @wxTimerEx {
        unsafe { wxTimerEx_Create() }
    }
    #[fixed_stack_segment]
    fn getClosure(&self) -> @wxClosure {
        unsafe { wxTimerEx_GetClosure(self) }
    }
}
trait wxTimerRunner {
}
trait wxTipProvider {
}
trait wxTipWindow {
    #[fixed_stack_segment]
    fn close(&self) {
        unsafe { wxTipWindow_Close(self) }
    }
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, text: @wxString, maxLength: int) -> @wxTipWindow {
        unsafe { wxTipWindow_Create(parent, text, maxLength) }
    }
    #[fixed_stack_segment]
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setTipWindowPtr(&self, windowPtr: *c_void) {
        unsafe { wxTipWindow_SetTipWindowPtr(self, windowPtr) }
    }
}
trait wxToggleButton {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, label: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxToggleButton {
        unsafe { wxToggleButton_Create(parent, id, label, _arg3, style) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> bool {
        unsafe { wxToggleButton_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxToggleButton_GetValue(self) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, label: @wxString) {
        unsafe { wxToggleButton_SetLabel(self, label) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, state: bool) {
        unsafe { wxToggleButton_SetValue(self, state) }
    }
}
trait wxToolBar {
    #[fixed_stack_segment]
    fn addControl(&self, ctrl: @wxControl) -> bool {
        unsafe { wxToolBar_AddControl(self, ctrl) }
    }
    #[fixed_stack_segment]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self) }
    }
    #[fixed_stack_segment]
    fn addTool(&self, id: int, bmp: @wxBitmap, shelp: @wxString, lhelp: @wxString) {
        unsafe { wxToolBar_AddTool(self, id, bmp, shelp, lhelp) }
    }
    #[fixed_stack_segment]
    fn addToolEx(&self, id: int, bmp1: @wxBitmap, bmp2: @wxBitmap, isToggle: bool, x: c_int, y: c_int, data: @wxObject, shelp: @wxString, lhelp: @wxString) {
        unsafe { wxToolBar_AddToolEx(self, id, bmp1, bmp2, isToggle, _arg5, data, shelp, lhelp) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxToolBar {
        unsafe { wxToolBar_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxToolBar_Delete(self) }
    }
    #[fixed_stack_segment]
    fn deleteTool(&self, id: int) -> bool {
        unsafe { wxToolBar_DeleteTool(self, id) }
    }
    #[fixed_stack_segment]
    fn deleteToolByPos(&self, pos: int) -> bool {
        unsafe { wxToolBar_DeleteToolByPos(self, pos) }
    }
    #[fixed_stack_segment]
    fn enableTool(&self, id: int, enable: bool) {
        unsafe { wxToolBar_EnableTool(self, id, enable) }
    }
    #[fixed_stack_segment]
    fn getMargins(&self) -> @wxPoint {
        unsafe { wxToolBar_GetMargins(self) }
    }
    #[fixed_stack_segment]
    fn getToolBitmapSize(&self) -> @wxSize {
        unsafe { wxToolBar_GetToolBitmapSize(self) }
    }
    #[fixed_stack_segment]
    fn getToolClientData(&self, id: int) -> @wxObject {
        unsafe { wxToolBar_GetToolClientData(self, id) }
    }
    #[fixed_stack_segment]
    fn getToolEnabled(&self, id: int) -> bool {
        unsafe { wxToolBar_GetToolEnabled(self, id) }
    }
    #[fixed_stack_segment]
    fn getToolLongHelp(&self, id: int) -> @wxString {
        unsafe { wxToolBar_GetToolLongHelp(self, id) }
    }
    #[fixed_stack_segment]
    fn getToolPacking(&self) -> int {
        unsafe { wxToolBar_GetToolPacking(self) }
    }
    #[fixed_stack_segment]
    fn getToolShortHelp(&self, id: int) -> @wxString {
        unsafe { wxToolBar_GetToolShortHelp(self, id) }
    }
    #[fixed_stack_segment]
    fn getToolSize(&self) -> @wxSize {
        unsafe { wxToolBar_GetToolSize(self) }
    }
    #[fixed_stack_segment]
    fn getToolState(&self, id: int) -> bool {
        unsafe { wxToolBar_GetToolState(self, id) }
    }
    #[fixed_stack_segment]
    fn insertControl(&self, pos: int, ctrl: @wxControl) {
        unsafe { wxToolBar_InsertControl(self, pos, ctrl) }
    }
    #[fixed_stack_segment]
    fn insertSeparator(&self, pos: int) {
        unsafe { wxToolBar_InsertSeparator(self, pos) }
    }
    #[fixed_stack_segment]
    fn insertTool(&self, pos: int, id: int, bmp1: @wxBitmap, bmp2: @wxBitmap, isToggle: bool, data: @wxObject, shelp: @wxString, lhelp: @wxString) {
        unsafe { wxToolBar_InsertTool(self, pos, id, bmp1, bmp2, isToggle, data, shelp, lhelp) }
    }
    #[fixed_stack_segment]
    fn realize(&self) -> bool {
        unsafe { wxToolBar_Realize(self) }
    }
    #[fixed_stack_segment]
    fn removeTool(&self, id: int) {
        unsafe { wxToolBar_RemoveTool(self, id) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetMargins(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setToolBitmapSize(&self, x: c_int, y: c_int) {
        unsafe { wxToolBar_SetToolBitmapSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setToolClientData(&self, id: int, data: @wxObject) {
        unsafe { wxToolBar_SetToolClientData(self, id, data) }
    }
    #[fixed_stack_segment]
    fn setToolLongHelp(&self, id: int, str: @wxString) {
        unsafe { wxToolBar_SetToolLongHelp(self, id, str) }
    }
    #[fixed_stack_segment]
    fn setToolPacking(&self, packing: int) {
        unsafe { wxToolBar_SetToolPacking(self, packing) }
    }
    #[fixed_stack_segment]
    fn setToolSeparation(&self, separation: int) {
        unsafe { wxToolBar_SetToolSeparation(self, separation) }
    }
    #[fixed_stack_segment]
    fn setToolShortHelp(&self, id: int, str: @wxString) {
        unsafe { wxToolBar_SetToolShortHelp(self, id, str) }
    }
    #[fixed_stack_segment]
    fn toggleTool(&self, id: int, toggle: bool) {
        unsafe { wxToolBar_ToggleTool(self, id, toggle) }
    }
    #[fixed_stack_segment]
    fn addTool2(&self, toolId: int, label: @wxString, bmp: @wxBitmap, bmpDisabled: @wxBitmap, itemKind: int, shortHelp: @wxString, longHelp: @wxString) {
        unsafe { wxToolBar_AddTool2(self, toolId, label, bmp, bmpDisabled, itemKind, shortHelp, longHelp) }
    }
}
trait wxToolBarBase {
}
trait wxToolLayoutItem {
    #[fixed_stack_segment]
    fn isSeparator(&self) -> bool {
        unsafe { wxToolLayoutItem_IsSeparator(self) }
    }
    #[fixed_stack_segment]
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxToolLayoutItem_Rect(self, _arg1) }
    }
}
trait wxToolTip {
}
trait wxToolWindow {
    #[fixed_stack_segment]
    fn addMiniButton(&self, _btn: *c_void) {
        unsafe { wxToolWindow_AddMiniButton(self, _btn) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _btn: *c_void, _ttl: *c_void) -> @wxToolWindow {
        unsafe { wxToolWindow_Create(_obj, _btn, _ttl) }
    }
    #[fixed_stack_segment]
    fn getClient(&self) -> @wxClient {
        unsafe { wxToolWindow_GetClient(self) }
    }
    #[fixed_stack_segment]
    fn setClient(&self, _wnd: @wxWindow) {
        unsafe { wxToolWindow_SetClient(self, _wnd) }
    }
    #[fixed_stack_segment]
    fn setTitleFont(&self, _fnt: *c_void) {
        unsafe { wxToolWindow_SetTitleFont(self, _fnt) }
    }
}
trait wxTopLevelWindow {
    #[fixed_stack_segment]
    fn enableCloseButton(&self, enable: bool) -> bool {
        unsafe { wxTopLevelWindow_EnableCloseButton(self, enable) }
    }
    #[fixed_stack_segment]
    fn getDefaultButton(&self) -> @wxButton {
        unsafe { wxTopLevelWindow_GetDefaultButton(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultItem(&self) -> @wxWindow {
        unsafe { wxTopLevelWindow_GetDefaultItem(self) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self) -> @wxIcon {
        unsafe { wxTopLevelWindow_GetIcon(self) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @wxString {
        unsafe { wxTopLevelWindow_GetTitle(self) }
    }
    #[fixed_stack_segment]
    fn iconize(&self, iconize: bool) -> bool {
        unsafe { wxTopLevelWindow_Iconize(self, iconize) }
    }
    #[fixed_stack_segment]
    fn isActive(&self) -> bool {
        unsafe { wxTopLevelWindow_IsActive(self) }
    }
    #[fixed_stack_segment]
    fn isIconized(&self) -> bool {
        unsafe { wxTopLevelWindow_IsIconized(self) }
    }
    #[fixed_stack_segment]
    fn isMaximized(&self) -> bool {
        unsafe { wxTopLevelWindow_IsMaximized(self) }
    }
    #[fixed_stack_segment]
    fn maximize(&self, maximize: bool) {
        unsafe { wxTopLevelWindow_Maximize(self, maximize) }
    }
    #[fixed_stack_segment]
    fn requestUserAttention(&self, flags: int) {
        unsafe { wxTopLevelWindow_RequestUserAttention(self, flags) }
    }
    #[fixed_stack_segment]
    fn setDefaultButton(&self, pBut: @wxButton) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self, pBut) }
    }
    #[fixed_stack_segment]
    fn setDefaultItem(&self, pBut: @wxWindow) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self, pBut) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, pIcon: @wxIcon) {
        unsafe { wxTopLevelWindow_SetIcon(self, pIcon) }
    }
    #[fixed_stack_segment]
    fn setIcons(&self, _icons: *c_void) {
        unsafe { wxTopLevelWindow_SetIcons(self, _icons) }
    }
    #[fixed_stack_segment]
    fn setMaxSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMaxSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, w: c_int, h: c_int) {
        unsafe { wxTopLevelWindow_SetMinSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, pString: @wxString) {
        unsafe { wxTopLevelWindow_SetTitle(self, pString) }
    }
}
trait wxTreeCompanionWindow {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxTreeCompanionWindow {
        unsafe { wxTreeCompanionWindow_Create(parent, id, _arg2, style) }
    }
    #[fixed_stack_segment]
    fn drawItem(&self, dc: @wxDC, id: *c_void, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTreeCompanionWindow_DrawItem(self, dc, id, _arg3) }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl(&self) -> @wxTreeCtrl {
        unsafe { wxTreeCompanionWindow_GetTreeCtrl(self) }
    }
    #[fixed_stack_segment]
    fn setTreeCtrl(&self, treeCtrl: @wxTreeCtrl) {
        unsafe { wxTreeCompanionWindow_SetTreeCtrl(self, treeCtrl) }
    }
}
trait wxTreeCtrl {
    #[fixed_stack_segment]
    fn addRoot(&self, text: @wxString, image: int, selectedImage: int, data: @wxTreeItemData, @wxTreeItemId) {
        unsafe { wxTreeCtrl_AddRoot(self, text, image, selectedImage, data, _item) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, parent: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, data: @wxTreeItemData, @wxTreeItemId) {
        unsafe { wxTreeCtrl_AppendItem(self, parent, text, image, selectedImage, data, _item) }
    }
    #[fixed_stack_segment]
    fn collapse(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Collapse(self, item) }
    }
    #[fixed_stack_segment]
    fn collapseAndReset(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_CollapseAndReset(self, item) }
    }
    #[fixed_stack_segment]
    fn new(_obj: *c_void, _cmp: *c_void, _prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxTreeCtrl {
        unsafe { wxTreeCtrl_Create(_obj, _cmp, _prt, _id, _arg4, _stl) }
    }
    #[fixed_stack_segment]
    fn delete(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Delete(self, item) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self) }
    }
    #[fixed_stack_segment]
    fn deleteChildren(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_DeleteChildren(self, item) }
    }
    #[fixed_stack_segment]
    fn editLabel(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_EditLabel(self, item) }
    }
    #[fixed_stack_segment]
    fn endEditLabel(&self, item: @wxTreeItemId, discardChanges: bool) {
        unsafe { wxTreeCtrl_EndEditLabel(self, item, discardChanges) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_EnsureVisible(self, item) }
    }
    #[fixed_stack_segment]
    fn expand(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Expand(self, item) }
    }
    #[fixed_stack_segment]
    fn getBoundingRect(&self, item: @wxTreeItemId, textOnly: bool) -> @wxRect {
        unsafe { wxTreeCtrl_GetBoundingRect(self, item, textOnly) }
    }
    #[fixed_stack_segment]
    fn getChildrenCount(&self, item: @wxTreeItemId, recursively: bool) -> int {
        unsafe { wxTreeCtrl_GetChildrenCount(self, item, recursively) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> int {
        unsafe { wxTreeCtrl_GetCount(self) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> @wxTextCtrl {
        unsafe { wxTreeCtrl_GetEditControl(self) }
    }
    #[fixed_stack_segment]
    fn getFirstChild(&self, item: @wxTreeItemId, cookie: *int, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetFirstChild(self, item, cookie, _item) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleItem(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> @wxImageList {
        unsafe { wxTreeCtrl_GetImageList(self) }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> int {
        unsafe { wxTreeCtrl_GetIndent(self) }
    }
    #[fixed_stack_segment]
    fn getItemData(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetItemData(self, item) }
    }
    #[fixed_stack_segment]
    fn getItemImage(&self, item: @wxTreeItemId, which: int) -> int {
        unsafe { wxTreeCtrl_GetItemImage(self, item, which) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: @wxTreeItemId) -> @wxString {
        unsafe { wxTreeCtrl_GetItemText(self, item) }
    }
    #[fixed_stack_segment]
    fn getLastChild(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetLastChild(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getNextChild(&self, item: @wxTreeItemId, cookie: *int, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextChild(self, item, cookie, _item) }
    }
    #[fixed_stack_segment]
    fn getNextSibling(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextSibling(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getNextVisible(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetNextVisible(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getParent(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetParent(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getPrevSibling(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetPrevSibling(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getPrevVisible(&self, item: @wxTreeItemId, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetPrevVisible(self, item, _item) }
    }
    #[fixed_stack_segment]
    fn getRootItem(&self, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetRootItem(self, _item) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, @wxTreeItemId) {
        unsafe { wxTreeCtrl_GetSelection(self, _item) }
    }
    #[fixed_stack_segment]
    fn getSelections(&self, selections: *intptr_t) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self, selections) }
    }
    #[fixed_stack_segment]
    fn getSpacing(&self) -> int {
        unsafe { wxTreeCtrl_GetSpacing(self) }
    }
    #[fixed_stack_segment]
    fn getStateImageList(&self) -> @wxImageList {
        unsafe { wxTreeCtrl_GetStateImageList(self) }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, _x: c_int, _y: c_int, flags: *int, @wxTreeItemId) {
        unsafe { wxTreeCtrl_HitTest(self, _arg1, flags, _item) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, parent: @wxTreeItemId, idPrevious: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, data: *c_void, @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItem(self, parent, idPrevious, text, image, selectedImage, data, _item) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex(&self, parent: @wxTreeItemId, index: int, text: @wxString, image: int, selectedImage: int, data: *c_void, @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex(self, parent, index, text, image, selectedImage, data, _item) }
    }
    #[fixed_stack_segment]
    fn isBold(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsBold(self, item) }
    }
    #[fixed_stack_segment]
    fn isExpanded(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsExpanded(self, item) }
    }
    #[fixed_stack_segment]
    fn isSelected(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsSelected(self, item) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self, item: @wxTreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsVisible(self, item) }
    }
    #[fixed_stack_segment]
    fn itemHasChildren(&self, item: @wxTreeItemId) -> int {
        unsafe { wxTreeCtrl_ItemHasChildren(self, item) }
    }
    #[fixed_stack_segment]
    fn onCompareItems(&self, item1: @wxTreeItemId, item2: @wxTreeItemId) -> int {
        unsafe { wxTreeCtrl_OnCompareItems(self, item1, item2) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, parent: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, data: *c_void, @wxTreeItemId) {
        unsafe { wxTreeCtrl_PrependItem(self, parent, text, image, selectedImage, data, _item) }
    }
    #[fixed_stack_segment]
    fn scrollTo(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_ScrollTo(self, item) }
    }
    #[fixed_stack_segment]
    fn selectItem(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_SelectItem(self, item) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_SetImageList(self, imageList) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indent: int) {
        unsafe { wxTreeCtrl_SetIndent(self, indent) }
    }
    #[fixed_stack_segment]
    fn setItemBackgroundColour(&self, item: @wxTreeItemId, col: @wxColour) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self, item, col) }
    }
    #[fixed_stack_segment]
    fn setItemBold(&self, item: @wxTreeItemId, bold: bool) {
        unsafe { wxTreeCtrl_SetItemBold(self, item, bold) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: @wxTreeItemId, data: *c_void) {
        unsafe { wxTreeCtrl_SetItemData(self, item, data) }
    }
    #[fixed_stack_segment]
    fn setItemDropHighlight(&self, item: @wxTreeItemId, highlight: bool) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self, item, highlight) }
    }
    #[fixed_stack_segment]
    fn setItemFont(&self, item: @wxTreeItemId, font: @wxFont) {
        unsafe { wxTreeCtrl_SetItemFont(self, item, font) }
    }
    #[fixed_stack_segment]
    fn setItemHasChildren(&self, item: @wxTreeItemId, hasChildren: bool) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self, item, hasChildren) }
    }
    #[fixed_stack_segment]
    fn setItemImage(&self, item: @wxTreeItemId, image: int, which: int) {
        unsafe { wxTreeCtrl_SetItemImage(self, item, image, which) }
    }
    #[fixed_stack_segment]
    fn setItemText(&self, item: @wxTreeItemId, text: @wxString) {
        unsafe { wxTreeCtrl_SetItemText(self, item, text) }
    }
    #[fixed_stack_segment]
    fn setItemTextColour(&self, item: @wxTreeItemId, col: @wxColour) {
        unsafe { wxTreeCtrl_SetItemTextColour(self, item, col) }
    }
    #[fixed_stack_segment]
    fn setSpacing(&self, spacing: int) {
        unsafe { wxTreeCtrl_SetSpacing(self, spacing) }
    }
    #[fixed_stack_segment]
    fn setStateImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_SetStateImageList(self, imageList) }
    }
    #[fixed_stack_segment]
    fn sortChildren(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_SortChildren(self, item) }
    }
    #[fixed_stack_segment]
    fn toggle(&self, item: @wxTreeItemId) {
        unsafe { wxTreeCtrl_Toggle(self, item) }
    }
    #[fixed_stack_segment]
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self) }
    }
    #[fixed_stack_segment]
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self) }
    }
    #[fixed_stack_segment]
    fn new2(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxTreeCtrl {
        unsafe { wxTreeCtrl_Create2(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn insertItem2(&self, parent: @wxWindow, idPrevious: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, closure: @wxClosure, @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItem2(self, parent, idPrevious, text, image, selectedImage, closure, _item) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex2(&self, parent: @wxWindow, index: int, text: @wxString, image: int, selectedImage: int, closure: @wxClosure, @wxTreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex2(self, parent, index, text, image, selectedImage, closure, _item) }
    }
    #[fixed_stack_segment]
    fn getItemClientClosure(&self, item: @wxTreeItemId) -> @wxClosure {
        unsafe { wxTreeCtrl_GetItemClientClosure(self, item) }
    }
    #[fixed_stack_segment]
    fn setItemClientClosure(&self, item: @wxTreeItemId, closure: @wxClosure) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self, item, closure) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_AssignImageList(self, imageList) }
    }
    #[fixed_stack_segment]
    fn assignStateImageList(&self, imageList: @wxImageList) {
        unsafe { wxTreeCtrl_AssignStateImageList(self, imageList) }
    }
}
trait wxTreeEvent {
    #[fixed_stack_segment]
    fn getCode(&self) -> int {
        unsafe { wxTreeEvent_GetCode(self) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, @wxTreeItemId) {
        unsafe { wxTreeEvent_GetItem(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { wxTreeEvent_GetLabel(self) }
    }
    #[fixed_stack_segment]
    fn getOldItem(&self, @wxTreeItemId) {
        unsafe { wxTreeEvent_GetOldItem(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @wxPoint {
        unsafe { wxTreeEvent_GetPoint(self) }
    }
    #[fixed_stack_segment]
    fn getKeyEvent(&self) -> @wxKeyEvent {
        unsafe { wxTreeEvent_GetKeyEvent(self) }
    }
    #[fixed_stack_segment]
    fn isEditCancelled(&self) -> int {
        unsafe { wxTreeEvent_IsEditCancelled(self) }
    }
    #[fixed_stack_segment]
    fn allow(&self) {
        unsafe { wxTreeEvent_Allow(self) }
    }
}
trait wxTreeItemData {
}
trait wxTreeItemId {
    #[fixed_stack_segment]
    fn new() -> @wxTreeItemId {
        unsafe { wxTreeItemId_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxTreeItemId_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @wxTreeItemId {
        unsafe { wxTreeItemId_Clone(self) }
    }
    #[fixed_stack_segment]
    fn newFromValue(value: intptr_t) -> @wxTreeItemId {
        unsafe { wxTreeItemId_CreateFromValue(value) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self) }
    }
}
trait wxTreeLayout {
}
trait wxTreeLayoutStored {
}
trait wxURL {
}
trait wxUpdateUIEvent {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxUpdateUIEvent_Check(self, check) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @wxObject) {
        unsafe { wxUpdateUIEvent_CopyObject(self, obj) }
    }
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) {
        unsafe { wxUpdateUIEvent_Enable(self, enable) }
    }
    #[fixed_stack_segment]
    fn getChecked(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetChecked(self) }
    }
    #[fixed_stack_segment]
    fn getEnabled(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetEnabled(self) }
    }
    #[fixed_stack_segment]
    fn getSetChecked(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetChecked(self) }
    }
    #[fixed_stack_segment]
    fn getSetEnabled(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetEnabled(self) }
    }
    #[fixed_stack_segment]
    fn getSetText(&self) -> bool {
        unsafe { wxUpdateUIEvent_GetSetText(self) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { wxUpdateUIEvent_GetText(self) }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @wxString) {
        unsafe { wxUpdateUIEvent_SetText(self, text) }
    }
}
trait wxValidator {
    #[fixed_stack_segment]
    fn new() -> @wxValidator {
        unsafe { wxValidator_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxValidator_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { wxValidator_GetWindow(self) }
    }
    #[fixed_stack_segment]
    fn setBellOnError(doIt: bool) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, win: @wxWindow) {
        unsafe { wxValidator_SetWindow(self, win) }
    }
    #[fixed_stack_segment]
    fn transferFromWindow(&self) -> bool {
        unsafe { wxValidator_TransferFromWindow(self) }
    }
    #[fixed_stack_segment]
    fn transferToWindow(&self) -> bool {
        unsafe { wxValidator_TransferToWindow(self) }
    }
    #[fixed_stack_segment]
    fn validate(&self, parent: @wxWindow) -> bool {
        unsafe { wxValidator_Validate(self, parent) }
    }
}
trait wxVariant {
}
trait wxVariantData {
}
trait wxView {
}
trait wxSound {
    #[fixed_stack_segment]
    fn new(fileName: @wxString, isResource: bool) -> @wxSound {
        unsafe { wxSound_Create(fileName, isResource) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSound_Delete(self) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxSound_IsOk(self) }
    }
    #[fixed_stack_segment]
    fn play(&self, flag: int) -> bool {
        unsafe { wxSound_Play(self, flag) }
    }
    #[fixed_stack_segment]
    fn stop(&self) {
        unsafe { wxSound_Stop(self) }
    }
}
trait wxWindow {
    #[fixed_stack_segment]
    fn addChild(&self, child: @wxWindow) {
        unsafe { wxWindow_AddChild(self, child) }
    }
    #[fixed_stack_segment]
    fn addConstraintReference(&self, otherWin: @wxWindow) {
        unsafe { wxWindow_AddConstraintReference(self, otherWin) }
    }
    #[fixed_stack_segment]
    fn captureMouse(&self) {
        unsafe { wxWindow_CaptureMouse(self) }
    }
    #[fixed_stack_segment]
    fn center(&self, direction: int) {
        unsafe { wxWindow_Center(self, direction) }
    }
    #[fixed_stack_segment]
    fn centerOnParent(&self, dir: int) {
        unsafe { wxWindow_CenterOnParent(self, dir) }
    }
    #[fixed_stack_segment]
    fn clearBackground(&self) {
        unsafe { wxWindow_ClearBackground(self) }
    }
    #[fixed_stack_segment]
    fn clientToScreen(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { wxWindow_ClientToScreen(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn close(&self, _force: bool) -> bool {
        unsafe { wxWindow_Close(self, _force) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixels(&self) -> @wxPoint {
        unsafe { wxWindow_ConvertDialogToPixels(self) }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialog(&self) -> @wxPoint {
        unsafe { wxWindow_ConvertPixelsToDialog(self) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: int) -> @wxWindow {
        unsafe { wxWindow_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn deleteRelatedConstraints(&self) {
        unsafe { wxWindow_DeleteRelatedConstraints(self) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) -> bool {
        unsafe { wxWindow_Destroy(self) }
    }
    #[fixed_stack_segment]
    fn destroyChildren(&self) -> bool {
        unsafe { wxWindow_DestroyChildren(self) }
    }
    #[fixed_stack_segment]
    fn disable(&self) -> bool {
        unsafe { wxWindow_Disable(self) }
    }
    #[fixed_stack_segment]
    fn doPhase(&self, phase: int) -> int {
        unsafe { wxWindow_DoPhase(self, phase) }
    }
    #[fixed_stack_segment]
    fn enable(&self) -> bool {
        unsafe { wxWindow_Enable(self) }
    }
    #[fixed_stack_segment]
    fn findFocus(&self) -> @wxWindow {
        unsafe { wxWindow_FindFocus(self) }
    }
    #[fixed_stack_segment]
    fn findWindow(&self, name: @wxString) -> @wxWindow {
        unsafe { wxWindow_FindWindow(self, name) }
    }
    #[fixed_stack_segment]
    fn fit(&self) {
        unsafe { wxWindow_Fit(self) }
    }
    #[fixed_stack_segment]
    fn fitInside(&self) {
        unsafe { wxWindow_FitInside(self) }
    }
    #[fixed_stack_segment]
    fn freeze(&self) {
        unsafe { wxWindow_Freeze(self) }
    }
    #[fixed_stack_segment]
    fn getEffectiveMinSize(&self) -> @wxSize {
        unsafe { wxWindow_GetEffectiveMinSize(self) }
    }
    #[fixed_stack_segment]
    fn getAutoLayout(&self) -> int {
        unsafe { wxWindow_GetAutoLayout(self) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, @wxColour) {
        unsafe { wxWindow_GetBackgroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @wxSize {
        unsafe { wxWindow_GetBestSize(self) }
    }
    #[fixed_stack_segment]
    fn getCaret(&self) -> @wxCaret {
        unsafe { wxWindow_GetCaret(self) }
    }
    #[fixed_stack_segment]
    fn getCharHeight(&self) -> int {
        unsafe { wxWindow_GetCharHeight(self) }
    }
    #[fixed_stack_segment]
    fn getCharWidth(&self) -> int {
        unsafe { wxWindow_GetCharWidth(self) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *c_void, _cnt: int) -> int {
        unsafe { wxWindow_GetChildren(self, _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @wxClientData {
        unsafe { wxWindow_GetClientData(self) }
    }
    #[fixed_stack_segment]
    fn getClientSize(&self) -> @wxSize {
        unsafe { wxWindow_GetClientSize(self) }
    }
    #[fixed_stack_segment]
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getConstraints(&self) -> @wxLayoutConstraints {
        unsafe { wxWindow_GetConstraints(self) }
    }
    #[fixed_stack_segment]
    fn getConstraintsInvolvedIn(&self) {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self) }
    }
    #[fixed_stack_segment]
    fn getCursor(&self) -> @wxCursor {
        unsafe { wxWindow_GetCursor(self) }
    }
    #[fixed_stack_segment]
    fn getDropTarget(&self) -> @wxDropTarget {
        unsafe { wxWindow_GetDropTarget(self) }
    }
    #[fixed_stack_segment]
    fn getEventHandler(&self) -> @wxEvtHandler {
        unsafe { wxWindow_GetEventHandler(self) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, @wxFont) {
        unsafe { wxWindow_GetFont(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getForegroundColour(&self, @wxColour) {
        unsafe { wxWindow_GetForegroundColour(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getHandle(&self) {
        unsafe { wxWindow_GetHandle(self) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> int {
        unsafe { wxWindow_GetId(self) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { wxWindow_GetLabel(self) }
    }
    #[fixed_stack_segment]
    fn getLabelEmpty(&self) -> int {
        unsafe { wxWindow_GetLabelEmpty(self) }
    }
    #[fixed_stack_segment]
    fn getMaxHeight(&self) -> int {
        unsafe { wxWindow_GetMaxHeight(self) }
    }
    #[fixed_stack_segment]
    fn getMaxWidth(&self) -> int {
        unsafe { wxWindow_GetMaxWidth(self) }
    }
    #[fixed_stack_segment]
    fn getMinHeight(&self) -> int {
        unsafe { wxWindow_GetMinHeight(self) }
    }
    #[fixed_stack_segment]
    fn getMinWidth(&self) -> int {
        unsafe { wxWindow_GetMinWidth(self) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { wxWindow_GetName(self) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @wxWindow {
        unsafe { wxWindow_GetParent(self) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @wxPoint {
        unsafe { wxWindow_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxWindow_GetPositionConstraint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @wxRect {
        unsafe { wxWindow_GetRect(self) }
    }
    #[fixed_stack_segment]
    fn getScrollPos(&self, orient: int) -> int {
        unsafe { wxWindow_GetScrollPos(self, orient) }
    }
    #[fixed_stack_segment]
    fn getScrollRange(&self, orient: int) -> int {
        unsafe { wxWindow_GetScrollRange(self, orient) }
    }
    #[fixed_stack_segment]
    fn getScrollThumb(&self, orient: int) -> int {
        unsafe { wxWindow_GetScrollThumb(self, orient) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @wxSize {
        unsafe { wxWindow_GetSize(self) }
    }
    #[fixed_stack_segment]
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetSizeConstraint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @wxSizer {
        unsafe { wxWindow_GetSizer(self) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @wxString, x: *int, y: *int, descent: *int, externalLeading: *int, theFont: @wxFont) {
        unsafe { wxWindow_GetTextExtent(self, string, x, y, descent, externalLeading, theFont) }
    }
    #[fixed_stack_segment]
    fn getToolTip(&self) -> @wxString {
        unsafe { wxWindow_GetToolTip(self) }
    }
    #[fixed_stack_segment]
    fn getUpdateRegion(&self) -> @wxRegion {
        unsafe { wxWindow_GetUpdateRegion(self) }
    }
    #[fixed_stack_segment]
    fn getValidator(&self) -> @wxValidator {
        unsafe { wxWindow_GetValidator(self) }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self) -> @wxSize {
        unsafe { wxWindow_GetVirtualSize(self) }
    }
    #[fixed_stack_segment]
    fn getWindowStyleFlag(&self) -> int {
        unsafe { wxWindow_GetWindowStyleFlag(self) }
    }
    #[fixed_stack_segment]
    fn hasFlag(&self, flag: int) -> bool {
        unsafe { wxWindow_HasFlag(self, flag) }
    }
    #[fixed_stack_segment]
    fn hide(&self) -> bool {
        unsafe { wxWindow_Hide(self) }
    }
    #[fixed_stack_segment]
    fn initDialog(&self) {
        unsafe { wxWindow_InitDialog(self) }
    }
    #[fixed_stack_segment]
    fn isBeingDeleted(&self) -> bool {
        unsafe { wxWindow_IsBeingDeleted(self) }
    }
    #[fixed_stack_segment]
    fn isEnabled(&self) -> bool {
        unsafe { wxWindow_IsEnabled(self) }
    }
    #[fixed_stack_segment]
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { wxWindow_IsExposed(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn isShown(&self) -> bool {
        unsafe { wxWindow_IsShown(self) }
    }
    #[fixed_stack_segment]
    fn isTopLevel(&self) -> bool {
        unsafe { wxWindow_IsTopLevel(self) }
    }
    #[fixed_stack_segment]
    fn layout(&self) -> int {
        unsafe { wxWindow_Layout(self) }
    }
    #[fixed_stack_segment]
    fn layoutPhase1(&self, noChanges: *int) -> int {
        unsafe { wxWindow_LayoutPhase1(self, noChanges) }
    }
    #[fixed_stack_segment]
    fn layoutPhase2(&self, noChanges: *int) -> int {
        unsafe { wxWindow_LayoutPhase2(self, noChanges) }
    }
    #[fixed_stack_segment]
    fn lower(&self) {
        unsafe { wxWindow_Lower(self) }
    }
    #[fixed_stack_segment]
    fn makeModal(&self, modal: bool) {
        unsafe { wxWindow_MakeModal(self, modal) }
    }
    #[fixed_stack_segment]
    fn move(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_Move(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn moveConstraint(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_MoveConstraint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn popEventHandler(&self, deleteHandler: bool) {
        unsafe { wxWindow_PopEventHandler(self, deleteHandler) }
    }
    #[fixed_stack_segment]
    fn popupMenu(&self, menu: @wxMenu, x: c_int, y: c_int) -> int {
        unsafe { wxWindow_PopupMenu(self, menu, _arg2) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @wxDC) {
        unsafe { wxWindow_PrepareDC(self, dc) }
    }
    #[fixed_stack_segment]
    fn pushEventHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxWindow_PushEventHandler(self, handler) }
    }
    #[fixed_stack_segment]
    fn raise(&self) {
        unsafe { wxWindow_Raise(self) }
    }
    #[fixed_stack_segment]
    fn refresh(&self, eraseBackground: bool) {
        unsafe { wxWindow_Refresh(self, eraseBackground) }
    }
    #[fixed_stack_segment]
    fn refreshRect(&self, eraseBackground: bool, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_RefreshRect(self, eraseBackground, _arg2) }
    }
    #[fixed_stack_segment]
    fn releaseMouse(&self) {
        unsafe { wxWindow_ReleaseMouse(self) }
    }
    #[fixed_stack_segment]
    fn removeChild(&self, child: @wxWindow) {
        unsafe { wxWindow_RemoveChild(self, child) }
    }
    #[fixed_stack_segment]
    fn removeConstraintReference(&self, otherWin: @wxWindow) {
        unsafe { wxWindow_RemoveConstraintReference(self, otherWin) }
    }
    #[fixed_stack_segment]
    fn reparent(&self, _par: @wxWindow) -> int {
        unsafe { wxWindow_Reparent(self, _par) }
    }
    #[fixed_stack_segment]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self) }
    }
    #[fixed_stack_segment]
    fn screenToClient(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { wxWindow_ScreenToClient(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn setAcceleratorTable(&self, accel: @wxAcceleratorTable) {
        unsafe { wxWindow_SetAcceleratorTable(self, accel) }
    }
    #[fixed_stack_segment]
    fn setAutoLayout(&self, autoLayout: bool) {
        unsafe { wxWindow_SetAutoLayout(self, autoLayout) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @wxColour) -> int {
        unsafe { wxWindow_SetBackgroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setCaret(&self, caret: @wxCaret) {
        unsafe { wxWindow_SetCaret(self, caret) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, data: @wxClientData) {
        unsafe { wxWindow_SetClientData(self, data) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, data: @wxClientData) {
        unsafe { wxWindow_SetClientObject(self, data) }
    }
    #[fixed_stack_segment]
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setConstraintSizes(&self, recurse: int) {
        unsafe { wxWindow_SetConstraintSizes(self, recurse) }
    }
    #[fixed_stack_segment]
    fn setConstraints(&self, constraints: @wxLayoutConstraints) {
        unsafe { wxWindow_SetConstraints(self, constraints) }
    }
    #[fixed_stack_segment]
    fn setCursor(&self, cursor: @wxCursor) -> int {
        unsafe { wxWindow_SetCursor(self, cursor) }
    }
    #[fixed_stack_segment]
    fn setDropTarget(&self, dropTarget: @wxDropTarget) {
        unsafe { wxWindow_SetDropTarget(self, dropTarget) }
    }
    #[fixed_stack_segment]
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self, exStyle) }
    }
    #[fixed_stack_segment]
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont) -> int {
        unsafe { wxWindow_SetFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, colour: @wxColour) -> int {
        unsafe { wxWindow_SetForegroundColour(self, colour) }
    }
    #[fixed_stack_segment]
    fn setId(&self, _id: int) {
        unsafe { wxWindow_SetId(self, _id) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, _title: @wxString) {
        unsafe { wxWindow_SetLabel(self, _title) }
    }
    #[fixed_stack_segment]
    fn setName(&self, _name: @wxString) {
        unsafe { wxWindow_SetName(self, _name) }
    }
    #[fixed_stack_segment]
    fn setScrollPos(&self, orient: int, pos: int, refresh: bool) {
        unsafe { wxWindow_SetScrollPos(self, orient, pos, refresh) }
    }
    #[fixed_stack_segment]
    fn setScrollbar(&self, orient: int, pos: int, thumbVisible: int, range: int, refresh: bool) {
        unsafe { wxWindow_SetScrollbar(self, orient, pos, thumbVisible, range, refresh) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, width: c_int, height: c_int, sizeFlags: int) {
        unsafe { wxWindow_SetSize(self, _arg1, sizeFlags) }
    }
    #[fixed_stack_segment]
    fn setSizeConstraint(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_SetSizeConstraint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn setSizeHints(&self, minW: int, minH: int, maxW: int, maxH: int, incW: int, incH: int) {
        unsafe { wxWindow_SetSizeHints(self, minW, minH, maxW, maxH, incW, incH) }
    }
    #[fixed_stack_segment]
    fn setSizer(&self, sizer: @wxSizer) {
        unsafe { wxWindow_SetSizer(self, sizer) }
    }
    #[fixed_stack_segment]
    fn setToolTip(&self, tip: @wxString) {
        unsafe { wxWindow_SetToolTip(self, tip) }
    }
    #[fixed_stack_segment]
    fn setValidator(&self, validator: @wxValidator) {
        unsafe { wxWindow_SetValidator(self, validator) }
    }
    #[fixed_stack_segment]
    fn setWindowStyleFlag(&self, style: c_long) {
        unsafe { wxWindow_SetWindowStyleFlag(self, style) }
    }
    #[fixed_stack_segment]
    fn show(&self) -> bool {
        unsafe { wxWindow_Show(self) }
    }
    #[fixed_stack_segment]
    fn thaw(&self) {
        unsafe { wxWindow_Thaw(self) }
    }
    #[fixed_stack_segment]
    fn transferDataFromWindow(&self) -> bool {
        unsafe { wxWindow_TransferDataFromWindow(self) }
    }
    #[fixed_stack_segment]
    fn transferDataToWindow(&self) -> bool {
        unsafe { wxWindow_TransferDataToWindow(self) }
    }
    #[fixed_stack_segment]
    fn unsetConstraints(&self, c: *c_void) {
        unsafe { wxWindow_UnsetConstraints(self, c) }
    }
    #[fixed_stack_segment]
    fn updateWindowUI(&self) {
        unsafe { wxWindow_UpdateWindowUI(self) }
    }
    #[fixed_stack_segment]
    fn validate(&self) -> bool {
        unsafe { wxWindow_Validate(self) }
    }
    #[fixed_stack_segment]
    fn setVirtualSize(&self, w: c_int, h: c_int) {
        unsafe { wxWindow_SetVirtualSize(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn warpPointer(&self, x: c_int, y: c_int) {
        unsafe { wxWindow_WarpPointer(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixelsEx(&self) -> @wxPoint {
        unsafe { wxWindow_ConvertDialogToPixelsEx(self) }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialogEx(&self) -> @wxPoint {
        unsafe { wxWindow_ConvertPixelsToDialogEx(self) }
    }
    #[fixed_stack_segment]
    fn screenToClient2(&self, x: c_int, y: c_int) -> @wxPoint {
        unsafe { wxWindow_ScreenToClient2(self, _arg1) }
    }
}
trait wxWindowCreateEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { wxWindowCreateEvent_GetWindow(self) }
    }
}
trait wxWindowDC {
    #[fixed_stack_segment]
    fn new(win: @wxWindow) -> @wxWindowDC {
        unsafe { wxWindowDC_Create(win) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxWindowDC_Delete(self) }
    }
}
trait wxWindowDestroyEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @wxWindow {
        unsafe { wxWindowDestroyEvent_GetWindow(self) }
    }
}
trait wxWindowDisabler {
}
trait wxWizard {
    #[fixed_stack_segment]
    fn chain(f: @wxWizardPageSimple, s: @wxWizardPageSimple) {
        unsafe { wxWizard_Chain(f, s) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _bmp: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> @wxWizard {
        unsafe { wxWizard_Create(_prt, _id, _txt, _bmp, _arg4) }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> @wxWizardPage {
        unsafe { wxWizard_GetCurrentPage(self) }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> @wxSize {
        unsafe { wxWizard_GetPageSize(self) }
    }
    #[fixed_stack_segment]
    fn runWizard(&self, firstPage: @wxWizardPage) -> int {
        unsafe { wxWizard_RunWizard(self, firstPage) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self, _arg1) }
    }
}
trait wxWizardEvent {
    #[fixed_stack_segment]
    fn getDirection(&self) -> int {
        unsafe { wxWizardEvent_GetDirection(self) }
    }
}
trait wxWizardPage {
}
trait wxWizardPageSimple {
    #[fixed_stack_segment]
    fn new(_prt: @wxWizard) -> @wxWizardPageSimple {
        unsafe { wxWizardPageSimple_Create(_prt) }
    }
    #[fixed_stack_segment]
    fn getBitmap(&self, @wxBitmap) {
        unsafe { wxWizardPageSimple_GetBitmap(self, _ref) }
    }
    #[fixed_stack_segment]
    fn getNext(&self) -> @wxWizardPageSimple {
        unsafe { wxWizardPageSimple_GetNext(self) }
    }
    #[fixed_stack_segment]
    fn getPrev(&self) -> @wxWizardPageSimple {
        unsafe { wxWizardPageSimple_GetPrev(self) }
    }
    #[fixed_stack_segment]
    fn setNext(&self, next: @wxWizardPageSimple) {
        unsafe { wxWizardPageSimple_SetNext(self, next) }
    }
    #[fixed_stack_segment]
    fn setPrev(&self, prev: @wxWizardPageSimple) {
        unsafe { wxWizardPageSimple_SetPrev(self, prev) }
    }
}
trait wxXmlResource {
    #[fixed_stack_segment]
    fn addHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxXmlResource_AddHandler(self, handler) }
    }
    #[fixed_stack_segment]
    fn addSubclassFactory(&self, factory: *c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self, factory) }
    }
    #[fixed_stack_segment]
    fn attachUnknownControl(&self, control: @wxControl, parent: @wxWindow) -> int {
        unsafe { wxXmlResource_AttachUnknownControl(self, control, parent) }
    }
    #[fixed_stack_segment]
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self) }
    }
    #[fixed_stack_segment]
    fn compareVersion(&self, major: int, minor: int, release: int, revision: int) -> int {
        unsafe { wxXmlResource_CompareVersion(self, major, minor, release, revision) }
    }
    #[fixed_stack_segment]
    fn new(flags: int) -> @wxXmlResource {
        unsafe { wxXmlResource_Create(flags) }
    }
    #[fixed_stack_segment]
    fn newFromFile(filemask: @wxString, flags: int) -> @wxXmlResource {
        unsafe { wxXmlResource_CreateFromFile(filemask, flags) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxXmlResource_Delete(self) }
    }
    #[fixed_stack_segment]
    fn get() -> @wxXmlResource {
        unsafe { wxXmlResource_Get() }
    }
    #[fixed_stack_segment]
    fn getDomain(&self) -> @wxString {
        unsafe { wxXmlResource_GetDomain(self) }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> int {
        unsafe { wxXmlResource_GetFlags(self) }
    }
    #[fixed_stack_segment]
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self) }
    }
    #[fixed_stack_segment]
    fn getXRCID(&self, str_id: @wxString) -> int {
        unsafe { wxXmlResource_GetXRCID(self, str_id) }
    }
    #[fixed_stack_segment]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self) }
    }
    #[fixed_stack_segment]
    fn insertHandler(&self, handler: @wxEvtHandler) {
        unsafe { wxXmlResource_InsertHandler(self, handler) }
    }
    #[fixed_stack_segment]
    fn load(&self, filemask: @wxString) -> bool {
        unsafe { wxXmlResource_Load(self, filemask) }
    }
    #[fixed_stack_segment]
    fn loadBitmap(&self, name: @wxString, @wxBitmap) {
        unsafe { wxXmlResource_LoadBitmap(self, name, _ref) }
    }
    #[fixed_stack_segment]
    fn loadDialog(&self, parent: @wxWindow, name: @wxString) -> @wxDialog {
        unsafe { wxXmlResource_LoadDialog(self, parent, name) }
    }
    #[fixed_stack_segment]
    fn loadFrame(&self, parent: @wxWindow, name: @wxString) -> @wxFrame {
        unsafe { wxXmlResource_LoadFrame(self, parent, name) }
    }
    #[fixed_stack_segment]
    fn loadIcon(&self, name: @wxString, @wxIcon) {
        unsafe { wxXmlResource_LoadIcon(self, name, _ref) }
    }
    #[fixed_stack_segment]
    fn loadMenu(&self, name: @wxString) -> @wxMenu {
        unsafe { wxXmlResource_LoadMenu(self, name) }
    }
    #[fixed_stack_segment]
    fn loadMenuBar(&self, parent: @wxWindow, name: @wxString) -> @wxMenuBar {
        unsafe { wxXmlResource_LoadMenuBar(self, parent, name) }
    }
    #[fixed_stack_segment]
    fn loadPanel(&self, parent: @wxWindow, name: @wxString) -> @wxPanel {
        unsafe { wxXmlResource_LoadPanel(self, parent, name) }
    }
    #[fixed_stack_segment]
    fn loadToolBar(&self, parent: @wxWindow, name: @wxString) -> @wxToolBar {
        unsafe { wxXmlResource_LoadToolBar(self, parent, name) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self, str_id: @wxString) -> @wxSizer {
        unsafe { wxXmlResource_GetSizer(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getBoxSizer(&self, str_id: @wxString) -> @wxBoxSizer {
        unsafe { wxXmlResource_GetBoxSizer(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getStaticBoxSizer(&self, str_id: @wxString) -> @wxStaticBoxSizer {
        unsafe { wxXmlResource_GetStaticBoxSizer(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getGridSizer(&self, str_id: @wxString) -> @wxGridSizer {
        unsafe { wxXmlResource_GetGridSizer(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getFlexGridSizer(&self, str_id: @wxString) -> @wxFlexGridSizer {
        unsafe { wxXmlResource_GetFlexGridSizer(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getBitmapButton(&self, str_id: @wxString) -> @wxBitmapButton {
        unsafe { wxXmlResource_GetBitmapButton(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getButton(&self, str_id: @wxString) -> @wxButton {
        unsafe { wxXmlResource_GetButton(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getCalendarCtrl(&self, str_id: @wxString) -> @wxCalendarCtrl {
        unsafe { wxXmlResource_GetCalendarCtrl(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getCheckBox(&self, str_id: @wxString) -> @wxCheckBox {
        unsafe { wxXmlResource_GetCheckBox(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getCheckListBox(&self, str_id: @wxString) -> @wxCheckListBox {
        unsafe { wxXmlResource_GetCheckListBox(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getChoice(&self, str_id: @wxString) -> @wxChoice {
        unsafe { wxXmlResource_GetChoice(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getComboBox(&self, str_id: @wxString) -> @wxComboBox {
        unsafe { wxXmlResource_GetComboBox(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getGauge(&self, str_id: @wxString) -> @wxGauge {
        unsafe { wxXmlResource_GetGauge(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getGrid(&self, str_id: @wxString) -> @wxGrid {
        unsafe { wxXmlResource_GetGrid(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getHtmlWindow(&self, str_id: @wxString) -> @wxHtmlWindow {
        unsafe { wxXmlResource_GetHtmlWindow(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getListBox(&self, str_id: @wxString) -> @wxListBox {
        unsafe { wxXmlResource_GetListBox(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getListCtrl(&self, str_id: @wxString) -> @wxListCtrl {
        unsafe { wxXmlResource_GetListCtrl(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getMDIChildFrame(&self, str_id: @wxString) -> @wxMDIChildFrame {
        unsafe { wxXmlResource_GetMDIChildFrame(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getMDIParentFrame(&self, str_id: @wxString) -> @wxMDIParentFrame {
        unsafe { wxXmlResource_GetMDIParentFrame(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, str_id: @wxString) -> @wxMenu {
        unsafe { wxXmlResource_GetMenu(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self, str_id: @wxString) -> @wxMenuBar {
        unsafe { wxXmlResource_GetMenuBar(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getMenuItem(&self, str_id: @wxString) -> @wxMenuItem {
        unsafe { wxXmlResource_GetMenuItem(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getNotebook(&self, str_id: @wxString) -> @wxNotebook {
        unsafe { wxXmlResource_GetNotebook(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getPanel(&self, str_id: @wxString) -> @wxPanel {
        unsafe { wxXmlResource_GetPanel(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getRadioButton(&self, str_id: @wxString) -> @wxRadioButton {
        unsafe { wxXmlResource_GetRadioButton(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getRadioBox(&self, str_id: @wxString) -> @wxRadioBox {
        unsafe { wxXmlResource_GetRadioBox(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getScrollBar(&self, str_id: @wxString) -> @wxScrollBar {
        unsafe { wxXmlResource_GetScrollBar(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow(&self, str_id: @wxString) -> @wxScrolledWindow {
        unsafe { wxXmlResource_GetScrolledWindow(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getSlider(&self, str_id: @wxString) -> @wxSlider {
        unsafe { wxXmlResource_GetSlider(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getSpinButton(&self, str_id: @wxString) -> @wxSpinButton {
        unsafe { wxXmlResource_GetSpinButton(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getSpinCtrl(&self, str_id: @wxString) -> @wxSpinCtrl {
        unsafe { wxXmlResource_GetSpinCtrl(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getSplitterWindow(&self, str_id: @wxString) -> @wxSplitterWindow {
        unsafe { wxXmlResource_GetSplitterWindow(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getStaticBitmap(&self, str_id: @wxString) -> @wxStaticBitmap {
        unsafe { wxXmlResource_GetStaticBitmap(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self, str_id: @wxString) -> @wxStaticBox {
        unsafe { wxXmlResource_GetStaticBox(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getStaticLine(&self, str_id: @wxString) -> @wxStaticLine {
        unsafe { wxXmlResource_GetStaticLine(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getStaticText(&self, str_id: @wxString) -> @wxStaticText {
        unsafe { wxXmlResource_GetStaticText(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getTextCtrl(&self, str_id: @wxString) -> @wxTextCtrl {
        unsafe { wxXmlResource_GetTextCtrl(self, str_id) }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl(&self, str_id: @wxString) -> @wxTreeCtrl {
        unsafe { wxXmlResource_GetTreeCtrl(self, str_id) }
    }
    #[fixed_stack_segment]
    fn unload(&self, filemask: @wxString) -> bool {
        unsafe { wxXmlResource_Unload(self, filemask) }
    }
    #[fixed_stack_segment]
    fn set(&self, res: @wxXmlResource) -> @wxXmlResource {
        unsafe { wxXmlResource_Set(self, res) }
    }
    #[fixed_stack_segment]
    fn setDomain(&self, domain: @wxString) {
        unsafe { wxXmlResource_SetDomain(self, domain) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: int) {
        unsafe { wxXmlResource_SetFlags(self, flags) }
    }
    #[fixed_stack_segment]
    fn getStyledTextCtrl(&self, str_id: @wxString) -> @wxStyledTextCtrl {
        unsafe { wxXmlResource_GetStyledTextCtrl(self, str_id) }
    }
}
trait wxXmlResourceHandler {
}
trait wxZipInputStream {
}
trait wxZlibInputStream {
}
trait wxZlibOutputStream {
}
trait wxPropertyGrid {
    #[fixed_stack_segment]
    fn append(&self, prop: @wxPGProperty) -> @wxPGProperty {
        unsafe { wxPropertyGrid_Append(self, prop) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxPropertyGrid {
        unsafe { wxPropertyGrid_Create(_prt, _id, _arg2, _stl) }
    }
    #[fixed_stack_segment]
    fn disableProperty(&self, propName: @wxString) -> bool {
        unsafe { wxPropertyGrid_DisableProperty(self, propName) }
    }
}
trait wxPropertyGridEvent {
    #[fixed_stack_segment]
    fn hasProperty(&self) -> bool {
        unsafe { wxPropertyGridEvent_HasProperty(self) }
    }
    #[fixed_stack_segment]
    fn getProperty(&self) -> @wxPGProperty {
        unsafe { wxPropertyGridEvent_GetProperty(self) }
    }
}
trait wxPGProperty {
    #[fixed_stack_segment]
    fn getLabel(&self) -> @wxString {
        unsafe { wxPGProperty_GetLabel(self) }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @wxString {
        unsafe { wxPGProperty_GetName(self) }
    }
    #[fixed_stack_segment]
    fn getValueAsString(&self) -> @wxString {
        unsafe { wxPGProperty_GetValueAsString(self) }
    }
    #[fixed_stack_segment]
    fn getValueType(&self) -> @wxString {
        unsafe { wxPGProperty_GetValueType(self) }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, helpString: @wxString) {
        unsafe { wxPGProperty_SetHelpString(self, helpString) }
    }
}
trait wxStringProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: @wxString) -> @wxStringProperty {
        unsafe { wxStringProperty_Create(label, name, value) }
    }
}
trait wxIntProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: int) -> @wxIntProperty {
        unsafe { wxIntProperty_Create(label, name, value) }
    }
}
trait wxBoolProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: bool) -> @wxBoolProperty {
        unsafe { wxBoolProperty_Create(label, name, value) }
    }
}
trait wxFloatProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: c_float) -> @wxFloatProperty {
        unsafe { wxFloatProperty_Create(label, name, value) }
    }
}
trait wxDateProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: @wxDateTime) -> @wxDateProperty {
        unsafe { wxDateProperty_Create(label, name, value) }
    }
}
trait wxFileProperty {
    #[fixed_stack_segment]
    fn new(label: @wxString, name: @wxString, value: @wxString) -> @wxFileProperty {
        unsafe { wxFileProperty_Create(label, name, value) }
    }
}
trait wxPropertyCategory {
    #[fixed_stack_segment]
    fn new(label: @wxString) -> @wxPropertyCategory {
        unsafe { wxPropertyCategory_Create(label) }
    }
}
trait wxGenericDragImage {
    #[fixed_stack_segment]
    fn new(cursor: @wxCursor) -> @wxGenericDragImage {
        unsafe { wxGenericDragImage_Create(cursor) }
    }
    #[fixed_stack_segment]
    fn doDrawImage(&self, dc: @wxDC, x: int, y: int) -> bool {
        unsafe { wxGenericDragImage_DoDrawImage(self, dc, x, y) }
    }
    #[fixed_stack_segment]
    fn getImageRect(&self, x_pos: int, y_pos: int) -> @wxRect {
        unsafe { wxGenericDragImage_GetImageRect(self, x_pos, y_pos) }
    }
    #[fixed_stack_segment]
    fn updateBackingFromWindow(&self, windowDC: @wxDC, destDC: @wxMemoryDC, x: int, y: int, w: int, h: int, xdest: int, ydest: int, width: int, height: int) -> bool {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self, windowDC, destDC, x, y, w, h, xdest, ydest, width, height) }
    }
}
trait wxGraphicsObject {
    #[fixed_stack_segment]
    fn getRenderer() -> @wxGraphicsRenderer {
        unsafe { wxGraphicsObject_GetRenderer() }
    }
    #[fixed_stack_segment]
    fn isNull(&self) -> bool {
        unsafe { wxGraphicsObject_IsNull(self) }
    }
}
trait wxGraphicsBrush {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsBrush {
        unsafe { wxGraphicsBrush_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsBrush_Delete(self) }
    }
}
trait wxGraphicsContext {
    #[fixed_stack_segment]
    fn new(dc: @wxWindowDC) -> @wxGraphicsContext {
        unsafe { wxGraphicsContext_Create(dc) }
    }
    #[fixed_stack_segment]
    fn newFromWindow(window: @wxWindow) -> @wxGraphicsContext {
        unsafe { wxGraphicsContext_CreateFromWindow(window) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsContext_Delete(self) }
    }
    #[fixed_stack_segment]
    fn newFromNative(context: *c_void) -> @wxGraphicsContext {
        unsafe { wxGraphicsContext_CreateFromNative(context) }
    }
    #[fixed_stack_segment]
    fn newFromNativeWindow(window: *c_void) -> @wxGraphicsContext {
        unsafe { wxGraphicsContext_CreateFromNativeWindow(window) }
    }
    #[fixed_stack_segment]
    fn clip(&self, region: @wxRegion) {
        unsafe { wxGraphicsContext_Clip(self, region) }
    }
    #[fixed_stack_segment]
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self) }
    }
    #[fixed_stack_segment]
    fn drawBitmap(&self, bmp: @wxBitmap, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self, bmp, _arg2) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn drawIcon(&self, icon: @wxIcon, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self, icon, _arg2) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: size_t, x: *c_void, y: *c_void, style: int) {
        unsafe { wxGraphicsContext_DrawLines(self, n, x, y, style) }
    }
    #[fixed_stack_segment]
    fn drawPath(&self, path: @wxGraphicsPath, style: int) {
        unsafe { wxGraphicsContext_DrawPath(self, path, style) }
    }
    #[fixed_stack_segment]
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawRectangle(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawRoundedRectangle(self, _arg1, radius) }
    }
    #[fixed_stack_segment]
    fn drawText(&self, text: @wxString, x: c_double, y: c_double) {
        unsafe { wxGraphicsContext_DrawText(self, text, _arg2) }
    }
    #[fixed_stack_segment]
    fn drawTextWithAngle(&self, text: @wxString, x: c_double, y: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawTextWithAngle(self, text, _arg2, radius) }
    }
    #[fixed_stack_segment]
    fn fillPath(&self, path: @wxGraphicsPath, style: int) {
        unsafe { wxGraphicsContext_FillPath(self, path, style) }
    }
    #[fixed_stack_segment]
    fn strokePath(&self, path: @wxGraphicsPath) {
        unsafe { wxGraphicsContext_StrokePath(self, path) }
    }
    #[fixed_stack_segment]
    fn getNativeContext(&self) {
        unsafe { wxGraphicsContext_GetNativeContext(self) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, text: @wxString, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double) {
        unsafe { wxGraphicsContext_GetTextExtent(self, text, width, height, descent, externalLeading) }
    }
    #[fixed_stack_segment]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsContext_Rotate(self, angle) }
    }
    #[fixed_stack_segment]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsContext_Scale(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsContext_Translate(self, dx, dy) }
    }
    #[fixed_stack_segment]
    fn setTransform(&self, path: @wxGraphicsMatrix) {
        unsafe { wxGraphicsContext_SetTransform(self, path) }
    }
    #[fixed_stack_segment]
    fn concatTransform(&self, path: @wxGraphicsMatrix) {
        unsafe { wxGraphicsContext_ConcatTransform(self, path) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @wxBrush) {
        unsafe { wxGraphicsContext_SetBrush(self, brush) }
    }
    #[fixed_stack_segment]
    fn setGraphicsBrush(&self, brush: @wxGraphicsBrush) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self, brush) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @wxFont, colour: @wxColour) {
        unsafe { wxGraphicsContext_SetFont(self, font, colour) }
    }
    #[fixed_stack_segment]
    fn setGraphicsFont(&self, font: @wxGraphicsFont) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self, font) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @wxPen) {
        unsafe { wxGraphicsContext_SetPen(self, pen) }
    }
    #[fixed_stack_segment]
    fn setGraphicsPen(&self, pen: @wxGraphicsPen) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self, pen) }
    }
    #[fixed_stack_segment]
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn strokeLines(&self, n: size_t, x: *c_void, y: *c_void, style: int) {
        unsafe { wxGraphicsContext_StrokeLines(self, n, x, y, style) }
    }
}
trait wxGraphicsFont {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsFont {
        unsafe { wxGraphicsFont_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsFont_Delete(self) }
    }
}
trait wxGraphicsMatrix {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsMatrix {
        unsafe { wxGraphicsMatrix_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsMatrix_Delete(self) }
    }
    #[fixed_stack_segment]
    fn concat(&self, t: @wxGraphicsMatrix) {
        unsafe { wxGraphicsMatrix_Concat(self, t) }
    }
    #[fixed_stack_segment]
    fn get(&self, a: *c_double, b: *c_double, c: *c_double, d: *c_double, tx: *c_double, ty: *c_double) {
        unsafe { wxGraphicsMatrix_Get(self, a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    fn getNativeMatrix(&self) {
        unsafe { wxGraphicsMatrix_GetNativeMatrix(self) }
    }
    #[fixed_stack_segment]
    fn invert(&self) {
        unsafe { wxGraphicsMatrix_Invert(self) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, t: @wxGraphicsMatrix) -> bool {
        unsafe { wxGraphicsMatrix_IsEqual(self, t) }
    }
    #[fixed_stack_segment]
    fn isIdentity(&self) -> bool {
        unsafe { wxGraphicsMatrix_IsIdentity(self) }
    }
    #[fixed_stack_segment]
    fn rotate(&self, angle: c_double) {
        unsafe { wxGraphicsMatrix_Rotate(self, angle) }
    }
    #[fixed_stack_segment]
    fn scale(&self, xScale: c_double, yScale: c_double) {
        unsafe { wxGraphicsMatrix_Scale(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn set(&self, a: c_double, b: c_double, c: c_double, d: c_double, tx: c_double, ty: c_double) {
        unsafe { wxGraphicsMatrix_Set(self, a, b, c, d, tx, ty) }
    }
    #[fixed_stack_segment]
    fn translate(&self, dx: c_double, dy: c_double) {
        unsafe { wxGraphicsMatrix_Translate(self, dx, dy) }
    }
    #[fixed_stack_segment]
    fn transformPoint(&self, x: *c_double, y: *c_double) {
        unsafe { wxGraphicsMatrix_TransformPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn transformDistance(&self, dx: *c_double, dy: *c_double) {
        unsafe { wxGraphicsMatrix_TransformDistance(self, dx, dy) }
    }
}
trait wxGraphicsPath {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsPath {
        unsafe { wxGraphicsPath_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsPath_Delete(self) }
    }
    #[fixed_stack_segment]
    fn moveToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_MoveToPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: bool) {
        unsafe { wxGraphicsPath_AddArc(self, _arg1, r, startAngle, endAngle, clockwise) }
    }
    #[fixed_stack_segment]
    fn addArcToPoint(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddArcToPoint(self, _arg1, _arg2, r) }
    }
    #[fixed_stack_segment]
    fn addCircle(&self, x: c_double, y: c_double, r: c_double) {
        unsafe { wxGraphicsPath_AddCircle(self, _arg1, r) }
    }
    #[fixed_stack_segment]
    fn addCurveToPoint(&self, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddCurveToPoint(self, _arg1, _arg2, _arg3) }
    }
    #[fixed_stack_segment]
    fn addEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddEllipse(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn addLineToPoint(&self, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddLineToPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn addPath(&self, x: c_double, y: c_double, path: @wxGraphicsPath) {
        unsafe { wxGraphicsPath_AddPath(self, _arg1, path) }
    }
    #[fixed_stack_segment]
    fn addQuadCurveToPoint(&self, cx: c_double, cy: c_double, x: c_double, y: c_double) {
        unsafe { wxGraphicsPath_AddQuadCurveToPoint(self, _arg1, _arg2) }
    }
    #[fixed_stack_segment]
    fn addRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsPath_AddRectangle(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn addRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double) {
        unsafe { wxGraphicsPath_AddRoundedRectangle(self, _arg1, radius) }
    }
    #[fixed_stack_segment]
    fn closeSubpath(&self) {
        unsafe { wxGraphicsPath_CloseSubpath(self) }
    }
    #[fixed_stack_segment]
    fn contains(&self, x: c_double, y: c_double, style: int) {
        unsafe { wxGraphicsPath_Contains(self, _arg1, style) }
    }
    #[fixed_stack_segment]
    fn getBox(&self, x: *c_double, y: *c_double, w: *c_double, h: *c_double) {
        unsafe { wxGraphicsPath_GetBox(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getCurrentPoint(&self, x: *c_double, y: *c_double) {
        unsafe { wxGraphicsPath_GetCurrentPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn transform(&self, matrix: @wxGraphicsMatrix) {
        unsafe { wxGraphicsPath_Transform(self, matrix) }
    }
    #[fixed_stack_segment]
    fn getNativePath(&self) {
        unsafe { wxGraphicsPath_GetNativePath(self) }
    }
    #[fixed_stack_segment]
    fn unGetNativePath(p: *c_void) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}
trait wxGraphicsPen {
    #[fixed_stack_segment]
    fn new() -> @wxGraphicsPen {
        unsafe { wxGraphicsPen_Create() }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsPen_Delete(self) }
    }
}
trait wxGraphicsRenderer {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsRenderer_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @wxGraphicsRenderer {
        unsafe { wxGraphicsRenderer_GetDefaultRenderer(self) }
    }
    #[fixed_stack_segment]
    fn newContext(dc: @wxWindowDC) -> @wxGraphicsContext {
        unsafe { wxGraphicsRenderer_CreateContext(dc) }
    }
    #[fixed_stack_segment]
    fn newContextFromWindow(window: @wxWindow) -> @wxGraphicsContext {
        unsafe { wxGraphicsRenderer_CreateContextFromWindow(window) }
    }
    #[fixed_stack_segment]
    fn newContextFromNativeContext(context: *c_void) -> @wxGraphicsContext {
        unsafe { wxGraphicsRenderer_CreateContextFromNativeContext(context) }
    }
    #[fixed_stack_segment]
    fn newContextFromNativeWindow(window: *c_void) -> @wxGraphicsContext {
        unsafe { wxGraphicsRenderer_CreateContextFromNativeWindow(window) }
    }
}
trait wxGLContext {
    #[fixed_stack_segment]
    fn new(win: @wxGLCanvas, other: @wxGLContext) -> @wxGLContext {
        unsafe { wxGLContext_Create(win, other) }
    }
    #[fixed_stack_segment]
    fn newFromNull(win: @wxGLCanvas) -> @wxGLContext {
        unsafe { wxGLContext_CreateFromNull(win) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, win: @wxGLCanvas) -> bool {
        unsafe { wxGLContext_SetCurrent(self, win) }
    }
}
trait wxManagedPtr {
    #[fixed_stack_segment]
    fn getPtr(&self) {
        unsafe { wxManagedPtr_GetPtr(self) }
    }
    #[fixed_stack_segment]
    fn noFinalize(&self) {
        unsafe { wxManagedPtr_NoFinalize(self) }
    }
    #[fixed_stack_segment]
    fn finalize(&self) {
        unsafe { wxManagedPtr_Finalize(self) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxManagedPtr_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getDeleteFunction() {
        unsafe { wxManagedPtr_GetDeleteFunction() }
    }
    #[fixed_stack_segment]
    fn newFromObject(obj: @wxObject) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromObject(obj) }
    }
    #[fixed_stack_segment]
    fn newFromDateTime(obj: @wxDateTime) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromDateTime(obj) }
    }
    #[fixed_stack_segment]
    fn newFromGridCellCoordsArray(obj: @wxGridCellCoordsArray) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromGridCellCoordsArray(obj) }
    }
    #[fixed_stack_segment]
    fn newFromBitmap(obj: @wxBitmap) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromBitmap(obj) }
    }
    #[fixed_stack_segment]
    fn newFromIcon(obj: @wxIcon) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromIcon(obj) }
    }
    #[fixed_stack_segment]
    fn newFromBrush(obj: @wxBrush) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromBrush(obj) }
    }
    #[fixed_stack_segment]
    fn newFromColour(obj: @wxColour) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromColour(obj) }
    }
    #[fixed_stack_segment]
    fn newFromCursor(obj: @wxCursor) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromCursor(obj) }
    }
    #[fixed_stack_segment]
    fn newFromFont(obj: @wxFont) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromFont(obj) }
    }
    #[fixed_stack_segment]
    fn newFromPen(obj: @wxPen) -> @wxManagedPtr {
        unsafe { wxManagedPtr_CreateFromPen(obj) }
    }
}
trait wxMediaCtrl {
    #[fixed_stack_segment]
    fn new(parent: @wxWindow, windowID: int, fileName: @wxString, x: int, y: int, w: int, h: int, style: c_long, szBackend: @wxString, name: @wxString) -> @wxMediaCtrl {
        unsafe { wxMediaCtrl_Create(parent, windowID, fileName, x, y, w, h, style, szBackend, name) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMediaCtrl_Delete(self) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @wxSize {
        unsafe { wxMediaCtrl_GetBestSize(self) }
    }
    #[fixed_stack_segment]
    fn getPlaybackRate(&self) -> c_double {
        unsafe { wxMediaCtrl_GetPlaybackRate(self) }
    }
    #[fixed_stack_segment]
    fn getVolume(&self) -> c_double {
        unsafe { wxMediaCtrl_GetVolume(self) }
    }
    #[fixed_stack_segment]
    fn getState(&self) -> int {
        unsafe { wxMediaCtrl_GetState(self) }
    }
    #[fixed_stack_segment]
    fn length(&self) -> i64 {
        unsafe { wxMediaCtrl_Length(self) }
    }
    #[fixed_stack_segment]
    fn load(&self, fileName: @wxString) -> bool {
        unsafe { wxMediaCtrl_Load(self, fileName) }
    }
    #[fixed_stack_segment]
    fn loadURI(&self, uri: @wxString) -> bool {
        unsafe { wxMediaCtrl_LoadURI(self, uri) }
    }
    #[fixed_stack_segment]
    fn loadURIWithProxy(&self, uri: @wxString, proxy: @wxString) -> bool {
        unsafe { wxMediaCtrl_LoadURIWithProxy(self, uri, proxy) }
    }
    #[fixed_stack_segment]
    fn pause(&self) -> bool {
        unsafe { wxMediaCtrl_Pause(self) }
    }
    #[fixed_stack_segment]
    fn play(&self) -> bool {
        unsafe { wxMediaCtrl_Play(self) }
    }
    #[fixed_stack_segment]
    fn seek(&self, offsetWhere: i64, mode: int) -> i64 {
        unsafe { wxMediaCtrl_Seek(self, offsetWhere, mode) }
    }
    #[fixed_stack_segment]
    fn setPlaybackRate(&self, dRate: c_double) -> bool {
        unsafe { wxMediaCtrl_SetPlaybackRate(self, dRate) }
    }
    #[fixed_stack_segment]
    fn setVolume(&self, dVolume: c_double) -> bool {
        unsafe { wxMediaCtrl_SetVolume(self, dVolume) }
    }
    #[fixed_stack_segment]
    fn showPlayerControls(&self, flags: int) -> bool {
        unsafe { wxMediaCtrl_ShowPlayerControls(self, flags) }
    }
    #[fixed_stack_segment]
    fn stop(&self) -> bool {
        unsafe { wxMediaCtrl_Stop(self) }
    }
    #[fixed_stack_segment]
    fn tell(&self) -> i64 {
        unsafe { wxMediaCtrl_Tell(self) }
    }
}
trait wxMediaEvent {
}
trait wxcPrintout {
    #[fixed_stack_segment]
    fn new(title: @wxString) -> @wxcPrintout {
        unsafe { wxcPrintout_Create(title) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxcPrintout_Delete(self) }
    }
    #[fixed_stack_segment]
    fn setPageLimits(&self, startPage: int, endPage: int, fromPage: int, toPage: int) {
        unsafe { wxcPrintout_SetPageLimits(self, startPage, endPage, fromPage, toPage) }
    }
    #[fixed_stack_segment]
    fn getEvtHandler(&self) -> @wxcPrintoutHandler {
        unsafe { wxcPrintout_GetEvtHandler(self) }
    }
}
trait wxcPrintEvent {
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @wxcPrintout {
        unsafe { wxcPrintEvent_GetPrintout(self) }
    }
    #[fixed_stack_segment]
    fn getPage(&self) -> int {
        unsafe { wxcPrintEvent_GetPage(self) }
    }
    #[fixed_stack_segment]
    fn getEndPage(&self) -> int {
        unsafe { wxcPrintEvent_GetEndPage(self) }
    }
    #[fixed_stack_segment]
    fn getContinue(&self) -> bool {
        unsafe { wxcPrintEvent_GetContinue(self) }
    }
    #[fixed_stack_segment]
    fn setContinue(&self, cont: bool) {
        unsafe { wxcPrintEvent_SetContinue(self, cont) }
    }
    #[fixed_stack_segment]
    fn setPageLimits(&self, startPage: int, endPage: int, fromPage: int, toPage: int) {
        unsafe { wxcPrintEvent_SetPageLimits(self, startPage, endPage, fromPage, toPage) }
    }
}
trait wxcPrintoutHandler {
}
trait wxStyledTextCtrl {
    #[fixed_stack_segment]
    fn addText(&self, text: @wxString) {
        unsafe { wxStyledTextCtrl_AddText(self, text) }
    }
    #[fixed_stack_segment]
    fn addStyledText(&self, data: @wxMemoryBuffer) {
        unsafe { wxStyledTextCtrl_AddStyledText(self, data) }
    }
    #[fixed_stack_segment]
    fn insertText(&self, pos: int, text: @wxString) {
        unsafe { wxStyledTextCtrl_InsertText(self, pos, text) }
    }
    #[fixed_stack_segment]
    fn clearAll(&self) {
        unsafe { wxStyledTextCtrl_ClearAll(self) }
    }
    #[fixed_stack_segment]
    fn clearDocumentStyle(&self) {
        unsafe { wxStyledTextCtrl_ClearDocumentStyle(self) }
    }
    #[fixed_stack_segment]
    fn getLength(&self) -> int {
        unsafe { wxStyledTextCtrl_GetLength(self) }
    }
    #[fixed_stack_segment]
    fn getCharAt(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_GetCharAt(self, pos) }
    }
    #[fixed_stack_segment]
    fn getCurrentPos(&self) -> int {
        unsafe { wxStyledTextCtrl_GetCurrentPos(self) }
    }
    #[fixed_stack_segment]
    fn getAnchor(&self) -> int {
        unsafe { wxStyledTextCtrl_GetAnchor(self) }
    }
    #[fixed_stack_segment]
    fn getStyleAt(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_GetStyleAt(self, pos) }
    }
    #[fixed_stack_segment]
    fn redo(&self) {
        unsafe { wxStyledTextCtrl_Redo(self) }
    }
    #[fixed_stack_segment]
    fn setUndoCollection(&self, collectUndo: bool) {
        unsafe { wxStyledTextCtrl_SetUndoCollection(self, collectUndo) }
    }
    #[fixed_stack_segment]
    fn selectAll(&self) {
        unsafe { wxStyledTextCtrl_SelectAll(self) }
    }
    #[fixed_stack_segment]
    fn setSavePoint(&self) {
        unsafe { wxStyledTextCtrl_SetSavePoint(self) }
    }
    #[fixed_stack_segment]
    fn canRedo(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanRedo(self) }
    }
    #[fixed_stack_segment]
    fn markerLineFromHandle(&self, handle: int) -> int {
        unsafe { wxStyledTextCtrl_MarkerLineFromHandle(self, handle) }
    }
    #[fixed_stack_segment]
    fn markerDeleteHandle(&self, handle: int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteHandle(self, handle) }
    }
    #[fixed_stack_segment]
    fn getUndoCollection(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUndoCollection(self) }
    }
    #[fixed_stack_segment]
    fn getViewWhiteSpace(&self) -> int {
        unsafe { wxStyledTextCtrl_GetViewWhiteSpace(self) }
    }
    #[fixed_stack_segment]
    fn setViewWhiteSpace(&self, viewWS: int) {
        unsafe { wxStyledTextCtrl_SetViewWhiteSpace(self, viewWS) }
    }
    #[fixed_stack_segment]
    fn positionFromPoint(&self, pt_x: c_int, pt_y: c_int) -> int {
        unsafe { wxStyledTextCtrl_PositionFromPoint(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn positionFromPointClose(&self, x: int, y: int) -> int {
        unsafe { wxStyledTextCtrl_PositionFromPointClose(self, x, y) }
    }
    #[fixed_stack_segment]
    fn gotoLine(&self, line: int) {
        unsafe { wxStyledTextCtrl_GotoLine(self, line) }
    }
    #[fixed_stack_segment]
    fn gotoPos(&self, pos: int) {
        unsafe { wxStyledTextCtrl_GotoPos(self, pos) }
    }
    #[fixed_stack_segment]
    fn setAnchor(&self, posAnchor: int) {
        unsafe { wxStyledTextCtrl_SetAnchor(self, posAnchor) }
    }
    #[fixed_stack_segment]
    fn getEndStyled(&self) -> int {
        unsafe { wxStyledTextCtrl_GetEndStyled(self) }
    }
    #[fixed_stack_segment]
    fn convertEOLs(&self, eolMode: int) {
        unsafe { wxStyledTextCtrl_ConvertEOLs(self, eolMode) }
    }
    #[fixed_stack_segment]
    fn getEOLMode(&self) -> int {
        unsafe { wxStyledTextCtrl_GetEOLMode(self) }
    }
    #[fixed_stack_segment]
    fn setEOLMode(&self, eolMode: int) {
        unsafe { wxStyledTextCtrl_SetEOLMode(self, eolMode) }
    }
    #[fixed_stack_segment]
    fn startStyling(&self, pos: int, mask: int) {
        unsafe { wxStyledTextCtrl_StartStyling(self, pos, mask) }
    }
    #[fixed_stack_segment]
    fn setStyling(&self, length: int, style: int) {
        unsafe { wxStyledTextCtrl_SetStyling(self, length, style) }
    }
    #[fixed_stack_segment]
    fn getBufferedDraw(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetBufferedDraw(self) }
    }
    #[fixed_stack_segment]
    fn setBufferedDraw(&self, buffered: bool) {
        unsafe { wxStyledTextCtrl_SetBufferedDraw(self, buffered) }
    }
    #[fixed_stack_segment]
    fn setTabWidth(&self, tabWidth: int) {
        unsafe { wxStyledTextCtrl_SetTabWidth(self, tabWidth) }
    }
    #[fixed_stack_segment]
    fn getTabWidth(&self) -> int {
        unsafe { wxStyledTextCtrl_GetTabWidth(self) }
    }
    #[fixed_stack_segment]
    fn setCodePage(&self, codePage: int) {
        unsafe { wxStyledTextCtrl_SetCodePage(self, codePage) }
    }
    #[fixed_stack_segment]
    fn markerDefine(&self, markerNumber: int, markerSymbol: int, foreground_r: u8, foreground_g: u8, foreground_b: u8, background_r: u8, background_g: u8, background_b: u8) {
        unsafe { wxStyledTextCtrl_MarkerDefine(self, markerNumber, markerSymbol, _arg3, _arg4) }
    }
    #[fixed_stack_segment]
    fn markerSetForeground(&self, markerNumber: int, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_MarkerSetForeground(self, markerNumber, _arg2) }
    }
    #[fixed_stack_segment]
    fn markerSetBackground(&self, markerNumber: int, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_MarkerSetBackground(self, markerNumber, _arg2) }
    }
    #[fixed_stack_segment]
    fn markerAdd(&self, line: int, markerNumber: int) -> int {
        unsafe { wxStyledTextCtrl_MarkerAdd(self, line, markerNumber) }
    }
    #[fixed_stack_segment]
    fn markerDelete(&self, line: int, markerNumber: int) {
        unsafe { wxStyledTextCtrl_MarkerDelete(self, line, markerNumber) }
    }
    #[fixed_stack_segment]
    fn markerDeleteAll(&self, markerNumber: int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteAll(self, markerNumber) }
    }
    #[fixed_stack_segment]
    fn markerGet(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_MarkerGet(self, line) }
    }
    #[fixed_stack_segment]
    fn markerNext(&self, lineStart: int, markerMask: int) -> int {
        unsafe { wxStyledTextCtrl_MarkerNext(self, lineStart, markerMask) }
    }
    #[fixed_stack_segment]
    fn markerPrevious(&self, lineStart: int, markerMask: int) -> int {
        unsafe { wxStyledTextCtrl_MarkerPrevious(self, lineStart, markerMask) }
    }
    #[fixed_stack_segment]
    fn markerDefineBitmap(&self, markerNumber: int, bmp: @wxBitmap) {
        unsafe { wxStyledTextCtrl_MarkerDefineBitmap(self, markerNumber, bmp) }
    }
    #[fixed_stack_segment]
    fn setMarginType(&self, margin: int, marginType: int) {
        unsafe { wxStyledTextCtrl_SetMarginType(self, margin, marginType) }
    }
    #[fixed_stack_segment]
    fn getMarginType(&self, margin: int) -> int {
        unsafe { wxStyledTextCtrl_GetMarginType(self, margin) }
    }
    #[fixed_stack_segment]
    fn setMarginWidth(&self, margin: int, pixelWidth: int) {
        unsafe { wxStyledTextCtrl_SetMarginWidth(self, margin, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getMarginWidth(&self, margin: int) -> int {
        unsafe { wxStyledTextCtrl_GetMarginWidth(self, margin) }
    }
    #[fixed_stack_segment]
    fn setMarginMask(&self, margin: int, mask: int) {
        unsafe { wxStyledTextCtrl_SetMarginMask(self, margin, mask) }
    }
    #[fixed_stack_segment]
    fn getMarginMask(&self, margin: int) -> int {
        unsafe { wxStyledTextCtrl_GetMarginMask(self, margin) }
    }
    #[fixed_stack_segment]
    fn setMarginSensitive(&self, margin: int, sensitive: bool) {
        unsafe { wxStyledTextCtrl_SetMarginSensitive(self, margin, sensitive) }
    }
    #[fixed_stack_segment]
    fn getMarginSensitive(&self, margin: int) -> bool {
        unsafe { wxStyledTextCtrl_GetMarginSensitive(self, margin) }
    }
    #[fixed_stack_segment]
    fn styleClearAll(&self) {
        unsafe { wxStyledTextCtrl_StyleClearAll(self) }
    }
    #[fixed_stack_segment]
    fn styleSetForeground(&self, style: int, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_StyleSetForeground(self, style, _arg2) }
    }
    #[fixed_stack_segment]
    fn styleSetBackground(&self, style: int, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_StyleSetBackground(self, style, _arg2) }
    }
    #[fixed_stack_segment]
    fn styleSetBold(&self, style: int, bold: bool) {
        unsafe { wxStyledTextCtrl_StyleSetBold(self, style, bold) }
    }
    #[fixed_stack_segment]
    fn styleSetItalic(&self, style: int, italic: bool) {
        unsafe { wxStyledTextCtrl_StyleSetItalic(self, style, italic) }
    }
    #[fixed_stack_segment]
    fn styleSetSize(&self, style: int, sizePoints: int) {
        unsafe { wxStyledTextCtrl_StyleSetSize(self, style, sizePoints) }
    }
    #[fixed_stack_segment]
    fn styleSetFaceName(&self, style: int, fontName: @wxString) {
        unsafe { wxStyledTextCtrl_StyleSetFaceName(self, style, fontName) }
    }
    #[fixed_stack_segment]
    fn styleSetEOLFilled(&self, style: int, filled: bool) {
        unsafe { wxStyledTextCtrl_StyleSetEOLFilled(self, style, filled) }
    }
    #[fixed_stack_segment]
    fn styleResetDefault(&self) {
        unsafe { wxStyledTextCtrl_StyleResetDefault(self) }
    }
    #[fixed_stack_segment]
    fn styleSetUnderline(&self, style: int, underline: bool) {
        unsafe { wxStyledTextCtrl_StyleSetUnderline(self, style, underline) }
    }
    #[fixed_stack_segment]
    fn styleSetCase(&self, style: int, caseForce: int) {
        unsafe { wxStyledTextCtrl_StyleSetCase(self, style, caseForce) }
    }
    #[fixed_stack_segment]
    fn styleSetCharacterSet(&self, style: int, characterSet: int) {
        unsafe { wxStyledTextCtrl_StyleSetCharacterSet(self, style, characterSet) }
    }
    #[fixed_stack_segment]
    fn styleSetHotSpot(&self, style: int, hotspot: bool) {
        unsafe { wxStyledTextCtrl_StyleSetHotSpot(self, style, hotspot) }
    }
    #[fixed_stack_segment]
    fn setSelForeground(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_SetSelForeground(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setSelBackground(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_SetSelBackground(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setCaretForeground(&self, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_SetCaretForeground(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn cmdKeyAssign(&self, key: int, modifiers: int, cmd: int) {
        unsafe { wxStyledTextCtrl_CmdKeyAssign(self, key, modifiers, cmd) }
    }
    #[fixed_stack_segment]
    fn cmdKeyClear(&self, key: int, modifiers: int) {
        unsafe { wxStyledTextCtrl_CmdKeyClear(self, key, modifiers) }
    }
    #[fixed_stack_segment]
    fn cmdKeyClearAll(&self) {
        unsafe { wxStyledTextCtrl_CmdKeyClearAll(self) }
    }
    #[fixed_stack_segment]
    fn setStyleBytes(&self, length: int, styleBytes: *char) {
        unsafe { wxStyledTextCtrl_SetStyleBytes(self, length, styleBytes) }
    }
    #[fixed_stack_segment]
    fn styleSetVisible(&self, style: int, visible: bool) {
        unsafe { wxStyledTextCtrl_StyleSetVisible(self, style, visible) }
    }
    #[fixed_stack_segment]
    fn getCaretPeriod(&self) -> int {
        unsafe { wxStyledTextCtrl_GetCaretPeriod(self) }
    }
    #[fixed_stack_segment]
    fn setCaretPeriod(&self, periodMilliseconds: int) {
        unsafe { wxStyledTextCtrl_SetCaretPeriod(self, periodMilliseconds) }
    }
    #[fixed_stack_segment]
    fn setWordChars(&self, characters: @wxString) {
        unsafe { wxStyledTextCtrl_SetWordChars(self, characters) }
    }
    #[fixed_stack_segment]
    fn beginUndoAction(&self) {
        unsafe { wxStyledTextCtrl_BeginUndoAction(self) }
    }
    #[fixed_stack_segment]
    fn endUndoAction(&self) {
        unsafe { wxStyledTextCtrl_EndUndoAction(self) }
    }
    #[fixed_stack_segment]
    fn indicatorSetStyle(&self, indic: int, style: int) {
        unsafe { wxStyledTextCtrl_IndicatorSetStyle(self, indic, style) }
    }
    #[fixed_stack_segment]
    fn indicatorGetStyle(&self, indic: int) -> int {
        unsafe { wxStyledTextCtrl_IndicatorGetStyle(self, indic) }
    }
    #[fixed_stack_segment]
    fn indicatorSetForeground(&self, indic: int, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_IndicatorSetForeground(self, indic, _arg2) }
    }
    #[fixed_stack_segment]
    fn setWhitespaceForeground(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_SetWhitespaceForeground(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setWhitespaceBackground(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_SetWhitespaceBackground(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setStyleBits(&self, bits: int) {
        unsafe { wxStyledTextCtrl_SetStyleBits(self, bits) }
    }
    #[fixed_stack_segment]
    fn getStyleBits(&self) -> int {
        unsafe { wxStyledTextCtrl_GetStyleBits(self) }
    }
    #[fixed_stack_segment]
    fn setLineState(&self, line: int, state: int) {
        unsafe { wxStyledTextCtrl_SetLineState(self, line, state) }
    }
    #[fixed_stack_segment]
    fn getLineState(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_GetLineState(self, line) }
    }
    #[fixed_stack_segment]
    fn getMaxLineState(&self) -> int {
        unsafe { wxStyledTextCtrl_GetMaxLineState(self) }
    }
    #[fixed_stack_segment]
    fn getCaretLineVisible(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetCaretLineVisible(self) }
    }
    #[fixed_stack_segment]
    fn setCaretLineVisible(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetCaretLineVisible(self, show) }
    }
    #[fixed_stack_segment]
    fn styleSetChangeable(&self, style: int, changeable: bool) {
        unsafe { wxStyledTextCtrl_StyleSetChangeable(self, style, changeable) }
    }
    #[fixed_stack_segment]
    fn autoCompShow(&self, lenEntered: int, itemList: @wxString) {
        unsafe { wxStyledTextCtrl_AutoCompShow(self, lenEntered, itemList) }
    }
    #[fixed_stack_segment]
    fn autoCompCancel(&self) {
        unsafe { wxStyledTextCtrl_AutoCompCancel(self) }
    }
    #[fixed_stack_segment]
    fn autoCompActive(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompActive(self) }
    }
    #[fixed_stack_segment]
    fn autoCompPosStart(&self) -> int {
        unsafe { wxStyledTextCtrl_AutoCompPosStart(self) }
    }
    #[fixed_stack_segment]
    fn autoCompComplete(&self) {
        unsafe { wxStyledTextCtrl_AutoCompComplete(self) }
    }
    #[fixed_stack_segment]
    fn autoCompStops(&self, characterSet: @wxString) {
        unsafe { wxStyledTextCtrl_AutoCompStops(self, characterSet) }
    }
    #[fixed_stack_segment]
    fn autoCompSetSeparator(&self, separatorCharacter: int) {
        unsafe { wxStyledTextCtrl_AutoCompSetSeparator(self, separatorCharacter) }
    }
    #[fixed_stack_segment]
    fn autoCompGetSeparator(&self) -> int {
        unsafe { wxStyledTextCtrl_AutoCompGetSeparator(self) }
    }
    #[fixed_stack_segment]
    fn autoCompSelect(&self, text: @wxString) {
        unsafe { wxStyledTextCtrl_AutoCompSelect(self, text) }
    }
    #[fixed_stack_segment]
    fn autoCompSetCancelAtStart(&self, cancel: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetCancelAtStart(self, cancel) }
    }
    #[fixed_stack_segment]
    fn autoCompGetCancelAtStart(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetCancelAtStart(self) }
    }
    #[fixed_stack_segment]
    fn autoCompSetFillUps(&self, characterSet: @wxString) {
        unsafe { wxStyledTextCtrl_AutoCompSetFillUps(self, characterSet) }
    }
    #[fixed_stack_segment]
    fn autoCompSetChooseSingle(&self, chooseSingle: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetChooseSingle(self, chooseSingle) }
    }
    #[fixed_stack_segment]
    fn autoCompGetChooseSingle(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetChooseSingle(self) }
    }
    #[fixed_stack_segment]
    fn autoCompSetIgnoreCase(&self, ignoreCase: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetIgnoreCase(self, ignoreCase) }
    }
    #[fixed_stack_segment]
    fn autoCompGetIgnoreCase(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetIgnoreCase(self) }
    }
    #[fixed_stack_segment]
    fn userListShow(&self, listType: int, itemList: @wxString) {
        unsafe { wxStyledTextCtrl_UserListShow(self, listType, itemList) }
    }
    #[fixed_stack_segment]
    fn autoCompSetAutoHide(&self, autoHide: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetAutoHide(self, autoHide) }
    }
    #[fixed_stack_segment]
    fn autoCompGetAutoHide(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetAutoHide(self) }
    }
    #[fixed_stack_segment]
    fn autoCompSetDropRestOfWord(&self, dropRestOfWord: bool) {
        unsafe { wxStyledTextCtrl_AutoCompSetDropRestOfWord(self, dropRestOfWord) }
    }
    #[fixed_stack_segment]
    fn autoCompGetDropRestOfWord(&self) -> bool {
        unsafe { wxStyledTextCtrl_AutoCompGetDropRestOfWord(self) }
    }
    #[fixed_stack_segment]
    fn registerImage(&self, type_: int, bmp: @wxBitmap) {
        unsafe { wxStyledTextCtrl_RegisterImage(self, type_, bmp) }
    }
    #[fixed_stack_segment]
    fn clearRegisteredImages(&self) {
        unsafe { wxStyledTextCtrl_ClearRegisteredImages(self) }
    }
    #[fixed_stack_segment]
    fn autoCompGetTypeSeparator(&self) -> int {
        unsafe { wxStyledTextCtrl_AutoCompGetTypeSeparator(self) }
    }
    #[fixed_stack_segment]
    fn autoCompSetTypeSeparator(&self, separatorCharacter: int) {
        unsafe { wxStyledTextCtrl_AutoCompSetTypeSeparator(self, separatorCharacter) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indentSize: int) {
        unsafe { wxStyledTextCtrl_SetIndent(self, indentSize) }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> int {
        unsafe { wxStyledTextCtrl_GetIndent(self) }
    }
    #[fixed_stack_segment]
    fn setUseTabs(&self, useTabs: bool) {
        unsafe { wxStyledTextCtrl_SetUseTabs(self, useTabs) }
    }
    #[fixed_stack_segment]
    fn getUseTabs(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseTabs(self) }
    }
    #[fixed_stack_segment]
    fn setLineIndentation(&self, line: int, indentSize: int) {
        unsafe { wxStyledTextCtrl_SetLineIndentation(self, line, indentSize) }
    }
    #[fixed_stack_segment]
    fn getLineIndentation(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_GetLineIndentation(self, line) }
    }
    #[fixed_stack_segment]
    fn getLineIndentPosition(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_GetLineIndentPosition(self, line) }
    }
    #[fixed_stack_segment]
    fn getColumn(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_GetColumn(self, pos) }
    }
    #[fixed_stack_segment]
    fn setUseHorizontalScrollBar(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetUseHorizontalScrollBar(self, show) }
    }
    #[fixed_stack_segment]
    fn getUseHorizontalScrollBar(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseHorizontalScrollBar(self) }
    }
    #[fixed_stack_segment]
    fn setIndentationGuides(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetIndentationGuides(self, show) }
    }
    #[fixed_stack_segment]
    fn getIndentationGuides(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetIndentationGuides(self) }
    }
    #[fixed_stack_segment]
    fn setHighlightGuide(&self, column: int) {
        unsafe { wxStyledTextCtrl_SetHighlightGuide(self, column) }
    }
    #[fixed_stack_segment]
    fn getHighlightGuide(&self) -> int {
        unsafe { wxStyledTextCtrl_GetHighlightGuide(self) }
    }
    #[fixed_stack_segment]
    fn getLineEndPosition(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_GetLineEndPosition(self, line) }
    }
    #[fixed_stack_segment]
    fn getCodePage(&self) -> int {
        unsafe { wxStyledTextCtrl_GetCodePage(self) }
    }
    #[fixed_stack_segment]
    fn getReadOnly(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetReadOnly(self) }
    }
    #[fixed_stack_segment]
    fn setCurrentPos(&self, pos: int) {
        unsafe { wxStyledTextCtrl_SetCurrentPos(self, pos) }
    }
    #[fixed_stack_segment]
    fn setSelectionStart(&self, pos: int) {
        unsafe { wxStyledTextCtrl_SetSelectionStart(self, pos) }
    }
    #[fixed_stack_segment]
    fn getSelectionStart(&self) -> int {
        unsafe { wxStyledTextCtrl_GetSelectionStart(self) }
    }
    #[fixed_stack_segment]
    fn setSelectionEnd(&self, pos: int) {
        unsafe { wxStyledTextCtrl_SetSelectionEnd(self, pos) }
    }
    #[fixed_stack_segment]
    fn getSelectionEnd(&self) -> int {
        unsafe { wxStyledTextCtrl_GetSelectionEnd(self) }
    }
    #[fixed_stack_segment]
    fn setPrintMagnification(&self, magnification: int) {
        unsafe { wxStyledTextCtrl_SetPrintMagnification(self, magnification) }
    }
    #[fixed_stack_segment]
    fn getPrintMagnification(&self) -> int {
        unsafe { wxStyledTextCtrl_GetPrintMagnification(self) }
    }
    #[fixed_stack_segment]
    fn setPrintColourMode(&self, mode: int) {
        unsafe { wxStyledTextCtrl_SetPrintColourMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn getPrintColourMode(&self) -> int {
        unsafe { wxStyledTextCtrl_GetPrintColourMode(self) }
    }
    #[fixed_stack_segment]
    fn findText(&self, minPos: int, maxPos: int, text: @wxString, flags: int) -> int {
        unsafe { wxStyledTextCtrl_FindText(self, minPos, maxPos, text, flags) }
    }
    #[fixed_stack_segment]
    fn formatRange(&self, doDraw: bool, startPos: int, endPos: int, draw: @wxDC, target: @wxDC, renderRect: @wxRect, pageRect: @wxRect) -> int {
        unsafe { wxStyledTextCtrl_FormatRange(self, doDraw, startPos, endPos, draw, target, renderRect, pageRect) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleLine(&self) -> int {
        unsafe { wxStyledTextCtrl_GetFirstVisibleLine(self) }
    }
    #[fixed_stack_segment]
    fn getLineCount(&self) -> int {
        unsafe { wxStyledTextCtrl_GetLineCount(self) }
    }
    #[fixed_stack_segment]
    fn setMarginLeft(&self, pixelWidth: int) {
        unsafe { wxStyledTextCtrl_SetMarginLeft(self, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getMarginLeft(&self) -> int {
        unsafe { wxStyledTextCtrl_GetMarginLeft(self) }
    }
    #[fixed_stack_segment]
    fn setMarginRight(&self, pixelWidth: int) {
        unsafe { wxStyledTextCtrl_SetMarginRight(self, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getMarginRight(&self) -> int {
        unsafe { wxStyledTextCtrl_GetMarginRight(self) }
    }
    #[fixed_stack_segment]
    fn getModify(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetModify(self) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, start: int, end: int) {
        unsafe { wxStyledTextCtrl_SetSelection(self, start, end) }
    }
    #[fixed_stack_segment]
    fn hideSelection(&self, normal: bool) {
        unsafe { wxStyledTextCtrl_HideSelection(self, normal) }
    }
    #[fixed_stack_segment]
    fn lineFromPosition(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_LineFromPosition(self, pos) }
    }
    #[fixed_stack_segment]
    fn positionFromLine(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_PositionFromLine(self, line) }
    }
    #[fixed_stack_segment]
    fn lineScroll(&self, columns: int, lines: int) {
        unsafe { wxStyledTextCtrl_LineScroll(self, columns, lines) }
    }
    #[fixed_stack_segment]
    fn ensureCaretVisible(&self) {
        unsafe { wxStyledTextCtrl_EnsureCaretVisible(self) }
    }
    #[fixed_stack_segment]
    fn replaceSelection(&self, text: @wxString) {
        unsafe { wxStyledTextCtrl_ReplaceSelection(self, text) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, readOnly: bool) {
        unsafe { wxStyledTextCtrl_SetReadOnly(self, readOnly) }
    }
    #[fixed_stack_segment]
    fn canPaste(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanPaste(self) }
    }
    #[fixed_stack_segment]
    fn canUndo(&self) -> bool {
        unsafe { wxStyledTextCtrl_CanUndo(self) }
    }
    #[fixed_stack_segment]
    fn emptyUndoBuffer(&self) {
        unsafe { wxStyledTextCtrl_EmptyUndoBuffer(self) }
    }
    #[fixed_stack_segment]
    fn undo(&self) {
        unsafe { wxStyledTextCtrl_Undo(self) }
    }
    #[fixed_stack_segment]
    fn cut(&self) {
        unsafe { wxStyledTextCtrl_Cut(self) }
    }
    #[fixed_stack_segment]
    fn copy(&self) {
        unsafe { wxStyledTextCtrl_Copy(self) }
    }
    #[fixed_stack_segment]
    fn paste(&self) {
        unsafe { wxStyledTextCtrl_Paste(self) }
    }
    #[fixed_stack_segment]
    fn clear(&self) {
        unsafe { wxStyledTextCtrl_Clear(self) }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @wxString) {
        unsafe { wxStyledTextCtrl_SetText(self, text) }
    }
    #[fixed_stack_segment]
    fn getTextLength(&self) -> int {
        unsafe { wxStyledTextCtrl_GetTextLength(self) }
    }
    #[fixed_stack_segment]
    fn setOvertype(&self, overtype: bool) {
        unsafe { wxStyledTextCtrl_SetOvertype(self, overtype) }
    }
    #[fixed_stack_segment]
    fn getOvertype(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetOvertype(self) }
    }
    #[fixed_stack_segment]
    fn setCaretWidth(&self, pixelWidth: int) {
        unsafe { wxStyledTextCtrl_SetCaretWidth(self, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getCaretWidth(&self) -> int {
        unsafe { wxStyledTextCtrl_GetCaretWidth(self) }
    }
    #[fixed_stack_segment]
    fn setTargetStart(&self, pos: int) {
        unsafe { wxStyledTextCtrl_SetTargetStart(self, pos) }
    }
    #[fixed_stack_segment]
    fn getTargetStart(&self) -> int {
        unsafe { wxStyledTextCtrl_GetTargetStart(self) }
    }
    #[fixed_stack_segment]
    fn setTargetEnd(&self, pos: int) {
        unsafe { wxStyledTextCtrl_SetTargetEnd(self, pos) }
    }
    #[fixed_stack_segment]
    fn getTargetEnd(&self) -> int {
        unsafe { wxStyledTextCtrl_GetTargetEnd(self) }
    }
    #[fixed_stack_segment]
    fn replaceTarget(&self, text: @wxString) -> int {
        unsafe { wxStyledTextCtrl_ReplaceTarget(self, text) }
    }
    #[fixed_stack_segment]
    fn replaceTargetRE(&self, text: @wxString) -> int {
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self, text) }
    }
    #[fixed_stack_segment]
    fn searchInTarget(&self, text: @wxString) -> int {
        unsafe { wxStyledTextCtrl_SearchInTarget(self, text) }
    }
    #[fixed_stack_segment]
    fn setSearchFlags(&self, flags: int) {
        unsafe { wxStyledTextCtrl_SetSearchFlags(self, flags) }
    }
    #[fixed_stack_segment]
    fn getSearchFlags(&self) -> int {
        unsafe { wxStyledTextCtrl_GetSearchFlags(self) }
    }
    #[fixed_stack_segment]
    fn callTipShow(&self, pos: int, definition: @wxString) {
        unsafe { wxStyledTextCtrl_CallTipShow(self, pos, definition) }
    }
    #[fixed_stack_segment]
    fn callTipCancel(&self) {
        unsafe { wxStyledTextCtrl_CallTipCancel(self) }
    }
    #[fixed_stack_segment]
    fn callTipActive(&self) -> bool {
        unsafe { wxStyledTextCtrl_CallTipActive(self) }
    }
    #[fixed_stack_segment]
    fn callTipPosAtStart(&self) -> int {
        unsafe { wxStyledTextCtrl_CallTipPosAtStart(self) }
    }
    #[fixed_stack_segment]
    fn callTipSetHighlight(&self, start: int, end: int) {
        unsafe { wxStyledTextCtrl_CallTipSetHighlight(self, start, end) }
    }
    #[fixed_stack_segment]
    fn callTipSetBackground(&self, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_CallTipSetBackground(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn callTipSetForeground(&self, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_CallTipSetForeground(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn callTipSetForegroundHighlight(&self, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_CallTipSetForegroundHighlight(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn visibleFromDocLine(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_VisibleFromDocLine(self, line) }
    }
    #[fixed_stack_segment]
    fn docLineFromVisible(&self, lineDisplay: int) -> int {
        unsafe { wxStyledTextCtrl_DocLineFromVisible(self, lineDisplay) }
    }
    #[fixed_stack_segment]
    fn setFoldLevel(&self, line: int, level: int) {
        unsafe { wxStyledTextCtrl_SetFoldLevel(self, line, level) }
    }
    #[fixed_stack_segment]
    fn getFoldLevel(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_GetFoldLevel(self, line) }
    }
    #[fixed_stack_segment]
    fn getLastChild(&self, line: int, level: int) -> int {
        unsafe { wxStyledTextCtrl_GetLastChild(self, line, level) }
    }
    #[fixed_stack_segment]
    fn getFoldParent(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_GetFoldParent(self, line) }
    }
    #[fixed_stack_segment]
    fn showLines(&self, lineStart: int, lineEnd: int) {
        unsafe { wxStyledTextCtrl_ShowLines(self, lineStart, lineEnd) }
    }
    #[fixed_stack_segment]
    fn hideLines(&self, lineStart: int, lineEnd: int) {
        unsafe { wxStyledTextCtrl_HideLines(self, lineStart, lineEnd) }
    }
    #[fixed_stack_segment]
    fn getLineVisible(&self, line: int) -> bool {
        unsafe { wxStyledTextCtrl_GetLineVisible(self, line) }
    }
    #[fixed_stack_segment]
    fn setFoldExpanded(&self, line: int, expanded: bool) {
        unsafe { wxStyledTextCtrl_SetFoldExpanded(self, line, expanded) }
    }
    #[fixed_stack_segment]
    fn getFoldExpanded(&self, line: int) -> bool {
        unsafe { wxStyledTextCtrl_GetFoldExpanded(self, line) }
    }
    #[fixed_stack_segment]
    fn toggleFold(&self, line: int) {
        unsafe { wxStyledTextCtrl_ToggleFold(self, line) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, line: int) {
        unsafe { wxStyledTextCtrl_EnsureVisible(self, line) }
    }
    #[fixed_stack_segment]
    fn setFoldFlags(&self, flags: int) {
        unsafe { wxStyledTextCtrl_SetFoldFlags(self, flags) }
    }
    #[fixed_stack_segment]
    fn ensureVisibleEnforcePolicy(&self, line: int) {
        unsafe { wxStyledTextCtrl_EnsureVisibleEnforcePolicy(self, line) }
    }
    #[fixed_stack_segment]
    fn setTabIndents(&self, tabIndents: bool) {
        unsafe { wxStyledTextCtrl_SetTabIndents(self, tabIndents) }
    }
    #[fixed_stack_segment]
    fn getTabIndents(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetTabIndents(self) }
    }
    #[fixed_stack_segment]
    fn setBackSpaceUnIndents(&self, bsUnIndents: bool) {
        unsafe { wxStyledTextCtrl_SetBackSpaceUnIndents(self, bsUnIndents) }
    }
    #[fixed_stack_segment]
    fn getBackSpaceUnIndents(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetBackSpaceUnIndents(self) }
    }
    #[fixed_stack_segment]
    fn setMouseDwellTime(&self, periodMilliseconds: int) {
        unsafe { wxStyledTextCtrl_SetMouseDwellTime(self, periodMilliseconds) }
    }
    #[fixed_stack_segment]
    fn getMouseDwellTime(&self) -> int {
        unsafe { wxStyledTextCtrl_GetMouseDwellTime(self) }
    }
    #[fixed_stack_segment]
    fn wordStartPosition(&self, pos: int, onlyWordCharacters: bool) -> int {
        unsafe { wxStyledTextCtrl_WordStartPosition(self, pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    fn wordEndPosition(&self, pos: int, onlyWordCharacters: bool) -> int {
        unsafe { wxStyledTextCtrl_WordEndPosition(self, pos, onlyWordCharacters) }
    }
    #[fixed_stack_segment]
    fn setWrapMode(&self, mode: int) {
        unsafe { wxStyledTextCtrl_SetWrapMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn getWrapMode(&self) -> int {
        unsafe { wxStyledTextCtrl_GetWrapMode(self) }
    }
    #[fixed_stack_segment]
    fn setLayoutCache(&self, mode: int) {
        unsafe { wxStyledTextCtrl_SetLayoutCache(self, mode) }
    }
    #[fixed_stack_segment]
    fn getLayoutCache(&self) -> int {
        unsafe { wxStyledTextCtrl_GetLayoutCache(self) }
    }
    #[fixed_stack_segment]
    fn setScrollWidth(&self, pixelWidth: int) {
        unsafe { wxStyledTextCtrl_SetScrollWidth(self, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn getScrollWidth(&self) -> int {
        unsafe { wxStyledTextCtrl_GetScrollWidth(self) }
    }
    #[fixed_stack_segment]
    fn textWidth(&self, style: int, text: @wxString) -> int {
        unsafe { wxStyledTextCtrl_TextWidth(self, style, text) }
    }
    #[fixed_stack_segment]
    fn setEndAtLastLine(&self, endAtLastLine: bool) {
        unsafe { wxStyledTextCtrl_SetEndAtLastLine(self, endAtLastLine) }
    }
    #[fixed_stack_segment]
    fn getEndAtLastLine(&self) -> int {
        unsafe { wxStyledTextCtrl_GetEndAtLastLine(self) }
    }
    #[fixed_stack_segment]
    fn textHeight(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_TextHeight(self, line) }
    }
    #[fixed_stack_segment]
    fn setUseVerticalScrollBar(&self, show: bool) {
        unsafe { wxStyledTextCtrl_SetUseVerticalScrollBar(self, show) }
    }
    #[fixed_stack_segment]
    fn getUseVerticalScrollBar(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetUseVerticalScrollBar(self) }
    }
    #[fixed_stack_segment]
    fn appendText(&self, text: @wxString) {
        unsafe { wxStyledTextCtrl_AppendText(self, text) }
    }
    #[fixed_stack_segment]
    fn getTwoPhaseDraw(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetTwoPhaseDraw(self) }
    }
    #[fixed_stack_segment]
    fn setTwoPhaseDraw(&self, twoPhase: bool) {
        unsafe { wxStyledTextCtrl_SetTwoPhaseDraw(self, twoPhase) }
    }
    #[fixed_stack_segment]
    fn targetFromSelection(&self) {
        unsafe { wxStyledTextCtrl_TargetFromSelection(self) }
    }
    #[fixed_stack_segment]
    fn linesJoin(&self) {
        unsafe { wxStyledTextCtrl_LinesJoin(self) }
    }
    #[fixed_stack_segment]
    fn linesSplit(&self, pixelWidth: int) {
        unsafe { wxStyledTextCtrl_LinesSplit(self, pixelWidth) }
    }
    #[fixed_stack_segment]
    fn setFoldMarginColour(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_SetFoldMarginColour(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setFoldMarginHiColour(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_SetFoldMarginHiColour(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn lineDuplicate(&self) {
        unsafe { wxStyledTextCtrl_LineDuplicate(self) }
    }
    #[fixed_stack_segment]
    fn homeDisplay(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplay(self) }
    }
    #[fixed_stack_segment]
    fn homeDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplayExtend(self) }
    }
    #[fixed_stack_segment]
    fn lineEndDisplay(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplay(self) }
    }
    #[fixed_stack_segment]
    fn lineEndDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplayExtend(self) }
    }
    #[fixed_stack_segment]
    fn lineCopy(&self) {
        unsafe { wxStyledTextCtrl_LineCopy(self) }
    }
    #[fixed_stack_segment]
    fn moveCaretInsideView(&self) {
        unsafe { wxStyledTextCtrl_MoveCaretInsideView(self) }
    }
    #[fixed_stack_segment]
    fn lineLength(&self, line: int) -> int {
        unsafe { wxStyledTextCtrl_LineLength(self, line) }
    }
    #[fixed_stack_segment]
    fn braceHighlight(&self, pos1: int, pos2: int) {
        unsafe { wxStyledTextCtrl_BraceHighlight(self, pos1, pos2) }
    }
    #[fixed_stack_segment]
    fn braceBadLight(&self, pos: int) {
        unsafe { wxStyledTextCtrl_BraceBadLight(self, pos) }
    }
    #[fixed_stack_segment]
    fn braceMatch(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_BraceMatch(self, pos) }
    }
    #[fixed_stack_segment]
    fn getViewEOL(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetViewEOL(self) }
    }
    #[fixed_stack_segment]
    fn setViewEOL(&self, visible: bool) {
        unsafe { wxStyledTextCtrl_SetViewEOL(self, visible) }
    }
    #[fixed_stack_segment]
    fn setDocPointer(&self, docPointer: @wxSTCDoc) {
        unsafe { wxStyledTextCtrl_SetDocPointer(self, docPointer) }
    }
    #[fixed_stack_segment]
    fn setModEventMask(&self, mask: int) {
        unsafe { wxStyledTextCtrl_SetModEventMask(self, mask) }
    }
    #[fixed_stack_segment]
    fn getEdgeColumn(&self) -> int {
        unsafe { wxStyledTextCtrl_GetEdgeColumn(self) }
    }
    #[fixed_stack_segment]
    fn setEdgeColumn(&self, column: int) {
        unsafe { wxStyledTextCtrl_SetEdgeColumn(self, column) }
    }
    #[fixed_stack_segment]
    fn getEdgeMode(&self) -> int {
        unsafe { wxStyledTextCtrl_GetEdgeMode(self) }
    }
    #[fixed_stack_segment]
    fn setEdgeMode(&self, mode: int) {
        unsafe { wxStyledTextCtrl_SetEdgeMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn setEdgeColour(&self, edgeColour_r: u8, edgeColour_g: u8, edgeColour_b: u8) {
        unsafe { wxStyledTextCtrl_SetEdgeColour(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn searchAnchor(&self) {
        unsafe { wxStyledTextCtrl_SearchAnchor(self) }
    }
    #[fixed_stack_segment]
    fn searchNext(&self, flags: int, text: @wxString) -> int {
        unsafe { wxStyledTextCtrl_SearchNext(self, flags, text) }
    }
    #[fixed_stack_segment]
    fn searchPrev(&self, flags: int, text: @wxString) -> int {
        unsafe { wxStyledTextCtrl_SearchPrev(self, flags, text) }
    }
    #[fixed_stack_segment]
    fn linesOnScreen(&self) -> int {
        unsafe { wxStyledTextCtrl_LinesOnScreen(self) }
    }
    #[fixed_stack_segment]
    fn usePopUp(&self, allowPopUp: bool) {
        unsafe { wxStyledTextCtrl_UsePopUp(self, allowPopUp) }
    }
    #[fixed_stack_segment]
    fn selectionIsRectangle(&self) -> bool {
        unsafe { wxStyledTextCtrl_SelectionIsRectangle(self) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, zoom: int) {
        unsafe { wxStyledTextCtrl_SetZoom(self, zoom) }
    }
    #[fixed_stack_segment]
    fn getZoom(&self) -> int {
        unsafe { wxStyledTextCtrl_GetZoom(self) }
    }
    #[fixed_stack_segment]
    fn addRefDocument(&self, docPointer: @wxSTCDoc) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self, docPointer) }
    }
    #[fixed_stack_segment]
    fn releaseDocument(&self, docPointer: @wxSTCDoc) {
        unsafe { wxStyledTextCtrl_ReleaseDocument(self, docPointer) }
    }
    #[fixed_stack_segment]
    fn getModEventMask(&self) -> int {
        unsafe { wxStyledTextCtrl_GetModEventMask(self) }
    }
    #[fixed_stack_segment]
    fn setSTCFocus(&self, focus: bool) {
        unsafe { wxStyledTextCtrl_SetSTCFocus(self, focus) }
    }
    #[fixed_stack_segment]
    fn getSTCFocus(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetSTCFocus(self) }
    }
    #[fixed_stack_segment]
    fn setStatus(&self, statusCode: int) {
        unsafe { wxStyledTextCtrl_SetStatus(self, statusCode) }
    }
    #[fixed_stack_segment]
    fn getStatus(&self) -> int {
        unsafe { wxStyledTextCtrl_GetStatus(self) }
    }
    #[fixed_stack_segment]
    fn setMouseDownCaptures(&self, captures: bool) {
        unsafe { wxStyledTextCtrl_SetMouseDownCaptures(self, captures) }
    }
    #[fixed_stack_segment]
    fn getMouseDownCaptures(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetMouseDownCaptures(self) }
    }
    #[fixed_stack_segment]
    fn setSTCCursor(&self, cursorType: int) {
        unsafe { wxStyledTextCtrl_SetSTCCursor(self, cursorType) }
    }
    #[fixed_stack_segment]
    fn getSTCCursor(&self) -> int {
        unsafe { wxStyledTextCtrl_GetSTCCursor(self) }
    }
    #[fixed_stack_segment]
    fn setControlCharSymbol(&self, symbol: int) {
        unsafe { wxStyledTextCtrl_SetControlCharSymbol(self, symbol) }
    }
    #[fixed_stack_segment]
    fn getControlCharSymbol(&self) -> int {
        unsafe { wxStyledTextCtrl_GetControlCharSymbol(self) }
    }
    #[fixed_stack_segment]
    fn wordPartLeft(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeft(self) }
    }
    #[fixed_stack_segment]
    fn wordPartLeftExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeftExtend(self) }
    }
    #[fixed_stack_segment]
    fn wordPartRight(&self) {
        unsafe { wxStyledTextCtrl_WordPartRight(self) }
    }
    #[fixed_stack_segment]
    fn wordPartRightExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartRightExtend(self) }
    }
    #[fixed_stack_segment]
    fn setVisiblePolicy(&self, visiblePolicy: int, visibleSlop: int) {
        unsafe { wxStyledTextCtrl_SetVisiblePolicy(self, visiblePolicy, visibleSlop) }
    }
    #[fixed_stack_segment]
    fn delLineLeft(&self) {
        unsafe { wxStyledTextCtrl_DelLineLeft(self) }
    }
    #[fixed_stack_segment]
    fn delLineRight(&self) {
        unsafe { wxStyledTextCtrl_DelLineRight(self) }
    }
    #[fixed_stack_segment]
    fn setXOffset(&self, newOffset: int) {
        unsafe { wxStyledTextCtrl_SetXOffset(self, newOffset) }
    }
    #[fixed_stack_segment]
    fn getXOffset(&self) -> int {
        unsafe { wxStyledTextCtrl_GetXOffset(self) }
    }
    #[fixed_stack_segment]
    fn chooseCaretX(&self) {
        unsafe { wxStyledTextCtrl_ChooseCaretX(self) }
    }
    #[fixed_stack_segment]
    fn setXCaretPolicy(&self, caretPolicy: int, caretSlop: int) {
        unsafe { wxStyledTextCtrl_SetXCaretPolicy(self, caretPolicy, caretSlop) }
    }
    #[fixed_stack_segment]
    fn setYCaretPolicy(&self, caretPolicy: int, caretSlop: int) {
        unsafe { wxStyledTextCtrl_SetYCaretPolicy(self, caretPolicy, caretSlop) }
    }
    #[fixed_stack_segment]
    fn setPrintWrapMode(&self, mode: int) {
        unsafe { wxStyledTextCtrl_SetPrintWrapMode(self, mode) }
    }
    #[fixed_stack_segment]
    fn getPrintWrapMode(&self) -> int {
        unsafe { wxStyledTextCtrl_GetPrintWrapMode(self) }
    }
    #[fixed_stack_segment]
    fn setHotspotActiveForeground(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveForeground(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setHotspotActiveBackground(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveBackground(self, useSetting, _arg2) }
    }
    #[fixed_stack_segment]
    fn setHotspotActiveUnderline(&self, underline: bool) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveUnderline(self, underline) }
    }
    #[fixed_stack_segment]
    fn positionBefore(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_PositionBefore(self, pos) }
    }
    #[fixed_stack_segment]
    fn positionAfter(&self, pos: int) -> int {
        unsafe { wxStyledTextCtrl_PositionAfter(self, pos) }
    }
    #[fixed_stack_segment]
    fn copyRange(&self, start: int, end: int) {
        unsafe { wxStyledTextCtrl_CopyRange(self, start, end) }
    }
    #[fixed_stack_segment]
    fn copyText(&self, length: int, text: @wxString) {
        unsafe { wxStyledTextCtrl_CopyText(self, length, text) }
    }
    #[fixed_stack_segment]
    fn startRecord(&self) {
        unsafe { wxStyledTextCtrl_StartRecord(self) }
    }
    #[fixed_stack_segment]
    fn stopRecord(&self) {
        unsafe { wxStyledTextCtrl_StopRecord(self) }
    }
    #[fixed_stack_segment]
    fn setLexer(&self, lexer: int) {
        unsafe { wxStyledTextCtrl_SetLexer(self, lexer) }
    }
    #[fixed_stack_segment]
    fn getLexer(&self) -> int {
        unsafe { wxStyledTextCtrl_GetLexer(self) }
    }
    #[fixed_stack_segment]
    fn colourise(&self, start: int, end: int) {
        unsafe { wxStyledTextCtrl_Colourise(self, start, end) }
    }
    #[fixed_stack_segment]
    fn setProperty(&self, key: @wxString, value: @wxString) {
        unsafe { wxStyledTextCtrl_SetProperty(self, key, value) }
    }
    #[fixed_stack_segment]
    fn setKeyWords(&self, keywordSet: int, keyWords: @wxString) {
        unsafe { wxStyledTextCtrl_SetKeyWords(self, keywordSet, keyWords) }
    }
    #[fixed_stack_segment]
    fn setLexerLanguage(&self, language: @wxString) {
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self, language) }
    }
    #[fixed_stack_segment]
    fn getCurrentLine(&self) -> int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self) }
    }
    #[fixed_stack_segment]
    fn styleSetSpec(&self, styleNum: int, spec: @wxString) {
        unsafe { wxStyledTextCtrl_StyleSetSpec(self, styleNum, spec) }
    }
    #[fixed_stack_segment]
    fn styleSetFont(&self, styleNum: int, font: @wxFont) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self, styleNum, font) }
    }
    #[fixed_stack_segment]
    fn styleSetFontAttr(&self, styleNum: int, size: int, faceName: @wxString, bold: bool, italic: bool, underline: bool) {
        unsafe { wxStyledTextCtrl_StyleSetFontAttr(self, styleNum, size, faceName, bold, italic, underline) }
    }
    #[fixed_stack_segment]
    fn cmdKeyExecute(&self, cmd: int) {
        unsafe { wxStyledTextCtrl_CmdKeyExecute(self, cmd) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, left: int, right: int) {
        unsafe { wxStyledTextCtrl_SetMargins(self, left, right) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, startPos: *int, endPos: *int) {
        unsafe { wxStyledTextCtrl_GetSelection(self, startPos, endPos) }
    }
    #[fixed_stack_segment]
    fn scrollToLine(&self, line: int) {
        unsafe { wxStyledTextCtrl_ScrollToLine(self, line) }
    }
    #[fixed_stack_segment]
    fn scrollToColumn(&self, column: int) {
        unsafe { wxStyledTextCtrl_ScrollToColumn(self, column) }
    }
    #[fixed_stack_segment]
    fn setVScrollBar(&self, bar: @wxScrollBar) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self, bar) }
    }
    #[fixed_stack_segment]
    fn setHScrollBar(&self, bar: @wxScrollBar) {
        unsafe { wxStyledTextCtrl_SetHScrollBar(self, bar) }
    }
    #[fixed_stack_segment]
    fn getLastKeydownProcessed(&self) -> bool {
        unsafe { wxStyledTextCtrl_GetLastKeydownProcessed(self) }
    }
    #[fixed_stack_segment]
    fn setLastKeydownProcessed(&self, val: bool) {
        unsafe { wxStyledTextCtrl_SetLastKeydownProcessed(self, val) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, filename: @wxString) -> bool {
        unsafe { wxStyledTextCtrl_SaveFile(self, filename) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, filename: @wxString) -> bool {
        unsafe { wxStyledTextCtrl_LoadFile(self, filename) }
    }
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: int) -> @wxStyledTextCtrl {
        unsafe { wxStyledTextCtrl_Create(_prt, _id, _txt, _arg3, style) }
    }
    #[fixed_stack_segment]
    fn indicatorGetForeground(&self, indic: int) -> @wxColour {
        unsafe { wxStyledTextCtrl_IndicatorGetForeground(self, indic) }
    }
    #[fixed_stack_segment]
    fn getCaretLineBackground(&self) -> @wxColour {
        unsafe { wxStyledTextCtrl_GetCaretLineBackground(self) }
    }
    #[fixed_stack_segment]
    fn setCaretLineBackground(&self, back_r: u8, back_g: u8, back_b: u8) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self, _arg1) }
    }
    #[fixed_stack_segment]
    fn getCaretForeground(&self) -> @wxColour {
        unsafe { wxStyledTextCtrl_GetCaretForeground(self) }
    }
    #[fixed_stack_segment]
    fn getLine(&self, line: int) -> @wxString {
        unsafe { wxStyledTextCtrl_GetLine(self, line) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { wxStyledTextCtrl_GetText(self) }
    }
    #[fixed_stack_segment]
    fn getTextRange(&self, startPos: int, endPos: int) -> @wxString {
        unsafe { wxStyledTextCtrl_GetTextRange(self, startPos, endPos) }
    }
    #[fixed_stack_segment]
    fn getSelectedText(&self) -> @wxString {
        unsafe { wxStyledTextCtrl_GetSelectedText(self) }
    }
    #[fixed_stack_segment]
    fn newDocument(&self) -> @wxSTCDoc {
        unsafe { wxStyledTextCtrl_CreateDocument(self) }
    }
    #[fixed_stack_segment]
    fn getEdgeColour(&self) -> @wxColour {
        unsafe { wxStyledTextCtrl_GetEdgeColour(self) }
    }
    #[fixed_stack_segment]
    fn getDocPointer(&self) -> @wxSTCDoc {
        unsafe { wxStyledTextCtrl_GetDocPointer(self) }
    }
    #[fixed_stack_segment]
    fn pointFromPosition(&self) -> @wxPoint {
        unsafe { wxStyledTextCtrl_PointFromPosition(self) }
    }
}
trait wxSTCDoc {
}
trait wxMemoryBuffer {
}
trait wxStyledTextEvent {
    #[fixed_stack_segment]
    fn getPosition(&self) -> int {
        unsafe { wxStyledTextEvent_GetPosition(self) }
    }
    #[fixed_stack_segment]
    fn getKey(&self) -> int {
        unsafe { wxStyledTextEvent_GetKey(self) }
    }
    #[fixed_stack_segment]
    fn getModifiers(&self) -> int {
        unsafe { wxStyledTextEvent_GetModifiers(self) }
    }
    #[fixed_stack_segment]
    fn getModificationType(&self) -> int {
        unsafe { wxStyledTextEvent_GetModificationType(self) }
    }
    #[fixed_stack_segment]
    fn getLength(&self) -> int {
        unsafe { wxStyledTextEvent_GetLength(self) }
    }
    #[fixed_stack_segment]
    fn getLinesAdded(&self) -> int {
        unsafe { wxStyledTextEvent_GetLinesAdded(self) }
    }
    #[fixed_stack_segment]
    fn getLine(&self) -> int {
        unsafe { wxStyledTextEvent_GetLine(self) }
    }
    #[fixed_stack_segment]
    fn getFoldLevelNow(&self) -> int {
        unsafe { wxStyledTextEvent_GetFoldLevelNow(self) }
    }
    #[fixed_stack_segment]
    fn getFoldLevelPrev(&self) -> int {
        unsafe { wxStyledTextEvent_GetFoldLevelPrev(self) }
    }
    #[fixed_stack_segment]
    fn getMargin(&self) -> int {
        unsafe { wxStyledTextEvent_GetMargin(self) }
    }
    #[fixed_stack_segment]
    fn getMessage(&self) -> int {
        unsafe { wxStyledTextEvent_GetMessage(self) }
    }
    #[fixed_stack_segment]
    fn getWParam(&self) -> int {
        unsafe { wxStyledTextEvent_GetWParam(self) }
    }
    #[fixed_stack_segment]
    fn getLParam(&self) -> int {
        unsafe { wxStyledTextEvent_GetLParam(self) }
    }
    #[fixed_stack_segment]
    fn getListType(&self) -> int {
        unsafe { wxStyledTextEvent_GetListType(self) }
    }
    #[fixed_stack_segment]
    fn getX(&self) -> int {
        unsafe { wxStyledTextEvent_GetX(self) }
    }
    #[fixed_stack_segment]
    fn getY(&self) -> int {
        unsafe { wxStyledTextEvent_GetY(self) }
    }
    #[fixed_stack_segment]
    fn getDragText(&self) -> @wxString {
        unsafe { wxStyledTextEvent_GetDragText(self) }
    }
    #[fixed_stack_segment]
    fn getDragAllowMove(&self) -> bool {
        unsafe { wxStyledTextEvent_GetDragAllowMove(self) }
    }
    #[fixed_stack_segment]
    fn getDragResult(&self) -> int {
        unsafe { wxStyledTextEvent_GetDragResult(self) }
    }
    #[fixed_stack_segment]
    fn getShift(&self) -> bool {
        unsafe { wxStyledTextEvent_GetShift(self) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> bool {
        unsafe { wxStyledTextEvent_GetControl(self) }
    }
    #[fixed_stack_segment]
    fn getAlt(&self) -> bool {
        unsafe { wxStyledTextEvent_GetAlt(self) }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @wxString {
        unsafe { wxStyledTextEvent_GetText(self) }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @wxStyledTextEvent {
        unsafe { wxStyledTextEvent_Clone(self) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, pos: int) {
        unsafe { wxStyledTextEvent_SetPosition(self, pos) }
    }
    #[fixed_stack_segment]
    fn setKey(&self, k: int) {
        unsafe { wxStyledTextEvent_SetKey(self, k) }
    }
    #[fixed_stack_segment]
    fn setModifiers(&self, m: int) {
        unsafe { wxStyledTextEvent_SetModifiers(self, m) }
    }
    #[fixed_stack_segment]
    fn setModificationType(&self, t: int) {
        unsafe { wxStyledTextEvent_SetModificationType(self, t) }
    }
    #[fixed_stack_segment]
    fn setText(&self, t: @wxString) {
        unsafe { wxStyledTextEvent_SetText(self, t) }
    }
    #[fixed_stack_segment]
    fn setLength(&self, len: int) {
        unsafe { wxStyledTextEvent_SetLength(self, len) }
    }
    #[fixed_stack_segment]
    fn setLinesAdded(&self, num: int) {
        unsafe { wxStyledTextEvent_SetLinesAdded(self, num) }
    }
    #[fixed_stack_segment]
    fn setLine(&self, val: int) {
        unsafe { wxStyledTextEvent_SetLine(self, val) }
    }
    #[fixed_stack_segment]
    fn setFoldLevelNow(&self, val: int) {
        unsafe { wxStyledTextEvent_SetFoldLevelNow(self, val) }
    }
    #[fixed_stack_segment]
    fn setFoldLevelPrev(&self, val: int) {
        unsafe { wxStyledTextEvent_SetFoldLevelPrev(self, val) }
    }
    #[fixed_stack_segment]
    fn setMargin(&self, val: int) {
        unsafe { wxStyledTextEvent_SetMargin(self, val) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, val: int) {
        unsafe { wxStyledTextEvent_SetMessage(self, val) }
    }
    #[fixed_stack_segment]
    fn setWParam(&self, val: int) {
        unsafe { wxStyledTextEvent_SetWParam(self, val) }
    }
    #[fixed_stack_segment]
    fn setLParam(&self, val: int) {
        unsafe { wxStyledTextEvent_SetLParam(self, val) }
    }
    #[fixed_stack_segment]
    fn setListType(&self, val: int) {
        unsafe { wxStyledTextEvent_SetListType(self, val) }
    }
    #[fixed_stack_segment]
    fn setX(&self, val: int) {
        unsafe { wxStyledTextEvent_SetX(self, val) }
    }
    #[fixed_stack_segment]
    fn setY(&self, val: int) {
        unsafe { wxStyledTextEvent_SetY(self, val) }
    }
    #[fixed_stack_segment]
    fn setDragText(&self, val: @wxString) {
        unsafe { wxStyledTextEvent_SetDragText(self, val) }
    }
    #[fixed_stack_segment]
    fn setDragAllowMove(&self, val: bool) {
        unsafe { wxStyledTextEvent_SetDragAllowMove(self, val) }
    }
    #[fixed_stack_segment]
    fn setDragResult(&self, val: int) {
        unsafe { wxStyledTextEvent_SetDragResult(self, val) }
    }
}
trait wxGauge95 {
}
trait wxGaugeMSW {
}
trait wxSlider95 {
}
trait wxSliderMSW {
}
trait wxcTreeItemData {
    #[fixed_stack_segment]
    fn new(closure: @wxClosure) -> @wxcTreeItemData {
        unsafe { wxcTreeItemData_Create(closure) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @wxClosure {
        unsafe { wxcTreeItemData_GetClientClosure(self) }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @wxClosure) {
        unsafe { wxcTreeItemData_SetClientClosure(self, closure) }
    }
}
trait wxInputSink {
    #[fixed_stack_segment]
    fn new(input: @wxInputStream, evtHandler: @wxEvtHandler, bufferLen: int) -> @wxInputSink {
        unsafe { wxInputSink_Create(input, evtHandler, bufferLen) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> int {
        unsafe { wxInputSink_GetId(self) }
    }
    #[fixed_stack_segment]
    fn start(&self) {
        unsafe { wxInputSink_Start(self) }
    }
}
trait wxInputSinkEvent {
    #[fixed_stack_segment]
    fn lastError(&self) -> int {
        unsafe { wxInputSinkEvent_LastError(self) }
    }
    #[fixed_stack_segment]
    fn lastRead(&self) -> int {
        unsafe { wxInputSinkEvent_LastRead(self) }
    }
    #[fixed_stack_segment]
    fn lastInput(&self) -> *char {
        unsafe { wxInputSinkEvent_LastInput(self) }
    }
}
trait wxcHtmlEvent {
    #[fixed_stack_segment]
    fn getMouseEvent(&self) -> @wxMouseEvent {
        unsafe { wxcHtmlEvent_GetMouseEvent(self) }
    }
    #[fixed_stack_segment]
    fn getHtmlCell(&self) -> @wxHtmlCell {
        unsafe { wxcHtmlEvent_GetHtmlCell(self) }
    }
    #[fixed_stack_segment]
    fn getHtmlCellId(&self) -> @wxString {
        unsafe { wxcHtmlEvent_GetHtmlCellId(self) }
    }
    #[fixed_stack_segment]
    fn getHref(&self) -> @wxString {
        unsafe { wxcHtmlEvent_GetHref(self) }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @wxString {
        unsafe { wxcHtmlEvent_GetTarget(self) }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self) -> @wxPoint {
        unsafe { wxcHtmlEvent_GetLogicalPosition(self) }
    }
}
trait wxcHtmlWindow {
    #[fixed_stack_segment]
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int, _txt: @wxString) -> @wxcHtmlWindow {
        unsafe { wxcHtmlWindow_Create(_prt, _id, _arg2, _stl, _txt) }
    }
}
trait wxGridCellTextEnterEditor {
    #[fixed_stack_segment]
    fn ctor() -> @wxGridCellTextEnterEditor {
        unsafe { wxGridCellTextEnterEditor_Ctor() }
    }
}
trait wxFileConfig {
    #[fixed_stack_segment]
    fn new(inp: @wxInputStream) -> @wxFileConfig {
        unsafe { wxFileConfig_Create(inp) }
    }
}
