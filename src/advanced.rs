use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct ELJGridTable { handle: *mut c_void }
impl _ELJGridTable for ELJGridTable {}
impl _wxGridTableBase for ELJGridTable {}
impl _wxObject for ELJGridTable { fn handle(&self) -> *mut c_void { self.handle } }

impl ELJGridTable {
    pub fn from(handle: *mut c_void) -> ELJGridTable { ELJGridTable { handle: handle } }
    pub fn null() -> ELJGridTable { ELJGridTable::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _EifGetNumberRows: *mut c_void, _EifGetNumberCols: *mut c_void, _EifGetValue: *mut c_void, _EifSetValue: *mut c_void, _EifIsEmptyCell: *mut c_void, _EifClear: *mut c_void, _EifInsertRows: *mut c_void, _EifAppendRows: *mut c_void, _EifDeleteRows: *mut c_void, _EifInsertCols: *mut c_void, _EifAppendCols: *mut c_void, _EifDeleteCols: *mut c_void, _EifSetRowLabelValue: *mut c_void, _EifSetColLabelValue: *mut c_void, _EifGetRowLabelValue: *mut c_void, _EifGetColLabelValue: *mut c_void) -> ELJGridTable {
        unsafe { ELJGridTable { handle: ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue) } }
    }
}

pub trait _ELJGridTable : _wxGridTableBase {
    fn getView(&self) -> wxView {
        unsafe { wxView { handle: ELJGridTable_GetView(self.handle()) } }
    }
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *mut c_void {
        unsafe { ELJGridTable_SendTableMessage(self.handle(), id, val1, val2) }
    }
}

pub struct wxCalculateLayoutEvent { handle: *mut c_void }
impl _wxCalculateLayoutEvent for wxCalculateLayoutEvent {}
impl _wxEvent for wxCalculateLayoutEvent {}
impl _wxObject for wxCalculateLayoutEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxCalculateLayoutEvent {
    pub fn from(handle: *mut c_void) -> wxCalculateLayoutEvent { wxCalculateLayoutEvent { handle: handle } }
    pub fn null() -> wxCalculateLayoutEvent { wxCalculateLayoutEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> wxCalculateLayoutEvent {
        unsafe { wxCalculateLayoutEvent { handle: wxCalculateLayoutEvent_Create(id) } }
    }
}

pub trait _wxCalculateLayoutEvent : _wxEvent {
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.handle()) }
    }
    fn getRect(&self) -> wxRect {
        unsafe { wxRect { handle: wxCalculateLayoutEvent_GetRect(self.handle()) } }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self.handle(), flags) }
    }
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxCalculateLayoutEvent_SetRect(self.handle(), x, y, w, h) }
    }
}

pub struct wxCalendarCtrl { handle: *mut c_void }
impl _wxCalendarCtrl for wxCalendarCtrl {}
impl _wxControl for wxCalendarCtrl {}
impl _wxWindow for wxCalendarCtrl {}
impl _wxEvtHandler for wxCalendarCtrl {}
impl _wxObject for wxCalendarCtrl { fn handle(&self) -> *mut c_void { self.handle } }

impl wxCalendarCtrl {
    pub fn from(handle: *mut c_void) -> wxCalendarCtrl { wxCalendarCtrl { handle: handle } }
    pub fn null() -> wxCalendarCtrl { wxCalendarCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow, U: _wxDateTime>(_prt: &T, _id: c_int, _dat: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxCalendarCtrl {
        unsafe { wxCalendarCtrl { handle: wxCalendarCtrl_Create(_prt.handle(), _id, _dat.handle(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxCalendarCtrl : _wxControl {
    fn enableHolidayDisplay(&self, display: c_int) {
        unsafe { wxCalendarCtrl_EnableHolidayDisplay(self.handle(), display) }
    }
    fn enableMonthChange(&self, enable: c_int) {
        unsafe { wxCalendarCtrl_EnableMonthChange(self.handle(), enable) }
    }
    fn getAttr(&self, day: c_int) -> *mut c_void {
        unsafe { wxCalendarCtrl_GetAttr(self.handle(), day) }
    }
    fn getDate(&self, date: *mut c_void) {
        unsafe { wxCalendarCtrl_GetDate(self.handle(), date) }
    }
    fn getHeaderColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.handle(), _ref.handle()) }
    }
    fn getHeaderColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.handle(), _ref.handle()) }
    }
    fn getHighlightColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.handle(), _ref.handle()) }
    }
    fn getHighlightColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.handle(), _ref.handle()) }
    }
    fn getHolidayColourBg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.handle(), _ref.handle()) }
    }
    fn getHolidayColourFg<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourFg(self.handle(), _ref.handle()) }
    }
    fn hitTest(&self, x: c_int, y: c_int, date: *mut c_void, wd: *mut c_void) -> c_int {
        unsafe { wxCalendarCtrl_HitTest(self.handle(), x, y, date, wd) }
    }
    fn resetAttr(&self, day: c_int) {
        unsafe { wxCalendarCtrl_ResetAttr(self.handle(), day) }
    }
    fn setAttr(&self, day: c_int, attr: *mut c_void) {
        unsafe { wxCalendarCtrl_SetAttr(self.handle(), day, attr) }
    }
    fn setDate(&self, date: *mut c_void) {
        unsafe { wxCalendarCtrl_SetDate(self.handle(), date) }
    }
    fn setHeaderColours(&self, colFg: *mut c_void, colBg: *mut c_void) {
        unsafe { wxCalendarCtrl_SetHeaderColours(self.handle(), colFg, colBg) }
    }
    fn setHighlightColours(&self, colFg: *mut c_void, colBg: *mut c_void) {
        unsafe { wxCalendarCtrl_SetHighlightColours(self.handle(), colFg, colBg) }
    }
    fn setHoliday(&self, day: c_int) {
        unsafe { wxCalendarCtrl_SetHoliday(self.handle(), day) }
    }
    fn setHolidayColours(&self, colFg: *mut c_void, colBg: *mut c_void) {
        unsafe { wxCalendarCtrl_SetHolidayColours(self.handle(), colFg, colBg) }
    }
}

pub struct wxCalendarDateAttr { handle: *mut c_void }
impl _wxCalendarDateAttr for wxCalendarDateAttr { fn handle(&self) -> *mut c_void { self.handle } }

impl wxCalendarDateAttr {
    pub fn from(handle: *mut c_void) -> wxCalendarDateAttr { wxCalendarDateAttr { handle: handle } }
    pub fn null() -> wxCalendarDateAttr { wxCalendarDateAttr::from(0 as *mut c_void) }
    
    pub fn new(_ctxt: *mut c_void, _cbck: *mut c_void, _cbrd: *mut c_void, _fnt: *mut c_void, _brd: c_int) -> wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr { handle: wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd) } }
    }
    pub fn newDefault() -> wxCalendarDateAttr {
        unsafe { wxCalendarDateAttr { handle: wxCalendarDateAttr_CreateDefault() } }
    }
}

pub trait _wxCalendarDateAttr {
    fn handle(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.handle()) }
    }
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.handle()) }
    }
    fn getBorderColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.handle(), _ref.handle()) }
    }
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetFont(self.handle(), _ref.handle()) }
    }
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetTextColour(self.handle(), _ref.handle()) }
    }
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasBackgroundColour(self.handle()) }
    }
    fn hasBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasBorder(self.handle()) }
    }
    fn hasBorderColour(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasBorderColour(self.handle()) }
    }
    fn hasFont(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasFont(self.handle()) }
    }
    fn hasTextColour(&self) -> c_int {
        unsafe { wxCalendarDateAttr_HasTextColour(self.handle()) }
    }
    fn isHoliday(&self) -> c_int {
        unsafe { wxCalendarDateAttr_IsHoliday(self.handle()) }
    }
    fn setBackgroundColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.handle(), col.handle()) }
    }
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.handle(), border) }
    }
    fn setBorderColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.handle(), col.handle()) }
    }
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxCalendarDateAttr_SetFont(self.handle(), font.handle()) }
    }
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.handle(), holiday) }
    }
    fn setTextColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.handle(), col.handle()) }
    }
}

pub struct wxCalendarEvent { handle: *mut c_void }
impl _wxCalendarEvent for wxCalendarEvent {}
impl _wxCommandEvent for wxCalendarEvent {}
impl _wxEvent for wxCalendarEvent {}
impl _wxObject for wxCalendarEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxCalendarEvent {
    pub fn from(handle: *mut c_void) -> wxCalendarEvent { wxCalendarEvent { handle: handle } }
    pub fn null() -> wxCalendarEvent { wxCalendarEvent::from(0 as *mut c_void) }
    
}

pub trait _wxCalendarEvent : _wxCommandEvent {
    fn getDate(&self, _dte: *mut c_void) {
        unsafe { wxCalendarEvent_GetDate(self.handle(), _dte) }
    }
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.handle()) }
    }
}

pub struct wxEditableListBox { handle: *mut c_void }
impl _wxEditableListBox for wxEditableListBox {}
impl _wxPanel for wxEditableListBox {}
impl _wxWindow for wxEditableListBox {}
impl _wxEvtHandler for wxEditableListBox {}
impl _wxObject for wxEditableListBox { fn handle(&self) -> *mut c_void { self.handle } }

