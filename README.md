# wxRust [![Build Status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=master)](https://travis-ci.org/kenz-gelsoft/wxRust)

This is a [Rust](http://www.rust-lang.org/) binding for the [wxWidgets cross platform toolkit](http://www.wxwidgets.org/).

## API

[wxRust API documentation](http://kenz-gelsoft.github.io/wxRust/)

## How it works

The wxRust library is heavily based on the [wxHaskell](http://www.haskell.org/haskellwiki/WxHaskell)'s wxc library.

The [wxc](https://github.com/wxHaskell/wxHaskell/tree/master/wxc) is a C language binding for the C++ wxWidgets toolkit.

We utilize the [rust-bindgen](https://github.com/crabtw/rust-bindgen) automatic rust binding generator for its [_unsafe](http://kenz-gelsoft.github.io/wxRust/src/wx/src/_unsafe.rs.html) low-level binding.

And we generate an OOP-style high-level binding (other modules than _unsafe) by [codegen.py code generator](https://github.com/kenz-gelsoft/wxRust/blob/rust-servo/src/codegen.py).

## Build

We use [CMake](http://www.cmake.org/) for cross platform build, but Windows platform is not yet tested.

For Linux build instructions, see [INSTALL.linux.md](INSTALL.linux.md)

### Build Prerequisite

We're using the Rust compiler version bundled with the [Servo master](https://github.com/mozilla/servo/).

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

### Generate Documentation

At the CMake binary directory:

    make docs

Generates [a rustdoc documentation](http://kenz-gelsoft.github.io/wxRust/) under docs directory.
