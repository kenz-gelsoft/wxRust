from __future__ import print_function
from subprocess import PIPE, Popen

import re
import sys


PY3 = sys.version_info[0] == 3


modules = {
    'base': {
        'depends': ['_unsafe'],
        'classes': [
                    'ELJClient',
                    'ELJConnection',
                    'ELJLocale',
                    'ELJServer',
                    'wxArray',
                    'wxArrayString',
                    'wxBufferedInputStream',
                    'wxBufferedOutputStream',
                    'wxCSConv',
                    'wxClassInfo',
                    'wxClient',
                    'wxClientBase',
                    'wxClientData',
                    'wxClientDataContainer',
                    'wxClosure',
                    'wxCommandLineParser',
                    'wxCondition',
                    'wxConfigBase',
                    'wxConnection',
                    'wxConnectionBase',
                    'wxCountingOutputStream',
                    'wxCriticalSection',
                    'wxCriticalSectionLocker',
                    'wxDDEClient',
                    'wxDDEConnection',
                    'wxDDEServer',
                    'wxDllLoader',
                    'wxDataInputStream',
                    'wxDataOutputStream',
                    'wxDateTime',
                    'wxDebugContext',
                    'wxDirTraverser',
                    'wxDynamicLibrary',
                    'wxEncodingConverter',
                    'wxFFile',
                    'wxFFileInputStream',
                    'wxFFileOutputStream',
                    'wxFSFile',
                    'wxFileConfig',
                    'wxFileInputStream',
                    'wxFileName',
                    'wxFileOutputStream',
                    'wxFileSystem',
                    'wxFileSystemHandler',
                    'wxFilterInputStream',
                    'wxFilterOutputStream',
                    'wxInputStream',
                    'wxList',
                    'wxLocale',
                    'wxLongLong',
                    'wxMBConv',
                    'wxMBConvUTF7',
                    'wxMBConvUTF8',
                    'wxMemoryBuffer',
                    'wxMemoryFSHandler',
                    'wxMemoryInputStream',
                    'wxMemoryOutputStream',
                    'wxModule',
                    'wxMutex',
                    'wxMutexLocker',
                    'wxNodeBase',
                    'wxObject',
                    'wxObjectRefData',
                    'wxOutputStream',
                    'wxPathList',
                    'wxRegEx',
                    'wxScopedArray',
                    'wxScopedPtr',
                    'wxSemaphore',
                    'wxServer',
                    'wxServerBase',
                    'wxSingleInstanceChecker',
                    'wxStopWatch',
                    'wxStreamBase',
                    'wxStreamBuffer',
                    'wxString',
                    'wxStringBuffer',
                    'wxStringClientData',
                    'wxStringList',
                    'wxStringTokenizer',
                    'wxSystemOptions',
                    'wxTempFile',
                    'wxTextFile',
                    'wxTextInputStream',
                    'wxTextOutputStream',
                    'wxThread',
                    'wxTime',
                    'wxTimeSpan',
                    'wxVariant',
                    'wxVariantData',
                    'wxZipInputStream',
                    'wxZlibInputStream',
                    'wxZlibOutputStream',
                    ]
    },
    'core': {
        'depends': ['_unsafe', 'base'],
        'classes': [
                    'ELJApp',
                    'ELJArtProv',
                    'ELJCommand',
                    'ELJDragDataObject',
                    'ELJDropTarget',
                    'ELJFileDropTarget',
                    'ELJLog',
                    'ELJPreviewControlBar',
                    'ELJPreviewFrame',
                    'ELJTextDropTarget',
                    'ELJTextValidator',
                    'wxAcceleratorEntry',
                    'wxAcceleratorTable',
                    'wxActivateEvent',
                    'wxApp',
                    'wxArtProvider',
                    'wxAutoBufferedPaintDC',
                    'wxAutomationObject',
                    'wxBitmap',
                    'wxBitmapButton',
                    'wxBitmapDataObject',
                    'wxBitmapHandler',
                    'wxBitmapToggleButton',
                    'wxBoxSizer',
                    'wxBrush',
                    'wxBrushList',
                    'wxBufferedDC',
                    'wxBufferedPaintDC',
                    'wxBusyCursor',
                    'wxBusyInfo',
                    'wxButton',
                    'wxCaret',
                    'wxCheckBox',
                    'wxCheckListBox',
                    'wxChoice',
                    'wxClientDC',
                    'wxClipboard',
                    'wxCloseEvent',
                    'wxColour',
                    'wxColourData',
                    'wxColourDatabase',
                    'wxColourDialog',
                    'wxComboBox',
                    'wxCommand',
                    'wxCommandEvent',
                    'wxCommandProcessor',
                    'wxContextHelp',
                    'wxContextHelpButton',
                    'wxControl',
                    'wxCursor',
                    'wxCustomDataObject',
                    'wxDC',
                    'wxDCClipper',
                    'wxDataFormat',
                    'wxDataObject',
                    'wxDataObjectComposite',
                    'wxDataObjectSimple',
                    'wxDialUpEvent',
                    'wxDialUpManager',
                    'wxDialog',
                    'wxDirDialog',
                    'wxDocChildFrame',
                    'wxDocParentFrame',
                    'wxDocMDIChildFrame',
                    'wxDocMDIParentFrame',
                    'wxDocManager',
                    'wxDocTemplate',
                    'wxDocument',
                    'wxDragImage',
                    'wxDrawControl',
                    'wxDrawWindow',
                    'wxDropFilesEvent',
                    'wxDropSource',
                    'wxDropTarget',
                    'wxEraseEvent',
                    'wxEvent',
                    'wxEvtHandler',
                    'wxFileDataObject',
                    'wxFileDialog',
                    'wxFileDropTarget',
                    'wxFileHistory',
                    'wxFileType',
                    'wxFindDialogEvent',
                    'wxFindReplaceData',
                    'wxFindReplaceDialog',
                    'wxFlexGridSizer',
                    'wxFocusEvent',
                    'wxFont',
                    'wxFontData',
                    'wxFontDialog',
                    'wxFontEnumerator',
                    'wxFontList',
                    'wxFontMapper',
                    'wxFrame',
                    'wxGDIObject',
                    'wxGauge',
                    'wxGenericDirCtrl',
                    'wxGenericDragImage',
                    'wxGenericValidator',
                    'wxGraphicsBrush',
                    'wxGraphicsContext',
                    'wxGraphicsFont',
                    'wxGraphicsMatrix',
                    'wxGraphicsObject',
                    'wxGraphicsPath',
                    'wxGraphicsPen',
                    'wxGraphicsRenderer',
                    'wxGridSizer',
                    'wxHelpController',
                    'wxHelpControllerBase',
                    'wxHelpControllerHelpProvider',
                    'wxHelpEvent',
                    'wxHelpProvider',
                    'wxIcon',
                    'wxIconBundle',
                    'wxIconizeEvent',
                    'wxIdleEvent',
                    'wxImage',
                    'wxImageHandler',
                    'wxImageList',
                    'wxIndividualLayoutConstraint',
                    'wxInitDialogEvent',
                    'wxInputSink',
                    'wxInputSinkEvent',
                    'wxJoystickEvent',
                    'wxKeyEvent',
                    'wxLayoutConstraints',
                    'wxListBox',
                    'wxListCtrl',
                    'wxListEvent',
                    'wxListItem',
                    'wxLog',
                    'wxLogChain',
                    'wxLogGUI',
                    'wxLogNull',
                    'wxLogPassThrough',
                    'wxLogStderr',
                    'wxLogStream',
                    'wxLogTextCtrl',
                    'wxLogWindow',
                    'wxMDIChildFrame',
                    'wxMDIClientWindow',
                    'wxMDIParentFrame',
                    'wxMask',
                    'wxMaximizeEvent',
                    'wxMemoryDC',
                    'wxMenu',
                    'wxMenuBar',
                    'wxMenuEvent',
                    'wxMenuItem',
                    'wxMessageDialog',
                    'wxMetafile',
                    'wxMetafileDC',
                    'wxMimeTypesManager',
                    'wxMiniFrame',
                    'wxMirrorDC',
                    'wxMouseCaptureChangedEvent',
                    'wxMouseEvent',
                    'wxMoveEvent',
                    'wxNavigationKeyEvent',
                    'wxNotebook',
                    'wxNotebookEvent',
                    'wxNotifyEvent',
                    'wxPageSetupDialog',
                    'wxPageSetupDialogData',
                    'wxPaintDC',
                    'wxPaintEvent',
                    'wxPalette',
                    'wxPaletteChangedEvent',
                    'wxPanel',
                    'wxPen',
                    'wxPenList',
                    'wxPoint',
                    'wxPopupTransientWindow',
                    'wxPopupWindow',
                    'wxPostScriptDC',
                    'wxPostScriptPrintNativeData',
                    'wxPreviewCanvas',
                    'wxPreviewControlBar',
                    'wxPreviewFrame',
                    'wxPrintData',
                    'wxPrintDialog',
                    'wxPrintDialogData',
                    'wxPrintPreview',
                    'wxPrinter',
                    'wxPrinterDC',
                    'wxPrintout',
                    'wxPrivateDropTarget',
                    'wxProcess',
                    'wxProcessEvent',
                    'wxProgressDialog',
                    'wxQuantize',
                    'wxQueryNewPaletteEvent',
                    'wxRadioBox',
                    'wxRadioButton',
                    'wxRealPoint',
                    'wxRect',
                    'wxRegion',
                    'wxRegionIterator',
                    'wxSVGFileDC',
                    'wxScreenDC',
                    'wxScrollBar',
                    'wxScrollEvent',
                    'wxScrollWinEvent',
                    'wxScrolledWindow',
                    'wxSetCursorEvent',
                    'wxShowEvent',
                    'wxSimpleHelpProvider',
                    'wxSingleChoiceDialog',
                    'wxSize',
                    'wxSizeEvent',
                    'wxSizer',
                    'wxSizerItem',
                    'wxSlider',
                    'wxSound',
                    'wxSpinButton',
                    'wxSpinCtrl',
                    'wxSpinEvent',
                    'wxSplitterEvent',
                    'wxSplitterWindow',
                    'wxStaticBitmap',
                    'wxStaticBox',
                    'wxStaticBoxSizer',
                    'wxStaticLine',
                    'wxStaticText',
                    'wxStatusBar',
                    'wxSysColourChangedEvent',
                    'wxSystemSettings',
                    'wxTextAttr',
                    'wxTextCtrl',
                    'wxTextDataObject',
                    'wxTextDropTarget',
                    'wxTextEntryDialog',
                    'wxTextValidator',
                    'wxTimer',
                    'wxTimerEvent',
                    'wxTimerEx',
                    'wxTimerRunner',
                    'wxTipWindow',
                    'wxToggleButton',
                    'wxToolBar',
                    'wxToolBarBase',
                    'wxToolTip',
                    'wxTopLevelWindow',
                    'wxTreeCtrl',
                    'wxTreeEvent',
                    'wxTreeItemData',
                    'wxTreeItemId',
                    'wxUpdateUIEvent',
                    'wxValidator',
                    'wxView',
                    'wxWindow',
                    'wxWindowCreateEvent',
                    'wxWindowDestroyEvent',
                    'wxWindowDC',
                    'wxWindowDisabler',
                    'wxXmlResourceHandler',
                    'wxcPrintEvent',
                    'wxcPrintout',
                    'wxcPrintoutHandler',
                    'wxcTreeItemData',
                    ]
    },
    'advanced': {
        'depends': ['_unsafe', 'base', 'core'],
        'classes': [
                    'ELJGridTable',
                    'wxCalculateLayoutEvent',
                    'wxCalendarCtrl',
                    'wxCalendarDateAttr',
                    'wxCalendarEvent',
                    'wxEditableListBox',
                    'wxGrid',
                    'wxGridCellAttr',
                    'wxGridCellAutoWrapStringRenderer',
                    'wxGridCellBoolEditor',
                    'wxGridCellBoolRenderer',
                    'wxGridCellChoiceEditor',
                    'wxGridCellCoordsArray',
                    'wxGridCellEditor',
                    'wxGridCellFloatEditor',
                    'wxGridCellFloatRenderer',
                    'wxGridCellNumberEditor',
                    'wxGridCellNumberRenderer',
                    'wxGridCellRenderer',
                    'wxGridCellStringRenderer',
                    'wxGridCellTextEditor',
                    'wxGridCellTextEnterEditor',
                    'wxGridCellWorker',
                    'wxGridEditorCreatedEvent',
                    'wxGridEvent',
                    'wxGridRangeSelectEvent',
                    'wxGridSizeEvent',
                    'wxGridTableBase',
                    'wxJoystick',
                    'wxLayoutAlgorithm',
                    'wxManagedPtr',
                    'wxQueryLayoutInfoEvent',
                    'wxSashEvent',
                    'wxSashLayoutWindow',
                    'wxSashWindow',
                    'wxSplashScreen',
                    'wxTaskBarIcon',
                    'wxTipProvider',
                    'wxWizard',
                    'wxWizardEvent',
                    'wxWizardPage',
                    'wxWizardPageSimple',
                    ]
    },
    'cb': {
        'depends': ['_unsafe', 'base', 'core', '_unavailable'],
        'classes': [
                    'cbAntiflickerPlugin',
                    'cbBarDragPlugin',
                    'cbBarHintsPlugin',
                    'cbBarInfo',
                    'cbBarSpy',
                    'cbCloseBox',
                    'cbCollapseBox',
                    'cbCommonPaneProperties',
                    'cbCustomizeBarEvent',
                    'cbCustomizeLayoutEvent',
                    'cbDimHandlerBase',
                    'cbDimInfo',
                    'cbDockBox',
                    'cbDockPane',
                    'cbDrawBarDecorEvent',
                    'cbDrawBarHandlesEvent',
                    'cbDrawHintRectEvent',
                    'cbDrawPaneBkGroundEvent',
                    'cbDrawPaneDecorEvent',
                    'cbDrawRowBkGroundEvent',
                    'cbDrawRowDecorEvent',
                    'cbDrawRowHandlesEvent',
                    'cbDynToolBarDimHandler',
                    'cbFinishDrawInAreaEvent',
                    'cbFloatedBarWindow',
                    'cbGCUpdatesMgr',
                    'cbHintAnimationPlugin',
                    'cbInsertBarEvent',
                    'cbLayoutRowEvent',
                    'cbLeftDClickEvent',
                    'cbLeftDownEvent',
                    'cbLeftUpEvent',
                    'cbMiniButton',
                    'cbMotionEvent',
                    'cbPaneDrawPlugin',
                    'cbPluginBase',
                    'cbPluginEvent',
                    'cbRemoveBarEvent',
                    'cbResizeBarEvent',
                    'cbResizeRowEvent',
                    'cbRightDownEvent',
                    'cbRightUpEvent',
                    'cbRowDragPlugin',
                    'cbRowInfo',
                    'cbRowLayoutPlugin',
                    'cbSimpleCustomizationPlugin',
                    'cbSimpleUpdatesMgr',
                    'cbSizeBarWndEvent',
                    'cbStartBarDraggingEvent',
                    'cbStartDrawInAreaEvent',
                    'cbUpdatesManagerBase',
                    ]
    },
    'gl': {
        'depends': ['_unsafe', 'base', 'core', 'advanced'],
        'classes': [
                    'wxGLCanvas',
                    'wxGLContext',
                    ]
    },
    'html': {
        'depends': ['_unsafe', 'base', 'core'],
        'classes': [
                    'wxHtmlCell',
                    'wxHtmlContainerCell',
                    'wxHtmlColourCell',
                    'wxHtmlDCRenderer',
                    'wxHtmlEasyPrinting',
                    'wxHtmlFilter',
                    'wxHtmlHelpController',
                    'wxHtmlHelpData',
                    'wxHtmlHelpFrame',
                    'wxHtmlLinkInfo',
                    'wxHtmlParser',
                    'wxHtmlPrintout',
                    'wxHtmlTag',
                    'wxHtmlTagHandler',
                    'wxHtmlTagsModule',
                    'wxHtmlWidgetCell',
                    'wxHtmlWinParser',
                    'wxHtmlWinTagHandler',
                    'wxHtmlWindow',
                    'wxcHtmlEvent',
                    'wxcHtmlWindow',
        ]
    },
    'media': {
        'depends': ['_unsafe', 'base', 'core'],
        'classes': [
                    'wxMediaCtrl',
                    'wxMediaEvent',
                    ]
    },
    'net': {
        'depends': ['base', 'core'],
        'classes': [
                    'wxFTP',
                    'wxHTTP',
                    'wxIPV4address',
                    'wxProtocol',
                    'wxSockAddress',
                    'wxSocketBase',
                    'wxSocketClient',
                    'wxSocketEvent',
                    'wxSocketInputStream',
                    'wxSocketOutputStream',
                    'wxSocketServer',
                    'wxURL',
                    ]
    },
    'odbc': {
        'depends': ['base'],
        'classes': [
                    'wxDatabase',
                    'wxDb',
                    'wxDbColDef',
                    'wxDbColFor',
                    'wxDbColInf',
                    'wxDbConnectInf',
                    'wxDbInf',
                    'wxDbSqlTypeInfo',
                    'wxDbTable',
                    'wxDbTableInfo',
                    'wxQueryCol',
                    'wxQueryField',
                    'wxRecordSet',
                    'wxTablesInUse',
                    ]
    },
    'propertygrid' : {
        'depends': ['_unsafe', 'base', 'core', 'advanced'],
        'classes': [
                    'wxBoolProperty',
                    'wxDateProperty',
                    'wxFileProperty',
                    'wxFloatProperty',
                    'wxIntProperty',
                    'wxPGProperty',
                    'wxPropertyCategory',
                    'wxPropertyGrid',
                    'wxPropertyGridEvent',
                    'wxStringProperty',
                    ]
    },
    'stc': {
        'depends': ['_unsafe', 'base', 'core', 'advanced'],
        'classes': [
                    'wxSTCDoc',
                    'wxStyledTextCtrl',
                    'wxStyledTextEvent',
                    ]
    },
    'xrc': {
        'depends': ['_unsafe', 'base', 'core', 'advanced', 'html', 'stc'],
        'classes': [
                    'wxXmlResource',
                    ]
    },
    '_unavailable': {
        'depends': ['base', 'core'],
        'classes': [
                    'ELJMessageParameters',
                    'ELJPlotCurve',
                    'wxDynToolInfo',
                    'wxDynamicSashWindow',
                    'wxDynamicToolBar',
                    'wxExpr',
                    'wxExprDatabase',
                    'wxFrameLayout',
                    'wxGauge95',
                    'wxGaugeMSW',
                    'wxHashMap',
                    'wxLEDNumberCtrl',
                    'wxMBConvFile',
                    'wxMultiCellCanvas',
                    'wxMultiCellItemHandle',
                    'wxMultiCellSizer',
                    'wxNewBitmapButton',
                    'wxPlotCurve',
                    'wxPlotEvent',
                    'wxPlotOnOffCurve',
                    'wxPlotWindow',
                    'wxRemotelyScrolledTreeCtrl',
                    'wxSlider95',
                    'wxSliderMSW',
                    'wxSplitterScrolledWindow',
                    'wxStreamToTextRedirector',
                    'wxTabCtrl',
                    'wxTabEvent',
                    'wxThinSplitterWindow',
                    'wxTimerBase',
                    'wxToolLayoutItem',
                    'wxToolWindow',
                    'wxTreeCompanionWindow',
                    'wxTreeLayout',
                    'wxTreeLayoutStored',
                    ]
    },
}


