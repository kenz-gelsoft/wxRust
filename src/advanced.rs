use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;

pub struct RustGridTable { ptr: *mut c_void }
impl TRustGridTable for RustGridTable {}
impl TGridTableBase for RustGridTable {}
impl TObject for RustGridTable { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustGridTable {
    pub fn from(ptr: *mut c_void) -> RustGridTable { RustGridTable { ptr: ptr } }
    pub fn null() -> RustGridTable { RustGridTable::from(0 as *mut c_void) }
    
    pub fn new(_obj: *mut c_void, _EifGetNumberRows: *mut c_void, _EifGetNumberCols: *mut c_void, _EifGetValue: *mut c_void, _EifSetValue: *mut c_void, _EifIsEmptyCell: *mut c_void, _EifClear: *mut c_void, _EifInsertRows: *mut c_void, _EifAppendRows: *mut c_void, _EifDeleteRows: *mut c_void, _EifInsertCols: *mut c_void, _EifAppendCols: *mut c_void, _EifDeleteCols: *mut c_void, _EifSetRowLabelValue: *mut c_void, _EifSetColLabelValue: *mut c_void, _EifGetRowLabelValue: *mut c_void, _EifGetColLabelValue: *mut c_void) -> RustGridTable {
        unsafe { RustGridTable { ptr: ELJGridTable_Create(_obj, _EifGetNumberRows, _EifGetNumberCols, _EifGetValue, _EifSetValue, _EifIsEmptyCell, _EifClear, _EifInsertRows, _EifAppendRows, _EifDeleteRows, _EifInsertCols, _EifAppendCols, _EifDeleteCols, _EifSetRowLabelValue, _EifSetColLabelValue, _EifGetRowLabelValue, _EifGetColLabelValue) } }
    }
}

pub trait TRustGridTable : TGridTableBase {
    fn getView(&self) -> View {
        unsafe { View { ptr: ELJGridTable_GetView(self.ptr()) } }
    }
    fn sendTableMessage(&self, id: c_int, val1: c_int, val2: c_int) -> *mut c_void {
        unsafe { ELJGridTable_SendTableMessage(self.ptr(), id, val1, val2) }
    }
}

pub struct CalculateLayoutEvent { ptr: *mut c_void }
impl TCalculateLayoutEvent for CalculateLayoutEvent {}
impl TEvent for CalculateLayoutEvent {}
impl TObject for CalculateLayoutEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CalculateLayoutEvent {
    pub fn from(ptr: *mut c_void) -> CalculateLayoutEvent { CalculateLayoutEvent { ptr: ptr } }
    pub fn null() -> CalculateLayoutEvent { CalculateLayoutEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> CalculateLayoutEvent {
        unsafe { CalculateLayoutEvent { ptr: wxCalculateLayoutEvent_Create(id) } }
    }
}

pub trait TCalculateLayoutEvent : TEvent {
    fn getFlags(&self) -> c_int {
        unsafe { wxCalculateLayoutEvent_GetFlags(self.ptr()) }
    }
    fn getRect(&self) -> Rect {
        unsafe { Rect { ptr: wxCalculateLayoutEvent_GetRect(self.ptr()) } }
    }
    fn setFlags(&self, flags: c_int) {
        unsafe { wxCalculateLayoutEvent_SetFlags(self.ptr(), flags) }
    }
    fn setRect(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxCalculateLayoutEvent_SetRect(self.ptr(), x, y, w, h) }
    }
}

pub struct CalendarCtrl { ptr: *mut c_void }
impl TCalendarCtrl for CalendarCtrl {}
impl TControl for CalendarCtrl {}
impl TWindow for CalendarCtrl {}
impl TEvtHandler for CalendarCtrl {}
impl TObject for CalendarCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CalendarCtrl {
    pub fn from(ptr: *mut c_void) -> CalendarCtrl { CalendarCtrl { ptr: ptr } }
    pub fn null() -> CalendarCtrl { CalendarCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow, U: TDateTime>(_prt: &T, _id: c_int, _dat: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> CalendarCtrl {
        unsafe { CalendarCtrl { ptr: wxCalendarCtrl_Create(_prt.ptr(), _id, _dat.ptr(), _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TCalendarCtrl : TControl {
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
    fn getHeaderColourBg<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHeaderColourFg<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHeaderColourFg(self.ptr(), _ref.ptr()) }
    }
    fn getHighlightColourBg<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHighlightColourFg<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHighlightColourFg(self.ptr(), _ref.ptr()) }
    }
    fn getHolidayColourBg<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarCtrl_GetHolidayColourBg(self.ptr(), _ref.ptr()) }
    }
    fn getHolidayColourFg<T: TColour>(&self, _ref: &T) {
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

pub struct CalendarDateAttr { ptr: *mut c_void }
impl TCalendarDateAttr for CalendarDateAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CalendarDateAttr {
    pub fn from(ptr: *mut c_void) -> CalendarDateAttr { CalendarDateAttr { ptr: ptr } }
    pub fn null() -> CalendarDateAttr { CalendarDateAttr::from(0 as *mut c_void) }
    
    pub fn new(_ctxt: *mut c_void, _cbck: *mut c_void, _cbrd: *mut c_void, _fnt: *mut c_void, _brd: c_int) -> CalendarDateAttr {
        unsafe { CalendarDateAttr { ptr: wxCalendarDateAttr_Create(_ctxt, _cbck, _cbrd, _fnt, _brd) } }
    }
    pub fn newDefault() -> CalendarDateAttr {
        unsafe { CalendarDateAttr { ptr: wxCalendarDateAttr_CreateDefault() } }
    }
}

pub trait TCalendarDateAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn delete(&self) {
        unsafe { wxCalendarDateAttr_Delete(self.ptr()) }
    }
    fn getBackgroundColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getBorder(&self) -> c_int {
        unsafe { wxCalendarDateAttr_GetBorder(self.ptr()) }
    }
    fn getBorderColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetBorderColour(self.ptr(), _ref.ptr()) }
    }
    fn getFont<T: TFont>(&self, _ref: &T) {
        unsafe { wxCalendarDateAttr_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getTextColour<T: TColour>(&self, _ref: &T) {
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
    fn setBackgroundColour<T: TColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBackgroundColour(self.ptr(), col.ptr()) }
    }
    fn setBorder(&self, border: c_int) {
        unsafe { wxCalendarDateAttr_SetBorder(self.ptr(), border) }
    }
    fn setBorderColour<T: TColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetBorderColour(self.ptr(), col.ptr()) }
    }
    fn setFont<T: TFont>(&self, font: &T) {
        unsafe { wxCalendarDateAttr_SetFont(self.ptr(), font.ptr()) }
    }
    fn setHoliday(&self, holiday: c_int) {
        unsafe { wxCalendarDateAttr_SetHoliday(self.ptr(), holiday) }
    }
    fn setTextColour<T: TColour>(&self, col: &T) {
        unsafe { wxCalendarDateAttr_SetTextColour(self.ptr(), col.ptr()) }
    }
}