impl wxEditableListBox {
    pub fn from(handle: *mut c_void) -> wxEditableListBox { wxEditableListBox { handle: handle } }
    pub fn null() -> wxEditableListBox { wxEditableListBox::from(0 as *mut c_void) }
    
}

pub trait _wxEditableListBox : _wxPanel {
}

pub struct wxGrid { handle: *mut c_void }
impl _wxGrid for wxGrid {}
impl _wxScrolledWindow for wxGrid {}
impl _wxPanel for wxGrid {}
impl _wxWindow for wxGrid {}
impl _wxEvtHandler for wxGrid {}
impl _wxObject for wxGrid { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGrid {
    pub fn from(handle: *mut c_void) -> wxGrid { wxGrid { handle: handle } }
    pub fn null() -> wxGrid { wxGrid::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> wxGrid {
        unsafe { wxGrid { handle: wxGrid_Create(_prt.handle(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait _wxGrid : _wxScrolledWindow {
    fn appendCols(&self, numCols: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_AppendCols(self.handle(), numCols, updateLabels) }
    }
    fn appendRows(&self, numRows: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_AppendRows(self.handle(), numRows, updateLabels) }
    }
    fn autoSize(&self) {
        unsafe { wxGrid_AutoSize(self.handle()) }
    }
    fn autoSizeColumn(&self, col: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumn(self.handle(), col, setAsMin) }
    }
    fn autoSizeColumns(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeColumns(self.handle(), setAsMin) }
    }
    fn autoSizeRow(&self, row: c_int, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRow(self.handle(), row, setAsMin) }
    }
    fn autoSizeRows(&self, setAsMin: c_int) {
        unsafe { wxGrid_AutoSizeRows(self.handle(), setAsMin) }
    }
    fn beginBatch(&self) {
        unsafe { wxGrid_BeginBatch(self.handle()) }
    }
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> wxRect {
        unsafe { wxRect { handle: wxGrid_BlockToDeviceRect(self.handle(), top, left, bottom, right) } }
    }
    fn canDragColSize(&self) -> c_int {
        unsafe { wxGrid_CanDragColSize(self.handle()) }
    }
    fn canDragGridSize(&self) -> c_int {
        unsafe { wxGrid_CanDragGridSize(self.handle()) }
    }
    fn canDragRowSize(&self) -> c_int {
        unsafe { wxGrid_CanDragRowSize(self.handle()) }
    }
    fn canEnableCellControl(&self) -> c_int {
        unsafe { wxGrid_CanEnableCellControl(self.handle()) }
    }
    fn cellToRect(&self, row: c_int, col: c_int) -> wxRect {
        unsafe { wxRect { handle: wxGrid_CellToRect(self.handle(), row, col) } }
    }
    fn clearGrid(&self) {
        unsafe { wxGrid_ClearGrid(self.handle()) }
    }
    fn clearSelection(&self) {
        unsafe { wxGrid_ClearSelection(self.handle()) }
    }
    fn newGrid(&self, rows: c_int, cols: c_int, selmode: c_int) {
        unsafe { wxGrid_CreateGrid(self.handle(), rows, cols, selmode) }
    }
    fn deleteCols(&self, pos: c_int, numCols: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_DeleteCols(self.handle(), pos, numCols, updateLabels) }
    }
    fn deleteRows(&self, pos: c_int, numRows: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_DeleteRows(self.handle(), pos, numRows, updateLabels) }
    }
    fn disableCellEditControl(&self) {
        unsafe { wxGrid_DisableCellEditControl(self.handle()) }
    }
    fn disableDragColSize(&self) {
        unsafe { wxGrid_DisableDragColSize(self.handle()) }
    }
    fn disableDragGridSize(&self) {
        unsafe { wxGrid_DisableDragGridSize(self.handle()) }
    }
    fn disableDragRowSize(&self) {
        unsafe { wxGrid_DisableDragRowSize(self.handle()) }
    }
    fn drawAllGridLines<T: _wxDC, U: _wxRegion>(&self, dc: &T, reg: &U) {
        unsafe { wxGrid_DrawAllGridLines(self.handle(), dc.handle(), reg.handle()) }
    }
    fn drawCell<T: _wxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.handle(), dc.handle(), _row, _col) }
    }
    fn drawCellBorder<T: _wxDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.handle(), dc.handle(), _row, _col) }
    }
    fn drawCellHighlight<T: _wxDC, U: _wxGridCellAttr>(&self, dc: &T, attr: &U) {
        unsafe { wxGrid_DrawCellHighlight(self.handle(), dc.handle(), attr.handle()) }
    }
    fn drawColLabel<T: _wxDC>(&self, dc: &T, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.handle(), dc.handle(), col) }
    }
    fn drawColLabels<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawColLabels(self.handle(), dc.handle()) }
    }
    fn drawGridSpace<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawGridSpace(self.handle(), dc.handle()) }
    }
    fn drawRowLabel<T: _wxDC>(&self, dc: &T, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.handle(), dc.handle(), row) }
    }
    fn drawRowLabels<T: _wxDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawRowLabels(self.handle(), dc.handle()) }
    }
    fn drawTextRectangle<T: _wxDC>(&self, dc: &T, txt: &str, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
        let txt = wxT(txt);
        unsafe { wxGrid_DrawTextRectangle(self.handle(), dc.handle(), txt.handle(), x, y, w, h, horizontalAlignment, verticalAlignment) }
    }
    fn enableCellEditControl(&self, enable: c_int) {
        unsafe { wxGrid_EnableCellEditControl(self.handle(), enable) }
    }
    fn enableDragColSize(&self, enable: c_int) {
        unsafe { wxGrid_EnableDragColSize(self.handle(), enable) }
    }
    fn enableDragGridSize(&self, enable: c_int) {
        unsafe { wxGrid_EnableDragGridSize(self.handle(), enable) }
    }
    fn enableDragRowSize(&self, enable: c_int) {
        unsafe { wxGrid_EnableDragRowSize(self.handle(), enable) }
    }
    fn enableEditing(&self, edit: c_int) {
        unsafe { wxGrid_EnableEditing(self.handle(), edit) }
    }
    fn enableGridLines(&self, enable: c_int) {
        unsafe { wxGrid_EnableGridLines(self.handle(), enable) }
    }
    fn endBatch(&self) {
        unsafe { wxGrid_EndBatch(self.handle()) }
    }
    fn getBatchCount(&self) -> c_int {
        unsafe { wxGrid_GetBatchCount(self.handle()) }
    }
    fn getCellAlignment(&self, row: c_int, col: c_int, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetCellAlignment(self.handle(), row, col, horiz, vert) }
    }
    fn getCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    fn getCellEditor(&self, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { handle: wxGrid_GetCellEditor(self.handle(), row, col) } }
    }
    fn getCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_GetCellFont(self.handle(), row, col, font.handle()) }
    }
    fn getCellHighlightColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetCellHighlightColour(self.handle(), _ref.handle()) }
    }
    fn getCellRenderer(&self, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { handle: wxGrid_GetCellRenderer(self.handle(), row, col) } }
    }
    fn getCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    fn getCellValue(&self, row: c_int, col: c_int) -> ~str {
        unsafe { wxString { handle: wxGrid_GetCellValue(self.handle(), row, col) }.to_str() }
    }
    fn getColLabelAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetColLabelAlignment(self.handle(), horiz, vert) }
    }
    fn getColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetColLabelSize(self.handle()) }
    }
    fn getColLabelValue(&self, col: c_int) -> ~str {
        unsafe { wxString { handle: wxGrid_GetColLabelValue(self.handle(), col) }.to_str() }
    }
    fn getColSize(&self, col: c_int) -> c_int {
        unsafe { wxGrid_GetColSize(self.handle(), col) }
    }
    fn getDefaultCellAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetDefaultCellAlignment(self.handle(), horiz, vert) }
    }
    fn getDefaultCellBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.handle(), _ref.handle()) }
    }
    fn getDefaultCellFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellFont(self.handle(), _ref.handle()) }
    }
    fn getDefaultCellTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellTextColour(self.handle(), _ref.handle()) }
    }
    fn getDefaultColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColLabelSize(self.handle()) }
    }
    fn getDefaultColSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColSize(self.handle()) }
    }
    fn getDefaultEditor(&self) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { handle: wxGrid_GetDefaultEditor(self.handle()) } }
    }
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { handle: wxGrid_GetDefaultEditorForCell(self.handle(), row, col) } }
    }
    fn getDefaultEditorForType(&self, typeName: &str) -> wxGridCellEditor {
        let typeName = wxT(typeName);
        unsafe { wxGridCellEditor { handle: wxGrid_GetDefaultEditorForType(self.handle(), typeName.handle()) } }
    }
    fn getDefaultRenderer(&self) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { handle: wxGrid_GetDefaultRenderer(self.handle()) } }
    }
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { handle: wxGrid_GetDefaultRendererForCell(self.handle(), row, col) } }
    }
    fn getDefaultRendererForType(&self, typeName: &str) -> wxGridCellRenderer {
        let typeName = wxT(typeName);
        unsafe { wxGridCellRenderer { handle: wxGrid_GetDefaultRendererForType(self.handle(), typeName.handle()) } }
    }
    fn getDefaultRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowLabelSize(self.handle()) }
    }
    fn getDefaultRowSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultRowSize(self.handle()) }
    }
    fn getGridCursorCol(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorCol(self.handle()) }
    }
    fn getGridCursorRow(&self) -> c_int {
        unsafe { wxGrid_GetGridCursorRow(self.handle()) }
    }
    fn getGridLineColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetGridLineColour(self.handle(), _ref.handle()) }
    }
    fn getLabelBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.handle(), _ref.handle()) }
    }
    fn getLabelFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelFont(self.handle(), _ref.handle()) }
    }
    fn getLabelTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelTextColour(self.handle(), _ref.handle()) }
    }
    fn getNumberCols(&self) -> c_int {
        unsafe { wxGrid_GetNumberCols(self.handle()) }
    }
    fn getNumberRows(&self) -> c_int {
        unsafe { wxGrid_GetNumberRows(self.handle()) }
    }
    fn getRowLabelAlignment(&self, horiz: *mut c_int, vert: *mut c_int) {
        unsafe { wxGrid_GetRowLabelAlignment(self.handle(), horiz, vert) }
    }
    fn getRowLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetRowLabelSize(self.handle()) }
    }
    fn getRowLabelValue(&self, row: c_int) -> ~str {
        unsafe { wxString { handle: wxGrid_GetRowLabelValue(self.handle(), row) }.to_str() }
    }
    fn getRowSize(&self, row: c_int) -> c_int {
        unsafe { wxGrid_GetRowSize(self.handle(), row) }
    }
    fn getSelectionBackground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionBackground(self.handle(), _ref.handle()) }
    }
    fn getSelectionForeground<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionForeground(self.handle(), _ref.handle()) }
    }
    fn getTable(&self) -> wxGridTableBase {
        unsafe { wxGridTableBase { handle: wxGrid_GetTable(self.handle()) } }
    }
    fn getTextBoxSize<T: _wxDC>(&self, dc: &T, count: c_int, lines: *mut *mut c_char, _w: *mut c_void, _h: *mut c_void) {
        unsafe { wxGrid_GetTextBoxSize(self.handle(), dc.handle(), count, lines, _w, _h) }
    }
    fn gridLinesEnabled(&self) -> c_int {
        unsafe { wxGrid_GridLinesEnabled(self.handle()) }
    }
    fn hideCellEditControl(&self) {
        unsafe { wxGrid_HideCellEditControl(self.handle()) }
    }
    fn insertCols(&self, pos: c_int, numCols: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_InsertCols(self.handle(), pos, numCols, updateLabels) }
    }
    fn insertRows(&self, pos: c_int, numRows: c_int, updateLabels: c_int) -> c_int {
        unsafe { wxGrid_InsertRows(self.handle(), pos, numRows, updateLabels) }
    }
    fn isCellEditControlEnabled(&self) -> c_int {
        unsafe { wxGrid_IsCellEditControlEnabled(self.handle()) }
    }
    fn isCellEditControlShown(&self) -> c_int {
        unsafe { wxGrid_IsCellEditControlShown(self.handle()) }
    }
    fn isCurrentCellReadOnly(&self) -> c_int {
        unsafe { wxGrid_IsCurrentCellReadOnly(self.handle()) }
    }
    fn isEditable(&self) -> c_int {
        unsafe { wxGrid_IsEditable(self.handle()) }
    }
    fn isInSelection(&self, row: c_int, col: c_int) -> c_int {
        unsafe { wxGrid_IsInSelection(self.handle(), row, col) }
    }
    fn isReadOnly(&self, row: c_int, col: c_int) -> c_int {
        unsafe { wxGrid_IsReadOnly(self.handle(), row, col) }
    }
    fn isSelection(&self) -> c_int {
        unsafe { wxGrid_IsSelection(self.handle()) }
    }
    fn isVisible(&self, row: c_int, col: c_int, wholeCellVisible: c_int) -> c_int {
        unsafe { wxGrid_IsVisible(self.handle(), row, col, wholeCellVisible) }
    }
    fn makeCellVisible(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_MakeCellVisible(self.handle(), row, col) }
    }
    fn moveCursorDown(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorDown(self.handle(), expandSelection) }
    }
    fn moveCursorDownBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorDownBlock(self.handle(), expandSelection) }
    }
    fn moveCursorLeft(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorLeft(self.handle(), expandSelection) }
    }
    fn moveCursorLeftBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorLeftBlock(self.handle(), expandSelection) }
    }
    fn moveCursorRight(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorRight(self.handle(), expandSelection) }
    }
    fn moveCursorRightBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorRightBlock(self.handle(), expandSelection) }
    }
    fn moveCursorUp(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorUp(self.handle(), expandSelection) }
    }
    fn moveCursorUpBlock(&self, expandSelection: c_int) -> c_int {
        unsafe { wxGrid_MoveCursorUpBlock(self.handle(), expandSelection) }
    }
    fn movePageDown(&self) -> c_int {
        unsafe { wxGrid_MovePageDown(self.handle()) }
    }
    fn movePageUp(&self) -> c_int {
        unsafe { wxGrid_MovePageUp(self.handle()) }
    }
    fn processTableMessage<T: _wxEvent>(&self, evt: &T) -> c_int {
        unsafe { wxGrid_ProcessTableMessage(self.handle(), evt.handle()) }
    }
    fn registerDataType<T: _wxGridCellRenderer, U: _wxGridCellEditor>(&self, typeName: &str, renderer: &T, editor: &U) {
        let typeName = wxT(typeName);
        unsafe { wxGrid_RegisterDataType(self.handle(), typeName.handle(), renderer.handle(), editor.handle()) }
    }
    fn saveEditControlValue(&self) {
        unsafe { wxGrid_SaveEditControlValue(self.handle()) }
    }
    fn selectAll(&self) {
        unsafe { wxGrid_SelectAll(self.handle()) }
    }
    fn selectBlock(&self, topRow: c_int, leftCol: c_int, bottomRow: c_int, rightCol: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectBlock(self.handle(), topRow, leftCol, bottomRow, rightCol, addToSelected) }
    }
    fn selectCol(&self, col: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectCol(self.handle(), col, addToSelected) }
    }
    fn selectRow(&self, row: c_int, addToSelected: c_int) {
        unsafe { wxGrid_SelectRow(self.handle(), row, addToSelected) }
    }
    fn setCellAlignment(&self, row: c_int, col: c_int, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetCellAlignment(self.handle(), row, col, horiz, vert) }
    }
    fn setCellBackgroundColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellBackgroundColour(self.handle(), row, col, colour.handle()) }
    }
    fn setCellEditor<T: _wxGridCellEditor>(&self, row: c_int, col: c_int, editor: &T) {
        unsafe { wxGrid_SetCellEditor(self.handle(), row, col, editor.handle()) }
    }
    fn setCellFont<T: _wxFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_SetCellFont(self.handle(), row, col, font.handle()) }
    }
    fn setCellHighlightColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxGrid_SetCellHighlightColour(self.handle(), col.handle()) }
    }
    fn setCellRenderer<T: _wxGridCellRenderer>(&self, row: c_int, col: c_int, renderer: &T) {
        unsafe { wxGrid_SetCellRenderer(self.handle(), row, col, renderer.handle()) }
    }
    fn setCellTextColour<T: _wxColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellTextColour(self.handle(), row, col, colour.handle()) }
    }
    fn setCellValue(&self, row: c_int, col: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxGrid_SetCellValue(self.handle(), row, col, s.handle()) }
    }
    fn setColAttr<T: _wxGridCellAttr>(&self, col: c_int, attr: &T) {
        unsafe { wxGrid_SetColAttr(self.handle(), col, attr.handle()) }
    }
    fn setColFormatBool(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatBool(self.handle(), col) }
    }
    fn setColFormatCustom(&self, col: c_int, typeName: &str) {
        let typeName = wxT(typeName);
        unsafe { wxGrid_SetColFormatCustom(self.handle(), col, typeName.handle()) }
    }
    fn setColFormatFloat(&self, col: c_int, width: c_int, precision: c_int) {
        unsafe { wxGrid_SetColFormatFloat(self.handle(), col, width, precision) }
    }
    fn setColFormatNumber(&self, col: c_int) {
        unsafe { wxGrid_SetColFormatNumber(self.handle(), col) }
    }
    fn setColLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetColLabelAlignment(self.handle(), horiz, vert) }
    }
    fn setColLabelSize(&self, height: c_int) {
        unsafe { wxGrid_SetColLabelSize(self.handle(), height) }
    }
    fn setColLabelValue(&self, col: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxGrid_SetColLabelValue(self.handle(), col, label.handle()) }
    }
    fn setColMinimalWidth(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColMinimalWidth(self.handle(), col, width) }
    }
    fn setColSize(&self, col: c_int, width: c_int) {
        unsafe { wxGrid_SetColSize(self.handle(), col, width) }
    }
    fn setDefaultCellAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetDefaultCellAlignment(self.handle(), horiz, vert) }
    }
    fn setDefaultCellBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.handle(), colour.handle()) }
    }
    fn setDefaultCellFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGrid_SetDefaultCellFont(self.handle(), font.handle()) }
    }
    fn setDefaultCellTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.handle(), colour.handle()) }
    }
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.handle(), width, resizeExistingCols) }
    }
    fn setDefaultEditor<T: _wxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGrid_SetDefaultEditor(self.handle(), editor.handle()) }
    }
    fn setDefaultRenderer<T: _wxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGrid_SetDefaultRenderer(self.handle(), renderer.handle()) }
    }
    fn setDefaultRowSize(&self, height: c_int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self.handle(), height, resizeExistingRows) }
    }
    fn setGridCursor(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_SetGridCursor(self.handle(), row, col) }
    }
    fn setGridLineColour<T: _wxColour>(&self, col: &T) {
        unsafe { wxGrid_SetGridLineColour(self.handle(), col.handle()) }
    }
    fn setLabelBackgroundColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.handle(), colour.handle()) }
    }
    fn setLabelFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGrid_SetLabelFont(self.handle(), font.handle()) }
    }
    fn setLabelTextColour<T: _wxColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelTextColour(self.handle(), colour.handle()) }
    }
    fn setMargins(&self, extraWidth: c_int, extraHeight: c_int) {
        unsafe { wxGrid_SetMargins(self.handle(), extraWidth, extraHeight) }
    }
    fn setReadOnly(&self, row: c_int, col: c_int, isReadOnly: c_int) {
        unsafe { wxGrid_SetReadOnly(self.handle(), row, col, isReadOnly) }
    }
    fn setRowAttr<T: _wxGridCellAttr>(&self, row: c_int, attr: &T) {
        unsafe { wxGrid_SetRowAttr(self.handle(), row, attr.handle()) }
    }
    fn setRowLabelAlignment(&self, horiz: c_int, vert: c_int) {
        unsafe { wxGrid_SetRowLabelAlignment(self.handle(), horiz, vert) }
    }
    fn setRowLabelSize(&self, width: c_int) {
        unsafe { wxGrid_SetRowLabelSize(self.handle(), width) }
    }
    fn setRowLabelValue(&self, row: c_int, label: &str) {
        let label = wxT(label);
        unsafe { wxGrid_SetRowLabelValue(self.handle(), row, label.handle()) }
    }
    fn setRowMinimalHeight(&self, row: c_int, width: c_int) {
        unsafe { wxGrid_SetRowMinimalHeight(self.handle(), row, width) }
    }
    fn setRowSize(&self, row: c_int, height: c_int) {
        unsafe { wxGrid_SetRowSize(self.handle(), row, height) }
    }
    fn setSelectionBackground<T: _wxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionBackground(self.handle(), c.handle()) }
    }
    fn setSelectionForeground<T: _wxColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionForeground(self.handle(), c.handle()) }
    }
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.handle(), selmode) }
    }
    fn setTable<T: _wxGridTableBase>(&self, table: &T, takeOwnership: c_int, selmode: c_int) -> c_int {
        unsafe { wxGrid_SetTable(self.handle(), table.handle(), takeOwnership, selmode) }
    }
    fn showCellEditControl(&self) {
        unsafe { wxGrid_ShowCellEditControl(self.handle()) }
    }
    fn stringToLines(&self, value: &str, lines: *mut c_void) -> c_int {
        let value = wxT(value);
        unsafe { wxGrid_StringToLines(self.handle(), value.handle(), lines) }
    }
    fn xToCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToCol(self.handle(), x) }
    }
    fn xToEdgeOfCol(&self, x: c_int) -> c_int {
        unsafe { wxGrid_XToEdgeOfCol(self.handle(), x) }
    }
    fn xYToCell(&self, x: c_int, y: c_int, row: *mut c_int, col: *mut c_int) {
        unsafe { wxGrid_XYToCell(self.handle(), x, y, row, col) }
    }
    fn yToEdgeOfRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToEdgeOfRow(self.handle(), y) }
    }
    fn yToRow(&self, y: c_int) -> c_int {
        unsafe { wxGrid_YToRow(self.handle(), y) }
    }
    fn getSelectedCells<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectedCells(self.handle(), _arr.handle()) }
    }
    fn getSelectionBlockTopLeft<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.handle(), _arr.handle()) }
    }
    fn getSelectionBlockBottomRight<T: _wxGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockBottomRight(self.handle(), _arr.handle()) }
    }
    fn getSelectedRows(&self, _arr: *mut c_void) -> c_int {
        unsafe { wxGrid_GetSelectedRows(self.handle(), _arr) }
    }
    fn getSelectedCols(&self, _arr: *mut c_void) -> c_int {
        unsafe { wxGrid_GetSelectedCols(self.handle(), _arr) }
    }
    fn getCellSize(&self, row: c_int, col: c_int, srow: *mut c_int, scol: *mut c_int) {
        unsafe { wxGrid_GetCellSize(self.handle(), row, col, srow, scol) }
    }
    fn setCellSize(&self, row: c_int, col: c_int, srow: c_int, scol: c_int) {
        unsafe { wxGrid_SetCellSize(self.handle(), row, col, srow, scol) }
    }
}

