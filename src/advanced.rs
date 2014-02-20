use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct wxRustGridTable { ptr: *mut c_void }
impl _wxRustGridTable for wxRustGridTable {}
impl _wxGridTableBase for wxRustGridTable {}
impl _wxObject for wxRustGridTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxRustGridTable {
    pub fn from(ptr: *mut c_void) -> wxRustGridTable { wxRustGridTable { ptr: ptr } }
    pub fn null() -> wxRustGridTable { wxRustGridTable::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _EifGetNumberRows: *mut c_void, _EifGetNumberCols: *mut c_void, _EifGetValue: *mut c_void, _EifSetValue: *mut c_void, _EifIsEmptyCell: *mut c_void, _EifClear: *mut c_void, _EifInsertRows: *mut c_void, _EifAppendRows: *mut c_void, _EifDeleteRows: *mut c_void, _EifInsertCols: *mut c_void, _EifAppendCols: *mut c_void, _EifDeleteCols: *mut c_void, _EifSetRowLabelValue: *mut c_void, _EifSetColLabelValue: *mut c_void, _EifGetRowLabelValue: *mut c_void, _EifGetColLabelValue: *mut c_void) -> wxRustGridTable {
        unsafe { wxRustGridTable { ptr: ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue) } }
    }
}

pub trait _wxRustGridTable : _wxGridTableBase {
    fn getView(&self) -> wxView {
        unsafe { wxView { ptr: ELJGridTable_GetView(self.ptr()) } }
    }
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *mut c_void {
        unsafe { ELJGridTable_SendTableMessage(self.ptr(), id, val1, val2) }
    }
}

pub struct wxCalculateLayoutEvent { ptr: *mut c_void }
impl _wxCalculateLayoutEvent for wxCalculateLayoutEvent {}
impl _wxEvent for wxCalculateLayoutEvent {}
impl _wxObject for wxCalculateLayoutEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCalculateLayoutEvent {
    pub fn from(ptr: *mut c_void) -> wxCalculateLayoutEvent { wxCalculateLayoutEvent { ptr: ptr } }
    pub fn null() -> wxCalculateLayoutEvent { wxCalculateLayoutEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> wxCalculateLayoutEvent {
        unsafe { wxCalculateLayoutEvent { ptr: wxCalculateLayoutEvent_Create(id) } }
    }
}

pub trait _wxCalculateLayoutEvent : _wxEvent {
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.ptr()) }
    }
    fn getRect(&self) -> wxRect {
        unsafe { wxRect { ptr: wxCalculateLayoutEvent_GetRect(self.ptr()) } }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self.ptr(), flags) }
    }
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxCalculateLayoutEvent_SetRect(self.ptr(), x, y, w, h) }
    }
}

pub struct wxCalendarCtrl { ptr: *mut c_void }
impl _wxCalendarCtrl for wxCalendarCtrl {}
impl _wxControl for wxCalendarCtrl {}
impl _wxWindow for wxCalendarCtrl {}
impl _wxEvtHandler for wxCalendarCtrl {}
impl _wxObject for wxCalendarCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCalendarCtrl {
    pub fn from(ptr: *mut c_void) -> wxCalendarCtrl { wxCalendarCtrl { ptr: ptr } }
    pub fn null() -> wxCalendarCtrl { wxCalendarCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow, U: _wxDateTime>(_prt: &T, _id: c_int, _dat: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxCalendarCtrl {
        unsafe { wxCalendarCtrl { ptr: wxCalendarCtrl_Create(_prt.ptr(), _id, _dat.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxCalendarCtrl : _wxControl {
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
    fn getHeaderColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHeaderColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.ptr(), _ref.ptr()) }
    }
    fn getHighlightColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHighlightColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.ptr(), _ref.ptr()) }
    }
    fn getHolidayColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHolidayColourFg<T: _wxColour>(&self, _ref: &T) {
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

pub struct wxCalendarDateAttr { ptr: *mut c_void }
impl _wxCalendarDateAttr for wxCalendarDateAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCalendarDateAttr {
    pub fn from(ptr: *mut c_void) -> wxCalendarDateAttr { wxCalendarDateAttr { ptr: ptr } }
    pub fn null() -> wxCalendarDateAttr { wxCalendarDateAttr::from(0 as *mut c_void) }
    
    pub fn new(_ctxt: *mut c_void, _cbck: *mut c_void, _cbrd: *mut c_void, _fnt: *mut c_void, _brd: c_int) -> wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr { ptr: wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd) } }
    }
    pub fn newDefault() -> wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr { ptr: wxCalendarDateAttr_CreateDefault() } }
    }
}

pub trait _wxCalendarDateAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.ptr()) }
    }
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.ptr()) }
    }
    fn getBorderColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.ptr(), _ref.ptr()) }
    }
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
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
    fn setBackgroundColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.ptr(), col.ptr()) }
    }
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.ptr(), border) }
    }
    fn setBorderColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.ptr(), col.ptr()) }
    }
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxCalendarDateAttr_SetFont(self.ptr(), font.ptr()) }
    }
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.ptr(), holiday) }
    }
    fn setTextColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.ptr(), col.ptr()) }
    }
}

pub struct wxCalendarEvent { ptr: *mut c_void }
impl _wxCalendarEvent for wxCalendarEvent {}
impl _wxCommandEvent for wxCalendarEvent {}
impl _wxEvent for wxCalendarEvent {}
impl _wxObject for wxCalendarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxCalendarEvent {
    pub fn from(ptr: *mut c_void) -> wxCalendarEvent { wxCalendarEvent { ptr: ptr } }
    pub fn null() -> wxCalendarEvent { wxCalendarEvent::from(0 as *mut c_void) }
    
}

pub trait _wxCalendarEvent : _wxCommandEvent {
    fn getDate(&self, _dte: *mut c_void) {
        unsafe { wxCalendarEvent_GetDate(self.ptr(), _dte) }
    }
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.ptr()) }
    }
}

