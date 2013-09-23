import re
import sys


def main():
    p = Parser()
    for file in sys.argv[1:]:
        with open(file) as f:
            p.parse(Preprocessor().preprocess(f))
        p.print_parsed()


class Preprocessor(object):
    def __init__(self):
        self.__cond_stack = [True]
        pass

    def expand_cond_macros(self, condition):
        while True:
            rewritten = condition
            rules = (
                     ('defined(WXC_GLUE_H)', '0'),
                     ('wxCHECK_VERSION(2,9,5)', '0'),
                     ('(wxVERSION_NUMBER >= 2905)', '0'),
                     ('0 || 0', '0'),
                     ('1 || 0', '1'),
                     ('!0', '1'),
                     ('!1', '0'),
                     ('0 && 0', '0'),
                     ('0 && 1', '0'),
                     ('1 && 0', '0'),
                     ('1 && 1', '1'),
                     ('(0)', '0'),
                     ('(1)', '1'),
                     )
            for macro, value in rules:
                rewritten = rewritten.replace(macro, value)
            if rewritten == condition:
                return rewritten
            condition = rewritten

    def expand_normal_macros(self, line):
        line = re.sub(r'TArrayIntOutVoid', 'intptr_t*', line)
        line = re.sub(r'TArrayIntPtrOutVoid', 'intptr_t*', line)
        line = re.sub(r'TArrayLen', 'int', line)
        line = re.sub(r'TArrayObjectOutVoid\([^)]*\)', 'void**', line)
        line = re.sub(r'TArrayString\([^)]*\)', 'int, wchar_t*', line)
        line = re.sub(r'TArrayStringOutVoid', 'wchar_t**', line)
        line = re.sub(r'TBoolInt', 'int', line)
        line = re.sub(r'TBool', 'bool', line)
        line = re.sub(r'TByteString\([^)]*\)', 'char**, int', line)
        line = re.sub(r'TByteStringLazy\([^)]*\)', 'char**, int', line)
        line = re.sub(r'TByteStringLazyOut', 'char*', line)
        line = re.sub(r'TByteStringLen', 'int', line)
        line = re.sub(r'TByteStringOut', 'char*', line)
        line = re.sub(r'TChar', 'wchar_t', line)
        line = re.sub(r'TClass\([^)]*\)', 'void*', line)
        line = re.sub(r'TClassRef\([^)]*\)', 'void*', line)
        line = re.sub(r'TColorRGB\([^)]*\)', 'ColorRGB', line)
        line = re.sub(r'TPoint\([^)]*\)', 'Point<c_int>', line)
        line = re.sub(r'TPointLong\([^)]*\)', 'Point<c_long>', line)
        line = re.sub(r'TPointOut\([^)]*\)', 'Point<c_int>*', line)
        line = re.sub(r'TPointOutVoid\([^)]*\)', 'Point<c_int>*', line)
        line = re.sub(r'TRect\([^)]*\)', 'Rect<c_int>', line)
        line = re.sub(r'TRectOutVoid\([^)]*\)', 'Rect<c_int>*', line)
        line = re.sub(r'TSelf\([^)]*\)', 'void*', line)
        line = re.sub(r'TSize\([^)]*\)', 'Size<c_int>', line)
        line = re.sub(r'TSizeOut\([^)]*\)', 'Size<c_int>*', line)
        line = re.sub(r'TSizeOutDouble\([^)]*\)', 'Size<c_double>*', line)
        line = re.sub(r'TSizeOutVoid\([^)]*\)', 'Size<c_int>*', line)
        line = re.sub(r'TStringVoid', 'wchar_t*', line)
        line = re.sub(r'TString', 'wchar_t*', line)
        line = re.sub(r'TUInt8', 'uint8_t', line)
        line = re.sub(r'TVector\([^)]*\)', 'Vector<c_int>', line)
        return line

    def preprocess(self, file):
        output = []
        for line in  Preprocessor._normalize(file.read()).splitlines():
            line = line.strip()
            if line.startswith('#'):
                # directive line
                line = line[1:].strip()
                if line.startswith('import') or line.startswith('include') or line.startswith('define'):
                    # ignore imports, includes and defines for now.
                    continue
                elif line.startswith('if') or line.startswith('elif') or line.startswith('ifdef') or line.startswith('ifndef'):
                    # ifdef/ifndef directive
                    if line.startswith('ifdef') or line.startswith('ifndef'):
                        if line.startswith('ifdef'):
                            macro = line[len('ifdef'):].strip()
                            line = 'if defined(%s)' % macro
                        else:
                            macro = line[len('ifndef'):].strip()
                            line = 'if !defined(%s)' % macro
                    # if/elif directive
                    if line.startswith('if'):
                        condition = line[len('if'):].strip()
                    else:
                        self.__cond_stack.pop()
                        condition = line[len('elif'):].strip()
                    condition = self.expand_cond_macros(condition)
                    # condition
                    if condition == '1':
                        self.__cond_stack.append(True)
                        continue
                    elif condition == '0':
                        self.__cond_stack.append(False)
                        continue
                    else:
                        raise RuntimeError('invalid condition |%s|' % condition)
                elif line.startswith('else'):
                    self.__cond_stack.append(not self.__cond_stack.pop())
                    continue
                elif line.startswith('endif'):
                    self.__cond_stack.pop()
                    continue
                else:
                    raise RuntimeError('not supported directive |%s|' % line)
            if self.__cond_stack[-1]:
                output.append(self.expand_normal_macros(line))
        return output

    @staticmethod
    def _strip_comments(text):
        text = re.sub(r'(\/\*.*?\*\/|\/\/[^\n]*)', '', text, flags=re.DOTALL)
        return text
    
    @staticmethod
    def _normalize(text):
        text = Preprocessor._strip_comments(text)
        text = re.sub(r'^\s*\n', '', text)
        text = re.sub(r'\n+', '\n', text)
        text = re.sub(r'\\\n\s+', '', text)
        return text


