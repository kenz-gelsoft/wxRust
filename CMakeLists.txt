cmake_minimum_required(VERSION 2.8)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/RustCMake/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

add_subdirectory(wxc)

if(APPLE)
    set(PATCH_UNSAFE_RS patch -p0 < src/_unsafe.rs.patch)
    execute_process(
        COMMAND xcode-select --print-path
        OUTPUT_VARIABLE XCODE_PATH
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    execute_process(
        COMMAND xcrun --show-sdk-path
        OUTPUT_VARIABLE OSX_SDK
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    execute_process(
        COMMAND rustc --print sysroot
        OUTPUT_VARIABLE RUSTC_SYSROOT
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    set(LIBCLANG_PATH ${XCODE_PATH}/Toolchains/XcodeDefault.xctoolchain/usr/lib)
    set(BINDGEN_LIBPATH DYLD_FALLBACK_LIBRARY_PATH=${LIBCLANG_PATH}:${RUSTC_SYSROOT}/lib/rustlib/x86_64-apple-darwin/lib)
    set(BINDGEN_INCLUDEPATH -I${OSX_SDK}/usr/include)
else(APPLE)
    set(PATCH_UNSAFE_RS)
    file(GLOB LIBCLANG_PATH1 "/usr/lib/llvm*/lib")
    file(GLOB LIBCLANG_PATH2 "/usr/lib64/llvm")
    set(LIBCLANG_PATH ${LIBCLANG_PATH1}${LIBCLANG_PATH2})
    set(BINDGEN_LIBPATH LD_LIBRARY_PATH=${LIBCLANG_PATH})
    set(BINDGEN_INCLUDEPATH)
endif(APPLE)

set(RUSTC_FLAGS ${RUSTC_FLAGS}
    -L ${LIBCLANG_PATH}
    -L wxc
    -L .
)
set(GENSRC wxHaskell/wxc/src/include)
set(GENSRCS
    ${GENSRC}/wxc.h
)

add_custom_command(
    OUTPUT ${CMAKE_SOURCE_DIR}/rust-bindgen/target/debug/bindgen
    COMMAND LIBCLANG_PATH=${LIBCLANG_PATH} cargo build --verbose
    DEPENDS
        rust-bindgen/src/clang.rs
        rust-bindgen/src/clangll.rs
        rust-bindgen/src/gen.rs
        rust-bindgen/src/lib.rs
        rust-bindgen/src/parser.rs
        rust-bindgen/src/types.rs
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rust-bindgen
)

add_custom_command(
    OUTPUT  ${CMAKE_SOURCE_DIR}/src/_unsafe.rs
    COMMAND ${BINDGEN_LIBPATH} ${CMAKE_SOURCE_DIR}/rust-bindgen/target/debug/bindgen
        -allow-unknown-types
        -x c++
        `wx-config --cppflags`
        ${BINDGEN_INCLUDEPATH}
        --include stdint.h
        --include time.h
        ${GENSRCS}
        > src/_unsafe.rs.tmp
    COMMAND mv src/_unsafe.rs.tmp src/_unsafe.rs
    COMMAND ${PATCH_UNSAFE_RS}
    DEPENDS ${CMAKE_SOURCE_DIR}/rust-bindgen/target/debug/bindgen ${GENSRCS}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_custom_command(
    OUTPUT  ${CMAKE_SOURCE_DIR}/src/generated.dummy
    COMMAND python src/codegen.py ${GENSRCS}
    COMMAND touch  src/generated.dummy
    DEPENDS src/codegen.py ${GENSRCS}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

rust_crate(src/lib.rs
    TARGET_NAME wxrust
    DESTINATION lib
    DEPENDS
        src/defs.rs
        src/_unsafe.rs
        src/generated.dummy
        wxc
)

rust_doc(src/lib.rs
    TARGET_NAME doc
    DESTINATION doc
    DEPENDS src/generated.dummy
)

rust_crate(tests/test.rs
    TARGET_NAME test_cmd
    DEPENDS
        tests/macros.rs
        ${wxrust_FULL_TARGET}
)

# Make Mac OS X App Bundle
if(APPLE)
    add_custom_command(
        OUTPUT Test.app
        COMMAND mkdir -p Test.app/Contents/MacOS
        COMMAND cp test  Test.app/Contents/MacOS/
        COMMAND cp ${CMAKE_SOURCE_DIR}/tests/Info.plist Test.app/Contents/
        COMMAND touch Test.app
        DEPENDS tests/Info.plist ${test_cmd_FULL_TARGET}
    )
    add_custom_target(test ALL DEPENDS Test.app)
else(APPLE)
    add_custom_target(test ALL DEPENDS ${test_cmd_FULL_TARGET})
endif(APPLE)
