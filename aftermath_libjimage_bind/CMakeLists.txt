cmake_minimum_required(VERSION 3.22)

IF (NOT DEFINED ENV{JAVA_HOME})
    message(FATAL_ERROR "JAVA_HOME does not exist, please set it. Exiting.")
endif()

PROJECT(aftermath_libjimage_bind)
INCLUDE_DIRECTORIES($ENV{JAVA_HOME}/include/ include/ libfmt/include/)

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

IF (WIN32)
    message("Detected Windows, linking to libjimage.dll")
    LINK_LIBRARIES($ENV{JAVA_HOME}/lib/libjimage.dll)
else()
    message("Detected non-Windows, linking to libjimage.so")
    LINK_LIBRARIES($ENV{JAVA_HOME}/lib/libjimage.so)
endif()

add_subdirectory(libfmt EXCLUDE_FROM_ALL)

ADD_LIBRARY(aftermath_libjimage_bind
            STATIC
            ./src/aftermath_libjimage_bind.cpp
            include/aftermath_libjimage_bind.h
            include/jimage.hpp)

ADD_EXECUTABLE(aftermath_libjimage_bind_executable
            ./src/aftermath_libjimage_bind.cpp
            include/aftermath_libjimage_bind.h
            include/jimage.hpp)

target_link_libraries(aftermath_libjimage_bind_executable PUBLIC fmt)
