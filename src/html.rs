use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct wxHtmlCell { ptr: *mut c_void }
impl _wxHtmlCell for wxHtmlCell {}
impl _wxObject for wxHtmlCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlCell {
    pub fn from(ptr: *mut c_void) -> wxHtmlCell { wxHtmlCell { ptr: ptr } }
    pub fn null() -> wxHtmlCell { wxHtmlCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlCell : _wxObject {
}

pub struct wxHtmlColourCell { ptr: *mut c_void }
impl _wxHtmlColourCell for wxHtmlColourCell {}
impl _wxHtmlCell for wxHtmlColourCell {}
impl _wxObject for wxHtmlColourCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlColourCell {
    pub fn from(ptr: *mut c_void) -> wxHtmlColourCell { wxHtmlColourCell { ptr: ptr } }
    pub fn null() -> wxHtmlColourCell { wxHtmlColourCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlColourCell : _wxHtmlCell {
}

pub struct wxHtmlContainerCell { ptr: *mut c_void }
impl _wxHtmlContainerCell for wxHtmlContainerCell {}
impl _wxHtmlCell for wxHtmlContainerCell {}
impl _wxObject for wxHtmlContainerCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlContainerCell {
    pub fn from(ptr: *mut c_void) -> wxHtmlContainerCell { wxHtmlContainerCell { ptr: ptr } }
    pub fn null() -> wxHtmlContainerCell { wxHtmlContainerCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlContainerCell : _wxHtmlCell {
}

pub struct wxHtmlDCRenderer { ptr: *mut c_void }
impl _wxHtmlDCRenderer for wxHtmlDCRenderer {}
impl _wxObject for wxHtmlDCRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlDCRenderer {
    pub fn from(ptr: *mut c_void) -> wxHtmlDCRenderer { wxHtmlDCRenderer { ptr: ptr } }
    pub fn null() -> wxHtmlDCRenderer { wxHtmlDCRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlDCRenderer : _wxObject {
}

pub struct wxHtmlEasyPrinting { ptr: *mut c_void }
impl _wxHtmlEasyPrinting for wxHtmlEasyPrinting {}
impl _wxObject for wxHtmlEasyPrinting { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlEasyPrinting {
    pub fn from(ptr: *mut c_void) -> wxHtmlEasyPrinting { wxHtmlEasyPrinting { ptr: ptr } }
    pub fn null() -> wxHtmlEasyPrinting { wxHtmlEasyPrinting::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlEasyPrinting : _wxObject {
}

pub struct wxHtmlFilter { ptr: *mut c_void }
impl _wxHtmlFilter for wxHtmlFilter {}
impl _wxObject for wxHtmlFilter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlFilter {
    pub fn from(ptr: *mut c_void) -> wxHtmlFilter { wxHtmlFilter { ptr: ptr } }
    pub fn null() -> wxHtmlFilter { wxHtmlFilter::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlFilter : _wxObject {
}

pub struct wxHtmlHelpController { ptr: *mut c_void }
impl _wxHtmlHelpController for wxHtmlHelpController {}
impl _wxHelpControllerBase for wxHtmlHelpController {}
impl _wxObject for wxHtmlHelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlHelpController {
    pub fn from(ptr: *mut c_void) -> wxHtmlHelpController { wxHtmlHelpController { ptr: ptr } }
    pub fn null() -> wxHtmlHelpController { wxHtmlHelpController::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> wxHtmlHelpController {
        unsafe { wxHtmlHelpController { ptr: wxHtmlHelpController_Create(_style) } }
    }
}

pub trait _wxHtmlHelpController : _wxHelpControllerBase {
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
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame { ptr: wxHtmlHelpController_GetFrame(self.ptr()) } }
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
    fn readCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
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
    fn useConfig<T: _wxConfigBase>(&self, config: &T, rootpath: &str) {
        let rootpath = wxT(rootpath);
        unsafe { wxHtmlHelpController_UseConfig(self.ptr(), config.ptr(), rootpath.ptr()) }
    }
    fn writeCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

pub struct wxHtmlHelpData { ptr: *mut c_void }
impl _wxHtmlHelpData for wxHtmlHelpData {}
impl _wxObject for wxHtmlHelpData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlHelpData {
    pub fn from(ptr: *mut c_void) -> wxHtmlHelpData { wxHtmlHelpData { ptr: ptr } }
    pub fn null() -> wxHtmlHelpData { wxHtmlHelpData::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlHelpData : _wxObject {
}

pub struct wxHtmlHelpFrame { ptr: *mut c_void }
impl _wxHtmlHelpFrame for wxHtmlHelpFrame {}
impl _wxFrame for wxHtmlHelpFrame {}
impl _wxTopLevelWindow for wxHtmlHelpFrame {}
impl _wxWindow for wxHtmlHelpFrame {}
impl _wxEvtHandler for wxHtmlHelpFrame {}
impl _wxObject for wxHtmlHelpFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlHelpFrame {
    pub fn from(ptr: *mut c_void) -> wxHtmlHelpFrame { wxHtmlHelpFrame { ptr: ptr } }
    pub fn null() -> wxHtmlHelpFrame { wxHtmlHelpFrame::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlHelpFrame : _wxFrame {
}

pub struct wxHtmlLinkInfo { ptr: *mut c_void }
impl _wxHtmlLinkInfo for wxHtmlLinkInfo {}
impl _wxObject for wxHtmlLinkInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlLinkInfo {
    pub fn from(ptr: *mut c_void) -> wxHtmlLinkInfo { wxHtmlLinkInfo { ptr: ptr } }
    pub fn null() -> wxHtmlLinkInfo { wxHtmlLinkInfo::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlLinkInfo : _wxObject {
}

pub struct wxHtmlParser { ptr: *mut c_void }
impl _wxHtmlParser for wxHtmlParser {}
impl _wxObject for wxHtmlParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlParser {
    pub fn from(ptr: *mut c_void) -> wxHtmlParser { wxHtmlParser { ptr: ptr } }
    pub fn null() -> wxHtmlParser { wxHtmlParser::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlParser : _wxObject {
}

pub struct wxHtmlPrintout { ptr: *mut c_void }
impl _wxHtmlPrintout for wxHtmlPrintout {}
impl _wxPrintout for wxHtmlPrintout {}
impl _wxObject for wxHtmlPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlPrintout {
    pub fn from(ptr: *mut c_void) -> wxHtmlPrintout { wxHtmlPrintout { ptr: ptr } }
    pub fn null() -> wxHtmlPrintout { wxHtmlPrintout::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlPrintout : _wxPrintout {
}

pub struct wxHtmlTag { ptr: *mut c_void }
impl _wxHtmlTag for wxHtmlTag {}
impl _wxObject for wxHtmlTag { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlTag {
    pub fn from(ptr: *mut c_void) -> wxHtmlTag { wxHtmlTag { ptr: ptr } }
    pub fn null() -> wxHtmlTag { wxHtmlTag::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlTag : _wxObject {
}

pub struct wxHtmlTagHandler { ptr: *mut c_void }
impl _wxHtmlTagHandler for wxHtmlTagHandler {}
impl _wxObject for wxHtmlTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlTagHandler {
    pub fn from(ptr: *mut c_void) -> wxHtmlTagHandler { wxHtmlTagHandler { ptr: ptr } }
    pub fn null() -> wxHtmlTagHandler { wxHtmlTagHandler::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlTagHandler : _wxObject {
}

pub struct wxHtmlTagsModule { ptr: *mut c_void }
impl _wxHtmlTagsModule for wxHtmlTagsModule {}
impl _wxModule for wxHtmlTagsModule {}
impl _wxObject for wxHtmlTagsModule { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlTagsModule {
    pub fn from(ptr: *mut c_void) -> wxHtmlTagsModule { wxHtmlTagsModule { ptr: ptr } }
    pub fn null() -> wxHtmlTagsModule { wxHtmlTagsModule::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlTagsModule : _wxModule {
}

pub struct wxHtmlWidgetCell { ptr: *mut c_void }
impl _wxHtmlWidgetCell for wxHtmlWidgetCell {}
impl _wxHtmlCell for wxHtmlWidgetCell {}
impl _wxObject for wxHtmlWidgetCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlWidgetCell {
    pub fn from(ptr: *mut c_void) -> wxHtmlWidgetCell { wxHtmlWidgetCell { ptr: ptr } }
    pub fn null() -> wxHtmlWidgetCell { wxHtmlWidgetCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlWidgetCell : _wxHtmlCell {
}

pub struct wxHtmlWinParser { ptr: *mut c_void }
impl _wxHtmlWinParser for wxHtmlWinParser {}
impl _wxHtmlParser for wxHtmlWinParser {}
impl _wxObject for wxHtmlWinParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlWinParser {
    pub fn from(ptr: *mut c_void) -> wxHtmlWinParser { wxHtmlWinParser { ptr: ptr } }
    pub fn null() -> wxHtmlWinParser { wxHtmlWinParser::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlWinParser : _wxHtmlParser {
}

pub struct wxHtmlWinTagHandler { ptr: *mut c_void }
impl _wxHtmlWinTagHandler for wxHtmlWinTagHandler {}
impl _wxHtmlTagHandler for wxHtmlWinTagHandler {}
impl _wxObject for wxHtmlWinTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlWinTagHandler {
    pub fn from(ptr: *mut c_void) -> wxHtmlWinTagHandler { wxHtmlWinTagHandler { ptr: ptr } }
    pub fn null() -> wxHtmlWinTagHandler { wxHtmlWinTagHandler::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlWinTagHandler : _wxHtmlTagHandler {
}

pub struct wxHtmlWindow { ptr: *mut c_void }
impl _wxHtmlWindow for wxHtmlWindow {}
impl _wxScrolledWindow for wxHtmlWindow {}
impl _wxPanel for wxHtmlWindow {}
impl _wxWindow for wxHtmlWindow {}
impl _wxEvtHandler for wxHtmlWindow {}
impl _wxObject for wxHtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxHtmlWindow {
    pub fn from(ptr: *mut c_void) -> wxHtmlWindow { wxHtmlWindow { ptr: ptr } }
    pub fn null() -> wxHtmlWindow { wxHtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> wxHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { wxHtmlWindow { ptr: wxHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr()) } }
    }
}

pub trait _wxHtmlWindow : _wxScrolledWindow {
    fn appendToPage(&self, source: &str) -> c_int {
        let source = wxT(source);
        unsafe { wxHtmlWindow_AppendToPage(self.ptr(), source.ptr()) }
    }
    fn getInternalRepresentation(&self) -> wxHtmlContainerCell {
        unsafe { wxHtmlContainerCell { ptr: wxHtmlWindow_GetInternalRepresentation(self.ptr()) } }
    }
    fn getOpenedAnchor(&self) -> ~str {
        unsafe { wxString { ptr: wxHtmlWindow_GetOpenedAnchor(self.ptr()) }.to_str() }
    }
    fn getOpenedPage(&self) -> ~str {
        unsafe { wxString { ptr: wxHtmlWindow_GetOpenedPage(self.ptr()) }.to_str() }
    }
    fn getOpenedPageTitle(&self) -> ~str {
        unsafe { wxString { ptr: wxHtmlWindow_GetOpenedPageTitle(self.ptr()) }.to_str() }
    }
    fn getRelatedFrame(&self) -> wxFrame {
        unsafe { wxFrame { ptr: wxHtmlWindow_GetRelatedFrame(self.ptr()) } }
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
    fn readCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
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
    fn setRelatedFrame<T: _wxFrame>(&self, frame: &T, format: &str) {
        let format = wxT(format);
        unsafe { wxHtmlWindow_SetRelatedFrame(self.ptr(), frame.ptr(), format.ptr()) }
    }
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.ptr(), bar) }
    }
    fn writeCustomization<T: _wxConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlWindow_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

pub struct wxcHtmlEvent { ptr: *mut c_void }
impl _wxcHtmlEvent for wxcHtmlEvent {}
impl _wxCommandEvent for wxcHtmlEvent {}
impl _wxEvent for wxcHtmlEvent {}
impl _wxObject for wxcHtmlEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxcHtmlEvent {
    pub fn from(ptr: *mut c_void) -> wxcHtmlEvent { wxcHtmlEvent { ptr: ptr } }
    pub fn null() -> wxcHtmlEvent { wxcHtmlEvent::from(0 as *mut c_void) }
    
}

pub trait _wxcHtmlEvent : _wxCommandEvent {
    fn getMouseEvent(&self) -> wxMouseEvent {
        unsafe { wxMouseEvent { ptr: wxcHtmlEvent_GetMouseEvent(self.ptr()) } }
    }
    fn getHtmlCell(&self) -> wxHtmlCell {
        unsafe { wxHtmlCell { ptr: wxcHtmlEvent_GetHtmlCell(self.ptr()) } }
    }
    fn getHtmlCellId(&self) -> ~str {
        unsafe { wxString { ptr: wxcHtmlEvent_GetHtmlCellId(self.ptr()) }.to_str() }
    }
    fn getHref(&self) -> ~str {
        unsafe { wxString { ptr: wxcHtmlEvent_GetHref(self.ptr()) }.to_str() }
    }
    fn getTarget(&self) -> ~str {
        unsafe { wxString { ptr: wxcHtmlEvent_GetTarget(self.ptr()) }.to_str() }
    }
    fn getLogicalPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxcHtmlEvent_GetLogicalPosition(self.ptr()) } }
    }
}

pub struct wxcHtmlWindow { ptr: *mut c_void }
impl _wxcHtmlWindow for wxcHtmlWindow {}
impl _wxHtmlWindow for wxcHtmlWindow {}
impl _wxScrolledWindow for wxcHtmlWindow {}
impl _wxPanel for wxcHtmlWindow {}
impl _wxWindow for wxcHtmlWindow {}
impl _wxEvtHandler for wxcHtmlWindow {}
impl _wxObject for wxcHtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxcHtmlWindow {
    pub fn from(ptr: *mut c_void) -> wxcHtmlWindow { wxcHtmlWindow { ptr: ptr } }
    pub fn null() -> wxcHtmlWindow { wxcHtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> wxcHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { wxcHtmlWindow { ptr: wxcHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr()) } }
    }
}

pub trait _wxcHtmlWindow : _wxHtmlWindow {
}

