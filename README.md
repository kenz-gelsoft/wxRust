# wxRust

master: [![master build status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=master)](https://travis-ci.org/kenz-gelsoft/wxRust)
/ rust-0.9: [![rust 0.9 build status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=rust-0.9)](https://travis-ci.org/kenz-gelsoft/wxRust)
/ rust-0.8: [![rust 0.8 build status](https://travis-ci.org/kenz-gelsoft/wxRust.png?branch=rust-0.8)](https://travis-ci.org/kenz-gelsoft/wxRust)

This is a [Rust](http://www.rust-lang.org/) binding for the [wxWidgets cross platform toolkit](http://www.wxwidgets.org/).

## API

[wxRust API documentation](http://kenz-gelsoft.github.io/wxRust/)

## How it works

The wxRust library is heavily based on the [wxHaskell](http://www.haskell.org/haskellwiki/WxHaskell)'s wxc library.

The [wxc](https://github.com/wxHaskell/wxHaskell/tree/master/wxc) is a C language binding for the C++ wxWidgets toolkit.

We utilize the [rust-bindgen](https://github.com/crabtw/rust-bindgen)
[![rust-bindgen build status](https://api.travis-ci.org/crabtw/rust-bindgen.png?branch=master)](https://travis-ci.org/crabtw/rust-bindgen) 
automatic rust binding generator for its [_unsafe](http://kenz-gelsoft.github.io/wxRust/src/wx/src/_unsafe.rs.html) low-level binding.

And we generate an OOP-style high-level binding (other modules than _unsafe)
by [codegen.py code generator](https://github.com/kenz-gelsoft/wxRust/blob/rust-servo/src/codegen.py).

## Build

We use [CMake](http://www.cmake.org/) for cross platform build, but Windows platform is not yet tested.

For Linux build instructions, see [INSTALL.linux.md](INSTALL.linux.md)

### Build Prerequisite

Use following Rust compiler version for your wxRust branch. We're using Servo master's one for main development.

<table>
<tr><td><strong>wxRust branch</strong></td><td><strong>Supported Rust compiler version</strong>        </td></tr>
<tr><td>master    </td><td><a href="https://github.com/mozilla/rust">master</a>                        </td></tr>
<tr><td>rust-0.9  </td><td><a href="https://github.com/mozilla/rust/releases/tag/0.9">0.9</a>          </td></tr>
<tr><td>rust-servo</td><td><a href="https://github.com/mozilla/servo/">Servo master</a> bundled version</td></tr>
<tr><td>rust-0.8  </td><td><a href="https://github.com/mozilla/rust/releases/tag/0.8">0.8</a>          </td></tr>
</table>

Install the wxWidgets 2.9.5 (or later) and CMake as below
(in the case of [Homebrew](http://brew.sh/)):

    brew install wxmac --devel
    brew install cmake

With some tweak you may be able to compile wxRust with a bit older versions (2.9.0 < x < 2.9.4) of wxWidgets.
See issue #21 comments for details.

### Build the library

At the project root directory,

Checkout git submodules:

    git submodule init # for the first time.
    git submodule update

And generate Makefiles and make:

    mkdir build
    cd build
    cmake ..
    make

If you use wxmac installed by [MacPorts](http://www.macports.org/),
add "-DUSE_MACPORTS=ON" option to above cmake command line:

    cmake -DUSE_MACPORTS=ON ..

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
