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
        self.println('trait %s {' % clazz.name)
        self.indent()
        for method in clazz.methods:
            method.trait_fn(self)
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

    def expand_normal_macros(self, line):
        return line
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
        line = re.sub(r'TClass\s*\([^)]*\)', 'void*', line)
        line = re.sub(r'TClassRef\([^)]*\)', 'void*', line)
        line = re.sub(r'TClosureFun', 'void*', line)
        line = re.sub(r'TColorRGB\([^)]*\)', 'u8, u8, u8', line)
        line = re.sub(r'TInt64', 'i64', line)
        line = re.sub(r'TIntPtr', 'intptr_t', line)
        line = re.sub(r'TPoint\([^)]*\)', 'int, int', line)
        line = re.sub(r'TPointDouble\([^)]*\)', 'double, double', line)
        line = re.sub(r'TPointLong\([^)]*\)', 'long, long', line)
        line = re.sub(r'TPointOut\([^)]*\)', 'int*, int*', line)
        line = re.sub(r'TPointOutDouble\([^)]*\)', 'double*, double*', line)
        line = re.sub(r'TPointOutVoid\([^)]*\)', 'int*, int*', line)
        line = re.sub(r'TRect\([^)]*\)', 'int, int, int, int', line)
        line = re.sub(r'TRectDouble\([^)]*\)', 'double, double, double, double', line)
        line = re.sub(r'TRectOutDouble\([^)]*\)', 'double*, double*, double*, double*', line)
        line = re.sub(r'TRectOutVoid\([^)]*\)', 'int*, int*, int*, int*', line)
        line = re.sub(r'TSelf\([^)]*\)', 'void*', line)
        line = re.sub(r'TSize\([^)]*\)', 'int, int', line)
        line = re.sub(r'TSizeDouble\([^)]*\)', 'double, double', line)
        line = re.sub(r'TSizeOut\([^)]*\)', 'int*, int*', line)
        line = re.sub(r'TSizeOutDouble\([^)]*\)', 'double*, double*', line)
        line = re.sub(r'TSizeOutVoid\([^)]*\)', 'int*, int*', line)
        line = re.sub(r'TStringVoid', 'wchar_t*', line)
        line = re.sub(r'TStringLen', 'int', line)
        line = re.sub(r'TString', 'wchar_t*', line)
        line = re.sub(r'TUInt8', 'uint8_t', line)
        line = re.sub(r'TUInt', 'uint32_t', line)
        line = re.sub(r'TVector\([^)]*\)', 'int, int', line)
        return line

    def preprocess(self, file):
        cppflags = Popen(['wx-config', '--cppflags'], stdout=PIPE).communicate()[0].split()
        cppflags.remove('-D__WXMAC__')
        cmdline = ['cpp', '-DWXC_TYPES_H'] + cppflags + ['-I/Users/kenz/src/wxRust/wxHaskell/wxc/src/include', file]
        content = Popen(cmdline, stdout=PIPE).communicate()[0]
        content = Preprocessor._normalize(content)
        return [self.expand_normal_macros(line) for line in content.splitlines() if len(line.strip()) and not line.strip().startswith('#')]
    
    @staticmethod
    def _normalize(text):
        text = re.sub(r'[ \t]+', ' ', text)
        text = re.sub(r'^\s*\n', '', text)
        text = re.sub(r'\n+', '\n', text)
        text = re.sub(r'\\\n\s+', '', text)
        return text


class Parser(object):
    def __init__(self):
        self.__classes = []
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
        #print line
        stack = []
        node = []
        lexer = (t.group(0) for t in re.finditer(re.compile(r'\s+|\*|\(|,|\)|;|[^\s*(,);]+'), line))
        for token in lexer:
            if not token.strip() or token == ';':
                # ignore separators
                continue
            if token == '(':
                stack.append(node)
                node = [node.pop()] # function name start-arg-list
                stack.append(node)
                node = [] # start-arg
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
                continue
            if token == '*':
                node.append(['*', node.pop()])
                continue
            node.append(token)
            continue
        #print node # parsed tree
        
        if 'TClassDef' in line:
            # class def
            self.__classes.append(Class(node))
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
        return self.__node[0][1]
    
    @property
    def methods(self):
        return self.__methods

    def add_if_member(self, f):
        if f.name.startswith('%s_' % self.name):
            self.__methods.append(f)


class Function(object):
    def __init__(self, node):
        assert len(node) > 1
        assert len(node[1]) > 1
        self.__node = node

    @property
    def name(self):
        return self.__node[1][0]

    @property
    def args(self):
        s = ''
        _args = (self.__node[1])[1:]
        for i, arg in enumerate(_args):
            if i:
                s += ', '
            s += str(Arg(arg, i))
        return s
    
    @property
    def returns(self):
        pass

    def trait_fn(self, gen):
        gen.println('fn %s(%s)%s' % self.name, self.args, self.returns)


class Arg(object):
    def __init__(self, node):
        assert len(node) > 0
        self.__node = node

    def name(self):
        return self.__node[1]

    def type(self):
        return Type2(self.__node[0])

    def __str__(self):
        return '%s: %s' % (self.name, self.type)


class Type2(object):
    def __init__(self, node):
        assert len(node) > 0
        self.__node = node

    def __str__(self):
        # TODO
        return self.__node


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
        if s == 'float':
            s = 'c_float'
        if s == 'int':
            s = 'c_int'
        if s == 'long':
            s = 'c_long'
        if s == 'long long':
            s = 'c_longlong'
        if self.is_ptr:
            if self.is_primitive():
                if s == 'void':
                    s = 'u8'
                s = '*%s' % s
            else:
                s = '@mut %s' % s
        return '%s /* %s */' % (s, self.original)


if __name__ == '__main__':
    main()
