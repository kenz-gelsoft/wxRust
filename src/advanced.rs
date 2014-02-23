use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct WxrGridTable { ptr: *mut c_void }
impl TWxrGridTable for WxrGridTable {}
impl TWxGridTableBase for WxrGridTable {}
impl TWxObject for WxrGridTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxrGridTable {
    pub fn from(ptr: *mut c_void) -> WxrGridTable { WxrGridTable { ptr: ptr } }
    pub fn null() -> WxrGridTable { WxrGridTable::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _EifGetNumberRows: *mut c_void, _EifGetNumberCols: *mut c_void, _EifGetValue: *mut c_void, _EifSetValue: *mut c_void, _EifIsEmptyCell: *mut c_void, _EifClear: *mut c_void, _EifInsertRows: *mut c_void, _EifAppendRows: *mut c_void, _EifDeleteRows: *mut c_void, _EifInsertCols: *mut c_void, _EifAppendCols: *mut c_void, _EifDeleteCols: *mut c_void, _EifSetRowLabelValue: *mut c_void, _EifSetColLabelValue: *mut c_void, _EifGetRowLabelValue: *mut c_void, _EifGetColLabelValue: *mut c_void) -> WxrGridTable {
        unsafe { WxrGridTable { ptr: ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue) } }
    }
}

pub trait TWxrGridTable : TWxGridTableBase {
    fn getView(&self) -> WxView {
        unsafe { WxView { ptr: ELJGridTable_GetView(self.ptr()) } }
    }
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *mut c_void {
        unsafe { ELJGridTable_SendTableMessage(self.ptr(), id, val1, val2) }
    }
}

pub struct WxCalculateLayoutEvent { ptr: *mut c_void }
impl TWxCalculateLayoutEvent for WxCalculateLayoutEvent {}
impl TWxEvent for WxCalculateLayoutEvent {}
impl TWxObject for WxCalculateLayoutEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCalculateLayoutEvent {
    pub fn from(ptr: *mut c_void) -> WxCalculateLayoutEvent { WxCalculateLayoutEvent { ptr: ptr } }
    pub fn null() -> WxCalculateLayoutEvent { WxCalculateLayoutEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> WxCalculateLayoutEvent {
        unsafe { WxCalculateLayoutEvent { ptr: wxCalculateLayoutEvent_Create(id) } }
    }
}

pub trait TWxCalculateLayoutEvent : TWxEvent {
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.ptr()) }
    }
    fn getRect(&self) -> WxRect {
        unsafe { WxRect { ptr: wxCalculateLayoutEvent_GetRect(self.ptr()) } }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self.ptr(), flags) }
    }
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxCalculateLayoutEvent_SetRect(self.ptr(), x, y, w, h) }
    }
}

pub struct WxCalendarCtrl { ptr: *mut c_void }
impl TWxCalendarCtrl for WxCalendarCtrl {}
impl TWxControl for WxCalendarCtrl {}
impl TWxWindow for WxCalendarCtrl {}
impl TWxEvtHandler for WxCalendarCtrl {}
impl TWxObject for WxCalendarCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCalendarCtrl {
    pub fn from(ptr: *mut c_void) -> WxCalendarCtrl { WxCalendarCtrl { ptr: ptr } }
    pub fn null() -> WxCalendarCtrl { WxCalendarCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow, U: TWxDateTime>(_prt: &T, _id: c_int, _dat: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxCalendarCtrl {
        unsafe { WxCalendarCtrl { ptr: wxCalendarCtrl_Create(_prt.ptr(), _id, _dat.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxCalendarCtrl : TWxControl {
    fn enableHolidayDisplay(&self, display: c_int) {
        unsafe { wxCalendarCtrl_EnableHolidayDisplay(self.ptr(), display) }
    }
    fn enableMonthChange(&self, enable: c_int) {
        unsafe { wxCalendarCtrl_EnableMonthChange(self.ptr(), enable) }
    }
    fn getAttr(&self, day: c_int) -> *mut c_void {
        unsafe { wxCalendarCtrl_GetAttr(self.ptr(), day) }
    }
    fn getDate(&self, date: *mut c_void) {
        unsafe { wxCalendarCtrl_GetDate(self.ptr(), date) }
    }
    fn getHeaderColourBg<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHeaderColourFg<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.ptr(), _ref.ptr()) }
    }
    fn getHighlightColourBg<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHighlightColourFg<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.ptr(), _ref.ptr()) }
    }
    fn getHolidayColourBg<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHolidayColourFg<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourFg(self.ptr(), _ref.ptr()) }
    }
    fn hitTest(&self, x: c_int, y: c_int, date: *mut c_void, wd: *mut c_void) -> c_int {
        unsafe { wxCalendarCtrl_HitTest(self.ptr(), x, y, date, wd) }
    }
    fn resetAttr(&self, day: c_int) {
        unsafe { wxCalendarCtrl_ResetAttr(self.ptr(), day) }
    }
    fn setAttr(&self, day: c_int, attr: *mut c_void) {
        unsafe { wxCalendarCtrl_SetAttr(self.ptr(), day, attr) }
    }
    fn setDate(&self, date: *mut c_void) {
        unsafe { wxCalendarCtrl_SetDate(self.ptr(), date) }
    }
    fn setHeaderColours(&self, colFg: *mut c_void, colBg: *mut c_void) {
        unsafe { wxCalendarCtrl_SetHeaderColours(self.ptr(), colFg, colBg) }
    }
    fn setHighlightColours(&self, colFg: *mut c_void, colBg: *mut c_void) {
        unsafe { wxCalendarCtrl_SetHighlightColours(self.ptr(), colFg, colBg) }
    }
    fn setHoliday(&self, day: c_int) {
        unsafe { wxCalendarCtrl_SetHoliday(self.ptr(), day) }
    }
    fn setHolidayColours(&self, colFg: *mut c_void, colBg: *mut c_void) {
        unsafe { wxCalendarCtrl_SetHolidayColours(self.ptr(), colFg, colBg) }
    }
}

pub struct WxCalendarDateAttr { ptr: *mut c_void }
impl TWxCalendarDateAttr for WxCalendarDateAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCalendarDateAttr {
    pub fn from(ptr: *mut c_void) -> WxCalendarDateAttr { WxCalendarDateAttr { ptr: ptr } }
    pub fn null() -> WxCalendarDateAttr { WxCalendarDateAttr::from(0 as *mut c_void) }
    
    pub fn new(_ctxt: *mut c_void, _cbck: *mut c_void, _cbrd: *mut c_void, _fnt: *mut c_void, _brd: c_int) -> WxCalendarDateAttr {
        unsafe { WxCalendarDateAttr { ptr: wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd) } }
    }
    pub fn newDefault() -> WxCalendarDateAttr {
        unsafe { WxCalendarDateAttr { ptr: wxCalendarDateAttr_CreateDefault() } }
    }
}

pub trait TWxCalendarDateAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.ptr()) }
    }
    fn getBackgroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.ptr()) }
    }
    fn getBorderColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.ptr(), _ref.ptr()) }
    }
    fn getFont<T: TWxFont>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getTextColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasBackgroundColour(self.ptr()) }
    }
    fn hasBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasBorder(self.ptr()) }
    }
    fn hasBorderColour(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasBorderColour(self.ptr()) }
    }
    fn hasFont(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasFont(self.ptr()) }
    }
    fn hasTextColour(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasTextColour(self.ptr()) }
    }
    fn isHoliday(&self) -> c_int {
        unsafe { wxCalendarDateAttr_IsHoliday(self.ptr()) }
    }
    fn setBackgroundColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.ptr(), col.ptr()) }
    }
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.ptr(), border) }
    }
    fn setBorderColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.ptr(), col.ptr()) }
    }
    fn setFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxCalendarDateAttr_SetFont(self.ptr(), font.ptr()) }
    }
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.ptr(), holiday) }
    }
    fn setTextColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.ptr(), col.ptr()) }
    }
}

pub struct WxCalendarEvent { ptr: *mut c_void }
impl TWxCalendarEvent for WxCalendarEvent {}
impl TWxCommandEvent for WxCalendarEvent {}
impl TWxEvent for WxCalendarEvent {}
impl TWxObject for WxCalendarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxCalendarEvent {
    pub fn from(ptr: *mut c_void) -> WxCalendarEvent { WxCalendarEvent { ptr: ptr } }
    pub fn null() -> WxCalendarEvent { WxCalendarEvent::from(0 as *mut c_void) }
    
}

pub trait TWxCalendarEvent : TWxCommandEvent {
    fn getDate(&self, _dte: *mut c_void) {
        unsafe { wxCalendarEvent_GetDate(self.ptr(), _dte) }
    }
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.ptr()) }
    }
}

pub struct WxEditableListBox { ptr: *mut c_void }
impl TWxEditableListBox for WxEditableListBox {}
impl TWxPanel for WxEditableListBox {}
impl TWxWindow for WxEditableListBox {}
impl TWxEvtHandler for WxEditableListBox {}
impl TWxObject for WxEditableListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxEditableListBox {
    pub fn from(ptr: *mut c_void) -> WxEditableListBox { WxEditableListBox { ptr: ptr } }
    pub fn null() -> WxEditableListBox { WxEditableListBox::from(0 as *mut c_void) }
    
}

