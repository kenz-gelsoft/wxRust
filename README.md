# wxRust

This is an attempt creating wxWidgets binding for Rust programming language.

## How it'll work

I'll build up a thin Rust wrapper of the wxc C-wrapped binding maintained at wxHaskell's repository.

## Build

Currently, we target only Mac OS X for the platform to build with.

### Build Prerequisite

We're using the Rust compiler version bundled with Servo master.

Install the wxWidgets 2.9.4 and CMake as below:

    brew install wxmac --devel`
    brew install cmake

### Build the library

At the project root directory:

    mkdir build
    cd build
    cmake ..
    make

### Compile the Test Program

At the CMake binary directory:

    make test && ./test

For now, test program make and show new wxFrame on screen, but doesn't bring up.
(We need to package the built executable as as Mac application package (.app).)

