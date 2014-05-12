use libc::*;
use _unsafe::*;
use base::*;
use core::*;
use advanced::*;

/// Wraps the wxWidgets' [wxStyledTextCtrl](http://docs.wxwidgets.org/3.0/classwx_styled_text_ctrl.html) class.
pub struct StyledTextCtrl { ptr: *mut c_void }
impl StyledTextCtrlMethods for StyledTextCtrl {}
impl ControlMethods for StyledTextCtrl {}
impl WindowMethods for StyledTextCtrl {}
impl EvtHandlerMethods for StyledTextCtrl {}
impl ObjectMethods for StyledTextCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StyledTextCtrl {
    pub fn from(ptr: *mut c_void) -> StyledTextCtrl { StyledTextCtrl { ptr: ptr } }
    pub fn null() -> StyledTextCtrl { StyledTextCtrl::from(0 as *mut c_void) }
    
    pub fn new<T: WindowMethods>(_prt: &T, _id: c_int, _txt: &str, _lft: c_int, _top: c_int, _wdt: c_int, _hgt: c_int, style: c_int) -> StyledTextCtrl {
        let _txt = strToString(_txt);
        unsafe { StyledTextCtrl::from(wxStyledTextCtrl_Create(_prt.ptr(), _id, _txt.ptr(), _lft, _top, _wdt, _hgt, style)) }
    }
}