pub trait TWxEditableListBox : TWxPanel {
}

pub struct WxGrid { ptr: *mut c_void }
impl TWxGrid for WxGrid {}
impl TWxScrolledWindow for WxGrid {}
impl TWxPanel for WxGrid {}
impl TWxWindow for WxGrid {}
impl TWxEvtHandler for WxGrid {}
impl TWxObject for WxGrid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGrid {
    pub fn from(ptr: *mut c_void) -> WxGrid { WxGrid { ptr: ptr } }
    pub fn null() -> WxGrid { WxGrid::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> WxGrid {
        unsafe { WxGrid { ptr: wxGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TWxGrid : TWxScrolledWindow {
    fn appendCols(&self, numCols: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_AppendCols(self.ptr(), numCols, updateLabels) }
    }
    fn appendRows(&self, numRows: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_AppendRows(self.ptr(), numRows, updateLabels) }
    }
    fn autoSize(&self) {
        unsafe { wxGrid_AutoSize(self.ptr()) }
    }
    fn autoSizeColumn(&self, col: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumn(self.ptr(), col, setAsMin) }
    }
    fn autoSizeColumns(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumns(self.ptr(), setAsMin) }
    }
    fn autoSizeRow(&self, row: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRow(self.ptr(), row, setAsMin) }
    }
    fn autoSizeRows(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRows(self.ptr(), setAsMin) }
    }
    fn beginBatch(&self) {
        unsafe { wxGrid_BeginBatch(self.ptr()) }
    }
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> WxRect {
        unsafe { WxRect { ptr: wxGrid_BlockToDeviceRect(self.ptr(), top, left, bottom, right) } }
    }
    fn canDragColSize(&self) -> c_int {
        unsafe { wxGrid_CanDragColSize(self.ptr()) }
    }
    fn canDragGridSize(&self) -> c_int {
        unsafe { wxGrid_CanDragGridSize(self.ptr()) }
    }
    fn canDragRowSize(&self) -> c_int {
        unsafe { wxGrid_CanDragRowSize(self.ptr()) }
    }
    fn canEnableCellControl(&self) -> c_int {
        unsafe { wxGrid_CanEnableCellControl(self.ptr()) }
    }
    fn cellToRect(&self, row: c_int, col: c_int) -> WxRect {
        unsafe { WxRect { ptr: wxGrid_CellToRect(self.ptr(), row, col) } }
    }
    fn clearGrid(&self) {
        unsafe { wxGrid_ClearGrid(self.ptr()) }
    }
    fn clearSelection(&self) {
        unsafe { wxGrid_ClearSelection(self.ptr()) }
    }
    fn newGrid(&self, rows: c_int, cols: c_int, selmode: c_int) {
        unsafe { wxGrid_CreateGrid(self.ptr(), rows, cols, selmode) }
    }
    fn deleteCols(&self, pos: c_int, numCols: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_DeleteCols(self.ptr(), pos, numCols, updateLabels) }
    }
    fn deleteRows(&self, pos: c_int, numRows: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_DeleteRows(self.ptr(), pos, numRows, updateLabels) }
    }
    fn disableCellEditControl(&self) {
        unsafe { wxGrid_DisableCellEditControl(self.ptr()) }
    }
    fn disableDragColSize(&self) {
        unsafe { wxGrid_DisableDragColSize(self.ptr()) }
    }
    fn disableDragGridSize(&self) {
        unsafe { wxGrid_DisableDragGridSize(self.ptr()) }
    }
    fn disableDragRowSize(&self) {
        unsafe { wxGrid_DisableDragRowSize(self.ptr()) }
    }
    fn drawAllGridLines<T: TWxDC, U: TWxRegion>(&self, dc: &T, reg: &U) {
        unsafe { wxGrid_DrawAllGridLines(self.ptr(), dc.ptr(), reg.ptr()) }
    }
    fn drawCell<T: TWxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.ptr(), dc.ptr(), _row, _col) }
    }
    fn drawCellBorder<T: TWxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.ptr(), dc.ptr(), _row, _col) }
    }
    fn drawCellHighlight<T: TWxDC, U: TWxGridCellAttr>(&self, dc: &T, attr: &U) {
        unsafe { wxGrid_DrawCellHighlight(self.ptr(), dc.ptr(), attr.ptr()) }
    }
    fn drawColLabel<T: TWxDC>(&self, dc: &T, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.ptr(), dc.ptr(), col) }
    }
    fn drawColLabels<T: TWxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawColLabels(self.ptr(), dc.ptr()) }
    }
    fn drawGridSpace<T: TWxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawGridSpace(self.ptr(), dc.ptr()) }
    }
    fn drawRowLabel<T: TWxDC>(&self, dc: &T, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.ptr(), dc.ptr(), row) }
    }
    fn drawRowLabels<T: TWxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawRowLabels(self.ptr(), dc.ptr()) }
    }
    fn drawTextRectangle<T: TWxDC>(&self, dc: &T, txt: &str, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
        let txt = wxT(txt);
        unsafe { wxGrid_DrawTextRectangle(self.ptr(), dc.ptr(), txt.ptr(), x, y, w, h, horizontalAlignment, verticalAlignment) }
    }
    fn enableCellEditControl(&self, enable: c_int) {
        unsafe { wxGrid_EnableCellEditControl(self.ptr(), enable) }
    }
    fn enableDragColSize(&self, enable: c_int) {
        unsafe { wxGrid_EnableDragColSize(self.ptr(), enable) }
    }
    fn enableDragGridSize(&self, enable: c_int) {
        unsafe { wxGrid_EnableDragGridSize(self.ptr(), enable) }
    }
    fn enableDragRowSize(&self, enable: c_int) {
        unsafe { wxGrid_EnableDragRowSize(self.ptr(), enable) }
    }
    fn enableEditing(&self, edit: c_int) {
        unsafe { wxGrid_EnableEditing(self.ptr(), edit) }
    }
    fn enableGridLines(&self, enable: c_int) {
        unsafe { wxGrid_EnableGridLines(self.ptr(), enable) }
    }
    fn endBatch(&self) {
        unsafe { wxGrid_EndBatch(self.ptr()) }
    }
    fn getBatchCount(&self) -> c_int {
        unsafe { wxGrid_GetBatchCount(self.ptr()) }
    }
    fn getCellAlignment(&self, row: c_int, col: c_int, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetCellAlignment(self.ptr(), row, col, horiz, vert) }
    }
    fn getCellBackgroundColour<T: TWxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellBackgroundColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn getCellEditor(&self, row: c_int, col: c_int) -> WxGridCellEditor {
        unsafe { WxGridCellEditor { ptr: wxGrid_GetCellEditor(self.ptr(), row, col) } }
    }
    fn getCellFont<T: TWxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_GetCellFont(self.ptr(), row, col, font.ptr()) }
    }
    fn getCellHighlightColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetCellHighlightColour(self.ptr(), _ref.ptr()) }
    }
    fn getCellRenderer(&self, row: c_int, col: c_int) -> WxGridCellRenderer {
        unsafe { WxGridCellRenderer { ptr: wxGrid_GetCellRenderer(self.ptr(), row, col) } }
    }
    fn getCellTextColour<T: TWxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellTextColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn getCellValue(&self, row: c_int, col: c_int) -> ~str {
        unsafe { WxString { ptr: wxGrid_GetCellValue(self.ptr(), row, col) }.to_str() }
    }
    fn getColLabelAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetColLabelAlignment(self.ptr(), horiz, vert) }
    }
    fn getColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetColLabelSize(self.ptr()) }
    }
    fn getColLabelValue(&self, col: c_int) -> ~str {
        unsafe { WxString { ptr: wxGrid_GetColLabelValue(self.ptr(), col) }.to_str() }
    }
    fn getColSize(&self, col: c_int) -> c_int {
        unsafe { wxGrid_GetColSize(self.ptr(), col) }
    }
    fn getDefaultCellAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetDefaultCellAlignment(self.ptr(), horiz, vert) }
    }
    fn getDefaultCellBackgroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultCellFont<T: TWxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellFont(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultCellTextColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColLabelSize(self.ptr()) }
    }
    fn getDefaultColSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColSize(self.ptr()) }
    }
    fn getDefaultEditor(&self) -> WxGridCellEditor {
        unsafe { WxGridCellEditor { ptr: wxGrid_GetDefaultEditor(self.ptr()) } }
    }
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> WxGridCellEditor {
        unsafe { WxGridCellEditor { ptr: wxGrid_GetDefaultEditorForCell(self.ptr(), row, col) } }
    }
    fn getDefaultEditorForType(&self, typeName: &str) -> WxGridCellEditor {
        let typeName = wxT(typeName);
        unsafe { WxGridCellEditor { ptr: wxGrid_GetDefaultEditorForType(self.ptr(), typeName.ptr()) } }
    }
    fn getDefaultRenderer(&self) -> WxGridCellRenderer {
        unsafe { WxGridCellRenderer { ptr: wxGrid_GetDefaultRenderer(self.ptr()) } }
    }
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> WxGridCellRenderer {
        unsafe { WxGridCellRenderer { ptr: wxGrid_GetDefaultRendererForCell(self.ptr(), row, col) } }
    }
    fn getDefaultRendererForType(&self, typeName: &str) -> WxGridCellRenderer {
        let typeName = wxT(typeName);
        unsafe { WxGridCellRenderer { ptr: wxGrid_GetDefaultRendererForType(self.ptr(), typeName.ptr()) } }
    }
    fn getDefaultRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowLabelSize(self.ptr()) }
    }
    fn getDefaultRowSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowSize(self.ptr()) }
    }
    fn getGridCursorCol(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorCol(self.ptr()) }
    }
    fn getGridCursorRow(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorRow(self.ptr()) }
    }
    fn getGridLineColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetGridLineColour(self.ptr(), _ref.ptr()) }
    }
    fn getLabelBackgroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getLabelFont<T: TWxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelFont(self.ptr(), _ref.ptr()) }
    }
    fn getLabelTextColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getNumberCols(&self) -> c_int {
        unsafe { wxGrid_GetNumberCols(self.ptr()) }
    }
    fn getNumberRows(&self) -> c_int {
        unsafe { wxGrid_GetNumberRows(self.ptr()) }
    }
    fn getRowLabelAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetRowLabelAlignment(self.ptr(), horiz, vert) }
    }
    fn getRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetRowLabelSize(self.ptr()) }
    }
    fn getRowLabelValue(&self, row: c_int) -> ~str {
        unsafe { WxString { ptr: wxGrid_GetRowLabelValue(self.ptr(), row) }.to_str() }
    }
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.ptr(), row) }
    }
    fn getSelectionBackground<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionBackground(self.ptr(), _ref.ptr()) }
    }
    fn getSelectionForeground<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionForeground(self.ptr(), _ref.ptr()) }
    }
    fn getTable(&self) -> WxGridTableBase {
        unsafe { WxGridTableBase { ptr: wxGrid_GetTable(self.ptr()) } }
    }
    fn getTextBoxSize<T: TWxDC>(&self, dc: &T, count: c_int, lines: *mut *mut c_char, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxGrid_GetTextBoxSize(self.ptr(), dc.ptr(), count, lines, _w, _h) }
    }
    fn gridLinesEnabled(&self) -> c_int {
        unsafe { wxGrid_GridLinesEnabled(self.ptr()) }
    }
    fn hideCellEditControl(&self) {
        unsafe { wxGrid_HideCellEditControl(self.ptr()) }
    }
    fn insertCols(&self, pos: c_int, numCols: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_InsertCols(self.ptr(), pos, numCols, updateLabels) }
    }
    fn insertRows(&self, pos: c_int, numRows: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_InsertRows(self.ptr(), pos, numRows, updateLabels) }
    }
    fn isCellEditControlEnabled(&self) -> c_int {
        unsafe { wxGrid_IsCellEditControlEnabled(self.ptr()) }
    }
    fn isCellEditControlShown(&self) -> c_int {
        unsafe { wxGrid_IsCellEditControlShown(self.ptr()) }
    }
    fn isCurrentCellReadOnly(&self) -> c_int {
        unsafe { wxGrid_IsCurrentCellReadOnly(self.ptr()) }
    }
    fn isEditable(&self) -> c_int {
        unsafe { wxGrid_IsEditable(self.ptr()) }
    }
    fn isInSelection(&self, row: c_int, col: c_int) -> c_int {
        unsafe { wxGrid_IsInSelection(self.ptr(), row, col) }
    }
    fn isReadOnly(&self, row: c_int, col: c_int) -> c_int {
        unsafe { wxGrid_IsReadOnly(self.ptr(), row, col) }
    }
    fn isSelection(&self) -> c_int {
        unsafe { wxGrid_IsSelection(self.ptr()) }
    }
    fn isVisible(&self, row: c_int, col: c_int, wholeCellVisible: c_int) -> c_int {
        unsafe { wxGrid_IsVisible(self.ptr(), row, col, wholeCellVisible) }
    }
    fn makeCellVisible(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_MakeCellVisible(self.ptr(), row, col) }
    }
    fn moveCursorDown(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorDown(self.ptr(), expandSelection) }
    }
    fn moveCursorDownBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorDownBlock(self.ptr(), expandSelection) }
    }
    fn moveCursorLeft(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorLeft(self.ptr(), expandSelection) }
    }
    fn moveCursorLeftBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorLeftBlock(self.ptr(), expandSelection) }
    }
    fn moveCursorRight(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorRight(self.ptr(), expandSelection) }
    }
    fn moveCursorRightBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorRightBlock(self.ptr(), expandSelection) }
    }
    fn moveCursorUp(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorUp(self.ptr(), expandSelection) }
    }
    fn moveCursorUpBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorUpBlock(self.ptr(), expandSelection) }
    }
    fn movePageDown(&self) -> c_int {
        unsafe { wxGrid_MovePageDown(self.ptr()) }
    }
    fn movePageUp(&self) -> c_int {
        unsafe { wxGrid_MovePageUp(self.ptr()) }
    }
    fn processTableMessage<T: TWxEvent>(&self, evt: &T) -> c_int {
        unsafe { wxGrid_ProcessTableMessage(self.ptr(), evt.ptr()) }
    }
    fn registerDataType<T: TWxGridCellRenderer, U: TWxGridCellEditor>(&self, typeName: &str, renderer: &T, editor: &U) {
        let typeName = wxT(typeName);
        unsafe { wxGrid_RegisterDataType(self.ptr(), typeName.ptr(), renderer.ptr(), editor.ptr()) }
    }
    fn saveEditControlValue(&self) {
        unsafe { wxGrid_SaveEditControlValue(self.ptr()) }
    }
    fn selectAll(&self) {
        unsafe { wxGrid_SelectAll(self.ptr()) }
    }
    fn selectBlock(&self, topRow: c_int, leftCol: c_int, bottomRow: c_int, rightCol: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectBlock(self.ptr(), topRow, leftCol, bottomRow, rightCol, addToSelected) }
    }
    fn selectCol(&self, col: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectCol(self.ptr(), col, addToSelected) }
    }
    fn selectRow(&self, row: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectRow(self.ptr(), row, addToSelected) }
    }
    fn setCellAlignment(&self, row: c_int, col: c_int, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetCellAlignment(self.ptr(), row, col, horiz, vert) }
    }
    fn setCellBackgroundColour<T: TWxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellBackgroundColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn setCellEditor<T: TWxGridCellEditor>(&self, row: c_int, col: c_int, editor: &T) {
        unsafe { wxGrid_SetCellEditor(self.ptr(), row, col, editor.ptr()) }
    }
    fn setCellFont<T: TWxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_SetCellFont(self.ptr(), row, col, font.ptr()) }
    }
    fn setCellHighlightColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxGrid_SetCellHighlightColour(self.ptr(), col.ptr()) }
    }
    fn setCellRenderer<T: TWxGridCellRenderer>(&self, row: c_int, col: c_int, renderer: &T) {
        unsafe { wxGrid_SetCellRenderer(self.ptr(), row, col, renderer.ptr()) }
    }
    fn setCellTextColour<T: TWxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellTextColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn setCellValue(&self, row: c_int, col: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxGrid_SetCellValue(self.ptr(), row, col, s.ptr()) }
    }
    fn setColAttr<T: TWxGridCellAttr>(&self, col: c_int, attr: &T) {
        unsafe { wxGrid_SetColAttr(self.ptr(), col, attr.ptr()) }
    }
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.ptr(), col) }
    }
    fn setColFormatCustom(&self, col: c_int, typeName: &str) {
        let typeName = wxT(typeName);
        unsafe { wxGrid_SetColFormatCustom(self.ptr(), col, typeName.ptr()) }
    }
    fn setColFormatFloat(&self, col: c_int, width: c_int, precision: c_int) {
        unsafe { wxGrid_SetColFormatFloat(self.ptr(), col, width, precision) }
    }
    fn setColFormatNumber(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatNumber(self.ptr(), col) }
    }
    fn setColLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetColLabelAlignment(self.ptr(), horiz, vert) }
    }
    fn setColLabelSize(&self, height: c_int) {
        unsafe { wxGrid_SetColLabelSize(self.ptr(), height) }
    }
    fn setColLabelValue(&self, col: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxGrid_SetColLabelValue(self.ptr(), col, label.ptr()) }
    }
    fn setColMinimalWidth(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColMinimalWidth(self.ptr(), col, width) }
    }
    fn setColSize(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColSize(self.ptr(), col, width) }
    }
    fn setDefaultCellAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetDefaultCellAlignment(self.ptr(), horiz, vert) }
    }
    fn setDefaultCellBackgroundColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setDefaultCellFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxGrid_SetDefaultCellFont(self.ptr(), font.ptr()) }
    }
    fn setDefaultCellTextColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.ptr(), colour.ptr()) }
    }
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.ptr(), width, resizeExistingCols) }
    }
    fn setDefaultEditor<T: TWxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGrid_SetDefaultEditor(self.ptr(), editor.ptr()) }
    }
    fn setDefaultRenderer<T: TWxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGrid_SetDefaultRenderer(self.ptr(), renderer.ptr()) }
    }
    fn setDefaultRowSize(&self, height: c_int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self.ptr(), height, resizeExistingRows) }
    }
    fn setGridCursor(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_SetGridCursor(self.ptr(), row, col) }
    }
    fn setGridLineColour<T: TWxColour>(&self, col: &T) {
        unsafe { wxGrid_SetGridLineColour(self.ptr(), col.ptr()) }
    }
    fn setLabelBackgroundColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setLabelFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxGrid_SetLabelFont(self.ptr(), font.ptr()) }
    }
    fn setLabelTextColour<T: TWxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelTextColour(self.ptr(), colour.ptr()) }
    }
    fn setMargins(&self, extraWidth: c_int, extraHeight: c_int) {
        unsafe { wxGrid_SetMargins(self.ptr(), extraWidth, extraHeight) }
    }
    fn setReadOnly(&self, row: c_int, col: c_int, isReadOnly: c_int) {
        unsafe { wxGrid_SetReadOnly(self.ptr(), row, col, isReadOnly) }
    }
    fn setRowAttr<T: TWxGridCellAttr>(&self, row: c_int, attr: &T) {
        unsafe { wxGrid_SetRowAttr(self.ptr(), row, attr.ptr()) }
    }
    fn setRowLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetRowLabelAlignment(self.ptr(), horiz, vert) }
    }
    fn setRowLabelSize(&self, width: c_int) {
        unsafe { wxGrid_SetRowLabelSize(self.ptr(), width) }
    }
    fn setRowLabelValue(&self, row: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxGrid_SetRowLabelValue(self.ptr(), row, label.ptr()) }
    }
    fn setRowMinimalHeight(&self, row: c_int, width: c_int) {
        unsafe { wxGrid_SetRowMinimalHeight(self.ptr(), row, width) }
    }
    fn setRowSize(&self, row: c_int, height: c_int) {
        unsafe { wxGrid_SetRowSize(self.ptr(), row, height) }
    }
    fn setSelectionBackground<T: TWxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionBackground(self.ptr(), c.ptr()) }
    }
    fn setSelectionForeground<T: TWxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionForeground(self.ptr(), c.ptr()) }
    }
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.ptr(), selmode) }
    }
    fn setTable<T: TWxGridTableBase>(&self, table: &T, takeOwnership: c_int, selmode: c_int) -> c_int {
        unsafe { wxGrid_SetTable(self.ptr(), table.ptr(), takeOwnership, selmode) }
    }
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.ptr()) }
    }
    fn stringToLines(&self, value: &str, lines: *mut c_void) -> c_int {
        let value = wxT(value);
        unsafe { wxGrid_StringToLines(self.ptr(), value.ptr(), lines) }
    }
    fn xToCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToCol(self.ptr(), x) }
    }
    fn xToEdgeOfCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToEdgeOfCol(self.ptr(), x) }
    }
    fn xYToCell(&self, x: c_int, y: c_int, row: *mut c_int, col: *mut c_int) {
        unsafe { wxGrid_XYToCell(self.ptr(), x, y, row, col) }
    }
    fn yToEdgeOfRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToEdgeOfRow(self.ptr(), y) }
    }
    fn yToRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToRow(self.ptr(), y) }
    }
    fn getSelectedCells<T: TWxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectedCells(self.ptr(), _arr.ptr()) }
    }
    fn getSelectionBlockTopLeft<T: TWxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.ptr(), _arr.ptr()) }
    }
    fn getSelectionBlockBottomRight<T: TWxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockBottomRight(self.ptr(), _arr.ptr()) }
    }
    fn getSelectedRows(&self, _arr: *mut c_void) -> c_int {
        unsafe { wxGrid_GetSelectedRows(self.ptr(), _arr) }
    }
    fn getSelectedCols(&self, _arr: *mut c_void) -> c_int {
        unsafe { wxGrid_GetSelectedCols(self.ptr(), _arr) }
    }
    fn getCellSize(&self, row: c_int, col: c_int, srow: *mut c_int, scol: *mut c_int) {
        unsafe { wxGrid_GetCellSize(self.ptr(), row, col, srow, scol) }
    }
    fn setCellSize(&self, row: c_int, col: c_int, srow: c_int, scol: c_int) {
        unsafe { wxGrid_SetCellSize(self.ptr(), row, col, srow, scol) }
    }
}

