use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct wxHtmlCell { handle: *mut c_void }
impl _wxHtmlCell for wxHtmlCell {}
impl _wxObject for wxHtmlCell { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlCell {
    pub fn from(handle: *mut c_void) -> wxHtmlCell { wxHtmlCell { handle: handle } }
    pub fn null() -> wxHtmlCell { wxHtmlCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlCell : _wxObject {
}

pub struct wxHtmlColourCell { handle: *mut c_void }
impl _wxHtmlColourCell for wxHtmlColourCell {}
impl _wxHtmlCell for wxHtmlColourCell {}
impl _wxObject for wxHtmlColourCell { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlColourCell {
    pub fn from(handle: *mut c_void) -> wxHtmlColourCell { wxHtmlColourCell { handle: handle } }
    pub fn null() -> wxHtmlColourCell { wxHtmlColourCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlColourCell : _wxHtmlCell {
}

pub struct wxHtmlContainerCell { handle: *mut c_void }
impl _wxHtmlContainerCell for wxHtmlContainerCell {}
impl _wxHtmlCell for wxHtmlContainerCell {}
impl _wxObject for wxHtmlContainerCell { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlContainerCell {
    pub fn from(handle: *mut c_void) -> wxHtmlContainerCell { wxHtmlContainerCell { handle: handle } }
    pub fn null() -> wxHtmlContainerCell { wxHtmlContainerCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlContainerCell : _wxHtmlCell {
}

pub struct wxHtmlDCRenderer { handle: *mut c_void }
impl _wxHtmlDCRenderer for wxHtmlDCRenderer {}
impl _wxObject for wxHtmlDCRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlDCRenderer {
    pub fn from(handle: *mut c_void) -> wxHtmlDCRenderer { wxHtmlDCRenderer { handle: handle } }
    pub fn null() -> wxHtmlDCRenderer { wxHtmlDCRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlDCRenderer : _wxObject {
}

pub struct wxHtmlEasyPrinting { handle: *mut c_void }
impl _wxHtmlEasyPrinting for wxHtmlEasyPrinting {}
impl _wxObject for wxHtmlEasyPrinting { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlEasyPrinting {
    pub fn from(handle: *mut c_void) -> wxHtmlEasyPrinting { wxHtmlEasyPrinting { handle: handle } }
    pub fn null() -> wxHtmlEasyPrinting { wxHtmlEasyPrinting::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlEasyPrinting : _wxObject {
}

pub struct wxHtmlFilter { handle: *mut c_void }
impl _wxHtmlFilter for wxHtmlFilter {}
impl _wxObject for wxHtmlFilter { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlFilter {
    pub fn from(handle: *mut c_void) -> wxHtmlFilter { wxHtmlFilter { handle: handle } }
    pub fn null() -> wxHtmlFilter { wxHtmlFilter::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlFilter : _wxObject {
}

pub struct wxHtmlHelpController { handle: *mut c_void }
impl _wxHtmlHelpController for wxHtmlHelpController {}
impl _wxHelpControllerBase for wxHtmlHelpController {}
impl _wxObject for wxHtmlHelpController { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlHelpController {
    pub fn from(handle: *mut c_void) -> wxHtmlHelpController { wxHtmlHelpController { handle: handle } }
    pub fn null() -> wxHtmlHelpController { wxHtmlHelpController::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(_style: c_int) -> wxHtmlHelpController {
        unsafe { wxHtmlHelpController { handle: wxHtmlHelpController_Create(_style) } }
    }
}

pub trait _wxHtmlHelpController : _wxHelpControllerBase {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addBook(&self, book: *mut c_void, show_wait_msg: c_int) -> c_int {
        unsafe { wxHtmlHelpController_AddBook(self.handle(), book, show_wait_msg) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn display(&self, x: *mut c_void) -> c_int {
        unsafe { wxHtmlHelpController_Display(self.handle(), x) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displayBlock(&self, blockNo: c_int) -> c_int {
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
    fn displaySection(&self, section: &str) -> c_int {
        let section = wxT(section);
        unsafe { wxHtmlHelpController_DisplaySection(self.handle(), section.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn displaySectionNumber(&self, sectionNo: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.handle(), sectionNo) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrame(&self) -> wxFrame {
        unsafe { wxFrame { handle: wxHtmlHelpController_GetFrame(self.handle()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFrameParameters(&self, title: *mut c_void, width: *mut c_int, height: *mut c_int, pos_x: *mut c_int, pos_y: *mut c_int, newFrameEachTime: *mut c_int) -> *mut c_void {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.handle(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initialize(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxHtmlHelpController_Initialize(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn keywordSearch(&self, keyword: &str) -> c_int {
        let keyword = wxT(keyword);
        unsafe { wxHtmlHelpController_KeywordSearch(self.handle(), keyword.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFile(&self, file: &str) -> c_int {
        let file = wxT(file);
        unsafe { wxHtmlHelpController_LoadFile(self.handle(), file.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn quit(&self) -> c_int {
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
    fn setFrameParameters(&self, title: *mut c_void, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: c_int) {
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
    fn setTitleFormat(&self, format: *mut c_void) {
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

pub struct wxHtmlHelpData { handle: *mut c_void }
impl _wxHtmlHelpData for wxHtmlHelpData {}
impl _wxObject for wxHtmlHelpData { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlHelpData {
    pub fn from(handle: *mut c_void) -> wxHtmlHelpData { wxHtmlHelpData { handle: handle } }
    pub fn null() -> wxHtmlHelpData { wxHtmlHelpData::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlHelpData : _wxObject {
}

pub struct wxHtmlHelpFrame { handle: *mut c_void }
impl _wxHtmlHelpFrame for wxHtmlHelpFrame {}
impl _wxFrame for wxHtmlHelpFrame {}
impl _wxTopLevelWindow for wxHtmlHelpFrame {}
impl _wxWindow for wxHtmlHelpFrame {}
impl _wxEvtHandler for wxHtmlHelpFrame {}
impl _wxObject for wxHtmlHelpFrame { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlHelpFrame {
    pub fn from(handle: *mut c_void) -> wxHtmlHelpFrame { wxHtmlHelpFrame { handle: handle } }
    pub fn null() -> wxHtmlHelpFrame { wxHtmlHelpFrame::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlHelpFrame : _wxFrame {
}

pub struct wxHtmlLinkInfo { handle: *mut c_void }
impl _wxHtmlLinkInfo for wxHtmlLinkInfo {}
impl _wxObject for wxHtmlLinkInfo { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlLinkInfo {
    pub fn from(handle: *mut c_void) -> wxHtmlLinkInfo { wxHtmlLinkInfo { handle: handle } }
    pub fn null() -> wxHtmlLinkInfo { wxHtmlLinkInfo::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlLinkInfo : _wxObject {
}

pub struct wxHtmlParser { handle: *mut c_void }
impl _wxHtmlParser for wxHtmlParser {}
impl _wxObject for wxHtmlParser { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlParser {
    pub fn from(handle: *mut c_void) -> wxHtmlParser { wxHtmlParser { handle: handle } }
    pub fn null() -> wxHtmlParser { wxHtmlParser::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlParser : _wxObject {
}

pub struct wxHtmlPrintout { handle: *mut c_void }
impl _wxHtmlPrintout for wxHtmlPrintout {}
impl _wxPrintout for wxHtmlPrintout {}
impl _wxObject for wxHtmlPrintout { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlPrintout {
    pub fn from(handle: *mut c_void) -> wxHtmlPrintout { wxHtmlPrintout { handle: handle } }
    pub fn null() -> wxHtmlPrintout { wxHtmlPrintout::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlPrintout : _wxPrintout {
}

pub struct wxHtmlTag { handle: *mut c_void }
impl _wxHtmlTag for wxHtmlTag {}
impl _wxObject for wxHtmlTag { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlTag {
    pub fn from(handle: *mut c_void) -> wxHtmlTag { wxHtmlTag { handle: handle } }
    pub fn null() -> wxHtmlTag { wxHtmlTag::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlTag : _wxObject {
}

pub struct wxHtmlTagHandler { handle: *mut c_void }
impl _wxHtmlTagHandler for wxHtmlTagHandler {}
impl _wxObject for wxHtmlTagHandler { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlTagHandler {
    pub fn from(handle: *mut c_void) -> wxHtmlTagHandler { wxHtmlTagHandler { handle: handle } }
    pub fn null() -> wxHtmlTagHandler { wxHtmlTagHandler::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlTagHandler : _wxObject {
}

pub struct wxHtmlTagsModule { handle: *mut c_void }
impl _wxHtmlTagsModule for wxHtmlTagsModule {}
impl _wxModule for wxHtmlTagsModule {}
impl _wxObject for wxHtmlTagsModule { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlTagsModule {
    pub fn from(handle: *mut c_void) -> wxHtmlTagsModule { wxHtmlTagsModule { handle: handle } }
    pub fn null() -> wxHtmlTagsModule { wxHtmlTagsModule::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlTagsModule : _wxModule {
}

pub struct wxHtmlWidgetCell { handle: *mut c_void }
impl _wxHtmlWidgetCell for wxHtmlWidgetCell {}
impl _wxHtmlCell for wxHtmlWidgetCell {}
impl _wxObject for wxHtmlWidgetCell { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlWidgetCell {
    pub fn from(handle: *mut c_void) -> wxHtmlWidgetCell { wxHtmlWidgetCell { handle: handle } }
    pub fn null() -> wxHtmlWidgetCell { wxHtmlWidgetCell::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlWidgetCell : _wxHtmlCell {
}

pub struct wxHtmlWinParser { handle: *mut c_void }
impl _wxHtmlWinParser for wxHtmlWinParser {}
impl _wxHtmlParser for wxHtmlWinParser {}
impl _wxObject for wxHtmlWinParser { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlWinParser {
    pub fn from(handle: *mut c_void) -> wxHtmlWinParser { wxHtmlWinParser { handle: handle } }
    pub fn null() -> wxHtmlWinParser { wxHtmlWinParser::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlWinParser : _wxHtmlParser {
}

pub struct wxHtmlWinTagHandler { handle: *mut c_void }
impl _wxHtmlWinTagHandler for wxHtmlWinTagHandler {}
impl _wxHtmlTagHandler for wxHtmlWinTagHandler {}
impl _wxObject for wxHtmlWinTagHandler { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlWinTagHandler {
    pub fn from(handle: *mut c_void) -> wxHtmlWinTagHandler { wxHtmlWinTagHandler { handle: handle } }
    pub fn null() -> wxHtmlWinTagHandler { wxHtmlWinTagHandler::from(0 as *mut c_void) }
    
}

pub trait _wxHtmlWinTagHandler : _wxHtmlTagHandler {
}

pub struct wxHtmlWindow { handle: *mut c_void }
impl _wxHtmlWindow for wxHtmlWindow {}
impl _wxScrolledWindow for wxHtmlWindow {}
impl _wxPanel for wxHtmlWindow {}
impl _wxWindow for wxHtmlWindow {}
impl _wxEvtHandler for wxHtmlWindow {}
impl _wxObject for wxHtmlWindow { fn handle(&self) -> *mut c_void { self.handle } }

impl wxHtmlWindow {
    pub fn from(handle: *mut c_void) -> wxHtmlWindow { wxHtmlWindow { handle: handle } }
    pub fn null() -> wxHtmlWindow { wxHtmlWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> wxHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { wxHtmlWindow { handle: wxHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle()) } }
    }
}

pub trait _wxHtmlWindow : _wxScrolledWindow {
    #[fixed_stack_segment]
    #[inline(never)]
    fn appendToPage(&self, source: &str) -> c_int {
        let source = wxT(source);
        unsafe { wxHtmlWindow_AppendToPage(self.handle(), source.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getInternalRepresentation(&self) -> wxHtmlContainerCell {
        unsafe { wxHtmlContainerCell { handle: wxHtmlWindow_GetInternalRepresentation(self.handle()) } }
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
    fn getRelatedFrame(&self) -> wxFrame {
        unsafe { wxFrame { handle: wxHtmlWindow_GetRelatedFrame(self.handle()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyBack(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryBack(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyCanBack(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryCanBack(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyCanForward(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryCanForward(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyClear(&self) {
        unsafe { wxHtmlWindow_HistoryClear(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn historyForward(&self) -> c_int {
        unsafe { wxHtmlWindow_HistoryForward(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadPage(&self, location: &str) -> c_int {
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
    fn setFonts(&self, normal_face: &str, fixed_face: &str, sizes: *mut c_int) {
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

pub struct wxcHtmlEvent { handle: *mut c_void }
impl _wxcHtmlEvent for wxcHtmlEvent {}
impl _wxCommandEvent for wxcHtmlEvent {}
impl _wxEvent for wxcHtmlEvent {}
impl _wxObject for wxcHtmlEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxcHtmlEvent {
    pub fn from(handle: *mut c_void) -> wxcHtmlEvent { wxcHtmlEvent { handle: handle } }
    pub fn null() -> wxcHtmlEvent { wxcHtmlEvent::from(0 as *mut c_void) }
    
}

pub trait _wxcHtmlEvent : _wxCommandEvent {
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMouseEvent(&self) -> wxMouseEvent {
        unsafe { wxMouseEvent { handle: wxcHtmlEvent_GetMouseEvent(self.handle()) } }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHtmlCell(&self) -> wxHtmlCell {
        unsafe { wxHtmlCell { handle: wxcHtmlEvent_GetHtmlCell(self.handle()) } }
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
    fn getLogicalPosition(&self) -> wxPoint {
        unsafe { wxPoint { handle: wxcHtmlEvent_GetLogicalPosition(self.handle()) } }
    }
}

pub struct wxcHtmlWindow { handle: *mut c_void }
impl _wxcHtmlWindow for wxcHtmlWindow {}
impl _wxHtmlWindow for wxcHtmlWindow {}
impl _wxScrolledWindow for wxcHtmlWindow {}
impl _wxPanel for wxcHtmlWindow {}
impl _wxWindow for wxcHtmlWindow {}
impl _wxEvtHandler for wxcHtmlWindow {}
impl _wxObject for wxcHtmlWindow { fn handle(&self) -> *mut c_void { self.handle } }

impl wxcHtmlWindow {
    pub fn from(handle: *mut c_void) -> wxcHtmlWindow { wxcHtmlWindow { handle: handle } }
    pub fn null() -> wxcHtmlWindow { wxcHtmlWindow::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> wxcHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { wxcHtmlWindow { handle: wxcHtmlWindow_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.handle()) } }
    }
}

pub trait _wxcHtmlWindow : _wxHtmlWindow {
}