pub struct wxGridCellAttr { handle: *mut c_void }
impl _wxGridCellAttr for wxGridCellAttr { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellAttr {
    pub fn from(handle: *mut c_void) -> wxGridCellAttr { wxGridCellAttr { handle: handle } }
    pub fn null() -> wxGridCellAttr { wxGridCellAttr::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellAttr {
        unsafe { wxGridCellAttr { handle: wxGridCellAttr_Ctor() } }
    }
}

pub trait _wxGridCellAttr {
    fn handle(&self) -> *mut c_void;
    
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.handle()) }
    }
    fn getAlignment(&self, hAlign: *mut c_int, vAlign: *mut c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.handle(), hAlign, vAlign) }
    }
    fn getBackgroundColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.handle(), _ref.handle()) }
    }
    fn getEditor<T: _wxGrid>(&self, grid: &T, row: c_int, col: c_int) -> wxGridCellEditor {
        unsafe { wxGridCellEditor { handle: wxGridCellAttr_GetEditor(self.handle(), grid.handle(), row, col) } }
    }
    fn getFont<T: _wxFont>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetFont(self.handle(), _ref.handle()) }
    }
    fn getRenderer<T: _wxGrid>(&self, grid: &T, row: c_int, col: c_int) -> wxGridCellRenderer {
        unsafe { wxGridCellRenderer { handle: wxGridCellAttr_GetRenderer(self.handle(), grid.handle(), row, col) } }
    }
    fn getTextColour<T: _wxColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetTextColour(self.handle(), _ref.handle()) }
    }
    fn hasAlignment(&self) -> c_int {
        unsafe { wxGridCellAttr_HasAlignment(self.handle()) }
    }
    fn hasBackgroundColour(&self) -> c_int {
        unsafe { wxGridCellAttr_HasBackgroundColour(self.handle()) }
    }
    fn hasEditor(&self) -> c_int {
        unsafe { wxGridCellAttr_HasEditor(self.handle()) }
    }
    fn hasFont(&self) -> c_int {
        unsafe { wxGridCellAttr_HasFont(self.handle()) }
    }
    fn hasRenderer(&self) -> c_int {
        unsafe { wxGridCellAttr_HasRenderer(self.handle()) }
    }
    fn hasTextColour(&self) -> c_int {
        unsafe { wxGridCellAttr_HasTextColour(self.handle()) }
    }
    fn incRef(&self) {
        unsafe { wxGridCellAttr_IncRef(self.handle()) }
    }
    fn isReadOnly(&self) -> c_int {
        unsafe { wxGridCellAttr_IsReadOnly(self.handle()) }
    }
    fn setAlignment(&self, hAlign: c_int, vAlign: c_int) {
        unsafe { wxGridCellAttr_SetAlignment(self.handle(), hAlign, vAlign) }
    }
    fn setBackgroundColour<T: _wxColour>(&self, colBack: &T) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.handle(), colBack.handle()) }
    }
    fn setDefAttr<T: _wxGridCellAttr>(&self, defAttr: &T) {
        unsafe { wxGridCellAttr_SetDefAttr(self.handle(), defAttr.handle()) }
    }
    fn setEditor<T: _wxGridCellEditor>(&self, editor: &T) {
        unsafe { wxGridCellAttr_SetEditor(self.handle(), editor.handle()) }
    }
    fn setFont<T: _wxFont>(&self, font: &T) {
        unsafe { wxGridCellAttr_SetFont(self.handle(), font.handle()) }
    }
    fn setReadOnly(&self, isReadOnly: c_int) {
        unsafe { wxGridCellAttr_SetReadOnly(self.handle(), isReadOnly) }
    }
    fn setRenderer<T: _wxGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGridCellAttr_SetRenderer(self.handle(), renderer.handle()) }
    }
    fn setTextColour<T: _wxColour>(&self, colText: &T) {
        unsafe { wxGridCellAttr_SetTextColour(self.handle(), colText.handle()) }
    }
}