pub struct wxEditableListBox { ptr: *mut c_void }
impl _wxEditableListBox for wxEditableListBox {}
impl _wxPanel for wxEditableListBox {}
impl _wxWindow for wxEditableListBox {}
impl _wxEvtHandler for wxEditableListBox {}
impl _wxObject for wxEditableListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxEditableListBox {
    pub fn from(ptr: *mut c_void) -> wxEditableListBox { wxEditableListBox { ptr: ptr } }
    pub fn null() -> wxEditableListBox { wxEditableListBox::from(0 as *mut c_void) }
    
}

pub trait _wxEditableListBox : _wxPanel {
}

pub struct wxGrid { ptr: *mut c_void }
impl _wxGrid for wxGrid {}
impl _wxScrolledWindow for wxGrid {}
impl _wxPanel for wxGrid {}
impl _wxWindow for wxGrid {}
impl _wxEvtHandler for wxGrid {}
impl _wxObject for wxGrid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGrid {
    pub fn from(ptr: *mut c_void) -> wxGrid { wxGrid { ptr: ptr } }
    pub fn null() -> wxGrid { wxGrid::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxGrid {
        unsafe { wxGrid { ptr: wxGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxGrid : _wxScrolledWindow {
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
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> wxRect {
        unsafe { wxRect { ptr: wxGrid_BlockToDeviceRect(self.ptr(), top, left, bottom, right) } }
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
    fn cellToRect(&self, row: c_int, col: c_int) -> wxRect {
        unsafe { wxRect { ptr: wxGrid_CellToRect(self.ptr(), row, col) } }
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
    fn drawAllGridLines<T: _wxDC, U: _wxRegion>(&self, dc: &T, reg: &U) {
        unsafe { wxGrid_DrawAllGridLines(self.ptr(), dc.ptr(), reg.ptr()) }
    }
    fn drawCell<T: _wxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.ptr(), dc.ptr(), _row, _col) }
    }
    fn drawCellBorder<T: _wxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.ptr(), dc.ptr(), _row, _col) }
    }
    fn drawCellHighlight<T: _wxDC, U: _wxGridCellAttr>(&self, dc: &T, attr: &U) {
        unsafe { wxGrid_DrawCellHighlight(self.ptr(), dc.ptr(), attr.ptr()) }
    }
    fn drawColLabel<T: _wxDC>(&self, dc: &T, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.ptr(), dc.ptr(), col) }
    }
    fn drawColLabels<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawColLabels(self.ptr(), dc.ptr()) }
    }
    fn drawGridSpace<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawGridSpace(self.ptr(), dc.ptr()) }
    }
    fn drawRowLabel<T: _wxDC>(&self, dc: &T, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.ptr(), dc.ptr(), row) }
    }
    fn drawRowLabels<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawRowLabels(self.ptr(), dc.ptr()) }
    }
    fn drawTextRectangle<T: _wxDC>(&self, dc: &T, txt: &str, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
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
    fn getCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellBackgroundColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn getCellEditor(&self, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { ptr: wxGrid_GetCellEditor(self.ptr(), row, col) } }
    }
    fn getCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_GetCellFont(self.ptr(), row, col, font.ptr()) }
    }
    fn getCellHighlightColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetCellHighlightColour(self.ptr(), _ref.ptr()) }
    }
    fn getCellRenderer(&self, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { ptr: wxGrid_GetCellRenderer(self.ptr(), row, col) } }
    }
    fn getCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellTextColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn getCellValue(&self, row: c_int, col: c_int) -> ~str {
        unsafe { wxString { ptr: wxGrid_GetCellValue(self.ptr(), row, col) }.to_str() }
    }
    fn getColLabelAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetColLabelAlignment(self.ptr(), horiz, vert) }
    }
    fn getColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetColLabelSize(self.ptr()) }
    }
    fn getColLabelValue(&self, col: c_int) -> ~str {
        unsafe { wxString { ptr: wxGrid_GetColLabelValue(self.ptr(), col) }.to_str() }
    }
    fn getColSize(&self, col: c_int) -> c_int {
        unsafe { wxGrid_GetColSize(self.ptr(), col) }
    }
    fn getDefaultCellAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetDefaultCellAlignment(self.ptr(), horiz, vert) }
    }
    fn getDefaultCellBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultCellFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellFont(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultCellTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColLabelSize(self.ptr()) }
    }
    fn getDefaultColSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColSize(self.ptr()) }
    }
    fn getDefaultEditor(&self) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { ptr: wxGrid_GetDefaultEditor(self.ptr()) } }
    }
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { ptr: wxGrid_GetDefaultEditorForCell(self.ptr(), row, col) } }
    }
    fn getDefaultEditorForType(&self, typeName: &str) -> wxGridCellEditor {
        let typeName = wxT(typeName);
        unsafe { wxGridCellEditor { ptr: wxGrid_GetDefaultEditorForType(self.ptr(), typeName.ptr()) } }
    }
    fn getDefaultRenderer(&self) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { ptr: wxGrid_GetDefaultRenderer(self.ptr()) } }
    }
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { ptr: wxGrid_GetDefaultRendererForCell(self.ptr(), row, col) } }
    }
    fn getDefaultRendererForType(&self, typeName: &str) -> wxGridCellRenderer {
        let typeName = wxT(typeName);
        unsafe { wxGridCellRenderer { ptr: wxGrid_GetDefaultRendererForType(self.ptr(), typeName.ptr()) } }
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
    fn getGridLineColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetGridLineColour(self.ptr(), _ref.ptr()) }
    }
    fn getLabelBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getLabelFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelFont(self.ptr(), _ref.ptr()) }
    }
    fn getLabelTextColour<T: _wxColour>(&self, _ref: &T) {
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
        unsafe { wxString { ptr: wxGrid_GetRowLabelValue(self.ptr(), row) }.to_str() }
    }
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.ptr(), row) }
    }
    fn getSelectionBackground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionBackground(self.ptr(), _ref.ptr()) }
    }
    fn getSelectionForeground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionForeground(self.ptr(), _ref.ptr()) }
    }
    fn getTable(&self) -> wxGridTableBase {
        unsafe { wxGridTableBase { ptr: wxGrid_GetTable(self.ptr()) } }
    }
    fn getTextBoxSize<T: _wxDC>(&self, dc: &T, count: c_int, lines: *mut *mut c_char, _w: *mut c_void, _h: *mut c_void) {
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
    fn processTableMessage<T: _wxEvent>(&self, evt: &T) -> c_int {
        unsafe { wxGrid_ProcessTableMessage(self.ptr(), evt.ptr()) }
    }
    fn registerDataType<T: _wxGridCellRenderer, U: _wxGridCellEditor>(&self, typeName: &str, renderer: &T, editor: &U) {
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
    fn setCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellBackgroundColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn setCellEditor<T: _wxGridCellEditor>(&self, row: c_int, col: c_int, editor: &T) {
        unsafe { wxGrid_SetCellEditor(self.ptr(), row, col, editor.ptr()) }
    }
    fn setCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_SetCellFont(self.ptr(), row, col, font.ptr()) }
    }
    fn setCellHighlightColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxGrid_SetCellHighlightColour(self.ptr(), col.ptr()) }
    }
    fn setCellRenderer<T: _wxGridCellRenderer>(&self, row: c_int, col: c_int, renderer: &T) {
        unsafe { wxGrid_SetCellRenderer(self.ptr(), row, col, renderer.ptr()) }
    }
    fn setCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellTextColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn setCellValue(&self, row: c_int, col: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxGrid_SetCellValue(self.ptr(), row, col, s.ptr()) }
    }
    fn setColAttr<T: _wxGridCellAttr>(&self, col: c_int, attr: &T) {
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
    fn setDefaultCellBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setDefaultCellFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGrid_SetDefaultCellFont(self.ptr(), font.ptr()) }
    }
    fn setDefaultCellTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.ptr(), colour.ptr()) }
    }
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.ptr(), width, resizeExistingCols) }
    }
    fn setDefaultEditor<T: _wxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGrid_SetDefaultEditor(self.ptr(), editor.ptr()) }
    }
    fn setDefaultRenderer<T: _wxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGrid_SetDefaultRenderer(self.ptr(), renderer.ptr()) }
    }
    fn setDefaultRowSize(&self, height: c_int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self.ptr(), height, resizeExistingRows) }
    }
    fn setGridCursor(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_SetGridCursor(self.ptr(), row, col) }
    }
    fn setGridLineColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxGrid_SetGridLineColour(self.ptr(), col.ptr()) }
    }
    fn setLabelBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setLabelFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGrid_SetLabelFont(self.ptr(), font.ptr()) }
    }
    fn setLabelTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelTextColour(self.ptr(), colour.ptr()) }
    }
    fn setMargins(&self, extraWidth: c_int, extraHeight: c_int) {
        unsafe { wxGrid_SetMargins(self.ptr(), extraWidth, extraHeight) }
    }
    fn setReadOnly(&self, row: c_int, col: c_int, isReadOnly: c_int) {
        unsafe { wxGrid_SetReadOnly(self.ptr(), row, col, isReadOnly) }
    }
    fn setRowAttr<T: _wxGridCellAttr>(&self, row: c_int, attr: &T) {
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
    fn setSelectionBackground<T: _wxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionBackground(self.ptr(), c.ptr()) }
    }
    fn setSelectionForeground<T: _wxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionForeground(self.ptr(), c.ptr()) }
    }
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.ptr(), selmode) }
    }
    fn setTable<T: _wxGridTableBase>(&self, table: &T, takeOwnership: c_int, selmode: c_int) -> c_int {
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
    fn getSelectedCells<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectedCells(self.ptr(), _arr.ptr()) }
    }
    fn getSelectionBlockTopLeft<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.ptr(), _arr.ptr()) }
    }
    fn getSelectionBlockBottomRight<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
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