pub struct CalendarEvent { ptr: *mut c_void }
impl TCalendarEvent for CalendarEvent {}
impl TCommandEvent for CalendarEvent {}
impl TEvent for CalendarEvent {}
impl TObject for CalendarEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl CalendarEvent {
    pub fn from(ptr: *mut c_void) -> CalendarEvent { CalendarEvent { ptr: ptr } }
    pub fn null() -> CalendarEvent { CalendarEvent::from(0 as *mut c_void) }
    
}

pub trait TCalendarEvent : TCommandEvent {
    fn getDate(&self, _dte: *mut c_void) {
        unsafe { wxCalendarEvent_GetDate(self.ptr(), _dte) }
    }
    fn getWeekDay(&self) -> c_int {
        unsafe { wxCalendarEvent_GetWeekDay(self.ptr()) }
    }
}

pub struct EditableListBox { ptr: *mut c_void }
impl TEditableListBox for EditableListBox {}
impl TPanel for EditableListBox {}
impl TWindow for EditableListBox {}
impl TEvtHandler for EditableListBox {}
impl TObject for EditableListBox { fn ptr(&self) -> *mut c_void { self.ptr } }

impl EditableListBox {
    pub fn from(ptr: *mut c_void) -> EditableListBox { EditableListBox { ptr: ptr } }
    pub fn null() -> EditableListBox { EditableListBox::from(0 as *mut c_void) }
    
}

pub trait TEditableListBox : TPanel {
}