pub struct wxGridCellBoolEditor { handle: *mut c_void }
impl _wxGridCellBoolEditor for wxGridCellBoolEditor {}
impl _wxGridCellEditor for wxGridCellBoolEditor {}
impl _wxGridCellWorker for wxGridCellBoolEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellBoolEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellBoolEditor { wxGridCellBoolEditor { handle: handle } }
    pub fn null() -> wxGridCellBoolEditor { wxGridCellBoolEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellBoolEditor {
        unsafe { wxGridCellBoolEditor { handle: wxGridCellBoolEditor_Ctor() } }
    }
}

pub trait _wxGridCellBoolEditor : _wxGridCellEditor {
}

pub struct wxGridCellBoolRenderer { handle: *mut c_void }
impl _wxGridCellBoolRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellRenderer for wxGridCellBoolRenderer {}
impl _wxGridCellWorker for wxGridCellBoolRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellBoolRenderer {
    pub fn from(handle: *mut c_void) -> wxGridCellBoolRenderer { wxGridCellBoolRenderer { handle: handle } }
    pub fn null() -> wxGridCellBoolRenderer { wxGridCellBoolRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellBoolRenderer : _wxGridCellRenderer {
}

pub struct wxGridCellChoiceEditor { handle: *mut c_void }
impl _wxGridCellChoiceEditor for wxGridCellChoiceEditor {}
impl _wxGridCellEditor for wxGridCellChoiceEditor {}
impl _wxGridCellWorker for wxGridCellChoiceEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellChoiceEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellChoiceEditor { wxGridCellChoiceEditor { handle: handle } }
    pub fn null() -> wxGridCellChoiceEditor { wxGridCellChoiceEditor::from(0 as *mut c_void) }
    
    pub fn ctor(count: c_int, choices: *mut *mut c_char, allowOthers: c_int) -> wxGridCellChoiceEditor {
        unsafe { wxGridCellChoiceEditor { handle: wxGridCellChoiceEditor_Ctor(count, choices, allowOthers) } }
    }
}

pub trait _wxGridCellChoiceEditor : _wxGridCellEditor {
}

pub struct wxGridCellCoordsArray { handle: *mut c_void }
impl _wxGridCellCoordsArray for wxGridCellCoordsArray { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellCoordsArray {
    pub fn from(handle: *mut c_void) -> wxGridCellCoordsArray { wxGridCellCoordsArray { handle: handle } }
    pub fn null() -> wxGridCellCoordsArray { wxGridCellCoordsArray::from(0 as *mut c_void) }
    
