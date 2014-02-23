use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct WxHtmlCell { ptr: *mut c_void }
impl TWxHtmlCell for WxHtmlCell {}
impl TWxObject for WxHtmlCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlCell {
    pub fn from(ptr: *mut c_void) -> WxHtmlCell { WxHtmlCell { ptr: ptr } }
    pub fn null() -> WxHtmlCell { WxHtmlCell::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlCell : TWxObject {
}

pub struct WxHtmlColourCell { ptr: *mut c_void }
impl TWxHtmlColourCell for WxHtmlColourCell {}
impl TWxHtmlCell for WxHtmlColourCell {}
impl TWxObject for WxHtmlColourCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlColourCell {
    pub fn from(ptr: *mut c_void) -> WxHtmlColourCell { WxHtmlColourCell { ptr: ptr } }
    pub fn null() -> WxHtmlColourCell { WxHtmlColourCell::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlColourCell : TWxHtmlCell {
}

pub struct WxHtmlContainerCell { ptr: *mut c_void }
impl TWxHtmlContainerCell for WxHtmlContainerCell {}
impl TWxHtmlCell for WxHtmlContainerCell {}
impl TWxObject for WxHtmlContainerCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlContainerCell {
    pub fn from(ptr: *mut c_void) -> WxHtmlContainerCell { WxHtmlContainerCell { ptr: ptr } }
    pub fn null() -> WxHtmlContainerCell { WxHtmlContainerCell::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlContainerCell : TWxHtmlCell {
}

pub struct WxHtmlDCRenderer { ptr: *mut c_void }
impl TWxHtmlDCRenderer for WxHtmlDCRenderer {}
impl TWxObject for WxHtmlDCRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlDCRenderer {
    pub fn from(ptr: *mut c_void) -> WxHtmlDCRenderer { WxHtmlDCRenderer { ptr: ptr } }
    pub fn null() -> WxHtmlDCRenderer { WxHtmlDCRenderer::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlDCRenderer : TWxObject {
}

pub struct WxHtmlEasyPrinting { ptr: *mut c_void }
impl TWxHtmlEasyPrinting for WxHtmlEasyPrinting {}
impl TWxObject for WxHtmlEasyPrinting { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlEasyPrinting {
    pub fn from(ptr: *mut c_void) -> WxHtmlEasyPrinting { WxHtmlEasyPrinting { ptr: ptr } }
    pub fn null() -> WxHtmlEasyPrinting { WxHtmlEasyPrinting::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlEasyPrinting : TWxObject {
}

pub struct WxHtmlFilter { ptr: *mut c_void }
impl TWxHtmlFilter for WxHtmlFilter {}
impl TWxObject for WxHtmlFilter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlFilter {
    pub fn from(ptr: *mut c_void) -> WxHtmlFilter { WxHtmlFilter { ptr: ptr } }
    pub fn null() -> WxHtmlFilter { WxHtmlFilter::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlFilter : TWxObject {
}

pub struct WxHtmlHelpController { ptr: *mut c_void }
impl TWxHtmlHelpController for WxHtmlHelpController {}
impl TWxHelpControllerBase for WxHtmlHelpController {}
impl TWxObject for WxHtmlHelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlHelpController {
    pub fn from(ptr: *mut c_void) -> WxHtmlHelpController { WxHtmlHelpController { ptr: ptr } }
    pub fn null() -> WxHtmlHelpController { WxHtmlHelpController::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> WxHtmlHelpController {
        unsafe { WxHtmlHelpController { ptr: wxHtmlHelpController_Create(_style) } }
    }
}

pub trait TWxHtmlHelpController : TWxHelpControllerBase {
    fn addBook(&self, book: *mut c_void, show_wait_msg: c_int) -> c_int {
        unsafe { wxHtmlHelpController_AddBook(self.ptr(), book, show_wait_msg) }
    }
    fn display(&self, x: *mut c_void) -> c_int {
        unsafe { wxHtmlHelpController_Display(self.ptr(), x) }
    }
    fn displayBlock(&self, blockNo: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplayBlock(self.ptr(), blockNo) }
    }
    fn displayContents(&self) -> c_int {
        unsafe { wxHtmlHelpController_DisplayContents(self.ptr()) }
    }
    fn displayIndex(&self) -> c_int {
        unsafe { wxHtmlHelpController_DisplayIndex(self.ptr()) }
    }
    fn displayNumber(&self, id: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplayNumber(self.ptr(), id) }
    }
    fn displaySection(&self, section: &str) -> c_int {
        let section = wxT(section);
        unsafe { wxHtmlHelpController_DisplaySection(self.ptr(), section.ptr()) }
    }
    fn displaySectionNumber(&self, sectionNo: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.ptr(), sectionNo) }
    }
    fn getFrame(&self) -> WxFrame {
        unsafe { WxFrame { ptr: wxHtmlHelpController_GetFrame(self.ptr()) } }
    }
    fn getFrameParameters(&self, title: *mut c_void, width: *mut c_int, height: *mut c_int, pos_x: *mut c_int, pos_y: *mut c_int, newFrameEachTime: *mut c_int) -> *mut c_void {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.ptr(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    fn initialize(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxHtmlHelpController_Initialize(self.ptr(), file.ptr()) }
    }
    fn keywordSearch(&self, keyword: &str) -> c_int {
        let keyword = wxT(keyword);
        unsafe { wxHtmlHelpController_KeywordSearch(self.ptr(), keyword.ptr()) }
    }
    fn loadFile(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxHtmlHelpController_LoadFile(self.ptr(), file.ptr()) }
    }
    fn quit(&self) -> c_int {
        unsafe { wxHtmlHelpController_Quit(self.ptr()) }
    }
    fn readCustomization<T: TWxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_ReadCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
    fn setFrameParameters(&self, title: *mut c_void, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: c_int) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.ptr(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    fn setTempDir(&self, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_SetTempDir(self.ptr(), path.ptr()) }
    }
    fn setTitleFormat(&self, format: *mut c_void) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.ptr(), format) }
    }
    fn setViewer(&self, viewer: &str, flags: c_int) {
        let viewer = wxT(viewer);
        unsafe { wxHtmlHelpController_SetViewer(self.ptr(), viewer.ptr(), flags) }
    }
    fn useConfig<T: TWxConfigBase>(&self, config: &T, rootpath: &str) {
        let rootpath = wxT(rootpath);
        unsafe { wxHtmlHelpController_UseConfig(self.ptr(), config.ptr(), rootpath.ptr()) }
    }
    fn writeCustomization<T: TWxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

pub struct WxHtmlHelpData { ptr: *mut c_void }
impl TWxHtmlHelpData for WxHtmlHelpData {}
impl TWxObject for WxHtmlHelpData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlHelpData {
    pub fn from(ptr: *mut c_void) -> WxHtmlHelpData { WxHtmlHelpData { ptr: ptr } }
    pub fn null() -> WxHtmlHelpData { WxHtmlHelpData::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlHelpData : TWxObject {
}

pub struct WxHtmlHelpFrame { ptr: *mut c_void }
impl TWxHtmlHelpFrame for WxHtmlHelpFrame {}
impl TWxFrame for WxHtmlHelpFrame {}
impl TWxTopLevelWindow for WxHtmlHelpFrame {}
impl TWxWindow for WxHtmlHelpFrame {}
impl TWxEvtHandler for WxHtmlHelpFrame {}
impl TWxObject for WxHtmlHelpFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlHelpFrame {
    pub fn from(ptr: *mut c_void) -> WxHtmlHelpFrame { WxHtmlHelpFrame { ptr: ptr } }
    pub fn null() -> WxHtmlHelpFrame { WxHtmlHelpFrame::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlHelpFrame : TWxFrame {
}

pub struct WxHtmlLinkInfo { ptr: *mut c_void }
impl TWxHtmlLinkInfo for WxHtmlLinkInfo {}
impl TWxObject for WxHtmlLinkInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlLinkInfo {
    pub fn from(ptr: *mut c_void) -> WxHtmlLinkInfo { WxHtmlLinkInfo { ptr: ptr } }
    pub fn null() -> WxHtmlLinkInfo { WxHtmlLinkInfo::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlLinkInfo : TWxObject {
}

pub struct WxHtmlParser { ptr: *mut c_void }
impl TWxHtmlParser for WxHtmlParser {}
impl TWxObject for WxHtmlParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlParser {
    pub fn from(ptr: *mut c_void) -> WxHtmlParser { WxHtmlParser { ptr: ptr } }
    pub fn null() -> WxHtmlParser { WxHtmlParser::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlParser : TWxObject {
}

pub struct WxHtmlPrintout { ptr: *mut c_void }
impl TWxHtmlPrintout for WxHtmlPrintout {}
impl TWxPrintout for WxHtmlPrintout {}
impl TWxObject for WxHtmlPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlPrintout {
    pub fn from(ptr: *mut c_void) -> WxHtmlPrintout { WxHtmlPrintout { ptr: ptr } }
    pub fn null() -> WxHtmlPrintout { WxHtmlPrintout::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlPrintout : TWxPrintout {
}

pub struct WxHtmlTag { ptr: *mut c_void }
impl TWxHtmlTag for WxHtmlTag {}
impl TWxObject for WxHtmlTag { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlTag {
    pub fn from(ptr: *mut c_void) -> WxHtmlTag { WxHtmlTag { ptr: ptr } }
    pub fn null() -> WxHtmlTag { WxHtmlTag::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlTag : TWxObject {
}

pub struct WxHtmlTagHandler { ptr: *mut c_void }
impl TWxHtmlTagHandler for WxHtmlTagHandler {}
impl TWxObject for WxHtmlTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlTagHandler {
    pub fn from(ptr: *mut c_void) -> WxHtmlTagHandler { WxHtmlTagHandler { ptr: ptr } }
    pub fn null() -> WxHtmlTagHandler { WxHtmlTagHandler::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlTagHandler : TWxObject {
}

pub struct WxHtmlTagsModule { ptr: *mut c_void }
impl TWxHtmlTagsModule for WxHtmlTagsModule {}
impl TWxModule for WxHtmlTagsModule {}
impl TWxObject for WxHtmlTagsModule { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlTagsModule {
    pub fn from(ptr: *mut c_void) -> WxHtmlTagsModule { WxHtmlTagsModule { ptr: ptr } }
    pub fn null() -> WxHtmlTagsModule { WxHtmlTagsModule::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlTagsModule : TWxModule {
}

pub struct WxHtmlWidgetCell { ptr: *mut c_void }
impl TWxHtmlWidgetCell for WxHtmlWidgetCell {}
impl TWxHtmlCell for WxHtmlWidgetCell {}
impl TWxObject for WxHtmlWidgetCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlWidgetCell {
    pub fn from(ptr: *mut c_void) -> WxHtmlWidgetCell { WxHtmlWidgetCell { ptr: ptr } }
    pub fn null() -> WxHtmlWidgetCell { WxHtmlWidgetCell::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlWidgetCell : TWxHtmlCell {
}

pub struct WxHtmlWinParser { ptr: *mut c_void }
impl TWxHtmlWinParser for WxHtmlWinParser {}
impl TWxHtmlParser for WxHtmlWinParser {}
impl TWxObject for WxHtmlWinParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlWinParser {
    pub fn from(ptr: *mut c_void) -> WxHtmlWinParser { WxHtmlWinParser { ptr: ptr } }
    pub fn null() -> WxHtmlWinParser { WxHtmlWinParser::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlWinParser : TWxHtmlParser {
}

pub struct WxHtmlWinTagHandler { ptr: *mut c_void }
impl TWxHtmlWinTagHandler for WxHtmlWinTagHandler {}
impl TWxHtmlTagHandler for WxHtmlWinTagHandler {}
impl TWxObject for WxHtmlWinTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlWinTagHandler {
    pub fn from(ptr: *mut c_void) -> WxHtmlWinTagHandler { WxHtmlWinTagHandler { ptr: ptr } }
    pub fn null() -> WxHtmlWinTagHandler { WxHtmlWinTagHandler::from(0 as *mut c_void) }
    
}

pub trait TWxHtmlWinTagHandler : TWxHtmlTagHandler {
}

pub struct WxHtmlWindow { ptr: *mut c_void }
impl TWxHtmlWindow for WxHtmlWindow {}
impl TWxScrolledWindow for WxHtmlWindow {}
impl TWxPanel for WxHtmlWindow {}
impl TWxWindow for WxHtmlWindow {}
impl TWxEvtHandler for WxHtmlWindow {}
impl TWxObject for WxHtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxHtmlWindow {
    pub fn from(ptr: *mut c_void) -> WxHtmlWindow { WxHtmlWindow { ptr: ptr } }
    pub fn null() -> WxHtmlWindow { WxHtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> WxHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { WxHtmlWindow { ptr: wxHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr()) } }
    }
}

pub trait TWxHtmlWindow : TWxScrolledWindow {
    fn appendToPage(&self, source: &str) -> c_int {
        let source = wxT(source);
        unsafe { wxHtmlWindow_AppendToPage(self.ptr(), source.ptr()) }
    }
    fn getInternalRepresentation(&self) -> WxHtmlContainerCell {
        unsafe { WxHtmlContainerCell { ptr: wxHtmlWindow_GetInternalRepresentation(self.ptr()) } }
    }
    fn getOpenedAnchor(&self) -> ~str {
        unsafe { WxString { ptr: wxHtmlWindow_GetOpenedAnchor(self.ptr()) }.to_str() }
    }
    fn getOpenedPage(&self) -> ~str {
        unsafe { WxString { ptr: wxHtmlWindow_GetOpenedPage(self.ptr()) }.to_str() }
    }
    fn getOpenedPageTitle(&self) -> ~str {
        unsafe { WxString { ptr: wxHtmlWindow_GetOpenedPageTitle(self.ptr()) }.to_str() }
    }
    fn getRelatedFrame(&self) -> WxFrame {
        unsafe { WxFrame { ptr: wxHtmlWindow_GetRelatedFrame(self.ptr()) } }
    }
    fn historyBack(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryBack(self.ptr()) }
    }
    fn historyCanBack(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryCanBack(self.ptr()) }
    }
    fn historyCanForward(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryCanForward(self.ptr()) }
    }
    fn historyClear(&self) {
        unsafe { wxHtmlWindow_HistoryClear(self.ptr()) }
    }
    fn historyForward(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryForward(self.ptr()) }
    }
    fn loadPage(&self, location: &str) -> c_int {
        let location = wxT(location);
        unsafe { wxHtmlWindow_LoadPage(self.ptr(), location.ptr()) }
    }
    fn readCustomization<T: TWxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlWindow_ReadCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.ptr(), b) }
    }
    fn setFonts(&self, normal_face: &str, fixed_face: &str, sizes: *mut c_int) {
        let normal_face = wxT(normal_face);
        let fixed_face = wxT(fixed_face);
        unsafe { wxHtmlWindow_SetFonts(self.ptr(), normal_face.ptr(), fixed_face.ptr(), sizes) }
    }
    fn setPage(&self, source: &str) {
        let source = wxT(source);
        unsafe { wxHtmlWindow_SetPage(self.ptr(), source.ptr()) }
    }
    fn setRelatedFrame<T: TWxFrame>(&self, frame: &T, format: &str) {
        let format = wxT(format);
        unsafe { wxHtmlWindow_SetRelatedFrame(self.ptr(), frame.ptr(), format.ptr()) }
    }
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.ptr(), bar) }
    }
    fn writeCustomization<T: TWxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlWindow_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

pub struct WxcHtmlEvent { ptr: *mut c_void }
impl TWxcHtmlEvent for WxcHtmlEvent {}
impl TWxCommandEvent for WxcHtmlEvent {}
impl TWxEvent for WxcHtmlEvent {}
impl TWxObject for WxcHtmlEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxcHtmlEvent {
    pub fn from(ptr: *mut c_void) -> WxcHtmlEvent { WxcHtmlEvent { ptr: ptr } }
    pub fn null() -> WxcHtmlEvent { WxcHtmlEvent::from(0 as *mut c_void) }
    
}

pub trait TWxcHtmlEvent : TWxCommandEvent {
    fn getMouseEvent(&self) -> WxMouseEvent {
        unsafe { WxMouseEvent { ptr: wxcHtmlEvent_GetMouseEvent(self.ptr()) } }
    }
    fn getHtmlCell(&self) -> WxHtmlCell {
        unsafe { WxHtmlCell { ptr: wxcHtmlEvent_GetHtmlCell(self.ptr()) } }
    }
    fn getHtmlCellId(&self) -> ~str {
        unsafe { WxString { ptr: wxcHtmlEvent_GetHtmlCellId(self.ptr()) }.to_str() }
    }
    fn getHref(&self) -> ~str {
        unsafe { WxString { ptr: wxcHtmlEvent_GetHref(self.ptr()) }.to_str() }
    }
    fn getTarget(&self) -> ~str {
        unsafe { WxString { ptr: wxcHtmlEvent_GetTarget(self.ptr()) }.to_str() }
    }
    fn getLogicalPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxcHtmlEvent_GetLogicalPosition(self.ptr()) } }
    }
}

pub struct WxcHtmlWindow { ptr: *mut c_void }
impl TWxcHtmlWindow for WxcHtmlWindow {}
impl TWxHtmlWindow for WxcHtmlWindow {}
impl TWxScrolledWindow for WxcHtmlWindow {}
impl TWxPanel for WxcHtmlWindow {}
impl TWxWindow for WxcHtmlWindow {}
impl TWxEvtHandler for WxcHtmlWindow {}
impl TWxObject for WxcHtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxcHtmlWindow {
    pub fn from(ptr: *mut c_void) -> WxcHtmlWindow { WxcHtmlWindow { ptr: ptr } }
    pub fn null() -> WxcHtmlWindow { WxcHtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> WxcHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { WxcHtmlWindow { ptr: wxcHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr()) } }
    }
}

pub trait TWxcHtmlWindow : TWxHtmlWindow {
}