pub struct WxGridCellAttr { ptr: *mut c_void }
impl TWxGridCellAttr for WxGridCellAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellAttr {
    pub fn from(ptr: *mut c_void) -> WxGridCellAttr { WxGridCellAttr { ptr: ptr } }
    pub fn null() -> WxGridCellAttr { WxGridCellAttr::from(0 as *mut c_void) }
    
    pub fn ctor() -> WxGridCellAttr {
        unsafe { WxGridCellAttr { ptr: wxGridCellAttr_Ctor() } }
    }
}

pub trait TWxGridCellAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.ptr()) }
    }
    fn getAlignment(&self, hAlign: *mut c_int, vAlign: *mut c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.ptr(), hAlign, vAlign) }
    }
    fn getBackgroundColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getEditor<T: TWxGrid>(&self, grid: &T, row: c_int, col: c_int) -> WxGridCellEditor {
        unsafe { WxGridCellEditor { ptr: wxGridCellAttr_GetEditor(self.ptr(), grid.ptr(), row, col) } }
    }
    fn getFont<T: TWxFont>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getRenderer<T: TWxGrid>(&self, grid: &T, row: c_int, col: c_int) -> WxGridCellRenderer {
        unsafe { WxGridCellRenderer { ptr: wxGridCellAttr_GetRenderer(self.ptr(), grid.ptr(), row, col) } }
    }
    fn getTextColour<T: TWxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetTextColour(self.ptr(), _ref.ptr()) }
    }
    fn hasAlignment(&self) -> c_int {
        unsafe { wxGridCellAttr_HasAlignment(self.ptr()) }
    }
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxGridCellAttr_HasBackgroundColour(self.ptr()) }
    }
    fn hasEditor(&self) -> c_int {
        unsafe { wxGridCellAttr_HasEditor(self.ptr()) }
    }
    fn hasFont(&self) -> c_int {
        unsafe { wxGridCellAttr_HasFont(self.ptr()) }
    }
    fn hasRenderer(&self) -> c_int {
        unsafe { wxGridCellAttr_HasRenderer(self.ptr()) }
    }
    fn hasTextColour(&self) -> c_int {
        unsafe { wxGridCellAttr_HasTextColour(self.ptr()) }
    }
    fn incRef(&self) {
        unsafe { wxGridCellAttr_IncRef(self.ptr()) }
    }
    fn isReadOnly(&self) -> c_int {
        unsafe { wxGridCellAttr_IsReadOnly(self.ptr()) }
    }
    fn setAlignment(&self, hAlign: c_int, vAlign: c_int) {
        unsafe { wxGridCellAttr_SetAlignment(self.ptr(), hAlign, vAlign) }
    }
    fn setBackgroundColour<T: TWxColour>(&self, colBack: &T) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.ptr(), colBack.ptr()) }
    }
    fn setDefAttr<T: TWxGridCellAttr>(&self, defAttr: &T) {
        unsafe { wxGridCellAttr_SetDefAttr(self.ptr(), defAttr.ptr()) }
    }
    fn setEditor<T: TWxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGridCellAttr_SetEditor(self.ptr(), editor.ptr()) }
    }
    fn setFont<T: TWxFont>(&self, font: &T) {
        unsafe { wxGridCellAttr_SetFont(self.ptr(), font.ptr()) }
    }
    fn setReadOnly(&self, isReadOnly: c_int) {
        unsafe { wxGridCellAttr_SetReadOnly(self.ptr(), isReadOnly) }
    }
    fn setRenderer<T: TWxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGridCellAttr_SetRenderer(self.ptr(), renderer.ptr()) }
    }
    fn setTextColour<T: TWxColour>(&self, colText: &T) {
        unsafe { wxGridCellAttr_SetTextColour(self.ptr(), colText.ptr()) }
    }
}