    pub fn new() -> wxGridCellCoordsArray {
        unsafe { wxGridCellCoordsArray { handle: wxGridCellCoordsArray_Create() } }
    }
}

pub trait _wxGridCellCoordsArray {
    fn handle(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxGridCellCoordsArray_Delete(self.handle()) }
    }
    fn getCount(&self) -> c_int {
        unsafe { wxGridCellCoordsArray_GetCount(self.handle()) }
    }
    fn item(&self, _idx: c_int, _c: *mut c_int, _r: *mut c_int) {
        unsafe { wxGridCellCoordsArray_Item(self.handle(), _idx, _c, _r) }
    }
}

pub struct wxGridCellEditor { handle: *mut c_void }
impl _wxGridCellEditor for wxGridCellEditor {}
impl _wxGridCellWorker for wxGridCellEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellEditor { wxGridCellEditor { handle: handle } }
    pub fn null() -> wxGridCellEditor { wxGridCellEditor::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellEditor : _wxGridCellWorker {
    fn beginEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: &T) {
        unsafe { wxGridCellEditor_BeginEdit(self.handle(), row, col, grid.handle()) }
    }
    fn new<T: _wxWindow, U: _wxEvtHandler>(&self, parent: &T, id: c_int, evtHandler: &U) {
        unsafe { wxGridCellEditor_Create(self.handle(), parent.handle(), id, evtHandler.handle()) }
    }
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.handle()) }
    }
    fn endEdit<T: _wxGrid>(&self, row: c_int, col: c_int, grid: &T, oldStr: &str, newStr: &str) -> c_int {
        let oldStr = wxT(oldStr);
        let newStr = wxT(newStr);
        unsafe { wxGridCellEditor_EndEdit(self.handle(), row, col, grid.handle(), oldStr.handle(), newStr.handle()) }
    }
    fn getControl(&self) -> wxControl {
        unsafe { wxControl { handle: wxGridCellEditor_GetControl(self.handle()) } }
    }
    fn handleReturn<T: _wxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_HandleReturn(self.handle(), event.handle()) }
    }
    fn isAcceptedKey<T: _wxEvent>(&self, event: &T) -> c_int {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.handle(), event.handle()) }
    }
    fn isCreated(&self) -> c_int {
        unsafe { wxGridCellEditor_IsCreated(self.handle()) }
    }
    fn paintBackground<T: _wxDC, U: _wxGridCellAttr>(&self, dc: &T, x: c_int, y: c_int, w: c_int, h: c_int, attr: &U) {
        unsafe { wxGridCellEditor_PaintBackground(self.handle(), dc.handle(), x, y, w, h, attr.handle()) }
    }
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.handle()) }
    }
    fn setControl<T: _wxControl>(&self, control: &T) {
        unsafe { wxGridCellEditor_SetControl(self.handle(), control.handle()) }
    }
    fn setParameters(&self, params: &str) {
        let params = wxT(params);
        unsafe { wxGridCellEditor_SetParameters(self.handle(), params.handle()) }
    }
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.handle(), x, y, w, h) }
    }
    fn show<T: _wxGridCellAttr>(&self, show: c_int, attr: &T) {
        unsafe { wxGridCellEditor_Show(self.handle(), show, attr.handle()) }
    }
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.handle()) }
    }
    fn startingKey<T: _wxEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_StartingKey(self.handle(), event.handle()) }
    }
}

pub struct wxGridCellFloatEditor { handle: *mut c_void }
impl _wxGridCellFloatEditor for wxGridCellFloatEditor {}
impl _wxGridCellTextEditor for wxGridCellFloatEditor {}
impl _wxGridCellEditor for wxGridCellFloatEditor {}
impl _wxGridCellWorker for wxGridCellFloatEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellFloatEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellFloatEditor { wxGridCellFloatEditor { handle: handle } }
    pub fn null() -> wxGridCellFloatEditor { wxGridCellFloatEditor::from(0 as *mut c_void) }
    
    pub fn ctor(width: c_int, precision: c_int) -> wxGridCellFloatEditor {
        unsafe { wxGridCellFloatEditor { handle: wxGridCellFloatEditor_Ctor(width, precision) } }
    }
}

pub trait _wxGridCellFloatEditor : _wxGridCellTextEditor {
}