def main():
    parser = Parser()
    parser.parse_files(sys.argv[1:])
    WrapperGenerator(parser).generate()
    print('\nDone.', file=sys.stderr)


def progress():
    if PY3:
        print('.', file=sys.stderr, end='', flush=True)
    else:
        print('.', file=sys.stderr, end='')


class WrapperGenerator(object):
    def __init__(self, parser):
        self.__parser = parser
        self._indent = 0
        self.__file = sys.stdout

    def generate(self):
        print('\nGenerating code ', file=sys.stderr, end='')
        classes = list(self.__parser.classes)
        
        for name, module in PY3 and modules.items() or modules.iteritems():
            with open('src/%s.rs' % name, 'w') as f:
                self.__file = f
                self.println('use std::libc::*;')
                if name == 'base':
                    self.println('use std::str;')
                for m in module['depends']:
                    self.println('use %s::*;' % m)
                self.println()
                if name == 'base':
                    self.println('''\
#[link(name="wxc")]
extern {
    fn wxString_CreateUTF8(buffer: *mut c_void) -> *mut c_void;
    fn wxString_GetUtf8(wxs: *mut c_void) -> *mut c_void;
    fn wxCharBuffer_DataUtf8(wxcb: *mut c_void) -> *c_char;
    fn wxCharBuffer_Delete(wxcb: *mut c_void);
}

pub fn strToString(s: &str) -> String {
    unsafe {
        s.to_c_str().with_ref(|c_str| {
            String::from(wxString_CreateUTF8(c_str as *mut c_void))
        })
    }
}

pub struct String { ptr: *mut c_void }
impl Drop for String {
    fn drop(&mut self) {
        unsafe { wxString_Delete(self.ptr); }
    }
}
impl String {
    pub fn ptr(&self) -> *mut c_void { self.ptr }
    pub fn from(ptr: *mut c_void) -> String { String { ptr: ptr } }
    pub fn to_str(&self) -> ~str {
        unsafe {
            let charBuffer = wxString_GetUtf8(self.ptr);
            let utf8 = wxCharBuffer_DataUtf8(charBuffer);
            wxCharBuffer_Delete(charBuffer);
            str::raw::from_c_str(utf8)
        }
    }
}
''')
                tmpClasses = list(classes)
                for clazz in classes:
                    if clazz.name not in module['classes']:
                        continue
                    progress()
                    self._print_class(clazz)
                    tmpClasses.remove(clazz)
                classes = tmpClasses

        assert(len(classes) == 0)
    
    def _print_class(self, clazz):
        struct_name = '%s' % clazz.struct_name
        if clazz.name.startswith('wx') and not clazz.name.startswith('wxc'):
            self.println("/// Wraps the wxWidgets' [%s](%s) class." % (clazz.name, clazz.link))
            unprefixed = clazz.name[len('wx'):]
            if self.__parser.class_for_name('ELJ' + unprefixed):
                self.println("/// Rather use the wxRust-specific [Rust%s](struct.Rust%s.html) class." % (unprefixed, unprefixed))
            if self.__parser.class_for_name('wxc' + unprefixed):
                self.println("/// Rather use the wxRust-specific [C%s](struct.C%s.html) class." % (unprefixed, unprefixed))
        if (clazz.name.startswith('ELJ') or clazz.name.startswith('wxc')) and clazz.has_base:
            base = self.__parser.class_for_name(clazz.base)
            self.println("/// The wxRust-specific derived class of [%s](%s)." % (base.name, base.link))
        self.println('pub struct %s { ptr: *mut c_void }' % struct_name)
        for trait in clazz.inheritance:
            body = ''
            if trait in self.__parser.root_classes:
                body = ' fn ptr(&self) -> *mut c_void { self.ptr } '
            self.println('impl %s for %s {%s}' % (trait_name(trait), struct_name, body))
        self.println()
    
        # static methods go to struct impl
        self.println('impl %s {' % struct_name)
        with self.indent():
            self.println('pub fn from(ptr: *mut c_void) -> %s { %s { ptr: ptr } }' % (struct_name, struct_name))
            self.println('pub fn null() -> %s { %s::from(0 as *mut c_void) }' % (struct_name, struct_name))
            self.println()
            for method in clazz.static_methods:
                method.trait_fn(self, clazz.name)
        self.println('}')
        
        # instance methods go to trait's default impl
        base = clazz.has_base and ' : %s' % trait_name(clazz.base) or ''
        self.println()
        if clazz.name.startswith('wx') and not clazz.name.startswith('wxc'):
            self.println("/// Methods of the wxWidgets' [%s](%s) class." % (clazz.name, clazz.link))
        if (clazz.name.startswith('ELJ') or clazz.name.startswith('wxc')) and clazz.has_base:
            base2 = self.__parser.class_for_name(clazz.base)
            self.println("/// Methods of the wxRust-specific derived class of [%s](%s)." % (base2.name, base2.link))
        self.println('pub trait %s%s {' % (clazz.trait_name, base))
        with self.indent():
            if clazz.name in self.__parser.root_classes:
                self.println('fn ptr(&self) -> *mut c_void;')
                self.println()
            for method in clazz.methods:
                method.trait_fn(self, clazz.name)
        self.println('}')
        self.println()

    def println(self, text=''):
        lines = text.split('\n')
        for line in lines:
            line = '%s%s' % (''+(' ' * 4 * self._indent), line)
            print(line, file=self.__file)

    def indent(self):
        class Indent:
            def __init__(self, target):
                self.__target = target
                pass
            
            def __enter__(self):
                self.__target._indent += 1
                return self
            
            def __exit__(self, type, value, traceback):
                self.__target._indent -= 1
        return Indent(self)


