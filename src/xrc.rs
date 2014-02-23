use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;
use html::*;
use stc::*;

pub struct XmlResource { ptr: *mut c_void }
impl TXmlResource for XmlResource {}
impl TObject for XmlResource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl XmlResource {
    pub fn from(ptr: *mut c_void) -> XmlResource { XmlResource { ptr: ptr } }
    pub fn null() -> XmlResource { XmlResource::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> XmlResource {
        unsafe { XmlResource { ptr: wxXmlResource_Create(flags) } }
    }
    pub fn newFromFile(filemask: &str, flags: c_int) -> XmlResource {
        let filemask = wxT(filemask);
        unsafe { XmlResource { ptr: wxXmlResource_CreateFromFile(filemask.ptr(), flags) } }
    }
    pub fn get() -> XmlResource {
        unsafe { XmlResource { ptr: wxXmlResource_Get() } }
    }
}

pub trait TXmlResource : TObject {
    fn addHandler<T: TEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.ptr(), handler.ptr()) }
    }
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.ptr(), factory) }
    }
    fn attachUnknownControl<T: TControl, U: TWindow>(&self, control: &T, parent: &U) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.ptr(), control.ptr(), parent.ptr()) }
    }
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.ptr()) }
    }
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.ptr(), major, minor, release, revision) }
    }
    fn getDomain(&self) -> ~str {
        unsafe { WxString { ptr: wxXmlResource_GetDomain(self.ptr()) }.to_str() }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxXmlResource_GetFlags(self.ptr()) }
    }
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self.ptr()) }
    }
    fn getXRCID(&self, str_id: &str) -> c_int {
        let str_id = wxT(str_id);
        unsafe { wxXmlResource_GetXRCID(self.ptr(), str_id.ptr()) }
    }
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.ptr()) }
    }
    fn insertHandler<T: TEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.ptr(), handler.ptr()) }
    }
    fn load(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Load(self.ptr(), filemask.ptr()) }
    }
    fn loadBitmap<T: TBitmap>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadBitmap(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadDialog<T: TWindow>(&self, parent: &T, name: &str) -> Dialog {
        let name = wxT(name);
        unsafe { Dialog { ptr: wxXmlResource_LoadDialog(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadFrame<T: TWindow>(&self, parent: &T, name: &str) -> Frame {
        let name = wxT(name);
        unsafe { Frame { ptr: wxXmlResource_LoadFrame(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadIcon<T: TIcon>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadIcon(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadMenu(&self, name: &str) -> Menu {
        let name = wxT(name);
        unsafe { Menu { ptr: wxXmlResource_LoadMenu(self.ptr(), name.ptr()) } }
    }
    fn loadMenuBar<T: TWindow>(&self, parent: &T, name: &str) -> MenuBar {
        let name = wxT(name);
        unsafe { MenuBar { ptr: wxXmlResource_LoadMenuBar(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadPanel<T: TWindow>(&self, parent: &T, name: &str) -> Panel {
        let name = wxT(name);
        unsafe { Panel { ptr: wxXmlResource_LoadPanel(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadToolBar<T: TWindow>(&self, parent: &T, name: &str) -> ToolBar {
        let name = wxT(name);
        unsafe { ToolBar { ptr: wxXmlResource_LoadToolBar(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn getSizer(&self, str_id: &str) -> Sizer {
        let str_id = wxT(str_id);
        unsafe { Sizer { ptr: wxXmlResource_GetSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getBoxSizer(&self, str_id: &str) -> BoxSizer {
        let str_id = wxT(str_id);
        unsafe { BoxSizer { ptr: wxXmlResource_GetBoxSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBoxSizer(&self, str_id: &str) -> StaticBoxSizer {
        let str_id = wxT(str_id);
        unsafe { StaticBoxSizer { ptr: wxXmlResource_GetStaticBoxSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getGridSizer(&self, str_id: &str) -> GridSizer {
        let str_id = wxT(str_id);
        unsafe { GridSizer { ptr: wxXmlResource_GetGridSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getFlexGridSizer(&self, str_id: &str) -> FlexGridSizer {
        let str_id = wxT(str_id);
        unsafe { FlexGridSizer { ptr: wxXmlResource_GetFlexGridSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getBitmapButton(&self, str_id: &str) -> BitmapButton {
        let str_id = wxT(str_id);
        unsafe { BitmapButton { ptr: wxXmlResource_GetBitmapButton(self.ptr(), str_id.ptr()) } }
    }
    fn getButton(&self, str_id: &str) -> Button {
        let str_id = wxT(str_id);
        unsafe { Button { ptr: wxXmlResource_GetButton(self.ptr(), str_id.ptr()) } }
    }
    fn getCalendarCtrl(&self, str_id: &str) -> CalendarCtrl {
        let str_id = wxT(str_id);
        unsafe { CalendarCtrl { ptr: wxXmlResource_GetCalendarCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getCheckBox(&self, str_id: &str) -> CheckBox {
        let str_id = wxT(str_id);
        unsafe { CheckBox { ptr: wxXmlResource_GetCheckBox(self.ptr(), str_id.ptr()) } }
    }
    fn getCheckListBox(&self, str_id: &str) -> CheckListBox {
        let str_id = wxT(str_id);
        unsafe { CheckListBox { ptr: wxXmlResource_GetCheckListBox(self.ptr(), str_id.ptr()) } }
    }
    fn getChoice(&self, str_id: &str) -> Choice {
        let str_id = wxT(str_id);
        unsafe { Choice { ptr: wxXmlResource_GetChoice(self.ptr(), str_id.ptr()) } }
    }
    fn getComboBox(&self, str_id: &str) -> ComboBox {
        let str_id = wxT(str_id);
        unsafe { ComboBox { ptr: wxXmlResource_GetComboBox(self.ptr(), str_id.ptr()) } }
    }
    fn getGauge(&self, str_id: &str) -> Gauge {
        let str_id = wxT(str_id);
        unsafe { Gauge { ptr: wxXmlResource_GetGauge(self.ptr(), str_id.ptr()) } }
    }
    fn getGrid(&self, str_id: &str) -> Grid {
        let str_id = wxT(str_id);
        unsafe { Grid { ptr: wxXmlResource_GetGrid(self.ptr(), str_id.ptr()) } }
    }
    fn getHtmlWindow(&self, str_id: &str) -> HtmlWindow {
        let str_id = wxT(str_id);
        unsafe { HtmlWindow { ptr: wxXmlResource_GetHtmlWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getListBox(&self, str_id: &str) -> ListBox {
        let str_id = wxT(str_id);
        unsafe { ListBox { ptr: wxXmlResource_GetListBox(self.ptr(), str_id.ptr()) } }
    }
    fn getListCtrl(&self, str_id: &str) -> ListCtrl {
        let str_id = wxT(str_id);
        unsafe { ListCtrl { ptr: wxXmlResource_GetListCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getMDIChildFrame(&self, str_id: &str) -> MDIChildFrame {
        let str_id = wxT(str_id);
        unsafe { MDIChildFrame { ptr: wxXmlResource_GetMDIChildFrame(self.ptr(), str_id.ptr()) } }
    }
    fn getMDIParentFrame(&self, str_id: &str) -> MDIParentFrame {
        let str_id = wxT(str_id);
        unsafe { MDIParentFrame { ptr: wxXmlResource_GetMDIParentFrame(self.ptr(), str_id.ptr()) } }
    }
    fn getMenu(&self, str_id: &str) -> Menu {
        let str_id = wxT(str_id);
        unsafe { Menu { ptr: wxXmlResource_GetMenu(self.ptr(), str_id.ptr()) } }
    }
    fn getMenuBar(&self, str_id: &str) -> MenuBar {
        let str_id = wxT(str_id);
        unsafe { MenuBar { ptr: wxXmlResource_GetMenuBar(self.ptr(), str_id.ptr()) } }
    }
    fn getMenuItem(&self, str_id: &str) -> MenuItem {
        let str_id = wxT(str_id);
        unsafe { MenuItem { ptr: wxXmlResource_GetMenuItem(self.ptr(), str_id.ptr()) } }
    }
    fn getNotebook(&self, str_id: &str) -> Notebook {
        let str_id = wxT(str_id);
        unsafe { Notebook { ptr: wxXmlResource_GetNotebook(self.ptr(), str_id.ptr()) } }
    }
    fn getPanel(&self, str_id: &str) -> Panel {
        let str_id = wxT(str_id);
        unsafe { Panel { ptr: wxXmlResource_GetPanel(self.ptr(), str_id.ptr()) } }
    }
    fn getRadioButton(&self, str_id: &str) -> RadioButton {
        let str_id = wxT(str_id);
        unsafe { RadioButton { ptr: wxXmlResource_GetRadioButton(self.ptr(), str_id.ptr()) } }
    }
    fn getRadioBox(&self, str_id: &str) -> RadioBox {
        let str_id = wxT(str_id);
        unsafe { RadioBox { ptr: wxXmlResource_GetRadioBox(self.ptr(), str_id.ptr()) } }
    }
    fn getScrollBar(&self, str_id: &str) -> ScrollBar {
        let str_id = wxT(str_id);
        unsafe { ScrollBar { ptr: wxXmlResource_GetScrollBar(self.ptr(), str_id.ptr()) } }
    }
    fn getScrolledWindow(&self, str_id: &str) -> ScrolledWindow {
        let str_id = wxT(str_id);
        unsafe { ScrolledWindow { ptr: wxXmlResource_GetScrolledWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getSlider(&self, str_id: &str) -> Slider {
        let str_id = wxT(str_id);
        unsafe { Slider { ptr: wxXmlResource_GetSlider(self.ptr(), str_id.ptr()) } }
    }
    fn getSpinButton(&self, str_id: &str) -> SpinButton {
        let str_id = wxT(str_id);
        unsafe { SpinButton { ptr: wxXmlResource_GetSpinButton(self.ptr(), str_id.ptr()) } }
    }
    fn getSpinCtrl(&self, str_id: &str) -> SpinCtrl {
        let str_id = wxT(str_id);
        unsafe { SpinCtrl { ptr: wxXmlResource_GetSpinCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getSplitterWindow(&self, str_id: &str) -> SplitterWindow {
        let str_id = wxT(str_id);
        unsafe { SplitterWindow { ptr: wxXmlResource_GetSplitterWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBitmap(&self, str_id: &str) -> StaticBitmap {
        let str_id = wxT(str_id);
        unsafe { StaticBitmap { ptr: wxXmlResource_GetStaticBitmap(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBox(&self, str_id: &str) -> StaticBox {
        let str_id = wxT(str_id);
        unsafe { StaticBox { ptr: wxXmlResource_GetStaticBox(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticLine(&self, str_id: &str) -> StaticLine {
        let str_id = wxT(str_id);
        unsafe { StaticLine { ptr: wxXmlResource_GetStaticLine(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticText(&self, str_id: &str) -> StaticText {
        let str_id = wxT(str_id);
        unsafe { StaticText { ptr: wxXmlResource_GetStaticText(self.ptr(), str_id.ptr()) } }
    }
    fn getTextCtrl(&self, str_id: &str) -> TextCtrl {
        let str_id = wxT(str_id);
        unsafe { TextCtrl { ptr: wxXmlResource_GetTextCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getTreeCtrl(&self, str_id: &str) -> TreeCtrl {
        let str_id = wxT(str_id);
        unsafe { TreeCtrl { ptr: wxXmlResource_GetTreeCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Unload(self.ptr(), filemask.ptr()) }
    }
    fn set(&self, res: &TXmlResource) -> XmlResource {
        unsafe { XmlResource { ptr: wxXmlResource_Set(self.ptr(), res.ptr()) } }
    }
    fn setDomain(&self, domain: &str) {
        let domain = wxT(domain);
        unsafe { wxXmlResource_SetDomain(self.ptr(), domain.ptr()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.ptr(), flags) }
    }
    fn getStyledTextCtrl(&self, str_id: &str) -> StyledTextCtrl {
        let str_id = wxT(str_id);
        unsafe { StyledTextCtrl { ptr: wxXmlResource_GetStyledTextCtrl(self.ptr(), str_id.ptr()) } }
    }
}

