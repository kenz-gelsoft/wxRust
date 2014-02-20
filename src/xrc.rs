use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;
use html::*;
use stc::*;

pub struct wxXmlResource { ptr: *mut c_void }
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxXmlResource {
    pub fn from(ptr: *mut c_void) -> wxXmlResource { wxXmlResource { ptr: ptr } }
    pub fn null() -> wxXmlResource { wxXmlResource::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> wxXmlResource {
        unsafe { wxXmlResource { ptr: wxXmlResource_Create(flags) } }
    }
    pub fn newFromFile(filemask: &str, flags: c_int) -> wxXmlResource {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource { ptr: wxXmlResource_CreateFromFile(filemask.ptr(), flags) } }
    }
    pub fn get() -> wxXmlResource {
        unsafe { wxXmlResource { ptr: wxXmlResource_Get() } }
    }
}

pub trait _wxXmlResource : _wxObject {
    fn addHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.ptr(), handler.ptr()) }
    }
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.ptr(), factory) }
    }
    fn attachUnknownControl<T: _wxControl, U: _wxWindow>(&self, control: &T, parent: &U) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.ptr(), control.ptr(), parent.ptr()) }
    }
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.ptr()) }
    }
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.ptr(), major, minor, release, revision) }
    }
    fn getDomain(&self) -> ~str {
        unsafe { wxString { ptr: wxXmlResource_GetDomain(self.ptr()) }.to_str() }
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
    fn insertHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.ptr(), handler.ptr()) }
    }
    fn load(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Load(self.ptr(), filemask.ptr()) }
    }
    fn loadBitmap<T: _wxBitmap>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadBitmap(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadDialog<T: _wxWindow>(&self, parent: &T, name: &str) -> wxDialog {
        let name = wxT(name);
        unsafe { wxDialog { ptr: wxXmlResource_LoadDialog(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadFrame<T: _wxWindow>(&self, parent: &T, name: &str) -> wxFrame {
        let name = wxT(name);
        unsafe { wxFrame { ptr: wxXmlResource_LoadFrame(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadIcon<T: _wxIcon>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadIcon(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadMenu(&self, name: &str) -> wxMenu {
        let name = wxT(name);
        unsafe { wxMenu { ptr: wxXmlResource_LoadMenu(self.ptr(), name.ptr()) } }
    }
    fn loadMenuBar<T: _wxWindow>(&self, parent: &T, name: &str) -> wxMenuBar {
        let name = wxT(name);
        unsafe { wxMenuBar { ptr: wxXmlResource_LoadMenuBar(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadPanel<T: _wxWindow>(&self, parent: &T, name: &str) -> wxPanel {
        let name = wxT(name);
        unsafe { wxPanel { ptr: wxXmlResource_LoadPanel(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadToolBar<T: _wxWindow>(&self, parent: &T, name: &str) -> wxToolBar {
        let name = wxT(name);
        unsafe { wxToolBar { ptr: wxXmlResource_LoadToolBar(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn getSizer(&self, str_id: &str) -> wxSizer {
        let str_id = wxT(str_id);
        unsafe { wxSizer { ptr: wxXmlResource_GetSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getBoxSizer(&self, str_id: &str) -> wxBoxSizer {
        let str_id = wxT(str_id);
        unsafe { wxBoxSizer { ptr: wxXmlResource_GetBoxSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBoxSizer(&self, str_id: &str) -> wxStaticBoxSizer {
        let str_id = wxT(str_id);
        unsafe { wxStaticBoxSizer { ptr: wxXmlResource_GetStaticBoxSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getGridSizer(&self, str_id: &str) -> wxGridSizer {
        let str_id = wxT(str_id);
        unsafe { wxGridSizer { ptr: wxXmlResource_GetGridSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getFlexGridSizer(&self, str_id: &str) -> wxFlexGridSizer {
        let str_id = wxT(str_id);
        unsafe { wxFlexGridSizer { ptr: wxXmlResource_GetFlexGridSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getBitmapButton(&self, str_id: &str) -> wxBitmapButton {
        let str_id = wxT(str_id);
        unsafe { wxBitmapButton { ptr: wxXmlResource_GetBitmapButton(self.ptr(), str_id.ptr()) } }
    }
    fn getButton(&self, str_id: &str) -> wxButton {
        let str_id = wxT(str_id);
        unsafe { wxButton { ptr: wxXmlResource_GetButton(self.ptr(), str_id.ptr()) } }
    }
    fn getCalendarCtrl(&self, str_id: &str) -> wxCalendarCtrl {
        let str_id = wxT(str_id);
        unsafe { wxCalendarCtrl { ptr: wxXmlResource_GetCalendarCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getCheckBox(&self, str_id: &str) -> wxCheckBox {
        let str_id = wxT(str_id);
        unsafe { wxCheckBox { ptr: wxXmlResource_GetCheckBox(self.ptr(), str_id.ptr()) } }
    }
    fn getCheckListBox(&self, str_id: &str) -> wxCheckListBox {
        let str_id = wxT(str_id);
        unsafe { wxCheckListBox { ptr: wxXmlResource_GetCheckListBox(self.ptr(), str_id.ptr()) } }
    }
    fn getChoice(&self, str_id: &str) -> wxChoice {
        let str_id = wxT(str_id);
        unsafe { wxChoice { ptr: wxXmlResource_GetChoice(self.ptr(), str_id.ptr()) } }
    }
    fn getComboBox(&self, str_id: &str) -> wxComboBox {
        let str_id = wxT(str_id);
        unsafe { wxComboBox { ptr: wxXmlResource_GetComboBox(self.ptr(), str_id.ptr()) } }
    }
    fn getGauge(&self, str_id: &str) -> wxGauge {
        let str_id = wxT(str_id);
        unsafe { wxGauge { ptr: wxXmlResource_GetGauge(self.ptr(), str_id.ptr()) } }
    }
    fn getGrid(&self, str_id: &str) -> wxGrid {
        let str_id = wxT(str_id);
        unsafe { wxGrid { ptr: wxXmlResource_GetGrid(self.ptr(), str_id.ptr()) } }
    }
    fn getHtmlWindow(&self, str_id: &str) -> wxHtmlWindow {
        let str_id = wxT(str_id);
        unsafe { wxHtmlWindow { ptr: wxXmlResource_GetHtmlWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getListBox(&self, str_id: &str) -> wxListBox {
        let str_id = wxT(str_id);
        unsafe { wxListBox { ptr: wxXmlResource_GetListBox(self.ptr(), str_id.ptr()) } }
    }
    fn getListCtrl(&self, str_id: &str) -> wxListCtrl {
        let str_id = wxT(str_id);
        unsafe { wxListCtrl { ptr: wxXmlResource_GetListCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getMDIChildFrame(&self, str_id: &str) -> wxMDIChildFrame {
        let str_id = wxT(str_id);
        unsafe { wxMDIChildFrame { ptr: wxXmlResource_GetMDIChildFrame(self.ptr(), str_id.ptr()) } }
    }
    fn getMDIParentFrame(&self, str_id: &str) -> wxMDIParentFrame {
        let str_id = wxT(str_id);
        unsafe { wxMDIParentFrame { ptr: wxXmlResource_GetMDIParentFrame(self.ptr(), str_id.ptr()) } }
    }
    fn getMenu(&self, str_id: &str) -> wxMenu {
        let str_id = wxT(str_id);
        unsafe { wxMenu { ptr: wxXmlResource_GetMenu(self.ptr(), str_id.ptr()) } }
    }
    fn getMenuBar(&self, str_id: &str) -> wxMenuBar {
        let str_id = wxT(str_id);
        unsafe { wxMenuBar { ptr: wxXmlResource_GetMenuBar(self.ptr(), str_id.ptr()) } }
    }
    fn getMenuItem(&self, str_id: &str) -> wxMenuItem {
        let str_id = wxT(str_id);
        unsafe { wxMenuItem { ptr: wxXmlResource_GetMenuItem(self.ptr(), str_id.ptr()) } }
    }
    fn getNotebook(&self, str_id: &str) -> wxNotebook {
        let str_id = wxT(str_id);
        unsafe { wxNotebook { ptr: wxXmlResource_GetNotebook(self.ptr(), str_id.ptr()) } }
    }
    fn getPanel(&self, str_id: &str) -> wxPanel {
        let str_id = wxT(str_id);
        unsafe { wxPanel { ptr: wxXmlResource_GetPanel(self.ptr(), str_id.ptr()) } }
    }
    fn getRadioButton(&self, str_id: &str) -> wxRadioButton {
        let str_id = wxT(str_id);
        unsafe { wxRadioButton { ptr: wxXmlResource_GetRadioButton(self.ptr(), str_id.ptr()) } }
    }
    fn getRadioBox(&self, str_id: &str) -> wxRadioBox {
        let str_id = wxT(str_id);
        unsafe { wxRadioBox { ptr: wxXmlResource_GetRadioBox(self.ptr(), str_id.ptr()) } }
    }
    fn getScrollBar(&self, str_id: &str) -> wxScrollBar {
        let str_id = wxT(str_id);
        unsafe { wxScrollBar { ptr: wxXmlResource_GetScrollBar(self.ptr(), str_id.ptr()) } }
    }
    fn getScrolledWindow(&self, str_id: &str) -> wxScrolledWindow {
        let str_id = wxT(str_id);
        unsafe { wxScrolledWindow { ptr: wxXmlResource_GetScrolledWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getSlider(&self, str_id: &str) -> wxSlider {
        let str_id = wxT(str_id);
        unsafe { wxSlider { ptr: wxXmlResource_GetSlider(self.ptr(), str_id.ptr()) } }
    }
    fn getSpinButton(&self, str_id: &str) -> wxSpinButton {
        let str_id = wxT(str_id);
        unsafe { wxSpinButton { ptr: wxXmlResource_GetSpinButton(self.ptr(), str_id.ptr()) } }
    }
    fn getSpinCtrl(&self, str_id: &str) -> wxSpinCtrl {
        let str_id = wxT(str_id);
        unsafe { wxSpinCtrl { ptr: wxXmlResource_GetSpinCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getSplitterWindow(&self, str_id: &str) -> wxSplitterWindow {
        let str_id = wxT(str_id);
        unsafe { wxSplitterWindow { ptr: wxXmlResource_GetSplitterWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBitmap(&self, str_id: &str) -> wxStaticBitmap {
        let str_id = wxT(str_id);
        unsafe { wxStaticBitmap { ptr: wxXmlResource_GetStaticBitmap(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBox(&self, str_id: &str) -> wxStaticBox {
        let str_id = wxT(str_id);
        unsafe { wxStaticBox { ptr: wxXmlResource_GetStaticBox(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticLine(&self, str_id: &str) -> wxStaticLine {
        let str_id = wxT(str_id);
        unsafe { wxStaticLine { ptr: wxXmlResource_GetStaticLine(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticText(&self, str_id: &str) -> wxStaticText {
        let str_id = wxT(str_id);
        unsafe { wxStaticText { ptr: wxXmlResource_GetStaticText(self.ptr(), str_id.ptr()) } }
    }
    fn getTextCtrl(&self, str_id: &str) -> wxTextCtrl {
        let str_id = wxT(str_id);
        unsafe { wxTextCtrl { ptr: wxXmlResource_GetTextCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getTreeCtrl(&self, str_id: &str) -> wxTreeCtrl {
        let str_id = wxT(str_id);
        unsafe { wxTreeCtrl { ptr: wxXmlResource_GetTreeCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Unload(self.ptr(), filemask.ptr()) }
    }
    fn set(&self, res: &_wxXmlResource) -> wxXmlResource {
        unsafe { wxXmlResource { ptr: wxXmlResource_Set(self.ptr(), res.ptr()) } }
    }
    fn setDomain(&self, domain: &str) {
        let domain = wxT(domain);
        unsafe { wxXmlResource_SetDomain(self.ptr(), domain.ptr()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.ptr(), flags) }
    }
    fn getStyledTextCtrl(&self, str_id: &str) -> wxStyledTextCtrl {
        let str_id = wxT(str_id);
        unsafe { wxStyledTextCtrl { ptr: wxXmlResource_GetStyledTextCtrl(self.ptr(), str_id.ptr()) } }
    }
}