pub struct wxGridCellFloatRenderer { handle: *mut c_void }
impl _wxGridCellFloatRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellStringRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellRenderer for wxGridCellFloatRenderer {}
impl _wxGridCellWorker for wxGridCellFloatRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellFloatRenderer {
    pub fn from(handle: *mut c_void) -> wxGridCellFloatRenderer { wxGridCellFloatRenderer { handle: handle } }
    pub fn null() -> wxGridCellFloatRenderer { wxGridCellFloatRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellFloatRenderer : _wxGridCellStringRenderer {
}

pub struct wxGridCellNumberEditor { handle: *mut c_void }
impl _wxGridCellNumberEditor for wxGridCellNumberEditor {}
impl _wxGridCellTextEditor for wxGridCellNumberEditor {}
impl _wxGridCellEditor for wxGridCellNumberEditor {}
impl _wxGridCellWorker for wxGridCellNumberEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellNumberEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellNumberEditor { wxGridCellNumberEditor { handle: handle } }
    pub fn null() -> wxGridCellNumberEditor { wxGridCellNumberEditor::from(0 as *mut c_void) }
    
    pub fn ctor(min: c_int, max: c_int) -> wxGridCellNumberEditor {
        unsafe { wxGridCellNumberEditor { handle: wxGridCellNumberEditor_Ctor(min, max) } }
    }
}

pub trait _wxGridCellNumberEditor : _wxGridCellTextEditor {
}

pub struct wxGridCellNumberRenderer { handle: *mut c_void }
impl _wxGridCellNumberRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellStringRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellRenderer for wxGridCellNumberRenderer {}
impl _wxGridCellWorker for wxGridCellNumberRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellNumberRenderer {
    pub fn from(handle: *mut c_void) -> wxGridCellNumberRenderer { wxGridCellNumberRenderer { handle: handle } }
    pub fn null() -> wxGridCellNumberRenderer { wxGridCellNumberRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellNumberRenderer {
        unsafe { wxGridCellNumberRenderer { handle: wxGridCellNumberRenderer_Ctor() } }
    }
}

pub trait _wxGridCellNumberRenderer : _wxGridCellStringRenderer {
}

pub struct wxGridCellAutoWrapStringRenderer { handle: *mut c_void }
impl _wxGridCellAutoWrapStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellStringRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellRenderer for wxGridCellAutoWrapStringRenderer {}
impl _wxGridCellWorker for wxGridCellAutoWrapStringRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellAutoWrapStringRenderer {
    pub fn from(handle: *mut c_void) -> wxGridCellAutoWrapStringRenderer { wxGridCellAutoWrapStringRenderer { handle: handle } }
    pub fn null() -> wxGridCellAutoWrapStringRenderer { wxGridCellAutoWrapStringRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellAutoWrapStringRenderer {
        unsafe { wxGridCellAutoWrapStringRenderer { handle: wxGridCellAutoWrapStringRenderer_Ctor() } }
    }
}

pub trait _wxGridCellAutoWrapStringRenderer : _wxGridCellStringRenderer {
}

pub struct wxGridCellRenderer { handle: *mut c_void }
impl _wxGridCellRenderer for wxGridCellRenderer {}
impl _wxGridCellWorker for wxGridCellRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellRenderer {
    pub fn from(handle: *mut c_void) -> wxGridCellRenderer { wxGridCellRenderer { handle: handle } }
    pub fn null() -> wxGridCellRenderer { wxGridCellRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellRenderer : _wxGridCellWorker {
}

pub struct wxGridCellStringRenderer { handle: *mut c_void }
impl _wxGridCellStringRenderer for wxGridCellStringRenderer {}
impl _wxGridCellRenderer for wxGridCellStringRenderer {}
impl _wxGridCellWorker for wxGridCellStringRenderer { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellStringRenderer {
    pub fn from(handle: *mut c_void) -> wxGridCellStringRenderer { wxGridCellStringRenderer { handle: handle } }
    pub fn null() -> wxGridCellStringRenderer { wxGridCellStringRenderer::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellStringRenderer : _wxGridCellRenderer {
}

pub struct wxGridCellTextEditor { handle: *mut c_void }
impl _wxGridCellTextEditor for wxGridCellTextEditor {}
impl _wxGridCellEditor for wxGridCellTextEditor {}
impl _wxGridCellWorker for wxGridCellTextEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellTextEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellTextEditor { wxGridCellTextEditor { handle: handle } }
    pub fn null() -> wxGridCellTextEditor { wxGridCellTextEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellTextEditor {
        unsafe { wxGridCellTextEditor { handle: wxGridCellTextEditor_Ctor() } }
    }
}

pub trait _wxGridCellTextEditor : _wxGridCellEditor {
}

pub struct wxGridCellWorker { handle: *mut c_void }
impl _wxGridCellWorker for wxGridCellWorker { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellWorker {
    pub fn from(handle: *mut c_void) -> wxGridCellWorker { wxGridCellWorker { handle: handle } }
    pub fn null() -> wxGridCellWorker { wxGridCellWorker::from(0 as *mut c_void) }
    
}

pub trait _wxGridCellWorker {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxGridEditorCreatedEvent { handle: *mut c_void }
impl _wxGridEditorCreatedEvent for wxGridEditorCreatedEvent {}
impl _wxCommandEvent for wxGridEditorCreatedEvent {}
impl _wxEvent for wxGridEditorCreatedEvent {}
impl _wxObject for wxGridEditorCreatedEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridEditorCreatedEvent {
    pub fn from(handle: *mut c_void) -> wxGridEditorCreatedEvent { wxGridEditorCreatedEvent { handle: handle } }
    pub fn null() -> wxGridEditorCreatedEvent { wxGridEditorCreatedEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridEditorCreatedEvent : _wxCommandEvent {
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.handle()) }
    }
    fn getControl(&self) -> wxControl {
        unsafe { wxControl { handle: wxGridEditorCreatedEvent_GetControl(self.handle()) } }
    }
    fn getRow(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self.handle()) }
    }
    fn setCol(&self, col: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self.handle(), col) }
    }
    fn setControl<T: _wxControl>(&self, ctrl: &T) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.handle(), ctrl.handle()) }
    }
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.handle(), row) }
    }
}

pub struct wxGridEvent { handle: *mut c_void }
impl _wxGridEvent for wxGridEvent {}
impl _wxNotifyEvent for wxGridEvent {}
impl _wxCommandEvent for wxGridEvent {}
impl _wxEvent for wxGridEvent {}
impl _wxObject for wxGridEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridEvent {
    pub fn from(handle: *mut c_void) -> wxGridEvent { wxGridEvent { handle: handle } }
    pub fn null() -> wxGridEvent { wxGridEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridEvent : _wxNotifyEvent {
    fn altDown(&self) -> c_int {
        unsafe { wxGridEvent_AltDown(self.handle()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridEvent_ControlDown(self.handle()) }
    }
    fn getCol(&self) -> c_int {
        unsafe { wxGridEvent_GetCol(self.handle()) }
    }
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { handle: wxGridEvent_GetPosition(self.handle()) } }
    }
    fn getRow(&self) -> c_int {
        unsafe { wxGridEvent_GetRow(self.handle()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxGridEvent_MetaDown(self.handle()) }
    }
    fn selecting(&self) -> c_int {
        unsafe { wxGridEvent_Selecting(self.handle()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxGridEvent_ShiftDown(self.handle()) }
    }
}

pub struct wxGridRangeSelectEvent { handle: *mut c_void }
impl _wxGridRangeSelectEvent for wxGridRangeSelectEvent {}
impl _wxNotifyEvent for wxGridRangeSelectEvent {}
impl _wxCommandEvent for wxGridRangeSelectEvent {}
impl _wxEvent for wxGridRangeSelectEvent {}
impl _wxObject for wxGridRangeSelectEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridRangeSelectEvent {
    pub fn from(handle: *mut c_void) -> wxGridRangeSelectEvent { wxGridRangeSelectEvent { handle: handle } }
    pub fn null() -> wxGridRangeSelectEvent { wxGridRangeSelectEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridRangeSelectEvent : _wxNotifyEvent {
    fn getTopLeftCoords(&self, col: *mut c_void, row: *mut c_void) {
        unsafe { wxGridRangeSelectEvent_GetTopLeftCoords(self.handle(), col, row) }
    }
    fn getBottomRightCoords(&self, col: *mut c_void, row: *mut c_void) {
        unsafe { wxGridRangeSelectEvent_GetBottomRightCoords(self.handle(), col, row) }
    }
    fn getTopRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetTopRow(self.handle()) }
    }
    fn getBottomRow(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetBottomRow(self.handle()) }
    }
    fn getLeftCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetLeftCol(self.handle()) }
    }
    fn getRightCol(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_GetRightCol(self.handle()) }
    }
    fn selecting(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_Selecting(self.handle()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_ControlDown(self.handle()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_MetaDown(self.handle()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_ShiftDown(self.handle()) }
    }
    fn altDown(&self) -> c_int {
        unsafe { wxGridRangeSelectEvent_AltDown(self.handle()) }
    }
}

pub struct wxGridSizeEvent { handle: *mut c_void }
impl _wxGridSizeEvent for wxGridSizeEvent {}
impl _wxNotifyEvent for wxGridSizeEvent {}
impl _wxCommandEvent for wxGridSizeEvent {}
impl _wxEvent for wxGridSizeEvent {}
impl _wxObject for wxGridSizeEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridSizeEvent {
    pub fn from(handle: *mut c_void) -> wxGridSizeEvent { wxGridSizeEvent { handle: handle } }
    pub fn null() -> wxGridSizeEvent { wxGridSizeEvent::from(0 as *mut c_void) }
    
}

pub trait _wxGridSizeEvent : _wxNotifyEvent {
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.handle()) }
    }
    fn getPosition(&self) -> wxPoint {
        unsafe { wxPoint { handle: wxGridSizeEvent_GetPosition(self.handle()) } }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_ControlDown(self.handle()) }
    }
    fn metaDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_MetaDown(self.handle()) }
    }
    fn shiftDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_ShiftDown(self.handle()) }
    }
    fn altDown(&self) -> c_int {
        unsafe { wxGridSizeEvent_AltDown(self.handle()) }
    }
}

pub struct wxGridTableBase { handle: *mut c_void }
impl _wxGridTableBase for wxGridTableBase {}
impl _wxObject for wxGridTableBase { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridTableBase {
    pub fn from(handle: *mut c_void) -> wxGridTableBase { wxGridTableBase { handle: handle } }
    pub fn null() -> wxGridTableBase { wxGridTableBase::from(0 as *mut c_void) }
    
}

pub trait _wxGridTableBase : _wxObject {
}

pub struct wxJoystick { handle: *mut c_void }
impl _wxJoystick for wxJoystick {}
impl _wxObject for wxJoystick { fn handle(&self) -> *mut c_void { self.handle } }

impl wxJoystick {
    pub fn from(handle: *mut c_void) -> wxJoystick { wxJoystick { handle: handle } }
    pub fn null() -> wxJoystick { wxJoystick::from(0 as *mut c_void) }
    
}

pub trait _wxJoystick : _wxObject {
}

pub struct wxLayoutAlgorithm { handle: *mut c_void }
impl _wxLayoutAlgorithm for wxLayoutAlgorithm {}
impl _wxObject for wxLayoutAlgorithm { fn handle(&self) -> *mut c_void { self.handle } }

impl wxLayoutAlgorithm {
    pub fn from(handle: *mut c_void) -> wxLayoutAlgorithm { wxLayoutAlgorithm { handle: handle } }
    pub fn null() -> wxLayoutAlgorithm { wxLayoutAlgorithm::from(0 as *mut c_void) }
    
