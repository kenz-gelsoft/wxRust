use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

/// Wraps the wxWidgets' [wxHtmlCell](http://docs.wxwidgets.org/3.0/classwx_html_cell.html) class.
pub struct HtmlCell { ptr: *mut c_void }
impl THtmlCell for HtmlCell {}
impl TObject for HtmlCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlCell {
    pub fn from(ptr: *mut c_void) -> HtmlCell { HtmlCell { ptr: ptr } }
    pub fn null() -> HtmlCell { HtmlCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlCell](http://docs.wxwidgets.org/3.0/classwx_html_cell.html) class.
pub trait THtmlCell : TObject {
}

/// Wraps the wxWidgets' [wxHtmlColourCell](http://docs.wxwidgets.org/3.0/classwx_html_colour_cell.html) class.
pub struct HtmlColourCell { ptr: *mut c_void }
impl THtmlColourCell for HtmlColourCell {}
impl THtmlCell for HtmlColourCell {}
impl TObject for HtmlColourCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlColourCell {
    pub fn from(ptr: *mut c_void) -> HtmlColourCell { HtmlColourCell { ptr: ptr } }
    pub fn null() -> HtmlColourCell { HtmlColourCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlColourCell](http://docs.wxwidgets.org/3.0/classwx_html_colour_cell.html) class.
pub trait THtmlColourCell : THtmlCell {
}

/// Wraps the wxWidgets' [wxHtmlContainerCell](http://docs.wxwidgets.org/3.0/classwx_html_container_cell.html) class.
pub struct HtmlContainerCell { ptr: *mut c_void }
impl THtmlContainerCell for HtmlContainerCell {}
impl THtmlCell for HtmlContainerCell {}
impl TObject for HtmlContainerCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlContainerCell {
    pub fn from(ptr: *mut c_void) -> HtmlContainerCell { HtmlContainerCell { ptr: ptr } }
    pub fn null() -> HtmlContainerCell { HtmlContainerCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlContainerCell](http://docs.wxwidgets.org/3.0/classwx_html_container_cell.html) class.
pub trait THtmlContainerCell : THtmlCell {
}

/// Wraps the wxWidgets' [wxHtmlDCRenderer](http://docs.wxwidgets.org/3.0/classwx_html_dcr_enderer.html) class.
pub struct HtmlDCRenderer { ptr: *mut c_void }
impl THtmlDCRenderer for HtmlDCRenderer {}
impl TObject for HtmlDCRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlDCRenderer {
    pub fn from(ptr: *mut c_void) -> HtmlDCRenderer { HtmlDCRenderer { ptr: ptr } }
    pub fn null() -> HtmlDCRenderer { HtmlDCRenderer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlDCRenderer](http://docs.wxwidgets.org/3.0/classwx_html_dcr_enderer.html) class.
pub trait THtmlDCRenderer : TObject {
}

/// Wraps the wxWidgets' [wxHtmlEasyPrinting](http://docs.wxwidgets.org/3.0/classwx_html_easy_printing.html) class.
pub struct HtmlEasyPrinting { ptr: *mut c_void }
impl THtmlEasyPrinting for HtmlEasyPrinting {}
impl TObject for HtmlEasyPrinting { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlEasyPrinting {
    pub fn from(ptr: *mut c_void) -> HtmlEasyPrinting { HtmlEasyPrinting { ptr: ptr } }
    pub fn null() -> HtmlEasyPrinting { HtmlEasyPrinting::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlEasyPrinting](http://docs.wxwidgets.org/3.0/classwx_html_easy_printing.html) class.
pub trait THtmlEasyPrinting : TObject {
}

/// Wraps the wxWidgets' [wxHtmlFilter](http://docs.wxwidgets.org/3.0/classwx_html_filter.html) class.
pub struct HtmlFilter { ptr: *mut c_void }
impl THtmlFilter for HtmlFilter {}
impl TObject for HtmlFilter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlFilter {
    pub fn from(ptr: *mut c_void) -> HtmlFilter { HtmlFilter { ptr: ptr } }
    pub fn null() -> HtmlFilter { HtmlFilter::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlFilter](http://docs.wxwidgets.org/3.0/classwx_html_filter.html) class.
pub trait THtmlFilter : TObject {
}

/// Wraps the wxWidgets' [wxHtmlHelpController](http://docs.wxwidgets.org/3.0/classwx_html_help_controller.html) class.
pub struct HtmlHelpController { ptr: *mut c_void }
impl THtmlHelpController for HtmlHelpController {}
impl THelpControllerBase for HtmlHelpController {}
impl TObject for HtmlHelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlHelpController {
    pub fn from(ptr: *mut c_void) -> HtmlHelpController { HtmlHelpController { ptr: ptr } }
    pub fn null() -> HtmlHelpController { HtmlHelpController::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> HtmlHelpController {
        unsafe { HtmlHelpController { ptr: wxHtmlHelpController_Create(_style) } }
    }
}

/// Methods of the wxWidgets' [wxHtmlHelpController](http://docs.wxwidgets.org/3.0/classwx_html_help_controller.html) class.
pub trait THtmlHelpController : THelpControllerBase {
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
    fn getFrame(&self) -> Frame {
        unsafe { Frame { ptr: wxHtmlHelpController_GetFrame(self.ptr()) } }
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
    fn readCustomization<T: TConfigBase>(&self, cfg: &T, path: &str) {
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
    fn useConfig<T: TConfigBase>(&self, config: &T, rootpath: &str) {
        let rootpath = wxT(rootpath);
        unsafe { wxHtmlHelpController_UseConfig(self.ptr(), config.ptr(), rootpath.ptr()) }
    }
    fn writeCustomization<T: TConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlHelpController_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxHtmlHelpData](http://docs.wxwidgets.org/3.0/classwx_html_help_data.html) class.
pub struct HtmlHelpData { ptr: *mut c_void }
impl THtmlHelpData for HtmlHelpData {}
impl TObject for HtmlHelpData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlHelpData {
    pub fn from(ptr: *mut c_void) -> HtmlHelpData { HtmlHelpData { ptr: ptr } }
    pub fn null() -> HtmlHelpData { HtmlHelpData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlHelpData](http://docs.wxwidgets.org/3.0/classwx_html_help_data.html) class.
pub trait THtmlHelpData : TObject {
}

/// Wraps the wxWidgets' [wxHtmlHelpFrame](http://docs.wxwidgets.org/3.0/classwx_html_help_frame.html) class.
pub struct HtmlHelpFrame { ptr: *mut c_void }
impl THtmlHelpFrame for HtmlHelpFrame {}
impl TFrame for HtmlHelpFrame {}
impl TTopLevelWindow for HtmlHelpFrame {}
impl TWindow for HtmlHelpFrame {}
impl TEvtHandler for HtmlHelpFrame {}
impl TObject for HtmlHelpFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlHelpFrame {
    pub fn from(ptr: *mut c_void) -> HtmlHelpFrame { HtmlHelpFrame { ptr: ptr } }
    pub fn null() -> HtmlHelpFrame { HtmlHelpFrame::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlHelpFrame](http://docs.wxwidgets.org/3.0/classwx_html_help_frame.html) class.
pub trait THtmlHelpFrame : TFrame {
}

/// Wraps the wxWidgets' [wxHtmlLinkInfo](http://docs.wxwidgets.org/3.0/classwx_html_link_info.html) class.
pub struct HtmlLinkInfo { ptr: *mut c_void }
impl THtmlLinkInfo for HtmlLinkInfo {}
impl TObject for HtmlLinkInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlLinkInfo {
    pub fn from(ptr: *mut c_void) -> HtmlLinkInfo { HtmlLinkInfo { ptr: ptr } }
    pub fn null() -> HtmlLinkInfo { HtmlLinkInfo::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlLinkInfo](http://docs.wxwidgets.org/3.0/classwx_html_link_info.html) class.
pub trait THtmlLinkInfo : TObject {
}

/// Wraps the wxWidgets' [wxHtmlParser](http://docs.wxwidgets.org/3.0/classwx_html_parser.html) class.
pub struct HtmlParser { ptr: *mut c_void }
impl THtmlParser for HtmlParser {}
impl TObject for HtmlParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlParser {
    pub fn from(ptr: *mut c_void) -> HtmlParser { HtmlParser { ptr: ptr } }
    pub fn null() -> HtmlParser { HtmlParser::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlParser](http://docs.wxwidgets.org/3.0/classwx_html_parser.html) class.
pub trait THtmlParser : TObject {
}

/// Wraps the wxWidgets' [wxHtmlPrintout](http://docs.wxwidgets.org/3.0/classwx_html_printout.html) class.
pub struct HtmlPrintout { ptr: *mut c_void }
impl THtmlPrintout for HtmlPrintout {}
impl TPrintout for HtmlPrintout {}
impl TObject for HtmlPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlPrintout {
    pub fn from(ptr: *mut c_void) -> HtmlPrintout { HtmlPrintout { ptr: ptr } }
    pub fn null() -> HtmlPrintout { HtmlPrintout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlPrintout](http://docs.wxwidgets.org/3.0/classwx_html_printout.html) class.
pub trait THtmlPrintout : TPrintout {
}

/// Wraps the wxWidgets' [wxHtmlTag](http://docs.wxwidgets.org/3.0/classwx_html_tag.html) class.
pub struct HtmlTag { ptr: *mut c_void }
impl THtmlTag for HtmlTag {}
impl TObject for HtmlTag { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlTag {
    pub fn from(ptr: *mut c_void) -> HtmlTag { HtmlTag { ptr: ptr } }
    pub fn null() -> HtmlTag { HtmlTag::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlTag](http://docs.wxwidgets.org/3.0/classwx_html_tag.html) class.
pub trait THtmlTag : TObject {
}

/// Wraps the wxWidgets' [wxHtmlTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_tag_handler.html) class.
pub struct HtmlTagHandler { ptr: *mut c_void }
impl THtmlTagHandler for HtmlTagHandler {}
impl TObject for HtmlTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlTagHandler {
    pub fn from(ptr: *mut c_void) -> HtmlTagHandler { HtmlTagHandler { ptr: ptr } }
    pub fn null() -> HtmlTagHandler { HtmlTagHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_tag_handler.html) class.
pub trait THtmlTagHandler : TObject {
}

/// Wraps the wxWidgets' [wxHtmlTagsModule](http://docs.wxwidgets.org/3.0/classwx_html_tags_module.html) class.
pub struct HtmlTagsModule { ptr: *mut c_void }
impl THtmlTagsModule for HtmlTagsModule {}
impl TModule for HtmlTagsModule {}
impl TObject for HtmlTagsModule { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlTagsModule {
    pub fn from(ptr: *mut c_void) -> HtmlTagsModule { HtmlTagsModule { ptr: ptr } }
    pub fn null() -> HtmlTagsModule { HtmlTagsModule::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlTagsModule](http://docs.wxwidgets.org/3.0/classwx_html_tags_module.html) class.
pub trait THtmlTagsModule : TModule {
}

/// Wraps the wxWidgets' [wxHtmlWidgetCell](http://docs.wxwidgets.org/3.0/classwx_html_widget_cell.html) class.
pub struct HtmlWidgetCell { ptr: *mut c_void }
impl THtmlWidgetCell for HtmlWidgetCell {}
impl THtmlCell for HtmlWidgetCell {}
impl TObject for HtmlWidgetCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWidgetCell {
    pub fn from(ptr: *mut c_void) -> HtmlWidgetCell { HtmlWidgetCell { ptr: ptr } }
    pub fn null() -> HtmlWidgetCell { HtmlWidgetCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlWidgetCell](http://docs.wxwidgets.org/3.0/classwx_html_widget_cell.html) class.
pub trait THtmlWidgetCell : THtmlCell {
}

/// Wraps the wxWidgets' [wxHtmlWinParser](http://docs.wxwidgets.org/3.0/classwx_html_win_parser.html) class.
pub struct HtmlWinParser { ptr: *mut c_void }
impl THtmlWinParser for HtmlWinParser {}
impl THtmlParser for HtmlWinParser {}
impl TObject for HtmlWinParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWinParser {
    pub fn from(ptr: *mut c_void) -> HtmlWinParser { HtmlWinParser { ptr: ptr } }
    pub fn null() -> HtmlWinParser { HtmlWinParser::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlWinParser](http://docs.wxwidgets.org/3.0/classwx_html_win_parser.html) class.
pub trait THtmlWinParser : THtmlParser {
}

/// Wraps the wxWidgets' [wxHtmlWinTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_win_tag_handler.html) class.
pub struct HtmlWinTagHandler { ptr: *mut c_void }
impl THtmlWinTagHandler for HtmlWinTagHandler {}
impl THtmlTagHandler for HtmlWinTagHandler {}
impl TObject for HtmlWinTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWinTagHandler {
    pub fn from(ptr: *mut c_void) -> HtmlWinTagHandler { HtmlWinTagHandler { ptr: ptr } }
    pub fn null() -> HtmlWinTagHandler { HtmlWinTagHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlWinTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_win_tag_handler.html) class.
pub trait THtmlWinTagHandler : THtmlTagHandler {
}

/// Wraps the wxWidgets' [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html) class.
/// Rather use the wxRust-specific [CHtmlWindow](struct.CHtmlWindow.html) class.
pub struct HtmlWindow { ptr: *mut c_void }
impl THtmlWindow for HtmlWindow {}
impl TScrolledWindow for HtmlWindow {}
impl TPanel for HtmlWindow {}
impl TWindow for HtmlWindow {}
impl TEvtHandler for HtmlWindow {}
impl TObject for HtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWindow {
    pub fn from(ptr: *mut c_void) -> HtmlWindow { HtmlWindow { ptr: ptr } }
    pub fn null() -> HtmlWindow { HtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> HtmlWindow {
        let _txt = wxT(_txt);
        unsafe { HtmlWindow { ptr: wxHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr()) } }
    }
}

/// Methods of the wxWidgets' [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html) class.
pub trait THtmlWindow : TScrolledWindow {
    fn appendToPage(&self, source: &str) -> c_int {
        let source = wxT(source);
        unsafe { wxHtmlWindow_AppendToPage(self.ptr(), source.ptr()) }
    }
    fn getInternalRepresentation(&self) -> HtmlContainerCell {
        unsafe { HtmlContainerCell { ptr: wxHtmlWindow_GetInternalRepresentation(self.ptr()) } }
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
    fn getRelatedFrame(&self) -> Frame {
        unsafe { Frame { ptr: wxHtmlWindow_GetRelatedFrame(self.ptr()) } }
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
    fn readCustomization<T: TConfigBase>(&self, cfg: &T, path: &str) {
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
    fn setRelatedFrame<T: TFrame>(&self, frame: &T, format: &str) {
        let format = wxT(format);
        unsafe { wxHtmlWindow_SetRelatedFrame(self.ptr(), frame.ptr(), format.ptr()) }
    }
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.ptr(), bar) }
    }
    fn writeCustomization<T: TConfigBase>(&self, cfg: &T, path: &str) {
        let path = wxT(path);
        unsafe { wxHtmlWindow_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

/// The wxRust-specific derived class of [wxCommandEvent](http://docs.wxwidgets.org/3.0/classwx_command_event.html).
pub struct CHtmlEvent { ptr: *mut c_void }
impl TCHtmlEvent for CHtmlEvent {}
impl TCommandEvent for CHtmlEvent {}
impl TEvent for CHtmlEvent {}
impl TObject for CHtmlEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CHtmlEvent {
    pub fn from(ptr: *mut c_void) -> CHtmlEvent { CHtmlEvent { ptr: ptr } }
    pub fn null() -> CHtmlEvent { CHtmlEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxCommandEvent](http://docs.wxwidgets.org/3.0/classwx_command_event.html).
pub trait TCHtmlEvent : TCommandEvent {
    fn getMouseEvent(&self) -> MouseEvent {
        unsafe { MouseEvent { ptr: wxcHtmlEvent_GetMouseEvent(self.ptr()) } }
    }
    fn getHtmlCell(&self) -> HtmlCell {
        unsafe { HtmlCell { ptr: wxcHtmlEvent_GetHtmlCell(self.ptr()) } }
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
    fn getLogicalPosition(&self) -> Point {
        unsafe { Point { ptr: wxcHtmlEvent_GetLogicalPosition(self.ptr()) } }
    }
}

/// The wxRust-specific derived class of [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html).
pub struct CHtmlWindow { ptr: *mut c_void }
impl TCHtmlWindow for CHtmlWindow {}
impl THtmlWindow for CHtmlWindow {}
impl TScrolledWindow for CHtmlWindow {}
impl TPanel for CHtmlWindow {}
impl TWindow for CHtmlWindow {}
impl TEvtHandler for CHtmlWindow {}
impl TObject for CHtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CHtmlWindow {
    pub fn from(ptr: *mut c_void) -> CHtmlWindow { CHtmlWindow { ptr: ptr } }
    pub fn null() -> CHtmlWindow { CHtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> CHtmlWindow {
        let _txt = wxT(_txt);
        unsafe { CHtmlWindow { ptr: wxcHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr()) } }
    }
}

/// Methods of the wxRust-specific derived class of [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html).
pub trait TCHtmlWindow : THtmlWindow {
}

