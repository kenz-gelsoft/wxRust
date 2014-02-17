use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;
use html::*;
use stc::*;

pub struct wxXmlResource { handle: *mut c_void }
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn handle(&self) -> *mut c_void { self.handle } }

impl wxXmlResource {
    pub fn from(handle: *mut c_void) -> wxXmlResource { wxXmlResource { handle: handle } }
    pub fn null() -> wxXmlResource { wxXmlResource::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> wxXmlResource {
        unsafe { wxXmlResource { handle: wxXmlResource_Create(flags) } }
    }
    pub fn newFromFile(filemask: &str, flags: c_int) -> wxXmlResource {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource { handle: wxXmlResource_CreateFromFile(filemask.handle(), flags) } }
    }
    pub fn get() -> wxXmlResource {
        unsafe { wxXmlResource { handle: wxXmlResource_Get() } }
    }
}

pub trait _wxXmlResource : _wxObject {
    fn addHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    fn attachUnknownControl<T: _wxControl, U: _wxWindow>(&self, control: &T, parent: &U) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.handle(), control.handle(), parent.handle()) }
    }
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.handle()) }
    }
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.handle(), major, minor, release, revision) }
    }
    fn getDomain(&self) -> ~str {
        unsafe { wxString { handle: wxXmlResource_GetDomain(self.handle()) }.to_str() }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxXmlResource_GetFlags(self.handle()) }
    }
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self.handle()) }
    }
    fn getXRCID(&self, str_id: &str) -> c_int {
        let str_id = wxT(str_id);
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    fn insertHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    fn load(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    fn loadBitmap<T: _wxBitmap>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    fn loadDialog<T: _wxWindow>(&self, parent: &T, name: &str) -> wxDialog {
        let name = wxT(name);
        unsafe { wxDialog { handle: wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle()) } }
    }
    fn loadFrame<T: _wxWindow>(&self, parent: &T, name: &str) -> wxFrame {
        let name = wxT(name);
        unsafe { wxFrame { handle: wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle()) } }
    }
    fn loadIcon<T: _wxIcon>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    fn loadMenu(&self, name: &str) -> wxMenu {
        let name = wxT(name);
        unsafe { wxMenu { handle: wxXmlResource_LoadMenu(self.handle(), name.handle()) } }
    }
    fn loadMenuBar<T: _wxWindow>(&self, parent: &T, name: &str) -> wxMenuBar {
        let name = wxT(name);
        unsafe { wxMenuBar { handle: wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle()) } }
    }
    fn loadPanel<T: _wxWindow>(&self, parent: &T, name: &str) -> wxPanel {
        let name = wxT(name);
        unsafe { wxPanel { handle: wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle()) } }
    }
    fn loadToolBar<T: _wxWindow>(&self, parent: &T, name: &str) -> wxToolBar {
        let name = wxT(name);
        unsafe { wxToolBar { handle: wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle()) } }
    }
    fn getSizer(&self, str_id: &str) -> wxSizer {
        let str_id = wxT(str_id);
        unsafe { wxSizer { handle: wxXmlResource_GetSizer(self.handle(), str_id.handle()) } }
    }
    fn getBoxSizer(&self, str_id: &str) -> wxBoxSizer {
        let str_id = wxT(str_id);
        unsafe { wxBoxSizer { handle: wxXmlResource_GetBoxSizer(self.handle(), str_id.handle()) } }
    }
    fn getStaticBoxSizer(&self, str_id: &str) -> wxStaticBoxSizer {
        let str_id = wxT(str_id);
        unsafe { wxStaticBoxSizer { handle: wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle()) } }
    }
    fn getGridSizer(&self, str_id: &str) -> wxGridSizer {
        let str_id = wxT(str_id);
        unsafe { wxGridSizer { handle: wxXmlResource_GetGridSizer(self.handle(), str_id.handle()) } }
    }
    fn getFlexGridSizer(&self, str_id: &str) -> wxFlexGridSizer {
        let str_id = wxT(str_id);
        unsafe { wxFlexGridSizer { handle: wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle()) } }
    }
    fn getBitmapButton(&self, str_id: &str) -> wxBitmapButton {
        let str_id = wxT(str_id);
        unsafe { wxBitmapButton { handle: wxXmlResource_GetBitmapButton(self.handle(), str_id.handle()) } }
    }
    fn getButton(&self, str_id: &str) -> wxButton {
        let str_id = wxT(str_id);
        unsafe { wxButton { handle: wxXmlResource_GetButton(self.handle(), str_id.handle()) } }
    }
    fn getCalendarCtrl(&self, str_id: &str) -> wxCalendarCtrl {
        let str_id = wxT(str_id);
        unsafe { wxCalendarCtrl { handle: wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle()) } }
    }
    fn getCheckBox(&self, str_id: &str) -> wxCheckBox {
        let str_id = wxT(str_id);
        unsafe { wxCheckBox { handle: wxXmlResource_GetCheckBox(self.handle(), str_id.handle()) } }
    }
    fn getCheckListBox(&self, str_id: &str) -> wxCheckListBox {
        let str_id = wxT(str_id);
        unsafe { wxCheckListBox { handle: wxXmlResource_GetCheckListBox(self.handle(), str_id.handle()) } }
    }
    fn getChoice(&self, str_id: &str) -> wxChoice {
        let str_id = wxT(str_id);
        unsafe { wxChoice { handle: wxXmlResource_GetChoice(self.handle(), str_id.handle()) } }
    }
    fn getComboBox(&self, str_id: &str) -> wxComboBox {
        let str_id = wxT(str_id);
        unsafe { wxComboBox { handle: wxXmlResource_GetComboBox(self.handle(), str_id.handle()) } }
    }
    fn getGauge(&self, str_id: &str) -> wxGauge {
        let str_id = wxT(str_id);
        unsafe { wxGauge { handle: wxXmlResource_GetGauge(self.handle(), str_id.handle()) } }
    }
    fn getGrid(&self, str_id: &str) -> wxGrid {
        let str_id = wxT(str_id);
        unsafe { wxGrid { handle: wxXmlResource_GetGrid(self.handle(), str_id.handle()) } }
    }
    fn getHtmlWindow(&self, str_id: &str) -> wxHtmlWindow {
        let str_id = wxT(str_id);
        unsafe { wxHtmlWindow { handle: wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle()) } }
    }
    fn getListBox(&self, str_id: &str) -> wxListBox {
        let str_id = wxT(str_id);
        unsafe { wxListBox { handle: wxXmlResource_GetListBox(self.handle(), str_id.handle()) } }
    }
    fn getListCtrl(&self, str_id: &str) -> wxListCtrl {
        let str_id = wxT(str_id);
        unsafe { wxListCtrl { handle: wxXmlResource_GetListCtrl(self.handle(), str_id.handle()) } }
    }
    fn getMDIChildFrame(&self, str_id: &str) -> wxMDIChildFrame {
        let str_id = wxT(str_id);
        unsafe { wxMDIChildFrame { handle: wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle()) } }
    }
    fn getMDIParentFrame(&self, str_id: &str) -> wxMDIParentFrame {
        let str_id = wxT(str_id);
        unsafe { wxMDIParentFrame { handle: wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle()) } }
    }
    fn getMenu(&self, str_id: &str) -> wxMenu {
        let str_id = wxT(str_id);
        unsafe { wxMenu { handle: wxXmlResource_GetMenu(self.handle(), str_id.handle()) } }
    }
    fn getMenuBar(&self, str_id: &str) -> wxMenuBar {
        let str_id = wxT(str_id);
        unsafe { wxMenuBar { handle: wxXmlResource_GetMenuBar(self.handle(), str_id.handle()) } }
    }
    fn getMenuItem(&self, str_id: &str) -> wxMenuItem {
        let str_id = wxT(str_id);
        unsafe { wxMenuItem { handle: wxXmlResource_GetMenuItem(self.handle(), str_id.handle()) } }
    }
    fn getNotebook(&self, str_id: &str) -> wxNotebook {
        let str_id = wxT(str_id);
        unsafe { wxNotebook { handle: wxXmlResource_GetNotebook(self.handle(), str_id.handle()) } }
    }
    fn getPanel(&self, str_id: &str) -> wxPanel {
        let str_id = wxT(str_id);
        unsafe { wxPanel { handle: wxXmlResource_GetPanel(self.handle(), str_id.handle()) } }
    }
    fn getRadioButton(&self, str_id: &str) -> wxRadioButton {
        let str_id = wxT(str_id);
        unsafe { wxRadioButton { handle: wxXmlResource_GetRadioButton(self.handle(), str_id.handle()) } }
    }
    fn getRadioBox(&self, str_id: &str) -> wxRadioBox {
        let str_id = wxT(str_id);
        unsafe { wxRadioBox { handle: wxXmlResource_GetRadioBox(self.handle(), str_id.handle()) } }
    }
    fn getScrollBar(&self, str_id: &str) -> wxScrollBar {
        let str_id = wxT(str_id);
        unsafe { wxScrollBar { handle: wxXmlResource_GetScrollBar(self.handle(), str_id.handle()) } }
    }
    fn getScrolledWindow(&self, str_id: &str) -> wxScrolledWindow {
        let str_id = wxT(str_id);
        unsafe { wxScrolledWindow { handle: wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle()) } }
    }
    fn getSlider(&self, str_id: &str) -> wxSlider {
        let str_id = wxT(str_id);
        unsafe { wxSlider { handle: wxXmlResource_GetSlider(self.handle(), str_id.handle()) } }
    }
    fn getSpinButton(&self, str_id: &str) -> wxSpinButton {
        let str_id = wxT(str_id);
        unsafe { wxSpinButton { handle: wxXmlResource_GetSpinButton(self.handle(), str_id.handle()) } }
    }
    fn getSpinCtrl(&self, str_id: &str) -> wxSpinCtrl {
        let str_id = wxT(str_id);
        unsafe { wxSpinCtrl { handle: wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle()) } }
    }
    fn getSplitterWindow(&self, str_id: &str) -> wxSplitterWindow {
        let str_id = wxT(str_id);
        unsafe { wxSplitterWindow { handle: wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle()) } }
    }
    fn getStaticBitmap(&self, str_id: &str) -> wxStaticBitmap {
        let str_id = wxT(str_id);
        unsafe { wxStaticBitmap { handle: wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle()) } }
    }
    fn getStaticBox(&self, str_id: &str) -> wxStaticBox {
        let str_id = wxT(str_id);
        unsafe { wxStaticBox { handle: wxXmlResource_GetStaticBox(self.handle(), str_id.handle()) } }
    }
    fn getStaticLine(&self, str_id: &str) -> wxStaticLine {
        let str_id = wxT(str_id);
        unsafe { wxStaticLine { handle: wxXmlResource_GetStaticLine(self.handle(), str_id.handle()) } }
    }
    fn getStaticText(&self, str_id: &str) -> wxStaticText {
        let str_id = wxT(str_id);
        unsafe { wxStaticText { handle: wxXmlResource_GetStaticText(self.handle(), str_id.handle()) } }
    }
    fn getTextCtrl(&self, str_id: &str) -> wxTextCtrl {
        let str_id = wxT(str_id);
        unsafe { wxTextCtrl { handle: wxXmlResource_GetTextCtrl(self.handle(), str_id.handle()) } }
    }
    fn getTreeCtrl(&self, str_id: &str) -> wxTreeCtrl {
        let str_id = wxT(str_id);
        unsafe { wxTreeCtrl { handle: wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle()) } }
    }
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    fn set(&self, res: &_wxXmlResource) -> wxXmlResource {
        unsafe { wxXmlResource { handle: wxXmlResource_Set(self.handle(), res.handle()) } }
    }
    fn setDomain(&self, domain: &str) {
        let domain = wxT(domain);
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    fn getStyledTextCtrl(&self, str_id: &str) -> wxStyledTextCtrl {
        let str_id = wxT(str_id);
        unsafe { wxStyledTextCtrl { handle: wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle()) } }
    }
}

