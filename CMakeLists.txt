cmake_minimum_required(VERSION 3.30)
project(rust_c_bindings_playground C)

set(CMAKE_C_STANDARD 23)

add_custom_target(
	build_rust ALL
	COMMAND cargo build --release
	WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/rust
	COMMENT "Building Rust library..."
)

set(RUST_INCLUDE ${CMAKE_SOURCE_DIR}/rust/target/include)
set(RUST_LIB ${CMAKE_SOURCE_DIR}/rust/target/release/librust.a)

add_library(rust STATIC IMPORTED)
set_target_properties(rust PROPERTIES IMPORTED_LOCATION ${RUST_LIB})

add_executable(main main.c)
add_dependencies(main build_rust)
target_include_directories(main PRIVATE ${RUST_INCLUDE})
target_link_libraries(main PRIVATE rust)