pub struct wxGridCellAttr { ptr: *mut c_void }
impl _wxGridCellAttr for wxGridCellAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellAttr {
    pub fn from(ptr: *mut c_void) -> wxGridCellAttr { wxGridCellAttr { ptr: ptr } }
    pub fn null() -> wxGridCellAttr { wxGridCellAttr::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellAttr {
        unsafe { wxGridCellAttr { ptr: wxGridCellAttr_Ctor() } }
    }
}

pub trait _wxGridCellAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.ptr()) }
    }
    fn getAlignment(&self, hAlign: *mut c_int, vAlign: *mut c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.ptr(), hAlign, vAlign) }
    }
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getEditor<T: _wxGrid>(&self, grid: &T, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { ptr: wxGridCellAttr_GetEditor(self.ptr(), grid.ptr(), row, col) } }
    }
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getRenderer<T: _wxGrid>(&self, grid: &T, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { ptr: wxGridCellAttr_GetRenderer(self.ptr(), grid.ptr(), row, col) } }
    }
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
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
    fn setBackgroundColour<T: _wxColour>(&self, colBack: &T) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.ptr(), colBack.ptr()) }
    }
    fn setDefAttr<T: _wxGridCellAttr>(&self, defAttr: &T) {
        unsafe { wxGridCellAttr_SetDefAttr(self.ptr(), defAttr.ptr()) }
    }
    fn setEditor<T: _wxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGridCellAttr_SetEditor(self.ptr(), editor.ptr()) }
    }
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGridCellAttr_SetFont(self.ptr(), font.ptr()) }
    }
    fn setReadOnly(&self, isReadOnly: c_int) {
        unsafe { wxGridCellAttr_SetReadOnly(self.ptr(), isReadOnly) }
    }
    fn setRenderer<T: _wxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGridCellAttr_SetRenderer(self.ptr(), renderer.ptr()) }
    }
    fn setTextColour<T: _wxColour>(&self, colText: &T) {
        unsafe { wxGridCellAttr_SetTextColour(self.ptr(), colText.ptr()) }
    }
}

