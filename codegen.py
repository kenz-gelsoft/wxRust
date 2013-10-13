from subprocess import PIPE, Popen

import re
import sys


def main():
    parser = Parser()
    parser.parse_files(sys.argv[1:])
    WrapperGenerator(parser).generate()


class WrapperGenerator(object):
    def __init__(self, parser):
        self.__parser = parser
        self.__indent = 0

    def generate(self):
        self.println('use std::libc::*;')
        self.println('use native::*;')
        self.println()
        for clazz in self.__parser.classes:
            self.print_class(clazz)

    def print_class(self, clazz):
        struct_name = '%s' % clazz.struct_name
        self.println('struct %s(*u8);' % struct_name)
        for trait in clazz.inheritance:
            body = ''
            if trait in self.__parser.root_classes:
                body = ' fn handle(&self) -> *u8 { **self } '
            self.println('impl %s for %s {%s}' % (trait_name(trait), struct_name, body))
        self.println()
    
        # static methods go to struct impl
        self.println('impl %s {' % struct_name)
        self.indent()
        self.println('pub fn from(handle: *u8) -> @%s {' % clazz.trait_name)
        self.println('    @%s(handle) as @%s' % (clazz.struct_name, clazz.trait_name))
        self.println('}')
        self.println()
        for method in clazz.static_methods:
            method.trait_fn(self, clazz.name)
        self.unindent()
        self.println('}')
        
        # instance methods go to trait's default impl
        base = clazz.has_base and ' : %s' % trait_name(clazz.base) or ''
        self.println()
        self.println('trait %s%s {' % (clazz.trait_name, base))
        self.indent()
        if clazz.name in self.__parser.root_classes:
            self.println('fn handle(&self) -> *u8;')
            self.println()
        for method in clazz.methods:
            method.trait_fn(self, clazz.name)
        self.unindent()
        self.println('}')
        self.println()

    def println(self, text=''):
        lines = text.split('\n')
        for line in lines:
            line = '%s%s' % (''+(' ' * 4 * self.__indent), line)
            print line

    def indent(self):
        self.__indent += 1

    def unindent(self):
        self.__indent -= 1


def struct_name(name):
    return name

def trait_name(name):
    return '_' + name


class Preprocessor(object):
    def __init__(self):
        pass

    def call_cpp(self, file):
        cppflags = Popen(['wx-config', '--cppflags'], stdout=PIPE).communicate()[0].split()
        cppflags.remove('-D__WXMAC__')
        cmdline = ['cpp', '-DWXC_TYPES_H'] + cppflags + ['-I/Users/kenz/src/wxRust/wxHaskell/wxc/src/include', file]
        return Popen(cmdline, stdout=PIPE).communicate()[0]
    
    def preprocess(self, file):
        for line in Preprocessor._normalize(self.call_cpp(file)).splitlines():
            line = line.strip()
            if not len(line) or line.startswith('#'):
                continue
            yield line
    
    @staticmethod
    def _normalize(text):
        text = re.sub(r'^\s*\n', '', text)
        text = re.sub(r'\n+', '\n', text)
        text = re.sub(r'\\\n\s+', '', text)
        return text


# Function style arg macros
def TArrayString(args):
    return [['int', args[0]],
            [['*', 'wchar_t'], args[1]]]
