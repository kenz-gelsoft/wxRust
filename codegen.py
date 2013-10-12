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
        self.println('trait %s {' % clazz.wrapper_name)
        self.indent()
        for method in clazz.methods:
            method.trait_fn(self, clazz.name)
        self.unindent()
        self.println('}')

    def println(self, text=''):
        lines = text.split('\n')
        for line in lines:
            line = '%s%s' % (''+(' ' * 4 * self.__indent), line)
            print line

    def indent(self):
        self.__indent += 1

    def unindent(self):
        self.__indent -= 1


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
    
    def parse_files(self, files):
        for file in files:
            for line in Preprocessor().preprocess(file):
                self._parse_line(line.strip())
        for clazz in self.__classes:
            for f in self.__functions:
                clazz.add_if_member(f)

    @property
    def classes(self):
        return self.__classes

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
            clazz = Class(node)
            # linear search isn't fast.
            for clazz2 in self.classes:
                if clazz.name == clazz2.name:
                    return
            self.__classes.append(clazz)
        else:
            # func def
            self.__functions.append(Function(node))


class Class(object):
    def __init__(self, node):
        assert len(node) == 1
        assert len(node[0]) > 1
        self.__node = node
        self.__methods = []

    @property
    def name(self):
        return self.__node[0][1][0]

    @property
    def wrapper_name(self):
        return self.name
#        if self.name.startswith('wx'):
#            return self.name[2:]
#        return self.name
    
    @property
    def methods(self):
        return self.__methods

    def add_if_member(self, f):
        if f.name.startswith('%s_' % self.name):
            self.__methods.append(f)


class Function(object):
    def __init__(self, node):
        #print node
        assert len(node) > 1
        assert node[1][0]
        self.__node = node

    @property
    def name(self):
        return self.__node[1][0]

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
            return ''
        s = ''
        _args = (self.__node[1])[1:]
        for i, arg in enumerate(_args):
            if i:
                s += ', '
            s += str(Arg(arg, i))
        return s
    
    @property
    def calling_args(self):
        if len(self.__node[1][1]) == 0:
            # no args
            return ''
        s = ''
        _args = (self.__node[1])[1:]
        for i, arg in enumerate(_args):
            if i:
                s += ', '
            s += Arg(arg, i).name
        return s
    
    @property
    def returns(self):
        r = Type(self.__node[0])
        if r.is_void:
            return ''
        return ' -> %s' % r

    def trait_fn(self, gen, classname):
        #gen.println('// %s' % self.__node)
        gen.println('#[fixed_stack_segment]')
        gen.println('fn %s(%s)%s {' % (self.method_name(classname), self.args, self.returns))
        gen.indent()
        gen.println('unsafe { %s(%s) }' % (self.name, self.calling_args))
        gen.unindent()
        gen.println('}')


# Function style type macros
def TArrayObjectOutVoid(args):
    return '~[@%s]' % args # it would be **c_void ?
def TClassRef(args):
    return '@%s' % args


class Arg(object):
    def __init__(self, node, index):
        assert len(node) > 0
        self.__node = node
        self.__index = index

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
    
    def __str__(self):
        tag = self.__node[0][0]
        if tag in ['TArrayObjectOutVoid', 'TClassRef']:
            macro = globals()[tag]
            macro_args = tuple([x[0] for x in self.__node[0][1:]])
            return '%s: %s' % (self.name, macro(macro_args))
        if self.is_self:
            return '&self'
        return '%s: %s' % (self.name, Type(self.__node[0]))

# Other type mappings
type_mapping = {
    # header type          # rust type
    'TArrayIntOutVoid':    '*intptr_t',
    'TArrayIntPtrOutVoid': '*intptr_t',
    'TArrayLen':           'c_int',
    'TArrayStringOutVoid': '**wchar_t',
    'TBool':               'bool',
    'TBoolInt':            'c_int',
    'TByteStringLazyOut':  '*c_char',
    'TByteStringLen':      'c_int',
    'TByteStringOut':      '*c_char',
    'TChar':               'wchar_t',
    'TClosureFun':         '*c_void',
    'TInt64':              'i64',
    'TIntPtr':             'intptr_t',
    'TString':             '*wchar_t',
    'TStringLen':          'c_int',
    'TStringOut':          '**wchar_t',
    'TStringVoid':         '*wchar_t',
    'TUInt':               'uint32_t',
    'TUInt8':              'uint8_t',
    'char':                'c_char',
    'double':              'c_double',
    'float':               'c_float',
    'int':                 'c_int',
    'long':                'c_long',
    'void':                'c_void',
}

class Type(object):
    def __init__(self, node):
        self.__node = node

    @property
    def is_complex(self):
        return not isinstance(self.__node, basestring)
    
    @property
    def is_self(self):
        return self.is_complex and self.__node[0] == 'TSelf'
    
    @property
    def inner(self):
        if self.is_complex:
            return self.__node[1]
        return self.__node
    
    @property
    def is_void(self):
        return self.inner == 'void'
    
    @property
    def is_class(self):
        return self.is_complex and self.__node[0] == 'TClass'
    
    @property
    def is_ptr(self):
        return self.is_complex and self.__node[0] == '*'
    
    def __str__(self):
        if self.is_self or self.is_class:
            return '@%s' % self.__node[1][0]
        if self.is_ptr:
            return '*%s' % Type(self.inner)
        s = str(self.__node)
        if s in type_mapping:
            return type_mapping[s]
        return s


if __name__ == '__main__':
    main()