pub struct wxGridCellBoolEditor { ptr: *mut c_void }
impl _wxGridCellBoolEditor for wxGridCellBoolEditor {}
impl _wxGridCellEditor for wxGridCellBoolEditor {}
impl _wxGridCellWorker for wxGridCellBoolEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellBoolEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellBoolEditor { wxGridCellBoolEditor { ptr: ptr } }
    pub fn null() -> wxGridCellBoolEditor { wxGridCellBoolEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellBoolEditor {
        unsafe { wxGridCellBoolEditor { ptr: wxGridCellBoolEditor_Ctor() } }
    }
}

pub trait _wxGridCellBoolEditor : _wxGridCellEditor {
}

pub struct wxGridCellBoolRenderer { ptr: *mut c_void }
impl _wxGridCellBoolRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellWorker for wxGridCellBoolRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellBoolRenderer {
    pub fn from(ptr: *mut c_void) -> wxGridCellBoolRenderer { wxGridCellBoolRenderer { ptr: ptr } }
    pub fn null() -> wxGridCellBoolRenderer { wxGridCellBoolRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellBoolRenderer : _wxGridCellRenderer {
}

pub struct wxGridCellChoiceEditor { ptr: *mut c_void }
impl _wxGridCellChoiceEditor for wxGridCellChoiceEditor {}
impl _wxGridCellEditor for wxGridCellChoiceEditor {}
impl _wxGridCellWorker for wxGridCellChoiceEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellChoiceEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellChoiceEditor { wxGridCellChoiceEditor { ptr: ptr } }
    pub fn null() -> wxGridCellChoiceEditor { wxGridCellChoiceEditor::from(0 as *mut c_void) }
    
    pub fn ctor(count: c_int, choices: *mut *mut c_char, allowOthers: c_int) -> wxGridCellChoiceEditor {
        unsafe { wxGridCellChoiceEditor { ptr: wxGridCellChoiceEditor_Ctor(count, choices, allowOthers) } }
    }
}

pub trait _wxGridCellChoiceEditor : _wxGridCellEditor {
}

pub struct wxGridCellCoordsArray { ptr: *mut c_void }
impl _wxGridCellCoordsArray for wxGridCellCoordsArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellCoordsArray {
    pub fn from(ptr: *mut c_void) -> wxGridCellCoordsArray { wxGridCellCoordsArray { ptr: ptr } }
    pub fn null() -> wxGridCellCoordsArray { wxGridCellCoordsArray::from(0 as *mut c_void) }
    
    pub fn new() -> wxGridCellCoordsArray {
        unsafe { wxGridCellCoordsArray { ptr: wxGridCellCoordsArray_Create() } }
    }
}

pub trait _wxGridCellCoordsArray {
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

pub struct wxGridCellEditor { ptr: *mut c_void }
impl _wxGridCellEditor for wxGridCellEditor {}
impl _wxGridCellWorker for wxGridCellEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellEditor { wxGridCellEditor { ptr: ptr } }
    pub fn null() -> wxGridCellEditor { wxGridCellEditor::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellEditor : _wxGridCellWorker {
    fn beginEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: &T) {
        unsafe { wxGridCellEditor_BeginEdit(self.ptr(), row, col, grid.ptr()) }
    }
    fn new<T: _wxWindow, U: _wxEvtHandler>(&self, parent: &T, id: c_int, evtHandler: &U) {
        unsafe { wxGridCellEditor_Create(self.ptr(), parent.ptr(), id, evtHandler.ptr()) }
    }
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.ptr()) }
    }
    fn endEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: &T, oldStr: &str, newStr: &str) -> c_int {
        let oldStr = wxT(oldStr);
        let newStr = wxT(newStr);
        unsafe { wxGridCellEditor_EndEdit(self.ptr(), row, col, grid.ptr(), oldStr.ptr(), newStr.ptr()) }
    }
    fn getControl(&self) -> wxControl {
        unsafe { wxControl { ptr: wxGridCellEditor_GetControl(self.ptr()) } }
    }
    fn handleReturn<T: _wxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_HandleReturn(self.ptr(), event.ptr()) }
    }
    fn isAcceptedKey<T: _wxEvent>(&self, event: &T) -> c_int {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.ptr(), event.ptr()) }
    }
    fn isCreated(&self) -> c_int {
        unsafe { wxGridCellEditor_IsCreated(self.ptr()) }
    }
    fn paintBackground<T: _wxDC, U: _wxGridCellAttr>(&self, dc: &T, x: c_int, y: c_int, w: c_int, h: c_int, attr: &U) {
        unsafe { wxGridCellEditor_PaintBackground(self.ptr(), dc.ptr(), x, y, w, h, attr.ptr()) }
    }
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.ptr()) }
    }
    fn setControl<T: _wxControl>(&self, control: &T) {
        unsafe { wxGridCellEditor_SetControl(self.ptr(), control.ptr()) }
    }
    fn setParameters(&self, params: &str) {
        let params = wxT(params);
        unsafe { wxGridCellEditor_SetParameters(self.ptr(), params.ptr()) }
    }
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.ptr(), x, y, w, h) }
    }
    fn show<T: _wxGridCellAttr>(&self, show: c_int, attr: &T) {
        unsafe { wxGridCellEditor_Show(self.ptr(), show, attr.ptr()) }
    }
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.ptr()) }
    }
    fn startingKey<T: _wxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_StartingKey(self.ptr(), event.ptr()) }
    }
}

