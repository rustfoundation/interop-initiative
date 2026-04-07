cmake_minimum_required(VERSION 3.10)
project(RustInterop)

add_executable(main main.cpp)

add_custom_command(
    OUTPUT ${CMAKE_SOURCE_DIR}/target/release/librust_interop.a
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_custom_target(rustlib ALL
    DEPENDS ${CMAKE_SOURCE_DIR}/target/release/librust_interop.a
)

add_dependencies(main rustlib)

target_link_libraries(main
    ${CMAKE_SOURCE_DIR}/target/release/librust_interop.a
)