class Parser(object):
    def __init__(self):
        self.__classes   = []
        self.__functions = []
        self.__root_classes = set()
    
    def parse_files(self, files):
        print('Parsing files', file=sys.stderr, end='')
        for file in files:
            progress()
            for line in Preprocessor().preprocess(file):
                self._parse_line(line.strip())
        print('\nConstructing class information ', file=sys.stderr, end='')
        for clazz in self.__classes:
            progress()
            self.__root_classes.add(clazz.inheritance[-1])
            for f in self.__functions:
                if f.name in missing_functions:
                    continue
                clazz.add_if_member(f)
        print('\nRemoving duplicated methods ', file=sys.stderr, end='')
        for clazz in self.__classes:
            progress()
            clazz.remove_methods_in_base()
    
    @property
    def classes(self):
        return self.__classes
    
    def class_for_name(self, name):
        for clazz in self.__classes:
            if clazz.name == name:
                return clazz
        return None

    @property
    def root_classes(self):
        return self.__root_classes

    def _parse_line(self, line):
        if not len(line):
            return
        
        class_or_function = LineParser(line).parse()
        if 'TClassDef' in line:
            # class def
            clazz = Class(self, class_or_function)
            if clazz.name == 'wxString':
                return # ignore
            # linear search isn't fast.
            for clazz2 in self.classes:
                if clazz.name == clazz2.name:
                    return
            self.__classes.append(clazz)
        else:
            # func def
            self.__functions.append(Function(class_or_function))


