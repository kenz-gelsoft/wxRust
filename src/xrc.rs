use libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;
use html::*;
use stc::*;

/// Wraps the wxWidgets' [wxXmlResource](http://docs.wxwidgets.org/3.0/classwx_xml_resource.html) class.
pub struct XmlResource { ptr: *mut c_void }
impl XmlResourceMethods for XmlResource {}
impl ObjectMethods for XmlResource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl XmlResource {
    pub fn from(ptr: *mut c_void) -> XmlResource { XmlResource { ptr: ptr } }
    pub fn null() -> XmlResource { XmlResource::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> XmlResource {
        unsafe { XmlResource::from(wxXmlResource_Create(flags)) }
    }
    pub fn newFromFile(filemask: &str, flags: c_int) -> XmlResource {
        let filemask = strToString(filemask);
        unsafe { XmlResource::from(wxXmlResource_CreateFromFile(filemask.ptr(), flags)) }
    }
    pub fn get() -> XmlResource {
        unsafe { XmlResource::from(wxXmlResource_Get()) }
    }
}

/// Methods of the wxWidgets' [wxXmlResource](http://docs.wxwidgets.org/3.0/classwx_xml_resource.html) class.
pub trait XmlResourceMethods : ObjectMethods {
    fn addHandler<T: EvtHandlerMethods>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.ptr(), handler.ptr()) }
    }
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.ptr(), factory) }
    }
    fn attachUnknownControl<T: ControlMethods, U: WindowMethods>(&self, control: &T, parent: &U) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.ptr(), control.ptr(), parent.ptr()) }
    }
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.ptr()) }
    }
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.ptr(), major, minor, release, revision) }
    }
    fn getDomain(&self) -> ~str {
        unsafe { String::from(wxXmlResource_GetDomain(self.ptr())).to_str() }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxXmlResource_GetFlags(self.ptr()) }
    }
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self.ptr()) }
    }
    fn getXRCID(&self, str_id: &str) -> c_int {
        let str_id = strToString(str_id);
        unsafe { wxXmlResource_GetXRCID(self.ptr(), str_id.ptr()) }
    }
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.ptr()) }
    }
    fn insertHandler<T: EvtHandlerMethods>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.ptr(), handler.ptr()) }
    }
    fn load(&self, filemask: &str) -> c_int {
        let filemask = strToString(filemask);
        unsafe { wxXmlResource_Load(self.ptr(), filemask.ptr()) }
    }
    fn loadBitmap<T: BitmapMethods>(&self, name: &str, _ref: &T) {
        let name = strToString(name);
        unsafe { wxXmlResource_LoadBitmap(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadDialog<T: WindowMethods>(&self, parent: &T, name: &str) -> Dialog {
        let name = strToString(name);
        unsafe { Dialog::from(wxXmlResource_LoadDialog(self.ptr(), parent.ptr(), name.ptr())) }
    }
    fn loadFrame<T: WindowMethods>(&self, parent: &T, name: &str) -> Frame {
        let name = strToString(name);
        unsafe { Frame::from(wxXmlResource_LoadFrame(self.ptr(), parent.ptr(), name.ptr())) }
    }
    fn loadIcon<T: IconMethods>(&self, name: &str, _ref: &T) {
        let name = strToString(name);
        unsafe { wxXmlResource_LoadIcon(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadMenu(&self, name: &str) -> Menu {
        let name = strToString(name);
        unsafe { Menu::from(wxXmlResource_LoadMenu(self.ptr(), name.ptr())) }
    }
    fn loadMenuBar<T: WindowMethods>(&self, parent: &T, name: &str) -> MenuBar {
        let name = strToString(name);
        unsafe { MenuBar::from(wxXmlResource_LoadMenuBar(self.ptr(), parent.ptr(), name.ptr())) }
    }
    fn loadPanel<T: WindowMethods>(&self, parent: &T, name: &str) -> Panel {
        let name = strToString(name);
        unsafe { Panel::from(wxXmlResource_LoadPanel(self.ptr(), parent.ptr(), name.ptr())) }
    }
    fn loadToolBar<T: WindowMethods>(&self, parent: &T, name: &str) -> ToolBar {
        let name = strToString(name);
        unsafe { ToolBar::from(wxXmlResource_LoadToolBar(self.ptr(), parent.ptr(), name.ptr())) }
    }
    fn getSizer(&self, str_id: &str) -> Sizer {
        let str_id = strToString(str_id);
        unsafe { Sizer::from(wxXmlResource_GetSizer(self.ptr(), str_id.ptr())) }
    }
    fn getBoxSizer(&self, str_id: &str) -> BoxSizer {
        let str_id = strToString(str_id);
        unsafe { BoxSizer::from(wxXmlResource_GetBoxSizer(self.ptr(), str_id.ptr())) }
    }
    fn getStaticBoxSizer(&self, str_id: &str) -> StaticBoxSizer {
        let str_id = strToString(str_id);
        unsafe { StaticBoxSizer::from(wxXmlResource_GetStaticBoxSizer(self.ptr(), str_id.ptr())) }
    }
    fn getGridSizer(&self, str_id: &str) -> GridSizer {
        let str_id = strToString(str_id);
        unsafe { GridSizer::from(wxXmlResource_GetGridSizer(self.ptr(), str_id.ptr())) }
    }
    fn getFlexGridSizer(&self, str_id: &str) -> FlexGridSizer {
        let str_id = strToString(str_id);
        unsafe { FlexGridSizer::from(wxXmlResource_GetFlexGridSizer(self.ptr(), str_id.ptr())) }
    }
    fn getBitmapButton(&self, str_id: &str) -> BitmapButton {
        let str_id = strToString(str_id);
        unsafe { BitmapButton::from(wxXmlResource_GetBitmapButton(self.ptr(), str_id.ptr())) }
    }
    fn getButton(&self, str_id: &str) -> Button {
        let str_id = strToString(str_id);
        unsafe { Button::from(wxXmlResource_GetButton(self.ptr(), str_id.ptr())) }
    }
    fn getCalendarCtrl(&self, str_id: &str) -> CalendarCtrl {
        let str_id = strToString(str_id);
        unsafe { CalendarCtrl::from(wxXmlResource_GetCalendarCtrl(self.ptr(), str_id.ptr())) }
    }
    fn getCheckBox(&self, str_id: &str) -> CheckBox {
        let str_id = strToString(str_id);
        unsafe { CheckBox::from(wxXmlResource_GetCheckBox(self.ptr(), str_id.ptr())) }
    }
    fn getCheckListBox(&self, str_id: &str) -> CheckListBox {
        let str_id = strToString(str_id);
        unsafe { CheckListBox::from(wxXmlResource_GetCheckListBox(self.ptr(), str_id.ptr())) }
    }
    fn getChoice(&self, str_id: &str) -> Choice {
        let str_id = strToString(str_id);
        unsafe { Choice::from(wxXmlResource_GetChoice(self.ptr(), str_id.ptr())) }
    }
    fn getComboBox(&self, str_id: &str) -> ComboBox {
        let str_id = strToString(str_id);
        unsafe { ComboBox::from(wxXmlResource_GetComboBox(self.ptr(), str_id.ptr())) }
    }
    fn getGauge(&self, str_id: &str) -> Gauge {
        let str_id = strToString(str_id);
        unsafe { Gauge::from(wxXmlResource_GetGauge(self.ptr(), str_id.ptr())) }
    }
    fn getGrid(&self, str_id: &str) -> Grid {
        let str_id = strToString(str_id);
        unsafe { Grid::from(wxXmlResource_GetGrid(self.ptr(), str_id.ptr())) }
    }
    fn getHtmlWindow(&self, str_id: &str) -> HtmlWindow {
        let str_id = strToString(str_id);
        unsafe { HtmlWindow::from(wxXmlResource_GetHtmlWindow(self.ptr(), str_id.ptr())) }
    }
    fn getListBox(&self, str_id: &str) -> ListBox {
        let str_id = strToString(str_id);
        unsafe { ListBox::from(wxXmlResource_GetListBox(self.ptr(), str_id.ptr())) }
    }
    fn getListCtrl(&self, str_id: &str) -> ListCtrl {
        let str_id = strToString(str_id);
        unsafe { ListCtrl::from(wxXmlResource_GetListCtrl(self.ptr(), str_id.ptr())) }
    }
    fn getMDIChildFrame(&self, str_id: &str) -> MDIChildFrame {
        let str_id = strToString(str_id);
        unsafe { MDIChildFrame::from(wxXmlResource_GetMDIChildFrame(self.ptr(), str_id.ptr())) }
    }
    fn getMDIParentFrame(&self, str_id: &str) -> MDIParentFrame {
        let str_id = strToString(str_id);
        unsafe { MDIParentFrame::from(wxXmlResource_GetMDIParentFrame(self.ptr(), str_id.ptr())) }
    }
    fn getMenu(&self, str_id: &str) -> Menu {
        let str_id = strToString(str_id);
        unsafe { Menu::from(wxXmlResource_GetMenu(self.ptr(), str_id.ptr())) }
    }
    fn getMenuBar(&self, str_id: &str) -> MenuBar {
        let str_id = strToString(str_id);
        unsafe { MenuBar::from(wxXmlResource_GetMenuBar(self.ptr(), str_id.ptr())) }
    }
    fn getMenuItem(&self, str_id: &str) -> MenuItem {
        let str_id = strToString(str_id);
        unsafe { MenuItem::from(wxXmlResource_GetMenuItem(self.ptr(), str_id.ptr())) }
    }
    fn getNotebook(&self, str_id: &str) -> Notebook {
        let str_id = strToString(str_id);
        unsafe { Notebook::from(wxXmlResource_GetNotebook(self.ptr(), str_id.ptr())) }
    }
    fn getPanel(&self, str_id: &str) -> Panel {
        let str_id = strToString(str_id);
        unsafe { Panel::from(wxXmlResource_GetPanel(self.ptr(), str_id.ptr())) }
    }
    fn getRadioButton(&self, str_id: &str) -> RadioButton {
        let str_id = strToString(str_id);
        unsafe { RadioButton::from(wxXmlResource_GetRadioButton(self.ptr(), str_id.ptr())) }
    }
    fn getRadioBox(&self, str_id: &str) -> RadioBox {
        let str_id = strToString(str_id);
        unsafe { RadioBox::from(wxXmlResource_GetRadioBox(self.ptr(), str_id.ptr())) }
    }
    fn getScrollBar(&self, str_id: &str) -> ScrollBar {
        let str_id = strToString(str_id);
        unsafe { ScrollBar::from(wxXmlResource_GetScrollBar(self.ptr(), str_id.ptr())) }
    }
    fn getScrolledWindow(&self, str_id: &str) -> ScrolledWindow {
        let str_id = strToString(str_id);
        unsafe { ScrolledWindow::from(wxXmlResource_GetScrolledWindow(self.ptr(), str_id.ptr())) }
    }
    fn getSlider(&self, str_id: &str) -> Slider {
        let str_id = strToString(str_id);
        unsafe { Slider::from(wxXmlResource_GetSlider(self.ptr(), str_id.ptr())) }
    }
    fn getSpinButton(&self, str_id: &str) -> SpinButton {
        let str_id = strToString(str_id);
        unsafe { SpinButton::from(wxXmlResource_GetSpinButton(self.ptr(), str_id.ptr())) }
    }
    fn getSpinCtrl(&self, str_id: &str) -> SpinCtrl {
        let str_id = strToString(str_id);
        unsafe { SpinCtrl::from(wxXmlResource_GetSpinCtrl(self.ptr(), str_id.ptr())) }
    }
    fn getSplitterWindow(&self, str_id: &str) -> SplitterWindow {
        let str_id = strToString(str_id);
        unsafe { SplitterWindow::from(wxXmlResource_GetSplitterWindow(self.ptr(), str_id.ptr())) }
    }
    fn getStaticBitmap(&self, str_id: &str) -> StaticBitmap {
        let str_id = strToString(str_id);
        unsafe { StaticBitmap::from(wxXmlResource_GetStaticBitmap(self.ptr(), str_id.ptr())) }
    }
    fn getStaticBox(&self, str_id: &str) -> StaticBox {
        let str_id = strToString(str_id);
        unsafe { StaticBox::from(wxXmlResource_GetStaticBox(self.ptr(), str_id.ptr())) }
    }
    fn getStaticLine(&self, str_id: &str) -> StaticLine {
        let str_id = strToString(str_id);
        unsafe { StaticLine::from(wxXmlResource_GetStaticLine(self.ptr(), str_id.ptr())) }
    }
    fn getStaticText(&self, str_id: &str) -> StaticText {
        let str_id = strToString(str_id);
        unsafe { StaticText::from(wxXmlResource_GetStaticText(self.ptr(), str_id.ptr())) }
    }
    fn getTextCtrl(&self, str_id: &str) -> TextCtrl {
        let str_id = strToString(str_id);
        unsafe { TextCtrl::from(wxXmlResource_GetTextCtrl(self.ptr(), str_id.ptr())) }
    }
    fn getTreeCtrl(&self, str_id: &str) -> TreeCtrl {
        let str_id = strToString(str_id);
        unsafe { TreeCtrl::from(wxXmlResource_GetTreeCtrl(self.ptr(), str_id.ptr())) }
    }
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = strToString(filemask);
        unsafe { wxXmlResource_Unload(self.ptr(), filemask.ptr()) }
    }
    fn set(&self, res: &XmlResourceMethods) -> XmlResource {
        unsafe { XmlResource::from(wxXmlResource_Set(self.ptr(), res.ptr())) }
    }
    fn setDomain(&self, domain: &str) {
        let domain = strToString(domain);
        unsafe { wxXmlResource_SetDomain(self.ptr(), domain.ptr()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.ptr(), flags) }
    }
    fn getStyledTextCtrl(&self, str_id: &str) -> StyledTextCtrl {
        let str_id = strToString(str_id);
        unsafe { StyledTextCtrl::from(wxXmlResource_GetStyledTextCtrl(self.ptr(), str_id.ptr())) }
    }
}