class Parser(object):
    def __init__(self):
        self.__indent = 0
        self.__current = Class(self)
        self.__classes = [self.__current]
    
    def parse(self, lines):
        for line in lines:
            self._parse_line(line.strip())
    
    def _parse_line(self, line):
        if not len(line):
            return
        if 'TClassDef' in line:
            # special line
            self.__current = Class(self)
            self.__classes.append(self.__current)
            self.__current.parse_header_line(line)
            return
        if not('(' in line and ')' in line):
            # all delarations are functions.
            assert False
        self.__current.parse_line(line)

    def print_parsed(self):
        self.println('use std::libc::*;')
        self.println('use types::*;')
        self.println()
        self.println('#[link_args="-lwxc"]')
        self.println('extern {')
        self.__indent += 1
        for clazz in self.__classes:
            clazz.print_class()
        self.__indent -= 1
        self.println('}')
            
    def println(self, text=''):
        lines = text.split('\n')
        for line in lines:
            line = '%s%s' % (''+(' ' * 4 * self.__indent), line)
            print line


class Class(object):
    def __init__(self, parser):
        self.__parser = parser
        self.__header_line = None
        self.__methods = []
        pass

    def parse_header_line(self, line):
        self.__header_line = line

    def parse_line(self, line):
        self.__methods.append(Method(None).parse(line))

    def print_class(self):
        if self.__header_line:
            self.println('\n// %s' % self.__header_line)
        for method in self.__methods:
            self.println(repr(method))

    def println(self, text=''):
        self.__parser.println(text)


