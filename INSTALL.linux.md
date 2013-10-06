# Installation instructions for linux.

If your distribution is missing, please add it.

## arch

### Prerequisites

You will need to install the following packages first:

* wxgtk2.9
* webkitgtk2
* python2

### Modify CMakeLists.txt:

codegen.py uses python 2 rather than arch's default python 3:

-    COMMAND python codegen.py ${GENSRCS} > native.rs
+    COMMAND python2 codegen.py ${GENSRCS} > native.rs

### Build the library

At the project root directory:

    $ mkdir build
    $ cd build
    $ cmake .. -DwxWidgets_CONFIG_EXECUTABLE=/usr/bin/wx-config-2.9
    $ make

### Compile and Run the Test program

At the CMake binary directory:

    $ make test
    $ LD_LIBRARY_PATH=./wxc ./test