pub struct wxGridCellFloatEditor { ptr: *mut c_void }
impl _wxGridCellFloatEditor for wxGridCellFloatEditor {}
impl _wxGridCellTextEditor for wxGridCellFloatEditor {}
impl _wxGridCellEditor for wxGridCellFloatEditor {}
impl _wxGridCellWorker for wxGridCellFloatEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellFloatEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellFloatEditor { wxGridCellFloatEditor { ptr: ptr } }
    pub fn null() -> wxGridCellFloatEditor { wxGridCellFloatEditor::from(0 as *mut c_void) }
    
    pub fn ctor(width: c_int, precision: c_int) -> wxGridCellFloatEditor {
        unsafe { wxGridCellFloatEditor { ptr: wxGridCellFloatEditor_Ctor(width, precision) } }
    }
}

pub trait _wxGridCellFloatEditor : _wxGridCellTextEditor {
}

pub struct wxGridCellFloatRenderer { ptr: *mut c_void }
impl _wxGridCellFloatRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellStringRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellWorker for wxGridCellFloatRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellFloatRenderer {
    pub fn from(ptr: *mut c_void) -> wxGridCellFloatRenderer { wxGridCellFloatRenderer { ptr: ptr } }
    pub fn null() -> wxGridCellFloatRenderer { wxGridCellFloatRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellFloatRenderer : _wxGridCellStringRenderer {
}

pub struct wxGridCellNumberEditor { ptr: *mut c_void }
impl _wxGridCellNumberEditor for wxGridCellNumberEditor {}
impl _wxGridCellTextEditor for wxGridCellNumberEditor {}
impl _wxGridCellEditor for wxGridCellNumberEditor {}
impl _wxGridCellWorker for wxGridCellNumberEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellNumberEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellNumberEditor { wxGridCellNumberEditor { ptr: ptr } }
    pub fn null() -> wxGridCellNumberEditor { wxGridCellNumberEditor::from(0 as *mut c_void) }
    
    pub fn ctor(min: c_int, max: c_int) -> wxGridCellNumberEditor {
        unsafe { wxGridCellNumberEditor { ptr: wxGridCellNumberEditor_Ctor(min, max) } }
    }
}

pub trait _wxGridCellNumberEditor : _wxGridCellTextEditor {
}

pub struct wxGridCellNumberRenderer { ptr: *mut c_void }
impl _wxGridCellNumberRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellStringRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellWorker for wxGridCellNumberRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellNumberRenderer {
    pub fn from(ptr: *mut c_void) -> wxGridCellNumberRenderer { wxGridCellNumberRenderer { ptr: ptr } }
    pub fn null() -> wxGridCellNumberRenderer { wxGridCellNumberRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellNumberRenderer {
        unsafe { wxGridCellNumberRenderer { ptr: wxGridCellNumberRenderer_Ctor() } }
    }
}

pub trait _wxGridCellNumberRenderer : _wxGridCellStringRenderer {
}

pub struct wxGridCellAutoWrapStringRenderer { ptr: *mut c_void }
impl _wxGridCellAutoWrapStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellWorker for wxGridCellAutoWrapStringRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellAutoWrapStringRenderer {
    pub fn from(ptr: *mut c_void) -> wxGridCellAutoWrapStringRenderer { wxGridCellAutoWrapStringRenderer { ptr: ptr } }
    pub fn null() -> wxGridCellAutoWrapStringRenderer { wxGridCellAutoWrapStringRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellAutoWrapStringRenderer {
        unsafe { wxGridCellAutoWrapStringRenderer { ptr: wxGridCellAutoWrapStringRenderer_Ctor() } }
    }
}

pub trait _wxGridCellAutoWrapStringRenderer : _wxGridCellStringRenderer {
}

pub struct wxGridCellRenderer { ptr: *mut c_void }
impl _wxGridCellRenderer for wxGridCellRenderer {}
impl _wxGridCellWorker for wxGridCellRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellRenderer {
    pub fn from(ptr: *mut c_void) -> wxGridCellRenderer { wxGridCellRenderer { ptr: ptr } }
    pub fn null() -> wxGridCellRenderer { wxGridCellRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellRenderer : _wxGridCellWorker {
}

pub struct wxGridCellStringRenderer { ptr: *mut c_void }
impl _wxGridCellStringRenderer for wxGridCellStringRenderer {}
impl _wxGridCellRenderer for wxGridCellStringRenderer {}
impl _wxGridCellWorker for wxGridCellStringRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellStringRenderer {
    pub fn from(ptr: *mut c_void) -> wxGridCellStringRenderer { wxGridCellStringRenderer { ptr: ptr } }
    pub fn null() -> wxGridCellStringRenderer { wxGridCellStringRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellStringRenderer : _wxGridCellRenderer {
}

pub struct wxGridCellTextEditor { ptr: *mut c_void }
impl _wxGridCellTextEditor for wxGridCellTextEditor {}
impl _wxGridCellEditor for wxGridCellTextEditor {}
impl _wxGridCellWorker for wxGridCellTextEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellTextEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellTextEditor { wxGridCellTextEditor { ptr: ptr } }
    pub fn null() -> wxGridCellTextEditor { wxGridCellTextEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellTextEditor {
        unsafe { wxGridCellTextEditor { ptr: wxGridCellTextEditor_Ctor() } }
    }
}

pub trait _wxGridCellTextEditor : _wxGridCellEditor {
}

pub struct wxGridCellWorker { ptr: *mut c_void }
impl _wxGridCellWorker for wxGridCellWorker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellWorker {
    pub fn from(ptr: *mut c_void) -> wxGridCellWorker { wxGridCellWorker { ptr: ptr } }
    pub fn null() -> wxGridCellWorker { wxGridCellWorker::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellWorker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxGridEditorCreatedEvent { ptr: *mut c_void }
impl _wxGridEditorCreatedEvent for wxGridEditorCreatedEvent {}
impl _wxCommandEvent for wxGridEditorCreatedEvent {}
impl _wxEvent for wxGridEditorCreatedEvent {}
impl _wxObject for wxGridEditorCreatedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridEditorCreatedEvent {
    pub fn from(ptr: *mut c_void) -> wxGridEditorCreatedEvent { wxGridEditorCreatedEvent { ptr: ptr } }
    pub fn null() -> wxGridEditorCreatedEvent { wxGridEditorCreatedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridEditorCreatedEvent : _wxCommandEvent {
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.ptr()) }
    }
    fn getControl(&self) -> wxControl {
        unsafe { wxControl { ptr: wxGridEditorCreatedEvent_GetControl(self.ptr()) } }
    }
    fn getRow(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self.ptr()) }
    }
    fn setCol(&self, col: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self.ptr(), col) }
    }
    fn setControl<T: _wxControl>(&self, ctrl: &T) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.ptr(), ctrl.ptr()) }
    }
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.ptr(), row) }
    }
}

