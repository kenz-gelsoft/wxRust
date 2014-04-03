use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

/// Wraps the wxWidgets' [wxHtmlCell](http://docs.wxwidgets.org/3.0/classwx_html_cell.html) class.
pub struct HtmlCell { ptr: *mut c_void }
impl HtmlCellMethods for HtmlCell {}
impl ObjectMethods for HtmlCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlCell {
    pub fn from(ptr: *mut c_void) -> HtmlCell { HtmlCell { ptr: ptr } }
    pub fn null() -> HtmlCell { HtmlCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlCell](http://docs.wxwidgets.org/3.0/classwx_html_cell.html) class.
pub trait HtmlCellMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlColourCell](http://docs.wxwidgets.org/3.0/classwx_html_colour_cell.html) class.
pub struct HtmlColourCell { ptr: *mut c_void }
impl HtmlColourCellMethods for HtmlColourCell {}
impl HtmlCellMethods for HtmlColourCell {}
impl ObjectMethods for HtmlColourCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlColourCell {
    pub fn from(ptr: *mut c_void) -> HtmlColourCell { HtmlColourCell { ptr: ptr } }
    pub fn null() -> HtmlColourCell { HtmlColourCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlColourCell](http://docs.wxwidgets.org/3.0/classwx_html_colour_cell.html) class.
pub trait HtmlColourCellMethods : HtmlCellMethods {
}

/// Wraps the wxWidgets' [wxHtmlContainerCell](http://docs.wxwidgets.org/3.0/classwx_html_container_cell.html) class.
pub struct HtmlContainerCell { ptr: *mut c_void }
impl HtmlContainerCellMethods for HtmlContainerCell {}
impl HtmlCellMethods for HtmlContainerCell {}
impl ObjectMethods for HtmlContainerCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlContainerCell {
    pub fn from(ptr: *mut c_void) -> HtmlContainerCell { HtmlContainerCell { ptr: ptr } }
    pub fn null() -> HtmlContainerCell { HtmlContainerCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlContainerCell](http://docs.wxwidgets.org/3.0/classwx_html_container_cell.html) class.
pub trait HtmlContainerCellMethods : HtmlCellMethods {
}

/// Wraps the wxWidgets' [wxHtmlDCRenderer](http://docs.wxwidgets.org/3.0/classwx_html_dcr_enderer.html) class.
pub struct HtmlDCRenderer { ptr: *mut c_void }
impl HtmlDCRendererMethods for HtmlDCRenderer {}
impl ObjectMethods for HtmlDCRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlDCRenderer {
    pub fn from(ptr: *mut c_void) -> HtmlDCRenderer { HtmlDCRenderer { ptr: ptr } }
    pub fn null() -> HtmlDCRenderer { HtmlDCRenderer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlDCRenderer](http://docs.wxwidgets.org/3.0/classwx_html_dcr_enderer.html) class.
pub trait HtmlDCRendererMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlEasyPrinting](http://docs.wxwidgets.org/3.0/classwx_html_easy_printing.html) class.
pub struct HtmlEasyPrinting { ptr: *mut c_void }
impl HtmlEasyPrintingMethods for HtmlEasyPrinting {}
impl ObjectMethods for HtmlEasyPrinting { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlEasyPrinting {
    pub fn from(ptr: *mut c_void) -> HtmlEasyPrinting { HtmlEasyPrinting { ptr: ptr } }
    pub fn null() -> HtmlEasyPrinting { HtmlEasyPrinting::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlEasyPrinting](http://docs.wxwidgets.org/3.0/classwx_html_easy_printing.html) class.
pub trait HtmlEasyPrintingMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlFilter](http://docs.wxwidgets.org/3.0/classwx_html_filter.html) class.
pub struct HtmlFilter { ptr: *mut c_void }
impl HtmlFilterMethods for HtmlFilter {}
impl ObjectMethods for HtmlFilter { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlFilter {
    pub fn from(ptr: *mut c_void) -> HtmlFilter { HtmlFilter { ptr: ptr } }
    pub fn null() -> HtmlFilter { HtmlFilter::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlFilter](http://docs.wxwidgets.org/3.0/classwx_html_filter.html) class.
pub trait HtmlFilterMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlHelpController](http://docs.wxwidgets.org/3.0/classwx_html_help_controller.html) class.
pub struct HtmlHelpController { ptr: *mut c_void }
impl HtmlHelpControllerMethods for HtmlHelpController {}
impl HelpControllerBaseMethods for HtmlHelpController {}
impl ObjectMethods for HtmlHelpController { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlHelpController {
    pub fn from(ptr: *mut c_void) -> HtmlHelpController { HtmlHelpController { ptr: ptr } }
    pub fn null() -> HtmlHelpController { HtmlHelpController::from(0 as *mut c_void) }
    
    pub fn new(_style: c_int) -> HtmlHelpController {
        unsafe { HtmlHelpController::from(wxHtmlHelpController_Create(_style)) }
    }
}

/// Methods of the wxWidgets' [wxHtmlHelpController](http://docs.wxwidgets.org/3.0/classwx_html_help_controller.html) class.
pub trait HtmlHelpControllerMethods : HelpControllerBaseMethods {
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
        let section = strToString(section);
        unsafe { wxHtmlHelpController_DisplaySection(self.ptr(), section.ptr()) }
    }
    fn displaySectionNumber(&self, sectionNo: c_int) -> c_int {
        unsafe { wxHtmlHelpController_DisplaySectionNumber(self.ptr(), sectionNo) }
    }
    fn getFrame(&self) -> Frame {
        unsafe { Frame::from(wxHtmlHelpController_GetFrame(self.ptr())) }
    }
    fn getFrameParameters(&self, title: *mut c_void, width: *mut c_int, height: *mut c_int, pos_x: *mut c_int, pos_y: *mut c_int, newFrameEachTime: *mut c_int) -> *mut c_void {
        unsafe { wxHtmlHelpController_GetFrameParameters(self.ptr(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    fn initialize(&self, file: &str) -> c_int {
        let file = strToString(file);
        unsafe { wxHtmlHelpController_Initialize(self.ptr(), file.ptr()) }
    }
    fn keywordSearch(&self, keyword: &str) -> c_int {
        let keyword = strToString(keyword);
        unsafe { wxHtmlHelpController_KeywordSearch(self.ptr(), keyword.ptr()) }
    }
    fn loadFile(&self, file: &str) -> c_int {
        let file = strToString(file);
        unsafe { wxHtmlHelpController_LoadFile(self.ptr(), file.ptr()) }
    }
    fn quit(&self) -> c_int {
        unsafe { wxHtmlHelpController_Quit(self.ptr()) }
    }
    fn readCustomization<T: ConfigBaseMethods>(&self, cfg: &T, path: &str) {
        let path = strToString(path);
        unsafe { wxHtmlHelpController_ReadCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
    fn setFrameParameters(&self, title: *mut c_void, width: c_int, height: c_int, pos_x: c_int, pos_y: c_int, newFrameEachTime: c_int) {
        unsafe { wxHtmlHelpController_SetFrameParameters(self.ptr(), title, width, height, pos_x, pos_y, newFrameEachTime) }
    }
    fn setTempDir(&self, path: &str) {
        let path = strToString(path);
        unsafe { wxHtmlHelpController_SetTempDir(self.ptr(), path.ptr()) }
    }
    fn setTitleFormat(&self, format: *mut c_void) {
        unsafe { wxHtmlHelpController_SetTitleFormat(self.ptr(), format) }
    }
    fn setViewer(&self, viewer: &str, flags: c_int) {
        let viewer = strToString(viewer);
        unsafe { wxHtmlHelpController_SetViewer(self.ptr(), viewer.ptr(), flags) }
    }
    fn useConfig<T: ConfigBaseMethods>(&self, config: &T, rootpath: &str) {
        let rootpath = strToString(rootpath);
        unsafe { wxHtmlHelpController_UseConfig(self.ptr(), config.ptr(), rootpath.ptr()) }
    }
    fn writeCustomization<T: ConfigBaseMethods>(&self, cfg: &T, path: &str) {
        let path = strToString(path);
        unsafe { wxHtmlHelpController_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

/// Wraps the wxWidgets' [wxHtmlHelpData](http://docs.wxwidgets.org/3.0/classwx_html_help_data.html) class.
pub struct HtmlHelpData { ptr: *mut c_void }
impl HtmlHelpDataMethods for HtmlHelpData {}
impl ObjectMethods for HtmlHelpData { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlHelpData {
    pub fn from(ptr: *mut c_void) -> HtmlHelpData { HtmlHelpData { ptr: ptr } }
    pub fn null() -> HtmlHelpData { HtmlHelpData::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlHelpData](http://docs.wxwidgets.org/3.0/classwx_html_help_data.html) class.
pub trait HtmlHelpDataMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlHelpFrame](http://docs.wxwidgets.org/3.0/classwx_html_help_frame.html) class.
pub struct HtmlHelpFrame { ptr: *mut c_void }
impl HtmlHelpFrameMethods for HtmlHelpFrame {}
impl FrameMethods for HtmlHelpFrame {}
impl TopLevelWindowMethods for HtmlHelpFrame {}
impl WindowMethods for HtmlHelpFrame {}
impl EvtHandlerMethods for HtmlHelpFrame {}
impl ObjectMethods for HtmlHelpFrame { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlHelpFrame {
    pub fn from(ptr: *mut c_void) -> HtmlHelpFrame { HtmlHelpFrame { ptr: ptr } }
    pub fn null() -> HtmlHelpFrame { HtmlHelpFrame::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlHelpFrame](http://docs.wxwidgets.org/3.0/classwx_html_help_frame.html) class.
pub trait HtmlHelpFrameMethods : FrameMethods {
}

/// Wraps the wxWidgets' [wxHtmlLinkInfo](http://docs.wxwidgets.org/3.0/classwx_html_link_info.html) class.
pub struct HtmlLinkInfo { ptr: *mut c_void }
impl HtmlLinkInfoMethods for HtmlLinkInfo {}
impl ObjectMethods for HtmlLinkInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlLinkInfo {
    pub fn from(ptr: *mut c_void) -> HtmlLinkInfo { HtmlLinkInfo { ptr: ptr } }
    pub fn null() -> HtmlLinkInfo { HtmlLinkInfo::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlLinkInfo](http://docs.wxwidgets.org/3.0/classwx_html_link_info.html) class.
pub trait HtmlLinkInfoMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlParser](http://docs.wxwidgets.org/3.0/classwx_html_parser.html) class.
pub struct HtmlParser { ptr: *mut c_void }
impl HtmlParserMethods for HtmlParser {}
impl ObjectMethods for HtmlParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlParser {
    pub fn from(ptr: *mut c_void) -> HtmlParser { HtmlParser { ptr: ptr } }
    pub fn null() -> HtmlParser { HtmlParser::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlParser](http://docs.wxwidgets.org/3.0/classwx_html_parser.html) class.
pub trait HtmlParserMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlPrintout](http://docs.wxwidgets.org/3.0/classwx_html_printout.html) class.
pub struct HtmlPrintout { ptr: *mut c_void }
impl HtmlPrintoutMethods for HtmlPrintout {}
impl PrintoutMethods for HtmlPrintout {}
impl ObjectMethods for HtmlPrintout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlPrintout {
    pub fn from(ptr: *mut c_void) -> HtmlPrintout { HtmlPrintout { ptr: ptr } }
    pub fn null() -> HtmlPrintout { HtmlPrintout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlPrintout](http://docs.wxwidgets.org/3.0/classwx_html_printout.html) class.
pub trait HtmlPrintoutMethods : PrintoutMethods {
}

/// Wraps the wxWidgets' [wxHtmlTag](http://docs.wxwidgets.org/3.0/classwx_html_tag.html) class.
pub struct HtmlTag { ptr: *mut c_void }
impl HtmlTagMethods for HtmlTag {}
impl ObjectMethods for HtmlTag { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlTag {
    pub fn from(ptr: *mut c_void) -> HtmlTag { HtmlTag { ptr: ptr } }
    pub fn null() -> HtmlTag { HtmlTag::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlTag](http://docs.wxwidgets.org/3.0/classwx_html_tag.html) class.
pub trait HtmlTagMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_tag_handler.html) class.
pub struct HtmlTagHandler { ptr: *mut c_void }
impl HtmlTagHandlerMethods for HtmlTagHandler {}
impl ObjectMethods for HtmlTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlTagHandler {
    pub fn from(ptr: *mut c_void) -> HtmlTagHandler { HtmlTagHandler { ptr: ptr } }
    pub fn null() -> HtmlTagHandler { HtmlTagHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_tag_handler.html) class.
pub trait HtmlTagHandlerMethods : ObjectMethods {
}

/// Wraps the wxWidgets' [wxHtmlTagsModule](http://docs.wxwidgets.org/3.0/classwx_html_tags_module.html) class.
pub struct HtmlTagsModule { ptr: *mut c_void }
impl HtmlTagsModuleMethods for HtmlTagsModule {}
impl ModuleMethods for HtmlTagsModule {}
impl ObjectMethods for HtmlTagsModule { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlTagsModule {
    pub fn from(ptr: *mut c_void) -> HtmlTagsModule { HtmlTagsModule { ptr: ptr } }
    pub fn null() -> HtmlTagsModule { HtmlTagsModule::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlTagsModule](http://docs.wxwidgets.org/3.0/classwx_html_tags_module.html) class.
pub trait HtmlTagsModuleMethods : ModuleMethods {
}

/// Wraps the wxWidgets' [wxHtmlWidgetCell](http://docs.wxwidgets.org/3.0/classwx_html_widget_cell.html) class.
pub struct HtmlWidgetCell { ptr: *mut c_void }
impl HtmlWidgetCellMethods for HtmlWidgetCell {}
impl HtmlCellMethods for HtmlWidgetCell {}
impl ObjectMethods for HtmlWidgetCell { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWidgetCell {
    pub fn from(ptr: *mut c_void) -> HtmlWidgetCell { HtmlWidgetCell { ptr: ptr } }
    pub fn null() -> HtmlWidgetCell { HtmlWidgetCell::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlWidgetCell](http://docs.wxwidgets.org/3.0/classwx_html_widget_cell.html) class.
pub trait HtmlWidgetCellMethods : HtmlCellMethods {
}

/// Wraps the wxWidgets' [wxHtmlWinParser](http://docs.wxwidgets.org/3.0/classwx_html_win_parser.html) class.
pub struct HtmlWinParser { ptr: *mut c_void }
impl HtmlWinParserMethods for HtmlWinParser {}
impl HtmlParserMethods for HtmlWinParser {}
impl ObjectMethods for HtmlWinParser { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWinParser {
    pub fn from(ptr: *mut c_void) -> HtmlWinParser { HtmlWinParser { ptr: ptr } }
    pub fn null() -> HtmlWinParser { HtmlWinParser::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlWinParser](http://docs.wxwidgets.org/3.0/classwx_html_win_parser.html) class.
pub trait HtmlWinParserMethods : HtmlParserMethods {
}

/// Wraps the wxWidgets' [wxHtmlWinTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_win_tag_handler.html) class.
pub struct HtmlWinTagHandler { ptr: *mut c_void }
impl HtmlWinTagHandlerMethods for HtmlWinTagHandler {}
impl HtmlTagHandlerMethods for HtmlWinTagHandler {}
impl ObjectMethods for HtmlWinTagHandler { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWinTagHandler {
    pub fn from(ptr: *mut c_void) -> HtmlWinTagHandler { HtmlWinTagHandler { ptr: ptr } }
    pub fn null() -> HtmlWinTagHandler { HtmlWinTagHandler::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHtmlWinTagHandler](http://docs.wxwidgets.org/3.0/classwx_html_win_tag_handler.html) class.
pub trait HtmlWinTagHandlerMethods : HtmlTagHandlerMethods {
}

/// Wraps the wxWidgets' [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html) class.
/// Rather use the wxRust-specific [CHtmlWindow](struct.CHtmlWindow.html) class.
pub struct HtmlWindow { ptr: *mut c_void }
impl HtmlWindowMethods for HtmlWindow {}
impl ScrolledWindowMethods for HtmlWindow {}
impl PanelMethods for HtmlWindow {}
impl WindowMethods for HtmlWindow {}
impl EvtHandlerMethods for HtmlWindow {}
impl ObjectMethods for HtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HtmlWindow {
    pub fn from(ptr: *mut c_void) -> HtmlWindow { HtmlWindow { ptr: ptr } }
    pub fn null() -> HtmlWindow { HtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> HtmlWindow {
        let _txt = strToString(_txt);
        unsafe { HtmlWindow::from(wxHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr())) }
    }
}

/// Methods of the wxWidgets' [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html) class.
pub trait HtmlWindowMethods : ScrolledWindowMethods {
    fn appendToPage(&self, source: &str) -> c_int {
        let source = strToString(source);
        unsafe { wxHtmlWindow_AppendToPage(self.ptr(), source.ptr()) }
    }
    fn getInternalRepresentation(&self) -> HtmlContainerCell {
        unsafe { HtmlContainerCell::from(wxHtmlWindow_GetInternalRepresentation(self.ptr())) }
    }
    fn getOpenedAnchor(&self) -> ~str {
        unsafe { String::from(wxHtmlWindow_GetOpenedAnchor(self.ptr())).to_str() }
    }
    fn getOpenedPage(&self) -> ~str {
        unsafe { String::from(wxHtmlWindow_GetOpenedPage(self.ptr())).to_str() }
    }
    fn getOpenedPageTitle(&self) -> ~str {
        unsafe { String::from(wxHtmlWindow_GetOpenedPageTitle(self.ptr())).to_str() }
    }
    fn getRelatedFrame(&self) -> Frame {
        unsafe { Frame::from(wxHtmlWindow_GetRelatedFrame(self.ptr())) }
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
        let location = strToString(location);
        unsafe { wxHtmlWindow_LoadPage(self.ptr(), location.ptr()) }
    }
    fn readCustomization<T: ConfigBaseMethods>(&self, cfg: &T, path: &str) {
        let path = strToString(path);
        unsafe { wxHtmlWindow_ReadCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
    fn setBorders(&self, b: c_int) {
        unsafe { wxHtmlWindow_SetBorders(self.ptr(), b) }
    }
    fn setFonts(&self, normal_face: &str, fixed_face: &str, sizes: *mut c_int) {
        let normal_face = strToString(normal_face);
        let fixed_face = strToString(fixed_face);
        unsafe { wxHtmlWindow_SetFonts(self.ptr(), normal_face.ptr(), fixed_face.ptr(), sizes) }
    }
    fn setPage(&self, source: &str) {
        let source = strToString(source);
        unsafe { wxHtmlWindow_SetPage(self.ptr(), source.ptr()) }
    }
    fn setRelatedFrame<T: FrameMethods>(&self, frame: &T, format: &str) {
        let format = strToString(format);
        unsafe { wxHtmlWindow_SetRelatedFrame(self.ptr(), frame.ptr(), format.ptr()) }
    }
    fn setRelatedStatusBar(&self, bar: c_int) {
        unsafe { wxHtmlWindow_SetRelatedStatusBar(self.ptr(), bar) }
    }
    fn writeCustomization<T: ConfigBaseMethods>(&self, cfg: &T, path: &str) {
        let path = strToString(path);
        unsafe { wxHtmlWindow_WriteCustomization(self.ptr(), cfg.ptr(), path.ptr()) }
    }
}

/// The wxRust-specific derived class of [wxCommandEvent](http://docs.wxwidgets.org/3.0/classwx_command_event.html).
pub struct RustHtmlEvent { ptr: *mut c_void }
impl RustHtmlEventMethods for RustHtmlEvent {}
impl CommandEventMethods for RustHtmlEvent {}
impl EventMethods for RustHtmlEvent {}
impl ObjectMethods for RustHtmlEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustHtmlEvent {
    pub fn from(ptr: *mut c_void) -> RustHtmlEvent { RustHtmlEvent { ptr: ptr } }
    pub fn null() -> RustHtmlEvent { RustHtmlEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxCommandEvent](http://docs.wxwidgets.org/3.0/classwx_command_event.html).
pub trait RustHtmlEventMethods : CommandEventMethods {
    fn getMouseEvent(&self) -> MouseEvent {
        unsafe { MouseEvent::from(wxcHtmlEvent_GetMouseEvent(self.ptr())) }
    }
    fn getHtmlCell(&self) -> HtmlCell {
        unsafe { HtmlCell::from(wxcHtmlEvent_GetHtmlCell(self.ptr())) }
    }
    fn getHtmlCellId(&self) -> ~str {
        unsafe { String::from(wxcHtmlEvent_GetHtmlCellId(self.ptr())).to_str() }
    }
    fn getHref(&self) -> ~str {
        unsafe { String::from(wxcHtmlEvent_GetHref(self.ptr())).to_str() }
    }
    fn getTarget(&self) -> ~str {
        unsafe { String::from(wxcHtmlEvent_GetTarget(self.ptr())).to_str() }
    }
    fn getLogicalPosition(&self) -> Point {
        unsafe { Point::from(wxcHtmlEvent_GetLogicalPosition(self.ptr())) }
    }
}

/// The wxRust-specific derived class of [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html).
pub struct RustHtmlWindow { ptr: *mut c_void }
impl RustHtmlWindowMethods for RustHtmlWindow {}
impl HtmlWindowMethods for RustHtmlWindow {}
impl ScrolledWindowMethods for RustHtmlWindow {}
impl PanelMethods for RustHtmlWindow {}
impl WindowMethods for RustHtmlWindow {}
impl EvtHandlerMethods for RustHtmlWindow {}
impl ObjectMethods for RustHtmlWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustHtmlWindow {
    pub fn from(ptr: *mut c_void) -> RustHtmlWindow { RustHtmlWindow { ptr: ptr } }
    pub fn null() -> RustHtmlWindow { RustHtmlWindow::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int, _txt: &str) -> RustHtmlWindow {
        let _txt = strToString(_txt);
        unsafe { RustHtmlWindow::from(wxcHtmlWindow_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl, _txt.ptr())) }
    }
}

/// Methods of the wxRust-specific derived class of [wxHtmlWindow](http://docs.wxwidgets.org/3.0/classwx_html_window.html).
pub trait RustHtmlWindowMethods : HtmlWindowMethods {
}

