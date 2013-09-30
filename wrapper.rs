trait None {
    fn Null_AcceleratorTable() -> *u8 /* void* */;
    fn Null_Bitmap() -> *u8 /* void* */;
    fn Null_Brush() -> *u8 /* void* */;
    fn Null_Colour() -> *u8 /* void* */;
    fn Null_Cursor() -> *u8 /* void* */;
    fn Null_Font() -> *u8 /* void* */;
    fn Null_Icon() -> *u8 /* void* */;
    fn Null_Palette() -> *u8 /* void* */;
    fn Null_Pen() -> *u8 /* void* */;
    fn expEVT_COMMAND_AUITOOLBAR_TOOL_DROPDOWN() -> c_int /* int */;
    fn expEVT_COMMAND_AUITOOLBAR_OVERFLOW_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_AUITOOLBAR_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_AUITOOLBAR_MIDDLE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_AUITOOLBAR_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CLOSE() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_PAGE_CLOSED() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_BUTTON() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_END_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_DRAG_MOTION() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_ALLOW_DND() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_TAB_MIDDLE_DOWN() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_TAB_MIDDLE_UP() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_TAB_RIGHT_DOWN() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_TAB_RIGHT_UP() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_DRAG_DONE() -> c_int /* int */;
    fn expEVT_COMMAND_AUINOTEBOOK_BG_DCLICK() -> c_int /* int */;
    fn expEVT_AUI_PANE_BUTTON() -> c_int /* int */;
    fn expEVT_AUI_PANE_CLOSE() -> c_int /* int */;
    fn expEVT_AUI_PANE_MAXIMIZE() -> c_int /* int */;
    fn expEVT_AUI_PANE_RESTORE() -> c_int /* int */;
    fn expEVT_AUI_RENDER() -> c_int /* int */;
    fn expEVT_AUI_FIND_MANAGER() -> c_int /* int */;
    fn expEVT_CALENDAR_SEL_CHANGED() -> c_int /* int */;
    fn expEVT_CALENDAR_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_CALENDAR_DOUBLECLICKED() -> c_int /* int */;
    fn expEVT_CALENDAR_WEEKDAY_CLICKED() -> c_int /* int */;
    fn expEVT_CALENDAR_WEEK_CLICKED() -> c_int /* int */;
    fn expEVT_CALENDAR_DAY_CHANGED() -> c_int /* int */;
    fn expEVT_CALENDAR_MONTH_CHANGED() -> c_int /* int */;
    fn expEVT_CALENDAR_YEAR_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_CHOICEBOOK_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_CHOICEBOOK_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_CLIPBOARD_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_COLOURPICKER_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_COLLPANE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_SELECTION_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_ACTIVATED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_COLLAPSED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_EXPANDED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_COLLAPSING() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_EXPANDING() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_START_EDITING() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_EDITING_STARTED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_EDITING_DONE() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_VALUE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_CONTEXT_MENU() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_COLUMN_HEADER_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_COLUMN_HEADER_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_COLUMN_SORTED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_COLUMN_REORDERED() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_CACHE_HINT() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_DROP_POSSIBLE() -> c_int /* int */;
    fn expEVT_COMMAND_DATAVIEW_ITEM_DROP() -> c_int /* int */;
    fn expEVT_DATE_CHANGED() -> c_int /* int */;
    fn expEVT_WINDOW_MODAL_DIALOG_CLOSED() -> c_int /* int */;
    fn expEVT_COMMAND_BUTTON_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_CHECKBOX_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_CHOICE_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_LISTBOX_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_LISTBOX_DOUBLECLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_CHECKLISTBOX_TOGGLED() -> c_int /* int */;
    fn expEVT_COMMAND_MENU_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_SLIDER_UPDATED() -> c_int /* int */;
    fn expEVT_COMMAND_RADIOBOX_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_RADIOBUTTON_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_VLBOX_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_COMBOBOX_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_TOOL_RCLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_TOOL_DROPDOWN_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_TOOL_ENTER() -> c_int /* int */;
    fn expEVT_COMMAND_COMBOBOX_DROPDOWN() -> c_int /* int */;
    fn expEVT_COMMAND_COMBOBOX_CLOSEUP() -> c_int /* int */;
    fn expEVT_COMMAND_THREAD() -> c_int /* int */;
    fn expEVT_LEFT_DOWN() -> c_int /* int */;
    fn expEVT_LEFT_UP() -> c_int /* int */;
    fn expEVT_MIDDLE_DOWN() -> c_int /* int */;
    fn expEVT_MIDDLE_UP() -> c_int /* int */;
    fn expEVT_RIGHT_DOWN() -> c_int /* int */;
    fn expEVT_RIGHT_UP() -> c_int /* int */;
    fn expEVT_MOTION() -> c_int /* int */;
    fn expEVT_ENTER_WINDOW() -> c_int /* int */;
    fn expEVT_LEAVE_WINDOW() -> c_int /* int */;
    fn expEVT_LEFT_DCLICK() -> c_int /* int */;
    fn expEVT_MIDDLE_DCLICK() -> c_int /* int */;
    fn expEVT_RIGHT_DCLICK() -> c_int /* int */;
    fn expEVT_SET_FOCUS() -> c_int /* int */;
    fn expEVT_KILL_FOCUS() -> c_int /* int */;
    fn expEVT_CHILD_FOCUS() -> c_int /* int */;
    fn expEVT_MOUSEWHEEL() -> c_int /* int */;
    fn expEVT_AUX1_DOWN() -> c_int /* int */;
    fn expEVT_AUX1_UP() -> c_int /* int */;
    fn expEVT_AUX1_DCLICK() -> c_int /* int */;
    fn expEVT_AUX2_DOWN() -> c_int /* int */;
    fn expEVT_AUX2_UP() -> c_int /* int */;
    fn expEVT_AUX2_DCLICK() -> c_int /* int */;
    fn expEVT_CHAR() -> c_int /* int */;
    fn expEVT_CHAR_HOOK() -> c_int /* int */;
    fn expEVT_NAVIGATION_KEY() -> c_int /* int */;
    fn expEVT_KEY_DOWN() -> c_int /* int */;
    fn expEVT_KEY_UP() -> c_int /* int */;
    fn expEVT_HOTKEY() -> c_int /* int */;
    fn expEVT_SET_CURSOR() -> c_int /* int */;
    fn expEVT_SCROLL_TOP() -> c_int /* int */;
    fn expEVT_SCROLL_BOTTOM() -> c_int /* int */;
    fn expEVT_SCROLL_LINEUP() -> c_int /* int */;
    fn expEVT_SCROLL_LINEDOWN() -> c_int /* int */;
    fn expEVT_SCROLL_PAGEUP() -> c_int /* int */;
    fn expEVT_SCROLL_PAGEDOWN() -> c_int /* int */;
    fn expEVT_SCROLL_THUMBTRACK() -> c_int /* int */;
    fn expEVT_SCROLL_THUMBRELEASE() -> c_int /* int */;
    fn expEVT_SCROLL_CHANGED() -> c_int /* int */;
    fn expEVT_SPIN_UP() -> c_int /* int */;
    fn expEVT_SPIN_DOWN() -> c_int /* int */;
    fn expEVT_SPIN() -> c_int /* int */;
    fn expEVT_SCROLLWIN_TOP() -> c_int /* int */;
    fn expEVT_SCROLLWIN_BOTTOM() -> c_int /* int */;
    fn expEVT_SCROLLWIN_LINEUP() -> c_int /* int */;
    fn expEVT_SCROLLWIN_LINEDOWN() -> c_int /* int */;
    fn expEVT_SCROLLWIN_PAGEUP() -> c_int /* int */;
    fn expEVT_SCROLLWIN_PAGEDOWN() -> c_int /* int */;
    fn expEVT_SCROLLWIN_THUMBTRACK() -> c_int /* int */;
    fn expEVT_SCROLLWIN_THUMBRELEASE() -> c_int /* int */;
    fn expEVT_SIZE() -> c_int /* int */;
    fn expEVT_MOVE() -> c_int /* int */;
    fn expEVT_CLOSE_WINDOW() -> c_int /* int */;
    fn expEVT_END_SESSION() -> c_int /* int */;
    fn expEVT_QUERY_END_SESSION() -> c_int /* int */;
    fn expEVT_ACTIVATE_APP() -> c_int /* int */;
    fn expEVT_ACTIVATE() -> c_int /* int */;
    fn expEVT_CREATE() -> c_int /* int */;
    fn expEVT_DESTROY() -> c_int /* int */;
    fn expEVT_SHOW() -> c_int /* int */;
    fn expEVT_ICONIZE() -> c_int /* int */;
    fn expEVT_MAXIMIZE() -> c_int /* int */;
    fn expEVT_MOUSE_CAPTURE_CHANGED() -> c_int /* int */;
    fn expEVT_MOUSE_CAPTURE_LOST() -> c_int /* int */;
    fn expEVT_PAINT() -> c_int /* int */;
    fn expEVT_ERASE_BACKGROUND() -> c_int /* int */;
    fn expEVT_NC_PAINT() -> c_int /* int */;
    fn expEVT_MENU_OPEN() -> c_int /* int */;
    fn expEVT_MENU_CLOSE() -> c_int /* int */;
    fn expEVT_MENU_HIGHLIGHT() -> c_int /* int */;
    fn expEVT_CONTEXT_MENU() -> c_int /* int */;
    fn expEVT_SYS_COLOUR_CHANGED() -> c_int /* int */;
    fn expEVT_DISPLAY_CHANGED() -> c_int /* int */;
    fn expEVT_QUERY_NEW_PALETTE() -> c_int /* int */;
    fn expEVT_PALETTE_CHANGED() -> c_int /* int */;
    fn expEVT_JOY_BUTTON_DOWN() -> c_int /* int */;
    fn expEVT_JOY_BUTTON_UP() -> c_int /* int */;
    fn expEVT_JOY_MOVE() -> c_int /* int */;
    fn expEVT_JOY_ZMOVE() -> c_int /* int */;
    fn expEVT_DROP_FILES() -> c_int /* int */;
    fn expEVT_INIT_DIALOG() -> c_int /* int */;
    fn expEVT_IDLE() -> c_int /* int */;
    fn expEVT_UPDATE_UI() -> c_int /* int */;
    fn expEVT_SIZING() -> c_int /* int */;
    fn expEVT_MOVING() -> c_int /* int */;
    fn expEVT_MOVE_START() -> c_int /* int */;
    fn expEVT_MOVE_END() -> c_int /* int */;
    fn expEVT_HIBERNATE() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_COPY() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_CUT() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_PASTE() -> c_int /* int */;
    fn expEVT_COMMAND_LEFT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_LEFT_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RIGHT_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_SET_FOCUS() -> c_int /* int */;
    fn expEVT_COMMAND_KILL_FOCUS() -> c_int /* int */;
    fn expEVT_COMMAND_ENTER() -> c_int /* int */;
    fn expEVT_HELP() -> c_int /* int */;
    fn expEVT_DETAILED_HELP() -> c_int /* int */;
    fn expEVT_COMMAND_TOOL_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_FIND() -> c_int /* int */;
    fn expEVT_COMMAND_FIND_NEXT() -> c_int /* int */;
    fn expEVT_COMMAND_FIND_REPLACE() -> c_int /* int */;
    fn expEVT_COMMAND_FIND_REPLACE_ALL() -> c_int /* int */;
    fn expEVT_COMMAND_FIND_CLOSE() -> c_int /* int */;
    fn expEVT_FILECTRL_SELECTIONCHANGED() -> c_int /* int */;
    fn expEVT_FILECTRL_FILEACTIVATED() -> c_int /* int */;
    fn expEVT_FILECTRL_FOLDERCHANGED() -> c_int /* int */;
    fn expEVT_FILECTRL_FILTERCHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_FILEPICKER_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_DIRPICKER_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_FONTPICKER_CHANGED() -> c_int /* int */;
    fn expEVT_FSWATCHER() -> c_int /* int */;
    fn expEVT_GRID_CELL_LEFT_CLICK() -> c_int /* int */;
    fn expEVT_GRID_CELL_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_GRID_CELL_LEFT_DCLICK() -> c_int /* int */;
    fn expEVT_GRID_CELL_RIGHT_DCLICK() -> c_int /* int */;
    fn expEVT_GRID_LABEL_LEFT_CLICK() -> c_int /* int */;
    fn expEVT_GRID_LABEL_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_GRID_LABEL_LEFT_DCLICK() -> c_int /* int */;
    fn expEVT_GRID_LABEL_RIGHT_DCLICK() -> c_int /* int */;
    fn expEVT_GRID_ROW_SIZE() -> c_int /* int */;
    fn expEVT_GRID_COL_SIZE() -> c_int /* int */;
    fn expEVT_GRID_RANGE_SELECT() -> c_int /* int */;
    fn expEVT_GRID_CELL_CHANGING() -> c_int /* int */;
    fn expEVT_GRID_CELL_CHANGED() -> c_int /* int */;
    fn expEVT_GRID_SELECT_CELL() -> c_int /* int */;
    fn expEVT_GRID_EDITOR_SHOWN() -> c_int /* int */;
    fn expEVT_GRID_EDITOR_HIDDEN() -> c_int /* int */;
    fn expEVT_GRID_EDITOR_CREATED() -> c_int /* int */;
    fn expEVT_GRID_CELL_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_GRID_COL_MOVE() -> c_int /* int */;
    fn expEVT_GRID_COL_SORT() -> c_int /* int */;
    fn expEVT_QUERY_LAYOUT_INFO() -> c_int /* int */;
    fn expEVT_CALCULATE_LAYOUT() -> c_int /* int */;
    fn expEVT_SASH_DRAGGED() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_MIDDLE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_RIGHT_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_MIDDLE_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_SEPARATOR_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_BEGIN_RESIZE() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_RESIZING() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_END_RESIZE() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_BEGIN_REORDER() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_END_REORDER() -> c_int /* int */;
    fn expEVT_COMMAND_HEADER_DRAGGING_CANCELLED() -> c_int /* int */;
    fn expEVT_COMMAND_HTML_CELL_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_HTML_CELL_HOVER() -> c_int /* int */;
    fn expEVT_COMMAND_HTML_LINK_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_HYPERLINK() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_BEGIN_RDRAG() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_BEGIN_LABEL_EDIT() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_END_LABEL_EDIT() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_DELETE_ITEM() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_DELETE_ALL_ITEMS() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_ITEM_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_ITEM_DESELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_KEY_DOWN() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_INSERT_ITEM() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_COL_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_ITEM_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_ITEM_MIDDLE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_ITEM_ACTIVATED() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_CACHE_HINT() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_COL_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_COL_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_COL_DRAGGING() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_COL_END_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_LIST_ITEM_FOCUSED() -> c_int /* int */;
    fn expEVT_COMMAND_LISTBOOK_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_LISTBOOK_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_NOTEBOOK_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_NOTEBOOK_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_POWER_SUSPENDING() -> c_int /* int */;
    fn expEVT_POWER_SUSPENDED() -> c_int /* int */;
    fn expEVT_POWER_SUSPEND_CANCEL() -> c_int /* int */;
    fn expEVT_POWER_RESUME() -> c_int /* int */;
    fn expEVT_END_PROCESS() -> c_int /* int */;
    fn expEVT_PG_SELECTED() -> c_int /* int */;
    fn expEVT_PG_CHANGING() -> c_int /* int */;
    fn expEVT_PG_CHANGED() -> c_int /* int */;
    fn expEVT_PG_HIGHLIGHTED() -> c_int /* int */;
    fn expEVT_PG_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_PG_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_PG_ITEM_COLLAPSED() -> c_int /* int */;
    fn expEVT_PG_ITEM_EXPANDED() -> c_int /* int */;
    fn expEVT_PG_DOUBLE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBAR_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBAR_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBAR_TAB_MIDDLE_DOWN() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBAR_TAB_MIDDLE_UP() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBAR_TAB_RIGHT_DOWN() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBAR_TAB_RIGHT_UP() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBUTTON_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONBUTTON_DROPDOWN_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONGALLERY_HOVER_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONGALLERY_SELECTED() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONTOOL_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_RIBBONTOOL_DROPDOWN_CLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_LEFT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_MIDDLE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_LEFT_DCLICK() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_RETURN() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_CHARACTER() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_DELETE() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_STYLESHEET_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_STYLESHEET_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_STYLESHEET_REPLACING() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_STYLESHEET_REPLACED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_CONTENT_INSERTED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_CONTENT_DELETED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_STYLE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_SELECTION_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_RICHTEXT_BUFFER_RESET() -> c_int /* int */;
    fn expEVT_SOCKET() -> c_int /* int */;
    fn expEVT_COMMAND_SPINCTRL_UPDATED() -> c_int /* int */;
    fn expEVT_COMMAND_SPINCTRLDOUBLE_UPDATED() -> c_int /* int */;
    fn expEVT_COMMAND_SPLITTER_SASH_POS_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_SPLITTER_SASH_POS_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_SPLITTER_DOUBLECLICKED() -> c_int /* int */;
    fn expEVT_COMMAND_SPLITTER_UNSPLIT() -> c_int /* int */;
    fn expEVT_COMMAND_SEARCHCTRL_CANCEL_BTN() -> c_int /* int */;
    fn expEVT_COMMAND_SEARCHCTRL_SEARCH_BTN() -> c_int /* int */;
    fn expEVT_TASKBAR_MOVE() -> c_int /* int */;
    fn expEVT_TASKBAR_LEFT_DOWN() -> c_int /* int */;
    fn expEVT_TASKBAR_LEFT_UP() -> c_int /* int */;
    fn expEVT_TASKBAR_RIGHT_DOWN() -> c_int /* int */;
    fn expEVT_TASKBAR_RIGHT_UP() -> c_int /* int */;
    fn expEVT_TASKBAR_LEFT_DCLICK() -> c_int /* int */;
    fn expEVT_TASKBAR_RIGHT_DCLICK() -> c_int /* int */;
    fn expEVT_TASKBAR_BALLOON_TIMEOUT() -> c_int /* int */;
    fn expEVT_TASKBAR_BALLOON_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_UPDATED() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_ENTER() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_URL() -> c_int /* int */;
    fn expEVT_COMMAND_TEXT_MAXLEN() -> c_int /* int */;
    fn expEVT_COMMAND_TOGGLEBUTTON_CLICKED() -> c_int /* int */;
    fn expEVT_TIMER() -> c_int /* int */;
    fn expEVT_COMMAND_TOOLBOOK_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_TOOLBOOK_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_BEGIN_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_BEGIN_RDRAG() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_BEGIN_LABEL_EDIT() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_END_LABEL_EDIT() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_DELETE_ITEM() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_GET_INFO() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_SET_INFO() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_EXPANDED() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_EXPANDING() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_COLLAPSED() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_COLLAPSING() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_SEL_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_SEL_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_KEY_DOWN() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_ACTIVATED() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_RIGHT_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_MIDDLE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_END_DRAG() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_STATE_IMAGE_CLICK() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_GETTOOLTIP() -> c_int /* int */;
    fn expEVT_COMMAND_TREE_ITEM_MENU() -> c_int /* int */;
    fn expEVT_COMMAND_TREEBOOK_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_COMMAND_TREEBOOK_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_COMMAND_TREEBOOK_NODE_COLLAPSED() -> c_int /* int */;
    fn expEVT_COMMAND_TREEBOOK_NODE_EXPANDED() -> c_int /* int */;
    fn expEVT_WIZARD_PAGE_CHANGED() -> c_int /* int */;
    fn expEVT_WIZARD_PAGE_CHANGING() -> c_int /* int */;
    fn expEVT_WIZARD_CANCEL() -> c_int /* int */;
    fn expEVT_WIZARD_HELP() -> c_int /* int */;
    fn expEVT_WIZARD_FINISHED() -> c_int /* int */;
    fn expEVT_WIZARD_PAGE_SHOWN() -> c_int /* int */;
    fn expEVT_DELETE() -> c_int /* int */;
    fn expEVT_HTML_CELL_CLICKED() -> c_int /* int */;
    fn expEVT_HTML_CELL_MOUSE_HOVER() -> c_int /* int */;
    fn expEVT_HTML_LINK_CLICKED() -> c_int /* int */;
    fn expEVT_HTML_SET_TITLE() -> c_int /* int */;
    fn expEVT_INPUT_SINK() -> c_int /* int */;
    fn expEVT_SORT() -> c_int /* int */;
    fn expK_BACK() -> c_int /* int */;
    fn expK_TAB() -> c_int /* int */;
    fn expK_RETURN() -> c_int /* int */;
    fn expK_ESCAPE() -> c_int /* int */;
    fn expK_SPACE() -> c_int /* int */;
    fn expK_DELETE() -> c_int /* int */;
    fn expK_START() -> c_int /* int */;
    fn expK_LBUTTON() -> c_int /* int */;
    fn expK_RBUTTON() -> c_int /* int */;
    fn expK_CANCEL() -> c_int /* int */;
    fn expK_MBUTTON() -> c_int /* int */;
    fn expK_CLEAR() -> c_int /* int */;
    fn expK_SHIFT() -> c_int /* int */;
    fn expK_ALT() -> c_int /* int */;
    fn expK_CONTROL() -> c_int /* int */;
    fn expK_MENU() -> c_int /* int */;
    fn expK_PAUSE() -> c_int /* int */;
    fn expK_CAPITAL() -> c_int /* int */;
    fn expK_END() -> c_int /* int */;
    fn expK_HOME() -> c_int /* int */;
    fn expK_LEFT() -> c_int /* int */;
    fn expK_UP() -> c_int /* int */;
    fn expK_RIGHT() -> c_int /* int */;
    fn expK_DOWN() -> c_int /* int */;
    fn expK_SELECT() -> c_int /* int */;
    fn expK_PRINT() -> c_int /* int */;
    fn expK_EXECUTE() -> c_int /* int */;
    fn expK_SNAPSHOT() -> c_int /* int */;
    fn expK_INSERT() -> c_int /* int */;
    fn expK_HELP() -> c_int /* int */;
    fn expK_NUMPAD0() -> c_int /* int */;
    fn expK_NUMPAD1() -> c_int /* int */;
    fn expK_NUMPAD2() -> c_int /* int */;
    fn expK_NUMPAD3() -> c_int /* int */;
    fn expK_NUMPAD4() -> c_int /* int */;
    fn expK_NUMPAD5() -> c_int /* int */;
    fn expK_NUMPAD6() -> c_int /* int */;
    fn expK_NUMPAD7() -> c_int /* int */;
    fn expK_NUMPAD8() -> c_int /* int */;
    fn expK_NUMPAD9() -> c_int /* int */;
    fn expK_MULTIPLY() -> c_int /* int */;
    fn expK_ADD() -> c_int /* int */;
    fn expK_SEPARATOR() -> c_int /* int */;
    fn expK_SUBTRACT() -> c_int /* int */;
    fn expK_DECIMAL() -> c_int /* int */;
    fn expK_DIVIDE() -> c_int /* int */;
    fn expK_F1() -> c_int /* int */;
    fn expK_F2() -> c_int /* int */;
    fn expK_F3() -> c_int /* int */;
    fn expK_F4() -> c_int /* int */;
    fn expK_F5() -> c_int /* int */;
    fn expK_F6() -> c_int /* int */;
    fn expK_F7() -> c_int /* int */;
    fn expK_F8() -> c_int /* int */;
    fn expK_F9() -> c_int /* int */;
    fn expK_F10() -> c_int /* int */;
    fn expK_F11() -> c_int /* int */;
    fn expK_F12() -> c_int /* int */;
    fn expK_F13() -> c_int /* int */;
    fn expK_F14() -> c_int /* int */;
    fn expK_F15() -> c_int /* int */;
    fn expK_F16() -> c_int /* int */;
    fn expK_F17() -> c_int /* int */;
    fn expK_F18() -> c_int /* int */;
    fn expK_F19() -> c_int /* int */;
    fn expK_F20() -> c_int /* int */;
    fn expK_F21() -> c_int /* int */;
    fn expK_F22() -> c_int /* int */;
    fn expK_F23() -> c_int /* int */;
    fn expK_F24() -> c_int /* int */;
    fn expK_NUMLOCK() -> c_int /* int */;
    fn expK_SCROLL() -> c_int /* int */;
    fn expK_PAGEUP() -> c_int /* int */;
    fn expK_PAGEDOWN() -> c_int /* int */;
    fn expK_NUMPAD_SPACE() -> c_int /* int */;
    fn expK_NUMPAD_TAB() -> c_int /* int */;
    fn expK_NUMPAD_ENTER() -> c_int /* int */;
    fn expK_NUMPAD_F1() -> c_int /* int */;
    fn expK_NUMPAD_F2() -> c_int /* int */;
    fn expK_NUMPAD_F3() -> c_int /* int */;
    fn expK_NUMPAD_F4() -> c_int /* int */;
    fn expK_NUMPAD_HOME() -> c_int /* int */;
    fn expK_NUMPAD_LEFT() -> c_int /* int */;
    fn expK_NUMPAD_UP() -> c_int /* int */;
    fn expK_NUMPAD_RIGHT() -> c_int /* int */;
    fn expK_NUMPAD_DOWN() -> c_int /* int */;
    fn expK_NUMPAD_PAGEUP() -> c_int /* int */;
    fn expK_NUMPAD_PAGEDOWN() -> c_int /* int */;
    fn expK_NUMPAD_END() -> c_int /* int */;
    fn expK_NUMPAD_BEGIN() -> c_int /* int */;
    fn expK_NUMPAD_INSERT() -> c_int /* int */;
    fn expK_NUMPAD_DELETE() -> c_int /* int */;
    fn expK_NUMPAD_EQUAL() -> c_int /* int */;
    fn expK_NUMPAD_MULTIPLY() -> c_int /* int */;
    fn expK_NUMPAD_ADD() -> c_int /* int */;
    fn expK_NUMPAD_SEPARATOR() -> c_int /* int */;
    fn expK_NUMPAD_SUBTRACT() -> c_int /* int */;
    fn expK_NUMPAD_DECIMAL() -> c_int /* int */;
    fn expK_NUMPAD_DIVIDE() -> c_int /* int */;
    fn ELJSysErrorCode() -> c_int /* int */;
    fn ELJSysErrorMsg(nErrCode: c_int /* int */) -> *u8 /* void* */;
    fn LogErrorMsg(_msg: *u8 /* void* */);
    fn LogFatalErrorMsg(_msg: *u8 /* void* */);
    fn LogMessageMsg(_msg: *u8 /* void* */);
    fn LogWarningMsg(_msg: *u8 /* void* */);
    fn Quantize(src: *u8 /* void* */, dest: *u8 /* void* */, desiredNoColours: c_int /* int */, eightBitData: *u8 /* void* */, flags: c_int /* int */) -> bool /* bool */;
    fn QuantizePalette(src: *u8 /* void* */, dest: *u8 /* void* */, pPalette: *u8 /* void* */, desiredNoColours: c_int /* int */, eightBitData: *u8 /* void* */, flags: c_int /* int */) -> bool /* bool */;
    fn wxCFree(_ptr: *u8 /* void* */);
    fn wxGetELJLocale() -> *u8 /* void* */;
    fn wxGetELJTranslation(sz: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    // missing: wxMutexGui_Enter
    // missing: wxMutexGui_Leave
}
trait ELJApp {
    fn Bell();
    fn CreateLogTarget() -> *u8 /* void* */;
    fn Dispatch();
    fn DisplaySize() -> *u8 /* void* */;
    fn EnableTooltips(_enable: bool /* bool */);
    fn EnableTopLevelWindows(_enb: c_int /* int */);
    fn ExecuteProcess(_cmd: *u8 /* void* */, _snc: c_int /* int */, _prc: *u8 /* void* */) -> c_int /* int */;
    fn Exit();
    fn ExitMainLoop();
    fn FindWindowById(_id: c_int /* int */, _prt: *u8 /* void* */) -> *u8 /* void* */;
    fn FindWindowByLabel(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */;
    fn FindWindowByName(_lbl: *u8 /* void* */, _prt: *u8 /* void* */) -> *u8 /* void* */;
    fn GetApp() -> *u8 /* void* */;
    fn GetAppName() -> *u8 /* void* */;
    fn GetClassName() -> *u8 /* void* */;
    fn GetExitOnFrameDelete() -> c_int /* int */;
    fn GetOsDescription() -> *u8 /* void* */;
    fn GetOsVersion(_maj: *u8 /* void* */, _min: *u8 /* void* */) -> c_int /* int */;
    fn GetTopWindow() -> *u8 /* void* */;
    fn GetUseBestVisual() -> c_int /* int */;
    fn GetUserHome(_usr: *u8 /* void* */) -> *u8 /* void* */;
    fn GetUserId() -> *u8 /* void* */;
    fn GetUserName() -> *u8 /* void* */;
    fn GetVendorName() -> *u8 /* void* */;
    fn InitAllImageHandlers();
    fn Initialized() -> bool /* bool */;
    fn MainLoop() -> c_int /* int */;
    fn MousePosition() -> *u8 /* void* */;
    fn Pending() -> c_int /* int */;
    fn SafeYield(_win: *u8 /* void* */) -> c_int /* int */;
    fn SetAppName(name: *u8 /* void* */);
    fn SetClassName(name: *u8 /* void* */);
    fn SetExitOnFrameDelete(flag: c_int /* int */);
    fn SetPrintMode(mode: c_int /* int */);
    fn SetTooltipDelay(_ms: c_int /* int */);
    fn SetTopWindow(_wnd: *u8 /* void* */);
    fn SetUseBestVisual(flag: c_int /* int */);
    fn SetVendorName(name: *u8 /* void* */);
    fn Sleep(_scs: c_int /* int */);
    fn MilliSleep(_mscs: c_int /* int */);
    fn Yield() -> c_int /* int */;
    fn IsTerminating() -> c_int /* int */;
}
trait ELJArtProv {
    fn Create(_obj: *u8 /* void* */, _clb: *u8 /* void* */) -> *u8 /* void* */;
    fn Release(_obj: *u8 /* void* */);
}
trait ELJClient {
    // missing: ELJClient_Create
    // missing: ELJClient_Delete
    // missing: ELJClient_MakeConnection
}
trait ELJCommand {
    // missing: ELJCommand_CanUndo
    // missing: ELJCommand_Create
    // missing: ELJCommand_Delete
    // missing: ELJCommand_GetName
}
trait ELJConnection {
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
}
trait ELJDragDataObject {
    fn Create(_obj: *u8 /* void* */, _fmt: *u8 /* void* */, _func1: *u8 /* void* */, _func2: *u8 /* void* */, _func3: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait ELJDropTarget {
    fn Create(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
}
trait ELJFileDropTarget {
    fn Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
}
trait ELJGridTable {
    fn Create(_obj: *u8 /* void* */, _EifGetNumberRows: *u8 /* void* */, _EifGetNumberCols: *u8 /* void* */, _EifGetValue: *u8 /* void* */, _EifSetValue: *u8 /* void* */, _EifIsEmptyCell: *u8 /* void* */, _EifClear: *u8 /* void* */, _EifInsertRows: *u8 /* void* */, _EifAppendRows: *u8 /* void* */, _EifDeleteRows: *u8 /* void* */, _EifInsertCols: *u8 /* void* */, _EifAppendCols: *u8 /* void* */, _EifDeleteCols: *u8 /* void* */, _EifSetRowLabelValue: *u8 /* void* */, _EifSetColLabelValue: *u8 /* void* */, _EifGetRowLabelValue: *u8 /* void* */, _EifGetColLabelValue: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetView(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SendTableMessage(_obj: *u8 /* void* */, id: c_int /* int */, val1: c_int /* int */, val2: c_int /* int */) -> *u8 /* void* */;
}
trait ELJLocale {
}
trait ELJLog {
    fn AddTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */);
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn DontCreateOnDemand(_obj: *u8 /* void* */);
    fn EnableLogging(_obj: *u8 /* void* */, doIt: bool /* bool */) -> c_int /* int */;
    fn Flush(_obj: *u8 /* void* */);
    fn FlushActive(_obj: *u8 /* void* */);
    fn GetActiveTarget() -> *u8 /* void* */;
    fn GetTimestamp(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */;
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *u8 /* void* */, t: c_int /* int */);
    fn RemoveTraceMask(_obj: *u8 /* void* */, str: *wchar_t /* wchar_t* */);
    fn Resume(_obj: *u8 /* void* */);
    fn SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */;
    fn SetTimestamp(_obj: *u8 /* void* */, ts: *u8 /* void* */);
    fn SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */);
    fn SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */);
    fn Suspend(_obj: *u8 /* void* */);
}
trait ELJMessageParameters {
    // missing: wxMessageParameters_Create
    // missing: wxMessageParameters_Delete
}
trait ELJPlotCurve {
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
}
trait ELJPreviewControlBar {
    fn Create(preview: *u8 /* void* */, buttons: c_int /* int */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
}
trait ELJPreviewFrame {
    fn Create(_obj: *u8 /* void* */, _init: *u8 /* void* */, _create_canvas: *u8 /* void* */, _create_toolbar: *u8 /* void* */, preview: *u8 /* void* */, parent: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    fn GetControlBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPreviewCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrintPreview(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Initialize(_obj: *u8 /* void* */);
    fn SetControlBar(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn SetPreviewCanvas(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn SetPrintPreview(_obj: *u8 /* void* */, obj: *u8 /* void* */);
}
trait ELJServer {
    // missing: ELJServer_Create
    // missing: ELJServer_Delete
    // missing: ELJServer_Initialize
}
trait ELJTextDropTarget {
    fn Create(_obj: *u8 /* void* */, _func: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn SetOnData(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnDragOver(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnDrop(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnEnter(_obj: *u8 /* void* */, _func: *u8 /* void* */);
    fn SetOnLeave(_obj: *u8 /* void* */, _func: *u8 /* void* */);
}
trait ELJTextValidator {
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */, _txt: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait cbAntiflickerPlugin {
    // missing: cbAntiflickerPlugin_Create
    // missing: cbAntiflickerPlugin_CreateDefault
    // missing: cbAntiflickerPlugin_Delete
}
trait cbBarDragPlugin {
    // missing: cbBarDragPlugin_Create
    // missing: cbBarDragPlugin_CreateDefault
    // missing: cbBarDragPlugin_Delete
}
trait cbBarHintsPlugin {
    // missing: cbBarHintsPlugin_Create
    // missing: cbBarHintsPlugin_CreateDefault
    // missing: cbBarHintsPlugin_Delete
    // missing: cbBarHintsPlugin_SetGrooveCount
}
trait cbBarInfo {
    // missing: cbBarInfo_Create
    // missing: cbBarInfo_Delete
    // missing: cbBarInfo_IsExpanded
    // missing: cbBarInfo_IsFixed
}
trait cbBarSpy {
    // missing: cbBarSpy_Create
    // missing: cbBarSpy_CreateDefault
    // missing: cbBarSpy_Delete
    // missing: cbBarSpy_ProcessEvent
    // missing: cbBarSpy_SetBarWindow
}
trait cbCloseBox {
    // missing: cbCloseBox_Create
}
trait cbCollapseBox {
    // missing: cbCollapseBox_Create
}
trait cbCommonPaneProperties {
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
}
trait cbCustomizeBarEvent {
    // missing: cbCustomizeBarEvent_Bar
    // missing: cbCustomizeBarEvent_ClickPos
}
trait cbCustomizeLayoutEvent {
    // missing: cbCustomizeLayoutEvent_ClickPos
}
trait cbDimHandlerBase {
}
trait cbDimInfo {
    // missing: cbDimInfo_Assign
    // missing: cbDimInfo_Create
    // missing: cbDimInfo_CreateDefault
    // missing: cbDimInfo_CreateWithHandler
    // missing: cbDimInfo_CreateWithInfo
    // missing: cbDimInfo_Delete
    // missing: cbDimInfo_GetDimHandler
}
trait cbDockBox {
    // missing: cbDockBox_Create
}
trait cbDockPane {
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
}
trait cbDrawBarDecorEvent {
    // missing: cbDrawBarDecorEvent_Bar
    // missing: cbDrawBarDecorEvent_BoundsInParent
    // missing: cbDrawBarDecorEvent_Dc
}
trait cbDrawBarHandlesEvent {
    // missing: cbDrawBarHandlesEvent_Bar
    // missing: cbDrawBarHandlesEvent_Dc
}
trait cbDrawHintRectEvent {
    // missing: cbDrawHintRectEvent_EraseRect
    // missing: cbDrawHintRectEvent_IsInClient
    // missing: cbDrawHintRectEvent_LastTime
    // missing: cbDrawHintRectEvent_Rect
}
trait cbDrawPaneBkGroundEvent {
    // missing: cbDrawPaneBkGroundEvent_Dc
}
trait cbDrawPaneDecorEvent {
    // missing: cbDrawPaneDecorEvent_Dc
}
trait cbDrawRowBkGroundEvent {
    // missing: cbDrawRowBkGroundEvent_Dc
    // missing: cbDrawRowBkGroundEvent_Row
}
trait cbDrawRowDecorEvent {
    // missing: cbDrawRowDecorEvent_Dc
    // missing: cbDrawRowDecorEvent_Row
}
trait cbDrawRowHandlesEvent {
    // missing: cbDrawRowHandlesEvent_Dc
    // missing: cbDrawRowHandlesEvent_Row
}
trait cbDynToolBarDimHandler {
    // missing: cbDynToolBarDimHandler_Create
    // missing: cbDynToolBarDimHandler_Delete
}
trait cbFinishDrawInAreaEvent {
    // missing: cbFinishDrawInAreaEvent_Area
}
trait cbFloatedBarWindow {
    // missing: cbFloatedBarWindow_Create
    // missing: cbFloatedBarWindow_GetBar
    // missing: cbFloatedBarWindow_PositionFloatedWnd
    // missing: cbFloatedBarWindow_SetBar
    // missing: cbFloatedBarWindow_SetLayout
}
trait cbGCUpdatesMgr {
    // missing: cbGCUpdatesMgr_Create
    // missing: cbGCUpdatesMgr_CreateDefault
    // missing: cbGCUpdatesMgr_Delete
    // missing: cbGCUpdatesMgr_UpdateNow
}
trait cbHintAnimationPlugin {
    // missing: cbHintAnimationPlugin_Create
    // missing: cbHintAnimationPlugin_CreateDefault
    // missing: cbHintAnimationPlugin_Delete
}
trait cbInsertBarEvent {
    // missing: cbInsertBarEvent_Bar
    // missing: cbInsertBarEvent_Row
}
trait cbLayoutRowEvent {
    // missing: cbLayoutRowEvent_Row
}
trait cbLeftDClickEvent {
    // missing: cbLeftDClickEvent_Pos
}
trait cbLeftDownEvent {
    // missing: cbLeftDownEvent_Pos
}
trait cbLeftUpEvent {
    // missing: cbLeftUpEvent_Pos
}
trait cbMiniButton {
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
}
trait cbMotionEvent {
    // missing: cbMotionEvent_Pos
}
trait cbPaneDrawPlugin {
    // missing: cbPaneDrawPlugin_Create
    // missing: cbPaneDrawPlugin_CreateDefault
    // missing: cbPaneDrawPlugin_Delete
}
trait cbPluginBase {
    // missing: cbPluginBase_Delete
    // missing: cbPluginBase_GetPaneMask
    // missing: cbPluginBase_IsReady
    // missing: cbPluginBase_Plugin
    // missing: cbPluginBase_ProcessEvent
}
trait cbPluginEvent {
    // missing: cbPluginEvent_Pane
}
trait cbRemoveBarEvent {
    // missing: cbRemoveBarEvent_Bar
}
trait cbResizeBarEvent {
    // missing: cbResizeBarEvent_Bar
    // missing: cbResizeBarEvent_Row
}
trait cbResizeRowEvent {
    // missing: cbResizeRowEvent_ForUpperHandle
    // missing: cbResizeRowEvent_HandleOfs
    // missing: cbResizeRowEvent_Row
}
trait cbRightDownEvent {
    // missing: cbRightDownEvent_Pos
}
trait cbRightUpEvent {
    // missing: cbRightUpEvent_Pos
}
trait cbRowDragPlugin {
    // missing: cbRowDragPlugin_Create
    // missing: cbRowDragPlugin_CreateDefault
    // missing: cbRowDragPlugin_Delete
}
trait cbRowInfo {
    // missing: cbRowInfo_Create
    // missing: cbRowInfo_Delete
    // missing: cbRowInfo_GetFirstBar
}
trait cbRowLayoutPlugin {
    // missing: cbRowLayoutPlugin_Create
    // missing: cbRowLayoutPlugin_CreateDefault
    // missing: cbRowLayoutPlugin_Delete
}
trait cbSimpleCustomizationPlugin {
    // missing: cbSimpleCustomizationPlugin_Create
    // missing: cbSimpleCustomizationPlugin_CreateDefault
    // missing: cbSimpleCustomizationPlugin_Delete
}
trait cbSimpleUpdatesMgr {
}
trait cbSizeBarWndEvent {
    // missing: cbSizeBarWndEvent_Bar
    // missing: cbSizeBarWndEvent_BoundsInParent
}
trait cbStartBarDraggingEvent {
    // missing: cbStartBarDraggingEvent_Bar
    // missing: cbStartBarDraggingEvent_Pos
}
trait cbStartDrawInAreaEvent {
    // missing: cbStartDrawInAreaEvent_Area
}
trait cbUpdatesManagerBase {
}
trait wxAcceleratorEntry {
    fn Create(flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetCommand(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn Set(_obj: *u8 /* void* */, flags: c_int /* int */, keyCode: c_int /* int */, cmd: c_int /* int */);
}
trait wxAcceleratorTable {
    fn Create(n: c_int /* int */, entries: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxActivateEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetActive(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxApp {
}
trait wxArray {
}
trait wxArrayString {
}
trait wxArtProvider {
    fn PopProvider() -> bool /* bool */;
    fn PushProvider(provider: *u8 /* void* */);
    fn RemoveProvider(provider: *u8 /* void* */) -> bool /* bool */;
}
trait wxAutoBufferedPaintDC {
    fn Create(window: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(self_: *u8 /* void* */);
}
trait wxAutomationObject {
}
trait wxBitmap {
    fn AddHandler(handler: *u8 /* void* */);
    fn CleanUpHandlers();
    fn Create(_data: *u8 /* void* */, _type: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */;
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateEmpty(arg0: c_int /* int */, arg1: c_int /* int */, _depth: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromXPM(data: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateLoad(name: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn FindHandlerByExtension(extension: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    fn FindHandlerByName(name: *u8 /* void* */) -> *u8 /* void* */;
    fn FindHandlerByType(type_: c_int /* int */) -> *u8 /* void* */;
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMask(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSubBitmap(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _ref: *u8 /* void* */);
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn InitStandardHandlers();
    fn InsertHandler(handler: *u8 /* void* */);
    fn LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> c_int /* int */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RemoveHandler(name: *u8 /* void* */) -> bool /* bool */;
    fn SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */, cmap: *u8 /* void* */) -> c_int /* int */;
    fn SetDepth(_obj: *u8 /* void* */, d: c_int /* int */);
    fn SetHeight(_obj: *u8 /* void* */, h: c_int /* int */);
    fn SetMask(_obj: *u8 /* void* */, mask: *u8 /* void* */);
    fn SetWidth(_obj: *u8 /* void* */, w: c_int /* int */);
}
trait wxBitmapButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetBitmapDisabled(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetBitmapFocus(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetBitmapLabel(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetBitmapSelected(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetMarginX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMarginY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetBitmapDisabled(_obj: *u8 /* void* */, disabled: *u8 /* void* */);
    fn SetBitmapFocus(_obj: *u8 /* void* */, focus: *u8 /* void* */);
    fn SetBitmapLabel(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    fn SetBitmapSelected(_obj: *u8 /* void* */, sel: *u8 /* void* */);
    fn SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
}
trait wxBitmapToggleButton {
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetValue(_obj: *u8 /* void* */, state: bool /* bool */);
    fn SetBitmapLabel(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
}
trait wxBitmapDataObject {
    fn BitmapDataObject_Create(_bmp: *u8 /* void* */) -> *u8 /* void* */;
    fn BitmapDataObject_CreateEmpty() -> *u8 /* void* */;
    fn BitmapDataObject_Delete(_obj: *u8 /* void* */);
    fn BitmapDataObject_GetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
    fn BitmapDataObject_SetBitmap(_obj: *u8 /* void* */, _bmp: *u8 /* void* */);
}
trait wxBitmapHandler {
}
trait wxBoxSizer {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(orient: c_int /* int */) -> *u8 /* void* */;
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn RecalcSizes(_obj: *u8 /* void* */);
}
trait wxBrush {
    fn Assign(_obj: *u8 /* void* */, brush: *u8 /* void* */);
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateFromColour(col: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsEqual(_obj: *u8 /* void* */, brush: *u8 /* void* */) -> bool /* bool */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */);
    fn SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
}
trait wxBrushList {
}
trait wxBufferedDC {
    fn CreateByDCAndSize(dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    fn CreateByDCAndBitmap(dc: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    fn Delete(self_: *u8 /* void* */);
}
trait wxBufferedPaintDC {
    fn Create(window: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    fn CreateWithBitmap(window: *u8 /* void* */, bitmap: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    fn Delete(self_: *u8 /* void* */);
}
trait wxBufferedInputStream {
}
trait wxBufferedOutputStream {
}
trait wxBusyCursor {
    fn Create() -> *u8 /* void* */;
    fn CreateWithCursor(_cur: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxBusyInfo {
    fn Create(_txt: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    fn SetDefault(_obj: *u8 /* void* */);
}
trait wxCSConv {
}
trait wxCalculateLayoutEvent {
    fn Create(id: c_int /* int */) -> *u8 /* void* */;
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    fn SetRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
}
trait wxCalendarCtrl {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _dat: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn EnableHolidayDisplay(_obj: *u8 /* void* */, display: c_int /* int */);
    fn EnableMonthChange(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn GetAttr(_obj: *u8 /* void* */, day: c_int /* int */) -> *u8 /* void* */;
    fn GetDate(_obj: *u8 /* void* */, date: *u8 /* void* */);
    fn GetHeaderColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetHeaderColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetHighlightColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetHighlightColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetHolidayColourBg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetHolidayColourFg(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, date: *u8 /* void* */, wd: *u8 /* void* */) -> c_int /* int */;
    fn ResetAttr(_obj: *u8 /* void* */, day: c_int /* int */);
    fn SetAttr(_obj: *u8 /* void* */, day: c_int /* int */, attr: *u8 /* void* */);
    fn SetDate(_obj: *u8 /* void* */, date: *u8 /* void* */);
    fn SetHeaderColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */);
    fn SetHighlightColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */);
    fn SetHoliday(_obj: *u8 /* void* */, day: c_int /* int */);
    fn SetHolidayColours(_obj: *u8 /* void* */, colFg: *u8 /* void* */, colBg: *u8 /* void* */);
}
trait wxCalendarDateAttr {
    fn Create(_ctxt: *u8 /* void* */, _cbck: *u8 /* void* */, _cbrd: *u8 /* void* */, _fnt: *u8 /* void* */, _brd: c_int /* int */) -> *u8 /* void* */;
    fn CreateDefault() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetBorder(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetBorderColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasBorder(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasBorderColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasFont(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsHoliday(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetBorder(_obj: *u8 /* void* */, border: c_int /* int */);
    fn SetBorderColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetHoliday(_obj: *u8 /* void* */, holiday: c_int /* int */);
    fn SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
}
trait wxCalendarEvent {
    fn GetDate(_obj: *u8 /* void* */, _dte: *u8 /* void* */);
    fn GetWeekDay(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxCaret {
    fn Create(_wnd: *u8 /* void* */, _wth: c_int /* int */, _hgt: c_int /* int */) -> *u8 /* void* */;
    fn GetBlinkTime() -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Hide(_obj: *u8 /* void* */);
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsVisible(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetBlinkTime(milliseconds: c_int /* int */);
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn Show(_obj: *u8 /* void* */);
}
trait wxCheckBox {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetValue(_obj: *u8 /* void* */, value: c_int /* int */);
}
trait wxCheckListBox {
    fn Check(_obj: *u8 /* void* */, item: c_int /* int */, check: bool /* bool */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn IsChecked(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
}
trait wxChoice {
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn Clear(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */);
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */);
    fn SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */);
}
trait wxClassInfo {
    fn CreateClassByName(_inf: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClassName(_inf: *u8 /* void* */) -> *u8 /* void* */;
    fn IsKindOf(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> bool /* bool */;
}
trait wxClient {
}
trait wxClientBase {
}
trait wxClientDC {
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxClientData {
}
trait wxClientDataContainer {
}
trait wxClipboard {
    fn AddData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */;
    fn Clear(_obj: *u8 /* void* */);
    fn Close(_obj: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn Flush(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */;
    fn IsOpened(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsSupported(_obj: *u8 /* void* */, format: *u8 /* void* */) -> bool /* bool */;
    fn Open(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */) -> bool /* bool */;
    fn UsePrimarySelection(_obj: *u8 /* void* */, primary: bool /* bool */);
}
trait wxCloseEvent {
    fn CanVeto(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetLoggingOff(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetVeto(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetCanVeto(_obj: *u8 /* void* */, canVeto: bool /* bool */);
    fn SetLoggingOff(_obj: *u8 /* void* */, logOff: bool /* bool */);
    fn Veto(_obj: *u8 /* void* */, veto: bool /* bool */);
}
trait wxClosure {
}
trait wxColour {
    fn Alpha(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */);
    fn Blue(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    fn Copy(_obj: *u8 /* void* */, _other: *u8 /* void* */);
    fn CreateByName(_name: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateEmpty() -> *u8 /* void* */;
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    fn CreateRGB(_red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Green(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Red(_obj: *u8 /* void* */) -> uint8_t /* uint8_t */;
    fn Set(_obj: *u8 /* void* */, _red: uint8_t /* uint8_t */, _green: uint8_t /* uint8_t */, _blue: uint8_t /* uint8_t */, _alpha: uint8_t /* uint8_t */);
    fn SetByName(_obj: *u8 /* void* */, _name: *u8 /* void* */);
    fn ValidName(_name: *wchar_t /* wchar_t* */) -> bool /* bool */;
}
trait wxColourData {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetChooseFull(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, _ref: *u8 /* void* */);
    fn SetChooseFull(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetCustomColour(_obj: *u8 /* void* */, i: c_int /* int */, colour: *u8 /* void* */);
}
trait wxColourDatabase {
}
trait wxColourDialog {
    fn Create(_prt: *u8 /* void* */, col: *u8 /* void* */) -> *u8 /* void* */;
    fn GetColourData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
}
trait wxComboBox {
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, d: *u8 /* void* */);
    fn Clear(_obj: *u8 /* void* */);
    fn Copy(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Cut(_obj: *u8 /* void* */);
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */);
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    fn GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetInsertionPoint(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetLastPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    fn GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Paste(_obj: *u8 /* void* */);
    fn Remove(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */);
    fn Replace(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */, value: *u8 /* void* */);
    fn SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */);
    fn SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */);
    fn SetInsertionPoint(_obj: *u8 /* void* */, pos: c_int /* int */);
    fn SetInsertionPointEnd(_obj: *u8 /* void* */);
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */);
    fn SetTextSelection(_obj: *u8 /* void* */, from: c_int /* int */, to: c_int /* int */);
}
trait wxCommand {
}
trait wxCommandEvent {
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    fn Create(_typ: c_int /* int */, _id: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClientObject(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetExtraLong(_obj: *u8 /* void* */) -> c_long /* long */;
    fn GetInt(_obj: *u8 /* void* */) -> c_long /* long */;
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */);
    fn SetClientObject(_obj: *u8 /* void* */, clientObject: *u8 /* void* */);
    fn SetExtraLong(_obj: *u8 /* void* */, extraLong: c_long /* long */);
    fn SetInt(_obj: *u8 /* void* */, i: c_int /* int */);
    fn SetString(_obj: *u8 /* void* */, s: *u8 /* void* */);
}
trait wxCommandLineParser {
}
trait wxCommandProcessor {
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
}
trait wxCondition {
    // missing: wxCondition_Broadcast
    // missing: wxCondition_Create
    // missing: wxCondition_Delete
    // missing: wxCondition_Signal
    // missing: wxCondition_Wait
    // missing: wxCondition_WaitFor
}
trait wxConfigBase {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn DeleteAll(_obj: *u8 /* void* */) -> bool /* bool */;
    fn DeleteEntry(_obj: *u8 /* void* */, key: *u8 /* void* */, bDeleteGroupIfEmpty: bool /* bool */) -> bool /* bool */;
    fn DeleteGroup(_obj: *u8 /* void* */, key: *u8 /* void* */) -> bool /* bool */;
    fn Exists(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */;
    fn ExpandEnvVars(_obj: *u8 /* void* */, str: *u8 /* void* */) -> *u8 /* void* */;
    fn Flush(_obj: *u8 /* void* */, bCurrentOnly: bool /* bool */) -> bool /* bool */;
    fn GetAppName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetEntryType(_obj: *u8 /* void* */, name: *u8 /* void* */) -> c_int /* int */;
    fn GetFirstEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFirstGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    fn GetNextEntry(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    fn GetNextGroup(_obj: *u8 /* void* */, lIndex: *u8 /* void* */) -> *u8 /* void* */;
    fn GetNumberOfEntries(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */;
    fn GetNumberOfGroups(_obj: *u8 /* void* */, bRecursive: bool /* bool */) -> c_int /* int */;
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetVendorName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn HasEntry(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */;
    fn HasGroup(_obj: *u8 /* void* */, strName: *u8 /* void* */) -> bool /* bool */;
    fn IsExpandingEnvVars(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsRecordingDefaults(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ReadBool(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: bool /* bool */) -> bool /* bool */;
    fn ReadDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_double /* double */) -> c_double /* double */;
    fn ReadInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: c_int /* int */) -> c_int /* int */;
    fn ReadString(_obj: *u8 /* void* */, key: *u8 /* void* */, defVal: *u8 /* void* */) -> *u8 /* void* */;
    fn RenameEntry(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */;
    fn RenameGroup(_obj: *u8 /* void* */, oldName: *u8 /* void* */, newName: *u8 /* void* */) -> bool /* bool */;
    fn SetAppName(_obj: *u8 /* void* */, appName: *u8 /* void* */);
    fn SetExpandEnvVars(_obj: *u8 /* void* */, bDoIt: bool /* bool */);
    fn SetPath(_obj: *u8 /* void* */, strPath: *u8 /* void* */);
    fn SetRecordDefaults(_obj: *u8 /* void* */, bDoIt: bool /* bool */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    fn SetVendorName(_obj: *u8 /* void* */, vendorName: *u8 /* void* */);
    fn WriteBool(_obj: *u8 /* void* */, key: *u8 /* void* */, value: bool /* bool */) -> bool /* bool */;
    fn WriteDouble(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_double /* double */) -> bool /* bool */;
    fn WriteInteger(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */;
    fn WriteLong(_obj: *u8 /* void* */, key: *u8 /* void* */, value: c_long /* long */) -> bool /* bool */;
    fn WriteString(_obj: *u8 /* void* */, key: *u8 /* void* */, value: *u8 /* void* */) -> bool /* bool */;
}
trait wxConnection {
}
trait wxConnectionBase {
}
trait wxContextHelp {
    fn BeginContextHelp(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    fn Create(win: *u8 /* void* */, beginHelp: bool /* bool */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn EndContextHelp(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxContextHelpButton {
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_long /* long */) -> *u8 /* void* */;
}
trait wxControl {
    fn Command(_obj: *u8 /* void* */, event: *u8 /* void* */);
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetLabel(_obj: *u8 /* void* */, text: *u8 /* void* */);
}
trait wxCountingOutputStream {
}
trait wxCriticalSection {
    // missing: wxCriticalSection_Create
    // missing: wxCriticalSection_Delete
    // missing: wxCriticalSection_Enter
    // missing: wxCriticalSection_Leave
}
trait wxCriticalSectionLocker {
}
trait wxCursor {
    fn Cursor_CreateFromStock(_id: c_int /* int */) -> *u8 /* void* */;
    fn Cursor_CreateFromImage(image: *u8 /* void* */) -> *u8 /* void* */;
    fn Cursor_CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
}
trait wxCustomDataObject {
}
trait wxDC {
    fn Blit(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, source: *u8 /* void* */, arg4: c_int /* int */, arg5: c_int /* int */, rop: c_int /* int */, useMask: bool /* bool */) -> bool /* bool */;
    fn CalcBoundingBox(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn CanDrawBitmap(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanGetTextExtent(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Clear(_obj: *u8 /* void* */);
    fn ComputeScaleAndOrigin(obj: *u8 /* void* */);
    fn CrossHair(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn Delete(_obj: *u8 /* void* */);
    fn DestroyClippingRegion(_obj: *u8 /* void* */);
    fn DeviceToLogicalX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    fn DeviceToLogicalXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    fn DeviceToLogicalY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    fn DeviceToLogicalYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    fn DrawArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */);
    fn DrawBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, useMask: bool /* bool */);
    fn DrawCheckMark(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn DrawCircle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, radius: c_int /* int */);
    fn DrawEllipse(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn DrawEllipticArc(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sa: c_double /* double */, ea: c_double /* double */);
    fn DrawIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn DrawLabel(_obj: *u8 /* void* */, str: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */);
    fn DrawLabelBitmap(_obj: *u8 /* void* */, str: *u8 /* void* */, bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, align: c_int /* int */, indexAccel: c_int /* int */) -> *u8 /* void* */;
    fn DrawLine(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn DrawLines(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn DrawPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn DrawPolygon(_obj: *u8 /* void* */, n: c_int /* int */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */);
    fn DrawPolyPolygon(_obj: *u8 /* void* */, n: c_int /* int */, count: *u8 /* void* */, x: *u8 /* void* */, y: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, fillStyle: c_int /* int */);
    fn DrawRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn DrawRotatedText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, angle: c_double /* double */);
    fn DrawRoundedRectangle(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, radius: c_double /* double */);
    fn DrawText(_obj: *u8 /* void* */, text: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn EndDoc(_obj: *u8 /* void* */);
    fn EndPage(_obj: *u8 /* void* */);
    fn FloodFill(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */, style: c_int /* int */);
    fn GetBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetBackgroundMode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetBrush(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetClippingBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetDeviceOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetLogicalFunction(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetLogicalOrigin(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetLogicalScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */);
    fn GetMapMode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPPI(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPen(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetPixel(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */) -> bool /* bool */;
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSizeMM(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTextBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, descent: *u8 /* void* */, externalLeading: *u8 /* void* */, theFont: *u8 /* void* */);
    fn GetMultiLineTextExtent(self_: *u8 /* void* */, string: *u8 /* void* */, w: *u8 /* void* */, h: *u8 /* void* */, heightLine: *u8 /* void* */, theFont: *u8 /* void* */);
    fn GetTextForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetUserScale(_obj: *u8 /* void* */, arg0: *c_double /* double* */, arg1: *c_double /* double* */);
    fn LogicalToDeviceX(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    fn LogicalToDeviceXRel(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    fn LogicalToDeviceY(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    fn LogicalToDeviceYRel(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    fn MaxX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn MaxY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn MinX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn MinY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ResetBoundingBox(_obj: *u8 /* void* */);
    fn SetAxisOrientation(_obj: *u8 /* void* */, xLeftRight: bool /* bool */, yBottomUp: bool /* bool */);
    fn SetBackground(_obj: *u8 /* void* */, brush: *u8 /* void* */);
    fn SetBackgroundMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    fn SetBrush(_obj: *u8 /* void* */, brush: *u8 /* void* */);
    fn SetClippingRegion(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn SetClippingRegionFromRegion(_obj: *u8 /* void* */, region: *u8 /* void* */);
    fn SetDeviceOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetLogicalFunction(_obj: *u8 /* void* */, function: c_int /* int */);
    fn SetLogicalOrigin(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetLogicalScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */);
    fn SetMapMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    fn SetPalette(_obj: *u8 /* void* */, palette: *u8 /* void* */);
    fn SetPen(_obj: *u8 /* void* */, pen: *u8 /* void* */);
    fn SetTextBackground(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetTextForeground(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetUserScale(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */);
    fn StartDoc(_obj: *u8 /* void* */, msg: *u8 /* void* */) -> bool /* bool */;
    fn StartPage(_obj: *u8 /* void* */);
}
trait wxDCClipper {
}
trait wxDDEClient {
}
trait wxDDEConnection {
}
trait wxDDEServer {
}
trait wxDataFormat {
    fn CreateFromId(name: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateFromType(typ: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetId(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetType(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */;
    fn SetId(_obj: *u8 /* void* */, id: *u8 /* void* */);
    fn SetType(_obj: *u8 /* void* */, typ: c_int /* int */);
}
trait wxDataInputStream {
}
trait wxDataObject {
}
trait wxDataObjectComposite {
    fn Add(_obj: *u8 /* void* */, _dat: *u8 /* void* */, _preferred: c_int /* int */);
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxDataObjectSimple {
}
trait wxDataOutputStream {
}
trait wxDatabase {
}
trait wxDateTime {
    fn AddDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    fn AddDateValues(_obj: *u8 /* void* */, _yrs: c_int /* int */, _mnt: c_int /* int */, _wek: c_int /* int */, _day: c_int /* int */);
    fn AddTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    fn AddTimeValues(_obj: *u8 /* void* */, _hrs: c_int /* int */, _min: c_int /* int */, _sec: c_int /* int */, _mls: c_int /* int */);
    fn ConvertYearToBC(year: c_int /* int */) -> c_int /* int */;
    fn Create() -> *u8 /* void* */;
    fn Format(_obj: *u8 /* void* */, format: *u8 /* void* */, tz: c_int /* int */) -> *u8 /* void* */;
    fn FormatDate(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn FormatISODate(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn FormatISOTime(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn FormatTime(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetAmString() -> *u8 /* void* */;
    fn GetBeginDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */);
    fn GetCentury(year: c_int /* int */) -> c_int /* int */;
    fn GetCountry() -> c_int /* int */;
    fn GetCurrentMonth(cal: c_int /* int */) -> c_int /* int */;
    fn GetCurrentYear(cal: c_int /* int */) -> c_int /* int */;
    fn GetDay(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetDayOfYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetEndDST(year: c_int /* int */, country: c_int /* int */, dt: *u8 /* void* */);
    fn GetHour(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */);
    fn GetLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */);
    fn GetMillisecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetMinute(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetMonth(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetMonthName(month: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */;
    fn GetNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */);
    fn GetNumberOfDays(year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */;
    fn GetNumberOfDaysMonth(month: c_int /* int */, year: c_int /* int */, cal: c_int /* int */) -> c_int /* int */;
    fn GetPmString() -> *u8 /* void* */;
    fn GetPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */);
    fn GetSecond(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetTicks(_obj: *u8 /* void* */) -> time_t /* time_t */;
    fn GetTimeNow() -> c_int /* int */;
    fn GetValue(_obj: *u8 /* void* */, hi_long: *u8 /* void* */, lo_long: *u8 /* void* */);
    fn GetWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */, _ref: *u8 /* void* */);
    fn GetWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */, _ref: *u8 /* void* */);
    fn GetWeekDayName(weekday: c_int /* int */, flags: c_int /* int */) -> *u8 /* void* */;
    fn GetWeekDayTZ(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn GetWeekOfMonth(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */;
    fn GetWeekOfYear(_obj: *u8 /* void* */, flags: c_int /* int */, tz: c_int /* int */) -> c_int /* int */;
    fn GetYear(_obj: *u8 /* void* */, tz: c_int /* int */) -> c_int /* int */;
    fn IsBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */;
    fn IsDST(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */;
    fn IsDSTApplicable(year: c_int /* int */, country: c_int /* int */) -> bool /* bool */;
    fn IsEarlierThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */;
    fn IsEqualTo(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */;
    fn IsEqualUpTo(_obj: *u8 /* void* */, dt: *u8 /* void* */, ts: *u8 /* void* */) -> bool /* bool */;
    // missing: wxDateTime_IsGregorianDate
    fn IsLaterThan(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> bool /* bool */;
    fn IsLeapYear(year: c_int /* int */, cal: c_int /* int */) -> bool /* bool */;
    fn IsSameDate(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */;
    fn IsSameTime(_obj: *u8 /* void* */, dt: *u8 /* void* */) -> bool /* bool */;
    fn IsStrictlyBetween(_obj: *u8 /* void* */, t1: *u8 /* void* */, t2: *u8 /* void* */) -> bool /* bool */;
    fn IsValid(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsWestEuropeanCountry(country: c_int /* int */) -> bool /* bool */;
    fn IsWorkDay(_obj: *u8 /* void* */, country: c_int /* int */) -> bool /* bool */;
    fn MakeGMT(_obj: *u8 /* void* */, noDST: c_int /* int */);
    fn MakeTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */);
    fn Now(dt: *u8 /* void* */);
    fn ParseDate(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */;
    fn ParseDateTime(_obj: *u8 /* void* */, datetime: *u8 /* void* */) -> *u8 /* void* */;
    fn ParseFormat(_obj: *u8 /* void* */, date: *u8 /* void* */, format: *u8 /* void* */, dateDef: *u8 /* void* */) -> *u8 /* void* */;
    fn ParseRfc822Date(_obj: *u8 /* void* */, date: *u8 /* void* */) -> *u8 /* void* */;
    fn ParseTime(_obj: *u8 /* void* */, time: *u8 /* void* */) -> *u8 /* void* */;
    fn ResetTime(_obj: *u8 /* void* */);
    fn Set(_obj: *u8 /* void* */, day: c_int /* int */, month: c_int /* int */, year: c_int /* int */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */);
    fn SetCountry(country: c_int /* int */);
    fn SetDay(_obj: *u8 /* void* */, day: c_int /* int */);
    fn SetHour(_obj: *u8 /* void* */, hour: c_int /* int */);
    fn SetMillisecond(_obj: *u8 /* void* */, millisecond: c_int /* int */);
    fn SetMinute(_obj: *u8 /* void* */, minute: c_int /* int */);
    fn SetMonth(_obj: *u8 /* void* */, month: c_int /* int */);
    fn SetSecond(_obj: *u8 /* void* */, second: c_int /* int */);
    fn SetTime(_obj: *u8 /* void* */, hour: c_int /* int */, minute: c_int /* int */, second: c_int /* int */, millisec: c_int /* int */);
    fn SetToCurrent(_obj: *u8 /* void* */);
    fn SetToLastMonthDay(_obj: *u8 /* void* */, month: c_int /* int */, year: c_int /* int */);
    fn SetToLastWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */;
    fn SetToNextWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */);
    fn SetToPrevWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */);
    fn SetToWeekDay(_obj: *u8 /* void* */, weekday: c_int /* int */, n: c_int /* int */, month: c_int /* int */, year: c_int /* int */) -> bool /* bool */;
    fn SetToWeekDayInSameWeek(_obj: *u8 /* void* */, weekday: c_int /* int */);
    fn SetYear(_obj: *u8 /* void* */, year: c_int /* int */);
    fn SubtractDate(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    fn SubtractTime(_obj: *u8 /* void* */, diff: *u8 /* void* */, _ref: *u8 /* void* */);
    fn ToGMT(_obj: *u8 /* void* */, noDST: c_int /* int */);
    fn ToTimezone(_obj: *u8 /* void* */, tz: c_int /* int */, noDST: c_int /* int */);
    fn Today(dt: *u8 /* void* */);
    fn UNow(dt: *u8 /* void* */);
    fn wxDateTime(hi_long: c_int /* int */, lo_long: c_int /* int */) -> *u8 /* void* */;
}
trait wxDb {
}
trait wxDbColDef {
}
trait wxDbColFor {
}
trait wxDbColInf {
}
trait wxDbConnectInf {
}
trait wxDbInf {
}
trait wxDbSqlTypeInfo {
}
trait wxDbTable {
}
trait wxDbTableInfo {
}
trait wxDebugContext {
}
trait wxDialUpEvent {
    // missing: wxDialUpEvent_IsConnectedEvent
    // missing: wxDialUpEvent_IsOwnEvent
}
trait wxDialUpManager {
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
}
trait wxDialog {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn EndModal(_obj: *u8 /* void* */, retCode: c_int /* int */);
    fn GetReturnCode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsModal(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetReturnCode(_obj: *u8 /* void* */, returnCode: c_int /* int */);
    fn ShowModal(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxDirDialog {
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetMessage(_obj: *u8 /* void* */, msg: *u8 /* void* */);
    fn SetPath(_obj: *u8 /* void* */, pth: *u8 /* void* */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
}
trait wxDirTraverser {
}
trait wxDllLoader {
}
trait wxDocChildFrame {
}
trait wxDocMDIChildFrame {
}
trait wxDocMDIParentFrame {
}
trait wxDocManager {
}
trait wxDocParentFrame {
}
trait wxDocTemplate {
}
trait wxDocument {
}
trait wxDragImage {
}
trait wxDrawControl {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait wxDrawWindow {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait wxDropFilesEvent {
}
trait wxDropSource {
    fn DropSource_Create(data: *u8 /* void* */, win: *u8 /* void* */, copy: *u8 /* void* */, move: *u8 /* void* */, none: *u8 /* void* */) -> *u8 /* void* */;
    fn DropSource_Delete(_obj: *u8 /* void* */);
    fn DropSource_DoDragDrop(_obj: *u8 /* void* */, _move: c_int /* int */) -> c_int /* int */;
}
trait wxDropTarget {
    fn GetData(_obj: *u8 /* void* */);
    fn SetDataObject(_obj: *u8 /* void* */, _dat: *u8 /* void* */);
}
trait wxDynToolInfo {
    // missing: wxDynToolInfo_Index
    // missing: wxDynToolInfo_RealSize
    // missing: wxDynToolInfo_pToolWnd
}
trait wxDynamicLibrary {
}
trait wxDynamicSashWindow {
    // missing: wxDynamicSashWindow_Create
    // missing: wxDynamicSashWindow_Delete
    // missing: wxDynamicSashWindow_GetHScrollBar
    // missing: wxDynamicSashWindow_GetVScrollBar
}
trait wxDynamicToolBar {
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
}
trait wxEditableListBox {
    // missing: wxEditableListBox_Create
    // missing: wxEditableListBox_GetDelButton
    // missing: wxEditableListBox_GetDownButton
    // missing: wxEditableListBox_GetEditButton
    // missing: wxEditableListBox_GetListCtrl
    // missing: wxEditableListBox_GetNewButton
    // missing: wxEditableListBox_GetStrings
    // missing: wxEditableListBox_GetUpButton
    // missing: wxEditableListBox_SetStrings
}
trait wxEncodingConverter {
    fn Convert(_obj: *u8 /* void* */, input: *u8 /* void* */, output: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetAllEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */;
    fn GetPlatformEquivalents(_obj: *u8 /* void* */, enc: c_int /* int */, platform: c_int /* int */, _lst: *u8 /* void* */) -> c_int /* int */;
    fn Init(_obj: *u8 /* void* */, input_enc: c_int /* int */, output_enc: c_int /* int */, method: c_int /* int */) -> c_int /* int */;
}
trait wxEraseEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxEvent {
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    fn GetEventObject(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetEventType(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSkipped(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetTimestamp(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsCommandEvent(_obj: *u8 /* void* */) -> bool /* bool */;
    fn NewEventType() -> c_int /* int */;
    fn SetEventObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn SetEventType(_obj: *u8 /* void* */, typ: c_int /* int */);
    fn SetId(_obj: *u8 /* void* */, Id: c_int /* int */);
    fn SetTimestamp(_obj: *u8 /* void* */, ts: c_int /* int */);
    fn Skip(_obj: *u8 /* void* */);
}
trait wxEvtHandler {
    fn AddPendingEvent(_obj: *u8 /* void* */, event: *u8 /* void* */);
    fn Connect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, data: *u8 /* void* */) -> c_int /* int */;
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Disconnect(_obj: *u8 /* void* */, first: c_int /* int */, last: c_int /* int */, type_: c_int /* int */, id: c_int /* int */) -> c_int /* int */;
    fn GetEvtHandlerEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetNextHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPreviousHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn ProcessEvent(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */;
    fn ProcessPendingEvents(_obj: *u8 /* void* */);
    fn SetEvtHandlerEnabled(_obj: *u8 /* void* */, enabled: bool /* bool */);
    fn SetNextHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    fn SetPreviousHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
}
trait wxExpr {
}
trait wxExprDatabase {
}
trait wxFFile {
}
trait wxFFileInputStream {
}
trait wxFFileOutputStream {
}
trait wxFSFile {
}
trait wxFTP {
}
trait wxFileDataObject {
    fn FileDataObject_AddFile(_obj: *u8 /* void* */, _fle: *u8 /* void* */);
    fn FileDataObject_Create(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    fn FileDataObject_Delete(_obj: *u8 /* void* */);
    fn FileDataObject_GetFilenames(_obj: *u8 /* void* */, _lst: *wchar_t /* wchar_t* */) -> c_int /* int */;
}
trait wxFileDialog {
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _dir: *u8 /* void* */, _fle: *u8 /* void* */, _wcd: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetDirectory(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFilenames(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn GetFilterIndex(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMessage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPaths(_obj: *u8 /* void* */, paths: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWildcard(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetDirectory(_obj: *u8 /* void* */, dir: *u8 /* void* */);
    fn SetFilename(_obj: *u8 /* void* */, name: *u8 /* void* */);
    fn SetFilterIndex(_obj: *u8 /* void* */, filterIndex: c_int /* int */);
    fn SetMessage(_obj: *u8 /* void* */, message: *u8 /* void* */);
    fn SetPath(_obj: *u8 /* void* */, path: *u8 /* void* */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    fn SetWildcard(_obj: *u8 /* void* */, wildCard: *u8 /* void* */);
}
trait wxFileDropTarget {
}
trait wxFileHistory {
    fn AddFileToHistory(_obj: *u8 /* void* */, file: *u8 /* void* */);
    fn AddFilesToMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    fn Create(maxFiles: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetHistoryFile(_obj: *u8 /* void* */, i: c_int /* int */) -> *u8 /* void* */;
    fn GetMaxFiles(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMenus(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    fn Load(_obj: *u8 /* void* */, config: *u8 /* void* */);
    fn RemoveFileFromHistory(_obj: *u8 /* void* */, i: c_int /* int */);
    fn RemoveMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    fn Save(_obj: *u8 /* void* */, config: *u8 /* void* */);
    fn UseMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
}
trait wxFileInputStream {
}
trait wxFileName {
}
trait wxFileOutputStream {
}
trait wxFileSystem {
}
trait wxFileSystemHandler {
}
trait wxFileType {
    fn Delete(_obj: *u8 /* void* */);
    fn ExpandCommand(_obj: *u8 /* void* */, _cmd: *u8 /* void* */, _params: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDescription(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetExtensions(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    fn GetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */;
    fn GetMimeType(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMimeTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    fn GetOpenCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */;
    fn GetPrintCommand(_obj: *u8 /* void* */, _buf: *u8 /* void* */, _params: *u8 /* void* */) -> c_int /* int */;
}
trait wxFilterInputStream {
}
trait wxFilterOutputStream {
}
trait wxFindDialogEvent {
    fn GetFindString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetReplaceString(_obj: *u8 /* void* */, _ref: *u8 /* void* */) -> c_int /* int */;
}
trait wxFindReplaceData {
    fn Create(flags: c_int /* int */) -> *u8 /* void* */;
    fn CreateDefault() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetFindString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetReplaceString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetFindString(_obj: *u8 /* void* */, str: *u8 /* void* */);
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    fn SetReplaceString(_obj: *u8 /* void* */, str: *u8 /* void* */);
}
trait wxFindReplaceDialog {
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */, title: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */);
}
trait wxFlexGridSizer {
    fn AddGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    fn AddGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */;
    fn RecalcSizes(_obj: *u8 /* void* */);
    fn RemoveGrowableCol(_obj: *u8 /* void* */, idx: size_t /* size_t */);
    fn RemoveGrowableRow(_obj: *u8 /* void* */, idx: size_t /* size_t */);
}
trait wxFocusEvent {
}
trait wxFont {
    fn Create(pointSize: c_int /* int */, family: c_int /* int */, style: c_int /* int */, weight: c_int /* int */, underlined: bool /* bool */, face: *u8 /* void* */, enc: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    fn CreateDefault() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetDefaultEncoding(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFaceName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFamily(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFamilyString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPointSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetStyleString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetUnderlined(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWeightString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetDefaultEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */);
    fn SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */);
    fn SetFaceName(_obj: *u8 /* void* */, faceName: *u8 /* void* */);
    fn SetFamily(_obj: *u8 /* void* */, family: c_int /* int */);
    fn SetPointSize(_obj: *u8 /* void* */, pointSize: c_int /* int */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    fn SetUnderlined(_obj: *u8 /* void* */, underlined: c_int /* int */);
    fn SetWeight(_obj: *u8 /* void* */, weight: c_int /* int */);
}
trait wxFontData {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn EnableEffects(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn GetAllowSymbols(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetChosenFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */);
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetEnableEffects(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEncoding(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetInitialFont(_obj: *u8 /* void* */, ref_: *u8 /* void* */);
    fn GetShowHelp(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetAllowSymbols(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetChosenFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */);
    fn SetInitialFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetRange(_obj: *u8 /* void* */, minRange: c_int /* int */, maxRange: c_int /* int */);
    fn SetShowHelp(_obj: *u8 /* void* */, flag: bool /* bool */);
}
trait wxFontDialog {
    fn Create(_prt: *u8 /* void* */, fnt: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFontData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
}
trait wxFontEnumerator {
    fn Create(_obj: *u8 /* void* */, _fnc: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn EnumerateEncodings(_obj: *u8 /* void* */, facename: *u8 /* void* */) -> bool /* bool */;
    fn EnumerateFacenames(_obj: *u8 /* void* */, encoding: c_int /* int */, fixedWidthOnly: c_int /* int */) -> bool /* bool */;
}
trait wxFontList {
}
trait wxFontMapper {
    fn Create() -> *u8 /* void* */;
    fn GetAltForEncoding(_obj: *u8 /* void* */, encoding: c_int /* int */, alt_encoding: *u8 /* void* */, _buf: *u8 /* void* */) -> bool /* bool */;
    fn IsEncodingAvailable(_obj: *u8 /* void* */, encoding: c_int /* int */, _buf: *u8 /* void* */) -> bool /* bool */;
}
trait wxFrame {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn CreateStatusBar(_obj: *u8 /* void* */, number: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    fn CreateToolBar(_obj: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */;
    fn GetClientAreaOrigin_left(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetClientAreaOrigin_top(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStatusBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetToolBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Restore(_obj: *u8 /* void* */);
    fn SetMenuBar(_obj: *u8 /* void* */, menubar: *u8 /* void* */);
    fn SetStatusBar(_obj: *u8 /* void* */, statBar: *u8 /* void* */);
    fn SetStatusText(_obj: *u8 /* void* */, _txt: *u8 /* void* */, _number: c_int /* int */);
    fn SetStatusWidths(_obj: *u8 /* void* */, _n: c_int /* int */, _widths_field: *u8 /* void* */);
    fn SetToolBar(_obj: *u8 /* void* */, _toolbar: *u8 /* void* */);
}
trait wxFrameLayout {
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
}
trait wxGDIObject {
}
trait wxGLCanvas {
}
trait wxGauge {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _rng: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetBezelFace(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRange(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetShadowWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetBezelFace(_obj: *u8 /* void* */, w: c_int /* int */);
    fn SetRange(_obj: *u8 /* void* */, r: c_int /* int */);
    fn SetShadowWidth(_obj: *u8 /* void* */, w: c_int /* int */);
    fn SetValue(_obj: *u8 /* void* */, pos: c_int /* int */);
}
trait wxGenericDirCtrl {
}
trait wxGenericValidator {
}
trait wxGrid {
    fn AppendCols(_obj: *u8 /* void* */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    fn AppendRows(_obj: *u8 /* void* */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    fn AutoSize(_obj: *u8 /* void* */);
    fn AutoSizeColumn(_obj: *u8 /* void* */, col: c_int /* int */, setAsMin: c_int /* int */);
    fn AutoSizeColumns(_obj: *u8 /* void* */, setAsMin: c_int /* int */);
    fn AutoSizeRow(_obj: *u8 /* void* */, row: c_int /* int */, setAsMin: c_int /* int */);
    fn AutoSizeRows(_obj: *u8 /* void* */, setAsMin: c_int /* int */);
    fn BeginBatch(_obj: *u8 /* void* */);
    fn BlockToDeviceRect(_obj: *u8 /* void* */, top: c_int /* int */, left: c_int /* int */, bottom: c_int /* int */, right: c_int /* int */) -> *u8 /* void* */;
    fn CanDragColSize(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanDragGridSize(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanDragRowSize(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanEnableCellControl(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CellToRect(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn ClearGrid(_obj: *u8 /* void* */);
    fn ClearSelection(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn CreateGrid(_obj: *u8 /* void* */, rows: c_int /* int */, cols: c_int /* int */, selmode: c_int /* int */);
    fn DeleteCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    fn DeleteRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    fn DisableCellEditControl(_obj: *u8 /* void* */);
    fn DisableDragColSize(_obj: *u8 /* void* */);
    fn DisableDragGridSize(_obj: *u8 /* void* */);
    fn DisableDragRowSize(_obj: *u8 /* void* */);
    fn DrawAllGridLines(_obj: *u8 /* void* */, dc: *u8 /* void* */, reg: *u8 /* void* */);
    fn DrawCell(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */);
    fn DrawCellBorder(_obj: *u8 /* void* */, dc: *u8 /* void* */, _row: c_int /* int */, _col: c_int /* int */);
    fn DrawCellHighlight(_obj: *u8 /* void* */, dc: *u8 /* void* */, attr: *u8 /* void* */);
    fn DrawColLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, col: c_int /* int */);
    fn DrawColLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    fn DrawGridSpace(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    fn DrawRowLabel(_obj: *u8 /* void* */, dc: *u8 /* void* */, row: c_int /* int */);
    fn DrawRowLabels(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    fn DrawTextRectangle(_obj: *u8 /* void* */, dc: *u8 /* void* */, txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, horizontalAlignment: c_int /* int */, verticalAlignment: c_int /* int */);
    fn EnableCellEditControl(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn EnableDragColSize(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn EnableDragGridSize(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn EnableDragRowSize(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn EnableEditing(_obj: *u8 /* void* */, edit: c_int /* int */);
    fn EnableGridLines(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn EndBatch(_obj: *u8 /* void* */);
    fn GetBatchCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    fn GetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */);
    fn GetCellHighlightColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    fn GetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetColLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetColSize(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */;
    fn GetDefaultCellAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetDefaultCellBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetDefaultCellFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetDefaultCellTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetDefaultColLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetDefaultColSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetDefaultEditor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDefaultEditorForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetDefaultEditorForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDefaultRenderer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDefaultRendererForCell(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetDefaultRendererForType(_obj: *u8 /* void* */, typeName: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDefaultRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetDefaultRowSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetGridCursorCol(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetGridCursorRow(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetGridLineColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetLabelBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetLabelFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetLabelTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetNumberCols(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetNumberRows(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRowLabelAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetRowLabelSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */) -> *u8 /* void* */;
    fn GetRowSize(_obj: *u8 /* void* */, row: c_int /* int */) -> c_int /* int */;
    fn GetSelectionBackground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetSelectionForeground(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetTable(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTextBoxSize(_obj: *u8 /* void* */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    fn GridLinesEnabled(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HideCellEditControl(_obj: *u8 /* void* */);
    fn InsertCols(_obj: *u8 /* void* */, pos: c_int /* int */, numCols: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    fn InsertRows(_obj: *u8 /* void* */, pos: c_int /* int */, numRows: c_int /* int */, updateLabels: bool /* bool */) -> bool /* bool */;
    fn IsCellEditControlEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsCellEditControlShown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsCurrentCellReadOnly(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsEditable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsInSelection(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */;
    fn IsReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> bool /* bool */;
    fn IsSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, wholeCellVisible: bool /* bool */) -> bool /* bool */;
    fn MakeCellVisible(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */);
    fn MoveCursorDown(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorDownBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorLeft(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorLeftBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorRight(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorRightBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorUp(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MoveCursorUpBlock(_obj: *u8 /* void* */, expandSelection: bool /* bool */) -> bool /* bool */;
    fn MovePageDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MovePageUp(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ProcessTableMessage(_obj: *u8 /* void* */, evt: *u8 /* void* */) -> bool /* bool */;
    fn RegisterDataType(_obj: *u8 /* void* */, typeName: *u8 /* void* */, renderer: *u8 /* void* */, editor: *u8 /* void* */);
    fn SaveEditControlValue(_obj: *u8 /* void* */);
    fn SelectAll(_obj: *u8 /* void* */);
    fn SelectBlock(_obj: *u8 /* void* */, topRow: c_int /* int */, leftCol: c_int /* int */, bottomRow: c_int /* int */, rightCol: c_int /* int */, addToSelected: c_int /* int */);
    fn SelectCol(_obj: *u8 /* void* */, col: c_int /* int */, addToSelected: c_int /* int */);
    fn SelectRow(_obj: *u8 /* void* */, row: c_int /* int */, addToSelected: c_int /* int */);
    fn SetCellAlignment(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, horiz: c_int /* int */, vert: c_int /* int */);
    fn SetCellBackgroundColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    fn SetCellEditor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, editor: *u8 /* void* */);
    fn SetCellFont(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, font: *u8 /* void* */);
    fn SetCellHighlightColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetCellRenderer(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, renderer: *u8 /* void* */);
    fn SetCellTextColour(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, colour: *u8 /* void* */);
    fn SetCellValue(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, s: *u8 /* void* */);
    fn SetColAttr(_obj: *u8 /* void* */, col: c_int /* int */, attr: *u8 /* void* */);
    fn SetColFormatBool(_obj: *u8 /* void* */, col: c_int /* int */);
    fn SetColFormatCustom(_obj: *u8 /* void* */, col: c_int /* int */, typeName: *u8 /* void* */);
    fn SetColFormatFloat(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */, precision: c_int /* int */);
    fn SetColFormatNumber(_obj: *u8 /* void* */, col: c_int /* int */);
    fn SetColLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */);
    fn SetColLabelSize(_obj: *u8 /* void* */, height: c_int /* int */);
    fn SetColLabelValue(_obj: *u8 /* void* */, col: c_int /* int */, label: *u8 /* void* */);
    fn SetColMinimalWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */);
    fn SetColSize(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */);
    fn SetDefaultCellAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */);
    fn SetDefaultCellBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetDefaultCellFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetDefaultCellTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetDefaultColSize(_obj: *u8 /* void* */, width: c_int /* int */, resizeExistingCols: c_int /* int */);
    fn SetDefaultEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */);
    fn SetDefaultRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */);
    fn SetDefaultRowSize(_obj: *u8 /* void* */, height: c_int /* int */, resizeExistingRows: c_int /* int */);
    fn SetGridCursor(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */);
    fn SetGridLineColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetLabelBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetLabelFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetLabelTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn SetMargins(_obj: *u8 /* void* */, extraWidth: c_int /* int */, extraHeight: c_int /* int */);
    fn SetReadOnly(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, isReadOnly: bool /* bool */);
    fn SetRowAttr(_obj: *u8 /* void* */, row: c_int /* int */, attr: *u8 /* void* */);
    fn SetRowLabelAlignment(_obj: *u8 /* void* */, horiz: c_int /* int */, vert: c_int /* int */);
    fn SetRowLabelSize(_obj: *u8 /* void* */, width: c_int /* int */);
    fn SetRowLabelValue(_obj: *u8 /* void* */, row: c_int /* int */, label: *u8 /* void* */);
    fn SetRowMinimalHeight(_obj: *u8 /* void* */, row: c_int /* int */, width: c_int /* int */);
    fn SetRowSize(_obj: *u8 /* void* */, row: c_int /* int */, height: c_int /* int */);
    fn SetSelectionBackground(_obj: *u8 /* void* */, c: *u8 /* void* */);
    fn SetSelectionForeground(_obj: *u8 /* void* */, c: *u8 /* void* */);
    fn SetSelectionMode(_obj: *u8 /* void* */, selmode: c_int /* int */);
    fn SetTable(_obj: *u8 /* void* */, table: *u8 /* void* */, takeOwnership: bool /* bool */, selmode: c_int /* int */) -> bool /* bool */;
    fn ShowCellEditControl(_obj: *u8 /* void* */);
    fn StringToLines(_obj: *u8 /* void* */, value: *u8 /* void* */, lines: *u8 /* void* */) -> c_int /* int */;
    fn XToCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    fn XToEdgeOfCol(_obj: *u8 /* void* */, x: c_int /* int */) -> c_int /* int */;
    fn XYToCell(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    fn YToEdgeOfRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    fn YToRow(_obj: *u8 /* void* */, y: c_int /* int */) -> c_int /* int */;
    fn GetSelectedCells(_obj: *u8 /* void* */, _arr: *u8 /* void* */);
    fn GetSelectionBlockTopLeft(_obj: *u8 /* void* */, _arr: *u8 /* void* */);
    fn GetSelectionBlockBottomRight(_obj: *u8 /* void* */, _arr: *u8 /* void* */);
    fn GetSelectedRows(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */;
    fn GetSelectedCols(_obj: *u8 /* void* */, _arr: *intptr_t /* intptr_t* */) -> c_int /* int */;
    fn GetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn SetCellSize(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */);
}
trait wxGridCellAttr {
    fn Ctor() -> *u8 /* void* */;
    fn DecRef(_obj: *u8 /* void* */);
    fn GetAlignment(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetEditor(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetRenderer(_obj: *u8 /* void* */, grid: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */) -> *u8 /* void* */;
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn HasAlignment(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasEditor(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasFont(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasRenderer(_obj: *u8 /* void* */) -> bool /* bool */;
    fn HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IncRef(_obj: *u8 /* void* */);
    fn IsReadOnly(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetAlignment(_obj: *u8 /* void* */, hAlign: c_int /* int */, vAlign: c_int /* int */);
    fn SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */);
    fn SetDefAttr(_obj: *u8 /* void* */, defAttr: *u8 /* void* */);
    fn SetEditor(_obj: *u8 /* void* */, editor: *u8 /* void* */);
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetReadOnly(_obj: *u8 /* void* */, isReadOnly: bool /* bool */);
    fn SetRenderer(_obj: *u8 /* void* */, renderer: *u8 /* void* */);
    fn SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */);
}
trait wxGridCellBoolEditor {
    fn Ctor() -> *u8 /* void* */;
}
trait wxGridCellBoolRenderer {
}
trait wxGridCellChoiceEditor {
    fn Ctor(arg0: c_int /* int */, arg1: *wchar_t /* wchar_t* */, allowOthers: c_int /* int */) -> *u8 /* void* */;
}
trait wxGridCellCoordsArray {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn Item(_obj: *u8 /* void* */, _idx: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
}
trait wxGridCellEditor {
    fn BeginEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */);
    fn Create(_obj: *u8 /* void* */, parent: *u8 /* void* */, id: c_int /* int */, evtHandler: *u8 /* void* */);
    fn Destroy(_obj: *u8 /* void* */);
    fn EndEdit(_obj: *u8 /* void* */, row: c_int /* int */, col: c_int /* int */, grid: *u8 /* void* */, oldStr: *u8 /* void* */, newStr: *u8 /* void* */) -> c_int /* int */;
    fn GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn HandleReturn(_obj: *u8 /* void* */, event: *u8 /* void* */);
    fn IsAcceptedKey(_obj: *u8 /* void* */, event: *u8 /* void* */) -> bool /* bool */;
    fn IsCreated(_obj: *u8 /* void* */) -> bool /* bool */;
    fn PaintBackground(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, attr: *u8 /* void* */);
    fn Reset(_obj: *u8 /* void* */);
    fn SetControl(_obj: *u8 /* void* */, control: *u8 /* void* */);
    fn SetParameters(_obj: *u8 /* void* */, params: *u8 /* void* */);
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn Show(_obj: *u8 /* void* */, show: c_int /* int */, attr: *u8 /* void* */);
    fn StartingClick(_obj: *u8 /* void* */);
    fn StartingKey(_obj: *u8 /* void* */, event: *u8 /* void* */);
}
trait wxGridCellFloatEditor {
    fn Ctor(width: c_int /* int */, precision: c_int /* int */) -> *u8 /* void* */;
}
trait wxGridCellFloatRenderer {
}
trait wxGridCellNumberEditor {
    fn Ctor(min: c_int /* int */, max: c_int /* int */) -> *u8 /* void* */;
}
trait wxGridCellNumberRenderer {
    fn Ctor() -> *u8 /* void* */;
}
trait wxGridCellAutoWrapStringRenderer {
    fn Ctor() -> *u8 /* void* */;
}
trait wxGridCellRenderer {
}
trait wxGridCellStringRenderer {
}
trait wxGridCellTextEditor {
    fn Ctor() -> *u8 /* void* */;
}
trait wxGridCellWorker {
}
trait wxGridEditorCreatedEvent {
    fn GetCol(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetCol(_obj: *u8 /* void* */, col: c_int /* int */);
    fn SetControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */);
    fn SetRow(_obj: *u8 /* void* */, row: c_int /* int */);
}
trait wxGridEvent {
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetCol(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetRow(_obj: *u8 /* void* */) -> c_int /* int */;
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Selecting(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxGridRangeSelectEvent {
    fn GetTopLeftCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetBottomRightCoords(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetTopRow(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetBottomRow(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetLeftCol(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRightCol(_obj: *u8 /* void* */) -> c_int /* int */;
    fn Selecting(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxGridSizeEvent {
    fn GetRowOrCol(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxGridSizer {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(rows: c_int /* int */, cols: c_int /* int */, vgap: c_int /* int */, hgap: c_int /* int */) -> *u8 /* void* */;
    fn GetCols(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetHGap(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRows(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetVGap(_obj: *u8 /* void* */) -> c_int /* int */;
    fn RecalcSizes(_obj: *u8 /* void* */);
    fn SetCols(_obj: *u8 /* void* */, cols: c_int /* int */);
    fn SetHGap(_obj: *u8 /* void* */, gap: c_int /* int */);
    fn SetRows(_obj: *u8 /* void* */, rows: c_int /* int */);
    fn SetVGap(_obj: *u8 /* void* */, gap: c_int /* int */);
}
trait wxGridTableBase {
}
trait wxHTTP {
}
trait wxHashMap {
}
trait wxHelpController {
}
trait wxHelpControllerBase {
}
trait wxHelpControllerHelpProvider {
    fn Create(ctr: *u8 /* void* */) -> *u8 /* void* */;
    fn GetHelpController(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetHelpController(_obj: *u8 /* void* */, hc: *u8 /* void* */);
}
trait wxHelpEvent {
    fn GetLink(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTarget(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetLink(_obj: *u8 /* void* */, link: *u8 /* void* */);
    fn SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetTarget(_obj: *u8 /* void* */, target: *u8 /* void* */);
}
trait wxHelpProvider {
    fn AddHelp(_obj: *u8 /* void* */, window: *u8 /* void* */, text: *u8 /* void* */);
    fn AddHelpById(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */);
    fn Delete(_obj: *u8 /* void* */);
    fn Get() -> *u8 /* void* */;
    fn GetHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> *u8 /* void* */;
    fn RemoveHelp(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn Set(helpProvider: *u8 /* void* */) -> *u8 /* void* */;
    fn ShowHelp(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
}
trait wxHtmlCell {
}
trait wxHtmlColourCell {
}
trait wxHtmlContainerCell {
}
trait wxHtmlDCRenderer {
}
trait wxHtmlEasyPrinting {
}
trait wxHtmlFilter {
}
trait wxHtmlHelpController {
    fn AddBook(_obj: *u8 /* void* */, book: *u8 /* void* */, show_wait_msg: c_int /* int */) -> bool /* bool */;
    fn Create(_style: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Display(_obj: *u8 /* void* */, x: *u8 /* void* */) -> c_int /* int */;
    fn DisplayBlock(_obj: *u8 /* void* */, blockNo: c_int /* int */) -> bool /* bool */;
    fn DisplayContents(_obj: *u8 /* void* */) -> c_int /* int */;
    fn DisplayIndex(_obj: *u8 /* void* */) -> c_int /* int */;
    fn DisplayNumber(_obj: *u8 /* void* */, id: c_int /* int */) -> c_int /* int */;
    fn DisplaySection(_obj: *u8 /* void* */, section: *u8 /* void* */) -> bool /* bool */;
    fn DisplaySectionNumber(_obj: *u8 /* void* */, sectionNo: c_int /* int */) -> bool /* bool */;
    fn GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, width: *c_int /* int* */, height: *c_int /* int* */, pos_x: *c_int /* int* */, pos_y: *c_int /* int* */, newFrameEachTime: *c_int /* int* */) -> *u8 /* void* */;
    fn Initialize(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    fn KeywordSearch(_obj: *u8 /* void* */, keyword: *u8 /* void* */) -> bool /* bool */;
    fn LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    fn Quit(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
    fn SetFrameParameters(_obj: *u8 /* void* */, title: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, pos_x: c_int /* int */, pos_y: c_int /* int */, newFrameEachTime: bool /* bool */);
    fn SetTempDir(_obj: *u8 /* void* */, path: *u8 /* void* */);
    fn SetTitleFormat(_obj: *u8 /* void* */, format: *u8 /* void* */);
    fn SetViewer(_obj: *u8 /* void* */, viewer: *u8 /* void* */, flags: c_int /* int */);
    fn UseConfig(_obj: *u8 /* void* */, config: *u8 /* void* */, rootpath: *u8 /* void* */);
    fn WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
}
trait wxHtmlHelpData {
}
trait wxHtmlHelpFrame {
}
trait wxHtmlLinkInfo {
}
trait wxHtmlParser {
}
trait wxHtmlPrintout {
}
trait wxHtmlTag {
}
trait wxHtmlTagHandler {
}
trait wxHtmlTagsModule {
}
trait wxHtmlWidgetCell {
}
trait wxHtmlWinParser {
}
trait wxHtmlWinTagHandler {
}
trait wxHtmlWindow {
}
trait wxIPV4address {
}
trait wxIcon {
    fn Assign(_obj: *u8 /* void* */, other: *u8 /* void* */);
    fn CopyFromBitmap(_obj: *u8 /* void* */, bmp: *u8 /* void* */);
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateLoad(name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn FromRaw(data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn FromXPM(data: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDepth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsEqual(_obj: *u8 /* void* */, other: *u8 /* void* */) -> bool /* bool */;
    fn Load(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_long /* long */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetDepth(_obj: *u8 /* void* */, depth: c_int /* int */);
    fn SetHeight(_obj: *u8 /* void* */, height: c_int /* int */);
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */);
}
trait wxIconBundle {
    fn AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */);
    fn AddIconFromFile(_obj: *u8 /* void* */, file: *u8 /* void* */, type_: c_int /* int */);
    fn Assign(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateFromFile(file: *u8 /* void* */, type_: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromIcon(icon: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetIcon(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, _ref: *u8 /* void* */);
}
trait wxIconizeEvent {
}
trait wxIdleEvent {
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    fn MoreRequested(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RequestMore(_obj: *u8 /* void* */, needMore: bool /* bool */);
}
trait wxImage {
    fn CanRead(name: *u8 /* void* */) -> bool /* bool */;
    fn ConvertToBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    fn ConvertToByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */;
    fn ConvertToLazyByteString(_obj: *u8 /* void* */, type_: c_int /* int */, data: *char /* char* */) -> c_int /* int */;
    fn CountColours(_obj: *u8 /* void* */, stopafter: c_int /* int */) -> c_int /* int */;
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateFromBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateFromByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromLazyByteString(arg0: *char /* char* */, arg1: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromData(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateFromFile(name: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateSized(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn Destroy(_obj: *u8 /* void* */);
    fn GetBlue(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */;
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetGreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */;
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMaskBlue(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    fn GetMaskGreen(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    fn GetMaskRed(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    fn GetRed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> wchar_t /* wchar_t */;
    fn GetSubImage(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, image: *u8 /* void* */);
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HasMask(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn GetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */;
    fn HasOption(_obj: *u8 /* void* */, name: *u8 /* void* */) -> bool /* bool */;
    fn Initialize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn InitializeFromData(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */);
    fn LoadFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */;
    fn Mirror(_obj: *u8 /* void* */, horizontally: c_int /* int */, image: *u8 /* void* */);
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Paste(_obj: *u8 /* void* */, image: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn Replace(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */, arg5: u8 /* u8 */);
    fn Rescale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn Rotate(_obj: *u8 /* void* */, angle: c_double /* double */, arg0: c_int /* int */, arg1: c_int /* int */, interpolating: c_int /* int */, offset_after_rotation: *u8 /* void* */, image: *u8 /* void* */);
    fn Rotate90(_obj: *u8 /* void* */, clockwise: c_int /* int */, image: *u8 /* void* */);
    fn SaveFile(_obj: *u8 /* void* */, name: *u8 /* void* */, type_: c_int /* int */) -> bool /* bool */;
    fn Scale(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, image: *u8 /* void* */);
    fn SetData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn SetDataAndSize(_obj: *u8 /* void* */, data: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetMask(_obj: *u8 /* void* */, mask: c_int /* int */);
    fn SetMaskColour(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */);
    fn SetOption(_obj: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */);
    fn SetOptionInt(_obj: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */);
    fn SetRGB(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: u8 /* u8 */, arg3: u8 /* u8 */, arg4: u8 /* u8 */);
}
trait wxImageHandler {
}
trait wxImageList {
    fn AddBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> c_int /* int */;
    fn AddIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */) -> c_int /* int */;
    fn AddMasked(_obj: *u8 /* void* */, bitmap: *u8 /* void* */, maskColour: *u8 /* void* */) -> c_int /* int */;
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, mask: c_int /* int */, initialCount: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Draw(_obj: *u8 /* void* */, index: c_int /* int */, dc: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: c_int /* int */, solidBackground: bool /* bool */) -> bool /* bool */;
    fn GetImageCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSize(_obj: *u8 /* void* */, index: c_int /* int */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn Remove(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    fn RemoveAll(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Replace(_obj: *u8 /* void* */, index: c_int /* int */, bitmap: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */;
    fn ReplaceIcon(_obj: *u8 /* void* */, index: c_int /* int */, icon: *u8 /* void* */) -> bool /* bool */;
}
trait wxIndividualLayoutConstraint {
    fn Above(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    fn Absolute(_obj: *u8 /* void* */, val: c_int /* int */);
    fn AsIs(_obj: *u8 /* void* */);
    fn Below(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    fn GetDone(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEdge(_obj: *u8 /* void* */, which: c_int /* int */, thisWin: *u8 /* void* */, other: *u8 /* void* */) -> c_int /* int */;
    fn GetMargin(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMyEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetOtherEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetOtherWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPercent(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRelationship(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    fn LeftOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    fn PercentOf(_obj: *u8 /* void* */, otherW: *u8 /* void* */, wh: c_int /* int */, per: c_int /* int */);
    fn ResetIfWin(_obj: *u8 /* void* */, otherW: *u8 /* void* */) -> bool /* bool */;
    fn RightOf(_obj: *u8 /* void* */, sibling: *u8 /* void* */, marg: c_int /* int */);
    fn SameAs(_obj: *u8 /* void* */, otherW: *u8 /* void* */, edge: c_int /* int */, marg: c_int /* int */);
    fn SatisfyConstraint(_obj: *u8 /* void* */, constraints: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
    fn Set(_obj: *u8 /* void* */, rel: c_int /* int */, otherW: *u8 /* void* */, otherE: c_int /* int */, val: c_int /* int */, marg: c_int /* int */);
    fn SetDone(_obj: *u8 /* void* */, d: bool /* bool */);
    fn SetEdge(_obj: *u8 /* void* */, which: c_int /* int */);
    fn SetMargin(_obj: *u8 /* void* */, m: c_int /* int */);
    fn SetRelationship(_obj: *u8 /* void* */, r: c_int /* int */);
    fn SetValue(_obj: *u8 /* void* */, v: c_int /* int */);
    fn Unconstrained(_obj: *u8 /* void* */);
}
trait wxInitDialogEvent {
}
trait wxInputStream {
    fn Delete(_obj: *u8 /* void* */);
    fn Eof(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetC(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    fn LastRead(_obj: *u8 /* void* */) -> c_int /* int */;
    fn Peek(_obj: *u8 /* void* */) -> wchar_t /* wchar_t */;
    fn Read(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */);
    fn SeekI(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */;
    fn Tell(_obj: *u8 /* void* */) -> c_int /* int */;
    fn UngetBuffer(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */) -> c_int /* int */;
    fn Ungetch(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */) -> c_int /* int */;
}
trait wxJoystick {
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
}
trait wxJoystickEvent {
    fn ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetButtonChange(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetButtonState(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetJoystick(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetZPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsButton(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsMove(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsZMove(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetButtonChange(_obj: *u8 /* void* */, change: c_int /* int */);
    fn SetButtonState(_obj: *u8 /* void* */, state: c_int /* int */);
    fn SetJoystick(_obj: *u8 /* void* */, stick: c_int /* int */);
    fn SetPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetZPosition(_obj: *u8 /* void* */, zPos: c_int /* int */);
}
trait wxKeyEvent {
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetKeyCode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetModifiers(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HasModifiers(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetKeyCode(_obj: *u8 /* void* */, code: c_int /* int */);
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxLEDNumberCtrl {
    // missing: wxLEDNumberCtrl_Create
    // missing: wxLEDNumberCtrl_GetAlignment
    // missing: wxLEDNumberCtrl_GetDrawFaded
    // missing: wxLEDNumberCtrl_GetValue
    // missing: wxLEDNumberCtrl_SetAlignment
    // missing: wxLEDNumberCtrl_SetDrawFaded
    // missing: wxLEDNumberCtrl_SetValue
}
trait wxLayoutAlgorithm {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn LayoutFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */;
    fn LayoutMDIFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, use_: c_int /* int */) -> bool /* bool */;
    fn LayoutWindow(_obj: *u8 /* void* */, frame: *u8 /* void* */, mainWindow: *u8 /* void* */) -> bool /* bool */;
}
trait wxLayoutConstraints {
    fn Create() -> *u8 /* void* */;
    fn bottom(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn centreX(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn centreY(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn height(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn left(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn right(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn top(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn width(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxList {
}
trait wxListBox {
    fn Append(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn AppendData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */);
    fn Clear(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */, n: c_int /* int */);
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    fn GetClientData(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelections(_obj: *u8 /* void* */, aSelections: *c_int /* int* */, allocated: c_int /* int */) -> c_int /* int */;
    fn GetString(_obj: *u8 /* void* */, n: c_int /* int */) -> *u8 /* void* */;
    fn InsertItems(_obj: *u8 /* void* */, items: *u8 /* void* */, pos: c_int /* int */, count: c_int /* int */);
    fn IsSelected(_obj: *u8 /* void* */, n: c_int /* int */) -> bool /* bool */;
    fn SetClientData(_obj: *u8 /* void* */, n: c_int /* int */, clientData: *u8 /* void* */);
    fn SetFirstItem(_obj: *u8 /* void* */, n: c_int /* int */);
    fn SetSelection(_obj: *u8 /* void* */, n: c_int /* int */, select: c_int /* int */);
    fn SetString(_obj: *u8 /* void* */, n: c_int /* int */, s: *u8 /* void* */);
    fn SetStringSelection(_obj: *u8 /* void* */, str: *u8 /* void* */, sel: bool /* bool */);
}
trait wxListCtrl {
    fn Arrange(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */;
    fn ClearAll(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn DeleteAllColumns(_obj: *u8 /* void* */) -> bool /* bool */;
    fn DeleteAllItems(_obj: *u8 /* void* */) -> bool /* bool */;
    fn DeleteColumn(_obj: *u8 /* void* */, col: c_int /* int */) -> bool /* bool */;
    fn DeleteItem(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
    fn EditLabel(_obj: *u8 /* void* */, item: c_int /* int */);
    fn EndEditLabel(_obj: *u8 /* void* */, cancel: c_int /* int */) -> bool /* bool */;
    fn EnsureVisible(_obj: *u8 /* void* */, item: c_int /* int */) -> bool /* bool */;
    fn FindItem(_obj: *u8 /* void* */, start: c_int /* int */, str: *u8 /* void* */, partial: bool /* bool */) -> c_int /* int */;
    fn FindItemByData(_obj: *u8 /* void* */, start: c_int /* int */, data: c_int /* int */) -> c_int /* int */;
    fn FindItemByPosition(_obj: *u8 /* void* */, start: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, direction: c_int /* int */) -> c_int /* int */;
    fn GetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */;
    fn GetColumnCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */) -> c_int /* int */;
    fn GetCountPerPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetImageList(_obj: *u8 /* void* */, which: c_int /* int */) -> *u8 /* void* */;
    fn GetItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */;
    fn GetItemCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetItemData(_obj: *u8 /* void* */, item: c_int /* int */) -> c_int /* int */;
    fn GetItemFont(_obj: *u8 /* void* */, item: c_long /* long */) -> *u8 /* void* */;
    fn GetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    fn GetItemRect(_obj: *u8 /* void* */, item: c_int /* int */, code: c_int /* int */) -> *u8 /* void* */;
    fn GetItemSpacing(_obj: *u8 /* void* */, isSmall: bool /* bool */) -> *u8 /* void* */;
    fn GetItemState(_obj: *u8 /* void* */, item: c_int /* int */, stateMask: c_int /* int */) -> c_int /* int */;
    fn GetItemText(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    fn GetNextItem(_obj: *u8 /* void* */, item: c_int /* int */, geometry: c_int /* int */, state: c_int /* int */) -> c_int /* int */;
    fn GetSelectedItemCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetTopItem(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *u8 /* void* */) -> c_int /* int */;
    fn InsertColumn(_obj: *u8 /* void* */, col: c_int /* int */, heading: *u8 /* void* */, format: c_int /* int */, width: c_int /* int */) -> c_int /* int */;
    fn InsertColumnFromInfo(_obj: *u8 /* void* */, col: c_int /* int */, info: *u8 /* void* */) -> c_int /* int */;
    fn InsertItem(_obj: *u8 /* void* */, info: *u8 /* void* */) -> c_int /* int */;
    fn InsertItemWithData(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */) -> c_int /* int */;
    fn InsertItemWithImage(_obj: *u8 /* void* */, index: c_int /* int */, imageIndex: c_int /* int */) -> c_int /* int */;
    fn InsertItemWithLabel(_obj: *u8 /* void* */, index: c_int /* int */, label: *u8 /* void* */, imageIndex: c_int /* int */) -> c_int /* int */;
    fn IsVirtual(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RefreshItem(_obj: *u8 /* void* */, item: c_long /* long */);
    fn ScrollList(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    fn SetBackgroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetColumn(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */) -> bool /* bool */;
    fn SetColumnWidth(_obj: *u8 /* void* */, col: c_int /* int */, width: c_int /* int */) -> bool /* bool */;
    fn SetForegroundColour(_obj: *u8 /* void* */, col: *u8 /* void* */) -> c_int /* int */;
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */, which: c_int /* int */);
    fn SetItem(_obj: *u8 /* void* */, index: c_int /* int */, col: c_int /* int */, label: *u8 /* void* */, imageId: c_int /* int */) -> bool /* bool */;
    fn SetItemData(_obj: *u8 /* void* */, item: c_int /* int */, data: c_int /* int */) -> bool /* bool */;
    fn SetItemFromInfo(_obj: *u8 /* void* */, info: *u8 /* void* */) -> bool /* bool */;
    fn SetItemImage(_obj: *u8 /* void* */, item: c_int /* int */, image: c_int /* int */, selImage: c_int /* int */) -> bool /* bool */;
    fn SetItemPosition(_obj: *u8 /* void* */, item: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    fn SetItemState(_obj: *u8 /* void* */, item: c_int /* int */, state: c_int /* int */, stateMask: c_int /* int */) -> bool /* bool */;
    fn SetItemText(_obj: *u8 /* void* */, item: c_int /* int */, str: *u8 /* void* */);
    fn SetSingleStyle(_obj: *u8 /* void* */, style: c_int /* int */, add: bool /* bool */);
    fn SetTextColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_int /* int */);
    fn SortItems(_obj: *u8 /* void* */, fn_: *u8 /* void* */, eif_obj: *u8 /* void* */) -> bool /* bool */;
    fn UpdateStyle(_obj: *u8 /* void* */);
}
trait wxListEvent {
    fn Cancelled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetCode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetData(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetImage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetIndex(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMask(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxListItem {
    fn Clear(_obj: *u8 /* void* */);
    fn ClearAttributes(_obj: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetAlign(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetAttributes(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetColumn(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetData(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetImage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMask(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetState(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTextColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HasAttributes(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetAlign(_obj: *u8 /* void* */, align: c_int /* int */);
    fn SetBackgroundColour(_obj: *u8 /* void* */, colBack: *u8 /* void* */);
    fn SetColumn(_obj: *u8 /* void* */, col: c_int /* int */);
    fn SetData(_obj: *u8 /* void* */, data: c_int /* int */);
    fn SetDataPointer(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn SetId(_obj: *u8 /* void* */, id: c_int /* int */);
    fn SetImage(_obj: *u8 /* void* */, image: c_int /* int */);
    fn SetMask(_obj: *u8 /* void* */, mask: c_int /* int */);
    fn SetState(_obj: *u8 /* void* */, state: c_int /* int */);
    fn SetStateMask(_obj: *u8 /* void* */, stateMask: c_int /* int */);
    fn SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    fn SetTextColour(_obj: *u8 /* void* */, colText: *u8 /* void* */);
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */);
}
trait wxLocale {
    fn AddCatalog(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> c_int /* int */;
    fn AddCatalogLookupPathPrefix(_obj: *u8 /* void* */, prefix: *u8 /* void* */);
    fn Create(_name: c_int /* int */, _flags: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetLocale(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetString(_obj: *u8 /* void* */, szOrigString: *u8 /* void* */, szDomain: *u8 /* void* */) -> *wchar_t /* wchar_t* */;
    fn IsLoaded(_obj: *u8 /* void* */, szDomain: *u8 /* void* */) -> bool /* bool */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxLog {
}
trait wxLogChain {
    fn Create(logger: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetOldLog(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsPassingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    fn PassMessages(_obj: *u8 /* void* */, bDoPass: bool /* bool */);
    fn SetLog(_obj: *u8 /* void* */, logger: *u8 /* void* */);
}
trait wxLogGUI {
}
trait wxLogNull {
}
trait wxLogPassThrough {
}
trait wxLogStderr {
}
trait wxLogStream {
}
trait wxLogTextCtrl {
}
trait wxLogWindow {
}
trait wxLongLong {
}
trait wxMBConv {
}
trait wxMBConvFile {
}
trait wxMBConvUTF7 {
}
trait wxMBConvUTF8 {
}
trait wxMDIChildFrame {
    fn Activate(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait wxMDIClientWindow {
}
trait wxMDIParentFrame {
    fn ActivateNext(_obj: *u8 /* void* */);
    fn ActivatePrevious(_obj: *u8 /* void* */);
    fn ArrangeIcons(_obj: *u8 /* void* */);
    fn Cascade(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetActiveChild(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClientWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetWindowMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn OnCreateClient(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetWindowMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    fn Tile(_obj: *u8 /* void* */);
}
trait wxMask {
    fn Create(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateColoured(bitmap: *u8 /* void* */, colour: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxMaximizeEvent {
}
trait wxMemoryDC {
    fn Create() -> *u8 /* void* */;
    fn CreateCompatible(dc: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateWithBitmap(bitmap: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn SelectObject(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
}
trait wxMemoryFSHandler {
}
trait wxMemoryInputStream {
}
trait wxMemoryOutputStream {
}
trait wxMenu {
    fn Append(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */);
    fn AppendItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    fn AppendSeparator(_obj: *u8 /* void* */);
    fn AppendSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */);
    fn Break(_obj: *u8 /* void* */);
    fn Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */);
    fn Create(title: *u8 /* void* */, style: c_long /* long */) -> *u8 /* void* */;
    fn DeleteById(_obj: *u8 /* void* */, id: c_int /* int */);
    fn DeleteByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    fn DeletePointer(_obj: *u8 /* void* */);
    fn DestroyById(_obj: *u8 /* void* */, id: c_int /* int */);
    fn DestroyByItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    fn Enable(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */);
    fn FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn FindItemByLabel(_obj: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */;
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetInvokingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetMenuItemCount(_obj: *u8 /* void* */) -> size_t /* size_t */;
    fn GetMenuItems(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    fn GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Insert(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */);
    fn InsertItem(_obj: *u8 /* void* */, pos: size_t /* size_t */, _itm: *u8 /* void* */);
    fn InsertSub(_obj: *u8 /* void* */, pos: size_t /* size_t */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */);
    fn IsAttached(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn Prepend(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */, isCheckable: bool /* bool */);
    fn PrependItem(_obj: *u8 /* void* */, _itm: *u8 /* void* */);
    fn PrependSub(_obj: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, submenu: *u8 /* void* */, help: *u8 /* void* */);
    fn RemoveById(_obj: *u8 /* void* */, id: c_int /* int */, _itm: *u8 /* void* */);
    fn RemoveByItem(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn SetClientData(_obj: *u8 /* void* */, clientData: *u8 /* void* */);
    fn SetEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    fn SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */);
    fn SetInvokingWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    fn SetLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */);
    fn SetParent(_obj: *u8 /* void* */, parent: *u8 /* void* */);
    fn SetTitle(_obj: *u8 /* void* */, title: *u8 /* void* */);
    fn UpdateUI(_obj: *u8 /* void* */, source: *u8 /* void* */);
}
trait wxMenuBar {
    fn Append(_obj: *u8 /* void* */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */;
    fn Check(_obj: *u8 /* void* */, id: c_int /* int */, check: bool /* bool */);
    fn Create(_style: c_int /* int */) -> *u8 /* void* */;
    fn DeletePointer(_obj: *u8 /* void* */);
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> c_int /* int */;
    fn EnableItem(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */);
    fn EnableTop(_obj: *u8 /* void* */, pos: c_int /* int */, enable: bool /* bool */);
    fn FindItem(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn FindMenu(_obj: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */;
    fn FindMenuItem(_obj: *u8 /* void* */, menuString: *u8 /* void* */, itemString: *u8 /* void* */) -> c_int /* int */;
    fn GetHelpString(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetLabel(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */;
    fn GetMenu(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */;
    fn GetMenuCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn Insert(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> c_int /* int */;
    fn IsChecked(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn IsEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn Remove(_obj: *u8 /* void* */, pos: c_int /* int */) -> *u8 /* void* */;
    fn Replace(_obj: *u8 /* void* */, pos: c_int /* int */, menu: *u8 /* void* */, title: *u8 /* void* */) -> *u8 /* void* */;
    fn SetHelpString(_obj: *u8 /* void* */, id: c_int /* int */, helpString: *u8 /* void* */);
    fn SetItemLabel(_obj: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */);
    fn SetLabel(_obj: *u8 /* void* */, s: *u8 /* void* */);
    fn SetLabelTop(_obj: *u8 /* void* */, pos: c_int /* int */, label: *u8 /* void* */);
}
trait wxMenuEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetMenuId(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxMenuItem {
    fn Check(_obj: *u8 /* void* */, check: bool /* bool */);
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn GetHelp(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetLabelFromText(text: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    fn GetMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSubMenu(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsCheckable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsSeparator(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsSubMenu(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetCheckable(_obj: *u8 /* void* */, checkable: bool /* bool */);
    fn SetHelp(_obj: *u8 /* void* */, str: *u8 /* void* */);
    fn SetId(_obj: *u8 /* void* */, id: c_int /* int */);
    fn SetSubMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */);
    fn SetText(_obj: *u8 /* void* */, str: *u8 /* void* */);
}
trait wxMessageDialog {
    fn Create(_prt: *u8 /* void* */, _msg: *u8 /* void* */, _cap: *u8 /* void* */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn ShowModal(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxMetafile {
    fn Create(_file: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Play(_obj: *u8 /* void* */, _dc: *u8 /* void* */) -> bool /* bool */;
    fn SetClipboard(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
}
trait wxMetafileDC {
    fn Close(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(_file: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxMimeTypesManager {
    fn AddFallbacks(_obj: *u8 /* void* */, _types: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn EnumAllFileTypes(_obj: *u8 /* void* */, _lst: *u8 /* void* */) -> c_int /* int */;
    fn GetFileTypeFromExtension(_obj: *u8 /* void* */, _ext: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFileTypeFromMimeType(_obj: *u8 /* void* */, _name: *u8 /* void* */) -> *u8 /* void* */;
    fn IsOfType(_obj: *u8 /* void* */, _type: *u8 /* void* */, _wildcard: *u8 /* void* */) -> bool /* bool */;
}
trait wxMiniFrame {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait wxMirrorDC {
    fn Create(dc: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxModule {
}
trait wxMouseCaptureChangedEvent {
}
trait wxMouseEvent {
    fn AltDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Button(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ButtonDClick(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ButtonDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ButtonIsDown(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ButtonUp(_obj: *u8 /* void* */, but: c_int /* int */) -> bool /* bool */;
    fn ControlDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    fn Dragging(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Entering(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetLogicalPosition(_obj: *u8 /* void* */, dc: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsButton(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Leaving(_obj: *u8 /* void* */) -> bool /* bool */;
    fn LeftDClick(_obj: *u8 /* void* */) -> bool /* bool */;
    fn LeftDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn LeftIsDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn LeftUp(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MetaDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MiddleDClick(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MiddleDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MiddleIsDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn MiddleUp(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Moving(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RightDClick(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RightDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RightIsDown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RightUp(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ShiftDown(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxMoveEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxMultiCellCanvas {
    // missing: wxMultiCellCanvas_Add
    // missing: wxMultiCellCanvas_CalculateConstraints
    // missing: wxMultiCellCanvas_Create
    // missing: wxMultiCellCanvas_MaxCols
    // missing: wxMultiCellCanvas_MaxRows
    // missing: wxMultiCellCanvas_SetMinCellSize
}
trait wxMultiCellItemHandle {
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
}
trait wxMultiCellSizer {
    // missing: wxMultiCellSizer_CalcMin
    // missing: wxMultiCellSizer_Create
    // missing: wxMultiCellSizer_Delete
    // missing: wxMultiCellSizer_EnableGridLines
    // missing: wxMultiCellSizer_RecalcSizes
    // missing: wxMultiCellSizer_SetColumnWidth
    // missing: wxMultiCellSizer_SetDefaultCellSize
    // missing: wxMultiCellSizer_SetGridPen
    // missing: wxMultiCellSizer_SetRowHeight
}
trait wxMutex {
    // missing: wxMutex_Create
    // missing: wxMutex_Delete
    // missing: wxMutex_IsLocked
    // missing: wxMutex_Lock
    // missing: wxMutex_TryLock
    // missing: wxMutex_Unlock
}
trait wxMutexLocker {
}
trait wxNavigationKeyEvent {
    fn GetCurrentFocus(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDirection(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsWindowChange(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetCurrentFocus(_obj: *u8 /* void* */, win: *u8 /* void* */);
    fn SetDirection(_obj: *u8 /* void* */, bForward: bool /* bool */);
    fn SetWindowChange(_obj: *u8 /* void* */, bIs: bool /* bool */);
    fn ShouldPropagate(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxNewBitmapButton {
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
}
trait wxNodeBase {
}
trait wxNotebook {
    fn AddPage(_obj: *u8 /* void* */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */;
    fn AdvanceSelection(_obj: *u8 /* void* */, bForward: bool /* bool */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn DeleteAllPages(_obj: *u8 /* void* */) -> bool /* bool */;
    fn DeletePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */;
    fn GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */;
    fn GetPageCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */;
    fn GetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */) -> *u8 /* void* */;
    fn GetRowCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_long /* long* */) -> c_int /* int */;
    fn InsertPage(_obj: *u8 /* void* */, nPage: c_int /* int */, pPage: *u8 /* void* */, strText: *u8 /* void* */, bSelect: bool /* bool */, imageId: c_int /* int */) -> bool /* bool */;
    fn RemovePage(_obj: *u8 /* void* */, nPage: c_int /* int */) -> bool /* bool */;
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    fn SetPadding(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetPageImage(_obj: *u8 /* void* */, nPage: c_int /* int */, nImage: c_int /* int */) -> bool /* bool */;
    fn SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetPageText(_obj: *u8 /* void* */, nPage: c_int /* int */, strText: *u8 /* void* */) -> bool /* bool */;
    fn SetSelection(_obj: *u8 /* void* */, nPage: c_int /* int */) -> c_int /* int */;
    fn expNB_TOP() -> c_int /* int */;
    fn expNB_BOTTOM() -> c_int /* int */;
    fn expNB_LEFT() -> c_int /* int */;
    fn expNB_RIGHT() -> c_int /* int */;
    fn expBK_HITTEST_NOWHERE() -> c_int /* int */;
    fn expBK_HITTEST_ONICON() -> c_int /* int */;
    fn expBK_HITTEST_ONLABEL() -> c_int /* int */;
    fn expBK_HITTEST_ONITEM() -> c_int /* int */;
    fn expBK_HITTEST_ONPAGE() -> c_int /* int */;
}
trait wxNotebookEvent {
}
trait wxNotifyEvent {
    fn Allow(_obj: *u8 /* void* */);
    fn CopyObject(_obj: *u8 /* void* */, object_dest: *u8 /* void* */);
    fn IsAllowed(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Veto(_obj: *u8 /* void* */);
}
trait wxObject {
}
trait wxObjectRefData {
}
trait wxOutputStream {
    fn Delete(_obj: *u8 /* void* */);
    fn LastWrite(_obj: *u8 /* void* */) -> c_int /* int */;
    fn PutC(_obj: *u8 /* void* */, c: wchar_t /* wchar_t */);
    fn Seek(_obj: *u8 /* void* */, pos: c_int /* int */, mode: c_int /* int */) -> c_int /* int */;
    fn Sync(_obj: *u8 /* void* */);
    fn Tell(_obj: *u8 /* void* */) -> c_int /* int */;
    fn Write(_obj: *u8 /* void* */, buffer: *u8 /* void* */, size: c_int /* int */);
}
trait wxPageSetupDialog {
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPageSetupData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
}
trait wxPageSetupDialogData {
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn AssignData(_obj: *u8 /* void* */, printData: *u8 /* void* */);
    fn CalculateIdFromPaperSize(_obj: *u8 /* void* */);
    fn CalculatePaperSizeFromId(_obj: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnableMargins(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnableOrientation(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnablePaper(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnablePrinter(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn GetDefaultInfo(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetDefaultMinMargins(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnableMargins(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnableOrientation(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnablePaper(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnablePrinter(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMinMarginBottomRight(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMinMarginTopLeft(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn SetDefaultInfo(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetDefaultMinMargins(_obj: *u8 /* void* */, flag: c_int /* int */);
    fn SetMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetMinMarginBottomRight(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetMinMarginTopLeft(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetPaperId(_obj: *u8 /* void* */, id: *u8 /* void* */);
    fn SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetPaperSizeId(_obj: *u8 /* void* */, id: c_int /* int */);
    fn SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */);
}
trait wxPaintDC {
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxPaintEvent {
}
trait wxPalette {
    fn Assign(_obj: *u8 /* void* */, palette: *u8 /* void* */);
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateRGB(n: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetPixel(_obj: *u8 /* void* */, arg0: u8 /* u8 */, arg1: u8 /* u8 */, arg2: u8 /* u8 */) -> c_int /* int */;
    fn GetRGB(_obj: *u8 /* void* */, pixel: c_int /* int */, red: *u8 /* void* */, green: *u8 /* void* */, blue: *u8 /* void* */) -> bool /* bool */;
    fn IsEqual(_obj: *u8 /* void* */, palette: *u8 /* void* */) -> bool /* bool */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxPaletteChangedEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetChangedWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetChangedWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
}
trait wxPanel {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn InitDialog(_obj: *u8 /* void* */);
    fn SetFocus(_obj: *u8 /* void* */);
}
trait wxPathList {
}
trait wxPen {
    fn Assign(_obj: *u8 /* void* */, pen: *u8 /* void* */);
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateFromBitmap(stipple: *u8 /* void* */, width: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromColour(col: *u8 /* void* */, width: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromStock(id: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetCap(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetDashes(_obj: *u8 /* void* */, ptr: *u8 /* void* */) -> c_int /* int */;
    fn GetJoin(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetStipple(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsEqual(_obj: *u8 /* void* */, pen: *u8 /* void* */) -> bool /* bool */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetCap(_obj: *u8 /* void* */, cap: c_int /* int */);
    fn SetColour(_obj: *u8 /* void* */, col: *u8 /* void* */);
    fn SetColourSingle(_obj: *u8 /* void* */, r: wchar_t /* wchar_t */, g: wchar_t /* wchar_t */, b: wchar_t /* wchar_t */);
    fn SetDashes(_obj: *u8 /* void* */, nb_dashes: c_int /* int */, dash: *u8 /* void* */);
    fn SetJoin(_obj: *u8 /* void* */, join: c_int /* int */);
    fn SetStipple(_obj: *u8 /* void* */, stipple: *u8 /* void* */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
    fn SetWidth(_obj: *u8 /* void* */, width: c_int /* int */);
}
trait wxPenList {
}
trait wxPlotCurve {
}
trait wxPlotEvent {
    // missing: wxPlotEvent_GetCurve
    // missing: wxPlotEvent_GetPosition
    // missing: wxPlotEvent_GetZoom
    // missing: wxPlotEvent_SetPosition
    // missing: wxPlotEvent_SetZoom
}
trait wxPlotOnOffCurve {
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
}
trait wxPlotWindow {
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
}
trait wxPoint {
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    // missing: wxPoint_Destroy
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetX(_obj: *u8 /* void* */, w: c_int /* int */);
    fn SetY(_obj: *u8 /* void* */, h: c_int /* int */);
}
trait wxPopupTransientWindow {
}
trait wxPopupWindow {
}
trait wxPostScriptDC {
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(self_: *u8 /* void* */);
    fn SetResolution(self_: *u8 /* void* */, ppi: c_int /* int */);
    fn GetResolution(self_: *u8 /* void* */) -> c_int /* int */;
}
trait wxPreviewCanvas {
    fn Create(preview: *u8 /* void* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
}
trait wxPreviewControlBar {
}
trait wxPreviewFrame {
}
trait wxPrintData {
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetCollate(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetDuplex(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFilename(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFontMetricPath(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPaperId(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPaperSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPreviewCommand(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrintMode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPrinterCommand(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrinterName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrinterOptions(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrinterScaleX(_obj: *u8 /* void* */) -> c_double /* double */;
    fn GetPrinterScaleY(_obj: *u8 /* void* */) -> c_double /* double */;
    fn GetPrinterTranslateX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPrinterTranslateY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetQuality(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetCollate(_obj: *u8 /* void* */, flag: c_int /* int */);
    fn SetColour(_obj: *u8 /* void* */, colour: c_int /* int */);
    fn SetDuplex(_obj: *u8 /* void* */, duplex: c_int /* int */);
    fn SetFilename(_obj: *u8 /* void* */, filename: *u8 /* void* */);
    fn SetFontMetricPath(_obj: *u8 /* void* */, path: *u8 /* void* */);
    fn SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */);
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    fn SetPaperId(_obj: *u8 /* void* */, sizeId: c_int /* int */);
    fn SetPaperSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetPreviewCommand(_obj: *u8 /* void* */, command: *u8 /* void* */);
    fn SetPrintMode(_obj: *u8 /* void* */, printMode: c_int /* int */);
    fn SetPrinterCommand(_obj: *u8 /* void* */, command: *u8 /* void* */);
    fn SetPrinterName(_obj: *u8 /* void* */, name: *u8 /* void* */);
    fn SetPrinterOptions(_obj: *u8 /* void* */, options: *u8 /* void* */);
    fn SetPrinterScaleX(_obj: *u8 /* void* */, x: c_double /* double */);
    fn SetPrinterScaleY(_obj: *u8 /* void* */, y: c_double /* double */);
    fn SetPrinterScaling(_obj: *u8 /* void* */, x: c_double /* double */, y: c_double /* double */);
    fn SetPrinterTranslateX(_obj: *u8 /* void* */, x: c_int /* int */);
    fn SetPrinterTranslateY(_obj: *u8 /* void* */, y: c_int /* int */);
    fn SetPrinterTranslation(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetQuality(_obj: *u8 /* void* */, quality: c_int /* int */);
}
trait wxPostScriptPrintNativeData {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxPrintDialog {
    fn Create(parent: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrintDC(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetPrintDialogData(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxPrintDialogData {
    fn Assign(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn AssignData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateFromData(printData: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn EnableHelp(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnablePageNumbers(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnablePrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn EnableSelection(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn GetAllPages(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetCollate(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnableHelp(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnablePageNumbers(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnablePrintToFile(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnableSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetFromPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetNoCopies(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPrintData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetPrintToFile(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetSelection(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetToPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetAllPages(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetCollate(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetFromPage(_obj: *u8 /* void* */, v: c_int /* int */);
    fn SetMaxPage(_obj: *u8 /* void* */, v: c_int /* int */);
    fn SetMinPage(_obj: *u8 /* void* */, v: c_int /* int */);
    fn SetNoCopies(_obj: *u8 /* void* */, v: c_int /* int */);
    fn SetPrintData(_obj: *u8 /* void* */, printData: *u8 /* void* */);
    fn SetPrintToFile(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetSelection(_obj: *u8 /* void* */, flag: bool /* bool */);
    fn SetToPage(_obj: *u8 /* void* */, v: c_int /* int */);
}
trait wxPrintPreview {
    fn CreateFromData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateFromDialogData(printout: *u8 /* void* */, printoutForPrinting: *u8 /* void* */, data: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn DetermineScaling(_obj: *u8 /* void* */);
    fn DrawBlankPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */;
    fn GetCanvas(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCurrentPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMaxPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinPage(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetPrintout(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrintoutForPrinting(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetZoom(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn PaintPage(_obj: *u8 /* void* */, canvas: *u8 /* void* */, dc: *u8 /* void* */) -> bool /* bool */;
    fn Print(_obj: *u8 /* void* */, interactive: bool /* bool */) -> bool /* bool */;
    fn RenderPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */;
    fn SetCanvas(_obj: *u8 /* void* */, canvas: *u8 /* void* */);
    fn SetCurrentPage(_obj: *u8 /* void* */, pageNum: c_int /* int */) -> bool /* bool */;
    fn SetFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */);
    fn SetOk(_obj: *u8 /* void* */, ok: bool /* bool */);
    fn SetPrintout(_obj: *u8 /* void* */, printout: *u8 /* void* */);
    fn SetZoom(_obj: *u8 /* void* */, percent: c_int /* int */);
}
trait wxPrinter {
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateAbortWindow(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetAbort(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetLastError(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPrintDialogData(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn Print(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, prompt: bool /* bool */) -> bool /* bool */;
    fn PrintDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> *u8 /* void* */;
    fn ReportError(_obj: *u8 /* void* */, parent: *u8 /* void* */, printout: *u8 /* void* */, message: *u8 /* void* */);
    fn Setup(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */;
}
trait wxPrinterDC {
    fn Create(data: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(self_: *u8 /* void* */);
    fn GetPaperRect(self_: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxPrintout {
}
trait wxPrivateDropTarget {
}
trait wxProcess {
    fn CloseOutput(_obj: *u8 /* void* */);
    fn CreateDefault(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */;
    fn CreateRedirect(_prt: *u8 /* void* */, _rdr: bool /* bool */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Detach(_obj: *u8 /* void* */);
    fn GetErrorStream(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetInputStream(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetOutputStream(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsRedirected(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Redirect(_obj: *u8 /* void* */);
}
trait wxProcessEvent {
    fn GetExitCode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPid(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxProgressDialog {
}
trait wxProtocol {
}
trait wxQuantize {
}
trait wxQueryCol {
}
trait wxQueryField {
}
trait wxQueryLayoutInfoEvent {
    fn Create(id: c_int /* int */) -> *u8 /* void* */;
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRequestedLength(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */);
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    fn SetRequestedLength(_obj: *u8 /* void* */, length: c_int /* int */);
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
}
trait wxQueryNewPaletteEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetPaletteRealized(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetPaletteRealized(_obj: *u8 /* void* */, realized: bool /* bool */);
}
trait wxRadioBox {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: *wchar_t /* wchar_t* */, _dim: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn EnableItem(_obj: *u8 /* void* */, item: c_int /* int */, enable: bool /* bool */);
    fn FindString(_obj: *u8 /* void* */, s: *u8 /* void* */) -> c_int /* int */;
    fn GetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    fn GetNumberOfRowsOrCols(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelection(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Number(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetItemBitmap(_obj: *u8 /* void* */, item: c_int /* int */, bitmap: *u8 /* void* */);
    fn SetItemLabel(_obj: *u8 /* void* */, item: c_int /* int */, label: *u8 /* void* */);
    fn SetNumberOfRowsOrCols(_obj: *u8 /* void* */, n: c_int /* int */);
    fn SetSelection(_obj: *u8 /* void* */, _n: c_int /* int */);
    fn SetStringSelection(_obj: *u8 /* void* */, s: *u8 /* void* */);
    fn ShowItem(_obj: *u8 /* void* */, item: c_int /* int */, show: bool /* bool */);
}
trait wxRadioButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetValue(_obj: *u8 /* void* */, value: bool /* bool */);
}
trait wxRealPoint {
}
trait wxRecordSet {
}
trait wxRect {
}
trait wxRegEx {
}
trait wxRegion {
    fn Assign(_obj: *u8 /* void* */, region: *u8 /* void* */);
    fn Clear(_obj: *u8 /* void* */);
    fn ContainsPoint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> bool /* bool */;
    fn ContainsRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn CreateDefault() -> *u8 /* void* */;
    fn CreateFromRect(arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn IsEmpty(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetBox(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    fn IntersectRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn IntersectRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    fn SubtractRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn SubtractRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    fn UnionRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn UnionRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    fn XorRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn XorRegion(_obj: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
}
trait wxRegionIterator {
    fn Create() -> *u8 /* void* */;
    fn CreateFromRegion(region: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HaveRects(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Next(_obj: *u8 /* void* */);
    fn Reset(_obj: *u8 /* void* */);
    fn ResetToRegion(_obj: *u8 /* void* */, region: *u8 /* void* */);
}
trait wxRemotelyScrolledTreeCtrl {
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
}
trait wxSVGFileDC {
    fn Create(fileName: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateWithSize(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn CreateWithSizeAndResolution(fileName: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, a_dpi: float /* float */) -> *u8 /* void* */;
    fn Delete(obj: *u8 /* void* */);
}
trait wxSashEvent {
    fn Create(id: c_int /* int */, edge: c_int /* int */) -> *u8 /* void* */;
    fn GetDragRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDragStatus(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetEdge(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetDragRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn SetDragStatus(_obj: *u8 /* void* */, status: c_int /* int */);
    fn SetEdge(_obj: *u8 /* void* */, edge: c_int /* int */);
}
trait wxSashLayoutWindow {
    fn Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetAlignment(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetAlignment(_obj: *u8 /* void* */, align: c_int /* int */);
    fn SetDefaultSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
}
trait wxSashWindow {
    fn Create(_par: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetDefaultBorderSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetEdgeMargin(_obj: *u8 /* void* */, edge: c_int /* int */) -> c_int /* int */;
    fn GetExtraBorderSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMaximumSizeX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMaximumSizeY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinimumSizeX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinimumSizeY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */;
    fn HasBorder(_obj: *u8 /* void* */, edge: c_int /* int */) -> bool /* bool */;
    fn SetDefaultBorderSize(_obj: *u8 /* void* */, width: c_int /* int */);
    fn SetExtraBorderSize(_obj: *u8 /* void* */, width: c_int /* int */);
    fn SetMaximumSizeX(_obj: *u8 /* void* */, max: c_int /* int */);
    fn SetMaximumSizeY(_obj: *u8 /* void* */, max: c_int /* int */);
    fn SetMinimumSizeX(_obj: *u8 /* void* */, min: c_int /* int */);
    fn SetMinimumSizeY(_obj: *u8 /* void* */, min: c_int /* int */);
    fn SetSashBorder(_obj: *u8 /* void* */, edge: c_int /* int */, border: bool /* bool */);
    fn SetSashVisible(_obj: *u8 /* void* */, edge: c_int /* int */, sash: bool /* bool */);
}
trait wxScopedArray {
}
trait wxScopedPtr {
}
trait wxScreenDC {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn EndDrawingOnTop(_obj: *u8 /* void* */) -> bool /* bool */;
    fn StartDrawingOnTop(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn StartDrawingOnTopOfWin(_obj: *u8 /* void* */, win: *u8 /* void* */) -> bool /* bool */;
}
trait wxScrollBar {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRange(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetThumbPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetThumbSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetScrollbar(_obj: *u8 /* void* */, position: c_int /* int */, thumbSize: c_int /* int */, range: c_int /* int */, pageSize: c_int /* int */, refresh: bool /* bool */);
    fn SetThumbPosition(_obj: *u8 /* void* */, viewStart: c_int /* int */);
}
trait wxScrollEvent {
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxScrollWinEvent {
    fn GetOrientation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetOrientation(_obj: *u8 /* void* */, orient: c_int /* int */);
    fn SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
}
trait wxScrolledWindow {
    fn AdjustScrollbars(_obj: *u8 /* void* */);
    fn CalcScrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    fn CalcUnscrolledPosition(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: *c_int /* int* */, arg3: *c_int /* int* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn EnableScrolling(_obj: *u8 /* void* */, x_scrolling: bool /* bool */, y_scrolling: bool /* bool */);
    fn GetScaleX(_obj: *u8 /* void* */) -> c_double /* double */;
    fn GetScaleY(_obj: *u8 /* void* */) -> c_double /* double */;
    fn GetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    fn GetScrollPixelsPerUnit(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetTargetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetVirtualSize(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn OnDraw(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    fn Scroll(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetScale(_obj: *u8 /* void* */, xs: c_double /* double */, ys: c_double /* double */);
    fn SetScrollPageSize(_obj: *u8 /* void* */, orient: c_int /* int */, pageSize: c_int /* int */);
    fn SetScrollbars(_obj: *u8 /* void* */, pixelsPerUnitX: c_int /* int */, pixelsPerUnitY: c_int /* int */, noUnitsX: c_int /* int */, noUnitsY: c_int /* int */, xPos: c_int /* int */, yPos: c_int /* int */, noRefresh: bool /* bool */);
    fn ShowScrollbars(_obj: *u8 /* void* */, showh: c_int /* int */, showv: c_int /* int */);
    fn SetTargetWindow(_obj: *u8 /* void* */, target: *u8 /* void* */);
    fn ViewStart(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
}
trait wxSemaphore {
}
trait wxServer {
}
trait wxServerBase {
}
trait wxSetCursorEvent {
    fn GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HasCursor(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */);
}
trait wxShowEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn IsShown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetShow(_obj: *u8 /* void* */, show: bool /* bool */);
}
trait wxSimpleHelpProvider {
    fn Create() -> *u8 /* void* */;
}
trait wxSingleChoiceDialog {
}
trait wxSingleInstanceChecker {
    fn Create(_obj: *u8 /* void* */, name: *u8 /* void* */, path: *u8 /* void* */) -> bool /* bool */;
    fn CreateDefault() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn IsAnotherRunning(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxSize {
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    // missing: wxSize_Destroy
    fn GetHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetHeight(_obj: *u8 /* void* */, h: c_int /* int */);
    fn SetWidth(_obj: *u8 /* void* */, w: c_int /* int */);
}
trait wxSizeEvent {
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxSizer {
    fn Add(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn AddSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn AddWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Fit(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */;
    fn GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Insert(_obj: *u8 /* void* */, before: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn InsertSizer(_obj: *u8 /* void* */, before: c_int /* int */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn InsertWindow(_obj: *u8 /* void* */, before: c_int /* int */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn Layout(_obj: *u8 /* void* */);
    fn Prepend(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn PrependSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn PrependWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */);
    fn RecalcSizes(_obj: *u8 /* void* */);
    fn SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn SetItemMinSize(_obj: *u8 /* void* */, pos: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetItemMinSizeSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetItemMinSizeWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn AddSpacer(_obj: *u8 /* void* */, size: c_int /* int */);
    fn AddStretchSpacer(_obj: *u8 /* void* */, size: c_int /* int */);
    fn Clear(_obj: *u8 /* void* */, delete_windows: bool /* bool */);
    fn DetachWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    fn DetachSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */;
    fn Detach(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    fn FitInside(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn GetContainingWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetItemWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */;
    fn GetItemSizer(_obj: *u8 /* void* */, window: *u8 /* void* */, recursive: bool /* bool */) -> *u8 /* void* */;
    fn GetItem(_obj: *u8 /* void* */, index: c_int /* int */) -> *u8 /* void* */;
    fn HideWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    fn HideSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */;
    fn Hide(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    fn InsertSpacer(_obj: *u8 /* void* */, index: c_int /* int */, size: c_int /* int */) -> *u8 /* void* */;
    fn InsertStretchSpacer(_obj: *u8 /* void* */, index: c_int /* int */, prop: c_int /* int */) -> *u8 /* void* */;
    fn IsShownWindow(_obj: *u8 /* void* */, window: *u8 /* void* */) -> bool /* bool */;
    fn IsShownSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */) -> bool /* bool */;
    fn IsShown(_obj: *u8 /* void* */, index: c_int /* int */) -> bool /* bool */;
    fn PrependSpacer(_obj: *u8 /* void* */, size: c_int /* int */) -> *u8 /* void* */;
    fn PrependStretchSpacer(_obj: *u8 /* void* */, prop: c_int /* int */) -> *u8 /* void* */;
    fn ReplaceWindow(_obj: *u8 /* void* */, oldwin: *u8 /* void* */, newwin: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */;
    fn ReplaceSizer(_obj: *u8 /* void* */, oldsz: *u8 /* void* */, newsz: *u8 /* void* */, recursive: bool /* bool */) -> bool /* bool */;
    fn Replace(_obj: *u8 /* void* */, oldindex: c_int /* int */, newitem: *u8 /* void* */) -> bool /* bool */;
    fn SetVirtualSizeHints(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn ShowWindow(_obj: *u8 /* void* */, window: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */;
    fn ShowSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */, show: bool /* bool */, recursive: bool /* bool */) -> bool /* bool */;
    fn Show(_obj: *u8 /* void* */, sizer: *u8 /* void* */, index: c_int /* int */, show: bool /* bool */) -> bool /* bool */;
}
trait wxSizerItem {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(arg0: c_int /* int */, arg1: c_int /* int */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateInSizer(sizer: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */;
    fn CreateInWindow(window: *u8 /* void* */, option: c_int /* int */, flag: c_int /* int */, border: c_int /* int */, userData: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBorder(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFlag(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetRatio(_obj: *u8 /* void* */) -> float /* float */;
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetUserData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsSizer(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsSpacer(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetBorder(_obj: *u8 /* void* */, border: c_int /* int */);
    fn SetDimension(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn SetFlag(_obj: *u8 /* void* */, flag: c_int /* int */);
    fn SetFloatRatio(_obj: *u8 /* void* */, ratio: float /* float */);
    fn SetInitSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetRatio(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */);
    fn SetWindow(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn Delete(_obj: *u8 /* void* */);
    fn DeleteWindows(_obj: *u8 /* void* */);
    fn DetachSizer(_obj: *u8 /* void* */);
    fn GetProportion(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSpacer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsShown(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetProportion(_obj: *u8 /* void* */, proportion: c_int /* int */);
    fn SetSpacer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn Show(_obj: *u8 /* void* */, show: c_int /* int */);
}
trait wxSlider {
    fn ClearSel(_obj: *u8 /* void* */);
    fn ClearTicks(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _init: c_int /* int */, _min: c_int /* int */, _max: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    fn GetLineSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetPageSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelEnd(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelStart(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetThumbLength(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetTickFreq(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetLineSize(_obj: *u8 /* void* */, lineSize: c_int /* int */);
    fn SetPageSize(_obj: *u8 /* void* */, pageSize: c_int /* int */);
    fn SetRange(_obj: *u8 /* void* */, minValue: c_int /* int */, maxValue: c_int /* int */);
    fn SetSelection(_obj: *u8 /* void* */, minPos: c_int /* int */, maxPos: c_int /* int */);
    fn SetThumbLength(_obj: *u8 /* void* */, len: c_int /* int */);
    fn SetTick(_obj: *u8 /* void* */, tickPos: c_int /* int */);
    fn SetTickFreq(_obj: *u8 /* void* */, n: c_int /* int */, pos: c_int /* int */);
    fn SetValue(_obj: *u8 /* void* */, value: c_int /* int */);
}
trait wxSockAddress {
}
trait wxSocketBase {
}
trait wxSocketClient {
}
trait wxSocketEvent {
}
trait wxSocketInputStream {
}
trait wxSocketOutputStream {
}
trait wxSocketServer {
}
trait wxSpinButton {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetRange(_obj: *u8 /* void* */, minVal: c_int /* int */, maxVal: c_int /* int */);
    fn SetValue(_obj: *u8 /* void* */, val: c_int /* int */);
}
trait wxSpinCtrl {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */, _min: c_int /* int */, _max: c_int /* int */, _init: c_int /* int */) -> *u8 /* void* */;
    fn GetMax(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMin(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetValue(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetRange(_obj: *u8 /* void* */, min_val: c_int /* int */, max_val: c_int /* int */);
    fn SetValue(_obj: *u8 /* void* */, val: c_int /* int */);
}
trait wxSpinEvent {
    fn GetPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    fn SetPosition(_obj: *u8 /* void* */, pos: c_int /* int */);
}
trait wxSplashScreen {
}
trait wxSplitterEvent {
}
trait wxSplitterScrolledWindow {
    // missing: wxSplitterScrolledWindow_Create
}
trait wxSplitterWindow {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetBorderSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinimumPaneSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSashPosition(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSashSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSplitMode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetWindow1(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetWindow2(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Initialize(_obj: *u8 /* void* */, window: *u8 /* void* */);
    fn IsSplit(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ReplaceWindow(_obj: *u8 /* void* */, winOld: *u8 /* void* */, winNew: *u8 /* void* */) -> bool /* bool */;
    fn SetBorderSize(_obj: *u8 /* void* */, width: c_int /* int */);
    fn SetMinimumPaneSize(_obj: *u8 /* void* */, min: c_int /* int */);
    fn SetSashPosition(_obj: *u8 /* void* */, position: c_int /* int */, redraw: bool /* bool */);
    fn SetSashSize(_obj: *u8 /* void* */, width: c_int /* int */);
    fn SetSplitMode(_obj: *u8 /* void* */, mode: c_int /* int */);
    fn SplitHorizontally(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */;
    fn SplitVertically(_obj: *u8 /* void* */, window1: *u8 /* void* */, window2: *u8 /* void* */, sashPosition: c_int /* int */) -> bool /* bool */;
    fn Unsplit(_obj: *u8 /* void* */, toRemove: *u8 /* void* */) -> bool /* bool */;
    fn GetSashGravity(_obj: *u8 /* void* */) -> c_double /* double */;
    fn SetSashGravity(_obj: *u8 /* void* */, gravity: c_double /* double */);
}
trait wxStaticBitmap {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, bitmap: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetIcon(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn SetBitmap(_obj: *u8 /* void* */, bitmap: *u8 /* void* */);
    fn SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */);
}
trait wxStaticBox {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait wxStaticBoxSizer {
    fn CalcMin(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(box: *u8 /* void* */, orient: c_int /* int */) -> *u8 /* void* */;
    fn GetStaticBox(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn RecalcSizes(_obj: *u8 /* void* */);
}
trait wxStaticLine {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetDefaultSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsVertical(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxStaticText {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
}
trait wxStatusBar {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn GetBorderX(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetBorderY(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetFieldsCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetStatusText(_obj: *u8 /* void* */, number: c_int /* int */) -> *u8 /* void* */;
    fn SetFieldsCount(_obj: *u8 /* void* */, number: c_int /* int */, widths: *c_int /* int* */);
    fn SetMinHeight(_obj: *u8 /* void* */, height: c_int /* int */);
    fn SetStatusText(_obj: *u8 /* void* */, text: *u8 /* void* */, number: c_int /* int */);
    fn SetStatusWidths(_obj: *u8 /* void* */, n: c_int /* int */, widths: *c_int /* int* */);
}
trait wxStopWatch {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn Start(_obj: *u8 /* void* */, msec: c_int /* int */);
    fn Pause(_obj: *u8 /* void* */);
    fn Resume(_obj: *u8 /* void* */);
    fn Time(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxStreamBase {
    fn GetLastError(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxStreamBuffer {
}
trait wxStreamToTextRedirector {
}
trait wxString {
}
trait wxStringBuffer {
}
trait wxStringClientData {
}
trait wxStringList {
}
trait wxStringTokenizer {
}
trait wxSysColourChangedEvent {
}
trait wxSystemOptions {
}
trait wxSystemSettings {
    fn GetColour(index: c_int /* int */, _ref: *u8 /* void* */);
    fn GetFont(index: c_int /* int */, _ref: *u8 /* void* */);
    fn GetMetric(index: c_int /* int */) -> c_int /* int */;
    fn GetScreenType() -> c_int /* int */;
}
trait wxTabCtrl {
}
trait wxTabEvent {
}
trait wxTablesInUse {
}
trait wxTaskBarIcon {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn IsIconInstalled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
    fn PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */) -> bool /* bool */;
    fn RemoveIcon(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetIcon(_obj: *u8 /* void* */, icon: *u8 /* void* */, text: *u8 /* void* */) -> bool /* bool */;
}
trait wxTempFile {
}
trait wxTextAttr {
}
trait wxTextCtrl {
    fn AppendText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    fn CanCopy(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanCut(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanPaste(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanRedo(_obj: *u8 /* void* */) -> bool /* bool */;
    fn CanUndo(_obj: *u8 /* void* */) -> bool /* bool */;
    fn ChangeValue(_obj: *u8 /* void* */, text: *u8 /* void* */);
    fn Clear(_obj: *u8 /* void* */);
    fn Copy(_obj: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_long /* long */) -> *u8 /* void* */;
    fn Cut(_obj: *u8 /* void* */);
    fn DiscardEdits(_obj: *u8 /* void* */);
    fn GetInsertionPoint(_obj: *u8 /* void* */) -> c_long /* long */;
    fn GetLastPosition(_obj: *u8 /* void* */) -> c_long /* long */;
    fn GetLineLength(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> c_int /* int */;
    fn GetLineText(_obj: *u8 /* void* */, lineNo: c_long /* long */) -> *u8 /* void* */;
    fn GetNumberOfLines(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetSelection(_obj: *u8 /* void* */, from: *u8 /* void* */, to: *u8 /* void* */);
    fn GetValue(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsEditable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsModified(_obj: *u8 /* void* */) -> bool /* bool */;
    fn LoadFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    fn Paste(_obj: *u8 /* void* */);
    fn PositionToXY(_obj: *u8 /* void* */, pos: c_long /* long */, x: *c_long /* long* */, y: *c_long /* long* */) -> c_int /* int */;
    fn Redo(_obj: *u8 /* void* */);
    fn Remove(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */);
    fn Replace(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */, value: *u8 /* void* */);
    fn SaveFile(_obj: *u8 /* void* */, file: *u8 /* void* */) -> bool /* bool */;
    fn SetEditable(_obj: *u8 /* void* */, editable: bool /* bool */);
    fn SetInsertionPoint(_obj: *u8 /* void* */, pos: c_long /* long */);
    fn SetInsertionPointEnd(_obj: *u8 /* void* */);
    fn SetSelection(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */);
    fn SetValue(_obj: *u8 /* void* */, value: *u8 /* void* */);
    fn ShowPosition(_obj: *u8 /* void* */, pos: c_long /* long */);
    fn Undo(_obj: *u8 /* void* */);
    fn WriteText(_obj: *u8 /* void* */, text: *u8 /* void* */);
    fn XYToPosition(_obj: *u8 /* void* */, arg0: c_long /* long */, arg1: c_long /* long */) -> c_long /* long */;
}
trait wxTextDataObject {
    fn TextDataObject_Create(_txt: *u8 /* void* */) -> *u8 /* void* */;
    fn TextDataObject_Delete(_obj: *u8 /* void* */);
    fn TextDataObject_GetTextLength(_obj: *u8 /* void* */) -> size_t /* size_t */;
    fn TextDataObject_GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn TextDataObject_SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
}
trait wxTextDropTarget {
}
trait wxTextEntryDialog {
}
trait wxTextFile {
}
trait wxTextInputStream {
}
trait wxTextOutputStream {
}
trait wxTextValidator {
    fn Create(style: c_int /* int */, val: *u8 /* void* */) -> *u8 /* void* */;
    fn GetExcludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn GetIncludes(_obj: *u8 /* void* */, _ref: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn SetExcludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */);
    fn SetIncludes(_obj: *u8 /* void* */, list: *wchar_t /* wchar_t* */, count: c_int /* int */);
    fn Clone(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetStyle(_obj: *u8 /* void* */) -> c_int /* int */;
    fn OnChar(_obj: *u8 /* void* */, event: *u8 /* void* */);
    fn SetStyle(_obj: *u8 /* void* */, style: c_int /* int */);
}
trait wxThinSplitterWindow {
    // missing: wxThinSplitterWindow_Create
    // missing: wxThinSplitterWindow_DrawSash
    // missing: wxThinSplitterWindow_SashHitTest
    // missing: wxThinSplitterWindow_SizeWindows
}
trait wxThread {
}
trait wxTime {
}
trait wxTimeSpan {
}
trait wxTimer {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetInterval(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsOneShot(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsRuning(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Start(_obj: *u8 /* void* */, _int: c_int /* int */, _one: bool /* bool */) -> bool /* bool */;
    fn Stop(_obj: *u8 /* void* */);
}
trait wxTimerBase {
}
trait wxTimerEvent {
    fn GetInterval(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxTimerEx {
}
trait wxTimerRunner {
}
trait wxTipProvider {
}
trait wxTipWindow {
    fn Close(_obj: *u8 /* void* */);
    fn Create(parent: *u8 /* void* */, text: *u8 /* void* */, maxLength: c_int /* int */) -> *u8 /* void* */;
    fn SetBoundingRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn SetTipWindowPtr(_obj: *u8 /* void* */, windowPtr: *u8 /* void* */);
}
trait wxToggleButton {
    fn Create(parent: *u8 /* void* */, id: c_int /* int */, label: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, style: c_int /* int */) -> *u8 /* void* */;
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    fn GetValue(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetLabel(_obj: *u8 /* void* */, label: *u8 /* void* */);
    fn SetValue(_obj: *u8 /* void* */, state: bool /* bool */);
}
trait wxToolBar {
    fn AddControl(_obj: *u8 /* void* */, ctrl: *u8 /* void* */) -> bool /* bool */;
    fn AddSeparator(_obj: *u8 /* void* */);
    fn AddTool(_obj: *u8 /* void* */, id: c_int /* int */, bmp: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */);
    fn AddToolEx(_obj: *u8 /* void* */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn DeleteTool(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn DeleteToolByPos(_obj: *u8 /* void* */, pos: c_int /* int */) -> bool /* bool */;
    fn EnableTool(_obj: *u8 /* void* */, id: c_int /* int */, enable: bool /* bool */);
    fn GetMargins(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetToolBitmapSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetToolEnabled(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn GetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetToolPacking(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */) -> *u8 /* void* */;
    fn GetToolSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetToolState(_obj: *u8 /* void* */, id: c_int /* int */) -> bool /* bool */;
    fn InsertControl(_obj: *u8 /* void* */, pos: c_int /* int */, ctrl: *u8 /* void* */);
    fn InsertSeparator(_obj: *u8 /* void* */, pos: c_int /* int */);
    fn InsertTool(_obj: *u8 /* void* */, pos: c_int /* int */, id: c_int /* int */, bmp1: *u8 /* void* */, bmp2: *u8 /* void* */, isToggle: bool /* bool */, data: *u8 /* void* */, shelp: *u8 /* void* */, lhelp: *u8 /* void* */);
    fn Realize(_obj: *u8 /* void* */) -> bool /* bool */;
    fn RemoveTool(_obj: *u8 /* void* */, id: c_int /* int */);
    fn SetMargins(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetToolBitmapSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetToolClientData(_obj: *u8 /* void* */, id: c_int /* int */, data: *u8 /* void* */);
    fn SetToolLongHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */);
    fn SetToolPacking(_obj: *u8 /* void* */, packing: c_int /* int */);
    fn SetToolSeparation(_obj: *u8 /* void* */, separation: c_int /* int */);
    fn SetToolShortHelp(_obj: *u8 /* void* */, id: c_int /* int */, str: *u8 /* void* */);
    fn ToggleTool(_obj: *u8 /* void* */, id: c_int /* int */, toggle: bool /* bool */);
}
trait wxToolBarBase {
}
trait wxToolLayoutItem {
    // missing: wxToolLayoutItem_IsSeparator
    // missing: wxToolLayoutItem_Rect
}
trait wxToolTip {
}
trait wxToolWindow {
    // missing: wxToolWindow_AddMiniButton
    // missing: wxToolWindow_Create
    // missing: wxToolWindow_GetClient
    // missing: wxToolWindow_SetClient
    // missing: wxToolWindow_SetTitleFont
}
trait wxTopLevelWindow {
    fn EnableCloseButton(_obj: *u8 /* void* */, enable: bool /* bool */) -> bool /* bool */;
    fn GetDefaultButton(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDefaultItem(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetIcon(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Iconize(_obj: *u8 /* void* */, iconize: bool /* bool */) -> bool /* bool */;
    fn IsActive(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsIconized(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsMaximized(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Maximize(_obj: *u8 /* void* */, maximize: bool /* bool */);
    fn RequestUserAttention(_obj: *u8 /* void* */, flags: c_int /* int */);
    fn SetDefaultButton(_obj: *u8 /* void* */, pBut: *u8 /* void* */);
    fn SetDefaultItem(_obj: *u8 /* void* */, pBut: *u8 /* void* */);
    fn SetIcon(_obj: *u8 /* void* */, pIcon: *u8 /* void* */);
    fn SetIcons(_obj: *u8 /* void* */, _icons: *u8 /* void* */);
    fn SetMaxSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetMinSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetTitle(_obj: *u8 /* void* */, pString: *u8 /* void* */);
}
trait wxTreeCompanionWindow {
    // missing: wxTreeCompanionWindow_Create
    // missing: wxTreeCompanionWindow_DrawItem
    // missing: wxTreeCompanionWindow_GetTreeCtrl
    // missing: wxTreeCompanionWindow_SetTreeCtrl
}
trait wxTreeCtrl {
    fn AddRoot(_obj: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    fn AppendItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    fn Collapse(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn CollapseAndReset(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn Create(_obj: *u8 /* void* */, _cmp: *u8 /* void* */, _prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn DeleteAllItems(_obj: *u8 /* void* */);
    fn DeleteChildren(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn EditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn EndEditLabel(_obj: *u8 /* void* */, item: *u8 /* void* */, discardChanges: bool /* bool */);
    fn EnsureVisible(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn Expand(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn GetBoundingRect(_obj: *u8 /* void* */, item: *u8 /* void* */, textOnly: bool /* bool */) -> *u8 /* void* */;
    fn GetChildrenCount(_obj: *u8 /* void* */, item: *u8 /* void* */, recursively: bool /* bool */) -> c_int /* int */;
    fn GetCount(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetEditControl(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFirstChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */);
    fn GetFirstVisibleItem(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetImageList(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetIndent(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */;
    fn GetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, which: c_int /* int */) -> c_int /* int */;
    fn GetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */;
    fn GetLastChild(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetNextChild(_obj: *u8 /* void* */, item: *u8 /* void* */, cookie: *c_int /* int* */, _item: *u8 /* void* */);
    fn GetNextSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetNextVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetParent(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetPrevSibling(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetPrevVisible(_obj: *u8 /* void* */, item: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetRootItem(_obj: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetSelection(_obj: *u8 /* void* */, _item: *u8 /* void* */);
    fn GetSelections(_obj: *u8 /* void* */, selections: *intptr_t /* intptr_t* */) -> c_int /* int */;
    fn GetSpacing(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetStateImageList(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn HitTest(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, flags: *c_int /* int* */, _item: *u8 /* void* */);
    fn InsertItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    fn InsertItemByIndex(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    fn IsBold(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    fn IsExpanded(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    fn IsSelected(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    fn IsVisible(_obj: *u8 /* void* */, item: *u8 /* void* */) -> bool /* bool */;
    fn ItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */) -> c_int /* int */;
    fn OnCompareItems(_obj: *u8 /* void* */, item1: *u8 /* void* */, item2: *u8 /* void* */) -> c_int /* int */;
    fn PrependItem(_obj: *u8 /* void* */, parent: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, data: *u8 /* void* */, _item: *u8 /* void* */);
    fn ScrollTo(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn SelectItem(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn SetImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    fn SetIndent(_obj: *u8 /* void* */, indent: c_int /* int */);
    fn SetItemBackgroundColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */);
    fn SetItemBold(_obj: *u8 /* void* */, item: *u8 /* void* */, bold: bool /* bool */);
    fn SetItemData(_obj: *u8 /* void* */, item: *u8 /* void* */, data: *u8 /* void* */);
    fn SetItemDropHighlight(_obj: *u8 /* void* */, item: *u8 /* void* */, highlight: bool /* bool */);
    fn SetItemFont(_obj: *u8 /* void* */, item: *u8 /* void* */, font: *u8 /* void* */);
    fn SetItemHasChildren(_obj: *u8 /* void* */, item: *u8 /* void* */, hasChildren: bool /* bool */);
    fn SetItemImage(_obj: *u8 /* void* */, item: *u8 /* void* */, image: c_int /* int */, which: c_int /* int */);
    fn SetItemText(_obj: *u8 /* void* */, item: *u8 /* void* */, text: *u8 /* void* */);
    fn SetItemTextColour(_obj: *u8 /* void* */, item: *u8 /* void* */, col: *u8 /* void* */);
    fn SetSpacing(_obj: *u8 /* void* */, spacing: c_int /* int */);
    fn SetStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    fn SortChildren(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn Toggle(_obj: *u8 /* void* */, item: *u8 /* void* */);
    fn Unselect(_obj: *u8 /* void* */);
    fn UnselectAll(_obj: *u8 /* void* */);
}
trait wxTreeEvent {
    fn GetCode(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetOldItem(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetPoint(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxTreeItemData {
}
trait wxTreeItemId {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn IsOk(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxTreeLayout {
}
trait wxTreeLayoutStored {
}
trait wxURL {
}
trait wxUpdateUIEvent {
    fn Check(_obj: *u8 /* void* */, check: bool /* bool */);
    fn CopyObject(_obj: *u8 /* void* */, obj: *u8 /* void* */);
    fn Enable(_obj: *u8 /* void* */, enable: bool /* bool */);
    fn GetChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetSetChecked(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetSetEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetSetText(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetText(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetText(_obj: *u8 /* void* */, text: *u8 /* void* */);
}
trait wxValidator {
    fn Create() -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetBellOnError(doIt: bool /* bool */);
    fn SetWindow(_obj: *u8 /* void* */, win: *u8 /* void* */);
    fn TransferFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn TransferToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Validate(_obj: *u8 /* void* */, parent: *u8 /* void* */) -> bool /* bool */;
}
trait wxVariant {
}
trait wxVariantData {
}
trait wxView {
}
trait wxSound {
}
trait wxWindow {
    fn AddChild(_obj: *u8 /* void* */, child: *u8 /* void* */);
    fn AddConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */);
    fn CaptureMouse(_obj: *u8 /* void* */);
    fn Center(_obj: *u8 /* void* */, direction: c_int /* int */);
    fn CenterOnParent(_obj: *u8 /* void* */, dir: c_int /* int */);
    fn ClearBackground(_obj: *u8 /* void* */);
    fn ClientToScreen(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn Close(_obj: *u8 /* void* */, _force: bool /* bool */) -> bool /* bool */;
    fn ConvertDialogToPixels(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn ConvertPixelsToDialog(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn DeleteRelatedConstraints(_obj: *u8 /* void* */);
    fn Destroy(_obj: *u8 /* void* */) -> bool /* bool */;
    fn DestroyChildren(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Disable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn DoPhase(_obj: *u8 /* void* */, phase: c_int /* int */) -> c_int /* int */;
    fn Enable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn FindFocus(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn FindWindow(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn Fit(_obj: *u8 /* void* */);
    fn FitInside(_obj: *u8 /* void* */);
    fn Freeze(_obj: *u8 /* void* */);
    fn GetEffectiveMinSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetAutoLayout(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetBackgroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetBestSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCaret(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCharHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetCharWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetChildren(_obj: *u8 /* void* */, _res: *u8 /* void* */, _cnt: c_int /* int */) -> c_int /* int */;
    fn GetClientData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClientSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClientSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetConstraints(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetConstraintsInvolvedIn(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCursor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetDropTarget(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetEventHandler(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFont(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetForegroundColour(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetHandle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetId(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetLabelEmpty(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMaxHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMaxWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinHeight(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetMinWidth(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetParent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPosition(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPositionConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetRect(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    fn GetScrollRange(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    fn GetScrollThumb(_obj: *u8 /* void* */, orient: c_int /* int */) -> c_int /* int */;
    fn GetSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSizeConstraint(_obj: *u8 /* void* */, arg0: *c_int /* int* */, arg1: *c_int /* int* */);
    fn GetSizer(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTextExtent(_obj: *u8 /* void* */, string: *u8 /* void* */, x: *c_int /* int* */, y: *c_int /* int* */, descent: *c_int /* int* */, externalLeading: *c_int /* int* */, theFont: *u8 /* void* */);
    fn GetToolTip(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetUpdateRegion(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetValidator(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetVirtualSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetWindowStyleFlag(_obj: *u8 /* void* */) -> c_int /* int */;
    fn HasFlag(_obj: *u8 /* void* */, flag: c_int /* int */) -> bool /* bool */;
    fn Hide(_obj: *u8 /* void* */) -> bool /* bool */;
    fn InitDialog(_obj: *u8 /* void* */);
    fn IsBeingDeleted(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsEnabled(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsExposed(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> bool /* bool */;
    fn IsShown(_obj: *u8 /* void* */) -> bool /* bool */;
    fn IsTopLevel(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Layout(_obj: *u8 /* void* */) -> c_int /* int */;
    fn LayoutPhase1(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */;
    fn LayoutPhase2(_obj: *u8 /* void* */, noChanges: *c_int /* int* */) -> c_int /* int */;
    fn Lower(_obj: *u8 /* void* */);
    fn MakeModal(_obj: *u8 /* void* */, modal: bool /* bool */);
    fn Move(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn MoveConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn PopEventHandler(_obj: *u8 /* void* */, deleteHandler: bool /* bool */) -> *u8 /* void* */;
    fn PopupMenu(_obj: *u8 /* void* */, menu: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    fn PrepareDC(_obj: *u8 /* void* */, dc: *u8 /* void* */);
    fn PushEventHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    fn Raise(_obj: *u8 /* void* */);
    fn Refresh(_obj: *u8 /* void* */, eraseBackground: bool /* bool */);
    fn RefreshRect(_obj: *u8 /* void* */, eraseBackground: bool /* bool */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn ReleaseMouse(_obj: *u8 /* void* */);
    fn RemoveChild(_obj: *u8 /* void* */, child: *u8 /* void* */);
    fn RemoveConstraintReference(_obj: *u8 /* void* */, otherWin: *u8 /* void* */);
    fn Reparent(_obj: *u8 /* void* */, _par: *u8 /* void* */) -> c_int /* int */;
    fn ResetConstraints(_obj: *u8 /* void* */);
    fn ScreenToClient(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn ScrollWindow(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn ScrollWindowRect(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, arg4: c_int /* int */, arg5: c_int /* int */);
    fn SetAcceleratorTable(_obj: *u8 /* void* */, accel: *u8 /* void* */);
    fn SetAutoLayout(_obj: *u8 /* void* */, autoLayout: bool /* bool */);
    fn SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    fn SetCaret(_obj: *u8 /* void* */, caret: *u8 /* void* */);
    fn SetClientData(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn SetClientObject(_obj: *u8 /* void* */, data: *u8 /* void* */);
    fn SetClientSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn SetConstraintSizes(_obj: *u8 /* void* */, recurse: c_int /* int */);
    fn SetConstraints(_obj: *u8 /* void* */, constraints: *u8 /* void* */);
    fn SetCursor(_obj: *u8 /* void* */, cursor: *u8 /* void* */) -> c_int /* int */;
    fn SetDropTarget(_obj: *u8 /* void* */, dropTarget: *u8 /* void* */);
    fn SetExtraStyle(_obj: *u8 /* void* */, exStyle: c_long /* long */);
    fn SetFocus(_obj: *u8 /* void* */);
    fn SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */) -> c_int /* int */;
    fn SetForegroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */) -> c_int /* int */;
    fn SetId(_obj: *u8 /* void* */, _id: c_int /* int */);
    fn SetLabel(_obj: *u8 /* void* */, _title: *u8 /* void* */);
    fn SetName(_obj: *u8 /* void* */, _name: *u8 /* void* */);
    fn SetScrollPos(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, refresh: bool /* bool */);
    fn SetScrollbar(_obj: *u8 /* void* */, orient: c_int /* int */, pos: c_int /* int */, thumbVisible: c_int /* int */, range: c_int /* int */, refresh: bool /* bool */);
    fn SetSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, sizeFlags: c_int /* int */);
    fn SetSizeConstraint(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */);
    fn SetSizeHints(_obj: *u8 /* void* */, minW: c_int /* int */, minH: c_int /* int */, maxW: c_int /* int */, maxH: c_int /* int */, incW: c_int /* int */, incH: c_int /* int */);
    fn SetSizer(_obj: *u8 /* void* */, sizer: *u8 /* void* */);
    fn SetToolTip(_obj: *u8 /* void* */, tip: *u8 /* void* */);
    fn SetValidator(_obj: *u8 /* void* */, validator: *u8 /* void* */);
    fn SetWindowStyleFlag(_obj: *u8 /* void* */, style: c_long /* long */);
    fn Show(_obj: *u8 /* void* */) -> bool /* bool */;
    fn Thaw(_obj: *u8 /* void* */);
    fn TransferDataFromWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn TransferDataToWindow(_obj: *u8 /* void* */) -> bool /* bool */;
    fn UnsetConstraints(_obj: *u8 /* void* */, c: *u8 /* void* */);
    fn UpdateWindowUI(_obj: *u8 /* void* */);
    fn Validate(_obj: *u8 /* void* */) -> bool /* bool */;
    fn SetVirtualSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
    fn WarpPointer(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
}
trait wxWindowCreateEvent {
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxWindowDC {
    fn Create(win: *u8 /* void* */) -> *u8 /* void* */;
    fn Delete(_obj: *u8 /* void* */);
}
trait wxWindowDestroyEvent {
    fn GetWindow(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxWindowDisabler {
}
trait wxWizard {
    fn Chain(f: *u8 /* void* */, s: *u8 /* void* */);
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, _txt: *u8 /* void* */, _bmp: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */) -> *u8 /* void* */;
    fn GetCurrentPage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPageSize(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn RunWizard(_obj: *u8 /* void* */, firstPage: *u8 /* void* */) -> c_int /* int */;
    fn SetPageSize(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */);
}
trait wxWizardEvent {
    fn GetDirection(_obj: *u8 /* void* */) -> c_int /* int */;
}
trait wxWizardPage {
}
trait wxWizardPageSimple {
    fn Create(_prt: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBitmap(_obj: *u8 /* void* */, _ref: *u8 /* void* */);
    fn GetNext(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPrev(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetNext(_obj: *u8 /* void* */, next: *u8 /* void* */);
    fn SetPrev(_obj: *u8 /* void* */, prev: *u8 /* void* */);
}
trait wxXmlResource {
    fn AddHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    fn AddSubclassFactory(_obj: *u8 /* void* */, factory: *u8 /* void* */);
    fn AttachUnknownControl(_obj: *u8 /* void* */, control: *u8 /* void* */, parent: *u8 /* void* */) -> c_int /* int */;
    fn ClearHandlers(_obj: *u8 /* void* */);
    fn CompareVersion(_obj: *u8 /* void* */, major: c_int /* int */, minor: c_int /* int */, release: c_int /* int */, revision: c_int /* int */) -> c_int /* int */;
    fn Create(flags: c_int /* int */) -> *u8 /* void* */;
    fn CreateFromFile(filemask: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */;
    // missing: wxXmlResource_Delete
    fn Get() -> *u8 /* void* */;
    fn GetDomain(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFlags(_obj: *u8 /* void* */) -> c_int /* int */;
    fn GetVersion(_obj: *u8 /* void* */) -> c_long /* long */;
    fn GetXRCID(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> c_int /* int */;
    fn InitAllHandlers(_obj: *u8 /* void* */);
    fn InsertHandler(_obj: *u8 /* void* */, handler: *u8 /* void* */);
    fn Load(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */;
    fn LoadBitmap(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */);
    fn LoadDialog(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn LoadFrame(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn LoadIcon(_obj: *u8 /* void* */, name: *u8 /* void* */, _ref: *u8 /* void* */);
    fn LoadMenu(_obj: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn LoadMenuBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn LoadPanel(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn LoadToolBar(_obj: *u8 /* void* */, parent: *u8 /* void* */, name: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStaticBoxSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetFlexGridSizer(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBitmapButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCalendarCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCheckBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetCheckListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetChoice(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetComboBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetGauge(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetGrid(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetHtmlWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetListBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetListCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMDIChildFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMDIParentFrame(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMenu(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMenuBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetMenuItem(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetNotebook(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetPanel(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetRadioButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetRadioBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetScrollBar(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetScrolledWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSlider(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSpinButton(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSpinCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSplitterWindow(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStaticBitmap(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStaticBox(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStaticLine(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetStaticText(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTextCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTreeCtrl(_obj: *u8 /* void* */, str_id: *u8 /* void* */) -> *u8 /* void* */;
    fn Unload(_obj: *u8 /* void* */, filemask: *u8 /* void* */) -> bool /* bool */;
    fn Set(_obj: *u8 /* void* */, res: *u8 /* void* */) -> *u8 /* void* */;
    fn SetDomain(_obj: *u8 /* void* */, domain: *u8 /* void* */);
    fn SetFlags(_obj: *u8 /* void* */, flags: c_int /* int */);
}
trait wxXmlResourceHandler {
}
trait wxZipInputStream {
}
trait wxZlibInputStream {
}
trait wxZlibOutputStream {
}
trait wxPropertyGrid {
    fn Append(_obj: *u8 /* void* */, prop: *u8 /* void* */) -> *u8 /* void* */;
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn DisableProperty(_obj: *u8 /* void* */, propName: *u8 /* void* */) -> bool /* bool */;
}
trait wxPropertyGridEvent {
    fn HasProperty(_obj: *u8 /* void* */) -> bool /* bool */;
    fn GetProperty(_obj: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxPGProperty {
    fn GetLabel(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetName(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetValueAsString(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetValueType(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn SetHelpString(_obj: *u8 /* void* */, helpString: *u8 /* void* */);
}
trait wxStringProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxIntProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: c_int /* int */) -> *u8 /* void* */;
}
trait wxBoolProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: bool /* bool */) -> *u8 /* void* */;
}
trait wxFloatProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: float /* float */) -> *u8 /* void* */;
}
trait wxDateProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxFileProperty {
    fn Create(label: *u8 /* void* */, name: *u8 /* void* */, value: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxPropertyCategory {
    fn Create(label: *u8 /* void* */) -> *u8 /* void* */;
}
trait None {
}
trait wxClosure {
    fn Create(_fun_CEvent: *u8 /* void* */, _data: *u8 /* void* */) -> *u8 /* void* */;
    fn GetData(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxEvtHandler_GetClosure(_obj: *u8 /* void* */, id: c_int /* int */, type_: c_int /* int */) -> *u8 /* void* */;
    fn wxEvtHandler_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxEvtHandler_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    fn wxObject_GetClientClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxObject_SetClientClosure(_obj: *u8 /* void* */, closure: *u8 /* void* */);
}
trait wxGauge95 {
}
trait wxGaugeMSW {
}
trait wxSlider95 {
}
trait wxSliderMSW {
    fn wxObject_Delete(obj: *u8 /* void* */);
    fn wxFrame_GetTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxFrame_SetTitle(_frame: *u8 /* void* */, _txt: *u8 /* void* */);
    fn wxFrame_SetShape(self_: *u8 /* void* */, region: *u8 /* void* */) -> bool /* bool */;
    fn wxFrame_ShowFullScreen(self_: *u8 /* void* */, show: bool /* bool */, style: c_int /* int */) -> bool /* bool */;
    fn wxFrame_IsFullScreen(self_: *u8 /* void* */) -> bool /* bool */;
    fn wxFrame_Centre(self_: *u8 /* void* */, orientation: c_int /* int */);
    fn wxCursor_Delete(_obj: *u8 /* void* */);
    fn wxDateTime_Delete(_obj: *u8 /* void* */);
    fn wxMouseEvent_GetWheelDelta(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxMouseEvent_GetWheelRotation(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxMouseEvent_GetButton(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxcGetMousePosition() -> *u8 /* void* */;
    fn wxDC_GetUserScaleX(dc: *u8 /* void* */) -> c_double /* double */;
    fn wxDC_GetUserScaleY(dc: *u8 /* void* */) -> c_double /* double */;
    fn wxWindow_ConvertDialogToPixelsEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxWindow_ConvertPixelsToDialogEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxWindow_ScreenToClient2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> *u8 /* void* */;
    fn wxString_Create(buffer: *wchar_t /* wchar_t* */) -> *u8 /* void* */;
    fn wxString_CreateLen(buffer: *wchar_t /* wchar_t* */, len: c_int /* int */) -> *u8 /* void* */;
    fn wxString_Delete(s: *u8 /* void* */);
    fn wxString_GetString(s: *u8 /* void* */, buffer: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn wxString_Length(s: *u8 /* void* */) -> size_t /* size_t */;
    fn wxMenu_GetMenuBar(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxMenuBar_GetFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxListEvent_GetCacheFrom(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxListEvent_GetCacheTo(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxListCtrl_AssignImageList(_obj: *u8 /* void* */, images: *u8 /* void* */, which: c_int /* int */);
    fn wxListCtrl_GetColumn2(_obj: *u8 /* void* */, col: c_int /* int */, item: *u8 /* void* */);
    fn wxListCtrl_GetItem2(_obj: *u8 /* void* */, info: *u8 /* void* */);
    fn wxListCtrl_GetItemPosition2(_obj: *u8 /* void* */, item: c_int /* int */) -> *u8 /* void* */;
    fn wxListCtrl_SortItems2(_obj: *u8 /* void* */, closure: *u8 /* void* */) -> bool /* bool */;
}
trait wxcTreeItemData {
    fn Create(closure: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClientClosure(self_: *u8 /* void* */) -> *u8 /* void* */;
    fn SetClientClosure(self_: *u8 /* void* */, closure: *u8 /* void* */);
    fn wxTreeItemId_Clone(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxTreeItemId_CreateFromValue(value: intptr_t /* intptr_t */) -> *u8 /* void* */;
    fn wxTreeItemId_GetValue(_obj: *u8 /* void* */) -> intptr_t /* intptr_t */;
    fn wxTreeEvent_GetKeyEvent(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxTreeEvent_IsEditCancelled(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxTreeEvent_Allow(_obj: *u8 /* void* */);
    fn wxTreeCtrl_Create2(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */) -> *u8 /* void* */;
    fn wxTreeCtrl_InsertItem2(_obj: *u8 /* void* */, parent: *u8 /* void* */, idPrevious: *u8 /* void* */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */);
    fn wxTreeCtrl_InsertItemByIndex2(_obj: *u8 /* void* */, parent: *u8 /* void* */, index: c_int /* int */, text: *u8 /* void* */, image: c_int /* int */, selectedImage: c_int /* int */, closure: *u8 /* void* */, _item: *u8 /* void* */);
    fn wxTreeCtrl_GetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */) -> *u8 /* void* */;
    fn wxTreeCtrl_SetItemClientClosure(_obj: *u8 /* void* */, item: *u8 /* void* */, closure: *u8 /* void* */);
    fn wxTreeCtrl_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    fn wxTreeCtrl_AssignStateImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
    fn wxDC_GetPixel2(_obj: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, col: *u8 /* void* */);
    fn wxScrolledWindow_SetScrollRate(_obj: *u8 /* void* */, xstep: c_int /* int */, ystep: c_int /* int */);
}
trait wxObject {
    fn GetClassInfo(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn IsKindOf(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */;
    fn IsScrolledWindow(_obj: *u8 /* void* */) -> bool /* bool */;
}
trait wxClassInfo {
    fn FindClass(_txt: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBaseClassName1(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetBaseClassName2(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetClassNameEx(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn GetSize(_obj: *u8 /* void* */) -> c_int /* int */;
    fn IsKindOfEx(_obj: *u8 /* void* */, classInfo: *u8 /* void* */) -> bool /* bool */;
    fn wxNotebook_AssignImageList(_obj: *u8 /* void* */, imageList: *u8 /* void* */);
}
trait wxTimerEx {
    fn Connect(_obj: *u8 /* void* */, closure: *u8 /* void* */);
    fn Create() -> *u8 /* void* */;
    fn GetClosure(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxMenu_AppendRadioItem(self_: *u8 /* void* */, id: c_int /* int */, text: *u8 /* void* */, help: *u8 /* void* */);
    fn wxMenuItem_CreateSeparator() -> *u8 /* void* */;
    fn wxMenuItem_CreateEx(id: c_int /* int */, label: *u8 /* void* */, help: *u8 /* void* */, itemkind: c_int /* int */, submenu: *u8 /* void* */) -> *u8 /* void* */;
    fn wxToolBar_AddTool2(_obj: *u8 /* void* */, toolId: c_int /* int */, label: *u8 /* void* */, bmp: *u8 /* void* */, bmpDisabled: *u8 /* void* */, itemKind: c_int /* int */, shortHelp: *u8 /* void* */, longHelp: *u8 /* void* */);
    fn wxProgressDialog_Create(title: *u8 /* void* */, message: *u8 /* void* */, max: c_int /* int */, parent: *u8 /* void* */, style: c_int /* int */) -> *u8 /* void* */;
    fn wxProgressDialog_Update(obj: *u8 /* void* */, value: c_int /* int */) -> bool /* bool */;
    fn wxProgressDialog_UpdateWithMessage(obj: *u8 /* void* */, value: c_int /* int */, message: *u8 /* void* */) -> bool /* bool */;
    fn wxProgressDialog_Resume(obj: *u8 /* void* */);
    fn wxVersionNumber() -> c_int /* int */;
    fn wxIsDefined(s: *wchar_t /* wchar_t* */) -> c_int /* int */;
}
trait wxInputSink {
    fn Create(input: *u8 /* void* */, evtHandler: *u8 /* void* */, bufferLen: c_int /* int */) -> *u8 /* void* */;
    fn GetId(obj: *u8 /* void* */) -> c_int /* int */;
    fn Start(obj: *u8 /* void* */);
}
trait wxInputSinkEvent {
    fn LastError(obj: *u8 /* void* */) -> c_int /* int */;
    fn LastRead(obj: *u8 /* void* */) -> c_int /* int */;
    fn LastInput(obj: *u8 /* void* */) -> *char /* char* */;
}
trait wxcHtmlEvent {
    fn GetMouseEvent(self_: *u8 /* void* */) -> *u8 /* void* */;
    fn GetHtmlCell(self_: *u8 /* void* */) -> *u8 /* void* */;
    fn GetHtmlCellId(self_: *u8 /* void* */) -> *u8 /* void* */;
    fn GetHref(self_: *u8 /* void* */) -> *u8 /* void* */;
    fn GetTarget(self_: *u8 /* void* */) -> *u8 /* void* */;
    fn GetLogicalPosition(self_: *u8 /* void* */) -> *u8 /* void* */;
}
trait wxcHtmlWindow {
    fn Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_Create(_prt: *u8 /* void* */, _id: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, arg2: c_int /* int */, arg3: c_int /* int */, _stl: c_int /* int */, _txt: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_AppendToPage(_obj: *u8 /* void* */, source: *u8 /* void* */) -> bool /* bool */;
    fn wxHtmlWindow_GetInternalRepresentation(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_GetOpenedAnchor(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_GetOpenedPage(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_GetOpenedPageTitle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_GetRelatedFrame(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxHtmlWindow_HistoryBack(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxHtmlWindow_HistoryCanBack(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxHtmlWindow_HistoryCanForward(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxHtmlWindow_HistoryClear(_obj: *u8 /* void* */);
    fn wxHtmlWindow_HistoryForward(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxHtmlWindow_LoadPage(_obj: *u8 /* void* */, location: *u8 /* void* */) -> bool /* bool */;
    fn wxHtmlWindow_ReadCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
    fn wxHtmlWindow_SetBorders(_obj: *u8 /* void* */, b: c_int /* int */);
    fn wxHtmlWindow_SetFonts(_obj: *u8 /* void* */, normal_face: *u8 /* void* */, fixed_face: *u8 /* void* */, sizes: *c_int /* int* */);
    fn wxHtmlWindow_SetPage(_obj: *u8 /* void* */, source: *u8 /* void* */);
    fn wxHtmlWindow_SetRelatedFrame(_obj: *u8 /* void* */, frame: *u8 /* void* */, format: *u8 /* void* */);
    fn wxHtmlWindow_SetRelatedStatusBar(_obj: *u8 /* void* */, bar: c_int /* int */);
    fn wxHtmlWindow_WriteCustomization(_obj: *u8 /* void* */, cfg: *u8 /* void* */, path: *u8 /* void* */);
}
trait wxGridCellTextEnterEditor {
    fn Ctor() -> *u8 /* void* */;
    fn wxLogStderr_Create() -> *u8 /* void* */;
    fn wxLogStderr_CreateStdOut() -> *u8 /* void* */;
    fn wxLogNull_Create() -> *u8 /* void* */;
    fn wxLogTextCtrl_Create(text: *u8 /* void* */) -> *u8 /* void* */;
    fn wxLogWindow_Create(parent: *u8 /* void* */, title: *wchar_t /* wchar_t* */, showit: bool /* bool */, passthrough: bool /* bool */) -> *u8 /* void* */;
    fn wxLogWindow_GetFrame(obj: *u8 /* void* */) -> *u8 /* void* */;
    fn LogError(_msg: *u8 /* void* */);
    fn LogFatalError(_msg: *u8 /* void* */);
    fn LogWarning(_msg: *u8 /* void* */);
    fn LogMessage(_msg: *u8 /* void* */);
    fn LogVerbose(_msg: *u8 /* void* */);
    fn LogStatus(_msg: *u8 /* void* */);
    fn LogSysError(_msg: *u8 /* void* */);
    fn LogDebug(_msg: *u8 /* void* */);
    fn LogTrace(mask: *u8 /* void* */, _msg: *u8 /* void* */);
    fn wxLog_AddTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */);
    fn wxLog_Delete(_obj: *u8 /* void* */);
    fn wxLog_DontCreateOnDemand(_obj: *u8 /* void* */);
    fn wxLog_Flush(_obj: *u8 /* void* */);
    fn wxLog_FlushActive(_obj: *u8 /* void* */);
    fn wxLog_GetActiveTarget() -> *u8 /* void* */;
    fn wxLog_GetTimestamp(_obj: *u8 /* void* */) -> *char /* char* */;
    fn wxLog_GetTraceMask(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxLog_GetVerbose(_obj: *u8 /* void* */) -> c_int /* int */;
    fn wxLog_HasPendingMessages(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxLog_IsAllowedTraceMask(_obj: *u8 /* void* */, mask: *u8 /* void* */) -> bool /* bool */;
    fn wxLog_OnLog(_obj: *u8 /* void* */, level: c_int /* int */, szString: *wchar_t /* wchar_t* */, t: c_int /* int */);
    fn wxLog_RemoveTraceMask(_obj: *u8 /* void* */, str: *u8 /* void* */);
    fn wxLog_Resume(_obj: *u8 /* void* */);
    fn wxLog_SetActiveTarget(pLogger: *u8 /* void* */) -> *u8 /* void* */;
    fn wxLog_SetTimestamp(_obj: *u8 /* void* */, ts: *wchar_t /* wchar_t* */);
    fn wxLog_SetTraceMask(_obj: *u8 /* void* */, ulMask: c_int /* int */);
    fn wxLog_SetVerbose(_obj: *u8 /* void* */, bVerbose: c_int /* int */);
    fn wxLog_Suspend(_obj: *u8 /* void* */);
    fn wxProcess_Open(cmd: *u8 /* void* */, flags: c_int /* int */) -> *u8 /* void* */;
    fn wxProcess_IsErrorAvailable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxProcess_IsInputAvailable(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxProcess_IsInputOpened(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxKill(pid: c_int /* int */, signal: c_int /* int */) -> c_int /* int */;
    fn wxStreamBase_Delete(obj: *u8 /* void* */);
    fn wxGetColourFromUser(parent: *u8 /* void* */, colInit: *u8 /* void* */, colour: *u8 /* void* */);
    fn wxGetFontFromUser(parent: *u8 /* void* */, fontInit: *u8 /* void* */, font: *u8 /* void* */);
    fn wxGetPasswordFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn wxGetTextFromUser(message: *wchar_t /* wchar_t* */, caption: *wchar_t /* wchar_t* */, defaultText: *wchar_t /* wchar_t* */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */, center: bool /* bool */, _buf: *wchar_t /* wchar_t* */) -> c_int /* int */;
    fn wxGetNumberFromUser(message: *u8 /* void* */, prompt: *u8 /* void* */, caption: *u8 /* void* */, value: c_long /* long */, min: c_long /* long */, max: c_long /* long */, parent: *u8 /* void* */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_long /* long */;
    fn wxcBell();
    fn wxcBeginBusyCursor();
    fn wxcEndBusyCursor();
    fn wxcIsBusy();
    fn wxTextCtrl_EmulateKeyPress(_obj: *u8 /* void* */, keyevent: *u8 /* void* */) -> bool /* bool */;
    fn wxTextCtrl_GetDefaultStyle(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxTextCtrl_GetRange(_obj: *u8 /* void* */, from: c_long /* long */, to: c_long /* long */) -> *u8 /* void* */;
    fn wxTextCtrl_GetStringSelection(_obj: *u8 /* void* */) -> *u8 /* void* */;
    fn wxTextCtrl_IsMultiLine(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxTextCtrl_IsSingleLine(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxTextCtrl_SetDefaultStyle(_obj: *u8 /* void* */, style: *u8 /* void* */) -> bool /* bool */;
    fn wxTextCtrl_SetMaxLength(_obj: *u8 /* void* */, len: c_long /* long */);
    fn wxTextCtrl_SetStyle(_obj: *u8 /* void* */, start: c_long /* long */, end: c_long /* long */, style: *u8 /* void* */) -> bool /* bool */;
    fn wxTextAttr_Create(colText: *u8 /* void* */, colBack: *u8 /* void* */, font: *u8 /* void* */) -> *u8 /* void* */;
    fn wxTextAttr_CreateDefault() -> *u8 /* void* */;
    fn wxTextAttr_Delete(_obj: *u8 /* void* */);
    fn wxTextAttr_GetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn wxTextAttr_GetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
    fn wxTextAttr_GetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn wxTextAttr_HasBackgroundColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxTextAttr_HasFont(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxTextAttr_HasTextColour(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxTextAttr_IsDefault(_obj: *u8 /* void* */) -> bool /* bool */;
    fn wxTextAttr_SetTextColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn wxTextAttr_SetBackgroundColour(_obj: *u8 /* void* */, colour: *u8 /* void* */);
    fn wxTextAttr_SetFont(_obj: *u8 /* void* */, font: *u8 /* void* */);
}
trait wxFileConfig {
    fn wxConfigBase_Get() -> *u8 /* void* */;
    fn wxConfigBase_Set(self_: *u8 /* void* */);
    fn Create(inp: *u8 /* void* */) -> *u8 /* void* */;
    fn wxBitmap_CreateFromImage(image: *u8 /* void* */, depth: c_int /* int */) -> *u8 /* void* */;
    fn wxImage_CreateFromDataEx(arg0: c_int /* int */, arg1: c_int /* int */, data: *u8 /* void* */, isStaticData: c_int /* int */) -> *u8 /* void* */;
    fn wxImage_Delete(image: *u8 /* void* */);
    fn wxColour_CreateFromInt(rgb: c_int /* int */) -> *u8 /* void* */;
    fn wxColour_GetInt(colour: *u8 /* void* */) -> c_int /* int */;
    fn wxColour_CreateFromUnsignedInt(rgba: uint32_t /* uint32_t */) -> *u8 /* void* */;
    fn wxColour_GetUnsignedInt(colour: *u8 /* void* */) -> uint32_t /* uint32_t */;
    fn wxcSystemSettingsGetColour(systemColour: c_int /* int */) -> *u8 /* void* */;
    fn wxcSetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgb: c_int /* int */);
    fn wxcGetPixelRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> c_int /* int */;
    fn wxcSetPixelRowRGB(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbStart: c_int /* int */, rgbEnd: c_int /* int */, count: c_int /* int */);
    fn wxcInitPixelsRGB(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: c_int /* int */);
    fn wxcSetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */);
    fn wxcGetPixelRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */) -> uint32_t /* uint32_t */;
    fn wxcSetPixelRowRGBA(buffer: *uint8_t /* uint8_t* */, width: c_int /* int */, arg0: c_int /* int */, arg1: c_int /* int */, rgbaStart: c_int /* int */, rgbEnd: c_int /* int */, count: uint32_t /* uint32_t */);
    fn wxcInitPixelsRGBA(buffer: *uint8_t /* uint8_t* */, arg0: c_int /* int */, arg1: c_int /* int */, rgba: uint32_t /* uint32_t */);
    fn wxcMalloc(size: c_int /* int */) -> *u8 /* void* */;
    fn wxcFree(p: *u8 /* void* */);
    fn wxcWakeUpIdle();
    fn wxGetApplicationDir() -> *u8 /* void* */;
    fn wxGetApplicationPath() -> *u8 /* void* */;
    fn ELJApp_InitializeC(closure: *u8 /* void* */, _argc: c_int /* int */, _argv: *wchar_t /* wchar_t* */);
    fn ELJApp_GetIdleInterval() -> c_int /* int */;
    fn ELJApp_SetIdleInterval(interval: c_int /* int */);
}