pub struct wxGridEvent { ptr: *mut c_void }
impl _wxGridEvent for wxGridEvent {}
impl _wxNotifyEvent for wxGridEvent {}
impl _wxCommandEvent for wxGridEvent {}
impl _wxEvent for wxGridEvent {}
impl _wxObject for wxGridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridEvent {
    pub fn from(ptr: *mut c_void) -> wxGridEvent { wxGridEvent { ptr: ptr } }
    pub fn null() -> wxGridEvent { wxGridEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridEvent : _wxNotifyEvent {
    fn altDown(&self) -> c_int {
        unsafe { wxGridEvent_AltDown(self.ptr()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridEvent_ControlDown(self.ptr()) }
    }
    fn getCol(&self) -> c_int {
        unsafe { wxGridEvent_GetCol(self.ptr()) }
    }
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxGridEvent_GetPosition(self.ptr()) } }
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

pub struct wxGridRangeSelectEvent { ptr: *mut c_void }
impl _wxGridRangeSelectEvent for wxGridRangeSelectEvent {}
impl _wxNotifyEvent for wxGridRangeSelectEvent {}
impl _wxCommandEvent for wxGridRangeSelectEvent {}
impl _wxEvent for wxGridRangeSelectEvent {}
impl _wxObject for wxGridRangeSelectEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridRangeSelectEvent {
    pub fn from(ptr: *mut c_void) -> wxGridRangeSelectEvent { wxGridRangeSelectEvent { ptr: ptr } }
    pub fn null() -> wxGridRangeSelectEvent { wxGridRangeSelectEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridRangeSelectEvent : _wxNotifyEvent {
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

pub struct wxGridSizeEvent { ptr: *mut c_void }
impl _wxGridSizeEvent for wxGridSizeEvent {}
impl _wxNotifyEvent for wxGridSizeEvent {}
impl _wxCommandEvent for wxGridSizeEvent {}
impl _wxEvent for wxGridSizeEvent {}
impl _wxObject for wxGridSizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridSizeEvent {
    pub fn from(ptr: *mut c_void) -> wxGridSizeEvent { wxGridSizeEvent { ptr: ptr } }
    pub fn null() -> wxGridSizeEvent { wxGridSizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridSizeEvent : _wxNotifyEvent {
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.ptr()) }
    }
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { ptr: wxGridSizeEvent_GetPosition(self.ptr()) } }
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

pub struct wxGridTableBase { ptr: *mut c_void }
impl _wxGridTableBase for wxGridTableBase {}
impl _wxObject for wxGridTableBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridTableBase {
    pub fn from(ptr: *mut c_void) -> wxGridTableBase { wxGridTableBase { ptr: ptr } }
    pub fn null() -> wxGridTableBase { wxGridTableBase::from(0 as *mut c_void) }
    
}

pub trait _wxGridTableBase : _wxObject {
}

pub struct wxJoystick { ptr: *mut c_void }
impl _wxJoystick for wxJoystick {}
impl _wxObject for wxJoystick { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxJoystick {
    pub fn from(ptr: *mut c_void) -> wxJoystick { wxJoystick { ptr: ptr } }
    pub fn null() -> wxJoystick { wxJoystick::from(0 as *mut c_void) }
    
}

pub trait _wxJoystick : _wxObject {
}

pub struct wxLayoutAlgorithm { ptr: *mut c_void }
impl _wxLayoutAlgorithm for wxLayoutAlgorithm {}
impl _wxObject for wxLayoutAlgorithm { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxLayoutAlgorithm {
    pub fn from(ptr: *mut c_void) -> wxLayoutAlgorithm { wxLayoutAlgorithm { ptr: ptr } }
    pub fn null() -> wxLayoutAlgorithm { wxLayoutAlgorithm::from(0 as *mut c_void) }
    
    pub fn new() -> wxLayoutAlgorithm {
        unsafe { wxLayoutAlgorithm { ptr: wxLayoutAlgorithm_Create() } }
    }
}

pub trait _wxLayoutAlgorithm : _wxObject {
    fn layoutFrame<T: _wxFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.ptr(), frame.ptr(), mainWindow) }
    }
    fn layoutMDIFrame<T: _wxFrame>(&self, frame: &T, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.ptr(), frame.ptr(), x, y, w, h, use_) }
    }
    fn layoutWindow<T: _wxFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.ptr(), frame.ptr(), mainWindow) }
    }
}