pub struct Grid { ptr: *mut c_void }
impl TGrid for Grid {}
impl TScrolledWindow for Grid {}
impl TPanel for Grid {}
impl TWindow for Grid {}
impl TEvtHandler for Grid {}
impl TObject for Grid { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Grid {
    pub fn from(ptr: *mut c_void) -> Grid { Grid { ptr: ptr } }
    pub fn null() -> Grid { Grid::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_prt: &T, _id: c_int, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, _stl: c_int) -> Grid {
        unsafe { Grid { ptr: wxGrid_Create(_prt.ptr(), _id, _lft, _top, _wdt, _hgt, _stl) } }
    }
}

pub trait TGrid : TScrolledWindow {
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
    fn blockToDeviceRect(&self, top: c_int, left: c_int, bottom: c_int, right: c_int) -> Rect {
        unsafe { Rect { ptr: wxGrid_BlockToDeviceRect(self.ptr(), top, left, bottom, right) } }
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
    fn cellToRect(&self, row: c_int, col: c_int) -> Rect {
        unsafe { Rect { ptr: wxGrid_CellToRect(self.ptr(), row, col) } }
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
    fn drawAllGridLines<T: TDC, U: TRegion>(&self, dc: &T, reg: &U) {
        unsafe { wxGrid_DrawAllGridLines(self.ptr(), dc.ptr(), reg.ptr()) }
    }
    fn drawCell<T: TDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCell(self.ptr(), dc.ptr(), _row, _col) }
    }
    fn drawCellBorder<T: TDC>(&self, dc: &T, _row: c_int, _col: c_int) {
        unsafe { wxGrid_DrawCellBorder(self.ptr(), dc.ptr(), _row, _col) }
    }
    fn drawCellHighlight<T: TDC, U: TGridCellAttr>(&self, dc: &T, attr: &U) {
        unsafe { wxGrid_DrawCellHighlight(self.ptr(), dc.ptr(), attr.ptr()) }
    }
    fn drawColLabel<T: TDC>(&self, dc: &T, col: c_int) {
        unsafe { wxGrid_DrawColLabel(self.ptr(), dc.ptr(), col) }
    }
    fn drawColLabels<T: TDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawColLabels(self.ptr(), dc.ptr()) }
    }
    fn drawGridSpace<T: TDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawGridSpace(self.ptr(), dc.ptr()) }
    }
    fn drawRowLabel<T: TDC>(&self, dc: &T, row: c_int) {
        unsafe { wxGrid_DrawRowLabel(self.ptr(), dc.ptr(), row) }
    }
    fn drawRowLabels<T: TDC>(&self, dc: &T) {
        unsafe { wxGrid_DrawRowLabels(self.ptr(), dc.ptr()) }
    }
    fn drawTextRectangle<T: TDC>(&self, dc: &T, txt: &str, x: c_int, y: c_int, w: c_int, h: c_int, horizontalAlignment: c_int, verticalAlignment: c_int) {
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
    fn getCellBackgroundColour<T: TColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_GetCellBackgroundColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn getCellEditor(&self, row: c_int, col: c_int) -> GridCellEditor {
        unsafe { GridCellEditor { ptr: wxGrid_GetCellEditor(self.ptr(), row, col) } }
    }
    fn getCellFont<T: TFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_GetCellFont(self.ptr(), row, col, font.ptr()) }
    }
    fn getCellHighlightColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetCellHighlightColour(self.ptr(), _ref.ptr()) }
    }
    fn getCellRenderer(&self, row: c_int, col: c_int) -> GridCellRenderer {
        unsafe { GridCellRenderer { ptr: wxGrid_GetCellRenderer(self.ptr(), row, col) } }
    }
    fn getCellTextColour<T: TColour>(&self, row: c_int, col: c_int, colour: &T) {
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
    fn getDefaultCellBackgroundColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultCellFont<T: TFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellFont(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultCellTextColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetDefaultCellTextColour(self.ptr(), _ref.ptr()) }
    }
    fn getDefaultColLabelSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColLabelSize(self.ptr()) }
    }
    fn getDefaultColSize(&self) -> c_int {
        unsafe { wxGrid_GetDefaultColSize(self.ptr()) }
    }
    fn getDefaultEditor(&self) -> GridCellEditor {
        unsafe { GridCellEditor { ptr: wxGrid_GetDefaultEditor(self.ptr()) } }
    }
    fn getDefaultEditorForCell(&self, row: c_int, col: c_int) -> GridCellEditor {
        unsafe { GridCellEditor { ptr: wxGrid_GetDefaultEditorForCell(self.ptr(), row, col) } }
    }
    fn getDefaultEditorForType(&self, typeName: &str) -> GridCellEditor {
        let typeName = wxT(typeName);
        unsafe { GridCellEditor { ptr: wxGrid_GetDefaultEditorForType(self.ptr(), typeName.ptr()) } }
    }
    fn getDefaultRenderer(&self) -> GridCellRenderer {
        unsafe { GridCellRenderer { ptr: wxGrid_GetDefaultRenderer(self.ptr()) } }
    }
    fn getDefaultRendererForCell(&self, row: c_int, col: c_int) -> GridCellRenderer {
        unsafe { GridCellRenderer { ptr: wxGrid_GetDefaultRendererForCell(self.ptr(), row, col) } }
    }
    fn getDefaultRendererForType(&self, typeName: &str) -> GridCellRenderer {
        let typeName = wxT(typeName);
        unsafe { GridCellRenderer { ptr: wxGrid_GetDefaultRendererForType(self.ptr(), typeName.ptr()) } }
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
    fn getGridLineColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetGridLineColour(self.ptr(), _ref.ptr()) }
    }
    fn getLabelBackgroundColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getLabelFont<T: TFont>(&self, _ref: &T) {
        unsafe { wxGrid_GetLabelFont(self.ptr(), _ref.ptr()) }
    }
    fn getLabelTextColour<T: TColour>(&self, _ref: &T) {
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
    fn getSelectionBackground<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionBackground(self.ptr(), _ref.ptr()) }
    }
    fn getSelectionForeground<T: TColour>(&self, _ref: &T) {
        unsafe { wxGrid_GetSelectionForeground(self.ptr(), _ref.ptr()) }
    }
    fn getTable(&self) -> GridTableBase {
        unsafe { GridTableBase { ptr: wxGrid_GetTable(self.ptr()) } }
    }
    fn getTextBoxSize<T: TDC>(&self, dc: &T, count: c_int, lines: *mut *mut c_char, _w: *mut c_void, _h: *mut c_void) {
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
    fn processTableMessage<T: TEvent>(&self, evt: &T) -> c_int {
        unsafe { wxGrid_ProcessTableMessage(self.ptr(), evt.ptr()) }
    }
    fn registerDataType<T: TGridCellRenderer, U: TGridCellEditor>(&self, typeName: &str, renderer: &T, editor: &U) {
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
    fn setCellBackgroundColour<T: TColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellBackgroundColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn setCellEditor<T: TGridCellEditor>(&self, row: c_int, col: c_int, editor: &T) {
        unsafe { wxGrid_SetCellEditor(self.ptr(), row, col, editor.ptr()) }
    }
    fn setCellFont<T: TFont>(&self, row: c_int, col: c_int, font: &T) {
        unsafe { wxGrid_SetCellFont(self.ptr(), row, col, font.ptr()) }
    }
    fn setCellHighlightColour<T: TColour>(&self, col: &T) {
        unsafe { wxGrid_SetCellHighlightColour(self.ptr(), col.ptr()) }
    }
    fn setCellRenderer<T: TGridCellRenderer>(&self, row: c_int, col: c_int, renderer: &T) {
        unsafe { wxGrid_SetCellRenderer(self.ptr(), row, col, renderer.ptr()) }
    }
    fn setCellTextColour<T: TColour>(&self, row: c_int, col: c_int, colour: &T) {
        unsafe { wxGrid_SetCellTextColour(self.ptr(), row, col, colour.ptr()) }
    }
    fn setCellValue(&self, row: c_int, col: c_int, s: &str) {
        let s = wxT(s);
        unsafe { wxGrid_SetCellValue(self.ptr(), row, col, s.ptr()) }
    }
    fn setColAttr<T: TGridCellAttr>(&self, col: c_int, attr: &T) {
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
    fn setDefaultCellBackgroundColour<T: TColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setDefaultCellFont<T: TFont>(&self, font: &T) {
        unsafe { wxGrid_SetDefaultCellFont(self.ptr(), font.ptr()) }
    }
    fn setDefaultCellTextColour<T: TColour>(&self, colour: &T) {
        unsafe { wxGrid_SetDefaultCellTextColour(self.ptr(), colour.ptr()) }
    }
    fn setDefaultColSize(&self, width: c_int, resizeExistingCols: c_int) {
        unsafe { wxGrid_SetDefaultColSize(self.ptr(), width, resizeExistingCols) }
    }
    fn setDefaultEditor<T: TGridCellEditor>(&self, editor: &T) {
        unsafe { wxGrid_SetDefaultEditor(self.ptr(), editor.ptr()) }
    }
    fn setDefaultRenderer<T: TGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGrid_SetDefaultRenderer(self.ptr(), renderer.ptr()) }
    }
    fn setDefaultRowSize(&self, height: c_int, resizeExistingRows: c_int) {
        unsafe { wxGrid_SetDefaultRowSize(self.ptr(), height, resizeExistingRows) }
    }
    fn setGridCursor(&self, row: c_int, col: c_int) {
        unsafe { wxGrid_SetGridCursor(self.ptr(), row, col) }
    }
    fn setGridLineColour<T: TColour>(&self, col: &T) {
        unsafe { wxGrid_SetGridLineColour(self.ptr(), col.ptr()) }
    }
    fn setLabelBackgroundColour<T: TColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelBackgroundColour(self.ptr(), colour.ptr()) }
    }
    fn setLabelFont<T: TFont>(&self, font: &T) {
        unsafe { wxGrid_SetLabelFont(self.ptr(), font.ptr()) }
    }
    fn setLabelTextColour<T: TColour>(&self, colour: &T) {
        unsafe { wxGrid_SetLabelTextColour(self.ptr(), colour.ptr()) }
    }
    fn setMargins(&self, extraWidth: c_int, extraHeight: c_int) {
        unsafe { wxGrid_SetMargins(self.ptr(), extraWidth, extraHeight) }
    }
    fn setReadOnly(&self, row: c_int, col: c_int, isReadOnly: c_int) {
        unsafe { wxGrid_SetReadOnly(self.ptr(), row, col, isReadOnly) }
    }
    fn setRowAttr<T: TGridCellAttr>(&self, row: c_int, attr: &T) {
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
    fn setSelectionBackground<T: TColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionBackground(self.ptr(), c.ptr()) }
    }
    fn setSelectionForeground<T: TColour>(&self, c: &T) {
        unsafe { wxGrid_SetSelectionForeground(self.ptr(), c.ptr()) }
    }
    fn setSelectionMode(&self, selmode: c_int) {
        unsafe { wxGrid_SetSelectionMode(self.ptr(), selmode) }
    }
    fn setTable<T: TGridTableBase>(&self, table: &T, takeOwnership: c_int, selmode: c_int) -> c_int {
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
    fn getSelectedCells<T: TGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectedCells(self.ptr(), _arr.ptr()) }
    }
    fn getSelectionBlockTopLeft<T: TGridCellCoordsArray>(&self, _arr: &T) {
        unsafe { wxGrid_GetSelectionBlockTopLeft(self.ptr(), _arr.ptr()) }
    }
    fn getSelectionBlockBottomRight<T: TGridCellCoordsArray>(&self, _arr: &T) {
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

pub struct GridCellAttr { ptr: *mut c_void }
impl TGridCellAttr for GridCellAttr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellAttr {
    pub fn from(ptr: *mut c_void) -> GridCellAttr { GridCellAttr { ptr: ptr } }
    pub fn null() -> GridCellAttr { GridCellAttr::from(0 as *mut c_void) }
    
    pub fn ctor() -> GridCellAttr {
        unsafe { GridCellAttr { ptr: wxGridCellAttr_Ctor() } }
    }
}

pub trait TGridCellAttr {
    fn ptr(&self) -> *mut c_void;
    
    fn decRef(&self) {
        unsafe { wxGridCellAttr_DecRef(self.ptr()) }
    }
    fn getAlignment(&self, hAlign: *mut c_int, vAlign: *mut c_int) {
        unsafe { wxGridCellAttr_GetAlignment(self.ptr(), hAlign, vAlign) }
    }
    fn getBackgroundColour<T: TColour>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetBackgroundColour(self.ptr(), _ref.ptr()) }
    }
    fn getEditor<T: TGrid>(&self, grid: &T, row: c_int, col: c_int) -> GridCellEditor {
        unsafe { GridCellEditor { ptr: wxGridCellAttr_GetEditor(self.ptr(), grid.ptr(), row, col) } }
    }
    fn getFont<T: TFont>(&self, _ref: &T) {
        unsafe { wxGridCellAttr_GetFont(self.ptr(), _ref.ptr()) }
    }
    fn getRenderer<T: TGrid>(&self, grid: &T, row: c_int, col: c_int) -> GridCellRenderer {
        unsafe { GridCellRenderer { ptr: wxGridCellAttr_GetRenderer(self.ptr(), grid.ptr(), row, col) } }
    }
    fn getTextColour<T: TColour>(&self, _ref: &T) {
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
    fn setBackgroundColour<T: TColour>(&self, colBack: &T) {
        unsafe { wxGridCellAttr_SetBackgroundColour(self.ptr(), colBack.ptr()) }
    }
    fn setDefAttr<T: TGridCellAttr>(&self, defAttr: &T) {
        unsafe { wxGridCellAttr_SetDefAttr(self.ptr(), defAttr.ptr()) }
    }
    fn setEditor<T: TGridCellEditor>(&self, editor: &T) {
        unsafe { wxGridCellAttr_SetEditor(self.ptr(), editor.ptr()) }
    }
    fn setFont<T: TFont>(&self, font: &T) {
        unsafe { wxGridCellAttr_SetFont(self.ptr(), font.ptr()) }
    }
    fn setReadOnly(&self, isReadOnly: c_int) {
        unsafe { wxGridCellAttr_SetReadOnly(self.ptr(), isReadOnly) }
    }
    fn setRenderer<T: TGridCellRenderer>(&self, renderer: &T) {
        unsafe { wxGridCellAttr_SetRenderer(self.ptr(), renderer.ptr()) }
    }
    fn setTextColour<T: TColour>(&self, colText: &T) {
        unsafe { wxGridCellAttr_SetTextColour(self.ptr(), colText.ptr()) }
    }
}

