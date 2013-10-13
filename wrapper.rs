use std::libc::*;
use native::*;

struct ELJAppImpl(*u8);
impl ELJApp for ELJAppImpl {}
impl App for ELJAppImpl {}
impl EvtHandler for ELJAppImpl {}
impl Object for ELJAppImpl { fn handle(&self) -> *u8 { **self } }

impl ELJAppImpl {
    #[fixed_stack_segment]
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    #[fixed_stack_segment]
    pub fn newLogTarget() -> @ELJLog {
        unsafe { @ELJLogImpl(ELJApp_CreateLogTarget()) as @ELJLog }
    }
    #[fixed_stack_segment]
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    #[fixed_stack_segment]
    pub fn displaySize() -> @Size {
        unsafe { @wxSize(ELJApp_DisplaySize()) as @Size }
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
    pub fn executeProcess(_cmd: @String, _snc: c_int, _prc: @Process) -> c_int {
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
    pub fn findWindowById(_id: c_int, _prt: @Window) -> *u8 {
        unsafe { ELJApp_FindWindowById(_id, _prt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn findWindowByLabel(_lbl: @String, _prt: @Window) -> @Window {
        unsafe { @wxWindow(ELJApp_FindWindowByLabel(_lbl.handle(), _prt.handle())) as @Window }
    }
    #[fixed_stack_segment]
    pub fn findWindowByName(_lbl: @String, _prt: @Window) -> @Window {
        unsafe { @wxWindow(ELJApp_FindWindowByName(_lbl.handle(), _prt.handle())) as @Window }
    }
    #[fixed_stack_segment]
    pub fn getApp() -> @App {
        unsafe { @wxApp(ELJApp_GetApp()) as @App }
    }
    #[fixed_stack_segment]
    pub fn getAppName() -> @String {
        unsafe { @wxString(ELJApp_GetAppName()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getClassName() -> @String {
        unsafe { @wxString(ELJApp_GetClassName()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getExitOnFrameDelete() -> c_int {
        unsafe { ELJApp_GetExitOnFrameDelete() }
    }
    #[fixed_stack_segment]
    pub fn getOsDescription() -> @String {
        unsafe { @wxString(ELJApp_GetOsDescription()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getOsVersion(_maj: *u8, _min: *u8) -> c_int {
        unsafe { ELJApp_GetOsVersion(_maj, _min) }
    }
    #[fixed_stack_segment]
    pub fn getTopWindow() -> @Window {
        unsafe { @wxWindow(ELJApp_GetTopWindow()) as @Window }
    }
    #[fixed_stack_segment]
    pub fn getUseBestVisual() -> c_int {
        unsafe { ELJApp_GetUseBestVisual() }
    }
    #[fixed_stack_segment]
    pub fn getUserHome(_usr: *u8) -> @String {
        unsafe { @wxString(ELJApp_GetUserHome(_usr)) as @String }
    }
    #[fixed_stack_segment]
    pub fn getUserId() -> @String {
        unsafe { @wxString(ELJApp_GetUserId()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getUserName() -> @String {
        unsafe { @wxString(ELJApp_GetUserName()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getVendorName() -> @String {
        unsafe { @wxString(ELJApp_GetVendorName()) as @String }
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
    pub fn mousePosition() -> @Point {
        unsafe { @wxPoint(ELJApp_MousePosition()) as @Point }
    }
    #[fixed_stack_segment]
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    #[fixed_stack_segment]
    pub fn safeYield(_win: @Window) -> c_int {
        unsafe { ELJApp_SafeYield(_win.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setAppName(name: @String) {
        unsafe { ELJApp_SetAppName(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setClassName(name: @String) {
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
    pub fn setTopWindow(_wnd: @Window) {
        unsafe { ELJApp_SetTopWindow(_wnd.handle()) }
    }
    #[fixed_stack_segment]
    pub fn setUseBestVisual(flag: c_int) {
        unsafe { ELJApp_SetUseBestVisual(flag) }
    }
    #[fixed_stack_segment]
    pub fn setVendorName(name: @String) {
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
    pub fn initializeC(closure: @Closure, _argc: c_int, _argv: *wchar_t) {
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

trait ELJApp : App {
}

struct ELJArtProvImpl(*u8);
impl ELJArtProv for ELJArtProvImpl {}
impl ArtProvider for ELJArtProvImpl {}
impl Object for ELJArtProvImpl { fn handle(&self) -> *u8 { **self } }

impl ELJArtProvImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _clb: *u8) -> @ELJArtProv {
        unsafe { @ELJArtProvImpl(ELJArtProv_Create(_obj, _clb)) as @ELJArtProv }
    }
}

trait ELJArtProv : ArtProvider {
    #[fixed_stack_segment]
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.handle()) }
    }
}

struct ELJClientImpl(*u8);
impl ELJClient for ELJClientImpl {}
impl Client for ELJClientImpl {}
impl ClientBase for ELJClientImpl {}
impl Object for ELJClientImpl { fn handle(&self) -> *u8 { **self } }

impl ELJClientImpl {
}

trait ELJClient : Client {
}

struct ELJCommandImpl(*u8);
impl ELJCommand for ELJCommandImpl {}
impl Command for ELJCommandImpl {}
impl Object for ELJCommandImpl { fn handle(&self) -> *u8 { **self } }

impl ELJCommandImpl {
}

trait ELJCommand : Command {
}

struct ELJConnectionImpl(*u8);
impl ELJConnection for ELJConnectionImpl {}
impl Connection for ELJConnectionImpl {}
impl ConnectionBase for ELJConnectionImpl {}
impl Object for ELJConnectionImpl { fn handle(&self) -> *u8 { **self } }

impl ELJConnectionImpl {
}

trait ELJConnection : Connection {
}

struct ELJDragDataObjectImpl(*u8);
impl ELJDragDataObject for ELJDragDataObjectImpl { fn handle(&self) -> *u8 { **self } }

impl ELJDragDataObjectImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fmt: @String, _func1: *u8, _func2: *u8, _func3: *u8) -> @ELJDragDataObject {
        unsafe { @ELJDragDataObjectImpl(ELJDragDataObject_Create(_obj, _fmt.handle(), _func1, _func2, _func3)) as @ELJDragDataObject }
    }
}

trait ELJDragDataObject {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.handle()) }
    }
}

struct ELJDropTargetImpl(*u8);
impl ELJDropTarget for ELJDropTargetImpl {}
impl DropTarget for ELJDropTargetImpl { fn handle(&self) -> *u8 { **self } }

impl ELJDropTargetImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8) -> @ELJDropTarget {
        unsafe { @ELJDropTargetImpl(ELJDropTarget_Create(_obj)) as @ELJDropTarget }
    }
}

trait ELJDropTarget : DropTarget {
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
impl FileDropTarget for ELJFileDropTargetImpl {}
impl DropTarget for ELJFileDropTargetImpl { fn handle(&self) -> *u8 { **self } }

impl ELJFileDropTargetImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _func: *u8) -> @ELJFileDropTarget {
        unsafe { @ELJFileDropTargetImpl(ELJFileDropTarget_Create(_obj, _func)) as @ELJFileDropTarget }
    }
}

trait ELJFileDropTarget : FileDropTarget {
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
impl GridTableBase for ELJGridTableImpl {}
impl Object for ELJGridTableImpl { fn handle(&self) -> *u8 { **self } }

impl ELJGridTableImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _EifGetNumberRows: *u8, _EifGetNumberCols: *u8, _EifGetValue: *u8, _EifSetValue: *u8, _EifIsEmptyCell: *u8, _EifClear: *u8, _EifInsertRows: *u8, _EifAppendRows: *u8, _EifDeleteRows: *u8, _EifInsertCols: *u8, _EifAppendCols: *u8, _EifDeleteCols: *u8, _EifSetRowLabelValue: *u8, _EifSetColLabelValue: *u8, _EifGetRowLabelValue: *u8, _EifGetColLabelValue: *u8) -> @ELJGridTable {
        unsafe { @ELJGridTableImpl(ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue)) as @ELJGridTable }
    }
}

trait ELJGridTable : GridTableBase {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { ELJGridTable_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getView(&self) -> @View {
        unsafe { @wxView(ELJGridTable_GetView(self.handle())) as @View }
    }
    #[fixed_stack_segment]
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *u8 {
        unsafe { ELJGridTable_SendTableMessage(self.handle(), id, val1, val2) }
    }
}

struct ELJLocaleImpl(*u8);
impl ELJLocale for ELJLocaleImpl {}
impl Locale for ELJLocaleImpl { fn handle(&self) -> *u8 { **self } }

impl ELJLocaleImpl {
}

trait ELJLocale : Locale {
}

struct ELJLogImpl(*u8);
impl ELJLog for ELJLogImpl {}
impl Log for ELJLogImpl { fn handle(&self) -> *u8 { **self } }

impl ELJLogImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8) -> @ELJLog {
        unsafe { @ELJLogImpl(ELJLog_Create(_obj, _fnc)) as @ELJLog }
    }
    #[fixed_stack_segment]
    pub fn getActiveTarget() -> *u8 {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

trait ELJLog : Log {
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
    fn isAllowedTraceMask(&self, mask: @Mask) -> bool {
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
impl ELJMessageParameters for ELJMessageParametersImpl { fn handle(&self) -> *u8 { **self } }

impl ELJMessageParametersImpl {
}

trait ELJMessageParameters {
    fn handle(&self) -> *u8;
    
}

struct ELJPlotCurveImpl(*u8);
impl ELJPlotCurve for ELJPlotCurveImpl {}
impl PlotCurve for ELJPlotCurveImpl {}
impl Object for ELJPlotCurveImpl { fn handle(&self) -> *u8 { **self } }

impl ELJPlotCurveImpl {
}

trait ELJPlotCurve : PlotCurve {
}

struct ELJPreviewControlBarImpl(*u8);
impl ELJPreviewControlBar for ELJPreviewControlBarImpl {}
impl PreviewControlBar for ELJPreviewControlBarImpl {}
impl Panel for ELJPreviewControlBarImpl {}
impl Window for ELJPreviewControlBarImpl {}
impl EvtHandler for ELJPreviewControlBarImpl {}
impl Object for ELJPreviewControlBarImpl { fn handle(&self) -> *u8 { **self } }

impl ELJPreviewControlBarImpl {
    #[fixed_stack_segment]
    pub fn new(preview: *u8, buttons: c_int, parent: @Window, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewControlBar {
        unsafe { @ELJPreviewControlBarImpl(ELJPreviewControlBar_Create(preview, buttons, parent.handle(), title, x, y, w, h, style)) as @ELJPreviewControlBar }
    }
}

trait ELJPreviewControlBar : PreviewControlBar {
}

struct ELJPreviewFrameImpl(*u8);
impl ELJPreviewFrame for ELJPreviewFrameImpl {}
impl PreviewFrame for ELJPreviewFrameImpl {}
impl Frame for ELJPreviewFrameImpl {}
impl TopLevelWindow for ELJPreviewFrameImpl {}
impl Window for ELJPreviewFrameImpl {}
impl EvtHandler for ELJPreviewFrameImpl {}
impl Object for ELJPreviewFrameImpl { fn handle(&self) -> *u8 { **self } }

impl ELJPreviewFrameImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _init: *u8, _create_canvas: *u8, _create_toolbar: *u8, preview: *u8, parent: @Window, title: *u8, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ELJPreviewFrame {
        unsafe { @ELJPreviewFrameImpl(ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.handle(), title, x, y, w, h, style)) as @ELJPreviewFrame }
    }
}

trait ELJPreviewFrame : PreviewFrame {
    #[fixed_stack_segment]
    fn getControlBar(&self) -> *u8 {
        unsafe { ELJPreviewFrame_GetControlBar(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPreviewCanvas(&self) -> @PreviewCanvas {
        unsafe { @wxPreviewCanvas(ELJPreviewFrame_GetPreviewCanvas(self.handle())) as @PreviewCanvas }
    }
    #[fixed_stack_segment]
    fn getPrintPreview(&self) -> @PrintPreview {
        unsafe { @wxPrintPreview(ELJPreviewFrame_GetPrintPreview(self.handle())) as @PrintPreview }
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
    fn setPreviewCanvas(&self, obj: @PreviewCanvas) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.handle(), obj.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintPreview(&self, obj: @PrintPreview) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.handle(), obj.handle()) }
    }
}

struct ELJServerImpl(*u8);
impl ELJServer for ELJServerImpl {}
impl Server for ELJServerImpl {}
impl ServerBase for ELJServerImpl {}
impl Object for ELJServerImpl { fn handle(&self) -> *u8 { **self } }

impl ELJServerImpl {
}

trait ELJServer : Server {
}

struct ELJTextDropTargetImpl(*u8);
impl ELJTextDropTarget for ELJTextDropTargetImpl {}
impl TextDropTarget for ELJTextDropTargetImpl {}
impl DropTarget for ELJTextDropTargetImpl { fn handle(&self) -> *u8 { **self } }

impl ELJTextDropTargetImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _func: *u8) -> @ELJTextDropTarget {
        unsafe { @ELJTextDropTargetImpl(ELJTextDropTarget_Create(_obj, _func)) as @ELJTextDropTarget }
    }
}

trait ELJTextDropTarget : TextDropTarget {
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
impl TextValidator for ELJTextValidatorImpl {}
impl Validator for ELJTextValidatorImpl {}
impl EvtHandler for ELJTextValidatorImpl {}
impl Object for ELJTextValidatorImpl { fn handle(&self) -> *u8 { **self } }

impl ELJTextValidatorImpl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8, _txt: *wchar_t, _stl: c_int) -> @ELJTextValidator {
        unsafe { @ELJTextValidatorImpl(ELJTextValidator_Create(_obj, _fnc, _txt, _stl)) as @ELJTextValidator }
    }
}

trait ELJTextValidator : TextValidator {
}

struct cbAntiflickerPluginImpl(*u8);
impl cbAntiflickerPlugin for cbAntiflickerPluginImpl {}
impl cbPluginBase for cbAntiflickerPluginImpl {}
impl EvtHandler for cbAntiflickerPluginImpl {}
impl Object for cbAntiflickerPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbAntiflickerPluginImpl {
}

trait cbAntiflickerPlugin : cbPluginBase {
}

struct cbBarDragPluginImpl(*u8);
impl cbBarDragPlugin for cbBarDragPluginImpl {}
impl cbPluginBase for cbBarDragPluginImpl {}
impl EvtHandler for cbBarDragPluginImpl {}
impl Object for cbBarDragPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbBarDragPluginImpl {
}

trait cbBarDragPlugin : cbPluginBase {
}

struct cbBarHintsPluginImpl(*u8);
impl cbBarHintsPlugin for cbBarHintsPluginImpl {}
impl cbPluginBase for cbBarHintsPluginImpl {}
impl EvtHandler for cbBarHintsPluginImpl {}
impl Object for cbBarHintsPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbBarHintsPluginImpl {
}

trait cbBarHintsPlugin : cbPluginBase {
}

struct cbBarInfoImpl(*u8);
impl cbBarInfo for cbBarInfoImpl {}
impl Object for cbBarInfoImpl { fn handle(&self) -> *u8 { **self } }

impl cbBarInfoImpl {
}

trait cbBarInfo : Object {
}

struct cbBarSpyImpl(*u8);
impl cbBarSpy for cbBarSpyImpl {}
impl EvtHandler for cbBarSpyImpl {}
impl Object for cbBarSpyImpl { fn handle(&self) -> *u8 { **self } }

impl cbBarSpyImpl {
}

trait cbBarSpy : EvtHandler {
}

struct cbCloseBoxImpl(*u8);
impl cbCloseBox for cbCloseBoxImpl {}
impl cbMiniButton for cbCloseBoxImpl {}
impl Object for cbCloseBoxImpl { fn handle(&self) -> *u8 { **self } }

impl cbCloseBoxImpl {
}

trait cbCloseBox : cbMiniButton {
}

struct cbCollapseBoxImpl(*u8);
impl cbCollapseBox for cbCollapseBoxImpl {}
impl cbMiniButton for cbCollapseBoxImpl {}
impl Object for cbCollapseBoxImpl { fn handle(&self) -> *u8 { **self } }

impl cbCollapseBoxImpl {
}

trait cbCollapseBox : cbMiniButton {
}

struct cbCommonPanePropertiesImpl(*u8);
impl cbCommonPaneProperties for cbCommonPanePropertiesImpl {}
impl Object for cbCommonPanePropertiesImpl { fn handle(&self) -> *u8 { **self } }

impl cbCommonPanePropertiesImpl {
}

trait cbCommonPaneProperties : Object {
}

struct cbCustomizeBarEventImpl(*u8);
impl cbCustomizeBarEvent for cbCustomizeBarEventImpl {}
impl cbPluginEvent for cbCustomizeBarEventImpl {}
impl Event for cbCustomizeBarEventImpl {}
impl Object for cbCustomizeBarEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeBarEventImpl {
}

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
impl Event for cbCustomizeLayoutEventImpl {}
impl Object for cbCustomizeLayoutEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbCustomizeLayoutEventImpl {
}

trait cbCustomizeLayoutEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn clickPos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbCustomizeLayoutEvent_ClickPos(self.handle(), _x, _y) }
    }
}

struct cbDimHandlerBaseImpl(*u8);
impl cbDimHandlerBase for cbDimHandlerBaseImpl {}
impl Object for cbDimHandlerBaseImpl { fn handle(&self) -> *u8 { **self } }

impl cbDimHandlerBaseImpl {
}

trait cbDimHandlerBase : Object {
}

struct cbDimInfoImpl(*u8);
impl cbDimInfo for cbDimInfoImpl {}
impl Object for cbDimInfoImpl { fn handle(&self) -> *u8 { **self } }

impl cbDimInfoImpl {
}

trait cbDimInfo : Object {
}

struct cbDockBoxImpl(*u8);
impl cbDockBox for cbDockBoxImpl {}
impl cbMiniButton for cbDockBoxImpl {}
impl Object for cbDockBoxImpl { fn handle(&self) -> *u8 { **self } }

impl cbDockBoxImpl {
}

trait cbDockBox : cbMiniButton {
}

struct cbDockPaneImpl(*u8);
impl cbDockPane for cbDockPaneImpl {}
impl Object for cbDockPaneImpl { fn handle(&self) -> *u8 { **self } }

impl cbDockPaneImpl {
}

trait cbDockPane : Object {
}

struct cbDrawBarDecorEventImpl(*u8);
impl cbDrawBarDecorEvent for cbDrawBarDecorEventImpl {}
impl cbPluginEvent for cbDrawBarDecorEventImpl {}
impl Event for cbDrawBarDecorEventImpl {}
impl Object for cbDrawBarDecorEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarDecorEventImpl {
}

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
impl Event for cbDrawBarHandlesEventImpl {}
impl Object for cbDrawBarHandlesEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawBarHandlesEventImpl {
}

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
impl Event for cbDrawHintRectEventImpl {}
impl Object for cbDrawHintRectEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawHintRectEventImpl {
}

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
impl Event for cbDrawPaneBkGroundEventImpl {}
impl Object for cbDrawPaneBkGroundEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneBkGroundEventImpl {
}

trait cbDrawPaneBkGroundEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawPaneBkGroundEvent_Dc(self.handle()) }
    }
}

struct cbDrawPaneDecorEventImpl(*u8);
impl cbDrawPaneDecorEvent for cbDrawPaneDecorEventImpl {}
impl cbPluginEvent for cbDrawPaneDecorEventImpl {}
impl Event for cbDrawPaneDecorEventImpl {}
impl Object for cbDrawPaneDecorEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawPaneDecorEventImpl {
}

trait cbDrawPaneDecorEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn dc(&self) -> *u8 {
        unsafe { cbDrawPaneDecorEvent_Dc(self.handle()) }
    }
}

struct cbDrawRowBkGroundEventImpl(*u8);
impl cbDrawRowBkGroundEvent for cbDrawRowBkGroundEventImpl {}
impl cbPluginEvent for cbDrawRowBkGroundEventImpl {}
impl Event for cbDrawRowBkGroundEventImpl {}
impl Object for cbDrawRowBkGroundEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowBkGroundEventImpl {
}

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
impl Event for cbDrawRowDecorEventImpl {}
impl Object for cbDrawRowDecorEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowDecorEventImpl {
}

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
impl Event for cbDrawRowHandlesEventImpl {}
impl Object for cbDrawRowHandlesEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbDrawRowHandlesEventImpl {
}

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
impl Object for cbDynToolBarDimHandlerImpl { fn handle(&self) -> *u8 { **self } }

impl cbDynToolBarDimHandlerImpl {
}

trait cbDynToolBarDimHandler : cbDimHandlerBase {
}

struct cbFinishDrawInAreaEventImpl(*u8);
impl cbFinishDrawInAreaEvent for cbFinishDrawInAreaEventImpl {}
impl cbPluginEvent for cbFinishDrawInAreaEventImpl {}
impl Event for cbFinishDrawInAreaEventImpl {}
impl Object for cbFinishDrawInAreaEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbFinishDrawInAreaEventImpl {
}

trait cbFinishDrawInAreaEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbFinishDrawInAreaEvent_Area(self.handle(), _x, _y, _w, _h) }
    }
}

struct cbFloatedBarWindowImpl(*u8);
impl cbFloatedBarWindow for cbFloatedBarWindowImpl {}
impl ToolWindow for cbFloatedBarWindowImpl {}
impl Frame for cbFloatedBarWindowImpl {}
impl TopLevelWindow for cbFloatedBarWindowImpl {}
impl Window for cbFloatedBarWindowImpl {}
impl EvtHandler for cbFloatedBarWindowImpl {}
impl Object for cbFloatedBarWindowImpl { fn handle(&self) -> *u8 { **self } }

impl cbFloatedBarWindowImpl {
}

trait cbFloatedBarWindow : ToolWindow {
}

struct cbGCUpdatesMgrImpl(*u8);
impl cbGCUpdatesMgr for cbGCUpdatesMgrImpl {}
impl cbSimpleUpdatesMgr for cbGCUpdatesMgrImpl {}
impl cbUpdatesManagerBase for cbGCUpdatesMgrImpl {}
impl Object for cbGCUpdatesMgrImpl { fn handle(&self) -> *u8 { **self } }

impl cbGCUpdatesMgrImpl {
}

trait cbGCUpdatesMgr : cbSimpleUpdatesMgr {
}

struct cbHintAnimationPluginImpl(*u8);
impl cbHintAnimationPlugin for cbHintAnimationPluginImpl {}
impl cbPluginBase for cbHintAnimationPluginImpl {}
impl EvtHandler for cbHintAnimationPluginImpl {}
impl Object for cbHintAnimationPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbHintAnimationPluginImpl {
}

trait cbHintAnimationPlugin : cbPluginBase {
}

struct cbInsertBarEventImpl(*u8);
impl cbInsertBarEvent for cbInsertBarEventImpl {}
impl cbPluginEvent for cbInsertBarEventImpl {}
impl Event for cbInsertBarEventImpl {}
impl Object for cbInsertBarEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbInsertBarEventImpl {
}

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
impl Event for cbLayoutRowEventImpl {}
impl Object for cbLayoutRowEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbLayoutRowEventImpl {
}

trait cbLayoutRowEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn row(&self) -> *u8 {
        unsafe { cbLayoutRowEvent_Row(self.handle()) }
    }
}

struct cbLeftDClickEventImpl(*u8);
impl cbLeftDClickEvent for cbLeftDClickEventImpl {}
impl cbPluginEvent for cbLeftDClickEventImpl {}
impl Event for cbLeftDClickEventImpl {}
impl Object for cbLeftDClickEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbLeftDClickEventImpl {
}

trait cbLeftDClickEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftDClickEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbLeftDownEventImpl(*u8);
impl cbLeftDownEvent for cbLeftDownEventImpl {}
impl cbPluginEvent for cbLeftDownEventImpl {}
impl Event for cbLeftDownEventImpl {}
impl Object for cbLeftDownEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbLeftDownEventImpl {
}

trait cbLeftDownEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftDownEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbLeftUpEventImpl(*u8);
impl cbLeftUpEvent for cbLeftUpEventImpl {}
impl cbPluginEvent for cbLeftUpEventImpl {}
impl Event for cbLeftUpEventImpl {}
impl Object for cbLeftUpEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbLeftUpEventImpl {
}

trait cbLeftUpEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbLeftUpEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbMiniButtonImpl(*u8);
impl cbMiniButton for cbMiniButtonImpl {}
impl Object for cbMiniButtonImpl { fn handle(&self) -> *u8 { **self } }

impl cbMiniButtonImpl {
}

trait cbMiniButton : Object {
}

struct cbMotionEventImpl(*u8);
impl cbMotionEvent for cbMotionEventImpl {}
impl cbPluginEvent for cbMotionEventImpl {}
impl Event for cbMotionEventImpl {}
impl Object for cbMotionEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbMotionEventImpl {
}

trait cbMotionEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbMotionEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbPaneDrawPluginImpl(*u8);
impl cbPaneDrawPlugin for cbPaneDrawPluginImpl {}
impl cbPluginBase for cbPaneDrawPluginImpl {}
impl EvtHandler for cbPaneDrawPluginImpl {}
impl Object for cbPaneDrawPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbPaneDrawPluginImpl {
}

trait cbPaneDrawPlugin : cbPluginBase {
}

struct cbPluginBaseImpl(*u8);
impl cbPluginBase for cbPluginBaseImpl {}
impl EvtHandler for cbPluginBaseImpl {}
impl Object for cbPluginBaseImpl { fn handle(&self) -> *u8 { **self } }

impl cbPluginBaseImpl {
}

trait cbPluginBase : EvtHandler {
}

struct cbPluginEventImpl(*u8);
impl cbPluginEvent for cbPluginEventImpl {}
impl Event for cbPluginEventImpl {}
impl Object for cbPluginEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbPluginEventImpl {
}

trait cbPluginEvent : Event {
    #[fixed_stack_segment]
    fn pane(&self) -> *u8 {
        unsafe { cbPluginEvent_Pane(self.handle()) }
    }
}

struct cbRemoveBarEventImpl(*u8);
impl cbRemoveBarEvent for cbRemoveBarEventImpl {}
impl cbPluginEvent for cbRemoveBarEventImpl {}
impl Event for cbRemoveBarEventImpl {}
impl Object for cbRemoveBarEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbRemoveBarEventImpl {
}

trait cbRemoveBarEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn bar(&self) -> *u8 {
        unsafe { cbRemoveBarEvent_Bar(self.handle()) }
    }
}

struct cbResizeBarEventImpl(*u8);
impl cbResizeBarEvent for cbResizeBarEventImpl {}
impl cbPluginEvent for cbResizeBarEventImpl {}
impl Event for cbResizeBarEventImpl {}
impl Object for cbResizeBarEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbResizeBarEventImpl {
}

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
impl Event for cbResizeRowEventImpl {}
impl Object for cbResizeRowEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbResizeRowEventImpl {
}

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
impl Event for cbRightDownEventImpl {}
impl Object for cbRightDownEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbRightDownEventImpl {
}

trait cbRightDownEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbRightDownEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbRightUpEventImpl(*u8);
impl cbRightUpEvent for cbRightUpEventImpl {}
impl cbPluginEvent for cbRightUpEventImpl {}
impl Event for cbRightUpEventImpl {}
impl Object for cbRightUpEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbRightUpEventImpl {
}

trait cbRightUpEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn pos(&self, _x: *c_int, _y: *c_int) {
        unsafe { cbRightUpEvent_Pos(self.handle(), _x, _y) }
    }
}

struct cbRowDragPluginImpl(*u8);
impl cbRowDragPlugin for cbRowDragPluginImpl {}
impl cbPluginBase for cbRowDragPluginImpl {}
impl EvtHandler for cbRowDragPluginImpl {}
impl Object for cbRowDragPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbRowDragPluginImpl {
}

trait cbRowDragPlugin : cbPluginBase {
}

struct cbRowInfoImpl(*u8);
impl cbRowInfo for cbRowInfoImpl {}
impl Object for cbRowInfoImpl { fn handle(&self) -> *u8 { **self } }

impl cbRowInfoImpl {
}

trait cbRowInfo : Object {
}

struct cbRowLayoutPluginImpl(*u8);
impl cbRowLayoutPlugin for cbRowLayoutPluginImpl {}
impl cbPluginBase for cbRowLayoutPluginImpl {}
impl EvtHandler for cbRowLayoutPluginImpl {}
impl Object for cbRowLayoutPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbRowLayoutPluginImpl {
}

trait cbRowLayoutPlugin : cbPluginBase {
}

struct cbSimpleCustomizationPluginImpl(*u8);
impl cbSimpleCustomizationPlugin for cbSimpleCustomizationPluginImpl {}
impl cbPluginBase for cbSimpleCustomizationPluginImpl {}
impl EvtHandler for cbSimpleCustomizationPluginImpl {}
impl Object for cbSimpleCustomizationPluginImpl { fn handle(&self) -> *u8 { **self } }

impl cbSimpleCustomizationPluginImpl {
}

trait cbSimpleCustomizationPlugin : cbPluginBase {
}

struct cbSimpleUpdatesMgrImpl(*u8);
impl cbSimpleUpdatesMgr for cbSimpleUpdatesMgrImpl {}
impl cbUpdatesManagerBase for cbSimpleUpdatesMgrImpl {}
impl Object for cbSimpleUpdatesMgrImpl { fn handle(&self) -> *u8 { **self } }

impl cbSimpleUpdatesMgrImpl {
}

trait cbSimpleUpdatesMgr : cbUpdatesManagerBase {
}

struct cbSizeBarWndEventImpl(*u8);
impl cbSizeBarWndEvent for cbSizeBarWndEventImpl {}
impl cbPluginEvent for cbSizeBarWndEventImpl {}
impl Event for cbSizeBarWndEventImpl {}
impl Object for cbSizeBarWndEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbSizeBarWndEventImpl {
}

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
impl Event for cbStartBarDraggingEventImpl {}
impl Object for cbStartBarDraggingEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbStartBarDraggingEventImpl {
}

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
impl Event for cbStartDrawInAreaEventImpl {}
impl Object for cbStartDrawInAreaEventImpl { fn handle(&self) -> *u8 { **self } }

impl cbStartDrawInAreaEventImpl {
}

trait cbStartDrawInAreaEvent : cbPluginEvent {
    #[fixed_stack_segment]
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { cbStartDrawInAreaEvent_Area(self.handle(), _x, _y, _w, _h) }
    }
}

struct cbUpdatesManagerBaseImpl(*u8);
impl cbUpdatesManagerBase for cbUpdatesManagerBaseImpl {}
impl Object for cbUpdatesManagerBaseImpl { fn handle(&self) -> *u8 { **self } }

impl cbUpdatesManagerBaseImpl {
}

trait cbUpdatesManagerBase : Object {
}

struct wxAcceleratorEntry(*u8);
impl AcceleratorEntry for wxAcceleratorEntry { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorEntry {
    #[fixed_stack_segment]
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> @AcceleratorEntry {
        unsafe { @wxAcceleratorEntry(wxAcceleratorEntry_Create(flags, keyCode, cmd)) as @AcceleratorEntry }
    }
}

trait AcceleratorEntry {
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
impl AcceleratorTable for wxAcceleratorTable { fn handle(&self) -> *u8 { **self } }

impl wxAcceleratorTable {
    #[fixed_stack_segment]
    pub fn new(n: c_int, entries: *u8) -> @AcceleratorTable {
        unsafe { @wxAcceleratorTable(wxAcceleratorTable_Create(n, entries)) as @AcceleratorTable }
    }
}

trait AcceleratorTable {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.handle()) }
    }
}

struct wxActivateEvent(*u8);
impl ActivateEvent for wxActivateEvent {}
impl Event for wxActivateEvent {}
impl Object for wxActivateEvent { fn handle(&self) -> *u8 { **self } }

impl wxActivateEvent {
}

trait ActivateEvent : Event {
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
impl App for wxApp {}
impl EvtHandler for wxApp {}
impl Object for wxApp { fn handle(&self) -> *u8 { **self } }

impl wxApp {
}

trait App : EvtHandler {
}

struct wxArray(*u8);
impl Array for wxArray { fn handle(&self) -> *u8 { **self } }

impl wxArray {
}

trait Array {
    fn handle(&self) -> *u8;
    
}

struct wxArrayString(*u8);
impl ArrayString for wxArrayString {}
impl Array for wxArrayString { fn handle(&self) -> *u8 { **self } }

impl wxArrayString {
}

trait ArrayString : Array {
}

struct wxArtProvider(*u8);
impl ArtProvider for wxArtProvider {}
impl Object for wxArtProvider { fn handle(&self) -> *u8 { **self } }

impl wxArtProvider {
}

trait ArtProvider : Object {
}

struct wxAutoBufferedPaintDC(*u8);
impl AutoBufferedPaintDC for wxAutoBufferedPaintDC {}
impl DC for wxAutoBufferedPaintDC {}
impl Object for wxAutoBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxAutoBufferedPaintDC {
    #[fixed_stack_segment]
    pub fn new(window: @Window) -> @AutoBufferedPaintDC {
        unsafe { @wxAutoBufferedPaintDC(wxAutoBufferedPaintDC_Create(window.handle())) as @AutoBufferedPaintDC }
    }
}

trait AutoBufferedPaintDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxAutoBufferedPaintDC_Delete(self.handle()) }
    }
}

struct wxAutomationObject(*u8);
impl AutomationObject for wxAutomationObject {}
impl Object for wxAutomationObject { fn handle(&self) -> *u8 { **self } }

impl wxAutomationObject {
}

trait AutomationObject : Object {
}

struct wxBitmap(*u8);
impl Bitmap for wxBitmap {}
impl GDIObject for wxBitmap {}
impl Object for wxBitmap { fn handle(&self) -> *u8 { **self } }

impl wxBitmap {
    #[fixed_stack_segment]
    pub fn addHandler(handler: @EvtHandler) {
        unsafe { wxBitmap_AddHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    #[fixed_stack_segment]
    pub fn new(_data: *u8, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> @Bitmap {
        unsafe { @wxBitmap(wxBitmap_Create(_data, _type, _width, _height, _depth)) as @Bitmap }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @Bitmap {
        unsafe { @wxBitmap(wxBitmap_CreateDefault()) as @Bitmap }
    }
    #[fixed_stack_segment]
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> @Bitmap {
        unsafe { @wxBitmap(wxBitmap_CreateEmpty(_width, _height, _depth)) as @Bitmap }
    }
    #[fixed_stack_segment]
    pub fn newLoad(name: @String, type_: c_int) -> @Bitmap {
        unsafe { @wxBitmap(wxBitmap_CreateLoad(name.handle(), type_)) as @Bitmap }
    }
    #[fixed_stack_segment]
    pub fn findHandlerByName(name: @String) -> *u8 {
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
    pub fn insertHandler(handler: @EvtHandler) {
        unsafe { wxBitmap_InsertHandler(handler.handle()) }
    }
    #[fixed_stack_segment]
    pub fn removeHandler(name: @String) -> bool {
        unsafe { wxBitmap_RemoveHandler(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newFromImage(image: @Image, depth: c_int) -> @Bitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromImage(image.handle(), depth)) as @Bitmap }
    }
}

trait Bitmap : GDIObject {
    #[fixed_stack_segment]
    fn newFromXPM(&self) -> @Bitmap {
        unsafe { @wxBitmap(wxBitmap_CreateFromXPM(self.handle())) as @Bitmap }
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
    fn getMask(&self) -> @Mask {
        unsafe { @wxMask(wxBitmap_GetMask(self.handle())) as @Mask }
    }
    #[fixed_stack_segment]
    fn getSubBitmap(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: @Bitmap) {
        unsafe { wxBitmap_GetSubBitmap(self.handle(), x, y, w, h, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWidth(&self) -> c_int {
        unsafe { wxBitmap_GetWidth(self.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, name: @String, type_: c_int) -> c_int {
        unsafe { wxBitmap_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBitmap_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @String, type_: c_int, cmap: @Palette) -> c_int {
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
    fn setMask(&self, mask: @Mask) {
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
impl BitmapButton for wxBitmapButton {}
impl Button for wxBitmapButton {}
impl Control for wxBitmapButton {}
impl Window for wxBitmapButton {}
impl EvtHandler for wxBitmapButton {}
impl Object for wxBitmapButton { fn handle(&self) -> *u8 { **self } }

impl wxBitmapButton {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _bmp: @Bitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @BitmapButton {
        unsafe { @wxBitmapButton(wxBitmapButton_Create(_prt.handle(), _id, _bmp.handle(), _lft, _top, _wdt, _hgt, _stl)) as @BitmapButton }
    }
}

trait BitmapButton : Button {
    #[fixed_stack_segment]
    fn getBitmapDisabled(&self, _ref: @Bitmap) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapFocus(&self, _ref: @Bitmap) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapLabel(&self, _ref: @Bitmap) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmapSelected(&self, _ref: @Bitmap) {
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
    fn setBitmapDisabled(&self, disabled: @Bitmap) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.handle(), disabled.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapFocus(&self, focus: @Bitmap) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.handle(), focus.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapLabel(&self, bitmap: @Bitmap) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmapSelected(&self, sel: @Bitmap) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.handle(), sel.handle()) }
    }
    #[fixed_stack_segment]
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.handle(), x, y) }
    }
}

struct wxBitmapToggleButton(*u8);
impl BitmapToggleButton for wxBitmapToggleButton {}
impl ToggleButton for wxBitmapToggleButton {}
impl Control for wxBitmapToggleButton {}
impl Window for wxBitmapToggleButton {}
impl EvtHandler for wxBitmapToggleButton {}
impl Object for wxBitmapToggleButton { fn handle(&self) -> *u8 { **self } }

impl wxBitmapToggleButton {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, id: c_int, _bmp: @Bitmap, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @BitmapToggleButton {
        unsafe { @wxBitmapToggleButton(wxBitmapToggleButton_Create(parent.handle(), id, _bmp.handle(), x, y, w, h, style)) as @BitmapToggleButton }
    }
}

trait BitmapToggleButton : ToggleButton {
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
    fn setBitmapLabel(&self, _bmp: @Bitmap) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.handle(), _bmp.handle()) }
    }
}

struct wxBitmapDataObject(*u8);
impl BitmapDataObject for wxBitmapDataObject {}
impl DataObjectSimple for wxBitmapDataObject {}
impl DataObject for wxBitmapDataObject { fn handle(&self) -> *u8 { **self } }

impl wxBitmapDataObject {
}

trait BitmapDataObject : DataObjectSimple {
}

struct wxBitmapHandler(*u8);
impl BitmapHandler for wxBitmapHandler {}
impl Object for wxBitmapHandler { fn handle(&self) -> *u8 { **self } }

impl wxBitmapHandler {
}

trait BitmapHandler : Object {
}

struct wxBoxSizer(*u8);
impl BoxSizer for wxBoxSizer {}
impl Sizer for wxBoxSizer {}
impl Object for wxBoxSizer { fn handle(&self) -> *u8 { **self } }

impl wxBoxSizer {
    #[fixed_stack_segment]
    pub fn new(orient: c_int) -> @BoxSizer {
        unsafe { @wxBoxSizer(wxBoxSizer_Create(orient)) as @BoxSizer }
    }
}

trait BoxSizer : Sizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @Size {
        unsafe { @wxSize(wxBoxSizer_CalcMin(self.handle())) as @Size }
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
impl Brush for wxBrush {}
impl GDIObject for wxBrush {}
impl Object for wxBrush { fn handle(&self) -> *u8 { **self } }

impl wxBrush {
    #[fixed_stack_segment]
    pub fn newDefault() -> @Brush {
        unsafe { @wxBrush(wxBrush_CreateDefault()) as @Brush }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(bitmap: @Bitmap) -> @Brush {
        unsafe { @wxBrush(wxBrush_CreateFromBitmap(bitmap.handle())) as @Brush }
    }
    #[fixed_stack_segment]
    pub fn newFromColour(col: @Colour, style: c_int) -> @Brush {
        unsafe { @wxBrush(wxBrush_CreateFromColour(col.handle(), style)) as @Brush }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @Brush {
        unsafe { @wxBrush(wxBrush_CreateFromStock(id)) as @Brush }
    }
}

trait Brush : GDIObject {
    #[fixed_stack_segment]
    fn assign(&self, brush: @Brush) {
        unsafe { wxBrush_Assign(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBrush_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @Colour) {
        unsafe { wxBrush_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStipple(&self, _ref: @Bitmap) {
        unsafe { wxBrush_GetStipple(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isEqual(&self, brush: @Brush) -> bool {
        unsafe { wxBrush_IsEqual(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxBrush_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, col: @Colour) {
        unsafe { wxBrush_SetColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t) {
        unsafe { wxBrush_SetColourSingle(self.handle(), r, g, b) }
    }
    #[fixed_stack_segment]
    fn setStipple(&self, stipple: @Bitmap) {
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
impl BrushList for wxBrushList {}
impl List for wxBrushList {}
impl Object for wxBrushList { fn handle(&self) -> *u8 { **self } }

impl wxBrushList {
}

trait BrushList : List {
}

struct wxBufferedDC(*u8);
impl BufferedDC for wxBufferedDC {}
impl DC for wxBufferedDC {}
impl Object for wxBufferedDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedDC {
    #[fixed_stack_segment]
    pub fn newByDCAndSize(dc: @DC, width: c_int, hight: c_int, style: c_int) -> @BufferedDC {
        unsafe { @wxBufferedDC(wxBufferedDC_CreateByDCAndSize(dc.handle(), width, hight, style)) as @BufferedDC }
    }
    #[fixed_stack_segment]
    pub fn newByDCAndBitmap(dc: @DC, bitmap: @Bitmap, style: c_int) -> @BufferedDC {
        unsafe { @wxBufferedDC(wxBufferedDC_CreateByDCAndBitmap(dc.handle(), bitmap.handle(), style)) as @BufferedDC }
    }
}

trait BufferedDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedDC_Delete(self.handle()) }
    }
}

struct wxBufferedPaintDC(*u8);
impl BufferedPaintDC for wxBufferedPaintDC {}
impl DC for wxBufferedPaintDC {}
impl Object for wxBufferedPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxBufferedPaintDC {
    #[fixed_stack_segment]
    pub fn new(window: @Window, style: c_int) -> @BufferedPaintDC {
        unsafe { @wxBufferedPaintDC(wxBufferedPaintDC_Create(window.handle(), style)) as @BufferedPaintDC }
    }
    #[fixed_stack_segment]
    pub fn newWithBitmap(window: @Window, bitmap: @Bitmap, style: c_int) -> @BufferedPaintDC {
        unsafe { @wxBufferedPaintDC(wxBufferedPaintDC_CreateWithBitmap(window.handle(), bitmap.handle(), style)) as @BufferedPaintDC }
    }
}

trait BufferedPaintDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBufferedPaintDC_Delete(self.handle()) }
    }
}

struct wxBufferedInputStream(*u8);
impl BufferedInputStream for wxBufferedInputStream {}
impl FilterInputStream for wxBufferedInputStream {}
impl InputStream for wxBufferedInputStream {}
impl StreamBase for wxBufferedInputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedInputStream {
}

trait BufferedInputStream : FilterInputStream {
}

struct wxBufferedOutputStream(*u8);
impl BufferedOutputStream for wxBufferedOutputStream {}
impl FilterOutputStream for wxBufferedOutputStream {}
impl OutputStream for wxBufferedOutputStream {}
impl StreamBase for wxBufferedOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxBufferedOutputStream {
}

trait BufferedOutputStream : FilterOutputStream {
}

struct wxBusyCursor(*u8);
impl BusyCursor for wxBusyCursor { fn handle(&self) -> *u8 { **self } }

impl wxBusyCursor {
    #[fixed_stack_segment]
    pub fn new() -> @BusyCursor {
        unsafe { @wxBusyCursor(wxBusyCursor_Create()) as @BusyCursor }
    }
}

trait BusyCursor {
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
impl BusyInfo for wxBusyInfo { fn handle(&self) -> *u8 { **self } }

impl wxBusyInfo {
    #[fixed_stack_segment]
    pub fn new(_txt: @String) -> @BusyInfo {
        unsafe { @wxBusyInfo(wxBusyInfo_Create(_txt.handle())) as @BusyInfo }
    }
}

trait BusyInfo {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.handle()) }
    }
}

struct wxButton(*u8);
impl Button for wxButton {}
impl Control for wxButton {}
impl Window for wxButton {}
impl EvtHandler for wxButton {}
impl Object for wxButton { fn handle(&self) -> *u8 { **self } }

impl wxButton {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Button {
        unsafe { @wxButton(wxButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @Button }
    }
}

trait Button : Control {
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @Colour) -> c_int {
        unsafe { wxButton_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.handle()) }
    }
}

struct wxCSConv(*u8);
impl CSConv for wxCSConv {}
impl MBConv for wxCSConv { fn handle(&self) -> *u8 { **self } }

impl wxCSConv {
}

trait CSConv : MBConv {
}

struct wxCalculateLayoutEvent(*u8);
impl CalculateLayoutEvent for wxCalculateLayoutEvent {}
impl Event for wxCalculateLayoutEvent {}
impl Object for wxCalculateLayoutEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalculateLayoutEvent {
    #[fixed_stack_segment]
    pub fn new(id: c_int) -> @CalculateLayoutEvent {
        unsafe { @wxCalculateLayoutEvent(wxCalculateLayoutEvent_Create(id)) as @CalculateLayoutEvent }
    }
}

trait CalculateLayoutEvent : Event {
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @Rect {
        unsafe { @wxRect(wxCalculateLayoutEvent_GetRect(self.handle())) as @Rect }
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
impl CalendarCtrl for wxCalendarCtrl {}
impl Control for wxCalendarCtrl {}
impl Window for wxCalendarCtrl {}
impl EvtHandler for wxCalendarCtrl {}
impl Object for wxCalendarCtrl { fn handle(&self) -> *u8 { **self } }

impl wxCalendarCtrl {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _dat: @DateTime, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @CalendarCtrl {
        unsafe { @wxCalendarCtrl(wxCalendarCtrl_Create(_prt.handle(), _id, _dat.handle(), _lft, _top, _wdt, _hgt, _stl)) as @CalendarCtrl }
    }
}

trait CalendarCtrl : Control {
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
    fn getHeaderColourBg(&self, _ref: @Colour) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHeaderColourFg(&self, _ref: @Colour) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourBg(&self, _ref: @Colour) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHighlightColourFg(&self, _ref: @Colour) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourBg(&self, _ref: @Colour) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getHolidayColourFg(&self, _ref: @Colour) {
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
impl CalendarDateAttr for wxCalendarDateAttr { fn handle(&self) -> *u8 { **self } }

impl wxCalendarDateAttr {
    #[fixed_stack_segment]
    pub fn new(_ctxt: *u8, _cbck: *u8, _cbrd: *u8, _fnt: *u8, _brd: c_int) -> @CalendarDateAttr {
        unsafe { @wxCalendarDateAttr(wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd)) as @CalendarDateAttr }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @CalendarDateAttr {
        unsafe { @wxCalendarDateAttr(wxCalendarDateAttr_CreateDefault()) as @CalendarDateAttr }
    }
}

trait CalendarDateAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @Colour) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBorderColour(&self, _ref: @Colour) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @Font) {
        unsafe { wxCalendarDateAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @Colour) {
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
    fn setBackgroundColour(&self, col: @Colour) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.handle(), border) }
    }
    #[fixed_stack_segment]
    fn setBorderColour(&self, col: @Colour) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @Font) {
        unsafe { wxCalendarDateAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.handle(), holiday) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @Colour) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.handle(), col.handle()) }
    }
}

struct wxCalendarEvent(*u8);
impl CalendarEvent for wxCalendarEvent {}
impl CommandEvent for wxCalendarEvent {}
impl Event for wxCalendarEvent {}
impl Object for wxCalendarEvent { fn handle(&self) -> *u8 { **self } }

impl wxCalendarEvent {
}

trait CalendarEvent : CommandEvent {
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
impl Caret for wxCaret { fn handle(&self) -> *u8 { **self } }

impl wxCaret {
    #[fixed_stack_segment]
    pub fn new(_wnd: @Window, _wth: c_int, _hgt: c_int) -> @Caret {
        unsafe { @wxCaret(wxCaret_Create(_wnd.handle(), _wth, _hgt)) as @Caret }
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

trait Caret {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxCaret_GetPosition(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxCaret_GetSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @Window {
        unsafe { @wxWindow(wxCaret_GetWindow(self.handle())) as @Window }
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
impl CheckBox for wxCheckBox {}
impl Control for wxCheckBox {}
impl Window for wxCheckBox {}
impl EvtHandler for wxCheckBox {}
impl Object for wxCheckBox { fn handle(&self) -> *u8 { **self } }

impl wxCheckBox {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @CheckBox {
        unsafe { @wxCheckBox(wxCheckBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @CheckBox }
    }
}

trait CheckBox : Control {
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
impl CheckListBox for wxCheckListBox {}
impl ListBox for wxCheckListBox {}
impl Control for wxCheckListBox {}
impl Window for wxCheckListBox {}
impl EvtHandler for wxCheckListBox {}
impl Object for wxCheckListBox { fn handle(&self) -> *u8 { **self } }

impl wxCheckListBox {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @CheckListBox {
        unsafe { @wxCheckListBox(wxCheckListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @CheckListBox }
    }
}

trait CheckListBox : ListBox {
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
impl Choice for wxChoice {}
impl Control for wxChoice {}
impl Window for wxChoice {}
impl EvtHandler for wxChoice {}
impl Object for wxChoice { fn handle(&self) -> *u8 { **self } }

impl wxChoice {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @Choice {
        unsafe { @wxChoice(wxChoice_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @Choice }
    }
}

trait Choice : Control {
    #[fixed_stack_segment]
    fn append(&self, item: @String) {
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
    fn findString(&self, s: @String) -> c_int {
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
    fn getString(&self, n: c_int) -> @String {
        unsafe { @wxString(wxChoice_GetString(self.handle(), n)) as @String }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, n: c_int) {
        unsafe { wxChoice_SetSelection(self.handle(), n) }
    }
    #[fixed_stack_segment]
    fn setString(&self, n: c_int, s: @String) {
        unsafe { wxChoice_SetString(self.handle(), n, s.handle()) }
    }
}

struct wxClassInfo(*u8);
impl ClassInfo for wxClassInfo { fn handle(&self) -> *u8 { **self } }

impl wxClassInfo {
    #[fixed_stack_segment]
    pub fn findClass(_txt: @String) -> @ClassInfo {
        unsafe { @wxClassInfo(wxClassInfo_FindClass(_txt.handle())) as @ClassInfo }
    }
}

trait ClassInfo {
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
    fn isKindOf(&self, _name: @String) -> bool {
        unsafe { wxClassInfo_IsKindOf(self.handle(), _name.handle()) }
    }
    #[fixed_stack_segment]
    fn getBaseClassName1(&self) -> @String {
        unsafe { @wxString(wxClassInfo_GetBaseClassName1(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getBaseClassName2(&self) -> @String {
        unsafe { @wxString(wxClassInfo_GetBaseClassName2(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getClassNameEx(&self) -> @String {
        unsafe { @wxString(wxClassInfo_GetClassNameEx(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> c_int {
        unsafe { wxClassInfo_GetSize(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isKindOfEx(&self, classInfo: @ClassInfo) -> bool {
        unsafe { wxClassInfo_IsKindOfEx(self.handle(), classInfo.handle()) }
    }
}

struct wxClient(*u8);
impl Client for wxClient {}
impl ClientBase for wxClient {}
impl Object for wxClient { fn handle(&self) -> *u8 { **self } }

impl wxClient {
}

trait Client : ClientBase {
}

struct wxClientBase(*u8);
impl ClientBase for wxClientBase {}
impl Object for wxClientBase { fn handle(&self) -> *u8 { **self } }

impl wxClientBase {
}

trait ClientBase : Object {
}

struct wxClientDC(*u8);
impl ClientDC for wxClientDC {}
impl WindowDC for wxClientDC {}
impl DC for wxClientDC {}
impl Object for wxClientDC { fn handle(&self) -> *u8 { **self } }

impl wxClientDC {
    #[fixed_stack_segment]
    pub fn new(win: @Window) -> @ClientDC {
        unsafe { @wxClientDC(wxClientDC_Create(win.handle())) as @ClientDC }
    }
}

trait ClientDC : WindowDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxClientDC_Delete(self.handle()) }
    }
}

struct wxClientData(*u8);
impl ClientData for wxClientData { fn handle(&self) -> *u8 { **self } }

impl wxClientData {
}

trait ClientData {
    fn handle(&self) -> *u8;
    
}

struct wxClientDataContainer(*u8);
impl ClientDataContainer for wxClientDataContainer { fn handle(&self) -> *u8 { **self } }

impl wxClientDataContainer {
}

trait ClientDataContainer {
    fn handle(&self) -> *u8;
    
}

struct wxClipboard(*u8);
impl Clipboard for wxClipboard {}
impl Object for wxClipboard { fn handle(&self) -> *u8 { **self } }

impl wxClipboard {
    #[fixed_stack_segment]
    pub fn new() -> @Clipboard {
        unsafe { @wxClipboard(wxClipboard_Create()) as @Clipboard }
    }
}

trait Clipboard : Object {
    #[fixed_stack_segment]
    fn addData(&self, data: @DataObject) -> bool {
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
    fn getData(&self, data: @DataObject) -> bool {
        unsafe { wxClipboard_GetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn isOpened(&self) -> bool {
        unsafe { wxClipboard_IsOpened(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isSupported(&self, format: @DataFormat) -> bool {
        unsafe { wxClipboard_IsSupported(self.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn open(&self) -> bool {
        unsafe { wxClipboard_Open(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @DataObject) -> bool {
        unsafe { wxClipboard_SetData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn usePrimarySelection(&self, primary: bool) {
        unsafe { wxClipboard_UsePrimarySelection(self.handle(), primary) }
    }
}

struct wxCloseEvent(*u8);
impl CloseEvent for wxCloseEvent {}
impl Event for wxCloseEvent {}
impl Object for wxCloseEvent { fn handle(&self) -> *u8 { **self } }

impl wxCloseEvent {
}

trait CloseEvent : Event {
    #[fixed_stack_segment]
    fn canVeto(&self) -> bool {
        unsafe { wxCloseEvent_CanVeto(self.handle()) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @Object) {
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
impl Closure for wxClosure {}
impl Object for wxClosure { fn handle(&self) -> *u8 { **self } }

impl wxClosure {
    #[fixed_stack_segment]
    pub fn new(_fun_CEvent: *u8, _data: *u8) -> @Closure {
        unsafe { @wxClosure(wxClosure_Create(_fun_CEvent, _data)) as @Closure }
    }
}

trait Closure : Object {
    #[fixed_stack_segment]
    fn getData(&self) -> *u8 {
        unsafe { wxClosure_GetData(self.handle()) }
    }
}

struct wxColour(*u8);
impl Colour for wxColour {}
impl Object for wxColour { fn handle(&self) -> *u8 { **self } }

impl wxColour {
    #[fixed_stack_segment]
    pub fn newByName(_name: @String) -> @Colour {
        unsafe { @wxColour(wxColour_CreateByName(_name.handle())) as @Colour }
    }
    #[fixed_stack_segment]
    pub fn newEmpty() -> @Colour {
        unsafe { @wxColour(wxColour_CreateEmpty()) as @Colour }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @Colour {
        unsafe { @wxColour(wxColour_CreateFromStock(id)) as @Colour }
    }
    #[fixed_stack_segment]
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> @Colour {
        unsafe { @wxColour(wxColour_CreateRGB(_red, _green, _blue, _alpha)) as @Colour }
    }
    #[fixed_stack_segment]
    pub fn validName(_name: *wchar_t) -> bool {
        unsafe { wxColour_ValidName(_name) }
    }
    #[fixed_stack_segment]
    pub fn newFromInt(rgb: c_int) -> @Colour {
        unsafe { @wxColour(wxColour_CreateFromInt(rgb)) as @Colour }
    }
    #[fixed_stack_segment]
    pub fn newFromUnsignedInt(rgba: uint32_t) -> @Colour {
        unsafe { @wxColour(wxColour_CreateFromUnsignedInt(rgba)) as @Colour }
    }
}

trait Colour : Object {
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
    fn setByName(&self, _name: @String) {
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
impl ColourData for wxColourData {}
impl Object for wxColourData { fn handle(&self) -> *u8 { **self } }

impl wxColourData {
    #[fixed_stack_segment]
    pub fn new() -> @ColourData {
        unsafe { @wxColourData(wxColourData_Create()) as @ColourData }
    }
}

trait ColourData : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxColourData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getChooseFull(&self) -> bool {
        unsafe { wxColourData_GetChooseFull(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @Colour) {
        unsafe { wxColourData_GetColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCustomColour(&self, i: c_int, _ref: @Colour) {
        unsafe { wxColourData_GetCustomColour(self.handle(), i, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setChooseFull(&self, flag: bool) {
        unsafe { wxColourData_SetChooseFull(self.handle(), flag) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @Colour) {
        unsafe { wxColourData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCustomColour(&self, i: c_int, colour: @Colour) {
        unsafe { wxColourData_SetCustomColour(self.handle(), i, colour.handle()) }
    }
}

struct wxColourDatabase(*u8);
impl ColourDatabase for wxColourDatabase {}
impl List for wxColourDatabase {}
impl Object for wxColourDatabase { fn handle(&self) -> *u8 { **self } }

impl wxColourDatabase {
}

trait ColourDatabase : List {
}

struct wxColourDialog(*u8);
impl ColourDialog for wxColourDialog {}
impl Dialog for wxColourDialog {}
impl TopLevelWindow for wxColourDialog {}
impl Window for wxColourDialog {}
impl EvtHandler for wxColourDialog {}
impl Object for wxColourDialog { fn handle(&self) -> *u8 { **self } }

impl wxColourDialog {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, col: @ColourData) -> @ColourDialog {
        unsafe { @wxColourDialog(wxColourDialog_Create(_prt.handle(), col.handle())) as @ColourDialog }
    }
}

trait ColourDialog : Dialog {
    #[fixed_stack_segment]
    fn getColourData(&self, _ref: @ColourData) {
        unsafe { wxColourDialog_GetColourData(self.handle(), _ref.handle()) }
    }
}

struct wxComboBox(*u8);
impl ComboBox for wxComboBox {}
impl Choice for wxComboBox {}
impl Control for wxComboBox {}
impl Window for wxComboBox {}
impl EvtHandler for wxComboBox {}
impl Object for wxComboBox { fn handle(&self) -> *u8 { **self } }

impl wxComboBox {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @ComboBox {
        unsafe { @wxComboBox(wxComboBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, str, _stl)) as @ComboBox }
    }
}

trait ComboBox : Choice {
    #[fixed_stack_segment]
    fn append(&self, item: @String) {
        unsafe { wxComboBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @String, d: *u8) {
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
    fn findString(&self, s: @String) -> c_int {
        unsafe { wxComboBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> @ClientData {
        unsafe { @wxClientData(wxComboBox_GetClientData(self.handle(), n)) as @ClientData }
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
    fn getString(&self, n: c_int) -> @String {
        unsafe { @wxString(wxComboBox_GetString(self.handle(), n)) as @String }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @String {
        unsafe { @wxString(wxComboBox_GetStringSelection(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> @String {
        unsafe { @wxString(wxComboBox_GetValue(self.handle())) as @String }
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
    fn replace(&self, from: c_int, to: c_int, value: @String) {
        unsafe { wxComboBox_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, n: c_int, clientData: @ClientData) {
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
impl Command for wxCommand {}
impl Object for wxCommand { fn handle(&self) -> *u8 { **self } }

impl wxCommand {
}

trait Command : Object {
}

struct wxCommandEvent(*u8);
impl CommandEvent for wxCommandEvent {}
impl Event for wxCommandEvent {}
impl Object for wxCommandEvent { fn handle(&self) -> *u8 { **self } }

impl wxCommandEvent {
    #[fixed_stack_segment]
    pub fn new(_typ: c_int, _id: c_int) -> @CommandEvent {
        unsafe { @wxCommandEvent(wxCommandEvent_Create(_typ, _id)) as @CommandEvent }
    }
}

trait CommandEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxCommandEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxCommandEvent_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @ClientData {
        unsafe { @wxClientData(wxCommandEvent_GetClientData(self.handle())) as @ClientData }
    }
    #[fixed_stack_segment]
    fn getClientObject(&self) -> @ClientData {
        unsafe { @wxClientData(wxCommandEvent_GetClientObject(self.handle())) as @ClientData }
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
    fn getString(&self) -> @String {
        unsafe { @wxString(wxCommandEvent_GetString(self.handle())) as @String }
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
    fn setClientData(&self, clientData: @ClientData) {
        unsafe { wxCommandEvent_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, clientObject: @ClientData) {
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
    fn setString(&self, s: @String) {
        unsafe { wxCommandEvent_SetString(self.handle(), s.handle()) }
    }
}

struct wxCommandLineParser(*u8);
impl CommandLineParser for wxCommandLineParser { fn handle(&self) -> *u8 { **self } }

impl wxCommandLineParser {
}

trait CommandLineParser {
    fn handle(&self) -> *u8;
    
}

struct wxCommandProcessor(*u8);
impl CommandProcessor for wxCommandProcessor {}
impl Object for wxCommandProcessor { fn handle(&self) -> *u8 { **self } }

impl wxCommandProcessor {
}

trait CommandProcessor : Object {
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
    fn setEditMenu(&self, menu: @Menu) {
        unsafe { wxCommandProcessor_SetEditMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn setMenuStrings(&self) {
        unsafe { wxCommandProcessor_SetMenuStrings(self.handle()) }
    }
    #[fixed_stack_segment]
    fn submit(&self, command: @Command, storeIt: c_int) -> c_int {
        unsafe { wxCommandProcessor_Submit(self.handle(), command.handle(), storeIt) }
    }
    #[fixed_stack_segment]
    fn undo(&self) -> c_int {
        unsafe { wxCommandProcessor_Undo(self.handle()) }
    }
}

struct wxCondition(*u8);
impl Condition for wxCondition { fn handle(&self) -> *u8 { **self } }

impl wxCondition {
}

trait Condition {
    fn handle(&self) -> *u8;
    
}

struct wxConfigBase(*u8);
impl ConfigBase for wxConfigBase { fn handle(&self) -> *u8 { **self } }

impl wxConfigBase {
    #[fixed_stack_segment]
    pub fn new() -> @ConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Create()) as @ConfigBase }
    }
    #[fixed_stack_segment]
    pub fn get() -> @ConfigBase {
        unsafe { @wxConfigBase(wxConfigBase_Get()) as @ConfigBase }
    }
    #[fixed_stack_segment]
    pub fn set(self_: @ConfigBase) {
        unsafe { wxConfigBase_Set(self_.handle()) }
    }
}

trait ConfigBase {
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
    fn deleteEntry(&self, key: @String, bDeleteGroupIfEmpty: bool) -> bool {
        unsafe { wxConfigBase_DeleteEntry(self.handle(), key.handle(), bDeleteGroupIfEmpty) }
    }
    #[fixed_stack_segment]
    fn deleteGroup(&self, key: @String) -> bool {
        unsafe { wxConfigBase_DeleteGroup(self.handle(), key.handle()) }
    }
    #[fixed_stack_segment]
    fn exists(&self, strName: @String) -> bool {
        unsafe { wxConfigBase_Exists(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn expandEnvVars(&self, str: @String) -> @String {
        unsafe { @wxString(wxConfigBase_ExpandEnvVars(self.handle(), str.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn flush(&self, bCurrentOnly: bool) -> bool {
        unsafe { wxConfigBase_Flush(self.handle(), bCurrentOnly) }
    }
    #[fixed_stack_segment]
    fn getAppName(&self) -> @String {
        unsafe { @wxString(wxConfigBase_GetAppName(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getEntryType(&self, name: @String) -> c_int {
        unsafe { wxConfigBase_GetEntryType(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstEntry(&self, lIndex: *u8) -> @String {
        unsafe { @wxString(wxConfigBase_GetFirstEntry(self.handle(), lIndex)) as @String }
    }
    #[fixed_stack_segment]
    fn getFirstGroup(&self, lIndex: *u8) -> @String {
        unsafe { @wxString(wxConfigBase_GetFirstGroup(self.handle(), lIndex)) as @String }
    }
    #[fixed_stack_segment]
    fn getNextEntry(&self, lIndex: *u8) -> @String {
        unsafe { @wxString(wxConfigBase_GetNextEntry(self.handle(), lIndex)) as @String }
    }
    #[fixed_stack_segment]
    fn getNextGroup(&self, lIndex: *u8) -> @String {
        unsafe { @wxString(wxConfigBase_GetNextGroup(self.handle(), lIndex)) as @String }
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
    fn getPath(&self) -> @String {
        unsafe { @wxString(wxConfigBase_GetPath(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxConfigBase_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getVendorName(&self) -> @String {
        unsafe { @wxString(wxConfigBase_GetVendorName(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn hasEntry(&self, strName: @String) -> bool {
        unsafe { wxConfigBase_HasEntry(self.handle(), strName.handle()) }
    }
    #[fixed_stack_segment]
    fn hasGroup(&self, strName: @String) -> bool {
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
    fn readBool(&self, key: @String, defVal: bool) -> bool {
        unsafe { wxConfigBase_ReadBool(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readDouble(&self, key: @String, defVal: c_double) -> c_double {
        unsafe { wxConfigBase_ReadDouble(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readInteger(&self, key: @String, defVal: c_int) -> c_int {
        unsafe { wxConfigBase_ReadInteger(self.handle(), key.handle(), defVal) }
    }
    #[fixed_stack_segment]
    fn readString(&self, key: @String, defVal: @String) -> @String {
        unsafe { @wxString(wxConfigBase_ReadString(self.handle(), key.handle(), defVal.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn renameEntry(&self, oldName: @String, newName: @String) -> bool {
        unsafe { wxConfigBase_RenameEntry(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn renameGroup(&self, oldName: @String, newName: @String) -> bool {
        unsafe { wxConfigBase_RenameGroup(self.handle(), oldName.handle(), newName.handle()) }
    }
    #[fixed_stack_segment]
    fn setAppName(&self, appName: @String) {
        unsafe { wxConfigBase_SetAppName(self.handle(), appName.handle()) }
    }
    #[fixed_stack_segment]
    fn setExpandEnvVars(&self, bDoIt: bool) {
        unsafe { wxConfigBase_SetExpandEnvVars(self.handle(), bDoIt) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, strPath: @String) {
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
    fn setVendorName(&self, vendorName: @String) {
        unsafe { wxConfigBase_SetVendorName(self.handle(), vendorName.handle()) }
    }
    #[fixed_stack_segment]
    fn writeBool(&self, key: @String, value: bool) -> bool {
        unsafe { wxConfigBase_WriteBool(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeDouble(&self, key: @String, value: c_double) -> bool {
        unsafe { wxConfigBase_WriteDouble(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeInteger(&self, key: @String, value: c_int) -> bool {
        unsafe { wxConfigBase_WriteInteger(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeLong(&self, key: @String, value: c_long) -> bool {
        unsafe { wxConfigBase_WriteLong(self.handle(), key.handle(), value) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, key: @String, value: @String) -> bool {
        unsafe { wxConfigBase_WriteString(self.handle(), key.handle(), value.handle()) }
    }
}

struct wxConnection(*u8);
impl Connection for wxConnection {}
impl ConnectionBase for wxConnection {}
impl Object for wxConnection { fn handle(&self) -> *u8 { **self } }

impl wxConnection {
}

trait Connection : ConnectionBase {
}

struct wxConnectionBase(*u8);
impl ConnectionBase for wxConnectionBase {}
impl Object for wxConnectionBase { fn handle(&self) -> *u8 { **self } }

impl wxConnectionBase {
}

trait ConnectionBase : Object {
}

struct wxContextHelp(*u8);
impl ContextHelp for wxContextHelp {}
impl Object for wxContextHelp { fn handle(&self) -> *u8 { **self } }

impl wxContextHelp {
    #[fixed_stack_segment]
    pub fn new(win: @Window, beginHelp: bool) -> @ContextHelp {
        unsafe { @wxContextHelp(wxContextHelp_Create(win.handle(), beginHelp)) as @ContextHelp }
    }
}

trait ContextHelp : Object {
    #[fixed_stack_segment]
    fn beginContextHelp(&self, win: @Window) -> bool {
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
impl ContextHelpButton for wxContextHelpButton {}
impl BitmapButton for wxContextHelpButton {}
impl Button for wxContextHelpButton {}
impl Control for wxContextHelpButton {}
impl Window for wxContextHelpButton {}
impl EvtHandler for wxContextHelpButton {}
impl Object for wxContextHelpButton { fn handle(&self) -> *u8 { **self } }

impl wxContextHelpButton {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @ContextHelpButton {
        unsafe { @wxContextHelpButton(wxContextHelpButton_Create(parent.handle(), id, x, y, w, h, style)) as @ContextHelpButton }
    }
}

trait ContextHelpButton : BitmapButton {
}

struct wxControl(*u8);
impl Control for wxControl {}
impl Window for wxControl {}
impl EvtHandler for wxControl {}
impl Object for wxControl { fn handle(&self) -> *u8 { **self } }

impl wxControl {
}

trait Control : Window {
    #[fixed_stack_segment]
    fn command(&self, event: @Event) {
        unsafe { wxControl_Command(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @String {
        unsafe { @wxString(wxControl_GetLabel(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, text: @String) {
        unsafe { wxControl_SetLabel(self.handle(), text.handle()) }
    }
}

struct wxCountingOutputStream(*u8);
impl CountingOutputStream for wxCountingOutputStream {}
impl OutputStream for wxCountingOutputStream {}
impl StreamBase for wxCountingOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxCountingOutputStream {
}

trait CountingOutputStream : OutputStream {
}

struct wxCriticalSection(*u8);
impl CriticalSection for wxCriticalSection { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSection {
}

trait CriticalSection {
    fn handle(&self) -> *u8;
    
}

struct wxCriticalSectionLocker(*u8);
impl CriticalSectionLocker for wxCriticalSectionLocker { fn handle(&self) -> *u8 { **self } }

impl wxCriticalSectionLocker {
}

trait CriticalSectionLocker {
    fn handle(&self) -> *u8;
    
}

struct wxCursor(*u8);
impl Cursor for wxCursor {}
impl Bitmap for wxCursor {}
impl GDIObject for wxCursor {}
impl Object for wxCursor { fn handle(&self) -> *u8 { **self } }

impl wxCursor {
}

trait Cursor : Bitmap {
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
impl CustomDataObject for wxCustomDataObject {}
impl DataObjectSimple for wxCustomDataObject {}
impl DataObject for wxCustomDataObject { fn handle(&self) -> *u8 { **self } }

impl wxCustomDataObject {
}

trait CustomDataObject : DataObjectSimple {
}

struct wxDC(*u8);
impl DC for wxDC {}
impl Object for wxDC { fn handle(&self) -> *u8 { **self } }

impl wxDC {
}

trait DC : Object {
    #[fixed_stack_segment]
    fn blit(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: @DC, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: bool) -> bool {
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
    fn drawBitmap(&self, bmp: @Bitmap, x: c_int, y: c_int, useMask: bool) {
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
    fn drawIcon(&self, icon: @Icon, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.handle(), icon.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawLabel(&self, str: @String, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        unsafe { wxDC_DrawLabel(self.handle(), str.handle(), x, y, w, h, align, indexAccel) }
    }
    #[fixed_stack_segment]
    fn drawLabelBitmap(&self, str: @String, bmp: @Bitmap, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> @Rect {
        unsafe { @wxRect(wxDC_DrawLabelBitmap(self.handle(), str.handle(), bmp.handle(), x, y, w, h, align, indexAccel)) as @Rect }
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
    fn drawRotatedText(&self, text: @String, x: c_int, y: c_int, angle: c_double) {
        unsafe { wxDC_DrawRotatedText(self.handle(), text.handle(), x, y, angle) }
    }
    #[fixed_stack_segment]
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double) {
        unsafe { wxDC_DrawRoundedRectangle(self.handle(), x, y, width, height, radius) }
    }
    #[fixed_stack_segment]
    fn drawText(&self, text: @String, x: c_int, y: c_int) {
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
    fn floodFill(&self, x: c_int, y: c_int, col: @Colour, style: c_int) {
        unsafe { wxDC_FloodFill(self.handle(), x, y, col.handle(), style) }
    }
    #[fixed_stack_segment]
    fn getBackground(&self, _ref: @Brush) {
        unsafe { wxDC_GetBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBrush(&self, _ref: @Brush) {
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
    fn getFont(&self, _ref: @Font) {
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
    fn getPPI(&self) -> @Size {
        unsafe { @wxSize(wxDC_GetPPI(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getPen(&self, _ref: @Pen) {
        unsafe { wxDC_GetPen(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPixel(&self, x: c_int, y: c_int, col: @Colour) -> bool {
        unsafe { wxDC_GetPixel(self.handle(), x, y, col.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxDC_GetSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getSizeMM(&self) -> @Size {
        unsafe { @wxSize(wxDC_GetSizeMM(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getTextBackground(&self, _ref: @Colour) {
        unsafe { wxDC_GetTextBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @String, w: *u8, h: *u8, descent: *u8, externalLeading: *u8, theFont: @Font) {
        unsafe { wxDC_GetTextExtent(self.handle(), string.handle(), w, h, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getMultiLineTextExtent(&self, string: @String, w: *u8, h: *u8, heightLine: *u8, theFont: @Font) {
        unsafe { wxDC_GetMultiLineTextExtent(self.handle(), string.handle(), w, h, heightLine, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextForeground(&self, _ref: @Colour) {
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
    fn setBackground(&self, brush: @Brush) {
        unsafe { wxDC_SetBackground(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.handle(), mode) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @Brush) {
        unsafe { wxDC_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn setClippingRegionFromRegion(&self, region: @Region) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @Font) {
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
    fn setPalette(&self, palette: @Palette) {
        unsafe { wxDC_SetPalette(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @Pen) {
        unsafe { wxDC_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextBackground(&self, colour: @Colour) {
        unsafe { wxDC_SetTextBackground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextForeground(&self, colour: @Colour) {
        unsafe { wxDC_SetTextForeground(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setUserScale(&self, x: c_double, y: c_double) {
        unsafe { wxDC_SetUserScale(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn startDoc(&self, msg: @String) -> bool {
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
    fn getPixel2(&self, x: c_int, y: c_int, col: @Colour) {
        unsafe { wxDC_GetPixel2(self.handle(), x, y, col.handle()) }
    }
}

struct wxDCClipper(*u8);
impl DCClipper for wxDCClipper { fn handle(&self) -> *u8 { **self } }

impl wxDCClipper {
}

trait DCClipper {
    fn handle(&self) -> *u8;
    
}

struct wxDDEClient(*u8);
impl DDEClient for wxDDEClient {}
impl ClientBase for wxDDEClient {}
impl Object for wxDDEClient { fn handle(&self) -> *u8 { **self } }

impl wxDDEClient {
}

trait DDEClient : ClientBase {
}

struct wxDDEConnection(*u8);
impl DDEConnection for wxDDEConnection {}
impl ConnectionBase for wxDDEConnection {}
impl Object for wxDDEConnection { fn handle(&self) -> *u8 { **self } }

impl wxDDEConnection {
}

trait DDEConnection : ConnectionBase {
}

struct wxDDEServer(*u8);
impl DDEServer for wxDDEServer {}
impl ServerBase for wxDDEServer {}
impl Object for wxDDEServer { fn handle(&self) -> *u8 { **self } }

impl wxDDEServer {
}

trait DDEServer : ServerBase {
}

struct wxDataFormat(*u8);
impl DataFormat for wxDataFormat { fn handle(&self) -> *u8 { **self } }

impl wxDataFormat {
    #[fixed_stack_segment]
    pub fn newFromId(name: @String) -> @DataFormat {
        unsafe { @wxDataFormat(wxDataFormat_CreateFromId(name.handle())) as @DataFormat }
    }
    #[fixed_stack_segment]
    pub fn newFromType(typ: c_int) -> @DataFormat {
        unsafe { @wxDataFormat(wxDataFormat_CreateFromType(typ)) as @DataFormat }
    }
}

trait DataFormat {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDataFormat_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> @String {
        unsafe { @wxString(wxDataFormat_GetId(self.handle())) as @String }
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
impl DataInputStream for wxDataInputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataInputStream {
}

trait DataInputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDataObject(*u8);
impl DataObject for wxDataObject { fn handle(&self) -> *u8 { **self } }

impl wxDataObject {
}

trait DataObject {
    fn handle(&self) -> *u8;
    
}

struct wxDataObjectComposite(*u8);
impl DataObjectComposite for wxDataObjectComposite {}
impl DataObject for wxDataObjectComposite { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectComposite {
    #[fixed_stack_segment]
    pub fn new() -> @DataObjectComposite {
        unsafe { @wxDataObjectComposite(wxDataObjectComposite_Create()) as @DataObjectComposite }
    }
}

trait DataObjectComposite : DataObject {
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
impl DataObjectSimple for wxDataObjectSimple {}
impl DataObject for wxDataObjectSimple { fn handle(&self) -> *u8 { **self } }

impl wxDataObjectSimple {
}

trait DataObjectSimple : DataObject {
}

struct wxDataOutputStream(*u8);
impl DataOutputStream for wxDataOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxDataOutputStream {
}

trait DataOutputStream {
    fn handle(&self) -> *u8;
    
}

struct wxDatabase(*u8);
impl Database for wxDatabase {}
impl Object for wxDatabase { fn handle(&self) -> *u8 { **self } }

impl wxDatabase {
}

trait Database : Object {
}

struct wxDateTime(*u8);
impl DateTime for wxDateTime { fn handle(&self) -> *u8 { **self } }

impl wxDateTime {
    #[fixed_stack_segment]
    pub fn convertYearToBC(year: c_int) -> c_int {
        unsafe { wxDateTime_ConvertYearToBC(year) }
    }
    #[fixed_stack_segment]
    pub fn new() -> @DateTime {
        unsafe { @wxDateTime(wxDateTime_Create()) as @DateTime }
    }
    #[fixed_stack_segment]
    pub fn getAmString() -> @String {
        unsafe { @wxString(wxDateTime_GetAmString()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getBeginDST(year: c_int, country: c_int, dt: @DateTime) {
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
    pub fn getEndDST(year: c_int, country: c_int, dt: @DateTime) {
        unsafe { wxDateTime_GetEndDST(year, country, dt.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getMonthName(month: c_int, flags: c_int) -> @String {
        unsafe { @wxString(wxDateTime_GetMonthName(month, flags)) as @String }
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
    pub fn getPmString() -> @String {
        unsafe { @wxString(wxDateTime_GetPmString()) as @String }
    }
    #[fixed_stack_segment]
    pub fn getTimeNow() -> c_int {
        unsafe { wxDateTime_GetTimeNow() }
    }
    #[fixed_stack_segment]
    pub fn getWeekDayName(weekday: c_int, flags: c_int) -> @String {
        unsafe { @wxString(wxDateTime_GetWeekDayName(weekday, flags)) as @String }
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

trait DateTime {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addDate(&self, diff: *u8, _ref: @DateTime) {
        unsafe { wxDateTime_AddDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addDateValues(&self, _yrs: c_int, _mnt: c_int, _wek: c_int, _day: c_int) {
        unsafe { wxDateTime_AddDateValues(self.handle(), _yrs, _mnt, _wek, _day) }
    }
    #[fixed_stack_segment]
    fn addTime(&self, diff: *u8, _ref: @DateTime) {
        unsafe { wxDateTime_AddTime(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn addTimeValues(&self, _hrs: c_int, _min: c_int, _sec: c_int, _mls: c_int) {
        unsafe { wxDateTime_AddTimeValues(self.handle(), _hrs, _min, _sec, _mls) }
    }
    #[fixed_stack_segment]
    fn format(&self, format: *u8, tz: c_int) -> @String {
        unsafe { @wxString(wxDateTime_Format(self.handle(), format, tz)) as @String }
    }
    #[fixed_stack_segment]
    fn formatDate(&self) -> @String {
        unsafe { @wxString(wxDateTime_FormatDate(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn formatISODate(&self) -> @String {
        unsafe { @wxString(wxDateTime_FormatISODate(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn formatISOTime(&self) -> @String {
        unsafe { @wxString(wxDateTime_FormatISOTime(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn formatTime(&self) -> @String {
        unsafe { @wxString(wxDateTime_FormatTime(self.handle())) as @String }
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
    fn getLastMonthDay(&self, month: c_int, year: c_int, _ref: @DateTime) {
        unsafe { wxDateTime_GetLastMonthDay(self.handle(), month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLastWeekDay(&self, weekday: c_int, month: c_int, year: c_int, _ref: @DateTime) {
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
    fn getNextWeekDay(&self, weekday: c_int, _ref: @DateTime) {
        unsafe { wxDateTime_GetNextWeekDay(self.handle(), weekday, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevWeekDay(&self, weekday: c_int, _ref: @DateTime) {
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
    fn getWeekDay(&self, weekday: c_int, n: c_int, month: c_int, year: c_int, _ref: @DateTime) {
        unsafe { wxDateTime_GetWeekDay(self.handle(), weekday, n, month, year, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getWeekDayInSameWeek(&self, weekday: c_int, _ref: @DateTime) {
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
    fn isBetween(&self, t1: @DateTime, t2: @DateTime) -> bool {
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
    fn isEqualUpTo(&self, dt: @DateTime, ts: *u8) -> bool {
        unsafe { wxDateTime_IsEqualUpTo(self.handle(), dt.handle(), ts) }
    }
    #[fixed_stack_segment]
    fn isLaterThan(&self, datetime: *u8) -> bool {
        unsafe { wxDateTime_IsLaterThan(self.handle(), datetime) }
    }
    #[fixed_stack_segment]
    fn isSameDate(&self, dt: @DateTime) -> bool {
        unsafe { wxDateTime_IsSameDate(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isSameTime(&self, dt: @DateTime) -> bool {
        unsafe { wxDateTime_IsSameTime(self.handle(), dt.handle()) }
    }
    #[fixed_stack_segment]
    fn isStrictlyBetween(&self, t1: @DateTime, t2: @DateTime) -> bool {
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
    fn parseTime(&self, time: @Time) -> *u8 {
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
    fn subtractDate(&self, diff: *u8, _ref: @DateTime) {
        unsafe { wxDateTime_SubtractDate(self.handle(), diff, _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractTime(&self, diff: *u8, _ref: @DateTime) {
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
impl Db for wxDb { fn handle(&self) -> *u8 { **self } }

impl wxDb {
}

trait Db {
    fn handle(&self) -> *u8;
    
}

struct wxDbColDef(*u8);
impl DbColDef for wxDbColDef { fn handle(&self) -> *u8 { **self } }

impl wxDbColDef {
}

trait DbColDef {
    fn handle(&self) -> *u8;
    
}

struct wxDbColFor(*u8);
impl DbColFor for wxDbColFor { fn handle(&self) -> *u8 { **self } }

impl wxDbColFor {
}

trait DbColFor {
    fn handle(&self) -> *u8;
    
}

struct wxDbColInf(*u8);
impl DbColInf for wxDbColInf { fn handle(&self) -> *u8 { **self } }

impl wxDbColInf {
}

trait DbColInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbConnectInf(*u8);
impl DbConnectInf for wxDbConnectInf { fn handle(&self) -> *u8 { **self } }

impl wxDbConnectInf {
}

trait DbConnectInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbInf(*u8);
impl DbInf for wxDbInf { fn handle(&self) -> *u8 { **self } }

impl wxDbInf {
}

trait DbInf {
    fn handle(&self) -> *u8;
    
}

struct wxDbSqlTypeInfo(*u8);
impl DbSqlTypeInfo for wxDbSqlTypeInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbSqlTypeInfo {
}

trait DbSqlTypeInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDbTable(*u8);
impl DbTable for wxDbTable { fn handle(&self) -> *u8 { **self } }

impl wxDbTable {
}

trait DbTable {
    fn handle(&self) -> *u8;
    
}

struct wxDbTableInfo(*u8);
impl DbTableInfo for wxDbTableInfo { fn handle(&self) -> *u8 { **self } }

impl wxDbTableInfo {
}

trait DbTableInfo {
    fn handle(&self) -> *u8;
    
}

struct wxDebugContext(*u8);
impl DebugContext for wxDebugContext { fn handle(&self) -> *u8 { **self } }

impl wxDebugContext {
}

trait DebugContext {
    fn handle(&self) -> *u8;
    
}

struct wxDialUpEvent(*u8);
impl DialUpEvent for wxDialUpEvent {}
impl Event for wxDialUpEvent {}
impl Object for wxDialUpEvent { fn handle(&self) -> *u8 { **self } }

impl wxDialUpEvent {
}

trait DialUpEvent : Event {
    #[fixed_stack_segment]
    fn isConnectedEvent(&self) -> bool {
        unsafe { wxDialUpEvent_IsConnectedEvent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOwnEvent(&self) -> bool {
        unsafe { wxDialUpEvent_IsOwnEvent(self.handle()) }
    }
}

struct wxDialUpManager(*u8);
impl DialUpManager for wxDialUpManager { fn handle(&self) -> *u8 { **self } }

impl wxDialUpManager {
}

trait DialUpManager {
    fn handle(&self) -> *u8;
    
}

struct wxDialog(*u8);
impl Dialog for wxDialog {}
impl TopLevelWindow for wxDialog {}
impl Window for wxDialog {}
impl EvtHandler for wxDialog {}
impl Object for wxDialog { fn handle(&self) -> *u8 { **self } }

impl wxDialog {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Dialog {
        unsafe { @wxDialog(wxDialog_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @Dialog }
    }
}

trait Dialog : TopLevelWindow {
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
impl DirDialog for wxDirDialog {}
impl Dialog for wxDirDialog {}
impl TopLevelWindow for wxDirDialog {}
impl Window for wxDirDialog {}
impl EvtHandler for wxDirDialog {}
impl Object for wxDirDialog { fn handle(&self) -> *u8 { **self } }

impl wxDirDialog {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _msg: @String, _dir: @String, _lft: c_int, _top: c_int, _stl: c_int) -> @DirDialog {
        unsafe { @wxDirDialog(wxDirDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _lft, _top, _stl)) as @DirDialog }
    }
}

trait DirDialog : Dialog {
    #[fixed_stack_segment]
    fn getMessage(&self) -> @String {
        unsafe { @wxString(wxDirDialog_GetMessage(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @String {
        unsafe { @wxString(wxDirDialog_GetPath(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxDirDialog_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, msg: @String) {
        unsafe { wxDirDialog_SetMessage(self.handle(), msg.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, pth: @String) {
        unsafe { wxDirDialog_SetPath(self.handle(), pth.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxDirDialog_SetStyle(self.handle(), style) }
    }
}

struct wxDirTraverser(*u8);
impl DirTraverser for wxDirTraverser { fn handle(&self) -> *u8 { **self } }

impl wxDirTraverser {
}

trait DirTraverser {
    fn handle(&self) -> *u8;
    
}

struct wxDllLoader(*u8);
impl DllLoader for wxDllLoader { fn handle(&self) -> *u8 { **self } }

impl wxDllLoader {
}

trait DllLoader {
    fn handle(&self) -> *u8;
    
}

struct wxDocChildFrame(*u8);
impl DocChildFrame for wxDocChildFrame {}
impl Frame for wxDocChildFrame {}
impl TopLevelWindow for wxDocChildFrame {}
impl Window for wxDocChildFrame {}
impl EvtHandler for wxDocChildFrame {}
impl Object for wxDocChildFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocChildFrame {
}

trait DocChildFrame : Frame {
}

struct wxDocMDIChildFrame(*u8);
impl DocMDIChildFrame for wxDocMDIChildFrame {}
impl MDIChildFrame for wxDocMDIChildFrame {}
impl Frame for wxDocMDIChildFrame {}
impl TopLevelWindow for wxDocMDIChildFrame {}
impl Window for wxDocMDIChildFrame {}
impl EvtHandler for wxDocMDIChildFrame {}
impl Object for wxDocMDIChildFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocMDIChildFrame {
}

trait DocMDIChildFrame : MDIChildFrame {
}

struct wxDocMDIParentFrame(*u8);
impl DocMDIParentFrame for wxDocMDIParentFrame {}
impl MDIParentFrame for wxDocMDIParentFrame {}
impl Frame for wxDocMDIParentFrame {}
impl TopLevelWindow for wxDocMDIParentFrame {}
impl Window for wxDocMDIParentFrame {}
impl EvtHandler for wxDocMDIParentFrame {}
impl Object for wxDocMDIParentFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocMDIParentFrame {
}

trait DocMDIParentFrame : MDIParentFrame {
}

struct wxDocManager(*u8);
impl DocManager for wxDocManager {}
impl EvtHandler for wxDocManager {}
impl Object for wxDocManager { fn handle(&self) -> *u8 { **self } }

impl wxDocManager {
}

trait DocManager : EvtHandler {
}

struct wxDocParentFrame(*u8);
impl DocParentFrame for wxDocParentFrame {}
impl Frame for wxDocParentFrame {}
impl TopLevelWindow for wxDocParentFrame {}
impl Window for wxDocParentFrame {}
impl EvtHandler for wxDocParentFrame {}
impl Object for wxDocParentFrame { fn handle(&self) -> *u8 { **self } }

impl wxDocParentFrame {
}

trait DocParentFrame : Frame {
}

struct wxDocTemplate(*u8);
impl DocTemplate for wxDocTemplate {}
impl Object for wxDocTemplate { fn handle(&self) -> *u8 { **self } }

impl wxDocTemplate {
}

trait DocTemplate : Object {
}

struct wxDocument(*u8);
impl Document for wxDocument {}
impl EvtHandler for wxDocument {}
impl Object for wxDocument { fn handle(&self) -> *u8 { **self } }

impl wxDocument {
}

trait Document : EvtHandler {
}

struct wxDragImage(*u8);
impl DragImage for wxDragImage {}
impl Object for wxDragImage { fn handle(&self) -> *u8 { **self } }

impl wxDragImage {
    #[fixed_stack_segment]
    pub fn new(image: @Bitmap, x: c_int, y: c_int) -> @DragImage {
        unsafe { @wxDragImage(wxDragImage_Create(image.handle(), x, y)) as @DragImage }
    }
}

trait DragImage : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxDragImage_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDragFullScreen(&self, x_pos: c_int, y_pos: c_int, window: @Window, fullScreen: bool, rect: @Rect) -> bool {
        unsafe { wxDragImage_BeginDragFullScreen(self.handle(), x_pos, y_pos, window.handle(), fullScreen, rect.handle()) }
    }
    #[fixed_stack_segment]
    fn beginDrag(&self, x: c_int, y: c_int, window: @Window, boundingWindow: @Window) -> bool {
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
impl DrawControl for wxDrawControl {}
impl Control for wxDrawControl {}
impl Window for wxDrawControl {}
impl EvtHandler for wxDrawControl {}
impl Object for wxDrawControl { fn handle(&self) -> *u8 { **self } }

impl wxDrawControl {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @DrawControl {
        unsafe { @wxDrawControl(wxDrawControl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @DrawControl }
    }
}

trait DrawControl : Control {
}

struct wxDrawWindow(*u8);
impl DrawWindow for wxDrawWindow {}
impl Window for wxDrawWindow {}
impl EvtHandler for wxDrawWindow {}
impl Object for wxDrawWindow { fn handle(&self) -> *u8 { **self } }

impl wxDrawWindow {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @DrawWindow {
        unsafe { @wxDrawWindow(wxDrawWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @DrawWindow }
    }
}

trait DrawWindow : Window {
}

struct wxDropFilesEvent(*u8);
impl DropFilesEvent for wxDropFilesEvent {}
impl Event for wxDropFilesEvent {}
impl Object for wxDropFilesEvent { fn handle(&self) -> *u8 { **self } }

impl wxDropFilesEvent {
}

trait DropFilesEvent : Event {
}

struct wxDropSource(*u8);
impl DropSource for wxDropSource { fn handle(&self) -> *u8 { **self } }

impl wxDropSource {
}

trait DropSource {
    fn handle(&self) -> *u8;
    
}

struct wxDropTarget(*u8);
impl DropTarget for wxDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxDropTarget {
}

trait DropTarget {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDataObject(&self, _dat: @DataObject) {
        unsafe { wxDropTarget_SetDataObject(self.handle(), _dat.handle()) }
    }
}

struct wxDynToolInfo(*u8);
impl DynToolInfo for wxDynToolInfo {}
impl ToolLayoutItem for wxDynToolInfo {}
impl Object for wxDynToolInfo { fn handle(&self) -> *u8 { **self } }

impl wxDynToolInfo {
}

trait DynToolInfo : ToolLayoutItem {
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

struct wxDynamicLibrary(*u8);
impl DynamicLibrary for wxDynamicLibrary { fn handle(&self) -> *u8 { **self } }

impl wxDynamicLibrary {
}

trait DynamicLibrary {
    fn handle(&self) -> *u8;
    
}

struct wxDynamicSashWindow(*u8);
impl DynamicSashWindow for wxDynamicSashWindow {}
impl Window for wxDynamicSashWindow {}
impl EvtHandler for wxDynamicSashWindow {}
impl Object for wxDynamicSashWindow { fn handle(&self) -> *u8 { **self } }

impl wxDynamicSashWindow {
}

trait DynamicSashWindow : Window {
}

struct wxDynamicToolBar(*u8);
impl DynamicToolBar for wxDynamicToolBar {}
impl ToolBarBase for wxDynamicToolBar {}
impl Control for wxDynamicToolBar {}
impl Window for wxDynamicToolBar {}
impl EvtHandler for wxDynamicToolBar {}
impl Object for wxDynamicToolBar { fn handle(&self) -> *u8 { **self } }

impl wxDynamicToolBar {
}

trait DynamicToolBar : ToolBarBase {
}

struct wxEditableListBox(*u8);
impl EditableListBox for wxEditableListBox {}
impl Panel for wxEditableListBox {}
impl Window for wxEditableListBox {}
impl EvtHandler for wxEditableListBox {}
impl Object for wxEditableListBox { fn handle(&self) -> *u8 { **self } }

impl wxEditableListBox {
}

trait EditableListBox : Panel {
}

struct wxEncodingConverter(*u8);
impl EncodingConverter for wxEncodingConverter {}
impl Object for wxEncodingConverter { fn handle(&self) -> *u8 { **self } }

impl wxEncodingConverter {
    #[fixed_stack_segment]
    pub fn new() -> @EncodingConverter {
        unsafe { @wxEncodingConverter(wxEncodingConverter_Create()) as @EncodingConverter }
    }
}

trait EncodingConverter : Object {
    #[fixed_stack_segment]
    fn convert(&self, input: *u8, output: *u8) {
        unsafe { wxEncodingConverter_Convert(self.handle(), input, output) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxEncodingConverter_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getAllEquivalents(&self, enc: c_int, _lst: @List) -> c_int {
        unsafe { wxEncodingConverter_GetAllEquivalents(self.handle(), enc, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getPlatformEquivalents(&self, enc: c_int, platform: c_int, _lst: @List) -> c_int {
        unsafe { wxEncodingConverter_GetPlatformEquivalents(self.handle(), enc, platform, _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn init(&self, input_enc: c_int, output_enc: c_int, method: c_int) -> c_int {
        unsafe { wxEncodingConverter_Init(self.handle(), input_enc, output_enc, method) }
    }
}

struct wxEraseEvent(*u8);
impl EraseEvent for wxEraseEvent {}
impl Event for wxEraseEvent {}
impl Object for wxEraseEvent { fn handle(&self) -> *u8 { **self } }

impl wxEraseEvent {
}

trait EraseEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxEraseEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getDC(&self) -> @DC {
        unsafe { @wxDC(wxEraseEvent_GetDC(self.handle())) as @DC }
    }
}

struct wxEvent(*u8);
impl Event for wxEvent {}
impl Object for wxEvent { fn handle(&self) -> *u8 { **self } }

impl wxEvent {
    #[fixed_stack_segment]
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

trait Event : Object {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: *u8) {
        unsafe { wxEvent_CopyObject(self.handle(), object_dest) }
    }
    #[fixed_stack_segment]
    fn getEventObject(&self) -> @Object {
        unsafe { @wxObject(wxEvent_GetEventObject(self.handle())) as @Object }
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
    fn setEventObject(&self, obj: @Object) {
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
impl EvtHandler for wxEvtHandler {}
impl Object for wxEvtHandler { fn handle(&self) -> *u8 { **self } }

impl wxEvtHandler {
    #[fixed_stack_segment]
    pub fn new() -> @EvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_Create()) as @EvtHandler }
    }
}

trait EvtHandler : Object {
    #[fixed_stack_segment]
    fn addPendingEvent(&self, event: @Event) {
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
    fn getNextHandler(&self) -> @EvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_GetNextHandler(self.handle())) as @EvtHandler }
    }
    #[fixed_stack_segment]
    fn getPreviousHandler(&self) -> @EvtHandler {
        unsafe { @wxEvtHandler(wxEvtHandler_GetPreviousHandler(self.handle())) as @EvtHandler }
    }
    #[fixed_stack_segment]
    fn processEvent(&self, event: @Event) -> bool {
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
    fn setNextHandler(&self, handler: @EvtHandler) {
        unsafe { wxEvtHandler_SetNextHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setPreviousHandler(&self, handler: @EvtHandler) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self, id: c_int, type_: c_int) -> @Closure {
        unsafe { @wxClosure(wxEvtHandler_GetClosure(self.handle(), id, type_)) as @Closure }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @Closure {
        unsafe { @wxClosure(wxEvtHandler_GetClientClosure(self.handle())) as @Closure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @Closure) {
        unsafe { wxEvtHandler_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxExpr(*u8);
impl Expr for wxExpr { fn handle(&self) -> *u8 { **self } }

impl wxExpr {
}

trait Expr {
    fn handle(&self) -> *u8;
    
}

struct wxExprDatabase(*u8);
impl ExprDatabase for wxExprDatabase {}
impl List for wxExprDatabase {}
impl Object for wxExprDatabase { fn handle(&self) -> *u8 { **self } }

impl wxExprDatabase {
}

trait ExprDatabase : List {
}

struct wxFFile(*u8);
impl FFile for wxFFile { fn handle(&self) -> *u8 { **self } }

impl wxFFile {
}

trait FFile {
    fn handle(&self) -> *u8;
    
}

struct wxFFileInputStream(*u8);
impl FFileInputStream for wxFFileInputStream {}
impl InputStream for wxFFileInputStream {}
impl StreamBase for wxFFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileInputStream {
}

trait FFileInputStream : InputStream {
}

struct wxFFileOutputStream(*u8);
impl FFileOutputStream for wxFFileOutputStream {}
impl OutputStream for wxFFileOutputStream {}
impl StreamBase for wxFFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFFileOutputStream {
}

trait FFileOutputStream : OutputStream {
}

struct wxFSFile(*u8);
impl FSFile for wxFSFile {}
impl Object for wxFSFile { fn handle(&self) -> *u8 { **self } }

impl wxFSFile {
}

trait FSFile : Object {
}

struct wxFTP(*u8);
impl FTP for wxFTP {}
impl Protocol for wxFTP {}
impl SocketClient for wxFTP {}
impl SocketBase for wxFTP {}
impl Object for wxFTP { fn handle(&self) -> *u8 { **self } }

impl wxFTP {
}

trait FTP : Protocol {
}

struct wxFileDataObject(*u8);
impl FileDataObject for wxFileDataObject {}
impl DataObjectSimple for wxFileDataObject {}
impl DataObject for wxFileDataObject { fn handle(&self) -> *u8 { **self } }

impl wxFileDataObject {
}

trait FileDataObject : DataObjectSimple {
}

struct wxFileDialog(*u8);
impl FileDialog for wxFileDialog {}
impl Dialog for wxFileDialog {}
impl TopLevelWindow for wxFileDialog {}
impl Window for wxFileDialog {}
impl EvtHandler for wxFileDialog {}
impl Object for wxFileDialog { fn handle(&self) -> *u8 { **self } }

impl wxFileDialog {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _msg: @String, _dir: @String, _fle: @String, _wcd: @String, _lft: c_int, _top: c_int, _stl: c_int) -> @FileDialog {
        unsafe { @wxFileDialog(wxFileDialog_Create(_prt.handle(), _msg.handle(), _dir.handle(), _fle.handle(), _wcd.handle(), _lft, _top, _stl)) as @FileDialog }
    }
}

trait FileDialog : Dialog {
    #[fixed_stack_segment]
    fn getDirectory(&self) -> @String {
        unsafe { @wxString(wxFileDialog_GetDirectory(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getFilename(&self) -> @String {
        unsafe { @wxString(wxFileDialog_GetFilename(self.handle())) as @String }
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
    fn getMessage(&self) -> @String {
        unsafe { @wxString(wxFileDialog_GetMessage(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getPath(&self) -> @String {
        unsafe { @wxString(wxFileDialog_GetPath(self.handle())) as @String }
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
    fn getWildcard(&self) -> @String {
        unsafe { @wxString(wxFileDialog_GetWildcard(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setDirectory(&self, dir: @String) {
        unsafe { wxFileDialog_SetDirectory(self.handle(), dir.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilename(&self, name: @String) {
        unsafe { wxFileDialog_SetFilename(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setFilterIndex(&self, filterIndex: c_int) {
        unsafe { wxFileDialog_SetFilterIndex(self.handle(), filterIndex) }
    }
    #[fixed_stack_segment]
    fn setMessage(&self, message: @String) {
        unsafe { wxFileDialog_SetMessage(self.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setPath(&self, path: @String) {
        unsafe { wxFileDialog_SetPath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxFileDialog_SetStyle(self.handle(), style) }
    }
    #[fixed_stack_segment]
    fn setWildcard(&self, wildCard: @String) {
        unsafe { wxFileDialog_SetWildcard(self.handle(), wildCard.handle()) }
    }
}

struct wxFileDropTarget(*u8);
impl FileDropTarget for wxFileDropTarget {}
impl DropTarget for wxFileDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxFileDropTarget {
}

trait FileDropTarget : DropTarget {
}

struct wxFileHistory(*u8);
impl FileHistory for wxFileHistory {}
impl Object for wxFileHistory { fn handle(&self) -> *u8 { **self } }

impl wxFileHistory {
    #[fixed_stack_segment]
    pub fn new(maxFiles: c_int) -> @FileHistory {
        unsafe { @wxFileHistory(wxFileHistory_Create(maxFiles)) as @FileHistory }
    }
}

trait FileHistory : Object {
    #[fixed_stack_segment]
    fn addFileToHistory(&self, file: @String) {
        unsafe { wxFileHistory_AddFileToHistory(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn addFilesToMenu(&self, menu: @Menu) {
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
    fn getHistoryFile(&self, i: c_int) -> @String {
        unsafe { @wxString(wxFileHistory_GetHistoryFile(self.handle(), i)) as @String }
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
    fn load(&self, config: @ConfigBase) {
        unsafe { wxFileHistory_Load(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.handle(), i) }
    }
    #[fixed_stack_segment]
    fn removeMenu(&self, menu: @Menu) {
        unsafe { wxFileHistory_RemoveMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn save(&self, config: @ConfigBase) {
        unsafe { wxFileHistory_Save(self.handle(), config.handle()) }
    }
    #[fixed_stack_segment]
    fn useMenu(&self, menu: @Menu) {
        unsafe { wxFileHistory_UseMenu(self.handle(), menu.handle()) }
    }
}

struct wxFileInputStream(*u8);
impl FileInputStream for wxFileInputStream {}
impl InputStream for wxFileInputStream {}
impl StreamBase for wxFileInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileInputStream {
}

trait FileInputStream : InputStream {
}

struct wxFileName(*u8);
impl FileName for wxFileName { fn handle(&self) -> *u8 { **self } }

impl wxFileName {
}

trait FileName {
    fn handle(&self) -> *u8;
    
}

struct wxFileOutputStream(*u8);
impl FileOutputStream for wxFileOutputStream {}
impl OutputStream for wxFileOutputStream {}
impl StreamBase for wxFileOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFileOutputStream {
}

trait FileOutputStream : OutputStream {
}

struct wxFileSystem(*u8);
impl FileSystem for wxFileSystem {}
impl Object for wxFileSystem { fn handle(&self) -> *u8 { **self } }

impl wxFileSystem {
}

trait FileSystem : Object {
}

struct wxFileSystemHandler(*u8);
impl FileSystemHandler for wxFileSystemHandler {}
impl Object for wxFileSystemHandler { fn handle(&self) -> *u8 { **self } }

impl wxFileSystemHandler {
}

trait FileSystemHandler : Object {
}

struct wxFileType(*u8);
impl FileType for wxFileType { fn handle(&self) -> *u8 { **self } }

impl wxFileType {
}

trait FileType {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFileType_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn expandCommand(&self, _cmd: *u8, _params: *u8) -> @String {
        unsafe { @wxString(wxFileType_ExpandCommand(self.handle(), _cmd, _params)) as @String }
    }
    #[fixed_stack_segment]
    fn getDescription(&self) -> @String {
        unsafe { @wxString(wxFileType_GetDescription(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getExtensions(&self, _lst: @List) -> c_int {
        unsafe { wxFileType_GetExtensions(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, icon: @Icon) -> c_int {
        unsafe { wxFileType_GetIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn getMimeType(&self) -> @String {
        unsafe { @wxString(wxFileType_GetMimeType(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getMimeTypes(&self, _lst: @List) -> c_int {
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
impl FilterInputStream for wxFilterInputStream {}
impl InputStream for wxFilterInputStream {}
impl StreamBase for wxFilterInputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterInputStream {
}

trait FilterInputStream : InputStream {
}

struct wxFilterOutputStream(*u8);
impl FilterOutputStream for wxFilterOutputStream {}
impl OutputStream for wxFilterOutputStream {}
impl StreamBase for wxFilterOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxFilterOutputStream {
}

trait FilterOutputStream : OutputStream {
}

struct wxFindDialogEvent(*u8);
impl FindDialogEvent for wxFindDialogEvent {}
impl CommandEvent for wxFindDialogEvent {}
impl Event for wxFindDialogEvent {}
impl Object for wxFindDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxFindDialogEvent {
}

trait FindDialogEvent : CommandEvent {
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
impl FindReplaceData for wxFindReplaceData {}
impl Object for wxFindReplaceData { fn handle(&self) -> *u8 { **self } }

impl wxFindReplaceData {
    #[fixed_stack_segment]
    pub fn new(flags: c_int) -> @FindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceData_Create(flags)) as @FindReplaceData }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @FindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceData_CreateDefault()) as @FindReplaceData }
    }
}

trait FindReplaceData : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFindReplaceData_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFindString(&self) -> @String {
        unsafe { @wxString(wxFindReplaceData_GetFindString(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getFlags(&self) -> c_int {
        unsafe { wxFindReplaceData_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getReplaceString(&self) -> @String {
        unsafe { @wxString(wxFindReplaceData_GetReplaceString(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setFindString(&self, str: @String) {
        unsafe { wxFindReplaceData_SetFindString(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxFindReplaceData_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn setReplaceString(&self, str: @String) {
        unsafe { wxFindReplaceData_SetReplaceString(self.handle(), str.handle()) }
    }
}

struct wxFindReplaceDialog(*u8);
impl FindReplaceDialog for wxFindReplaceDialog {}
impl Dialog for wxFindReplaceDialog {}
impl TopLevelWindow for wxFindReplaceDialog {}
impl Window for wxFindReplaceDialog {}
impl EvtHandler for wxFindReplaceDialog {}
impl Object for wxFindReplaceDialog { fn handle(&self) -> *u8 { **self } }

impl wxFindReplaceDialog {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, data: @FindReplaceData, title: @String, style: c_int) -> @FindReplaceDialog {
        unsafe { @wxFindReplaceDialog(wxFindReplaceDialog_Create(parent.handle(), data.handle(), title.handle(), style)) as @FindReplaceDialog }
    }
}

trait FindReplaceDialog : Dialog {
    #[fixed_stack_segment]
    fn getData(&self) -> @FindReplaceData {
        unsafe { @wxFindReplaceData(wxFindReplaceDialog_GetData(self.handle())) as @FindReplaceData }
    }
    #[fixed_stack_segment]
    fn setData(&self, data: @FindReplaceData) {
        unsafe { wxFindReplaceDialog_SetData(self.handle(), data.handle()) }
    }
}

struct wxFlexGridSizer(*u8);
impl FlexGridSizer for wxFlexGridSizer {}
impl GridSizer for wxFlexGridSizer {}
impl Sizer for wxFlexGridSizer {}
impl Object for wxFlexGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxFlexGridSizer {
    #[fixed_stack_segment]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @FlexGridSizer {
        unsafe { @wxFlexGridSizer(wxFlexGridSizer_Create(rows, cols, vgap, hgap)) as @FlexGridSizer }
    }
}

trait FlexGridSizer : GridSizer {
    #[fixed_stack_segment]
    fn addGrowableCol(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableCol(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn addGrowableRow(&self, idx: size_t) {
        unsafe { wxFlexGridSizer_AddGrowableRow(self.handle(), idx) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @Size {
        unsafe { @wxSize(wxFlexGridSizer_CalcMin(self.handle())) as @Size }
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
impl FocusEvent for wxFocusEvent {}
impl Event for wxFocusEvent {}
impl Object for wxFocusEvent { fn handle(&self) -> *u8 { **self } }

impl wxFocusEvent {
}

trait FocusEvent : Event {
}

struct wxFont(*u8);
impl Font for wxFont {}
impl GDIObject for wxFont {}
impl Object for wxFont { fn handle(&self) -> *u8 { **self } }

impl wxFont {
    #[fixed_stack_segment]
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: bool, face: @String, enc: c_int) -> @Font {
        unsafe { @wxFont(wxFont_Create(pointSize, family, style, weight, underlined, face.handle(), enc)) as @Font }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @Font {
        unsafe { @wxFont(wxFont_CreateFromStock(id)) as @Font }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @Font {
        unsafe { @wxFont(wxFont_CreateDefault()) as @Font }
    }
}

trait Font : GDIObject {
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
    fn getFaceName(&self) -> @String {
        unsafe { @wxString(wxFont_GetFaceName(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getFamily(&self) -> c_int {
        unsafe { wxFont_GetFamily(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFamilyString(&self) -> @String {
        unsafe { @wxString(wxFont_GetFamilyString(self.handle())) as @String }
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
    fn getStyleString(&self) -> @String {
        unsafe { @wxString(wxFont_GetStyleString(self.handle())) as @String }
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
    fn getWeightString(&self) -> @String {
        unsafe { @wxString(wxFont_GetWeightString(self.handle())) as @String }
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
    fn setFaceName(&self, faceName: @String) {
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
impl FontData for wxFontData {}
impl Object for wxFontData { fn handle(&self) -> *u8 { **self } }

impl wxFontData {
    #[fixed_stack_segment]
    pub fn new() -> @FontData {
        unsafe { @wxFontData(wxFontData_Create()) as @FontData }
    }
}

trait FontData : Object {
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
    fn getChosenFont(&self, ref_: @Font) {
        unsafe { wxFontData_GetChosenFont(self.handle(), ref_.handle()) }
    }
    #[fixed_stack_segment]
    fn getColour(&self, _ref: @Colour) {
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
    fn getInitialFont(&self, ref_: @Font) {
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
    fn setChosenFont(&self, font: @Font) {
        unsafe { wxFontData_SetChosenFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setColour(&self, colour: @Colour) {
        unsafe { wxFontData_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.handle(), encoding) }
    }
    #[fixed_stack_segment]
    fn setInitialFont(&self, font: @Font) {
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
impl FontDialog for wxFontDialog {}
impl Dialog for wxFontDialog {}
impl TopLevelWindow for wxFontDialog {}
impl Window for wxFontDialog {}
impl EvtHandler for wxFontDialog {}
impl Object for wxFontDialog { fn handle(&self) -> *u8 { **self } }

impl wxFontDialog {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, fnt: @FontData) -> @FontDialog {
        unsafe { @wxFontDialog(wxFontDialog_Create(_prt.handle(), fnt.handle())) as @FontDialog }
    }
}

trait FontDialog : Dialog {
    #[fixed_stack_segment]
    fn getFontData(&self, _ref: @FontData) {
        unsafe { wxFontDialog_GetFontData(self.handle(), _ref.handle()) }
    }
}

struct wxFontEnumerator(*u8);
impl FontEnumerator for wxFontEnumerator { fn handle(&self) -> *u8 { **self } }

impl wxFontEnumerator {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _fnc: *u8) -> @FontEnumerator {
        unsafe { @wxFontEnumerator(wxFontEnumerator_Create(_obj, _fnc)) as @FontEnumerator }
    }
}

trait FontEnumerator {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxFontEnumerator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateEncodings(&self, facename: @String) -> bool {
        unsafe { wxFontEnumerator_EnumerateEncodings(self.handle(), facename.handle()) }
    }
    #[fixed_stack_segment]
    fn enumerateFacenames(&self, encoding: c_int, fixedWidthOnly: c_int) -> bool {
        unsafe { wxFontEnumerator_EnumerateFacenames(self.handle(), encoding, fixedWidthOnly) }
    }
}

struct wxFontList(*u8);
impl FontList for wxFontList {}
impl List for wxFontList {}
impl Object for wxFontList { fn handle(&self) -> *u8 { **self } }

impl wxFontList {
}

trait FontList : List {
}

struct wxFontMapper(*u8);
impl FontMapper for wxFontMapper { fn handle(&self) -> *u8 { **self } }

impl wxFontMapper {
    #[fixed_stack_segment]
    pub fn new() -> @FontMapper {
        unsafe { @wxFontMapper(wxFontMapper_Create()) as @FontMapper }
    }
}

trait FontMapper {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn getAltForEncoding(&self, encoding: c_int, alt_encoding: *u8, _buf: @String) -> bool {
        unsafe { wxFontMapper_GetAltForEncoding(self.handle(), encoding, alt_encoding, _buf.handle()) }
    }
    #[fixed_stack_segment]
    fn isEncodingAvailable(&self, encoding: c_int, _buf: @String) -> bool {
        unsafe { wxFontMapper_IsEncodingAvailable(self.handle(), encoding, _buf.handle()) }
    }
}

struct wxFrame(*u8);
impl Frame for wxFrame {}
impl TopLevelWindow for wxFrame {}
impl Window for wxFrame {}
impl EvtHandler for wxFrame {}
impl Object for wxFrame { fn handle(&self) -> *u8 { **self } }

impl wxFrame {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Frame {
        unsafe { @wxFrame(wxFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @Frame }
    }
}

trait Frame : TopLevelWindow {
    #[fixed_stack_segment]
    fn newStatusBar(&self, number: c_int, style: c_int) -> @StatusBar {
        unsafe { @wxStatusBar(wxFrame_CreateStatusBar(self.handle(), number, style)) as @StatusBar }
    }
    #[fixed_stack_segment]
    fn newToolBar(&self, style: c_long) -> @ToolBar {
        unsafe { @wxToolBar(wxFrame_CreateToolBar(self.handle(), style)) as @ToolBar }
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
    fn getMenuBar(&self) -> @MenuBar {
        unsafe { @wxMenuBar(wxFrame_GetMenuBar(self.handle())) as @MenuBar }
    }
    #[fixed_stack_segment]
    fn getStatusBar(&self) -> @StatusBar {
        unsafe { @wxStatusBar(wxFrame_GetStatusBar(self.handle())) as @StatusBar }
    }
    #[fixed_stack_segment]
    fn getToolBar(&self) -> @ToolBar {
        unsafe { @wxToolBar(wxFrame_GetToolBar(self.handle())) as @ToolBar }
    }
    #[fixed_stack_segment]
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setMenuBar(&self, menubar: @MenuBar) {
        unsafe { wxFrame_SetMenuBar(self.handle(), menubar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusBar(&self, statBar: @StatusBar) {
        unsafe { wxFrame_SetStatusBar(self.handle(), statBar.handle()) }
    }
    #[fixed_stack_segment]
    fn setStatusText(&self, _txt: @String, _number: c_int) {
        unsafe { wxFrame_SetStatusText(self.handle(), _txt.handle(), _number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, _n: c_int, _widths_field: *u8) {
        unsafe { wxFrame_SetStatusWidths(self.handle(), _n, _widths_field) }
    }
    #[fixed_stack_segment]
    fn setToolBar(&self, _toolbar: @ToolBar) {
        unsafe { wxFrame_SetToolBar(self.handle(), _toolbar.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @String {
        unsafe { @wxString(wxFrame_GetTitle(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, _txt: @String) {
        unsafe { wxFrame_SetTitle(self.handle(), _txt.handle()) }
    }
    #[fixed_stack_segment]
    fn setShape(&self, region: @Region) -> bool {
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
impl FrameLayout for wxFrameLayout {}
impl EvtHandler for wxFrameLayout {}
impl Object for wxFrameLayout { fn handle(&self) -> *u8 { **self } }

impl wxFrameLayout {
}

trait FrameLayout : EvtHandler {
}

struct wxGDIObject(*u8);
impl GDIObject for wxGDIObject {}
impl Object for wxGDIObject { fn handle(&self) -> *u8 { **self } }

impl wxGDIObject {
}

trait GDIObject : Object {
}

struct wxGLCanvas(*u8);
impl GLCanvas for wxGLCanvas {}
impl ScrolledWindow for wxGLCanvas {}
impl Panel for wxGLCanvas {}
impl Window for wxGLCanvas {}
impl EvtHandler for wxGLCanvas {}
impl Object for wxGLCanvas { fn handle(&self) -> *u8 { **self } }

impl wxGLCanvas {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, windowID: c_int, attributes: *c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int, title: @String, palette: @Palette) -> @GLCanvas {
        unsafe { @wxGLCanvas(wxGLCanvas_Create(parent.handle(), windowID, attributes, x, y, w, h, style, title.handle(), palette.handle())) as @GLCanvas }
    }
    #[fixed_stack_segment]
    pub fn isDisplaySupported(attributes: *c_int) -> bool {
        unsafe { wxGLCanvas_IsDisplaySupported(attributes) }
    }
    #[fixed_stack_segment]
    pub fn isExtensionSupported(extension: @String) -> bool {
        unsafe { wxGLCanvas_IsExtensionSupported(extension.handle()) }
    }
}

trait GLCanvas : ScrolledWindow {
    #[fixed_stack_segment]
    fn setColour(&self, colour: @Colour) -> bool {
        unsafe { wxGLCanvas_SetColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrent(&self, ctxt: @GLContext) -> bool {
        unsafe { wxGLCanvas_SetCurrent(self.handle(), ctxt.handle()) }
    }
    #[fixed_stack_segment]
    fn swapBuffers(&self) -> bool {
        unsafe { wxGLCanvas_SwapBuffers(self.handle()) }
    }
}

struct wxGauge(*u8);
impl Gauge for wxGauge {}
impl Control for wxGauge {}
impl Window for wxGauge {}
impl EvtHandler for wxGauge {}
impl Object for wxGauge { fn handle(&self) -> *u8 { **self } }

impl wxGauge {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Gauge {
        unsafe { @wxGauge(wxGauge_Create(_prt.handle(), _id, _rng, _lft, _top, _wdt, _hgt, _stl)) as @Gauge }
    }
}

trait Gauge : Control {
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
impl GenericDirCtrl for wxGenericDirCtrl {}
impl Control for wxGenericDirCtrl {}
impl Window for wxGenericDirCtrl {}
impl EvtHandler for wxGenericDirCtrl {}
impl Object for wxGenericDirCtrl { fn handle(&self) -> *u8 { **self } }

impl wxGenericDirCtrl {
}

trait GenericDirCtrl : Control {
}

struct wxGenericValidator(*u8);
impl GenericValidator for wxGenericValidator {}
impl Validator for wxGenericValidator {}
impl EvtHandler for wxGenericValidator {}
impl Object for wxGenericValidator { fn handle(&self) -> *u8 { **self } }

impl wxGenericValidator {
}

trait GenericValidator : Validator {
}

struct wxGrid(*u8);
impl Grid for wxGrid {}
impl ScrolledWindow for wxGrid {}
impl Panel for wxGrid {}
impl Window for wxGrid {}
impl EvtHandler for wxGrid {}
impl Object for wxGrid { fn handle(&self) -> *u8 { **self } }

impl wxGrid {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Grid {
        unsafe { @wxGrid(wxGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @Grid }
    }
}

trait Grid : ScrolledWindow {
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
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> @Rect {
        unsafe { @wxRect(wxGrid_BlockToDeviceRect(self.handle(), top, left, bottom, right)) as @Rect }
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
    fn cellToRect(&self, row: c_int, col: c_int) -> @Rect {
        unsafe { @wxRect(wxGrid_CellToRect(self.handle(), row, col)) as @Rect }
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
    fn drawAllGridLines(&self, dc: @DC, reg: @Region) {
        unsafe { wxGrid_DrawAllGridLines(self.handle(), dc.handle(), reg.handle()) }
    }
    #[fixed_stack_segment]
    fn drawCell(&self, dc: @DC, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellBorder(&self, dc: @DC, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.handle(), dc.handle(), _row, _col) }
    }
    #[fixed_stack_segment]
    fn drawCellHighlight(&self, dc: @DC, attr: @GridCellAttr) {
        unsafe { wxGrid_DrawCellHighlight(self.handle(), dc.handle(), attr.handle()) }
    }
    #[fixed_stack_segment]
    fn drawColLabel(&self, dc: @DC, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.handle(), dc.handle(), col) }
    }
    #[fixed_stack_segment]
    fn drawColLabels(&self, dc: @DC) {
        unsafe { wxGrid_DrawColLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawGridSpace(&self, dc: @DC) {
        unsafe { wxGrid_DrawGridSpace(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawRowLabel(&self, dc: @DC, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.handle(), dc.handle(), row) }
    }
    #[fixed_stack_segment]
    fn drawRowLabels(&self, dc: @DC) {
        unsafe { wxGrid_DrawRowLabels(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn drawTextRectangle(&self, dc: @DC, txt: @String, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
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
    fn getCellBackgroundColour(&self, row: c_int, col: c_int, colour: @Colour) {
        unsafe { wxGrid_GetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellEditor(&self, row: c_int, col: c_int) -> @GridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetCellEditor(self.handle(), row, col)) as @GridCellEditor }
    }
    #[fixed_stack_segment]
    fn getCellFont(&self, row: c_int, col: c_int, font: @Font) {
        unsafe { wxGrid_GetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellHighlightColour(&self, _ref: @Colour) {
        unsafe { wxGrid_GetCellHighlightColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellRenderer(&self, row: c_int, col: c_int) -> @GridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetCellRenderer(self.handle(), row, col)) as @GridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getCellTextColour(&self, row: c_int, col: c_int, colour: @Colour) {
        unsafe { wxGrid_GetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getCellValue(&self, row: c_int, col: c_int) -> @String {
        unsafe { @wxString(wxGrid_GetCellValue(self.handle(), row, col)) as @String }
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
    fn getColLabelValue(&self, col: c_int) -> @String {
        unsafe { @wxString(wxGrid_GetColLabelValue(self.handle(), col)) as @String }
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
    fn getDefaultCellBackgroundColour(&self, _ref: @Colour) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellFont(&self, _ref: @Font) {
        unsafe { wxGrid_GetDefaultCellFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultCellTextColour(&self, _ref: @Colour) {
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
    fn getDefaultEditor(&self) -> @GridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditor(self.handle())) as @GridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> @GridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditorForCell(self.handle(), row, col)) as @GridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultEditorForType(&self, typeName: @String) -> @GridCellEditor {
        unsafe { @wxGridCellEditor(wxGrid_GetDefaultEditorForType(self.handle(), typeName.handle())) as @GridCellEditor }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @GridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRenderer(self.handle())) as @GridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> @GridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRendererForCell(self.handle(), row, col)) as @GridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getDefaultRendererForType(&self, typeName: @String) -> @GridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGrid_GetDefaultRendererForType(self.handle(), typeName.handle())) as @GridCellRenderer }
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
    fn getGridLineColour(&self, _ref: @Colour) {
        unsafe { wxGrid_GetGridLineColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelBackgroundColour(&self, _ref: @Colour) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelFont(&self, _ref: @Font) {
        unsafe { wxGrid_GetLabelFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabelTextColour(&self, _ref: @Colour) {
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
    fn getRowLabelValue(&self, row: c_int) -> @String {
        unsafe { @wxString(wxGrid_GetRowLabelValue(self.handle(), row)) as @String }
    }
    #[fixed_stack_segment]
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.handle(), row) }
    }
    #[fixed_stack_segment]
    fn getSelectionBackground(&self, _ref: @Colour) {
        unsafe { wxGrid_GetSelectionBackground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionForeground(&self, _ref: @Colour) {
        unsafe { wxGrid_GetSelectionForeground(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getTable(&self) -> @GridTableBase {
        unsafe { @wxGridTableBase(wxGrid_GetTable(self.handle())) as @GridTableBase }
    }
    #[fixed_stack_segment]
    fn getTextBoxSize(&self, dc: @DC, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int) {
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
    fn processTableMessage(&self, evt: @Event) -> bool {
        unsafe { wxGrid_ProcessTableMessage(self.handle(), evt.handle()) }
    }
    #[fixed_stack_segment]
    fn registerDataType(&self, typeName: @String, renderer: @GridCellRenderer, editor: @GridCellEditor) {
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
    fn setCellBackgroundColour(&self, row: c_int, col: c_int, colour: @Colour) {
        unsafe { wxGrid_SetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellEditor(&self, row: c_int, col: c_int, editor: @GridCellEditor) {
        unsafe { wxGrid_SetCellEditor(self.handle(), row, col, editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellFont(&self, row: c_int, col: c_int, font: @Font) {
        unsafe { wxGrid_SetCellFont(self.handle(), row, col, font.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellHighlightColour(&self, col: @Colour) {
        unsafe { wxGrid_SetCellHighlightColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellRenderer(&self, row: c_int, col: c_int, renderer: @GridCellRenderer) {
        unsafe { wxGrid_SetCellRenderer(self.handle(), row, col, renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellTextColour(&self, row: c_int, col: c_int, colour: @Colour) {
        unsafe { wxGrid_SetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCellValue(&self, row: c_int, col: c_int, s: @String) {
        unsafe { wxGrid_SetCellValue(self.handle(), row, col, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setColAttr(&self, col: c_int, attr: @GridCellAttr) {
        unsafe { wxGrid_SetColAttr(self.handle(), col, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.handle(), col) }
    }
    #[fixed_stack_segment]
    fn setColFormatCustom(&self, col: c_int, typeName: @String) {
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
    fn setColLabelValue(&self, col: c_int, label: @String) {
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
    fn setDefaultCellBackgroundColour(&self, colour: @Colour) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellFont(&self, font: @Font) {
        unsafe { wxGrid_SetDefaultCellFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultCellTextColour(&self, colour: @Colour) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.handle(), width, resizeExistingCols) }
    }
    #[fixed_stack_segment]
    fn setDefaultEditor(&self, editor: @GridCellEditor) {
        unsafe { wxGrid_SetDefaultEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultRenderer(&self, renderer: @GridCellRenderer) {
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
    fn setGridLineColour(&self, col: @Colour) {
        unsafe { wxGrid_SetGridLineColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelBackgroundColour(&self, colour: @Colour) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelFont(&self, font: @Font) {
        unsafe { wxGrid_SetLabelFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTextColour(&self, colour: @Colour) {
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
    fn setRowAttr(&self, row: c_int, attr: @GridCellAttr) {
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
    fn setRowLabelValue(&self, row: c_int, label: @String) {
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
    fn setSelectionBackground(&self, c: @Colour) {
        unsafe { wxGrid_SetSelectionBackground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionForeground(&self, c: @Colour) {
        unsafe { wxGrid_SetSelectionForeground(self.handle(), c.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.handle(), selmode) }
    }
    #[fixed_stack_segment]
    fn setTable(&self, table: @GridTableBase, takeOwnership: bool, selmode: c_int) -> bool {
        unsafe { wxGrid_SetTable(self.handle(), table.handle(), takeOwnership, selmode) }
    }
    #[fixed_stack_segment]
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.handle()) }
    }
    #[fixed_stack_segment]
    fn stringToLines(&self, value: @String, lines: *u8) -> c_int {
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
    fn getSelectedCells(&self, _arr: @GridCellCoordsArray) {
        unsafe { wxGrid_GetSelectedCells(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockTopLeft(&self, _arr: @GridCellCoordsArray) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.handle(), _arr.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelectionBlockBottomRight(&self, _arr: @GridCellCoordsArray) {
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
impl GridCellAttr for wxGridCellAttr { fn handle(&self) -> *u8 { **self } }

impl wxGridCellAttr {
    #[fixed_stack_segment]
    pub fn ctor() -> @GridCellAttr {
        unsafe { @wxGridCellAttr(wxGridCellAttr_Ctor()) as @GridCellAttr }
    }
}

trait GridCellAttr {
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
    fn getBackgroundColour(&self, _ref: @Colour) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditor(&self, grid: @Grid, row: c_int, col: c_int) -> @GridCellEditor {
        unsafe { @wxGridCellEditor(wxGridCellAttr_GetEditor(self.handle(), grid.handle(), row, col)) as @GridCellEditor }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @Font) {
        unsafe { wxGridCellAttr_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getRenderer(&self, grid: @Grid, row: c_int, col: c_int) -> @GridCellRenderer {
        unsafe { @wxGridCellRenderer(wxGridCellAttr_GetRenderer(self.handle(), grid.handle(), row, col)) as @GridCellRenderer }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @Colour) {
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
    fn setBackgroundColour(&self, colBack: @Colour) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefAttr(&self, defAttr: @GridCellAttr) {
        unsafe { wxGridCellAttr_SetDefAttr(self.handle(), defAttr.handle()) }
    }
    #[fixed_stack_segment]
    fn setEditor(&self, editor: @GridCellEditor) {
        unsafe { wxGridCellAttr_SetEditor(self.handle(), editor.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @Font) {
        unsafe { wxGridCellAttr_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setReadOnly(&self, isReadOnly: bool) {
        unsafe { wxGridCellAttr_SetReadOnly(self.handle(), isReadOnly) }
    }
    #[fixed_stack_segment]
    fn setRenderer(&self, renderer: @GridCellRenderer) {
        unsafe { wxGridCellAttr_SetRenderer(self.handle(), renderer.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @Colour) {
        unsafe { wxGridCellAttr_SetTextColour(self.handle(), colText.handle()) }
    }
}

struct wxGridCellBoolEditor(*u8);
impl GridCellBoolEditor for wxGridCellBoolEditor {}
impl GridCellEditor for wxGridCellBoolEditor {}
impl GridCellWorker for wxGridCellBoolEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolEditor {
    #[fixed_stack_segment]
    pub fn ctor() -> @GridCellBoolEditor {
        unsafe { @wxGridCellBoolEditor(wxGridCellBoolEditor_Ctor()) as @GridCellBoolEditor }
    }
}

trait GridCellBoolEditor : GridCellEditor {
}

struct wxGridCellBoolRenderer(*u8);
impl GridCellBoolRenderer for wxGridCellBoolRenderer {}
impl GridCellRenderer for wxGridCellBoolRenderer {}
impl GridCellWorker for wxGridCellBoolRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellBoolRenderer {
}

trait GridCellBoolRenderer : GridCellRenderer {
}

struct wxGridCellChoiceEditor(*u8);
impl GridCellChoiceEditor for wxGridCellChoiceEditor {}
impl GridCellEditor for wxGridCellChoiceEditor {}
impl GridCellWorker for wxGridCellChoiceEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellChoiceEditor {
    #[fixed_stack_segment]
    pub fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> @GridCellChoiceEditor {
        unsafe { @wxGridCellChoiceEditor(wxGridCellChoiceEditor_Ctor(count, choices, allowOthers)) as @GridCellChoiceEditor }
    }
}

trait GridCellChoiceEditor : GridCellEditor {
}

struct wxGridCellCoordsArray(*u8);
impl GridCellCoordsArray for wxGridCellCoordsArray { fn handle(&self) -> *u8 { **self } }

impl wxGridCellCoordsArray {
    #[fixed_stack_segment]
    pub fn new() -> @GridCellCoordsArray {
        unsafe { @wxGridCellCoordsArray(wxGridCellCoordsArray_Create()) as @GridCellCoordsArray }
    }
}

trait GridCellCoordsArray {
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
impl GridCellEditor for wxGridCellEditor {}
impl GridCellWorker for wxGridCellEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellEditor {
}

trait GridCellEditor : GridCellWorker {
    #[fixed_stack_segment]
    fn beginEdit(&self, row: c_int, col: c_int, grid: @Grid) {
        unsafe { wxGridCellEditor_BeginEdit(self.handle(), row, col, grid.handle()) }
    }
    #[fixed_stack_segment]
    fn new(&self, parent: @Window, id: c_int, evtHandler: @EvtHandler) {
        unsafe { wxGridCellEditor_Create(self.handle(), parent.handle(), id, evtHandler.handle()) }
    }
    #[fixed_stack_segment]
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.handle()) }
    }
    #[fixed_stack_segment]
    fn endEdit(&self, row: c_int, col: c_int, grid: @Grid, oldStr: @String, newStr: @String) -> c_int {
        unsafe { wxGridCellEditor_EndEdit(self.handle(), row, col, grid.handle(), oldStr.handle(), newStr.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @Control {
        unsafe { @wxControl(wxGridCellEditor_GetControl(self.handle())) as @Control }
    }
    #[fixed_stack_segment]
    fn handleReturn(&self, event: @Event) {
        unsafe { wxGridCellEditor_HandleReturn(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isAcceptedKey(&self, event: @Event) -> bool {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn isCreated(&self) -> bool {
        unsafe { wxGridCellEditor_IsCreated(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paintBackground(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: @GridCellAttr) {
        unsafe { wxGridCellEditor_PaintBackground(self.handle(), x, y, w, h, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setControl(&self, control: @Control) {
        unsafe { wxGridCellEditor_SetControl(self.handle(), control.handle()) }
    }
    #[fixed_stack_segment]
    fn setParameters(&self, params: @String) {
        unsafe { wxGridCellEditor_SetParameters(self.handle(), params.handle()) }
    }
    #[fixed_stack_segment]
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn show(&self, show: c_int, attr: @GridCellAttr) {
        unsafe { wxGridCellEditor_Show(self.handle(), show, attr.handle()) }
    }
    #[fixed_stack_segment]
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.handle()) }
    }
    #[fixed_stack_segment]
    fn startingKey(&self, event: @Event) {
        unsafe { wxGridCellEditor_StartingKey(self.handle(), event.handle()) }
    }
}

struct wxGridCellFloatEditor(*u8);
impl GridCellFloatEditor for wxGridCellFloatEditor {}
impl GridCellTextEditor for wxGridCellFloatEditor {}
impl GridCellEditor for wxGridCellFloatEditor {}
impl GridCellWorker for wxGridCellFloatEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellFloatEditor {
    #[fixed_stack_segment]
    pub fn ctor(width: c_int, precision: c_int) -> @GridCellFloatEditor {
        unsafe { @wxGridCellFloatEditor(wxGridCellFloatEditor_Ctor(width, precision)) as @GridCellFloatEditor }
    }
}

trait GridCellFloatEditor : GridCellTextEditor {
}

struct wxGridCellFloatRenderer(*u8);
impl GridCellFloatRenderer for wxGridCellFloatRenderer {}
impl GridCellStringRenderer for wxGridCellFloatRenderer {}
impl GridCellRenderer for wxGridCellFloatRenderer {}
impl GridCellWorker for wxGridCellFloatRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellFloatRenderer {
}

trait GridCellFloatRenderer : GridCellStringRenderer {
}

struct wxGridCellNumberEditor(*u8);
impl GridCellNumberEditor for wxGridCellNumberEditor {}
impl GridCellTextEditor for wxGridCellNumberEditor {}
impl GridCellEditor for wxGridCellNumberEditor {}
impl GridCellWorker for wxGridCellNumberEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellNumberEditor {
    #[fixed_stack_segment]
    pub fn ctor(min: c_int, max: c_int) -> @GridCellNumberEditor {
        unsafe { @wxGridCellNumberEditor(wxGridCellNumberEditor_Ctor(min, max)) as @GridCellNumberEditor }
    }
}

trait GridCellNumberEditor : GridCellTextEditor {
}

struct wxGridCellNumberRenderer(*u8);
impl GridCellNumberRenderer for wxGridCellNumberRenderer {}
impl GridCellStringRenderer for wxGridCellNumberRenderer {}
impl GridCellRenderer for wxGridCellNumberRenderer {}
impl GridCellWorker for wxGridCellNumberRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellNumberRenderer {
    #[fixed_stack_segment]
    pub fn ctor() -> @GridCellNumberRenderer {
        unsafe { @wxGridCellNumberRenderer(wxGridCellNumberRenderer_Ctor()) as @GridCellNumberRenderer }
    }
}

trait GridCellNumberRenderer : GridCellStringRenderer {
}

struct wxGridCellAutoWrapStringRenderer(*u8);
impl GridCellAutoWrapStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl GridCellStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl GridCellRenderer for wxGridCellAutoWrapStringRenderer {}
impl GridCellWorker for wxGridCellAutoWrapStringRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellAutoWrapStringRenderer {
    #[fixed_stack_segment]
    pub fn ctor() -> @GridCellAutoWrapStringRenderer {
        unsafe { @wxGridCellAutoWrapStringRenderer(wxGridCellAutoWrapStringRenderer_Ctor()) as @GridCellAutoWrapStringRenderer }
    }
}

trait GridCellAutoWrapStringRenderer : GridCellStringRenderer {
}

struct wxGridCellRenderer(*u8);
impl GridCellRenderer for wxGridCellRenderer {}
impl GridCellWorker for wxGridCellRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellRenderer {
}

trait GridCellRenderer : GridCellWorker {
}

struct wxGridCellStringRenderer(*u8);
impl GridCellStringRenderer for wxGridCellStringRenderer {}
impl GridCellRenderer for wxGridCellStringRenderer {}
impl GridCellWorker for wxGridCellStringRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGridCellStringRenderer {
}

trait GridCellStringRenderer : GridCellRenderer {
}

struct wxGridCellTextEditor(*u8);
impl GridCellTextEditor for wxGridCellTextEditor {}
impl GridCellEditor for wxGridCellTextEditor {}
impl GridCellWorker for wxGridCellTextEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellTextEditor {
    #[fixed_stack_segment]
    pub fn ctor() -> @GridCellTextEditor {
        unsafe { @wxGridCellTextEditor(wxGridCellTextEditor_Ctor()) as @GridCellTextEditor }
    }
}

trait GridCellTextEditor : GridCellEditor {
}

struct wxGridCellWorker(*u8);
impl GridCellWorker for wxGridCellWorker { fn handle(&self) -> *u8 { **self } }

impl wxGridCellWorker {
}

trait GridCellWorker {
    fn handle(&self) -> *u8;
    
}

struct wxGridEditorCreatedEvent(*u8);
impl GridEditorCreatedEvent for wxGridEditorCreatedEvent {}
impl CommandEvent for wxGridEditorCreatedEvent {}
impl Event for wxGridEditorCreatedEvent {}
impl Object for wxGridEditorCreatedEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridEditorCreatedEvent {
}

trait GridEditorCreatedEvent : CommandEvent {
    #[fixed_stack_segment]
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getControl(&self) -> @Control {
        unsafe { @wxControl(wxGridEditorCreatedEvent_GetControl(self.handle())) as @Control }
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
    fn setControl(&self, ctrl: @Control) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.handle(), row) }
    }
}

struct wxGridEvent(*u8);
impl GridEvent for wxGridEvent {}
impl NotifyEvent for wxGridEvent {}
impl CommandEvent for wxGridEvent {}
impl Event for wxGridEvent {}
impl Object for wxGridEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridEvent {
}

trait GridEvent : NotifyEvent {
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
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxGridEvent_GetPosition(self.handle())) as @Point }
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
impl GridRangeSelectEvent for wxGridRangeSelectEvent {}
impl NotifyEvent for wxGridRangeSelectEvent {}
impl CommandEvent for wxGridRangeSelectEvent {}
impl Event for wxGridRangeSelectEvent {}
impl Object for wxGridRangeSelectEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridRangeSelectEvent {
}

trait GridRangeSelectEvent : NotifyEvent {
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
impl GridSizeEvent for wxGridSizeEvent {}
impl NotifyEvent for wxGridSizeEvent {}
impl CommandEvent for wxGridSizeEvent {}
impl Event for wxGridSizeEvent {}
impl Object for wxGridSizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxGridSizeEvent {
}

trait GridSizeEvent : NotifyEvent {
    #[fixed_stack_segment]
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxGridSizeEvent_GetPosition(self.handle())) as @Point }
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
impl GridSizer for wxGridSizer {}
impl Sizer for wxGridSizer {}
impl Object for wxGridSizer { fn handle(&self) -> *u8 { **self } }

impl wxGridSizer {
    #[fixed_stack_segment]
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> @GridSizer {
        unsafe { @wxGridSizer(wxGridSizer_Create(rows, cols, vgap, hgap)) as @GridSizer }
    }
}

trait GridSizer : Sizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @Size {
        unsafe { @wxSize(wxGridSizer_CalcMin(self.handle())) as @Size }
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
impl GridTableBase for wxGridTableBase {}
impl Object for wxGridTableBase { fn handle(&self) -> *u8 { **self } }

impl wxGridTableBase {
}

trait GridTableBase : Object {
}

struct wxHTTP(*u8);
impl HTTP for wxHTTP {}
impl Protocol for wxHTTP {}
impl SocketClient for wxHTTP {}
impl SocketBase for wxHTTP {}
impl Object for wxHTTP { fn handle(&self) -> *u8 { **self } }

impl wxHTTP {
}

trait HTTP : Protocol {
}

struct wxHashMap(*u8);
impl HashMap for wxHashMap { fn handle(&self) -> *u8 { **self } }

impl wxHashMap {
}

trait HashMap {
    fn handle(&self) -> *u8;
    
}

struct wxHelpController(*u8);
impl HelpController for wxHelpController {}
impl HelpControllerBase for wxHelpController {}
impl Object for wxHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHelpController {
}

trait HelpController : HelpControllerBase {
}

struct wxHelpControllerBase(*u8);
impl HelpControllerBase for wxHelpControllerBase {}
impl Object for wxHelpControllerBase { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerBase {
}

trait HelpControllerBase : Object {
}

struct wxHelpControllerHelpProvider(*u8);
impl HelpControllerHelpProvider for wxHelpControllerHelpProvider {}
impl SimpleHelpProvider for wxHelpControllerHelpProvider {}
impl HelpProvider for wxHelpControllerHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpControllerHelpProvider {
    #[fixed_stack_segment]
    pub fn new(ctr: @HelpControllerBase) -> @HelpControllerHelpProvider {
        unsafe { @wxHelpControllerHelpProvider(wxHelpControllerHelpProvider_Create(ctr.handle())) as @HelpControllerHelpProvider }
    }
}

trait HelpControllerHelpProvider : SimpleHelpProvider {
    #[fixed_stack_segment]
    fn getHelpController(&self) -> @HelpControllerBase {
        unsafe { @wxHelpControllerBase(wxHelpControllerHelpProvider_GetHelpController(self.handle())) as @HelpControllerBase }
    }
    #[fixed_stack_segment]
    fn setHelpController(&self, hc: @HelpController) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.handle(), hc.handle()) }
    }
}

struct wxHelpEvent(*u8);
impl HelpEvent for wxHelpEvent {}
impl CommandEvent for wxHelpEvent {}
impl Event for wxHelpEvent {}
impl Object for wxHelpEvent { fn handle(&self) -> *u8 { **self } }

impl wxHelpEvent {
}

trait HelpEvent : CommandEvent {
    #[fixed_stack_segment]
    fn getLink(&self) -> @String {
        unsafe { @wxString(wxHelpEvent_GetLink(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxHelpEvent_GetPosition(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @String {
        unsafe { @wxString(wxHelpEvent_GetTarget(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setLink(&self, link: @String) {
        unsafe { wxHelpEvent_SetLink(self.handle(), link.handle()) }
    }
    #[fixed_stack_segment]
    fn setPosition(&self, x: c_int, y: c_int) {
        unsafe { wxHelpEvent_SetPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn setTarget(&self, target: @String) {
        unsafe { wxHelpEvent_SetTarget(self.handle(), target.handle()) }
    }
}

struct wxHelpProvider(*u8);
impl HelpProvider for wxHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxHelpProvider {
    #[fixed_stack_segment]
    pub fn get() -> @HelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Get()) as @HelpProvider }
    }
}

trait HelpProvider {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addHelp(&self, window: @Window, text: @String) {
        unsafe { wxHelpProvider_AddHelp(self.handle(), window.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addHelpById(&self, id: c_int, text: @String) {
        unsafe { wxHelpProvider_AddHelpById(self.handle(), id, text.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxHelpProvider_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelp(&self, window: @Window) -> @String {
        unsafe { @wxString(wxHelpProvider_GetHelp(self.handle(), window.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn removeHelp(&self, window: @Window) {
        unsafe { wxHelpProvider_RemoveHelp(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self) -> @HelpProvider {
        unsafe { @wxHelpProvider(wxHelpProvider_Set(self.handle())) as @HelpProvider }
    }
    #[fixed_stack_segment]
    fn showHelp(&self, window: @Window) -> bool {
        unsafe { wxHelpProvider_ShowHelp(self.handle(), window.handle()) }
    }
}

struct wxHtmlCell(*u8);
impl HtmlCell for wxHtmlCell {}
impl Object for wxHtmlCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlCell {
}

trait HtmlCell : Object {
}

struct wxHtmlColourCell(*u8);
impl HtmlColourCell for wxHtmlColourCell {}
impl HtmlCell for wxHtmlColourCell {}
impl Object for wxHtmlColourCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlColourCell {
}

trait HtmlColourCell : HtmlCell {
}

struct wxHtmlContainerCell(*u8);
impl HtmlContainerCell for wxHtmlContainerCell {}
impl HtmlCell for wxHtmlContainerCell {}
impl Object for wxHtmlContainerCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlContainerCell {
}

trait HtmlContainerCell : HtmlCell {
}

struct wxHtmlDCRenderer(*u8);
impl HtmlDCRenderer for wxHtmlDCRenderer {}
impl Object for wxHtmlDCRenderer { fn handle(&self) -> *u8 { **self } }

impl wxHtmlDCRenderer {
}

trait HtmlDCRenderer : Object {
}

struct wxHtmlEasyPrinting(*u8);
impl HtmlEasyPrinting for wxHtmlEasyPrinting {}
impl Object for wxHtmlEasyPrinting { fn handle(&self) -> *u8 { **self } }

impl wxHtmlEasyPrinting {
}

trait HtmlEasyPrinting : Object {
}

struct wxHtmlFilter(*u8);
impl HtmlFilter for wxHtmlFilter {}
impl Object for wxHtmlFilter { fn handle(&self) -> *u8 { **self } }

impl wxHtmlFilter {
}

trait HtmlFilter : Object {
}

struct wxHtmlHelpController(*u8);
impl HtmlHelpController for wxHtmlHelpController {}
impl HelpControllerBase for wxHtmlHelpController {}
impl Object for wxHtmlHelpController { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpController {
    #[fixed_stack_segment]
    pub fn new(_style: c_int) -> @HtmlHelpController {
        unsafe { @wxHtmlHelpController(wxHtmlHelpController_Create(_style)) as @HtmlHelpController }
    }
}

trait HtmlHelpController : HelpControllerBase {
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
    fn displaySection(&self, section: @String) -> bool {
        unsafe { wxHtmlHelpController_DisplaySection(self.handle(), section.handle()) }
    }
    #[fixed_stack_segment]
    fn displaySectionNumber(&self, sectionNo: c_int) -> bool {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.handle(), sectionNo) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @Frame {
        unsafe { @wxFrame(wxHtmlHelpController_GetFrame(self.handle())) as @Frame }
    }
    #[fixed_stack_segment]
    fn getFrameParameters(&self, title: *u8, width: *c_int, height: *c_int, pos_x: *c_int, pos_y: *c_int, newFrameEachTime: *c_int) -> *u8 {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn initialize(&self, file: @String) -> bool {
        unsafe { wxHtmlHelpController_Initialize(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn keywordSearch(&self, keyword: @String) -> bool {
        unsafe { wxHtmlHelpController_KeywordSearch(self.handle(), keyword.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, file: @String) -> bool {
        unsafe { wxHtmlHelpController_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    fn quit(&self) -> bool {
        unsafe { wxHtmlHelpController_Quit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @ConfigBase, path: @String) {
        unsafe { wxHtmlHelpController_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setFrameParameters(&self, title: *u8, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: bool) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    fn setTempDir(&self, path: @String) {
        unsafe { wxHtmlHelpController_SetTempDir(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitleFormat(&self, format: *u8) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.handle(), format) }
    }
    #[fixed_stack_segment]
    fn setViewer(&self, viewer: @String, flags: c_int) {
        unsafe { wxHtmlHelpController_SetViewer(self.handle(), viewer.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn useConfig(&self, config: @ConfigBase, rootpath: @String) {
        unsafe { wxHtmlHelpController_UseConfig(self.handle(), config.handle(), rootpath.handle()) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @ConfigBase, path: @String) {
        unsafe { wxHtmlHelpController_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxHtmlHelpData(*u8);
impl HtmlHelpData for wxHtmlHelpData {}
impl Object for wxHtmlHelpData { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpData {
}

trait HtmlHelpData : Object {
}

struct wxHtmlHelpFrame(*u8);
impl HtmlHelpFrame for wxHtmlHelpFrame {}
impl Frame for wxHtmlHelpFrame {}
impl TopLevelWindow for wxHtmlHelpFrame {}
impl Window for wxHtmlHelpFrame {}
impl EvtHandler for wxHtmlHelpFrame {}
impl Object for wxHtmlHelpFrame { fn handle(&self) -> *u8 { **self } }

impl wxHtmlHelpFrame {
}

trait HtmlHelpFrame : Frame {
}

struct wxHtmlLinkInfo(*u8);
impl HtmlLinkInfo for wxHtmlLinkInfo {}
impl Object for wxHtmlLinkInfo { fn handle(&self) -> *u8 { **self } }

impl wxHtmlLinkInfo {
}

trait HtmlLinkInfo : Object {
}

struct wxHtmlParser(*u8);
impl HtmlParser for wxHtmlParser {}
impl Object for wxHtmlParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlParser {
}

trait HtmlParser : Object {
}

struct wxHtmlPrintout(*u8);
impl HtmlPrintout for wxHtmlPrintout {}
impl Printout for wxHtmlPrintout {}
impl Object for wxHtmlPrintout { fn handle(&self) -> *u8 { **self } }

impl wxHtmlPrintout {
}

trait HtmlPrintout : Printout {
}

struct wxHtmlTag(*u8);
impl HtmlTag for wxHtmlTag {}
impl Object for wxHtmlTag { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTag {
}

trait HtmlTag : Object {
}

struct wxHtmlTagHandler(*u8);
impl HtmlTagHandler for wxHtmlTagHandler {}
impl Object for wxHtmlTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagHandler {
}

trait HtmlTagHandler : Object {
}

struct wxHtmlTagsModule(*u8);
impl HtmlTagsModule for wxHtmlTagsModule {}
impl Module for wxHtmlTagsModule {}
impl Object for wxHtmlTagsModule { fn handle(&self) -> *u8 { **self } }

impl wxHtmlTagsModule {
}

trait HtmlTagsModule : Module {
}

struct wxHtmlWidgetCell(*u8);
impl HtmlWidgetCell for wxHtmlWidgetCell {}
impl HtmlCell for wxHtmlWidgetCell {}
impl Object for wxHtmlWidgetCell { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWidgetCell {
}

trait HtmlWidgetCell : HtmlCell {
}

struct wxHtmlWinParser(*u8);
impl HtmlWinParser for wxHtmlWinParser {}
impl HtmlParser for wxHtmlWinParser {}
impl Object for wxHtmlWinParser { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinParser {
}

trait HtmlWinParser : HtmlParser {
}

struct wxHtmlWinTagHandler(*u8);
impl HtmlWinTagHandler for wxHtmlWinTagHandler {}
impl HtmlTagHandler for wxHtmlWinTagHandler {}
impl Object for wxHtmlWinTagHandler { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWinTagHandler {
}

trait HtmlWinTagHandler : HtmlTagHandler {
}

struct wxHtmlWindow(*u8);
impl HtmlWindow for wxHtmlWindow {}
impl ScrolledWindow for wxHtmlWindow {}
impl Panel for wxHtmlWindow {}
impl Window for wxHtmlWindow {}
impl EvtHandler for wxHtmlWindow {}
impl Object for wxHtmlWindow { fn handle(&self) -> *u8 { **self } }

impl wxHtmlWindow {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: @String) -> @HtmlWindow {
        unsafe { @wxHtmlWindow(wxHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) as @HtmlWindow }
    }
}

trait HtmlWindow : ScrolledWindow {
    #[fixed_stack_segment]
    fn appendToPage(&self, source: @String) -> bool {
        unsafe { wxHtmlWindow_AppendToPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn getInternalRepresentation(&self) -> @HtmlContainerCell {
        unsafe { @wxHtmlContainerCell(wxHtmlWindow_GetInternalRepresentation(self.handle())) as @HtmlContainerCell }
    }
    #[fixed_stack_segment]
    fn getOpenedAnchor(&self) -> @String {
        unsafe { @wxString(wxHtmlWindow_GetOpenedAnchor(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getOpenedPage(&self) -> @String {
        unsafe { @wxString(wxHtmlWindow_GetOpenedPage(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getOpenedPageTitle(&self) -> @String {
        unsafe { @wxString(wxHtmlWindow_GetOpenedPageTitle(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getRelatedFrame(&self) -> @Frame {
        unsafe { @wxFrame(wxHtmlWindow_GetRelatedFrame(self.handle())) as @Frame }
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
    fn loadPage(&self, location: @String) -> bool {
        unsafe { wxHtmlWindow_LoadPage(self.handle(), location.handle()) }
    }
    #[fixed_stack_segment]
    fn readCustomization(&self, cfg: @ConfigBase, path: @String) {
        unsafe { wxHtmlWindow_ReadCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.handle(), b) }
    }
    #[fixed_stack_segment]
    fn setFonts(&self, normal_face: @String, fixed_face: @String, sizes: *c_int) {
        unsafe { wxHtmlWindow_SetFonts(self.handle(), normal_face.handle(), fixed_face.handle(), sizes) }
    }
    #[fixed_stack_segment]
    fn setPage(&self, source: @String) {
        unsafe { wxHtmlWindow_SetPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedFrame(&self, frame: @Frame, format: @String) {
        unsafe { wxHtmlWindow_SetRelatedFrame(self.handle(), frame.handle(), format.handle()) }
    }
    #[fixed_stack_segment]
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.handle(), bar) }
    }
    #[fixed_stack_segment]
    fn writeCustomization(&self, cfg: @ConfigBase, path: @String) {
        unsafe { wxHtmlWindow_WriteCustomization(self.handle(), cfg.handle(), path.handle()) }
    }
}

struct wxIPV4address(*u8);
impl IPV4address for wxIPV4address {}
impl SockAddress for wxIPV4address {}
impl Object for wxIPV4address { fn handle(&self) -> *u8 { **self } }

impl wxIPV4address {
}

trait IPV4address : SockAddress {
}

struct wxIcon(*u8);
impl Icon for wxIcon {}
impl Bitmap for wxIcon {}
impl GDIObject for wxIcon {}
impl Object for wxIcon { fn handle(&self) -> *u8 { **self } }

impl wxIcon {
    #[fixed_stack_segment]
    pub fn newDefault() -> @Icon {
        unsafe { @wxIcon(wxIcon_CreateDefault()) as @Icon }
    }
    #[fixed_stack_segment]
    pub fn newLoad(name: @String, type_: c_long, width: c_int, height: c_int) -> @Icon {
        unsafe { @wxIcon(wxIcon_CreateLoad(name.handle(), type_, width, height)) as @Icon }
    }
}

trait Icon : Bitmap {
    #[fixed_stack_segment]
    fn assign(&self, other: *u8) {
        unsafe { wxIcon_Assign(self.handle(), other) }
    }
    #[fixed_stack_segment]
    fn copyFromBitmap(&self, bmp: @Bitmap) {
        unsafe { wxIcon_CopyFromBitmap(self.handle(), bmp.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIcon_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn fromRaw(&self, width: c_int, height: c_int) -> @Icon {
        unsafe { @wxIcon(wxIcon_FromRaw(self.handle(), width, height)) as @Icon }
    }
    #[fixed_stack_segment]
    fn fromXPM(&self) -> @Icon {
        unsafe { @wxIcon(wxIcon_FromXPM(self.handle())) as @Icon }
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
    fn isEqual(&self, other: @Icon) -> bool {
        unsafe { wxIcon_IsEqual(self.handle(), other.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, name: @String, type_: c_long, width: c_int, height: c_int) -> c_int {
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
impl IconBundle for wxIconBundle { fn handle(&self) -> *u8 { **self } }

impl wxIconBundle {
    #[fixed_stack_segment]
    pub fn newDefault() -> @IconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateDefault()) as @IconBundle }
    }
    #[fixed_stack_segment]
    pub fn newFromFile(file: @String, type_: c_int) -> @IconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateFromFile(file.handle(), type_)) as @IconBundle }
    }
    #[fixed_stack_segment]
    pub fn newFromIcon(icon: @Icon) -> @IconBundle {
        unsafe { @wxIconBundle(wxIconBundle_CreateFromIcon(icon.handle())) as @IconBundle }
    }
}

trait IconBundle {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @Icon) {
        unsafe { wxIconBundle_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addIconFromFile(&self, file: @String, type_: c_int) {
        unsafe { wxIconBundle_AddIconFromFile(self.handle(), file.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn assign(&self, _ref: @IconBundle) {
        unsafe { wxIconBundle_Assign(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, w: c_int, h: c_int, _ref: @Icon) {
        unsafe { wxIconBundle_GetIcon(self.handle(), w, h, _ref.handle()) }
    }
}

struct wxIconizeEvent(*u8);
impl IconizeEvent for wxIconizeEvent {}
impl Event for wxIconizeEvent {}
impl Object for wxIconizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxIconizeEvent {
}

trait IconizeEvent : Event {
}

struct wxIdleEvent(*u8);
impl IdleEvent for wxIdleEvent {}
impl Event for wxIdleEvent {}
impl Object for wxIdleEvent { fn handle(&self) -> *u8 { **self } }

impl wxIdleEvent {
}

trait IdleEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, object_dest: @Object) {
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
impl Image for wxImage {}
impl Object for wxImage { fn handle(&self) -> *u8 { **self } }

impl wxImage {
    #[fixed_stack_segment]
    pub fn canRead(name: @String) -> bool {
        unsafe { wxImage_CanRead(name.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @Image {
        unsafe { @wxImage(wxImage_CreateDefault()) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(bitmap: @Bitmap) -> @Image {
        unsafe { @wxImage(wxImage_CreateFromBitmap(bitmap.handle())) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newFromByteString(data: *char, length: c_int, type_: c_int) -> @Image {
        unsafe { @wxImage(wxImage_CreateFromByteString(data, length, type_)) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newFromLazyByteString(data: *char, length: c_int, type_: c_int) -> @Image {
        unsafe { @wxImage(wxImage_CreateFromLazyByteString(data, length, type_)) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newFromData(width: c_int, height: c_int, data: *u8) -> @Image {
        unsafe { @wxImage(wxImage_CreateFromData(width, height, data)) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newFromFile(name: @String) -> @Image {
        unsafe { @wxImage(wxImage_CreateFromFile(name.handle())) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newSized(width: c_int, height: c_int) -> @Image {
        unsafe { @wxImage(wxImage_CreateSized(width, height)) as @Image }
    }
    #[fixed_stack_segment]
    pub fn newFromDataEx(width: c_int, height: c_int, data: *u8, isStaticData: c_int) -> @Image {
        unsafe { @wxImage(wxImage_CreateFromDataEx(width, height, data, isStaticData)) as @Image }
    }
}

trait Image : Object {
    #[fixed_stack_segment]
    fn convertToBitmap(&self, bitmap: @Bitmap) {
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
    fn getSubImage(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: @Image) {
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
    fn getOption(&self, name: @String) -> @String {
        unsafe { @wxString(wxImage_GetOption(self.handle(), name.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getOptionInt(&self, name: @String) -> bool {
        unsafe { wxImage_GetOptionInt(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn hasOption(&self, name: @String) -> bool {
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
    fn loadFile(&self, name: @String, type_: c_int) -> bool {
        unsafe { wxImage_LoadFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn mirror(&self, horizontally: c_int, image: @Image) {
        unsafe { wxImage_Mirror(self.handle(), horizontally, image.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxImage_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn paste(&self, image: @Image, x: c_int, y: c_int) {
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
    fn rotate(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *u8, image: @Image) {
        unsafe { wxImage_Rotate(self.handle(), angle, c_x, c_y, interpolating, offset_after_rotation, image.handle()) }
    }
    #[fixed_stack_segment]
    fn rotate90(&self, clockwise: c_int, image: @Image) {
        unsafe { wxImage_Rotate90(self.handle(), clockwise, image.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, name: @String, type_: c_int) -> bool {
        unsafe { wxImage_SaveFile(self.handle(), name.handle(), type_) }
    }
    #[fixed_stack_segment]
    fn scale(&self, width: c_int, height: c_int, image: @Image) {
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
    fn setOption(&self, name: @String, value: @String) {
        unsafe { wxImage_SetOption(self.handle(), name.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setOptionInt(&self, name: @String, value: c_int) {
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
impl ImageHandler for wxImageHandler {}
impl Object for wxImageHandler { fn handle(&self) -> *u8 { **self } }

impl wxImageHandler {
}

trait ImageHandler : Object {
}

struct wxImageList(*u8);
impl ImageList for wxImageList {}
impl Object for wxImageList { fn handle(&self) -> *u8 { **self } }

impl wxImageList {
    #[fixed_stack_segment]
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> @ImageList {
        unsafe { @wxImageList(wxImageList_Create(width, height, mask, initialCount)) as @ImageList }
    }
}

trait ImageList : Object {
    #[fixed_stack_segment]
    fn addBitmap(&self, bitmap: @Bitmap, mask: @Bitmap) -> c_int {
        unsafe { wxImageList_AddBitmap(self.handle(), bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn addIcon(&self, icon: @Icon) -> c_int {
        unsafe { wxImageList_AddIcon(self.handle(), icon.handle()) }
    }
    #[fixed_stack_segment]
    fn addMasked(&self, bitmap: @Bitmap, maskColour: @Colour) -> c_int {
        unsafe { wxImageList_AddMasked(self.handle(), bitmap.handle(), maskColour.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxImageList_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn draw(&self, index: c_int, dc: @DC, x: c_int, y: c_int, flags: c_int, solidBackground: bool) -> bool {
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
    fn replace(&self, index: c_int, bitmap: @Bitmap, mask: @Bitmap) -> bool {
        unsafe { wxImageList_Replace(self.handle(), index, bitmap.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceIcon(&self, index: c_int, icon: @Icon) -> bool {
        unsafe { wxImageList_ReplaceIcon(self.handle(), index, icon.handle()) }
    }
}

struct wxIndividualLayoutConstraint(*u8);
impl IndividualLayoutConstraint for wxIndividualLayoutConstraint {}
impl Object for wxIndividualLayoutConstraint { fn handle(&self) -> *u8 { **self } }

impl wxIndividualLayoutConstraint {
}

trait IndividualLayoutConstraint : Object {
    #[fixed_stack_segment]
    fn above(&self, sibling: @Window, marg: c_int) {
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
    fn below(&self, sibling: @Window, marg: c_int) {
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
    fn leftOf(&self, sibling: @Window, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn percentOf(&self, otherW: @Window, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.handle(), otherW.handle(), wh, per) }
    }
    #[fixed_stack_segment]
    fn resetIfWin(&self, otherW: @Window) -> bool {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.handle(), otherW.handle()) }
    }
    #[fixed_stack_segment]
    fn rightOf(&self, sibling: @Window, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.handle(), sibling.handle(), marg) }
    }
    #[fixed_stack_segment]
    fn sameAs(&self, otherW: @Window, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.handle(), otherW.handle(), edge, marg) }
    }
    #[fixed_stack_segment]
    fn satisfyConstraint(&self, constraints: *u8, win: @Window) -> bool {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.handle(), constraints, win.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, rel: c_int, otherW: @Window, otherE: c_int, val: c_int, marg: c_int) {
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
impl InitDialogEvent for wxInitDialogEvent {}
impl Event for wxInitDialogEvent {}
impl Object for wxInitDialogEvent { fn handle(&self) -> *u8 { **self } }

impl wxInitDialogEvent {
}

trait InitDialogEvent : Event {
}

struct wxInputStream(*u8);
impl InputStream for wxInputStream {}
impl StreamBase for wxInputStream { fn handle(&self) -> *u8 { **self } }

impl wxInputStream {
}

trait InputStream : StreamBase {
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
impl Joystick for wxJoystick {}
impl Object for wxJoystick { fn handle(&self) -> *u8 { **self } }

impl wxJoystick {
}

trait Joystick : Object {
}

struct wxJoystickEvent(*u8);
impl JoystickEvent for wxJoystickEvent {}
impl Event for wxJoystickEvent {}
impl Object for wxJoystickEvent { fn handle(&self) -> *u8 { **self } }

impl wxJoystickEvent {
}

trait JoystickEvent : Event {
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
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxJoystickEvent_GetPosition(self.handle())) as @Point }
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
impl KeyEvent for wxKeyEvent {}
impl Event for wxKeyEvent {}
impl Object for wxKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxKeyEvent {
}

trait KeyEvent : Event {
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
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxKeyEvent_GetPosition(self.handle())) as @Point }
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
impl LEDNumberCtrl for wxLEDNumberCtrl {}
impl Control for wxLEDNumberCtrl {}
impl Window for wxLEDNumberCtrl {}
impl EvtHandler for wxLEDNumberCtrl {}
impl Object for wxLEDNumberCtrl { fn handle(&self) -> *u8 { **self } }

impl wxLEDNumberCtrl {
}

trait LEDNumberCtrl : Control {
}

struct wxLayoutAlgorithm(*u8);
impl LayoutAlgorithm for wxLayoutAlgorithm {}
impl Object for wxLayoutAlgorithm { fn handle(&self) -> *u8 { **self } }

impl wxLayoutAlgorithm {
    #[fixed_stack_segment]
    pub fn new() -> @LayoutAlgorithm {
        unsafe { @wxLayoutAlgorithm(wxLayoutAlgorithm_Create()) as @LayoutAlgorithm }
    }
}

trait LayoutAlgorithm : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLayoutAlgorithm_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn layoutFrame(&self, frame: @Frame, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.handle(), frame.handle(), mainWindow) }
    }
    #[fixed_stack_segment]
    fn layoutMDIFrame(&self, frame: @Frame, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.handle(), frame.handle(), x, y, w, h, use_) }
    }
    #[fixed_stack_segment]
    fn layoutWindow(&self, frame: @Frame, mainWindow: *u8) -> bool {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.handle(), frame.handle(), mainWindow) }
    }
}

struct wxLayoutConstraints(*u8);
impl LayoutConstraints for wxLayoutConstraints {}
impl Object for wxLayoutConstraints { fn handle(&self) -> *u8 { **self } }

impl wxLayoutConstraints {
    #[fixed_stack_segment]
    pub fn new() -> @LayoutConstraints {
        unsafe { @wxLayoutConstraints(wxLayoutConstraints_Create()) as @LayoutConstraints }
    }
}

trait LayoutConstraints : Object {
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
impl List for wxList {}
impl Object for wxList { fn handle(&self) -> *u8 { **self } }

impl wxList {
}

trait List : Object {
}

struct wxListBox(*u8);
impl ListBox for wxListBox {}
impl Control for wxListBox {}
impl Window for wxListBox {}
impl EvtHandler for wxListBox {}
impl Object for wxListBox { fn handle(&self) -> *u8 { **self } }

impl wxListBox {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: c_int) -> @ListBox {
        unsafe { @wxListBox(wxListBox_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, n, str, _stl)) as @ListBox }
    }
}

trait ListBox : Control {
    #[fixed_stack_segment]
    fn append(&self, item: @String) {
        unsafe { wxListBox_Append(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendData(&self, item: @String, data: *u8) {
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
    fn findString(&self, s: @String) -> c_int {
        unsafe { wxListBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self, n: c_int) -> @ClientData {
        unsafe { @wxClientData(wxListBox_GetClientData(self.handle(), n)) as @ClientData }
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
    fn getString(&self, n: c_int) -> @String {
        unsafe { @wxString(wxListBox_GetString(self.handle(), n)) as @String }
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
    fn setClientData(&self, n: c_int, clientData: @ClientData) {
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
    fn setString(&self, n: c_int, s: @String) {
        unsafe { wxListBox_SetString(self.handle(), n, s.handle()) }
    }
    #[fixed_stack_segment]
    fn setStringSelection(&self, str: @String, sel: bool) {
        unsafe { wxListBox_SetStringSelection(self.handle(), str.handle(), sel) }
    }
}

struct wxListCtrl(*u8);
impl ListCtrl for wxListCtrl {}
impl Control for wxListCtrl {}
impl Window for wxListCtrl {}
impl EvtHandler for wxListCtrl {}
impl Object for wxListCtrl { fn handle(&self) -> *u8 { **self } }

impl wxListCtrl {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @ListCtrl {
        unsafe { @wxListCtrl(wxListCtrl_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @ListCtrl }
    }
}

trait ListCtrl : Control {
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
    fn findItem(&self, start: c_int, str: @String, partial: bool) -> c_int {
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
    fn getColumn(&self, col: c_int, item: @ListItem) -> bool {
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
    fn getEditControl(&self) -> @TextCtrl {
        unsafe { @wxTextCtrl(wxListCtrl_GetEditControl(self.handle())) as @TextCtrl }
    }
    #[fixed_stack_segment]
    fn getImageList(&self, which: c_int) -> @ImageList {
        unsafe { @wxImageList(wxListCtrl_GetImageList(self.handle(), which)) as @ImageList }
    }
    #[fixed_stack_segment]
    fn getItem(&self, info: @ListItem) -> bool {
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
    fn getItemFont(&self, item: c_long) -> @Font {
        unsafe { @wxFont(wxListCtrl_GetItemFont(self.handle(), item)) as @Font }
    }
    #[fixed_stack_segment]
    fn getItemPosition(&self, item: c_int) -> @Point {
        unsafe { @wxPoint(wxListCtrl_GetItemPosition(self.handle(), item)) as @Point }
    }
    #[fixed_stack_segment]
    fn getItemRect(&self, item: c_int, code: c_int) -> @Rect {
        unsafe { @wxRect(wxListCtrl_GetItemRect(self.handle(), item, code)) as @Rect }
    }
    #[fixed_stack_segment]
    fn getItemSpacing(&self, isSmall: bool) -> @Size {
        unsafe { @wxSize(wxListCtrl_GetItemSpacing(self.handle(), isSmall)) as @Size }
    }
    #[fixed_stack_segment]
    fn getItemState(&self, item: c_int, stateMask: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemState(self.handle(), item, stateMask) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: c_int) -> @String {
        unsafe { @wxString(wxListCtrl_GetItemText(self.handle(), item)) as @String }
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
    fn getTextColour(&self, _ref: @Colour) {
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
    fn insertColumn(&self, col: c_int, heading: @String, format: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_InsertColumn(self.handle(), col, heading.handle(), format, width) }
    }
    #[fixed_stack_segment]
    fn insertColumnFromInfo(&self, col: c_int, info: @ListItem) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.handle(), col, info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, info: @ListItem) -> c_int {
        unsafe { wxListCtrl_InsertItem(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithData(&self, index: c_int, label: @String) -> c_int {
        unsafe { wxListCtrl_InsertItemWithData(self.handle(), index, label.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemWithImage(&self, index: c_int, imageIndex: c_int) -> c_int {
        unsafe { wxListCtrl_InsertItemWithImage(self.handle(), index, imageIndex) }
    }
    #[fixed_stack_segment]
    fn insertItemWithLabel(&self, index: c_int, label: @String, imageIndex: c_int) -> c_int {
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
    fn setBackgroundColour(&self, col: @Colour) {
        unsafe { wxListCtrl_SetBackgroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumn(&self, col: c_int, item: @ListItem) -> bool {
        unsafe { wxListCtrl_SetColumn(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn setColumnWidth(&self, col: c_int, width: c_int) -> bool {
        unsafe { wxListCtrl_SetColumnWidth(self.handle(), col, width) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, col: @Colour) -> c_int {
        unsafe { wxListCtrl_SetForegroundColour(self.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @ImageList, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.handle(), imageList.handle(), which) }
    }
    #[fixed_stack_segment]
    fn setItem(&self, index: c_int, col: c_int, label: @String, imageId: c_int) -> bool {
        unsafe { wxListCtrl_SetItem(self.handle(), index, col, label.handle(), imageId) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: c_int, data: c_int) -> bool {
        unsafe { wxListCtrl_SetItemData(self.handle(), item, data) }
    }
    #[fixed_stack_segment]
    fn setItemFromInfo(&self, info: @ListItem) -> bool {
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
    fn setItemText(&self, item: c_int, str: @String) {
        unsafe { wxListCtrl_SetItemText(self.handle(), item, str.handle()) }
    }
    #[fixed_stack_segment]
    fn setSingleStyle(&self, style: c_int, add: bool) {
        unsafe { wxListCtrl_SetSingleStyle(self.handle(), style, add) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, col: @Colour) {
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
    fn assignImageList(&self, images: @ImageList, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.handle(), images.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getColumn2(&self, col: c_int, item: @ListItem) {
        unsafe { wxListCtrl_GetColumn2(self.handle(), col, item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem2(&self, info: @ListItem) {
        unsafe { wxListCtrl_GetItem2(self.handle(), info.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemPosition2(&self, item: c_int) -> @Point {
        unsafe { @wxPoint(wxListCtrl_GetItemPosition2(self.handle(), item)) as @Point }
    }
    #[fixed_stack_segment]
    fn sortItems2(&self, closure: @Closure) -> bool {
        unsafe { wxListCtrl_SortItems2(self.handle(), closure.handle()) }
    }
}

struct wxListEvent(*u8);
impl ListEvent for wxListEvent {}
impl NotifyEvent for wxListEvent {}
impl CommandEvent for wxListEvent {}
impl Event for wxListEvent {}
impl Object for wxListEvent { fn handle(&self) -> *u8 { **self } }

impl wxListEvent {
}

trait ListEvent : NotifyEvent {
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
    fn getItem(&self, _ref: @ListItem) {
        unsafe { wxListEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @String {
        unsafe { @wxString(wxListEvent_GetLabel(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @Point {
        unsafe { @wxPoint(wxListEvent_GetPoint(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @String {
        unsafe { @wxString(wxListEvent_GetText(self.handle())) as @String }
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
impl ListItem for wxListItem {}
impl Object for wxListItem { fn handle(&self) -> *u8 { **self } }

impl wxListItem {
    #[fixed_stack_segment]
    pub fn new() -> @ListItem {
        unsafe { @wxListItem(wxListItem_Create()) as @ListItem }
    }
}

trait ListItem : Object {
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
    fn getBackgroundColour(&self, _ref: @Colour) {
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
    fn getFont(&self, _ref: @Font) {
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
    fn getText(&self) -> @String {
        unsafe { @wxString(wxListItem_GetText(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, _ref: @Colour) {
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
    fn setBackgroundColour(&self, colBack: @Colour) {
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
    fn setFont(&self, font: @Font) {
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
    fn setText(&self, text: @String) {
        unsafe { wxListItem_SetText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setTextColour(&self, colText: @Colour) {
        unsafe { wxListItem_SetTextColour(self.handle(), colText.handle()) }
    }
    #[fixed_stack_segment]
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.handle(), width) }
    }
}

struct wxLocale(*u8);
impl Locale for wxLocale { fn handle(&self) -> *u8 { **self } }

impl wxLocale {
    #[fixed_stack_segment]
    pub fn new(_name: c_int, _flags: c_int) -> @Locale {
        unsafe { @wxLocale(wxLocale_Create(_name, _flags)) as @Locale }
    }
}

trait Locale {
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
    fn getLocale(&self) -> @Locale {
        unsafe { @wxLocale(wxLocale_GetLocale(self.handle())) as @Locale }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @String {
        unsafe { @wxString(wxLocale_GetName(self.handle())) as @String }
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
impl Log for wxLog { fn handle(&self) -> *u8 { **self } }

impl wxLog {
    #[fixed_stack_segment]
    pub fn getActiveTarget() -> @Log {
        unsafe { @wxLog(wxLog_GetActiveTarget()) as @Log }
    }
}

trait Log {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addTraceMask(&self, str: @String) {
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
    fn isAllowedTraceMask(&self, mask: @Mask) -> bool {
        unsafe { wxLog_IsAllowedTraceMask(self.handle(), mask.handle()) }
    }
    #[fixed_stack_segment]
    fn onLog(&self, level: c_int, szString: *wchar_t, t: c_int) {
        unsafe { wxLog_OnLog(self.handle(), level, szString, t) }
    }
    #[fixed_stack_segment]
    fn removeTraceMask(&self, str: @String) {
        unsafe { wxLog_RemoveTraceMask(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxLog_Resume(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setActiveTarget(&self) -> @Log {
        unsafe { @wxLog(wxLog_SetActiveTarget(self.handle())) as @Log }
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
impl LogChain for wxLogChain {}
impl Log for wxLogChain { fn handle(&self) -> *u8 { **self } }

impl wxLogChain {
    #[fixed_stack_segment]
    pub fn new(logger: @Log) -> @LogChain {
        unsafe { @wxLogChain(wxLogChain_Create(logger.handle())) as @LogChain }
    }
}

trait LogChain : Log {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxLogChain_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getOldLog(&self) -> @Log {
        unsafe { @wxLog(wxLogChain_GetOldLog(self.handle())) as @Log }
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
    fn setLog(&self, logger: @Log) {
        unsafe { wxLogChain_SetLog(self.handle(), logger.handle()) }
    }
}

struct wxLogGUI(*u8);
impl LogGUI for wxLogGUI {}
impl Log for wxLogGUI { fn handle(&self) -> *u8 { **self } }

impl wxLogGUI {
}

trait LogGUI : Log {
}

struct wxLogNull(*u8);
impl LogNull for wxLogNull {}
impl Log for wxLogNull { fn handle(&self) -> *u8 { **self } }

impl wxLogNull {
    #[fixed_stack_segment]
    pub fn new() -> @LogNull {
        unsafe { @wxLogNull(wxLogNull_Create()) as @LogNull }
    }
}

trait LogNull : Log {
}

struct wxLogPassThrough(*u8);
impl LogPassThrough for wxLogPassThrough {}
impl LogChain for wxLogPassThrough {}
impl Log for wxLogPassThrough { fn handle(&self) -> *u8 { **self } }

impl wxLogPassThrough {
}

trait LogPassThrough : LogChain {
}

struct wxLogStderr(*u8);
impl LogStderr for wxLogStderr {}
impl Log for wxLogStderr { fn handle(&self) -> *u8 { **self } }

impl wxLogStderr {
    #[fixed_stack_segment]
    pub fn new() -> @LogStderr {
        unsafe { @wxLogStderr(wxLogStderr_Create()) as @LogStderr }
    }
    #[fixed_stack_segment]
    pub fn newStdOut() -> @LogStderr {
        unsafe { @wxLogStderr(wxLogStderr_CreateStdOut()) as @LogStderr }
    }
}

trait LogStderr : Log {
}

struct wxLogStream(*u8);
impl LogStream for wxLogStream {}
impl Log for wxLogStream { fn handle(&self) -> *u8 { **self } }

impl wxLogStream {
}

trait LogStream : Log {
}

struct wxLogTextCtrl(*u8);
impl LogTextCtrl for wxLogTextCtrl {}
impl Log for wxLogTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxLogTextCtrl {
    #[fixed_stack_segment]
    pub fn new(text: @TextCtrl) -> @LogTextCtrl {
        unsafe { @wxLogTextCtrl(wxLogTextCtrl_Create(text.handle())) as @LogTextCtrl }
    }
}

trait LogTextCtrl : Log {
}

struct wxLogWindow(*u8);
impl LogWindow for wxLogWindow {}
impl LogPassThrough for wxLogWindow {}
impl LogChain for wxLogWindow {}
impl Log for wxLogWindow { fn handle(&self) -> *u8 { **self } }

impl wxLogWindow {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, title: *wchar_t, showit: bool, passthrough: bool) -> @LogWindow {
        unsafe { @wxLogWindow(wxLogWindow_Create(parent.handle(), title, showit, passthrough)) as @LogWindow }
    }
}

trait LogWindow : LogPassThrough {
    #[fixed_stack_segment]
    fn getFrame(&self) -> @Frame {
        unsafe { @wxFrame(wxLogWindow_GetFrame(self.handle())) as @Frame }
    }
}

struct wxLongLong(*u8);
impl LongLong for wxLongLong { fn handle(&self) -> *u8 { **self } }

impl wxLongLong {
}

trait LongLong {
    fn handle(&self) -> *u8;
    
}

struct wxMBConv(*u8);
impl MBConv for wxMBConv { fn handle(&self) -> *u8 { **self } }

impl wxMBConv {
}

trait MBConv {
    fn handle(&self) -> *u8;
    
}

struct wxMBConvFile(*u8);
impl MBConvFile for wxMBConvFile {}
impl MBConv for wxMBConvFile { fn handle(&self) -> *u8 { **self } }

impl wxMBConvFile {
}

trait MBConvFile : MBConv {
}

struct wxMBConvUTF7(*u8);
impl MBConvUTF7 for wxMBConvUTF7 {}
impl MBConv for wxMBConvUTF7 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF7 {
}

trait MBConvUTF7 : MBConv {
}

struct wxMBConvUTF8(*u8);
impl MBConvUTF8 for wxMBConvUTF8 {}
impl MBConv for wxMBConvUTF8 { fn handle(&self) -> *u8 { **self } }

impl wxMBConvUTF8 {
}

trait MBConvUTF8 : MBConv {
}

struct wxMDIChildFrame(*u8);
impl MDIChildFrame for wxMDIChildFrame {}
impl Frame for wxMDIChildFrame {}
impl TopLevelWindow for wxMDIChildFrame {}
impl Window for wxMDIChildFrame {}
impl EvtHandler for wxMDIChildFrame {}
impl Object for wxMDIChildFrame { fn handle(&self) -> *u8 { **self } }

impl wxMDIChildFrame {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @MDIChildFrame {
        unsafe { @wxMDIChildFrame(wxMDIChildFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @MDIChildFrame }
    }
}

trait MDIChildFrame : Frame {
    #[fixed_stack_segment]
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.handle()) }
    }
}

struct wxMDIClientWindow(*u8);
impl MDIClientWindow for wxMDIClientWindow {}
impl Window for wxMDIClientWindow {}
impl EvtHandler for wxMDIClientWindow {}
impl Object for wxMDIClientWindow { fn handle(&self) -> *u8 { **self } }

impl wxMDIClientWindow {
}

trait MDIClientWindow : Window {
}

struct wxMDIParentFrame(*u8);
impl MDIParentFrame for wxMDIParentFrame {}
impl Frame for wxMDIParentFrame {}
impl TopLevelWindow for wxMDIParentFrame {}
impl Window for wxMDIParentFrame {}
impl EvtHandler for wxMDIParentFrame {}
impl Object for wxMDIParentFrame { fn handle(&self) -> *u8 { **self } }

impl wxMDIParentFrame {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @MDIParentFrame {
        unsafe { @wxMDIParentFrame(wxMDIParentFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @MDIParentFrame }
    }
}

trait MDIParentFrame : Frame {
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
    fn getActiveChild(&self) -> @MDIChildFrame {
        unsafe { @wxMDIChildFrame(wxMDIParentFrame_GetActiveChild(self.handle())) as @MDIChildFrame }
    }
    #[fixed_stack_segment]
    fn getClientWindow(&self) -> @MDIClientWindow {
        unsafe { @wxMDIClientWindow(wxMDIParentFrame_GetClientWindow(self.handle())) as @MDIClientWindow }
    }
    #[fixed_stack_segment]
    fn getWindowMenu(&self) -> @Menu {
        unsafe { @wxMenu(wxMDIParentFrame_GetWindowMenu(self.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn onCreateClient(&self) -> @MDIClientWindow {
        unsafe { @wxMDIClientWindow(wxMDIParentFrame_OnCreateClient(self.handle())) as @MDIClientWindow }
    }
    #[fixed_stack_segment]
    fn setWindowMenu(&self, menu: @Menu) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.handle()) }
    }
}

struct wxMask(*u8);
impl Mask for wxMask {}
impl Object for wxMask { fn handle(&self) -> *u8 { **self } }

impl wxMask {
    #[fixed_stack_segment]
    pub fn new(bitmap: @Bitmap) -> @Mask {
        unsafe { @wxMask(wxMask_Create(bitmap.handle())) as @Mask }
    }
    #[fixed_stack_segment]
    pub fn newColoured(bitmap: @Bitmap, colour: @Colour) -> *u8 {
        unsafe { wxMask_CreateColoured(bitmap.handle(), colour.handle()) }
    }
}

trait Mask : Object {
}

struct wxMaximizeEvent(*u8);
impl MaximizeEvent for wxMaximizeEvent {}
impl Event for wxMaximizeEvent {}
impl Object for wxMaximizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxMaximizeEvent {
}

trait MaximizeEvent : Event {
}

struct wxMemoryDC(*u8);
impl MemoryDC for wxMemoryDC {}
impl DC for wxMemoryDC {}
impl Object for wxMemoryDC { fn handle(&self) -> *u8 { **self } }

impl wxMemoryDC {
    #[fixed_stack_segment]
    pub fn new() -> @MemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_Create()) as @MemoryDC }
    }
    #[fixed_stack_segment]
    pub fn newCompatible(dc: @DC) -> @MemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_CreateCompatible(dc.handle())) as @MemoryDC }
    }
    #[fixed_stack_segment]
    pub fn newWithBitmap(bitmap: @Bitmap) -> @MemoryDC {
        unsafe { @wxMemoryDC(wxMemoryDC_CreateWithBitmap(bitmap.handle())) as @MemoryDC }
    }
}

trait MemoryDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMemoryDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn selectObject(&self, bitmap: @Bitmap) {
        unsafe { wxMemoryDC_SelectObject(self.handle(), bitmap.handle()) }
    }
}

struct wxMemoryFSHandler(*u8);
impl MemoryFSHandler for wxMemoryFSHandler {}
impl FileSystemHandler for wxMemoryFSHandler {}
impl Object for wxMemoryFSHandler { fn handle(&self) -> *u8 { **self } }

impl wxMemoryFSHandler {
}

trait MemoryFSHandler : FileSystemHandler {
}

struct wxMemoryInputStream(*u8);
impl MemoryInputStream for wxMemoryInputStream {}
impl InputStream for wxMemoryInputStream {}
impl StreamBase for wxMemoryInputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryInputStream {
}

trait MemoryInputStream : InputStream {
}

struct wxMemoryOutputStream(*u8);
impl MemoryOutputStream for wxMemoryOutputStream {}
impl OutputStream for wxMemoryOutputStream {}
impl StreamBase for wxMemoryOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxMemoryOutputStream {
}

trait MemoryOutputStream : OutputStream {
}

struct wxMenu(*u8);
impl Menu for wxMenu {}
impl EvtHandler for wxMenu {}
impl Object for wxMenu { fn handle(&self) -> *u8 { **self } }

impl wxMenu {
    #[fixed_stack_segment]
    pub fn new(title: @String, style: c_long) -> @Menu {
        unsafe { @wxMenu(wxMenu_Create(title.handle(), style)) as @Menu }
    }
}

trait Menu : EvtHandler {
    #[fixed_stack_segment]
    fn append(&self, id: c_int, text: @String, help: @String, isCheckable: bool) {
        unsafe { wxMenu_Append(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, _itm: @MenuItem) {
        unsafe { wxMenu_AppendItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn appendSub(&self, id: c_int, text: @String, submenu: @Menu, help: @String) {
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
    fn deleteByItem(&self, _itm: @MenuItem) {
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
    fn destroyByItem(&self, _itm: @MenuItem) {
        unsafe { wxMenu_DestroyByItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { wxMenu_Enable(self.handle(), id, enable) }
    }
    #[fixed_stack_segment]
    fn findItem(&self, id: c_int) -> @MenuItem {
        unsafe { @wxMenuItem(wxMenu_FindItem(self.handle(), id)) as @MenuItem }
    }
    #[fixed_stack_segment]
    fn findItemByLabel(&self, itemString: @String) -> c_int {
        unsafe { wxMenu_FindItemByLabel(self.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientData(&self) -> @ClientData {
        unsafe { @wxClientData(wxMenu_GetClientData(self.handle())) as @ClientData }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> @String {
        unsafe { @wxString(wxMenu_GetHelpString(self.handle(), id)) as @String }
    }
    #[fixed_stack_segment]
    fn getInvokingWindow(&self) -> @Window {
        unsafe { @wxWindow(wxMenu_GetInvokingWindow(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> @String {
        unsafe { @wxString(wxMenu_GetLabel(self.handle(), id)) as @String }
    }
    #[fixed_stack_segment]
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getMenuItems(&self, _lst: @List) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @Menu {
        unsafe { @wxMenu(wxMenu_GetParent(self.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn getStyle(&self) -> c_int {
        unsafe { wxMenu_GetStyle(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @String {
        unsafe { @wxString(wxMenu_GetTitle(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: size_t, id: c_int, text: @String, help: @String, isCheckable: bool) {
        unsafe { wxMenu_Insert(self.handle(), pos, id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, pos: size_t, _itm: @MenuItem) {
        unsafe { wxMenu_InsertItem(self.handle(), pos, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSub(&self, pos: size_t, id: c_int, text: @String, submenu: @Menu, help: @String) {
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
    fn prepend(&self, id: c_int, text: @String, help: @String, isCheckable: bool) {
        unsafe { wxMenu_Prepend(self.handle(), id, text.handle(), help.handle(), isCheckable) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, _itm: @MenuItem) {
        unsafe { wxMenu_PrependItem(self.handle(), _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn prependSub(&self, id: c_int, text: @String, submenu: @Menu, help: @String) {
        unsafe { wxMenu_PrependSub(self.handle(), id, text.handle(), submenu.handle(), help.handle()) }
    }
    #[fixed_stack_segment]
    fn removeById(&self, id: c_int, _itm: @MenuItem) {
        unsafe { wxMenu_RemoveById(self.handle(), id, _itm.handle()) }
    }
    #[fixed_stack_segment]
    fn removeByItem(&self, item: *u8) {
        unsafe { wxMenu_RemoveByItem(self.handle(), item) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, clientData: @ClientData) {
        unsafe { wxMenu_SetClientData(self.handle(), clientData.handle()) }
    }
    #[fixed_stack_segment]
    fn setEventHandler(&self, handler: @EvtHandler) {
        unsafe { wxMenu_SetEventHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: c_int, helpString: @String) {
        unsafe { wxMenu_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setInvokingWindow(&self, win: @Window) {
        unsafe { wxMenu_SetInvokingWindow(self.handle(), win.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, id: c_int, label: @String) {
        unsafe { wxMenu_SetLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setParent(&self, parent: @Window) {
        unsafe { wxMenu_SetParent(self.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    fn setTitle(&self, title: @String) {
        unsafe { wxMenu_SetTitle(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn updateUI(&self, source: *u8) {
        unsafe { wxMenu_UpdateUI(self.handle(), source) }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self) -> @MenuBar {
        unsafe { @wxMenuBar(wxMenu_GetMenuBar(self.handle())) as @MenuBar }
    }
    #[fixed_stack_segment]
    fn appendRadioItem(&self, id: c_int, text: @String, help: @String) {
        unsafe { wxMenu_AppendRadioItem(self.handle(), id, text.handle(), help.handle()) }
    }
}

struct wxMenuBar(*u8);
impl MenuBar for wxMenuBar {}
impl EvtHandler for wxMenuBar {}
impl Object for wxMenuBar { fn handle(&self) -> *u8 { **self } }

impl wxMenuBar {
    #[fixed_stack_segment]
    pub fn new(_style: c_int) -> @MenuBar {
        unsafe { @wxMenuBar(wxMenuBar_Create(_style)) as @MenuBar }
    }
}

trait MenuBar : EvtHandler {
    #[fixed_stack_segment]
    fn append(&self, menu: @Menu, title: @String) -> c_int {
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
    fn findItem(&self, id: c_int) -> @MenuItem {
        unsafe { @wxMenuItem(wxMenuBar_FindItem(self.handle(), id)) as @MenuItem }
    }
    #[fixed_stack_segment]
    fn findMenu(&self, title: @String) -> c_int {
        unsafe { wxMenuBar_FindMenu(self.handle(), title.handle()) }
    }
    #[fixed_stack_segment]
    fn findMenuItem(&self, menuString: @String, itemString: @String) -> c_int {
        unsafe { wxMenuBar_FindMenuItem(self.handle(), menuString.handle(), itemString.handle()) }
    }
    #[fixed_stack_segment]
    fn getHelpString(&self, id: c_int) -> @String {
        unsafe { @wxString(wxMenuBar_GetHelpString(self.handle(), id)) as @String }
    }
    #[fixed_stack_segment]
    fn getLabel(&self, id: c_int) -> @String {
        unsafe { @wxString(wxMenuBar_GetLabel(self.handle(), id)) as @String }
    }
    #[fixed_stack_segment]
    fn getLabelTop(&self, pos: c_int) -> @String {
        unsafe { @wxString(wxMenuBar_GetLabelTop(self.handle(), pos)) as @String }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, pos: c_int) -> @Menu {
        unsafe { @wxMenu(wxMenuBar_GetMenu(self.handle(), pos)) as @Menu }
    }
    #[fixed_stack_segment]
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insert(&self, pos: c_int, menu: @Menu, title: @String) -> c_int {
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
    fn remove(&self, pos: c_int) -> @Menu {
        unsafe { @wxMenu(wxMenuBar_Remove(self.handle(), pos)) as @Menu }
    }
    #[fixed_stack_segment]
    fn replace(&self, pos: c_int, menu: @Menu, title: @String) -> @Menu {
        unsafe { @wxMenu(wxMenuBar_Replace(self.handle(), pos, menu.handle(), title.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, id: c_int, helpString: @String) {
        unsafe { wxMenuBar_SetHelpString(self.handle(), id, helpString.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, id: c_int, label: @String) {
        unsafe { wxMenuBar_SetItemLabel(self.handle(), id, label.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, s: @String) {
        unsafe { wxMenuBar_SetLabel(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabelTop(&self, pos: c_int, label: @String) {
        unsafe { wxMenuBar_SetLabelTop(self.handle(), pos, label.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @Frame {
        unsafe { @wxFrame(wxMenuBar_GetFrame(self.handle())) as @Frame }
    }
}

struct wxMenuEvent(*u8);
impl MenuEvent for wxMenuEvent {}
impl Event for wxMenuEvent {}
impl Object for wxMenuEvent { fn handle(&self) -> *u8 { **self } }

impl wxMenuEvent {
}

trait MenuEvent : Event {
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
impl MenuItem for wxMenuItem {}
impl Object for wxMenuItem { fn handle(&self) -> *u8 { **self } }

impl wxMenuItem {
    #[fixed_stack_segment]
    pub fn new() -> @MenuItem {
        unsafe { @wxMenuItem(wxMenuItem_Create()) as @MenuItem }
    }
    #[fixed_stack_segment]
    pub fn getLabelFromText(text: *wchar_t) -> @String {
        unsafe { @wxString(wxMenuItem_GetLabelFromText(text)) as @String }
    }
    #[fixed_stack_segment]
    pub fn newSeparator() -> @MenuItem {
        unsafe { @wxMenuItem(wxMenuItem_CreateSeparator()) as @MenuItem }
    }
    #[fixed_stack_segment]
    pub fn newEx(id: c_int, label: @String, help: @String, itemkind: c_int, submenu: @Menu) -> @MenuItem {
        unsafe { @wxMenuItem(wxMenuItem_CreateEx(id, label.handle(), help.handle(), itemkind, submenu.handle())) as @MenuItem }
    }
}

trait MenuItem : Object {
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
    fn getHelp(&self) -> @String {
        unsafe { @wxString(wxMenuItem_GetHelp(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getId(&self) -> c_int {
        unsafe { wxMenuItem_GetId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @String {
        unsafe { @wxString(wxMenuItem_GetLabel(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getMenu(&self) -> @Menu {
        unsafe { @wxMenu(wxMenuItem_GetMenu(self.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn getSubMenu(&self) -> @Menu {
        unsafe { @wxMenu(wxMenuItem_GetSubMenu(self.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @String {
        unsafe { @wxString(wxMenuItem_GetText(self.handle())) as @String }
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
    fn setHelp(&self, str: @String) {
        unsafe { wxMenuItem_SetHelp(self.handle(), str.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, id: c_int) {
        unsafe { wxMenuItem_SetId(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn setSubMenu(&self, menu: @Menu) {
        unsafe { wxMenuItem_SetSubMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn setText(&self, str: @String) {
        unsafe { wxMenuItem_SetText(self.handle(), str.handle()) }
    }
}

struct wxMessageDialog(*u8);
impl MessageDialog for wxMessageDialog {}
impl Dialog for wxMessageDialog {}
impl TopLevelWindow for wxMessageDialog {}
impl Window for wxMessageDialog {}
impl EvtHandler for wxMessageDialog {}
impl Object for wxMessageDialog { fn handle(&self) -> *u8 { **self } }

impl wxMessageDialog {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _msg: @String, _cap: @String, _stl: c_int) -> @MessageDialog {
        unsafe { @wxMessageDialog(wxMessageDialog_Create(_prt.handle(), _msg.handle(), _cap.handle(), _stl)) as @MessageDialog }
    }
}

trait MessageDialog : Dialog {
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
impl Metafile for wxMetafile {}
impl Object for wxMetafile { fn handle(&self) -> *u8 { **self } }

impl wxMetafile {
    #[fixed_stack_segment]
    pub fn new(_file: @String) -> @Metafile {
        unsafe { @wxMetafile(wxMetafile_Create(_file.handle())) as @Metafile }
    }
}

trait Metafile : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMetafile_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxMetafile_IsOk(self.handle()) }
    }
    #[fixed_stack_segment]
    fn play(&self, _dc: @DC) -> bool {
        unsafe { wxMetafile_Play(self.handle(), _dc.handle()) }
    }
    #[fixed_stack_segment]
    fn setClipboard(&self, width: c_int, height: c_int) -> bool {
        unsafe { wxMetafile_SetClipboard(self.handle(), width, height) }
    }
}

struct wxMetafileDC(*u8);
impl MetafileDC for wxMetafileDC {}
impl DC for wxMetafileDC {}
impl Object for wxMetafileDC { fn handle(&self) -> *u8 { **self } }

impl wxMetafileDC {
    #[fixed_stack_segment]
    pub fn new(_file: @String) -> @MetafileDC {
        unsafe { @wxMetafileDC(wxMetafileDC_Create(_file.handle())) as @MetafileDC }
    }
}

trait MetafileDC : DC {
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
impl MimeTypesManager for wxMimeTypesManager { fn handle(&self) -> *u8 { **self } }

impl wxMimeTypesManager {
    #[fixed_stack_segment]
    pub fn new() -> @MimeTypesManager {
        unsafe { @wxMimeTypesManager(wxMimeTypesManager_Create()) as @MimeTypesManager }
    }
}

trait MimeTypesManager {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn addFallbacks(&self, _types: *u8) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.handle(), _types) }
    }
    #[fixed_stack_segment]
    fn enumAllFileTypes(&self, _lst: @List) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.handle(), _lst.handle()) }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromExtension(&self, _ext: @String) -> @FileType {
        unsafe { @wxFileType(wxMimeTypesManager_GetFileTypeFromExtension(self.handle(), _ext.handle())) as @FileType }
    }
    #[fixed_stack_segment]
    fn getFileTypeFromMimeType(&self, _name: @String) -> @FileType {
        unsafe { @wxFileType(wxMimeTypesManager_GetFileTypeFromMimeType(self.handle(), _name.handle())) as @FileType }
    }
    #[fixed_stack_segment]
    fn isOfType(&self, _type: @String, _wildcard: @String) -> bool {
        unsafe { wxMimeTypesManager_IsOfType(self.handle(), _type.handle(), _wildcard.handle()) }
    }
}

struct wxMiniFrame(*u8);
impl MiniFrame for wxMiniFrame {}
impl Frame for wxMiniFrame {}
impl TopLevelWindow for wxMiniFrame {}
impl Window for wxMiniFrame {}
impl EvtHandler for wxMiniFrame {}
impl Object for wxMiniFrame { fn handle(&self) -> *u8 { **self } }

impl wxMiniFrame {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @MiniFrame {
        unsafe { @wxMiniFrame(wxMiniFrame_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @MiniFrame }
    }
}

trait MiniFrame : Frame {
}

struct wxMirrorDC(*u8);
impl MirrorDC for wxMirrorDC {}
impl DC for wxMirrorDC {}
impl Object for wxMirrorDC { fn handle(&self) -> *u8 { **self } }

impl wxMirrorDC {
    #[fixed_stack_segment]
    pub fn new(dc: @DC) -> @MirrorDC {
        unsafe { @wxMirrorDC(wxMirrorDC_Create(dc.handle())) as @MirrorDC }
    }
}

trait MirrorDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMirrorDC_Delete(self.handle()) }
    }
}

struct wxModule(*u8);
impl Module for wxModule {}
impl Object for wxModule { fn handle(&self) -> *u8 { **self } }

impl wxModule {
}

trait Module : Object {
}

struct wxMouseCaptureChangedEvent(*u8);
impl MouseCaptureChangedEvent for wxMouseCaptureChangedEvent {}
impl Event for wxMouseCaptureChangedEvent {}
impl Object for wxMouseCaptureChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseCaptureChangedEvent {
}

trait MouseCaptureChangedEvent : Event {
}

struct wxMouseEvent(*u8);
impl MouseEvent for wxMouseEvent {}
impl Event for wxMouseEvent {}
impl Object for wxMouseEvent { fn handle(&self) -> *u8 { **self } }

impl wxMouseEvent {
}

trait MouseEvent : Event {
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
    fn getLogicalPosition(&self, dc: @DC) -> @Point {
        unsafe { @wxPoint(wxMouseEvent_GetLogicalPosition(self.handle(), dc.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxMouseEvent_GetPosition(self.handle())) as @Point }
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
impl MoveEvent for wxMoveEvent {}
impl Event for wxMoveEvent {}
impl Object for wxMoveEvent { fn handle(&self) -> *u8 { **self } }

impl wxMoveEvent {
}

trait MoveEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxMoveEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxMoveEvent_GetPosition(self.handle())) as @Point }
    }
}

struct wxMultiCellCanvas(*u8);
impl MultiCellCanvas for wxMultiCellCanvas {}
impl FlexGridSizer for wxMultiCellCanvas {}
impl GridSizer for wxMultiCellCanvas {}
impl Sizer for wxMultiCellCanvas {}
impl Object for wxMultiCellCanvas { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellCanvas {
}

trait MultiCellCanvas : FlexGridSizer {
}

struct wxMultiCellItemHandle(*u8);
impl MultiCellItemHandle for wxMultiCellItemHandle {}
impl Object for wxMultiCellItemHandle { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellItemHandle {
}

trait MultiCellItemHandle : Object {
}

struct wxMultiCellSizer(*u8);
impl MultiCellSizer for wxMultiCellSizer {}
impl Sizer for wxMultiCellSizer {}
impl Object for wxMultiCellSizer { fn handle(&self) -> *u8 { **self } }

impl wxMultiCellSizer {
}

trait MultiCellSizer : Sizer {
}

struct wxMutex(*u8);
impl Mutex for wxMutex { fn handle(&self) -> *u8 { **self } }

impl wxMutex {
}

trait Mutex {
    fn handle(&self) -> *u8;
    
}

struct wxMutexLocker(*u8);
impl MutexLocker for wxMutexLocker { fn handle(&self) -> *u8 { **self } }

impl wxMutexLocker {
}

trait MutexLocker {
    fn handle(&self) -> *u8;
    
}

struct wxNavigationKeyEvent(*u8);
impl NavigationKeyEvent for wxNavigationKeyEvent {}
impl Event for wxNavigationKeyEvent {}
impl Object for wxNavigationKeyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNavigationKeyEvent {
}

trait NavigationKeyEvent : Event {
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
    fn setCurrentFocus(&self, win: @Window) {
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
impl NewBitmapButton for wxNewBitmapButton {}
impl Panel for wxNewBitmapButton {}
impl Window for wxNewBitmapButton {}
impl EvtHandler for wxNewBitmapButton {}
impl Object for wxNewBitmapButton { fn handle(&self) -> *u8 { **self } }

impl wxNewBitmapButton {
}

trait NewBitmapButton : Panel {
}

struct wxNodeBase(*u8);
impl NodeBase for wxNodeBase { fn handle(&self) -> *u8 { **self } }

impl wxNodeBase {
}

trait NodeBase {
    fn handle(&self) -> *u8;
    
}

struct wxNotebook(*u8);
impl Notebook for wxNotebook {}
impl Control for wxNotebook {}
impl Window for wxNotebook {}
impl EvtHandler for wxNotebook {}
impl Object for wxNotebook { fn handle(&self) -> *u8 { **self } }

impl wxNotebook {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Notebook {
        unsafe { @wxNotebook(wxNotebook_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @Notebook }
    }
}

trait Notebook : Control {
    #[fixed_stack_segment]
    fn addPage(&self, pPage: @Window, strText: @String, bSelect: bool, imageId: c_int) -> bool {
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
    fn getImageList(&self) -> @ImageList {
        unsafe { @wxImageList(wxNotebook_GetImageList(self.handle())) as @ImageList }
    }
    #[fixed_stack_segment]
    fn getPage(&self, nPage: c_int) -> @Window {
        unsafe { @wxWindow(wxNotebook_GetPage(self.handle(), nPage)) as @Window }
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
    fn getPageText(&self, nPage: c_int) -> @String {
        unsafe { @wxString(wxNotebook_GetPageText(self.handle(), nPage)) as @String }
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
    fn insertPage(&self, nPage: c_int, pPage: @Window, strText: @String, bSelect: bool, imageId: c_int) -> bool {
        unsafe { wxNotebook_InsertPage(self.handle(), nPage, pPage.handle(), strText.handle(), bSelect, imageId) }
    }
    #[fixed_stack_segment]
    fn removePage(&self, nPage: c_int) -> bool {
        unsafe { wxNotebook_RemovePage(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @ImageList) {
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
    fn setPageText(&self, nPage: c_int, strText: @String) -> bool {
        unsafe { wxNotebook_SetPageText(self.handle(), nPage, strText.handle()) }
    }
    #[fixed_stack_segment]
    fn setSelection(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_SetSelection(self.handle(), nPage) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @ImageList) {
        unsafe { wxNotebook_AssignImageList(self.handle(), imageList.handle()) }
    }
}

struct wxNotebookEvent(*u8);
impl NotebookEvent for wxNotebookEvent {}
impl NotifyEvent for wxNotebookEvent {}
impl CommandEvent for wxNotebookEvent {}
impl Event for wxNotebookEvent {}
impl Object for wxNotebookEvent { fn handle(&self) -> *u8 { **self } }

impl wxNotebookEvent {
}

trait NotebookEvent : NotifyEvent {
}

struct wxNotifyEvent(*u8);
impl NotifyEvent for wxNotifyEvent {}
impl CommandEvent for wxNotifyEvent {}
impl Event for wxNotifyEvent {}
impl Object for wxNotifyEvent { fn handle(&self) -> *u8 { **self } }

impl wxNotifyEvent {
}

trait NotifyEvent : CommandEvent {
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
impl Object for wxObject { fn handle(&self) -> *u8 { **self } }

impl wxObject {
}

trait Object {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn safeDelete(&self) {
        unsafe { wxObject_SafeDelete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @Closure {
        unsafe { @wxClosure(wxObject_GetClientClosure(self.handle())) as @Closure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @Closure) {
        unsafe { wxObject_SetClientClosure(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxObject_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getClassInfo(&self) -> @ClassInfo {
        unsafe { @wxClassInfo(wxObject_GetClassInfo(self.handle())) as @ClassInfo }
    }
    #[fixed_stack_segment]
    fn isKindOf(&self, classInfo: @ClassInfo) -> bool {
        unsafe { wxObject_IsKindOf(self.handle(), classInfo.handle()) }
    }
    #[fixed_stack_segment]
    fn isScrolledWindow(&self) -> bool {
        unsafe { wxObject_IsScrolledWindow(self.handle()) }
    }
}

struct wxObjectRefData(*u8);
impl ObjectRefData for wxObjectRefData { fn handle(&self) -> *u8 { **self } }

impl wxObjectRefData {
}

trait ObjectRefData {
    fn handle(&self) -> *u8;
    
}

struct wxOutputStream(*u8);
impl OutputStream for wxOutputStream {}
impl StreamBase for wxOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxOutputStream {
}

trait OutputStream : StreamBase {
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
impl PageSetupDialog for wxPageSetupDialog {}
impl Dialog for wxPageSetupDialog {}
impl TopLevelWindow for wxPageSetupDialog {}
impl Window for wxPageSetupDialog {}
impl EvtHandler for wxPageSetupDialog {}
impl Object for wxPageSetupDialog { fn handle(&self) -> *u8 { **self } }

impl wxPageSetupDialog {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, data: @PageSetupDialogData) -> @PageSetupDialog {
        unsafe { @wxPageSetupDialog(wxPageSetupDialog_Create(parent.handle(), data.handle())) as @PageSetupDialog }
    }
}

trait PageSetupDialog : Dialog {
    #[fixed_stack_segment]
    fn getPageSetupData(&self, _ref: @PageSetupDialogData) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.handle(), _ref.handle()) }
    }
}

struct wxPageSetupDialogData(*u8);
impl PageSetupDialogData for wxPageSetupDialogData {}
impl Object for wxPageSetupDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPageSetupDialogData {
    #[fixed_stack_segment]
    pub fn new() -> @PageSetupDialogData {
        unsafe { @wxPageSetupDialogData(wxPageSetupDialogData_Create()) as @PageSetupDialogData }
    }
    #[fixed_stack_segment]
    pub fn newFromData(printData: @PrintData) -> @PageSetupDialogData {
        unsafe { @wxPageSetupDialogData(wxPageSetupDialogData_CreateFromData(printData.handle())) as @PageSetupDialogData }
    }
}

trait PageSetupDialogData : Object {
    #[fixed_stack_segment]
    fn assign(&self, data: @PageSetupDialogData) {
        unsafe { wxPageSetupDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, printData: @PrintData) {
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
    fn getMarginBottomRight(&self) -> @Point {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMarginBottomRight(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getMarginTopLeft(&self) -> @Point {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMarginTopLeft(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getMinMarginBottomRight(&self) -> @Point {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMinMarginBottomRight(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getMinMarginTopLeft(&self) -> @Point {
        unsafe { @wxPoint(wxPageSetupDialogData_GetMinMarginTopLeft(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperSize(&self) -> @Size {
        unsafe { @wxSize(wxPageSetupDialogData_GetPaperSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @PrintData) {
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
    fn setPrintData(&self, printData: @PrintData) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.handle(), printData.handle()) }
    }
}

struct wxPaintDC(*u8);
impl PaintDC for wxPaintDC {}
impl WindowDC for wxPaintDC {}
impl DC for wxPaintDC {}
impl Object for wxPaintDC { fn handle(&self) -> *u8 { **self } }

impl wxPaintDC {
    #[fixed_stack_segment]
    pub fn new(win: @Window) -> @PaintDC {
        unsafe { @wxPaintDC(wxPaintDC_Create(win.handle())) as @PaintDC }
    }
}

trait PaintDC : WindowDC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPaintDC_Delete(self.handle()) }
    }
}

struct wxPaintEvent(*u8);
impl PaintEvent for wxPaintEvent {}
impl Event for wxPaintEvent {}
impl Object for wxPaintEvent { fn handle(&self) -> *u8 { **self } }

impl wxPaintEvent {
}

trait PaintEvent : Event {
}

struct wxPalette(*u8);
impl Palette for wxPalette {}
impl GDIObject for wxPalette {}
impl Object for wxPalette { fn handle(&self) -> *u8 { **self } }

impl wxPalette {
    #[fixed_stack_segment]
    pub fn newDefault() -> @Palette {
        unsafe { @wxPalette(wxPalette_CreateDefault()) as @Palette }
    }
    #[fixed_stack_segment]
    pub fn newRGB(n: c_int, red: *u8, green: *u8, blue: *u8) -> @Palette {
        unsafe { @wxPalette(wxPalette_CreateRGB(n, red, green, blue)) as @Palette }
    }
}

trait Palette : GDIObject {
    #[fixed_stack_segment]
    fn assign(&self, palette: @Palette) {
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
    fn isEqual(&self, palette: @Palette) -> bool {
        unsafe { wxPalette_IsEqual(self.handle(), palette.handle()) }
    }
    #[fixed_stack_segment]
    fn isOk(&self) -> bool {
        unsafe { wxPalette_IsOk(self.handle()) }
    }
}

struct wxPaletteChangedEvent(*u8);
impl PaletteChangedEvent for wxPaletteChangedEvent {}
impl Event for wxPaletteChangedEvent {}
impl Object for wxPaletteChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxPaletteChangedEvent {
}

trait PaletteChangedEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxPaletteChangedEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getChangedWindow(&self) -> *u8 {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setChangedWindow(&self, win: @Window) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.handle(), win.handle()) }
    }
}

struct wxPanel(*u8);
impl Panel for wxPanel {}
impl Window for wxPanel {}
impl EvtHandler for wxPanel {}
impl Object for wxPanel { fn handle(&self) -> *u8 { **self } }

impl wxPanel {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @Panel {
        unsafe { @wxPanel(wxPanel_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @Panel }
    }
}

trait Panel : Window {
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
impl PathList for wxPathList {}
impl List for wxPathList {}
impl Object for wxPathList { fn handle(&self) -> *u8 { **self } }

impl wxPathList {
}

trait PathList : List {
}

struct wxPen(*u8);
impl Pen for wxPen {}
impl GDIObject for wxPen {}
impl Object for wxPen { fn handle(&self) -> *u8 { **self } }

impl wxPen {
    #[fixed_stack_segment]
    pub fn newDefault() -> @Pen {
        unsafe { @wxPen(wxPen_CreateDefault()) as @Pen }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(stipple: @Bitmap, width: c_int) -> @Pen {
        unsafe { @wxPen(wxPen_CreateFromBitmap(stipple.handle(), width)) as @Pen }
    }
    #[fixed_stack_segment]
    pub fn newFromColour(col: @Colour, width: c_int, style: c_int) -> @Pen {
        unsafe { @wxPen(wxPen_CreateFromColour(col.handle(), width, style)) as @Pen }
    }
    #[fixed_stack_segment]
    pub fn newFromStock(id: c_int) -> @Pen {
        unsafe { @wxPen(wxPen_CreateFromStock(id)) as @Pen }
    }
}

trait Pen : GDIObject {
    #[fixed_stack_segment]
    fn assign(&self, pen: @Pen) {
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
    fn getColour(&self, _ref: @Colour) {
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
    fn getStipple(&self, _ref: @Bitmap) {
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
    fn isEqual(&self, pen: @Pen) -> bool {
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
    fn setColour(&self, col: @Colour) {
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
    fn setStipple(&self, stipple: @Bitmap) {
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
impl PenList for wxPenList {}
impl List for wxPenList {}
impl Object for wxPenList { fn handle(&self) -> *u8 { **self } }

impl wxPenList {
}

trait PenList : List {
}

struct wxPlotCurve(*u8);
impl PlotCurve for wxPlotCurve {}
impl Object for wxPlotCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotCurve {
}

trait PlotCurve : Object {
}

struct wxPlotEvent(*u8);
impl PlotEvent for wxPlotEvent {}
impl NotifyEvent for wxPlotEvent {}
impl CommandEvent for wxPlotEvent {}
impl Event for wxPlotEvent {}
impl Object for wxPlotEvent { fn handle(&self) -> *u8 { **self } }

impl wxPlotEvent {
}

trait PlotEvent : NotifyEvent {
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

struct wxPlotOnOffCurve(*u8);
impl PlotOnOffCurve for wxPlotOnOffCurve {}
impl Object for wxPlotOnOffCurve { fn handle(&self) -> *u8 { **self } }

impl wxPlotOnOffCurve {
}

trait PlotOnOffCurve : Object {
}

struct wxPlotWindow(*u8);
impl PlotWindow for wxPlotWindow {}
impl ScrolledWindow for wxPlotWindow {}
impl Panel for wxPlotWindow {}
impl Window for wxPlotWindow {}
impl EvtHandler for wxPlotWindow {}
impl Object for wxPlotWindow { fn handle(&self) -> *u8 { **self } }

impl wxPlotWindow {
}

trait PlotWindow : ScrolledWindow {
}

struct wxPoint(*u8);
impl Point for wxPoint { fn handle(&self) -> *u8 { **self } }

impl wxPoint {
    #[fixed_stack_segment]
    pub fn new(xx: c_int, yy: c_int) -> @Point {
        unsafe { @wxPoint(wxPoint_Create(xx, yy)) as @Point }
    }
}

trait Point {
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
impl PopupTransientWindow for wxPopupTransientWindow {}
impl PopupWindow for wxPopupTransientWindow {}
impl Window for wxPopupTransientWindow {}
impl EvtHandler for wxPopupTransientWindow {}
impl Object for wxPopupTransientWindow { fn handle(&self) -> *u8 { **self } }

impl wxPopupTransientWindow {
}

trait PopupTransientWindow : PopupWindow {
}

struct wxPopupWindow(*u8);
impl PopupWindow for wxPopupWindow {}
impl Window for wxPopupWindow {}
impl EvtHandler for wxPopupWindow {}
impl Object for wxPopupWindow { fn handle(&self) -> *u8 { **self } }

impl wxPopupWindow {
}

trait PopupWindow : Window {
}

struct wxPostScriptDC(*u8);
impl PostScriptDC for wxPostScriptDC {}
impl DC for wxPostScriptDC {}
impl Object for wxPostScriptDC { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptDC {
    #[fixed_stack_segment]
    pub fn new(data: @PrintData) -> @PostScriptDC {
        unsafe { @wxPostScriptDC(wxPostScriptDC_Create(data.handle())) as @PostScriptDC }
    }
}

trait PostScriptDC : DC {
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
impl PreviewCanvas for wxPreviewCanvas {}
impl ScrolledWindow for wxPreviewCanvas {}
impl Panel for wxPreviewCanvas {}
impl Window for wxPreviewCanvas {}
impl EvtHandler for wxPreviewCanvas {}
impl Object for wxPreviewCanvas { fn handle(&self) -> *u8 { **self } }

impl wxPreviewCanvas {
    #[fixed_stack_segment]
    pub fn new(preview: @PrintPreview, parent: @Window, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @PreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPreviewCanvas_Create(preview.handle(), parent.handle(), x, y, w, h, style)) as @PreviewCanvas }
    }
}

trait PreviewCanvas : ScrolledWindow {
}

struct wxPreviewControlBar(*u8);
impl PreviewControlBar for wxPreviewControlBar {}
impl Panel for wxPreviewControlBar {}
impl Window for wxPreviewControlBar {}
impl EvtHandler for wxPreviewControlBar {}
impl Object for wxPreviewControlBar { fn handle(&self) -> *u8 { **self } }

impl wxPreviewControlBar {
}

trait PreviewControlBar : Panel {
}

struct wxPreviewFrame(*u8);
impl PreviewFrame for wxPreviewFrame {}
impl Frame for wxPreviewFrame {}
impl TopLevelWindow for wxPreviewFrame {}
impl Window for wxPreviewFrame {}
impl EvtHandler for wxPreviewFrame {}
impl Object for wxPreviewFrame { fn handle(&self) -> *u8 { **self } }

impl wxPreviewFrame {
    #[fixed_stack_segment]
    pub fn new(preview: @PrintPreview, parent: @Frame, title: @String, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: @String) -> @PreviewFrame {
        unsafe { @wxPreviewFrame(wxPreviewFrame_Create(preview.handle(), parent.handle(), title.handle(), x, y, width, height, style, name.handle())) as @PreviewFrame }
    }
}

trait PreviewFrame : Frame {
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
impl PrintData for wxPrintData {}
impl Object for wxPrintData { fn handle(&self) -> *u8 { **self } }

impl wxPrintData {
    #[fixed_stack_segment]
    pub fn new() -> @PrintData {
        unsafe { @wxPrintData(wxPrintData_Create()) as @PrintData }
    }
}

trait PrintData : Object {
    #[fixed_stack_segment]
    fn assign(&self, data: @PrintData) {
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
    fn getFilename(&self) -> @String {
        unsafe { @wxString(wxPrintData_GetFilename(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getFontMetricPath(&self) -> @String {
        unsafe { @wxString(wxPrintData_GetFontMetricPath(self.handle())) as @String }
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
    fn getPaperSize(&self) -> @Size {
        unsafe { @wxSize(wxPrintData_GetPaperSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getPreviewCommand(&self) -> @String {
        unsafe { @wxString(wxPrintData_GetPreviewCommand(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getPrintMode(&self) -> c_int {
        unsafe { wxPrintData_GetPrintMode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrinterCommand(&self) -> @String {
        unsafe { @wxString(wxPrintData_GetPrinterCommand(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getPrinterName(&self) -> @String {
        unsafe { @wxString(wxPrintData_GetPrinterName(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getPrinterOptions(&self) -> @String {
        unsafe { @wxString(wxPrintData_GetPrinterOptions(self.handle())) as @String }
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
    fn setFilename(&self, filename: @String) {
        unsafe { wxPrintData_SetFilename(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn setFontMetricPath(&self, path: @String) {
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
    fn setPreviewCommand(&self, command: @Command) {
        unsafe { wxPrintData_SetPreviewCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.handle(), printMode) }
    }
    #[fixed_stack_segment]
    fn setPrinterCommand(&self, command: @Command) {
        unsafe { wxPrintData_SetPrinterCommand(self.handle(), command.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterName(&self, name: @String) {
        unsafe { wxPrintData_SetPrinterName(self.handle(), name.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrinterOptions(&self, options: @String) {
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
impl PostScriptPrintNativeData for wxPostScriptPrintNativeData {}
impl Object for wxPostScriptPrintNativeData { fn handle(&self) -> *u8 { **self } }

impl wxPostScriptPrintNativeData {
    #[fixed_stack_segment]
    pub fn new() -> @PostScriptPrintNativeData {
        unsafe { @wxPostScriptPrintNativeData(wxPostScriptPrintNativeData_Create()) as @PostScriptPrintNativeData }
    }
}

trait PostScriptPrintNativeData : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPostScriptPrintNativeData_Delete(self.handle()) }
    }
}

struct wxPrintDialog(*u8);
impl PrintDialog for wxPrintDialog {}
impl Dialog for wxPrintDialog {}
impl TopLevelWindow for wxPrintDialog {}
impl Window for wxPrintDialog {}
impl EvtHandler for wxPrintDialog {}
impl Object for wxPrintDialog { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialog {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, data: @PrintDialogData) -> @PrintDialog {
        unsafe { @wxPrintDialog(wxPrintDialog_Create(parent.handle(), data.handle())) as @PrintDialog }
    }
}

trait PrintDialog : Dialog {
    #[fixed_stack_segment]
    fn getPrintDC(&self) -> @DC {
        unsafe { @wxDC(wxPrintDialog_GetPrintDC(self.handle())) as @DC }
    }
    #[fixed_stack_segment]
    fn getPrintData(&self, _ref: @PrintData) {
        unsafe { wxPrintDialog_GetPrintData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintDialogData(&self) -> @PrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialog_GetPrintDialogData(self.handle())) as @PrintDialogData }
    }
}

struct wxPrintDialogData(*u8);
impl PrintDialogData for wxPrintDialogData {}
impl Object for wxPrintDialogData { fn handle(&self) -> *u8 { **self } }

impl wxPrintDialogData {
    #[fixed_stack_segment]
    pub fn newDefault() -> @PrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialogData_CreateDefault()) as @PrintDialogData }
    }
    #[fixed_stack_segment]
    pub fn newFromData(printData: @PrintData) -> @PrintDialogData {
        unsafe { @wxPrintDialogData(wxPrintDialogData_CreateFromData(printData.handle())) as @PrintDialogData }
    }
}

trait PrintDialogData : Object {
    #[fixed_stack_segment]
    fn assign(&self, data: @PrintDialogData) {
        unsafe { wxPrintDialogData_Assign(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn assignData(&self, data: @PrintData) {
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
    fn getPrintData(&self, _ref: @PrintData) {
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
    fn setPrintData(&self, printData: @PrintData) {
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
impl PrintPreview for wxPrintPreview {}
impl Object for wxPrintPreview { fn handle(&self) -> *u8 { **self } }

impl wxPrintPreview {
    #[fixed_stack_segment]
    pub fn newFromData(printout: @Printout, printoutForPrinting: @Printout, data: @PrintData) -> @PrintPreview {
        unsafe { @wxPrintPreview(wxPrintPreview_CreateFromData(printout.handle(), printoutForPrinting.handle(), data.handle())) as @PrintPreview }
    }
    #[fixed_stack_segment]
    pub fn newFromDialogData(printout: @Printout, printoutForPrinting: @Printout, data: @PrintDialogData) -> @PrintPreview {
        unsafe { @wxPrintPreview(wxPrintPreview_CreateFromDialogData(printout.handle(), printoutForPrinting.handle(), data.handle())) as @PrintPreview }
    }
}

trait PrintPreview : Object {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrintPreview_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.handle()) }
    }
    #[fixed_stack_segment]
    fn drawBlankPage(&self, canvas: @PreviewCanvas, dc: @DC) -> bool {
        unsafe { wxPrintPreview_DrawBlankPage(self.handle(), canvas.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn getCanvas(&self) -> @PreviewCanvas {
        unsafe { @wxPreviewCanvas(wxPrintPreview_GetCanvas(self.handle())) as @PreviewCanvas }
    }
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getFrame(&self) -> @Frame {
        unsafe { @wxFrame(wxPrintPreview_GetFrame(self.handle())) as @Frame }
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
    fn getPrintDialogData(&self, _ref: @PrintDialogData) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @Printout {
        unsafe { @wxPrintout(wxPrintPreview_GetPrintout(self.handle())) as @Printout }
    }
    #[fixed_stack_segment]
    fn getPrintoutForPrinting(&self) -> @Printout {
        unsafe { @wxPrintout(wxPrintPreview_GetPrintoutForPrinting(self.handle())) as @Printout }
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
    fn paintPage(&self, canvas: @PrintPreview, dc: @DC) -> bool {
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
    fn setCanvas(&self, canvas: @PreviewCanvas) {
        unsafe { wxPrintPreview_SetCanvas(self.handle(), canvas.handle()) }
    }
    #[fixed_stack_segment]
    fn setCurrentPage(&self, pageNum: c_int) -> bool {
        unsafe { wxPrintPreview_SetCurrentPage(self.handle(), pageNum) }
    }
    #[fixed_stack_segment]
    fn setFrame(&self, frame: @Frame) {
        unsafe { wxPrintPreview_SetFrame(self.handle(), frame.handle()) }
    }
    #[fixed_stack_segment]
    fn setOk(&self, ok: bool) {
        unsafe { wxPrintPreview_SetOk(self.handle(), ok) }
    }
    #[fixed_stack_segment]
    fn setPrintout(&self, printout: @Printout) {
        unsafe { wxPrintPreview_SetPrintout(self.handle(), printout.handle()) }
    }
    #[fixed_stack_segment]
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.handle(), percent) }
    }
}

struct wxPrinter(*u8);
impl Printer for wxPrinter {}
impl Object for wxPrinter { fn handle(&self) -> *u8 { **self } }

impl wxPrinter {
    #[fixed_stack_segment]
    pub fn new(data: @PrintDialogData) -> @Printer {
        unsafe { @wxPrinter(wxPrinter_Create(data.handle())) as @Printer }
    }
}

trait Printer : Object {
    #[fixed_stack_segment]
    fn newAbortWindow(&self, parent: @Window, printout: @Printout) -> @Window {
        unsafe { @wxWindow(wxPrinter_CreateAbortWindow(self.handle(), parent.handle(), printout.handle())) as @Window }
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
    fn getPrintDialogData(&self, _ref: @PrintDialogData) {
        unsafe { wxPrinter_GetPrintDialogData(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn print(&self, parent: @Window, printout: @Printout, prompt: bool) -> bool {
        unsafe { wxPrinter_Print(self.handle(), parent.handle(), printout.handle(), prompt) }
    }
    #[fixed_stack_segment]
    fn printDialog(&self, parent: @Window) -> @DC {
        unsafe { @wxDC(wxPrinter_PrintDialog(self.handle(), parent.handle())) as @DC }
    }
    #[fixed_stack_segment]
    fn reportError(&self, parent: @Window, printout: @Printout, message: @String) {
        unsafe { wxPrinter_ReportError(self.handle(), parent.handle(), printout.handle(), message.handle()) }
    }
    #[fixed_stack_segment]
    fn setup(&self, parent: @Window) -> bool {
        unsafe { wxPrinter_Setup(self.handle(), parent.handle()) }
    }
}

struct wxPrinterDC(*u8);
impl PrinterDC for wxPrinterDC {}
impl DC for wxPrinterDC {}
impl Object for wxPrinterDC { fn handle(&self) -> *u8 { **self } }

impl wxPrinterDC {
    #[fixed_stack_segment]
    pub fn new(data: @PrintData) -> @PrinterDC {
        unsafe { @wxPrinterDC(wxPrinterDC_Create(data.handle())) as @PrinterDC }
    }
}

trait PrinterDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxPrinterDC_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getPaperRect(&self) -> @Rect {
        unsafe { @wxRect(wxPrinterDC_GetPaperRect(self.handle())) as @Rect }
    }
}

struct wxPrintout(*u8);
impl Printout for wxPrintout {}
impl Object for wxPrintout { fn handle(&self) -> *u8 { **self } }

impl wxPrintout {
}

trait Printout : Object {
    #[fixed_stack_segment]
    fn getDC(&self) -> @DC {
        unsafe { @wxDC(wxPrintout_GetDC(self.handle())) as @DC }
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
    fn getTitle(&self) -> @String {
        unsafe { @wxString(wxPrintout_GetTitle(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn isPreview(&self) -> bool {
        unsafe { wxPrintout_IsPreview(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setDC(&self, dc: @DC) {
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
impl PrivateDropTarget for wxPrivateDropTarget {}
impl DropTarget for wxPrivateDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxPrivateDropTarget {
}

trait PrivateDropTarget : DropTarget {
}

struct wxProcess(*u8);
impl Process for wxProcess {}
impl EvtHandler for wxProcess {}
impl Object for wxProcess { fn handle(&self) -> *u8 { **self } }

impl wxProcess {
    #[fixed_stack_segment]
    pub fn newDefault(_prt: @Window, _id: c_int) -> @Process {
        unsafe { @wxProcess(wxProcess_CreateDefault(_prt.handle(), _id)) as @Process }
    }
    #[fixed_stack_segment]
    pub fn newRedirect(_prt: @Window, _rdr: bool) -> @Process {
        unsafe { @wxProcess(wxProcess_CreateRedirect(_prt.handle(), _rdr)) as @Process }
    }
    #[fixed_stack_segment]
    pub fn open(cmd: @String, flags: c_int) -> @Process {
        unsafe { @wxProcess(wxProcess_Open(cmd.handle(), flags)) as @Process }
    }
}

trait Process : EvtHandler {
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
    fn getErrorStream(&self) -> @InputStream {
        unsafe { @wxInputStream(wxProcess_GetErrorStream(self.handle())) as @InputStream }
    }
    #[fixed_stack_segment]
    fn getInputStream(&self) -> @InputStream {
        unsafe { @wxInputStream(wxProcess_GetInputStream(self.handle())) as @InputStream }
    }
    #[fixed_stack_segment]
    fn getOutputStream(&self) -> @OutputStream {
        unsafe { @wxOutputStream(wxProcess_GetOutputStream(self.handle())) as @OutputStream }
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
impl ProcessEvent for wxProcessEvent {}
impl Event for wxProcessEvent {}
impl Object for wxProcessEvent { fn handle(&self) -> *u8 { **self } }

impl wxProcessEvent {
}

trait ProcessEvent : Event {
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
impl ProgressDialog for wxProgressDialog {}
impl Frame for wxProgressDialog {}
impl TopLevelWindow for wxProgressDialog {}
impl Window for wxProgressDialog {}
impl EvtHandler for wxProgressDialog {}
impl Object for wxProgressDialog { fn handle(&self) -> *u8 { **self } }

impl wxProgressDialog {
    #[fixed_stack_segment]
    pub fn new(title: @String, message: @String, max: c_int, parent: @Window, style: c_int) -> @ProgressDialog {
        unsafe { @wxProgressDialog(wxProgressDialog_Create(title.handle(), message.handle(), max, parent.handle(), style)) as @ProgressDialog }
    }
}

trait ProgressDialog : Frame {
    #[fixed_stack_segment]
    fn update(&self, value: c_int) -> bool {
        unsafe { wxProgressDialog_Update(self.handle(), value) }
    }
    #[fixed_stack_segment]
    fn updateWithMessage(&self, value: c_int, message: @String) -> bool {
        unsafe { wxProgressDialog_UpdateWithMessage(self.handle(), value, message.handle()) }
    }
    #[fixed_stack_segment]
    fn resume(&self) {
        unsafe { wxProgressDialog_Resume(self.handle()) }
    }
}

struct wxProtocol(*u8);
impl Protocol for wxProtocol {}
impl SocketClient for wxProtocol {}
impl SocketBase for wxProtocol {}
impl Object for wxProtocol { fn handle(&self) -> *u8 { **self } }

impl wxProtocol {
}

trait Protocol : SocketClient {
}

struct wxQuantize(*u8);
impl Quantize for wxQuantize {}
impl Object for wxQuantize { fn handle(&self) -> *u8 { **self } }

impl wxQuantize {
}

trait Quantize : Object {
}

struct wxQueryCol(*u8);
impl QueryCol for wxQueryCol {}
impl Object for wxQueryCol { fn handle(&self) -> *u8 { **self } }

impl wxQueryCol {
}

trait QueryCol : Object {
}

struct wxQueryField(*u8);
impl QueryField for wxQueryField {}
impl Object for wxQueryField { fn handle(&self) -> *u8 { **self } }

impl wxQueryField {
}

trait QueryField : Object {
}

struct wxQueryLayoutInfoEvent(*u8);
impl QueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl Event for wxQueryLayoutInfoEvent {}
impl Object for wxQueryLayoutInfoEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryLayoutInfoEvent {
    #[fixed_stack_segment]
    pub fn new(id: c_int) -> @QueryLayoutInfoEvent {
        unsafe { @wxQueryLayoutInfoEvent(wxQueryLayoutInfoEvent_Create(id)) as @QueryLayoutInfoEvent }
    }
}

trait QueryLayoutInfoEvent : Event {
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
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxQueryLayoutInfoEvent_GetSize(self.handle())) as @Size }
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
impl QueryNewPaletteEvent for wxQueryNewPaletteEvent {}
impl Event for wxQueryNewPaletteEvent {}
impl Object for wxQueryNewPaletteEvent { fn handle(&self) -> *u8 { **self } }

impl wxQueryNewPaletteEvent {
}

trait QueryNewPaletteEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @Object) {
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
impl RadioBox for wxRadioBox {}
impl Control for wxRadioBox {}
impl Window for wxRadioBox {}
impl EvtHandler for wxRadioBox {}
impl Object for wxRadioBox { fn handle(&self) -> *u8 { **self } }

impl wxRadioBox {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: c_int, _stl: c_int) -> @RadioBox {
        unsafe { @wxRadioBox(wxRadioBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl)) as @RadioBox }
    }
}

trait RadioBox : Control {
    #[fixed_stack_segment]
    fn enableItem(&self, item: c_int, enable: bool) {
        unsafe { wxRadioBox_EnableItem(self.handle(), item, enable) }
    }
    #[fixed_stack_segment]
    fn findString(&self, s: @String) -> c_int {
        unsafe { wxRadioBox_FindString(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemLabel(&self, item: c_int) -> @String {
        unsafe { @wxString(wxRadioBox_GetItemLabel(self.handle(), item)) as @String }
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
    fn getStringSelection(&self) -> @String {
        unsafe { @wxString(wxRadioBox_GetStringSelection(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn number(&self) -> c_int {
        unsafe { wxRadioBox_Number(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBitmap(&self, item: c_int, bitmap: @Bitmap) {
        unsafe { wxRadioBox_SetItemBitmap(self.handle(), item, bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemLabel(&self, item: c_int, label: @String) {
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
    fn setStringSelection(&self, s: @String) {
        unsafe { wxRadioBox_SetStringSelection(self.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    fn showItem(&self, item: c_int, show: bool) {
        unsafe { wxRadioBox_ShowItem(self.handle(), item, show) }
    }
}

struct wxRadioButton(*u8);
impl RadioButton for wxRadioButton {}
impl Control for wxRadioButton {}
impl Window for wxRadioButton {}
impl EvtHandler for wxRadioButton {}
impl Object for wxRadioButton { fn handle(&self) -> *u8 { **self } }

impl wxRadioButton {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @RadioButton {
        unsafe { @wxRadioButton(wxRadioButton_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @RadioButton }
    }
}

trait RadioButton : Control {
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
impl RealPoint for wxRealPoint { fn handle(&self) -> *u8 { **self } }

impl wxRealPoint {
}

trait RealPoint {
    fn handle(&self) -> *u8;
    
}

struct wxRecordSet(*u8);
impl RecordSet for wxRecordSet {}
impl Object for wxRecordSet { fn handle(&self) -> *u8 { **self } }

impl wxRecordSet {
}

trait RecordSet : Object {
}

struct wxRect(*u8);
impl Rect for wxRect { fn handle(&self) -> *u8 { **self } }

impl wxRect {
}

trait Rect {
    fn handle(&self) -> *u8;
    
}

struct wxRegEx(*u8);
impl RegEx for wxRegEx { fn handle(&self) -> *u8 { **self } }

impl wxRegEx {
}

trait RegEx {
    fn handle(&self) -> *u8;
    
}

struct wxRegion(*u8);
impl Region for wxRegion {}
impl GDIObject for wxRegion {}
impl Object for wxRegion { fn handle(&self) -> *u8 { **self } }

impl wxRegion {
    #[fixed_stack_segment]
    pub fn newDefault() -> @Region {
        unsafe { @wxRegion(wxRegion_CreateDefault()) as @Region }
    }
    #[fixed_stack_segment]
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> @Region {
        unsafe { @wxRegion(wxRegion_CreateFromRect(x, y, w, h)) as @Region }
    }
}

trait Region : GDIObject {
    #[fixed_stack_segment]
    fn assign(&self, region: @Region) {
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
    fn intersectRegion(&self, region: @Region) -> bool {
        unsafe { wxRegion_IntersectRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_SubtractRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn subtractRegion(&self, region: @Region) -> bool {
        unsafe { wxRegion_SubtractRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_UnionRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn unionRegion(&self, region: @Region) -> bool {
        unsafe { wxRegion_UnionRegion(self.handle(), region.handle()) }
    }
    #[fixed_stack_segment]
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxRegion_XorRect(self.handle(), x, y, width, height) }
    }
    #[fixed_stack_segment]
    fn xorRegion(&self, region: @Region) -> bool {
        unsafe { wxRegion_XorRegion(self.handle(), region.handle()) }
    }
}

struct wxRegionIterator(*u8);
impl RegionIterator for wxRegionIterator {}
impl Object for wxRegionIterator { fn handle(&self) -> *u8 { **self } }

impl wxRegionIterator {
    #[fixed_stack_segment]
    pub fn new() -> @RegionIterator {
        unsafe { @wxRegionIterator(wxRegionIterator_Create()) as @RegionIterator }
    }
    #[fixed_stack_segment]
    pub fn newFromRegion(region: @Region) -> @RegionIterator {
        unsafe { @wxRegionIterator(wxRegionIterator_CreateFromRegion(region.handle())) as @RegionIterator }
    }
}

trait RegionIterator : Object {
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
    fn resetToRegion(&self, region: @Region) {
        unsafe { wxRegionIterator_ResetToRegion(self.handle(), region.handle()) }
    }
}

struct wxRemotelyScrolledTreeCtrl(*u8);
impl RemotelyScrolledTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl TreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl Control for wxRemotelyScrolledTreeCtrl {}
impl Window for wxRemotelyScrolledTreeCtrl {}
impl EvtHandler for wxRemotelyScrolledTreeCtrl {}
impl Object for wxRemotelyScrolledTreeCtrl { fn handle(&self) -> *u8 { **self } }

impl wxRemotelyScrolledTreeCtrl {
}

trait RemotelyScrolledTreeCtrl : TreeCtrl {
}

struct wxSVGFileDC(*u8);
impl SVGFileDC for wxSVGFileDC {}
impl DC for wxSVGFileDC {}
impl Object for wxSVGFileDC { fn handle(&self) -> *u8 { **self } }

impl wxSVGFileDC {
    #[fixed_stack_segment]
    pub fn new(fileName: @String) -> @SVGFileDC {
        unsafe { @wxSVGFileDC(wxSVGFileDC_Create(fileName.handle())) as @SVGFileDC }
    }
    #[fixed_stack_segment]
    pub fn newWithSize(fileName: @String, w: c_int, h: c_int) -> @SVGFileDC {
        unsafe { @wxSVGFileDC(wxSVGFileDC_CreateWithSize(fileName.handle(), w, h)) as @SVGFileDC }
    }
    #[fixed_stack_segment]
    pub fn newWithSizeAndResolution(fileName: @String, w: c_int, h: c_int, a_dpi: c_float) -> @SVGFileDC {
        unsafe { @wxSVGFileDC(wxSVGFileDC_CreateWithSizeAndResolution(fileName.handle(), w, h, a_dpi)) as @SVGFileDC }
    }
}

trait SVGFileDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxSVGFileDC_Delete(self.handle()) }
    }
}

struct wxSashEvent(*u8);
impl SashEvent for wxSashEvent {}
impl Event for wxSashEvent {}
impl Object for wxSashEvent { fn handle(&self) -> *u8 { **self } }

impl wxSashEvent {
    #[fixed_stack_segment]
    pub fn new(id: c_int, edge: c_int) -> @SashEvent {
        unsafe { @wxSashEvent(wxSashEvent_Create(id, edge)) as @SashEvent }
    }
}

trait SashEvent : Event {
    #[fixed_stack_segment]
    fn getDragRect(&self) -> @Rect {
        unsafe { @wxRect(wxSashEvent_GetDragRect(self.handle())) as @Rect }
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
impl SashLayoutWindow for wxSashLayoutWindow {}
impl SashWindow for wxSashLayoutWindow {}
impl Window for wxSashLayoutWindow {}
impl EvtHandler for wxSashLayoutWindow {}
impl Object for wxSashLayoutWindow { fn handle(&self) -> *u8 { **self } }

impl wxSashLayoutWindow {
    #[fixed_stack_segment]
    pub fn new(_par: @Window, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @SashLayoutWindow {
        unsafe { @wxSashLayoutWindow(wxSashLayoutWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) as @SashLayoutWindow }
    }
}

trait SashLayoutWindow : SashWindow {
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
impl SashWindow for wxSashWindow {}
impl Window for wxSashWindow {}
impl EvtHandler for wxSashWindow {}
impl Object for wxSashWindow { fn handle(&self) -> *u8 { **self } }

impl wxSashWindow {
    #[fixed_stack_segment]
    pub fn new(_par: @Window, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @SashWindow {
        unsafe { @wxSashWindow(wxSashWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl)) as @SashWindow }
    }
}

trait SashWindow : Window {
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
impl ScopedArray for wxScopedArray { fn handle(&self) -> *u8 { **self } }

impl wxScopedArray {
}

trait ScopedArray {
    fn handle(&self) -> *u8;
    
}

struct wxScopedPtr(*u8);
impl ScopedPtr for wxScopedPtr { fn handle(&self) -> *u8 { **self } }

impl wxScopedPtr {
}

trait ScopedPtr {
    fn handle(&self) -> *u8;
    
}

struct wxScreenDC(*u8);
impl ScreenDC for wxScreenDC {}
impl DC for wxScreenDC {}
impl Object for wxScreenDC { fn handle(&self) -> *u8 { **self } }

impl wxScreenDC {
    #[fixed_stack_segment]
    pub fn new() -> @ScreenDC {
        unsafe { @wxScreenDC(wxScreenDC_Create()) as @ScreenDC }
    }
}

trait ScreenDC : DC {
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
    fn startDrawingOnTopOfWin(&self, win: @Window) -> bool {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.handle(), win.handle()) }
    }
}

struct wxScrollBar(*u8);
impl ScrollBar for wxScrollBar {}
impl Control for wxScrollBar {}
impl Window for wxScrollBar {}
impl EvtHandler for wxScrollBar {}
impl Object for wxScrollBar { fn handle(&self) -> *u8 { **self } }

impl wxScrollBar {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @ScrollBar {
        unsafe { @wxScrollBar(wxScrollBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @ScrollBar }
    }
}

trait ScrollBar : Control {
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
impl ScrollEvent for wxScrollEvent {}
impl Event for wxScrollEvent {}
impl Object for wxScrollEvent { fn handle(&self) -> *u8 { **self } }

impl wxScrollEvent {
}

trait ScrollEvent : Event {
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
impl ScrollWinEvent for wxScrollWinEvent {}
impl Event for wxScrollWinEvent {}
impl Object for wxScrollWinEvent { fn handle(&self) -> *u8 { **self } }

impl wxScrollWinEvent {
}

trait ScrollWinEvent : Event {
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
impl ScrolledWindow for wxScrolledWindow {}
impl Panel for wxScrolledWindow {}
impl Window for wxScrolledWindow {}
impl EvtHandler for wxScrolledWindow {}
impl Object for wxScrolledWindow { fn handle(&self) -> *u8 { **self } }

impl wxScrolledWindow {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @ScrolledWindow {
        unsafe { @wxScrolledWindow(wxScrolledWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @ScrolledWindow }
    }
}

trait ScrolledWindow : Panel {
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
    fn getTargetWindow(&self) -> @Window {
        unsafe { @wxWindow(wxScrolledWindow_GetTargetWindow(self.handle())) as @Window }
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
    fn onDraw(&self, dc: @DC) {
        unsafe { wxScrolledWindow_OnDraw(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @DC) {
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
    fn setTargetWindow(&self, target: @Window) {
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
impl Semaphore for wxSemaphore { fn handle(&self) -> *u8 { **self } }

impl wxSemaphore {
}

trait Semaphore {
    fn handle(&self) -> *u8;
    
}

struct wxServer(*u8);
impl Server for wxServer {}
impl ServerBase for wxServer {}
impl Object for wxServer { fn handle(&self) -> *u8 { **self } }

impl wxServer {
}

trait Server : ServerBase {
}

struct wxServerBase(*u8);
impl ServerBase for wxServerBase {}
impl Object for wxServerBase { fn handle(&self) -> *u8 { **self } }

impl wxServerBase {
}

trait ServerBase : Object {
}

struct wxSetCursorEvent(*u8);
impl SetCursorEvent for wxSetCursorEvent {}
impl Event for wxSetCursorEvent {}
impl Object for wxSetCursorEvent { fn handle(&self) -> *u8 { **self } }

impl wxSetCursorEvent {
}

trait SetCursorEvent : Event {
    #[fixed_stack_segment]
    fn getCursor(&self) -> @Cursor {
        unsafe { @wxCursor(wxSetCursorEvent_GetCursor(self.handle())) as @Cursor }
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
    fn setCursor(&self, cursor: @Cursor) {
        unsafe { wxSetCursorEvent_SetCursor(self.handle(), cursor.handle()) }
    }
}

struct wxShowEvent(*u8);
impl ShowEvent for wxShowEvent {}
impl Event for wxShowEvent {}
impl Object for wxShowEvent { fn handle(&self) -> *u8 { **self } }

impl wxShowEvent {
}

trait ShowEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @Object) {
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
impl SimpleHelpProvider for wxSimpleHelpProvider {}
impl HelpProvider for wxSimpleHelpProvider { fn handle(&self) -> *u8 { **self } }

impl wxSimpleHelpProvider {
    #[fixed_stack_segment]
    pub fn new() -> @SimpleHelpProvider {
        unsafe { @wxSimpleHelpProvider(wxSimpleHelpProvider_Create()) as @SimpleHelpProvider }
    }
}

trait SimpleHelpProvider : HelpProvider {
}

struct wxSingleChoiceDialog(*u8);
impl SingleChoiceDialog for wxSingleChoiceDialog {}
impl Dialog for wxSingleChoiceDialog {}
impl TopLevelWindow for wxSingleChoiceDialog {}
impl Window for wxSingleChoiceDialog {}
impl EvtHandler for wxSingleChoiceDialog {}
impl Object for wxSingleChoiceDialog { fn handle(&self) -> *u8 { **self } }

impl wxSingleChoiceDialog {
}

trait SingleChoiceDialog : Dialog {
}

struct wxSingleInstanceChecker(*u8);
impl SingleInstanceChecker for wxSingleInstanceChecker { fn handle(&self) -> *u8 { **self } }

impl wxSingleInstanceChecker {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, name: @String, path: @String) -> bool {
        unsafe { wxSingleInstanceChecker_Create(_obj, name.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @SingleInstanceChecker {
        unsafe { @wxSingleInstanceChecker(wxSingleInstanceChecker_CreateDefault()) as @SingleInstanceChecker }
    }
}

trait SingleInstanceChecker {
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
impl Size for wxSize { fn handle(&self) -> *u8 { **self } }

impl wxSize {
    #[fixed_stack_segment]
    pub fn new(w: c_int, h: c_int) -> @Size {
        unsafe { @wxSize(wxSize_Create(w, h)) as @Size }
    }
}

trait Size {
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
impl SizeEvent for wxSizeEvent {}
impl Event for wxSizeEvent {}
impl Object for wxSizeEvent { fn handle(&self) -> *u8 { **self } }

impl wxSizeEvent {
}

trait SizeEvent : Event {
    #[fixed_stack_segment]
    fn copyObject(&self, obj: *u8) {
        unsafe { wxSizeEvent_CopyObject(self.handle(), obj) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxSizeEvent_GetSize(self.handle())) as @Size }
    }
}

struct wxSizer(*u8);
impl Sizer for wxSizer {}
impl Object for wxSizer { fn handle(&self) -> *u8 { **self } }

impl wxSizer {
}

trait Sizer : Object {
    #[fixed_stack_segment]
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Add(self.handle(), width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addSizer(&self, sizer: @Sizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn addWindow(&self, window: @Window, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_AddWindow(self.handle(), window.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn calcMin(&self) -> @Size {
        unsafe { @wxSize(wxSizer_CalcMin(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn fit(&self, window: @Window) {
        unsafe { wxSizer_Fit(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getChildren(&self, _res: *u8, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.handle(), _res, _cnt) }
    }
    #[fixed_stack_segment]
    fn getMinSize(&self) -> @Size {
        unsafe { @wxSize(wxSizer_GetMinSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxSizer_GetPosition(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxSizer_GetSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_Insert(self.handle(), before, width, height, option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertSizer(&self, before: c_int, sizer: @Sizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_InsertSizer(self.handle(), before, sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn insertWindow(&self, before: c_int, window: @Window, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn prependSizer(&self, sizer: @Sizer, option: c_int, flag: c_int, border: c_int, userData: *u8) {
        unsafe { wxSizer_PrependSizer(self.handle(), sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    fn prependWindow(&self, window: @Window, option: c_int, flag: c_int, border: c_int, userData: *u8) {
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
    fn setItemMinSizeSizer(&self, sizer: @Sizer, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.handle(), sizer.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setItemMinSizeWindow(&self, window: @Window, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.handle(), window.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.handle(), width, height) }
    }
    #[fixed_stack_segment]
    fn setSizeHints(&self, window: @Window) {
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
    fn detachWindow(&self, window: @Window) -> bool {
        unsafe { wxSizer_DetachWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn detachSizer(&self, sizer: @Sizer) -> bool {
        unsafe { wxSizer_DetachSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn detach(&self, index: c_int) -> bool {
        unsafe { wxSizer_Detach(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn fitInside(&self, window: @Window) {
        unsafe { wxSizer_FitInside(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn getContainingWindow(&self) -> @Window {
        unsafe { @wxWindow(wxSizer_GetContainingWindow(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn getItemWindow(&self, window: @Window, recursive: bool) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemWindow(self.handle(), window.handle(), recursive)) as @SizerItem }
    }
    #[fixed_stack_segment]
    fn getItemSizer(&self, window: @Sizer, recursive: bool) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItemSizer(self.handle(), window.handle(), recursive)) as @SizerItem }
    }
    #[fixed_stack_segment]
    fn getItem(&self, index: c_int) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_GetItem(self.handle(), index)) as @SizerItem }
    }
    #[fixed_stack_segment]
    fn hideWindow(&self, window: @Window) -> bool {
        unsafe { wxSizer_HideWindow(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn hideSizer(&self, sizer: @Sizer) -> bool {
        unsafe { wxSizer_HideSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn hide(&self, index: c_int) -> bool {
        unsafe { wxSizer_Hide(self.handle(), index) }
    }
    #[fixed_stack_segment]
    fn insertSpacer(&self, index: c_int, size: c_int) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_InsertSpacer(self.handle(), index, size)) as @SizerItem }
    }
    #[fixed_stack_segment]
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_InsertStretchSpacer(self.handle(), index, prop)) as @SizerItem }
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
    fn prependSpacer(&self, size: c_int) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_PrependSpacer(self.handle(), size)) as @SizerItem }
    }
    #[fixed_stack_segment]
    fn prependStretchSpacer(&self, prop: c_int) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizer_PrependStretchSpacer(self.handle(), prop)) as @SizerItem }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, oldwin: @Window, newwin: @Window, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceWindow(self.handle(), oldwin.handle(), newwin.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replaceSizer(&self, oldsz: @Sizer, newsz: @Sizer, recursive: bool) -> bool {
        unsafe { wxSizer_ReplaceSizer(self.handle(), oldsz.handle(), newsz.handle(), recursive) }
    }
    #[fixed_stack_segment]
    fn replace(&self, oldindex: c_int, newitem: @SizerItem) -> bool {
        unsafe { wxSizer_Replace(self.handle(), oldindex, newitem.handle()) }
    }
    #[fixed_stack_segment]
    fn setVirtualSizeHints(&self, window: @Window) {
        unsafe { wxSizer_SetVirtualSizeHints(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn showWindow(&self, window: @Window, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowWindow(self.handle(), window.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn showSizer(&self, sizer: @Sizer, show: bool, recursive: bool) -> bool {
        unsafe { wxSizer_ShowSizer(self.handle(), sizer.handle(), show, recursive) }
    }
    #[fixed_stack_segment]
    fn show(&self, sizer: @Sizer, index: c_int, show: bool) -> bool {
        unsafe { wxSizer_Show(self.handle(), sizer.handle(), index, show) }
    }
}

struct wxSizerItem(*u8);
impl SizerItem for wxSizerItem {}
impl Object for wxSizerItem { fn handle(&self) -> *u8 { **self } }

impl wxSizerItem {
    #[fixed_stack_segment]
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *u8) -> @SizerItem {
        unsafe { @wxSizerItem(wxSizerItem_Create(width, height, option, flag, border, userData)) as @SizerItem }
    }
    #[fixed_stack_segment]
    pub fn newInSizer(sizer: @Sizer, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInSizer(sizer.handle(), option, flag, border, userData) }
    }
    #[fixed_stack_segment]
    pub fn newInWindow(window: @Window, option: c_int, flag: c_int, border: c_int, userData: *u8) -> *u8 {
        unsafe { wxSizerItem_CreateInWindow(window.handle(), option, flag, border, userData) }
    }
}

trait SizerItem : Object {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @Size {
        unsafe { @wxSize(wxSizerItem_CalcMin(self.handle())) as @Size }
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
    fn getMinSize(&self) -> @Size {
        unsafe { @wxSize(wxSizerItem_GetMinSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxSizerItem_GetPosition(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxSizerItem_GetSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @Sizer {
        unsafe { @wxSizer(wxSizerItem_GetSizer(self.handle())) as @Sizer }
    }
    #[fixed_stack_segment]
    fn getUserData(&self) -> *u8 {
        unsafe { wxSizerItem_GetUserData(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @Window {
        unsafe { @wxWindow(wxSizerItem_GetWindow(self.handle())) as @Window }
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
    fn setSizer(&self, sizer: @Sizer) {
        unsafe { wxSizerItem_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, window: @Window) {
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
    fn getRect(&self) -> @Rect {
        unsafe { @wxRect(wxSizerItem_GetRect(self.handle())) as @Rect }
    }
    #[fixed_stack_segment]
    fn getSpacer(&self) -> @Size {
        unsafe { @wxSize(wxSizerItem_GetSpacer(self.handle())) as @Size }
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
impl Slider for wxSlider {}
impl Control for wxSlider {}
impl Window for wxSlider {}
impl EvtHandler for wxSlider {}
impl Object for wxSlider { fn handle(&self) -> *u8 { **self } }

impl wxSlider {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @Slider {
        unsafe { @wxSlider(wxSlider_Create(_prt.handle(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl)) as @Slider }
    }
}

trait Slider : Control {
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
impl SockAddress for wxSockAddress {}
impl Object for wxSockAddress { fn handle(&self) -> *u8 { **self } }

impl wxSockAddress {
}

trait SockAddress : Object {
}

struct wxSocketBase(*u8);
impl SocketBase for wxSocketBase {}
impl Object for wxSocketBase { fn handle(&self) -> *u8 { **self } }

impl wxSocketBase {
}

trait SocketBase : Object {
}

struct wxSocketClient(*u8);
impl SocketClient for wxSocketClient {}
impl SocketBase for wxSocketClient {}
impl Object for wxSocketClient { fn handle(&self) -> *u8 { **self } }

impl wxSocketClient {
}

trait SocketClient : SocketBase {
}

struct wxSocketEvent(*u8);
impl SocketEvent for wxSocketEvent {}
impl Event for wxSocketEvent {}
impl Object for wxSocketEvent { fn handle(&self) -> *u8 { **self } }

impl wxSocketEvent {
}

trait SocketEvent : Event {
}

struct wxSocketInputStream(*u8);
impl SocketInputStream for wxSocketInputStream {}
impl InputStream for wxSocketInputStream {}
impl StreamBase for wxSocketInputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketInputStream {
}

trait SocketInputStream : InputStream {
}

struct wxSocketOutputStream(*u8);
impl SocketOutputStream for wxSocketOutputStream {}
impl OutputStream for wxSocketOutputStream {}
impl StreamBase for wxSocketOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxSocketOutputStream {
}

trait SocketOutputStream : OutputStream {
}

struct wxSocketServer(*u8);
impl SocketServer for wxSocketServer {}
impl SocketBase for wxSocketServer {}
impl Object for wxSocketServer { fn handle(&self) -> *u8 { **self } }

impl wxSocketServer {
}

trait SocketServer : SocketBase {
}

struct wxSpinButton(*u8);
impl SpinButton for wxSpinButton {}
impl Control for wxSpinButton {}
impl Window for wxSpinButton {}
impl EvtHandler for wxSpinButton {}
impl Object for wxSpinButton { fn handle(&self) -> *u8 { **self } }

impl wxSpinButton {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @SpinButton {
        unsafe { @wxSpinButton(wxSpinButton_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @SpinButton }
    }
}

trait SpinButton : Control {
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
impl SpinCtrl for wxSpinCtrl {}
impl Control for wxSpinCtrl {}
impl Window for wxSpinCtrl {}
impl EvtHandler for wxSpinCtrl {}
impl Object for wxSpinCtrl { fn handle(&self) -> *u8 { **self } }

impl wxSpinCtrl {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> @SpinCtrl {
        unsafe { @wxSpinCtrl(wxSpinCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init)) as @SpinCtrl }
    }
}

trait SpinCtrl : Control {
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
impl SpinEvent for wxSpinEvent {}
impl NotifyEvent for wxSpinEvent {}
impl CommandEvent for wxSpinEvent {}
impl Event for wxSpinEvent {}
impl Object for wxSpinEvent { fn handle(&self) -> *u8 { **self } }

impl wxSpinEvent {
}

trait SpinEvent : NotifyEvent {
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
impl SplashScreen for wxSplashScreen {}
impl Frame for wxSplashScreen {}
impl TopLevelWindow for wxSplashScreen {}
impl Window for wxSplashScreen {}
impl EvtHandler for wxSplashScreen {}
impl Object for wxSplashScreen { fn handle(&self) -> *u8 { **self } }

impl wxSplashScreen {
}

trait SplashScreen : Frame {
}

struct wxSplitterEvent(*u8);
impl SplitterEvent for wxSplitterEvent {}
impl NotifyEvent for wxSplitterEvent {}
impl CommandEvent for wxSplitterEvent {}
impl Event for wxSplitterEvent {}
impl Object for wxSplitterEvent { fn handle(&self) -> *u8 { **self } }

impl wxSplitterEvent {
}

trait SplitterEvent : NotifyEvent {
}

struct wxSplitterScrolledWindow(*u8);
impl SplitterScrolledWindow for wxSplitterScrolledWindow {}
impl ScrolledWindow for wxSplitterScrolledWindow {}
impl Panel for wxSplitterScrolledWindow {}
impl Window for wxSplitterScrolledWindow {}
impl EvtHandler for wxSplitterScrolledWindow {}
impl Object for wxSplitterScrolledWindow { fn handle(&self) -> *u8 { **self } }

impl wxSplitterScrolledWindow {
}

trait SplitterScrolledWindow : ScrolledWindow {
}

struct wxSplitterWindow(*u8);
impl SplitterWindow for wxSplitterWindow {}
impl Window for wxSplitterWindow {}
impl EvtHandler for wxSplitterWindow {}
impl Object for wxSplitterWindow { fn handle(&self) -> *u8 { **self } }

impl wxSplitterWindow {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @SplitterWindow {
        unsafe { @wxSplitterWindow(wxSplitterWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @SplitterWindow }
    }
}

trait SplitterWindow : Window {
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
    fn getWindow1(&self) -> @Window {
        unsafe { @wxWindow(wxSplitterWindow_GetWindow1(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn getWindow2(&self) -> @Window {
        unsafe { @wxWindow(wxSplitterWindow_GetWindow2(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn initialize(&self, window: @Window) {
        unsafe { wxSplitterWindow_Initialize(self.handle(), window.handle()) }
    }
    #[fixed_stack_segment]
    fn isSplit(&self) -> bool {
        unsafe { wxSplitterWindow_IsSplit(self.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceWindow(&self, winOld: @Window, winNew: @Window) -> bool {
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
    fn splitHorizontally(&self, window1: @Window, window2: @Window, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitHorizontally(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn splitVertically(&self, window1: @Window, window2: @Window, sashPosition: c_int) -> bool {
        unsafe { wxSplitterWindow_SplitVertically(self.handle(), window1.handle(), window2.handle(), sashPosition) }
    }
    #[fixed_stack_segment]
    fn unsplit(&self, toRemove: @Window) -> bool {
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
impl StaticBitmap for wxStaticBitmap {}
impl Control for wxStaticBitmap {}
impl Window for wxStaticBitmap {}
impl EvtHandler for wxStaticBitmap {}
impl Object for wxStaticBitmap { fn handle(&self) -> *u8 { **self } }

impl wxStaticBitmap {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, bitmap: @Bitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @StaticBitmap {
        unsafe { @wxStaticBitmap(wxStaticBitmap_Create(_prt.handle(), _id, bitmap.handle(), _lft, _top, _wdt, _hgt, _stl)) as @StaticBitmap }
    }
}

trait StaticBitmap : Control {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxStaticBitmap_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBitmap(&self, _ref: @Bitmap) {
        unsafe { wxStaticBitmap_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getIcon(&self, _ref: @Icon) {
        unsafe { wxStaticBitmap_GetIcon(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn setBitmap(&self, bitmap: @Bitmap) {
        unsafe { wxStaticBitmap_SetBitmap(self.handle(), bitmap.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @Icon) {
        unsafe { wxStaticBitmap_SetIcon(self.handle(), icon.handle()) }
    }
}

struct wxStaticBox(*u8);
impl StaticBox for wxStaticBox {}
impl Control for wxStaticBox {}
impl Window for wxStaticBox {}
impl EvtHandler for wxStaticBox {}
impl Object for wxStaticBox { fn handle(&self) -> *u8 { **self } }

impl wxStaticBox {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @StaticBox {
        unsafe { @wxStaticBox(wxStaticBox_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @StaticBox }
    }
}

trait StaticBox : Control {
}

struct wxStaticBoxSizer(*u8);
impl StaticBoxSizer for wxStaticBoxSizer {}
impl BoxSizer for wxStaticBoxSizer {}
impl Sizer for wxStaticBoxSizer {}
impl Object for wxStaticBoxSizer { fn handle(&self) -> *u8 { **self } }

impl wxStaticBoxSizer {
    #[fixed_stack_segment]
    pub fn new(box: @StaticBox, orient: c_int) -> @StaticBoxSizer {
        unsafe { @wxStaticBoxSizer(wxStaticBoxSizer_Create(box.handle(), orient)) as @StaticBoxSizer }
    }
}

trait StaticBoxSizer : BoxSizer {
    #[fixed_stack_segment]
    fn calcMin(&self) -> @Size {
        unsafe { @wxSize(wxStaticBoxSizer_CalcMin(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self) -> @StaticBox {
        unsafe { @wxStaticBox(wxStaticBoxSizer_GetStaticBox(self.handle())) as @StaticBox }
    }
    #[fixed_stack_segment]
    fn recalcSizes(&self) {
        unsafe { wxStaticBoxSizer_RecalcSizes(self.handle()) }
    }
}

struct wxStaticLine(*u8);
impl StaticLine for wxStaticLine {}
impl Control for wxStaticLine {}
impl Window for wxStaticLine {}
impl EvtHandler for wxStaticLine {}
impl Object for wxStaticLine { fn handle(&self) -> *u8 { **self } }

impl wxStaticLine {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @StaticLine {
        unsafe { @wxStaticLine(wxStaticLine_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @StaticLine }
    }
}

trait StaticLine : Control {
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
impl StaticText for wxStaticText {}
impl Control for wxStaticText {}
impl Window for wxStaticText {}
impl EvtHandler for wxStaticText {}
impl Object for wxStaticText { fn handle(&self) -> *u8 { **self } }

impl wxStaticText {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @StaticText {
        unsafe { @wxStaticText(wxStaticText_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @StaticText }
    }
}

trait StaticText : Control {
}

struct wxStatusBar(*u8);
impl StatusBar for wxStatusBar {}
impl Window for wxStatusBar {}
impl EvtHandler for wxStatusBar {}
impl Object for wxStatusBar { fn handle(&self) -> *u8 { **self } }

impl wxStatusBar {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @StatusBar {
        unsafe { @wxStatusBar(wxStatusBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @StatusBar }
    }
}

trait StatusBar : Window {
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
    fn getStatusText(&self, number: c_int) -> @String {
        unsafe { @wxString(wxStatusBar_GetStatusText(self.handle(), number)) as @String }
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
    fn setStatusText(&self, text: @String, number: c_int) {
        unsafe { wxStatusBar_SetStatusText(self.handle(), text.handle(), number) }
    }
    #[fixed_stack_segment]
    fn setStatusWidths(&self, n: c_int, widths: *c_int) {
        unsafe { wxStatusBar_SetStatusWidths(self.handle(), n, widths) }
    }
}

struct wxStopWatch(*u8);
impl StopWatch for wxStopWatch { fn handle(&self) -> *u8 { **self } }

impl wxStopWatch {
    #[fixed_stack_segment]
    pub fn new() -> @StopWatch {
        unsafe { @wxStopWatch(wxStopWatch_Create()) as @StopWatch }
    }
}

trait StopWatch {
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
impl StreamBase for wxStreamBase { fn handle(&self) -> *u8 { **self } }

impl wxStreamBase {
}

trait StreamBase {
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
impl StreamBuffer for wxStreamBuffer { fn handle(&self) -> *u8 { **self } }

impl wxStreamBuffer {
}

trait StreamBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStreamToTextRedirector(*u8);
impl StreamToTextRedirector for wxStreamToTextRedirector { fn handle(&self) -> *u8 { **self } }

impl wxStreamToTextRedirector {
}

trait StreamToTextRedirector {
    fn handle(&self) -> *u8;
    
}

struct wxString(*u8);
impl String for wxString { fn handle(&self) -> *u8 { **self } }

impl wxString {
    #[fixed_stack_segment]
    pub fn new(buffer: *wchar_t) -> @String {
        unsafe { @wxString(wxString_Create(buffer)) as @String }
    }
    #[fixed_stack_segment]
    pub fn newLen(buffer: *wchar_t, len: c_int) -> @String {
        unsafe { @wxString(wxString_CreateLen(buffer, len)) as @String }
    }
}

trait String {
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
impl StringBuffer for wxStringBuffer { fn handle(&self) -> *u8 { **self } }

impl wxStringBuffer {
}

trait StringBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStringClientData(*u8);
impl StringClientData for wxStringClientData {}
impl ClientData for wxStringClientData { fn handle(&self) -> *u8 { **self } }

impl wxStringClientData {
}

trait StringClientData : ClientData {
}

struct wxStringList(*u8);
impl StringList for wxStringList {}
impl List for wxStringList {}
impl Object for wxStringList { fn handle(&self) -> *u8 { **self } }

impl wxStringList {
}

trait StringList : List {
}

struct wxStringTokenizer(*u8);
impl StringTokenizer for wxStringTokenizer {}
impl Object for wxStringTokenizer { fn handle(&self) -> *u8 { **self } }

impl wxStringTokenizer {
}

trait StringTokenizer : Object {
}

struct wxSysColourChangedEvent(*u8);
impl SysColourChangedEvent for wxSysColourChangedEvent {}
impl Event for wxSysColourChangedEvent {}
impl Object for wxSysColourChangedEvent { fn handle(&self) -> *u8 { **self } }

impl wxSysColourChangedEvent {
}

trait SysColourChangedEvent : Event {
}

struct wxSystemOptions(*u8);
impl SystemOptions for wxSystemOptions {}
impl Object for wxSystemOptions { fn handle(&self) -> *u8 { **self } }

impl wxSystemOptions {
}

trait SystemOptions : Object {
}

struct wxSystemSettings(*u8);
impl SystemSettings for wxSystemSettings {}
impl Object for wxSystemSettings { fn handle(&self) -> *u8 { **self } }

impl wxSystemSettings {
    #[fixed_stack_segment]
    pub fn getColour(index: c_int, _ref: @Colour) {
        unsafe { wxSystemSettings_GetColour(index, _ref.handle()) }
    }
    #[fixed_stack_segment]
    pub fn getFont(index: c_int, _ref: @Font) {
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

trait SystemSettings : Object {
}

struct wxTabCtrl(*u8);
impl TabCtrl for wxTabCtrl {}
impl Control for wxTabCtrl {}
impl Window for wxTabCtrl {}
impl EvtHandler for wxTabCtrl {}
impl Object for wxTabCtrl { fn handle(&self) -> *u8 { **self } }

impl wxTabCtrl {
}

trait TabCtrl : Control {
}

struct wxTabEvent(*u8);
impl TabEvent for wxTabEvent {}
impl CommandEvent for wxTabEvent {}
impl Event for wxTabEvent {}
impl Object for wxTabEvent { fn handle(&self) -> *u8 { **self } }

impl wxTabEvent {
}

trait TabEvent : CommandEvent {
}

struct wxTablesInUse(*u8);
impl TablesInUse for wxTablesInUse {}
impl Object for wxTablesInUse { fn handle(&self) -> *u8 { **self } }

impl wxTablesInUse {
}

trait TablesInUse : Object {
}

struct wxTaskBarIcon(*u8);
impl TaskBarIcon for wxTaskBarIcon {}
impl EvtHandler for wxTaskBarIcon {}
impl Object for wxTaskBarIcon { fn handle(&self) -> *u8 { **self } }

impl wxTaskBarIcon {
    #[fixed_stack_segment]
    pub fn new() -> @TaskBarIcon {
        unsafe { @wxTaskBarIcon(wxTaskBarIcon_Create()) as @TaskBarIcon }
    }
}

trait TaskBarIcon : EvtHandler {
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
    fn popupMenu(&self, menu: @Menu) -> bool {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    #[fixed_stack_segment]
    fn removeIcon(&self) -> bool {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, icon: @Icon, text: @String) -> bool {
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

struct wxTempFile(*u8);
impl TempFile for wxTempFile { fn handle(&self) -> *u8 { **self } }

impl wxTempFile {
}

trait TempFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextAttr(*u8);
impl TextAttr for wxTextAttr { fn handle(&self) -> *u8 { **self } }

impl wxTextAttr {
    #[fixed_stack_segment]
    pub fn new(colText: @Colour, colBack: @Colour, font: @Font) -> @TextAttr {
        unsafe { @wxTextAttr(wxTextAttr_Create(colText.handle(), colBack.handle(), font.handle())) as @TextAttr }
    }
    #[fixed_stack_segment]
    pub fn newDefault() -> @TextAttr {
        unsafe { @wxTextAttr(wxTextAttr_CreateDefault()) as @TextAttr }
    }
}

trait TextAttr {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, colour: @Colour) {
        unsafe { wxTextAttr_GetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn getFont(&self, font: @Font) {
        unsafe { wxTextAttr_GetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextColour(&self, colour: @Colour) {
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
    fn setTextColour(&self, colour: @Colour) {
        unsafe { wxTextAttr_SetTextColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @Colour) {
        unsafe { wxTextAttr_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @Font) {
        unsafe { wxTextAttr_SetFont(self.handle(), font.handle()) }
    }
}

struct wxTextCtrl(*u8);
impl TextCtrl for wxTextCtrl {}
impl Control for wxTextCtrl {}
impl Window for wxTextCtrl {}
impl EvtHandler for wxTextCtrl {}
impl Object for wxTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxTextCtrl {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @TextCtrl {
        unsafe { @wxTextCtrl(wxTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, _stl)) as @TextCtrl }
    }
}

trait TextCtrl : Control {
    #[fixed_stack_segment]
    fn appendText(&self, text: @String) {
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
    fn changeValue(&self, text: @String) {
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
    fn getLineText(&self, lineNo: c_long) -> @String {
        unsafe { @wxString(wxTextCtrl_GetLineText(self.handle(), lineNo)) as @String }
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
    fn getValue(&self) -> @String {
        unsafe { @wxString(wxTextCtrl_GetValue(self.handle())) as @String }
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
    fn loadFile(&self, file: @String) -> bool {
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
    fn replace(&self, from: c_long, to: c_long, value: @String) {
        unsafe { wxTextCtrl_Replace(self.handle(), from, to, value.handle()) }
    }
    #[fixed_stack_segment]
    fn saveFile(&self, file: @String) -> bool {
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
    fn setValue(&self, value: @String) {
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
    fn writeText(&self, text: @String) {
        unsafe { wxTextCtrl_WriteText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long {
        unsafe { wxTextCtrl_XYToPosition(self.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn emulateKeyPress(&self, keyevent: @KeyEvent) -> bool {
        unsafe { wxTextCtrl_EmulateKeyPress(self.handle(), keyevent.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultStyle(&self) -> @TextAttr {
        unsafe { @wxTextAttr(wxTextCtrl_GetDefaultStyle(self.handle())) as @TextAttr }
    }
    #[fixed_stack_segment]
    fn getRange(&self, from: c_long, to: c_long) -> @String {
        unsafe { @wxString(wxTextCtrl_GetRange(self.handle(), from, to)) as @String }
    }
    #[fixed_stack_segment]
    fn getStringSelection(&self) -> @String {
        unsafe { @wxString(wxTextCtrl_GetStringSelection(self.handle())) as @String }
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
    fn setDefaultStyle(&self, style: @TextAttr) -> bool {
        unsafe { wxTextCtrl_SetDefaultStyle(self.handle(), style.handle()) }
    }
    #[fixed_stack_segment]
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.handle(), len) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, start: c_long, end: c_long, style: @TextAttr) -> bool {
        unsafe { wxTextCtrl_SetStyle(self.handle(), start, end, style.handle()) }
    }
}

struct wxTextDataObject(*u8);
impl TextDataObject for wxTextDataObject {}
impl DataObjectSimple for wxTextDataObject {}
impl DataObject for wxTextDataObject { fn handle(&self) -> *u8 { **self } }

impl wxTextDataObject {
}

trait TextDataObject : DataObjectSimple {
}

struct wxTextDropTarget(*u8);
impl TextDropTarget for wxTextDropTarget {}
impl DropTarget for wxTextDropTarget { fn handle(&self) -> *u8 { **self } }

impl wxTextDropTarget {
}

trait TextDropTarget : DropTarget {
}

struct wxTextEntryDialog(*u8);
impl TextEntryDialog for wxTextEntryDialog {}
impl Dialog for wxTextEntryDialog {}
impl TopLevelWindow for wxTextEntryDialog {}
impl Window for wxTextEntryDialog {}
impl EvtHandler for wxTextEntryDialog {}
impl Object for wxTextEntryDialog { fn handle(&self) -> *u8 { **self } }

impl wxTextEntryDialog {
}

trait TextEntryDialog : Dialog {
}

struct wxTextFile(*u8);
impl TextFile for wxTextFile { fn handle(&self) -> *u8 { **self } }

impl wxTextFile {
}

trait TextFile {
    fn handle(&self) -> *u8;
    
}

struct wxTextInputStream(*u8);
impl TextInputStream for wxTextInputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextInputStream {
    #[fixed_stack_segment]
    pub fn new(inputStream: @InputStream, sep: @String) -> @TextInputStream {
        unsafe { @wxTextInputStream(wxTextInputStream_Create(inputStream.handle(), sep.handle())) as @TextInputStream }
    }
}

trait TextInputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextInputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn readLine(&self) -> @String {
        unsafe { @wxString(wxTextInputStream_ReadLine(self.handle())) as @String }
    }
}

struct wxTextOutputStream(*u8);
impl TextOutputStream for wxTextOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxTextOutputStream {
    #[fixed_stack_segment]
    pub fn new(outputStream: @OutputStream, mode: c_int) -> @TextOutputStream {
        unsafe { @wxTextOutputStream(wxTextOutputStream_Create(outputStream.handle(), mode)) as @TextOutputStream }
    }
}

trait TextOutputStream {
    fn handle(&self) -> *u8;
    
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxTextOutputStream_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn writeString(&self, txt: @String) {
        unsafe { wxTextOutputStream_WriteString(self.handle(), txt.handle()) }
    }
}

struct wxTextValidator(*u8);
impl TextValidator for wxTextValidator {}
impl Validator for wxTextValidator {}
impl EvtHandler for wxTextValidator {}
impl Object for wxTextValidator { fn handle(&self) -> *u8 { **self } }

impl wxTextValidator {
    #[fixed_stack_segment]
    pub fn new(style: c_int, val: *u8) -> @TextValidator {
        unsafe { @wxTextValidator(wxTextValidator_Create(style, val)) as @TextValidator }
    }
}

trait TextValidator : Validator {
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
    fn clone(&self) -> @Validator {
        unsafe { @wxValidator(wxTextValidator_Clone(self.handle())) as @Validator }
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
    fn onChar(&self, event: @Event) {
        unsafe { wxTextValidator_OnChar(self.handle(), event.handle()) }
    }
    #[fixed_stack_segment]
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.handle(), style) }
    }
}

struct wxThinSplitterWindow(*u8);
impl ThinSplitterWindow for wxThinSplitterWindow {}
impl SplitterWindow for wxThinSplitterWindow {}
impl Window for wxThinSplitterWindow {}
impl EvtHandler for wxThinSplitterWindow {}
impl Object for wxThinSplitterWindow { fn handle(&self) -> *u8 { **self } }

impl wxThinSplitterWindow {
}

trait ThinSplitterWindow : SplitterWindow {
}

struct wxThread(*u8);
impl Thread for wxThread { fn handle(&self) -> *u8 { **self } }

impl wxThread {
}

trait Thread {
    fn handle(&self) -> *u8;
    
}

struct wxTime(*u8);
impl Time for wxTime {}
impl Object for wxTime { fn handle(&self) -> *u8 { **self } }

impl wxTime {
}

trait Time : Object {
}

struct wxTimeSpan(*u8);
impl TimeSpan for wxTimeSpan { fn handle(&self) -> *u8 { **self } }

impl wxTimeSpan {
}

trait TimeSpan {
    fn handle(&self) -> *u8;
    
}

struct wxTimer(*u8);
impl Timer for wxTimer {}
impl Object for wxTimer { fn handle(&self) -> *u8 { **self } }

impl wxTimer {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int) -> @Timer {
        unsafe { @wxTimer(wxTimer_Create(_prt.handle(), _id)) as @Timer }
    }
}

trait Timer : Object {
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
impl TimerBase for wxTimerBase {}
impl Object for wxTimerBase { fn handle(&self) -> *u8 { **self } }

impl wxTimerBase {
}

trait TimerBase : Object {
}

struct wxTimerEvent(*u8);
impl TimerEvent for wxTimerEvent {}
impl Event for wxTimerEvent {}
impl Object for wxTimerEvent { fn handle(&self) -> *u8 { **self } }

impl wxTimerEvent {
}

trait TimerEvent : Event {
    #[fixed_stack_segment]
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.handle()) }
    }
}

struct wxTimerEx(*u8);
impl TimerEx for wxTimerEx {}
impl Timer for wxTimerEx {}
impl Object for wxTimerEx { fn handle(&self) -> *u8 { **self } }

impl wxTimerEx {
    #[fixed_stack_segment]
    pub fn new() -> @TimerEx {
        unsafe { @wxTimerEx(wxTimerEx_Create()) as @TimerEx }
    }
}

trait TimerEx : Timer {
    #[fixed_stack_segment]
    fn connect(&self, closure: @Closure) {
        unsafe { wxTimerEx_Connect(self.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn getClosure(&self) -> @Closure {
        unsafe { @wxClosure(wxTimerEx_GetClosure(self.handle())) as @Closure }
    }
}

struct wxTimerRunner(*u8);
impl TimerRunner for wxTimerRunner { fn handle(&self) -> *u8 { **self } }

impl wxTimerRunner {
}

trait TimerRunner {
    fn handle(&self) -> *u8;
    
}

struct wxTipProvider(*u8);
impl TipProvider for wxTipProvider { fn handle(&self) -> *u8 { **self } }

impl wxTipProvider {
}

trait TipProvider {
    fn handle(&self) -> *u8;
    
}

struct wxTipWindow(*u8);
impl TipWindow for wxTipWindow {}
impl PopupTransientWindow for wxTipWindow {}
impl PopupWindow for wxTipWindow {}
impl Window for wxTipWindow {}
impl EvtHandler for wxTipWindow {}
impl Object for wxTipWindow { fn handle(&self) -> *u8 { **self } }

impl wxTipWindow {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, text: @String, maxLength: c_int) -> @TipWindow {
        unsafe { @wxTipWindow(wxTipWindow_Create(parent.handle(), text.handle(), maxLength)) as @TipWindow }
    }
}

trait TipWindow : PopupTransientWindow {
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
impl ToggleButton for wxToggleButton {}
impl Control for wxToggleButton {}
impl Window for wxToggleButton {}
impl EvtHandler for wxToggleButton {}
impl Object for wxToggleButton { fn handle(&self) -> *u8 { **self } }

impl wxToggleButton {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, id: c_int, label: @String, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> @ToggleButton {
        unsafe { @wxToggleButton(wxToggleButton_Create(parent.handle(), id, label.handle(), x, y, w, h, style)) as @ToggleButton }
    }
}

trait ToggleButton : Control {
    #[fixed_stack_segment]
    fn enable(&self, enable: bool) -> bool {
        unsafe { wxToggleButton_Enable(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> bool {
        unsafe { wxToggleButton_GetValue(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, label: @String) {
        unsafe { wxToggleButton_SetLabel(self.handle(), label.handle()) }
    }
    #[fixed_stack_segment]
    fn setValue(&self, state: bool) {
        unsafe { wxToggleButton_SetValue(self.handle(), state) }
    }
}

struct wxToolBar(*u8);
impl ToolBar for wxToolBar {}
impl ToolBarBase for wxToolBar {}
impl Control for wxToolBar {}
impl Window for wxToolBar {}
impl EvtHandler for wxToolBar {}
impl Object for wxToolBar { fn handle(&self) -> *u8 { **self } }

impl wxToolBar {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @ToolBar {
        unsafe { @wxToolBar(wxToolBar_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @ToolBar }
    }
}

trait ToolBar : ToolBarBase {
    #[fixed_stack_segment]
    fn addControl(&self, ctrl: @Control) -> bool {
        unsafe { wxToolBar_AddControl(self.handle(), ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn addTool(&self, id: c_int, bmp: @Bitmap, shelp: @String, lhelp: @String) {
        unsafe { wxToolBar_AddTool(self.handle(), id, bmp.handle(), shelp.handle(), lhelp.handle()) }
    }
    #[fixed_stack_segment]
    fn addToolEx(&self, id: c_int, bmp1: @Bitmap, bmp2: @Bitmap, isToggle: bool, x: c_int, y: c_int, data: @Object, shelp: @String, lhelp: @String) {
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
    fn getMargins(&self) -> @Point {
        unsafe { @wxPoint(wxToolBar_GetMargins(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getToolBitmapSize(&self) -> @Size {
        unsafe { @wxSize(wxToolBar_GetToolBitmapSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getToolClientData(&self, id: c_int) -> @Object {
        unsafe { @wxObject(wxToolBar_GetToolClientData(self.handle(), id)) as @Object }
    }
    #[fixed_stack_segment]
    fn getToolEnabled(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolEnabled(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn getToolLongHelp(&self, id: c_int) -> @String {
        unsafe { @wxString(wxToolBar_GetToolLongHelp(self.handle(), id)) as @String }
    }
    #[fixed_stack_segment]
    fn getToolPacking(&self) -> c_int {
        unsafe { wxToolBar_GetToolPacking(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolShortHelp(&self, id: c_int) -> @String {
        unsafe { @wxString(wxToolBar_GetToolShortHelp(self.handle(), id)) as @String }
    }
    #[fixed_stack_segment]
    fn getToolSize(&self) -> @Size {
        unsafe { @wxSize(wxToolBar_GetToolSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getToolState(&self, id: c_int) -> bool {
        unsafe { wxToolBar_GetToolState(self.handle(), id) }
    }
    #[fixed_stack_segment]
    fn insertControl(&self, pos: c_int, ctrl: @Control) {
        unsafe { wxToolBar_InsertControl(self.handle(), pos, ctrl.handle()) }
    }
    #[fixed_stack_segment]
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.handle(), pos) }
    }
    #[fixed_stack_segment]
    fn insertTool(&self, pos: c_int, id: c_int, bmp1: @Bitmap, bmp2: @Bitmap, isToggle: bool, data: @Object, shelp: @String, lhelp: @String) {
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
    fn setToolClientData(&self, id: c_int, data: @Object) {
        unsafe { wxToolBar_SetToolClientData(self.handle(), id, data.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolLongHelp(&self, id: c_int, str: @String) {
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
    fn setToolShortHelp(&self, id: c_int, str: @String) {
        unsafe { wxToolBar_SetToolShortHelp(self.handle(), id, str.handle()) }
    }
    #[fixed_stack_segment]
    fn toggleTool(&self, id: c_int, toggle: bool) {
        unsafe { wxToolBar_ToggleTool(self.handle(), id, toggle) }
    }
    #[fixed_stack_segment]
    fn addTool2(&self, toolId: c_int, label: @String, bmp: @Bitmap, bmpDisabled: @Bitmap, itemKind: c_int, shortHelp: @String, longHelp: @String) {
        unsafe { wxToolBar_AddTool2(self.handle(), toolId, label.handle(), bmp.handle(), bmpDisabled.handle(), itemKind, shortHelp.handle(), longHelp.handle()) }
    }
}

struct wxToolBarBase(*u8);
impl ToolBarBase for wxToolBarBase {}
impl Control for wxToolBarBase {}
impl Window for wxToolBarBase {}
impl EvtHandler for wxToolBarBase {}
impl Object for wxToolBarBase { fn handle(&self) -> *u8 { **self } }

impl wxToolBarBase {
}

trait ToolBarBase : Control {
}

struct wxToolLayoutItem(*u8);
impl ToolLayoutItem for wxToolLayoutItem {}
impl Object for wxToolLayoutItem { fn handle(&self) -> *u8 { **self } }

impl wxToolLayoutItem {
}

trait ToolLayoutItem : Object {
    #[fixed_stack_segment]
    fn isSeparator(&self) -> bool {
        unsafe { wxToolLayoutItem_IsSeparator(self.handle()) }
    }
    #[fixed_stack_segment]
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int) {
        unsafe { wxToolLayoutItem_Rect(self.handle(), _x, _y, _w, _h) }
    }
}

struct wxToolTip(*u8);
impl ToolTip for wxToolTip {}
impl Object for wxToolTip { fn handle(&self) -> *u8 { **self } }

impl wxToolTip {
}

trait ToolTip : Object {
}

struct wxToolWindow(*u8);
impl ToolWindow for wxToolWindow {}
impl Frame for wxToolWindow {}
impl TopLevelWindow for wxToolWindow {}
impl Window for wxToolWindow {}
impl EvtHandler for wxToolWindow {}
impl Object for wxToolWindow { fn handle(&self) -> *u8 { **self } }

impl wxToolWindow {
}

trait ToolWindow : Frame {
}

struct wxTopLevelWindow(*u8);
impl TopLevelWindow for wxTopLevelWindow {}
impl Window for wxTopLevelWindow {}
impl EvtHandler for wxTopLevelWindow {}
impl Object for wxTopLevelWindow { fn handle(&self) -> *u8 { **self } }

impl wxTopLevelWindow {
}

trait TopLevelWindow : Window {
    #[fixed_stack_segment]
    fn enableCloseButton(&self, enable: bool) -> bool {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.handle(), enable) }
    }
    #[fixed_stack_segment]
    fn getDefaultButton(&self) -> @Button {
        unsafe { @wxButton(wxTopLevelWindow_GetDefaultButton(self.handle())) as @Button }
    }
    #[fixed_stack_segment]
    fn getDefaultItem(&self) -> @Window {
        unsafe { @wxWindow(wxTopLevelWindow_GetDefaultItem(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn getIcon(&self) -> @Icon {
        unsafe { @wxIcon(wxTopLevelWindow_GetIcon(self.handle())) as @Icon }
    }
    #[fixed_stack_segment]
    fn getTitle(&self) -> @String {
        unsafe { @wxString(wxTopLevelWindow_GetTitle(self.handle())) as @String }
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
    fn setDefaultButton(&self, pBut: @Button) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setDefaultItem(&self, pBut: @Window) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.handle(), pBut.handle()) }
    }
    #[fixed_stack_segment]
    fn setIcon(&self, pIcon: @Icon) {
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
    fn setTitle(&self, pString: @String) {
        unsafe { wxTopLevelWindow_SetTitle(self.handle(), pString.handle()) }
    }
}

struct wxTreeCompanionWindow(*u8);
impl TreeCompanionWindow for wxTreeCompanionWindow {}
impl Window for wxTreeCompanionWindow {}
impl EvtHandler for wxTreeCompanionWindow {}
impl Object for wxTreeCompanionWindow { fn handle(&self) -> *u8 { **self } }

impl wxTreeCompanionWindow {
}

trait TreeCompanionWindow : Window {
}

struct wxTreeCtrl(*u8);
impl TreeCtrl for wxTreeCtrl {}
impl Control for wxTreeCtrl {}
impl Window for wxTreeCtrl {}
impl EvtHandler for wxTreeCtrl {}
impl Object for wxTreeCtrl { fn handle(&self) -> *u8 { **self } }

impl wxTreeCtrl {
    #[fixed_stack_segment]
    pub fn new(_obj: *u8, _cmp: *u8, _prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @TreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create(_obj, _cmp, _prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @TreeCtrl }
    }
    #[fixed_stack_segment]
    pub fn new2(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @TreeCtrl {
        unsafe { @wxTreeCtrl(wxTreeCtrl_Create2(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @TreeCtrl }
    }
}

trait TreeCtrl : Control {
    #[fixed_stack_segment]
    fn addRoot(&self, text: @String, image: c_int, selectedImage: c_int, data: @TreeItemData, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_AddRoot(self.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn appendItem(&self, parent: @TreeItemId, text: @String, image: c_int, selectedImage: c_int, data: @TreeItemData, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_AppendItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapse(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_Collapse(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn collapseAndReset(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn delete(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_Delete(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.handle()) }
    }
    #[fixed_stack_segment]
    fn deleteChildren(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_DeleteChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn editLabel(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_EditLabel(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn endEditLabel(&self, item: @TreeItemId, discardChanges: bool) {
        unsafe { wxTreeCtrl_EndEditLabel(self.handle(), item.handle(), discardChanges) }
    }
    #[fixed_stack_segment]
    fn ensureVisible(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_EnsureVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn expand(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_Expand(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getBoundingRect(&self, item: @TreeItemId, textOnly: bool) -> @Rect {
        unsafe { @wxRect(wxTreeCtrl_GetBoundingRect(self.handle(), item.handle(), textOnly)) as @Rect }
    }
    #[fixed_stack_segment]
    fn getChildrenCount(&self, item: @TreeItemId, recursively: bool) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.handle(), item.handle(), recursively) }
    }
    #[fixed_stack_segment]
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getEditControl(&self) -> @TextCtrl {
        unsafe { @wxTextCtrl(wxTreeCtrl_GetEditControl(self.handle())) as @TextCtrl }
    }
    #[fixed_stack_segment]
    fn getFirstChild(&self, item: @TreeItemId, cookie: *c_int, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetFirstChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getFirstVisibleItem(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getImageList(&self) -> @ImageList {
        unsafe { @wxImageList(wxTreeCtrl_GetImageList(self.handle())) as @ImageList }
    }
    #[fixed_stack_segment]
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemData(&self, item: @TreeItemId) -> *u8 {
        unsafe { wxTreeCtrl_GetItemData(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemImage(&self, item: @TreeItemId, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.handle(), item.handle(), which) }
    }
    #[fixed_stack_segment]
    fn getItemText(&self, item: @TreeItemId) -> @String {
        unsafe { @wxString(wxTreeCtrl_GetItemText(self.handle(), item.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getLastChild(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetLastChild(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextChild(&self, item: @TreeItemId, cookie: *c_int, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetNextChild(self.handle(), item.handle(), cookie, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextSibling(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetNextSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getNextVisible(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetNextVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getParent(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetParent(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevSibling(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getPrevVisible(&self, item: @TreeItemId, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.handle(), item.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getRootItem(&self, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_GetRootItem(self.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getSelection(&self, _item: @TreeItemId) {
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
    fn getStateImageList(&self) -> @ImageList {
        unsafe { @wxImageList(wxTreeCtrl_GetStateImageList(self.handle())) as @ImageList }
    }
    #[fixed_stack_segment]
    fn hitTest(&self, _x: c_int, _y: c_int, flags: *c_int, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_HitTest(self.handle(), _x, _y, flags, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItem(&self, parent: @TreeItemId, idPrevious: @TreeItemId, text: @String, image: c_int, selectedImage: c_int, data: *u8, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_InsertItem(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex(&self, parent: @TreeItemId, index: c_int, text: @String, image: c_int, selectedImage: c_int, data: *u8, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn isBold(&self, item: @TreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsBold(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isExpanded(&self, item: @TreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsExpanded(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isSelected(&self, item: @TreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsSelected(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn isVisible(&self, item: @TreeItemId) -> bool {
        unsafe { wxTreeCtrl_IsVisible(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn itemHasChildren(&self, item: @TreeItemId) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn onCompareItems(&self, item1: @TreeItemId, item2: @TreeItemId) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.handle(), item1.handle(), item2.handle()) }
    }
    #[fixed_stack_segment]
    fn prependItem(&self, parent: @TreeItemId, text: @String, image: c_int, selectedImage: c_int, data: *u8, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_PrependItem(self.handle(), parent.handle(), text.handle(), image, selectedImage, data, _item.handle()) }
    }
    #[fixed_stack_segment]
    fn scrollTo(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_ScrollTo(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn selectItem(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_SelectItem(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn setImageList(&self, imageList: @ImageList) {
        unsafe { wxTreeCtrl_SetImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.handle(), indent) }
    }
    #[fixed_stack_segment]
    fn setItemBackgroundColour(&self, item: @TreeItemId, col: @Colour) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemBold(&self, item: @TreeItemId, bold: bool) {
        unsafe { wxTreeCtrl_SetItemBold(self.handle(), item.handle(), bold) }
    }
    #[fixed_stack_segment]
    fn setItemData(&self, item: @TreeItemId, data: *u8) {
        unsafe { wxTreeCtrl_SetItemData(self.handle(), item.handle(), data) }
    }
    #[fixed_stack_segment]
    fn setItemDropHighlight(&self, item: @TreeItemId, highlight: bool) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.handle(), item.handle(), highlight) }
    }
    #[fixed_stack_segment]
    fn setItemFont(&self, item: @TreeItemId, font: @Font) {
        unsafe { wxTreeCtrl_SetItemFont(self.handle(), item.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemHasChildren(&self, item: @TreeItemId, hasChildren: bool) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.handle(), item.handle(), hasChildren) }
    }
    #[fixed_stack_segment]
    fn setItemImage(&self, item: @TreeItemId, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.handle(), item.handle(), image, which) }
    }
    #[fixed_stack_segment]
    fn setItemText(&self, item: @TreeItemId, text: @String) {
        unsafe { wxTreeCtrl_SetItemText(self.handle(), item.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn setItemTextColour(&self, item: @TreeItemId, col: @Colour) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.handle(), item.handle(), col.handle()) }
    }
    #[fixed_stack_segment]
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.handle(), spacing) }
    }
    #[fixed_stack_segment]
    fn setStateImageList(&self, imageList: @ImageList) {
        unsafe { wxTreeCtrl_SetStateImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn sortChildren(&self, item: @TreeItemId) {
        unsafe { wxTreeCtrl_SortChildren(self.handle(), item.handle()) }
    }
    #[fixed_stack_segment]
    fn toggle(&self, item: @TreeItemId) {
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
    fn insertItem2(&self, parent: @Window, idPrevious: @TreeItemId, text: @String, image: c_int, selectedImage: c_int, closure: @Closure, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_InsertItem2(self.handle(), parent.handle(), idPrevious.handle(), text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn insertItemByIndex2(&self, parent: @Window, index: c_int, text: @String, image: c_int, selectedImage: c_int, closure: @Closure, _item: @TreeItemId) {
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.handle(), parent.handle(), index, text.handle(), image, selectedImage, closure.handle(), _item.handle()) }
    }
    #[fixed_stack_segment]
    fn getItemClientClosure(&self, item: @TreeItemId) -> @Closure {
        unsafe { @wxClosure(wxTreeCtrl_GetItemClientClosure(self.handle(), item.handle())) as @Closure }
    }
    #[fixed_stack_segment]
    fn setItemClientClosure(&self, item: @TreeItemId, closure: @Closure) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.handle(), item.handle(), closure.handle()) }
    }
    #[fixed_stack_segment]
    fn assignImageList(&self, imageList: @ImageList) {
        unsafe { wxTreeCtrl_AssignImageList(self.handle(), imageList.handle()) }
    }
    #[fixed_stack_segment]
    fn assignStateImageList(&self, imageList: @ImageList) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.handle(), imageList.handle()) }
    }
}

struct wxTreeEvent(*u8);
impl TreeEvent for wxTreeEvent {}
impl NotifyEvent for wxTreeEvent {}
impl CommandEvent for wxTreeEvent {}
impl Event for wxTreeEvent {}
impl Object for wxTreeEvent { fn handle(&self) -> *u8 { **self } }

impl wxTreeEvent {
}

trait TreeEvent : NotifyEvent {
    #[fixed_stack_segment]
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getItem(&self, _ref: @TreeItemId) {
        unsafe { wxTreeEvent_GetItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getLabel(&self) -> @String {
        unsafe { @wxString(wxTreeEvent_GetLabel(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getOldItem(&self, _ref: @TreeItemId) {
        unsafe { wxTreeEvent_GetOldItem(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getPoint(&self) -> @Point {
        unsafe { @wxPoint(wxTreeEvent_GetPoint(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getKeyEvent(&self) -> @KeyEvent {
        unsafe { @wxKeyEvent(wxTreeEvent_GetKeyEvent(self.handle())) as @KeyEvent }
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
impl TreeItemData for wxTreeItemData {}
impl ClientData for wxTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemData {
}

trait TreeItemData : ClientData {
}

struct wxTreeItemId(*u8);
impl TreeItemId for wxTreeItemId { fn handle(&self) -> *u8 { **self } }

impl wxTreeItemId {
    #[fixed_stack_segment]
    pub fn new() -> @TreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_Create()) as @TreeItemId }
    }
    #[fixed_stack_segment]
    pub fn newFromValue(value: intptr_t) -> @TreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_CreateFromValue(value)) as @TreeItemId }
    }
}

trait TreeItemId {
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
    fn clone(&self) -> @TreeItemId {
        unsafe { @wxTreeItemId(wxTreeItemId_Clone(self.handle())) as @TreeItemId }
    }
    #[fixed_stack_segment]
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.handle()) }
    }
}

struct wxTreeLayout(*u8);
impl TreeLayout for wxTreeLayout {}
impl Object for wxTreeLayout { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayout {
}

trait TreeLayout : Object {
}

struct wxTreeLayoutStored(*u8);
impl TreeLayoutStored for wxTreeLayoutStored {}
impl TreeLayout for wxTreeLayoutStored {}
impl Object for wxTreeLayoutStored { fn handle(&self) -> *u8 { **self } }

impl wxTreeLayoutStored {
}

trait TreeLayoutStored : TreeLayout {
}

struct wxURL(*u8);
impl URL for wxURL {}
impl Object for wxURL { fn handle(&self) -> *u8 { **self } }

impl wxURL {
}

trait URL : Object {
}

struct wxUpdateUIEvent(*u8);
impl UpdateUIEvent for wxUpdateUIEvent {}
impl Event for wxUpdateUIEvent {}
impl Object for wxUpdateUIEvent { fn handle(&self) -> *u8 { **self } }

impl wxUpdateUIEvent {
}

trait UpdateUIEvent : Event {
    #[fixed_stack_segment]
    fn check(&self, check: bool) {
        unsafe { wxUpdateUIEvent_Check(self.handle(), check) }
    }
    #[fixed_stack_segment]
    fn copyObject(&self, obj: @Object) {
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
    fn getText(&self) -> @String {
        unsafe { @wxString(wxUpdateUIEvent_GetText(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setText(&self, text: @String) {
        unsafe { wxUpdateUIEvent_SetText(self.handle(), text.handle()) }
    }
}

struct wxValidator(*u8);
impl Validator for wxValidator {}
impl EvtHandler for wxValidator {}
impl Object for wxValidator { fn handle(&self) -> *u8 { **self } }

impl wxValidator {
    #[fixed_stack_segment]
    pub fn new() -> @Validator {
        unsafe { @wxValidator(wxValidator_Create()) as @Validator }
    }
    #[fixed_stack_segment]
    pub fn setBellOnError(doIt: bool) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

trait Validator : EvtHandler {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxValidator_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getWindow(&self) -> @Window {
        unsafe { @wxWindow(wxValidator_GetWindow(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn setWindow(&self, win: @Window) {
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
    fn validate(&self, parent: @Window) -> bool {
        unsafe { wxValidator_Validate(self.handle(), parent.handle()) }
    }
}

struct wxVariant(*u8);
impl Variant for wxVariant {}
impl Object for wxVariant { fn handle(&self) -> *u8 { **self } }

impl wxVariant {
}

trait Variant : Object {
}

struct wxVariantData(*u8);
impl VariantData for wxVariantData {}
impl Object for wxVariantData { fn handle(&self) -> *u8 { **self } }

impl wxVariantData {
}

trait VariantData : Object {
}

struct wxView(*u8);
impl View for wxView {}
impl EvtHandler for wxView {}
impl Object for wxView { fn handle(&self) -> *u8 { **self } }

impl wxView {
}

trait View : EvtHandler {
}

struct wxSound(*u8);
impl Sound for wxSound {}
impl EvtHandler for wxSound {}
impl Object for wxSound { fn handle(&self) -> *u8 { **self } }

impl wxSound {
    #[fixed_stack_segment]
    pub fn new(fileName: @String, isResource: bool) -> @Sound {
        unsafe { @wxSound(wxSound_Create(fileName.handle(), isResource)) as @Sound }
    }
}

trait Sound : EvtHandler {
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
impl Window for wxWindow {}
impl EvtHandler for wxWindow {}
impl Object for wxWindow { fn handle(&self) -> *u8 { **self } }

impl wxWindow {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> @Window {
        unsafe { @wxWindow(wxWindow_Create(_prt.handle(), _id, _x, _y, _w, _h, _stl)) as @Window }
    }
}

trait Window : EvtHandler {
    #[fixed_stack_segment]
    fn addChild(&self, child: @Window) {
        unsafe { wxWindow_AddChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn addConstraintReference(&self, otherWin: @Window) {
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
    fn clientToScreen(&self, x: c_int, y: c_int) -> @Point {
        unsafe { @wxPoint(wxWindow_ClientToScreen(self.handle(), x, y)) as @Point }
    }
    #[fixed_stack_segment]
    fn close(&self, _force: bool) -> bool {
        unsafe { wxWindow_Close(self.handle(), _force) }
    }
    #[fixed_stack_segment]
    fn convertDialogToPixels(&self) -> @Point {
        unsafe { @wxPoint(wxWindow_ConvertDialogToPixels(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialog(&self) -> @Point {
        unsafe { @wxPoint(wxWindow_ConvertPixelsToDialog(self.handle())) as @Point }
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
    fn findFocus(&self) -> @Window {
        unsafe { @wxWindow(wxWindow_FindFocus(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn findWindow(&self, name: @String) -> @Window {
        unsafe { @wxWindow(wxWindow_FindWindow(self.handle(), name.handle())) as @Window }
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
    fn getEffectiveMinSize(&self) -> @Size {
        unsafe { @wxSize(wxWindow_GetEffectiveMinSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBackgroundColour(&self, _ref: @Colour) {
        unsafe { wxWindow_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @Size {
        unsafe { @wxSize(wxWindow_GetBestSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getCaret(&self) -> @Caret {
        unsafe { @wxCaret(wxWindow_GetCaret(self.handle())) as @Caret }
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
    fn getClientData(&self) -> @ClientData {
        unsafe { @wxClientData(wxWindow_GetClientData(self.handle())) as @ClientData }
    }
    #[fixed_stack_segment]
    fn getClientSize(&self) -> @Size {
        unsafe { @wxSize(wxWindow_GetClientSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getConstraints(&self) -> @LayoutConstraints {
        unsafe { @wxLayoutConstraints(wxWindow_GetConstraints(self.handle())) as @LayoutConstraints }
    }
    #[fixed_stack_segment]
    fn getConstraintsInvolvedIn(&self) -> *u8 {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getCursor(&self) -> @Cursor {
        unsafe { @wxCursor(wxWindow_GetCursor(self.handle())) as @Cursor }
    }
    #[fixed_stack_segment]
    fn getDropTarget(&self) -> @DropTarget {
        unsafe { @wxDropTarget(wxWindow_GetDropTarget(self.handle())) as @DropTarget }
    }
    #[fixed_stack_segment]
    fn getEventHandler(&self) -> @EvtHandler {
        unsafe { @wxEvtHandler(wxWindow_GetEventHandler(self.handle())) as @EvtHandler }
    }
    #[fixed_stack_segment]
    fn getFont(&self, _ref: @Font) {
        unsafe { wxWindow_GetFont(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getForegroundColour(&self, _ref: @Colour) {
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
    fn getLabel(&self) -> @String {
        unsafe { @wxString(wxWindow_GetLabel(self.handle())) as @String }
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
    fn getName(&self) -> @String {
        unsafe { @wxString(wxWindow_GetName(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getParent(&self) -> @Window {
        unsafe { @wxWindow(wxWindow_GetParent(self.handle())) as @Window }
    }
    #[fixed_stack_segment]
    fn getPosition(&self) -> @Point {
        unsafe { @wxPoint(wxWindow_GetPosition(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.handle(), _x, _y) }
    }
    #[fixed_stack_segment]
    fn getRect(&self) -> @Rect {
        unsafe { @wxRect(wxWindow_GetRect(self.handle())) as @Rect }
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
    fn getSize(&self) -> @Size {
        unsafe { @wxSize(wxWindow_GetSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.handle(), _w, _h) }
    }
    #[fixed_stack_segment]
    fn getSizer(&self) -> @Sizer {
        unsafe { @wxSizer(wxWindow_GetSizer(self.handle())) as @Sizer }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, string: @String, x: *c_int, y: *c_int, descent: *c_int, externalLeading: *c_int, theFont: @Font) {
        unsafe { wxWindow_GetTextExtent(self.handle(), string.handle(), x, y, descent, externalLeading, theFont.handle()) }
    }
    #[fixed_stack_segment]
    fn getToolTip(&self) -> @String {
        unsafe { @wxString(wxWindow_GetToolTip(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getUpdateRegion(&self) -> @Region {
        unsafe { @wxRegion(wxWindow_GetUpdateRegion(self.handle())) as @Region }
    }
    #[fixed_stack_segment]
    fn getValidator(&self) -> @Validator {
        unsafe { @wxValidator(wxWindow_GetValidator(self.handle())) as @Validator }
    }
    #[fixed_stack_segment]
    fn getVirtualSize(&self) -> @Size {
        unsafe { @wxSize(wxWindow_GetVirtualSize(self.handle())) as @Size }
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
    fn popupMenu(&self, menu: @Menu, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.handle(), menu.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn prepareDC(&self, dc: @DC) {
        unsafe { wxWindow_PrepareDC(self.handle(), dc.handle()) }
    }
    #[fixed_stack_segment]
    fn pushEventHandler(&self, handler: @EvtHandler) {
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
    fn removeChild(&self, child: @Window) {
        unsafe { wxWindow_RemoveChild(self.handle(), child.handle()) }
    }
    #[fixed_stack_segment]
    fn removeConstraintReference(&self, otherWin: @Window) {
        unsafe { wxWindow_RemoveConstraintReference(self.handle(), otherWin.handle()) }
    }
    #[fixed_stack_segment]
    fn reparent(&self, _par: @Window) -> c_int {
        unsafe { wxWindow_Reparent(self.handle(), _par.handle()) }
    }
    #[fixed_stack_segment]
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.handle()) }
    }
    #[fixed_stack_segment]
    fn screenToClient(&self, x: c_int, y: c_int) -> @Point {
        unsafe { @wxPoint(wxWindow_ScreenToClient(self.handle(), x, y)) as @Point }
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
    fn setAcceleratorTable(&self, accel: @AcceleratorTable) {
        unsafe { wxWindow_SetAcceleratorTable(self.handle(), accel.handle()) }
    }
    #[fixed_stack_segment]
    fn setAutoLayout(&self, autoLayout: bool) {
        unsafe { wxWindow_SetAutoLayout(self.handle(), autoLayout) }
    }
    #[fixed_stack_segment]
    fn setBackgroundColour(&self, colour: @Colour) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setCaret(&self, caret: @Caret) {
        unsafe { wxWindow_SetCaret(self.handle(), caret.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientData(&self, data: @ClientData) {
        unsafe { wxWindow_SetClientData(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn setClientObject(&self, data: @ClientData) {
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
    fn setConstraints(&self, constraints: @LayoutConstraints) {
        unsafe { wxWindow_SetConstraints(self.handle(), constraints.handle()) }
    }
    #[fixed_stack_segment]
    fn setCursor(&self, cursor: @Cursor) -> c_int {
        unsafe { wxWindow_SetCursor(self.handle(), cursor.handle()) }
    }
    #[fixed_stack_segment]
    fn setDropTarget(&self, dropTarget: @DropTarget) {
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
    fn setFont(&self, font: @Font) -> c_int {
        unsafe { wxWindow_SetFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setForegroundColour(&self, colour: @Colour) -> c_int {
        unsafe { wxWindow_SetForegroundColour(self.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setId(&self, _id: c_int) {
        unsafe { wxWindow_SetId(self.handle(), _id) }
    }
    #[fixed_stack_segment]
    fn setLabel(&self, _title: @String) {
        unsafe { wxWindow_SetLabel(self.handle(), _title.handle()) }
    }
    #[fixed_stack_segment]
    fn setName(&self, _name: @String) {
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
    fn setSizer(&self, sizer: @Sizer) {
        unsafe { wxWindow_SetSizer(self.handle(), sizer.handle()) }
    }
    #[fixed_stack_segment]
    fn setToolTip(&self, tip: @String) {
        unsafe { wxWindow_SetToolTip(self.handle(), tip.handle()) }
    }
    #[fixed_stack_segment]
    fn setValidator(&self, validator: @Validator) {
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
    fn convertDialogToPixelsEx(&self) -> @Point {
        unsafe { @wxPoint(wxWindow_ConvertDialogToPixelsEx(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn convertPixelsToDialogEx(&self) -> @Point {
        unsafe { @wxPoint(wxWindow_ConvertPixelsToDialogEx(self.handle())) as @Point }
    }
    #[fixed_stack_segment]
    fn screenToClient2(&self, x: c_int, y: c_int) -> @Point {
        unsafe { @wxPoint(wxWindow_ScreenToClient2(self.handle(), x, y)) as @Point }
    }
}

struct wxWindowCreateEvent(*u8);
impl WindowCreateEvent for wxWindowCreateEvent {}
impl CommandEvent for wxWindowCreateEvent {}
impl Event for wxWindowCreateEvent {}
impl Object for wxWindowCreateEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowCreateEvent {
}

trait WindowCreateEvent : CommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @Window {
        unsafe { @wxWindow(wxWindowCreateEvent_GetWindow(self.handle())) as @Window }
    }
}

struct wxWindowDC(*u8);
impl WindowDC for wxWindowDC {}
impl DC for wxWindowDC {}
impl Object for wxWindowDC { fn handle(&self) -> *u8 { **self } }

impl wxWindowDC {
    #[fixed_stack_segment]
    pub fn new(win: @Window) -> @WindowDC {
        unsafe { @wxWindowDC(wxWindowDC_Create(win.handle())) as @WindowDC }
    }
}

trait WindowDC : DC {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxWindowDC_Delete(self.handle()) }
    }
}

struct wxWindowDestroyEvent(*u8);
impl WindowDestroyEvent for wxWindowDestroyEvent {}
impl CommandEvent for wxWindowDestroyEvent {}
impl Event for wxWindowDestroyEvent {}
impl Object for wxWindowDestroyEvent { fn handle(&self) -> *u8 { **self } }

impl wxWindowDestroyEvent {
}

trait WindowDestroyEvent : CommandEvent {
    #[fixed_stack_segment]
    fn getWindow(&self) -> @Window {
        unsafe { @wxWindow(wxWindowDestroyEvent_GetWindow(self.handle())) as @Window }
    }
}

struct wxWindowDisabler(*u8);
impl WindowDisabler for wxWindowDisabler { fn handle(&self) -> *u8 { **self } }

impl wxWindowDisabler {
}

trait WindowDisabler {
    fn handle(&self) -> *u8;
    
}

struct wxWizard(*u8);
impl Wizard for wxWizard {}
impl Dialog for wxWizard {}
impl TopLevelWindow for wxWizard {}
impl Window for wxWizard {}
impl EvtHandler for wxWizard {}
impl Object for wxWizard { fn handle(&self) -> *u8 { **self } }

impl wxWizard {
    #[fixed_stack_segment]
    pub fn chain(f: @WizardPageSimple, s: @WizardPageSimple) {
        unsafe { wxWizard_Chain(f.handle(), s.handle()) }
    }
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _bmp: @Bitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> @Wizard {
        unsafe { @wxWizard(wxWizard_Create(_prt.handle(), _id, _txt.handle(), _bmp.handle(), _lft, _top, _wdt, _hgt)) as @Wizard }
    }
}

trait Wizard : Dialog {
    #[fixed_stack_segment]
    fn getCurrentPage(&self) -> @WizardPage {
        unsafe { @wxWizardPage(wxWizard_GetCurrentPage(self.handle())) as @WizardPage }
    }
    #[fixed_stack_segment]
    fn getPageSize(&self) -> @Size {
        unsafe { @wxSize(wxWizard_GetPageSize(self.handle())) as @Size }
    }
    #[fixed_stack_segment]
    fn runWizard(&self, firstPage: @WizardPage) -> c_int {
        unsafe { wxWizard_RunWizard(self.handle(), firstPage.handle()) }
    }
    #[fixed_stack_segment]
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.handle(), w, h) }
    }
}

struct wxWizardEvent(*u8);
impl WizardEvent for wxWizardEvent {}
impl NotifyEvent for wxWizardEvent {}
impl CommandEvent for wxWizardEvent {}
impl Event for wxWizardEvent {}
impl Object for wxWizardEvent { fn handle(&self) -> *u8 { **self } }

impl wxWizardEvent {
}

trait WizardEvent : NotifyEvent {
    #[fixed_stack_segment]
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.handle()) }
    }
}

struct wxWizardPage(*u8);
impl WizardPage for wxWizardPage {}
impl Panel for wxWizardPage {}
impl Window for wxWizardPage {}
impl EvtHandler for wxWizardPage {}
impl Object for wxWizardPage { fn handle(&self) -> *u8 { **self } }

impl wxWizardPage {
}

trait WizardPage : Panel {
}

struct wxWizardPageSimple(*u8);
impl WizardPageSimple for wxWizardPageSimple {}
impl WizardPage for wxWizardPageSimple {}
impl Panel for wxWizardPageSimple {}
impl Window for wxWizardPageSimple {}
impl EvtHandler for wxWizardPageSimple {}
impl Object for wxWizardPageSimple { fn handle(&self) -> *u8 { **self } }

impl wxWizardPageSimple {
    #[fixed_stack_segment]
    pub fn new(_prt: @Wizard) -> @WizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_Create(_prt.handle())) as @WizardPageSimple }
    }
}

trait WizardPageSimple : WizardPage {
    #[fixed_stack_segment]
    fn getBitmap(&self, _ref: @Bitmap) {
        unsafe { wxWizardPageSimple_GetBitmap(self.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn getNext(&self) -> @WizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_GetNext(self.handle())) as @WizardPageSimple }
    }
    #[fixed_stack_segment]
    fn getPrev(&self) -> @WizardPageSimple {
        unsafe { @wxWizardPageSimple(wxWizardPageSimple_GetPrev(self.handle())) as @WizardPageSimple }
    }
    #[fixed_stack_segment]
    fn setNext(&self, next: @WizardPageSimple) {
        unsafe { wxWizardPageSimple_SetNext(self.handle(), next.handle()) }
    }
    #[fixed_stack_segment]
    fn setPrev(&self, prev: @WizardPageSimple) {
        unsafe { wxWizardPageSimple_SetPrev(self.handle(), prev.handle()) }
    }
}

struct wxXmlResource(*u8);
impl XmlResource for wxXmlResource {}
impl Object for wxXmlResource { fn handle(&self) -> *u8 { **self } }

impl wxXmlResource {
    #[fixed_stack_segment]
    pub fn new(flags: c_int) -> @XmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Create(flags)) as @XmlResource }
    }
    #[fixed_stack_segment]
    pub fn newFromFile(filemask: @String, flags: c_int) -> @XmlResource {
        unsafe { @wxXmlResource(wxXmlResource_CreateFromFile(filemask.handle(), flags)) as @XmlResource }
    }
    #[fixed_stack_segment]
    pub fn get() -> @XmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Get()) as @XmlResource }
    }
}

trait XmlResource : Object {
    #[fixed_stack_segment]
    fn addHandler(&self, handler: @EvtHandler) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn addSubclassFactory(&self, factory: *u8) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    #[fixed_stack_segment]
    fn attachUnknownControl(&self, control: @Control, parent: @Window) -> c_int {
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
    fn getDomain(&self) -> @String {
        unsafe { @wxString(wxXmlResource_GetDomain(self.handle())) as @String }
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
    fn getXRCID(&self, str_id: @String) -> c_int {
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    #[fixed_stack_segment]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    fn insertHandler(&self, handler: @EvtHandler) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    fn load(&self, filemask: @String) -> bool {
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn loadBitmap(&self, name: @String, _ref: @Bitmap) {
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadDialog(&self, parent: @Window, name: @String) -> @Dialog {
        unsafe { @wxDialog(wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle())) as @Dialog }
    }
    #[fixed_stack_segment]
    fn loadFrame(&self, parent: @Window, name: @String) -> @Frame {
        unsafe { @wxFrame(wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle())) as @Frame }
    }
    #[fixed_stack_segment]
    fn loadIcon(&self, name: @String, _ref: @Icon) {
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    fn loadMenu(&self, name: @String) -> @Menu {
        unsafe { @wxMenu(wxXmlResource_LoadMenu(self.handle(), name.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn loadMenuBar(&self, parent: @Window, name: @String) -> @MenuBar {
        unsafe { @wxMenuBar(wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle())) as @MenuBar }
    }
    #[fixed_stack_segment]
    fn loadPanel(&self, parent: @Window, name: @String) -> @Panel {
        unsafe { @wxPanel(wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle())) as @Panel }
    }
    #[fixed_stack_segment]
    fn loadToolBar(&self, parent: @Window, name: @String) -> @ToolBar {
        unsafe { @wxToolBar(wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle())) as @ToolBar }
    }
    #[fixed_stack_segment]
    fn getSizer(&self, str_id: @String) -> @Sizer {
        unsafe { @wxSizer(wxXmlResource_GetSizer(self.handle(), str_id.handle())) as @Sizer }
    }
    #[fixed_stack_segment]
    fn getBoxSizer(&self, str_id: @String) -> @BoxSizer {
        unsafe { @wxBoxSizer(wxXmlResource_GetBoxSizer(self.handle(), str_id.handle())) as @BoxSizer }
    }
    #[fixed_stack_segment]
    fn getStaticBoxSizer(&self, str_id: @String) -> @StaticBoxSizer {
        unsafe { @wxStaticBoxSizer(wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle())) as @StaticBoxSizer }
    }
    #[fixed_stack_segment]
    fn getGridSizer(&self, str_id: @String) -> @GridSizer {
        unsafe { @wxGridSizer(wxXmlResource_GetGridSizer(self.handle(), str_id.handle())) as @GridSizer }
    }
    #[fixed_stack_segment]
    fn getFlexGridSizer(&self, str_id: @String) -> @FlexGridSizer {
        unsafe { @wxFlexGridSizer(wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle())) as @FlexGridSizer }
    }
    #[fixed_stack_segment]
    fn getBitmapButton(&self, str_id: @String) -> @BitmapButton {
        unsafe { @wxBitmapButton(wxXmlResource_GetBitmapButton(self.handle(), str_id.handle())) as @BitmapButton }
    }
    #[fixed_stack_segment]
    fn getButton(&self, str_id: @String) -> @Button {
        unsafe { @wxButton(wxXmlResource_GetButton(self.handle(), str_id.handle())) as @Button }
    }
    #[fixed_stack_segment]
    fn getCalendarCtrl(&self, str_id: @String) -> @CalendarCtrl {
        unsafe { @wxCalendarCtrl(wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle())) as @CalendarCtrl }
    }
    #[fixed_stack_segment]
    fn getCheckBox(&self, str_id: @String) -> @CheckBox {
        unsafe { @wxCheckBox(wxXmlResource_GetCheckBox(self.handle(), str_id.handle())) as @CheckBox }
    }
    #[fixed_stack_segment]
    fn getCheckListBox(&self, str_id: @String) -> @CheckListBox {
        unsafe { @wxCheckListBox(wxXmlResource_GetCheckListBox(self.handle(), str_id.handle())) as @CheckListBox }
    }
    #[fixed_stack_segment]
    fn getChoice(&self, str_id: @String) -> @Choice {
        unsafe { @wxChoice(wxXmlResource_GetChoice(self.handle(), str_id.handle())) as @Choice }
    }
    #[fixed_stack_segment]
    fn getComboBox(&self, str_id: @String) -> @ComboBox {
        unsafe { @wxComboBox(wxXmlResource_GetComboBox(self.handle(), str_id.handle())) as @ComboBox }
    }
    #[fixed_stack_segment]
    fn getGauge(&self, str_id: @String) -> @Gauge {
        unsafe { @wxGauge(wxXmlResource_GetGauge(self.handle(), str_id.handle())) as @Gauge }
    }
    #[fixed_stack_segment]
    fn getGrid(&self, str_id: @String) -> @Grid {
        unsafe { @wxGrid(wxXmlResource_GetGrid(self.handle(), str_id.handle())) as @Grid }
    }
    #[fixed_stack_segment]
    fn getHtmlWindow(&self, str_id: @String) -> @HtmlWindow {
        unsafe { @wxHtmlWindow(wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle())) as @HtmlWindow }
    }
    #[fixed_stack_segment]
    fn getListBox(&self, str_id: @String) -> @ListBox {
        unsafe { @wxListBox(wxXmlResource_GetListBox(self.handle(), str_id.handle())) as @ListBox }
    }
    #[fixed_stack_segment]
    fn getListCtrl(&self, str_id: @String) -> @ListCtrl {
        unsafe { @wxListCtrl(wxXmlResource_GetListCtrl(self.handle(), str_id.handle())) as @ListCtrl }
    }
    #[fixed_stack_segment]
    fn getMDIChildFrame(&self, str_id: @String) -> @MDIChildFrame {
        unsafe { @wxMDIChildFrame(wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle())) as @MDIChildFrame }
    }
    #[fixed_stack_segment]
    fn getMDIParentFrame(&self, str_id: @String) -> @MDIParentFrame {
        unsafe { @wxMDIParentFrame(wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle())) as @MDIParentFrame }
    }
    #[fixed_stack_segment]
    fn getMenu(&self, str_id: @String) -> @Menu {
        unsafe { @wxMenu(wxXmlResource_GetMenu(self.handle(), str_id.handle())) as @Menu }
    }
    #[fixed_stack_segment]
    fn getMenuBar(&self, str_id: @String) -> @MenuBar {
        unsafe { @wxMenuBar(wxXmlResource_GetMenuBar(self.handle(), str_id.handle())) as @MenuBar }
    }
    #[fixed_stack_segment]
    fn getMenuItem(&self, str_id: @String) -> @MenuItem {
        unsafe { @wxMenuItem(wxXmlResource_GetMenuItem(self.handle(), str_id.handle())) as @MenuItem }
    }
    #[fixed_stack_segment]
    fn getNotebook(&self, str_id: @String) -> @Notebook {
        unsafe { @wxNotebook(wxXmlResource_GetNotebook(self.handle(), str_id.handle())) as @Notebook }
    }
    #[fixed_stack_segment]
    fn getPanel(&self, str_id: @String) -> @Panel {
        unsafe { @wxPanel(wxXmlResource_GetPanel(self.handle(), str_id.handle())) as @Panel }
    }
    #[fixed_stack_segment]
    fn getRadioButton(&self, str_id: @String) -> @RadioButton {
        unsafe { @wxRadioButton(wxXmlResource_GetRadioButton(self.handle(), str_id.handle())) as @RadioButton }
    }
    #[fixed_stack_segment]
    fn getRadioBox(&self, str_id: @String) -> @RadioBox {
        unsafe { @wxRadioBox(wxXmlResource_GetRadioBox(self.handle(), str_id.handle())) as @RadioBox }
    }
    #[fixed_stack_segment]
    fn getScrollBar(&self, str_id: @String) -> @ScrollBar {
        unsafe { @wxScrollBar(wxXmlResource_GetScrollBar(self.handle(), str_id.handle())) as @ScrollBar }
    }
    #[fixed_stack_segment]
    fn getScrolledWindow(&self, str_id: @String) -> @ScrolledWindow {
        unsafe { @wxScrolledWindow(wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle())) as @ScrolledWindow }
    }
    #[fixed_stack_segment]
    fn getSlider(&self, str_id: @String) -> @Slider {
        unsafe { @wxSlider(wxXmlResource_GetSlider(self.handle(), str_id.handle())) as @Slider }
    }
    #[fixed_stack_segment]
    fn getSpinButton(&self, str_id: @String) -> @SpinButton {
        unsafe { @wxSpinButton(wxXmlResource_GetSpinButton(self.handle(), str_id.handle())) as @SpinButton }
    }
    #[fixed_stack_segment]
    fn getSpinCtrl(&self, str_id: @String) -> @SpinCtrl {
        unsafe { @wxSpinCtrl(wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle())) as @SpinCtrl }
    }
    #[fixed_stack_segment]
    fn getSplitterWindow(&self, str_id: @String) -> @SplitterWindow {
        unsafe { @wxSplitterWindow(wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle())) as @SplitterWindow }
    }
    #[fixed_stack_segment]
    fn getStaticBitmap(&self, str_id: @String) -> @StaticBitmap {
        unsafe { @wxStaticBitmap(wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle())) as @StaticBitmap }
    }
    #[fixed_stack_segment]
    fn getStaticBox(&self, str_id: @String) -> @StaticBox {
        unsafe { @wxStaticBox(wxXmlResource_GetStaticBox(self.handle(), str_id.handle())) as @StaticBox }
    }
    #[fixed_stack_segment]
    fn getStaticLine(&self, str_id: @String) -> @StaticLine {
        unsafe { @wxStaticLine(wxXmlResource_GetStaticLine(self.handle(), str_id.handle())) as @StaticLine }
    }
    #[fixed_stack_segment]
    fn getStaticText(&self, str_id: @String) -> @StaticText {
        unsafe { @wxStaticText(wxXmlResource_GetStaticText(self.handle(), str_id.handle())) as @StaticText }
    }
    #[fixed_stack_segment]
    fn getTextCtrl(&self, str_id: @String) -> @TextCtrl {
        unsafe { @wxTextCtrl(wxXmlResource_GetTextCtrl(self.handle(), str_id.handle())) as @TextCtrl }
    }
    #[fixed_stack_segment]
    fn getTreeCtrl(&self, str_id: @String) -> @TreeCtrl {
        unsafe { @wxTreeCtrl(wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle())) as @TreeCtrl }
    }
    #[fixed_stack_segment]
    fn unload(&self, filemask: @String) -> bool {
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    fn set(&self, res: @XmlResource) -> @XmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Set(self.handle(), res.handle())) as @XmlResource }
    }
    #[fixed_stack_segment]
    fn setDomain(&self, domain: @String) {
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    #[fixed_stack_segment]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn getStyledTextCtrl(&self, str_id: @String) -> @StyledTextCtrl {
        unsafe { @wxStyledTextCtrl(wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle())) as @StyledTextCtrl }
    }
}

struct wxXmlResourceHandler(*u8);
impl XmlResourceHandler for wxXmlResourceHandler {}
impl Object for wxXmlResourceHandler { fn handle(&self) -> *u8 { **self } }

impl wxXmlResourceHandler {
}

trait XmlResourceHandler : Object {
}

struct wxZipInputStream(*u8);
impl ZipInputStream for wxZipInputStream {}
impl InputStream for wxZipInputStream {}
impl StreamBase for wxZipInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZipInputStream {
}

trait ZipInputStream : InputStream {
}

struct wxZlibInputStream(*u8);
impl ZlibInputStream for wxZlibInputStream {}
impl FilterInputStream for wxZlibInputStream {}
impl InputStream for wxZlibInputStream {}
impl StreamBase for wxZlibInputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibInputStream {
}

trait ZlibInputStream : FilterInputStream {
}

struct wxZlibOutputStream(*u8);
impl ZlibOutputStream for wxZlibOutputStream {}
impl FilterOutputStream for wxZlibOutputStream {}
impl OutputStream for wxZlibOutputStream {}
impl StreamBase for wxZlibOutputStream { fn handle(&self) -> *u8 { **self } }

impl wxZlibOutputStream {
}

trait ZlibOutputStream : FilterOutputStream {
}

struct wxPropertyGrid(*u8);
impl PropertyGrid for wxPropertyGrid {}
impl Control for wxPropertyGrid {}
impl Window for wxPropertyGrid {}
impl EvtHandler for wxPropertyGrid {}
impl Object for wxPropertyGrid { fn handle(&self) -> *u8 { **self } }

impl wxPropertyGrid {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> @PropertyGrid {
        unsafe { @wxPropertyGrid(wxPropertyGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl)) as @PropertyGrid }
    }
}

trait PropertyGrid : Control {
    #[fixed_stack_segment]
    fn append(&self, prop: @PGProperty) -> @PGProperty {
        unsafe { @wxPGProperty(wxPropertyGrid_Append(self.handle(), prop.handle())) as @PGProperty }
    }
    #[fixed_stack_segment]
    fn disableProperty(&self, propName: @String) -> bool {
        unsafe { wxPropertyGrid_DisableProperty(self.handle(), propName.handle()) }
    }
}

struct wxPropertyGridEvent(*u8);
impl PropertyGridEvent for wxPropertyGridEvent {}
impl NotifyEvent for wxPropertyGridEvent {}
impl CommandEvent for wxPropertyGridEvent {}
impl Event for wxPropertyGridEvent {}
impl Object for wxPropertyGridEvent { fn handle(&self) -> *u8 { **self } }

impl wxPropertyGridEvent {
}

trait PropertyGridEvent : NotifyEvent {
    #[fixed_stack_segment]
    fn hasProperty(&self) -> bool {
        unsafe { wxPropertyGridEvent_HasProperty(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getProperty(&self) -> @PGProperty {
        unsafe { @wxPGProperty(wxPropertyGridEvent_GetProperty(self.handle())) as @PGProperty }
    }
}

struct wxPGProperty(*u8);
impl PGProperty for wxPGProperty {}
impl Object for wxPGProperty { fn handle(&self) -> *u8 { **self } }

impl wxPGProperty {
}

trait PGProperty : Object {
    #[fixed_stack_segment]
    fn getLabel(&self) -> @String {
        unsafe { @wxString(wxPGProperty_GetLabel(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getName(&self) -> @String {
        unsafe { @wxString(wxPGProperty_GetName(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getValueAsString(&self) -> @String {
        unsafe { @wxString(wxPGProperty_GetValueAsString(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getValueType(&self) -> @String {
        unsafe { @wxString(wxPGProperty_GetValueType(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn setHelpString(&self, helpString: @String) {
        unsafe { wxPGProperty_SetHelpString(self.handle(), helpString.handle()) }
    }
}

struct wxStringProperty(*u8);
impl StringProperty for wxStringProperty {}
impl PGProperty for wxStringProperty {}
impl Object for wxStringProperty { fn handle(&self) -> *u8 { **self } }

impl wxStringProperty {
    #[fixed_stack_segment]
    pub fn new(label: @String, name: @String, value: @String) -> @StringProperty {
        unsafe { @wxStringProperty(wxStringProperty_Create(label.handle(), name.handle(), value.handle())) as @StringProperty }
    }
}

trait StringProperty : PGProperty {
}

struct wxIntProperty(*u8);
impl IntProperty for wxIntProperty {}
impl PGProperty for wxIntProperty {}
impl Object for wxIntProperty { fn handle(&self) -> *u8 { **self } }

impl wxIntProperty {
    #[fixed_stack_segment]
    pub fn new(label: @String, name: @String, value: c_int) -> @IntProperty {
        unsafe { @wxIntProperty(wxIntProperty_Create(label.handle(), name.handle(), value)) as @IntProperty }
    }
}

trait IntProperty : PGProperty {
}

struct wxBoolProperty(*u8);
impl BoolProperty for wxBoolProperty {}
impl PGProperty for wxBoolProperty {}
impl Object for wxBoolProperty { fn handle(&self) -> *u8 { **self } }

impl wxBoolProperty {
    #[fixed_stack_segment]
    pub fn new(label: @String, name: @String, value: bool) -> @BoolProperty {
        unsafe { @wxBoolProperty(wxBoolProperty_Create(label.handle(), name.handle(), value)) as @BoolProperty }
    }
}

trait BoolProperty : PGProperty {
}

struct wxFloatProperty(*u8);
impl FloatProperty for wxFloatProperty {}
impl PGProperty for wxFloatProperty {}
impl Object for wxFloatProperty { fn handle(&self) -> *u8 { **self } }

impl wxFloatProperty {
    #[fixed_stack_segment]
    pub fn new(label: @String, name: @String, value: c_float) -> @FloatProperty {
        unsafe { @wxFloatProperty(wxFloatProperty_Create(label.handle(), name.handle(), value)) as @FloatProperty }
    }
}

trait FloatProperty : PGProperty {
}

struct wxDateProperty(*u8);
impl DateProperty for wxDateProperty {}
impl PGProperty for wxDateProperty {}
impl Object for wxDateProperty { fn handle(&self) -> *u8 { **self } }

impl wxDateProperty {
    #[fixed_stack_segment]
    pub fn new(label: @String, name: @String, value: @DateTime) -> @DateProperty {
        unsafe { @wxDateProperty(wxDateProperty_Create(label.handle(), name.handle(), value.handle())) as @DateProperty }
    }
}

trait DateProperty : PGProperty {
}

struct wxFileProperty(*u8);
impl FileProperty for wxFileProperty {}
impl PGProperty for wxFileProperty {}
impl Object for wxFileProperty { fn handle(&self) -> *u8 { **self } }

impl wxFileProperty {
    #[fixed_stack_segment]
    pub fn new(label: @String, name: @String, value: @String) -> @FileProperty {
        unsafe { @wxFileProperty(wxFileProperty_Create(label.handle(), name.handle(), value.handle())) as @FileProperty }
    }
}

trait FileProperty : PGProperty {
}

struct wxPropertyCategory(*u8);
impl PropertyCategory for wxPropertyCategory {}
impl PGProperty for wxPropertyCategory {}
impl Object for wxPropertyCategory { fn handle(&self) -> *u8 { **self } }

impl wxPropertyCategory {
    #[fixed_stack_segment]
    pub fn new(label: @String) -> @PropertyCategory {
        unsafe { @wxPropertyCategory(wxPropertyCategory_Create(label.handle())) as @PropertyCategory }
    }
}

trait PropertyCategory : PGProperty {
}

struct wxGenericDragImage(*u8);
impl GenericDragImage for wxGenericDragImage {}
impl DragImage for wxGenericDragImage {}
impl Object for wxGenericDragImage { fn handle(&self) -> *u8 { **self } }

impl wxGenericDragImage {
    #[fixed_stack_segment]
    pub fn new(cursor: @Cursor) -> @GenericDragImage {
        unsafe { @wxGenericDragImage(wxGenericDragImage_Create(cursor.handle())) as @GenericDragImage }
    }
}

trait GenericDragImage : DragImage {
    #[fixed_stack_segment]
    fn doDrawImage(&self, dc: @DC, x: c_int, y: c_int) -> bool {
        unsafe { wxGenericDragImage_DoDrawImage(self.handle(), dc.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> @Rect {
        unsafe { @wxRect(wxGenericDragImage_GetImageRect(self.handle(), x_pos, y_pos)) as @Rect }
    }
    #[fixed_stack_segment]
    fn updateBackingFromWindow(&self, windowDC: @DC, destDC: @MemoryDC, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> bool {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.handle(), windowDC.handle(), destDC.handle(), x, y, w, h, xdest, ydest, width, height) }
    }
}

struct wxGraphicsObject(*u8);
impl GraphicsObject for wxGraphicsObject {}
impl Object for wxGraphicsObject { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsObject {
    #[fixed_stack_segment]
    pub fn getRenderer() -> @GraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsObject_GetRenderer()) as @GraphicsRenderer }
    }
}

trait GraphicsObject : Object {
    #[fixed_stack_segment]
    fn isNull(&self) -> bool {
        unsafe { wxGraphicsObject_IsNull(self.handle()) }
    }
}

struct wxGraphicsBrush(*u8);
impl GraphicsBrush for wxGraphicsBrush {}
impl GraphicsObject for wxGraphicsBrush {}
impl Object for wxGraphicsBrush { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsBrush {
    #[fixed_stack_segment]
    pub fn new() -> @GraphicsBrush {
        unsafe { @wxGraphicsBrush(wxGraphicsBrush_Create()) as @GraphicsBrush }
    }
}

trait GraphicsBrush : GraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsBrush_Delete(self.handle()) }
    }
}

struct wxGraphicsContext(*u8);
impl GraphicsContext for wxGraphicsContext {}
impl GraphicsObject for wxGraphicsContext {}
impl Object for wxGraphicsContext { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsContext {
    #[fixed_stack_segment]
    pub fn new(dc: @WindowDC) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_Create(dc.handle())) as @GraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newFromWindow(window: @Window) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromWindow(window.handle())) as @GraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newFromNative(context: *u8) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNative(context)) as @GraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newFromNativeWindow(window: *u8) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsContext_CreateFromNativeWindow(window)) as @GraphicsContext }
    }
}

trait GraphicsContext : GraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsContext_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn clip(&self, region: @Region) {
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
    fn drawBitmap(&self, bmp: @Bitmap, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.handle(), bmp.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawIcon(&self, icon: @Icon, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.handle(), icon.handle(), x, y, w, h) }
    }
    #[fixed_stack_segment]
    fn drawLines(&self, n: size_t, x: *u8, y: *u8, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.handle(), n, x, y, style) }
    }
    #[fixed_stack_segment]
    fn drawPath(&self, path: @GraphicsPath, style: c_int) {
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
    fn drawText(&self, text: @String, x: c_double, y: c_double) {
        unsafe { wxGraphicsContext_DrawText(self.handle(), text.handle(), x, y) }
    }
    #[fixed_stack_segment]
    fn drawTextWithAngle(&self, text: @String, x: c_double, y: c_double, radius: c_double) {
        unsafe { wxGraphicsContext_DrawTextWithAngle(self.handle(), text.handle(), x, y, radius) }
    }
    #[fixed_stack_segment]
    fn fillPath(&self, path: @GraphicsPath, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.handle(), path.handle(), style) }
    }
    #[fixed_stack_segment]
    fn strokePath(&self, path: @GraphicsPath) {
        unsafe { wxGraphicsContext_StrokePath(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativeContext(&self) -> *u8 {
        unsafe { wxGraphicsContext_GetNativeContext(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getTextExtent(&self, text: @String, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double) {
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
    fn setTransform(&self, path: @GraphicsMatrix) {
        unsafe { wxGraphicsContext_SetTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn concatTransform(&self, path: @GraphicsMatrix) {
        unsafe { wxGraphicsContext_ConcatTransform(self.handle(), path.handle()) }
    }
    #[fixed_stack_segment]
    fn setBrush(&self, brush: @Brush) {
        unsafe { wxGraphicsContext_SetBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsBrush(&self, brush: @GraphicsBrush) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.handle(), brush.handle()) }
    }
    #[fixed_stack_segment]
    fn setFont(&self, font: @Font, colour: @Colour) {
        unsafe { wxGraphicsContext_SetFont(self.handle(), font.handle(), colour.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsFont(&self, font: @GraphicsFont) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.handle(), font.handle()) }
    }
    #[fixed_stack_segment]
    fn setPen(&self, pen: @Pen) {
        unsafe { wxGraphicsContext_SetPen(self.handle(), pen.handle()) }
    }
    #[fixed_stack_segment]
    fn setGraphicsPen(&self, pen: @GraphicsPen) {
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
impl GraphicsFont for wxGraphicsFont {}
impl GraphicsObject for wxGraphicsFont {}
impl Object for wxGraphicsFont { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsFont {
    #[fixed_stack_segment]
    pub fn new() -> @GraphicsFont {
        unsafe { @wxGraphicsFont(wxGraphicsFont_Create()) as @GraphicsFont }
    }
}

trait GraphicsFont : GraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsFont_Delete(self.handle()) }
    }
}

struct wxGraphicsMatrix(*u8);
impl GraphicsMatrix for wxGraphicsMatrix {}
impl GraphicsObject for wxGraphicsMatrix {}
impl Object for wxGraphicsMatrix { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsMatrix {
    #[fixed_stack_segment]
    pub fn new() -> @GraphicsMatrix {
        unsafe { @wxGraphicsMatrix(wxGraphicsMatrix_Create()) as @GraphicsMatrix }
    }
}

trait GraphicsMatrix : GraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsMatrix_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn concat(&self, t: @GraphicsMatrix) {
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
    fn isEqual(&self, t: @GraphicsMatrix) -> bool {
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
impl GraphicsPath for wxGraphicsPath {}
impl GraphicsObject for wxGraphicsPath {}
impl Object for wxGraphicsPath { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsPath {
    #[fixed_stack_segment]
    pub fn new() -> @GraphicsPath {
        unsafe { @wxGraphicsPath(wxGraphicsPath_Create()) as @GraphicsPath }
    }
    #[fixed_stack_segment]
    pub fn unGetNativePath(p: *u8) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

trait GraphicsPath : GraphicsObject {
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
    fn addPath(&self, x: c_double, y: c_double, path: @GraphicsPath) {
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
    fn transform(&self, matrix: @GraphicsMatrix) {
        unsafe { wxGraphicsPath_Transform(self.handle(), matrix.handle()) }
    }
    #[fixed_stack_segment]
    fn getNativePath(&self) -> *u8 {
        unsafe { wxGraphicsPath_GetNativePath(self.handle()) }
    }
}

struct wxGraphicsPen(*u8);
impl GraphicsPen for wxGraphicsPen {}
impl GraphicsObject for wxGraphicsPen {}
impl Object for wxGraphicsPen { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsPen {
    #[fixed_stack_segment]
    pub fn new() -> @GraphicsPen {
        unsafe { @wxGraphicsPen(wxGraphicsPen_Create()) as @GraphicsPen }
    }
}

trait GraphicsPen : GraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsPen_Delete(self.handle()) }
    }
}

struct wxGraphicsRenderer(*u8);
impl GraphicsRenderer for wxGraphicsRenderer {}
impl GraphicsObject for wxGraphicsRenderer {}
impl Object for wxGraphicsRenderer { fn handle(&self) -> *u8 { **self } }

impl wxGraphicsRenderer {
    #[fixed_stack_segment]
    pub fn newContext(dc: @WindowDC) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContext(dc.handle())) as @GraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newContextFromWindow(window: @Window) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromWindow(window.handle())) as @GraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newContextFromNativeContext(context: *u8) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeContext(context)) as @GraphicsContext }
    }
    #[fixed_stack_segment]
    pub fn newContextFromNativeWindow(window: *u8) -> @GraphicsContext {
        unsafe { @wxGraphicsContext(wxGraphicsRenderer_CreateContextFromNativeWindow(window)) as @GraphicsContext }
    }
}

trait GraphicsRenderer : GraphicsObject {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxGraphicsRenderer_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getDefaultRenderer(&self) -> @GraphicsRenderer {
        unsafe { @wxGraphicsRenderer(wxGraphicsRenderer_GetDefaultRenderer(self.handle())) as @GraphicsRenderer }
    }
}

struct wxGLContext(*u8);
impl GLContext for wxGLContext {}
impl Object for wxGLContext { fn handle(&self) -> *u8 { **self } }

impl wxGLContext {
    #[fixed_stack_segment]
    pub fn new(win: @GLCanvas, other: @GLContext) -> @GLContext {
        unsafe { @wxGLContext(wxGLContext_Create(win.handle(), other.handle())) as @GLContext }
    }
    #[fixed_stack_segment]
    pub fn newFromNull(win: @GLCanvas) -> @GLContext {
        unsafe { @wxGLContext(wxGLContext_CreateFromNull(win.handle())) as @GLContext }
    }
}

trait GLContext : Object {
    #[fixed_stack_segment]
    fn setCurrent(&self, win: @GLCanvas) -> bool {
        unsafe { wxGLContext_SetCurrent(self.handle(), win.handle()) }
    }
}

struct wxManagedPtr(*u8);
impl ManagedPtr for wxManagedPtr { fn handle(&self) -> *u8 { **self } }

impl wxManagedPtr {
    #[fixed_stack_segment]
    pub fn getDeleteFunction() -> *u8 {
        unsafe { wxManagedPtr_GetDeleteFunction() }
    }
    #[fixed_stack_segment]
    pub fn newFromObject(obj: @Object) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromObject(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromDateTime(obj: @DateTime) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromDateTime(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromGridCellCoordsArray(obj: @GridCellCoordsArray) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromGridCellCoordsArray(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromBitmap(obj: @Bitmap) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromBitmap(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromIcon(obj: @Icon) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromIcon(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromBrush(obj: @Brush) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromBrush(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromColour(obj: @Colour) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromColour(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromCursor(obj: @Cursor) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromCursor(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromFont(obj: @Font) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromFont(obj.handle())) as @ManagedPtr }
    }
    #[fixed_stack_segment]
    pub fn newFromPen(obj: @Pen) -> @ManagedPtr {
        unsafe { @wxManagedPtr(wxManagedPtr_CreateFromPen(obj.handle())) as @ManagedPtr }
    }
}

trait ManagedPtr {
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
impl MediaCtrl for wxMediaCtrl {}
impl Window for wxMediaCtrl {}
impl EvtHandler for wxMediaCtrl {}
impl Object for wxMediaCtrl { fn handle(&self) -> *u8 { **self } }

impl wxMediaCtrl {
    #[fixed_stack_segment]
    pub fn new(parent: @Window, windowID: c_int, fileName: @String, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long, szBackend: @String, name: @String) -> @MediaCtrl {
        unsafe { @wxMediaCtrl(wxMediaCtrl_Create(parent.handle(), windowID, fileName.handle(), x, y, w, h, style, szBackend.handle(), name.handle())) as @MediaCtrl }
    }
}

trait MediaCtrl : Window {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxMediaCtrl_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn getBestSize(&self) -> @Size {
        unsafe { @wxSize(wxMediaCtrl_GetBestSize(self.handle())) as @Size }
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
    fn load(&self, fileName: @String) -> bool {
        unsafe { wxMediaCtrl_Load(self.handle(), fileName.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURI(&self, uri: @String) -> bool {
        unsafe { wxMediaCtrl_LoadURI(self.handle(), uri.handle()) }
    }
    #[fixed_stack_segment]
    fn loadURIWithProxy(&self, uri: @String, proxy: @String) -> bool {
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
impl MediaEvent for wxMediaEvent {}
impl NotifyEvent for wxMediaEvent {}
impl CommandEvent for wxMediaEvent {}
impl Event for wxMediaEvent {}
impl Object for wxMediaEvent { fn handle(&self) -> *u8 { **self } }

impl wxMediaEvent {
}

trait MediaEvent : NotifyEvent {
}

struct wxcPrintout(*u8);
impl cPrintout for wxcPrintout {}
impl Printout for wxcPrintout {}
impl Object for wxcPrintout { fn handle(&self) -> *u8 { **self } }

impl wxcPrintout {
    #[fixed_stack_segment]
    pub fn new(title: @String) -> @cPrintout {
        unsafe { @wxcPrintout(wxcPrintout_Create(title.handle())) as @cPrintout }
    }
}

trait cPrintout : Printout {
    #[fixed_stack_segment]
    fn delete(&self) {
        unsafe { wxcPrintout_Delete(self.handle()) }
    }
    #[fixed_stack_segment]
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.handle(), startPage, endPage, fromPage, toPage) }
    }
    #[fixed_stack_segment]
    fn getEvtHandler(&self) -> @cPrintoutHandler {
        unsafe { @wxcPrintoutHandler(wxcPrintout_GetEvtHandler(self.handle())) as @cPrintoutHandler }
    }
}

struct wxcPrintEvent(*u8);
impl cPrintEvent for wxcPrintEvent {}
impl Event for wxcPrintEvent {}
impl Object for wxcPrintEvent { fn handle(&self) -> *u8 { **self } }

impl wxcPrintEvent {
}

trait cPrintEvent : Event {
    #[fixed_stack_segment]
    fn getPrintout(&self) -> @cPrintout {
        unsafe { @wxcPrintout(wxcPrintEvent_GetPrintout(self.handle())) as @cPrintout }
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
impl cPrintoutHandler for wxcPrintoutHandler {}
impl EvtHandler for wxcPrintoutHandler {}
impl Object for wxcPrintoutHandler { fn handle(&self) -> *u8 { **self } }

impl wxcPrintoutHandler {
}

trait cPrintoutHandler : EvtHandler {
}

struct wxStyledTextCtrl(*u8);
impl StyledTextCtrl for wxStyledTextCtrl {}
impl Control for wxStyledTextCtrl {}
impl Window for wxStyledTextCtrl {}
impl EvtHandler for wxStyledTextCtrl {}
impl Object for wxStyledTextCtrl { fn handle(&self) -> *u8 { **self } }

impl wxStyledTextCtrl {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _txt: @String, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> @StyledTextCtrl {
        unsafe { @wxStyledTextCtrl(wxStyledTextCtrl_Create(_prt.handle(), _id, _txt.handle(), _lft, _top, _wdt, _hgt, style)) as @StyledTextCtrl }
    }
}

trait StyledTextCtrl : Control {
    #[fixed_stack_segment]
    fn addText(&self, text: @String) {
        unsafe { wxStyledTextCtrl_AddText(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn addStyledText(&self, data: @MemoryBuffer) {
        unsafe { wxStyledTextCtrl_AddStyledText(self.handle(), data.handle()) }
    }
    #[fixed_stack_segment]
    fn insertText(&self, pos: c_int, text: @String) {
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
    fn markerDefineBitmap(&self, markerNumber: c_int, bmp: @Bitmap) {
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
    fn styleSetFaceName(&self, style: c_int, fontName: @String) {
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
    fn setWordChars(&self, characters: @String) {
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
    fn autoCompShow(&self, lenEntered: c_int, itemList: @String) {
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
    fn autoCompStops(&self, characterSet: @String) {
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
    fn autoCompSelect(&self, text: @String) {
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
    fn autoCompSetFillUps(&self, characterSet: @String) {
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
    fn userListShow(&self, listType: c_int, itemList: @String) {
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
    fn registerImage(&self, type_: c_int, bmp: @Bitmap) {
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
    fn findText(&self, minPos: c_int, maxPos: c_int, text: @String, flags: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_FindText(self.handle(), minPos, maxPos, text.handle(), flags) }
    }
    #[fixed_stack_segment]
    fn formatRange(&self, doDraw: bool, startPos: c_int, endPos: c_int, draw: @DC, target: @DC, renderRect: @Rect, pageRect: @Rect) -> c_int {
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
    fn replaceSelection(&self, text: @String) {
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
    fn setText(&self, text: @String) {
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
    fn replaceTarget(&self, text: @String) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTarget(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn replaceTargetRE(&self, text: @String) -> c_int {
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self.handle(), text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchInTarget(&self, text: @String) -> c_int {
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
    fn callTipShow(&self, pos: c_int, definition: @String) {
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
    fn textWidth(&self, style: c_int, text: @String) -> c_int {
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
    fn appendText(&self, text: @String) {
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
    fn setDocPointer(&self, docPointer: @STCDoc) {
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
    fn searchNext(&self, flags: c_int, text: @String) -> c_int {
        unsafe { wxStyledTextCtrl_SearchNext(self.handle(), flags, text.handle()) }
    }
    #[fixed_stack_segment]
    fn searchPrev(&self, flags: c_int, text: @String) -> c_int {
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
    fn addRefDocument(&self, docPointer: @STCDoc) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self.handle(), docPointer.handle()) }
    }
    #[fixed_stack_segment]
    fn releaseDocument(&self, docPointer: @STCDoc) {
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
    fn copyText(&self, length: c_int, text: @String) {
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
    fn setProperty(&self, key: @String, value: @String) {
        unsafe { wxStyledTextCtrl_SetProperty(self.handle(), key.handle(), value.handle()) }
    }
    #[fixed_stack_segment]
    fn setKeyWords(&self, keywordSet: c_int, keyWords: @String) {
        unsafe { wxStyledTextCtrl_SetKeyWords(self.handle(), keywordSet, keyWords.handle()) }
    }
    #[fixed_stack_segment]
    fn setLexerLanguage(&self, language: @String) {
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self.handle(), language.handle()) }
    }
    #[fixed_stack_segment]
    fn getCurrentLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetSpec(&self, styleNum: c_int, spec: @String) {
        unsafe { wxStyledTextCtrl_StyleSetSpec(self.handle(), styleNum, spec.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFont(&self, styleNum: c_int, font: @Font) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self.handle(), styleNum, font.handle()) }
    }
    #[fixed_stack_segment]
    fn styleSetFontAttr(&self, styleNum: c_int, size: c_int, faceName: @String, bold: bool, italic: bool, underline: bool) {
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
    fn setVScrollBar(&self, bar: @ScrollBar) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self.handle(), bar.handle()) }
    }
    #[fixed_stack_segment]
    fn setHScrollBar(&self, bar: @ScrollBar) {
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
    fn saveFile(&self, filename: @String) -> bool {
        unsafe { wxStyledTextCtrl_SaveFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn loadFile(&self, filename: @String) -> bool {
        unsafe { wxStyledTextCtrl_LoadFile(self.handle(), filename.handle()) }
    }
    #[fixed_stack_segment]
    fn indicatorGetForeground(&self, indic: c_int) -> @Colour {
        unsafe { @wxColour(wxStyledTextCtrl_IndicatorGetForeground(self.handle(), indic)) as @Colour }
    }
    #[fixed_stack_segment]
    fn getCaretLineBackground(&self) -> @Colour {
        unsafe { @wxColour(wxStyledTextCtrl_GetCaretLineBackground(self.handle())) as @Colour }
    }
    #[fixed_stack_segment]
    fn setCaretLineBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self.handle(), back_r, back_g, back_b) }
    }
    #[fixed_stack_segment]
    fn getCaretForeground(&self) -> @Colour {
        unsafe { @wxColour(wxStyledTextCtrl_GetCaretForeground(self.handle())) as @Colour }
    }
    #[fixed_stack_segment]
    fn getLine(&self, line: c_int) -> @String {
        unsafe { @wxString(wxStyledTextCtrl_GetLine(self.handle(), line)) as @String }
    }
    #[fixed_stack_segment]
    fn getText(&self) -> @String {
        unsafe { @wxString(wxStyledTextCtrl_GetText(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getTextRange(&self, startPos: c_int, endPos: c_int) -> @String {
        unsafe { @wxString(wxStyledTextCtrl_GetTextRange(self.handle(), startPos, endPos)) as @String }
    }
    #[fixed_stack_segment]
    fn getSelectedText(&self) -> @String {
        unsafe { @wxString(wxStyledTextCtrl_GetSelectedText(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn newDocument(&self) -> @STCDoc {
        unsafe { @wxSTCDoc(wxStyledTextCtrl_CreateDocument(self.handle())) as @STCDoc }
    }
    #[fixed_stack_segment]
    fn getEdgeColour(&self) -> @Colour {
        unsafe { @wxColour(wxStyledTextCtrl_GetEdgeColour(self.handle())) as @Colour }
    }
    #[fixed_stack_segment]
    fn getDocPointer(&self) -> @STCDoc {
        unsafe { @wxSTCDoc(wxStyledTextCtrl_GetDocPointer(self.handle())) as @STCDoc }
    }
    #[fixed_stack_segment]
    fn pointFromPosition(&self) -> @Point {
        unsafe { @wxPoint(wxStyledTextCtrl_PointFromPosition(self.handle())) as @Point }
    }
}

struct wxSTCDoc(*u8);
impl STCDoc for wxSTCDoc { fn handle(&self) -> *u8 { **self } }

impl wxSTCDoc {
}

trait STCDoc {
    fn handle(&self) -> *u8;
    
}

struct wxMemoryBuffer(*u8);
impl MemoryBuffer for wxMemoryBuffer { fn handle(&self) -> *u8 { **self } }

impl wxMemoryBuffer {
}

trait MemoryBuffer {
    fn handle(&self) -> *u8;
    
}

struct wxStyledTextEvent(*u8);
impl StyledTextEvent for wxStyledTextEvent {}
impl CommandEvent for wxStyledTextEvent {}
impl Event for wxStyledTextEvent {}
impl Object for wxStyledTextEvent { fn handle(&self) -> *u8 { **self } }

impl wxStyledTextEvent {
}

trait StyledTextEvent : CommandEvent {
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
    fn getDragText(&self) -> @String {
        unsafe { @wxString(wxStyledTextEvent_GetDragText(self.handle())) as @String }
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
    fn getText(&self) -> @String {
        unsafe { @wxString(wxStyledTextEvent_GetText(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn clone(&self) -> @StyledTextEvent {
        unsafe { @wxStyledTextEvent(wxStyledTextEvent_Clone(self.handle())) as @StyledTextEvent }
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
    fn setText(&self, t: @String) {
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
    fn setDragText(&self, val: @String) {
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
impl Gauge95 for wxGauge95 {}
impl Gauge for wxGauge95 {}
impl Control for wxGauge95 {}
impl Window for wxGauge95 {}
impl EvtHandler for wxGauge95 {}
impl Object for wxGauge95 { fn handle(&self) -> *u8 { **self } }

impl wxGauge95 {
}

trait Gauge95 : Gauge {
}

struct wxGaugeMSW(*u8);
impl GaugeMSW for wxGaugeMSW {}
impl Gauge for wxGaugeMSW {}
impl Control for wxGaugeMSW {}
impl Window for wxGaugeMSW {}
impl EvtHandler for wxGaugeMSW {}
impl Object for wxGaugeMSW { fn handle(&self) -> *u8 { **self } }

impl wxGaugeMSW {
}

trait GaugeMSW : Gauge {
}

struct wxSlider95(*u8);
impl Slider95 for wxSlider95 {}
impl Slider for wxSlider95 {}
impl Control for wxSlider95 {}
impl Window for wxSlider95 {}
impl EvtHandler for wxSlider95 {}
impl Object for wxSlider95 { fn handle(&self) -> *u8 { **self } }

impl wxSlider95 {
}

trait Slider95 : Slider {
}

struct wxSliderMSW(*u8);
impl SliderMSW for wxSliderMSW {}
impl Slider for wxSliderMSW {}
impl Control for wxSliderMSW {}
impl Window for wxSliderMSW {}
impl EvtHandler for wxSliderMSW {}
impl Object for wxSliderMSW { fn handle(&self) -> *u8 { **self } }

impl wxSliderMSW {
}

trait SliderMSW : Slider {
}

struct wxcTreeItemData(*u8);
impl cTreeItemData for wxcTreeItemData {}
impl TreeItemData for wxcTreeItemData {}
impl ClientData for wxcTreeItemData { fn handle(&self) -> *u8 { **self } }

impl wxcTreeItemData {
    #[fixed_stack_segment]
    pub fn new(closure: @Closure) -> @cTreeItemData {
        unsafe { @wxcTreeItemData(wxcTreeItemData_Create(closure.handle())) as @cTreeItemData }
    }
}

trait cTreeItemData : TreeItemData {
    #[fixed_stack_segment]
    fn getClientClosure(&self) -> @Closure {
        unsafe { @wxClosure(wxcTreeItemData_GetClientClosure(self.handle())) as @Closure }
    }
    #[fixed_stack_segment]
    fn setClientClosure(&self, closure: @Closure) {
        unsafe { wxcTreeItemData_SetClientClosure(self.handle(), closure.handle()) }
    }
}

struct wxInputSink(*u8);
impl InputSink for wxInputSink {}
impl Thread for wxInputSink { fn handle(&self) -> *u8 { **self } }

impl wxInputSink {
    #[fixed_stack_segment]
    pub fn new(input: @InputStream, evtHandler: @EvtHandler, bufferLen: c_int) -> @InputSink {
        unsafe { @wxInputSink(wxInputSink_Create(input.handle(), evtHandler.handle(), bufferLen)) as @InputSink }
    }
}

trait InputSink : Thread {
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
impl InputSinkEvent for wxInputSinkEvent {}
impl Event for wxInputSinkEvent {}
impl Object for wxInputSinkEvent { fn handle(&self) -> *u8 { **self } }

impl wxInputSinkEvent {
}

trait InputSinkEvent : Event {
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
impl cHtmlEvent for wxcHtmlEvent {}
impl CommandEvent for wxcHtmlEvent {}
impl Event for wxcHtmlEvent {}
impl Object for wxcHtmlEvent { fn handle(&self) -> *u8 { **self } }

impl wxcHtmlEvent {
}

trait cHtmlEvent : CommandEvent {
    #[fixed_stack_segment]
    fn getMouseEvent(&self) -> @MouseEvent {
        unsafe { @wxMouseEvent(wxcHtmlEvent_GetMouseEvent(self.handle())) as @MouseEvent }
    }
    #[fixed_stack_segment]
    fn getHtmlCell(&self) -> @HtmlCell {
        unsafe { @wxHtmlCell(wxcHtmlEvent_GetHtmlCell(self.handle())) as @HtmlCell }
    }
    #[fixed_stack_segment]
    fn getHtmlCellId(&self) -> @String {
        unsafe { @wxString(wxcHtmlEvent_GetHtmlCellId(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getHref(&self) -> @String {
        unsafe { @wxString(wxcHtmlEvent_GetHref(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getTarget(&self) -> @String {
        unsafe { @wxString(wxcHtmlEvent_GetTarget(self.handle())) as @String }
    }
    #[fixed_stack_segment]
    fn getLogicalPosition(&self) -> @Point {
        unsafe { @wxPoint(wxcHtmlEvent_GetLogicalPosition(self.handle())) as @Point }
    }
}

struct wxcHtmlWindow(*u8);
impl cHtmlWindow for wxcHtmlWindow {}
impl HtmlWindow for wxcHtmlWindow {}
impl ScrolledWindow for wxcHtmlWindow {}
impl Panel for wxcHtmlWindow {}
impl Window for wxcHtmlWindow {}
impl EvtHandler for wxcHtmlWindow {}
impl Object for wxcHtmlWindow { fn handle(&self) -> *u8 { **self } }

impl wxcHtmlWindow {
    #[fixed_stack_segment]
    pub fn new(_prt: @Window, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: @String) -> @cHtmlWindow {
        unsafe { @wxcHtmlWindow(wxcHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle())) as @cHtmlWindow }
    }
}

trait cHtmlWindow : HtmlWindow {
}

struct wxGridCellTextEnterEditor(*u8);
impl GridCellTextEnterEditor for wxGridCellTextEnterEditor {}
impl GridCellTextEditor for wxGridCellTextEnterEditor {}
impl GridCellEditor for wxGridCellTextEnterEditor {}
impl GridCellWorker for wxGridCellTextEnterEditor { fn handle(&self) -> *u8 { **self } }

impl wxGridCellTextEnterEditor {
    #[fixed_stack_segment]
    pub fn ctor() -> @GridCellTextEnterEditor {
        unsafe { @wxGridCellTextEnterEditor(wxGridCellTextEnterEditor_Ctor()) as @GridCellTextEnterEditor }
    }
}

trait GridCellTextEnterEditor : GridCellTextEditor {
}

struct wxFileConfig(*u8);
impl FileConfig for wxFileConfig {}
impl ConfigBase for wxFileConfig { fn handle(&self) -> *u8 { **self } }

impl wxFileConfig {
    #[fixed_stack_segment]
    pub fn new(inp: @InputStream) -> @FileConfig {
        unsafe { @wxFileConfig(wxFileConfig_Create(inp.handle())) as @FileConfig }
    }
}

trait FileConfig : ConfigBase {
}

