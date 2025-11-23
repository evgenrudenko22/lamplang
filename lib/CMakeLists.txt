cmake_minimum_required(VERSION 3.15)
project(lamp_lib LANGUAGES C)

option(BUILD_SHARED_STD "Build stdlib as shared library (ON) or static (OFF)" OFF)

set(STDLIB_SOURCES
    src/io.c
    src/area.c
)

if(BUILD_SHARED_STD)
    add_library(lamp_lib SHARED ${STDLIB_SOURCES})
else()
    add_library(lamp_lib STATIC ${STDLIB_SOURCES})
endif()

target_include_directories(lamp_lib
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

set_target_properties(lamp_lib PROPERTIES
    POSITION_INDEPENDENT_CODE ON
    PUBLIC_HEADER "include/area.h;include/io.h"
)

install(TARGETS lamp_lib
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
    PUBLIC_HEADER DESTINATION include
)