pub struct wxQueryLayoutInfoEvent { ptr: *mut c_void }
impl _wxQueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl _wxEvent for wxQueryLayoutInfoEvent {}
impl _wxObject for wxQueryLayoutInfoEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxQueryLayoutInfoEvent {
    pub fn from(ptr: *mut c_void) -> wxQueryLayoutInfoEvent { wxQueryLayoutInfoEvent { ptr: ptr } }
    pub fn null() -> wxQueryLayoutInfoEvent { wxQueryLayoutInfoEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> wxQueryLayoutInfoEvent {
        unsafe { wxQueryLayoutInfoEvent { ptr: wxQueryLayoutInfoEvent_Create(id) } }
    }
}

pub trait _wxQueryLayoutInfoEvent : _wxEvent {
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
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxQueryLayoutInfoEvent_GetSize(self.ptr()) } }
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

pub struct wxSashEvent { ptr: *mut c_void }
impl _wxSashEvent for wxSashEvent {}
impl _wxEvent for wxSashEvent {}
impl _wxObject for wxSashEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSashEvent {
    pub fn from(ptr: *mut c_void) -> wxSashEvent { wxSashEvent { ptr: ptr } }
    pub fn null() -> wxSashEvent { wxSashEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int, edge: c_int) -> wxSashEvent {
        unsafe { wxSashEvent { ptr: wxSashEvent_Create(id, edge) } }
    }
}

