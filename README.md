# wxRust

This is an attempt creating wxWidgets binding for the Rust programming language.

## How it'll work

I'll build up a thin Rust wrapper of the wxc C-wrapped binding maintained at the wxHaskell's repository.

## Build

We use CMake for cross platform build, but Windows and Linux platforms are not yet tested.

For Linux build instructions, see INSTALL.linux.md

### Build Prerequisite

We're using the Rust compiler version bundled with the Servo master.

Install the wxWidgets 2.9 (or later) and CMake as below:

    brew install wxmac --devel
    brew install cmake

### Build the library

At the project root directory:

    mkdir build
    cd build
    cmake ..
    make

### Compile and Run the Test program

At the CMake binary directory:

    make test && ./test

On Mac, Run as below:

    make Test.app
    open ./Test.app # or open in Finder