pub struct WxGridCellBoolEditor { ptr: *mut c_void }
impl TWxGridCellBoolEditor for WxGridCellBoolEditor {}
impl TWxGridCellEditor for WxGridCellBoolEditor {}
impl TWxGridCellWorker for WxGridCellBoolEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellBoolEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellBoolEditor { WxGridCellBoolEditor { ptr: ptr } }
    pub fn null() -> WxGridCellBoolEditor { WxGridCellBoolEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> WxGridCellBoolEditor {
        unsafe { WxGridCellBoolEditor { ptr: wxGridCellBoolEditor_Ctor() } }
    }
}

pub trait TWxGridCellBoolEditor : TWxGridCellEditor {
}

pub struct WxGridCellBoolRenderer { ptr: *mut c_void }
impl TWxGridCellBoolRenderer for WxGridCellBoolRenderer {}
impl TWxGridCellRenderer for WxGridCellBoolRenderer {}
impl TWxGridCellWorker for WxGridCellBoolRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellBoolRenderer {
    pub fn from(ptr: *mut c_void) -> WxGridCellBoolRenderer { WxGridCellBoolRenderer { ptr: ptr } }
    pub fn null() -> WxGridCellBoolRenderer { WxGridCellBoolRenderer::from(0 as *mut c_void) }
    
}

