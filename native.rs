use std::libc::*;
use types::*;

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
    // missing: wxMutexGui_Enter
    // missing: wxMutexGui_Leave
    
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
    
    // TClassDefExtend(ELJArtProv,wxArtProvider)
    pub fn ELJArtProv_Create(_obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJArtProv_Release(_obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJClient,wxClient)
    // missing: ELJClient_Create
    // missing: ELJClient_Delete
    // missing: ELJClient_MakeConnection
    
    // TClassDefExtend(ELJCommand,wxCommand)
    // missing: ELJCommand_CanUndo
    // missing: ELJCommand_Create
    // missing: ELJCommand_Delete
    // missing: ELJCommand_GetName
    
    // TClassDefExtend(ELJConnection,wxConnection)
    // missing: ELJConnection_Advise
    // missing: ELJConnection_Compress
    // missing: ELJConnection_Create
    // missing: ELJConnection_CreateDefault
    // missing: ELJConnection_Delete
    // missing: ELJConnection_Disconnect
    // missing: ELJConnection_Execute
    // missing: ELJConnection_Poke
    // missing: ELJConnection_Request
    // missing: ELJConnection_SetOnAdvise
    // missing: ELJConnection_SetOnDisconnect
    // missing: ELJConnection_SetOnExecute
    // missing: ELJConnection_SetOnPoke
    // missing: ELJConnection_SetOnRequest
    // missing: ELJConnection_SetOnStartAdvise
    // missing: ELJConnection_SetOnStopAdvise
    // missing: ELJConnection_StartAdvise
    // missing: ELJConnection_StopAdvise
    
    // TClassDef(ELJDragDataObject)
    pub fn ELJDragDataObject_Create(_obj: *u8 /* void* */, _fmt: *u8 /* void* */, _func1: *u8 /* void* */, _func2: *u8 /* void* */, _func3: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJDragDataObject_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJDropTarget,wxDropTarget)
    pub fn ELJDropTarget_Create(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJDropTarget_Delete(_obj: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJDropTarget_SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    
    // TClassDefExtend(ELJFileDropTarget,wxFileDropTarget)
    pub fn ELJFileDropTarget_Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJFileDropTarget_Delete(_obj: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJFileDropTarget_SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    
    // TClassDefExtend(ELJGridTable,wxGridTableBase)
    pub fn ELJGridTable_Create(_obj: *u8 /* void* */, _EifGetNumberRows: *u8 /* void* */, _EifGetNumberCols: *u8 /* void* */, _EifGetValue: *u8 /* void* */, _EifSetValue: *u8 /* void* */, _EifIsEmptyCell: *u8 /* void* */, _EifClear: *u8 /* void* */, _EifInsertRows: *u8 /* void* */, _EifAppendRows: *u8 /* void* */, _EifDeleteRows: *u8 /* void* */, _EifInsertCols: *u8 /* void* */, _EifAppendCols: *u8 /* void* */, _EifDeleteCols: *u8 /* void* */, _EifSetRowLabelValue: *u8 /* void* */, _EifSetColLabelValue: *u8 /* void* */, _EifGetRowLabelValue: *u8 /* void* */, _EifGetColLabelValue: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJGridTable_Delete(_obj: *u8 /* void* */);
    pub fn ELJGridTable_GetView(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJGridTable_SendTableMessage(_obj: *u8 /* void* */, id: c_int /* int */, val1: c_int /* int */, val2: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(ELJLocale,wxLocale)
    
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
    
    // TClassDef(ELJMessageParameters)
    // missing: wxMessageParameters_Create
    // missing: wxMessageParameters_Delete
    
    // TClassDefExtend(ELJPlotCurve,wxPlotCurve)
    // missing: ELJPlotCurve_Create
    // missing: ELJPlotCurve_Delete
    // missing: ELJPlotCurve_GetEndY
    // missing: ELJPlotCurve_GetOffsetY
    // missing: ELJPlotCurve_GetStartY
    // missing: ELJPlotCurve_SetEndY
    // missing: ELJPlotCurve_SetOffsetY
    // missing: ELJPlotCurve_SetPenNormal
    // missing: ELJPlotCurve_SetPenSelected
    // missing: ELJPlotCurve_SetStartY
    
    // TClassDefExtend(ELJPreviewControlBar,wxPreviewControlBar)
    pub fn ELJPreviewControlBar_Create(preview: *u8 /* void* */, buttons: c_int /* int */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(ELJPreviewFrame,wxPreviewFrame)
    pub fn ELJPreviewFrame_Create(_obj: *u8 /* void* */, _init: *u8 /* void* */, _create_canvas: *u8 /* void* */, _create_toolbar: *u8 /* void* */, preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_GetControlBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_GetPreviewCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_GetPrintPreview(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJPreviewFrame_Initialize(_obj: *u8 /* void* */);
    pub fn ELJPreviewFrame_SetControlBar(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn ELJPreviewFrame_SetPreviewCanvas(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn ELJPreviewFrame_SetPrintPreview(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    
    // TClassDefExtend(ELJServer,wxServer)
    // missing: ELJServer_Create
    // missing: ELJServer_Delete
    // missing: ELJServer_Initialize
    
    // TClassDefExtend(ELJTextDropTarget,wxTextDropTarget)
    pub fn ELJTextDropTarget_Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    pub fn ELJTextDropTarget_Delete(_obj: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    pub fn ELJTextDropTarget_SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    
    // TClassDefExtend(ELJTextValidator,wxTextValidator)
    pub fn ELJTextValidator_Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */, _txt: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(cbAntiflickerPlugin,cbPluginBase)
    // missing: cbAntiflickerPlugin_Create
    // missing: cbAntiflickerPlugin_CreateDefault
    // missing: cbAntiflickerPlugin_Delete
    
    // TClassDefExtend(cbBarDragPlugin,cbPluginBase)
    // missing: cbBarDragPlugin_Create
    // missing: cbBarDragPlugin_CreateDefault
    // missing: cbBarDragPlugin_Delete
    
    // TClassDefExtend(cbBarHintsPlugin,cbPluginBase)
    // missing: cbBarHintsPlugin_Create
    // missing: cbBarHintsPlugin_CreateDefault
    // missing: cbBarHintsPlugin_Delete
    // missing: cbBarHintsPlugin_SetGrooveCount
    
    // TClassDefExtend(cbBarInfo,wxObject)
    // missing: cbBarInfo_Create
    // missing: cbBarInfo_Delete
    // missing: cbBarInfo_IsExpanded
    // missing: cbBarInfo_IsFixed
    
    // TClassDefExtend(cbBarSpy,wxEvtHandler)
    // missing: cbBarSpy_Create
    // missing: cbBarSpy_CreateDefault
    // missing: cbBarSpy_Delete
    // missing: cbBarSpy_ProcessEvent
    // missing: cbBarSpy_SetBarWindow
    
    // TClassDefExtend(cbCloseBox,cbMiniButton)
    // missing: cbCloseBox_Create
    
    // TClassDefExtend(cbCollapseBox,cbMiniButton)
    // missing: cbCollapseBox_Create
    
    // TClassDefExtend(cbCommonPaneProperties,wxObject)
    // missing: cbCommonPaneProperties_Assign
    // missing: cbCommonPaneProperties_BarCollapseIconsOn
    // missing: cbCommonPaneProperties_BarDragHintsOn
    // missing: cbCommonPaneProperties_BarFloatingOn
    // missing: cbCommonPaneProperties_ColProportionsOn
    // missing: cbCommonPaneProperties_CreateDefault
    // missing: cbCommonPaneProperties_Delete
    // missing: cbCommonPaneProperties_ExactDockPredictionOn
    // missing: cbCommonPaneProperties_MinCBarDim
    // missing: cbCommonPaneProperties_NonDestructFrictionOn
    // missing: cbCommonPaneProperties_OutOfPaneDragOn
    // missing: cbCommonPaneProperties_RealTimeUpdatesOn
    // missing: cbCommonPaneProperties_ResizeHandleSize
    // missing: cbCommonPaneProperties_RowProportionsOn
    // missing: cbCommonPaneProperties_SetBarCollapseIconsOn
    // missing: cbCommonPaneProperties_SetBarDragHintsOn
    // missing: cbCommonPaneProperties_SetBarFloatingOn
    // missing: cbCommonPaneProperties_SetColProportionsOn
    // missing: cbCommonPaneProperties_SetExactDockPredictionOn
    // missing: cbCommonPaneProperties_SetMinCBarDim
    // missing: cbCommonPaneProperties_SetNonDestructFrictionOn
    // missing: cbCommonPaneProperties_SetOutOfPaneDragOn
    // missing: cbCommonPaneProperties_SetRealTimeUpdatesOn
    // missing: cbCommonPaneProperties_SetResizeHandleSize
    // missing: cbCommonPaneProperties_SetRowProportionsOn
    // missing: cbCommonPaneProperties_SetShow3DPaneBorderOn
    // missing: cbCommonPaneProperties_Show3DPaneBorderOn
    
    // TClassDefExtend(cbCustomizeBarEvent,cbPluginEvent)
    // missing: cbCustomizeBarEvent_Bar
    // missing: cbCustomizeBarEvent_ClickPos
    
    // TClassDefExtend(cbCustomizeLayoutEvent,cbPluginEvent)
    // missing: cbCustomizeLayoutEvent_ClickPos
    
    // TClassDefExtend(cbDimHandlerBase,wxObject)
    
    // TClassDefExtend(cbDimInfo,wxObject)
    // missing: cbDimInfo_Assign
    // missing: cbDimInfo_Create
    // missing: cbDimInfo_CreateDefault
    // missing: cbDimInfo_CreateWithHandler
    // missing: cbDimInfo_CreateWithInfo
    // missing: cbDimInfo_Delete
    // missing: cbDimInfo_GetDimHandler
    
    // TClassDefExtend(cbDockBox,cbMiniButton)
    // missing: cbDockBox_Create
    
    // TClassDefExtend(cbDockPane,wxObject)
    // missing: cbDockPane_BarPresent
    // missing: cbDockPane_Create
    // missing: cbDockPane_CreateDefault
    // missing: cbDockPane_Delete
    // missing: cbDockPane_GetAlignment
    // missing: cbDockPane_GetBarInfoByWindow
    // missing: cbDockPane_GetBarResizeRange
    // missing: cbDockPane_GetDockingState
    // missing: cbDockPane_GetFirstRow
    // missing: cbDockPane_GetPaneHeight
    // missing: cbDockPane_GetRealRect
    // missing: cbDockPane_GetRowList
    // missing: cbDockPane_GetRowResizeRange
    // missing: cbDockPane_HitTestPaneItems
    // missing: cbDockPane_InsertBarByCoord
    // missing: cbDockPane_InsertBarByInfo
    // missing: cbDockPane_InsertBarToRow
    // missing: cbDockPane_InsertRow
    // missing: cbDockPane_IsHorizontal
    // missing: cbDockPane_MatchesMask
    // missing: cbDockPane_RemoveBar
    // missing: cbDockPane_RemoveRow
    // missing: cbDockPane_SetBoundsInParent
    // missing: cbDockPane_SetMargins
    // missing: cbDockPane_SetPaneWidth
    
    // TClassDefExtend(cbDrawBarDecorEvent,cbPluginEvent)
    // missing: cbDrawBarDecorEvent_Bar
    // missing: cbDrawBarDecorEvent_BoundsInParent
    // missing: cbDrawBarDecorEvent_Dc
    
    // TClassDefExtend(cbDrawBarHandlesEvent,cbPluginEvent)
    // missing: cbDrawBarHandlesEvent_Bar
    // missing: cbDrawBarHandlesEvent_Dc
    
    // TClassDefExtend(cbDrawHintRectEvent,cbPluginEvent)
    // missing: cbDrawHintRectEvent_EraseRect
    // missing: cbDrawHintRectEvent_IsInClient
    // missing: cbDrawHintRectEvent_LastTime
    // missing: cbDrawHintRectEvent_Rect
    
    // TClassDefExtend(cbDrawPaneBkGroundEvent,cbPluginEvent)
    // missing: cbDrawPaneBkGroundEvent_Dc
    
    // TClassDefExtend(cbDrawPaneDecorEvent,cbPluginEvent)
    // missing: cbDrawPaneDecorEvent_Dc
    
    // TClassDefExtend(cbDrawRowBkGroundEvent,cbPluginEvent)
    // missing: cbDrawRowBkGroundEvent_Dc
    // missing: cbDrawRowBkGroundEvent_Row
    
    // TClassDefExtend(cbDrawRowDecorEvent,cbPluginEvent)
    // missing: cbDrawRowDecorEvent_Dc
    // missing: cbDrawRowDecorEvent_Row
    
    // TClassDefExtend(cbDrawRowHandlesEvent,cbPluginEvent)
    // missing: cbDrawRowHandlesEvent_Dc
    // missing: cbDrawRowHandlesEvent_Row
    
    // TClassDefExtend(cbDynToolBarDimHandler,cbDimHandlerBase)
    // missing: cbDynToolBarDimHandler_Create
    // missing: cbDynToolBarDimHandler_Delete
    
    // TClassDefExtend(cbFinishDrawInAreaEvent,cbPluginEvent)
    // missing: cbFinishDrawInAreaEvent_Area
    
    // TClassDefExtend(cbFloatedBarWindow,wxToolWindow)
    // missing: cbFloatedBarWindow_Create
    // missing: cbFloatedBarWindow_GetBar
    // missing: cbFloatedBarWindow_PositionFloatedWnd
    // missing: cbFloatedBarWindow_SetBar
    // missing: cbFloatedBarWindow_SetLayout
    
    // TClassDefExtend(cbGCUpdatesMgr,cbSimpleUpdatesMgr)
    // missing: cbGCUpdatesMgr_Create
    // missing: cbGCUpdatesMgr_CreateDefault
    // missing: cbGCUpdatesMgr_Delete
    // missing: cbGCUpdatesMgr_UpdateNow
    
    // TClassDefExtend(cbHintAnimationPlugin,cbPluginBase)
    // missing: cbHintAnimationPlugin_Create
    // missing: cbHintAnimationPlugin_CreateDefault
    // missing: cbHintAnimationPlugin_Delete
    
    // TClassDefExtend(cbInsertBarEvent,cbPluginEvent)
    // missing: cbInsertBarEvent_Bar
    // missing: cbInsertBarEvent_Row
    
    // TClassDefExtend(cbLayoutRowEvent,cbPluginEvent)
    // missing: cbLayoutRowEvent_Row
    
    // TClassDefExtend(cbLeftDClickEvent,cbPluginEvent)
    // missing: cbLeftDClickEvent_Pos
    
    // TClassDefExtend(cbLeftDownEvent,cbPluginEvent)
    // missing: cbLeftDownEvent_Pos
    
    // TClassDefExtend(cbLeftUpEvent,cbPluginEvent)
    // missing: cbLeftUpEvent_Pos
    
    // TClassDefExtend(cbMiniButton,wxObject)
    // missing: cbMiniButton_Create
    // missing: cbMiniButton_Delete
    // missing: cbMiniButton_Dim
    // missing: cbMiniButton_DragStarted
    // missing: cbMiniButton_Enable
    // missing: cbMiniButton_Enabled
    // missing: cbMiniButton_HitTest
    // missing: cbMiniButton_IsPressed
    // missing: cbMiniButton_Layout
    // missing: cbMiniButton_Pane
    // missing: cbMiniButton_Plugin
    // missing: cbMiniButton_Pos
    // missing: cbMiniButton_Pressed
    // missing: cbMiniButton_Refresh
    // missing: cbMiniButton_Reset
    // missing: cbMiniButton_SetPos
    // missing: cbMiniButton_Visible
    // missing: cbMiniButton_WasClicked
    // missing: cbMiniButton_Wnd
    
    // TClassDefExtend(cbMotionEvent,cbPluginEvent)
    // missing: cbMotionEvent_Pos
    
    // TClassDefExtend(cbPaneDrawPlugin,cbPluginBase)
    // missing: cbPaneDrawPlugin_Create
    // missing: cbPaneDrawPlugin_CreateDefault
    // missing: cbPaneDrawPlugin_Delete
    
    // TClassDefExtend(cbPluginBase,wxEvtHandler)
    // missing: cbPluginBase_Delete
    // missing: cbPluginBase_GetPaneMask
    // missing: cbPluginBase_IsReady
    // missing: cbPluginBase_Plugin
    // missing: cbPluginBase_ProcessEvent
    
    // TClassDefExtend(cbPluginEvent,wxEvent)
    // missing: cbPluginEvent_Pane
    
    // TClassDefExtend(cbRemoveBarEvent,cbPluginEvent)
    // missing: cbRemoveBarEvent_Bar
    
    // TClassDefExtend(cbResizeBarEvent,cbPluginEvent)
    // missing: cbResizeBarEvent_Bar
    // missing: cbResizeBarEvent_Row
    
    // TClassDefExtend(cbResizeRowEvent,cbPluginEvent)
    // missing: cbResizeRowEvent_ForUpperHandle
    // missing: cbResizeRowEvent_HandleOfs
    // missing: cbResizeRowEvent_Row
    
    // TClassDefExtend(cbRightDownEvent,cbPluginEvent)
    // missing: cbRightDownEvent_Pos
    
    // TClassDefExtend(cbRightUpEvent,cbPluginEvent)
    // missing: cbRightUpEvent_Pos
    
    // TClassDefExtend(cbRowDragPlugin,cbPluginBase)
    // missing: cbRowDragPlugin_Create
    // missing: cbRowDragPlugin_CreateDefault
    // missing: cbRowDragPlugin_Delete
    
    // TClassDefExtend(cbRowInfo,wxObject)
    // missing: cbRowInfo_Create
    // missing: cbRowInfo_Delete
    // missing: cbRowInfo_GetFirstBar
    
    // TClassDefExtend(cbRowLayoutPlugin,cbPluginBase)
    // missing: cbRowLayoutPlugin_Create
    // missing: cbRowLayoutPlugin_CreateDefault
    // missing: cbRowLayoutPlugin_Delete
    
    // TClassDefExtend(cbSimpleCustomizationPlugin,cbPluginBase)
    // missing: cbSimpleCustomizationPlugin_Create
    // missing: cbSimpleCustomizationPlugin_CreateDefault
    // missing: cbSimpleCustomizationPlugin_Delete
    
    // TClassDefExtend(cbSimpleUpdatesMgr,cbUpdatesManagerBase)
    
    // TClassDefExtend(cbSizeBarWndEvent,cbPluginEvent)
    // missing: cbSizeBarWndEvent_Bar
    // missing: cbSizeBarWndEvent_BoundsInParent
    
    // TClassDefExtend(cbStartBarDraggingEvent,cbPluginEvent)
    // missing: cbStartBarDraggingEvent_Bar
    // missing: cbStartBarDraggingEvent_Pos
    
    // TClassDefExtend(cbStartDrawInAreaEvent,cbPluginEvent)
    // missing: cbStartDrawInAreaEvent_Area
    
    // TClassDefExtend(cbUpdatesManagerBase,wxObject)
    
    // TClassDef(wxAcceleratorEntry)
    pub fn wxAcceleratorEntry_Create(flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) -> *u8 /* void* */;
    pub fn wxAcceleratorEntry_Delete(_obj: *u8 /* void* */);
    pub fn wxAcceleratorEntry_GetCommand(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxAcceleratorEntry_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxAcceleratorEntry_GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxAcceleratorEntry_Set(_obj: *u8 /* void* */, flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */);
    
    // TClassDef(wxAcceleratorTable)
    pub fn wxAcceleratorTable_Create(n: c_int /* int */, entries: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxAcceleratorTable_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxActivateEvent,wxEvent)
    pub fn wxActivateEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxActivateEvent_GetActive(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxApp,wxEvtHandler)
    
    // TClassDef(wxArray)
    
    // TClassDefExtend(wxArrayString,wxArray)
    
    // TClassDefExtend(wxArtProvider,wxObject)
    pub fn PopProvider() -> bool /* bool */;
    pub fn PushProvider(provider: *u8 /* void* */);
    pub fn RemoveProvider(provider: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxAutoBufferedPaintDC,wxDC)
    pub fn wxAutoBufferedPaintDC_Create(window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxAutoBufferedPaintDC_Delete(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxAutomationObject,wxObject)
    
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
    
    // TClassDefExtend(wxBitmapToggleButton,wxToggleButton)
    pub fn wxBitmapToggleButton_Create(parent: *u8 /* void* */, id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBitmapToggleButton_Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    pub fn wxBitmapToggleButton_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxBitmapToggleButton_SetValue(_obj: *u8 /* void* */, state: bool /* bool */);
    pub fn wxBitmapToggleButton_SetBitmapLabel(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    
    // TClassDefExtend(wxBitmapDataObject,wxDataObjectSimple)
    pub fn BitmapDataObject_Create(_bmp: *u8 /* void* */) -> *u8 /* void* */;
    pub fn BitmapDataObject_CreateEmpty() -> *u8 /* void* */;
    pub fn BitmapDataObject_Delete(_obj: *u8 /* void* */);
    pub fn BitmapDataObject_GetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    pub fn BitmapDataObject_SetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    
    // TClassDefExtend(wxBitmapHandler,wxObject)
    
    // TClassDefExtend(wxBoxSizer,wxSizer)
    pub fn wxBoxSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBoxSizer_Create(orient: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBoxSizer_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxBoxSizer_RecalcSizes(_obj: *u8 /* void* */);
    
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
    
    // TClassDefExtend(wxBrushList,wxList)
    
    // TClassDefExtend(wxBufferedDC,wxDC)
    pub fn wxBufferedDC_CreateByDCAndSize(dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedDC_CreateByDCAndBitmap(dc: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedDC_Delete(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxBufferedPaintDC,wxDC)
    pub fn wxBufferedPaintDC_Create(window: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedPaintDC_CreateWithBitmap(window: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxBufferedPaintDC_Delete(self_: *u8 /* void* */);
    
    // TClassDefExtend(wxBufferedInputStream,wxFilterInputStream)
    
    // TClassDefExtend(wxBufferedOutputStream,wxFilterOutputStream)
    
    // TClassDef(wxBusyCursor)
    pub fn wxBusyCursor_Create() -> *u8 /* void* */;
    pub fn wxBusyCursor_CreateWithCursor(_cur: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBusyCursor_Delete(_obj: *u8 /* void* */);
    
    // TClassDef(wxBusyInfo)
    pub fn wxBusyInfo_Create(_txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxBusyInfo_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxButton,wxControl)
    pub fn wxButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxButton_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    pub fn wxButton_SetDefault(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxCSConv,wxMBConv)
    
    // TClassDefExtend(wxCalculateLayoutEvent,wxEvent)
    pub fn wxCalculateLayoutEvent_Create(id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCalculateLayoutEvent_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxCalculateLayoutEvent_GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxCalculateLayoutEvent_SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    pub fn wxCalculateLayoutEvent_SetRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    
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
    
    // TClassDefExtend(wxCalendarEvent,wxCommandEvent)
    pub fn wxCalendarEvent_GetDate(_obj: *u8 /* void* */, _dte: *u8 /* void* */);
    pub fn wxCalendarEvent_GetWeekDay(_obj: *u8 /* void* */) -> c_int /* int */;
    
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
    
    // TClassDefExtend(wxCheckBox,wxControl)
    pub fn wxCheckBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCheckBox_Delete(_obj: *u8 /* void* */);
    pub fn wxCheckBox_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCheckBox_SetValue(_obj: *u8 /* void* */, value: c_int /* int */);
    
    // TClassDefExtend(wxCheckListBox,wxListBox)
    pub fn wxCheckListBox_Check(_obj: *u8 /* void* */, item: c_int /* int */, check: bool /* bool */);
    pub fn wxCheckListBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxCheckListBox_Delete(_obj: *u8 /* void* */);
    pub fn wxCheckListBox_IsChecked(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
    
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
    
    // TClassDef(wxClassInfo)
    pub fn wxClassInfo_CreateClassByName(_inf: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetClassName(_inf: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_IsKindOf(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxClient,wxClientBase)
    
    // TClassDefExtend(wxClientBase,wxObject)
    
    // TClassDefExtend(wxClientDC,wxWindowDC)
    pub fn wxClientDC_Create(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClientDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDef(wxClientData)
    
    // TClassDef(wxClientDataContainer)
    
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
    
    // TClassDefExtend(wxCloseEvent,wxEvent)
    pub fn wxCloseEvent_CanVeto(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCloseEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxCloseEvent_GetLoggingOff(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCloseEvent_GetVeto(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxCloseEvent_SetCanVeto(_obj: *u8 /* void* */, canVeto: bool /* bool */);
    pub fn wxCloseEvent_SetLoggingOff(_obj: *u8 /* void* */, logOff: bool /* bool */);
    pub fn wxCloseEvent_Veto(_obj: *u8 /* void* */, veto: bool /* bool */);
    
    // TClassDefExtend(wxClosure,wxObject)
    
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
    
    // TClassDefExtend(wxColourData,wxObject)
    pub fn wxColourData_Create() -> *u8 /* void* */;
    pub fn wxColourData_Delete(_obj: *u8 /* void* */);
    pub fn wxColourData_GetChooseFull(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxColourData_GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxColourData_GetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxColourData_SetChooseFull(_obj: *u8 /* void* */, flag: bool /* bool */);
    pub fn wxColourData_SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    pub fn wxColourData_SetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, colour: *u8 /* void* */);
    
    // TClassDefExtend(wxColourDatabase,wxList)
    
    // TClassDefExtend(wxColourDialog,wxDialog)
    pub fn wxColourDialog_Create(_prt: *u8 /* void* */, col: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxColourDialog_GetColourData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    
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
    
    // TClassDefExtend(wxCommand,wxObject)
    
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
    
    // TClassDef(wxCommandLineParser)
    
    // TClassDefExtend(wxCommandProcessor,wxObject)
    // missing: wxCommandProcessor_CanRedo
    // missing: wxCommandProcessor_CanUndo
    // missing: wxCommandProcessor_ClearCommands
    // missing: wxCommandProcessor_Delete
    // missing: wxCommandProcessor_GetCommands
    // missing: wxCommandProcessor_GetEditMenu
    // missing: wxCommandProcessor_GetMaxCommands
    // missing: wxCommandProcessor_Initialize
    // missing: wxCommandProcessor_Redo
    // missing: wxCommandProcessor_SetEditMenu
    // missing: wxCommandProcessor_SetMenuStrings
    // missing: wxCommandProcessor_Submit
    // missing: wxCommandProcessor_Undo
    // missing: wxCommandProcessor_wxCommandProcessor
    
    // TClassDef(wxCondition)
    // missing: wxCondition_Broadcast
    // missing: wxCondition_Create
    // missing: wxCondition_Delete
    // missing: wxCondition_Signal
    // missing: wxCondition_Wait
    // missing: wxCondition_WaitFor
    
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
    
    // TClassDefExtend(wxConnection,wxConnectionBase)
    
    // TClassDefExtend(wxConnectionBase,wxObject)
    
    // TClassDefExtend(wxContextHelp,wxObject)
    pub fn wxContextHelp_BeginContextHelp(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    pub fn wxContextHelp_Create(win: *u8 /* void* */, beginHelp: bool /* bool */) -> *u8 /* void* */;
    pub fn wxContextHelp_Delete(_obj: *u8 /* void* */);
    pub fn wxContextHelp_EndContextHelp(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxContextHelpButton,wxBitmapButton)
    pub fn wxContextHelpButton_Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_long /* long */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxControl,wxWindow)
    pub fn wxControl_Command(_obj: *u8 /* void* */, event: *u8 /* void* */);
    pub fn wxControl_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxControl_SetLabel(_obj: *u8 /* void* */, text: *u8 /* void* */);
    
    // TClassDefExtend(wxCountingOutputStream,wxOutputStream)
    
    // TClassDef(wxCriticalSection)
    // missing: wxCriticalSection_Create
    // missing: wxCriticalSection_Delete
    // missing: wxCriticalSection_Enter
    // missing: wxCriticalSection_Leave
    
    // TClassDef(wxCriticalSectionLocker)
    
    // TClassDefExtend(wxCursor,wxBitmap)
    pub fn Cursor_CreateFromStock(_id: c_int /* int */) -> *u8 /* void* */;
    pub fn Cursor_CreateFromImage(image: *u8 /* void* */) -> *u8 /* void* */;
    pub fn Cursor_CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxCustomDataObject,wxDataObjectSimple)
    
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
    
    // TClassDef(wxDCClipper)
    
    // TClassDefExtend(wxDDEClient,wxClientBase)
    
    // TClassDefExtend(wxDDEConnection,wxConnectionBase)
    
    // TClassDefExtend(wxDDEServer,wxServerBase)
    
    // TClassDef(wxDataFormat)
    pub fn wxDataFormat_CreateFromId(name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDataFormat_CreateFromType(typ: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDataFormat_Delete(_obj: *u8 /* void* */);
    pub fn wxDataFormat_GetId(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDataFormat_GetType(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDataFormat_IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDataFormat_SetId(_obj: *u8 /* void* */, id: *u8 /* void* */);
    pub fn wxDataFormat_SetType(_obj: *u8 /* void* */, typ: c_int /* int */);
    
    // TClassDef(wxDataInputStream)
    
    // TClassDef(wxDataObject)
    
    // TClassDefExtend(wxDataObjectComposite,wxDataObject)
    pub fn wxDataObjectComposite_Add(_obj: *u8 /* void* */, _dat: *u8 /* void* */, _preferred: c_int /* int */);
    pub fn wxDataObjectComposite_Create() -> *u8 /* void* */;
    pub fn wxDataObjectComposite_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxDataObjectSimple,wxDataObject)
    
    // TClassDef(wxDataOutputStream)
    
    // TClassDefExtend(wxDatabase,wxObject)
    
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
    // missing: wxDateTime_IsGregorianDate
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
    
    // TClassDef(wxDb)
    
    // TClassDef(wxDbColDef)
    
    // TClassDef(wxDbColFor)
    
    // TClassDef(wxDbColInf)
    
    // TClassDef(wxDbConnectInf)
    
    // TClassDef(wxDbInf)
    
    // TClassDef(wxDbSqlTypeInfo)
    
    // TClassDef(wxDbTable)
    
    // TClassDef(wxDbTableInfo)
    
    // TClassDef(wxDebugContext)
    
    // TClassDefExtend(wxDialUpEvent,wxEvent)
    // missing: wxDialUpEvent_IsConnectedEvent
    // missing: wxDialUpEvent_IsOwnEvent
    
    // TClassDef(wxDialUpManager)
    // missing: wxDialUpManager_CancelDialing
    // missing: wxDialUpManager_Create
    // missing: wxDialUpManager_Delete
    // missing: wxDialUpManager_Dial
    // missing: wxDialUpManager_DisableAutoCheckOnlineStatus
    // missing: wxDialUpManager_EnableAutoCheckOnlineStatus
    // missing: wxDialUpManager_GetISPNames
    // missing: wxDialUpManager_HangUp
    // missing: wxDialUpManager_IsAlwaysOnline
    // missing: wxDialUpManager_IsDialing
    // missing: wxDialUpManager_IsOk
    // missing: wxDialUpManager_IsOnline
    // missing: wxDialUpManager_SetConnectCommand
    // missing: wxDialUpManager_SetOnlineStatus
    // missing: wxDialUpManager_SetWellKnownHost
    
    // TClassDefExtend(wxDialog,wxTopLevelWindow)
    pub fn wxDialog_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDialog_EndModal(_obj: *u8 /* void* */, retCode: c_int /* int */);
    pub fn wxDialog_GetReturnCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDialog_IsModal(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxDialog_SetReturnCode(_obj: *u8 /* void* */, returnCode: c_int /* int */);
    pub fn wxDialog_ShowModal(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxDirDialog,wxDialog)
    pub fn wxDirDialog_Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxDirDialog_GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDirDialog_GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxDirDialog_GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxDirDialog_SetMessage(_obj: *u8 /* void* */, msg: *u8 /* void* */);
    pub fn wxDirDialog_SetPath(_obj: *u8 /* void* */, pth: *u8 /* void* */);
    pub fn wxDirDialog_SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    
    // TClassDef(wxDirTraverser)
    
    // TClassDef(wxDllLoader)
    
    // TClassDefExtend(wxDocChildFrame,wxFrame)
    
    // TClassDefExtend(wxDocMDIChildFrame,wxMDIChildFrame)
    
    // TClassDefExtend(wxDocMDIParentFrame,wxMDIParentFrame)
    
    // TClassDefExtend(wxDocManager,wxEvtHandler)
    
    // TClassDefExtend(wxDocParentFrame,wxFrame)
    
    // TClassDefExtend(wxDocTemplate,wxObject)
    
    // TClassDefExtend(wxDocument,wxEvtHandler)
    
    // TClassDefExtend(wxDragImage,wxObject)
    
    // TClassDefExtend(wxDrawControl,wxControl)
    pub fn wxDrawControl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDrawWindow,wxWindow)
    pub fn wxDrawWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDropFilesEvent,wxEvent)
    
    // TClassDef(wxDropSource)
    pub fn DropSource_Create(data: *u8 /* void* */, win: *u8 /* void* */, copy: *u8 /* void* */, move: *u8 /* void* */, none: *u8 /* void* */) -> *u8 /* void* */;
    pub fn DropSource_Delete(_obj: *u8 /* void* */);
    pub fn DropSource_DoDragDrop(_obj: *u8 /* void* */, _move: c_int /* int */) -> c_int /* int */;
    
    // TClassDef(wxDropTarget)
    pub fn wxDropTarget_GetData(_obj: *u8 /* void* */);
    pub fn wxDropTarget_SetDataObject(_obj: *u8 /* void* */, _dat: *u8 /* void* */);
    
    // TClassDefExtend(wxDynToolInfo,wxToolLayoutItem)
    // missing: wxDynToolInfo_Index
    // missing: wxDynToolInfo_RealSize
    // missing: wxDynToolInfo_pToolWnd
    
    // TClassDef(wxDynamicLibrary)
    
    // TClassDefExtend(wxDynamicSashWindow,wxWindow)
    // missing: wxDynamicSashWindow_Create
    // missing: wxDynamicSashWindow_Delete
    // missing: wxDynamicSashWindow_GetHScrollBar
    // missing: wxDynamicSashWindow_GetVScrollBar
    
    // TClassDefExtend(wxDynamicToolBar,wxToolBarBase)
    // missing: wxDynamicToolBar_AddSeparator
    // missing: wxDynamicToolBar_AddTool
    // missing: wxDynamicToolBar_AddToolBitmap
    // missing: wxDynamicToolBar_AddToolImage
    // missing: wxDynamicToolBar_AddToolLabel
    // missing: wxDynamicToolBar_Create
    // missing: wxDynamicToolBar_CreateDefault
    // missing: wxDynamicToolBar_CreateDefaultLayout
    // missing: wxDynamicToolBar_CreateParams
    // missing: wxDynamicToolBar_CreateTool
    // missing: wxDynamicToolBar_CreateToolControl
    // missing: wxDynamicToolBar_Delete
    // missing: wxDynamicToolBar_DoDeleteTool
    // missing: wxDynamicToolBar_DoEnableTool
    // missing: wxDynamicToolBar_DoInsertTool
    // missing: wxDynamicToolBar_DoSetToggle
    // missing: wxDynamicToolBar_DoToggleTool
    // missing: wxDynamicToolBar_DrawSeparator
    // missing: wxDynamicToolBar_EnableTool
    // missing: wxDynamicToolBar_FindToolForPosition
    // missing: wxDynamicToolBar_GetPreferredDim
    // missing: wxDynamicToolBar_GetToolInfo
    // missing: wxDynamicToolBar_Layout
    // missing: wxDynamicToolBar_RemoveTool
    // missing: wxDynamicToolBar_SetLayout
    
    // TClassDefExtend(wxEditableListBox,wxPanel)
    // missing: wxEditableListBox_Create
    // missing: wxEditableListBox_GetDelButton
    // missing: wxEditableListBox_GetDownButton
    // missing: wxEditableListBox_GetEditButton
    // missing: wxEditableListBox_GetListCtrl
    // missing: wxEditableListBox_GetNewButton
    // missing: wxEditableListBox_GetStrings
    // missing: wxEditableListBox_GetUpButton
    // missing: wxEditableListBox_SetStrings
    
    // TClassDefExtend(wxEncodingConverter,wxObject)
    pub fn wxEncodingConverter_Convert(_obj: *u8 /* void* */, input: *u8 /* void* */, output: *u8 /* void* */);
    pub fn wxEncodingConverter_Create() -> *u8 /* void* */;
    pub fn wxEncodingConverter_Delete(_obj: *u8 /* void* */);
    pub fn wxEncodingConverter_GetAllEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEncodingConverter_GetPlatformEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, platform: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxEncodingConverter_Init(_obj: *u8 /* void* */, input_enc: c_int /* int */, output_enc: c_int /* int */, method: c_int /* int */) -> c_int /* int */;
    
    // TClassDefExtend(wxEraseEvent,wxEvent)
    pub fn wxEraseEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxEraseEvent_GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
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
    
    // TClassDef(wxExpr)
    
    // TClassDefExtend(wxExprDatabase,wxList)
    
    // TClassDef(wxFFile)
    
    // TClassDefExtend(wxFFileInputStream,wxInputStream)
    
    // TClassDefExtend(wxFFileOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxFSFile,wxObject)
    
    // TClassDefExtend(wxFTP,wxProtocol)
    
    // TClassDefExtend(wxFileDataObject,wxDataObjectSimple)
    pub fn FileDataObject_AddFile(_obj: *u8 /* void* */, _fle: *u8 /* void* */);
    pub fn FileDataObject_Create(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    pub fn FileDataObject_Delete(_obj: *u8 /* void* */);
    pub fn FileDataObject_GetFilenames(_obj: *u8 /* void* */, _lst: *wchar_t /* wchar_t* */) -> c_int /* int */;
    
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
    
    // TClassDefExtend(wxFileDropTarget,wxDropTarget)
    
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
    
    // TClassDefExtend(wxFileInputStream,wxInputStream)
    
    // TClassDef(wxFileName)
    
    // TClassDefExtend(wxFileOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxFileSystem,wxObject)
    
    // TClassDefExtend(wxFileSystemHandler,wxObject)
    
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
    
    // TClassDefExtend(wxFilterInputStream,wxInputStream)
    
    // TClassDefExtend(wxFilterOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxFindDialogEvent,wxCommandEvent)
    pub fn wxFindDialogEvent_GetFindString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFindDialogEvent_GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxFindDialogEvent_GetReplaceString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    
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
    
    // TClassDefExtend(wxFindReplaceDialog,wxDialog)
    pub fn wxFindReplaceDialog_Create(parent: *u8 /* void* */, data: *u8 /* void* */, title: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFindReplaceDialog_GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFindReplaceDialog_SetData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    
    // TClassDefExtend(wxFlexGridSizer,wxGridSizer)
    pub fn wxFlexGridSizer_AddGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    pub fn wxFlexGridSizer_AddGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    pub fn wxFlexGridSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFlexGridSizer_Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */;
    pub fn wxFlexGridSizer_RecalcSizes(_obj: *u8 /* void* */);
    pub fn wxFlexGridSizer_RemoveGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    pub fn wxFlexGridSizer_RemoveGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    
    // TClassDefExtend(wxFocusEvent,wxEvent)
    
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
    
    // TClassDefExtend(wxFontDialog,wxDialog)
    pub fn wxFontDialog_Create(_prt: *u8 /* void* */, fnt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFontDialog_GetFontData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    
    // TClassDef(wxFontEnumerator)
    pub fn wxFontEnumerator_Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxFontEnumerator_Delete(_obj: *u8 /* void* */);
    pub fn wxFontEnumerator_EnumerateEncodings(_obj: *u8 /* void* */, facename: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFontEnumerator_EnumerateFacenames(_obj: *u8 /* void* */, encoding: c_int /* int */, fixedWidthOnly: c_int /* int */) -> bool /* bool */;
    
    // TClassDefExtend(wxFontList,wxList)
    
    // TClassDef(wxFontMapper)
    pub fn wxFontMapper_Create() -> *u8 /* void* */;
    pub fn wxFontMapper_GetAltForEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */, alt_encoding: *u8 /* void* */, _buf: *u8 /* void* */) -> bool /* bool */;
    pub fn wxFontMapper_IsEncodingAvailable(_obj: *u8 /* void* */, encoding: c_int /* int */, _buf: *u8 /* void* */) -> bool /* bool */;
    
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
    
    // TClassDefExtend(wxFrameLayout,wxEvtHandler)
    // missing: wxFrameLayout_Activate
    // missing: wxFrameLayout_AddBar
    // missing: wxFrameLayout_AddPlugin
    // missing: wxFrameLayout_AddPluginBefore
    // missing: wxFrameLayout_ApplyBarProperties
    // missing: wxFrameLayout_CaptureEventsForPane
    // missing: wxFrameLayout_CaptureEventsForPlugin
    // missing: wxFrameLayout_Create
    // missing: wxFrameLayout_Deactivate
    // missing: wxFrameLayout_Delete
    // missing: wxFrameLayout_DestroyBarWindows
    // missing: wxFrameLayout_EnableFloating
    // missing: wxFrameLayout_FindBarByName
    // missing: wxFrameLayout_FindBarByWindow
    // missing: wxFrameLayout_FindPlugin
    // missing: wxFrameLayout_FirePluginEvent
    // missing: wxFrameLayout_GetBars
    // missing: wxFrameLayout_GetClientHeight
    // missing: wxFrameLayout_GetClientRect
    // missing: wxFrameLayout_GetClientWidth
    // missing: wxFrameLayout_GetFrameClient
    // missing: wxFrameLayout_GetPane
    // missing: wxFrameLayout_GetPaneProperties
    // missing: wxFrameLayout_GetParentFrame
    // missing: wxFrameLayout_GetTopPlugin
    // missing: wxFrameLayout_GetUpdatesManager
    // missing: wxFrameLayout_HasTopPlugin
    // missing: wxFrameLayout_HideBarWindows
    // missing: wxFrameLayout_InverseVisibility
    // missing: wxFrameLayout_OnLButtonDown
    // missing: wxFrameLayout_OnLButtonUp
    // missing: wxFrameLayout_OnLDblClick
    // missing: wxFrameLayout_OnMouseMove
    // missing: wxFrameLayout_OnRButtonDown
    // missing: wxFrameLayout_OnRButtonUp
    // missing: wxFrameLayout_OnSize
    // missing: wxFrameLayout_PopAllPlugins
    // missing: wxFrameLayout_PopPlugin
    // missing: wxFrameLayout_PushDefaultPlugins
    // missing: wxFrameLayout_PushPlugin
    // missing: wxFrameLayout_RecalcLayout
    // missing: wxFrameLayout_RedockBar
    // missing: wxFrameLayout_RefreshNow
    // missing: wxFrameLayout_ReleaseEventsFromPane
    // missing: wxFrameLayout_ReleaseEventsFromPlugin
    // missing: wxFrameLayout_RemoveBar
    // missing: wxFrameLayout_RemovePlugin
    // missing: wxFrameLayout_SetBarState
    // missing: wxFrameLayout_SetFrameClient
    // missing: wxFrameLayout_SetMargins
    // missing: wxFrameLayout_SetPaneBackground
    // missing: wxFrameLayout_SetPaneProperties
    // missing: wxFrameLayout_SetTopPlugin
    // missing: wxFrameLayout_SetUpdatesManager
    
    // TClassDefExtend(wxGDIObject,wxObject)
    
    // TClassDefExtend(wxGLCanvas,wxScrolledWindow)
    
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
    
    // TClassDefExtend(wxGenericDirCtrl,wxControl)
    
    // TClassDefExtend(wxGenericValidator,wxValidator)
    
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
    
    // TClassDefExtend(wxGridCellBoolEditor,wxGridCellEditor)
    pub fn wxGridCellBoolEditor_Ctor() -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellBoolRenderer,wxGridCellRenderer)
    
    // TClassDefExtend(wxGridCellChoiceEditor,wxGridCellEditor)
    pub fn wxGridCellChoiceEditor_Ctor(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, allowOthers: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDef(wxGridCellCoordsArray)
    pub fn wxGridCellCoordsArray_Create() -> *u8 /* void* */;
    pub fn wxGridCellCoordsArray_Delete(_obj: *u8 /* void* */);
    pub fn wxGridCellCoordsArray_GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridCellCoordsArray_Item(_obj: *u8 /* void* */, _idx: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    
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
    
    // TClassDefExtend(wxGridCellFloatEditor,wxGridCellTextEditor)
    pub fn wxGridCellFloatEditor_Ctor(width: c_int /* int */, precision: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellFloatRenderer,wxGridCellStringRenderer)
    
    // TClassDefExtend(wxGridCellNumberEditor,wxGridCellTextEditor)
    pub fn wxGridCellNumberEditor_Ctor(min: c_int /* int */, max: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellNumberRenderer,wxGridCellStringRenderer)
    pub fn wxGridCellNumberRenderer_Ctor() -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellAutoWrapStringRenderer,wxGridCellStringRenderer)
    pub fn wxGridCellAutoWrapStringRenderer_Ctor() -> *u8 /* void* */;
    
    // TClassDefExtend(wxGridCellRenderer,wxGridCellWorker)
    
    // TClassDefExtend(wxGridCellStringRenderer,wxGridCellRenderer)
    
    // TClassDefExtend(wxGridCellTextEditor,wxGridCellEditor)
    pub fn wxGridCellTextEditor_Ctor() -> *u8 /* void* */;
    
    // TClassDef(wxGridCellWorker)
    
    // TClassDefExtend(wxGridEditorCreatedEvent,wxCommandEvent)
    pub fn wxGridEditorCreatedEvent_GetCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEditorCreatedEvent_GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridEditorCreatedEvent_GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEditorCreatedEvent_SetCol(_obj: *u8 /* void* */, col: c_int /* int */);
    pub fn wxGridEditorCreatedEvent_SetControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */);
    pub fn wxGridEditorCreatedEvent_SetRow(_obj: *u8 /* void* */, row: c_int /* int */);
    
    // TClassDefExtend(wxGridEvent,wxNotifyEvent)
    pub fn wxGridEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_GetCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridEvent_GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_Selecting(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
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
    
    // TClassDefExtend(wxGridSizeEvent,wxNotifyEvent)
    pub fn wxGridSizeEvent_GetRowOrCol(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxGridSizeEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxGridSizeEvent_ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridSizeEvent_MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridSizeEvent_ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxGridSizeEvent_AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    
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
    
    // TClassDefExtend(wxGridTableBase,wxObject)
    
    // TClassDefExtend(wxHTTP,wxProtocol)
    
    // TClassDef(wxHashMap)
    
    // TClassDefExtend(wxHelpController,wxHelpControllerBase)
    
    // TClassDefExtend(wxHelpControllerBase,wxObject)
    
    // TClassDefExtend(wxHelpControllerHelpProvider,wxSimpleHelpProvider)
    pub fn wxHelpControllerHelpProvider_Create(ctr: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpControllerHelpProvider_GetHelpController(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpControllerHelpProvider_SetHelpController(_obj: *u8 /* void* */, hc: *u8 /* void* */);
    
    // TClassDefExtend(wxHelpEvent,wxCommandEvent)
    pub fn wxHelpEvent_GetLink(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpEvent_GetTarget(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpEvent_SetLink(_obj: *u8 /* void* */, link: *u8 /* void* */);
    pub fn wxHelpEvent_SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxHelpEvent_SetTarget(_obj: *u8 /* void* */, target: *u8 /* void* */);
    
    // TClassDef(wxHelpProvider)
    pub fn wxHelpProvider_AddHelp(_obj: *u8 /* void* */, window: *u8 /* void* */, text: *u8 /* void* */);
    pub fn wxHelpProvider_AddHelpById(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */);
    pub fn wxHelpProvider_Delete(_obj: *u8 /* void* */);
    pub fn wxHelpProvider_Get() -> *u8 /* void* */;
    pub fn wxHelpProvider_GetHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpProvider_RemoveHelp(_obj: *u8 /* void* */, window: *u8 /* void* */);
    pub fn wxHelpProvider_Set(helpProvider: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxHelpProvider_ShowHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxHtmlCell,wxObject)
    
    // TClassDefExtend(wxHtmlColourCell,wxHtmlCell)
    
    // TClassDefExtend(wxHtmlContainerCell,wxHtmlCell)
    
    // TClassDefExtend(wxHtmlDCRenderer,wxObject)
    
    // TClassDefExtend(wxHtmlEasyPrinting,wxObject)
    
    // TClassDefExtend(wxHtmlFilter,wxObject)
    
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
    
    // TClassDefExtend(wxHtmlHelpData,wxObject)
    
    // TClassDefExtend(wxHtmlHelpFrame,wxFrame)
    
    // TClassDefExtend(wxHtmlLinkInfo,wxObject)
    
    // TClassDefExtend(wxHtmlParser,wxObject)
    
    // TClassDefExtend(wxHtmlPrintout,wxPrintout)
    
    // TClassDefExtend(wxHtmlTag,wxObject)
    
    // TClassDefExtend(wxHtmlTagHandler,wxObject)
    
    // TClassDefExtend(wxHtmlTagsModule,wxModule)
    
    // TClassDefExtend(wxHtmlWidgetCell,wxHtmlCell)
    
    // TClassDefExtend(wxHtmlWinParser,wxHtmlParser)
    
    // TClassDefExtend(wxHtmlWinTagHandler,wxHtmlTagHandler)
    
    // TClassDefExtend(wxHtmlWindow,wxScrolledWindow)
    
    // TClassDefExtend(wxIPV4address,wxSockAddress)
    
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
    
    // TClassDef(wxIconBundle)
    pub fn wxIconBundle_AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */);
    pub fn wxIconBundle_AddIconFromFile(_obj: *u8 /* void* */, file: *u8 /* void* */, type_: c_int /* int */);
    pub fn wxIconBundle_Assign(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxIconBundle_CreateDefault() -> *u8 /* void* */;
    pub fn wxIconBundle_CreateFromFile(file: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxIconBundle_CreateFromIcon(icon: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxIconBundle_Delete(_obj: *u8 /* void* */);
    pub fn wxIconBundle_GetIcon(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _ref: *u8 /* void* */);
    
    // TClassDefExtend(wxIconizeEvent,wxEvent)
    
    // TClassDefExtend(wxIdleEvent,wxEvent)
    pub fn wxIdleEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxIdleEvent_MoreRequested(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxIdleEvent_RequestMore(_obj: *u8 /* void* */, needMore: bool /* bool */);
    
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
    
    // TClassDefExtend(wxImageHandler,wxObject)
    
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
    
    // TClassDefExtend(wxInitDialogEvent,wxEvent)
    
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
    
    // TClassDefExtend(wxJoystick,wxObject)
    // missing: wxJoystick_Create
    // missing: wxJoystick_Delete
    // missing: wxJoystick_GetButtonState
    // missing: wxJoystick_GetManufacturerId
    // missing: wxJoystick_GetMaxAxes
    // missing: wxJoystick_GetMaxButtons
    // missing: wxJoystick_GetMovementThreshold
    // missing: wxJoystick_GetNumberAxes
    // missing: wxJoystick_GetNumberButtons
    // missing: wxJoystick_GetNumberJoysticks
    // missing: wxJoystick_GetPOVCTSPosition
    // missing: wxJoystick_GetPOVPosition
    // missing: wxJoystick_GetPollingMax
    // missing: wxJoystick_GetPollingMin
    // missing: wxJoystick_GetPosition
    // missing: wxJoystick_GetProductId
    // missing: wxJoystick_GetProductName
    // missing: wxJoystick_GetRudderMax
    // missing: wxJoystick_GetRudderMin
    // missing: wxJoystick_GetRudderPosition
    // missing: wxJoystick_GetUMax
    // missing: wxJoystick_GetUMin
    // missing: wxJoystick_GetUPosition
    // missing: wxJoystick_GetVMax
    // missing: wxJoystick_GetVMin
    // missing: wxJoystick_GetVPosition
    // missing: wxJoystick_GetXMax
    // missing: wxJoystick_GetXMin
    // missing: wxJoystick_GetYMax
    // missing: wxJoystick_GetYMin
    // missing: wxJoystick_GetZMax
    // missing: wxJoystick_GetZMin
    // missing: wxJoystick_GetZPosition
    // missing: wxJoystick_HasPOV
    // missing: wxJoystick_HasPOV4Dir
    // missing: wxJoystick_HasPOVCTS
    // missing: wxJoystick_HasRudder
    // missing: wxJoystick_HasU
    // missing: wxJoystick_HasV
    // missing: wxJoystick_HasZ
    // missing: wxJoystick_IsOk
    // missing: wxJoystick_ReleaseCapture
    // missing: wxJoystick_SetCapture
    // missing: wxJoystick_SetMovementThreshold
    
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
    
    // TClassDefExtend(wxLEDNumberCtrl,wxControl)
    // missing: wxLEDNumberCtrl_Create
    // missing: wxLEDNumberCtrl_GetAlignment
    // missing: wxLEDNumberCtrl_GetDrawFaded
    // missing: wxLEDNumberCtrl_GetValue
    // missing: wxLEDNumberCtrl_SetAlignment
    // missing: wxLEDNumberCtrl_SetDrawFaded
    // missing: wxLEDNumberCtrl_SetValue
    
    // TClassDefExtend(wxLayoutAlgorithm,wxObject)
    pub fn wxLayoutAlgorithm_Create() -> *u8 /* void* */;
    pub fn wxLayoutAlgorithm_Delete(_obj: *u8 /* void* */);
    pub fn wxLayoutAlgorithm_LayoutFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLayoutAlgorithm_LayoutMDIFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, use_: c_int /* int */) -> bool /* bool */;
    pub fn wxLayoutAlgorithm_LayoutWindow(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */;
    
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
    
    // TClassDefExtend(wxList,wxObject)
    
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
    
    // TClassDef(wxLog)
    
    // TClassDefExtend(wxLogChain,wxLog)
    pub fn wxLogChain_Create(logger: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLogChain_Delete(_obj: *u8 /* void* */);
    pub fn wxLogChain_GetOldLog(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxLogChain_IsPassingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxLogChain_PassMessages(_obj: *u8 /* void* */, bDoPass: bool /* bool */);
    pub fn wxLogChain_SetLog(_obj: *u8 /* void* */, logger: *u8 /* void* */);
    
    // TClassDefExtend(wxLogGUI,wxLog)
    
    // TClassDefExtend(wxLogNull,wxLog)
    
    // TClassDefExtend(wxLogPassThrough,wxLogChain)
    
    // TClassDefExtend(wxLogStderr,wxLog)
    
    // TClassDefExtend(wxLogStream,wxLog)
    
    // TClassDefExtend(wxLogTextCtrl,wxLog)
    
    // TClassDefExtend(wxLogWindow,wxLogPassThrough)
    
    // TClassDef(wxLongLong)
    
    // TClassDef(wxMBConv)
    
    // TClassDefExtend(wxMBConvFile,wxMBConv)
    
    // TClassDefExtend(wxMBConvUTF7,wxMBConv)
    
    // TClassDefExtend(wxMBConvUTF8,wxMBConv)
    
    // TClassDefExtend(wxMDIChildFrame,wxFrame)
    pub fn wxMDIChildFrame_Activate(_obj: *u8 /* void* */);
    pub fn wxMDIChildFrame_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxMDIClientWindow,wxWindow)
    
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
    
    // TClassDefExtend(wxMask,wxObject)
    pub fn wxMask_Create(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMask_CreateColoured(bitmap: *u8 /* void* */, colour: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxMaximizeEvent,wxEvent)
    
    // TClassDefExtend(wxMemoryDC,wxDC)
    pub fn wxMemoryDC_Create() -> *u8 /* void* */;
    pub fn wxMemoryDC_CreateCompatible(dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMemoryDC_CreateWithBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMemoryDC_Delete(_obj: *u8 /* void* */);
    pub fn wxMemoryDC_SelectObject(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    
    // TClassDefExtend(wxMemoryFSHandler,wxFileSystemHandler)
    
    // TClassDefExtend(wxMemoryInputStream,wxInputStream)
    
    // TClassDefExtend(wxMemoryOutputStream,wxOutputStream)
    
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
    
    // TClassDefExtend(wxMenuEvent,wxEvent)
    pub fn wxMenuEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxMenuEvent_GetMenuId(_obj: *u8 /* void* */) -> c_int /* int */;
    
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
    
    // TClassDefExtend(wxMessageDialog,wxDialog)
    pub fn wxMessageDialog_Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _cap: *u8 /* void* */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxMessageDialog_Delete(_obj: *u8 /* void* */);
    pub fn wxMessageDialog_ShowModal(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxMetafile,wxObject)
    pub fn wxMetafile_Create(_file: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMetafile_Delete(_obj: *u8 /* void* */);
    pub fn wxMetafile_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMetafile_Play(_obj: *u8 /* void* */, _dc: *u8 /* void* */) -> bool /* bool */;
    pub fn wxMetafile_SetClipboard(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    
    // TClassDefExtend(wxMetafileDC,wxDC)
    pub fn wxMetafileDC_Close(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMetafileDC_Create(_file: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMetafileDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDef(wxMimeTypesManager)
    pub fn wxMimeTypesManager_AddFallbacks(_obj: *u8 /* void* */, _types: *u8 /* void* */);
    pub fn wxMimeTypesManager_Create() -> *u8 /* void* */;
    pub fn wxMimeTypesManager_EnumAllFileTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    pub fn wxMimeTypesManager_GetFileTypeFromExtension(_obj: *u8 /* void* */, _ext: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMimeTypesManager_GetFileTypeFromMimeType(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMimeTypesManager_IsOfType(_obj: *u8 /* void* */, _type: *u8 /* void* */, _wildcard: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxMiniFrame,wxFrame)
    pub fn wxMiniFrame_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxMirrorDC,wxDC)
    pub fn wxMirrorDC_Create(dc: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxMirrorDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxModule,wxObject)
    
    // TClassDefExtend(wxMouseCaptureChangedEvent,wxEvent)
    
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
    
    // TClassDefExtend(wxMoveEvent,wxEvent)
    pub fn wxMoveEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxMoveEvent_GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxMultiCellCanvas,wxFlexGridSizer)
    // missing: wxMultiCellCanvas_Add
    // missing: wxMultiCellCanvas_CalculateConstraints
    // missing: wxMultiCellCanvas_Create
    // missing: wxMultiCellCanvas_MaxCols
    // missing: wxMultiCellCanvas_MaxRows
    // missing: wxMultiCellCanvas_SetMinCellSize
    
    // TClassDefExtend(wxMultiCellItemHandle,wxObject)
    // missing: wxMultiCellItemHandle_Create
    // missing: wxMultiCellItemHandle_CreateWithSize
    // missing: wxMultiCellItemHandle_CreateWithStyle
    // missing: wxMultiCellItemHandle_GetAlignment
    // missing: wxMultiCellItemHandle_GetColumn
    // missing: wxMultiCellItemHandle_GetHeight
    // missing: wxMultiCellItemHandle_GetLocalSize
    // missing: wxMultiCellItemHandle_GetRow
    // missing: wxMultiCellItemHandle_GetStyle
    // missing: wxMultiCellItemHandle_GetWeight
    // missing: wxMultiCellItemHandle_GetWidth
    
    // TClassDefExtend(wxMultiCellSizer,wxSizer)
    // missing: wxMultiCellSizer_CalcMin
    // missing: wxMultiCellSizer_Create
    // missing: wxMultiCellSizer_Delete
    // missing: wxMultiCellSizer_EnableGridLines
    // missing: wxMultiCellSizer_RecalcSizes
    // missing: wxMultiCellSizer_SetColumnWidth
    // missing: wxMultiCellSizer_SetDefaultCellSize
    // missing: wxMultiCellSizer_SetGridPen
    // missing: wxMultiCellSizer_SetRowHeight
    
    // TClassDef(wxMutex)
    // missing: wxMutex_Create
    // missing: wxMutex_Delete
    // missing: wxMutex_IsLocked
    // missing: wxMutex_Lock
    // missing: wxMutex_TryLock
    // missing: wxMutex_Unlock
    
    // TClassDef(wxMutexLocker)
    
    // TClassDefExtend(wxNavigationKeyEvent,wxEvent)
    pub fn wxNavigationKeyEvent_GetCurrentFocus(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxNavigationKeyEvent_GetDirection(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNavigationKeyEvent_IsWindowChange(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNavigationKeyEvent_SetCurrentFocus(_obj: *u8 /* void* */, win: *u8 /* void* */);
    pub fn wxNavigationKeyEvent_SetDirection(_obj: *u8 /* void* */, bForward: bool /* bool */);
    pub fn wxNavigationKeyEvent_SetWindowChange(_obj: *u8 /* void* */, bIs: bool /* bool */);
    pub fn wxNavigationKeyEvent_ShouldPropagate(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxNewBitmapButton,wxPanel)
    // missing: wxNewBitmapButton_Create
    // missing: wxNewBitmapButton_CreateFromFile
    // missing: wxNewBitmapButton_Delete
    // missing: wxNewBitmapButton_DrawDecorations
    // missing: wxNewBitmapButton_DrawLabel
    // missing: wxNewBitmapButton_Enable
    // missing: wxNewBitmapButton_Realize
    // missing: wxNewBitmapButton_RenderAllLabelImages
    // missing: wxNewBitmapButton_RenderLabelImage
    // missing: wxNewBitmapButton_RenderLabelImages
    // missing: wxNewBitmapButton_Reshape
    // missing: wxNewBitmapButton_SetAlignments
    // missing: wxNewBitmapButton_SetLabel
    
    // TClassDef(wxNodeBase)
    
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
    
    // TClassDefExtend(wxNotebookEvent,wxNotifyEvent)
    
    // TClassDefExtend(wxNotifyEvent,wxCommandEvent)
    pub fn wxNotifyEvent_Allow(_obj: *u8 /* void* */);
    pub fn wxNotifyEvent_CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    pub fn wxNotifyEvent_IsAllowed(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNotifyEvent_Veto(_obj: *u8 /* void* */);
    
    // TClassDef(wxObject)
    
    // TClassDef(wxObjectRefData)
    
    // TClassDefExtend(wxOutputStream,wxStreamBase)
    pub fn wxOutputStream_Delete(_obj: *u8 /* void* */);
    pub fn wxOutputStream_LastWrite(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxOutputStream_PutC(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */);
    pub fn wxOutputStream_Seek(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */;
    pub fn wxOutputStream_Sync(_obj: *u8 /* void* */);
    pub fn wxOutputStream_Tell(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxOutputStream_Write(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */);
    
    // TClassDefExtend(wxPageSetupDialog,wxDialog)
    pub fn wxPageSetupDialog_Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPageSetupDialog_GetPageSetupData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    
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
    
    // TClassDefExtend(wxPaintDC,wxWindowDC)
    pub fn wxPaintDC_Create(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPaintDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxPaintEvent,wxEvent)
    
    // TClassDefExtend(wxPalette,wxGDIObject)
    pub fn wxPalette_Assign(_obj: *u8 /* void* */, palette: *u8 /* void* */);
    pub fn wxPalette_CreateDefault() -> *u8 /* void* */;
    pub fn wxPalette_CreateRGB(n: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPalette_Delete(_obj: *u8 /* void* */);
    pub fn wxPalette_GetPixel(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) -> c_int /* int */;
    pub fn wxPalette_GetRGB(_obj: *u8 /* void* */, pixel: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPalette_IsEqual(_obj: *u8 /* void* */, palette: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPalette_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxPaletteChangedEvent,wxEvent)
    pub fn wxPaletteChangedEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxPaletteChangedEvent_GetChangedWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPaletteChangedEvent_SetChangedWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    
    // TClassDefExtend(wxPanel,wxWindow)
    pub fn wxPanel_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPanel_InitDialog(_obj: *u8 /* void* */);
    pub fn wxPanel_SetFocus(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxPathList,wxList)
    
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
    
    // TClassDefExtend(wxPenList,wxList)
    
    // TClassDefExtend(wxPlotCurve,wxObject)
    
    // TClassDefExtend(wxPlotEvent,wxNotifyEvent)
    // missing: wxPlotEvent_GetCurve
    // missing: wxPlotEvent_GetPosition
    // missing: wxPlotEvent_GetZoom
    // missing: wxPlotEvent_SetPosition
    // missing: wxPlotEvent_SetZoom
    
    // TClassDefExtend(wxPlotOnOffCurve,wxObject)
    // missing: wxPlotOnOffCurve_Add
    // missing: wxPlotOnOffCurve_Create
    // missing: wxPlotOnOffCurve_Delete
    // missing: wxPlotOnOffCurve_DrawOffLine
    // missing: wxPlotOnOffCurve_DrawOnLine
    // missing: wxPlotOnOffCurve_GetAt
    // missing: wxPlotOnOffCurve_GetClientData
    // missing: wxPlotOnOffCurve_GetCount
    // missing: wxPlotOnOffCurve_GetEndX
    // missing: wxPlotOnOffCurve_GetOff
    // missing: wxPlotOnOffCurve_GetOffsetY
    // missing: wxPlotOnOffCurve_GetOn
    // missing: wxPlotOnOffCurve_GetStartX
    // missing: wxPlotOnOffCurve_SetOffsetY
    
    // TClassDefExtend(wxPlotWindow,wxScrolledWindow)
    // missing: wxPlotWindow_Add
    // missing: wxPlotWindow_AddOnOff
    // missing: wxPlotWindow_Create
    // missing: wxPlotWindow_Delete
    // missing: wxPlotWindow_DeleteOnOff
    // missing: wxPlotWindow_Enlarge
    // missing: wxPlotWindow_GetAt
    // missing: wxPlotWindow_GetCount
    // missing: wxPlotWindow_GetCurrent
    // missing: wxPlotWindow_GetEnlargeAroundWindowCentre
    // missing: wxPlotWindow_GetOnOffCurveAt
    // missing: wxPlotWindow_GetOnOffCurveCount
    // missing: wxPlotWindow_GetScrollOnThumbRelease
    // missing: wxPlotWindow_GetUnitsPerValue
    // missing: wxPlotWindow_GetZoom
    // missing: wxPlotWindow_Move
    // missing: wxPlotWindow_RedrawEverything
    // missing: wxPlotWindow_RedrawXAxis
    // missing: wxPlotWindow_RedrawYAxis
    // missing: wxPlotWindow_ResetScrollbar
    // missing: wxPlotWindow_SetCurrent
    // missing: wxPlotWindow_SetEnlargeAroundWindowCentre
    // missing: wxPlotWindow_SetScrollOnThumbRelease
    // missing: wxPlotWindow_SetUnitsPerValue
    // missing: wxPlotWindow_SetZoom
    
    // TClassDef(wxPoint)
    pub fn wxPoint_Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    // missing: wxPoint_Destroy
    pub fn wxPoint_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPoint_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxPoint_SetX(_obj: *u8 /* void* */, w: c_int /* int */);
    pub fn wxPoint_SetY(_obj: *u8 /* void* */, h: c_int /* int */);
    
    // TClassDefExtend(wxPopupTransientWindow,wxPopupWindow)
    
    // TClassDefExtend(wxPopupWindow,wxWindow)
    
    // TClassDefExtend(wxPostScriptDC,wxDC)
    pub fn wxPostScriptDC_Create(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPostScriptDC_Delete(self_: *u8 /* void* */);
    pub fn wxPostScriptDC_SetResolution(self_: *u8 /* void* */, ppi: c_int /* int */);
    pub fn wxPostScriptDC_GetResolution(self_: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxPreviewCanvas,wxScrolledWindow)
    pub fn wxPreviewCanvas_Create(preview: *u8 /* void* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPreviewControlBar,wxPanel)
    
    // TClassDefExtend(wxPreviewFrame,wxFrame)
    
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
    
    // TClassDefExtend(wxPostScriptPrintNativeData,wxObject)
    pub fn wxPostScriptPrintNativeData_Create() -> *u8 /* void* */;
    pub fn wxPostScriptPrintNativeData_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxPrintDialog,wxDialog)
    pub fn wxPrintDialog_Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintDialog_GetPrintDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrintDialog_GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxPrintDialog_GetPrintDialogData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
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
    
    // TClassDefExtend(wxPrinterDC,wxDC)
    pub fn wxPrinterDC_Create(data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPrinterDC_Delete(self_: *u8 /* void* */);
    pub fn wxPrinterDC_GetPaperRect(self_: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPrintout,wxObject)
    
    // TClassDefExtend(wxPrivateDropTarget,wxDropTarget)
    
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
    
    // TClassDefExtend(wxProcessEvent,wxEvent)
    pub fn wxProcessEvent_GetExitCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxProcessEvent_GetPid(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxProgressDialog,wxFrame)
    
    // TClassDefExtend(wxProtocol,wxSocketClient)
    
    // TClassDefExtend(wxQuantize,wxObject)
    
    // TClassDefExtend(wxQueryCol,wxObject)
    
    // TClassDefExtend(wxQueryField,wxObject)
    
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
    
    // TClassDefExtend(wxQueryNewPaletteEvent,wxEvent)
    pub fn wxQueryNewPaletteEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxQueryNewPaletteEvent_GetPaletteRealized(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxQueryNewPaletteEvent_SetPaletteRealized(_obj: *u8 /* void* */, realized: bool /* bool */);
    
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
    
    // TClassDefExtend(wxRadioButton,wxControl)
    pub fn wxRadioButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxRadioButton_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxRadioButton_SetValue(_obj: *u8 /* void* */, value: bool /* bool */);
    
    // TClassDef(wxRealPoint)
    
    // TClassDefExtend(wxRecordSet,wxObject)
    
    // TClassDef(wxRect)
    
    // TClassDef(wxRegEx)
    
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
    
    // TClassDefExtend(wxRemotelyScrolledTreeCtrl,wxTreeCtrl)
    // missing: wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars
    // missing: wxRemotelyScrolledTreeCtrl_CalcTreeSize
    // missing: wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem
    // missing: wxRemotelyScrolledTreeCtrl_Create
    // missing: wxRemotelyScrolledTreeCtrl_Delete
    // missing: wxRemotelyScrolledTreeCtrl_GetCompanionWindow
    // missing: wxRemotelyScrolledTreeCtrl_GetScrollPos
    // missing: wxRemotelyScrolledTreeCtrl_GetScrolledWindow
    // missing: wxRemotelyScrolledTreeCtrl_GetViewStart
    // missing: wxRemotelyScrolledTreeCtrl_HideVScrollbar
    // missing: wxRemotelyScrolledTreeCtrl_PrepareDC
    // missing: wxRemotelyScrolledTreeCtrl_ScrollToLine
    // missing: wxRemotelyScrolledTreeCtrl_SetCompanionWindow
    // missing: wxRemotelyScrolledTreeCtrl_SetScrollbars
    
    // TClassDefExtend(wxSVGFileDC,wxDC)
    pub fn wxSVGFileDC_Create(fileName: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSVGFileDC_CreateWithSize(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSVGFileDC_CreateWithSizeAndResolution(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, a_dpi: float /* float */) -> *u8 /* void* */;
    pub fn wxSVGFileDC_Delete(obj: *u8 /* void* */);
    
    // TClassDefExtend(wxSashEvent,wxEvent)
    pub fn wxSashEvent_Create(id: c_int /* int */, edge: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSashEvent_GetDragRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSashEvent_GetDragStatus(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashEvent_GetEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashEvent_SetDragRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxSashEvent_SetDragStatus(_obj: *u8 /* void* */, status: c_int /* int */);
    pub fn wxSashEvent_SetEdge(_obj: *u8 /* void* */, edge: c_int /* int */);
    
    // TClassDefExtend(wxSashLayoutWindow,wxSashWindow)
    pub fn wxSashLayoutWindow_Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSashLayoutWindow_GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashLayoutWindow_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSashLayoutWindow_SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */);
    pub fn wxSashLayoutWindow_SetDefaultSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    pub fn wxSashLayoutWindow_SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    
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
    
    // TClassDef(wxScopedArray)
    
    // TClassDef(wxScopedPtr)
    
    // TClassDefExtend(wxScreenDC,wxDC)
    pub fn wxScreenDC_Create() -> *u8 /* void* */;
    pub fn wxScreenDC_Delete(_obj: *u8 /* void* */);
    pub fn wxScreenDC_EndDrawingOnTop(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxScreenDC_StartDrawingOnTop(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    pub fn wxScreenDC_StartDrawingOnTopOfWin(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxScrollBar,wxControl)
    pub fn wxScrollBar_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxScrollBar_GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_GetRange(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_GetThumbPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_GetThumbSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollBar_SetScrollbar(_obj: *u8 /* void* */, position: c_int /* int */, thumbSize: c_int /* int */, range: c_int /* int */, pageSize: c_int /* int */, refresh: bool /* bool */);
    pub fn wxScrollBar_SetThumbPosition(_obj: *u8 /* void* */, viewStart: c_int /* int */);
    
    // TClassDefExtend(wxScrollEvent,wxEvent)
    pub fn wxScrollEvent_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxScrollWinEvent,wxEvent)
    pub fn wxScrollWinEvent_GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollWinEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxScrollWinEvent_SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    pub fn wxScrollWinEvent_SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
    
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
    
    // TClassDef(wxSemaphore)
    
    // TClassDefExtend(wxServer,wxServerBase)
    
    // TClassDefExtend(wxServerBase,wxObject)
    
    // TClassDefExtend(wxSetCursorEvent,wxEvent)
    pub fn wxSetCursorEvent_GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxSetCursorEvent_GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSetCursorEvent_GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSetCursorEvent_HasCursor(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSetCursorEvent_SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */);
    
    // TClassDefExtend(wxShowEvent,wxEvent)
    pub fn wxShowEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxShowEvent_IsShown(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxShowEvent_SetShow(_obj: *u8 /* void* */, show: bool /* bool */);
    
    // TClassDefExtend(wxSimpleHelpProvider,wxHelpProvider)
    pub fn wxSimpleHelpProvider_Create() -> *u8 /* void* */;
    
    // TClassDefExtend(wxSingleChoiceDialog,wxDialog)
    
    // TClassDef(wxSingleInstanceChecker)
    pub fn wxSingleInstanceChecker_Create(_obj: *u8 /* void* */, name: *u8 /* void* */, path: *u8 /* void* */) -> bool /* bool */;
    pub fn wxSingleInstanceChecker_CreateDefault() -> *u8 /* void* */;
    pub fn wxSingleInstanceChecker_Delete(_obj: *u8 /* void* */);
    pub fn wxSingleInstanceChecker_IsAnotherRunning(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxSize)
    pub fn wxSize_Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    // missing: wxSize_Destroy
    pub fn wxSize_GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSize_GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSize_SetHeight(_obj: *u8 /* void* */, h: c_int /* int */);
    pub fn wxSize_SetWidth(_obj: *u8 /* void* */, w: c_int /* int */);
    
    // TClassDefExtend(wxSizeEvent,wxEvent)
    pub fn wxSizeEvent_CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    pub fn wxSizeEvent_GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
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
    
    // TClassDefExtend(wxSockAddress,wxObject)
    
    // TClassDefExtend(wxSocketBase,wxObject)
    
    // TClassDefExtend(wxSocketClient,wxSocketBase)
    
    // TClassDefExtend(wxSocketEvent,wxEvent)
    
    // TClassDefExtend(wxSocketInputStream,wxInputStream)
    
    // TClassDefExtend(wxSocketOutputStream,wxOutputStream)
    
    // TClassDefExtend(wxSocketServer,wxSocketBase)
    
    // TClassDefExtend(wxSpinButton,wxControl)
    pub fn wxSpinButton_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    pub fn wxSpinButton_GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinButton_GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinButton_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinButton_SetRange(_obj: *u8 /* void* */, minVal: c_int /* int */, maxVal: c_int /* int */);
    pub fn wxSpinButton_SetValue(_obj: *u8 /* void* */, val: c_int /* int */);
    
    // TClassDefExtend(wxSpinCtrl,wxControl)
    pub fn wxSpinCtrl_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */, _min: c_int /* int */, _max: c_int /* int */, _init: c_int /* int */) -> *u8 /* void* */;
    pub fn wxSpinCtrl_GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinCtrl_GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinCtrl_GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinCtrl_SetRange(_obj: *u8 /* void* */, min_val: c_int /* int */, max_val: c_int /* int */);
    pub fn wxSpinCtrl_SetValue(_obj: *u8 /* void* */, val: c_int /* int */);
    
    // TClassDefExtend(wxSpinEvent,wxNotifyEvent)
    pub fn wxSpinEvent_GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxSpinEvent_SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
    
    // TClassDefExtend(wxSplashScreen,wxFrame)
    
    // TClassDefExtend(wxSplitterEvent,wxNotifyEvent)
    
    // TClassDefExtend(wxSplitterScrolledWindow,wxScrolledWindow)
    // missing: wxSplitterScrolledWindow_Create
    
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
    
    // TClassDefExtend(wxStaticBitmap,wxControl)
    pub fn wxStaticBitmap_Create(_prt: *u8 /* void* */, _id: c_int /* int */, bitmap: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStaticBitmap_Delete(_obj: *u8 /* void* */);
    pub fn wxStaticBitmap_GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxStaticBitmap_GetIcon(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxStaticBitmap_SetBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    pub fn wxStaticBitmap_SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */);
    
    // TClassDefExtend(wxStaticBox,wxControl)
    pub fn wxStaticBox_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxStaticBoxSizer,wxBoxSizer)
    pub fn wxStaticBoxSizer_CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStaticBoxSizer_Create(box: *u8 /* void* */, orient: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStaticBoxSizer_GetStaticBox(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxStaticBoxSizer_RecalcSizes(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxStaticLine,wxControl)
    pub fn wxStaticLine_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxStaticLine_GetDefaultSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStaticLine_IsVertical(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxStaticText,wxControl)
    pub fn wxStaticText_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    
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
    
    // TClassDef(wxStopWatch)
    pub fn wxStopWatch_Create() -> *u8 /* void* */;
    pub fn wxStopWatch_Delete(_obj: *u8 /* void* */);
    pub fn wxStopWatch_Start(_obj: *u8 /* void* */, msec: c_int /* int */);
    pub fn wxStopWatch_Pause(_obj: *u8 /* void* */);
    pub fn wxStopWatch_Resume(_obj: *u8 /* void* */);
    pub fn wxStopWatch_Time(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDef(wxStreamBase)
    pub fn wxStreamBase_GetLastError(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStreamBase_GetSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxStreamBase_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxStreamBuffer)
    
    // TClassDef(wxStreamToTextRedirector)
    
    // TClassDef(wxString)
    
    // TClassDef(wxStringBuffer)
    
    // TClassDefExtend(wxStringClientData,wxClientData)
    
    // TClassDefExtend(wxStringList,wxList)
    
    // TClassDefExtend(wxStringTokenizer,wxObject)
    
    // TClassDefExtend(wxSysColourChangedEvent,wxEvent)
    
    // TClassDefExtend(wxSystemOptions,wxObject)
    
    // TClassDefExtend(wxSystemSettings,wxObject)
    pub fn wxSystemSettings_GetColour(index: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxSystemSettings_GetFont(index: c_int /* int */, _ref: *u8 /* void* */);
    pub fn wxSystemSettings_GetMetric(index: c_int /* int */) -> c_int /* int */;
    pub fn wxSystemSettings_GetScreenType() -> c_int /* int */;
    
    // TClassDefExtend(wxTabCtrl,wxControl)
    
    // TClassDefExtend(wxTabEvent,wxCommandEvent)
    
    // TClassDefExtend(wxTablesInUse,wxObject)
    
    // TClassDefExtend(wxTaskBarIcon,wxEvtHandler)
    pub fn wxTaskBarIcon_Create() -> *u8 /* void* */;
    pub fn wxTaskBarIcon_Delete(_obj: *u8 /* void* */);
    pub fn wxTaskBarIcon_IsIconInstalled(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_RemoveIcon(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTaskBarIcon_SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, text: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxTempFile)
    
    // TClassDef(wxTextAttr)
    
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
    
    // TClassDefExtend(wxTextDataObject,wxDataObjectSimple)
    pub fn TextDataObject_Create(_txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn TextDataObject_Delete(_obj: *u8 /* void* */);
    pub fn TextDataObject_GetTextLength(_obj: *u8 /* void* */) -> size_t /* size_t */;
    pub fn TextDataObject_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn TextDataObject_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    
    // TClassDefExtend(wxTextDropTarget,wxDropTarget)
    
    // TClassDefExtend(wxTextEntryDialog,wxDialog)
    
    // TClassDef(wxTextFile)
    
    // TClassDef(wxTextInputStream)
    
    // TClassDef(wxTextOutputStream)
    
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
    
    // TClassDefExtend(wxThinSplitterWindow,wxSplitterWindow)
    // missing: wxThinSplitterWindow_Create
    // missing: wxThinSplitterWindow_DrawSash
    // missing: wxThinSplitterWindow_SashHitTest
    // missing: wxThinSplitterWindow_SizeWindows
    
    // TClassDef(wxThread)
    
    // TClassDefExtend(wxTime,wxObject)
    
    // TClassDef(wxTimeSpan)
    
    // TClassDefExtend(wxTimer,wxObject)
    pub fn wxTimer_Create(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTimer_Delete(_obj: *u8 /* void* */);
    pub fn wxTimer_GetInterval(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTimer_IsOneShot(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTimer_IsRuning(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxTimer_Start(_obj: *u8 /* void* */, _int: c_int /* int */, _one: bool /* bool */) -> bool /* bool */;
    pub fn wxTimer_Stop(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxTimerBase,wxObject)
    
    // TClassDefExtend(wxTimerEvent,wxEvent)
    pub fn wxTimerEvent_GetInterval(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxTimerEx,wxTimer)
    
    // TClassDef(wxTimerRunner)
    
    // TClassDef(wxTipProvider)
    
    // TClassDefExtend(wxTipWindow,wxPopupTransientWindow)
    pub fn wxTipWindow_Close(_obj: *u8 /* void* */);
    pub fn wxTipWindow_Create(parent: *u8 /* void* */, text: *u8 /* void* */, maxLength: c_int /* int */) -> *u8 /* void* */;
    pub fn wxTipWindow_SetBoundingRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    pub fn wxTipWindow_SetTipWindowPtr(_obj: *u8 /* void* */, windowPtr: *u8 /* void* */);
    
    // TClassDefExtend(wxToggleButton,wxControl)
    pub fn wxToggleButton_Create(parent: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    pub fn wxToggleButton_Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    pub fn wxToggleButton_GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxToggleButton_SetLabel(_obj: *u8 /* void* */, label: *u8 /* void* */);
    pub fn wxToggleButton_SetValue(_obj: *u8 /* void* */, state: bool /* bool */);
    
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
    
    // TClassDefExtend(wxToolBarBase,wxControl)
    
    // TClassDefExtend(wxToolLayoutItem,wxObject)
    // missing: wxToolLayoutItem_IsSeparator
    // missing: wxToolLayoutItem_Rect
    
    // TClassDefExtend(wxToolTip,wxObject)
    
    // TClassDefExtend(wxToolWindow,wxFrame)
    // missing: wxToolWindow_AddMiniButton
    // missing: wxToolWindow_Create
    // missing: wxToolWindow_GetClient
    // missing: wxToolWindow_SetClient
    // missing: wxToolWindow_SetTitleFont
    
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
    
    // TClassDefExtend(wxTreeCompanionWindow,wxWindow)
    // missing: wxTreeCompanionWindow_Create
    // missing: wxTreeCompanionWindow_DrawItem
    // missing: wxTreeCompanionWindow_GetTreeCtrl
    // missing: wxTreeCompanionWindow_SetTreeCtrl
    
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
    
    // TClassDefExtend(wxTreeEvent,wxNotifyEvent)
    pub fn wxTreeEvent_GetCode(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxTreeEvent_GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxTreeEvent_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxTreeEvent_GetOldItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxTreeEvent_GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxTreeItemData,wxClientData)
    
    // TClassDef(wxTreeItemId)
    pub fn wxTreeItemId_Create() -> *u8 /* void* */;
    pub fn wxTreeItemId_Delete(_obj: *u8 /* void* */);
    pub fn wxTreeItemId_IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxTreeLayout,wxObject)
    
    // TClassDefExtend(wxTreeLayoutStored,wxTreeLayout)
    
    // TClassDefExtend(wxURL,wxObject)
    
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
    
    // TClassDefExtend(wxValidator,wxEvtHandler)
    pub fn wxValidator_Create() -> *u8 /* void* */;
    pub fn wxValidator_Delete(_obj: *u8 /* void* */);
    pub fn wxValidator_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxValidator_SetBellOnError(doIt: bool /* bool */);
    pub fn wxValidator_SetWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    pub fn wxValidator_TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxValidator_TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxValidator_Validate(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxVariant,wxObject)
    
    // TClassDefExtend(wxVariantData,wxObject)
    
    // TClassDefExtend(wxView,wxEvtHandler)
    
    // TClassDefExtend(wxSound,wxEvtHandler)
    
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
    
    // TClassDefExtend(wxWindowCreateEvent,wxCommandEvent)
    pub fn wxWindowCreateEvent_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxWindowDC,wxDC)
    pub fn wxWindowDC_Create(win: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWindowDC_Delete(_obj: *u8 /* void* */);
    
    // TClassDefExtend(wxWindowDestroyEvent,wxCommandEvent)
    pub fn wxWindowDestroyEvent_GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDef(wxWindowDisabler)
    
    // TClassDefExtend(wxWizard,wxDialog)
    pub fn wxWizard_Chain(f: *u8 /* void* */, s: *u8 /* void* */);
    pub fn wxWizard_Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */;
    pub fn wxWizard_GetCurrentPage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizard_GetPageSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizard_RunWizard(_obj: *u8 /* void* */, firstPage: *u8 /* void* */) -> c_int /* int */;
    pub fn wxWizard_SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    
    // TClassDefExtend(wxWizardEvent,wxNotifyEvent)
    pub fn wxWizardEvent_GetDirection(_obj: *u8 /* void* */) -> c_int /* int */;
    
    // TClassDefExtend(wxWizardPage,wxPanel)
    
    // TClassDefExtend(wxWizardPageSimple,wxWizardPage)
    pub fn wxWizardPageSimple_Create(_prt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizardPageSimple_GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    pub fn wxWizardPageSimple_GetNext(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizardPageSimple_GetPrev(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxWizardPageSimple_SetNext(_obj: *u8 /* void* */, next: *u8 /* void* */);
    pub fn wxWizardPageSimple_SetPrev(_obj: *u8 /* void* */, prev: *u8 /* void* */);
    
    // TClassDefExtend(wxXmlResource,wxObject)
    pub fn wxXmlResource_AddHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    pub fn wxXmlResource_AddSubclassFactory(_obj: *u8 /* void* */, factory: *u8 /* void* */);
    pub fn wxXmlResource_AttachUnknownControl(_obj: *u8 /* void* */, control: *u8 /* void* */, parent: *u8 /* void* */) -> c_int /* int */;
    pub fn wxXmlResource_ClearHandlers(_obj: *u8 /* void* */);
    pub fn wxXmlResource_CompareVersion(_obj: *u8 /* void* */, major: c_int /* int */, minor: c_int /* int */, release: c_int /* int */, revision: c_int /* int */) -> c_int /* int */;
    pub fn wxXmlResource_Create(flags: c_int /* int */) -> *u8 /* void* */;
    pub fn wxXmlResource_CreateFromFile(filemask: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */;
    // missing: wxXmlResource_Delete
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
    
    // TClassDefExtend(wxXmlResourceHandler,wxObject)
    
    // TClassDefExtend(wxZipInputStream,wxInputStream)
    
    // TClassDefExtend(wxZlibInputStream,wxFilterInputStream)
    
    // TClassDefExtend(wxZlibOutputStream,wxFilterOutputStream)
    
    // TClassDefExtend(wxPropertyGrid,wxControl)
    pub fn wxPropertyGrid_Append(_obj: *u8 /* void* */, prop: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPropertyGrid_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    pub fn wxPropertyGrid_DisableProperty(_obj: *u8 /* void* */, propName: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDefExtend(wxPropertyGridEvent,wxNotifyEvent)
    pub fn wxPropertyGridEvent_HasProperty(_obj: *u8 /* void* */) -> bool /* bool */;
    pub fn wxPropertyGridEvent_GetProperty(_obj: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPGProperty,wxObject)
    pub fn wxPGProperty_GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_GetValueAsString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_GetValueType(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxPGProperty_SetHelpString(_obj: *u8 /* void* */, helpString: *u8 /* void* */);
    
    // TClassDefExtend(wxStringProperty,wxPGProperty)
    pub fn wxStringProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxIntProperty,wxPGProperty)
    pub fn wxIntProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxBoolProperty,wxPGProperty)
    pub fn wxBoolProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: bool /* bool */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxFloatProperty,wxPGProperty)
    pub fn wxFloatProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: float /* float */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxDateProperty,wxPGProperty)
    pub fn wxDateProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxFileProperty,wxPGProperty)
    pub fn wxFileProperty_Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxPropertyCategory,wxPGProperty)
    pub fn wxPropertyCategory_Create(label: *u8 /* void* */) -> *u8 /* void* */;
    
    // TClassDefExtend(wxClosure,wxObject)
    pub fn wxClosure_Create(_fun_CEvent: *u8 /* void* */, _data: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClosure_GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvtHandler_GetClosure(_obj: *u8 /* void* */, id: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    pub fn wxEvtHandler_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxEvtHandler_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    pub fn wxObject_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxObject_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    
    // TClassDefExtend(wxGauge95,wxGauge)
    
    // TClassDefExtend(wxGaugeMSW,wxGauge)
    
    // TClassDefExtend(wxSlider95,wxSlider)
    
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
    
    // TClassDef(wxObject)
    pub fn wxObject_GetClassInfo(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxObject_IsKindOf(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */;
    pub fn wxObject_IsScrolledWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    
    // TClassDef(wxClassInfo)
    pub fn wxClassInfo_FindClass(_txt: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetBaseClassName1(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetBaseClassName2(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetClassNameEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxClassInfo_GetSize(_obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxClassInfo_IsKindOfEx(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */;
    pub fn wxNotebook_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    
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
    
    // TClassDefExtend(wxInputSink,wxThread)
    pub fn wxInputSink_Create(input: *u8 /* void* */, evtHandler: *u8 /* void* */, bufferLen: c_int /* int */) -> *u8 /* void* */;
    pub fn wxInputSink_GetId(obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputSink_Start(obj: *u8 /* void* */);
    
    // TClassDefExtend(wxInputSinkEvent,wxEvent)
    pub fn wxInputSinkEvent_LastError(obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputSinkEvent_LastRead(obj: *u8 /* void* */) -> c_int /* int */;
    pub fn wxInputSinkEvent_LastInput(obj: *u8 /* void* */) -> *char /* char* */;
    
    // TClassDefExtend(wxcHtmlEvent,wxCommandEvent)
    pub fn wxcHtmlEvent_GetMouseEvent(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetHtmlCell(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetHtmlCellId(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetHref(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetTarget(self_: *u8 /* void* */) -> *u8 /* void* */;
    pub fn wxcHtmlEvent_GetLogicalPosition(self_: *u8 /* void* */) -> *u8 /* void* */;
    
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
}
