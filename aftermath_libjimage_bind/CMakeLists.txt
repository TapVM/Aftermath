cmake_minimum_required(VERSION 3.22)

IF (NOT DEFINED ENV{JAVA_HOME})
    message(FATAL_ERROR "JAVA_HOME does not exist, please set it. Exiting.")
endif()

PROJECT(aftermath_libjimage_bind)
INCLUDE_DIRECTORIES($ENV{JAVA_HOME}/include/ include/ libfmt/include/)

set(CMAKE_CXX_FLAGS "-Wall -Wextra -Wpedantic -Werror -Wno-unused-parameter")
set(CMAKE_CXX_STANDARD 20)

IF (APPLE)
    message("Detected MacOS, adding JAVA_HOME/include/darwin/ to include path.")
    INCLUDE_DIRECTORIES($ENV{JAVA_HOME}/include/darwin/)
elseif(WIN32)
    message("Detected Windows, adding JAVA_HOME/include/win32 to include path.")
    INCLUDE_DIRECTORIES($ENV{JAVA_HOME}/include/win32/)
else()
    message("Detected neither MacOS or Windows, adding JAVA_HOME/include/linux/ to include path.")
    INCLUDE_DIRECTORIES($ENV{JAVA_HOME}/include/linux/)
endif()

add_subdirectory(libfmt EXCLUDE_FROM_ALL)

ADD_LIBRARY(aftermath_libjimage_bind
            STATIC
            # C++ files.
            ./src/aftermath_libjimage_bind.cpp
            ./src/endian.cpp
            ./src/imageDecompressor.cpp
            ./src/imageFile.cpp
            ./src/jimage.cpp
            ./src/NativeImageBuffer.cpp)

ADD_EXECUTABLE(aftermath_libjimage_bind_executable
            ./src/aftermath_libjimage_bind.cpp
            include/aftermath_libjimage_bind.h
            include/jimage.hpp)

target_link_libraries(aftermath_libjimage_bind_executable PUBLIC fmt)