pub trait TWxGridCellBoolRenderer : TWxGridCellRenderer {
}

pub struct WxGridCellChoiceEditor { ptr: *mut c_void }
impl TWxGridCellChoiceEditor for WxGridCellChoiceEditor {}
impl TWxGridCellEditor for WxGridCellChoiceEditor {}
impl TWxGridCellWorker for WxGridCellChoiceEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellChoiceEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellChoiceEditor { WxGridCellChoiceEditor { ptr: ptr } }
    pub fn null() -> WxGridCellChoiceEditor { WxGridCellChoiceEditor::from(0 as *mut c_void) }
    
    pub fn ctor(count: c_int, choices: *mut *mut c_char, allowOthers: c_int) -> WxGridCellChoiceEditor {
        unsafe { WxGridCellChoiceEditor { ptr: wxGridCellChoiceEditor_Ctor(count, choices, allowOthers) } }
    }
}

pub trait TWxGridCellChoiceEditor : TWxGridCellEditor {
}

pub struct WxGridCellCoordsArray { ptr: *mut c_void }
impl TWxGridCellCoordsArray for WxGridCellCoordsArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellCoordsArray {
    pub fn from(ptr: *mut c_void) -> WxGridCellCoordsArray { WxGridCellCoordsArray { ptr: ptr } }
    pub fn null() -> WxGridCellCoordsArray { WxGridCellCoordsArray::from(0 as *mut c_void) }
    
    pub fn new() -> WxGridCellCoordsArray {
        unsafe { WxGridCellCoordsArray { ptr: wxGridCellCoordsArray_Create() } }
    }
}

pub trait TWxGridCellCoordsArray {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxGridCellCoordsArray_Delete(self.ptr()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxGridCellCoordsArray_GetCount(self.ptr()) }
    }
    fn item(&self, _idx: c_int, _c: *mut c_int, _r: *mut c_int) {
        unsafe { wxGridCellCoordsArray_Item(self.ptr(), _idx, _c, _r) }
    }
}

pub struct WxGridCellEditor { ptr: *mut c_void }
impl TWxGridCellEditor for WxGridCellEditor {}
impl TWxGridCellWorker for WxGridCellEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellEditor { WxGridCellEditor { ptr: ptr } }
    pub fn null() -> WxGridCellEditor { WxGridCellEditor::from(0 as *mut c_void) }
    
}

pub trait TWxGridCellEditor : TWxGridCellWorker {
    fn beginEdit<T: TWxGrid>(&self, row: c_int, col: c_int, grid: &T) {
        unsafe { wxGridCellEditor_BeginEdit(self.ptr(), row, col, grid.ptr()) }
    }
    fn new<T: TWxWindow, U: TWxEvtHandler>(&self, parent: &T, id: c_int, evtHandler: &U) {
        unsafe { wxGridCellEditor_Create(self.ptr(), parent.ptr(), id, evtHandler.ptr()) }
    }
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.ptr()) }
    }
    fn endEdit<T: TWxGrid>(&self, row: c_int, col: c_int, grid: &T, oldStr: &str, newStr: &str) -> c_int {
        let oldStr = wxT(oldStr);
        let newStr = wxT(newStr);
        unsafe { wxGridCellEditor_EndEdit(self.ptr(), row, col, grid.ptr(), oldStr.ptr(), newStr.ptr()) }
    }
    fn getControl(&self) -> WxControl {
        unsafe { WxControl { ptr: wxGridCellEditor_GetControl(self.ptr()) } }
    }
    fn handleReturn<T: TWxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_HandleReturn(self.ptr(), event.ptr()) }
    }
    fn isAcceptedKey<T: TWxEvent>(&self, event: &T) -> c_int {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.ptr(), event.ptr()) }
    }
    fn isCreated(&self) -> c_int {
        unsafe { wxGridCellEditor_IsCreated(self.ptr()) }
    }
    fn paintBackground<T: TWxDC, U: TWxGridCellAttr>(&self, dc: &T, x: c_int, y: c_int, w: c_int, h: c_int, attr: &U) {
        unsafe { wxGridCellEditor_PaintBackground(self.ptr(), dc.ptr(), x, y, w, h, attr.ptr()) }
    }
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.ptr()) }
    }
    fn setControl<T: TWxControl>(&self, control: &T) {
        unsafe { wxGridCellEditor_SetControl(self.ptr(), control.ptr()) }
    }
    fn setParameters(&self, params: &str) {
        let params = wxT(params);
        unsafe { wxGridCellEditor_SetParameters(self.ptr(), params.ptr()) }
    }
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.ptr(), x, y, w, h) }
    }
    fn show<T: TWxGridCellAttr>(&self, show: c_int, attr: &T) {
        unsafe { wxGridCellEditor_Show(self.ptr(), show, attr.ptr()) }
    }
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.ptr()) }
    }
    fn startingKey<T: TWxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_StartingKey(self.ptr(), event.ptr()) }
    }
}

pub struct WxGridCellFloatEditor { ptr: *mut c_void }
impl TWxGridCellFloatEditor for WxGridCellFloatEditor {}
impl TWxGridCellTextEditor for WxGridCellFloatEditor {}
impl TWxGridCellEditor for WxGridCellFloatEditor {}
impl TWxGridCellWorker for WxGridCellFloatEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellFloatEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellFloatEditor { WxGridCellFloatEditor { ptr: ptr } }
    pub fn null() -> WxGridCellFloatEditor { WxGridCellFloatEditor::from(0 as *mut c_void) }
    
    pub fn ctor(width: c_int, precision: c_int) -> WxGridCellFloatEditor {
        unsafe { WxGridCellFloatEditor { ptr: wxGridCellFloatEditor_Ctor(width, precision) } }
    }
}

pub trait TWxGridCellFloatEditor : TWxGridCellTextEditor {
}

pub struct WxGridCellFloatRenderer { ptr: *mut c_void }
impl TWxGridCellFloatRenderer for WxGridCellFloatRenderer {}
impl TWxGridCellStringRenderer for WxGridCellFloatRenderer {}
impl TWxGridCellRenderer for WxGridCellFloatRenderer {}
impl TWxGridCellWorker for WxGridCellFloatRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellFloatRenderer {
    pub fn from(ptr: *mut c_void) -> WxGridCellFloatRenderer { WxGridCellFloatRenderer { ptr: ptr } }
    pub fn null() -> WxGridCellFloatRenderer { WxGridCellFloatRenderer::from(0 as *mut c_void) }
    
}

pub trait TWxGridCellFloatRenderer : TWxGridCellStringRenderer {
}

pub struct WxGridCellNumberEditor { ptr: *mut c_void }
impl TWxGridCellNumberEditor for WxGridCellNumberEditor {}
impl TWxGridCellTextEditor for WxGridCellNumberEditor {}
impl TWxGridCellEditor for WxGridCellNumberEditor {}
impl TWxGridCellWorker for WxGridCellNumberEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellNumberEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellNumberEditor { WxGridCellNumberEditor { ptr: ptr } }
    pub fn null() -> WxGridCellNumberEditor { WxGridCellNumberEditor::from(0 as *mut c_void) }
    
    pub fn ctor(min: c_int, max: c_int) -> WxGridCellNumberEditor {
        unsafe { WxGridCellNumberEditor { ptr: wxGridCellNumberEditor_Ctor(min, max) } }
    }
}

pub trait TWxGridCellNumberEditor : TWxGridCellTextEditor {
}