class LineParser(object):
    def __init__(self, line):
        self.__line = line
        self.__stack = []
        self.__node = []
    
    def parse(self):
        lexer = (t.group(0) for t in re.finditer(re.compile(r'\s+|\*|\(|,|\)|;|[^\s*(,);]+'), self.__line))
        for token in lexer:
            if not token.strip() or token == ';':
                # ignore separators
                continue
            if token == '(':
                self._function_start()
                continue
            if token == ',':
                self._next_arg()
                continue
            if token == ')':
                self._function_end()
                # Handles function style argument macros here
                self._expand_if_macro()
                continue
            if token == '*':
                self._pointer()
                continue
            self.__node.append(token)
            continue
        return self.__node
    
    # a b(c); parsed to below:
    # [ - function declaration
    #    a - return type
    #    [ - function
    #        b
    #        [ - arg list
    #            [c] - arg pair
    #        ]
    #    ]
    # ]
    #
    # TClass(F) a(TSelf(b), int c); parsed to below:
    # [
    #    [ - function (style macro) - return type
    #        TClass - name
    #        [ - arg list
    #            [F] - arg pair (arg type only, the name is optional)
    #        ]
    #    ]
    #    [ - function
    #        a - name
    #        [ - arg list
    #            [ - arg pair
    #                [
    #                    TSelf - function (style macro) - arg type
    #                    [ - arg list
    #                        [b] - arg pair
    #                    ]
    #                    - arg name is optional
    #                ]
    #            ]
    #            [ - arg pair
    #                int - arg type
    #                c   - arg name
    #            ]
    #        ]
    #    ]
    # ]
    
    def _function_start(self):
        funcname = self.__node.pop()
        self.__stack.append(self.__node)
        self.__node = [funcname]
        self.__stack.append(self.__node)
        self.__node = [] # arg list start
        self.__stack.append(self.__node)
        self.__node = [] # type-(optional) name pair
    
    def _next_arg(self):
        self.__stack[-1].append(self.__node)
        self.__node = [] # new-arg
    
    def _function_end(self):
        if len(self.__node):
            self.__stack[-1].append(self.__node)
        self.__node = self.__stack.pop() # end-arg
        self.__stack[-1].append(self.__node)
        self.__node = self.__stack.pop() # end-arg-list
        self.__stack[-1].append(self.__node)
        self.__node = self.__stack.pop() # end-function

    def _expand_if_macro(self):
        macro_name = self.__node[-1][0]
        if macro_name in ['TArrayString',
                          'TByteString', 'TByteStringLazy',
                          'TColorRGB',
                          'TPoint', 'TPointDouble', 'TPointLong', 'TPointOut', 'TPointOutDouble', 'TPointOutVoid',
                          'TRect',  'TRectDouble',                             'TRectOutDouble',  'TRectOutVoid',
                          'TSize',  'TSizeDouble',                'TSizeOut',  'TSizeOutDouble',  'TSizeOutVoid',
                          'TVector']:
            macro = globals()[macro_name]
            macro_args = [arg[0] for arg in self.__node.pop()[1]]
            macro_result = macro(macro_args)
            for result_node in macro_result[:-1]:
                self.__stack[-1].append(result_node)
            # We assume the last node read until here
            for item in macro_result[-1]:
                self.__node.append(item)
    
    def _pointer(self):
        self.__node.append(['*', self.__node.pop()])


# Function style arg macros
def TArrayString(args):
    return [['int', args[0]],
            [['*', ['*', 'char']], args[1]]]
def TByteString(args):
    return [[['*', ['*', 'char']], args[0]],
            ['int', args[1]]]
def TByteStringLazy(args):
    return [[['*', ['*', 'char']], args[0]],
            ['int', args[1]]]
TByteStringLazy = TByteString
def TColorRGB(args):
    return [['uint8_t', args[0]],
            ['uint8_t', args[1]],
            ['uint8_t', args[2]]];
def TPoint(args, T='int'):
    return [[T, args[0]],
            [T, args[1]]];
def TPointDouble(args):
    return TPoint(args, T='double')
def TPointLong(args):
    return TPoint(args, T='long')
def TPointOut(args):
    return TPoint(args, T=['*', 'int'])
def TPointOutDouble(args):
    return TPoint(args, T=['*', 'double'])
def TPointOutVoid(args):
    return TPoint(args, T=['*', 'void'])
def TRect(args, T='int'):
    return [[T, args[0]],
            [T, args[1]],
            [T, args[2]],
            [T, args[3]]]
def TRectDouble(args):
    return TRect(args, T='double')
def TRectOutDouble(args):
    return TRect(args, T=['*', 'double'])
def TRectOutVoid(args):
    return TRect(args, T=['*', 'void'])
TSize = TPoint
def TSizeDouble(args):
    return TSize(args, T='double')
def TSizeOut(args):
    return TSize(args, T=['*', 'int'])
def TSizeOutDouble(args):
    return TSize(args, T=['*', 'double'])
def TSizeOutVoid(args):
    return TSize(args, T=['*', 'void'])
TVector = TPoint


class Preprocessor(object):
    def __init__(self):
        pass
    
    def preprocess(self, file):
        for line in Preprocessor._normalize(self._call_cpp(file)).splitlines():
            line = line.strip()
            if not len(line) or line.startswith('#'):
                continue
            yield line

    def _call_cpp(self, file):
        cppflags = Popen(['wx-config', '--cppflags'], stdout=PIPE).communicate()[0].split()
        cmdline = ['cpp', '-DWXC_TYPES_H'] + cppflags + ['-I/Users/kenz/src/wxRust/wxHaskell/wxc/src/include', file]
        result = Popen(cmdline, stdout=PIPE).communicate()[0]
        return PY3 and result.decode('unicode_escape') or result

    @staticmethod
    def _normalize(text):
        text = re.sub(r'^\s*\n', '', text)
        text = re.sub(r'\n+', '\n', text)
        text = re.sub(r'\\\n\s+', '', text)
        return text


class Class(object):
    def __init__(self, parser, node):
        assert len(node) == 1
        assert len(node[0]) > 1
        self.__parser = parser
        self.__node = node
        self.__methods = []

    @property
    def name(self):
        return self.__node[0][1][0][0]

    @property
    def link(self):
        words = []
        word = ''
        case = self.name[0].islower()
        for c in self.name:
            if c.islower() == case or (len(word) == 1 and word.isupper()):
                word += c
            else:
                words.append(word.lower())
                word = c
            case = c.islower()
        if len(word):
            words.append(word.lower())
        return 'http://docs.wxwidgets.org/3.0/class%s.html' % '_'.join(words)

    @property
    def struct_name(self):
        return struct_name(self.name)

    @property
    def trait_name(self):
        return trait_name(self.name)

    @property
    def has_base(self):
        return self.__node[0][0] == 'TClassDefExtend'

    @property
    def base(self):
        assert self.has_base
        return self.__node[0][1][1][0]
    
    @property
    def inheritance(self):
        list = [self.name]
        if self.has_base:
            list += self.__parser.class_for_name(self.base).inheritance
        return list

    @property
    def static_methods(self):
        return (m for m in self.__methods if m.is_static)

    @property
    def methods(self):
        return (m for m in self.__methods if not m.is_static)

    def add_if_member(self, f):
        if f.name.startswith('%s_' % self.name):
            self.__methods.append(f)

    def remove_methods_in_base(self):
        removes = set()
        for mym in self.methods:
            for base_class in self.inheritance[1:]:
                base = self.__parser.class_for_name(base_class)
                for m in base.methods:
                    if m.method_name(base.name) == mym.method_name(self.name):
                        removes.add(mym)
        for m in removes:
            self.__methods.remove(m)