/// Methods of the wxWidgets' [wxStyledTextCtrl](http://docs.wxwidgets.org/3.0/classwx_styled_text_ctrl.html) class.
pub trait StyledTextCtrlMethods : ControlMethods {
    fn addText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_AddText(self.ptr(), text.ptr()) }
    }
    fn addStyledText<T: MemoryBufferMethods>(&self, data: &T) {
        unsafe { wxStyledTextCtrl_AddStyledText(self.ptr(), data.ptr()) }
    }
    fn insertText(&self, pos: c_int, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_InsertText(self.ptr(), pos, text.ptr()) }
    }
    fn clearAll(&self) {
        unsafe { wxStyledTextCtrl_ClearAll(self.ptr()) }
    }
    fn clearDocumentStyle(&self) {
        unsafe { wxStyledTextCtrl_ClearDocumentStyle(self.ptr()) }
    }
    fn getLength(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLength(self.ptr()) }
    }
    fn getCharAt(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetCharAt(self.ptr(), pos) }
    }
    fn getCurrentPos(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentPos(self.ptr()) }
    }
    fn getAnchor(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetAnchor(self.ptr()) }
    }
    fn getStyleAt(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetStyleAt(self.ptr(), pos) }
    }
    fn redo(&self) {
        unsafe { wxStyledTextCtrl_Redo(self.ptr()) }
    }
    fn setUndoCollection(&self, collectUndo: c_int) {
        unsafe { wxStyledTextCtrl_SetUndoCollection(self.ptr(), collectUndo) }
    }
    fn selectAll(&self) {
        unsafe { wxStyledTextCtrl_SelectAll(self.ptr()) }
    }
    fn setSavePoint(&self) {
        unsafe { wxStyledTextCtrl_SetSavePoint(self.ptr()) }
    }
    fn canRedo(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CanRedo(self.ptr()) }
    }
    fn markerLineFromHandle(&self, handle: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerLineFromHandle(self.ptr(), handle) }
    }
    fn markerDeleteHandle(&self, handle: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteHandle(self.ptr(), handle) }
    }
    fn getUndoCollection(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUndoCollection(self.ptr()) }
    }
    fn getViewWhiteSpace(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetViewWhiteSpace(self.ptr()) }
    }
    fn setViewWhiteSpace(&self, viewWS: c_int) {
        unsafe { wxStyledTextCtrl_SetViewWhiteSpace(self.ptr(), viewWS) }
    }
    fn positionFromPoint(&self, pt_x: c_int, pt_y: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromPoint(self.ptr(), pt_x, pt_y) }
    }
    fn positionFromPointClose(&self, x: c_int, y: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromPointClose(self.ptr(), x, y) }
    }
    fn gotoLine(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_GotoLine(self.ptr(), line) }
    }
    fn gotoPos(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_GotoPos(self.ptr(), pos) }
    }
    fn setAnchor(&self, posAnchor: c_int) {
        unsafe { wxStyledTextCtrl_SetAnchor(self.ptr(), posAnchor) }
    }
    fn getEndStyled(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEndStyled(self.ptr()) }
    }
    fn convertEOLs(&self, eolMode: c_int) {
        unsafe { wxStyledTextCtrl_ConvertEOLs(self.ptr(), eolMode) }
    }
    fn getEOLMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEOLMode(self.ptr()) }
    }
    fn setEOLMode(&self, eolMode: c_int) {
        unsafe { wxStyledTextCtrl_SetEOLMode(self.ptr(), eolMode) }
    }
    fn startStyling(&self, pos: c_int, mask: c_int) {
        unsafe { wxStyledTextCtrl_StartStyling(self.ptr(), pos, mask) }
    }
    fn setStyling(&self, length: c_int, style: c_int) {
        unsafe { wxStyledTextCtrl_SetStyling(self.ptr(), length, style) }
    }
    fn getBufferedDraw(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetBufferedDraw(self.ptr()) }
    }
    fn setBufferedDraw(&self, buffered: c_int) {
        unsafe { wxStyledTextCtrl_SetBufferedDraw(self.ptr(), buffered) }
    }
    fn setTabWidth(&self, tabWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetTabWidth(self.ptr(), tabWidth) }
    }
    fn getTabWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTabWidth(self.ptr()) }
    }
    fn setCodePage(&self, codePage: c_int) {
        unsafe { wxStyledTextCtrl_SetCodePage(self.ptr(), codePage) }
    }
    fn markerDefine(&self, markerNumber: c_int, markerSymbol: c_int, foreground_r: uint8_t, foreground_g: uint8_t, foreground_b: uint8_t, background_r: uint8_t, background_g: uint8_t, background_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerDefine(self.ptr(), markerNumber, markerSymbol, foreground_r, foreground_g, foreground_b, background_r, background_g, background_b) }
    }
    fn markerSetForeground(&self, markerNumber: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerSetForeground(self.ptr(), markerNumber, fore_r, fore_g, fore_b) }
    }
    fn markerSetBackground(&self, markerNumber: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_MarkerSetBackground(self.ptr(), markerNumber, back_r, back_g, back_b) }
    }
    fn markerAdd(&self, line: c_int, markerNumber: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerAdd(self.ptr(), line, markerNumber) }
    }
    fn markerDelete(&self, line: c_int, markerNumber: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDelete(self.ptr(), line, markerNumber) }
    }
    fn markerDeleteAll(&self, markerNumber: c_int) {
        unsafe { wxStyledTextCtrl_MarkerDeleteAll(self.ptr(), markerNumber) }
    }
    fn markerGet(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerGet(self.ptr(), line) }
    }
    fn markerNext(&self, lineStart: c_int, markerMask: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerNext(self.ptr(), lineStart, markerMask) }
    }
    fn markerPrevious(&self, lineStart: c_int, markerMask: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_MarkerPrevious(self.ptr(), lineStart, markerMask) }
    }
    fn markerDefineBitmap<T: BitmapMethods>(&self, markerNumber: c_int, bmp: &T) {
        unsafe { wxStyledTextCtrl_MarkerDefineBitmap(self.ptr(), markerNumber, bmp.ptr()) }
    }
    fn setMarginType(&self, margin: c_int, marginType: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginType(self.ptr(), margin, marginType) }
    }
    fn getMarginType(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginType(self.ptr(), margin) }
    }
    fn setMarginWidth(&self, margin: c_int, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginWidth(self.ptr(), margin, pixelWidth) }
    }
    fn getMarginWidth(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginWidth(self.ptr(), margin) }
    }
    fn setMarginMask(&self, margin: c_int, mask: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginMask(self.ptr(), margin, mask) }
    }
    fn getMarginMask(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginMask(self.ptr(), margin) }
    }
    fn setMarginSensitive(&self, margin: c_int, sensitive: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginSensitive(self.ptr(), margin, sensitive) }
    }
    fn getMarginSensitive(&self, margin: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginSensitive(self.ptr(), margin) }
    }
    fn styleClearAll(&self) {
        unsafe { wxStyledTextCtrl_StyleClearAll(self.ptr()) }
    }
    fn styleSetForeground(&self, style: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_StyleSetForeground(self.ptr(), style, fore_r, fore_g, fore_b) }
    }
    fn styleSetBackground(&self, style: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_StyleSetBackground(self.ptr(), style, back_r, back_g, back_b) }
    }
    fn styleSetBold(&self, style: c_int, bold: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetBold(self.ptr(), style, bold) }
    }
    fn styleSetItalic(&self, style: c_int, italic: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetItalic(self.ptr(), style, italic) }
    }
    fn styleSetSize(&self, style: c_int, sizePoints: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetSize(self.ptr(), style, sizePoints) }
    }
    fn styleSetFaceName(&self, style: c_int, fontName: &str) {
        let fontName = strToString(fontName);
        unsafe { wxStyledTextCtrl_StyleSetFaceName(self.ptr(), style, fontName.ptr()) }
    }
    fn styleSetEOLFilled(&self, style: c_int, filled: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetEOLFilled(self.ptr(), style, filled) }
    }
    fn styleResetDefault(&self) {
        unsafe { wxStyledTextCtrl_StyleResetDefault(self.ptr()) }
    }
    fn styleSetUnderline(&self, style: c_int, underline: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetUnderline(self.ptr(), style, underline) }
    }
    fn styleSetCase(&self, style: c_int, caseForce: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetCase(self.ptr(), style, caseForce) }
    }
    fn styleSetCharacterSet(&self, style: c_int, characterSet: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetCharacterSet(self.ptr(), style, characterSet) }
    }
    fn styleSetHotSpot(&self, style: c_int, hotspot: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetHotSpot(self.ptr(), style, hotspot) }
    }
    fn setSelForeground(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelForeground(self.ptr(), useSetting, fore_r, fore_g, fore_b) }
    }
    fn setSelBackground(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetSelBackground(self.ptr(), useSetting, back_r, back_g, back_b) }
    }
    fn setCaretForeground(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretForeground(self.ptr(), fore_r, fore_g, fore_b) }
    }
    fn cmdKeyAssign(&self, key: c_int, modifiers: c_int, cmd: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyAssign(self.ptr(), key, modifiers, cmd) }
    }
    fn cmdKeyClear(&self, key: c_int, modifiers: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyClear(self.ptr(), key, modifiers) }
    }
    fn cmdKeyClearAll(&self) {
        unsafe { wxStyledTextCtrl_CmdKeyClearAll(self.ptr()) }
    }
    fn setStyleBytes(&self, length: c_int, styleBytes: *mut c_char) {
        unsafe { wxStyledTextCtrl_SetStyleBytes(self.ptr(), length, styleBytes) }
    }
    fn styleSetVisible(&self, style: c_int, visible: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetVisible(self.ptr(), style, visible) }
    }
    fn getCaretPeriod(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretPeriod(self.ptr()) }
    }
    fn setCaretPeriod(&self, periodMilliseconds: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretPeriod(self.ptr(), periodMilliseconds) }
    }
    fn setWordChars(&self, characters: &str) {
        let characters = strToString(characters);
        unsafe { wxStyledTextCtrl_SetWordChars(self.ptr(), characters.ptr()) }
    }
    fn beginUndoAction(&self) {
        unsafe { wxStyledTextCtrl_BeginUndoAction(self.ptr()) }
    }
    fn endUndoAction(&self) {
        unsafe { wxStyledTextCtrl_EndUndoAction(self.ptr()) }
    }
    fn indicatorSetStyle(&self, indic: c_int, style: c_int) {
        unsafe { wxStyledTextCtrl_IndicatorSetStyle(self.ptr(), indic, style) }
    }
    fn indicatorGetStyle(&self, indic: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_IndicatorGetStyle(self.ptr(), indic) }
    }
    fn indicatorSetForeground(&self, indic: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_IndicatorSetForeground(self.ptr(), indic, fore_r, fore_g, fore_b) }
    }
    fn setWhitespaceForeground(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceForeground(self.ptr(), useSetting, fore_r, fore_g, fore_b) }
    }
    fn setWhitespaceBackground(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetWhitespaceBackground(self.ptr(), useSetting, back_r, back_g, back_b) }
    }
    fn setStyleBits(&self, bits: c_int) {
        unsafe { wxStyledTextCtrl_SetStyleBits(self.ptr(), bits) }
    }
    fn getStyleBits(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetStyleBits(self.ptr()) }
    }
    fn setLineState(&self, line: c_int, state: c_int) {
        unsafe { wxStyledTextCtrl_SetLineState(self.ptr(), line, state) }
    }
    fn getLineState(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineState(self.ptr(), line) }
    }
    fn getMaxLineState(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMaxLineState(self.ptr()) }
    }
    fn getCaretLineVisible(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretLineVisible(self.ptr()) }
    }
    fn setCaretLineVisible(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretLineVisible(self.ptr(), show) }
    }
    fn styleSetChangeable(&self, style: c_int, changeable: c_int) {
        unsafe { wxStyledTextCtrl_StyleSetChangeable(self.ptr(), style, changeable) }
    }
    fn autoCompShow(&self, lenEntered: c_int, itemList: &str) {
        let itemList = strToString(itemList);
        unsafe { wxStyledTextCtrl_AutoCompShow(self.ptr(), lenEntered, itemList.ptr()) }
    }
    fn autoCompCancel(&self) {
        unsafe { wxStyledTextCtrl_AutoCompCancel(self.ptr()) }
    }
    fn autoCompActive(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompActive(self.ptr()) }
    }
    fn autoCompPosStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompPosStart(self.ptr()) }
    }
    fn autoCompComplete(&self) {
        unsafe { wxStyledTextCtrl_AutoCompComplete(self.ptr()) }
    }
    fn autoCompStops(&self, characterSet: &str) {
        let characterSet = strToString(characterSet);
        unsafe { wxStyledTextCtrl_AutoCompStops(self.ptr(), characterSet.ptr()) }
    }
    fn autoCompSetSeparator(&self, separatorCharacter: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetSeparator(self.ptr(), separatorCharacter) }
    }
    fn autoCompGetSeparator(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetSeparator(self.ptr()) }
    }
    fn autoCompSelect(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_AutoCompSelect(self.ptr(), text.ptr()) }
    }
    fn autoCompSetCancelAtStart(&self, cancel: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetCancelAtStart(self.ptr(), cancel) }
    }
    fn autoCompGetCancelAtStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetCancelAtStart(self.ptr()) }
    }
    fn autoCompSetFillUps(&self, characterSet: &str) {
        let characterSet = strToString(characterSet);
        unsafe { wxStyledTextCtrl_AutoCompSetFillUps(self.ptr(), characterSet.ptr()) }
    }
    fn autoCompSetChooseSingle(&self, chooseSingle: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetChooseSingle(self.ptr(), chooseSingle) }
    }
    fn autoCompGetChooseSingle(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetChooseSingle(self.ptr()) }
    }
    fn autoCompSetIgnoreCase(&self, ignoreCase: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetIgnoreCase(self.ptr(), ignoreCase) }
    }
    fn autoCompGetIgnoreCase(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetIgnoreCase(self.ptr()) }
    }
    fn userListShow(&self, listType: c_int, itemList: &str) {
        let itemList = strToString(itemList);
        unsafe { wxStyledTextCtrl_UserListShow(self.ptr(), listType, itemList.ptr()) }
    }
    fn autoCompSetAutoHide(&self, autoHide: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetAutoHide(self.ptr(), autoHide) }
    }
    fn autoCompGetAutoHide(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetAutoHide(self.ptr()) }
    }
    fn autoCompSetDropRestOfWord(&self, dropRestOfWord: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetDropRestOfWord(self.ptr(), dropRestOfWord) }
    }
    fn autoCompGetDropRestOfWord(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetDropRestOfWord(self.ptr()) }
    }
    fn registerImage<T: BitmapMethods>(&self, type_: c_int, bmp: &T) {
        unsafe { wxStyledTextCtrl_RegisterImage(self.ptr(), type_, bmp.ptr()) }
    }
    fn clearRegisteredImages(&self) {
        unsafe { wxStyledTextCtrl_ClearRegisteredImages(self.ptr()) }
    }
    fn autoCompGetTypeSeparator(&self) -> c_int {
        unsafe { wxStyledTextCtrl_AutoCompGetTypeSeparator(self.ptr()) }
    }
    fn autoCompSetTypeSeparator(&self, separatorCharacter: c_int) {
        unsafe { wxStyledTextCtrl_AutoCompSetTypeSeparator(self.ptr(), separatorCharacter) }
    }
    fn setIndent(&self, indentSize: c_int) {
        unsafe { wxStyledTextCtrl_SetIndent(self.ptr(), indentSize) }
    }
    fn getIndent(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetIndent(self.ptr()) }
    }
    fn setUseTabs(&self, useTabs: c_int) {
        unsafe { wxStyledTextCtrl_SetUseTabs(self.ptr(), useTabs) }
    }
    fn getUseTabs(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUseTabs(self.ptr()) }
    }
    fn setLineIndentation(&self, line: c_int, indentSize: c_int) {
        unsafe { wxStyledTextCtrl_SetLineIndentation(self.ptr(), line, indentSize) }
    }
    fn getLineIndentation(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineIndentation(self.ptr(), line) }
    }
    fn getLineIndentPosition(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineIndentPosition(self.ptr(), line) }
    }
    fn getColumn(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetColumn(self.ptr(), pos) }
    }
    fn setUseHorizontalScrollBar(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetUseHorizontalScrollBar(self.ptr(), show) }
    }
    fn getUseHorizontalScrollBar(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUseHorizontalScrollBar(self.ptr()) }
    }
    fn setIndentationGuides(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetIndentationGuides(self.ptr(), show) }
    }
    fn getIndentationGuides(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetIndentationGuides(self.ptr()) }
    }
    fn setHighlightGuide(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_SetHighlightGuide(self.ptr(), column) }
    }
    fn getHighlightGuide(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetHighlightGuide(self.ptr()) }
    }
    fn getLineEndPosition(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineEndPosition(self.ptr(), line) }
    }
    fn getCodePage(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCodePage(self.ptr()) }
    }
    fn getReadOnly(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetReadOnly(self.ptr()) }
    }
    fn setCurrentPos(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetCurrentPos(self.ptr(), pos) }
    }
    fn setSelectionStart(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetSelectionStart(self.ptr(), pos) }
    }
    fn getSelectionStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSelectionStart(self.ptr()) }
    }
    fn setSelectionEnd(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetSelectionEnd(self.ptr(), pos) }
    }
    fn getSelectionEnd(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSelectionEnd(self.ptr()) }
    }
    fn setPrintMagnification(&self, magnification: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintMagnification(self.ptr(), magnification) }
    }
    fn getPrintMagnification(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintMagnification(self.ptr()) }
    }
    fn setPrintColourMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintColourMode(self.ptr(), mode) }
    }
    fn getPrintColourMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintColourMode(self.ptr()) }
    }
    fn findText(&self, minPos: c_int, maxPos: c_int, text: &str, flags: c_int) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_FindText(self.ptr(), minPos, maxPos, text.ptr(), flags) }
    }
    fn formatRange<T: DCMethods, U: DCMethods, V: RectMethods, W: RectMethods>(&self, doDraw: c_int, startPos: c_int, endPos: c_int, draw: &T, target: &U, renderRect: &V, pageRect: &W) -> c_int {
        unsafe { wxStyledTextCtrl_FormatRange(self.ptr(), doDraw, startPos, endPos, draw.ptr(), target.ptr(), renderRect.ptr(), pageRect.ptr()) }
    }
    fn getFirstVisibleLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetFirstVisibleLine(self.ptr()) }
    }
    fn getLineCount(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineCount(self.ptr()) }
    }
    fn setMarginLeft(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginLeft(self.ptr(), pixelWidth) }
    }
    fn getMarginLeft(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginLeft(self.ptr()) }
    }
    fn setMarginRight(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetMarginRight(self.ptr(), pixelWidth) }
    }
    fn getMarginRight(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMarginRight(self.ptr()) }
    }
    fn getModify(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetModify(self.ptr()) }
    }
    fn setSelection(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_SetSelection(self.ptr(), start, end) }
    }
    fn hideSelection(&self, normal: c_int) {
        unsafe { wxStyledTextCtrl_HideSelection(self.ptr(), normal) }
    }
    fn lineFromPosition(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_LineFromPosition(self.ptr(), pos) }
    }
    fn positionFromLine(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionFromLine(self.ptr(), line) }
    }
    fn lineScroll(&self, columns: c_int, lines: c_int) {
        unsafe { wxStyledTextCtrl_LineScroll(self.ptr(), columns, lines) }
    }
    fn ensureCaretVisible(&self) {
        unsafe { wxStyledTextCtrl_EnsureCaretVisible(self.ptr()) }
    }
    fn replaceSelection(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_ReplaceSelection(self.ptr(), text.ptr()) }
    }
    fn setReadOnly(&self, readOnly: c_int) {
        unsafe { wxStyledTextCtrl_SetReadOnly(self.ptr(), readOnly) }
    }
    fn canPaste(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CanPaste(self.ptr()) }
    }
    fn canUndo(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CanUndo(self.ptr()) }
    }
    fn emptyUndoBuffer(&self) {
        unsafe { wxStyledTextCtrl_EmptyUndoBuffer(self.ptr()) }
    }
    fn undo(&self) {
        unsafe { wxStyledTextCtrl_Undo(self.ptr()) }
    }
    fn cut(&self) {
        unsafe { wxStyledTextCtrl_Cut(self.ptr()) }
    }
    fn copy(&self) {
        unsafe { wxStyledTextCtrl_Copy(self.ptr()) }
    }
    fn paste(&self) {
        unsafe { wxStyledTextCtrl_Paste(self.ptr()) }
    }
    fn clear(&self) {
        unsafe { wxStyledTextCtrl_Clear(self.ptr()) }
    }
    fn setText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_SetText(self.ptr(), text.ptr()) }
    }
    fn getTextLength(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTextLength(self.ptr()) }
    }
    fn setOvertype(&self, overtype: c_int) {
        unsafe { wxStyledTextCtrl_SetOvertype(self.ptr(), overtype) }
    }
    fn getOvertype(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetOvertype(self.ptr()) }
    }
    fn setCaretWidth(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetCaretWidth(self.ptr(), pixelWidth) }
    }
    fn getCaretWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCaretWidth(self.ptr()) }
    }
    fn setTargetStart(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetTargetStart(self.ptr(), pos) }
    }
    fn getTargetStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTargetStart(self.ptr()) }
    }
    fn setTargetEnd(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_SetTargetEnd(self.ptr(), pos) }
    }
    fn getTargetEnd(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTargetEnd(self.ptr()) }
    }
    fn replaceTarget(&self, text: &str) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_ReplaceTarget(self.ptr(), text.ptr()) }
    }
    fn replaceTargetRE(&self, text: &str) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_ReplaceTargetRE(self.ptr(), text.ptr()) }
    }
    fn searchInTarget(&self, text: &str) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_SearchInTarget(self.ptr(), text.ptr()) }
    }
    fn setSearchFlags(&self, flags: c_int) {
        unsafe { wxStyledTextCtrl_SetSearchFlags(self.ptr(), flags) }
    }
    fn getSearchFlags(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSearchFlags(self.ptr()) }
    }
    fn callTipShow(&self, pos: c_int, definition: &str) {
        let definition = strToString(definition);
        unsafe { wxStyledTextCtrl_CallTipShow(self.ptr(), pos, definition.ptr()) }
    }
    fn callTipCancel(&self) {
        unsafe { wxStyledTextCtrl_CallTipCancel(self.ptr()) }
    }
    fn callTipActive(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CallTipActive(self.ptr()) }
    }
    fn callTipPosAtStart(&self) -> c_int {
        unsafe { wxStyledTextCtrl_CallTipPosAtStart(self.ptr()) }
    }
    fn callTipSetHighlight(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_CallTipSetHighlight(self.ptr(), start, end) }
    }
    fn callTipSetBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetBackground(self.ptr(), back_r, back_g, back_b) }
    }
    fn callTipSetForeground(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetForeground(self.ptr(), fore_r, fore_g, fore_b) }
    }
    fn callTipSetForegroundHighlight(&self, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_CallTipSetForegroundHighlight(self.ptr(), fore_r, fore_g, fore_b) }
    }
    fn visibleFromDocLine(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_VisibleFromDocLine(self.ptr(), line) }
    }
    fn docLineFromVisible(&self, lineDisplay: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_DocLineFromVisible(self.ptr(), lineDisplay) }
    }
    fn setFoldLevel(&self, line: c_int, level: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldLevel(self.ptr(), line, level) }
    }
    fn getFoldLevel(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldLevel(self.ptr(), line) }
    }
    fn getLastChild(&self, line: c_int, level: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLastChild(self.ptr(), line, level) }
    }
    fn getFoldParent(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldParent(self.ptr(), line) }
    }
    fn showLines(&self, lineStart: c_int, lineEnd: c_int) {
        unsafe { wxStyledTextCtrl_ShowLines(self.ptr(), lineStart, lineEnd) }
    }
    fn hideLines(&self, lineStart: c_int, lineEnd: c_int) {
        unsafe { wxStyledTextCtrl_HideLines(self.ptr(), lineStart, lineEnd) }
    }
    fn getLineVisible(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetLineVisible(self.ptr(), line) }
    }
    fn setFoldExpanded(&self, line: c_int, expanded: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldExpanded(self.ptr(), line, expanded) }
    }
    fn getFoldExpanded(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_GetFoldExpanded(self.ptr(), line) }
    }
    fn toggleFold(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_ToggleFold(self.ptr(), line) }
    }
    fn ensureVisible(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_EnsureVisible(self.ptr(), line) }
    }
    fn setFoldFlags(&self, flags: c_int) {
        unsafe { wxStyledTextCtrl_SetFoldFlags(self.ptr(), flags) }
    }
    fn ensureVisibleEnforcePolicy(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_EnsureVisibleEnforcePolicy(self.ptr(), line) }
    }
    fn setTabIndents(&self, tabIndents: c_int) {
        unsafe { wxStyledTextCtrl_SetTabIndents(self.ptr(), tabIndents) }
    }
    fn getTabIndents(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTabIndents(self.ptr()) }
    }
    fn setBackSpaceUnIndents(&self, bsUnIndents: c_int) {
        unsafe { wxStyledTextCtrl_SetBackSpaceUnIndents(self.ptr(), bsUnIndents) }
    }
    fn getBackSpaceUnIndents(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetBackSpaceUnIndents(self.ptr()) }
    }
    fn setMouseDwellTime(&self, periodMilliseconds: c_int) {
        unsafe { wxStyledTextCtrl_SetMouseDwellTime(self.ptr(), periodMilliseconds) }
    }
    fn getMouseDwellTime(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMouseDwellTime(self.ptr()) }
    }
    fn wordStartPosition(&self, pos: c_int, onlyWordCharacters: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_WordStartPosition(self.ptr(), pos, onlyWordCharacters) }
    }
    fn wordEndPosition(&self, pos: c_int, onlyWordCharacters: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_WordEndPosition(self.ptr(), pos, onlyWordCharacters) }
    }
    fn setWrapMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetWrapMode(self.ptr(), mode) }
    }
    fn getWrapMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetWrapMode(self.ptr()) }
    }
    fn setLayoutCache(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetLayoutCache(self.ptr(), mode) }
    }
    fn getLayoutCache(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLayoutCache(self.ptr()) }
    }
    fn setScrollWidth(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_SetScrollWidth(self.ptr(), pixelWidth) }
    }
    fn getScrollWidth(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetScrollWidth(self.ptr()) }
    }
    fn textWidth(&self, style: c_int, text: &str) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_TextWidth(self.ptr(), style, text.ptr()) }
    }
    fn setEndAtLastLine(&self, endAtLastLine: c_int) {
        unsafe { wxStyledTextCtrl_SetEndAtLastLine(self.ptr(), endAtLastLine) }
    }
    fn getEndAtLastLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEndAtLastLine(self.ptr()) }
    }
    fn textHeight(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_TextHeight(self.ptr(), line) }
    }
    fn setUseVerticalScrollBar(&self, show: c_int) {
        unsafe { wxStyledTextCtrl_SetUseVerticalScrollBar(self.ptr(), show) }
    }
    fn getUseVerticalScrollBar(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetUseVerticalScrollBar(self.ptr()) }
    }
    fn appendText(&self, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_AppendText(self.ptr(), text.ptr()) }
    }
    fn getTwoPhaseDraw(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetTwoPhaseDraw(self.ptr()) }
    }
    fn setTwoPhaseDraw(&self, twoPhase: c_int) {
        unsafe { wxStyledTextCtrl_SetTwoPhaseDraw(self.ptr(), twoPhase) }
    }
    fn targetFromSelection(&self) {
        unsafe { wxStyledTextCtrl_TargetFromSelection(self.ptr()) }
    }
    fn linesJoin(&self) {
        unsafe { wxStyledTextCtrl_LinesJoin(self.ptr()) }
    }
    fn linesSplit(&self, pixelWidth: c_int) {
        unsafe { wxStyledTextCtrl_LinesSplit(self.ptr(), pixelWidth) }
    }
    fn setFoldMarginColour(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginColour(self.ptr(), useSetting, back_r, back_g, back_b) }
    }
    fn setFoldMarginHiColour(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetFoldMarginHiColour(self.ptr(), useSetting, fore_r, fore_g, fore_b) }
    }
    fn lineDuplicate(&self) {
        unsafe { wxStyledTextCtrl_LineDuplicate(self.ptr()) }
    }
    fn homeDisplay(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplay(self.ptr()) }
    }
    fn homeDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_HomeDisplayExtend(self.ptr()) }
    }
    fn lineEndDisplay(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplay(self.ptr()) }
    }
    fn lineEndDisplayExtend(&self) {
        unsafe { wxStyledTextCtrl_LineEndDisplayExtend(self.ptr()) }
    }
    fn lineCopy(&self) {
        unsafe { wxStyledTextCtrl_LineCopy(self.ptr()) }
    }
    fn moveCaretInsideView(&self) {
        unsafe { wxStyledTextCtrl_MoveCaretInsideView(self.ptr()) }
    }
    fn lineLength(&self, line: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_LineLength(self.ptr(), line) }
    }
    fn braceHighlight(&self, pos1: c_int, pos2: c_int) {
        unsafe { wxStyledTextCtrl_BraceHighlight(self.ptr(), pos1, pos2) }
    }
    fn braceBadLight(&self, pos: c_int) {
        unsafe { wxStyledTextCtrl_BraceBadLight(self.ptr(), pos) }
    }
    fn braceMatch(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_BraceMatch(self.ptr(), pos) }
    }
    fn getViewEOL(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetViewEOL(self.ptr()) }
    }
    fn setViewEOL(&self, visible: c_int) {
        unsafe { wxStyledTextCtrl_SetViewEOL(self.ptr(), visible) }
    }
    fn setDocPointer<T: STCDocMethods>(&self, docPointer: &T) {
        unsafe { wxStyledTextCtrl_SetDocPointer(self.ptr(), docPointer.ptr()) }
    }
    fn setModEventMask(&self, mask: c_int) {
        unsafe { wxStyledTextCtrl_SetModEventMask(self.ptr(), mask) }
    }
    fn getEdgeColumn(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEdgeColumn(self.ptr()) }
    }
    fn setEdgeColumn(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_SetEdgeColumn(self.ptr(), column) }
    }
    fn getEdgeMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetEdgeMode(self.ptr()) }
    }
    fn setEdgeMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetEdgeMode(self.ptr(), mode) }
    }
    fn setEdgeColour(&self, edgeColour_r: uint8_t, edgeColour_g: uint8_t, edgeColour_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetEdgeColour(self.ptr(), edgeColour_r, edgeColour_g, edgeColour_b) }
    }
    fn searchAnchor(&self) {
        unsafe { wxStyledTextCtrl_SearchAnchor(self.ptr()) }
    }
    fn searchNext(&self, flags: c_int, text: &str) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_SearchNext(self.ptr(), flags, text.ptr()) }
    }
    fn searchPrev(&self, flags: c_int, text: &str) -> c_int {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_SearchPrev(self.ptr(), flags, text.ptr()) }
    }
    fn linesOnScreen(&self) -> c_int {
        unsafe { wxStyledTextCtrl_LinesOnScreen(self.ptr()) }
    }
    fn usePopUp(&self, allowPopUp: c_int) {
        unsafe { wxStyledTextCtrl_UsePopUp(self.ptr(), allowPopUp) }
    }
    fn selectionIsRectangle(&self) -> c_int {
        unsafe { wxStyledTextCtrl_SelectionIsRectangle(self.ptr()) }
    }
    fn setZoom(&self, zoom: c_int) {
        unsafe { wxStyledTextCtrl_SetZoom(self.ptr(), zoom) }
    }
    fn getZoom(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetZoom(self.ptr()) }
    }
    fn addRefDocument<T: STCDocMethods>(&self, docPointer: &T) {
        unsafe { wxStyledTextCtrl_AddRefDocument(self.ptr(), docPointer.ptr()) }
    }
    fn releaseDocument<T: STCDocMethods>(&self, docPointer: &T) {
        unsafe { wxStyledTextCtrl_ReleaseDocument(self.ptr(), docPointer.ptr()) }
    }
    fn getModEventMask(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetModEventMask(self.ptr()) }
    }
    fn setSTCFocus(&self, focus: c_int) {
        unsafe { wxStyledTextCtrl_SetSTCFocus(self.ptr(), focus) }
    }
    fn getSTCFocus(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSTCFocus(self.ptr()) }
    }
    fn setStatus(&self, statusCode: c_int) {
        unsafe { wxStyledTextCtrl_SetStatus(self.ptr(), statusCode) }
    }
    fn getStatus(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetStatus(self.ptr()) }
    }
    fn setMouseDownCaptures(&self, captures: c_int) {
        unsafe { wxStyledTextCtrl_SetMouseDownCaptures(self.ptr(), captures) }
    }
    fn getMouseDownCaptures(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetMouseDownCaptures(self.ptr()) }
    }
    fn setSTCCursor(&self, cursorType: c_int) {
        unsafe { wxStyledTextCtrl_SetSTCCursor(self.ptr(), cursorType) }
    }
    fn getSTCCursor(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetSTCCursor(self.ptr()) }
    }
    fn setControlCharSymbol(&self, symbol: c_int) {
        unsafe { wxStyledTextCtrl_SetControlCharSymbol(self.ptr(), symbol) }
    }
    fn getControlCharSymbol(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetControlCharSymbol(self.ptr()) }
    }
    fn wordPartLeft(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeft(self.ptr()) }
    }
    fn wordPartLeftExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartLeftExtend(self.ptr()) }
    }
    fn wordPartRight(&self) {
        unsafe { wxStyledTextCtrl_WordPartRight(self.ptr()) }
    }
    fn wordPartRightExtend(&self) {
        unsafe { wxStyledTextCtrl_WordPartRightExtend(self.ptr()) }
    }
    fn setVisiblePolicy(&self, visiblePolicy: c_int, visibleSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetVisiblePolicy(self.ptr(), visiblePolicy, visibleSlop) }
    }
    fn delLineLeft(&self) {
        unsafe { wxStyledTextCtrl_DelLineLeft(self.ptr()) }
    }
    fn delLineRight(&self) {
        unsafe { wxStyledTextCtrl_DelLineRight(self.ptr()) }
    }
    fn setXOffset(&self, newOffset: c_int) {
        unsafe { wxStyledTextCtrl_SetXOffset(self.ptr(), newOffset) }
    }
    fn getXOffset(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetXOffset(self.ptr()) }
    }
    fn chooseCaretX(&self) {
        unsafe { wxStyledTextCtrl_ChooseCaretX(self.ptr()) }
    }
    fn setXCaretPolicy(&self, caretPolicy: c_int, caretSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetXCaretPolicy(self.ptr(), caretPolicy, caretSlop) }
    }
    fn setYCaretPolicy(&self, caretPolicy: c_int, caretSlop: c_int) {
        unsafe { wxStyledTextCtrl_SetYCaretPolicy(self.ptr(), caretPolicy, caretSlop) }
    }
    fn setPrintWrapMode(&self, mode: c_int) {
        unsafe { wxStyledTextCtrl_SetPrintWrapMode(self.ptr(), mode) }
    }
    fn getPrintWrapMode(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetPrintWrapMode(self.ptr()) }
    }
    fn setHotspotActiveForeground(&self, useSetting: c_int, fore_r: uint8_t, fore_g: uint8_t, fore_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveForeground(self.ptr(), useSetting, fore_r, fore_g, fore_b) }
    }
    fn setHotspotActiveBackground(&self, useSetting: c_int, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveBackground(self.ptr(), useSetting, back_r, back_g, back_b) }
    }
    fn setHotspotActiveUnderline(&self, underline: c_int) {
        unsafe { wxStyledTextCtrl_SetHotspotActiveUnderline(self.ptr(), underline) }
    }
    fn positionBefore(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionBefore(self.ptr(), pos) }
    }
    fn positionAfter(&self, pos: c_int) -> c_int {
        unsafe { wxStyledTextCtrl_PositionAfter(self.ptr(), pos) }
    }
    fn copyRange(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_CopyRange(self.ptr(), start, end) }
    }
    fn copyText(&self, length: c_int, text: &str) {
        let text = strToString(text);
        unsafe { wxStyledTextCtrl_CopyText(self.ptr(), length, text.ptr()) }
    }
    fn startRecord(&self) {
        unsafe { wxStyledTextCtrl_StartRecord(self.ptr()) }
    }
    fn stopRecord(&self) {
        unsafe { wxStyledTextCtrl_StopRecord(self.ptr()) }
    }
    fn setLexer(&self, lexer: c_int) {
        unsafe { wxStyledTextCtrl_SetLexer(self.ptr(), lexer) }
    }
    fn getLexer(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLexer(self.ptr()) }
    }
    fn colourise(&self, start: c_int, end: c_int) {
        unsafe { wxStyledTextCtrl_Colourise(self.ptr(), start, end) }
    }
    fn setProperty(&self, key: &str, value: &str) {
        let key = strToString(key);
        let value = strToString(value);
        unsafe { wxStyledTextCtrl_SetProperty(self.ptr(), key.ptr(), value.ptr()) }
    }
    fn setKeyWords(&self, keywordSet: c_int, keyWords: &str) {
        let keyWords = strToString(keyWords);
        unsafe { wxStyledTextCtrl_SetKeyWords(self.ptr(), keywordSet, keyWords.ptr()) }
    }
    fn setLexerLanguage(&self, language: &str) {
        let language = strToString(language);
        unsafe { wxStyledTextCtrl_SetLexerLanguage(self.ptr(), language.ptr()) }
    }
    fn getCurrentLine(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetCurrentLine(self.ptr()) }
    }
    fn styleSetSpec(&self, styleNum: c_int, spec: &str) {
        let spec = strToString(spec);
        unsafe { wxStyledTextCtrl_StyleSetSpec(self.ptr(), styleNum, spec.ptr()) }
    }
    fn styleSetFont<T: FontMethods>(&self, styleNum: c_int, font: &T) {
        unsafe { wxStyledTextCtrl_StyleSetFont(self.ptr(), styleNum, font.ptr()) }
    }
    fn styleSetFontAttr(&self, styleNum: c_int, size: c_int, faceName: &str, bold: c_int, italic: c_int, underline: c_int) {
        let faceName = strToString(faceName);
        unsafe { wxStyledTextCtrl_StyleSetFontAttr(self.ptr(), styleNum, size, faceName.ptr(), bold, italic, underline) }
    }
    fn cmdKeyExecute(&self, cmd: c_int) {
        unsafe { wxStyledTextCtrl_CmdKeyExecute(self.ptr(), cmd) }
    }
    fn setMargins(&self, left: c_int, right: c_int) {
        unsafe { wxStyledTextCtrl_SetMargins(self.ptr(), left, right) }
    }
    fn getSelection(&self, startPos: *mut c_int, endPos: *mut c_int) {
        unsafe { wxStyledTextCtrl_GetSelection(self.ptr(), startPos, endPos) }
    }
    fn scrollToLine(&self, line: c_int) {
        unsafe { wxStyledTextCtrl_ScrollToLine(self.ptr(), line) }
    }
    fn scrollToColumn(&self, column: c_int) {
        unsafe { wxStyledTextCtrl_ScrollToColumn(self.ptr(), column) }
    }
    fn setVScrollBar<T: ScrollBarMethods>(&self, bar: &T) {
        unsafe { wxStyledTextCtrl_SetVScrollBar(self.ptr(), bar.ptr()) }
    }
    fn setHScrollBar<T: ScrollBarMethods>(&self, bar: &T) {
        unsafe { wxStyledTextCtrl_SetHScrollBar(self.ptr(), bar.ptr()) }
    }
    fn getLastKeydownProcessed(&self) -> c_int {
        unsafe { wxStyledTextCtrl_GetLastKeydownProcessed(self.ptr()) }
    }
    fn setLastKeydownProcessed(&self, val: c_int) {
        unsafe { wxStyledTextCtrl_SetLastKeydownProcessed(self.ptr(), val) }
    }
    fn saveFile(&self, filename: &str) -> c_int {
        let filename = strToString(filename);
        unsafe { wxStyledTextCtrl_SaveFile(self.ptr(), filename.ptr()) }
    }
    fn loadFile(&self, filename: &str) -> c_int {
        let filename = strToString(filename);
        unsafe { wxStyledTextCtrl_LoadFile(self.ptr(), filename.ptr()) }
    }
    fn indicatorGetForeground(&self, indic: c_int) -> Colour {
        unsafe { Colour::from(wxStyledTextCtrl_IndicatorGetForeground(self.ptr(), indic)) }
    }
    fn getCaretLineBackground(&self) -> Colour {
        unsafe { Colour::from(wxStyledTextCtrl_GetCaretLineBackground(self.ptr())) }
    }
    fn setCaretLineBackground(&self, back_r: uint8_t, back_g: uint8_t, back_b: uint8_t) {
        unsafe { wxStyledTextCtrl_SetCaretLineBackground(self.ptr(), back_r, back_g, back_b) }
    }
    fn getCaretForeground(&self) -> Colour {
        unsafe { Colour::from(wxStyledTextCtrl_GetCaretForeground(self.ptr())) }
    }
    fn getLine(&self, line: c_int) -> ~str {
        unsafe { String::from(wxStyledTextCtrl_GetLine(self.ptr(), line)).to_str() }
    }
    fn getText(&self) -> ~str {
        unsafe { String::from(wxStyledTextCtrl_GetText(self.ptr())).to_str() }
    }
    fn getTextRange(&self, startPos: c_int, endPos: c_int) -> ~str {
        unsafe { String::from(wxStyledTextCtrl_GetTextRange(self.ptr(), startPos, endPos)).to_str() }
    }
    fn getSelectedText(&self) -> ~str {
        unsafe { String::from(wxStyledTextCtrl_GetSelectedText(self.ptr())).to_str() }
    }
    fn newDocument(&self) -> STCDoc {
        unsafe { STCDoc::from(wxStyledTextCtrl_CreateDocument(self.ptr())) }
    }
    fn getEdgeColour(&self) -> Colour {
        unsafe { Colour::from(wxStyledTextCtrl_GetEdgeColour(self.ptr())) }
    }
    fn getDocPointer(&self) -> STCDoc {
        unsafe { STCDoc::from(wxStyledTextCtrl_GetDocPointer(self.ptr())) }
    }
    fn pointFromPosition(&self) -> Point {
        unsafe { Point::from(wxStyledTextCtrl_PointFromPosition(self.ptr())) }
    }
}