pub struct GridCellBoolEditor { ptr: *mut c_void }
impl TGridCellBoolEditor for GridCellBoolEditor {}
impl TGridCellEditor for GridCellBoolEditor {}
impl TGridCellWorker for GridCellBoolEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellBoolEditor {
    pub fn from(ptr: *mut c_void) -> GridCellBoolEditor { GridCellBoolEditor { ptr: ptr } }
    pub fn null() -> GridCellBoolEditor { GridCellBoolEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> GridCellBoolEditor {
        unsafe { GridCellBoolEditor { ptr: wxGridCellBoolEditor_Ctor() } }
    }
}

pub trait TGridCellBoolEditor : TGridCellEditor {
}

pub struct GridCellBoolRenderer { ptr: *mut c_void }
impl TGridCellBoolRenderer for GridCellBoolRenderer {}
impl TGridCellRenderer for GridCellBoolRenderer {}
impl TGridCellWorker for GridCellBoolRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellBoolRenderer {
    pub fn from(ptr: *mut c_void) -> GridCellBoolRenderer { GridCellBoolRenderer { ptr: ptr } }
    pub fn null() -> GridCellBoolRenderer { GridCellBoolRenderer::from(0 as *mut c_void) }
    
}

pub trait TGridCellBoolRenderer : TGridCellRenderer {
}

pub struct GridCellChoiceEditor { ptr: *mut c_void }
impl TGridCellChoiceEditor for GridCellChoiceEditor {}
impl TGridCellEditor for GridCellChoiceEditor {}
impl TGridCellWorker for GridCellChoiceEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellChoiceEditor {
    pub fn from(ptr: *mut c_void) -> GridCellChoiceEditor { GridCellChoiceEditor { ptr: ptr } }
    pub fn null() -> GridCellChoiceEditor { GridCellChoiceEditor::from(0 as *mut c_void) }
    
    pub fn ctor(count: c_int, choices: *mut *mut c_char, allowOthers: c_int) -> GridCellChoiceEditor {
        unsafe { GridCellChoiceEditor { ptr: wxGridCellChoiceEditor_Ctor(count, choices, allowOthers) } }
    }
}

pub trait TGridCellChoiceEditor : TGridCellEditor {
}

pub struct GridCellCoordsArray { ptr: *mut c_void }
impl TGridCellCoordsArray for GridCellCoordsArray { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellCoordsArray {
    pub fn from(ptr: *mut c_void) -> GridCellCoordsArray { GridCellCoordsArray { ptr: ptr } }
    pub fn null() -> GridCellCoordsArray { GridCellCoordsArray::from(0 as *mut c_void) }
    