def struct_name(name):
    if name.startswith('ELJ') or name.startswith('wxc'):
        return 'Rust' + name[3:]
    if name.startswith('wx'):
        return name[len('wx'):]
    if name.startswith('cb'):
        return 'Cb' + name[len('cb'):]
    return name

def trait_name(name):
    return struct_name(name) + 'Methods'


class Function(object):
    def __init__(self, node):
        assert len(node)    > 1
        assert len(node[1]) > 1
        self.__node = node
        self.__return_type = Type(self.__node[0])
        self.__args = [Arg(arg, i) for i, arg in enumerate(self.__node[1][1])]

    @property
    def is_static(self):
        if len(self.__args) < 1:
            return True
        return not self.__args[0].type.is_self

    @property
    def name(self):
        return self.__node[1][0]
    
    @property
    def _type_params(self):
        type_params = []
        number = 0
        for a in self.__args:
            if a.type.is_class and not a.type.is_self:
                letter = chr(ord('T') + number)
                a.type_param = '&%s' % letter
                type_params.append('%s: %s' % (letter, a.type.trait_name))
                number += 1
        if len(type_params):
            return '<%s>' % ', '.join(type_params)
        return ''

    def trait_fn(self, gen, classname):
        modifier = self.is_static and 'pub ' or ''
        gen.println('%sfn %s%s(%s)%s {' % (modifier,
                                         self.method_name(classname),
                                         self._type_params,
                                         self._decl_args,
                                         self._returns))
        with gen.indent():
            body = '%s(%s)' % (self.name, self._calling_args)
            if self.__return_type.is_string:
                body = 'String::from(%s).to_str()' % (body)
            if self.__return_type.is_self or self.__return_type.is_class:
                body = '%s::from(%s)' % (self.__return_type.struct_name, body)
            gen.println('%sunsafe { %s }' % (self._strings, body))
        gen.println('}')

    @property
    def _strings(self):
        s = ''
        for a in self.__args:
            if a.type.is_string:
                s += 'let %s = strToString(%s);\n' % (a.name, a.name)
        return s
    
    def method_name(self, classname):
        _name = self.name[len(classname)+1:]
        _name = _name[0].lower() + _name[1:]
        if _name in ['break', 'yield']:
            _name += '_'
        if _name.startswith('create'):
            return 'new' + _name[len('create'):]
        return _name

    @property
    def _decl_args(self):
        return ', '.join((str(a) for a in self.__args))
    
    @property
    def _calling_args(self):
        return ', '.join((a.calling_arg for a in self.__args))
    
    @property
    def _returns(self):
        if self.__return_type.is_void:
            return ''
        if self.__return_type.is_string:
            return ' -> ~str'
        if self.__return_type.is_class:
            return ' -> %s' % self.__return_type.struct_name
        return ' -> %s' % self.__return_type


class Arg(object):
    def __init__(self, node, index):
        assert len(node) > 0
        self.__node = node
        self.__index = index
        self.__type = Type(self.__node[0])
        self.type_param = None

    @property
    def _is_self(self):
        # work around second TSelf(T) arguments...
        return self.__node[0][0] == 'TSelf' and self.__index == 0
    
    @property
    def name(self):
        if len(self.__node) == 1:
            return '_arg%s' % self.__index
        else:
            if self._is_self:
                return 'self'
            _name = self.__node[1]
            if _name in ['box', 'fn', 'ref', 'self', 'type', 'use']:
                _name += '_'
            return _name

    @property
    def type(self):
        return self.__type

    @property
    def calling_arg(self):
        if self._is_self:
            return 'self.ptr()'
        if self.type.is_string:
            return '%s.ptr()' % self.name
        elif self.type.is_class:
            return '%s.ptr()' % self.name
        else:
            return self.name

    def __str__(self):
        if self.type_param:
            return '%s: %s' % (self.name, self.type_param)
        macro_name = self.__node[0][0]
        if macro_name in ['TArrayObjectOutVoid']:
            macro = globals()[macro_name]
            macro_args = tuple((x[0] for x in self.__node[0][1]))
            return '%s: %s' % (self.name, macro(macro_args))
        if self._is_self:
            return '&self'
        return '%s: %s' % (self.name, self.type)


# Function style type macros
def TArrayObjectOutVoid(args):
    return '*mut c_void'#'~[%s]' % args # it would be **c_void ?


class Type(object):
    def __init__(self, node):
        self.__node = node

    @property
    def _head(self):
        if self._is_complex:
            return self.__node[0]
        return self.__node

    @property
    def _is_complex(self):
        return not isinstance(self.__node, PY3 and str or basestring)
    
    @property
    def is_self(self):
        return self._is_complex and self._head == 'TSelf'
    
    @property
    def _inner(self):
        if self._is_ptr:
            return self.__node[1]
        if self._is_complex:
            return self.__node[1][0][0]
        return self.__node
    
    @property
    def is_void(self):
        return self._head == 'void'
    
    @property
    def is_string(self):
        return self._head.startswith('TClass') and self._inner == 'wxString'
    
    @property
    def is_class(self):
        return self._is_complex and self._head in ['TClass', 'TClassRef', 'TSelf'] and not self.is_string
    
    @property
    def _is_ptr(self):
        return self._is_complex and self._head == '*'
    
    @property
    def struct_name(self):
        assert self.is_class
        return struct_name(self._inner)
    
    @property
    def trait_name(self):
        assert self.is_class
        return trait_name(self._inner)
    
    def __str__(self):
        if self.is_string:
            return '&str'
        if self.is_self or self.is_class:
            return '&%s' % self.trait_name
        if self._is_ptr:
            t = Type(self._inner)
            if t.is_class:
                return '*mut *mut c_void'
            # work around _unsafe.rs bug
            if t._is_ptr and t._inner == 'TChar':
                return '*mut *mut c_char'
            return '*mut %s' % t
        s = str(self.__node)
        if s in type_mapping:
            return type_mapping[s]
        return s


# Other type mappings
type_mapping = {
    # header type          # rust type
    'TArrayIntOutVoid':    '*mut c_void',
    'TArrayIntPtrOutVoid': '*mut c_void',
    'TArrayLen':           'c_int',
    'TArrayStringOutVoid': '*mut c_void',
    'TBool':               'c_int',
    'TBoolInt':            'c_int',
    'TByteStringLazyOut':  '*mut c_char',
    'TByteStringLen':      'c_int',
    'TByteStringOut':      '*mut c_char',
    'TChar':               'int8_t',
    'TClosureFun':         '*mut c_void',#'*c_void',
    'TInt64':              'i64',
    'TIntPtr':             'intptr_t',
    'TString':             '*mut int8_t',
    'TStringLen':          'c_int',
    'TStringOut':          '*wchar_t',#'**wchar_t',
    'TStringVoid':         '*mut c_void',
    'TUInt':               'uint32_t',
    'TUInt8':              'uint8_t',
    'char':                'c_char',
    'double':              'c_double',
    'float':               'c_float',
    'int':                 'c_int',
    'long':                'c_long',
    'void':                'c_void',
}


