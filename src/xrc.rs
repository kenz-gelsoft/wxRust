use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;
use html::*;
use stc::*;

pub struct WxXmlResource { ptr: *mut c_void }
impl TWxXmlResource for WxXmlResource {}
impl TWxObject for WxXmlResource { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxXmlResource {
    pub fn from(ptr: *mut c_void) -> WxXmlResource { WxXmlResource { ptr: ptr } }
    pub fn null() -> WxXmlResource { WxXmlResource::from(0 as *mut c_void) }
    
    pub fn new(flags: c_int) -> WxXmlResource {
        unsafe { WxXmlResource { ptr: wxXmlResource_Create(flags) } }
    }
    pub fn newFromFile(filemask: &str, flags: c_int) -> WxXmlResource {
        let filemask = wxT(filemask);
        unsafe { WxXmlResource { ptr: wxXmlResource_CreateFromFile(filemask.ptr(), flags) } }
    }
    pub fn get() -> WxXmlResource {
        unsafe { WxXmlResource { ptr: wxXmlResource_Get() } }
    }
}

pub trait TWxXmlResource : TWxObject {
    fn addHandler<T: TWxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_AddHandler(self.ptr(), handler.ptr()) }
    }
    fn addSubclassFactory(&self, factory: *mut c_void) {
        unsafe { wxXmlResource_AddSubclassFactory(self.ptr(), factory) }
    }
    fn attachUnknownControl<T: TWxControl, U: TWxWindow>(&self, control: &T, parent: &U) -> c_int {
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
    fn insertHandler<T: TWxEvtHandler>(&self, handler: &T) {
        unsafe { wxXmlResource_InsertHandler(self.ptr(), handler.ptr()) }
    }
    fn load(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Load(self.ptr(), filemask.ptr()) }
    }
    fn loadBitmap<T: TWxBitmap>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadBitmap(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadDialog<T: TWxWindow>(&self, parent: &T, name: &str) -> WxDialog {
        let name = wxT(name);
        unsafe { WxDialog { ptr: wxXmlResource_LoadDialog(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadFrame<T: TWxWindow>(&self, parent: &T, name: &str) -> WxFrame {
        let name = wxT(name);
        unsafe { WxFrame { ptr: wxXmlResource_LoadFrame(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadIcon<T: TWxIcon>(&self, name: &str, _ref: &T) {
        let name = wxT(name);
        unsafe { wxXmlResource_LoadIcon(self.ptr(), name.ptr(), _ref.ptr()) }
    }
    fn loadMenu(&self, name: &str) -> WxMenu {
        let name = wxT(name);
        unsafe { WxMenu { ptr: wxXmlResource_LoadMenu(self.ptr(), name.ptr()) } }
    }
    fn loadMenuBar<T: TWxWindow>(&self, parent: &T, name: &str) -> WxMenuBar {
        let name = wxT(name);
        unsafe { WxMenuBar { ptr: wxXmlResource_LoadMenuBar(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadPanel<T: TWxWindow>(&self, parent: &T, name: &str) -> WxPanel {
        let name = wxT(name);
        unsafe { WxPanel { ptr: wxXmlResource_LoadPanel(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn loadToolBar<T: TWxWindow>(&self, parent: &T, name: &str) -> WxToolBar {
        let name = wxT(name);
        unsafe { WxToolBar { ptr: wxXmlResource_LoadToolBar(self.ptr(), parent.ptr(), name.ptr()) } }
    }
    fn getSizer(&self, str_id: &str) -> WxSizer {
        let str_id = wxT(str_id);
        unsafe { WxSizer { ptr: wxXmlResource_GetSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getBoxSizer(&self, str_id: &str) -> WxBoxSizer {
        let str_id = wxT(str_id);
        unsafe { WxBoxSizer { ptr: wxXmlResource_GetBoxSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBoxSizer(&self, str_id: &str) -> WxStaticBoxSizer {
        let str_id = wxT(str_id);
        unsafe { WxStaticBoxSizer { ptr: wxXmlResource_GetStaticBoxSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getGridSizer(&self, str_id: &str) -> WxGridSizer {
        let str_id = wxT(str_id);
        unsafe { WxGridSizer { ptr: wxXmlResource_GetGridSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getFlexGridSizer(&self, str_id: &str) -> WxFlexGridSizer {
        let str_id = wxT(str_id);
        unsafe { WxFlexGridSizer { ptr: wxXmlResource_GetFlexGridSizer(self.ptr(), str_id.ptr()) } }
    }
    fn getBitmapButton(&self, str_id: &str) -> WxBitmapButton {
        let str_id = wxT(str_id);
        unsafe { WxBitmapButton { ptr: wxXmlResource_GetBitmapButton(self.ptr(), str_id.ptr()) } }
    }
    fn getButton(&self, str_id: &str) -> WxButton {
        let str_id = wxT(str_id);
        unsafe { WxButton { ptr: wxXmlResource_GetButton(self.ptr(), str_id.ptr()) } }
    }
    fn getCalendarCtrl(&self, str_id: &str) -> WxCalendarCtrl {
        let str_id = wxT(str_id);
        unsafe { WxCalendarCtrl { ptr: wxXmlResource_GetCalendarCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getCheckBox(&self, str_id: &str) -> WxCheckBox {
        let str_id = wxT(str_id);
        unsafe { WxCheckBox { ptr: wxXmlResource_GetCheckBox(self.ptr(), str_id.ptr()) } }
    }
    fn getCheckListBox(&self, str_id: &str) -> WxCheckListBox {
        let str_id = wxT(str_id);
        unsafe { WxCheckListBox { ptr: wxXmlResource_GetCheckListBox(self.ptr(), str_id.ptr()) } }
    }
    fn getChoice(&self, str_id: &str) -> WxChoice {
        let str_id = wxT(str_id);
        unsafe { WxChoice { ptr: wxXmlResource_GetChoice(self.ptr(), str_id.ptr()) } }
    }
    fn getComboBox(&self, str_id: &str) -> WxComboBox {
        let str_id = wxT(str_id);
        unsafe { WxComboBox { ptr: wxXmlResource_GetComboBox(self.ptr(), str_id.ptr()) } }
    }
    fn getGauge(&self, str_id: &str) -> WxGauge {
        let str_id = wxT(str_id);
        unsafe { WxGauge { ptr: wxXmlResource_GetGauge(self.ptr(), str_id.ptr()) } }
    }
    fn getGrid(&self, str_id: &str) -> WxGrid {
        let str_id = wxT(str_id);
        unsafe { WxGrid { ptr: wxXmlResource_GetGrid(self.ptr(), str_id.ptr()) } }
    }
    fn getHtmlWindow(&self, str_id: &str) -> WxHtmlWindow {
        let str_id = wxT(str_id);
        unsafe { WxHtmlWindow { ptr: wxXmlResource_GetHtmlWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getListBox(&self, str_id: &str) -> WxListBox {
        let str_id = wxT(str_id);
        unsafe { WxListBox { ptr: wxXmlResource_GetListBox(self.ptr(), str_id.ptr()) } }
    }
    fn getListCtrl(&self, str_id: &str) -> WxListCtrl {
        let str_id = wxT(str_id);
        unsafe { WxListCtrl { ptr: wxXmlResource_GetListCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getMDIChildFrame(&self, str_id: &str) -> WxMDIChildFrame {
        let str_id = wxT(str_id);
        unsafe { WxMDIChildFrame { ptr: wxXmlResource_GetMDIChildFrame(self.ptr(), str_id.ptr()) } }
    }
    fn getMDIParentFrame(&self, str_id: &str) -> WxMDIParentFrame {
        let str_id = wxT(str_id);
        unsafe { WxMDIParentFrame { ptr: wxXmlResource_GetMDIParentFrame(self.ptr(), str_id.ptr()) } }
    }
    fn getMenu(&self, str_id: &str) -> WxMenu {
        let str_id = wxT(str_id);
        unsafe { WxMenu { ptr: wxXmlResource_GetMenu(self.ptr(), str_id.ptr()) } }
    }
    fn getMenuBar(&self, str_id: &str) -> WxMenuBar {
        let str_id = wxT(str_id);
        unsafe { WxMenuBar { ptr: wxXmlResource_GetMenuBar(self.ptr(), str_id.ptr()) } }
    }
    fn getMenuItem(&self, str_id: &str) -> WxMenuItem {
        let str_id = wxT(str_id);
        unsafe { WxMenuItem { ptr: wxXmlResource_GetMenuItem(self.ptr(), str_id.ptr()) } }
    }
    fn getNotebook(&self, str_id: &str) -> WxNotebook {
        let str_id = wxT(str_id);
        unsafe { WxNotebook { ptr: wxXmlResource_GetNotebook(self.ptr(), str_id.ptr()) } }
    }
    fn getPanel(&self, str_id: &str) -> WxPanel {
        let str_id = wxT(str_id);
        unsafe { WxPanel { ptr: wxXmlResource_GetPanel(self.ptr(), str_id.ptr()) } }
    }
    fn getRadioButton(&self, str_id: &str) -> WxRadioButton {
        let str_id = wxT(str_id);
        unsafe { WxRadioButton { ptr: wxXmlResource_GetRadioButton(self.ptr(), str_id.ptr()) } }
    }
    fn getRadioBox(&self, str_id: &str) -> WxRadioBox {
        let str_id = wxT(str_id);
        unsafe { WxRadioBox { ptr: wxXmlResource_GetRadioBox(self.ptr(), str_id.ptr()) } }
    }
    fn getScrollBar(&self, str_id: &str) -> WxScrollBar {
        let str_id = wxT(str_id);
        unsafe { WxScrollBar { ptr: wxXmlResource_GetScrollBar(self.ptr(), str_id.ptr()) } }
    }
    fn getScrolledWindow(&self, str_id: &str) -> WxScrolledWindow {
        let str_id = wxT(str_id);
        unsafe { WxScrolledWindow { ptr: wxXmlResource_GetScrolledWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getSlider(&self, str_id: &str) -> WxSlider {
        let str_id = wxT(str_id);
        unsafe { WxSlider { ptr: wxXmlResource_GetSlider(self.ptr(), str_id.ptr()) } }
    }
    fn getSpinButton(&self, str_id: &str) -> WxSpinButton {
        let str_id = wxT(str_id);
        unsafe { WxSpinButton { ptr: wxXmlResource_GetSpinButton(self.ptr(), str_id.ptr()) } }
    }
    fn getSpinCtrl(&self, str_id: &str) -> WxSpinCtrl {
        let str_id = wxT(str_id);
        unsafe { WxSpinCtrl { ptr: wxXmlResource_GetSpinCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getSplitterWindow(&self, str_id: &str) -> WxSplitterWindow {
        let str_id = wxT(str_id);
        unsafe { WxSplitterWindow { ptr: wxXmlResource_GetSplitterWindow(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBitmap(&self, str_id: &str) -> WxStaticBitmap {
        let str_id = wxT(str_id);
        unsafe { WxStaticBitmap { ptr: wxXmlResource_GetStaticBitmap(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticBox(&self, str_id: &str) -> WxStaticBox {
        let str_id = wxT(str_id);
        unsafe { WxStaticBox { ptr: wxXmlResource_GetStaticBox(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticLine(&self, str_id: &str) -> WxStaticLine {
        let str_id = wxT(str_id);
        unsafe { WxStaticLine { ptr: wxXmlResource_GetStaticLine(self.ptr(), str_id.ptr()) } }
    }
    fn getStaticText(&self, str_id: &str) -> WxStaticText {
        let str_id = wxT(str_id);
        unsafe { WxStaticText { ptr: wxXmlResource_GetStaticText(self.ptr(), str_id.ptr()) } }
    }
    fn getTextCtrl(&self, str_id: &str) -> WxTextCtrl {
        let str_id = wxT(str_id);
        unsafe { WxTextCtrl { ptr: wxXmlResource_GetTextCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn getTreeCtrl(&self, str_id: &str) -> WxTreeCtrl {
        let str_id = wxT(str_id);
        unsafe { WxTreeCtrl { ptr: wxXmlResource_GetTreeCtrl(self.ptr(), str_id.ptr()) } }
    }
    fn unload(&self, filemask: &str) -> c_int {
        let filemask = wxT(filemask);
        unsafe { wxXmlResource_Unload(self.ptr(), filemask.ptr()) }
    }
    fn set(&self, res: &TWxXmlResource) -> WxXmlResource {
        unsafe { WxXmlResource { ptr: wxXmlResource_Set(self.ptr(), res.ptr()) } }
    }
    fn setDomain(&self, domain: &str) {
        let domain = wxT(domain);
        unsafe { wxXmlResource_SetDomain(self.ptr(), domain.ptr()) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxXmlResource_SetFlags(self.ptr(), flags) }
    }
    fn getStyledTextCtrl(&self, str_id: &str) -> WxStyledTextCtrl {
        let str_id = wxT(str_id);
        unsafe { WxStyledTextCtrl { ptr: wxXmlResource_GetStyledTextCtrl(self.ptr(), str_id.ptr()) } }
    }
}

