cmake_minimum_required(VERSION 2.8)

include(rust.cmake)

add_subdirectory(wxc)

set(RUSTCFLAGS ${RUSTCFLAGS} -L wxc)
set(GENSRC wxHaskell/wxc/src/include)
set(GENSRCS
    ${GENSRC}/wxc.h
)

macro(codegen output)
    add_custom_command(
        OUTPUT  ${CMAKE_SOURCE_DIR}/${output}
        COMMAND python codegen.py ${GENSRCS} > ${output}
        DEPENDS codegen.py ${GENSRCS}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
endmacro()

codegen(wrapper.rs)

add_rust_crate(wxrust
    wx.rc
    native.rs
    wrapper.rs
    wxc
)

add_rust_crate(test
    test.rs
    wxrust
)

# Make Mac OS X App Bundle
if(APPLE)
    add_custom_command(
        OUTPUT Test.app.dummy
        COMMAND mkdir -p Test.app/Contents/MacOS
        COMMAND cp test  Test.app/Contents/MacOS/
        COMMAND cp ${CMAKE_SOURCE_DIR}/Info.plist Test.app/Contents/
        COMMAND touch Test.app.dummy
        DEPENDS Info.plist test
    )
    add_custom_target(Test.app ALL DEPENDS Test.app.dummy)
endif(APPLE)