def TByteString(args):
    return [[['*', 'char'], args[0]],
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
    return TPoint(args, T=['*', 'int'])
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
    return TRect(args, T=['*', 'int'])
TSize = TPoint
def TSizeDouble(args):
    return TSize(args, T='double')
def TSizeOut(args):
    return TSize(args, T=['*', 'int'])
def TSizeOutDouble(args):
    return TSize(args, T=['*', 'double'])
def TSizeOutVoid(args):
    return TSize(args, T=['*', 'int'])
TVector = TPoint


class Parser(object):
    def __init__(self):
        self.__classes   = []
        self.__functions = []
        self.__root_classes = set()
    
    def parse_files(self, files):
        for file in files:
            for line in Preprocessor().preprocess(file):
                self._parse_line(line.strip())
        for clazz in self.__classes:
            self.__root_classes.add(clazz.inheritance[-1])
            for f in self.__functions:
                if f.name in missing_functions:
                    continue
                clazz.add_if_member(f)

    @property
    def root_classes(self):
        return self.__root_classes

    @property
    def classes(self):
        return self.__classes
    
    def classForName(self, name):
        for clazz in self.__classes:
            if clazz.name == name:
                return clazz
        return None

    def _parse_line(self, line):
        if not len(line):
            return

        # Trivial parser
        # TODO: extract LineParser class
        stack = []
        node = []
        lexer = (t.group(0) for t in re.finditer(re.compile(r'\s+|\*|\(|,|\)|;|[^\s*(,);]+'), line))
        for token in lexer:
            if not token.strip() or token == ';':
                # ignore separators
                continue
            if token == '(':
                tagname = node.pop()
                stack.append(node)
                stack.append([tagname])
                node = []
                continue
            if token == ',':
                stack[-1].append(node)
                node = [] # new-arg
                continue
            if token == ')':
                stack[-1].append(node)
                node = stack.pop() # end-arg
                stack[-1].append(node)
                node = stack.pop() # end-arg-list
                
                # Handles function style argument macros here
                tag = node[-1][0]
                if tag in ['TArrayString',
                           'TByteString', 'TByteStringLazy',
                           'TColorRGB',
                           'TPoint', 'TPointDouble', 'TPointLong', 'TPointOut', 'TPointOutDouble', 'TPointOutVoid',
                           'TRect',  'TRectDouble',                             'TRectOutDouble',  'TRectOutVoid',
                           'TSize',  'TSizeDouble',                'TSizeOut',  'TSizeOutDouble',  'TSizeOutVoid',
                           'TVector']:
#                    # ---- before
#                    for pair in enumerate(stack):
#                        print '%s: %s' % pair
#                    print node
                    macro = globals()[tag]
                    macro_args = [arg[0] for arg in node.pop()[1:]]
                    macro_result = macro(macro_args)
                    for result_node in macro_result[:-1]:
                        stack[-1].append(result_node)
                    # We assume the last node read until here
                    for item in macro_result[-1]:
                        node.append(item)
#                    # ---- after
#                    for pair in enumerate(stack):
#                        print '%s: %s' % pair
#                    print node
#                    sys.exit()
                continue
            if token == '*':
                node.append(['*', node.pop()])
                continue
            node.append(token)
            continue
        
        #print node
        if 'TClassDef' in line:
            # class def
            clazz = Class(self, node)
            # linear search isn't fast.
            for clazz2 in self.classes:
                if clazz.name == clazz2.name:
                    return
            self.__classes.append(clazz)
        else:
            # func def
            self.__functions.append(Function(node))


class Class(object):
    def __init__(self, parser, node):
        assert len(node) == 1
        assert len(node[0]) > 1
        self.__parser = parser
        self.__node = node
        self.__methods = []

    @property
    def name(self):
        return self.__node[0][1][0]

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
        return self.__node[0][2][0]
    
    @property
    def inheritance(self):
        list = [self.name]
        if self.has_base:
            list += self.__parser.classForName(self.base).inheritance
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


class Function(object):
    def __init__(self, node):
        #print node
        assert len(node) > 1
        assert node[1][0]
        self.__node = node
        self.__return_type = Type(self.__node[0])

    @property
    def name(self):
        return self.__node[1][0]

    @property
    def is_static(self):
        args = list(self.args)
        if len(args) < 1:
            return True
        return not args[0].type.is_self

    def method_name(self, classname):
        _name = self.name[len(classname)+1:]
        _name = _name[0].lower() + _name[1:]
        if _name in ['break', 'yield']:
            _name += '_'
        if _name.startswith('create'):
            return 'new' + _name[len('create'):]
        return _name
    
    @property
    def args(self):
        if len(self.__node[1][1]) == 0:
            # no args
            return ()
        _args = (self.__node[1])[1:]
        return (Arg(arg, i) for i, arg in enumerate(_args))
    
    @property
    def decl_args(self):
        return ', '.join((str(a) for a in self.args))
    
    @property
    def calling_args(self):
        return ', '.join((a.calling_arg for a in self.args))
    
    @property
    def returns(self):
        if self.__return_type.is_void:
            return ''
        return ' -> %s' % self.__return_type

    def trait_fn(self, gen, classname):
        #gen.println('// %s' % self.__node)
        modifier = self.is_static and 'pub ' or ''
        gen.println('#[fixed_stack_segment]')
        gen.println('%sfn %s(%s)%s {' % (modifier,
                                         self.method_name(classname),
                                         self.decl_args,
                                         self.returns))
        gen.indent()
        body = '%s(%s)' % (self.name, self.calling_args)
        if self.__return_type.is_self or \
            self.__return_type.is_class:
            body = '@%s(%s) as %s' % (self.__return_type.struct_name, body, self.__return_type)
        gen.println('unsafe { %s }' % body)
        gen.unindent()
        gen.println('}')


# Function style type macros
def TArrayObjectOutVoid(args):
    return '*u8'#'~[@%s]' % args # it would be **c_void ?


class Arg(object):
    def __init__(self, node, index):
        assert len(node) > 0
        self.__node = node
        self.__index = index
        self.__type = Type(self.__node[0])

    @property
    def is_self(self):
        # work around second TSelf(T) arguments...
        return self.__node[0][0] == 'TSelf' and self.__index == 0
    
    @property
    def name(self):
        if len(self.__node) == 1:
            return '_arg%s' % self.__index
        else:
            if self.is_self:
                return 'self'
            _name = self.__node[1]
            if _name in ['fn', 'ref', 'self', 'type', 'use']:
                _name += '_'
            return _name

    @property
    def type(self):
        return self.__type

    @property
    def calling_arg(self):
        if self.is_self:
            return 'self.handle()'
        elif self.type.is_class:
            return '%s.handle()' % self.name
        else:
            return self.name

    def __str__(self):
        tag = self.__node[0][0]
        if tag in ['TArrayObjectOutVoid']:
            macro = globals()[tag]
            macro_args = tuple([x[0] for x in self.__node[0][1:]])
            return '%s: %s' % (self.name, macro(macro_args))
        if self.is_self:
            return '&self'
        return '%s: %s' % (self.name, self.type)


# Other type mappings
type_mapping = {
    # header type          # rust type
    'TArrayIntOutVoid':    '*intptr_t',
    'TArrayIntPtrOutVoid': '*intptr_t',
    'TArrayLen':           'c_int',
    'TArrayStringOutVoid': '*wchar_t',#'**wchar_t',
    'TBool':               'bool',
    'TBoolInt':            'c_int',
    'TByteStringLazyOut':  '*char',#'*c_char',
    'TByteStringLen':      'c_int',
    'TByteStringOut':      '*char',#'*c_char',
    'TChar':               'wchar_t',
    'TClosureFun':         '*u8',#'*c_void',
    'TInt64':              'i64',
    'TIntPtr':             'intptr_t',
    'TString':             '*wchar_t',
    'TStringLen':          'c_int',
    'TStringOut':          '*wchar_t',#'**wchar_t',
    'TStringVoid':         '*wchar_t',
    'TUInt':               'uint32_t',
    'TUInt8':              'uint8_t',
#    'char':                'c_char',
    'double':              'c_double',
    'float':               'c_float',
    'int':                 'c_int',
    'long':                'c_long',
    'void':                'u8',#'c_void',
}


class Type(object):
    def __init__(self, node):
        self.__node = node

    @property
    def head(self):
        if self.is_complex:
            return self.__node[0]
        return self.__node

    @property
    def is_complex(self):
        return not isinstance(self.__node, basestring)
    
    @property
    def is_self(self):
        return self.is_complex and self.head == 'TSelf'
    
    @property
    def inner(self):
        if self.is_complex:
            return self.__node[1]
        return self.__node
    
    @property
    def is_void(self):
#        print self.head
#        sys.exit()
        return self.head == 'void'
    
    @property
    def is_class(self):
        return self.is_complex and self.head in ['TClass', 'TClassRef', 'TSelf']
    
    @property
    def is_ptr(self):
        return self.is_complex and self.head == '*'
    
    @property
    def struct_name(self):
        assert self.is_class
        return struct_name(self.__node[1][0])
    
    @property
    def trait_name(self):
        assert self.is_class
        return trait_name(self.__node[1][0])
    
    def __str__(self):
        if self.is_self or self.is_class:
            return '@%s' % self.trait_name
        if self.is_ptr:
            t = Type(self.inner)
            if t.is_class:
                return '*u8'
            # work around native.rs bug
            if t.is_ptr and t.inner == 'TChar':
                return '*wchar_t'
            return '*%s' % t
        s = str(self.__node)
        if s in type_mapping:
            return type_mapping[s]
        return s


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