    pub fn new() -> GridCellCoordsArray {
        unsafe { GridCellCoordsArray { ptr: wxGridCellCoordsArray_Create() } }
    }
}

pub trait TGridCellCoordsArray {
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

pub struct GridCellEditor { ptr: *mut c_void }
impl TGridCellEditor for GridCellEditor {}
impl TGridCellWorker for GridCellEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellEditor {
    pub fn from(ptr: *mut c_void) -> GridCellEditor { GridCellEditor { ptr: ptr } }
    pub fn null() -> GridCellEditor { GridCellEditor::from(0 as *mut c_void) }
    
}

pub trait TGridCellEditor : TGridCellWorker {
    fn beginEdit<T: TGrid>(&self, row: c_int, col: c_int, grid: &T) {
        unsafe { wxGridCellEditor_BeginEdit(self.ptr(), row, col, grid.ptr()) }
    }
    fn new<T: TWindow, U: TEvtHandler>(&self, parent: &T, id: c_int, evtHandler: &U) {
        unsafe { wxGridCellEditor_Create(self.ptr(), parent.ptr(), id, evtHandler.ptr()) }
    }
    fn destroy(&self) {
        unsafe { wxGridCellEditor_Destroy(self.ptr()) }
    }
    fn endEdit<T: TGrid>(&self, row: c_int, col: c_int, grid: &T, oldStr: &str, newStr: &str) -> c_int {
        let oldStr = wxT(oldStr);
        let newStr = wxT(newStr);
        unsafe { wxGridCellEditor_EndEdit(self.ptr(), row, col, grid.ptr(), oldStr.ptr(), newStr.ptr()) }
    }
    fn getControl(&self) -> Control {
        unsafe { Control { ptr: wxGridCellEditor_GetControl(self.ptr()) } }
    }
    fn handleReturn<T: TEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_HandleReturn(self.ptr(), event.ptr()) }
    }
    fn isAcceptedKey<T: TEvent>(&self, event: &T) -> c_int {
        unsafe { wxGridCellEditor_IsAcceptedKey(self.ptr(), event.ptr()) }
    }
    fn isCreated(&self) -> c_int {
        unsafe { wxGridCellEditor_IsCreated(self.ptr()) }
    }
    fn paintBackground<T: TDC, U: TGridCellAttr>(&self, dc: &T, x: c_int, y: c_int, w: c_int, h: c_int, attr: &U) {
        unsafe { wxGridCellEditor_PaintBackground(self.ptr(), dc.ptr(), x, y, w, h, attr.ptr()) }
    }
    fn reset(&self) {
        unsafe { wxGridCellEditor_Reset(self.ptr()) }
    }
    fn setControl<T: TControl>(&self, control: &T) {
        unsafe { wxGridCellEditor_SetControl(self.ptr(), control.ptr()) }
    }
    fn setParameters(&self, params: &str) {
        let params = wxT(params);
        unsafe { wxGridCellEditor_SetParameters(self.ptr(), params.ptr()) }
    }
    fn setSize(&self, x: c_int, y: c_int, w: c_int, h: c_int) {
        unsafe { wxGridCellEditor_SetSize(self.ptr(), x, y, w, h) }
    }
    fn show<T: TGridCellAttr>(&self, show: c_int, attr: &T) {
        unsafe { wxGridCellEditor_Show(self.ptr(), show, attr.ptr()) }
    }
    fn startingClick(&self) {
        unsafe { wxGridCellEditor_StartingClick(self.ptr()) }
    }
    fn startingKey<T: TEvent>(&self, event: &T) {
        unsafe { wxGridCellEditor_StartingKey(self.ptr(), event.ptr()) }
    }
}

pub struct GridCellFloatEditor { ptr: *mut c_void }
impl TGridCellFloatEditor for GridCellFloatEditor {}
impl TGridCellTextEditor for GridCellFloatEditor {}
impl TGridCellEditor for GridCellFloatEditor {}
impl TGridCellWorker for GridCellFloatEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellFloatEditor {
    pub fn from(ptr: *mut c_void) -> GridCellFloatEditor { GridCellFloatEditor { ptr: ptr } }
    pub fn null() -> GridCellFloatEditor { GridCellFloatEditor::from(0 as *mut c_void) }
    
    pub fn ctor(width: c_int, precision: c_int) -> GridCellFloatEditor {
        unsafe { GridCellFloatEditor { ptr: wxGridCellFloatEditor_Ctor(width, precision) } }
    }
}

pub trait TGridCellFloatEditor : TGridCellTextEditor {
}

pub struct GridCellFloatRenderer { ptr: *mut c_void }
impl TGridCellFloatRenderer for GridCellFloatRenderer {}
impl TGridCellStringRenderer for GridCellFloatRenderer {}
impl TGridCellRenderer for GridCellFloatRenderer {}
impl TGridCellWorker for GridCellFloatRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellFloatRenderer {
    pub fn from(ptr: *mut c_void) -> GridCellFloatRenderer { GridCellFloatRenderer { ptr: ptr } }
    pub fn null() -> GridCellFloatRenderer { GridCellFloatRenderer::from(0 as *mut c_void) }
    
}

pub trait TGridCellFloatRenderer : TGridCellStringRenderer {
}

pub struct GridCellNumberEditor { ptr: *mut c_void }
impl TGridCellNumberEditor for GridCellNumberEditor {}
impl TGridCellTextEditor for GridCellNumberEditor {}
impl TGridCellEditor for GridCellNumberEditor {}
impl TGridCellWorker for GridCellNumberEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellNumberEditor {
    pub fn from(ptr: *mut c_void) -> GridCellNumberEditor { GridCellNumberEditor { ptr: ptr } }
    pub fn null() -> GridCellNumberEditor { GridCellNumberEditor::from(0 as *mut c_void) }
    
    pub fn ctor(min: c_int, max: c_int) -> GridCellNumberEditor {
        unsafe { GridCellNumberEditor { ptr: wxGridCellNumberEditor_Ctor(min, max) } }
    }
}

pub trait TGridCellNumberEditor : TGridCellTextEditor {
}

pub struct GridCellNumberRenderer { ptr: *mut c_void }
impl TGridCellNumberRenderer for GridCellNumberRenderer {}
impl TGridCellStringRenderer for GridCellNumberRenderer {}
impl TGridCellRenderer for GridCellNumberRenderer {}
impl TGridCellWorker for GridCellNumberRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellNumberRenderer {
    pub fn from(ptr: *mut c_void) -> GridCellNumberRenderer { GridCellNumberRenderer { ptr: ptr } }
    pub fn null() -> GridCellNumberRenderer { GridCellNumberRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> GridCellNumberRenderer {
        unsafe { GridCellNumberRenderer { ptr: wxGridCellNumberRenderer_Ctor() } }
    }
}

pub trait TGridCellNumberRenderer : TGridCellStringRenderer {
}

pub struct GridCellAutoWrapStringRenderer { ptr: *mut c_void }
impl TGridCellAutoWrapStringRenderer for GridCellAutoWrapStringRenderer {}
impl TGridCellStringRenderer for GridCellAutoWrapStringRenderer {}
impl TGridCellRenderer for GridCellAutoWrapStringRenderer {}
impl TGridCellWorker for GridCellAutoWrapStringRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellAutoWrapStringRenderer {
    pub fn from(ptr: *mut c_void) -> GridCellAutoWrapStringRenderer { GridCellAutoWrapStringRenderer { ptr: ptr } }
    pub fn null() -> GridCellAutoWrapStringRenderer { GridCellAutoWrapStringRenderer::from(0 as *mut c_void) }
    
