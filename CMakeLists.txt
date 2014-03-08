cmake_minimum_required(VERSION 2.8)

include(rust.cmake)

add_subdirectory(wxc)

set(RUSTCFLAGS ${RUSTCFLAGS} -L wxc)
set(GENSRC wxHaskell/wxc/src/include)
set(GENSRCS
    ${GENSRC}/wxc.h
)

if(APPLE)
    set(PATCH_UNSAFE_RS patch -p0 < src/_unsafe.rs.patch)
    execute_process(
        COMMAND xcode-select -p
        OUTPUT_VARIABLE OSX_SDK
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    set(LIBCLANG_PATH ${OSX_SDK}/Toolchains/XcodeDefault.xctoolchain/usr/lib)
    set(BINDGEN_LIBPATH DYLD_FALLBACK_LIBRARY_PATH=${LIBCLANG_PATH})
    set(RUSTCFLAGS ${RUSTCFLAGS} -L ${LIBCLANG_PATH})
else(APPLE)
    set(PATCH_UNSAFE_RS)
    set(BINDGEN_LIBPATH)
endif(APPLE)

add_rust_crate(bindgen
    rust-bindgen/bindgen.rs
    rust-bindgen/clang.rs
    rust-bindgen/clangll.rs
    rust-bindgen/gen.rs
    rust-bindgen/main.rs
    rust-bindgen/types.rs
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
    DEPENDS bindgen.dummy ${GENSRCS}
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
    COMMAND rustdoc -o doc src/wx.rc
    DEPENDS src/generated.dummy
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
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
