# wxRust

This is an attempt creating wxWidgets binding for Rust programming language.

## How it'll work

I'll build up a thin Rust wrapper of the wxc C-wrapped binding maintained at wxHaskell's repository.

## Build

There is no artifact of this library. But there is only a build instruction of dependency of this library.

Currently, we target only Mac OS X for the platform to build with.

### Build Prerequisite

Install the wxWidgets 2.9.4 as below:

    brew install wxmac --devel`

### libwxc.dylib

    cd wxc
    make
