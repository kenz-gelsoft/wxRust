use std::libc::*;
use _unsafe::*;
use base::*;

pub struct RustApp { ptr: *mut c_void }
impl TRustApp for RustApp {}
impl TApp for RustApp {}
impl TEvtHandler for RustApp {}
impl TObject for RustApp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustApp {
    pub fn from(ptr: *mut c_void) -> RustApp { RustApp { ptr: ptr } }
    pub fn null() -> RustApp { RustApp::from(0 as *mut c_void) }
    
    pub fn bell() {
        unsafe { ELJApp_Bell() }
    }
    pub fn newLogTarget() -> RustLog {
        unsafe { RustLog { ptr: ELJApp_CreateLogTarget() } }
    }
    pub fn dispatch() {
        unsafe { ELJApp_Dispatch() }
    }
    pub fn displaySize() -> Size {
        unsafe { Size { ptr: ELJApp_DisplaySize() } }
    }
    pub fn enableTooltips(_enable: c_int) {
        unsafe { ELJApp_EnableTooltips(_enable) }
    }
    pub fn enableTopLevelWindows(_enb: c_int) {
        unsafe { ELJApp_EnableTopLevelWindows(_enb) }
    }
    pub fn executeProcess<T: TProcess>(_cmd: &str, _snc: c_int, _prc: &T) -> c_int {
        let _cmd = wxT(_cmd);
        unsafe { ELJApp_ExecuteProcess(_cmd.ptr(), _snc, _prc.ptr()) }
    }
    pub fn exit() {
        unsafe { ELJApp_Exit() }
    }
    pub fn exitMainLoop() {
        unsafe { ELJApp_ExitMainLoop() }
    }
    pub fn findWindowById<T: TWindow>(_id: c_int, _prt: &T) -> *mut c_void {
        unsafe { ELJApp_FindWindowById(_id, _prt.ptr()) }
    }
    pub fn findWindowByLabel<T: TWindow>(_lbl: &str, _prt: &T) -> Window {
        let _lbl = wxT(_lbl);
        unsafe { Window { ptr: ELJApp_FindWindowByLabel(_lbl.ptr(), _prt.ptr()) } }
    }
    pub fn findWindowByName<T: TWindow>(_lbl: &str, _prt: &T) -> Window {
        let _lbl = wxT(_lbl);
        unsafe { Window { ptr: ELJApp_FindWindowByName(_lbl.ptr(), _prt.ptr()) } }
    }
    pub fn getApp() -> App {
        unsafe { App { ptr: ELJApp_GetApp() } }
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
    pub fn getTopWindow() -> Window {
        unsafe { Window { ptr: ELJApp_GetTopWindow() } }
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
    pub fn mousePosition() -> Point {
        unsafe { Point { ptr: ELJApp_MousePosition() } }
    }
    pub fn pending() -> c_int {
        unsafe { ELJApp_Pending() }
    }
    pub fn safeYield<T: TWindow>(_win: &T) -> c_int {
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
    pub fn setTopWindow<T: TWindow>(_wnd: &T) {
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
    pub fn initializeC<T: TClosure>(closure: &T, _argc: c_int, _argv: *mut *mut c_char) {
        unsafe { ELJApp_InitializeC(closure.ptr(), _argc, _argv) }
    }
    pub fn getIdleInterval() -> c_int {
        unsafe { ELJApp_GetIdleInterval() }
    }
    pub fn setIdleInterval(interval: c_int) {
        unsafe { ELJApp_SetIdleInterval(interval) }
    }
}

pub trait TRustApp : TApp {
}

pub struct RustArtProv { ptr: *mut c_void }
impl TRustArtProv for RustArtProv {}
impl TArtProvider for RustArtProv {}
impl TObject for RustArtProv { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustArtProv {
    pub fn from(ptr: *mut c_void) -> RustArtProv { RustArtProv { ptr: ptr } }
    pub fn null() -> RustArtProv { RustArtProv::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _clb: *mut c_void) -> RustArtProv {
        unsafe { RustArtProv { ptr: ELJArtProv_Create(_obj, _clb) } }
    }
}

pub trait TRustArtProv : TArtProvider {
    fn release(&self) {
        unsafe { ELJArtProv_Release(self.ptr()) }
    }
}

pub struct RustCommand { ptr: *mut c_void }
impl TRustCommand for RustCommand {}
impl TCommand for RustCommand {}
impl TObject for RustCommand { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustCommand {
    pub fn from(ptr: *mut c_void) -> RustCommand { RustCommand { ptr: ptr } }
    pub fn null() -> RustCommand { RustCommand::from(0 as *mut c_void) }
    
}

pub trait TRustCommand : TCommand {
}

pub struct RustDragDataObject { ptr: *mut c_void }
impl TRustDragDataObject for RustDragDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustDragDataObject {
    pub fn from(ptr: *mut c_void) -> RustDragDataObject { RustDragDataObject { ptr: ptr } }
    pub fn null() -> RustDragDataObject { RustDragDataObject::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fmt: &str, _func1: *mut c_void, _func2: *mut c_void, _func3: *mut c_void) -> RustDragDataObject {
        let _fmt = wxT(_fmt);
        unsafe { RustDragDataObject { ptr: ELJDragDataObject_Create(_obj, _fmt.ptr(), _func1, _func2, _func3) } }
    }
}

pub trait TRustDragDataObject {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { ELJDragDataObject_Delete(self.ptr()) }
    }
}

pub struct RustDropTarget { ptr: *mut c_void }
impl TRustDropTarget for RustDropTarget {}
impl TDropTarget for RustDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustDropTarget {
    pub fn from(ptr: *mut c_void) -> RustDropTarget { RustDropTarget { ptr: ptr } }
    pub fn null() -> RustDropTarget { RustDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void) -> RustDropTarget {
        unsafe { RustDropTarget { ptr: ELJDropTarget_Create(_obj) } }
    }
}

pub trait TRustDropTarget : TDropTarget {
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

pub struct RustFileDropTarget { ptr: *mut c_void }
impl TRustFileDropTarget for RustFileDropTarget {}
impl TFileDropTarget for RustFileDropTarget {}
impl TDropTarget for RustFileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustFileDropTarget {
    pub fn from(ptr: *mut c_void) -> RustFileDropTarget { RustFileDropTarget { ptr: ptr } }
    pub fn null() -> RustFileDropTarget { RustFileDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> RustFileDropTarget {
        unsafe { RustFileDropTarget { ptr: ELJFileDropTarget_Create(_obj, _func) } }
    }
}

pub trait TRustFileDropTarget : TFileDropTarget {
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

pub struct RustLog { ptr: *mut c_void }
impl TRustLog for RustLog {}
impl TLog for RustLog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustLog {
    pub fn from(ptr: *mut c_void) -> RustLog { RustLog { ptr: ptr } }
    pub fn null() -> RustLog { RustLog::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> RustLog {
        unsafe { RustLog { ptr: ELJLog_Create(_obj, _fnc) } }
    }
    pub fn getActiveTarget() -> *mut c_void {
        unsafe { ELJLog_GetActiveTarget() }
    }
}

pub trait TRustLog : TLog {
    fn enableLogging(&self, doIt: c_int) -> c_int {
        unsafe { ELJLog_EnableLogging(self.ptr(), doIt) }
    }
    fn isEnabled(&self) -> c_int {
        unsafe { ELJLog_IsEnabled(self.ptr()) }
    }
}

pub struct RustPreviewControlBar { ptr: *mut c_void }
impl TRustPreviewControlBar for RustPreviewControlBar {}
impl TPreviewControlBar for RustPreviewControlBar {}
impl TPanel for RustPreviewControlBar {}
impl TWindow for RustPreviewControlBar {}
impl TEvtHandler for RustPreviewControlBar {}
impl TObject for RustPreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPreviewControlBar {
    pub fn from(ptr: *mut c_void) -> RustPreviewControlBar { RustPreviewControlBar { ptr: ptr } }
    pub fn null() -> RustPreviewControlBar { RustPreviewControlBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(preview: *mut c_void, buttons: c_int, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> RustPreviewControlBar {
        unsafe { RustPreviewControlBar { ptr: ELJPreviewControlBar_Create(preview, buttons, parent.ptr(), title, x, y, w, h, style) } }
    }
}

pub trait TRustPreviewControlBar : TPreviewControlBar {
}

pub struct RustPreviewFrame { ptr: *mut c_void }
impl TRustPreviewFrame for RustPreviewFrame {}
impl TPreviewFrame for RustPreviewFrame {}
impl TFrame for RustPreviewFrame {}
impl TTopLevelWindow for RustPreviewFrame {}
impl TWindow for RustPreviewFrame {}
impl TEvtHandler for RustPreviewFrame {}
impl TObject for RustPreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPreviewFrame {
    pub fn from(ptr: *mut c_void) -> RustPreviewFrame { RustPreviewFrame { ptr: ptr } }
    pub fn null() -> RustPreviewFrame { RustPreviewFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_obj: *mut c_void, _init: *mut c_void, _create_canvas: *mut c_void, _create_toolbar: *mut c_void, preview: *mut c_void, parent: &T, title: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> RustPreviewFrame {
        unsafe { RustPreviewFrame { ptr: ELJPreviewFrame_Create(_obj, _init, _create_canvas, _create_toolbar, preview, parent.ptr(), title, x, y, w, h, style) } }
    }
}

pub trait TRustPreviewFrame : TPreviewFrame {
    fn getControlBar(&self) -> *mut c_void {
        unsafe { ELJPreviewFrame_GetControlBar(self.ptr()) }
    }
    fn getPreviewCanvas(&self) -> PreviewCanvas {
        unsafe { PreviewCanvas { ptr: ELJPreviewFrame_GetPreviewCanvas(self.ptr()) } }
    }
    fn getPrintPreview(&self) -> PrintPreview {
        unsafe { PrintPreview { ptr: ELJPreviewFrame_GetPrintPreview(self.ptr()) } }
    }
    fn setControlBar(&self, obj: *mut c_void) {
        unsafe { ELJPreviewFrame_SetControlBar(self.ptr(), obj) }
    }
    fn setPreviewCanvas<T: TPreviewCanvas>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPreviewCanvas(self.ptr(), obj.ptr()) }
    }
    fn setPrintPreview<T: TPrintPreview>(&self, obj: &T) {
        unsafe { ELJPreviewFrame_SetPrintPreview(self.ptr(), obj.ptr()) }
    }
}

pub struct RustTextDropTarget { ptr: *mut c_void }
impl TRustTextDropTarget for RustTextDropTarget {}
impl TTextDropTarget for RustTextDropTarget {}
impl TDropTarget for RustTextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustTextDropTarget {
    pub fn from(ptr: *mut c_void) -> RustTextDropTarget { RustTextDropTarget { ptr: ptr } }
    pub fn null() -> RustTextDropTarget { RustTextDropTarget::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _func: *mut c_void) -> RustTextDropTarget {
        unsafe { RustTextDropTarget { ptr: ELJTextDropTarget_Create(_obj, _func) } }
    }
}

pub trait TRustTextDropTarget : TTextDropTarget {
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

pub struct RustTextValidator { ptr: *mut c_void }
impl TRustTextValidator for RustTextValidator {}
impl TTextValidator for RustTextValidator {}
impl TValidator for RustTextValidator {}
impl TEvtHandler for RustTextValidator {}
impl TObject for RustTextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustTextValidator {
    pub fn from(ptr: *mut c_void) -> RustTextValidator { RustTextValidator { ptr: ptr } }
    pub fn null() -> RustTextValidator { RustTextValidator::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void, _txt: *mut c_void, _stl: c_int) -> RustTextValidator {
        unsafe { RustTextValidator { ptr: ELJTextValidator_Create(_obj, _fnc, _txt, _stl) } }
    }
}

pub trait TRustTextValidator : TTextValidator {
}

pub struct AcceleratorEntry { ptr: *mut c_void }
impl TAcceleratorEntry for AcceleratorEntry { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AcceleratorEntry {
    pub fn from(ptr: *mut c_void) -> AcceleratorEntry { AcceleratorEntry { ptr: ptr } }
    pub fn null() -> AcceleratorEntry { AcceleratorEntry::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int, keyCode: c_int, cmd: c_int) -> AcceleratorEntry {
        unsafe { AcceleratorEntry { ptr: wxAcceleratorEntry_Create(flags, keyCode, cmd) } }
    }
}

pub trait TAcceleratorEntry {
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

pub struct AcceleratorTable { ptr: *mut c_void }
impl TAcceleratorTable for AcceleratorTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AcceleratorTable {
    pub fn from(ptr: *mut c_void) -> AcceleratorTable { AcceleratorTable { ptr: ptr } }
    pub fn null() -> AcceleratorTable { AcceleratorTable::from(0 as *mut c_void) }
    
    pub fn new(n: c_int, entries: *mut c_void) -> AcceleratorTable {
        unsafe { AcceleratorTable { ptr: wxAcceleratorTable_Create(n, entries) } }
    }
}

pub trait TAcceleratorTable {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxAcceleratorTable_Delete(self.ptr()) }
    }
}

pub struct ActivateEvent { ptr: *mut c_void }
impl TActivateEvent for ActivateEvent {}
impl TEvent for ActivateEvent {}
impl TObject for ActivateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ActivateEvent {
    pub fn from(ptr: *mut c_void) -> ActivateEvent { ActivateEvent { ptr: ptr } }
    pub fn null() -> ActivateEvent { ActivateEvent::from(0 as *mut c_void) }
    
}

pub trait TActivateEvent : TEvent {
    fn getActive(&self) -> c_int {
        unsafe { wxActivateEvent_GetActive(self.ptr()) }
    }
}

pub struct App { ptr: *mut c_void }
impl TApp for App {}
impl TEvtHandler for App {}
impl TObject for App { fn ptr(&self) -> *mut c_void { self.ptr } }

impl App {
    pub fn from(ptr: *mut c_void) -> App { App { ptr: ptr } }
    pub fn null() -> App { App::from(0 as *mut c_void) }
    
}

pub trait TApp : TEvtHandler {
}

pub struct ArtProvider { ptr: *mut c_void }
impl TArtProvider for ArtProvider {}
impl TObject for ArtProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ArtProvider {
    pub fn from(ptr: *mut c_void) -> ArtProvider { ArtProvider { ptr: ptr } }
    pub fn null() -> ArtProvider { ArtProvider::from(0 as *mut c_void) }
    
}

pub trait TArtProvider : TObject {
}

pub struct AutoBufferedPaintDC { ptr: *mut c_void }
impl TAutoBufferedPaintDC for AutoBufferedPaintDC {}
impl TDC for AutoBufferedPaintDC {}
impl TObject for AutoBufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AutoBufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> AutoBufferedPaintDC { AutoBufferedPaintDC { ptr: ptr } }
    pub fn null() -> AutoBufferedPaintDC { AutoBufferedPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(window: &T) -> AutoBufferedPaintDC {
        unsafe { AutoBufferedPaintDC { ptr: wxAutoBufferedPaintDC_Create(window.ptr()) } }
    }
}

pub trait TAutoBufferedPaintDC : TDC {
}

pub struct AutomationObject { ptr: *mut c_void }
impl TAutomationObject for AutomationObject {}
impl TObject for AutomationObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl AutomationObject {
    pub fn from(ptr: *mut c_void) -> AutomationObject { AutomationObject { ptr: ptr } }
    pub fn null() -> AutomationObject { AutomationObject::from(0 as *mut c_void) }
    
}

pub trait TAutomationObject : TObject {
}

pub struct Bitmap { ptr: *mut c_void }
impl TBitmap for Bitmap {}
impl TGDIObject for Bitmap {}
impl TObject for Bitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Bitmap {
    pub fn from(ptr: *mut c_void) -> Bitmap { Bitmap { ptr: ptr } }
    pub fn null() -> Bitmap { Bitmap::from(0 as *mut c_void) }
    
    pub fn addHandler<T: TEvtHandler>(handler: &T) {
        unsafe { wxBitmap_AddHandler(handler.ptr()) }
    }
    pub fn cleanUpHandlers() {
        unsafe { wxBitmap_CleanUpHandlers() }
    }
    pub fn new(_data: *mut c_void, _type: c_int, _width: c_int, _height: c_int, _depth: c_int) -> Bitmap {
        unsafe { Bitmap { ptr: wxBitmap_Create(_data, _type, _width, _height, _depth) } }
    }
    pub fn newDefault() -> Bitmap {
        unsafe { Bitmap { ptr: wxBitmap_CreateDefault() } }
    }
    pub fn newEmpty(_width: c_int, _height: c_int, _depth: c_int) -> Bitmap {
        unsafe { Bitmap { ptr: wxBitmap_CreateEmpty(_width, _height, _depth) } }
    }
    pub fn newLoad(name: &str, type_: c_int) -> Bitmap {
        let name = wxT(name);
        unsafe { Bitmap { ptr: wxBitmap_CreateLoad(name.ptr(), type_) } }
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
    pub fn insertHandler<T: TEvtHandler>(handler: &T) {
        unsafe { wxBitmap_InsertHandler(handler.ptr()) }
    }
    pub fn removeHandler(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_RemoveHandler(name.ptr()) }
    }
    pub fn newFromImage<T: TImage>(image: &T, depth: c_int) -> Bitmap {
        unsafe { Bitmap { ptr: wxBitmap_CreateFromImage(image.ptr(), depth) } }
    }
}

pub trait TBitmap : TGDIObject {
    fn newFromXPM(&self) -> Bitmap {
        unsafe { Bitmap { ptr: wxBitmap_CreateFromXPM(self.ptr()) } }
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
        unsafe { Mask { ptr: wxBitmap_GetMask(self.ptr()) } }
    }
    fn getSubBitmap<T: TBitmap>(&self, x: c_int, y: c_int, w: c_int, h: c_int, _ref: &T) {
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
    fn saveFile<T: TPalette>(&self, name: &str, type_: c_int, cmap: &T) -> c_int {
        let name = wxT(name);
        unsafe { wxBitmap_SaveFile(self.ptr(), name.ptr(), type_, cmap.ptr()) }
    }
    fn setDepth(&self, d: c_int) {
        unsafe { wxBitmap_SetDepth(self.ptr(), d) }
    }
    fn setHeight(&self, h: c_int) {
        unsafe { wxBitmap_SetHeight(self.ptr(), h) }
    }
    fn setMask<T: TMask>(&self, mask: &T) {
        unsafe { wxBitmap_SetMask(self.ptr(), mask.ptr()) }
    }
    fn setWidth(&self, w: c_int) {
        unsafe { wxBitmap_SetWidth(self.ptr(), w) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxBitmap_IsStatic(self.ptr()) }
    }
}

pub struct BitmapButton { ptr: *mut c_void }
impl TBitmapButton for BitmapButton {}
impl TButton for BitmapButton {}
impl TControl for BitmapButton {}
impl TWindow for BitmapButton {}
impl TEvtHandler for BitmapButton {}
impl TObject for BitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapButton {
    pub fn from(ptr: *mut c_void) -> BitmapButton { BitmapButton { ptr: ptr } }
    pub fn null() -> BitmapButton { BitmapButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TBitmap>(_prt: &T, _id: c_int, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> BitmapButton {
        unsafe { BitmapButton { ptr: wxBitmapButton_Create(_prt.ptr(), _id, _bmp.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TBitmapButton : TButton {
    fn getBitmapDisabled<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapDisabled(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapFocus<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapFocus(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapLabel<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapLabel(self.ptr(), _ref.ptr()) }
    }
    fn getBitmapSelected<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxBitmapButton_GetBitmapSelected(self.ptr(), _ref.ptr()) }
    }
    fn getMarginX(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginX(self.ptr()) }
    }
    fn getMarginY(&self) -> c_int {
        unsafe { wxBitmapButton_GetMarginY(self.ptr()) }
    }
    fn setBitmapDisabled<T: TBitmap>(&self, disabled: &T) {
        unsafe { wxBitmapButton_SetBitmapDisabled(self.ptr(), disabled.ptr()) }
    }
    fn setBitmapFocus<T: TBitmap>(&self, focus: &T) {
        unsafe { wxBitmapButton_SetBitmapFocus(self.ptr(), focus.ptr()) }
    }
    fn setBitmapLabel<T: TBitmap>(&self, bitmap: &T) {
        unsafe { wxBitmapButton_SetBitmapLabel(self.ptr(), bitmap.ptr()) }
    }
    fn setBitmapSelected<T: TBitmap>(&self, sel: &T) {
        unsafe { wxBitmapButton_SetBitmapSelected(self.ptr(), sel.ptr()) }
    }
    fn setMargins(&self, x: c_int, y: c_int) {
        unsafe { wxBitmapButton_SetMargins(self.ptr(), x, y) }
    }
}

pub struct BitmapToggleButton { ptr: *mut c_void }
impl TBitmapToggleButton for BitmapToggleButton {}
impl TToggleButton for BitmapToggleButton {}
impl TControl for BitmapToggleButton {}
impl TWindow for BitmapToggleButton {}
impl TEvtHandler for BitmapToggleButton {}
impl TObject for BitmapToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapToggleButton {
    pub fn from(ptr: *mut c_void) -> BitmapToggleButton { BitmapToggleButton { ptr: ptr } }
    pub fn null() -> BitmapToggleButton { BitmapToggleButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TBitmap>(parent: &T, id: c_int, _bmp: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> BitmapToggleButton {
        unsafe { BitmapToggleButton { ptr: wxBitmapToggleButton_Create(parent.ptr(), id, _bmp.ptr(), x, y, w, h, style) } }
    }
}

pub trait TBitmapToggleButton : TToggleButton {
    fn setBitmapLabel<T: TBitmap>(&self, _bmp: &T) {
        unsafe { wxBitmapToggleButton_SetBitmapLabel(self.ptr(), _bmp.ptr()) }
    }
}

pub struct BitmapDataObject { ptr: *mut c_void }
impl TBitmapDataObject for BitmapDataObject {}
impl TDataObjectSimple for BitmapDataObject {}
impl TDataObject for BitmapDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapDataObject {
    pub fn from(ptr: *mut c_void) -> BitmapDataObject { BitmapDataObject { ptr: ptr } }
    pub fn null() -> BitmapDataObject { BitmapDataObject::from(0 as *mut c_void) }
    
}

pub trait TBitmapDataObject : TDataObjectSimple {
}

pub struct BitmapHandler { ptr: *mut c_void }
impl TBitmapHandler for BitmapHandler {}
impl TObject for BitmapHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BitmapHandler {
    pub fn from(ptr: *mut c_void) -> BitmapHandler { BitmapHandler { ptr: ptr } }
    pub fn null() -> BitmapHandler { BitmapHandler::from(0 as *mut c_void) }
    
}

pub trait TBitmapHandler : TObject {
}

pub struct BoxSizer { ptr: *mut c_void }
impl TBoxSizer for BoxSizer {}
impl TSizer for BoxSizer {}
impl TObject for BoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BoxSizer {
    pub fn from(ptr: *mut c_void) -> BoxSizer { BoxSizer { ptr: ptr } }
    pub fn null() -> BoxSizer { BoxSizer::from(0 as *mut c_void) }
    
    pub fn new(orient: c_int) -> BoxSizer {
        unsafe { BoxSizer { ptr: wxBoxSizer_Create(orient) } }
    }
}

pub trait TBoxSizer : TSizer {
    fn getOrientation(&self) -> c_int {
        unsafe { wxBoxSizer_GetOrientation(self.ptr()) }
    }
}

pub struct Brush { ptr: *mut c_void }
impl TBrush for Brush {}
impl TGDIObject for Brush {}
impl TObject for Brush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Brush {
    pub fn from(ptr: *mut c_void) -> Brush { Brush { ptr: ptr } }
    pub fn null() -> Brush { Brush::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Brush {
        unsafe { Brush { ptr: wxBrush_CreateDefault() } }
    }
    pub fn newFromBitmap<T: TBitmap>(bitmap: &T) -> Brush {
        unsafe { Brush { ptr: wxBrush_CreateFromBitmap(bitmap.ptr()) } }
    }
    pub fn newFromColour<T: TColour>(col: &T, style: c_int) -> Brush {
        unsafe { Brush { ptr: wxBrush_CreateFromColour(col.ptr(), style) } }
    }
    pub fn newFromStock(id: c_int) -> Brush {
        unsafe { Brush { ptr: wxBrush_CreateFromStock(id) } }
    }
}

pub trait TBrush : TGDIObject {
    fn assign<T: TBrush>(&self, brush: &T) {
        unsafe { wxBrush_Assign(self.ptr(), brush.ptr()) }
    }
    fn getColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxBrush_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getStipple<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxBrush_GetStipple(self.ptr(), _ref.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxBrush_GetStyle(self.ptr()) }
    }
    fn isEqual<T: TBrush>(&self, brush: &T) -> c_int {
        unsafe { wxBrush_IsEqual(self.ptr(), brush.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxBrush_IsOk(self.ptr()) }
    }
    fn setColour<T: TColour>(&self, col: &T) {
        unsafe { wxBrush_SetColour(self.ptr(), col.ptr()) }
    }
    fn setColourSingle(&self, r: int8_t, g: int8_t, b: int8_t) {
        unsafe { wxBrush_SetColourSingle(self.ptr(), r, g, b) }
    }
    fn setStipple<T: TBitmap>(&self, stipple: &T) {
        unsafe { wxBrush_SetStipple(self.ptr(), stipple.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxBrush_SetStyle(self.ptr(), style) }
    }
    fn isStatic(&self) -> c_int {
        unsafe { wxBrush_IsStatic(self.ptr()) }
    }
}

pub struct BrushList { ptr: *mut c_void }
impl TBrushList for BrushList {}
impl TList for BrushList {}
impl TObject for BrushList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BrushList {
    pub fn from(ptr: *mut c_void) -> BrushList { BrushList { ptr: ptr } }
    pub fn null() -> BrushList { BrushList::from(0 as *mut c_void) }
    
}

pub trait TBrushList : TList {
}

pub struct BufferedDC { ptr: *mut c_void }
impl TBufferedDC for BufferedDC {}
impl TDC for BufferedDC {}
impl TObject for BufferedDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedDC {
    pub fn from(ptr: *mut c_void) -> BufferedDC { BufferedDC { ptr: ptr } }
    pub fn null() -> BufferedDC { BufferedDC::from(0 as *mut c_void) }
    
    pub fn newByDCAndSize<T: TDC>(dc: &T, width: c_int, hight: c_int, style: c_int) -> BufferedDC {
        unsafe { BufferedDC { ptr: wxBufferedDC_CreateByDCAndSize(dc.ptr(), width, hight, style) } }
    }
    pub fn newByDCAndBitmap<T: TDC, U: TBitmap>(dc: &T, bitmap: &U, style: c_int) -> BufferedDC {
        unsafe { BufferedDC { ptr: wxBufferedDC_CreateByDCAndBitmap(dc.ptr(), bitmap.ptr(), style) } }
    }
}

pub trait TBufferedDC : TDC {
}

pub struct BufferedPaintDC { ptr: *mut c_void }
impl TBufferedPaintDC for BufferedPaintDC {}
impl TDC for BufferedPaintDC {}
impl TObject for BufferedPaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BufferedPaintDC {
    pub fn from(ptr: *mut c_void) -> BufferedPaintDC { BufferedPaintDC { ptr: ptr } }
    pub fn null() -> BufferedPaintDC { BufferedPaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(window: &T, style: c_int) -> BufferedPaintDC {
        unsafe { BufferedPaintDC { ptr: wxBufferedPaintDC_Create(window.ptr(), style) } }
    }
    pub fn newWithBitmap<T: TWindow, U: TBitmap>(window: &T, bitmap: &U, style: c_int) -> BufferedPaintDC {
        unsafe { BufferedPaintDC { ptr: wxBufferedPaintDC_CreateWithBitmap(window.ptr(), bitmap.ptr(), style) } }
    }
}

pub trait TBufferedPaintDC : TDC {
}

pub struct BusyCursor { ptr: *mut c_void }
impl TBusyCursor for BusyCursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BusyCursor {
    pub fn from(ptr: *mut c_void) -> BusyCursor { BusyCursor { ptr: ptr } }
    pub fn null() -> BusyCursor { BusyCursor::from(0 as *mut c_void) }
    
    pub fn new() -> BusyCursor {
        unsafe { BusyCursor { ptr: wxBusyCursor_Create() } }
    }
}

pub trait TBusyCursor {
    fn ptr(&self) -> *mut c_void;
    
    fn newWithCursor(&self) -> *mut c_void {
        unsafe { wxBusyCursor_CreateWithCursor(self.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxBusyCursor_Delete(self.ptr()) }
    }
}

pub struct BusyInfo { ptr: *mut c_void }
impl TBusyInfo for BusyInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl BusyInfo {
    pub fn from(ptr: *mut c_void) -> BusyInfo { BusyInfo { ptr: ptr } }
    pub fn null() -> BusyInfo { BusyInfo::from(0 as *mut c_void) }
    
    pub fn new(_txt: &str) -> BusyInfo {
        let _txt = wxT(_txt);
        unsafe { BusyInfo { ptr: wxBusyInfo_Create(_txt.ptr()) } }
    }
}

pub trait TBusyInfo {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxBusyInfo_Delete(self.ptr()) }
    }
}

pub struct Button { ptr: *mut c_void }
impl TButton for Button {}
impl TControl for Button {}
impl TWindow for Button {}
impl TEvtHandler for Button {}
impl TObject for Button { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Button {
    pub fn from(ptr: *mut c_void) -> Button { Button { ptr: ptr } }
    pub fn null() -> Button { Button::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Button {
        let _txt = wxT(_txt);
        unsafe { Button { ptr: wxButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TButton : TControl {
    fn setDefault(&self) {
        unsafe { wxButton_SetDefault(self.ptr()) }
    }
}

pub struct Caret { ptr: *mut c_void }
impl TCaret for Caret { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Caret {
    pub fn from(ptr: *mut c_void) -> Caret { Caret { ptr: ptr } }
    pub fn null() -> Caret { Caret::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_wnd: &T, _wth: c_int, _hgt: c_int) -> Caret {
        unsafe { Caret { ptr: wxCaret_Create(_wnd.ptr(), _wth, _hgt) } }
    }
    pub fn getBlinkTime() -> c_int {
        unsafe { wxCaret_GetBlinkTime() }
    }
    pub fn setBlinkTime(milliseconds: c_int) {
        unsafe { wxCaret_SetBlinkTime(milliseconds) }
    }
}

pub trait TCaret {
    fn ptr(&self) -> *mut c_void;
    
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxCaret_GetPosition(self.ptr()) } }
    }
    fn getSize(&self) -> Size {
        unsafe { Size { ptr: wxCaret_GetSize(self.ptr()) } }
    }
    fn getWindow(&self) -> Window {
        unsafe { Window { ptr: wxCaret_GetWindow(self.ptr()) } }
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

pub struct CheckBox { ptr: *mut c_void }
impl TCheckBox for CheckBox {}
impl TControl for CheckBox {}
impl TWindow for CheckBox {}
impl TEvtHandler for CheckBox {}
impl TObject for CheckBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CheckBox {
    pub fn from(ptr: *mut c_void) -> CheckBox { CheckBox { ptr: ptr } }
    pub fn null() -> CheckBox { CheckBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> CheckBox {
        let _txt = wxT(_txt);
        unsafe { CheckBox { ptr: wxCheckBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TCheckBox : TControl {
    fn getValue(&self) -> c_int {
        unsafe { wxCheckBox_GetValue(self.ptr()) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxCheckBox_SetValue(self.ptr(), value) }
    }
}

pub struct CheckListBox { ptr: *mut c_void }
impl TCheckListBox for CheckListBox {}
impl TListBox for CheckListBox {}
impl TControl for CheckListBox {}
impl TWindow for CheckListBox {}
impl TEvtHandler for CheckListBox {}
impl TObject for CheckListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CheckListBox {
    pub fn from(ptr: *mut c_void) -> CheckListBox { CheckListBox { ptr: ptr } }
    pub fn null() -> CheckListBox { CheckListBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> CheckListBox {
        unsafe { CheckListBox { ptr: wxCheckListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TCheckListBox : TListBox {
    fn check(&self, item: c_int, check: c_int) {
        unsafe { wxCheckListBox_Check(self.ptr(), item, check) }
    }
    fn isChecked(&self, item: c_int) -> c_int {
        unsafe { wxCheckListBox_IsChecked(self.ptr(), item) }
    }
}

pub struct Choice { ptr: *mut c_void }
impl TChoice for Choice {}
impl TControl for Choice {}
impl TWindow for Choice {}
impl TEvtHandler for Choice {}
impl TObject for Choice { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Choice {
    pub fn from(ptr: *mut c_void) -> Choice { Choice { ptr: ptr } }
    pub fn null() -> Choice { Choice::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> Choice {
        unsafe { Choice { ptr: wxChoice_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TChoice : TControl {
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

pub struct ClientDC { ptr: *mut c_void }
impl TClientDC for ClientDC {}
impl TWindowDC for ClientDC {}
impl TDC for ClientDC {}
impl TObject for ClientDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ClientDC {
    pub fn from(ptr: *mut c_void) -> ClientDC { ClientDC { ptr: ptr } }
    pub fn null() -> ClientDC { ClientDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(win: &T) -> ClientDC {
        unsafe { ClientDC { ptr: wxClientDC_Create(win.ptr()) } }
    }
}

pub trait TClientDC : TWindowDC {
}

pub struct Clipboard { ptr: *mut c_void }
impl TClipboard for Clipboard {}
impl TObject for Clipboard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Clipboard {
    pub fn from(ptr: *mut c_void) -> Clipboard { Clipboard { ptr: ptr } }
    pub fn null() -> Clipboard { Clipboard::from(0 as *mut c_void) }
    
    pub fn new() -> Clipboard {
        unsafe { Clipboard { ptr: wxClipboard_Create() } }
    }
}

pub trait TClipboard : TObject {
    fn addData<T: TDataObject>(&self, data: &T) -> c_int {
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
    fn getData<T: TDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_GetData(self.ptr(), data.ptr()) }
    }
    fn isOpened(&self) -> c_int {
        unsafe { wxClipboard_IsOpened(self.ptr()) }
    }
    fn isSupported<T: TDataFormat>(&self, format: &T) -> c_int {
        unsafe { wxClipboard_IsSupported(self.ptr(), format.ptr()) }
    }
    fn open(&self) -> c_int {
        unsafe { wxClipboard_Open(self.ptr()) }
    }
    fn setData<T: TDataObject>(&self, data: &T) -> c_int {
        unsafe { wxClipboard_SetData(self.ptr(), data.ptr()) }
    }
    fn usePrimarySelection(&self, primary: c_int) {
        unsafe { wxClipboard_UsePrimarySelection(self.ptr(), primary) }
    }
}

pub struct CloseEvent { ptr: *mut c_void }
impl TCloseEvent for CloseEvent {}
impl TEvent for CloseEvent {}
impl TObject for CloseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CloseEvent {
    pub fn from(ptr: *mut c_void) -> CloseEvent { CloseEvent { ptr: ptr } }
    pub fn null() -> CloseEvent { CloseEvent::from(0 as *mut c_void) }
    
}

pub trait TCloseEvent : TEvent {
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

pub struct Colour { ptr: *mut c_void }
impl TColour for Colour {}
impl TObject for Colour { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Colour {
    pub fn from(ptr: *mut c_void) -> Colour { Colour { ptr: ptr } }
    pub fn null() -> Colour { Colour::from(0 as *mut c_void) }
    
    pub fn newByName(_name: &str) -> Colour {
        let _name = wxT(_name);
        unsafe { Colour { ptr: wxColour_CreateByName(_name.ptr()) } }
    }
    pub fn newEmpty() -> Colour {
        unsafe { Colour { ptr: wxColour_CreateEmpty() } }
    }
    pub fn newFromStock(id: c_int) -> Colour {
        unsafe { Colour { ptr: wxColour_CreateFromStock(id) } }
    }
    pub fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> Colour {
        unsafe { Colour { ptr: wxColour_CreateRGB(_red, _green, _blue, _alpha) } }
    }
    pub fn validName(_name: *mut c_void) -> c_int {
        unsafe { wxColour_ValidName(_name) }
    }
    pub fn newFromInt(rgb: c_int) -> Colour {
        unsafe { Colour { ptr: wxColour_CreateFromInt(rgb) } }
    }
    pub fn newFromUnsignedInt(rgba: uint32_t) -> Colour {
        unsafe { Colour { ptr: wxColour_CreateFromUnsignedInt(rgba) } }
    }
}

pub trait TColour : TObject {
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

pub struct ColourData { ptr: *mut c_void }
impl TColourData for ColourData {}
impl TObject for ColourData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ColourData {
    pub fn from(ptr: *mut c_void) -> ColourData { ColourData { ptr: ptr } }
    pub fn null() -> ColourData { ColourData::from(0 as *mut c_void) }
    
    pub fn new() -> ColourData {
        unsafe { ColourData { ptr: wxColourData_Create() } }
    }
}

pub trait TColourData : TObject {
    fn getChooseFull(&self) -> c_int {
        unsafe { wxColourData_GetChooseFull(self.ptr()) }
    }
    fn getColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxColourData_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getCustomColour<T: TColour>(&self, i: c_int, _ref: &T) {
        unsafe { wxColourData_GetCustomColour(self.ptr(), i, _ref.ptr()) }
    }
    fn setChooseFull(&self, flag: c_int) {
        unsafe { wxColourData_SetChooseFull(self.ptr(), flag) }
    }
    fn setColour<T: TColour>(&self, colour: &T) {
        unsafe { wxColourData_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setCustomColour<T: TColour>(&self, i: c_int, colour: &T) {
        unsafe { wxColourData_SetCustomColour(self.ptr(), i, colour.ptr()) }
    }
}

pub struct ColourDatabase { ptr: *mut c_void }
impl TColourDatabase for ColourDatabase {}
impl TList for ColourDatabase {}
impl TObject for ColourDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ColourDatabase {
    pub fn from(ptr: *mut c_void) -> ColourDatabase { ColourDatabase { ptr: ptr } }
    pub fn null() -> ColourDatabase { ColourDatabase::from(0 as *mut c_void) }
    
}

pub trait TColourDatabase : TList {
}

pub struct ColourDialog { ptr: *mut c_void }
impl TColourDialog for ColourDialog {}
impl TDialog for ColourDialog {}
impl TTopLevelWindow for ColourDialog {}
impl TWindow for ColourDialog {}
impl TEvtHandler for ColourDialog {}
impl TObject for ColourDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ColourDialog {
    pub fn from(ptr: *mut c_void) -> ColourDialog { ColourDialog { ptr: ptr } }
    pub fn null() -> ColourDialog { ColourDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TColourData>(_prt: &T, col: &U) -> ColourDialog {
        unsafe { ColourDialog { ptr: wxColourDialog_Create(_prt.ptr(), col.ptr()) } }
    }
}

pub trait TColourDialog : TDialog {
    fn getColourData<T: TColourData>(&self, _ref: &T) {
        unsafe { wxColourDialog_GetColourData(self.ptr(), _ref.ptr()) }
    }
}

pub struct ComboBox { ptr: *mut c_void }
impl TComboBox for ComboBox {}
impl TChoice for ComboBox {}
impl TControl for ComboBox {}
impl TWindow for ComboBox {}
impl TEvtHandler for ComboBox {}
impl TObject for ComboBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ComboBox {
    pub fn from(ptr: *mut c_void) -> ComboBox { ComboBox { ptr: ptr } }
    pub fn null() -> ComboBox { ComboBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> ComboBox {
        let _txt = wxT(_txt);
        unsafe { ComboBox { ptr: wxComboBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TComboBox : TChoice {
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

pub struct Command { ptr: *mut c_void }
impl TCommand for Command {}
impl TObject for Command { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Command {
    pub fn from(ptr: *mut c_void) -> Command { Command { ptr: ptr } }
    pub fn null() -> Command { Command::from(0 as *mut c_void) }
    
}

pub trait TCommand : TObject {
}

pub struct CommandEvent { ptr: *mut c_void }
impl TCommandEvent for CommandEvent {}
impl TEvent for CommandEvent {}
impl TObject for CommandEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CommandEvent {
    pub fn from(ptr: *mut c_void) -> CommandEvent { CommandEvent { ptr: ptr } }
    pub fn null() -> CommandEvent { CommandEvent::from(0 as *mut c_void) }
    
    pub fn new(_typ: c_int, _id: c_int) -> CommandEvent {
        unsafe { CommandEvent { ptr: wxCommandEvent_Create(_typ, _id) } }
    }
}

pub trait TCommandEvent : TEvent {
    fn getClientData(&self) -> ClientData {
        unsafe { ClientData { ptr: wxCommandEvent_GetClientData(self.ptr()) } }
    }
    fn getClientObject(&self) -> ClientData {
        unsafe { ClientData { ptr: wxCommandEvent_GetClientObject(self.ptr()) } }
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
    fn setClientData<T: TClientData>(&self, clientData: &T) {
        unsafe { wxCommandEvent_SetClientData(self.ptr(), clientData.ptr()) }
    }
    fn setClientObject<T: TClientData>(&self, clientObject: &T) {
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

pub struct CommandProcessor { ptr: *mut c_void }
impl TCommandProcessor for CommandProcessor {}
impl TObject for CommandProcessor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CommandProcessor {
    pub fn from(ptr: *mut c_void) -> CommandProcessor { CommandProcessor { ptr: ptr } }
    pub fn null() -> CommandProcessor { CommandProcessor::from(0 as *mut c_void) }
    
}

pub trait TCommandProcessor : TObject {
}

pub struct ContextHelp { ptr: *mut c_void }
impl TContextHelp for ContextHelp {}
impl TObject for ContextHelp { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ContextHelp {
    pub fn from(ptr: *mut c_void) -> ContextHelp { ContextHelp { ptr: ptr } }
    pub fn null() -> ContextHelp { ContextHelp::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(win: &T, beginHelp: c_int) -> ContextHelp {
        unsafe { ContextHelp { ptr: wxContextHelp_Create(win.ptr(), beginHelp) } }
    }
}

pub trait TContextHelp : TObject {
    fn beginContextHelp<T: TWindow>(&self, win: &T) -> c_int {
        unsafe { wxContextHelp_BeginContextHelp(self.ptr(), win.ptr()) }
    }
    fn endContextHelp(&self) -> c_int {
        unsafe { wxContextHelp_EndContextHelp(self.ptr()) }
    }
}

pub struct ContextHelpButton { ptr: *mut c_void }
impl TContextHelpButton for ContextHelpButton {}
impl TBitmapButton for ContextHelpButton {}
impl TButton for ContextHelpButton {}
impl TControl for ContextHelpButton {}
impl TWindow for ContextHelpButton {}
impl TEvtHandler for ContextHelpButton {}
impl TObject for ContextHelpButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ContextHelpButton {
    pub fn from(ptr: *mut c_void) -> ContextHelpButton { ContextHelpButton { ptr: ptr } }
    pub fn null() -> ContextHelpButton { ContextHelpButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(parent: &T, id: c_int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> ContextHelpButton {
        unsafe { ContextHelpButton { ptr: wxContextHelpButton_Create(parent.ptr(), id, x, y, w, h, style) } }
    }
}

pub trait TContextHelpButton : TBitmapButton {
}

pub struct Control { ptr: *mut c_void }
impl TControl for Control {}
impl TWindow for Control {}
impl TEvtHandler for Control {}
impl TObject for Control { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Control {
    pub fn from(ptr: *mut c_void) -> Control { Control { ptr: ptr } }
    pub fn null() -> Control { Control::from(0 as *mut c_void) }
    
}

pub trait TControl : TWindow {
    fn command<T: TEvent>(&self, event: &T) {
        unsafe { wxControl_Command(self.ptr(), event.ptr()) }
    }
}

pub struct Cursor { ptr: *mut c_void }
impl TCursor for Cursor {}
impl TBitmap for Cursor {}
impl TGDIObject for Cursor {}
impl TObject for Cursor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Cursor {
    pub fn from(ptr: *mut c_void) -> Cursor { Cursor { ptr: ptr } }
    pub fn null() -> Cursor { Cursor::from(0 as *mut c_void) }
    
}

pub trait TCursor : TBitmap {
}

pub struct CustomDataObject { ptr: *mut c_void }
impl TCustomDataObject for CustomDataObject {}
impl TDataObjectSimple for CustomDataObject {}
impl TDataObject for CustomDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CustomDataObject {
    pub fn from(ptr: *mut c_void) -> CustomDataObject { CustomDataObject { ptr: ptr } }
    pub fn null() -> CustomDataObject { CustomDataObject::from(0 as *mut c_void) }
    
}

pub trait TCustomDataObject : TDataObjectSimple {
}

pub struct DC { ptr: *mut c_void }
impl TDC for DC {}
impl TObject for DC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DC {
    pub fn from(ptr: *mut c_void) -> DC { DC { ptr: ptr } }
    pub fn null() -> DC { DC::from(0 as *mut c_void) }
    
}

pub trait TDC : TObject {
    fn blit<T: TDC>(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: &T, xsrc: c_int, ysrc: c_int, rop: c_int, useMask: c_int) -> c_int {
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
    fn drawBitmap<T: TBitmap>(&self, bmp: &T, x: c_int, y: c_int, useMask: c_int) {
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
    fn drawIcon<T: TIcon>(&self, icon: &T, x: c_int, y: c_int) {
        unsafe { wxDC_DrawIcon(self.ptr(), icon.ptr(), x, y) }
    }
    fn drawLabel(&self, str: &str, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) {
        let str = wxT(str);
        unsafe { wxDC_DrawLabel(self.ptr(), str.ptr(), x, y, w, h, align, indexAccel) }
    }
    fn drawLabelBitmap<T: TBitmap>(&self, str: &str, bmp: &T, x: c_int, y: c_int, w: c_int, h: c_int, align: c_int, indexAccel: c_int) -> Rect {
        let str = wxT(str);
        unsafe { Rect { ptr: wxDC_DrawLabelBitmap(self.ptr(), str.ptr(), bmp.ptr(), x, y, w, h, align, indexAccel) } }
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
    fn floodFill<T: TColour>(&self, x: c_int, y: c_int, col: &T, style: c_int) {
        unsafe { wxDC_FloodFill(self.ptr(), x, y, col.ptr(), style) }
    }
    fn getBackground<T: TBrush>(&self, _ref: &T) {
        unsafe { wxDC_GetBackground(self.ptr(), _ref.ptr()) }
    }
    fn getBackgroundMode(&self) -> c_int {
        unsafe { wxDC_GetBackgroundMode(self.ptr()) }
    }
    fn getBrush<T: TBrush>(&self, _ref: &T) {
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
    fn getFont<T: TFont>(&self, _ref: &T) {
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
        unsafe { Size { ptr: wxDC_GetPPI(self.ptr()) } }
    }
    fn getPen<T: TPen>(&self, _ref: &T) {
        unsafe { wxDC_GetPen(self.ptr(), _ref.ptr()) }
    }
    fn getPixel<T: TColour>(&self, x: c_int, y: c_int, col: &T) -> c_int {
        unsafe { wxDC_GetPixel(self.ptr(), x, y, col.ptr()) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size { ptr: wxDC_GetSize(self.ptr()) } }
    }
    fn getSizeMM(&self) -> Size {
        unsafe { Size { ptr: wxDC_GetSizeMM(self.ptr()) } }
    }
    fn getTextBackground<T: TColour>(&self, _ref: &T) {
        unsafe { wxDC_GetTextBackground(self.ptr(), _ref.ptr()) }
    }
    fn getTextExtent<T: TFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, externalLeading: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetTextExtent(self.ptr(), string.ptr(), w, h, descent, externalLeading, theFont.ptr()) }
    }
    fn getMultiLineTextExtent<T: TFont>(&self, string: &str, w: *mut c_void, h: *mut c_void, heightLine: *mut c_void, theFont: &T) {
        let string = wxT(string);
        unsafe { wxDC_GetMultiLineTextExtent(self.ptr(), string.ptr(), w, h, heightLine, theFont.ptr()) }
    }
    fn getTextForeground<T: TColour>(&self, _ref: &T) {
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
    fn setBackground<T: TBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBackground(self.ptr(), brush.ptr()) }
    }
    fn setBackgroundMode(&self, mode: c_int) {
        unsafe { wxDC_SetBackgroundMode(self.ptr(), mode) }
    }
    fn setBrush<T: TBrush>(&self, brush: &T) {
        unsafe { wxDC_SetBrush(self.ptr(), brush.ptr()) }
    }
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { wxDC_SetClippingRegion(self.ptr(), x, y, width, height) }
    }
    fn setClippingRegionFromRegion<T: TRegion>(&self, region: &T) {
        unsafe { wxDC_SetClippingRegionFromRegion(self.ptr(), region.ptr()) }
    }
    fn setDeviceClippingRegion<T: TRegion>(&self, region: &T) {
        unsafe { wxDC_SetDeviceClippingRegion(self.ptr(), region.ptr()) }
    }
    fn setDeviceOrigin(&self, x: c_int, y: c_int) {
        unsafe { wxDC_SetDeviceOrigin(self.ptr(), x, y) }
    }
    fn setFont<T: TFont>(&self, font: &T) {
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
    fn setPalette<T: TPalette>(&self, palette: &T) {
        unsafe { wxDC_SetPalette(self.ptr(), palette.ptr()) }
    }
    fn setPen<T: TPen>(&self, pen: &T) {
        unsafe { wxDC_SetPen(self.ptr(), pen.ptr()) }
    }
    fn setTextBackground<T: TColour>(&self, colour: &T) {
        unsafe { wxDC_SetTextBackground(self.ptr(), colour.ptr()) }
    }
    fn setTextForeground<T: TColour>(&self, colour: &T) {
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
    fn getPixel2<T: TColour>(&self, x: c_int, y: c_int, col: &T) {
        unsafe { wxDC_GetPixel2(self.ptr(), x, y, col.ptr()) }
    }
}

pub struct DCClipper { ptr: *mut c_void }
impl TDCClipper for DCClipper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DCClipper {
    pub fn from(ptr: *mut c_void) -> DCClipper { DCClipper { ptr: ptr } }
    pub fn null() -> DCClipper { DCClipper::from(0 as *mut c_void) }
    
}

pub trait TDCClipper {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DataFormat { ptr: *mut c_void }
impl TDataFormat for DataFormat { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataFormat {
    pub fn from(ptr: *mut c_void) -> DataFormat { DataFormat { ptr: ptr } }
    pub fn null() -> DataFormat { DataFormat::from(0 as *mut c_void) }
    
    pub fn newFromId(name: &str) -> DataFormat {
        let name = wxT(name);
        unsafe { DataFormat { ptr: wxDataFormat_CreateFromId(name.ptr()) } }
    }
    pub fn newFromType(typ: c_int) -> DataFormat {
        unsafe { DataFormat { ptr: wxDataFormat_CreateFromType(typ) } }
    }
}

pub trait TDataFormat {
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

pub struct DataObject { ptr: *mut c_void }
impl TDataObject for DataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataObject {
    pub fn from(ptr: *mut c_void) -> DataObject { DataObject { ptr: ptr } }
    pub fn null() -> DataObject { DataObject::from(0 as *mut c_void) }
    
}

pub trait TDataObject {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DataObjectComposite { ptr: *mut c_void }
impl TDataObjectComposite for DataObjectComposite {}
impl TDataObject for DataObjectComposite { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataObjectComposite {
    pub fn from(ptr: *mut c_void) -> DataObjectComposite { DataObjectComposite { ptr: ptr } }
    pub fn null() -> DataObjectComposite { DataObjectComposite::from(0 as *mut c_void) }
    
    pub fn new() -> DataObjectComposite {
        unsafe { DataObjectComposite { ptr: wxDataObjectComposite_Create() } }
    }
}

pub trait TDataObjectComposite : TDataObject {
    fn add(&self, _dat: *mut c_void, _preferred: c_int) {
        unsafe { wxDataObjectComposite_Add(self.ptr(), _dat, _preferred) }
    }
    fn delete(&self) {
        unsafe { wxDataObjectComposite_Delete(self.ptr()) }
    }
}

pub struct DataObjectSimple { ptr: *mut c_void }
impl TDataObjectSimple for DataObjectSimple {}
impl TDataObject for DataObjectSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DataObjectSimple {
    pub fn from(ptr: *mut c_void) -> DataObjectSimple { DataObjectSimple { ptr: ptr } }
    pub fn null() -> DataObjectSimple { DataObjectSimple::from(0 as *mut c_void) }
    
}

pub trait TDataObjectSimple : TDataObject {
}

pub struct DialUpEvent { ptr: *mut c_void }
impl TDialUpEvent for DialUpEvent {}
impl TEvent for DialUpEvent {}
impl TObject for DialUpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DialUpEvent {
    pub fn from(ptr: *mut c_void) -> DialUpEvent { DialUpEvent { ptr: ptr } }
    pub fn null() -> DialUpEvent { DialUpEvent::from(0 as *mut c_void) }
    
}

pub trait TDialUpEvent : TEvent {
}

pub struct DialUpManager { ptr: *mut c_void }
impl TDialUpManager for DialUpManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DialUpManager {
    pub fn from(ptr: *mut c_void) -> DialUpManager { DialUpManager { ptr: ptr } }
    pub fn null() -> DialUpManager { DialUpManager::from(0 as *mut c_void) }
    
}

pub trait TDialUpManager {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Dialog { ptr: *mut c_void }
impl TDialog for Dialog {}
impl TTopLevelWindow for Dialog {}
impl TWindow for Dialog {}
impl TEvtHandler for Dialog {}
impl TObject for Dialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Dialog {
    pub fn from(ptr: *mut c_void) -> Dialog { Dialog { ptr: ptr } }
    pub fn null() -> Dialog { Dialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Dialog {
        let _txt = wxT(_txt);
        unsafe { Dialog { ptr: wxDialog_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TDialog : TTopLevelWindow {
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

pub struct DirDialog { ptr: *mut c_void }
impl TDirDialog for DirDialog {}
impl TDialog for DirDialog {}
impl TTopLevelWindow for DirDialog {}
impl TWindow for DirDialog {}
impl TEvtHandler for DirDialog {}
impl TObject for DirDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DirDialog {
    pub fn from(ptr: *mut c_void) -> DirDialog { DirDialog { ptr: ptr } }
    pub fn null() -> DirDialog { DirDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _msg: &str, _dir: &str, _lft: c_int, _top: c_int, _stl: c_int) -> DirDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        unsafe { DirDialog { ptr: wxDirDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _lft, _top, _stl) } }
    }
}

pub trait TDirDialog : TDialog {
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

pub struct DocChildFrame { ptr: *mut c_void }
impl TDocChildFrame for DocChildFrame {}
impl TFrame for DocChildFrame {}
impl TTopLevelWindow for DocChildFrame {}
impl TWindow for DocChildFrame {}
impl TEvtHandler for DocChildFrame {}
impl TObject for DocChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocChildFrame {
    pub fn from(ptr: *mut c_void) -> DocChildFrame { DocChildFrame { ptr: ptr } }
    pub fn null() -> DocChildFrame { DocChildFrame::from(0 as *mut c_void) }
    
}

pub trait TDocChildFrame : TFrame {
}

pub struct DocMDIChildFrame { ptr: *mut c_void }
impl TDocMDIChildFrame for DocMDIChildFrame {}
impl TMDIChildFrame for DocMDIChildFrame {}
impl TFrame for DocMDIChildFrame {}
impl TTopLevelWindow for DocMDIChildFrame {}
impl TWindow for DocMDIChildFrame {}
impl TEvtHandler for DocMDIChildFrame {}
impl TObject for DocMDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocMDIChildFrame {
    pub fn from(ptr: *mut c_void) -> DocMDIChildFrame { DocMDIChildFrame { ptr: ptr } }
    pub fn null() -> DocMDIChildFrame { DocMDIChildFrame::from(0 as *mut c_void) }
    
}

pub trait TDocMDIChildFrame : TMDIChildFrame {
}

pub struct DocMDIParentFrame { ptr: *mut c_void }
impl TDocMDIParentFrame for DocMDIParentFrame {}
impl TMDIParentFrame for DocMDIParentFrame {}
impl TFrame for DocMDIParentFrame {}
impl TTopLevelWindow for DocMDIParentFrame {}
impl TWindow for DocMDIParentFrame {}
impl TEvtHandler for DocMDIParentFrame {}
impl TObject for DocMDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocMDIParentFrame {
    pub fn from(ptr: *mut c_void) -> DocMDIParentFrame { DocMDIParentFrame { ptr: ptr } }
    pub fn null() -> DocMDIParentFrame { DocMDIParentFrame::from(0 as *mut c_void) }
    
}

pub trait TDocMDIParentFrame : TMDIParentFrame {
}

pub struct DocManager { ptr: *mut c_void }
impl TDocManager for DocManager {}
impl TEvtHandler for DocManager {}
impl TObject for DocManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocManager {
    pub fn from(ptr: *mut c_void) -> DocManager { DocManager { ptr: ptr } }
    pub fn null() -> DocManager { DocManager::from(0 as *mut c_void) }
    
}

pub trait TDocManager : TEvtHandler {
}

pub struct DocParentFrame { ptr: *mut c_void }
impl TDocParentFrame for DocParentFrame {}
impl TFrame for DocParentFrame {}
impl TTopLevelWindow for DocParentFrame {}
impl TWindow for DocParentFrame {}
impl TEvtHandler for DocParentFrame {}
impl TObject for DocParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocParentFrame {
    pub fn from(ptr: *mut c_void) -> DocParentFrame { DocParentFrame { ptr: ptr } }
    pub fn null() -> DocParentFrame { DocParentFrame::from(0 as *mut c_void) }
    
}

pub trait TDocParentFrame : TFrame {
}

pub struct DocTemplate { ptr: *mut c_void }
impl TDocTemplate for DocTemplate {}
impl TObject for DocTemplate { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DocTemplate {
    pub fn from(ptr: *mut c_void) -> DocTemplate { DocTemplate { ptr: ptr } }
    pub fn null() -> DocTemplate { DocTemplate::from(0 as *mut c_void) }
    
}

pub trait TDocTemplate : TObject {
}

pub struct Document { ptr: *mut c_void }
impl TDocument for Document {}
impl TEvtHandler for Document {}
impl TObject for Document { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Document {
    pub fn from(ptr: *mut c_void) -> Document { Document { ptr: ptr } }
    pub fn null() -> Document { Document::from(0 as *mut c_void) }
    
}

pub trait TDocument : TEvtHandler {
}

pub struct DragImage { ptr: *mut c_void }
impl TDragImage for DragImage {}
impl TObject for DragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DragImage {
    pub fn from(ptr: *mut c_void) -> DragImage { DragImage { ptr: ptr } }
    pub fn null() -> DragImage { DragImage::from(0 as *mut c_void) }
    
    pub fn new<T: TBitmap>(image: &T, x: c_int, y: c_int) -> DragImage {
        unsafe { DragImage { ptr: wxDragImage_Create(image.ptr(), x, y) } }
    }
}

pub trait TDragImage : TObject {
    fn beginDragFullScreen<T: TWindow, U: TRect>(&self, x_pos: c_int, y_pos: c_int, window: &T, fullScreen: c_int, rect: &U) -> c_int {
        unsafe { wxDragImage_BeginDragFullScreen(self.ptr(), x_pos, y_pos, window.ptr(), fullScreen, rect.ptr()) }
    }
    fn beginDrag<T: TWindow, U: TWindow>(&self, x: c_int, y: c_int, window: &T, boundingWindow: &U) -> c_int {
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

pub struct DrawControl { ptr: *mut c_void }
impl TDrawControl for DrawControl {}
impl TControl for DrawControl {}
impl TWindow for DrawControl {}
impl TEvtHandler for DrawControl {}
impl TObject for DrawControl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DrawControl {
    pub fn from(ptr: *mut c_void) -> DrawControl { DrawControl { ptr: ptr } }
    pub fn null() -> DrawControl { DrawControl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> DrawControl {
        unsafe { DrawControl { ptr: wxDrawControl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TDrawControl : TControl {
}

pub struct DrawWindow { ptr: *mut c_void }
impl TDrawWindow for DrawWindow {}
impl TWindow for DrawWindow {}
impl TEvtHandler for DrawWindow {}
impl TObject for DrawWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DrawWindow {
    pub fn from(ptr: *mut c_void) -> DrawWindow { DrawWindow { ptr: ptr } }
    pub fn null() -> DrawWindow { DrawWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> DrawWindow {
        unsafe { DrawWindow { ptr: wxDrawWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TDrawWindow : TWindow {
}

pub struct DropFilesEvent { ptr: *mut c_void }
impl TDropFilesEvent for DropFilesEvent {}
impl TEvent for DropFilesEvent {}
impl TObject for DropFilesEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DropFilesEvent {
    pub fn from(ptr: *mut c_void) -> DropFilesEvent { DropFilesEvent { ptr: ptr } }
    pub fn null() -> DropFilesEvent { DropFilesEvent::from(0 as *mut c_void) }
    
}

pub trait TDropFilesEvent : TEvent {
}

pub struct DropSource { ptr: *mut c_void }
impl TDropSource for DropSource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DropSource {
    pub fn from(ptr: *mut c_void) -> DropSource { DropSource { ptr: ptr } }
    pub fn null() -> DropSource { DropSource::from(0 as *mut c_void) }
    
}

pub trait TDropSource {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct DropTarget { ptr: *mut c_void }
impl TDropTarget for DropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DropTarget {
    pub fn from(ptr: *mut c_void) -> DropTarget { DropTarget { ptr: ptr } }
    pub fn null() -> DropTarget { DropTarget::from(0 as *mut c_void) }
    
}

pub trait TDropTarget {
    fn ptr(&self) -> *mut c_void;
    
    fn getData(&self) {
        unsafe { wxDropTarget_GetData(self.ptr()) }
    }
    fn setDataObject<T: TDataObject>(&self, _dat: &T) {
        unsafe { wxDropTarget_SetDataObject(self.ptr(), _dat.ptr()) }
    }
}

pub struct EraseEvent { ptr: *mut c_void }
impl TEraseEvent for EraseEvent {}
impl TEvent for EraseEvent {}
impl TObject for EraseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EraseEvent {
    pub fn from(ptr: *mut c_void) -> EraseEvent { EraseEvent { ptr: ptr } }
    pub fn null() -> EraseEvent { EraseEvent::from(0 as *mut c_void) }
    
}

pub trait TEraseEvent : TEvent {
    fn getDC(&self) -> DC {
        unsafe { DC { ptr: wxEraseEvent_GetDC(self.ptr()) } }
    }
}

pub struct Event { ptr: *mut c_void }
impl TEvent for Event {}
impl TObject for Event { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Event {
    pub fn from(ptr: *mut c_void) -> Event { Event { ptr: ptr } }
    pub fn null() -> Event { Event::from(0 as *mut c_void) }
    
    pub fn newEventType() -> c_int {
        unsafe { wxEvent_NewEventType() }
    }
}

pub trait TEvent : TObject {
    fn copyObject(&self, object_dest: *mut c_void) {
        unsafe { wxEvent_CopyObject(self.ptr(), object_dest) }
    }
    fn getEventObject(&self) -> Object {
        unsafe { Object { ptr: wxEvent_GetEventObject(self.ptr()) } }
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
    fn setEventObject<T: TObject>(&self, obj: &T) {
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

pub struct EvtHandler { ptr: *mut c_void }
impl TEvtHandler for EvtHandler {}
impl TObject for EvtHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EvtHandler {
    pub fn from(ptr: *mut c_void) -> EvtHandler { EvtHandler { ptr: ptr } }
    pub fn null() -> EvtHandler { EvtHandler::from(0 as *mut c_void) }
    
    pub fn new() -> EvtHandler {
        unsafe { EvtHandler { ptr: wxEvtHandler_Create() } }
    }
}

pub trait TEvtHandler : TObject {
    fn addPendingEvent<T: TEvent>(&self, event: &T) {
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
        unsafe { EvtHandler { ptr: wxEvtHandler_GetNextHandler(self.ptr()) } }
    }
    fn getPreviousHandler(&self) -> EvtHandler {
        unsafe { EvtHandler { ptr: wxEvtHandler_GetPreviousHandler(self.ptr()) } }
    }
    fn processEvent<T: TEvent>(&self, event: &T) -> c_int {
        unsafe { wxEvtHandler_ProcessEvent(self.ptr(), event.ptr()) }
    }
    fn processPendingEvents(&self) {
        unsafe { wxEvtHandler_ProcessPendingEvents(self.ptr()) }
    }
    fn setEvtHandlerEnabled(&self, enabled: c_int) {
        unsafe { wxEvtHandler_SetEvtHandlerEnabled(self.ptr(), enabled) }
    }
    fn setNextHandler<T: TEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetNextHandler(self.ptr(), handler.ptr()) }
    }
    fn setPreviousHandler<T: TEvtHandler>(&self, handler: &T) {
        unsafe { wxEvtHandler_SetPreviousHandler(self.ptr(), handler.ptr()) }
    }
    fn getClosure(&self, id: c_int, type_: c_int) -> Closure {
        unsafe { Closure { ptr: wxEvtHandler_GetClosure(self.ptr(), id, type_) } }
    }
}

pub struct FileDataObject { ptr: *mut c_void }
impl TFileDataObject for FileDataObject {}
impl TDataObjectSimple for FileDataObject {}
impl TDataObject for FileDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileDataObject {
    pub fn from(ptr: *mut c_void) -> FileDataObject { FileDataObject { ptr: ptr } }
    pub fn null() -> FileDataObject { FileDataObject::from(0 as *mut c_void) }
    
}

pub trait TFileDataObject : TDataObjectSimple {
}

pub struct FileDialog { ptr: *mut c_void }
impl TFileDialog for FileDialog {}
impl TDialog for FileDialog {}
impl TTopLevelWindow for FileDialog {}
impl TWindow for FileDialog {}
impl TEvtHandler for FileDialog {}
impl TObject for FileDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileDialog {
    pub fn from(ptr: *mut c_void) -> FileDialog { FileDialog { ptr: ptr } }
    pub fn null() -> FileDialog { FileDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _msg: &str, _dir: &str, _fle: &str, _wcd: &str, _lft: c_int, _top: c_int, _stl: c_int) -> FileDialog {
        let _msg = wxT(_msg);
        let _dir = wxT(_dir);
        let _fle = wxT(_fle);
        let _wcd = wxT(_wcd);
        unsafe { FileDialog { ptr: wxFileDialog_Create(_prt.ptr(), _msg.ptr(), _dir.ptr(), _fle.ptr(), _wcd.ptr(), _lft, _top, _stl) } }
    }
}

pub trait TFileDialog : TDialog {
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

pub struct FileDropTarget { ptr: *mut c_void }
impl TFileDropTarget for FileDropTarget {}
impl TDropTarget for FileDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileDropTarget {
    pub fn from(ptr: *mut c_void) -> FileDropTarget { FileDropTarget { ptr: ptr } }
    pub fn null() -> FileDropTarget { FileDropTarget::from(0 as *mut c_void) }
    
}

pub trait TFileDropTarget : TDropTarget {
}

pub struct FileHistory { ptr: *mut c_void }
impl TFileHistory for FileHistory {}
impl TObject for FileHistory { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileHistory {
    pub fn from(ptr: *mut c_void) -> FileHistory { FileHistory { ptr: ptr } }
    pub fn null() -> FileHistory { FileHistory::from(0 as *mut c_void) }
    
    pub fn new(maxFiles: c_int) -> FileHistory {
        unsafe { FileHistory { ptr: wxFileHistory_Create(maxFiles) } }
    }
}

pub trait TFileHistory : TObject {
    fn addFileToHistory(&self, file: &str) {
        let file = wxT(file);
        unsafe { wxFileHistory_AddFileToHistory(self.ptr(), file.ptr()) }
    }
    fn addFilesToMenu<T: TMenu>(&self, menu: &T) {
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
    fn load<T: TConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Load(self.ptr(), config.ptr()) }
    }
    fn removeFileFromHistory(&self, i: c_int) {
        unsafe { wxFileHistory_RemoveFileFromHistory(self.ptr(), i) }
    }
    fn removeMenu<T: TMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_RemoveMenu(self.ptr(), menu.ptr()) }
    }
    fn save<T: TConfigBase>(&self, config: &T) {
        unsafe { wxFileHistory_Save(self.ptr(), config.ptr()) }
    }
    fn useMenu<T: TMenu>(&self, menu: &T) {
        unsafe { wxFileHistory_UseMenu(self.ptr(), menu.ptr()) }
    }
}

pub struct FileType { ptr: *mut c_void }
impl TFileType for FileType { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FileType {
    pub fn from(ptr: *mut c_void) -> FileType { FileType { ptr: ptr } }
    pub fn null() -> FileType { FileType::from(0 as *mut c_void) }
    
}

pub trait TFileType {
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
    fn getExtensions<T: TList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetExtensions(self.ptr(), _lst.ptr()) }
    }
    fn getIcon<T: TIcon>(&self, icon: &T) -> c_int {
        unsafe { wxFileType_GetIcon(self.ptr(), icon.ptr()) }
    }
    fn getMimeType(&self) -> ~str {
        unsafe { WxString { ptr: wxFileType_GetMimeType(self.ptr()) }.to_str() }
    }
    fn getMimeTypes<T: TList>(&self, _lst: &T) -> c_int {
        unsafe { wxFileType_GetMimeTypes(self.ptr(), _lst.ptr()) }
    }
    fn getOpenCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetOpenCommand(self.ptr(), _buf, _params) }
    }
    fn getPrintCommand(&self, _buf: *mut c_void, _params: *mut c_void) -> c_int {
        unsafe { wxFileType_GetPrintCommand(self.ptr(), _buf, _params) }
    }
}

pub struct FindDialogEvent { ptr: *mut c_void }
impl TFindDialogEvent for FindDialogEvent {}
impl TCommandEvent for FindDialogEvent {}
impl TEvent for FindDialogEvent {}
impl TObject for FindDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FindDialogEvent {
    pub fn from(ptr: *mut c_void) -> FindDialogEvent { FindDialogEvent { ptr: ptr } }
    pub fn null() -> FindDialogEvent { FindDialogEvent::from(0 as *mut c_void) }
    
}

pub trait TFindDialogEvent : TCommandEvent {
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

pub struct FindReplaceData { ptr: *mut c_void }
impl TFindReplaceData for FindReplaceData {}
impl TObject for FindReplaceData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FindReplaceData {
    pub fn from(ptr: *mut c_void) -> FindReplaceData { FindReplaceData { ptr: ptr } }
    pub fn null() -> FindReplaceData { FindReplaceData::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> FindReplaceData {
        unsafe { FindReplaceData { ptr: wxFindReplaceData_Create(flags) } }
    }
    pub fn newDefault() -> FindReplaceData {
        unsafe { FindReplaceData { ptr: wxFindReplaceData_CreateDefault() } }
    }
}

pub trait TFindReplaceData : TObject {
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

pub struct FindReplaceDialog { ptr: *mut c_void }
impl TFindReplaceDialog for FindReplaceDialog {}
impl TDialog for FindReplaceDialog {}
impl TTopLevelWindow for FindReplaceDialog {}
impl TWindow for FindReplaceDialog {}
impl TEvtHandler for FindReplaceDialog {}
impl TObject for FindReplaceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FindReplaceDialog {
    pub fn from(ptr: *mut c_void) -> FindReplaceDialog { FindReplaceDialog { ptr: ptr } }
    pub fn null() -> FindReplaceDialog { FindReplaceDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TFindReplaceData>(parent: &T, data: &U, title: &str, style: c_int) -> FindReplaceDialog {
        let title = wxT(title);
        unsafe { FindReplaceDialog { ptr: wxFindReplaceDialog_Create(parent.ptr(), data.ptr(), title.ptr(), style) } }
    }
}

pub trait TFindReplaceDialog : TDialog {
    fn getData(&self) -> FindReplaceData {
        unsafe { FindReplaceData { ptr: wxFindReplaceDialog_GetData(self.ptr()) } }
    }
    fn setData<T: TFindReplaceData>(&self, data: &T) {
        unsafe { wxFindReplaceDialog_SetData(self.ptr(), data.ptr()) }
    }
}

pub struct FlexGridSizer { ptr: *mut c_void }
impl TFlexGridSizer for FlexGridSizer {}
impl TGridSizer for FlexGridSizer {}
impl TSizer for FlexGridSizer {}
impl TObject for FlexGridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FlexGridSizer {
    pub fn from(ptr: *mut c_void) -> FlexGridSizer { FlexGridSizer { ptr: ptr } }
    pub fn null() -> FlexGridSizer { FlexGridSizer::from(0 as *mut c_void) }
    
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> FlexGridSizer {
        unsafe { FlexGridSizer { ptr: wxFlexGridSizer_Create(rows, cols, vgap, hgap) } }
    }
}

pub trait TFlexGridSizer : TGridSizer {
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

pub struct FocusEvent { ptr: *mut c_void }
impl TFocusEvent for FocusEvent {}
impl TEvent for FocusEvent {}
impl TObject for FocusEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FocusEvent {
    pub fn from(ptr: *mut c_void) -> FocusEvent { FocusEvent { ptr: ptr } }
    pub fn null() -> FocusEvent { FocusEvent::from(0 as *mut c_void) }
    
}

pub trait TFocusEvent : TEvent {
}

pub struct Font { ptr: *mut c_void }
impl TFont for Font {}
impl TGDIObject for Font {}
impl TObject for Font { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Font {
    pub fn from(ptr: *mut c_void) -> Font { Font { ptr: ptr } }
    pub fn null() -> Font { Font::from(0 as *mut c_void) }
    
    pub fn new(pointSize: c_int, family: c_int, style: c_int, weight: c_int, underlined: c_int, face: &str, enc: c_int) -> Font {
        let face = wxT(face);
        unsafe { Font { ptr: wxFont_Create(pointSize, family, style, weight, underlined, face.ptr(), enc) } }
    }
    pub fn newFromStock(id: c_int) -> Font {
        unsafe { Font { ptr: wxFont_CreateFromStock(id) } }
    }
    pub fn newDefault() -> Font {
        unsafe { Font { ptr: wxFont_CreateDefault() } }
    }
}

pub trait TFont : TGDIObject {
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

pub struct FontData { ptr: *mut c_void }
impl TFontData for FontData {}
impl TObject for FontData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontData {
    pub fn from(ptr: *mut c_void) -> FontData { FontData { ptr: ptr } }
    pub fn null() -> FontData { FontData::from(0 as *mut c_void) }
    
    pub fn new() -> FontData {
        unsafe { FontData { ptr: wxFontData_Create() } }
    }
}

pub trait TFontData : TObject {
    fn enableEffects(&self, flag: c_int) {
        unsafe { wxFontData_EnableEffects(self.ptr(), flag) }
    }
    fn getAllowSymbols(&self) -> c_int {
        unsafe { wxFontData_GetAllowSymbols(self.ptr()) }
    }
    fn getChosenFont<T: TFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetChosenFont(self.ptr(), ref_.ptr()) }
    }
    fn getColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxFontData_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getEnableEffects(&self) -> c_int {
        unsafe { wxFontData_GetEnableEffects(self.ptr()) }
    }
    fn getEncoding(&self) -> c_int {
        unsafe { wxFontData_GetEncoding(self.ptr()) }
    }
    fn getInitialFont<T: TFont>(&self, ref_: &T) {
        unsafe { wxFontData_GetInitialFont(self.ptr(), ref_.ptr()) }
    }
    fn getShowHelp(&self) -> c_int {
        unsafe { wxFontData_GetShowHelp(self.ptr()) }
    }
    fn setAllowSymbols(&self, flag: c_int) {
        unsafe { wxFontData_SetAllowSymbols(self.ptr(), flag) }
    }
    fn setChosenFont<T: TFont>(&self, font: &T) {
        unsafe { wxFontData_SetChosenFont(self.ptr(), font.ptr()) }
    }
    fn setColour<T: TColour>(&self, colour: &T) {
        unsafe { wxFontData_SetColour(self.ptr(), colour.ptr()) }
    }
    fn setEncoding(&self, encoding: c_int) {
        unsafe { wxFontData_SetEncoding(self.ptr(), encoding) }
    }
    fn setInitialFont<T: TFont>(&self, font: &T) {
        unsafe { wxFontData_SetInitialFont(self.ptr(), font.ptr()) }
    }
    fn setRange(&self, minRange: c_int, maxRange: c_int) {
        unsafe { wxFontData_SetRange(self.ptr(), minRange, maxRange) }
    }
    fn setShowHelp(&self, flag: c_int) {
        unsafe { wxFontData_SetShowHelp(self.ptr(), flag) }
    }
}

pub struct FontDialog { ptr: *mut c_void }
impl TFontDialog for FontDialog {}
impl TDialog for FontDialog {}
impl TTopLevelWindow for FontDialog {}
impl TWindow for FontDialog {}
impl TEvtHandler for FontDialog {}
impl TObject for FontDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontDialog {
    pub fn from(ptr: *mut c_void) -> FontDialog { FontDialog { ptr: ptr } }
    pub fn null() -> FontDialog { FontDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TFontData>(_prt: &T, fnt: &U) -> FontDialog {
        unsafe { FontDialog { ptr: wxFontDialog_Create(_prt.ptr(), fnt.ptr()) } }
    }
}

pub trait TFontDialog : TDialog {
    fn getFontData<T: TFontData>(&self, _ref: &T) {
        unsafe { wxFontDialog_GetFontData(self.ptr(), _ref.ptr()) }
    }
}

pub struct FontEnumerator { ptr: *mut c_void }
impl TFontEnumerator for FontEnumerator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontEnumerator {
    pub fn from(ptr: *mut c_void) -> FontEnumerator { FontEnumerator { ptr: ptr } }
    pub fn null() -> FontEnumerator { FontEnumerator::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _fnc: *mut c_void) -> FontEnumerator {
        unsafe { FontEnumerator { ptr: wxFontEnumerator_Create(_obj, _fnc) } }
    }
}

pub trait TFontEnumerator {
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

pub struct FontList { ptr: *mut c_void }
impl TFontList for FontList {}
impl TList for FontList {}
impl TObject for FontList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontList {
    pub fn from(ptr: *mut c_void) -> FontList { FontList { ptr: ptr } }
    pub fn null() -> FontList { FontList::from(0 as *mut c_void) }
    
}

pub trait TFontList : TList {
}

pub struct FontMapper { ptr: *mut c_void }
impl TFontMapper for FontMapper { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FontMapper {
    pub fn from(ptr: *mut c_void) -> FontMapper { FontMapper { ptr: ptr } }
    pub fn null() -> FontMapper { FontMapper::from(0 as *mut c_void) }
    
    pub fn new() -> FontMapper {
        unsafe { FontMapper { ptr: wxFontMapper_Create() } }
    }
}

pub trait TFontMapper {
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

pub struct Frame { ptr: *mut c_void }
impl TFrame for Frame {}
impl TTopLevelWindow for Frame {}
impl TWindow for Frame {}
impl TEvtHandler for Frame {}
impl TObject for Frame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Frame {
    pub fn from(ptr: *mut c_void) -> Frame { Frame { ptr: ptr } }
    pub fn null() -> Frame { Frame::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Frame {
        let _txt = wxT(_txt);
        unsafe { Frame { ptr: wxFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TFrame : TTopLevelWindow {
    fn newStatusBar(&self, number: c_int, style: c_int) -> StatusBar {
        unsafe { StatusBar { ptr: wxFrame_CreateStatusBar(self.ptr(), number, style) } }
    }
    fn newToolBar(&self, style: c_long) -> ToolBar {
        unsafe { ToolBar { ptr: wxFrame_CreateToolBar(self.ptr(), style) } }
    }
    fn getClientAreaOrigin_left(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_left(self.ptr()) }
    }
    fn getClientAreaOrigin_top(&self) -> c_int {
        unsafe { wxFrame_GetClientAreaOrigin_top(self.ptr()) }
    }
    fn getMenuBar(&self) -> MenuBar {
        unsafe { MenuBar { ptr: wxFrame_GetMenuBar(self.ptr()) } }
    }
    fn getStatusBar(&self) -> StatusBar {
        unsafe { StatusBar { ptr: wxFrame_GetStatusBar(self.ptr()) } }
    }
    fn getToolBar(&self) -> ToolBar {
        unsafe { ToolBar { ptr: wxFrame_GetToolBar(self.ptr()) } }
    }
    fn restore(&self) {
        unsafe { wxFrame_Restore(self.ptr()) }
    }
    fn setMenuBar<T: TMenuBar>(&self, menubar: &T) {
        unsafe { wxFrame_SetMenuBar(self.ptr(), menubar.ptr()) }
    }
    fn setStatusBar<T: TStatusBar>(&self, statBar: &T) {
        unsafe { wxFrame_SetStatusBar(self.ptr(), statBar.ptr()) }
    }
    fn setStatusText(&self, _txt: &str, _number: c_int) {
        let _txt = wxT(_txt);
        unsafe { wxFrame_SetStatusText(self.ptr(), _txt.ptr(), _number) }
    }
    fn setStatusWidths(&self, _n: c_int, _widths_field: *mut c_void) {
        unsafe { wxFrame_SetStatusWidths(self.ptr(), _n, _widths_field) }
    }
    fn setToolBar<T: TToolBar>(&self, _toolbar: &T) {
        unsafe { wxFrame_SetToolBar(self.ptr(), _toolbar.ptr()) }
    }
    fn setShape<T: TRegion>(&self, region: &T) -> c_int {
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

pub struct GDIObject { ptr: *mut c_void }
impl TGDIObject for GDIObject {}
impl TObject for GDIObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GDIObject {
    pub fn from(ptr: *mut c_void) -> GDIObject { GDIObject { ptr: ptr } }
    pub fn null() -> GDIObject { GDIObject::from(0 as *mut c_void) }
    
}

pub trait TGDIObject : TObject {
}

pub struct Gauge { ptr: *mut c_void }
impl TGauge for Gauge {}
impl TControl for Gauge {}
impl TWindow for Gauge {}
impl TEvtHandler for Gauge {}
impl TObject for Gauge { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Gauge {
    pub fn from(ptr: *mut c_void) -> Gauge { Gauge { ptr: ptr } }
    pub fn null() -> Gauge { Gauge::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _rng: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Gauge {
        unsafe { Gauge { ptr: wxGauge_Create(_prt.ptr(), _id, _rng, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TGauge : TControl {
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

pub struct GenericDirCtrl { ptr: *mut c_void }
impl TGenericDirCtrl for GenericDirCtrl {}
impl TControl for GenericDirCtrl {}
impl TWindow for GenericDirCtrl {}
impl TEvtHandler for GenericDirCtrl {}
impl TObject for GenericDirCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GenericDirCtrl {
    pub fn from(ptr: *mut c_void) -> GenericDirCtrl { GenericDirCtrl { ptr: ptr } }
    pub fn null() -> GenericDirCtrl { GenericDirCtrl::from(0 as *mut c_void) }
    
}

pub trait TGenericDirCtrl : TControl {
}

pub struct GenericValidator { ptr: *mut c_void }
impl TGenericValidator for GenericValidator {}
impl TValidator for GenericValidator {}
impl TEvtHandler for GenericValidator {}
impl TObject for GenericValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GenericValidator {
    pub fn from(ptr: *mut c_void) -> GenericValidator { GenericValidator { ptr: ptr } }
    pub fn null() -> GenericValidator { GenericValidator::from(0 as *mut c_void) }
    
}

pub trait TGenericValidator : TValidator {
}

pub struct GridSizer { ptr: *mut c_void }
impl TGridSizer for GridSizer {}
impl TSizer for GridSizer {}
impl TObject for GridSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridSizer {
    pub fn from(ptr: *mut c_void) -> GridSizer { GridSizer { ptr: ptr } }
    pub fn null() -> GridSizer { GridSizer::from(0 as *mut c_void) }
    
    pub fn new(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> GridSizer {
        unsafe { GridSizer { ptr: wxGridSizer_Create(rows, cols, vgap, hgap) } }
    }
}

pub trait TGridSizer : TSizer {
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

pub struct HelpController { ptr: *mut c_void }
impl THelpController for HelpController {}
impl THelpControllerBase for HelpController {}
impl TObject for HelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpController {
    pub fn from(ptr: *mut c_void) -> HelpController { HelpController { ptr: ptr } }
    pub fn null() -> HelpController { HelpController::from(0 as *mut c_void) }
    
}

pub trait THelpController : THelpControllerBase {
}

pub struct HelpControllerBase { ptr: *mut c_void }
impl THelpControllerBase for HelpControllerBase {}
impl TObject for HelpControllerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpControllerBase {
    pub fn from(ptr: *mut c_void) -> HelpControllerBase { HelpControllerBase { ptr: ptr } }
    pub fn null() -> HelpControllerBase { HelpControllerBase::from(0 as *mut c_void) }
    
}

pub trait THelpControllerBase : TObject {
}

pub struct HelpControllerHelpProvider { ptr: *mut c_void }
impl THelpControllerHelpProvider for HelpControllerHelpProvider {}
impl TSimpleHelpProvider for HelpControllerHelpProvider {}
impl THelpProvider for HelpControllerHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpControllerHelpProvider {
    pub fn from(ptr: *mut c_void) -> HelpControllerHelpProvider { HelpControllerHelpProvider { ptr: ptr } }
    pub fn null() -> HelpControllerHelpProvider { HelpControllerHelpProvider::from(0 as *mut c_void) }
    
    pub fn new<T: THelpControllerBase>(ctr: &T) -> HelpControllerHelpProvider {
        unsafe { HelpControllerHelpProvider { ptr: wxHelpControllerHelpProvider_Create(ctr.ptr()) } }
    }
}

pub trait THelpControllerHelpProvider : TSimpleHelpProvider {
    fn getHelpController(&self) -> HelpControllerBase {
        unsafe { HelpControllerBase { ptr: wxHelpControllerHelpProvider_GetHelpController(self.ptr()) } }
    }
    fn setHelpController<T: THelpController>(&self, hc: &T) {
        unsafe { wxHelpControllerHelpProvider_SetHelpController(self.ptr(), hc.ptr()) }
    }
}

pub struct HelpEvent { ptr: *mut c_void }
impl THelpEvent for HelpEvent {}
impl TCommandEvent for HelpEvent {}
impl TEvent for HelpEvent {}
impl TObject for HelpEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpEvent {
    pub fn from(ptr: *mut c_void) -> HelpEvent { HelpEvent { ptr: ptr } }
    pub fn null() -> HelpEvent { HelpEvent::from(0 as *mut c_void) }
    
}

pub trait THelpEvent : TCommandEvent {
    fn getLink(&self) -> ~str {
        unsafe { WxString { ptr: wxHelpEvent_GetLink(self.ptr()) }.to_str() }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxHelpEvent_GetPosition(self.ptr()) } }
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

pub struct HelpProvider { ptr: *mut c_void }
impl THelpProvider for HelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HelpProvider {
    pub fn from(ptr: *mut c_void) -> HelpProvider { HelpProvider { ptr: ptr } }
    pub fn null() -> HelpProvider { HelpProvider::from(0 as *mut c_void) }
    
    pub fn get() -> HelpProvider {
        unsafe { HelpProvider { ptr: wxHelpProvider_Get() } }
    }
}

pub trait THelpProvider {
    fn ptr(&self) -> *mut c_void;
    
    fn addHelp<T: TWindow>(&self, window: &T, text: &str) {
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
    fn getHelp<T: TWindow>(&self, window: &T) -> ~str {
        unsafe { WxString { ptr: wxHelpProvider_GetHelp(self.ptr(), window.ptr()) }.to_str() }
    }
    fn removeHelp<T: TWindow>(&self, window: &T) {
        unsafe { wxHelpProvider_RemoveHelp(self.ptr(), window.ptr()) }
    }
    fn set(&self) -> HelpProvider {
        unsafe { HelpProvider { ptr: wxHelpProvider_Set(self.ptr()) } }
    }
    fn showHelp<T: TWindow>(&self, window: &T) -> c_int {
        unsafe { wxHelpProvider_ShowHelp(self.ptr(), window.ptr()) }
    }
}

pub struct Icon { ptr: *mut c_void }
impl TIcon for Icon {}
impl TBitmap for Icon {}
impl TGDIObject for Icon {}
impl TObject for Icon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Icon {
    pub fn from(ptr: *mut c_void) -> Icon { Icon { ptr: ptr } }
    pub fn null() -> Icon { Icon::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Icon {
        unsafe { Icon { ptr: wxIcon_CreateDefault() } }
    }
    pub fn newLoad(name: &str, type_: c_long, width: c_int, height: c_int) -> Icon {
        let name = wxT(name);
        unsafe { Icon { ptr: wxIcon_CreateLoad(name.ptr(), type_, width, height) } }
    }
}

pub trait TIcon : TBitmap {
    fn assign(&self, other: *mut c_void) {
        unsafe { wxIcon_Assign(self.ptr(), other) }
    }
    fn copyFromBitmap<T: TBitmap>(&self, bmp: &T) {
        unsafe { wxIcon_CopyFromBitmap(self.ptr(), bmp.ptr()) }
    }
    fn fromRaw(&self, width: c_int, height: c_int) -> Icon {
        unsafe { Icon { ptr: wxIcon_FromRaw(self.ptr(), width, height) } }
    }
    fn fromXPM(&self) -> Icon {
        unsafe { Icon { ptr: wxIcon_FromXPM(self.ptr()) } }
    }
    fn isEqual(&self, other: &TIcon) -> c_int {
        unsafe { wxIcon_IsEqual(self.ptr(), other.ptr()) }
    }
    fn load(&self, name: &str, type_: c_long, width: c_int, height: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxIcon_Load(self.ptr(), name.ptr(), type_, width, height) }
    }
}

pub struct IconBundle { ptr: *mut c_void }
impl TIconBundle for IconBundle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IconBundle {
    pub fn from(ptr: *mut c_void) -> IconBundle { IconBundle { ptr: ptr } }
    pub fn null() -> IconBundle { IconBundle::from(0 as *mut c_void) }
    
    pub fn newDefault() -> IconBundle {
        unsafe { IconBundle { ptr: wxIconBundle_CreateDefault() } }
    }
    pub fn newFromFile(file: &str, type_: c_int) -> IconBundle {
        let file = wxT(file);
        unsafe { IconBundle { ptr: wxIconBundle_CreateFromFile(file.ptr(), type_) } }
    }
    pub fn newFromIcon<T: TIcon>(icon: &T) -> IconBundle {
        unsafe { IconBundle { ptr: wxIconBundle_CreateFromIcon(icon.ptr()) } }
    }
}

pub trait TIconBundle {
    fn ptr(&self) -> *mut c_void;
    
    fn addIcon<T: TIcon>(&self, icon: &T) {
        unsafe { wxIconBundle_AddIcon(self.ptr(), icon.ptr()) }
    }
    fn addIconFromFile(&self, file: &str, type_: c_int) {
        let file = wxT(file);
        unsafe { wxIconBundle_AddIconFromFile(self.ptr(), file.ptr(), type_) }
    }
    fn assign<T: TIconBundle>(&self, _ref: &T) {
        unsafe { wxIconBundle_Assign(self.ptr(), _ref.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxIconBundle_Delete(self.ptr()) }
    }
    fn getIcon<T: TIcon>(&self, w: c_int, h: c_int, _ref: &T) {
        unsafe { wxIconBundle_GetIcon(self.ptr(), w, h, _ref.ptr()) }
    }
}

pub struct IconizeEvent { ptr: *mut c_void }
impl TIconizeEvent for IconizeEvent {}
impl TEvent for IconizeEvent {}
impl TObject for IconizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IconizeEvent {
    pub fn from(ptr: *mut c_void) -> IconizeEvent { IconizeEvent { ptr: ptr } }
    pub fn null() -> IconizeEvent { IconizeEvent::from(0 as *mut c_void) }
    
}

pub trait TIconizeEvent : TEvent {
}

pub struct IdleEvent { ptr: *mut c_void }
impl TIdleEvent for IdleEvent {}
impl TEvent for IdleEvent {}
impl TObject for IdleEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IdleEvent {
    pub fn from(ptr: *mut c_void) -> IdleEvent { IdleEvent { ptr: ptr } }
    pub fn null() -> IdleEvent { IdleEvent::from(0 as *mut c_void) }
    
}

pub trait TIdleEvent : TEvent {
    fn moreRequested(&self) -> c_int {
        unsafe { wxIdleEvent_MoreRequested(self.ptr()) }
    }
    fn requestMore(&self, needMore: c_int) {
        unsafe { wxIdleEvent_RequestMore(self.ptr(), needMore) }
    }
}

pub struct Image { ptr: *mut c_void }
impl TImage for Image {}
impl TObject for Image { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Image {
    pub fn from(ptr: *mut c_void) -> Image { Image { ptr: ptr } }
    pub fn null() -> Image { Image::from(0 as *mut c_void) }
    
    pub fn canRead(name: &str) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_CanRead(name.ptr()) }
    }
    pub fn newDefault() -> Image {
        unsafe { Image { ptr: wxImage_CreateDefault() } }
    }
    pub fn newFromBitmap<T: TBitmap>(bitmap: &T) -> Image {
        unsafe { Image { ptr: wxImage_CreateFromBitmap(bitmap.ptr()) } }
    }
    pub fn newFromByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> Image {
        unsafe { Image { ptr: wxImage_CreateFromByteString(data, length, type_) } }
    }
    pub fn newFromLazyByteString(data: *mut *mut c_char, length: c_int, type_: c_int) -> Image {
        unsafe { Image { ptr: wxImage_CreateFromLazyByteString(data, length, type_) } }
    }
    pub fn newFromData(width: c_int, height: c_int, data: *mut c_void) -> Image {
        unsafe { Image { ptr: wxImage_CreateFromData(width, height, data) } }
    }
    pub fn newFromFile(name: &str) -> Image {
        let name = wxT(name);
        unsafe { Image { ptr: wxImage_CreateFromFile(name.ptr()) } }
    }
    pub fn newSized(width: c_int, height: c_int) -> Image {
        unsafe { Image { ptr: wxImage_CreateSized(width, height) } }
    }
    pub fn newFromDataEx(width: c_int, height: c_int, data: *mut c_void, isStaticData: c_int) -> Image {
        unsafe { Image { ptr: wxImage_CreateFromDataEx(width, height, data, isStaticData) } }
    }
}

pub trait TImage : TObject {
    fn convertToBitmap<T: TBitmap>(&self, bitmap: &T) {
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
    fn getSubImage<T: TImage>(&self, x: c_int, y: c_int, w: c_int, h: c_int, image: &T) {
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
    fn mirror<T: TImage>(&self, horizontally: c_int, image: &T) {
        unsafe { wxImage_Mirror(self.ptr(), horizontally, image.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxImage_IsOk(self.ptr()) }
    }
    fn paste<T: TImage>(&self, image: &T, x: c_int, y: c_int) {
        unsafe { wxImage_Paste(self.ptr(), image.ptr(), x, y) }
    }
    fn replace(&self, r1: uint8_t, g1: uint8_t, b1: uint8_t, r2: uint8_t, g2: uint8_t, b2: uint8_t) {
        unsafe { wxImage_Replace(self.ptr(), r1, g1, b1, r2, g2, b2) }
    }
    fn rescale(&self, width: c_int, height: c_int) {
        unsafe { wxImage_Rescale(self.ptr(), width, height) }
    }
    fn rotate<T: TImage>(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *mut c_void, image: &T) {
        unsafe { wxImage_Rotate(self.ptr(), angle, c_x, c_y, interpolating, offset_after_rotation, image.ptr()) }
    }
    fn rotate90<T: TImage>(&self, clockwise: c_int, image: &T) {
        unsafe { wxImage_Rotate90(self.ptr(), clockwise, image.ptr()) }
    }
    fn saveFile(&self, name: &str, type_: c_int) -> c_int {
        let name = wxT(name);
        unsafe { wxImage_SaveFile(self.ptr(), name.ptr(), type_) }
    }
    fn scale<T: TImage>(&self, width: c_int, height: c_int, image: &T) {
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

pub struct ImageHandler { ptr: *mut c_void }
impl TImageHandler for ImageHandler {}
impl TObject for ImageHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ImageHandler {
    pub fn from(ptr: *mut c_void) -> ImageHandler { ImageHandler { ptr: ptr } }
    pub fn null() -> ImageHandler { ImageHandler::from(0 as *mut c_void) }
    
}

pub trait TImageHandler : TObject {
}

pub struct ImageList { ptr: *mut c_void }
impl TImageList for ImageList {}
impl TObject for ImageList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ImageList {
    pub fn from(ptr: *mut c_void) -> ImageList { ImageList { ptr: ptr } }
    pub fn null() -> ImageList { ImageList::from(0 as *mut c_void) }
    
    pub fn new(width: c_int, height: c_int, mask: c_int, initialCount: c_int) -> ImageList {
        unsafe { ImageList { ptr: wxImageList_Create(width, height, mask, initialCount) } }
    }
}

pub trait TImageList : TObject {
    fn addBitmap<T: TBitmap, U: TBitmap>(&self, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_AddBitmap(self.ptr(), bitmap.ptr(), mask.ptr()) }
    }
    fn addIcon<T: TIcon>(&self, icon: &T) -> c_int {
        unsafe { wxImageList_AddIcon(self.ptr(), icon.ptr()) }
    }
    fn addMasked<T: TBitmap, U: TColour>(&self, bitmap: &T, maskColour: &U) -> c_int {
        unsafe { wxImageList_AddMasked(self.ptr(), bitmap.ptr(), maskColour.ptr()) }
    }
    fn draw<T: TDC>(&self, index: c_int, dc: &T, x: c_int, y: c_int, flags: c_int, solidBackground: c_int) -> c_int {
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
    fn replace<T: TBitmap, U: TBitmap>(&self, index: c_int, bitmap: &T, mask: &U) -> c_int {
        unsafe { wxImageList_Replace(self.ptr(), index, bitmap.ptr(), mask.ptr()) }
    }
    fn replaceIcon<T: TIcon>(&self, index: c_int, icon: &T) -> c_int {
        unsafe { wxImageList_ReplaceIcon(self.ptr(), index, icon.ptr()) }
    }
}

pub struct IndividualLayoutConstraint { ptr: *mut c_void }
impl TIndividualLayoutConstraint for IndividualLayoutConstraint {}
impl TObject for IndividualLayoutConstraint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl IndividualLayoutConstraint {
    pub fn from(ptr: *mut c_void) -> IndividualLayoutConstraint { IndividualLayoutConstraint { ptr: ptr } }
    pub fn null() -> IndividualLayoutConstraint { IndividualLayoutConstraint::from(0 as *mut c_void) }
    
}

pub trait TIndividualLayoutConstraint : TObject {
    fn above<T: TWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_Above(self.ptr(), sibling.ptr(), marg) }
    }
    fn absolute(&self, val: c_int) {
        unsafe { wxIndividualLayoutConstraint_Absolute(self.ptr(), val) }
    }
    fn asIs(&self) {
        unsafe { wxIndividualLayoutConstraint_AsIs(self.ptr()) }
    }
    fn below<T: TWindow>(&self, sibling: &T, marg: c_int) {
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
    fn leftOf<T: TWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_LeftOf(self.ptr(), sibling.ptr(), marg) }
    }
    fn percentOf<T: TWindow>(&self, otherW: &T, wh: c_int, per: c_int) {
        unsafe { wxIndividualLayoutConstraint_PercentOf(self.ptr(), otherW.ptr(), wh, per) }
    }
    fn resetIfWin<T: TWindow>(&self, otherW: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_ResetIfWin(self.ptr(), otherW.ptr()) }
    }
    fn rightOf<T: TWindow>(&self, sibling: &T, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_RightOf(self.ptr(), sibling.ptr(), marg) }
    }
    fn sameAs<T: TWindow>(&self, otherW: &T, edge: c_int, marg: c_int) {
        unsafe { wxIndividualLayoutConstraint_SameAs(self.ptr(), otherW.ptr(), edge, marg) }
    }
    fn satisfyConstraint<T: TWindow>(&self, constraints: *mut c_void, win: &T) -> c_int {
        unsafe { wxIndividualLayoutConstraint_SatisfyConstraint(self.ptr(), constraints, win.ptr()) }
    }
    fn set<T: TWindow>(&self, rel: c_int, otherW: &T, otherE: c_int, val: c_int, marg: c_int) {
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

pub struct InitDialogEvent { ptr: *mut c_void }
impl TInitDialogEvent for InitDialogEvent {}
impl TEvent for InitDialogEvent {}
impl TObject for InitDialogEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InitDialogEvent {
    pub fn from(ptr: *mut c_void) -> InitDialogEvent { InitDialogEvent { ptr: ptr } }
    pub fn null() -> InitDialogEvent { InitDialogEvent::from(0 as *mut c_void) }
    
}

pub trait TInitDialogEvent : TEvent {
}

pub struct JoystickEvent { ptr: *mut c_void }
impl TJoystickEvent for JoystickEvent {}
impl TEvent for JoystickEvent {}
impl TObject for JoystickEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl JoystickEvent {
    pub fn from(ptr: *mut c_void) -> JoystickEvent { JoystickEvent { ptr: ptr } }
    pub fn null() -> JoystickEvent { JoystickEvent::from(0 as *mut c_void) }
    
}

pub trait TJoystickEvent : TEvent {
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
        unsafe { Point { ptr: wxJoystickEvent_GetPosition(self.ptr()) } }
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

pub struct KeyEvent { ptr: *mut c_void }
impl TKeyEvent for KeyEvent {}
impl TEvent for KeyEvent {}
impl TObject for KeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl KeyEvent {
    pub fn from(ptr: *mut c_void) -> KeyEvent { KeyEvent { ptr: ptr } }
    pub fn null() -> KeyEvent { KeyEvent::from(0 as *mut c_void) }
    
}

pub trait TKeyEvent : TEvent {
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
        unsafe { Point { ptr: wxKeyEvent_GetPosition(self.ptr()) } }
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

pub struct LayoutConstraints { ptr: *mut c_void }
impl TLayoutConstraints for LayoutConstraints {}
impl TObject for LayoutConstraints { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LayoutConstraints {
    pub fn from(ptr: *mut c_void) -> LayoutConstraints { LayoutConstraints { ptr: ptr } }
    pub fn null() -> LayoutConstraints { LayoutConstraints::from(0 as *mut c_void) }
    
    pub fn new() -> LayoutConstraints {
        unsafe { LayoutConstraints { ptr: wxLayoutConstraints_Create() } }
    }
}

pub trait TLayoutConstraints : TObject {
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

pub struct ListBox { ptr: *mut c_void }
impl TListBox for ListBox {}
impl TControl for ListBox {}
impl TWindow for ListBox {}
impl TEvtHandler for ListBox {}
impl TObject for ListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListBox {
    pub fn from(ptr: *mut c_void) -> ListBox { ListBox { ptr: ptr } }
    pub fn null() -> ListBox { ListBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *mut *mut c_char, _stl: c_int) -> ListBox {
        unsafe { ListBox { ptr: wxListBox_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, n, str, _stl) } }
    }
}

pub trait TListBox : TControl {
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

pub struct ListCtrl { ptr: *mut c_void }
impl TListCtrl for ListCtrl {}
impl TControl for ListCtrl {}
impl TWindow for ListCtrl {}
impl TEvtHandler for ListCtrl {}
impl TObject for ListCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListCtrl {
    pub fn from(ptr: *mut c_void) -> ListCtrl { ListCtrl { ptr: ptr } }
    pub fn null() -> ListCtrl { ListCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ListCtrl {
        unsafe { ListCtrl { ptr: wxListCtrl_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TListCtrl : TControl {
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
    fn getColumn<T: TListItem>(&self, col: c_int, item: &T) -> c_int {
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
        unsafe { TextCtrl { ptr: wxListCtrl_GetEditControl(self.ptr()) } }
    }
    fn getImageList(&self, which: c_int) -> ImageList {
        unsafe { ImageList { ptr: wxListCtrl_GetImageList(self.ptr(), which) } }
    }
    fn getItem<T: TListItem>(&self, info: &T) -> c_int {
        unsafe { wxListCtrl_GetItem(self.ptr(), info.ptr()) }
    }
    fn getItemCount(&self) -> c_int {
        unsafe { wxListCtrl_GetItemCount(self.ptr()) }
    }
    fn getItemData(&self, item: c_int) -> c_int {
        unsafe { wxListCtrl_GetItemData(self.ptr(), item) }
    }
    fn getItemFont(&self, item: c_long) -> Font {
        unsafe { Font { ptr: wxListCtrl_GetItemFont(self.ptr(), item) } }
    }
    fn getItemPosition(&self, item: c_int) -> Point {
        unsafe { Point { ptr: wxListCtrl_GetItemPosition(self.ptr(), item) } }
    }
    fn getItemRect(&self, item: c_int, code: c_int) -> Rect {
        unsafe { Rect { ptr: wxListCtrl_GetItemRect(self.ptr(), item, code) } }
    }
    fn getItemSpacing(&self, isSmall: c_int) -> Size {
        unsafe { Size { ptr: wxListCtrl_GetItemSpacing(self.ptr(), isSmall) } }
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
    fn getTextColour<T: TColour>(&self, _ref: &T) {
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
    fn insertColumnFromInfo<T: TListItem>(&self, col: c_int, info: &T) -> c_int {
        unsafe { wxListCtrl_InsertColumnFromInfo(self.ptr(), col, info.ptr()) }
    }
    fn insertItem<T: TListItem>(&self, info: &T) -> c_int {
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
    fn setColumn<T: TListItem>(&self, col: c_int, item: &T) -> c_int {
        unsafe { wxListCtrl_SetColumn(self.ptr(), col, item.ptr()) }
    }
    fn setColumnWidth(&self, col: c_int, width: c_int) -> c_int {
        unsafe { wxListCtrl_SetColumnWidth(self.ptr(), col, width) }
    }
    fn setImageList<T: TImageList>(&self, imageList: &T, which: c_int) {
        unsafe { wxListCtrl_SetImageList(self.ptr(), imageList.ptr(), which) }
    }
    fn setItem(&self, index: c_int, col: c_int, label: &str, imageId: c_int) -> c_int {
        let label = wxT(label);
        unsafe { wxListCtrl_SetItem(self.ptr(), index, col, label.ptr(), imageId) }
    }
    fn setItemData(&self, item: c_int, data: c_int) -> c_int {
        unsafe { wxListCtrl_SetItemData(self.ptr(), item, data) }
    }
    fn setItemFromInfo<T: TListItem>(&self, info: &T) -> c_int {
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
    fn setTextColour<T: TColour>(&self, col: &T) {
        unsafe { wxListCtrl_SetTextColour(self.ptr(), col.ptr()) }
    }
    fn sortItems(&self, fn_: *mut c_void, eif_obj: *mut c_void) -> c_int {
        unsafe { wxListCtrl_SortItems(self.ptr(), fn_, eif_obj) }
    }
    fn updateStyle(&self) {
        unsafe { wxListCtrl_UpdateStyle(self.ptr()) }
    }
    fn assignImageList<T: TImageList>(&self, images: &T, which: c_int) {
        unsafe { wxListCtrl_AssignImageList(self.ptr(), images.ptr(), which) }
    }
    fn getColumn2<T: TListItem>(&self, col: c_int, item: &T) {
        unsafe { wxListCtrl_GetColumn2(self.ptr(), col, item.ptr()) }
    }
    fn getItem2<T: TListItem>(&self, info: &T) {
        unsafe { wxListCtrl_GetItem2(self.ptr(), info.ptr()) }
    }
    fn getItemPosition2(&self, item: c_int) -> Point {
        unsafe { Point { ptr: wxListCtrl_GetItemPosition2(self.ptr(), item) } }
    }
    fn sortItems2<T: TClosure>(&self, closure: &T) -> c_int {
        unsafe { wxListCtrl_SortItems2(self.ptr(), closure.ptr()) }
    }
}

pub struct ListEvent { ptr: *mut c_void }
impl TListEvent for ListEvent {}
impl TNotifyEvent for ListEvent {}
impl TCommandEvent for ListEvent {}
impl TEvent for ListEvent {}
impl TObject for ListEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListEvent {
    pub fn from(ptr: *mut c_void) -> ListEvent { ListEvent { ptr: ptr } }
    pub fn null() -> ListEvent { ListEvent::from(0 as *mut c_void) }
    
}

pub trait TListEvent : TNotifyEvent {
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
    fn getItem<T: TListItem>(&self, _ref: &T) {
        unsafe { wxListEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxListEvent_GetLabel(self.ptr()) }.to_str() }
    }
    fn getMask(&self) -> c_int {
        unsafe { wxListEvent_GetMask(self.ptr()) }
    }
    fn getPoint(&self) -> Point {
        unsafe { Point { ptr: wxListEvent_GetPoint(self.ptr()) } }
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

pub struct ListItem { ptr: *mut c_void }
impl TListItem for ListItem {}
impl TObject for ListItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ListItem {
    pub fn from(ptr: *mut c_void) -> ListItem { ListItem { ptr: ptr } }
    pub fn null() -> ListItem { ListItem::from(0 as *mut c_void) }
    
    pub fn new() -> ListItem {
        unsafe { ListItem { ptr: wxListItem_Create() } }
    }
}

pub trait TListItem : TObject {
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
    fn getBackgroundColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxListItem_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getColumn(&self) -> c_int {
        unsafe { wxListItem_GetColumn(self.ptr()) }
    }
    fn getData(&self) -> c_int {
        unsafe { wxListItem_GetData(self.ptr()) }
    }
    fn getFont<T: TFont>(&self, _ref: &T) {
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
    fn getTextColour<T: TColour>(&self, _ref: &T) {
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
    fn setBackgroundColour<T: TColour>(&self, colBack: &T) {
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
    fn setFont<T: TFont>(&self, font: &T) {
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
    fn setTextColour<T: TColour>(&self, colText: &T) {
        unsafe { wxListItem_SetTextColour(self.ptr(), colText.ptr()) }
    }
    fn setWidth(&self, width: c_int) {
        unsafe { wxListItem_SetWidth(self.ptr(), width) }
    }
}

pub struct Log { ptr: *mut c_void }
impl TLog for Log { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Log {
    pub fn from(ptr: *mut c_void) -> Log { Log { ptr: ptr } }
    pub fn null() -> Log { Log::from(0 as *mut c_void) }
    
    pub fn getActiveTarget() -> Log {
        unsafe { Log { ptr: wxLog_GetActiveTarget() } }
    }
}

pub trait TLog {
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
    fn isAllowedTraceMask<T: TMask>(&self, mask: &T) -> c_int {
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
    fn setActiveTarget(&self) -> Log {
        unsafe { Log { ptr: wxLog_SetActiveTarget(self.ptr()) } }
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

pub struct LogChain { ptr: *mut c_void }
impl TLogChain for LogChain {}
impl TLog for LogChain { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogChain {
    pub fn from(ptr: *mut c_void) -> LogChain { LogChain { ptr: ptr } }
    pub fn null() -> LogChain { LogChain::from(0 as *mut c_void) }
    
    pub fn new<T: TLog>(logger: &T) -> LogChain {
        unsafe { LogChain { ptr: wxLogChain_Create(logger.ptr()) } }
    }
}

pub trait TLogChain : TLog {
    fn getOldLog(&self) -> Log {
        unsafe { Log { ptr: wxLogChain_GetOldLog(self.ptr()) } }
    }
    fn isPassingMessages(&self) -> c_int {
        unsafe { wxLogChain_IsPassingMessages(self.ptr()) }
    }
    fn passMessages(&self, bDoPass: c_int) {
        unsafe { wxLogChain_PassMessages(self.ptr(), bDoPass) }
    }
    fn setLog<T: TLog>(&self, logger: &T) {
        unsafe { wxLogChain_SetLog(self.ptr(), logger.ptr()) }
    }
}

pub struct LogGUI { ptr: *mut c_void }
impl TLogGUI for LogGUI {}
impl TLog for LogGUI { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogGUI {
    pub fn from(ptr: *mut c_void) -> LogGUI { LogGUI { ptr: ptr } }
    pub fn null() -> LogGUI { LogGUI::from(0 as *mut c_void) }
    
}

pub trait TLogGUI : TLog {
}

pub struct LogNull { ptr: *mut c_void }
impl TLogNull for LogNull {}
impl TLog for LogNull { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogNull {
    pub fn from(ptr: *mut c_void) -> LogNull { LogNull { ptr: ptr } }
    pub fn null() -> LogNull { LogNull::from(0 as *mut c_void) }
    
    pub fn new() -> LogNull {
        unsafe { LogNull { ptr: wxLogNull_Create() } }
    }
}

pub trait TLogNull : TLog {
}

pub struct LogPassThrough { ptr: *mut c_void }
impl TLogPassThrough for LogPassThrough {}
impl TLogChain for LogPassThrough {}
impl TLog for LogPassThrough { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogPassThrough {
    pub fn from(ptr: *mut c_void) -> LogPassThrough { LogPassThrough { ptr: ptr } }
    pub fn null() -> LogPassThrough { LogPassThrough::from(0 as *mut c_void) }
    
}

pub trait TLogPassThrough : TLogChain {
}

pub struct LogStderr { ptr: *mut c_void }
impl TLogStderr for LogStderr {}
impl TLog for LogStderr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogStderr {
    pub fn from(ptr: *mut c_void) -> LogStderr { LogStderr { ptr: ptr } }
    pub fn null() -> LogStderr { LogStderr::from(0 as *mut c_void) }
    
    pub fn new() -> LogStderr {
        unsafe { LogStderr { ptr: wxLogStderr_Create() } }
    }
    pub fn newStdOut() -> LogStderr {
        unsafe { LogStderr { ptr: wxLogStderr_CreateStdOut() } }
    }
}

pub trait TLogStderr : TLog {
}

pub struct LogStream { ptr: *mut c_void }
impl TLogStream for LogStream {}
impl TLog for LogStream { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogStream {
    pub fn from(ptr: *mut c_void) -> LogStream { LogStream { ptr: ptr } }
    pub fn null() -> LogStream { LogStream::from(0 as *mut c_void) }
    
}

pub trait TLogStream : TLog {
}

pub struct LogTextCtrl { ptr: *mut c_void }
impl TLogTextCtrl for LogTextCtrl {}
impl TLog for LogTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogTextCtrl {
    pub fn from(ptr: *mut c_void) -> LogTextCtrl { LogTextCtrl { ptr: ptr } }
    pub fn null() -> LogTextCtrl { LogTextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TTextCtrl>(text: &T) -> LogTextCtrl {
        unsafe { LogTextCtrl { ptr: wxLogTextCtrl_Create(text.ptr()) } }
    }
}

pub trait TLogTextCtrl : TLog {
}

pub struct LogWindow { ptr: *mut c_void }
impl TLogWindow for LogWindow {}
impl TLogPassThrough for LogWindow {}
impl TLogChain for LogWindow {}
impl TLog for LogWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LogWindow {
    pub fn from(ptr: *mut c_void) -> LogWindow { LogWindow { ptr: ptr } }
    pub fn null() -> LogWindow { LogWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(parent: &T, title: *mut int8_t, showit: c_int, passthrough: c_int) -> LogWindow {
        unsafe { LogWindow { ptr: wxLogWindow_Create(parent.ptr(), title, showit, passthrough) } }
    }
}

pub trait TLogWindow : TLogPassThrough {
    fn getFrame(&self) -> Frame {
        unsafe { Frame { ptr: wxLogWindow_GetFrame(self.ptr()) } }
    }
}

pub struct MDIChildFrame { ptr: *mut c_void }
impl TMDIChildFrame for MDIChildFrame {}
impl TFrame for MDIChildFrame {}
impl TTopLevelWindow for MDIChildFrame {}
impl TWindow for MDIChildFrame {}
impl TEvtHandler for MDIChildFrame {}
impl TObject for MDIChildFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MDIChildFrame {
    pub fn from(ptr: *mut c_void) -> MDIChildFrame { MDIChildFrame { ptr: ptr } }
    pub fn null() -> MDIChildFrame { MDIChildFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> MDIChildFrame {
        let _txt = wxT(_txt);
        unsafe { MDIChildFrame { ptr: wxMDIChildFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TMDIChildFrame : TFrame {
    fn activate(&self) {
        unsafe { wxMDIChildFrame_Activate(self.ptr()) }
    }
}

pub struct MDIClientWindow { ptr: *mut c_void }
impl TMDIClientWindow for MDIClientWindow {}
impl TWindow for MDIClientWindow {}
impl TEvtHandler for MDIClientWindow {}
impl TObject for MDIClientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MDIClientWindow {
    pub fn from(ptr: *mut c_void) -> MDIClientWindow { MDIClientWindow { ptr: ptr } }
    pub fn null() -> MDIClientWindow { MDIClientWindow::from(0 as *mut c_void) }
    
}

pub trait TMDIClientWindow : TWindow {
}

pub struct MDIParentFrame { ptr: *mut c_void }
impl TMDIParentFrame for MDIParentFrame {}
impl TFrame for MDIParentFrame {}
impl TTopLevelWindow for MDIParentFrame {}
impl TWindow for MDIParentFrame {}
impl TEvtHandler for MDIParentFrame {}
impl TObject for MDIParentFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MDIParentFrame {
    pub fn from(ptr: *mut c_void) -> MDIParentFrame { MDIParentFrame { ptr: ptr } }
    pub fn null() -> MDIParentFrame { MDIParentFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> MDIParentFrame {
        let _txt = wxT(_txt);
        unsafe { MDIParentFrame { ptr: wxMDIParentFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TMDIParentFrame : TFrame {
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
        unsafe { MDIChildFrame { ptr: wxMDIParentFrame_GetActiveChild(self.ptr()) } }
    }
    fn getClientWindow(&self) -> MDIClientWindow {
        unsafe { MDIClientWindow { ptr: wxMDIParentFrame_GetClientWindow(self.ptr()) } }
    }
    fn getWindowMenu(&self) -> Menu {
        unsafe { Menu { ptr: wxMDIParentFrame_GetWindowMenu(self.ptr()) } }
    }
    fn onCreateClient(&self) -> MDIClientWindow {
        unsafe { MDIClientWindow { ptr: wxMDIParentFrame_OnCreateClient(self.ptr()) } }
    }
    fn setWindowMenu<T: TMenu>(&self, menu: &T) {
        unsafe { wxMDIParentFrame_SetWindowMenu(self.ptr(), menu.ptr()) }
    }
    fn tile(&self) {
        unsafe { wxMDIParentFrame_Tile(self.ptr()) }
    }
}

pub struct Mask { ptr: *mut c_void }
impl TMask for Mask {}
impl TObject for Mask { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Mask {
    pub fn from(ptr: *mut c_void) -> Mask { Mask { ptr: ptr } }
    pub fn null() -> Mask { Mask::from(0 as *mut c_void) }
    
    pub fn new<T: TBitmap>(bitmap: &T) -> Mask {
        unsafe { Mask { ptr: wxMask_Create(bitmap.ptr()) } }
    }
    pub fn newColoured<T: TBitmap, U: TColour>(bitmap: &T, colour: &U) -> *mut c_void {
        unsafe { wxMask_CreateColoured(bitmap.ptr(), colour.ptr()) }
    }
}

pub trait TMask : TObject {
}

pub struct MaximizeEvent { ptr: *mut c_void }
impl TMaximizeEvent for MaximizeEvent {}
impl TEvent for MaximizeEvent {}
impl TObject for MaximizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MaximizeEvent {
    pub fn from(ptr: *mut c_void) -> MaximizeEvent { MaximizeEvent { ptr: ptr } }
    pub fn null() -> MaximizeEvent { MaximizeEvent::from(0 as *mut c_void) }
    
}

pub trait TMaximizeEvent : TEvent {
}

pub struct MemoryDC { ptr: *mut c_void }
impl TMemoryDC for MemoryDC {}
impl TDC for MemoryDC {}
impl TObject for MemoryDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MemoryDC {
    pub fn from(ptr: *mut c_void) -> MemoryDC { MemoryDC { ptr: ptr } }
    pub fn null() -> MemoryDC { MemoryDC::from(0 as *mut c_void) }
    
    pub fn new() -> MemoryDC {
        unsafe { MemoryDC { ptr: wxMemoryDC_Create() } }
    }
    pub fn newCompatible<T: TDC>(dc: &T) -> MemoryDC {
        unsafe { MemoryDC { ptr: wxMemoryDC_CreateCompatible(dc.ptr()) } }
    }
    pub fn newWithBitmap<T: TBitmap>(bitmap: &T) -> MemoryDC {
        unsafe { MemoryDC { ptr: wxMemoryDC_CreateWithBitmap(bitmap.ptr()) } }
    }
}

pub trait TMemoryDC : TDC {
    fn selectObject<T: TBitmap>(&self, bitmap: &T) {
        unsafe { wxMemoryDC_SelectObject(self.ptr(), bitmap.ptr()) }
    }
}

pub struct Menu { ptr: *mut c_void }
impl TMenu for Menu {}
impl TEvtHandler for Menu {}
impl TObject for Menu { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Menu {
    pub fn from(ptr: *mut c_void) -> Menu { Menu { ptr: ptr } }
    pub fn null() -> Menu { Menu::from(0 as *mut c_void) }
    
    pub fn new(title: &str, style: c_long) -> Menu {
        let title = wxT(title);
        unsafe { Menu { ptr: wxMenu_Create(title.ptr(), style) } }
    }
}

pub trait TMenu : TEvtHandler {
    fn append(&self, id: c_int, text: &str, help: &str, isCheckable: c_int) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_Append(self.ptr(), id, text.ptr(), help.ptr(), isCheckable) }
    }
    fn appendItem<T: TMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_AppendItem(self.ptr(), _itm.ptr()) }
    }
    fn appendSeparator(&self) {
        unsafe { wxMenu_AppendSeparator(self.ptr()) }
    }
    fn appendSub<T: TMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
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
    fn deleteByItem<T: TMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DeleteByItem(self.ptr(), _itm.ptr()) }
    }
    fn deletePointer(&self) {
        unsafe { wxMenu_DeletePointer(self.ptr()) }
    }
    fn destroyById(&self, id: c_int) {
        unsafe { wxMenu_DestroyById(self.ptr(), id) }
    }
    fn destroyByItem<T: TMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_DestroyByItem(self.ptr(), _itm.ptr()) }
    }
    fn enable(&self, id: c_int, enable: c_int) {
        unsafe { wxMenu_Enable(self.ptr(), id, enable) }
    }
    fn findItem(&self, id: c_int) -> MenuItem {
        unsafe { MenuItem { ptr: wxMenu_FindItem(self.ptr(), id) } }
    }
    fn findItemByLabel(&self, itemString: &str) -> c_int {
        let itemString = wxT(itemString);
        unsafe { wxMenu_FindItemByLabel(self.ptr(), itemString.ptr()) }
    }
    fn getClientData(&self) -> ClientData {
        unsafe { ClientData { ptr: wxMenu_GetClientData(self.ptr()) } }
    }
    fn getHelpString(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenu_GetHelpString(self.ptr(), id) }.to_str() }
    }
    fn getInvokingWindow(&self) -> Window {
        unsafe { Window { ptr: wxMenu_GetInvokingWindow(self.ptr()) } }
    }
    fn getLabel(&self, id: c_int) -> ~str {
        unsafe { WxString { ptr: wxMenu_GetLabel(self.ptr(), id) }.to_str() }
    }
    fn getMenuItemCount(&self) -> size_t {
        unsafe { wxMenu_GetMenuItemCount(self.ptr()) }
    }
    fn getMenuItems<T: TList>(&self, _lst: &T) -> c_int {
        unsafe { wxMenu_GetMenuItems(self.ptr(), _lst.ptr()) }
    }
    fn getParent(&self) -> Menu {
        unsafe { Menu { ptr: wxMenu_GetParent(self.ptr()) } }
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
    fn insertItem<T: TMenuItem>(&self, pos: size_t, _itm: &T) {
        unsafe { wxMenu_InsertItem(self.ptr(), pos, _itm.ptr()) }
    }
    fn insertSub<T: TMenu>(&self, pos: size_t, id: c_int, text: &str, submenu: &T, help: &str) {
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
    fn prependItem<T: TMenuItem>(&self, _itm: &T) {
        unsafe { wxMenu_PrependItem(self.ptr(), _itm.ptr()) }
    }
    fn prependSub<T: TMenu>(&self, id: c_int, text: &str, submenu: &T, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_PrependSub(self.ptr(), id, text.ptr(), submenu.ptr(), help.ptr()) }
    }
    fn removeById<T: TMenuItem>(&self, id: c_int, _itm: &T) {
        unsafe { wxMenu_RemoveById(self.ptr(), id, _itm.ptr()) }
    }
    fn removeByItem(&self, item: *mut c_void) {
        unsafe { wxMenu_RemoveByItem(self.ptr(), item) }
    }
    fn setClientData<T: TClientData>(&self, clientData: &T) {
        unsafe { wxMenu_SetClientData(self.ptr(), clientData.ptr()) }
    }
    fn setEventHandler<T: TEvtHandler>(&self, handler: &T) {
        unsafe { wxMenu_SetEventHandler(self.ptr(), handler.ptr()) }
    }
    fn setHelpString(&self, id: c_int, helpString: &str) {
        let helpString = wxT(helpString);
        unsafe { wxMenu_SetHelpString(self.ptr(), id, helpString.ptr()) }
    }
    fn setInvokingWindow<T: TWindow>(&self, win: &T) {
        unsafe { wxMenu_SetInvokingWindow(self.ptr(), win.ptr()) }
    }
    fn setLabel(&self, id: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxMenu_SetLabel(self.ptr(), id, label.ptr()) }
    }
    fn setParent<T: TWindow>(&self, parent: &T) {
        unsafe { wxMenu_SetParent(self.ptr(), parent.ptr()) }
    }
    fn setTitle(&self, title: &str) {
        let title = wxT(title);
        unsafe { wxMenu_SetTitle(self.ptr(), title.ptr()) }
    }
    fn updateUI(&self, source: *mut c_void) {
        unsafe { wxMenu_UpdateUI(self.ptr(), source) }
    }
    fn getMenuBar(&self) -> MenuBar {
        unsafe { MenuBar { ptr: wxMenu_GetMenuBar(self.ptr()) } }
    }
    fn appendRadioItem(&self, id: c_int, text: &str, help: &str) {
        let text = wxT(text);
        let help = wxT(help);
        unsafe { wxMenu_AppendRadioItem(self.ptr(), id, text.ptr(), help.ptr()) }
    }
}

pub struct MenuBar { ptr: *mut c_void }
impl TMenuBar for MenuBar {}
impl TEvtHandler for MenuBar {}
impl TObject for MenuBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MenuBar {
    pub fn from(ptr: *mut c_void) -> MenuBar { MenuBar { ptr: ptr } }
    pub fn null() -> MenuBar { MenuBar::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> MenuBar {
        unsafe { MenuBar { ptr: wxMenuBar_Create(_style) } }
    }
}

pub trait TMenuBar : TEvtHandler {
    fn append<T: TMenu>(&self, menu: &T, title: &str) -> c_int {
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
    fn findItem(&self, id: c_int) -> MenuItem {
        unsafe { MenuItem { ptr: wxMenuBar_FindItem(self.ptr(), id) } }
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
    fn getMenu(&self, pos: c_int) -> Menu {
        unsafe { Menu { ptr: wxMenuBar_GetMenu(self.ptr(), pos) } }
    }
    fn getMenuCount(&self) -> c_int {
        unsafe { wxMenuBar_GetMenuCount(self.ptr()) }
    }
    fn insert<T: TMenu>(&self, pos: c_int, menu: &T, title: &str) -> c_int {
        let title = wxT(title);
        unsafe { wxMenuBar_Insert(self.ptr(), pos, menu.ptr(), title.ptr()) }
    }
    fn isChecked(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsChecked(self.ptr(), id) }
    }
    fn isEnabled(&self, id: c_int) -> c_int {
        unsafe { wxMenuBar_IsEnabled(self.ptr(), id) }
    }
    fn remove(&self, pos: c_int) -> Menu {
        unsafe { Menu { ptr: wxMenuBar_Remove(self.ptr(), pos) } }
    }
    fn replace<T: TMenu>(&self, pos: c_int, menu: &T, title: &str) -> Menu {
        let title = wxT(title);
        unsafe { Menu { ptr: wxMenuBar_Replace(self.ptr(), pos, menu.ptr(), title.ptr()) } }
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
    fn getFrame(&self) -> Frame {
        unsafe { Frame { ptr: wxMenuBar_GetFrame(self.ptr()) } }
    }
}

pub struct MenuEvent { ptr: *mut c_void }
impl TMenuEvent for MenuEvent {}
impl TEvent for MenuEvent {}
impl TObject for MenuEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MenuEvent {
    pub fn from(ptr: *mut c_void) -> MenuEvent { MenuEvent { ptr: ptr } }
    pub fn null() -> MenuEvent { MenuEvent::from(0 as *mut c_void) }
    
}

pub trait TMenuEvent : TEvent {
    fn getMenuId(&self) -> c_int {
        unsafe { wxMenuEvent_GetMenuId(self.ptr()) }
    }
}

pub struct MenuItem { ptr: *mut c_void }
impl TMenuItem for MenuItem {}
impl TObject for MenuItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MenuItem {
    pub fn from(ptr: *mut c_void) -> MenuItem { MenuItem { ptr: ptr } }
    pub fn null() -> MenuItem { MenuItem::from(0 as *mut c_void) }
    
    pub fn new() -> MenuItem {
        unsafe { MenuItem { ptr: wxMenuItem_Create() } }
    }
    pub fn getLabelFromText(text: *mut c_void) -> ~str {
        unsafe { WxString { ptr: wxMenuItem_GetLabelFromText(text) }.to_str() }
    }
    pub fn newSeparator() -> MenuItem {
        unsafe { MenuItem { ptr: wxMenuItem_CreateSeparator() } }
    }
    pub fn newEx<T: TMenu>(id: c_int, label: &str, help: &str, itemkind: c_int, submenu: &T) -> MenuItem {
        let label = wxT(label);
        let help = wxT(help);
        unsafe { MenuItem { ptr: wxMenuItem_CreateEx(id, label.ptr(), help.ptr(), itemkind, submenu.ptr()) } }
    }
}

pub trait TMenuItem : TObject {
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
    fn getMenu(&self) -> Menu {
        unsafe { Menu { ptr: wxMenuItem_GetMenu(self.ptr()) } }
    }
    fn getSubMenu(&self) -> Menu {
        unsafe { Menu { ptr: wxMenuItem_GetSubMenu(self.ptr()) } }
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
    fn setSubMenu<T: TMenu>(&self, menu: &T) {
        unsafe { wxMenuItem_SetSubMenu(self.ptr(), menu.ptr()) }
    }
    fn setText(&self, str: &str) {
        let str = wxT(str);
        unsafe { wxMenuItem_SetText(self.ptr(), str.ptr()) }
    }
}

pub struct MessageDialog { ptr: *mut c_void }
impl TMessageDialog for MessageDialog {}
impl TDialog for MessageDialog {}
impl TTopLevelWindow for MessageDialog {}
impl TWindow for MessageDialog {}
impl TEvtHandler for MessageDialog {}
impl TObject for MessageDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MessageDialog {
    pub fn from(ptr: *mut c_void) -> MessageDialog { MessageDialog { ptr: ptr } }
    pub fn null() -> MessageDialog { MessageDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _msg: &str, _cap: &str, _stl: c_int) -> MessageDialog {
        let _msg = wxT(_msg);
        let _cap = wxT(_cap);
        unsafe { MessageDialog { ptr: wxMessageDialog_Create(_prt.ptr(), _msg.ptr(), _cap.ptr(), _stl) } }
    }
}

pub trait TMessageDialog : TDialog {
}

pub struct Metafile { ptr: *mut c_void }
impl TMetafile for Metafile {}
impl TObject for Metafile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Metafile {
    pub fn from(ptr: *mut c_void) -> Metafile { Metafile { ptr: ptr } }
    pub fn null() -> Metafile { Metafile::from(0 as *mut c_void) }
    
    pub fn new(_file: &str) -> Metafile {
        let _file = wxT(_file);
        unsafe { Metafile { ptr: wxMetafile_Create(_file.ptr()) } }
    }
}

pub trait TMetafile : TObject {
    fn isOk(&self) -> c_int {
        unsafe { wxMetafile_IsOk(self.ptr()) }
    }
    fn play<T: TDC>(&self, _dc: &T) -> c_int {
        unsafe { wxMetafile_Play(self.ptr(), _dc.ptr()) }
    }
    fn setClipboard(&self, width: c_int, height: c_int) -> c_int {
        unsafe { wxMetafile_SetClipboard(self.ptr(), width, height) }
    }
}

pub struct MetafileDC { ptr: *mut c_void }
impl TMetafileDC for MetafileDC {}
impl TDC for MetafileDC {}
impl TObject for MetafileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MetafileDC {
    pub fn from(ptr: *mut c_void) -> MetafileDC { MetafileDC { ptr: ptr } }
    pub fn null() -> MetafileDC { MetafileDC::from(0 as *mut c_void) }
    
    pub fn new(_file: &str) -> MetafileDC {
        let _file = wxT(_file);
        unsafe { MetafileDC { ptr: wxMetafileDC_Create(_file.ptr()) } }
    }
}

pub trait TMetafileDC : TDC {
    fn close(&self) -> *mut c_void {
        unsafe { wxMetafileDC_Close(self.ptr()) }
    }
}

pub struct MimeTypesManager { ptr: *mut c_void }
impl TMimeTypesManager for MimeTypesManager { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MimeTypesManager {
    pub fn from(ptr: *mut c_void) -> MimeTypesManager { MimeTypesManager { ptr: ptr } }
    pub fn null() -> MimeTypesManager { MimeTypesManager::from(0 as *mut c_void) }
    
    pub fn new() -> MimeTypesManager {
        unsafe { MimeTypesManager { ptr: wxMimeTypesManager_Create() } }
    }
}

pub trait TMimeTypesManager {
    fn ptr(&self) -> *mut c_void;
    
    fn addFallbacks(&self, _types: *mut c_void) {
        unsafe { wxMimeTypesManager_AddFallbacks(self.ptr(), _types) }
    }
    fn enumAllFileTypes<T: TList>(&self, _lst: &T) -> c_int {
        unsafe { wxMimeTypesManager_EnumAllFileTypes(self.ptr(), _lst.ptr()) }
    }
    fn getFileTypeFromExtension(&self, _ext: &str) -> FileType {
        let _ext = wxT(_ext);
        unsafe { FileType { ptr: wxMimeTypesManager_GetFileTypeFromExtension(self.ptr(), _ext.ptr()) } }
    }
    fn getFileTypeFromMimeType(&self, _name: &str) -> FileType {
        let _name = wxT(_name);
        unsafe { FileType { ptr: wxMimeTypesManager_GetFileTypeFromMimeType(self.ptr(), _name.ptr()) } }
    }
    fn isOfType(&self, _type: &str, _wildcard: &str) -> c_int {
        let _type = wxT(_type);
        let _wildcard = wxT(_wildcard);
        unsafe { wxMimeTypesManager_IsOfType(self.ptr(), _type.ptr(), _wildcard.ptr()) }
    }
}

pub struct MiniFrame { ptr: *mut c_void }
impl TMiniFrame for MiniFrame {}
impl TFrame for MiniFrame {}
impl TTopLevelWindow for MiniFrame {}
impl TWindow for MiniFrame {}
impl TEvtHandler for MiniFrame {}
impl TObject for MiniFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MiniFrame {
    pub fn from(ptr: *mut c_void) -> MiniFrame { MiniFrame { ptr: ptr } }
    pub fn null() -> MiniFrame { MiniFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> MiniFrame {
        let _txt = wxT(_txt);
        unsafe { MiniFrame { ptr: wxMiniFrame_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TMiniFrame : TFrame {
}

pub struct MirrorDC { ptr: *mut c_void }
impl TMirrorDC for MirrorDC {}
impl TDC for MirrorDC {}
impl TObject for MirrorDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MirrorDC {
    pub fn from(ptr: *mut c_void) -> MirrorDC { MirrorDC { ptr: ptr } }
    pub fn null() -> MirrorDC { MirrorDC::from(0 as *mut c_void) }
    
    pub fn new<T: TDC>(dc: &T) -> MirrorDC {
        unsafe { MirrorDC { ptr: wxMirrorDC_Create(dc.ptr()) } }
    }
}

pub trait TMirrorDC : TDC {
}

pub struct MouseCaptureChangedEvent { ptr: *mut c_void }
impl TMouseCaptureChangedEvent for MouseCaptureChangedEvent {}
impl TEvent for MouseCaptureChangedEvent {}
impl TObject for MouseCaptureChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MouseCaptureChangedEvent {
    pub fn from(ptr: *mut c_void) -> MouseCaptureChangedEvent { MouseCaptureChangedEvent { ptr: ptr } }
    pub fn null() -> MouseCaptureChangedEvent { MouseCaptureChangedEvent::from(0 as *mut c_void) }
    
}

pub trait TMouseCaptureChangedEvent : TEvent {
}

pub struct MouseEvent { ptr: *mut c_void }
impl TMouseEvent for MouseEvent {}
impl TEvent for MouseEvent {}
impl TObject for MouseEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MouseEvent {
    pub fn from(ptr: *mut c_void) -> MouseEvent { MouseEvent { ptr: ptr } }
    pub fn null() -> MouseEvent { MouseEvent::from(0 as *mut c_void) }
    
}

pub trait TMouseEvent : TEvent {
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
    fn getLogicalPosition<T: TDC>(&self, dc: &T) -> Point {
        unsafe { Point { ptr: wxMouseEvent_GetLogicalPosition(self.ptr(), dc.ptr()) } }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxMouseEvent_GetPosition(self.ptr()) } }
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

pub struct MoveEvent { ptr: *mut c_void }
impl TMoveEvent for MoveEvent {}
impl TEvent for MoveEvent {}
impl TObject for MoveEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MoveEvent {
    pub fn from(ptr: *mut c_void) -> MoveEvent { MoveEvent { ptr: ptr } }
    pub fn null() -> MoveEvent { MoveEvent::from(0 as *mut c_void) }
    
}

pub trait TMoveEvent : TEvent {
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxMoveEvent_GetPosition(self.ptr()) } }
    }
}

pub struct NavigationKeyEvent { ptr: *mut c_void }
impl TNavigationKeyEvent for NavigationKeyEvent {}
impl TEvent for NavigationKeyEvent {}
impl TObject for NavigationKeyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NavigationKeyEvent {
    pub fn from(ptr: *mut c_void) -> NavigationKeyEvent { NavigationKeyEvent { ptr: ptr } }
    pub fn null() -> NavigationKeyEvent { NavigationKeyEvent::from(0 as *mut c_void) }
    
}

pub trait TNavigationKeyEvent : TEvent {
    fn getCurrentFocus(&self) -> *mut c_void {
        unsafe { wxNavigationKeyEvent_GetCurrentFocus(self.ptr()) }
    }
    fn getDirection(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_GetDirection(self.ptr()) }
    }
    fn isWindowChange(&self) -> c_int {
        unsafe { wxNavigationKeyEvent_IsWindowChange(self.ptr()) }
    }
    fn setCurrentFocus<T: TWindow>(&self, win: &T) {
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

pub struct Notebook { ptr: *mut c_void }
impl TNotebook for Notebook {}
impl TControl for Notebook {}
impl TWindow for Notebook {}
impl TEvtHandler for Notebook {}
impl TObject for Notebook { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Notebook {
    pub fn from(ptr: *mut c_void) -> Notebook { Notebook { ptr: ptr } }
    pub fn null() -> Notebook { Notebook::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Notebook {
        unsafe { Notebook { ptr: wxNotebook_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TNotebook : TControl {
    fn addPage<T: TWindow>(&self, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
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
    fn getImageList(&self) -> ImageList {
        unsafe { ImageList { ptr: wxNotebook_GetImageList(self.ptr()) } }
    }
    fn getPage(&self, nPage: c_int) -> Window {
        unsafe { Window { ptr: wxNotebook_GetPage(self.ptr(), nPage) } }
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
    fn insertPage<T: TWindow>(&self, nPage: c_int, pPage: &T, strText: &str, bSelect: c_int, imageId: c_int) -> c_int {
        let strText = wxT(strText);
        unsafe { wxNotebook_InsertPage(self.ptr(), nPage, pPage.ptr(), strText.ptr(), bSelect, imageId) }
    }
    fn removePage(&self, nPage: c_int) -> c_int {
        unsafe { wxNotebook_RemovePage(self.ptr(), nPage) }
    }
    fn setImageList<T: TImageList>(&self, imageList: &T) {
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
    fn assignImageList<T: TImageList>(&self, imageList: &T) {
        unsafe { wxNotebook_AssignImageList(self.ptr(), imageList.ptr()) }
    }
}

pub struct NotebookEvent { ptr: *mut c_void }
impl TNotebookEvent for NotebookEvent {}
impl TNotifyEvent for NotebookEvent {}
impl TCommandEvent for NotebookEvent {}
impl TEvent for NotebookEvent {}
impl TObject for NotebookEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NotebookEvent {
    pub fn from(ptr: *mut c_void) -> NotebookEvent { NotebookEvent { ptr: ptr } }
    pub fn null() -> NotebookEvent { NotebookEvent::from(0 as *mut c_void) }
    
}

pub trait TNotebookEvent : TNotifyEvent {
}

pub struct NotifyEvent { ptr: *mut c_void }
impl TNotifyEvent for NotifyEvent {}
impl TCommandEvent for NotifyEvent {}
impl TEvent for NotifyEvent {}
impl TObject for NotifyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NotifyEvent {
    pub fn from(ptr: *mut c_void) -> NotifyEvent { NotifyEvent { ptr: ptr } }
    pub fn null() -> NotifyEvent { NotifyEvent::from(0 as *mut c_void) }
    
}

pub trait TNotifyEvent : TCommandEvent {
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

pub struct PageSetupDialog { ptr: *mut c_void }
impl TPageSetupDialog for PageSetupDialog {}
impl TDialog for PageSetupDialog {}
impl TTopLevelWindow for PageSetupDialog {}
impl TWindow for PageSetupDialog {}
impl TEvtHandler for PageSetupDialog {}
impl TObject for PageSetupDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PageSetupDialog {
    pub fn from(ptr: *mut c_void) -> PageSetupDialog { PageSetupDialog { ptr: ptr } }
    pub fn null() -> PageSetupDialog { PageSetupDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TPageSetupDialogData>(parent: &T, data: &U) -> PageSetupDialog {
        unsafe { PageSetupDialog { ptr: wxPageSetupDialog_Create(parent.ptr(), data.ptr()) } }
    }
}

pub trait TPageSetupDialog : TDialog {
    fn getPageSetupData<T: TPageSetupDialogData>(&self, _ref: &T) {
        unsafe { wxPageSetupDialog_GetPageSetupData(self.ptr(), _ref.ptr()) }
    }
}

pub struct PageSetupDialogData { ptr: *mut c_void }
impl TPageSetupDialogData for PageSetupDialogData {}
impl TObject for PageSetupDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PageSetupDialogData {
    pub fn from(ptr: *mut c_void) -> PageSetupDialogData { PageSetupDialogData { ptr: ptr } }
    pub fn null() -> PageSetupDialogData { PageSetupDialogData::from(0 as *mut c_void) }
    
    pub fn new() -> PageSetupDialogData {
        unsafe { PageSetupDialogData { ptr: wxPageSetupDialogData_Create() } }
    }
    pub fn newFromData<T: TPrintData>(printData: &T) -> PageSetupDialogData {
        unsafe { PageSetupDialogData { ptr: wxPageSetupDialogData_CreateFromData(printData.ptr()) } }
    }
}

pub trait TPageSetupDialogData : TObject {
    fn assign<T: TPageSetupDialogData>(&self, data: &T) {
        unsafe { wxPageSetupDialogData_Assign(self.ptr(), data.ptr()) }
    }
    fn assignData<T: TPrintData>(&self, printData: &T) {
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
        unsafe { Point { ptr: wxPageSetupDialogData_GetMarginBottomRight(self.ptr()) } }
    }
    fn getMarginTopLeft(&self) -> Point {
        unsafe { Point { ptr: wxPageSetupDialogData_GetMarginTopLeft(self.ptr()) } }
    }
    fn getMinMarginBottomRight(&self) -> Point {
        unsafe { Point { ptr: wxPageSetupDialogData_GetMinMarginBottomRight(self.ptr()) } }
    }
    fn getMinMarginTopLeft(&self) -> Point {
        unsafe { Point { ptr: wxPageSetupDialogData_GetMinMarginTopLeft(self.ptr()) } }
    }
    fn getPaperId(&self) -> c_int {
        unsafe { wxPageSetupDialogData_GetPaperId(self.ptr()) }
    }
    fn getPaperSize(&self) -> Size {
        unsafe { Size { ptr: wxPageSetupDialogData_GetPaperSize(self.ptr()) } }
    }
    fn getPrintData<T: TPrintData>(&self, _ref: &T) {
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
    fn setPrintData<T: TPrintData>(&self, printData: &T) {
        unsafe { wxPageSetupDialogData_SetPrintData(self.ptr(), printData.ptr()) }
    }
}

pub struct PaintDC { ptr: *mut c_void }
impl TPaintDC for PaintDC {}
impl TWindowDC for PaintDC {}
impl TDC for PaintDC {}
impl TObject for PaintDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PaintDC {
    pub fn from(ptr: *mut c_void) -> PaintDC { PaintDC { ptr: ptr } }
    pub fn null() -> PaintDC { PaintDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(win: &T) -> PaintDC {
        unsafe { PaintDC { ptr: wxPaintDC_Create(win.ptr()) } }
    }
}

pub trait TPaintDC : TWindowDC {
}

pub struct PaintEvent { ptr: *mut c_void }
impl TPaintEvent for PaintEvent {}
impl TEvent for PaintEvent {}
impl TObject for PaintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PaintEvent {
    pub fn from(ptr: *mut c_void) -> PaintEvent { PaintEvent { ptr: ptr } }
    pub fn null() -> PaintEvent { PaintEvent::from(0 as *mut c_void) }
    
}

pub trait TPaintEvent : TEvent {
}

pub struct Palette { ptr: *mut c_void }
impl TPalette for Palette {}
impl TGDIObject for Palette {}
impl TObject for Palette { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Palette {
    pub fn from(ptr: *mut c_void) -> Palette { Palette { ptr: ptr } }
    pub fn null() -> Palette { Palette::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Palette {
        unsafe { Palette { ptr: wxPalette_CreateDefault() } }
    }
    pub fn newRGB(n: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> Palette {
        unsafe { Palette { ptr: wxPalette_CreateRGB(n, red, green, blue) } }
    }
}

pub trait TPalette : TGDIObject {
    fn assign<T: TPalette>(&self, palette: &T) {
        unsafe { wxPalette_Assign(self.ptr(), palette.ptr()) }
    }
    fn getPixel(&self, red: uint8_t, green: uint8_t, blue: uint8_t) -> c_int {
        unsafe { wxPalette_GetPixel(self.ptr(), red, green, blue) }
    }
    fn getRGB(&self, pixel: c_int, red: *mut c_void, green: *mut c_void, blue: *mut c_void) -> c_int {
        unsafe { wxPalette_GetRGB(self.ptr(), pixel, red, green, blue) }
    }
    fn isEqual<T: TPalette>(&self, palette: &T) -> c_int {
        unsafe { wxPalette_IsEqual(self.ptr(), palette.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPalette_IsOk(self.ptr()) }
    }
}

pub struct PaletteChangedEvent { ptr: *mut c_void }
impl TPaletteChangedEvent for PaletteChangedEvent {}
impl TEvent for PaletteChangedEvent {}
impl TObject for PaletteChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PaletteChangedEvent {
    pub fn from(ptr: *mut c_void) -> PaletteChangedEvent { PaletteChangedEvent { ptr: ptr } }
    pub fn null() -> PaletteChangedEvent { PaletteChangedEvent::from(0 as *mut c_void) }
    
}

pub trait TPaletteChangedEvent : TEvent {
    fn getChangedWindow(&self) -> *mut c_void {
        unsafe { wxPaletteChangedEvent_GetChangedWindow(self.ptr()) }
    }
    fn setChangedWindow<T: TWindow>(&self, win: &T) {
        unsafe { wxPaletteChangedEvent_SetChangedWindow(self.ptr(), win.ptr()) }
    }
}

pub struct Panel { ptr: *mut c_void }
impl TPanel for Panel {}
impl TWindow for Panel {}
impl TEvtHandler for Panel {}
impl TObject for Panel { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Panel {
    pub fn from(ptr: *mut c_void) -> Panel { Panel { ptr: ptr } }
    pub fn null() -> Panel { Panel::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Panel {
        unsafe { Panel { ptr: wxPanel_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TPanel : TWindow {
}

pub struct Pen { ptr: *mut c_void }
impl TPen for Pen {}
impl TGDIObject for Pen {}
impl TObject for Pen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Pen {
    pub fn from(ptr: *mut c_void) -> Pen { Pen { ptr: ptr } }
    pub fn null() -> Pen { Pen::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Pen {
        unsafe { Pen { ptr: wxPen_CreateDefault() } }
    }
    pub fn newFromBitmap<T: TBitmap>(stipple: &T, width: c_int) -> Pen {
        unsafe { Pen { ptr: wxPen_CreateFromBitmap(stipple.ptr(), width) } }
    }
    pub fn newFromColour<T: TColour>(col: &T, width: c_int, style: c_int) -> Pen {
        unsafe { Pen { ptr: wxPen_CreateFromColour(col.ptr(), width, style) } }
    }
    pub fn newFromStock(id: c_int) -> Pen {
        unsafe { Pen { ptr: wxPen_CreateFromStock(id) } }
    }
}

pub trait TPen : TGDIObject {
    fn assign<T: TPen>(&self, pen: &T) {
        unsafe { wxPen_Assign(self.ptr(), pen.ptr()) }
    }
    fn getCap(&self) -> c_int {
        unsafe { wxPen_GetCap(self.ptr()) }
    }
    fn getColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxPen_GetColour(self.ptr(), _ref.ptr()) }
    }
    fn getDashes(&self, ptr: *mut c_void) -> c_int {
        unsafe { wxPen_GetDashes(self.ptr(), ptr) }
    }
    fn getJoin(&self) -> c_int {
        unsafe { wxPen_GetJoin(self.ptr()) }
    }
    fn getStipple<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxPen_GetStipple(self.ptr(), _ref.ptr()) }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxPen_GetStyle(self.ptr()) }
    }
    fn getWidth(&self) -> c_int {
        unsafe { wxPen_GetWidth(self.ptr()) }
    }
    fn isEqual<T: TPen>(&self, pen: &T) -> c_int {
        unsafe { wxPen_IsEqual(self.ptr(), pen.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPen_IsOk(self.ptr()) }
    }
    fn setCap(&self, cap: c_int) {
        unsafe { wxPen_SetCap(self.ptr(), cap) }
    }
    fn setColour<T: TColour>(&self, col: &T) {
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
    fn setStipple<T: TBitmap>(&self, stipple: &T) {
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

pub struct PenList { ptr: *mut c_void }
impl TPenList for PenList {}
impl TList for PenList {}
impl TObject for PenList { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PenList {
    pub fn from(ptr: *mut c_void) -> PenList { PenList { ptr: ptr } }
    pub fn null() -> PenList { PenList::from(0 as *mut c_void) }
    
}

pub trait TPenList : TList {
}

pub struct Point { ptr: *mut c_void }
impl TPoint for Point { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Point {
    pub fn from(ptr: *mut c_void) -> Point { Point { ptr: ptr } }
    pub fn null() -> Point { Point::from(0 as *mut c_void) }
    
    pub fn new(xx: c_int, yy: c_int) -> Point {
        unsafe { Point { ptr: wxPoint_Create(xx, yy) } }
    }
}

pub trait TPoint {
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

pub struct PopupTransientWindow { ptr: *mut c_void }
impl TPopupTransientWindow for PopupTransientWindow {}
impl TPopupWindow for PopupTransientWindow {}
impl TWindow for PopupTransientWindow {}
impl TEvtHandler for PopupTransientWindow {}
impl TObject for PopupTransientWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PopupTransientWindow {
    pub fn from(ptr: *mut c_void) -> PopupTransientWindow { PopupTransientWindow { ptr: ptr } }
    pub fn null() -> PopupTransientWindow { PopupTransientWindow::from(0 as *mut c_void) }
    
}

pub trait TPopupTransientWindow : TPopupWindow {
}

pub struct PopupWindow { ptr: *mut c_void }
impl TPopupWindow for PopupWindow {}
impl TWindow for PopupWindow {}
impl TEvtHandler for PopupWindow {}
impl TObject for PopupWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PopupWindow {
    pub fn from(ptr: *mut c_void) -> PopupWindow { PopupWindow { ptr: ptr } }
    pub fn null() -> PopupWindow { PopupWindow::from(0 as *mut c_void) }
    
}

pub trait TPopupWindow : TWindow {
}

pub struct PostScriptDC { ptr: *mut c_void }
impl TPostScriptDC for PostScriptDC {}
impl TDC for PostScriptDC {}
impl TObject for PostScriptDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PostScriptDC {
    pub fn from(ptr: *mut c_void) -> PostScriptDC { PostScriptDC { ptr: ptr } }
    pub fn null() -> PostScriptDC { PostScriptDC::from(0 as *mut c_void) }
    
    pub fn new<T: TPrintData>(data: &T) -> PostScriptDC {
        unsafe { PostScriptDC { ptr: wxPostScriptDC_Create(data.ptr()) } }
    }
}

pub trait TPostScriptDC : TDC {
    fn setResolution(&self, ppi: c_int) {
        unsafe { wxPostScriptDC_SetResolution(self.ptr(), ppi) }
    }
    fn getResolution(&self) -> c_int {
        unsafe { wxPostScriptDC_GetResolution(self.ptr()) }
    }
}

pub struct PreviewCanvas { ptr: *mut c_void }
impl TPreviewCanvas for PreviewCanvas {}
impl TScrolledWindow for PreviewCanvas {}
impl TPanel for PreviewCanvas {}
impl TWindow for PreviewCanvas {}
impl TEvtHandler for PreviewCanvas {}
impl TObject for PreviewCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PreviewCanvas {
    pub fn from(ptr: *mut c_void) -> PreviewCanvas { PreviewCanvas { ptr: ptr } }
    pub fn null() -> PreviewCanvas { PreviewCanvas::from(0 as *mut c_void) }
    
    pub fn new<T: TPrintPreview, U: TWindow>(preview: &T, parent: &U, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> PreviewCanvas {
        unsafe { PreviewCanvas { ptr: wxPreviewCanvas_Create(preview.ptr(), parent.ptr(), x, y, w, h, style) } }
    }
}

pub trait TPreviewCanvas : TScrolledWindow {
}

pub struct PreviewControlBar { ptr: *mut c_void }
impl TPreviewControlBar for PreviewControlBar {}
impl TPanel for PreviewControlBar {}
impl TWindow for PreviewControlBar {}
impl TEvtHandler for PreviewControlBar {}
impl TObject for PreviewControlBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PreviewControlBar {
    pub fn from(ptr: *mut c_void) -> PreviewControlBar { PreviewControlBar { ptr: ptr } }
    pub fn null() -> PreviewControlBar { PreviewControlBar::from(0 as *mut c_void) }
    
}

pub trait TPreviewControlBar : TPanel {
}

pub struct PreviewFrame { ptr: *mut c_void }
impl TPreviewFrame for PreviewFrame {}
impl TFrame for PreviewFrame {}
impl TTopLevelWindow for PreviewFrame {}
impl TWindow for PreviewFrame {}
impl TEvtHandler for PreviewFrame {}
impl TObject for PreviewFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PreviewFrame {
    pub fn from(ptr: *mut c_void) -> PreviewFrame { PreviewFrame { ptr: ptr } }
    pub fn null() -> PreviewFrame { PreviewFrame::from(0 as *mut c_void) }
    
    pub fn new<T: TPrintPreview, U: TFrame>(preview: &T, parent: &U, title: &str, x: c_int, y: c_int, width: c_int, height: c_int, style: c_int, name: &str) -> PreviewFrame {
        let title = wxT(title);
        let name = wxT(name);
        unsafe { PreviewFrame { ptr: wxPreviewFrame_Create(preview.ptr(), parent.ptr(), title.ptr(), x, y, width, height, style, name.ptr()) } }
    }
}

pub trait TPreviewFrame : TFrame {
    fn initialize(&self) {
        unsafe { wxPreviewFrame_Initialize(self.ptr()) }
    }
}

pub struct PrintData { ptr: *mut c_void }
impl TPrintData for PrintData {}
impl TObject for PrintData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintData {
    pub fn from(ptr: *mut c_void) -> PrintData { PrintData { ptr: ptr } }
    pub fn null() -> PrintData { PrintData::from(0 as *mut c_void) }
    
    pub fn new() -> PrintData {
        unsafe { PrintData { ptr: wxPrintData_Create() } }
    }
}

pub trait TPrintData : TObject {
    fn assign<T: TPrintData>(&self, data: &T) {
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
    fn getPaperSize(&self) -> Size {
        unsafe { Size { ptr: wxPrintData_GetPaperSize(self.ptr()) } }
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
    fn setPreviewCommand<T: TCommand>(&self, command: &T) {
        unsafe { wxPrintData_SetPreviewCommand(self.ptr(), command.ptr()) }
    }
    fn setPrintMode(&self, printMode: c_int) {
        unsafe { wxPrintData_SetPrintMode(self.ptr(), printMode) }
    }
    fn setPrinterCommand<T: TCommand>(&self, command: &T) {
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

pub struct PostScriptPrintNativeData { ptr: *mut c_void }
impl TPostScriptPrintNativeData for PostScriptPrintNativeData {}
impl TObject for PostScriptPrintNativeData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PostScriptPrintNativeData {
    pub fn from(ptr: *mut c_void) -> PostScriptPrintNativeData { PostScriptPrintNativeData { ptr: ptr } }
    pub fn null() -> PostScriptPrintNativeData { PostScriptPrintNativeData::from(0 as *mut c_void) }
    
    pub fn new() -> PostScriptPrintNativeData {
        unsafe { PostScriptPrintNativeData { ptr: wxPostScriptPrintNativeData_Create() } }
    }
}

pub trait TPostScriptPrintNativeData : TObject {
}

pub struct PrintDialog { ptr: *mut c_void }
impl TPrintDialog for PrintDialog {}
impl TDialog for PrintDialog {}
impl TTopLevelWindow for PrintDialog {}
impl TWindow for PrintDialog {}
impl TEvtHandler for PrintDialog {}
impl TObject for PrintDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintDialog {
    pub fn from(ptr: *mut c_void) -> PrintDialog { PrintDialog { ptr: ptr } }
    pub fn null() -> PrintDialog { PrintDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TPrintDialogData>(parent: &T, data: &U) -> PrintDialog {
        unsafe { PrintDialog { ptr: wxPrintDialog_Create(parent.ptr(), data.ptr()) } }
    }
}

pub trait TPrintDialog : TDialog {
    fn getPrintDC(&self) -> DC {
        unsafe { DC { ptr: wxPrintDialog_GetPrintDC(self.ptr()) } }
    }
    fn getPrintData<T: TPrintData>(&self, _ref: &T) {
        unsafe { wxPrintDialog_GetPrintData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintDialogData(&self) -> PrintDialogData {
        unsafe { PrintDialogData { ptr: wxPrintDialog_GetPrintDialogData(self.ptr()) } }
    }
}

pub struct PrintDialogData { ptr: *mut c_void }
impl TPrintDialogData for PrintDialogData {}
impl TObject for PrintDialogData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintDialogData {
    pub fn from(ptr: *mut c_void) -> PrintDialogData { PrintDialogData { ptr: ptr } }
    pub fn null() -> PrintDialogData { PrintDialogData::from(0 as *mut c_void) }
    
    pub fn newDefault() -> PrintDialogData {
        unsafe { PrintDialogData { ptr: wxPrintDialogData_CreateDefault() } }
    }
    pub fn newFromData<T: TPrintData>(printData: &T) -> PrintDialogData {
        unsafe { PrintDialogData { ptr: wxPrintDialogData_CreateFromData(printData.ptr()) } }
    }
}

pub trait TPrintDialogData : TObject {
    fn assign<T: TPrintDialogData>(&self, data: &T) {
        unsafe { wxPrintDialogData_Assign(self.ptr(), data.ptr()) }
    }
    fn assignData<T: TPrintData>(&self, data: &T) {
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
    fn getPrintData<T: TPrintData>(&self, _ref: &T) {
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
    fn setPrintData<T: TPrintData>(&self, printData: &T) {
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

pub struct PrintPreview { ptr: *mut c_void }
impl TPrintPreview for PrintPreview {}
impl TObject for PrintPreview { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrintPreview {
    pub fn from(ptr: *mut c_void) -> PrintPreview { PrintPreview { ptr: ptr } }
    pub fn null() -> PrintPreview { PrintPreview::from(0 as *mut c_void) }
    
    pub fn newFromData<T: TPrintout, U: TPrintout, V: TPrintData>(printout: &T, printoutForPrinting: &U, data: &V) -> PrintPreview {
        unsafe { PrintPreview { ptr: wxPrintPreview_CreateFromData(printout.ptr(), printoutForPrinting.ptr(), data.ptr()) } }
    }
    pub fn newFromDialogData<T: TPrintout, U: TPrintout, V: TPrintDialogData>(printout: &T, printoutForPrinting: &U, data: &V) -> PrintPreview {
        unsafe { PrintPreview { ptr: wxPrintPreview_CreateFromDialogData(printout.ptr(), printoutForPrinting.ptr(), data.ptr()) } }
    }
}

pub trait TPrintPreview : TObject {
    fn determineScaling(&self) {
        unsafe { wxPrintPreview_DetermineScaling(self.ptr()) }
    }
    fn drawBlankPage<T: TPreviewCanvas, U: TDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_DrawBlankPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    fn getCanvas(&self) -> PreviewCanvas {
        unsafe { PreviewCanvas { ptr: wxPrintPreview_GetCanvas(self.ptr()) } }
    }
    fn getCurrentPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetCurrentPage(self.ptr()) }
    }
    fn getFrame(&self) -> Frame {
        unsafe { Frame { ptr: wxPrintPreview_GetFrame(self.ptr()) } }
    }
    fn getMaxPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMaxPage(self.ptr()) }
    }
    fn getMinPage(&self) -> c_int {
        unsafe { wxPrintPreview_GetMinPage(self.ptr()) }
    }
    fn getPrintDialogData<T: TPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrintPreview_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    fn getPrintout(&self) -> Printout {
        unsafe { Printout { ptr: wxPrintPreview_GetPrintout(self.ptr()) } }
    }
    fn getPrintoutForPrinting(&self) -> Printout {
        unsafe { Printout { ptr: wxPrintPreview_GetPrintoutForPrinting(self.ptr()) } }
    }
    fn getZoom(&self) -> c_int {
        unsafe { wxPrintPreview_GetZoom(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxPrintPreview_IsOk(self.ptr()) }
    }
    fn paintPage<T: TPrintPreview, U: TDC>(&self, canvas: &T, dc: &U) -> c_int {
        unsafe { wxPrintPreview_PaintPage(self.ptr(), canvas.ptr(), dc.ptr()) }
    }
    fn print(&self, interactive: c_int) -> c_int {
        unsafe { wxPrintPreview_Print(self.ptr(), interactive) }
    }
    fn renderPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_RenderPage(self.ptr(), pageNum) }
    }
    fn setCanvas<T: TPreviewCanvas>(&self, canvas: &T) {
        unsafe { wxPrintPreview_SetCanvas(self.ptr(), canvas.ptr()) }
    }
    fn setCurrentPage(&self, pageNum: c_int) -> c_int {
        unsafe { wxPrintPreview_SetCurrentPage(self.ptr(), pageNum) }
    }
    fn setFrame<T: TFrame>(&self, frame: &T) {
        unsafe { wxPrintPreview_SetFrame(self.ptr(), frame.ptr()) }
    }
    fn setOk(&self, ok: c_int) {
        unsafe { wxPrintPreview_SetOk(self.ptr(), ok) }
    }
    fn setPrintout<T: TPrintout>(&self, printout: &T) {
        unsafe { wxPrintPreview_SetPrintout(self.ptr(), printout.ptr()) }
    }
    fn setZoom(&self, percent: c_int) {
        unsafe { wxPrintPreview_SetZoom(self.ptr(), percent) }
    }
}

pub struct Printer { ptr: *mut c_void }
impl TPrinter for Printer {}
impl TObject for Printer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Printer {
    pub fn from(ptr: *mut c_void) -> Printer { Printer { ptr: ptr } }
    pub fn null() -> Printer { Printer::from(0 as *mut c_void) }
    
    pub fn new<T: TPrintDialogData>(data: &T) -> Printer {
        unsafe { Printer { ptr: wxPrinter_Create(data.ptr()) } }
    }
}

pub trait TPrinter : TObject {
    fn newAbortWindow<T: TWindow, U: TPrintout>(&self, parent: &T, printout: &U) -> Window {
        unsafe { Window { ptr: wxPrinter_CreateAbortWindow(self.ptr(), parent.ptr(), printout.ptr()) } }
    }
    fn getAbort(&self) -> c_int {
        unsafe { wxPrinter_GetAbort(self.ptr()) }
    }
    fn getLastError(&self) -> c_int {
        unsafe { wxPrinter_GetLastError(self.ptr()) }
    }
    fn getPrintDialogData<T: TPrintDialogData>(&self, _ref: &T) {
        unsafe { wxPrinter_GetPrintDialogData(self.ptr(), _ref.ptr()) }
    }
    fn print<T: TWindow, U: TPrintout>(&self, parent: &T, printout: &U, prompt: c_int) -> c_int {
        unsafe { wxPrinter_Print(self.ptr(), parent.ptr(), printout.ptr(), prompt) }
    }
    fn printDialog<T: TWindow>(&self, parent: &T) -> DC {
        unsafe { DC { ptr: wxPrinter_PrintDialog(self.ptr(), parent.ptr()) } }
    }
    fn reportError<T: TWindow, U: TPrintout>(&self, parent: &T, printout: &U, message: &str) {
        let message = wxT(message);
        unsafe { wxPrinter_ReportError(self.ptr(), parent.ptr(), printout.ptr(), message.ptr()) }
    }
    fn setup<T: TWindow>(&self, parent: &T) -> c_int {
        unsafe { wxPrinter_Setup(self.ptr(), parent.ptr()) }
    }
}

pub struct PrinterDC { ptr: *mut c_void }
impl TPrinterDC for PrinterDC {}
impl TDC for PrinterDC {}
impl TObject for PrinterDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrinterDC {
    pub fn from(ptr: *mut c_void) -> PrinterDC { PrinterDC { ptr: ptr } }
    pub fn null() -> PrinterDC { PrinterDC::from(0 as *mut c_void) }
    
    pub fn new<T: TPrintData>(data: &T) -> PrinterDC {
        unsafe { PrinterDC { ptr: wxPrinterDC_Create(data.ptr()) } }
    }
}

pub trait TPrinterDC : TDC {
    fn getPaperRect(&self) -> Rect {
        unsafe { Rect { ptr: wxPrinterDC_GetPaperRect(self.ptr()) } }
    }
}

pub struct Printout { ptr: *mut c_void }
impl TPrintout for Printout {}
impl TObject for Printout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Printout {
    pub fn from(ptr: *mut c_void) -> Printout { Printout { ptr: ptr } }
    pub fn null() -> Printout { Printout::from(0 as *mut c_void) }
    
}

pub trait TPrintout : TObject {
    fn getDC(&self) -> DC {
        unsafe { DC { ptr: wxPrintout_GetDC(self.ptr()) } }
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
    fn setDC<T: TDC>(&self, dc: &T) {
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

pub struct PrivateDropTarget { ptr: *mut c_void }
impl TPrivateDropTarget for PrivateDropTarget {}
impl TDropTarget for PrivateDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PrivateDropTarget {
    pub fn from(ptr: *mut c_void) -> PrivateDropTarget { PrivateDropTarget { ptr: ptr } }
    pub fn null() -> PrivateDropTarget { PrivateDropTarget::from(0 as *mut c_void) }
    
}

pub trait TPrivateDropTarget : TDropTarget {
}

pub struct Process { ptr: *mut c_void }
impl TProcess for Process {}
impl TEvtHandler for Process {}
impl TObject for Process { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Process {
    pub fn from(ptr: *mut c_void) -> Process { Process { ptr: ptr } }
    pub fn null() -> Process { Process::from(0 as *mut c_void) }
    
    pub fn newDefault<T: TWindow>(_prt: &T, _id: c_int) -> Process {
        unsafe { Process { ptr: wxProcess_CreateDefault(_prt.ptr(), _id) } }
    }
    pub fn newRedirect<T: TWindow>(_prt: &T, _rdr: c_int) -> Process {
        unsafe { Process { ptr: wxProcess_CreateRedirect(_prt.ptr(), _rdr) } }
    }
    pub fn open(cmd: &str, flags: c_int) -> Process {
        let cmd = wxT(cmd);
        unsafe { Process { ptr: wxProcess_Open(cmd.ptr(), flags) } }
    }
}

pub trait TProcess : TEvtHandler {
    fn closeOutput(&self) {
        unsafe { wxProcess_CloseOutput(self.ptr()) }
    }
    fn detach(&self) {
        unsafe { wxProcess_Detach(self.ptr()) }
    }
    fn getErrorStream(&self) -> InputStream {
        unsafe { InputStream { ptr: wxProcess_GetErrorStream(self.ptr()) } }
    }
    fn getInputStream(&self) -> InputStream {
        unsafe { InputStream { ptr: wxProcess_GetInputStream(self.ptr()) } }
    }
    fn getOutputStream(&self) -> OutputStream {
        unsafe { OutputStream { ptr: wxProcess_GetOutputStream(self.ptr()) } }
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

pub struct ProcessEvent { ptr: *mut c_void }
impl TProcessEvent for ProcessEvent {}
impl TEvent for ProcessEvent {}
impl TObject for ProcessEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ProcessEvent {
    pub fn from(ptr: *mut c_void) -> ProcessEvent { ProcessEvent { ptr: ptr } }
    pub fn null() -> ProcessEvent { ProcessEvent::from(0 as *mut c_void) }
    
}

pub trait TProcessEvent : TEvent {
    fn getExitCode(&self) -> c_int {
        unsafe { wxProcessEvent_GetExitCode(self.ptr()) }
    }
    fn getPid(&self) -> c_int {
        unsafe { wxProcessEvent_GetPid(self.ptr()) }
    }
}

pub struct ProgressDialog { ptr: *mut c_void }
impl TProgressDialog for ProgressDialog {}
impl TFrame for ProgressDialog {}
impl TTopLevelWindow for ProgressDialog {}
impl TWindow for ProgressDialog {}
impl TEvtHandler for ProgressDialog {}
impl TObject for ProgressDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ProgressDialog {
    pub fn from(ptr: *mut c_void) -> ProgressDialog { ProgressDialog { ptr: ptr } }
    pub fn null() -> ProgressDialog { ProgressDialog::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(title: &str, message: &str, max: c_int, parent: &T, style: c_int) -> ProgressDialog {
        let title = wxT(title);
        let message = wxT(message);
        unsafe { ProgressDialog { ptr: wxProgressDialog_Create(title.ptr(), message.ptr(), max, parent.ptr(), style) } }
    }
}

pub trait TProgressDialog : TFrame {
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

pub struct Quantize { ptr: *mut c_void }
impl TQuantize for Quantize {}
impl TObject for Quantize { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Quantize {
    pub fn from(ptr: *mut c_void) -> Quantize { Quantize { ptr: ptr } }
    pub fn null() -> Quantize { Quantize::from(0 as *mut c_void) }
    
}

pub trait TQuantize : TObject {
}

pub struct QueryNewPaletteEvent { ptr: *mut c_void }
impl TQueryNewPaletteEvent for QueryNewPaletteEvent {}
impl TEvent for QueryNewPaletteEvent {}
impl TObject for QueryNewPaletteEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl QueryNewPaletteEvent {
    pub fn from(ptr: *mut c_void) -> QueryNewPaletteEvent { QueryNewPaletteEvent { ptr: ptr } }
    pub fn null() -> QueryNewPaletteEvent { QueryNewPaletteEvent::from(0 as *mut c_void) }
    
}

pub trait TQueryNewPaletteEvent : TEvent {
    fn getPaletteRealized(&self) -> c_int {
        unsafe { wxQueryNewPaletteEvent_GetPaletteRealized(self.ptr()) }
    }
    fn setPaletteRealized(&self, realized: c_int) {
        unsafe { wxQueryNewPaletteEvent_SetPaletteRealized(self.ptr(), realized) }
    }
}

pub struct RadioBox { ptr: *mut c_void }
impl TRadioBox for RadioBox {}
impl TControl for RadioBox {}
impl TWindow for RadioBox {}
impl TEvtHandler for RadioBox {}
impl TObject for RadioBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RadioBox {
    pub fn from(ptr: *mut c_void) -> RadioBox { RadioBox { ptr: ptr } }
    pub fn null() -> RadioBox { RadioBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *mut *mut c_char, _dim: c_int, _stl: c_int) -> RadioBox {
        let _txt = wxT(_txt);
        unsafe { RadioBox { ptr: wxRadioBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, n, _str, _dim, _stl) } }
    }
}

pub trait TRadioBox : TControl {
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
    fn setItemBitmap<T: TBitmap>(&self, item: c_int, bitmap: &T) {
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

pub struct RadioButton { ptr: *mut c_void }
impl TRadioButton for RadioButton {}
impl TControl for RadioButton {}
impl TWindow for RadioButton {}
impl TEvtHandler for RadioButton {}
impl TObject for RadioButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RadioButton {
    pub fn from(ptr: *mut c_void) -> RadioButton { RadioButton { ptr: ptr } }
    pub fn null() -> RadioButton { RadioButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> RadioButton {
        let _txt = wxT(_txt);
        unsafe { RadioButton { ptr: wxRadioButton_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TRadioButton : TControl {
    fn getValue(&self) -> c_int {
        unsafe { wxRadioButton_GetValue(self.ptr()) }
    }
    fn setValue(&self, value: c_int) {
        unsafe { wxRadioButton_SetValue(self.ptr(), value) }
    }
}

pub struct RealPoint { ptr: *mut c_void }
impl TRealPoint for RealPoint { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RealPoint {
    pub fn from(ptr: *mut c_void) -> RealPoint { RealPoint { ptr: ptr } }
    pub fn null() -> RealPoint { RealPoint::from(0 as *mut c_void) }
    
}

pub trait TRealPoint {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Rect { ptr: *mut c_void }
impl TRect for Rect { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Rect {
    pub fn from(ptr: *mut c_void) -> Rect { Rect { ptr: ptr } }
    pub fn null() -> Rect { Rect::from(0 as *mut c_void) }
    
}

pub trait TRect {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Region { ptr: *mut c_void }
impl TRegion for Region {}
impl TGDIObject for Region {}
impl TObject for Region { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Region {
    pub fn from(ptr: *mut c_void) -> Region { Region { ptr: ptr } }
    pub fn null() -> Region { Region::from(0 as *mut c_void) }
    
    pub fn newDefault() -> Region {
        unsafe { Region { ptr: wxRegion_CreateDefault() } }
    }
    pub fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> Region {
        unsafe { Region { ptr: wxRegion_CreateFromRect(x, y, w, h) } }
    }
}

pub trait TRegion : TGDIObject {
    fn assign<T: TRegion>(&self, region: &T) {
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
    fn intersectRegion<T: TRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_IntersectRegion(self.ptr(), region.ptr()) }
    }
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_SubtractRect(self.ptr(), x, y, width, height) }
    }
    fn subtractRegion<T: TRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_SubtractRegion(self.ptr(), region.ptr()) }
    }
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_UnionRect(self.ptr(), x, y, width, height) }
    }
    fn unionRegion<T: TRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_UnionRegion(self.ptr(), region.ptr()) }
    }
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxRegion_XorRect(self.ptr(), x, y, width, height) }
    }
    fn xorRegion<T: TRegion>(&self, region: &T) -> c_int {
        unsafe { wxRegion_XorRegion(self.ptr(), region.ptr()) }
    }
}

pub struct RegionIterator { ptr: *mut c_void }
impl TRegionIterator for RegionIterator {}
impl TObject for RegionIterator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RegionIterator {
    pub fn from(ptr: *mut c_void) -> RegionIterator { RegionIterator { ptr: ptr } }
    pub fn null() -> RegionIterator { RegionIterator::from(0 as *mut c_void) }
    
    pub fn new() -> RegionIterator {
        unsafe { RegionIterator { ptr: wxRegionIterator_Create() } }
    }
    pub fn newFromRegion<T: TRegion>(region: &T) -> RegionIterator {
        unsafe { RegionIterator { ptr: wxRegionIterator_CreateFromRegion(region.ptr()) } }
    }
}

pub trait TRegionIterator : TObject {
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
    fn resetToRegion<T: TRegion>(&self, region: &T) {
        unsafe { wxRegionIterator_ResetToRegion(self.ptr(), region.ptr()) }
    }
}

pub struct SVGFileDC { ptr: *mut c_void }
impl TSVGFileDC for SVGFileDC {}
impl TDC for SVGFileDC {}
impl TObject for SVGFileDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SVGFileDC {
    pub fn from(ptr: *mut c_void) -> SVGFileDC { SVGFileDC { ptr: ptr } }
    pub fn null() -> SVGFileDC { SVGFileDC::from(0 as *mut c_void) }
    
    pub fn new(fileName: &str) -> SVGFileDC {
        let fileName = wxT(fileName);
        unsafe { SVGFileDC { ptr: wxSVGFileDC_Create(fileName.ptr()) } }
    }
    pub fn newWithSize(fileName: &str, w: c_int, h: c_int) -> SVGFileDC {
        let fileName = wxT(fileName);
        unsafe { SVGFileDC { ptr: wxSVGFileDC_CreateWithSize(fileName.ptr(), w, h) } }
    }
    pub fn newWithSizeAndResolution(fileName: &str, w: c_int, h: c_int, a_dpi: c_float) -> SVGFileDC {
        let fileName = wxT(fileName);
        unsafe { SVGFileDC { ptr: wxSVGFileDC_CreateWithSizeAndResolution(fileName.ptr(), w, h, a_dpi) } }
    }
}

pub trait TSVGFileDC : TDC {
}

pub struct ScreenDC { ptr: *mut c_void }
impl TScreenDC for ScreenDC {}
impl TDC for ScreenDC {}
impl TObject for ScreenDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScreenDC {
    pub fn from(ptr: *mut c_void) -> ScreenDC { ScreenDC { ptr: ptr } }
    pub fn null() -> ScreenDC { ScreenDC::from(0 as *mut c_void) }
    
    pub fn new() -> ScreenDC {
        unsafe { ScreenDC { ptr: wxScreenDC_Create() } }
    }
}

pub trait TScreenDC : TDC {
    fn endDrawingOnTop(&self) -> c_int {
        unsafe { wxScreenDC_EndDrawingOnTop(self.ptr()) }
    }
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTop(self.ptr(), x, y, w, h) }
    }
    fn startDrawingOnTopOfWin<T: TWindow>(&self, win: &T) -> c_int {
        unsafe { wxScreenDC_StartDrawingOnTopOfWin(self.ptr(), win.ptr()) }
    }
}

pub struct ScrollBar { ptr: *mut c_void }
impl TScrollBar for ScrollBar {}
impl TControl for ScrollBar {}
impl TWindow for ScrollBar {}
impl TEvtHandler for ScrollBar {}
impl TObject for ScrollBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrollBar {
    pub fn from(ptr: *mut c_void) -> ScrollBar { ScrollBar { ptr: ptr } }
    pub fn null() -> ScrollBar { ScrollBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ScrollBar {
        unsafe { ScrollBar { ptr: wxScrollBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TScrollBar : TControl {
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

pub struct ScrollEvent { ptr: *mut c_void }
impl TScrollEvent for ScrollEvent {}
impl TEvent for ScrollEvent {}
impl TObject for ScrollEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrollEvent {
    pub fn from(ptr: *mut c_void) -> ScrollEvent { ScrollEvent { ptr: ptr } }
    pub fn null() -> ScrollEvent { ScrollEvent::from(0 as *mut c_void) }
    
}

pub trait TScrollEvent : TEvent {
    fn getOrientation(&self) -> c_int {
        unsafe { wxScrollEvent_GetOrientation(self.ptr()) }
    }
    fn getPosition(&self) -> c_int {
        unsafe { wxScrollEvent_GetPosition(self.ptr()) }
    }
}

pub struct ScrollWinEvent { ptr: *mut c_void }
impl TScrollWinEvent for ScrollWinEvent {}
impl TEvent for ScrollWinEvent {}
impl TObject for ScrollWinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrollWinEvent {
    pub fn from(ptr: *mut c_void) -> ScrollWinEvent { ScrollWinEvent { ptr: ptr } }
    pub fn null() -> ScrollWinEvent { ScrollWinEvent::from(0 as *mut c_void) }
    
}

pub trait TScrollWinEvent : TEvent {
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

pub struct ScrolledWindow { ptr: *mut c_void }
impl TScrolledWindow for ScrolledWindow {}
impl TPanel for ScrolledWindow {}
impl TWindow for ScrolledWindow {}
impl TEvtHandler for ScrolledWindow {}
impl TObject for ScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ScrolledWindow {
    pub fn from(ptr: *mut c_void) -> ScrolledWindow { ScrolledWindow { ptr: ptr } }
    pub fn null() -> ScrolledWindow { ScrolledWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ScrolledWindow {
        unsafe { ScrolledWindow { ptr: wxScrolledWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TScrolledWindow : TPanel {
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
        unsafe { Window { ptr: wxScrolledWindow_GetTargetWindow(self.ptr()) } }
    }
    fn getViewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_GetViewStart(self.ptr(), _x, _y) }
    }
    fn onDraw<T: TDC>(&self, dc: &T) {
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
    fn setTargetWindow<T: TWindow>(&self, target: &T) {
        unsafe { wxScrolledWindow_SetTargetWindow(self.ptr(), target.ptr()) }
    }
    fn viewStart(&self, _x: *mut c_void, _y: *mut c_void) {
        unsafe { wxScrolledWindow_ViewStart(self.ptr(), _x, _y) }
    }
    fn setScrollRate(&self, xstep: c_int, ystep: c_int) {
        unsafe { wxScrolledWindow_SetScrollRate(self.ptr(), xstep, ystep) }
    }
}

pub struct SetCursorEvent { ptr: *mut c_void }
impl TSetCursorEvent for SetCursorEvent {}
impl TEvent for SetCursorEvent {}
impl TObject for SetCursorEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SetCursorEvent {
    pub fn from(ptr: *mut c_void) -> SetCursorEvent { SetCursorEvent { ptr: ptr } }
    pub fn null() -> SetCursorEvent { SetCursorEvent::from(0 as *mut c_void) }
    
}

pub trait TSetCursorEvent : TEvent {
    fn getCursor(&self) -> Cursor {
        unsafe { Cursor { ptr: wxSetCursorEvent_GetCursor(self.ptr()) } }
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
    fn setCursor<T: TCursor>(&self, cursor: &T) {
        unsafe { wxSetCursorEvent_SetCursor(self.ptr(), cursor.ptr()) }
    }
}

pub struct ShowEvent { ptr: *mut c_void }
impl TShowEvent for ShowEvent {}
impl TEvent for ShowEvent {}
impl TObject for ShowEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ShowEvent {
    pub fn from(ptr: *mut c_void) -> ShowEvent { ShowEvent { ptr: ptr } }
    pub fn null() -> ShowEvent { ShowEvent::from(0 as *mut c_void) }
    
}

pub trait TShowEvent : TEvent {
    fn isShown(&self) -> c_int {
        unsafe { wxShowEvent_IsShown(self.ptr()) }
    }
    fn setShow(&self, show: c_int) {
        unsafe { wxShowEvent_SetShow(self.ptr(), show) }
    }
}

pub struct SimpleHelpProvider { ptr: *mut c_void }
impl TSimpleHelpProvider for SimpleHelpProvider {}
impl THelpProvider for SimpleHelpProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SimpleHelpProvider {
    pub fn from(ptr: *mut c_void) -> SimpleHelpProvider { SimpleHelpProvider { ptr: ptr } }
    pub fn null() -> SimpleHelpProvider { SimpleHelpProvider::from(0 as *mut c_void) }
    
    pub fn new() -> SimpleHelpProvider {
        unsafe { SimpleHelpProvider { ptr: wxSimpleHelpProvider_Create() } }
    }
}

pub trait TSimpleHelpProvider : THelpProvider {
}

pub struct SingleChoiceDialog { ptr: *mut c_void }
impl TSingleChoiceDialog for SingleChoiceDialog {}
impl TDialog for SingleChoiceDialog {}
impl TTopLevelWindow for SingleChoiceDialog {}
impl TWindow for SingleChoiceDialog {}
impl TEvtHandler for SingleChoiceDialog {}
impl TObject for SingleChoiceDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SingleChoiceDialog {
    pub fn from(ptr: *mut c_void) -> SingleChoiceDialog { SingleChoiceDialog { ptr: ptr } }
    pub fn null() -> SingleChoiceDialog { SingleChoiceDialog::from(0 as *mut c_void) }
    
}

pub trait TSingleChoiceDialog : TDialog {
}

pub struct Size { ptr: *mut c_void }
impl TSize for Size { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Size {
    pub fn from(ptr: *mut c_void) -> Size { Size { ptr: ptr } }
    pub fn null() -> Size { Size::from(0 as *mut c_void) }
    
    pub fn new(w: c_int, h: c_int) -> Size {
        unsafe { Size { ptr: wxSize_Create(w, h) } }
    }
}

pub trait TSize {
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

pub struct SizeEvent { ptr: *mut c_void }
impl TSizeEvent for SizeEvent {}
impl TEvent for SizeEvent {}
impl TObject for SizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SizeEvent {
    pub fn from(ptr: *mut c_void) -> SizeEvent { SizeEvent { ptr: ptr } }
    pub fn null() -> SizeEvent { SizeEvent::from(0 as *mut c_void) }
    
}

pub trait TSizeEvent : TEvent {
    fn getSize(&self) -> Size {
        unsafe { Size { ptr: wxSizeEvent_GetSize(self.ptr()) } }
    }
}

pub struct Sizer { ptr: *mut c_void }
impl TSizer for Sizer {}
impl TObject for Sizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Sizer {
    pub fn from(ptr: *mut c_void) -> Sizer { Sizer { ptr: ptr } }
    pub fn null() -> Sizer { Sizer::from(0 as *mut c_void) }
    
}

pub trait TSizer : TObject {
    fn add(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Add(self.ptr(), width, height, option, flag, border, userData) }
    }
    fn addSizer<T: TSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    fn addWindow<T: TWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_AddWindow(self.ptr(), window.ptr(), option, flag, border, userData) }
    }
    fn calcMin(&self) -> Size {
        unsafe { Size { ptr: wxSizer_CalcMin(self.ptr()) } }
    }
    fn fit<T: TWindow>(&self, window: &T) {
        unsafe { wxSizer_Fit(self.ptr(), window.ptr()) }
    }
    fn getChildren(&self, _res: *mut c_void, _cnt: c_int) -> c_int {
        unsafe { wxSizer_GetChildren(self.ptr(), _res, _cnt) }
    }
    fn getMinSize(&self) -> Size {
        unsafe { Size { ptr: wxSizer_GetMinSize(self.ptr()) } }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxSizer_GetPosition(self.ptr()) } }
    }
    fn getSize(&self) -> Size {
        unsafe { Size { ptr: wxSizer_GetSize(self.ptr()) } }
    }
    fn insert(&self, before: c_int, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Insert(self.ptr(), before, width, height, option, flag, border, userData) }
    }
    fn insertSizer<T: TSizer>(&self, before: c_int, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertSizer(self.ptr(), before, sizer.ptr(), option, flag, border, userData) }
    }
    fn insertWindow<T: TWindow>(&self, before: c_int, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_InsertWindow(self.ptr(), before, window.ptr(), option, flag, border, userData) }
    }
    fn layout(&self) {
        unsafe { wxSizer_Layout(self.ptr()) }
    }
    fn prepend(&self, width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_Prepend(self.ptr(), width, height, option, flag, border, userData) }
    }
    fn prependSizer<T: TSizer>(&self, sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
        unsafe { wxSizer_PrependSizer(self.ptr(), sizer.ptr(), option, flag, border, userData) }
    }
    fn prependWindow<T: TWindow>(&self, window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) {
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
    fn setItemMinSizeSizer<T: TSizer>(&self, sizer: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeSizer(self.ptr(), sizer.ptr(), width, height) }
    }
    fn setItemMinSizeWindow<T: TWindow>(&self, window: &T, width: c_int, height: c_int) {
        unsafe { wxSizer_SetItemMinSizeWindow(self.ptr(), window.ptr(), width, height) }
    }
    fn setMinSize(&self, width: c_int, height: c_int) {
        unsafe { wxSizer_SetMinSize(self.ptr(), width, height) }
    }
    fn setSizeHints<T: TWindow>(&self, window: &T) {
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
    fn detachWindow<T: TWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_DetachWindow(self.ptr(), window.ptr()) }
    }
    fn detachSizer<T: TSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_DetachSizer(self.ptr(), sizer.ptr()) }
    }
    fn detach(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Detach(self.ptr(), index) }
    }
    fn fitInside<T: TWindow>(&self, window: &T) {
        unsafe { wxSizer_FitInside(self.ptr(), window.ptr()) }
    }
    fn getContainingWindow(&self) -> Window {
        unsafe { Window { ptr: wxSizer_GetContainingWindow(self.ptr()) } }
    }
    fn getItemWindow<T: TWindow>(&self, window: &T, recursive: c_int) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizer_GetItemWindow(self.ptr(), window.ptr(), recursive) } }
    }
    fn getItemSizer<T: TSizer>(&self, window: &T, recursive: c_int) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizer_GetItemSizer(self.ptr(), window.ptr(), recursive) } }
    }
    fn getItem(&self, index: c_int) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizer_GetItem(self.ptr(), index) } }
    }
    fn hideWindow<T: TWindow>(&self, window: &T) -> c_int {
        unsafe { wxSizer_HideWindow(self.ptr(), window.ptr()) }
    }
    fn hideSizer<T: TSizer>(&self, sizer: &T) -> c_int {
        unsafe { wxSizer_HideSizer(self.ptr(), sizer.ptr()) }
    }
    fn hide(&self, index: c_int) -> c_int {
        unsafe { wxSizer_Hide(self.ptr(), index) }
    }
    fn insertSpacer(&self, index: c_int, size: c_int) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizer_InsertSpacer(self.ptr(), index, size) } }
    }
    fn insertStretchSpacer(&self, index: c_int, prop: c_int) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizer_InsertStretchSpacer(self.ptr(), index, prop) } }
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
        unsafe { SizerItem { ptr: wxSizer_PrependSpacer(self.ptr(), size) } }
    }
    fn prependStretchSpacer(&self, prop: c_int) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizer_PrependStretchSpacer(self.ptr(), prop) } }
    }
    fn replaceWindow<T: TWindow, U: TWindow>(&self, oldwin: &T, newwin: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceWindow(self.ptr(), oldwin.ptr(), newwin.ptr(), recursive) }
    }
    fn replaceSizer<T: TSizer, U: TSizer>(&self, oldsz: &T, newsz: &U, recursive: c_int) -> c_int {
        unsafe { wxSizer_ReplaceSizer(self.ptr(), oldsz.ptr(), newsz.ptr(), recursive) }
    }
    fn replace<T: TSizerItem>(&self, oldindex: c_int, newitem: &T) -> c_int {
        unsafe { wxSizer_Replace(self.ptr(), oldindex, newitem.ptr()) }
    }
    fn setVirtualSizeHints<T: TWindow>(&self, window: &T) {
        unsafe { wxSizer_SetVirtualSizeHints(self.ptr(), window.ptr()) }
    }
    fn showWindow<T: TWindow>(&self, window: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowWindow(self.ptr(), window.ptr(), show, recursive) }
    }
    fn showSizer<T: TSizer>(&self, sizer: &T, show: c_int, recursive: c_int) -> c_int {
        unsafe { wxSizer_ShowSizer(self.ptr(), sizer.ptr(), show, recursive) }
    }
    fn show<T: TSizer>(&self, sizer: &T, index: c_int, show: c_int) -> c_int {
        unsafe { wxSizer_Show(self.ptr(), sizer.ptr(), index, show) }
    }
}

pub struct SizerItem { ptr: *mut c_void }
impl TSizerItem for SizerItem {}
impl TObject for SizerItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SizerItem {
    pub fn from(ptr: *mut c_void) -> SizerItem { SizerItem { ptr: ptr } }
    pub fn null() -> SizerItem { SizerItem::from(0 as *mut c_void) }
    
    pub fn new(width: c_int, height: c_int, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> SizerItem {
        unsafe { SizerItem { ptr: wxSizerItem_Create(width, height, option, flag, border, userData) } }
    }
    pub fn newInSizer<T: TSizer>(sizer: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInSizer(sizer.ptr(), option, flag, border, userData) }
    }
    pub fn newInWindow<T: TWindow>(window: &T, option: c_int, flag: c_int, border: c_int, userData: *mut c_void) -> *mut c_void {
        unsafe { wxSizerItem_CreateInWindow(window.ptr(), option, flag, border, userData) }
    }
}

pub trait TSizerItem : TObject {
    fn calcMin(&self) -> Size {
        unsafe { Size { ptr: wxSizerItem_CalcMin(self.ptr()) } }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxSizerItem_GetBorder(self.ptr()) }
    }
    fn getFlag(&self) -> c_int {
        unsafe { wxSizerItem_GetFlag(self.ptr()) }
    }
    fn getMinSize(&self) -> Size {
        unsafe { Size { ptr: wxSizerItem_GetMinSize(self.ptr()) } }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxSizerItem_GetPosition(self.ptr()) } }
    }
    fn getRatio(&self) -> c_float {
        unsafe { wxSizerItem_GetRatio(self.ptr()) }
    }
    fn getSize(&self) -> Size {
        unsafe { Size { ptr: wxSizerItem_GetSize(self.ptr()) } }
    }
    fn getSizer(&self) -> Sizer {
        unsafe { Sizer { ptr: wxSizerItem_GetSizer(self.ptr()) } }
    }
    fn getUserData(&self) -> *mut c_void {
        unsafe { wxSizerItem_GetUserData(self.ptr()) }
    }
    fn getWindow(&self) -> Window {
        unsafe { Window { ptr: wxSizerItem_GetWindow(self.ptr()) } }
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
    fn setSizer<T: TSizer>(&self, sizer: &T) {
        unsafe { wxSizerItem_SetSizer(self.ptr(), sizer.ptr()) }
    }
    fn setWindow<T: TWindow>(&self, window: &T) {
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
        unsafe { Rect { ptr: wxSizerItem_GetRect(self.ptr()) } }
    }
    fn getSpacer(&self) -> Size {
        unsafe { Size { ptr: wxSizerItem_GetSpacer(self.ptr()) } }
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

pub struct Slider { ptr: *mut c_void }
impl TSlider for Slider {}
impl TControl for Slider {}
impl TWindow for Slider {}
impl TEvtHandler for Slider {}
impl TObject for Slider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Slider {
    pub fn from(ptr: *mut c_void) -> Slider { Slider { ptr: ptr } }
    pub fn null() -> Slider { Slider::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _init: c_int, _min: c_int, _max: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> Slider {
        unsafe { Slider { ptr: wxSlider_Create(_prt.ptr(), _id, _init, _min, _max, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TSlider : TControl {
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

pub struct SpinButton { ptr: *mut c_void }
impl TSpinButton for SpinButton {}
impl TControl for SpinButton {}
impl TWindow for SpinButton {}
impl TEvtHandler for SpinButton {}
impl TObject for SpinButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SpinButton {
    pub fn from(ptr: *mut c_void) -> SpinButton { SpinButton { ptr: ptr } }
    pub fn null() -> SpinButton { SpinButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> SpinButton {
        unsafe { SpinButton { ptr: wxSpinButton_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TSpinButton : TControl {
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

pub struct SpinCtrl { ptr: *mut c_void }
impl TSpinCtrl for SpinCtrl {}
impl TControl for SpinCtrl {}
impl TWindow for SpinCtrl {}
impl TEvtHandler for SpinCtrl {}
impl TObject for SpinCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SpinCtrl {
    pub fn from(ptr: *mut c_void) -> SpinCtrl { SpinCtrl { ptr: ptr } }
    pub fn null() -> SpinCtrl { SpinCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: c_int, _max: c_int, _init: c_int) -> SpinCtrl {
        let _txt = wxT(_txt);
        unsafe { SpinCtrl { ptr: wxSpinCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl, _min, _max, _init) } }
    }
}

pub trait TSpinCtrl : TControl {
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

pub struct SpinEvent { ptr: *mut c_void }
impl TSpinEvent for SpinEvent {}
impl TNotifyEvent for SpinEvent {}
impl TCommandEvent for SpinEvent {}
impl TEvent for SpinEvent {}
impl TObject for SpinEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SpinEvent {
    pub fn from(ptr: *mut c_void) -> SpinEvent { SpinEvent { ptr: ptr } }
    pub fn null() -> SpinEvent { SpinEvent::from(0 as *mut c_void) }
    
}

pub trait TSpinEvent : TNotifyEvent {
    fn getPosition(&self) -> c_int {
        unsafe { wxSpinEvent_GetPosition(self.ptr()) }
    }
    fn setPosition(&self, pos: c_int) {
        unsafe { wxSpinEvent_SetPosition(self.ptr(), pos) }
    }
}

pub struct SplitterEvent { ptr: *mut c_void }
impl TSplitterEvent for SplitterEvent {}
impl TNotifyEvent for SplitterEvent {}
impl TCommandEvent for SplitterEvent {}
impl TEvent for SplitterEvent {}
impl TObject for SplitterEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterEvent {
    pub fn from(ptr: *mut c_void) -> SplitterEvent { SplitterEvent { ptr: ptr } }
    pub fn null() -> SplitterEvent { SplitterEvent::from(0 as *mut c_void) }
    
}

pub trait TSplitterEvent : TNotifyEvent {
}

pub struct SplitterWindow { ptr: *mut c_void }
impl TSplitterWindow for SplitterWindow {}
impl TWindow for SplitterWindow {}
impl TEvtHandler for SplitterWindow {}
impl TObject for SplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterWindow {
    pub fn from(ptr: *mut c_void) -> SplitterWindow { SplitterWindow { ptr: ptr } }
    pub fn null() -> SplitterWindow { SplitterWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> SplitterWindow {
        unsafe { SplitterWindow { ptr: wxSplitterWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TSplitterWindow : TWindow {
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
        unsafe { Window { ptr: wxSplitterWindow_GetWindow1(self.ptr()) } }
    }
    fn getWindow2(&self) -> Window {
        unsafe { Window { ptr: wxSplitterWindow_GetWindow2(self.ptr()) } }
    }
    fn initialize<T: TWindow>(&self, window: &T) {
        unsafe { wxSplitterWindow_Initialize(self.ptr(), window.ptr()) }
    }
    fn isSplit(&self) -> c_int {
        unsafe { wxSplitterWindow_IsSplit(self.ptr()) }
    }
    fn replaceWindow<T: TWindow, U: TWindow>(&self, winOld: &T, winNew: &U) -> c_int {
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
    fn splitHorizontally<T: TWindow, U: TWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitHorizontally(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    fn splitVertically<T: TWindow, U: TWindow>(&self, window1: &T, window2: &U, sashPosition: c_int) -> c_int {
        unsafe { wxSplitterWindow_SplitVertically(self.ptr(), window1.ptr(), window2.ptr(), sashPosition) }
    }
    fn unsplit<T: TWindow>(&self, toRemove: &T) -> c_int {
        unsafe { wxSplitterWindow_Unsplit(self.ptr(), toRemove.ptr()) }
    }
    fn getSashGravity(&self) -> c_double {
        unsafe { wxSplitterWindow_GetSashGravity(self.ptr()) }
    }
    fn setSashGravity(&self, gravity: c_double) {
        unsafe { wxSplitterWindow_SetSashGravity(self.ptr(), gravity) }
    }
}

pub struct StaticBitmap { ptr: *mut c_void }
impl TStaticBitmap for StaticBitmap {}
impl TControl for StaticBitmap {}
impl TWindow for StaticBitmap {}
impl TEvtHandler for StaticBitmap {}
impl TObject for StaticBitmap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticBitmap {
    pub fn from(ptr: *mut c_void) -> StaticBitmap { StaticBitmap { ptr: ptr } }
    pub fn null() -> StaticBitmap { StaticBitmap::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TBitmap>(_prt: &T, _id: c_int, bitmap: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticBitmap {
        unsafe { StaticBitmap { ptr: wxStaticBitmap_Create(_prt.ptr(), _id, bitmap.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TStaticBitmap : TControl {
    fn getBitmap<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    fn getIcon<T: TIcon>(&self, _ref: &T) {
        unsafe { wxStaticBitmap_GetIcon(self.ptr(), _ref.ptr()) }
    }
    fn setBitmap<T: TBitmap>(&self, bitmap: &T) {
        unsafe { wxStaticBitmap_SetBitmap(self.ptr(), bitmap.ptr()) }
    }
    fn setIcon<T: TIcon>(&self, icon: &T) {
        unsafe { wxStaticBitmap_SetIcon(self.ptr(), icon.ptr()) }
    }
}

pub struct StaticBox { ptr: *mut c_void }
impl TStaticBox for StaticBox {}
impl TControl for StaticBox {}
impl TWindow for StaticBox {}
impl TEvtHandler for StaticBox {}
impl TObject for StaticBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticBox {
    pub fn from(ptr: *mut c_void) -> StaticBox { StaticBox { ptr: ptr } }
    pub fn null() -> StaticBox { StaticBox::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticBox {
        let _txt = wxT(_txt);
        unsafe { StaticBox { ptr: wxStaticBox_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TStaticBox : TControl {
}

pub struct StaticBoxSizer { ptr: *mut c_void }
impl TStaticBoxSizer for StaticBoxSizer {}
impl TBoxSizer for StaticBoxSizer {}
impl TSizer for StaticBoxSizer {}
impl TObject for StaticBoxSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticBoxSizer {
    pub fn from(ptr: *mut c_void) -> StaticBoxSizer { StaticBoxSizer { ptr: ptr } }
    pub fn null() -> StaticBoxSizer { StaticBoxSizer::from(0 as *mut c_void) }
    
    pub fn new<T: TStaticBox>(box_: &T, orient: c_int) -> StaticBoxSizer {
        unsafe { StaticBoxSizer { ptr: wxStaticBoxSizer_Create(box_.ptr(), orient) } }
    }
}

pub trait TStaticBoxSizer : TBoxSizer {
    fn getStaticBox(&self) -> StaticBox {
        unsafe { StaticBox { ptr: wxStaticBoxSizer_GetStaticBox(self.ptr()) } }
    }
}

pub struct StaticLine { ptr: *mut c_void }
impl TStaticLine for StaticLine {}
impl TControl for StaticLine {}
impl TWindow for StaticLine {}
impl TEvtHandler for StaticLine {}
impl TObject for StaticLine { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticLine {
    pub fn from(ptr: *mut c_void) -> StaticLine { StaticLine { ptr: ptr } }
    pub fn null() -> StaticLine { StaticLine::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticLine {
        unsafe { StaticLine { ptr: wxStaticLine_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TStaticLine : TControl {
    fn getDefaultSize(&self) -> c_int {
        unsafe { wxStaticLine_GetDefaultSize(self.ptr()) }
    }
    fn isVertical(&self) -> c_int {
        unsafe { wxStaticLine_IsVertical(self.ptr()) }
    }
}

pub struct StaticText { ptr: *mut c_void }
impl TStaticText for StaticText {}
impl TControl for StaticText {}
impl TWindow for StaticText {}
impl TEvtHandler for StaticText {}
impl TObject for StaticText { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StaticText {
    pub fn from(ptr: *mut c_void) -> StaticText { StaticText { ptr: ptr } }
    pub fn null() -> StaticText { StaticText::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StaticText {
        let _txt = wxT(_txt);
        unsafe { StaticText { ptr: wxStaticText_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TStaticText : TControl {
}

pub struct StatusBar { ptr: *mut c_void }
impl TStatusBar for StatusBar {}
impl TWindow for StatusBar {}
impl TEvtHandler for StatusBar {}
impl TObject for StatusBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StatusBar {
    pub fn from(ptr: *mut c_void) -> StatusBar { StatusBar { ptr: ptr } }
    pub fn null() -> StatusBar { StatusBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> StatusBar {
        unsafe { StatusBar { ptr: wxStatusBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TStatusBar : TWindow {
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

pub struct SysColourChangedEvent { ptr: *mut c_void }
impl TSysColourChangedEvent for SysColourChangedEvent {}
impl TEvent for SysColourChangedEvent {}
impl TObject for SysColourChangedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SysColourChangedEvent {
    pub fn from(ptr: *mut c_void) -> SysColourChangedEvent { SysColourChangedEvent { ptr: ptr } }
    pub fn null() -> SysColourChangedEvent { SysColourChangedEvent::from(0 as *mut c_void) }
    
}

pub trait TSysColourChangedEvent : TEvent {
}

pub struct SystemSettings { ptr: *mut c_void }
impl TSystemSettings for SystemSettings {}
impl TObject for SystemSettings { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SystemSettings {
    pub fn from(ptr: *mut c_void) -> SystemSettings { SystemSettings { ptr: ptr } }
    pub fn null() -> SystemSettings { SystemSettings::from(0 as *mut c_void) }
    
    pub fn getColour<T: TColour>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetColour(index, _ref.ptr()) }
    }
    pub fn getFont<T: TFont>(index: c_int, _ref: &T) {
        unsafe { wxSystemSettings_GetFont(index, _ref.ptr()) }
    }
    pub fn getMetric(index: c_int) -> c_int {
        unsafe { wxSystemSettings_GetMetric(index) }
    }
    pub fn getScreenType() -> c_int {
        unsafe { wxSystemSettings_GetScreenType() }
    }
}

pub trait TSystemSettings : TObject {
}

pub struct TextAttr { ptr: *mut c_void }
impl TTextAttr for TextAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextAttr {
    pub fn from(ptr: *mut c_void) -> TextAttr { TextAttr { ptr: ptr } }
    pub fn null() -> TextAttr { TextAttr::from(0 as *mut c_void) }
    
    pub fn new<T: TColour, U: TColour, V: TFont>(colText: &T, colBack: &U, font: &V) -> TextAttr {
        unsafe { TextAttr { ptr: wxTextAttr_Create(colText.ptr(), colBack.ptr(), font.ptr()) } }
    }
    pub fn newDefault() -> TextAttr {
        unsafe { TextAttr { ptr: wxTextAttr_CreateDefault() } }
    }
}

pub trait TTextAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTextAttr_Delete(self.ptr()) }
    }
    fn getBackgroundColour<T: TColour>(&self, colour: &T) {
        unsafe { wxTextAttr_GetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn getFont<T: TFont>(&self, font: &T) {
        unsafe { wxTextAttr_GetFont(self.ptr(), font.ptr()) }
    }
    fn getTextColour<T: TColour>(&self, colour: &T) {
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
    fn setTextColour<T: TColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetTextColour(self.ptr(), colour.ptr()) }
    }
    fn setBackgroundColour<T: TColour>(&self, colour: &T) {
        unsafe { wxTextAttr_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setFont<T: TFont>(&self, font: &T) {
        unsafe { wxTextAttr_SetFont(self.ptr(), font.ptr()) }
    }
}

pub struct TextCtrl { ptr: *mut c_void }
impl TTextCtrl for TextCtrl {}
impl TControl for TextCtrl {}
impl TWindow for TextCtrl {}
impl TEvtHandler for TextCtrl {}
impl TObject for TextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextCtrl {
    pub fn from(ptr: *mut c_void) -> TextCtrl { TextCtrl { ptr: ptr } }
    pub fn null() -> TextCtrl { TextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> TextCtrl {
        let _txt = wxT(_txt);
        unsafe { TextCtrl { ptr: wxTextCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TTextCtrl : TControl {
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
    fn emulateKeyPress<T: TKeyEvent>(&self, keyevent: &T) -> c_int {
        unsafe { wxTextCtrl_EmulateKeyPress(self.ptr(), keyevent.ptr()) }
    }
    fn getDefaultStyle(&self) -> TextAttr {
        unsafe { TextAttr { ptr: wxTextCtrl_GetDefaultStyle(self.ptr()) } }
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
    fn setDefaultStyle<T: TTextAttr>(&self, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetDefaultStyle(self.ptr(), style.ptr()) }
    }
    fn setMaxLength(&self, len: c_long) {
        unsafe { wxTextCtrl_SetMaxLength(self.ptr(), len) }
    }
    fn setStyle<T: TTextAttr>(&self, start: c_long, end: c_long, style: &T) -> c_int {
        unsafe { wxTextCtrl_SetStyle(self.ptr(), start, end, style.ptr()) }
    }
}

pub struct TextDataObject { ptr: *mut c_void }
impl TTextDataObject for TextDataObject {}
impl TDataObjectSimple for TextDataObject {}
impl TDataObject for TextDataObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextDataObject {
    pub fn from(ptr: *mut c_void) -> TextDataObject { TextDataObject { ptr: ptr } }
    pub fn null() -> TextDataObject { TextDataObject::from(0 as *mut c_void) }
    
}

pub trait TTextDataObject : TDataObjectSimple {
}

pub struct TextDropTarget { ptr: *mut c_void }
impl TTextDropTarget for TextDropTarget {}
impl TDropTarget for TextDropTarget { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextDropTarget {
    pub fn from(ptr: *mut c_void) -> TextDropTarget { TextDropTarget { ptr: ptr } }
    pub fn null() -> TextDropTarget { TextDropTarget::from(0 as *mut c_void) }
    
}

pub trait TTextDropTarget : TDropTarget {
}

pub struct TextEntryDialog { ptr: *mut c_void }
impl TTextEntryDialog for TextEntryDialog {}
impl TDialog for TextEntryDialog {}
impl TTopLevelWindow for TextEntryDialog {}
impl TWindow for TextEntryDialog {}
impl TEvtHandler for TextEntryDialog {}
impl TObject for TextEntryDialog { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextEntryDialog {
    pub fn from(ptr: *mut c_void) -> TextEntryDialog { TextEntryDialog { ptr: ptr } }
    pub fn null() -> TextEntryDialog { TextEntryDialog::from(0 as *mut c_void) }
    
}

pub trait TTextEntryDialog : TDialog {
}

pub struct TextValidator { ptr: *mut c_void }
impl TTextValidator for TextValidator {}
impl TValidator for TextValidator {}
impl TEvtHandler for TextValidator {}
impl TObject for TextValidator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TextValidator {
    pub fn from(ptr: *mut c_void) -> TextValidator { TextValidator { ptr: ptr } }
    pub fn null() -> TextValidator { TextValidator::from(0 as *mut c_void) }
    
    pub fn new(style: c_int, val: *mut c_void) -> TextValidator {
        unsafe { TextValidator { ptr: wxTextValidator_Create(style, val) } }
    }
}

pub trait TTextValidator : TValidator {
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
        unsafe { Validator { ptr: wxTextValidator_Clone(self.ptr()) } }
    }
    fn getStyle(&self) -> c_int {
        unsafe { wxTextValidator_GetStyle(self.ptr()) }
    }
    fn onChar<T: TEvent>(&self, event: &T) {
        unsafe { wxTextValidator_OnChar(self.ptr(), event.ptr()) }
    }
    fn setStyle(&self, style: c_int) {
        unsafe { wxTextValidator_SetStyle(self.ptr(), style) }
    }
}

pub struct Timer { ptr: *mut c_void }
impl TTimer for Timer {}
impl TObject for Timer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Timer {
    pub fn from(ptr: *mut c_void) -> Timer { Timer { ptr: ptr } }
    pub fn null() -> Timer { Timer::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int) -> Timer {
        unsafe { Timer { ptr: wxTimer_Create(_prt.ptr(), _id) } }
    }
}

pub trait TTimer : TObject {
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

pub struct TimerEvent { ptr: *mut c_void }
impl TTimerEvent for TimerEvent {}
impl TEvent for TimerEvent {}
impl TObject for TimerEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerEvent {
    pub fn from(ptr: *mut c_void) -> TimerEvent { TimerEvent { ptr: ptr } }
    pub fn null() -> TimerEvent { TimerEvent::from(0 as *mut c_void) }
    
}

pub trait TTimerEvent : TEvent {
    fn getInterval(&self) -> c_int {
        unsafe { wxTimerEvent_GetInterval(self.ptr()) }
    }
}

pub struct TimerEx { ptr: *mut c_void }
impl TTimerEx for TimerEx {}
impl TTimer for TimerEx {}
impl TObject for TimerEx { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerEx {
    pub fn from(ptr: *mut c_void) -> TimerEx { TimerEx { ptr: ptr } }
    pub fn null() -> TimerEx { TimerEx::from(0 as *mut c_void) }
    
    pub fn new() -> TimerEx {
        unsafe { TimerEx { ptr: wxTimerEx_Create() } }
    }
}

pub trait TTimerEx : TTimer {
    fn connect<T: TClosure>(&self, closure: &T) {
        unsafe { wxTimerEx_Connect(self.ptr(), closure.ptr()) }
    }
    fn getClosure(&self) -> Closure {
        unsafe { Closure { ptr: wxTimerEx_GetClosure(self.ptr()) } }
    }
}

pub struct TimerRunner { ptr: *mut c_void }
impl TTimerRunner for TimerRunner { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerRunner {
    pub fn from(ptr: *mut c_void) -> TimerRunner { TimerRunner { ptr: ptr } }
    pub fn null() -> TimerRunner { TimerRunner::from(0 as *mut c_void) }
    
}

pub trait TTimerRunner {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct TipWindow { ptr: *mut c_void }
impl TTipWindow for TipWindow {}
impl TPopupTransientWindow for TipWindow {}
impl TPopupWindow for TipWindow {}
impl TWindow for TipWindow {}
impl TEvtHandler for TipWindow {}
impl TObject for TipWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TipWindow {
    pub fn from(ptr: *mut c_void) -> TipWindow { TipWindow { ptr: ptr } }
    pub fn null() -> TipWindow { TipWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(parent: &T, text: &str, maxLength: c_int) -> TipWindow {
        let text = wxT(text);
        unsafe { TipWindow { ptr: wxTipWindow_Create(parent.ptr(), text.ptr(), maxLength) } }
    }
}

pub trait TTipWindow : TPopupTransientWindow {
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxTipWindow_SetBoundingRect(self.ptr(), x, y, w, h) }
    }
    fn setTipWindowPtr(&self, windowPtr: *mut c_void) {
        unsafe { wxTipWindow_SetTipWindowPtr(self.ptr(), windowPtr) }
    }
}

pub struct ToggleButton { ptr: *mut c_void }
impl TToggleButton for ToggleButton {}
impl TControl for ToggleButton {}
impl TWindow for ToggleButton {}
impl TEvtHandler for ToggleButton {}
impl TObject for ToggleButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToggleButton {
    pub fn from(ptr: *mut c_void) -> ToggleButton { ToggleButton { ptr: ptr } }
    pub fn null() -> ToggleButton { ToggleButton::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(parent: &T, id: c_int, label: &str, x: c_int, y: c_int, w: c_int, h: c_int, style: c_int) -> ToggleButton {
        let label = wxT(label);
        unsafe { ToggleButton { ptr: wxToggleButton_Create(parent.ptr(), id, label.ptr(), x, y, w, h, style) } }
    }
}

pub trait TToggleButton : TControl {
    fn getValue(&self) -> c_int {
        unsafe { wxToggleButton_GetValue(self.ptr()) }
    }
    fn setValue(&self, state: c_int) {
        unsafe { wxToggleButton_SetValue(self.ptr(), state) }
    }
}

pub struct ToolBar { ptr: *mut c_void }
impl TToolBar for ToolBar {}
impl TToolBarBase for ToolBar {}
impl TControl for ToolBar {}
impl TWindow for ToolBar {}
impl TEvtHandler for ToolBar {}
impl TObject for ToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolBar {
    pub fn from(ptr: *mut c_void) -> ToolBar { ToolBar { ptr: ptr } }
    pub fn null() -> ToolBar { ToolBar::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> ToolBar {
        unsafe { ToolBar { ptr: wxToolBar_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TToolBar : TToolBarBase {
    fn addControl<T: TControl>(&self, ctrl: &T) -> c_int {
        unsafe { wxToolBar_AddControl(self.ptr(), ctrl.ptr()) }
    }
    fn addSeparator(&self) {
        unsafe { wxToolBar_AddSeparator(self.ptr()) }
    }
    fn addTool<T: TBitmap>(&self, id: c_int, bmp: &T, shelp: &str, lhelp: &str) {
        let shelp = wxT(shelp);
        let lhelp = wxT(lhelp);
        unsafe { wxToolBar_AddTool(self.ptr(), id, bmp.ptr(), shelp.ptr(), lhelp.ptr()) }
    }
    fn addToolEx<T: TBitmap, U: TBitmap, V: TObject>(&self, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, x: c_int, y: c_int, data: &V, shelp: &str, lhelp: &str) {
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
    fn getMargins(&self) -> Point {
        unsafe { Point { ptr: wxToolBar_GetMargins(self.ptr()) } }
    }
    fn getToolBitmapSize(&self) -> Size {
        unsafe { Size { ptr: wxToolBar_GetToolBitmapSize(self.ptr()) } }
    }
    fn getToolClientData(&self, id: c_int) -> Object {
        unsafe { Object { ptr: wxToolBar_GetToolClientData(self.ptr(), id) } }
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
    fn getToolSize(&self) -> Size {
        unsafe { Size { ptr: wxToolBar_GetToolSize(self.ptr()) } }
    }
    fn getToolState(&self, id: c_int) -> c_int {
        unsafe { wxToolBar_GetToolState(self.ptr(), id) }
    }
    fn insertControl<T: TControl>(&self, pos: c_int, ctrl: &T) {
        unsafe { wxToolBar_InsertControl(self.ptr(), pos, ctrl.ptr()) }
    }
    fn insertSeparator(&self, pos: c_int) {
        unsafe { wxToolBar_InsertSeparator(self.ptr(), pos) }
    }
    fn insertTool<T: TBitmap, U: TBitmap, V: TObject>(&self, pos: c_int, id: c_int, bmp1: &T, bmp2: &U, isToggle: c_int, data: &V, shelp: &str, lhelp: &str) {
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
    fn setToolClientData<T: TObject>(&self, id: c_int, data: &T) {
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
    fn addTool2<T: TBitmap, U: TBitmap>(&self, toolId: c_int, label: &str, bmp: &T, bmpDisabled: &U, itemKind: c_int, shortHelp: &str, longHelp: &str) {
        let label = wxT(label);
        let shortHelp = wxT(shortHelp);
        let longHelp = wxT(longHelp);
        unsafe { wxToolBar_AddTool2(self.ptr(), toolId, label.ptr(), bmp.ptr(), bmpDisabled.ptr(), itemKind, shortHelp.ptr(), longHelp.ptr()) }
    }
}

pub struct ToolBarBase { ptr: *mut c_void }
impl TToolBarBase for ToolBarBase {}
impl TControl for ToolBarBase {}
impl TWindow for ToolBarBase {}
impl TEvtHandler for ToolBarBase {}
impl TObject for ToolBarBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolBarBase {
    pub fn from(ptr: *mut c_void) -> ToolBarBase { ToolBarBase { ptr: ptr } }
    pub fn null() -> ToolBarBase { ToolBarBase::from(0 as *mut c_void) }
    
}

pub trait TToolBarBase : TControl {
}

pub struct ToolTip { ptr: *mut c_void }
impl TToolTip for ToolTip {}
impl TObject for ToolTip { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolTip {
    pub fn from(ptr: *mut c_void) -> ToolTip { ToolTip { ptr: ptr } }
    pub fn null() -> ToolTip { ToolTip::from(0 as *mut c_void) }
    
}

pub trait TToolTip : TObject {
}

pub struct TopLevelWindow { ptr: *mut c_void }
impl TTopLevelWindow for TopLevelWindow {}
impl TWindow for TopLevelWindow {}
impl TEvtHandler for TopLevelWindow {}
impl TObject for TopLevelWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TopLevelWindow {
    pub fn from(ptr: *mut c_void) -> TopLevelWindow { TopLevelWindow { ptr: ptr } }
    pub fn null() -> TopLevelWindow { TopLevelWindow::from(0 as *mut c_void) }
    
}

pub trait TTopLevelWindow : TWindow {
    fn enableCloseButton(&self, enable: c_int) -> c_int {
        unsafe { wxTopLevelWindow_EnableCloseButton(self.ptr(), enable) }
    }
    fn getDefaultButton(&self) -> Button {
        unsafe { Button { ptr: wxTopLevelWindow_GetDefaultButton(self.ptr()) } }
    }
    fn getDefaultItem(&self) -> Window {
        unsafe { Window { ptr: wxTopLevelWindow_GetDefaultItem(self.ptr()) } }
    }
    fn getIcon(&self) -> Icon {
        unsafe { Icon { ptr: wxTopLevelWindow_GetIcon(self.ptr()) } }
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
    fn setDefaultButton<T: TButton>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultButton(self.ptr(), pBut.ptr()) }
    }
    fn setDefaultItem<T: TWindow>(&self, pBut: &T) {
        unsafe { wxTopLevelWindow_SetDefaultItem(self.ptr(), pBut.ptr()) }
    }
    fn setIcon<T: TIcon>(&self, pIcon: &T) {
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

pub struct TreeCtrl { ptr: *mut c_void }
impl TTreeCtrl for TreeCtrl {}
impl TControl for TreeCtrl {}
impl TWindow for TreeCtrl {}
impl TEvtHandler for TreeCtrl {}
impl TObject for TreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeCtrl {
    pub fn from(ptr: *mut c_void) -> TreeCtrl { TreeCtrl { ptr: ptr } }
    pub fn null() -> TreeCtrl { TreeCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_obj: *mut c_void, _cmp: *mut c_void, _prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> TreeCtrl {
        unsafe { TreeCtrl { ptr: wxTreeCtrl_Create(_obj, _cmp, _prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
    pub fn new2<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> TreeCtrl {
        unsafe { TreeCtrl { ptr: wxTreeCtrl_Create2(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TTreeCtrl : TControl {
    fn addRoot<T: TTreeItemData, U: TTreeItemId>(&self, text: &str, image: c_int, selectedImage: c_int, data: &T, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AddRoot(self.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    fn appendItem<T: TTreeItemId, U: TTreeItemData, V: TTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_AppendItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data.ptr(), _item.ptr()) }
    }
    fn collapse<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Collapse(self.ptr(), item.ptr()) }
    }
    fn collapseAndReset<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_CollapseAndReset(self.ptr(), item.ptr()) }
    }
    fn deleteAllItems(&self) {
        unsafe { wxTreeCtrl_DeleteAllItems(self.ptr()) }
    }
    fn deleteChildren<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_DeleteChildren(self.ptr(), item.ptr()) }
    }
    fn editLabel<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EditLabel(self.ptr(), item.ptr()) }
    }
    fn endEditLabel<T: TTreeItemId>(&self, item: &T, discardChanges: c_int) {
        unsafe { wxTreeCtrl_EndEditLabel(self.ptr(), item.ptr(), discardChanges) }
    }
    fn ensureVisible<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_EnsureVisible(self.ptr(), item.ptr()) }
    }
    fn expand<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Expand(self.ptr(), item.ptr()) }
    }
    fn getBoundingRect<T: TTreeItemId>(&self, item: &T, textOnly: c_int) -> Rect {
        unsafe { Rect { ptr: wxTreeCtrl_GetBoundingRect(self.ptr(), item.ptr(), textOnly) } }
    }
    fn getChildrenCount<T: TTreeItemId>(&self, item: &T, recursively: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetChildrenCount(self.ptr(), item.ptr(), recursively) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxTreeCtrl_GetCount(self.ptr()) }
    }
    fn getEditControl(&self) -> TextCtrl {
        unsafe { TextCtrl { ptr: wxTreeCtrl_GetEditControl(self.ptr()) } }
    }
    fn getFirstChild<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    fn getFirstVisibleItem<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetFirstVisibleItem(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getImageList(&self) -> ImageList {
        unsafe { ImageList { ptr: wxTreeCtrl_GetImageList(self.ptr()) } }
    }
    fn getIndent(&self) -> c_int {
        unsafe { wxTreeCtrl_GetIndent(self.ptr()) }
    }
    fn getItemData<T: TTreeItemId>(&self, item: &T) -> *mut c_void {
        unsafe { wxTreeCtrl_GetItemData(self.ptr(), item.ptr()) }
    }
    fn getItemImage<T: TTreeItemId>(&self, item: &T, which: c_int) -> c_int {
        unsafe { wxTreeCtrl_GetItemImage(self.ptr(), item.ptr(), which) }
    }
    fn getItemText<T: TTreeItemId>(&self, item: &T) -> ~str {
        unsafe { WxString { ptr: wxTreeCtrl_GetItemText(self.ptr(), item.ptr()) }.to_str() }
    }
    fn getLastChild<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetLastChild(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getNextChild<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, cookie: *mut c_int, _item: &U) {
        unsafe { wxTreeCtrl_GetNextChild(self.ptr(), item.ptr(), cookie, _item.ptr()) }
    }
    fn getNextSibling<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getNextVisible<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetNextVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getPrevSibling<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevSibling(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getPrevVisible<T: TTreeItemId, U: TTreeItemId>(&self, item: &T, _item: &U) {
        unsafe { wxTreeCtrl_GetPrevVisible(self.ptr(), item.ptr(), _item.ptr()) }
    }
    fn getRootItem<T: TTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetRootItem(self.ptr(), _item.ptr()) }
    }
    fn getSelection<T: TTreeItemId>(&self, _item: &T) {
        unsafe { wxTreeCtrl_GetSelection(self.ptr(), _item.ptr()) }
    }
    fn getSelections(&self, selections: *mut c_void) -> c_int {
        unsafe { wxTreeCtrl_GetSelections(self.ptr(), selections) }
    }
    fn getSpacing(&self) -> c_int {
        unsafe { wxTreeCtrl_GetSpacing(self.ptr()) }
    }
    fn getStateImageList(&self) -> ImageList {
        unsafe { ImageList { ptr: wxTreeCtrl_GetStateImageList(self.ptr()) } }
    }
    fn hitTest<T: TTreeItemId>(&self, _x: c_int, _y: c_int, flags: *mut c_int, _item: &T) {
        unsafe { wxTreeCtrl_HitTest(self.ptr(), _x, _y, flags, _item.ptr()) }
    }
    fn insertItem<T: TTreeItemId, U: TTreeItemId, V: TTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn insertItemByIndex<T: TTreeItemId, U: TTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn isBold<T: TTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsBold(self.ptr(), item.ptr()) }
    }
    fn isExpanded<T: TTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsExpanded(self.ptr(), item.ptr()) }
    }
    fn isSelected<T: TTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsSelected(self.ptr(), item.ptr()) }
    }
    fn isVisible<T: TTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_IsVisible(self.ptr(), item.ptr()) }
    }
    fn itemHasChildren<T: TTreeItemId>(&self, item: &T) -> c_int {
        unsafe { wxTreeCtrl_ItemHasChildren(self.ptr(), item.ptr()) }
    }
    fn onCompareItems<T: TTreeItemId, U: TTreeItemId>(&self, item1: &T, item2: &U) -> c_int {
        unsafe { wxTreeCtrl_OnCompareItems(self.ptr(), item1.ptr(), item2.ptr()) }
    }
    fn prependItem<T: TTreeItemId, U: TTreeItemId>(&self, parent: &T, text: &str, image: c_int, selectedImage: c_int, data: *mut c_void, _item: &U) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_PrependItem(self.ptr(), parent.ptr(), text.ptr(), image, selectedImage, data, _item.ptr()) }
    }
    fn scrollTo<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_ScrollTo(self.ptr(), item.ptr()) }
    }
    fn selectItem<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SelectItem(self.ptr(), item.ptr()) }
    }
    fn setImageList<T: TImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetImageList(self.ptr(), imageList.ptr()) }
    }
    fn setIndent(&self, indent: c_int) {
        unsafe { wxTreeCtrl_SetIndent(self.ptr(), indent) }
    }
    fn setItemBackgroundColour<T: TTreeItemId, U: TColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemBackgroundColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    fn setItemBold<T: TTreeItemId>(&self, item: &T, bold: c_int) {
        unsafe { wxTreeCtrl_SetItemBold(self.ptr(), item.ptr(), bold) }
    }
    fn setItemData<T: TTreeItemId>(&self, item: &T, data: *mut c_void) {
        unsafe { wxTreeCtrl_SetItemData(self.ptr(), item.ptr(), data) }
    }
    fn setItemDropHighlight<T: TTreeItemId>(&self, item: &T, highlight: c_int) {
        unsafe { wxTreeCtrl_SetItemDropHighlight(self.ptr(), item.ptr(), highlight) }
    }
    fn setItemFont<T: TTreeItemId, U: TFont>(&self, item: &T, font: &U) {
        unsafe { wxTreeCtrl_SetItemFont(self.ptr(), item.ptr(), font.ptr()) }
    }
    fn setItemHasChildren<T: TTreeItemId>(&self, item: &T, hasChildren: c_int) {
        unsafe { wxTreeCtrl_SetItemHasChildren(self.ptr(), item.ptr(), hasChildren) }
    }
    fn setItemImage<T: TTreeItemId>(&self, item: &T, image: c_int, which: c_int) {
        unsafe { wxTreeCtrl_SetItemImage(self.ptr(), item.ptr(), image, which) }
    }
    fn setItemText<T: TTreeItemId>(&self, item: &T, text: &str) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_SetItemText(self.ptr(), item.ptr(), text.ptr()) }
    }
    fn setItemTextColour<T: TTreeItemId, U: TColour>(&self, item: &T, col: &U) {
        unsafe { wxTreeCtrl_SetItemTextColour(self.ptr(), item.ptr(), col.ptr()) }
    }
    fn setSpacing(&self, spacing: c_int) {
        unsafe { wxTreeCtrl_SetSpacing(self.ptr(), spacing) }
    }
    fn setStateImageList<T: TImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_SetStateImageList(self.ptr(), imageList.ptr()) }
    }
    fn sortChildren<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_SortChildren(self.ptr(), item.ptr()) }
    }
    fn toggle<T: TTreeItemId>(&self, item: &T) {
        unsafe { wxTreeCtrl_Toggle(self.ptr(), item.ptr()) }
    }
    fn unselect(&self) {
        unsafe { wxTreeCtrl_Unselect(self.ptr()) }
    }
    fn unselectAll(&self) {
        unsafe { wxTreeCtrl_UnselectAll(self.ptr()) }
    }
    fn insertItem2<T: TWindow, U: TTreeItemId, V: TClosure, W: TTreeItemId>(&self, parent: &T, idPrevious: &U, text: &str, image: c_int, selectedImage: c_int, closure: &V, _item: &W) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItem2(self.ptr(), parent.ptr(), idPrevious.ptr(), text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    fn insertItemByIndex2<T: TWindow, U: TClosure, V: TTreeItemId>(&self, parent: &T, index: c_int, text: &str, image: c_int, selectedImage: c_int, closure: &U, _item: &V) {
        let text = wxT(text);
        unsafe { wxTreeCtrl_InsertItemByIndex2(self.ptr(), parent.ptr(), index, text.ptr(), image, selectedImage, closure.ptr(), _item.ptr()) }
    }
    fn getItemClientClosure<T: TTreeItemId>(&self, item: &T) -> Closure {
        unsafe { Closure { ptr: wxTreeCtrl_GetItemClientClosure(self.ptr(), item.ptr()) } }
    }
    fn setItemClientClosure<T: TTreeItemId, U: TClosure>(&self, item: &T, closure: &U) {
        unsafe { wxTreeCtrl_SetItemClientClosure(self.ptr(), item.ptr(), closure.ptr()) }
    }
    fn assignImageList<T: TImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignImageList(self.ptr(), imageList.ptr()) }
    }
    fn assignStateImageList<T: TImageList>(&self, imageList: &T) {
        unsafe { wxTreeCtrl_AssignStateImageList(self.ptr(), imageList.ptr()) }
    }
}

pub struct TreeEvent { ptr: *mut c_void }
impl TTreeEvent for TreeEvent {}
impl TNotifyEvent for TreeEvent {}
impl TCommandEvent for TreeEvent {}
impl TEvent for TreeEvent {}
impl TObject for TreeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeEvent {
    pub fn from(ptr: *mut c_void) -> TreeEvent { TreeEvent { ptr: ptr } }
    pub fn null() -> TreeEvent { TreeEvent::from(0 as *mut c_void) }
    
}

pub trait TTreeEvent : TNotifyEvent {
    fn getCode(&self) -> c_int {
        unsafe { wxTreeEvent_GetCode(self.ptr()) }
    }
    fn getItem<T: TTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetItem(self.ptr(), _ref.ptr()) }
    }
    fn getLabel(&self) -> ~str {
        unsafe { WxString { ptr: wxTreeEvent_GetLabel(self.ptr()) }.to_str() }
    }
    fn getOldItem<T: TTreeItemId>(&self, _ref: &T) {
        unsafe { wxTreeEvent_GetOldItem(self.ptr(), _ref.ptr()) }
    }
    fn getPoint(&self) -> Point {
        unsafe { Point { ptr: wxTreeEvent_GetPoint(self.ptr()) } }
    }
    fn getKeyEvent(&self) -> KeyEvent {
        unsafe { KeyEvent { ptr: wxTreeEvent_GetKeyEvent(self.ptr()) } }
    }
    fn isEditCancelled(&self) -> c_int {
        unsafe { wxTreeEvent_IsEditCancelled(self.ptr()) }
    }
}

pub struct TreeItemData { ptr: *mut c_void }
impl TTreeItemData for TreeItemData {}
impl TClientData for TreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeItemData {
    pub fn from(ptr: *mut c_void) -> TreeItemData { TreeItemData { ptr: ptr } }
    pub fn null() -> TreeItemData { TreeItemData::from(0 as *mut c_void) }
    
}

pub trait TTreeItemData : TClientData {
}

pub struct TreeItemId { ptr: *mut c_void }
impl TTreeItemId for TreeItemId { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeItemId {
    pub fn from(ptr: *mut c_void) -> TreeItemId { TreeItemId { ptr: ptr } }
    pub fn null() -> TreeItemId { TreeItemId::from(0 as *mut c_void) }
    
    pub fn new() -> TreeItemId {
        unsafe { TreeItemId { ptr: wxTreeItemId_Create() } }
    }
    pub fn newFromValue(value: intptr_t) -> TreeItemId {
        unsafe { TreeItemId { ptr: wxTreeItemId_CreateFromValue(value) } }
    }
}

pub trait TTreeItemId {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxTreeItemId_Delete(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTreeItemId_IsOk(self.ptr()) }
    }
    fn clone(&self) -> TreeItemId {
        unsafe { TreeItemId { ptr: wxTreeItemId_Clone(self.ptr()) } }
    }
    fn getValue(&self) -> intptr_t {
        unsafe { wxTreeItemId_GetValue(self.ptr()) }
    }
}

pub struct UpdateUIEvent { ptr: *mut c_void }
impl TUpdateUIEvent for UpdateUIEvent {}
impl TEvent for UpdateUIEvent {}
impl TObject for UpdateUIEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl UpdateUIEvent {
    pub fn from(ptr: *mut c_void) -> UpdateUIEvent { UpdateUIEvent { ptr: ptr } }
    pub fn null() -> UpdateUIEvent { UpdateUIEvent::from(0 as *mut c_void) }
    
}

pub trait TUpdateUIEvent : TEvent {
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

pub struct Validator { ptr: *mut c_void }
impl TValidator for Validator {}
impl TEvtHandler for Validator {}
impl TObject for Validator { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Validator {
    pub fn from(ptr: *mut c_void) -> Validator { Validator { ptr: ptr } }
    pub fn null() -> Validator { Validator::from(0 as *mut c_void) }
    
    pub fn new() -> Validator {
        unsafe { Validator { ptr: wxValidator_Create() } }
    }
    pub fn setBellOnError(doIt: c_int) {
        unsafe { wxValidator_SetBellOnError(doIt) }
    }
}

pub trait TValidator : TEvtHandler {
    fn getWindow(&self) -> Window {
        unsafe { Window { ptr: wxValidator_GetWindow(self.ptr()) } }
    }
    fn setWindow<T: TWindow>(&self, win: &T) {
        unsafe { wxValidator_SetWindow(self.ptr(), win.ptr()) }
    }
    fn transferFromWindow(&self) -> c_int {
        unsafe { wxValidator_TransferFromWindow(self.ptr()) }
    }
    fn transferToWindow(&self) -> c_int {
        unsafe { wxValidator_TransferToWindow(self.ptr()) }
    }
    fn validate<T: TWindow>(&self, parent: &T) -> c_int {
        unsafe { wxValidator_Validate(self.ptr(), parent.ptr()) }
    }
}

pub struct View { ptr: *mut c_void }
impl TView for View {}
impl TEvtHandler for View {}
impl TObject for View { fn ptr(&self) -> *mut c_void { self.ptr } }

impl View {
    pub fn from(ptr: *mut c_void) -> View { View { ptr: ptr } }
    pub fn null() -> View { View::from(0 as *mut c_void) }
    
}

pub trait TView : TEvtHandler {
}

pub struct Sound { ptr: *mut c_void }
impl TSound for Sound {}
impl TEvtHandler for Sound {}
impl TObject for Sound { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Sound {
    pub fn from(ptr: *mut c_void) -> Sound { Sound { ptr: ptr } }
    pub fn null() -> Sound { Sound::from(0 as *mut c_void) }
    
    pub fn new(fileName: &str, isResource: c_int) -> Sound {
        let fileName = wxT(fileName);
        unsafe { Sound { ptr: wxSound_Create(fileName.ptr(), isResource) } }
    }
}

pub trait TSound : TEvtHandler {
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

pub struct Window { ptr: *mut c_void }
impl TWindow for Window {}
impl TEvtHandler for Window {}
impl TObject for Window { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Window {
    pub fn from(ptr: *mut c_void) -> Window { Window { ptr: ptr } }
    pub fn null() -> Window { Window::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> Window {
        unsafe { Window { ptr: wxWindow_Create(_prt.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait TWindow : TEvtHandler {
    fn addChild<T: TWindow>(&self, child: &T) {
        unsafe { wxWindow_AddChild(self.ptr(), child.ptr()) }
    }
    fn addConstraintReference<T: TWindow>(&self, otherWin: &T) {
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
        unsafe { Point { ptr: wxWindow_ClientToScreen(self.ptr(), x, y) } }
    }
    fn close(&self, _force: c_int) -> c_int {
        unsafe { wxWindow_Close(self.ptr(), _force) }
    }
    fn convertDialogToPixels(&self) -> Point {
        unsafe { Point { ptr: wxWindow_ConvertDialogToPixels(self.ptr()) } }
    }
    fn convertPixelsToDialog(&self) -> Point {
        unsafe { Point { ptr: wxWindow_ConvertPixelsToDialog(self.ptr()) } }
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
        unsafe { Window { ptr: wxWindow_FindFocus(self.ptr()) } }
    }
    fn findWindow(&self, name: &str) -> Window {
        let name = wxT(name);
        unsafe { Window { ptr: wxWindow_FindWindow(self.ptr(), name.ptr()) } }
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
        unsafe { Size { ptr: wxWindow_GetEffectiveMinSize(self.ptr()) } }
    }
    fn getAutoLayout(&self) -> c_int {
        unsafe { wxWindow_GetAutoLayout(self.ptr()) }
    }
    fn getBackgroundColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxWindow_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getBestSize(&self) -> Size {
        unsafe { Size { ptr: wxWindow_GetBestSize(self.ptr()) } }
    }
    fn getCaret(&self) -> Caret {
        unsafe { Caret { ptr: wxWindow_GetCaret(self.ptr()) } }
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
        unsafe { ClientData { ptr: wxWindow_GetClientData(self.ptr()) } }
    }
    fn getClientSize(&self) -> Size {
        unsafe { Size { ptr: wxWindow_GetClientSize(self.ptr()) } }
    }
    fn getClientSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetClientSizeConstraint(self.ptr(), _w, _h) }
    }
    fn getConstraints(&self) -> LayoutConstraints {
        unsafe { LayoutConstraints { ptr: wxWindow_GetConstraints(self.ptr()) } }
    }
    fn getConstraintsInvolvedIn(&self) -> *mut c_void {
        unsafe { wxWindow_GetConstraintsInvolvedIn(self.ptr()) }
    }
    fn getCursor(&self) -> Cursor {
        unsafe { Cursor { ptr: wxWindow_GetCursor(self.ptr()) } }
    }
    fn getDropTarget(&self) -> DropTarget {
        unsafe { DropTarget { ptr: wxWindow_GetDropTarget(self.ptr()) } }
    }
    fn getEventHandler(&self) -> EvtHandler {
        unsafe { EvtHandler { ptr: wxWindow_GetEventHandler(self.ptr()) } }
    }
    fn getFont<T: TFont>(&self, _ref: &T) {
        unsafe { wxWindow_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getForegroundColour<T: TColour>(&self, _ref: &T) {
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
    fn getParent(&self) -> Window {
        unsafe { Window { ptr: wxWindow_GetParent(self.ptr()) } }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxWindow_GetPosition(self.ptr()) } }
    }
    fn getPositionConstraint(&self, _x: *mut c_int, _y: *mut c_int) {
        unsafe { wxWindow_GetPositionConstraint(self.ptr(), _x, _y) }
    }
    fn getRect(&self) -> Rect {
        unsafe { Rect { ptr: wxWindow_GetRect(self.ptr()) } }
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
        unsafe { Size { ptr: wxWindow_GetSize(self.ptr()) } }
    }
    fn getSizeConstraint(&self, _w: *mut c_int, _h: *mut c_int) {
        unsafe { wxWindow_GetSizeConstraint(self.ptr(), _w, _h) }
    }
    fn getSizer(&self) -> Sizer {
        unsafe { Sizer { ptr: wxWindow_GetSizer(self.ptr()) } }
    }
    fn getTextExtent<T: TFont>(&self, string: &str, x: *mut c_int, y: *mut c_int, descent: *mut c_int, externalLeading: *mut c_int, theFont: &T) {
        let string = wxT(string);
        unsafe { wxWindow_GetTextExtent(self.ptr(), string.ptr(), x, y, descent, externalLeading, theFont.ptr()) }
    }
    fn getToolTip(&self) -> ~str {
        unsafe { WxString { ptr: wxWindow_GetToolTip(self.ptr()) }.to_str() }
    }
    fn getUpdateRegion(&self) -> Region {
        unsafe { Region { ptr: wxWindow_GetUpdateRegion(self.ptr()) } }
    }
    fn getValidator(&self) -> Validator {
        unsafe { Validator { ptr: wxWindow_GetValidator(self.ptr()) } }
    }
    fn getVirtualSize(&self) -> Size {
        unsafe { Size { ptr: wxWindow_GetVirtualSize(self.ptr()) } }
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
    fn popupMenu<T: TMenu>(&self, menu: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxWindow_PopupMenu(self.ptr(), menu.ptr(), x, y) }
    }
    fn prepareDC<T: TDC>(&self, dc: &T) {
        unsafe { wxWindow_PrepareDC(self.ptr(), dc.ptr()) }
    }
    fn pushEventHandler<T: TEvtHandler>(&self, handler: &T) {
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
    fn removeChild<T: TWindow>(&self, child: &T) {
        unsafe { wxWindow_RemoveChild(self.ptr(), child.ptr()) }
    }
    fn removeConstraintReference<T: TWindow>(&self, otherWin: &T) {
        unsafe { wxWindow_RemoveConstraintReference(self.ptr(), otherWin.ptr()) }
    }
    fn reparent<T: TWindow>(&self, _par: &T) -> c_int {
        unsafe { wxWindow_Reparent(self.ptr(), _par.ptr()) }
    }
    fn resetConstraints(&self) {
        unsafe { wxWindow_ResetConstraints(self.ptr()) }
    }
    fn screenToClient(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point { ptr: wxWindow_ScreenToClient(self.ptr(), x, y) } }
    }
    fn scrollWindow(&self, dx: c_int, dy: c_int) {
        unsafe { wxWindow_ScrollWindow(self.ptr(), dx, dy) }
    }
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxWindow_ScrollWindowRect(self.ptr(), dx, dy, x, y, w, h) }
    }
    fn setAcceleratorTable<T: TAcceleratorTable>(&self, accel: &T) {
        unsafe { wxWindow_SetAcceleratorTable(self.ptr(), accel.ptr()) }
    }
    fn setAutoLayout(&self, autoLayout: c_int) {
        unsafe { wxWindow_SetAutoLayout(self.ptr(), autoLayout) }
    }
    fn setBackgroundColour<T: TColour>(&self, colour: &T) -> c_int {
        unsafe { wxWindow_SetBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setCaret<T: TCaret>(&self, caret: &T) {
        unsafe { wxWindow_SetCaret(self.ptr(), caret.ptr()) }
    }
    fn setClientData<T: TClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientData(self.ptr(), data.ptr()) }
    }
    fn setClientObject<T: TClientData>(&self, data: &T) {
        unsafe { wxWindow_SetClientObject(self.ptr(), data.ptr()) }
    }
    fn setClientSize(&self, width: c_int, height: c_int) {
        unsafe { wxWindow_SetClientSize(self.ptr(), width, height) }
    }
    fn setConstraintSizes(&self, recurse: c_int) {
        unsafe { wxWindow_SetConstraintSizes(self.ptr(), recurse) }
    }
    fn setConstraints<T: TLayoutConstraints>(&self, constraints: &T) {
        unsafe { wxWindow_SetConstraints(self.ptr(), constraints.ptr()) }
    }
    fn setCursor<T: TCursor>(&self, cursor: &T) -> c_int {
        unsafe { wxWindow_SetCursor(self.ptr(), cursor.ptr()) }
    }
    fn setDropTarget<T: TDropTarget>(&self, dropTarget: &T) {
        unsafe { wxWindow_SetDropTarget(self.ptr(), dropTarget.ptr()) }
    }
    fn setExtraStyle(&self, exStyle: c_long) {
        unsafe { wxWindow_SetExtraStyle(self.ptr(), exStyle) }
    }
    fn setFocus(&self) {
        unsafe { wxWindow_SetFocus(self.ptr()) }
    }
    fn setFont<T: TFont>(&self, font: &T) -> c_int {
        unsafe { wxWindow_SetFont(self.ptr(), font.ptr()) }
    }
    fn setForegroundColour<T: TColour>(&self, colour: &T) -> c_int {
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
    fn setSizer<T: TSizer>(&self, sizer: &T) {
        unsafe { wxWindow_SetSizer(self.ptr(), sizer.ptr()) }
    }
    fn setToolTip(&self, tip: &str) {
        let tip = wxT(tip);
        unsafe { wxWindow_SetToolTip(self.ptr(), tip.ptr()) }
    }
    fn setValidator<T: TValidator>(&self, validator: &T) {
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
        unsafe { Point { ptr: wxWindow_ConvertDialogToPixelsEx(self.ptr()) } }
    }
    fn convertPixelsToDialogEx(&self) -> Point {
        unsafe { Point { ptr: wxWindow_ConvertPixelsToDialogEx(self.ptr()) } }
    }
    fn screenToClient2(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point { ptr: wxWindow_ScreenToClient2(self.ptr(), x, y) } }
    }
}

pub struct WindowCreateEvent { ptr: *mut c_void }
impl TWindowCreateEvent for WindowCreateEvent {}
impl TCommandEvent for WindowCreateEvent {}
impl TEvent for WindowCreateEvent {}
impl TObject for WindowCreateEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowCreateEvent {
    pub fn from(ptr: *mut c_void) -> WindowCreateEvent { WindowCreateEvent { ptr: ptr } }
    pub fn null() -> WindowCreateEvent { WindowCreateEvent::from(0 as *mut c_void) }
    
}

pub trait TWindowCreateEvent : TCommandEvent {
    fn getWindow(&self) -> Window {
        unsafe { Window { ptr: wxWindowCreateEvent_GetWindow(self.ptr()) } }
    }
}

pub struct WindowDC { ptr: *mut c_void }
impl TWindowDC for WindowDC {}
impl TDC for WindowDC {}
impl TObject for WindowDC { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowDC {
    pub fn from(ptr: *mut c_void) -> WindowDC { WindowDC { ptr: ptr } }
    pub fn null() -> WindowDC { WindowDC::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(win: &T) -> WindowDC {
        unsafe { WindowDC { ptr: wxWindowDC_Create(win.ptr()) } }
    }
}

pub trait TWindowDC : TDC {
}

pub struct WindowDestroyEvent { ptr: *mut c_void }
impl TWindowDestroyEvent for WindowDestroyEvent {}
impl TCommandEvent for WindowDestroyEvent {}
impl TEvent for WindowDestroyEvent {}
impl TObject for WindowDestroyEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowDestroyEvent {
    pub fn from(ptr: *mut c_void) -> WindowDestroyEvent { WindowDestroyEvent { ptr: ptr } }
    pub fn null() -> WindowDestroyEvent { WindowDestroyEvent::from(0 as *mut c_void) }
    
}

pub trait TWindowDestroyEvent : TCommandEvent {
    fn getWindow(&self) -> Window {
        unsafe { Window { ptr: wxWindowDestroyEvent_GetWindow(self.ptr()) } }
    }
}

pub struct WindowDisabler { ptr: *mut c_void }
impl TWindowDisabler for WindowDisabler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WindowDisabler {
    pub fn from(ptr: *mut c_void) -> WindowDisabler { WindowDisabler { ptr: ptr } }
    pub fn null() -> WindowDisabler { WindowDisabler::from(0 as *mut c_void) }
    
}

pub trait TWindowDisabler {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct XmlResourceHandler { ptr: *mut c_void }
impl TXmlResourceHandler for XmlResourceHandler {}
impl TObject for XmlResourceHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl XmlResourceHandler {
    pub fn from(ptr: *mut c_void) -> XmlResourceHandler { XmlResourceHandler { ptr: ptr } }
    pub fn null() -> XmlResourceHandler { XmlResourceHandler::from(0 as *mut c_void) }
    
}

pub trait TXmlResourceHandler : TObject {
}

pub struct GenericDragImage { ptr: *mut c_void }
impl TGenericDragImage for GenericDragImage {}
impl TDragImage for GenericDragImage {}
impl TObject for GenericDragImage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GenericDragImage {
    pub fn from(ptr: *mut c_void) -> GenericDragImage { GenericDragImage { ptr: ptr } }
    pub fn null() -> GenericDragImage { GenericDragImage::from(0 as *mut c_void) }
    
    pub fn new<T: TCursor>(cursor: &T) -> GenericDragImage {
        unsafe { GenericDragImage { ptr: wxGenericDragImage_Create(cursor.ptr()) } }
    }
}

pub trait TGenericDragImage : TDragImage {
    fn doDrawImage<T: TDC>(&self, dc: &T, x: c_int, y: c_int) -> c_int {
        unsafe { wxGenericDragImage_DoDrawImage(self.ptr(), dc.ptr(), x, y) }
    }
    fn getImageRect(&self, x_pos: c_int, y_pos: c_int) -> Rect {
        unsafe { Rect { ptr: wxGenericDragImage_GetImageRect(self.ptr(), x_pos, y_pos) } }
    }
    fn updateBackingFromWindow<T: TDC, U: TMemoryDC>(&self, windowDC: &T, destDC: &U, x: c_int, y: c_int, w: c_int, h: c_int, xdest: c_int, ydest: c_int, width: c_int, height: c_int) -> c_int {
        unsafe { wxGenericDragImage_UpdateBackingFromWindow(self.ptr(), windowDC.ptr(), destDC.ptr(), x, y, w, h, xdest, ydest, width, height) }
    }
}

pub struct GraphicsObject { ptr: *mut c_void }
impl TGraphicsObject for GraphicsObject {}
impl TObject for GraphicsObject { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsObject {
    pub fn from(ptr: *mut c_void) -> GraphicsObject { GraphicsObject { ptr: ptr } }
    pub fn null() -> GraphicsObject { GraphicsObject::from(0 as *mut c_void) }
    
    pub fn getRenderer() -> GraphicsRenderer {
        unsafe { GraphicsRenderer { ptr: wxGraphicsObject_GetRenderer() } }
    }
}

pub trait TGraphicsObject : TObject {
    fn isNull(&self) -> c_int {
        unsafe { wxGraphicsObject_IsNull(self.ptr()) }
    }
}

pub struct GraphicsBrush { ptr: *mut c_void }
impl TGraphicsBrush for GraphicsBrush {}
impl TGraphicsObject for GraphicsBrush {}
impl TObject for GraphicsBrush { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsBrush {
    pub fn from(ptr: *mut c_void) -> GraphicsBrush { GraphicsBrush { ptr: ptr } }
    pub fn null() -> GraphicsBrush { GraphicsBrush::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsBrush {
        unsafe { GraphicsBrush { ptr: wxGraphicsBrush_Create() } }
    }
}

pub trait TGraphicsBrush : TGraphicsObject {
}

pub struct GraphicsContext { ptr: *mut c_void }
impl TGraphicsContext for GraphicsContext {}
impl TGraphicsObject for GraphicsContext {}
impl TObject for GraphicsContext { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsContext {
    pub fn from(ptr: *mut c_void) -> GraphicsContext { GraphicsContext { ptr: ptr } }
    pub fn null() -> GraphicsContext { GraphicsContext::from(0 as *mut c_void) }
    
    pub fn new<T: TWindowDC>(dc: &T) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsContext_Create(dc.ptr()) } }
    }
    pub fn newFromWindow<T: TWindow>(window: &T) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsContext_CreateFromWindow(window.ptr()) } }
    }
    pub fn newFromNative(context: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsContext_CreateFromNative(context) } }
    }
    pub fn newFromNativeWindow(window: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsContext_CreateFromNativeWindow(window) } }
    }
}

pub trait TGraphicsContext : TGraphicsObject {
    fn clip<T: TRegion>(&self, region: &T) {
        unsafe { wxGraphicsContext_Clip(self.ptr(), region.ptr()) }
    }
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_ClipByRectangle(self.ptr(), x, y, w, h) }
    }
    fn resetClip(&self) {
        unsafe { wxGraphicsContext_ResetClip(self.ptr()) }
    }
    fn drawBitmap<T: TBitmap>(&self, bmp: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawBitmap(self.ptr(), bmp.ptr(), x, y, w, h) }
    }
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawEllipse(self.ptr(), x, y, w, h) }
    }
    fn drawIcon<T: TIcon>(&self, icon: &T, x: c_double, y: c_double, w: c_double, h: c_double) {
        unsafe { wxGraphicsContext_DrawIcon(self.ptr(), icon.ptr(), x, y, w, h) }
    }
    fn drawLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_DrawLines(self.ptr(), n, x, y, style) }
    }
    fn drawPath<T: TGraphicsPath>(&self, path: &T, style: c_int) {
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
    fn fillPath<T: TGraphicsPath>(&self, path: &T, style: c_int) {
        unsafe { wxGraphicsContext_FillPath(self.ptr(), path.ptr(), style) }
    }
    fn strokePath<T: TGraphicsPath>(&self, path: &T) {
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
    fn setTransform<T: TGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_SetTransform(self.ptr(), path.ptr()) }
    }
    fn concatTransform<T: TGraphicsMatrix>(&self, path: &T) {
        unsafe { wxGraphicsContext_ConcatTransform(self.ptr(), path.ptr()) }
    }
    fn setBrush<T: TBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetBrush(self.ptr(), brush.ptr()) }
    }
    fn setGraphicsBrush<T: TGraphicsBrush>(&self, brush: &T) {
        unsafe { wxGraphicsContext_SetGraphicsBrush(self.ptr(), brush.ptr()) }
    }
    fn setFont<T: TFont, U: TColour>(&self, font: &T, colour: &U) {
        unsafe { wxGraphicsContext_SetFont(self.ptr(), font.ptr(), colour.ptr()) }
    }
    fn setGraphicsFont<T: TGraphicsFont>(&self, font: &T) {
        unsafe { wxGraphicsContext_SetGraphicsFont(self.ptr(), font.ptr()) }
    }
    fn setPen<T: TPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetPen(self.ptr(), pen.ptr()) }
    }
    fn setGraphicsPen<T: TGraphicsPen>(&self, pen: &T) {
        unsafe { wxGraphicsContext_SetGraphicsPen(self.ptr(), pen.ptr()) }
    }
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double) {
        unsafe { wxGraphicsContext_StrokeLine(self.ptr(), x1, y1, x2, y2) }
    }
    fn strokeLines(&self, n: size_t, x: *mut c_void, y: *mut c_void, style: c_int) {
        unsafe { wxGraphicsContext_StrokeLines(self.ptr(), n, x, y, style) }
    }
}

pub struct GraphicsFont { ptr: *mut c_void }
impl TGraphicsFont for GraphicsFont {}
impl TGraphicsObject for GraphicsFont {}
impl TObject for GraphicsFont { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsFont {
    pub fn from(ptr: *mut c_void) -> GraphicsFont { GraphicsFont { ptr: ptr } }
    pub fn null() -> GraphicsFont { GraphicsFont::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsFont {
        unsafe { GraphicsFont { ptr: wxGraphicsFont_Create() } }
    }
}

pub trait TGraphicsFont : TGraphicsObject {
}

pub struct GraphicsMatrix { ptr: *mut c_void }
impl TGraphicsMatrix for GraphicsMatrix {}
impl TGraphicsObject for GraphicsMatrix {}
impl TObject for GraphicsMatrix { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsMatrix {
    pub fn from(ptr: *mut c_void) -> GraphicsMatrix { GraphicsMatrix { ptr: ptr } }
    pub fn null() -> GraphicsMatrix { GraphicsMatrix::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsMatrix {
        unsafe { GraphicsMatrix { ptr: wxGraphicsMatrix_Create() } }
    }
}

pub trait TGraphicsMatrix : TGraphicsObject {
    fn concat<T: TGraphicsMatrix>(&self, t: &T) {
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
    fn isEqual<T: TGraphicsMatrix>(&self, t: &T) -> c_int {
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

pub struct GraphicsPath { ptr: *mut c_void }
impl TGraphicsPath for GraphicsPath {}
impl TGraphicsObject for GraphicsPath {}
impl TObject for GraphicsPath { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsPath {
    pub fn from(ptr: *mut c_void) -> GraphicsPath { GraphicsPath { ptr: ptr } }
    pub fn null() -> GraphicsPath { GraphicsPath::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsPath {
        unsafe { GraphicsPath { ptr: wxGraphicsPath_Create() } }
    }
    pub fn unGetNativePath(p: *mut c_void) {
        unsafe { wxGraphicsPath_UnGetNativePath(p) }
    }
}

pub trait TGraphicsPath : TGraphicsObject {
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
    fn addPath<T: TGraphicsPath>(&self, x: c_double, y: c_double, path: &T) {
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
    fn transform<T: TGraphicsMatrix>(&self, matrix: &T) {
        unsafe { wxGraphicsPath_Transform(self.ptr(), matrix.ptr()) }
    }
    fn getNativePath(&self) -> *mut c_void {
        unsafe { wxGraphicsPath_GetNativePath(self.ptr()) }
    }
}

pub struct GraphicsPen { ptr: *mut c_void }
impl TGraphicsPen for GraphicsPen {}
impl TGraphicsObject for GraphicsPen {}
impl TObject for GraphicsPen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsPen {
    pub fn from(ptr: *mut c_void) -> GraphicsPen { GraphicsPen { ptr: ptr } }
    pub fn null() -> GraphicsPen { GraphicsPen::from(0 as *mut c_void) }
    
    pub fn new() -> GraphicsPen {
        unsafe { GraphicsPen { ptr: wxGraphicsPen_Create() } }
    }
}

pub trait TGraphicsPen : TGraphicsObject {
}

pub struct GraphicsRenderer { ptr: *mut c_void }
impl TGraphicsRenderer for GraphicsRenderer {}
impl TGraphicsObject for GraphicsRenderer {}
impl TObject for GraphicsRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GraphicsRenderer {
    pub fn from(ptr: *mut c_void) -> GraphicsRenderer { GraphicsRenderer { ptr: ptr } }
    pub fn null() -> GraphicsRenderer { GraphicsRenderer::from(0 as *mut c_void) }
    
    pub fn newContext<T: TWindowDC>(dc: &T) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsRenderer_CreateContext(dc.ptr()) } }
    }
    pub fn newContextFromWindow<T: TWindow>(window: &T) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromWindow(window.ptr()) } }
    }
    pub fn newContextFromNativeContext(context: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromNativeContext(context) } }
    }
    pub fn newContextFromNativeWindow(window: *mut c_void) -> GraphicsContext {
        unsafe { GraphicsContext { ptr: wxGraphicsRenderer_CreateContextFromNativeWindow(window) } }
    }
}

pub trait TGraphicsRenderer : TGraphicsObject {
    fn getDefaultRenderer(&self) -> GraphicsRenderer {
        unsafe { GraphicsRenderer { ptr: wxGraphicsRenderer_GetDefaultRenderer(self.ptr()) } }
    }
}

pub struct CPrintout { ptr: *mut c_void }
impl TCPrintout for CPrintout {}
impl TPrintout for CPrintout {}
impl TObject for CPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CPrintout {
    pub fn from(ptr: *mut c_void) -> CPrintout { CPrintout { ptr: ptr } }
    pub fn null() -> CPrintout { CPrintout::from(0 as *mut c_void) }
    
    pub fn new(title: &str) -> CPrintout {
        let title = wxT(title);
        unsafe { CPrintout { ptr: wxcPrintout_Create(title.ptr()) } }
    }
}

pub trait TCPrintout : TPrintout {
    fn setPageLimits(&self, startPage: c_int, endPage: c_int, fromPage: c_int, toPage: c_int) {
        unsafe { wxcPrintout_SetPageLimits(self.ptr(), startPage, endPage, fromPage, toPage) }
    }
    fn getEvtHandler(&self) -> CPrintoutHandler {
        unsafe { CPrintoutHandler { ptr: wxcPrintout_GetEvtHandler(self.ptr()) } }
    }
}

pub struct CPrintEvent { ptr: *mut c_void }
impl TCPrintEvent for CPrintEvent {}
impl TEvent for CPrintEvent {}
impl TObject for CPrintEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CPrintEvent {
    pub fn from(ptr: *mut c_void) -> CPrintEvent { CPrintEvent { ptr: ptr } }
    pub fn null() -> CPrintEvent { CPrintEvent::from(0 as *mut c_void) }
    
}

pub trait TCPrintEvent : TEvent {
    fn getPrintout(&self) -> CPrintout {
        unsafe { CPrintout { ptr: wxcPrintEvent_GetPrintout(self.ptr()) } }
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

pub struct CPrintoutHandler { ptr: *mut c_void }
impl TCPrintoutHandler for CPrintoutHandler {}
impl TEvtHandler for CPrintoutHandler {}
impl TObject for CPrintoutHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CPrintoutHandler {
    pub fn from(ptr: *mut c_void) -> CPrintoutHandler { CPrintoutHandler { ptr: ptr } }
    pub fn null() -> CPrintoutHandler { CPrintoutHandler::from(0 as *mut c_void) }
    
}

pub trait TCPrintoutHandler : TEvtHandler {
}

pub struct CTreeItemData { ptr: *mut c_void }
impl TCTreeItemData for CTreeItemData {}
impl TTreeItemData for CTreeItemData {}
impl TClientData for CTreeItemData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CTreeItemData {
    pub fn from(ptr: *mut c_void) -> CTreeItemData { CTreeItemData { ptr: ptr } }
    pub fn null() -> CTreeItemData { CTreeItemData::from(0 as *mut c_void) }
    
    pub fn new<T: TClosure>(closure: &T) -> CTreeItemData {
        unsafe { CTreeItemData { ptr: wxcTreeItemData_Create(closure.ptr()) } }
    }
}

pub trait TCTreeItemData : TTreeItemData {
    fn getClientClosure(&self) -> Closure {
        unsafe { Closure { ptr: wxcTreeItemData_GetClientClosure(self.ptr()) } }
    }
    fn setClientClosure<T: TClosure>(&self, closure: &T) {
        unsafe { wxcTreeItemData_SetClientClosure(self.ptr(), closure.ptr()) }
    }
}

pub struct InputSink { ptr: *mut c_void }
impl TInputSink for InputSink {}
impl TThread for InputSink { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InputSink {
    pub fn from(ptr: *mut c_void) -> InputSink { InputSink { ptr: ptr } }
    pub fn null() -> InputSink { InputSink::from(0 as *mut c_void) }
    
    pub fn new<T: TInputStream, U: TEvtHandler>(input: &T, evtHandler: &U, bufferLen: c_int) -> InputSink {
        unsafe { InputSink { ptr: wxInputSink_Create(input.ptr(), evtHandler.ptr(), bufferLen) } }
    }
}

pub trait TInputSink : TThread {
    fn getId(&self) -> c_int {
        unsafe { wxInputSink_GetId(self.ptr()) }
    }
    fn start(&self) {
        unsafe { wxInputSink_Start(self.ptr()) }
    }
}

pub struct InputSinkEvent { ptr: *mut c_void }
impl TInputSinkEvent for InputSinkEvent {}
impl TEvent for InputSinkEvent {}
impl TObject for InputSinkEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl InputSinkEvent {
    pub fn from(ptr: *mut c_void) -> InputSinkEvent { InputSinkEvent { ptr: ptr } }
    pub fn null() -> InputSinkEvent { InputSinkEvent::from(0 as *mut c_void) }
    
}

pub trait TInputSinkEvent : TEvent {
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