pub trait _wxSashEvent : _wxEvent {
    fn getDragRect(&self) -> wxRect {
        unsafe { wxRect { ptr: wxSashEvent_GetDragRect(self.ptr()) } }
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

pub struct wxSashLayoutWindow { ptr: *mut c_void }
impl _wxSashLayoutWindow for wxSashLayoutWindow {}
impl _wxSashWindow for wxSashLayoutWindow {}
impl _wxWindow for wxSashLayoutWindow {}
impl _wxEvtHandler for wxSashLayoutWindow {}
impl _wxObject for wxSashLayoutWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSashLayoutWindow {
    pub fn from(ptr: *mut c_void) -> wxSashLayoutWindow { wxSashLayoutWindow { ptr: ptr } }
    pub fn null() -> wxSashLayoutWindow { wxSashLayoutWindow::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxSashLayoutWindow {
        unsafe { wxSashLayoutWindow { ptr: wxSashLayoutWindow_Create(_par.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait _wxSashLayoutWindow : _wxSashWindow {
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

pub struct wxSashWindow { ptr: *mut c_void }
impl _wxSashWindow for wxSashWindow {}
impl _wxWindow for wxSashWindow {}
impl _wxEvtHandler for wxSashWindow {}
impl _wxObject for wxSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSashWindow {
    pub fn from(ptr: *mut c_void) -> wxSashWindow { wxSashWindow { ptr: ptr } }
    pub fn null() -> wxSashWindow { wxSashWindow::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxSashWindow {
        unsafe { wxSashWindow { ptr: wxSashWindow_Create(_par.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait _wxSashWindow : _wxWindow {
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

pub struct wxSplashScreen { ptr: *mut c_void }
impl _wxSplashScreen for wxSplashScreen {}
impl _wxFrame for wxSplashScreen {}
impl _wxTopLevelWindow for wxSplashScreen {}
impl _wxWindow for wxSplashScreen {}
impl _wxEvtHandler for wxSplashScreen {}
impl _wxObject for wxSplashScreen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxSplashScreen {
    pub fn from(ptr: *mut c_void) -> wxSplashScreen { wxSplashScreen { ptr: ptr } }
    pub fn null() -> wxSplashScreen { wxSplashScreen::from(0 as *mut c_void) }
    
}

pub trait _wxSplashScreen : _wxFrame {
}

pub struct wxTaskBarIcon { ptr: *mut c_void }
impl _wxTaskBarIcon for wxTaskBarIcon {}
impl _wxEvtHandler for wxTaskBarIcon {}
impl _wxObject for wxTaskBarIcon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTaskBarIcon {
    pub fn from(ptr: *mut c_void) -> wxTaskBarIcon { wxTaskBarIcon { ptr: ptr } }
    pub fn null() -> wxTaskBarIcon { wxTaskBarIcon::from(0 as *mut c_void) }
    
    pub fn new() -> wxTaskBarIcon {
        unsafe { wxTaskBarIcon { ptr: wxTaskBarIcon_Create() } }
    }
}

pub trait _wxTaskBarIcon : _wxEvtHandler {
    fn isIconInstalled(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsOk(self.ptr()) }
    }
    fn popupMenu<T: _wxMenu>(&self, menu: &T) -> c_int {
        unsafe { wxTaskBarIcon_PopupMenu(self.ptr(), menu.ptr()) }
    }
    fn removeIcon(&self) -> c_int {
        unsafe { wxTaskBarIcon_RemoveIcon(self.ptr()) }
    }
    fn setIcon<T: _wxIcon>(&self, icon: &T, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxTaskBarIcon_SetIcon(self.ptr(), icon.ptr(), text.ptr()) }
    }
}

pub struct wxTipProvider { ptr: *mut c_void }
impl _wxTipProvider for wxTipProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxTipProvider {
    pub fn from(ptr: *mut c_void) -> wxTipProvider { wxTipProvider { ptr: ptr } }
    pub fn null() -> wxTipProvider { wxTipProvider::from(0 as *mut c_void) }
    
}

pub trait _wxTipProvider {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct wxWizard { ptr: *mut c_void }
impl _wxWizard for wxWizard {}
impl _wxDialog for wxWizard {}
impl _wxTopLevelWindow for wxWizard {}
impl _wxWindow for wxWizard {}
impl _wxEvtHandler for wxWizard {}
impl _wxObject for wxWizard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWizard {
    pub fn from(ptr: *mut c_void) -> wxWizard { wxWizard { ptr: ptr } }
    pub fn null() -> wxWizard { wxWizard::from(0 as *mut c_void) }
    
    pub fn chain<T: _wxWizardPageSimple, U: _wxWizardPageSimple>(f: &T, s: &U) {
        unsafe { wxWizard_Chain(f.ptr(), s.ptr()) }
    }
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, _txt: &str, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> wxWizard {
        let _txt = wxT(_txt);
        unsafe { wxWizard { ptr: wxWizard_Create(_prt.ptr(), _id, _txt.ptr(), _bmp.ptr(), _lft, _top, _wdt, _hgt) } }
    }
}

pub trait _wxWizard : _wxDialog {
    fn getCurrentPage(&self) -> wxWizardPage {
        unsafe { wxWizardPage { ptr: wxWizard_GetCurrentPage(self.ptr()) } }
    }
    fn getPageSize(&self) -> wxSize {
        unsafe { wxSize { ptr: wxWizard_GetPageSize(self.ptr()) } }
    }
    fn runWizard<T: _wxWizardPage>(&self, firstPage: &T) -> c_int {
        unsafe { wxWizard_RunWizard(self.ptr(), firstPage.ptr()) }
    }
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.ptr(), w, h) }
    }
}

pub struct wxWizardEvent { ptr: *mut c_void }
impl _wxWizardEvent for wxWizardEvent {}
impl _wxNotifyEvent for wxWizardEvent {}
impl _wxCommandEvent for wxWizardEvent {}
impl _wxEvent for wxWizardEvent {}
impl _wxObject for wxWizardEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWizardEvent {
    pub fn from(ptr: *mut c_void) -> wxWizardEvent { wxWizardEvent { ptr: ptr } }
    pub fn null() -> wxWizardEvent { wxWizardEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWizardEvent : _wxNotifyEvent {
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.ptr()) }
    }
}

pub struct wxWizardPage { ptr: *mut c_void }
impl _wxWizardPage for wxWizardPage {}
impl _wxPanel for wxWizardPage {}
impl _wxWindow for wxWizardPage {}
impl _wxEvtHandler for wxWizardPage {}
impl _wxObject for wxWizardPage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWizardPage {
    pub fn from(ptr: *mut c_void) -> wxWizardPage { wxWizardPage { ptr: ptr } }
    pub fn null() -> wxWizardPage { wxWizardPage::from(0 as *mut c_void) }
    
}

pub trait _wxWizardPage : _wxPanel {
}

pub struct wxWizardPageSimple { ptr: *mut c_void }
impl _wxWizardPageSimple for wxWizardPageSimple {}
impl _wxWizardPage for wxWizardPageSimple {}
impl _wxPanel for wxWizardPageSimple {}
impl _wxWindow for wxWizardPageSimple {}
impl _wxEvtHandler for wxWizardPageSimple {}
impl _wxObject for wxWizardPageSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxWizardPageSimple {
    pub fn from(ptr: *mut c_void) -> wxWizardPageSimple { wxWizardPageSimple { ptr: ptr } }
    pub fn null() -> wxWizardPageSimple { wxWizardPageSimple::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWizard>(_prt: &T) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple { ptr: wxWizardPageSimple_Create(_prt.ptr()) } }
    }
}

pub trait _wxWizardPageSimple : _wxWizardPage {
    fn getBitmap<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxWizardPageSimple_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    fn getNext(&self) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple { ptr: wxWizardPageSimple_GetNext(self.ptr()) } }
    }
    fn getPrev(&self) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple { ptr: wxWizardPageSimple_GetPrev(self.ptr()) } }
    }
    fn setNext<T: _wxWizardPageSimple>(&self, next: &T) {
        unsafe { wxWizardPageSimple_SetNext(self.ptr(), next.ptr()) }
    }
    fn setPrev<T: _wxWizardPageSimple>(&self, prev: &T) {
        unsafe { wxWizardPageSimple_SetPrev(self.ptr(), prev.ptr()) }
    }
}

pub struct wxManagedPtr { ptr: *mut c_void }
impl _wxManagedPtr for wxManagedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxManagedPtr {
    pub fn from(ptr: *mut c_void) -> wxManagedPtr { wxManagedPtr { ptr: ptr } }
    pub fn null() -> wxManagedPtr { wxManagedPtr::from(0 as *mut c_void) }
    
    pub fn newFromObject<T: _wxObject>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromObject(obj.ptr()) } }
    }
    pub fn newFromDateTime<T: _wxDateTime>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromDateTime(obj.ptr()) } }
    }
    pub fn newFromGridCellCoordsArray<T: _wxGridCellCoordsArray>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromGridCellCoordsArray(obj.ptr()) } }
    }
    pub fn newFromBitmap<T: _wxBitmap>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromBitmap(obj.ptr()) } }
    }
    pub fn newFromIcon<T: _wxIcon>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromIcon(obj.ptr()) } }
    }
    pub fn newFromBrush<T: _wxBrush>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromBrush(obj.ptr()) } }
    }
    pub fn newFromColour<T: _wxColour>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromColour(obj.ptr()) } }
    }
    pub fn newFromCursor<T: _wxCursor>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromCursor(obj.ptr()) } }
    }
    pub fn newFromFont<T: _wxFont>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromFont(obj.ptr()) } }
    }
    pub fn newFromPen<T: _wxPen>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { ptr: wxManagedPtr_CreateFromPen(obj.ptr()) } }
    }
}

pub trait _wxManagedPtr {
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

pub struct wxGridCellTextEnterEditor { ptr: *mut c_void }
impl _wxGridCellTextEnterEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellTextEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellWorker for wxGridCellTextEnterEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl wxGridCellTextEnterEditor {
    pub fn from(ptr: *mut c_void) -> wxGridCellTextEnterEditor { wxGridCellTextEnterEditor { ptr: ptr } }
    pub fn null() -> wxGridCellTextEnterEditor { wxGridCellTextEnterEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellTextEnterEditor {
        unsafe { wxGridCellTextEnterEditor { ptr: wxGridCellTextEnterEditor_Ctor() } }
    }
}

pub trait _wxGridCellTextEnterEditor : _wxGridCellTextEditor {
}

