set(RUSTCFLAGS ${RUSTCFLAGS} -L ${CMAKE_BINARY_DIR} --out-dir ${CMAKE_BINARY_DIR})

if(WIN32)
    set(RUST_MACRO_TOUCH type nul >)
else(WIN32)
    set(RUST_MACRO_TOUCH touch)
endif(WIN32)

macro(add_rust_crate name src)
    add_custom_command(
        OUTPUT  ${name}.dummy
        COMMAND rustc ${RUSTCFLAGS} ${CMAKE_SOURCE_DIR}/${src}
        COMMAND ${RUST_MACRO_TOUCH} ${name}.dummy
        DEPENDS ${src} ${ARGN}
    )
    add_custom_target(${name} ALL DEPENDS ${CMAKE_BINARY_DIR}/${name}.dummy)
endmacro(add_rust_crate)

macro(add_rust_crate_nonall name src)
    add_custom_command(
        OUTPUT  ${name}.dummy
        COMMAND rustc ${RUSTCFLAGS} ${CMAKE_SOURCE_DIR}/${src}
        COMMAND ${RUST_MACRO_TOUCH} ${name}.dummy
        DEPENDS ${src} ${ARGN}
    )
    add_custom_target(${name} DEPENDS ${CMAKE_BINARY_DIR}/${name}.dummy)
endmacro(add_rust_crate_nonall)