class Method(object):
    def __init__(self, classname):
        self.__classname = classname

    def parse(self, line):
        line = re.sub(r'\s+', ' ', line);
        line = re.sub(r' \(', '(', line);
        front = line[:line.find('(')]
        self.__rtype = Type().parse(front[:front.rfind(' ')].strip())
        self.__name = front[front.rfind(' ')+1:]
        args = line[line.find('(')+1:line.rfind(')')].strip()
        self.__args = []
        if len(args.strip()):
            count = 0
            for arg in args.split(','):
                pair = self.normalize_ptr(arg).split(' ')
                if len(pair) < 2:
                    pair.append('arg%s' % count)
                    count += 1
                for keyword in ['fn', 'ref', 'self', 'type', 'use']:
                    if pair[1] == keyword:
                        pair[1] += '_'
                self.__args.append([Type(param=True).parse(pair[0]), pair[1]])
        return self

            
    def normalize_ptr(self, arg):
        arg = re.sub(r'\*\s*', '* ', arg)
        arg = re.sub(r'\*\s\*', '*', arg)
        arg = re.sub(r'\s+\*', '*', arg)
        arg = re.sub(r'\s+', ' ', arg)
        arg = arg.strip()
        assert len(arg.split(' ')) <= 2
        return arg

    def __repr__(self):
        for ignore in ['ELJClient_',
                       'ELJCommand_',
                       'ELJConnection_',
                       'ELJPlotCurve_',
                       'ELJServer_',
                       'cb',
                       'wxCommandProcessor_',
                       'wxCondition_',
                       'wxCriticalSection_',
                       'wxDateTime_IsGregorianDate',
                       'wxDialUpEvent_',
                       'wxDialUpManager_',
                       'wxDynToolInfo_',
                       'wxDynamicSashWindow_',
                       'wxDynamicToolBar_',
                       'wxEditableListBox_',
                       'wxFrameLayout_',
                       'wxJoystick_',
                       'wxLEDNumberCtrl_',
                       'wxMessageParameters_',
                       'wxMultiCellCanvas_',
                       'wxMultiCellItemHandle_',
                       'wxMultiCellSizer_',
                       'wxMutexGui_',
                       'wxMutex_',
                       'wxNewBitmapButton_',
                       'wxPlotEvent_',
                       'wxPlotOnOffCurve_',
                       'wxPlotWindow_',
                       'wxPoint_Destroy',
                       'wxRemotelyScrolledTreeCtrl_',
                       'wxSize_Destroy',
                       'wxSplitterScrolledWindow_',
                       'wxThinSplitterWindow_',
                       'wxToolLayoutItem_',
                       'wxToolWindow_',
                       'wxTreeCompanionWindow_',
                       'wxXmlResource_Delete',
                       ]:
            if self.__name.startswith(ignore):
                return '// missing: %s' % self.__name
        
        args = ''
        for arg in self.__args:
            if len(args):
                args += ', '
            args += '%s: %s' % (arg[1], arg[0])
        ret = ''
        if not self.__rtype.is_void:
            ret = ' -> %s' % self.__rtype
        return 'pub fn %s(%s)%s;' % (self.__name, args, ret)


class Type(object):
    def __init__(self, param=False):
        self.__param = param
        self.is_ptr = False
        self.is_const_ptr = False
        self.is_pp = False
        self.is_const = False
        self.is_void = False
        self.name = ''
    
    def parse(self, type):
        self.original = type
        type = type.strip()
        if type == 'void':
            self.is_void = True
        if type.endswith('*'):
            self.is_ptr = True
            self.is_const_ptr = self.is_const
            self.is_const = False
            type = type[:-1].strip()
            if type.endswith('*'):
                self.pp = True
                type = type[:-1].strip()
        if '(*' in type:
            raise RuntimeError("function pointer isn't supported yet.")
        if '[' in type:
            raise RuntimeError("array isn't supported yet.")
        self.name = type
        return self
    
    def is_primitive(self):
        return not self.name.startswith('NS')
    
    def __repr__(self):
        s = self.name
        if s == 'double':
            s = 'c_double'
        if s == 'long':
            s = 'c_long'
        if s == 'long long':
            s = 'c_longlong'
        if s == 'int':
            s = 'c_int'
        if self.is_ptr:
            if self.is_primitive():
                if s == 'void':
                    s = 'u8'
                if self.__param:
                    s = '&[%s]' % s
                else:
                    s = '~[%s]' % s
            else:
                s = '@mut %s' % s
        return '%s /* %s */' % (s, self.original)


if __name__ == '__main__':
    main()