/// Wraps the wxWidgets' [wxSTCDoc](http://docs.wxwidgets.org/3.0/classwx_stcd_oc.html) class.
pub struct STCDoc { ptr: *mut c_void }
impl STCDocMethods for STCDoc { fn ptr(&self) -> *mut c_void { self.ptr } }

impl STCDoc {
    pub fn from(ptr: *mut c_void) -> STCDoc { STCDoc { ptr: ptr } }
    pub fn null() -> STCDoc { STCDoc::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSTCDoc](http://docs.wxwidgets.org/3.0/classwx_stcd_oc.html) class.
pub trait STCDocMethods {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxStyledTextEvent](http://docs.wxwidgets.org/3.0/classwx_styled_text_event.html) class.
pub struct StyledTextEvent { ptr: *mut c_void }
impl StyledTextEventMethods for StyledTextEvent {}
impl CommandEventMethods for StyledTextEvent {}
impl EventMethods for StyledTextEvent {}
impl ObjectMethods for StyledTextEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StyledTextEvent {
    pub fn from(ptr: *mut c_void) -> StyledTextEvent { StyledTextEvent { ptr: ptr } }
    pub fn null() -> StyledTextEvent { StyledTextEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStyledTextEvent](http://docs.wxwidgets.org/3.0/classwx_styled_text_event.html) class.
pub trait StyledTextEventMethods : CommandEventMethods {
    fn getPosition(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetPosition(self.ptr()) }
    }
    fn getKey(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetKey(self.ptr()) }
    }
    fn getModifiers(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetModifiers(self.ptr()) }
    }
    fn getModificationType(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetModificationType(self.ptr()) }
    }
    fn getLength(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLength(self.ptr()) }
    }
    fn getLinesAdded(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLinesAdded(self.ptr()) }
    }
    fn getLine(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLine(self.ptr()) }
    }
    fn getFoldLevelNow(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetFoldLevelNow(self.ptr()) }
    }
    fn getFoldLevelPrev(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetFoldLevelPrev(self.ptr()) }
    }
    fn getMargin(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetMargin(self.ptr()) }
    }
    fn getMessage(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetMessage(self.ptr()) }
    }
    fn getWParam(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetWParam(self.ptr()) }
    }
    fn getLParam(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetLParam(self.ptr()) }
    }
    fn getListType(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetListType(self.ptr()) }
    }
    fn getX(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetX(self.ptr()) }
    }
    fn getY(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetY(self.ptr()) }
    }
    fn getDragText(&self) -> ~str {
        unsafe { String::from(wxStyledTextEvent_GetDragText(self.ptr())).to_str() }
    }
    fn getDragAllowMove(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetDragAllowMove(self.ptr()) }
    }
    fn getDragResult(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetDragResult(self.ptr()) }
    }
    fn getShift(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetShift(self.ptr()) }
    }
    fn getControl(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetControl(self.ptr()) }
    }
    fn getAlt(&self) -> c_int {
        unsafe { wxStyledTextEvent_GetAlt(self.ptr()) }
    }
    fn getText(&self) -> ~str {
        unsafe { String::from(wxStyledTextEvent_GetText(self.ptr())).to_str() }
    }
    fn clone(&self) -> StyledTextEvent {
        unsafe { StyledTextEvent::from(wxStyledTextEvent_Clone(self.ptr())) }
    }
    fn setPosition(&self, pos: c_int) {
        unsafe { wxStyledTextEvent_SetPosition(self.ptr(), pos) }
    }
    fn setKey(&self, k: c_int) {
        unsafe { wxStyledTextEvent_SetKey(self.ptr(), k) }
    }
    fn setModifiers(&self, m: c_int) {
        unsafe { wxStyledTextEvent_SetModifiers(self.ptr(), m) }
    }
    fn setModificationType(&self, t: c_int) {
        unsafe { wxStyledTextEvent_SetModificationType(self.ptr(), t) }
    }
    fn setText(&self, t: &str) {
        let t = strToString(t);
        unsafe { wxStyledTextEvent_SetText(self.ptr(), t.ptr()) }
    }
    fn setLength(&self, len: c_int) {
        unsafe { wxStyledTextEvent_SetLength(self.ptr(), len) }
    }
    fn setLinesAdded(&self, num: c_int) {
        unsafe { wxStyledTextEvent_SetLinesAdded(self.ptr(), num) }
    }
    fn setLine(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetLine(self.ptr(), val) }
    }
    fn setFoldLevelNow(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetFoldLevelNow(self.ptr(), val) }
    }
    fn setFoldLevelPrev(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetFoldLevelPrev(self.ptr(), val) }
    }
    fn setMargin(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetMargin(self.ptr(), val) }
    }
    fn setMessage(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetMessage(self.ptr(), val) }
    }
    fn setWParam(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetWParam(self.ptr(), val) }
    }
    fn setLParam(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetLParam(self.ptr(), val) }
    }
    fn setListType(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetListType(self.ptr(), val) }
    }
    fn setX(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetX(self.ptr(), val) }
    }
    fn setY(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetY(self.ptr(), val) }
    }
    fn setDragText(&self, val: &str) {
        let val = strToString(val);
        unsafe { wxStyledTextEvent_SetDragText(self.ptr(), val.ptr()) }
    }
    fn setDragAllowMove(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetDragAllowMove(self.ptr(), val) }
    }
    fn setDragResult(&self, val: c_int) {
        unsafe { wxStyledTextEvent_SetDragResult(self.ptr(), val) }
    }
}