pub struct WxGridCellNumberRenderer { ptr: *mut c_void }
impl TWxGridCellNumberRenderer for WxGridCellNumberRenderer {}
impl TWxGridCellStringRenderer for WxGridCellNumberRenderer {}
impl TWxGridCellRenderer for WxGridCellNumberRenderer {}
impl TWxGridCellWorker for WxGridCellNumberRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellNumberRenderer {
    pub fn from(ptr: *mut c_void) -> WxGridCellNumberRenderer { WxGridCellNumberRenderer { ptr: ptr } }
    pub fn null() -> WxGridCellNumberRenderer { WxGridCellNumberRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> WxGridCellNumberRenderer {
        unsafe { WxGridCellNumberRenderer { ptr: wxGridCellNumberRenderer_Ctor() } }
    }
}

pub trait TWxGridCellNumberRenderer : TWxGridCellStringRenderer {
}

pub struct WxGridCellAutoWrapStringRenderer { ptr: *mut c_void }
impl TWxGridCellAutoWrapStringRenderer for WxGridCellAutoWrapStringRenderer {}
impl TWxGridCellStringRenderer for WxGridCellAutoWrapStringRenderer {}
impl TWxGridCellRenderer for WxGridCellAutoWrapStringRenderer {}
impl TWxGridCellWorker for WxGridCellAutoWrapStringRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellAutoWrapStringRenderer {
    pub fn from(ptr: *mut c_void) -> WxGridCellAutoWrapStringRenderer { WxGridCellAutoWrapStringRenderer { ptr: ptr } }
    pub fn null() -> WxGridCellAutoWrapStringRenderer { WxGridCellAutoWrapStringRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> WxGridCellAutoWrapStringRenderer {
        unsafe { WxGridCellAutoWrapStringRenderer { ptr: wxGridCellAutoWrapStringRenderer_Ctor() } }
    }
}

pub trait TWxGridCellAutoWrapStringRenderer : TWxGridCellStringRenderer {
}

pub struct WxGridCellRenderer { ptr: *mut c_void }
impl TWxGridCellRenderer for WxGridCellRenderer {}
impl TWxGridCellWorker for WxGridCellRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellRenderer {
    pub fn from(ptr: *mut c_void) -> WxGridCellRenderer { WxGridCellRenderer { ptr: ptr } }
    pub fn null() -> WxGridCellRenderer { WxGridCellRenderer::from(0 as *mut c_void) }
    
}

pub trait TWxGridCellRenderer : TWxGridCellWorker {
}

pub struct WxGridCellStringRenderer { ptr: *mut c_void }
impl TWxGridCellStringRenderer for WxGridCellStringRenderer {}
impl TWxGridCellRenderer for WxGridCellStringRenderer {}
impl TWxGridCellWorker for WxGridCellStringRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellStringRenderer {
    pub fn from(ptr: *mut c_void) -> WxGridCellStringRenderer { WxGridCellStringRenderer { ptr: ptr } }
    pub fn null() -> WxGridCellStringRenderer { WxGridCellStringRenderer::from(0 as *mut c_void) }
    
}

pub trait TWxGridCellStringRenderer : TWxGridCellRenderer {
}

pub struct WxGridCellTextEditor { ptr: *mut c_void }
impl TWxGridCellTextEditor for WxGridCellTextEditor {}
impl TWxGridCellEditor for WxGridCellTextEditor {}
impl TWxGridCellWorker for WxGridCellTextEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellTextEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellTextEditor { WxGridCellTextEditor { ptr: ptr } }
    pub fn null() -> WxGridCellTextEditor { WxGridCellTextEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> WxGridCellTextEditor {
        unsafe { WxGridCellTextEditor { ptr: wxGridCellTextEditor_Ctor() } }
    }
}

pub trait TWxGridCellTextEditor : TWxGridCellEditor {
}

pub struct WxGridCellWorker { ptr: *mut c_void }
impl TWxGridCellWorker for WxGridCellWorker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellWorker {
    pub fn from(ptr: *mut c_void) -> WxGridCellWorker { WxGridCellWorker { ptr: ptr } }
    pub fn null() -> WxGridCellWorker { WxGridCellWorker::from(0 as *mut c_void) }
    
}

pub trait TWxGridCellWorker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxGridEditorCreatedEvent { ptr: *mut c_void }
impl TWxGridEditorCreatedEvent for WxGridEditorCreatedEvent {}
impl TWxCommandEvent for WxGridEditorCreatedEvent {}
impl TWxEvent for WxGridEditorCreatedEvent {}
impl TWxObject for WxGridEditorCreatedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridEditorCreatedEvent {
    pub fn from(ptr: *mut c_void) -> WxGridEditorCreatedEvent { WxGridEditorCreatedEvent { ptr: ptr } }
    pub fn null() -> WxGridEditorCreatedEvent { WxGridEditorCreatedEvent::from(0 as *mut c_void) }
    
}

pub trait TWxGridEditorCreatedEvent : TWxCommandEvent {
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.ptr()) }
    }
    fn getControl(&self) -> WxControl {
        unsafe { WxControl { ptr: wxGridEditorCreatedEvent_GetControl(self.ptr()) } }
    }
    fn getRow(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self.ptr()) }
    }
    fn setCol(&self, col: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self.ptr(), col) }
    }
    fn setControl<T: TWxControl>(&self, ctrl: &T) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.ptr(), ctrl.ptr()) }
    }
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.ptr(), row) }
    }
}

pub struct WxGridEvent { ptr: *mut c_void }
impl TWxGridEvent for WxGridEvent {}
impl TWxNotifyEvent for WxGridEvent {}
impl TWxCommandEvent for WxGridEvent {}
impl TWxEvent for WxGridEvent {}
impl TWxObject for WxGridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridEvent {
    pub fn from(ptr: *mut c_void) -> WxGridEvent { WxGridEvent { ptr: ptr } }
    pub fn null() -> WxGridEvent { WxGridEvent::from(0 as *mut c_void) }
    
}

pub trait TWxGridEvent : TWxNotifyEvent {
    fn altDown(&self) -> c_int {
        unsafe { wxGridEvent_AltDown(self.ptr()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridEvent_ControlDown(self.ptr()) }
    }
    fn getCol(&self) -> c_int {
        unsafe { wxGridEvent_GetCol(self.ptr()) }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxGridEvent_GetPosition(self.ptr()) } }
    }
    fn getRow(&self) -> c_int {
        unsafe { wxGridEvent_GetRow(self.ptr()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxGridEvent_MetaDown(self.ptr()) }
    }
    fn selecting(&self) -> c_int {
        unsafe { wxGridEvent_Selecting(self.ptr()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxGridEvent_ShiftDown(self.ptr()) }
    }
}

pub struct WxGridRangeSelectEvent { ptr: *mut c_void }
impl TWxGridRangeSelectEvent for WxGridRangeSelectEvent {}
impl TWxNotifyEvent for WxGridRangeSelectEvent {}
impl TWxCommandEvent for WxGridRangeSelectEvent {}
impl TWxEvent for WxGridRangeSelectEvent {}
impl TWxObject for WxGridRangeSelectEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridRangeSelectEvent {
    pub fn from(ptr: *mut c_void) -> WxGridRangeSelectEvent { WxGridRangeSelectEvent { ptr: ptr } }
    pub fn null() -> WxGridRangeSelectEvent { WxGridRangeSelectEvent::from(0 as *mut c_void) }
    
}

pub trait TWxGridRangeSelectEvent : TWxNotifyEvent {
    fn getTopLeftCoords(&self, col: *mut c_void, row: *mut c_void) {
        unsafe { wxGridRangeSelectEvent_GetTopLeftCoords(self.ptr(), col, row) }
    }
    fn getBottomRightCoords(&self, col: *mut c_void, row: *mut c_void) {
        unsafe { wxGridRangeSelectEvent_GetBottomRightCoords(self.ptr(), col, row) }
    }
    fn getTopRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetTopRow(self.ptr()) }
    }
    fn getBottomRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetBottomRow(self.ptr()) }
    }
    fn getLeftCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetLeftCol(self.ptr()) }
    }
    fn getRightCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetRightCol(self.ptr()) }
    }
    fn selecting(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_Selecting(self.ptr()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_ControlDown(self.ptr()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_MetaDown(self.ptr()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_ShiftDown(self.ptr()) }
    }
    fn altDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_AltDown(self.ptr()) }
    }
}

pub struct WxGridSizeEvent { ptr: *mut c_void }
impl TWxGridSizeEvent for WxGridSizeEvent {}
impl TWxNotifyEvent for WxGridSizeEvent {}
impl TWxCommandEvent for WxGridSizeEvent {}
impl TWxEvent for WxGridSizeEvent {}
impl TWxObject for WxGridSizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridSizeEvent {
    pub fn from(ptr: *mut c_void) -> WxGridSizeEvent { WxGridSizeEvent { ptr: ptr } }
    pub fn null() -> WxGridSizeEvent { WxGridSizeEvent::from(0 as *mut c_void) }
    
}

pub trait TWxGridSizeEvent : TWxNotifyEvent {
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.ptr()) }
    }
    fn getPosition(&self) -> WxPoint {
        unsafe { WxPoint { ptr: wxGridSizeEvent_GetPosition(self.ptr()) } }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_ControlDown(self.ptr()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_MetaDown(self.ptr()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_ShiftDown(self.ptr()) }
    }
    fn altDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_AltDown(self.ptr()) }
    }
}

pub struct WxGridTableBase { ptr: *mut c_void }
impl TWxGridTableBase for WxGridTableBase {}
impl TWxObject for WxGridTableBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridTableBase {
    pub fn from(ptr: *mut c_void) -> WxGridTableBase { WxGridTableBase { ptr: ptr } }
    pub fn null() -> WxGridTableBase { WxGridTableBase::from(0 as *mut c_void) }
    
}

pub trait TWxGridTableBase : TWxObject {
}