    pub fn new() -> wxLayoutAlgorithm {
        unsafe { wxLayoutAlgorithm { handle: wxLayoutAlgorithm_Create() } }
    }
}

pub trait _wxLayoutAlgorithm : _wxObject {
    fn layoutFrame<T: _wxFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.handle(), frame.handle(), mainWindow) }
    }
    fn layoutMDIFrame<T: _wxFrame>(&self, frame: &T, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.handle(), frame.handle(), x, y, w, h, use_) }
    }
    fn layoutWindow<T: _wxFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.handle(), frame.handle(), mainWindow) }
    }
}

pub struct wxQueryLayoutInfoEvent { handle: *mut c_void }
impl _wxQueryLayoutInfoEvent for wxQueryLayoutInfoEvent {}
impl _wxEvent for wxQueryLayoutInfoEvent {}
impl _wxObject for wxQueryLayoutInfoEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxQueryLayoutInfoEvent {
    pub fn from(handle: *mut c_void) -> wxQueryLayoutInfoEvent { wxQueryLayoutInfoEvent { handle: handle } }
    pub fn null() -> wxQueryLayoutInfoEvent { wxQueryLayoutInfoEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> wxQueryLayoutInfoEvent {
        unsafe { wxQueryLayoutInfoEvent { handle: wxQueryLayoutInfoEvent_Create(id) } }
    }
}

pub trait _wxQueryLayoutInfoEvent : _wxEvent {
    fn getAlignment(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetAlignment(self.handle()) }
    }
    fn getFlags(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetFlags(self.handle()) }
    }
    fn getOrientation(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetOrientation(self.handle()) }
    }
    fn getRequestedLength(&self) -> c_int {
        unsafe { wxQueryLayoutInfoEvent_GetRequestedLength(self.handle()) }
    }
    fn getSize(&self) -> wxSize {
        unsafe { wxSize { handle: wxQueryLayoutInfoEvent_GetSize(self.handle()) } }
    }
    fn setAlignment(&self, align: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetAlignment(self.handle(), align) }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetFlags(self.handle(), flags) }
    }
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetOrientation(self.handle(), orient) }
    }
    fn setRequestedLength(&self, length: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetRequestedLength(self.handle(), length) }
    }
    fn setSize(&self, w: c_int, h: c_int) {
        unsafe { wxQueryLayoutInfoEvent_SetSize(self.handle(), w, h) }
    }
}

pub struct wxSashEvent { handle: *mut c_void }
impl _wxSashEvent for wxSashEvent {}
impl _wxEvent for wxSashEvent {}
impl _wxObject for wxSashEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxSashEvent {
    pub fn from(handle: *mut c_void) -> wxSashEvent { wxSashEvent { handle: handle } }
    pub fn null() -> wxSashEvent { wxSashEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int, edge: c_int) -> wxSashEvent {
        unsafe { wxSashEvent { handle: wxSashEvent_Create(id, edge) } }
    }
}

pub trait _wxSashEvent : _wxEvent {
    fn getDragRect(&self) -> wxRect {
        unsafe { wxRect { handle: wxSashEvent_GetDragRect(self.handle()) } }
    }
    fn getDragStatus(&self) -> c_int {
        unsafe { wxSashEvent_GetDragStatus(self.handle()) }
    }
    fn getEdge(&self) -> c_int {
        unsafe { wxSashEvent_GetEdge(self.handle()) }
    }
    fn setDragRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxSashEvent_SetDragRect(self.handle(), x, y, w, h) }
    }
    fn setDragStatus(&self, status: c_int) {
        unsafe { wxSashEvent_SetDragStatus(self.handle(), status) }
    }
    fn setEdge(&self, edge: c_int) {
        unsafe { wxSashEvent_SetEdge(self.handle(), edge) }
    }
}

pub struct wxSashLayoutWindow { handle: *mut c_void }
impl _wxSashLayoutWindow for wxSashLayoutWindow {}
impl _wxSashWindow for wxSashLayoutWindow {}
impl _wxWindow for wxSashLayoutWindow {}
impl _wxEvtHandler for wxSashLayoutWindow {}
impl _wxObject for wxSashLayoutWindow { fn handle(&self) -> *mut c_void { self.handle } }

impl wxSashLayoutWindow {
    pub fn from(handle: *mut c_void) -> wxSashLayoutWindow { wxSashLayoutWindow { handle: handle } }
    pub fn null() -> wxSashLayoutWindow { wxSashLayoutWindow::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxSashLayoutWindow {
        unsafe { wxSashLayoutWindow { handle: wxSashLayoutWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait _wxSashLayoutWindow : _wxSashWindow {
    fn getAlignment(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetAlignment(self.handle()) }
    }
    fn getOrientation(&self) -> c_int {
        unsafe { wxSashLayoutWindow_GetOrientation(self.handle()) }
    }
    fn setAlignment(&self, align: c_int) {
        unsafe { wxSashLayoutWindow_SetAlignment(self.handle(), align) }
    }
    fn setDefaultSize(&self, w: c_int, h: c_int) {
        unsafe { wxSashLayoutWindow_SetDefaultSize(self.handle(), w, h) }
    }
    fn setOrientation(&self, orient: c_int) {
        unsafe { wxSashLayoutWindow_SetOrientation(self.handle(), orient) }
    }
}

pub struct wxSashWindow { handle: *mut c_void }
impl _wxSashWindow for wxSashWindow {}
impl _wxWindow for wxSashWindow {}
impl _wxEvtHandler for wxSashWindow {}
impl _wxObject for wxSashWindow { fn handle(&self) -> *mut c_void { self.handle } }

impl wxSashWindow {
    pub fn from(handle: *mut c_void) -> wxSashWindow { wxSashWindow { handle: handle } }
    pub fn null() -> wxSashWindow { wxSashWindow::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> wxSashWindow {
        unsafe { wxSashWindow { handle: wxSashWindow_Create(_par.handle(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait _wxSashWindow : _wxWindow {
    fn getDefaultBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetDefaultBorderSize(self.handle()) }
    }
    fn getEdgeMargin(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_GetEdgeMargin(self.handle(), edge) }
    }
    fn getExtraBorderSize(&self) -> c_int {
        unsafe { wxSashWindow_GetExtraBorderSize(self.handle()) }
    }
    fn getMaximumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeX(self.handle()) }
    }
    fn getMaximumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMaximumSizeY(self.handle()) }
    }
    fn getMinimumSizeX(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeX(self.handle()) }
    }
    fn getMinimumSizeY(&self) -> c_int {
        unsafe { wxSashWindow_GetMinimumSizeY(self.handle()) }
    }
    fn getSashVisible(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_GetSashVisible(self.handle(), edge) }
    }
    fn hasBorder(&self, edge: c_int) -> c_int {
        unsafe { wxSashWindow_HasBorder(self.handle(), edge) }
    }
    fn setDefaultBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetDefaultBorderSize(self.handle(), width) }
    }
    fn setExtraBorderSize(&self, width: c_int) {
        unsafe { wxSashWindow_SetExtraBorderSize(self.handle(), width) }
    }
    fn setMaximumSizeX(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeX(self.handle(), max) }
    }
    fn setMaximumSizeY(&self, max: c_int) {
        unsafe { wxSashWindow_SetMaximumSizeY(self.handle(), max) }
    }
    fn setMinimumSizeX(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeX(self.handle(), min) }
    }
    fn setMinimumSizeY(&self, min: c_int) {
        unsafe { wxSashWindow_SetMinimumSizeY(self.handle(), min) }
    }
    fn setSashBorder(&self, edge: c_int, border: c_int) {
        unsafe { wxSashWindow_SetSashBorder(self.handle(), edge, border) }
    }
    fn setSashVisible(&self, edge: c_int, sash: c_int) {
        unsafe { wxSashWindow_SetSashVisible(self.handle(), edge, sash) }
    }
}

pub struct wxSplashScreen { handle: *mut c_void }
impl _wxSplashScreen for wxSplashScreen {}
impl _wxFrame for wxSplashScreen {}
impl _wxTopLevelWindow for wxSplashScreen {}
impl _wxWindow for wxSplashScreen {}
impl _wxEvtHandler for wxSplashScreen {}
impl _wxObject for wxSplashScreen { fn handle(&self) -> *mut c_void { self.handle } }

impl wxSplashScreen {
    pub fn from(handle: *mut c_void) -> wxSplashScreen { wxSplashScreen { handle: handle } }
    pub fn null() -> wxSplashScreen { wxSplashScreen::from(0 as *mut c_void) }
    
}

pub trait _wxSplashScreen : _wxFrame {
}

pub struct wxTaskBarIcon { handle: *mut c_void }
impl _wxTaskBarIcon for wxTaskBarIcon {}
impl _wxEvtHandler for wxTaskBarIcon {}
impl _wxObject for wxTaskBarIcon { fn handle(&self) -> *mut c_void { self.handle } }

impl wxTaskBarIcon {
    pub fn from(handle: *mut c_void) -> wxTaskBarIcon { wxTaskBarIcon { handle: handle } }
    pub fn null() -> wxTaskBarIcon { wxTaskBarIcon::from(0 as *mut c_void) }
    
