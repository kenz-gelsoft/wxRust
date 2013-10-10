from optparse   import OptionParser
from subprocess import PIPE, Popen

import re
import sys


def main():
    op = OptionParser()
    op.add_option('-e', '--extern_fn', action='store_true', dest='extern_fn',
                  help='output extern_fn rust source code.')
    op.add_option('-w', '--wrapper', action='store_true', dest='wrapper',
                  help='output wrapper rust source code.')
    (options, args) = op.parse_args()

    parser = Parser()
    
    codegen = None
    if options.extern_fn:
        codegen = ExternFnGenerator(parser)
    elif options.wrapper:
        codegen = WrapperGenerator(parser)
    else:
        sys.exit('no command specified')

    parser.parse_files(args)
    codegen.generate()


class ExternFnGenerator(object):
    def __init__(self, parser):
        self.__parser = parser
        self.__indent = 0

    def generate(self):
        self.println('use std::libc::*;')
        self.println()
        self.println('#[link_args="-lwxc"]')
        self.println('extern {')
        self.indent()
        for name, parts in self.__parser.classes.iteritems():
            self.print_class(name, parts)
        self.unindent()
        self.println('}')
    
    def print_class(self, name, parts):
        for part in parts:
            if part.header_line:
                self.println('\n// %s' % part.header_line)
            for method in part.methods:
                self.println(method.extern_fn)
    
    def println(self, text=''):
        lines = text.split('\n')
        for line in lines:
            line = '%s%s' % (''+(' ' * 4 * self.__indent), line)
            print line

    def indent(self):
        self.__indent += 1

    def unindent(self):
        self.__indent -= 1


class WrapperGenerator(object):
    def __init__(self, parser):
        self.__parser = parser
        self.__indent = 0

    def generate(self):
        self.println('use std::libc::*;')
        self.println('use native::*;')
        self.println()
        for name, parts in self.__parser.classes.iteritems():
            self.print_class(name, parts)

    def print_class(self, name, parts):
        if not name:
            self.println('// skipping globals...')
            self.println()
            return
                
        self.println('trait %s {' % name)
        self.indent()
        for part in parts:
            if len(part.methods) == 0:
                continue
            
            for method in part.methods:
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
        self.__current = Class(self)
        self.__classes = {}
    
    def parse_files(self, files):
        for file in files:
            self._new_class()
            self._parse(Preprocessor().preprocess(file))
        self._new_class()

    @property
    def classes(self):
        return self.__classes

    def _new_class(self):
        if self.__current:
            if self.__current.name not in self.__classes:
                self.__classes[self.__current.name] = []
            self.__classes[self.__current.name].append(self.__current)
        self.__current = Class(self)
    
    def _parse(self, lines):
        for line in lines:
            self._parse_line(line.strip())
    
    def _parse_line(self, line):
        if not len(line):
            return
        if 'TClassDef' in line:
            # special line
            self._new_class()
            self.__current.parse_header_line(line)
            return
        if not('(' in line and ')' in line):
            # all delarations are functions.
            #assert False
            return
        self.__current.parse_line(line)


class Class(object):
    def __init__(self, parser):
        self.__parser = parser
        self.__header_line = None
        self.__methods = []
        self.__name = None

    def parse_header_line(self, line):
        self.__header_line = line
        matched = re.search(r'TClassDef(?:Extend)?\(([^)]*)\)', line)
        assert matched
        classes = matched.group(1).split(',')
        if len(classes) > 1:
            self.__base = classes[1].strip()
        self.__name = classes[0].strip()

    def parse_line(self, line):
        self.__methods.append(Method(self.name).parse(line))

    @property
    def header_line(self):
        return self.__header_line

    @property
    def name(self):
        if not self.__name:
            return ''
        return self.__name
    
    @property
    def base(self):
        return self.__base or None

    @property
    def methods(self):
        return self.__methods


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
        arg = arg.replace(' Out ', ' ')
        arg = arg.strip()
        assert len(arg.split(' ')) <= 2
        return arg

    @property
    def args(self):
        args = ''
        for arg in self.__args:
            if len(args):
                args += ', '
            args += '%s: %s' % (arg[1], arg[0])
        return args

    @property
    def calling_args(self):
        args = ''
        for arg in self.__args:
            if len(args):
                args += ', '
            args += arg[1]
        return args

    @property
    def fn_return(self):
        ret = ''
        if not self.__rtype.is_void:
            ret = ' -> %s' % self.__rtype
        return ret

    @property
    def trait_method_name(self):
        prefix = '%s_' % self.__classname
        if self.__name.startswith(prefix):
            return self.__name[len(prefix):]
        return self.__name

    @property
    def extern_fn(self):
        return 'pub fn %s(%s)%s;' % (self.__name, self.args, self.fn_return)

    def trait_fn(self, gen):
        gen.println('#[fixed_stack_segment]')
        gen.println('fn %s(%s)%s {' % (self.trait_method_name, self.args, self.fn_return))
        gen.indent()
        gen.println('unsafe {')
        gen.indent()
        gen.println('%s(%s)' % (self.__name, self.calling_args))
        gen.unindent()
        gen.println('}')
        gen.unindent()
        gen.println('}')


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