pub struct WxJoystick { ptr: *mut c_void }
impl TWxJoystick for WxJoystick {}
impl TWxObject for WxJoystick { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxJoystick {
    pub fn from(ptr: *mut c_void) -> WxJoystick { WxJoystick { ptr: ptr } }
    pub fn null() -> WxJoystick { WxJoystick::from(0 as *mut c_void) }
    
}

pub trait TWxJoystick : TWxObject {
}

pub struct WxLayoutAlgorithm { ptr: *mut c_void }
impl TWxLayoutAlgorithm for WxLayoutAlgorithm {}
impl TWxObject for WxLayoutAlgorithm { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxLayoutAlgorithm {
    pub fn from(ptr: *mut c_void) -> WxLayoutAlgorithm { WxLayoutAlgorithm { ptr: ptr } }
    pub fn null() -> WxLayoutAlgorithm { WxLayoutAlgorithm::from(0 as *mut c_void) }
    
    pub fn new() -> WxLayoutAlgorithm {
        unsafe { WxLayoutAlgorithm { ptr: wxLayoutAlgorithm_Create() } }
    }
}

pub trait TWxLayoutAlgorithm : TWxObject {
    fn layoutFrame<T: TWxFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.ptr(), frame.ptr(), mainWindow) }
    }
    fn layoutMDIFrame<T: TWxFrame>(&self, frame: &T, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.ptr(), frame.ptr(), x, y, w, h, use_) }
    }
    fn layoutWindow<T: TWxFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.ptr(), frame.ptr(), mainWindow) }
    }
}

pub struct WxQueryLayoutInfoEvent { ptr: *mut c_void }
impl TWxQueryLayoutInfoEvent for WxQueryLayoutInfoEvent {}
impl TWxEvent for WxQueryLayoutInfoEvent {}
impl TWxObject for WxQueryLayoutInfoEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxQueryLayoutInfoEvent {
    pub fn from(ptr: *mut c_void) -> WxQueryLayoutInfoEvent { WxQueryLayoutInfoEvent { ptr: ptr } }
    pub fn null() -> WxQueryLayoutInfoEvent { WxQueryLayoutInfoEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> WxQueryLayoutInfoEvent {
        unsafe { WxQueryLayoutInfoEvent { ptr: wxQueryLayoutInfoEvent_Create(id) } }
    }
}

pub trait TWxQueryLayoutInfoEvent : TWxEvent {
    fn getAlignment(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetAlignment(self.ptr()) }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetFlags(self.ptr()) }
    }
    fn getOrientation(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetOrientation(self.ptr()) }
    }
    fn getRequestedLength(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetRequestedLength(self.ptr()) }
    }
    fn getSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxQueryLayoutInfoEvent_GetSize(self.ptr()) } }
    }
    fn setAlignment(&self, align: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetAlignment(self.ptr(), align) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetFlags(self.ptr(), flags) }
    }
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetOrientation(self.ptr(), orient) }
    }
    fn setRequestedLength(&self, length: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetRequestedLength(self.ptr(), length) }
    }
    fn setSize(&self, w: c_int, h: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetSize(self.ptr(), w, h) }
    }
}

pub struct WxSashEvent { ptr: *mut c_void }
impl TWxSashEvent for WxSashEvent {}
impl TWxEvent for WxSashEvent {}
impl TWxObject for WxSashEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSashEvent {
    pub fn from(ptr: *mut c_void) -> WxSashEvent { WxSashEvent { ptr: ptr } }
    pub fn null() -> WxSashEvent { WxSashEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int, edge: c_int) -> WxSashEvent {
        unsafe { WxSashEvent { ptr: wxSashEvent_Create(id, edge) } }
    }
}

pub trait TWxSashEvent : TWxEvent {
    fn getDragRect(&self) -> WxRect {
        unsafe { WxRect { ptr: wxSashEvent_GetDragRect(self.ptr()) } }
    }
    fn getDragStatus(&self) -> c_int {
        unsafe { wxSashEvent_GetDragStatus(self.ptr()) }
    }
    fn getEdge(&self) -> c_int {
        unsafe { wxSashEvent_GetEdge(self.ptr()) }
    }
    fn setDragRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxSashEvent_SetDragRect(self.ptr(), x, y, w, h) }
    }
    fn setDragStatus(&self, status: c_int) {
        unsafe { wxSashEvent_SetDragStatus(self.ptr(), status) }
    }
    fn setEdge(&self, edge: c_int) {
        unsafe { wxSashEvent_SetEdge(self.ptr(), edge) }
    }
}

pub struct WxSashLayoutWindow { ptr: *mut c_void }
impl TWxSashLayoutWindow for WxSashLayoutWindow {}
impl TWxSashWindow for WxSashLayoutWindow {}
impl TWxWindow for WxSashLayoutWindow {}
impl TWxEvtHandler for WxSashLayoutWindow {}
impl TWxObject for WxSashLayoutWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSashLayoutWindow {
    pub fn from(ptr: *mut c_void) -> WxSashLayoutWindow { WxSashLayoutWindow { ptr: ptr } }
    pub fn null() -> WxSashLayoutWindow { WxSashLayoutWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> WxSashLayoutWindow {
        unsafe { WxSashLayoutWindow { ptr: wxSashLayoutWindow_Create(_par.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait TWxSashLayoutWindow : TWxSashWindow {
    fn getAlignment(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetAlignment(self.ptr()) }
    }
    fn getOrientation(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetOrientation(self.ptr()) }
    }
    fn setAlignment(&self, align: c_int) {
        unsafe { wxSashLayoutWindow_SetAlignment(self.ptr(), align) }
    }
    fn setDefaultSize(&self, w: c_int, h: c_int) {
        unsafe { wxSashLayoutWindow_SetDefaultSize(self.ptr(), w, h) }
    }
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxSashLayoutWindow_SetOrientation(self.ptr(), orient) }
    }
}

pub struct WxSashWindow { ptr: *mut c_void }
impl TWxSashWindow for WxSashWindow {}
impl TWxWindow for WxSashWindow {}
impl TWxEvtHandler for WxSashWindow {}
impl TWxObject for WxSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSashWindow {
    pub fn from(ptr: *mut c_void) -> WxSashWindow { WxSashWindow { ptr: ptr } }
    pub fn null() -> WxSashWindow { WxSashWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> WxSashWindow {
        unsafe { WxSashWindow { ptr: wxSashWindow_Create(_par.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait TWxSashWindow : TWxWindow {
    fn getDefaultBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetDefaultBorderSize(self.ptr()) }
    }
    fn getEdgeMargin(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_GetEdgeMargin(self.ptr(), edge) }
    }
    fn getExtraBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetExtraBorderSize(self.ptr()) }
    }
    fn getMaximumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeX(self.ptr()) }
    }
    fn getMaximumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeY(self.ptr()) }
    }
    fn getMinimumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeX(self.ptr()) }
    }
    fn getMinimumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeY(self.ptr()) }
    }
    fn getSashVisible(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_GetSashVisible(self.ptr(), edge) }
    }
    fn hasBorder(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_HasBorder(self.ptr(), edge) }
    }
    fn setDefaultBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetDefaultBorderSize(self.ptr(), width) }
    }
    fn setExtraBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetExtraBorderSize(self.ptr(), width) }
    }
    fn setMaximumSizeX(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeX(self.ptr(), max) }
    }
    fn setMaximumSizeY(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeY(self.ptr(), max) }
    }
    fn setMinimumSizeX(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeX(self.ptr(), min) }
    }
    fn setMinimumSizeY(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeY(self.ptr(), min) }
    }
    fn setSashBorder(&self, edge: c_int, border: c_int) {
        unsafe { wxSashWindow_SetSashBorder(self.ptr(), edge, border) }
    }
    fn setSashVisible(&self, edge: c_int, sash: c_int) {
        unsafe { wxSashWindow_SetSashVisible(self.ptr(), edge, sash) }
    }
}

pub struct WxSplashScreen { ptr: *mut c_void }
impl TWxSplashScreen for WxSplashScreen {}
impl TWxFrame for WxSplashScreen {}
impl TWxTopLevelWindow for WxSplashScreen {}
impl TWxWindow for WxSplashScreen {}
impl TWxEvtHandler for WxSplashScreen {}
impl TWxObject for WxSplashScreen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxSplashScreen {
    pub fn from(ptr: *mut c_void) -> WxSplashScreen { WxSplashScreen { ptr: ptr } }
    pub fn null() -> WxSplashScreen { WxSplashScreen::from(0 as *mut c_void) }
    
}

pub trait TWxSplashScreen : TWxFrame {
}

pub struct WxTaskBarIcon { ptr: *mut c_void }
impl TWxTaskBarIcon for WxTaskBarIcon {}
impl TWxEvtHandler for WxTaskBarIcon {}
impl TWxObject for WxTaskBarIcon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTaskBarIcon {
    pub fn from(ptr: *mut c_void) -> WxTaskBarIcon { WxTaskBarIcon { ptr: ptr } }
    pub fn null() -> WxTaskBarIcon { WxTaskBarIcon::from(0 as *mut c_void) }
    
