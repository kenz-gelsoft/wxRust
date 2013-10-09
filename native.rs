use std::libc::*;

#[link_args="-lwxc"]
extern {
    pub fn Null_AcceleratorTable() -> *u8 /* void* */;
    pub fn Null_Bitmap() -> *u8 /* void* */;
    pub fn Null_Brush() -> *u8 /* void* */;
    pub fn Null_Colour() -> *u8 /* void* */;
    pub fn Null_Cursor() -> *u8 /* void* */;
    pub fn Null_Font() -> *u8 /* void* */;
    pub fn Null_Icon() -> *u8 /* void* */;
    pub fn Null_Palette() -> *u8 /* void* */;
    pub fn Null_Pen() -> *u8 /* void* */;
    pub fn expEVT_COMMAND_AUITOOLBAR_TOOL_DROPDOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUITOOLBAR_OVERFLOW_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUITOOLBAR_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUITOOLBAR_MIDDLE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUITOOLBAR_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CLOSE() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CLOSED() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_BUTTON() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_END_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_DRAG_MOTION() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_ALLOW_DND() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_MIDDLE_DOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_MIDDLE_UP() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_RIGHT_DOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_TAB_RIGHT_UP() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_DRAG_DONE() -> c_int /* int */;
    pub fn expEVT_COMMAND_AUINOTEBOOK_BG_DCLICK() -> c_int /* int */;
    pub fn expEVT_AUI_PANE_BUTTON() -> c_int /* int */;
    pub fn expEVT_AUI_PANE_CLOSE() -> c_int /* int */;
    pub fn expEVT_AUI_PANE_MAXIMIZE() -> c_int /* int */;
    pub fn expEVT_AUI_PANE_RESTORE() -> c_int /* int */;
    pub fn expEVT_AUI_RENDER() -> c_int /* int */;
    pub fn expEVT_AUI_FIND_MANAGER() -> c_int /* int */;
    pub fn expEVT_CALENDAR_SEL_CHANGED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_DOUBLECLICKED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_WEEKDAY_CLICKED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_WEEK_CLICKED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_DAY_CHANGED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_MONTH_CHANGED() -> c_int /* int */;
    pub fn expEVT_CALENDAR_YEAR_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_CHOICEBOOK_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_CHOICEBOOK_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_CLIPBOARD_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_COLOURPICKER_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_COLLPANE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_SELECTION_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_ACTIVATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_COLLAPSED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EXPANDED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_COLLAPSING() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EXPANDING() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_START_EDITING() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EDITING_STARTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_EDITING_DONE() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_VALUE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_CONTEXT_MENU() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_HEADER_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_SORTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_COLUMN_REORDERED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_CACHE_HINT() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_DROP_POSSIBLE() -> c_int /* int */;
    pub fn expEVT_COMMAND_DATAVIEW_ITEM_DROP() -> c_int /* int */;
    pub fn expEVT_DATE_CHANGED() -> c_int /* int */;
    pub fn expEVT_WINDOW_MODAL_DIALOG_CLOSED() -> c_int /* int */;
    pub fn expEVT_COMMAND_BUTTON_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_CHECKBOX_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_CHOICE_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LISTBOX_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LISTBOX_DOUBLECLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_CHECKLISTBOX_TOGGLED() -> c_int /* int */;
    pub fn expEVT_COMMAND_MENU_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_SLIDER_UPDATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RADIOBOX_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RADIOBUTTON_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_VLBOX_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_COMBOBOX_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOOL_RCLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOOL_DROPDOWN_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOOL_ENTER() -> c_int /* int */;
    pub fn expEVT_COMMAND_COMBOBOX_DROPDOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_COMBOBOX_CLOSEUP() -> c_int /* int */;
    pub fn expEVT_COMMAND_THREAD() -> c_int /* int */;
    pub fn expEVT_LEFT_DOWN() -> c_int /* int */;
    pub fn expEVT_LEFT_UP() -> c_int /* int */;
    pub fn expEVT_MIDDLE_DOWN() -> c_int /* int */;
    pub fn expEVT_MIDDLE_UP() -> c_int /* int */;
    pub fn expEVT_RIGHT_DOWN() -> c_int /* int */;
    pub fn expEVT_RIGHT_UP() -> c_int /* int */;
    pub fn expEVT_MOTION() -> c_int /* int */;
    pub fn expEVT_ENTER_WINDOW() -> c_int /* int */;
    pub fn expEVT_LEAVE_WINDOW() -> c_int /* int */;
    pub fn expEVT_LEFT_DCLICK() -> c_int /* int */;
    pub fn expEVT_MIDDLE_DCLICK() -> c_int /* int */;
    pub fn expEVT_RIGHT_DCLICK() -> c_int /* int */;
    pub fn expEVT_SET_FOCUS() -> c_int /* int */;
    pub fn expEVT_KILL_FOCUS() -> c_int /* int */;
    pub fn expEVT_CHILD_FOCUS() -> c_int /* int */;
    pub fn expEVT_MOUSEWHEEL() -> c_int /* int */;
    pub fn expEVT_AUX1_DOWN() -> c_int /* int */;
    pub fn expEVT_AUX1_UP() -> c_int /* int */;
    pub fn expEVT_AUX1_DCLICK() -> c_int /* int */;
    pub fn expEVT_AUX2_DOWN() -> c_int /* int */;
    pub fn expEVT_AUX2_UP() -> c_int /* int */;
    pub fn expEVT_AUX2_DCLICK() -> c_int /* int */;
    pub fn expEVT_CHAR() -> c_int /* int */;
    pub fn expEVT_CHAR_HOOK() -> c_int /* int */;
    pub fn expEVT_NAVIGATION_KEY() -> c_int /* int */;
    pub fn expEVT_KEY_DOWN() -> c_int /* int */;
    pub fn expEVT_KEY_UP() -> c_int /* int */;
    pub fn expEVT_HOTKEY() -> c_int /* int */;
    pub fn expEVT_SET_CURSOR() -> c_int /* int */;
    pub fn expEVT_SCROLL_TOP() -> c_int /* int */;
    pub fn expEVT_SCROLL_BOTTOM() -> c_int /* int */;
    pub fn expEVT_SCROLL_LINEUP() -> c_int /* int */;
    pub fn expEVT_SCROLL_LINEDOWN() -> c_int /* int */;
    pub fn expEVT_SCROLL_PAGEUP() -> c_int /* int */;
    pub fn expEVT_SCROLL_PAGEDOWN() -> c_int /* int */;
    pub fn expEVT_SCROLL_THUMBTRACK() -> c_int /* int */;
    pub fn expEVT_SCROLL_THUMBRELEASE() -> c_int /* int */;
    pub fn expEVT_SCROLL_CHANGED() -> c_int /* int */;
    pub fn expEVT_SPIN_UP() -> c_int /* int */;
    pub fn expEVT_SPIN_DOWN() -> c_int /* int */;
    pub fn expEVT_SPIN() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_TOP() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_BOTTOM() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_LINEUP() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_LINEDOWN() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_PAGEUP() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_PAGEDOWN() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_THUMBTRACK() -> c_int /* int */;
    pub fn expEVT_SCROLLWIN_THUMBRELEASE() -> c_int /* int */;
    pub fn expEVT_SIZE() -> c_int /* int */;
    pub fn expEVT_MOVE() -> c_int /* int */;
    pub fn expEVT_CLOSE_WINDOW() -> c_int /* int */;
    pub fn expEVT_END_SESSION() -> c_int /* int */;
    pub fn expEVT_QUERY_END_SESSION() -> c_int /* int */;
    pub fn expEVT_ACTIVATE_APP() -> c_int /* int */;
    pub fn expEVT_ACTIVATE() -> c_int /* int */;
    pub fn expEVT_CREATE() -> c_int /* int */;
    pub fn expEVT_DESTROY() -> c_int /* int */;
    pub fn expEVT_SHOW() -> c_int /* int */;
    pub fn expEVT_ICONIZE() -> c_int /* int */;
    pub fn expEVT_MAXIMIZE() -> c_int /* int */;
    pub fn expEVT_MOUSE_CAPTURE_CHANGED() -> c_int /* int */;
    pub fn expEVT_MOUSE_CAPTURE_LOST() -> c_int /* int */;
    pub fn expEVT_PAINT() -> c_int /* int */;
    pub fn expEVT_ERASE_BACKGROUND() -> c_int /* int */;
    pub fn expEVT_NC_PAINT() -> c_int /* int */;
    pub fn expEVT_MENU_OPEN() -> c_int /* int */;
    pub fn expEVT_MENU_CLOSE() -> c_int /* int */;
    pub fn expEVT_MENU_HIGHLIGHT() -> c_int /* int */;
    pub fn expEVT_CONTEXT_MENU() -> c_int /* int */;
    pub fn expEVT_SYS_COLOUR_CHANGED() -> c_int /* int */;
    pub fn expEVT_DISPLAY_CHANGED() -> c_int /* int */;
    pub fn expEVT_QUERY_NEW_PALETTE() -> c_int /* int */;
    pub fn expEVT_PALETTE_CHANGED() -> c_int /* int */;
    pub fn expEVT_JOY_BUTTON_DOWN() -> c_int /* int */;
    pub fn expEVT_JOY_BUTTON_UP() -> c_int /* int */;
    pub fn expEVT_JOY_MOVE() -> c_int /* int */;
    pub fn expEVT_JOY_ZMOVE() -> c_int /* int */;
    pub fn expEVT_DROP_FILES() -> c_int /* int */;
    pub fn expEVT_INIT_DIALOG() -> c_int /* int */;
    pub fn expEVT_IDLE() -> c_int /* int */;
    pub fn expEVT_UPDATE_UI() -> c_int /* int */;
    pub fn expEVT_SIZING() -> c_int /* int */;
    pub fn expEVT_MOVING() -> c_int /* int */;
    pub fn expEVT_MOVE_START() -> c_int /* int */;
    pub fn expEVT_MOVE_END() -> c_int /* int */;
    pub fn expEVT_HIBERNATE() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_COPY() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_CUT() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_PASTE() -> c_int /* int */;
    pub fn expEVT_COMMAND_LEFT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_LEFT_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIGHT_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_SET_FOCUS() -> c_int /* int */;
    pub fn expEVT_COMMAND_KILL_FOCUS() -> c_int /* int */;
    pub fn expEVT_COMMAND_ENTER() -> c_int /* int */;
    pub fn expEVT_HELP() -> c_int /* int */;
    pub fn expEVT_DETAILED_HELP() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOOL_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_FIND() -> c_int /* int */;
    pub fn expEVT_COMMAND_FIND_NEXT() -> c_int /* int */;
    pub fn expEVT_COMMAND_FIND_REPLACE() -> c_int /* int */;
    pub fn expEVT_COMMAND_FIND_REPLACE_ALL() -> c_int /* int */;
    pub fn expEVT_COMMAND_FIND_CLOSE() -> c_int /* int */;
    pub fn expEVT_FILECTRL_SELECTIONCHANGED() -> c_int /* int */;
    pub fn expEVT_FILECTRL_FILEACTIVATED() -> c_int /* int */;
    pub fn expEVT_FILECTRL_FOLDERCHANGED() -> c_int /* int */;
    pub fn expEVT_FILECTRL_FILTERCHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_FILEPICKER_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_DIRPICKER_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_FONTPICKER_CHANGED() -> c_int /* int */;
    pub fn expEVT_FSWATCHER() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_LEFT_CLICK() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_LEFT_DCLICK() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_RIGHT_DCLICK() -> c_int /* int */;
    pub fn expEVT_GRID_LABEL_LEFT_CLICK() -> c_int /* int */;
    pub fn expEVT_GRID_LABEL_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_GRID_LABEL_LEFT_DCLICK() -> c_int /* int */;
    pub fn expEVT_GRID_LABEL_RIGHT_DCLICK() -> c_int /* int */;
    pub fn expEVT_GRID_ROW_SIZE() -> c_int /* int */;
    pub fn expEVT_GRID_COL_SIZE() -> c_int /* int */;
    pub fn expEVT_GRID_RANGE_SELECT() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_CHANGING() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_CHANGED() -> c_int /* int */;
    pub fn expEVT_GRID_SELECT_CELL() -> c_int /* int */;
    pub fn expEVT_GRID_EDITOR_SHOWN() -> c_int /* int */;
    pub fn expEVT_GRID_EDITOR_HIDDEN() -> c_int /* int */;
    pub fn expEVT_GRID_EDITOR_CREATED() -> c_int /* int */;
    pub fn expEVT_GRID_CELL_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_GRID_COL_MOVE() -> c_int /* int */;
    pub fn expEVT_GRID_COL_SORT() -> c_int /* int */;
    pub fn expEVT_QUERY_LAYOUT_INFO() -> c_int /* int */;
    pub fn expEVT_CALCULATE_LAYOUT() -> c_int /* int */;
    pub fn expEVT_SASH_DRAGGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_MIDDLE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_RIGHT_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_MIDDLE_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_SEPARATOR_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_BEGIN_RESIZE() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_RESIZING() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_END_RESIZE() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_BEGIN_REORDER() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_END_REORDER() -> c_int /* int */;
    pub fn expEVT_COMMAND_HEADER_DRAGGING_CANCELLED() -> c_int /* int */;
    pub fn expEVT_COMMAND_HTML_CELL_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_HTML_CELL_HOVER() -> c_int /* int */;
    pub fn expEVT_COMMAND_HTML_LINK_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_HYPERLINK() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_BEGIN_RDRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_BEGIN_LABEL_EDIT() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_END_LABEL_EDIT() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_DELETE_ITEM() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_DELETE_ALL_ITEMS() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_ITEM_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_ITEM_DESELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_KEY_DOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_INSERT_ITEM() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_COL_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_ITEM_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_ITEM_MIDDLE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_ITEM_ACTIVATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_CACHE_HINT() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_COL_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_COL_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_COL_DRAGGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_COL_END_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_LIST_ITEM_FOCUSED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LISTBOOK_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_LISTBOOK_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_NOTEBOOK_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_NOTEBOOK_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_POWER_SUSPENDING() -> c_int /* int */;
    pub fn expEVT_POWER_SUSPENDED() -> c_int /* int */;
    pub fn expEVT_POWER_SUSPEND_CANCEL() -> c_int /* int */;
    pub fn expEVT_POWER_RESUME() -> c_int /* int */;
    pub fn expEVT_END_PROCESS() -> c_int /* int */;
    pub fn expEVT_PG_SELECTED() -> c_int /* int */;
    pub fn expEVT_PG_CHANGING() -> c_int /* int */;
    pub fn expEVT_PG_CHANGED() -> c_int /* int */;
    pub fn expEVT_PG_HIGHLIGHTED() -> c_int /* int */;
    pub fn expEVT_PG_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_PG_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_PG_ITEM_COLLAPSED() -> c_int /* int */;
    pub fn expEVT_PG_ITEM_EXPANDED() -> c_int /* int */;
    pub fn expEVT_PG_DOUBLE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBAR_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBAR_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_MIDDLE_DOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_MIDDLE_UP() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_RIGHT_DOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBAR_TAB_RIGHT_UP() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBUTTON_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONBUTTON_DROPDOWN_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONGALLERY_HOVER_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONGALLERY_SELECTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONTOOL_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RIBBONTOOL_DROPDOWN_CLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_LEFT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_MIDDLE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_LEFT_DCLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_RETURN() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_CHARACTER() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_DELETE() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_REPLACING() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_STYLESHEET_REPLACED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_CONTENT_INSERTED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_CONTENT_DELETED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_STYLE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_SELECTION_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_RICHTEXT_BUFFER_RESET() -> c_int /* int */;
    pub fn expEVT_SOCKET() -> c_int /* int */;
    pub fn expEVT_COMMAND_SPINCTRL_UPDATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_SPINCTRLDOUBLE_UPDATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_SPLITTER_SASH_POS_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_SPLITTER_SASH_POS_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_SPLITTER_DOUBLECLICKED() -> c_int /* int */;
    pub fn expEVT_COMMAND_SPLITTER_UNSPLIT() -> c_int /* int */;
    pub fn expEVT_COMMAND_SEARCHCTRL_CANCEL_BTN() -> c_int /* int */;
    pub fn expEVT_COMMAND_SEARCHCTRL_SEARCH_BTN() -> c_int /* int */;
    pub fn expEVT_TASKBAR_MOVE() -> c_int /* int */;
    pub fn expEVT_TASKBAR_LEFT_DOWN() -> c_int /* int */;
    pub fn expEVT_TASKBAR_LEFT_UP() -> c_int /* int */;
    pub fn expEVT_TASKBAR_RIGHT_DOWN() -> c_int /* int */;
    pub fn expEVT_TASKBAR_RIGHT_UP() -> c_int /* int */;
    pub fn expEVT_TASKBAR_LEFT_DCLICK() -> c_int /* int */;
    pub fn expEVT_TASKBAR_RIGHT_DCLICK() -> c_int /* int */;
    pub fn expEVT_TASKBAR_BALLOON_TIMEOUT() -> c_int /* int */;
    pub fn expEVT_TASKBAR_BALLOON_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_UPDATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_ENTER() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_URL() -> c_int /* int */;
    pub fn expEVT_COMMAND_TEXT_MAXLEN() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOGGLEBUTTON_CLICKED() -> c_int /* int */;
    pub fn expEVT_TIMER() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOOLBOOK_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TOOLBOOK_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_BEGIN_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_BEGIN_RDRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_BEGIN_LABEL_EDIT() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_END_LABEL_EDIT() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_DELETE_ITEM() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_GET_INFO() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_SET_INFO() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_EXPANDED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_EXPANDING() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_COLLAPSED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_COLLAPSING() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_SEL_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_SEL_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_KEY_DOWN() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_ACTIVATED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_RIGHT_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_MIDDLE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_END_DRAG() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_STATE_IMAGE_CLICK() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_GETTOOLTIP() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREE_ITEM_MENU() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREEBOOK_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREEBOOK_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREEBOOK_NODE_COLLAPSED() -> c_int /* int */;
    pub fn expEVT_COMMAND_TREEBOOK_NODE_EXPANDED() -> c_int /* int */;
    pub fn expEVT_WIZARD_PAGE_CHANGED() -> c_int /* int */;
    pub fn expEVT_WIZARD_PAGE_CHANGING() -> c_int /* int */;
    pub fn expEVT_WIZARD_CANCEL() -> c_int /* int */;
    pub fn expEVT_WIZARD_HELP() -> c_int /* int */;
    pub fn expEVT_WIZARD_FINISHED() -> c_int /* int */;
    pub fn expEVT_WIZARD_PAGE_SHOWN() -> c_int /* int */;
    pub fn expEVT_DELETE() -> c_int /* int */;
    pub fn expEVT_HTML_CELL_CLICKED() -> c_int /* int */;
    pub fn expEVT_HTML_CELL_MOUSE_HOVER() -> c_int /* int */;
    pub fn expEVT_HTML_LINK_CLICKED() -> c_int /* int */;
    pub fn expEVT_HTML_SET_TITLE() -> c_int /* int */;
    pub fn expEVT_INPUT_SINK() -> c_int /* int */;
    pub fn expEVT_SORT() -> c_int /* int */;
    pub fn expK_BACK() -> c_int /* int */;
    pub fn expK_TAB() -> c_int /* int */;
    pub fn expK_RETURN() -> c_int /* int */;
    pub fn expK_ESCAPE() -> c_int /* int */;
    pub fn expK_SPACE() -> c_int /* int */;
    pub fn expK_DELETE() -> c_int /* int */;
    pub fn expK_START() -> c_int /* int */;
    pub fn expK_LBUTTON() -> c_int /* int */;
    pub fn expK_RBUTTON() -> c_int /* int */;
    pub fn expK_CANCEL() -> c_int /* int */;
    pub fn expK_MBUTTON() -> c_int /* int */;
    pub fn expK_CLEAR() -> c_int /* int */;
    pub fn expK_SHIFT() -> c_int /* int */;
    pub fn expK_ALT() -> c_int /* int */;
    pub fn expK_CONTROL() -> c_int /* int */;
    pub fn expK_MENU() -> c_int /* int */;
    pub fn expK_PAUSE() -> c_int /* int */;
    pub fn expK_CAPITAL() -> c_int /* int */;
    pub fn expK_END() -> c_int /* int */;
    pub fn expK_HOME() -> c_int /* int */;
    pub fn expK_LEFT() -> c_int /* int */;
    pub fn expK_UP() -> c_int /* int */;
    pub fn expK_RIGHT() -> c_int /* int */;
    pub fn expK_DOWN() -> c_int /* int */;
    pub fn expK_SELECT() -> c_int /* int */;
    pub fn expK_PRINT() -> c_int /* int */;
    pub fn expK_EXECUTE() -> c_int /* int */;
    pub fn expK_SNAPSHOT() -> c_int /* int */;
    pub fn expK_INSERT() -> c_int /* int */;
    pub fn expK_HELP() -> c_int /* int */;
    pub fn expK_NUMPAD0() -> c_int /* int */;
    pub fn expK_NUMPAD1() -> c_int /* int */;
    pub fn expK_NUMPAD2() -> c_int /* int */;
    pub fn expK_NUMPAD3() -> c_int /* int */;
    pub fn expK_NUMPAD4() -> c_int /* int */;
    pub fn expK_NUMPAD5() -> c_int /* int */;
    pub fn expK_NUMPAD6() -> c_int /* int */;
    pub fn expK_NUMPAD7() -> c_int /* int */;
    pub fn expK_NUMPAD8() -> c_int /* int */;
    pub fn expK_NUMPAD9() -> c_int /* int */;
    pub fn expK_MULTIPLY() -> c_int /* int */;
    pub fn expK_ADD() -> c_int /* int */;
    pub fn expK_SEPARATOR() -> c_int /* int */;
    pub fn expK_SUBTRACT() -> c_int /* int */;
    pub fn expK_DECIMAL() -> c_int /* int */;
    pub fn expK_DIVIDE() -> c_int /* int */;
    pub fn expK_F1() -> c_int /* int */;
    pub fn expK_F2() -> c_int /* int */;
    pub fn expK_F3() -> c_int /* int */;
    pub fn expK_F4() -> c_int /* int */;
    pub fn expK_F5() -> c_int /* int */;
    pub fn expK_F6() -> c_int /* int */;
    pub fn expK_F7() -> c_int /* int */;
    pub fn expK_F8() -> c_int /* int */;
    pub fn expK_F9() -> c_int /* int */;
    pub fn expK_F10() -> c_int /* int */;
    pub fn expK_F11() -> c_int /* int */;
    pub fn expK_F12() -> c_int /* int */;
    pub fn expK_F13() -> c_int /* int */;
    pub fn expK_F14() -> c_int /* int */;
    pub fn expK_F15() -> c_int /* int */;
    pub fn expK_F16() -> c_int /* int */;
    pub fn expK_F17() -> c_int /* int */;
    pub fn expK_F18() -> c_int /* int */;
    pub fn expK_F19() -> c_int /* int */;
    pub fn expK_F20() -> c_int /* int */;
    pub fn expK_F21() -> c_int /* int */;
    pub fn expK_F22() -> c_int /* int */;
    pub fn expK_F23() -> c_int /* int */;
    pub fn expK_F24() -> c_int /* int */;
    pub fn expK_NUMLOCK() -> c_int /* int */;
    pub fn expK_SCROLL() -> c_int /* int */;
    pub fn expK_PAGEUP() -> c_int /* int */;
    pub fn expK_PAGEDOWN() -> c_int /* int */;
    pub fn expK_NUMPAD_SPACE() -> c_int /* int */;
    pub fn expK_NUMPAD_TAB() -> c_int /* int */;
    pub fn expK_NUMPAD_ENTER() -> c_int /* int */;
    pub fn expK_NUMPAD_F1() -> c_int /* int */;
    pub fn expK_NUMPAD_F2() -> c_int /* int */;
    pub fn expK_NUMPAD_F3() -> c_int /* int */;
    pub fn expK_NUMPAD_F4() -> c_int /* int */;
    pub fn expK_NUMPAD_HOME() -> c_int /* int */;
    pub fn expK_NUMPAD_LEFT() -> c_int /* int */;
    pub fn expK_NUMPAD_UP() -> c_int /* int */;
    pub fn expK_NUMPAD_RIGHT() -> c_int /* int */;
    pub fn expK_NUMPAD_DOWN() -> c_int /* int */;
    pub fn expK_NUMPAD_PAGEUP() -> c_int /* int */;
    pub fn expK_NUMPAD_PAGEDOWN() -> c_int /* int */;
    pub fn expK_NUMPAD_END() -> c_int /* int */;
    pub fn expK_NUMPAD_BEGIN() -> c_int /* int */;
    pub fn expK_NUMPAD_INSERT() -> c_int /* int */;
    pub fn expK_NUMPAD_DELETE() -> c_int /* int */;
    pub fn expK_NUMPAD_EQUAL() -> c_int /* int */;
    pub fn expK_NUMPAD_MULTIPLY() -> c_int /* int */;
    pub fn expK_NUMPAD_ADD() -> c_int /* int */;
    pub fn expK_NUMPAD_SEPARATOR() -> c_int /* int */;
    pub fn expK_NUMPAD_SUBTRACT() -> c_int /* int */;
    pub fn expK_NUMPAD_DECIMAL() -> c_int /* int */;
    pub fn expK_NUMPAD_DIVIDE() -> c_int /* int */;
    pub fn ELJSysErrorCode() -> c_int /* int */;
    pub fn ELJSysErrorMsg(nErrCode: c_int /* int */) -> *u8 /* void* */;
    pub fn LogErrorMsg(_msg: *u8 /* void* */);
    pub fn LogFatalErrorMsg(_msg: *u8 /* void* */);
    pub fn LogMessageMsg(_msg: *u8 /* void* */);
    pub fn LogWarningMsg(_msg: *u8 /* void* */);
    pub fn Quantize(src: *u8 /* void* */, dest: *u8 /* void* */, desiredNoColours: c_int /* int */, eightBitData: *u8 /* void* */, flags: c_int /* int */) -> bool /* bool */;
    pub fn QuantizePalette(src: *u8 /* void* */, dest: *u8 /* void* */, pPalette: *u8 /* void* */, desiredNoColours: c_int /* int */, eightBitData: *u8 /* void* */, flags: c_int /* int */) -> bool /* bool */;
    pub fn wxCFree(_ptr: *u8 /* void* */);
    pub fn wxGetELJLocale() -> *u8 /* void* */;
    pub fn wxGetELJTranslation(sz: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    pub fn wxMutexGui_Enter();
    pub fn wxMutexGui_Leave();
    
    // TClassDefExtend(wxMultiCellItemHandle,wxObject)
    pub fn wxMultiCellItemHandle_Create(row: c_int /* int */, column: c_int /* int */, height: c_int /* int */, width: c_int /* int */, sx: c_int /* int */, sy: c_int /* int */, style: c_int /* int */, wx: c_int /* int */, wy: c_int /* int */, align: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMultiCellItemHandle_CreateWithSize(_obj: *u8 /* void* */, row: c_int /* int */, column: c_int /* int */, sx: c_int /* int */, sy: c_int /* int */, style: c_int /* int */, wx: c_int /* int */, wy: c_int /* int */, align: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMultiCellItemHandle_CreateWithStyle(_obj: *u8 /* void* */, row: c_int /* int */, column: c_int /* int */, style: c_int /* int */, wx: c_int /* int */, wy: c_int /* int */, align: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMultiCellItemHandle_GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellItemHandle_GetColumn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellItemHandle_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellItemHandle_GetLocalSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxMultiCellItemHandle_GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellItemHandle_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellItemHandle_GetWeight(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxMultiCellItemHandle_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxKeyEvent,wxEvent)
    pub fn wxKeyEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxKeyEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxKeyEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxKeyEvent_GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxKeyEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxKeyEvent_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxKeyEvent_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxKeyEvent_GetModifiers(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxKeyEvent_HasModifiers(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxKeyEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxKeyEvent_SetKeyCode(_obj: *u8 /* void* */, code: c_int /* int */);
    pub fn wxKeyEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxWindow,wxEvtHandler)
    pub fn wxWindow_AddChild(_obj: *u8 /* void* */, child: *u8 /* void* */);
    pub fn wxWindow_AddConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */);
    pub fn wxWindow_CaptureMouse(_obj: *u8 /* void* */);
    pub fn wxWindow_Center(_obj: *u8 /* void* */, direction: c_int /* int */);
    pub fn wxWindow_CenterOnParent(_obj: *u8 /* void* */, dir: c_int /* int */);
    pub fn wxWindow_ClearBackground(_obj: *u8 /* void* */);
    pub fn wxWindow_ClientToScreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxWindow_Close(_obj: *u8 /* void* */, _force: bool /* bool */) -> bool /* bool */;
    pub fn wxWindow_ConvertDialogToPixels(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_ConvertPixelsToDialog(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxWindow_DeleteRelatedConstraints(_obj: *u8 /* void* */);
    pub fn wxWindow_Destroy(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_DestroyChildren(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_Disable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_DoPhase(_obj: *u8 /* void* */, phase: c_int /* int */) -> c_int /* int */;
    pub fn wxWindow_Enable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_FindFocus(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_FindWindow(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_Fit(_obj: *u8 /* void* */);
    pub fn wxWindow_FitInside(_obj: *u8 /* void* */);
    pub fn wxWindow_Freeze(_obj: *u8 /* void* */);
    pub fn wxWindow_GetEffectiveMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetAutoLayout(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxWindow_GetBestSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetCaret(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */;
    pub fn wxWindow_GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetClientSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetClientSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxWindow_GetConstraints(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetConstraintsInvolvedIn(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetDropTarget(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetEventHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxWindow_GetForegroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxWindow_GetHandle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetLabelEmpty(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetMaxHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetMaxWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetMinHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetMinWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetPositionConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxWindow_GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    pub fn wxWindow_GetScrollRange(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    pub fn wxWindow_GetScrollThumb(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    pub fn wxWindow_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxWindow_GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetTextExtent(_obj: *u8 /* void* */, string: *u8 /* void* */, x: *c_int /* int* */, y: *c_int /* int* */, descent: *c_int /* int* */, externalLeading: *c_int /* int* */, theFont: *u8 /* void* */);
    pub fn wxWindow_GetToolTip(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetUpdateRegion(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetValidator(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetVirtualSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_GetWindowStyleFlag(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_HasFlag(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */;
    pub fn wxWindow_Hide(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_InitDialog(_obj: *u8 /* void* */);
    pub fn wxWindow_IsBeingDeleted(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_IsExposed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxWindow_IsShown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_IsTopLevel(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_Layout(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_LayoutPhase1(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */;
    pub fn wxWindow_LayoutPhase2(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */;
    pub fn wxWindow_Lower(_obj: *u8 /* void* */);
    pub fn wxWindow_MakeModal(_obj: *u8 /* void* */, modal: bool /* bool */);
    pub fn wxWindow_Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxWindow_MoveConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxWindow_PopEventHandler(_obj: *u8 /* void* */, deleteHandler: bool /* bool */) -> *u8 /* void* */;
    pub fn wxWindow_PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    pub fn wxWindow_PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxWindow_PushEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    pub fn wxWindow_Raise(_obj: *u8 /* void* */);
    pub fn wxWindow_Refresh(_obj: *u8 /* void* */, eraseBackground: bool /* bool */);
    pub fn wxWindow_RefreshRect(_obj: *u8 /* void* */, eraseBackground: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxWindow_ReleaseMouse(_obj: *u8 /* void* */);
    pub fn wxWindow_RemoveChild(_obj: *u8 /* void* */, child: *u8 /* void* */);
    pub fn wxWindow_RemoveConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */);
    pub fn wxWindow_Reparent(_obj: *u8 /* void* */, _par: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_ResetConstraints(_obj: *u8 /* void* */);
    pub fn wxWindow_ScreenToClient(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxWindow_ScrollWindow(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxWindow_ScrollWindowRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */);
    pub fn wxWindow_SetAcceleratorTable(_obj: *u8 /* void* */, accel: *u8 /* void* */);
    pub fn wxWindow_SetAutoLayout(_obj: *u8 /* void* */, autoLayout: bool /* bool */);
    pub fn wxWindow_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_SetCaret(_obj: *u8 /* void* */, caret: *u8 /* void* */);
    pub fn wxWindow_SetClientData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxWindow_SetClientObject(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxWindow_SetClientSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxWindow_SetConstraintSizes(_obj: *u8 /* void* */, recurse: c_int /* int */);
    pub fn wxWindow_SetConstraints(_obj: *u8 /* void* */, constraints: *u8 /* void* */);
    pub fn wxWindow_SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_SetDropTarget(_obj: *u8 /* void* */, dropTarget: *u8 /* void* */);
    pub fn wxWindow_SetExtraStyle(_obj: *u8 /* void* */, exStyle: c_long /* long */);
    pub fn wxWindow_SetFocus(_obj: *u8 /* void* */);
    pub fn wxWindow_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_SetForegroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWindow_SetId(_obj: *u8 /* void* */, _id: c_int /* int */);
    pub fn wxWindow_SetLabel(_obj: *u8 /* void* */, _title: *u8 /* void* */);
    pub fn wxWindow_SetName(_obj: *u8 /* void* */, _name: *u8 /* void* */);
    pub fn wxWindow_SetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, refresh: bool /* bool */);
    pub fn wxWindow_SetScrollbar(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, thumbVisible: c_int /* int */, range: c_int /* int */, refresh: bool /* bool */);
    pub fn wxWindow_SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sizeFlags: c_int /* int */);
    pub fn wxWindow_SetSizeConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxWindow_SetSizeHints(_obj: *u8 /* void* */, minW: c_int /* int */, minH: c_int /* int */, maxW: c_int /* int */, maxH: c_int /* int */, incW: c_int /* int */, incH: c_int /* int */);
    pub fn wxWindow_SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */);
    pub fn wxWindow_SetToolTip(_obj: *u8 /* void* */, tip: *u8 /* void* */);
    pub fn wxWindow_SetValidator(_obj: *u8 /* void* */, validator: *u8 /* void* */);
    pub fn wxWindow_SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_long /* long */);
    pub fn wxWindow_Show(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_Thaw(_obj: *u8 /* void* */);
    pub fn wxWindow_TransferDataFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_TransferDataToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_UnsetConstraints(_obj: *u8 /* void* */, c: *u8 /* void* */);
    pub fn wxWindow_UpdateWindowUI(_obj: *u8 /* void* */);
    pub fn wxWindow_Validate(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxWindow_SetVirtualSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxWindow_WarpPointer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(wxCustomDataObject,wxDataObjectSimple)
    
    // TClassDefExtend(wxSashWindow,wxWindow)
    pub fn wxSashWindow_Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSashWindow_GetDefaultBorderSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashWindow_GetEdgeMargin(_obj: *u8 /* void* */, edge: c_int /* int */) -> c_int /* int */;
    pub fn wxSashWindow_GetExtraBorderSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashWindow_GetMaximumSizeX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashWindow_GetMaximumSizeY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashWindow_GetMinimumSizeX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashWindow_GetMinimumSizeY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashWindow_GetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */;
    pub fn wxSashWindow_HasBorder(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */;
    pub fn wxSashWindow_SetDefaultBorderSize(_obj: *u8 /* void* */, width: c_int /* int */);
    pub fn wxSashWindow_SetExtraBorderSize(_obj: *u8 /* void* */, width: c_int /* int */);
    pub fn wxSashWindow_SetMaximumSizeX(_obj: *u8 /* void* */, max: c_int /* int */);
    pub fn wxSashWindow_SetMaximumSizeY(_obj: *u8 /* void* */, max: c_int /* int */);
    pub fn wxSashWindow_SetMinimumSizeX(_obj: *u8 /* void* */, min: c_int /* int */);
    pub fn wxSashWindow_SetMinimumSizeY(_obj: *u8 /* void* */, min: c_int /* int */);
    pub fn wxSashWindow_SetSashBorder(_obj: *u8 /* void* */, edge: c_int /* int */, border: bool /* bool */);
    pub fn wxSashWindow_SetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */, sash: bool /* bool */);
    
    // TClassDef(wxRealPoint)
    
    // TClassDefExtend(wxSystemSettings,wxObject)
    pub fn wxSystemSettings_GetColour(index: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxSystemSettings_GetFont(index: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxSystemSettings_GetMetric(index: c_int /* int */) -> c_int /* int */;
    pub fn wxSystemSettings_GetScreenType() -> c_int /* int */;
    
    // TClassDefExtend(cbBarSpy,wxEvtHandler)
    pub fn cbBarSpy_Create(pPanel: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbBarSpy_CreateDefault() -> *u8 /* void* */;
    pub fn cbBarSpy_Delete(_obj: *u8 /* void* */);
    pub fn cbBarSpy_ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> c_int /* int */;
    pub fn cbBarSpy_SetBarWindow(_obj: *u8 /* void* */, pWnd: *u8 /* void* */);
    
    // TClassDefExtend(wxNotebookEvent,wxNotifyEvent)
    
    // TClassDefExtend(wxJoystickEvent,wxEvent)
    pub fn wxJoystickEvent_ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxJoystickEvent_ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxJoystickEvent_ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxJoystickEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxJoystickEvent_GetButtonChange(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystickEvent_GetButtonState(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystickEvent_GetJoystick(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystickEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxJoystickEvent_GetZPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystickEvent_IsButton(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystickEvent_IsMove(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystickEvent_IsZMove(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystickEvent_SetButtonChange(_obj: *u8 /* void* */, change: c_int /* int */);
    pub fn wxJoystickEvent_SetButtonState(_obj: *u8 /* void* */, state: c_int /* int */);
    pub fn wxJoystickEvent_SetJoystick(_obj: *u8 /* void* */, stick: c_int /* int */);
    pub fn wxJoystickEvent_SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxJoystickEvent_SetZPosition(_obj: *u8 /* void* */, zPos: c_int /* int */);
    
    // TClassDefExtend(wxMBConvUTF7,wxMBConv)
    
    // TClassDefExtend(wxScrollWinEvent,wxEvent)
    pub fn wxScrollWinEvent_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollWinEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollWinEvent_SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    pub fn wxScrollWinEvent_SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
    
    // TClassDefExtend(wxMBConvUTF8,wxMBConv)
    
    // TClassDefExtend(wxCountingOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxDataObjectComposite,wxDataObject)
    pub fn wxDataObjectComposite_Add(_obj: *u8 /* void* */, _dat: *u8 /* void* */, _preferred: c_int /* int */);
    pub fn wxDataObjectComposite_Create() -> *u8 /* void* */;
    pub fn wxDataObjectComposite_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxScreenDC,wxDC)
    pub fn wxScreenDC_Create() -> *u8 /* void* */;
    pub fn wxScreenDC_Delete(_obj: *u8 /* void* */);
    pub fn wxScreenDC_EndDrawingOnTop(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxScreenDC_StartDrawingOnTop(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxScreenDC_StartDrawingOnTopOfWin(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxGraphicsPath,wxGraphicsObject);
    
    // TClassDefExtend(wxHelpControllerBase,wxObject)
    
    // TClassDefExtend(wxTextDropTarget,wxDropTarget)
    
    // TClassDefExtend(wxBoolProperty,wxPGProperty)
    pub fn wxBoolProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: bool /* bool */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxSysColourChangedEvent,wxEvent)
    
    // TClassDefExtend(wxDrawControl,wxControl)
    pub fn wxDrawControl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(cbRowDragPlugin,cbPluginBase)
    pub fn cbRowDragPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbRowDragPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbRowDragPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxBufferedInputStream,wxFilterInputStream)
    
    // TClassDef(wxIconBundle)
    pub fn wxIconBundle_AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */);
    pub fn wxIconBundle_AddIconFromFile(_obj: *u8 /* void* */, file: *u8 /* void* */, type_: c_int /* int */);
    pub fn wxIconBundle_Assign(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxIconBundle_CreateDefault() -> *u8 /* void* */;
    pub fn wxIconBundle_CreateFromFile(file: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxIconBundle_CreateFromIcon(icon: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxIconBundle_Delete(_obj: *u8 /* void* */);
    pub fn wxIconBundle_GetIcon(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _ref: *u8 /* void* */);
    
    // TClassDefExtend(wxGridCellFloatRenderer,wxGridCellStringRenderer)
    
    // TClassDefExtend( wxcPrintout, wxPrintout );
    
    // TClassDefExtend(wxTextValidator,wxValidator)
    pub fn wxTextValidator_Create(style: c_int /* int */, val: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextValidator_GetExcludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxTextValidator_GetIncludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxTextValidator_SetExcludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */);
    pub fn wxTextValidator_SetIncludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */);
    pub fn wxTextValidator_Clone(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextValidator_TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextValidator_TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextValidator_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTextValidator_OnChar(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxTextValidator_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    
    // TClassDefExtend(wxDDEConnection,wxConnectionBase)
    
    // TClassDefExtend(wxCSConv,wxMBConv)
    
    // TClassDefExtend(cbStartDrawInAreaEvent,cbPluginEvent)
    pub fn cbStartDrawInAreaEvent_Area(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    
    // TClassDefExtend(ELJCommand,wxCommand)
    pub fn ELJCommand_CanUndo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn ELJCommand_Create(_und: c_int /* int */, _nme: *u8 /* void* */, _obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJCommand_Delete(_obj: *u8 /* void* */);
    pub fn ELJCommand_GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(ELJDragDataObject)
    pub fn ELJDragDataObject_Create(_obj: *u8 /* void* */, _fmt: *u8 /* void* */, _func1: *u8 /* void* */, _func2: *u8 /* void* */, _func3: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJDragDataObject_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxStaticLine,wxControl)
    pub fn wxStaticLine_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStaticLine_GetDefaultSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStaticLine_IsVertical(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxWindowDisabler)
    
    // TClassDef(wxTextInputStream)
    
    // TClassDef( wxTextInputStream );
    pub fn wxTextInputStream_Create(inputStream: *u8 /* void* */, sep: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextInputStream_Delete(self_: *u8 /* void* */);
    pub fn wxTextInputStream_ReadLine(self_: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxListCtrl,wxControl)
    pub fn wxListCtrl_Arrange(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_ClearAll(_obj: *u8 /* void* */);
    pub fn wxListCtrl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListCtrl_DeleteAllColumns(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_DeleteAllItems(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_DeleteColumn(_obj: *u8 /* void* */, col: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_DeleteItem(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_EditLabel(_obj: *u8 /* void* */, item: c_int /* int */);
    pub fn wxListCtrl_EndEditLabel(_obj: *u8 /* void* */, cancel: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_EnsureVisible(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_FindItem(_obj: *u8 /* void* */, start: c_int /* int */, str: *u8 /* void* */, partial: bool /* bool */) -> c_int /* int */;
    pub fn wxListCtrl_FindItemByData(_obj: *u8 /* void* */, start: c_int /* int */, data: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_FindItemByPosition(_obj: *u8 /* void* */, start: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, direction: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_GetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_GetColumnCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_GetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_GetCountPerPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetImageList(_obj: *u8 /* void* */, which: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_GetItemCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_GetItemData(_obj: *u8 /* void* */, item: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_GetItemFont(_obj: *u8 /* void* */, item: c_long /* long */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetItemRect(_obj: *u8 /* void* */, item: c_int /* int */, code: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetItemSpacing(_obj: *u8 /* void* */, isSmall: bool /* bool */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetItemState(_obj: *u8 /* void* */, item: c_int /* int */, stateMask: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_GetItemText(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListCtrl_GetNextItem(_obj: *u8 /* void* */, item: c_int /* int */, geometry: c_int /* int */, state: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_GetSelectedItemCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxListCtrl_GetTopItem(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_InsertColumn(_obj: *u8 /* void* */, col: c_int /* int */, heading: *u8 /* void* */, format: c_int /* int */, width: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_InsertColumnFromInfo(_obj: *u8 /* void* */, col: c_int /* int */, info: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_InsertItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_InsertItemWithData(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_InsertItemWithImage(_obj: *u8 /* void* */, index: c_int /* int */, imageIndex: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_InsertItemWithLabel(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */, imageIndex: c_int /* int */) -> c_int /* int */;
    pub fn wxListCtrl_IsVirtual(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_RefreshItem(_obj: *u8 /* void* */, item: c_long /* long */);
    pub fn wxListCtrl_ScrollList(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxListCtrl_SetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_SetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetForegroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */, which: c_int /* int */);
    pub fn wxListCtrl_SetItem(_obj: *u8 /* void* */, index: c_int /* int */, col: c_int /* int */, label: *u8 /* void* */, imageId: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetItemData(_obj: *u8 /* void* */, item: c_int /* int */, data: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetItemFromInfo(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_SetItemImage(_obj: *u8 /* void* */, item: c_int /* int */, image: c_int /* int */, selImage: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetItemState(_obj: *u8 /* void* */, item: c_int /* int */, state: c_int /* int */, stateMask: c_int /* int */) -> bool /* bool */;
    pub fn wxListCtrl_SetItemText(_obj: *u8 /* void* */, item: c_int /* int */, str: *u8 /* void* */);
    pub fn wxListCtrl_SetSingleStyle(_obj: *u8 /* void* */, style: c_int /* int */, add: bool /* bool */);
    pub fn wxListCtrl_SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxListCtrl_SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_int /* int */);
    pub fn wxListCtrl_SortItems(_obj: *u8 /* void* */, fn_: *u8 /* void* */, eif_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListCtrl_UpdateStyle(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxThinSplitterWindow,wxSplitterWindow)
    pub fn wxThinSplitterWindow_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxThinSplitterWindow_DrawSash(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxThinSplitterWindow_SashHitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, tolerance: c_int /* int */) -> c_int /* int */;
    pub fn wxThinSplitterWindow_SizeWindows(_obj: *u8 /* void* */);
    
    // TClassDef(wxGridCellWorker)
    
    // TClassDefExtend(wxWindowCreateEvent,wxCommandEvent)
    pub fn wxWindowCreateEvent_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxcHtmlEvent,wxCommandEvent)
    pub fn wxcHtmlEvent_GetMouseEvent(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetHtmlCell(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetHtmlCellId(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetHref(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetTarget(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetLogicalPosition(self_: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxAutomationObject,wxObject)
    
    // TClassDefExtend(wxPageSetupDialog,wxDialog)
    pub fn wxPageSetupDialog_Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialog_GetPageSetupData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    
    // TClassDefExtend(wxMultiCellSizer,wxSizer)
    pub fn wxMultiCellSizer_CalcMin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxMultiCellSizer_Create(rows: c_int /* int */, cols: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMultiCellSizer_Delete(_obj: *u8 /* void* */);
    pub fn wxMultiCellSizer_EnableGridLines(_obj: *u8 /* void* */, win: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellSizer_RecalcSizes(_obj: *u8 /* void* */);
    pub fn wxMultiCellSizer_SetColumnWidth(_obj: *u8 /* void* */, column: c_int /* int */, colSize: c_int /* int */, expandable: c_int /* int */) -> c_int /* int */;
    pub fn wxMultiCellSizer_SetDefaultCellSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    pub fn wxMultiCellSizer_SetGridPen(_obj: *u8 /* void* */, pen: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellSizer_SetRowHeight(_obj: *u8 /* void* */, row: c_int /* int */, rowSize: c_int /* int */, expandable: c_int /* int */) -> c_int /* int */;
    
    // TClassDefExtend(wxPaintDC,wxWindowDC)
    pub fn wxPaintDC_Create(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPaintDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxToolBarBase,wxControl)
    
    // TClassDefExtend(wxVariant,wxObject)
    
    // TClassDefExtend(wxSpinEvent,wxNotifyEvent)
    pub fn wxSpinEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinEvent_SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
    
    // TClassDefExtend(wxLogStderr,wxLog)
    
    // TClassDefExtend(wxDocMDIParentFrame,wxMDIParentFrame)
    
    // TClassDefExtend(wxIPV4address,wxSockAddress)
    
    // TClassDefExtend(wxMediaCtrl,wxWindow);
    pub fn wxMediaCtrl_Create(parent: *u8 /* void* */, windowID: c_int /* int */, fileName: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */, w: c_int /* int */, h: c_int /* int */, style: c_long /* long */, szBackend: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMediaCtrl_Delete(self_: *u8 /* void* */);
    pub fn wxMediaCtrl_GetBestSize(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMediaCtrl_GetPlaybackRate(self_: *u8 /* void* */) -> c_double /* double */;
    pub fn wxMediaCtrl_GetVolume(self_: *u8 /* void* */) -> c_double /* double */;
    pub fn wxMediaCtrl_GetState(self_: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMediaCtrl_Length(self_: *u8 /* void* */) -> i64 /* i64 */;
    pub fn wxMediaCtrl_Load(self_: *u8 /* void* */, fileName: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMediaCtrl_LoadURI(self_: *u8 /* void* */, uri: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMediaCtrl_LoadURIWithProxy(self_: *u8 /* void* */, uri: *u8 /* void* */, proxy: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMediaCtrl_Pause(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMediaCtrl_Play(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMediaCtrl_Seek(self_: *u8 /* void* */, offsetWhere: i64 /* i64 */, mode: c_int /* int */) -> i64 /* i64 */;
    pub fn wxMediaCtrl_SetPlaybackRate(self_: *u8 /* void* */, dRate: c_double /* double */) -> bool /* bool */;
    pub fn wxMediaCtrl_SetVolume(self_: *u8 /* void* */, dVolume: c_double /* double */) -> bool /* bool */;
    pub fn wxMediaCtrl_ShowPlayerControls(self_: *u8 /* void* */, flags: c_int /* int */) -> bool /* bool */;
    pub fn wxMediaCtrl_Stop(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMediaCtrl_Tell(self_: *u8 /* void* */) -> i64 /* i64 */;
    
    // TClassDefExtend(wxPaintEvent,wxEvent)
    
    // TClassDefExtend(wxSocketServer,wxSocketBase)
    
    // TClassDef(wxString)
    
    // TClassDefExtend(wxBitmapToggleButton,wxToggleButton)
    pub fn wxBitmapToggleButton_Create(parent: *u8 /* void* */, id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmapToggleButton_Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    pub fn wxBitmapToggleButton_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBitmapToggleButton_SetValue(_obj: *u8 /* void* */, state: bool /* bool */);
    pub fn wxBitmapToggleButton_SetBitmapLabel(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    
    // TClassDef(wxDbSqlTypeInfo)
    
    // TClassDefExtend(wxFontDialog,wxDialog)
    pub fn wxFontDialog_Create(_prt: *u8 /* void* */, fnt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFontDialog_GetFontData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    
    // TClassDefExtend(wxOutputStream,wxStreamBase)
    pub fn wxOutputStream_Delete(_obj: *u8 /* void* */);
    pub fn wxOutputStream_LastWrite(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxOutputStream_PutC(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */);
    pub fn wxOutputStream_Seek(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */;
    pub fn wxOutputStream_Sync(_obj: *u8 /* void* */);
    pub fn wxOutputStream_Tell(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxOutputStream_Write(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */);
    
    // TClassDef(wxTextAttr)
    
    // TClassDefExtend(wxHelpController,wxHelpControllerBase)
    
    // TClassDef(wxDbColDef)
    
    // TClassDefExtend(wxMaximizeEvent,wxEvent)
    
    // TClassDefExtend(wxDocMDIChildFrame,wxMDIChildFrame)
    
    // TClassDef(wxAcceleratorEntry)
    pub fn wxAcceleratorEntry_Create(flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) -> *u8 /* void* */;
    pub fn wxAcceleratorEntry_Delete(_obj: *u8 /* void* */);
    pub fn wxAcceleratorEntry_GetCommand(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxAcceleratorEntry_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxAcceleratorEntry_GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxAcceleratorEntry_Set(_obj: *u8 /* void* */, flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */);
    
    // TClassDef(wxDialUpManager)
    pub fn wxDialUpManager_CancelDialing(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpManager_Create() -> *u8 /* void* */;
    pub fn wxDialUpManager_Delete(_obj: *u8 /* void* */);
    pub fn wxDialUpManager_Dial(_obj: *u8 /* void* */, nameOfISP: *u8 /* void* */, username: *u8 /* void* */, password: *u8 /* void* */, async: bool /* bool */) -> bool /* bool */;
    pub fn wxDialUpManager_DisableAutoCheckOnlineStatus(_obj: *u8 /* void* */);
    pub fn wxDialUpManager_EnableAutoCheckOnlineStatus(_obj: *u8 /* void* */, nSeconds: c_int /* int */) -> bool /* bool */;
    pub fn wxDialUpManager_GetISPNames(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDialUpManager_HangUp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpManager_IsAlwaysOnline(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpManager_IsDialing(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpManager_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpManager_IsOnline(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpManager_SetConnectCommand(_obj: *u8 /* void* */, commandDial: *u8 /* void* */, commandHangup: *u8 /* void* */);
    pub fn wxDialUpManager_SetOnlineStatus(_obj: *u8 /* void* */, isOnline: bool /* bool */);
    pub fn wxDialUpManager_SetWellKnownHost(_obj: *u8 /* void* */, hostname: *u8 /* void* */, portno: c_int /* int */);
    
    // TClassDefExtend(wxDynToolInfo,wxToolLayoutItem)
    pub fn wxDynToolInfo_Index(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDynToolInfo_RealSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxDynToolInfo_pToolWnd(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDocTemplate,wxObject)
    
    // TClassDefExtend(wxPreviewControlBar,wxPanel)
    
    // TClassDefExtend(wxPreviewControlBar,wxPanel);
    pub fn expEVT_PRINT_BEGIN() -> c_int /* int */;
    pub fn expEVT_PRINT_BEGIN_DOC() -> c_int /* int */;
    pub fn expEVT_PRINT_END() -> c_int /* int */;
    pub fn expEVT_PRINT_END_DOC() -> c_int /* int */;
    pub fn expEVT_PRINT_PREPARE() -> c_int /* int */;
    pub fn expEVT_PRINT_PAGE() -> c_int /* int */;
    pub fn wxPrintout_GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintout_GetPPIPrinter(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxPrintout_GetPPIScreen(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxPrintout_GetPageSizeMM(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxPrintout_GetPageSizePixels(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxPrintout_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintout_IsPreview(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintout_SetDC(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxPrintout_SetPPIPrinter(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPrintout_SetPPIScreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPrintout_SetPageSizeMM(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPrintout_SetPageSizePixels(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(ELJPreviewFrame,wxPreviewFrame)
    pub fn ELJPreviewFrame_Create(_obj: *u8 /* void* */, _init: *u8 /* void* */, _create_canvas: *u8 /* void* */, _create_toolbar: *u8 /* void* */, preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_GetControlBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_GetPreviewCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_GetPrintPreview(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_Initialize(_obj: *u8 /* void* */);
    pub fn ELJPreviewFrame_SetControlBar(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn ELJPreviewFrame_SetPreviewCanvas(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn ELJPreviewFrame_SetPrintPreview(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    
    // TClassDefExtend(wxBitmapHandler,wxObject)
    
    // TClassDefExtend(wxGraphicsContext,wxGraphicsObject);
    
    // TClassDefExtend(wxGaugeMSW,wxGauge)
    
    // TClassDefExtend(cbDimInfo,wxObject)
    pub fn cbDimInfo_Assign(_obj: *u8 /* void* */, other: *u8 /* void* */);
    pub fn cbDimInfo_Create(arg0: c_int /* int */, arg1: c_int /* int */, isFixed: bool /* bool */, gap: c_int /* int */, pDimHandler: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDimInfo_CreateDefault() -> *u8 /* void* */;
    pub fn cbDimInfo_CreateWithHandler(pDimHandler: *u8 /* void* */, isFixed: bool /* bool */) -> *u8 /* void* */;
    pub fn cbDimInfo_CreateWithInfo(dh_x: c_int /* int */, dh_y: c_int /* int */, dv_x: c_int /* int */, dv_y: c_int /* int */, f_x: c_int /* int */, f_y: c_int /* int */, isFixed: bool /* bool */, horizGap: c_int /* int */, vertGap: c_int /* int */, pDimHandler: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDimInfo_Delete(_obj: *u8 /* void* */);
    pub fn cbDimInfo_GetDimHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxMDIClientWindow,wxWindow)
    
    // TClassDefExtend(wxDataObjectSimple,wxDataObject)
    
    // TClassDefExtend(wxDialog,wxTopLevelWindow)
    pub fn wxDialog_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDialog_EndModal(_obj: *u8 /* void* */, retCode: c_int /* int */);
    pub fn wxDialog_GetReturnCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDialog_IsModal(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialog_SetReturnCode(_obj: *u8 /* void* */, returnCode: c_int /* int */);
    pub fn wxDialog_ShowModal(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxImageHandler,wxObject)
    
    // TClassDefExtend(wxClosure,wxObject)
    
    // TClassDefExtend(wxClosure,wxObject)
    pub fn wxClosure_Create(_fun_CEvent: *u8 /* void* */, _data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClosure_GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvtHandler_GetClosure(_obj: *u8 /* void* */, id: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxEvtHandler_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvtHandler_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    pub fn wxObject_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxObject_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    
    // TClassDefExtend(wxIndividualLayoutConstraint,wxObject)
    pub fn wxIndividualLayoutConstraint_Above(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_Absolute(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_AsIs(_obj: *u8 /* void* */);
    pub fn wxIndividualLayoutConstraint_Below(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_GetDone(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIndividualLayoutConstraint_GetEdge(_obj: *u8 /* void* */, which: c_int /* int */, thisWin: *u8 /* void* */, other: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_GetMargin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_GetMyEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_GetOtherEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_GetOtherWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxIndividualLayoutConstraint_GetPercent(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_GetRelationship(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIndividualLayoutConstraint_LeftOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_PercentOf(_obj: *u8 /* void* */, otherW: *u8 /* void* */, wh: c_int /* int */, per: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_ResetIfWin(_obj: *u8 /* void* */, otherW: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIndividualLayoutConstraint_RightOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_SameAs(_obj: *u8 /* void* */, otherW: *u8 /* void* */, edge: c_int /* int */, marg: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_SatisfyConstraint(_obj: *u8 /* void* */, constraints: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIndividualLayoutConstraint_Set(_obj: *u8 /* void* */, rel: c_int /* int */, otherW: *u8 /* void* */, otherE: c_int /* int */, val: c_int /* int */, marg: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_SetDone(_obj: *u8 /* void* */, d: bool /* bool */);
    pub fn wxIndividualLayoutConstraint_SetEdge(_obj: *u8 /* void* */, which: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_SetMargin(_obj: *u8 /* void* */, m: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_SetRelationship(_obj: *u8 /* void* */, r: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_SetValue(_obj: *u8 /* void* */, v: c_int /* int */);
    pub fn wxIndividualLayoutConstraint_Unconstrained(_obj: *u8 /* void* */);
    
    // TClassDef(wxTempFile)
    
    // TClassDefExtend(cbFloatedBarWindow,wxToolWindow)
    pub fn cbFloatedBarWindow_Create(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbFloatedBarWindow_GetBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbFloatedBarWindow_PositionFloatedWnd(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn cbFloatedBarWindow_SetBar(_obj: *u8 /* void* */, _bar: *u8 /* void* */);
    pub fn cbFloatedBarWindow_SetLayout(_obj: *u8 /* void* */, _layout: *u8 /* void* */);
    
    // TClassDefExtend(cbLeftUpEvent,cbPluginEvent)
    pub fn cbLeftUpEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxHtmlTagsModule,wxModule)
    
    // TClassDefExtend(wxMirrorDC,wxDC)
    pub fn wxMirrorDC_Create(dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMirrorDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDef(wxLocale)
    pub fn wxLocale_AddCatalog(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> c_int /* int */;
    pub fn wxLocale_AddCatalogLookupPathPrefix(_obj: *u8 /* void* */, prefix: *u8 /* void* */);
    pub fn wxLocale_Create(_name: c_int /* int */, _flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxLocale_Delete(_obj: *u8 /* void* */);
    pub fn wxLocale_GetLocale(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLocale_GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLocale_GetString(_obj: *u8 /* void* */, szOrigString: *u8 /* void* */, szDomain: *u8 /* void* */) -> *wchar_t /* wchar_t* */;
    pub fn wxLocale_IsLoaded(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLocale_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxFindDialogEvent,wxCommandEvent)
    pub fn wxFindDialogEvent_GetFindString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFindDialogEvent_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFindDialogEvent_GetReplaceString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDef(wxDb)
    
    // TClassDefExtend(wxQueryField,wxObject)
    
    // TClassDefExtend(wxHtmlDCRenderer,wxObject)
    
    // TClassDefExtend(wxGauge95,wxGauge)
    
    // TClassDefExtend(wxDC,wxObject)
    pub fn wxDC_Blit(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, source: *u8 /* void* */, arg4: c_int /* int */, arg5: c_int /* int */, rop: c_int /* int */, useMask: bool /* bool */) -> bool /* bool */;
    pub fn wxDC_CalcBoundingBox(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_CanDrawBitmap(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDC_CanGetTextExtent(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDC_Clear(_obj: *u8 /* void* */);
    pub fn wxDC_ComputeScaleAndOrigin(obj: *u8 /* void* */);
    pub fn wxDC_CrossHair(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_Delete(_obj: *u8 /* void* */);
    pub fn wxDC_DestroyClippingRegion(_obj: *u8 /* void* */);
    pub fn wxDC_DeviceToLogicalX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_DeviceToLogicalXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_DeviceToLogicalY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_DeviceToLogicalYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_DrawArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */);
    pub fn wxDC_DrawBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, useMask: bool /* bool */);
    pub fn wxDC_DrawCheckMark(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxDC_DrawCircle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, radius: c_int /* int */);
    pub fn wxDC_DrawEllipse(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxDC_DrawEllipticArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sa: c_double /* double */, ea: c_double /* double */);
    pub fn wxDC_DrawIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_DrawLabel(_obj: *u8 /* void* */, str: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */);
    pub fn wxDC_DrawLabelBitmap(_obj: *u8 /* void* */, str: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDC_DrawLine(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxDC_DrawLines(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_DrawPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_DrawPolygon(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */);
    pub fn wxDC_DrawPolyPolygon(_obj: *u8 /* void* */, n: c_int /* int */, count: *u8 /* void* */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */);
    pub fn wxDC_DrawRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxDC_DrawRotatedText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, angle: c_double /* double */);
    pub fn wxDC_DrawRoundedRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, radius: c_double /* double */);
    pub fn wxDC_DrawText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_EndDoc(_obj: *u8 /* void* */);
    pub fn wxDC_EndPage(_obj: *u8 /* void* */);
    pub fn wxDC_FloodFill(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */, style: c_int /* int */);
    pub fn wxDC_GetBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDC_GetBackgroundMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_GetBrush(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDC_GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_GetClippingBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxDC_GetDepth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_GetDeviceOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxDC_GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDC_GetLogicalFunction(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_GetLogicalOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxDC_GetLogicalScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */);
    pub fn wxDC_GetMapMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_GetPPI(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDC_GetPen(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDC_GetPixel(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDC_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDC_GetSizeMM(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDC_GetTextBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDC_GetTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, descent: *u8 /* void* */, externalLeading: *u8 /* void* */, theFont: *u8 /* void* */);
    pub fn wxDC_GetMultiLineTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, heightLine: *u8 /* void* */, theFont: *u8 /* void* */);
    pub fn wxDC_GetTextForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDC_GetUserScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */);
    pub fn wxDC_LogicalToDeviceX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_LogicalToDeviceXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_LogicalToDeviceY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_LogicalToDeviceYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxDC_MaxX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_MaxY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_MinX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_MinY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDC_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDC_ResetBoundingBox(_obj: *u8 /* void* */);
    pub fn wxDC_SetAxisOrientation(_obj: *u8 /* void* */, xLeftRight: bool /* bool */, yBottomUp: bool /* bool */);
    pub fn wxDC_SetBackground(_obj: *u8 /* void* */, brush: *u8 /* void* */);
    pub fn wxDC_SetBackgroundMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxDC_SetBrush(_obj: *u8 /* void* */, brush: *u8 /* void* */);
    pub fn wxDC_SetClippingRegion(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxDC_SetClippingRegionFromRegion(_obj: *u8 /* void* */, region: *u8 /* void* */);
    pub fn wxDC_SetDeviceOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxDC_SetLogicalFunction(_obj: *u8 /* void* */, function: c_int /* int */);
    pub fn wxDC_SetLogicalOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDC_SetLogicalScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */);
    pub fn wxDC_SetMapMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxDC_SetPalette(_obj: *u8 /* void* */, palette: *u8 /* void* */);
    pub fn wxDC_SetPen(_obj: *u8 /* void* */, pen: *u8 /* void* */);
    pub fn wxDC_SetTextBackground(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxDC_SetTextForeground(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxDC_SetUserScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */);
    pub fn wxDC_StartDoc(_obj: *u8 /* void* */, msg: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDC_StartPage(_obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJTextValidator,wxTextValidator)
    pub fn ELJTextValidator_Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */, _txt: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxHelpProvider)
    pub fn wxHelpProvider_AddHelp(_obj: *u8 /* void* */, window: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxHelpProvider_AddHelpById(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */);
    pub fn wxHelpProvider_Delete(_obj: *u8 /* void* */);
    pub fn wxHelpProvider_Get() -> *u8 /* void* */;
    pub fn wxHelpProvider_GetHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpProvider_RemoveHelp(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxHelpProvider_Set(helpProvider: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpProvider_ShowHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxHashMap)
    
    // TClassDefExtend(wxGLContext,wxObject);
    pub fn wxGLCanvas_Create(parent: *u8 /* void* */, windowID: c_int /* int */, attributes: *c_int /* int* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, title: *u8 /* void* */, palette: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGLCanvas_SetColour(self_: *u8 /* void* */, colour: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGLCanvas_SetCurrent(self_: *u8 /* void* */, ctxt: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGLCanvas_SwapBuffers(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGLCanvas_IsDisplaySupported(attributes: *c_int /* int* */) -> bool /* bool */;
    pub fn wxGLCanvas_IsExtensionSupported(extension: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGLContext_Create(win: *u8 /* void* */, other: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGLContext_CreateFromNull(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGLContext_SetCurrent(self_: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxGraphicsRenderer,wxGraphicsObject);
    pub fn wxGraphicsBrush_Create() -> *u8 /* void* */;
    pub fn wxGraphicsBrush_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsContext_Create(dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsContext_CreateFromWindow(window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsContext_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsContext_CreateFromNative(context: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsContext_CreateFromNativeWindow(window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsContext_Clip(self_: *u8 /* void* */, region: *u8 /* void* */);
    pub fn wxGraphicsContext_ClipByRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsContext_ResetClip(self_: *u8 /* void* */);
    pub fn wxGraphicsContext_DrawBitmap(self_: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsContext_DrawEllipse(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsContext_DrawIcon(self_: *u8 /* void* */, icon: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsContext_DrawLines(self_: *u8 /* void* */, n: size_t /* size_t */, x: *u8 /* void* */, y: *u8 /* void* */, style: c_int /* int */);
    pub fn wxGraphicsContext_DrawPath(self_: *u8 /* void* */, path: *u8 /* void* */, style: c_int /* int */);
    pub fn wxGraphicsContext_DrawRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsContext_DrawRoundedRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, radius: c_double /* double */);
    pub fn wxGraphicsContext_DrawText(self_: *u8 /* void* */, text: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */);
    pub fn wxGraphicsContext_DrawTextWithAngle(self_: *u8 /* void* */, text: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, radius: c_double /* double */);
    pub fn wxGraphicsContext_FillPath(self_: *u8 /* void* */, path: *u8 /* void* */, style: c_int /* int */);
    pub fn wxGraphicsContext_StrokePath(self_: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxGraphicsContext_GetNativeContext(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsContext_GetTextExtent(self_: *u8 /* void* */, text: *u8 /* void* */, width: *c_double /* double* */, height: *c_double /* double* */, descent: *c_double /* double* */, externalLeading: *c_double /* double* */);
    pub fn wxGraphicsContext_Rotate(self_: *u8 /* void* */, angle: c_double /* double */);
    pub fn wxGraphicsContext_Scale(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */);
    pub fn wxGraphicsContext_Translate(self_: *u8 /* void* */, dx: c_double /* double */, dy: c_double /* double */);
    pub fn wxGraphicsContext_SetTransform(self_: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxGraphicsContext_ConcatTransform(self_: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxGraphicsContext_SetBrush(self_: *u8 /* void* */, brush: *u8 /* void* */);
    pub fn wxGraphicsContext_SetGraphicsBrush(self_: *u8 /* void* */, brush: *u8 /* void* */);
    pub fn wxGraphicsContext_SetFont(self_: *u8 /* void* */, font: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxGraphicsContext_SetGraphicsFont(self_: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxGraphicsContext_SetPen(self_: *u8 /* void* */, pen: *u8 /* void* */);
    pub fn wxGraphicsContext_SetGraphicsPen(self_: *u8 /* void* */, pen: *u8 /* void* */);
    pub fn wxGraphicsContext_StrokeLine(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsContext_StrokeLines(self_: *u8 /* void* */, n: size_t /* size_t */, x: *u8 /* void* */, y: *u8 /* void* */, style: c_int /* int */);
    pub fn wxGraphicsFont_Create() -> *u8 /* void* */;
    pub fn wxGraphicsFont_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsMatrix_Create() -> *u8 /* void* */;
    pub fn wxGraphicsMatrix_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsMatrix_Concat(self_: *u8 /* void* */, t: *u8 /* void* */);
    pub fn wxGraphicsMatrix_Get(self_: *u8 /* void* */, a: *c_double /* double* */, b: *c_double /* double* */, c: *c_double /* double* */, d: *c_double /* double* */, tx: *c_double /* double* */, ty: *c_double /* double* */);
    pub fn wxGraphicsMatrix_GetNativeMatrix(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsMatrix_Invert(self_: *u8 /* void* */);
    pub fn wxGraphicsMatrix_IsEqual(self_: *u8 /* void* */, t: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGraphicsMatrix_IsIdentity(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGraphicsMatrix_Rotate(self_: *u8 /* void* */, angle: c_double /* double */);
    pub fn wxGraphicsMatrix_Scale(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */);
    pub fn wxGraphicsMatrix_Set(self_: *u8 /* void* */, a: c_double /* double */, b: c_double /* double */, c: c_double /* double */, d: c_double /* double */, tx: c_double /* double */, ty: c_double /* double */);
    pub fn wxGraphicsMatrix_Translate(self_: *u8 /* void* */, dx: c_double /* double */, dy: c_double /* double */);
    pub fn wxGraphicsMatrix_TransformPoint(self_: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */);
    pub fn wxGraphicsMatrix_TransformDistance(self_: *u8 /* void* */, dx: *c_double /* double* */, dy: *c_double /* double* */);
    pub fn wxGraphicsObject_GetRenderer() -> *u8 /* void* */;
    pub fn wxGraphicsObject_IsNull(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGraphicsPath_Create() -> *u8 /* void* */;
    pub fn wxGraphicsPath_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsPath_MoveToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */);
    pub fn wxGraphicsPath_AddArc(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, r: c_double /* double */, startAngle: c_double /* double */, endAngle: c_double /* double */, clockwise: bool /* bool */);
    pub fn wxGraphicsPath_AddArcToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, r: c_double /* double */);
    pub fn wxGraphicsPath_AddCircle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, r: c_double /* double */);
    pub fn wxGraphicsPath_AddCurveToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, arg4: c_double /* double */, arg5: c_double /* double */);
    pub fn wxGraphicsPath_AddEllipse(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsPath_AddLineToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */);
    pub fn wxGraphicsPath_AddPath(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, path: *u8 /* void* */);
    pub fn wxGraphicsPath_AddQuadCurveToPoint(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsPath_AddRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */);
    pub fn wxGraphicsPath_AddRoundedRectangle(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, arg2: c_double /* double */, arg3: c_double /* double */, radius: c_double /* double */);
    pub fn wxGraphicsPath_CloseSubpath(self_: *u8 /* void* */);
    pub fn wxGraphicsPath_Contains(self_: *u8 /* void* */, arg0: c_double /* double */, arg1: c_double /* double */, style: c_int /* int */);
    pub fn wxGraphicsPath_GetBox(self_: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */, arg2: *c_double /* double* */, arg3: *c_double /* double* */);
    pub fn wxGraphicsPath_GetCurrentPoint(self_: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */);
    pub fn wxGraphicsPath_Transform(self_: *u8 /* void* */, matrix: *u8 /* void* */);
    pub fn wxGraphicsPath_GetNativePath(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsPath_UnGetNativePath(p: *u8 /* void* */);
    pub fn wxGraphicsPen_Create() -> *u8 /* void* */;
    pub fn wxGraphicsPen_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsRenderer_Delete(self_: *u8 /* void* */);
    pub fn wxGraphicsRenderer_GetDefaultRenderer(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsRenderer_CreateContext(dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsRenderer_CreateContextFromWindow(window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsRenderer_CreateContextFromNativeContext(context: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGraphicsRenderer_CreateContextFromNativeWindow(window: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxCondition)
    pub fn wxCondition_Broadcast(_obj: *u8 /* void* */);
    pub fn wxCondition_Create(_mut: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCondition_Delete(_obj: *u8 /* void* */);
    pub fn wxCondition_Signal(_obj: *u8 /* void* */);
    pub fn wxCondition_Wait(_obj: *u8 /* void* */);
    pub fn wxCondition_WaitFor(_obj: *u8 /* void* */, sec: c_int /* int */, nsec: c_int /* int */) -> c_int /* int */;
    
    // TClassDefExtend(wxGridTableBase,wxObject)
    
    // TClassDefExtend(wxApp,wxEvtHandler)
    
    // TClassDefExtend(wxCommand,wxObject)
    
    // TClassDef(wxStreamBase)
    pub fn wxStreamBase_GetLastError(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStreamBase_GetSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStreamBase_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxMDIParentFrame,wxFrame)
    pub fn wxMDIParentFrame_ActivateNext(_obj: *u8 /* void* */);
    pub fn wxMDIParentFrame_ActivatePrevious(_obj: *u8 /* void* */);
    pub fn wxMDIParentFrame_ArrangeIcons(_obj: *u8 /* void* */);
    pub fn wxMDIParentFrame_Cascade(_obj: *u8 /* void* */);
    pub fn wxMDIParentFrame_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMDIParentFrame_GetActiveChild(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMDIParentFrame_GetClientWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMDIParentFrame_GetWindowMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMDIParentFrame_OnCreateClient(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMDIParentFrame_SetWindowMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    pub fn wxMDIParentFrame_Tile(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxFilterOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxClientDC,wxWindowDC)
    pub fn wxClientDC_Create(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClientDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxWizardPage,wxPanel)
    
    // TClassDefExtend(wxTextCtrl,wxControl)
    pub fn wxTextCtrl_AppendText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxTextCtrl_CanCopy(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_CanCut(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_CanPaste(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_CanRedo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_CanUndo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_ChangeValue(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxTextCtrl_Clear(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_Copy(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    pub fn wxTextCtrl_Cut(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_DiscardEdits(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_GetInsertionPoint(_obj: *u8 /* void* */) -> c_long /* long */;
    pub fn wxTextCtrl_GetLastPosition(_obj: *u8 /* void* */) -> c_long /* long */;
    pub fn wxTextCtrl_GetLineLength(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> c_int /* int */;
    pub fn wxTextCtrl_GetLineText(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> *u8 /* void* */;
    pub fn wxTextCtrl_GetNumberOfLines(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTextCtrl_GetSelection(_obj: *u8 /* void* */, from: *u8 /* void* */, to: *u8 /* void* */);
    pub fn wxTextCtrl_GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextCtrl_IsEditable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_IsModified(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_Paste(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_PositionToXY(_obj: *u8 /* void* */, pos: c_long /* long */, x: *c_long /* long* */, y: *c_long /* long* */) -> c_int /* int */;
    pub fn wxTextCtrl_Redo(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_Remove(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */);
    pub fn wxTextCtrl_Replace(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */, value: *u8 /* void* */);
    pub fn wxTextCtrl_SaveFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */);
    pub fn wxTextCtrl_SetInsertionPoint(_obj: *u8 /* void* */, pos: c_long /* long */);
    pub fn wxTextCtrl_SetInsertionPointEnd(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_SetSelection(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */);
    pub fn wxTextCtrl_SetValue(_obj: *u8 /* void* */, value: *u8 /* void* */);
    pub fn wxTextCtrl_ShowPosition(_obj: *u8 /* void* */, pos: c_long /* long */);
    pub fn wxTextCtrl_Undo(_obj: *u8 /* void* */);
    pub fn wxTextCtrl_WriteText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxTextCtrl_XYToPosition(_obj: *u8 /* void* */, arg0: c_long /* long */, arg1: c_long /* long */) -> c_long /* long */;
    
    // TClassDefExtend(wxConnection,wxConnectionBase)
    
    // TClassDefExtend(wxTimerEx,wxTimer)
    
    // TClassDefExtend(wxTimerEx,wxTimer)
    pub fn wxTimerEx_Connect(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    pub fn wxTimerEx_Create() -> *u8 /* void* */;
    pub fn wxTimerEx_GetClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenu_AppendRadioItem(self_: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */);
    pub fn wxMenuItem_CreateSeparator() -> *u8 /* void* */;
    pub fn wxMenuItem_CreateEx(id: c_int /* int */, label: *u8 /* void* */, help: *u8 /* void* */, itemkind: c_int /* int */, submenu: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxToolBar_AddTool2(_obj: *u8 /* void* */, toolId: c_int /* int */, label: *u8 /* void* */, bmp: *u8 /* void* */, bmpDisabled: *u8 /* void* */, itemKind: c_int /* int */, shortHelp: *u8 /* void* */, longHelp: *u8 /* void* */);
    pub fn wxProgressDialog_Create(title: *u8 /* void* */, message: *u8 /* void* */, max: c_int /* int */, parent: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxProgressDialog_Update(obj: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */;
    pub fn wxProgressDialog_UpdateWithMessage(obj: *u8 /* void* */, value: c_int /* int */, message: *u8 /* void* */) -> bool /* bool */;
    pub fn wxProgressDialog_Resume(obj: *u8 /* void* */);
    pub fn wxVersionNumber() -> c_int /* int */;
    pub fn wxIsDefined(s: *wchar_t /* wchar_t* */) -> c_int /* int */;
    
    // TClassDefExtend(wxFrame,wxTopLevelWindow)
    pub fn wxFrame_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFrame_CreateStatusBar(_obj: *u8 /* void* */, number: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFrame_CreateToolBar(_obj: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */;
    pub fn wxFrame_GetClientAreaOrigin_left(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFrame_GetClientAreaOrigin_top(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFrame_GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrame_GetStatusBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrame_GetToolBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrame_Restore(_obj: *u8 /* void* */);
    pub fn wxFrame_SetMenuBar(_obj: *u8 /* void* */, menubar: *u8 /* void* */);
    pub fn wxFrame_SetStatusBar(_obj: *u8 /* void* */, statBar: *u8 /* void* */);
    pub fn wxFrame_SetStatusText(_obj: *u8 /* void* */, _txt: *u8 /* void* */, _number: c_int /* int */);
    pub fn wxFrame_SetStatusWidths(_obj: *u8 /* void* */, _n: c_int /* int */, _widths_field: *u8 /* void* */);
    pub fn wxFrame_SetToolBar(_obj: *u8 /* void* */, _toolbar: *u8 /* void* */);
    
    // TClassDef(wxDateTime)
    pub fn wxDateTime_AddDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDateTime_AddDateValues(_obj: *u8 /* void* */, _yrs: c_int /* int */, _mnt: c_int /* int */, _wek: c_int /* int */, _day: c_int /* int */);
    pub fn wxDateTime_AddTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDateTime_AddTimeValues(_obj: *u8 /* void* */, _hrs: c_int /* int */, _min: c_int /* int */, _sec: c_int /* int */, _mls: c_int /* int */);
    pub fn wxDateTime_ConvertYearToBC(year: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_Create() -> *u8 /* void* */;
    pub fn wxDateTime_Format(_obj: *u8 /* void* */, format: *u8 /* void* */, tz: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDateTime_FormatDate(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_FormatISODate(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_FormatISOTime(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_FormatTime(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_GetAmString() -> *u8 /* void* */;
    pub fn wxDateTime_GetBeginDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */);
    pub fn wxDateTime_GetCentury(year: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetCountry() -> c_int /* int */;
    pub fn wxDateTime_GetCurrentMonth(cal: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetCurrentYear(cal: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetDay(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetDayOfYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetEndDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */);
    pub fn wxDateTime_GetHour(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxDateTime_GetLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxDateTime_GetMillisecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetMinute(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetMonth(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetMonthName(month: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDateTime_GetNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxDateTime_GetNumberOfDays(year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetNumberOfDaysMonth(month: c_int /* int */, year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetPmString() -> *u8 /* void* */;
    pub fn wxDateTime_GetPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxDateTime_GetSecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetTicks(_obj: *u8 /* void* */) -> time_t /* time_t */;
    pub fn wxDateTime_GetTimeNow() -> c_int /* int */;
    pub fn wxDateTime_GetValue(_obj: *u8 /* void* */, hi_long: *u8 /* void* */, lo_long: *u8 /* void* */);
    pub fn wxDateTime_GetWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxDateTime_GetWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxDateTime_GetWeekDayName(weekday: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDateTime_GetWeekDayTZ(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetWeekOfMonth(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetWeekOfYear(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_GetYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    pub fn wxDateTime_IsBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsDST(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_IsDSTApplicable(year: c_int /* int */, country: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_IsEarlierThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsEqualTo(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsEqualUpTo(_obj: *u8 /* void* */, dt: *u8 /* void* */, ts: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsGregorianDate(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_IsLaterThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsLeapYear(year: c_int /* int */, cal: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_IsSameDate(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsSameTime(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsStrictlyBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsValid(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDateTime_IsWestEuropeanCountry(country: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_IsWorkDay(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_MakeGMT(_obj: *u8 /* void* */, noDST: c_int /* int */);
    pub fn wxDateTime_MakeTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */);
    pub fn wxDateTime_Now(dt: *u8 /* void* */);
    pub fn wxDateTime_ParseDate(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_ParseDateTime(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_ParseFormat(_obj: *u8 /* void* */, date: *u8 /* void* */, format: *u8 /* void* */, dateDef: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_ParseRfc822Date(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_ParseTime(_obj: *u8 /* void* */, time: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDateTime_ResetTime(_obj: *u8 /* void* */);
    pub fn wxDateTime_Set(_obj: *u8 /* void* */, day: c_int /* int */, month: c_int /* int */, year: c_int /* int */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */);
    pub fn wxDateTime_SetCountry(country: c_int /* int */);
    pub fn wxDateTime_SetDay(_obj: *u8 /* void* */, day: c_int /* int */);
    pub fn wxDateTime_SetHour(_obj: *u8 /* void* */, hour: c_int /* int */);
    pub fn wxDateTime_SetMillisecond(_obj: *u8 /* void* */, millisecond: c_int /* int */);
    pub fn wxDateTime_SetMinute(_obj: *u8 /* void* */, minute: c_int /* int */);
    pub fn wxDateTime_SetMonth(_obj: *u8 /* void* */, month: c_int /* int */);
    pub fn wxDateTime_SetSecond(_obj: *u8 /* void* */, second: c_int /* int */);
    pub fn wxDateTime_SetTime(_obj: *u8 /* void* */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */);
    pub fn wxDateTime_SetToCurrent(_obj: *u8 /* void* */);
    pub fn wxDateTime_SetToLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */);
    pub fn wxDateTime_SetToLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_SetToNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */);
    pub fn wxDateTime_SetToPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */);
    pub fn wxDateTime_SetToWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */;
    pub fn wxDateTime_SetToWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */);
    pub fn wxDateTime_SetYear(_obj: *u8 /* void* */, year: c_int /* int */);
    pub fn wxDateTime_SubtractDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDateTime_SubtractTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxDateTime_ToGMT(_obj: *u8 /* void* */, noDST: c_int /* int */);
    pub fn wxDateTime_ToTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */);
    pub fn wxDateTime_Today(dt: *u8 /* void* */);
    pub fn wxDateTime_UNow(dt: *u8 /* void* */);
    pub fn wxDateTime_wxDateTime(hi_long: c_int /* int */, lo_long: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(ELJGridTable,wxGridTableBase)
    pub fn ELJGridTable_Create(_obj: *u8 /* void* */, _EifGetNumberRows: *u8 /* void* */, _EifGetNumberCols: *u8 /* void* */, _EifGetValue: *u8 /* void* */, _EifSetValue: *u8 /* void* */, _EifIsEmptyCell: *u8 /* void* */, _EifClear: *u8 /* void* */, _EifInsertRows: *u8 /* void* */, _EifAppendRows: *u8 /* void* */, _EifDeleteRows: *u8 /* void* */, _EifInsertCols: *u8 /* void* */, _EifAppendCols: *u8 /* void* */, _EifDeleteCols: *u8 /* void* */, _EifSetRowLabelValue: *u8 /* void* */, _EifSetColLabelValue: *u8 /* void* */, _EifGetRowLabelValue: *u8 /* void* */, _EifGetColLabelValue: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJGridTable_Delete(_obj: *u8 /* void* */);
    pub fn ELJGridTable_GetView(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJGridTable_SendTableMessage(_obj: *u8 /* void* */, id: c_int /* int */, val1: c_int /* int */, val2: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxTabCtrl,wxControl)
    
    // TClassDefExtend(wxQueryNewPaletteEvent,wxEvent)
    pub fn wxQueryNewPaletteEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxQueryNewPaletteEvent_GetPaletteRealized(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxQueryNewPaletteEvent_SetPaletteRealized(_obj: *u8 /* void* */, realized: bool /* bool */);
    
    // TClassDefExtend(wxBitmapButton,wxButton)
    pub fn wxBitmapButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmapButton_GetBitmapDisabled(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxBitmapButton_GetBitmapFocus(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxBitmapButton_GetBitmapLabel(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxBitmapButton_GetBitmapSelected(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxBitmapButton_GetMarginX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBitmapButton_GetMarginY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBitmapButton_SetBitmapDisabled(_obj: *u8 /* void* */, disabled: *u8 /* void* */);
    pub fn wxBitmapButton_SetBitmapFocus(_obj: *u8 /* void* */, focus: *u8 /* void* */);
    pub fn wxBitmapButton_SetBitmapLabel(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    pub fn wxBitmapButton_SetBitmapSelected(_obj: *u8 /* void* */, sel: *u8 /* void* */);
    pub fn wxBitmapButton_SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(wxRecordSet,wxObject)
    
    // TClassDef(wxStringBuffer)
    
    // TClassDefExtend(cbDrawRowBkGroundEvent,cbPluginEvent)
    pub fn cbDrawRowBkGroundEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDrawRowBkGroundEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxHtmlTagHandler,wxObject)
    
    // TClassDefExtend(wxSizerItem,wxObject)
    pub fn wxSizerItem_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_Create(arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_CreateInSizer(sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_CreateInWindow(window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetBorder(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSizerItem_GetFlag(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSizerItem_GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetRatio(_obj: *u8 /* void* */) -> float /* float */;
    pub fn wxSizerItem_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetUserData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_IsSizer(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizerItem_IsSpacer(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizerItem_IsWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizerItem_SetBorder(_obj: *u8 /* void* */, border: c_int /* int */);
    pub fn wxSizerItem_SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxSizerItem_SetFlag(_obj: *u8 /* void* */, flag: c_int /* int */);
    pub fn wxSizerItem_SetFloatRatio(_obj: *u8 /* void* */, ratio: float /* float */);
    pub fn wxSizerItem_SetInitSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizerItem_SetRatio(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizerItem_SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */);
    pub fn wxSizerItem_SetWindow(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxSizerItem_Delete(_obj: *u8 /* void* */);
    pub fn wxSizerItem_DeleteWindows(_obj: *u8 /* void* */);
    pub fn wxSizerItem_DetachSizer(_obj: *u8 /* void* */);
    pub fn wxSizerItem_GetProportion(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSizerItem_GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_GetSpacer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizerItem_IsShown(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSizerItem_SetProportion(_obj: *u8 /* void* */, proportion: c_int /* int */);
    pub fn wxSizerItem_SetSpacer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizerItem_Show(_obj: *u8 /* void* */, show: c_int /* int */);
    
    // TClassDefExtend(wxMenuBar,wxEvtHandler)
    pub fn wxMenuBar_Append(_obj: *u8 /* void* */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenuBar_Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */);
    pub fn wxMenuBar_Create(_style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_DeletePointer(_obj: *u8 /* void* */);
    pub fn wxMenuBar_Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> c_int /* int */;
    pub fn wxMenuBar_EnableItem(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */);
    pub fn wxMenuBar_EnableTop(_obj: *u8 /* void* */, pos: c_int /* int */, enable: bool /* bool */);
    pub fn wxMenuBar_FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_FindMenu(_obj: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenuBar_FindMenuItem(_obj: *u8 /* void* */, menuString: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenuBar_GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_GetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_GetMenu(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_GetMenuCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenuBar_Insert(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenuBar_IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxMenuBar_IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxMenuBar_Remove(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenuBar_Replace(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuBar_SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */);
    pub fn wxMenuBar_SetItemLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */);
    pub fn wxMenuBar_SetLabel(_obj: *u8 /* void* */, s: *u8 /* void* */);
    pub fn wxMenuBar_SetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */, label: *u8 /* void* */);
    
    // TClassDefExtend(cbCloseBox,cbMiniButton)
    pub fn cbCloseBox_Create() -> *u8 /* void* */;
    
    // TClassDefExtend(wxMetafileDC,wxDC)
    pub fn wxMetafileDC_Close(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMetafileDC_Create(_file: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMetafileDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDef(wxDataOutputStream)
    
    // TClassDefExtend(wxGridCellEditor,wxGridCellWorker)
    pub fn wxGridCellEditor_BeginEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */);
    pub fn wxGridCellEditor_Create(_obj: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, evtHandler: *u8 /* void* */);
    pub fn wxGridCellEditor_Destroy(_obj: *u8 /* void* */);
    pub fn wxGridCellEditor_EndEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */, oldStr: *u8 /* void* */, newStr: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridCellEditor_GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridCellEditor_HandleReturn(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxGridCellEditor_IsAcceptedKey(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellEditor_IsCreated(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellEditor_PaintBackground(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, attr: *u8 /* void* */);
    pub fn wxGridCellEditor_Reset(_obj: *u8 /* void* */);
    pub fn wxGridCellEditor_SetControl(_obj: *u8 /* void* */, control: *u8 /* void* */);
    pub fn wxGridCellEditor_SetParameters(_obj: *u8 /* void* */, params: *u8 /* void* */);
    pub fn wxGridCellEditor_SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxGridCellEditor_Show(_obj: *u8 /* void* */, show: c_int /* int */, attr: *u8 /* void* */);
    pub fn wxGridCellEditor_StartingClick(_obj: *u8 /* void* */);
    pub fn wxGridCellEditor_StartingKey(_obj: *u8 /* void* */, event: *u8 /* void* */);
    
    // TClassDefExtend(wxBufferedDC,wxDC)
    pub fn wxBufferedDC_CreateByDCAndSize(dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedDC_CreateByDCAndBitmap(dc: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedDC_Delete(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxLogPassThrough,wxLogChain)
    
    // TClassDefExtend(wxTimer,wxObject)
    pub fn wxTimer_Create(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTimer_Delete(_obj: *u8 /* void* */);
    pub fn wxTimer_GetInterval(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTimer_IsOneShot(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTimer_IsRuning(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTimer_Start(_obj: *u8 /* void* */, _int: c_int /* int */, _one: bool /* bool */) -> bool /* bool */;
    pub fn wxTimer_Stop(_obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJArtProv,wxArtProvider)
    pub fn ELJArtProv_Create(_obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJArtProv_Release(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxXmlResourceHandler,wxObject)
    
    // TClassDef(wxExpr)
    
    // TClassDefExtend(wxWizardEvent,wxNotifyEvent)
    pub fn wxWizardEvent_GetDirection(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxMediaEvent,wxNotifyEvent);
    
    // TClassDefExtend(wxSplitterScrolledWindow,wxScrolledWindow)
    pub fn wxSplitterScrolledWindow_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDocManager,wxEvtHandler)
    
    // TClassDefExtend(wxDatabase,wxObject)
    
    // TClassDefExtend(wxPGProperty,wxObject)
    pub fn wxPGProperty_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_GetValueAsString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_GetValueType(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_SetHelpString(_obj: *u8 /* void* */, helpString: *u8 /* void* */);
    
    // TClassDefExtend(wxList,wxObject)
    
    // TClassDefExtend(wxDynamicToolBar,wxToolBarBase)
    pub fn wxDynamicToolBar_AddSeparator(_obj: *u8 /* void* */, pSepartorWnd: *u8 /* void* */);
    pub fn wxDynamicToolBar_AddTool(_obj: *u8 /* void* */, toolIndex: c_int /* int */, pToolWindow: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxDynamicToolBar_AddToolBitmap(_obj: *u8 /* void* */, toolIndex: c_int /* int */, bitmap: *u8 /* void* */, pushedBitmap: *u8 /* void* */, toggle: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, clientData: *u8 /* void* */, helpString1: *u8 /* void* */, helpString2: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_AddToolImage(_obj: *u8 /* void* */, toolIndex: c_int /* int */, imageFileName: *u8 /* void* */, imageFileType: c_int /* int */, labelText: *u8 /* void* */, alignTextRight: c_int /* int */, isFlat: bool /* bool */);
    pub fn wxDynamicToolBar_AddToolLabel(_obj: *u8 /* void* */, toolIndex: c_int /* int */, labelBmp: *u8 /* void* */, labelText: *u8 /* void* */, alignTextRight: c_int /* int */, isFlat: bool /* bool */);
    pub fn wxDynamicToolBar_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, orientation: c_int /* int */, RowsOrColumns: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_CreateDefault() -> *u8 /* void* */;
    pub fn wxDynamicToolBar_CreateDefaultLayout(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_CreateParams(_obj: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, orientation: c_int /* int */, RowsOrColumns: c_int /* int */) -> c_int /* int */;
    pub fn wxDynamicToolBar_CreateTool(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, bmpNormal: *u8 /* void* */, bmpDisabled: *u8 /* void* */, kind: c_int /* int */, clientData: *u8 /* void* */, shortHelp: *u8 /* void* */, longHelp: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_CreateToolControl(_obj: *u8 /* void* */, control: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_Delete(_obj: *u8 /* void* */);
    pub fn wxDynamicToolBar_DoDeleteTool(_obj: *u8 /* void* */, pos: c_int /* int */, tool: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDynamicToolBar_DoEnableTool(_obj: *u8 /* void* */, tool: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxDynamicToolBar_DoInsertTool(_obj: *u8 /* void* */, pos: c_int /* int */, tool: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDynamicToolBar_DoSetToggle(_obj: *u8 /* void* */, tool: *u8 /* void* */, toggle: c_int /* int */);
    pub fn wxDynamicToolBar_DoToggleTool(_obj: *u8 /* void* */, tool: *u8 /* void* */, toggle: c_int /* int */);
    pub fn wxDynamicToolBar_DrawSeparator(_obj: *u8 /* void* */, info: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxDynamicToolBar_EnableTool(_obj: *u8 /* void* */, toolIndex: c_int /* int */, enable: bool /* bool */);
    pub fn wxDynamicToolBar_FindToolForPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_GetPreferredDim(_obj: *u8 /* void* */, gw: c_int /* int */, gh: c_int /* int */, pw: *u8 /* void* */, ph: *u8 /* void* */);
    pub fn wxDynamicToolBar_GetToolInfo(_obj: *u8 /* void* */, toolIndex: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDynamicToolBar_Layout(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDynamicToolBar_RemoveTool(_obj: *u8 /* void* */, toolIndex: c_int /* int */);
    pub fn wxDynamicToolBar_SetLayout(_obj: *u8 /* void* */, pLayout: *u8 /* void* */);
    
    // TClassDefExtend(wxDirDialog,wxDialog)
    pub fn wxDirDialog_Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDirDialog_GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDirDialog_GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDirDialog_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDirDialog_SetMessage(_obj: *u8 /* void* */, msg: *u8 /* void* */);
    pub fn wxDirDialog_SetPath(_obj: *u8 /* void* */, pth: *u8 /* void* */);
    pub fn wxDirDialog_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    
    // TClassDefExtend(cbCustomizeBarEvent,cbPluginEvent)
    pub fn cbCustomizeBarEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbCustomizeBarEvent_ClickPos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDef(wxArray)
    
    // TClassDefExtend(wxDialUpEvent,wxEvent)
    pub fn wxDialUpEvent_IsConnectedEvent(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialUpEvent_IsOwnEvent(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(cbCommonPaneProperties,wxObject)
    pub fn cbCommonPaneProperties_Assign(_obj: *u8 /* void* */, _other: *u8 /* void* */);
    pub fn cbCommonPaneProperties_BarCollapseIconsOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_BarDragHintsOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_BarFloatingOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_ColProportionsOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_CreateDefault() -> *u8 /* void* */;
    pub fn cbCommonPaneProperties_Delete(_obj: *u8 /* void* */);
    pub fn cbCommonPaneProperties_ExactDockPredictionOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_MinCBarDim(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn cbCommonPaneProperties_NonDestructFrictionOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_OutOfPaneDragOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_RealTimeUpdatesOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_ResizeHandleSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_RowProportionsOn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbCommonPaneProperties_SetBarCollapseIconsOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetBarDragHintsOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetBarFloatingOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetColProportionsOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetExactDockPredictionOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetMinCBarDim(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn cbCommonPaneProperties_SetNonDestructFrictionOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetOutOfPaneDragOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetRealTimeUpdatesOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetResizeHandleSize(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetRowProportionsOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_SetShow3DPaneBorderOn(_obj: *u8 /* void* */, _val: c_int /* int */);
    pub fn cbCommonPaneProperties_Show3DPaneBorderOn(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDef(wxDirTraverser)
    
    // TClassDefExtend(wxColourData,wxObject)
    pub fn wxColourData_Create() -> *u8 /* void* */;
    pub fn wxColourData_Delete(_obj: *u8 /* void* */);
    pub fn wxColourData_GetChooseFull(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxColourData_GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxColourData_GetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxColourData_SetChooseFull(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxColourData_SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxColourData_SetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, colour: *u8 /* void* */);
    
    // TClassDefExtend(wxTaskBarIcon,wxEvtHandler)
    pub fn wxTaskBarIcon_Create() -> *u8 /* void* */;
    pub fn wxTaskBarIcon_Delete(_obj: *u8 /* void* */);
    pub fn wxTaskBarIcon_IsIconInstalled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_RemoveIcon(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, text: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxStaticBoxSizer,wxBoxSizer)
    pub fn wxStaticBoxSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStaticBoxSizer_Create(box: *u8 /* void* */, orient: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStaticBoxSizer_GetStaticBox(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStaticBoxSizer_RecalcSizes(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxTime,wxObject)
    
    // TClassDefExtend(wxMouseEvent,wxEvent)
    pub fn wxMouseEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_Button(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxMouseEvent_ButtonDClick(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxMouseEvent_ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxMouseEvent_ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxMouseEvent_ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    pub fn wxMouseEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxMouseEvent_Dragging(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_Entering(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_GetLogicalPosition(_obj: *u8 /* void* */, dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMouseEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMouseEvent_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMouseEvent_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMouseEvent_IsButton(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_Leaving(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_LeftDClick(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_LeftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_LeftIsDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_LeftUp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_MiddleDClick(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_MiddleDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_MiddleIsDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_MiddleUp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_Moving(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_RightDClick(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_RightDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_RightIsDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_RightUp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMouseEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(ELJApp,wxApp)
    pub fn ELJApp_Bell();
    pub fn ELJApp_CreateLogTarget() -> *u8 /* void* */;
    pub fn ELJApp_Dispatch();
    pub fn ELJApp_DisplaySize() -> *u8 /* void* */;
    pub fn ELJApp_EnableTooltips(_enable: bool /* bool */);
    pub fn ELJApp_EnableTopLevelWindows(_enb: c_int /* int */);
    pub fn ELJApp_ExecuteProcess(_cmd: *u8 /* void* */, _snc: c_int /* int */, _prc: *u8 /* void* */) -> c_int /* int */;
    pub fn ELJApp_Exit();
    pub fn ELJApp_ExitMainLoop();
    pub fn ELJApp_FindWindowById(_id: c_int /* int */, _prt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJApp_FindWindowByLabel(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJApp_FindWindowByName(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJApp_GetApp() -> *u8 /* void* */;
    pub fn ELJApp_GetAppName() -> *u8 /* void* */;
    pub fn ELJApp_GetClassName() -> *u8 /* void* */;
    pub fn ELJApp_GetExitOnFrameDelete() -> c_int /* int */;
    pub fn ELJApp_GetOsDescription() -> *u8 /* void* */;
    pub fn ELJApp_GetOsVersion(_maj: *u8 /* void* */, _min: *u8 /* void* */) -> c_int /* int */;
    pub fn ELJApp_GetTopWindow() -> *u8 /* void* */;
    pub fn ELJApp_GetUseBestVisual() -> c_int /* int */;
    pub fn ELJApp_GetUserHome(_usr: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJApp_GetUserId() -> *u8 /* void* */;
    pub fn ELJApp_GetUserName() -> *u8 /* void* */;
    pub fn ELJApp_GetVendorName() -> *u8 /* void* */;
    pub fn ELJApp_InitAllImageHandlers();
    pub fn ELJApp_Initialized() -> bool /* bool */;
    pub fn ELJApp_MainLoop() -> c_int /* int */;
    pub fn ELJApp_MousePosition() -> *u8 /* void* */;
    pub fn ELJApp_Pending() -> c_int /* int */;
    pub fn ELJApp_SafeYield(_win: *u8 /* void* */) -> c_int /* int */;
    pub fn ELJApp_SetAppName(name: *u8 /* void* */);
    pub fn ELJApp_SetClassName(name: *u8 /* void* */);
    pub fn ELJApp_SetExitOnFrameDelete(flag: c_int /* int */);
    pub fn ELJApp_SetPrintMode(mode: c_int /* int */);
    pub fn ELJApp_SetTooltipDelay(_ms: c_int /* int */);
    pub fn ELJApp_SetTopWindow(_wnd: *u8 /* void* */);
    pub fn ELJApp_SetUseBestVisual(flag: c_int /* int */);
    pub fn ELJApp_SetVendorName(name: *u8 /* void* */);
    pub fn ELJApp_Sleep(_scs: c_int /* int */);
    pub fn ELJApp_MilliSleep(_mscs: c_int /* int */);
    pub fn ELJApp_Yield() -> c_int /* int */;
    pub fn ELJApp_IsTerminating() -> c_int /* int */;
    
    // TClassDefExtend(wxTabEvent,wxCommandEvent)
    
    // TClassDefExtend(cbLeftDClickEvent,cbPluginEvent)
    pub fn cbLeftDClickEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxMBConvFile,wxMBConv)
    
    // TClassDef(wxCaret)
    pub fn wxCaret_Create(_wnd: *u8 /* void* */, _wth: c_int /* int */, _hgt: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCaret_GetBlinkTime() -> c_int /* int */;
    pub fn wxCaret_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCaret_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCaret_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCaret_Hide(_obj: *u8 /* void* */);
    pub fn wxCaret_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCaret_IsVisible(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCaret_Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxCaret_SetBlinkTime(milliseconds: c_int /* int */);
    pub fn wxCaret_SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxCaret_Show(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxHelpControllerHelpProvider,wxSimpleHelpProvider)
    pub fn wxHelpControllerHelpProvider_Create(ctr: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpControllerHelpProvider_GetHelpController(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpControllerHelpProvider_SetHelpController(_obj: *u8 /* void* */, hc: *u8 /* void* */);
    
    // TClassDefExtend(wxPlotWindow,wxScrolledWindow)
    pub fn wxPlotWindow_Add(_obj: *u8 /* void* */, curve: *u8 /* void* */);
    pub fn wxPlotWindow_AddOnOff(_obj: *u8 /* void* */, curve: *u8 /* void* */);
    pub fn wxPlotWindow_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPlotWindow_Delete(_obj: *u8 /* void* */, curve: *u8 /* void* */);
    pub fn wxPlotWindow_DeleteOnOff(_obj: *u8 /* void* */, curve: *u8 /* void* */);
    pub fn wxPlotWindow_Enlarge(_obj: *u8 /* void* */, curve: *u8 /* void* */, factor: c_double /* double */);
    pub fn wxPlotWindow_GetAt(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPlotWindow_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotWindow_GetCurrent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPlotWindow_GetEnlargeAroundWindowCentre(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotWindow_GetOnOffCurveAt(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPlotWindow_GetOnOffCurveCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotWindow_GetScrollOnThumbRelease(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotWindow_GetUnitsPerValue(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxPlotWindow_GetZoom(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxPlotWindow_Move(_obj: *u8 /* void* */, curve: *u8 /* void* */, pixels_up: c_int /* int */);
    pub fn wxPlotWindow_RedrawEverything(_obj: *u8 /* void* */);
    pub fn wxPlotWindow_RedrawXAxis(_obj: *u8 /* void* */);
    pub fn wxPlotWindow_RedrawYAxis(_obj: *u8 /* void* */);
    pub fn wxPlotWindow_ResetScrollbar(_obj: *u8 /* void* */);
    pub fn wxPlotWindow_SetCurrent(_obj: *u8 /* void* */, current: *u8 /* void* */);
    pub fn wxPlotWindow_SetEnlargeAroundWindowCentre(_obj: *u8 /* void* */, enlargeAroundWindowCentre: c_int /* int */);
    pub fn wxPlotWindow_SetScrollOnThumbRelease(_obj: *u8 /* void* */, scrollOnThumbRelease: c_int /* int */);
    pub fn wxPlotWindow_SetUnitsPerValue(_obj: *u8 /* void* */, upv: c_double /* double */);
    pub fn wxPlotWindow_SetZoom(_obj: *u8 /* void* */, zoom: c_double /* double */);
    
    // TClassDefExtend(wxSocketClient,wxSocketBase)
    
    // TClassDefExtend(wxPlotEvent,wxNotifyEvent)
    pub fn wxPlotEvent_GetCurve(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPlotEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotEvent_GetZoom(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxPlotEvent_SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxPlotEvent_SetZoom(_obj: *u8 /* void* */, zoom: c_double /* double */);
    
    // TClassDefExtend(wxCommandEvent,wxEvent)
    pub fn wxCommandEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxCommandEvent_Create(_typ: c_int /* int */, _id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCommandEvent_Delete(_obj: *u8 /* void* */);
    pub fn wxCommandEvent_GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCommandEvent_GetClientObject(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCommandEvent_GetExtraLong(_obj: *u8 /* void* */) -> c_long /* long */;
    pub fn wxCommandEvent_GetInt(_obj: *u8 /* void* */) -> c_long /* long */;
    pub fn wxCommandEvent_GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCommandEvent_GetString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCommandEvent_IsChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCommandEvent_IsSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCommandEvent_SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */);
    pub fn wxCommandEvent_SetClientObject(_obj: *u8 /* void* */, clientObject: *u8 /* void* */);
    pub fn wxCommandEvent_SetExtraLong(_obj: *u8 /* void* */, extraLong: c_long /* long */);
    pub fn wxCommandEvent_SetInt(_obj: *u8 /* void* */, i: c_int /* int */);
    pub fn wxCommandEvent_SetString(_obj: *u8 /* void* */, s: *u8 /* void* */);
    
    // TClassDefExtend(cbRemoveBarEvent,cbPluginEvent)
    pub fn cbRemoveBarEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxZlibOutputStream,wxFilterOutputStream)
    
    // TClassDefExtend(wxLogWindow,wxLogPassThrough)
    
    // TClassDefExtend(wxPaletteChangedEvent,wxEvent)
    pub fn wxPaletteChangedEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxPaletteChangedEvent_GetChangedWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPaletteChangedEvent_SetChangedWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    
    // TClassDefExtend(ELJClient,wxClient)
    pub fn ELJClient_Create(_eobj: *u8 /* void* */, _cnct: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJClient_Delete(_obj: *u8 /* void* */);
    pub fn ELJClient_MakeConnection(_obj: *u8 /* void* */, host: *u8 /* void* */, server: *u8 /* void* */, topic: *u8 /* void* */);
    
    // TClassDefExtend(wxGenericValidator,wxValidator)
    
    // TClassDefExtend(wxStyledTextEvent, wxCommandEvent);
    pub fn wxStyledTextEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetKey(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetModifiers(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetModificationType(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetLength(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetLinesAdded(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetLine(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetFoldLevelNow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetFoldLevelPrev(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetMargin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetMessage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetWParam(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetLParam(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetListType(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetDragText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextEvent_GetDragAllowMove(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextEvent_GetDragResult(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextEvent_GetShift(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextEvent_GetControl(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextEvent_GetAlt(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextEvent_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextEvent_Clone(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextEvent_SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextEvent_SetKey(_obj: *u8 /* void* */, k: c_int /* int */);
    pub fn wxStyledTextEvent_SetModifiers(_obj: *u8 /* void* */, m: c_int /* int */);
    pub fn wxStyledTextEvent_SetModificationType(_obj: *u8 /* void* */, t: c_int /* int */);
    pub fn wxStyledTextEvent_SetText(_obj: *u8 /* void* */, t: *u8 /* void* */);
    pub fn wxStyledTextEvent_SetLength(_obj: *u8 /* void* */, len: c_int /* int */);
    pub fn wxStyledTextEvent_SetLinesAdded(_obj: *u8 /* void* */, num: c_int /* int */);
    pub fn wxStyledTextEvent_SetLine(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetFoldLevelNow(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetFoldLevelPrev(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetMargin(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetMessage(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetWParam(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetLParam(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetListType(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetX(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetY(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn wxStyledTextEvent_SetDragText(_obj: *u8 /* void* */, val: *u8 /* void* */);
    pub fn wxStyledTextEvent_SetDragAllowMove(_obj: *u8 /* void* */, val: bool /* bool */);
    pub fn wxStyledTextEvent_SetDragResult(_obj: *u8 /* void* */, val: c_int /* int */);
    pub fn expEVT_STC_CHANGE() -> c_int /* int */;
    pub fn expEVT_STC_STYLENEEDED() -> c_int /* int */;
    pub fn expEVT_STC_CHARADDED() -> c_int /* int */;
    pub fn expEVT_STC_SAVEPOINTREACHED() -> c_int /* int */;
    pub fn expEVT_STC_SAVEPOINTLEFT() -> c_int /* int */;
    pub fn expEVT_STC_ROMODIFYATTEMPT() -> c_int /* int */;
    pub fn expEVT_STC_KEY() -> c_int /* int */;
    pub fn expEVT_STC_DOUBLECLICK() -> c_int /* int */;
    pub fn expEVT_STC_UPDATEUI() -> c_int /* int */;
    pub fn expEVT_STC_MODIFIED() -> c_int /* int */;
    pub fn expEVT_STC_MACRORECORD() -> c_int /* int */;
    pub fn expEVT_STC_MARGINCLICK() -> c_int /* int */;
    pub fn expEVT_STC_NEEDSHOWN() -> c_int /* int */;
    pub fn expEVT_STC_PAINTED() -> c_int /* int */;
    pub fn expEVT_STC_USERLISTSELECTION() -> c_int /* int */;
    pub fn expEVT_STC_URIDROPPED() -> c_int /* int */;
    pub fn expEVT_STC_DWELLSTART() -> c_int /* int */;
    pub fn expEVT_STC_DWELLEND() -> c_int /* int */;
    pub fn expEVT_STC_START_DRAG() -> c_int /* int */;
    pub fn expEVT_STC_DRAG_OVER() -> c_int /* int */;
    pub fn expEVT_STC_DO_DROP() -> c_int /* int */;
    pub fn expEVT_STC_ZOOM() -> c_int /* int */;
    pub fn expEVT_STC_HOTSPOT_CLICK() -> c_int /* int */;
    pub fn expEVT_STC_HOTSPOT_DCLICK() -> c_int /* int */;
    pub fn expEVT_STC_CALLTIP_CLICK() -> c_int /* int */;
    pub fn expEVT_STC_AUTOCOMP_SELECTION() -> c_int /* int */;
    
    // TClassDefExtend(wxLogChain,wxLog)
    pub fn wxLogChain_Create(logger: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLogChain_Delete(_obj: *u8 /* void* */);
    pub fn wxLogChain_GetOldLog(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLogChain_IsPassingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLogChain_PassMessages(_obj: *u8 /* void* */, bDoPass: bool /* bool */);
    pub fn wxLogChain_SetLog(_obj: *u8 /* void* */, logger: *u8 /* void* */);
    
    // TClassDefExtend(wxPreviewCanvas,wxScrolledWindow)
    pub fn wxPreviewCanvas_Create(preview: *u8 /* void* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxActivateEvent,wxEvent)
    pub fn wxActivateEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxActivateEvent_GetActive(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(cbDrawHintRectEvent,cbPluginEvent)
    pub fn cbDrawHintRectEvent_EraseRect(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDrawHintRectEvent_IsInClient(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn cbDrawHintRectEvent_LastTime(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDrawHintRectEvent_Rect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    
    // TClassDefExtend(cbInsertBarEvent,cbPluginEvent)
    pub fn cbInsertBarEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbInsertBarEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxHtmlColourCell,wxHtmlCell)
    
    // TClassDefExtend(cbDrawBarDecorEvent,cbPluginEvent)
    pub fn cbDrawBarDecorEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDrawBarDecorEvent_BoundsInParent(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn cbDrawBarDecorEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellTextEditor,wxGridCellEditor)
    pub fn wxGridCellTextEditor_Ctor() -> *u8 /* void* */;
    
    // TClassDefExtend(wxHTTP,wxProtocol)
    
    // TClassDefExtend(wxDocument,wxEvtHandler)
    
    // TClassDefExtend(cbBarHintsPlugin,cbPluginBase)
    pub fn cbBarHintsPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbBarHintsPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbBarHintsPlugin_Delete(_obj: *u8 /* void* */);
    pub fn cbBarHintsPlugin_SetGrooveCount(_obj: *u8 /* void* */, nGrooves: c_int /* int */);
    
    // TClassDefExtend(cbSizeBarWndEvent,cbPluginEvent)
    pub fn cbSizeBarWndEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbSizeBarWndEvent_BoundsInParent(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    
    // TClassDefExtend(wxPropertyCategory,wxPGProperty)
    pub fn wxPropertyCategory_Create(label: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxGLCanvas,wxScrolledWindow)
    
    // TClassDefExtend(wxGLCanvas,wxWindow);
    
    // TClassDefExtend(wxSystemOptions,wxObject)
    
    // TClassDefExtend(wxSizeEvent,wxEvent)
    pub fn wxSizeEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxSizeEvent_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxWizardPageSimple,wxWizardPage)
    pub fn wxWizardPageSimple_Create(_prt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizardPageSimple_GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxWizardPageSimple_GetNext(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizardPageSimple_GetPrev(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizardPageSimple_SetNext(_obj: *u8 /* void* */, next: *u8 /* void* */);
    pub fn wxWizardPageSimple_SetPrev(_obj: *u8 /* void* */, prev: *u8 /* void* */);
    
    // TClassDefExtend(wxFilterInputStream,wxInputStream)
    
    // TClassDefExtend(cbBarDragPlugin,cbPluginBase)
    pub fn cbBarDragPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbBarDragPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbBarDragPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDef(wxBusyInfo)
    pub fn wxBusyInfo_Create(_txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBusyInfo_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxInputSink,wxThread)
    pub fn wxInputSink_Create(input: *u8 /* void* */, evtHandler: *u8 /* void* */, bufferLen: c_int /* int */) -> *u8 /* void* */;
    pub fn wxInputSink_GetId(obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputSink_Start(obj: *u8 /* void* */);
    
    // TClassDefExtend(wxTimerEvent,wxEvent)
    pub fn wxTimerEvent_GetInterval(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxListEvent,wxNotifyEvent)
    pub fn wxListEvent_Cancelled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListEvent_GetCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetColumn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetData(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetImage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetIndex(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxListEvent_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxListEvent_GetMask(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxListEvent_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxColour,wxObject)
    pub fn wxColour_Alpha(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    pub fn wxColour_Assign(_obj: *u8 /* void* */, other: *u8 /* void* */);
    pub fn wxColour_Blue(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    pub fn wxColour_Copy(_obj: *u8 /* void* */, _other: *u8 /* void* */);
    pub fn wxColour_CreateByName(_name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxColour_CreateEmpty() -> *u8 /* void* */;
    pub fn wxColour_CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxColour_CreateRGB(_red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */) -> *u8 /* void* */;
    pub fn wxColour_Delete(_obj: *u8 /* void* */);
    pub fn wxColour_Green(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    pub fn wxColour_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxColour_Red(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    pub fn wxColour_Set(_obj: *u8 /* void* */, _red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */);
    pub fn wxColour_SetByName(_obj: *u8 /* void* */, _name: *u8 /* void* */);
    pub fn wxColour_ValidName(_name: *wchar_t /* wchar_t* */) -> bool /* bool */;
    
    // TClassDefExtend(wxGridCellStringRenderer,wxGridCellRenderer)
    
    // TClassDefExtend(wxTreeCompanionWindow,wxWindow)
    pub fn wxTreeCompanionWindow_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTreeCompanionWindow_DrawItem(_obj: *u8 /* void* */, dc: *u8 /* void* */, id: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxTreeCompanionWindow_GetTreeCtrl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCompanionWindow_SetTreeCtrl(_obj: *u8 /* void* */, treeCtrl: *u8 /* void* */);
    
    // TClassDefExtend(wxRegionIterator,wxObject)
    pub fn wxRegionIterator_Create() -> *u8 /* void* */;
    pub fn wxRegionIterator_CreateFromRegion(region: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxRegionIterator_Delete(_obj: *u8 /* void* */);
    pub fn wxRegionIterator_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRegionIterator_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRegionIterator_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRegionIterator_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRegionIterator_HaveRects(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRegionIterator_Next(_obj: *u8 /* void* */);
    pub fn wxRegionIterator_Reset(_obj: *u8 /* void* */);
    pub fn wxRegionIterator_ResetToRegion(_obj: *u8 /* void* */, region: *u8 /* void* */);
    
    // TClassDef(wxCriticalSectionLocker)
    
    // TClassDefExtend(wxHtmlWindow,wxScrolledWindow)
    
    // TClassDefExtend(wxFFileOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxHtmlHelpFrame,wxFrame)
    
    // TClassDefExtend(wxQueryCol,wxObject)
    
    // TClassDefExtend(wxLEDNumberCtrl,wxControl)
    pub fn wxLEDNumberCtrl_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxLEDNumberCtrl_GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxLEDNumberCtrl_GetDrawFaded(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxLEDNumberCtrl_GetValue(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn wxLEDNumberCtrl_SetAlignment(_obj: *u8 /* void* */, Alignment: c_int /* int */, Redraw: c_int /* int */);
    pub fn wxLEDNumberCtrl_SetDrawFaded(_obj: *u8 /* void* */, DrawFaded: c_int /* int */, Redraw: c_int /* int */);
    pub fn wxLEDNumberCtrl_SetValue(_obj: *u8 /* void* */, Value: *u8 /* void* */, Redraw: c_int /* int */);
    
    // TClassDef(wxMimeTypesManager)
    pub fn wxMimeTypesManager_AddFallbacks(_obj: *u8 /* void* */, _types: *u8 /* void* */);
    pub fn wxMimeTypesManager_Create() -> *u8 /* void* */;
    pub fn wxMimeTypesManager_EnumAllFileTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMimeTypesManager_GetFileTypeFromExtension(_obj: *u8 /* void* */, _ext: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMimeTypesManager_GetFileTypeFromMimeType(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMimeTypesManager_IsOfType(_obj: *u8 /* void* */, _type: *u8 /* void* */, _wildcard: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxCursor,wxBitmap)
    pub fn Cursor_CreateFromStock(_id: c_int /* int */) -> *u8 /* void* */;
    pub fn Cursor_CreateFromImage(image: *u8 /* void* */) -> *u8 /* void* */;
    pub fn Cursor_CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxRect)
    
    // TClassDef(wxClientDataContainer)
    
    // TClassDef(wxStopWatch)
    pub fn wxStopWatch_Create() -> *u8 /* void* */;
    pub fn wxStopWatch_Delete(_obj: *u8 /* void* */);
    pub fn wxStopWatch_Start(_obj: *u8 /* void* */, msec: c_int /* int */);
    pub fn wxStopWatch_Pause(_obj: *u8 /* void* */);
    pub fn wxStopWatch_Resume(_obj: *u8 /* void* */);
    pub fn wxStopWatch_Time(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxNotebook,wxControl)
    pub fn wxNotebook_AddPage(_obj: *u8 /* void* */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */;
    pub fn wxNotebook_AdvanceSelection(_obj: *u8 /* void* */, bForward: bool /* bool */);
    pub fn wxNotebook_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxNotebook_DeleteAllPages(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNotebook_DeletePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */;
    pub fn wxNotebook_GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxNotebook_GetPage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */;
    pub fn wxNotebook_GetPageCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxNotebook_GetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */;
    pub fn wxNotebook_GetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */;
    pub fn wxNotebook_GetRowCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxNotebook_GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxNotebook_HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_long /* long* */) -> c_int /* int */;
    pub fn wxNotebook_InsertPage(_obj: *u8 /* void* */, nPage: c_int /* int */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */;
    pub fn wxNotebook_RemovePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */;
    pub fn wxNotebook_SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    pub fn wxNotebook_SetPadding(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxNotebook_SetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */, nImage: c_int /* int */) -> bool /* bool */;
    pub fn wxNotebook_SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxNotebook_SetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */, strText: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNotebook_SetSelection(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */;
    pub fn expNB_TOP() -> c_int /* int */;
    pub fn expNB_BOTTOM() -> c_int /* int */;
    pub fn expNB_LEFT() -> c_int /* int */;
    pub fn expNB_RIGHT() -> c_int /* int */;
    pub fn expBK_HITTEST_NOWHERE() -> c_int /* int */;
    pub fn expBK_HITTEST_ONICON() -> c_int /* int */;
    pub fn expBK_HITTEST_ONLABEL() -> c_int /* int */;
    pub fn expBK_HITTEST_ONITEM() -> c_int /* int */;
    pub fn expBK_HITTEST_ONPAGE() -> c_int /* int */;
    
    // TClassDef(wxDCClipper)
    
    // TClassDefExtend(wxBitmapDataObject,wxDataObjectSimple)
    pub fn BitmapDataObject_Create(_bmp: *u8 /* void* */) -> *u8 /* void* */;
    pub fn BitmapDataObject_CreateEmpty() -> *u8 /* void* */;
    pub fn BitmapDataObject_Delete(_obj: *u8 /* void* */);
    pub fn BitmapDataObject_GetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    pub fn BitmapDataObject_SetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    
    // TClassDefExtend(cbCustomizeLayoutEvent,cbPluginEvent)
    pub fn cbCustomizeLayoutEvent_ClickPos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxPostScriptPrintNativeData,wxObject)
    pub fn wxPostScriptPrintNativeData_Create() -> *u8 /* void* */;
    pub fn wxPostScriptPrintNativeData_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJPreviewControlBar,wxPreviewControlBar)
    pub fn ELJPreviewControlBar_Create(preview: *u8 /* void* */, buttons: c_int /* int */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxFFile)
    
    // TClassDefExtend(wxPageSetupDialogData,wxObject)
    pub fn wxPageSetupDialogData_Assign(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxPageSetupDialogData_AssignData(_obj: *u8 /* void* */, printData: *u8 /* void* */);
    pub fn wxPageSetupDialogData_CalculateIdFromPaperSize(_obj: *u8 /* void* */);
    pub fn wxPageSetupDialogData_CalculatePaperSizeFromId(_obj: *u8 /* void* */);
    pub fn wxPageSetupDialogData_Create() -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_Delete(_obj: *u8 /* void* */);
    pub fn wxPageSetupDialogData_EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPageSetupDialogData_EnableMargins(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPageSetupDialogData_EnableOrientation(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPageSetupDialogData_EnablePaper(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPageSetupDialogData_EnablePrinter(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPageSetupDialogData_GetDefaultInfo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetDefaultMinMargins(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetEnableMargins(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetEnableOrientation(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetEnablePaper(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetEnablePrinter(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPageSetupDialogData_GetMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_GetMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_GetMinMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_GetMinMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPageSetupDialogData_GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialogData_GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPageSetupDialogData_SetDefaultInfo(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPageSetupDialogData_SetDefaultMinMargins(_obj: *u8 /* void* */, flag: c_int /* int */);
    pub fn wxPageSetupDialogData_SetMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPageSetupDialogData_SetMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPageSetupDialogData_SetMinMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPageSetupDialogData_SetMinMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPageSetupDialogData_SetPaperId(_obj: *u8 /* void* */, id: *u8 /* void* */);
    pub fn wxPageSetupDialogData_SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPageSetupDialogData_SetPaperSizeId(_obj: *u8 /* void* */, id: c_int /* int */);
    pub fn wxPageSetupDialogData_SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */);
    
    // TClassDef(wxCommandLineParser)
    
    // TClassDefExtend(wxLayoutConstraints,wxObject)
    pub fn wxLayoutConstraints_Create() -> *u8 /* void* */;
    pub fn wxLayoutConstraints_bottom(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_centreX(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_centreY(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_height(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_left(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_right(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_top(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLayoutConstraints_width(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDrawWindow,wxWindow)
    pub fn wxDrawWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxDataFormat)
    pub fn wxDataFormat_CreateFromId(name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDataFormat_CreateFromType(typ: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDataFormat_Delete(_obj: *u8 /* void* */);
    pub fn wxDataFormat_GetId(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDataFormat_GetType(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDataFormat_IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDataFormat_SetId(_obj: *u8 /* void* */, id: *u8 /* void* */);
    pub fn wxDataFormat_SetType(_obj: *u8 /* void* */, typ: c_int /* int */);
    
    // TClassDefExtend(cbDockBox,cbMiniButton)
    pub fn cbDockBox_Create() -> *u8 /* void* */;
    
    // TClassDefExtend(wxBitmap,wxGDIObject)
    pub fn wxBitmap_AddHandler(handler: *u8 /* void* */);
    pub fn wxBitmap_CleanUpHandlers();
    pub fn wxBitmap_Create(_data: *u8 /* void* */, _type: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmap_CreateDefault() -> *u8 /* void* */;
    pub fn wxBitmap_CreateEmpty(arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmap_CreateFromXPM(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBitmap_CreateLoad(name: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmap_Delete(_obj: *u8 /* void* */);
    pub fn wxBitmap_FindHandlerByExtension(extension: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmap_FindHandlerByName(name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBitmap_FindHandlerByType(type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmap_GetDepth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBitmap_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBitmap_GetMask(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBitmap_GetSubBitmap(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxBitmap_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBitmap_InitStandardHandlers();
    pub fn wxBitmap_InsertHandler(handler: *u8 /* void* */);
    pub fn wxBitmap_LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> c_int /* int */;
    pub fn wxBitmap_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBitmap_RemoveHandler(name: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBitmap_SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */, cmap: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBitmap_SetDepth(_obj: *u8 /* void* */, d: c_int /* int */);
    pub fn wxBitmap_SetHeight(_obj: *u8 /* void* */, h: c_int /* int */);
    pub fn wxBitmap_SetMask(_obj: *u8 /* void* */, mask: *u8 /* void* */);
    pub fn wxBitmap_SetWidth(_obj: *u8 /* void* */, w: c_int /* int */);
    
    // TClassDefExtend(wxBufferedOutputStream,wxFilterOutputStream)
    
    // TClassDefExtend(cbMotionEvent,cbPluginEvent)
    pub fn cbMotionEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(cbRowInfo,wxObject)
    pub fn cbRowInfo_Create() -> *u8 /* void* */;
    pub fn cbRowInfo_Delete(_obj: *u8 /* void* */);
    pub fn cbRowInfo_GetFirstBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellChoiceEditor,wxGridCellEditor)
    pub fn wxGridCellChoiceEditor_Ctor(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, allowOthers: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxScopedPtr)
    
    // TClassDefExtend(wxEraseEvent,wxEvent)
    pub fn wxEraseEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxEraseEvent_GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPanel,wxWindow)
    pub fn wxPanel_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPanel_InitDialog(_obj: *u8 /* void* */);
    pub fn wxPanel_SetFocus(_obj: *u8 /* void* */);
    
    // TClassDefExtend(cbFinishDrawInAreaEvent,cbPluginEvent)
    pub fn cbFinishDrawInAreaEvent_Area(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    
    // TClassDefExtend(wxPreviewFrame,wxFrame)
    
    // TClassDefExtend(wxPreviewFrame,wxFrame);
    pub fn wxPreviewFrame_Create(preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPreviewFrame_Delete(self_: *u8 /* void* */);
    pub fn wxPreviewFrame_Initialize(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxFocusEvent,wxEvent)
    
    // TClassDefExtend(cbCollapseBox,cbMiniButton)
    pub fn cbCollapseBox_Create() -> *u8 /* void* */;
    
    // TClassDefExtend(wxLogStream,wxLog)
    
    // TClassDefExtend(wxScrolledWindow,wxPanel)
    pub fn wxScrolledWindow_AdjustScrollbars(_obj: *u8 /* void* */);
    pub fn wxScrolledWindow_CalcScrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxScrolledWindow_CalcUnscrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxScrolledWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxScrolledWindow_EnableScrolling(_obj: *u8 /* void* */, x_scrolling: bool /* bool */, y_scrolling: bool /* bool */);
    pub fn wxScrolledWindow_GetScaleX(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxScrolledWindow_GetScaleY(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxScrolledWindow_GetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    pub fn wxScrolledWindow_GetScrollPixelsPerUnit(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxScrolledWindow_GetTargetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxScrolledWindow_GetViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxScrolledWindow_GetVirtualSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxScrolledWindow_OnDraw(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxScrolledWindow_PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxScrolledWindow_Scroll(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxScrolledWindow_SetScale(_obj: *u8 /* void* */, xs: c_double /* double */, ys: c_double /* double */);
    pub fn wxScrolledWindow_SetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */, pageSize: c_int /* int */);
    pub fn wxScrolledWindow_SetScrollbars(_obj: *u8 /* void* */, pixelsPerUnitX: c_int /* int */, pixelsPerUnitY: c_int /* int */, noUnitsX: c_int /* int */, noUnitsY: c_int /* int */, xPos: c_int /* int */, yPos: c_int /* int */, noRefresh: bool /* bool */);
    pub fn wxScrolledWindow_ShowScrollbars(_obj: *u8 /* void* */, showh: c_int /* int */, showv: c_int /* int */);
    pub fn wxScrolledWindow_SetTargetWindow(_obj: *u8 /* void* */, target: *u8 /* void* */);
    pub fn wxScrolledWindow_ViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxSockAddress,wxObject)
    
    // TClassDefExtend(wxMask,wxObject)
    pub fn wxMask_Create(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMask_CreateColoured(bitmap: *u8 /* void* */, colour: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxImageList,wxObject)
    pub fn wxImageList_AddBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> c_int /* int */;
    pub fn wxImageList_AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */;
    pub fn wxImageList_AddMasked(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, maskColour: *u8 /* void* */) -> c_int /* int */;
    pub fn wxImageList_Create(arg0: c_int /* int */, arg1: c_int /* int */, mask: c_int /* int */, initialCount: c_int /* int */) -> *u8 /* void* */;
    pub fn wxImageList_Delete(_obj: *u8 /* void* */);
    pub fn wxImageList_Draw(_obj: *u8 /* void* */, index: c_int /* int */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: c_int /* int */, solidBackground: bool /* bool */) -> bool /* bool */;
    pub fn wxImageList_GetImageCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxImageList_GetSize(_obj: *u8 /* void* */, index: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxImageList_Remove(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    pub fn wxImageList_RemoveAll(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImageList_Replace(_obj: *u8 /* void* */, index: c_int /* int */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImageList_ReplaceIcon(_obj: *u8 /* void* */, index: c_int /* int */, icon: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxGDIObject,wxObject)
    
    // TClassDef(wxClientData)
    
    // TClassDef(wxTextOutputStream)
    
    // TClassDef( wxTextOutputStream );
    pub fn wxTextOutputStream_Create(outputStream: *u8 /* void* */, mode: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTextOutputStream_Delete(self_: *u8 /* void* */);
    pub fn wxTextOutputStream_WriteString(self_: *u8 /* void* */, txt: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AddText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AddStyledText(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxStyledTextCtrl_InsertText(_obj: *u8 /* void* */, pos: c_int /* int */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_ClearAll(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_ClearDocumentStyle(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_GetLength(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetCharAt(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetCurrentPos(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetAnchor(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetStyleAt(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_Redo(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetUndoCollection(_obj: *u8 /* void* */, collectUndo: bool /* bool */);
    pub fn wxStyledTextCtrl_SelectAll(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetSavePoint(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_CanRedo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_MarkerLineFromHandle(_obj: *u8 /* void* */, handle: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_MarkerDeleteHandle(_obj: *u8 /* void* */, handle: c_int /* int */);
    pub fn wxStyledTextCtrl_GetUndoCollection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_GetViewWhiteSpace(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetViewWhiteSpace(_obj: *u8 /* void* */, viewWS: c_int /* int */);
    pub fn wxStyledTextCtrl_PositionFromPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_PositionFromPointClose(_obj: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GotoLine(_obj: *u8 /* void* */, line: c_int /* int */);
    pub fn wxStyledTextCtrl_GotoPos(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_SetAnchor(_obj: *u8 /* void* */, posAnchor: c_int /* int */);
    pub fn wxStyledTextCtrl_GetEndStyled(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_ConvertEOLs(_obj: *u8 /* void* */, eolMode: c_int /* int */);
    pub fn wxStyledTextCtrl_GetEOLMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetEOLMode(_obj: *u8 /* void* */, eolMode: c_int /* int */);
    pub fn wxStyledTextCtrl_StartStyling(_obj: *u8 /* void* */, pos: c_int /* int */, mask: c_int /* int */);
    pub fn wxStyledTextCtrl_SetStyling(_obj: *u8 /* void* */, length: c_int /* int */, style: c_int /* int */);
    pub fn wxStyledTextCtrl_GetBufferedDraw(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetBufferedDraw(_obj: *u8 /* void* */, buffered: bool /* bool */);
    pub fn wxStyledTextCtrl_SetTabWidth(_obj: *u8 /* void* */, tabWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_GetTabWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetCodePage(_obj: *u8 /* void* */, codePage: c_int /* int */);
    pub fn wxStyledTextCtrl_MarkerDefine(_obj: *u8 /* void* */, markerNumber: c_int /* int */, markerSymbol: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */, arg5: u8 /* u8 */);
    pub fn wxStyledTextCtrl_MarkerSetForeground(_obj: *u8 /* void* */, markerNumber: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_MarkerSetBackground(_obj: *u8 /* void* */, markerNumber: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_MarkerAdd(_obj: *u8 /* void* */, line: c_int /* int */, markerNumber: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_MarkerDelete(_obj: *u8 /* void* */, line: c_int /* int */, markerNumber: c_int /* int */);
    pub fn wxStyledTextCtrl_MarkerDeleteAll(_obj: *u8 /* void* */, markerNumber: c_int /* int */);
    pub fn wxStyledTextCtrl_MarkerGet(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_MarkerNext(_obj: *u8 /* void* */, lineStart: c_int /* int */, markerMask: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_MarkerPrevious(_obj: *u8 /* void* */, lineStart: c_int /* int */, markerMask: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_MarkerDefineBitmap(_obj: *u8 /* void* */, markerNumber: c_int /* int */, bmp: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetMarginType(_obj: *u8 /* void* */, margin: c_int /* int */, marginType: c_int /* int */);
    pub fn wxStyledTextCtrl_GetMarginType(_obj: *u8 /* void* */, margin: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetMarginWidth(_obj: *u8 /* void* */, margin: c_int /* int */, pixelWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_GetMarginWidth(_obj: *u8 /* void* */, margin: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetMarginMask(_obj: *u8 /* void* */, margin: c_int /* int */, mask: c_int /* int */);
    pub fn wxStyledTextCtrl_GetMarginMask(_obj: *u8 /* void* */, margin: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetMarginSensitive(_obj: *u8 /* void* */, margin: c_int /* int */, sensitive: bool /* bool */);
    pub fn wxStyledTextCtrl_GetMarginSensitive(_obj: *u8 /* void* */, margin: c_int /* int */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_StyleClearAll(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StyleSetForeground(_obj: *u8 /* void* */, style: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_StyleSetBackground(_obj: *u8 /* void* */, style: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_StyleSetBold(_obj: *u8 /* void* */, style: c_int /* int */, bold: bool /* bool */);
    pub fn wxStyledTextCtrl_StyleSetItalic(_obj: *u8 /* void* */, style: c_int /* int */, italic: bool /* bool */);
    pub fn wxStyledTextCtrl_StyleSetSize(_obj: *u8 /* void* */, style: c_int /* int */, sizePoints: c_int /* int */);
    pub fn wxStyledTextCtrl_StyleSetFaceName(_obj: *u8 /* void* */, style: c_int /* int */, fontName: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StyleSetEOLFilled(_obj: *u8 /* void* */, style: c_int /* int */, filled: bool /* bool */);
    pub fn wxStyledTextCtrl_StyleResetDefault(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StyleSetUnderline(_obj: *u8 /* void* */, style: c_int /* int */, underline: bool /* bool */);
    pub fn wxStyledTextCtrl_StyleSetCase(_obj: *u8 /* void* */, style: c_int /* int */, caseForce: c_int /* int */);
    pub fn wxStyledTextCtrl_StyleSetCharacterSet(_obj: *u8 /* void* */, style: c_int /* int */, characterSet: c_int /* int */);
    pub fn wxStyledTextCtrl_StyleSetHotSpot(_obj: *u8 /* void* */, style: c_int /* int */, hotspot: bool /* bool */);
    pub fn wxStyledTextCtrl_SetSelForeground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetSelBackground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetCaretForeground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_CmdKeyAssign(_obj: *u8 /* void* */, key: c_int /* int */, modifiers: c_int /* int */, cmd: c_int /* int */);
    pub fn wxStyledTextCtrl_CmdKeyClear(_obj: *u8 /* void* */, key: c_int /* int */, modifiers: c_int /* int */);
    pub fn wxStyledTextCtrl_CmdKeyClearAll(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetStyleBytes(_obj: *u8 /* void* */, length: c_int /* int */, styleBytes: *char /* char* */);
    pub fn wxStyledTextCtrl_StyleSetVisible(_obj: *u8 /* void* */, style: c_int /* int */, visible: bool /* bool */);
    pub fn wxStyledTextCtrl_GetCaretPeriod(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetCaretPeriod(_obj: *u8 /* void* */, periodMilliseconds: c_int /* int */);
    pub fn wxStyledTextCtrl_SetWordChars(_obj: *u8 /* void* */, characters: *u8 /* void* */);
    pub fn wxStyledTextCtrl_BeginUndoAction(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_EndUndoAction(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_IndicatorSetStyle(_obj: *u8 /* void* */, indic: c_int /* int */, style: c_int /* int */);
    pub fn wxStyledTextCtrl_IndicatorGetStyle(_obj: *u8 /* void* */, indic: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_IndicatorSetForeground(_obj: *u8 /* void* */, indic: c_int /* int */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetWhitespaceForeground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetWhitespaceBackground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetStyleBits(_obj: *u8 /* void* */, bits: c_int /* int */);
    pub fn wxStyledTextCtrl_GetStyleBits(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetLineState(_obj: *u8 /* void* */, line: c_int /* int */, state: c_int /* int */);
    pub fn wxStyledTextCtrl_GetLineState(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetMaxLineState(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetCaretLineVisible(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetCaretLineVisible(_obj: *u8 /* void* */, show: bool /* bool */);
    pub fn wxStyledTextCtrl_StyleSetChangeable(_obj: *u8 /* void* */, style: c_int /* int */, changeable: bool /* bool */);
    pub fn wxStyledTextCtrl_AutoCompShow(_obj: *u8 /* void* */, lenEntered: c_int /* int */, itemList: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompCancel(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompActive(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_AutoCompPosStart(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_AutoCompComplete(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompStops(_obj: *u8 /* void* */, characterSet: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompSetSeparator(_obj: *u8 /* void* */, separatorCharacter: c_int /* int */);
    pub fn wxStyledTextCtrl_AutoCompGetSeparator(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_AutoCompSelect(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompSetCancelAtStart(_obj: *u8 /* void* */, cancel: bool /* bool */);
    pub fn wxStyledTextCtrl_AutoCompGetCancelAtStart(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_AutoCompSetFillUps(_obj: *u8 /* void* */, characterSet: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompSetChooseSingle(_obj: *u8 /* void* */, chooseSingle: bool /* bool */);
    pub fn wxStyledTextCtrl_AutoCompGetChooseSingle(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_AutoCompSetIgnoreCase(_obj: *u8 /* void* */, ignoreCase: bool /* bool */);
    pub fn wxStyledTextCtrl_AutoCompGetIgnoreCase(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_UserListShow(_obj: *u8 /* void* */, listType: c_int /* int */, itemList: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompSetAutoHide(_obj: *u8 /* void* */, autoHide: bool /* bool */);
    pub fn wxStyledTextCtrl_AutoCompGetAutoHide(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_AutoCompSetDropRestOfWord(_obj: *u8 /* void* */, dropRestOfWord: bool /* bool */);
    pub fn wxStyledTextCtrl_AutoCompGetDropRestOfWord(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_RegisterImage(_obj: *u8 /* void* */, type_: c_int /* int */, bmp: *u8 /* void* */);
    pub fn wxStyledTextCtrl_ClearRegisteredImages(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_AutoCompGetTypeSeparator(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_AutoCompSetTypeSeparator(_obj: *u8 /* void* */, separatorCharacter: c_int /* int */);
    pub fn wxStyledTextCtrl_SetIndent(_obj: *u8 /* void* */, indentSize: c_int /* int */);
    pub fn wxStyledTextCtrl_GetIndent(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetUseTabs(_obj: *u8 /* void* */, useTabs: bool /* bool */);
    pub fn wxStyledTextCtrl_GetUseTabs(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetLineIndentation(_obj: *u8 /* void* */, line: c_int /* int */, indentSize: c_int /* int */);
    pub fn wxStyledTextCtrl_GetLineIndentation(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetLineIndentPosition(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetColumn(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetUseHorizontalScrollBar(_obj: *u8 /* void* */, show: bool /* bool */);
    pub fn wxStyledTextCtrl_GetUseHorizontalScrollBar(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetIndentationGuides(_obj: *u8 /* void* */, show: bool /* bool */);
    pub fn wxStyledTextCtrl_GetIndentationGuides(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetHighlightGuide(_obj: *u8 /* void* */, column: c_int /* int */);
    pub fn wxStyledTextCtrl_GetHighlightGuide(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetLineEndPosition(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetCodePage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetReadOnly(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetCurrentPos(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_SetSelectionStart(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_GetSelectionStart(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetSelectionEnd(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_GetSelectionEnd(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetPrintMagnification(_obj: *u8 /* void* */, magnification: c_int /* int */);
    pub fn wxStyledTextCtrl_GetPrintMagnification(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetPrintColourMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxStyledTextCtrl_GetPrintColourMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_FindText(_obj: *u8 /* void* */, minPos: c_int /* int */, maxPos: c_int /* int */, text: *u8 /* void* */, flags: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_FormatRange(_obj: *u8 /* void* */, doDraw: bool /* bool */, startPos: c_int /* int */, endPos: c_int /* int */, draw: *u8 /* void* */, target: *u8 /* void* */, renderRect: *u8 /* void* */, pageRect: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetFirstVisibleLine(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetLineCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetMarginLeft(_obj: *u8 /* void* */, pixelWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_GetMarginLeft(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetMarginRight(_obj: *u8 /* void* */, pixelWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_GetMarginRight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetModify(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetSelection(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */);
    pub fn wxStyledTextCtrl_HideSelection(_obj: *u8 /* void* */, normal: bool /* bool */);
    pub fn wxStyledTextCtrl_LineFromPosition(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_PositionFromLine(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_LineScroll(_obj: *u8 /* void* */, columns: c_int /* int */, lines: c_int /* int */);
    pub fn wxStyledTextCtrl_EnsureCaretVisible(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_ReplaceSelection(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetReadOnly(_obj: *u8 /* void* */, readOnly: bool /* bool */);
    pub fn wxStyledTextCtrl_CanPaste(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_CanUndo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_EmptyUndoBuffer(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_Undo(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_Cut(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_Copy(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_Paste(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_Clear(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_GetTextLength(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetOvertype(_obj: *u8 /* void* */, overtype: bool /* bool */);
    pub fn wxStyledTextCtrl_GetOvertype(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetCaretWidth(_obj: *u8 /* void* */, pixelWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_GetCaretWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetTargetStart(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_GetTargetStart(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetTargetEnd(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_GetTargetEnd(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_ReplaceTarget(_obj: *u8 /* void* */, text: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_ReplaceTargetRE(_obj: *u8 /* void* */, text: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SearchInTarget(_obj: *u8 /* void* */, text: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetSearchFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxStyledTextCtrl_GetSearchFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_CallTipShow(_obj: *u8 /* void* */, pos: c_int /* int */, definition: *u8 /* void* */);
    pub fn wxStyledTextCtrl_CallTipCancel(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_CallTipActive(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_CallTipPosAtStart(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_CallTipSetHighlight(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */);
    pub fn wxStyledTextCtrl_CallTipSetBackground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_CallTipSetForeground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_CallTipSetForegroundHighlight(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_VisibleFromDocLine(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_DocLineFromVisible(_obj: *u8 /* void* */, lineDisplay: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetFoldLevel(_obj: *u8 /* void* */, line: c_int /* int */, level: c_int /* int */);
    pub fn wxStyledTextCtrl_GetFoldLevel(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetLastChild(_obj: *u8 /* void* */, line: c_int /* int */, level: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetFoldParent(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_ShowLines(_obj: *u8 /* void* */, lineStart: c_int /* int */, lineEnd: c_int /* int */);
    pub fn wxStyledTextCtrl_HideLines(_obj: *u8 /* void* */, lineStart: c_int /* int */, lineEnd: c_int /* int */);
    pub fn wxStyledTextCtrl_GetLineVisible(_obj: *u8 /* void* */, line: c_int /* int */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetFoldExpanded(_obj: *u8 /* void* */, line: c_int /* int */, expanded: bool /* bool */);
    pub fn wxStyledTextCtrl_GetFoldExpanded(_obj: *u8 /* void* */, line: c_int /* int */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_ToggleFold(_obj: *u8 /* void* */, line: c_int /* int */);
    pub fn wxStyledTextCtrl_EnsureVisible(_obj: *u8 /* void* */, line: c_int /* int */);
    pub fn wxStyledTextCtrl_SetFoldFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxStyledTextCtrl_EnsureVisibleEnforcePolicy(_obj: *u8 /* void* */, line: c_int /* int */);
    pub fn wxStyledTextCtrl_SetTabIndents(_obj: *u8 /* void* */, tabIndents: bool /* bool */);
    pub fn wxStyledTextCtrl_GetTabIndents(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetBackSpaceUnIndents(_obj: *u8 /* void* */, bsUnIndents: bool /* bool */);
    pub fn wxStyledTextCtrl_GetBackSpaceUnIndents(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetMouseDwellTime(_obj: *u8 /* void* */, periodMilliseconds: c_int /* int */);
    pub fn wxStyledTextCtrl_GetMouseDwellTime(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_WordStartPosition(_obj: *u8 /* void* */, pos: c_int /* int */, onlyWordCharacters: bool /* bool */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_WordEndPosition(_obj: *u8 /* void* */, pos: c_int /* int */, onlyWordCharacters: bool /* bool */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetWrapMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxStyledTextCtrl_GetWrapMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetLayoutCache(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxStyledTextCtrl_GetLayoutCache(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetScrollWidth(_obj: *u8 /* void* */, pixelWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_GetScrollWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_TextWidth(_obj: *u8 /* void* */, style: c_int /* int */, text: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetEndAtLastLine(_obj: *u8 /* void* */, endAtLastLine: bool /* bool */);
    pub fn wxStyledTextCtrl_GetEndAtLastLine(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_TextHeight(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetUseVerticalScrollBar(_obj: *u8 /* void* */, show: bool /* bool */);
    pub fn wxStyledTextCtrl_GetUseVerticalScrollBar(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_AppendText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_GetTwoPhaseDraw(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetTwoPhaseDraw(_obj: *u8 /* void* */, twoPhase: bool /* bool */);
    pub fn wxStyledTextCtrl_TargetFromSelection(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_LinesJoin(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_LinesSplit(_obj: *u8 /* void* */, pixelWidth: c_int /* int */);
    pub fn wxStyledTextCtrl_SetFoldMarginColour(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetFoldMarginHiColour(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_LineDuplicate(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_HomeDisplay(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_HomeDisplayExtend(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_LineEndDisplay(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_LineEndDisplayExtend(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_LineCopy(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_MoveCaretInsideView(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_LineLength(_obj: *u8 /* void* */, line: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_BraceHighlight(_obj: *u8 /* void* */, pos1: c_int /* int */, pos2: c_int /* int */);
    pub fn wxStyledTextCtrl_BraceBadLight(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxStyledTextCtrl_BraceMatch(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_GetViewEOL(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetViewEOL(_obj: *u8 /* void* */, visible: bool /* bool */);
    pub fn wxStyledTextCtrl_SetDocPointer(_obj: *u8 /* void* */, docPointer: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetModEventMask(_obj: *u8 /* void* */, mask: c_int /* int */);
    pub fn wxStyledTextCtrl_GetEdgeColumn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetEdgeColumn(_obj: *u8 /* void* */, column: c_int /* int */);
    pub fn wxStyledTextCtrl_GetEdgeMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetEdgeMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxStyledTextCtrl_SetEdgeColour(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SearchAnchor(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SearchNext(_obj: *u8 /* void* */, flags: c_int /* int */, text: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SearchPrev(_obj: *u8 /* void* */, flags: c_int /* int */, text: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_LinesOnScreen(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_UsePopUp(_obj: *u8 /* void* */, allowPopUp: bool /* bool */);
    pub fn wxStyledTextCtrl_SelectionIsRectangle(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetZoom(_obj: *u8 /* void* */, zoom: c_int /* int */);
    pub fn wxStyledTextCtrl_GetZoom(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_AddRefDocument(_obj: *u8 /* void* */, docPointer: *u8 /* void* */);
    pub fn wxStyledTextCtrl_ReleaseDocument(_obj: *u8 /* void* */, docPointer: *u8 /* void* */);
    pub fn wxStyledTextCtrl_GetModEventMask(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetSTCFocus(_obj: *u8 /* void* */, focus: bool /* bool */);
    pub fn wxStyledTextCtrl_GetSTCFocus(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetStatus(_obj: *u8 /* void* */, statusCode: c_int /* int */);
    pub fn wxStyledTextCtrl_GetStatus(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetMouseDownCaptures(_obj: *u8 /* void* */, captures: bool /* bool */);
    pub fn wxStyledTextCtrl_GetMouseDownCaptures(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetSTCCursor(_obj: *u8 /* void* */, cursorType: c_int /* int */);
    pub fn wxStyledTextCtrl_GetSTCCursor(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetControlCharSymbol(_obj: *u8 /* void* */, symbol: c_int /* int */);
    pub fn wxStyledTextCtrl_GetControlCharSymbol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_WordPartLeft(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_WordPartLeftExtend(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_WordPartRight(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_WordPartRightExtend(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetVisiblePolicy(_obj: *u8 /* void* */, visiblePolicy: c_int /* int */, visibleSlop: c_int /* int */);
    pub fn wxStyledTextCtrl_DelLineLeft(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_DelLineRight(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetXOffset(_obj: *u8 /* void* */, newOffset: c_int /* int */);
    pub fn wxStyledTextCtrl_GetXOffset(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_ChooseCaretX(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetXCaretPolicy(_obj: *u8 /* void* */, caretPolicy: c_int /* int */, caretSlop: c_int /* int */);
    pub fn wxStyledTextCtrl_SetYCaretPolicy(_obj: *u8 /* void* */, caretPolicy: c_int /* int */, caretSlop: c_int /* int */);
    pub fn wxStyledTextCtrl_SetPrintWrapMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxStyledTextCtrl_GetPrintWrapMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_SetHotspotActiveForeground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetHotspotActiveBackground(_obj: *u8 /* void* */, useSetting: bool /* bool */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_SetHotspotActiveUnderline(_obj: *u8 /* void* */, underline: bool /* bool */);
    pub fn wxStyledTextCtrl_PositionBefore(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_PositionAfter(_obj: *u8 /* void* */, pos: c_int /* int */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_CopyRange(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */);
    pub fn wxStyledTextCtrl_CopyText(_obj: *u8 /* void* */, length: c_int /* int */, text: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StartRecord(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StopRecord(_obj: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetLexer(_obj: *u8 /* void* */, lexer: c_int /* int */);
    pub fn wxStyledTextCtrl_GetLexer(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_Colourise(_obj: *u8 /* void* */, start: c_int /* int */, end: c_int /* int */);
    pub fn wxStyledTextCtrl_SetProperty(_obj: *u8 /* void* */, key: *u8 /* void* */, value: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetKeyWords(_obj: *u8 /* void* */, keywordSet: c_int /* int */, keyWords: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetLexerLanguage(_obj: *u8 /* void* */, language: *u8 /* void* */);
    pub fn wxStyledTextCtrl_GetCurrentLine(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStyledTextCtrl_StyleSetSpec(_obj: *u8 /* void* */, styleNum: c_int /* int */, spec: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StyleSetFont(_obj: *u8 /* void* */, styleNum: c_int /* int */, font: *u8 /* void* */);
    pub fn wxStyledTextCtrl_StyleSetFontAttr(_obj: *u8 /* void* */, styleNum: c_int /* int */, size: c_int /* int */, faceName: *u8 /* void* */, bold: bool /* bool */, italic: bool /* bool */, underline: bool /* bool */);
    pub fn wxStyledTextCtrl_CmdKeyExecute(_obj: *u8 /* void* */, cmd: c_int /* int */);
    pub fn wxStyledTextCtrl_SetMargins(_obj: *u8 /* void* */, left: c_int /* int */, right: c_int /* int */);
    pub fn wxStyledTextCtrl_GetSelection(_obj: *u8 /* void* */, startPos: *c_int /* int* */, endPos: *c_int /* int* */);
    pub fn wxStyledTextCtrl_ScrollToLine(_obj: *u8 /* void* */, line: c_int /* int */);
    pub fn wxStyledTextCtrl_ScrollToColumn(_obj: *u8 /* void* */, column: c_int /* int */);
    pub fn wxStyledTextCtrl_SetVScrollBar(_obj: *u8 /* void* */, bar: *u8 /* void* */);
    pub fn wxStyledTextCtrl_SetHScrollBar(_obj: *u8 /* void* */, bar: *u8 /* void* */);
    pub fn wxStyledTextCtrl_GetLastKeydownProcessed(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_SetLastKeydownProcessed(_obj: *u8 /* void* */, val: bool /* bool */);
    pub fn wxStyledTextCtrl_SaveFile(_obj: *u8 /* void* */, filename: *u8 /* void* */) -> bool /* bool */;
    pub fn wxStyledTextCtrl_LoadFile(_obj: *u8 /* void* */, filename: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxDynamicSashWindow,wxWindow)
    pub fn wxDynamicSashWindow_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDynamicSashWindow_Delete(_obj: *u8 /* void* */);
    pub fn wxDynamicSashWindow_GetHScrollBar(_obj: *u8 /* void* */, child: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDynamicSashWindow_GetVScrollBar(_obj: *u8 /* void* */, child: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxTimerRunner)
    
    // TClassDefExtend(wxMDIChildFrame,wxFrame)
    pub fn wxMDIChildFrame_Activate(_obj: *u8 /* void* */);
    pub fn wxMDIChildFrame_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridEditorCreatedEvent,wxCommandEvent)
    pub fn wxGridEditorCreatedEvent_GetCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEditorCreatedEvent_GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridEditorCreatedEvent_GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEditorCreatedEvent_SetCol(_obj: *u8 /* void* */, col: c_int /* int */);
    pub fn wxGridEditorCreatedEvent_SetControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */);
    pub fn wxGridEditorCreatedEvent_SetRow(_obj: *u8 /* void* */, row: c_int /* int */);
    
    // TClassDefExtend(wxPostScriptDC,wxDC)
    pub fn wxPostScriptDC_Create(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPostScriptDC_Delete(self_: *u8 /* void* */);
    pub fn wxPostScriptDC_SetResolution(self_: *u8 /* void* */, ppi: c_int /* int */);
    pub fn wxPostScriptDC_GetResolution(self_: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend( wxcPrintEvent, wxEvent );
    
    // TClassDefExtend(wxColourDialog,wxDialog)
    pub fn wxColourDialog_Create(_prt: *u8 /* void* */, col: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxColourDialog_GetColourData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    
    // TClassDefExtend(wxSplitterWindow,wxWindow)
    pub fn wxSplitterWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSplitterWindow_GetBorderSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSplitterWindow_GetMinimumPaneSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSplitterWindow_GetSashPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSplitterWindow_GetSashSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSplitterWindow_GetSplitMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSplitterWindow_GetWindow1(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSplitterWindow_GetWindow2(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSplitterWindow_Initialize(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxSplitterWindow_IsSplit(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSplitterWindow_ReplaceWindow(_obj: *u8 /* void* */, winOld: *u8 /* void* */, winNew: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSplitterWindow_SetBorderSize(_obj: *u8 /* void* */, width: c_int /* int */);
    pub fn wxSplitterWindow_SetMinimumPaneSize(_obj: *u8 /* void* */, min: c_int /* int */);
    pub fn wxSplitterWindow_SetSashPosition(_obj: *u8 /* void* */, position: c_int /* int */, redraw: bool /* bool */);
    pub fn wxSplitterWindow_SetSashSize(_obj: *u8 /* void* */, width: c_int /* int */);
    pub fn wxSplitterWindow_SetSplitMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    pub fn wxSplitterWindow_SplitHorizontally(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */;
    pub fn wxSplitterWindow_SplitVertically(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */;
    pub fn wxSplitterWindow_Unsplit(_obj: *u8 /* void* */, toRemove: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSplitterWindow_GetSashGravity(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxSplitterWindow_SetSashGravity(_obj: *u8 /* void* */, gravity: c_double /* double */);
    
    // TClassDefExtend(wxWindowDC,wxDC)
    pub fn wxWindowDC_Create(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindowDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxRegion,wxGDIObject)
    pub fn wxRegion_Assign(_obj: *u8 /* void* */, region: *u8 /* void* */);
    pub fn wxRegion_Clear(_obj: *u8 /* void* */);
    pub fn wxRegion_ContainsPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    pub fn wxRegion_ContainsRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxRegion_CreateDefault() -> *u8 /* void* */;
    pub fn wxRegion_CreateFromRect(arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */;
    pub fn wxRegion_Delete(_obj: *u8 /* void* */);
    pub fn wxRegion_IsEmpty(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRegion_GetBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxRegion_IntersectRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxRegion_IntersectRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRegion_SubtractRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxRegion_SubtractRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRegion_UnionRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxRegion_UnionRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRegion_XorRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxRegion_XorRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxTablesInUse,wxObject)
    
    // TClassDefExtend(wxProtocol,wxSocketClient)
    
    // TClassDefExtend(wxMiniFrame,wxFrame)
    pub fn wxMiniFrame_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxGridCellCoordsArray)
    pub fn wxGridCellCoordsArray_Create() -> *u8 /* void* */;
    pub fn wxGridCellCoordsArray_Delete(_obj: *u8 /* void* */);
    pub fn wxGridCellCoordsArray_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridCellCoordsArray_Item(_obj: *u8 /* void* */, _idx: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxSocketEvent,wxEvent)
    
    // TClassDefExtend(wxGridCellTextEnterEditor,wxGridCellTextEditor)
    pub fn wxGridCellTextEnterEditor_Ctor() -> *u8 /* void* */;
    pub fn wxLogStderr_Create() -> *u8 /* void* */;
    pub fn wxLogStderr_CreateStdOut() -> *u8 /* void* */;
    pub fn wxLogNull_Create() -> *u8 /* void* */;
    pub fn wxLogTextCtrl_Create(text: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLogWindow_Create(parent: *u8 /* void* */, title: *wchar_t /* wchar_t* */, showit: bool /* bool */, passthrough: bool /* bool */) -> *u8 /* void* */;
    pub fn wxLogWindow_GetFrame(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn LogError(_msg: *u8 /* void* */);
    pub fn LogFatalError(_msg: *u8 /* void* */);
    pub fn LogWarning(_msg: *u8 /* void* */);
    pub fn LogMessage(_msg: *u8 /* void* */);
    pub fn LogVerbose(_msg: *u8 /* void* */);
    pub fn LogStatus(_msg: *u8 /* void* */);
    pub fn LogSysError(_msg: *u8 /* void* */);
    pub fn LogDebug(_msg: *u8 /* void* */);
    pub fn LogTrace(mask: *u8 /* void* */, _msg: *u8 /* void* */);
    pub fn wxLog_AddTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */);
    pub fn wxLog_Delete(_obj: *u8 /* void* */);
    pub fn wxLog_DontCreateOnDemand(_obj: *u8 /* void* */);
    pub fn wxLog_Flush(_obj: *u8 /* void* */);
    pub fn wxLog_FlushActive(_obj: *u8 /* void* */);
    pub fn wxLog_GetActiveTarget() -> *u8 /* void* */;
    pub fn wxLog_GetTimestamp(_obj: *u8 /* void* */) -> *char /* char* */;
    pub fn wxLog_GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxLog_GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxLog_HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLog_IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLog_OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *wchar_t /* wchar_t* */, t: c_int /* int */);
    pub fn wxLog_RemoveTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */);
    pub fn wxLog_Resume(_obj: *u8 /* void* */);
    pub fn wxLog_SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLog_SetTimestamp(_obj: *u8 /* void* */, ts: *wchar_t /* wchar_t* */);
    pub fn wxLog_SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */);
    pub fn wxLog_SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */);
    pub fn wxLog_Suspend(_obj: *u8 /* void* */);
    pub fn wxProcess_Open(cmd: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxProcess_IsErrorAvailable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxProcess_IsInputAvailable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxProcess_IsInputOpened(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxKill(pid: c_int /* int */, signal: c_int /* int */) -> c_int /* int */;
    pub fn wxStreamBase_Delete(obj: *u8 /* void* */);
    pub fn wxGetColourFromUser(parent: *u8 /* void* */, colInit: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxGetFontFromUser(parent: *u8 /* void* */, fontInit: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxGetPasswordFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxGetTextFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, center: bool /* bool */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxGetNumberFromUser(message: *u8 /* void* */, prompt: *u8 /* void* */, caption: *u8 /* void* */, value: c_long /* long */, min: c_long /* long */, max: c_long /* long */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_long /* long */;
    pub fn wxcBell();
    pub fn wxcBeginBusyCursor();
    pub fn wxcEndBusyCursor();
    pub fn wxcIsBusy();
    pub fn wxTextCtrl_EmulateKeyPress(_obj: *u8 /* void* */, keyevent: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_GetDefaultStyle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextCtrl_GetRange(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) -> *u8 /* void* */;
    pub fn wxTextCtrl_GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextCtrl_IsMultiLine(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_IsSingleLine(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_SetDefaultStyle(_obj: *u8 /* void* */, style: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextCtrl_SetMaxLength(_obj: *u8 /* void* */, len: c_long /* long */);
    pub fn wxTextCtrl_SetStyle(_obj: *u8 /* void* */, start: c_long /* long */, end: c_long /* long */, style: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextAttr_Create(colText: *u8 /* void* */, colBack: *u8 /* void* */, font: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTextAttr_CreateDefault() -> *u8 /* void* */;
    pub fn wxTextAttr_Delete(_obj: *u8 /* void* */);
    pub fn wxTextAttr_GetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxTextAttr_GetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxTextAttr_GetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxTextAttr_HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextAttr_HasFont(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextAttr_HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextAttr_IsDefault(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTextAttr_SetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxTextAttr_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxTextAttr_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    
    // TClassDef(wxDebugContext)
    
    // TClassDefExtend(wxLayoutAlgorithm,wxObject)
    pub fn wxLayoutAlgorithm_Create() -> *u8 /* void* */;
    pub fn wxLayoutAlgorithm_Delete(_obj: *u8 /* void* */);
    pub fn wxLayoutAlgorithm_LayoutFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLayoutAlgorithm_LayoutMDIFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, use_: c_int /* int */) -> bool /* bool */;
    pub fn wxLayoutAlgorithm_LayoutWindow(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxStringProperty,wxPGProperty)
    pub fn wxStringProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxObjectRefData)
    
    // TClassDefExtend(wxRadioBox,wxControl)
    pub fn wxRadioBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _dim: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxRadioBox_EnableItem(_obj: *u8 /* void* */, item: c_int /* int */, enable: bool /* bool */);
    pub fn wxRadioBox_FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRadioBox_GetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    pub fn wxRadioBox_GetNumberOfRowsOrCols(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRadioBox_GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRadioBox_GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxRadioBox_Number(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxRadioBox_SetItemBitmap(_obj: *u8 /* void* */, item: c_int /* int */, bitmap: *u8 /* void* */);
    pub fn wxRadioBox_SetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */, label: *u8 /* void* */);
    pub fn wxRadioBox_SetNumberOfRowsOrCols(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxRadioBox_SetSelection(_obj: *u8 /* void* */, _n: c_int /* int */);
    pub fn wxRadioBox_SetStringSelection(_obj: *u8 /* void* */, s: *u8 /* void* */);
    pub fn wxRadioBox_ShowItem(_obj: *u8 /* void* */, item: c_int /* int */, show: bool /* bool */);
    
    // TClassDefExtend(wxArrayString,wxArray)
    
    // TClassDefExtend(wxTopLevelWindow,wxWindow)
    pub fn wxTopLevelWindow_EnableCloseButton(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    pub fn wxTopLevelWindow_GetDefaultButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTopLevelWindow_GetDefaultItem(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTopLevelWindow_GetIcon(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTopLevelWindow_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTopLevelWindow_Iconize(_obj: *u8 /* void* */, iconize: bool /* bool */) -> bool /* bool */;
    pub fn wxTopLevelWindow_IsActive(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTopLevelWindow_IsIconized(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTopLevelWindow_IsMaximized(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTopLevelWindow_Maximize(_obj: *u8 /* void* */, maximize: bool /* bool */);
    pub fn wxTopLevelWindow_RequestUserAttention(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxTopLevelWindow_SetDefaultButton(_obj: *u8 /* void* */, pBut: *u8 /* void* */);
    pub fn wxTopLevelWindow_SetDefaultItem(_obj: *u8 /* void* */, pBut: *u8 /* void* */);
    pub fn wxTopLevelWindow_SetIcon(_obj: *u8 /* void* */, pIcon: *u8 /* void* */);
    pub fn wxTopLevelWindow_SetIcons(_obj: *u8 /* void* */, _icons: *u8 /* void* */);
    pub fn wxTopLevelWindow_SetMaxSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxTopLevelWindow_SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxTopLevelWindow_SetTitle(_obj: *u8 /* void* */, pString: *u8 /* void* */);
    
    // TClassDefExtend(ELJConnection,wxConnection)
    pub fn ELJConnection_Advise(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */, size: c_int /* int */, format: c_int /* int */) -> c_int /* int */;
    pub fn ELJConnection_Compress(_obj: *u8 /* void* */, on: c_int /* int */);
    pub fn ELJConnection_Create(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) -> *u8 /* void* */;
    pub fn ELJConnection_CreateDefault(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJConnection_Delete(_obj: *u8 /* void* */);
    pub fn ELJConnection_Disconnect(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn ELJConnection_Execute(_obj: *u8 /* void* */, data: *u8 /* void* */, size: c_int /* int */, format: c_int /* int */) -> bool /* bool */;
    pub fn ELJConnection_Poke(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */, size: c_int /* int */, format: c_int /* int */) -> bool /* bool */;
    pub fn ELJConnection_Request(_obj: *u8 /* void* */, item: *u8 /* void* */, size: *u8 /* void* */, format: c_int /* int */) -> *u8 /* void* */;
    pub fn ELJConnection_SetOnAdvise(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_SetOnDisconnect(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_SetOnExecute(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_SetOnPoke(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_SetOnRequest(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_SetOnStartAdvise(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_SetOnStopAdvise(_obj: *u8 /* void* */, _fnc: *u8 /* void* */);
    pub fn ELJConnection_StartAdvise(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn ELJConnection_StopAdvise(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxProcess,wxEvtHandler)
    pub fn wxProcess_CloseOutput(_obj: *u8 /* void* */);
    pub fn wxProcess_CreateDefault(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxProcess_CreateRedirect(_prt: *u8 /* void* */, _rdr: bool /* bool */) -> *u8 /* void* */;
    pub fn wxProcess_Delete(_obj: *u8 /* void* */);
    pub fn wxProcess_Detach(_obj: *u8 /* void* */);
    pub fn wxProcess_GetErrorStream(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxProcess_GetInputStream(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxProcess_GetOutputStream(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxProcess_IsRedirected(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxProcess_Redirect(_obj: *u8 /* void* */);
    
    // TClassDef(wxDbConnectInf)
    
    // TClassDefExtend(wxHtmlWinTagHandler,wxHtmlTagHandler)
    
    // TClassDefExtend(wxVariantData,wxObject)
    
    // TClassDefExtend(wxServerBase,wxObject)
    
    // TClassDef(wxDbTable)
    
    // TClassDef(wxDbInf)
    
    // TClassDefExtend(cbRowLayoutPlugin,cbPluginBase)
    pub fn cbRowLayoutPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbRowLayoutPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbRowLayoutPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(cbResizeRowEvent,cbPluginEvent)
    pub fn cbResizeRowEvent_ForUpperHandle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbResizeRowEvent_HandleOfs(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbResizeRowEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxInputStream,wxStreamBase)
    pub fn wxInputStream_Delete(_obj: *u8 /* void* */);
    pub fn wxInputStream_Eof(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxInputStream_GetC(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    pub fn wxInputStream_LastRead(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputStream_Peek(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    pub fn wxInputStream_Read(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */);
    pub fn wxInputStream_SeekI(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */;
    pub fn wxInputStream_Tell(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputStream_UngetBuffer(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) -> c_int /* int */;
    pub fn wxInputStream_Ungetch(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */) -> c_int /* int */;
    
    // TClassDefExtend(wxLogGUI,wxLog)
    
    // TClassDefExtend(wxSlider,wxControl)
    pub fn wxSlider_ClearSel(_obj: *u8 /* void* */);
    pub fn wxSlider_ClearTicks(_obj: *u8 /* void* */);
    pub fn wxSlider_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _init: c_int /* int */, _min: c_int /* int */, _max: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    pub fn wxSlider_GetLineSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetSelEnd(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetSelStart(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetThumbLength(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetTickFreq(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSlider_SetLineSize(_obj: *u8 /* void* */, lineSize: c_int /* int */);
    pub fn wxSlider_SetPageSize(_obj: *u8 /* void* */, pageSize: c_int /* int */);
    pub fn wxSlider_SetRange(_obj: *u8 /* void* */, minValue: c_int /* int */, maxValue: c_int /* int */);
    pub fn wxSlider_SetSelection(_obj: *u8 /* void* */, minPos: c_int /* int */, maxPos: c_int /* int */);
    pub fn wxSlider_SetThumbLength(_obj: *u8 /* void* */, len: c_int /* int */);
    pub fn wxSlider_SetTick(_obj: *u8 /* void* */, tickPos: c_int /* int */);
    pub fn wxSlider_SetTickFreq(_obj: *u8 /* void* */, n: c_int /* int */, pos: c_int /* int */);
    pub fn wxSlider_SetValue(_obj: *u8 /* void* */, value: c_int /* int */);
    
    // TClassDef(wxAcceleratorTable)
    pub fn wxAcceleratorTable_Create(n: c_int /* int */, entries: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxAcceleratorTable_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxStringList,wxList)
    
    // TClassDefExtend(wxCalendarEvent,wxCommandEvent)
    pub fn wxCalendarEvent_GetDate(_obj: *u8 /* void* */, _dte: *u8 /* void* */);
    pub fn wxCalendarEvent_GetWeekDay(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxPlotOnOffCurve,wxObject)
    pub fn wxPlotOnOffCurve_Add(_obj: *u8 /* void* */, on: c_int /* int */, off: c_int /* int */, clientData: *u8 /* void* */);
    pub fn wxPlotOnOffCurve_Create(offsetY: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPlotOnOffCurve_Delete(_obj: *u8 /* void* */);
    pub fn wxPlotOnOffCurve_DrawOffLine(_obj: *u8 /* void* */, dc: *u8 /* void* */, y: c_int /* int */, start: c_int /* int */, end: c_int /* int */);
    pub fn wxPlotOnOffCurve_DrawOnLine(_obj: *u8 /* void* */, dc: *u8 /* void* */, y: c_int /* int */, start: c_int /* int */, end: c_int /* int */, clientData: *u8 /* void* */);
    pub fn wxPlotOnOffCurve_GetAt(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPlotOnOffCurve_GetClientData(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPlotOnOffCurve_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotOnOffCurve_GetEndX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotOnOffCurve_GetOff(_obj: *u8 /* void* */, index: c_int /* int */) -> c_int /* int */;
    pub fn wxPlotOnOffCurve_GetOffsetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotOnOffCurve_GetOn(_obj: *u8 /* void* */, index: c_int /* int */) -> c_int /* int */;
    pub fn wxPlotOnOffCurve_GetStartX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPlotOnOffCurve_SetOffsetY(_obj: *u8 /* void* */, offsetY: c_int /* int */);
    
    // TClassDefExtend(wxStaticText,wxControl)
    pub fn wxStaticText_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxClient,wxClientBase)
    
    // TClassDefExtend(wxSplashScreen,wxFrame)
    
    // TClassDefExtend(wxHtmlHelpController,wxHelpControllerBase)
    pub fn wxHtmlHelpController_AddBook(_obj: *u8 /* void* */, book: *u8 /* void* */, show_wait_msg: c_int /* int */) -> bool /* bool */;
    pub fn wxHtmlHelpController_Create(_style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxHtmlHelpController_Delete(_obj: *u8 /* void* */);
    pub fn wxHtmlHelpController_Display(_obj: *u8 /* void* */, x: *u8 /* void* */) -> c_int /* int */;
    pub fn wxHtmlHelpController_DisplayBlock(_obj: *u8 /* void* */, blockNo: c_int /* int */) -> bool /* bool */;
    pub fn wxHtmlHelpController_DisplayContents(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxHtmlHelpController_DisplayIndex(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxHtmlHelpController_DisplayNumber(_obj: *u8 /* void* */, id: c_int /* int */) -> c_int /* int */;
    pub fn wxHtmlHelpController_DisplaySection(_obj: *u8 /* void* */, section: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlHelpController_DisplaySectionNumber(_obj: *u8 /* void* */, sectionNo: c_int /* int */) -> bool /* bool */;
    pub fn wxHtmlHelpController_GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlHelpController_GetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, width: *c_int /* int* */, height: *c_int /* int* */, pos_x: *c_int /* int* */, pos_y: *c_int /* int* */, newFrameEachTime: *c_int /* int* */) -> *u8 /* void* */;
    pub fn wxHtmlHelpController_Initialize(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlHelpController_KeywordSearch(_obj: *u8 /* void* */, keyword: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlHelpController_LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlHelpController_Quit(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlHelpController_ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxHtmlHelpController_SetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, pos_x: c_int /* int */, pos_y: c_int /* int */, newFrameEachTime: bool /* bool */);
    pub fn wxHtmlHelpController_SetTempDir(_obj: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxHtmlHelpController_SetTitleFormat(_obj: *u8 /* void* */, format: *u8 /* void* */);
    pub fn wxHtmlHelpController_SetViewer(_obj: *u8 /* void* */, viewer: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxHtmlHelpController_UseConfig(_obj: *u8 /* void* */, config: *u8 /* void* */, rootpath: *u8 /* void* */);
    pub fn wxHtmlHelpController_WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
    
    // TClassDefExtend(wxSocketBase,wxObject)
    
    // TClassDefExtend(wxRemotelyScrolledTreeCtrl,wxTreeCtrl)
    pub fn wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars(_obj: *u8 /* void* */);
    pub fn wxRemotelyScrolledTreeCtrl_CalcTreeSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem(_obj: *u8 /* void* */, id: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxRemotelyScrolledTreeCtrl_Create(_obj: *u8 /* void* */, _cmp: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxRemotelyScrolledTreeCtrl_Delete(_obj: *u8 /* void* */);
    pub fn wxRemotelyScrolledTreeCtrl_GetCompanionWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxRemotelyScrolledTreeCtrl_GetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    pub fn wxRemotelyScrolledTreeCtrl_GetScrolledWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxRemotelyScrolledTreeCtrl_GetViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxRemotelyScrolledTreeCtrl_HideVScrollbar(_obj: *u8 /* void* */);
    pub fn wxRemotelyScrolledTreeCtrl_PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxRemotelyScrolledTreeCtrl_ScrollToLine(_obj: *u8 /* void* */, posHoriz: c_int /* int */, posVert: c_int /* int */);
    pub fn wxRemotelyScrolledTreeCtrl_SetCompanionWindow(_obj: *u8 /* void* */, companion: *u8 /* void* */);
    pub fn wxRemotelyScrolledTreeCtrl_SetScrollbars(_obj: *u8 /* void* */, pixelsPerUnitX: c_int /* int */, pixelsPerUnitY: c_int /* int */, noUnitsX: c_int /* int */, noUnitsY: c_int /* int */, xPos: c_int /* int */, yPos: c_int /* int */, noRefresh: c_int /* int */);
    
    // TClassDefExtend(ELJTextDropTarget,wxTextDropTarget)
    pub fn ELJTextDropTarget_Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJTextDropTarget_Delete(_obj: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    
    // TClassDefExtend(wxAutoBufferedPaintDC,wxDC)
    pub fn wxAutoBufferedPaintDC_Create(window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxAutoBufferedPaintDC_Delete(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxFontData,wxObject)
    pub fn wxFontData_Create() -> *u8 /* void* */;
    pub fn wxFontData_Delete(_obj: *u8 /* void* */);
    pub fn wxFontData_EnableEffects(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxFontData_GetAllowSymbols(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFontData_GetChosenFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */);
    pub fn wxFontData_GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxFontData_GetEnableEffects(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFontData_GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFontData_GetInitialFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */);
    pub fn wxFontData_GetShowHelp(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFontData_SetAllowSymbols(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxFontData_SetChosenFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxFontData_SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxFontData_SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */);
    pub fn wxFontData_SetInitialFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxFontData_SetRange(_obj: *u8 /* void* */, minRange: c_int /* int */, maxRange: c_int /* int */);
    pub fn wxFontData_SetShowHelp(_obj: *u8 /* void* */, flag: bool /* bool */);
    
    // TClassDefExtend(wxButton,wxControl)
    pub fn wxButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxButton_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    pub fn wxButton_SetDefault(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxGauge,wxControl)
    pub fn wxGauge_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _rng: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGauge_GetBezelFace(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGauge_GetRange(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGauge_GetShadowWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGauge_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGauge_SetBezelFace(_obj: *u8 /* void* */, w: c_int /* int */);
    pub fn wxGauge_SetRange(_obj: *u8 /* void* */, r: c_int /* int */);
    pub fn wxGauge_SetShadowWidth(_obj: *u8 /* void* */, w: c_int /* int */);
    pub fn wxGauge_SetValue(_obj: *u8 /* void* */, pos: c_int /* int */);
    
    // TClassDefExtend(wxBrush,wxGDIObject)
    pub fn wxBrush_Assign(_obj: *u8 /* void* */, brush: *u8 /* void* */);
    pub fn wxBrush_CreateDefault() -> *u8 /* void* */;
    pub fn wxBrush_CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBrush_CreateFromColour(col: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBrush_CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBrush_Delete(_obj: *u8 /* void* */);
    pub fn wxBrush_GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxBrush_GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxBrush_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBrush_IsEqual(_obj: *u8 /* void* */, brush: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBrush_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBrush_SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxBrush_SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */);
    pub fn wxBrush_SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */);
    pub fn wxBrush_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    
    // TClassDefExtend(wxTreeCtrl,wxControl)
    pub fn wxTreeCtrl_AddRoot(_obj: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_AppendItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_Collapse(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_CollapseAndReset(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_Create(_obj: *u8 /* void* */, _cmp: *u8 /* void* */, _prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_Delete(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_DeleteAllItems(_obj: *u8 /* void* */);
    pub fn wxTreeCtrl_DeleteChildren(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_EditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_EndEditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */, discardChanges: bool /* bool */);
    pub fn wxTreeCtrl_EnsureVisible(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_Expand(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetBoundingRect(_obj: *u8 /* void* */, item: *u8 /* void* */, textOnly: bool /* bool */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_GetChildrenCount(_obj: *u8 /* void* */, item: *u8 /* void* */, recursively: bool /* bool */) -> c_int /* int */;
    pub fn wxTreeCtrl_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeCtrl_GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_GetFirstChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetFirstVisibleItem(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_GetIndent(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeCtrl_GetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_GetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, which: c_int /* int */) -> c_int /* int */;
    pub fn wxTreeCtrl_GetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_GetLastChild(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetNextChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetNextSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetNextVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetParent(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetPrevSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetPrevVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetRootItem(_obj: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetSelection(_obj: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetSelections(_obj: *u8 /* void* */, selections: *intptr_t /* intptr_t* */) -> c_int /* int */;
    pub fn wxTreeCtrl_GetSpacing(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeCtrl_GetStateImageList(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_int /* int* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_InsertItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_InsertItemByIndex(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_IsBold(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTreeCtrl_IsExpanded(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTreeCtrl_IsSelected(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTreeCtrl_IsVisible(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTreeCtrl_ItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeCtrl_OnCompareItems(_obj: *u8 /* void* */, item1: *u8 /* void* */, item2: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeCtrl_PrependItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_ScrollTo(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_SelectItem(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    pub fn wxTreeCtrl_SetIndent(_obj: *u8 /* void* */, indent: c_int /* int */);
    pub fn wxTreeCtrl_SetItemBackgroundColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxTreeCtrl_SetItemBold(_obj: *u8 /* void* */, item: *u8 /* void* */, bold: bool /* bool */);
    pub fn wxTreeCtrl_SetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxTreeCtrl_SetItemDropHighlight(_obj: *u8 /* void* */, item: *u8 /* void* */, highlight: bool /* bool */);
    pub fn wxTreeCtrl_SetItemFont(_obj: *u8 /* void* */, item: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxTreeCtrl_SetItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */, hasChildren: bool /* bool */);
    pub fn wxTreeCtrl_SetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, image: c_int /* int */, which: c_int /* int */);
    pub fn wxTreeCtrl_SetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxTreeCtrl_SetItemTextColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxTreeCtrl_SetSpacing(_obj: *u8 /* void* */, spacing: c_int /* int */);
    pub fn wxTreeCtrl_SetStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    pub fn wxTreeCtrl_SortChildren(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_Toggle(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxTreeCtrl_Unselect(_obj: *u8 /* void* */);
    pub fn wxTreeCtrl_UnselectAll(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxNotifyEvent,wxCommandEvent)
    pub fn wxNotifyEvent_Allow(_obj: *u8 /* void* */);
    pub fn wxNotifyEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxNotifyEvent_IsAllowed(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNotifyEvent_Veto(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxQuantize,wxObject)
    
    // TClassDefExtend(cbMiniButton,wxObject)
    pub fn cbMiniButton_Create() -> *u8 /* void* */;
    pub fn cbMiniButton_Delete(_obj: *u8 /* void* */);
    pub fn cbMiniButton_Dim(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn cbMiniButton_DragStarted(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbMiniButton_Enable(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn cbMiniButton_Enabled(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbMiniButton_HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    pub fn cbMiniButton_IsPressed(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn cbMiniButton_Layout(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbMiniButton_Pane(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbMiniButton_Plugin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbMiniButton_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn cbMiniButton_Pressed(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbMiniButton_Refresh(_obj: *u8 /* void* */);
    pub fn cbMiniButton_Reset(_obj: *u8 /* void* */);
    pub fn cbMiniButton_SetPos(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn cbMiniButton_Visible(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbMiniButton_WasClicked(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbMiniButton_Wnd(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPopupWindow,wxWindow)
    
    // TClassDefExtend(wxFileOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxFileConfig,wxConfigBase)
    pub fn wxConfigBase_Get() -> *u8 /* void* */;
    pub fn wxConfigBase_Set(self_: *u8 /* void* */);
    pub fn wxFileConfig_Create(inp: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBitmap_CreateFromImage(image: *u8 /* void* */, depth: c_int /* int */) -> *u8 /* void* */;
    pub fn wxImage_CreateFromDataEx(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, isStaticData: c_int /* int */) -> *u8 /* void* */;
    pub fn wxImage_Delete(image: *u8 /* void* */);
    pub fn wxColour_CreateFromInt(rgb: c_int /* int */) -> *u8 /* void* */;
    pub fn wxColour_GetInt(colour: *u8 /* void* */) -> c_int /* int */;
    pub fn wxColour_CreateFromUnsignedInt(rgba: uint32_t /* uint32_t */) -> *u8 /* void* */;
    pub fn wxColour_GetUnsignedInt(colour: *u8 /* void* */) -> uint32_t /* uint32_t */;
    pub fn wxcSystemSettingsGetColour(systemColour: c_int /* int */) -> *u8 /* void* */;
    pub fn wxcSetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgb: c_int /* int */);
    pub fn wxcGetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    pub fn wxcSetPixelRowRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbStart: c_int /* int */, rgbEnd: c_int /* int */, count: c_int /* int */);
    pub fn wxcInitPixelsRGB(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: c_int /* int */);
    pub fn wxcSetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */);
    pub fn wxcGetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> uint32_t /* uint32_t */;
    pub fn wxcSetPixelRowRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbaStart: c_int /* int */, rgbEnd: c_int /* int */, count: uint32_t /* uint32_t */);
    pub fn wxcInitPixelsRGBA(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */);
    pub fn wxcMalloc(size: c_int /* int */) -> *u8 /* void* */;
    pub fn wxcFree(p: *u8 /* void* */);
    pub fn wxcWakeUpIdle();
    pub fn wxGetApplicationDir() -> *u8 /* void* */;
    pub fn wxGetApplicationPath() -> *u8 /* void* */;
    pub fn ELJApp_InitializeC(closure: *u8 /* void* */, _argc: c_int /* int */, _argv: *wchar_t /* wchar_t* */);
    pub fn ELJApp_GetIdleInterval() -> c_int /* int */;
    pub fn ELJApp_SetIdleInterval(interval: c_int /* int */);
    
    // TClassDefExtend(wxExprDatabase,wxList)
    
    // TClassDefExtend(wxHelpEvent,wxCommandEvent)
    pub fn wxHelpEvent_GetLink(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpEvent_GetTarget(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpEvent_SetLink(_obj: *u8 /* void* */, link: *u8 /* void* */);
    pub fn wxHelpEvent_SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxHelpEvent_SetTarget(_obj: *u8 /* void* */, target: *u8 /* void* */);
    
    // TClassDefExtend(cbRightUpEvent,cbPluginEvent)
    pub fn cbRightUpEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxToolBar,wxToolBarBase)
    pub fn wxToolBar_AddControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */) -> bool /* bool */;
    pub fn wxToolBar_AddSeparator(_obj: *u8 /* void* */);
    pub fn wxToolBar_AddTool(_obj: *u8 /* void* */, id: c_int /* int */, bmp: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */);
    pub fn wxToolBar_AddToolEx(_obj: *u8 /* void* */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */);
    pub fn wxToolBar_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxToolBar_Delete(_obj: *u8 /* void* */);
    pub fn wxToolBar_DeleteTool(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxToolBar_DeleteToolByPos(_obj: *u8 /* void* */, pos: c_int /* int */) -> bool /* bool */;
    pub fn wxToolBar_EnableTool(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */);
    pub fn wxToolBar_GetMargins(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxToolBar_GetToolBitmapSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxToolBar_GetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxToolBar_GetToolEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxToolBar_GetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxToolBar_GetToolPacking(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxToolBar_GetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxToolBar_GetToolSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxToolBar_GetToolState(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxToolBar_InsertControl(_obj: *u8 /* void* */, pos: c_int /* int */, ctrl: *u8 /* void* */);
    pub fn wxToolBar_InsertSeparator(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxToolBar_InsertTool(_obj: *u8 /* void* */, pos: c_int /* int */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */);
    pub fn wxToolBar_Realize(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxToolBar_RemoveTool(_obj: *u8 /* void* */, id: c_int /* int */);
    pub fn wxToolBar_SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxToolBar_SetToolBitmapSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxToolBar_SetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */, data: *u8 /* void* */);
    pub fn wxToolBar_SetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */);
    pub fn wxToolBar_SetToolPacking(_obj: *u8 /* void* */, packing: c_int /* int */);
    pub fn wxToolBar_SetToolSeparation(_obj: *u8 /* void* */, separation: c_int /* int */);
    pub fn wxToolBar_SetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */);
    pub fn wxToolBar_ToggleTool(_obj: *u8 /* void* */, id: c_int /* int */, toggle: bool /* bool */);
    
    // TClassDefExtend(wxIdleEvent,wxEvent)
    pub fn wxIdleEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxIdleEvent_MoreRequested(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIdleEvent_RequestMore(_obj: *u8 /* void* */, needMore: bool /* bool */);
    
    // TClassDefExtend(wxProgressDialog,wxFrame)
    
    // TClassDefExtend(wxGridCellRenderer,wxGridCellWorker)
    
    // TClassDefExtend(wxHtmlEasyPrinting,wxObject)
    
    // TClassDefExtend(wxFloatProperty,wxPGProperty)
    pub fn wxFloatProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: float /* float */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxContextHelp,wxObject)
    pub fn wxContextHelp_BeginContextHelp(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    pub fn wxContextHelp_Create(win: *u8 /* void* */, beginHelp: bool /* bool */) -> *u8 /* void* */;
    pub fn wxContextHelp_Delete(_obj: *u8 /* void* */);
    pub fn wxContextHelp_EndContextHelp(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxPoint)
    pub fn wxPoint_Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPoint_Destroy(_obj: *u8 /* void* */);
    pub fn wxPoint_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPoint_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPoint_SetX(_obj: *u8 /* void* */, w: c_int /* int */);
    pub fn wxPoint_SetY(_obj: *u8 /* void* */, h: c_int /* int */);
    
    // TClassDef(wxRegEx)
    
    // TClassDefExtend(wxDropFilesEvent,wxEvent)
    
    // TClassDefExtend(wxSocketOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxSpinCtrl,wxControl)
    pub fn wxSpinCtrl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */, _min: c_int /* int */, _max: c_int /* int */, _init: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSpinCtrl_GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinCtrl_GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinCtrl_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinCtrl_SetRange(_obj: *u8 /* void* */, min_val: c_int /* int */, max_val: c_int /* int */);
    pub fn wxSpinCtrl_SetValue(_obj: *u8 /* void* */, val: c_int /* int */);
    
    // TClassDefExtend(wxGenericDragImage,wxDragImage);
    pub fn wxDragImage_Create(image: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDragIcon(icon: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDragString(test: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDragTreeItem(treeCtrl: *u8 /* void* */, id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDragListItem(treeCtrl: *u8 /* void* */, id: c_long /* long */) -> *u8 /* void* */;
    pub fn wxGenericDragImage_Create(cursor: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGenericDragIcon(icon: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGenericDragString(test: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGenericDragTreeItem(treeCtrl: *u8 /* void* */, id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGenericDragListItem(treeCtrl: *u8 /* void* */, id: c_long /* long */) -> *u8 /* void* */;
    pub fn wxDragImage_Delete(self_: *u8 /* void* */);
    pub fn wxDragImage_BeginDragFullScreen(self_: *u8 /* void* */, x_pos: c_int /* int */, y_pos: c_int /* int */, window: *u8 /* void* */, fullScreen: bool /* bool */, rect: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDragImage_BeginDrag(self_: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */, window: *u8 /* void* */, boundingWindow: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGenericDragImage_DoDrawImage(self_: *u8 /* void* */, dc: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> bool /* bool */;
    pub fn wxDragImage_EndDrag(self_: *u8 /* void* */);
    pub fn wxGenericDragImage_GetImageRect(self_: *u8 /* void* */, x_pos: c_int /* int */, y_pos: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDragImage_Hide(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDragImage_Move(self_: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */) -> bool /* bool */;
    pub fn wxDragImage_Show(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGenericDragImage_UpdateBackingFromWindow(self_: *u8 /* void* */, windowDC: *u8 /* void* */, destDC: *u8 /* void* */, x: c_int /* int */, y: c_int /* int */, w: c_int /* int */, h: c_int /* int */, xdest: c_int /* int */, ydest: c_int /* int */, width: c_int /* int */, height: c_int /* int */) -> bool /* bool */;
    
    // TClassDefExtend(wxScrollEvent,wxEvent)
    pub fn wxScrollEvent_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxPrintPreview,wxObject)
    pub fn wxPrintPreview_CreateFromData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintPreview_CreateFromDialogData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintPreview_Delete(_obj: *u8 /* void* */);
    pub fn wxPrintPreview_DetermineScaling(_obj: *u8 /* void* */);
    pub fn wxPrintPreview_DrawBlankPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintPreview_GetCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintPreview_GetCurrentPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintPreview_GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintPreview_GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintPreview_GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintPreview_GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPrintPreview_GetPrintout(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintPreview_GetPrintoutForPrinting(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintPreview_GetZoom(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintPreview_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintPreview_PaintPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintPreview_Print(_obj: *u8 /* void* */, interactive: bool /* bool */) -> bool /* bool */;
    pub fn wxPrintPreview_RenderPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */;
    pub fn wxPrintPreview_SetCanvas(_obj: *u8 /* void* */, canvas: *u8 /* void* */);
    pub fn wxPrintPreview_SetCurrentPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */;
    pub fn wxPrintPreview_SetFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */);
    pub fn wxPrintPreview_SetOk(_obj: *u8 /* void* */, ok: bool /* bool */);
    pub fn wxPrintPreview_SetPrintout(_obj: *u8 /* void* */, printout: *u8 /* void* */);
    pub fn wxPrintPreview_SetZoom(_obj: *u8 /* void* */, percent: c_int /* int */);
    
    // TClassDefExtend(wxPalette,wxGDIObject)
    pub fn wxPalette_Assign(_obj: *u8 /* void* */, palette: *u8 /* void* */);
    pub fn wxPalette_CreateDefault() -> *u8 /* void* */;
    pub fn wxPalette_CreateRGB(n: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPalette_Delete(_obj: *u8 /* void* */);
    pub fn wxPalette_GetPixel(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) -> c_int /* int */;
    pub fn wxPalette_GetRGB(_obj: *u8 /* void* */, pixel: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPalette_IsEqual(_obj: *u8 /* void* */, palette: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPalette_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxGridRangeSelectEvent,wxNotifyEvent)
    pub fn wxGridRangeSelectEvent_GetTopLeftCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGridRangeSelectEvent_GetBottomRightCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGridRangeSelectEvent_GetTopRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridRangeSelectEvent_GetBottomRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridRangeSelectEvent_GetLeftCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridRangeSelectEvent_GetRightCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridRangeSelectEvent_Selecting(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridRangeSelectEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridRangeSelectEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridRangeSelectEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridRangeSelectEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(cbSimpleUpdatesMgr,cbUpdatesManagerBase)
    
    // TClassDefExtend(wxGenericDirCtrl,wxControl)
    
    // TClassDefExtend(wxMetafile,wxObject)
    pub fn wxMetafile_Create(_file: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMetafile_Delete(_obj: *u8 /* void* */);
    pub fn wxMetafile_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMetafile_Play(_obj: *u8 /* void* */, _dc: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMetafile_SetClipboard(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    
    // TClassDefExtend(wxListItem,wxObject)
    pub fn wxListItem_Clear(_obj: *u8 /* void* */);
    pub fn wxListItem_ClearAttributes(_obj: *u8 /* void* */);
    pub fn wxListItem_Create() -> *u8 /* void* */;
    pub fn wxListItem_Delete(_obj: *u8 /* void* */);
    pub fn wxListItem_GetAlign(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetAttributes(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxListItem_GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxListItem_GetColumn(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetData(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxListItem_GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetImage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetMask(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetState(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxListItem_GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxListItem_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListItem_HasAttributes(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxListItem_SetAlign(_obj: *u8 /* void* */, align: c_int /* int */);
    pub fn wxListItem_SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */);
    pub fn wxListItem_SetColumn(_obj: *u8 /* void* */, col: c_int /* int */);
    pub fn wxListItem_SetData(_obj: *u8 /* void* */, data: c_int /* int */);
    pub fn wxListItem_SetDataPointer(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxListItem_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxListItem_SetId(_obj: *u8 /* void* */, id: c_int /* int */);
    pub fn wxListItem_SetImage(_obj: *u8 /* void* */, image: c_int /* int */);
    pub fn wxListItem_SetMask(_obj: *u8 /* void* */, mask: c_int /* int */);
    pub fn wxListItem_SetState(_obj: *u8 /* void* */, state: c_int /* int */);
    pub fn wxListItem_SetStateMask(_obj: *u8 /* void* */, stateMask: c_int /* int */);
    pub fn wxListItem_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxListItem_SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */);
    pub fn wxListItem_SetWidth(_obj: *u8 /* void* */, width: c_int /* int */);
    
    // TClassDef(wxScopedArray)
    
    // TClassDef(wxDataObject)
    
    // TClassDefExtend(wxCommandProcessor,wxObject)
    pub fn wxCommandProcessor_CanRedo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCommandProcessor_CanUndo(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCommandProcessor_ClearCommands(_obj: *u8 /* void* */);
    pub fn wxCommandProcessor_Delete(_obj: *u8 /* void* */);
    pub fn wxCommandProcessor_GetCommands(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCommandProcessor_GetEditMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCommandProcessor_GetMaxCommands(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCommandProcessor_Initialize(_obj: *u8 /* void* */);
    pub fn wxCommandProcessor_Redo(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCommandProcessor_SetEditMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    pub fn wxCommandProcessor_SetMenuStrings(_obj: *u8 /* void* */);
    pub fn wxCommandProcessor_Submit(_obj: *u8 /* void* */, command: *u8 /* void* */, storeIt: c_int /* int */) -> c_int /* int */;
    pub fn wxCommandProcessor_Undo(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCommandProcessor_wxCommandProcessor(maxCommands: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxInitDialogEvent,wxEvent)
    
    // TClassDef(wxTreeItemId)
    pub fn wxTreeItemId_Create() -> *u8 /* void* */;
    pub fn wxTreeItemId_Delete(_obj: *u8 /* void* */);
    pub fn wxTreeItemId_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxSashLayoutWindow,wxSashWindow)
    pub fn wxSashLayoutWindow_Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSashLayoutWindow_GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashLayoutWindow_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashLayoutWindow_SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */);
    pub fn wxSashLayoutWindow_SetDefaultSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSashLayoutWindow_SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    
    // TClassDefExtend(cbHintAnimationPlugin,cbPluginBase)
    pub fn cbHintAnimationPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbHintAnimationPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbHintAnimationPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxPrinterDC,wxDC)
    pub fn wxPrinterDC_Create(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrinterDC_Delete(self_: *u8 /* void* */);
    pub fn wxPrinterDC_GetPaperRect(self_: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxWindowDestroyEvent,wxCommandEvent)
    pub fn wxWindowDestroyEvent_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxIcon,wxBitmap)
    pub fn wxIcon_Assign(_obj: *u8 /* void* */, other: *u8 /* void* */);
    pub fn wxIcon_CopyFromBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */);
    pub fn wxIcon_CreateDefault() -> *u8 /* void* */;
    pub fn wxIcon_CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxIcon_Delete(_obj: *u8 /* void* */);
    pub fn wxIcon_FromRaw(data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxIcon_FromXPM(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxIcon_GetDepth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIcon_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIcon_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxIcon_IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIcon_Load(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    pub fn wxIcon_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIcon_SetDepth(_obj: *u8 /* void* */, depth: c_int /* int */);
    pub fn wxIcon_SetHeight(_obj: *u8 /* void* */, height: c_int /* int */);
    pub fn wxIcon_SetWidth(_obj: *u8 /* void* */, width: c_int /* int */);
    
    // TClassDefExtend(cbPluginBase,wxEvtHandler)
    pub fn cbPluginBase_Delete(_obj: *u8 /* void* */);
    pub fn cbPluginBase_GetPaneMask(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbPluginBase_IsReady(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn cbPluginBase_Plugin(_swt: c_int /* int */) -> *u8 /* void* */;
    pub fn cbPluginBase_ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxBufferedPaintDC,wxDC)
    pub fn wxBufferedPaintDC_Create(window: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedPaintDC_CreateWithBitmap(window: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedPaintDC_Delete(self_: *u8 /* void* */);
    
    // TClassDefExtend(ELJPlotCurve,wxPlotCurve)
    pub fn ELJPlotCurve_Create(_obj: *u8 /* void* */, _str: *u8 /* void* */, _end: *u8 /* void* */, _y: *u8 /* void* */, offsetY: c_int /* int */, startY: c_double /* double */, endY: c_double /* double */) -> *u8 /* void* */;
    pub fn ELJPlotCurve_Delete(_obj: *u8 /* void* */);
    pub fn ELJPlotCurve_GetEndY(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn ELJPlotCurve_GetOffsetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn ELJPlotCurve_GetStartY(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn ELJPlotCurve_SetEndY(_obj: *u8 /* void* */, endY: c_double /* double */);
    pub fn ELJPlotCurve_SetOffsetY(_obj: *u8 /* void* */, offsetY: c_int /* int */);
    pub fn ELJPlotCurve_SetPenNormal(_obj: *u8 /* void* */, pen: *u8 /* void* */);
    pub fn ELJPlotCurve_SetPenSelected(_obj: *u8 /* void* */, pen: *u8 /* void* */);
    pub fn ELJPlotCurve_SetStartY(_obj: *u8 /* void* */, startY: c_double /* double */);
    
    // TClassDefExtend(wxGraphicsBrush,wxGraphicsObject);
    
    // TClassDefExtend(wxToolTip,wxObject)
    
    // TClassDefExtend(wxcHtmlWindow,wxHtmlWindow)
    pub fn wxcHtmlWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_AppendToPage(_obj: *u8 /* void* */, source: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlWindow_GetInternalRepresentation(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_GetOpenedAnchor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_GetOpenedPage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_GetOpenedPageTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_GetRelatedFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHtmlWindow_HistoryBack(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlWindow_HistoryCanBack(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlWindow_HistoryCanForward(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlWindow_HistoryClear(_obj: *u8 /* void* */);
    pub fn wxHtmlWindow_HistoryForward(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlWindow_LoadPage(_obj: *u8 /* void* */, location: *u8 /* void* */) -> bool /* bool */;
    pub fn wxHtmlWindow_ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxHtmlWindow_SetBorders(_obj: *u8 /* void* */, b: c_int /* int */);
    pub fn wxHtmlWindow_SetFonts(_obj: *u8 /* void* */, normal_face: *u8 /* void* */, fixed_face: *u8 /* void* */, sizes: *c_int /* int* */);
    pub fn wxHtmlWindow_SetPage(_obj: *u8 /* void* */, source: *u8 /* void* */);
    pub fn wxHtmlWindow_SetRelatedFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, format: *u8 /* void* */);
    pub fn wxHtmlWindow_SetRelatedStatusBar(_obj: *u8 /* void* */, bar: c_int /* int */);
    pub fn wxHtmlWindow_WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
    
    // TClassDefExtend(wxValidator,wxEvtHandler)
    pub fn wxValidator_Create() -> *u8 /* void* */;
    pub fn wxValidator_Delete(_obj: *u8 /* void* */);
    pub fn wxValidator_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxValidator_SetBellOnError(doIt: bool /* bool */);
    pub fn wxValidator_SetWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    pub fn wxValidator_TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxValidator_TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxValidator_Validate(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxNodeBase)
    
    // TClassDef(wxDbTableInfo)
    
    // TClassDef(wxDropTarget)
    pub fn wxDropTarget_GetData(_obj: *u8 /* void* */);
    pub fn wxDropTarget_SetDataObject(_obj: *u8 /* void* */, _dat: *u8 /* void* */);
    
    // TClassDefExtend(cbDockPane,wxObject)
    pub fn cbDockPane_BarPresent(_obj: *u8 /* void* */, pBar: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDockPane_Create(alignment: c_int /* int */, pPanel: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDockPane_CreateDefault() -> *u8 /* void* */;
    pub fn cbDockPane_Delete(_obj: *u8 /* void* */);
    pub fn cbDockPane_GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDockPane_GetBarInfoByWindow(_obj: *u8 /* void* */, pBarWnd: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDockPane_GetBarResizeRange(_obj: *u8 /* void* */, pBar: *u8 /* void* */, from: *u8 /* void* */, till: *u8 /* void* */, forLeftHandle: c_int /* int */);
    pub fn cbDockPane_GetDockingState(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDockPane_GetFirstRow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDockPane_GetPaneHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDockPane_GetRealRect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn cbDockPane_GetRowList(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDockPane_GetRowResizeRange(_obj: *u8 /* void* */, pRow: *u8 /* void* */, from: *u8 /* void* */, till: *u8 /* void* */, forUpperHandle: c_int /* int */);
    pub fn cbDockPane_HitTestPaneItems(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, ppRow: *u8 /* void* */, ppBar: *u8 /* void* */) -> c_int /* int */;
    pub fn cbDockPane_InsertBarByCoord(_obj: *u8 /* void* */, pBar: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn cbDockPane_InsertBarByInfo(_obj: *u8 /* void* */, pBarInfo: *u8 /* void* */);
    pub fn cbDockPane_InsertBarToRow(_obj: *u8 /* void* */, pBar: *u8 /* void* */, pIntoRow: *u8 /* void* */);
    pub fn cbDockPane_InsertRow(_obj: *u8 /* void* */, pRow: *u8 /* void* */, pBeforeRow: *u8 /* void* */);
    pub fn cbDockPane_IsHorizontal(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn cbDockPane_MatchesMask(_obj: *u8 /* void* */, paneMask: c_int /* int */) -> c_int /* int */;
    pub fn cbDockPane_RemoveBar(_obj: *u8 /* void* */, pBar: *u8 /* void* */);
    pub fn cbDockPane_RemoveRow(_obj: *u8 /* void* */, pRow: *u8 /* void* */);
    pub fn cbDockPane_SetBoundsInParent(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn cbDockPane_SetMargins(_obj: *u8 /* void* */, top: c_int /* int */, bottom: c_int /* int */, left: c_int /* int */, right: c_int /* int */);
    pub fn cbDockPane_SetPaneWidth(_obj: *u8 /* void* */, width: c_int /* int */);
    
    // TClassDefExtend(wxToggleButton,wxControl)
    pub fn wxToggleButton_Create(parent: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxToggleButton_Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    pub fn wxToggleButton_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxToggleButton_SetLabel(_obj: *u8 /* void* */, label: *u8 /* void* */);
    pub fn wxToggleButton_SetValue(_obj: *u8 /* void* */, state: bool /* bool */);
    
    // TClassDefExtend(wxControl,wxWindow)
    pub fn wxControl_Command(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxControl_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxControl_SetLabel(_obj: *u8 /* void* */, text: *u8 /* void* */);
    
    // TClassDefExtend(wxContextHelpButton,wxBitmapButton)
    pub fn wxContextHelpButton_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_long /* long */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxHtmlWidgetCell,wxHtmlCell)
    
    // TClassDefExtend(wxZipInputStream,wxInputStream)
    
    // TClassDefExtend(wxHtmlHelpData,wxObject)
    
    // TClassDefExtend(wxComboBox,wxChoice)
    pub fn wxComboBox_Append(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxComboBox_AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, d: *u8 /* void* */);
    pub fn wxComboBox_Clear(_obj: *u8 /* void* */);
    pub fn wxComboBox_Copy(_obj: *u8 /* void* */);
    pub fn wxComboBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxComboBox_Cut(_obj: *u8 /* void* */);
    pub fn wxComboBox_Delete(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxComboBox_FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    pub fn wxComboBox_GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxComboBox_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxComboBox_GetInsertionPoint(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxComboBox_GetLastPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxComboBox_GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxComboBox_GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxComboBox_GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxComboBox_GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxComboBox_Paste(_obj: *u8 /* void* */);
    pub fn wxComboBox_Remove(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */);
    pub fn wxComboBox_Replace(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */, value: *u8 /* void* */);
    pub fn wxComboBox_SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */);
    pub fn wxComboBox_SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */);
    pub fn wxComboBox_SetInsertionPoint(_obj: *u8 /* void* */, pos: c_int /* int */);
    pub fn wxComboBox_SetInsertionPointEnd(_obj: *u8 /* void* */);
    pub fn wxComboBox_SetSelection(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxComboBox_SetTextSelection(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */);
    
    // TClassDefExtend(wxHtmlCell,wxObject)
    
    // TClassDefExtend(wxFont,wxGDIObject)
    pub fn wxFont_Create(pointSize: c_int /* int */, family: c_int /* int */, style: c_int /* int */, weight: c_int /* int */, underlined: bool /* bool */, face: *u8 /* void* */, enc: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFont_CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFont_CreateDefault() -> *u8 /* void* */;
    pub fn wxFont_Delete(_obj: *u8 /* void* */);
    pub fn wxFont_GetDefaultEncoding(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetFaceName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFont_GetFamily(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetFamilyString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFont_GetPointSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetStyleString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFont_GetUnderlined(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetWeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFont_GetWeightString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFont_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFont_SetDefaultEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */);
    pub fn wxFont_SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */);
    pub fn wxFont_SetFaceName(_obj: *u8 /* void* */, faceName: *u8 /* void* */);
    pub fn wxFont_SetFamily(_obj: *u8 /* void* */, family: c_int /* int */);
    pub fn wxFont_SetPointSize(_obj: *u8 /* void* */, pointSize: c_int /* int */);
    pub fn wxFont_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    pub fn wxFont_SetUnderlined(_obj: *u8 /* void* */, underlined: c_int /* int */);
    pub fn wxFont_SetWeight(_obj: *u8 /* void* */, weight: c_int /* int */);
    
    // TClassDefExtend(cbAntiflickerPlugin,cbPluginBase)
    pub fn cbAntiflickerPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbAntiflickerPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbAntiflickerPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxMemoryOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxNavigationKeyEvent,wxEvent)
    pub fn wxNavigationKeyEvent_GetCurrentFocus(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxNavigationKeyEvent_GetDirection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNavigationKeyEvent_IsWindowChange(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNavigationKeyEvent_SetCurrentFocus(_obj: *u8 /* void* */, win: *u8 /* void* */);
    pub fn wxNavigationKeyEvent_SetDirection(_obj: *u8 /* void* */, bForward: bool /* bool */);
    pub fn wxNavigationKeyEvent_SetWindowChange(_obj: *u8 /* void* */, bIs: bool /* bool */);
    pub fn wxNavigationKeyEvent_ShouldPropagate(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxDateProperty,wxPGProperty)
    pub fn wxDateProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxZlibInputStream,wxFilterInputStream)
    
    // TClassDefExtend(cbPaneDrawPlugin,cbPluginBase)
    pub fn cbPaneDrawPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbPaneDrawPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbPaneDrawPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxProcessEvent,wxEvent)
    pub fn wxProcessEvent_GetExitCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxProcessEvent_GetPid(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxPrinter,wxObject)
    pub fn wxPrinter_Create(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrinter_CreateAbortWindow(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrinter_Delete(_obj: *u8 /* void* */);
    pub fn wxPrinter_GetAbort(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrinter_GetLastError(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrinter_GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPrinter_Print(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, prompt: bool /* bool */) -> bool /* bool */;
    pub fn wxPrinter_PrintDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrinter_ReportError(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, message: *u8 /* void* */);
    pub fn wxPrinter_Setup(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxView,wxEvtHandler)
    
    // TClassDef(wxDbColInf)
    
    // TClassDefExtend(wxColourDatabase,wxList)
    
    // TClassDefExtend(wxGridCellNumberEditor,wxGridCellTextEditor)
    pub fn wxGridCellNumberEditor_Ctor(min: c_int /* int */, max: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxFlexGridSizer,wxGridSizer)
    pub fn wxFlexGridSizer_AddGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    pub fn wxFlexGridSizer_AddGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    pub fn wxFlexGridSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFlexGridSizer_Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFlexGridSizer_RecalcSizes(_obj: *u8 /* void* */);
    pub fn wxFlexGridSizer_RemoveGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    pub fn wxFlexGridSizer_RemoveGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    
    // TClassDefExtend(wxMouseCaptureChangedEvent,wxEvent)
    
    // TClassDefExtend(wxGraphicsObject,wxObject);
    
    // TClassDefExtend(wxInputSinkEvent,wxEvent)
    pub fn wxInputSinkEvent_LastError(obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputSinkEvent_LastRead(obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputSinkEvent_LastInput(obj: *u8 /* void* */) -> *char /* char* */;
    
    // TClassDefExtend(wxPrintout,wxObject)
    
    // TClassDefExtend(wxXmlResource,wxObject)
    pub fn wxXmlResource_AddHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    pub fn wxXmlResource_AddSubclassFactory(_obj: *u8 /* void* */, factory: *u8 /* void* */);
    pub fn wxXmlResource_AttachUnknownControl(_obj: *u8 /* void* */, control: *u8 /* void* */, parent: *u8 /* void* */) -> c_int /* int */;
    pub fn wxXmlResource_ClearHandlers(_obj: *u8 /* void* */);
    pub fn wxXmlResource_CompareVersion(_obj: *u8 /* void* */, major: c_int /* int */, minor: c_int /* int */, release: c_int /* int */, revision: c_int /* int */) -> c_int /* int */;
    pub fn wxXmlResource_Create(flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxXmlResource_CreateFromFile(filemask: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxXmlResource_Delete(_obj: *u8 /* void* */);
    pub fn wxXmlResource_Get() -> *u8 /* void* */;
    pub fn wxXmlResource_GetDomain(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxXmlResource_GetVersion(_obj: *u8 /* void* */) -> c_long /* long */;
    pub fn wxXmlResource_GetXRCID(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> c_int /* int */;
    pub fn wxXmlResource_InitAllHandlers(_obj: *u8 /* void* */);
    pub fn wxXmlResource_InsertHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    pub fn wxXmlResource_Load(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */;
    pub fn wxXmlResource_LoadBitmap(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxXmlResource_LoadDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_LoadFrame(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_LoadIcon(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxXmlResource_LoadMenu(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_LoadMenuBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_LoadPanel(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_LoadToolBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetStaticBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetFlexGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetBitmapButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetCalendarCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetCheckBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetCheckListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetChoice(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetComboBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetGauge(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetGrid(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetHtmlWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetListCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetMDIChildFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetMDIParentFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetMenu(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetMenuBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetMenuItem(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetNotebook(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetPanel(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetRadioButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetRadioBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetScrollBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetScrolledWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetSlider(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetSpinButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetSpinCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetSplitterWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetStaticBitmap(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetStaticBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetStaticLine(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetStaticText(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetTextCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_GetTreeCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_Unload(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */;
    pub fn wxXmlResource_Set(_obj: *u8 /* void* */, res: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxXmlResource_SetDomain(_obj: *u8 /* void* */, domain: *u8 /* void* */);
    pub fn wxXmlResource_SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    
    // TClassDefExtend(wxXmlResource,wxObject)
    pub fn wxXmlResource_GetStyledTextCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxClientBase,wxObject)
    
    // TClassDefExtend(wxPrintDialog,wxDialog)
    pub fn wxPrintDialog_Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintDialog_GetPrintDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintDialog_GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPrintDialog_GetPrintDialogData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDDEClient,wxClientBase)
    
    // TClassDefExtend(wxPathList,wxList)
    
    // TClassDefExtend(wxStaticBitmap,wxControl)
    pub fn wxStaticBitmap_Create(_prt: *u8 /* void* */, _id: c_int /* int */, bitmap: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStaticBitmap_Delete(_obj: *u8 /* void* */);
    pub fn wxStaticBitmap_GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxStaticBitmap_GetIcon(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxStaticBitmap_SetBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    pub fn wxStaticBitmap_SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */);
    
    // TClassDefExtend(wxSVGFileDC,wxDC)
    pub fn wxSVGFileDC_Create(fileName: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSVGFileDC_CreateWithSize(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSVGFileDC_CreateWithSizeAndResolution(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, a_dpi: float /* float */) -> *u8 /* void* */;
    pub fn wxSVGFileDC_Delete(obj: *u8 /* void* */);
    
    // TClassDefExtend(wxStringClientData,wxClientData)
    
    // TClassDefExtend(wxQueryLayoutInfoEvent,wxEvent)
    pub fn wxQueryLayoutInfoEvent_Create(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxQueryLayoutInfoEvent_GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxQueryLayoutInfoEvent_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxQueryLayoutInfoEvent_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxQueryLayoutInfoEvent_GetRequestedLength(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxQueryLayoutInfoEvent_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxQueryLayoutInfoEvent_SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */);
    pub fn wxQueryLayoutInfoEvent_SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxQueryLayoutInfoEvent_SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    pub fn wxQueryLayoutInfoEvent_SetRequestedLength(_obj: *u8 /* void* */, length: c_int /* int */);
    pub fn wxQueryLayoutInfoEvent_SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(wxGridCellAutoWrapStringRenderer,wxGridCellStringRenderer)
    pub fn wxGridCellAutoWrapStringRenderer_Ctor() -> *u8 /* void* */;
    
    // TClassDefExtend(wxFileDialog,wxDialog)
    pub fn wxFileDialog_Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, _fle: *u8 /* void* */, _wcd: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFileDialog_GetDirectory(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileDialog_GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileDialog_GetFilenames(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxFileDialog_GetFilterIndex(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileDialog_GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileDialog_GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileDialog_GetPaths(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxFileDialog_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileDialog_GetWildcard(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileDialog_SetDirectory(_obj: *u8 /* void* */, dir: *u8 /* void* */);
    pub fn wxFileDialog_SetFilename(_obj: *u8 /* void* */, name: *u8 /* void* */);
    pub fn wxFileDialog_SetFilterIndex(_obj: *u8 /* void* */, filterIndex: c_int /* int */);
    pub fn wxFileDialog_SetMessage(_obj: *u8 /* void* */, message: *u8 /* void* */);
    pub fn wxFileDialog_SetPath(_obj: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxFileDialog_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    pub fn wxFileDialog_SetWildcard(_obj: *u8 /* void* */, wildCard: *u8 /* void* */);
    
    // TClassDef(wxDllLoader)
    
    // TClassDefExtend(ELJLog,wxLog)
    pub fn ELJLog_AddTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */);
    pub fn ELJLog_Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJLog_Delete(_obj: *u8 /* void* */);
    pub fn ELJLog_DontCreateOnDemand(_obj: *u8 /* void* */);
    pub fn ELJLog_EnableLogging(_obj: *u8 /* void* */, doIt: bool /* bool */) -> c_int /* int */;
    pub fn ELJLog_Flush(_obj: *u8 /* void* */);
    pub fn ELJLog_FlushActive(_obj: *u8 /* void* */);
    pub fn ELJLog_GetActiveTarget() -> *u8 /* void* */;
    pub fn ELJLog_GetTimestamp(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJLog_GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn ELJLog_GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn ELJLog_HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn ELJLog_IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */;
    pub fn ELJLog_IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn ELJLog_OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *u8 /* void* */, t: c_int /* int */);
    pub fn ELJLog_RemoveTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */);
    pub fn ELJLog_Resume(_obj: *u8 /* void* */);
    pub fn ELJLog_SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJLog_SetTimestamp(_obj: *u8 /* void* */, ts: *u8 /* void* */);
    pub fn ELJLog_SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */);
    pub fn ELJLog_SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */);
    pub fn ELJLog_Suspend(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxConnectionBase,wxObject)
    
    // TClassDefExtend(wxMenuItem,wxObject)
    pub fn wxMenuItem_Check(_obj: *u8 /* void* */, check: bool /* bool */);
    pub fn wxMenuItem_Create() -> *u8 /* void* */;
    pub fn wxMenuItem_Delete(_obj: *u8 /* void* */);
    pub fn wxMenuItem_Enable(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxMenuItem_GetHelp(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuItem_GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenuItem_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuItem_GetLabelFromText(text: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    pub fn wxMenuItem_GetMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuItem_GetSubMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuItem_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuItem_IsCheckable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMenuItem_IsChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMenuItem_IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMenuItem_IsSeparator(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMenuItem_IsSubMenu(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMenuItem_SetCheckable(_obj: *u8 /* void* */, checkable: bool /* bool */);
    pub fn wxMenuItem_SetHelp(_obj: *u8 /* void* */, str: *u8 /* void* */);
    pub fn wxMenuItem_SetId(_obj: *u8 /* void* */, id: c_int /* int */);
    pub fn wxMenuItem_SetSubMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    pub fn wxMenuItem_SetText(_obj: *u8 /* void* */, str: *u8 /* void* */);
    
    // TClassDef(wxDynamicLibrary)
    
    // TClassDefExtend(wxSound,wxEvtHandler)
    
    // TClassDefExtend(wxSound,wxObject);
    pub fn wxSound_Create(fileName: *u8 /* void* */, isResource: bool /* bool */) -> *u8 /* void* */;
    pub fn wxSound_Delete(self_: *u8 /* void* */);
    pub fn wxSound_IsOk(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSound_Play(self_: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */;
    pub fn wxSound_Stop(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxMoveEvent,wxEvent)
    pub fn wxMoveEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxMoveEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxToolWindow,wxFrame)
    pub fn wxToolWindow_AddMiniButton(_obj: *u8 /* void* */, _btn: *u8 /* void* */);
    pub fn wxToolWindow_Create(_obj: *u8 /* void* */, _btn: *u8 /* void* */, _ttl: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxToolWindow_GetClient(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxToolWindow_SetClient(_obj: *u8 /* void* */, _wnd: *u8 /* void* */);
    pub fn wxToolWindow_SetTitleFont(_obj: *u8 /* void* */, _fnt: *u8 /* void* */);
    
    // TClassDef(wxFontEnumerator)
    pub fn wxFontEnumerator_Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFontEnumerator_Delete(_obj: *u8 /* void* */);
    pub fn wxFontEnumerator_EnumerateEncodings(_obj: *u8 /* void* */, facename: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFontEnumerator_EnumerateFacenames(_obj: *u8 /* void* */, encoding: c_int /* int */, fixedWidthOnly: c_int /* int */) -> bool /* bool */;
    
    // TClassDefExtend(wxMultiCellCanvas,wxFlexGridSizer)
    pub fn wxMultiCellCanvas_Add(_obj: *u8 /* void* */, win: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */);
    pub fn wxMultiCellCanvas_CalculateConstraints(_obj: *u8 /* void* */);
    pub fn wxMultiCellCanvas_Create(parent: *u8 /* void* */, numRows: c_int /* int */, numCols: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMultiCellCanvas_MaxCols(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellCanvas_MaxRows(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMultiCellCanvas_SetMinCellSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(cbStartBarDraggingEvent,cbPluginEvent)
    pub fn cbStartBarDraggingEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbStartBarDraggingEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxGridSizeEvent,wxNotifyEvent)
    pub fn wxGridSizeEvent_GetRowOrCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridSizeEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridSizeEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridSizeEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridSizeEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridSizeEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxFTP,wxProtocol)
    
    // TClassDefExtend(wxWizard,wxDialog)
    pub fn wxWizard_Chain(f: *u8 /* void* */, s: *u8 /* void* */);
    pub fn wxWizard_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */;
    pub fn wxWizard_GetCurrentPage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizard_GetPageSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizard_RunWizard(_obj: *u8 /* void* */, firstPage: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWizard_SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(wxHtmlWinParser,wxHtmlParser)
    
    // TClassDef(wxFontMapper)
    pub fn wxFontMapper_Create() -> *u8 /* void* */;
    pub fn wxFontMapper_GetAltForEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */, alt_encoding: *u8 /* void* */, _buf: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFontMapper_IsEncodingAvailable(_obj: *u8 /* void* */, encoding: c_int /* int */, _buf: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxPrintData,wxObject)
    pub fn wxPrintData_Assign(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxPrintData_Create() -> *u8 /* void* */;
    pub fn wxPrintData_Delete(_obj: *u8 /* void* */);
    pub fn wxPrintData_GetCollate(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintData_GetColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintData_GetDuplex(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetFontMetricPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetPreviewCommand(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetPrintMode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetPrinterCommand(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetPrinterName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetPrinterOptions(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintData_GetPrinterScaleX(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxPrintData_GetPrinterScaleY(_obj: *u8 /* void* */) -> c_double /* double */;
    pub fn wxPrintData_GetPrinterTranslateX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetPrinterTranslateY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_GetQuality(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintData_SetCollate(_obj: *u8 /* void* */, flag: c_int /* int */);
    pub fn wxPrintData_SetColour(_obj: *u8 /* void* */, colour: c_int /* int */);
    pub fn wxPrintData_SetDuplex(_obj: *u8 /* void* */, duplex: c_int /* int */);
    pub fn wxPrintData_SetFilename(_obj: *u8 /* void* */, filename: *u8 /* void* */);
    pub fn wxPrintData_SetFontMetricPath(_obj: *u8 /* void* */, path: *u8 /* void* */);
    pub fn wxPrintData_SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */);
    pub fn wxPrintData_SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    pub fn wxPrintData_SetPaperId(_obj: *u8 /* void* */, sizeId: c_int /* int */);
    pub fn wxPrintData_SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPrintData_SetPreviewCommand(_obj: *u8 /* void* */, command: *u8 /* void* */);
    pub fn wxPrintData_SetPrintMode(_obj: *u8 /* void* */, printMode: c_int /* int */);
    pub fn wxPrintData_SetPrinterCommand(_obj: *u8 /* void* */, command: *u8 /* void* */);
    pub fn wxPrintData_SetPrinterName(_obj: *u8 /* void* */, name: *u8 /* void* */);
    pub fn wxPrintData_SetPrinterOptions(_obj: *u8 /* void* */, options: *u8 /* void* */);
    pub fn wxPrintData_SetPrinterScaleX(_obj: *u8 /* void* */, x: c_double /* double */);
    pub fn wxPrintData_SetPrinterScaleY(_obj: *u8 /* void* */, y: c_double /* double */);
    pub fn wxPrintData_SetPrinterScaling(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */);
    pub fn wxPrintData_SetPrinterTranslateX(_obj: *u8 /* void* */, x: c_int /* int */);
    pub fn wxPrintData_SetPrinterTranslateY(_obj: *u8 /* void* */, y: c_int /* int */);
    pub fn wxPrintData_SetPrinterTranslation(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxPrintData_SetQuality(_obj: *u8 /* void* */, quality: c_int /* int */);
    
    // TClassDefExtend(wxSashEvent,wxEvent)
    pub fn wxSashEvent_Create(id: c_int /* int */, edge: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSashEvent_GetDragRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSashEvent_GetDragStatus(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashEvent_GetEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashEvent_SetDragRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxSashEvent_SetDragStatus(_obj: *u8 /* void* */, status: c_int /* int */);
    pub fn wxSashEvent_SetEdge(_obj: *u8 /* void* */, edge: c_int /* int */);
    
    // TClassDefExtend(cbPluginEvent,wxEvent)
    pub fn cbPluginEvent_Pane(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPropertyGridEvent,wxNotifyEvent)
    pub fn wxPropertyGridEvent_HasProperty(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPropertyGridEvent_GetProperty(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxSingleChoiceDialog,wxDialog)
    
    // TClassDef(wxObject)
    
    // TClassDef(wxObject)
    pub fn wxObject_GetClassInfo(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxObject_IsKindOf(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */;
    pub fn wxObject_IsScrolledWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxDropSource)
    pub fn DropSource_Create(data: *u8 /* void* */, win: *u8 /* void* */, copy: *u8 /* void* */, move: *u8 /* void* */, none: *u8 /* void* */) -> *u8 /* void* */;
    pub fn DropSource_Delete(_obj: *u8 /* void* */);
    pub fn DropSource_DoDragDrop(_obj: *u8 /* void* */, _move: c_int /* int */) -> c_int /* int */;
    
    // TClassDefExtend(wxHtmlParser,wxObject)
    
    // TClassDefExtend(wxFindReplaceData,wxObject)
    pub fn wxFindReplaceData_Create(flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFindReplaceData_CreateDefault() -> *u8 /* void* */;
    pub fn wxFindReplaceData_Delete(_obj: *u8 /* void* */);
    pub fn wxFindReplaceData_GetFindString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFindReplaceData_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFindReplaceData_GetReplaceString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFindReplaceData_SetFindString(_obj: *u8 /* void* */, str: *u8 /* void* */);
    pub fn wxFindReplaceData_SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxFindReplaceData_SetReplaceString(_obj: *u8 /* void* */, str: *u8 /* void* */);
    
    // TClassDefExtend(wxToolLayoutItem,wxObject)
    pub fn wxToolLayoutItem_IsSeparator(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxToolLayoutItem_Rect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    
    // TClassDefExtend(cbLeftDownEvent,cbPluginEvent)
    pub fn cbLeftDownEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxPlotCurve,wxObject)
    
    // TClassDef(wxMBConv)
    
    // TClassDefExtend(cbUpdatesManagerBase,wxObject)
    
    // TClassDefExtend(wxLogTextCtrl,wxLog)
    
    // TClassDefExtend(wxStatusBar,wxWindow)
    pub fn wxStatusBar_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStatusBar_GetBorderX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStatusBar_GetBorderY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStatusBar_GetFieldsCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStatusBar_GetStatusText(_obj: *u8 /* void* */, number: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStatusBar_SetFieldsCount(_obj: *u8 /* void* */, number: c_int /* int */, widths: *c_int /* int* */);
    pub fn wxStatusBar_SetMinHeight(_obj: *u8 /* void* */, height: c_int /* int */);
    pub fn wxStatusBar_SetStatusText(_obj: *u8 /* void* */, text: *u8 /* void* */, number: c_int /* int */);
    pub fn wxStatusBar_SetStatusWidths(_obj: *u8 /* void* */, n: c_int /* int */, widths: *c_int /* int* */);
    
    // TClassDefExtend(wxTextDataObject,wxDataObjectSimple)
    pub fn TextDataObject_Create(_txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn TextDataObject_Delete(_obj: *u8 /* void* */);
    pub fn TextDataObject_GetTextLength(_obj: *u8 /* void* */) -> size_t /* size_t */;
    pub fn TextDataObject_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn TextDataObject_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    
    // TClassDefExtend(wxSizer,wxObject)
    pub fn wxSizer_Add(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_AddSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_AddWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizer_Fit(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxSizer_GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */;
    pub fn wxSizer_GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizer_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizer_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizer_Insert(_obj: *u8 /* void* */, before: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_InsertSizer(_obj: *u8 /* void* */, before: c_int /* int */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_InsertWindow(_obj: *u8 /* void* */, before: c_int /* int */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_Layout(_obj: *u8 /* void* */);
    pub fn wxSizer_Prepend(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_PrependSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_PrependWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    pub fn wxSizer_RecalcSizes(_obj: *u8 /* void* */);
    pub fn wxSizer_SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxSizer_SetItemMinSize(_obj: *u8 /* void* */, pos: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizer_SetItemMinSizeSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizer_SetItemMinSizeWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizer_SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSizer_SetSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxSizer_AddSpacer(_obj: *u8 /* void* */, size: c_int /* int */);
    pub fn wxSizer_AddStretchSpacer(_obj: *u8 /* void* */, size: c_int /* int */);
    pub fn wxSizer_Clear(_obj: *u8 /* void* */, delete_windows: bool /* bool */);
    pub fn wxSizer_DetachWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_DetachSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_Detach(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    pub fn wxSizer_FitInside(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxSizer_GetContainingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSizer_GetItemWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */;
    pub fn wxSizer_GetItemSizer(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */;
    pub fn wxSizer_GetItem(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSizer_HideWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_HideSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_Hide(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    pub fn wxSizer_InsertSpacer(_obj: *u8 /* void* */, index: c_int /* int */, size: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSizer_InsertStretchSpacer(_obj: *u8 /* void* */, index: c_int /* int */, prop: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSizer_IsShownWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_IsShownSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_IsShown(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    pub fn wxSizer_PrependSpacer(_obj: *u8 /* void* */, size: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSizer_PrependStretchSpacer(_obj: *u8 /* void* */, prop: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSizer_ReplaceWindow(_obj: *u8 /* void* */, oldwin: *u8 /* void* */, newwin: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */;
    pub fn wxSizer_ReplaceSizer(_obj: *u8 /* void* */, oldsz: *u8 /* void* */, newsz: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */;
    pub fn wxSizer_Replace(_obj: *u8 /* void* */, oldindex: c_int /* int */, newitem: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSizer_SetVirtualSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxSizer_ShowWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */;
    pub fn wxSizer_ShowSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */;
    pub fn wxSizer_Show(_obj: *u8 /* void* */, sizer: *u8 /* void* */, index: c_int /* int */, show: bool /* bool */) -> bool /* bool */;
    
    // TClassDefExtend(wxMenu,wxEvtHandler)
    pub fn wxMenu_Append(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */);
    pub fn wxMenu_AppendItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    pub fn wxMenu_AppendSeparator(_obj: *u8 /* void* */);
    pub fn wxMenu_AppendSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */);
    pub fn wxMenu_Break(_obj: *u8 /* void* */);
    pub fn wxMenu_Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */);
    pub fn wxMenu_Create(title: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */;
    pub fn wxMenu_DeleteById(_obj: *u8 /* void* */, id: c_int /* int */);
    pub fn wxMenu_DeleteByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    pub fn wxMenu_DeletePointer(_obj: *u8 /* void* */);
    pub fn wxMenu_DestroyById(_obj: *u8 /* void* */, id: c_int /* int */);
    pub fn wxMenu_DestroyByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    pub fn wxMenu_Enable(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */);
    pub fn wxMenu_FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenu_FindItemByLabel(_obj: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenu_GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenu_GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenu_GetInvokingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenu_GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMenu_GetMenuItemCount(_obj: *u8 /* void* */) -> size_t /* size_t */;
    pub fn wxMenu_GetMenuItems(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenu_GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenu_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMenu_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenu_Insert(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */);
    pub fn wxMenu_InsertItem(_obj: *u8 /* void* */, pos: size_t /* size_t */, _itm: *u8 /* void* */);
    pub fn wxMenu_InsertSub(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */);
    pub fn wxMenu_IsAttached(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMenu_IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxMenu_IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    pub fn wxMenu_Prepend(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */);
    pub fn wxMenu_PrependItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    pub fn wxMenu_PrependSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */);
    pub fn wxMenu_RemoveById(_obj: *u8 /* void* */, id: c_int /* int */, _itm: *u8 /* void* */);
    pub fn wxMenu_RemoveByItem(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxMenu_SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */);
    pub fn wxMenu_SetEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    pub fn wxMenu_SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */);
    pub fn wxMenu_SetInvokingWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    pub fn wxMenu_SetLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */);
    pub fn wxMenu_SetParent(_obj: *u8 /* void* */, parent: *u8 /* void* */);
    pub fn wxMenu_SetTitle(_obj: *u8 /* void* */, title: *u8 /* void* */);
    pub fn wxMenu_UpdateUI(_obj: *u8 /* void* */, source: *u8 /* void* */);
    
    // TClassDef(wxFileType)
    pub fn wxFileType_Delete(_obj: *u8 /* void* */);
    pub fn wxFileType_ExpandCommand(_obj: *u8 /* void* */, _cmd: *u8 /* void* */, _params: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileType_GetDescription(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileType_GetExtensions(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileType_GetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileType_GetMimeType(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFileType_GetMimeTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileType_GetOpenCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileType_GetPrintCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxRadioButton,wxControl)
    pub fn wxRadioButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxRadioButton_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRadioButton_SetValue(_obj: *u8 /* void* */, value: bool /* bool */);
    
    // TClassDefExtend(wxFontList,wxList)
    
    // TClassDefExtend(cbDrawRowHandlesEvent,cbPluginEvent)
    pub fn cbDrawRowHandlesEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDrawRowHandlesEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxCheckListBox,wxListBox)
    pub fn wxCheckListBox_Check(_obj: *u8 /* void* */, item: c_int /* int */, check: bool /* bool */);
    pub fn wxCheckListBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCheckListBox_Delete(_obj: *u8 /* void* */);
    pub fn wxCheckListBox_IsChecked(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
    
    // TClassDefExtend(wxTreeLayoutStored,wxTreeLayout)
    
    // TClassDefExtend(wxSocketInputStream,wxInputStream)
    
    // TClassDefExtend(wxDocChildFrame,wxFrame)
    
    // TClassDefExtend(wxCloseEvent,wxEvent)
    pub fn wxCloseEvent_CanVeto(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCloseEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxCloseEvent_GetLoggingOff(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCloseEvent_GetVeto(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCloseEvent_SetCanVeto(_obj: *u8 /* void* */, canVeto: bool /* bool */);
    pub fn wxCloseEvent_SetLoggingOff(_obj: *u8 /* void* */, logOff: bool /* bool */);
    pub fn wxCloseEvent_Veto(_obj: *u8 /* void* */, veto: bool /* bool */);
    
    // TClassDefExtend(wxFileProperty,wxPGProperty)
    pub fn wxFileProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxChoice,wxControl)
    pub fn wxChoice_Append(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxChoice_Clear(_obj: *u8 /* void* */);
    pub fn wxChoice_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxChoice_Delete(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxChoice_FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    pub fn wxChoice_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxChoice_GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxChoice_GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxChoice_SetSelection(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxChoice_SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */);
    
    // TClassDefExtend(wxIntProperty,wxPGProperty)
    pub fn wxIntProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxDbColFor)
    
    // TClassDefExtend(wxMenuEvent,wxEvent)
    pub fn wxMenuEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxMenuEvent_GetMenuId(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDef(wxCalendarDateAttr)
    pub fn wxCalendarDateAttr_Create(_ctxt: *u8 /* void* */, _cbck: *u8 /* void* */, _cbrd: *u8 /* void* */, _fnt: *u8 /* void* */, _brd: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCalendarDateAttr_CreateDefault() -> *u8 /* void* */;
    pub fn wxCalendarDateAttr_Delete(_obj: *u8 /* void* */);
    pub fn wxCalendarDateAttr_GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarDateAttr_GetBorder(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCalendarDateAttr_GetBorderColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarDateAttr_GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarDateAttr_GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarDateAttr_HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCalendarDateAttr_HasBorder(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCalendarDateAttr_HasBorderColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCalendarDateAttr_HasFont(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCalendarDateAttr_HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCalendarDateAttr_IsHoliday(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCalendarDateAttr_SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxCalendarDateAttr_SetBorder(_obj: *u8 /* void* */, border: c_int /* int */);
    pub fn wxCalendarDateAttr_SetBorderColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxCalendarDateAttr_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxCalendarDateAttr_SetHoliday(_obj: *u8 /* void* */, holiday: c_int /* int */);
    pub fn wxCalendarDateAttr_SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    
    // TClassDefExtend(wxGridCellNumberRenderer,wxGridCellStringRenderer)
    pub fn wxGridCellNumberRenderer_Ctor() -> *u8 /* void* */;
    
    // TClassDef(wxSTCDoc)
    
    // TClassDefExtend(wxMessageDialog,wxDialog)
    pub fn wxMessageDialog_Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _cap: *u8 /* void* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMessageDialog_Delete(_obj: *u8 /* void* */);
    pub fn wxMessageDialog_ShowModal(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxPrivateDropTarget,wxDropTarget)
    
    // TClassDefExtend(wxEncodingConverter,wxObject)
    pub fn wxEncodingConverter_Convert(_obj: *u8 /* void* */, input: *u8 /* void* */, output: *u8 /* void* */);
    pub fn wxEncodingConverter_Create() -> *u8 /* void* */;
    pub fn wxEncodingConverter_Delete(_obj: *u8 /* void* */);
    pub fn wxEncodingConverter_GetAllEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEncodingConverter_GetPlatformEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, platform: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEncodingConverter_Init(_obj: *u8 /* void* */, input_enc: c_int /* int */, output_enc: c_int /* int */, method: c_int /* int */) -> c_int /* int */;
    
    // TClassDef(wxCriticalSection)
    pub fn wxCriticalSection_Create() -> *u8 /* void* */;
    pub fn wxCriticalSection_Delete(_obj: *u8 /* void* */);
    pub fn wxCriticalSection_Enter(_obj: *u8 /* void* */);
    pub fn wxCriticalSection_Leave(_obj: *u8 /* void* */);
    
    // TClassDef(wxTextFile)
    
    // TClassDefExtend(wxUpdateUIEvent,wxEvent)
    pub fn wxUpdateUIEvent_Check(_obj: *u8 /* void* */, check: bool /* bool */);
    pub fn wxUpdateUIEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxUpdateUIEvent_Enable(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxUpdateUIEvent_GetChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxUpdateUIEvent_GetEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxUpdateUIEvent_GetSetChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxUpdateUIEvent_GetSetEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxUpdateUIEvent_GetSetText(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxUpdateUIEvent_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxUpdateUIEvent_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    
    // TClassDef(wxTimeSpan)
    
    // TClassDefExtend(wxGridSizer,wxSizer)
    pub fn wxGridSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridSizer_Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGridSizer_GetCols(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridSizer_GetHGap(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridSizer_GetRows(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridSizer_GetVGap(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridSizer_RecalcSizes(_obj: *u8 /* void* */);
    pub fn wxGridSizer_SetCols(_obj: *u8 /* void* */, cols: c_int /* int */);
    pub fn wxGridSizer_SetHGap(_obj: *u8 /* void* */, gap: c_int /* int */);
    pub fn wxGridSizer_SetRows(_obj: *u8 /* void* */, rows: c_int /* int */);
    pub fn wxGridSizer_SetVGap(_obj: *u8 /* void* */, gap: c_int /* int */);
    
    // TClassDef(wxThread)
    
    // TClassDef(ELJMessageParameters)
    pub fn wxMessageParameters_Create(_file: *wchar_t /* wchar_t* */, _type: *wchar_t /* wchar_t* */, _object: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMessageParameters_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxPopupTransientWindow,wxPopupWindow)
    
    // TClassDefExtend(ELJLocale,wxLocale)
    
    // TClassDefExtend(wxEditableListBox,wxPanel)
    pub fn wxEditableListBox_Create(parent: *u8 /* void* */, id: c_int /* int */, label: *wchar_t /* wchar_t* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxEditableListBox_GetDelButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEditableListBox_GetDownButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEditableListBox_GetEditButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEditableListBox_GetListCtrl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEditableListBox_GetNewButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEditableListBox_GetStrings(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxEditableListBox_GetUpButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEditableListBox_SetStrings(_obj: *u8 /* void* */, strings: *u8 /* void* */, _n: c_int /* int */);
    
    // TClassDefExtend(cbDrawPaneBkGroundEvent,cbPluginEvent)
    pub fn cbDrawPaneBkGroundEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend( wxcPrintoutHandler, wxEvtHandler );
    pub fn wxcPrintout_Create(title: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcPrintout_Delete(self_: *u8 /* void* */);
    pub fn wxcPrintout_SetPageLimits(self_: *u8 /* void* */, startPage: c_int /* int */, endPage: c_int /* int */, fromPage: c_int /* int */, toPage: c_int /* int */);
    pub fn wxcPrintout_GetEvtHandler(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcPrintEvent_GetPrintout(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcPrintEvent_GetPage(self_: *u8 /* void* */) -> c_int /* int */;
    pub fn wxcPrintEvent_GetEndPage(self_: *u8 /* void* */) -> c_int /* int */;
    pub fn wxcPrintEvent_GetContinue(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxcPrintEvent_SetContinue(self_: *u8 /* void* */, cont: bool /* bool */);
    pub fn wxcPrintEvent_SetPageLimits(self_: *u8 /* void* */, startPage: c_int /* int */, endPage: c_int /* int */, fromPage: c_int /* int */, toPage: c_int /* int */);
    pub fn wxInputStream_CanRead(self_: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxSize)
    pub fn wxSize_Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSize_Destroy(_obj: *u8 /* void* */);
    pub fn wxSize_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSize_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSize_SetHeight(_obj: *u8 /* void* */, h: c_int /* int */);
    pub fn wxSize_SetWidth(_obj: *u8 /* void* */, w: c_int /* int */);
    
    // TClassDefExtend(cbDynToolBarDimHandler,cbDimHandlerBase)
    pub fn cbDynToolBarDimHandler_Create() -> *u8 /* void* */;
    pub fn cbDynToolBarDimHandler_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(cbGCUpdatesMgr,cbSimpleUpdatesMgr)
    pub fn cbGCUpdatesMgr_Create(pPanel: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbGCUpdatesMgr_CreateDefault() -> *u8 /* void* */;
    pub fn cbGCUpdatesMgr_Delete(_obj: *u8 /* void* */);
    pub fn cbGCUpdatesMgr_UpdateNow(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxFileDataObject,wxDataObjectSimple)
    pub fn FileDataObject_AddFile(_obj: *u8 /* void* */, _fle: *u8 /* void* */);
    pub fn FileDataObject_Create(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    pub fn FileDataObject_Delete(_obj: *u8 /* void* */);
    pub fn FileDataObject_GetFilenames(_obj: *u8 /* void* */, _lst: *wchar_t /* wchar_t* */) -> c_int /* int */;
    
    // TClassDefExtend(wxGraphicsFont,wxGraphicsObject);
    
    // TClassDefExtend(wxDocParentFrame,wxFrame)
    
    // TClassDefExtend(wxSliderMSW,wxSlider)
    pub fn wxObject_Delete(obj: *u8 /* void* */);
    pub fn wxFrame_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrame_SetTitle(_frame: *u8 /* void* */, _txt: *u8 /* void* */);
    pub fn wxFrame_SetShape(self_: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFrame_ShowFullScreen(self_: *u8 /* void* */, show: bool /* bool */, style: c_int /* int */) -> bool /* bool */;
    pub fn wxFrame_IsFullScreen(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFrame_Centre(self_: *u8 /* void* */, orientation: c_int /* int */);
    pub fn wxCursor_Delete(_obj: *u8 /* void* */);
    pub fn wxDateTime_Delete(_obj: *u8 /* void* */);
    pub fn wxMouseEvent_GetWheelDelta(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMouseEvent_GetWheelRotation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMouseEvent_GetButton(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxcGetMousePosition() -> *u8 /* void* */;
    pub fn wxDC_GetUserScaleX(dc: *u8 /* void* */) -> c_double /* double */;
    pub fn wxDC_GetUserScaleY(dc: *u8 /* void* */) -> c_double /* double */;
    pub fn wxWindow_ConvertDialogToPixelsEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_ConvertPixelsToDialogEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindow_ScreenToClient2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxString_Create(buffer: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    pub fn wxString_CreateLen(buffer: *wchar_t /* wchar_t* */, len: c_int /* int */) -> *u8 /* void* */;
    pub fn wxString_Delete(s: *u8 /* void* */);
    pub fn wxString_GetString(s: *u8 /* void* */, buffer: *wchar_t /* wchar_t* */) -> c_int /* int */;
    pub fn wxString_Length(s: *u8 /* void* */) -> size_t /* size_t */;
    pub fn wxMenu_GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMenuBar_GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxListEvent_GetCacheFrom(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListEvent_GetCacheTo(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListCtrl_AssignImageList(_obj: *u8 /* void* */, images: *u8 /* void* */, which: c_int /* int */);
    pub fn wxListCtrl_GetColumn2(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */);
    pub fn wxListCtrl_GetItem2(_obj: *u8 /* void* */, info: *u8 /* void* */);
    pub fn wxListCtrl_GetItemPosition2(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListCtrl_SortItems2(_obj: *u8 /* void* */, closure: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxURL,wxObject)
    
    // TClassDefExtend(wxStyledTextCtrl,wxControl)
    pub fn wxStyledTextCtrl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxHtmlContainerCell,wxHtmlCell)
    
    // TClassDefExtend(wxFrameLayout,wxEvtHandler)
    pub fn wxFrameLayout_Activate(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_AddBar(_obj: *u8 /* void* */, pBarWnd: *u8 /* void* */, dimInfo: *u8 /* void* */, alignment: c_int /* int */, rowNo: c_int /* int */, columnPos: c_int /* int */, name: *wchar_t /* wchar_t* */, spyEvents: c_int /* int */, state: c_int /* int */);
    pub fn wxFrameLayout_AddPlugin(_obj: *u8 /* void* */, pPlInfo: *u8 /* void* */, paneMask: c_int /* int */);
    pub fn wxFrameLayout_AddPluginBefore(_obj: *u8 /* void* */, pNextPlInfo: *u8 /* void* */, pPlInfo: *u8 /* void* */, paneMask: c_int /* int */);
    pub fn wxFrameLayout_ApplyBarProperties(_obj: *u8 /* void* */, pBar: *u8 /* void* */);
    pub fn wxFrameLayout_CaptureEventsForPane(_obj: *u8 /* void* */, toPane: *u8 /* void* */);
    pub fn wxFrameLayout_CaptureEventsForPlugin(_obj: *u8 /* void* */, pPlugin: *u8 /* void* */);
    pub fn wxFrameLayout_Create(pParentFrame: *u8 /* void* */, pFrameClient: *u8 /* void* */, activateNow: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFrameLayout_Deactivate(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_Delete(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_DestroyBarWindows(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_EnableFloating(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxFrameLayout_FindBarByName(_obj: *u8 /* void* */, name: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_FindBarByWindow(_obj: *u8 /* void* */, pWnd: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_FindPlugin(_obj: *u8 /* void* */, pPlInfo: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_FirePluginEvent(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_GetBars(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFrameLayout_GetClientHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFrameLayout_GetClientRect(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxFrameLayout_GetClientWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFrameLayout_GetFrameClient(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_GetPane(_obj: *u8 /* void* */, alignment: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFrameLayout_GetPaneProperties(_obj: *u8 /* void* */, props: *u8 /* void* */, alignment: c_int /* int */);
    pub fn wxFrameLayout_GetParentFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_GetTopPlugin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_GetUpdatesManager(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFrameLayout_HasTopPlugin(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFrameLayout_HideBarWindows(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_InverseVisibility(_obj: *u8 /* void* */, pBar: *u8 /* void* */);
    pub fn wxFrameLayout_OnLButtonDown(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_OnLButtonUp(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_OnLDblClick(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_OnMouseMove(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_OnRButtonDown(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_OnRButtonUp(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_OnSize(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxFrameLayout_PopAllPlugins(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_PopPlugin(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_PushDefaultPlugins(_obj: *u8 /* void* */);
    pub fn wxFrameLayout_PushPlugin(_obj: *u8 /* void* */, pPugin: *u8 /* void* */);
    pub fn wxFrameLayout_RecalcLayout(_obj: *u8 /* void* */, repositionBarsNow: c_int /* int */);
    pub fn wxFrameLayout_RedockBar(_obj: *u8 /* void* */, pBar: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, pToPane: *u8 /* void* */, updateNow: c_int /* int */) -> c_int /* int */;
    pub fn wxFrameLayout_RefreshNow(_obj: *u8 /* void* */, recalcLayout: c_int /* int */);
    pub fn wxFrameLayout_ReleaseEventsFromPane(_obj: *u8 /* void* */, fromPane: *u8 /* void* */);
    pub fn wxFrameLayout_ReleaseEventsFromPlugin(_obj: *u8 /* void* */, pPlugin: *u8 /* void* */);
    pub fn wxFrameLayout_RemoveBar(_obj: *u8 /* void* */, pBar: *u8 /* void* */);
    pub fn wxFrameLayout_RemovePlugin(_obj: *u8 /* void* */, pPlInfo: *u8 /* void* */);
    pub fn wxFrameLayout_SetBarState(_obj: *u8 /* void* */, pBar: *u8 /* void* */, newStatem: c_int /* int */, updateNow: c_int /* int */);
    pub fn wxFrameLayout_SetFrameClient(_obj: *u8 /* void* */, pFrameClient: *u8 /* void* */);
    pub fn wxFrameLayout_SetMargins(_obj: *u8 /* void* */, top: c_int /* int */, bottom: c_int /* int */, left: c_int /* int */, right: c_int /* int */, paneMask: c_int /* int */);
    pub fn wxFrameLayout_SetPaneBackground(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxFrameLayout_SetPaneProperties(_obj: *u8 /* void* */, props: *u8 /* void* */, paneMask: c_int /* int */);
    pub fn wxFrameLayout_SetTopPlugin(_obj: *u8 /* void* */, pPlugin: *u8 /* void* */);
    pub fn wxFrameLayout_SetUpdatesManager(_obj: *u8 /* void* */, pUMgr: *u8 /* void* */);
    
    // TClassDefExtend(wxDDEServer,wxServerBase)
    
    // TClassDefExtend(cbDrawRowDecorEvent,cbPluginEvent)
    pub fn cbDrawRowDecorEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDrawRowDecorEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxcTreeItemData,wxTreeItemData)
    pub fn wxcTreeItemData_Create(closure: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcTreeItemData_GetClientClosure(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcTreeItemData_SetClientClosure(self_: *u8 /* void* */, closure: *u8 /* void* */);
    pub fn wxTreeItemId_Clone(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeItemId_CreateFromValue(value: intptr_t /* intptr_t */) -> *u8 /* void* */;
    pub fn wxTreeItemId_GetValue(_obj: *u8 /* void* */) -> intptr_t /* intptr_t */;
    pub fn wxTreeEvent_GetKeyEvent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeEvent_IsEditCancelled(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeEvent_Allow(_obj: *u8 /* void* */);
    pub fn wxTreeCtrl_Create2(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_InsertItem2(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_InsertItemByIndex2(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */);
    pub fn wxTreeCtrl_GetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeCtrl_SetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */, closure: *u8 /* void* */);
    pub fn wxTreeCtrl_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    pub fn wxTreeCtrl_AssignStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    pub fn wxDC_GetPixel2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */);
    pub fn wxScrolledWindow_SetScrollRate(_obj: *u8 /* void* */, xstep: c_int /* int */, ystep: c_int /* int */);
    
    // TClassDefExtend(wxEvtHandler,wxObject)
    pub fn wxEvtHandler_AddPendingEvent(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxEvtHandler_Connect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, data: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEvtHandler_Create() -> *u8 /* void* */;
    pub fn wxEvtHandler_Delete(_obj: *u8 /* void* */);
    pub fn wxEvtHandler_Disconnect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, id: c_int /* int */) -> c_int /* int */;
    pub fn wxEvtHandler_GetEvtHandlerEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxEvtHandler_GetNextHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvtHandler_GetPreviousHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvtHandler_ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */;
    pub fn wxEvtHandler_ProcessPendingEvents(_obj: *u8 /* void* */);
    pub fn wxEvtHandler_SetEvtHandlerEnabled(_obj: *u8 /* void* */, enabled: bool /* bool */);
    pub fn wxEvtHandler_SetNextHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    pub fn wxEvtHandler_SetPreviousHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    
    // TClassDefExtend(wxSpinButton,wxControl)
    pub fn wxSpinButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    pub fn wxSpinButton_GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinButton_GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinButton_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinButton_SetRange(_obj: *u8 /* void* */, minVal: c_int /* int */, maxVal: c_int /* int */);
    pub fn wxSpinButton_SetValue(_obj: *u8 /* void* */, val: c_int /* int */);
    
    // TClassDefExtend(ELJDropTarget,wxDropTarget)
    pub fn ELJDropTarget_Create(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJDropTarget_Delete(_obj: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    
    // TClassDefExtend(wxFindReplaceDialog,wxDialog)
    pub fn wxFindReplaceDialog_Create(parent: *u8 /* void* */, data: *u8 /* void* */, title: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFindReplaceDialog_GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFindReplaceDialog_SetData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    
    // TClassDefExtend(wxFileInputStream,wxInputStream)
    
    // TClassDefExtend(wxIconizeEvent,wxEvent)
    
    // TClassDefExtend(wxShowEvent,wxEvent)
    pub fn wxShowEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxShowEvent_IsShown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxShowEvent_SetShow(_obj: *u8 /* void* */, show: bool /* bool */);
    
    // TClassDefExtend(cbResizeBarEvent,cbPluginEvent)
    pub fn cbResizeBarEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbResizeBarEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxHtmlTag,wxObject)
    
    // TClassDefExtend(wxMemoryDC,wxDC)
    pub fn wxMemoryDC_Create() -> *u8 /* void* */;
    pub fn wxMemoryDC_CreateCompatible(dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMemoryDC_CreateWithBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMemoryDC_Delete(_obj: *u8 /* void* */);
    pub fn wxMemoryDC_SelectObject(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    
    // TClassDefExtend(wxStringTokenizer,wxObject)
    
    // TClassDefExtend(wxFileSystem,wxObject)
    
    // TClassDefExtend(wxSlider95,wxSlider)
    
    // TClassDefExtend(wxPen,wxGDIObject)
    pub fn wxPen_Assign(_obj: *u8 /* void* */, pen: *u8 /* void* */);
    pub fn wxPen_CreateDefault() -> *u8 /* void* */;
    pub fn wxPen_CreateFromBitmap(stipple: *u8 /* void* */, width: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPen_CreateFromColour(col: *u8 /* void* */, width: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPen_CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPen_Delete(_obj: *u8 /* void* */);
    pub fn wxPen_GetCap(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPen_GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPen_GetDashes(_obj: *u8 /* void* */, ptr: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPen_GetJoin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPen_GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPen_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPen_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPen_IsEqual(_obj: *u8 /* void* */, pen: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPen_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPen_SetCap(_obj: *u8 /* void* */, cap: c_int /* int */);
    pub fn wxPen_SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxPen_SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */);
    pub fn wxPen_SetDashes(_obj: *u8 /* void* */, nb_dashes: c_int /* int */, dash: *u8 /* void* */);
    pub fn wxPen_SetJoin(_obj: *u8 /* void* */, join: c_int /* int */);
    pub fn wxPen_SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */);
    pub fn wxPen_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    pub fn wxPen_SetWidth(_obj: *u8 /* void* */, width: c_int /* int */);
    
    // TClassDefExtend(wxArtProvider,wxObject)
    pub fn PopProvider() -> bool /* bool */;
    pub fn PushProvider(provider: *u8 /* void* */);
    pub fn RemoveProvider(provider: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxLongLong)
    
    // TClassDefExtend(wxTreeItemData,wxClientData)
    
    // TClassDefExtend(wxListBox,wxControl)
    pub fn wxListBox_Append(_obj: *u8 /* void* */, item: *u8 /* void* */);
    pub fn wxListBox_AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxListBox_Clear(_obj: *u8 /* void* */);
    pub fn wxListBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListBox_Delete(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxListBox_FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListBox_GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListBox_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListBox_GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxListBox_GetSelections(_obj: *u8 /* void* */, aSelections: *c_int /* int* */, allocated: c_int /* int */) -> c_int /* int */;
    pub fn wxListBox_GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    pub fn wxListBox_InsertItems(_obj: *u8 /* void* */, items: *u8 /* void* */, pos: c_int /* int */, count: c_int /* int */);
    pub fn wxListBox_IsSelected(_obj: *u8 /* void* */, n: c_int /* int */) -> bool /* bool */;
    pub fn wxListBox_SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */);
    pub fn wxListBox_SetFirstItem(_obj: *u8 /* void* */, n: c_int /* int */);
    pub fn wxListBox_SetSelection(_obj: *u8 /* void* */, n: c_int /* int */, select: c_int /* int */);
    pub fn wxListBox_SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */);
    pub fn wxListBox_SetStringSelection(_obj: *u8 /* void* */, str: *u8 /* void* */, sel: bool /* bool */);
    
    // TClassDef(wxSemaphore)
    
    // TClassDefExtend(wxFileDropTarget,wxDropTarget)
    
    // TClassDef(wxMutexLocker)
    
    // TClassDef(wxLog)
    
    // TClassDefExtend(wxGraphicsPen,wxGraphicsObject);
    
    // TClassDefExtend(wxTipWindow,wxPopupTransientWindow)
    pub fn wxTipWindow_Close(_obj: *u8 /* void* */);
    pub fn wxTipWindow_Create(parent: *u8 /* void* */, text: *u8 /* void* */, maxLength: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTipWindow_SetBoundingRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxTipWindow_SetTipWindowPtr(_obj: *u8 /* void* */, windowPtr: *u8 /* void* */);
    
    // TClassDefExtend(cbRightDownEvent,cbPluginEvent)
    pub fn cbRightDownEvent_Pos(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
    // TClassDefExtend(wxSplitterEvent,wxNotifyEvent)
    
    // TClassDefExtend(ELJFileDropTarget,wxFileDropTarget)
    pub fn ELJFileDropTarget_Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJFileDropTarget_Delete(_obj: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    
    // TClassDefExtend(wxTreeLayout,wxObject)
    
    // TClassDef(wxMemoryBuffer)
    pub fn wxStyledTextCtrl_IndicatorGetForeground(_obj: *u8 /* void* */, indic: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetCaretLineBackground(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_SetCaretLineBackground(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxStyledTextCtrl_GetCaretForeground(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetLine(_obj: *u8 /* void* */, line: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetTextRange(_obj: *u8 /* void* */, startPos: c_int /* int */, endPos: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetSelectedText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_CreateDocument(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetEdgeColour(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_GetDocPointer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStyledTextCtrl_PointFromPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(cbSimpleCustomizationPlugin,cbPluginBase)
    pub fn cbSimpleCustomizationPlugin_Create(pPanel: *u8 /* void* */, paneMask: c_int /* int */) -> *u8 /* void* */;
    pub fn cbSimpleCustomizationPlugin_CreateDefault() -> *u8 /* void* */;
    pub fn cbSimpleCustomizationPlugin_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJServer,wxServer)
    pub fn ELJServer_Create(_eobj: *u8 /* void* */, _cnct: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJServer_Delete(_obj: *u8 /* void* */);
    pub fn ELJServer_Initialize(_obj: *u8 /* void* */, name: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxMemoryFSHandler,wxFileSystemHandler)
    
    // TClassDefExtend(cbDrawBarHandlesEvent,cbPluginEvent)
    pub fn cbDrawBarHandlesEvent_Bar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn cbDrawBarHandlesEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxGridCellAttr)
    pub fn wxGridCellAttr_Ctor() -> *u8 /* void* */;
    pub fn wxGridCellAttr_DecRef(_obj: *u8 /* void* */);
    pub fn wxGridCellAttr_GetAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGridCellAttr_GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGridCellAttr_GetEditor(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGridCellAttr_GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGridCellAttr_GetRenderer(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGridCellAttr_GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGridCellAttr_HasAlignment(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_HasEditor(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_HasFont(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_HasRenderer(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_IncRef(_obj: *u8 /* void* */);
    pub fn wxGridCellAttr_IsReadOnly(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridCellAttr_SetAlignment(_obj: *u8 /* void* */, hAlign: c_int /* int */, vAlign: c_int /* int */);
    pub fn wxGridCellAttr_SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */);
    pub fn wxGridCellAttr_SetDefAttr(_obj: *u8 /* void* */, defAttr: *u8 /* void* */);
    pub fn wxGridCellAttr_SetEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */);
    pub fn wxGridCellAttr_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxGridCellAttr_SetReadOnly(_obj: *u8 /* void* */, isReadOnly: bool /* bool */);
    pub fn wxGridCellAttr_SetRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */);
    pub fn wxGridCellAttr_SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */);
    
    // TClassDefExtend(wxCalculateLayoutEvent,wxEvent)
    pub fn wxCalculateLayoutEvent_Create(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCalculateLayoutEvent_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCalculateLayoutEvent_GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCalculateLayoutEvent_SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxCalculateLayoutEvent_SetRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    
    // TClassDef(wxBusyCursor)
    pub fn wxBusyCursor_Create() -> *u8 /* void* */;
    pub fn wxBusyCursor_CreateWithCursor(_cur: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBusyCursor_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxJoystick,wxObject)
    pub fn wxJoystick_Create(joystick: c_int /* int */) -> *u8 /* void* */;
    pub fn wxJoystick_Delete(_obj: *u8 /* void* */);
    pub fn wxJoystick_GetButtonState(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetManufacturerId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetMaxAxes(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetMaxButtons(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetMovementThreshold(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetNumberAxes(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetNumberButtons(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetNumberJoysticks(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetPOVCTSPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetPOVPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetPollingMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetPollingMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxJoystick_GetProductId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetProductName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxJoystick_GetRudderMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetRudderMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetRudderPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetUMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetUMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetUPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetVMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetVMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetVPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetXMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetXMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetYMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetYMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetZMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetZMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_GetZPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_HasPOV(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_HasPOV4Dir(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_HasPOVCTS(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_HasRudder(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_HasU(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_HasV(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_HasZ(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxJoystick_ReleaseCapture(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxJoystick_SetCapture(_obj: *u8 /* void* */, win: *u8 /* void* */, pollingFreq: c_int /* int */) -> c_int /* int */;
    pub fn wxJoystick_SetMovementThreshold(_obj: *u8 /* void* */, threshold: c_int /* int */);
    
    // TClassDefExtend(wxPrintDialogData,wxObject)
    pub fn wxPrintDialogData_Assign(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxPrintDialogData_AssignData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxPrintDialogData_CreateDefault() -> *u8 /* void* */;
    pub fn wxPrintDialogData_CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintDialogData_Delete(_obj: *u8 /* void* */);
    pub fn wxPrintDialogData_EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_EnablePageNumbers(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_EnablePrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_EnableSelection(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_GetAllPages(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintDialogData_GetCollate(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetEnablePageNumbers(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetEnablePrintToFile(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetEnableSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetFromPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintDialogData_GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintDialogData_GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintDialogData_GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintDialogData_GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPrintDialogData_GetPrintToFile(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPrintDialogData_GetToPage(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPrintDialogData_SetAllPages(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_SetCollate(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_SetFromPage(_obj: *u8 /* void* */, v: c_int /* int */);
    pub fn wxPrintDialogData_SetMaxPage(_obj: *u8 /* void* */, v: c_int /* int */);
    pub fn wxPrintDialogData_SetMinPage(_obj: *u8 /* void* */, v: c_int /* int */);
    pub fn wxPrintDialogData_SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */);
    pub fn wxPrintDialogData_SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */);
    pub fn wxPrintDialogData_SetPrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_SetSelection(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxPrintDialogData_SetToPage(_obj: *u8 /* void* */, v: c_int /* int */);
    
    // TClassDefExtend(wxBrushList,wxList)
    
    // TClassDef(wxTipProvider)
    
    // TClassDefExtend(wxImage,wxObject)
    pub fn wxImage_CanRead(name: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImage_ConvertToBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    pub fn wxImage_ConvertToByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */;
    pub fn wxImage_ConvertToLazyByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */;
    pub fn wxImage_CountColours(_obj: *u8 /* void* */, stopafter: c_int /* int */) -> c_int /* int */;
    pub fn wxImage_CreateDefault() -> *u8 /* void* */;
    pub fn wxImage_CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxImage_CreateFromByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxImage_CreateFromLazyByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxImage_CreateFromData(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxImage_CreateFromFile(name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxImage_CreateSized(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxImage_Destroy(_obj: *u8 /* void* */);
    pub fn wxImage_GetBlue(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */;
    pub fn wxImage_GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxImage_GetGreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */;
    pub fn wxImage_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxImage_GetMaskBlue(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    pub fn wxImage_GetMaskGreen(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    pub fn wxImage_GetMaskRed(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    pub fn wxImage_GetRed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */;
    pub fn wxImage_GetSubImage(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, image: *u8 /* void* */);
    pub fn wxImage_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxImage_HasMask(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImage_GetOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxImage_GetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImage_HasOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImage_Initialize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxImage_InitializeFromData(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */);
    pub fn wxImage_LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */;
    pub fn wxImage_Mirror(_obj: *u8 /* void* */, horizontally: c_int /* int */, image: *u8 /* void* */);
    pub fn wxImage_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxImage_Paste(_obj: *u8 /* void* */, image: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxImage_Replace(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */, arg5: u8 /* u8 */);
    pub fn wxImage_Rescale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxImage_Rotate(_obj: *u8 /* void* */, angle: c_double /* double */, arg0: c_int /* int */, arg1: c_int /* int */, interpolating: c_int /* int */, offset_after_rotation: *u8 /* void* */, image: *u8 /* void* */);
    pub fn wxImage_Rotate90(_obj: *u8 /* void* */, clockwise: c_int /* int */, image: *u8 /* void* */);
    pub fn wxImage_SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */;
    pub fn wxImage_Scale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, image: *u8 /* void* */);
    pub fn wxImage_SetData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    pub fn wxImage_SetDataAndSize(_obj: *u8 /* void* */, data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxImage_SetMask(_obj: *u8 /* void* */, mask: c_int /* int */);
    pub fn wxImage_SetMaskColour(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    pub fn wxImage_SetOption(_obj: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */);
    pub fn wxImage_SetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */);
    pub fn wxImage_SetRGB(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */);
    
    // TClassDefExtend(wxFileHistory,wxObject)
    pub fn wxFileHistory_AddFileToHistory(_obj: *u8 /* void* */, file: *u8 /* void* */);
    pub fn wxFileHistory_AddFilesToMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    pub fn wxFileHistory_Create(maxFiles: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFileHistory_Delete(_obj: *u8 /* void* */);
    pub fn wxFileHistory_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileHistory_GetHistoryFile(_obj: *u8 /* void* */, i: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFileHistory_GetMaxFiles(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileHistory_GetMenus(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFileHistory_Load(_obj: *u8 /* void* */, config: *u8 /* void* */);
    pub fn wxFileHistory_RemoveFileFromHistory(_obj: *u8 /* void* */, i: c_int /* int */);
    pub fn wxFileHistory_RemoveMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    pub fn wxFileHistory_Save(_obj: *u8 /* void* */, config: *u8 /* void* */);
    pub fn wxFileHistory_UseMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    
    // TClassDefExtend(wxGraphicsMatrix,wxGraphicsObject);
    
    // TClassDefExtend(wxLogNull,wxLog)
    
    // TClassDefExtend(wxNewBitmapButton,wxPanel)
    pub fn wxNewBitmapButton_Create(labelBitmap: *u8 /* void* */, labelText: *u8 /* void* */, alignText: c_int /* int */, isFlat: bool /* bool */, firedEventType: c_int /* int */, marginX: c_int /* int */, marginY: c_int /* int */, textToLabelGap: c_int /* int */, isSticky: bool /* bool */) -> *u8 /* void* */;
    pub fn wxNewBitmapButton_CreateFromFile(bitmapFileName: *u8 /* void* */, bitmapFileType: c_int /* int */, labelText: *u8 /* void* */, alignText: c_int /* int */, isFlat: bool /* bool */, firedEventType: c_int /* int */, marginX: c_int /* int */, marginY: c_int /* int */, textToLabelGap: c_int /* int */, isSticky: bool /* bool */) -> *u8 /* void* */;
    pub fn wxNewBitmapButton_Delete(_obj: *u8 /* void* */);
    pub fn wxNewBitmapButton_DrawDecorations(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxNewBitmapButton_DrawLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxNewBitmapButton_Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> c_int /* int */;
    pub fn wxNewBitmapButton_Realize(_obj: *u8 /* void* */, _prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxNewBitmapButton_RenderAllLabelImages(_obj: *u8 /* void* */);
    pub fn wxNewBitmapButton_RenderLabelImage(_obj: *u8 /* void* */, destBmp: *u8 /* void* */, srcBmp: *u8 /* void* */, isEnabled: bool /* bool */, isPressed: bool /* bool */);
    pub fn wxNewBitmapButton_RenderLabelImages(_obj: *u8 /* void* */);
    pub fn wxNewBitmapButton_Reshape(_obj: *u8 /* void* */);
    pub fn wxNewBitmapButton_SetAlignments(_obj: *u8 /* void* */, alignText: c_int /* int */, marginX: c_int /* int */, marginY: c_int /* int */, textToLabelGap: c_int /* int */);
    pub fn wxNewBitmapButton_SetLabel(_obj: *u8 /* void* */, labelBitmap: *u8 /* void* */, labelText: *u8 /* void* */);
    
    // TClassDefExtend(wxEvent,wxObject)
    pub fn wxEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxEvent_GetEventObject(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvent_GetEventType(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEvent_GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEvent_GetSkipped(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxEvent_GetTimestamp(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEvent_IsCommandEvent(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxEvent_NewEventType() -> c_int /* int */;
    pub fn wxEvent_SetEventObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxEvent_SetEventType(_obj: *u8 /* void* */, typ: c_int /* int */);
    pub fn wxEvent_SetId(_obj: *u8 /* void* */, Id: c_int /* int */);
    pub fn wxEvent_SetTimestamp(_obj: *u8 /* void* */, ts: c_int /* int */);
    pub fn wxEvent_Skip(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxTimerBase,wxObject)
    
    // TClassDefExtend(wxGridCellBoolRenderer,wxGridCellRenderer)
    
    // TClassDefExtend(wxMemoryInputStream,wxInputStream)
    
    // TClassDefExtend(wxGridEvent,wxNotifyEvent)
    pub fn wxGridEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_GetCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridEvent_GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_Selecting(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxFFileInputStream,wxInputStream)
    
    // TClassDefExtend(wxClipboard,wxObject)
    pub fn wxClipboard_AddData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_Clear(_obj: *u8 /* void* */);
    pub fn wxClipboard_Close(_obj: *u8 /* void* */);
    pub fn wxClipboard_Create() -> *u8 /* void* */;
    pub fn wxClipboard_Flush(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_GetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_IsOpened(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_IsSupported(_obj: *u8 /* void* */, format: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_Open(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */;
    pub fn wxClipboard_UsePrimarySelection(_obj: *u8 /* void* */, primary: bool /* bool */);
    
    // TClassDefExtend(wxStaticBox,wxControl)
    pub fn wxStaticBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxMutex)
    pub fn wxMutex_Create() -> *u8 /* void* */;
    pub fn wxMutex_Delete(_obj: *u8 /* void* */);
    pub fn wxMutex_IsLocked(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMutex_Lock(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMutex_TryLock(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMutex_Unlock(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDef(wxStreamToTextRedirector)
    
    // TClassDefExtend(wxModule,wxObject)
    
    // TClassDefExtend(wxPenList,wxList)
    
    // TClassDefExtend(wxHtmlPrintout,wxPrintout)
    
    // TClassDef(wxConfigBase)
    pub fn wxConfigBase_Create() -> *u8 /* void* */;
    pub fn wxConfigBase_Delete(_obj: *u8 /* void* */);
    pub fn wxConfigBase_DeleteAll(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_DeleteEntry(_obj: *u8 /* void* */, key: *u8 /* void* */, bDeleteGroupIfEmpty: bool /* bool */) -> bool /* bool */;
    pub fn wxConfigBase_DeleteGroup(_obj: *u8 /* void* */, key: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_Exists(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_ExpandEnvVars(_obj: *u8 /* void* */, str: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_Flush(_obj: *u8 /* void* */, bCurrentOnly: bool /* bool */) -> bool /* bool */;
    pub fn wxConfigBase_GetAppName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_GetEntryType(_obj: *u8 /* void* */, name: *u8 /* void* */) -> c_int /* int */;
    pub fn wxConfigBase_GetFirstEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_GetFirstGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_GetNextEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_GetNextGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_GetNumberOfEntries(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */;
    pub fn wxConfigBase_GetNumberOfGroups(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */;
    pub fn wxConfigBase_GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxConfigBase_GetVendorName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_HasEntry(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_HasGroup(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_IsExpandingEnvVars(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_IsRecordingDefaults(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_ReadBool(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: bool /* bool */) -> bool /* bool */;
    pub fn wxConfigBase_ReadDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_double /* double */) -> c_double /* double */;
    pub fn wxConfigBase_ReadInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_int /* int */) -> c_int /* int */;
    pub fn wxConfigBase_ReadString(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxConfigBase_RenameEntry(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_RenameGroup(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */;
    pub fn wxConfigBase_SetAppName(_obj: *u8 /* void* */, appName: *u8 /* void* */);
    pub fn wxConfigBase_SetExpandEnvVars(_obj: *u8 /* void* */, bDoIt: bool /* bool */);
    pub fn wxConfigBase_SetPath(_obj: *u8 /* void* */, strPath: *u8 /* void* */);
    pub fn wxConfigBase_SetRecordDefaults(_obj: *u8 /* void* */, bDoIt: bool /* bool */);
    pub fn wxConfigBase_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    pub fn wxConfigBase_SetVendorName(_obj: *u8 /* void* */, vendorName: *u8 /* void* */);
    pub fn wxConfigBase_WriteBool(_obj: *u8 /* void* */, key: *u8 /* void* */, value: bool /* bool */) -> bool /* bool */;
    pub fn wxConfigBase_WriteDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_double /* double */) -> bool /* bool */;
    pub fn wxConfigBase_WriteInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */;
    pub fn wxConfigBase_WriteLong(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_long /* long */) -> bool /* bool */;
    pub fn wxConfigBase_WriteString(_obj: *u8 /* void* */, key: *u8 /* void* */, value: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxDataInputStream)
    
    // TClassDefExtend(wxGridCellFloatEditor,wxGridCellTextEditor)
    pub fn wxGridCellFloatEditor_Ctor(width: c_int /* int */, precision: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxServer,wxServerBase)
    
    // TClassDefExtend(wxScrollBar,wxControl)
    pub fn wxScrollBar_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxScrollBar_GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_GetRange(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_GetThumbPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_GetThumbSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_SetScrollbar(_obj: *u8 /* void* */, position: c_int /* int */, thumbSize: c_int /* int */, range: c_int /* int */, pageSize: c_int /* int */, refresh: bool /* bool */);
    pub fn wxScrollBar_SetThumbPosition(_obj: *u8 /* void* */, viewStart: c_int /* int */);
    
    // TClassDefExtend(wxHtmlLinkInfo,wxObject)
    
    // TClassDefExtend(wxSimpleHelpProvider,wxHelpProvider)
    pub fn wxSimpleHelpProvider_Create() -> *u8 /* void* */;
    
    // TClassDefExtend(wxGrid,wxScrolledWindow)
    pub fn wxGrid_AppendCols(_obj: *u8 /* void* */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_AppendRows(_obj: *u8 /* void* */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_AutoSize(_obj: *u8 /* void* */);
    pub fn wxGrid_AutoSizeColumn(_obj: *u8 /* void* */, col: c_int /* int */, setAsMin: c_int /* int */);
    pub fn wxGrid_AutoSizeColumns(_obj: *u8 /* void* */, setAsMin: c_int /* int */);
    pub fn wxGrid_AutoSizeRow(_obj: *u8 /* void* */, row: c_int /* int */, setAsMin: c_int /* int */);
    pub fn wxGrid_AutoSizeRows(_obj: *u8 /* void* */, setAsMin: c_int /* int */);
    pub fn wxGrid_BeginBatch(_obj: *u8 /* void* */);
    pub fn wxGrid_BlockToDeviceRect(_obj: *u8 /* void* */, top: c_int /* int */, left: c_int /* int */, bottom: c_int /* int */, right: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_CanDragColSize(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_CanDragGridSize(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_CanDragRowSize(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_CanEnableCellControl(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_CellToRect(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_ClearGrid(_obj: *u8 /* void* */);
    pub fn wxGrid_ClearSelection(_obj: *u8 /* void* */);
    pub fn wxGrid_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_CreateGrid(_obj: *u8 /* void* */, rows: c_int /* int */, cols: c_int /* int */, selmode: c_int /* int */);
    pub fn wxGrid_DeleteCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_DeleteRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_DisableCellEditControl(_obj: *u8 /* void* */);
    pub fn wxGrid_DisableDragColSize(_obj: *u8 /* void* */);
    pub fn wxGrid_DisableDragGridSize(_obj: *u8 /* void* */);
    pub fn wxGrid_DisableDragRowSize(_obj: *u8 /* void* */);
    pub fn wxGrid_DrawAllGridLines(_obj: *u8 /* void* */, dc: *u8 /* void* */, reg: *u8 /* void* */);
    pub fn wxGrid_DrawCell(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */);
    pub fn wxGrid_DrawCellBorder(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */);
    pub fn wxGrid_DrawCellHighlight(_obj: *u8 /* void* */, dc: *u8 /* void* */, attr: *u8 /* void* */);
    pub fn wxGrid_DrawColLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, col: c_int /* int */);
    pub fn wxGrid_DrawColLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxGrid_DrawGridSpace(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxGrid_DrawRowLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, row: c_int /* int */);
    pub fn wxGrid_DrawRowLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    pub fn wxGrid_DrawTextRectangle(_obj: *u8 /* void* */, dc: *u8 /* void* */, txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, horizontalAlignment: c_int /* int */, verticalAlignment: c_int /* int */);
    pub fn wxGrid_EnableCellEditControl(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxGrid_EnableDragColSize(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxGrid_EnableDragGridSize(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxGrid_EnableDragRowSize(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxGrid_EnableEditing(_obj: *u8 /* void* */, edit: c_int /* int */);
    pub fn wxGrid_EnableGridLines(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxGrid_EndBatch(_obj: *u8 /* void* */);
    pub fn wxGrid_GetBatchCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGrid_GetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    pub fn wxGrid_GetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */);
    pub fn wxGrid_GetCellHighlightColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    pub fn wxGrid_GetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetColLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGrid_GetColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetColSize(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */;
    pub fn wxGrid_GetDefaultCellAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGrid_GetDefaultCellBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetDefaultCellFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetDefaultCellTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetDefaultColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetDefaultColSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetDefaultEditor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGrid_GetDefaultEditorForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetDefaultEditorForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGrid_GetDefaultRenderer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGrid_GetDefaultRendererForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetDefaultRendererForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGrid_GetDefaultRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetDefaultRowSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetGridCursorCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetGridCursorRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetGridLineColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetLabelBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetLabelFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetLabelTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetNumberCols(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetNumberRows(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetRowLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGrid_GetRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_GetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */) -> *u8 /* void* */;
    pub fn wxGrid_GetRowSize(_obj: *u8 /* void* */, row: c_int /* int */) -> c_int /* int */;
    pub fn wxGrid_GetSelectionBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetSelectionForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxGrid_GetTable(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGrid_GetTextBoxSize(_obj: *u8 /* void* */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxGrid_GridLinesEnabled(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_HideCellEditControl(_obj: *u8 /* void* */);
    pub fn wxGrid_InsertCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_InsertRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_IsCellEditControlEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_IsCellEditControlShown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_IsCurrentCellReadOnly(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_IsEditable(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_IsInSelection(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */;
    pub fn wxGrid_IsReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */;
    pub fn wxGrid_IsSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_IsVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, wholeCellVisible: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MakeCellVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */);
    pub fn wxGrid_MoveCursorDown(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorDownBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorLeft(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorLeftBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorRight(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorRightBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorUp(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MoveCursorUpBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    pub fn wxGrid_MovePageDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_MovePageUp(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_ProcessTableMessage(_obj: *u8 /* void* */, evt: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGrid_RegisterDataType(_obj: *u8 /* void* */, typeName: *u8 /* void* */, renderer: *u8 /* void* */, editor: *u8 /* void* */);
    pub fn wxGrid_SaveEditControlValue(_obj: *u8 /* void* */);
    pub fn wxGrid_SelectAll(_obj: *u8 /* void* */);
    pub fn wxGrid_SelectBlock(_obj: *u8 /* void* */, topRow: c_int /* int */, leftCol: c_int /* int */, bottomRow: c_int /* int */, rightCol: c_int /* int */, addToSelected: c_int /* int */);
    pub fn wxGrid_SelectCol(_obj: *u8 /* void* */, col: c_int /* int */, addToSelected: c_int /* int */);
    pub fn wxGrid_SelectRow(_obj: *u8 /* void* */, row: c_int /* int */, addToSelected: c_int /* int */);
    pub fn wxGrid_SetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, horiz: c_int /* int */, vert: c_int /* int */);
    pub fn wxGrid_SetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    pub fn wxGrid_SetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, editor: *u8 /* void* */);
    pub fn wxGrid_SetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */);
    pub fn wxGrid_SetCellHighlightColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxGrid_SetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, renderer: *u8 /* void* */);
    pub fn wxGrid_SetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    pub fn wxGrid_SetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, s: *u8 /* void* */);
    pub fn wxGrid_SetColAttr(_obj: *u8 /* void* */, col: c_int /* int */, attr: *u8 /* void* */);
    pub fn wxGrid_SetColFormatBool(_obj: *u8 /* void* */, col: c_int /* int */);
    pub fn wxGrid_SetColFormatCustom(_obj: *u8 /* void* */, col: c_int /* int */, typeName: *u8 /* void* */);
    pub fn wxGrid_SetColFormatFloat(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */, precision: c_int /* int */);
    pub fn wxGrid_SetColFormatNumber(_obj: *u8 /* void* */, col: c_int /* int */);
    pub fn wxGrid_SetColLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */);
    pub fn wxGrid_SetColLabelSize(_obj: *u8 /* void* */, height: c_int /* int */);
    pub fn wxGrid_SetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */, label: *u8 /* void* */);
    pub fn wxGrid_SetColMinimalWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */);
    pub fn wxGrid_SetColSize(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */);
    pub fn wxGrid_SetDefaultCellAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */);
    pub fn wxGrid_SetDefaultCellBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxGrid_SetDefaultCellFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxGrid_SetDefaultCellTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxGrid_SetDefaultColSize(_obj: *u8 /* void* */, width: c_int /* int */, resizeExistingCols: c_int /* int */);
    pub fn wxGrid_SetDefaultEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */);
    pub fn wxGrid_SetDefaultRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */);
    pub fn wxGrid_SetDefaultRowSize(_obj: *u8 /* void* */, height: c_int /* int */, resizeExistingRows: c_int /* int */);
    pub fn wxGrid_SetGridCursor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */);
    pub fn wxGrid_SetGridLineColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    pub fn wxGrid_SetLabelBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxGrid_SetLabelFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    pub fn wxGrid_SetLabelTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxGrid_SetMargins(_obj: *u8 /* void* */, extraWidth: c_int /* int */, extraHeight: c_int /* int */);
    pub fn wxGrid_SetReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, isReadOnly: bool /* bool */);
    pub fn wxGrid_SetRowAttr(_obj: *u8 /* void* */, row: c_int /* int */, attr: *u8 /* void* */);
    pub fn wxGrid_SetRowLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */);
    pub fn wxGrid_SetRowLabelSize(_obj: *u8 /* void* */, width: c_int /* int */);
    pub fn wxGrid_SetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */, label: *u8 /* void* */);
    pub fn wxGrid_SetRowMinimalHeight(_obj: *u8 /* void* */, row: c_int /* int */, width: c_int /* int */);
    pub fn wxGrid_SetRowSize(_obj: *u8 /* void* */, row: c_int /* int */, height: c_int /* int */);
    pub fn wxGrid_SetSelectionBackground(_obj: *u8 /* void* */, c: *u8 /* void* */);
    pub fn wxGrid_SetSelectionForeground(_obj: *u8 /* void* */, c: *u8 /* void* */);
    pub fn wxGrid_SetSelectionMode(_obj: *u8 /* void* */, selmode: c_int /* int */);
    pub fn wxGrid_SetTable(_obj: *u8 /* void* */, table: *u8 /* void* */, takeOwnership: bool /* bool */, selmode: c_int /* int */) -> bool /* bool */;
    pub fn wxGrid_ShowCellEditControl(_obj: *u8 /* void* */);
    pub fn wxGrid_StringToLines(_obj: *u8 /* void* */, value: *u8 /* void* */, lines: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGrid_XToCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    pub fn wxGrid_XToEdgeOfCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    pub fn wxGrid_XYToCell(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    pub fn wxGrid_YToEdgeOfRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxGrid_YToRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    pub fn wxGrid_GetSelectedCells(_obj: *u8 /* void* */, _arr: *u8 /* void* */);
    pub fn wxGrid_GetSelectionBlockTopLeft(_obj: *u8 /* void* */, _arr: *u8 /* void* */);
    pub fn wxGrid_GetSelectionBlockBottomRight(_obj: *u8 /* void* */, _arr: *u8 /* void* */);
    pub fn wxGrid_GetSelectedRows(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */;
    pub fn wxGrid_GetSelectedCols(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */;
    pub fn wxGrid_GetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    pub fn wxGrid_SetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(wxHtmlFilter,wxObject)
    
    // TClassDefExtend(cbDimHandlerBase,wxObject)
    
    // TClassDefExtend(cbBarInfo,wxObject)
    pub fn cbBarInfo_Create() -> *u8 /* void* */;
    pub fn cbBarInfo_Delete(_obj: *u8 /* void* */);
    pub fn cbBarInfo_IsExpanded(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn cbBarInfo_IsFixed(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxPropertyGrid,wxControl)
    pub fn wxPropertyGrid_Append(_obj: *u8 /* void* */, prop: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPropertyGrid_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPropertyGrid_DisableProperty(_obj: *u8 /* void* */, propName: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxGridCellBoolEditor,wxGridCellEditor)
    pub fn wxGridCellBoolEditor_Ctor() -> *u8 /* void* */;
    
    // TClassDefExtend(wxCalendarCtrl,wxControl)
    pub fn wxCalendarCtrl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _dat: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCalendarCtrl_EnableHolidayDisplay(_obj: *u8 /* void* */, display: c_int /* int */);
    pub fn wxCalendarCtrl_EnableMonthChange(_obj: *u8 /* void* */, enable: bool /* bool */);
    pub fn wxCalendarCtrl_GetAttr(_obj: *u8 /* void* */, day: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCalendarCtrl_GetDate(_obj: *u8 /* void* */, date: *u8 /* void* */);
    pub fn wxCalendarCtrl_GetHeaderColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarCtrl_GetHeaderColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarCtrl_GetHighlightColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarCtrl_GetHighlightColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarCtrl_GetHolidayColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarCtrl_GetHolidayColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxCalendarCtrl_HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, date: *u8 /* void* */, wd: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCalendarCtrl_ResetAttr(_obj: *u8 /* void* */, day: c_int /* int */);
    pub fn wxCalendarCtrl_SetAttr(_obj: *u8 /* void* */, day: c_int /* int */, attr: *u8 /* void* */);
    pub fn wxCalendarCtrl_SetDate(_obj: *u8 /* void* */, date: *u8 /* void* */);
    pub fn wxCalendarCtrl_SetHeaderColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */);
    pub fn wxCalendarCtrl_SetHighlightColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */);
    pub fn wxCalendarCtrl_SetHoliday(_obj: *u8 /* void* */, day: c_int /* int */);
    pub fn wxCalendarCtrl_SetHolidayColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */);
    
    // TClassDef(wxSingleInstanceChecker)
    pub fn wxSingleInstanceChecker_Create(_obj: *u8 /* void* */, name: *u8 /* void* */, path: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSingleInstanceChecker_CreateDefault() -> *u8 /* void* */;
    pub fn wxSingleInstanceChecker_Delete(_obj: *u8 /* void* */);
    pub fn wxSingleInstanceChecker_IsAnotherRunning(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxTreeEvent,wxNotifyEvent)
    pub fn wxTreeEvent_GetCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeEvent_GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxTreeEvent_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeEvent_GetOldItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxTreeEvent_GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxClassInfo)
    pub fn wxClassInfo_CreateClassByName(_inf: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetClassName(_inf: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_IsKindOf(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxClassInfo)
    pub fn wxClassInfo_FindClass(_txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetBaseClassName1(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetBaseClassName2(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetClassNameEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxClassInfo_IsKindOfEx(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNotebook_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    
    // TClassDefExtend(wxFSFile,wxObject)
    
    // TClassDefExtend(wxFileSystemHandler,wxObject)
    
    // TClassDef(wxStreamBuffer)
    
    // TClassDefExtend(wxDragImage,wxObject)
    
    // TClassDef(wxManagedPtr)
    pub fn wxManagedPtr_GetPtr(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_NoFinalize(self_: *u8 /* void* */);
    pub fn wxManagedPtr_Finalize(self_: *u8 /* void* */);
    pub fn wxManagedPtr_Delete(self_: *u8 /* void* */);
    pub fn wxManagedPtr_GetDeleteFunction() -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromObject(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromDateTime(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromGridCellCoordsArray(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromBitmap(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromIcon(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromBrush(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromColour(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromCursor(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromFont(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxManagedPtr_CreateFromPen(obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxObject_SafeDelete(self_: *u8 /* void* */);
    pub fn wxBitmap_SafeDelete(self_: *u8 /* void* */);
    pub fn wxIcon_SafeDelete(self_: *u8 /* void* */);
    pub fn wxBrush_SafeDelete(self_: *u8 /* void* */);
    pub fn wxColour_SafeDelete(self_: *u8 /* void* */);
    pub fn wxCursor_SafeDelete(self_: *u8 /* void* */);
    pub fn wxFont_SafeDelete(self_: *u8 /* void* */);
    pub fn wxPen_SafeDelete(self_: *u8 /* void* */);
    pub fn wxBitmap_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIcon_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBrush_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxColour_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCursor_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFont_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPen_IsStatic(self_: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxCheckBox,wxControl)
    pub fn wxCheckBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCheckBox_Delete(_obj: *u8 /* void* */);
    pub fn wxCheckBox_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCheckBox_SetValue(_obj: *u8 /* void* */, value: c_int /* int */);
    
    // TClassDefExtend(cbLayoutRowEvent,cbPluginEvent)
    pub fn cbLayoutRowEvent_Row(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxTextEntryDialog,wxDialog)
    
    // TClassDefExtend(wxBoxSizer,wxSizer)
    pub fn wxBoxSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBoxSizer_Create(orient: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBoxSizer_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBoxSizer_RecalcSizes(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxSetCursorEvent,wxEvent)
    pub fn wxSetCursorEvent_GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSetCursorEvent_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSetCursorEvent_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSetCursorEvent_HasCursor(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSetCursorEvent_SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */);
    
    // TClassDefExtend(cbDrawPaneDecorEvent,cbPluginEvent)
    pub fn cbDrawPaneDecorEvent_Dc(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxFileName)
}
