use std::libc::*;
use native::*;

trait ELJApp {
    fn bell();
    fn newLogTarget() -> @ELJLog;
    fn dispatch();
    fn displaySize() -> @wxSize;
    fn enableTooltips(_enable: bool);
    fn enableTopLevelWindows(_enb: int);
    fn executeProcess(_cmd: @wxString, _snc: int, _prc: @wxProcess) -> int;
    fn exit();
    fn exitMainLoop();
    fn findWindowById(_id: int, _prt: @wxWindow);
    fn findWindowByLabel(_lbl: @wxString, _prt: @wxWindow) -> @wxWindow;
    fn findWindowByName(_lbl: @wxString, _prt: @wxWindow) -> @wxWindow;
    fn getApp() -> @wxApp;
    fn getAppName() -> @wxString;
    fn getClassName() -> @wxString;
    fn getExitOnFrameDelete() -> int;
    fn getOsDescription() -> @wxString;
    fn getOsVersion(_maj: *c_void, _min: *c_void) -> int;
    fn getTopWindow() -> @wxWindow;
    fn getUseBestVisual() -> int;
    fn getUserHome(_usr: *c_void) -> @wxString;
    fn getUserId() -> @wxString;
    fn getUserName() -> @wxString;
    fn getVendorName() -> @wxString;
    fn initAllImageHandlers();
    fn initialized() -> bool;
    fn mainLoop() -> int;
    fn mousePosition() -> @wxPoint;
    fn pending() -> int;
    fn safeYield(_win: @wxWindow) -> int;
    fn setAppName(name: @wxString);
    fn setClassName(name: @wxString);
    fn setExitOnFrameDelete(flag: int);
    fn setPrintMode(mode: int);
    fn setTooltipDelay(_ms: int);
    fn setTopWindow(_wnd: @wxWindow);
    fn setUseBestVisual(flag: int);
    fn setVendorName(name: @wxString);
    fn sleep(_scs: int);
    fn milliSleep(_mscs: int);
    fn yield_() -> int;
    fn isTerminating() -> c_int;
    fn initializeC(closure: @wxClosure, _argc: int, _argv: **wchar_t);
    fn getIdleInterval() -> int;
    fn setIdleInterval(interval: int);
}
trait ELJArtProv {
    fn new(_obj: *c_void, _clb: *c_void) -> @ELJArtProv;
    fn release(&self);
}
trait ELJClient {
    fn new(_eobj: *c_void, _cnct: *c_void) -> @ELJClient;
    fn delete(&self);
    fn makeConnection(&self, host: @wxString, server: @wxServer, topic: @wxString);
}
trait ELJCommand {
    fn canUndo(&self) -> bool;
    fn new(_und: int, _nme: @wxString, _obj: *c_void, _clb: *c_void) -> @ELJCommand;
    fn delete(&self);
    fn getName(&self) -> @wxString;
}
trait ELJConnection {
    fn advise(&self, item: @wxString, data: *c_void, size: int, format: int) -> int;
    fn compress(&self, on: int);
    fn new(_obj: *c_void, buffer: *c_void, size: int) -> @ELJConnection;
    fn newDefault(&self) -> @ELJConnection;
    fn delete(&self);
    fn disconnect(&self) -> bool;
    fn execute(&self, data: @wxString, size: int, format: int) -> bool;
    fn poke(&self, item: @wxString, data: *c_void, size: int, format: int) -> bool;
    fn request(&self, item: @wxString, size: @wxSize, format: int);
    fn setOnAdvise(&self, _fnc: *c_void);
    fn setOnDisconnect(&self, _fnc: *c_void);
    fn setOnExecute(&self, _fnc: *c_void);
    fn setOnPoke(&self, _fnc: *c_void);
    fn setOnRequest(&self, _fnc: *c_void);
    fn setOnStartAdvise(&self, _fnc: *c_void);
    fn setOnStopAdvise(&self, _fnc: *c_void);
    fn startAdvise(&self, item: @wxString) -> bool;
    fn stopAdvise(&self, item: @wxString) -> bool;
}
trait ELJDragDataObject {
    fn new(_obj: *c_void, _fmt: @wxString, _func1: *c_void, _func2: *c_void, _func3: *c_void) -> @ELJDragDataObject;
    fn delete(&self);
}
trait ELJDropTarget {
    fn new(_obj: *c_void) -> @ELJDropTarget;
    fn delete(&self);
    fn setOnData(&self, _func: *c_void);
    fn setOnDragOver(&self, _func: *c_void);
    fn setOnDrop(&self, _func: *c_void);
    fn setOnEnter(&self, _func: *c_void);
    fn setOnLeave(&self, _func: *c_void);
}
trait ELJFileDropTarget {
    fn new(_obj: *c_void, _func: *c_void) -> @ELJFileDropTarget;
    fn delete(&self);
    fn setOnData(&self, _func: *c_void);
    fn setOnDragOver(&self, _func: *c_void);
    fn setOnDrop(&self, _func: *c_void);
    fn setOnEnter(&self, _func: *c_void);
    fn setOnLeave(&self, _func: *c_void);
}
trait ELJGridTable {
    fn new(_obj: *c_void, _EifGetNumberRows: *c_void, _EifGetNumberCols: *c_void, _EifGetValue: *c_void, _EifSetValue: *c_void, _EifIsEmptyCell: *c_void, _EifClear: *c_void, _EifInsertRows: *c_void, _EifAppendRows: *c_void, _EifDeleteRows: *c_void, _EifInsertCols: *c_void, _EifAppendCols: *c_void, _EifDeleteCols: *c_void, _EifSetRowLabelValue: *c_void, _EifSetColLabelValue: *c_void, _EifGetRowLabelValue: *c_void, _EifGetColLabelValue: *c_void) -> @ELJGridTable;
    fn delete(&self);
    fn getView(&self) -> @wxView;
    fn sendTableMessage(&self, id: int, val1: int, val2: int);
}
trait ELJLocale {
}
trait ELJLog {
    fn addTraceMask(&self, str: *wchar_t);
    fn new(_obj: *c_void, _fnc: *c_void) -> @ELJLog;
    fn delete(&self);
    fn dontCreateOnDemand(&self);
    fn enableLogging(&self, doIt: bool) -> int;
    fn flush(&self);
    fn flushActive(&self);
    fn getActiveTarget();
    fn getTimestamp(&self);
    fn getTraceMask(&self) -> int;
    fn getVerbose(&self) -> int;
    fn hasPendingMessages(&self) -> bool;
    fn isAllowedTraceMask(&self, mask: @wxMask) -> bool;
    fn isEnabled(&self) -> bool;
    fn onLog(&self, level: int, szString: *c_void, t: int);
    fn removeTraceMask(&self, str: *wchar_t);
    fn resume(&self);
    fn setActiveTarget(&self);
    fn setTimestamp(&self, ts: *c_void);
    fn setTraceMask(&self, ulMask: int);
    fn setVerbose(&self, bVerbose: int);
    fn suspend(&self);
}
trait ELJMessageParameters {
}
trait ELJPlotCurve {
    fn new(_obj: *c_void, _str: *c_void, _end: *c_void, _y: *c_void, offsetY: int, startY: c_double, endY: c_double) -> @ELJPlotCurve;
    fn delete(&self);
    fn getEndY(&self) -> c_double;
    fn getOffsetY(&self) -> int;
    fn getStartY(&self) -> c_double;
    fn setEndY(&self, endY: c_double);
    fn setOffsetY(&self, offsetY: int);
    fn setPenNormal(&self, pen: @wxPen);
    fn setPenSelected(&self, pen: @wxPen);
    fn setStartY(&self, startY: c_double);
}
trait ELJPreviewControlBar {
    fn new(preview: *c_void, buttons: int, parent: @wxWindow, title: *c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @ELJPreviewControlBar;
}
trait ELJPreviewFrame {
    fn new(_obj: *c_void, _init: *c_void, _create_canvas: *c_void, _create_toolbar: *c_void, preview: *c_void, parent: @wxWindow, title: *c_void, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @ELJPreviewFrame;
    fn getControlBar(&self);
    fn getPreviewCanvas(&self) -> @wxPreviewCanvas;
    fn getPrintPreview(&self) -> @wxPrintPreview;
    fn initialize(&self);
    fn setControlBar(&self, obj: *c_void);
    fn setPreviewCanvas(&self, obj: @wxPreviewCanvas);
    fn setPrintPreview(&self, obj: @wxPrintPreview);
}
trait ELJServer {
    fn new(_eobj: *c_void, _cnct: *c_void) -> @ELJServer;
    fn delete(&self);
    fn initialize(&self, name: @wxString) -> int;
}
trait ELJTextDropTarget {
    fn new(_obj: *c_void, _func: *c_void) -> @ELJTextDropTarget;
    fn delete(&self);
    fn setOnData(&self, _func: *c_void);
    fn setOnDragOver(&self, _func: *c_void);
    fn setOnDrop(&self, _func: *c_void);
    fn setOnEnter(&self, _func: *c_void);
    fn setOnLeave(&self, _func: *c_void);
}
trait ELJTextValidator {
    fn new(_obj: *c_void, _fnc: *c_void, _txt: *wchar_t, _stl: int) -> @ELJTextValidator;
}
trait cbAntiflickerPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbAntiflickerPlugin;
    fn newDefault() -> @cbAntiflickerPlugin;
    fn delete(&self);
}
trait cbBarDragPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbBarDragPlugin;
    fn newDefault() -> @cbBarDragPlugin;
    fn delete(&self);
}
trait cbBarHintsPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbBarHintsPlugin;
    fn newDefault() -> @cbBarHintsPlugin;
    fn delete(&self);
    fn setGrooveCount(&self, nGrooves: int);
}
trait cbBarInfo {
    fn new() -> @cbBarInfo;
    fn delete(&self);
    fn isExpanded(&self) -> bool;
    fn isFixed(&self) -> bool;
}
trait cbBarSpy {
    fn new(pPanel: *c_void) -> @cbBarSpy;
    fn newDefault() -> @cbBarSpy;
    fn delete(&self);
    fn processEvent(&self, event: @wxEvent) -> int;
    fn setBarWindow(&self, pWnd: *c_void);
}
trait cbCloseBox {
    fn new() -> @cbCloseBox;
}
trait cbCollapseBox {
    fn new() -> @cbCollapseBox;
}
trait cbCommonPaneProperties {
    fn assign(&self, _other: *c_void);
    fn barCollapseIconsOn(&self) -> int;
    fn barDragHintsOn(&self) -> int;
    fn barFloatingOn(&self) -> int;
    fn colProportionsOn(&self) -> int;
    fn newDefault() -> @cbCommonPaneProperties;
    fn delete(&self);
    fn exactDockPredictionOn(&self) -> int;
    fn minCBarDim(&self, _w: *c_int, _h: *c_int);
    fn nonDestructFrictionOn(&self) -> int;
    fn outOfPaneDragOn(&self) -> int;
    fn realTimeUpdatesOn(&self) -> int;
    fn resizeHandleSize(&self) -> int;
    fn rowProportionsOn(&self) -> int;
    fn setBarCollapseIconsOn(&self, _val: int);
    fn setBarDragHintsOn(&self, _val: int);
    fn setBarFloatingOn(&self, _val: int);
    fn setColProportionsOn(&self, _val: int);
    fn setExactDockPredictionOn(&self, _val: int);
    fn setMinCBarDim(&self, _w: c_int, _h: c_int);
    fn setNonDestructFrictionOn(&self, _val: int);
    fn setOutOfPaneDragOn(&self, _val: int);
    fn setRealTimeUpdatesOn(&self, _val: int);
    fn setResizeHandleSize(&self, _val: int);
    fn setRowProportionsOn(&self, _val: int);
    fn setShow3DPaneBorderOn(&self, _val: int);
    fn show3DPaneBorderOn(&self) -> int;
}
trait cbCustomizeBarEvent {
    fn bar(&self);
    fn clickPos(&self, _x: *c_int, _y: *c_int);
}
trait cbCustomizeLayoutEvent {
    fn clickPos(&self, _x: *c_int, _y: *c_int);
}
trait cbDimHandlerBase {
}
trait cbDimInfo {
    fn assign(&self, other: *c_void);
    fn new(x: c_int, y: c_int, isFixed: bool, gap: int, pDimHandler: *c_void) -> @cbDimInfo;
    fn newDefault() -> @cbDimInfo;
    fn newWithHandler(&self, isFixed: bool);
    fn newWithInfo(dh_x: int, dh_y: int, dv_x: int, dv_y: int, f_x: int, f_y: int, isFixed: bool, horizGap: int, vertGap: int, pDimHandler: *c_void);
    fn delete(&self);
    fn getDimHandler(&self);
}
trait cbDockBox {
    fn new() -> @cbDockBox;
}
trait cbDockPane {
    fn barPresent(&self, pBar: *c_void) -> int;
    fn new(alignment: int, pPanel: *c_void) -> @cbDockPane;
    fn newDefault() -> @cbDockPane;
    fn delete(&self);
    fn getAlignment(&self) -> int;
    fn getBarInfoByWindow(&self, pBarWnd: *c_void);
    fn getBarResizeRange(&self, pBar: *c_void, from: *c_void, till: *c_void, forLeftHandle: int);
    fn getDockingState(&self) -> int;
    fn getFirstRow(&self);
    fn getPaneHeight(&self) -> int;
    fn getRealRect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn getRowList(&self, _ref: *c_void) -> int;
    fn getRowResizeRange(&self, pRow: *c_void, from: *c_void, till: *c_void, forUpperHandle: int);
    fn hitTestPaneItems(&self, x: c_int, y: c_int, ppRow: *c_void, ppBar: *c_void) -> int;
    fn insertBarByCoord(&self, pBar: *c_void, x: c_int, y: c_int, w: c_int, h: c_int);
    fn insertBarByInfo(&self, pBarInfo: *c_void);
    fn insertBarToRow(&self, pBar: *c_void, pIntoRow: *c_void);
    fn insertRow(&self, pRow: *c_void, pBeforeRow: *c_void);
    fn isHorizontal(&self) -> bool;
    fn matchesMask(&self, paneMask: int) -> int;
    fn removeBar(&self, pBar: *c_void);
    fn removeRow(&self, pRow: *c_void);
    fn setBoundsInParent(&self, x: c_int, y: c_int, w: c_int, h: c_int);
    fn setMargins(&self, top: int, bottom: int, left: int, right: int);
    fn setPaneWidth(&self, width: int);
}
trait cbDrawBarDecorEvent {
    fn bar(&self);
    fn boundsInParent(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn dc(&self);
}
trait cbDrawBarHandlesEvent {
    fn bar(&self);
    fn dc(&self);
}
trait cbDrawHintRectEvent {
    fn eraseRect(&self) -> int;
    fn isInClient(&self) -> bool;
    fn lastTime(&self) -> int;
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
}
trait cbDrawPaneBkGroundEvent {
    fn dc(&self);
}
trait cbDrawPaneDecorEvent {
    fn dc(&self);
}
trait cbDrawRowBkGroundEvent {
    fn dc(&self);
    fn row(&self);
}
trait cbDrawRowDecorEvent {
    fn dc(&self);
    fn row(&self);
}
trait cbDrawRowHandlesEvent {
    fn dc(&self);
    fn row(&self);
}
trait cbDynToolBarDimHandler {
    fn new() -> @cbDynToolBarDimHandler;
    fn delete(&self);
}
trait cbFinishDrawInAreaEvent {
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
}
trait cbFloatedBarWindow {
    fn new(_obj: *c_void) -> @cbFloatedBarWindow;
    fn getBar(&self);
    fn positionFloatedWnd(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int);
    fn setBar(&self, _bar: *c_void);
    fn setLayout(&self, _layout: *c_void);
}
trait cbGCUpdatesMgr {
    fn new(pPanel: *c_void) -> @cbGCUpdatesMgr;
    fn newDefault() -> @cbGCUpdatesMgr;
    fn delete(&self);
    fn updateNow(&self);
}
trait cbHintAnimationPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbHintAnimationPlugin;
    fn newDefault() -> @cbHintAnimationPlugin;
    fn delete(&self);
}
trait cbInsertBarEvent {
    fn bar(&self);
    fn row(&self);
}
trait cbLayoutRowEvent {
    fn row(&self);
}
trait cbLeftDClickEvent {
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbLeftDownEvent {
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbLeftUpEvent {
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbMiniButton {
    fn new() -> @cbMiniButton;
    fn delete(&self);
    fn dim(&self, _w: *c_int, _h: *c_int);
    fn dragStarted(&self) -> int;
    fn enable(&self, enable: bool);
    fn enabled(&self) -> int;
    fn hitTest(&self, x: c_int, y: c_int) -> int;
    fn isPressed(&self) -> bool;
    fn layout(&self);
    fn pane(&self);
    fn plugin(&self);
    fn pos(&self, _x: *c_int, _y: *c_int);
    fn pressed(&self) -> int;
    fn refresh(&self);
    fn reset(&self);
    fn setPos(&self, x: c_int, y: c_int);
    fn visible(&self) -> int;
    fn wasClicked(&self) -> int;
    fn wnd(&self);
}
trait cbMotionEvent {
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbPaneDrawPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbPaneDrawPlugin;
    fn newDefault() -> @cbPaneDrawPlugin;
    fn delete(&self);
}
trait cbPluginBase {
    fn delete(&self);
    fn getPaneMask(&self) -> int;
    fn isReady(&self) -> bool;
    fn plugin(_swt: int);
    fn processEvent(&self, event: @wxEvent) -> int;
}
trait cbPluginEvent {
    fn pane(&self);
}
trait cbRemoveBarEvent {
    fn bar(&self);
}
trait cbResizeBarEvent {
    fn bar(&self);
    fn row(&self);
}
trait cbResizeRowEvent {
    fn forUpperHandle(&self) -> int;
    fn handleOfs(&self) -> int;
    fn row(&self);
}
trait cbRightDownEvent {
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbRightUpEvent {
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbRowDragPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbRowDragPlugin;
    fn newDefault() -> @cbRowDragPlugin;
    fn delete(&self);
}
trait cbRowInfo {
    fn new() -> @cbRowInfo;
    fn delete(&self);
    fn getFirstBar(&self);
}
trait cbRowLayoutPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbRowLayoutPlugin;
    fn newDefault() -> @cbRowLayoutPlugin;
    fn delete(&self);
}
trait cbSimpleCustomizationPlugin {
    fn new(pPanel: *c_void, paneMask: int) -> @cbSimpleCustomizationPlugin;
    fn newDefault() -> @cbSimpleCustomizationPlugin;
    fn delete(&self);
}
trait cbSimpleUpdatesMgr {
}
trait cbSizeBarWndEvent {
    fn bar(&self);
    fn boundsInParent(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
}
trait cbStartBarDraggingEvent {
    fn bar(&self);
    fn pos(&self, _x: *c_int, _y: *c_int);
}
trait cbStartDrawInAreaEvent {
    fn area(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
}
trait cbUpdatesManagerBase {
}
trait wxAcceleratorEntry {
    fn new(flags: int, keyCode: int, cmd: int) -> @wxAcceleratorEntry;
    fn delete(&self);
    fn getCommand(&self) -> int;
    fn getFlags(&self) -> int;
    fn getKeyCode(&self) -> int;
    fn set(&self, flags: int, keyCode: int, cmd: int);
}
trait wxAcceleratorTable {
    fn new(n: int, entries: *c_void) -> @wxAcceleratorTable;
    fn delete(&self);
}
trait wxActivateEvent {
    fn copyObject(&self, obj: *c_void);
    fn getActive(&self) -> bool;
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
    fn new(window: @wxWindow) -> @wxAutoBufferedPaintDC;
    fn delete(&self);
}
trait wxAutomationObject {
}
trait wxBitmap {
    fn addHandler(handler: @wxEvtHandler);
    fn cleanUpHandlers();
    fn new(_data: *c_void, _type: int, _width: c_int, _height: c_int, _depth: int) -> @wxBitmap;
    fn newDefault() -> @wxBitmap;
    fn newEmpty(_width: c_int, _height: c_int, _depth: int) -> @wxBitmap;
    fn newFromXPM(&self) -> @wxBitmap;
    fn newLoad(name: @wxString, type_: int) -> @wxBitmap;
    fn delete(&self);
    fn findHandlerByExtension(&self, type_: int);
    fn findHandlerByName(name: @wxString);
    fn findHandlerByType(type_: int);
    fn getDepth(&self) -> int;
    fn getHeight(&self) -> int;
    fn getMask(&self) -> @wxMask;
    fn getSubBitmap(&self, x: c_int, y: c_int, w: c_int, h: c_int, @wxBitmap);
    fn getWidth(&self) -> int;
    fn initStandardHandlers();
    fn insertHandler(handler: @wxEvtHandler);
    fn loadFile(&self, name: @wxString, type_: int) -> int;
    fn isOk(&self) -> bool;
    fn removeHandler(name: @wxString) -> bool;
    fn saveFile(&self, name: @wxString, type_: int, cmap: @wxPalette) -> int;
    fn setDepth(&self, d: int);
    fn setHeight(&self, h: int);
    fn setMask(&self, mask: @wxMask);
    fn setWidth(&self, w: int);
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
    fn newFromImage(image: @wxImage, depth: int) -> @wxBitmap;
}
trait wxBitmapButton {
    fn new(_prt: @wxWindow, _id: int, _bmp: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxBitmapButton;
    fn getBitmapDisabled(&self, @wxBitmap);
    fn getBitmapFocus(&self, @wxBitmap);
    fn getBitmapLabel(&self, @wxBitmap);
    fn getBitmapSelected(&self, @wxBitmap);
    fn getMarginX(&self) -> int;
    fn getMarginY(&self) -> int;
    fn setBitmapDisabled(&self, disabled: @wxBitmap);
    fn setBitmapFocus(&self, focus: @wxBitmap);
    fn setBitmapLabel(&self, bitmap: @wxBitmap);
    fn setBitmapSelected(&self, sel: @wxBitmap);
    fn setMargins(&self, x: c_int, y: c_int);
}
trait wxBitmapToggleButton {
    fn new(parent: @wxWindow, id: int, _bmp: @wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxBitmapToggleButton;
    fn enable(&self, enable: bool) -> bool;
    fn getValue(&self) -> bool;
    fn setValue(&self, state: bool);
    fn setBitmapLabel(&self, _bmp: @wxBitmap);
}
trait wxBitmapDataObject {
}
trait wxBitmapHandler {
}
trait wxBoxSizer {
    fn calcMin(&self) -> @wxSize;
    fn new(orient: int) -> @wxBoxSizer;
    fn getOrientation(&self) -> int;
    fn recalcSizes(&self);
}
trait wxBrush {
    fn assign(&self, brush: @wxBrush);
    fn newDefault() -> @wxBrush;
    fn newFromBitmap(bitmap: @wxBitmap) -> @wxBrush;
    fn newFromColour(col: @wxColour, style: int) -> @wxBrush;
    fn newFromStock(id: int) -> @wxBrush;
    fn delete(&self);
    fn getColour(&self, @wxColour);
    fn getStipple(&self, @wxBitmap);
    fn getStyle(&self) -> int;
    fn isEqual(&self, brush: @wxBrush) -> bool;
    fn isOk(&self) -> bool;
    fn setColour(&self, col: @wxColour);
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t);
    fn setStipple(&self, stipple: @wxBitmap);
    fn setStyle(&self, style: int);
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
}
trait wxBrushList {
}
trait wxBufferedDC {
    fn newByDCAndSize(dc: @wxDC, width: c_int, hight: c_int, style: int) -> @wxBufferedDC;
    fn newByDCAndBitmap(dc: @wxDC, bitmap: @wxBitmap, style: int) -> @wxBufferedDC;
    fn delete(&self);
}
trait wxBufferedPaintDC {
    fn new(window: @wxWindow, style: int) -> @wxBufferedPaintDC;
    fn newWithBitmap(window: @wxWindow, bitmap: @wxBitmap, style: int) -> @wxBufferedPaintDC;
    fn delete(&self);
}
trait wxBufferedInputStream {
}
trait wxBufferedOutputStream {
}
trait wxBusyCursor {
    fn new() -> @wxBusyCursor;
    fn newWithCursor(&self);
    fn delete(&self);
}
trait wxBusyInfo {
    fn new(_txt: @wxString) -> @wxBusyInfo;
    fn delete(&self);
}
trait wxButton {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxButton;
    fn setBackgroundColour(&self, colour: @wxColour) -> int;
    fn setDefault(&self);
}
trait wxCSConv {
}
trait wxCalculateLayoutEvent {
    fn new(id: int) -> @wxCalculateLayoutEvent;
    fn getFlags(&self) -> int;
    fn getRect(&self) -> @wxRect;
    fn setFlags(&self, flags: int);
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int);
}
trait wxCalendarCtrl {
    fn new(_prt: @wxWindow, _id: int, _dat: @wxDateTime, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxCalendarCtrl;
    fn enableHolidayDisplay(&self, display: int);
    fn enableMonthChange(&self, enable: bool);
    fn getAttr(&self, day: int);
    fn getDate(&self, date: *c_void);
    fn getHeaderColourBg(&self, @wxColour);
    fn getHeaderColourFg(&self, @wxColour);
    fn getHighlightColourBg(&self, @wxColour);
    fn getHighlightColourFg(&self, @wxColour);
    fn getHolidayColourBg(&self, @wxColour);
    fn getHolidayColourFg(&self, @wxColour);
    fn hitTest(&self, x: c_int, y: c_int, date: *c_void, wd: *c_void) -> int;
    fn resetAttr(&self, day: int);
    fn setAttr(&self, day: int, attr: *c_void);
    fn setDate(&self, date: *c_void);
    fn setHeaderColours(&self, colFg: *c_void, colBg: *c_void);
    fn setHighlightColours(&self, colFg: *c_void, colBg: *c_void);
    fn setHoliday(&self, day: int);
    fn setHolidayColours(&self, colFg: *c_void, colBg: *c_void);
}
trait wxCalendarDateAttr {
    fn new(_ctxt: *c_void, _cbck: *c_void, _cbrd: *c_void, _fnt: *c_void, _brd: int) -> @wxCalendarDateAttr;
    fn newDefault() -> @wxCalendarDateAttr;
    fn delete(&self);
    fn getBackgroundColour(&self, @wxColour);
    fn getBorder(&self) -> int;
    fn getBorderColour(&self, @wxColour);
    fn getFont(&self, @wxFont);
    fn getTextColour(&self, @wxColour);
    fn hasBackgroundColour(&self) -> bool;
    fn hasBorder(&self) -> bool;
    fn hasBorderColour(&self) -> bool;
    fn hasFont(&self) -> bool;
    fn hasTextColour(&self) -> bool;
    fn isHoliday(&self) -> bool;
    fn setBackgroundColour(&self, col: @wxColour);
    fn setBorder(&self, border: int);
    fn setBorderColour(&self, col: @wxColour);
    fn setFont(&self, font: @wxFont);
    fn setHoliday(&self, holiday: int);
    fn setTextColour(&self, col: @wxColour);
}
trait wxCalendarEvent {
    fn getDate(&self, _dte: *c_void);
    fn getWeekDay(&self) -> int;
}
trait wxCaret {
    fn new(_wnd: @wxWindow, _wth: int, _hgt: int) -> @wxCaret;
    fn getBlinkTime() -> int;
    fn getPosition(&self) -> @wxPoint;
    fn getSize(&self) -> @wxSize;
    fn getWindow(&self) -> @wxWindow;
    fn hide(&self);
    fn isOk(&self) -> bool;
    fn isVisible(&self) -> bool;
    fn move(&self, x: c_int, y: c_int);
    fn setBlinkTime(milliseconds: int);
    fn setSize(&self, width: c_int, height: c_int);
    fn show(&self);
}
trait wxCheckBox {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxCheckBox;
    fn delete(&self);
    fn getValue(&self) -> bool;
    fn setValue(&self, value: c_int);
}
trait wxCheckListBox {
    fn check(&self, item: int, check: bool);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxCheckListBox;
    fn delete(&self);
    fn isChecked(&self, item: int) -> bool;
}
trait wxChoice {
    fn append(&self, item: @wxString);
    fn clear(&self);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxChoice;
    fn delete(&self, n: int);
    fn findString(&self, s: @wxString) -> int;
    fn getCount(&self) -> int;
    fn getSelection(&self) -> int;
    fn getString(&self, n: int) -> @wxString;
    fn setSelection(&self, n: int);
    fn setString(&self, n: int, s: @wxString);
}
trait wxClassInfo {
    fn newClassByName(&self);
    fn getClassName(&self);
    fn isKindOf(&self, _name: @wxString) -> bool;
    fn findClass(_txt: @wxString) -> @wxClassInfo;
    fn getBaseClassName1(&self) -> @wxString;
    fn getBaseClassName2(&self) -> @wxString;
    fn getClassNameEx(&self) -> @wxString;
    fn getSize(&self) -> int;
    fn isKindOfEx(&self, classInfo: @wxClassInfo) -> bool;
}
trait wxClient {
}
trait wxClientBase {
}
trait wxClientDC {
    fn new(win: @wxWindow) -> @wxClientDC;
    fn delete(&self);
}
trait wxClientData {
}
trait wxClientDataContainer {
}
trait wxClipboard {
    fn addData(&self, data: @wxDataObject) -> bool;
    fn clear(&self);
    fn close(&self);
    fn new() -> @wxClipboard;
    fn flush(&self) -> bool;
    fn getData(&self, data: @wxDataObject) -> bool;
    fn isOpened(&self) -> bool;
    fn isSupported(&self, format: @wxDataFormat) -> bool;
    fn open(&self) -> bool;
    fn setData(&self, data: @wxDataObject) -> bool;
    fn usePrimarySelection(&self, primary: bool);
}
trait wxCloseEvent {
    fn canVeto(&self) -> bool;
    fn copyObject(&self, obj: @wxObject);
    fn getLoggingOff(&self) -> bool;
    fn getVeto(&self) -> bool;
    fn setCanVeto(&self, canVeto: bool);
    fn setLoggingOff(&self, logOff: bool);
    fn veto(&self, veto: bool);
}
trait wxClosure {
    fn new(_fun_CEvent: *c_void, _data: *c_void) -> @wxClosure;
    fn getData(&self);
}
trait wxColour {
    fn alpha(&self) -> uint8_t;
    fn assign(&self, other: *c_void);
    fn blue(&self) -> uint8_t;
    fn copy(&self, _other: *c_void);
    fn newByName(_name: @wxString) -> @wxColour;
    fn newEmpty() -> @wxColour;
    fn newFromStock(id: int) -> @wxColour;
    fn newRGB(_red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t) -> @wxColour;
    fn delete(&self);
    fn green(&self) -> uint8_t;
    fn isOk(&self) -> bool;
    fn red(&self) -> uint8_t;
    fn set(&self, _red: uint8_t, _green: uint8_t, _blue: uint8_t, _alpha: uint8_t);
    fn setByName(&self, _name: @wxString);
    fn validName(_name: *wchar_t) -> bool;
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
    fn newFromInt(rgb: int) -> @wxColour;
    fn getInt(&self) -> int;
    fn newFromUnsignedInt(rgba: uint32_t) -> @wxColour;
    fn getUnsignedInt(&self) -> uint32_t;
}
trait wxColourData {
    fn new() -> @wxColourData;
    fn delete(&self);
    fn getChooseFull(&self) -> bool;
    fn getColour(&self, @wxColour);
    fn getCustomColour(&self, i: int, @wxColour);
    fn setChooseFull(&self, flag: bool);
    fn setColour(&self, colour: @wxColour);
    fn setCustomColour(&self, i: int, colour: @wxColour);
}
trait wxColourDatabase {
}
trait wxColourDialog {
    fn new(_prt: @wxWindow, col: @wxColourData) -> @wxColourDialog;
    fn getColourData(&self, @wxColourData);
}
trait wxComboBox {
    fn append(&self, item: @wxString);
    fn appendData(&self, item: @wxString, d: *c_void);
    fn clear(&self);
    fn copy(&self);
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxComboBox;
    fn cut(&self);
    fn delete(&self, n: int);
    fn findString(&self, s: @wxString) -> int;
    fn getClientData(&self, n: int) -> @wxClientData;
    fn getCount(&self) -> int;
    fn getInsertionPoint(&self) -> int;
    fn getLastPosition(&self) -> int;
    fn getSelection(&self) -> int;
    fn getString(&self, n: int) -> @wxString;
    fn getStringSelection(&self) -> @wxString;
    fn getValue(&self) -> @wxString;
    fn paste(&self);
    fn remove(&self, from: int, to: int);
    fn replace(&self, from: int, to: int, value: @wxString);
    fn setClientData(&self, n: int, clientData: @wxClientData);
    fn setEditable(&self, editable: bool);
    fn setInsertionPoint(&self, pos: int);
    fn setInsertionPointEnd(&self);
    fn setSelection(&self, n: int);
    fn setTextSelection(&self, from: int, to: int);
}
trait wxCommand {
}
trait wxCommandEvent {
    fn copyObject(&self, object_dest: *c_void);
    fn new(_typ: int, _id: int) -> @wxCommandEvent;
    fn delete(&self);
    fn getClientData(&self) -> @wxClientData;
    fn getClientObject(&self) -> @wxClientData;
    fn getExtraLong(&self) -> c_long;
    fn getInt(&self) -> c_long;
    fn getSelection(&self) -> int;
    fn getString(&self) -> @wxString;
    fn isChecked(&self) -> bool;
    fn isSelection(&self) -> bool;
    fn setClientData(&self, clientData: @wxClientData);
    fn setClientObject(&self, clientObject: @wxClientData);
    fn setExtraLong(&self, extraLong: c_long);
    fn setInt(&self, i: int);
    fn setString(&self, s: @wxString);
}
trait wxCommandLineParser {
}
trait wxCommandProcessor {
    fn canRedo(&self) -> bool;
    fn canUndo(&self) -> bool;
    fn clearCommands(&self);
    fn delete(&self);
    fn getCommands(&self, _ref: *c_void) -> int;
    fn getEditMenu(&self);
    fn getMaxCommands(&self) -> int;
    fn initialize(&self);
    fn redo(&self) -> int;
    fn setEditMenu(&self, menu: @wxMenu);
    fn setMenuStrings(&self);
    fn submit(&self, command: @wxCommand, storeIt: int) -> int;
    fn undo(&self) -> int;
    fn wxCommandProcessor(maxCommands: int);
}
trait wxCondition {
    fn broadcast(&self);
    fn new(_mut: *c_void) -> @wxCondition;
    fn delete(&self);
    fn signal(&self);
    fn wait(&self);
    fn waitFor(&self, sec: int, nsec: int) -> int;
}
trait wxConfigBase {
    fn new() -> @wxConfigBase;
    fn delete(&self);
    fn deleteAll(&self) -> bool;
    fn deleteEntry(&self, key: @wxString, bDeleteGroupIfEmpty: bool) -> bool;
    fn deleteGroup(&self, key: @wxString) -> bool;
    fn exists(&self, strName: @wxString) -> bool;
    fn expandEnvVars(&self, str: @wxString) -> @wxString;
    fn flush(&self, bCurrentOnly: bool) -> bool;
    fn getAppName(&self) -> @wxString;
    fn getEntryType(&self, name: @wxString) -> int;
    fn getFirstEntry(&self, lIndex: *c_void) -> @wxString;
    fn getFirstGroup(&self, lIndex: *c_void) -> @wxString;
    fn getNextEntry(&self, lIndex: *c_void) -> @wxString;
    fn getNextGroup(&self, lIndex: *c_void) -> @wxString;
    fn getNumberOfEntries(&self, bRecursive: bool) -> int;
    fn getNumberOfGroups(&self, bRecursive: bool) -> int;
    fn getPath(&self) -> @wxString;
    fn getStyle(&self) -> int;
    fn getVendorName(&self) -> @wxString;
    fn hasEntry(&self, strName: @wxString) -> bool;
    fn hasGroup(&self, strName: @wxString) -> bool;
    fn isExpandingEnvVars(&self) -> bool;
    fn isRecordingDefaults(&self) -> bool;
    fn readBool(&self, key: @wxString, defVal: bool) -> bool;
    fn readDouble(&self, key: @wxString, defVal: c_double) -> c_double;
    fn readInteger(&self, key: @wxString, defVal: int) -> int;
    fn readString(&self, key: @wxString, defVal: @wxString) -> @wxString;
    fn renameEntry(&self, oldName: @wxString, newName: @wxString) -> bool;
    fn renameGroup(&self, oldName: @wxString, newName: @wxString) -> bool;
    fn setAppName(&self, appName: @wxString);
    fn setExpandEnvVars(&self, bDoIt: bool);
    fn setPath(&self, strPath: @wxString);
    fn setRecordDefaults(&self, bDoIt: bool);
    fn setStyle(&self, style: int);
    fn setVendorName(&self, vendorName: @wxString);
    fn writeBool(&self, key: @wxString, value: bool) -> bool;
    fn writeDouble(&self, key: @wxString, value: c_double) -> bool;
    fn writeInteger(&self, key: @wxString, value: int) -> bool;
    fn writeLong(&self, key: @wxString, value: c_long) -> bool;
    fn writeString(&self, key: @wxString, value: @wxString) -> bool;
    fn get() -> @wxConfigBase;
    fn set(self_: @wxConfigBase);
}
trait wxConnection {
}
trait wxConnectionBase {
}
trait wxContextHelp {
    fn beginContextHelp(&self, win: @wxWindow) -> bool;
    fn new(win: @wxWindow, beginHelp: bool) -> @wxContextHelp;
    fn delete(&self);
    fn endContextHelp(&self) -> bool;
}
trait wxContextHelpButton {
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: c_long) -> @wxContextHelpButton;
}
trait wxControl {
    fn command(&self, event: @wxEvent);
    fn getLabel(&self) -> @wxString;
    fn setLabel(&self, text: @wxString);
}
trait wxCountingOutputStream {
}
trait wxCriticalSection {
    fn new() -> @wxCriticalSection;
    fn delete(&self);
    fn enter(&self);
    fn leave(&self);
}
trait wxCriticalSectionLocker {
}
trait wxCursor {
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
    fn delete(&self);
}
trait wxCustomDataObject {
}
trait wxDC {
    fn blit(&self, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: @wxDC, xsrc: c_int, ysrc: c_int, rop: int, useMask: bool) -> bool;
    fn calcBoundingBox(&self, x: c_int, y: c_int);
    fn canDrawBitmap(&self) -> bool;
    fn canGetTextExtent(&self) -> bool;
    fn clear(&self);
    fn computeScaleAndOrigin(&self);
    fn crossHair(&self, x: c_int, y: c_int);
    fn delete(&self);
    fn destroyClippingRegion(&self);
    fn deviceToLogicalX(&self, x: int) -> int;
    fn deviceToLogicalXRel(&self, x: int) -> int;
    fn deviceToLogicalY(&self, y: int) -> int;
    fn deviceToLogicalYRel(&self, y: int) -> int;
    fn drawArc(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, xc: c_int, yc: c_int);
    fn drawBitmap(&self, bmp: @wxBitmap, x: c_int, y: c_int, useMask: bool);
    fn drawCheckMark(&self, x: c_int, y: c_int, width: c_int, height: c_int);
    fn drawCircle(&self, x: c_int, y: c_int, radius: int);
    fn drawEllipse(&self, x: c_int, y: c_int, width: c_int, height: c_int);
    fn drawEllipticArc(&self, x: c_int, y: c_int, w: c_int, h: c_int, sa: c_double, ea: c_double);
    fn drawIcon(&self, icon: @wxIcon, x: c_int, y: c_int);
    fn drawLabel(&self, str: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, align: int, indexAccel: int);
    fn drawLabelBitmap(&self, str: @wxString, bmp: @wxBitmap, x: c_int, y: c_int, w: c_int, h: c_int, align: int, indexAccel: int) -> @wxRect;
    fn drawLine(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    fn drawLines(&self, n: int, x: *c_void, y: *c_void, xoffset: c_int, yoffset: c_int);
    fn drawPoint(&self, x: c_int, y: c_int);
    fn drawPolygon(&self, n: int, x: *c_void, y: *c_void, xoffset: c_int, yoffset: c_int, fillStyle: int);
    fn drawPolyPolygon(&self, n: int, count: *c_void, x: *c_void, y: *c_void, xoffset: c_int, yoffset: c_int, fillStyle: int);
    fn drawRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int);
    fn drawRotatedText(&self, text: @wxString, x: c_int, y: c_int, angle: c_double);
    fn drawRoundedRectangle(&self, x: c_int, y: c_int, width: c_int, height: c_int, radius: c_double);
    fn drawText(&self, text: @wxString, x: c_int, y: c_int);
    fn endDoc(&self);
    fn endPage(&self);
    fn floodFill(&self, x: c_int, y: c_int, col: @wxColour, style: int);
    fn getBackground(&self, @wxBrush);
    fn getBackgroundMode(&self) -> int;
    fn getBrush(&self, @wxBrush);
    fn getCharHeight(&self) -> int;
    fn getCharWidth(&self) -> int;
    fn getClippingBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn getDepth(&self) -> int;
    fn getDeviceOrigin(&self, _x: *c_int, _y: *c_int);
    fn getFont(&self, @wxFont);
    fn getLogicalFunction(&self) -> int;
    fn getLogicalOrigin(&self, _x: *c_int, _y: *c_int);
    fn getLogicalScale(&self, _x: *c_double, _y: *c_double);
    fn getMapMode(&self) -> int;
    fn getPPI(&self) -> @wxSize;
    fn getPen(&self, @wxPen);
    fn getPixel(&self, x: c_int, y: c_int, col: @wxColour) -> bool;
    fn getSize(&self) -> @wxSize;
    fn getSizeMM(&self) -> @wxSize;
    fn getTextBackground(&self, @wxColour);
    fn getTextExtent(&self, string: @wxString, w: *c_void, h: *c_void, descent: *c_void, externalLeading: *c_void, theFont: @wxFont);
    fn getMultiLineTextExtent(&self, string: @wxString, w: *c_void, h: *c_void, heightLine: *c_void, theFont: @wxFont);
    fn getTextForeground(&self, @wxColour);
    fn getUserScale(&self, x: *c_double, y: *c_double);
    fn logicalToDeviceX(&self, x: int) -> int;
    fn logicalToDeviceXRel(&self, x: int) -> int;
    fn logicalToDeviceY(&self, y: int) -> int;
    fn logicalToDeviceYRel(&self, y: int) -> int;
    fn maxX(&self) -> int;
    fn maxY(&self) -> int;
    fn minX(&self) -> int;
    fn minY(&self) -> int;
    fn isOk(&self) -> bool;
    fn resetBoundingBox(&self);
    fn setAxisOrientation(&self, xLeftRight: bool, yBottomUp: bool);
    fn setBackground(&self, brush: @wxBrush);
    fn setBackgroundMode(&self, mode: int);
    fn setBrush(&self, brush: @wxBrush);
    fn setClippingRegion(&self, x: c_int, y: c_int, width: c_int, height: c_int);
    fn setClippingRegionFromRegion(&self, region: @wxRegion);
    fn setDeviceOrigin(&self, x: c_int, y: c_int);
    fn setFont(&self, font: @wxFont);
    fn setLogicalFunction(&self, function: int);
    fn setLogicalOrigin(&self, x: c_int, y: c_int);
    fn setLogicalScale(&self, x: c_double, y: c_double);
    fn setMapMode(&self, mode: int);
    fn setPalette(&self, palette: @wxPalette);
    fn setPen(&self, pen: @wxPen);
    fn setTextBackground(&self, colour: @wxColour);
    fn setTextForeground(&self, colour: @wxColour);
    fn setUserScale(&self, x: c_double, y: c_double);
    fn startDoc(&self, msg: @wxString) -> bool;
    fn startPage(&self);
    fn getUserScaleX(&self) -> c_double;
    fn getUserScaleY(&self) -> c_double;
    fn getPixel2(&self, x: c_int, y: c_int, @wxColour);
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
    fn newFromId(name: @wxString) -> @wxDataFormat;
    fn newFromType(typ: int) -> @wxDataFormat;
    fn delete(&self);
    fn getId(&self) -> @wxString;
    fn getType(&self) -> int;
    fn isEqual(&self, other: *c_void) -> bool;
    fn setId(&self, id: *c_void);
    fn setType(&self, typ: int);
}
trait wxDataInputStream {
}
trait wxDataObject {
}
trait wxDataObjectComposite {
    fn add(&self, _dat: *c_void, _preferred: int);
    fn new() -> @wxDataObjectComposite;
    fn delete(&self);
}
trait wxDataObjectSimple {
}
trait wxDataOutputStream {
}
trait wxDatabase {
}
trait wxDateTime {
    fn addDate(&self, diff: *c_void, @wxDateTime);
    fn addDateValues(&self, _yrs: int, _mnt: int, _wek: int, _day: int);
    fn addTime(&self, diff: *c_void, @wxDateTime);
    fn addTimeValues(&self, _hrs: int, _min: int, _sec: int, _mls: int);
    fn convertYearToBC(year: int) -> int;
    fn new() -> @wxDateTime;
    fn format(&self, format: *c_void, tz: int) -> @wxString;
    fn formatDate(&self) -> @wxString;
    fn formatISODate(&self) -> @wxString;
    fn formatISOTime(&self) -> @wxString;
    fn formatTime(&self) -> @wxString;
    fn getAmString() -> @wxString;
    fn getBeginDST(year: int, country: int, dt: @wxDateTime);
    fn getCentury(year: int) -> int;
    fn getCountry() -> int;
    fn getCurrentMonth(cal: int) -> int;
    fn getCurrentYear(cal: int) -> int;
    fn getDay(&self, tz: int) -> int;
    fn getDayOfYear(&self, tz: int) -> int;
    fn getEndDST(year: int, country: int, dt: @wxDateTime);
    fn getHour(&self, tz: int) -> int;
    fn getLastMonthDay(&self, month: int, year: int, @wxDateTime);
    fn getLastWeekDay(&self, weekday: int, month: int, year: int, @wxDateTime);
    fn getMillisecond(&self, tz: int) -> int;
    fn getMinute(&self, tz: int) -> int;
    fn getMonth(&self, tz: int) -> int;
    fn getMonthName(month: int, flags: int) -> @wxString;
    fn getNextWeekDay(&self, weekday: int, @wxDateTime);
    fn getNumberOfDays(year: int, cal: int) -> int;
    fn getNumberOfDaysMonth(month: int, year: int, cal: int) -> int;
    fn getPmString() -> @wxString;
    fn getPrevWeekDay(&self, weekday: int, @wxDateTime);
    fn getSecond(&self, tz: int) -> int;
    fn getTicks(&self) -> time_t;
    fn getTimeNow() -> int;
    fn getValue(&self, hi_long: *c_void, lo_long: *c_void);
    fn getWeekDay(&self, weekday: int, n: int, month: int, year: int, @wxDateTime);
    fn getWeekDayInSameWeek(&self, weekday: int, @wxDateTime);
    fn getWeekDayName(weekday: int, flags: int) -> @wxString;
    fn getWeekDayTZ(&self, tz: int) -> int;
    fn getWeekOfMonth(&self, flags: int, tz: int) -> int;
    fn getWeekOfYear(&self, flags: int, tz: int) -> int;
    fn getYear(&self, tz: int) -> int;
    fn isBetween(&self, t1: @wxDateTime, t2: @wxDateTime) -> bool;
    fn isDST(&self, country: int) -> bool;
    fn isDSTApplicable(year: int, country: int) -> bool;
    fn isEarlierThan(&self, datetime: *c_void) -> bool;
    fn isEqualTo(&self, datetime: *c_void) -> bool;
    fn isEqualUpTo(&self, dt: @wxDateTime, ts: *c_void) -> bool;
    fn isGregorianDate(&self, country: int) -> bool;
    fn isLaterThan(&self, datetime: *c_void) -> bool;
    fn isLeapYear(year: int, cal: int) -> bool;
    fn isSameDate(&self, dt: @wxDateTime) -> bool;
    fn isSameTime(&self, dt: @wxDateTime) -> bool;
    fn isStrictlyBetween(&self, t1: @wxDateTime, t2: @wxDateTime) -> bool;
    fn isValid(&self) -> bool;
    fn isWestEuropeanCountry(country: int) -> bool;
    fn isWorkDay(&self, country: int) -> bool;
    fn makeGMT(&self, noDST: int);
    fn makeTimezone(&self, tz: int, noDST: int);
    fn now(&self);
    fn parseDate(&self, date: *c_void);
    fn parseDateTime(&self, datetime: *c_void);
    fn parseFormat(&self, date: *c_void, format: *c_void, dateDef: *c_void);
    fn parseRfc822Date(&self, date: *c_void);
    fn parseTime(&self, time: @wxTime);
    fn resetTime(&self);
    fn set(&self, day: int, month: int, year: int, hour: int, minute: int, second: int, millisec: int);
    fn setCountry(country: int);
    fn setDay(&self, day: int);
    fn setHour(&self, hour: int);
    fn setMillisecond(&self, millisecond: int);
    fn setMinute(&self, minute: int);
    fn setMonth(&self, month: int);
    fn setSecond(&self, second: int);
    fn setTime(&self, hour: int, minute: int, second: int, millisec: int);
    fn setToCurrent(&self);
    fn setToLastMonthDay(&self, month: int, year: int);
    fn setToLastWeekDay(&self, weekday: int, month: int, year: int) -> bool;
    fn setToNextWeekDay(&self, weekday: int);
    fn setToPrevWeekDay(&self, weekday: int);
    fn setToWeekDay(&self, weekday: int, n: int, month: int, year: int) -> bool;
    fn setToWeekDayInSameWeek(&self, weekday: int);
    fn setYear(&self, year: int);
    fn subtractDate(&self, diff: *c_void, @wxDateTime);
    fn subtractTime(&self, diff: *c_void, @wxDateTime);
    fn toGMT(&self, noDST: int);
    fn toTimezone(&self, tz: int, noDST: int);
    fn today(&self);
    fn uNow(&self);
    fn wxDateTime(hi_long: int, lo_long: int);
    fn delete(&self);
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
    fn isConnectedEvent(&self) -> bool;
    fn isOwnEvent(&self) -> bool;
}
trait wxDialUpManager {
    fn cancelDialing(&self) -> bool;
    fn new() -> @wxDialUpManager;
    fn delete(&self);
    fn dial(&self, nameOfISP: @wxString, username: @wxString, password: @wxString, async: bool) -> bool;
    fn disableAutoCheckOnlineStatus(&self);
    fn enableAutoCheckOnlineStatus(&self, nSeconds: int) -> bool;
    fn getISPNames(&self, _lst: @wxList) -> int;
    fn hangUp(&self) -> bool;
    fn isAlwaysOnline(&self) -> bool;
    fn isDialing(&self) -> bool;
    fn isOk(&self) -> bool;
    fn isOnline(&self) -> bool;
    fn setConnectCommand(&self, commandDial: @wxString, commandHangup: @wxString);
    fn setOnlineStatus(&self, isOnline: bool);
    fn setWellKnownHost(&self, hostname: @wxString, portno: int);
}
trait wxDialog {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxDialog;
    fn endModal(&self, retCode: int);
    fn getReturnCode(&self) -> int;
    fn isModal(&self) -> bool;
    fn setReturnCode(&self, returnCode: int);
    fn showModal(&self) -> int;
}
trait wxDirDialog {
    fn new(_prt: @wxWindow, _msg: @wxString, _dir: @wxString, _lft: c_int, _top: c_int, _stl: int) -> @wxDirDialog;
    fn getMessage(&self) -> @wxString;
    fn getPath(&self) -> @wxString;
    fn getStyle(&self) -> int;
    fn setMessage(&self, msg: @wxString);
    fn setPath(&self, pth: @wxString);
    fn setStyle(&self, style: int);
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
    fn new(image: @wxBitmap, x: int, y: int) -> @wxDragImage;
    fn delete(&self);
    fn beginDragFullScreen(&self, x_pos: int, y_pos: int, window: @wxWindow, fullScreen: bool, rect: @wxRect) -> bool;
    fn beginDrag(&self, x: int, y: int, window: @wxWindow, boundingWindow: @wxWindow) -> bool;
    fn endDrag(&self);
    fn hide(&self) -> bool;
    fn move(&self, x: int, y: int) -> bool;
    fn show(&self) -> bool;
}
trait wxDrawControl {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxDrawControl;
}
trait wxDrawWindow {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxDrawWindow;
}
trait wxDropFilesEvent {
}
trait wxDropSource {
}
trait wxDropTarget {
    fn getData(&self);
    fn setDataObject(&self, _dat: @wxDataObject);
}
trait wxDynToolInfo {
    fn index(&self) -> int;
    fn realSize(&self, _w: *c_int, _h: *c_int);
    fn pToolWnd(&self);
}
trait wxDynamicLibrary {
}
trait wxDynamicSashWindow {
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxDynamicSashWindow;
    fn delete(&self);
    fn getHScrollBar(&self, child: @wxWindow);
    fn getVScrollBar(&self, child: @wxWindow);
}
trait wxDynamicToolBar {
    fn addSeparator(&self, pSepartorWnd: *c_void);
    fn addTool(&self, toolIndex: int, pToolWindow: *c_void, w: c_int, h: c_int);
    fn addToolBitmap(&self, toolIndex: int, bitmap: @wxBitmap, pushedBitmap: *c_void, toggle: int, x: c_int, y: c_int, clientData: @wxClientData, helpString1: *c_void, helpString2: *c_void);
    fn addToolImage(&self, toolIndex: int, imageFileName: *c_void, imageFileType: int, labelText: *c_void, alignTextRight: int, isFlat: bool);
    fn addToolLabel(&self, toolIndex: int, labelBmp: *c_void, labelText: *c_void, alignTextRight: int, isFlat: bool);
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int, orientation: int, RowsOrColumns: int) -> @wxDynamicToolBar;
    fn newDefault() -> @wxDynamicToolBar;
    fn newDefaultLayout(&self);
    fn newParams(&self, parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int, orientation: int, RowsOrColumns: int) -> int;
    fn newTool(&self, id: int, label: *c_void, bmpNormal: *c_void, bmpDisabled: *c_void, kind: int, clientData: @wxClientData, shortHelp: *c_void, longHelp: *c_void);
    fn newToolControl(&self, control: @wxControl);
    fn delete(&self);
    fn doDeleteTool(&self, pos: int, tool: *c_void) -> int;
    fn doEnableTool(&self, tool: *c_void, enable: bool);
    fn doInsertTool(&self, pos: int, tool: *c_void) -> int;
    fn doSetToggle(&self, tool: *c_void, toggle: int);
    fn doToggleTool(&self, tool: *c_void, toggle: int);
    fn drawSeparator(&self, info: *c_void, dc: @wxDC);
    fn enableTool(&self, toolIndex: int, enable: bool);
    fn findToolForPosition(&self, x: c_int, y: c_int);
    fn getPreferredDim(&self, gw: int, gh: int, pw: *c_void, ph: *c_void);
    fn getToolInfo(&self, toolIndex: int);
    fn layout(&self) -> int;
    fn removeTool(&self, toolIndex: int);
    fn setLayout(&self, pLayout: *c_void);
}
trait wxEditableListBox {
    fn new(parent: @wxWindow, id: int, label: *wchar_t, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxEditableListBox;
    fn getDelButton(&self);
    fn getDownButton(&self);
    fn getEditButton(&self);
    fn getListCtrl(&self) -> @wxListCtrl;
    fn getNewButton(&self);
    fn getStrings(&self, _ref: **wchar_t) -> c_int;
    fn getUpButton(&self);
    fn setStrings(&self, strings: *c_void, _n: int);
}
trait wxEncodingConverter {
    fn convert(&self, input: *c_void, output: *c_void);
    fn new() -> @wxEncodingConverter;
    fn delete(&self);
    fn getAllEquivalents(&self, enc: int, _lst: @wxList) -> int;
    fn getPlatformEquivalents(&self, enc: int, platform: int, _lst: @wxList) -> int;
    fn init(&self, input_enc: int, output_enc: int, method: int) -> int;
}
trait wxEraseEvent {
    fn copyObject(&self, obj: *c_void);
    fn getDC(&self) -> @wxDC;
}
trait wxEvent {
    fn copyObject(&self, object_dest: *c_void);
    fn getEventObject(&self) -> @wxObject;
    fn getEventType(&self) -> int;
    fn getId(&self) -> int;
    fn getSkipped(&self) -> bool;
    fn getTimestamp(&self) -> int;
    fn isCommandEvent(&self) -> bool;
    fn newEventType() -> int;
    fn setEventObject(&self, obj: @wxObject);
    fn setEventType(&self, typ: int);
    fn setId(&self, Id: int);
    fn setTimestamp(&self, ts: int);
    fn skip(&self);
}
trait wxEvtHandler {
    fn addPendingEvent(&self, event: @wxEvent);
    fn connect(&self, first: int, last: int, type_: int, data: *c_void) -> int;
    fn new() -> @wxEvtHandler;
    fn delete(&self);
    fn disconnect(&self, first: int, last: int, type_: int, id: int) -> int;
    fn getEvtHandlerEnabled(&self) -> bool;
    fn getNextHandler(&self) -> @wxEvtHandler;
    fn getPreviousHandler(&self) -> @wxEvtHandler;
    fn processEvent(&self, event: @wxEvent) -> bool;
    fn processPendingEvents(&self);
    fn setEvtHandlerEnabled(&self, enabled: bool);
    fn setNextHandler(&self, handler: @wxEvtHandler);
    fn setPreviousHandler(&self, handler: @wxEvtHandler);
    fn getClosure(&self, id: int, type_: int) -> @wxClosure;
    fn getClientClosure(&self) -> @wxClosure;
    fn setClientClosure(&self, closure: @wxClosure);
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
    fn new(_prt: @wxWindow, _msg: @wxString, _dir: @wxString, _fle: @wxString, _wcd: @wxString, _lft: c_int, _top: c_int, _stl: int) -> @wxFileDialog;
    fn getDirectory(&self) -> @wxString;
    fn getFilename(&self) -> @wxString;
    fn getFilenames(&self, paths: **wchar_t) -> c_int;
    fn getFilterIndex(&self) -> int;
    fn getMessage(&self) -> @wxString;
    fn getPath(&self) -> @wxString;
    fn getPaths(&self, paths: **wchar_t) -> c_int;
    fn getStyle(&self) -> int;
    fn getWildcard(&self) -> @wxString;
    fn setDirectory(&self, dir: @wxString);
    fn setFilename(&self, name: @wxString);
    fn setFilterIndex(&self, filterIndex: int);
    fn setMessage(&self, message: @wxString);
    fn setPath(&self, path: @wxString);
    fn setStyle(&self, style: int);
    fn setWildcard(&self, wildCard: @wxString);
}
trait wxFileDropTarget {
}
trait wxFileHistory {
    fn addFileToHistory(&self, file: @wxString);
    fn addFilesToMenu(&self, menu: @wxMenu);
    fn new(maxFiles: int) -> @wxFileHistory;
    fn delete(&self);
    fn getCount(&self) -> int;
    fn getHistoryFile(&self, i: int) -> @wxString;
    fn getMaxFiles(&self) -> int;
    fn getMenus(&self, ~[@wxMenu]) -> c_int;
    fn load(&self, config: @wxConfigBase);
    fn removeFileFromHistory(&self, i: int);
    fn removeMenu(&self, menu: @wxMenu);
    fn save(&self, config: @wxConfigBase);
    fn useMenu(&self, menu: @wxMenu);
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
    fn delete(&self);
    fn expandCommand(&self, _cmd: *c_void, _params: *c_void) -> @wxString;
    fn getDescription(&self) -> @wxString;
    fn getExtensions(&self, _lst: @wxList) -> int;
    fn getIcon(&self, icon: @wxIcon) -> int;
    fn getMimeType(&self) -> @wxString;
    fn getMimeTypes(&self, _lst: @wxList) -> int;
    fn getOpenCommand(&self, _buf: *c_void, _params: *c_void) -> int;
    fn getPrintCommand(&self, _buf: *c_void, _params: *c_void) -> int;
}
trait wxFilterInputStream {
}
trait wxFilterOutputStream {
}
trait wxFindDialogEvent {
    fn getFindString(&self, _ref: *c_void) -> int;
    fn getFlags(&self) -> int;
    fn getReplaceString(&self, _ref: *c_void) -> int;
}
trait wxFindReplaceData {
    fn new(flags: int) -> @wxFindReplaceData;
    fn newDefault() -> @wxFindReplaceData;
    fn delete(&self);
    fn getFindString(&self) -> @wxString;
    fn getFlags(&self) -> int;
    fn getReplaceString(&self) -> @wxString;
    fn setFindString(&self, str: @wxString);
    fn setFlags(&self, flags: int);
    fn setReplaceString(&self, str: @wxString);
}
trait wxFindReplaceDialog {
    fn new(parent: @wxWindow, data: @wxFindReplaceData, title: @wxString, style: int) -> @wxFindReplaceDialog;
    fn getData(&self) -> @wxFindReplaceData;
    fn setData(&self, data: @wxFindReplaceData);
}
trait wxFlexGridSizer {
    fn addGrowableCol(&self, idx: size_t);
    fn addGrowableRow(&self, idx: size_t);
    fn calcMin(&self) -> @wxSize;
    fn new(rows: int, cols: int, vgap: int, hgap: int) -> @wxFlexGridSizer;
    fn recalcSizes(&self);
    fn removeGrowableCol(&self, idx: size_t);
    fn removeGrowableRow(&self, idx: size_t);
}
trait wxFocusEvent {
}
trait wxFont {
    fn new(pointSize: int, family: int, style: int, weight: int, underlined: bool, face: @wxString, enc: int) -> @wxFont;
    fn newFromStock(id: int) -> @wxFont;
    fn newDefault() -> @wxFont;
    fn delete(&self);
    fn getDefaultEncoding(&self) -> int;
    fn getEncoding(&self) -> int;
    fn getFaceName(&self) -> @wxString;
    fn getFamily(&self) -> int;
    fn getFamilyString(&self) -> @wxString;
    fn getPointSize(&self) -> int;
    fn getStyle(&self) -> int;
    fn getStyleString(&self) -> @wxString;
    fn getUnderlined(&self) -> int;
    fn getWeight(&self) -> int;
    fn getWeightString(&self) -> @wxString;
    fn isOk(&self) -> bool;
    fn setDefaultEncoding(&self, encoding: int);
    fn setEncoding(&self, encoding: int);
    fn setFaceName(&self, faceName: @wxString);
    fn setFamily(&self, family: int);
    fn setPointSize(&self, pointSize: int);
    fn setStyle(&self, style: int);
    fn setUnderlined(&self, underlined: int);
    fn setWeight(&self, weight: int);
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
}
trait wxFontData {
    fn new() -> @wxFontData;
    fn delete(&self);
    fn enableEffects(&self, flag: bool);
    fn getAllowSymbols(&self) -> bool;
    fn getChosenFont(&self, @wxFont);
    fn getColour(&self, @wxColour);
    fn getEnableEffects(&self) -> bool;
    fn getEncoding(&self) -> int;
    fn getInitialFont(&self, @wxFont);
    fn getShowHelp(&self) -> int;
    fn setAllowSymbols(&self, flag: bool);
    fn setChosenFont(&self, font: @wxFont);
    fn setColour(&self, colour: @wxColour);
    fn setEncoding(&self, encoding: int);
    fn setInitialFont(&self, font: @wxFont);
    fn setRange(&self, minRange: int, maxRange: int);
    fn setShowHelp(&self, flag: bool);
}
trait wxFontDialog {
    fn new(_prt: @wxWindow, fnt: @wxFontData) -> @wxFontDialog;
    fn getFontData(&self, @wxFontData);
}
trait wxFontEnumerator {
    fn new(_obj: *c_void, _fnc: *c_void) -> @wxFontEnumerator;
    fn delete(&self);
    fn enumerateEncodings(&self, facename: @wxString) -> bool;
    fn enumerateFacenames(&self, encoding: int, fixedWidthOnly: int) -> bool;
}
trait wxFontList {
}
trait wxFontMapper {
    fn new() -> @wxFontMapper;
    fn getAltForEncoding(&self, encoding: int, alt_encoding: *c_void, _buf: @wxString) -> bool;
    fn isEncodingAvailable(&self, encoding: int, _buf: @wxString) -> bool;
}
trait wxFrame {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxFrame;
    fn newStatusBar(&self, number: int, style: int) -> @wxStatusBar;
    fn newToolBar(&self, style: c_long) -> @wxToolBar;
    fn getClientAreaOrigin_left(&self) -> int;
    fn getClientAreaOrigin_top(&self) -> int;
    fn getMenuBar(&self) -> @wxMenuBar;
    fn getStatusBar(&self) -> @wxStatusBar;
    fn getToolBar(&self) -> @wxToolBar;
    fn restore(&self);
    fn setMenuBar(&self, menubar: @wxMenuBar);
    fn setStatusBar(&self, statBar: @wxStatusBar);
    fn setStatusText(&self, _txt: @wxString, _number: int);
    fn setStatusWidths(&self, _n: int, _widths_field: *c_void);
    fn setToolBar(&self, _toolbar: @wxToolBar);
    fn getTitle(&self) -> @wxString;
    fn setTitle(&self, _txt: @wxString);
    fn setShape(&self, region: @wxRegion) -> bool;
    fn showFullScreen(&self, show: bool, style: int) -> bool;
    fn isFullScreen(&self) -> bool;
    fn centre(&self, orientation: int);
}
trait wxFrameLayout {
    fn activate(&self);
    fn addBar(&self, pBarWnd: *c_void, dimInfo: *c_void, alignment: int, rowNo: int, columnPos: int, name: *wchar_t, spyEvents: int, state: int);
    fn addPlugin(&self, pPlInfo: *c_void, paneMask: int);
    fn addPluginBefore(&self, pNextPlInfo: *c_void, pPlInfo: *c_void, paneMask: int);
    fn applyBarProperties(&self, pBar: *c_void);
    fn captureEventsForPane(&self, toPane: *c_void);
    fn captureEventsForPlugin(&self, pPlugin: *c_void);
    fn new(pParentFrame: *c_void, pFrameClient: *c_void, activateNow: int) -> @wxFrameLayout;
    fn deactivate(&self);
    fn delete(&self);
    fn destroyBarWindows(&self);
    fn enableFloating(&self, enable: bool);
    fn findBarByName(&self, name: *wchar_t);
    fn findBarByWindow(&self, pWnd: *c_void);
    fn findPlugin(&self, pPlInfo: *c_void);
    fn firePluginEvent(&self, event: @wxEvent);
    fn getBars(&self, _ref: *c_void) -> int;
    fn getClientHeight(&self) -> int;
    fn getClientRect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn getClientWidth(&self) -> int;
    fn getFrameClient(&self);
    fn getPane(&self, alignment: int);
    fn getPaneProperties(&self, props: *c_void, alignment: int);
    fn getParentFrame(&self);
    fn getTopPlugin(&self);
    fn getUpdatesManager(&self);
    fn hasTopPlugin(&self) -> bool;
    fn hideBarWindows(&self);
    fn inverseVisibility(&self, pBar: *c_void);
    fn onLButtonDown(&self, event: @wxEvent);
    fn onLButtonUp(&self, event: @wxEvent);
    fn onLDblClick(&self, event: @wxEvent);
    fn onMouseMove(&self, event: @wxEvent);
    fn onRButtonDown(&self, event: @wxEvent);
    fn onRButtonUp(&self, event: @wxEvent);
    fn onSize(&self, event: @wxEvent);
    fn popAllPlugins(&self);
    fn popPlugin(&self);
    fn pushDefaultPlugins(&self);
    fn pushPlugin(&self, pPugin: *c_void);
    fn recalcLayout(&self, repositionBarsNow: int);
    fn redockBar(&self, pBar: *c_void, x: c_int, y: c_int, w: c_int, h: c_int, pToPane: *c_void, updateNow: int) -> int;
    fn refreshNow(&self, recalcLayout: int);
    fn releaseEventsFromPane(&self, fromPane: *c_void);
    fn releaseEventsFromPlugin(&self, pPlugin: *c_void);
    fn removeBar(&self, pBar: *c_void);
    fn removePlugin(&self, pPlInfo: *c_void);
    fn setBarState(&self, pBar: *c_void, newStatem: int, updateNow: int);
    fn setFrameClient(&self, pFrameClient: *c_void);
    fn setMargins(&self, top: int, bottom: int, left: int, right: int, paneMask: int);
    fn setPaneBackground(&self, colour: @wxColour);
    fn setPaneProperties(&self, props: *c_void, paneMask: int);
    fn setTopPlugin(&self, pPlugin: *c_void);
    fn setUpdatesManager(&self, pUMgr: *c_void);
}
trait wxGDIObject {
}
trait wxGLCanvas {
    fn new(parent: @wxWindow, windowID: int, attributes: *int, x: c_int, y: c_int, w: c_int, h: c_int, style: int, title: @wxString, palette: @wxPalette) -> @wxGLCanvas;
    fn setColour(&self, colour: @wxColour) -> bool;
    fn setCurrent(&self, ctxt: @wxGLContext) -> bool;
    fn swapBuffers(&self) -> bool;
    fn isDisplaySupported(attributes: *int) -> bool;
    fn isExtensionSupported(extension: @wxString) -> bool;
}
trait wxGauge {
    fn new(_prt: @wxWindow, _id: int, _rng: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxGauge;
    fn getBezelFace(&self) -> int;
    fn getRange(&self) -> int;
    fn getShadowWidth(&self) -> int;
    fn getValue(&self) -> int;
    fn setBezelFace(&self, w: int);
    fn setRange(&self, r: int);
    fn setShadowWidth(&self, w: int);
    fn setValue(&self, pos: int);
}
trait wxGenericDirCtrl {
}
trait wxGenericValidator {
}
trait wxGrid {
    fn appendCols(&self, numCols: int, updateLabels: bool) -> bool;
    fn appendRows(&self, numRows: int, updateLabels: bool) -> bool;
    fn autoSize(&self);
    fn autoSizeColumn(&self, col: int, setAsMin: c_int);
    fn autoSizeColumns(&self, setAsMin: c_int);
    fn autoSizeRow(&self, row: int, setAsMin: c_int);
    fn autoSizeRows(&self, setAsMin: c_int);
    fn beginBatch(&self);
    fn blockToDeviceRect(&self, top: int, left: int, bottom: int, right: int) -> @wxRect;
    fn canDragColSize(&self) -> bool;
    fn canDragGridSize(&self) -> bool;
    fn canDragRowSize(&self) -> bool;
    fn canEnableCellControl(&self) -> bool;
    fn cellToRect(&self, row: int, col: int) -> @wxRect;
    fn clearGrid(&self);
    fn clearSelection(&self);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxGrid;
    fn newGrid(&self, rows: int, cols: int, selmode: int);
    fn deleteCols(&self, pos: int, numCols: int, updateLabels: bool) -> bool;
    fn deleteRows(&self, pos: int, numRows: int, updateLabels: bool) -> bool;
    fn disableCellEditControl(&self);
    fn disableDragColSize(&self);
    fn disableDragGridSize(&self);
    fn disableDragRowSize(&self);
    fn drawAllGridLines(&self, dc: @wxDC, reg: @wxRegion);
    fn drawCell(&self, dc: @wxDC, _row: int, _col: int);
    fn drawCellBorder(&self, dc: @wxDC, _row: int, _col: int);
    fn drawCellHighlight(&self, dc: @wxDC, attr: @wxGridCellAttr);
    fn drawColLabel(&self, dc: @wxDC, col: int);
    fn drawColLabels(&self, dc: @wxDC);
    fn drawGridSpace(&self, dc: @wxDC);
    fn drawRowLabel(&self, dc: @wxDC, row: int);
    fn drawRowLabels(&self, dc: @wxDC);
    fn drawTextRectangle(&self, dc: @wxDC, txt: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: int, verticalAlignment: int);
    fn enableCellEditControl(&self, enable: bool);
    fn enableDragColSize(&self, enable: bool);
    fn enableDragGridSize(&self, enable: bool);
    fn enableDragRowSize(&self, enable: bool);
    fn enableEditing(&self, edit: c_int);
    fn enableGridLines(&self, enable: bool);
    fn endBatch(&self);
    fn getBatchCount(&self) -> int;
    fn getCellAlignment(&self, row: int, col: int, horiz: *c_int, vert: *c_int);
    fn getCellBackgroundColour(&self, row: int, col: int, colour: @wxColour);
    fn getCellEditor(&self, row: int, col: int) -> @wxGridCellEditor;
    fn getCellFont(&self, row: int, col: int, font: @wxFont);
    fn getCellHighlightColour(&self, @wxColour);
    fn getCellRenderer(&self, row: int, col: int) -> @wxGridCellRenderer;
    fn getCellTextColour(&self, row: int, col: int, colour: @wxColour);
    fn getCellValue(&self, row: int, col: int) -> @wxString;
    fn getColLabelAlignment(&self, horiz: *c_int, vert: *c_int);
    fn getColLabelSize(&self) -> int;
    fn getColLabelValue(&self, col: int) -> @wxString;
    fn getColSize(&self, col: int) -> int;
    fn getDefaultCellAlignment(&self, horiz: *c_int, vert: *c_int);
    fn getDefaultCellBackgroundColour(&self, @wxColour);
    fn getDefaultCellFont(&self, @wxFont);
    fn getDefaultCellTextColour(&self, @wxColour);
    fn getDefaultColLabelSize(&self) -> int;
    fn getDefaultColSize(&self) -> int;
    fn getDefaultEditor(&self) -> @wxGridCellEditor;
    fn getDefaultEditorForCell(&self, row: int, col: int) -> @wxGridCellEditor;
    fn getDefaultEditorForType(&self, typeName: @wxString) -> @wxGridCellEditor;
    fn getDefaultRenderer(&self) -> @wxGridCellRenderer;
    fn getDefaultRendererForCell(&self, row: int, col: int) -> @wxGridCellRenderer;
    fn getDefaultRendererForType(&self, typeName: @wxString) -> @wxGridCellRenderer;
    fn getDefaultRowLabelSize(&self) -> int;
    fn getDefaultRowSize(&self) -> int;
    fn getGridCursorCol(&self) -> int;
    fn getGridCursorRow(&self) -> int;
    fn getGridLineColour(&self, @wxColour);
    fn getLabelBackgroundColour(&self, @wxColour);
    fn getLabelFont(&self, @wxFont);
    fn getLabelTextColour(&self, @wxColour);
    fn getNumberCols(&self) -> int;
    fn getNumberRows(&self) -> int;
    fn getRowLabelAlignment(&self, horiz: *c_int, vert: *c_int);
    fn getRowLabelSize(&self) -> int;
    fn getRowLabelValue(&self, row: int) -> @wxString;
    fn getRowSize(&self, row: int) -> int;
    fn getSelectionBackground(&self, @wxColour);
    fn getSelectionForeground(&self, @wxColour);
    fn getTable(&self) -> @wxGridTableBase;
    fn getTextBoxSize(&self, dc: @wxDC, count: c_int, lines: *wchar_t, _w: *c_int, _h: *c_int);
    fn gridLinesEnabled(&self) -> int;
    fn hideCellEditControl(&self);
    fn insertCols(&self, pos: int, numCols: int, updateLabels: bool) -> bool;
    fn insertRows(&self, pos: int, numRows: int, updateLabels: bool) -> bool;
    fn isCellEditControlEnabled(&self) -> bool;
    fn isCellEditControlShown(&self) -> bool;
    fn isCurrentCellReadOnly(&self) -> bool;
    fn isEditable(&self) -> bool;
    fn isInSelection(&self, row: int, col: int) -> bool;
    fn isReadOnly(&self, row: int, col: int) -> bool;
    fn isSelection(&self) -> bool;
    fn isVisible(&self, row: int, col: int, wholeCellVisible: bool) -> bool;
    fn makeCellVisible(&self, row: int, col: int);
    fn moveCursorDown(&self, expandSelection: bool) -> bool;
    fn moveCursorDownBlock(&self, expandSelection: bool) -> bool;
    fn moveCursorLeft(&self, expandSelection: bool) -> bool;
    fn moveCursorLeftBlock(&self, expandSelection: bool) -> bool;
    fn moveCursorRight(&self, expandSelection: bool) -> bool;
    fn moveCursorRightBlock(&self, expandSelection: bool) -> bool;
    fn moveCursorUp(&self, expandSelection: bool) -> bool;
    fn moveCursorUpBlock(&self, expandSelection: bool) -> bool;
    fn movePageDown(&self) -> bool;
    fn movePageUp(&self) -> bool;
    fn processTableMessage(&self, evt: @wxEvent) -> bool;
    fn registerDataType(&self, typeName: @wxString, renderer: @wxGridCellRenderer, editor: @wxGridCellEditor);
    fn saveEditControlValue(&self);
    fn selectAll(&self);
    fn selectBlock(&self, topRow: int, leftCol: int, bottomRow: int, rightCol: int, addToSelected: c_int);
    fn selectCol(&self, col: int, addToSelected: c_int);
    fn selectRow(&self, row: int, addToSelected: c_int);
    fn setCellAlignment(&self, row: int, col: int, horiz: int, vert: int);
    fn setCellBackgroundColour(&self, row: int, col: int, colour: @wxColour);
    fn setCellEditor(&self, row: int, col: int, editor: @wxGridCellEditor);
    fn setCellFont(&self, row: int, col: int, font: @wxFont);
    fn setCellHighlightColour(&self, col: @wxColour);
    fn setCellRenderer(&self, row: int, col: int, renderer: @wxGridCellRenderer);
    fn setCellTextColour(&self, row: int, col: int, colour: @wxColour);
    fn setCellValue(&self, row: int, col: int, s: @wxString);
    fn setColAttr(&self, col: int, attr: @wxGridCellAttr);
    fn setColFormatBool(&self, col: int);
    fn setColFormatCustom(&self, col: int, typeName: @wxString);
    fn setColFormatFloat(&self, col: int, width: int, precision: int);
    fn setColFormatNumber(&self, col: int);
    fn setColLabelAlignment(&self, horiz: int, vert: int);
    fn setColLabelSize(&self, height: int);
    fn setColLabelValue(&self, col: int, label: @wxString);
    fn setColMinimalWidth(&self, col: int, width: int);
    fn setColSize(&self, col: int, width: int);
    fn setDefaultCellAlignment(&self, horiz: int, vert: int);
    fn setDefaultCellBackgroundColour(&self, colour: @wxColour);
    fn setDefaultCellFont(&self, font: @wxFont);
    fn setDefaultCellTextColour(&self, colour: @wxColour);
    fn setDefaultColSize(&self, width: int, resizeExistingCols: c_int);
    fn setDefaultEditor(&self, editor: @wxGridCellEditor);
    fn setDefaultRenderer(&self, renderer: @wxGridCellRenderer);
    fn setDefaultRowSize(&self, height: int, resizeExistingRows: c_int);
    fn setGridCursor(&self, row: int, col: int);
    fn setGridLineColour(&self, col: @wxColour);
    fn setLabelBackgroundColour(&self, colour: @wxColour);
    fn setLabelFont(&self, font: @wxFont);
    fn setLabelTextColour(&self, colour: @wxColour);
    fn setMargins(&self, extraWidth: int, extraHeight: int);
    fn setReadOnly(&self, row: int, col: int, isReadOnly: bool);
    fn setRowAttr(&self, row: int, attr: @wxGridCellAttr);
    fn setRowLabelAlignment(&self, horiz: int, vert: int);
    fn setRowLabelSize(&self, width: int);
    fn setRowLabelValue(&self, row: int, label: @wxString);
    fn setRowMinimalHeight(&self, row: int, width: int);
    fn setRowSize(&self, row: int, height: int);
    fn setSelectionBackground(&self, c: @wxColour);
    fn setSelectionForeground(&self, c: @wxColour);
    fn setSelectionMode(&self, selmode: int);
    fn setTable(&self, table: @wxGridTableBase, takeOwnership: bool, selmode: int) -> bool;
    fn showCellEditControl(&self);
    fn stringToLines(&self, value: @wxString, lines: *c_void) -> int;
    fn xToCol(&self, x: int) -> int;
    fn xToEdgeOfCol(&self, x: int) -> int;
    fn xYToCell(&self, x: c_int, y: c_int, row: *c_int, col: *c_int);
    fn yToEdgeOfRow(&self, y: int) -> int;
    fn yToRow(&self, y: int) -> int;
    fn getSelectedCells(&self, @wxGridCellCoordsArray);
    fn getSelectionBlockTopLeft(&self, @wxGridCellCoordsArray);
    fn getSelectionBlockBottomRight(&self, @wxGridCellCoordsArray);
    fn getSelectedRows(&self, _arr: *intptr_t) -> c_int;
    fn getSelectedCols(&self, _arr: *intptr_t) -> c_int;
    fn getCellSize(&self, row: int, col: int, srow: *c_int, scol: *c_int);
    fn setCellSize(&self, row: int, col: int, srow: c_int, scol: c_int);
}
trait wxGridCellAttr {
    fn ctor() -> @wxGridCellAttr;
    fn decRef(&self);
    fn getAlignment(&self, hAlign: *c_int, vAlign: *c_int);
    fn getBackgroundColour(&self, @wxColour);
    fn getEditor(&self, grid: @wxGrid, row: int, col: int) -> @wxGridCellEditor;
    fn getFont(&self, @wxFont);
    fn getRenderer(&self, grid: @wxGrid, row: int, col: int) -> @wxGridCellRenderer;
    fn getTextColour(&self, @wxColour);
    fn hasAlignment(&self) -> bool;
    fn hasBackgroundColour(&self) -> bool;
    fn hasEditor(&self) -> bool;
    fn hasFont(&self) -> bool;
    fn hasRenderer(&self) -> bool;
    fn hasTextColour(&self) -> bool;
    fn incRef(&self);
    fn isReadOnly(&self) -> bool;
    fn setAlignment(&self, hAlign: int, vAlign: int);
    fn setBackgroundColour(&self, colBack: @wxColour);
    fn setDefAttr(&self, defAttr: @wxGridCellAttr);
    fn setEditor(&self, editor: @wxGridCellEditor);
    fn setFont(&self, font: @wxFont);
    fn setReadOnly(&self, isReadOnly: bool);
    fn setRenderer(&self, renderer: @wxGridCellRenderer);
    fn setTextColour(&self, colText: @wxColour);
}
trait wxGridCellBoolEditor {
    fn ctor() -> @wxGridCellBoolEditor;
}
trait wxGridCellBoolRenderer {
}
trait wxGridCellChoiceEditor {
    fn ctor(count: c_int, choices: *wchar_t, allowOthers: c_int) -> @wxGridCellChoiceEditor;
}
trait wxGridCellCoordsArray {
    fn new() -> @wxGridCellCoordsArray;
    fn delete(&self);
    fn getCount(&self) -> int;
    fn item(&self, _idx: int, _c: *c_int, _r: *c_int);
}
trait wxGridCellEditor {
    fn beginEdit(&self, row: int, col: int, grid: @wxGrid);
    fn new(&self, parent: @wxWindow, id: int, evtHandler: @wxEvtHandler);
    fn destroy(&self);
    fn endEdit(&self, row: int, col: int, grid: @wxGrid, oldStr: @wxString, newStr: @wxString) -> int;
    fn getControl(&self) -> @wxControl;
    fn handleReturn(&self, event: @wxEvent);
    fn isAcceptedKey(&self, event: @wxEvent) -> bool;
    fn isCreated(&self) -> bool;
    fn paintBackground(&self, x: c_int, y: c_int, w: c_int, h: c_int, attr: @wxGridCellAttr);
    fn reset(&self);
    fn setControl(&self, control: @wxControl);
    fn setParameters(&self, params: @wxString);
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int);
    fn show(&self, show: c_int, attr: @wxGridCellAttr);
    fn startingClick(&self);
    fn startingKey(&self, event: @wxEvent);
}
trait wxGridCellFloatEditor {
    fn ctor(width: int, precision: int) -> @wxGridCellFloatEditor;
}
trait wxGridCellFloatRenderer {
}
trait wxGridCellNumberEditor {
    fn ctor(min: int, max: int) -> @wxGridCellNumberEditor;
}
trait wxGridCellNumberRenderer {
    fn ctor() -> @wxGridCellNumberRenderer;
}
trait wxGridCellAutoWrapStringRenderer {
    fn ctor() -> @wxGridCellAutoWrapStringRenderer;
}
trait wxGridCellRenderer {
}
trait wxGridCellStringRenderer {
}
trait wxGridCellTextEditor {
    fn ctor() -> @wxGridCellTextEditor;
}
trait wxGridCellWorker {
}
trait wxGridEditorCreatedEvent {
    fn getCol(&self) -> int;
    fn getControl(&self) -> @wxControl;
    fn getRow(&self) -> int;
    fn setCol(&self, col: int);
    fn setControl(&self, ctrl: @wxControl);
    fn setRow(&self, row: int);
}
trait wxGridEvent {
    fn altDown(&self) -> bool;
    fn controlDown(&self) -> bool;
    fn getCol(&self) -> int;
    fn getPosition(&self) -> @wxPoint;
    fn getRow(&self) -> int;
    fn metaDown(&self) -> bool;
    fn selecting(&self) -> bool;
    fn shiftDown(&self) -> bool;
}
trait wxGridRangeSelectEvent {
    fn getTopLeftCoords(&self, col: *c_int, row: *c_int);
    fn getBottomRightCoords(&self, col: *c_int, row: *c_int);
    fn getTopRow(&self) -> int;
    fn getBottomRow(&self) -> int;
    fn getLeftCol(&self) -> int;
    fn getRightCol(&self) -> int;
    fn selecting(&self) -> bool;
    fn controlDown(&self) -> bool;
    fn metaDown(&self) -> bool;
    fn shiftDown(&self) -> bool;
    fn altDown(&self) -> bool;
}
trait wxGridSizeEvent {
    fn getRowOrCol(&self) -> int;
    fn getPosition(&self) -> @wxPoint;
    fn controlDown(&self) -> bool;
    fn metaDown(&self) -> bool;
    fn shiftDown(&self) -> bool;
    fn altDown(&self) -> bool;
}
trait wxGridSizer {
    fn calcMin(&self) -> @wxSize;
    fn new(rows: int, cols: int, vgap: int, hgap: int) -> @wxGridSizer;
    fn getCols(&self) -> int;
    fn getHGap(&self) -> int;
    fn getRows(&self) -> int;
    fn getVGap(&self) -> int;
    fn recalcSizes(&self);
    fn setCols(&self, cols: int);
    fn setHGap(&self, gap: int);
    fn setRows(&self, rows: int);
    fn setVGap(&self, gap: int);
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
    fn new(ctr: @wxHelpControllerBase) -> @wxHelpControllerHelpProvider;
    fn getHelpController(&self) -> @wxHelpControllerBase;
    fn setHelpController(&self, hc: @wxHelpController);
}
trait wxHelpEvent {
    fn getLink(&self) -> @wxString;
    fn getPosition(&self) -> @wxPoint;
    fn getTarget(&self) -> @wxString;
    fn setLink(&self, link: @wxString);
    fn setPosition(&self, x: c_int, y: c_int);
    fn setTarget(&self, target: @wxString);
}
trait wxHelpProvider {
    fn addHelp(&self, window: @wxWindow, text: @wxString);
    fn addHelpById(&self, id: int, text: @wxString);
    fn delete(&self);
    fn get() -> @wxHelpProvider;
    fn getHelp(&self, window: @wxWindow) -> @wxString;
    fn removeHelp(&self, window: @wxWindow);
    fn set(&self) -> @wxHelpProvider;
    fn showHelp(&self, window: @wxWindow) -> bool;
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
    fn addBook(&self, book: *c_void, show_wait_msg: int) -> bool;
    fn new(_style: int) -> @wxHtmlHelpController;
    fn delete(&self);
    fn display(&self, x: *c_void) -> int;
    fn displayBlock(&self, blockNo: int) -> bool;
    fn displayContents(&self) -> int;
    fn displayIndex(&self) -> int;
    fn displayNumber(&self, id: int) -> int;
    fn displaySection(&self, section: @wxString) -> bool;
    fn displaySectionNumber(&self, sectionNo: int) -> bool;
    fn getFrame(&self) -> @wxFrame;
    fn getFrameParameters(&self, title: *c_void, width: *int, height: *int, pos_x: *int, pos_y: *int, newFrameEachTime: *int);
    fn initialize(&self, file: @wxString) -> bool;
    fn keywordSearch(&self, keyword: @wxString) -> bool;
    fn loadFile(&self, file: @wxString) -> bool;
    fn quit(&self) -> bool;
    fn readCustomization(&self, cfg: @wxConfigBase, path: @wxString);
    fn setFrameParameters(&self, title: *c_void, width: c_int, height: c_int, pos_x: int, pos_y: int, newFrameEachTime: bool);
    fn setTempDir(&self, path: @wxString);
    fn setTitleFormat(&self, format: *c_void);
    fn setViewer(&self, viewer: @wxString, flags: int);
    fn useConfig(&self, config: @wxConfigBase, rootpath: @wxString);
    fn writeCustomization(&self, cfg: @wxConfigBase, path: @wxString);
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
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int, _txt: @wxString) -> @wxHtmlWindow;
    fn appendToPage(&self, source: @wxString) -> bool;
    fn getInternalRepresentation(&self) -> @wxHtmlContainerCell;
    fn getOpenedAnchor(&self) -> @wxString;
    fn getOpenedPage(&self) -> @wxString;
    fn getOpenedPageTitle(&self) -> @wxString;
    fn getRelatedFrame(&self) -> @wxFrame;
    fn historyBack(&self) -> bool;
    fn historyCanBack(&self) -> bool;
    fn historyCanForward(&self) -> bool;
    fn historyClear(&self);
    fn historyForward(&self) -> bool;
    fn loadPage(&self, location: @wxString) -> bool;
    fn readCustomization(&self, cfg: @wxConfigBase, path: @wxString);
    fn setBorders(&self, b: int);
    fn setFonts(&self, normal_face: @wxString, fixed_face: @wxString, sizes: *int);
    fn setPage(&self, source: @wxString);
    fn setRelatedFrame(&self, frame: @wxFrame, format: @wxString);
    fn setRelatedStatusBar(&self, bar: int);
    fn writeCustomization(&self, cfg: @wxConfigBase, path: @wxString);
}
trait wxIPV4address {
}
trait wxIcon {
    fn assign(&self, other: *c_void);
    fn copyFromBitmap(&self, bmp: @wxBitmap);
    fn newDefault() -> @wxIcon;
    fn newLoad(name: @wxString, type_: c_long, width: c_int, height: c_int) -> @wxIcon;
    fn delete(&self);
    fn fromRaw(&self, width: c_int, height: c_int) -> @wxIcon;
    fn fromXPM(&self) -> @wxIcon;
    fn getDepth(&self) -> int;
    fn getHeight(&self) -> int;
    fn getWidth(&self) -> int;
    fn isEqual(&self, other: @wxIcon) -> bool;
    fn load(&self, name: @wxString, type_: c_long, width: c_int, height: c_int) -> int;
    fn isOk(&self) -> bool;
    fn setDepth(&self, depth: int);
    fn setHeight(&self, height: int);
    fn setWidth(&self, width: int);
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
}
trait wxIconBundle {
    fn addIcon(&self, icon: @wxIcon);
    fn addIconFromFile(&self, file: @wxString, type_: int);
    fn assign(&self, @wxIconBundle);
    fn newDefault() -> @wxIconBundle;
    fn newFromFile(file: @wxString, type_: int) -> @wxIconBundle;
    fn newFromIcon(icon: @wxIcon) -> @wxIconBundle;
    fn delete(&self);
    fn getIcon(&self, w: c_int, h: c_int, @wxIcon);
}
trait wxIconizeEvent {
}
trait wxIdleEvent {
    fn copyObject(&self, object_dest: @wxObject);
    fn moreRequested(&self) -> bool;
    fn requestMore(&self, needMore: bool);
}
trait wxImage {
    fn canRead(name: @wxString) -> bool;
    fn convertToBitmap(&self, @wxBitmap);
    fn convertToByteString(&self, type_: int, data: *c_char) -> c_int;
    fn convertToLazyByteString(&self, type_: int, data: *c_char) -> c_int;
    fn countColours(&self, stopafter: int) -> int;
    fn newDefault() -> @wxImage;
    fn newFromBitmap(bitmap: @wxBitmap) -> @wxImage;
    fn newFromByteString(data: **char, length: c_int, type_: int) -> @wxImage;
    fn newFromLazyByteString(data: **char, length: c_int, type_: int) -> @wxImage;
    fn newFromData(width: c_int, height: c_int, data: *c_void) -> @wxImage;
    fn newFromFile(name: @wxString) -> @wxImage;
    fn newSized(width: c_int, height: c_int) -> @wxImage;
    fn destroy(&self);
    fn getBlue(&self, x: c_int, y: c_int) -> wchar_t;
    fn getData(&self);
    fn getGreen(&self, x: c_int, y: c_int) -> wchar_t;
    fn getHeight(&self) -> int;
    fn getMaskBlue(&self) -> wchar_t;
    fn getMaskGreen(&self) -> wchar_t;
    fn getMaskRed(&self) -> wchar_t;
    fn getRed(&self, x: c_int, y: c_int) -> wchar_t;
    fn getSubImage(&self, x: c_int, y: c_int, w: c_int, h: c_int, @wxImage);
    fn getWidth(&self) -> int;
    fn hasMask(&self) -> bool;
    fn getOption(&self, name: @wxString) -> @wxString;
    fn getOptionInt(&self, name: @wxString) -> bool;
    fn hasOption(&self, name: @wxString) -> bool;
    fn initialize(&self, width: c_int, height: c_int);
    fn initializeFromData(&self, width: c_int, height: c_int, data: *c_void);
    fn loadFile(&self, name: @wxString, type_: int) -> bool;
    fn mirror(&self, horizontally: c_int, @wxImage);
    fn isOk(&self) -> bool;
    fn paste(&self, image: @wxImage, x: c_int, y: c_int);
    fn replace(&self, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8);
    fn rescale(&self, width: c_int, height: c_int);
    fn rotate(&self, angle: c_double, c_x: c_int, c_y: c_int, interpolating: c_int, offset_after_rotation: *c_void, @wxImage);
    fn rotate90(&self, clockwise: c_int, @wxImage);
    fn saveFile(&self, name: @wxString, type_: int) -> bool;
    fn scale(&self, width: c_int, height: c_int, @wxImage);
    fn setData(&self, data: *c_void);
    fn setDataAndSize(&self, data: *c_void, new_width: c_int, new_height: c_int);
    fn setMask(&self, mask: int);
    fn setMaskColour(&self, r: u8, g: u8, b: u8);
    fn setOption(&self, name: @wxString, value: @wxString);
    fn setOptionInt(&self, name: @wxString, value: int);
    fn setRGB(&self, x: c_int, y: c_int, r: u8, g: u8, b: u8);
    fn newFromDataEx(width: c_int, height: c_int, data: *c_void, isStaticData: c_int) -> @wxImage;
    fn delete(&self);
}
trait wxImageHandler {
}
trait wxImageList {
    fn addBitmap(&self, bitmap: @wxBitmap, mask: @wxBitmap) -> int;
    fn addIcon(&self, icon: @wxIcon) -> int;
    fn addMasked(&self, bitmap: @wxBitmap, maskColour: @wxColour) -> int;
    fn new(width: c_int, height: c_int, mask: c_int, initialCount: int) -> @wxImageList;
    fn delete(&self);
    fn draw(&self, index: int, dc: @wxDC, x: c_int, y: c_int, flags: int, solidBackground: bool) -> bool;
    fn getImageCount(&self) -> int;
    fn getSize(&self, index: int, width: *c_int, height: *c_int);
    fn remove(&self, index: int) -> bool;
    fn removeAll(&self) -> bool;
    fn replace(&self, index: int, bitmap: @wxBitmap, mask: @wxBitmap) -> bool;
    fn replaceIcon(&self, index: int, icon: @wxIcon) -> bool;
}
trait wxIndividualLayoutConstraint {
    fn above(&self, sibling: @wxWindow, marg: int);
    fn absolute(&self, val: int);
    fn asIs(&self);
    fn below(&self, sibling: @wxWindow, marg: int);
    fn getDone(&self) -> bool;
    fn getEdge(&self, which: int, thisWin: *c_void, other: *c_void) -> int;
    fn getMargin(&self) -> int;
    fn getMyEdge(&self) -> int;
    fn getOtherEdge(&self) -> int;
    fn getOtherWindow(&self);
    fn getPercent(&self) -> int;
    fn getRelationship(&self) -> int;
    fn getValue(&self) -> int;
    fn leftOf(&self, sibling: @wxWindow, marg: int);
    fn percentOf(&self, otherW: @wxWindow, wh: int, per: int);
    fn resetIfWin(&self, otherW: @wxWindow) -> bool;
    fn rightOf(&self, sibling: @wxWindow, marg: int);
    fn sameAs(&self, otherW: @wxWindow, edge: int, marg: int);
    fn satisfyConstraint(&self, constraints: *c_void, win: @wxWindow) -> bool;
    fn set(&self, rel: int, otherW: @wxWindow, otherE: int, val: int, marg: int);
    fn setDone(&self, d: bool);
    fn setEdge(&self, which: int);
    fn setMargin(&self, m: int);
    fn setRelationship(&self, r: int);
    fn setValue(&self, v: int);
    fn unconstrained(&self);
}
trait wxInitDialogEvent {
}
trait wxInputStream {
    fn delete(&self);
    fn eof(&self) -> bool;
    fn getC(&self) -> wchar_t;
    fn lastRead(&self) -> int;
    fn peek(&self) -> wchar_t;
    fn read(&self, buffer: *c_void, size: int);
    fn seekI(&self, pos: int, mode: int) -> int;
    fn tell(&self) -> int;
    fn ungetBuffer(&self, buffer: *c_void, size: int) -> int;
    fn ungetch(&self, c: wchar_t) -> int;
    fn canRead(&self) -> bool;
}
trait wxJoystick {
    fn new(joystick: int) -> @wxJoystick;
    fn delete(&self);
    fn getButtonState(&self) -> int;
    fn getManufacturerId(&self) -> int;
    fn getMaxAxes(&self) -> int;
    fn getMaxButtons(&self) -> int;
    fn getMovementThreshold(&self) -> int;
    fn getNumberAxes(&self) -> int;
    fn getNumberButtons(&self) -> int;
    fn getNumberJoysticks(&self) -> int;
    fn getPOVCTSPosition(&self) -> int;
    fn getPOVPosition(&self) -> int;
    fn getPollingMax(&self) -> int;
    fn getPollingMin(&self) -> int;
    fn getPosition(&self) -> @wxPoint;
    fn getProductId(&self) -> int;
    fn getProductName(&self) -> @wxString;
    fn getRudderMax(&self) -> int;
    fn getRudderMin(&self) -> int;
    fn getRudderPosition(&self) -> int;
    fn getUMax(&self) -> int;
    fn getUMin(&self) -> int;
    fn getUPosition(&self) -> int;
    fn getVMax(&self) -> int;
    fn getVMin(&self) -> int;
    fn getVPosition(&self) -> int;
    fn getXMax(&self) -> int;
    fn getXMin(&self) -> int;
    fn getYMax(&self) -> int;
    fn getYMin(&self) -> int;
    fn getZMax(&self) -> int;
    fn getZMin(&self) -> int;
    fn getZPosition(&self) -> int;
    fn hasPOV(&self) -> bool;
    fn hasPOV4Dir(&self) -> bool;
    fn hasPOVCTS(&self) -> bool;
    fn hasRudder(&self) -> bool;
    fn hasU(&self) -> bool;
    fn hasV(&self) -> bool;
    fn hasZ(&self) -> bool;
    fn isOk(&self) -> bool;
    fn releaseCapture(&self) -> int;
    fn setCapture(&self, win: @wxWindow, pollingFreq: int) -> int;
    fn setMovementThreshold(&self, threshold: int);
}
trait wxJoystickEvent {
    fn buttonDown(&self, but: int) -> bool;
    fn buttonIsDown(&self, but: int) -> bool;
    fn buttonUp(&self, but: int) -> bool;
    fn copyObject(&self, obj: *c_void);
    fn getButtonChange(&self) -> int;
    fn getButtonState(&self) -> int;
    fn getJoystick(&self) -> int;
    fn getPosition(&self) -> @wxPoint;
    fn getZPosition(&self) -> int;
    fn isButton(&self) -> bool;
    fn isMove(&self) -> bool;
    fn isZMove(&self) -> bool;
    fn setButtonChange(&self, change: int);
    fn setButtonState(&self, state: int);
    fn setJoystick(&self, stick: int);
    fn setPosition(&self, x: c_int, y: c_int);
    fn setZPosition(&self, zPos: int);
}
trait wxKeyEvent {
    fn altDown(&self) -> bool;
    fn controlDown(&self) -> bool;
    fn copyObject(&self, obj: *c_void);
    fn getKeyCode(&self) -> int;
    fn getPosition(&self) -> @wxPoint;
    fn getX(&self) -> int;
    fn getY(&self) -> int;
    fn getModifiers(&self) -> int;
    fn hasModifiers(&self) -> bool;
    fn metaDown(&self) -> bool;
    fn setKeyCode(&self, code: int);
    fn shiftDown(&self) -> bool;
}
trait wxLEDNumberCtrl {
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxLEDNumberCtrl;
    fn getAlignment(&self) -> int;
    fn getDrawFaded(&self) -> int;
    fn getValue(&self, _ref: *c_void) -> int;
    fn setAlignment(&self, Alignment: int, Redraw: int);
    fn setDrawFaded(&self, DrawFaded: int, Redraw: int);
    fn setValue(&self, Value: *c_void, Redraw: int);
}
trait wxLayoutAlgorithm {
    fn new() -> @wxLayoutAlgorithm;
    fn delete(&self);
    fn layoutFrame(&self, frame: @wxFrame, mainWindow: *c_void) -> bool;
    fn layoutMDIFrame(&self, frame: @wxFrame, x: c_int, y: c_int, w: c_int, h: c_int, use_: int) -> bool;
    fn layoutWindow(&self, frame: @wxFrame, mainWindow: *c_void) -> bool;
}
trait wxLayoutConstraints {
    fn new() -> @wxLayoutConstraints;
    fn bottom(&self);
    fn centreX(&self);
    fn centreY(&self);
    fn height(&self);
    fn left(&self);
    fn right(&self);
    fn top(&self);
    fn width(&self);
}
trait wxList {
}
trait wxListBox {
    fn append(&self, item: @wxString);
    fn appendData(&self, item: @wxString, data: *c_void);
    fn clear(&self);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, str: *wchar_t, _stl: int) -> @wxListBox;
    fn delete(&self, n: int);
    fn findString(&self, s: @wxString) -> int;
    fn getClientData(&self, n: int) -> @wxClientData;
    fn getCount(&self) -> int;
    fn getSelection(&self) -> int;
    fn getSelections(&self, aSelections: *int, allocated: int) -> int;
    fn getString(&self, n: int) -> @wxString;
    fn insertItems(&self, items: *c_void, pos: int, count: int);
    fn isSelected(&self, n: int) -> bool;
    fn setClientData(&self, n: int, clientData: @wxClientData);
    fn setFirstItem(&self, n: int);
    fn setSelection(&self, n: int, select: c_int);
    fn setString(&self, n: int, s: @wxString);
    fn setStringSelection(&self, str: @wxString, sel: bool);
}
trait wxListCtrl {
    fn arrange(&self, flag: int) -> bool;
    fn clearAll(&self);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxListCtrl;
    fn deleteAllColumns(&self) -> bool;
    fn deleteAllItems(&self) -> bool;
    fn deleteColumn(&self, col: int) -> bool;
    fn deleteItem(&self, item: int) -> bool;
    fn editLabel(&self, item: int);
    fn endEditLabel(&self, cancel: int) -> bool;
    fn ensureVisible(&self, item: int) -> bool;
    fn findItem(&self, start: int, str: @wxString, partial: bool) -> int;
    fn findItemByData(&self, start: int, data: int) -> int;
    fn findItemByPosition(&self, start: int, x: c_int, y: c_int, direction: int) -> int;
    fn getColumn(&self, col: int, item: @wxListItem) -> bool;
    fn getColumnCount(&self) -> int;
    fn getColumnWidth(&self, col: int) -> int;
    fn getCountPerPage(&self) -> int;
    fn getEditControl(&self) -> @wxTextCtrl;
    fn getImageList(&self, which: int) -> @wxImageList;
    fn getItem(&self, info: @wxListItem) -> bool;
    fn getItemCount(&self) -> int;
    fn getItemData(&self, item: int) -> int;
    fn getItemFont(&self, item: c_long) -> @wxFont;
    fn getItemPosition(&self, item: int) -> @wxPoint;
    fn getItemRect(&self, item: int, code: int) -> @wxRect;
    fn getItemSpacing(&self, isSmall: bool) -> @wxSize;
    fn getItemState(&self, item: int, stateMask: int) -> int;
    fn getItemText(&self, item: int) -> @wxString;
    fn getNextItem(&self, item: int, geometry: int, state: int) -> int;
    fn getSelectedItemCount(&self) -> int;
    fn getTextColour(&self, @wxColour);
    fn getTopItem(&self) -> int;
    fn hitTest(&self, x: c_int, y: c_int, flags: *c_void) -> int;
    fn insertColumn(&self, col: int, heading: @wxString, format: int, width: int) -> int;
    fn insertColumnFromInfo(&self, col: int, info: @wxListItem) -> int;
    fn insertItem(&self, info: @wxListItem) -> int;
    fn insertItemWithData(&self, index: int, label: @wxString) -> int;
    fn insertItemWithImage(&self, index: int, imageIndex: int) -> int;
    fn insertItemWithLabel(&self, index: int, label: @wxString, imageIndex: int) -> int;
    fn isVirtual(&self) -> bool;
    fn refreshItem(&self, item: c_long);
    fn scrollList(&self, dx: c_int, dy: c_int) -> bool;
    fn setBackgroundColour(&self, col: @wxColour);
    fn setColumn(&self, col: int, item: @wxListItem) -> bool;
    fn setColumnWidth(&self, col: int, width: int) -> bool;
    fn setForegroundColour(&self, col: @wxColour) -> int;
    fn setImageList(&self, imageList: @wxImageList, which: int);
    fn setItem(&self, index: int, col: int, label: @wxString, imageId: int) -> bool;
    fn setItemData(&self, item: int, data: int) -> bool;
    fn setItemFromInfo(&self, info: @wxListItem) -> bool;
    fn setItemImage(&self, item: int, image: int, selImage: int) -> bool;
    fn setItemPosition(&self, item: int, x: c_int, y: c_int) -> bool;
    fn setItemState(&self, item: int, state: int, stateMask: int) -> bool;
    fn setItemText(&self, item: int, str: @wxString);
    fn setSingleStyle(&self, style: int, add: bool);
    fn setTextColour(&self, col: @wxColour);
    fn setWindowStyleFlag(&self, style: int);
    fn sortItems(&self, fn_: *c_void, eif_obj: *c_void) -> bool;
    fn updateStyle(&self);
    fn assignImageList(&self, images: @wxImageList, which: int);
    fn getColumn2(&self, col: int, @wxListItem);
    fn getItem2(&self, @wxListItem);
    fn getItemPosition2(&self, item: int) -> @wxPoint;
    fn sortItems2(&self, closure: @wxClosure) -> bool;
}
trait wxListEvent {
    fn cancelled(&self) -> bool;
    fn getCode(&self) -> int;
    fn getColumn(&self) -> int;
    fn getData(&self) -> int;
    fn getImage(&self) -> int;
    fn getIndex(&self) -> int;
    fn getItem(&self, @wxListItem);
    fn getLabel(&self) -> @wxString;
    fn getMask(&self) -> int;
    fn getPoint(&self) -> @wxPoint;
    fn getText(&self) -> @wxString;
    fn getCacheFrom(&self) -> int;
    fn getCacheTo(&self) -> int;
}
trait wxListItem {
    fn clear(&self);
    fn clearAttributes(&self);
    fn new() -> @wxListItem;
    fn delete(&self);
    fn getAlign(&self) -> int;
    fn getAttributes(&self);
    fn getBackgroundColour(&self, @wxColour);
    fn getColumn(&self) -> int;
    fn getData(&self) -> int;
    fn getFont(&self, @wxFont);
    fn getId(&self) -> int;
    fn getImage(&self) -> int;
    fn getMask(&self) -> int;
    fn getState(&self) -> int;
    fn getText(&self) -> @wxString;
    fn getTextColour(&self, @wxColour);
    fn getWidth(&self) -> int;
    fn hasAttributes(&self) -> bool;
    fn setAlign(&self, align: int);
    fn setBackgroundColour(&self, colBack: @wxColour);
    fn setColumn(&self, col: int);
    fn setData(&self, data: int);
    fn setDataPointer(&self, data: *c_void);
    fn setFont(&self, font: @wxFont);
    fn setId(&self, id: int);
    fn setImage(&self, image: int);
    fn setMask(&self, mask: int);
    fn setState(&self, state: int);
    fn setStateMask(&self, stateMask: int);
    fn setText(&self, text: @wxString);
    fn setTextColour(&self, colText: @wxColour);
    fn setWidth(&self, width: int);
}
trait wxLocale {
    fn addCatalog(&self, szDomain: *c_void) -> int;
    fn addCatalogLookupPathPrefix(&self, prefix: *c_void);
    fn new(_name: int, _flags: int) -> @wxLocale;
    fn delete(&self);
    fn getLocale(&self) -> @wxLocale;
    fn getName(&self) -> @wxString;
    fn getString(&self, szOrigString: *c_void, szDomain: *c_void) -> *wchar_t;
    fn isLoaded(&self, szDomain: *c_void) -> bool;
    fn isOk(&self) -> bool;
}
trait wxLog {
    fn addTraceMask(&self, str: @wxString);
    fn delete(&self);
    fn dontCreateOnDemand(&self);
    fn flush(&self);
    fn flushActive(&self);
    fn getActiveTarget() -> @wxLog;
    fn getTimestamp(&self) -> *char;
    fn getTraceMask(&self) -> int;
    fn getVerbose(&self) -> int;
    fn hasPendingMessages(&self) -> bool;
    fn isAllowedTraceMask(&self, mask: @wxMask) -> bool;
    fn onLog(&self, level: int, szString: *wchar_t, t: int);
    fn removeTraceMask(&self, str: @wxString);
    fn resume(&self);
    fn setActiveTarget(&self) -> @wxLog;
    fn setTimestamp(&self, ts: *wchar_t);
    fn setTraceMask(&self, ulMask: int);
    fn setVerbose(&self, bVerbose: c_int);
    fn suspend(&self);
}
trait wxLogChain {
    fn new(logger: @wxLog) -> @wxLogChain;
    fn delete(&self);
    fn getOldLog(&self) -> @wxLog;
    fn isPassingMessages(&self) -> bool;
    fn passMessages(&self, bDoPass: bool);
    fn setLog(&self, logger: @wxLog);
}
trait wxLogGUI {
}
trait wxLogNull {
    fn new() -> @wxLogNull;
}
trait wxLogPassThrough {
}
trait wxLogStderr {
    fn new() -> @wxLogStderr;
    fn newStdOut() -> @wxLogStderr;
}
trait wxLogStream {
}
trait wxLogTextCtrl {
    fn new(text: @wxTextCtrl) -> @wxLogTextCtrl;
}
trait wxLogWindow {
    fn new(parent: @wxWindow, title: *wchar_t, showit: bool, passthrough: bool) -> @wxLogWindow;
    fn getFrame(&self) -> @wxFrame;
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
    fn activate(&self);
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxMDIChildFrame;
}
trait wxMDIClientWindow {
}
trait wxMDIParentFrame {
    fn activateNext(&self);
    fn activatePrevious(&self);
    fn arrangeIcons(&self);
    fn cascade(&self);
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxMDIParentFrame;
    fn getActiveChild(&self) -> @wxMDIChildFrame;
    fn getClientWindow(&self) -> @wxMDIClientWindow;
    fn getWindowMenu(&self) -> @wxMenu;
    fn onCreateClient(&self) -> @wxMDIClientWindow;
    fn setWindowMenu(&self, menu: @wxMenu);
    fn tile(&self);
}
trait wxMask {
    fn new(bitmap: @wxBitmap) -> @wxMask;
    fn newColoured(bitmap: @wxBitmap, colour: @wxColour);
}
trait wxMaximizeEvent {
}
trait wxMemoryDC {
    fn new() -> @wxMemoryDC;
    fn newCompatible(dc: @wxDC) -> @wxMemoryDC;
    fn newWithBitmap(bitmap: @wxBitmap) -> @wxMemoryDC;
    fn delete(&self);
    fn selectObject(&self, bitmap: @wxBitmap);
}
trait wxMemoryFSHandler {
}
trait wxMemoryInputStream {
}
trait wxMemoryOutputStream {
}
trait wxMenu {
    fn append(&self, id: int, text: @wxString, help: @wxString, isCheckable: bool);
    fn appendItem(&self, _itm: @wxMenuItem);
    fn appendSeparator(&self);
    fn appendSub(&self, id: int, text: @wxString, submenu: @wxMenu, help: @wxString);
    fn break_(&self);
    fn check(&self, id: int, check: bool);
    fn new(title: @wxString, style: c_long) -> @wxMenu;
    fn deleteById(&self, id: int);
    fn deleteByItem(&self, _itm: @wxMenuItem);
    fn deletePointer(&self);
    fn destroyById(&self, id: int);
    fn destroyByItem(&self, _itm: @wxMenuItem);
    fn enable(&self, id: int, enable: bool);
    fn findItem(&self, id: int) -> @wxMenuItem;
    fn findItemByLabel(&self, itemString: @wxString) -> int;
    fn getClientData(&self) -> @wxClientData;
    fn getHelpString(&self, id: int) -> @wxString;
    fn getInvokingWindow(&self) -> @wxWindow;
    fn getLabel(&self, id: int) -> @wxString;
    fn getMenuItemCount(&self) -> size_t;
    fn getMenuItems(&self, _lst: @wxList) -> int;
    fn getParent(&self) -> @wxMenu;
    fn getStyle(&self) -> int;
    fn getTitle(&self) -> @wxString;
    fn insert(&self, pos: size_t, id: int, text: @wxString, help: @wxString, isCheckable: bool);
    fn insertItem(&self, pos: size_t, _itm: @wxMenuItem);
    fn insertSub(&self, pos: size_t, id: int, text: @wxString, submenu: @wxMenu, help: @wxString);
    fn isAttached(&self) -> bool;
    fn isChecked(&self, id: int) -> bool;
    fn isEnabled(&self, id: int) -> bool;
    fn prepend(&self, id: int, text: @wxString, help: @wxString, isCheckable: bool);
    fn prependItem(&self, _itm: @wxMenuItem);
    fn prependSub(&self, id: int, text: @wxString, submenu: @wxMenu, help: @wxString);
    fn removeById(&self, id: int, _itm: @wxMenuItem);
    fn removeByItem(&self, item: *c_void);
    fn setClientData(&self, clientData: @wxClientData);
    fn setEventHandler(&self, handler: @wxEvtHandler);
    fn setHelpString(&self, id: int, helpString: @wxString);
    fn setInvokingWindow(&self, win: @wxWindow);
    fn setLabel(&self, id: int, label: @wxString);
    fn setParent(&self, parent: @wxWindow);
    fn setTitle(&self, title: @wxString);
    fn updateUI(&self, source: *c_void);
    fn getMenuBar(&self) -> @wxMenuBar;
    fn appendRadioItem(&self, id: int, text: @wxString, help: @wxString);
}
trait wxMenuBar {
    fn append(&self, menu: @wxMenu, title: @wxString) -> int;
    fn check(&self, id: int, check: bool);
    fn new(_style: int) -> @wxMenuBar;
    fn deletePointer(&self);
    fn enable(&self, enable: bool) -> int;
    fn enableItem(&self, id: int, enable: bool);
    fn enableTop(&self, pos: int, enable: bool);
    fn findItem(&self, id: int) -> @wxMenuItem;
    fn findMenu(&self, title: @wxString) -> int;
    fn findMenuItem(&self, menuString: @wxString, itemString: @wxString) -> int;
    fn getHelpString(&self, id: int) -> @wxString;
    fn getLabel(&self, id: int) -> @wxString;
    fn getLabelTop(&self, pos: int) -> @wxString;
    fn getMenu(&self, pos: int) -> @wxMenu;
    fn getMenuCount(&self) -> int;
    fn insert(&self, pos: int, menu: @wxMenu, title: @wxString) -> int;
    fn isChecked(&self, id: int) -> bool;
    fn isEnabled(&self, id: int) -> bool;
    fn remove(&self, pos: int) -> @wxMenu;
    fn replace(&self, pos: int, menu: @wxMenu, title: @wxString) -> @wxMenu;
    fn setHelpString(&self, id: int, helpString: @wxString);
    fn setItemLabel(&self, id: int, label: @wxString);
    fn setLabel(&self, s: @wxString);
    fn setLabelTop(&self, pos: int, label: @wxString);
    fn getFrame(&self) -> @wxFrame;
}
trait wxMenuEvent {
    fn copyObject(&self, obj: *c_void);
    fn getMenuId(&self) -> int;
}
trait wxMenuItem {
    fn check(&self, check: bool);
    fn new() -> @wxMenuItem;
    fn delete(&self);
    fn enable(&self, enable: bool);
    fn getHelp(&self) -> @wxString;
    fn getId(&self) -> int;
    fn getLabel(&self) -> @wxString;
    fn getLabelFromText(text: *wchar_t) -> @wxString;
    fn getMenu(&self) -> @wxMenu;
    fn getSubMenu(&self) -> @wxMenu;
    fn getText(&self) -> @wxString;
    fn isCheckable(&self) -> bool;
    fn isChecked(&self) -> bool;
    fn isEnabled(&self) -> bool;
    fn isSeparator(&self) -> bool;
    fn isSubMenu(&self) -> bool;
    fn setCheckable(&self, checkable: bool);
    fn setHelp(&self, str: @wxString);
    fn setId(&self, id: int);
    fn setSubMenu(&self, menu: @wxMenu);
    fn setText(&self, str: @wxString);
    fn newSeparator() -> @wxMenuItem;
    fn newEx(id: int, label: @wxString, help: @wxString, itemkind: int, submenu: @wxMenu) -> @wxMenuItem;
}
trait wxMessageDialog {
    fn new(_prt: @wxWindow, _msg: @wxString, _cap: @wxString, _stl: int) -> @wxMessageDialog;
    fn delete(&self);
    fn showModal(&self) -> int;
}
trait wxMetafile {
    fn new(_file: @wxString) -> @wxMetafile;
    fn delete(&self);
    fn isOk(&self) -> bool;
    fn play(&self, _dc: @wxDC) -> bool;
    fn setClipboard(&self, width: c_int, height: c_int) -> bool;
}
trait wxMetafileDC {
    fn close(&self);
    fn new(_file: @wxString) -> @wxMetafileDC;
    fn delete(&self);
}
trait wxMimeTypesManager {
    fn addFallbacks(&self, _types: *c_void);
    fn new() -> @wxMimeTypesManager;
    fn enumAllFileTypes(&self, _lst: @wxList) -> int;
    fn getFileTypeFromExtension(&self, _ext: @wxString) -> @wxFileType;
    fn getFileTypeFromMimeType(&self, _name: @wxString) -> @wxFileType;
    fn isOfType(&self, _type: @wxString, _wildcard: @wxString) -> bool;
}
trait wxMiniFrame {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxMiniFrame;
}
trait wxMirrorDC {
    fn new(dc: @wxDC) -> @wxMirrorDC;
    fn delete(&self);
}
trait wxModule {
}
trait wxMouseCaptureChangedEvent {
}
trait wxMouseEvent {
    fn altDown(&self) -> bool;
    fn button(&self, but: int) -> bool;
    fn buttonDClick(&self, but: int) -> bool;
    fn buttonDown(&self, but: int) -> bool;
    fn buttonIsDown(&self, but: int) -> bool;
    fn buttonUp(&self, but: int) -> bool;
    fn controlDown(&self) -> bool;
    fn copyObject(&self, object_dest: *c_void);
    fn dragging(&self) -> bool;
    fn entering(&self) -> bool;
    fn getLogicalPosition(&self, dc: @wxDC) -> @wxPoint;
    fn getPosition(&self) -> @wxPoint;
    fn getX(&self) -> int;
    fn getY(&self) -> int;
    fn isButton(&self) -> bool;
    fn leaving(&self) -> bool;
    fn leftDClick(&self) -> bool;
    fn leftDown(&self) -> bool;
    fn leftIsDown(&self) -> bool;
    fn leftUp(&self) -> bool;
    fn metaDown(&self) -> bool;
    fn middleDClick(&self) -> bool;
    fn middleDown(&self) -> bool;
    fn middleIsDown(&self) -> bool;
    fn middleUp(&self) -> bool;
    fn moving(&self) -> bool;
    fn rightDClick(&self) -> bool;
    fn rightDown(&self) -> bool;
    fn rightIsDown(&self) -> bool;
    fn rightUp(&self) -> bool;
    fn shiftDown(&self) -> bool;
    fn getWheelDelta(&self) -> int;
    fn getWheelRotation(&self) -> int;
    fn getButton(&self) -> int;
}
trait wxMoveEvent {
    fn copyObject(&self, obj: *c_void);
    fn getPosition(&self) -> @wxPoint;
}
trait wxMultiCellCanvas {
    fn add(&self, win: @wxWindow, row: int, col: int);
    fn calculateConstraints(&self);
    fn new(parent: @wxWindow, numRows: int, numCols: int) -> @wxMultiCellCanvas;
    fn maxCols(&self) -> int;
    fn maxRows(&self) -> int;
    fn setMinCellSize(&self, w: c_int, h: c_int);
}
trait wxMultiCellItemHandle {
    fn new(row: int, column: int, height: int, width: int, sx: int, sy: int, style: int, wx: int, wy: int, align: int) -> @wxMultiCellItemHandle;
    fn newWithSize(&self, row: int, column: int, sx: int, sy: int, style: int, wx: int, wy: int, align: int);
    fn newWithStyle(&self, row: int, column: int, style: int, wx: int, wy: int, align: int);
    fn getAlignment(&self) -> int;
    fn getColumn(&self) -> int;
    fn getHeight(&self) -> int;
    fn getLocalSize(&self, _w: *c_int, _h: *c_int);
    fn getRow(&self) -> int;
    fn getStyle(&self) -> int;
    fn getWeight(&self, _w: *c_int, _h: *c_int);
    fn getWidth(&self) -> int;
}
trait wxMultiCellSizer {
    fn calcMin(&self, _w: *c_int, _h: *c_int);
    fn new(rows: int, cols: int) -> @wxMultiCellSizer;
    fn delete(&self);
    fn enableGridLines(&self, win: @wxWindow) -> int;
    fn recalcSizes(&self);
    fn setColumnWidth(&self, column: int, colSize: int, expandable: int) -> int;
    fn setDefaultCellSize(&self, w: c_int, h: c_int) -> int;
    fn setGridPen(&self, pen: @wxPen) -> int;
    fn setRowHeight(&self, row: int, rowSize: int, expandable: int) -> int;
}
trait wxMutex {
    fn new() -> @wxMutex;
    fn delete(&self);
    fn isLocked(&self) -> bool;
    fn lock(&self) -> int;
    fn tryLock(&self) -> int;
    fn unlock(&self) -> int;
}
trait wxMutexLocker {
}
trait wxNavigationKeyEvent {
    fn getCurrentFocus(&self);
    fn getDirection(&self) -> bool;
    fn isWindowChange(&self) -> bool;
    fn setCurrentFocus(&self, win: @wxWindow);
    fn setDirection(&self, bForward: bool);
    fn setWindowChange(&self, bIs: bool);
    fn shouldPropagate(&self) -> int;
}
trait wxNewBitmapButton {
    fn new(labelBitmap: *c_void, labelText: *c_void, alignText: int, isFlat: bool, firedEventType: int, marginX: int, marginY: int, textToLabelGap: int, isSticky: bool) -> @wxNewBitmapButton;
    fn newFromFile(&self, bitmapFileType: int, labelText: *c_void, alignText: int, isFlat: bool, firedEventType: int, marginX: int, marginY: int, textToLabelGap: int, isSticky: bool) -> @wxNewBitmapButton;
    fn delete(&self);
    fn drawDecorations(&self, dc: @wxDC);
    fn drawLabel(&self, dc: @wxDC);
    fn enable(&self, enable: bool) -> int;
    fn realize(&self, _prt: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int);
    fn renderAllLabelImages(&self);
    fn renderLabelImage(&self, destBmp: *c_void, srcBmp: *c_void, isEnabled: bool, isPressed: bool);
    fn renderLabelImages(&self);
    fn reshape(&self);
    fn setAlignments(&self, alignText: int, marginX: int, marginY: int, textToLabelGap: int);
    fn setLabel(&self, labelBitmap: *c_void, labelText: *c_void);
}
trait wxNodeBase {
}
trait wxNotebook {
    fn addPage(&self, pPage: @wxWindow, strText: @wxString, bSelect: bool, imageId: int) -> bool;
    fn advanceSelection(&self, bForward: bool);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxNotebook;
    fn deleteAllPages(&self) -> bool;
    fn deletePage(&self, nPage: int) -> bool;
    fn getImageList(&self) -> @wxImageList;
    fn getPage(&self, nPage: int) -> @wxWindow;
    fn getPageCount(&self) -> int;
    fn getPageImage(&self, nPage: int) -> int;
    fn getPageText(&self, nPage: int) -> @wxString;
    fn getRowCount(&self) -> int;
    fn getSelection(&self) -> int;
    fn hitTest(&self, x: c_int, y: c_int, flags: *c_long) -> int;
    fn insertPage(&self, nPage: int, pPage: @wxWindow, strText: @wxString, bSelect: bool, imageId: int) -> bool;
    fn removePage(&self, nPage: int) -> bool;
    fn setImageList(&self, imageList: @wxImageList);
    fn setPadding(&self, _w: c_int, _h: c_int);
    fn setPageImage(&self, nPage: int, nImage: int) -> bool;
    fn setPageSize(&self, _w: c_int, _h: c_int);
    fn setPageText(&self, nPage: int, strText: @wxString) -> bool;
    fn setSelection(&self, nPage: int) -> int;
    fn assignImageList(&self, imageList: @wxImageList);
}
trait wxNotebookEvent {
}
trait wxNotifyEvent {
    fn allow(&self);
    fn copyObject(&self, object_dest: *c_void);
    fn isAllowed(&self) -> bool;
    fn veto(&self);
}
trait wxObject {
    fn safeDelete(&self);
    fn getClientClosure(&self) -> @wxClosure;
    fn setClientClosure(&self, closure: @wxClosure);
    fn delete(&self);
    fn getClassInfo(&self) -> @wxClassInfo;
    fn isKindOf(&self, classInfo: @wxClassInfo) -> bool;
    fn isScrolledWindow(&self) -> bool;
}
trait wxObjectRefData {
}
trait wxOutputStream {
    fn delete(&self);
    fn lastWrite(&self) -> int;
    fn putC(&self, c: wchar_t);
    fn seek(&self, pos: int, mode: int) -> int;
    fn sync(&self);
    fn tell(&self) -> int;
    fn write(&self, buffer: *c_void, size: int);
}
trait wxPageSetupDialog {
    fn new(parent: @wxWindow, data: @wxPageSetupDialogData) -> @wxPageSetupDialog;
    fn getPageSetupData(&self, @wxPageSetupDialogData);
}
trait wxPageSetupDialogData {
    fn assign(&self, @wxPageSetupDialogData);
    fn assignData(&self, printData: @wxPrintData);
    fn calculateIdFromPaperSize(&self);
    fn calculatePaperSizeFromId(&self);
    fn new() -> @wxPageSetupDialogData;
    fn newFromData(printData: @wxPrintData) -> @wxPageSetupDialogData;
    fn delete(&self);
    fn enableHelp(&self, flag: bool);
    fn enableMargins(&self, flag: bool);
    fn enableOrientation(&self, flag: bool);
    fn enablePaper(&self, flag: bool);
    fn enablePrinter(&self, flag: bool);
    fn getDefaultInfo(&self) -> bool;
    fn getDefaultMinMargins(&self) -> bool;
    fn getEnableHelp(&self) -> bool;
    fn getEnableMargins(&self) -> bool;
    fn getEnableOrientation(&self) -> bool;
    fn getEnablePaper(&self) -> bool;
    fn getEnablePrinter(&self) -> bool;
    fn getMarginBottomRight(&self) -> @wxPoint;
    fn getMarginTopLeft(&self) -> @wxPoint;
    fn getMinMarginBottomRight(&self) -> @wxPoint;
    fn getMinMarginTopLeft(&self) -> @wxPoint;
    fn getPaperId(&self) -> int;
    fn getPaperSize(&self) -> @wxSize;
    fn getPrintData(&self, @wxPrintData);
    fn setDefaultInfo(&self, flag: bool);
    fn setDefaultMinMargins(&self, flag: int);
    fn setMarginBottomRight(&self, x: c_int, y: c_int);
    fn setMarginTopLeft(&self, x: c_int, y: c_int);
    fn setMinMarginBottomRight(&self, x: c_int, y: c_int);
    fn setMinMarginTopLeft(&self, x: c_int, y: c_int);
    fn setPaperId(&self, id: *c_void);
    fn setPaperSize(&self, w: c_int, h: c_int);
    fn setPaperSizeId(&self, id: int);
    fn setPrintData(&self, printData: @wxPrintData);
}
trait wxPaintDC {
    fn new(win: @wxWindow) -> @wxPaintDC;
    fn delete(&self);
}
trait wxPaintEvent {
}
trait wxPalette {
    fn assign(&self, palette: @wxPalette);
    fn newDefault() -> @wxPalette;
    fn newRGB(n: int, red: *c_void, green: *c_void, blue: *c_void) -> @wxPalette;
    fn delete(&self);
    fn getPixel(&self, red: u8, green: u8, blue: u8) -> int;
    fn getRGB(&self, pixel: int, red: *c_void, green: *c_void, blue: *c_void) -> bool;
    fn isEqual(&self, palette: @wxPalette) -> bool;
    fn isOk(&self) -> bool;
}
trait wxPaletteChangedEvent {
    fn copyObject(&self, obj: *c_void);
    fn getChangedWindow(&self);
    fn setChangedWindow(&self, win: @wxWindow);
}
trait wxPanel {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxPanel;
    fn initDialog(&self);
    fn setFocus(&self);
}
trait wxPathList {
}
trait wxPen {
    fn assign(&self, pen: @wxPen);
    fn newDefault() -> @wxPen;
    fn newFromBitmap(stipple: @wxBitmap, width: int) -> @wxPen;
    fn newFromColour(col: @wxColour, width: int, style: int) -> @wxPen;
    fn newFromStock(id: int) -> @wxPen;
    fn delete(&self);
    fn getCap(&self) -> int;
    fn getColour(&self, @wxColour);
    fn getDashes(&self, ptr: *c_void) -> int;
    fn getJoin(&self) -> int;
    fn getStipple(&self, @wxBitmap);
    fn getStyle(&self) -> int;
    fn getWidth(&self) -> int;
    fn isEqual(&self, pen: @wxPen) -> bool;
    fn isOk(&self) -> bool;
    fn setCap(&self, cap: int);
    fn setColour(&self, col: @wxColour);
    fn setColourSingle(&self, r: wchar_t, g: wchar_t, b: wchar_t);
    fn setDashes(&self, nb_dashes: int, dash: *c_void);
    fn setJoin(&self, join: int);
    fn setStipple(&self, stipple: @wxBitmap);
    fn setStyle(&self, style: int);
    fn setWidth(&self, width: int);
    fn safeDelete(&self);
    fn isStatic(&self) -> bool;
}
trait wxPenList {
}
trait wxPlotCurve {
}
trait wxPlotEvent {
    fn getCurve(&self);
    fn getPosition(&self) -> int;
    fn getZoom(&self) -> c_double;
    fn setPosition(&self, pos: int);
    fn setZoom(&self, zoom: c_double);
}
trait wxPlotOnOffCurve {
    fn add(&self, on: int, off: int, clientData: @wxClientData);
    fn new(offsetY: int) -> @wxPlotOnOffCurve;
    fn delete(&self);
    fn drawOffLine(&self, dc: @wxDC, y: int, start: int, end: int);
    fn drawOnLine(&self, dc: @wxDC, y: int, start: int, end: int, clientData: @wxClientData);
    fn getAt(&self, index: int);
    fn getClientData(&self, index: int) -> @wxClientData;
    fn getCount(&self) -> int;
    fn getEndX(&self) -> int;
    fn getOff(&self, index: int) -> int;
    fn getOffsetY(&self) -> int;
    fn getOn(&self, index: int) -> int;
    fn getStartX(&self) -> int;
    fn setOffsetY(&self, offsetY: int);
}
trait wxPlotWindow {
    fn add(&self, curve: @wxPlotCurve);
    fn addOnOff(&self, curve: @wxPlotCurve);
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, flags: int) -> @wxPlotWindow;
    fn delete(&self, curve: @wxPlotCurve);
    fn deleteOnOff(&self, curve: @wxPlotOnOffCurve);
    fn enlarge(&self, curve: @wxPlotCurve, factor: c_double);
    fn getAt(&self, n: int) -> @wxPlotCurve;
    fn getCount(&self) -> int;
    fn getCurrent(&self) -> @wxPlotCurve;
    fn getEnlargeAroundWindowCentre(&self) -> int;
    fn getOnOffCurveAt(&self, n: int) -> @wxPlotOnOffCurve;
    fn getOnOffCurveCount(&self) -> int;
    fn getScrollOnThumbRelease(&self) -> int;
    fn getUnitsPerValue(&self) -> c_double;
    fn getZoom(&self) -> c_double;
    fn move(&self, curve: @wxPlotCurve, pixels_up: int);
    fn redrawEverything(&self);
    fn redrawXAxis(&self);
    fn redrawYAxis(&self);
    fn resetScrollbar(&self);
    fn setCurrent(&self, current: @wxPlotCurve);
    fn setEnlargeAroundWindowCentre(&self, enlargeAroundWindowCentre: int);
    fn setScrollOnThumbRelease(&self, scrollOnThumbRelease: int);
    fn setUnitsPerValue(&self, upv: c_double);
    fn setZoom(&self, zoom: c_double);
}
trait wxPoint {
    fn new(xx: c_int, yy: c_int) -> @wxPoint;
    fn destroy(&self);
    fn getX(&self) -> int;
    fn getY(&self) -> int;
    fn setX(&self, w: int);
    fn setY(&self, h: int);
}
trait wxPopupTransientWindow {
}
trait wxPopupWindow {
}
trait wxPostScriptDC {
    fn new(data: @wxPrintData) -> @wxPostScriptDC;
    fn delete(&self);
    fn setResolution(&self, ppi: int);
    fn getResolution(&self) -> int;
}
trait wxPreviewCanvas {
    fn new(preview: @wxPrintPreview, parent: @wxWindow, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxPreviewCanvas;
}
trait wxPreviewControlBar {
}
trait wxPreviewFrame {
    fn new(preview: @wxPrintPreview, parent: @wxFrame, title: @wxString, x: c_int, y: c_int, width: c_int, height: c_int, style: int, name: @wxString) -> @wxPreviewFrame;
    fn delete(&self);
    fn initialize(&self);
}
trait wxPrintData {
    fn assign(&self, data: @wxPrintData);
    fn new() -> @wxPrintData;
    fn delete(&self);
    fn getCollate(&self) -> bool;
    fn getColour(&self) -> bool;
    fn getDuplex(&self) -> int;
    fn getFilename(&self) -> @wxString;
    fn getFontMetricPath(&self) -> @wxString;
    fn getNoCopies(&self) -> int;
    fn getOrientation(&self) -> int;
    fn getPaperId(&self) -> int;
    fn getPaperSize(&self) -> @wxSize;
    fn getPreviewCommand(&self) -> @wxString;
    fn getPrintMode(&self) -> int;
    fn getPrinterCommand(&self) -> @wxString;
    fn getPrinterName(&self) -> @wxString;
    fn getPrinterOptions(&self) -> @wxString;
    fn getPrinterScaleX(&self) -> c_double;
    fn getPrinterScaleY(&self) -> c_double;
    fn getPrinterTranslateX(&self) -> int;
    fn getPrinterTranslateY(&self) -> int;
    fn getQuality(&self) -> int;
    fn setCollate(&self, flag: c_int);
    fn setColour(&self, colour: c_int);
    fn setDuplex(&self, duplex: int);
    fn setFilename(&self, filename: @wxString);
    fn setFontMetricPath(&self, path: @wxString);
    fn setNoCopies(&self, v: int);
    fn setOrientation(&self, orient: int);
    fn setPaperId(&self, sizeId: int);
    fn setPaperSize(&self, w: c_int, h: c_int);
    fn setPreviewCommand(&self, command: @wxCommand);
    fn setPrintMode(&self, printMode: int);
    fn setPrinterCommand(&self, command: @wxCommand);
    fn setPrinterName(&self, name: @wxString);
    fn setPrinterOptions(&self, options: @wxString);
    fn setPrinterScaleX(&self, x: c_double);
    fn setPrinterScaleY(&self, y: c_double);
    fn setPrinterScaling(&self, x: c_double, y: c_double);
    fn setPrinterTranslateX(&self, x: int);
    fn setPrinterTranslateY(&self, y: int);
    fn setPrinterTranslation(&self, x: c_int, y: c_int);
    fn setQuality(&self, quality: int);
}
trait wxPostScriptPrintNativeData {
    fn new() -> @wxPostScriptPrintNativeData;
    fn delete(&self);
}
trait wxPrintDialog {
    fn new(parent: @wxWindow, data: @wxPrintDialogData) -> @wxPrintDialog;
    fn getPrintDC(&self) -> @wxDC;
    fn getPrintData(&self, @wxPrintData);
    fn getPrintDialogData(&self) -> @wxPrintDialogData;
}
trait wxPrintDialogData {
    fn assign(&self, data: @wxPrintDialogData);
    fn assignData(&self, data: @wxPrintData);
    fn newDefault() -> @wxPrintDialogData;
    fn newFromData(printData: @wxPrintData) -> @wxPrintDialogData;
    fn delete(&self);
    fn enableHelp(&self, flag: bool);
    fn enablePageNumbers(&self, flag: bool);
    fn enablePrintToFile(&self, flag: bool);
    fn enableSelection(&self, flag: bool);
    fn getAllPages(&self) -> int;
    fn getCollate(&self) -> bool;
    fn getEnableHelp(&self) -> bool;
    fn getEnablePageNumbers(&self) -> bool;
    fn getEnablePrintToFile(&self) -> bool;
    fn getEnableSelection(&self) -> bool;
    fn getFromPage(&self) -> int;
    fn getMaxPage(&self) -> int;
    fn getMinPage(&self) -> int;
    fn getNoCopies(&self) -> int;
    fn getPrintData(&self, @wxPrintData);
    fn getPrintToFile(&self) -> bool;
    fn getSelection(&self) -> bool;
    fn getToPage(&self) -> int;
    fn setAllPages(&self, flag: bool);
    fn setCollate(&self, flag: bool);
    fn setFromPage(&self, v: int);
    fn setMaxPage(&self, v: int);
    fn setMinPage(&self, v: int);
    fn setNoCopies(&self, v: int);
    fn setPrintData(&self, printData: @wxPrintData);
    fn setPrintToFile(&self, flag: bool);
    fn setSelection(&self, flag: bool);
    fn setToPage(&self, v: int);
}
trait wxPrintPreview {
    fn newFromData(printout: @wxPrintout, printoutForPrinting: @wxPrintout, data: @wxPrintData) -> @wxPrintPreview;
    fn newFromDialogData(printout: @wxPrintout, printoutForPrinting: @wxPrintout, data: @wxPrintDialogData) -> @wxPrintPreview;
    fn delete(&self);
    fn determineScaling(&self);
    fn drawBlankPage(&self, canvas: @wxPreviewCanvas, dc: @wxDC) -> bool;
    fn getCanvas(&self) -> @wxPreviewCanvas;
    fn getCurrentPage(&self) -> int;
    fn getFrame(&self) -> @wxFrame;
    fn getMaxPage(&self) -> int;
    fn getMinPage(&self) -> int;
    fn getPrintDialogData(&self, @wxPrintDialogData);
    fn getPrintout(&self) -> @wxPrintout;
    fn getPrintoutForPrinting(&self) -> @wxPrintout;
    fn getZoom(&self) -> int;
    fn isOk(&self) -> bool;
    fn paintPage(&self, canvas: @wxPrintPreview, dc: @wxDC) -> bool;
    fn print(&self, interactive: bool) -> bool;
    fn renderPage(&self, pageNum: int) -> bool;
    fn setCanvas(&self, canvas: @wxPreviewCanvas);
    fn setCurrentPage(&self, pageNum: int) -> bool;
    fn setFrame(&self, frame: @wxFrame);
    fn setOk(&self, ok: bool);
    fn setPrintout(&self, printout: @wxPrintout);
    fn setZoom(&self, percent: int);
}
trait wxPrinter {
    fn new(data: @wxPrintDialogData) -> @wxPrinter;
    fn newAbortWindow(&self, parent: @wxWindow, printout: @wxPrintout) -> @wxWindow;
    fn delete(&self);
    fn getAbort(&self) -> bool;
    fn getLastError(&self) -> int;
    fn getPrintDialogData(&self, @wxPrintDialogData);
    fn print(&self, parent: @wxWindow, printout: @wxPrintout, prompt: bool) -> bool;
    fn printDialog(&self, parent: @wxWindow) -> @wxDC;
    fn reportError(&self, parent: @wxWindow, printout: @wxPrintout, message: @wxString);
    fn setup(&self, parent: @wxWindow) -> bool;
}
trait wxPrinterDC {
    fn new(data: @wxPrintData) -> @wxPrinterDC;
    fn delete(&self);
    fn getPaperRect(&self) -> @wxRect;
}
trait wxPrintout {
    fn getDC(&self) -> @wxDC;
    fn getPPIPrinter(&self, _x: *c_int, _y: *c_int);
    fn getPPIScreen(&self, _x: *c_int, _y: *c_int);
    fn getPageSizeMM(&self, _w: *c_int, _h: *c_int);
    fn getPageSizePixels(&self, _w: *c_int, _h: *c_int);
    fn getTitle(&self) -> @wxString;
    fn isPreview(&self) -> bool;
    fn setDC(&self, dc: @wxDC);
    fn setPPIPrinter(&self, x: c_int, y: c_int);
    fn setPPIScreen(&self, x: c_int, y: c_int);
    fn setPageSizeMM(&self, w: c_int, h: c_int);
    fn setPageSizePixels(&self, w: c_int, h: c_int);
}
trait wxPrivateDropTarget {
}
trait wxProcess {
    fn closeOutput(&self);
    fn newDefault(_prt: @wxWindow, _id: int) -> @wxProcess;
    fn newRedirect(_prt: @wxWindow, _rdr: bool) -> @wxProcess;
    fn delete(&self);
    fn detach(&self);
    fn getErrorStream(&self) -> @wxInputStream;
    fn getInputStream(&self) -> @wxInputStream;
    fn getOutputStream(&self) -> @wxOutputStream;
    fn isRedirected(&self) -> bool;
    fn redirect(&self);
    fn open(cmd: @wxString, flags: int) -> @wxProcess;
    fn isErrorAvailable(&self) -> bool;
    fn isInputAvailable(&self) -> bool;
    fn isInputOpened(&self) -> bool;
}
trait wxProcessEvent {
    fn getExitCode(&self) -> int;
    fn getPid(&self) -> int;
}
trait wxProgressDialog {
    fn new(title: @wxString, message: @wxString, max: int, parent: @wxWindow, style: int) -> @wxProgressDialog;
    fn update(&self, value: int) -> bool;
    fn updateWithMessage(&self, value: int, message: @wxString) -> bool;
    fn resume(&self);
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
    fn new(id: int) -> @wxQueryLayoutInfoEvent;
    fn getAlignment(&self) -> int;
    fn getFlags(&self) -> int;
    fn getOrientation(&self) -> int;
    fn getRequestedLength(&self) -> int;
    fn getSize(&self) -> @wxSize;
    fn setAlignment(&self, align: int);
    fn setFlags(&self, flags: int);
    fn setOrientation(&self, orient: int);
    fn setRequestedLength(&self, length: int);
    fn setSize(&self, w: c_int, h: c_int);
}
trait wxQueryNewPaletteEvent {
    fn copyObject(&self, obj: @wxObject);
    fn getPaletteRealized(&self) -> bool;
    fn setPaletteRealized(&self, realized: bool);
}
trait wxRadioBox {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, n: c_int, _str: *wchar_t, _dim: int, _stl: int) -> @wxRadioBox;
    fn enableItem(&self, item: int, enable: bool);
    fn findString(&self, s: @wxString) -> int;
    fn getItemLabel(&self, item: int) -> @wxString;
    fn getNumberOfRowsOrCols(&self) -> int;
    fn getSelection(&self) -> int;
    fn getStringSelection(&self) -> @wxString;
    fn number(&self) -> int;
    fn setItemBitmap(&self, item: int, bitmap: @wxBitmap);
    fn setItemLabel(&self, item: int, label: @wxString);
    fn setNumberOfRowsOrCols(&self, n: int);
    fn setSelection(&self, _n: int);
    fn setStringSelection(&self, s: @wxString);
    fn showItem(&self, item: int, show: bool);
}
trait wxRadioButton {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxRadioButton;
    fn getValue(&self) -> bool;
    fn setValue(&self, value: bool);
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
    fn assign(&self, region: @wxRegion);
    fn clear(&self);
    fn containsPoint(&self, x: c_int, y: c_int) -> bool;
    fn containsRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool;
    fn newDefault() -> @wxRegion;
    fn newFromRect(x: c_int, y: c_int, w: c_int, h: c_int) -> @wxRegion;
    fn delete(&self);
    fn isEmpty(&self) -> bool;
    fn getBox(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn intersectRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool;
    fn intersectRegion(&self, region: @wxRegion) -> bool;
    fn subtractRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool;
    fn subtractRegion(&self, region: @wxRegion) -> bool;
    fn unionRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool;
    fn unionRegion(&self, region: @wxRegion) -> bool;
    fn xorRect(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool;
    fn xorRegion(&self, region: @wxRegion) -> bool;
}
trait wxRegionIterator {
    fn new() -> @wxRegionIterator;
    fn newFromRegion(region: @wxRegion) -> @wxRegionIterator;
    fn delete(&self);
    fn getHeight(&self) -> int;
    fn getWidth(&self) -> int;
    fn getX(&self) -> int;
    fn getY(&self) -> int;
    fn haveRects(&self) -> bool;
    fn next(&self);
    fn reset(&self);
    fn resetToRegion(&self, region: @wxRegion);
}
trait wxRemotelyScrolledTreeCtrl {
    fn adjustRemoteScrollbars(&self);
    fn calcTreeSize(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn calcTreeSizeItem(&self, id: *c_void, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
    fn new(_obj: *c_void, _cmp: *c_void, parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxRemotelyScrolledTreeCtrl;
    fn delete(&self);
    fn getCompanionWindow(&self);
    fn getScrollPos(&self, orient: int) -> int;
    fn getScrolledWindow(&self) -> @wxScrolledWindow;
    fn getViewStart(&self, _x: *c_int, _y: *c_int);
    fn hideVScrollbar(&self);
    fn prepareDC(&self, dc: @wxDC);
    fn scrollToLine(&self, posHoriz: int, posVert: int);
    fn setCompanionWindow(&self, companion: *c_void);
    fn setScrollbars(&self, pixelsPerUnitX: int, pixelsPerUnitY: int, noUnitsX: int, noUnitsY: int, xPos: int, yPos: int, noRefresh: int);
}
trait wxSVGFileDC {
    fn new(fileName: @wxString) -> @wxSVGFileDC;
    fn newWithSize(fileName: @wxString, w: c_int, h: c_int) -> @wxSVGFileDC;
    fn newWithSizeAndResolution(fileName: @wxString, w: c_int, h: c_int, a_dpi: c_float) -> @wxSVGFileDC;
    fn delete(&self);
}
trait wxSashEvent {
    fn new(id: int, edge: int) -> @wxSashEvent;
    fn getDragRect(&self) -> @wxRect;
    fn getDragStatus(&self) -> int;
    fn getEdge(&self) -> int;
    fn setDragRect(&self, x: c_int, y: c_int, w: c_int, h: c_int);
    fn setDragStatus(&self, status: int);
    fn setEdge(&self, edge: int);
}
trait wxSashLayoutWindow {
    fn new(_par: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: int) -> @wxSashLayoutWindow;
    fn getAlignment(&self) -> int;
    fn getOrientation(&self) -> int;
    fn setAlignment(&self, align: int);
    fn setDefaultSize(&self, w: c_int, h: c_int);
    fn setOrientation(&self, orient: int);
}
trait wxSashWindow {
    fn new(_par: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: int) -> @wxSashWindow;
    fn getDefaultBorderSize(&self) -> int;
    fn getEdgeMargin(&self, edge: int) -> int;
    fn getExtraBorderSize(&self) -> int;
    fn getMaximumSizeX(&self) -> int;
    fn getMaximumSizeY(&self) -> int;
    fn getMinimumSizeX(&self) -> int;
    fn getMinimumSizeY(&self) -> int;
    fn getSashVisible(&self, edge: int) -> bool;
    fn hasBorder(&self, edge: int) -> bool;
    fn setDefaultBorderSize(&self, width: int);
    fn setExtraBorderSize(&self, width: int);
    fn setMaximumSizeX(&self, max: int);
    fn setMaximumSizeY(&self, max: int);
    fn setMinimumSizeX(&self, min: int);
    fn setMinimumSizeY(&self, min: int);
    fn setSashBorder(&self, edge: int, border: bool);
    fn setSashVisible(&self, edge: int, sash: bool);
}
trait wxScopedArray {
}
trait wxScopedPtr {
}
trait wxScreenDC {
    fn new() -> @wxScreenDC;
    fn delete(&self);
    fn endDrawingOnTop(&self) -> bool;
    fn startDrawingOnTop(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool;
    fn startDrawingOnTopOfWin(&self, win: @wxWindow) -> bool;
}
trait wxScrollBar {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxScrollBar;
    fn getPageSize(&self) -> int;
    fn getRange(&self) -> int;
    fn getThumbPosition(&self) -> int;
    fn getThumbSize(&self) -> int;
    fn setScrollbar(&self, position: int, thumbSize: int, range: int, pageSize: int, refresh: bool);
    fn setThumbPosition(&self, viewStart: int);
}
trait wxScrollEvent {
    fn getOrientation(&self) -> int;
    fn getPosition(&self) -> int;
}
trait wxScrollWinEvent {
    fn getOrientation(&self) -> int;
    fn getPosition(&self) -> int;
    fn setOrientation(&self, orient: int);
    fn setPosition(&self, pos: int);
}
trait wxScrolledWindow {
    fn adjustScrollbars(&self);
    fn calcScrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int);
    fn calcUnscrolledPosition(&self, x: c_int, y: c_int, xx: *c_int, yy: *c_int);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxScrolledWindow;
    fn enableScrolling(&self, x_scrolling: bool, y_scrolling: bool);
    fn getScaleX(&self) -> c_double;
    fn getScaleY(&self) -> c_double;
    fn getScrollPageSize(&self, orient: int) -> int;
    fn getScrollPixelsPerUnit(&self, _x: *c_int, _y: *c_int);
    fn getTargetWindow(&self) -> @wxWindow;
    fn getViewStart(&self, _x: *c_int, _y: *c_int);
    fn getVirtualSize(&self, _x: *c_int, _y: *c_int);
    fn onDraw(&self, dc: @wxDC);
    fn prepareDC(&self, dc: @wxDC);
    fn scroll(&self, x_pos: c_int, y_pos: c_int);
    fn setScale(&self, xs: c_double, ys: c_double);
    fn setScrollPageSize(&self, orient: int, pageSize: int);
    fn setScrollbars(&self, pixelsPerUnitX: int, pixelsPerUnitY: int, noUnitsX: int, noUnitsY: int, xPos: int, yPos: int, noRefresh: bool);
    fn showScrollbars(&self, showh: int, showv: int);
    fn setTargetWindow(&self, target: @wxWindow);
    fn viewStart(&self, _x: *c_int, _y: *c_int);
    fn setScrollRate(&self, xstep: int, ystep: int);
}
trait wxSemaphore {
}
trait wxServer {
}
trait wxServerBase {
}
trait wxSetCursorEvent {
    fn getCursor(&self) -> @wxCursor;
    fn getX(&self) -> int;
    fn getY(&self) -> int;
    fn hasCursor(&self) -> bool;
    fn setCursor(&self, cursor: @wxCursor);
}
trait wxShowEvent {
    fn copyObject(&self, obj: @wxObject);
    fn isShown(&self) -> bool;
    fn setShow(&self, show: bool);
}
trait wxSimpleHelpProvider {
    fn new() -> @wxSimpleHelpProvider;
}
trait wxSingleChoiceDialog {
}
trait wxSingleInstanceChecker {
    fn new(_obj: *c_void, name: @wxString, path: @wxString) -> bool;
    fn newDefault() -> @wxSingleInstanceChecker;
    fn delete(&self);
    fn isAnotherRunning(&self) -> bool;
}
trait wxSize {
    fn new(w: c_int, h: c_int) -> @wxSize;
    fn destroy(&self);
    fn getHeight(&self) -> int;
    fn getWidth(&self) -> int;
    fn setHeight(&self, h: int);
    fn setWidth(&self, w: int);
}
trait wxSizeEvent {
    fn copyObject(&self, obj: *c_void);
    fn getSize(&self) -> @wxSize;
}
trait wxSizer {
    fn add(&self, width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void);
    fn addSizer(&self, sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void);
    fn addWindow(&self, window: @wxWindow, option: int, flag: int, border: int, userData: *c_void);
    fn calcMin(&self) -> @wxSize;
    fn fit(&self, window: @wxWindow);
    fn getChildren(&self, _res: *c_void, _cnt: int) -> int;
    fn getMinSize(&self) -> @wxSize;
    fn getPosition(&self) -> @wxPoint;
    fn getSize(&self) -> @wxSize;
    fn insert(&self, before: int, width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void);
    fn insertSizer(&self, before: int, sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void);
    fn insertWindow(&self, before: int, window: @wxWindow, option: int, flag: int, border: int, userData: *c_void);
    fn layout(&self);
    fn prepend(&self, width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void);
    fn prependSizer(&self, sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void);
    fn prependWindow(&self, window: @wxWindow, option: int, flag: int, border: int, userData: *c_void);
    fn recalcSizes(&self);
    fn setDimension(&self, x: c_int, y: c_int, width: c_int, height: c_int);
    fn setItemMinSize(&self, pos: int, width: c_int, height: c_int);
    fn setItemMinSizeSizer(&self, sizer: @wxSizer, width: c_int, height: c_int);
    fn setItemMinSizeWindow(&self, window: @wxWindow, width: c_int, height: c_int);
    fn setMinSize(&self, width: c_int, height: c_int);
    fn setSizeHints(&self, window: @wxWindow);
    fn addSpacer(&self, size: int);
    fn addStretchSpacer(&self, size: int);
    fn clear(&self, delete_windows: bool);
    fn detachWindow(&self, window: @wxWindow) -> bool;
    fn detachSizer(&self, sizer: @wxSizer) -> bool;
    fn detach(&self, index: int) -> bool;
    fn fitInside(&self, window: @wxWindow);
    fn getContainingWindow(&self) -> @wxWindow;
    fn getItemWindow(&self, window: @wxWindow, recursive: bool) -> @wxSizerItem;
    fn getItemSizer(&self, window: @wxSizer, recursive: bool) -> @wxSizerItem;
    fn getItem(&self, index: int) -> @wxSizerItem;
    fn hideWindow(&self, window: @wxWindow) -> bool;
    fn hideSizer(&self, sizer: @wxSizer) -> bool;
    fn hide(&self, index: int) -> bool;
    fn insertSpacer(&self, index: int, size: int) -> @wxSizerItem;
    fn insertStretchSpacer(&self, index: int, prop: int) -> @wxSizerItem;
    fn isShownWindow(&self, window: *@wxWindow) -> bool;
    fn isShownSizer(&self, sizer: *@wxSizer) -> bool;
    fn isShown(&self, index: int) -> bool;
    fn prependSpacer(&self, size: int) -> @wxSizerItem;
    fn prependStretchSpacer(&self, prop: int) -> @wxSizerItem;
    fn replaceWindow(&self, oldwin: @wxWindow, newwin: @wxWindow, recursive: bool) -> bool;
    fn replaceSizer(&self, oldsz: @wxSizer, newsz: @wxSizer, recursive: bool) -> bool;
    fn replace(&self, oldindex: int, newitem: @wxSizerItem) -> bool;
    fn setVirtualSizeHints(&self, window: @wxWindow);
    fn showWindow(&self, window: @wxWindow, show: bool, recursive: bool) -> bool;
    fn showSizer(&self, sizer: @wxSizer, show: bool, recursive: bool) -> bool;
    fn show(&self, sizer: @wxSizer, index: int, show: bool) -> bool;
}
trait wxSizerItem {
    fn calcMin(&self) -> @wxSize;
    fn new(width: c_int, height: c_int, option: int, flag: int, border: int, userData: *c_void) -> @wxSizerItem;
    fn newInSizer(sizer: @wxSizer, option: int, flag: int, border: int, userData: *c_void);
    fn newInWindow(window: @wxWindow, option: int, flag: int, border: int, userData: *c_void);
    fn getBorder(&self) -> int;
    fn getFlag(&self) -> int;
    fn getMinSize(&self) -> @wxSize;
    fn getPosition(&self) -> @wxPoint;
    fn getRatio(&self) -> c_float;
    fn getSize(&self) -> @wxSize;
    fn getSizer(&self) -> @wxSizer;
    fn getUserData(&self);
    fn getWindow(&self) -> @wxWindow;
    fn isSizer(&self) -> bool;
    fn isSpacer(&self) -> bool;
    fn isWindow(&self) -> bool;
    fn setBorder(&self, border: int);
    fn setDimension(&self, _x: c_int, _y: c_int, _w: c_int, _h: c_int);
    fn setFlag(&self, flag: int);
    fn setFloatRatio(&self, ratio: c_float);
    fn setInitSize(&self, x: c_int, y: c_int);
    fn setRatio(&self, width: c_int, height: c_int);
    fn setSizer(&self, sizer: @wxSizer);
    fn setWindow(&self, window: @wxWindow);
    fn delete(&self);
    fn deleteWindows(&self);
    fn detachSizer(&self);
    fn getProportion(&self) -> int;
    fn getRect(&self) -> @wxRect;
    fn getSpacer(&self) -> @wxSize;
    fn isShown(&self) -> int;
    fn setProportion(&self, proportion: int);
    fn setSpacer(&self, width: c_int, height: c_int);
    fn show(&self, show: int);
}
trait wxSlider {
    fn clearSel(&self);
    fn clearTicks(&self);
    fn new(_prt: @wxWindow, _id: int, _init: int, _min: int, _max: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSlider;
    fn getLineSize(&self) -> int;
    fn getMax(&self) -> int;
    fn getMin(&self) -> int;
    fn getPageSize(&self) -> int;
    fn getSelEnd(&self) -> int;
    fn getSelStart(&self) -> int;
    fn getThumbLength(&self) -> int;
    fn getTickFreq(&self) -> int;
    fn getValue(&self) -> int;
    fn setLineSize(&self, lineSize: int);
    fn setPageSize(&self, pageSize: int);
    fn setRange(&self, minValue: int, maxValue: int);
    fn setSelection(&self, minPos: int, maxPos: int);
    fn setThumbLength(&self, len: int);
    fn setTick(&self, tickPos: int);
    fn setTickFreq(&self, n: int, pos: int);
    fn setValue(&self, value: int);
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
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxSpinButton;
    fn getMax(&self) -> int;
    fn getMin(&self) -> int;
    fn getValue(&self) -> int;
    fn setRange(&self, minVal: int, maxVal: int);
    fn setValue(&self, val: int);
}
trait wxSpinCtrl {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long, _min: int, _max: int, _init: int) -> @wxSpinCtrl;
    fn getMax(&self) -> int;
    fn getMin(&self) -> int;
    fn getValue(&self) -> int;
    fn setRange(&self, min_val: int, max_val: int);
    fn setValue(&self, val: int);
}
trait wxSpinEvent {
    fn getPosition(&self) -> int;
    fn setPosition(&self, pos: int);
}
trait wxSplashScreen {
}
trait wxSplitterEvent {
}
trait wxSplitterScrolledWindow {
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxSplitterScrolledWindow;
}
trait wxSplitterWindow {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxSplitterWindow;
    fn getBorderSize(&self) -> int;
    fn getMinimumPaneSize(&self) -> int;
    fn getSashPosition(&self) -> int;
    fn getSashSize(&self) -> int;
    fn getSplitMode(&self) -> int;
    fn getWindow1(&self) -> @wxWindow;
    fn getWindow2(&self) -> @wxWindow;
    fn initialize(&self, window: @wxWindow);
    fn isSplit(&self) -> bool;
    fn replaceWindow(&self, winOld: @wxWindow, winNew: @wxWindow) -> bool;
    fn setBorderSize(&self, width: int);
    fn setMinimumPaneSize(&self, min: int);
    fn setSashPosition(&self, position: int, redraw: bool);
    fn setSashSize(&self, width: int);
    fn setSplitMode(&self, mode: int);
    fn splitHorizontally(&self, window1: @wxWindow, window2: @wxWindow, sashPosition: int) -> bool;
    fn splitVertically(&self, window1: @wxWindow, window2: @wxWindow, sashPosition: int) -> bool;
    fn unsplit(&self, toRemove: @wxWindow) -> bool;
    fn getSashGravity(&self) -> c_double;
    fn setSashGravity(&self, gravity: c_double);
}
trait wxStaticBitmap {
    fn new(_prt: @wxWindow, _id: int, bitmap: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticBitmap;
    fn delete(&self);
    fn getBitmap(&self, @wxBitmap);
    fn getIcon(&self, @wxIcon);
    fn setBitmap(&self, bitmap: @wxBitmap);
    fn setIcon(&self, icon: @wxIcon);
}
trait wxStaticBox {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticBox;
}
trait wxStaticBoxSizer {
    fn calcMin(&self) -> @wxSize;
    fn new(box: @wxStaticBox, orient: int) -> @wxStaticBoxSizer;
    fn getStaticBox(&self) -> @wxStaticBox;
    fn recalcSizes(&self);
}
trait wxStaticLine {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticLine;
    fn getDefaultSize(&self) -> int;
    fn isVertical(&self) -> bool;
}
trait wxStaticText {
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStaticText;
}
trait wxStatusBar {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxStatusBar;
    fn getBorderX(&self) -> int;
    fn getBorderY(&self) -> int;
    fn getFieldsCount(&self) -> int;
    fn getStatusText(&self, number: int) -> @wxString;
    fn setFieldsCount(&self, number: int, widths: *int);
    fn setMinHeight(&self, height: int);
    fn setStatusText(&self, text: @wxString, number: int);
    fn setStatusWidths(&self, n: int, widths: *int);
}
trait wxStopWatch {
    fn new() -> @wxStopWatch;
    fn delete(&self);
    fn start(&self, msec: int);
    fn pause(&self);
    fn resume(&self);
    fn time(&self) -> int;
}
trait wxStreamBase {
    fn getLastError(&self) -> int;
    fn getSize(&self) -> int;
    fn isOk(&self) -> bool;
    fn delete(&self);
}
trait wxStreamBuffer {
}
trait wxStreamToTextRedirector {
}
trait wxString {
    fn new(buffer: *wchar_t) -> @wxString;
    fn newLen(buffer: *wchar_t, len: int) -> @wxString;
    fn delete(&self);
    fn getString(&self, buffer: **wchar_t) -> c_int;
    fn length(&self) -> size_t;
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
    fn getColour(index: int, @wxColour);
    fn getFont(index: int, @wxFont);
    fn getMetric(index: int) -> int;
    fn getScreenType() -> int;
}
trait wxTabCtrl {
}
trait wxTabEvent {
}
trait wxTablesInUse {
}
trait wxTaskBarIcon {
    fn new() -> @wxTaskBarIcon;
    fn delete(&self);
    fn isIconInstalled(&self) -> bool;
    fn isOk(&self) -> bool;
    fn popupMenu(&self, menu: @wxMenu) -> bool;
    fn removeIcon(&self) -> bool;
    fn setIcon(&self, icon: @wxIcon, text: @wxString) -> bool;
}
trait wxTempFile {
}
trait wxTextAttr {
    fn new(colText: @wxColour, colBack: @wxColour, font: @wxFont) -> @wxTextAttr;
    fn newDefault() -> @wxTextAttr;
    fn delete(&self);
    fn getBackgroundColour(&self, @wxColour);
    fn getFont(&self, @wxFont);
    fn getTextColour(&self, @wxColour);
    fn hasBackgroundColour(&self) -> bool;
    fn hasFont(&self) -> bool;
    fn hasTextColour(&self) -> bool;
    fn isDefault(&self) -> bool;
    fn setTextColour(&self, colour: @wxColour);
    fn setBackgroundColour(&self, colour: @wxColour);
    fn setFont(&self, font: @wxFont);
}
trait wxTextCtrl {
    fn appendText(&self, text: @wxString);
    fn canCopy(&self) -> bool;
    fn canCut(&self) -> bool;
    fn canPaste(&self) -> bool;
    fn canRedo(&self) -> bool;
    fn canUndo(&self) -> bool;
    fn changeValue(&self, text: @wxString);
    fn clear(&self);
    fn copy(&self);
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_long) -> @wxTextCtrl;
    fn cut(&self);
    fn discardEdits(&self);
    fn getInsertionPoint(&self) -> c_long;
    fn getLastPosition(&self) -> c_long;
    fn getLineLength(&self, lineNo: c_long) -> int;
    fn getLineText(&self, lineNo: c_long) -> @wxString;
    fn getNumberOfLines(&self) -> int;
    fn getSelection(&self, from: *c_void, to: *c_void);
    fn getValue(&self) -> @wxString;
    fn isEditable(&self) -> bool;
    fn isModified(&self) -> bool;
    fn loadFile(&self, file: @wxString) -> bool;
    fn paste(&self);
    fn positionToXY(&self, pos: c_long, x: *c_long, y: *c_long) -> int;
    fn redo(&self);
    fn remove(&self, from: c_long, to: c_long);
    fn replace(&self, from: c_long, to: c_long, value: @wxString);
    fn saveFile(&self, file: @wxString) -> bool;
    fn setEditable(&self, editable: bool);
    fn setInsertionPoint(&self, pos: c_long);
    fn setInsertionPointEnd(&self);
    fn setSelection(&self, from: c_long, to: c_long);
    fn setValue(&self, value: @wxString);
    fn showPosition(&self, pos: c_long);
    fn undo(&self);
    fn writeText(&self, text: @wxString);
    fn xYToPosition(&self, x: c_long, y: c_long) -> c_long;
    fn emulateKeyPress(&self, keyevent: @wxKeyEvent) -> bool;
    fn getDefaultStyle(&self) -> @wxTextAttr;
    fn getRange(&self, from: c_long, to: c_long) -> @wxString;
    fn getStringSelection(&self) -> @wxString;
    fn isMultiLine(&self) -> bool;
    fn isSingleLine(&self) -> bool;
    fn setDefaultStyle(&self, style: @wxTextAttr) -> bool;
    fn setMaxLength(&self, len: c_long);
    fn setStyle(&self, start: c_long, end: c_long, style: @wxTextAttr) -> bool;
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
    fn new(inputStream: @wxInputStream, sep: @wxString) -> @wxTextInputStream;
    fn delete(&self);
    fn readLine(&self) -> @wxString;
}
trait wxTextOutputStream {
    fn new(outputStream: @wxOutputStream, mode: int) -> @wxTextOutputStream;
    fn delete(&self);
    fn writeString(&self, txt: @wxString);
}
trait wxTextValidator {
    fn new(style: int, val: *c_void) -> @wxTextValidator;
    fn getExcludes(&self, _ref: **wchar_t) -> c_int;
    fn getIncludes(&self, _ref: **wchar_t) -> c_int;
    fn setExcludes(&self, list: *wchar_t, count: int);
    fn setIncludes(&self, list: *wchar_t, count: int);
    fn clone(&self) -> @wxValidator;
    fn transferToWindow(&self) -> bool;
    fn transferFromWindow(&self) -> bool;
    fn getStyle(&self) -> int;
    fn onChar(&self, event: @wxEvent);
    fn setStyle(&self, style: int);
}
trait wxThinSplitterWindow {
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxThinSplitterWindow;
    fn drawSash(&self, dc: @wxDC);
    fn sashHitTest(&self, x: c_int, y: c_int, tolerance: int) -> int;
    fn sizeWindows(&self);
}
trait wxThread {
}
trait wxTime {
}
trait wxTimeSpan {
}
trait wxTimer {
    fn new(_prt: @wxWindow, _id: int) -> @wxTimer;
    fn delete(&self);
    fn getInterval(&self) -> int;
    fn isOneShot(&self) -> bool;
    fn isRuning(&self) -> bool;
    fn start(&self, _int: int, _one: bool) -> bool;
    fn stop(&self);
}
trait wxTimerBase {
}
trait wxTimerEvent {
    fn getInterval(&self) -> int;
}
trait wxTimerEx {
    fn connect(&self, closure: @wxClosure);
    fn new() -> @wxTimerEx;
    fn getClosure(&self) -> @wxClosure;
}
trait wxTimerRunner {
}
trait wxTipProvider {
}
trait wxTipWindow {
    fn close(&self);
    fn new(parent: @wxWindow, text: @wxString, maxLength: int) -> @wxTipWindow;
    fn setBoundingRect(&self, x: c_int, y: c_int, w: c_int, h: c_int);
    fn setTipWindowPtr(&self, windowPtr: *c_void);
}
trait wxToggleButton {
    fn new(parent: @wxWindow, id: int, label: @wxString, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxToggleButton;
    fn enable(&self, enable: bool) -> bool;
    fn getValue(&self) -> bool;
    fn setLabel(&self, label: @wxString);
    fn setValue(&self, state: bool);
}
trait wxToolBar {
    fn addControl(&self, ctrl: @wxControl) -> bool;
    fn addSeparator(&self);
    fn addTool(&self, id: int, bmp: @wxBitmap, shelp: @wxString, lhelp: @wxString);
    fn addToolEx(&self, id: int, bmp1: @wxBitmap, bmp2: @wxBitmap, isToggle: bool, x: c_int, y: c_int, data: @wxObject, shelp: @wxString, lhelp: @wxString);
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxToolBar;
    fn delete(&self);
    fn deleteTool(&self, id: int) -> bool;
    fn deleteToolByPos(&self, pos: int) -> bool;
    fn enableTool(&self, id: int, enable: bool);
    fn getMargins(&self) -> @wxPoint;
    fn getToolBitmapSize(&self) -> @wxSize;
    fn getToolClientData(&self, id: int) -> @wxObject;
    fn getToolEnabled(&self, id: int) -> bool;
    fn getToolLongHelp(&self, id: int) -> @wxString;
    fn getToolPacking(&self) -> int;
    fn getToolShortHelp(&self, id: int) -> @wxString;
    fn getToolSize(&self) -> @wxSize;
    fn getToolState(&self, id: int) -> bool;
    fn insertControl(&self, pos: int, ctrl: @wxControl);
    fn insertSeparator(&self, pos: int);
    fn insertTool(&self, pos: int, id: int, bmp1: @wxBitmap, bmp2: @wxBitmap, isToggle: bool, data: @wxObject, shelp: @wxString, lhelp: @wxString);
    fn realize(&self) -> bool;
    fn removeTool(&self, id: int);
    fn setMargins(&self, x: c_int, y: c_int);
    fn setToolBitmapSize(&self, x: c_int, y: c_int);
    fn setToolClientData(&self, id: int, data: @wxObject);
    fn setToolLongHelp(&self, id: int, str: @wxString);
    fn setToolPacking(&self, packing: int);
    fn setToolSeparation(&self, separation: int);
    fn setToolShortHelp(&self, id: int, str: @wxString);
    fn toggleTool(&self, id: int, toggle: bool);
    fn addTool2(&self, toolId: int, label: @wxString, bmp: @wxBitmap, bmpDisabled: @wxBitmap, itemKind: int, shortHelp: @wxString, longHelp: @wxString);
}
trait wxToolBarBase {
}
trait wxToolLayoutItem {
    fn isSeparator(&self) -> bool;
    fn rect(&self, _x: *c_int, _y: *c_int, _w: *c_int, _h: *c_int);
}
trait wxToolTip {
}
trait wxToolWindow {
    fn addMiniButton(&self, _btn: *c_void);
    fn new(_obj: *c_void, _btn: *c_void, _ttl: *c_void) -> @wxToolWindow;
    fn getClient(&self) -> @wxClient;
    fn setClient(&self, _wnd: @wxWindow);
    fn setTitleFont(&self, _fnt: *c_void);
}
trait wxTopLevelWindow {
    fn enableCloseButton(&self, enable: bool) -> bool;
    fn getDefaultButton(&self) -> @wxButton;
    fn getDefaultItem(&self) -> @wxWindow;
    fn getIcon(&self) -> @wxIcon;
    fn getTitle(&self) -> @wxString;
    fn iconize(&self, iconize: bool) -> bool;
    fn isActive(&self) -> bool;
    fn isIconized(&self) -> bool;
    fn isMaximized(&self) -> bool;
    fn maximize(&self, maximize: bool);
    fn requestUserAttention(&self, flags: int);
    fn setDefaultButton(&self, pBut: @wxButton);
    fn setDefaultItem(&self, pBut: @wxWindow);
    fn setIcon(&self, pIcon: @wxIcon);
    fn setIcons(&self, _icons: *c_void);
    fn setMaxSize(&self, w: c_int, h: c_int);
    fn setMinSize(&self, w: c_int, h: c_int);
    fn setTitle(&self, pString: @wxString);
}
trait wxTreeCompanionWindow {
    fn new(parent: @wxWindow, id: int, x: c_int, y: c_int, w: c_int, h: c_int, style: int) -> @wxTreeCompanionWindow;
    fn drawItem(&self, dc: @wxDC, id: *c_void, x: c_int, y: c_int, w: c_int, h: c_int);
    fn getTreeCtrl(&self) -> @wxTreeCtrl;
    fn setTreeCtrl(&self, treeCtrl: @wxTreeCtrl);
}
trait wxTreeCtrl {
    fn addRoot(&self, text: @wxString, image: int, selectedImage: int, data: @wxTreeItemData, @wxTreeItemId);
    fn appendItem(&self, parent: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, data: @wxTreeItemData, @wxTreeItemId);
    fn collapse(&self, item: @wxTreeItemId);
    fn collapseAndReset(&self, item: @wxTreeItemId);
    fn new(_obj: *c_void, _cmp: *c_void, _prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxTreeCtrl;
    fn delete(&self, item: @wxTreeItemId);
    fn deleteAllItems(&self);
    fn deleteChildren(&self, item: @wxTreeItemId);
    fn editLabel(&self, item: @wxTreeItemId);
    fn endEditLabel(&self, item: @wxTreeItemId, discardChanges: bool);
    fn ensureVisible(&self, item: @wxTreeItemId);
    fn expand(&self, item: @wxTreeItemId);
    fn getBoundingRect(&self, item: @wxTreeItemId, textOnly: bool) -> @wxRect;
    fn getChildrenCount(&self, item: @wxTreeItemId, recursively: bool) -> int;
    fn getCount(&self) -> int;
    fn getEditControl(&self) -> @wxTextCtrl;
    fn getFirstChild(&self, item: @wxTreeItemId, cookie: *int, @wxTreeItemId);
    fn getFirstVisibleItem(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getImageList(&self) -> @wxImageList;
    fn getIndent(&self) -> int;
    fn getItemData(&self, item: @wxTreeItemId);
    fn getItemImage(&self, item: @wxTreeItemId, which: int) -> int;
    fn getItemText(&self, item: @wxTreeItemId) -> @wxString;
    fn getLastChild(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getNextChild(&self, item: @wxTreeItemId, cookie: *int, @wxTreeItemId);
    fn getNextSibling(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getNextVisible(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getParent(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getPrevSibling(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getPrevVisible(&self, item: @wxTreeItemId, @wxTreeItemId);
    fn getRootItem(&self, @wxTreeItemId);
    fn getSelection(&self, @wxTreeItemId);
    fn getSelections(&self, selections: *intptr_t) -> c_int;
    fn getSpacing(&self) -> int;
    fn getStateImageList(&self) -> @wxImageList;
    fn hitTest(&self, _x: c_int, _y: c_int, flags: *int, @wxTreeItemId);
    fn insertItem(&self, parent: @wxTreeItemId, idPrevious: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, data: *c_void, @wxTreeItemId);
    fn insertItemByIndex(&self, parent: @wxTreeItemId, index: int, text: @wxString, image: int, selectedImage: int, data: *c_void, @wxTreeItemId);
    fn isBold(&self, item: @wxTreeItemId) -> bool;
    fn isExpanded(&self, item: @wxTreeItemId) -> bool;
    fn isSelected(&self, item: @wxTreeItemId) -> bool;
    fn isVisible(&self, item: @wxTreeItemId) -> bool;
    fn itemHasChildren(&self, item: @wxTreeItemId) -> int;
    fn onCompareItems(&self, item1: @wxTreeItemId, item2: @wxTreeItemId) -> int;
    fn prependItem(&self, parent: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, data: *c_void, @wxTreeItemId);
    fn scrollTo(&self, item: @wxTreeItemId);
    fn selectItem(&self, item: @wxTreeItemId);
    fn setImageList(&self, imageList: @wxImageList);
    fn setIndent(&self, indent: int);
    fn setItemBackgroundColour(&self, item: @wxTreeItemId, col: @wxColour);
    fn setItemBold(&self, item: @wxTreeItemId, bold: bool);
    fn setItemData(&self, item: @wxTreeItemId, data: *c_void);
    fn setItemDropHighlight(&self, item: @wxTreeItemId, highlight: bool);
    fn setItemFont(&self, item: @wxTreeItemId, font: @wxFont);
    fn setItemHasChildren(&self, item: @wxTreeItemId, hasChildren: bool);
    fn setItemImage(&self, item: @wxTreeItemId, image: int, which: int);
    fn setItemText(&self, item: @wxTreeItemId, text: @wxString);
    fn setItemTextColour(&self, item: @wxTreeItemId, col: @wxColour);
    fn setSpacing(&self, spacing: int);
    fn setStateImageList(&self, imageList: @wxImageList);
    fn sortChildren(&self, item: @wxTreeItemId);
    fn toggle(&self, item: @wxTreeItemId);
    fn unselect(&self);
    fn unselectAll(&self);
    fn new2(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxTreeCtrl;
    fn insertItem2(&self, parent: @wxWindow, idPrevious: @wxTreeItemId, text: @wxString, image: int, selectedImage: int, closure: @wxClosure, @wxTreeItemId);
    fn insertItemByIndex2(&self, parent: @wxWindow, index: int, text: @wxString, image: int, selectedImage: int, closure: @wxClosure, @wxTreeItemId);
    fn getItemClientClosure(&self, item: @wxTreeItemId) -> @wxClosure;
    fn setItemClientClosure(&self, item: @wxTreeItemId, closure: @wxClosure);
    fn assignImageList(&self, imageList: @wxImageList);
    fn assignStateImageList(&self, imageList: @wxImageList);
}
trait wxTreeEvent {
    fn getCode(&self) -> int;
    fn getItem(&self, @wxTreeItemId);
    fn getLabel(&self) -> @wxString;
    fn getOldItem(&self, @wxTreeItemId);
    fn getPoint(&self) -> @wxPoint;
    fn getKeyEvent(&self) -> @wxKeyEvent;
    fn isEditCancelled(&self) -> int;
    fn allow(&self);
}
trait wxTreeItemData {
}
trait wxTreeItemId {
    fn new() -> @wxTreeItemId;
    fn delete(&self);
    fn isOk(&self) -> bool;
    fn clone(&self) -> @wxTreeItemId;
    fn newFromValue(value: intptr_t) -> @wxTreeItemId;
    fn getValue(&self) -> intptr_t;
}
trait wxTreeLayout {
}
trait wxTreeLayoutStored {
}
trait wxURL {
}
trait wxUpdateUIEvent {
    fn check(&self, check: bool);
    fn copyObject(&self, obj: @wxObject);
    fn enable(&self, enable: bool);
    fn getChecked(&self) -> bool;
    fn getEnabled(&self) -> bool;
    fn getSetChecked(&self) -> bool;
    fn getSetEnabled(&self) -> bool;
    fn getSetText(&self) -> bool;
    fn getText(&self) -> @wxString;
    fn setText(&self, text: @wxString);
}
trait wxValidator {
    fn new() -> @wxValidator;
    fn delete(&self);
    fn getWindow(&self) -> @wxWindow;
    fn setBellOnError(doIt: bool);
    fn setWindow(&self, win: @wxWindow);
    fn transferFromWindow(&self) -> bool;
    fn transferToWindow(&self) -> bool;
    fn validate(&self, parent: @wxWindow) -> bool;
}
trait wxVariant {
}
trait wxVariantData {
}
trait wxView {
}
trait wxSound {
    fn new(fileName: @wxString, isResource: bool) -> @wxSound;
    fn delete(&self);
    fn isOk(&self) -> bool;
    fn play(&self, flag: int) -> bool;
    fn stop(&self);
}
trait wxWindow {
    fn addChild(&self, child: @wxWindow);
    fn addConstraintReference(&self, otherWin: @wxWindow);
    fn captureMouse(&self);
    fn center(&self, direction: int);
    fn centerOnParent(&self, dir: int);
    fn clearBackground(&self);
    fn clientToScreen(&self, x: c_int, y: c_int) -> @wxPoint;
    fn close(&self, _force: bool) -> bool;
    fn convertDialogToPixels(&self) -> @wxPoint;
    fn convertPixelsToDialog(&self) -> @wxPoint;
    fn new(_prt: @wxWindow, _id: int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: int) -> @wxWindow;
    fn deleteRelatedConstraints(&self);
    fn destroy(&self) -> bool;
    fn destroyChildren(&self) -> bool;
    fn disable(&self) -> bool;
    fn doPhase(&self, phase: int) -> int;
    fn enable(&self) -> bool;
    fn findFocus(&self) -> @wxWindow;
    fn findWindow(&self, name: @wxString) -> @wxWindow;
    fn fit(&self);
    fn fitInside(&self);
    fn freeze(&self);
    fn getEffectiveMinSize(&self) -> @wxSize;
    fn getAutoLayout(&self) -> int;
    fn getBackgroundColour(&self, @wxColour);
    fn getBestSize(&self) -> @wxSize;
    fn getCaret(&self) -> @wxCaret;
    fn getCharHeight(&self) -> int;
    fn getCharWidth(&self) -> int;
    fn getChildren(&self, _res: *c_void, _cnt: int) -> int;
    fn getClientData(&self) -> @wxClientData;
    fn getClientSize(&self) -> @wxSize;
    fn getClientSizeConstraint(&self, _w: *c_int, _h: *c_int);
    fn getConstraints(&self) -> @wxLayoutConstraints;
    fn getConstraintsInvolvedIn(&self);
    fn getCursor(&self) -> @wxCursor;
    fn getDropTarget(&self) -> @wxDropTarget;
    fn getEventHandler(&self) -> @wxEvtHandler;
    fn getFont(&self, @wxFont);
    fn getForegroundColour(&self, @wxColour);
    fn getHandle(&self);
    fn getId(&self) -> int;
    fn getLabel(&self) -> @wxString;
    fn getLabelEmpty(&self) -> int;
    fn getMaxHeight(&self) -> int;
    fn getMaxWidth(&self) -> int;
    fn getMinHeight(&self) -> int;
    fn getMinWidth(&self) -> int;
    fn getName(&self) -> @wxString;
    fn getParent(&self) -> @wxWindow;
    fn getPosition(&self) -> @wxPoint;
    fn getPositionConstraint(&self, _x: *c_int, _y: *c_int);
    fn getRect(&self) -> @wxRect;
    fn getScrollPos(&self, orient: int) -> int;
    fn getScrollRange(&self, orient: int) -> int;
    fn getScrollThumb(&self, orient: int) -> int;
    fn getSize(&self) -> @wxSize;
    fn getSizeConstraint(&self, _w: *c_int, _h: *c_int);
    fn getSizer(&self) -> @wxSizer;
    fn getTextExtent(&self, string: @wxString, x: *int, y: *int, descent: *int, externalLeading: *int, theFont: @wxFont);
    fn getToolTip(&self) -> @wxString;
    fn getUpdateRegion(&self) -> @wxRegion;
    fn getValidator(&self) -> @wxValidator;
    fn getVirtualSize(&self) -> @wxSize;
    fn getWindowStyleFlag(&self) -> int;
    fn hasFlag(&self, flag: int) -> bool;
    fn hide(&self) -> bool;
    fn initDialog(&self);
    fn isBeingDeleted(&self) -> bool;
    fn isEnabled(&self) -> bool;
    fn isExposed(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool;
    fn isShown(&self) -> bool;
    fn isTopLevel(&self) -> bool;
    fn layout(&self) -> int;
    fn layoutPhase1(&self, noChanges: *int) -> int;
    fn layoutPhase2(&self, noChanges: *int) -> int;
    fn lower(&self);
    fn makeModal(&self, modal: bool);
    fn move(&self, x: c_int, y: c_int);
    fn moveConstraint(&self, x: c_int, y: c_int);
    fn popEventHandler(&self, deleteHandler: bool);
    fn popupMenu(&self, menu: @wxMenu, x: c_int, y: c_int) -> int;
    fn prepareDC(&self, dc: @wxDC);
    fn pushEventHandler(&self, handler: @wxEvtHandler);
    fn raise(&self);
    fn refresh(&self, eraseBackground: bool);
    fn refreshRect(&self, eraseBackground: bool, x: c_int, y: c_int, w: c_int, h: c_int);
    fn releaseMouse(&self);
    fn removeChild(&self, child: @wxWindow);
    fn removeConstraintReference(&self, otherWin: @wxWindow);
    fn reparent(&self, _par: @wxWindow) -> int;
    fn resetConstraints(&self);
    fn screenToClient(&self, x: c_int, y: c_int) -> @wxPoint;
    fn scrollWindow(&self, dx: c_int, dy: c_int);
    fn scrollWindowRect(&self, dx: c_int, dy: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
    fn setAcceleratorTable(&self, accel: @wxAcceleratorTable);
    fn setAutoLayout(&self, autoLayout: bool);
    fn setBackgroundColour(&self, colour: @wxColour) -> int;
    fn setCaret(&self, caret: @wxCaret);
    fn setClientData(&self, data: @wxClientData);
    fn setClientObject(&self, data: @wxClientData);
    fn setClientSize(&self, width: c_int, height: c_int);
    fn setConstraintSizes(&self, recurse: int);
    fn setConstraints(&self, constraints: @wxLayoutConstraints);
    fn setCursor(&self, cursor: @wxCursor) -> int;
    fn setDropTarget(&self, dropTarget: @wxDropTarget);
    fn setExtraStyle(&self, exStyle: c_long);
    fn setFocus(&self);
    fn setFont(&self, font: @wxFont) -> int;
    fn setForegroundColour(&self, colour: @wxColour) -> int;
    fn setId(&self, _id: int);
    fn setLabel(&self, _title: @wxString);
    fn setName(&self, _name: @wxString);
    fn setScrollPos(&self, orient: int, pos: int, refresh: bool);
    fn setScrollbar(&self, orient: int, pos: int, thumbVisible: int, range: int, refresh: bool);
    fn setSize(&self, x: c_int, y: c_int, width: c_int, height: c_int, sizeFlags: int);
    fn setSizeConstraint(&self, x: c_int, y: c_int, w: c_int, h: c_int);
    fn setSizeHints(&self, minW: int, minH: int, maxW: int, maxH: int, incW: int, incH: int);
    fn setSizer(&self, sizer: @wxSizer);
    fn setToolTip(&self, tip: @wxString);
    fn setValidator(&self, validator: @wxValidator);
    fn setWindowStyleFlag(&self, style: c_long);
    fn show(&self) -> bool;
    fn thaw(&self);
    fn transferDataFromWindow(&self) -> bool;
    fn transferDataToWindow(&self) -> bool;
    fn unsetConstraints(&self, c: *c_void);
    fn updateWindowUI(&self);
    fn validate(&self) -> bool;
    fn setVirtualSize(&self, w: c_int, h: c_int);
    fn warpPointer(&self, x: c_int, y: c_int);
    fn convertDialogToPixelsEx(&self) -> @wxPoint;
    fn convertPixelsToDialogEx(&self) -> @wxPoint;
    fn screenToClient2(&self, x: c_int, y: c_int) -> @wxPoint;
}
trait wxWindowCreateEvent {
    fn getWindow(&self) -> @wxWindow;
}
trait wxWindowDC {
    fn new(win: @wxWindow) -> @wxWindowDC;
    fn delete(&self);
}
trait wxWindowDestroyEvent {
    fn getWindow(&self) -> @wxWindow;
}
trait wxWindowDisabler {
}
trait wxWizard {
    fn chain(f: @wxWizardPageSimple, s: @wxWizardPageSimple);
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _bmp: @wxBitmap, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> @wxWizard;
    fn getCurrentPage(&self) -> @wxWizardPage;
    fn getPageSize(&self) -> @wxSize;
    fn runWizard(&self, firstPage: @wxWizardPage) -> int;
    fn setPageSize(&self, w: c_int, h: c_int);
}
trait wxWizardEvent {
    fn getDirection(&self) -> int;
}
trait wxWizardPage {
}
trait wxWizardPageSimple {
    fn new(_prt: @wxWizard) -> @wxWizardPageSimple;
    fn getBitmap(&self, @wxBitmap);
    fn getNext(&self) -> @wxWizardPageSimple;
    fn getPrev(&self) -> @wxWizardPageSimple;
    fn setNext(&self, next: @wxWizardPageSimple);
    fn setPrev(&self, prev: @wxWizardPageSimple);
}
trait wxXmlResource {
    fn addHandler(&self, handler: @wxEvtHandler);
    fn addSubclassFactory(&self, factory: *c_void);
    fn attachUnknownControl(&self, control: @wxControl, parent: @wxWindow) -> int;
    fn clearHandlers(&self);
    fn compareVersion(&self, major: int, minor: int, release: int, revision: int) -> int;
    fn new(flags: int) -> @wxXmlResource;
    fn newFromFile(filemask: @wxString, flags: int) -> @wxXmlResource;
    fn delete(&self);
    fn get() -> @wxXmlResource;
    fn getDomain(&self) -> @wxString;
    fn getFlags(&self) -> int;
    fn getVersion(&self) -> c_long;
    fn getXRCID(&self, str_id: @wxString) -> int;
    fn initAllHandlers(&self);
    fn insertHandler(&self, handler: @wxEvtHandler);
    fn load(&self, filemask: @wxString) -> bool;
    fn loadBitmap(&self, name: @wxString, @wxBitmap);
    fn loadDialog(&self, parent: @wxWindow, name: @wxString) -> @wxDialog;
    fn loadFrame(&self, parent: @wxWindow, name: @wxString) -> @wxFrame;
    fn loadIcon(&self, name: @wxString, @wxIcon);
    fn loadMenu(&self, name: @wxString) -> @wxMenu;
    fn loadMenuBar(&self, parent: @wxWindow, name: @wxString) -> @wxMenuBar;
    fn loadPanel(&self, parent: @wxWindow, name: @wxString) -> @wxPanel;
    fn loadToolBar(&self, parent: @wxWindow, name: @wxString) -> @wxToolBar;
    fn getSizer(&self, str_id: @wxString) -> @wxSizer;
    fn getBoxSizer(&self, str_id: @wxString) -> @wxBoxSizer;
    fn getStaticBoxSizer(&self, str_id: @wxString) -> @wxStaticBoxSizer;
    fn getGridSizer(&self, str_id: @wxString) -> @wxGridSizer;
    fn getFlexGridSizer(&self, str_id: @wxString) -> @wxFlexGridSizer;
    fn getBitmapButton(&self, str_id: @wxString) -> @wxBitmapButton;
    fn getButton(&self, str_id: @wxString) -> @wxButton;
    fn getCalendarCtrl(&self, str_id: @wxString) -> @wxCalendarCtrl;
    fn getCheckBox(&self, str_id: @wxString) -> @wxCheckBox;
    fn getCheckListBox(&self, str_id: @wxString) -> @wxCheckListBox;
    fn getChoice(&self, str_id: @wxString) -> @wxChoice;
    fn getComboBox(&self, str_id: @wxString) -> @wxComboBox;
    fn getGauge(&self, str_id: @wxString) -> @wxGauge;
    fn getGrid(&self, str_id: @wxString) -> @wxGrid;
    fn getHtmlWindow(&self, str_id: @wxString) -> @wxHtmlWindow;
    fn getListBox(&self, str_id: @wxString) -> @wxListBox;
    fn getListCtrl(&self, str_id: @wxString) -> @wxListCtrl;
    fn getMDIChildFrame(&self, str_id: @wxString) -> @wxMDIChildFrame;
    fn getMDIParentFrame(&self, str_id: @wxString) -> @wxMDIParentFrame;
    fn getMenu(&self, str_id: @wxString) -> @wxMenu;
    fn getMenuBar(&self, str_id: @wxString) -> @wxMenuBar;
    fn getMenuItem(&self, str_id: @wxString) -> @wxMenuItem;
    fn getNotebook(&self, str_id: @wxString) -> @wxNotebook;
    fn getPanel(&self, str_id: @wxString) -> @wxPanel;
    fn getRadioButton(&self, str_id: @wxString) -> @wxRadioButton;
    fn getRadioBox(&self, str_id: @wxString) -> @wxRadioBox;
    fn getScrollBar(&self, str_id: @wxString) -> @wxScrollBar;
    fn getScrolledWindow(&self, str_id: @wxString) -> @wxScrolledWindow;
    fn getSlider(&self, str_id: @wxString) -> @wxSlider;
    fn getSpinButton(&self, str_id: @wxString) -> @wxSpinButton;
    fn getSpinCtrl(&self, str_id: @wxString) -> @wxSpinCtrl;
    fn getSplitterWindow(&self, str_id: @wxString) -> @wxSplitterWindow;
    fn getStaticBitmap(&self, str_id: @wxString) -> @wxStaticBitmap;
    fn getStaticBox(&self, str_id: @wxString) -> @wxStaticBox;
    fn getStaticLine(&self, str_id: @wxString) -> @wxStaticLine;
    fn getStaticText(&self, str_id: @wxString) -> @wxStaticText;
    fn getTextCtrl(&self, str_id: @wxString) -> @wxTextCtrl;
    fn getTreeCtrl(&self, str_id: @wxString) -> @wxTreeCtrl;
    fn unload(&self, filemask: @wxString) -> bool;
    fn set(&self, res: @wxXmlResource) -> @wxXmlResource;
    fn setDomain(&self, domain: @wxString);
    fn setFlags(&self, flags: int);
    fn getStyledTextCtrl(&self, str_id: @wxString) -> @wxStyledTextCtrl;
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
    fn append(&self, prop: @wxPGProperty) -> @wxPGProperty;
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int) -> @wxPropertyGrid;
    fn disableProperty(&self, propName: @wxString) -> bool;
}
trait wxPropertyGridEvent {
    fn hasProperty(&self) -> bool;
    fn getProperty(&self) -> @wxPGProperty;
}
trait wxPGProperty {
    fn getLabel(&self) -> @wxString;
    fn getName(&self) -> @wxString;
    fn getValueAsString(&self) -> @wxString;
    fn getValueType(&self) -> @wxString;
    fn setHelpString(&self, helpString: @wxString);
}
trait wxStringProperty {
    fn new(label: @wxString, name: @wxString, value: @wxString) -> @wxStringProperty;
}
trait wxIntProperty {
    fn new(label: @wxString, name: @wxString, value: int) -> @wxIntProperty;
}
trait wxBoolProperty {
    fn new(label: @wxString, name: @wxString, value: bool) -> @wxBoolProperty;
}
trait wxFloatProperty {
    fn new(label: @wxString, name: @wxString, value: c_float) -> @wxFloatProperty;
}
trait wxDateProperty {
    fn new(label: @wxString, name: @wxString, value: @wxDateTime) -> @wxDateProperty;
}
trait wxFileProperty {
    fn new(label: @wxString, name: @wxString, value: @wxString) -> @wxFileProperty;
}
trait wxPropertyCategory {
    fn new(label: @wxString) -> @wxPropertyCategory;
}
trait wxGenericDragImage {
    fn new(cursor: @wxCursor) -> @wxGenericDragImage;
    fn doDrawImage(&self, dc: @wxDC, x: int, y: int) -> bool;
    fn getImageRect(&self, x_pos: int, y_pos: int) -> @wxRect;
    fn updateBackingFromWindow(&self, windowDC: @wxDC, destDC: @wxMemoryDC, x: int, y: int, w: int, h: int, xdest: int, ydest: int, width: int, height: int) -> bool;
}
trait wxGraphicsObject {
    fn getRenderer() -> @wxGraphicsRenderer;
    fn isNull(&self) -> bool;
}
trait wxGraphicsBrush {
    fn new() -> @wxGraphicsBrush;
    fn delete(&self);
}
trait wxGraphicsContext {
    fn new(dc: @wxWindowDC) -> @wxGraphicsContext;
    fn newFromWindow(window: @wxWindow) -> @wxGraphicsContext;
    fn delete(&self);
    fn newFromNative(context: *c_void) -> @wxGraphicsContext;
    fn newFromNativeWindow(window: *c_void) -> @wxGraphicsContext;
    fn clip(&self, region: @wxRegion);
    fn clipByRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double);
    fn resetClip(&self);
    fn drawBitmap(&self, bmp: @wxBitmap, x: c_double, y: c_double, w: c_double, h: c_double);
    fn drawEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double);
    fn drawIcon(&self, icon: @wxIcon, x: c_double, y: c_double, w: c_double, h: c_double);
    fn drawLines(&self, n: size_t, x: *c_void, y: *c_void, style: int);
    fn drawPath(&self, path: @wxGraphicsPath, style: int);
    fn drawRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double);
    fn drawRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double);
    fn drawText(&self, text: @wxString, x: c_double, y: c_double);
    fn drawTextWithAngle(&self, text: @wxString, x: c_double, y: c_double, radius: c_double);
    fn fillPath(&self, path: @wxGraphicsPath, style: int);
    fn strokePath(&self, path: @wxGraphicsPath);
    fn getNativeContext(&self);
    fn getTextExtent(&self, text: @wxString, width: *c_double, height: *c_double, descent: *c_double, externalLeading: *c_double);
    fn rotate(&self, angle: c_double);
    fn scale(&self, xScale: c_double, yScale: c_double);
    fn translate(&self, dx: c_double, dy: c_double);
    fn setTransform(&self, path: @wxGraphicsMatrix);
    fn concatTransform(&self, path: @wxGraphicsMatrix);
    fn setBrush(&self, brush: @wxBrush);
    fn setGraphicsBrush(&self, brush: @wxGraphicsBrush);
    fn setFont(&self, font: @wxFont, colour: @wxColour);
    fn setGraphicsFont(&self, font: @wxGraphicsFont);
    fn setPen(&self, pen: @wxPen);
    fn setGraphicsPen(&self, pen: @wxGraphicsPen);
    fn strokeLine(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double);
    fn strokeLines(&self, n: size_t, x: *c_void, y: *c_void, style: int);
}
trait wxGraphicsFont {
    fn new() -> @wxGraphicsFont;
    fn delete(&self);
}
trait wxGraphicsMatrix {
    fn new() -> @wxGraphicsMatrix;
    fn delete(&self);
    fn concat(&self, t: @wxGraphicsMatrix);
    fn get(&self, a: *c_double, b: *c_double, c: *c_double, d: *c_double, tx: *c_double, ty: *c_double);
    fn getNativeMatrix(&self);
    fn invert(&self);
    fn isEqual(&self, t: @wxGraphicsMatrix) -> bool;
    fn isIdentity(&self) -> bool;
    fn rotate(&self, angle: c_double);
    fn scale(&self, xScale: c_double, yScale: c_double);
    fn set(&self, a: c_double, b: c_double, c: c_double, d: c_double, tx: c_double, ty: c_double);
    fn translate(&self, dx: c_double, dy: c_double);
    fn transformPoint(&self, x: *c_double, y: *c_double);
    fn transformDistance(&self, dx: *c_double, dy: *c_double);
}
trait wxGraphicsPath {
    fn new() -> @wxGraphicsPath;
    fn delete(&self);
    fn moveToPoint(&self, x: c_double, y: c_double);
    fn addArc(&self, x: c_double, y: c_double, r: c_double, startAngle: c_double, endAngle: c_double, clockwise: bool);
    fn addArcToPoint(&self, x1: c_double, y1: c_double, x2: c_double, y2: c_double, r: c_double);
    fn addCircle(&self, x: c_double, y: c_double, r: c_double);
    fn addCurveToPoint(&self, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double);
    fn addEllipse(&self, x: c_double, y: c_double, w: c_double, h: c_double);
    fn addLineToPoint(&self, x: c_double, y: c_double);
    fn addPath(&self, x: c_double, y: c_double, path: @wxGraphicsPath);
    fn addQuadCurveToPoint(&self, cx: c_double, cy: c_double, x: c_double, y: c_double);
    fn addRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double);
    fn addRoundedRectangle(&self, x: c_double, y: c_double, w: c_double, h: c_double, radius: c_double);
    fn closeSubpath(&self);
    fn contains(&self, x: c_double, y: c_double, style: int);
    fn getBox(&self, x: *c_double, y: *c_double, w: *c_double, h: *c_double);
    fn getCurrentPoint(&self, x: *c_double, y: *c_double);
    fn transform(&self, matrix: @wxGraphicsMatrix);
    fn getNativePath(&self);
    fn unGetNativePath(p: *c_void);
}
trait wxGraphicsPen {
    fn new() -> @wxGraphicsPen;
    fn delete(&self);
}
trait wxGraphicsRenderer {
    fn delete(&self);
    fn getDefaultRenderer(&self) -> @wxGraphicsRenderer;
    fn newContext(dc: @wxWindowDC) -> @wxGraphicsContext;
    fn newContextFromWindow(window: @wxWindow) -> @wxGraphicsContext;
    fn newContextFromNativeContext(context: *c_void) -> @wxGraphicsContext;
    fn newContextFromNativeWindow(window: *c_void) -> @wxGraphicsContext;
}
trait wxGLContext {
    fn new(win: @wxGLCanvas, other: @wxGLContext) -> @wxGLContext;
    fn newFromNull(win: @wxGLCanvas) -> @wxGLContext;
    fn setCurrent(&self, win: @wxGLCanvas) -> bool;
}
trait wxManagedPtr {
    fn getPtr(&self);
    fn noFinalize(&self);
    fn finalize(&self);
    fn delete(&self);
    fn getDeleteFunction();
    fn newFromObject(obj: @wxObject) -> @wxManagedPtr;
    fn newFromDateTime(obj: @wxDateTime) -> @wxManagedPtr;
    fn newFromGridCellCoordsArray(obj: @wxGridCellCoordsArray) -> @wxManagedPtr;
    fn newFromBitmap(obj: @wxBitmap) -> @wxManagedPtr;
    fn newFromIcon(obj: @wxIcon) -> @wxManagedPtr;
    fn newFromBrush(obj: @wxBrush) -> @wxManagedPtr;
    fn newFromColour(obj: @wxColour) -> @wxManagedPtr;
    fn newFromCursor(obj: @wxCursor) -> @wxManagedPtr;
    fn newFromFont(obj: @wxFont) -> @wxManagedPtr;
    fn newFromPen(obj: @wxPen) -> @wxManagedPtr;
}
trait wxMediaCtrl {
    fn new(parent: @wxWindow, windowID: int, fileName: @wxString, x: int, y: int, w: int, h: int, style: c_long, szBackend: @wxString, name: @wxString) -> @wxMediaCtrl;
    fn delete(&self);
    fn getBestSize(&self) -> @wxSize;
    fn getPlaybackRate(&self) -> c_double;
    fn getVolume(&self) -> c_double;
    fn getState(&self) -> int;
    fn length(&self) -> i64;
    fn load(&self, fileName: @wxString) -> bool;
    fn loadURI(&self, uri: @wxString) -> bool;
    fn loadURIWithProxy(&self, uri: @wxString, proxy: @wxString) -> bool;
    fn pause(&self) -> bool;
    fn play(&self) -> bool;
    fn seek(&self, offsetWhere: i64, mode: int) -> i64;
    fn setPlaybackRate(&self, dRate: c_double) -> bool;
    fn setVolume(&self, dVolume: c_double) -> bool;
    fn showPlayerControls(&self, flags: int) -> bool;
    fn stop(&self) -> bool;
    fn tell(&self) -> i64;
}
trait wxMediaEvent {
}
trait wxcPrintout {
    fn new(title: @wxString) -> @wxcPrintout;
    fn delete(&self);
    fn setPageLimits(&self, startPage: int, endPage: int, fromPage: int, toPage: int);
    fn getEvtHandler(&self) -> @wxcPrintoutHandler;
}
trait wxcPrintEvent {
    fn getPrintout(&self) -> @wxcPrintout;
    fn getPage(&self) -> int;
    fn getEndPage(&self) -> int;
    fn getContinue(&self) -> bool;
    fn setContinue(&self, cont: bool);
    fn setPageLimits(&self, startPage: int, endPage: int, fromPage: int, toPage: int);
}
trait wxcPrintoutHandler {
}
trait wxStyledTextCtrl {
    fn addText(&self, text: @wxString);
    fn addStyledText(&self, data: @wxMemoryBuffer);
    fn insertText(&self, pos: int, text: @wxString);
    fn clearAll(&self);
    fn clearDocumentStyle(&self);
    fn getLength(&self) -> int;
    fn getCharAt(&self, pos: int) -> int;
    fn getCurrentPos(&self) -> int;
    fn getAnchor(&self) -> int;
    fn getStyleAt(&self, pos: int) -> int;
    fn redo(&self);
    fn setUndoCollection(&self, collectUndo: bool);
    fn selectAll(&self);
    fn setSavePoint(&self);
    fn canRedo(&self) -> bool;
    fn markerLineFromHandle(&self, handle: int) -> int;
    fn markerDeleteHandle(&self, handle: int);
    fn getUndoCollection(&self) -> bool;
    fn getViewWhiteSpace(&self) -> int;
    fn setViewWhiteSpace(&self, viewWS: int);
    fn positionFromPoint(&self, pt_x: c_int, pt_y: c_int) -> int;
    fn positionFromPointClose(&self, x: int, y: int) -> int;
    fn gotoLine(&self, line: int);
    fn gotoPos(&self, pos: int);
    fn setAnchor(&self, posAnchor: int);
    fn getEndStyled(&self) -> int;
    fn convertEOLs(&self, eolMode: int);
    fn getEOLMode(&self) -> int;
    fn setEOLMode(&self, eolMode: int);
    fn startStyling(&self, pos: int, mask: int);
    fn setStyling(&self, length: int, style: int);
    fn getBufferedDraw(&self) -> bool;
    fn setBufferedDraw(&self, buffered: bool);
    fn setTabWidth(&self, tabWidth: int);
    fn getTabWidth(&self) -> int;
    fn setCodePage(&self, codePage: int);
    fn markerDefine(&self, markerNumber: int, markerSymbol: int, foreground_r: u8, foreground_g: u8, foreground_b: u8, background_r: u8, background_g: u8, background_b: u8);
    fn markerSetForeground(&self, markerNumber: int, fore_r: u8, fore_g: u8, fore_b: u8);
    fn markerSetBackground(&self, markerNumber: int, back_r: u8, back_g: u8, back_b: u8);
    fn markerAdd(&self, line: int, markerNumber: int) -> int;
    fn markerDelete(&self, line: int, markerNumber: int);
    fn markerDeleteAll(&self, markerNumber: int);
    fn markerGet(&self, line: int) -> int;
    fn markerNext(&self, lineStart: int, markerMask: int) -> int;
    fn markerPrevious(&self, lineStart: int, markerMask: int) -> int;
    fn markerDefineBitmap(&self, markerNumber: int, bmp: @wxBitmap);
    fn setMarginType(&self, margin: int, marginType: int);
    fn getMarginType(&self, margin: int) -> int;
    fn setMarginWidth(&self, margin: int, pixelWidth: int);
    fn getMarginWidth(&self, margin: int) -> int;
    fn setMarginMask(&self, margin: int, mask: int);
    fn getMarginMask(&self, margin: int) -> int;
    fn setMarginSensitive(&self, margin: int, sensitive: bool);
    fn getMarginSensitive(&self, margin: int) -> bool;
    fn styleClearAll(&self);
    fn styleSetForeground(&self, style: int, fore_r: u8, fore_g: u8, fore_b: u8);
    fn styleSetBackground(&self, style: int, back_r: u8, back_g: u8, back_b: u8);
    fn styleSetBold(&self, style: int, bold: bool);
    fn styleSetItalic(&self, style: int, italic: bool);
    fn styleSetSize(&self, style: int, sizePoints: int);
    fn styleSetFaceName(&self, style: int, fontName: @wxString);
    fn styleSetEOLFilled(&self, style: int, filled: bool);
    fn styleResetDefault(&self);
    fn styleSetUnderline(&self, style: int, underline: bool);
    fn styleSetCase(&self, style: int, caseForce: int);
    fn styleSetCharacterSet(&self, style: int, characterSet: int);
    fn styleSetHotSpot(&self, style: int, hotspot: bool);
    fn setSelForeground(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8);
    fn setSelBackground(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8);
    fn setCaretForeground(&self, fore_r: u8, fore_g: u8, fore_b: u8);
    fn cmdKeyAssign(&self, key: int, modifiers: int, cmd: int);
    fn cmdKeyClear(&self, key: int, modifiers: int);
    fn cmdKeyClearAll(&self);
    fn setStyleBytes(&self, length: int, styleBytes: *char);
    fn styleSetVisible(&self, style: int, visible: bool);
    fn getCaretPeriod(&self) -> int;
    fn setCaretPeriod(&self, periodMilliseconds: int);
    fn setWordChars(&self, characters: @wxString);
    fn beginUndoAction(&self);
    fn endUndoAction(&self);
    fn indicatorSetStyle(&self, indic: int, style: int);
    fn indicatorGetStyle(&self, indic: int) -> int;
    fn indicatorSetForeground(&self, indic: int, fore_r: u8, fore_g: u8, fore_b: u8);
    fn setWhitespaceForeground(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8);
    fn setWhitespaceBackground(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8);
    fn setStyleBits(&self, bits: int);
    fn getStyleBits(&self) -> int;
    fn setLineState(&self, line: int, state: int);
    fn getLineState(&self, line: int) -> int;
    fn getMaxLineState(&self) -> int;
    fn getCaretLineVisible(&self) -> bool;
    fn setCaretLineVisible(&self, show: bool);
    fn styleSetChangeable(&self, style: int, changeable: bool);
    fn autoCompShow(&self, lenEntered: int, itemList: @wxString);
    fn autoCompCancel(&self);
    fn autoCompActive(&self) -> bool;
    fn autoCompPosStart(&self) -> int;
    fn autoCompComplete(&self);
    fn autoCompStops(&self, characterSet: @wxString);
    fn autoCompSetSeparator(&self, separatorCharacter: int);
    fn autoCompGetSeparator(&self) -> int;
    fn autoCompSelect(&self, text: @wxString);
    fn autoCompSetCancelAtStart(&self, cancel: bool);
    fn autoCompGetCancelAtStart(&self) -> bool;
    fn autoCompSetFillUps(&self, characterSet: @wxString);
    fn autoCompSetChooseSingle(&self, chooseSingle: bool);
    fn autoCompGetChooseSingle(&self) -> bool;
    fn autoCompSetIgnoreCase(&self, ignoreCase: bool);
    fn autoCompGetIgnoreCase(&self) -> bool;
    fn userListShow(&self, listType: int, itemList: @wxString);
    fn autoCompSetAutoHide(&self, autoHide: bool);
    fn autoCompGetAutoHide(&self) -> bool;
    fn autoCompSetDropRestOfWord(&self, dropRestOfWord: bool);
    fn autoCompGetDropRestOfWord(&self) -> bool;
    fn registerImage(&self, type_: int, bmp: @wxBitmap);
    fn clearRegisteredImages(&self);
    fn autoCompGetTypeSeparator(&self) -> int;
    fn autoCompSetTypeSeparator(&self, separatorCharacter: int);
    fn setIndent(&self, indentSize: int);
    fn getIndent(&self) -> int;
    fn setUseTabs(&self, useTabs: bool);
    fn getUseTabs(&self) -> bool;
    fn setLineIndentation(&self, line: int, indentSize: int);
    fn getLineIndentation(&self, line: int) -> int;
    fn getLineIndentPosition(&self, line: int) -> int;
    fn getColumn(&self, pos: int) -> int;
    fn setUseHorizontalScrollBar(&self, show: bool);
    fn getUseHorizontalScrollBar(&self) -> bool;
    fn setIndentationGuides(&self, show: bool);
    fn getIndentationGuides(&self) -> bool;
    fn setHighlightGuide(&self, column: int);
    fn getHighlightGuide(&self) -> int;
    fn getLineEndPosition(&self, line: int) -> int;
    fn getCodePage(&self) -> int;
    fn getReadOnly(&self) -> bool;
    fn setCurrentPos(&self, pos: int);
    fn setSelectionStart(&self, pos: int);
    fn getSelectionStart(&self) -> int;
    fn setSelectionEnd(&self, pos: int);
    fn getSelectionEnd(&self) -> int;
    fn setPrintMagnification(&self, magnification: int);
    fn getPrintMagnification(&self) -> int;
    fn setPrintColourMode(&self, mode: int);
    fn getPrintColourMode(&self) -> int;
    fn findText(&self, minPos: int, maxPos: int, text: @wxString, flags: int) -> int;
    fn formatRange(&self, doDraw: bool, startPos: int, endPos: int, draw: @wxDC, target: @wxDC, renderRect: @wxRect, pageRect: @wxRect) -> int;
    fn getFirstVisibleLine(&self) -> int;
    fn getLineCount(&self) -> int;
    fn setMarginLeft(&self, pixelWidth: int);
    fn getMarginLeft(&self) -> int;
    fn setMarginRight(&self, pixelWidth: int);
    fn getMarginRight(&self) -> int;
    fn getModify(&self) -> bool;
    fn setSelection(&self, start: int, end: int);
    fn hideSelection(&self, normal: bool);
    fn lineFromPosition(&self, pos: int) -> int;
    fn positionFromLine(&self, line: int) -> int;
    fn lineScroll(&self, columns: int, lines: int);
    fn ensureCaretVisible(&self);
    fn replaceSelection(&self, text: @wxString);
    fn setReadOnly(&self, readOnly: bool);
    fn canPaste(&self) -> bool;
    fn canUndo(&self) -> bool;
    fn emptyUndoBuffer(&self);
    fn undo(&self);
    fn cut(&self);
    fn copy(&self);
    fn paste(&self);
    fn clear(&self);
    fn setText(&self, text: @wxString);
    fn getTextLength(&self) -> int;
    fn setOvertype(&self, overtype: bool);
    fn getOvertype(&self) -> bool;
    fn setCaretWidth(&self, pixelWidth: int);
    fn getCaretWidth(&self) -> int;
    fn setTargetStart(&self, pos: int);
    fn getTargetStart(&self) -> int;
    fn setTargetEnd(&self, pos: int);
    fn getTargetEnd(&self) -> int;
    fn replaceTarget(&self, text: @wxString) -> int;
    fn replaceTargetRE(&self, text: @wxString) -> int;
    fn searchInTarget(&self, text: @wxString) -> int;
    fn setSearchFlags(&self, flags: int);
    fn getSearchFlags(&self) -> int;
    fn callTipShow(&self, pos: int, definition: @wxString);
    fn callTipCancel(&self);
    fn callTipActive(&self) -> bool;
    fn callTipPosAtStart(&self) -> int;
    fn callTipSetHighlight(&self, start: int, end: int);
    fn callTipSetBackground(&self, back_r: u8, back_g: u8, back_b: u8);
    fn callTipSetForeground(&self, fore_r: u8, fore_g: u8, fore_b: u8);
    fn callTipSetForegroundHighlight(&self, fore_r: u8, fore_g: u8, fore_b: u8);
    fn visibleFromDocLine(&self, line: int) -> int;
    fn docLineFromVisible(&self, lineDisplay: int) -> int;
    fn setFoldLevel(&self, line: int, level: int);
    fn getFoldLevel(&self, line: int) -> int;
    fn getLastChild(&self, line: int, level: int) -> int;
    fn getFoldParent(&self, line: int) -> int;
    fn showLines(&self, lineStart: int, lineEnd: int);
    fn hideLines(&self, lineStart: int, lineEnd: int);
    fn getLineVisible(&self, line: int) -> bool;
    fn setFoldExpanded(&self, line: int, expanded: bool);
    fn getFoldExpanded(&self, line: int) -> bool;
    fn toggleFold(&self, line: int);
    fn ensureVisible(&self, line: int);
    fn setFoldFlags(&self, flags: int);
    fn ensureVisibleEnforcePolicy(&self, line: int);
    fn setTabIndents(&self, tabIndents: bool);
    fn getTabIndents(&self) -> bool;
    fn setBackSpaceUnIndents(&self, bsUnIndents: bool);
    fn getBackSpaceUnIndents(&self) -> bool;
    fn setMouseDwellTime(&self, periodMilliseconds: int);
    fn getMouseDwellTime(&self) -> int;
    fn wordStartPosition(&self, pos: int, onlyWordCharacters: bool) -> int;
    fn wordEndPosition(&self, pos: int, onlyWordCharacters: bool) -> int;
    fn setWrapMode(&self, mode: int);
    fn getWrapMode(&self) -> int;
    fn setLayoutCache(&self, mode: int);
    fn getLayoutCache(&self) -> int;
    fn setScrollWidth(&self, pixelWidth: int);
    fn getScrollWidth(&self) -> int;
    fn textWidth(&self, style: int, text: @wxString) -> int;
    fn setEndAtLastLine(&self, endAtLastLine: bool);
    fn getEndAtLastLine(&self) -> int;
    fn textHeight(&self, line: int) -> int;
    fn setUseVerticalScrollBar(&self, show: bool);
    fn getUseVerticalScrollBar(&self) -> bool;
    fn appendText(&self, text: @wxString);
    fn getTwoPhaseDraw(&self) -> bool;
    fn setTwoPhaseDraw(&self, twoPhase: bool);
    fn targetFromSelection(&self);
    fn linesJoin(&self);
    fn linesSplit(&self, pixelWidth: int);
    fn setFoldMarginColour(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8);
    fn setFoldMarginHiColour(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8);
    fn lineDuplicate(&self);
    fn homeDisplay(&self);
    fn homeDisplayExtend(&self);
    fn lineEndDisplay(&self);
    fn lineEndDisplayExtend(&self);
    fn lineCopy(&self);
    fn moveCaretInsideView(&self);
    fn lineLength(&self, line: int) -> int;
    fn braceHighlight(&self, pos1: int, pos2: int);
    fn braceBadLight(&self, pos: int);
    fn braceMatch(&self, pos: int) -> int;
    fn getViewEOL(&self) -> bool;
    fn setViewEOL(&self, visible: bool);
    fn setDocPointer(&self, docPointer: @wxSTCDoc);
    fn setModEventMask(&self, mask: int);
    fn getEdgeColumn(&self) -> int;
    fn setEdgeColumn(&self, column: int);
    fn getEdgeMode(&self) -> int;
    fn setEdgeMode(&self, mode: int);
    fn setEdgeColour(&self, edgeColour_r: u8, edgeColour_g: u8, edgeColour_b: u8);
    fn searchAnchor(&self);
    fn searchNext(&self, flags: int, text: @wxString) -> int;
    fn searchPrev(&self, flags: int, text: @wxString) -> int;
    fn linesOnScreen(&self) -> int;
    fn usePopUp(&self, allowPopUp: bool);
    fn selectionIsRectangle(&self) -> bool;
    fn setZoom(&self, zoom: int);
    fn getZoom(&self) -> int;
    fn addRefDocument(&self, docPointer: @wxSTCDoc);
    fn releaseDocument(&self, docPointer: @wxSTCDoc);
    fn getModEventMask(&self) -> int;
    fn setSTCFocus(&self, focus: bool);
    fn getSTCFocus(&self) -> bool;
    fn setStatus(&self, statusCode: int);
    fn getStatus(&self) -> int;
    fn setMouseDownCaptures(&self, captures: bool);
    fn getMouseDownCaptures(&self) -> bool;
    fn setSTCCursor(&self, cursorType: int);
    fn getSTCCursor(&self) -> int;
    fn setControlCharSymbol(&self, symbol: int);
    fn getControlCharSymbol(&self) -> int;
    fn wordPartLeft(&self);
    fn wordPartLeftExtend(&self);
    fn wordPartRight(&self);
    fn wordPartRightExtend(&self);
    fn setVisiblePolicy(&self, visiblePolicy: int, visibleSlop: int);
    fn delLineLeft(&self);
    fn delLineRight(&self);
    fn setXOffset(&self, newOffset: int);
    fn getXOffset(&self) -> int;
    fn chooseCaretX(&self);
    fn setXCaretPolicy(&self, caretPolicy: int, caretSlop: int);
    fn setYCaretPolicy(&self, caretPolicy: int, caretSlop: int);
    fn setPrintWrapMode(&self, mode: int);
    fn getPrintWrapMode(&self) -> int;
    fn setHotspotActiveForeground(&self, useSetting: bool, fore_r: u8, fore_g: u8, fore_b: u8);
    fn setHotspotActiveBackground(&self, useSetting: bool, back_r: u8, back_g: u8, back_b: u8);
    fn setHotspotActiveUnderline(&self, underline: bool);
    fn positionBefore(&self, pos: int) -> int;
    fn positionAfter(&self, pos: int) -> int;
    fn copyRange(&self, start: int, end: int);
    fn copyText(&self, length: int, text: @wxString);
    fn startRecord(&self);
    fn stopRecord(&self);
    fn setLexer(&self, lexer: int);
    fn getLexer(&self) -> int;
    fn colourise(&self, start: int, end: int);
    fn setProperty(&self, key: @wxString, value: @wxString);
    fn setKeyWords(&self, keywordSet: int, keyWords: @wxString);
    fn setLexerLanguage(&self, language: @wxString);
    fn getCurrentLine(&self) -> int;
    fn styleSetSpec(&self, styleNum: int, spec: @wxString);
    fn styleSetFont(&self, styleNum: int, font: @wxFont);
    fn styleSetFontAttr(&self, styleNum: int, size: int, faceName: @wxString, bold: bool, italic: bool, underline: bool);
    fn cmdKeyExecute(&self, cmd: int);
    fn setMargins(&self, left: int, right: int);
    fn getSelection(&self, startPos: *int, endPos: *int);
    fn scrollToLine(&self, line: int);
    fn scrollToColumn(&self, column: int);
    fn setVScrollBar(&self, bar: @wxScrollBar);
    fn setHScrollBar(&self, bar: @wxScrollBar);
    fn getLastKeydownProcessed(&self) -> bool;
    fn setLastKeydownProcessed(&self, val: bool);
    fn saveFile(&self, filename: @wxString) -> bool;
    fn loadFile(&self, filename: @wxString) -> bool;
    fn new(_prt: @wxWindow, _id: int, _txt: @wxString, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: int) -> @wxStyledTextCtrl;
    fn indicatorGetForeground(&self, indic: int) -> @wxColour;
    fn getCaretLineBackground(&self) -> @wxColour;
    fn setCaretLineBackground(&self, back_r: u8, back_g: u8, back_b: u8);
    fn getCaretForeground(&self) -> @wxColour;
    fn getLine(&self, line: int) -> @wxString;
    fn getText(&self) -> @wxString;
    fn getTextRange(&self, startPos: int, endPos: int) -> @wxString;
    fn getSelectedText(&self) -> @wxString;
    fn newDocument(&self) -> @wxSTCDoc;
    fn getEdgeColour(&self) -> @wxColour;
    fn getDocPointer(&self) -> @wxSTCDoc;
    fn pointFromPosition(&self) -> @wxPoint;
}
trait wxSTCDoc {
}
trait wxMemoryBuffer {
}
trait wxStyledTextEvent {
    fn getPosition(&self) -> int;
    fn getKey(&self) -> int;
    fn getModifiers(&self) -> int;
    fn getModificationType(&self) -> int;
    fn getLength(&self) -> int;
    fn getLinesAdded(&self) -> int;
    fn getLine(&self) -> int;
    fn getFoldLevelNow(&self) -> int;
    fn getFoldLevelPrev(&self) -> int;
    fn getMargin(&self) -> int;
    fn getMessage(&self) -> int;
    fn getWParam(&self) -> int;
    fn getLParam(&self) -> int;
    fn getListType(&self) -> int;
    fn getX(&self) -> int;
    fn getY(&self) -> int;
    fn getDragText(&self) -> @wxString;
    fn getDragAllowMove(&self) -> bool;
    fn getDragResult(&self) -> int;
    fn getShift(&self) -> bool;
    fn getControl(&self) -> bool;
    fn getAlt(&self) -> bool;
    fn getText(&self) -> @wxString;
    fn clone(&self) -> @wxStyledTextEvent;
    fn setPosition(&self, pos: int);
    fn setKey(&self, k: int);
    fn setModifiers(&self, m: int);
    fn setModificationType(&self, t: int);
    fn setText(&self, t: @wxString);
    fn setLength(&self, len: int);
    fn setLinesAdded(&self, num: int);
    fn setLine(&self, val: int);
    fn setFoldLevelNow(&self, val: int);
    fn setFoldLevelPrev(&self, val: int);
    fn setMargin(&self, val: int);
    fn setMessage(&self, val: int);
    fn setWParam(&self, val: int);
    fn setLParam(&self, val: int);
    fn setListType(&self, val: int);
    fn setX(&self, val: int);
    fn setY(&self, val: int);
    fn setDragText(&self, val: @wxString);
    fn setDragAllowMove(&self, val: bool);
    fn setDragResult(&self, val: int);
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
    fn new(closure: @wxClosure) -> @wxcTreeItemData;
    fn getClientClosure(&self) -> @wxClosure;
    fn setClientClosure(&self, closure: @wxClosure);
}
trait wxInputSink {
    fn new(input: @wxInputStream, evtHandler: @wxEvtHandler, bufferLen: int) -> @wxInputSink;
    fn getId(&self) -> int;
    fn start(&self);
}
trait wxInputSinkEvent {
    fn lastError(&self) -> int;
    fn lastRead(&self) -> int;
    fn lastInput(&self) -> *char;
}
trait wxcHtmlEvent {
    fn getMouseEvent(&self) -> @wxMouseEvent;
    fn getHtmlCell(&self) -> @wxHtmlCell;
    fn getHtmlCellId(&self) -> @wxString;
    fn getHref(&self) -> @wxString;
    fn getTarget(&self) -> @wxString;
    fn getLogicalPosition(&self) -> @wxPoint;
}
trait wxcHtmlWindow {
    fn new(_prt: @wxWindow, _id: int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: int, _txt: @wxString) -> @wxcHtmlWindow;
}
trait wxGridCellTextEnterEditor {
    fn ctor() -> @wxGridCellTextEnterEditor;
}
trait wxFileConfig {
    fn new(inp: @wxInputStream) -> @wxFileConfig;
}