    pub fn new() -> WxTaskBarIcon {
        unsafe { WxTaskBarIcon { ptr: wxTaskBarIcon_Create() } }
    }
}

pub trait TWxTaskBarIcon : TWxEvtHandler {
    fn isIconInstalled(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsOk(self.ptr()) }
    }
    fn popupMenu<T: TWxMenu>(&self, menu: &T) -> c_int {
        unsafe { wxTaskBarIcon_PopupMenu(self.ptr(), menu.ptr()) }
    }
    fn removeIcon(&self) -> c_int {
        unsafe { wxTaskBarIcon_RemoveIcon(self.ptr()) }
    }
    fn setIcon<T: TWxIcon>(&self, icon: &T, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxTaskBarIcon_SetIcon(self.ptr(), icon.ptr(), text.ptr()) }
    }
}

pub struct WxTipProvider { ptr: *mut c_void }
impl TWxTipProvider for WxTipProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxTipProvider {
    pub fn from(ptr: *mut c_void) -> WxTipProvider { WxTipProvider { ptr: ptr } }
    pub fn null() -> WxTipProvider { WxTipProvider::from(0 as *mut c_void) }
    
}

pub trait TWxTipProvider {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct WxWizard { ptr: *mut c_void }
impl TWxWizard for WxWizard {}
impl TWxDialog for WxWizard {}
impl TWxTopLevelWindow for WxWizard {}
impl TWxWindow for WxWizard {}
impl TWxEvtHandler for WxWizard {}
impl TWxObject for WxWizard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWizard {
    pub fn from(ptr: *mut c_void) -> WxWizard { WxWizard { ptr: ptr } }
    pub fn null() -> WxWizard { WxWizard::from(0 as *mut c_void) }
    
    pub fn chain<T: TWxWizardPageSimple, U: TWxWizardPageSimple>(f: &T, s: &U) {
        unsafe { wxWizard_Chain(f.ptr(), s.ptr()) }
    }
    pub fn new<T: TWxWindow, U: TWxBitmap>(_prt: &T, _id: c_int, _txt: &str, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> WxWizard {
        let _txt = wxT(_txt);
        unsafe { WxWizard { ptr: wxWizard_Create(_prt.ptr(), _id, _txt.ptr(), _bmp.ptr(), _lft, _top, _wdt, _hgt) } }
    }
}

pub trait TWxWizard : TWxDialog {
    fn getCurrentPage(&self) -> WxWizardPage {
        unsafe { WxWizardPage { ptr: wxWizard_GetCurrentPage(self.ptr()) } }
    }
    fn getPageSize(&self) -> WxSize {
        unsafe { WxSize { ptr: wxWizard_GetPageSize(self.ptr()) } }
    }
    fn runWizard<T: TWxWizardPage>(&self, firstPage: &T) -> c_int {
        unsafe { wxWizard_RunWizard(self.ptr(), firstPage.ptr()) }
    }
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.ptr(), w, h) }
    }
}

pub struct WxWizardEvent { ptr: *mut c_void }
impl TWxWizardEvent for WxWizardEvent {}
impl TWxNotifyEvent for WxWizardEvent {}
impl TWxCommandEvent for WxWizardEvent {}
impl TWxEvent for WxWizardEvent {}
impl TWxObject for WxWizardEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWizardEvent {
    pub fn from(ptr: *mut c_void) -> WxWizardEvent { WxWizardEvent { ptr: ptr } }
    pub fn null() -> WxWizardEvent { WxWizardEvent::from(0 as *mut c_void) }
    
}

pub trait TWxWizardEvent : TWxNotifyEvent {
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.ptr()) }
    }
}

pub struct WxWizardPage { ptr: *mut c_void }
impl TWxWizardPage for WxWizardPage {}
impl TWxPanel for WxWizardPage {}
impl TWxWindow for WxWizardPage {}
impl TWxEvtHandler for WxWizardPage {}
impl TWxObject for WxWizardPage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWizardPage {
    pub fn from(ptr: *mut c_void) -> WxWizardPage { WxWizardPage { ptr: ptr } }
    pub fn null() -> WxWizardPage { WxWizardPage::from(0 as *mut c_void) }
    
}

pub trait TWxWizardPage : TWxPanel {
}

pub struct WxWizardPageSimple { ptr: *mut c_void }
impl TWxWizardPageSimple for WxWizardPageSimple {}
impl TWxWizardPage for WxWizardPageSimple {}
impl TWxPanel for WxWizardPageSimple {}
impl TWxWindow for WxWizardPageSimple {}
impl TWxEvtHandler for WxWizardPageSimple {}
impl TWxObject for WxWizardPageSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxWizardPageSimple {
    pub fn from(ptr: *mut c_void) -> WxWizardPageSimple { WxWizardPageSimple { ptr: ptr } }
    pub fn null() -> WxWizardPageSimple { WxWizardPageSimple::from(0 as *mut c_void) }
    
    pub fn new<T: TWxWizard>(_prt: &T) -> WxWizardPageSimple {
        unsafe { WxWizardPageSimple { ptr: wxWizardPageSimple_Create(_prt.ptr()) } }
    }
}

pub trait TWxWizardPageSimple : TWxWizardPage {
    fn getBitmap<T: TWxBitmap>(&self, _ref: &T) {
        unsafe { wxWizardPageSimple_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    fn getNext(&self) -> WxWizardPageSimple {
        unsafe { WxWizardPageSimple { ptr: wxWizardPageSimple_GetNext(self.ptr()) } }
    }
    fn getPrev(&self) -> WxWizardPageSimple {
        unsafe { WxWizardPageSimple { ptr: wxWizardPageSimple_GetPrev(self.ptr()) } }
    }
    fn setNext<T: TWxWizardPageSimple>(&self, next: &T) {
        unsafe { wxWizardPageSimple_SetNext(self.ptr(), next.ptr()) }
    }
    fn setPrev<T: TWxWizardPageSimple>(&self, prev: &T) {
        unsafe { wxWizardPageSimple_SetPrev(self.ptr(), prev.ptr()) }
    }
}

pub struct WxManagedPtr { ptr: *mut c_void }
impl TWxManagedPtr for WxManagedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxManagedPtr {
    pub fn from(ptr: *mut c_void) -> WxManagedPtr { WxManagedPtr { ptr: ptr } }
    pub fn null() -> WxManagedPtr { WxManagedPtr::from(0 as *mut c_void) }
    
    pub fn newFromObject<T: TWxObject>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromObject(obj.ptr()) } }
    }
    pub fn newFromDateTime<T: TWxDateTime>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromDateTime(obj.ptr()) } }
    }
    pub fn newFromGridCellCoordsArray<T: TWxGridCellCoordsArray>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromGridCellCoordsArray(obj.ptr()) } }
    }
    pub fn newFromBitmap<T: TWxBitmap>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromBitmap(obj.ptr()) } }
    }
    pub fn newFromIcon<T: TWxIcon>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromIcon(obj.ptr()) } }
    }
    pub fn newFromBrush<T: TWxBrush>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromBrush(obj.ptr()) } }
    }
    pub fn newFromColour<T: TWxColour>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromColour(obj.ptr()) } }
    }
    pub fn newFromCursor<T: TWxCursor>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromCursor(obj.ptr()) } }
    }
    pub fn newFromFont<T: TWxFont>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromFont(obj.ptr()) } }
    }
    pub fn newFromPen<T: TWxPen>(obj: &T) -> WxManagedPtr {
        unsafe { WxManagedPtr { ptr: wxManagedPtr_CreateFromPen(obj.ptr()) } }
    }
}

pub trait TWxManagedPtr {
    fn ptr(&self) -> *mut c_void;
    
    fn getPtr(&self) -> *mut c_void {
        unsafe { wxManagedPtr_GetPtr(self.ptr()) }
    }
    fn noFinalize(&self) {
        unsafe { wxManagedPtr_NoFinalize(self.ptr()) }
    }
    fn finalize(&self) {
        unsafe { wxManagedPtr_Finalize(self.ptr()) }
    }
    fn delete(&self) {
        unsafe { wxManagedPtr_Delete(self.ptr()) }
    }
}

pub struct WxGridCellTextEnterEditor { ptr: *mut c_void }
impl TWxGridCellTextEnterEditor for WxGridCellTextEnterEditor {}
impl TWxGridCellTextEditor for WxGridCellTextEnterEditor {}
impl TWxGridCellEditor for WxGridCellTextEnterEditor {}
impl TWxGridCellWorker for WxGridCellTextEnterEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WxGridCellTextEnterEditor {
    pub fn from(ptr: *mut c_void) -> WxGridCellTextEnterEditor { WxGridCellTextEnterEditor { ptr: ptr } }
    pub fn null() -> WxGridCellTextEnterEditor { WxGridCellTextEnterEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> WxGridCellTextEnterEditor {
        unsafe { WxGridCellTextEnterEditor { ptr: wxGridCellTextEnterEditor_Ctor() } }
    }
}

pub trait TWxGridCellTextEnterEditor : TWxGridCellTextEditor {
}