    pub fn new() -> wxTaskBarIcon {
        unsafe { wxTaskBarIcon { handle: wxTaskBarIcon_Create() } }
    }
}

pub trait _wxTaskBarIcon : _wxEvtHandler {
    fn isIconInstalled(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.handle()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsOk(self.handle()) }
    }
    fn popupMenu<T: _wxMenu>(&self, menu: &T) -> c_int {
        unsafe { wxTaskBarIcon_PopupMenu(self.handle(), menu.handle()) }
    }
    fn removeIcon(&self) -> c_int {
        unsafe { wxTaskBarIcon_RemoveIcon(self.handle()) }
    }
    fn setIcon<T: _wxIcon>(&self, icon: &T, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxTaskBarIcon_SetIcon(self.handle(), icon.handle(), text.handle()) }
    }
}

pub struct wxTipProvider { handle: *mut c_void }
impl _wxTipProvider for wxTipProvider { fn handle(&self) -> *mut c_void { self.handle } }

impl wxTipProvider {
    pub fn from(handle: *mut c_void) -> wxTipProvider { wxTipProvider { handle: handle } }
    pub fn null() -> wxTipProvider { wxTipProvider::from(0 as *mut c_void) }
    
}

pub trait _wxTipProvider {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxWizard { handle: *mut c_void }
impl _wxWizard for wxWizard {}
impl _wxDialog for wxWizard {}
impl _wxTopLevelWindow for wxWizard {}
impl _wxWindow for wxWizard {}
impl _wxEvtHandler for wxWizard {}
impl _wxObject for wxWizard { fn handle(&self) -> *mut c_void { self.handle } }

impl wxWizard {
    pub fn from(handle: *mut c_void) -> wxWizard { wxWizard { handle: handle } }
    pub fn null() -> wxWizard { wxWizard::from(0 as *mut c_void) }
    
    pub fn chain<T: _wxWizardPageSimple, U: _wxWizardPageSimple>(f: &T, s: &U) {
        unsafe { wxWizard_Chain(f.handle(), s.handle()) }
    }
    pub fn new<T: _wxWindow, U: _wxBitmap>(_prt: &T, _id: c_int, _txt: &str, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> wxWizard {
        let _txt = wxT(_txt);
        unsafe { wxWizard { handle: wxWizard_Create(_prt.handle(), _id, _txt.handle(), _bmp.handle(), _lft, _top, _wdt, _hgt) } }
    }
}

pub trait _wxWizard : _wxDialog {
    fn getCurrentPage(&self) -> wxWizardPage {
        unsafe { wxWizardPage { handle: wxWizard_GetCurrentPage(self.handle()) } }
    }
    fn getPageSize(&self) -> wxSize {
        unsafe { wxSize { handle: wxWizard_GetPageSize(self.handle()) } }
    }
    fn runWizard<T: _wxWizardPage>(&self, firstPage: &T) -> c_int {
        unsafe { wxWizard_RunWizard(self.handle(), firstPage.handle()) }
    }
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.handle(), w, h) }
    }
}

pub struct wxWizardEvent { handle: *mut c_void }
impl _wxWizardEvent for wxWizardEvent {}
impl _wxNotifyEvent for wxWizardEvent {}
impl _wxCommandEvent for wxWizardEvent {}
impl _wxEvent for wxWizardEvent {}
impl _wxObject for wxWizardEvent { fn handle(&self) -> *mut c_void { self.handle } }

impl wxWizardEvent {
    pub fn from(handle: *mut c_void) -> wxWizardEvent { wxWizardEvent { handle: handle } }
    pub fn null() -> wxWizardEvent { wxWizardEvent::from(0 as *mut c_void) }
    
}

pub trait _wxWizardEvent : _wxNotifyEvent {
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.handle()) }
    }
}

pub struct wxWizardPage { handle: *mut c_void }
impl _wxWizardPage for wxWizardPage {}
impl _wxPanel for wxWizardPage {}
impl _wxWindow for wxWizardPage {}
impl _wxEvtHandler for wxWizardPage {}
impl _wxObject for wxWizardPage { fn handle(&self) -> *mut c_void { self.handle } }

impl wxWizardPage {
    pub fn from(handle: *mut c_void) -> wxWizardPage { wxWizardPage { handle: handle } }
    pub fn null() -> wxWizardPage { wxWizardPage::from(0 as *mut c_void) }
    
}

pub trait _wxWizardPage : _wxPanel {
}

pub struct wxWizardPageSimple { handle: *mut c_void }
impl _wxWizardPageSimple for wxWizardPageSimple {}
impl _wxWizardPage for wxWizardPageSimple {}
impl _wxPanel for wxWizardPageSimple {}
impl _wxWindow for wxWizardPageSimple {}
impl _wxEvtHandler for wxWizardPageSimple {}
impl _wxObject for wxWizardPageSimple { fn handle(&self) -> *mut c_void { self.handle } }

impl wxWizardPageSimple {
    pub fn from(handle: *mut c_void) -> wxWizardPageSimple { wxWizardPageSimple { handle: handle } }
    pub fn null() -> wxWizardPageSimple { wxWizardPageSimple::from(0 as *mut c_void) }
    
    pub fn new<T: _wxWizard>(_prt: &T) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple { handle: wxWizardPageSimple_Create(_prt.handle()) } }
    }
}

pub trait _wxWizardPageSimple : _wxWizardPage {
    fn getBitmap<T: _wxBitmap>(&self, _ref: &T) {
        unsafe { wxWizardPageSimple_GetBitmap(self.handle(), _ref.handle()) }
    }
    fn getNext(&self) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple { handle: wxWizardPageSimple_GetNext(self.handle()) } }
    }
    fn getPrev(&self) -> wxWizardPageSimple {
        unsafe { wxWizardPageSimple { handle: wxWizardPageSimple_GetPrev(self.handle()) } }
    }
    fn setNext<T: _wxWizardPageSimple>(&self, next: &T) {
        unsafe { wxWizardPageSimple_SetNext(self.handle(), next.handle()) }
    }
    fn setPrev<T: _wxWizardPageSimple>(&self, prev: &T) {
        unsafe { wxWizardPageSimple_SetPrev(self.handle(), prev.handle()) }
    }
}

pub struct wxManagedPtr { handle: *mut c_void }
impl _wxManagedPtr for wxManagedPtr { fn handle(&self) -> *mut c_void { self.handle } }

impl wxManagedPtr {
    pub fn from(handle: *mut c_void) -> wxManagedPtr { wxManagedPtr { handle: handle } }
    pub fn null() -> wxManagedPtr { wxManagedPtr::from(0 as *mut c_void) }
    
    pub fn newFromObject<T: _wxObject>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromObject(obj.handle()) } }
    }
    pub fn newFromDateTime<T: _wxDateTime>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromDateTime(obj.handle()) } }
    }
    pub fn newFromGridCellCoordsArray<T: _wxGridCellCoordsArray>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromGridCellCoordsArray(obj.handle()) } }
    }
    pub fn newFromBitmap<T: _wxBitmap>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromBitmap(obj.handle()) } }
    }
    pub fn newFromIcon<T: _wxIcon>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromIcon(obj.handle()) } }
    }
    pub fn newFromBrush<T: _wxBrush>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromBrush(obj.handle()) } }
    }
    pub fn newFromColour<T: _wxColour>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromColour(obj.handle()) } }
    }
    pub fn newFromCursor<T: _wxCursor>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromCursor(obj.handle()) } }
    }
    pub fn newFromFont<T: _wxFont>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromFont(obj.handle()) } }
    }
    pub fn newFromPen<T: _wxPen>(obj: &T) -> wxManagedPtr {
        unsafe { wxManagedPtr { handle: wxManagedPtr_CreateFromPen(obj.handle()) } }
    }
}

pub trait _wxManagedPtr {
    fn handle(&self) -> *mut c_void;
    
    fn getPtr(&self) -> *mut c_void {
        unsafe { wxManagedPtr_GetPtr(self.handle()) }
    }
    fn noFinalize(&self) {
        unsafe { wxManagedPtr_NoFinalize(self.handle()) }
    }
    fn finalize(&self) {
        unsafe { wxManagedPtr_Finalize(self.handle()) }
    }
    fn delete(&self) {
        unsafe { wxManagedPtr_Delete(self.handle()) }
    }
}

pub struct wxGridCellTextEnterEditor { handle: *mut c_void }
impl _wxGridCellTextEnterEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellTextEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellEditor for wxGridCellTextEnterEditor {}
impl _wxGridCellWorker for wxGridCellTextEnterEditor { fn handle(&self) -> *mut c_void { self.handle } }

impl wxGridCellTextEnterEditor {
    pub fn from(handle: *mut c_void) -> wxGridCellTextEnterEditor { wxGridCellTextEnterEditor { handle: handle } }
    pub fn null() -> wxGridCellTextEnterEditor { wxGridCellTextEnterEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> wxGridCellTextEnterEditor {
        unsafe { wxGridCellTextEnterEditor { handle: wxGridCellTextEnterEditor_Ctor() } }
    }
}

pub trait _wxGridCellTextEnterEditor : _wxGridCellTextEditor {
}