    pub fn ctor() -> GridCellAutoWrapStringRenderer {
        unsafe { GridCellAutoWrapStringRenderer { ptr: wxGridCellAutoWrapStringRenderer_Ctor() } }
    }
}

pub trait TGridCellAutoWrapStringRenderer : TGridCellStringRenderer {
}

pub struct GridCellRenderer { ptr: *mut c_void }
impl TGridCellRenderer for GridCellRenderer {}
impl TGridCellWorker for GridCellRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellRenderer {
    pub fn from(ptr: *mut c_void) -> GridCellRenderer { GridCellRenderer { ptr: ptr } }
    pub fn null() -> GridCellRenderer { GridCellRenderer::from(0 as *mut c_void) }
    
}

pub trait TGridCellRenderer : TGridCellWorker {
}

pub struct GridCellStringRenderer { ptr: *mut c_void }
impl TGridCellStringRenderer for GridCellStringRenderer {}
impl TGridCellRenderer for GridCellStringRenderer {}
impl TGridCellWorker for GridCellStringRenderer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellStringRenderer {
    pub fn from(ptr: *mut c_void) -> GridCellStringRenderer { GridCellStringRenderer { ptr: ptr } }
    pub fn null() -> GridCellStringRenderer { GridCellStringRenderer::from(0 as *mut c_void) }
    
}

pub trait TGridCellStringRenderer : TGridCellRenderer {
}

pub struct GridCellTextEditor { ptr: *mut c_void }
impl TGridCellTextEditor for GridCellTextEditor {}
impl TGridCellEditor for GridCellTextEditor {}
impl TGridCellWorker for GridCellTextEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellTextEditor {
    pub fn from(ptr: *mut c_void) -> GridCellTextEditor { GridCellTextEditor { ptr: ptr } }
    pub fn null() -> GridCellTextEditor { GridCellTextEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> GridCellTextEditor {
        unsafe { GridCellTextEditor { ptr: wxGridCellTextEditor_Ctor() } }
    }
}

pub trait TGridCellTextEditor : TGridCellEditor {
}

pub struct GridCellWorker { ptr: *mut c_void }
impl TGridCellWorker for GridCellWorker { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellWorker {
    pub fn from(ptr: *mut c_void) -> GridCellWorker { GridCellWorker { ptr: ptr } }
    pub fn null() -> GridCellWorker { GridCellWorker::from(0 as *mut c_void) }
    
}

pub trait TGridCellWorker {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct GridEditorCreatedEvent { ptr: *mut c_void }
impl TGridEditorCreatedEvent for GridEditorCreatedEvent {}
impl TCommandEvent for GridEditorCreatedEvent {}
impl TEvent for GridEditorCreatedEvent {}
impl TObject for GridEditorCreatedEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridEditorCreatedEvent {
    pub fn from(ptr: *mut c_void) -> GridEditorCreatedEvent { GridEditorCreatedEvent { ptr: ptr } }
    pub fn null() -> GridEditorCreatedEvent { GridEditorCreatedEvent::from(0 as *mut c_void) }
    
}

pub trait TGridEditorCreatedEvent : TCommandEvent {
    fn getCol(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetCol(self.ptr()) }
    }
    fn getControl(&self) -> Control {
        unsafe { Control { ptr: wxGridEditorCreatedEvent_GetControl(self.ptr()) } }
    }
    fn getRow(&self) -> c_int {
        unsafe { wxGridEditorCreatedEvent_GetRow(self.ptr()) }
    }
    fn setCol(&self, col: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetCol(self.ptr(), col) }
    }
    fn setControl<T: TControl>(&self, ctrl: &T) {
        unsafe { wxGridEditorCreatedEvent_SetControl(self.ptr(), ctrl.ptr()) }
    }
    fn setRow(&self, row: c_int) {
        unsafe { wxGridEditorCreatedEvent_SetRow(self.ptr(), row) }
    }
}

pub struct GridEvent { ptr: *mut c_void }
impl TGridEvent for GridEvent {}
impl TNotifyEvent for GridEvent {}
impl TCommandEvent for GridEvent {}
impl TEvent for GridEvent {}
impl TObject for GridEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridEvent {
    pub fn from(ptr: *mut c_void) -> GridEvent { GridEvent { ptr: ptr } }
    pub fn null() -> GridEvent { GridEvent::from(0 as *mut c_void) }
    
}

pub trait TGridEvent : TNotifyEvent {
    fn altDown(&self) -> c_int {
        unsafe { wxGridEvent_AltDown(self.ptr()) }
    }
    fn controlDown(&self) -> c_int {
        unsafe { wxGridEvent_ControlDown(self.ptr()) }
    }
    fn getCol(&self) -> c_int {
        unsafe { wxGridEvent_GetCol(self.ptr()) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxGridEvent_GetPosition(self.ptr()) } }
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

pub struct GridRangeSelectEvent { ptr: *mut c_void }
impl TGridRangeSelectEvent for GridRangeSelectEvent {}
impl TNotifyEvent for GridRangeSelectEvent {}
impl TCommandEvent for GridRangeSelectEvent {}
impl TEvent for GridRangeSelectEvent {}
impl TObject for GridRangeSelectEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridRangeSelectEvent {
    pub fn from(ptr: *mut c_void) -> GridRangeSelectEvent { GridRangeSelectEvent { ptr: ptr } }
    pub fn null() -> GridRangeSelectEvent { GridRangeSelectEvent::from(0 as *mut c_void) }
    
}

pub trait TGridRangeSelectEvent : TNotifyEvent {
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

pub struct GridSizeEvent { ptr: *mut c_void }
impl TGridSizeEvent for GridSizeEvent {}
impl TNotifyEvent for GridSizeEvent {}
impl TCommandEvent for GridSizeEvent {}
impl TEvent for GridSizeEvent {}
impl TObject for GridSizeEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridSizeEvent {
    pub fn from(ptr: *mut c_void) -> GridSizeEvent { GridSizeEvent { ptr: ptr } }
    pub fn null() -> GridSizeEvent { GridSizeEvent::from(0 as *mut c_void) }
    
}

pub trait TGridSizeEvent : TNotifyEvent {
    fn getRowOrCol(&self) -> c_int {
        unsafe { wxGridSizeEvent_GetRowOrCol(self.ptr()) }
    }
    fn getPosition(&self) -> Point {
        unsafe { Point { ptr: wxGridSizeEvent_GetPosition(self.ptr()) } }
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

pub struct GridTableBase { ptr: *mut c_void }
impl TGridTableBase for GridTableBase {}
impl TObject for GridTableBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridTableBase {
    pub fn from(ptr: *mut c_void) -> GridTableBase { GridTableBase { ptr: ptr } }
    pub fn null() -> GridTableBase { GridTableBase::from(0 as *mut c_void) }
    
}

pub trait TGridTableBase : TObject {
}

pub struct Joystick { ptr: *mut c_void }
impl TJoystick for Joystick {}
impl TObject for Joystick { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Joystick {
    pub fn from(ptr: *mut c_void) -> Joystick { Joystick { ptr: ptr } }
    pub fn null() -> Joystick { Joystick::from(0 as *mut c_void) }
    
}

pub trait TJoystick : TObject {
}

pub struct LayoutAlgorithm { ptr: *mut c_void }
impl TLayoutAlgorithm for LayoutAlgorithm {}
impl TObject for LayoutAlgorithm { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LayoutAlgorithm {
    pub fn from(ptr: *mut c_void) -> LayoutAlgorithm { LayoutAlgorithm { ptr: ptr } }
    pub fn null() -> LayoutAlgorithm { LayoutAlgorithm::from(0 as *mut c_void) }
    
    pub fn new() -> LayoutAlgorithm {
        unsafe { LayoutAlgorithm { ptr: wxLayoutAlgorithm_Create() } }
    }
}

pub trait TLayoutAlgorithm : TObject {
    fn layoutFrame<T: TFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutFrame(self.ptr(), frame.ptr(), mainWindow) }
    }
    fn layoutMDIFrame<T: TFrame>(&self, frame: &T, x: c_int, y: c_int, w: c_int, h: c_int, use_: c_int) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutMDIFrame(self.ptr(), frame.ptr(), x, y, w, h, use_) }
    }
    fn layoutWindow<T: TFrame>(&self, frame: &T, mainWindow: *mut c_void) -> c_int {
        unsafe { wxLayoutAlgorithm_LayoutWindow(self.ptr(), frame.ptr(), mainWindow) }
    }
}

pub struct QueryLayoutInfoEvent { ptr: *mut c_void }
impl TQueryLayoutInfoEvent for QueryLayoutInfoEvent {}
impl TEvent for QueryLayoutInfoEvent {}
impl TObject for QueryLayoutInfoEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl QueryLayoutInfoEvent {
    pub fn from(ptr: *mut c_void) -> QueryLayoutInfoEvent { QueryLayoutInfoEvent { ptr: ptr } }
    pub fn null() -> QueryLayoutInfoEvent { QueryLayoutInfoEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int) -> QueryLayoutInfoEvent {
        unsafe { QueryLayoutInfoEvent { ptr: wxQueryLayoutInfoEvent_Create(id) } }
    }
}

pub trait TQueryLayoutInfoEvent : TEvent {
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
    fn getSize(&self) -> Size {
        unsafe { Size { ptr: wxQueryLayoutInfoEvent_GetSize(self.ptr()) } }
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

pub struct SashEvent { ptr: *mut c_void }
impl TSashEvent for SashEvent {}
impl TEvent for SashEvent {}
impl TObject for SashEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SashEvent {
    pub fn from(ptr: *mut c_void) -> SashEvent { SashEvent { ptr: ptr } }
    pub fn null() -> SashEvent { SashEvent::from(0 as *mut c_void) }
    
    pub fn new(id: c_int, edge: c_int) -> SashEvent {
        unsafe { SashEvent { ptr: wxSashEvent_Create(id, edge) } }
    }
}

pub trait TSashEvent : TEvent {
    fn getDragRect(&self) -> Rect {
        unsafe { Rect { ptr: wxSashEvent_GetDragRect(self.ptr()) } }
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

pub struct SashLayoutWindow { ptr: *mut c_void }
impl TSashLayoutWindow for SashLayoutWindow {}
impl TSashWindow for SashLayoutWindow {}
impl TWindow for SashLayoutWindow {}
impl TEvtHandler for SashLayoutWindow {}
impl TObject for SashLayoutWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SashLayoutWindow {
    pub fn from(ptr: *mut c_void) -> SashLayoutWindow { SashLayoutWindow { ptr: ptr } }
    pub fn null() -> SashLayoutWindow { SashLayoutWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> SashLayoutWindow {
        unsafe { SashLayoutWindow { ptr: wxSashLayoutWindow_Create(_par.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait TSashLayoutWindow : TSashWindow {
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

pub struct SashWindow { ptr: *mut c_void }
impl TSashWindow for SashWindow {}
impl TWindow for SashWindow {}
impl TEvtHandler for SashWindow {}
impl TObject for SashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SashWindow {
    pub fn from(ptr: *mut c_void) -> SashWindow { SashWindow { ptr: ptr } }
    pub fn null() -> SashWindow { SashWindow::from(0 as *mut c_void) }
    
    pub fn new<T: TWindow>(_par: &T, _id: c_int, _x: c_int, _y: c_int, _w: c_int, _h: c_int, _stl: c_int) -> SashWindow {
        unsafe { SashWindow { ptr: wxSashWindow_Create(_par.ptr(), _id, _x, _y, _w, _h, _stl) } }
    }
}

pub trait TSashWindow : TWindow {
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

pub struct SplashScreen { ptr: *mut c_void }
impl TSplashScreen for SplashScreen {}
impl TFrame for SplashScreen {}
impl TTopLevelWindow for SplashScreen {}
impl TWindow for SplashScreen {}
impl TEvtHandler for SplashScreen {}
impl TObject for SplashScreen { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplashScreen {
    pub fn from(ptr: *mut c_void) -> SplashScreen { SplashScreen { ptr: ptr } }
    pub fn null() -> SplashScreen { SplashScreen::from(0 as *mut c_void) }
    
}

pub trait TSplashScreen : TFrame {
}

pub struct TaskBarIcon { ptr: *mut c_void }
impl TTaskBarIcon for TaskBarIcon {}
impl TEvtHandler for TaskBarIcon {}
impl TObject for TaskBarIcon { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TaskBarIcon {
    pub fn from(ptr: *mut c_void) -> TaskBarIcon { TaskBarIcon { ptr: ptr } }
    pub fn null() -> TaskBarIcon { TaskBarIcon::from(0 as *mut c_void) }
    
    pub fn new() -> TaskBarIcon {
        unsafe { TaskBarIcon { ptr: wxTaskBarIcon_Create() } }
    }
}

pub trait TTaskBarIcon : TEvtHandler {
    fn isIconInstalled(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsIconInstalled(self.ptr()) }
    }
    fn isOk(&self) -> c_int {
        unsafe { wxTaskBarIcon_IsOk(self.ptr()) }
    }
    fn popupMenu<T: TMenu>(&self, menu: &T) -> c_int {
        unsafe { wxTaskBarIcon_PopupMenu(self.ptr(), menu.ptr()) }
    }
    fn removeIcon(&self) -> c_int {
        unsafe { wxTaskBarIcon_RemoveIcon(self.ptr()) }
    }
    fn setIcon<T: TIcon>(&self, icon: &T, text: &str) -> c_int {
        let text = wxT(text);
        unsafe { wxTaskBarIcon_SetIcon(self.ptr(), icon.ptr(), text.ptr()) }
    }
}

pub struct TipProvider { ptr: *mut c_void }
impl TTipProvider for TipProvider { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TipProvider {
    pub fn from(ptr: *mut c_void) -> TipProvider { TipProvider { ptr: ptr } }
    pub fn null() -> TipProvider { TipProvider::from(0 as *mut c_void) }
    
}

pub trait TTipProvider {
    fn ptr(&self) -> *mut c_void;
    
}

pub struct Wizard { ptr: *mut c_void }
impl TWizard for Wizard {}
impl TDialog for Wizard {}
impl TTopLevelWindow for Wizard {}
impl TWindow for Wizard {}
impl TEvtHandler for Wizard {}
impl TObject for Wizard { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Wizard {
    pub fn from(ptr: *mut c_void) -> Wizard { Wizard { ptr: ptr } }
    pub fn null() -> Wizard { Wizard::from(0 as *mut c_void) }
    
    pub fn chain<T: TWizardPageSimple, U: TWizardPageSimple>(f: &T, s: &U) {
        unsafe { wxWizard_Chain(f.ptr(), s.ptr()) }
    }
    pub fn new<T: TWindow, U: TBitmap>(_prt: &T, _id: c_int, _txt: &str, _bmp: &U, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int) -> Wizard {
        let _txt = wxT(_txt);
        unsafe { Wizard { ptr: wxWizard_Create(_prt.ptr(), _id, _txt.ptr(), _bmp.ptr(), _lft, _top, _wdt, _hgt) } }
    }
}

pub trait TWizard : TDialog {
    fn getCurrentPage(&self) -> WizardPage {
        unsafe { WizardPage { ptr: wxWizard_GetCurrentPage(self.ptr()) } }
    }
    fn getPageSize(&self) -> Size {
        unsafe { Size { ptr: wxWizard_GetPageSize(self.ptr()) } }
    }
    fn runWizard<T: TWizardPage>(&self, firstPage: &T) -> c_int {
        unsafe { wxWizard_RunWizard(self.ptr(), firstPage.ptr()) }
    }
    fn setPageSize(&self, w: c_int, h: c_int) {
        unsafe { wxWizard_SetPageSize(self.ptr(), w, h) }
    }
}

pub struct WizardEvent { ptr: *mut c_void }
impl TWizardEvent for WizardEvent {}
impl TNotifyEvent for WizardEvent {}
impl TCommandEvent for WizardEvent {}
impl TEvent for WizardEvent {}
impl TObject for WizardEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WizardEvent {
    pub fn from(ptr: *mut c_void) -> WizardEvent { WizardEvent { ptr: ptr } }
    pub fn null() -> WizardEvent { WizardEvent::from(0 as *mut c_void) }
    
}

pub trait TWizardEvent : TNotifyEvent {
    fn getDirection(&self) -> c_int {
        unsafe { wxWizardEvent_GetDirection(self.ptr()) }
    }
}

pub struct WizardPage { ptr: *mut c_void }
impl TWizardPage for WizardPage {}
impl TPanel for WizardPage {}
impl TWindow for WizardPage {}
impl TEvtHandler for WizardPage {}
impl TObject for WizardPage { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WizardPage {
    pub fn from(ptr: *mut c_void) -> WizardPage { WizardPage { ptr: ptr } }
    pub fn null() -> WizardPage { WizardPage::from(0 as *mut c_void) }
    
}

pub trait TWizardPage : TPanel {
}

pub struct WizardPageSimple { ptr: *mut c_void }
impl TWizardPageSimple for WizardPageSimple {}
impl TWizardPage for WizardPageSimple {}
impl TPanel for WizardPageSimple {}
impl TWindow for WizardPageSimple {}
impl TEvtHandler for WizardPageSimple {}
impl TObject for WizardPageSimple { fn ptr(&self) -> *mut c_void { self.ptr } }

impl WizardPageSimple {
    pub fn from(ptr: *mut c_void) -> WizardPageSimple { WizardPageSimple { ptr: ptr } }
    pub fn null() -> WizardPageSimple { WizardPageSimple::from(0 as *mut c_void) }
    
    pub fn new<T: TWizard>(_prt: &T) -> WizardPageSimple {
        unsafe { WizardPageSimple { ptr: wxWizardPageSimple_Create(_prt.ptr()) } }
    }
}

pub trait TWizardPageSimple : TWizardPage {
    fn getBitmap<T: TBitmap>(&self, _ref: &T) {
        unsafe { wxWizardPageSimple_GetBitmap(self.ptr(), _ref.ptr()) }
    }
    fn getNext(&self) -> WizardPageSimple {
        unsafe { WizardPageSimple { ptr: wxWizardPageSimple_GetNext(self.ptr()) } }
    }
    fn getPrev(&self) -> WizardPageSimple {
        unsafe { WizardPageSimple { ptr: wxWizardPageSimple_GetPrev(self.ptr()) } }
    }
    fn setNext<T: TWizardPageSimple>(&self, next: &T) {
        unsafe { wxWizardPageSimple_SetNext(self.ptr(), next.ptr()) }
    }
    fn setPrev<T: TWizardPageSimple>(&self, prev: &T) {
        unsafe { wxWizardPageSimple_SetPrev(self.ptr(), prev.ptr()) }
    }
}

pub struct ManagedPtr { ptr: *mut c_void }
impl TManagedPtr for ManagedPtr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ManagedPtr {
    pub fn from(ptr: *mut c_void) -> ManagedPtr { ManagedPtr { ptr: ptr } }
    pub fn null() -> ManagedPtr { ManagedPtr::from(0 as *mut c_void) }
    
    pub fn newFromObject<T: TObject>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromObject(obj.ptr()) } }
    }
    pub fn newFromDateTime<T: TDateTime>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromDateTime(obj.ptr()) } }
    }
    pub fn newFromGridCellCoordsArray<T: TGridCellCoordsArray>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromGridCellCoordsArray(obj.ptr()) } }
    }
    pub fn newFromBitmap<T: TBitmap>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromBitmap(obj.ptr()) } }
    }
    pub fn newFromIcon<T: TIcon>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromIcon(obj.ptr()) } }
    }
    pub fn newFromBrush<T: TBrush>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromBrush(obj.ptr()) } }
    }
    pub fn newFromColour<T: TColour>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromColour(obj.ptr()) } }
    }
    pub fn newFromCursor<T: TCursor>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromCursor(obj.ptr()) } }
    }
    pub fn newFromFont<T: TFont>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromFont(obj.ptr()) } }
    }
    pub fn newFromPen<T: TPen>(obj: &T) -> ManagedPtr {
        unsafe { ManagedPtr { ptr: wxManagedPtr_CreateFromPen(obj.ptr()) } }
    }
}

pub trait TManagedPtr {
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

pub struct GridCellTextEnterEditor { ptr: *mut c_void }
impl TGridCellTextEnterEditor for GridCellTextEnterEditor {}
impl TGridCellTextEditor for GridCellTextEnterEditor {}
impl TGridCellEditor for GridCellTextEnterEditor {}
impl TGridCellWorker for GridCellTextEnterEditor { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GridCellTextEnterEditor {
    pub fn from(ptr: *mut c_void) -> GridCellTextEnterEditor { GridCellTextEnterEditor { ptr: ptr } }
    pub fn null() -> GridCellTextEnterEditor { GridCellTextEnterEditor::from(0 as *mut c_void) }
    
    pub fn ctor() -> GridCellTextEnterEditor {
        unsafe { GridCellTextEnterEditor { ptr: wxGridCellTextEnterEditor_Ctor() } }
    }
}

pub trait TGridCellTextEnterEditor : TGridCellTextEditor {
}

