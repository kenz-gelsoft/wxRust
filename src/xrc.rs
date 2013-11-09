use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;
use html::*;
use stc::*;

pub struct wxXmlResource(*mut c_void);
impl _wxXmlResource for wxXmlResource {}
impl _wxObject for wxXmlResource { fn handle(&self) -> *mut c_void { **self } }

impl wxXmlResource {
    pub fn from(handle: *mut c_void) -> @wxXmlResource { @wxXmlResource(handle) }
    pub fn null() -> @wxXmlResource { wxXmlResource::from(0 as *mut c_void) }
    
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn new(flags: c_int) -> @wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Create(flags)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn newFromFile(filemask: &str, flags: c_int) -> @wxXmlResource {
        let filemask = wxT(filemask);
        unsafe { @wxXmlResource(wxXmlResource_CreateFromFile(filemask.handle(), flags)) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    pub fn get() -> @wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Get()) }
    }
}

pub trait _wxXmlResource : _wxObject {
    #[fixed_stack_segment]
    #[inline(never)]
    fn addHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.handle(), factory) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn attachUnknownControl<T: _wxControl, U: _wxWindow>(&self, control: &T, parent: &U) -> c_int {
        unsafe { wxXmlResource_AttachUnknownControl(self.handle(), control.handle(), parent.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn clearHandlers(&self) {
        unsafe { wxXmlResource_ClearHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn compareVersion(&self, major: c_int, minor: c_int, release: c_int, revision: c_int) -> c_int {
        unsafe { wxXmlResource_CompareVersion(self.handle(), major, minor, release, revision) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getDomain(&self) -> ~str {
        unsafe { wxString { handle: wxXmlResource_GetDomain(self.handle()) }.to_str() }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlags(&self) -> c_int {
        unsafe { wxXmlResource_GetFlags(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getVersion(&self) -> c_long {
        unsafe { wxXmlResource_GetVersion(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getXRCID(&self, str_id: &str) -> c_int {
        let str_id = wxT(str_id);
        unsafe { wxXmlResource_GetXRCID(self.handle(), str_id.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn initAllHandlers(&self) {
        unsafe { wxXmlResource_InitAllHandlers(self.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn insertHandler<T: _wxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.handle(), handler.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn load(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Load(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadBitmap<T: _wxBitmap>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadBitmap(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadDialog<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxDialog {
        let name = wxT(name);
        unsafe { @wxDialog(wxXmlResource_LoadDialog(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadFrame<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxFrame {
        let name = wxT(name);
        unsafe { @wxFrame(wxXmlResource_LoadFrame(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadIcon<T: _wxIcon>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadIcon(self.handle(), name.handle(), _ref.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadMenu(&self, name: &str) -> @wxMenu {
        let name = wxT(name);
        unsafe { @wxMenu(wxXmlResource_LoadMenu(self.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadMenuBar<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxMenuBar {
        let name = wxT(name);
        unsafe { @wxMenuBar(wxXmlResource_LoadMenuBar(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadPanel<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxPanel {
        let name = wxT(name);
        unsafe { @wxPanel(wxXmlResource_LoadPanel(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn loadToolBar<T: _wxWindow>(&self, parent: &T, name: &str) -> @wxToolBar {
        let name = wxT(name);
        unsafe { @wxToolBar(wxXmlResource_LoadToolBar(self.handle(), parent.handle(), name.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSizer(&self, str_id: &str) -> @wxSizer {
        let str_id = wxT(str_id);
        unsafe { @wxSizer(wxXmlResource_GetSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBoxSizer(&self, str_id: &str) -> @wxBoxSizer {
        let str_id = wxT(str_id);
        unsafe { @wxBoxSizer(wxXmlResource_GetBoxSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBoxSizer(&self, str_id: &str) -> @wxStaticBoxSizer {
        let str_id = wxT(str_id);
        unsafe { @wxStaticBoxSizer(wxXmlResource_GetStaticBoxSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGridSizer(&self, str_id: &str) -> @wxGridSizer {
        let str_id = wxT(str_id);
        unsafe { @wxGridSizer(wxXmlResource_GetGridSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getFlexGridSizer(&self, str_id: &str) -> @wxFlexGridSizer {
        let str_id = wxT(str_id);
        unsafe { @wxFlexGridSizer(wxXmlResource_GetFlexGridSizer(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getBitmapButton(&self, str_id: &str) -> @wxBitmapButton {
        let str_id = wxT(str_id);
        unsafe { @wxBitmapButton(wxXmlResource_GetBitmapButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getButton(&self, str_id: &str) -> @wxButton {
        let str_id = wxT(str_id);
        unsafe { @wxButton(wxXmlResource_GetButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCalendarCtrl(&self, str_id: &str) -> @wxCalendarCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxCalendarCtrl(wxXmlResource_GetCalendarCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCheckBox(&self, str_id: &str) -> @wxCheckBox {
        let str_id = wxT(str_id);
        unsafe { @wxCheckBox(wxXmlResource_GetCheckBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getCheckListBox(&self, str_id: &str) -> @wxCheckListBox {
        let str_id = wxT(str_id);
        unsafe { @wxCheckListBox(wxXmlResource_GetCheckListBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getChoice(&self, str_id: &str) -> @wxChoice {
        let str_id = wxT(str_id);
        unsafe { @wxChoice(wxXmlResource_GetChoice(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getComboBox(&self, str_id: &str) -> @wxComboBox {
        let str_id = wxT(str_id);
        unsafe { @wxComboBox(wxXmlResource_GetComboBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGauge(&self, str_id: &str) -> @wxGauge {
        let str_id = wxT(str_id);
        unsafe { @wxGauge(wxXmlResource_GetGauge(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getGrid(&self, str_id: &str) -> @wxGrid {
        let str_id = wxT(str_id);
        unsafe { @wxGrid(wxXmlResource_GetGrid(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getHtmlWindow(&self, str_id: &str) -> @wxHtmlWindow {
        let str_id = wxT(str_id);
        unsafe { @wxHtmlWindow(wxXmlResource_GetHtmlWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getListBox(&self, str_id: &str) -> @wxListBox {
        let str_id = wxT(str_id);
        unsafe { @wxListBox(wxXmlResource_GetListBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getListCtrl(&self, str_id: &str) -> @wxListCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxListCtrl(wxXmlResource_GetListCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMDIChildFrame(&self, str_id: &str) -> @wxMDIChildFrame {
        let str_id = wxT(str_id);
        unsafe { @wxMDIChildFrame(wxXmlResource_GetMDIChildFrame(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMDIParentFrame(&self, str_id: &str) -> @wxMDIParentFrame {
        let str_id = wxT(str_id);
        unsafe { @wxMDIParentFrame(wxXmlResource_GetMDIParentFrame(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenu(&self, str_id: &str) -> @wxMenu {
        let str_id = wxT(str_id);
        unsafe { @wxMenu(wxXmlResource_GetMenu(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuBar(&self, str_id: &str) -> @wxMenuBar {
        let str_id = wxT(str_id);
        unsafe { @wxMenuBar(wxXmlResource_GetMenuBar(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getMenuItem(&self, str_id: &str) -> @wxMenuItem {
        let str_id = wxT(str_id);
        unsafe { @wxMenuItem(wxXmlResource_GetMenuItem(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getNotebook(&self, str_id: &str) -> @wxNotebook {
        let str_id = wxT(str_id);
        unsafe { @wxNotebook(wxXmlResource_GetNotebook(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getPanel(&self, str_id: &str) -> @wxPanel {
        let str_id = wxT(str_id);
        unsafe { @wxPanel(wxXmlResource_GetPanel(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRadioButton(&self, str_id: &str) -> @wxRadioButton {
        let str_id = wxT(str_id);
        unsafe { @wxRadioButton(wxXmlResource_GetRadioButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getRadioBox(&self, str_id: &str) -> @wxRadioBox {
        let str_id = wxT(str_id);
        unsafe { @wxRadioBox(wxXmlResource_GetRadioBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrollBar(&self, str_id: &str) -> @wxScrollBar {
        let str_id = wxT(str_id);
        unsafe { @wxScrollBar(wxXmlResource_GetScrollBar(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getScrolledWindow(&self, str_id: &str) -> @wxScrolledWindow {
        let str_id = wxT(str_id);
        unsafe { @wxScrolledWindow(wxXmlResource_GetScrolledWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSlider(&self, str_id: &str) -> @wxSlider {
        let str_id = wxT(str_id);
        unsafe { @wxSlider(wxXmlResource_GetSlider(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpinButton(&self, str_id: &str) -> @wxSpinButton {
        let str_id = wxT(str_id);
        unsafe { @wxSpinButton(wxXmlResource_GetSpinButton(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSpinCtrl(&self, str_id: &str) -> @wxSpinCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxSpinCtrl(wxXmlResource_GetSpinCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getSplitterWindow(&self, str_id: &str) -> @wxSplitterWindow {
        let str_id = wxT(str_id);
        unsafe { @wxSplitterWindow(wxXmlResource_GetSplitterWindow(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBitmap(&self, str_id: &str) -> @wxStaticBitmap {
        let str_id = wxT(str_id);
        unsafe { @wxStaticBitmap(wxXmlResource_GetStaticBitmap(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticBox(&self, str_id: &str) -> @wxStaticBox {
        let str_id = wxT(str_id);
        unsafe { @wxStaticBox(wxXmlResource_GetStaticBox(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticLine(&self, str_id: &str) -> @wxStaticLine {
        let str_id = wxT(str_id);
        unsafe { @wxStaticLine(wxXmlResource_GetStaticLine(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStaticText(&self, str_id: &str) -> @wxStaticText {
        let str_id = wxT(str_id);
        unsafe { @wxStaticText(wxXmlResource_GetStaticText(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTextCtrl(&self, str_id: &str) -> @wxTextCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxTextCtrl(wxXmlResource_GetTextCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getTreeCtrl(&self, str_id: &str) -> @wxTreeCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxTreeCtrl(wxXmlResource_GetTreeCtrl(self.handle(), str_id.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Unload(self.handle(), filemask.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn set(&self, res: &_wxXmlResource) -> @wxXmlResource {
        unsafe { @wxXmlResource(wxXmlResource_Set(self.handle(), res.handle())) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setDomain(&self, domain: &str) {
        let domain = wxT(domain);
        unsafe { wxXmlResource_SetDomain(self.handle(), domain.handle()) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.handle(), flags) }
    }
    #[fixed_stack_segment]
    #[inline(never)]
    fn getStyledTextCtrl(&self, str_id: &str) -> @wxStyledTextCtrl {
        let str_id = wxT(str_id);
        unsafe { @wxStyledTextCtrl(wxXmlResource_GetStyledTextCtrl(self.handle(), str_id.handle())) }
    }
}

