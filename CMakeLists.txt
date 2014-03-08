cmake_minimum_required(VERSION 2.8)

include(rust.cmake)

add_subdirectory(wxc)

if(APPLE)
    set(PATCH_UNSAFE_RS patch -p0 < src/_unsafe.rs.patch)
    execute_process(
        COMMAND xcode-select --print-path
        OUTPUT_VARIABLE OSX_SDK
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    set(LIBCLANG_PATH ${OSX_SDK}/Toolchains/XcodeDefault.xctoolchain/usr/lib)
    set(BINDGEN_LIBPATH DYLD_FALLBACK_LIBRARY_PATH=${LIBCLANG_PATH})
else(APPLE)
    set(PATCH_UNSAFE_RS)
    file(GLOB LIBCLANG_PATH "/usr/lib/llvm*/lib")
    set(BINDGEN_LIBPATH LD_LIBRARY_PATH=${LIBCLANG_PATH})
endif(APPLE)

set(RUSTCFLAGS ${RUSTCFLAGS} -L ${LIBCLANG_PATH} -L wxc)
set(GENSRC wxHaskell/wxc/src/include)
set(GENSRCS
    ${GENSRC}/wxc.h
)

add_rust_crate_nonall(bindgen
    rust-bindgen/bindgen.rs
    rust-bindgen/clang.rs
    rust-bindgen/clangll.rs
    rust-bindgen/gen.rs
    rust-bindgen/main.rs
    rust-bindgen/types.rs
    bindgen_patched
)

add_custom_command(
    OUTPUT bindgen_patched
    COMMAND patch -p1 < ../rust-bindgen.patch
    COMMAND touch bindgen_patched
    DEPENDS ${CMAKE_SOURCE_DIR}/rust-bindgen.patch
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rust-bindgen
)

add_custom_target(bindgen_unpatched ALL
    COMMAND patch -p1 -R < ../rust-bindgen.patch
    DEPENDS bindgen.dummy
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rust-bindgen
)

add_custom_command(
    OUTPUT  ${CMAKE_SOURCE_DIR}/src/_unsafe.rs
    COMMAND ${BINDGEN_LIBPATH} ${CMAKE_BINARY_DIR}/bindgen
        -allow-bitfields
        -x c++
        `wx-config --cppflags`
        --include stdint.h
        --include time.h
        ${GENSRCS}
        > src/_unsafe.rs
    COMMAND ${PATCH_UNSAFE_RS}
    DEPENDS bindgen_unpatched ${GENSRCS}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_custom_command(
    OUTPUT  ${CMAKE_SOURCE_DIR}/src/generated.dummy
    COMMAND python src/codegen.py ${GENSRCS}
    COMMAND touch  src/generated.dummy
    DEPENDS src/codegen.py ${GENSRCS}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_rust_crate(wxrust
    src/wx.rc
    src/defs.rs
    src/_unsafe.rs
    src/generated.dummy
    wxc
)

add_custom_target(doc
    COMMAND rustdoc -o doc ${CMAKE_SOURCE_DIR}/src/wx.rc
    DEPENDS src/generated.dummy
)

add_rust_crate(test
    tests/test.rs
    tests/macros.rs
    wxrust.dummy
)

# Make Mac OS X App Bundle
if(APPLE)
    add_custom_command(
        OUTPUT Test.app.dummy
        COMMAND mkdir -p Test.app/Contents/MacOS
        COMMAND cp test  Test.app/Contents/MacOS/
        COMMAND cp ${CMAKE_SOURCE_DIR}/tests/Info.plist Test.app/Contents/
        COMMAND touch Test.app.dummy
        DEPENDS tests/Info.plist test.dummy
    )
    add_custom_target(Test.app ALL DEPENDS Test.app.dummy)
endif(APPLE)