# These are in wxc.h but not linked yet. skip them.
missing_functions = ['ELJClient_Create',
                     'ELJClient_Delete',
                     'ELJClient_MakeConnection',
                     'ELJCommand_CanUndo',
                     'ELJCommand_Create',
                     'ELJCommand_Delete',
                     'ELJCommand_GetName',
                     'ELJConnection_Advise',
                     'ELJConnection_Compress',
                     'ELJConnection_Create',
                     'ELJConnection_CreateDefault',
                     'ELJConnection_Delete',
                     'ELJConnection_Disconnect',
                     'ELJConnection_Execute',
                     'ELJConnection_Poke',
                     'ELJConnection_Request',
                     'ELJConnection_SetOnAdvise',
                     'ELJConnection_SetOnDisconnect',
                     'ELJConnection_SetOnExecute',
                     'ELJConnection_SetOnPoke',
                     'ELJConnection_SetOnRequest',
                     'ELJConnection_SetOnStartAdvise',
                     'ELJConnection_SetOnStopAdvise',
                     'ELJConnection_StartAdvise',
                     'ELJConnection_StopAdvise',
                     'ELJPlotCurve_Create',
                     'ELJPlotCurve_Delete',
                     'ELJPlotCurve_GetEndY',
                     'ELJPlotCurve_GetOffsetY',
                     'ELJPlotCurve_GetStartY',
                     'ELJPlotCurve_SetEndY',
                     'ELJPlotCurve_SetOffsetY',
                     'ELJPlotCurve_SetPenNormal',
                     'ELJPlotCurve_SetPenSelected',
                     'ELJPlotCurve_SetStartY',
                     'ELJServer_Create',
                     'ELJServer_Delete',
                     'ELJServer_Initialize',
                     'cbAntiflickerPlugin_Create',
                     'cbAntiflickerPlugin_CreateDefault',
                     'cbAntiflickerPlugin_Delete',
                     'cbBarDragPlugin_Create',
                     'cbBarDragPlugin_CreateDefault',
                     'cbBarDragPlugin_Delete',
                     'cbBarHintsPlugin_Create',
                     'cbBarHintsPlugin_CreateDefault',
                     'cbBarHintsPlugin_Delete',
                     'cbBarHintsPlugin_SetGrooveCount',
                     'cbBarInfo_Create',
                     'cbBarInfo_Delete',
                     'cbBarInfo_IsExpanded',
                     'cbBarInfo_IsFixed',
                     'cbBarSpy_Create',
                     'cbBarSpy_CreateDefault',
                     'cbBarSpy_Delete',
                     'cbBarSpy_ProcessEvent',
                     'cbBarSpy_SetBarWindow',
                     'cbCloseBox_Create',
                     'cbCollapseBox_Create',
                     'cbCommonPaneProperties_Assign',
                     'cbCommonPaneProperties_BarCollapseIconsOn',
                     'cbCommonPaneProperties_BarDragHintsOn',
                     'cbCommonPaneProperties_BarFloatingOn',
                     'cbCommonPaneProperties_ColProportionsOn',
                     'cbCommonPaneProperties_CreateDefault',
                     'cbCommonPaneProperties_Delete',
                     'cbCommonPaneProperties_ExactDockPredictionOn',
                     'cbCommonPaneProperties_MinCBarDim',
                     'cbCommonPaneProperties_NonDestructFrictionOn',
                     'cbCommonPaneProperties_OutOfPaneDragOn',
                     'cbCommonPaneProperties_RealTimeUpdatesOn',
                     'cbCommonPaneProperties_ResizeHandleSize',
                     'cbCommonPaneProperties_RowProportionsOn',
                     'cbCommonPaneProperties_SetBarCollapseIconsOn',
                     'cbCommonPaneProperties_SetBarDragHintsOn',
                     'cbCommonPaneProperties_SetBarFloatingOn',
                     'cbCommonPaneProperties_SetColProportionsOn',
                     'cbCommonPaneProperties_SetExactDockPredictionOn',
                     'cbCommonPaneProperties_SetMinCBarDim',
                     'cbCommonPaneProperties_SetNonDestructFrictionOn',
                     'cbCommonPaneProperties_SetOutOfPaneDragOn',
                     'cbCommonPaneProperties_SetRealTimeUpdatesOn',
                     'cbCommonPaneProperties_SetResizeHandleSize',
                     'cbCommonPaneProperties_SetRowProportionsOn',
                     'cbCommonPaneProperties_SetShow3DPaneBorderOn',
                     'cbCommonPaneProperties_Show3DPaneBorderOn',
                     'cbCustomizeBarEvent_Bar',
                     'cbCustomizeBarEvent_ClickPos',
                     'cbCustomizeLayoutEvent_ClickPos',
                     'cbDimInfo_Assign',
                     'cbDimInfo_Create',
                     'cbDimInfo_CreateDefault',
                     'cbDimInfo_CreateWithHandler',
                     'cbDimInfo_CreateWithInfo',
                     'cbDimInfo_Delete',
                     'cbDimInfo_GetDimHandler',
                     'cbDockBox_Create',
                     'cbDockPane_BarPresent',
                     'cbDockPane_Create',
                     'cbDockPane_CreateDefault',
                     'cbDockPane_Delete',
                     'cbDockPane_GetAlignment',
                     'cbDockPane_GetBarInfoByWindow',
                     'cbDockPane_GetBarResizeRange',
                     'cbDockPane_GetDockingState',
                     'cbDockPane_GetFirstRow',
                     'cbDockPane_GetPaneHeight',
                     'cbDockPane_GetRealRect',
                     'cbDockPane_GetRowList',
                     'cbDockPane_GetRowResizeRange',
                     'cbDockPane_HitTestPaneItems',
                     'cbDockPane_InsertBarByCoord',
                     'cbDockPane_InsertBarByInfo',
                     'cbDockPane_InsertBarToRow',
                     'cbDockPane_InsertRow',
                     'cbDockPane_IsHorizontal',
                     'cbDockPane_MatchesMask',
                     'cbDockPane_RemoveBar',
                     'cbDockPane_RemoveRow',
                     'cbDockPane_SetBoundsInParent',
                     'cbDockPane_SetMargins',
                     'cbDockPane_SetPaneWidth',
                     'cbDrawBarDecorEvent_Bar',
                     'cbDrawBarDecorEvent_BoundsInParent',
                     'cbDrawBarDecorEvent_Dc',
                     'cbDrawBarHandlesEvent_Bar',
                     'cbDrawBarHandlesEvent_Dc',
                     'cbDrawHintRectEvent_EraseRect',
                     'cbDrawHintRectEvent_IsInClient',
                     'cbDrawHintRectEvent_LastTime',
                     'cbDrawHintRectEvent_Rect',
                     'cbDrawPaneBkGroundEvent_Dc',
                     'cbDrawPaneDecorEvent_Dc',
                     'cbDrawRowBkGroundEvent_Dc',
                     'cbDrawRowBkGroundEvent_Row',
                     'cbDrawRowDecorEvent_Dc',
                     'cbDrawRowDecorEvent_Row',
                     'cbDrawRowHandlesEvent_Dc',
                     'cbDrawRowHandlesEvent_Row',
                     'cbDynToolBarDimHandler_Create',
                     'cbDynToolBarDimHandler_Delete',
                     'cbFinishDrawInAreaEvent_Area',
                     'cbFloatedBarWindow_Create',
                     'cbFloatedBarWindow_GetBar',
                     'cbFloatedBarWindow_PositionFloatedWnd',
                     'cbFloatedBarWindow_SetBar',
                     'cbFloatedBarWindow_SetLayout',
                     'cbGCUpdatesMgr_Create',
                     'cbGCUpdatesMgr_CreateDefault',
                     'cbGCUpdatesMgr_Delete',
                     'cbGCUpdatesMgr_UpdateNow',
                     'cbHintAnimationPlugin_Create',
                     'cbHintAnimationPlugin_CreateDefault',
                     'cbHintAnimationPlugin_Delete',
                     'cbInsertBarEvent_Bar',
                     'cbInsertBarEvent_Row',
                     'cbLayoutRowEvent_Row',
                     'cbLeftDClickEvent_Pos',
                     'cbLeftDownEvent_Pos',
                     'cbLeftUpEvent_Pos',
                     'cbMiniButton_Create',
                     'cbMiniButton_Delete',
                     'cbMiniButton_Dim',
                     'cbMiniButton_DragStarted',
                     'cbMiniButton_Enable',
                     'cbMiniButton_Enabled',
                     'cbMiniButton_HitTest',
                     'cbMiniButton_IsPressed',
                     'cbMiniButton_Layout',
                     'cbMiniButton_Pane',
                     'cbMiniButton_Plugin',
                     'cbMiniButton_Pos',
                     'cbMiniButton_Pressed',
                     'cbMiniButton_Refresh',
                     'cbMiniButton_Reset',
                     'cbMiniButton_SetPos',
                     'cbMiniButton_Visible',
                     'cbMiniButton_WasClicked',
                     'cbMiniButton_Wnd',
                     'cbMotionEvent_Pos',
                     'cbPaneDrawPlugin_Create',
                     'cbPaneDrawPlugin_CreateDefault',
                     'cbPaneDrawPlugin_Delete',
                     'cbPluginBase_Delete',
                     'cbPluginBase_GetPaneMask',
                     'cbPluginBase_IsReady',
                     'cbPluginBase_Plugin',
                     'cbPluginBase_ProcessEvent',
                     'cbPluginEvent_Pane',
                     'cbRemoveBarEvent_Bar',
                     'cbResizeBarEvent_Bar',
                     'cbResizeBarEvent_Row',
                     'cbResizeRowEvent_ForUpperHandle',
                     'cbResizeRowEvent_HandleOfs',
                     'cbResizeRowEvent_Row',
                     'cbRightDownEvent_Pos',
                     'cbRightUpEvent_Pos',
                     'cbRowDragPlugin_Create',
                     'cbRowDragPlugin_CreateDefault',
                     'cbRowDragPlugin_Delete',
                     'cbRowInfo_Create',
                     'cbRowInfo_Delete',
                     'cbRowInfo_GetFirstBar',
                     'cbRowLayoutPlugin_Create',
                     'cbRowLayoutPlugin_CreateDefault',
                     'cbRowLayoutPlugin_Delete',
                     'cbSimpleCustomizationPlugin_Create',
                     'cbSimpleCustomizationPlugin_CreateDefault',
                     'cbSimpleCustomizationPlugin_Delete',
                     'cbSizeBarWndEvent_Bar',
                     'cbSizeBarWndEvent_BoundsInParent',
                     'cbStartBarDraggingEvent_Bar',
                     'cbStartBarDraggingEvent_Pos',
                     'cbStartDrawInAreaEvent_Area',
                     'wxCommandProcessor_CanRedo',
                     'wxCommandProcessor_CanUndo',
                     'wxCommandProcessor_ClearCommands',
                     'wxCommandProcessor_Delete',
                     'wxCommandProcessor_GetCommands',
                     'wxCommandProcessor_GetEditMenu',
                     'wxCommandProcessor_GetMaxCommands',
                     'wxCommandProcessor_Initialize',
                     'wxCommandProcessor_Redo',
                     'wxCommandProcessor_SetEditMenu',
                     'wxCommandProcessor_SetMenuStrings',
                     'wxCommandProcessor_Submit',
                     'wxCommandProcessor_Undo',
                     'wxCommandProcessor_wxCommandProcessor',
                     'wxCondition_Broadcast',
                     'wxCondition_Create',
                     'wxCondition_Delete',
                     'wxCondition_Signal',
                     'wxCondition_Wait',
                     'wxCondition_WaitFor',
                     'wxCriticalSection_Create',
                     'wxCriticalSection_Delete',
                     'wxCriticalSection_Enter',
                     'wxCriticalSection_Leave',
                     'wxDateTime_IsGregorianDate',
                     'wxDialUpEvent_IsConnectedEvent',
                     'wxDialUpEvent_IsOwnEvent',
                     'wxDialUpManager_CancelDialing',
                     'wxDialUpManager_Create',
                     'wxDialUpManager_Delete',
                     'wxDialUpManager_Dial',
                     'wxDialUpManager_DisableAutoCheckOnlineStatus',
                     'wxDialUpManager_EnableAutoCheckOnlineStatus',
                     'wxDialUpManager_GetISPNames',
                     'wxDialUpManager_HangUp',
                     'wxDialUpManager_IsAlwaysOnline',
                     'wxDialUpManager_IsDialing',
                     'wxDialUpManager_IsOk',
                     'wxDialUpManager_IsOnline',
                     'wxDialUpManager_SetConnectCommand',
                     'wxDialUpManager_SetOnlineStatus',
                     'wxDialUpManager_SetWellKnownHost',
                     'wxDynToolInfo_Index',
                     'wxDynToolInfo_RealSize',
                     'wxDynToolInfo_pToolWnd',
                     'wxDynamicSashWindow_Create',
                     'wxDynamicSashWindow_Delete',
                     'wxDynamicSashWindow_GetHScrollBar',
                     'wxDynamicSashWindow_GetVScrollBar',
                     'wxDynamicToolBar_AddSeparator',
                     'wxDynamicToolBar_AddTool',
                     'wxDynamicToolBar_AddToolBitmap',
                     'wxDynamicToolBar_AddToolImage',
                     'wxDynamicToolBar_AddToolLabel',
                     'wxDynamicToolBar_Create',
                     'wxDynamicToolBar_CreateDefault',
                     'wxDynamicToolBar_CreateDefaultLayout',
                     'wxDynamicToolBar_CreateParams',
                     'wxDynamicToolBar_CreateTool',
                     'wxDynamicToolBar_CreateToolControl',
                     'wxDynamicToolBar_Delete',
                     'wxDynamicToolBar_DoDeleteTool',
                     'wxDynamicToolBar_DoEnableTool',
                     'wxDynamicToolBar_DoInsertTool',
                     'wxDynamicToolBar_DoSetToggle',
                     'wxDynamicToolBar_DoToggleTool',
                     'wxDynamicToolBar_DrawSeparator',
                     'wxDynamicToolBar_EnableTool',
                     'wxDynamicToolBar_FindToolForPosition',
                     'wxDynamicToolBar_GetPreferredDim',
                     'wxDynamicToolBar_GetToolInfo',
                     'wxDynamicToolBar_Layout',
                     'wxDynamicToolBar_RemoveTool',
                     'wxDynamicToolBar_SetLayout',
                     'wxEditableListBox_Create',
                     'wxEditableListBox_GetDelButton',
                     'wxEditableListBox_GetDownButton',
                     'wxEditableListBox_GetEditButton',
                     'wxEditableListBox_GetListCtrl',
                     'wxEditableListBox_GetNewButton',
                     'wxEditableListBox_GetStrings',
                     'wxEditableListBox_GetUpButton',
                     'wxEditableListBox_SetStrings',
                     'wxFrameLayout_Activate',
                     'wxFrameLayout_AddBar',
                     'wxFrameLayout_AddPlugin',
                     'wxFrameLayout_AddPluginBefore',
                     'wxFrameLayout_ApplyBarProperties',
                     'wxFrameLayout_CaptureEventsForPane',
                     'wxFrameLayout_CaptureEventsForPlugin',
                     'wxFrameLayout_Create',
                     'wxFrameLayout_Deactivate',
                     'wxFrameLayout_Delete',
                     'wxFrameLayout_DestroyBarWindows',
                     'wxFrameLayout_EnableFloating',
                     'wxFrameLayout_FindBarByName',
                     'wxFrameLayout_FindBarByWindow',
                     'wxFrameLayout_FindPlugin',
                     'wxFrameLayout_FirePluginEvent',
                     'wxFrameLayout_GetBars',
                     'wxFrameLayout_GetClientHeight',
                     'wxFrameLayout_GetClientRect',
                     'wxFrameLayout_GetClientWidth',
                     'wxFrameLayout_GetFrameClient',
                     'wxFrameLayout_GetPane',
                     'wxFrameLayout_GetPaneProperties',
                     'wxFrameLayout_GetParentFrame',
                     'wxFrameLayout_GetTopPlugin',
                     'wxFrameLayout_GetUpdatesManager',
                     'wxFrameLayout_HasTopPlugin',
                     'wxFrameLayout_HideBarWindows',
                     'wxFrameLayout_InverseVisibility',
                     'wxFrameLayout_OnLButtonDown',
                     'wxFrameLayout_OnLButtonUp',
                     'wxFrameLayout_OnLDblClick',
                     'wxFrameLayout_OnMouseMove',
                     'wxFrameLayout_OnRButtonDown',
                     'wxFrameLayout_OnRButtonUp',
                     'wxFrameLayout_OnSize',
                     'wxFrameLayout_PopAllPlugins',
                     'wxFrameLayout_PopPlugin',
                     'wxFrameLayout_PushDefaultPlugins',
                     'wxFrameLayout_PushPlugin',
                     'wxFrameLayout_RecalcLayout',
                     'wxFrameLayout_RedockBar',
                     'wxFrameLayout_RefreshNow',
                     'wxFrameLayout_ReleaseEventsFromPane',
                     'wxFrameLayout_ReleaseEventsFromPlugin',
                     'wxFrameLayout_RemoveBar',
                     'wxFrameLayout_RemovePlugin',
                     'wxFrameLayout_SetBarState',
                     'wxFrameLayout_SetFrameClient',
                     'wxFrameLayout_SetMargins',
                     'wxFrameLayout_SetPaneBackground',
                     'wxFrameLayout_SetPaneProperties',
                     'wxFrameLayout_SetTopPlugin',
                     'wxFrameLayout_SetUpdatesManager',
                     'wxJoystick_Create',
                     'wxJoystick_Delete',
                     'wxJoystick_GetButtonState',
                     'wxJoystick_GetManufacturerId',
                     'wxJoystick_GetMaxAxes',
                     'wxJoystick_GetMaxButtons',
                     'wxJoystick_GetMovementThreshold',
                     'wxJoystick_GetNumberAxes',
                     'wxJoystick_GetNumberButtons',
                     'wxJoystick_GetNumberJoysticks',
                     'wxJoystick_GetPOVCTSPosition',
                     'wxJoystick_GetPOVPosition',
                     'wxJoystick_GetPollingMax',
                     'wxJoystick_GetPollingMin',
                     'wxJoystick_GetPosition',
                     'wxJoystick_GetProductId',
                     'wxJoystick_GetProductName',
                     'wxJoystick_GetRudderMax',
                     'wxJoystick_GetRudderMin',
                     'wxJoystick_GetRudderPosition',
                     'wxJoystick_GetUMax',
                     'wxJoystick_GetUMin',
                     'wxJoystick_GetUPosition',
                     'wxJoystick_GetVMax',
                     'wxJoystick_GetVMin',
                     'wxJoystick_GetVPosition',
                     'wxJoystick_GetXMax',
                     'wxJoystick_GetXMin',
                     'wxJoystick_GetYMax',
                     'wxJoystick_GetYMin',
                     'wxJoystick_GetZMax',
                     'wxJoystick_GetZMin',
                     'wxJoystick_GetZPosition',
                     'wxJoystick_HasPOV',
                     'wxJoystick_HasPOV4Dir',
                     'wxJoystick_HasPOVCTS',
                     'wxJoystick_HasRudder',
                     'wxJoystick_HasU',
                     'wxJoystick_HasV',
                     'wxJoystick_HasZ',
                     'wxJoystick_IsOk',
                     'wxJoystick_ReleaseCapture',
                     'wxJoystick_SetCapture',
                     'wxJoystick_SetMovementThreshold',
                     'wxLEDNumberCtrl_Create',
                     'wxLEDNumberCtrl_GetAlignment',
                     'wxLEDNumberCtrl_GetDrawFaded',
                     'wxLEDNumberCtrl_GetValue',
                     'wxLEDNumberCtrl_SetAlignment',
                     'wxLEDNumberCtrl_SetDrawFaded',
                     'wxLEDNumberCtrl_SetValue',
                     'wxMultiCellCanvas_Add',
                     'wxMultiCellCanvas_CalculateConstraints',
                     'wxMultiCellCanvas_Create',
                     'wxMultiCellCanvas_MaxCols',
                     'wxMultiCellCanvas_MaxRows',
                     'wxMultiCellCanvas_SetMinCellSize',
                     'wxMultiCellItemHandle_Create',
                     'wxMultiCellItemHandle_CreateWithSize',
                     'wxMultiCellItemHandle_CreateWithStyle',
                     'wxMultiCellItemHandle_GetAlignment',
                     'wxMultiCellItemHandle_GetColumn',
                     'wxMultiCellItemHandle_GetHeight',
                     'wxMultiCellItemHandle_GetLocalSize',
                     'wxMultiCellItemHandle_GetRow',
                     'wxMultiCellItemHandle_GetStyle',
                     'wxMultiCellItemHandle_GetWeight',
                     'wxMultiCellItemHandle_GetWidth',
                     'wxMultiCellSizer_CalcMin',
                     'wxMultiCellSizer_Create',
                     'wxMultiCellSizer_Delete',
                     'wxMultiCellSizer_EnableGridLines',
                     'wxMultiCellSizer_RecalcSizes',
                     'wxMultiCellSizer_SetColumnWidth',
                     'wxMultiCellSizer_SetDefaultCellSize',
                     'wxMultiCellSizer_SetGridPen',
                     'wxMultiCellSizer_SetRowHeight',
                     'wxMutex_Create',
                     'wxMutex_Delete',
                     'wxMutex_IsLocked',
                     'wxMutex_Lock',
                     'wxMutex_TryLock',
                     'wxMutex_Unlock',
                     'wxNewBitmapButton_Create',
                     'wxNewBitmapButton_CreateFromFile',
                     'wxNewBitmapButton_Delete',
                     'wxNewBitmapButton_DrawDecorations',
                     'wxNewBitmapButton_DrawLabel',
                     'wxNewBitmapButton_Enable',
                     'wxNewBitmapButton_Realize',
                     'wxNewBitmapButton_RenderAllLabelImages',
                     'wxNewBitmapButton_RenderLabelImage',
                     'wxNewBitmapButton_RenderLabelImages',
                     'wxNewBitmapButton_Reshape',
                     'wxNewBitmapButton_SetAlignments',
                     'wxNewBitmapButton_SetLabel',
                     'wxPlotEvent_GetCurve',
                     'wxPlotEvent_GetPosition',
                     'wxPlotEvent_GetZoom',
                     'wxPlotEvent_SetPosition',
                     'wxPlotEvent_SetZoom',
                     'wxPlotOnOffCurve_Add',
                     'wxPlotOnOffCurve_Create',
                     'wxPlotOnOffCurve_Delete',
                     'wxPlotOnOffCurve_DrawOffLine',
                     'wxPlotOnOffCurve_DrawOnLine',
                     'wxPlotOnOffCurve_GetAt',
                     'wxPlotOnOffCurve_GetClientData',
                     'wxPlotOnOffCurve_GetCount',
                     'wxPlotOnOffCurve_GetEndX',
                     'wxPlotOnOffCurve_GetOff',
                     'wxPlotOnOffCurve_GetOffsetY',
                     'wxPlotOnOffCurve_GetOn',
                     'wxPlotOnOffCurve_GetStartX',
                     'wxPlotOnOffCurve_SetOffsetY',
                     'wxPlotWindow_Add',
                     'wxPlotWindow_AddOnOff',
                     'wxPlotWindow_Create',
                     'wxPlotWindow_Delete',
                     'wxPlotWindow_DeleteOnOff',
                     'wxPlotWindow_Enlarge',
                     'wxPlotWindow_GetAt',
                     'wxPlotWindow_GetCount',
                     'wxPlotWindow_GetCurrent',
                     'wxPlotWindow_GetEnlargeAroundWindowCentre',
                     'wxPlotWindow_GetOnOffCurveAt',
                     'wxPlotWindow_GetOnOffCurveCount',
                     'wxPlotWindow_GetScrollOnThumbRelease',
                     'wxPlotWindow_GetUnitsPerValue',
                     'wxPlotWindow_GetZoom',
                     'wxPlotWindow_Move',
                     'wxPlotWindow_RedrawEverything',
                     'wxPlotWindow_RedrawXAxis',
                     'wxPlotWindow_RedrawYAxis',
                     'wxPlotWindow_ResetScrollbar',
                     'wxPlotWindow_SetCurrent',
                     'wxPlotWindow_SetEnlargeAroundWindowCentre',
                     'wxPlotWindow_SetScrollOnThumbRelease',
                     'wxPlotWindow_SetUnitsPerValue',
                     'wxPlotWindow_SetZoom',
                     'wxPoint_Destroy',
                     'wxRemotelyScrolledTreeCtrl_AdjustRemoteScrollbars',
                     'wxRemotelyScrolledTreeCtrl_CalcTreeSize',
                     'wxRemotelyScrolledTreeCtrl_CalcTreeSizeItem',
                     'wxRemotelyScrolledTreeCtrl_Create',
                     'wxRemotelyScrolledTreeCtrl_Delete',
                     'wxRemotelyScrolledTreeCtrl_GetCompanionWindow',
                     'wxRemotelyScrolledTreeCtrl_GetScrollPos',
                     'wxRemotelyScrolledTreeCtrl_GetScrolledWindow',
                     'wxRemotelyScrolledTreeCtrl_GetViewStart',
                     'wxRemotelyScrolledTreeCtrl_HideVScrollbar',
                     'wxRemotelyScrolledTreeCtrl_PrepareDC',
                     'wxRemotelyScrolledTreeCtrl_ScrollToLine',
                     'wxRemotelyScrolledTreeCtrl_SetCompanionWindow',
                     'wxRemotelyScrolledTreeCtrl_SetScrollbars',
                     'wxSize_Destroy',
                     'wxSplitterScrolledWindow_Create',
                     'wxThinSplitterWindow_Create',
                     'wxThinSplitterWindow_DrawSash',
                     'wxThinSplitterWindow_SashHitTest',
                     'wxThinSplitterWindow_SizeWindows',
                     'wxToolLayoutItem_IsSeparator',
                     'wxToolLayoutItem_Rect',
                     'wxToolWindow_AddMiniButton',
                     'wxToolWindow_Create',
                     'wxToolWindow_GetClient',
                     'wxToolWindow_SetClient',
                     'wxToolWindow_SetTitleFont',
                     'wxTreeCompanionWindow_Create',
                     'wxTreeCompanionWindow_DrawItem',
                     'wxTreeCompanionWindow_GetTreeCtrl',
                     'wxTreeCompanionWindow_SetTreeCtrl',
                     'wxXmlResource_Delete']


if __name__ == '__main__':
    main()
