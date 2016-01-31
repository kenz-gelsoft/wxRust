# wxRust

master: [![master build status](https://travis-ci.org/kenz-gelsoft/wxRust.svg?branch=master)](https://travis-ci.org/kenz-gelsoft/wxRust)
/ mac(0.10): [![Mac(0.10) build status](https://travis-ci.org/kenz-gelsoft/wxRust.svg?branch=rust-mac)](https://travis-ci.org/kenz-gelsoft/wxRust)

This is a [Rust](http://www.rust-lang.org/) binding for the [wxWidgets cross platform toolkit](http://www.wxwidgets.org/).

## API

[wxRust API documentation](http://www.rust-ci.org/kenz-gelsoft/wxRust/doc/wx/)

## How it works

The wxRust library is heavily based on the [wxHaskell](http://www.haskell.org/haskellwiki/WxHaskell)'s wxc library.

The [wxc](https://github.com/wxHaskell/wxHaskell/tree/master/wxc) is a C language binding for the C++ wxWidgets toolkit.

We utilize the [rust-bindgen](https://github.com/crabtw/rust-bindgen)
[![rust-bindgen build status](https://api.travis-ci.org/crabtw/rust-bindgen.svg?branch=master)](https://travis-ci.org/crabtw/rust-bindgen) 
automatic rust binding generator for its [_unsafe](http://www.rust-ci.org/kenz-gelsoft/wxRust/doc/src/wx/home/travis/build/kenz-gelsoft/wxRust/src/_unsafe.rs.html) low-level binding.

And we generate an OOP-style high-level binding (other modules than _unsafe)
by [codegen.py code generator](https://github.com/kenz-gelsoft/wxRust/blob/master/src/codegen.py).

## Build

### Build Prerequisite

Use following Rust compiler version for your wxRust branch. We're using Servo master's one for main development.

<table>
<tr><td><strong>wxRust branch</strong></td><td><strong>Supported Rust compiler version</strong>        </td></tr>
<tr><td>master    </td><td><a href="https://github.com/mozilla/rust">master</a>                        </td></tr>
</table>

Install the wxWidgets 3.0 (2.9.5 or later is required) as below
(in the case of [Homebrew](http://brew.sh/)):

    brew install wxmac

With some tweak you may be able to compile wxRust with a bit older versions (2.9.0 < x < 2.9.4) of wxWidgets.
See issue #21 comments for details.

### Build the library

At the project root directory,

Checkout git submodules:

    git submodule init # for the first time.
    git submodule update

And build:

    cargo build

If you get following error on Mac:

       Compiling wx v0.1.0 (file:///path/to/wxRust)
    failed to run custom build command for `wx v0.1.0 (file:///path/to/wxRust)`
    Process didn't exit successfully: `/path/to/wxRust/target/debug/build/wx-a555275d7670a125/build-script-build` (signal: 5)
    --- stderr
    dyld: Library not loaded: @rpath/libclang.dylib
      Referenced from: /path/to/wxRust/target/debug/build/wx-a555275d7670a125/build-script-build
      Reason: image not found

Re-run `cargo build` after setting `LD_LIBRARY_PATH` as below:

    export LD_LIBRARY_PATH=`xcode-select -p`/Toolchains/XcodeDefault.xctoolchain/usr/lib

### Compile and Run the Test program

Use cargo:

    cargo test

Currently, this command runs test headless.
